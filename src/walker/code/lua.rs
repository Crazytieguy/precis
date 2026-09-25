//! Lua extraction. Declarations are the top-level function forms, all
//! `Callable` and public:
//!  - `function_declaration` (incl. `local function`),
//!  - `assignment_statement` / `variable_declaration` with a
//!    `function_definition` right-hand side, and
//!  - function-valued `field`s inside a top-level table-constructor
//!    right-hand side (the tables-as-classes idiom:
//!    `local M = { foo = function() ... end }`), surfaced one level deep
//!    so a `static = { ... }` sub-table resolves.
//!
//! The module doc is a top-of-file identity table (`_VERSION`, …).

use std::path::Path;

use tree_sitter::Node;

use super::SourceFile;
use super::model::{DeclInfo, FileModel, Item, Shape, Visibility};
use crate::walker::{WalkCtx, collect_doc_comments_above};

pub(super) const PORTED: bool = true;
pub(super) const EXTENSIONS: &[&str] = &["lua"];

pub(super) fn grammar(_path: &Path) -> tree_sitter::Language {
    tree_sitter_lua::LANGUAGE.into()
}

pub(super) fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let identity = module_identity_rows(file);
    FileModel {
        module_doc: if identity.is_empty() {
            Vec::new()
        } else {
            vec![Item::new(identity)]
        },
        reexports: Vec::new(),
        decls: find_decls(file.tree.root_node())
            .into_iter()
            .map(|node| decl_info(node, file))
            .collect(),
    }
}

pub(super) fn is_entrypoint(_path: &Path, _ctx: &WalkCtx) -> bool {
    false
}

pub(super) fn file_weight(_path: &Path, _ctx: &WalkCtx) -> f64 {
    1.0
}

#[derive(Default)]
pub(crate) struct RunState {}

/// Top-level fn-like declarations. Tables-as-classes
/// (`local M = { foo = function … }`) surface one nesting level deep.
fn find_decls(root: Node) -> Vec<Node> {
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "function_declaration" => out.push(child),
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
    let mut cursor = table.walk();
    for field in table.children(&mut cursor) {
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
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "expression_list" => {
                let mut inner = child.walk();
                for expr in child.children(&mut inner) {
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
    match node.kind() {
        "function_declaration" => node.child_by_field_name("body"),
        "assignment_statement" | "variable_declaration" => {
            rhs_of_kind(node, "function_definition")?.child_by_field_name("body")
        }
        "field" => {
            let value = node.child_by_field_name("value")?;
            (value.kind() == "function_definition")
                .then(|| value.child_by_field_name("body"))
                .flatten()
        }
        _ => None,
    }
}

/// Head: the declaration's start through the row before its body block
/// (the whole node when it has none). Body: one item per block statement,
/// leading comments included, past the head.
fn decl_info(node: Node, file: &SourceFile) -> DeclInfo {
    let rows = file.node_rows(node);
    let start = *rows.start();
    let block = body_block(node);
    let head_end = match block {
        Some(block) => block.start_position().row.max(start),
        None => *rows.end(),
    };
    let mut body = Vec::new();
    let mut pending = Vec::new();
    if let Some(block) = block {
        let mut cursor = block.walk();
        for statement in block.named_children(&mut cursor) {
            pending.extend(file.node_rows(statement).filter(|&row| row > head_end));
            if statement.kind() != "comment" && !pending.is_empty() {
                body.push(Item::new(std::mem::take(&mut pending)));
            }
        }
    }
    if !pending.is_empty() {
        body.push(Item::new(pending));
    }
    let doc = collect_doc_comments_above(node, &file.source).full;
    DeclInfo {
        name_rows: vec![start],
        head: (start..=head_end).collect(),
        doc: if doc.is_empty() {
            Vec::new()
        } else {
            vec![Item::new(doc)]
        },
        body,
        shape: Shape::Callable,
        visibility: Visibility::Public,
        members: Vec::new(),
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

/// Rows of a top-of-file module identity table: a
/// `local M = { _VERSION = …, _DESCRIPTION = … }` metadata block. The
/// table opener plus its leading single-line `_KEY = …` fields, stopping
/// before a long-string field (`[[`) or the table close, so a multi-line
/// `_LICENSE = [[ … ]]` body is excluded. Empty unless the table holds
/// at least one metadata key.
fn module_identity_rows(file: &SourceFile) -> Vec<usize> {
    let lines = file.line_count();
    let Some(opener) = (1..=lines).find(|&row| {
        let text = file.line(row).trim_start();
        !text.is_empty() && !text.starts_with("--")
    }) else {
        return Vec::new();
    };
    // Must open a table assignment: `local NAME = {` or `NAME = {`.
    let text = file.line(opener).trim_start();
    let assignee = text.strip_prefix("local ").unwrap_or(text);
    let opens_table = text.contains('{')
        && assignee.split('=').next().is_some_and(|name| {
            let name = name.trim();
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
        });
    if !opens_table {
        return Vec::new();
    }
    let mut out = vec![opener];
    out.extend((opener + 1..=lines).take_while(|&row| {
        let text = file.line(row).trim();
        !text.contains("[[") && !text.starts_with('}') && text.starts_with('_')
    }));
    let has_meta_key = out.iter().any(|&row| {
        IDENTITY_META_KEYS
            .iter()
            .any(|key| file.line(row).contains(key))
    });
    if has_meta_key { out } else { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walker::code::Language;

    fn extract_source(source: &str) -> FileModel {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.lua");
        std::fs::write(&path, source).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let file = SourceFile::parse(&path, Language::Lua, &ctx).unwrap();
        extract(&file, &ctx)
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
        let body: Vec<_> = foo.body.iter().map(|item| item.rows.clone()).collect();
        assert_eq!(body, [vec![3], vec![4, 5, 6, 7], vec![8]]);
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
