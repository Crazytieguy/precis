use crate::Lang;

use super::ast::has_preceding_attribute;

/// Default visibility rule for functions, types, consts, statics, impls,
/// headings. Language-specific: Go uses identifier case, Python `_` prefix,
/// C `static` + `_` prefix, Lua `local` keyword, others check for explicit
/// `pub` / `export`.
pub(crate) fn symbol_visibility(
    node: tree_sitter::Node,
    source: &str,
    lang: Lang,
) -> bool {
    let base = match lang {
        Lang::Go => {
            let ident = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .unwrap_or("");
            ident.starts_with(|c: char| c.is_ascii_uppercase())
        }
        Lang::Java => is_java_public(node, source),
        Lang::Python => {
            let ident = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .unwrap_or("");
            !ident.starts_with('_') || (ident.starts_with("__") && ident.ends_with("__"))
        }
        Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => true,
        Lang::C => {
            let ident = c_identifier_text(node, source);
            !is_c_static(node, source) && !ident.starts_with('_')
        }
        Lang::Lua => {
            let text = node.utf8_text(source.as_bytes()).unwrap_or("");
            let ident = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .unwrap_or("");
            !text.starts_with("local") || ident.contains('.')
        }
        _ => is_public_symbol(node, source),
    };
    apply_doc_hidden(base, node, source, lang)
}

/// Visibility for imports. Non-Rust imports are never public (importing is
/// not exporting in most languages). Rust `pub use` is public, subject to
/// `#[doc(hidden)]`.
pub(crate) fn import_visibility(
    node: tree_sitter::Node,
    source: &str,
    lang: Lang,
) -> bool {
    if lang != Lang::Rust {
        return false;
    }
    apply_doc_hidden(is_public_symbol(node, source), node, source, lang)
}

/// Visibility for macros. Same as `symbol_visibility` but with a Rust-only
/// `#[macro_export]` override that promotes otherwise-private macros to
/// public. `symbol_visibility` has already filtered `#[doc(hidden)]` out
/// of `base`, so we only need to handle the promotion direction here.
pub(crate) fn macro_visibility(
    node: tree_sitter::Node,
    source: &str,
    lang: Lang,
) -> bool {
    let base = symbol_visibility(node, source, lang);
    if !base
        && lang == Lang::Rust
        && has_preceding_attribute(node, source, "#[macro_export]")
        && !has_preceding_attribute(node, source, "#[doc(hidden)]")
    {
        return true;
    }
    base
}

fn apply_doc_hidden(base: bool, node: tree_sitter::Node, source: &str, lang: Lang) -> bool {
    if base && lang == Lang::Rust && has_preceding_attribute(node, source, "#[doc(hidden)]") {
        false
    } else {
        base
    }
}

fn c_identifier_text<'a>(node: tree_sitter::Node, source: &'a str) -> &'a str {
    if let Some(name_node) = node.child_by_field_name("name")
        && let Ok(text) = name_node.utf8_text(source.as_bytes())
    {
        return text.trim();
    }
    if let Some(decl) = node.child_by_field_name("declarator")
        && let Some(found) = super::ast::find_descendant_of_kind(decl, "identifier")
            .or_else(|| super::ast::find_descendant_of_kind(decl, "type_identifier"))
        && let Ok(text) = found.utf8_text(source.as_bytes())
    {
        return text.trim();
    }
    ""
}

fn is_public_symbol(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    if let Some(vis) = node
        .children(&mut cursor)
        .find(|child| child.kind() == "visibility_modifier")
    {
        let vis_text = vis.utf8_text(source.as_bytes()).unwrap_or("");
        return vis_text == "pub";
    }
    if let Some(parent) = node.parent()
        && parent.kind() == "export_statement"
    {
        return true;
    }
    if matches!(node.kind(), "function_signature_item" | "function_item")
        && let Some(parent) = node.parent()
        && parent.kind() == "declaration_list"
        && let Some(grandparent) = parent.parent()
        && grandparent.kind() == "trait_item"
    {
        return true;
    }
    if let Some(parent) = node.parent()
        && parent.kind() == "declaration_list"
        && let Some(grandparent) = parent.parent()
        && grandparent.kind() == "impl_item"
        && grandparent.child_by_field_name("trait").is_some()
    {
        return true;
    }
    if matches!(
        node.kind(),
        "method_definition"
            | "method_signature"
            | "abstract_method_signature"
            | "public_field_definition"
    ) {
        if node
            .child_by_field_name("name")
            .is_some_and(|n| n.kind() == "private_property_identifier")
        {
            return false;
        }
        let mut cursor = node.walk();
        let has_accessor = node
            .children(&mut cursor)
            .any(|child| child.kind() == "accessibility_modifier");
        if !has_accessor {
            if matches!(node.kind(), "method_definition" | "public_field_definition") {
                let is_underscore_prefixed = node
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .is_some_and(|name| name.starts_with('_'));
                if is_underscore_prefixed {
                    return false;
                }
            }
            return true;
        }
        let mut cursor = node.walk();
        return node.children(&mut cursor).any(|child| {
            child.kind() == "accessibility_modifier"
                && child.utf8_text(source.as_bytes()).unwrap_or("") == "public"
        });
    }
    false
}

fn is_java_public(node: tree_sitter::Node, source: &str) -> bool {
    if let Some(mods) = node
        .children(&mut node.walk())
        .find(|c| c.kind() == "modifiers")
    {
        let mut cursor = mods.walk();
        for child in mods.children(&mut cursor) {
            match child.utf8_text(source.as_bytes()).ok() {
                Some("public" | "protected") => return true,
                Some("private") => return false,
                _ => {}
            }
        }
    }
    if let Some(parent) = node.parent()
        && matches!(parent.kind(), "interface_body" | "annotation_type_body")
    {
        return true;
    }
    node.kind() == "module_declaration"
}

fn is_c_static(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|child| {
        child.kind() == "storage_class_specifier"
            && child.utf8_text(source.as_bytes()).unwrap_or("") == "static"
    })
}
