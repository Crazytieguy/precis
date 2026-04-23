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
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::batch::{Batch, BatchContent, BatchId, EntryKind, FsGroup, Render, Span};
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
        if let Some(cached) = self.0.borrow().get(path) {
            return Some(cached.clone());
        }
        let text = std::fs::read_to_string(path).ok()?;
        let arc: Arc<str> = Arc::from(text);
        self.0.borrow_mut().insert(path.to_path_buf(), arc.clone());
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
        children: BTreeMap<OsString, EntryKind>,
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

    /// Marginal cost of applying `batch` against the current state.
    pub fn marginal_cost(&self, batch: &Batch) -> Cost {
        match &batch.content {
            BatchContent::Fs { groups } => self.cost_fs_groups(groups),
            BatchContent::Lines { spans } => self.cost_spans(spans),
        }
    }

    /// Apply a batch. `owner` is the batch's id; `is_ancestor(id)` tells us
    /// whether an existing line's owner is an ancestor — used to detect
    /// non-ancestor line-overwrites (a walker bug; debug-asserts).
    pub fn apply(&mut self, batch: &Batch, owner: BatchId, is_ancestor: impl Fn(BatchId) -> bool) {
        match &batch.content {
            BatchContent::Fs { groups } => {
                for group in groups {
                    self.apply_fs_group(group);
                }
            }
            BatchContent::Lines { spans } => {
                self.apply_spans(spans, owner, &is_ancestor);
            }
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

    fn cost_fs_groups(&self, groups: &[FsGroup]) -> Cost {
        let mut cost = Cost::default();
        for group in groups {
            cost = cost + self.cost_one_listing(&group.parent, &group.children);
        }
        cost
    }

    fn cost_one_listing(&self, parent: &Path, children: &BTreeMap<OsString, EntryKind>) -> Cost {
        let indent_depth = self.depth_from_root(parent);
        let already_listed = match self.nodes.get(parent) {
            Some(TreeNode::Dir { children }) => Some(children),
            _ => None,
        };
        let mut cost = Cost::default();
        for (name, kind) in children {
            if already_listed.is_some_and(|c| c.contains_key(name)) {
                continue;
            }
            let row = format_entry_row(name, *kind, indent_depth);
            cost.tokens += tokenizer::count(&row);
            cost.bytes += row.len();
        }
        cost
    }

    fn cost_spans(&self, spans: &[Span]) -> Cost {
        let resolved = resolve_spans(spans);
        let mut by_path: BTreeMap<&Path, Vec<(usize, &Render)>> = BTreeMap::new();
        for (path, line, render) in &resolved {
            by_path
                .entry(path.as_path())
                .or_default()
                .push((*line, render));
        }

        let mut cost = Cost::default();
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
                if let Some(existing) = existing
                    && let Some(old) = existing.get(&line_num)
                {
                    let old_row = format_line_row(line_num, &old.render, source_line, indent_depth);
                    cost.tokens += new_tokens.saturating_sub(tokenizer::count(&old_row));
                    cost.bytes += new_bytes.saturating_sub(old_row.len());
                } else {
                    cost.tokens += new_tokens;
                    cost.bytes += new_bytes;
                }
            }
        }
        cost
    }

    fn apply_fs_group(&mut self, group: &FsGroup) {
        let parent = &group.parent;
        let children = &group.children;
        for (name, kind) in children {
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
        for (name, kind) in children {
            parent_children.entry(name.clone()).or_insert(*kind);
        }
    }

    fn apply_spans(
        &mut self,
        spans: &[Span],
        owner: BatchId,
        is_ancestor: &impl Fn(BatchId) -> bool,
    ) {
        for (path, line_num, render) in resolve_spans(spans) {
            let node = self
                .nodes
                .entry(path.clone())
                .or_insert_with(TreeNode::empty_file);
            let TreeNode::File { content } = node else {
                debug_assert!(false, "span targets non-file node at {}", path.display());
                continue;
            };
            if let Some(existing) = content.get(&line_num) {
                debug_assert!(
                    is_ancestor(existing.owner),
                    "non-ancestor overlap: batch {:?} would replace line {} of {} owned by non-ancestor batch {:?}",
                    owner,
                    line_num,
                    path.display(),
                    existing.owner,
                );
            }
            content.insert(line_num, LineRecord { render, owner });
        }
    }

    fn render_dir(&self, path: &Path, indent_depth: usize, out: &mut String) {
        let Some(TreeNode::Dir { children }) = self.nodes.get(path) else {
            return;
        };
        let indent = INDENT_UNIT.repeat(indent_depth);
        for (name, kind) in children {
            out.push_str(&indent);
            out.push_str(&name.to_string_lossy());
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

/// Expand a batch's spans into per-(path, line) entries. Overlaps within
/// the same batch resolve by `Render::priority` (Full > Truncated > Ellipsis).
/// Output is sorted by (path, line) — downstream relies on that order.
fn resolve_spans(spans: &[Span]) -> Vec<(PathBuf, usize, Render)> {
    let mut by_key: BTreeMap<(PathBuf, usize), Render> = BTreeMap::new();
    for span in spans {
        for line_num in span.start..=span.end {
            let key = (span.path.clone(), line_num);
            by_key
                .entry(key)
                .and_modify(|existing| {
                    if span.render.priority() > existing.priority() {
                        *existing = span.render.clone();
                    }
                })
                .or_insert_with(|| span.render.clone());
        }
    }
    by_key.into_iter().map(|((p, l), r)| (p, l, r)).collect()
}

fn format_entry_row(name: &OsString, kind: EntryKind, indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push_str(&name.to_string_lossy());
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
