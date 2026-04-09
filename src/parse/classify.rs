use std::path::Path;

use crate::Lang;

use super::SymbolKind;

/// Classify a tree-sitter node into a `SymbolKind`, or return `None` to skip it.
///
/// This encapsulates the massive node-kind match plus inline filtering logic
/// (require calls, header guards, simple typedefs, etc.). The caller should
/// treat `None` as "this node is not a symbol" and `continue`.
///
/// Takes a reference to the `QueryMatch` because Python `expression_statement`
/// classification needs to inspect the name capture text (uppercase/dunder check).
pub(super) fn classify_node<'a>(
    node: tree_sitter::Node<'a>,
    captures: &[tree_sitter::QueryCapture<'a>],
    name_idx: Option<u32>,
    source: &str,
    lang: Lang,
    path: &Path,
) -> Option<SymbolKind> {
    match node.kind() {
        // Rust
        "function_item" | "function_signature_item" => Some(SymbolKind::Function),
        "struct_item" => Some(SymbolKind::Struct),
        "enum_item" => Some(SymbolKind::Enum),
        "trait_item" => Some(SymbolKind::Trait),
        "impl_item" => Some(SymbolKind::Impl),
        "type_item" => Some(SymbolKind::TypeAlias),
        "const_item" => Some(SymbolKind::Const),
        "static_item" => Some(SymbolKind::Static),
        "macro_definition" => Some(SymbolKind::Macro),
        "mod_item" => Some(SymbolKind::Module),
        // TypeScript / Go
        "function_declaration" | "method_definition" | "method_signature"
        | "abstract_method_signature" | "method_declaration" => Some(SymbolKind::Function),
        "class_declaration" | "abstract_class_declaration" => Some(SymbolKind::Class),
        "interface_declaration" => Some(SymbolKind::Interface),
        "enum_declaration" => Some(SymbolKind::Enum),
        "type_alias_declaration" => Some(SymbolKind::TypeAlias),
        "lexical_declaration" => {
            classify_lexical_declaration(node, source)
        }
        "public_field_definition" => {
            let value_kind = node.child_by_field_name("value").map(|v| v.kind());
            if matches!(
                value_kind,
                Some("arrow_function" | "function_expression" | "generator_function")
            ) {
                Some(SymbolKind::Function)
            } else {
                None // Skip plain data fields
            }
        }
        "internal_module" => Some(SymbolKind::Module),
        // Go
        "type_spec" => {
            let type_child = node.child_by_field_name("type").map(|t| t.kind());
            match type_child {
                Some("struct_type") => Some(SymbolKind::Struct),
                Some("interface_type") => Some(SymbolKind::Interface),
                _ => Some(SymbolKind::TypeAlias),
            }
        }
        "type_alias" => Some(SymbolKind::TypeAlias),
        "const_declaration" | "var_declaration" => {
            classify_go_grouped_declaration(node)
        }
        "const_spec" => Some(SymbolKind::Const),
        "var_spec" => Some(SymbolKind::Static),
        // C / C++
        "function_definition" if lang == Lang::C => Some(SymbolKind::Function),
        "class_specifier" | "struct_specifier" | "union_specifier" | "enum_specifier" => {
            classify_c_type_specifier(node)
        }
        "type_definition" => {
            if is_simple_typedef_alias(node) {
                None
            } else {
                Some(SymbolKind::TypeAlias)
            }
        }
        "preproc_def" | "preproc_function_def" => {
            classify_c_preproc(node, source, path)
        }
        "preproc_include" => Some(SymbolKind::Import),
        "declaration" if lang == Lang::C => {
            if super::ast::find_descendant_of_kind(node, "function_declarator").is_some() {
                Some(SymbolKind::Function)
            } else {
                Some(SymbolKind::Static)
            }
        }
        // C++ specific
        "namespace_definition" => Some(SymbolKind::Module),
        "alias_declaration" => Some(SymbolKind::TypeAlias),
        // Python
        "function_definition" => Some(SymbolKind::Function),
        "class_definition" => Some(SymbolKind::Class),
        // Markdown / JSON / TOML / YAML
        "atx_heading" | "setext_heading" | "pair" | "table" | "table_array_element"
        | "block_mapping_pair" => Some(SymbolKind::Section),
        // Imports
        "use_declaration" => Some(SymbolKind::Import),
        "import_statement" => {
            if lang == Lang::Python || lang == Lang::JsTs {
                Some(SymbolKind::Import)
            } else {
                None
            }
        }
        "import_from_statement" => Some(SymbolKind::Import),
        "import_declaration" => Some(SymbolKind::Import),
        // Python module-level assignments
        "expression_statement" if lang == Lang::Python => {
            classify_python_expression(node, captures, name_idx, source)
        }
        // Lua
        "variable_declaration" | "assignment_statement" if lang == Lang::Lua => Some(SymbolKind::Const),
        _ => None,
    }
}

/// Classify a TypeScript/JavaScript `lexical_declaration` node.
fn classify_lexical_declaration(node: tree_sitter::Node, source: &str) -> Option<SymbolKind> {
    // Filter out `let` and `var` — only `const` declarations are symbols.
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

    // Arrow functions / function expressions assigned to const → Function
    if matches!(
        value_kind,
        Some("arrow_function" | "function_expression" | "generator_function")
    ) {
        Some(SymbolKind::Function)
    }
    // CommonJS require() calls are imports, not definitions
    else if is_require_call(declarator, source) {
        None
    } else {
        Some(SymbolKind::Const)
    }
}

/// Classify a Go grouped const/var declaration.
fn classify_go_grouped_declaration(node: tree_sitter::Node) -> Option<SymbolKind> {
    // Only capture grouped declarations (const (...) / var (...)).
    // Standalone declarations are captured via inner const_spec/var_spec.
    let is_grouped = (0..node.child_count())
        .filter_map(|i| node.child(i))
        .any(|c| c.kind() == "(" || c.kind() == "var_spec_list");
    if !is_grouped {
        return None;
    }
    if node.kind() == "const_declaration" {
        Some(SymbolKind::Const)
    } else {
        Some(SymbolKind::Static)
    }
}

/// Classify C/C++ class/struct/union/enum specifiers.
fn classify_c_type_specifier(node: tree_sitter::Node) -> Option<SymbolKind> {
    // Only capture definitions (with body), not forward declarations.
    // Skip specifiers inside typedef — the typedef node captures the whole thing.
    node.child_by_field_name("body")?;
    if node
        .parent()
        .is_some_and(|p| p.kind() == "type_definition")
    {
        return None;
    }
    match node.kind() {
        "enum_specifier" => Some(SymbolKind::Enum),
        "class_specifier" => Some(SymbolKind::Class),
        _ => Some(SymbolKind::Struct),
    }
}

/// Classify C preprocessor definitions, filtering header guards and platform type defines.
fn classify_c_preproc(node: tree_sitter::Node, source: &str, path: &Path) -> Option<SymbolKind> {
    if node.kind() == "preproc_def" {
        if is_c_header_guard(node, source, path) {
            return None;
        }
        if is_platform_type_define(node, source) {
            return None;
        }
    }
    Some(SymbolKind::Macro)
}

/// Classify a Python `expression_statement` (module-level assignment).
fn classify_python_expression<'a>(
    node: tree_sitter::Node<'a>,
    captures: &[tree_sitter::QueryCapture<'a>],
    name_idx: Option<u32>,
    source: &str,
) -> Option<SymbolKind> {
    // Must be at module level (direct child of module node)
    if node.parent().map(|p| p.kind()) != Some("module") {
        return None;
    }
    // Check if the assignment has a type annotation
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
        // No type annotation — only keep UPPER_CASE or dunder names
        let name_text = name_idx
            .and_then(|idx| captures.iter().find(|c| c.index == idx))
            .and_then(|c| c.node.utf8_text(source.as_bytes()).ok())
            .unwrap_or("");
        let is_upper = !name_text.is_empty()
            && name_text.bytes().all(|b| b.is_ascii_uppercase() || b == b'_')
            && name_text.bytes().any(|b| b.is_ascii_uppercase());
        let is_dunder =
            name_text.starts_with("__") && name_text.ends_with("__");
        if !is_upper && !is_dunder {
            return None;
        }
    }
    Some(SymbolKind::Const)
}

/// Check if a variable_declarator's value is a `require()` call (CommonJS import).
/// Handles both `const x = require('...')` and `const x = require('...').member`.
fn is_require_call(declarator: Option<tree_sitter::Node>, source: &str) -> bool {
    let value = declarator.and_then(|d| d.child_by_field_name("value"));
    match value {
        Some(v) if v.kind() == "call_expression" => {
            v.child_by_field_name("function")
                .and_then(|f| f.utf8_text(source.as_bytes()).ok())
                == Some("require")
        }
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

/// Check if a `type_definition` is a simple typedef alias (`typedef T name;`) where
/// both the source type and target are plain identifiers. These are type portability
/// boilerplate (e.g. `typedef int8_t i8;`, `typedef uint8_t u8;`) with zero architectural
/// value.
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

/// Check if a `preproc_def` is a C/C++ header include guard (`#define FOO_H` with no value,
/// where the name contains the uppercased filename stem).
fn is_c_header_guard(node: tree_sitter::Node, source: &str, path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !crate::is_header_extension(ext) {
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
        Some(n) => n.utf8_text(source.as_bytes()).unwrap_or("").to_ascii_uppercase(),
        None => return false,
    };
    name.contains(&stem)
}

/// Check if a `preproc_def` is a platform-compatibility type define
/// (`#define UINT32_TYPE uint32_t`, `#define INT8_TYPE signed char`, etc.)
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
                && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
}
