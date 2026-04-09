use crate::Lang;

use super::Symbol;
use super::SymbolKind;

/// Detect module-level documentation and return a synthetic Symbol for it.
///
/// Module-level docs are the most concise description of what a file does:
/// - Rust: `//!` inner doc comments at the start of a file
/// - Python: module docstring (triple-quoted string as first statement)
/// - Go: package comment (comment block adjacent to `package` clause)
///
/// Returns `None` if the file has no module-level documentation.
pub(super) fn detect_module_doc(
    root: tree_sitter::Node,
    source: &str,
    lang: Lang,
) -> Option<Symbol> {
    let (start_row, end_row, start_byte, end_byte) = match lang {
        Lang::Rust => detect_rust(root, source)?,
        Lang::Python => detect_python(root, source)?,
        Lang::Go => detect_go(root, source)?,
        Lang::Java => detect_java(root, source)?,
        _ => return None,
    };

    // Use the first line's text as the symbol name (used for classification, not rendering)
    let name = source
        .lines()
        .nth(start_row)
        .unwrap_or("")
        .trim()
        .to_string();

    Some(Symbol {
        kind: SymbolKind::ModuleDoc,
        name,
        is_public: true,
        is_first_party: false,
        line: start_row + 1,        // 1-indexed
        end_line: end_row + 1,      // 1-indexed inclusive
        sig_end_line: None,
        doc_start_line: None,
        is_trait_impl: false,
        is_reexport: false,
        start_byte,
        end_byte,
        composed_prefix_lens: vec![],
        layout: Default::default(),
    })
}

/// Detect Rust `//!` inner doc comments at the start of a file.
/// Also handles `/*! ... */` block inner doc comments.
fn detect_rust(
    root: tree_sitter::Node,
    source: &str,
) -> Option<(usize, usize, usize, usize)> {
    let mut start_row = None;
    let mut end_row = None;
    let mut start_byte = 0;
    let mut end_byte = 0;

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
                    start_byte = child.start_byte();
                }
                // Use start_position().row (not end_position().row) because
                // tree-sitter includes the trailing \n in line_comment nodes,
                // making end_position().row one too high.
                end_row = Some(child.start_position().row);
                end_byte = child.end_byte();
            }
            "block_comment" if text.starts_with("/*!") => {
                if start_row.is_none() {
                    start_row = Some(child.start_position().row);
                    start_byte = child.start_byte();
                }
                end_row = Some(child.end_position().row);
                end_byte = child.end_byte();
                // Block comment is a single node — stop after it
                break;
            }
            // Skip crate-level attributes (#![no_std], #![warn(...)], etc.)
            // and regular line comments that appear before module docs.
            "attribute_item" | "inner_attribute_item" | "line_comment" => continue,
            _ => break,
        }
    }

    Some((start_row?, end_row?, start_byte, end_byte))
}

/// Detect Python module docstring (triple-quoted string as the first statement).
fn detect_python(
    root: tree_sitter::Node,
    source: &str,
) -> Option<(usize, usize, usize, usize)> {
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        // Skip comments (e.g., shebang lines, encoding declarations)
        if child.kind() == "comment" {
            continue;
        }
        // Skip future imports that may precede the docstring
        if child.kind() == "future_import_statement" {
            continue;
        }

        // The module docstring must be an expression_statement containing a string
        if child.kind() == "expression_statement" {
            let first_child = child.child(0)?;
            if matches!(first_child.kind(), "string" | "concatenated_string") {
                let text = first_child.utf8_text(source.as_bytes()).ok()?;
                // Must be a triple-quoted string (docstring convention)
                if text.starts_with("\"\"\"") || text.starts_with("'''") {
                    return Some((
                        child.start_position().row,
                        child.end_position().row,
                        child.start_byte(),
                        child.end_byte(),
                    ));
                }
            }
        }

        // First non-comment, non-docstring statement → no module docstring
        break;
    }
    None
}

/// Detect Go package comment (comment block adjacent to `package` clause).
/// Skips build constraint comments (`//go:build`, `// +build`).
fn detect_go(
    root: tree_sitter::Node,
    source: &str,
) -> Option<(usize, usize, usize, usize)> {
    // Find the package_clause node
    let mut cursor = root.walk();
    let pkg_node = root
        .children(&mut cursor)
        .find(|child| child.kind() == "package_clause")?;
    let pkg_row = pkg_node.start_position().row;

    // Walk backwards from package_clause to find adjacent comment block.
    // Track first (topmost) and last (closest to package) doc comment nodes
    // directly instead of collecting into a Vec.
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

        // Skip build constraints — these are compiler directives, not docs
        if text.starts_with("//go:build") || text.starts_with("// +build") {
            sibling = node.prev_named_sibling();
            continue;
        }

        // Check adjacency: gap to the next doc comment (or package clause)
        // must be at most 1 blank line. Use end_position for the gap check
        // to handle multi-line block comments (/* ... */) correctly.
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

    // Use end_position for the last comment to handle multi-line block
    // comments. For line comments, end_position().row may include a trailing
    // \n, but that's still correct because the row of the content is the
    // same as start_position().row for single-line comments. For block
    // comments, end_position gives the closing */ row.
    let end_row = last.end_position().row;
    // Clamp: if tree-sitter puts end on the next line due to trailing \n,
    // use start_position instead (single-line comment case).
    let end_row = if end_row > last.start_position().row
        && last.end_position().column == 0
    {
        last.start_position().row
    } else {
        end_row
    };

    Some((
        first.start_position().row,
        end_row,
        first.start_byte(),
        last.end_byte(),
    ))
}

/// Detect Java Javadoc at the top of a file (before any declarations).
/// A leading `block_comment` starting with `/**` is treated as module-level
/// documentation. This covers package-info.java and any file with a top-level
/// Javadoc block preceding the first class/interface/enum declaration.
fn detect_java(
    root: tree_sitter::Node,
    source: &str,
) -> Option<(usize, usize, usize, usize)> {
    let mut cursor = root.walk();
    // Find the first block_comment that starts with /**
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
            // Skip line comments at the top of the file
            "line_comment" => continue,
            // If we hit a non-comment, non-package node, no module doc
            "package_declaration" => break,
            _ => break,
        }
    }

    let node = doc_node?;
    Some((
        node.start_position().row,
        node.end_position().row,
        node.start_byte(),
        node.end_byte(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_detect(source: &str, lang: Lang) -> Option<Symbol> {
        let ts_lang = match lang {
            Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
            Lang::Python => tree_sitter_python::LANGUAGE.into(),
            Lang::Go => tree_sitter_go::LANGUAGE.into(),
            _ => return None,
        };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&ts_lang).unwrap();
        let tree = parser.parse(source, None).unwrap();
        detect_module_doc(tree.root_node(), source, lang)
    }

    #[test]
    fn rust_inner_doc_comments() {
        let source = "//! Module docs line 1.\n//! Module docs line 2.\n\npub fn foo() {}\n";
        let sym = parse_and_detect(source, Lang::Rust).unwrap();
        assert_eq!(sym.kind, SymbolKind::ModuleDoc);
        assert_eq!(sym.line, 1);
        assert_eq!(sym.end_line, 2);
        assert!(sym.is_public);
    }

    #[test]
    fn rust_block_inner_doc() {
        let source = "/*!\n * Module docs.\n */\n\npub fn foo() {}\n";
        let sym = parse_and_detect(source, Lang::Rust).unwrap();
        assert_eq!(sym.kind, SymbolKind::ModuleDoc);
        assert_eq!(sym.line, 1);
        assert_eq!(sym.end_line, 3);
    }

    #[test]
    fn rust_no_module_doc() {
        let source = "/// Doc for foo.\npub fn foo() {}\n";
        assert!(parse_and_detect(source, Lang::Rust).is_none());
    }

    #[test]
    fn python_module_docstring() {
        let source = "\"\"\"Module docs.\n\nMore details.\n\"\"\"\n\nimport os\n";
        let sym = parse_and_detect(source, Lang::Python).unwrap();
        assert_eq!(sym.kind, SymbolKind::ModuleDoc);
        assert_eq!(sym.line, 1);
        assert_eq!(sym.end_line, 4);
    }

    #[test]
    fn python_shebang_then_docstring() {
        let source = "#!/usr/bin/env python3\n\"\"\"Module docs.\"\"\"\n\nimport os\n";
        let sym = parse_and_detect(source, Lang::Python).unwrap();
        assert_eq!(sym.kind, SymbolKind::ModuleDoc);
        assert_eq!(sym.line, 2);
        assert_eq!(sym.end_line, 2);
    }

    #[test]
    fn python_no_module_docstring() {
        let source = "# Just a comment.\nimport os\n";
        assert!(parse_and_detect(source, Lang::Python).is_none());
    }

    #[test]
    fn go_package_comment() {
        let source = "// Package token provides lexical token types.\npackage token\n\ntype Token struct{}\n";
        let sym = parse_and_detect(source, Lang::Go).unwrap();
        assert_eq!(sym.kind, SymbolKind::ModuleDoc);
        assert_eq!(sym.line, 1);
        assert_eq!(sym.end_line, 1);
    }

    #[test]
    fn go_multiline_package_comment() {
        let source = "// Package token provides lexical token types.\n// It supports multiple token kinds.\npackage token\n";
        let sym = parse_and_detect(source, Lang::Go).unwrap();
        assert_eq!(sym.line, 1);
        assert_eq!(sym.end_line, 2);
    }

    #[test]
    fn go_build_tags_skipped() {
        let source = "//go:build !windows\n\n// Package token provides types.\npackage token\n";
        let sym = parse_and_detect(source, Lang::Go).unwrap();
        assert_eq!(sym.line, 3); // skipped the build tag
        assert_eq!(sym.end_line, 3);
    }

    #[test]
    fn go_only_build_tags() {
        let source = "//go:build linux\n\npackage token\n";
        assert!(parse_and_detect(source, Lang::Go).is_none());
    }

    #[test]
    fn go_no_package_comment() {
        let source = "package token\n\ntype Token struct{}\n";
        assert!(parse_and_detect(source, Lang::Go).is_none());
    }

    #[test]
    fn rust_attrs_before_module_doc() {
        let source = "#![no_std]\n#![warn(missing_docs)]\n//! Module docs.\n//! More docs.\n\npub fn foo() {}\n";
        let sym = parse_and_detect(source, Lang::Rust).unwrap();
        assert_eq!(sym.kind, SymbolKind::ModuleDoc);
        assert_eq!(sym.line, 3);
        assert_eq!(sym.end_line, 4);
    }

    #[test]
    fn go_block_comment_package_doc() {
        let source = "/*\nPackage token provides lexical token types.\n\nIt supports multiple token kinds.\n*/\npackage token\n";
        let sym = parse_and_detect(source, Lang::Go).unwrap();
        assert_eq!(sym.line, 1);
        assert_eq!(sym.end_line, 5);
    }
}
