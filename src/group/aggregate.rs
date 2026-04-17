//! Generic `ParseTsGroup` pipeline passes.
//!
//! These are the kind-agnostic transforms that run between per-kind
//! `from_parse` and calibration. The files-group driver (`group::files`)
//! threads them together; nothing here knows which kind is which.
//!
//! Per-file passes, in order:
//! 1. `merge_groups_by_key` — coalesce per-match groups into one group per
//!    distinct key, so downstream passes see at most one group per key.
//! 2. `apply_line_overlap_chainer` — lifts items whose source lines overlap
//!    an earlier item into the earlier item's `dependent_siblings`, capping
//!    chain depth at 5.
//! 3. `apply_per_file_gating` — driven by `TsGroupKey::gating_key`, attaches
//!    gated groups as `dependent_siblings` of the first direct counterpart
//!    sharing the same kind and bucket. Promotes orphans to direct.
//!
//! Cross-file:
//! 4. `aggregate_across_files` — unions every file's groups into the final
//!    set fed to calibration.
//! 5. `finalize_group` — convert the aggregated `ParseTsGroup` tree into
//!    scheduled `TsGroup` values, applying per-key modifiers from
//!    `calibration::compute_item_modifier`.

use std::collections::btree_map::{BTreeMap, Entry};

use crate::group::TsGroupKey;
use crate::group::ts::kind::ParseTsGroup;
use crate::group::{Group, TsGroup};

pub fn merge_groups_by_key<'s>(groups: Vec<ParseTsGroup<'s>>) -> Vec<ParseTsGroup<'s>> {
    let mut by_key: BTreeMap<TsGroupKey, ParseTsGroup<'s>> = BTreeMap::new();
    for g in groups {
        insert_or_merge(&mut by_key, g);
    }
    by_key.into_values().collect()
}

pub fn aggregate_across_files<'s>(
    per_file: Vec<Vec<ParseTsGroup<'s>>>,
) -> Vec<ParseTsGroup<'s>> {
    let mut by_key: BTreeMap<TsGroupKey, ParseTsGroup<'s>> = BTreeMap::new();
    for file_groups in per_file {
        for g in file_groups {
            insert_or_merge(&mut by_key, g);
        }
    }
    by_key.into_values().collect()
}

fn insert_or_merge<'s>(
    map: &mut BTreeMap<TsGroupKey, ParseTsGroup<'s>>,
    g: ParseTsGroup<'s>,
) {
    match map.entry(g.key) {
        Entry::Occupied(mut e) => merge_into(e.get_mut(), g),
        Entry::Vacant(e) => {
            e.insert(g);
        }
    }
}

/// Depth-capped line-overlap chainer. When a later item's start_line falls
/// inside the running chain's end_line, lift that item out of its source
/// group and attach it as a `dependent_sibling` of the chain root's group.
/// Items beyond `MAX_CHAIN_DEPTH` are dropped (matching the old
/// `dedup_line_overlaps` behavior for deeper overlaps).
///
/// Why lift the single item, not the whole group: the source group may carry
/// unrelated items that shouldn't chain. Lifting preserves per-item semantics.
///
/// Scope: only direct groups are walked, not pre-existing `dependent_siblings`.
/// At chainer time, the only kind that pre-attaches siblings is `Heading`, and
/// markdown grammar forces headings onto distinct lines — heading nests
/// therefore have no overlapping items to normalize.
pub fn apply_line_overlap_chainer<'s>(
    mut groups: Vec<ParseTsGroup<'s>>,
) -> Vec<ParseTsGroup<'s>> {
    const MAX_CHAIN_DEPTH: usize = 5;

    if groups.iter().map(|g| g.items.len()).sum::<usize>() <= 1 {
        return groups;
    }

    let mut flat: Vec<(usize, usize)> = Vec::new();
    for (gi, g) in groups.iter().enumerate() {
        for ii in 0..g.items.len() {
            flat.push((gi, ii));
        }
    }
    flat.sort_by_key(|&(gi, ii)| {
        let item = &groups[gi].items[ii];
        (item.start_line(), item.node.start_byte())
    });

    let mut victims: Vec<(usize, usize, Option<usize>)> = Vec::new();
    let mut chain_end: usize = 0;
    let mut chain_root_gi: Option<usize> = None;
    let mut chain_depth: usize = 0;

    for &(gi, ii) in &flat {
        let item = &groups[gi].items[ii];
        let start = item.start_line();
        let end = item.end_line;

        if chain_root_gi.is_none() || start >= chain_end {
            chain_end = end;
            chain_root_gi = Some(gi);
            chain_depth = 0;
        } else {
            chain_end = chain_end.max(end);
            chain_depth += 1;
            let target = if chain_depth >= MAX_CHAIN_DEPTH {
                None
            } else {
                chain_root_gi
            };
            victims.push((gi, ii, target));
        }
    }

    if victims.is_empty() {
        return groups;
    }

    // Process in (gi, ii) descending so removals don't shift indices we
    // haven't yet touched.
    victims.sort_by_key(|&(gi, ii, _)| (std::cmp::Reverse(gi), std::cmp::Reverse(ii)));

    for (gi, ii, target) in victims {
        let key = groups[gi].key;
        let item = groups[gi].items.remove(ii);
        let Some(root_gi) = target else {
            continue;
        };
        let root = &mut groups[root_gi];
        if let Some(existing) = root
            .dependent_siblings
            .iter_mut()
            .find(|s| s.key == key)
        {
            existing.items.push(item);
        } else {
            root.dependent_siblings.push(ParseTsGroup {
                key,
                items: vec![item],
                dependent_siblings: Vec::new(),
            });
        }
    }

    groups.retain(|g| !g.items.is_empty());
    groups
}

/// Attach gated groups as `dependent_siblings` of their direct counterparts
/// within the same file. Driven entirely by `TsGroupKey::gating_key`:
/// groups with the same `(kind_ordinal, bucket)` compete for one direct slot;
/// those with `is_direct = false` attach to the first direct, or promote
/// themselves to direct when no counterpart exists.
pub fn apply_per_file_gating<'s>(groups: Vec<ParseTsGroup<'s>>) -> Vec<ParseTsGroup<'s>> {
    let mut out: Vec<ParseTsGroup<'s>> = Vec::new();
    // Deferred gated groups, tagged with their (kind_ordinal, bucket).
    let mut gated: Vec<((u32, u8), ParseTsGroup<'s>)> = Vec::new();

    for g in groups {
        match g.key.gating_key() {
            None | Some((true, _)) => out.push(g),
            Some((false, bucket)) => gated.push(((g.key.ordinal(), bucket), g)),
        }
    }

    for (bucket, g) in gated {
        let parent = out.iter_mut().find(|dg| {
            dg.key.gating_key()
                == Some((true, bucket.1))
                && dg.key.ordinal() == bucket.0
        });
        match parent {
            Some(p) => p.dependent_siblings.push(g),
            None => out.push(g),
        }
    }

    out
}

fn merge_into<'s>(dest: &mut ParseTsGroup<'s>, src: ParseTsGroup<'s>) {
    dest.items.extend(src.items);
    if src.dependent_siblings.is_empty() {
        return;
    }
    if dest.dependent_siblings.is_empty() {
        dest.dependent_siblings = src.dependent_siblings;
        return;
    }
    let mut sibling_map: BTreeMap<TsGroupKey, ParseTsGroup<'s>> = BTreeMap::new();
    for s in dest.dependent_siblings.drain(..) {
        insert_or_merge(&mut sibling_map, s);
    }
    for s in src.dependent_siblings {
        insert_or_merge(&mut sibling_map, s);
    }
    dest.dependent_siblings = sibling_map.into_values().collect();
}

/// Convert a pre-calibration `ParseTsGroup` tree into a scheduled `TsGroup`.
///
/// Dependent siblings are siblings, not children — each is finalized with the
/// same `parent_modifier`, not the containing group's already-finalized value.
pub fn finalize_group<'s>(pg: ParseTsGroup<'s>, parent_modifier: f64) -> TsGroup<'s> {
    let modifier = crate::calibration::compute_item_modifier(&pg.key, parent_modifier);
    let dependent_siblings: Vec<Group<'s>> = pg
        .dependent_siblings
        .into_iter()
        .map(|sibling| Group::Ts(finalize_group(sibling, parent_modifier)))
        .collect();
    TsGroup {
        key: pg.key,
        items: pg.items,
        inherited_modifier: modifier,
        dependent_siblings,
        cached_render: None,
    }
}
