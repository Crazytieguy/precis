use std::path::Path;

use crate::Lang;

use super::ItemKind;

pub(super) fn classify_node<'a>(
    node: tree_sitter::Node<'a>,
    captures: &[tree_sitter::QueryCapture<'a>],
    name_idx: Option<u32>,
    source: &str,
    lang: Lang,
    path: &Path,
) -> Option<ItemKind> {
    match node.kind() {
        // Rust
        "function_item" | "function_signature_item" => Some(ItemKind::Function),
        "struct_item" => Some(ItemKind::Struct),
        "enum_item" => Some(ItemKind::Enum),
        "trait_item" => Some(ItemKind::Trait),
        "impl_item" => Some(ItemKind::Impl),
        "type_item" => Some(ItemKind::TypeAlias),
        "const_item" => Some(ItemKind::Const),
        "static_item" => Some(ItemKind::Static),
        "macro_definition" => Some(ItemKind::Macro),
        "mod_item" => Some(ItemKind::Module),
        // TypeScript / Go
        "function_declaration" | "method_definition" | "method_signature"
        | "abstract_method_signature" | "method_declaration" => Some(ItemKind::Function),
        "class_declaration" | "abstract_class_declaration" => Some(ItemKind::Class),
        "interface_declaration" => Some(ItemKind::Interface),
        "enum_declaration" => Some(ItemKind::Enum),
        "type_alias_declaration" => Some(ItemKind::TypeAlias),
        "lexical_declaration" => classify_lexical_declaration(node, source),
        "public_field_definition" => {
            let value_kind = node.child_by_field_name("value").map(|v| v.kind());
            if matches!(
                value_kind,
                Some("arrow_function" | "function_expression" | "generator_function")
            ) {
                Some(ItemKind::Function)
            } else {
                None
            }
        }
        "internal_module" => Some(ItemKind::Module),
        // Go
        "type_spec" => {
            let type_child = node.child_by_field_name("type").map(|t| t.kind());
            match type_child {
                Some("struct_type") => Some(ItemKind::Struct),
                Some("interface_type") => Some(ItemKind::Interface),
                _ => Some(ItemKind::TypeAlias),
            }
        }
        "type_alias" => Some(ItemKind::TypeAlias),
        "const_declaration" | "var_declaration" => classify_go_grouped_declaration(node),
        "const_spec" => Some(ItemKind::Const),
        "var_spec" => Some(ItemKind::Static),
        // C / C++
        "function_definition" if lang == Lang::C => Some(ItemKind::Function),
        "class_specifier" | "struct_specifier" | "union_specifier" | "enum_specifier" => {
            classify_c_type_specifier(node)
        }
        "type_definition" => {
            if is_simple_typedef_alias(node) {
                None
            } else {
                Some(ItemKind::TypeAlias)
            }
        }
        "preproc_def" | "preproc_function_def" => classify_c_preproc(node, source, path),
        "preproc_include" => Some(ItemKind::Import),
        "declaration" if lang == Lang::C => {
            if super::ast::find_descendant_of_kind(node, "function_declarator").is_some() {
                Some(ItemKind::Function)
            } else {
                Some(ItemKind::Static)
            }
        }
        // C++ specific
        "namespace_definition" => Some(ItemKind::Module),
        "alias_declaration" => Some(ItemKind::TypeAlias),
        // Java
        "record_declaration" => Some(ItemKind::Struct),
        "annotation_type_declaration" => Some(ItemKind::Interface),
        "constructor_declaration" => Some(ItemKind::Function),
        "annotation_type_element_declaration" => Some(ItemKind::Function),
        "module_declaration" => Some(ItemKind::Module),
        "field_declaration" if lang == Lang::Java => classify_java_field(node, source),
        "constant_declaration" if lang == Lang::Java => Some(ItemKind::Const),
        // Python
        "function_definition" => Some(ItemKind::Function),
        "class_definition" => Some(ItemKind::Class),
        // Markdown / JSON / TOML / YAML
        "atx_heading" | "setext_heading" | "pair" | "table" | "table_array_element"
        | "block_mapping_pair" => Some(ItemKind::Section),
        // Imports
        "use_declaration" => Some(ItemKind::Import),
        "import_statement" => {
            if lang == Lang::Python || lang == Lang::JsTs {
                Some(ItemKind::Import)
            } else {
                None
            }
        }
        "import_from_statement" => Some(ItemKind::Import),
        "import_declaration" => Some(ItemKind::Import),
        // Python module-level assignments
        "expression_statement" if lang == Lang::Python => {
            classify_python_expression(node, captures, name_idx, source)
        }
        // Lua
        "variable_declaration" | "assignment_statement" if lang == Lang::Lua => {
            Some(classify_lua_assignment(node))
        }
        _ => None,
    }
}

fn classify_lexical_declaration(node: tree_sitter::Node, source: &str) -> Option<ItemKind> {
    let keyword = node.child(0).map(|c| c.kind());
    if keyword != Some("const") {
        return None;
    }
    let declarator = node
        .named_children(&mut node.walk())
        .find(|c| c.kind() == "variable_declarator");
    let value_kind = declarator
        .and_then(|d| d.child_by_field_name("value"))
        .map(|v| v.kind());
    if matches!(
        value_kind,
        Some("arrow_function" | "function_expression" | "generator_function")
    ) {
        Some(ItemKind::Function)
    } else if is_require_call(declarator, source) {
        None
    } else {
        Some(ItemKind::Const)
    }
}

fn classify_go_grouped_declaration(node: tree_sitter::Node) -> Option<ItemKind> {
    let is_grouped = (0..node.child_count())
        .filter_map(|i| node.child(i))
        .any(|c| c.kind() == "(" || c.kind() == "var_spec_list");
    if !is_grouped {
        return None;
    }
    if node.kind() == "const_declaration" {
        Some(ItemKind::Const)
    } else {
        Some(ItemKind::Static)
    }
}

fn classify_lua_assignment(node: tree_sitter::Node) -> ItemKind {
    let assign = if node.kind() == "variable_declaration" {
        node.named_children(&mut node.walk())
            .find(|c| c.kind() == "assignment_statement")
    } else {
        Some(node)
    };
    let value_kind = assign
        .and_then(|a| {
            a.named_children(&mut a.walk())
                .find(|c| c.kind() == "expression_list")
        })
        .and_then(|el| el.named_child(0))
        .map(|v| v.kind());
    if value_kind == Some("function_definition") {
        ItemKind::Function
    } else {
        ItemKind::Const
    }
}

fn classify_c_type_specifier(node: tree_sitter::Node) -> Option<ItemKind> {
    node.child_by_field_name("body")?;
    if node
        .parent()
        .is_some_and(|p| p.kind() == "type_definition")
    {
        return None;
    }
    match node.kind() {
        "enum_specifier" => Some(ItemKind::Enum),
        "class_specifier" => Some(ItemKind::Class),
        _ => Some(ItemKind::Struct),
    }
}

fn classify_c_preproc(
    node: tree_sitter::Node,
    source: &str,
    path: &Path,
) -> Option<ItemKind> {
    if node.kind() == "preproc_def" {
        if is_c_header_guard(node, source, path) {
            return None;
        }
        if is_platform_type_define(node, source) {
            return None;
        }
    }
    Some(ItemKind::Macro)
}

fn classify_python_expression<'a>(
    node: tree_sitter::Node<'a>,
    captures: &[tree_sitter::QueryCapture<'a>],
    name_idx: Option<u32>,
    source: &str,
) -> Option<ItemKind> {
    if node.parent().map(|p| p.kind()) != Some("module") {
        return None;
    }
    let assignment = node
        .named_children(&mut node.walk())
        .find(|c| c.kind() == "assignment");
    let has_type_annotation = assignment
        .map(|a| {
            let mut cursor = a.walk();
            a.children(&mut cursor).any(|c| c.kind() == "type")
        })
        .unwrap_or(false);
    if !has_type_annotation {
        let name_text = name_idx
            .and_then(|idx| captures.iter().find(|c| c.index == idx))
            .and_then(|c| c.node.utf8_text(source.as_bytes()).ok())
            .unwrap_or("");
        let is_upper = !name_text.is_empty()
            && name_text
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b == b'_')
            && name_text.bytes().any(|b| b.is_ascii_uppercase());
        let is_dunder = name_text.starts_with("__") && name_text.ends_with("__");
        if !is_upper && !is_dunder {
            return None;
        }
    }
    Some(ItemKind::Const)
}

fn is_require_call(declarator: Option<tree_sitter::Node>, source: &str) -> bool {
    let value = declarator.and_then(|d| d.child_by_field_name("value"));
    match value {
        Some(v) if v.kind() == "call_expression" => v
            .child_by_field_name("function")
            .and_then(|f| f.utf8_text(source.as_bytes()).ok())
            == Some("require"),
        Some(v) if v.kind() == "member_expression" => {
            v.child_by_field_name("object").is_some_and(|obj| {
                obj.kind() == "call_expression"
                    && obj
                        .child_by_field_name("function")
                        .and_then(|f| f.utf8_text(source.as_bytes()).ok())
                        == Some("require")
            })
        }
        _ => false,
    }
}

fn classify_java_field(node: tree_sitter::Node, source: &str) -> Option<ItemKind> {
    let modifiers = node
        .children(&mut node.walk())
        .find(|c| c.kind() == "modifiers")?;
    let has_static = has_modifier_keyword(modifiers, source, "static");
    let has_final = has_modifier_keyword(modifiers, source, "final");
    if has_static && has_final {
        Some(ItemKind::Const)
    } else {
        None
    }
}

fn has_modifier_keyword(modifiers: tree_sitter::Node, source: &str, keyword: &str) -> bool {
    let mut cursor = modifiers.walk();
    modifiers
        .children(&mut cursor)
        .any(|child| child.utf8_text(source.as_bytes()).ok() == Some(keyword))
}

fn is_simple_typedef_alias(node: tree_sitter::Node) -> bool {
    let type_child = match node.child_by_field_name("type") {
        Some(t) => t,
        None => return false,
    };
    if !matches!(
        type_child.kind(),
        "type_identifier" | "primitive_type" | "sized_type_specifier"
    ) {
        return false;
    }
    let declarator = match node.child_by_field_name("declarator") {
        Some(d) => d,
        None => return false,
    };
    matches!(declarator.kind(), "type_identifier" | "primitive_type")
}

fn is_c_header_guard(node: tree_sitter::Node, source: &str, path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !crate::classify::is_header_extension(ext) {
        return false;
    }
    if node.child_by_field_name("value").is_some() {
        return false;
    }
    let stem = match path.file_stem().and_then(|s| s.to_str()) {
        Some(s) if !s.is_empty() => s.to_ascii_uppercase(),
        _ => return false,
    };
    let name = match node.child_by_field_name("name") {
        Some(n) => n
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .to_ascii_uppercase(),
        None => return false,
    };
    name.contains(&stem)
}

fn is_platform_type_define(node: tree_sitter::Node, source: &str) -> bool {
    let name = match node.child_by_field_name("name") {
        Some(n) => n.utf8_text(source.as_bytes()).unwrap_or(""),
        None => return false,
    };
    if !name.ends_with("_TYPE") {
        return false;
    }
    let value = match node.child_by_field_name("value") {
        Some(v) => v.utf8_text(source.as_bytes()).unwrap_or(""),
        None => return false,
    };
    let value = value.trim();
    !value.is_empty()
        && value.split_whitespace().all(|token| {
            token.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                && token
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
}
