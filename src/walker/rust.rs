//! Per-file Rust batches: module-level `//!` doc, top-level use/mod plumbing,
//! public-item declarations (signature-only for fns; whole item — including
//! body — for structs/enums/traits/macros so fields, variants, and method
//! sigs are surfaced), outer rustdoc above public items, and inherent/trait
//! impl headers + method signatures. Tree-sitter-rust drives item detection.

use std::cell::RefCell;
use std::path::Path;

use tree_sitter::{Node, Parser};

use crate::batch::{BatchDraft, RenderedLine};

use super::lines_for_file;

const VALUE_MOD_DOC: f64 = 700.0;
const VALUE_USE_MOD: f64 = 400.0;
const VALUE_PUB_SIG: f64 = 600.0;
const VALUE_PUB_DOC: f64 = 400.0;

pub fn emit_batches(path: &Path, source: &str) -> Vec<BatchDraft> {
    let Some(tree) = parse(source) else {
        return Vec::new();
    };
    let root = tree.root_node();
    let source_lines: Vec<&str> = source.lines().collect();

    let mut module_doc_lines: Vec<usize> = Vec::new();
    let mut use_mod_lines: Vec<usize> = Vec::new();
    let mut pub_sig_lines: Vec<usize> = Vec::new();
    let mut pub_doc_lines: Vec<usize> = Vec::new();

    let mut cursor = root.walk();
    let mut seen_item = false;
    for child in root.children(&mut cursor) {
        if !seen_item && is_module_doc_comment(&child, source) {
            extend_lines(&mut module_doc_lines, &child, source);
            continue;
        }
        if matches!(child.kind(), "line_comment" | "block_comment") {
            continue;
        }
        seen_item = true;
        handle_item(
            child,
            source,
            &mut use_mod_lines,
            &mut pub_sig_lines,
            &mut pub_doc_lines,
        );
    }

    // Categories are sibling batches; the scheduler forbids non-ancestor
    // line overlap. Resolve any duplicates in priority order.
    let mut claimed: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    let mut take = |raw: Vec<usize>| -> Vec<usize> {
        raw.into_iter().filter(|n| claimed.insert(*n)).collect()
    };
    let module_doc_lines = take(module_doc_lines);
    let pub_sig_lines = take(pub_sig_lines);
    let pub_doc_lines = take(pub_doc_lines);
    let use_mod_lines = take(use_mod_lines);

    [
        (module_doc_lines, VALUE_MOD_DOC),
        (use_mod_lines, VALUE_USE_MOD),
        (pub_sig_lines, VALUE_PUB_SIG),
        (pub_doc_lines, VALUE_PUB_DOC),
    ]
    .into_iter()
    .filter_map(|(lines, value)| line_batch(path, &lines, &source_lines, value))
    .collect()
}

fn parse(source: &str) -> Option<tree_sitter::Tree> {
    thread_local! {
        static PARSER: RefCell<Parser> = RefCell::new({
            let mut p = Parser::new();
            p.set_language(&tree_sitter_rust::LANGUAGE.into())
                .expect("tree-sitter-rust language load");
            p
        });
    }
    PARSER.with(|p| p.borrow_mut().parse(source, None))
}

fn handle_item(
    node: Node,
    source: &str,
    use_mod_lines: &mut Vec<usize>,
    pub_sig_lines: &mut Vec<usize>,
    pub_doc_lines: &mut Vec<usize>,
) {
    match node.kind() {
        "use_declaration" => extend_lines(use_mod_lines, &node, source),
        "mod_item" => {
            // For inline `mod foo { ... }` we want the declaration only —
            // the body's items are handled when their containing file is
            // walked separately (or punted, for inline mods).
            let sig_end = signature_end_row(node);
            push_rows(use_mod_lines, node.start_position().row, sig_end);
        }
        "function_item" | "function_signature_item" => {
            if is_public(node, source) {
                extend_doc_comments_above(node, source, pub_doc_lines);
                let sig_end = signature_end_row(node);
                push_rows(pub_sig_lines, node.start_position().row, sig_end);
            }
        }
        "impl_item" => {
            // Always surface impl headers and method signatures — impl
            // bodies are how a type's API is actually used; even when the
            // impl block itself isn't visibility-marked, its methods are
            // load-bearing.
            push_rows(
                pub_sig_lines,
                node.start_position().row,
                signature_end_row(node),
            );
            if let Some(body) = node.child_by_field_name("body") {
                let mut cursor = body.walk();
                for inner in body.children(&mut cursor) {
                    if matches!(inner.kind(), "function_item" | "function_signature_item") {
                        extend_doc_comments_above(inner, source, pub_doc_lines);
                        let sig_end = signature_end_row(inner);
                        push_rows(pub_sig_lines, inner.start_position().row, sig_end);
                    }
                }
            }
        }
        _ => {
            if is_public(node, source) {
                extend_doc_comments_above(node, source, pub_doc_lines);
                // Whole item (struct/enum/trait/macro/type/const/static) so
                // fields, variants, trait method sigs, and macro arms appear.
                extend_lines(pub_sig_lines, &node, source);
            }
        }
    }
}

fn is_public(node: Node, source: &str) -> bool {
    if node.kind() == "macro_definition" {
        return has_macro_export_attribute(node, source);
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|c| c.kind() == "visibility_modifier")
}

fn has_macro_export_attribute(node: Node, source: &str) -> bool {
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

fn extend_doc_comments_above(node: Node, source: &str, out: &mut Vec<usize>) {
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "line_comment" | "block_comment" if is_outer_doc_comment(prev, source) => {
                extend_lines(out, &prev, source);
                cur = prev.prev_sibling();
            }
            "attribute_item" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
}

/// `///` and `/** */` only — NOT `//!`/`/*!`, which document the enclosing
/// item rather than the next sibling. Inner doc comments are surfaced via
/// the dedicated module-doc batch.
fn is_outer_doc_comment(node: Node, source: &str) -> bool {
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("///") || text.starts_with("/**")
}

fn is_module_doc_comment(node: &Node, source: &str) -> bool {
    if !matches!(node.kind(), "line_comment" | "block_comment") {
        return false;
    }
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("//!") || text.starts_with("/*!")
}

fn signature_end_row(node: Node) -> usize {
    if let Some(body) = node.child_by_field_name("body") {
        body.start_position().row
    } else {
        node.end_position().row
    }
}

/// Push the 1-indexed line numbers for a node, accounting for tree-sitter-
/// rust including a node's trailing newline in its end_position (so a
/// single-line `//!` reports rows = N..=N+1). Real span comes from the
/// source text.
fn extend_lines(out: &mut Vec<usize>, node: &Node, source: &str) {
    let start = node.start_position().row;
    let text = &source[node.start_byte()..node.end_byte()];
    let internal_lines = text.trim_end_matches(['\n', '\r']).split('\n').count();
    let span = internal_lines.max(1) - 1;
    push_rows(out, start, start + span);
}

fn push_rows(out: &mut Vec<usize>, start_row: usize, end_row: usize) {
    for row in start_row..=end_row {
        out.push(row + 1);
    }
}

fn line_batch(
    path: &Path,
    line_numbers: &[usize],
    source_lines: &[&str],
    value: f64,
) -> Option<BatchDraft> {
    let mut deduped: Vec<usize> = line_numbers.to_vec();
    deduped.sort();
    deduped.dedup();
    let lines = deduped.into_iter().filter_map(|n| {
        let text = source_lines.get(n - 1)?;
        if text.trim().is_empty() {
            return None;
        }
        Some((n, RenderedLine::Full(text.to_string())))
    });
    lines_for_file(path, lines, value)
}
