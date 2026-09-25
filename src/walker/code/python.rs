//! Python extraction for the code engine.
//!
//! - **Declarations**: top-level `def` (`Callable`), `class` (`Whole`
//!   container whose methods are members), simple `NAME = …` assignments
//!   and `type X = …` aliases (`Whole`). A decorated definition's head
//!   starts at its first decorator; its name row is the `def` / `class`
//!   row.
//! - **Doc**: the docstring opening a `def` / `class` body.
//! - **Module doc**: an entry file's (`__init__.py`, `__main__.py`)
//!   module docstring, and dunder assignments other than `__all__`.
//!   Other modules' docstrings and leading `#` comments (shebangs, license
//!   headers) are in no part.
//! - **Re-exports**: `__all__`, and in `__init__.py` every top-level
//!   `from … import …`.

use std::path::Path;

use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{Language, SourceFile};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["py"],
    grammar: |_| tree_sitter_python::LANGUAGE.into(),
    extract,
    is_entrypoint: Some(is_entrypoint),
    file_weight: None,
};

fn extract(file: &SourceFile, ctx: &WalkCtx) -> FileModel {
    let mut model = FileModel::default();
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
            if is_entrypoint(&file.path, ctx) {
                model
                    .module_doc
                    .extend(file.paragraphs(file.node_rows(node)));
            }
            continue;
        }
        let rows = || Item::new(file.node_rows(node));
        match node.kind() {
            "function_definition" | "class_definition" | "decorated_definition" => {
                model.decls.extend(definition(file, node, false));
            }
            "import_from_statement" if is_package_init => model.reexports.push(rows()),
            "type_alias_statement" => {
                if node.child_by_field_name("left").is_some() {
                    model.decls.push(whole_statement(file, node));
                }
            }
            "expression_statement" => match assignment_target(file, node) {
                Some(Assignment::Plain("__all__") | Assignment::Augmented("__all__")) => {
                    model.reexports.push(rows());
                }
                Some(Assignment::Plain(name)) if is_dunder(name) => model.module_doc.push(rows()),
                Some(Assignment::Plain(_)) => model.decls.push(whole_statement(file, node)),
                Some(Assignment::Augmented(_)) | None => {}
            },
            _ => {}
        }
    }
    model
}

fn is_entrypoint(path: &Path, _ctx: &WalkCtx) -> bool {
    matches!(file_name(path), Some("__init__.py" | "__main__.py"))
}

fn file_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(|name| name.to_str())
}

fn is_dunder(name: &str) -> bool {
    name.len() >= 4 && name.starts_with("__") && name.ends_with("__")
}

enum Assignment<'a> {
    Plain(&'a str),
    Augmented(&'a str),
}

/// The identifier an `expression_statement` assigns (`NAME = …`,
/// `NAME: T = …`, `NAME += …`, the first target of `NAME = OTHER = …`);
/// `None` for tuple, attribute or subscript targets and for non-assignments.
fn assignment_target<'a>(file: &'a SourceFile, statement: Node) -> Option<Assignment<'a>> {
    let mut cursor = statement.walk();
    let assignment = statement.named_children(&mut cursor).next()?;
    let left = assignment.child_by_field_name("left")?;
    if left.kind() != "identifier" {
        return None;
    }
    let name = file.text(left);
    match assignment.kind() {
        "assignment" => Some(Assignment::Plain(name)),
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
fn whole_statement(file: &SourceFile, node: Node) -> DeclInfo {
    let rows = file.node_rows(node);
    DeclInfo {
        name_rows: vec![*rows.start()],
        head: rows.collect(),
        doc: Vec::new(),
        body: Vec::new(),
        shape: Shape::Whole,
        members: Vec::new(),
    }
}

/// A `def` or `class`, possibly wrapped in `decorated_definition`.
/// A class inside a class (`in_class`) is not a member and yields `None`.
fn definition(file: &SourceFile, unit: Node, in_class: bool) -> Option<DeclInfo> {
    let inner = if unit.kind() == "decorated_definition" {
        unit.child_by_field_name("definition")?
    } else {
        unit
    };
    let shape = match (inner.kind(), in_class) {
        ("function_definition", _) => Shape::Callable,
        ("class_definition", false) => Shape::Whole,
        _ => return None,
    };
    let name_row = inner.start_position().row + 1;
    let head_end = colon_row(inner).max(name_row);
    let head: Vec<usize> = (unit.start_position().row + 1..=head_end).collect();
    let mut statements = suite_statements(inner);
    let mut after_row = head_end;
    let mut doc = Vec::new();
    if let Some(index) = statements
        .iter()
        .position(|node| node.kind() != "comment")
        .filter(|&index| is_docstring(statements[index]))
    {
        let docstring_rows = file.node_rows(statements.remove(index));
        after_row = after_row.max(*docstring_rows.end());
        doc = file.paragraphs(docstring_rows.filter(|&row| row > head_end));
    }
    let (body, members) = match shape {
        Shape::Callable => (file.node_items(statements, after_row), Vec::new()),
        Shape::Whole => class_body(file, &statements, after_row),
    };
    Some(DeclInfo {
        name_rows: vec![name_row],
        head,
        doc,
        body,
        shape,
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

/// A class suite (after its docstring): methods become members, listed in
/// the body by their name row; every other statement (fields, nested
/// classes, `if` blocks) is a body [`Item`] with the comments directly
/// above it. Comments directly above a method belong to no part.
fn class_body(
    file: &SourceFile,
    statements: &[Node],
    after_row: usize,
) -> (Vec<Item>, Vec<DeclInfo>) {
    let mut body = Vec::new();
    let mut members = Vec::new();
    let mut run: Vec<Node> = Vec::new();
    for node in statements {
        if let Some(member) = definition(file, *node, true) {
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
            body.extend(file.node_items(run.drain(..), after_row));
            body.push(Item::new(member.name_rows.iter().copied()));
            members.push(member);
        } else {
            run.push(*node);
        }
    }
    body.extend(file.node_items(run, after_row));
    (body, members)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::super::test_support::rows;
    use super::*;

    fn extract_source(relative_path: &str, source: &str) -> FileModel {
        super::super::test_support::extract_source(&LANGUAGE, relative_path, source)
    }

    #[test]
    fn code_python_module_doc_skips_license_header_and_keeps_dunders() {
        let source = "\
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
";
        let model = extract_source("pkg/__main__.py", source);
        assert_eq!(rows(&model.module_doc), vec![vec![3], vec![5, 6], vec![10]]);
        assert_eq!(rows(&model.reexports), vec![vec![11], vec![12]]);
        assert!(model.decls.is_empty());
        let module = extract_source("pkg/core.py", source);
        assert_eq!(rows(&module.module_doc), vec![vec![10]]);
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
        assert_eq!(decl.name_rows, vec![3]);
        assert_eq!(decl.head, vec![1, 2, 3, 4, 5]);
        assert_eq!(rows(&decl.doc), vec![vec![6], vec![8, 9]]);
        assert_eq!(rows(&decl.body), vec![vec![10, 11], vec![12], vec![13]]);
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
            .map(|decl| (decl.head.clone(), decl.doc.len(), rows(&decl.body)))
            .collect();
        assert_eq!(
            summary,
            vec![
                (vec![1], 0, vec![]),
                (vec![2], 0, vec![]),
                (vec![3], 0, vec![vec![4]]),
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
                )
            })
            .collect();
        assert_eq!(
            members,
            vec![
                (vec![11], vec![10, 11], vec![vec![12]], vec![vec![13]],),
                (vec![15], vec![15], vec![], vec![vec![16]],),
                (vec![18], vec![18], vec![], vec![]),
            ]
        );
        assert!(class.members.iter().all(|member| member.members.is_empty()));
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
            .map(|decl| (decl.name_rows.clone(), decl.head.clone(), decl.shape))
            .collect();
        assert_eq!(
            summary,
            vec![
                (vec![1], vec![1, 2, 3], Shape::Whole),
                (vec![4], vec![4], Shape::Whole),
                (vec![8], vec![8], Shape::Whole),
            ]
        );
        assert!(model.decls.iter().all(|decl| decl.body.is_empty()));
    }

    #[test]
    fn code_python_chained_assignment_is_one_statement() {
        let source = "\
__version__ = version = \"1.0\"
LIB = STATE = SUPPRESS = None
_first = second = 0
";
        let model = extract_source("pkg/__main__.py", source);
        assert_eq!(rows(&model.module_doc), vec![vec![1]]);
        let summary: Vec<_> = model.decls.iter().map(|decl| decl.head.clone()).collect();
        assert_eq!(summary, vec![vec![2], vec![3]]);
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
