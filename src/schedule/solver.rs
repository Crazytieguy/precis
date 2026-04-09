use std::collections::{BinaryHeap, HashMap, HashSet};
use std::path::Path;

use crate::Corpus;

use super::plan::{directory_marker_text, top_level_dir};
use super::{BuiltGroups, Cost, Group, SolverResult};

// ---------------------------------------------------------------------------
// Priority queue data structures
// ---------------------------------------------------------------------------

/// Mutable state for the lazy-deletion priority queue.
struct ScheduleQueue {
    /// Monotonic counter; each enqueued/invalidated item gets a unique value.
    generation: u64,
    /// Per-group map from cumulative position to the latest generation. Heap
    /// entries whose generation doesn't match are stale and get skipped.
    current_gen: Vec<HashMap<usize, u64>>,
    heap: BinaryHeap<QueueItem>,
}

impl ScheduleQueue {
    fn new(num_groups: usize) -> Self {
        Self {
            generation: 0,
            current_gen: vec![HashMap::new(); num_groups],
            heap: BinaryHeap::new(),
        }
    }

    fn next_generation(&mut self) -> u64 {
        let g = self.generation;
        self.generation += 1;
        g
    }
}

/// An item in the scheduling priority queue.
#[derive(Debug)]
struct QueueItem {
    group_idx: usize,
    /// Position in the group's cumulative array that this item targets.
    cumulative_pos: usize,
    /// Incremental value from the group's current position to this target
    /// (precomputed from cumulative prefix sums, excludes file path value).
    value: f64,
    /// Incremental cost from the group's current position to this target
    /// (precomputed from cumulative prefix sums, excludes file path cost).
    cost: Cost,
    /// Cost of file paths not yet shown (for files this group touches).
    file_path_cost: Cost,
    /// Generation counter for lazy deletion.
    generation: u64,
}

impl QueueItem {
    fn total_cost(&self) -> Cost {
        self.cost + self.file_path_cost
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for QueueItem {}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Cross-multiplication avoids float division precision loss:
        // self_value / self_cost vs other_value / other_cost
        // becomes self_value * other_cost vs other_value * self_cost
        let lhs = self.value * other.total_cost().tokens as f64;
        let rhs = other.value * self.total_cost().tokens as f64;
        lhs.partial_cmp(&rhs)
            .unwrap_or(std::cmp::Ordering::Equal)
            // Deterministic tiebreaker: lower group_idx, then higher
            // cumulative_pos (prefer more content when ratios are equal).
            .then_with(|| other.group_idx.cmp(&self.group_idx))
            .then_with(|| self.cumulative_pos.cmp(&other.cumulative_pos))
    }
}

// ---------------------------------------------------------------------------
// Cost computation helpers
// ---------------------------------------------------------------------------

/// Compute token and character cost of a file path line, including the
/// separator newline emitted by render_scheduled before each file header.
fn file_path_costs(relative_path: &Path) -> Cost {
    let text = format!("\n{}\n", relative_path.display());
    Cost::of(&text)
}

// ---------------------------------------------------------------------------
// Queue population
// ---------------------------------------------------------------------------

/// Enqueue (or re-enqueue) priority queue items for the given groups.
///
/// For each group, iterates cumulative positions beyond the current inclusion
/// state, computing incremental cost/value via O(1) prefix-sum subtraction.
/// File path costs are still computed dynamically from `files_shown`.
///
/// Skips items that exceed `remaining_budget` (with early break since costs
/// are monotonically non-decreasing with position). Invalidates over-budget
/// items' generation entries so stale heap entries are ignored.
#[allow(clippy::too_many_arguments)]
fn enqueue_group_items(
    group_indices: &[usize],
    groups: &[Group],
    group_positions: &[Option<usize>],
    fp_costs: &[Cost],
    files_shown: &HashSet<usize>,
    remaining_budget: usize,
    remaining_char_budget: Option<usize>,
    queue: &mut ScheduleQueue,
) {
    for &group_idx in group_indices {
        let group = &groups[group_idx];
        let cumulatives = &group.cumulatives;
        let current_pos = group_positions[group_idx];

        let start = current_pos.map(|p| p + 1).unwrap_or(0);

        // File path cost is the same for all positions within this group.
        let fp_cost: Cost = group
            .file_indices
            .iter()
            .filter(|fi| !files_shown.contains(fi))
            .map(|&fi| fp_costs[fi])
            .sum();

        for pos in start..cumulatives.len() {
            let incr_cost = cumulatives.incremental_cost(current_pos, pos);
            let incr_value = cumulatives.incremental_value(current_pos, pos);

            let total = incr_cost + fp_cost;

            // Budget pruning: skip items that can't fit either budget.
            // Costs are monotonically non-decreasing with position, so
            // we can break the loop early.
            let exceeds_budget = total.tokens > remaining_budget
                || remaining_char_budget.is_some_and(|rcb| total.chars > rcb);
            if exceeds_budget {
                for invalidate_pos in pos..cumulatives.len() {
                    let item_gen = queue.next_generation();
                    queue.current_gen[group_idx].insert(invalidate_pos, item_gen);
                }
                break;
            }

            // Skip zero-value items (e.g. Import Names with value 0).
            if incr_value <= 0.0 {
                let item_gen = queue.next_generation();
                queue.current_gen[group_idx].insert(pos, item_gen);
                continue;
            }

            let item_gen = queue.next_generation();
            queue.current_gen[group_idx].insert(pos, item_gen);

            queue.heap.push(QueueItem {
                group_idx,
                cumulative_pos: pos,
                value: incr_value,
                cost: incr_cost,
                file_path_cost: fp_cost,
                generation: item_gen,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Main scheduling algorithm
// ---------------------------------------------------------------------------

/// Run the greedy scheduling algorithm.
///
/// Returns raw solver decisions (per-group inclusion position and shown files).
/// The caller passes these to [`super::plan::build_render_plan`] to construct
/// the concrete render plan with file ordering and directory markers.
pub(super) fn solve(
    built: &BuiltGroups,
    corpus: &Corpus<'_>,
    char_budget: Option<usize>,
) -> SolverResult {
    let groups = &built.groups;
    let budget = built.budget;
    let files = corpus.files;

    // Precompute file path costs (tokens and chars, including separator newline for chars)
    let fp_costs: Vec<Cost> = files
        .iter()
        .map(|f| file_path_costs(&f.info.relative_path))
        .collect();

    // Reverse index: file_idx → set of group indices that reference this file.
    // Used to efficiently find groups affected when a file's path cost is paid.
    let mut file_to_groups: Vec<Vec<usize>> = vec![Vec::new(); files.len()];
    for (group_idx, group) in groups.iter().enumerate() {
        for &fi in &group.file_indices {
            file_to_groups[fi].push(group_idx);
        }
    }

    // Reserve budget for directory omission markers.
    let mut total_marker_cost = Cost::default();
    {
        let mut seen_dirs = HashSet::new();
        for f in files {
            if let Some(top) = top_level_dir(&f.info.relative_path)
                && seen_dirs.insert(top.clone())
            {
                total_marker_cost += Cost::of(&format!("\n{}", directory_marker_text(&top)));
            }
        }
    }

    // Track which files have been "shown" (path cost already paid)
    let mut files_shown: HashSet<usize> = HashSet::new();

    // Track per-group inclusion position in the cumulative array.
    let mut group_positions: Vec<Option<usize>> = vec![None; groups.len()];

    // Build all queue items
    let mut queue = ScheduleQueue::new(groups.len());

    let mut remaining_budget = budget.saturating_sub(total_marker_cost.tokens);
    let mut remaining_char_budget = char_budget.map(|cb| cb.saturating_sub(total_marker_cost.chars));

    let all_indices: Vec<usize> = (0..groups.len()).collect();
    enqueue_group_items(
        &all_indices,
        groups,
        &group_positions,
        &fp_costs,
        &files_shown,
        remaining_budget,
        remaining_char_budget,
        &mut queue,
    );

    while let Some(item) = queue.heap.pop() {
        // Lazy deletion: skip stale items
        let current = queue.current_gen[item.group_idx]
            .get(&item.cumulative_pos);
        match current {
            Some(&g) if g == item.generation => {}
            _ => continue,
        }

        // Skip zero-value items — these shouldn't consume budget.
        if item.value <= 0.0 {
            continue;
        }

        // Skip if this position is already covered by the group's current state.
        if group_positions[item.group_idx]
            .is_some_and(|pos| item.cumulative_pos <= pos)
        {
            continue;
        }

        let total = item.total_cost();
        let exceeds_budget = total.tokens > remaining_budget
            || remaining_char_budget.is_some_and(|rcb| total.chars > rcb);
        if exceeds_budget {
            continue;
        }

        // Include this item and all its prerequisites
        remaining_budget -= total.tokens;
        if let Some(ref mut rcb) = remaining_char_budget {
            *rcb = rcb.saturating_sub(total.chars);
        }

        // Mark file paths as shown, tracking which are newly shown
        let mut newly_shown_files: Vec<usize> = Vec::new();
        for &fi in &groups[item.group_idx].file_indices {
            if files_shown.insert(fi) {
                newly_shown_files.push(fi);
            }
        }

        // Update group state: advance to the accepted position
        group_positions[item.group_idx] = Some(item.cumulative_pos);

        // Re-enqueue affected items. Two kinds of groups need updates:
        // 1. The same group (current position changed → new incremental costs)
        // 2. Groups sharing *newly shown* files (file path costs decreased)
        let mut affected_groups: HashSet<usize> = HashSet::new();
        affected_groups.insert(item.group_idx);
        for &fi in &newly_shown_files {
            for &gi in &file_to_groups[fi] {
                affected_groups.insert(gi);
            }
        }

        let affected: Vec<usize> = affected_groups.into_iter().collect();
        enqueue_group_items(
            &affected,
            groups,
            &group_positions,
            &fp_costs,
            &files_shown,
            remaining_budget,
            remaining_char_budget,
            &mut queue,
        );
    }

    SolverResult {
        group_positions,
        files_shown,
    }
}
