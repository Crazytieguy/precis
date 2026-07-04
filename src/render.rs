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
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::batch::BatchId;
use crate::content::{
    BatchContent, FsEntries, FsGroup, Render, Span, explode_spans, with_truncate_regex,
};
use crate::fs_util::{EntryKind, list_dir};
use crate::tokenizer;

const INDENT_UNIT: &str = "    ";

/// Shared source-file cache — read each file at most once per run.
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

    /// Insert a pre-loaded source. Idempotent.
    pub fn insert(&self, path: PathBuf, source: Arc<str>) {
        self.0.borrow_mut().entry(path).or_insert(source);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cost {
    pub tokens: usize,
    pub bytes: usize,
}

/// A span tried to write a line owned by a non-ancestor batch.
/// Scheduler debug-asserts; simulator surfaces as a violation.
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

    /// Marginal cost of applying `content` — exact tokens.
    pub fn marginal_cost(&self, content: &BatchContent) -> Cost {
        let mut total = Cost::default();
        self.visit_atom_costs(
            content,
            |text| {
                let t = tokenizer::count(text);
                #[cfg(feature = "timing")]
                crate::timing::record_render_row(text.len(), t);
                t
            },
            |c| total = total + c,
        );
        total
    }

    /// Approximate token count (`bytes / k`) — for approx ranking.
    pub fn marginal_cost_approx(&self, content: &BatchContent) -> usize {
        let mut tokens: usize = 0;
        self.visit_atom_costs(content, tokenizer::approx_count, |c| tokens += c.tokens);
        tokens
    }

    /// Per-atom cost visitor — `tokens` chooses exact vs approx
    /// counting; everything else is identical.
    fn visit_atom_costs<F, T>(&self, content: &BatchContent, tokens: T, mut visit: F)
    where
        F: FnMut(Cost),
        T: Fn(&str) -> usize,
    {
        match content {
            BatchContent::Fs { groups } => self.visit_fs_atom_costs(groups, &tokens, &mut visit),
            BatchContent::Lines { spans } => self.visit_span_atom_costs(spans, &tokens, &mut visit),
        }
    }

    /// Apply `content` to the tree. Non-ancestor overlaps come back
    /// as [`ApplyConflict`]s for the caller to handle.
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

    fn visit_fs_atom_costs<F, T>(&self, groups: &[FsGroup], tokens: &T, visit: &mut F)
    where
        F: FnMut(Cost),
        T: Fn(&str) -> usize,
    {
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
                        tokens: tokens(&row),
                        bytes: row.len(),
                    }
                };
                visit(cost);
            }
        }
    }

    fn visit_span_atom_costs<F, T>(&self, spans: &[Span], tokens: &T, visit: &mut F)
    where
        F: FnMut(Cost),
        T: Fn(&str) -> usize,
    {
        let resolved = explode_spans(spans);
        // Group by path so source/indent lookups happen once per file,
        // preserving lex order to keep 1:1 with `atoms_from_content`.
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
            // Anchor sets before / after this batch, so Ellipsis rows
            // that render_file will suppress cost nothing (and Ellipsis
            // rows that were already suppressed deduct nothing).
            let anchors_before = existing.map(content_anchor_lines).unwrap_or_default();
            let mut anchors_after = anchors_before.clone();
            anchors_after.extend(
                entries
                    .iter()
                    .filter(|(_, r)| !matches!(r, Render::Ellipsis))
                    .map(|(n, _)| *n),
            );
            for (line_num, render) in entries {
                debug_assert!(
                    src_lines.is_empty() || line_num >= 1 && line_num <= src_lines.len(),
                    "span line {} out of range (1..={}) for {} — schema load should have caught this",
                    line_num,
                    src_lines.len(),
                    path.display(),
                );
                let source_line = src_lines.get(line_num - 1).copied().unwrap_or("");
                let (new_tokens, new_bytes) = if matches!(render, Render::Ellipsis)
                    && ellipsis_suppressed(line_num, &anchors_after, &src_lines)
                {
                    (0, 0)
                } else {
                    let new_row = format_line_row(line_num, render, source_line, indent_depth);
                    (tokens(&new_row), new_row.len())
                };
                let cost = if let Some(existing) = existing
                    && let Some(old) = existing.get(&line_num)
                {
                    let (old_tokens, old_bytes) = if matches!(old.render, Render::Ellipsis)
                        && ellipsis_suppressed(line_num, &anchors_before, &src_lines)
                    {
                        (0, 0)
                    } else {
                        let old_row =
                            format_line_row(line_num, &old.render, source_line, indent_depth);
                        (tokens(&old_row), old_row.len())
                    };
                    Cost {
                        tokens: new_tokens.saturating_sub(old_tokens),
                        bytes: new_bytes.saturating_sub(old_bytes),
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
        for (name, kind) in children {
            out.push_str(&format_entry_row(name, *kind, indent_depth));
            match kind {
                EntryKind::Directory => {
                    self.render_dir(&path.join(name), indent_depth + 1, out);
                }
                EntryKind::File => {
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
        let anchors = content_anchor_lines(content);
        for (number, record) in content {
            if matches!(record.render, Render::Ellipsis)
                && ellipsis_suppressed(*number, &anchors, &src_lines)
            {
                continue;
            }
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

/// Line numbers whose records render source content (Full/Truncated) —
/// the anchor lines that bound elision gaps. Ellipsis records sit
/// *inside* gaps and don't anchor them.
fn content_anchor_lines(content: &BTreeMap<usize, LineRecord>) -> BTreeSet<usize> {
    content
        .iter()
        .filter(|(_, r)| !matches!(r.render, Render::Ellipsis))
        .map(|(n, _)| *n)
        .collect()
}

/// Whether a bare `…` at `line` should be dropped: true when every
/// source line in the elided gap around it (bounded by the nearest
/// content-rendering lines, or the file edges) is blank — the
/// line-number jump already conveys the gap, and a marker there would
/// claim elided content where there is none.
fn ellipsis_suppressed(line: usize, anchors: &BTreeSet<usize>, src_lines: &[&str]) -> bool {
    let gap_start = anchors.range(..line).next_back().map_or(1, |p| p + 1);
    let gap_end = anchors
        .range(line + 1..)
        .next()
        .map_or(src_lines.len(), |n| n - 1);
    (gap_start..=gap_end).all(|l| src_lines.get(l - 1).is_none_or(|t| t.trim().is_empty()))
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

/// Render one line: render spec + raw source text (empty string when
/// unavailable — release tolerates, debug asserts).
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
            with_truncate_regex(pattern, |re| {
                debug_assert!(
                    re.is_some(),
                    "invalid Truncated regex `{pattern}` — should have been rejected at schema load"
                );
                if let Some(re) = re {
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
            });
            s.push('…');
        }
    }
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const STUB_DIR: &str = "/stub";

    fn stub_dir() -> PathBuf {
        PathBuf::from(STUB_DIR)
    }

    fn listing(entries: &[&str]) -> BatchContent {
        BatchContent::Fs {
            groups: vec![FsGroup {
                parent: stub_dir(),
                entries: FsEntries::Listed(entries.iter().map(PathBuf::from).collect()),
            }],
        }
    }

    fn one_span(path: PathBuf, line: usize, render: Render) -> BatchContent {
        BatchContent::Lines {
            spans: vec![Span {
                path,
                start: line,
                end: line,
                render,
            }],
        }
    }

    #[test]
    fn render_blank_gap_ellipsis_suppressed_nonblank_gap_kept() {
        let cache = SourceCache::new();
        let path = PathBuf::from(format!("{STUB_DIR}/f.c"));
        // Line 2 is blank; lines 4-5 are a non-blank elided gap.
        cache.insert(
            path.clone(),
            Arc::from("fn a();\n\nfn b();\nhidden1\nhidden2\nfn c();\n"),
        );
        let mut tree = RenderedTree::new(stub_dir(), cache);
        tree.apply(&listing(&["f.c"]), BatchId::new(0), |_| true);
        // Anchors first so each ellipsis's gap is bounded at cost time.
        for (line, render) in [
            (1, Render::Full),
            (3, Render::Full),
            (6, Render::Full),
            (2, Render::Ellipsis),
            (4, Render::Ellipsis),
        ] {
            let content = one_span(path.clone(), line, render.clone());
            let cost = tree.marginal_cost(&content);
            if line == 2 {
                assert_eq!(cost.tokens, 0, "blank-gap ellipsis must cost nothing");
                assert_eq!(cost.bytes, 0);
            } else {
                assert!(cost.tokens > 0, "line {line} should cost");
            }
            tree.apply(&content, BatchId::new(line), |_| true);
        }
        let out = tree.render();
        // Blank-only gap (line 2): no marker. Non-blank gap (lines 4-5,
        // with the Ellipsis record at 4): marker kept.
        assert_eq!(out.matches('…').count(), 1, "output:\n{out}");
        let ellipsis_row = out.lines().find(|l| l.trim() == "…");
        assert!(ellipsis_row.is_some(), "output:\n{out}");
        let idx_b = out.find("3→fn b();").unwrap();
        let idx_marker = out.find('…').unwrap();
        let idx_c = out.find("6→fn c();").unwrap();
        assert!(idx_b < idx_marker && idx_marker < idx_c, "output:\n{out}");
    }

    #[test]
    fn render_two_tier_already_listed_fs_zero_under_both_counters() {
        let cache = SourceCache::new();
        let mut tree = RenderedTree::new(stub_dir(), cache);
        tree.apply(&listing(&["a.rs", "b.rs"]), BatchId::new(0), |_| true);

        let overlap = listing(&["a.rs"]);
        let exact = tree.marginal_cost(&overlap);
        let approx_tokens = tree.marginal_cost_approx(&overlap);
        assert_eq!(exact.tokens, 0);
        assert_eq!(exact.bytes, 0);
        assert_eq!(approx_tokens, 0);
    }

    #[test]
    fn render_two_tier_span_refinement_is_delta_under_both_counters() {
        let cache = SourceCache::new();
        let path = PathBuf::from(format!("{STUB_DIR}/syn.rs"));
        cache.insert(
            path.clone(),
            Arc::from("pub fn long_function_name_here() -> Result<()> {}\n"),
        );
        let mut tree = RenderedTree::new(stub_dir(), cache);
        tree.apply(&listing(&["syn.rs"]), BatchId::new(0), |_| true);
        tree.apply(
            &one_span(
                path.clone(),
                1,
                Render::Truncated {
                    pattern: r"^pub fn \w+".into(),
                },
            ),
            BatchId::new(1),
            |_| true,
        );

        let full = one_span(path.clone(), 1, Render::Full);
        let delta_exact = tree.marginal_cost(&full);
        let delta_approx = tree.marginal_cost_approx(&full);

        let cache_fresh = SourceCache::new();
        cache_fresh.insert(
            path.clone(),
            Arc::from("pub fn long_function_name_here() -> Result<()> {}\n"),
        );
        let mut tree_fresh = RenderedTree::new(stub_dir(), cache_fresh);
        tree_fresh.apply(&listing(&["syn.rs"]), BatchId::new(0), |_| true);
        let fresh_exact = tree_fresh.marginal_cost(&full);
        let fresh_approx = tree_fresh.marginal_cost_approx(&full);

        assert!(delta_exact.bytes > 0);
        assert!(delta_exact.tokens > 0);
        assert!(delta_approx > 0);
        assert!(delta_exact.bytes < fresh_exact.bytes);
        assert!(delta_approx < fresh_approx);
    }
}
