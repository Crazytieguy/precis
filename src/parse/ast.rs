use crate::Lang;

/// Reject symbols whose parent chain crosses a callable scope boundary —
/// functions, methods, arrow functions, lambdas, generators. Item-level
/// containment (class methods dropped by an outer class, typedef-wrapped
/// struct fields, etc.) is handled separately by `filter_nested_items` —
/// this check is only for *uncaptured* scope boundaries that item-level
/// containment can't see, e.g. consts defined inside a
/// `describe('...', () => { ... })` callback.
pub(crate) fn is_inside_scope_boundary(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if matches!(
            parent.kind(),
            "function_item"
                | "function_declaration"
                | "function_expression"
                | "function_definition"
                | "method_definition"
                | "method_declaration"
                | "arrow_function"
                | "generator_function"
                | "generator_function_declaration"
                | "func_literal"
                | "constructor_declaration"
                | "lambda_expression"
        ) {
            return true;
        }
        current = parent.parent();
    }
    false
}

/// Returns the 0-indexed line where the body content begins (the line after
/// the opening `{` or `:` for Python).
pub(crate) fn compute_body_start_line(node: tree_sitter::Node, lang: Lang) -> Option<usize> {
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

    if let Some(body) = node.child_by_field_name("body") {
        return Some(body.start_position().row + 1);
    }

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

    if node.kind() == "type_definition"
        && let Some(type_child) = node.child_by_field_name("type")
        && matches!(type_child.kind(), "struct_specifier" | "union_specifier")
        && type_child.child_by_field_name("body").is_some()
        && type_child.child_by_field_name("name").is_none()
    {
        let end = node.end_position();
        return Some(if end.column == 0 {
            end.row
        } else {
            end.row + 1
        });
    }

    None
}

fn find_doc_before<'a>(
    node: tree_sitter::Node<'a>,
    source: &str,
    lang: Lang,
    max_gap: usize,
) -> Option<tree_sitter::Node<'a>> {
    let mut candidate = node.prev_named_sibling()?;
    while candidate.kind() == "attribute_item" {
        candidate = candidate.prev_named_sibling()?;
    }
    if is_doc_comment_node(candidate, source, lang)
        && node
            .start_position()
            .row
            .saturating_sub(candidate.end_position().row)
            <= max_gap
    {
        Some(candidate)
    } else {
        None
    }
}

pub(crate) fn compute_doc_start_line(
    symbol_node: tree_sitter::Node,
    source: &str,
    lang: Lang,
) -> Option<usize> {
    let (start, _end) = compute_doc_line_range(symbol_node, source, lang)?;
    Some(start)
}

/// Returns 1-indexed (start_line, end_line_exclusive) of the doc comment block.
/// The range excludes any non-doc lines (e.g. attributes) between the doc and the symbol.
pub(crate) fn compute_doc_line_range(
    symbol_node: tree_sitter::Node,
    source: &str,
    lang: Lang,
) -> Option<(usize, usize)> {
    if matches!(lang, Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml) {
        return None;
    }

    let max_gap = 2;

    let last_doc = find_doc_before(symbol_node, source, lang, max_gap).or_else(|| {
        let parent = symbol_node.parent()?;
        if matches!(
            parent.kind(),
            "export_statement"
                | "decorated_definition"
                | "type_declaration"
                | "const_declaration"
                | "var_declaration"
        ) {
            find_doc_before(parent, source, lang, max_gap)
        } else {
            None
        }
    })?;

    let end_pos = last_doc.end_position();
    let doc_end_row_excl = if end_pos.column == 0 {
        end_pos.row
    } else {
        end_pos.row + 1
    };
    let mut doc_start_row = last_doc.start_position().row;
    let mut current = last_doc;

    while let Some(prev) = current.prev_named_sibling() {
        if !is_doc_comment_node(prev, source, lang) {
            break;
        }
        if current
            .start_position()
            .row
            .saturating_sub(prev.end_position().row)
            > 1
        {
            break;
        }
        doc_start_row = prev.start_position().row;
        current = prev;
    }

    Some((doc_start_row + 1, doc_end_row_excl + 1))
}

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
        Lang::Go | Lang::C | Lang::Cpp => true,
        Lang::TypeScript | Lang::Tsx | Lang::Java => text.starts_with("/**"),
        Lang::Python => text.starts_with('#'),
        Lang::Lua => text.starts_with("---"),
        _ => false,
    }
}

pub(crate) fn is_inside_rust_anon_const(node: tree_sitter::Node, source: &str) -> bool {
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

pub(crate) fn is_rust_test_code(node: tree_sitter::Node, source: &str) -> bool {
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

pub(crate) fn has_preceding_attribute(node: tree_sitter::Node, source: &str, needle: &str) -> bool {
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

pub(crate) fn is_documented(node: tree_sitter::Node, source: &str, lang: Lang) -> bool {
    compute_doc_start_line(node, source, lang).is_some()
        || (lang == Lang::Python && has_python_docstring(node, source))
}

fn has_python_docstring(symbol_node: tree_sitter::Node, source: &str) -> bool {
    let Some(body) = symbol_node.child_by_field_name("body") else {
        return false;
    };
    if body.kind() != "block" {
        return false;
    }
    let Some(first_stmt) = body.named_child(0) else {
        return false;
    };
    if first_stmt.kind() != "expression_statement" {
        return false;
    }
    let Some(string_node) = first_stmt.named_child(0) else {
        return false;
    };
    if !matches!(string_node.kind(), "string" | "concatenated_string") {
        return false;
    }
    let Ok(text) = string_node.utf8_text(source.as_bytes()) else {
        return false;
    };
    text.starts_with("\"\"\"") || text.starts_with("'''")
}

/// For Python: returns 1-indexed (start_line, end_line) of docstring content (excluding quotes).
pub(crate) fn compute_python_docstring_range(
    symbol_node: tree_sitter::Node,
    source: &str,
) -> Option<(usize, usize)> {
    let body = symbol_node.child_by_field_name("body")?;
    let first_stmt = body.named_child(0)?;
    let string_node = first_stmt.named_child(0)?;
    let text = string_node.utf8_text(source.as_bytes()).ok()?;
    if !text.starts_with("\"\"\"") && !text.starts_with("'''") {
        return None;
    }

    let start_row = string_node.start_position().row;
    let end_row = string_node.end_position().row;
    let quote = if text.starts_with("\"\"\"") {
        "\"\"\""
    } else {
        "'''"
    };

    let start_line = source.lines().nth(start_row).unwrap_or("");
    let after_quotes = start_line
        .trim_start()
        .strip_prefix(quote)
        .map_or("", |s| s.trim());
    let content_start = if after_quotes.is_empty() || after_quotes == quote {
        start_row + 1
    } else {
        start_row
    };

    let end_line = source.lines().nth(end_row).unwrap_or("");
    let content_end = if end_line.trim() == quote {
        end_row
    } else {
        end_row + 1
    };

    if content_start >= content_end {
        return Some((start_row + 1, start_row + 2));
    }

    Some((content_start + 1, content_end))
}

pub(crate) fn find_descendant_of_kind<'a>(
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
