//! Tree-sitter group: key enum, children(), render(), and text helpers.

use std::path::PathBuf;

use crate::render::LineEntry;
use crate::schedule::ScheduleCtx;
use crate::Lang;

use super::{Group, TsGroup, TsItem};

// ---------------------------------------------------------------------------
// TsGroupKey enum (design §3.3 + plan extensions)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum TsGroupKey {
    // Module-level
    ModuleDocFirst,
    ModuleDocRest,

    // Imports
    Import { first_party: bool, reexport: bool },
    ImportedItems { first_party: bool, reexport: bool },

    // Function-like
    FunctionName { documented: bool, public: bool },
    FunctionDocFirst,
    FunctionDocRest,
    FunctionSig,
    FunctionBody,

    // Structs
    StructName { documented: bool, public: bool },
    StructDocFirst,
    StructDocRest,
    StructBody,

    // Enums
    EnumName { documented: bool, public: bool },
    EnumDocFirst,
    EnumDocRest,
    EnumBody,

    // Classes
    ClassName { documented: bool, public: bool },
    ClassDocFirst,
    ClassDocRest,

    // Interfaces
    InterfaceName { documented: bool, public: bool },
    InterfaceDocFirst,
    InterfaceDocRest,

    // Rust traits
    TraitName { documented: bool, public: bool },
    TraitDocFirst,
    TraitDocRest,

    // Rust impl blocks
    ImplBlock { is_trait_impl: bool },

    // Type aliases
    TypeAliasName { documented: bool, public: bool },
    TypeAliasDocFirst,
    TypeAliasDocRest,

    // Consts and statics
    ConstName { documented: bool, public: bool },
    ConstDocFirst,
    ConstDocRest,

    // Macros
    MacroName { documented: bool, public: bool },
    MacroDocFirst,
    MacroDocRest,

    // Markdown
    Heading { level: u8, boilerplate: bool },
    HeadingBody,

    // JSON / TOML / YAML
    DataSection,
    DataSectionBody,
}

impl TsGroupKey {
    /// Ordinal for deterministic tiebreaking (A1).
    pub fn ordinal(&self) -> u32 {
        use TsGroupKey::*;
        match self {
            ModuleDocFirst => 0,
            ModuleDocRest => 1,
            Import { .. } => 2,
            ImportedItems { .. } => 3,
            FunctionName { .. } => 10,
            FunctionDocFirst => 11,
            FunctionDocRest => 12,
            FunctionSig => 13,
            FunctionBody => 14,
            StructName { .. } => 20,
            StructDocFirst => 21,
            StructDocRest => 22,
            StructBody => 23,
            EnumName { .. } => 30,
            EnumDocFirst => 31,
            EnumDocRest => 32,
            EnumBody => 33,
            ClassName { .. } => 40,
            ClassDocFirst => 41,
            ClassDocRest => 42,
            InterfaceName { .. } => 50,
            InterfaceDocFirst => 51,
            InterfaceDocRest => 52,
            TraitName { .. } => 60,
            TraitDocFirst => 61,
            TraitDocRest => 62,
            ImplBlock { .. } => 70,
            TypeAliasName { .. } => 80,
            TypeAliasDocFirst => 81,
            TypeAliasDocRest => 82,
            ConstName { .. } => 90,
            ConstDocFirst => 91,
            ConstDocRest => 92,
            MacroName { .. } => 100,
            MacroDocFirst => 101,
            MacroDocRest => 102,
            Heading { .. } => 110,
            HeadingBody => 111,
            DataSection => 120,
            DataSectionBody => 121,
        }
    }

    /// Returns the heading level if this is a `Heading` key, `None` otherwise.
    pub fn heading_level(&self) -> Option<u8> {
        match self {
            TsGroupKey::Heading { level, .. } => Some(*level),
            _ => None,
        }
    }

    /// Whether this key should be gated behind a counterpart (dependent_sibling).
    pub fn is_gated(&self) -> bool {
        use TsGroupKey::*;
        matches!(
            self,
            FunctionName { public: false, .. }
                | StructName { public: false, .. }
                | EnumName { public: false, .. }
                | ClassName { public: false, .. }
                | InterfaceName { public: false, .. }
                | TraitName { public: false, .. }
                | TypeAliasName { public: false, .. }
                | ConstName { public: false, .. }
                | MacroName { public: false, .. }
                | Import { first_party: false, .. }
                | ImplBlock { is_trait_impl: true }
        )
    }

    /// Whether `self` should be gated behind `other` (design §4).
    /// Ignores the `documented` discriminant — a private undocumented group
    /// is gated behind a public documented group of the same kind.
    pub fn is_gated_by(&self, other: &TsGroupKey) -> bool {
        use TsGroupKey::*;
        match (self, other) {
            (FunctionName { public: false, .. }, FunctionName { public: true, .. }) => true,
            (StructName { public: false, .. }, StructName { public: true, .. }) => true,
            (EnumName { public: false, .. }, EnumName { public: true, .. }) => true,
            (ClassName { public: false, .. }, ClassName { public: true, .. }) => true,
            (InterfaceName { public: false, .. }, InterfaceName { public: true, .. }) => true,
            (TraitName { public: false, .. }, TraitName { public: true, .. }) => true,
            (TypeAliasName { public: false, .. }, TypeAliasName { public: true, .. }) => true,
            (ConstName { public: false, .. }, ConstName { public: true, .. }) => true,
            (MacroName { public: false, .. }, MacroName { public: true, .. }) => true,
            (
                Import { first_party: false, reexport: r1 },
                Import { first_party: true, reexport: r2 },
            ) => r1 == r2,
            (ImplBlock { is_trait_impl: true }, ImplBlock { is_trait_impl: false }) => true,
            _ => false,
        }
    }

    /// Maps a DocFirst variant to its corresponding DocRest variant.
    fn doc_rest_key(&self) -> Option<TsGroupKey> {
        use TsGroupKey::*;
        match self {
            FunctionDocFirst => Some(FunctionDocRest),
            StructDocFirst => Some(StructDocRest),
            EnumDocFirst => Some(EnumDocRest),
            ClassDocFirst => Some(ClassDocRest),
            InterfaceDocFirst => Some(InterfaceDocRest),
            TraitDocFirst => Some(TraitDocRest),
            TypeAliasDocFirst => Some(TypeAliasDocRest),
            ConstDocFirst => Some(ConstDocRest),
            MacroDocFirst => Some(MacroDocRest),
            ModuleDocFirst => Some(ModuleDocRest),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// children() — spawn child groups when this TsGroup is scheduled
// ---------------------------------------------------------------------------

pub fn children<'s>(g: &mut TsGroup<'s>, _ctx: &ScheduleCtx<'s>) -> Vec<Group<'s>> {
    let mut result: Vec<Group<'s>> = Vec::new();

    result.extend(std::mem::take(&mut g.dependent_siblings));

    use TsGroupKey::*;

    match &g.key {
        FunctionName { documented, .. } => {
            if !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: FunctionSig,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
            if *documented && !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: FunctionDocFirst,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        FunctionSig => {
            if !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: FunctionBody,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        StructName { documented, .. } => {
            spawn_type_children(&mut result, g, StructDocFirst, Some(StructBody), *documented);
        }
        EnumName { documented, .. } => {
            spawn_type_children(&mut result, g, EnumDocFirst, Some(EnumBody), *documented);
        }
        ClassName { documented, .. } => {
            spawn_type_children(&mut result, g, ClassDocFirst, None, *documented);
        }
        InterfaceName { documented, .. } => {
            spawn_type_children(&mut result, g, InterfaceDocFirst, None, *documented);
        }
        TraitName { documented, .. } => {
            spawn_type_children(&mut result, g, TraitDocFirst, None, *documented);
        }
        TypeAliasName { documented, .. } => {
            if *documented && !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: TypeAliasDocFirst,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        ConstName { documented, .. } => {
            if *documented && !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: ConstDocFirst,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        MacroName { documented, .. } => {
            if *documented && !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: MacroDocFirst,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        Import { first_party, reexport } => {
            if !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: ImportedItems { first_party: *first_party, reexport: *reexport },
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        Heading { level: _, .. } => {
            if !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: HeadingBody,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        DataSection => {
            if !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: DataSectionBody,
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        key if key.doc_rest_key().is_some() => {
            if !g.items.is_empty() {
                result.push(Group::Ts(TsGroup {
                    key: g.key.doc_rest_key().unwrap(),
                    items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
                    inherited_modifier: g.inherited_modifier,
                    dependent_siblings: vec![],
                    cached_render: None,
                }));
            }
        }
        // Leaf groups: no children
        _ => {}
    }

    result
}

fn spawn_type_children<'s>(
    result: &mut Vec<Group<'s>>,
    g: &TsGroup<'s>,
    doc_first: TsGroupKey,
    body: Option<TsGroupKey>,
    documented: bool,
) {
    if g.items.is_empty() {
        return;
    }
    if documented {
        result.push(Group::Ts(TsGroup {
            key: doc_first.clone(),
            items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
            inherited_modifier: g.inherited_modifier,
            dependent_siblings: vec![],
            cached_render: None,
        }));
    }
    if let Some(body_key) = body {
        result.push(Group::Ts(TsGroup {
            key: body_key,
            items: g.items.iter().map(|i| clone_ts_item(i)).collect(),
            inherited_modifier: g.inherited_modifier,
            dependent_siblings: vec![],
            cached_render: None,
        }));
    }
}

fn clone_ts_item<'s>(item: &TsItem<'s>) -> TsItem<'s> {
    TsItem {
        path: item.path.clone(),
        source: item.source,
        node: item.node,
        name: item.name.clone(),
        start_line: item.start_line,
        end_line: item.end_line,
    }
}

// ---------------------------------------------------------------------------
// render() — produce LineEntry values for this group (R3: context-free)
// ---------------------------------------------------------------------------

pub fn render_entries<'s>(g: &TsGroup<'s>, _ctx: &ScheduleCtx<'s>) -> Vec<(PathBuf, Vec<LineEntry<'s>>)> {
    let mut per_file: std::collections::HashMap<PathBuf, Vec<LineEntry<'s>>> = std::collections::HashMap::new();

    for item in &g.items {
        let entries = render_item(&g.key, item);
        per_file.entry(item.path.to_path_buf()).or_default().extend(entries);
    }

    let mut result: Vec<(PathBuf, Vec<LineEntry<'s>>)> = per_file.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

/// Render a single item according to the group key.
fn render_item<'s>(key: &TsGroupKey, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
    use TsGroupKey::*;
    let lines: Vec<&str> = item.source.lines().collect();

    match key {
        FunctionName { .. } | StructName { .. } | EnumName { .. } | ClassName { .. }
        | InterfaceName { .. } | TraitName { .. } | TypeAliasName { .. } | ConstName { .. }
        | MacroName { .. } | ImplBlock { .. } => {
            let line_idx = item.start_line;
            let line = lines.get(line_idx).copied().unwrap_or("");
            let prefix = find_name_prefix(line, &item.name);
            // prefix is a subslice of line, which is a subslice of source (R1)
            vec![LineEntry::Truncated {
                line: line_idx as u32,
                content: prefix,
            }]
        }

        FunctionSig => {
            let body_start = compute_body_start_line(item);
            let mut entries = Vec::new();
            for line_idx in item.start_line..body_start.min(lines.len()) {
                let content = lines.get(line_idx).copied().unwrap_or("");
                entries.push(LineEntry::Complete {
                    line: line_idx as u32,
                    content,
                });
            }
            entries
        }

        FunctionBody | StructBody | EnumBody => {
            let body_start = compute_body_start_line(item);
            let body_end = item.end_line;
            let mut entries = Vec::new();
            for line_idx in body_start..body_end {
                if line_idx >= lines.len() {
                    break;
                }
                let content = lines.get(line_idx).copied().unwrap_or("");
                entries.push(LineEntry::Complete {
                    line: line_idx as u32,
                    content,
                });
            }
            entries
        }

        FunctionDocFirst | StructDocFirst | EnumDocFirst | ClassDocFirst
        | InterfaceDocFirst | TraitDocFirst | TypeAliasDocFirst | ConstDocFirst
        | MacroDocFirst => {
            let doc_range = compute_doc_range(item, &lines);
            if let Some((start, end)) = doc_range {
                let mut entries = vec![];
                if start < end {
                    let content = lines.get(start).copied().unwrap_or("");
                    entries.push(LineEntry::Complete {
                        line: start as u32,
                        content,
                    });
                    if end - start > 1 {
                        entries.push(LineEntry::Ellipsis {
                            line: (start + 1) as u32,
                        });
                    }
                }
                entries
            } else {
                vec![]
            }
        }

        FunctionDocRest | StructDocRest | EnumDocRest | ClassDocRest
        | InterfaceDocRest | TraitDocRest | TypeAliasDocRest | ConstDocRest
        | MacroDocRest => {
            let doc_range = compute_doc_range(item, &lines);
            if let Some((start, end)) = doc_range {
                let mut entries = vec![];
                for line_idx in (start + 1)..end {
                    let content = lines.get(line_idx).copied().unwrap_or("");
                    entries.push(LineEntry::Complete {
                        line: line_idx as u32,
                        content,
                    });
                }
                entries
            } else {
                vec![]
            }
        }

        Import { .. } => {
            let line_idx = item.start_line;
            let line = lines.get(line_idx).copied().unwrap_or("");
            let prefix = find_import_prefix(line);
            vec![LineEntry::Truncated {
                line: line_idx as u32,
                content: prefix,
            }]
        }

        ImportedItems { .. } => {
            let mut entries = Vec::new();
            for line_idx in item.start_line..item.end_line {
                if line_idx >= lines.len() {
                    break;
                }
                let content = lines.get(line_idx).copied().unwrap_or("");
                entries.push(LineEntry::Complete {
                    line: line_idx as u32,
                    content,
                });
            }
            entries
        }

        ModuleDocFirst => {
            let lang = Lang::from_path(&item.path);
            let first_line = skip_doc_leading_noise(&lines, item.start_line, item.end_line, lang);
            if let Some(line_idx) = first_line {
                let content = lines.get(line_idx).copied().unwrap_or("");
                vec![LineEntry::Complete {
                    line: line_idx as u32,
                    content,
                }]
            } else {
                let content = lines.get(item.start_line).copied().unwrap_or("");
                vec![LineEntry::Complete {
                    line: item.start_line as u32,
                    content,
                }]
            }
        }

        ModuleDocRest => {
            let lang = Lang::from_path(&item.path);
            let first_line = skip_doc_leading_noise(&lines, item.start_line, item.end_line, lang)
                .unwrap_or(item.start_line);
            let mut entries = Vec::new();
            for line_idx in (first_line + 1)..item.end_line {
                if line_idx >= lines.len() {
                    break;
                }
                let content = lines.get(line_idx).copied().unwrap_or("");
                entries.push(LineEntry::Complete {
                    line: line_idx as u32,
                    content,
                });
            }
            entries
        }

        Heading { .. } => {
            let line_idx = item.start_line;
            let line = lines.get(line_idx).copied().unwrap_or("");
            let stripped = strip_heading_badges(line);
            // stripped is a subslice of line, which is a subslice of source
            vec![LineEntry::Complete {
                line: line_idx as u32,
                content: stripped,
            }]
        }

        HeadingBody => {
            let body_start = item.start_line + 1;
            let body_end = item.end_line;
            let content_start = skip_markdown_noise(&lines, body_start, body_end);
            let mut entries = Vec::new();
            for line_idx in content_start..body_end {
                if line_idx >= lines.len() {
                    break;
                }
                let content = lines.get(line_idx).copied().unwrap_or("");
                entries.push(LineEntry::Complete {
                    line: line_idx as u32,
                    content,
                });
            }
            entries
        }

        DataSection => {
            let line_idx = item.start_line;
            let content = lines.get(line_idx).copied().unwrap_or("");
            vec![LineEntry::Complete {
                line: line_idx as u32,
                content,
            }]
        }

        DataSectionBody => {
            let body_start = item.start_line + 1;
            let body_end = item.end_line;
            let mut entries = Vec::new();
            for line_idx in body_start..body_end {
                if line_idx >= lines.len() {
                    break;
                }
                let content = lines.get(line_idx).copied().unwrap_or("");
                entries.push(LineEntry::Complete {
                    line: line_idx as u32,
                    content,
                });
            }
            entries
        }
    }
}

// ---------------------------------------------------------------------------
// Text helpers (migrated from layout/)
// ---------------------------------------------------------------------------

/// Find a prefix of the source line up to and including the symbol name.
fn find_name_prefix<'a>(line: &'a str, name: &str) -> &'a str {
    let trimmed = line.trim_start();
    let _indent = &line[..line.len() - trimmed.len()];

    // Try to find the name as a word in the trimmed line
    if let Some(pos) = find_word(name, trimmed) {
        let end = pos + name.len();
        let prefix_end = (line.len() - trimmed.len()) + end;
        return &line[..prefix_end.min(line.len())];
    }

    // Fallback: return the whole trimmed line
    line
}

/// Find a prefix for an import line (module path before items).
fn find_import_prefix(line: &str) -> &str {
    // For most imports, the first line up to any `{` or end of line
    if let Some(pos) = line.find('{') {
        return line[..pos].trim_end();
    }
    line.trim_end()
}

fn find_word(needle: &str, haystack: &str) -> Option<usize> {
    let mut start = 0;
    let mut first_match = None;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let before_ok = abs == 0
            || (!haystack.as_bytes()[abs - 1].is_ascii_alphanumeric()
                && haystack.as_bytes()[abs - 1] != b'_');
        let end = abs + needle.len();
        let after_ok = end == haystack.len()
            || (!haystack.as_bytes()[end].is_ascii_alphanumeric()
                && haystack.as_bytes()[end] != b'_');
        if before_ok && after_ok {
            if first_match.is_none() {
                first_match = Some(abs);
            }
            let depth: i32 = haystack[..abs].bytes().fold(0, |d, b| match b {
                b'(' => d + 1,
                b')' => d - 1,
                _ => d,
            });
            if depth == 0 {
                return Some(abs);
            }
        }
        start = abs + 1;
    }
    first_match
}

/// Returns the 0-indexed line where body content begins (after `{` or `:` for Python).
fn compute_body_start_line(item: &TsItem<'_>) -> usize {
    let lang = Lang::from_path(&item.path);
    if let Some(body_start) = crate::parse::ast::compute_body_start_line(item.node, lang.unwrap_or(Lang::Rust)) {
        return body_start.min(item.end_line);
    }
    // Fallback: body starts after the declaration line
    item.start_line + 1
}

fn compute_doc_range(item: &TsItem<'_>, lines: &[&str]) -> Option<(usize, usize)> {
    let lang = Lang::from_path(&item.path);
    let doc_start_1 = crate::parse::ast::compute_doc_start_line(item.node, item.source, lang.unwrap_or(Lang::Rust))?;
    let doc_start = doc_start_1 - 1;
    let sym_line = item.start_line;
    if doc_start >= sym_line {
        return None;
    }
    // Trim delimiters
    let (trimmed_start, trimmed_end) = trim_doc_delimiters(lines, doc_start, sym_line);
    if trimmed_start >= trimmed_end {
        return None;
    }
    Some((trimmed_start, trimmed_end))
}

// Text helpers migrated from layout/doc.rs

fn trim_doc_delimiters(lines: &[&str], doc_start: usize, sym_line_0: usize) -> (usize, usize) {
    if doc_start >= sym_line_0 {
        return (sym_line_0, sym_line_0);
    }
    let mut start = doc_start;
    let mut end = sym_line_0;
    let first = lines[start].trim();
    if first == "/**" || first == "/*" {
        start += 1;
    }
    while end > start && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    if end > start && lines[end - 1].trim() == "*/" {
        end -= 1;
    }
    if start >= end {
        (sym_line_0, sym_line_0)
    } else {
        (start, end)
    }
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
    if trimmed.starts_with("![") && trimmed.ends_with(')')
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

pub(crate) use crate::parse::name::strip_heading_badges;

fn skip_doc_leading_noise(lines: &[&str], start: usize, end: usize, lang: Option<Lang>) -> Option<usize> {
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
        [b'r' | b'R', b'b' | b'B', ..] | [b'b' | b'B', b'r' | b'R', ..]
        | [b'r' | b'R', b'f' | b'F', ..] | [b'f' | b'F', b'r' | b'R', ..] => 2,
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
