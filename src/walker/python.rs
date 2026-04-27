//! Python walker. Per-file orientation (imports + module docstring +
//! `__all__` + dunder assignments) plus per-decl batches for top-level
//! classes, defs (sync / async), and module-level non-dunder
//! assignments. Per-method batches inside top-level classes mirror
//! Rust's per-item granularity so NS targets like
//! `pluggy._hooks.HookCaller._add_hookimpl` body land naturally.
//!
//! Per-file keys:
//! - [`PythonKey::Imports`]: `import` / `from … import …` directives
//!   plus the module docstring and module-level dunder assignments
//!   (`__all__`, `__version__`, `__author__`).
//! - [`PythonKey::DeclNames`]: surface listing of every top-level
//!   class, def, and module-level non-dunder simple-assignment first
//!   line — catastrophic-omission hedge.
//! - [`PythonKey::MethodSigs`]: surface listing of every method's
//!   first line across every top-level class. Decorator-aware.
//! - [`PythonKey::TestNames`]: in `test_*.py` / `*_test.py` only,
//!   surface listing of `def test_*` first lines.
//!
//! Per-decl keys (keyed by start line):
//! - [`PythonKey::Decl`]: one top-level class / def / non-dunder
//!   constant. For decorated forms, the span starts at the
//!   `@decorator` row.
//! - [`PythonKey::DeclDoc`]: top-level def / class docstring.
//! - [`PythonKey::DeclBody`]: top-level def body interior, sans
//!   leading docstring.
//! - [`PythonKey::ClassBody`]: top-level class body excluding methods
//!   and the leading docstring — TypedDict / dataclass / Pydantic
//!   fields, `__slots__`, class constants.
//!
//! Per-method keys (keyed by the method's start line):
//! - [`PythonKey::Method`] / [`PythonKey::MethodDoc`] /
//!   [`PythonKey::MethodBody`]: same shape as the per-decl trio.
//!
//! Visibility: emits everything. A `visibility_factor` discount
//! (1.0 unprefixed, 0.6 leading-`_`, 1.0 dunder) ranks public-by-PEP-8
//! names above leading-`_` "internal" ones rather than hard-filtering
//! — NSes anchor on intentionally-private names
//! (`pluggy._callers._multicall`, `pluggy._hooks.HookCaller._add_hookimpl`).
//!
//! Parse trees are cached in [`WalkCtx`].

use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, PythonKey};
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, dedup_sorted, extend_span, file_depth_factor, fs::files_with_extension,
    push_rows, signature_end_row, single_file_lines_content,
};

const VISIBILITY_PUBLIC: f64 = 1.0;
const VISIBILITY_UNDERSCORE: f64 = 0.6;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let py_files = files_with_extension(dir, "py");
    if py_files.is_empty() {
        return Vec::new();
    }
    let (test_files, source_files): (Vec<_>, Vec<_>) =
        py_files.into_iter().partition(|p| is_test_file(p));
    let mut out = Vec::new();
    out.extend(expand_test_files(&test_files, ctx));
    out.extend(expand_source_files(&source_files, ctx));
    out
}

fn expand_test_files(test_files: &[PathBuf], ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in test_files {
        let Some((source, tree)) = parse_python(ctx, file) else {
            continue;
        };
        let starts = collect_test_function_starts(&tree, &source);
        if starts.is_empty() {
            continue;
        }
        let ellipses = starts.iter().map(|&l| l + 1).collect();
        let lines = FileLines::new(starts).with_ellipses(ellipses);
        let Some(content) = single_file_lines_content(file, &source, lines) else {
            continue;
        };
        out.push(Batch {
            key: PythonKey::TestNames { file: file.clone() }.into(),
            predecessor: None,
            content,
            value: test_names_value(file, ctx),
        });
    }
    out
}

fn expand_source_files(source_files: &[PathBuf], ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in source_files {
        let Some((source, tree)) = parse_python(ctx, file) else {
            continue;
        };
        let src_lines: Vec<&str> = source.lines().collect();
        let decls = find_top_level_decls(&tree, &source);

        if let Some(content) =
            single_file_lines_content(file, &source, collect_imports(&tree, &source))
        {
            out.push(Batch {
                key: PythonKey::Imports { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: imports_value(file, ctx),
            });
        }

        if decls.is_empty() {
            continue;
        }

        let names_key = PythonKey::DeclNames { file: file.clone() };
        if let Some(content) =
            single_file_lines_content(file, &source, collect_decl_names_from(&decls))
        {
            out.push(Batch {
                key: names_key.clone().into(),
                predecessor: None,
                content,
                value: decl_names_value(file, ctx),
            });
        }
        let names_predecessor = BatchKey::Python(names_key);

        let methods_by_class = collect_methods_by_class(&decls, &source);
        let mut method_sigs_predecessor = None;
        if !methods_by_class.is_empty()
            && let Some(content) = single_file_lines_content(
                file,
                &source,
                collect_method_sigs_from(&methods_by_class),
            )
        {
            let key = PythonKey::MethodSigs { file: file.clone() };
            out.push(Batch {
                key: key.clone().into(),
                // Predecessor: DeclNames. The MethodSigs `Full+Ellipsis`
                // pair shares lines with DeclNames' ellipsis row when a
                // class header at line N is followed immediately by a
                // method/decorator at line N+1. Chaining through DeclNames
                // makes the line-N+1 overlap an ancestor overlap.
                predecessor: Some(names_predecessor.clone()),
                content,
                value: method_sigs_value(file, ctx),
            });
            method_sigs_predecessor = Some(BatchKey::Python(key));
        }

        for decl in &decls {
            let decl_key = PythonKey::Decl {
                file: file.clone(),
                start_line: decl.start_line,
            };
            if let Some(content) = single_file_lines_content(file, &source, collect_decl(decl)) {
                out.push(Batch {
                    key: decl_key.clone().into(),
                    predecessor: Some(names_predecessor.clone()),
                    content,
                    value: decl_value(file, decl, ctx),
                });
            }
            let decl_predecessor = BatchKey::Python(decl_key);

            if let Some(content) =
                single_file_lines_content(file, &source, collect_doc_for(decl.inner_node, &source))
            {
                out.push(Batch {
                    key: PythonKey::DeclDoc {
                        file: file.clone(),
                        start_line: decl.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: decl_doc_value(file, decl, ctx),
                });
            }

            match decl.kind {
                DeclKind::Function => {
                    if let Some(content) = single_file_lines_content(
                        file,
                        &source,
                        collect_def_body(decl.inner_node, &src_lines),
                    ) {
                        out.push(Batch {
                            key: PythonKey::DeclBody {
                                file: file.clone(),
                                start_line: decl.start_line,
                            }
                            .into(),
                            predecessor: Some(decl_predecessor.clone()),
                            content,
                            value: decl_body_value(file, decl, ctx),
                        });
                    }
                }
                DeclKind::Class => {
                    if let Some(content) = single_file_lines_content(
                        file,
                        &source,
                        collect_class_body(decl.inner_node, &src_lines),
                    ) {
                        out.push(Batch {
                            key: PythonKey::ClassBody {
                                file: file.clone(),
                                start_line: decl.start_line,
                            }
                            .into(),
                            predecessor: Some(decl_predecessor.clone()),
                            content,
                            value: class_body_value(file, decl, ctx),
                        });
                    }
                    out.extend(emit_methods(
                        file,
                        ctx,
                        &source,
                        &src_lines,
                        decl,
                        method_sigs_predecessor
                            .as_ref()
                            .unwrap_or(&decl_predecessor),
                    ));
                }
                DeclKind::Const => {}
            }
        }
    }
    out
}

fn emit_methods(
    file: &Path,
    ctx: &WalkCtx,
    source: &str,
    src_lines: &[&str],
    class_decl: &DeclInfo,
    class_predecessor: &BatchKey,
) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for method in collect_methods_in_class(class_decl, source) {
        let method_key = PythonKey::Method {
            file: file.to_path_buf(),
            start_line: method.start_line,
        };
        if let Some(content) = single_file_lines_content(file, source, collect_decl(&method)) {
            out.push(Batch {
                key: method_key.clone().into(),
                predecessor: Some(class_predecessor.clone()),
                content,
                value: method_value(file, &method, ctx),
            });
        }
        let method_predecessor = BatchKey::Python(method_key);

        if let Some(content) =
            single_file_lines_content(file, source, collect_doc_for(method.inner_node, source))
        {
            out.push(Batch {
                key: PythonKey::MethodDoc {
                    file: file.to_path_buf(),
                    start_line: method.start_line,
                }
                .into(),
                predecessor: Some(method_predecessor.clone()),
                content,
                value: method_doc_value(file, &method, ctx),
            });
        }

        if let Some(content) =
            single_file_lines_content(file, source, collect_def_body(method.inner_node, src_lines))
        {
            out.push(Batch {
                key: PythonKey::MethodBody {
                    file: file.to_path_buf(),
                    start_line: method.start_line,
                }
                .into(),
                predecessor: Some(method_predecessor),
                content,
                value: method_body_value(file, &method, ctx),
            });
        }
    }
    out
}

// --- decl model ---------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclKind {
    Class,
    Function,
    Const,
}

impl DeclKind {
    fn kind_weight(self) -> f64 {
        match self {
            DeclKind::Class => 1.10,
            DeclKind::Function => 1.00,
            DeclKind::Const => 0.85,
        }
    }
}

/// One top-level item or method. `unit_node` is the outermost node
/// (decorated_definition wrapper if decorated); `inner_node` is the
/// underlying `function_definition` / `class_definition` / for a const,
/// the `expression_statement` containing the assignment. `start_line`
/// is the unit_node's first row + 1 — so decorator rows are part of
/// the span.
#[derive(Debug, Clone, Copy)]
struct DeclInfo<'a> {
    kind: DeclKind,
    unit_node: Node<'a>,
    inner_node: Node<'a>,
    start_line: usize,
    /// Underscore-prefixed (excluding pure-dunder names) → 0.6 visibility.
    /// Dunders like `__init__` keep 1.0.
    underscore_private: bool,
}

impl<'a> DeclInfo<'a> {
    fn visibility_factor(&self) -> f64 {
        if self.underscore_private {
            VISIBILITY_UNDERSCORE
        } else {
            VISIBILITY_PUBLIC
        }
    }
}

/// Returns `(unit_node, inner_node)` for a `function_definition` /
/// `class_definition`, unwrapping a `decorated_definition` wrapper if
/// present. Returns `None` for any other node kind. The unit_node is
/// what the walker treats as one decl span (so decorator lines are
/// part of `start_line .. end_line`); the inner_node carries the
/// `name`, `body`, and parameters.
fn function_or_decorated<'a>(node: Node<'a>) -> Option<(Node<'a>, Node<'a>)> {
    match node.kind() {
        "function_definition" | "class_definition" => Some((node, node)),
        "decorated_definition" => {
            // The wrapped def/class is the "definition" field.
            let inner = node.child_by_field_name("definition")?;
            if matches!(inner.kind(), "function_definition" | "class_definition") {
                Some((node, inner))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn find_top_level_decls<'a>(tree: &'a Tree, source: &str) -> Vec<DeclInfo<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if let Some((unit, inner)) = function_or_decorated(child) {
            let kind = match inner.kind() {
                "class_definition" => DeclKind::Class,
                _ => DeclKind::Function,
            };
            let name = name_of(inner, source).unwrap_or("");
            out.push(DeclInfo {
                kind,
                unit_node: unit,
                inner_node: inner,
                start_line: unit.start_position().row + 1,
                underscore_private: is_underscore_private(name),
            });
        } else if child.kind() == "expression_statement"
            && let Some(target) = const_assignment_target(child, source)
            && !is_dunder(target)
        {
            out.push(DeclInfo {
                kind: DeclKind::Const,
                unit_node: child,
                inner_node: child,
                start_line: child.start_position().row + 1,
                underscore_private: is_underscore_private(target),
            });
        }
    }
    out
}

/// Methods of one class — direct children of the class body, with
/// `decorated_definition` unwrapped.
fn collect_methods_in_class<'a>(class_decl: &DeclInfo<'a>, source: &str) -> Vec<DeclInfo<'a>> {
    let Some(body) = class_decl.inner_node.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    let mut out = Vec::new();
    for child in body.children(&mut cursor) {
        let Some((unit, inner)) = function_or_decorated(child) else {
            continue;
        };
        if inner.kind() != "function_definition" {
            continue;
        }
        let name = name_of(inner, source).unwrap_or("");
        out.push(DeclInfo {
            kind: DeclKind::Function,
            unit_node: unit,
            inner_node: inner,
            start_line: unit.start_position().row + 1,
            underscore_private: is_underscore_private(name),
        });
    }
    out
}

fn name_of<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let n = node.child_by_field_name("name")?;
    Some(&source[n.start_byte()..n.end_byte()])
}

fn is_underscore_private(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    if is_dunder(name) {
        return false;
    }
    name.starts_with('_')
}

fn is_dunder(name: &str) -> bool {
    name.starts_with("__") && name.ends_with("__") && name.len() >= 4
}

/// Returns the assignment target name if `expression_statement` looks
/// like a simple `NAME = …` or `NAME: TYPE = …`. Multi-target assignments
/// (`a = b = …`), tuple targets (`a, b = …`), and unpack targets are
/// rejected — they aren't NS-anchor shaped.
fn const_assignment_target<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let mut cursor = node.walk();
    let inner = node
        .children(&mut cursor)
        .find(|c| matches!(c.kind(), "assignment"))?;
    let left = inner.child_by_field_name("left")?;
    match left.kind() {
        "identifier" => Some(&source[left.start_byte()..left.end_byte()]),
        _ => None,
    }
}

// --- collectors ---------------------------------------------------------

fn collect_imports(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    let mut first_real_statement_seen = false;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "import_statement" | "import_from_statement" | "future_import_statement" => {
                extend_span(&mut lines, child, source);
                first_real_statement_seen = true;
            }
            "expression_statement" => {
                // Module docstring: only the very first statement in the
                // file qualifies. A post-assignment `"""…"""` "variable
                // docstring" elsewhere in the module is not a module
                // docstring and stays out of `Imports`.
                if !first_real_statement_seen && is_docstring_statement(child) {
                    extend_span(&mut lines, child, source);
                } else if let Some(target) = const_assignment_target(child, source)
                    && is_dunder(target)
                {
                    extend_span(&mut lines, child, source);
                }
                first_real_statement_seen = true;
            }
            "if_statement" => {
                if is_type_checking_import_block(child, source) {
                    extend_span(&mut lines, child, source);
                }
                first_real_statement_seen = true;
            }
            "comment" => {}
            _ => {
                first_real_statement_seen = true;
            }
        }
    }
    FileLines::new(dedup_sorted(lines))
}

/// True iff `if_stmt` is shaped like `if TYPE_CHECKING: <imports only>`
/// with no `elif` / `else` branch — the canonical Python idiom for
/// importing types only available to type-checkers. Restricting to this
/// shape rules out platform/version dispatch (`if sys.version_info:
/// import fast_impl`) where the runtime branch carries real
/// implementation choices, not plumbing.
fn is_type_checking_import_block(if_stmt: Node, source: &str) -> bool {
    let condition = if_stmt.child_by_field_name("condition");
    if !condition.is_some_and(|c| is_type_checking_condition(c, source)) {
        return false;
    }
    let mut cursor = if_stmt.walk();
    for child in if_stmt.children(&mut cursor) {
        if matches!(child.kind(), "elif_clause" | "else_clause") {
            return false;
        }
    }
    let Some(body) = if_stmt.child_by_field_name("consequence") else {
        return false;
    };
    let mut body_cursor = body.walk();
    let mut any = false;
    for child in body.named_children(&mut body_cursor) {
        any = true;
        if !matches!(
            child.kind(),
            "import_statement" | "import_from_statement" | "future_import_statement"
        ) {
            return false;
        }
    }
    any
}

/// Match `TYPE_CHECKING` and `typing.TYPE_CHECKING` (the two forms
/// accepted by mypy / pyright as the type-checking guard).
fn is_type_checking_condition(node: Node, source: &str) -> bool {
    match node.kind() {
        "identifier" => &source[node.start_byte()..node.end_byte()] == "TYPE_CHECKING",
        "attribute" => {
            let attr_text = &source[node.start_byte()..node.end_byte()];
            attr_text.ends_with(".TYPE_CHECKING")
        }
        _ => false,
    }
}

/// Surface listing of every top-level decl's first line. Class / def
/// entries emit a `Full + Ellipsis` pair so the body-elision marker
/// renders. Const entries emit Full only — there's no body, and the
/// next-row ellipsis would conflict with whatever statement follows
/// (e.g. a `TYPE_CHECKING = False` const followed immediately by `if
/// TYPE_CHECKING:`).
fn collect_decl_names_from(decls: &[DeclInfo]) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for decl in decls {
        full.push(decl.start_line);
        if matches!(decl.kind, DeclKind::Class | DeclKind::Function) {
            ellipses.push(decl.start_line + 1);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// One full + ellipsis pair per method across every class in the file.
fn collect_method_sigs_from(methods_by_class: &[(usize, Vec<DeclInfo>)]) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for (_, methods) in methods_by_class {
        for m in methods {
            full.push(m.start_line);
            ellipses.push(m.start_line + 1);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

fn collect_methods_by_class<'a>(
    decls: &[DeclInfo<'a>],
    source: &str,
) -> Vec<(usize, Vec<DeclInfo<'a>>)> {
    decls
        .iter()
        .filter(|d| d.kind == DeclKind::Class)
        .map(|d| (d.start_line, collect_methods_in_class(d, source)))
        .filter(|(_, ms)| !ms.is_empty())
        .collect()
}

fn collect_decl(info: &DeclInfo) -> FileLines {
    let unit_start = info.unit_node.start_position().row;
    let end_row = match info.kind {
        DeclKind::Function | DeclKind::Class => signature_end_row(info.inner_node)
            .saturating_sub(1)
            .max(unit_start),
        DeclKind::Const => info.unit_node.end_position().row,
    };
    let mut lines = Vec::new();
    push_rows(&mut lines, unit_start, end_row);
    FileLines::new(dedup_sorted(lines))
}

/// Rows of the docstring inside a `function_definition` / `class_definition`
/// body: the first `expression_statement(string)` after any leading
/// comments. Returns empty `FileLines` when no docstring.
fn collect_doc_for(inner: Node, source: &str) -> FileLines {
    let Some(body) = inner.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let Some(doc_stmt) = first_docstring_statement(body) else {
        return FileLines::new(Vec::new());
    };
    let mut lines = Vec::new();
    extend_span(&mut lines, doc_stmt, source);
    FileLines::new(dedup_sorted(lines))
}

/// The first named child of `body` that's a docstring statement, or
/// `None`. Skips comments.
fn first_docstring_statement<'a>(body: Node<'a>) -> Option<Node<'a>> {
    let mut cursor = body.walk();
    for child in body.named_children(&mut cursor) {
        if child.kind() == "comment" {
            continue;
        }
        if is_docstring_statement(child) {
            return Some(child);
        }
        // First non-comment, non-docstring statement → no docstring.
        return None;
    }
    None
}

/// True iff `node` is `expression_statement` whose first named child is
/// a `string` or `concatenated_string` — the AST shape tree-sitter-python
/// gives a docstring.
fn is_docstring_statement(node: Node) -> bool {
    if node.kind() != "expression_statement" {
        return false;
    }
    let mut cursor = node.walk();
    let Some(first) = node.named_children(&mut cursor).next() else {
        return false;
    };
    matches!(first.kind(), "string" | "concatenated_string")
}

/// Body interior rows of a def, sans leading docstring + blank lines.
fn collect_def_body(inner: Node, src_lines: &[&str]) -> FileLines {
    let Some(body) = inner.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let docstring_rows =
        first_docstring_statement(body).map(|n| (n.start_position().row, n.end_position().row));
    let body_start = body.start_position().row;
    let body_end = body.end_position().row;
    let mut out = Vec::new();
    for row in body_start..=body_end {
        if let Some((s, e)) = docstring_rows
            && row >= s
            && row <= e
        {
            continue;
        }
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(row + 1);
        }
    }
    if out.is_empty() {
        return FileLines::new(Vec::new());
    }
    FileLines::new(out)
}

/// Class body rows, skipping methods (decorated or not) and the leading
/// docstring. Includes class-level annotated assignments, `__slots__`,
/// constants, inline comments.
fn collect_class_body(inner: Node, src_lines: &[&str]) -> FileLines {
    let Some(body) = inner.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let docstring = first_docstring_statement(body);
    let mut method_ranges = Vec::new();
    let mut cursor = body.walk();
    for child in body.children(&mut cursor) {
        if let Some((unit, inner_node)) = function_or_decorated(child)
            && inner_node.kind() == "function_definition"
        {
            method_ranges.push((unit.start_position().row, unit.end_position().row));
        }
    }
    let body_start = body.start_position().row;
    let body_end = body.end_position().row;
    let mut out = Vec::new();
    'rows: for row in body_start..=body_end {
        for (s, e) in &method_ranges {
            if row >= *s && row <= *e {
                continue 'rows;
            }
        }
        if let Some(d) = docstring
            && row >= d.start_position().row
            && row <= d.end_position().row
        {
            continue;
        }
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(row + 1);
        }
    }
    FileLines::new(out)
}

/// Test functions across a `_test.py` / `test_*.py` file: every
/// top-level + class-body `def test_*` first line. The emitted line is
/// the inner `def test_*` row, not the decorator row — pytest
/// discovers tests by the def name, and a decorator-row anchor combined
/// with the Full+Ellipsis pair can hide the actual signature behind a
/// multi-line `@pytest.mark.parametrize(...)` block.
fn collect_test_function_starts(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if let Some((_, inner)) = function_or_decorated(child) {
            match inner.kind() {
                "function_definition" => push_test_fn_row(inner, source, &mut out),
                "class_definition" => collect_test_methods_in_class(inner, source, &mut out),
                _ => {}
            }
        }
    }
    dedup_sorted(out)
}

fn push_test_fn_row(inner: Node, source: &str, out: &mut Vec<usize>) {
    if let Some(name) = name_of(inner, source)
        && name.starts_with("test_")
    {
        out.push(inner.start_position().row + 1);
    }
}

fn collect_test_methods_in_class(class_node: Node, source: &str, out: &mut Vec<usize>) {
    let Some(body) = class_node.child_by_field_name("body") else {
        return;
    };
    let mut cursor = body.walk();
    for child in body.children(&mut cursor) {
        if let Some((_, inner)) = function_or_decorated(child)
            && inner.kind() == "function_definition"
        {
            push_test_fn_row(inner, source, out);
        }
    }
}

// --- value functions ----------------------------------------------------

fn python_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(file, ctx, is_python_entrypoint(file))
}

fn is_python_entrypoint(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| matches!(n, "__init__.py" | "__main__.py"))
}

fn imports_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.35, 0.55, 0.30, python_depth_factor(file, ctx))
}

fn decl_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.65, 0.55, 0.35, python_depth_factor(file, ctx))
}

fn method_sigs_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.55, 0.50, 0.30, python_depth_factor(file, ctx))
}

fn decl_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.70 * kv).min(1.0);
    let fu = (0.85 * kv).min(1.0);
    mix_signals(cat, fu, 0.65, python_depth_factor(file, ctx))
}

fn decl_doc_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.20 * kv).min(1.0);
    let fu = (0.60 * kv).min(1.0);
    mix_signals(cat, fu, 0.80, python_depth_factor(file, ctx))
}

fn decl_body_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.30 * kv).min(1.0);
    let fu = (0.80 * kv).min(1.0);
    mix_signals(cat, fu, 0.70, python_depth_factor(file, ctx))
}

fn class_body_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.35 * v).min(1.0);
    let fu = (0.70 * v).min(1.0);
    mix_signals(cat, fu, 0.55, python_depth_factor(file, ctx))
}

fn method_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.55 * v).min(1.0);
    let fu = (0.75 * v).min(1.0);
    mix_signals(cat, fu, 0.55, python_depth_factor(file, ctx))
}

fn method_doc_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.15 * v).min(1.0);
    let fu = (0.50 * v).min(1.0);
    mix_signals(cat, fu, 0.70, python_depth_factor(file, ctx))
}

fn method_body_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.20 * v).min(1.0);
    let fu = (0.65 * v).min(1.0);
    mix_signals(cat, fu, 0.55, python_depth_factor(file, ctx))
}

fn test_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.40, 0.0, 0.30, python_depth_factor(file, ctx))
}

// --- helpers ------------------------------------------------------------

fn is_test_file(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with("test_") || n.ends_with("_test.py"))
}

fn parse_python(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_python::LANGUAGE.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    fn parse(source: &str) -> (String, Tree) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("load python grammar");
        let tree = parser.parse(source, None).expect("parse");
        (source.to_string(), tree)
    }

    /// Re-collects methods using the source string so tests can assert
    /// on names. The runtime walker doesn't need the names — it only
    /// reads them for visibility classification, which is done after
    /// the fact below.
    fn methods_with_names<'a>(decls: &[DeclInfo<'a>], source: &str) -> Vec<(usize, String, bool)> {
        let mut out = Vec::new();
        for d in decls {
            if d.kind != DeclKind::Class {
                continue;
            }
            for m in collect_methods_in_class(d, source) {
                let name = name_of(m.inner_node, source).unwrap_or("").to_string();
                out.push((m.start_line, name.clone(), is_underscore_private(&name)));
            }
        }
        out
    }

    #[test]
    fn python_classify_top_level_decls() {
        let src = "\
\"\"\"module doc.\"\"\"

import os

__version__ = \"1.0\"
ENV_PREFIX = \"X\"

def helper():
    return 1

async def fetch():
    return 2

class Foo:
    pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let lines: Vec<_> = decls.iter().map(|d| (d.kind, d.start_line)).collect();
        // Module-doc + import + __version__ go into Imports. ENV_PREFIX is the
        // first Const decl. helper / fetch / Foo follow. (Line numbers are taken
        // from tree-sitter — verify against the source.)
        let kinds_only: Vec<_> = lines.iter().map(|(k, _)| *k).collect();
        assert_eq!(
            kinds_only,
            vec![
                DeclKind::Const,
                DeclKind::Function,
                DeclKind::Function,
                DeclKind::Class,
            ],
        );
    }

    #[test]
    fn python_decorator_span_includes_decorator_lines() {
        let src = "\
@final
@deprecated
class X:
    pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].start_line, 1, "decorator row is start_line");
        let lines = collect_decl(&decls[0]);
        // Span: decorators (1, 2) + class header (3) — 4 lines for "X:" included.
        assert!(lines.full.contains(&1));
        assert!(lines.full.contains(&2));
        assert!(lines.full.contains(&3));
    }

    #[test]
    fn python_decorated_methods_surface_in_method_sigs() {
        let src = "\
class C:
    @property
    def excinfo(self):
        return self._x

    @classmethod
    def from_call(cls):
        return cls()

    def plain(self):
        pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let methods = methods_with_names(&decls, &source);
        let names: Vec<&str> = methods.iter().map(|(_, n, _)| n.as_str()).collect();
        assert_eq!(names, vec!["excinfo", "from_call", "plain"]);
        // Decorator rows are start lines for decorated methods.
        let starts: Vec<_> = methods.iter().map(|(l, _, _)| *l).collect();
        assert_eq!(starts, vec![2, 6, 10]);
    }

    #[test]
    fn python_visibility_factor_underscore_vs_dunder() {
        assert!(!is_underscore_private("Foo"));
        assert!(!is_underscore_private("foo"));
        assert!(is_underscore_private("_helper"));
        assert!(!is_underscore_private("__init__"));
        assert!(!is_underscore_private("__call__"));
        assert!(is_underscore_private("__private_helper"));
    }

    #[test]
    fn python_method_sigs_lists_each_method() {
        let src = "\
class A:
    def one(self): pass
    def two(self): pass
    def three(self): pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let by_class = collect_methods_by_class(&decls, &source);
        let sigs = collect_method_sigs_from(&by_class);
        assert_eq!(sigs.full, vec![2, 3, 4]);
    }

    #[test]
    fn python_class_body_emits_fields_skipping_methods() {
        let src = "\
class HookOpts:
    \"\"\"Options.\"\"\"
    firstresult: bool
    historic: bool
    warn_on_impl: Warning | None

    def __init__(self):
        pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let class = decls.iter().find(|d| d.kind == DeclKind::Class).unwrap();
        let src_lines: Vec<&str> = source.lines().collect();
        let body = collect_class_body(class.inner_node, &src_lines);
        assert_eq!(
            body.full,
            vec![3, 4, 5],
            "fields kept; docstring + method dropped"
        );
    }

    #[test]
    fn python_class_with_only_methods_emits_no_class_body() {
        let src = "\
class A:
    \"\"\"Just methods.\"\"\"
    def one(self):
        pass
    def two(self):
        pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let class = decls.iter().find(|d| d.kind == DeclKind::Class).unwrap();
        let src_lines: Vec<&str> = source.lines().collect();
        let body = collect_class_body(class.inner_node, &src_lines);
        assert!(body.full.is_empty(), "got {:?}", body.full);
    }

    #[test]
    fn python_test_file_surfaces_only_test_funcs() {
        let src = "\
def helper():
    pass

@pytest.mark.parametrize(
    \"x\",
    [1, 2],
)
def test_first(x):
    assert x

class TestThing:
    def test_inner(self):
        pass
    def helper(self):
        pass

@pytest.mark.skip
class TestDecorated:
    def test_dec(self):
        pass

def test_top():
    pass
";
        let (source, tree) = parse(src);
        let starts = collect_test_function_starts(&tree, &source);
        // Emitted line is the inner `def test_*` row, not the decorator
        // row — keeps the signature visible at small budgets.
        assert_eq!(starts, vec![8, 12, 19, 22], "got {starts:?}");
    }

    #[test]
    fn python_module_docstring_in_imports() {
        let src = "\
\"\"\"This is the module.

It does things.
\"\"\"

import os
";
        let (source, tree) = parse(src);
        let lines = collect_imports(&tree, &source);
        // The 4-line docstring on rows 1-4 plus the import on row 6.
        for r in [1, 2, 3, 4, 6] {
            assert!(lines.full.contains(&r), "missing {r}: {:?}", lines.full);
        }
    }

    #[test]
    fn python_type_checking_block_in_imports_but_not_runtime_dispatch() {
        // `if TYPE_CHECKING: import …` is plumbing — folded into Imports.
        let src_tc = "\
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from foo import Bar
";
        let (source, tree) = parse(src_tc);
        let lines = collect_imports(&tree, &source);
        assert!(
            lines.full.contains(&3),
            "TYPE_CHECKING block missing: {:?}",
            lines.full
        );
        assert!(lines.full.contains(&4));

        // `if sys.version_info: import fast` is runtime dispatch — must
        // NOT be classified as plumbing (codex adversarial review fix).
        let src_runtime = "\
import sys

if sys.version_info >= (3, 11):
    from foo import fast_impl as Impl
else:
    from foo import slow_impl as Impl
";
        let (source, tree) = parse(src_runtime);
        let lines = collect_imports(&tree, &source);
        assert!(
            !lines.full.contains(&3),
            "runtime version-dispatch must not be in Imports: {:?}",
            lines.full,
        );
    }

    #[test]
    fn python_all_assignment_in_imports() {
        let src = "\
import os

__all__ = [
    \"foo\",
    \"bar\",
]
";
        let (source, tree) = parse(src);
        let lines = collect_imports(&tree, &source);
        // Multi-line __all__ literal lines 3-6 plus import on line 1.
        for r in [1, 3, 4, 5, 6] {
            assert!(lines.full.contains(&r), "missing {r}: {:?}", lines.full);
        }
    }

    #[test]
    fn python_module_constant_emits_decl() {
        let src = "ENV_PREFIX = \"x\"\n";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].kind, DeclKind::Const);
        assert_eq!(decls[0].start_line, 1);
        let names = collect_decl_names_from(&decls);
        assert_eq!(names.full, vec![1]);
    }

    #[test]
    fn python_docstring_extracted_from_expression_statement() {
        let src = "\
def f():
    \"\"\"docstring.\"\"\"
    return 1
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let f = &decls[0];
        let doc = collect_doc_for(f.inner_node, &source);
        assert_eq!(doc.full, vec![2]);
        let src_lines: Vec<&str> = source.lines().collect();
        let body = collect_def_body(f.inner_node, &src_lines);
        assert_eq!(body.full, vec![3], "body skips docstring on row 2");
    }

    #[test]
    fn python_real_dir_renders_seeded_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("__init__.py"),
            "from .core import greet\n\n__all__ = [\"greet\"]\n",
        )
        .unwrap();
        std::fs::write(
            root.join("core.py"),
            "\"\"\"Core module.\"\"\"\n\ndef greet(name):\n    \"\"\"Say hi.\"\"\"\n    return f\"hello {name}\"\n\nclass Person:\n    \"\"\"A person.\"\"\"\n    name: str\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            rendered.contains("def greet(name)"),
            "missing function signature:\n{rendered}",
        );
        assert!(
            rendered.contains("class Person"),
            "missing class header:\n{rendered}",
        );
        assert!(
            rendered.contains("__all__"),
            "missing __all__ in imports surface:\n{rendered}",
        );

        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Python(PythonKey::Decl { .. }))),
            "expected a Python::Decl batch; keys: {keys:?}"
        );
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Python(PythonKey::Imports { .. }))),
            "expected a Python::Imports batch; keys: {keys:?}"
        );
    }

    #[test]
    fn python_test_file_only_emits_test_names() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("test_something.py"),
            "def test_one():\n    pass\n\ndef helper():\n    pass\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        let test_decl_count = keys
            .iter()
            .filter(|k| matches!(k, BatchKey::Python(PythonKey::Decl { .. })))
            .count();
        assert_eq!(
            test_decl_count, 0,
            "expected no Python::Decl batches in a test file; keys: {keys:?}"
        );
    }
}
