//! Rust content walker. First-pass Tier 1 scope: per file, emit four batches —
//! module-level doc, use/mod plumbing, public-item signatures (truncated at
//! the first `{`), and rustdoc lines above public items. Uses tree-sitter for
//! item detection.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use tree_sitter::{Node, Parser};

use crate::batch::{BatchContent, BatchDraft, RenderedLine};

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
        match child.kind() {
            "line_comment" | "block_comment" => {
                // Non-leading comment — these are doc comments above public
                // items, collected via extend_doc_comments_above when we hit
                // the item itself. Skip here.
            }
            _ => {
                seen_item = true;
                handle_item(
                    child,
                    source,
                    &mut use_mod_lines,
                    &mut pub_sig_lines,
                    &mut pub_doc_lines,
                );
            }
        }
    }

    // Categories are emitted as sibling batches under the folder listing —
    // they can't overlap on the same line (the scheduler's overlap check
    // would fire), so resolve any duplicates here by giving precedence to
    // the order below: module_doc, pub_sig, pub_doc, use_mod.
    let mut claimed: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    let mut take_lines = |raw: Vec<usize>| -> Vec<usize> {
        raw.into_iter().filter(|n| claimed.insert(*n)).collect()
    };
    let module_doc_lines = take_lines(module_doc_lines);
    let pub_sig_lines = take_lines(pub_sig_lines);
    let pub_doc_lines = take_lines(pub_doc_lines);
    let use_mod_lines = take_lines(use_mod_lines);

    let mut drafts = Vec::new();
    if !module_doc_lines.is_empty() {
        drafts.push(line_batch(
            path,
            &module_doc_lines,
            &source_lines,
            VALUE_MOD_DOC,
            false,
        ));
    }
    if !use_mod_lines.is_empty() {
        drafts.push(line_batch(
            path,
            &use_mod_lines,
            &source_lines,
            VALUE_USE_MOD,
            false,
        ));
    }
    if !pub_sig_lines.is_empty() {
        drafts.push(line_batch(
            path,
            &pub_sig_lines,
            &source_lines,
            VALUE_PUB_SIG,
            true,
        ));
    }
    if !pub_doc_lines.is_empty() {
        drafts.push(line_batch(
            path,
            &pub_doc_lines,
            &source_lines,
            VALUE_PUB_DOC,
            false,
        ));
    }
    drafts
}

fn parse(source: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .ok()?;
    parser.parse(source, None)
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
            // For `mod foo;` extend_lines is a single line; for inline
            // `mod foo { ... }` we want the declaration only — the inner
            // items are handled when the file (this one or a child mod's
            // file) is itself walked.
            let sig_end = signature_end_row(node);
            for row in node.start_position().row..=sig_end {
                use_mod_lines.push(row + 1);
            }
        }
        _ => {
            if is_public(node, source) {
                extend_doc_comments_above(node, source, pub_doc_lines);
                let sig_end = signature_end_row(node);
                for row in node.start_position().row..=sig_end {
                    pub_sig_lines.push(row + 1);
                }
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
    let mut collected: Vec<usize> = Vec::new();
    while let Some(prev) = cur {
        match prev.kind() {
            "line_comment" | "block_comment" if is_outer_doc_comment(prev, source) => {
                extend_lines(&mut collected, &prev, source);
                cur = prev.prev_sibling();
            }
            "attribute_item" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
    collected.sort();
    out.extend(collected);
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

/// Push the 1-indexed line numbers for a node, accounting for the fact that
/// tree-sitter-rust includes a node's trailing newline in its end_position —
/// which makes a single-line `//!` comment report rows = N..=N+1. We compute
/// the real line span from the source text instead.
fn extend_lines(out: &mut Vec<usize>, node: &Node, source: &str) {
    let start = node.start_position().row;
    let text = &source[node.start_byte()..node.end_byte()];
    let internal_lines = text.trim_end_matches(['\n', '\r']).split('\n').count();
    let span = internal_lines.max(1) - 1;
    for row in start..=start + span {
        out.push(row + 1);
    }
}

fn line_batch(
    path: &Path,
    line_numbers: &[usize],
    source_lines: &[&str],
    value: f64,
    truncate_at_brace: bool,
) -> BatchDraft {
    let mut deduped: Vec<usize> = line_numbers.to_vec();
    deduped.sort();
    deduped.dedup();
    let mut lines: BTreeMap<usize, RenderedLine> = BTreeMap::new();
    for n in deduped {
        let Some(text) = source_lines.get(n - 1) else {
            continue;
        };
        if text.trim().is_empty() {
            continue;
        }
        let rendered = if truncate_at_brace && let Some(idx) = text.find('{') {
            let prefix = text[..idx].trim_end().to_string();
            if prefix.is_empty() {
                continue;
            }
            RenderedLine::Truncated(prefix)
        } else {
            RenderedLine::Full(text.to_string())
        };
        lines.insert(n, rendered);
    }
    let mut file_map: BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>> = BTreeMap::new();
    file_map.insert(path.to_path_buf(), lines);
    BatchDraft {
        content: BatchContent::Lines(file_map),
        value,
    }
}
