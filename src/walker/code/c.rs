//! C extraction for the code engine.
//!
//! The declarations of a file are its "effective top level": the body of
//! a wrapping `#ifndef X` / `#define X` / `#endif` header guard, the body
//! of `extern "C" { … }` (bare or `#ifdef __cplusplus`-wrapped), and
//! `#if` / `#ifdef` blocks holding only declarations and directives.
//! `#ifndef X` / `#define X …` supplying a default counts as a
//! declaration, and so do function definitions in a source file and
//! `inline` ones in a header; another definition in a header marks the
//! implementation section of a single-header library, which stays
//! opaque, as does any block holding a statement other than an
//! expression (an `#if` splitting a function body).
//!
//! - Function definitions are `Callable`, and so is a function-like
//!   macro: its `#define NAME(args)` rows are the head, each later
//!   continuation row a body item. Prototypes, typedefs, structs / unions / enums,
//!   global variables and object-like macros are `Whole`, with one body
//!   [`Item`] per field or enumerator.
//! - A `static` in a header is hidden unless a specifier spells it
//!   inline (`inline`, `__inline`, `SDS_INLINE`; not `noinline`).
//! - A declaration's doc is the comment run directly above it, or above
//!   the feature gate it opens.
//!
//! A header written in C++ ([`is_cpp_header`]) is left to the plaintext
//! fallback, like a `.hpp`.

use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{
    Language, MAX_SCOPE_NESTING, SourceFile, ends_in_block_comment, has_extension, is_named_after,
    named_children,
};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["c", "h"],
    grammar: |_| tree_sitter_c::LANGUAGE.into(),
    extract,
    is_entrypoint: Some(is_entrypoint),
    file_weight: Some(file_weight),
    sibling_mentions: None,
    sibling_names: None,
};

/// Every batch of a `.c` file, relative to a header. Not tuned.
const IMPLEMENTATION_FILE_WEIGHT: f64 = 0.6;

fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    if is_cpp_header(&file.path, &file.source) {
        return FileModel::default();
    }
    let in_header = is_header(&file.path);
    let mut decls = Vec::new();
    let mut directives = Vec::new();
    let mut visit = |node: Node, enclosing: Option<GateBranch>| match enclosing {
        Some(enclosing) if node.kind().starts_with('#') => {
            directives.push(gate_directive(node, enclosing, file));
        }
        _ => decls.extend(declaration(node, file, in_header)),
    };
    let top_level = header_guard(root, file).unwrap_or(root);
    visit_with_envelope_descent(top_level, file, &mut visit);
    attach_directives(&mut decls, &directives);
    FileModel {
        decls,
        ..FileModel::default()
    }
}

/// A header named after the repository within one directory of the root
/// (`sds.h`, `src/jq.h`), or in an `include/` directory within two
/// (`src/include/liburing.h`): the library's public interface.
fn is_entrypoint(path: &Path, ctx: &WalkCtx) -> bool {
    let max_depth = if path.parent().and_then(Path::file_name) == Some("include".as_ref()) {
        3
    } else {
        2
    };
    is_header(path) && ctx.depth_from_root(path) <= max_depth && is_named_after(path, ctx.root())
}

fn file_weight(path: &Path, _ctx: &WalkCtx) -> f64 {
    if is_header(path) {
        1.0
    } else {
        IMPLEMENTATION_FILE_WEIGHT
    }
}

/// Whether `path` is a header written in C++, which the C grammar
/// misparses (namespaces read as functions, classes as statement lists,
/// templates as expressions): a line of `source` opens a namespace or a
/// class, starts a template, or is an access specifier, outside block
/// comments and the `#if … __cplusplus` blocks a C header keeps for C++
/// callers.
pub(in crate::walker) fn is_cpp_header(path: &Path, source: &str) -> bool {
    if !is_header(path) {
        return false;
    }
    static CPP_ONLY_LINE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(
            r"^\s*(namespace(\s+[A-Za-z_][\w:]*)?\s*\{",
            r"|class\s+[A-Za-z_]\w*(\s+[A-Za-z_]\w*)?\s*(final\s*)?[{:;]",
            r"|template\s*<",
            r"|(public|private|protected)\s*:)",
        ))
        .expect("valid regex")
    });
    let mut cplusplus_depth = 0usize;
    let mut in_block_comment = false;
    for line in source.lines() {
        let starts_in_block_comment = in_block_comment;
        in_block_comment = ends_in_block_comment(line, in_block_comment);
        if starts_in_block_comment {
            continue;
        }
        if let Some(directive) = line.trim_start().strip_prefix('#') {
            let directive = directive.trim_start();
            if cplusplus_depth > 0 {
                if directive.starts_with("if") {
                    cplusplus_depth += 1;
                } else if directive.starts_with("endif") {
                    cplusplus_depth -= 1;
                }
            } else if directive.starts_with("if") && directive.contains("__cplusplus") {
                cplusplus_depth = 1;
            }
        } else if cplusplus_depth == 0 && CPP_ONLY_LINE.is_match(line) {
            return true;
        }
    }
    false
}

fn is_header(path: &Path) -> bool {
    has_extension(path, &["h"])
}

fn declaration(node: Node, file: &SourceFile, in_header: bool) -> Option<DeclInfo> {
    let shape = match node.kind() {
        "function_definition" | "preproc_function_def" => Shape::Callable,
        "declaration" | "type_definition" | "struct_specifier" | "union_specifier"
        | "enum_specifier" => Shape::Whole,
        "preproc_def" if !is_header_guard_define(node, file) => Shape::Whole,
        _ => return None,
    };
    if is_flattened_parse_debris(node, file) {
        return None;
    }
    // A header's `static inline` definition is the header-only accessor
    // idiom: part of the API. Any other `static` in a header is an
    // implementation leak.
    if in_header && is_static(node, file) && !is_inline(node, file) {
        return None;
    }
    let rows = file.node_rows(node);
    let decl = match shape {
        Shape::Callable if node.kind() == "preproc_function_def" => {
            let first = *rows.start();
            let signature_end = node
                .child_by_field_name("parameters")
                .map_or(first, |parameters| *file.node_rows(parameters).end());
            DeclInfo {
                body: (signature_end + 1..=*rows.end())
                    .map(|row| Item::new([row]))
                    .collect(),
                ..DeclInfo::new(
                    vec![first],
                    (first..=signature_end).collect(),
                    Shape::Callable,
                )
            }
        }
        Shape::Callable => {
            let block = node.child_by_field_name("body");
            let mut statements = named_children(block);
            if statements.iter().all(|child| child.kind() == "comment") {
                statements.clear();
            }
            let open_row = block.map_or(*rows.start(), |block| block.start_position().row + 1);
            file.callable(name_rows(node), rows, statements, open_row)
        }
        Shape::Whole => file.whole(name_rows(node), rows, aggregate_body(node)),
    };
    Some(DeclInfo {
        doc: doc_above(node, file),
        ..decl
    })
}

/// The comment run directly above `node`, or, when `node` opens a
/// descended feature gate's first branch, the run above that gate.
fn doc_above(node: Node, file: &SourceFile) -> Vec<Item> {
    let mut anchor = node;
    loop {
        let doc = file.comment_paragraphs_above(anchor);
        let Some(gate) = anchor.parent() else {
            return doc;
        };
        let mut previous = anchor.prev_named_sibling();
        while let Some(comment) = previous.filter(|node| node.kind() == "comment") {
            previous = comment.prev_named_sibling();
        }
        let opens_gate = matches!(gate.kind(), "preproc_if" | "preproc_ifdef")
            && previous.is_some_and(|previous| gate.named_child(0) == Some(previous));
        if !doc.is_empty() || !opens_gate || is_header_guard(gate, file) {
            return doc;
        }
        anchor = gate;
    }
}

/// A directive of a descended feature gate: its rows (through the end
/// of a multiline condition), the rows of the whole gate, and whether
/// the gate has an `#elif` / `#else` alternate.
struct GateDirective {
    rows: RangeInclusive<usize>,
    gate_rows: RangeInclusive<usize>,
    has_alternates: bool,
}

/// A descended feature gate and the branch of it that holds a node: the
/// gate itself (alternate 0) or its `alternate`th `#elif` / `#else`.
#[derive(Clone, Copy)]
struct GateBranch<'a> {
    gate: Node<'a>,
    branch: Node<'a>,
    alternate: usize,
}

/// How many alternates of one gate are descended. C99 requires compilers
/// to support 1023 case labels in one `switch`, the construct an `#elif`
/// chain spells for the preprocessor.
const MAX_GATE_ALTERNATES: usize = 1023;

/// `token` is an `#if` / `#ifdef` / `#elif` / `#else` / `#endif` token
/// of the branch `enclosing` names.
fn gate_directive(token: Node, enclosing: GateBranch, file: &SourceFile) -> GateDirective {
    let GateBranch { gate, branch, .. } = enclosing;
    let start_row = token.start_position().row + 1;
    let end_row = if token.kind() == "#endif" {
        start_row
    } else {
        branch
            .child_by_field_name("condition")
            .or_else(|| branch.child_by_field_name("name"))
            .map_or(start_row, |condition| condition.end_position().row + 1)
    };
    GateDirective {
        rows: start_row..=end_row,
        gate_rows: file.node_rows(gate),
        has_alternates: gate.child_by_field_name("alternative").is_some(),
    }
}

/// Adds each feature gate directive to the head of the nearest
/// declaration inside its gate: the next one, or, for an `#endif` or a
/// directive opening an empty trailing branch, the previous one. When
/// the gate has alternates, a directive opening the next declaration's
/// branch also joins its name rows, so the roster doesn't list each
/// branch's declarations as if all of them held; a single-branch gate's
/// directive stays out of the roster's cost. An enclosing gate's directives
/// land on the same declarations as the nested gate's, so a gated
/// declaration never renders without any of its conditions.
fn attach_directives(decls: &mut [DeclInfo], directives: &[GateDirective]) {
    let spans: Vec<(usize, usize)> = decls
        .iter()
        .map(|decl| {
            let body = decl.body.iter().flat_map(|item| item.rows.iter());
            let rows = decl.head.iter().chain(body).copied();
            (rows.clone().min().unwrap_or(0), rows.max().unwrap_or(0))
        })
        .collect();
    let mut by_first: Vec<(usize, usize)> = spans
        .iter()
        .enumerate()
        .map(|(index, (first, _))| (*first, index))
        .collect();
    by_first.sort_unstable();
    let starting_from = |row: usize| by_first.partition_point(|(first, _)| *first < row);
    for directive in directives {
        let row = *directive.rows.start();
        let (gate_start, gate_end) = (*directive.gate_rows.start(), *directive.gate_rows.end());
        let in_gate = &by_first[starting_from(gate_start)..starting_from(gate_end + 1)];
        let next = by_first[starting_from(gate_start.max(row + 1))..]
            .first()
            .filter(|(first, _)| *first <= gate_end);
        if let Some(&(_, index)) = next {
            if directive.has_alternates {
                decls[index].name_rows.extend(directive.rows.clone());
            }
            decls[index].head.extend(directive.rows.clone());
        } else if let Some(index) = in_gate
            .iter()
            .map(|&(_, index)| (spans[index].1, index))
            .filter(|(last, _)| *last < row)
            .max()
            .map(|(_, index)| index)
        {
            decls[index].head.extend(directive.rows.clone());
        }
    }
}

/// The declaration's first row, plus the row of each name it declares:
/// the `} Name;` row of a `typedef struct { … } Name;`, the `name(` row of
/// a definition whose return type sits on the row above.
fn name_rows(node: Node) -> Vec<usize> {
    let mut rows = vec![node.start_position().row + 1];
    rows.extend(
        node.children_by_field_name("declarator", &mut node.walk())
            .filter_map(declared_name)
            .map(|name| name.start_position().row + 1),
    );
    rows
}

/// A leftover of a region tree-sitter could not parse and flattened
/// onto the top level: the `#define X` after the orphaned name of an
/// `#ifndef X` (a header guard whose block was dropped), or a declaration
/// after an orphaned function declarator (a local of the broken body).
fn is_flattened_parse_debris(node: Node, file: &SourceFile) -> bool {
    let Some(previous) = node.prev_named_sibling() else {
        return false;
    };
    match node.kind() {
        "preproc_def" => {
            previous.kind() == "identifier"
                && node
                    .parent()
                    .is_some_and(|parent| !parent.kind().starts_with("preproc"))
                && is_define_of(node, previous, file)
        }
        "declaration" => previous.kind() == "function_declarator",
        _ => false,
    }
}

fn declared_name(mut declarator: Node) -> Option<Node> {
    while !matches!(
        declarator.kind(),
        "identifier" | "type_identifier" | "field_identifier"
    ) {
        declarator = declarator
            .child_by_field_name("declarator")
            .or_else(|| declarator.named_child(0))?;
    }
    Some(declarator)
}

/// The struct / union / enum body a declaration defines, unless the
/// declaration is a function prototype (`struct point make(void);`).
fn aggregate_body(node: Node) -> Option<Node> {
    let specifier = match node.kind() {
        "struct_specifier" | "union_specifier" | "enum_specifier" => node,
        "declaration" | "type_definition" => {
            if node
                .children_by_field_name("declarator", &mut node.walk())
                .any(declarator_is_function)
            {
                return None;
            }
            node.child_by_field_name("type")?
        }
        _ => return None,
    };
    if !matches!(
        specifier.kind(),
        "struct_specifier" | "union_specifier" | "enum_specifier"
    ) {
        return None;
    }
    specifier
        .child_by_field_name("body")
        .filter(|body| matches!(body.kind(), "field_declaration_list" | "enumerator_list"))
}

fn declarator_is_function(mut node: Node) -> bool {
    while matches!(
        node.kind(),
        "pointer_declarator" | "parenthesized_declarator" | "init_declarator"
    ) {
        let Some(inner) = node
            .child_by_field_name("declarator")
            .or_else(|| node.named_child(0))
        else {
            return false;
        };
        node = inner;
    }
    node.kind() == "function_declarator"
}

/// Visits the named children of `top_level` in source order, descending
/// into each `extern "C" { … }` envelope (raw or `#ifdef
/// __cplusplus`-wrapped) and declaration-only feature gate. A descended
/// gate's children are visited, directive tokens (`#ifdef`, `#else`,
/// `#endif`) included, plus the bodies of its `#else` / `#elif`
/// alternates; nested gates descend (or stay opaque) on their own merits.
/// An envelope or gate counts as a scope, and one inside
/// [`MAX_SCOPE_NESTING`] others stays opaque; alternates past a gate's
/// [`MAX_GATE_ALTERNATES`]th stay opaque too.
fn visit_with_envelope_descent<'a>(
    top_level: Node<'a>,
    file: &SourceFile,
    mut visit: impl FnMut(Node<'a>, Option<GateBranch<'a>>),
) {
    let mut gate_answers = HashMap::new();
    // Each node with the gate branch it sits in, and the number of scopes
    // around it.
    let mut pending: Vec<(Node, Option<GateBranch>, usize)> = top_level
        .named_children(&mut top_level.walk())
        .map(|node| (node, None, 0))
        .collect();
    pending.reverse();
    while let Some((node, enclosing, depth)) = pending.pop() {
        let (children, enclosing): (Vec<Node>, _) = if depth >= MAX_SCOPE_NESTING {
            visit(node, enclosing);
            continue;
        } else if let Some(GateBranch {
            gate, alternate, ..
        }) = enclosing.filter(|enclosing| {
            is_gate_alternate(node.kind()) && enclosing.alternate < MAX_GATE_ALTERNATES
        }) {
            let branch = GateBranch {
                gate,
                branch: node,
                alternate: alternate + 1,
            };
            (node.children(&mut node.walk()).collect(), Some(branch))
        } else if let Some(decl_list) = extern_c_declaration_list(node, file) {
            (named_children(Some(decl_list)), None)
        } else if matches!(node.kind(), "preproc_if" | "preproc_ifdef")
            && !is_disabled_preproc_if(node, file)
            && feature_gate_is_declaration_only(node, file, &mut gate_answers) == Some(true)
        {
            let branch = GateBranch {
                gate: node,
                branch: node,
                alternate: 0,
            };
            (node.children(&mut node.walk()).collect(), Some(branch))
        } else {
            visit(node, enclosing);
            continue;
        };
        pending.extend(children.into_iter().rev().map(|child| {
            let child_depth = if is_gate_alternate(child.kind()) {
                depth
            } else {
                depth + 1
            };
            (child, enclosing, child_depth)
        }));
    }
}

fn is_gate_alternate(kind: &str) -> bool {
    matches!(kind, "preproc_else" | "preproc_elif" | "preproc_elifdef")
}

/// True for `#if 0` blocks: commented-out code, not a feature gate.
fn is_disabled_preproc_if(node: Node, file: &SourceFile) -> bool {
    node.child_by_field_name("condition")
        .is_some_and(|condition| file.text(condition).trim() == "0")
}

/// For a `preproc_if*` / `preproc_else*` subtree: `None` when any branch
/// holds a statement, or a non-`inline` function definition in a header,
/// else whether it holds at least one declaration beside its directives,
/// comments and other expression statements. A macro definition counts
/// only when it supplies a default for the name the block tests
/// (`#ifndef X` / `#define X …`). Answers are
/// memoized in `answers` by node id and found bottom-up, so nested gates
/// cost time linear in their size and no stack depth.
fn feature_gate_is_declaration_only(
    gate: Node,
    file: &SourceFile,
    answers: &mut HashMap<usize, Option<bool>>,
) -> Option<bool> {
    let is_nested_gate = |node: Node| {
        matches!(node.kind(), "preproc_if" | "preproc_ifdef") || is_gate_alternate(node.kind())
    };
    let mut pending = vec![gate];
    while let Some(&node) = pending.last() {
        let unanswered: Vec<Node> = node
            .named_children(&mut node.walk())
            .filter(|child| is_nested_gate(*child) && !answers.contains_key(&child.id()))
            .collect();
        if !unanswered.is_empty() {
            pending.extend(unanswered);
            continue;
        }
        pending.pop();
        let condition = node
            .child_by_field_name("condition")
            .or_else(|| node.child_by_field_name("name"));
        let mut answer = Some(false);
        for child in node.named_children(&mut node.walk()) {
            if Some(child) == condition {
                continue;
            }
            let declares = match child.kind() {
                "declaration" | "type_definition" | "struct_specifier" | "union_specifier"
                | "enum_specifier" => Some(true),
                "preproc_def" | "preproc_function_def" => {
                    Some(guarded_name(node).is_some_and(|name| is_define_of(child, name, file)))
                }
                "function_definition" if !is_header(&file.path) || is_inline(child, file) => {
                    Some(true)
                }
                "expression_statement" => Some(false),
                "preproc_include" | "preproc_call" | "comment" => Some(false),
                _ if is_nested_gate(child) => answers[&child.id()],
                _ => None,
            };
            answer = answer
                .zip(declares)
                .map(|(found, declares)| found || declares);
        }
        answers.insert(node.id(), answer);
    }
    answers[&gate.id()]
}

/// If `node` is an `extern "C" { … }` envelope (raw or
/// `#ifdef __cplusplus`-wrapped), the inner `declaration_list`. Any other
/// `#ifdef` around a linkage spec is a feature gate and stays opaque.
fn extern_c_declaration_list<'a>(node: Node<'a>, file: &SourceFile) -> Option<Node<'a>> {
    let linkage = match node.kind() {
        "linkage_specification" => node,
        "preproc_ifdef" => cplusplus_wrapped_linkage_specification(node, file)?,
        _ => return None,
    };
    linkage
        .children(&mut linkage.walk())
        .find(|child| child.kind() == "declaration_list")
}

/// If `ifdef` is `#ifdef __cplusplus` / linkage spec / `#endif`, the
/// linkage spec.
fn cplusplus_wrapped_linkage_specification<'a>(
    ifdef: Node<'a>,
    file: &SourceFile,
) -> Option<Node<'a>> {
    let mut cursor = ifdef.walk();
    let mut children = ifdef.children(&mut cursor);
    if children.next()?.kind() != "#ifdef" {
        return None;
    }
    let name = children.next()?;
    if name.kind() != "identifier" || file.text(name) != "__cplusplus" {
        return None;
    }
    let mut found = None;
    for child in children {
        match child.kind() {
            "#endif" | "\n" | "comment" => {}
            "linkage_specification" if found.is_none() => found = Some(child),
            _ => return None,
        }
    }
    found
}

/// The `#ifndef X` (or `#if !defined(X)`) / `#define X` / `#endif` block
/// wrapping the whole file (comments aside), whose children are the
/// effective top level.
fn header_guard<'a>(root: Node<'a>, file: &SourceFile) -> Option<Node<'a>> {
    let mut candidate = None;
    for child in root.children(&mut root.walk()) {
        match child.kind() {
            "comment" => {}
            "preproc_if" | "preproc_ifdef"
                if candidate.is_none() && is_header_guard(child, file) =>
            {
                candidate = Some(child);
            }
            _ => return None,
        }
    }
    candidate
}

/// The `X` that `guard` tests as `#ifndef X` or `#if !defined(X)`.
fn guarded_name(guard: Node) -> Option<Node> {
    match guard.kind() {
        "preproc_ifdef" => guard
            .child(0)
            .is_some_and(|token| token.kind() == "#ifndef")
            .then(|| guard.child_by_field_name("name"))
            .flatten(),
        "preproc_if" => {
            let condition = guard
                .child_by_field_name("condition")
                .filter(|condition| condition.kind() == "unary_expression")?;
            condition
                .child_by_field_name("operator")
                .is_some_and(|operator| operator.kind() == "!")
                .then(|| condition.child_by_field_name("argument"))
                .flatten()
                .filter(|argument| argument.kind() == "preproc_defined")?
                .named_child(0)
        }
        _ => None,
    }
}

/// True iff `guard` tests `X` as in [`guarded_name`], its first child
/// (comments aside) is `#define X`, and something follows it: a block
/// holding only the define supplies a default for `X`.
fn is_header_guard(guard: Node, file: &SourceFile) -> bool {
    let Some(name) = guarded_name(guard) else {
        return false;
    };
    let condition = guard.child_by_field_name("condition");
    let mut cursor = guard.walk();
    let mut body = guard
        .named_children(&mut cursor)
        .filter(|child| *child != name && Some(*child) != condition && child.kind() != "comment");
    body.next()
        .is_some_and(|define| is_define_of(define, name, file))
        && body.next().is_some()
}

/// True iff `define` is the `#define X` of a header guard on `X`.
fn is_header_guard_define(define: Node, file: &SourceFile) -> bool {
    define.parent().is_some_and(|guard| {
        is_header_guard(guard, file)
            && guarded_name(guard).is_some_and(|name| is_define_of(define, name, file))
    })
}

fn is_define_of(define: Node, name: Node, file: &SourceFile) -> bool {
    matches!(define.kind(), "preproc_def" | "preproc_function_def")
        && define
            .child_by_field_name("name")
            .is_some_and(|defined| file.text(defined) == file.text(name))
}

fn is_static(node: Node, file: &SourceFile) -> bool {
    node.children(&mut node.walk()).any(|child| {
        child.kind() == "storage_class_specifier" && file.text(child).trim() == "static"
    })
}

/// `inline` in any spelling: the keyword, a compiler's own (`__inline__`,
/// `__forceinline`), or a macro for one (`__always_inline`, `LIB_INLINE`),
/// which the grammar reads as the type. `noinline` / `NO_INLINE` are not.
fn is_inline(node: Node, file: &SourceFile) -> bool {
    node.children(&mut node.walk()).any(|child| {
        let spelling = file.text(child).to_ascii_lowercase();
        matches!(child.kind(), "storage_class_specifier" | "type_identifier")
            && spelling.contains("inline")
            && !spelling.contains("noinline")
            && !spelling.contains("no_inline")
    })
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{describe, rows};
    use super::*;

    fn model(file_name: &str, source: &str) -> FileModel {
        super::super::test_support::extract_source(&LANGUAGE, file_name, source)
    }

    fn name_rows_of(model: &FileModel) -> Vec<Vec<usize>> {
        model
            .decls
            .iter()
            .map(|decl| decl.name_rows.clone())
            .collect()
    }

    fn heads_of(model: &FileModel) -> Vec<Vec<usize>> {
        model.decls.iter().map(|decl| decl.head.clone()).collect()
    }

    #[test]
    fn c_header_guard_and_extern_c_are_descended() {
        let guarded = "\
#ifndef soluna_version_h
#define soluna_version_h
#define VERSION_MAJOR 1
int foo(void);
#endif
";
        let guarded = model("version.h", guarded);
        assert_eq!(name_rows_of(&guarded), vec![vec![3], vec![4]]);

        let if_guarded = "\
#if !defined(DECCONTEXT)
#define DECCONTEXT
#define DEC_INIT_BASE 0
int dec_init(void);
#endif
";
        let if_guarded = model("decContext.h", if_guarded);
        assert_eq!(name_rows_of(&if_guarded), vec![vec![3], vec![4]]);

        let wrapped = "\
#ifdef __cplusplus
extern \"C\" {
#endif

int foo(int x);
typedef int bar_t;

#ifdef __cplusplus
}
#endif
";
        let wrapped = model("api.h", wrapped);
        assert_eq!(name_rows_of(&wrapped), vec![vec![5], vec![6]]);
        assert!(wrapped.decls.iter().all(|decl| decl.shape == Shape::Whole));
    }

    #[test]
    fn c_feature_gates_descend_unless_they_hold_code() {
        let source = "\
#ifdef HAVE_SIMD
int search_simd(const char *text);
#else
int search_scalar(const char *text);
#endif
#if 0
int disabled(void);
#endif
#ifdef FEATURE_X
extern \"C\" {
int gated_linkage(int x);
}
#endif
#ifdef HAVE_IMPL
static inline int wraps_code(void) { return 1; }
#endif
#ifdef HAVE_PRAGMA
#pragma pack(push, 1)
int packed(void);
#endif
#ifdef SPLIT_BODY_TAIL
  counter += 1;
}
#endif
#ifdef HAVE_MODULE
static int module_fn(int x) { return x; }
REGISTER_MODULE(module_fn);
#endif
";
        assert_eq!(
            name_rows_of(&model("krep.c", source)),
            vec![vec![1, 2], vec![3, 4], vec![15], vec![19], vec![26]]
        );
        assert_eq!(
            name_rows_of(&model("krep.h", source)),
            vec![vec![1, 2], vec![3, 4], vec![15], vec![19]]
        );
    }

    /// A target-selection chain is one gate however many alternates it
    /// has, so its length is not nesting.
    #[test]
    fn c_feature_gate_alternates_do_not_count_as_nesting() {
        let alternates = MAX_SCOPE_NESTING + 16;
        let mut source =
            String::from("#ifndef CHIP_H\n#define CHIP_H\n#if defined(CHIP0)\nint chip0;\n");
        for index in 1..alternates {
            source.push_str(&format!("#elif defined(CHIP{index})\nint chip{index};\n"));
        }
        source.push_str("#endif\n#endif\n");
        assert_eq!(model("chip.h", &source).decls.len(), alternates);
    }

    #[test]
    fn c_feature_gates_supplying_a_default_list_their_macro() {
        let source = "\
#ifndef R3_H
#define R3_H
#ifndef CFG_1
#define CFG_1 (1)
#endif
#ifndef CFG_MAX
#define CFG_MAX(a, b) ((a) > (b) ? (a) : (b))
#endif
#ifdef __GNUC__
#define CFG_UNUSED __attribute__((unused))
#else
#define CFG_UNUSED
#endif
int use_cfg(void);
#endif
";
        assert_eq!(
            heads_of(&model("r3.h", source)),
            vec![vec![3, 4, 5], vec![6, 7, 8], vec![14]]
        );
    }

    #[test]
    fn c_feature_gate_directives_join_the_declarations_they_wrap() {
        let source = "\
#ifndef GLOBALS_H
#define GLOBALS_H
#ifdef ESP_PLATFORM
  void task_yield(void);
#else
  #define task_yield()
#endif
#ifdef EMPTY_BRANCH
int only_branch;
#else
#endif
#ifdef PLATFORM
#if defined(FEATURE) && \\
    defined(OTHER)
int api(void);
#endif
#endif
int after;
#endif
";
        assert_eq!(
            heads_of(&model("globals.h", source)),
            vec![
                vec![3, 4],
                vec![5, 6, 7],
                vec![8, 9, 10, 11],
                vec![12, 13, 14, 15, 16, 17],
                vec![18],
            ]
        );
    }

    #[test]
    fn c_statics_are_hidden_in_headers_only() {
        let source = "\
static inline int sdslen(const char *s) { return 0; }
static int helper(void) { return 0; }
static const int table_size = 4;
int api(void);
static __inline__ int gnu_inline(int x) { return x; }
static __forceinline int msvc_inline(int x) { return x; }
static __always_inline void
macro_inline(const volatile void *v)
{
	check(v);
}
static noinline int never_inlined(void) { return 0; }
static NO_INLINE int never_inlined_either(void) { return 0; }
";
        let listed = |file_name| {
            model(file_name, source)
                .decls
                .iter()
                .map(|decl| decl.name_rows[0])
                .collect::<Vec<_>>()
        };
        assert_eq!(listed("sds.h"), vec![1, 4, 5, 6, 7]);
        assert_eq!(listed("sds.c"), vec![1, 2, 3, 4, 5, 6, 7, 12, 13]);
    }

    #[test]
    fn c_function_definition_splits_head_from_statements() {
        let source = "\
#include \"sds.h\"
/* Build a string. */
static sds
sdsnewlen(const void *init,
          size_t initlen)
{
    // pick the header size
    char type = sdsReqType(initlen);
    if (type == SDS_TYPE_5) {
        type = SDS_TYPE_8;
    }
    return s; /* done */
}
int one_liner(void) { return 1; }
void empty(void) {
    /* nothing */
}
int add(int a,
        int b) { int sum = a + b;
    return sum;
}
int brace_below(void) { return 0;
}
";
        let model = model("sds.c", source);
        let [sdsnewlen, one_liner, empty, add, brace_below] = &model.decls[..] else {
            panic!("{:?}", model.decls);
        };
        assert_eq!(sdsnewlen.shape, Shape::Callable);
        assert_eq!(sdsnewlen.name_rows, vec![3, 4]);
        assert_eq!(sdsnewlen.head, vec![3, 4, 5, 6]);
        assert_eq!(
            rows(&sdsnewlen.body),
            vec![vec![7, 8], vec![9, 10, 11], vec![12]]
        );
        assert_eq!(rows(&sdsnewlen.doc), vec![vec![2]]);
        assert!(model.module_doc.is_empty());

        assert_eq!(one_liner.head, vec![14]);
        assert!(one_liner.body.is_empty());
        assert_eq!(empty.head, vec![15, 16, 17]);
        assert!(empty.body.is_empty());
        assert_eq!(add.head, vec![18, 19]);
        assert_eq!(rows(&add.body), vec![vec![20]]);
        assert_eq!(brace_below.head, vec![22, 23]);
        assert!(brace_below.body.is_empty());
    }

    /// An aggregate body lists its fields; a prototype returning a
    /// struct is all head.
    #[test]
    fn c_anonymous_typedef_lists_its_name_row() {
        let source = "\
typedef struct {
    // the needle
    const char *pattern;
    size_t len; /* bytes */
} search_params_t;

typedef enum {
    TY_INT,
    TY_PTR, TY_ARRAY,
} TypeKind;

struct Node {
    NodeKind kind; };
struct tm *gmtime_r(const time_t *timep,
                    struct tm *result);
enum color { RED } paint(void);
";
        let model = model("krep.h", source);
        assert_eq!(
            describe(&model),
            [
                "Whole name [1, 5] head [1, 5] doc [] body [[2, 3], [4]]",
                "Whole name [7, 10] head [7, 10] doc [] body [[8], [9]]",
                "Whole name [12] head [12] doc [] body [[13]]",
                "Whole name [14] head [14, 15] doc [] body []",
                "Whole name [16] head [16] doc [] body []",
            ]
        );
    }

    #[test]
    fn c_doc_is_the_comment_run_directly_above() {
        let source = "\
/* sds.h - dynamic strings */

typedef char *sds;

/* Create a new string.
 *
 * Returns NULL on failure. */
sds sdsnew(const char *init);
int trailing; /* end-of-line comment */
int next;
";
        let model = model("sds.h", source);
        assert!(model.module_doc.is_empty());
        let [sds, sdsnew, trailing, next] = &model.decls[..] else {
            panic!("{:?}", model.decls);
        };
        assert!(sds.doc.is_empty());
        assert_eq!(rows(&sdsnew.doc), vec![vec![5, 6, 7]]);
        assert_eq!(trailing.head, vec![9]);
        assert!(next.doc.is_empty());
    }

    #[test]
    fn c_doc_above_a_feature_gate_documents_its_first_declaration() {
        let source = "\
/* Banner. */
#ifndef CFG_H
#define CFG_H
/* Bytes per block. */
#ifndef BYTES_PER_BLOCK
#define BYTES_PER_BLOCK (16)
#endif
/* Open a channel. */
#ifdef HAVE_CH // channels
#if defined(FAST)
int ch_open(void);
#endif
int ch_close(void);
#endif
#endif
";
        let model = model("cfg.h", source);
        let docs: Vec<_> = model.decls.iter().map(|decl| rows(&decl.doc)).collect();
        assert_eq!(
            heads_of(&model),
            vec![vec![5, 6, 7], vec![9, 10, 11, 12], vec![13, 14]]
        );
        assert_eq!(docs, vec![vec![vec![4]], vec![vec![8]], vec![]]);
    }

    #[test]
    fn c_macros_list_all_but_the_guard_define() {
        let source = "\
#pragma once
#ifndef UTIL_H
#define UTIL_H
#define _GNU_SOURCE
#define HAVE_FEATURE
#define MAX(a, b) \\
    ((a) > (b) ? (a) : (b))
#define REGISTER(name, \\
                 handler) \\
    register_handler(name, \\
                     handler)
int util(void);
#endif
";
        let model = model("util.h", source);
        assert_eq!(
            name_rows_of(&model),
            vec![vec![4], vec![5], vec![6], vec![8], vec![12]]
        );
        assert_eq!(model.decls[1].shape, Shape::Whole);
        assert_eq!(model.decls[2].shape, Shape::Callable);
        assert_eq!(model.decls[2].head, vec![6]);
        assert_eq!(rows(&model.decls[2].body), vec![vec![7]]);
        assert_eq!(model.decls[3].head, vec![8, 9]);
        assert_eq!(rows(&model.decls[3].body), vec![vec![10], vec![11]]);
    }

    #[test]
    fn c_cpp_headers_are_left_to_the_fallback() {
        let cpp = "\
#pragma once
namespace lib {
// A widget.
class Widget {
 public:
  virtual int Method1(int x) = 0;
};
int FreeFn(const Widget& w);
}  // namespace lib
";
        assert!(is_cpp_header(Path::new("widget.h"), cpp));
        assert!(model("widget.h", cpp).decls.is_empty());
        assert!(!is_cpp_header(Path::new("widget.c"), cpp));
        for line in [
            "template <typename T>",
            "class LIB_EXPORT Snapshot {",
            "class Derived : public Base {",
            "namespace {",
            "  private:",
        ] {
            assert!(is_cpp_header(Path::new("x.h"), line), "{line}");
        }
        let c_with_cpp_wrappers = "\
#ifdef __cplusplus
extern \"C\" {
#endif
int api(void);
#if defined(__cplusplus) && defined(WRAP)
template <typename T> class wrapper {
#ifdef DEBUG
 public:
#endif
};
#endif
struct namespace_entry { int class_id; };
";
        assert!(!is_cpp_header(Path::new("api.h"), c_with_cpp_wrappers));
        let c_documenting_cpp_use = "\
/* From C++:

template <typename T>
class Widget {
public:
};
*/ int api(void); /* trailing
namespace lib {
*/
int other_api(void);
";
        assert!(!is_cpp_header(Path::new("api.h"), c_documenting_cpp_use));
    }

    #[test]
    fn c_file_weight_follows_the_header_convention() {
        let ctx = WalkCtx::new("/repo".into());
        assert_eq!(file_weight(Path::new("/repo/SDS.H"), &ctx), 1.0);
        assert_eq!(
            file_weight(Path::new("/repo/sds.c"), &ctx),
            IMPLEMENTATION_FILE_WEIGHT
        );
    }

    #[test]
    fn c_entrypoint_is_the_header_named_after_the_repository() {
        let ctx = WalkCtx::new("/work/sqlite-vec".into());
        let entrypoint =
            |relative: &str| is_entrypoint(&Path::new("/work/sqlite-vec").join(relative), &ctx);
        assert!(entrypoint("sqlite_vec.h"));
        assert!(entrypoint("src/SQLITE-VEC.h"));
        assert!(!entrypoint("sqlite-vec.c"));
        assert!(!entrypoint("include/deep/sqlite-vec.h"));
        assert!(entrypoint("src/include/sqlite-vec.h"));
        assert!(!entrypoint("src/lib/sqlite-vec.h"));
        assert!(!entrypoint("src/util.h"));
    }
}
