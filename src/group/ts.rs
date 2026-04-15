//! Tree-sitter group: key enum, children(), render(), and text helpers.

pub mod kind;

pub mod class;
pub mod const_;
pub mod data_section;
pub mod enum_;
pub mod function;
pub mod heading;
pub mod impl_block;
pub mod import;
pub mod interface;
pub mod macro_;
pub mod module;
pub mod struct_;
pub mod trait_;
pub mod type_alias;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::Lang;
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

use super::{Group, TsGroup, TsItem};
use kind::{OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods};

/// Build one `ParseTsGroup` per match, each with a single `TsItem`. The
/// closure returns `Some(key)` to emit a group, or `None` to skip the match
/// (e.g. for filtered or polymorphic node kinds).
pub(crate) fn simple_named_groups<'s, F>(
    matches: &[OwnedQueryMatch<'s>],
    ctx: &FileCtx<'s>,
    mut key_for: F,
) -> Vec<ParseTsGroup<'s>>
where
    F: FnMut(&OwnedQueryMatch<'s>, &FileCtx<'s>) -> Option<TsGroupKey>,
{
    let mut out = Vec::with_capacity(matches.len());
    for m in matches {
        let Some(key) = key_for(m, ctx) else { continue };
        let end_line = crate::parse::compute_end_line(m.range_node);
        let item = TsItem {
            path: ctx.display_path,
            source: ctx.source,
            node: m.range_node,
            end_line,
        };
        out.push(ParseTsGroup {
            key,
            items: vec![item],
            dependent_siblings: Vec::new(),
        });
    }
    out
}

// ---------------------------------------------------------------------------
// TsGroupKey enum (design §3.3 + plan extensions)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum TsGroupKey {
    // Module-level
    ModuleDocFirst(module::ModuleDocFirst),
    ModuleDocRest(module::ModuleDocRest),

    // Imports
    Import(import::Import),
    ImportedItems(import::ImportedItems),

    // Function-like
    FunctionName(function::FunctionName),
    FunctionDocFirst(function::FunctionDocFirst),
    FunctionDocRest(function::FunctionDocRest),
    FunctionSig(function::FunctionSig),
    FunctionBody(function::FunctionBody),

    // Structs
    StructName(struct_::StructName),
    StructDocFirst(struct_::StructDocFirst),
    StructDocRest(struct_::StructDocRest),
    StructBody(struct_::StructBody),

    // Enums
    EnumName(enum_::EnumName),
    EnumDocFirst(enum_::EnumDocFirst),
    EnumDocRest(enum_::EnumDocRest),
    EnumBody(enum_::EnumBody),

    // Classes
    ClassName(class::ClassName),
    ClassDocFirst(class::ClassDocFirst),
    ClassDocRest(class::ClassDocRest),
    ClassBody(class::ClassBody),

    // Interfaces
    InterfaceName(interface::InterfaceName),
    InterfaceDocFirst(interface::InterfaceDocFirst),
    InterfaceDocRest(interface::InterfaceDocRest),

    // Rust traits
    TraitName(trait_::TraitName),
    TraitDocFirst(trait_::TraitDocFirst),
    TraitDocRest(trait_::TraitDocRest),

    // Rust impl blocks
    ImplBlock(impl_block::ImplBlock),

    // Type aliases
    TypeAliasName(type_alias::TypeAliasName),
    TypeAliasDocFirst(type_alias::TypeAliasDocFirst),
    TypeAliasDocRest(type_alias::TypeAliasDocRest),

    // Consts and statics
    ConstName(const_::ConstName),
    ConstDocFirst(const_::ConstDocFirst),
    ConstDocRest(const_::ConstDocRest),

    // Macros
    MacroName(macro_::MacroName),
    MacroDocFirst(macro_::MacroDocFirst),
    MacroDocRest(macro_::MacroDocRest),

    // Markdown
    Heading(heading::Heading),
    HeadingBody(heading::HeadingBody),

    // JSON / TOML / YAML
    DataSection(data_section::DataSection),
    DataSectionBody(data_section::DataSectionBody),
}

impl TsGroupKey {
    /// Ordinal for deterministic tiebreaking (A1).
    pub fn ordinal(&self) -> u32 {
        use TsGroupKey::*;
        match self {
            ModuleDocFirst(_) => 0,
            ModuleDocRest(_) => 1,
            Import(_) => 2,
            ImportedItems(_) => 3,
            FunctionName(_) => 10,
            FunctionDocFirst(_) => 11,
            FunctionDocRest(_) => 12,
            FunctionSig(_) => 13,
            FunctionBody(_) => 14,
            StructName(_) => 20,
            StructDocFirst(_) => 21,
            StructDocRest(_) => 22,
            StructBody(_) => 23,
            EnumName(_) => 30,
            EnumDocFirst(_) => 31,
            EnumDocRest(_) => 32,
            EnumBody(_) => 33,
            ClassName(_) => 40,
            ClassDocFirst(_) => 41,
            ClassDocRest(_) => 42,
            ClassBody(_) => 43,
            InterfaceName(_) => 50,
            InterfaceDocFirst(_) => 51,
            InterfaceDocRest(_) => 52,
            TraitName(_) => 60,
            TraitDocFirst(_) => 61,
            TraitDocRest(_) => 62,
            ImplBlock(_) => 70,
            TypeAliasName(_) => 80,
            TypeAliasDocFirst(_) => 81,
            TypeAliasDocRest(_) => 82,
            ConstName(_) => 90,
            ConstDocFirst(_) => 91,
            ConstDocRest(_) => 92,
            MacroName(_) => 100,
            MacroDocFirst(_) => 101,
            MacroDocRest(_) => 102,
            Heading(_) => 110,
            HeadingBody(_) => 111,
            DataSection(_) => 120,
            DataSectionBody(_) => 121,
        }
    }

    /// Returns the heading level if this is a `Heading` key, `None` otherwise.
    pub fn heading_level(&self) -> Option<u8> {
        match self {
            TsGroupKey::Heading(h) => Some(h.level),
            _ => None,
        }
    }

    /// For name-variant keys (`FunctionName`, `StructName`, ...), returns
    /// `Some((documented, public))`. For all other keys, returns `None`.
    pub fn name_doc_visibility(&self) -> Option<(bool, bool)> {
        use TsGroupKey::*;
        match self {
            FunctionName(n) => Some((n.documented, n.public)),
            StructName(n) => Some((n.documented, n.public)),
            EnumName(n) => Some((n.documented, n.public)),
            ClassName(n) => Some((n.documented, n.public)),
            InterfaceName(n) => Some((n.documented, n.public)),
            TraitName(n) => Some((n.documented, n.public)),
            TypeAliasName(n) => Some((n.documented, n.public)),
            ConstName(n) => Some((n.documented, n.public)),
            MacroName(n) => Some((n.documented, n.public)),
            _ => None,
        }
    }

    /// Whether this key should be gated behind a counterpart (dependent_sibling).
    pub fn is_gated(&self) -> bool {
        use TsGroupKey::*;
        match self {
            FunctionName(n) => !n.public,
            StructName(n) => !n.public,
            EnumName(n) => !n.public,
            ClassName(n) => !n.public,
            InterfaceName(n) => !n.public,
            TraitName(n) => !n.public,
            TypeAliasName(n) => !n.public,
            ConstName(n) => !n.public,
            MacroName(n) => !n.public,
            Import(i) => !i.first_party,
            ImplBlock(b) => b.is_trait_impl,
            _ => false,
        }
    }

    /// Whether `self` should be gated behind `other` (design §4).
    /// Ignores the `documented` discriminant — a private undocumented group
    /// is gated behind a public documented group of the same kind.
    pub fn is_gated_by(&self, other: &TsGroupKey) -> bool {
        use TsGroupKey::*;
        match (self, other) {
            (FunctionName(a), FunctionName(b)) => !a.public && b.public,
            (StructName(a), StructName(b)) => !a.public && b.public,
            (EnumName(a), EnumName(b)) => !a.public && b.public,
            (ClassName(a), ClassName(b)) => !a.public && b.public,
            (InterfaceName(a), InterfaceName(b)) => !a.public && b.public,
            (TraitName(a), TraitName(b)) => !a.public && b.public,
            (TypeAliasName(a), TypeAliasName(b)) => !a.public && b.public,
            (ConstName(a), ConstName(b)) => !a.public && b.public,
            (MacroName(a), MacroName(b)) => !a.public && b.public,
            (Import(a), Import(b)) => !a.first_party && b.first_party && a.reexport == b.reexport,
            (ImplBlock(a), ImplBlock(b)) => a.is_trait_impl && !b.is_trait_impl,
            _ => false,
        }
    }

}

// ---------------------------------------------------------------------------
// children() — spawn child groups when this TsGroup is scheduled
// ---------------------------------------------------------------------------

pub fn children<'s>(g: &mut TsGroup<'s>) -> Vec<Group<'s>> {
    let mut result: Vec<Group<'s>> = Vec::new();
    result.extend(std::mem::take(&mut g.dependent_siblings));
    result.extend(g.key.children(g));
    result
}

/// Spawn a single child group with the given key, sharing the parent's items.
pub(super) fn spawn_simple_child<'s>(
    out: &mut Vec<Group<'s>>,
    parent: &TsGroup<'s>,
    key: TsGroupKey,
) {
    if parent.items.is_empty() {
        return;
    }
    out.push(Group::Ts(TsGroup {
        key,
        items: parent.items.clone(),
        inherited_modifier: parent.inherited_modifier,
        dependent_siblings: vec![],
        cached_render: None,
    }));
}

pub(super) fn spawn_type_children<'s>(
    out: &mut Vec<Group<'s>>,
    parent: &TsGroup<'s>,
    doc_first: TsGroupKey,
    body: Option<TsGroupKey>,
    documented: bool,
) {
    if documented {
        spawn_simple_child(out, parent, doc_first);
    }
    if let Some(body_key) = body {
        spawn_simple_child(out, parent, body_key);
    }
}

/// Spawn FunctionName child groups for methods within a container type
/// (class, interface, trait, or impl block). Design §4: methods are
/// represented by ordinary FunctionName groups whose inherited_modifier
/// comes from the parent chain.
///
/// `modifier_factor` is an additional multiplier (e.g. 0.5 for trait impl
/// methods per design §4).
pub(super) fn spawn_method_children<'s>(
    result: &mut Vec<Group<'s>>,
    parent: &TsGroup<'s>,
    modifier_factor: f64,
) {
    let Some(first_item) = parent.items.first() else {
        return;
    };
    let lang = Lang::from_path(first_item.path);

    let mut buckets: HashMap<(bool, bool), Vec<TsItem<'s>>> = HashMap::new();

    for item in &parent.items {
        let mut method_nodes = find_method_nodes(item.node, lang);
        if matches!(lang, Some(Lang::TypeScript | Lang::Tsx)) {
            method_nodes = dedup_method_overloads(method_nodes, item.source);
        }

        for method_node in method_nodes {
            let is_public = crate::parse::visibility::symbol_visibility(
                method_node,
                item.source,
                lang.unwrap_or(Lang::Rust),
            );
            let is_documented = crate::parse::ast::is_documented(
                method_node,
                item.source,
                lang.unwrap_or(Lang::Rust),
            );

            let ts_item = TsItem {
                path: item.path,
                source: item.source,
                node: method_node,
                end_line: method_node.end_position().row + 1,
            };

            buckets
                .entry((is_documented, is_public))
                .or_default()
                .push(ts_item);
        }
    }

    if buckets.is_empty() {
        return;
    }

    let base_modifier = parent.inherited_modifier * modifier_factor;

    let mut direct: Vec<Group<'s>> = Vec::new();
    let mut gated: Vec<Group<'s>> = Vec::new();

    let mut sorted_buckets: Vec<_> = buckets.into_iter().collect();
    sorted_buckets.sort_by_key(|&(k, _)| k);

    for ((documented, public), items) in sorted_buckets {
        let key = TsGroupKey::FunctionName(function::FunctionName { documented, public });
        let modifier = super::files::compute_item_modifier(&key, base_modifier, false);

        let group = Group::Ts(TsGroup {
            key,
            items,
            inherited_modifier: modifier,
            dependent_siblings: vec![],
            cached_render: None,
        });

        if public {
            direct.push(group);
        } else {
            gated.push(group);
        }
    }

    // Attach private method groups as dependent_siblings of public groups.
    // If no public methods exist, promote private groups directly (design §4).
    if !direct.is_empty() {
        for gated_group in gated {
            if let Group::Ts(ref mut parent_group) = direct[0] {
                parent_group.dependent_siblings.push(gated_group);
            }
        }
        result.extend(direct);
    } else {
        result.extend(gated);
    }
}

/// Deduplicate consecutive method overloads (3+) by name, keeping only the
/// last of each run. Runs of 2 are kept since pairs often represent meaningful
/// distinct signatures (e.g. generic + wildcard).
fn dedup_method_overloads<'a>(
    mut nodes: Vec<tree_sitter::Node<'a>>,
    source: &str,
) -> Vec<tree_sitter::Node<'a>> {
    if nodes.len() <= 2 {
        return nodes;
    }
    let names: Vec<&str> = nodes
        .iter()
        .map(|n| {
            n.child_by_field_name("name")
                .and_then(|name| name.utf8_text(source.as_bytes()).ok())
                .unwrap_or("")
        })
        .collect();
    let mut keep = vec![true; nodes.len()];
    let mut i = 0;
    while i < names.len() {
        if names[i].is_empty() {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < names.len() && names[j] == names[i] {
            j += 1;
        }
        if j - i >= 3 {
            for flag in keep.iter_mut().take(j - 1).skip(i) {
                *flag = false;
            }
        }
        i = j;
    }
    let mut idx = 0;
    nodes.retain(|_| {
        let kept = keep[idx];
        idx += 1;
        kept
    });
    nodes
}

/// Skip past a leading Python docstring in a body so body groups don't
/// duplicate content already handled by DocFirst/DocRest groups.
fn skip_leading_docstring(
    container_node: tree_sitter::Node,
    lang: Option<Lang>,
    body_start: usize,
) -> usize {
    if lang != Some(Lang::Python) {
        return body_start;
    }
    let Some(body) = container_node.child_by_field_name("body") else {
        return body_start;
    };
    let mut cursor = body.walk();
    for child in body.children(&mut cursor) {
        if child.is_extra() || child.kind() == "comment" {
            continue;
        }
        if child.kind() == "expression_statement"
            && child.child(0).is_some_and(|c| c.kind() == "string")
        {
            return child.end_position().row + 1;
        }
        break;
    }
    body_start
}

/// Find the source line of the first method (or decorated method) in a class body.
/// Returns `None` if the class has no methods.
fn find_first_method_line(container_node: tree_sitter::Node, lang: Option<Lang>) -> Option<usize> {
    let body = container_node.child_by_field_name("body")?;
    let mut cursor = body.walk();

    for child in body.children(&mut cursor) {
        if is_method_node(child, lang)
            || (lang == Some(Lang::Python) && child.kind() == "decorated_definition")
        {
            return Some(child.start_position().row);
        }
    }

    None
}

/// Find method/function child nodes within a container type's AST node.
fn find_method_nodes<'a>(
    container_node: tree_sitter::Node<'a>,
    lang: Option<Lang>,
) -> Vec<tree_sitter::Node<'a>> {
    let Some(body) = container_node.child_by_field_name("body") else {
        return vec![];
    };

    let mut methods = vec![];
    let mut cursor = body.walk();

    for child in body.children(&mut cursor) {
        if is_method_node(child, lang) {
            methods.push(child);
        } else if lang == Some(Lang::Python) && child.kind() == "decorated_definition" {
            // Python decorated methods: unwrap to the function_definition inside.
            let mut inner_cursor = child.walk();
            for inner in child.children(&mut inner_cursor) {
                if inner.kind() == "function_definition" {
                    methods.push(inner);
                    break;
                }
            }
        }
    }

    methods
}

/// Check whether a tree-sitter node represents a method/function declaration.
fn is_method_node(node: tree_sitter::Node, lang: Option<Lang>) -> bool {
    match lang {
        Some(Lang::Rust) => {
            matches!(node.kind(), "function_item" | "function_signature_item")
        }
        Some(Lang::TypeScript | Lang::Tsx) => match node.kind() {
            "method_definition" | "method_signature" | "abstract_method_signature" => true,
            "public_field_definition" => node.child_by_field_name("value").is_some_and(|v| {
                matches!(
                    v.kind(),
                    "arrow_function" | "function_expression" | "generator_function"
                )
            }),
            _ => false,
        },
        Some(Lang::Java) => {
            matches!(
                node.kind(),
                "method_declaration" | "constructor_declaration"
            )
        }
        Some(Lang::Python) => node.kind() == "function_definition",
        Some(Lang::C | Lang::Cpp) => node.kind() == "function_definition",
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// render() — produce LineEntry values for this group (R3: context-free)
// ---------------------------------------------------------------------------

pub fn render_entries<'s>(g: &TsGroup<'s>) -> Vec<(PathBuf, Vec<LineEntry<'s>>)> {
    let mut per_file: HashMap<&'s std::path::Path, Vec<LineEntry<'s>>> = HashMap::new();

    for item in &g.items {
        let entries = g.key.render_item(item);
        per_file.entry(item.path).or_default().extend(entries);
    }

    let mut result: Vec<(PathBuf, Vec<LineEntry<'s>>)> = per_file
        .into_iter()
        .map(|(p, entries)| (p.to_path_buf(), entries))
        .collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

// Render helpers shared across per-family impls. Each helper captures one
// rendering pattern from the old central `render_item` match and is called
// from the family impls in `src/group/ts/<family>.rs`.

/// Emit `LineEntry::Complete` for every line in `[start, end)`, clamping `end`
/// to `lines.len()`.
fn complete_line_entries<'s>(
    lines: &[&'s str],
    start: usize,
    end: usize,
) -> Vec<LineEntry<'s>> {
    let end = end.min(lines.len());
    (start..end)
        .map(|i| LineEntry::Complete {
            line: i as u32,
            content: lines[i],
        })
        .collect()
}

/// Return the `idx`-th line of `item.source` without materializing a
/// `Vec<&str>` for the entire file. Used by single-line render helpers where
/// the full line vector would be wasted work.
fn item_line<'s>(item: &TsItem<'s>, idx: usize) -> &'s str {
    item.source.lines().nth(idx).unwrap_or("")
}

pub(super) fn render_name_line<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let line_idx = item.start_line();
    let prefix = find_name_end_prefix(item_line(item, line_idx), item.node, line_idx);
    vec![LineEntry::Truncated {
        line: line_idx as u32,
        content: prefix,
    }]
}

pub(super) fn render_macro_name<'s>(preproc: bool, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    if !preproc {
        return render_name_line(item);
    }
    let line_idx = item.start_line();
    vec![LineEntry::Complete {
        line: line_idx as u32,
        content: item_line(item, line_idx),
    }]
}

pub(super) fn render_impl_block_line<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let line_idx = item.start_line();
    let line = item_line(item, line_idx);
    let prefix = item
        .node
        .child_by_field_name("type")
        .filter(|n| n.end_position().row == line_idx)
        .map(|n| &line[..n.end_position().column.min(line.len())])
        .unwrap_or(line);
    vec![LineEntry::Truncated {
        line: line_idx as u32,
        content: prefix,
    }]
}

pub(super) fn render_function_sig_lines<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    complete_line_entries(&lines, item.start_line(), compute_body_start_line(item))
}

/// Render a body (function/struct/enum/class body). `skip_docstring` is used
/// by FunctionBody and ClassBody; `stop_at_first_method` is used by ClassBody.
pub(super) fn render_body_lines<'s>(
    item: &TsItem<'s>,
    skip_docstring: bool,
    stop_at_first_method: bool,
) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let body_start = compute_body_start_line(item);
    let body_end = item.end_line;
    let lang = Lang::from_path(item.path);
    let content_start = if skip_docstring {
        skip_leading_docstring(item.node, lang, body_start)
    } else {
        body_start
    };
    let effective_end = if stop_at_first_method {
        find_first_method_line(item.node, lang).unwrap_or(body_end)
    } else {
        body_end
    };
    complete_line_entries(&lines, content_start, effective_end)
}

pub(super) fn render_doc_first_lines<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let Some((start, end)) = compute_doc_range(item, &lines) else {
        return vec![];
    };
    if start >= end {
        return vec![];
    }
    let mut entries = vec![LineEntry::Complete {
        line: start as u32,
        content: lines[start],
    }];
    if end - start > 1 {
        entries.push(LineEntry::Ellipsis {
            line: (start + 1) as u32,
        });
    }
    entries
}

pub(super) fn render_doc_rest_lines<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let Some((start, end)) = compute_doc_range(item, &lines) else {
        return vec![];
    };
    complete_line_entries(&lines, start + 1, end)
}

pub(super) fn render_import_line<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let line_idx = item.start_line();
    let prefix = find_import_prefix(lines[line_idx]);
    vec![LineEntry::Truncated {
        line: line_idx as u32,
        content: prefix,
    }]
}

pub(super) fn render_imported_items_lines<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    complete_line_entries(&lines, item.start_line(), item.end_line)
}

pub(super) fn render_module_doc_first_lines<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let start_line = item.start_line();
    let lang = Lang::from_path(item.path);
    let first_line =
        skip_doc_leading_noise(&lines, start_line, item.end_line, lang).unwrap_or(start_line);
    vec![LineEntry::Complete {
        line: first_line as u32,
        content: lines[first_line],
    }]
}

pub(super) fn render_module_doc_rest_lines<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let start_line = item.start_line();
    let lang = Lang::from_path(item.path);
    let first_line =
        skip_doc_leading_noise(&lines, start_line, item.end_line, lang).unwrap_or(start_line);
    complete_line_entries(&lines, first_line + 1, item.end_line)
}

pub(super) fn render_heading_line<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let line_idx = item.start_line();
    let stripped = strip_heading_badges(lines[line_idx]);
    vec![capped_line_entry(line_idx as u32, stripped)]
}

pub(super) fn render_heading_body_lines<'s>(
    level: u8,
    item: &TsItem<'s>,
) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let start_line = item.start_line();
    // Setext headings span both the title and the `===`/`---` underline rows;
    // body content starts past the underline. Other heading-like nodes (ATX,
    // TOML tables, YAML pairs) span either just the header row or the entire
    // section, neither of which we want to use to shift `body_start`.
    let body_start = if item.node.kind() == "setext_heading" {
        let node_end = item.node.end_position();
        let node_end_line = if node_end.column == 0 && node_end.row > start_line {
            node_end.row
        } else {
            node_end.row + 1
        };
        node_end_line.max(start_line + 1)
    } else {
        start_line + 1
    };
    let body_end = item.end_line;
    let content_start = skip_markdown_noise(&lines, body_start, body_end);
    let is_markdown = Lang::from_path(item.path) == Some(Lang::Markdown);
    let max_body_lines = if is_markdown && level == 1 {
        MARKDOWN_H1_BODY_LINE_CAP
    } else {
        usize::MAX
    };
    let capped_end = body_end.min(content_start.saturating_add(max_body_lines));
    complete_line_entries(&lines, content_start, capped_end)
}

pub(super) fn render_data_section_line<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    let line_idx = item.start_line();
    vec![capped_line_entry(line_idx as u32, lines[line_idx])]
}

pub(super) fn render_data_section_body_lines<'s>(item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    let lines: Vec<&str> = item.source.lines().collect();
    complete_line_entries(&lines, item.start_line() + 1, item.end_line)
}

// Central dispatcher — forwards to per-family impls in src/group/ts/<family>.rs.
impl TsGroupKindMethods for TsGroupKey {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        use TsGroupKey::*;
        match self {
            ModuleDocFirst(k) => k.render_item(item),
            ModuleDocRest(k) => k.render_item(item),
            Import(k) => k.render_item(item),
            ImportedItems(k) => k.render_item(item),
            FunctionName(k) => k.render_item(item),
            FunctionDocFirst(k) => k.render_item(item),
            FunctionDocRest(k) => k.render_item(item),
            FunctionSig(k) => k.render_item(item),
            FunctionBody(k) => k.render_item(item),
            StructName(k) => k.render_item(item),
            StructDocFirst(k) => k.render_item(item),
            StructDocRest(k) => k.render_item(item),
            StructBody(k) => k.render_item(item),
            EnumName(k) => k.render_item(item),
            EnumDocFirst(k) => k.render_item(item),
            EnumDocRest(k) => k.render_item(item),
            EnumBody(k) => k.render_item(item),
            ClassName(k) => k.render_item(item),
            ClassDocFirst(k) => k.render_item(item),
            ClassDocRest(k) => k.render_item(item),
            ClassBody(k) => k.render_item(item),
            InterfaceName(k) => k.render_item(item),
            InterfaceDocFirst(k) => k.render_item(item),
            InterfaceDocRest(k) => k.render_item(item),
            TraitName(k) => k.render_item(item),
            TraitDocFirst(k) => k.render_item(item),
            TraitDocRest(k) => k.render_item(item),
            ImplBlock(k) => k.render_item(item),
            TypeAliasName(k) => k.render_item(item),
            TypeAliasDocFirst(k) => k.render_item(item),
            TypeAliasDocRest(k) => k.render_item(item),
            ConstName(k) => k.render_item(item),
            ConstDocFirst(k) => k.render_item(item),
            ConstDocRest(k) => k.render_item(item),
            MacroName(k) => k.render_item(item),
            MacroDocFirst(k) => k.render_item(item),
            MacroDocRest(k) => k.render_item(item),
            Heading(k) => k.render_item(item),
            HeadingBody(k) => k.render_item(item),
            DataSection(k) => k.render_item(item),
            DataSectionBody(k) => k.render_item(item),
        }
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        use TsGroupKey::*;
        match self {
            ModuleDocFirst(k) => k.children(parent),
            ModuleDocRest(k) => k.children(parent),
            Import(k) => k.children(parent),
            ImportedItems(k) => k.children(parent),
            FunctionName(k) => k.children(parent),
            FunctionDocFirst(k) => k.children(parent),
            FunctionDocRest(k) => k.children(parent),
            FunctionSig(k) => k.children(parent),
            FunctionBody(k) => k.children(parent),
            StructName(k) => k.children(parent),
            StructDocFirst(k) => k.children(parent),
            StructDocRest(k) => k.children(parent),
            StructBody(k) => k.children(parent),
            EnumName(k) => k.children(parent),
            EnumDocFirst(k) => k.children(parent),
            EnumDocRest(k) => k.children(parent),
            EnumBody(k) => k.children(parent),
            ClassName(k) => k.children(parent),
            ClassDocFirst(k) => k.children(parent),
            ClassDocRest(k) => k.children(parent),
            ClassBody(k) => k.children(parent),
            InterfaceName(k) => k.children(parent),
            InterfaceDocFirst(k) => k.children(parent),
            InterfaceDocRest(k) => k.children(parent),
            TraitName(k) => k.children(parent),
            TraitDocFirst(k) => k.children(parent),
            TraitDocRest(k) => k.children(parent),
            ImplBlock(k) => k.children(parent),
            TypeAliasName(k) => k.children(parent),
            TypeAliasDocFirst(k) => k.children(parent),
            TypeAliasDocRest(k) => k.children(parent),
            ConstName(k) => k.children(parent),
            ConstDocFirst(k) => k.children(parent),
            ConstDocRest(k) => k.children(parent),
            MacroName(k) => k.children(parent),
            MacroDocFirst(k) => k.children(parent),
            MacroDocRest(k) => k.children(parent),
            Heading(k) => k.children(parent),
            HeadingBody(k) => k.children(parent),
            DataSection(k) => k.children(parent),
            DataSectionBody(k) => k.children(parent),
        }
    }
}

// ---------------------------------------------------------------------------
// Text helpers
// ---------------------------------------------------------------------------

const MAX_RENDERED_LINE_BYTES: usize = 200;
const MARKDOWN_H1_BODY_LINE_CAP: usize = 12;
pub(super) const README_H1_BODY_BOOST: f64 = 2.0;

fn capped_line_entry(line: u32, content: &str) -> LineEntry<'_> {
    if content.len() > MAX_RENDERED_LINE_BYTES {
        LineEntry::Truncated {
            line,
            content: &content[..content.floor_char_boundary(MAX_RENDERED_LINE_BYTES)],
        }
    } else {
        LineEntry::Complete { line, content }
    }
}

/// Find a prefix of the source line up to and including the item's name,
/// using the tree-sitter AST node to locate the identifier end position.
fn find_name_end_prefix<'a>(line: &'a str, node: tree_sitter::Node, line_row: usize) -> &'a str {
    if let Some(col) = find_name_end_col(node, line_row)
        && col <= line.len()
    {
        return &line[..col];
    }
    line
}

fn find_name_end_col(node: tree_sitter::Node, line_row: usize) -> Option<usize> {
    if let Some(name_node) = node.child_by_field_name("name")
        && name_node.end_position().row == line_row
    {
        return Some(name_node.end_position().column);
    }
    if let Some(decl) = node.child_by_field_name("declarator") {
        let found = crate::parse::ast::find_descendant_of_kind(decl, "identifier")
            .or_else(|| crate::parse::ast::find_descendant_of_kind(decl, "type_identifier"))
            .or_else(|| crate::parse::ast::find_descendant_of_kind(decl, "field_identifier"));
        if let Some(id) = found
            && id.end_position().row == line_row
        {
            return Some(id.end_position().column);
        }
    }
    if node.kind() == "lexical_declaration" {
        let mut cursor = node.walk();
        if let Some(vd) = node
            .children(&mut cursor)
            .find(|c| c.kind() == "variable_declarator")
            && let Some(name_node) = vd.child_by_field_name("name")
            && name_node.end_position().row == line_row
        {
            return Some(name_node.end_position().column);
        }
    }
    if matches!(node.kind(), "field_declaration" | "constant_declaration")
        && let Some(vd) = crate::parse::ast::find_descendant_of_kind(node, "variable_declarator")
        && let Some(name_node) = vd.child_by_field_name("name")
        && name_node.end_position().row == line_row
    {
        return Some(name_node.end_position().column);
    }
    if node.kind() == "expression_statement" {
        let mut cursor = node.walk();
        if let Some(assignment) = node
            .children(&mut cursor)
            .find(|c| c.kind() == "assignment")
            && let Some(left) = assignment.child_by_field_name("left")
            && left.end_position().row == line_row
        {
            return Some(left.end_position().column);
        }
    }
    if matches!(node.kind(), "variable_declaration" | "assignment_statement") {
        let target = if node.kind() == "variable_declaration" {
            let mut cursor = node.walk();
            node.children(&mut cursor)
                .find(|c| c.kind() == "assignment_statement")
        } else {
            Some(node)
        };
        if let Some(assign) = target {
            let mut cursor = assign.walk();
            if let Some(vl) = assign
                .children(&mut cursor)
                .find(|c| c.kind() == "variable_list")
                && let Some(name_node) = vl.child_by_field_name("name")
                && name_node.end_position().row == line_row
            {
                return Some(name_node.end_position().column);
            }
        }
    }
    if matches!(node.kind(), "const_declaration" | "var_declaration") {
        let kw_len = if node.kind() == "const_declaration" {
            5
        } else {
            3
        };
        return Some(node.start_position().column + kw_len);
    }
    None
}

/// Find a prefix for an import line (module path before items).
fn find_import_prefix(line: &str) -> &str {
    if let Some(pos) = line.find('{') {
        return line[..pos].trim_end();
    }
    line.trim_end()
}

/// Returns the 0-indexed line where body content begins (after `{` or `:` for Python).
pub(crate) fn compute_body_start_line(item: &TsItem<'_>) -> usize {
    let lang = Lang::from_path(item.path);
    if let Some(body_start) =
        crate::parse::ast::compute_body_start_line(item.node, lang.unwrap_or(Lang::Rust))
    {
        return body_start.min(item.end_line);
    }
    item.start_line() + 1
}

fn compute_doc_range(item: &TsItem<'_>, lines: &[&str]) -> Option<(usize, usize)> {
    let lang = Lang::from_path(item.path);
    let actual_lang = lang.unwrap_or(Lang::Rust);

    // Try outer doc comments first (preceding siblings)
    if let Some((doc_start_1, doc_end_1)) =
        crate::parse::ast::compute_doc_line_range(item.node, item.source, actual_lang)
    {
        let doc_start = doc_start_1 - 1;
        let doc_end = (doc_end_1 - 1).min(item.start_line());
        if doc_start < doc_end {
            let (trimmed_start, trimmed_end) = trim_doc_delimiters(lines, doc_start, doc_end);
            if trimmed_start < trimmed_end {
                return Some((trimmed_start, trimmed_end));
            }
        }
    }

    if actual_lang == Lang::Python
        && let Some((start_1, end_excl)) =
            crate::parse::ast::compute_python_docstring_range(item.node, item.source)
    {
        let start = start_1 - 1;
        if start < end_excl && end_excl <= lines.len() {
            return Some((start, end_excl));
        }
    }

    None
}

// Text helpers migrated from layout/doc.rs

fn trim_doc_delimiters(lines: &[&str], doc_start: usize, sym_line_0: usize) -> (usize, usize) {
    if doc_start >= sym_line_0 {
        return (sym_line_0, sym_line_0);
    }
    let mut start = doc_start;
    let mut end = sym_line_0;
    while start < end && is_bare_doc_delimiter(lines[start].trim()) {
        start += 1;
    }
    while end > start {
        let trimmed = lines[end - 1].trim();
        if !trimmed.is_empty() && !is_bare_doc_delimiter(trimmed) {
            break;
        }
        end -= 1;
    }
    if start >= end {
        (sym_line_0, sym_line_0)
    } else {
        (start, end)
    }
}

fn is_bare_doc_delimiter(trimmed: &str) -> bool {
    matches!(trimmed, "/**" | "/*" | "*/" | "---" | "///")
}

// Noise detection (migrated from layout/noise.rs)

fn is_markdown_leading_noise(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }
    if trimmed.starts_with("[![") && trimmed.ends_with(')') {
        return true;
    }
    if trimmed.starts_with("![")
        && trimmed.ends_with(')')
        && let Some(alt_end) = trimmed.find("](")
        && trimmed[2..alt_end].len() <= 30
    {
        return true;
    }
    if trimmed.starts_with('[')
        && let Some(pos) = trimmed.find("]: ")
    {
        let after = trimmed[pos + 3..].trim();
        if after.starts_with("http") || after.starts_with('/') || after.starts_with('#') {
            return true;
        }
    }
    if is_toc_link(trimmed) {
        return true;
    }
    if let Some(rest) = trimmed.strip_prefix('<') {
        let tag_start = rest.strip_prefix('/').unwrap_or(rest);
        let tag_end = tag_start
            .find(|c: char| !c.is_ascii_alphabetic())
            .unwrap_or(tag_start.len());
        let tag = &tag_start[..tag_end];
        if rest.starts_with('!')
            || tag.eq_ignore_ascii_case("div")
            || tag.eq_ignore_ascii_case("p")
            || tag.eq_ignore_ascii_case("img")
            || tag.eq_ignore_ascii_case("br")
            || tag.eq_ignore_ascii_case("hr")
            || tag.eq_ignore_ascii_case("table")
            || tag.eq_ignore_ascii_case("details")
            || tag.eq_ignore_ascii_case("summary")
            || tag.eq_ignore_ascii_case("picture")
            || tag.eq_ignore_ascii_case("figure")
            || tag.eq_ignore_ascii_case("center")
        {
            return true;
        }
    }
    if is_horizontal_rule(trimmed) {
        return true;
    }
    false
}

fn is_toc_link(trimmed: &str) -> bool {
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
        let rest = trimmed[2..].trim_start();
        return rest.starts_with('[') && rest.contains("](#") && rest.ends_with(')');
    }
    false
}

fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < 3 {
        return false;
    }
    let mut rule_char = None;
    let mut count = 0;
    for b in trimmed.bytes() {
        match b {
            b'-' | b'*' | b'_' => {
                if let Some(rc) = rule_char {
                    if b != rc {
                        return false;
                    }
                } else {
                    rule_char = Some(b);
                }
                count += 1;
            }
            b' ' => {}
            _ => return false,
        }
    }
    count >= 3
}

use crate::classify::strip_heading_badges;

fn skip_doc_leading_noise(
    lines: &[&str],
    start: usize,
    end: usize,
    lang: Option<Lang>,
) -> Option<usize> {
    (start..end).find(|&i| !is_markdown_leading_noise(strip_doc_line_prefix(lines[i], lang)))
}

fn strip_doc_line_prefix(line: &str, lang: Option<Lang>) -> &str {
    let trimmed = line.trim_start();
    if matches!(trimmed, "/*" | "/**" | "/*!" | "*/" | "*") {
        return "";
    }
    match lang {
        Some(Lang::Rust) => trimmed
            .strip_prefix("//!")
            .or_else(|| trimmed.strip_prefix("///"))
            .or_else(|| trimmed.strip_prefix("/*!"))
            .or_else(|| trimmed.strip_prefix("* "))
            .map(|s| s.strip_prefix(' ').unwrap_or(s))
            .unwrap_or(trimmed),
        Some(Lang::Go) => trimmed
            .strip_prefix("//")
            .or_else(|| trimmed.strip_prefix("* "))
            .map(|s| s.strip_prefix(' ').unwrap_or(s))
            .unwrap_or(trimmed),
        Some(Lang::Java) => trimmed
            .strip_prefix("/**")
            .or_else(|| trimmed.strip_prefix("* "))
            .map(|s| s.strip_prefix(' ').unwrap_or(s))
            .unwrap_or(trimmed),
        Some(Lang::Lua) => trimmed
            .strip_prefix("---")
            .map(|s| s.strip_prefix(' ').unwrap_or(s))
            .unwrap_or(trimmed),
        Some(Lang::Python) => {
            let prefix_len = python_string_prefix_len(trimmed);
            let after_prefix = &trimmed[prefix_len..];
            if let Some(rest) = after_prefix.strip_prefix("\"\"\"") {
                rest
            } else if let Some(rest) = after_prefix.strip_prefix("'''") {
                rest
            } else {
                trimmed
            }
        }
        _ => trimmed,
    }
}

fn python_string_prefix_len(trimmed: &str) -> usize {
    let bytes = trimmed.as_bytes();
    match bytes {
        [b'r' | b'R', b'b' | b'B', ..]
        | [b'b' | b'B', b'r' | b'R', ..]
        | [b'r' | b'R', b'f' | b'F', ..]
        | [b'f' | b'F', b'r' | b'R', ..] => 2,
        [b'r' | b'R', ..] | [b'u' | b'U', ..] | [b'f' | b'F', ..] | [b'b' | b'B', ..] => 1,
        _ => 0,
    }
}

fn skip_markdown_noise(lines: &[&str], start: usize, end: usize) -> usize {
    for i in start..end {
        if !is_markdown_leading_noise(lines.get(i).copied().unwrap_or("")) {
            return i;
        }
    }
    end
}
