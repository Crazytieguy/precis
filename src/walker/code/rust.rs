//! Rust extraction for the code engine.
//!
//! - **Module doc**: the file's leading `//!` / `/*! */` comments, one
//!   [`Item`] per paragraph, without the leading paragraphs that are only
//!   badges, link reference definitions or HTML (they render as nothing
//!   readable).
//! - **Re-exports**: `pub use …;` and every `mod name;` declaration (the
//!   file's module tree, whatever its visibility). Other `use` and
//!   `extern crate` items are plumbing and not modeled; an inline
//!   `mod name { … }` is left out with its contents.
//! - **Declarations**: `fn` is `Callable`; `struct`, `enum`, `union`,
//!   `type`, `const`, `static` and `macro_rules!` are `Whole`; `trait` and
//!   `impl` are `Whole` containers whose members are their functions.
//! - **Visibility**: `#[cfg(test)]`, `#[test]`-style and `#[doc(hidden)]`
//!   items are hidden. Bare `pub` (or `#[macro_export]`) is public;
//!   `pub(…)` and no modifier are private. Trait and trait-impl functions
//!   take their container's visibility; an inherent-impl function without
//!   `pub` is hidden, and the impl is as visible as its most visible
//!   member. An impl never outranks its self type or trait when the same
//!   file declares them, and is hidden with either.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use tree_sitter::Node;

use super::SourceFile;
use super::model::{DeclInfo, FileModel, Item, Shape, Visibility};
use crate::walker::WalkCtx;
use crate::walker::markdown::{fence_closes, fence_marker};

pub(super) const EXTENSIONS: &[&str] = &["rs"];

pub(super) fn grammar(_path: &Path) -> tree_sitter::Language {
    tree_sitter_rust::LANGUAGE.into()
}

pub(super) fn extract(file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    let root = file.tree.root_node();
    let mut model = FileModel {
        module_doc: module_doc(file, root),
        ..FileModel::default()
    };
    let type_visibility = declared_type_visibility(file, root);
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
                    && (node.kind() == "mod_item"
                        || modifier_visibility(node, file) == Some(Visibility::Public))
                {
                    let mut rows = leading.attribute_rows;
                    rows.extend(file.node_rows(node));
                    model.reexports.push(Item::new(rows));
                }
            }
            "impl_item" => {
                model
                    .decls
                    .extend(impl_container(node, leading, file, &type_visibility))
            }
            "trait_item" => {
                let visibility = item_visibility(node, file);
                model
                    .decls
                    .push(container(node, leading, file, visibility, |_| {
                        Some(visibility)
                    }));
            }
            "function_item" => {
                let visibility = item_visibility(node, file);
                model.decls.push(callable(node, leading, file, visibility));
            }
            "macro_definition" => {
                let visibility = if leading.macro_export {
                    Visibility::Public
                } else {
                    Visibility::Private
                };
                let body = macro_open_row(node, file).map(|open_row| (open_row, node));
                let entries = list_entries(node, file, |child| child.kind() == "macro_rule");
                model.decls.push(whole(
                    node,
                    leading,
                    file,
                    visibility,
                    body,
                    entries,
                    Vec::new(),
                ));
            }
            "struct_item" | "enum_item" | "union_item" | "type_item" | "const_item"
            | "static_item" => {
                let visibility = item_visibility(node, file);
                let list = node.child_by_field_name("body");
                let body = list.map(|list| (list.start_position().row + 1, list));
                let entries = list.map_or_else(Vec::new, |list| list_entries(list, file, |_| true));
                model.decls.push(whole(
                    node,
                    leading,
                    file,
                    visibility,
                    body,
                    entries,
                    Vec::new(),
                ));
            }
            _ => {}
        }
    }
    model
}

pub(super) fn is_entrypoint(path: &Path, _ctx: &WalkCtx) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "lib.rs" | "main.rs"))
}

pub(super) fn file_weight(_path: &Path, _ctx: &WalkCtx) -> f64 {
    1.0
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
    macro_export: bool,
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
                    leading.macro_export |= path == "macro_export";
                    if attribute.starts_with("doc=") {
                        leading.doc_rows.extend(file.node_rows(sibling));
                    } else {
                        leading.attribute_rows.extend(file.node_rows(sibling));
                    }
                }
                "line_comment" | "block_comment" => {
                    if !starts_own_line(sibling, file) {
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

fn starts_own_line(node: Node, file: &SourceFile) -> bool {
    let line = file.line(node.start_position().row + 1);
    line.len() - line.trim_start().len() == node.start_position().column
}

/// `Some(Public)` for bare `pub`, `Some(Private)` for `pub(…)`, `None`
/// without a modifier.
fn modifier_visibility(node: Node, file: &SourceFile) -> Option<Visibility> {
    let mut cursor = node.walk();
    let modifier = node
        .children(&mut cursor)
        .find(|child| child.kind() == "visibility_modifier")?;
    Some(if file.text(modifier).trim() == "pub" {
        Visibility::Public
    } else {
        Visibility::Private
    })
}

fn item_visibility(node: Node, file: &SourceFile) -> Visibility {
    modifier_visibility(node, file).unwrap_or(Visibility::Private)
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
fn callable(node: Node, leading: Leading, file: &SourceFile, visibility: Visibility) -> DeclInfo {
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
        visibility,
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
    visibility: Visibility,
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
        visibility,
        members,
    }
}

/// A `trait` or `impl`: functions in its declaration list become members
/// with the visibility `member_visibility` gives them (`None` hides one);
/// associated types and constants are body entries.
fn container(
    node: Node,
    leading: Leading,
    file: &SourceFile,
    visibility: Visibility,
    member_visibility: impl Fn(Node) -> Option<Visibility>,
) -> DeclInfo {
    let Some(list) = node.child_by_field_name("body") else {
        return whole(
            node,
            leading,
            file,
            visibility,
            None,
            Vec::new(),
            Vec::new(),
        );
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
        if let Some(own) = member_visibility(child) {
            members.push(callable(child, member_leading, file, own.min(visibility)));
        }
    }
    let entries = list_entries(list, file, |child| {
        !is_function(child)
            && !Leading::above(child, file).hidden
            && member_visibility(child).is_some()
    });
    let open_row = list.start_position().row + 1;
    whole(
        node,
        leading,
        file,
        visibility,
        Some((open_row, list)),
        entries,
        members,
    )
}

/// A trait impl is public, an inherent impl as visible as its most visible
/// member (`pub fn` / `pub(…) fn`; functions without `pub` are hidden), and
/// neither is more visible than its self type or trait when this file
/// declares them. `None` when nothing in the impl is admitted or its type
/// or trait is hidden.
fn impl_container(
    node: Node,
    leading: Leading,
    file: &SourceFile,
    type_visibility: &HashMap<String, Option<Visibility>>,
) -> Option<DeclInfo> {
    let mut declared_cap = Visibility::Public;
    for field in ["type", "trait"] {
        let declared = node
            .child_by_field_name(field)
            .and_then(|named| type_visibility.get(base_type_name(named, file)));
        match declared {
            Some(None) => return None,
            Some(Some(visibility)) => declared_cap = declared_cap.min(*visibility),
            None => {}
        }
    }
    let is_trait_impl = node.child_by_field_name("trait").is_some();
    let own_visibility = |child: Node| {
        if is_trait_impl {
            Some(Visibility::Public)
        } else {
            modifier_visibility(child, file)
        }
    };
    let visibility = if is_trait_impl {
        Visibility::Public
    } else {
        let list = node.child_by_field_name("body")?;
        let mut cursor = list.walk();
        list.named_children(&mut cursor)
            .filter_map(|child| {
                own_visibility(child).filter(|_| !Leading::above(child, file).hidden)
            })
            .max()?
    };
    Some(container(
        node,
        leading,
        file,
        visibility.min(declared_cap),
        own_visibility,
    ))
}

/// `Foo` for `Foo`, `Foo<T>` and `module::Foo<T>`.
fn base_type_name<'a>(self_type: Node, file: &'a SourceFile) -> &'a str {
    let text = file.text(self_type);
    let without_generics = text.split('<').next().unwrap_or(text);
    without_generics
        .rsplit("::")
        .next()
        .unwrap_or(without_generics)
        .trim()
}

/// Top-level type and trait names → their visibility (`None`: hidden). A
/// name declared more than once (e.g. under `#[cfg(test)]` and
/// `#[cfg(not(test))]`) takes its most visible declaration.
fn declared_type_visibility(file: &SourceFile, root: Node) -> HashMap<String, Option<Visibility>> {
    let mut declared: HashMap<String, Option<Visibility>> = HashMap::new();
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        if !matches!(
            node.kind(),
            "struct_item" | "enum_item" | "union_item" | "type_item" | "trait_item"
        ) {
            continue;
        }
        let Some(name) = node.child_by_field_name("name") else {
            continue;
        };
        let visibility = (!Leading::above(node, file).hidden).then(|| item_visibility(node, file));
        let entry = declared.entry(file.text(name).to_string()).or_default();
        *entry = (*entry).max(visibility);
    }
    declared
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
                if starts_own_line(child, file) {
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
        .into_iter()
        .skip_while(|paragraph| is_decorative_paragraph(paragraph, file))
        .collect()
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

/// Splits sorted doc rows into paragraph items at blank doc rows outside
/// code fences. A blank row stays with the paragraph above it, and a
/// paragraph followed directly by a fence keeps the fence (the prose
/// introduces it). Doctest-hidden `# …` rows inside Rust fences are left
/// out, as rustdoc leaves them out of the rendered docs.
fn rustdoc_paragraphs(rows: &[usize], file: &SourceFile) -> Vec<Item> {
    let mut items = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut after_blank = false;
    let mut open_fence: Option<((char, usize), bool)> = None;
    for &row in rows {
        let content = rustdoc_content(file.line(row)).trim_start();
        if let Some((marker, hides)) = open_fence {
            if fence_closes(content, marker) {
                open_fence = None;
            } else if hides && (content == "#" || content.starts_with("# ")) {
                continue;
            }
            current.push(row);
            continue;
        }
        if content.is_empty() {
            after_blank = true;
            current.push(row);
            continue;
        }
        let opens = fence_marker(content);
        if after_blank && opens.is_none() && !current.is_empty() {
            items.push(Item::new(std::mem::take(&mut current)));
        }
        after_blank = false;
        if let Some(marker) = opens {
            open_fence = Some((marker, is_rust_fence(&content[marker.1..])));
        }
        current.push(row);
    }
    if !current.is_empty() {
        items.push(Item::new(current));
    }
    items
}

/// Rustdoc tests an unlabeled fence and one labeled with a Rust attribute.
fn is_rust_fence(info: &str) -> bool {
    let label = info
        .trim()
        .split([',', ' ', '\t'])
        .next()
        .unwrap_or_default();
    label.is_empty()
        || label == "rust"
        || label.starts_with("edition")
        || matches!(label, "no_run" | "ignore" | "compile_fail" | "should_panic")
}

/// A paragraph that renders as no prose: only images / badges, link
/// reference definitions and HTML tags.
fn is_decorative_paragraph(paragraph: &Item, file: &SourceFile) -> bool {
    paragraph
        .rows
        .iter()
        .all(|&row| is_decorative_line(rustdoc_content(file.line(row)).trim()))
}

fn is_decorative_line(content: &str) -> bool {
    let is_reference_definition = content.starts_with('[')
        && content
            .find("]:")
            .is_some_and(|close| !content[1..close].contains(']'));
    if is_reference_definition {
        return true;
    }
    let mut depth = 0usize;
    let mut in_tag = false;
    let mut in_entity = false;
    for c in content.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            '&' if !in_tag => in_entity = true,
            ';' if in_entity => in_entity = false,
            '[' | '(' if !in_tag => depth += 1,
            ']' | ')' if !in_tag => depth = depth.saturating_sub(1),
            _ if c.is_alphanumeric() && depth == 0 && !in_tag && !in_entity => return false,
            _ => {}
        }
    }
    content.contains("![") || !content.contains('[')
}

#[cfg(test)]
mod tests {
    use super::super::Language;
    use super::*;

    fn extract_source(file_name: &str, source: &str) -> (SourceFile, FileModel) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(file_name);
        std::fs::write(&path, source).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let file = SourceFile::parse(&path, Language::Rust, &ctx).unwrap();
        let model = extract(&file, &ctx);
        assert_contract(&model);
        (file, model)
    }

    fn part_rows(items: &[Item]) -> HashSet<usize> {
        items
            .iter()
            .flat_map(|item| item.rows.iter().copied())
            .collect()
    }

    /// The [`super::super::model`] invariants a Rust model must hold:
    /// non-empty head and name rows, name rows inside the rendered decl,
    /// disjoint parts, one level of members sharing only their name rows
    /// with the container, and no row in two file-level owners.
    fn assert_contract(model: &FileModel) {
        let mut owned = part_rows(&model.module_doc);
        for row in part_rows(&model.reexports) {
            assert!(owned.insert(row), "re-export row {row} owned twice");
        }
        let check_decl = |decl: &DeclInfo| {
            assert!(!decl.name_rows.is_empty() && !decl.head.is_empty());
            let head: HashSet<usize> = decl.head.iter().copied().collect();
            let doc = part_rows(&decl.doc);
            let body = part_rows(&decl.body);
            assert!(head.is_disjoint(&doc) && head.is_disjoint(&body) && doc.is_disjoint(&body));
            let rendered: HashSet<usize> = match decl.shape {
                Shape::Callable => head.clone(),
                Shape::Whole => head.union(&body).copied().collect(),
            };
            assert!(decl.name_rows.iter().all(|row| rendered.contains(row)));
            head.union(&doc)
                .chain(body.iter())
                .copied()
                .collect::<HashSet<_>>()
        };
        for decl in &model.decls {
            let container_rows = check_decl(decl);
            let mut decl_rows = container_rows.clone();
            for member in &decl.members {
                assert!(member.members.is_empty());
                let member_rows = check_decl(member);
                let shared: Vec<_> = member_rows.intersection(&container_rows).collect();
                assert!(
                    shared.iter().all(|row| member.name_rows.contains(row)),
                    "member rows {shared:?} also in its container"
                );
                decl_rows.extend(member_rows);
            }
            for row in decl_rows {
                assert!(owned.insert(row), "declaration row {row} owned twice");
            }
        }
    }

    fn rows(items: &[Item]) -> Vec<Vec<usize>> {
        items.iter().map(|item| item.rows.clone()).collect()
    }

    fn sorted(mut rows: Vec<usize>) -> Vec<usize> {
        rows.sort_unstable();
        rows.dedup();
        rows
    }

    /// Each declaration's name row text and visibility, in source order.
    fn roster(file: &SourceFile, decls: &[DeclInfo]) -> Vec<(String, Visibility)> {
        let mut out: Vec<_> = decls
            .iter()
            .map(|decl| {
                (
                    decl.name_rows[0],
                    file.line(decl.name_rows[0]).trim().to_string(),
                    decl.visibility,
                )
            })
            .collect();
        out.sort();
        out.into_iter()
            .map(|(_, line, visibility)| (line, visibility))
            .collect()
    }

    #[test]
    fn rust_extract_module_doc_drops_badges_and_hidden_doctest_lines() {
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
            vec![vec![8, 9, 10], vec![11, 12], vec![13, 14, 16, 17]]
        );
    }

    #[test]
    fn rust_extract_doc_paragraph_keeps_the_fence_it_introduces() {
        let source = "\
/// Parses input.
///
/// For example:
///
/// ```text
/// a
///
/// b
/// ```
pub fn parse() {}
";
        let (_, model) = extract_source("a.rs", source);
        assert_eq!(
            rows(&model.decls[0].doc),
            vec![vec![1, 2], vec![3, 4, 5, 6, 7, 8, 9]]
        );
    }

    #[test]
    fn rust_extract_visibility_follows_pub_and_hides_tests_and_doc_hidden() {
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
                ("pub struct Public;".to_string(), Visibility::Public),
                ("pub(crate) struct Crate;".to_string(), Visibility::Private),
                ("struct Private;".to_string(), Visibility::Private),
                ("pub fn latest() {}".to_string(), Visibility::Public),
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
    fn rust_extract_trait_members_inherit_its_visibility() {
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
            .map(|member| (member.name_rows.clone(), member.visibility))
            .collect();
        assert_eq!(
            members,
            vec![(vec![5], Visibility::Public), (vec![7], Visibility::Public)]
        );
        assert_eq!(rows(&store.members[0].doc), vec![vec![4]]);
        assert_eq!(sorted(store.members[1].head.clone()), vec![7]);
        assert_eq!(rows(&store.members[1].body), vec![vec![8]]);
        assert_eq!(model.decls[1].members[0].visibility, Visibility::Private);
    }

    #[test]
    fn rust_extract_impl_visibility_comes_from_members_and_self_type() {
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
                    .map(|member| (member.name_rows[0], member.visibility))
                    .collect();
                (decl.name_rows[0], decl.visibility, members)
            })
            .collect();
        assert_eq!(
            impls,
            vec![
                (
                    3,
                    Visibility::Public,
                    vec![(4, Visibility::Public), (5, Visibility::Private)]
                ),
                (8, Visibility::Public, vec![(9, Visibility::Public)]),
                (13, Visibility::Private, vec![(14, Visibility::Private)]),
            ]
        );
        assert_eq!(model.decls.len(), 5);
        let display = &model.decls[3];
        assert_eq!(sorted(display.head.clone()), vec![8, 12]);
        assert_eq!(rows(&display.body), vec![vec![9]]);
    }

    #[test]
    fn rust_extract_impl_of_a_type_with_a_test_substitute_stays_visible() {
        for (first, second) in [("not(test)", "test"), ("test", "not(test)")] {
            let source = format!(
                "\
#[cfg({first})]
pub struct Client;
#[cfg({second})]
pub struct Client;
impl Client {{
    pub fn connect(&self) {{}}
}}
"
            );
            let (_, model) = extract_source("a.rs", &source);
            let client_impl = model
                .decls
                .iter()
                .find(|decl| !decl.members.is_empty())
                .expect("impl Client admitted");
            assert_eq!(client_impl.visibility, Visibility::Public);
        }
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
    fn rust_extract_macros_are_whole_and_public_only_when_exported() {
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
            vec![
                ("macro_rules! bail {".to_string(), Visibility::Public),
                ("macro_rules! local {".to_string(), Visibility::Private),
            ]
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
}
