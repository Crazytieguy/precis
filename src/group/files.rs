use std::collections::HashMap;
use std::path::PathBuf;

use crate::classify;
use crate::heuristics;
use crate::parse::{self, ExtractedItem, ItemKind};
use crate::schedule::ScheduleCtx;

use super::{FilesGroup, Group, TsGroup, TsItem};
use super::ts::TsGroupKey;

/// Produce children when a FilesGroup is scheduled.
/// Parses every file in the group, extracts items, and aggregates across files (D7).
pub fn children<'s>(g: &mut FilesGroup, ctx: &ScheduleCtx<'s>) -> Vec<Group<'s>> {
    let mut result: Vec<Group<'s>> = Vec::new();

    // Parse all files and extract items.
    // We collect (PathBuf, source_ref, items) where source_ref borrows from the store.
    struct FileItems<'s> {
        path: PathBuf,
        source: &'s str,
        items: Vec<ExtractedItem<'s>>,
    }
    let mut all_items: Vec<FileItems<'s>> = Vec::new();

    for file_path in &g.items {
        // Parse via the store (A5: parsing only happens in Files::children)
        let (source, tree) = match ctx.store.parse(file_path) {
            Some(pair) => pair,
            None => {
                // Unsupported language or read failure — store source for header
                ctx.store.store_source(
                    file_path,
                    std::fs::read_to_string(file_path).unwrap_or_default(),
                );
                continue;
            }
        };

        let config = match ctx.store.config_for(file_path) {
            Some(c) => c,
            None => continue,
        };

        let items = parse::extract_items(file_path, source, tree, config);

        // Check if this file is generated
        if !g.is_generated {
            let relative = file_path.strip_prefix(&g.parent_dir).unwrap_or(file_path);
            if classify::is_generated_file(source)
                || classify::is_autogen_api_doc(source, g.role)
                || classify::is_generated_filename(relative)
            {
                g.is_generated = true;
            }
        }

        all_items.push(FileItems {
            path: file_path.clone(),
            source,
            items,
        });
    }

    // Aggregate items across files into TsGroup buckets (D7)
    // Key: (TsGroupKey, modifier bucket discriminant)
    let mut buckets: HashMap<TsGroupKey, Vec<TsItem<'s>>> = HashMap::new();

    for fi in &all_items {
        let lines: Vec<&str> = fi.source.lines().collect();
        let lang = crate::Lang::from_path(fi.path.as_path());
        // Use relative path for display
        let display_path = fi.path.strip_prefix(&ctx.root).unwrap_or(&fi.path).to_path_buf();

        for item in &fi.items {
            let keys = item_to_group_keys(item, &lines, lang);

            for key in keys {
                let ts_item = TsItem {
                    path: display_path.clone(),
                    source: fi.source,
                    node: item.node,
                    name: item.name.clone(),
                    start_line: item.start_line,
                    end_line: item.end_line,
                };
                buckets.entry(key).or_default().push(ts_item);
            }
        }
    }

    // Convert buckets to TsGroups (sorted for determinism)
    let mut sorted_buckets: Vec<_> = buckets.into_iter().collect();
    sorted_buckets.sort_by(|a, b| a.0.cmp(&b.0));
    for (key, items) in sorted_buckets {
        if items.is_empty() {
            continue;
        }

        let modifier = compute_item_modifier(&key, g.inherited_modifier);

        result.push(Group::Ts(TsGroup {
            key,
            items,
            inherited_modifier: modifier,
            dependent_siblings: vec![],
            cached_render: None,
        }));
    }

    result
}

/// Map an extracted item to its initial TsGroupKey(s).
/// An item may produce multiple keys (e.g., a documented public function
/// produces FunctionName which will later spawn doc/sig/body children).
pub(crate) fn item_to_group_keys(
    item: &ExtractedItem<'_>,
    lines: &[&str],
    lang: Option<crate::Lang>,
) -> Vec<TsGroupKey> {
    use ItemKind::*;
    use TsGroupKey::*;

    match item.kind {
        Function => vec![FunctionName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Struct => vec![StructName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Enum => vec![EnumName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Class => vec![ClassName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Interface => vec![InterfaceName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Trait => vec![TraitName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Impl => {
            let is_trait_impl = item.is_trait_impl;
            vec![ImplBlock { is_trait_impl }]
        }
        TypeAlias => vec![TypeAliasName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Const | Static => vec![ConstName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Macro => vec![MacroName {
            documented: item.is_documented,
            public: item.is_public,
        }],
        Module => {
            // Module items are filtered out before reaching this point
            // (see extract_items in parse/mod.rs)
            unreachable!("Module items should be filtered before grouping")
        }
        Section => {
            let level = if let Some(crate::Lang::Toml) = lang {
                
                item.name.chars().filter(|&c| c == '.').count() as u8 + 1
            } else if let Some(crate::Lang::Markdown) = lang {
                classify::detect_heading_depth(lines, item.start_line, item.end_line)
            } else {
                1 // JSON/YAML: all top-level
            };
            let boilerplate = matches!(lang, Some(crate::Lang::Markdown))
                && classify::is_boilerplate_heading(&item.name);
            vec![Heading {
                level,
                boilerplate,
            }]
        }
        Import => {
            let reexport = item.is_reexport;
            vec![TsGroupKey::Import {
                first_party: item.is_first_party,
                reexport,
            }]
        }
        ModuleDoc => vec![ModuleDocFirst],
    }
}

/// Compute the inherited modifier for a TsGroup based on its key and parent modifier.
pub(crate) fn compute_item_modifier(key: &TsGroupKey, parent_modifier: f64) -> f64 {
    use TsGroupKey::*;

    let vis_factor = match key {
        FunctionName { public, .. } | StructName { public, .. } | EnumName { public, .. }
        | ClassName { public, .. } | InterfaceName { public, .. } | TraitName { public, .. }
        | TypeAliasName { public, .. } | ConstName { public, .. } | MacroName { public, .. } => {
            heuristics::visibility_contribution(*public)
        }
        _ => 1.0,
    };

    let doc_factor = match key {
        FunctionName { documented, .. } | StructName { documented, .. } | EnumName { documented, .. }
        | ClassName { documented, .. } | InterfaceName { documented, .. } | TraitName { documented, .. }
        | TypeAliasName { documented, .. } | ConstName { documented, .. } | MacroName { documented, .. } => {
            heuristics::documented_contribution(*documented, key)
        }
        _ => heuristics::documented_contribution(true, key),
    };

    let boilerplate_factor = match key {
        Heading { boilerplate: true, .. } => heuristics::boilerplate_heading_contribution(),
        _ => 1.0,
    };

    let reexport_factor = match key {
        Import { reexport: true, .. } | ImportedItems { reexport: true, .. } => {
            heuristics::reexport_contribution()
        }
        _ => 1.0,
    };

    parent_modifier * vis_factor * doc_factor * boilerplate_factor * reexport_factor
}
