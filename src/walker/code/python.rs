//! Python extraction for the code engine.
//!
//! - **Declarations**: top-level `def` (`Callable`), `class` (`Whole`
//!   container whose methods are members), simple `NAME = …` assignments
//!   and `type X = …` aliases (`Whole`). A decorated definition's head
//!   starts at its first decorator; its name row is the `def` / `class`
//!   row.
//! - **Doc**: the docstring opening a `def` / `class` body.
//! - **Visibility**: a leading `_` that isn't a `__dunder__` is `Private`.
//!   Test files (`test_*.py`, `*_test.py`) are hidden entirely.
//! - **Module doc**: the module docstring and dunder assignments other
//!   than `__all__`. Leading `#` comments (shebangs, license headers) are
//!   not module doc.
//! - **Re-exports**: `__all__`, and in `__init__.py` every top-level
//!   `from … import …`.

use std::collections::HashSet;
use std::path::Path;

use tree_sitter::Node;

use super::SourceFile;
use super::model::{DeclInfo, FileModel, Item, Shape, Visibility};
use crate::walker::{WalkCtx, name_of};

pub(super) const PORTED: bool = true;
pub(super) const EXTENSIONS: &[&str] = &["py"];

pub(super) fn grammar(_path: &Path) -> tree_sitter::Language {
    tree_sitter_python::LANGUAGE.into()
}

pub(super) fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let mut model = FileModel::default();
    if is_test_file(&file.path) {
        return model;
    }
    let is_package_init = file_name(&file.path) == Some("__init__.py");
    let root = file.tree.root_node();
    let mut cursor = root.walk();
    let mut first_statement = true;
    for node in root.named_children(&mut cursor) {
        if node.kind() == "comment" {
            continue;
        }
        let is_first_statement = std::mem::replace(&mut first_statement, false);
        if is_first_statement && is_docstring(node) {
            model
                .module_doc
                .extend(paragraphs(file, file.node_rows(node), &HashSet::new()));
            continue;
        }
        let rows = || Item::new(file.node_rows(node));
        match node.kind() {
            "function_definition" | "class_definition" | "decorated_definition" => {
                model.decls.extend(definition(file, node, None));
            }
            "import_from_statement" if is_package_init => model.reexports.push(rows()),
            "type_alias_statement" => {
                if let Some(left) = node.child_by_field_name("left") {
                    let name = file.text(left).split('[').next().unwrap_or_default();
                    model
                        .decls
                        .push(whole_statement(file, node, name_visibility(name.trim())));
                }
            }
            "expression_statement" => match assignment_target(file, node) {
                Some(Assignment::Plain("__all__") | Assignment::Augmented("__all__")) => {
                    model.reexports.push(rows());
                }
                Some(Assignment::Plain(name)) if is_dunder(name) => model.module_doc.push(rows()),
                Some(Assignment::Plain(name)) => {
                    model
                        .decls
                        .push(whole_statement(file, node, name_visibility(name)));
                }
                Some(Assignment::Augmented(_)) | None => {}
            },
            _ => {}
        }
    }
    model
}

pub(super) fn is_entrypoint(path: &Path, _ctx: &WalkCtx) -> bool {
    matches!(file_name(path), Some("__init__.py" | "__main__.py"))
}

pub(super) fn file_weight(_path: &Path, _ctx: &WalkCtx) -> f64 {
    1.0
}

#[derive(Default)]
pub(crate) struct RunState {}

fn file_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(|name| name.to_str())
}

fn is_test_file(path: &Path) -> bool {
    file_name(path).is_some_and(|name| name.starts_with("test_") || name.ends_with("_test.py"))
}

fn is_dunder(name: &str) -> bool {
    name.len() >= 4 && name.starts_with("__") && name.ends_with("__")
}

fn name_visibility(name: &str) -> Visibility {
    if name.starts_with('_') && !is_dunder(name) {
        Visibility::Private
    } else {
        Visibility::Public
    }
}

enum Assignment<'a> {
    Plain(&'a str),
    Augmented(&'a str),
}

/// The single identifier an `expression_statement` assigns (`NAME = …`,
/// `NAME: T = …`, `NAME += …`); `None` for tuple, attribute, subscript or
/// chained targets and for non-assignments.
fn assignment_target<'a>(file: &'a SourceFile, statement: Node) -> Option<Assignment<'a>> {
    let mut cursor = statement.walk();
    let assignment = statement.named_children(&mut cursor).next()?;
    let left = assignment.child_by_field_name("left")?;
    if left.kind() != "identifier" {
        return None;
    }
    let name = file.text(left);
    match assignment.kind() {
        "assignment" => {
            let chained = assignment
                .child_by_field_name("right")
                .is_some_and(|right| right.kind() == "assignment");
            (!chained).then_some(Assignment::Plain(name))
        }
        "augmented_assignment" => Some(Assignment::Augmented(name)),
        _ => None,
    }
}

fn is_docstring(node: Node) -> bool {
    if node.kind() != "expression_statement" {
        return false;
    }
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .next()
        .is_some_and(|first| matches!(first.kind(), "string" | "concatenated_string"))
}

/// A constant or alias: all head, no body.
fn whole_statement(file: &SourceFile, node: Node, visibility: Visibility) -> DeclInfo {
    let rows = file.node_rows(node);
    DeclInfo {
        name_rows: vec![*rows.start()],
        head: rows.collect(),
        doc: Vec::new(),
        body: Vec::new(),
        shape: Shape::Whole,
        visibility,
        members: Vec::new(),
    }
}

/// A `def` or `class`, possibly wrapped in `decorated_definition`.
/// `container` is the enclosing class's visibility for a method; a class
/// inside a class is not a member and yields `None` there.
fn definition(file: &SourceFile, unit: Node, container: Option<Visibility>) -> Option<DeclInfo> {
    let inner = if unit.kind() == "decorated_definition" {
        unit.child_by_field_name("definition")?
    } else {
        unit
    };
    let shape = match (inner.kind(), container) {
        ("function_definition", _) => Shape::Callable,
        ("class_definition", None) => Shape::Whole,
        _ => return None,
    };
    let own = name_visibility(name_of(inner, &file.source).unwrap_or_default());
    let visibility = container.map_or(own, |container| container.min(own));
    let name_row = inner.start_position().row + 1;
    let head_end = colon_row(inner).max(name_row);
    let head: Vec<usize> = (unit.start_position().row + 1..=head_end).collect();
    let mut statements = suite_statements(inner);
    let mut excluded: HashSet<usize> = head.iter().copied().collect();
    let mut doc = Vec::new();
    if let Some(index) = statements
        .iter()
        .position(|node| node.kind() != "comment")
        .filter(|&index| is_docstring(statements[index]))
    {
        let docstring = statements.remove(index);
        doc = paragraphs(file, file.node_rows(docstring), &excluded);
        excluded.extend(file.node_rows(docstring));
    }
    let (body, members) = match shape {
        Shape::Callable => (statement_items(file, &statements, &excluded), Vec::new()),
        Shape::Whole => class_body(file, &statements, &excluded, visibility),
    };
    Some(DeclInfo {
        name_rows: vec![name_row],
        head,
        doc,
        body,
        shape,
        visibility,
        members,
    })
}

/// 1-based row of the `:` that ends a `def` / `class` header.
fn colon_row(inner: Node) -> usize {
    let body_start = inner
        .child_by_field_name("body")
        .map_or(usize::MAX, |body| body.start_byte());
    let mut cursor = inner.walk();
    inner
        .children(&mut cursor)
        .filter(|child| child.kind() == ":" && child.end_byte() <= body_start)
        .last()
        .map_or(inner.start_position().row, |colon| {
            colon.start_position().row
        })
        + 1
}

/// The statements and comments of a `def` / `class` suite in source
/// order: the body block's children plus any comment tree-sitter hangs
/// on the definition between its `:` and the block.
fn suite_statements(inner: Node) -> Vec<Node> {
    let Some(body) = inner.child_by_field_name("body") else {
        return Vec::new();
    };
    let header_end = inner
        .child_by_field_name("return_type")
        .or_else(|| inner.child_by_field_name("parameters"))
        .or_else(|| inner.child_by_field_name("superclasses"))
        .or_else(|| inner.child_by_field_name("name"))
        .map_or(inner.start_byte(), |node| node.end_byte());
    let mut cursor = inner.walk();
    let mut statements: Vec<Node> = inner
        .named_children(&mut cursor)
        .filter(|child| child.kind() == "comment" && child.start_byte() >= header_end)
        .collect();
    let mut cursor = body.walk();
    statements.extend(body.named_children(&mut cursor));
    statements.sort_by_key(|node| node.start_byte());
    statements
}

/// One [`Item`] per statement, each carrying the comments directly above
/// it; a comment starting on a row an earlier item already holds (a
/// trailing `# …`) joins that item, as do comments after the last
/// statement. Rows in `excluded` are left out.
fn statement_items(file: &SourceFile, statements: &[Node], excluded: &HashSet<usize>) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut claimed = excluded.clone();
    let mut pending: Vec<usize> = Vec::new();
    for node in statements {
        let rows = file.node_rows(*node);
        if node.kind() == "comment" && claimed.contains(rows.start()) {
            if let Some(last) = items.last_mut() {
                last.rows.extend(rows.filter(|row| claimed.insert(*row)));
            }
            continue;
        }
        pending.extend(rows.filter(|row| claimed.insert(*row)));
        if node.kind() != "comment" && !pending.is_empty() {
            items.push(Item::new(pending.drain(..)));
        }
    }
    match items.last_mut() {
        Some(last) => last.rows.append(&mut pending),
        None if !pending.is_empty() => items.push(Item::new(pending)),
        None => {}
    }
    items
}

/// A class suite (after its docstring): methods become members, listed in
/// the body by their name row; every other statement (fields, nested
/// classes, `if` blocks) is a body [`Item`] with the comments directly
/// above it. Comments directly above a method belong to no part.
fn class_body(
    file: &SourceFile,
    statements: &[Node],
    excluded: &HashSet<usize>,
    visibility: Visibility,
) -> (Vec<Item>, Vec<DeclInfo>) {
    let mut body = Vec::new();
    let mut members = Vec::new();
    let mut run: Vec<Node> = Vec::new();
    for node in statements {
        if let Some(member) = definition(file, *node, Some(visibility)) {
            let last_statement_end = run
                .iter()
                .rfind(|pending| pending.kind() != "comment")
                .map_or(0, |statement| *file.node_rows(*statement).end());
            while run.last().is_some_and(|pending| {
                pending.kind() == "comment"
                    && *file.node_rows(*pending).start() > last_statement_end
            }) {
                run.pop();
            }
            body.extend(statement_items(file, &run, excluded));
            run.clear();
            body.push(Item::new(member.name_rows.iter().copied()));
            members.push(member);
        } else {
            run.push(*node);
        }
    }
    body.extend(statement_items(file, &run, excluded));
    (body, members)
}

/// `rows` split at blank lines into one [`Item`] per paragraph, without
/// `excluded` rows. A paragraph holding only a closing `"""` / `'''`
/// joins the one before it.
fn paragraphs(
    file: &SourceFile,
    rows: impl IntoIterator<Item = usize>,
    excluded: &HashSet<usize>,
) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let flush = |current: &mut Vec<usize>, items: &mut Vec<Item>| {
        if current.is_empty() {
            return;
        }
        let closing_only = current
            .iter()
            .all(|row| matches!(file.line(*row).trim(), "\"\"\"" | "'''"));
        match items.last_mut() {
            Some(last) if closing_only => last.rows.append(current),
            _ => items.push(Item::new(current.drain(..))),
        }
    };
    for row in rows {
        if excluded.contains(&row) {
            continue;
        }
        if file.line(row).trim().is_empty() {
            flush(&mut current, &mut items);
        } else {
            current.push(row);
        }
    }
    flush(&mut current, &mut items);
    items
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::super::Language;
    use super::*;

    fn extract_source(relative_path: &str, source: &str) -> FileModel {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(relative_path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, source).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let file = SourceFile::parse(&path, Language::Python, &ctx).unwrap();
        extract(&file, &ctx)
    }

    fn rows(items: &[Item]) -> Vec<Vec<usize>> {
        items.iter().map(|item| item.rows.clone()).collect()
    }

    #[test]
    fn code_python_module_doc_skips_license_header_and_keeps_dunders() {
        let model = extract_source(
            "pkg/core.py",
            "\
# Copyright (c) 2020 Someone
# SPDX-License-Identifier: MIT
\"\"\"Core helpers.

Longer description.
\"\"\"
from __future__ import annotations
import os

__version__ = \"1.0\"
__all__ = [\"run\", \"Thing\"]
__all__ += [\"extra\"]
",
        );
        assert_eq!(rows(&model.module_doc), vec![vec![3], vec![5, 6], vec![10]]);
        assert_eq!(rows(&model.reexports), vec![vec![11], vec![12]]);
        assert!(model.decls.is_empty());
    }

    #[test]
    fn code_python_package_init_reexports_from_imports_only() {
        let source = "\
\"\"\"Package.\"\"\"
import sys
from .core import (
    run,
    Thing,
)
from . import util

if sys.version_info >= (3, 8):
    from .compat import shim
";
        let model = extract_source("pkg/__init__.py", source);
        assert_eq!(rows(&model.reexports), vec![vec![3, 4, 5, 6], vec![7]]);
        assert_eq!(rows(&model.module_doc), vec![vec![1]]);
        let plain = extract_source("pkg/mod.py", source);
        assert!(plain.reexports.is_empty());
    }

    #[test]
    fn code_python_decorated_function_splits_head_doc_and_body() {
        let model = extract_source(
            "cli.py",
            "\
@click.command()
@click.option(\"--name\")
def greet(
    name: str,
) -> None:
    \"\"\"Greet someone.

    Prints a line.
    \"\"\"
    # Build the message.
    message = f\"hi {name}\"
    print(message)  # say it
    # done
",
        );
        let [decl] = model.decls.as_slice() else {
            panic!("one decl: {:?}", model.decls);
        };
        assert_eq!(decl.shape, Shape::Callable);
        assert_eq!(decl.visibility, Visibility::Public);
        assert_eq!(decl.name_rows, vec![3]);
        assert_eq!(decl.head, vec![1, 2, 3, 4, 5]);
        assert_eq!(rows(&decl.doc), vec![vec![6], vec![8, 9]]);
        assert_eq!(rows(&decl.body), vec![vec![10, 11], vec![12, 13]]);
        assert!(decl.members.is_empty());
    }

    #[test]
    fn code_python_one_line_def_is_all_head() {
        let model = extract_source(
            "m.py",
            "\
def one(): return 1
def documented(): \"\"\"Doc on the def row.\"\"\"
def _helper(x):  # noqa
    return x
",
        );
        let summary: Vec<_> = model
            .decls
            .iter()
            .map(|decl| {
                (
                    decl.head.clone(),
                    decl.doc.len(),
                    rows(&decl.body),
                    decl.visibility,
                )
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (vec![1], 0, vec![], Visibility::Public),
                (vec![2], 0, vec![], Visibility::Public),
                (vec![3], 0, vec![vec![4]], Visibility::Private),
            ]
        );
    }

    #[test]
    fn code_python_class_is_container_with_methods_as_members() {
        let model = extract_source(
            "models.py",
            "\
@dataclass
class Config(Base):  # the config
    \"\"\"Runtime configuration.\"\"\"

    # Field comment.
    name: str = \"x\"
    retries: int = 3  # tries

    # Comment above a method belongs to no part.
    @property
    def label(self) -> str:
        \"\"\"The label.\"\"\"
        return self.name

    def __init__(self):
        pass

    def _reset(self): ...

    class Meta:
        ordering = [\"name\"]
",
        );
        let [class] = model.decls.as_slice() else {
            panic!("one decl: {:?}", model.decls);
        };
        assert_eq!(class.shape, Shape::Whole);
        assert_eq!(class.name_rows, vec![2]);
        assert_eq!(class.head, vec![1, 2]);
        assert_eq!(rows(&class.doc), vec![vec![3]]);
        assert_eq!(
            rows(&class.body),
            vec![
                vec![5, 6],
                vec![7],
                vec![11],
                vec![15],
                vec![18],
                vec![20, 21]
            ]
        );
        let members: Vec<_> = class
            .members
            .iter()
            .map(|member| {
                (
                    member.name_rows.clone(),
                    member.head.clone(),
                    rows(&member.doc),
                    rows(&member.body),
                    member.visibility,
                )
            })
            .collect();
        assert_eq!(
            members,
            vec![
                (
                    vec![11],
                    vec![10, 11],
                    vec![vec![12]],
                    vec![vec![13]],
                    Visibility::Public
                ),
                (
                    vec![15],
                    vec![15],
                    vec![],
                    vec![vec![16]],
                    Visibility::Public
                ),
                (vec![18], vec![18], vec![], vec![], Visibility::Private),
            ]
        );
        assert!(class.members.iter().all(|member| member.members.is_empty()));
    }

    #[test]
    fn code_python_private_class_members_are_private() {
        let model = extract_source(
            "m.py",
            "class _Impl:\n    def run(self):\n        pass\n    def __repr__(self):\n        return ''\n",
        );
        let [class] = model.decls.as_slice() else {
            panic!("one decl: {:?}", model.decls);
        };
        assert_eq!(class.visibility, Visibility::Private);
        assert!(
            class
                .members
                .iter()
                .all(|member| member.visibility == Visibility::Private)
        );
    }

    #[test]
    fn code_python_constants_and_aliases_are_whole_head_only() {
        let model = extract_source(
            "consts.py",
            "\
DEFAULTS = {
    \"a\": 1,
}
_CACHE: dict = {}
a, b = 1, 2
obj.attr = 3
COUNT += 1
type Alias[T] = list[T]
if TYPE_CHECKING:
    HIDDEN = 1
",
        );
        let summary: Vec<_> = model
            .decls
            .iter()
            .map(|decl| {
                (
                    decl.name_rows.clone(),
                    decl.head.clone(),
                    decl.shape,
                    decl.visibility,
                )
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (vec![1], vec![1, 2, 3], Shape::Whole, Visibility::Public),
                (vec![4], vec![4], Shape::Whole, Visibility::Private),
                (vec![8], vec![8], Shape::Whole, Visibility::Public),
            ]
        );
        assert!(model.decls.iter().all(|decl| decl.body.is_empty()));
    }

    #[test]
    fn code_python_test_files_are_hidden() {
        let source = "def test_it():\n    assert True\n";
        assert!(extract_source("test_core.py", source).decls.is_empty());
        assert!(extract_source("core_test.py", source).decls.is_empty());
        assert_eq!(extract_source("core.py", source).decls.len(), 1);
    }

    #[test]
    fn code_python_entrypoints_are_package_init_and_main() {
        let ctx = WalkCtx::new(PathBuf::from("/repo"));
        let entrypoint = |path: &str| is_entrypoint(Path::new(path), &ctx);
        assert!(entrypoint("/repo/pkg/__init__.py"));
        assert!(entrypoint("/repo/pkg/__main__.py"));
        assert!(!entrypoint("/repo/pkg/main.py"));
    }
}
