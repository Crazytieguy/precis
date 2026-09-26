//! Rendered tree — the scheduler's "what have we committed to output" state.
//! Two node kinds (directory, file); applies batches (which carry spans,
//! not materialized text); computes marginal cost; renders to a string by
//! reading source through a shared cache.
//!
//! Span materialization happens here — walkers emit `Span { path, start,
//! end, render }`, the tree stores per-line `(owner: BatchId, render: Render)`
//! records, and `render()` reads source to produce the final text.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::io::Read as _;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use crate::batch::BatchId;
use crate::content::{
    BatchContent, FsEntries, FsGroup, Render, Span, explode_spans, with_truncate_regex,
};
use crate::fs_util::{DirFilter, EntryKind, list_dir, lists_nothing};
use crate::tokenizer;

const INDENT_UNIT: &str = "  ";

/// Longest source text a `Full` row shows; the rest of the line is
/// replaced by `…`.
const MAX_ROW_CHARS: usize = 500;

/// Directory entries by parent, for listings a cost probe adds to the tree.
type Listings = HashMap<PathBuf, BTreeMap<String, EntryKind>>;

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

    /// Memory the text and its line index take.
    fn held_bytes(&self) -> usize {
        self.text.len()
            + std::mem::size_of_val(&*self.line_ranges)
            + std::mem::size_of_val(&*self.non_blank_prefix)
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

/// Largest file precis reads. Past it a file is treated like a generated
/// or minified one and yields no batches: the 186-repo robustness sweep's
/// largest hand-written single-file library is 4.1 MB (`miniaudio.h`),
/// while a 25.9 MB generated `parser.c` cost 900 MB and seconds to parse.
pub(crate) const MAX_SOURCE_BYTES: usize = 8 * 1024 * 1024;

/// Most a run's [`SourceCache`] holds, text and line index together; a
/// read past it is refused. The largest run over the 286-repo robustness
/// and eval corpora, at a 1M-token budget, holds 82 MB of text in 2.2M
/// lines (134 MB with the index); without a cap, a few hundred generated
/// one-character-per-line files in one listing held 1.8 GB.
const SOURCE_CACHE_BYTE_CAP: usize = 256 * 1024 * 1024;

const UTF8_BYTE_ORDER_MARK: &[u8] = b"\xEF\xBB\xBF";

/// Shared source-file cache — read and index each file at most once per run.
#[derive(Clone, Debug, Default)]
pub struct SourceCache(Rc<RefCell<CachedSources>>);

#[derive(Debug, Default)]
struct CachedSources {
    by_path: HashMap<PathBuf, Arc<Source>>,
    /// Summed [`Source::held_bytes`] of `by_path`.
    held_bytes: usize,
}

impl SourceCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read `path`, caching. `None` unless it is a regular file of at
    /// most [`MAX_SOURCE_BYTES`] holding no NUL byte (binary), not
    /// refused ([`crate::walker::is_refused_by_name`],
    /// [`crate::walker::holds_private_key`]), and within what is left of
    /// [`SOURCE_CACHE_BYTE_CAP`]: nothing else is source, and a FIFO or
    /// device named by a link could block or read forever. Bytes that
    /// aren't UTF-8 (a Latin-1 name in a license header) read as U+FFFD
    /// rather than hiding the whole file. A leading byte-order mark is
    /// dropped, so the first line reads like any other.
    pub fn get(&self, path: &Path) -> Option<Arc<Source>> {
        self.read(path, MAX_SOURCE_BYTES, false)
    }

    /// [`Self::get`] of a file at most `byte_gate` long, a cached one
    /// by the length of its text.
    pub fn get_within(&self, path: &Path, byte_gate: usize) -> Option<Arc<Source>> {
        self.read(path, byte_gate.min(MAX_SOURCE_BYTES), false)
    }

    /// `path`'s source as far as `head_bytes` reach into it: whole when it
    /// is already cached or fits, else only its head. The head runs a few
    /// bytes past `head_bytes` so the line the cut falls in stays too long
    /// to select. A head is cached under `path` itself, where the renderer
    /// reads it. [`Self::get`] and [`Self::get_within`] would serve that
    /// head as the whole file; no walker reads one only because the
    /// plaintext floor, its one caller, runs after every expansion is done.
    pub fn get_head(&self, path: &Path, head_bytes: usize) -> Option<Arc<Source>> {
        self.read(path, head_bytes, true)
    }

    fn read(&self, path: &Path, limit: usize, allow_cut: bool) -> Option<Arc<Source>> {
        if let Some(cached) = self.cached(path) {
            let whole_read = limit == MAX_SOURCE_BYTES;
            return (allow_cut || whole_read || cached.len() <= limit).then_some(cached);
        }
        if crate::walker::is_refused_by_name(path) {
            return None;
        }
        let metadata = std::fs::metadata(path).ok()?;
        if !metadata.is_file()
            || (!allow_cut && metadata.len() as usize > limit.min(self.bytes_left()))
        {
            return None;
        }
        let mut bytes = Vec::new();
        std::fs::File::open(path)
            .ok()?
            .take(limit as u64 + 4)
            .read_to_end(&mut bytes)
            .ok()?;
        let cut = bytes.len() > limit;
        if (cut && !allow_cut) || bytes.contains(&0) {
            return None;
        }
        let text =
            String::from_utf8_lossy(bytes.strip_prefix(UTF8_BYTE_ORDER_MARK).unwrap_or(&bytes));
        if crate::walker::holds_private_key(&text, cut) {
            return None;
        }
        self.admit(path.to_path_buf(), Source::new(Arc::from(text)))
    }

    /// `path`'s source if it has already been read, without reading it.
    pub fn cached(&self, path: &Path) -> Option<Arc<Source>> {
        self.0.borrow().by_path.get(path).cloned()
    }

    /// Insert a pre-loaded source, unless it is past what is left of
    /// [`SOURCE_CACHE_BYTE_CAP`]. Idempotent.
    pub fn insert(&self, path: PathBuf, source: Arc<str>) {
        if self.cached(&path).is_none() {
            self.admit(path, Source::new(source));
        }
    }

    fn bytes_left(&self) -> usize {
        SOURCE_CACHE_BYTE_CAP.saturating_sub(self.0.borrow().held_bytes)
    }

    fn admit(&self, path: PathBuf, source: Source) -> Option<Arc<Source>> {
        let held_bytes = source.held_bytes();
        if held_bytes > self.bytes_left() {
            return None;
        }
        let source = Arc::new(source);
        let mut cache = self.0.borrow_mut();
        cache.held_bytes += held_bytes;
        cache.by_path.insert(path, source.clone());
        Some(source)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cost {
    pub tokens: usize,
    /// UTF-16 code units, see [`char_units`].
    pub chars: usize,
}

/// Length as JavaScript's `String.length` counts it (UTF-16 code units),
/// which is the unit Claude Code caps hook output in.
pub fn char_units(text: &str) -> usize {
    text.encode_utf16().count()
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

pub struct RenderedTree {
    root: PathBuf,
    nodes: HashMap<PathBuf, TreeNode>,
    source_cache: SourceCache,
    /// Memo for "is this file empty on disk" — one `stat` per
    /// file, asked once per render and once per scheduler cost probe.
    /// Directories answer from their listing instead.
    file_empty: RefCell<HashMap<PathBuf, bool>>,
    /// Same ignore rules discovery walks under: a directory holding only
    /// ignored entries has nothing any budget could show, so it is marked
    /// empty rather than read as unexpanded. Shared with the walk, so
    /// the filter's listing memo serves both.
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
            file_empty: RefCell::new(HashMap::new()),
            dir_filter,
        }
    }

    /// Marginal cost of applying `content`.
    pub fn marginal_cost(&self, content: &BatchContent) -> Cost {
        match content {
            BatchContent::Fs { groups } => self.fs_marginal_cost(groups),
            BatchContent::Lines { spans, .. } => self.spans_marginal_cost(spans),
        }
    }

    /// Longest prefix of `content` — its listing entries, source
    /// lines or line units, in order — whose marginal cost `fits`, with
    /// that cost. `None` when not even one fits.
    pub fn affordable_prefix(
        &self,
        content: &BatchContent,
        fits: impl Fn(Cost) -> bool,
    ) -> Option<(BatchContent, Cost)> {
        // Rows are nearly independent, so cost climbs with the prefix
        // length — binary-search the boundary, keeping the longest
        // prefix that fit. Nothing depends on the search finding the
        // exact boundary: whatever it returns was measured.
        let (mut lo, mut hi) = (0usize, content_len(content) + 1);
        let mut affordable = None;
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let candidate = content_prefix(content, mid, &self.source_cache);
            let cost = self.marginal_cost(&candidate);
            if fits(cost) {
                lo = mid;
                affordable = Some((candidate, cost));
            } else {
                hi = mid;
            }
        }
        affordable
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
            BatchContent::Lines { spans, .. } => self.apply_spans(spans, owner, &is_ancestor),
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        self.render_dir(&self.root, 0, &mut out);
        out
    }

    // ---- internal ----

    /// Whether `path` holds nothing any budget could show: a directory
    /// [`lists_nothing`] holds for, or a zero-byte file.
    fn entry_empty(&self, path: &Path, kind: EntryKind) -> bool {
        if matches!(kind, EntryKind::Directory) {
            return lists_nothing(path, &self.dir_filter);
        }
        if let Some(&known) = self.file_empty.borrow().get(path) {
            return known;
        }
        let empty = std::fs::metadata(path).is_ok_and(|m| m.len() == 0);
        self.file_empty
            .borrow_mut()
            .insert(path.to_path_buf(), empty);
        empty
    }

    fn fs_marginal_cost(&self, groups: &[FsGroup]) -> Cost {
        let mut added = Listings::new();
        for group in groups {
            let already_listed = self.tree_children(&group.parent);
            for (name, kind) in self.resolved_entries(group) {
                if already_listed.is_some_and(|c| c.contains_key(&name)) {
                    continue;
                }
                added
                    .entry(group.parent.clone())
                    .or_default()
                    .insert(name, kind);
            }
        }
        let none = Listings::new();
        // A new entry can extend an existing chain row instead of adding
        // a row of its own, so the delta is taken over whole rows: every
        // row holding a listed parent or a new entry, before and after.
        let mut rows_before = BTreeSet::new();
        let mut rows_after = BTreeSet::new();
        let mut d_tokens: isize = 0;
        let mut d_chars: isize = 0;
        let mut charge = |row: &str, sign: isize| {
            d_tokens += sign * tokenizer::count(row) as isize;
            d_chars += sign * char_units(row) as isize;
        };
        for (parent, names) in &added {
            rows_before.extend(self.row_head(parent, &none));
            rows_after.extend(self.row_head(parent, &added));
            for name in names.keys() {
                rows_after.extend(self.row_head(&parent.join(name), &added));
            }
            let shown = list_dir(parent, &self.dir_filter).len();
            let listed_before = self.tree_children(parent).map_or(0, BTreeMap::len);
            if listing_partial(listed_before, shown) {
                charge(&format_marker_row(self.child_indent(parent, &none)), -1);
            }
            if listing_partial(listed_before + names.len(), shown) {
                charge(&format_marker_row(self.child_indent(parent, &added)), 1);
            }
        }
        for head in &rows_before {
            charge(&self.chain_row(head, &none), -1);
        }
        for head in &rows_after {
            charge(&self.chain_row(head, &added), 1);
        }
        Cost {
            tokens: d_tokens.max(0) as usize,
            chars: d_chars.max(0) as usize,
        }
    }

    fn tree_children(&self, dir: &Path) -> Option<&BTreeMap<String, EntryKind>> {
        match self.nodes.get(dir) {
            Some(TreeNode::Dir { children }) => Some(children),
            _ => None,
        }
    }

    /// Kind of `path` if it is listed in the tree or in `added`.
    fn listed_kind(&self, path: &Path, added: &Listings) -> Option<EntryKind> {
        let parent = path.parent()?;
        let name = path.file_name()?.to_str()?;
        self.tree_children(parent)
            .and_then(|c| c.get(name))
            .or_else(|| added.get(parent).and_then(|c| c.get(name)))
            .copied()
    }

    /// The child `dir`'s row runs on into: its only entry on disk, when
    /// that entry is a listed directory. The root has no row to extend.
    fn chained_child(&self, dir: &Path, added: &Listings) -> Option<String> {
        if dir == self.root {
            return None;
        }
        let shown = list_dir(dir, &self.dir_filter);
        let (name, EntryKind::Directory) = shown.iter().next()? else {
            return None;
        };
        (shown.len() == 1 && self.listed_kind(&dir.join(name), added).is_some())
            .then(|| name.clone())
    }

    /// Indent of the rows listing `dir`'s entries: one step per row on
    /// the way down from the root.
    fn child_indent(&self, dir: &Path, added: &Listings) -> usize {
        let Ok(relative) = dir.strip_prefix(&self.root) else {
            return 0;
        };
        let mut indent = 0;
        let mut current = self.root.clone();
        for component in relative.components() {
            let chained = self
                .chained_child(&current, added)
                .is_some_and(|name| component.as_os_str() == name.as_str());
            indent += usize::from(!chained);
            current.push(component);
        }
        indent
    }

    /// First entry of the row `path` renders on, if it renders on one.
    fn row_head(&self, path: &Path, added: &Listings) -> Option<PathBuf> {
        self.listed_kind(path, added)?;
        let mut head = path.to_path_buf();
        while let Some(parent) = head.parent()
            && let Some(name) = self.chained_child(parent, added)
            && head.file_name().is_some_and(|n| n == name.as_str())
        {
            head.pop();
        }
        Some(head)
    }

    fn chain_row(&self, head: &Path, added: &Listings) -> String {
        let kind = self.listed_kind(head, added).unwrap_or(EntryKind::File);
        let indent = head
            .parent()
            .map_or(0, |parent| self.child_indent(parent, added));
        let (names, tail) = self.chain_from(head, kind, added);
        format_entry_row(&names, kind, indent, self.entry_empty(&tail, kind))
    }

    /// Names on the row headed by `head`, and the entry that ends it.
    fn chain_from(&self, head: &Path, kind: EntryKind, added: &Listings) -> (Vec<String>, PathBuf) {
        let mut names = vec![
            head.file_name()
                .map_or_else(String::new, |n| n.to_string_lossy().into_owned()),
        ];
        let mut tail = head.to_path_buf();
        if matches!(kind, EntryKind::Directory) {
            while let Some(name) = self.chained_child(&tail, added) {
                tail.push(&name);
                names.push(name);
            }
        }
        (names, tail)
    }

    fn spans_marginal_cost(&self, spans: &[Span]) -> Cost {
        // Group by path — costs are accounted per file (row deltas +
        // synthesized-marker delta), so source/indent lookups and the
        // anchor sets are built once per file.
        let mut by_path: BTreeMap<&Path, Vec<&Span>> = BTreeMap::new();
        for span in spans {
            by_path.entry(span.path.as_path()).or_default().push(span);
        }

        let mut total = Cost::default();
        for (path, file_spans) in by_path {
            let source = self.source_cache.get(path);
            let indent_depth = path
                .parent()
                .map_or(0, |parent| self.child_indent(parent, &Listings::new()) + 1);
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
            let mut d_chars: isize = 0;
            for span in &file_spans {
                for line_num in span.start..=span.end {
                    debug_assert!(
                        source
                            .as_ref()
                            .is_none_or(|s| (1..=s.line_count()).contains(&line_num)),
                        "span line {line_num} out of range for {} — schema load should have caught this",
                        path.display(),
                    );
                    let source_line = source.as_ref().and_then(|s| s.line(line_num)).unwrap_or("");
                    if !matches!(span.render, Render::Ellipsis) {
                        let new_row = format_line_row(
                            path,
                            line_num,
                            &span.render,
                            source_line,
                            indent_depth,
                        );
                        d_tokens += tokenizer::count(&new_row) as isize;
                        d_chars += char_units(&new_row) as isize;
                    }
                    if let Some(existing) = existing
                        && let Some(old) = existing.get(&line_num)
                        && !matches!(old.render, Render::Ellipsis)
                    {
                        let old_row =
                            format_line_row(path, line_num, &old.render, source_line, indent_depth);
                        d_tokens -= tokenizer::count(&old_row) as isize;
                        d_chars -= char_units(&old_row) as isize;
                    }
                }
            }
            let gap_delta = local_gap_marker_delta(existing, &file_spans, source.as_deref());
            if gap_delta != 0 {
                let marker_row = format_marker_row(indent_depth);
                d_tokens += gap_delta * tokenizer::count(&marker_row) as isize;
                d_chars += gap_delta * char_units(&marker_row) as isize;
            }
            total = total
                + Cost {
                    tokens: d_tokens.max(0) as usize,
                    chars: d_chars.max(0) as usize,
                };
        }
        total
    }

    /// The names `group` lists, each with its kind. Kind lookup only, for
    /// names the batch already carries. The walk's own filter rather than a
    /// bare one: it is what decided those names in the first place, its
    /// caches make the probe nearly free, and a bare filter would have no
    /// walk root to resolve a linked entry's kind against.
    fn resolved_entries(&self, group: &FsGroup) -> Vec<(String, EntryKind)> {
        let FsEntries::Listed(paths) = &group.entries else {
            debug_assert!(
                false,
                "unresolved FsEntries reached the renderer at {}",
                group.parent.display()
            );
            return Vec::new();
        };
        let probed = list_dir(&group.parent, &self.dir_filter);
        paths
            .iter()
            .filter_map(|p| {
                let name = p.file_name().and_then(|n| n.to_str())?.to_string();
                let kind = probed.get(&name).copied().unwrap_or(EntryKind::File);
                Some((name, kind))
            })
            .collect()
    }

    fn apply_fs_group(&mut self, group: &FsGroup) {
        let parent = &group.parent;
        let resolved = self.resolved_entries(group);
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
        let none = Listings::new();
        for (name, kind) in children {
            let (names, tail) = self.chain_from(&path.join(name), *kind, &none);
            out.push_str(&format_entry_row(
                &names,
                *kind,
                indent_depth,
                self.entry_empty(&tail, *kind),
            ));
            match kind {
                EntryKind::Directory => self.render_dir(&tail, indent_depth + 1, out),
                EntryKind::File => self.render_file(&tail, indent_depth + 1, out),
            }
        }
        if listing_partial(children.len(), list_dir(path, &self.dir_filter).len()) {
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
            return;
        }
        let cached = self.source_cache.get(path);
        let source = cached.as_deref();
        let marker_row = format_marker_row(indent_depth);
        walk_anchor_gaps(&anchors, (None, None), source, |event| match event {
            GapWalkEvent::Gap => out.push_str(&marker_row),
            GapWalkEvent::Anchor(number) => {
                let source_line = source.and_then(|s| s.line(number)).unwrap_or("");
                out.push_str(&format_line_row(
                    path,
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
            chars: self.chars + other.chars,
        }
    }
}

/// Listing entries or source lines in `content`.
fn content_len(content: &BatchContent) -> usize {
    match content {
        BatchContent::Fs { groups } => groups
            .iter()
            .map(|group| match &group.entries {
                FsEntries::Listed(paths) => paths.len(),
                FsEntries::All => 0,
            })
            .sum(),
        BatchContent::Lines { spans, units } if units.is_empty() => {
            spans.iter().map(|span| span.end + 1 - span.start).sum()
        }
        BatchContent::Lines { units, .. } => units.len(),
    }
}

/// The first `count` listing entries, source lines or line units of
/// `content`, in group order, (path, line) order or unit order. A unit
/// prefix keeps its span rows and the blank rows bridging them, rebuilt as
/// `Full` spans: unit batches show one file in full.
fn content_prefix(content: &BatchContent, count: usize, sources: &SourceCache) -> BatchContent {
    let mut left = count;
    match content {
        BatchContent::Fs { groups } => BatchContent::Fs {
            groups: groups
                .iter()
                .filter_map(|group| {
                    let FsEntries::Listed(paths) = &group.entries else {
                        return None;
                    };
                    let take = paths.len().min(left);
                    left -= take;
                    (take > 0).then(|| FsGroup {
                        parent: group.parent.clone(),
                        entries: FsEntries::Listed(paths[..take].to_vec()),
                    })
                })
                .collect(),
        },
        BatchContent::Lines { spans, units } if !units.is_empty() => {
            debug_assert!(
                !spans.is_empty()
                    && spans.iter().all(|span| {
                        span.path == spans[0].path && matches!(span.render, Render::Full)
                    }),
                "a unit batch is `Full` spans of one file"
            );
            let path = &spans[0].path;
            let rows = units[..count]
                .iter()
                .flatten()
                .copied()
                .filter(|row| {
                    spans
                        .iter()
                        .any(|span| (span.start..=span.end).contains(row))
                })
                .collect();
            BatchContent::Lines {
                spans: sources.get(path).map_or_else(Vec::new, |source| {
                    crate::walker::build_file_spans(path, &source, rows)
                }),
                units: units[..count].to_vec(),
            }
        }
        BatchContent::Lines { spans, .. } => {
            let mut ordered: Vec<&Span> = spans.iter().collect();
            ordered.sort_by(|a, b| (&a.path, a.start).cmp(&(&b.path, b.start)));
            let mut prefix = Vec::new();
            for span in ordered {
                if left == 0 {
                    break;
                }
                let take = (span.end + 1 - span.start).min(left);
                left -= take;
                prefix.push(Span {
                    end: span.start + take - 1,
                    ..span.clone()
                });
            }
            BatchContent::Lines {
                spans: prefix,
                units: Vec::new(),
            }
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

/// Change in synthesized in-file `…` rows caused by replacing `spans` in
/// one file. Only the gap bounded by the nearest unchanged anchors can
/// change, so this avoids cloning and rescanning the file's full
/// accumulated anchor set for every scheduler probe.
fn local_gap_marker_delta(
    existing: Option<&BTreeMap<usize, LineRecord>>,
    spans: &[&Span],
    source: Option<&Source>,
) -> isize {
    let Some(first_changed) = spans.iter().map(|span| span.start).min() else {
        return 0;
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
    // file shows any source at all before and after; a file showing none
    // has no gap rows.
    let outer_anchors = left.is_some() || right.is_some();
    let shown_before = outer_anchors || !before.is_empty();
    let shown_after = outer_anchors || !after.is_empty();

    let marker_count = |shown: bool, anchors: &BTreeSet<usize>| {
        let mut count = 0isize;
        if shown {
            walk_anchor_gaps(anchors, (left, right), source, |event| {
                count += isize::from(matches!(event, GapWalkEvent::Gap));
            });
        }
        count
    };
    marker_count(shown_after, &after) - marker_count(shown_before, &before)
}

enum GapWalkEvent {
    Gap,
    Anchor(usize),
}

/// The single traversal behind both marker counting and row emission:
/// visits each anchor in order and each elision gap (leading / between
/// anchors / trailing) that holds non-blank source. `left` and `right`
/// are the nearest anchors outside `anchors`, if any, so the cost side
/// can count one changed window. Charged tokens == rendered tokens
/// depends on cost and render never walking gaps differently, so both
/// must go through here.
fn walk_anchor_gaps(
    anchors: &BTreeSet<usize>,
    (left, right): (Option<usize>, Option<usize>),
    source: Option<&Source>,
    mut visit: impl FnMut(GapWalkEvent),
) {
    let mut prev = left.unwrap_or(0);
    for &a in anchors {
        if a > prev + 1 && source.is_some_and(|s| s.gap_has_content(prev + 1, a - 1)) {
            visit(GapWalkEvent::Gap);
        }
        visit(GapWalkEvent::Anchor(a));
        prev = a;
    }
    if let Some(s) = source {
        let gap_end = right.map_or(s.line_count(), |right| right.saturating_sub(1));
        if s.gap_has_content(prev + 1, gap_end) {
            visit(GapWalkEvent::Gap);
        }
    }
}

/// Whether a directory showing `listed` of its `shown` entries needs a
/// trailing `…` row: a listing cut short is otherwise identical to a
/// complete one. A directory showing none of its entries reads as
/// unexpanded without one.
fn listing_partial(listed: usize, shown: usize) -> bool {
    listed > 0 && listed < shown
}

fn format_marker_row(indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push('…');
    s.push('\n');
    s
}

/// One tree entry row: an entry, or a chain of directories that each
/// hold only the next (`src/main/java/`). An entry with nothing rendered
/// under it reads as unexpanded, so the rare entry that holds nothing to
/// show says so.
fn format_entry_row(names: &[String], kind: EntryKind, indent_depth: usize, empty: bool) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    push_escaped(&mut s, &names.join("/"), false);
    if matches!(kind, EntryKind::Directory) {
        s.push('/');
    }
    if empty {
        s.push_str(" (empty)");
    }
    s.push('\n');
    s
}

/// Append `text` with its control characters escaped (`\n`, `\u{1b}`), so
/// no row can break in two or drive a terminal. Tabs pass through when
/// `keep_tabs`: in source they are indentation.
fn push_escaped(out: &mut String, text: &str, keep_tabs: bool) {
    let escaped = |ch: char| ch.is_control() && !(keep_tabs && ch == '\t');
    if !text.contains(escaped) {
        out.push_str(text);
        return;
    }
    for ch in text.chars() {
        if escaped(ch) {
            out.extend(ch.escape_default());
        } else {
            out.push(ch);
        }
    }
}

/// The part of `source_line` a `Full` row shows: at most
/// [`MAX_ROW_CHARS`] characters, the rest replaced by `…`.
pub fn visible_full_line(source_line: &str) -> &str {
    match source_line.char_indices().nth(MAX_ROW_CHARS) {
        Some((cut, _)) => &source_line[..cut],
        None => source_line,
    }
}

/// `line` with its inline secrets shown as `…`: the password of a URL
/// (`postgresql://admin:PASSWORD@db/app`) and the quoted literal assigned
/// to a credential-named key (`password: "hunter2"`,
/// `"api_key" => 'sk-…'`, `SECRET_KEY = "…"`, `authToken: "…"`), whatever
/// punctuation it holds. A placeholder (`<password>`, `%s`, `$DB_PASSWORD`,
/// `$user:$password`, `env(DB_PASSWORD)`, `%env(DB_PASSWORD)%`), a value
/// holding a whole interpolation (`{}`, `{password}`, `{{ … }}`,
/// `${DB_PASSWORD}`), a phrase
/// (`"Save password": "Tallenna salasana"`),
/// a version (`"parse-passwd": "^1.0.0"`), a value spelling its
/// own key (`ACCESS_TOKEN = "access_token"`), and anything unquoted — a
/// variable, a call, a type — is code or documentation, and shows, as
/// does every literal in a document (`export API_KEY='your-key'`).
fn redact_secrets(line: &str, in_document: bool) -> std::borrow::Cow<'_, str> {
    static URL_PASSWORD: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r#"(?<before>://[^\s/:@"'`]*:)(?<password>[^\s/@"'`]+)@"#).unwrap()
    });
    static CREDENTIAL_LITERAL: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(concat!(
            r#"(?i)(?<open>["']?)"#,
            r"(?<key>[\w.-]*(?:password|passwd|pwd|secret|token|(?:api|access|secret|private|auth)[_-]?key))",
            r#"(?<close>["']?)\s*(?:=>|:=|:|=)\s*"#,
            r#"(?<literal>"(?:[^"\\\s]|\\.)*[a-z](?:[^"\\\s]|\\.)*"|'(?:[^'\\\s]|\\.)*[a-z](?:[^'\\\s]|\\.)*'|"\d+"|'\d+')"#,
        ))
        .unwrap()
    });
    static PLACEHOLDER: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(concat!(
            r"^(?:<[^>]*>|\$\w+(?:[^\w\s$]+\$[A-Za-z_]\w*)*|[\w.]+\(.*\)|%[\w.()]+%|%(?:\(\w+\))?[sd])$",
            r"|\$\{[^}]*\}|\{\{.*\}\}|\{%.*%\}|\{\w*\}",
        ))
        .unwrap()
    });
    let line = URL_PASSWORD.replace_all(line, |caps: &regex::Captures| {
        if PLACEHOLDER.is_match(&caps["password"]) {
            caps[0].to_string()
        } else {
            format!("{}…@", &caps["before"])
        }
    });
    if in_document {
        return line;
    }
    let alphanumerics = |text: &str| -> String {
        text.chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect()
    };
    let redacted = CREDENTIAL_LITERAL.replace_all(&line, |caps: &regex::Captures| {
        let whole = caps.get(0).unwrap();
        let literal = caps.name("literal").unwrap();
        if caps["open"] != caps["close"]
            || PLACEHOLDER.is_match(&literal.as_str()[1..literal.len() - 1])
            || alphanumerics(&caps["key"]).ends_with(&alphanumerics(literal.as_str()))
        {
            return whole.as_str().to_string();
        }
        let quote = &literal.as_str()[..1];
        format!("{}{quote}…{quote}", &line[whole.start()..literal.start()])
    });
    if let std::borrow::Cow::Owned(redacted) = redacted {
        return std::borrow::Cow::Owned(redacted);
    }
    line
}

/// Render one line: render spec + raw source text (empty string when
/// unavailable — release tolerates, debug asserts).
fn format_line_row(
    path: &Path,
    number: usize,
    render: &Render,
    source_line: &str,
    indent_depth: usize,
) -> String {
    let extension = path.extension().and_then(|ext| ext.to_str());
    let in_document = matches!(
        extension.map(str::to_ascii_lowercase).as_deref(),
        Some("md" | "mdx" | "rst" | "adoc")
    );
    let source_line = &*redact_secrets(source_line, in_document);
    let mut s = INDENT_UNIT.repeat(indent_depth);
    match render {
        Render::Ellipsis => debug_assert!(false, "Ellipsis records render no row of their own"),
        Render::Full => {
            let visible = visible_full_line(source_line);
            let _ = write!(s, "{number}→");
            push_escaped(&mut s, visible, true);
            if visible.len() < source_line.len() {
                s.push('…');
            }
        }
        Render::Truncated { pattern } => {
            let _ = write!(s, "{number}→");
            with_truncate_regex(pattern, |re| {
                debug_assert!(
                    re.is_some(),
                    "invalid Truncated regex `{pattern}` — should have been rejected at schema load"
                );
                if let Some(m) = re.and_then(|re| re.find(source_line)) {
                    push_escaped(&mut s, m.as_str(), true);
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

    #[test]
    fn render_source_cache_refuses_reads_past_its_byte_cap() {
        let temp = tempfile::tempdir().unwrap();
        let [first, second, head] = ["first.txt", "second.txt", "head.txt"].map(|name| {
            let path = temp.path().join(name);
            std::fs::write(&path, "a\nb\n").unwrap();
            path
        });
        let cache = SourceCache::new();
        assert!(cache.get(&first).is_some());
        let first_bytes = cache.0.borrow().held_bytes;
        assert!(first_bytes > "a\nb\n".len());
        cache.0.borrow_mut().held_bytes = SOURCE_CACHE_BYTE_CAP - first_bytes + 1;
        assert!(cache.get(&second).is_none());
        assert!(cache.get_head(&head, 1).is_none());
        assert!(cache.get(&first).is_some());
    }

    #[test]
    fn render_source_cache_keeps_serving_a_source_its_decoding_grew_past_the_cap() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("latin1.txt");
        let mut bytes = vec![0xFF; MAX_SOURCE_BYTES / 2];
        bytes.push(b'\n');
        std::fs::write(&path, bytes).unwrap();
        let cache = SourceCache::new();
        let first = cache.get(&path).unwrap();
        assert!(first.len() > MAX_SOURCE_BYTES);
        assert!(Arc::ptr_eq(&first, &cache.get(&path).unwrap()));
    }

    #[test]
    fn render_source_cache_drops_a_leading_byte_order_mark() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("Program.cs");
        std::fs::write(&path, "\u{feff}using System;\n").unwrap();
        let source = SourceCache::new().get(&path).unwrap();
        assert_eq!(source.line(1), Some("using System;"));
    }

    fn one_span(path: PathBuf, line: usize, render: Render) -> BatchContent {
        BatchContent::Lines {
            spans: vec![Span {
                path,
                start: line,
                end: line,
                render,
            }],
            units: Vec::new(),
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

    /// Real on-disk tree — the `(empty)` marker asks the filesystem
    /// whether an unopened entry holds nothing, so these cases can't
    /// run against the stub paths the gap tests use.
    ///
    /// `listed/` and `shown.rs` end up rendering content; `pruned/` and
    /// `hidden.rs` hold content that stays hidden; `empty/` and
    /// `zero.rs` hold nothing at all; `linked/` points at `listed/`.
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
        std::os::unix::fs::symlink("listed", root.join("linked")).unwrap();
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
        "linked",
        "listed",
        "pruned",
        "shown.rs",
        "zero.rs",
    ];

    #[test]
    fn render_empty_marker_separates_empty_entries_from_pruned_ones() {
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
        assert!(out.contains("pruned/\n"), "output:\n{out}");
        assert!(out.contains("hidden.rs\n"), "output:\n{out}");
        assert!(out.contains("empty/ (empty)\n"), "output:\n{out}");
        assert!(out.contains("zero.rs (empty)\n"), "output:\n{out}");
        assert!(out.contains("listed/\n"), "output:\n{out}");
        assert!(out.contains("linked/\n"), "output:\n{out}");
        assert!(out.contains("shown.rs\n"), "output:\n{out}");
    }

    #[test]
    fn render_charged_cost_matches_rendered_rows_across_expansions() {
        let temp = disk_fixture();
        let root = temp.path().to_path_buf();
        let mut tree = RenderedTree::new(root.clone(), SourceCache::new());
        // Each step expands an entry an earlier step listed, so a charge
        // that ignores the tree state shows up as a mismatch here.
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
        let relisted = dir_listing(root.clone(), &["shown.rs"]);
        assert_eq!(tree.marginal_cost(&relisted), Cost::default());
        let rendered: usize = tree
            .render()
            .lines()
            .map(|line| tokenizer::count(&format!("{line}\n")))
            .sum();
        assert_eq!(charged, rendered, "output:\n{}", tree.render());
    }

    #[test]
    fn render_single_entry_directories_chain_onto_one_row() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().to_path_buf();
        let core = root.join("deep/pkg/core");
        std::fs::create_dir_all(&core).unwrap();
        std::fs::create_dir_all(root.join("deep/pkg/core/leaf")).unwrap();
        std::fs::write(core.join("x.rs"), "fn x() {}\n").unwrap();
        std::fs::write(root.join("top.rs"), "fn top() {}\n").unwrap();
        let one_link_at_a_time = [
            dir_listing(root.clone(), &["deep", "top.rs"]),
            dir_listing(root.join("deep"), &["pkg"]),
            dir_listing(root.join("deep/pkg"), &["core"]),
            dir_listing(core.clone(), &["leaf", "x.rs"]),
            one_span(core.join("x.rs"), 1, Render::Full),
        ];
        let whole_chain_at_once = [
            dir_listing(root.clone(), &["deep", "top.rs"]),
            BatchContent::Fs {
                groups: [
                    (root.join("deep"), "pkg"),
                    (root.join("deep/pkg"), "core"),
                    (core.clone(), "x.rs"),
                ]
                .into_iter()
                .map(|(parent, name)| FsGroup {
                    parent,
                    entries: FsEntries::Listed(vec![PathBuf::from(name)]),
                })
                .collect(),
            },
            dir_listing(core.clone(), &["leaf"]),
            one_span(core.join("x.rs"), 1, Render::Full),
        ];
        for steps in [&one_link_at_a_time[..], &whole_chain_at_once[..]] {
            let mut tree = RenderedTree::new(root.clone(), SourceCache::new());
            let mut charged = 0;
            for (i, content) in steps.iter().enumerate() {
                charged += tree.marginal_cost(content).tokens;
                tree.apply(content, BatchId::new(i), |_| true);
            }
            let out = tree.render();
            assert_eq!(
                out,
                "deep/pkg/core/\n  leaf/ (empty)\n  x.rs\n    1→fn x() {}\ntop.rs\n"
            );
            assert_eq!(charged, tokenizer::count(&out), "output:\n{out}");
        }
    }

    #[test]
    fn render_inline_secrets_are_redacted_and_nothing_else() {
        for (line, shown) in [
            (
                r#"url = "postgresql://admin:hunter2@db:5432/app""#,
                r#"url = "postgresql://admin:…@db:5432/app""#,
            ),
            ("redis://:hunter2@cache/0", "redis://:…@cache/0"),
            (
                "http://localhost:8080/login@v2",
                "http://localhost:8080/login@v2",
            ),
            ("git@github.com:org/repo.git", "git@github.com:org/repo.git"),
            (
                r#"url = env("DATABASE_URL")"#,
                r#"url = env("DATABASE_URL")"#,
            ),
            ("mongodb://{}:{}@{}:{}/{}", "mongodb://{}:{}@{}:{}/{}"),
            (
                "postgres://app:${DB_PASSWORD}@db/app",
                "postgres://app:${DB_PASSWORD}@db/app",
            ),
            (r#"    password: "@ns1bl3""#, r#"    password: "…""#),
            (
                r#"{"api_key": "sk-123", "user": "a"}"#,
                r#"{"api_key": "…", "user": "a"}"#,
            ),
            ("'client_secret' => 'abc',", "'client_secret' => '…',"),
            (r#"#GOTIFY_TOKEN="123456789ABCDEF""#, r#"#GOTIFY_TOKEN="…""#),
            (
                r#"connect(authToken := "t0k")"#,
                r#"connect(authToken := "…")"#,
            ),
            ("password = get_password()", "password = get_password()"),
            ("token: Token,", "token: Token,"),
            ("password: '{{ vault_pw }}'", "password: '{{ vault_pw }}'"),
            (r#"secret = "${SECRET}""#, r#"secret = "${SECRET}""#),
            (
                r#"secret = "env(APPLE_SECRET)""#,
                r#"secret = "env(APPLE_SECRET)""#,
            ),
            ("password: '<password>'", "password: '<password>'"),
            (r#"token: """#, r#"token: """#),
            (r#"l.token = "=""#, r#"l.token = "=""#),
            (r#"if token == "if""#, r#"if token == "if""#),
            (
                r#"ACCESS_TOKEN = "access_token""#,
                r#"ACCESS_TOKEN = "access_token""#,
            ),
            (r#"tokenizer = "gpt2""#, r#"tokenizer = "gpt2""#),
            (r#""parse-passwd": "^1.0.0""#, r#""parse-passwd": "^1.0.0""#),
            (r#"githubToken: "ghp_abc""#, r#"githubToken: "…""#),
            (r#"password = "12345678""#, r#"password = "…""#),
            (r#""Password": "密码""#, r#""Password": "密码""#),
            (r#"password = "ab\"cd123""#, r#"password = "…""#),
            (r#"password = "\"hunter2""#, r#"password = "…""#),
            (r#"password = 'a\\' + b"#, r#"password = '…' + b"#),
            (
                r#"user_token="t1" api_key='k2'"#,
                r#"user_token="…" api_key='…'"#,
            ),
            (
                r#""Save password": "Tallenna salasana""#,
                r#""Save password": "Tallenna salasana""#,
            ),
            (
                r#"{"password": "Fake7$Value%42!", "api_key": "k(e)y<2>!"}"#,
                r#"{"password": "…", "api_key": "…"}"#,
            ),
            ("postgres://app:pa$$w%rd@db/app", "postgres://app:…@db/app"),
            (
                "url = \"postgres://app:p%d0%b0ss7@db/app\"",
                "url = \"postgres://app:…@db/app\"",
            ),
            ("postgres://%s:%s@%s/app", "postgres://%s:%s@%s/app"),
            (r#"password = "hunter%s2""#, r#"password = "…""#),
            (r#"password = "%s""#, r#"password = "%s""#),
            (
                "f\"postgres://{user}:{password}@{host}\"",
                "f\"postgres://{user}:{password}@{host}\"",
            ),
            (
                r#"password = "$DB_PASSWORD""#,
                r#"password = "$DB_PASSWORD""#,
            ),
            (
                r#"val usernameAndPassword = "$username:$password""#,
                r#"val usernameAndPassword = "$username:$password""#,
            ),
            (
                r#"password = "$2b$12$R9h/cIPz0gi.URNNX3kh2O""#,
                r#"password = "…""#,
            ),
            (r#"password = "$user:hunter2""#, r#"password = "…""#),
            (
                r#"password = "%(password)s""#,
                r#"password = "%(password)s""#,
            ),
            (r#"password = "{password}""#, r#"password = "{password}""#),
            (
                r#"password = "{{password}}""#,
                r#"password = "{{password}}""#,
            ),
            (
                r#"secret: '%env(APP_SECRET)%'"#,
                r#"secret: '%env(APP_SECRET)%'"#,
            ),
            (r#"password = "hunter2{{""#, r#"password = "…""#),
            (r#"password = "pa${ss""#, r#"password = "…""#),
            ("postgres://u:p{%w@h/db", "postgres://u:…@h/db"),
        ] {
            assert_eq!(redact_secrets(line, false), shown);
        }
        assert_eq!(
            redact_secrets("export API_KEY='your-key' # postgres://u:pw@db", true),
            "export API_KEY='your-key' # postgres://u:…@db"
        );
    }

    #[test]
    fn render_full_row_longer_than_the_cap_ends_in_ellipsis() {
        let long = "é".repeat(MAX_ROW_CHARS + 1);
        let row = format_line_row(Path::new("a.rs"), 7, &Render::Full, &long, 0);
        assert_eq!(row, format!("7→{}…\n", "é".repeat(MAX_ROW_CHARS)));
        let fits = &long[..long.len() - 'é'.len_utf8()];
        assert_eq!(
            format_line_row(Path::new("a.rs"), 7, &Render::Full, fits, 0),
            format!("7→{fits}\n")
        );
    }

    #[test]
    fn render_gap_markers_mark_only_elided_nonblank_source() {
        let (cache, path) = gap_fixture();
        let mut tree = RenderedTree::new(stub_dir(), cache);
        tree.apply(&listing(&["f.c"]), BatchId::new(0), |_| true);
        // Only line 3 rendered: elided source on both sides is
        // non-blank (line 1; lines 4-6) → leading + trailing markers.
        tree.apply(
            &one_span(path.clone(), 3, Render::Full),
            BatchId::new(3),
            |_| true,
        );
        let out = tree.render();
        assert_eq!(out.matches('…').count(), 2, "output:\n{out}");
        let row = out.find("3→fn b();").unwrap();
        assert!(
            out.find('…').unwrap() < row && row < out.rfind('…').unwrap(),
            "output:\n{out}"
        );
        // Blank-only gap (line 2): no marker. Non-blank gap (4-5):
        // marker synthesized despite no Ellipsis record.
        for line in [1, 6] {
            let content = one_span(path.clone(), line, Render::Full);
            assert!(tree.marginal_cost(&content).tokens > 0);
            tree.apply(&content, BatchId::new(line), |_| true);
        }
        let out = tree.render();
        assert_eq!(out.matches('…').count(), 1, "output:\n{out}");
        let marker = out.find('…').unwrap();
        assert!(
            out.find("3→fn b();").unwrap() < marker && marker < out.find("6→fn c();").unwrap(),
            "output:\n{out}"
        );
        // Author Ellipsis records inside blank (2) and non-blank (4)
        // gaps: zero marginal cost, output byte-identical.
        for line in [2, 4] {
            let content = one_span(path.clone(), line, Render::Ellipsis);
            assert_eq!(tree.marginal_cost(&content), Cost::default(), "line {line}");
            tree.apply(&content, BatchId::new(10 + line), |_| true);
        }
        assert_eq!(tree.render(), out);
    }

    #[test]
    fn render_escapes_control_characters_in_names_and_rows_but_tabs() {
        let cache = SourceCache::new();
        let path = PathBuf::from(format!("{STUB_DIR}/esc.txt"));
        cache.insert(
            path.clone(),
            Arc::from("\tline \x1b[31mred\x1b[0m\rover\n\x1b[1mbold\n"),
        );
        let mut tree = RenderedTree::new(stub_dir(), cache);
        let steps = [
            listing(&["esc.txt", "name\nwith\ttabs.md"]),
            one_span(path.clone(), 1, Render::Full),
            one_span(
                path,
                2,
                Render::Truncated {
                    pattern: r"^\x1b\[1m".into(),
                },
            ),
        ];
        let mut charged = Cost::default();
        for (i, content) in steps.iter().enumerate() {
            charged = charged + tree.marginal_cost(content);
            tree.apply(content, BatchId::new(i), |_| true);
        }
        let rendered = tree.render();
        assert_eq!(
            rendered,
            "esc.txt\n  1→\tline \\u{1b}[31mred\\u{1b}[0m\\rover\n  2→\\u{1b}[1m…\nname\\nwith\\ttabs.md\n"
        );
        assert_eq!(
            charged,
            Cost {
                tokens: tokenizer::count(&rendered),
                chars: char_units(&rendered),
            }
        );
    }

    #[test]
    fn render_span_refinement_costs_only_the_delta() {
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

        let cache_fresh = SourceCache::new();
        cache_fresh.insert(
            path.clone(),
            Arc::from("pub fn long_function_name_here() -> Result<()> {}\n"),
        );
        let mut tree_fresh = RenderedTree::new(stub_dir(), cache_fresh);
        tree_fresh.apply(&listing(&["syn.rs"]), BatchId::new(0), |_| true);
        let fresh_exact = tree_fresh.marginal_cost(&full);

        assert!(delta_exact.chars > 0);
        assert!(delta_exact.tokens > 0);
        assert!(delta_exact.chars < fresh_exact.chars);
    }

    #[test]
    fn render_unit_prefix_takes_whole_units_in_unit_order() {
        let (cache, path) = gap_fixture();
        let content = BatchContent::Lines {
            spans: vec![Span {
                path: path.clone(),
                start: 1,
                end: 6,
                render: Render::Full,
            }],
            units: vec![vec![1, 6], vec![3, 4], vec![5]],
        };
        let prefix_rows = |count| {
            let BatchContent::Lines { spans, units } = content_prefix(&content, count, &cache)
            else {
                unreachable!();
            };
            assert_eq!(units.len(), count);
            spans
                .iter()
                .map(|span| (span.start, span.end))
                .collect::<Vec<_>>()
        };
        assert_eq!(content_len(&content), 3);
        assert_eq!(prefix_rows(1), [(1, 1), (6, 6)]);
        assert_eq!(prefix_rows(2), [(1, 4), (6, 6)]);
        assert_eq!(prefix_rows(3), [(1, 6)]);
    }
}
