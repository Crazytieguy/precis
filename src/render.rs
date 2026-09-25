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
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::batch::BatchId;
use crate::content::{
    BatchContent, FsEntries, FsGroup, Render, Span, explode_spans, with_truncate_regex,
};
use crate::fs_util::{DirFilter, EntryKind, list_dir};
use crate::tokenizer;

const INDENT_UNIT: &str = "    ";

/// A source file's text with its line index, built once per file so
/// line lookups by number don't rescan the text.
#[derive(Debug)]
pub struct Source {
    text: Arc<str>,
    line_ranges: Box<[Range<usize>]>,
    /// Prefix count of non-blank lines; element 0 is the empty prefix.
    non_blank_prefix: Box<[usize]>,
}

impl std::ops::Deref for Source {
    type Target = str;

    fn deref(&self) -> &str {
        &self.text
    }
}

impl Source {
    pub fn new(text: Arc<str>) -> Self {
        let base = text.as_ptr() as usize;
        let mut line_ranges = Vec::new();
        let mut non_blank_prefix = vec![0];
        for line in text.lines() {
            let start = line.as_ptr() as usize - base;
            line_ranges.push(start..start + line.len());
            non_blank_prefix.push(
                non_blank_prefix.last().copied().unwrap_or(0)
                    + usize::from(!line.trim().is_empty()),
            );
        }
        Self {
            text,
            line_ranges: line_ranges.into_boxed_slice(),
            non_blank_prefix: non_blank_prefix.into_boxed_slice(),
        }
    }

    /// 1-based line `number`, without its line terminator.
    pub fn line(&self, number: usize) -> Option<&str> {
        let range = self.line_ranges.get(number.checked_sub(1)?)?.clone();
        self.text.get(range)
    }

    pub fn line_count(&self) -> usize {
        self.line_ranges.len()
    }

    /// Whether the inclusive 1-based range holds a non-blank source line.
    fn gap_has_content(&self, start: usize, end: usize) -> bool {
        if start == 0 || start > end || start > self.line_count() {
            return false;
        }
        let end = end.min(self.line_count());
        self.non_blank_prefix[end] > self.non_blank_prefix[start - 1]
    }
}

/// Shared source-file cache — read and index each file at most once per run.
#[derive(Clone, Debug, Default)]
pub struct SourceCache(Rc<RefCell<HashMap<PathBuf, Arc<Source>>>>);

impl SourceCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read `path`, caching. Returns `None` on I/O error.
    pub fn get(&self, path: &Path) -> Option<Arc<Source>> {
        if let Some(cached) = self.0.borrow().get(path) {
            return Some(cached.clone());
        }
        let text = std::fs::read_to_string(path).ok()?;
        let source = Arc::new(Source::new(Arc::from(text)));
        self.0
            .borrow_mut()
            .insert(path.to_path_buf(), source.clone());
        Some(source)
    }

    /// Insert a pre-loaded source. Idempotent.
    pub fn insert(&self, path: PathBuf, source: Arc<str>) {
        self.0
            .borrow_mut()
            .entry(path)
            .or_insert_with(|| Arc::new(Source::new(source)));
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
    /// Memo for "does this file hold anything on disk" — one `stat` per
    /// file, asked once per render and once per scheduler cost probe.
    /// Directories go through [`Self::dir_entry_counts`], which answers
    /// the same question and more.
    entry_non_empty: RefCell<HashMap<PathBuf, bool>>,
    /// Memo for "how many entries does this directory show" — the
    /// denominator behind [`Self::listing_partial`].
    dir_entry_counts: RefCell<HashMap<PathBuf, usize>>,
    /// Same ignore rules discovery walks under, so the elision marker
    /// means "content precis is withholding" rather than "content precis
    /// would never show". A directory holding only ignored entries is
    /// empty as far as the reader is concerned, and marking it would
    /// promise substance that no budget can buy.
    ///
    /// Shared rather than owned: the filter's caches are what keep the
    /// "is anything visible beneath this directory" question from
    /// costing a subtree walk, and a scheduler cost probe builds one of
    /// these trees per call — an owned filter would throw the answers
    /// away every time.
    dir_filter: Rc<DirFilter>,
}

impl RenderedTree {
    pub fn new(root: PathBuf, source_cache: SourceCache) -> Self {
        let dir_filter = Rc::new(DirFilter::new(&root));
        Self::with_filter(root, source_cache, dir_filter)
    }

    pub fn with_filter(
        root: PathBuf,
        source_cache: SourceCache,
        dir_filter: Rc<DirFilter>,
    ) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(root.clone(), TreeNode::empty_dir());
        Self {
            root,
            nodes,
            source_cache,
            entry_non_empty: RefCell::new(HashMap::new()),
            dir_entry_counts: RefCell::new(HashMap::new()),
            dir_filter,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Marginal cost of applying `content` — exact tokens.
    pub fn marginal_cost(&self, content: &BatchContent) -> Cost {
        let mut total = Cost::default();
        self.visit_atom_costs(content, tokenizer::count, |c| total = total + c);
        total
    }

    /// Approximate token count (`bytes / k`) — for approx ranking.
    pub fn marginal_cost_approx(&self, content: &BatchContent) -> usize {
        let mut tokens: usize = 0;
        self.visit_atom_costs(content, tokenizer::approx_count, |c| tokens += c.tokens);
        tokens
    }

    /// Marginal-cost visitor (per FS entry / per touched file) —
    /// `tokens` chooses exact vs approx counting; everything else is
    /// identical.
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

    /// Whether the node at `path` puts anything of its own below its
    /// entry row — listed children for a directory, source rows for a
    /// file. Ellipsis records are gap occupants, not rows.
    fn renders_content(&self, path: &Path) -> bool {
        match self.nodes.get(path) {
            Some(TreeNode::Dir { children }) => !children.is_empty(),
            Some(TreeNode::File { content }) => content.values().any(is_anchor_record),
            None => false,
        }
    }

    /// Whether `path` holds anything at all on disk. Guards the elision
    /// marker so a genuinely empty directory or zero-byte file keeps
    /// rendering as a bare name.
    fn entry_non_empty(&self, path: &Path, kind: EntryKind) -> bool {
        // Directories answer from the entry-count memo — a second bool
        // memo over the same key would be `count > 0` restated.
        if matches!(kind, EntryKind::Directory) {
            return self.dir_entry_count(path) > 0;
        }
        if let Some(&known) = self.entry_non_empty.borrow().get(path) {
            return known;
        }
        let non_empty = std::fs::metadata(path).is_ok_and(|m| m.len() > 0);
        self.entry_non_empty
            .borrow_mut()
            .insert(path.to_path_buf(), non_empty);
        non_empty
    }

    /// Entries `dir` shows through the walk's ignore rules — the same
    /// set a listing batch draws from, so it is the denominator for
    /// "is this listing complete".
    fn dir_entry_count(&self, dir: &Path) -> usize {
        if let Some(&known) = self.dir_entry_counts.borrow().get(dir) {
            return known;
        }
        let count = list_dir(dir, &self.dir_filter).len();
        self.dir_entry_counts
            .borrow_mut()
            .insert(dir.to_path_buf(), count);
        count
    }

    /// Whether `path`'s entry row carries the elision marker: it shows
    /// none of its contents, but there are contents to show.
    fn entry_elided(&self, path: &Path, kind: EntryKind) -> bool {
        !self.renders_content(path) && self.entry_non_empty(path, kind)
    }

    /// Marginal cost of the elision marker on `path`'s *own* entry row,
    /// or `None` when there is no such row (the root) or it isn't marked.
    /// Listing a directory's children — or admitting a file's first source
    /// row — takes this marker back off the parent-emitted row, so both
    /// cost paths refund it.
    fn entry_marker_cost<T: Fn(&str) -> usize>(&self, path: &Path, tokens: &T) -> Option<Cost> {
        if path == self.root {
            return None;
        }
        let parent = path.parent()?;
        let name = path.file_name()?.to_str()?;
        let TreeNode::Dir { children } = self.nodes.get(parent)? else {
            return None;
        };
        let kind = *children.get(name)?;
        if !self.entry_elided(path, kind) {
            return None;
        }
        let indent_depth = self.depth_from_root(parent);
        let marked = format_entry_row(name, kind, indent_depth, true);
        let plain = format_entry_row(name, kind, indent_depth, false);
        Some(Cost {
            tokens: tokens(&marked).saturating_sub(tokens(&plain)),
            bytes: marked.len().saturating_sub(plain.len()),
        })
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
            let probed = list_dir(parent, &self.dir_filter);
            // Signed per-group accounting: new entry rows, minus the
            // elision marker the parent's own row sheds once it renders
            // children, plus the change in its trailing partial-listing
            // marker.
            let mut d_tokens: isize = 0;
            let mut d_bytes: isize = 0;
            let listed_before = already_listed.map_or(0, |c| c.len());
            let mut new_rows = 0usize;
            for p in paths {
                let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };
                if already_listed.is_some_and(|c| c.contains_key(name)) {
                    continue;
                }
                let kind = probed.get(name).copied().unwrap_or(EntryKind::File);
                let child = parent.join(name);
                let row =
                    format_entry_row(name, kind, indent_depth, self.entry_elided(&child, kind));
                d_tokens += tokens(&row) as isize;
                d_bytes += row.len() as isize;
                new_rows += 1;
            }
            if new_rows > 0
                && let Some(marker) = self.entry_marker_cost(parent, tokens)
            {
                d_tokens -= marker.tokens as isize;
                d_bytes -= marker.bytes as isize;
            }
            let shown = self.dir_entry_count(parent);
            let partial_delta = isize::from(listing_partial(listed_before + new_rows, shown))
                - isize::from(listing_partial(listed_before, shown));
            if partial_delta != 0 {
                let row = format_marker_row(indent_depth);
                d_tokens += partial_delta * tokens(&row) as isize;
                d_bytes += partial_delta * row.len() as isize;
            }
            visit(Cost {
                tokens: d_tokens.max(0) as usize,
                bytes: d_bytes.max(0) as usize,
            });
        }
    }

    fn visit_span_atom_costs<F, T>(&self, spans: &[Span], tokens: &T, visit: &mut F)
    where
        F: FnMut(Cost),
        T: Fn(&str) -> usize,
    {
        // Group by path — costs are accounted per file (row deltas +
        // synthesized-marker delta), so source/indent lookups and the
        // anchor sets are built once per file.
        // Keep spans compressed here: expanding every range into cloned
        // `(PathBuf, line, Render)` rows dominated repeated scheduler probes
        // on large roster chunks.
        let mut by_path: BTreeMap<&Path, Vec<&Span>> = BTreeMap::new();
        for span in spans {
            by_path.entry(span.path.as_path()).or_default().push(span);
        }

        for (path, file_spans) in by_path {
            let source = self.source_cache.get(path);
            let indent_depth = self.depth_from_root(path);
            let existing = match self.nodes.get(path) {
                Some(TreeNode::File { content }) => Some(content),
                _ => None,
            };
            // Signed per-file accounting: row deltas for the batch's
            // lines (Ellipsis records never render rows of their own)
            // plus the change in synthesized `…` marker rows — the
            // renderer invariant means marker count is a function of
            // the anchor set, so the delta is exact given the current
            // tree state.
            let mut d_tokens: isize = 0;
            let mut d_bytes: isize = 0;
            for span in &file_spans {
                for line_num in span.start..=span.end {
                    debug_assert!(
                        source.is_none()
                            || line_num >= 1 && line_num <= source.as_ref().unwrap().line_count(),
                        "span line {} out of range (1..={}) for {} — schema load should have caught this",
                        line_num,
                        source.as_ref().map_or(0, |s| s.line_count()),
                        path.display(),
                    );
                    let source_line = source.as_ref().and_then(|s| s.line(line_num)).unwrap_or("");
                    if !matches!(span.render, Render::Ellipsis) {
                        let new_row =
                            format_line_row(line_num, &span.render, source_line, indent_depth);
                        d_tokens += tokens(&new_row) as isize;
                        d_bytes += new_row.len() as isize;
                    }
                    if let Some(existing) = existing
                        && let Some(old) = existing.get(&line_num)
                        && !matches!(old.render, Render::Ellipsis)
                    {
                        let old_row =
                            format_line_row(line_num, &old.render, source_line, indent_depth);
                        d_tokens -= tokens(&old_row) as isize;
                        d_bytes -= old_row.len() as isize;
                    }
                }
            }
            let delta = local_marker_delta(existing, &file_spans, source.as_deref());
            if delta.gaps != 0 {
                let marker_row = format_marker_row(indent_depth);
                d_tokens += delta.gaps * tokens(&marker_row) as isize;
                d_bytes += delta.gaps * marker_row.len() as isize;
            }
            if delta.entry != 0
                && let Some(marker) = self.entry_marker_cost(path, tokens)
            {
                d_tokens += delta.entry * marker.tokens as isize;
                d_bytes += delta.entry * marker.bytes as isize;
            }
            visit(Cost {
                tokens: d_tokens.max(0) as usize,
                bytes: d_bytes.max(0) as usize,
            });
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
        // Kind lookup only, for names the batch already carries. The
        // walk's own filter rather than a bare one: it is what decided
        // those names in the first place, its caches make the probe
        // nearly free, and a bare filter would have no walk root to
        // resolve a linked entry's kind against.
        let probed = list_dir(parent, &self.dir_filter);
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
            let child = path.join(name);
            out.push_str(&format_entry_row(
                name,
                *kind,
                indent_depth,
                self.entry_elided(&child, *kind),
            ));
            match kind {
                EntryKind::Directory => {
                    self.render_dir(&path.join(name), indent_depth + 1, out);
                }
                EntryKind::File => {
                    self.render_file(&path.join(name), indent_depth + 1, out);
                }
            }
        }
        if listing_partial(children.len(), self.dir_entry_count(path)) {
            out.push_str(&format_marker_row(indent_depth));
        }
    }

    fn render_file(&self, path: &Path, indent_depth: usize, out: &mut String) {
        let Some(TreeNode::File { content }) = self.nodes.get(path) else {
            return;
        };
        // Elision marking is a renderer invariant: a `…` row appears
        // iff the adjacent elided source is non-blank (leading,
        // between anchors, trailing). Author-supplied Ellipsis records
        // are gap occupants, not rows — synthesis subsumes them.
        let anchors = content_anchor_lines(content);
        if anchors.is_empty() {
            // Nothing of this file is shown; the marker rides on the
            // entry row `render_dir` emitted, not on a row of its own.
            return;
        }
        let source = self.source_cache.get(path);
        let marker_row = format_marker_row(indent_depth);
        walk_anchor_gaps(&anchors, source.as_deref(), |event| match event {
            GapWalkEvent::Gap => out.push_str(&marker_row),
            GapWalkEvent::Anchor(number) => {
                let source_line = source.as_ref().and_then(|s| s.line(number)).unwrap_or("");
                out.push_str(&format_line_row(
                    number,
                    &content[&number].render,
                    source_line,
                    indent_depth,
                ));
            }
        });
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
        .filter(|(_, r)| is_anchor_record(r))
        .map(|(n, _)| *n)
        .collect()
}

fn is_anchor_record(record: &LineRecord) -> bool {
    !matches!(record.render, Render::Ellipsis)
}

/// Marker change caused by replacing `entries` in one file: `gaps` counts
/// synthesized in-file `…` rows, `entry` the file's own entry-row elision
/// marker (which exists exactly while the file shows no source at all).
#[derive(Default)]
struct MarkerDelta {
    gaps: isize,
    entry: isize,
}

/// Exact marker change caused by replacing `entries` in one file. Only the
/// gap bounded by the nearest unchanged anchors can change, so this avoids
/// cloning and rescanning the file's full accumulated anchor set for every
/// scheduler probe.
fn local_marker_delta(
    existing: Option<&BTreeMap<usize, LineRecord>>,
    spans: &[&Span],
    source: Option<&Source>,
) -> MarkerDelta {
    let Some(first_changed) = spans.iter().map(|span| span.start).min() else {
        return MarkerDelta::default();
    };
    let last_changed = spans
        .iter()
        .map(|span| span.end)
        .max()
        .unwrap_or(first_changed);

    let left = existing.and_then(|content| {
        content
            .range(..first_changed)
            .rev()
            .find(|(_, record)| is_anchor_record(record))
            .map(|(line, _)| *line)
    });
    let right = existing.and_then(|content| {
        content
            .range((
                std::ops::Bound::Excluded(last_changed),
                std::ops::Bound::Unbounded,
            ))
            .find(|(_, record)| is_anchor_record(record))
            .map(|(line, _)| *line)
    });

    let mut before = BTreeSet::new();
    if let Some(content) = existing {
        before.extend(
            content
                .range(first_changed..=last_changed)
                .filter(|(_, record)| is_anchor_record(record))
                .map(|(line, _)| *line),
        );
    }
    let mut after = before.clone();
    for span in spans {
        for line in span.start..=span.end {
            if matches!(span.render, Render::Ellipsis) {
                after.remove(&line);
            } else {
                after.insert(line);
            }
        }
    }

    // `left`/`right` are the nearest anchors outside the changed range, so
    // either being present means the batch can't take the file's last
    // anchor away. Together with the local sets that decides whether the
    // file shows any source at all before and after — which is what both
    // the gap rows and the entry marker hinge on.
    let outer_anchors = left.is_some() || right.is_some();
    let shown_before = outer_anchors || !before.is_empty();
    let shown_after = outer_anchors || !after.is_empty();

    let gaps = source.map_or(0, |source| {
        let before_count = if shown_before {
            marker_count_between(left, right, &before, source)
        } else {
            0
        };
        let after_count = if shown_after {
            marker_count_between(left, right, &after, source)
        } else {
            0
        };
        after_count as isize - before_count as isize
    });
    MarkerDelta {
        gaps,
        entry: isize::from(!shown_after) - isize::from(!shown_before),
    }
}

/// Count non-blank gaps between unchanged outer anchors. `inner` contains
/// every anchor inside those bounds for the tree state being measured.
fn marker_count_between(
    left: Option<usize>,
    right: Option<usize>,
    inner: &BTreeSet<usize>,
    source: &Source,
) -> usize {
    let mut count = 0;
    let mut previous = left.unwrap_or(0);
    for &anchor in inner {
        if anchor > previous + 1 && source.gap_has_content(previous + 1, anchor - 1) {
            count += 1;
        }
        previous = anchor;
    }
    let gap_end = right
        .map(|anchor| anchor.saturating_sub(1))
        .unwrap_or_else(|| source.line_count());
    if gap_end > previous && source.gap_has_content(previous + 1, gap_end) {
        count += 1;
    }
    count
}

enum GapWalkEvent {
    Gap,
    Anchor(usize),
}

/// The single traversal behind both marker counting and row emission:
/// visits each anchor in order and each elision gap (leading / between
/// anchors / trailing) that holds non-blank source. Charged tokens ==
/// rendered tokens depends on cost and render never walking gaps
/// differently, so both must go through here.
fn walk_anchor_gaps(
    anchors: &BTreeSet<usize>,
    source: Option<&Source>,
    mut visit: impl FnMut(GapWalkEvent),
) {
    let mut prev = 0usize;
    for &a in anchors {
        if a > prev + 1 && source.is_some_and(|s| s.gap_has_content(prev + 1, a - 1)) {
            visit(GapWalkEvent::Gap);
        }
        visit(GapWalkEvent::Anchor(a));
        prev = a;
    }
    if source.is_some_and(|s| s.gap_has_content(prev + 1, s.line_count())) {
        visit(GapWalkEvent::Gap);
    }
}

/// One synthesized elision marker row.
/// Whether a directory showing `listed` of its `shown` entries needs a
/// trailing `…` row. Same argument as the entry-row marker: a listing
/// cut short is otherwise byte-identical to a complete one, and readers
/// take marker-absence as proof of completeness. A directory showing
/// none of its entries is covered by the marker on its own entry row
/// instead.
fn listing_partial(listed: usize, shown: usize) -> bool {
    listed > 0 && listed < shown
}

fn format_marker_row(indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push('…');
    s.push('\n');
    s
}

/// One tree entry row. `elided` trails the same `…` glyph the in-file
/// gap rows use: the entry holds content and none of it is shown. Without
/// it a pruned entry is byte-identical to an empty one, and readers take
/// marker-absence as proof of emptiness.
fn format_entry_row(name: &str, kind: EntryKind, indent_depth: usize, elided: bool) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    for ch in name.chars() {
        if ch.is_control() {
            s.extend(ch.escape_default());
        } else {
            s.push(ch);
        }
    }
    if matches!(kind, EntryKind::Directory) {
        s.push('/');
    }
    if elided {
        s.push_str(" …");
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
    // Right-align the line number so the code column doesn't shift at
    // digit boundaries (matters for indentation-significant sources).
    // Fixed width keeps each row's cost independent of the file's
    // other rendered rows; ≥5-digit numbers just extend the row.
    let gutter = format!("{number:>4}");
    match render {
        Render::Ellipsis => {
            // Unreachable from render/cost paths — Ellipsis records
            // never render rows of their own; `render_file` synthesizes
            // gap markers instead. Defensive marker shape in release.
            debug_assert!(false, "format_line_row called with Render::Ellipsis");
            s.push('…');
        }
        Render::Full => {
            s.push_str(&gutter);
            s.push('→');
            s.push_str(source_line);
        }
        Render::Truncated { pattern } => {
            s.push_str(&gutter);
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

    /// Source used by the gap-marker tests: line 2 blank, lines 4-5 a
    /// non-blank elided gap, line 6 the last line.
    fn gap_fixture() -> (SourceCache, PathBuf) {
        let cache = SourceCache::new();
        let path = PathBuf::from(format!("{STUB_DIR}/f.c"));
        cache.insert(
            path.clone(),
            Arc::from("fn a();\n\nfn b();\nhidden1\nhidden2\nfn c();\n"),
        );
        (cache, path)
    }

    /// Real on-disk tree — the elision marker asks the filesystem
    /// whether an unopened entry holds anything, so these cases can't
    /// run against the stub paths the gap tests use.
    ///
    /// `listed/` and `shown.rs` end up rendering content; `pruned/` and
    /// `hidden.rs` hold content that stays hidden; `empty/` and
    /// `zero.rs` hold nothing at all.
    fn disk_fixture() -> tempfile::TempDir {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::create_dir(root.join("pruned")).unwrap();
        std::fs::write(root.join("pruned/a.rs"), "fn a() {}\n").unwrap();
        std::fs::write(root.join("pruned/b.rs"), "fn b() {}\n").unwrap();
        std::fs::create_dir(root.join("listed")).unwrap();
        std::fs::write(root.join("listed/c.rs"), "fn c() {}\n").unwrap();
        std::fs::create_dir(root.join("empty")).unwrap();
        std::fs::write(root.join("zero.rs"), "").unwrap();
        std::fs::write(root.join("hidden.rs"), "fn hidden() {}\n").unwrap();
        std::fs::write(root.join("shown.rs"), "fn shown() {}\n").unwrap();
        temp
    }

    fn dir_listing(parent: PathBuf, entries: &[&str]) -> BatchContent {
        BatchContent::Fs {
            groups: vec![FsGroup {
                parent,
                entries: FsEntries::Listed(entries.iter().map(PathBuf::from).collect()),
            }],
        }
    }

    const DISK_FIXTURE_ROOT_ENTRIES: &[&str] = &[
        "empty",
        "hidden.rs",
        "listed",
        "pruned",
        "shown.rs",
        "zero.rs",
    ];

    #[test]
    fn render_entry_marker_separates_pruned_entries_from_empty_ones() {
        let temp = disk_fixture();
        let root = temp.path().to_path_buf();
        let mut tree = RenderedTree::new(root.clone(), SourceCache::new());
        tree.apply(
            &dir_listing(root.clone(), DISK_FIXTURE_ROOT_ENTRIES),
            BatchId::new(0),
            |_| true,
        );
        tree.apply(
            &dir_listing(root.join("listed"), &["c.rs"]),
            BatchId::new(1),
            |_| true,
        );
        tree.apply(
            &one_span(root.join("shown.rs"), 1, Render::Full),
            BatchId::new(2),
            |_| true,
        );

        let out = tree.render();
        // Content is hidden behind these, so they say so.
        assert!(out.contains("pruned/ …\n"), "output:\n{out}");
        assert!(out.contains("hidden.rs …\n"), "output:\n{out}");
        // These hold nothing; marking them would be the opposite lie.
        assert!(out.contains("empty/\n"), "output:\n{out}");
        assert!(out.contains("zero.rs\n"), "output:\n{out}");
        // These render their contents, so the marker comes back off.
        assert!(out.contains("listed/\n"), "output:\n{out}");
        assert!(out.contains("shown.rs\n"), "output:\n{out}");
    }

    #[test]
    fn render_entry_marker_charged_cost_matches_rendered_rows() {
        let temp = disk_fixture();
        let root = temp.path().to_path_buf();
        let mut tree = RenderedTree::new(root.clone(), SourceCache::new());
        // Each step takes a marker back off a row an earlier step paid
        // for, so a stale charge shows up as a mismatch here.
        let steps = [
            dir_listing(root.clone(), DISK_FIXTURE_ROOT_ENTRIES),
            dir_listing(root.join("listed"), &["c.rs"]),
            one_span(root.join("shown.rs"), 1, Render::Full),
            dir_listing(root.join("pruned"), &["a.rs", "b.rs"]),
        ];
        let mut charged = 0;
        for (i, content) in steps.iter().enumerate() {
            charged += tree.marginal_cost(content).tokens;
            tree.apply(content, BatchId::new(i), |_| true);
        }
        let rendered: usize = tree
            .render()
            .lines()
            .map(|line| tokenizer::count(&format!("{line}\n")))
            .sum();
        assert_eq!(charged, rendered, "output:\n{}", tree.render());
    }

    #[test]
    fn render_gap_marker_synthesized_iff_elided_source_nonblank() {
        let (cache, path) = gap_fixture();
        let mut tree = RenderedTree::new(stub_dir(), cache);
        tree.apply(&listing(&["f.c"]), BatchId::new(0), |_| true);
        // Full lines only — no author Ellipsis records at all.
        for line in [1, 3, 6] {
            let content = one_span(path.clone(), line, Render::Full);
            assert!(tree.marginal_cost(&content).tokens > 0);
            tree.apply(&content, BatchId::new(line), |_| true);
        }
        let out = tree.render();
        // Blank-only gap (line 2): no marker. Non-blank gap (4-5):
        // marker synthesized despite no Ellipsis record.
        assert_eq!(out.matches('…').count(), 1, "output:\n{out}");
        let idx_b = out.find("3→fn b();").unwrap();
        let idx_marker = out.find('…').unwrap();
        let idx_c = out.find("6→fn c();").unwrap();
        assert!(idx_b < idx_marker && idx_marker < idx_c, "output:\n{out}");
    }

    #[test]
    fn render_gap_marker_ellipsis_records_cost_nothing_and_never_duplicate() {
        let (cache, path) = gap_fixture();
        let mut tree = RenderedTree::new(stub_dir(), cache);
        tree.apply(&listing(&["f.c"]), BatchId::new(0), |_| true);
        for line in [1, 3, 6] {
            tree.apply(
                &one_span(path.clone(), line, Render::Full),
                BatchId::new(line),
                |_| true,
            );
        }
        let before = tree.render();
        // Author Ellipsis records inside blank (2) and non-blank (4)
        // gaps: zero marginal cost, output byte-identical.
        for line in [2, 4] {
            let content = one_span(path.clone(), line, Render::Ellipsis);
            let cost = tree.marginal_cost(&content);
            assert_eq!(cost.tokens, 0, "ellipsis at {line} must cost nothing");
            assert_eq!(cost.bytes, 0);
            tree.apply(&content, BatchId::new(10 + line), |_| true);
        }
        assert_eq!(tree.render(), before);
    }

    #[test]
    fn render_gap_marker_leading_and_trailing() {
        let (cache, path) = gap_fixture();
        let mut tree = RenderedTree::new(stub_dir(), cache);
        tree.apply(&listing(&["f.c"]), BatchId::new(0), |_| true);
        // Only line 3 rendered: elided source on both sides is
        // non-blank (line 1; lines 4-6) → leading + trailing markers.
        tree.apply(
            &one_span(path.clone(), 3, Render::Full),
            BatchId::new(1),
            |_| true,
        );
        let out = tree.render();
        assert_eq!(out.matches('…').count(), 2, "output:\n{out}");
        let idx_row = out.find("3→fn b();").unwrap();
        let first = out.find('…').unwrap();
        let last = out.rfind('…').unwrap();
        assert!(first < idx_row && idx_row < last, "output:\n{out}");
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
    fn render_listing_escapes_control_characters_in_names() {
        let cache = SourceCache::new();
        let mut tree = RenderedTree::new(stub_dir(), cache);
        let content = listing(&["name\nwith\ttabs.md"]);

        let cost = tree.marginal_cost(&content);
        tree.apply(&content, BatchId::new(0), |_| true);

        let rendered = tree.render();
        assert_eq!(rendered, "name\\nwith\\ttabs.md\n");
        assert_eq!(cost.bytes, rendered.len());
        assert_eq!(cost.tokens, tokenizer::count(&rendered));
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
