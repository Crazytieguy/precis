//! Lua extraction. Declarations are the top-level function forms, all
//! `Callable`:
//!  - `function_declaration` (incl. `local function`),
//!  - `assignment_statement` / `variable_declaration` / `return_statement`
//!    with a `function_definition` right-hand side, and
//!  - function-valued `field`s inside a top-level table-constructor
//!    right-hand side (the tables-as-classes idiom:
//!    `local M = { foo = function() ... end }`), surfaced one level deep
//!    so a `static = { ... }` sub-table resolves.
//!
//! The module doc is a top-of-file identity table (`_VERSION`, …). What
//! `require` returns (the top-level `return` and a `setmetatable(…)` call)
//! joins the roster as re-export rows; function-valued fields of the
//! tables it hands out are `Callable` declarations instead.

use std::collections::HashSet;

use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item};
use super::{Language, SourceFile, named_children};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["lua"],
    grammar: |_| tree_sitter_lua::LANGUAGE.into(),
    extract,
    is_entrypoint: None,
    file_weight: None,
    sibling_mentions: None,
    sibling_names: None,
};

fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let identity = module_identity_rows(file);
    let exports = module_export_nodes(file);
    let mut export_fields = Vec::new();
    for export in &exports {
        for table in export_tables(*export) {
            collect_function_fields(table, &mut export_fields, 1);
        }
    }
    let field_rows: HashSet<usize> = export_fields
        .iter()
        .flat_map(|field| file.node_rows(*field))
        .collect();
    let mut decls = find_decls(file.tree.root_node());
    decls.extend(export_fields);
    decls.sort_by_key(Node::start_byte);
    FileModel {
        module_doc: if identity.is_empty() {
            Vec::new()
        } else {
            vec![Item::new(identity)]
        },
        reexports: exports
            .iter()
            .flat_map(|export| file.node_rows(*export))
            .filter(|row| !field_rows.contains(row))
            .map(|row| Item::new([row]))
            .collect(),
        decls: decls.into_iter().map(|node| callable(node, file)).collect(),
        ..FileModel::default()
    }
}

/// What `require` returns: the top-level `return`, and a
/// `setmetatable(…)` call that makes the module callable. Their rows join
/// the roster one item per row, except function-valued fields of their
/// tables, which are declarations.
fn module_export_nodes<'a>(file: &'a SourceFile) -> Vec<Node<'a>> {
    let root = file.tree.root_node();
    root.children(&mut root.walk())
        .filter(|child| {
            (child.kind() == "return_statement"
                && rhs_of_kind(*child, "function_definition").is_none())
                || (child.kind() == "function_call"
                    && file.text(*child).starts_with("setmetatable"))
        })
        .collect()
}

/// Table constructors an export hands out: returned tables, and the
/// table arguments of a returned or top-level call.
fn export_tables(export: Node) -> Vec<Node> {
    let mut tables = Vec::new();
    let mut pending = vec![export];
    while let Some(node) = pending.pop() {
        let expressions = match node.kind() {
            "table_constructor" => {
                tables.push(node);
                continue;
            }
            "function_call" => node.child_by_field_name("arguments"),
            _ => node
                .children(&mut node.walk())
                .find(|child| child.kind() == "expression_list"),
        };
        let Some(expressions) = expressions else {
            continue;
        };
        let first = pending.len();
        pending.extend(
            expressions
                .named_children(&mut expressions.walk())
                .filter(|expression| {
                    matches!(expression.kind(), "table_constructor" | "function_call")
                }),
        );
        pending[first..].reverse();
    }
    tables
}

/// Top-level fn-like declarations. Tables-as-classes
/// (`local M = { foo = function … }`) surface one nesting level deep.
fn find_decls(root: Node) -> Vec<Node> {
    let mut out = Vec::new();
    for child in root.children(&mut root.walk()) {
        match child.kind() {
            "function_declaration" => out.push(child),
            "return_statement" if rhs_of_kind(child, "function_definition").is_some() => {
                out.push(child)
            }
            "variable_declaration" | "assignment_statement" => {
                if rhs_of_kind(child, "function_definition").is_some() {
                    out.push(child);
                } else if let Some(table) = rhs_of_kind(child, "table_constructor") {
                    collect_function_fields(table, &mut out, 1);
                }
            }
            _ => {}
        }
    }
    out
}

/// Fields with function values from a table constructor, recursing into
/// nested constructors up to `remaining_depth` more levels.
fn collect_function_fields<'a>(table: Node<'a>, out: &mut Vec<Node<'a>>, remaining_depth: u8) {
    for field in table.children(&mut table.walk()) {
        if field.kind() != "field" {
            continue;
        }
        let Some(value) = field.child_by_field_name("value") else {
            continue;
        };
        match value.kind() {
            "function_definition" => out.push(field),
            "table_constructor" if remaining_depth > 0 => {
                // Only surface the parent if a descendant would surface.
                let before = out.len();
                collect_function_fields(value, out, remaining_depth - 1);
                if out.len() > before {
                    out.push(field);
                }
            }
            _ => {}
        }
    }
}

/// The node of `kind` that's the right-hand side of an
/// `assignment_statement` / `variable_declaration`.
fn rhs_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    for child in node.children(&mut node.walk()) {
        match child.kind() {
            "expression_list" => {
                for expr in child.children(&mut child.walk()) {
                    if expr.kind() == kind {
                        return Some(expr);
                    }
                }
            }
            "assignment_statement" => {
                if let Some(found) = rhs_of_kind(child, kind) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

/// The function body's `block` node for any declaration shape.
fn body_block(node: Node) -> Option<Node> {
    let function = match node.kind() {
        "function_declaration" => node,
        "field" => node
            .child_by_field_name("value")
            .filter(|value| value.kind() == "function_definition")?,
        _ => rhs_of_kind(node, "function_definition")?,
    };
    function.child_by_field_name("body")
}

/// Head: the declaration's start through the row before its body block
/// (the whole node when it has none). Body: one item per block statement,
/// leading comments included, past the head.
fn callable(node: Node, file: &SourceFile) -> DeclInfo {
    let rows = file.node_rows(node);
    let start = *rows.start();
    let statements = named_children(body_block(node));
    DeclInfo {
        doc: file.comment_paragraphs_above(node),
        ..file.callable(vec![start], rows, statements, start)
    }
}

/// Identity-table metadata keys: the standard Lua library convention
/// (`local M = { _VERSION = …, _DESCRIPTION = … }`).
const IDENTITY_META_KEYS: [&str; 7] = [
    "_VERSION",
    "_DESCRIPTION",
    "_NAME",
    "_URL",
    "_AUTHOR",
    "_LICENSE",
    "_COPYRIGHT",
];

/// Rows of a top-of-file module identity table
/// (`local M = { _VERSION = …, _DESCRIPTION = … }`): the assignment's
/// first row and the table's leading one-row `_KEY = …` fields, so a
/// multi-line `_LICENSE = [[ … ]]` body is excluded. Empty unless one of
/// those fields is a metadata key.
fn module_identity_rows(file: &SourceFile) -> Vec<usize> {
    let root = file.tree.root_node();
    let Some((statement, table)) = root
        .named_children(&mut root.walk())
        .find(|node| node.kind() != "comment")
        .filter(|node| matches!(node.kind(), "variable_declaration" | "assignment_statement"))
        .and_then(|statement| Some((statement, rhs_of_kind(statement, "table_constructor")?)))
    else {
        return Vec::new();
    };
    let mut rows = vec![*file.node_rows(statement).start()];
    let mut has_meta_key = false;
    for field in table.named_children(&mut table.walk()) {
        let field_rows = file.node_rows(field);
        let Some(key) = field
            .child_by_field_name("name")
            .map(|name| file.text(name))
        else {
            break;
        };
        if !key.starts_with('_') || field_rows.start() != field_rows.end() {
            break;
        }
        has_meta_key |= IDENTITY_META_KEYS.contains(&key);
        rows.push(*field_rows.start());
    }
    if has_meta_key { rows } else { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::rows;
    use super::*;

    fn extract_source(source: &str) -> FileModel {
        super::super::test_support::extract_source(&LANGUAGE, "a.lua", source)
    }

    fn name_rows(model: &FileModel) -> Vec<usize> {
        let mut rows: Vec<usize> = model.decls.iter().map(|decl| decl.name_rows[0]).collect();
        rows.sort_unstable();
        rows
    }

    #[test]
    fn lua_finds_every_function_form() {
        let model = extract_source(
            "\
local M = {
  foo = function(self) return 1 end,
  static = {
    baz = function(self) return 3 end,
  },
  data = 42,
}
function M.bar(x) return x end
M.qux = function(x) return x end
local function helper(x)
  return x
end
return M
",
        );
        assert_eq!(name_rows(&model), [2, 3, 4, 8, 9, 10]);
    }

    #[test]
    fn lua_roster_lists_what_require_returns() {
        let model = extract_source(
            "\
local M = {}
function M.new() end
setmetatable(M, {
  __call = function(_, ...) return M.new(...) end,
})
local helper = 1
return M
",
        );
        assert_eq!(rows(&model.reexports), [vec![3], vec![5], vec![7]]);
        assert_eq!(name_rows(&model), [2, 4]);

        let returns_table = extract_source(
            "\
local function run(x)
  return x
end
return {
  run = run,
  stop = function(reason)
    print(reason)
    return false
  end,
}
",
        );
        assert_eq!(rows(&returns_table.reexports), [vec![4], vec![5], vec![10]]);
        let stop = &returns_table.decls[1];
        assert_eq!(stop.head, [6]);
        assert_eq!(stop.body, [Item::new([7]), Item::new([8])]);

        let returns_callable = extract_source(
            "\
return setmetatable({}, {
  __call = function(_, x)
    return x
  end,
})
",
        );
        assert_eq!(returns_callable.reexports, [Item::new([1]), Item::new([5])]);
        assert_eq!(returns_callable.decls[0].body, [Item::new([3])]);

        let returns_function = extract_source(
            "\
return function(title, f)
  f()
end
",
        );
        assert!(returns_function.reexports.is_empty());
        assert_eq!(returns_function.decls[0].head, [1]);
        assert_eq!(returns_function.decls[0].body, [Item::new([2])]);
    }

    #[test]
    fn lua_splits_head_body_and_doc() {
        let model = extract_source(
            "\
-- Adds one.
local function foo(x)
  local y = x + 1
  -- clamp
  if y > 2 then
    y = y - 1
  end
  return y
end
function M.version() return \"1.0\" end
",
        );
        let foo = &model.decls[0];
        assert_eq!(foo.head, [2]);
        assert_eq!(foo.doc, [Item::new([1])]);
        assert_eq!(rows(&foo.body), [vec![3], vec![4, 5, 6, 7], vec![8]]);
        let one_line = &model.decls[1];
        assert_eq!(
            (one_line.head.as_slice(), one_line.body.len()),
            (&[10][..], 0)
        );
    }

    #[test]
    fn lua_module_identity_stops_before_long_strings() {
        let model = extract_source(
            "\
local middleclass = {
  _VERSION     = 'middleclass v4.1.1',
  _DESCRIPTION = 'Object Orientation for Lua',
  _LICENSE     = [[
    MIT LICENSE
  ]]
}
",
        );
        assert_eq!(model.module_doc, [Item::new([1, 2, 3])]);
        assert!(
            extract_source("local cfg = {\n  host = 'x',\n}\n")
                .module_doc
                .is_empty()
        );
        assert!(
            extract_source("return {\n  _VERSION = '1.0',\n}\n")
                .module_doc
                .is_empty()
        );
    }
}
