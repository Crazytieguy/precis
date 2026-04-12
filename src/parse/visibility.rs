use crate::Lang;

use super::ItemKind;
use super::ast::has_preceding_attribute;

pub(crate) fn determine_visibility(
    node: tree_sitter::Node,
    kind: ItemKind,
    name: &str,
    source: &str,
    lang: Lang,
) -> bool {
    let base = if kind == ItemKind::Import {
        lang == Lang::Rust && is_public_symbol(node, source)
    } else {
        match lang {
            Lang::Go => name.starts_with(|c: char| c.is_ascii_uppercase()),
            Lang::Java => is_java_public(node, source),
            Lang::Python => {
                !name.starts_with('_') || (name.starts_with("__") && name.ends_with("__"))
            }
            Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => true,
            Lang::C => !is_c_static(node, source) && !name.starts_with('_'),
            Lang::Lua => {
                let text = node.utf8_text(source.as_bytes()).unwrap_or("");
                !text.starts_with("local") || name.contains('.')
            }
            _ => is_public_symbol(node, source),
        }
    };

    let is_public = if base
        && lang == Lang::Rust
        && has_preceding_attribute(node, source, "#[doc(hidden)]")
    {
        false
    } else {
        base
    };

    if !is_public
        && lang == Lang::Rust
        && kind == ItemKind::Macro
        && has_preceding_attribute(node, source, "#[macro_export]")
        && !has_preceding_attribute(node, source, "#[doc(hidden)]")
    {
        return true;
    }

    is_public
}

fn is_public_symbol(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    if let Some(vis) = node
        .children(&mut cursor)
        .find(|child| child.kind() == "visibility_modifier")
    {
        let vis_text = vis.utf8_text(source.as_bytes()).unwrap_or("");
        return vis_text.starts_with("pub");
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
