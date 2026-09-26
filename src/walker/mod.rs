//! Walker trait + shared context.
//!
//! A walker emits [`Batch`] units containing the key, optional
//! predecessor edge, fully-built `BatchContent`, and a scalar `value`. The
//! scheduler ranks emitted batches by `value / cost^k`, gates by
//! predecessor scheduling, and applies content to the rendered tree.
//! Source text is cached on [`WalkCtx`]; parse trees are not.

use std::cell::OnceCell;
use std::collections::{BTreeSet, HashMap};
use std::ops::Range;
use std::panic::AssertUnwindSafe;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};

use tree_sitter::{Language, Node, Tree};

use crate::batch::{Batch, BatchKey, FsKey};
use crate::content::{BatchContent, Render, Span};
use crate::fs_util::DirFilter;
use crate::render::{Source, SourceCache};

pub(crate) mod code;
pub mod fs;
pub mod go_mod;
pub mod json;
pub mod markdown;
pub mod plaintext;
pub mod prisma;
mod survey;
pub mod toml;
mod workspace;

use survey::EssentialSource;
pub(in crate::walker) use survey::language_group;

/// Greedily partition source-ordered items once a range reaches `target`.
/// A final range cheaper than `min_tail` folds into its predecessor.
/// `split_after` uses exclusive item indices, allowing callers to
/// preserve structural cut boundaries.
pub(super) fn budget_chunk_ranges(
    item_count: usize,
    cost: impl Fn(Range<usize>) -> usize,
    target: usize,
    min_tail: usize,
    split_after: impl Fn(usize) -> bool,
) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for end in 1..item_count {
        if split_after(end) && cost(start..end) >= target {
            ranges.push(start..end);
            start = end;
        }
    }
    if start < item_count {
        let tail = start..item_count;
        if cost(tail.clone()) < min_tail
            && let Some(previous) = ranges.last_mut()
        {
            previous.end = tail.end;
        } else {
            ranges.push(tail);
        }
    }
    ranges
}

/// Walker contract. [`FsWalker`] is the production walker; tests drive
/// the scheduler with stub walkers.
pub trait Walker {
    /// Initial batches before any scheduling decision (typically the
    /// root FS listing).
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch>;

    /// Called when a batch is scheduled — returns newly-discovered
    /// batches.
    fn expand(&mut self, scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Batch>;

    /// Called once, when every batch in `emitted` that can be scheduled
    /// is: batches that spend only the budget left over.
    fn floor(&mut self, _emitted: &[Batch], _ctx: &WalkCtx) -> Vec<Batch> {
        Vec::new()
    }
}

/// Top-level walker: FS listings drive discovery; per-language modules
/// own per-dir candidate emission and are dispatched here directly.
#[derive(Default)]
pub struct FsWalker;

impl Walker for FsWalker {
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch> {
        fs::seed(ctx)
    }

    fn expand(&mut self, scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Batch> {
        let BatchKey::Fs(key) = scheduled else {
            return Vec::new();
        };
        let FsKey::DirListing { dir } = key;
        let mut out = fs::expand_listed(key, ctx);
        out.extend(markdown::expand_in_dir(dir, ctx));
        out.extend(toml::expand_in_dir(dir, ctx));
        out.extend(json::expand_in_dir(dir, ctx));
        out.extend(plaintext::expand_in_dir(dir, ctx));
        out.extend(prisma::expand_in_dir(dir, ctx));
        out.extend(go_mod::expand_in_dir(dir, ctx));
        out.extend(code::expand_in_dir(dir, ctx));
        out
    }

    fn floor(&mut self, emitted: &[Batch], ctx: &WalkCtx) -> Vec<Batch> {
        plaintext::floor_batches(emitted, ctx)
    }
}

/// Per-run context: seed root, shared source/parse caches, and per-
/// walker run state in language-named fields below.
pub struct WalkCtx {
    root: PathBuf,
    /// Built once per run — every listing and file enumeration in the
    /// walk goes through it, so an ignored subtree is invisible to
    /// discovery rather than filtered out downstream.
    dir_filter: Rc<DirFilter>,
    source_cache: SourceCache,
    cargo_workspace: workspace::WorkspaceMembership,
    fs_state: fs::FsState,
    json_state: json::JsonState,
    /// Run state of the code engine's language modules.
    code: code::CodeState,
    /// The tree's essential source, walked once; `None` past
    /// [`crate::fs_util::PROBE_ENTRY_CAP`].
    essential_source: OnceCell<Option<EssentialSource>>,
    /// The language family carrying the most essential source bytes.
    primary_language: OnceCell<Option<&'static str>>,
    /// The one source file that carries a dominant share of the tree's
    /// essential source bytes, if any.
    dominant_source_file: OnceCell<Option<PathBuf>>,
}

impl WalkCtx {
    pub fn new(root: PathBuf) -> Self {
        Self::with_cache(root, SourceCache::new())
    }

    pub fn with_cache(root: PathBuf, source_cache: SourceCache) -> Self {
        Self::with_filter(DirFilter::new(&root), source_cache)
    }

    /// Context for a walk scoped by `dir_filter`, rooted at its root.
    pub fn with_filter(dir_filter: DirFilter, source_cache: SourceCache) -> Self {
        Self {
            root: dir_filter.root().to_path_buf(),
            dir_filter: Rc::new(dir_filter),
            source_cache,
            cargo_workspace: workspace::WorkspaceMembership::default(),
            fs_state: fs::FsState::default(),
            json_state: json::JsonState::default(),
            code: code::CodeState::default(),
            essential_source: OnceCell::new(),
            primary_language: OnceCell::new(),
            dominant_source_file: OnceCell::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Shared with the renderer, so cost probes reuse this run's ignore
    /// caches instead of rebuilding them per probe.
    pub fn dir_filter(&self) -> &Rc<DirFilter> {
        &self.dir_filter
    }

    pub fn source_cache(&self) -> &SourceCache {
        &self.source_cache
    }

    /// Depth of `path` relative to the seed root (root itself = 0).
    /// **Fails open**: a path outside the root also reads as depth 0 —
    /// the same as the root itself — which un-damps every depth-priced
    /// value. Callers handling possibly-out-of-root paths
    /// (canonicalized paths, workspace members) must check containment
    /// first.
    pub fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    /// Non-essential discount, scoped to this run's root.
    pub fn non_essential_factor(&self, path: &Path) -> f64 {
        crate::value::non_essential_factor(path, &self.root)
    }

    /// The single source file the repository is *about*, when one
    /// exists: a mass-dominant essential source file that independently
    /// carries at least [`DOMINANT_SOURCE_MASS_SHARE`] of the tree's source
    /// bytes, the largest if several do. Single-implementation-file libraries (a
    /// one-file parser, a single amalgamated `.c`, a `core.py`) put the content
    /// an NS author wants in one place, and the scheduler's
    /// breadth-first instincts spend the marginal token elsewhere.
    /// Returns `None` when source mass is spread across peers, which is
    /// the common case.
    pub fn dominant_source_file(&self) -> Option<&Path> {
        self.dominant_source_file
            .get_or_init(|| {
                survey::find_dominant_source_file(
                    self.essential_source()?,
                    self.primary_language()?,
                )
            })
            .as_deref()
    }

    /// The language family ([`language_group`]) the repository is written
    /// in: the one carrying the most essential source bytes. `None` for a
    /// tie for the lead — "the language this repo is written in" has no
    /// answer there, and answering it by hasher seeding would make the
    /// output differ between processes on identical input.
    pub(crate) fn primary_language(&self) -> Option<&'static str> {
        *self
            .primary_language
            .get_or_init(|| survey::find_primary_language(self.essential_source()?))
    }

    fn essential_source(&self) -> Option<&EssentialSource> {
        self.essential_source
            .get_or_init(|| survey::enumerate_essential_source(&self.root, &self.dir_filter))
            .as_ref()
    }

    /// Read `path` into memory, caching the result.
    pub fn read_source(&self, path: &Path) -> Option<Arc<Source>> {
        self.source_cache.get(path)
    }

    /// Parse `path` with `language`. The tree is not cached: a file is
    /// parsed while its directory expands, and its batches carry
    /// everything they need from it.
    pub fn parse_tree(&self, path: &Path, language: &Language) -> Option<(Arc<Source>, Tree)> {
        let source = gated_read_source(path, self, PARSE_BYTE_CAP)?;
        let tree = parser_for(language).parse(source.as_bytes(), None)?;
        Some((source, tree))
    }

    /// [`Self::parse_tree`] for each of `files`, handing each parse to
    /// `visit` in index order. Files parse on one worker per core, and a
    /// file starts parsing only while the sources of the parses not yet
    /// visited total at most [`PARSE_BYTE_CAP`] (or none are pending), so
    /// the trees alive at once stay within what one capped file costs.
    pub fn parse_each(
        &self,
        files: &[(&Path, Language)],
        mut visit: impl FnMut(usize, Arc<Source>, Tree),
    ) {
        let sources: Vec<Option<Arc<Source>>> = files
            .iter()
            .map(|(path, _)| gated_read_source(path, self, PARSE_BYTE_CAP))
            .collect();
        let source_len = |index: usize| sources[index].as_ref().map_or(0, |source| source.len());
        let workers = std::thread::available_parallelism()
            .map_or(1, usize::from)
            .min(files.len());
        let (job_sender, job_receiver) = mpsc::channel::<usize>();
        let job_receiver = Mutex::new(job_receiver);
        let (parsed_sender, parsed_receiver) = mpsc::channel();
        std::thread::scope(|scope| {
            for _ in 0..workers {
                let parsed_sender = parsed_sender.clone();
                let (job_receiver, sources) = (&job_receiver, &sources);
                scope.spawn(move || {
                    while let Ok(index) = job_receiver.lock().expect("parse job queue").recv() {
                        let parsed = std::panic::catch_unwind(AssertUnwindSafe(|| {
                            let source = sources[index].as_ref()?;
                            parser_for(&files[index].1).parse(source.as_bytes(), None)
                        }));
                        if parsed_sender.send((index, parsed)).is_err() {
                            return;
                        }
                    }
                });
            }
            drop(parsed_sender);
            let mut next_job = 0;
            let mut pending_bytes = 0;
            let mut parsed_out_of_order = HashMap::new();
            for (index, source) in sources.iter().enumerate() {
                while next_job < files.len()
                    && (pending_bytes == 0
                        || pending_bytes + source_len(next_job) <= PARSE_BYTE_CAP)
                {
                    pending_bytes += source_len(next_job);
                    job_sender
                        .send(next_job)
                        .expect("parse workers outlive the jobs");
                    next_job += 1;
                }
                let parsed = loop {
                    if let Some(parsed) = parsed_out_of_order.remove(&index) {
                        break parsed;
                    }
                    let (parsed_index, parsed) = parsed_receiver
                        .recv()
                        .expect("parse workers outlive the jobs");
                    parsed_out_of_order.insert(parsed_index, parsed);
                };
                let tree = parsed.unwrap_or_else(|panic| std::panic::resume_unwind(panic));
                if let (Some(source), Some(tree)) = (source, tree) {
                    visit(index, source.clone(), tree);
                }
                pending_bytes -= source_len(index);
            }
            drop(job_sender);
        });
    }

    /// `true` iff `file` is a Cargo workspace-member `Cargo.toml`.
    pub fn is_cargo_workspace_member(&self, file: &Path) -> bool {
        self.cargo_workspace
            .is_member(file, || toml::collect_workspace_members(&self.root))
    }

    /// `true` iff `file` is a JS/TS workspace-member `package.json`.
    pub fn is_js_workspace_member(&self, file: &Path) -> bool {
        self.json_state.is_workspace_member(file, &self.root)
    }

    /// `true` iff `file` is the unique primary JS/TS workspace member.
    pub fn is_primary_js_workspace_member(&self, file: &Path) -> bool {
        self.json_state
            .is_primary_workspace_member(file, &self.root)
    }
}

fn parser_for(language: &Language) -> tree_sitter::Parser {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(language)
        .expect("tree-sitter language load");
    parser
}

/// Largest file precis parses. Past it a file is treated like a
/// generated or minified one and yields no batches: the 186-repo
/// robustness sweep's largest hand-written single-file library is
/// 4.1 MB (`miniaudio.h`), while a 25.9 MB generated `parser.c` cost
/// 900 MB and seconds to parse.
const PARSE_BYTE_CAP: usize = 8 * 1024 * 1024;

pub(in crate::walker) fn first_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == kind)
}

/// Path-relative location prior: depth penalty × non-essential-dir
/// discount. Use [`file_depth_factor`] to add entrypoint pinning.
pub(crate) fn path_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(file, ctx, false)
}

/// [`path_depth_factor`] with optional entrypoint pinning — when
/// `is_entrypoint`, depth clamps to 1 so an `index.ts` at any depth
/// ranks like depth 1. Non-essential discount still applies.
pub(crate) fn file_depth_factor(file: &Path, ctx: &WalkCtx, is_entrypoint: bool) -> f64 {
    let depth = ctx.depth_from_root(file);
    let pinned_depth = if is_entrypoint { depth.min(1) } else { depth };
    crate::value::depth_factor(pinned_depth) * ctx.non_essential_factor(file)
}

/// `BatchContent::Lines` rendering `rows` of one file in full. `None`
/// if the resulting span set is empty.
pub(crate) fn single_file_lines_content(
    path: &Path,
    source: &Source,
    rows: Vec<usize>,
) -> Option<BatchContent> {
    let spans = build_file_spans(path, source, rows);
    if spans.is_empty() {
        return None;
    }
    Some(BatchContent::Lines {
        spans,
        units: Vec::new(),
    })
}

/// Cached read behind an FS-metadata byte pre-flight — skips the read
/// (and returns `None`) when the size hint alone disqualifies the
/// file. Bytes-per-line multipliers are per-format — callers keep
/// their own gate constants.
pub(crate) fn gated_read_source(
    file: &Path,
    ctx: &WalkCtx,
    byte_gate: usize,
) -> Option<Arc<Source>> {
    if std::fs::metadata(file).is_ok_and(|m| m.len() as usize > byte_gate) {
        return None;
    }
    ctx.read_source(file)
}

/// Whole-file content behind a size gate: [`gated_read_source`] byte
/// pre-flight, then a line cap on the read source.
pub(crate) fn gated_whole_file_content(
    file: &Path,
    ctx: &WalkCtx,
    byte_gate: usize,
    line_cap: usize,
) -> Option<BatchContent> {
    let source = gated_read_source(file, ctx, byte_gate)?;
    if source.line_count() > line_cap {
        return None;
    }
    single_file_lines_content(file, &source, (1..=source.line_count()).collect())
}

/// Rows (any order, duplicates allowed) → contiguous full-line [`Span`]
/// ranges. Blank source lines are dropped at span edges but **preserved
/// when interior**: a gap between two kept rows that consists solely of
/// blank source rows is bridged into one span, so the emitted region
/// mirrors the source's shape (NS spans are contiguous ranges that
/// include interior blanks). Walkers emit no `…` rows of their own; the
/// renderer marks every elided non-blank gap.
pub(crate) fn build_file_spans(path: &Path, source: &Source, rows: Vec<usize>) -> Vec<Span> {
    let blank = |n: usize| source.line(n).is_some_and(|t| t.trim().is_empty());
    let full: BTreeSet<usize> = rows
        .into_iter()
        .filter(|&n| source.line(n).is_some_and(|t| !t.trim().is_empty()))
        .collect();

    let mut spans = Vec::new();
    // Merge runs of Full line numbers into single-range spans. An
    // all-blank gap is an empty or bridgeable range, so one condition
    // covers both adjacency and interior-blank bridging.
    let full_vec: Vec<usize> = full.iter().copied().collect();
    let mut i = 0;
    while i < full_vec.len() {
        let start = full_vec[i];
        let mut end = start;
        while i + 1 < full_vec.len() && (end + 1..full_vec[i + 1]).all(blank) {
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
    spans
}

// --- shared tree-sitter span helpers ---

/// 0-based final row covered by `node`, ignoring trailing whitespace.
/// Trimming all whitespace (not just newlines) matters for grammars
/// whose block nodes swallow the next sibling's leading indentation
/// (tree-sitter-markdown list items): a final row the node covers only
/// with whitespace renders nothing and must not be claimed — sibling
/// batches each claiming it is a walker-contract overlap.
pub(crate) fn node_end_row_trimmed(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    node.start_position().row + text.trim_end().split('\n').count().max(1) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_shapes(source: &str, rows: Vec<usize>) -> Vec<(usize, usize, Render)> {
        build_file_spans(Path::new("f.txt"), &Source::new(source.into()), rows)
            .into_iter()
            .map(|s| (s.start, s.end, s.render))
            .collect()
    }

    #[test]
    fn walker_mod_parse_skips_files_over_the_byte_cap() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("small.c"), "int x;\n").unwrap();
        std::fs::write(root.join("huge.c"), " ".repeat(PARSE_BYTE_CAP + 1)).unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let c: Language = tree_sitter_c::LANGUAGE.into();
        assert!(ctx.parse_tree(&root.join("small.c"), &c).is_some());
        assert!(ctx.parse_tree(&root.join("huge.c"), &c).is_none());
    }

    #[test]
    fn walker_mod_build_file_spans_bridges_interior_blank_gaps() {
        // Rows 2 and 4 are blank; collecting 1/3/5 must yield one span
        // covering the whole region, blanks included.
        let source = "a\n\nb\n\nc\n";
        let shapes = span_shapes(source, vec![1, 3, 5]);
        assert_eq!(shapes, vec![(1, 5, Render::Full)]);
    }

    #[test]
    fn walker_mod_build_file_spans_drops_edge_blanks_and_content_gaps() {
        // Blank rows at the edges of the collected set never render, and
        // a gap containing an uncollected *content* row is not bridged.
        let source = "\na\nskipped\nb\n\n";
        let shapes = span_shapes(source, vec![1, 2, 4, 5]);
        assert_eq!(shapes, vec![(2, 2, Render::Full), (4, 4, Render::Full)]);
    }
}
