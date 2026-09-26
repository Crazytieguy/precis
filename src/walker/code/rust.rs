//! Rust extraction for the code engine.
//!
//! - **Module doc**: the file's leading `//!` / `/*! */` comments, one
//!   [`Item`] per paragraph.
//! - **Re-exports**: `pub use …;` and every `mod name;` declaration (the
//!   file's module tree, whatever its visibility). Other `use` and
//!   `extern crate` items are plumbing and not modeled. An inline
//!   `mod name {` row is a re-export too; the items in its body are
//!   modeled like the file's own when it has a visibility modifier.
//! - **Declarations**: `fn` is `Callable`, except the program flow of
//!   `main.rs` (see `show_program_flow`); `struct`, `enum`, `union`,
//!   `type`, `const`, `static`, `macro_rules!` and a macro invocation that
//!   declares items (see `item_macro`) are `Whole`; `trait`, `impl` and
//!   `extern` blocks are `Whole` containers whose members are their
//!   functions.
//! - **Hidden**: test (`#[cfg(test)]`, `#[cfg(all(test, …))]`,
//!   `#[test]`-style) and `#[doc(hidden)]` items, fields and variants,
//!   inline `mod test` / `mod tests`, and `const _`. Outside `main.rs`,
//!   a module that declares some unhidden visible item (a visibility
//!   modifier, or `#[macro_export]` on a `macro_rules!`) hides its
//!   private functions, macros and inherent-impl functions, and an
//!   inherent impl with no admitted function; its private types,
//!   constants and statics stay.

use std::collections::HashSet;
use std::path::Path;

use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{
    Language, MAX_SCOPE_NESTING, ProgramFunction, SourceFile, block_head, file_name,
    is_leading_trivia, named_children, show_program_flow,
};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["rs"],
    grammar: |_| tree_sitter_rust::LANGUAGE.into(),
    extract,
    is_entrypoint: Some(is_entrypoint),
    file_weight: None,
    sibling_mentions: None,
    sibling_names: None,
};

fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let is_program = file.path.file_name().is_some_and(|name| name == "main.rs");
    let mut functions = Vec::new();
    let mut model = FileModel {
        module_doc: module_doc(file, root),
        ..FileModel::default()
    };
    extract_items(root, file, &mut model, is_program.then_some(&mut functions));
    show_program_flow(&mut model.decls, &functions);
    model
}

/// The items of the file or of an inline module's body not yet modeled.
struct ModuleScope<'tree> {
    items: std::vec::IntoIter<(Node<'tree>, Leading)>,
    /// Whether the module's private helpers are hidden: it is not a
    /// program's top level, and some item says it is visible.
    hides_private: bool,
}

impl<'tree> ModuleScope<'tree> {
    fn new(scope: Node<'tree>, file: &SourceFile, is_program: bool) -> Self {
        // `Node::prev_sibling` rescans the parent's children on every call.
        let children: Vec<Node> = scope.children(&mut scope.walk()).collect();
        let items: Vec<(Node, Leading)> = (0..children.len())
            .filter(|&index| children[index].is_named() && !is_leading_trivia(children[index]))
            .map(|index| {
                let preceding = children[..index].iter().rev().copied();
                (children[index], Leading::walking_back(preceding, file))
            })
            .collect();
        let hides_private = !is_program
            && items.iter().any(|(node, leading)| {
                has_visibility_rule(*node) && !leading.hidden && is_visible(*node, leading, file)
            });
        Self {
            items: items.into_iter(),
            hides_private,
        }
    }
}

/// Models the items of the file and of its visible inline modules, in
/// source order, down to [`MAX_SCOPE_NESTING`] modules deep. The program
/// flow is collected from the file's top-level functions only.
fn extract_items(
    root: Node,
    file: &SourceFile,
    model: &mut FileModel,
    mut program_functions: Option<&mut Vec<ProgramFunction>>,
) {
    let mut scopes = vec![ModuleScope::new(root, file, program_functions.is_some())];
    while let Some(scope) = scopes.last_mut() {
        let Some((node, leading)) = scope.items.next() else {
            scopes.pop();
            continue;
        };
        if leading.hidden || is_anonymous_const(node, file) {
            continue;
        }
        let hides_private = scope.hides_private;
        let is_helper = matches!(node.kind(), "function_item" | "macro_definition");
        if hides_private && is_helper && !is_visible(node, &leading, file) {
            continue;
        }
        let at_top_level = scopes.len() == 1;
        match node.kind() {
            "use_declaration" | "mod_item" => match node.child_by_field_name("body") {
                None if node.kind() == "mod_item"
                    || visibility_modifier(node, file) == Some("pub") =>
                {
                    let mut rows = leading.attribute_rows;
                    rows.extend(file.node_rows(node));
                    model.reexports.push(Item::new(rows));
                }
                Some(body) if !is_test_module(node, file) => {
                    let mut rows = leading.attribute_rows;
                    rows.extend(node.start_position().row + 1..=body.start_position().row + 1);
                    model.reexports.push(Item::new(rows));
                    if visibility_modifier(node, file).is_some()
                        && scopes.len() <= MAX_SCOPE_NESTING
                    {
                        scopes.push(ModuleScope::new(body, file, false));
                    }
                }
                _ => {}
            },
            "impl_item" => {
                model
                    .decls
                    .extend(impl_container(node, leading, file, hides_private));
            }
            "trait_item" | "foreign_mod_item" => {
                model.decls.push(container(node, leading, file, |_| true));
            }
            "function_item" => {
                if let Some(functions) = program_functions.as_deref_mut().filter(|_| at_top_level) {
                    functions.push(ProgramFunction::new(model.decls.len(), node, file));
                }
                model.decls.push(callable(node, leading, file));
            }
            "macro_definition" => {
                let body = macro_open_row(node, file).map(|open_row| (open_row, node));
                let entries = file.admitted_items(node, 0, |child| child.kind() == "macro_rule");
                model
                    .decls
                    .push(whole(node, leading, file, body, entries, Vec::new()));
            }
            "struct_item" | "enum_item" | "union_item" | "type_item" | "const_item"
            | "static_item" => {
                let list = node.child_by_field_name("body");
                let body = list.map(|list| (list.start_position().row + 1, list));
                let entries = list.map_or_else(Vec::new, |list| {
                    file.admitted_items(list, 0, |entry| !Leading::above(entry, file).hidden)
                });
                model
                    .decls
                    .push(whole(node, leading, file, body, entries, Vec::new()));
            }
            "macro_invocation" | "expression_statement" => {
                model.decls.extend(item_macro(node, leading, file));
            }
            _ => {}
        }
    }
}

/// An item that is private to its module unless it says otherwise.
fn has_visibility_rule(node: Node) -> bool {
    matches!(
        node.kind(),
        "function_item"
            | "struct_item"
            | "enum_item"
            | "union_item"
            | "type_item"
            | "const_item"
            | "static_item"
            | "trait_item"
            | "macro_definition"
    )
}

/// A visibility modifier, or `#[macro_export]` on a `macro_rules!`.
fn is_visible(node: Node, leading: &Leading, file: &SourceFile) -> bool {
    visibility_modifier(node, file).is_some() || leading.exported
}

/// `const _: () = …;`, a compile-time check rather than a declaration.
fn is_anonymous_const(node: Node, file: &SourceFile) -> bool {
    node.kind() == "const_item"
        && node
            .child_by_field_name("name")
            .is_some_and(|name| file.text(name) == "_")
}

/// Keywords that, at the top level of a macro's token tree, mark the
/// invocation as one that declares items (`thread_local!`, `bitflags!`, a
/// `cfg_*!` wrapper around `pub use`s).
const DECLARATION_KEYWORDS: &[&str] = &[
    "pub", "struct", "enum", "static", "const", "fn", "type", "trait", "impl", "use", "mod",
];

/// An item-level macro invocation whose tokens declare items: `Whole`,
/// one body item per declaration (the tokens through a `;`, a `,` or a
/// `{ … }` group). Its name rows are the invocation's first row and each
/// declaration's first keyword row.
fn item_macro(node: Node, leading: Leading, file: &SourceFile) -> Option<DeclInfo> {
    let invocation = match node.kind() {
        "expression_statement" => node
            .named_child(0)
            .filter(|child| child.kind() == "macro_invocation")?,
        _ => node,
    };
    let tokens = invocation
        .named_children(&mut invocation.walk())
        .find(|child| child.kind() == "token_tree")?;
    let open_row = tokens.start_position().row + 1;
    let mut entries = Vec::new();
    let mut name_rows = vec![invocation.start_position().row + 1];
    let mut rows = Vec::new();
    let mut declares = false;
    let mut claimed_through = open_row;
    let inner_count = tokens.child_count().saturating_sub(2);
    for child in tokens
        .children(&mut tokens.walk())
        .skip(1)
        .take(inner_count)
    {
        let child_rows = file.node_rows(child);
        rows.extend((claimed_through + 1).max(*child_rows.start())..=*child_rows.end());
        claimed_through = claimed_through.max(*child_rows.end());
        if !declares && !child.is_named() && DECLARATION_KEYWORDS.contains(&child.kind()) {
            declares = true;
            name_rows.push(child.start_position().row + 1);
        }
        let ends_declaration = matches!(child.kind(), ";" | ",")
            || (child.kind() == "token_tree" && file.text(child).starts_with('{'));
        if ends_declaration && declares {
            entries.push(Item::new(std::mem::take(&mut rows)));
            declares = false;
        }
    }
    if name_rows.len() == 1 {
        return None;
    }
    name_rows.dedup();
    entries.push(Item::new(rows));
    let body = Some((open_row, tokens));
    let decl = whole(node, leading, file, body, entries, Vec::new());
    Some(DeclInfo { name_rows, ..decl })
}

/// A `mod test { … }` / `mod tests { … }`, test code whether or not it is
/// gated by `#[cfg(test)]`.
fn is_test_module(node: Node, file: &SourceFile) -> bool {
    node.child_by_field_name("name")
        .is_some_and(|name| matches!(file.text(name), "test" | "tests"))
}

fn is_entrypoint(path: &Path, _ctx: &WalkCtx) -> bool {
    file_name(path).is_some_and(|name| matches!(name, "lib.rs" | "main.rs"))
}

/// What the outer attributes and doc comments above an item say about it.
#[derive(Default)]
struct Leading {
    /// Rows of every outer attribute except `#[doc = …]`.
    attribute_rows: Vec<usize>,
    /// Rows of `///` / `/** */` comments and `#[doc = …]` attributes.
    doc_rows: Vec<usize>,
    /// `#[doc(hidden)]`, a test-only `#[cfg]` (see `is_test_cfg`), or a test
    /// attribute: `#[test]`, `#[tokio::test]`, `#[rstest]`, `#[test_case(…)]`.
    hidden: bool,
    /// `#[macro_export]`, with or without arguments.
    exported: bool,
}

impl Leading {
    /// Walks the attributes and comments directly preceding `node`. Plain
    /// comments, own-row or trailing an attribute, are skipped over (rustc
    /// ignores them); anything else ends the run.
    fn above(node: Node, file: &SourceFile) -> Self {
        Self::walking_back(
            std::iter::successors(node.prev_sibling(), Node::prev_sibling),
            file,
        )
    }

    /// [`Self::above`] over `preceding`, a node's earlier siblings,
    /// nearest first.
    fn walking_back<'tree>(
        preceding: impl Iterator<Item = Node<'tree>>,
        file: &SourceFile,
    ) -> Self {
        let mut leading = Leading::default();
        let mut preceding = preceding.peekable();
        while let Some(sibling) = preceding.next() {
            match sibling.kind() {
                "attribute_item" => {
                    let attribute = compact_attribute(sibling, file);
                    let path = attribute.split(['(', '=']).next().unwrap_or_default();
                    let last_segment = path.rsplit("::").next().unwrap_or_default();
                    let is_test =
                        last_segment.ends_with("test") || last_segment.starts_with("test_");
                    leading.hidden |=
                        is_test || is_test_cfg(&attribute) || attribute == "doc(hidden)";
                    leading.exported |= path == "macro_export";
                    if attribute.starts_with("doc=") {
                        leading.doc_rows.extend(file.node_rows(sibling));
                    } else {
                        leading.attribute_rows.extend(file.node_rows(sibling));
                    }
                }
                "line_comment" | "block_comment" => {
                    if !file.starts_own_row(sibling) {
                        let trails_attribute = preceding
                            .peek()
                            .is_some_and(|before| before.kind() == "attribute_item");
                        if !trails_attribute {
                            break;
                        }
                    }
                    if sibling.child_by_field_name("inner").is_some() {
                        break;
                    }
                    if sibling.child_by_field_name("outer").is_some() {
                        leading.doc_rows.extend(file.node_rows(sibling));
                    }
                }
                _ => break,
            }
        }
        leading
    }
}

/// `cfg(test)`, or `cfg(all(…))` with a bare `test` among its predicates.
fn is_test_cfg(compact_attribute: &str) -> bool {
    let Some(predicate) = compact_attribute
        .strip_prefix("cfg(")
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return false;
    };
    let Some(arguments) = predicate
        .strip_prefix("all(")
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return predicate == "test";
    };
    let mut depth = 0;
    let mut start = 0;
    let mut has_test = false;
    for (index, character) in arguments.char_indices().chain([(arguments.len(), ',')]) {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                has_test |= &arguments[start..index] == "test";
                start = index + 1;
            }
            _ => {}
        }
    }
    has_test
}

/// An attribute item's contents between `#[` and `]`, whitespace removed:
/// `cfg(test)`, `tokio::test`, `doc="…"`.
fn compact_attribute(attribute_item: Node, file: &SourceFile) -> String {
    attribute_item
        .named_children(&mut attribute_item.walk())
        .find(|child| child.kind() == "attribute")
        .map(|attribute| {
            file.text(attribute)
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect()
        })
        .unwrap_or_default()
}

/// `pub`, `pub(crate)`, …: the item's visibility modifier, if any.
fn visibility_modifier<'a>(node: Node, file: &'a SourceFile) -> Option<&'a str> {
    let modifier = node
        .children(&mut node.walk())
        .find(|child| child.kind() == "visibility_modifier")?;
    Some(file.text(modifier).trim())
}

fn name_rows(node: Node, field: &str) -> Vec<usize> {
    let anchor = node.child_by_field_name(field).unwrap_or(node);
    vec![anchor.start_position().row + 1]
}

fn doc_items(leading: &Leading, file: &SourceFile) -> Vec<Item> {
    let mut rows = leading.doc_rows.clone();
    rows.sort_unstable();
    rustdoc_paragraphs(rows, file)
}

/// A function: the head runs from its first attribute through the row
/// before its first statement; each statement is one body item.
fn callable(node: Node, leading: Leading, file: &SourceFile) -> DeclInfo {
    let name_rows = name_rows(node, "name");
    let floor = name_rows[0];
    let statements = named_children(node.child_by_field_name("body"));
    let mut decl = file.callable(name_rows, file.node_rows(node), statements, floor);
    decl.head.extend(&leading.attribute_rows);
    decl.doc = doc_items(&leading, file);
    decl
}

/// A declaration rendered whole. `body` is the row opening its entry list
/// and the node whose named children are the entries and members; the head
/// is the attributes, the rows through that opening row and the rows after
/// the last entry or member. Without a `body` the whole declaration is
/// head. `entries` are the body items besides the members' name rows.
fn whole(
    node: Node,
    leading: Leading,
    file: &SourceFile,
    body: Option<(usize, Node)>,
    entries: Vec<Item>,
    members: Vec<DeclInfo>,
) -> DeclInfo {
    let node_rows = file.node_rows(node);
    let name_rows = if node.kind() == "impl_item" {
        let mut rows = vec![*node_rows.start()];
        rows.extend(name_rows(node, "type"));
        rows.dedup();
        rows
    } else {
        name_rows(node, "name")
    };
    let mut head = match body {
        Some((open_row, list)) => {
            let last_content_row = list
                .named_children(&mut list.walk())
                .filter(|child| !matches!(child.kind(), "line_comment" | "block_comment"))
                .map(|child| *file.node_rows(child).end())
                .fold(open_row, usize::max);
            block_head(node_rows, open_row, last_content_row)
        }
        None => node_rows.collect(),
    };
    head.extend(&leading.attribute_rows);
    let mut body = entries;
    body.extend(
        members
            .iter()
            .map(|member| Item::new(member.name_rows.clone())),
    );
    body.sort_by_key(|item| item.rows.iter().min().copied());
    DeclInfo {
        doc: doc_items(&leading, file),
        body,
        members,
        ..DeclInfo::new(name_rows, head, Shape::Whole)
    }
}

/// A `trait` or `impl`: functions in its declaration list that `admit`
/// accepts become members; associated types and constants are body
/// entries.
fn container(
    node: Node,
    leading: Leading,
    file: &SourceFile,
    admit: impl Fn(Node) -> bool,
) -> DeclInfo {
    let Some(list) = node.child_by_field_name("body") else {
        return whole(node, leading, file, None, Vec::new(), Vec::new());
    };
    let is_function =
        |child: Node| matches!(child.kind(), "function_item" | "function_signature_item");
    let children: Vec<Node> = list.children(&mut list.walk()).collect();
    let mut members = Vec::new();
    let mut admitted_entries = HashSet::new();
    for (index, &child) in children.iter().enumerate() {
        if !child.is_named() || is_leading_trivia(child) {
            continue;
        }
        let child_leading = Leading::walking_back(children[..index].iter().rev().copied(), file);
        if child_leading.hidden || !admit(child) {
            continue;
        }
        if is_function(child) {
            members.push(callable(child, child_leading, file));
        } else {
            admitted_entries.insert(child.id());
        }
    }
    let entries = file.admitted_items(list, 0, |child| admitted_entries.contains(&child.id()));
    let open_row = list.start_position().row + 1;
    whole(
        node,
        leading,
        file,
        Some((open_row, list)),
        entries,
        members,
    )
}

/// A trait impl admits every function. An inherent impl in a module that
/// hides its private helpers admits only items with a visibility
/// modifier (`pub`, `pub(…)`), and is hidden without one.
fn impl_container(
    node: Node,
    leading: Leading,
    file: &SourceFile,
    hides_private: bool,
) -> Option<DeclInfo> {
    let is_trait_impl = node.child_by_field_name("trait").is_some();
    let admit =
        |child: Node| is_trait_impl || !hides_private || visibility_modifier(child, file).is_some();
    let decl = container(node, leading, file, admit);
    (is_trait_impl || !decl.body.is_empty()).then_some(decl)
}

/// The row of the delimiter opening a `macro_rules!` body.
fn macro_open_row(node: Node, file: &SourceFile) -> Option<usize> {
    let row = node
        .children(&mut node.walk())
        .find(|child| matches!(file.text(*child), "{" | "(" | "["))?
        .start_position()
        .row;
    Some(row + 1)
}

/// The leading `//!` / `/*! */` rows of the file (license comments and
/// `#![…]` attributes around them skipped), as paragraphs.
fn module_doc(file: &SourceFile, root: Node) -> Vec<Item> {
    let mut rows = Vec::new();
    for child in root.named_children(&mut root.walk()) {
        match child.kind() {
            "line_comment" | "block_comment" => {
                if child.child_by_field_name("inner").is_some() {
                    rows.extend(file.node_rows(child));
                }
            }
            "inner_attribute_item" => {}
            _ => break,
        }
    }
    rustdoc_paragraphs(rows, file)
}

/// Sorted doc rows split into paragraph items at rows holding no
/// markdown, only comment markers.
fn rustdoc_paragraphs(rows: Vec<usize>, file: &SourceFile) -> Vec<Item> {
    file.paragraphs_by(rows, |line| {
        let line = line.trim();
        let line = line.strip_suffix("*/").unwrap_or(line);
        ["///", "//!", "/**", "/*!", "*"]
            .iter()
            .find_map(|marker| line.strip_prefix(marker))
            .unwrap_or(line)
            .trim()
            .is_empty()
    })
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{describe, rows};
    use super::*;

    fn extract_source(file_name: &str, source: &str) -> (SourceFile, FileModel) {
        super::super::test_support::extract_in(&LANGUAGE, &[(file_name, source)], file_name)
    }

    /// Each declaration's name row text, in source order.
    fn roster(file: &SourceFile, decls: &[DeclInfo]) -> Vec<String> {
        let mut out: Vec<_> = decls
            .iter()
            .map(|decl| {
                (
                    decl.name_rows[0],
                    file.line(decl.name_rows[0]).trim().to_string(),
                )
            })
            .collect();
        out.sort();
        out.into_iter().map(|(_, line)| line).collect()
    }

    #[test]
    fn rust_extract_module_doc_skips_plain_comments_and_splits_paragraphs() {
        let source = "\
// Copyright header, not documentation.

//! [![docs]](https://docs.rs/x)&ensp;[![ci]](https://ci)
//!
//! [docs]: https://img.shields.io/badge/docs
//! <br>
//!
//! A crate that does one thing.
//! It does it well.
//!
//! <br>
//!
//! # Example
//! ```
//! # fn setup() {}
//! let x = 1;
//! ```
#![no_std]

pub fn f() {}
";
        let (_, model) = extract_source("lib.rs", source);
        assert_eq!(
            rows(&model.module_doc),
            vec![
                vec![3, 4],
                vec![5, 6, 7],
                vec![8, 9, 10],
                vec![11, 12],
                vec![13, 14, 15, 16, 17]
            ]
        );
    }

    #[test]
    fn rust_extract_hides_tests_and_doc_hidden() {
        let source = "\
pub struct Public;
pub(crate) struct Crate;
struct Private;
#[doc(hidden)]
pub struct Hidden;
#[cfg(test)]
fn helper() {}
#[test]
fn check() {}
#[tokio::test]
async fn check_async() {}
#[cfg(test)]
mod tests {
    #[test]
    fn inner() {}
}
pub fn latest() {}
";
        let (file, model) = extract_source("a.rs", source);
        assert_eq!(
            roster(&file, &model.decls),
            vec![
                "pub struct Public;",
                "pub(crate) struct Crate;",
                "struct Private;",
                "pub fn latest() {}",
            ]
        );
    }

    #[test]
    fn rust_extract_private_helpers_hide_behind_a_visible_item() {
        let source = "\
static STATE: AtomicUsize = AtomicUsize::new(0);
const _: () = assert!(size_of::<u8>() == 1);
macro_rules! debug {
    ($($t:tt)*) => {};
}
fn new_regex(pattern: &str) -> Regex {
    todo!()
}
pub fn escape(text: &str) -> String {
    todo!()
}
";
        let (file, model) = extract_source("glob.rs", source);
        assert_eq!(
            roster(&file, &model.decls),
            vec![
                "static STATE: AtomicUsize = AtomicUsize::new(0);",
                "pub fn escape(text: &str) -> String {",
            ]
        );
        let private_only = "fn a() {}\nfn b() {}\n";
        let (_, model) = extract_source("util.rs", private_only);
        assert_eq!(model.decls.len(), 2);
        let test_helper_only = "\
static STATE: u8 = 0;
fn a() {}
#[cfg(test)]
pub fn helper() {}
";
        let (file, model) = extract_source("util.rs", test_helper_only);
        assert_eq!(
            roster(&file, &model.decls),
            vec!["static STATE: u8 = 0;", "fn a() {}"]
        );
        let exported_with_arguments = "\
#[macro_export(local_inner_macros)]
macro_rules! bail {
    () => {};
}
pub fn escape() {}
";
        let (file, model) = extract_source("glob.rs", exported_with_arguments);
        assert_eq!(
            roster(&file, &model.decls),
            vec!["macro_rules! bail {", "pub fn escape() {}"]
        );
    }

    #[test]
    fn rust_extract_hides_test_only_cfgs_test_macros_and_hidden_variants() {
        let source = "\
#[cfg(all(test, feature = \"x\"))]
pub fn only_in_tests() {}
#[cfg(any(test, feature = \"alloc\"))]
pub fn parse() {}
#[cfg(not(test))]
pub fn production() {}
#[cfg(all(unix, any(feature = \"std\", test)))]
pub fn on_unix() {}
#[test_case::test_case(1)]
pub fn case(value: u8) {}
#[rstest]
pub fn fixture() {}
pub enum Kind {
    A,
    #[doc(hidden)]
    _Custom(String),
}
";
        let (file, model) = extract_source("a.rs", source);
        assert_eq!(
            roster(&file, &model.decls),
            vec![
                "pub fn parse() {}",
                "pub fn production() {}",
                "pub fn on_unix() {}",
                "pub enum Kind {"
            ]
        );
        assert_eq!(rows(&model.decls[3].body), vec![vec![14]]);
    }

    #[test]
    fn rust_extract_callable_splits_attributes_signature_and_statements() {
        let source = "\
/// Runs it.
#[inline]
pub fn run(
    a: u8,
) -> u8 {
    // leading comment
    let b = a;
    b
}

pub fn one_line() -> u8 { 1 }
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            [
                "Callable name [3] head [2, 3, 4, 5] doc [[1]] body [[6, 7], [8]]",
                "Callable name [11] head [11] doc [] body []",
            ]
        );
    }

    #[test]
    fn rust_extract_whole_types_keep_open_and_close_rows_in_head() {
        let source = "\
#[derive(Debug)]
pub enum Kind {
    /// First.
    A,
    #[default]
    B(u8),
}
pub struct Unit;
pub const LIMIT: usize = 3;
pub struct Point { pub x: i32 }
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            [
                "Whole name [2] head [1, 2, 7] doc [] body [[3, 4], [5, 6]]",
                "Whole name [8] head [8] doc [] body []",
                "Whole name [9] head [9] doc [] body []",
                "Whole name [10] head [10] doc [] body []",
            ]
        );
    }

    #[test]
    fn rust_extract_trait_functions_are_members() {
        let source = "\
pub trait Store {
    type Key;

    /// Loads.
    fn load(&self, key: Self::Key) -> u8;

    fn exists(&self) -> bool {
        true
    }
}
pub(crate) trait Internal {
    fn go(&self);
}
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1, 10] doc [] body [[2], [5], [7]]",
                "  Callable name [5] head [5] doc [[4]] body []",
                "  Callable name [7] head [7] doc [] body [[8]]",
                "Whole name [11] head [11, 13] doc [] body [[12]]",
                "  Callable name [12] head [12] doc [] body []",
            ]
        );
    }

    #[test]
    fn rust_extract_extern_block_functions_are_members() {
        let source = "\
pub struct Ctx;
extern \"C\" {
    /// Initialize.
    pub fn lib_init(ctx: *mut Ctx) -> i32;
    static VERSION: u32;
}
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body []",
                "Whole name [2] head [2, 6] doc [] body [[4], [5]]",
                "  Callable name [4] head [4] doc [[3]] body []",
            ]
        );
    }

    #[test]
    fn rust_extract_trailing_comment_on_a_rejected_item_stays_hidden() {
        let source = "\
pub struct Foo;
impl Foo {
    fn helper(&self) -> u32 { 1 } // fast path
    pub const LIMIT: u32 = 8;
}
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body []",
                "Whole name [2] head [2, 5] doc [] body [[4]]",
            ]
        );
    }

    #[test]
    fn rust_extract_comment_trailing_an_attribute_keeps_the_leading_run() {
        let source = "\
/// The type.
#[derive(Copy, Clone)]
#[allow(dead_code)] // trailing note
pub enum Opt<T> { None, Some(T) }
#[cfg(test)] // used in tests
impl Opt<u8> {
    pub fn check(&self) {}
}
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            ["Whole name [4] head [2, 3, 4] doc [[1]] body []"]
        );
    }

    #[test]
    fn rust_extract_impl_members_follow_modifiers() {
        let source = "\
pub struct Engine;
struct Helper;
impl Engine {
    pub fn start(&self) {}
    pub(crate) fn tune(&self) {}
    fn internal(&self) {}
}
impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
impl Helper {
    pub fn assist(&self) {}
}
impl Engine {
    fn only_private(&self) {}
}
#[doc(hidden)]
pub trait Sealed {}
impl Sealed for Engine {}
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body []",
                "Whole name [2] head [2] doc [] body []",
                "Whole name [3] head [3, 7] doc [] body [[4], [5]]",
                "  Callable name [4] head [4] doc [] body []",
                "  Callable name [5] head [5] doc [] body []",
                "Whole name [8] head [8, 12] doc [] body [[9]]",
                "  Callable name [9] head [9] doc [] body [[10]]",
                "Whole name [13] head [13, 15] doc [] body [[14]]",
                "  Callable name [14] head [14] doc [] body []",
                "Whole name [21] head [21] doc [] body []",
            ]
        );
        let wrapped = "\
impl<F> ParallelVisitorBuilder
    for FnBuilder<F>
{
    fn build(&mut self) {}
}
";
        let (_, model) = extract_source("walk.rs", wrapped);
        assert_eq!(model.decls[0].name_rows, vec![1, 2]);
        let private_module = "\
struct ServerImpl;
impl ServerImpl {
    fn handle(&mut self) {}
}
";
        let (_, model) = extract_source("main.rs", private_module);
        assert_eq!(
            describe(&model),
            [
                "Whole name [1] head [1] doc [] body []",
                "Whole name [2] head [2, 4] doc [] body [[3]]",
                "  Callable name [3] head [3] doc [] body []",
            ]
        );
    }

    #[test]
    fn rust_extract_reexports_are_pub_use_and_mod_declarations() {
        let source = "\
use std::io;
mod private;
pub(crate) use self::private::Thing;
#[cfg(feature = \"x\")]
pub mod api;
pub use crate::{
    a::A,
    b::B,
};
extern crate alloc;
";
        let (_, model) = extract_source("lib.rs", source);
        assert_eq!(
            rows(&model.reexports),
            vec![vec![2], vec![4, 5], vec![6, 7, 8, 9]]
        );
        assert!(model.decls.is_empty());
    }

    #[test]
    fn rust_extract_inline_module_items_are_modeled_like_top_level_ones() {
        let source = "\
#[cfg(feature = \"x\")]
pub mod inner {
    /// Inner API.
    pub fn api() -> u32 { 1 }
    pub(crate) mod nested {
        pub struct Deep;
    }
    mod private {
        pub fn helper() {}
    }
}
pub fn outer() -> u32 { let a = 1; a }
pub(crate) mod test {
    pub fn fixture() {}
}
#[cfg(test)]
mod checks {
    fn check() {}
}
";
        let (file, model) = extract_source("lib.rs", source);
        assert_eq!(rows(&model.reexports), vec![vec![1, 2], vec![5], vec![8]]);
        assert_eq!(
            roster(&file, &model.decls),
            vec![
                "pub fn api() -> u32 { 1 }",
                "pub struct Deep;",
                "pub fn outer() -> u32 { let a = 1; a }",
            ]
        );
        assert_eq!(rows(&model.decls[0].doc), vec![vec![3]]);
    }

    #[test]
    fn rust_extract_macros_are_whole() {
        let source = "\
/// Bails.
#[macro_export]
macro_rules! bail {
    ($msg:literal) => {
        return Err($msg)
    };
    ($err:expr) => {
        return Err($err)
    };
}
macro_rules! local {
    () => {};
}
#[doc(hidden)]
#[macro_export]
macro_rules! __private {
    () => {};
}
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            ["Whole name [3] head [2, 3, 10] doc [[1]] body [[4, 5, 6], [7, 8, 9]]"]
        );
    }

    #[test]
    fn rust_extract_entrypoints_are_lib_and_main() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        assert!(is_entrypoint(Path::new("src/lib.rs"), &ctx));
        assert!(is_entrypoint(Path::new("src/main.rs"), &ctx));
        assert!(!is_entrypoint(Path::new("src/mod.rs"), &ctx));
    }

    #[test]
    fn rust_extract_thin_main_shows_the_flow_it_delegates_to() {
        let source = "\
fn run() -> Result<()> {
    parse()?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        exit(error);
    }
}
";
        let shapes = |file_name| {
            let (_, model) = extract_source(file_name, source);
            let decls = model.decls.iter();
            decls
                .map(|decl| (decl.shape, decl.head.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            shapes("main.rs"),
            [(Shape::Whole, vec![1, 4]), (Shape::Whole, vec![6, 10])]
        );
        assert_eq!(
            shapes("run.rs"),
            [(Shape::Callable, vec![1]), (Shape::Callable, vec![6])]
        );
    }

    #[test]
    fn rust_extract_item_macros_are_whole_declarations() {
        let source = "\
/// Buffers.
thread_local! {
    /// Per-thread buffer.
    pub static BUF: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}
task_slot!(SYS, sys);
cfg_client! {
    pub use kube_client::api;
    #[doc(inline)] pub use api::Api;
}
bitflags! {
    pub struct Modifiers: u16 {
        const BOLD = 1;
    }
}
thread_local!(static DEPTH: Cell<u8> = Cell::new(0));
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            describe(&model),
            [
                "Whole name [2, 4] head [2, 5] doc [[1]] body [[3, 4]]",
                "Whole name [7, 8, 9] head [7, 10] doc [] body [[8], [9]]",
                "Whole name [11, 12] head [11, 15] doc [] body [[12, 13, 14]]",
                "Whole name [16] head [16] doc [] body []",
            ]
        );
    }
}
