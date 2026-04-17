use crate::classify;
use crate::group::aggregate;
use crate::parse;

use super::ts::kind::ParseTsGroup;
use super::{FilesGroup, Group, GroupCtx};

/// Produce children when a FilesGroup is scheduled.
///
/// For each file, run the per-file pipeline (parse → merge-by-key →
/// line-overlap chainer → per-file gating), then aggregate across files and
/// finalize modifiers. The generic passes live in `group::aggregate`.
pub fn children<'s>(g: &mut FilesGroup, ctx: &GroupCtx<'s>) -> Vec<Group<'s>> {
    let mut per_file: Vec<Vec<ParseTsGroup<'s>>> = Vec::with_capacity(g.items.len());

    for file_path in &g.items {
        if let Some(groups) = parse_one_file(file_path, g, ctx) {
            per_file.push(groups);
        }
    }

    let aggregated = aggregate::aggregate_across_files(per_file);

    aggregated
        .into_iter()
        .map(|pg| Group::Ts(aggregate::finalize_group(pg, g.inherited_modifier)))
        .collect()
}

/// Parse `file_path` and run the per-file pipeline, or return `None` if the
/// file is generated, unreadable, or the language config is missing.
///
/// `merge_groups_by_key` collapses single-match groups into one group per
/// kind so the chainer and gating pass see at most one public group per
/// bucket (rather than pairing against individual matches).
fn parse_one_file<'s>(
    file_path: &std::path::Path,
    g: &FilesGroup,
    ctx: &GroupCtx<'s>,
) -> Option<Vec<ParseTsGroup<'s>>> {
    let relative = file_path.strip_prefix(&g.parent_dir).unwrap_or(file_path);
    if classify::is_generated_filename(relative) {
        return None;
    }
    let source = ctx.store.read_source(file_path)?;
    if classify::is_generated_file(source) || classify::is_autogen_api_doc(source, g.role) {
        return None;
    }

    let (source, tree) = ctx.store.parse(file_path)?;
    let config = ctx.store.config_for(file_path)?;

    let display_path = ctx.store.intern_path(ctx.rel_path(file_path).to_path_buf());
    let groups = parse::extract_items(display_path, source, tree, config);
    let groups = aggregate::merge_groups_by_key(groups);
    let groups = aggregate::apply_line_overlap_chainer(groups);
    Some(aggregate::apply_per_file_gating(groups))
}

