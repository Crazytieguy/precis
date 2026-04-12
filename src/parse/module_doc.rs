use crate::Lang;

use super::{ExtractedItem, ItemKind};

pub(super) fn detect_module_doc<'t>(
    root: tree_sitter::Node<'t>,
    source: &str,
    lang: Lang,
) -> Option<ExtractedItem<'t>> {
    let (start_row, end_row, node) = match lang {
        Lang::Rust => detect_rust(root, source)?,
        Lang::Python => detect_python(root, source)?,
        Lang::Go => detect_go(root, source)?,
        Lang::Java => detect_java(root, source)?,
        _ => return None,
    };

    let name = source
        .lines()
        .nth(start_row)
        .unwrap_or("")
        .trim()
        .to_string();

    Some(ExtractedItem {
        node,
        kind: ItemKind::ModuleDoc,
        name,
        is_public: true,
        is_first_party: false,
        is_trait_impl: false,
        is_reexport: false,
        is_documented: false,
        start_line: start_row,
        end_line: end_row + 1,
    })
}

fn detect_rust<'t>(
    root: tree_sitter::Node<'t>,
    source: &str,
) -> Option<(usize, usize, tree_sitter::Node<'t>)> {
    let mut first_node = None;
    let mut start_row = None;
    let mut end_row = None;

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        let text = match child.utf8_text(source.as_bytes()) {
            Ok(t) => t,
            Err(_) => break,
        };

        match child.kind() {
            "line_comment" if text.starts_with("//!") => {
                if start_row.is_none() {
                    start_row = Some(child.start_position().row);
                    first_node = Some(child);
                }
                end_row = Some(child.start_position().row);
            }
            "block_comment" if text.starts_with("/*!") => {
                if start_row.is_none() {
                    start_row = Some(child.start_position().row);
                    first_node = Some(child);
                }
                end_row = Some(child.end_position().row);
                break;
            }
            "attribute_item" | "inner_attribute_item" | "line_comment" => continue,
            _ => break,
        }
    }

    // Return the first node as the representative node for the module doc
    Some((start_row?, end_row?, first_node?))
}

fn detect_python<'t>(
    root: tree_sitter::Node<'t>,
    source: &str,
) -> Option<(usize, usize, tree_sitter::Node<'t>)> {
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "comment" {
            continue;
        }
        if child.kind() == "future_import_statement" {
            continue;
        }
        if child.kind() == "expression_statement" {
            let first_child = child.child(0)?;
            if matches!(first_child.kind(), "string" | "concatenated_string") {
                let text = first_child.utf8_text(source.as_bytes()).ok()?;
                if text.starts_with("\"\"\"") || text.starts_with("'''") {
                    return Some((
                        child.start_position().row,
                        child.end_position().row,
                        child,
                    ));
                }
            }
        }
        break;
    }
    None
}

fn detect_go<'t>(
    root: tree_sitter::Node<'t>,
    source: &str,
) -> Option<(usize, usize, tree_sitter::Node<'t>)> {
    let mut cursor = root.walk();
    let pkg_node = root
        .children(&mut cursor)
        .find(|child| child.kind() == "package_clause")?;
    let pkg_row = pkg_node.start_position().row;

    let mut first_doc: Option<tree_sitter::Node> = None;
    let mut last_doc: Option<tree_sitter::Node> = None;
    let mut sibling = pkg_node.prev_named_sibling();
    while let Some(node) = sibling {
        if node.kind() != "comment" {
            break;
        }
        let text = match node.utf8_text(source.as_bytes()) {
            Ok(t) => t.trim(),
            Err(_) => break,
        };
        if text.starts_with("//go:build") || text.starts_with("// +build") {
            sibling = node.prev_named_sibling();
            continue;
        }
        let next_row = last_doc
            .map(|n: tree_sitter::Node| n.start_position().row)
            .unwrap_or(pkg_row);
        let this_end_row = node.end_position().row;
        if next_row.saturating_sub(this_end_row) > 1 {
            break;
        }
        if last_doc.is_none() {
            last_doc = Some(node);
        }
        first_doc = Some(node);
        sibling = node.prev_named_sibling();
    }

    let first = first_doc?;
    let last = last_doc?;

    let end_row = last.end_position().row;
    let end_row = if end_row > last.start_position().row && last.end_position().column == 0 {
        last.start_position().row
    } else {
        end_row
    };

    Some((first.start_position().row, end_row, first))
}

fn detect_java<'t>(
    root: tree_sitter::Node<'t>,
    source: &str,
) -> Option<(usize, usize, tree_sitter::Node<'t>)> {
    let mut cursor = root.walk();
    let mut doc_node = None;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "block_comment" => {
                let text = child.utf8_text(source.as_bytes()).ok()?;
                if text.starts_with("/**") {
                    doc_node = Some(child);
                    break;
                }
            }
            "line_comment" => continue,
            "package_declaration" => break,
            _ => break,
        }
    }

    let node = doc_node?;
    Some((node.start_position().row, node.end_position().row, node))
}
