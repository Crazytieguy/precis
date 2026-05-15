//! Rendered tree — the scheduler's "what have we committed to output" state.
//! Two node kinds (directory, file); applies batches (which carry spans,
//! not materialized text); computes marginal cost; renders to a string by
//! reading source through a shared cache.
//!
//! Span materialization happens here — walkers emit `Span { path, start,
//! end, render }`, the tree stores per-line `(owner: BatchId, render: Render)`
//! records, and `render()` reads source to produce the final text. This
//! decouples the walker authoring substrate from the rendering pipeline
//! and lets the NS schema reuse the same `Span`/`Render` types without any
//! materialization going through a separate code path.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::batch::BatchId;
use crate::content::{BatchContent, FsEntries, FsGroup, Render, Span, explode_spans};
use crate::fs_util::{EntryKind, list_dir};
use crate::tokenizer;

const INDENT_UNIT: &str = "    ";

/// Shared source-file cache. The scheduler constructs one and hands clones
/// to both `WalkCtx` (for walker-side parsing) and `RenderedTree` (for
/// materialization at render time), so each file is read at most once per
/// run regardless of how many batches touch it.
#[derive(Clone, Debug, Default)]
pub struct SourceCache(Rc<RefCell<HashMap<PathBuf, Arc<str>>>>);

impl SourceCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read `path`, caching. Returns `None` on I/O error.
    pub fn get(&self, path: &Path) -> Option<Arc<str>> {
        #[cfg(feature = "timing")]
        let _start = std::time::Instant::now();
        if let Some(cached) = self.0.borrow().get(path) {
            #[cfg(feature = "timing")]
            crate::timing::record(|c| &mut c.source_read, _start.elapsed(), Some(true));
            return Some(cached.clone());
        }
        let text = std::fs::read_to_string(path).ok()?;
        let arc: Arc<str> = Arc::from(text);
        self.0.borrow_mut().insert(path.to_path_buf(), arc.clone());
        #[cfg(feature = "timing")]
        crate::timing::record(|c| &mut c.source_read, _start.elapsed(), Some(false));
        Some(arc)
    }

    /// Insert a pre-loaded source (used when the walker has already read the
    /// file via its own path). Idempotent — repeated inserts are no-ops.
    pub fn insert(&self, path: PathBuf, source: Arc<str>) {
        self.0.borrow_mut().entry(path).or_insert(source);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cost {
    pub tokens: usize,
    pub bytes: usize,
}

/// A span wanted to write a line that's already owned by a non-ancestor
/// batch. Scheduler treats as a walker bug (debug-asserts); simulator
/// surfaces as an NS-authoring violation.
#[derive(Debug, Clone)]
pub struct ApplyConflict {
    pub path: PathBuf,
    pub line: usize,
    pub existing_owner: BatchId,
}

#[derive(Debug, Clone)]
struct LineRecord {
    render: Render,
    owner: BatchId,
}

/// A node in the rendered tree is either a directory (with named children)
/// or a file (with per-line render records).
#[derive(Debug)]
enum TreeNode {
    Dir {
        children: BTreeMap<String, EntryKind>,
    },
    File {
        content: BTreeMap<usize, LineRecord>,
    },
}

impl TreeNode {
    fn empty_dir() -> Self {
        Self::Dir {
            children: BTreeMap::new(),
        }
    }
    fn empty_file() -> Self {
        Self::File {
            content: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct RenderedTree {
    root: PathBuf,
    nodes: HashMap<PathBuf, TreeNode>,
    source_cache: SourceCache,
}

impl RenderedTree {
    pub fn new(root: PathBuf, source_cache: SourceCache) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(root.clone(), TreeNode::empty_dir());
        Self {
            root,
            nodes,
            source_cache,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Marginal cost of applying `content` against the current state.
    pub fn marginal_cost(&self, content: &BatchContent) -> Cost {
        let mut total = Cost::default();
        self.visit_atom_costs(content, |c| total = total + c);
        total
    }

    /// Marginal cost broken down per atom — one entry per atom in the
    /// batch, in `divergence::atoms_from_content` iteration order so
    /// callers can index 1:1. Atoms that don't contribute (FS entries
    /// already listed, line refinements no longer than the prior render)
    /// yield `Cost::default()`. Used by the divergence walker-waste
    /// accounting to attribute off-NS spend to atoms with their actual
    /// marginal contribution — bodies and short decls can sit in the
    /// same batch but differ ~10× in token weight.
    pub fn marginal_cost_per_atom(&self, content: &BatchContent) -> Vec<Cost> {
        let mut out = Vec::new();
        self.visit_atom_costs(content, |c| out.push(c));
        out
    }

    /// Internal visitor: invokes `visit(cost)` once per atom in iteration
    /// order. Single source of truth for the per-atom cost formula —
    /// `marginal_cost` sums into a scalar without allocating, and
    /// `marginal_cost_per_atom` collects into a `Vec<Cost>` for callers
    /// that need per-atom granularity.
    fn visit_atom_costs<F: FnMut(Cost)>(&self, content: &BatchContent, mut visit: F) {
        match content {
            BatchContent::Fs { groups } => self.visit_fs_atom_costs(groups, &mut visit),
            BatchContent::Lines { spans } => self.visit_span_atom_costs(spans, &mut visit),
        }
    }

    /// Apply `content` against the rendered tree. `owner` is the
    /// emitting batch's id; `is_ancestor(id)` tells us whether an
    /// existing line's owner is an ancestor — non-ancestor overlaps
    /// are returned as [`ApplyConflict`]s so callers can surface them
    /// as violations (simulator) or debug-assert (scheduler, which
    /// trusts walker-emitted batches to declare correct predecessors).
    pub fn apply(
        &mut self,
        content: &BatchContent,
        owner: BatchId,
        is_ancestor: impl Fn(BatchId) -> bool,
    ) -> Vec<ApplyConflict> {
        match content {
            BatchContent::Fs { groups } => {
                for group in groups {
                    self.apply_fs_group(group);
                }
                Vec::new()
            }
            BatchContent::Lines { spans } => self.apply_spans(spans, owner, &is_ancestor),
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        self.render_dir(&self.root, 0, &mut out);
        out
    }

    pub fn total_tokens(&self) -> usize {
        tokenizer::count(&self.render())
    }

    pub fn total_bytes(&self) -> usize {
        self.render().len()
    }

    // ---- internal ----

    fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    fn visit_fs_atom_costs<F: FnMut(Cost)>(&self, groups: &[FsGroup], visit: &mut F) {
        for group in groups {
            let FsEntries::Listed(paths) = &group.entries else {
                debug_assert!(
                    false,
                    "unresolved FsEntries reached cost path at {}",
                    group.parent.display()
                );
                continue;
            };
            let parent = &group.parent;
            let indent_depth = self.depth_from_root(parent);
            let already_listed = match self.nodes.get(parent) {
                Some(TreeNode::Dir { children }) => Some(children),
                _ => None,
            };
            let probed = list_dir(parent);
            for p in paths {
                let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };
                let cost = if already_listed.is_some_and(|c| c.contains_key(name)) {
                    Cost::default()
                } else {
                    let kind = probed.get(name).copied().unwrap_or(EntryKind::File);
                    let row = format_entry_row(name, kind, indent_depth);
                    Cost {
                        tokens: tokenizer::count(&row),
                        bytes: row.len(),
                    }
                };
                visit(cost);
            }
        }
    }

    fn visit_span_atom_costs<F: FnMut(Cost)>(&self, spans: &[Span], visit: &mut F) {
        let resolved = explode_spans(spans);
        // explode_spans yields `(path, line)` in lex order; grouping by
        // path lets source + indent + existing-content lookups happen
        // once per path while preserving that order — required so the
        // visitor's per-atom output indexes 1:1 with
        // `divergence::atoms_from_content`'s Lines arm.
        let mut by_path: BTreeMap<&Path, Vec<(usize, &Render)>> = BTreeMap::new();
        for (path, line, render) in &resolved {
            by_path
                .entry(path.as_path())
                .or_default()
                .push((*line, render));
        }

        for (path, entries) in by_path {
            let source = self.source_cache.get(path);
            let src_lines: Vec<&str> = source
                .as_deref()
                .map(|s| s.lines().collect())
                .unwrap_or_default();
            let indent_depth = self.depth_from_root(path);
            let existing = match self.nodes.get(path) {
                Some(TreeNode::File { content }) => Some(content),
                _ => None,
            };
            for (line_num, render) in entries {
                debug_assert!(
                    src_lines.is_empty() || line_num >= 1 && line_num <= src_lines.len(),
                    "span line {} out of range (1..={}) for {} — schema load should have caught this",
                    line_num,
                    src_lines.len(),
                    path.display(),
                );
                let source_line = src_lines.get(line_num - 1).copied().unwrap_or("");
                let new_row = format_line_row(line_num, render, source_line, indent_depth);
                let new_tokens = tokenizer::count(&new_row);
                let new_bytes = new_row.len();
                let cost = if let Some(existing) = existing
                    && let Some(old) = existing.get(&line_num)
                {
                    let old_row = format_line_row(line_num, &old.render, source_line, indent_depth);
                    Cost {
                        tokens: new_tokens.saturating_sub(tokenizer::count(&old_row)),
                        bytes: new_bytes.saturating_sub(old_row.len()),
                    }
                } else {
                    Cost {
                        tokens: new_tokens,
                        bytes: new_bytes,
                    }
                };
                visit(cost);
            }
        }
    }

    fn apply_fs_group(&mut self, group: &FsGroup) {
        let parent = &group.parent;
        let FsEntries::Listed(paths) = &group.entries else {
            debug_assert!(
                false,
                "unresolved FsEntries reached apply path at {}",
                parent.display()
            );
            return;
        };
        let probed = list_dir(parent);
        let resolved: Vec<(String, EntryKind)> = paths
            .iter()
            .filter_map(|p| {
                let name = p.file_name().and_then(|n| n.to_str())?.to_string();
                let kind = probed.get(&name).copied().unwrap_or(EntryKind::File);
                Some((name, kind))
            })
            .collect();
        for (name, kind) in &resolved {
            let child_path = parent.join(name);
            self.nodes.entry(child_path).or_insert_with(|| match kind {
                EntryKind::Directory => TreeNode::empty_dir(),
                EntryKind::File => TreeNode::empty_file(),
            });
        }
        let parent_node = self
            .nodes
            .entry(parent.clone())
            .or_insert_with(TreeNode::empty_dir);
        let TreeNode::Dir {
            children: parent_children,
        } = parent_node
        else {
            debug_assert!(false, "FsGroup parent {} is a File node", parent.display());
            return;
        };
        for (name, kind) in resolved {
            parent_children.entry(name).or_insert(kind);
        }
    }

    fn apply_spans(
        &mut self,
        spans: &[Span],
        owner: BatchId,
        is_ancestor: &impl Fn(BatchId) -> bool,
    ) -> Vec<ApplyConflict> {
        let mut conflicts = Vec::new();
        for (path, line_num, render) in explode_spans(spans) {
            let node = self
                .nodes
                .entry(path.clone())
                .or_insert_with(TreeNode::empty_file);
            let TreeNode::File { content } = node else {
                debug_assert!(false, "span targets non-file node at {}", path.display());
                continue;
            };
            if let Some(existing) = content.get(&line_num)
                && !is_ancestor(existing.owner)
            {
                conflicts.push(ApplyConflict {
                    path: path.clone(),
                    line: line_num,
                    existing_owner: existing.owner,
                });
                continue;
            }
            content.insert(line_num, LineRecord { render, owner });
        }
        conflicts
    }

    fn render_dir(&self, path: &Path, indent_depth: usize, out: &mut String) {
        let Some(TreeNode::Dir { children }) = self.nodes.get(path) else {
            return;
        };
        let indent = INDENT_UNIT.repeat(indent_depth);
        for (name, kind) in children {
            out.push_str(&indent);
            out.push_str(name);
            match kind {
                EntryKind::Directory => {
                    out.push_str("/\n");
                    self.render_dir(&path.join(name), indent_depth + 1, out);
                }
                EntryKind::File => {
                    out.push('\n');
                    self.render_file(&path.join(name), indent_depth + 1, out);
                }
            }
        }
    }

    fn render_file(&self, path: &Path, indent_depth: usize, out: &mut String) {
        let Some(TreeNode::File { content }) = self.nodes.get(path) else {
            return;
        };
        if content.is_empty() {
            return;
        }
        let source = self.source_cache.get(path);
        let src_lines: Vec<&str> = source
            .as_deref()
            .map(|s| s.lines().collect())
            .unwrap_or_default();
        for (number, record) in content {
            let source_line = src_lines.get(*number - 1).copied().unwrap_or("");
            out.push_str(&format_line_row(
                *number,
                &record.render,
                source_line,
                indent_depth,
            ));
        }
    }
}

impl std::ops::Add for Cost {
    type Output = Cost;
    fn add(self, other: Cost) -> Cost {
        Cost {
            tokens: self.tokens + other.tokens,
            bytes: self.bytes + other.bytes,
        }
    }
}

fn format_entry_row(name: &str, kind: EntryKind, indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push_str(name);
    if matches!(kind, EntryKind::Directory) {
        s.push('/');
    }
    s.push('\n');
    s
}

/// Materialize a single rendered line from its render spec + source text.
/// `source_line` is the raw source at that 1-indexed line (without the
/// trailing newline); empty string is the fallback when source is
/// unavailable (release tolerates; debug asserts the invariant).
fn format_line_row(
    number: usize,
    render: &Render,
    source_line: &str,
    indent_depth: usize,
) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    match render {
        Render::Ellipsis => {
            s.push('…');
        }
        Render::Full => {
            s.push_str(&number.to_string());
            s.push('→');
            s.push_str(source_line);
        }
        Render::Truncated { pattern } => {
            s.push_str(&number.to_string());
            s.push('→');
            let compiled = regex::Regex::new(pattern);
            debug_assert!(
                compiled.is_ok(),
                "invalid Truncated regex `{pattern}` — should have been rejected at schema load"
            );
            if let Ok(re) = compiled {
                let m = re.find(source_line);
                debug_assert!(
                    m.as_ref().is_some_and(|m| !m.as_str().is_empty()),
                    "Truncated regex `{pattern}` produced empty or no match on line {number} \
                     — schema loader should have rejected this span"
                );
                if let Some(m) = m {
                    s.push_str(m.as_str());
                }
            }
            s.push('…');
        }
    }
    s.push('\n');
    s
}
