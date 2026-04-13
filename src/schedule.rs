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
        let mut new_children = best_group.children(ctx);

        // Commit: update cache, childless_folders, and remaining budget
        commit_group(
            &mut best_group,
            ctx,
            &mut cache,
            &mut childless_folders,
            &mut remaining_tokens,
            &mut remaining_chars,
        );

        // Auto-commit compact enum bodies with their parent name,
        // bypassing ratio-based competition where they'd lose to
        // many cheap FunctionName entries.
        let mut i = 0;
        while i < new_children.len() {
            if is_auto_commit_body(&new_children[i], remaining_tokens, ctx.budget) {
                let mut child = new_children.swap_remove(i);
                ensure_cached(&mut child, ctx, &mut cache);
                let cost = probe_cost(&child, ctx, &mut cache, &childless_folders);
                if cost.tokens <= remaining_tokens
                    && remaining_chars.is_none_or(|cb| cost.chars <= cb)
                {
                    let grandchildren = child.children(ctx);
                    commit_group(
                        &mut child,
                        ctx,
                        &mut cache,
                        &mut childless_folders,
                        &mut remaining_tokens,
                        &mut remaining_chars,
                    );
                    new_children.extend(grandchildren);
                } else {
                    new_children.push(child);
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
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

/// Compute per-item folder costs for a FoldersGroup, caching the result.
fn ensure_folders_cached(g: &mut crate::group::FoldersGroup, ctx: &ScheduleCtx<'_>) {
    if g.cached_item_costs.is_some() {
        return;
    }
    let costs: Vec<FileCost> = g
        .items
        .iter()
        .map(|item_dir| {
            let rel = ctx.rel_path(item_dir);
            let header = format::folder_line(rel);
            FileCost {
                tokens: format::count_tokens(&header),
                chars: header.len(),
            }
        })
        .collect();
    g.cached_item_costs = Some(costs);
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
            g.cached_render.as_ref().unwrap().marginal_cost
        }
        Group::Folders(g) => {
            let costs = g.cached_item_costs.as_ref().unwrap();
            let mut fc = FileCost::default();
            for c in costs {
                fc.tokens += c.tokens;
                fc.chars += c.chars;
            }
            fc
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
        Group::Folders(g) => {
            let costs = g.cached_item_costs.take().unwrap();
            let mut total_cost = FileCost::default();
            for (item_dir, cost) in g.items.iter().zip(&costs) {
                let rel = ctx.rel_path(item_dir);
                childless_folders.insert(rel.to_path_buf(), *cost);
                total_cost.tokens += cost.tokens;
                total_cost.chars += cost.chars;
            }
            *remaining_tokens = remaining_tokens.saturating_sub(total_cost.tokens);
            if let Some(rc) = remaining_chars {
                *rc = rc.saturating_sub(total_cost.chars);
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

// Only auto-commit when plenty of budget remains, so bodies don't
// crowd out content that would otherwise win on ratio.
const AUTO_COMMIT_BUDGET_FRACTION: usize = 4; // remaining must exceed budget * 3/4
const AUTO_COMMIT_MIN_VALUE: f64 = 1.0;

fn is_auto_commit_body(group: &Group<'_>, remaining_tokens: usize, total_budget: usize) -> bool {
    let Group::Ts(g) = group else { return false };
    if !matches!(
        g.key,
        crate::group::TsGroupKey::EnumBody | crate::group::TsGroupKey::HeadingBody { level: 1 }
    ) {
        return false;
    }
    if group.value() < AUTO_COMMIT_MIN_VALUE {
        return false;
    }
    if remaining_tokens <= total_budget * (AUTO_COMMIT_BUDGET_FRACTION - 1) / AUTO_COMMIT_BUDGET_FRACTION {
        return false;
    }
    match g.key {
        crate::group::TsGroupKey::EnumBody => {
            let limit = crate::heuristics::COMPACT_BODY_LINE_LIMIT;
            g.items.iter().all(|item| {
                let body_start = crate::group::ts::compute_body_start_line(item);
                item.end_line.saturating_sub(body_start) <= limit
            })
        }
        crate::group::TsGroupKey::HeadingBody { level: 1 } => {
            g.items.iter().any(|item| {
                crate::classify::FileRole::from_path(item.path)
                    == crate::classify::FileRole::Readme
                    && item.path.parent().is_some_and(|p| p.as_os_str().is_empty())
            })
        }
        _ => false,
    }
}

/// Ensure a group has its cached cost computed.
fn ensure_cached<'s>(group: &mut Group<'s>, ctx: &ScheduleCtx<'s>, cache: &mut FileCache) {
    if let Group::Folders(g) = group {
        ensure_folders_cached(g, ctx);
        return;
    }
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
