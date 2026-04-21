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
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Language, Tree};

use crate::batch::{BatchContent, BatchKey, RenderedLine, ResolvedBatch, ValueSignals};

pub mod fs;
pub mod markdown;
pub mod multi;
pub mod rust;
pub mod toml;

/// A discovered batch that hasn't been materialized yet. Emitted by
/// `seed` / `expand`. Carries:
/// - the stable [`BatchKey`] so other candidates can name it as predecessor,
/// - [`ValueSignals`] the walker can fill in from FS-only evidence (used
///   as the speculative upper bound by the scheduler),
/// - optional cost hint (token upper bound) for the speculative frontier.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub key: BatchKey,
    pub predecessor: Option<BatchKey>,
    /// FS-only value signals. After materialization these are overwritten
    /// with the resolved batch's (usually richer) signals.
    pub signals: ValueSignals,
    /// Upper-bound estimate of this batch's token cost, from FS properties.
    /// Cheap bound only — used to compute the upper-bound ratio. The actual
    /// cost is computed post-materialization.
    pub cost_hint: usize,
}

impl Candidate {
    pub fn new(key: BatchKey, signals: ValueSignals, cost_hint: usize) -> Self {
        Self {
            key,
            predecessor: None,
            signals,
            cost_hint,
        }
    }

    pub fn with_predecessor(mut self, pred: BatchKey) -> Self {
        self.predecessor = Some(pred);
        self
    }
}

/// Walker contract. A single implementor composes the filesystem walker
/// with per-language walkers (see [`multi::MultiWalker`]).
pub trait Walker {
    /// Initial candidates. Typically the root filesystem listing.
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Candidate>;

    /// Called when a candidate is scheduled. Returns newly-discovered
    /// candidates. `scheduled` is the key just moved into the tree; the
    /// walker uses it to decide what to propose next (e.g. listing `src/`
    /// exposes `RustKey::PubDecls { src_dir: "src" }`).
    fn expand(&mut self, scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate>;

    /// Read source, parse, and build the concrete batch for `key`. Returns
    /// `None` when materialization finds nothing (e.g. no `pub` items in
    /// the crate) — the scheduler then marks the key dead and never
    /// retries. This is the **only** method allowed to call
    /// `fs::read_to_string` (enforced by convention; see [`WalkCtx`] which
    /// centralizes source + parse caches).
    fn materialize(&mut self, key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch>;
}

/// Per-run context. Holds the seed root plus source + parse caches so that
/// Rust's headline decl batch and its rustdoc refinement reuse the same
/// parsed tree. Caches are unbounded for v0.2 — bound is "files actually
/// materialized", which is exactly the laziness budget we care about.
pub struct WalkCtx {
    root: PathBuf,
    /// Full source of each file that's been read. `Arc<str>` so parsers
    /// and multiple walkers can share cheaply.
    source_cache: RefCell<HashMap<PathBuf, Arc<str>>>,
    /// Tree-sitter parse results, keyed by path.
    tree_cache: RefCell<HashMap<PathBuf, Arc<Tree>>>,
}

impl WalkCtx {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            source_cache: RefCell::new(HashMap::new()),
            tree_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Depth of `path` relative to the seed root (root itself = 0). Returns
    /// 0 for paths not under root — a walker bug, but don't panic mid-run.
    pub fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    /// Read `path` into memory, caching the result. Returns an `Arc<str>`
    /// so callers don't duplicate the string.
    pub fn read_source(&self, path: &Path) -> Option<Arc<str>> {
        if let Some(cached) = self.source_cache.borrow().get(path) {
            return Some(cached.clone());
        }
        let text = std::fs::read_to_string(path).ok()?;
        let arc: Arc<str> = Arc::from(text);
        self.source_cache
            .borrow_mut()
            .insert(path.to_path_buf(), arc.clone());
        Some(arc)
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

/// Build a [`ResolvedBatch`] whose content is a single-file Lines map. Empty
/// lines are filtered (renderer rejects them). Returns `None` when the
/// resulting set is empty — caller usually should propagate as "dead key".
pub(crate) fn single_file_lines_batch(
    path: &Path,
    source: &str,
    line_numbers: Vec<usize>,
    signals: ValueSignals,
) -> Option<ResolvedBatch> {
    let lines = lines_map_from(source, line_numbers);
    if lines.is_empty() {
        return None;
    }
    let mut file_map = BTreeMap::new();
    file_map.insert(path.to_path_buf(), lines);
    Some(ResolvedBatch {
        content: BatchContent::Lines(file_map),
        signals,
    })
}

/// Produce `{line_number → RenderedLine::Full(text)}` for a set of 1-indexed
/// source line numbers, skipping blank lines.
pub(crate) fn lines_map_from(
    source: &str,
    line_numbers: Vec<usize>,
) -> BTreeMap<usize, RenderedLine> {
    let src_lines: Vec<&str> = source.lines().collect();
    line_numbers
        .into_iter()
        .filter_map(|n| {
            let text = src_lines.get(n - 1)?;
            if text.trim().is_empty() {
                return None;
            }
            Some((n, RenderedLine::Full(text.to_string())))
        })
        .collect()
}
