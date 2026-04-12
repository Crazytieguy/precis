//! Tree-sitter query execution, symbol classification, and cross-kind nesting filter.

pub(crate) mod ast;
pub(crate) mod classify;
pub(crate) mod module_doc;
pub(crate) mod name;
pub(crate) mod postprocess;
pub(crate) mod visibility;

use std::path::Path;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, QueryCursor};

use crate::store::LanguageConfig;
use crate::Lang;

/// A single extracted top-level item from a source file.
pub struct ExtractedItem<'t> {
    pub node: Node<'t>,
    pub kind: ItemKind,
    pub name: String,
    pub is_public: bool,
    pub is_first_party: bool,
    pub is_trait_impl: bool,
    pub is_reexport: bool,
    pub is_documented: bool,
    pub start_line: usize,
    pub end_line: usize,
}

/// Classification of an extracted item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    TypeAlias,
    Const,
    Static,
    Macro,
    Module,
    Class,
    Interface,
    Section,
    Import,
    ModuleDoc,
}

impl ItemKind {
    pub fn is_section_like(self) -> bool {
        matches!(self, ItemKind::Section | ItemKind::ModuleDoc)
    }
}

/// Extract top-level items from a source file using a pre-compiled language config.
/// Applies the cross-kind nesting filter (D4) to ensure non-overlapping items.
pub fn extract_items<'t>(
    path: &Path,
    source: &'t str,
    tree: &'t tree_sitter::Tree,
    config: &LanguageConfig,
) -> Vec<ExtractedItem<'t>> {
    let lang = config.lang;
    let root = tree.root_node();

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&config.query, root, source.as_bytes());

    let symbol_idx = config.symbol_idx;
    let name_idx = config.name_idx;

    let mut items = Vec::new();

    // Detect module-level documentation first
    if let Some(module_doc) = module_doc::detect_module_doc(root, source, lang) {
        items.push(module_doc);
    }

    while let Some(m) = matches.next() {
        let symbol_node = match m.captures.iter().find(|c| c.index == symbol_idx) {
            Some(c) => c.node,
            None => continue,
        };

        let kind = match classify::classify_node(
            symbol_node,
            m.captures,
            name_idx,
            source,
            lang,
            path,
        ) {
            Some(k) => k,
            None => continue,
        };

        if ast::is_inside_function(symbol_node) {
            continue;
        }
        if lang == Lang::Rust && ast::is_rust_test_code(symbol_node, source) {
            continue;
        }
        if lang == Lang::Rust && ast::is_inside_rust_anon_const(symbol_node, source) {
            continue;
        }

        let name_capture = name_idx
            .and_then(|idx| m.captures.iter().find(|c| c.index == idx))
            .map(|c| c.node);
        let item_name = match name::extract_name(symbol_node, kind, name_capture, source, lang) {
            Some(n) => n,
            None => continue,
        };

        let is_public =
            visibility::determine_visibility(symbol_node, kind, &item_name, source, lang);

        let is_first_party = if kind == ItemKind::Import {
            name::is_first_party_import(&item_name, lang)
        } else {
            false
        };

        let is_trait_impl = lang == Lang::Rust
            && matches!(kind, ItemKind::Function | ItemKind::Const | ItemKind::TypeAlias)
            && ast::is_in_trait_impl(symbol_node);

        // C++ template handling
        let effective_node = if lang == Lang::C
            && symbol_node
                .parent()
                .is_some_and(|p| p.kind() == "template_declaration")
        {
            symbol_node.parent().unwrap()
        } else {
            symbol_node
        };

        let start_line = effective_node.start_position().row;
        let end_line = if lang == Lang::C
            && matches!(
                effective_node.kind(),
                "preproc_include" | "preproc_def" | "preproc_function_def"
            )
            && effective_node.end_position().column == 0
            && effective_node.end_position().row > effective_node.start_position().row
        {
            effective_node.end_position().row
        } else {
            effective_node.end_position().row + 1
        };

        let has_doc = ast::compute_doc_start_line(effective_node, source, lang).is_some();

        items.push(ExtractedItem {
            node: effective_node,
            kind,
            name: item_name,
            is_public,
            is_first_party,
            is_trait_impl,
            is_reexport: false,
            is_documented: has_doc,
            start_line,
            end_line,
        });
    }

    postprocess::finalize(&mut items, lang, source);
    filter_nested_items(&mut items);

    debug_assert!(
        {
            let mut sorted: Vec<_> = items.iter().map(|i| (i.start_line, i.end_line)).collect();
            sorted.sort();
            sorted.windows(2).all(|w| w[0].1 <= w[1].0)
        },
        "D4 violation: top-level items overlap after nesting filter"
    );

    items
}

/// Cross-kind nesting filter: drop any item whose node byte range is fully
/// contained within another item's node byte range.
fn filter_nested_items(items: &mut Vec<ExtractedItem<'_>>) {
    if items.len() <= 1 {
        return;
    }

    items.sort_by(|a, b| {
        a.node
            .start_byte()
            .cmp(&b.node.start_byte())
            .then(b.node.end_byte().cmp(&a.node.end_byte()))
    });

    let mut keep = vec![true; items.len()];
    let mut stack: Vec<(usize, usize)> = Vec::new();

    for (i, item) in items.iter().enumerate() {
        let start = item.node.start_byte();
        let end = item.node.end_byte();

        while let Some(&(_, parent_end)) = stack.last() {
            if parent_end <= start {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&(parent_start, parent_end)) = stack.last()
            && start >= parent_start && end <= parent_end {
                keep[i] = false;
                continue;
            }

        stack.push((start, end));
    }

    let mut write = 0;
    for (read, &kept) in keep.iter().enumerate() {
        if kept {
            if write != read {
                items.swap(write, read);
            }
            write += 1;
        }
    }
    items.truncate(write);

    items.sort_by_key(|i| i.start_line);
}
