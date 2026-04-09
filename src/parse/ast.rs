use crate::Lang;

/// Compute the signature end line (1-indexed) from the tree-sitter AST.
///
/// For C-like languages (Rust, TS/JS, Go, C), the body child (block,
/// field_declaration_list, etc.) starts at `{`, so `sig_end = body.start.row + 1`.
///
/// For Python, the body (block) starts at the first statement after `:`.
/// We find the `:` token that's a direct child of the function/class definition
/// and use its row.
///
/// For Go `type_spec`, the body braces are inside the `type` child (struct_type,
/// interface_type), so we look for `{` there.
///
/// Returns `None` for nodes without a detectable body (type aliases, constants,
/// method signatures, etc.) — the caller should fall back to text heuristics.
pub(super) fn compute_sig_end_line(node: tree_sitter::Node, lang: Lang) -> Option<usize> {
    if lang == Lang::Python {
        let body = node.child_by_field_name("body")?;
        let body_id = body.id();
        let mut body_idx = None;
        for i in 0..node.child_count() {
            if node.child(i).is_some_and(|c| c.id() == body_id) {
                body_idx = Some(i);
                break;
            }
        }
        if let Some(idx) = body_idx {
            for i in (0..idx).rev() {
                if let Some(child) = node.child(i)
                    && child.kind() == ":"
                {
                    return Some(child.start_position().row + 1);
                }
            }
        }
        return None;
    }

    // C-like languages: body field starts at '{'
    if let Some(body) = node.child_by_field_name("body") {
        return Some(body.start_position().row + 1);
    }

    // Go type_spec: the type child (struct_type, interface_type) contains the body.
    if node.kind() == "type_spec"
        && let Some(type_child) = node.child_by_field_name("type")
    {
        for i in 0..type_child.child_count() {
            if let Some(child) = type_child.child(i)
                && child.kind() == "{"
            {
                return Some(child.start_position().row + 1);
            }
        }
    }

    // C anonymous typedef struct/union: `typedef struct { ... } Name;`
    // Treat the entire definition as the signature so the name is always visible.
    if node.kind() == "type_definition"
        && let Some(type_child) = node.child_by_field_name("type")
        && matches!(type_child.kind(), "struct_specifier" | "union_specifier")
        && type_child.child_by_field_name("body").is_some()
        && type_child.child_by_field_name("name").is_none()
    {
        let end = node.end_position();
        return Some(if end.column == 0 { end.row } else { end.row + 1 });
    }

    None
}

/// Compute the first line (1-indexed) of the doc comment block preceding a symbol,
/// using tree-sitter AST sibling navigation. Returns `None` when no doc comment is
/// found (fallback to text heuristics in `layout::doc_comment_start`).
pub(super) fn compute_doc_start_line(symbol_node: tree_sitter::Node, source: &str, lang: Lang) -> Option<usize> {
    if matches!(lang, Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml) {
        return None;
    }

    let max_gap = 2;

    let first_doc = symbol_node
        .prev_named_sibling()
        .filter(|n| {
            is_doc_comment_node(*n, source, lang)
                && symbol_node.start_position().row.saturating_sub(n.end_position().row) <= max_gap
        })
        .or_else(|| {
            let parent = symbol_node.parent()?;
            if matches!(parent.kind(), "export_statement" | "decorated_definition") {
                parent.prev_named_sibling().filter(|n| {
                    is_doc_comment_node(*n, source, lang)
                        && parent.start_position().row.saturating_sub(n.end_position().row)
                            <= max_gap
                })
            } else {
                None
            }
        })?;

    // Walk backwards through contiguous doc comment siblings to find the block start
    let mut doc_start_row = first_doc.start_position().row;
    let mut current = first_doc;

    while let Some(prev) = current.prev_named_sibling() {
        if !is_doc_comment_node(prev, source, lang) {
            break;
        }
        if current.start_position().row.saturating_sub(prev.end_position().row) > 1 {
            break;
        }
        doc_start_row = prev.start_position().row;
        current = prev;
    }

    Some(doc_start_row + 1) // 1-indexed
}

/// Check whether a tree-sitter node is a doc comment for the given language.
fn is_doc_comment_node(node: tree_sitter::Node, source: &str, lang: Lang) -> bool {
    if !matches!(node.kind(), "comment" | "line_comment" | "block_comment") {
        return false;
    }
    let text = match node.utf8_text(source.as_bytes()) {
        Ok(t) => t.trim(),
        Err(_) => return false,
    };
    match lang {
        Lang::Rust => text.starts_with("///") || text.starts_with("/**"),
        Lang::Go | Lang::C => true,
        Lang::JsTs => text.starts_with("/**"),
        Lang::Python => text.starts_with('#'),
        _ => false,
    }
}

/// Check if a node is inside a function body (local declaration, not module-level).
pub(super) fn is_inside_function(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            // Rust
            "function_item" |
            // TypeScript / JavaScript
            "function_declaration" | "function_expression" | "method_definition" | "arrow_function"
            | "generator_function" | "generator_function_declaration" |
            // Go
            "method_declaration" | "func_literal" |
            // Java
            "constructor_declaration" | "lambda_expression" |
            // Python / Lua
            "function_definition" => {
                return true;
            }
            _ => {}
        }
        current = parent.parent();
    }
    false
}

/// Check if a node is inside a Rust trait implementation block (`impl Trait for Type`).
pub(super) fn is_in_trait_impl(node: tree_sitter::Node) -> bool {
    if let Some(parent) = node.parent()
        && parent.kind() == "declaration_list"
        && let Some(grandparent) = parent.parent()
        && grandparent.kind() == "impl_item"
        && grandparent.child_by_field_name("trait").is_some()
    {
        return true;
    }
    false
}

/// Check if a node is inside a Rust `const _: T = { ... }` block.
pub(super) fn is_inside_rust_anon_const(node: tree_sitter::Node, source: &str) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "const_item" {
            let mut cursor = parent.walk();
            let is_anon = parent.children(&mut cursor).any(|child| {
                child.kind() == "identifier"
                    && child.utf8_text(source.as_bytes()).unwrap_or("") == "_"
            });
            if is_anon {
                return true;
            }
        }
        current = parent.parent();
    }
    false
}

/// Check if a Rust node is test code that should be filtered from output.
/// Returns true for `#[test]` functions, `#[cfg(test)]` modules, and anything nested inside them.
pub(super) fn is_rust_test_code(node: tree_sitter::Node, source: &str) -> bool {
    if has_preceding_attribute(node, source, "#[test]")
        || has_preceding_attribute(node, source, "#[cfg(test)]")
    {
        return true;
    }
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "mod_item" && has_preceding_attribute(parent, source, "#[cfg(test)]") {
            return true;
        }
        if parent.kind() == "function_item" && has_preceding_attribute(parent, source, "#[test]") {
            return true;
        }
        current = parent.parent();
    }
    false
}

/// Check if a node's preceding attribute siblings include a specific attribute.
pub(super) fn has_preceding_attribute(node: tree_sitter::Node, source: &str, needle: &str) -> bool {
    let mut sibling = node.prev_sibling();
    while let Some(sib) = sibling {
        if sib.kind() == "attribute_item" {
            let text = sib.utf8_text(source.as_bytes()).unwrap_or("");
            if text == needle {
                return true;
            }
        } else {
            break;
        }
        sibling = sib.prev_sibling();
    }
    false
}

/// Recursively find the first descendant of the given kind in a subtree.
pub(super) fn find_descendant_of_kind<'a>(
    node: tree_sitter::Node<'a>,
    target_kind: &str,
) -> Option<tree_sitter::Node<'a>> {
    if node.kind() == target_kind {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(found) = find_descendant_of_kind(child, target_kind) {
            return Some(found);
        }
    }
    None
}
