use crate::Lang;

use super::SymbolKind;
use super::ast::has_preceding_attribute;

/// Determine the visibility of a symbol.
///
/// Incorporates language-specific visibility rules and Rust's three-step logic:
/// base visibility → `#[doc(hidden)]` override → `#[macro_export]` override.
pub(super) fn determine_visibility(
    node: tree_sitter::Node,
    kind: SymbolKind,
    name: &str,
    source: &str,
    lang: Lang,
) -> bool {
    let base = if kind == SymbolKind::Import {
        // Only Rust `pub use` re-exports are public; all other imports are private
        lang == Lang::Rust && is_public_symbol(node, source)
    } else {
        match lang {
            Lang::Go => name.starts_with(|c: char| c.is_ascii_uppercase()),
            Lang::Java => is_java_public(node, source),
            Lang::Python => !name.starts_with('_') || (name.starts_with("__") && name.ends_with("__")),
            Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => true,
            Lang::C => !is_c_static(node, source) && !name.starts_with('_'),
            Lang::Lua => {
                let text = node.utf8_text(source.as_bytes()).unwrap_or("");
                !text.starts_with("local") || name.contains('.')
            },
            _ => is_public_symbol(node, source),
        }
    };

    // Rust: #[doc(hidden)] items are technically `pub` but the author doesn't
    // want them shown. Treat as non-public.
    let is_public = if base
        && lang == Lang::Rust
        && has_preceding_attribute(node, source, "#[doc(hidden)]")
    {
        false
    } else {
        base
    };

    // Rust: `#[macro_export]` macros are public API (unless also #[doc(hidden)]).
    if !is_public
        && lang == Lang::Rust
        && kind == SymbolKind::Macro
        && has_preceding_attribute(node, source, "#[macro_export]")
        && !has_preceding_attribute(node, source, "#[doc(hidden)]")
    {
        return true;
    }

    is_public
}

/// Check if a tree-sitter node has public visibility.
fn is_public_symbol(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    // Rust: `pub` keyword appears as a visibility_modifier child.
    // Restricted visibility (`pub(crate)`, `pub(super)`, `pub(in ...)`) is NOT
    // considered public.
    if let Some(vis) = node
        .children(&mut cursor)
        .find(|child| child.kind() == "visibility_modifier")
    {
        let vis_text = vis.utf8_text(source.as_bytes()).unwrap_or("");
        return vis_text == "pub";
    }
    // TypeScript: exported symbols are children of export_statement
    if let Some(parent) = node.parent()
        && parent.kind() == "export_statement"
    {
        return true;
    }
    // Rust: trait methods are always public
    if matches!(node.kind(), "function_signature_item" | "function_item")
        && let Some(parent) = node.parent()
        && parent.kind() == "declaration_list"
        && let Some(grandparent) = parent.parent()
        && grandparent.kind() == "trait_item"
    {
        return true;
    }
    // Rust: items in trait implementations are public
    if let Some(parent) = node.parent()
        && parent.kind() == "declaration_list"
        && let Some(grandparent) = parent.parent()
        && grandparent.kind() == "impl_item"
        && grandparent.child_by_field_name("trait").is_some()
    {
        return true;
    }
    // TypeScript class/interface methods
    if matches!(
        node.kind(),
        "method_definition" | "method_signature" | "abstract_method_signature" | "public_field_definition"
    ) {
        // JS #private methods are always private
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
            // Convention: _prefix means private for concrete methods and fields.
            if matches!(node.kind(), "method_definition" | "public_field_definition") {
                let is_underscore_prefixed = node
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .is_some_and(|name| name.starts_with('_'));
                if is_underscore_prefixed {
                    return false;
                }
            }
            return true; // no modifier = public by default
        }
        let mut cursor = node.walk();
        return node.children(&mut cursor).any(|child| {
            child.kind() == "accessibility_modifier"
                && child.utf8_text(source.as_bytes()).unwrap_or("") == "public"
        });
    }
    false
}

/// Check if a Java symbol is public.
///
/// Checks for explicit `public` or `protected` modifier keywords by walking
/// AST children (not string matching, to avoid false positives from annotations).
/// Interface and annotation type members are implicitly public.
fn is_java_public(node: tree_sitter::Node, source: &str) -> bool {
    // Check for explicit modifier keywords
    if let Some(mods) = node.children(&mut node.walk()).find(|c| c.kind() == "modifiers") {
        let mut cursor = mods.walk();
        for child in mods.children(&mut cursor) {
            match child.utf8_text(source.as_bytes()).ok() {
                Some("public" | "protected") => return true,
                Some("private") => return false,
                _ => {}
            }
        }
    }
    // Interface and annotation type members are implicitly public
    if let Some(parent) = node.parent()
        && matches!(parent.kind(), "interface_body" | "annotation_type_body")
    {
        return true;
    }
    // Module declarations are public
    node.kind() == "module_declaration"
}

/// Check if a C symbol has `static` storage class (file-scoped, not public).
fn is_c_static(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|child| {
        child.kind() == "storage_class_specifier"
            && child.utf8_text(source.as_bytes()).unwrap_or("") == "static"
    })
}
