//! Walker trait + shared context.
//!
//! A walker emits [`Batch<K>`] units containing the key, optional
//! predecessor edge, fully-built `BatchContent`, and a scalar `value`. The
//! scheduler ranks emitted batches by `value / cost^k`, gates by
//! predecessor scheduling, and applies content to the rendered tree.
//!
//! Walkers are encouraged to keep emission as cheap as feasible (the
//! scheduler does the heavy lifting around ranking and applying), but
//! parsing in `expand` is fine — every per-file parse is cached on
//! [`WalkCtx`] and shared across all batches that touch the same file.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Language, Node, Tree};

use crate::batch::{Batch, BatchKey, FsKey, WalkerKey};
use crate::content::{BatchContent, Render, Span};
use crate::render::SourceCache;

pub mod c;
pub mod fs;
pub mod go;
pub(crate) mod import_chunks;
pub mod json;
pub mod markdown;
pub mod plaintext;
pub mod python;
pub mod rust;
pub mod toml;
pub mod typescript;

/// Walker contract. The associated `Key` type is walker-private: scheduler +
/// renderer never name it, and adding a new walker doesn't change
/// scheduler/renderer code.
pub trait Walker {
    type Key: WalkerKey;

    /// Initial batches emitted before any scheduling decision. Typically
    /// the root filesystem listing.
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch<Self::Key>>;

    /// Called when a batch is scheduled — returns newly-discovered
    /// batches. `scheduled` is the key just moved into the tree; the
    /// walker uses it to decide what to propose next (e.g. once a
    /// directory listing is scheduled, language walkers emit per-file
    /// batches for files in that dir).
    fn expand(&mut self, scheduled: &Self::Key, ctx: &WalkCtx) -> Vec<Batch<Self::Key>>;
}

/// Top-level walker: filesystem listings drive discovery; per-language
/// modules (`rust`, `markdown`, `toml`, `typescript`, `json`, `plaintext`)
/// own per-dir candidate emission and are dispatched here. There's no
/// trait-object indirection — the language list is a closed set known at
/// this site.
#[derive(Default)]
pub struct FsWalker;

impl Walker for FsWalker {
    type Key = BatchKey;

    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
        fs::seed(ctx)
    }

    fn expand(&mut self, scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
        let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
            return Vec::new();
        };
        let mut out = Vec::new();
        out.extend(fs::expand_subdirs(dir, ctx));
        out.extend(rust::expand_in_dir(dir, ctx));
        out.extend(markdown::expand_in_dir(dir, ctx));
        out.extend(toml::expand_in_dir(dir, ctx));
        out.extend(typescript::expand_in_dir(dir, ctx));
        out.extend(json::expand_in_dir(dir, ctx));
        out.extend(plaintext::expand_in_dir(dir, ctx));
        out.extend(c::expand_in_dir(dir, ctx));
        out.extend(go::expand_in_dir(dir, ctx));
        out.extend(python::expand_in_dir(dir, ctx));
        out
    }
}

/// Per-run context. Holds the seed root plus a shared source cache +
/// per-file parse cache. The source cache is the same handle the
/// [`RenderedTree`](crate::render::RenderedTree) uses for render-time
/// materialization, so each file is read at most once across the whole run.
///
/// Per-walker run state — cross-file analyses each language wants to
/// memoize for the run — lives in language-named fields below. Each
/// language walker's state struct is defined alongside that walker; the
/// shared cells stay typed and explicit rather than going through a
/// `TypeId`-keyed bag. New languages add a field here and own its
/// initialization.
pub struct WalkCtx {
    root: PathBuf,
    source_cache: SourceCache,
    /// Tree-sitter parse results, keyed by path.
    tree_cache: RefCell<HashMap<PathBuf, Arc<Tree>>>,
    /// Per-run state owned by `walker::rust` — module visibility,
    /// workspace membership, exported-macro names per dir.
    rust_state: rust::RustState,
    /// Per-run state owned by `walker::fs` — cached filesystem-shape probes.
    fs_state: fs::FsState,
    /// Per-run state owned by `walker::json` — npm/yarn/pnpm
    /// workspace-member resolution.
    json_state: json::JsonState,
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
            rust_state: rust::RustState::new(),
            fs_state: fs::FsState::default(),
            json_state: json::JsonState::default(),
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

    pub(in crate::walker) fn rust_state(&self) -> &rust::RustState {
        &self.rust_state
    }

    pub(in crate::walker) fn fs_state(&self) -> &fs::FsState {
        &self.fs_state
    }

    /// `true` iff `file` is a `Cargo.toml` declared (or auto-promoted) as
    /// a workspace member by the seed-root `Cargo.toml`. The TOML walker
    /// uses this to dampen `[package]` identity weighting on sub-crate
    /// manifests (where most identity is inherited from the workspace
    /// root). Lives on `RustState` because workspace resolution is a
    /// Cargo concept and the cache should die with the run.
    pub fn is_workspace_member(&self, file: &Path) -> bool {
        self.rust_state.is_workspace_member(file, &self.root)
    }

    /// `true` iff `file` is a `package.json` declared as a member of the
    /// seed-root JS/TS workspace (npm/yarn `workspaces` field or
    /// `pnpm-workspace.yaml` `packages:` list). Mirrors
    /// [`Self::is_workspace_member`] for the JSON walker, which uses it to
    /// damp `Identity` on nested package manifests where most metadata
    /// is inherited from / orchestrated by the workspace root.
    pub fn is_js_workspace_member(&self, file: &Path) -> bool {
        self.json_state.is_workspace_member(file, &self.root)
    }
}

/// Lines a collector wants to render for one file: `full` = emit the source
/// line verbatim; `ellipses` = emit a walker `…` marker at that line number
/// (no text, no rendered line number — but a real line number so descendant
/// batches can override it with real content).
#[derive(Default, Debug, Clone)]
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

pub(crate) fn file_lines_covered_by(child: &FileLines, parent: &FileLines) -> bool {
    child.full.iter().all(|line| parent.full.contains(line))
        && child
            .ellipses
            .iter()
            .all(|line| parent.ellipses.contains(line) || parent.full.contains(line))
}

/// Path-relative location prior shared by every per-file walker: depth
/// penalty (`value::depth_factor`) folded with the non-essential-directory
/// discount (`WalkCtx::non_essential_factor`). Walkers without an
/// entrypoint concept call this directly; walkers that pin entrypoints to
/// depth ≤ 1 use [`file_depth_factor`] which adds that knob.
pub(crate) fn path_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    crate::value::depth_factor(ctx.depth_from_root(file)) * ctx.non_essential_factor(file)
}

/// [`path_depth_factor`] with optional entrypoint pinning. When
/// `is_entrypoint` is true the depth is clamped to 1 — an `index.ts` at
/// depth 5 ranks the same as one at depth 1, since it's the file the
/// agent looks at first regardless of how the package is laid out. The
/// non-essential discount still applies (an entrypoint inside `tests/`
/// doesn't get an unconditional pass).
pub(crate) fn file_depth_factor(file: &Path, ctx: &WalkCtx, is_entrypoint: bool) -> f64 {
    let depth = ctx.depth_from_root(file);
    let pinned_depth = if is_entrypoint { depth.min(1) } else { depth };
    crate::value::depth_factor(pinned_depth) * ctx.non_essential_factor(file)
}

/// Build a [`BatchContent::Lines`] from a single file's [`FileLines`]
/// spec. Blank source lines are filtered from the `full` set; an ellipsis
/// at a line already in `full` is dropped. Returns `None` when the
/// resulting span set is empty (caller declines to emit the batch).
pub(crate) fn single_file_lines_content(
    path: &Path,
    source: &str,
    lines: FileLines,
) -> Option<BatchContent> {
    let spans = build_file_spans(path, source, lines);
    if spans.is_empty() {
        return None;
    }
    Some(BatchContent::Lines { spans })
}

/// Parse `file` and run a per-file `FileLines` collector, yielding a
/// `BatchContent::Lines` if the result is non-empty. Used by walkers
/// whose per-file batches share the parse-then-collect-spans shape
/// (currently rust + ts). Caller supplies the parser closure so
/// language-specific parser dispatch (e.g. `.ts` vs `.tsx`) stays in
/// the language module.
pub(crate) fn build_per_file_content(
    file: &Path,
    ctx: &WalkCtx,
    parse: impl Fn(&WalkCtx, &Path) -> Option<(Arc<str>, Arc<Tree>)>,
    collect: impl Fn(&Tree, &str) -> FileLines,
) -> Option<BatchContent> {
    let (source, tree) = parse(ctx, file)?;
    let lines = collect(&tree, &source);
    single_file_lines_content(file, &source, lines)
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
    push_rows(
        out,
        node.start_position().row,
        node_end_row_trimmed(node, source),
    );
}

/// 0-based final row covered by `node`, ignoring trailing newline bytes.
pub(crate) fn node_end_row_trimmed(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    node.start_position().row
        + text
            .trim_end_matches(['\n', '\r'])
            .split('\n')
            .count()
            .max(1)
        - 1
}

/// Declared `name` field as source text, when the grammar exposes one.
pub(crate) fn name_of<'a>(node: Node, source: &'a str) -> Option<&'a str> {
    let name = node.child_by_field_name("name")?;
    Some(&source[name.start_byte()..name.end_byte()])
}

/// Append non-blank rows from the inclusive 0-based range as 1-based lines.
pub(crate) fn extend_nonblank_rows(
    out: &mut Vec<usize>,
    src_lines: &[&str],
    start_row: usize,
    end_row: usize,
) {
    for row in start_row..=end_row {
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(row + 1);
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BodyPart {
    pub lines: Vec<usize>,
}

impl BodyPart {
    pub(crate) fn start_line(&self) -> Option<usize> {
        self.lines.first().copied()
    }
}

// Below roughly a dozen emitted atoms, splitting usually costs more scheduling
// surface than it saves: descendants are tiny and the parent body batch is
// already cheap enough to carry as a single region.
pub(crate) const BODY_SPLIT_MIN_LINES: usize = 12;

pub(crate) fn body_part_value_factor(part_count: usize) -> f64 {
    if part_count <= 1 {
        1.0
    } else {
        1.0 / part_count as f64
    }
}

/// Body slices for a brace-delimited statement block, using top-level
/// statements inside the block. Blank lines are filtered exactly like
/// materialized spans.
pub(crate) fn statement_block_parts(
    body: Option<Node>,
    src_lines: &[&str],
    block_kind: &str,
) -> Vec<BodyPart> {
    let Some(b) = body else { return Vec::new() };
    if b.kind() != block_kind {
        return Vec::new();
    }
    let mut cursor = b.walk();
    let named_children: Vec<_> = b.named_children(&mut cursor).collect();
    let body_start = b.start_position().row;
    let body_end = b.end_position().row;
    // Python function blocks are indentation-delimited; Rust/TS blocks include brace rows.
    let undelimited_block = block_kind == "block"
        && b.parent()
            .is_some_and(|parent| parent.kind() == "function_definition")
        && named_children
            .first()
            .is_some_and(|child| child.start_position().row == body_start);
    let content_start = if undelimited_block {
        body_start
    } else {
        body_start + 1
    };
    let content_end = if undelimited_block {
        body_end
    } else {
        body_end.saturating_sub(1)
    };
    if content_end < content_start {
        return Vec::new();
    }
    let mut interior = Vec::new();
    extend_nonblank_rows(&mut interior, src_lines, content_start, content_end);
    let interior = dedup_sorted(interior);
    if interior.is_empty() {
        return Vec::new();
    }
    if interior.len() <= BODY_SPLIT_MIN_LINES {
        return vec![BodyPart { lines: interior }];
    }

    let mut parts = Vec::new();
    for child in named_children {
        let start_row = child.start_position().row.max(content_start);
        let end_row = child.end_position().row.min(content_end);
        if end_row < start_row {
            continue;
        }
        let mut lines = Vec::new();
        extend_nonblank_rows(&mut lines, src_lines, start_row, end_row);
        let lines = dedup_sorted(lines);
        if !lines.is_empty() {
            parts.push(BodyPart { lines });
        }
    }
    if parts.len() <= 1 {
        // Named children omit comment-only/interstitial content. Preserve the
        // complete block when there is nothing meaningful to split.
        vec![BodyPart { lines: interior }]
    } else {
        parts
    }
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
