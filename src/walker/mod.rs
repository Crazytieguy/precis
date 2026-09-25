//! Walker trait + shared context.
//!
//! A walker emits [`Batch`] units containing the key, optional
//! predecessor edge, fully-built `BatchContent`, and a scalar `value`. The
//! scheduler ranks emitted batches by `value / cost^k`, gates by
//! predecessor scheduling, and applies content to the rendered tree.
//! Per-file parses are cached on [`WalkCtx`].

use std::cell::{OnceCell, RefCell};
use std::collections::{BTreeSet, HashMap};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    /// Tree-sitter parse results, keyed by path.
    tree_cache: RefCell<HashMap<PathBuf, Arc<Tree>>>,
    cargo_workspace: workspace::CargoWorkspace,
    fs_state: fs::FsState,
    json_state: json::JsonState,
    /// Run state of the code engine's language modules.
    code: code::CodeState,
    /// The tree's essential source, walked once.
    essential_source: OnceCell<EssentialSource>,
    /// The one source file that carries a dominant share of the tree's
    /// essential source bytes, if any.
    dominant_source_file: OnceCell<Option<PathBuf>>,
}

impl Drop for WalkCtx {
    /// Freeing every parse tree is a measurable slice of a run and
    /// nothing waits on it, so it happens on a background thread (which
    /// the CLI exits without joining).
    fn drop(&mut self) {
        let trees = std::mem::take(self.tree_cache.get_mut());
        std::thread::spawn(move || drop(trees));
    }
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
            tree_cache: RefCell::new(HashMap::new()),
            cargo_workspace: workspace::CargoWorkspace::default(),
            fs_state: fs::FsState::default(),
            json_state: json::JsonState::default(),
            code: code::CodeState::default(),
            essential_source: OnceCell::new(),
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
            .get_or_init(|| find_dominant_source_file(self.essential_source()))
            .as_deref()
    }

    fn essential_source(&self) -> &EssentialSource {
        self.essential_source
            .get_or_init(|| enumerate_essential_source(&self.root, &self.dir_filter))
    }

    /// Read `path` into memory, caching the result.
    pub fn read_source(&self, path: &Path) -> Option<Arc<Source>> {
        self.source_cache.get(path)
    }

    /// Parse `path` with `language`, caching the result.
    pub fn parse_tree(&self, path: &Path, language: &Language) -> Option<(Arc<Source>, Arc<Tree>)> {
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

    /// Parse every file of `files` not parsed yet, across threads, into
    /// the cache [`Self::parse_tree`] reads — for callers about to parse
    /// a whole set of files one by one.
    pub fn parse_trees<'a>(&self, files: impl IntoIterator<Item = (&'a Path, Language)>) {
        let pending: Vec<(&Path, Arc<Source>, Language)> = files
            .into_iter()
            .filter(|(path, _)| !self.tree_cache.borrow().contains_key(*path))
            .filter_map(|(path, language)| Some((path, self.read_source(path)?, language)))
            .collect();
        let next = AtomicUsize::new(0);
        let parse_pending = || {
            let mut parser = tree_sitter::Parser::new();
            let mut trees = Vec::new();
            while let Some((path, source, language)) =
                pending.get(next.fetch_add(1, Ordering::Relaxed))
            {
                parser
                    .set_language(language)
                    .expect("tree-sitter language load");
                trees.extend(
                    parser
                        .parse(source.as_bytes(), None)
                        .map(|tree| (*path, tree)),
                );
            }
            trees
        };
        let workers = std::thread::available_parallelism()
            .map_or(1, usize::from)
            .min(pending.len());
        let trees: Vec<(&Path, Tree)> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..workers).map(|_| scope.spawn(parse_pending)).collect();
            handles
                .into_iter()
                .flat_map(|handle| handle.join().expect("parse worker panicked"))
                .collect()
        });
        let mut cache = self.tree_cache.borrow_mut();
        for (path, tree) in trees {
            cache.insert(path.to_path_buf(), Arc::new(tree));
        }
    }

    /// `true` iff `file` is a Cargo workspace-member `Cargo.toml`.
    pub fn is_workspace_member(&self, file: &Path) -> bool {
        self.cargo_workspace.is_member(file, &self.root)
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

pub(in crate::walker) fn first_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == kind)
}

/// Minimum share of the tree's essential source bytes for the largest
/// source file to count as the repository's spine. Swept full-corpus;
/// see `git show a90ee9b6:docs/design-notes.md` ("Dominant source file").
const DOMINANT_SOURCE_MASS_SHARE: f64 = 0.20;

/// Upper bound on a spine file's size. Past this, a single file is a
/// generated table or an amalgamated bundle rather than something a
/// reader is meant to read more of.
const MASS_SHARE_MAX_FILE_BYTES: u64 = 400_000;

/// The tree's essential source files, as one walk: byte mass per
/// language family, plus the per-file candidate list (files under
/// [`MASS_SHARE_MAX_FILE_BYTES`]). Enumeration rules are documented on
/// [`find_dominant_source_file`], the original caller.
struct EssentialSource {
    per_language: HashMap<&'static str, u64>,
    candidates: Vec<(PathBuf, u64, &'static str)>,
}

fn enumerate_essential_source(root: &Path, filter: &DirFilter) -> EssentialSource {
    let mut per_language: HashMap<&'static str, u64> = HashMap::new();
    let mut candidates: Vec<(PathBuf, u64, &'static str)> = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
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
                if !crate::fs_util::should_skip_dir(&name) && !holds_derived_code(&name) {
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
                if len <= MASS_SHARE_MAX_FILE_BYTES {
                    candidates.push((path, len, language));
                }
            }
        }
    }
    EssentialSource {
        per_language,
        candidates,
    }
}

/// Directories of code nobody wrote by hand — generated output, test
/// corpora, vendored copies at any depth. Their bytes are source but not
/// the repository's own, so they neither win nor dilute the mass share.
fn holds_derived_code(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(lower.as_str(), "generated" | "testdata") || crate::value::is_vendor_dir_name(&lower)
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
/// containment answer typed source discovery gives. [`holds_derived_code`]
/// narrows that universe, and a candidate whose text reads as
/// machine-generated (a banner, or minified line lengths) never wins.
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
            *language == primary && *len as f64 / total as f64 >= DOMINANT_SOURCE_MASS_SHARE
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

pub(crate) fn dedup_sorted(mut v: Vec<usize>) -> Vec<usize> {
    v.sort();
    v.dedup();
    v
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
        find_dominant_source_file(&enumerate_essential_source(root, &DirFilter::new(root)))
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
        std::fs::create_dir_all(root.join("src/generated")).unwrap();
        std::fs::create_dir_all(root.join("lib/vendor")).unwrap();
        std::fs::write(root.join("src/generated/schema.ts"), "x = 1\n".repeat(900)).unwrap();
        std::fs::write(root.join("lib/vendor/dep.ts"), "x = 1\n".repeat(900)).unwrap();
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
