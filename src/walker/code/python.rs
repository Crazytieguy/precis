//! Python extraction for the code engine, for modules (`.py`) and type
//! stubs (`.pyi`) alike.
//!
//! - **Declarations**: top-level `def` (`Callable`), `class` (`Whole`
//!   container whose methods are members), simple `NAME = …` assignments
//!   and `type X = …` aliases (`Whole`), plus the `def`s and `class`es
//!   inside top-level `if` / `try` statements. An `@overload` stub whose
//!   implementation follows it is hidden: the implementation's signature
//!   stands for the function, so the roster doesn't repeat its name.
//! - **Head and name rows**: a decorated definition's head starts at its
//!   first decorator. Its name rows are its one-row decorators
//!   (`@property` and `@overload` say what a `def` is), the `def` /
//!   `class` row and, when the signature spans rows, the row that closes
//!   it (`) -> T:`), so a roster never lists an unclosed `def f(`.
//! - **Doc**: the docstring opening a `def` / `class` body, or else the
//!   `#` comments directly above the definition.
//! - **Module doc**: an entry file's (a dunder-named module:
//!   `__init__.py`, `__main__.py`, `__version__.py`) module docstring and
//!   dunder assignments other than `__all__`. Other modules' docstrings and
//!   leading `#` comments (shebangs, license headers) are in no part, and
//!   their dunder assignments are ordinary constants.
//! - **Re-exports**: `__all__`, and in `__init__.py` every top-level
//!   `from … import …` of the package's own modules (a relative import, or
//!   one whose module path starts at a directory above the file) or that
//!   explicitly re-exports a name (`import X as X`, or a name `__all__`
//!   lists). Other imports from the standard library and third-party
//!   packages are in no part.

use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{Language, SourceFile};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["py", "pyi"],
    grammar: |_| tree_sitter_python::LANGUAGE.into(),
    extract,
    is_entrypoint: Some(is_entrypoint),
    file_weight: Some(file_weight),
    sibling_mentions: Some(sibling_mentions),
    sibling_names: None,
};

/// Every word of the file's `import` and `from … import` lines: module
/// path components and imported names, which include its sibling modules.
fn sibling_mentions(file: &SourceFile) -> HashSet<String> {
    static IMPORT: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?m)^[ \t]*(?:from|import)[ \t].*").unwrap());
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\w+").unwrap());
    IMPORT
        .find_iter(&file.source)
        .flat_map(|line| WORD.find_iter(line.as_str()))
        .map(|word| word.as_str().to_owned())
        .collect()
}

fn extract(file: &SourceFile, ctx: &WalkCtx) -> FileModel {
    let mut model = FileModel::default();
    let is_package_init = file_stem(&file.path) == Some("__init__");
    let exported_names = if is_package_init {
        all_names(file)
    } else {
        Vec::new()
    };
    let is_entry = is_entrypoint(&file.path, ctx);
    let root = file.tree.root_node();
    let mut cursor = root.walk();
    let mut first_statement = true;
    for node in root.named_children(&mut cursor) {
        if node.kind() == "comment" {
            continue;
        }
        let is_first_statement = std::mem::replace(&mut first_statement, false);
        if is_first_statement && is_docstring(node) {
            if is_entry {
                model
                    .module_doc
                    .extend(file.paragraphs(file.node_rows(node)));
            }
            continue;
        }
        let rows = || Item::new(file.node_rows(node));
        match node.kind() {
            "function_definition" | "class_definition" | "decorated_definition" => {
                model.decls.extend(definition(file, node));
            }
            "if_statement" | "try_statement" => {
                model.decls.extend(conditional_definitions(file, node));
            }
            "import_from_statement"
                if is_package_init
                    && (imports_own_module(file, node, ctx)
                        || reexports_explicitly(file, node, &exported_names)) =>
            {
                model.reexports.push(rows())
            }
            "type_alias_statement" => {
                if node.child_by_field_name("left").is_some() {
                    model.decls.push(constant_or_alias(file, node));
                }
            }
            "expression_statement" => match assignment_target(file, node) {
                Some(("__all__", _)) => model.reexports.push(rows()),
                Some((name, false)) if is_entry && is_dunder(name) => model.module_doc.push(rows()),
                Some((_, false)) => model.decls.push(constant_or_alias(file, node)),
                Some((_, true)) | None => {}
            },
            _ => {}
        }
    }
    model
}

fn is_entrypoint(path: &Path, _ctx: &WalkCtx) -> bool {
    file_stem(path).is_some_and(is_dunder)
}

/// A module a package `__init__.py` above it imports names from
/// (`from .core import Engine`, `from .engine.core import Engine`,
/// `from pkg.engine.core import Engine`) implements a package's public API,
/// so it outranks its sibling helper modules. The `from . import core`
/// form doesn't count: adding it measured -0.0001 avg7 (2026-09-25).
fn file_weight(path: &Path, ctx: &WalkCtx) -> f64 {
    let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
        return 1.0;
    };
    let mut module_path = vec![stem];
    let mut inits = Vec::new();
    let mut dir = path.parent();
    while let Some(package_dir) = dir.filter(|dir| dir.starts_with(ctx.root()))
        && let Some(init) = ctx.read_source(&package_dir.join("__init__.py"))
    {
        inits.push((module_path.len(), init));
        let Some(package) = package_dir.file_name().and_then(|s| s.to_str()) else {
            break;
        };
        module_path.insert(0, package);
        dir = package_dir.parent();
    }
    let absolute = format!("from {} import", module_path.join("."));
    let imported_by_an_init = inits.iter().any(|(components_below, init)| {
        let relative = format!(
            "from .{} import",
            module_path[module_path.len() - components_below..].join(".")
        );
        init.lines()
            .map(str::trim_start)
            .any(|line| line.starts_with(&relative) || line.starts_with(&absolute))
    });
    if imported_by_an_init {
        PUBLIC_API_MODULE_WEIGHT
    } else {
        1.0
    }
}

const PUBLIC_API_MODULE_WEIGHT: f64 = 1.2;

/// Whether a `from … import …` names a module of the file's own package:
/// a relative module path, or a dotted one whose first component names a
/// directory between the walk root and the file (namespace packages have
/// no `__init__.py`).
fn imports_own_module(file: &SourceFile, statement: Node, ctx: &WalkCtx) -> bool {
    let Some(module) = statement.child_by_field_name("module_name") else {
        return false;
    };
    if module.kind() == "relative_import" {
        return true;
    }
    let first_component = file.text(module).split('.').next();
    file.path
        .ancestors()
        .skip(1)
        .take_while(|dir| *dir != ctx.root() && dir.starts_with(ctx.root()))
        .any(|dir| dir.file_name().and_then(|name| name.to_str()) == first_component)
}

/// Whether a `from … import …` re-exports a name on purpose: `X as X`, or
/// a name `__all__` lists.
fn reexports_explicitly(file: &SourceFile, statement: Node, exported_names: &[&str]) -> bool {
    let mut cursor = statement.walk();
    statement
        .children_by_field_name("name", &mut cursor)
        .any(|name| match name.kind() {
            "aliased_import" => {
                name.child_by_field_name("name")
                    .map(|original| file.text(original))
                    == name
                        .child_by_field_name("alias")
                        .map(|alias| file.text(alias))
            }
            _ => exported_names.contains(&file.text(name)),
        })
}

/// The string contents of the module's top-level `__all__` assignments.
fn all_names(file: &SourceFile) -> Vec<&str> {
    let root = file.tree.root_node();
    let mut cursor = root.walk();
    let mut names = Vec::new();
    for statement in root.named_children(&mut cursor) {
        if statement.kind() == "expression_statement"
            && matches!(assignment_target(file, statement), Some(("__all__", _)))
        {
            collect_string_contents(file, statement, &mut names);
        }
    }
    names
}

fn collect_string_contents<'a>(file: &'a SourceFile, node: Node, names: &mut Vec<&'a str>) {
    if node.kind() == "string_content" {
        names.push(file.text(node));
        return;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_string_contents(file, child, names);
    }
}

fn file_stem(path: &Path) -> Option<&str> {
    path.file_stem().and_then(|stem| stem.to_str())
}

fn is_dunder(name: &str) -> bool {
    name.len() >= 4 && name.starts_with("__") && name.ends_with("__")
}

/// The identifier an `expression_statement` assigns (`NAME = …`,
/// `NAME: T = …`, `NAME += …`, the first target of `NAME = OTHER = …`)
/// and whether the assignment is augmented; `None` for tuple, attribute
/// or subscript targets and for non-assignments.
fn assignment_target<'a>(file: &'a SourceFile, statement: Node) -> Option<(&'a str, bool)> {
    let mut cursor = statement.walk();
    let assignment = statement.named_children(&mut cursor).next()?;
    let left = assignment.child_by_field_name("left")?;
    if left.kind() != "identifier" {
        return None;
    }
    let is_augmented = match assignment.kind() {
        "assignment" => false,
        "augmented_assignment" => true,
        _ => return None,
    };
    Some((file.text(left), is_augmented))
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
fn constant_or_alias(file: &SourceFile, node: Node) -> DeclInfo {
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

/// The `def`s and `class`es in the blocks of an `if` / `try` statement,
/// nested ones included. The row opening each block (`if …:`, `else:`,
/// `except …:`) joins the head and name rows of the block's first
/// definition, so the roster says under which condition it exists.
fn conditional_definitions(file: &SourceFile, statement: Node) -> Vec<DeclInfo> {
    let mut decls = Vec::new();
    for block in clause_blocks(statement) {
        let first = decls.len();
        let mut cursor = block.walk();
        for node in block.named_children(&mut cursor) {
            match node.kind() {
                "function_definition" | "class_definition" | "decorated_definition" => {
                    decls.extend(definition(file, node));
                }
                "if_statement" | "try_statement" => {
                    decls.extend(conditional_definitions(file, node));
                }
                _ => {}
            }
        }
        if let (Some(decl), Some(clause)) = (decls.get_mut(first), block.parent()) {
            let opening_row = clause.start_position().row + 1;
            decl.head.insert(0, opening_row);
            decl.name_rows.insert(0, opening_row);
        }
    }
    decls
}

/// An `if` / `try` statement's blocks: its own (`if` consequence, `try`
/// body) and those of its `elif` / `else` / `except` / `finally` clauses.
fn clause_blocks(statement: Node) -> Vec<Node> {
    let mut blocks = Vec::new();
    let mut cursor = statement.walk();
    for child in statement.named_children(&mut cursor) {
        if child.kind() == "block" {
            blocks.push(child);
            continue;
        }
        let mut clause_cursor = child.walk();
        blocks.extend(
            child
                .named_children(&mut clause_cursor)
                .filter(|grandchild| grandchild.kind() == "block"),
        );
    }
    blocks
}

/// A `def` or `class`, possibly wrapped in `decorated_definition`;
/// `None` for an `@overload` stub whose implementation follows it.
fn definition(file: &SourceFile, unit: Node) -> Option<DeclInfo> {
    let inner = defined(unit)?;
    if is_overload(file, unit) && has_implementation_after(file, unit) {
        return None;
    }
    let shape = match inner.kind() {
        "function_definition" => Shape::Callable,
        "class_definition" => Shape::Whole,
        _ => return None,
    };
    let name_row = inner.start_position().row + 1;
    let head_end = colon_row(inner).max(name_row);
    let head: Vec<usize> = (unit.start_position().row + 1..=head_end).collect();
    let mut statements = suite_statements(inner);
    let mut doc = Vec::new();
    if let Some(index) = statements
        .iter()
        .position(|node| node.kind() != "comment")
        .filter(|&index| is_docstring(statements[index]))
    {
        let docstring_rows = file.node_rows(statements.remove(index));
        doc = file.paragraphs(docstring_rows.filter(|&row| row > head_end));
    }
    if doc.is_empty() {
        doc = file.comment_paragraphs_above(unit);
    }
    let (body, members) = match shape {
        Shape::Callable => (file.node_items(statements, head_end), Vec::new()),
        Shape::Whole => class_body(file, &statements, head_end),
    };
    let mut name_rows = single_row_decorators(file, unit);
    name_rows.push(name_row);
    if head_end > name_row {
        name_rows.push(head_end);
    }
    Some(DeclInfo {
        name_rows,
        head,
        doc,
        body,
        shape,
        members,
    })
}

/// The `def` / `class` node of a possibly decorated definition.
fn defined(unit: Node) -> Option<Node> {
    if unit.kind() == "decorated_definition" {
        unit.child_by_field_name("definition")
    } else {
        Some(unit)
    }
}

fn is_overload(file: &SourceFile, unit: Node) -> bool {
    let mut cursor = unit.walk();
    unit.named_children(&mut cursor)
        .filter(|child| child.kind() == "decorator")
        .filter_map(|decorator| decorator.named_child(0))
        .any(|expression| {
            let text = file.text(expression);
            text == "overload" || text.ends_with(".overload")
        })
}

/// Whether a later sibling defines the same name without `@overload`.
fn has_implementation_after(file: &SourceFile, unit: Node) -> bool {
    let name = |node: Node| {
        defined(node)
            .and_then(|inner| inner.child_by_field_name("name"))
            .map(|name| file.text(name))
    };
    let Some(overloaded) = name(unit) else {
        return false;
    };
    std::iter::successors(unit.next_named_sibling(), |node| node.next_named_sibling())
        .any(|sibling| name(sibling) == Some(overloaded) && !is_overload(file, sibling))
}

/// Rows of the decorators above a definition that each fit on one row.
fn single_row_decorators(file: &SourceFile, unit: Node) -> Vec<usize> {
    let mut cursor = unit.walk();
    unit.named_children(&mut cursor)
        .filter(|child| child.kind() == "decorator")
        .map(|decorator| file.node_rows(decorator))
        .filter(|rows| rows.start() == rows.end())
        .map(|rows| *rows.start())
        .collect()
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
/// the body by their name rows; every other statement (fields, `if`
/// blocks) is a body [`Item`] with the comments directly above it. A
/// nested class is flattened into the suite: its head is one item, its
/// doc, fields and member name rows follow, and its methods become
/// members, so it lists as a roster rather than as its whole source.
/// Comments directly above a method or nested class are its doc when it
/// has no docstring, and otherwise in no part.
fn class_body(
    file: &SourceFile,
    statements: &[Node],
    after_row: usize,
) -> (Vec<Item>, Vec<DeclInfo>) {
    let mut body = Vec::new();
    let mut members = Vec::new();
    let mut run: Vec<Node> = Vec::new();
    for node in statements {
        if !matches!(
            node.kind(),
            "function_definition" | "class_definition" | "decorated_definition"
        ) {
            run.push(*node);
            continue;
        }
        let Some(member) = definition(file, *node) else {
            continue;
        };
        let last_statement_end = run
            .iter()
            .rfind(|pending| pending.kind() != "comment")
            .map_or(0, |statement| *file.node_rows(*statement).end());
        while run.last().is_some_and(|pending| {
            pending.kind() == "comment" && *file.node_rows(*pending).start() > last_statement_end
        }) {
            run.pop();
        }
        body.extend(file.node_items(run.drain(..), after_row));
        match member.shape {
            Shape::Callable => {
                body.push(Item::new(member.name_rows.iter().copied()));
                members.push(member);
            }
            Shape::Whole => {
                body.push(Item::new(member.head));
                body.extend(member.doc);
                body.extend(member.body);
                members.extend(member.members);
            }
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
        assert!(module.module_doc.is_empty());
        let constants: Vec<_> = module.decls.iter().map(|decl| decl.head.clone()).collect();
        assert_eq!(constants, vec![vec![10]]);
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
    fn code_python_package_init_reexports_only_its_own_modules() {
        let model = extract_source(
            "src/pkg/markdown/__init__.py",
            "\
from __future__ import annotations
from typing import TYPE_CHECKING, Any
from markupsafe import Markup
from pkg.markdown.config import Config
from .props import Params
from pkgextra import helper
",
        );
        assert_eq!(rows(&model.reexports), vec![vec![4], vec![5]]);
    }

    #[test]
    fn code_python_package_init_reexports_explicitly_exported_imports() {
        let model = extract_source(
            "pkg/__init__.pyi",
            "\
from typing import Any
from werkzeug.exceptions import abort as abort
from markupsafe import Markup, escape
from json import dumps as to_json
__all__ = [\"escape\"]
",
        );
        assert_eq!(rows(&model.reexports), vec![vec![2], vec![3], vec![5]]);
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
        assert_eq!(decl.name_rows, vec![1, 2, 3, 5]);
        assert_eq!(decl.head, vec![1, 2, 3, 4, 5]);
        assert_eq!(rows(&decl.doc), vec![vec![6], vec![8, 9]]);
        assert_eq!(rows(&decl.body), vec![vec![10, 11], vec![12], vec![13]]);
        assert!(decl.members.is_empty());
    }

    #[test]
    fn code_python_comment_above_docstring_joins_first_statement() {
        let model = extract_source(
            "m.py",
            "\
def run():
    # pylint: disable=broad-except
    \"\"\"Run it.\"\"\"
    go()
",
        );
        let [decl] = model.decls.as_slice() else {
            panic!("one decl: {:?}", model.decls);
        };
        assert_eq!(rows(&decl.doc), vec![vec![3]]);
        assert_eq!(rows(&decl.body), vec![vec![2, 4]]);
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
        assert_eq!(class.name_rows, vec![1, 2]);
        assert_eq!(class.head, vec![1, 2]);
        assert_eq!(rows(&class.doc), vec![vec![3]]);
        assert_eq!(
            rows(&class.body),
            vec![
                vec![5, 6],
                vec![7],
                vec![10, 11],
                vec![15],
                vec![18],
                vec![20],
                vec![21]
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
                (vec![10, 11], vec![10, 11], vec![vec![12]], vec![vec![13]],),
                (vec![15], vec![15], vec![], vec![vec![16]],),
                (vec![18], vec![18], vec![], vec![]),
            ]
        );
        assert!(class.members.iter().all(|member| member.members.is_empty()));
    }

    #[test]
    fn code_python_nested_class_flattens_into_outer_roster() {
        let model = extract_source(
            "spec.py",
            "\
class Basic:
    INDEX = 60

    class Qos(Method):
        \"\"\"Set prefetch.\"\"\"
        NAME = \"Basic.Qos\"

        def encode(self) -> list[bytes]:
            pieces = []
            return pieces

        class Inner:
            def run(self): ...
",
        );
        let [class] = model.decls.as_slice() else {
            panic!("one decl: {:?}", model.decls);
        };
        assert_eq!(
            rows(&class.body),
            vec![
                vec![2],
                vec![4],
                vec![5],
                vec![6],
                vec![8],
                vec![12],
                vec![13]
            ]
        );
        let members: Vec<_> = class
            .members
            .iter()
            .map(|member| (member.name_rows.clone(), rows(&member.body)))
            .collect();
        assert_eq!(
            members,
            vec![(vec![8], vec![vec![9], vec![10]]), (vec![13], vec![])]
        );
    }

    #[test]
    fn code_python_roster_lists_decorators_and_signature_closing_row() {
        let model = extract_source(
            "props.py",
            "\
class Params:
    @property
    def port(self) -> int: ...
    @port.setter
    def port(self, value: int) -> None: ...
    @retry(
        times=3,
    )
    def connect(
        self, host: str, timeout: float = 10.0,
    ) -> \"Connection\": ...
class Wide(
    Base,
):
    pass
",
        );
        let summary: Vec<_> = model
            .decls
            .iter()
            .map(|decl| {
                let members: Vec<_> = decl
                    .members
                    .iter()
                    .map(|member| member.name_rows.clone())
                    .collect();
                (decl.name_rows.clone(), members, rows(&decl.body))
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (
                    vec![1],
                    vec![vec![2, 3], vec![4, 5], vec![9, 11]],
                    vec![vec![2, 3], vec![4, 5], vec![9, 11]]
                ),
                (vec![12, 14], vec![], vec![vec![15]]),
            ]
        );
    }

    #[test]
    fn code_python_type_stub_parses_like_a_module() {
        assert!(
            Language::from_path(Path::new("stub.pyi"))
                .is_some_and(|language| language.extensions == LANGUAGE.extensions)
        );
        let model = extract_source(
            "pkg/stub.pyi",
            "\
class Params:
    @property
    def port(self) -> int: ...
    @port.setter
    def port(self, value: int) -> None: ...
",
        );
        let [class] = model.decls.as_slice() else {
            panic!("one decl: {:?}", model.decls);
        };
        let members: Vec<_> = class
            .members
            .iter()
            .map(|member| member.name_rows.clone())
            .collect();
        assert_eq!(members, vec![vec![2, 3], vec![4, 5]]);
    }

    #[test]
    fn code_python_definitions_under_if_and_try_carry_their_condition() {
        let model = extract_source(
            "pkg/locks.py",
            "\
__all__ = [\"lock\", \"unlock\"]
def _fd(f):
    return f
if os.name == \"nt\":
    import msvcrt
    def lock(f, flags):
        return msvcrt.lock(f)
    def unlock(f): ...
else:
    try:
        import fcntl
    except ImportError:
        def lock(f, flags): ...
    else:
        def lock(f, flags):
            return fcntl.flock(f)
",
        );
        let summary: Vec<_> = model
            .decls
            .iter()
            .map(|decl| (decl.name_rows.clone(), decl.head.clone()))
            .collect();
        assert_eq!(
            summary,
            vec![
                (vec![2], vec![2]),
                (vec![4, 6], vec![4, 6]),
                (vec![8], vec![8]),
                (vec![9, 12, 13], vec![9, 12, 13]),
                (vec![14, 15], vec![14, 15]),
            ]
        );
    }

    #[test]
    fn code_python_overload_stubs_before_their_implementation_are_hidden() {
        let source = "\
@overload
def filter(x: int) -> list[int]: ...
@typing.overload
def filter(x: str) -> list[str]: ...
def filter(x):
    return [x]
class Wikicode:
    @overload
    def ifilter(self, x: int) -> int: ...
    def ifilter(self, x):
        return x
";
        let model = extract_source("wikicode.py", source);
        let summary: Vec<_> = model
            .decls
            .iter()
            .map(|decl| {
                let members: Vec<_> = decl
                    .members
                    .iter()
                    .map(|member| member.name_rows.clone())
                    .collect();
                (decl.name_rows.clone(), members)
            })
            .collect();
        assert_eq!(summary, vec![(vec![5], vec![]), (vec![7], vec![vec![10]])]);
        assert_eq!(rows(&model.decls[1].body), vec![vec![10]]);
        let stub = extract_source(
            "wikicode.pyi",
            &source[..source.find("def filter(x):").unwrap()],
        );
        let stub_names: Vec<_> = stub
            .decls
            .iter()
            .map(|decl| decl.name_rows.clone())
            .collect();
        assert_eq!(stub_names, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn code_python_comment_above_undocumented_definition_is_its_doc() {
        let model = extract_source(
            "tz.py",
            "\
# Template tags for time zones.

# Convert a datetime to the current time zone.
@register.filter
def localtime(value):
    return value

class Node:
    tag = None
    # Render the node.
    def render(self):
        return ''

    # Shadowed by the docstring.
    def close(self):
        \"\"\"Close it.\"\"\"
",
        );
        let docs: Vec<_> = model.decls.iter().map(|decl| rows(&decl.doc)).collect();
        assert_eq!(docs, vec![vec![vec![3]], vec![]]);
        let member_docs: Vec<_> = model.decls[1]
            .members
            .iter()
            .map(|member| rows(&member.doc))
            .collect();
        assert_eq!(member_docs, vec![vec![vec![10]], vec![vec![16]]]);
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
    fn code_python_file_weight_favors_modules_a_package_init_imports_from() {
        let dir = tempfile::tempdir().unwrap();
        let write = |relative: &str, content: &str| {
            let path = dir.path().join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        };
        write(
            "pkg/__init__.py",
            "from .core import Engine\nfrom .engine.runner import run\n",
        );
        write(
            "pkg/engine/__init__.py",
            "from pkg.engine.plan import Plan\n",
        );
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let weight = |relative: &str| file_weight(&dir.path().join(relative), &ctx);
        assert_eq!(weight("pkg/core.py"), PUBLIC_API_MODULE_WEIGHT);
        assert_eq!(weight("pkg/engine/runner.py"), PUBLIC_API_MODULE_WEIGHT);
        assert_eq!(weight("pkg/engine/plan.py"), PUBLIC_API_MODULE_WEIGHT);
        assert_eq!(weight("pkg/helpers.py"), 1.0);
        assert_eq!(weight("pkg/engine/helpers.py"), 1.0);
        assert_eq!(weight("script.py"), 1.0);
    }

    #[test]
    fn code_python_entrypoints_are_dunder_named_modules() {
        let ctx = WalkCtx::new(PathBuf::from("/repo"));
        let entrypoint = |path: &str| is_entrypoint(Path::new(path), &ctx);
        assert!(entrypoint("/repo/pkg/__init__.py"));
        assert!(entrypoint("/repo/pkg/__init__.pyi"));
        assert!(entrypoint("/repo/pkg/__main__.py"));
        assert!(entrypoint("/repo/pkg/__version__.py"));
        assert!(!entrypoint("/repo/pkg/main.py"));
        assert!(!entrypoint("/repo/pkg/_private.py"));
    }
}
