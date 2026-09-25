//! Rust extraction for the code engine.
//!
//! - **Module doc**: the file's leading `//!` / `/*! */` comments, one
//!   [`Item`] per paragraph.
//! - **Re-exports**: `pub use …;` and every `mod name;` declaration (the
//!   file's module tree, whatever its visibility). Other `use` and
//!   `extern crate` items are plumbing and not modeled; an inline
//!   `mod name { … }` is left out with its contents.
//! - **Declarations**: `fn` is `Callable`, except the program flow of
//!   `main.rs` (see `show_program_flow`); `struct`, `enum`, `union`,
//!   `type`, `const`, `static` and `macro_rules!` are `Whole`; `trait` and
//!   `impl` are `Whole` containers whose members are their functions.
//! - **Hidden**: `#[cfg(test)]`, `#[test]`-style and `#[doc(hidden)]`
//!   items, and an inherent-impl function without a visibility modifier.
//!   An inherent impl with no admitted function is hidden.

use std::collections::HashSet;
use std::path::Path;

use tree_sitter::Node;

use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{Language, SourceFile, show_program_flow};
use crate::walker::WalkCtx;

pub(super) const LANGUAGE: Language = Language {
    extensions: &["rs"],
    grammar: |_| tree_sitter_rust::LANGUAGE.into(),
    extract,
    is_entrypoint: Some(is_entrypoint),
    file_weight: None,
};

fn extract(file: &SourceFile, ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let entrypoint = is_entrypoint(&file.path, ctx);
    let mut functions = Vec::new();
    let mut model = FileModel {
        module_doc: module_doc(file, root),
        ..FileModel::default()
    };
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        if matches!(
            node.kind(),
            "line_comment" | "block_comment" | "attribute_item"
        ) {
            continue;
        }
        let leading = Leading::above(node, file);
        if leading.hidden {
            continue;
        }
        match node.kind() {
            "use_declaration" | "mod_item" => {
                let is_declaration_only = node.child_by_field_name("body").is_none();
                if is_declaration_only
                    && (node.kind() == "mod_item" || visibility_modifier(node, file) == Some("pub"))
                {
                    let mut rows = leading.attribute_rows;
                    rows.extend(file.node_rows(node));
                    model.reexports.push(Item::new(rows));
                }
            }
            "impl_item" => model.decls.extend(impl_container(node, leading, file)),
            "trait_item" => model.decls.push(container(node, leading, file, |_| true)),
            "function_item" => {
                if entrypoint {
                    let is_main = node
                        .child_by_field_name("name")
                        .is_some_and(|name| file.text(name) == "main");
                    functions.push((model.decls.len(), *file.node_rows(node).end(), is_main));
                }
                model.decls.push(callable(node, leading, file));
            }
            "macro_definition" => {
                let body = macro_open_row(node, file).map(|open_row| (open_row, node));
                let entries = list_entries(node, file, |child| child.kind() == "macro_rule");
                model
                    .decls
                    .push(whole(node, leading, file, body, entries, Vec::new()));
            }
            "struct_item" | "enum_item" | "union_item" | "type_item" | "const_item"
            | "static_item" => {
                let list = node.child_by_field_name("body");
                let body = list.map(|list| (list.start_position().row + 1, list));
                let entries = list.map_or_else(Vec::new, |list| list_entries(list, file, |_| true));
                model
                    .decls
                    .push(whole(node, leading, file, body, entries, Vec::new()));
            }
            _ => {}
        }
    }
    show_program_flow(&mut model.decls, &functions);
    model
}

fn is_entrypoint(path: &Path, _ctx: &WalkCtx) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "lib.rs" | "main.rs"))
}

/// What the outer attributes and doc comments above an item say about it.
#[derive(Default)]
struct Leading {
    /// Rows of every outer attribute except `#[doc = …]`.
    attribute_rows: Vec<usize>,
    /// Rows of `///` / `/** */` comments and `#[doc = …]` attributes.
    doc_rows: Vec<usize>,
    /// `#[cfg(test)]`, `#[test]` / `#[tokio::test]`-style or `#[doc(hidden)]`.
    hidden: bool,
}

impl Leading {
    /// Walks the attributes and comments directly preceding `node`. Plain
    /// comments are skipped over (rustc ignores them); anything else ends
    /// the run.
    fn above(node: Node, file: &SourceFile) -> Self {
        let mut leading = Leading::default();
        let mut previous = node.prev_sibling();
        while let Some(sibling) = previous {
            match sibling.kind() {
                "attribute_item" => {
                    let attribute = compact_attribute(sibling, file);
                    let path = attribute.split(['(', '=']).next().unwrap_or_default();
                    let is_test = path.rsplit("::").next() == Some("test");
                    leading.hidden |=
                        is_test || attribute == "cfg(test)" || attribute == "doc(hidden)";
                    if attribute.starts_with("doc=") {
                        leading.doc_rows.extend(file.node_rows(sibling));
                    } else {
                        leading.attribute_rows.extend(file.node_rows(sibling));
                    }
                }
                "line_comment" | "block_comment" => {
                    if !file.starts_own_row(sibling) {
                        break;
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
            previous = sibling.prev_sibling();
        }
        leading
    }
}

/// An attribute item's contents between `#[` and `]`, whitespace removed:
/// `cfg(test)`, `tokio::test`, `doc="…"`.
fn compact_attribute(attribute_item: Node, file: &SourceFile) -> String {
    let mut cursor = attribute_item.walk();
    attribute_item
        .named_children(&mut cursor)
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
    let mut cursor = node.walk();
    let modifier = node
        .children(&mut cursor)
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
    rows.dedup();
    rustdoc_paragraphs(&rows, file)
}

/// A function: the head runs from its first attribute through the row
/// before its first statement; each statement is one body item.
fn callable(node: Node, leading: Leading, file: &SourceFile) -> DeclInfo {
    let name_rows = name_rows(node, "name");
    let first_row = node.start_position().row + 1;
    let statements = node
        .child_by_field_name("body")
        .map_or_else(Vec::new, |block| list_entries(block, file, |_| true));
    let head_end = match statements.first().and_then(|item| item.rows.iter().min()) {
        Some(&first_statement_row) => (first_statement_row - 1).max(name_rows[0]),
        None => *file.node_rows(node).end(),
    };
    let mut head = leading.attribute_rows.clone();
    head.extend(first_row..=head_end);
    let body = statements
        .into_iter()
        .map(|item| Item::new(item.rows.into_iter().filter(|&row| row > head_end)))
        .filter(|item| !item.rows.is_empty())
        .collect();
    DeclInfo {
        name_rows,
        head,
        doc: doc_items(&leading, file),
        body,
        shape: Shape::Callable,
        members: Vec::new(),
    }
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
    mut entries: Vec<Item>,
    members: Vec<DeclInfo>,
) -> DeclInfo {
    let node_rows = file.node_rows(node);
    let mut head = leading.attribute_rows.clone();
    let name_rows = name_rows(
        node,
        if node.kind() == "impl_item" {
            "type"
        } else {
            "name"
        },
    );
    match body {
        Some((open_row, list)) => {
            head.extend(*node_rows.start()..=open_row);
            let mut cursor = list.walk();
            let last_content_row = list
                .named_children(&mut cursor)
                .filter(|child| !matches!(child.kind(), "line_comment" | "block_comment"))
                .map(|child| *file.node_rows(child).end())
                .fold(open_row, usize::max);
            head.extend(last_content_row + 1..=*node_rows.end());
        }
        None => head.extend(node_rows),
    }
    let head_rows: HashSet<usize> = head.iter().copied().collect();
    for item in &mut entries {
        item.rows.retain(|row| !head_rows.contains(row));
    }
    entries.retain(|item| !item.rows.is_empty());
    let mut body = entries;
    body.extend(
        members
            .iter()
            .map(|member| Item::new(member.name_rows.clone())),
    );
    body.sort_by_key(|item| item.rows.iter().min().copied());
    DeclInfo {
        name_rows,
        head,
        doc: doc_items(&leading, file),
        body,
        shape: Shape::Whole,
        members,
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
    let mut members = Vec::new();
    let is_function =
        |child: Node| matches!(child.kind(), "function_item" | "function_signature_item");
    let mut cursor = list.walk();
    for child in list.named_children(&mut cursor) {
        if !is_function(child) {
            continue;
        }
        let member_leading = Leading::above(child, file);
        if member_leading.hidden {
            continue;
        }
        if admit(child) {
            members.push(callable(child, member_leading, file));
        }
    }
    let entries = list_entries(list, file, |child| {
        !is_function(child) && !Leading::above(child, file).hidden && admit(child)
    });
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

/// A trait impl admits every function; an inherent impl only those with
/// a visibility modifier (`pub`, `pub(…)`), and is hidden without one.
fn impl_container(node: Node, leading: Leading, file: &SourceFile) -> Option<DeclInfo> {
    let is_trait_impl = node.child_by_field_name("trait").is_some();
    let admit = |child: Node| is_trait_impl || visibility_modifier(child, file).is_some();
    if !is_trait_impl {
        let list = node.child_by_field_name("body")?;
        let mut cursor = list.walk();
        let admits_any = list
            .named_children(&mut cursor)
            .any(|child| admit(child) && !Leading::above(child, file).hidden);
        if !admits_any {
            return None;
        }
    }
    Some(container(node, leading, file, admit))
}

/// The row of the delimiter opening a `macro_rules!` body.
fn macro_open_row(node: Node, file: &SourceFile) -> Option<usize> {
    let mut cursor = node.walk();
    let row = node
        .children(&mut cursor)
        .find(|child| matches!(file.text(*child), "{" | "(" | "["))?
        .start_position()
        .row;
    Some(row + 1)
}

/// One item per named child of `list` that `admit` accepts, with the
/// comments and attributes directly above it; rows claimed by an earlier
/// item are not repeated. Comments after the last entry form their own
/// item.
fn list_entries(list: Node, file: &SourceFile, admit: impl Fn(Node) -> bool) -> Vec<Item> {
    let mut items = Vec::new();
    let mut claimed = HashSet::new();
    let mut pending = Vec::new();
    let mut cursor = list.walk();
    for child in list.named_children(&mut cursor) {
        match child.kind() {
            "line_comment" | "block_comment" | "attribute_item" => {
                if file.starts_own_row(child) {
                    pending.extend(file.node_rows(child));
                }
            }
            _ if admit(child) => {
                pending.extend(file.node_rows(child));
                push_unclaimed(&mut items, &mut claimed, std::mem::take(&mut pending));
            }
            _ => pending.clear(),
        }
    }
    push_unclaimed(&mut items, &mut claimed, pending);
    items
}

fn push_unclaimed(items: &mut Vec<Item>, claimed: &mut HashSet<usize>, rows: Vec<usize>) {
    let rows: Vec<usize> = rows
        .into_iter()
        .filter(|row| claimed.insert(*row))
        .collect();
    if !rows.is_empty() {
        items.push(Item::new(rows));
    }
}

/// The leading `//!` / `/*! */` rows of the file (license comments and
/// `#![…]` attributes around them skipped), as paragraphs.
fn module_doc(file: &SourceFile, root: Node) -> Vec<Item> {
    let mut rows = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
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
    rows.sort_unstable();
    rows.dedup();
    rustdoc_paragraphs(&rows, file)
}

/// A doc comment row's markdown content: the comment marker and the one
/// space after it stripped, a block comment's `*/` and leading `*` too.
fn rustdoc_content(line: &str) -> &str {
    let trimmed = line.trim();
    for marker in ["///", "//!", "/**", "/*!"] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            let rest = rest.strip_suffix("*/").unwrap_or(rest);
            return rest.strip_prefix(' ').unwrap_or(rest);
        }
    }
    let trimmed = trimmed.strip_suffix("*/").unwrap_or(trimmed);
    let trimmed = trimmed.strip_prefix('*').unwrap_or(trimmed);
    trimmed.strip_prefix(' ').unwrap_or(trimmed)
}

/// Splits sorted doc rows into paragraph items at blank doc rows; a
/// blank row stays with the paragraph above it.
fn rustdoc_paragraphs(rows: &[usize], file: &SourceFile) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut after_blank = true;
    for &row in rows {
        let blank = rustdoc_content(file.line(row)).trim().is_empty();
        match items.last_mut() {
            Some(item) if blank || !after_blank => item.rows.push(row),
            _ => items.push(Item::new([row])),
        }
        after_blank = blank;
    }
    items
}

#[cfg(test)]
mod tests {
    use super::super::test_support::rows;
    use super::*;

    fn extract_source(file_name: &str, source: &str) -> (SourceFile, FileModel) {
        super::super::test_support::extract_in(&LANGUAGE, &[(file_name, source)], file_name)
    }

    fn sorted(mut rows: Vec<usize>) -> Vec<usize> {
        rows.sort_unstable();
        rows.dedup();
        rows
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

fn one_line() -> u8 { 1 }
";
        let (_, model) = extract_source("a.rs", source);
        let run = &model.decls[0];
        assert_eq!(run.shape, Shape::Callable);
        assert_eq!(run.name_rows, vec![3]);
        assert_eq!(sorted(run.head.clone()), vec![2, 3, 4, 5]);
        assert_eq!(rows(&run.doc), vec![vec![1]]);
        assert_eq!(rows(&run.body), vec![vec![6, 7], vec![8]]);

        let one_line = &model.decls[1];
        assert_eq!(sorted(one_line.head.clone()), vec![11]);
        assert!(one_line.body.is_empty());
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
        let kind = &model.decls[0];
        assert_eq!(kind.shape, Shape::Whole);
        assert_eq!(kind.name_rows, vec![2]);
        assert_eq!(sorted(kind.head.clone()), vec![1, 2, 7]);
        assert_eq!(rows(&kind.body), vec![vec![3, 4], vec![5, 6]]);
        for (decl, row) in model.decls[1..].iter().zip(8..) {
            assert_eq!(sorted(decl.head.clone()), vec![row]);
            assert!(decl.body.is_empty());
        }
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
        let store = &model.decls[0];
        assert_eq!(sorted(store.head.clone()), vec![1, 10]);
        assert_eq!(rows(&store.body), vec![vec![2], vec![5], vec![7]]);
        let members: Vec<_> = store
            .members
            .iter()
            .map(|member| member.name_rows.clone())
            .collect();
        assert_eq!(members, vec![vec![5], vec![7]]);
        assert_eq!(rows(&store.members[0].doc), vec![vec![4]]);
        assert_eq!(sorted(store.members[1].head.clone()), vec![7]);
        assert_eq!(rows(&store.members[1].body), vec![vec![8]]);
        assert_eq!(model.decls[1].members.len(), 1);
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
        let impls: Vec<_> = model
            .decls
            .iter()
            .filter(|decl| !decl.members.is_empty())
            .map(|decl| {
                let members: Vec<_> = decl
                    .members
                    .iter()
                    .map(|member| member.name_rows[0])
                    .collect();
                (decl.name_rows[0], members)
            })
            .collect();
        assert_eq!(impls, vec![(3, vec![4, 5]), (8, vec![9]), (13, vec![14])]);
        assert_eq!(model.decls.len(), 6);
        let display = &model.decls[3];
        assert_eq!(sorted(display.head.clone()), vec![8, 12]);
        assert_eq!(rows(&display.body), vec![vec![9]]);
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
pub mod inline {
    pub fn f() {}
}
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
        let (file, model) = extract_source("a.rs", source);
        assert_eq!(
            roster(&file, &model.decls),
            vec!["macro_rules! bail {", "macro_rules! local {",]
        );
        let bail = &model.decls[0];
        assert_eq!(sorted(bail.head.clone()), vec![2, 3, 10]);
        assert_eq!(rows(&bail.body), vec![vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(rows(&bail.doc), vec![vec![1]]);
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
}
