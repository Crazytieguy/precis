//! Rust walker. Per-item pub-declaration batches plus file-scope
//! crate-doc / mod-use / impl-method-groups and cross-file macro surface.
//!
//! Per-file keys:
//! - `CrateDocLede { file }`: `//!` opening paragraph (entrypoints only)
//! - `ModUse { file }`: `use` / `mod` / `pub use` plumbing
//! - `PubItemNames { file }`: every pub item's first line as a surface
//!   listing — a cheap catastrophic-omission hedge when individual item
//!   decls don't all fit
//! - `MethodSigs { file }`: inherent + trait impl headers + method sigs
//!
//! Per-item keys (keyed by start line so each item has a distinct batch):
//! - `PubItem { file, start_line }`: one pub item's declaration (struct
//!   fields / enum variants / trait method sigs / fn signature; no rustdoc)
//! - `PubItemDoc { file, start_line }`: rustdoc above that item,
//!   predecessor = the matching `PubItem`
//!
//! Cross-file keys (scoped by source directory):
//! - `MacroNames { src_dir }`: exported macro name list
//! - `MacroBodies { src_dir }`: full bodies (predecessor: `MacroNames`)
//!
//! Parse trees are cached in [`WalkCtx`]; the same file parsed once powers
//! every Rust batch that touches it.

use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{BatchKey, FsKey, ResolvedBatch, RustKey, ValueSignals};
use crate::content::{BatchContent, Span};
use crate::value::{depth_factor, non_essential_factor};

use super::{
    Candidate, FileLines, WalkCtx, build_file_spans, dedup_sorted, extend_span,
    fs::files_with_extension, push_rows, signature_end_row, single_file_lines_batch,
};

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
        return Vec::new();
    };
    let rust_files = files_with_extension(dir, "rs");
    if rust_files.is_empty() {
        return Vec::new();
    }
    let dir_depth = ctx.depth_from_root(dir);

    let mut out = Vec::new();

    for file in &rust_files {
        let depth = ctx.depth_from_root(file);
        let ep = is_entrypoint_file(file);
        if ep {
            let lede = RustKey::CrateDocLede { file: file.clone() };
            out.push(candidate(
                lede.clone(),
                crate_doc_lede_signals(file, depth),
                40,
            ));
            out.push(
                candidate(
                    RustKey::CrateDocBody { file: file.clone() },
                    crate_doc_body_signals(file, depth),
                    200,
                )
                .with_predecessor(BatchKey::Rust(lede)),
            );
            out.push(candidate(
                RustKey::ModUse { file: file.clone() },
                mod_use_signals(file, depth),
                60,
            ));
            out.push(candidate(
                RustKey::MethodSigs { file: file.clone() },
                method_sigs_signals(file, depth),
                60,
            ));
        }

        // Per-item pub declarations. We need to read the file to find item
        // start lines — but we cache the parse tree in `WalkCtx`, so this
        // work is shared with later `materialize` calls. If a file has zero
        // pub items, no per-item candidates are emitted.
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let items = find_pub_item_starts(&tree, &source);
        if items.is_empty() {
            continue;
        }
        // Intentionally not modeled as a separate batch in current north stars
        // (they rank full bodies only). Reviewers will flag the resulting
        // "struct header + …" rendering as a partial body, which is correct
        // against the frozen NS. Planned: next NS-author pass will rank a
        // PubItemNames-equivalent as its own tier-1 location hint so
        // reviewer grading matches walker emission. Deferred to batch with
        // other NS-side changes before the next fixture round.
        let names_key = RustKey::PubItemNames { file: file.clone() };
        out.push(candidate(
            names_key.clone(),
            pub_item_names_signals(file, depth),
            items.len() * 8,
        ));
        for item in &items {
            let key = RustKey::PubItem {
                file: file.clone(),
                start_line: item.start_line,
            };
            // PubItemNames is the parent surface listing; PubItem refines
            // that file's header lines with full content. PubItemDoc
            // refines PubItem with rustdoc.
            out.push(
                candidate(
                    key.clone(),
                    pub_item_signals(file, depth, item.kind),
                    item.estimated_cost(),
                )
                .with_predecessor(BatchKey::Rust(names_key.clone())),
            );
            out.push(
                candidate(
                    RustKey::PubItemDoc {
                        file: file.clone(),
                        start_line: item.start_line,
                    },
                    pub_item_doc_signals(file, depth, item.kind),
                    60,
                )
                .with_predecessor(BatchKey::Rust(key)),
            );
        }
    }

    // Cross-file macro batches, scoped to `dir` (non-recursive).
    let macro_names = RustKey::MacroNames {
        src_dir: dir.clone(),
    };
    out.push(candidate(
        macro_names.clone(),
        macro_names_signals(dir_depth),
        20,
    ));
    out.push(
        candidate(
            RustKey::MacroBodies {
                src_dir: dir.clone(),
            },
            macro_bodies_signals(dir_depth),
            60,
        )
        .with_predecessor(BatchKey::Rust(macro_names)),
    );

    out
}

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Rust(rk) = key else {
        return None;
    };
    match rk {
        RustKey::CrateDocLede { file } => mat_per_file(
            file,
            collect_module_doc_lede,
            crate_doc_lede_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        RustKey::CrateDocBody { file } => mat_per_file(
            file,
            collect_module_doc_body,
            crate_doc_body_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        RustKey::ModUse { file } => mat_per_file(
            file,
            collect_mod_use,
            mod_use_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        RustKey::PubItemNames { file } => mat_per_file(
            file,
            collect_pub_item_names,
            pub_item_names_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        RustKey::PubItem { file, start_line } => {
            let (source, tree) = parse_rust(ctx, file)?;
            let item = find_item_at(&tree, &source, *start_line)?;
            let lines = collect_pub_item(&tree, &source, *start_line);
            single_file_lines_batch(
                file,
                &source,
                lines,
                pub_item_signals(file, ctx.depth_from_root(file), item.kind),
            )
        }
        RustKey::PubItemDoc { file, start_line } => {
            let (source, tree) = parse_rust(ctx, file)?;
            let item = find_item_at(&tree, &source, *start_line)?;
            let lines = collect_pub_item_doc(&tree, &source, *start_line);
            single_file_lines_batch(
                file,
                &source,
                lines,
                pub_item_doc_signals(file, ctx.depth_from_root(file), item.kind),
            )
        }
        RustKey::MethodSigs { file } => mat_per_file(
            file,
            collect_method_sigs,
            method_sigs_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        RustKey::MacroNames { src_dir } => mat_cross_file(
            src_dir,
            collect_macro_name_lines,
            macro_names_signals(ctx.depth_from_root(src_dir)),
            ctx,
        ),
        RustKey::MacroBodies { src_dir } => mat_cross_file(
            src_dir,
            collect_macro_bodies,
            macro_bodies_signals(ctx.depth_from_root(src_dir)),
            ctx,
        ),
    }
}

// --- candidate + signal helpers ---

fn candidate(rk: RustKey, signals: ValueSignals, cost_hint: usize) -> Candidate<BatchKey> {
    Candidate::new(rk.into(), signals, cost_hint)
}

/// Kind of a top-level pub item, used to weight its batch. Traits are the
/// load-bearing abstraction every backend implements; enums and structs
/// carry the data-model; free fns are the call surface. First-pass
/// ordering — calibrate against the north stars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemKind {
    Trait,
    Enum,
    Struct,
    Union,
    Fn,
    TypeAlias,
    Const,
    Static,
}

impl ItemKind {
    fn kind_weight(self) -> f64 {
        match self {
            ItemKind::Trait => 1.15,
            ItemKind::Enum => 1.1,
            ItemKind::Struct | ItemKind::Union => 1.0,
            ItemKind::Fn => 0.95,
            ItemKind::TypeAlias | ItemKind::Const | ItemKind::Static => 0.85,
        }
    }
}

#[derive(Debug, Clone)]
struct PubItemInfo {
    start_line: usize,
    kind: ItemKind,
    /// Rough upper bound of the rendered cost of just this item's decl —
    /// lines × avg 6 tokens, with a floor of 40 for a one-liner decl.
    line_span: usize,
}

impl PubItemInfo {
    fn estimated_cost(&self) -> usize {
        (self.line_span * 6).max(40)
    }
}

fn item_kind_of(node: Node) -> Option<ItemKind> {
    Some(match node.kind() {
        "trait_item" => ItemKind::Trait,
        "enum_item" => ItemKind::Enum,
        "struct_item" => ItemKind::Struct,
        "union_item" => ItemKind::Union,
        "function_item" | "function_signature_item" => ItemKind::Fn,
        "type_item" => ItemKind::TypeAlias,
        "const_item" => ItemKind::Const,
        "static_item" => ItemKind::Static,
        _ => return None,
    })
}

fn find_pub_item_starts(tree: &Tree, source: &str) -> Vec<PubItemInfo> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            continue; // Handled by MacroNames/MacroBodies.
        }
        if !is_public(child) {
            continue;
        }
        let Some(kind) = item_kind_of(child) else {
            continue;
        };
        let start_line = child.start_position().row + 1;
        let end_line = item_end_line(child, source);
        out.push(PubItemInfo {
            start_line,
            kind,
            line_span: end_line.saturating_sub(start_line) + 1,
        });
    }
    out
}

fn find_item_at(tree: &Tree, source: &str, start_line: usize) -> Option<PubItemInfo> {
    find_pub_item_starts(tree, source)
        .into_iter()
        .find(|i| i.start_line == start_line)
}

fn item_end_line(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    let internal = text.trim_end_matches(['\n', '\r']).split('\n').count();
    node.start_position().row + internal.max(1)
}

/// Files whose filename signals "crate entrypoint / main module surface".
fn is_entrypoint_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| matches!(n, "lib.rs" | "main.rs" | "mod.rs"))
}

fn entrypoint_boost(path: &Path) -> f64 {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    match name {
        "lib.rs" | "main.rs" => 1.4,
        "mod.rs" => 1.15,
        _ => 1.0,
    }
}

/// Depth factor for a file. Entrypoints are pinned to depth 1 so
/// `src/lib.rs` isn't penalized relative to root-depth content. All files
/// multiply by a non-essential-directory factor.
fn file_depth_factor(path: &Path, depth: usize) -> f64 {
    let raw = if is_entrypoint_file(path) {
        depth_factor(depth.min(1))
    } else {
        depth_factor(depth)
    };
    raw * non_essential_factor(path)
}

fn crate_doc_lede_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.8 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.5,
        zero_tool_call_understanding: 0.9,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn crate_doc_body_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.35 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.75,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn mod_use_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.3 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.55,
        zero_tool_call_understanding: 0.3,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn pub_item_names_signals(file: &Path, depth: usize) -> ValueSignals {
    // Cheap surface listing — catastrophic-omission hedge. Ranks high
    // because missing it means the agent doesn't know items exist.
    ValueSignals {
        catastrophic_omission: (0.8 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.35,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn pub_item_signals(file: &Path, depth: usize, kind: ItemKind) -> ValueSignals {
    let k = kind.kind_weight();
    ValueSignals {
        catastrophic_omission: (0.85 * k * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.85 * k,
        zero_tool_call_understanding: 0.55,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn pub_item_doc_signals(file: &Path, depth: usize, kind: ItemKind) -> ValueSignals {
    let k = kind.kind_weight();
    ValueSignals {
        catastrophic_omission: (0.4 * k * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.6 * k,
        zero_tool_call_understanding: 0.8,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn method_sigs_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.5 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.8,
        zero_tool_call_understanding: 0.4,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn macro_names_signals(depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.75,
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.4,
        depth_factor: depth_factor(depth),
    }
}

fn macro_bodies_signals(depth: usize) -> ValueSignals {
    // For macro-heavy crates (anyhow, log), the `#[macro_export]` bodies
    // are the crate's public API — on par with `PubItem`. Predecessor
    // edge to MacroNames orders them.
    ValueSignals {
        catastrophic_omission: 0.9,
        follow_up_minimization: 0.9,
        zero_tool_call_understanding: 0.6,
        depth_factor: depth_factor(depth),
    }
}

// --- parser ---

fn parse_rust(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_rust::LANGUAGE.into())
}

// --- shared materializer shapes ---

fn mat_per_file<F>(
    file: &Path,
    collect: F,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch>
where
    F: Fn(&Tree, &str) -> FileLines,
{
    let (source, tree) = parse_rust(ctx, file)?;
    let lines = collect(&tree, &source);
    single_file_lines_batch(file, &source, lines, signals)
}

fn mat_cross_file<F>(
    src_dir: &Path,
    collect: F,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch>
where
    F: Fn(&Tree, &str) -> FileLines,
{
    let rust_files = files_with_extension(src_dir, "rs");
    if rust_files.is_empty() {
        return None;
    }
    let mut all_spans: Vec<Span> = Vec::new();
    for file in &rust_files {
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let lines = collect(&tree, &source);
        all_spans.extend(build_file_spans(file, &source, lines));
    }
    if all_spans.is_empty() {
        return None;
    }
    Some(ResolvedBatch {
        content: BatchContent::Lines { spans: all_spans },
        signals,
    })
}

// --- AST collectors ---

fn collect_module_doc_lede(tree: &Tree, source: &str) -> FileLines {
    FileLines::new(collect_module_doc_lines(tree, source, DocSection::Lede))
}

fn collect_module_doc_body(tree: &Tree, source: &str) -> FileLines {
    FileLines::new(collect_module_doc_lines(tree, source, DocSection::Body))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocSection {
    /// First paragraph of `//!` only.
    Lede,
    /// Everything after the first paragraph.
    Body,
}

/// Collect line numbers belonging to the crate-`//!` block, split at the
/// first Markdown heading (`//! #`, `//! ##`, …). Regular license-header
/// `// comments` above the `//!` run are skipped. The lede is everything
/// from the first `//!` up to (but not including) the first heading line;
/// the body is from the heading onwards. If no heading is present, the
/// whole block is the lede.
fn collect_module_doc_lines(tree: &Tree, source: &str, section: DocSection) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut all: Vec<usize> = Vec::new();
    for child in root.children(&mut cursor) {
        if matches!(child.kind(), "line_comment" | "block_comment") {
            if is_module_doc_comment(child, source) {
                extend_span(&mut all, child, source);
            }
            continue;
        }
        break;
    }
    if all.is_empty() {
        return Vec::new();
    }
    let src_lines: Vec<&str> = source.lines().collect();
    let heading_pos = all.iter().position(|&n| {
        src_lines.get(n - 1).is_some_and(|t| {
            t.trim_start()
                .trim_start_matches("//!")
                .trim_start()
                .starts_with('#')
        })
    });
    match (section, heading_pos) {
        (DocSection::Lede, Some(idx)) => all[..idx].to_vec(),
        (DocSection::Body, Some(idx)) => all[idx..].to_vec(),
        (DocSection::Lede, None) => all,
        (DocSection::Body, None) => Vec::new(),
    }
}

fn collect_mod_use(tree: &Tree, _source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "use_declaration" => push_rows(
                &mut full,
                child.start_position().row,
                signature_end_row(child),
            ),
            "mod_item" => {
                let sig_end = signature_end_row(child);
                push_rows(&mut full, child.start_position().row, sig_end);
                if child.child_by_field_name("body").is_some() {
                    ellipses.push(sig_end + 2);
                }
            }
            _ => {}
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Header lines for every top-level pub item (name + first line only, with
/// an ellipsis marker where the body would be). Surface listing — see
/// `PubItemNames`.
fn collect_pub_item_names(tree: &Tree, source: &str) -> FileLines {
    let items = find_pub_item_starts(tree, source);
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for item in &items {
        full.push(item.start_line);
        ellipses.push(item.start_line + 1);
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Lines for a single pub item's decl at `start_line`. For struct/enum/
/// trait/union: whole item (fields/variants/method sigs). For fn: signature
/// with body-elision marker. No outer rustdoc — that's `PubItemDoc`.
fn collect_pub_item(tree: &Tree, source: &str, start_line: usize) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        if !is_public(child) {
            return FileLines::new(Vec::new());
        }
        let mut full = Vec::new();
        let mut ellipses = Vec::new();
        match child.kind() {
            "function_item" | "function_signature_item" => {
                let sig_end = signature_end_row(child);
                push_rows(&mut full, child.start_position().row, sig_end);
                if child.child_by_field_name("body").is_some() {
                    ellipses.push(sig_end + 2);
                }
            }
            "struct_item" | "enum_item" | "trait_item" | "union_item" | "type_item"
            | "const_item" | "static_item" => {
                extend_span(&mut full, child, source);
            }
            _ => {}
        }
        return FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses));
    }
    FileLines::new(Vec::new())
}

/// Outer rustdoc (`///` / `/** */`) immediately preceding the item at
/// `start_line`. Returns empty when the item has no outer doc.
fn collect_pub_item_doc(tree: &Tree, source: &str, start_line: usize) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        let mut out = Vec::new();
        collect_outer_docs_above(child, source, &mut out);
        return FileLines::new(dedup_sorted(out));
    }
    FileLines::new(Vec::new())
}

fn collect_method_sigs(tree: &Tree, _source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "impl_item" {
            continue;
        }
        let sig_end = signature_end_row(child);
        push_rows(&mut full, child.start_position().row, sig_end);
        let Some(body) = child.child_by_field_name("body") else {
            continue;
        };
        let mut body_cursor = body.walk();
        for inner in body.children(&mut body_cursor) {
            if matches!(inner.kind(), "function_item" | "function_signature_item") {
                let inner_end = signature_end_row(inner);
                push_rows(&mut full, inner.start_position().row, inner_end);
                if inner.child_by_field_name("body").is_some() {
                    ellipses.push(inner_end + 2);
                }
            }
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

fn collect_macro_name_lines(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            let name_row = child.start_position().row;
            push_rows(&mut out, name_row, name_row);
            ellipses.push(name_row + 2);
        }
    }
    FileLines::new(out).with_ellipses(ellipses)
}

fn collect_macro_bodies(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition"
            && has_macro_export(child, source)
            && !is_underscore_private(child, source)
        {
            extend_span(&mut out, child, source);
        }
    }
    FileLines::new(dedup_sorted(out))
}

/// `__` prefix by convention marks an internal dispatcher macro.
fn is_underscore_private(node: Node, source: &str) -> bool {
    if let Some(name) = node.child_by_field_name("name") {
        let text = &source[name.start_byte()..name.end_byte()];
        return text.starts_with("__");
    }
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|c| {
        c.kind() == "identifier" && {
            let text = &source[c.start_byte()..c.end_byte()];
            text.starts_with("__")
        }
    })
}

// --- AST predicates ---

fn is_public(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|c| c.kind() == "visibility_modifier")
}

fn has_macro_export(node: Node, source: &str) -> bool {
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "attribute_item" => {
                let text = &source[prev.start_byte()..prev.end_byte()];
                if text.contains("macro_export") {
                    return true;
                }
                cur = prev.prev_sibling();
            }
            "line_comment" | "block_comment" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
    false
}

fn is_module_doc_comment(node: Node, source: &str) -> bool {
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("//!") || text.starts_with("/*!")
}

fn is_outer_doc_comment(node: Node, source: &str) -> bool {
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("///") || text.starts_with("/**")
}

fn collect_outer_docs_above(node: Node, source: &str, out: &mut Vec<usize>) {
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "line_comment" | "block_comment" if is_outer_doc_comment(prev, source) => {
                extend_span(out, prev, source);
                cur = prev.prev_sibling();
            }
            "attribute_item" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
}
