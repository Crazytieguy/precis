//! Walker trait + shared context.
//!
//! The walker trait splits three concerns that `precis` v0.2-first-pass
//! conflated:
//!
//! - **Discovery** (`seed` / `expand`): cheap; returns [`Candidate`]s named
//!   by [`BatchKey`]. No file I/O beyond `read_dir`. No parsing. The
//!   scheduler decides which candidates advance.
//! - **Materialization** (`materialize`): expensive; reads + parses the
//!   relevant source, returns a [`ResolvedBatch`] with final content and
//!   value signals. Only called once a candidate has enough FS-only
//!   evidence to be worth paying for (two-tier frontier in the scheduler).
//! - **Scoring**: not the walker's job at all. The scheduler composes
//!   [`ValueSignals`] via [`value`](crate::value) into a scalar.
//!
//! Why the split. The previous trait combined "emit a batch" with "schedule
//! me as a successor of the parent folder", which meant `successors()` on a
//! folder listing read + parsed every child file eagerly. Cross-file
//! batches couldn't exist (each walker saw one file at a time); predecessor
//! edges within a single call couldn't exist (the scheduler stamped the
//! predecessor from the currently-scheduled batch's id). The new split
//! makes all three unrepresentable by accident.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Language, Node, Tree};

use crate::batch::{ResolvedBatch, ValueSignals, WalkerKey};
use crate::content::{BatchContent, Render, Span};
use crate::render::SourceCache;

pub mod fs;
pub mod json;
pub mod markdown;
pub mod multi;
pub mod rust;
pub mod toml;
pub mod typescript;

/// A discovered batch that hasn't been materialized yet. Emitted by
/// `seed` / `expand`. Carries:
/// - the stable key so other candidates can name it as predecessor,
/// - [`ValueSignals`] the walker can fill in from FS-only evidence (used
///   as the speculative upper bound by the scheduler),
/// - optional cost hint (token upper bound) for the speculative frontier.
///
/// Generic over the walker's own key type (see [`Walker::Key`]), so the
/// scheduler/renderer never names any walker-specific enum.
#[derive(Debug, Clone)]
pub struct Candidate<K: WalkerKey> {
    pub key: K,
    pub predecessor: Option<K>,
    /// FS-only value signals. After materialization these are overwritten
    /// with the resolved batch's (usually richer) signals.
    pub signals: ValueSignals,
    /// Upper-bound estimate of this batch's token cost, from FS properties.
    /// Cheap bound only — used to compute the upper-bound ratio. The actual
    /// cost is computed post-materialization.
    pub cost_hint: usize,
}

impl<K: WalkerKey> Candidate<K> {
    pub fn new(key: K, signals: ValueSignals, cost_hint: usize) -> Self {
        Self {
            key,
            predecessor: None,
            signals,
            cost_hint,
        }
    }

    pub fn with_predecessor(mut self, pred: K) -> Self {
        self.predecessor = Some(pred);
        self
    }
}

/// Walker contract. A single implementor composes the filesystem walker
/// with per-language walkers (see [`multi::MultiWalker`]). The associated
/// `Key` type is walker-private: scheduler + renderer never name it, and
/// adding a new walker doesn't change scheduler/renderer code.
pub trait Walker {
    type Key: WalkerKey;

    /// Initial candidates. Typically the root filesystem listing.
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Candidate<Self::Key>>;

    /// Called when a candidate is scheduled. Returns newly-discovered
    /// candidates. `scheduled` is the key just moved into the tree; the
    /// walker uses it to decide what to propose next (e.g. listing `src/`
    /// exposes `RustKey::PubDecls { src_dir: "src" }`).
    fn expand(&mut self, scheduled: &Self::Key, ctx: &WalkCtx) -> Vec<Candidate<Self::Key>>;

    /// Read source, parse, and build the concrete batch for `key`. Returns
    /// `None` when materialization finds nothing (e.g. no `pub` items in
    /// the crate) — the scheduler then marks the key dead and never
    /// retries. This is the **only** method allowed to call
    /// `fs::read_to_string` (enforced by convention; see [`WalkCtx`] which
    /// centralizes source + parse caches).
    fn materialize(&mut self, key: &Self::Key, ctx: &WalkCtx) -> Option<ResolvedBatch>;
}

/// Per-run context. Holds the seed root plus a shared source cache +
/// per-file parse cache. The source cache is the same handle the
/// [`RenderedTree`](crate::render::RenderedTree) uses for render-time
/// materialization, so each file is read at most once across the whole run.
pub struct WalkCtx {
    root: PathBuf,
    source_cache: SourceCache,
    /// Tree-sitter parse results, keyed by path.
    tree_cache: RefCell<HashMap<PathBuf, Arc<Tree>>>,
}

impl WalkCtx {
    pub fn new(root: PathBuf) -> Self {
        Self::with_cache(root, SourceCache::new())
    }

    pub fn with_cache(root: PathBuf, source_cache: SourceCache) -> Self {
        Self {
            root,
            source_cache,
            tree_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn source_cache(&self) -> &SourceCache {
        &self.source_cache
    }

    /// Depth of `path` relative to the seed root (root itself = 0). Returns
    /// 0 for paths not under root — a walker bug, but don't panic mid-run.
    pub fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    /// Path-aware non-essential discount, scoped to this run's root so
    /// the outer test/tooling dirs of whoever invoked precis don't poison
    /// every fixture path.
    pub fn non_essential_factor(&self, path: &Path) -> f64 {
        crate::value::non_essential_factor(path, &self.root)
    }

    /// Read `path` into memory, caching the result. Returns an `Arc<str>`
    /// so callers don't duplicate the string.
    pub fn read_source(&self, path: &Path) -> Option<Arc<str>> {
        self.source_cache.get(path)
    }

    /// Parse `path` with the given tree-sitter grammar, caching the result.
    /// The grammar is only instantiated on the first parse of a file; the
    /// parsed tree is shared across subsequent materialize calls.
    pub fn parse_tree(&self, path: &Path, language: &Language) -> Option<(Arc<str>, Arc<Tree>)> {
        let source = self.read_source(path)?;
        if let Some(tree) = self.tree_cache.borrow().get(path) {
            return Some((source, tree.clone()));
        }
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .expect("tree-sitter language load");
        let tree = parser.parse(source.as_bytes(), None)?;
        let arc = Arc::new(tree);
        self.tree_cache
            .borrow_mut()
            .insert(path.to_path_buf(), arc.clone());
        Some((source, arc))
    }
}

/// Lines a collector wants to render for one file: `full` = emit the source
/// line verbatim; `ellipses` = emit a walker `…` marker at that line number
/// (no text, no rendered line number — but a real line number so descendant
/// batches can override it with real content).
#[derive(Default, Debug)]
pub(crate) struct FileLines {
    pub full: Vec<usize>,
    pub ellipses: Vec<usize>,
}

impl FileLines {
    pub fn new(full: Vec<usize>) -> Self {
        Self {
            full,
            ellipses: Vec::new(),
        }
    }
    pub fn with_ellipses(mut self, ellipses: Vec<usize>) -> Self {
        self.ellipses = ellipses;
        self
    }
}

/// Build a [`ResolvedBatch`] whose content is a set of spans for one file.
/// Blank source lines are filtered from the `full` set; an ellipsis at a
/// line already in `full` is dropped. Returns `None` when the resulting
/// span set is empty — caller propagates as "dead key".
pub(crate) fn single_file_lines_batch(
    path: &Path,
    source: &str,
    lines: FileLines,
    signals: ValueSignals,
) -> Option<ResolvedBatch> {
    let spans = build_file_spans(path, source, lines);
    if spans.is_empty() {
        return None;
    }
    Some(ResolvedBatch {
        content: BatchContent::Lines { spans },
        signals,
    })
}

/// Convert a `FileLines` spec for one file into contiguous [`Span`] ranges.
/// Blank source lines (all-whitespace) are excluded from the `Full` set;
/// an ellipsis line that also appears in `full` is dropped (a real line
/// always beats an ellipsis marker at the same position).
pub(crate) fn build_file_spans(path: &Path, source: &str, lines: FileLines) -> Vec<Span> {
    let src_lines: Vec<&str> = source.lines().collect();
    let full: BTreeSet<usize> = lines
        .full
        .into_iter()
        .filter(|n| src_lines.get(*n - 1).is_some_and(|t| !t.trim().is_empty()))
        .collect();
    let line_count = src_lines.len();
    let ellipses: BTreeSet<usize> = lines
        .ellipses
        .into_iter()
        .filter(|n| !full.contains(n) && *n >= 1 && *n <= line_count)
        .collect();

    let mut spans = Vec::new();
    // Merge contiguous runs of Full line numbers into single-range spans.
    let full_vec: Vec<usize> = full.iter().copied().collect();
    let mut i = 0;
    while i < full_vec.len() {
        let start = full_vec[i];
        let mut end = start;
        while i + 1 < full_vec.len() && full_vec[i + 1] == end + 1 {
            end = full_vec[i + 1];
            i += 1;
        }
        spans.push(Span {
            path: path.to_path_buf(),
            start,
            end,
            render: Render::Full,
        });
        i += 1;
    }
    for n in ellipses {
        spans.push(Span {
            path: path.to_path_buf(),
            start: n,
            end: n,
            render: Render::Ellipsis,
        });
    }
    spans
}

// --- shared tree-sitter span helpers ---
//
// Pure AST utilities reused by every per-language walker that emits line
// spans. Kept here so language walkers don't redeclare identical helpers.

/// Append every 1-based row covered by `node` to `out`, skipping any
/// trailing newline at the end of the node's text.
pub(crate) fn extend_span(out: &mut Vec<usize>, node: Node, source: &str) {
    let start = node.start_position().row;
    let text = &source[node.start_byte()..node.end_byte()];
    let internal_lines = text.trim_end_matches(['\n', '\r']).split('\n').count();
    let span = internal_lines.max(1) - 1;
    push_rows(out, start, start + span);
}

/// 0-based row of a declaration's signature end: the row before its body
/// starts, or its end row if there's no body field.
pub(crate) fn signature_end_row(node: Node) -> usize {
    node.child_by_field_name("body")
        .map(|b| b.start_position().row)
        .unwrap_or_else(|| node.end_position().row)
}

/// Push 1-based line numbers `start_row+1 ..= end_row+1` onto `out`.
/// Inputs are 0-based tree-sitter row indices.
pub(crate) fn push_rows(out: &mut Vec<usize>, start_row: usize, end_row: usize) {
    for row in start_row..=end_row {
        out.push(row + 1);
    }
}

pub(crate) fn dedup_sorted(mut v: Vec<usize>) -> Vec<usize> {
    v.sort();
    v.dedup();
    v
}
