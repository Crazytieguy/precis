use crate::Lang;

use super::SymbolKind;

/// Extract the display name for a symbol.
///
/// Returns `None` when no name can be determined (caller should skip the symbol).
/// Takes the resolved name capture node from the tree-sitter query, if available.
pub(super) fn extract_name(
    node: tree_sitter::Node,
    kind: SymbolKind,
    name_node: Option<tree_sitter::Node>,
    source: &str,
    lang: Lang,
) -> Option<String> {
    let name = if kind == SymbolKind::Impl {
        impl_name(node, source)
    } else if kind == SymbolKind::Import {
        import_name(node, source, lang)
    } else if matches!(node.kind(), "const_declaration" | "var_declaration") {
        // Go grouped const/var block: use keyword as display name
        (if node.kind() == "const_declaration" { "const" } else { "var" }).to_string()
    } else if node.kind() == "type_definition" {
        // C typedef: extract the declarator name
        typedef_name(node, source)
    } else if node.kind() == "declaration" && lang == Lang::C {
        // C/C++ declaration: extract name from the declarator
        c_declaration_name(node, source)
    } else {
        name_node
            .and_then(|n| n.utf8_text(source.as_bytes()).ok())
            .map(|s| s.trim().to_string())?
    };

    // Strip trailing badge markdown from section heading names.
    let name = if kind == SymbolKind::Section {
        crate::layout::strip_heading_badges(&name).to_string()
    } else {
        name
    };

    // Filter out blank identifier `_` in Go and Rust.
    if matches!(lang, Lang::Go | Lang::Rust) && name == "_" {
        return None;
    }

    Some(name)
}

/// Determine if an import is 1st-party (local/relative) based on its name.
pub(super) fn is_first_party_import(name: &str, lang: Lang) -> bool {
    match lang {
        Lang::Rust => {
            name.starts_with("crate::") || name.starts_with("self::") || name.starts_with("super::")
        }
        Lang::JsTs => name.starts_with("./") || name.starts_with("../"),
        Lang::Go => false,
        Lang::Python => name.starts_with('.'),
        Lang::C => name.starts_with('"'),
        Lang::Lua | Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => false,
    }
}

/// Build a display name for an import statement.
fn import_name(node: tree_sitter::Node, source: &str, lang: Lang) -> String {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("?").trim();
    match lang {
        Lang::Rust => {
            let rest = text
                .strip_prefix("pub")
                .map(|s| s.trim_start())
                .unwrap_or(text);
            let rest = rest
                .strip_prefix("use")
                .map(|s| s.trim_start())
                .unwrap_or(rest);
            rest.trim_end_matches(';').trim().to_string()
        }
        Lang::JsTs => {
            if let Some(from_pos) = text.rfind(" from ") {
                let after = &text[from_pos + 6..];
                after
                    .trim()
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == ';')
                    .to_string()
            } else {
                let rest = text
                    .strip_prefix("import")
                    .map(|s| s.trim_start())
                    .unwrap_or(text);
                rest.trim_matches(|c: char| c == '\'' || c == '"' || c == ';' || c.is_whitespace())
                    .to_string()
            }
        }
        Lang::Go => {
            if text.contains('(') {
                "import".to_string()
            } else if let Some(start) = text.find('"') {
                let rest = &text[start + 1..];
                rest.find('"')
                    .map(|end| rest[..end].to_string())
                    .unwrap_or_else(|| "import".to_string())
            } else {
                "import".to_string()
            }
        }
        Lang::Python => {
            if let Some(rest) = text.strip_prefix("from") {
                let rest = rest.trim_start();
                if let Some(import_pos) = rest.find(" import") {
                    rest[..import_pos].trim().to_string()
                } else {
                    rest.split_whitespace()
                        .next()
                        .unwrap_or("?")
                        .to_string()
                }
            } else {
                let rest = text
                    .strip_prefix("import")
                    .map(|s| s.trim_start())
                    .unwrap_or(text);
                rest.split(&[',', ' ', '\n'][..])
                    .next()
                    .unwrap_or("?")
                    .trim()
                    .to_string()
            }
        }
        Lang::C => {
            text.strip_prefix("#include")
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| text.to_string())
        }
        Lang::Lua | Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => "?".to_string(),
    }
}

/// Build a display name for an impl block, e.g. "Display for Foo" or "Foo".
fn impl_name(node: tree_sitter::Node, source: &str) -> String {
    let type_node = node.child_by_field_name("type");
    let trait_node = node.child_by_field_name("trait");

    let type_name = type_node
        .and_then(|n| n.utf8_text(source.as_bytes()).ok())
        .unwrap_or("?");

    match trait_node.and_then(|n| n.utf8_text(source.as_bytes()).ok()) {
        Some(trait_name) => format!("{trait_name} for {type_name}"),
        None => type_name.to_string(),
    }
}

/// Extract the typedef name from a C `type_definition` node.
fn typedef_name(node: tree_sitter::Node, source: &str) -> String {
    if let Some(decl) = node.child_by_field_name("declarator")
        && let Some(found) = super::ast::find_descendant_of_kind(decl, "type_identifier")
        && let Ok(name) = found.utf8_text(source.as_bytes())
    {
        return name.to_string();
    }
    "?".to_string()
}

/// Extract the name from a C `declaration` node (global variable or function prototype).
fn c_declaration_name(node: tree_sitter::Node, source: &str) -> String {
    if let Some(decl) = node.child_by_field_name("declarator")
        && let Some(found) = super::ast::find_descendant_of_kind(decl, "identifier")
        && let Ok(name) = found.utf8_text(source.as_bytes())
    {
        return name.to_string();
    }
    "?".to_string()
}
