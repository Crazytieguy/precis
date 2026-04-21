//! Rust walker. Per-file + cross-file batches driven by tree-sitter-rust.
//!
//! Per-file keys:
//! - `CrateDocLede { file }`: `//!` opening paragraph (entrypoints only)
//! - `ModUse { file }`: `use` / `mod` / `pub use` plumbing
//! - `PubDecls { file }`: bare public-item declarations
//! - `PubDocs { file }`: rustdoc above each pub item (predecessor: `PubDecls`)
//! - `MethodSigs { file }`: inherent + trait impl headers + method sigs
//!
//! Cross-file keys (scoped by source directory):
//! - `MacroNames { src_dir }`: exported macro name list
//! - `MacroBodies { src_dir }`: full bodies (predecessor: `MacroNames`)
//!
//! Parse trees are cached in [`WalkCtx`]; the same file parsed once powers
//! every Rust batch that touches it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{BatchContent, BatchKey, FsKey, RenderedLine, ResolvedBatch, RustKey, ValueSignals};
use crate::value::depth_factor;

use super::{
    Candidate, WalkCtx, fs::files_with_extension, lines_map_from, single_file_lines_batch,
};

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate> {
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
        if is_entrypoint_file(file) {
            out.push(candidate(
                RustKey::CrateDocLede { file: file.clone() },
                crate_doc_lede_signals(file, depth),
                40,
            ));
        }
        out.push(candidate(
            RustKey::ModUse { file: file.clone() },
            mod_use_signals(file, depth),
            60,
        ));
        let pub_decls = RustKey::PubDecls { file: file.clone() };
        out.push(candidate(
            pub_decls.clone(),
            pub_decls_signals(file, depth),
            80,
        ));
        out.push(
            candidate(
                RustKey::PubDocs { file: file.clone() },
                pub_docs_signals(file, depth),
                60,
            )
            .with_predecessor(BatchKey::Rust(pub_decls)),
        );
        out.push(candidate(
            RustKey::MethodSigs { file: file.clone() },
            method_sigs_signals(file, depth),
            60,
        ));
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
        RustKey::ModUse { file } => mat_per_file(
            file,
            collect_mod_use,
            mod_use_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        RustKey::PubDecls { file } => mat_per_file(
            file,
            collect_pub_decls,
            pub_decls_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        RustKey::PubDocs { file } => mat_per_file(
            file,
            collect_pub_docs,
            pub_docs_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
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

fn candidate(rk: RustKey, signals: ValueSignals, cost_hint: usize) -> Candidate {
    Candidate::new(rk.into(), signals, cost_hint)
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

/// Depth factor for a file, with entrypoints pinned to depth 1 so that
/// `src/lib.rs` (actual depth 2) isn't penalized relative to root-depth
/// content. Non-entrypoints use the plain `value::depth_factor`.
fn file_depth_factor(path: &Path, depth: usize) -> f64 {
    if is_entrypoint_file(path) {
        depth_factor(depth.min(1))
    } else {
        depth_factor(depth)
    }
}

fn crate_doc_lede_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.7 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.5,
        zero_tool_call_understanding: 0.85,
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

fn pub_decls_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.9 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.9,
        zero_tool_call_understanding: 0.5,
        depth_factor: file_depth_factor(file, depth),
    }
}

fn pub_docs_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.25 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.55,
        zero_tool_call_understanding: 0.7,
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
    ValueSignals {
        catastrophic_omission: 0.25,
        follow_up_minimization: 0.55,
        zero_tool_call_understanding: 0.4,
        depth_factor: depth_factor(depth),
    }
}

// --- parser ---

fn parse_rust(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_rust::LANGUAGE.into())
}

// --- shared materializer shapes ---

/// Materialize a per-file batch: parse, run the collector, build a
/// single-file Lines map.
fn mat_per_file<F>(
    file: &Path,
    collect: F,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch>
where
    F: Fn(&Tree, &str) -> Vec<usize>,
{
    let (source, tree) = parse_rust(ctx, file)?;
    let lines = collect(&tree, &source);
    single_file_lines_batch(file, &source, lines, signals)
}

/// Materialize a cross-file batch: parse each `.rs` file in `src_dir`
/// (non-recursive), run the collector per file, join results into a
/// multi-file Lines map.
fn mat_cross_file<F>(
    src_dir: &Path,
    collect: F,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch>
where
    F: Fn(&Tree, &str) -> Vec<usize>,
{
    let rust_files = files_with_extension(src_dir, "rs");
    if rust_files.is_empty() {
        return None;
    }
    let mut file_map: BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>> = BTreeMap::new();
    for file in &rust_files {
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let lines = lines_map_from(&source, collect(&tree, &source));
        if !lines.is_empty() {
            file_map.insert(file.clone(), lines);
        }
    }
    if file_map.is_empty() {
        return None;
    }
    Some(ResolvedBatch {
        content: BatchContent::Lines(file_map),
        signals,
    })
}

// --- AST collectors ---

fn collect_module_doc_lede(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if matches!(child.kind(), "line_comment" | "block_comment")
            && is_module_doc_comment(child, source)
        {
            extend_span(&mut out, child, source);
            continue;
        }
        if !matches!(child.kind(), "line_comment" | "block_comment") {
            break;
        }
    }
    // Lede = first paragraph. Truncate at the first blank line.
    let src_lines: Vec<&str> = source.lines().collect();
    if let Some(blank_idx) = out
        .iter()
        .position(|&n| src_lines.get(n - 1).is_some_and(|t| t.trim().is_empty()))
    {
        out.truncate(blank_idx);
    }
    out
}

fn collect_mod_use(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "use_declaration" => extend_span(&mut out, child, source),
            "mod_item" => {
                let sig_end = signature_end_row(child);
                push_rows(&mut out, child.start_position().row, sig_end);
            }
            _ => {}
        }
    }
    dedup_sorted(out)
}

fn collect_pub_decls(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            continue; // MacroNames/MacroBodies handle these.
        }
        if !is_public(child) {
            continue;
        }
        match child.kind() {
            "function_item" | "function_signature_item" => {
                let sig_end = signature_end_row(child);
                push_rows(&mut out, child.start_position().row, sig_end);
            }
            "struct_item" | "enum_item" | "trait_item" | "type_item" | "const_item"
            | "static_item" | "union_item" => {
                extend_span(&mut out, child, source);
            }
            _ => {}
        }
    }
    dedup_sorted(out)
}

fn collect_pub_docs(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if !is_public(child)
            || !matches!(
                child.kind(),
                "function_item"
                    | "function_signature_item"
                    | "struct_item"
                    | "enum_item"
                    | "trait_item"
                    | "type_item"
                    | "const_item"
                    | "static_item"
                    | "union_item"
            )
        {
            continue;
        }
        collect_outer_docs_above(child, source, &mut out);
    }
    dedup_sorted(out)
}

fn collect_method_sigs(tree: &Tree, _source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "impl_item" {
            continue;
        }
        let sig_end = signature_end_row(child);
        push_rows(&mut out, child.start_position().row, sig_end);
        let Some(body) = child.child_by_field_name("body") else {
            continue;
        };
        let mut body_cursor = body.walk();
        for inner in body.children(&mut body_cursor) {
            if matches!(inner.kind(), "function_item" | "function_signature_item") {
                let inner_end = signature_end_row(inner);
                push_rows(&mut out, inner.start_position().row, inner_end);
            }
        }
    }
    dedup_sorted(out)
}

fn collect_macro_name_lines(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            let name_row = child.start_position().row;
            push_rows(&mut out, name_row, name_row);
        }
    }
    out
}

fn collect_macro_bodies(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            extend_span(&mut out, child, source);
        }
    }
    dedup_sorted(out)
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

// --- line span helpers ---

/// Push 1-indexed line numbers for `node`. tree-sitter-rust's
/// `end_position().row` reports the row of the *trailing newline*, so for a
/// single-line node it would naively report 2 rows; instead count newlines
/// in the actual source slice.
fn extend_span(out: &mut Vec<usize>, node: Node, source: &str) {
    let start = node.start_position().row;
    let text = &source[node.start_byte()..node.end_byte()];
    let internal_lines = text.trim_end_matches(['\n', '\r']).split('\n').count();
    let span = internal_lines.max(1) - 1;
    push_rows(out, start, start + span);
}

fn signature_end_row(node: Node) -> usize {
    node.child_by_field_name("body")
        .map(|b| b.start_position().row)
        .unwrap_or_else(|| node.end_position().row)
}

fn push_rows(out: &mut Vec<usize>, start_row: usize, end_row: usize) {
    for row in start_row..=end_row {
        out.push(row + 1);
    }
}

fn dedup_sorted(mut v: Vec<usize>) -> Vec<usize> {
    v.sort();
    v.dedup();
    v
}

