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
//!   first decorator. Its name rows are the `def` / `class` row and,
//!   when the signature spans rows, the row that closes it (`) -> T:`),
//!   so a roster never lists an unclosed `def f(`; a member's also
//!   include its one-row decorators (`@property` and `@overload` say
//!   what a method is). The file's roster leaves decorators to the
//!   declaration's head: at two rows per entry, a long module's roster
//!   ran out before the classes at its end.
//! - **Doc**: the docstring opening a `def` / `class` body, or else the
//!   `#` comments directly above the definition.
//! - **Module doc**: a dunder-named module's (`__init__.py`,
//!   `__main__.py`, `__version__.py`) module docstring and
//!   dunder assignments other than `__all__`. Other modules' docstrings and
//!   leading `#` comments (shebangs, license headers) are in no part, and
//!   their dunder assignments are ordinary constants.
//! - **Re-exports**: `__all__`, and in `__init__.py` every top-level
//!   `from … import …` of the package's own modules (a relative import, or
//!   one whose module path starts at a directory above the file), unless
//!   it imports only private names. Other imports from the
//!   standard library and third-party packages are in no part. An
//!   `__init__.py`'s `__all__` is left out when those imports already
//!   re-export every name it lists, so the roster names each once.

use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{Language, SourceFile, file_name, file_stem};
use crate::fs_util::lists_file;
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
    let is_entry = file_stem(&file.path).is_some_and(is_dunder);
    let root = file.tree.root_node();
    let mut first_statement = true;
    let mut reexported_names: HashSet<&str> = HashSet::new();
    let mut all_statements = Vec::new();
    for node in root.named_children(&mut root.walk()) {
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
            "function_definition"
            | "class_definition"
            | "decorated_definition"
            | "if_statement"
            | "try_statement" => model
                .decls
                .extend(definitions(file, node, 0).0.into_iter().map(|mut decl| {
                    decl.name_rows
                        .retain(|&row| !file.line(row).trim_start().starts_with('@'));
                    decl
                })),
            "import_from_statement" if is_package_init && imports_own_module(file, node, ctx) => {
                let names = imported_names(file, node);
                let public = names.is_empty() || names.iter().any(|name| !is_private(name));
                if public {
                    reexported_names.extend(names);
                    model.reexports.push(rows())
                }
            }
            "type_alias_statement" => {
                if node.child_by_field_name("left").is_some() {
                    model.decls.push(constant_or_alias(file, node));
                }
            }
            "expression_statement" => match assignment_target(file, node) {
                Some(("__all__", _)) => all_statements.push(rows()),
                Some((name, false)) if is_entry && is_dunder(name) => model.module_doc.push(rows()),
                Some((_, false)) => model.decls.push(constant_or_alias(file, node)),
                Some((_, true)) | None => {}
            },
            _ => {}
        }
    }
    let all_repeats_imports = !exported_names.is_empty()
        && exported_names
            .iter()
            .all(|name| reexported_names.contains(name));
    if !all_repeats_imports {
        model.reexports.extend(all_statements);
    }
    model
}

/// A dunder module (`__init__.py`, `__main__.py`, `__version__.py`) of a
/// top-level package: a package directory whose parent is no package
/// or is the walk root (a root `__init__.py` makes a checkout importable
/// without turning its packages into subpackages). A subpackage's
/// `__init__.py` and a `__main__.py` outside any package are ordinary
/// modules at their own depth.
fn is_entrypoint(path: &Path, ctx: &WalkCtx) -> bool {
    let is_package = |dir: &Path| {
        ["__init__.py", "__init__.pyi"]
            .iter()
            .any(|init| lists_file(&dir.join(init), ctx.dir_filter()))
    };
    let is_parent_package = |dir: &Path| dir != ctx.root() && is_package(dir);
    file_stem(path).is_some_and(is_dunder)
        && path
            .parent()
            .is_some_and(|dir| is_package(dir) && !dir.parent().is_some_and(is_parent_package))
}

/// A module a package `__init__.py` above it imports names from
/// (`from .core import Engine`, `from .engine.core import Engine`,
/// `from pkg.engine.core import Engine`) implements a package's public API,
/// so it outranks its sibling helper modules. The `from . import core`
/// form doesn't count: adding it measured -0.0001 avg7 (2026-09-25).
fn file_weight(path: &Path, ctx: &WalkCtx) -> f64 {
    let Some(stem) = file_stem(path) else {
        return 1.0;
    };
    let mut module_path = vec![stem];
    let mut inits = Vec::new();
    let mut dir = path.parent();
    while let Some(package_dir) = dir.filter(|dir| dir.starts_with(ctx.root()))
        && let Some(init) = ctx.read_source(&package_dir.join("__init__.py"))
    {
        inits.push((module_path.len(), init));
        let Some(package) = file_name(package_dir) else {
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
        .any(|dir| file_name(dir) == first_component)
}

/// The names a `from … import …` binds: each alias, else the imported
/// name. Empty for `from x import *`.
fn imported_names<'a>(file: &'a SourceFile, statement: Node) -> Vec<&'a str> {
    statement
        .children_by_field_name("name", &mut statement.walk())
        .filter_map(|name| match name.kind() {
            "aliased_import" => name.child_by_field_name("alias"),
            _ => Some(name),
        })
        .map(|name| file.text(name))
        .collect()
}

fn is_private(name: &str) -> bool {
    name.starts_with('_') && !is_dunder(name)
}

/// The string contents of the module's top-level `__all__` assignments.
fn all_names(file: &SourceFile) -> Vec<&str> {
    let root = file.tree.root_node();
    let mut names = Vec::new();
    for statement in root.named_children(&mut root.walk()) {
        if statement.kind() == "expression_statement"
            && matches!(assignment_target(file, statement), Some(("__all__", _)))
        {
            collect_string_contents(file, statement, &mut names);
        }
    }
    names
}

fn collect_string_contents<'a>(file: &'a SourceFile, node: Node, names: &mut Vec<&'a str>) {
    let mut pending = vec![node];
    while let Some(node) = pending.pop() {
        if node.kind() == "string_content" {
            names.push(file.text(node));
            continue;
        }
        let first = pending.len();
        pending.extend(node.named_children(&mut node.walk()));
        pending[first..].reverse();
    }
}

fn is_dunder(name: &str) -> bool {
    name.len() >= 4 && name.starts_with("__") && name.ends_with("__")
}

/// The identifier an `expression_statement` assigns (`NAME = …`,
/// `NAME: T = …`, `NAME += …`, the first target of `NAME = OTHER = …`)
/// and whether the assignment is augmented; `None` for tuple, attribute
/// or subscript targets and for non-assignments.
fn assignment_target<'a>(file: &'a SourceFile, statement: Node) -> Option<(&'a str, bool)> {
    let assignment = statement.named_children(&mut statement.walk()).next()?;
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
    node.named_children(&mut node.walk())
        .next()
        .is_some_and(|first| matches!(first.kind(), "string" | "concatenated_string"))
}

/// A constant or alias: all head, no body.
fn constant_or_alias(file: &SourceFile, node: Node) -> DeclInfo {
    let rows = file.node_rows(node);
    DeclInfo::new(vec![*rows.start()], rows.collect(), Shape::Whole)
}

/// The `def`s and `class`es a statement defines: itself, or those in
/// the blocks of an `if` / `try` statement, nested ones included. The row
/// opening each block (`if …:`, `else:`, `except …:`) joins the head and
/// name rows of the block's first definition, so the roster says under
/// which condition it exists, and the statement's own `if …:` / `try:`
/// row joins its first definition, so an `else:` never lists alone.
/// Also the statements of those blocks that define nothing (fields,
/// assignments), as [`Item`]s holding rows past `after_row`; a block
/// defining nothing lists its opening row with its first statement.
fn definitions(file: &SourceFile, statement: Node, after_row: usize) -> (Vec<DeclInfo>, Vec<Item>) {
    if !matches!(statement.kind(), "if_statement" | "try_statement") {
        return (
            definition(file, statement).into_iter().collect(),
            Vec::new(),
        );
    }
    let mut decls = Vec::new();
    let mut items = Vec::new();
    for block in clause_blocks(statement) {
        let first = decls.len();
        let mut fields: Vec<Node> = Vec::new();
        for node in block.named_children(&mut block.walk()) {
            let (defined, nested_items) = definitions(file, node, after_row);
            if defined.is_empty() {
                if !is_definition_kind(node.kind()) {
                    fields.push(node);
                }
                continue;
            }
            while fields.last().is_some_and(|field| field.kind() == "comment") {
                fields.pop();
            }
            decls.extend(defined);
            items.extend(nested_items);
        }
        let mut block_items = file.node_items(fields, after_row);
        if let Some(clause) = block.parent() {
            let opening_row = clause.start_position().row + 1;
            if let Some(decl) = decls.get_mut(first) {
                decl.head.insert(0, opening_row);
                decl.name_rows.insert(0, opening_row);
            } else if let Some(item) = block_items.first_mut() {
                item.rows.insert(0, opening_row);
            }
        }
        items.extend(block_items);
    }
    let statement_row = statement.start_position().row + 1;
    if let Some(decl) = decls.first_mut()
        && !decl.head.contains(&statement_row)
    {
        decl.head.insert(0, statement_row);
        decl.name_rows.insert(0, statement_row);
    }
    (decls, items)
}

/// An `if` / `try` statement's blocks: its own (`if` consequence, `try`
/// body) and those of its `elif` / `else` / `except` / `finally` clauses.
fn clause_blocks(statement: Node) -> Vec<Node> {
    let mut blocks = Vec::new();
    for child in statement.named_children(&mut statement.walk()) {
        if child.kind() == "block" {
            blocks.push(child);
            continue;
        }
        blocks.extend(
            child
                .named_children(&mut child.walk())
                .filter(|grandchild| grandchild.kind() == "block"),
        );
    }
    blocks
}

/// A `def` or `class`, possibly wrapped in `decorated_definition`;
/// `None` for an `@overload` stub whose implementation follows it.
fn definition(file: &SourceFile, unit: Node) -> Option<DeclInfo> {
    let inner = defined(unit)?;
    let shape = match inner.kind() {
        "function_definition" => Shape::Callable,
        "class_definition" => Shape::Whole,
        _ => return None,
    };
    if is_overload(file, unit) && has_implementation_after(file, unit) {
        return None;
    }
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
        doc,
        body,
        members,
        ..DeclInfo::new(name_rows, head, shape)
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
    unit.named_children(&mut unit.walk())
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
    unit.named_children(&mut unit.walk())
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
    inner
        .children(&mut inner.walk())
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
    let mut statements: Vec<Node> = inner
        .named_children(&mut inner.walk())
        .filter(|child| child.kind() == "comment" && child.start_byte() >= header_end)
        .collect();
    statements.extend(body.named_children(&mut body.walk()));
    statements.sort_by_key(|node| node.start_byte());
    statements
}

/// A class suite (after its docstring): methods, including those in an
/// `if` / `try` block (see [`definitions`]), become members, listed in
/// the body by their name rows, and the block's other statements are
/// body items; every other statement (fields, blocks defining nothing)
/// is a body [`Item`] with the comments directly above it. A nested class is flattened into the suite: its head is one
/// item, its doc, fields and member name rows follow, and its methods
/// become members, so it lists as a roster rather than as its whole
/// source.
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
        let (defined, clause_items) = definitions(file, *node, after_row);
        if defined.is_empty() {
            if !is_definition_kind(node.kind()) {
                run.push(*node);
            }
            continue;
        }
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
        let first_new = body.len();
        for member in defined {
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
        if matches!(node.kind(), "if_statement" | "try_statement") {
            body.extend(clause_items);
            body[first_new..].sort_by_key(|item| item.rows.first().copied());
        }
    }
    body.extend(file.node_items(run, after_row));
    (body, members)
}

fn is_definition_kind(kind: &str) -> bool {
    matches!(
        kind,
        "function_definition" | "class_definition" | "decorated_definition"
    )
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{describe, rows};
    use super::*;

    fn extract_source(relative_path: &str, source: &str) -> FileModel {
        super::super::test_support::extract_source(&LANGUAGE, relative_path, source)
    }

    #[test]
    fn python_module_doc_skips_license_header_and_keeps_dunders() {
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
    fn python_package_init_reexports_from_imports_only() {
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
    fn python_package_init_reexports_only_its_own_modules() {
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

    /// A name is listed once: `__all__` goes when the imports above it
    /// already re-export every name it lists, and an import of private
    /// names only is not a re-export.
    #[test]
    fn python_package_init_lists_each_public_name_once() {
        let imports = "\
from .core import Engine, run
from .html import _SCAN_BUDGET as _SCAN_BUDGET
from .__about__ import __version__
";
        let model = extract_source(
            "pkg/__init__.py",
            &format!("{imports}__all__ = [\"Engine\", \"run\", \"__version__\"]\n"),
        );
        assert_eq!(rows(&model.reexports), vec![vec![1], vec![3]]);
        let model = extract_source(
            "pkg/__init__.py",
            &format!("{imports}__all__ = [\"Engine\", \"run\", \"helper\"]\n"),
        );
        assert_eq!(rows(&model.reexports), vec![vec![1], vec![3], vec![4]]);
    }

    #[test]
    fn python_methods_under_class_level_if_are_members() {
        let model = extract_source(
            "thing.py",
            "\
class Thing:
    if TYPE_CHECKING:
        def typed_only(self): ...
    if sys.version_info >= (3, 11):
        def modern(self):
            return 1
    else:
        def modern(self):
            return 0
    if DEBUG:
        level = 1
    def public(self): ...
    if sys.platform == \"win32\":
        supports_fork = False
        # Spawn a worker.
        def start(self): ...
    else:
        supports_fork = True
",
        );
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body [[2, 3], [4, 5], [7, 8], [10, 11], [12], [13, 16], [14], [17, 18]]",
                "  Callable name [2, 3] head [2, 3] doc [] body []",
                "  Callable name [4, 5] head [4, 5] doc [] body [[6]]",
                "  Callable name [7, 8] head [7, 8] doc [] body [[9]]",
                "  Callable name [12] head [12] doc [] body []",
                "  Callable name [13, 16] head [13, 16] doc [[15]] body []",
            ]
        );
    }

    #[test]
    fn python_decorated_function_splits_head_doc_and_body() {
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
        assert_eq!(
            describe(&model),
            [
                "Callable name [3, 5] head [1, 2, 3, 4, 5] doc [[6], [8, 9]] body [[10, 11], [12], [13]]"
            ]
        );
    }

    #[test]
    fn python_comment_above_docstring_joins_first_statement() {
        let model = extract_source(
            "m.py",
            "\
def run():
    # pylint: disable=broad-except
    \"\"\"Run it.\"\"\"
    go()
",
        );
        assert_eq!(
            describe(&model),
            ["Callable name [1] head [1] doc [[3]] body [[2, 4]]"]
        );
    }

    #[test]
    fn python_one_line_def_is_all_head() {
        let model = extract_source(
            "m.py",
            "\
def one(): return 1
def documented(): \"\"\"Doc on the def row.\"\"\"
def _helper(x):  # noqa
    return x
",
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [1] head [1] doc [] body []",
                "Callable name [2] head [2] doc [] body []",
                "Callable name [3] head [3] doc [] body [[4]]",
            ]
        );
    }

    #[test]
    fn python_class_is_container_with_methods_as_members() {
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
        assert_eq!(
            describe(&model),
            [
                "Whole name [2] head [1, 2] doc [[3]] body [[5, 6], [7], [10, 11], [15], [18], [20], [21]]",
                "  Callable name [10, 11] head [10, 11] doc [[12]] body [[13]]",
                "  Callable name [15] head [15] doc [] body [[16]]",
                "  Callable name [18] head [18] doc [] body []",
            ]
        );
    }

    #[test]
    fn python_nested_class_flattens_into_outer_roster() {
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
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body [[2], [4], [5], [6], [8], [12], [13]]",
                "  Callable name [8] head [8] doc [] body [[9], [10]]",
                "  Callable name [13] head [13] doc [] body []",
            ]
        );
    }

    #[test]
    fn python_roster_lists_decorators_and_signature_closing_row() {
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
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body [[2, 3], [4, 5], [9, 11]]",
                "  Callable name [2, 3] head [2, 3] doc [] body []",
                "  Callable name [4, 5] head [4, 5] doc [] body []",
                "  Callable name [9, 11] head [6, 7, 8, 9, 10, 11] doc [] body []",
                "Whole name [12, 14] head [12, 13, 14] doc [] body [[15]]",
            ]
        );
    }

    #[test]
    fn python_definitions_under_if_and_try_carry_their_condition() {
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
        assert_eq!(
            describe(&model),
            [
                "Callable name [2] head [2] doc [] body [[3]]",
                "Callable name [4, 6] head [4, 6] doc [] body [[7]]",
                "Callable name [8] head [8] doc [] body []",
                "Callable name [9, 10, 12, 13] head [9, 10, 12, 13] doc [] body []",
                "Callable name [14, 15] head [14, 15] doc [] body [[16]]",
            ]
        );
    }

    #[test]
    fn python_overload_stubs_before_their_implementation_are_hidden() {
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
        assert_eq!(
            describe(&model),
            [
                "Callable name [5] head [5] doc [] body [[6]]",
                "Whole name [7] head [7] doc [] body [[10]]",
                "  Callable name [10] head [10] doc [] body [[11]]",
            ]
        );
        assert!(
            Language::from_path(Path::new("stub.pyi"))
                .is_some_and(|language| language.extensions == LANGUAGE.extensions)
        );
        let model = extract_source(
            "wikicode.pyi",
            &source[..source.find("def filter(x):").unwrap()],
        );
        assert_eq!(
            describe(&model),
            [
                "Callable name [2] head [1, 2] doc [] body []",
                "Callable name [4] head [3, 4] doc [] body []",
            ]
        );
    }

    #[test]
    fn python_comment_above_undocumented_definition_is_its_doc() {
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
        assert_eq!(
            describe(&model),
            [
                "Callable name [5] head [4, 5] doc [[3]] body [[6]]",
                "Whole name [8] head [8] doc [] body [[9], [11], [15]]",
                "  Callable name [11] head [11] doc [[10]] body [[12]]",
                "  Callable name [15] head [15] doc [[16]] body []",
            ]
        );
    }

    #[test]
    fn python_constants_and_aliases_are_whole_head_only() {
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
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 2, 3] doc [] body []",
                "Whole name [4] head [4] doc [] body []",
                "Whole name [8] head [8] doc [] body []",
            ]
        );
    }

    #[test]
    fn python_chained_assignment_is_one_statement() {
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
    fn python_file_weight_favors_modules_a_package_init_imports_from() {
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
    fn python_entrypoints_are_dunder_modules_of_top_level_packages() {
        let dir = tempfile::tempdir().unwrap();
        for file in [
            "src/pkg/__init__.py",
            "src/pkg/sub/__init__.py",
            "stubs/__init__.pyi",
            "tools/android/__main__.py",
            "__init__.py",
            "lib/__init__.py",
        ] {
            let path = dir.path().join(file);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, "").unwrap();
        }
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let entrypoint = |path: &str| is_entrypoint(&dir.path().join(path), &ctx);
        assert!(entrypoint("src/pkg/__init__.py"));
        assert!(entrypoint("src/pkg/__main__.py"));
        assert!(entrypoint("src/pkg/__version__.py"));
        assert!(entrypoint("stubs/__init__.pyi"));
        assert!(entrypoint("lib/__init__.py"));
        assert!(entrypoint("__init__.py"));
        assert!(!entrypoint("src/pkg/main.py"));
        assert!(!entrypoint("src/pkg/sub/__init__.py"));
        assert!(!entrypoint("tools/android/__main__.py"));
    }
}
