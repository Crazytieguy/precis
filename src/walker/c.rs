//! C / C-header walker. Per-decl batches plus file-scope banner /
//! includes / decl-name surface.
//!
//! Per-file keys:
//! - `HeaderBanner { file }`: top-of-file `/* */` block (license / brief)
//! - `Includes { file }`: `#include` directives at the top of the file
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
//!
//! Public-vs-private rule:
//! - `.c` files: top-level `static` items are excluded (file-private).
//! - `.h` files: `static inline` definitions are included (header-only
//!   inline accessors are part of the public API expansion); other
//!   `static` items are excluded.
//! - The wrapping `#ifndef X` / `#define X` / `#endif` header guard
//!   (recognized structurally — first `#ifndef` whose name is then
//!   `#define`d on the next line, regardless of naming convention) is
//!   descended into transparently. Other `preproc_if` / `preproc_ifdef`
//!   blocks render verbatim as a single decl line at the top.
//!
//! Parse trees are cached in [`WalkCtx`].

use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, CKey};
use crate::value::{mix_signals, names_surface_chunk_factor};

/// Chunk size for C declaration-name surfaces. Larger than the
/// `NAMES_SURFACE_CHUNK_SIZE = 12` used by Python/TS because C
/// headers regularly expose 50+ decls and NS authors anchor on
/// unified subset rows (e.g. sds's "Public fn declarations — utility
/// fns" covers 14 specific lines). A 24-decl chunk keeps the
/// surface coherent for files in the 25–48 decl band while still
/// splitting catalog headers like krep.h (~80 decls) so the first
/// chunk reaches the budget.
const C_DECL_NAMES_CHUNK_SIZE: usize = 24;

fn c_names_surface_chunk_index(decl_index: usize) -> usize {
    decl_index / C_DECL_NAMES_CHUNK_SIZE
}

use super::{
    FileLines, WalkCtx, build_per_file_content, dedup_sorted, extend_span, file_depth_factor,
    file_lines_covered_by, fs::files_with_any_extension, push_rows, signature_end_row,
    single_file_lines_content,
};

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let c_files = files_with_any_extension(dir, &["c", "h"]);
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

        if let Some(content) = build_per_file_content(file, ctx, parse_c, collect_includes) {
            out.push(Batch {
                key: CKey::Includes { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: includes_value(file, ctx),
            });
        }

        let Some((source, tree)) = parse_c(ctx, file) else {
            continue;
        };
        let decls = find_decls(&tree, &source, file);
        if decls.is_empty() {
            continue;
        }
        // C catalog files (`sds.h`, headers exposing the whole public
        // surface) often anchor NS rows on the *unified* declaration
        // listing — splitting at the default 12 fragments rows like
        // "Public fn declarations — utility fns" across chunks. Chunk
        // only when the surface is large enough that the unified batch
        // would lose the value/cost race against per-decl batches.
        let chunk_size = C_DECL_NAMES_CHUNK_SIZE;
        let names_chunk_count = if decls.len() <= chunk_size {
            1
        } else {
            decls.len().div_ceil(chunk_size)
        };
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
        let names_lines_by_chunk: Vec<FileLines> = decls
            .chunks(chunk_size)
            .map(|c| collect_decl_names_from_with_global_starts(c, &all_starts))
            .collect();
        for (chunk_index, names_lines) in names_lines_by_chunk.iter().enumerate() {
            let Some(content) = single_file_lines_content(file, &source, names_lines.clone())
            else {
                continue;
            };
            out.push(Batch {
                key: names_predecessors[chunk_index].clone(),
                predecessor: None,
                content,
                value: decl_names_value(file, ctx, chunk_index, names_chunk_count),
            });
        }
        let src_lines: Vec<&str> = source.lines().collect();
        for (decl_index, (node, info)) in decls.iter().enumerate() {
            let names_chunk_index = c_names_surface_chunk_index(decl_index);
            let names_predecessor = names_predecessors[names_chunk_index].clone();
            let chunk_names_lines = &names_lines_by_chunk[names_chunk_index];
            let decl_key = CKey::Decl {
                file: file.clone(),
                start_line: info.start_line,
            };
            let decl_lines = collect_decl(*node, info, &source);
            let doc_lines = collect_decl_doc(*node, &source);
            let body_lines = if info.has_body {
                collect_decl_body(*node, &src_lines)
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
                    predecessor: Some(decl_predecessor),
                    content,
                    value: decl_body_value(file, info.kind, ctx),
                });
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
    /// Catch-all for top-level nodes we render but don't classify
    /// further (linkage_specification, attributed_statement, the
    /// rendered representation of an unrecognized preproc_if block).
    Other,
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
            DeclKind::Other => 0.85,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DeclInfo {
    start_line: usize,
    kind: DeclKind,
    /// True for `function_definition` only — gates `DeclBody` emission.
    has_body: bool,
}

/// All public top-level decls in `file`'s tree, in source order, paired
/// with the AST node so per-decl collectors don't have to re-walk.
/// Descends transparently through a single wrapping `#ifndef X` /
/// `#define X` / `#endif` header guard.
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
/// minus the wrapping header guard, with the guard's body's children
/// raised to top-level instead.
fn walk_top_level<'a, F: FnMut(Node<'a>)>(root: Node<'a>, source: &str, visit: &mut F) {
    let header_guard_body = header_guard_body_node(root, source);
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if Some(child) == header_guard_body {
            let mut inner = child.walk();
            for inner_child in child.children(&mut inner) {
                visit(inner_child);
            }
        } else {
            visit(child);
        }
    }
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
    match node.kind() {
        "function_definition" => {
            let static_ = has_static_specifier(node, source);
            let inline = has_inline_specifier(node, source);
            // .c file: static excluded. .h file: static excluded unless
            // also inline (header-only inline accessor).
            if static_ && !(in_header && inline) {
                return None;
            }
            Some(DeclInfo {
                start_line,
                kind: DeclKind::FunctionDef,
                has_body: true,
            })
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
            Some(DeclInfo {
                start_line,
                kind,
                has_body: false,
            })
        }
        "type_definition" => Some(DeclInfo {
            start_line,
            kind: DeclKind::Typedef,
            has_body: false,
        }),
        // Tree-sitter-c parses a top-level bare `struct foo { … };`
        // (no declarator) as a `struct_specifier` followed by a `;`
        // token rather than wrapping them in a `declaration`. Surface
        // the specifier itself.
        "struct_specifier" | "union_specifier" | "enum_specifier" => Some(DeclInfo {
            start_line,
            kind: DeclKind::Aggregate,
            has_body: false,
        }),
        "preproc_def" => {
            // Skip the header-guard's own `#define X` — it's part of
            // the guard envelope, not a public macro.
            if is_header_guard_define(node, source) {
                return None;
            }
            Some(DeclInfo {
                start_line,
                kind: DeclKind::Macro,
                has_body: false,
            })
        }
        "preproc_function_def" => Some(DeclInfo {
            start_line,
            kind: DeclKind::MacroFn,
            has_body: false,
        }),
        "linkage_specification" => Some(DeclInfo {
            start_line,
            kind: DeclKind::Other,
            has_body: false,
        }),
        _ => None,
    }
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

fn is_header_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("h"))
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
    file_depth_factor(file, ctx, is_header_file(file)) * secondary_root_pair_factor(file, ctx)
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
    let Some(stem) = file.file_stem().and_then(|s| s.to_str()) else {
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
    let primary_present = ["c", "h"]
        .iter()
        .any(|ext| ctx.root().join(format!("{repo}.{ext}")).is_file());
    if primary_present {
        SECONDARY_ROOT_PAIR_FACTOR
    } else {
        1.0
    }
}

fn header_banner_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Headers' banner is often the canonical "what is this header"
    // signal; .c file banners are usually license boilerplate.
    let cat = if is_header_file(file) { 0.55 } else { 0.05 };
    mix_signals(cat, 0.4, 0.7, c_depth_factor(file, ctx))
}

fn includes_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let cat = (0.30 * header_cat_factor(file)).min(1.0);
    mix_signals(cat, 0.55, 0.3, c_depth_factor(file, ctx))
}

fn decl_names_value(file: &Path, ctx: &WalkCtx, chunk_index: usize, chunk_count: usize) -> f64 {
    let cat = (0.80 * header_cat_factor(file)).min(1.0);
    mix_signals(cat, 0.6, 0.35, c_depth_factor(file, ctx))
        * names_surface_chunk_factor(chunk_index, chunk_count)
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

fn collect_includes(tree: &Tree, source: &str) -> FileLines {
    let mut lines = Vec::new();
    walk_top_level(tree.root_node(), source, &mut |node| {
        if node.kind() == "preproc_include" {
            extend_span(&mut lines, node, source);
        }
    });
    FileLines::new(dedup_sorted(lines))
}

fn collect_decl_names_from(decls: &[(Node, DeclInfo)]) -> FileLines {
    let starts: std::collections::HashSet<usize> =
        decls.iter().map(|(_, i)| i.start_line).collect();
    collect_decl_names_from_with_global_starts(decls, &starts)
}

fn collect_decl_names_from_with_global_starts(
    decls: &[(Node, DeclInfo)],
    all_starts: &std::collections::HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    // Don't drop an ellipsis on the row of any decl (this chunk or
    // another). Single-line decls on adjacent lines (`#define` runs)
    // would otherwise claim the next decl's start row as a truncation
    // marker and trip the scheduler's non-ancestor overlap guard once
    // the chunks are scheduled into the same render tree.
    for (_, info) in decls {
        full.push(info.start_line);
        if !all_starts.contains(&(info.start_line + 1)) {
            ellipses.push(info.start_line + 1);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Lines for the decl's signature/header. For function definitions, the
/// signature with a body-elision marker (only when the body has interior
/// rows to elide). For prototypes / typedefs / variables / `#define`s,
/// the whole statement. For struct / union / enum at top level, the
/// whole specifier.
fn collect_decl(node: Node, info: &DeclInfo, source: &str) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let start_row = node.start_position().row;
    match info.kind {
        DeclKind::FunctionDef => {
            let sig_end = signature_end_row(node);
            push_rows(&mut full, start_row, sig_end);
            // A single-line `void foo() { ... }` has body.start_row ==
            // body.end_row; emitting an ellipsis there would land on the
            // next decl's start line and trip non-ancestor overlap.
            if let Some(body) = node.child_by_field_name("body") {
                let bs = body.start_position().row;
                let be = body.end_position().row;
                if be > bs + 1 {
                    ellipses.push(sig_end + 2);
                }
            }
        }
        DeclKind::FunctionDecl
        | DeclKind::Variable
        | DeclKind::Typedef
        | DeclKind::Aggregate
        | DeclKind::Macro
        | DeclKind::MacroFn
        | DeclKind::Other => {
            extend_span(&mut full, node, source);
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Body interior of a function definition: rows strictly between the
/// `compound_statement`'s `{` and `}`, with blank source rows skipped.
/// Returns empty when the body has no interior to render (single-line
/// body or all-blank interior).
fn collect_decl_body(node: Node, src_lines: &[&str]) -> FileLines {
    let Some(body) = node.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let s = body.start_position().row;
    let e = body.end_position().row;
    if e <= s + 1 {
        return FileLines::new(Vec::new());
    }
    let mut out = Vec::new();
    for row in (s + 1)..e {
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(row + 1);
        }
    }
    FileLines::new(out)
}

/// Doc comment(s) immediately above a decl. A run of consecutive
/// `comment` nodes touching the decl (with no blank-line gap between
/// them) is treated as the doc — matches both `/** */` blocks above one
/// decl and `// Serverbound packets`-style group headers above a
/// prototype block.
fn collect_decl_doc(node: Node, source: &str) -> FileLines {
    let mut out = Vec::new();
    let mut cur = node.prev_sibling();
    let mut next_start = node.start_position().row;
    while let Some(prev) = cur {
        if prev.kind() != "comment" {
            break;
        }
        // Blank line between this comment and what it sits above means
        // it's not a doc comment for the decl.
        if next_start.saturating_sub(prev.end_position().row) > 1 {
            break;
        }
        extend_span(&mut out, prev, source);
        next_start = prev.start_position().row;
        cur = prev.prev_sibling();
    }
    FileLines::new(dedup_sorted(out))
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
            decls.iter().map(|(_, d)| *d).collect::<Vec<_>>()
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
}
