//! Greedy frontier scheduler (design §5).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::format;
use crate::group::Group;
use crate::render::{self, CachedGroupRender, FileCache, FileCost, RenderedEntry};
use crate::store::ParseStore;

/// Read-only shared context threaded through children() calls.
pub struct ScheduleCtx<'s> {
    pub store: &'s ParseStore,
    pub root: PathBuf,
    pub budget: usize,
    pub char_budget: Option<usize>,
}

impl ScheduleCtx<'_> {
    pub fn rel_path<'a>(&self, path: &'a Path) -> &'a Path {
        path.strip_prefix(&self.root).unwrap_or(path)
    }
}

/// Run the greedy scheduler. Returns the final output string.
pub fn schedule<'s>(seed: Vec<Group<'s>>, ctx: &ScheduleCtx<'s>) -> String {
    let mut frontier: Vec<Group<'s>> = seed;
    let mut cache = FileCache::new();
    let mut remaining_tokens = ctx.budget;
    let mut remaining_chars = ctx.char_budget;

    // Childless folder map: folders currently planned in the output and already
    // charged to the budget. When a child group commits under a folder, the
    // folder is removed from the map and its cost refunded.
    let mut childless_folders: HashMap<PathBuf, FileCost> = HashMap::new();

    loop {
        if frontier.is_empty() {
            break;
        }

        // Find the best candidate that fits the budget (A1)
        let mut best_idx: Option<usize> = None;
        let mut best_ratio: f64 = f64::NEG_INFINITY;

        #[allow(clippy::needless_range_loop)] // index-based access needed for re-borrows
        for idx in 0..frontier.len() {
            let group = &mut frontier[idx];
            let value = group.value();
            if value <= 0.0 {
                continue;
            }

            ensure_cached(group, ctx, &mut cache);

            let cost = probe_cost(&frontier[idx], ctx, &mut cache, &childless_folders);

            if cost.tokens > remaining_tokens {
                continue;
            }
            if let Some(cb) = remaining_chars
                && cost.chars > cb
            {
                continue;
            }

            let ratio = if cost.tokens == 0 {
                f64::INFINITY
            } else {
                value / cost.tokens.max(1) as f64
            };

            // Total-order comparison (A1): ratio descending, then path/line/kind ascending
            let is_better = match best_idx {
                None => true,
                Some(bi) => match ratio.total_cmp(&best_ratio) {
                    std::cmp::Ordering::Greater => true,
                    std::cmp::Ordering::Less => false,
                    std::cmp::Ordering::Equal => {
                        let (a_path, a_line, a_kind) = (
                            frontier[idx].first_path(),
                            frontier[idx].first_line(),
                            frontier[idx].kind_ordinal(),
                        );
                        let (b_path, b_line, b_kind) = (
                            frontier[bi].first_path(),
                            frontier[bi].first_line(),
                            frontier[bi].kind_ordinal(),
                        );
                        (a_path, a_line, a_kind) < (b_path, b_line, b_kind)
                    }
                },
            };

            if is_better {
                best_idx = Some(idx);
                best_ratio = ratio;
            }
        }

        // If no candidate fits, halt (A4)
        let best_idx = match best_idx {
            Some(i) => i,
            None => break,
        };

        // Atomic commit: remove → children → commit to cache → extend frontier
        let mut best_group = frontier.swap_remove(best_idx);
        let new_children = best_group.children(ctx);

        // Commit: update cache, childless_folders, and remaining budget
        commit_group(
            &mut best_group,
            &new_children,
            ctx,
            &mut cache,
            &mut childless_folders,
            &mut remaining_tokens,
            &mut remaining_chars,
        );

        frontier.extend(new_children);
    }

    // Final assembly: committed file entries + remaining childless folders
    let mut output = cache.assemble();

    // Append childless folder entries (P2). These are already paid for in the
    // budget, so no token check needed — just append in sorted order.
    let mut folder_entries: Vec<_> = childless_folders.into_iter().collect();
    folder_entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (folder_path, _cost) in folder_entries {
        let line = format::folder_line(&folder_path);
        if output.is_empty() {
            output.push_str(&line);
        } else {
            output.push('\n');
            output.push_str(&line);
        }
    }

    // Defensive trim to char budget
    if let Some(cb) = ctx.char_budget
        && output.len() > cb {
            output.truncate(cb);
            if let Some(pos) = output.rfind('\n') {
                output.truncate(pos + 1);
            }
        }

    output
}

/// Compute the marginal cost of scheduling a candidate group.
/// For child groups whose parent is in childless_folders, applies the discount.
fn probe_cost(
    group: &Group<'_>,
    ctx: &ScheduleCtx<'_>,
    cache: &mut FileCache,
    childless_folders: &HashMap<PathBuf, FileCost>,
) -> FileCost {
    match group {
        Group::Ts(g) => {
            
            // TsGroups don't directly trigger folder removal — their parent
            // FilesGroup does that. No discount here.
            g.cached_render.as_ref().unwrap().marginal_cost
        }
        Group::Folders(_g) => {
            // Cost = sum of folder-line costs for each item (sub-folder)
            // that will be added to childless_folders.
            // The Folders group itself was already in childless_folders
            // (charged when its parent committed), so there's no new cost
            // for the group's own folder line — only for its items.
            //
            // But wait: the root FoldersGroup has no parent. Its own folder
            // line is never shown (the root is implicit from the CLI arg).
            // So Folders cost = cost of the items that will become childless.
            
            // We don't know the items yet (they're discovered in children()),
            // so the cost of a Folders group is effectively zero at probe time.
            // The item costs are charged at commit time when items are discovered.
            FileCost::default()
        }
        Group::Files(g) => {
            let mut fc = FileCost::default();
            for file_path in &g.items {
                let rel = ctx.rel_path(file_path);
                if !cache.has_file(rel) {
                    let hcost = cache.header_cost_for(rel);
                    fc.tokens += hcost.tokens;
                    fc.chars += hcost.chars;
                }
            }
            // Discount: if parent folder is in childless_folders, committing
            // this Files group will remove the parent folder line from output.
            let parent_rel = ctx.rel_path(&g.parent_dir);
            if let Some(folder_cost) = childless_folders.get(parent_rel) {
                fc.tokens = fc.tokens.saturating_sub(folder_cost.tokens);
                fc.chars = fc.chars.saturating_sub(folder_cost.chars);
            }
            fc
        }
    }
}

/// Commit a group: update cache, childless_folders, and remaining budget.
fn commit_group<'s>(
    group: &mut Group<'s>,
    new_children: &[Group<'s>],
    ctx: &ScheduleCtx<'s>,
    cache: &mut FileCache,
    childless_folders: &mut HashMap<PathBuf, FileCost>,
    remaining_tokens: &mut usize,
    remaining_chars: &mut Option<usize>,
) {
    match group {
        Group::Ts(g) => {
            let cached = g.cached_render.take().unwrap();
            let cost = cached.marginal_cost;
            cache.commit(cached.per_file, cost);
            *remaining_tokens = remaining_tokens.saturating_sub(cost.tokens);
            if let Some(rc) = remaining_chars {
                *rc = rc.saturating_sub(cost.chars);
            }
        }
        Group::Folders(_g) => {
            // The Folders group's children() discovered its items (sub-folders
            // and file groups). Each sub-folder item becomes a childless folder
            // entry, charged to the budget.
            for child in new_children {
                if let Group::Folders(fg) = child {
                    let rel = ctx.rel_path(&fg.parent_dir);
                    let header = format::folder_line(rel);
                    let cost = FileCost {
                        tokens: format::count_tokens(&header),
                        chars: header.len(),
                    };
                    childless_folders.insert(rel.to_path_buf(), cost);
                    *remaining_tokens = remaining_tokens.saturating_sub(cost.tokens);
                    if let Some(rc) = remaining_chars {
                        *rc = rc.saturating_sub(cost.chars);
                    }
                }
            }
        }
        Group::Files(g) => {
            // Register file headers in the cache
            let mut fc = FileCost::default();
            for file_path in &g.items {
                let rel = ctx.rel_path(file_path);
                if !cache.has_file(rel) {
                    let hcost = cache.header_cost_for(rel);
                    fc.tokens += hcost.tokens;
                    fc.chars += hcost.chars;
                    cache.register_file(rel);
                }
            }

            // Remove parent from childless_folders and refund its cost.
            // The parent folder line is no longer needed — the child file
            // paths shown in the output already reveal the folder structure.
            let parent_rel = ctx.rel_path(&g.parent_dir);
            if let Some(folder_cost) = childless_folders.remove(parent_rel) {
                *remaining_tokens += folder_cost.tokens;
                if let Some(rc) = remaining_chars {
                    *rc += folder_cost.chars;
                }
            }

            // Charge the file header costs
            *remaining_tokens = remaining_tokens.saturating_sub(fc.tokens);
            if let Some(rc) = remaining_chars {
                *rc = rc.saturating_sub(fc.chars);
            }
        }
    }
}

/// Ensure a TsGroup has its CachedGroupRender computed.
fn ensure_cached<'s>(group: &mut Group<'s>, ctx: &ScheduleCtx<'s>, cache: &mut FileCache) {
    let Group::Ts(g) = group else { return };
    if g.cached_render.is_some() {
        return;
    }

    let raw = crate::group::ts::render_entries(g, ctx);

    let mut per_file: Vec<(PathBuf, Vec<RenderedEntry<'s>>)> = Vec::new();
    for (path, entries) in raw {
        let rendered: Vec<RenderedEntry<'s>> =
            entries.iter().map(|e| render::render_entry(e)).collect();
        per_file.push((path, rendered));
    }

    let marginal_cost = cache.marginal_cost(&per_file);

    g.cached_render = Some(CachedGroupRender {
        per_file,
        marginal_cost,
    });
}
