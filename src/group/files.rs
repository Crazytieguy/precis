use std::collections::HashMap;

use crate::calibration;
use crate::classify;
use crate::parse;

use super::ts::TsGroupKey;
use super::{FilesGroup, Group, GroupCtx, TsGroup, TsItem};

/// Produce children when a FilesGroup is scheduled.
/// Parses every file in the group, extracts items, and aggregates across files (D7).
pub fn children<'s>(g: &mut FilesGroup, ctx: &GroupCtx<'s>) -> Vec<Group<'s>> {
    let mut buckets: HashMap<TsGroupKey, Vec<TsItem<'s>>> = HashMap::new();

    for file_path in &g.items {
        // A5: parsing only happens through the file-to-Ts expansion path.
        // Read source first so generated-file detection can short-circuit
        // before tree-sitter parses anything.
        let relative = file_path.strip_prefix(&g.parent_dir).unwrap_or(file_path);
        if classify::is_generated_filename(relative) {
            continue;
        }
        let Some(source) = ctx.store.read_source(file_path) else {
            continue;
        };
        if classify::is_generated_file(source) || classify::is_autogen_api_doc(source, g.role) {
            continue;
        }

        let Some((source, tree)) = ctx.store.parse(file_path) else {
            continue;
        };
        let Some(config) = ctx.store.config_for(file_path) else {
            continue;
        };

        let display_path = ctx.store.intern_path(ctx.rel_path(file_path).to_path_buf());
        let items = parse::extract_items(display_path, source, tree, config);

        for (key, ts_item) in items {
            buckets.entry(key).or_default().push(ts_item);
        }
    }

    let mut sorted_buckets: Vec<_> = buckets.into_iter().collect();
    sorted_buckets.sort_by(|a, b| a.0.cmp(&b.0));

    let mut direct: Vec<Group<'s>> = Vec::new();
    let mut gated: Vec<Group<'s>> = Vec::new();

    for (key, items) in sorted_buckets {
        if items.is_empty() {
            continue;
        }

        let modifier = compute_item_modifier(&key, g.inherited_modifier);
        let is_gated = key.is_gated();

        let group = Group::Ts(TsGroup {
            key,
            items,
            inherited_modifier: modifier,
            dependent_siblings: vec![],
            cached_render: None,
        });

        if is_gated {
            gated.push(group);
        } else {
            direct.push(group);
        }
    }

    for gated_group in gated {
        let gated_key = match &gated_group {
            Group::Ts(ts) => &ts.key,
            _ => unreachable!(),
        };
        let idx = direct.iter().position(
            |g| matches!(g, Group::Ts(ts) if gated_key.is_gated_by(&ts.key)),
        );
        if let Some(i) = idx {
            if let Group::Ts(ref mut parent) = direct[i] {
                parent.dependent_siblings.push(gated_group);
            }
        } else {
            direct.push(gated_group);
        }
    }

    // Heading nesting: only the shallowest-level headings are direct children.
    // Deeper headings become dependent_siblings, gated behind shallower levels.
    nest_heading_groups(&mut direct);

    direct
}

/// Nest heading groups by level: deeper headings become `dependent_siblings`
/// of the closest shallower heading group so they're gated behind it (A2).
/// Only applies to Markdown headings — TOML/YAML levels are a flat namespace,
/// not structural nesting (design §4 shows nesting for headings as markdown-only).
fn nest_heading_groups(groups: &mut Vec<Group<'_>>) {
    // Check if these heading groups come from Markdown files.
    let is_markdown = groups.iter().any(|g| {
        matches!(g, Group::Ts(ts) if matches!(&ts.key, TsGroupKey::Heading(_))
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

    let min_level = groups.iter().filter_map(group_heading_level).min();
    let max_level = groups.iter().filter_map(group_heading_level).max();

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
            if group_heading_level(&entry) == Some(level) {
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
            .filter_map(|(i, g)| {
                if let Group::Ts(ts) = g {
                    let l = ts.key.heading_level().filter(|&l| l < level)?;
                    let boilerplate = matches!(
                        &ts.key,
                        TsGroupKey::Heading(h) if h.boilerplate
                    );
                    Some((i, l, boilerplate))
                } else {
                    None
                }
            })
            .max_by_key(|&(_, l, bp)| (l, std::cmp::Reverse(bp)))
            .map(|(i, _, _)| i);

        match parent_pos {
            Some(pos) => {
                if let Group::Ts(ref mut parent) = groups[pos] {
                    for child in at_level {
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

/// Compute the inherited modifier for a TsGroup based on its key and parent modifier.
pub(crate) fn compute_item_modifier(key: &TsGroupKey, parent_modifier: f64) -> f64 {
    use TsGroupKey::*;

    let (doc_factor, vis_factor) = match key.name_doc_visibility() {
        Some((documented, public)) => {
            let doc = if documented {
                1.0
            } else {
                calibration::UNDOCUMENTED_FACTOR
            };
            let vis = if public {
                1.0
            } else {
                calibration::PRIVATE_FACTOR
            };
            (doc, vis)
        }
        None => (1.0, 1.0),
    };

    let boilerplate_factor = match key {
        Heading(h) if h.boilerplate => calibration::BOILERPLATE_HEADING_FACTOR,
        _ => 1.0,
    };

    let reexport_factor = match key {
        Import(i) if i.reexport => calibration::REEXPORT_FACTOR,
        ImportedItems(i) if i.reexport => calibration::REEXPORT_FACTOR,
        _ => 1.0,
    };

    parent_modifier * vis_factor * doc_factor * boilerplate_factor * reexport_factor
}
