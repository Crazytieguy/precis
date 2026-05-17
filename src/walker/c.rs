//! C / C-header walker. Per-decl batches plus file-scope banner /
//! includes / decl-name surface.
//!
//! Per-file keys:
//! - `HeaderBanner { file }`: top-of-file `/* */` block (license / brief)
//! - `Includes { file }`: `#include` directives, plus any top-level
//!   `#if`/`#ifdef` blocks whose body is exclusively directive content
//!   and contains at least one `#include` (the conditional include-map
//!   idiom: `#if CFG_TUH_HID { #include "class/hid/hid_host.h" }`).
//!   Conditional blocks that wrap real code are opaque.
//! - `DeclNames { file }`: surface listing of every top-level public
//!   declaration's first line — catastrophic-omission hedge
//!
//! Per-decl keys (keyed by start line so each decl has a distinct batch):
//! - `Decl { file, start_line }`: one top-level public declaration's
//!   signature/header. For function definitions, signature with body
//!   marker. For typedefs, prototypes, structs, enums, `#define`s, the
//!   whole statement.
//! - `DeclDoc { file, start_line }`: doc comment block immediately above
//!   a decl. Predecessor: matching `Decl`.
//! - `DeclBody { file, start_line }`: body interior of a function
//!   definition. Predecessor: matching `Decl`.
//! - `AggregateMemberGroup { file, start_line, group_start_line }`:
//!   per-field-group split for big struct/union bodies (≥3 blank-line
//!   groups) and per-chunk split for big enum bodies
//!   (≥`AGGREGATE_ENUM_CHUNK_MIN` enumerators). Predecessor: matching
//!   `Decl`. When emitted, the parent `Decl` covers only the type
//!   header + closing brace so the two render disjoint body rows.
//!
//! Public-vs-private rule:
//! - `.c` files: top-level `static` items are excluded (file-private).
//! - `.h` files: `static inline` definitions are included (header-only
//!   inline accessors are part of the public API expansion); other
//!   `static` items are excluded.
//! - The wrapping `#ifndef X` / `#define X` / `#endif` header guard
//!   (recognized structurally — first `#ifndef` whose name is then
//!   `#define`d on the next line, regardless of naming convention) is
//!   descended into transparently. `extern "C" { ... }` linkage specs
//!   (including the `#ifdef __cplusplus` wrapper idiom common in C
//!   headers) are likewise descended through so the wrapped decls are
//!   visited as top-level. Other `preproc_if` / `preproc_ifdef` blocks
//!   are opaque — anything they wrap is invisible to the walker.
//!
//! Parse trees are cached in [`WalkCtx`].

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, CKey};
use crate::value::{mix_signals, names_surface_chunk_factor};

/// Chunk size for C declaration-name surfaces when there's no
/// structural signal (banner-separated sections) to chunk on. Larger
/// than the `NAMES_SURFACE_CHUNK_SIZE = 12` used by Python/TS because
/// C headers regularly expose 50+ decls and NS authors anchor on
/// unified subset rows (e.g. sds's "Public fn declarations — utility
/// fns" covers 14 specific lines). A 24-decl chunk keeps the surface
/// coherent for files in the 25–48 decl band while still splitting
/// catalog headers like krep.h (~80 decls) so the first chunk reaches
/// the budget.
const C_DECL_NAMES_CHUNK_SIZE: usize = 24;

/// Minimum number of `// <stem>.c` section banners required to switch
/// from count-based chunking to section-banner chunking on a canonical
/// entry header. Below this, the header isn't an amalgamation catalog
/// and the count-based chunks fit the surface better.
const SECTION_BANNER_MIN_COUNT: usize = 3;

/// Per-run C-walker state. Caches the seed-root's autotools
/// `include_HEADERS` declaration (parsed from `Makefile.am` once per
/// run), which lists the public-API headers an autotools project
/// installs. When present, internal headers — everything else under
/// `src/` — get demoted in early budget so the public surface ranks
/// first.
#[derive(Default)]
pub(in crate::walker) struct CState {
    /// `Some(set)` iff a `Makefile.am` exists at the seed root and
    /// declares at least one public header via `include_HEADERS` /
    /// `nobase_include_HEADERS`. The set holds canonicalized absolute
    /// paths. `None` means no information is available, in which case
    /// no public/private distinction is enforced and the walker falls
    /// back to treating every header equally.
    public_headers: OnceCell<Option<HashSet<PathBuf>>>,
    /// Memoized `is-public` lookup per header path. Mirrors the
    /// `workspace_member_lookup` pattern in `RustState`: every value
    /// signal on a header (`Decl`, `DeclDoc`, `DeclBody`, `DeclNames`
    /// chunks) reads visibility, so a single canonicalize per file
    /// rather than per call matters when a header carries 50+ decls.
    visibility_lookup: RefCell<HashMap<PathBuf, bool>>,
}

impl CState {
    /// Public-API header set for the seed root, computed once per run.
    /// Returns `None` when no `Makefile.am` with `include_HEADERS` is
    /// found — callers must treat that as "no information", not as
    /// "no headers are public".
    fn public_headers(&self, root: &Path) -> Option<&HashSet<PathBuf>> {
        self.public_headers
            .get_or_init(|| parse_include_headers(root))
            .as_ref()
    }

    /// `Some(true)` iff `header` is in the public-API set, `Some(false)`
    /// iff a public set exists but `header` isn't in it, `None` iff no
    /// public-API declaration is available. Canonicalize result is
    /// memoized per file.
    fn is_public_header(&self, header: &Path, root: &Path) -> Option<bool> {
        let public = self.public_headers(root)?;
        if let Some(&hit) = self.visibility_lookup.borrow().get(header) {
            return Some(hit);
        }
        let canonical = header.canonicalize();
        let target = canonical.as_deref().unwrap_or(header);
        let hit = public.contains(target);
        self.visibility_lookup
            .borrow_mut()
            .insert(header.to_path_buf(), hit);
        Some(hit)
    }
}

/// Parse `Makefile.am` at `root` for the `include_HEADERS` /
/// `nobase_include_HEADERS` / `pkginclude_HEADERS` declarations.
/// Returns the canonical paths of every listed `.h` file, or `None`
/// if the file is missing / unreadable / declares no public headers.
/// Line-continuation `\` is honored.
///
/// **Fails open** on any installing `*_HEADERS` line whose value
/// cannot be statically resolved to a literal list of `.h`
/// filenames — `+=` appends, variable-expanded values like
/// `$(MY_HEADERS)`, conditional `if FOO` arms with their own
/// assignments. Returning `None` in those cases is safer than
/// returning an incomplete set, since a partial public set would
/// silently demote any installed-but-missed header to internal.
fn parse_include_headers(root: &Path) -> Option<HashSet<PathBuf>> {
    let manifest = root.join("Makefile.am");
    let text = std::fs::read_to_string(&manifest).ok()?;
    let mut out = HashSet::new();
    let mut iter = text.lines();
    while let Some(line) = iter.next() {
        let trimmed = line.trim_start();
        // The three installing variables this walker understands.
        // Any other `*_HEADERS` we accept literally — `noinst_HEADERS`,
        // `EXTRA_HEADERS`, etc. are not installed into the include
        // path, so they aren't part of the public-API surface and we
        // ignore them. Only the three below add to the public set.
        let after_name = [
            "include_HEADERS",
            "nobase_include_HEADERS",
            "pkginclude_HEADERS",
        ]
        .iter()
        .find_map(|name| trimmed.strip_prefix(name));
        let Some(after_name) = after_name else {
            continue;
        };
        let after_name = after_name.trim_start();
        // Fail open on `+=` appends, conditional-assignment operators
        // (`?=`, `:=`), or any operator other than plain `=`. A `+=`
        // adds entries that this parser would silently miss, leaving
        // an incomplete public set; better to drop the signal entirely
        // than to demote a real public header.
        let mut rest = after_name.strip_prefix('=')?;
        // Collect the assignment value, honoring `\` line continuations.
        let mut value = String::new();
        loop {
            let (head, continued) = match rest.strip_suffix('\\') {
                Some(head) => (head, true),
                None => (rest, false),
            };
            value.push_str(head);
            if !continued {
                break;
            }
            value.push(' ');
            let Some(next) = iter.next() else { break };
            rest = next;
        }
        for tok in value.split_whitespace() {
            // Strip automake variable references like `$(srcdir)/foo.h`
            // — we only need the path component for canonicalization.
            let tok = tok.trim_start_matches("$(srcdir)/");
            // A `$(...)` token (variable expansion to an opaque value)
            // means the value list isn't statically known. Fail open.
            if tok.contains("$(") || tok.contains("${") {
                return None;
            }
            if !tok.ends_with(".h") {
                continue;
            }
            let resolved = root.join(tok);
            let canonical = resolved.canonicalize().unwrap_or(resolved);
            out.insert(canonical);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

use super::{
    FileLines, WalkCtx, build_per_file_content, collect_blank_line_groups,
    collect_doc_comments_above_bounded, dedup_sorted, extend_span, file_depth_factor,
    file_lines_covered_by, node_end_row_trimmed, push_rows, signature_end_row,
    single_file_lines_content, trim_end_before_next_decl,
};

/// Partition the in-source-order `decls` of a single C source file into
/// chunks for the decl-names surface, along with the chunking
/// strategy. Two strategies:
///
/// 1. **Section-banner chunking** (preferred for amalgamation-style
///    headers): when `file` is a header at a canonical entry location
///    and contains at least `SECTION_BANNER_MIN_COUNT` top-level
///    `// <stem>.c` banner comments, chunk on banner boundaries. Each
///    chunk's decls are the run of decls whose `start_line` falls
///    between consecutive banners; any decls before the first banner
///    form the leading "preamble" chunk. Chunks with zero decls are
///    skipped — banners with no following decls (a banner immediately
///    before the next banner) don't produce empty surfaces.
/// 2. **Count-based fallback**: split into `C_DECL_NAMES_CHUNK_SIZE`-
///    sized chunks. This is what every C header without amalgamation
///    structure uses (the typical case).
fn compute_decl_chunk_ranges(
    decls: &[(Node, DeclInfo)],
    tree: &Tree,
    source: &str,
    file: &Path,
    ctx: &WalkCtx,
) -> (Vec<Range<usize>>, DeclChunkingStrategy) {
    if let Some(ranges) = section_banner_chunk_ranges(decls, tree, source, file, ctx) {
        return (ranges, DeclChunkingStrategy::SectionBanner);
    }
    (
        count_based_chunk_ranges(decls.len()),
        DeclChunkingStrategy::CountBased,
    )
}

/// Which chunking strategy produced the ranges. The decl-names value
/// model treats section-banner chunks differently from count-based
/// chunks: a banner-delimited chunk is its own subject (the strings.c
/// section of an amalgamation header), so the source-order falloff
/// that count-based chunking carries (later chunks are less load-bearing
/// summaries of the same catalog) shouldn't apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclChunkingStrategy {
    /// Fixed-size source-order chunks; later chunks decay in value.
    CountBased,
    /// Chunks delimited by `// <stem>.c` banner comments; each chunk
    /// stands alone, so no source-order decay.
    SectionBanner,
}

/// Section-banner chunking — see [`compute_decl_chunk_ranges`].
/// Returns `None` when the file doesn't qualify (not a header at a
/// canonical entry location, or fewer than `SECTION_BANNER_MIN_COUNT`
/// banner comments).
fn section_banner_chunk_ranges(
    decls: &[(Node, DeclInfo)],
    tree: &Tree,
    source: &str,
    file: &Path,
    ctx: &WalkCtx,
) -> Option<Vec<Range<usize>>> {
    if !is_header_file(file) || !is_at_canonical_entry_location(file, ctx) {
        return None;
    }
    let banner_lines = find_module_section_banner_lines(tree, source);
    if banner_lines.len() < SECTION_BANNER_MIN_COUNT {
        return None;
    }
    // Partition decl indices by their start_line vs banner lines. A
    // decl at line >= banner_lines[k] and < banner_lines[k+1] belongs
    // to chunk k+1 (chunk 0 is the preamble); after the last banner,
    // decls belong to the final chunk.
    let mut ranges: Vec<Range<usize>> = Vec::new();
    let mut start = 0usize;
    for &boundary in &banner_lines {
        // Find the first decl whose start_line >= boundary.
        let end = decls
            .iter()
            .position(|(_, d)| d.start_line >= boundary)
            .unwrap_or(decls.len());
        if end > start {
            ranges.push(start..end);
        }
        start = end;
    }
    if start < decls.len() {
        ranges.push(start..decls.len());
    }
    // Without at least two chunks the section split doesn't add
    // anything over the unchunked surface; fall back so values stay
    // calibrated against count-based chunking.
    if ranges.len() < 2 {
        return None;
    }
    Some(ranges)
}

/// Count-based chunking — see [`compute_decl_chunk_ranges`]. A file
/// with `≤ C_DECL_NAMES_CHUNK_SIZE` decls yields one chunk; otherwise
/// chunks of exactly `C_DECL_NAMES_CHUNK_SIZE` decls (the final chunk
/// may be smaller).
fn count_based_chunk_ranges(decl_count: usize) -> Vec<Range<usize>> {
    if decl_count == 0 {
        return Vec::new();
    }
    let chunk_size = C_DECL_NAMES_CHUNK_SIZE;
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < decl_count {
        let end = (i + chunk_size).min(decl_count);
        ranges.push(i..end);
        i = end;
    }
    ranges
}

/// 1-based line numbers of top-level `// <stem>.c` (or `/* <stem>.c */`)
/// banner comments — the amalgamation-style section dividers chibicc.h
/// uses to partition prototypes by their backing translation unit. Only
/// `comment` children of the translation_unit root are scanned;
/// comments nested inside decls or preproc blocks don't count, since
/// the chunker partitions top-level decls.
fn find_module_section_banner_lines(tree: &Tree, source: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut cursor = tree.root_node().walk();
    for child in tree.root_node().children(&mut cursor) {
        if child.kind() != "comment" {
            continue;
        }
        let text = &source[child.start_byte()..child.end_byte()];
        if is_module_section_banner_comment(text) {
            out.push(child.start_position().row + 1);
        }
    }
    out
}

/// True iff `text` (a comment node's source slice) is a module-section
/// banner — `// <stem>.c` or `/* <stem>.c */` after trimming, where
/// `<stem>` is an identifier-shaped C identifier (letters, digits,
/// underscores). Tolerates surrounding whitespace and the chibicc
/// triple-slash idiom (`// \n// foo.c \n//`) which tree-sitter-c
/// surfaces as three sibling `comment` nodes — the middle one is the
/// banner.
fn is_module_section_banner_comment(text: &str) -> bool {
    let body = if let Some(rest) = text.strip_prefix("//") {
        rest
    } else if let Some(rest) = text.strip_prefix("/*").and_then(|s| s.strip_suffix("*/")) {
        rest
    } else {
        return false;
    };
    let trimmed = body.trim();
    let Some(stem) = trimmed.strip_suffix(".c") else {
        return false;
    };
    !stem.is_empty()
        && stem
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let c_files = c_source_files(dir);
    if c_files.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for file in &c_files {
        if let Some(content) = build_per_file_content(file, ctx, parse_c, collect_header_banner) {
            out.push(Batch {
                key: CKey::HeaderBanner { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: header_banner_value(file, ctx),
            });
        }

        let Some((source, tree)) = parse_c(ctx, file) else {
            continue;
        };

        let (includes_lines, include_count) = collect_includes(&tree, &source);
        if let Some(content) = single_file_lines_content(file, &source, includes_lines) {
            out.push(Batch {
                key: CKey::Includes { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: includes_value(file, ctx, include_count),
            });
        }

        let decls = find_decls(&tree, &source, file);
        if decls.is_empty() {
            continue;
        }
        // Last 0-based row claimed by the file's HeaderBanner (the
        // leading run of comment children of root). Used as a stop
        // boundary so the first decl's `DeclDoc` doesn't walk into
        // banner territory when there's no blank line between them.
        let banner_end_row = header_banner_end_row(&tree);
        // C catalog files (`sds.h`, headers exposing the whole public
        // surface) often anchor NS rows on the *unified* declaration
        // listing — splitting at the default 12 fragments rows like
        // "Public fn declarations — utility fns" across chunks. Chunk
        // only when the surface is large enough that the unified batch
        // would lose the value/cost race against per-decl batches.
        let (chunk_ranges, chunk_strategy) =
            compute_decl_chunk_ranges(&decls, &tree, &source, file, ctx);
        let names_chunk_count = chunk_ranges.len();
        let names_predecessors: Vec<_> = (0..names_chunk_count)
            .map(|chunk_index| {
                BatchKey::C(CKey::DeclNames {
                    file: file.clone(),
                    chunk_index,
                })
            })
            .collect();
        let all_starts: std::collections::HashSet<usize> =
            decls.iter().map(|(_, i)| i.start_line).collect();
        let src_lines: Vec<&str> = source.lines().collect();
        let names_lines_by_chunk: Vec<FileLines> = chunk_ranges
            .iter()
            .map(|range| {
                collect_decl_names_from_with_global_starts(
                    &decls[range.clone()],
                    &all_starts,
                    &src_lines,
                )
            })
            .collect();
        // Per-decl index → chunk index lookup. Built from the chunk
        // ranges so callers don't re-divide.
        let decl_to_chunk: Vec<usize> = {
            let mut v = vec![0_usize; decls.len()];
            for (chunk_index, range) in chunk_ranges.iter().enumerate() {
                for i in range.clone() {
                    v[i] = chunk_index;
                }
            }
            v
        };
        for (chunk_index, names_lines) in names_lines_by_chunk.iter().enumerate() {
            let Some(content) = single_file_lines_content(file, &source, names_lines.clone())
            else {
                continue;
            };
            out.push(Batch {
                key: names_predecessors[chunk_index].clone(),
                predecessor: None,
                content,
                value: decl_names_value(file, ctx, chunk_index, names_chunk_count, chunk_strategy),
            });
        }
        for (decl_index, (node, info)) in decls.iter().enumerate() {
            let names_chunk_index = decl_to_chunk[decl_index];
            let names_predecessor = names_predecessors[names_chunk_index].clone();
            let chunk_names_lines = &names_lines_by_chunk[names_chunk_index];
            let decl_key = CKey::Decl {
                file: file.clone(),
                start_line: info.start_line,
            };
            let decl_lines = collect_decl(*node, info, &source, &all_starts);
            let doc_lines = collect_doc_comments_above_bounded(*node, &source, banner_end_row);
            let init_tables = if info.kind == DeclKind::FunctionDef {
                find_init_tables_in_body(*node)
            } else {
                Vec::new()
            };
            let excluded_ranges: Vec<(usize, usize)> = init_tables
                .iter()
                .map(|t| {
                    (
                        t.decl_node.start_position().row + 1,
                        t.decl_node.end_position().row + 1,
                    )
                })
                .collect();
            let body_lines = if info.has_body {
                collect_decl_body(*node, &src_lines, &excluded_ranges)
            } else {
                FileLines::new(Vec::new())
            };
            let decl_has_descendants = !doc_lines.full.is_empty() || !body_lines.full.is_empty();
            if (!file_lines_covered_by(&decl_lines, chunk_names_lines) || decl_has_descendants)
                && let Some(content) = single_file_lines_content(file, &source, decl_lines)
            {
                out.push(Batch {
                    key: decl_key.clone().into(),
                    predecessor: Some(names_predecessor.clone()),
                    content,
                    value: decl_value(file, info.kind, ctx),
                });
            }
            let decl_predecessor = BatchKey::C(decl_key);
            if let Some(content) = single_file_lines_content(file, &source, doc_lines) {
                out.push(Batch {
                    key: CKey::DeclDoc {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: decl_doc_value(file, info.kind, ctx),
                });
            }
            if info.has_body
                && let Some(content) = single_file_lines_content(file, &source, body_lines)
            {
                out.push(Batch {
                    key: CKey::DeclBody {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: decl_body_value(file, info.kind, ctx),
                });
            }
            for table in &init_tables {
                let start_line = table.decl_node.start_position().row + 1;
                let end_line = table.decl_node.end_position().row + 1;
                let rows = collect_init_table_rows(table);
                if let Some(content) = single_file_lines_content(file, &source, rows) {
                    out.push(Batch {
                        key: CKey::InitTableRows {
                            file: file.clone(),
                            start_line,
                            end_line,
                        }
                        .into(),
                        // Top-level: tying to the enclosing Decl would block
                        // emission when the function lives deep in a large
                        // file (sqlite-vec.c:9751). DeclBody excludes these
                        // rows already — no sibling overlap.
                        predecessor: None,
                        content,
                        value: init_table_value(file, ctx),
                    });
                }
            }
            // Per-group / per-chunk batches for big aggregate bodies.
            // Sibling of `DeclDoc` under the same `Decl` predecessor;
            // the parent `Decl`'s span was trimmed in `collect_decl` to
            // the type header + closer so the group rows don't overlap.
            for group in &info.member_groups {
                let lines = FileLines::new(group.rows.clone());
                if let Some(content) = single_file_lines_content(file, &source, lines) {
                    out.push(Batch {
                        key: CKey::AggregateMemberGroup {
                            file: file.clone(),
                            start_line: info.start_line,
                            group_start_line: group.group_start_line,
                        }
                        .into(),
                        predecessor: Some(decl_predecessor.clone()),
                        content,
                        value: aggregate_member_group_value(file, info.kind, ctx),
                    });
                }
            }
        }
    }

    out
}

// --- decl classification ------------------------------------------------

/// Coarse decl kinds for value weighting. Matches the structural anchors
/// the C NSes consistently call out (typedefs, structs, function
/// prototypes / definitions, public macros).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclKind {
    /// `typedef …` — names a new type. Often the central data shape.
    Typedef,
    /// `struct foo { … };` / `union foo { … };` / `enum { … };` at top
    /// level (a `declaration` whose type_specifier is one of these).
    Aggregate,
    /// `int foo(...) { ... }` — function definition with body.
    FunctionDef,
    /// `int foo(...);` — function declaration / prototype.
    FunctionDecl,
    /// `extern T foo;` / `T foo;` — variable declaration.
    Variable,
    /// `#define X val` — object-like macro.
    Macro,
    /// `#define X(args) body` — function-like macro.
    MacroFn,
}

impl DeclKind {
    fn kind_weight(self) -> f64 {
        match self {
            DeclKind::Typedef => 1.10,
            DeclKind::Aggregate => 1.10,
            DeclKind::FunctionDef => 1.00,
            DeclKind::FunctionDecl => 0.95,
            DeclKind::MacroFn => 0.95,
            DeclKind::Macro => 0.90,
            DeclKind::Variable => 0.80,
        }
    }
}

#[derive(Debug, Clone)]
struct DeclInfo {
    start_line: usize,
    kind: DeclKind,
    /// True for `function_definition` only — gates `DeclBody` emission.
    has_body: bool,
    /// Per-group/per-chunk member-batches for big aggregates. Empty when
    /// the decl isn't a chunk-eligible struct/union/enum. When non-empty,
    /// the parent `Decl` is trimmed to the type header + closing brace so
    /// each group renders disjoint body rows.
    member_groups: Vec<AggregateMemberGroup>,
}

/// One chunk of a big aggregate body. Used for both blank-line-separated
/// struct/union field groups and sized enum-body chunks.
#[derive(Debug, Clone)]
struct AggregateMemberGroup {
    /// 1-based start line of the first row in the group.
    group_start_line: usize,
    /// All 1-based row numbers covered by this group.
    rows: Vec<usize>,
}

/// All public top-level decls in `file`'s tree, in source order, paired
/// with the AST node so per-decl collectors don't have to re-walk.
/// Descends transparently through a single wrapping `#ifndef X` /
/// `#define X` / `#endif` header guard and through `extern "C" { ... }`
/// linkage specs (raw or wrapped in `#ifdef __cplusplus`).
fn find_decls<'a>(tree: &'a Tree, source: &str, file: &Path) -> Vec<(Node<'a>, DeclInfo)> {
    let in_header = is_header_file(file);
    let mut out = Vec::new();
    walk_top_level(tree.root_node(), source, &mut |node| {
        if let Some(info) = classify_decl(node, source, in_header) {
            out.push((node, info));
        }
    });
    out.sort_by_key(|(_, d)| d.start_line);
    out.dedup_by_key(|(_, d)| d.start_line);
    out
}

/// Visit every "effective top-level" item — translation_unit children
/// minus envelopes that wrap real decls. Descends transparently
/// through:
/// - the file's `#ifndef X / #define X` header guard (a single wrapping
///   `preproc_ifdef`)
/// - `extern "C" { ... }` linkage specs, including the `#ifdef __cplusplus`
///   wrapper that C headers use to make the spec C++-only. tree-sitter-c
///   parses `extern "C" {` and its matching `}` into a single
///   `linkage_specification` node even when each brace lives in its own
///   `#ifdef __cplusplus` block, so the actual decls hang off the
///   `linkage_specification`'s `declaration_list`.
fn walk_top_level<'a, F: FnMut(Node<'a>)>(root: Node<'a>, source: &str, visit: &mut F) {
    let header_guard_body = header_guard_body_node(root, source);
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if Some(child) == header_guard_body {
            descend_envelopes(child, source, visit);
        } else {
            visit_with_envelope_descent(child, source, visit);
        }
    }
}

/// Visit each child of `node`, applying the same envelope-descent rule
/// the top-level walk uses. Used for nodes that are themselves a
/// descent boundary (header guard, `extern "C"` linkage_specification).
fn descend_envelopes<'a, F: FnMut(Node<'a>)>(node: Node<'a>, source: &str, visit: &mut F) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_with_envelope_descent(child, source, visit);
    }
}

/// Either visit `node` as top-level, or — if it's an `extern "C" { ... }`
/// envelope (raw or `#ifdef __cplusplus`-wrapped) — descend into the
/// decls it contains.
fn visit_with_envelope_descent<'a, F: FnMut(Node<'a>)>(
    node: Node<'a>,
    source: &str,
    visit: &mut F,
) {
    if let Some(decl_list) = extern_c_declaration_list(node, source) {
        descend_envelopes(decl_list, source, visit);
        return;
    }
    visit(node);
}

/// If `node` is an `extern "C" { ... }` envelope, return the inner
/// `declaration_list` whose children are the wrapped decls. Recognizes
/// both a bare `linkage_specification` and the
/// `#ifdef __cplusplus / extern "C" { / #endif` wrapper idiom common in
/// C headers. The `#ifdef` envelope is accepted only when the guard
/// symbol is `__cplusplus` and the body's sole non-token child is the
/// `linkage_specification` — a feature-gate (`#ifdef FEATURE_X`) around
/// a linkage spec must stay opaque to avoid advertising
/// platform/feature-gated decls as unconditional public API.
fn extern_c_declaration_list<'a>(node: Node<'a>, source: &str) -> Option<Node<'a>> {
    let linkage = match node.kind() {
        "linkage_specification" => node,
        "preproc_ifdef" => cplusplus_wrapped_linkage_specification(node, source)?,
        _ => return None,
    };
    let mut cursor = linkage.walk();
    linkage
        .children(&mut cursor)
        .find(|c| c.kind() == "declaration_list")
}

/// If `ifdef` is shaped like `#ifdef __cplusplus / linkage_specification /
/// #endif` (with the standard `#ifdef` / identifier / body / `#endif`
/// children tree-sitter-c surfaces), return the `linkage_specification`
/// child. Otherwise `None` — including when the guard symbol isn't
/// `__cplusplus` (a feature gate), the directive is `#ifndef` (which
/// inverts the gate), or the body contains anything besides a single
/// `linkage_specification`.
fn cplusplus_wrapped_linkage_specification<'a>(ifdef: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cursor = ifdef.walk();
    let mut children = ifdef.children(&mut cursor);
    if children.next()?.kind() != "#ifdef" {
        return None;
    }
    let name_node = children.next()?;
    if name_node.kind() != "identifier"
        || &source[name_node.start_byte()..name_node.end_byte()] != "__cplusplus"
    {
        return None;
    }
    let mut found: Option<Node<'a>> = None;
    for child in children {
        match child.kind() {
            "#endif" | "\n" | "comment" => {}
            "linkage_specification" if found.is_none() => found = Some(child),
            _ => return None,
        }
    }
    found
}

/// The `preproc_ifdef` node that wraps the file body as a header guard,
/// if any. Recognized by structure — `#ifndef X` whose body's first
/// child is `#define X` — not by naming convention. Returns the
/// `preproc_ifdef` node itself; its direct children are the lines we
/// want to treat as top-level.
fn header_guard_body_node<'a>(root: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cursor = root.walk();
    let mut candidate: Option<Node<'a>> = None;
    let mut other_top_level = 0;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "comment" => continue,
            "preproc_ifdef" if candidate.is_none() && other_top_level == 0 => {
                if is_header_guard(child, source) {
                    candidate = Some(child);
                } else {
                    other_top_level += 1;
                }
            }
            _ => other_top_level += 1,
        }
    }
    if other_top_level == 0 {
        candidate
    } else {
        None
    }
}

/// True iff `node` (a `preproc_ifdef`) is shaped like
/// `#ifndef X` / `#define X` / … / `#endif`. Detection is by source
/// text: tree-sitter-c surfaces the opening `#ifndef` / `#define` as
/// raw tokens rather than as a structured field, and the simplest way
/// to verify the name match is to scan the first non-blank, non-comment
/// directive line after the `#ifndef`.
fn is_header_guard(ifdef: Node, source: &str) -> bool {
    let text = &source[ifdef.start_byte()..ifdef.end_byte()];
    let mut lines = text.lines();
    let first = lines.next().unwrap_or("");
    let Some(name) = first
        .trim_start()
        .strip_prefix("#")
        .map(|s| s.trim_start())
        .and_then(|s| s.strip_prefix("ifndef"))
        .map(|s| s.split_whitespace().next().unwrap_or(""))
        .filter(|s| !s.is_empty())
    else {
        return false;
    };
    // The `#define X` follows on a subsequent line (sometimes after
    // blank lines / comments). Look ahead a small fixed window — header
    // guards in real fixtures put the `#define` immediately after.
    for line in lines.take(8) {
        let t = line.trim_start();
        if t.is_empty() || t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        let Some(rest) = t.strip_prefix("#").map(str::trim_start) else {
            return false;
        };
        let Some(after) = rest.strip_prefix("define") else {
            return false;
        };
        return after.split_whitespace().next() == Some(name);
    }
    false
}

/// Identify a top-level node as a public decl, returning its `DeclInfo`.
/// Returns `None` for nodes we don't surface (preproc_include, comments,
/// `static` items in `.c` files, etc.).
fn classify_decl(node: Node, source: &str, in_header: bool) -> Option<DeclInfo> {
    let start_line = node.start_position().row + 1;
    let (kind, has_body) = match node.kind() {
        "function_definition" => {
            let static_ = has_static_specifier(node, source);
            let inline = has_inline_specifier(node, source);
            // .c file: static excluded. .h file: static excluded unless
            // also inline (header-only inline accessor).
            if static_ && !(in_header && inline) {
                return None;
            }
            (DeclKind::FunctionDef, true)
        }
        "declaration" => {
            if has_static_specifier(node, source) {
                return None;
            }
            let kind = if has_struct_union_or_enum(node) {
                DeclKind::Aggregate
            } else if has_function_declarator(node) {
                DeclKind::FunctionDecl
            } else {
                DeclKind::Variable
            };
            (kind, false)
        }
        "type_definition" => (DeclKind::Typedef, false),
        // Tree-sitter-c parses a top-level bare `struct foo { … };`
        // (no declarator) as a `struct_specifier` followed by a `;`
        // token rather than wrapping them in a `declaration`. Surface
        // the specifier itself.
        "struct_specifier" | "union_specifier" | "enum_specifier" => (DeclKind::Aggregate, false),
        "preproc_def" => {
            // Skip the header-guard's own `#define X` — it's part of
            // the guard envelope, not a public macro.
            if is_header_guard_define(node, source) {
                return None;
            }
            (DeclKind::Macro, false)
        }
        "preproc_function_def" => (DeclKind::MacroFn, false),
        _ => return None,
    };
    let member_groups = if matches!(kind, DeclKind::Aggregate | DeclKind::Typedef) {
        find_aggregate_body(node)
            .map(|body| collect_aggregate_member_groups(body, source))
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    Some(DeclInfo {
        start_line,
        kind,
        has_body,
        member_groups,
    })
}

/// Minimum source-line span (closing brace minus opening brace) for a
/// struct/union body to be eligible for blank-line field-group chunking.
/// Below this, the unchunked aggregate fits cheaply at any reasonable
/// budget and splitting adds scheduling churn without payoff.
const AGGREGATE_STRUCT_MIN_LINES: usize = 30;

/// Minimum number of blank-line-separated field groups required for
/// struct/union chunking. A two-group struct's groups are roughly half
/// the struct each — the parent Decl is already cheap to schedule.
const AGGREGATE_STRUCT_MIN_GROUPS: usize = 3;

/// Minimum number of enumerators required for enum chunking. Below
/// this the whole enum is small enough that splitting adds scheduling
/// churn for no benefit (a 16-row enum already fits in a 3K budget as
/// a single `Decl`). chibicc.h's `NodeKind` (49 enumerators) and
/// `TypeKind` (16) anchor this threshold: NodeKind is the canonical
/// chunk target; TypeKind stays whole.
const AGGREGATE_ENUM_CHUNK_MIN: usize = 32;

/// Chunk size for big enum bodies. Roughly matches the inner-row
/// granularity of NS atoms — a typical NS author rows like "ND_ADD
/// through ND_SHR (arithmetic / bit ops)" cover 10–14 enumerators.
const AGGREGATE_ENUM_CHUNK_SIZE: usize = 12;

/// Locate the body node of a top-level aggregate decl. Handles:
/// - bare `struct_specifier` / `union_specifier` / `enum_specifier`,
/// - `declaration` whose type_specifier is one of the above,
/// - `type_definition` (`typedef struct { … } X;`).
///
/// Returns the inner `field_declaration_list` (struct/union) or
/// `enumerator_list` (enum); `None` for forward declarations and
/// any decl shape without an inner body.
fn find_aggregate_body(node: Node) -> Option<Node> {
    fn spec_body(spec: Node) -> Option<Node> {
        match spec.kind() {
            "struct_specifier" | "union_specifier" => {
                let body = spec.child_by_field_name("body")?;
                (body.kind() == "field_declaration_list").then_some(body)
            }
            "enum_specifier" => {
                let body = spec.child_by_field_name("body")?;
                (body.kind() == "enumerator_list").then_some(body)
            }
            _ => None,
        }
    }
    match node.kind() {
        "struct_specifier" | "union_specifier" | "enum_specifier" => spec_body(node),
        "declaration" | "type_definition" => {
            let mut cursor = node.walk();
            node.children(&mut cursor).find_map(spec_body)
        }
        _ => None,
    }
}

/// Decompose an aggregate body into chunks. Struct/union bodies split
/// on blank lines (each non-empty run is a group); enum bodies split
/// into fixed-size chunks. Returns an empty vec when the body is too
/// small to be worth chunking.
fn collect_aggregate_member_groups(body: Node, source: &str) -> Vec<AggregateMemberGroup> {
    let body_start = body.start_position().row;
    let body_end = body.end_position().row;
    if body_end <= body_start + 1 {
        return Vec::new();
    }
    match body.kind() {
        "field_declaration_list" => {
            if body_end.saturating_sub(body_start) + 1 < AGGREGATE_STRUCT_MIN_LINES {
                return Vec::new();
            }
            let groups = collect_struct_blank_line_groups(body, source);
            if groups.len() < AGGREGATE_STRUCT_MIN_GROUPS {
                return Vec::new();
            }
            groups
        }
        "enumerator_list" => collect_enum_chunks(body),
        _ => Vec::new(),
    }
}

/// Blank-line-separated field groups for a struct/union body, as
/// `AggregateMemberGroup`s. Wraps the shared
/// [`collect_blank_line_groups`] helper.
fn collect_struct_blank_line_groups(body: Node, source: &str) -> Vec<AggregateMemberGroup> {
    collect_blank_line_groups(body, source)
        .into_iter()
        .map(|(group_start_line, rows)| AggregateMemberGroup {
            group_start_line,
            rows,
        })
        .collect()
}

/// Split an enum body into fixed-size enumerator chunks. Each chunk's
/// `rows` covers every 1-based source row the chunked enumerators
/// occupy, including continuation rows of multi-line enumerators
/// (`FOO = (1 << 20)\n  | (1 << 21),`). Returns an empty vec when the
/// enum has fewer than `AGGREGATE_ENUM_CHUNK_MIN` enumerators (small
/// enums stay as a single `Decl`).
fn collect_enum_chunks(body: Node) -> Vec<AggregateMemberGroup> {
    let mut cursor = body.walk();
    let enumerator_spans: Vec<(usize, usize)> = body
        .named_children(&mut cursor)
        .filter(|c| c.kind() == "enumerator")
        .map(|c| (c.start_position().row, c.end_position().row))
        .collect();
    if enumerator_spans.len() < AGGREGATE_ENUM_CHUNK_MIN {
        return Vec::new();
    }
    enumerator_spans
        .chunks(AGGREGATE_ENUM_CHUNK_SIZE)
        .map(|chunk| {
            let mut rows = Vec::new();
            for (start, end) in chunk {
                push_rows(&mut rows, *start, *end);
            }
            AggregateMemberGroup {
                group_start_line: chunk[0].0 + 1,
                rows,
            }
        })
        .collect()
}

/// Is this `#define X` the back-half of a `#ifndef X` / `#define X`
/// header guard? Detection is conservative: the name must be uppercase
/// (`A-Z`, `0-9`, `_`) and the `#define` has no value.
fn is_header_guard_define(node: Node, source: &str) -> bool {
    let Some(name_node) = node.child_by_field_name("name") else {
        return false;
    };
    let name = &source[name_node.start_byte()..name_node.end_byte()];
    if name.is_empty() {
        return false;
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return false;
    }
    node.child_by_field_name("value").is_none()
}

fn has_static_specifier(node: Node, source: &str) -> bool {
    has_storage_class(node, source, "static")
}

fn has_inline_specifier(node: Node, source: &str) -> bool {
    has_storage_class(node, source, "inline")
}

fn has_storage_class(node: Node, source: &str, keyword: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "storage_class_specifier" {
            let text = source[child.start_byte()..child.end_byte()].trim();
            if text == keyword {
                return true;
            }
        }
    }
    false
}

fn has_struct_union_or_enum(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|c| {
        matches!(
            c.kind(),
            "struct_specifier" | "union_specifier" | "enum_specifier"
        )
    })
}

/// True when the `declaration`'s declarator is (recursively, through
/// pointers / parens) a `function_declarator` — i.e. this `declaration`
/// is a function prototype. A declarator without a `function_declarator`
/// somewhere underneath is a variable.
fn has_function_declarator(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(declarator_is_function)
}

fn declarator_is_function(node: Node) -> bool {
    match node.kind() {
        "function_declarator" => true,
        "pointer_declarator" | "parenthesized_declarator" | "init_declarator" => node
            .child_by_field_name("declarator")
            .is_some_and(declarator_is_function),
        _ => false,
    }
}

// --- value functions ----------------------------------------------------

/// C source files this walker owns: `.c`, `.h`, and `.h.tmpl` (a
/// template that compiles down to a public header at release time —
/// sqlite-vec's `sqlite-vec.h.tmpl` is the canonical example, with the
/// VERSION/DATE/SOURCE placeholders substituted by the build). Treated
/// as headers structurally: the same `#ifdef` / `#define` / extern-C
/// shape, and NSes anchor on them with the same value profile.
fn c_source_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = read_dir
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            if !path.is_file() {
                return None;
            }
            let name = path.file_name().and_then(|n| n.to_str())?;
            is_c_source_file_name(name).then_some(path)
        })
        .collect();
    out.sort();
    out
}

/// True for `.c`, `.h`, and `.h.tmpl` filenames (case-insensitive on
/// the extension; the literal `.tmpl` suffix must follow `.h`).
fn is_c_source_file_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".c") || lower.ends_with(".h") || lower.ends_with(".h.tmpl")
}

fn is_header_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".h") || lower.ends_with(".h.tmpl")
}

/// Multiplier applied to header-file batches that an autotools
/// `Makefile.am` *explicitly* marks as internal (a public set exists,
/// but the header isn't in it). Public headers and headers in projects
/// without an `include_HEADERS` declaration are unaffected. Gentle
/// enough to keep internal headers schedulable at deeper budgets while
/// letting the public surface win the early-budget race against the
/// dozen-odd implementation headers an autotools library typically
/// ships alongside its `.c` files.
const INTERNAL_HEADER_FACTOR: f64 = 0.4;

/// `INTERNAL_HEADER_FACTOR` when `file` is a `.h` known-internal under
/// the seed root's autotools manifest, `1.0` otherwise (including `.c`
/// files, projects without a public-header manifest, and the public
/// headers themselves).
fn explicit_visibility_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if !is_header_file(file) {
        return 1.0;
    }
    match ctx.c_state().is_public_header(file, ctx.root()) {
        Some(false) => INTERNAL_HEADER_FACTOR,
        Some(true) | None => 1.0,
    }
}

/// Catastrophic-axis multiplier for header (boost) vs `.c` (demote).
/// Headers carry the public API; `.c` content is implementation detail
/// that an agent reading a C library usually wants in less depth.
/// Calibrated against the krep / sds / bareiron divergence reports —
/// the ratio is wide because without it the walker spends most of its
/// budget on per-function bodies in `.c` files and displaces README
/// content that the NSes consistently rank as tier 1.
fn header_cat_factor(file: &Path) -> f64 {
    if is_header_file(file) { 1.15 } else { 0.55 }
}

/// Follow-up axis multiplier. Headers don't get a boost on follow-up
/// (their value is "what's the API", not "what does it do") — the boost
/// is catastrophic-axis-only — but `.c` content is still demoted, so an
/// agent that already saw the names surface doesn't burn budget on
/// implementation details.
fn body_fu_factor(file: &Path) -> f64 {
    if is_header_file(file) { 1.0 } else { 0.55 }
}

fn c_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(file, ctx, is_header_file(file))
        * secondary_root_pair_factor(file, ctx)
        * stdlib_shim_factor(file, ctx)
        * explicit_visibility_factor(file, ctx)
}

/// Damp depth-1 `.c/.h` files whose stem doesn't match the repo's
/// basename when a stem-matching primary pair exists. Mirrors the Rust
/// walker's `secondary_workspace_member_factor`: in a flat C project
/// with `<repo>.c` + `<repo>.h` plus a sibling vendored algorithm
/// (krep `aho_corasick.*`, single-file libraries pasted next to the
/// project's own header), the primary pair is the orientation
/// surface NS authors anchor on; the secondary stem's deep decl
/// catalog is reference content for the deeper budget.
const SECONDARY_ROOT_PAIR_FACTOR: f64 = 0.5;

fn secondary_root_pair_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if ctx.depth_from_root(file) != 1 {
        return 1.0;
    }
    let Some(stem) = c_source_stem(file) else {
        return 1.0;
    };
    let Some(repo) = ctx.root().file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    if stem.eq_ignore_ascii_case(repo) {
        return 1.0;
    }
    // Only damp when a stem-matching primary file actually exists at
    // the root — otherwise this is a single-pair flat project (sds
    // structure) where every depth-1 file is part of the project's
    // own surface.
    let primary_present = ["c", "h", "h.tmpl"]
        .iter()
        .any(|ext| ctx.root().join(format!("{repo}.{ext}")).is_file());
    if primary_present {
        SECONDARY_ROOT_PAIR_FACTOR
    } else {
        1.0
    }
}

/// Project-name stem of a C source file, treating `.h.tmpl` as a
/// header variant whose stem is everything before `.h.tmpl`
/// (`sqlite-vec.h.tmpl` → `sqlite-vec`). Plain `.c` / `.h` files use
/// `file_stem()`.
fn c_source_stem(file: &Path) -> Option<&str> {
    let name = file.file_name().and_then(|n| n.to_str())?;
    if name.to_ascii_lowercase().ends_with(".h.tmpl") {
        return Some(&name[..name.len() - ".h.tmpl".len()]);
    }
    file.file_stem().and_then(|s| s.to_str())
}

fn header_banner_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Headers' banner is often the canonical "what is this header"
    // signal; .c file banners are usually license boilerplate.
    let cat = if is_header_file(file) { 0.55 } else { 0.05 };
    mix_signals(cat, 0.4, 0.7, c_depth_factor(file, ctx))
}

/// Per-conditional-`#include` value increment for an umbrella header's
/// feature-gated include map. The Includes batch grows roughly
/// linearly in cost with directive count when it captures a class /
/// platform include map (tinyusb's `tusb.h`: each `#if CFG_TUH_*`
/// arm is ~3 lines around one `#include`), so value must scale
/// similarly to stay competitive under the scheduler's
/// `value / cost^0.35` ranking — otherwise the bigger batch slips
/// past 3K and the NS-anchored "what classes/modules ship" map never
/// lands in early budget. Gated on conditional content so plain
/// stdlib include lists (chibicc.h-style 18 stdlib lines) don't get
/// promoted ahead of higher-value surfaces.
const HUB_INCLUDE_VALUE_PER_DIRECTIVE: f64 = 80.0;

/// True iff `file` sits where a project's canonical entry header
/// would: directly at the repo root, or directly under `src/`. Used
/// to gate the hub-header boost so platform-specific Platform.h-style
/// files deeper in the tree (htop's `solaris/Platform.h`,
/// `freebsd/Platform.h`) don't trigger it — those have comparable
/// include counts but aren't the project's umbrella header.
fn is_at_canonical_entry_location(file: &Path, ctx: &WalkCtx) -> bool {
    match ctx.depth_from_root(file) {
        1 => true,
        2 => {
            file.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                == Some("src")
        }
        _ => false,
    }
}

fn includes_value(file: &Path, ctx: &WalkCtx, conditional_include_count: usize) -> f64 {
    let cat = (0.30 * header_cat_factor(file)).min(1.0);
    let base = mix_signals(cat, 0.55, 0.3, c_depth_factor(file, ctx));
    if !is_header_file(file)
        || conditional_include_count == 0
        || !is_at_canonical_entry_location(file, ctx)
    {
        return base;
    }
    let extra = HUB_INCLUDE_VALUE_PER_DIRECTIVE * conditional_include_count as f64;
    base + extra * c_depth_factor(file, ctx).max(0.0)
}

fn decl_names_value(
    file: &Path,
    ctx: &WalkCtx,
    chunk_index: usize,
    chunk_count: usize,
    strategy: DeclChunkingStrategy,
) -> f64 {
    let cat = (0.80 * header_cat_factor(file)).min(1.0);
    let base = mix_signals(cat, 0.6, 0.35, c_depth_factor(file, ctx));
    // Section-banner chunks are independent subjects (one per module
    // in an amalgamation header), not source-order summaries of the
    // same catalog — so the chunk-index falloff that's appropriate
    // for count-based chunking would unfairly suppress tail-module
    // sections. Flatten to the same per-chunk multiplier the first
    // count-based chunk would carry, so each section competes on its
    // own cost.
    let chunk_factor = match strategy {
        DeclChunkingStrategy::CountBased => names_surface_chunk_factor(chunk_index, chunk_count),
        DeclChunkingStrategy::SectionBanner => names_surface_chunk_factor(0, chunk_count.max(2)),
    };
    base * chunk_factor
}

/// Vendored / shim standard-library headers (`include/stdarg.h`,
/// `include/stdbool.h`, …) under non-root directories. These names
/// match a known C-stdlib header and almost never carry project-canonical
/// content — they're API-compat shims a compiler/runtime ships so its
/// own translation units can `#include <stdarg.h>`. NSes never anchor
/// on their decl-names surface. Apply a flat demotion so they sit
/// behind real project headers in the early budget.
const STDLIB_SHIM_FACTOR: f64 = 0.25;

fn stdlib_shim_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if !is_header_file(file) || ctx.depth_from_root(file) < 2 {
        return 1.0;
    }
    let Some(stem) = c_source_stem(file) else {
        return 1.0;
    };
    if is_known_c_stdlib_stem(stem) {
        STDLIB_SHIM_FACTOR
    } else {
        1.0
    }
}

fn is_known_c_stdlib_stem(stem: &str) -> bool {
    matches!(
        stem,
        "assert"
            | "complex"
            | "ctype"
            | "errno"
            | "fenv"
            | "float"
            | "inttypes"
            | "iso646"
            | "limits"
            | "locale"
            | "math"
            | "setjmp"
            | "signal"
            | "stdalign"
            | "stdarg"
            | "stdatomic"
            | "stdbool"
            | "stddef"
            | "stdint"
            | "stdio"
            | "stdlib"
            | "stdnoreturn"
            | "string"
            | "tgmath"
            | "threads"
            | "time"
            | "uchar"
            | "wchar"
            | "wctype"
    )
}

fn decl_value(file: &Path, kind: DeclKind, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.70 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.85 * k * body_fu_factor(file)).min(1.0);
    mix_signals(cat, fu, 0.65, c_depth_factor(file, ctx))
}

fn decl_doc_value(file: &Path, kind: DeclKind, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.20 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.6 * k * body_fu_factor(file)).min(1.0);
    mix_signals(cat, fu, 0.8, c_depth_factor(file, ctx))
}

fn decl_body_value(file: &Path, kind: DeclKind, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.30 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.80 * k * body_fu_factor(file)).min(1.0);
    mix_signals(cat, fu, 0.7, c_depth_factor(file, ctx))
}

/// Value for one slice of a chunked struct/union/enum body. Calibrated
/// lower than `decl_value` for a whole-Aggregate Decl — each group is
/// one section of the aggregate's identity, not the entire type — so
/// the scheduler still favours unchunked anchors in load-bearing files
/// over a blanket per-group sweep.
fn aggregate_member_group_value(file: &Path, kind: DeclKind, ctx: &WalkCtx) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.45 * k * header_cat_factor(file)).min(1.0);
    let fu = (0.75 * k * body_fu_factor(file)).min(1.0);
    mix_signals(cat, fu, 0.45, c_depth_factor(file, ctx))
}

fn init_table_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Modest value: NS authors rank registration tables in tier 2.5+
    // (typically NS_cum > 3K), so a high value just displaces tier-1
    // orientation that the NS *does* score at 3K. Tuned so tables
    // land between 3K and 10K instead.
    mix_signals(0.25, 0.45, 0.55, c_depth_factor(file, ctx))
}

// --- parser -------------------------------------------------------------

fn parse_c(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_c::LANGUAGE.into())
}

// --- collectors ---------------------------------------------------------

/// Top-of-file `/* */` block (or run of `//` comments). Stops at the
/// first non-comment, non-blank token.
fn collect_header_banner(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "comment" {
            extend_span(&mut lines, child, source);
            continue;
        }
        break;
    }
    FileLines::new(lines)
}

/// 0-based row of the last comment in the file's HeaderBanner — the
/// leading run of comment children of root. Mirrors
/// [`collect_header_banner`]'s span shape so callers can keep
/// `DeclDoc` from crossing the banner boundary. `None` when the file
/// has no leading comment block.
fn header_banner_end_row(tree: &Tree) -> Option<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut end = None;
    for child in root.children(&mut cursor) {
        if child.kind() != "comment" {
            break;
        }
        end = Some(child.end_position().row);
    }
    end
}

/// Collect the file's include-map lines along with a count of
/// conditional `#include` directives folded into the batch — the
/// signal a hub header needs the value boost. The line set captures
/// unconditional `#include`s plus any top-level conditional block
/// whose body is exclusively directive content and contains an
/// `#include` (the feature-gated include-map idiom). The count covers
/// `#include`s nested inside those captured conditional blocks; plain
/// stdlib include lists (chibicc.h-style 18-deep `#include <stdarg.h>`
/// runs) report zero, so the hub boost only fires when the file
/// actually carries a conditional/include-map shape.
fn collect_includes(tree: &Tree, source: &str) -> (FileLines, usize) {
    let mut lines = Vec::new();
    let mut conditional_include_count = 0;
    walk_top_level(tree.root_node(), source, &mut |node| {
        match node.kind() {
            "preproc_include" => extend_span(&mut lines, node, source),
            // Top-level conditional: classify once. If its body is
            // directive-only AND has at least one nested `#include`,
            // capture the whole block range (gate directives stay
            // legible) and count its includes as conditional-map
            // content. Code-heavy conditionals (mongoose amalgamation)
            // stay opaque and contribute nothing.
            "preproc_if" | "preproc_ifdef" => {
                let class = classify_conditional(node);
                if class.directive_only && class.include_count > 0 {
                    extend_span(&mut lines, node, source);
                    conditional_include_count += class.include_count;
                }
            }
            _ => {}
        }
    });
    (
        FileLines::new(dedup_sorted(lines)),
        conditional_include_count,
    )
}

/// Result of classifying a `preproc_if*` / `preproc_else*` block.
#[derive(Default)]
struct ConditionalClass {
    /// True iff every named child is preprocessor content — `#include`,
    /// `#define`, comments, condition/name tokens, and nested
    /// conditionals that are themselves directive-only. A block
    /// wrapping real C code (`declaration`, `function_definition`,
    /// `statement`, `linkage_specification`, …) is not directive-only.
    directive_only: bool,
    /// `#include` directives in this block's subtree, summed across
    /// every branch (top-level + `#else` / `#elif` arms + nested
    /// conditionals).
    include_count: usize,
}

/// Classify a `preproc_if*` / `preproc_else*` block.
fn classify_conditional(node: Node) -> ConditionalClass {
    let mut out = ConditionalClass {
        directive_only: true,
        include_count: 0,
    };
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "preproc_include" => out.include_count += 1,
            "preproc_def" | "preproc_function_def" | "comment" => {}
            "preproc_if" | "preproc_ifdef" | "preproc_else" | "preproc_elif"
            | "preproc_elifdef" => {
                let nested = classify_conditional(child);
                out.include_count += nested.include_count;
                out.directive_only &= nested.directive_only;
            }
            // Condition/name tokens on the `#if` / `#ifdef` itself.
            "identifier"
            | "binary_expression"
            | "parenthesized_expression"
            | "unary_expression"
            | "call_expression"
            | "preproc_defined"
            | "number_literal"
            | "char_literal"
            | "string_literal" => {}
            // Anything else (declaration, function_definition, …) means
            // the block wraps real code.
            _ => out.directive_only = false,
        }
    }
    out
}

fn collect_decl_names_from_with_global_starts(
    decls: &[(Node, DeclInfo)],
    all_starts: &std::collections::HashSet<usize>,
    src_lines: &[&str],
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    // Don't drop an ellipsis on a row owned by another non-ancestor
    // batch — the scheduler's overlap guard would panic when both fire.
    // Three classes to avoid:
    //   - another decl's start row (this chunk or another), since
    //     adjacent single-line decls (`#define` runs) would otherwise
    //     claim the next decl's anchor as a truncation marker;
    //   - `#include` lines, owned by the file's `Includes` batch
    //     (chibicc.h: the first decl is `#define _POSIX_C_SOURCE …` on
    //     line 1, immediately followed by `#include` directives — the
    //     naive ellipsis at line 2 conflicted with `Includes`);
    //   - comment-only lines, which the *next* decl's `DeclDoc` will
    //     claim (a doc-comment run between two decls falls in the
    //     no-man's-land that the previous decl's ellipsis would
    //     otherwise grab).
    for (_, info) in decls {
        full.push(info.start_line);
        let ellipsis_line = info.start_line + 1;
        if !all_starts.contains(&ellipsis_line) && ellipsis_line_safe(ellipsis_line, src_lines) {
            ellipses.push(ellipsis_line);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// True iff the 1-based source line is safe to claim as an ellipsis
/// marker — i.e. not a line another non-ancestor batch owns.
fn ellipsis_line_safe(line: usize, src_lines: &[&str]) -> bool {
    let Some(text) = src_lines.get(line - 1) else {
        return false;
    };
    let trimmed = text.trim_start();
    // `#include` lines belong to the file's `Includes` batch.
    if trimmed.starts_with("#include") {
        return false;
    }
    // Full-line comments — either the previous decl's trailing comment
    // (rare) or the next decl's leading doc comment (common). Either
    // way, another batch will claim them.
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }
    true
}

/// Lines for the decl's signature/header. For function definitions, the
/// signature with a body-elision marker (only when the body has interior
/// rows to elide). For prototypes / typedefs / variables / `#define`s,
/// the whole statement. For struct / union / enum at top level, the
/// whole specifier — except when the decl is chunked into per-member
/// groups (`info.member_groups` non-empty), in which case the parent
/// `Decl` covers only the type header + closing brace so the member
/// groups own the body rows.
///
/// `all_starts` is the set of 1-based `start_line`s of every sibling decl
/// in this translation unit (including this decl's own). Spans are
/// trimmed so they never claim a row that's another decl's start —
/// tree-sitter occasionally produces overlapping nodes (e.g. when a
/// macro like `LLCO_EXTERN` is folded into a following function as a
/// type qualifier and is also surfaced as a sibling node), and the
/// scheduler's non-ancestor-overlap guard would panic on unfolded
/// overlap.
fn collect_decl(
    node: Node,
    info: &DeclInfo,
    source: &str,
    all_starts: &std::collections::HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let start_row = node.start_position().row;
    match info.kind {
        DeclKind::FunctionDef => {
            let sig_end = trim_end_before_next_decl(signature_end_row(node), start_row, all_starts);
            push_rows(&mut full, start_row, sig_end);
            // A single-line `void foo() { ... }` has body.start_row ==
            // body.end_row; emitting an ellipsis there would land on the
            // next decl's start line and trip non-ancestor overlap.
            if let Some(body) = node.child_by_field_name("body") {
                let bs = body.start_position().row;
                let be = body.end_position().row;
                // 1-based line of the row right after sig_end.
                let ellipsis_line = sig_end + 2;
                if be > bs + 1 && !all_starts.contains(&ellipsis_line) {
                    ellipses.push(ellipsis_line);
                }
            }
        }
        DeclKind::FunctionDecl
        | DeclKind::Variable
        | DeclKind::Typedef
        | DeclKind::Aggregate
        | DeclKind::Macro
        | DeclKind::MacroFn => {
            let end_row = trim_end_before_next_decl(
                node_end_row_trimmed(node, source),
                start_row,
                all_starts,
            );
            if !info.member_groups.is_empty()
                && let Some(body) = find_aggregate_body(node)
            {
                // Chunked aggregate: keep the type header (start_row
                // through the body's opening line), the body's closing
                // brace line, and any trailing rows after the body (the
                // `;` line of a `struct foo { … };` is usually the same
                // as the closing-brace line for a bare specifier; for a
                // `typedef … { … } Name;` shape the typedef name lives
                // there).
                let body_start = body.start_position().row;
                let body_end = body.end_position().row;
                push_rows(&mut full, start_row, body_start);
                if body_end > body_start && body_end <= end_row {
                    full.push(body_end + 1);
                }
                if end_row > body_end {
                    push_rows(&mut full, body_end + 1, end_row);
                }
            } else {
                push_rows(&mut full, start_row, end_row);
            }
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Body interior of a function definition: rows strictly between the
/// `compound_statement`'s `{` and `}`, with blank source rows skipped.
/// `excluded` is a list of 1-based inclusive line ranges to leave out —
/// used by [`InitTableRows`](CKey::InitTableRows) so DeclBody and
/// InitTableRows can be emitted as disjoint siblings under the same
/// `Decl` predecessor. Returns empty when the body has no interior to
/// render (single-line body or all-blank interior).
fn collect_decl_body(node: Node, src_lines: &[&str], excluded: &[(usize, usize)]) -> FileLines {
    let Some(body) = node.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let s = body.start_position().row;
    let e = body.end_position().row;
    if e <= s + 1 {
        return FileLines::new(Vec::new());
    }
    let mut out = Vec::new();
    'rows: for row in (s + 1)..e {
        let line = row + 1;
        for (lo, hi) in excluded {
            if line >= *lo && line <= *hi {
                continue 'rows;
            }
        }
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(line);
        }
    }
    FileLines::new(out)
}

/// Detected `static struct { ... } X[] = { ... };` registration table
/// inside a function body. Used to emit a per-table batch independent
/// of the surrounding [`CKey::DeclBody`].
struct InitTable<'a> {
    /// The `declaration` node spanning `static struct { ... } X[] = { ... };`.
    decl_node: Node<'a>,
    /// The outer `initializer_list` (the RHS `{ ... }`).
    outer_list: Node<'a>,
}

/// Minimum number of inner initializer-list entries for a declaration
/// to qualify as a registration table. Two is the floor — sqlite-vec's
/// `aMod[]` has exactly two entries and the NS still credits it.
const INIT_TABLE_MIN_ENTRIES: usize = 2;

/// Maximum row span of an inner initializer-list. Each registration
/// row should fit on a line or two; multi-line entries usually mean a
/// non-table use of static array (or a nested struct that wouldn't
/// render usefully one-line-per-entry).
const INIT_TABLE_MAX_INNER_ROWS: usize = 2;

fn find_init_tables_in_body<'a>(node: Node<'a>) -> Vec<InitTable<'a>> {
    let Some(body) = node.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut cursor = body.walk();
    for child in body.named_children(&mut cursor) {
        if child.kind() != "declaration" {
            continue;
        }
        let Some(init_decl) = init_declarator_of(child) else {
            continue;
        };
        let Some(declarator) = init_decl.child_by_field_name("declarator") else {
            continue;
        };
        if declarator.kind() != "array_declarator" {
            continue;
        }
        let Some(outer_list) = init_decl.child_by_field_name("value") else {
            continue;
        };
        if outer_list.kind() != "initializer_list" {
            continue;
        }
        let inner_lists = inner_initializer_lists(outer_list);
        if inner_lists.len() < INIT_TABLE_MIN_ENTRIES {
            continue;
        }
        if !inner_lists.iter().all(|l| {
            l.end_position().row.saturating_sub(l.start_position().row) < INIT_TABLE_MAX_INNER_ROWS
        }) {
            continue;
        }
        out.push(InitTable {
            decl_node: child,
            outer_list,
        });
    }
    out
}

fn init_declarator_of(decl: Node) -> Option<Node> {
    let mut cursor = decl.walk();
    decl.named_children(&mut cursor)
        .find(|child| child.kind() == "init_declarator")
}

fn inner_initializer_lists<'a>(outer: Node<'a>) -> Vec<Node<'a>> {
    let mut cursor = outer.walk();
    outer
        .named_children(&mut cursor)
        .filter(|c| c.kind() == "initializer_list")
        .collect()
}

/// Emit the row spec for an `InitTableRows` batch: one line per inner
/// initializer's start row. NS authors trim these tables to just the
/// data rows (skipping the `static struct {…} X[] = {` preamble and
/// the closing `};`), so the walker matches that shape — the table
/// name is recoverable from the source `path:line` rendered prefix.
fn collect_init_table_rows(table: &InitTable) -> FileLines {
    let mut full = Vec::new();
    for inner in inner_initializer_lists(table.outer_list) {
        full.push(inner.start_position().row + 1);
    }
    FileLines::new(dedup_sorted(full))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    fn parse(source: &str) -> (String, Tree) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_c::LANGUAGE.into())
            .expect("load c grammar");
        let tree = parser.parse(source, None).expect("parse");
        (source.to_string(), tree)
    }

    #[test]
    fn c_header_guard_recognized_for_various_naming_conventions() {
        let cases = &[
            "#ifndef __SDS_H\n#define __SDS_H\nint foo();\n#endif\n",
            "#ifndef KREP_H\n#define KREP_H\nint foo();\n#endif\n",
            "#ifndef AHO_CORASICK_H\n#define AHO_CORASICK_H\nint foo();\n#endif\n",
            "#ifndef H_PACKETS\n#define H_PACKETS\nint foo();\n#endif\n",
        ];
        for src in cases {
            let (source, tree) = parse(src);
            let body = header_guard_body_node(tree.root_node(), &source);
            assert!(body.is_some(), "header guard not detected in:\n{src}");
            let decls = find_decls(&tree, &source, std::path::Path::new("test.h"));
            assert_eq!(
                decls.len(),
                1,
                "expected the one prototype to be visible after descending the guard:\n{src}"
            );
            assert_eq!(decls[0].1.kind, DeclKind::FunctionDecl);
        }
    }

    #[test]
    fn c_includes_capture_directive_only_conditional_block() {
        // The canonical "feature-gated include map" — `#if CFG_FOO {
        // #include "x.h" }` — should be folded into the Includes batch
        // so a reader sees which optional/platform components are
        // available. The block range (gate + include + #endif) is
        // captured verbatim.
        let src = "\
#include <stdint.h>

#if CFG_FOO_ENABLED
  #include \"foo.h\"

  #if CFG_FOO_HID
    #include \"hid_foo.h\"
  #endif
#endif

int bar(int x);
";
        let (source, tree) = parse(src);
        let (fl, conditional_count) = collect_includes(&tree, &source);
        // Lines 1 (#include stdint), 3-9 (the conditional block,
        // collapsing blanks). Blank lines (line 2, 5, 9 inside block)
        // are dropped by build_file_spans, not by the collector.
        let mut full = fl.full.clone();
        full.sort();
        assert_eq!(full, vec![1, 3, 4, 5, 6, 7, 8, 9]);
        // 2 `#include`s inside the captured conditional map — that's
        // the signal the hub-header boost rides on.
        assert_eq!(conditional_count, 2);
    }

    #[test]
    fn c_includes_skip_conditional_wrapping_real_code() {
        // A conditional that wraps real declarations (mongoose-style
        // amalgamation: `#if MG_ENABLE_HTTP { /* big impl */ }`) must
        // not be captured by Includes — only the unconditional
        // `#include` survives. The nested `#include` doesn't count
        // toward the hub signal either, since the conditional was
        // skipped: counting it would re-introduce the false positive
        // the directive-only gate exists to prevent.
        let src = "\
#include <stdint.h>

#if MG_ENABLE_HTTP
#include \"http_priv.h\"
int http_serve(void) { return 0; }
#endif
";
        let (source, tree) = parse(src);
        let (fl, conditional_count) = collect_includes(&tree, &source);
        let mut full = fl.full.clone();
        full.sort();
        // Only the unconditional include at line 1 — the conditional
        // wraps a `function_definition` so it stays opaque.
        assert_eq!(full, vec![1]);
        assert_eq!(conditional_count, 0);
    }

    #[test]
    fn c_includes_skip_conditional_without_any_include() {
        // A directive-only conditional that has no `#include` inside
        // (e.g., `#ifdef __GNUC__ { #define FALLTHROUGH … }`) isn't
        // part of the include map — leave it to the per-decl Macro
        // batches.
        let src = "\
#include <stdint.h>

#ifdef __GNUC__
  #define FALLTHROUGH __attribute__((fallthrough))
#endif
";
        let (source, tree) = parse(src);
        let (fl, conditional_count) = collect_includes(&tree, &source);
        let mut full = fl.full.clone();
        full.sort();
        assert_eq!(full, vec![1]);
        assert_eq!(conditional_count, 0);
    }

    #[test]
    fn c_includes_plain_stdlib_list_reports_no_conditional_includes() {
        // chibicc.h-style: many unconditional `#include <stdlib.h>`
        // directives. These shouldn't trigger the hub-header boost;
        // the hub framing is about feature-gated subsystem maps, not
        // a stdlib preamble.
        let src = "\
#include <assert.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/types.h>

int foo(int x);
";
        let (source, tree) = parse(src);
        let (_fl, conditional_count) = collect_includes(&tree, &source);
        assert_eq!(conditional_count, 0);
    }

    #[test]
    fn c_feature_gated_linkage_block_stays_opaque() {
        // A non-`__cplusplus` `#ifdef`/`#ifndef` around an `extern "C"`
        // block is a feature/platform gate, not the C++-only linkage
        // idiom. The wrapped decls must NOT be surfaced as top-level
        // public API — otherwise the walker would advertise
        // platform-gated entrypoints as unconditional.
        let cases = &[
            "\
#ifdef FEATURE_X
extern \"C\" {
int gated_only(int x);
}
#endif
",
            "\
#ifndef NO_LINKAGE
extern \"C\" {
int gated_only(int x);
}
#endif
",
        ];
        for src in cases {
            let (source, tree) = parse(src);
            let decls = find_decls(&tree, &source, std::path::Path::new("test.h"));
            assert!(
                decls.is_empty(),
                "feature-gated linkage spec must stay opaque; got decls:\n{src}\n{decls:?}",
            );
        }
    }

    #[test]
    fn c_extern_c_block_is_descended_transparently() {
        // Bare `extern "C" { ... }` — decls must surface as top-level.
        let bare = "\
extern \"C\" {
int foo(int x);
typedef int bar_t;
}
";
        // `#ifdef __cplusplus / extern \"C\" {` envelope idiom from real
        // headers (jq.h, neco.h, tinyusb's tusb.h). tree-sitter-c parses
        // the matching `{` and `}` into a single `linkage_specification`
        // even though each lives in its own `#ifdef __cplusplus` block.
        let wrapped = "\
#ifdef __cplusplus
extern \"C\" {
#endif

int foo(int x);
typedef int bar_t;

#ifdef __cplusplus
}
#endif
";
        for src in &[bare, wrapped] {
            let (source, tree) = parse(src);
            let decls = find_decls(&tree, &source, std::path::Path::new("test.h"));
            let kinds: Vec<DeclKind> = decls.iter().map(|(_, d)| d.kind).collect();
            assert_eq!(
                kinds,
                vec![DeclKind::FunctionDecl, DeclKind::Typedef],
                "expected wrapped decls to be visible at top-level:\n{src}\ngot {decls:?}",
            );
        }
    }

    #[test]
    fn c_static_inline_in_header_is_public_but_not_in_c() {
        let header = "static inline int sdslen(const char *s) { return 0; }\n";
        let (source, tree) = parse(header);
        let h_decls = find_decls(&tree, &source, std::path::Path::new("sds.h"));
        let c_decls = find_decls(&tree, &source, std::path::Path::new("sds.c"));
        assert_eq!(h_decls.len(), 1, "static inline in .h should be public");
        assert_eq!(h_decls[0].1.kind, DeclKind::FunctionDef);
        assert!(c_decls.is_empty(), "static in .c should be private");
    }

    #[test]
    fn c_top_level_typedef_struct_function_decl_classified() {
        let src = "\
typedef char *sds;
struct sdshdr8 { unsigned char flags; };
sds sdsnew(const char *init);
#define SDS_MAX_PREALLOC (1024*1024)
sds sdsnewlen(const void *init, size_t initlen) { return 0; }
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("sds.h"));
        let kinds: Vec<DeclKind> = decls.iter().map(|(_, d)| d.kind).collect();
        assert_eq!(
            kinds,
            vec![
                DeclKind::Typedef,
                DeclKind::Aggregate,
                DeclKind::FunctionDecl,
                DeclKind::Macro,
                DeclKind::FunctionDef,
            ],
            "decl kinds mismatch; got {:?}\nsource:\n{src}",
            decls.iter().map(|(_, d)| d.kind).collect::<Vec<_>>()
        );
    }

    #[test]
    fn c_macro_modifier_pattern_does_not_overlap_following_decl() {
        // tree-sitter-c folds `LLCO_EXTERN` (an attribute-like macro) into
        // the following function as a type qualifier, so the
        // `function_definition` spans the macro line plus the signature
        // line. If the catch-all decl for the macro and the FunctionDef
        // both claim the signature line, the scheduler panics on
        // non-ancestor overlap. The trim in collect_decl prevents that.
        let src = "\
LLCO_EXTERN
void llco_first(void) { return; }
LLCO_EXTERN
void llco_second(void) { return; }
";
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("neco-mini.c"), src).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        // The whole point: this used to panic on non-ancestor overlap.
        let report = scheduler.run_with_report();
        assert!(
            !report.scheduled.is_empty(),
            "expected at least one scheduled batch"
        );

        // Stronger guarantee: spans claimed by C::Decl batches in the
        // same file are pairwise disjoint by row.
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("neco-mini.c"));
        let all_starts: std::collections::HashSet<usize> =
            decls.iter().map(|(_, i)| i.start_line).collect();
        let mut claimed: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (node, info) in &decls {
            let lines = collect_decl(*node, info, &source, &all_starts);
            for line in &lines.full {
                let prev = claimed.insert(*line, info.start_line);
                assert!(
                    prev.is_none(),
                    "line {line} claimed by decls starting at {} and {} — overlap",
                    prev.unwrap(),
                    info.start_line,
                );
            }
        }
    }

    #[test]
    fn c_init_table_rows_recognizer_finds_static_array_of_structs() {
        // 4-entry positive case.
        let src = "\
int init(void) {
  static struct { const char *n; int v; } aReg[] = {
    {\"a\", 1},
    {\"b\", 2},
    {\"c\", 3},
    {\"d\", 4},
  };
  return 0;
}
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("test.c"));
        let fn_decls: Vec<_> = decls
            .iter()
            .filter(|(_, i)| i.kind == DeclKind::FunctionDef)
            .collect();
        assert_eq!(fn_decls.len(), 1, "expected 1 fn def, got {decls:?}");
        let tables = find_init_tables_in_body(fn_decls[0].0);
        assert_eq!(tables.len(), 1, "expected 1 init table");

        let rows = collect_init_table_rows(&tables[0]);
        // Expected: one row per inner entry. The fixture has four
        // entries on lines 3, 4, 5, 6 (1-based). No framing rows.
        assert_eq!(rows.full, vec![3, 4, 5, 6]);
    }

    #[test]
    fn c_init_table_rows_recognizer_skips_too_few_entries() {
        // 1-entry array isn't a table — below the 2-entry floor.
        let src = "\
int init(void) {
  static struct { int v; } aSingleton[] = {
    {1},
  };
  return 0;
}
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source, std::path::Path::new("test.c"));
        let fn_decl = decls
            .iter()
            .find(|(_, i)| i.kind == DeclKind::FunctionDef)
            .expect("fn def");
        let tables = find_init_tables_in_body(fn_decl.0);
        assert!(
            tables.is_empty(),
            "1-entry init list should not match: {tables:?}",
            tables = tables.len()
        );
    }

    #[test]
    fn c_init_table_rows_disjoint_from_decl_body() {
        // The DeclBody for the enclosing function MUST exclude rows
        // claimed by InitTableRows (siblings under the same Decl
        // predecessor — non-ancestor overlap would panic the
        // scheduler).
        let src = "\
int init(void) {
  int before = 1;
  static struct { const char *n; int v; } aReg[] = {
    {\"a\", 1},
    {\"b\", 2},
    {\"c\", 3},
    {\"d\", 4},
  };
  int after = 2;
  return 0;
}
";
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("init.c"), src).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        // The scheduler runs to completion only if no overlap panic
        // fires; the disjoint-row guarantee is checked there.
        let report = scheduler.run_with_report();
        let has_table = report
            .scheduled
            .iter()
            .any(|r| matches!(&r.key, BatchKey::C(CKey::InitTableRows { .. })));
        assert!(
            has_table,
            "expected an InitTableRows batch; keys: {:?}",
            report.scheduled.iter().map(|r| &r.key).collect::<Vec<_>>()
        );
    }

    #[test]
    fn c_real_dir_renders_seeded_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("foo.h"),
            "#ifndef FOO_H\n#define FOO_H\nint foo(int x);\n#endif\n",
        )
        .unwrap();
        std::fs::write(
            root.join("foo.c"),
            "#include \"foo.h\"\nint foo(int x) { return x + 1; }\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();
        assert!(
            rendered.contains("int foo(int x)"),
            "expected the C prototype/signature in rendered output:\n{rendered}",
        );

        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        let has_decl_names = keys
            .iter()
            .any(|k| matches!(k, BatchKey::C(CKey::DeclNames { .. })));
        assert!(
            has_decl_names,
            "expected a C::DeclNames batch; keys: {keys:?}"
        );
    }

    /// Run the scheduler on a synthetic single-file C fixture. Returns
    /// only after `run` completes — in debug builds the call panics on
    /// any non-ancestor overlap, so a successful return is the assertion.
    fn assert_c_walker_overlap_free(filename: &str, src: &str) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join(filename), src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        assert!(
            !report.scheduled.is_empty(),
            "expected at least one scheduled batch for {filename}; src:\n{src}"
        );
    }

    #[test]
    fn c_walker_overlap_audit_decl_then_includes() {
        // chibicc.h shape: a `#define` (a `Macro` decl) on line 1
        // followed immediately by `#include` directives. Without the
        // ellipsis-safety check, `DeclNames` would claim line 2 as an
        // ellipsis while `Includes` claims it as a real line.
        let src = "\
#define _POSIX_C_SOURCE 200809L
#include <assert.h>
#include <stdio.h>

int foo(int x);
";
        assert_c_walker_overlap_free("foo.h", src);
    }

    #[test]
    fn c_walker_overlap_audit_eol_comment_before_next_decl() {
        // tinyusb video.h shape: every decl carries an end-of-line
        // comment. The tree-sitter `comment` for the EOL trailer is a
        // sibling of the *next* decl, and `collect_doc_comments_above`
        // used to grab it as a `DeclDoc` — which then claimed a line
        // already owned by the previous `Decl`.
        let src = "\
typedef int alpha; // trailer
typedef int beta;  // trailer
typedef int gamma;
";
        assert_c_walker_overlap_free("eol.h", src);
    }

    #[test]
    fn c_walker_overlap_audit_eol_comment_run_before_decl() {
        // Multiple consecutive EOL-trailered decls — the prev-sibling
        // walk must stop at the first EOL trailer (not fall through to
        // an earlier real doc).
        let src = "\
// real doc for first
typedef int first; // EOL trailer
typedef int second;
";
        assert_c_walker_overlap_free("eol-run.h", src);
    }

    #[test]
    fn c_walker_overlap_audit_banner_touching_first_decl() {
        // File-top comment block followed immediately by the first
        // decl with no blank line. `HeaderBanner` claims the comments
        // and so does `DeclDoc` if it walks back into them — the two
        // are not in an ancestor relationship.
        let src = "\
/* license */
/* brief */
typedef int x;
";
        assert_c_walker_overlap_free("banner-touch.h", src);
    }

    #[test]
    fn c_walker_overlap_audit_multichunk_decls_with_comments_between() {
        // > 24 decls forces `DeclNames` to chunk. A doc comment line
        // sitting between two chunks must not be claimed both by the
        // previous chunk's ellipsis and by the next chunk's first
        // decl's `DeclDoc`.
        let mut src = String::new();
        for i in 0..30 {
            src.push_str(&format!("int fn_{i}(void);\n"));
            if i == 23 {
                src.push_str("// boundary doc\n");
            }
        }
        assert_c_walker_overlap_free("multichunk.h", &src);
    }

    #[test]
    fn c_walker_overlap_audit_adjacent_macro_runs() {
        // Long run of single-line `#define`s. `DeclNames` ellipsis
        // would otherwise land on each subsequent decl's start row.
        let mut src = String::new();
        for i in 0..40 {
            src.push_str(&format!("#define CONST_{i} {i}\n"));
        }
        assert_c_walker_overlap_free("macros.h", &src);
    }

    /// `Makefile.am`'s `include_HEADERS` line, with `$(srcdir)/` refs
    /// and `\` line continuations, parses to the set of declared
    /// public-API header paths.
    #[test]
    fn c_include_headers_parsed_from_makefile_am() {
        let tmp = tempfile::tempdir().expect("tmpdir");
        let root = tmp.path();
        std::fs::create_dir_all(root.join("src")).expect("mkdir");
        for name in ["jv.h", "jq.h", "jv_private.h"] {
            std::fs::write(root.join("src").join(name), "").expect("write header");
        }
        std::fs::write(
            root.join("Makefile.am"),
            "AM_CFLAGS = -Wall\n\
             include_HEADERS = src/jv.h \\\n\
                               src/jq.h\n\
             nobase_include_HEADERS = $(srcdir)/src/jv.h\n",
        )
        .expect("write Makefile.am");
        let set = parse_include_headers(root).expect("set");
        assert_eq!(set.len(), 2);
        assert!(set.contains(&root.join("src/jv.h").canonicalize().unwrap()));
        assert!(set.contains(&root.join("src/jq.h").canonicalize().unwrap()));
        assert!(!set.contains(&root.join("src/jv_private.h").canonicalize().unwrap()));
    }

    /// No `Makefile.am`, or one without an `include_HEADERS` line,
    /// returns `None` — internal-header demotion must not fire on
    /// projects with no public-API declaration.
    #[test]
    fn c_include_headers_absent_returns_none() {
        let tmp = tempfile::tempdir().expect("tmpdir");
        assert!(parse_include_headers(tmp.path()).is_none());
        std::fs::write(tmp.path().join("Makefile.am"), "AM_CFLAGS = -Wall\n").expect("write");
        assert!(parse_include_headers(tmp.path()).is_none());
    }

    #[test]
    fn c_aggregate_struct_with_three_blank_line_groups_chunks() {
        // chibicc-style: large struct with ≥3 blank-line-separated field
        // groups. Each group becomes an AggregateMemberGroup; the parent
        // Decl is trimmed to the header + closer so the two render
        // disjoint body rows.
        let mut src = String::from("struct Obj {\n");
        for i in 0..10 {
            src.push_str(&format!("  int field_a_{i};\n"));
        }
        src.push('\n');
        for i in 0..10 {
            src.push_str(&format!("  int field_b_{i};\n"));
        }
        src.push('\n');
        for i in 0..10 {
            src.push_str(&format!("  int field_c_{i};\n"));
        }
        src.push_str("};\n");
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("obj.h"), &src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        let group_count = report
            .scheduled
            .iter()
            .filter(|r| matches!(&r.key, BatchKey::C(CKey::AggregateMemberGroup { .. })))
            .count();
        assert_eq!(
            group_count,
            3,
            "expected 3 AggregateMemberGroup batches; keys: {:?}",
            report.scheduled.iter().map(|r| &r.key).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn c_aggregate_struct_with_two_groups_stays_whole() {
        // Two-group struct: below the chunking threshold (3 groups),
        // emitted as a single whole-aggregate Decl.
        let mut src = String::from("struct Pair {\n");
        for i in 0..20 {
            src.push_str(&format!("  int a_{i};\n"));
        }
        src.push('\n');
        for i in 0..20 {
            src.push_str(&format!("  int b_{i};\n"));
        }
        src.push_str("};\n");
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("pair.h"), &src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        let has_group = report
            .scheduled
            .iter()
            .any(|r| matches!(&r.key, BatchKey::C(CKey::AggregateMemberGroup { .. })));
        assert!(
            !has_group,
            "two-group struct must not chunk; keys: {:?}",
            report.scheduled.iter().map(|r| &r.key).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn c_aggregate_big_enum_chunks_into_fixed_size_groups() {
        // chibicc-style NodeKind: 49 enumerators. The enum body splits
        // into AGGREGATE_ENUM_CHUNK_SIZE-sized chunks.
        let mut src = String::from("typedef enum {\n");
        for i in 0..40 {
            src.push_str(&format!("  ND_{i},\n"));
        }
        src.push_str("} NodeKind;\n");
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("nodekind.h"), &src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        let group_count = report
            .scheduled
            .iter()
            .filter(|r| matches!(&r.key, BatchKey::C(CKey::AggregateMemberGroup { .. })))
            .count();
        let expected = 40_usize.div_ceil(AGGREGATE_ENUM_CHUNK_SIZE);
        assert_eq!(
            group_count, expected,
            "expected {expected} enum chunks for 40 enumerators at chunk size {AGGREGATE_ENUM_CHUNK_SIZE}",
        );
    }

    #[test]
    fn c_aggregate_big_enum_preserves_multiline_enumerator_rows() {
        // Multi-line enumerator values (`FOO = (1 << 20)\n  | (1 << 21),`)
        // must contribute every row to the chunk that owns them — the
        // trimmed parent Decl no longer covers them, so dropping
        // continuation rows would silently truncate the enum.
        let mut src = String::from("typedef enum {\n");
        for i in 0..(AGGREGATE_ENUM_CHUNK_MIN + 2) {
            // Every other enumerator has a multi-line value.
            if i % 2 == 0 {
                src.push_str(&format!(
                    "  K_{i} = (1 << {i})\n         | (1 << {}),\n",
                    i + 1
                ));
            } else {
                src.push_str(&format!("  K_{i},\n"));
            }
        }
        src.push_str("} Multi;\n");
        let (source, tree) = parse(&src);
        let decls = find_decls(&tree, &source, std::path::Path::new("multi.h"));
        let enum_decl = decls
            .iter()
            .find(|(_, d)| d.kind == DeclKind::Typedef && !d.member_groups.is_empty())
            .expect("multi-line enum should be chunked");
        let mut all_rows: Vec<usize> = Vec::new();
        for group in &enum_decl.1.member_groups {
            all_rows.extend(&group.rows);
        }
        all_rows.sort();
        all_rows.dedup();
        // Continuation rows: enumerator i (even) starts at line 2+2i and
        // ends at 2+2i+1. Every row between body open and close should
        // appear in some group.
        let multiline_continuations: Vec<usize> = (0..(AGGREGATE_ENUM_CHUNK_MIN + 2))
            .filter(|i| i % 2 == 0)
            .map(|i| {
                // Lines are 1-based; the typedef header is line 1, first
                // enumerator starts at line 2. Each multi-line enumerator
                // before this one consumed 2 lines, each single-line one
                // consumed 1. K_i starts at: 2 + sum over j<i of (j%2==0 ? 2 : 1).
                let prev_lines: usize = (0..i).map(|j| if j % 2 == 0 { 2 } else { 1 }).sum();
                // continuation row = first row + 1.
                2 + prev_lines + 1
            })
            .collect();
        for cont in &multiline_continuations {
            assert!(
                all_rows.contains(cont),
                "continuation row {cont} missing from chunked rows {all_rows:?}",
            );
        }
    }

    #[test]
    fn c_aggregate_small_enum_stays_whole() {
        // Below AGGREGATE_ENUM_CHUNK_MIN enumerators: a single whole-Decl
        // batch with no chunking.
        let mut src = String::from("typedef enum {\n");
        for i in 0..(AGGREGATE_ENUM_CHUNK_MIN - 1) {
            src.push_str(&format!("  K_{i},\n"));
        }
        src.push_str("} SmallKind;\n");
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("small.h"), &src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        let has_group = report
            .scheduled
            .iter()
            .any(|r| matches!(&r.key, BatchKey::C(CKey::AggregateMemberGroup { .. })));
        assert!(
            !has_group,
            "small enum must not chunk; keys: {:?}",
            report.scheduled.iter().map(|r| &r.key).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn c_section_banner_comment_recognizer() {
        for text in [
            "// strings.c",
            "//  tokenize.c",
            "//tokenize.c ",
            "/* preprocess.c */",
            "/*  parse.c  */",
            "// aho_corasick.c",
            "// foo-bar.c",
        ] {
            assert!(
                is_module_section_banner_comment(text),
                "expected banner match for {text:?}",
            );
        }
        for text in [
            "//",
            "// just a comment",
            "// foo.h",
            "// strings.cpp",
            "// 12 lines below",
            "/* license boilerplate */",
            "// .c",
            "// strings.c extra",
        ] {
            assert!(
                !is_module_section_banner_comment(text),
                "unexpected banner match for {text:?}",
            );
        }
    }

    #[test]
    fn c_section_banner_chunking_partitions_decls_by_module() {
        // chibicc-style amalgamation header at the canonical entry
        // location: each `// stem.c` banner partitions the following
        // decls into a chunk. The preamble decls form chunk 0; the
        // tail-module banners get their own chunks even when those
        // sections carry only a handful of prototypes.
        let src = "\
typedef struct Type Type;
typedef struct Node Node;

//
// strings.c
//

void strarray_push(int x);
void format(int x);

//
// tokenize.c
//

void tokenize(int x);

//
// codegen.c
//

void codegen(int x);

//
// main.c
//

int file_exists(int x);
";
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("chibicc.h"), src).unwrap();
        // The walker runs against the directory; we read decls through
        // the public path so canonical-entry gating is observed.
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 20_000, None);
        let report = scheduler.run_with_report();
        let chunk_indices: Vec<usize> = report
            .scheduled
            .iter()
            .filter_map(|r| match &r.key {
                BatchKey::C(CKey::DeclNames { file, chunk_index })
                    if file.ends_with("chibicc.h") =>
                {
                    Some(*chunk_index)
                }
                _ => None,
            })
            .collect();
        // 4 banners + 1 preamble = 5 chunks, all with decls so all
        // emitted.
        let mut sorted = chunk_indices.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            vec![0, 1, 2, 3, 4],
            "expected 5 section chunks, got {chunk_indices:?}",
        );
    }

    #[test]
    fn c_section_banner_chunking_falls_back_below_minimum_count() {
        // Only 2 banners — below SECTION_BANNER_MIN_COUNT (= 3). Should
        // fall back to count-based chunking; with this small a surface
        // (3 decls) a single unchunked DeclNames batch is emitted.
        let src = "\
//
// strings.c
//
void foo(int x);

//
// tokenize.c
//
void bar(int x);
void baz(int x);
";
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("small.h"), src).unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 20_000, None);
        let report = scheduler.run_with_report();
        let chunk_count = report
            .scheduled
            .iter()
            .filter(|r| {
                matches!(
                    &r.key,
                    BatchKey::C(CKey::DeclNames { file, .. }) if file.ends_with("small.h")
                )
            })
            .count();
        assert_eq!(
            chunk_count, 1,
            "below SECTION_BANNER_MIN_COUNT banners → single count-based chunk",
        );
    }

    /// `+=` appends and `$(VAR)` expansions would leave an incomplete
    /// public set if parsed naively; both must fail open so a real
    /// public header isn't silently demoted to internal.
    #[test]
    fn c_include_headers_fail_open_on_unsupported_syntax() {
        for body in [
            "include_HEADERS = src/jv.h\ninclude_HEADERS += src/jq.h\n",
            "include_HEADERS = src/jv.h $(EXTRA_API_HEADERS)\n",
            "include_HEADERS = src/jv.h ${EXTRA_API_HEADERS}\n",
            "include_HEADERS += src/jv.h\n",
        ] {
            let tmp = tempfile::tempdir().expect("tmpdir");
            std::fs::write(tmp.path().join("Makefile.am"), body).expect("write");
            assert!(
                parse_include_headers(tmp.path()).is_none(),
                "expected fail-open for:\n{body}",
            );
        }
    }
}
