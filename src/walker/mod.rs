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
use crate::fs_util::{DirFilter, PROBE_ENTRY_CAP};
use crate::render::{Source, SourceCache};

pub(crate) mod code;
pub mod fs;
pub mod go_mod;
pub mod json;
pub mod markdown;
pub mod plaintext;
pub mod prisma;
pub mod toml;
mod workspace;

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
        let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
            return Vec::new();
        };
        let mut out = Vec::new();
        out.extend(fs::expand_subdirs(dir, ctx));
        out.extend(markdown::expand_in_dir(dir, ctx));
        out.extend(toml::expand_in_dir(dir, ctx));
        out.extend(json::expand_in_dir(dir, ctx));
        out.extend(plaintext::expand_in_dir(dir, ctx));
        out.extend(prisma::expand_in_dir(dir, ctx));
        out.extend(go_mod::expand_in_dir(dir, ctx));
        out.extend(code::expand_in_dir(dir, ctx));
        out
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
            dominant_source_file: OnceCell::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn dir_filter(&self) -> &DirFilter {
        &self.dir_filter
    }

    /// Shared handle for the renderer, so cost probes reuse this run's
    /// ignore caches instead of rebuilding them per probe.
    pub fn dir_filter_handle(&self) -> Rc<DirFilter> {
        Rc::clone(&self.dir_filter)
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
                enumerate_essential_source(&self.root, &self.dir_filter)
                    .and_then(|source| find_dominant_source_file(&source))
            })
            .as_deref()
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
    pub fn is_workspace_member(&self, file: &Path) -> bool {
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

/// Minimum share of the tree's essential source bytes for the largest
/// source file to count as the repository's spine. Swept full-corpus;
/// see `git show a90ee9b6:docs/design-notes.md` ("Dominant source file").
const DOMINANT_SOURCE_MASS_SHARE: f64 = 0.20;

/// Upper bound on a spine file's size, checked before the file is read.
/// Past this, a single file is a generated table or an amalgamated bundle
/// rather than something a reader is meant to read more of.
const DOMINANT_SOURCE_MAX_FILE_BYTES: u64 = 400_000;

/// The tree's essential source files, as one walk: byte mass per
/// language family, plus the per-file candidate list. Enumeration rules
/// are documented on [`find_dominant_source_file`].
struct EssentialSource {
    per_language: HashMap<&'static str, u64>,
    candidates: Vec<(PathBuf, u64, &'static str)>,
}

/// `None` once the walk has seen more source than a file within
/// [`DOMINANT_SOURCE_MAX_FILE_BYTES`] can hold
/// [`DOMINANT_SOURCE_MASS_SHARE`] of: no file can be the spine then, and
/// stopping there keeps the survey from walking all of a huge tree. `None`
/// too once it has read [`PROBE_ENTRY_CAP`] entries; non-essential
/// directories are not entered, since nothing under one counts, so a
/// large test corpus can't spend that budget.
fn enumerate_essential_source(root: &Path, filter: &DirFilter) -> Option<EssentialSource> {
    let mut per_language: HashMap<&'static str, u64> = HashMap::new();
    let mut candidates: Vec<(PathBuf, u64, &'static str)> = Vec::new();
    let mut total = 0;
    let mut entries_read = 0;
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            entries_read += 1;
            if entries_read > PROBE_ENTRY_CAP {
                return None;
            }
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let path = entry.path();
            if filter.excludes(&path, file_type.is_dir()) {
                continue;
            }
            if file_type.is_dir() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if !crate::fs_util::should_skip_dir(&name)
                    && crate::value::non_essential_factor(&path, root) >= 1.0
                {
                    stack.push(path);
                }
            } else if file_type.is_file() {
                let Some(language) = language_group(&path) else {
                    continue;
                };
                if crate::value::non_essential_factor(&path, root) < 1.0 {
                    continue;
                }
                let Ok(len) = entry.metadata().map(|m| m.len()) else {
                    continue;
                };
                *per_language.entry(language).or_default() += len;
                candidates.push((path, len, language));
                total += len;
                if total as f64 * DOMINANT_SOURCE_MASS_SHARE > DOMINANT_SOURCE_MAX_FILE_BYTES as f64
                {
                    return None;
                }
            }
        }
    }
    Some(EssentialSource {
        per_language,
        candidates,
    })
}

/// Select a source file that carries at least
/// [`DOMINANT_SOURCE_MASS_SHARE`] of the total — the largest one when
/// several do. Non-essential subtrees (tests, examples, vendored, tooling) are
/// excluded from both the numerator and the denominator so a big test file can neither win nor
/// dilute the share.
///
/// Enumeration mirrors the walkers' own rules rather than inventing a
/// second traversal policy: gitignore exclusion via `filter`, the
/// heavy-directory blocklist via [`crate::fs_util::should_skip_dir`]
/// (the only thing bounding a walk of a non-repository tree, where the
/// filter is inert by design), and non-following file types so a
/// symlink is neither descended into nor weighed as source — the same
/// containment answer typed source discovery gives. A candidate over
/// [`DOMINANT_SOURCE_MAX_FILE_BYTES`] is never read, and one whose text
/// reads as machine-generated (a banner, or minified line lengths) never
/// wins.
fn find_dominant_source_file(source: &EssentialSource) -> Option<PathBuf> {
    // The spine has to be written in the language the repository is
    // written in — a vendored JS bundle inside a Go tree is source mass
    // but it is not what the repo is about. Ranked over a sorted vector
    // rather than the hash map's iteration order, and a tie for the lead
    // yields no primary at all: "the language this repo is written in"
    // has no answer there, and answering it by hasher seeding would make
    // the output differ between processes on identical input.
    let mut by_mass: Vec<(&'static str, u64)> = source
        .per_language
        .iter()
        .map(|(&language, &bytes)| (language, bytes))
        .collect();
    by_mass.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    let (primary, primary_bytes) = *by_mass.first()?;
    if by_mass
        .get(1)
        .is_some_and(|&(_, bytes)| bytes == primary_bytes)
    {
        return None;
    }
    let total: u64 = by_mass.iter().map(|&(_, bytes)| bytes).sum();
    if total == 0 {
        return None;
    }
    source
        .candidates
        .iter()
        .filter(|(_, len, language)| {
            *language == primary
                && *len <= DOMINANT_SOURCE_MAX_FILE_BYTES
                && *len as f64 / total as f64 >= DOMINANT_SOURCE_MASS_SHARE
        })
        .filter(|(path, _, _)| {
            std::fs::read_to_string(path)
                .is_ok_and(|text| !plaintext::is_machine_generated_text(&text))
        })
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .map(|(path, _, _)| path.clone())
}

/// Extension → language family, collapsing the families whose files sit
/// side by side in one codebase (a `.h` beside its `.c`, a `.js` beside
/// its `.ts`). `None` for anything that isn't hand-authored code.
fn language_group(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    Some(match ext.as_str() {
        "c" | "cc" | "cpp" | "cxx" | "h" | "hpp" => "c",
        "js" | "jsx" | "cjs" | "mjs" | "ts" | "tsx" => "js",
        "go" => "go",
        "lua" => "lua",
        "py" => "py",
        "rb" => "rb",
        "rs" => "rs",
        "swift" => "swift",
        "zig" => "zig",
        _ => return None,
    })
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
    Some(BatchContent::Lines { spans })
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
    let byte_len = std::fs::metadata(file)
        .map(|m| m.len() as usize)
        .unwrap_or(usize::MAX);
    if byte_len > byte_gate {
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

    fn dominant_source_file_of(root: &Path) -> Option<PathBuf> {
        enumerate_essential_source(root, &DirFilter::new(root))
            .and_then(|source| find_dominant_source_file(&source))
    }

    /// The spine detector runs on whatever path a user points precis
    /// at, including trees that are not repositories — where `DirFilter`
    /// is inert and the heavy-directory blocklist is the only thing
    /// between the scan and a dependency tree.
    #[test]
    fn walker_mod_dominant_file_skips_heavy_dirs_on_non_git_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("node_modules/dep")).unwrap();
        std::fs::create_dir_all(root.join("src")).unwrap();
        // Bigger than the spine, and in the same language — if it were
        // scanned it would both win the candidate race and dilute the
        // share out of range.
        std::fs::write(
            root.join("node_modules/dep/bundle.js"),
            "x = 1\n".repeat(4000),
        )
        .unwrap();
        std::fs::write(root.join("src/core.js"), "y = 2\n".repeat(100)).unwrap();

        let found = dominant_source_file_of(root);
        assert_eq!(found.as_deref(), Some(root.join("src/core.js").as_path()));
    }

    /// Typed source discovery rejects symlinks (non-following file
    /// types), so a link must not be weighed as source mass or selected
    /// as the spine — otherwise the scheduler boosts a path no walker
    /// ever emits, and the link's target is double-counted.
    #[cfg(unix)]
    #[test]
    fn walker_mod_dominant_file_ignores_in_root_file_symlink() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("core.py"), "y = 2\n".repeat(100)).unwrap();
        std::os::unix::fs::symlink("core.py", root.join("alias.py")).unwrap();
        std::fs::write(root.join("helper.py"), "z = 3\n".repeat(60)).unwrap();

        let found = dominant_source_file_of(root);
        assert_eq!(found.as_deref(), Some(root.join("core.py").as_path()));

        // Removing the real file leaves only the link plus a peer; the
        // link must not stand in for the mass it points at.
        std::fs::remove_file(root.join("core.py")).unwrap();
        assert_eq!(
            dominant_source_file_of(root).as_deref(),
            Some(root.join("helper.py").as_path())
        );
    }

    /// Two languages at exactly equal mass have no "primary", and the
    /// answer must not come from hash iteration order — same input, same
    /// output, across processes.
    #[test]
    fn walker_mod_dominant_file_declines_a_tied_primary_language() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("core.py"), "y = 2\n".repeat(100)).unwrap();
        std::fs::write(root.join("core.go"), "y = 2\n".repeat(100)).unwrap();
        assert_eq!(dominant_source_file_of(root), None);

        // One byte of lead is enough to make the question answerable.
        std::fs::write(root.join("core.py"), "y = 2\n".repeat(100) + "z").unwrap();
        assert_eq!(
            dominant_source_file_of(root).as_deref(),
            Some(root.join("core.py").as_path())
        );
    }

    /// Generated code and minified bundles are source mass nobody reads;
    /// the spine is the hand-written implementation even when either is
    /// bigger.
    #[test]
    fn walker_mod_dominant_file_passes_over_generated_and_minified_code() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src/app.bundle.js"), "var a=1;".repeat(1000)).unwrap();
        std::fs::write(
            root.join("src/client.ts"),
            "// Code generated by protoc. DO NOT EDIT.\n".to_string() + &"x = 1\n".repeat(1000),
        )
        .unwrap();
        std::fs::write(root.join("src/core.ts"), "y = 2\n".repeat(900)).unwrap();
        std::fs::write(root.join("src/util.ts"), "z = 3\n".repeat(200)).unwrap();

        let found = dominant_source_file_of(root);
        assert_eq!(found.as_deref(), Some(root.join("src/core.ts").as_path()));
    }

    /// A checked-in table too large to be read as a spine still counts as
    /// source mass, but is neither read nor picked.
    #[test]
    fn walker_mod_dominant_file_skips_oversized_source() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("src/generated")).unwrap();
        std::fs::write(
            root.join("src/generated/table.c"),
            "int t[] = {1, 2, 3};\n".repeat(25_000),
        )
        .unwrap();
        std::fs::write(root.join("src/main.c"), "int main(void) { return 0; }\n").unwrap();

        assert_eq!(dominant_source_file_of(root), None);
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
