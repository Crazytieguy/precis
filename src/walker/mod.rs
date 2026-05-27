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

use std::cell::{OnceCell, RefCell};
use std::collections::{BTreeSet, HashMap, HashSet};
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
pub mod lua;
pub mod markdown;
pub mod plaintext;
pub mod prisma;
pub mod python;
pub mod rust;
pub mod toml;
pub mod typescript;
mod workspace;
pub mod yaml;

/// Walker contract — `Key` is walker-private so the scheduler/render
/// code stays generic over walkers.
pub trait Walker {
    type Key: WalkerKey;

    /// Initial batches before any scheduling decision (typically the
    /// root FS listing).
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch<Self::Key>>;

    /// Called when a batch is scheduled — returns newly-discovered
    /// batches.
    fn expand(&mut self, scheduled: &Self::Key, ctx: &WalkCtx) -> Vec<Batch<Self::Key>>;
}

/// Top-level walker: FS listings drive discovery; per-language modules
/// own per-dir candidate emission and are dispatched here directly.
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
        out.extend(prisma::expand_in_dir(dir, ctx));
        out.extend(c::expand_in_dir(dir, ctx));
        out.extend(go::expand_in_dir(dir, ctx));
        out.extend(python::expand_in_dir(dir, ctx));
        out.extend(lua::expand_in_dir(dir, ctx));
        out.extend(yaml::expand_in_dir(dir, ctx));
        out
    }
}

/// Per-run context: seed root, shared source/parse caches, and per-
/// walker run state in language-named fields below.
pub struct WalkCtx {
    root: PathBuf,
    source_cache: SourceCache,
    /// Tree-sitter parse results, keyed by path.
    tree_cache: RefCell<HashMap<PathBuf, Arc<Tree>>>,
    rust_state: rust::RustState,
    fs_state: fs::FsState,
    json_state: json::JsonState,
    typescript_state: typescript::TypescriptState,
    c_state: c::CState,
    /// Files hyperlinked from the root README — exempts them from the
    /// `examples/`-style non-essential demotion.
    readme_cited_paths: OnceCell<HashSet<PathBuf>>,
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
            typescript_state: typescript::TypescriptState::new(),
            c_state: c::CState::default(),
            readme_cited_paths: OnceCell::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn source_cache(&self) -> &SourceCache {
        &self.source_cache
    }

    /// Depth of `path` relative to the seed root (root itself = 0).
    pub fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    /// Non-essential discount, scoped to this run's root. README-cited
    /// files skip the `examples/`-style classifier — other discount
    /// classes still apply.
    pub fn non_essential_factor(&self, path: &Path) -> f64 {
        let base = crate::value::non_essential_factor(path, &self.root);
        if base < 1.0 && self.is_readme_cited(path) {
            crate::value::non_essential_factor_inner(path, &self.root, true)
        } else {
            base
        }
    }

    /// True iff `path` is README-cited, or a directory containing a
    /// README-cited file.
    pub fn is_readme_cited(&self, path: &Path) -> bool {
        let cited = self
            .readme_cited_paths
            .get_or_init(|| collect_readme_cited_paths(&self.root));
        if cited.is_empty() {
            return false;
        }
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if cited.contains(&canonical) {
            return true;
        }
        // Directory case: any cited file lives under this directory.
        cited
            .iter()
            .any(|cited_path| cited_path.starts_with(&canonical))
    }

    /// True when `path` is an auto-injected agent doc (AGENTS.md /
    /// CLAUDE.md at root, or a text file under .claude/skills/,
    /// .agent/skills/, .cursor/rules/). Walkers consult this to skip
    /// emitting prose-body batches whose content is already in the
    /// model's context.
    pub fn is_auto_injected_doc_file(&self, path: &Path) -> bool {
        crate::value::is_auto_injected_doc_file(path, &self.root)
    }

    /// Read `path` into memory, caching the result.
    pub fn read_source(&self, path: &Path) -> Option<Arc<str>> {
        self.source_cache.get(path)
    }

    /// Parse `path` with `language`, caching the result.
    pub fn parse_tree(&self, path: &Path, language: &Language) -> Option<(Arc<str>, Arc<Tree>)> {
        #[cfg(feature = "timing")]
        let _start = std::time::Instant::now();
        let source = self.read_source(path)?;
        if let Some(tree) = self.tree_cache.borrow().get(path) {
            #[cfg(feature = "timing")]
            crate::timing::record(|c| &mut c.parse, _start.elapsed(), Some(true));
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
        #[cfg(feature = "timing")]
        crate::timing::record(|c| &mut c.parse, _start.elapsed(), Some(false));
        Some((source, arc))
    }

    pub(in crate::walker) fn rust_state(&self) -> &rust::RustState {
        &self.rust_state
    }

    pub(in crate::walker) fn typescript_state(&self) -> &typescript::TypescriptState {
        &self.typescript_state
    }

    pub(in crate::walker) fn fs_state(&self) -> &fs::FsState {
        &self.fs_state
    }

    pub(in crate::walker) fn c_state(&self) -> &c::CState {
        &self.c_state
    }

    /// `true` iff `file` is a Cargo workspace-member `Cargo.toml`.
    pub fn is_workspace_member(&self, file: &Path) -> bool {
        self.rust_state.is_workspace_member(file, &self.root)
    }

    /// `true` iff `file` is a JS/TS workspace-member `package.json`.
    pub fn is_js_workspace_member(&self, file: &Path) -> bool {
        self.json_state.is_workspace_member(file, &self.root)
    }

    /// `true` iff `file` is in the TS/JS public surface.
    pub fn is_ts_public_surface(&self, file: &Path) -> bool {
        self.typescript_state.is_in_public_surface(file, self)
    }
}

/// Scan the seed root's README for relative-path hyperlinks to source
/// files. Returns canonicalized paths.
fn collect_readme_cited_paths(root: &Path) -> HashSet<PathBuf> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return HashSet::new();
    };
    let mut out = HashSet::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let lower = name.to_ascii_lowercase();
        let is_readme = matches!(lower.as_str(), "readme.md" | "readme.rst" | "readme.txt");
        if !is_readme {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for cited in extract_inline_link_targets(&text) {
            // Only resolve relative references; skip URLs, anchors,
            // and absolute paths.
            if cited.starts_with("http")
                || cited.starts_with("#")
                || cited.starts_with('/')
                || cited.starts_with("mailto:")
            {
                continue;
            }
            // Strip any anchor or query suffix.
            let path_part = cited.split(['#', '?']).next().unwrap_or("");
            if path_part.is_empty() {
                continue;
            }
            // Only count source-file extensions to avoid matching
            // image links / generic documentation links.
            let lower = path_part.to_ascii_lowercase();
            let is_source = [
                ".js", ".mjs", ".cjs", ".ts", ".tsx", ".py", ".rs", ".go", ".c", ".h", ".cc",
                ".cpp", ".hpp",
            ]
            .iter()
            .any(|ext| lower.ends_with(ext));
            if !is_source {
                continue;
            }
            let trimmed = path_part.trim_start_matches("./");
            let resolved = root.join(trimmed);
            if let Ok(canonical) = resolved.canonicalize() {
                out.insert(canonical);
            }
        }
    }
    out
}

/// Extract `(target)` from `[label](target)` patterns in markdown.
/// Skips reference-style and image (`![alt](src)`) links.
fn extract_inline_link_targets(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'!' {
            // Skip image-link prefix; the image's `(target)` is rarely a
            // source file.
            i += 1;
            continue;
        }
        if c != b'[' {
            i += 1;
            continue;
        }
        // Find the matching `]` then check for immediately-following `(`.
        let mut depth = 1;
        let mut j = i + 1;
        while j < bytes.len() && depth > 0 {
            match bytes[j] {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b'\\' => j += 1,
                _ => {}
            }
            j += 1;
        }
        if depth != 0 || j >= bytes.len() || bytes[j] != b'(' {
            i = j;
            continue;
        }
        let target_start = j + 1;
        let mut k = target_start;
        let mut paren_depth = 1;
        while k < bytes.len() && paren_depth > 0 {
            match bytes[k] {
                b'(' => paren_depth += 1,
                b')' => paren_depth -= 1,
                b'\\' => k += 1,
                _ => {}
            }
            k += 1;
        }
        if paren_depth == 0 {
            let target = &text[target_start..k - 1];
            // Strip optional `"title"` suffix — `[label](url "title")`.
            let target = target
                .split_once(char::is_whitespace)
                .map(|(t, _)| t)
                .unwrap_or(target);
            out.push(target.trim().to_string());
        }
        i = k;
    }
    out
}

/// Lines a collector wants to render for one file: `full` = emit
/// verbatim; `ellipses` = emit a walker `…` marker (overrideable by
/// descendant batches).
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

/// Path-relative location prior: depth penalty × non-essential-dir
/// discount. Use [`file_depth_factor`] to add entrypoint pinning.
pub(crate) fn path_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    crate::value::depth_factor(ctx.depth_from_root(file)) * ctx.non_essential_factor(file)
}

/// [`path_depth_factor`] with optional entrypoint pinning — when
/// `is_entrypoint`, depth clamps to 1 so an `index.ts` at any depth
/// ranks like depth 1. Non-essential discount still applies.
pub(crate) fn file_depth_factor(file: &Path, ctx: &WalkCtx, is_entrypoint: bool) -> f64 {
    let depth = ctx.depth_from_root(file);
    let pinned_depth = if is_entrypoint { depth.min(1) } else { depth };
    crate::value::depth_factor(pinned_depth) * ctx.non_essential_factor(file)
}

/// `BatchContent::Lines` from a single file's `FileLines`. `None` if
/// the resulting span set is empty.
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

/// Parse + collect spans for per-file walker batches. Caller-supplied
/// parser closure handles language-specific parser dispatch.
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

/// `FileLines` → contiguous [`Span`] ranges. Blank source lines and
/// ellipses superseded by `Full` are dropped.
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

/// Append every 1-based row covered by `node` (trailing-newline aware).
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

/// Minimum body interior lines for per-statement splitting.
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
    // Python function blocks are indent-delimited (no brace rows).
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
        // Nothing meaningful to split — preserve the complete block.
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

/// Consecutive doc-comment siblings touching `node`. End-of-line
/// comments on a previous sibling's line are skipped (they would
/// trip non-ancestor overlap).
pub(crate) fn collect_doc_comments_above(node: Node, source: &str) -> FileLines {
    collect_doc_comments_above_bounded(node, source, None)
}

/// [`collect_doc_comments_above`] with a lower row boundary.
pub(crate) fn collect_doc_comments_above_bounded(
    node: Node,
    source: &str,
    boundary_row: Option<usize>,
) -> FileLines {
    let mut out = Vec::new();
    let mut cur = node.prev_sibling();
    let mut next_start = node.start_position().row;
    while let Some(prev) = cur {
        if prev.kind() != "comment" {
            break;
        }
        if next_start.saturating_sub(prev.end_position().row) > 1 {
            break;
        }
        if !comment_starts_at_line_start(prev, source) {
            break;
        }
        if boundary_row.is_some_and(|b| prev.start_position().row <= b) {
            break;
        }
        extend_span(&mut out, prev, source);
        next_start = prev.start_position().row;
        cur = prev.prev_sibling();
    }
    FileLines::new(dedup_sorted(out))
}

/// True iff `node` is the first non-whitespace token on its source line
/// (i.e., a standalone full-line comment rather than an end-of-line
/// trailer after some other token).
pub(crate) fn comment_starts_at_line_start(node: Node, source: &str) -> bool {
    let start = node.start_byte();
    let line_start = source[..start].rfind('\n').map_or(0, |n| n + 1);
    source[line_start..start].trim().is_empty()
}

/// Largest 0-based row `e ∈ [start_row, end_row]` such that no row
/// `r ∈ (start_row, e]` is the start of another decl (i.e., its
/// 1-based line `r + 1` is in `all_starts`). Returns `start_row` if a
/// sibling starts immediately at `start_row + 1`. Walkers use this to
/// trim span ends so emitted batches never claim a row that's another
/// decl's anchor — tree-sitter occasionally folds attribute-like
/// macros into a following function as a type qualifier, producing
/// nodes that span into the next sibling's line.
pub(crate) fn trim_end_before_next_decl(
    end_row: usize,
    start_row: usize,
    all_starts: &std::collections::HashSet<usize>,
) -> usize {
    for r in (start_row + 1)..=end_row {
        if all_starts.contains(&(r + 1)) {
            return r.saturating_sub(1).max(start_row);
        }
    }
    end_row
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

/// Walk the rows strictly between a struct/union body's `{` and `}`,
/// accumulating non-blank 1-based row numbers into groups separated by
/// blank source lines. Each returned `(group_start_line, rows)` tuple
/// describes one contiguous non-blank run in the body — what Cobra-/
/// chibicc-style aggregates use as the natural field-group split. Empty
/// when the body has no interior or no non-blank rows.
pub(crate) fn collect_blank_line_groups(body: Node, source: &str) -> Vec<(usize, Vec<usize>)> {
    let body_start = body.start_position().row;
    let body_end = body.end_position().row;
    if body_end <= body_start + 1 {
        return Vec::new();
    }
    let src_lines: Vec<&str> = source.lines().collect();
    let mut groups: Vec<(usize, Vec<usize>)> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    for row in (body_start + 1)..body_end {
        let line = src_lines.get(row).copied().unwrap_or("");
        if line.trim().is_empty() {
            if !current.is_empty() {
                let start = *current.first().unwrap();
                groups.push((start, std::mem::take(&mut current)));
            }
        } else {
            current.push(row + 1);
        }
    }
    if !current.is_empty() {
        let start = *current.first().unwrap();
        groups.push((start, current));
    }
    groups
}
