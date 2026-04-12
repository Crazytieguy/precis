use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::classify;
use crate::heuristics;
use crate::parse::{self, ExtractedItem, ItemKind};
use crate::schedule::ScheduleCtx;

use super::{FilesGroup, Group, TsGroup, TsItem};
use super::ts::TsGroupKey;

/// Produce children when a FilesGroup is scheduled.
/// Parses every file in the group, extracts items, and aggregates across files (D7).
pub fn children<'s>(g: &mut FilesGroup, ctx: &ScheduleCtx<'s>) -> Vec<Group<'s>> {
    struct FileItems<'s> {
        path: PathBuf,
        source: &'s str,
        items: Vec<ExtractedItem<'s>>,
    }
    let mut all_items: Vec<FileItems<'s>> = Vec::new();
    let mut generated_files: HashSet<&Path> = HashSet::new();

    for file_path in &g.items {
        // A5: parsing only happens in Files::children
        let (source, tree) = match ctx.store.parse(file_path) {
            Some(pair) => pair,
            None => {
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

        let relative = file_path.strip_prefix(&g.parent_dir).unwrap_or(file_path);
        if classify::is_generated_file(source)
            || classify::is_autogen_api_doc(source, g.role)
            || classify::is_generated_filename(relative)
        {
            generated_files.insert(file_path);
        }

        all_items.push(FileItems {
            path: file_path.clone(),
            source,
            items,
        });
    }

    // Aggregate items across files into TsGroup buckets (D7).
    // Bucket key includes generated status so items from generated files get
    // a separate group with the appropriate modifier (not tainting the rest).
    let mut buckets: HashMap<(TsGroupKey, bool), Vec<TsItem<'s>>> = HashMap::new();

    for fi in &all_items {
        let lines: Vec<&str> = fi.source.lines().collect();
        let lang = crate::Lang::from_path(fi.path.as_path());
        let display_path = ctx.store.intern_path(ctx.rel_path(&fi.path).to_path_buf());
        let is_generated = generated_files.contains(fi.path.as_path());

        for item in &fi.items {
            let keys = item_to_group_keys(item, &lines, lang);

            for key in keys {
                let ts_item = TsItem {
                    path: display_path,
                    source: fi.source,
                    node: item.node,
                    name: item.name.clone(),
                    end_line: item.end_line,
                };
                buckets.entry((key, is_generated)).or_default().push(ts_item);
            }
        }
    }

    let mut sorted_buckets: Vec<_> = buckets.into_iter().collect();
    sorted_buckets.sort_by(|a, b| a.0.cmp(&b.0));

    let mut direct: Vec<(Group<'s>, bool)> = Vec::new();
    let mut gated: Vec<(Group<'s>, bool)> = Vec::new();

    for ((key, is_generated), items) in sorted_buckets {
        if items.is_empty() {
            continue;
        }

        let modifier = compute_item_modifier(&key, g.inherited_modifier, is_generated);
        let is_gated = key.is_gated();

        let group = Group::Ts(TsGroup {
            key,
            items,
            inherited_modifier: modifier,
            dependent_siblings: vec![],
            cached_render: None,
        });

        if is_gated {
            gated.push((group, is_generated));
        } else {
            direct.push((group, is_generated));
        }
    }

    // Attach gated groups to their parent, preferring same generated status
    // so non-generated items aren't trapped behind a generated parent
    // (which may never be scheduled due to its 0.1x modifier).
    for (gated_group, gated_gen) in gated {
        let gated_key = match &gated_group {
            Group::Ts(ts) => &ts.key,
            _ => unreachable!(),
        };
        let key_matches =
            |g: &Group<'_>| matches!(g, Group::Ts(ts) if gated_key.is_gated_by(&ts.key));
        let idx = direct
            .iter()
            .position(|entry| entry.1 == gated_gen && key_matches(&entry.0))
            .or_else(|| direct.iter().position(|entry| key_matches(&entry.0)));
        if let Some(i) = idx {
            if let (Group::Ts(ref mut parent), _) = direct[i] {
                parent.dependent_siblings.push(gated_group);
            }
        } else {
            direct.push((gated_group, gated_gen));
        }
    }

    // Heading nesting: only the shallowest-level headings are direct children.
    // Deeper headings become dependent_siblings, gated behind shallower levels.
    nest_heading_groups(&mut direct);

    direct.into_iter().map(|(g, _)| g).collect()
}

/// Nest heading groups by level: deeper headings become `dependent_siblings`
/// of the closest shallower heading group so they're gated behind it (A2).
/// Only applies to Markdown headings — TOML/YAML levels are a flat namespace,
/// not structural nesting (design §4 shows nesting for headings as markdown-only).
fn nest_heading_groups(groups: &mut Vec<(Group<'_>, bool)>) {
    // Check if these heading groups come from Markdown files.
    let is_markdown = groups.iter().any(|(g, _)| {
        matches!(g, Group::Ts(ts) if matches!(&ts.key, TsGroupKey::Heading { .. })
            && ts.items.first().is_some_and(|item|
                crate::Lang::from_path(item.path) == Some(crate::Lang::Markdown)))
    });
    if !is_markdown {
        return;
    }

    let group_heading_level = |g: &Group<'_>| -> Option<u8> {
        match g {
            Group::Ts(ts) => ts.key.heading_level(),
            _ => None,
        }
    };

    let min_level = groups.iter().filter_map(|(g, _)| group_heading_level(g)).min();
    let max_level = groups.iter().filter_map(|(g, _)| group_heading_level(g)).max();

    let (Some(min_level), Some(max_level)) = (min_level, max_level) else {
        return;
    };
    if min_level == max_level {
        return;
    }

    // Process from deepest to shallowest. Each level's heading groups
    // are extracted and attached as dependent_siblings of the closest
    // shallower heading group. Since we go deep-to-shallow, a level-3
    // group is first attached to a level-2 group, then when level-2
    // is processed it (with its level-3 siblings) attaches to level-1.
    for level in (min_level + 1..=max_level).rev() {
        let mut at_level = Vec::new();
        let mut rest = Vec::new();
        for entry in groups.drain(..) {
            if group_heading_level(&entry.0) == Some(level) {
                at_level.push(entry);
            } else {
                rest.push(entry);
            }
        }
        *groups = rest;

        if at_level.is_empty() {
            continue;
        }

        // Find the closest shallower heading group: highest level < current,
        // preferring non-boilerplate on ties so sub-headings aren't trapped
        // behind a rarely-scheduled boilerplate parent.
        let parent_pos = groups
            .iter()
            .enumerate()
            .filter_map(|(i, (g, _))| {
                if let Group::Ts(ts) = g {
                    let l = ts.key.heading_level().filter(|&l| l < level)?;
                    let boilerplate = matches!(&ts.key, TsGroupKey::Heading { boilerplate: true, .. });
                    Some((i, l, boilerplate))
                } else {
                    None
                }
            })
            .max_by_key(|&(_, l, bp)| (l, std::cmp::Reverse(bp)))
            .map(|(i, _, _)| i);

        match parent_pos {
            Some(pos) => {
                if let (Group::Ts(ref mut parent), _) = groups[pos] {
                    for (child, _) in at_level {
                        parent.dependent_siblings.push(child);
                    }
                }
            }
            None => {
                // No shallower heading found; keep as direct children.
                groups.extend(at_level);
            }
        }
    }
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
pub(crate) fn compute_item_modifier(key: &TsGroupKey, parent_modifier: f64, is_generated: bool) -> f64 {
    use TsGroupKey::*;

    let generated_factor = heuristics::generated_contribution(is_generated);

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

    parent_modifier * vis_factor * doc_factor * boilerplate_factor * reexport_factor * generated_factor
}
