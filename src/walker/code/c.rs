//! C extraction for the code engine.
//!
//! The declarations of a file are its "effective top level": the body of
//! a wrapping `#ifndef X` / `#define X` / `#endif` header guard, the body
//! of `extern "C" { … }` (bare or `#ifdef __cplusplus`-wrapped), and
//! `#if` / `#ifdef` blocks holding only declarations and directives.
//! Other conditional blocks stay opaque.
//!
//! - Function definitions are `Callable`; prototypes, typedefs, structs /
//!   unions / enums, global variables and macros are `Whole`, with one
//!   body [`Item`] per field or enumerator.
//! - A non-`inline` `static` in a header is hidden.
//! - A declaration's doc is the comment run directly above it.

use std::ops::RangeInclusive;
use std::path::Path;

use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{Language, SourceFile, is_named_after};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["c", "h"],
    grammar: |_| tree_sitter_c::LANGUAGE.into(),
    extract,
    is_entrypoint: Some(is_entrypoint),
    file_weight: Some(file_weight),
};

/// Every batch of a `.c` file, relative to a header. Not tuned.
const IMPLEMENTATION_FILE_WEIGHT: f64 = 0.6;

fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let source = &*file.source;
    let in_header = is_header(&file.path);
    let guard_name = header_guard_name(root, source);
    let mut decls = Vec::new();
    let mut directives = Vec::new();
    walk_top_level(root, source, &mut |node| {
        if node.kind().starts_with('#') {
            directives.extend(gate_directive(node, file));
        } else {
            decls.extend(declaration(node, file, in_header, guard_name));
        }
    });
    attach_directives(&mut decls, &directives);
    FileModel {
        module_doc: Vec::new(),
        reexports: Vec::new(),
        decls,
    }
}

/// A header within one directory of the root named after the repository
/// (`sds.h`, `src/jq.h`): the library's public interface.
fn is_entrypoint(path: &Path, ctx: &WalkCtx) -> bool {
    is_header(path) && ctx.depth_from_root(path) <= 2 && is_named_after(path, ctx.root())
}

fn file_weight(path: &Path, _ctx: &WalkCtx) -> f64 {
    if is_header(path) {
        1.0
    } else {
        IMPLEMENTATION_FILE_WEIGHT
    }
}

fn is_header(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("h"))
}

fn declaration(
    node: Node,
    file: &SourceFile,
    in_header: bool,
    guard_name: Option<&str>,
) -> Option<DeclInfo> {
    let source = &*file.source;
    let shape = match node.kind() {
        "function_definition" => Shape::Callable,
        "declaration"
        | "type_definition"
        | "struct_specifier"
        | "union_specifier"
        | "enum_specifier"
        | "preproc_function_def" => Shape::Whole,
        "preproc_def" if !is_header_guard_define(node, source, guard_name) => Shape::Whole,
        _ => return None,
    };
    // A header's `static inline` definition is the header-only accessor
    // idiom: part of the API. Any other `static` in a header is an
    // implementation leak.
    if in_header
        && has_storage_class(node, source, "static")
        && !has_storage_class(node, source, "inline")
    {
        return None;
    }
    let name_rows = name_rows(node);
    let (head, body) = match shape {
        Shape::Callable => callable_parts(node, file),
        Shape::Whole => whole_parts(node, file),
    };
    let doc_rows = file.comment_rows_above(node);
    Some(DeclInfo {
        name_rows,
        head,
        doc: file.paragraphs(doc_rows),
        body,
        shape,
        members: Vec::new(),
    })
}

/// A directive of a descended feature gate: its rows (through the end
/// of a multiline condition) and the rows of the whole gate.
struct GateDirective {
    rows: RangeInclusive<usize>,
    gate: RangeInclusive<usize>,
}

/// `token` is an `#if` / `#ifdef` / `#elif` / `#else` / `#endif` token
/// of a feature gate.
fn gate_directive(token: Node, file: &SourceFile) -> Option<GateDirective> {
    let branch = token.parent()?;
    let mut gate = branch;
    while matches!(
        gate.kind(),
        "preproc_else" | "preproc_elif" | "preproc_elifdef"
    ) {
        gate = gate.parent()?;
    }
    let start_row = token.start_position().row + 1;
    let end_row = if token.kind() == "#endif" {
        start_row
    } else {
        branch
            .child_by_field_name("condition")
            .or_else(|| branch.child_by_field_name("name"))
            .map_or(start_row, |condition| condition.end_position().row + 1)
    };
    Some(GateDirective {
        rows: start_row..=end_row,
        gate: file.node_rows(gate),
    })
}

/// Adds each feature gate directive to the head of the nearest
/// declaration inside its gate: the next one, or, for an `#endif` or a
/// directive opening an empty trailing branch, the previous one. An
/// enclosing gate's directives land on the same declarations as the
/// nested gate's, so a gated declaration never renders without any of
/// its conditions.
fn attach_directives(decls: &mut [DeclInfo], directives: &[GateDirective]) {
    let spans: Vec<(usize, usize)> = decls
        .iter()
        .map(|decl| {
            let doc = decl.doc.iter().flat_map(|item| item.rows.iter());
            let body = decl.body.iter().flat_map(|item| item.rows.iter());
            let rows = doc.chain(&decl.head).chain(body).copied();
            (rows.clone().min().unwrap_or(0), rows.max().unwrap_or(0))
        })
        .collect();
    for directive in directives {
        let row = *directive.rows.start();
        let in_gate = || {
            spans
                .iter()
                .enumerate()
                .filter(|(_, (first, _))| directive.gate.contains(first))
        };
        let target = in_gate()
            .filter(|(_, (first, _))| *first > row)
            .min_by_key(|(_, (first, _))| *first)
            .or_else(|| {
                in_gate()
                    .filter(|(_, (_, last))| *last < row)
                    .max_by_key(|(_, (_, last))| *last)
            });
        if let Some((index, _)) = target {
            decls[index].head.extend(directive.rows.clone());
        }
    }
}

/// The declaration's first row, plus the row of each name it declares:
/// the `} Name;` row of a `typedef struct { … } Name;`, the `name(` row of
/// a definition whose return type sits on the row above.
fn name_rows(node: Node) -> Vec<usize> {
    let mut rows = vec![node.start_position().row + 1];
    let mut cursor = node.walk();
    rows.extend(
        node.children_by_field_name("declarator", &mut cursor)
            .filter_map(declared_name)
            .map(|name| name.start_position().row + 1),
    );
    rows.sort_unstable();
    rows.dedup();
    rows
}

fn declared_name(declarator: Node) -> Option<Node> {
    match declarator.kind() {
        "identifier" | "type_identifier" | "field_identifier" => Some(declarator),
        _ => declarator
            .child_by_field_name("declarator")
            .or_else(|| declarator.named_child(0))
            .and_then(declared_name),
    }
}

/// A function definition's head (through the row before its first
/// statement) and its statements. A statement sharing the row of the
/// body's opening `{` is head.
fn callable_parts(node: Node, file: &SourceFile) -> (Vec<usize>, Vec<Item>) {
    let rows = file.node_rows(node);
    let body = node
        .child_by_field_name("body")
        .filter(|body| {
            let mut cursor = body.walk();
            body.named_children(&mut cursor)
                .any(|child| child.kind() != "comment")
        })
        .map(|body| {
            let mut cursor = body.walk();
            file.node_items(
                body.named_children(&mut cursor),
                body.start_position().row + 1,
            )
        })
        .unwrap_or_default();
    let head_end = body.first().map_or(*rows.end(), |first| first.rows[0] - 1);
    ((*rows.start()..=head_end).collect(), body)
}

/// A `Whole` declaration's head and body. With a struct / union / enum
/// body, the head is the rows through the opening `{` plus the closing
/// row and anything after it (`} Name;`), and each field or enumerator is
/// one body item. Anything else is all head.
fn whole_parts(node: Node, file: &SourceFile) -> (Vec<usize>, Vec<Item>) {
    let rows = file.node_rows(node);
    let Some(body_node) = aggregate_body(node) else {
        return (rows.collect(), Vec::new());
    };
    let open_row = body_node.start_position().row + 1;
    let close_row = body_node.end_position().row + 1;
    let mut cursor = body_node.walk();
    let body = file.node_items(body_node.named_children(&mut cursor), open_row);
    let claimed_through = body
        .last()
        .and_then(|item| item.rows.last().copied())
        .unwrap_or(open_row);
    let mut head: Vec<usize> = (*rows.start()..=open_row).collect();
    head.extend((close_row..=*rows.end()).filter(|row| *row > claimed_through));
    (head, body)
}

/// The struct / union / enum body a declaration defines, unless the
/// declaration is a function prototype (`struct point make(void);`).
fn aggregate_body(node: Node) -> Option<Node> {
    let specifier = match node.kind() {
        "struct_specifier" | "union_specifier" | "enum_specifier" => node,
        "declaration" | "type_definition" => {
            let mut cursor = node.walk();
            if node
                .children_by_field_name("declarator", &mut cursor)
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

fn declarator_is_function(node: Node) -> bool {
    match node.kind() {
        "function_declarator" => true,
        "pointer_declarator" | "parenthesized_declarator" | "init_declarator" => node
            .child_by_field_name("declarator")
            .or_else(|| node.named_child(0))
            .is_some_and(declarator_is_function),
        _ => false,
    }
}

/// Visit each "effective top-level" item: descends through the file's
/// header guard, through `extern "C" { … }` linkage specs and through
/// declaration-only `#if` / `#ifdef` blocks.
fn walk_top_level<'a, F: FnMut(Node<'a>)>(root: Node<'a>, source: &str, visit: &mut F) {
    let header_guard_body = header_guard_body_node(root, source);
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if Some(child) == header_guard_body {
            descend_envelopes(child, source, visit);
        } else {
            visit_with_envelope_descent(child, source, visit);
        }
    }
}

fn descend_envelopes<'a, F: FnMut(Node<'a>)>(node: Node<'a>, source: &str, visit: &mut F) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        visit_with_envelope_descent(child, source, visit);
    }
}

/// Visit `node`, descending if it's an `extern "C" { … }` envelope (raw
/// or `#ifdef __cplusplus`-wrapped) or a declaration-only feature gate.
fn visit_with_envelope_descent<'a, F: FnMut(Node<'a>)>(
    node: Node<'a>,
    source: &str,
    visit: &mut F,
) {
    if let Some(decl_list) = extern_c_declaration_list(node, source) {
        descend_envelopes(decl_list, source, visit);
        return;
    }
    if matches!(node.kind(), "preproc_if" | "preproc_ifdef")
        && !is_disabled_preproc_if(node, source)
        && feature_gate_is_declaration_only(node) == Some(true)
    {
        descend_feature_gate_branches(node, source, visit);
        return;
    }
    visit(node);
}

/// Visit every branch of a declaration-only feature gate: its children,
/// directive tokens (`#ifdef`, `#else`, `#endif`) included, plus the
/// bodies of `#else` / `#elif` alternates. Nested gates descend (or stay
/// opaque) on their own merits.
fn descend_feature_gate_branches<'a, F: FnMut(Node<'a>)>(
    node: Node<'a>,
    source: &str,
    visit: &mut F,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "preproc_else" | "preproc_elif" | "preproc_elifdef" => {
                descend_feature_gate_branches(child, source, visit);
            }
            _ => visit_with_envelope_descent(child, source, visit),
        }
    }
}

/// True for `#if 0` blocks: commented-out code, not a feature gate.
fn is_disabled_preproc_if(node: Node, source: &str) -> bool {
    node.child_by_field_name("condition")
        .is_some_and(|condition| source[condition.byte_range()].trim() == "0")
}

/// For a `preproc_if*` / `preproc_else*` subtree: `None` when any branch
/// wraps code (a function body, a statement), else whether it holds at
/// least one declaration beside its directives and comments.
fn feature_gate_is_declaration_only(node: Node) -> Option<bool> {
    let mut declaration_found = false;
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "declaration" | "type_definition" | "struct_specifier" | "union_specifier"
            | "enum_specifier" => declaration_found = true,
            "preproc_include"
            | "preproc_def"
            | "preproc_function_def"
            | "preproc_call"
            | "comment" => {}
            "preproc_if" | "preproc_ifdef" | "preproc_else" | "preproc_elif"
            | "preproc_elifdef" => declaration_found |= feature_gate_is_declaration_only(child)?,
            // Condition / name tokens of the `#if` / `#ifdef` itself.
            "identifier"
            | "binary_expression"
            | "parenthesized_expression"
            | "unary_expression"
            | "call_expression"
            | "preproc_defined"
            | "number_literal"
            | "char_literal"
            | "string_literal" => {}
            _ => return None,
        }
    }
    Some(declaration_found)
}

/// If `node` is an `extern "C" { … }` envelope (raw or
/// `#ifdef __cplusplus`-wrapped), the inner `declaration_list`. Any other
/// `#ifdef` around a linkage spec is a feature gate and stays opaque.
fn extern_c_declaration_list<'a>(node: Node<'a>, source: &str) -> Option<Node<'a>> {
    let linkage = match node.kind() {
        "linkage_specification" => node,
        "preproc_ifdef" => cplusplus_wrapped_linkage_specification(node, source)?,
        _ => return None,
    };
    let mut cursor = linkage.walk();
    linkage
        .children(&mut cursor)
        .find(|child| child.kind() == "declaration_list")
}

/// If `ifdef` is `#ifdef __cplusplus` / linkage spec / `#endif`, the
/// linkage spec.
fn cplusplus_wrapped_linkage_specification<'a>(ifdef: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cursor = ifdef.walk();
    let mut children = ifdef.children(&mut cursor);
    if children.next()?.kind() != "#ifdef" {
        return None;
    }
    let name = children.next()?;
    if name.kind() != "identifier" || &source[name.byte_range()] != "__cplusplus" {
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

/// The `#ifndef X` / `#define X` / `#endif` block wrapping the whole file
/// (comments aside), whose children are the effective top level.
fn header_guard_body_node<'a>(root: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cursor = root.walk();
    let mut candidate = None;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "comment" => {}
            "preproc_ifdef" if candidate.is_none() && is_header_guard(child, source) => {
                candidate = Some(child);
            }
            _ => return None,
        }
    }
    candidate
}

fn header_guard_name<'a>(root: Node, source: &'a str) -> Option<&'a str> {
    let guard = header_guard_body_node(root, source)?;
    let name = guard.child_by_field_name("name")?;
    Some(&source[name.byte_range()])
}

/// True iff `ifdef` is `#ifndef X` whose first child (comments aside)
/// is `#define X`.
fn is_header_guard(ifdef: Node, source: &str) -> bool {
    let Some(name) = ifdef.child_by_field_name("name") else {
        return false;
    };
    if ifdef.child(0).is_none_or(|token| token.kind() != "#ifndef") {
        return false;
    }
    let mut cursor = ifdef.walk();
    let first = ifdef
        .named_children(&mut cursor)
        .find(|child| *child != name && child.kind() != "comment");
    first.is_some_and(|define| {
        define.kind() == "preproc_def"
            && define
                .child_by_field_name("name")
                .is_some_and(|defined| source[defined.byte_range()] == source[name.byte_range()])
    })
}

/// True iff a valueless `#define X` is header-guard envelope rather than
/// a macro: `X` is the file's guard symbol, or, for guards not
/// recognized as wrapping the file, an all-caps name.
fn is_header_guard_define(node: Node, source: &str, guard_name: Option<&str>) -> bool {
    if node.child_by_field_name("value").is_some() {
        return false;
    }
    let Some(name) = node.child_by_field_name("name") else {
        return false;
    };
    let name = &source[name.byte_range()];
    !name.is_empty()
        && (guard_name == Some(name)
            || name
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
}

fn has_storage_class(node: Node, source: &str, keyword: &str) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|child| {
        child.kind() == "storage_class_specifier" && source[child.byte_range()].trim() == keyword
    })
}

#[cfg(test)]
mod tests {
    use super::super::test_support::rows;
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
    fn c_feature_gates_descend_only_when_declaration_only() {
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
";
        for path in ["krep.h", "krep.c"] {
            assert_eq!(
                name_rows_of(&model(path, source)),
                vec![vec![2], vec![4], vec![19]]
            );
        }
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
        let heads: Vec<Vec<usize>> = model("globals.h", source)
            .decls
            .iter()
            .map(|decl| {
                let mut head = decl.head.clone();
                head.sort_unstable();
                head
            })
            .collect();
        assert_eq!(
            heads,
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
";
        let listed = |file_name| {
            model(file_name, source)
                .decls
                .iter()
                .map(|decl| decl.name_rows[0])
                .collect::<Vec<_>>()
        };
        assert_eq!(listed("sds.h"), vec![1, 4]);
        assert_eq!(listed("sds.c"), vec![1, 2, 3, 4]);
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
";
        let model = model("sds.c", source);
        let [sdsnewlen, one_liner, empty, add] = &model.decls[..] else {
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
    }

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
";
        let model = model("krep.h", source);
        let [params, kind, node] = &model.decls[..] else {
            panic!("{:?}", model.decls);
        };
        assert_eq!(params.name_rows, vec![1, 5]);
        assert_eq!(params.head, vec![1, 5]);
        assert_eq!(rows(&params.body), vec![vec![2, 3], vec![4]]);
        assert_eq!(kind.name_rows, vec![7, 10]);
        assert_eq!(kind.head, vec![7, 10]);
        assert_eq!(rows(&kind.body), vec![vec![8], vec![9]]);
        assert_eq!(node.head, vec![12]);
        assert_eq!(rows(&node.body), vec![vec![13]]);
    }

    #[test]
    fn c_prototypes_returning_structs_are_all_head() {
        let source = "\
struct tm *gmtime_r(const time_t *timep,
                    struct tm *result);
enum color { RED } paint(void);
";
        let model = model("time.h", source);
        let heads: Vec<_> = model.decls.iter().map(|decl| decl.head.clone()).collect();
        assert_eq!(heads, vec![vec![1, 2], vec![3]]);
        assert!(
            model
                .decls
                .iter()
                .all(|decl| decl.body.is_empty() && decl.shape == Shape::Whole)
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
    fn c_macros_are_whole_and_guard_like_defines_are_skipped() {
        let source = "\
#define _POSIX_C_SOURCE 200809L
#define HAVE_FEATURE
#define Foo_Debug
#define MAX(a, b) \\
    ((a) > (b) ? (a) : (b))
";
        let model = model("util.h", source);
        let heads: Vec<_> = model.decls.iter().map(|decl| decl.head.clone()).collect();
        assert_eq!(heads, vec![vec![1], vec![3], vec![4, 5]]);
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
        assert!(!entrypoint("src/util.h"));
    }
}
