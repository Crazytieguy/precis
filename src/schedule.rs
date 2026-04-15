//! Greedy frontier scheduler (design §5).

use crate::group::{Group, GroupCtx};
use crate::render::{SchedulerRenderer, TextRenderer};

/// Scheduler configuration and read-only project context.
pub struct ScheduleCtx<'s> {
    pub groups: GroupCtx<'s>,
    pub budget: usize,
    pub char_budget: Option<usize>,
}

/// Run the greedy scheduler. Returns the final output string.
pub fn schedule<'s>(seed: Vec<Group<'s>>, ctx: &ScheduleCtx<'s>) -> String {
    schedule_with_renderer(seed, ctx, TextRenderer::new())
}

fn schedule_with_renderer<'s, R>(
    seed: Vec<Group<'s>>,
    ctx: &ScheduleCtx<'s>,
    mut renderer: R,
) -> String
where
    R: SchedulerRenderer<'s>,
{
    let mut frontier: Vec<Group<'s>> = seed;
    let mut remaining_tokens = ctx.budget;
    let mut remaining_chars = ctx.char_budget;

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

            renderer.prepare(group, &ctx.groups);

            let cost = renderer.probe_cost(&frontier[idx], &ctx.groups);

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

        let mut best_group = frontier.swap_remove(best_idx);
        let mut new_children = best_group.children(&ctx.groups);

        renderer
            .commit(&mut best_group, &ctx.groups)
            .apply_to(&mut remaining_tokens, &mut remaining_chars);

        // Auto-commit compact enum bodies with their parent name,
        // bypassing ratio-based competition where they'd lose to
        // many cheap FunctionName entries.
        let mut i = 0;
        while i < new_children.len() {
            if is_auto_commit_body(&new_children[i], remaining_tokens, ctx.budget) {
                let mut child = new_children.swap_remove(i);
                renderer.prepare(&mut child, &ctx.groups);
                let cost = renderer.probe_cost(&child, &ctx.groups);
                if cost.tokens <= remaining_tokens
                    && remaining_chars.is_none_or(|cb| cost.chars <= cb)
                {
                    let grandchildren = child.children(&ctx.groups);
                    renderer
                        .commit(&mut child, &ctx.groups)
                        .apply_to(&mut remaining_tokens, &mut remaining_chars);
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

    renderer.assemble(ctx.char_budget)
}

// Only auto-commit when plenty of budget remains, so bodies don't
// crowd out content that would otherwise win on ratio.
const AUTO_COMMIT_BUDGET_FRACTION: usize = 4; // remaining must exceed budget * 3/4
const AUTO_COMMIT_MIN_VALUE: f64 = 1.0;

fn is_auto_commit_body(group: &Group<'_>, remaining_tokens: usize, total_budget: usize) -> bool {
    let Group::Ts(g) = group else { return false };
    if !matches!(
        g.key,
        crate::group::TsGroupKey::EnumBody(_)
            | crate::group::TsGroupKey::HeadingBody(crate::group::ts::heading::HeadingBody {
                level: 1
            })
    ) {
        return false;
    }
    if group.value() < AUTO_COMMIT_MIN_VALUE {
        return false;
    }
    if remaining_tokens
        <= total_budget * (AUTO_COMMIT_BUDGET_FRACTION - 1) / AUTO_COMMIT_BUDGET_FRACTION
    {
        return false;
    }
    match g.key {
        crate::group::TsGroupKey::EnumBody(_) => {
            let limit = crate::heuristics::COMPACT_BODY_LINE_LIMIT;
            g.items.iter().all(|item| {
                let body_start = crate::group::ts::compute_body_start_line(item);
                item.end_line.saturating_sub(body_start) <= limit
            })
        }
        crate::group::TsGroupKey::HeadingBody(crate::group::ts::heading::HeadingBody {
            level: 1,
        }) => g.items.iter().any(|item| {
            crate::classify::FileRole::from_path(item.path) == crate::classify::FileRole::Readme
                && item.path.parent().is_some_and(|p| p.as_os_str().is_empty())
        }),
        _ => false,
    }
}
