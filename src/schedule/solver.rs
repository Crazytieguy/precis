use std::collections::{BinaryHeap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::value::compute_value;
use super::{
    BuiltGroups, Cost, Group, IncludedStage, Schedule, StageKind, SymbolCosts,
};

// ---------------------------------------------------------------------------
// Priority queue data structures
// ---------------------------------------------------------------------------

/// Mutable state for the lazy-deletion priority queue.
struct ScheduleQueue {
    /// Monotonic counter; each enqueued/invalidated item gets a unique value.
    generation: u64,
    /// Per-group map from (stage_kind, n) to the latest generation. Heap
    /// entries whose generation doesn't match are stale and get skipped.
    current_gen: Vec<HashMap<(StageKind, usize), u64>>,
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
    stage_kind: StageKind,
    /// For Doc/Body: which line number (1-indexed) this item represents.
    /// For Names/Signatures: always 1.
    n: usize,
    own_value: f64,
    own_cost: Cost,
    /// Sum of values from unmet prerequisite stages.
    prereq_value: f64,
    prereq_cost: Cost,
    /// Cost of file paths not yet shown (for files this group touches).
    file_path_cost: Cost,
    /// Generation counter for lazy deletion.
    generation: u64,
}

impl QueueItem {
    fn total_value(&self) -> f64 {
        self.own_value + self.prereq_value
    }

    fn total_cost(&self) -> Cost {
        self.own_cost + self.prereq_cost + self.file_path_cost
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
        let lhs = self.total_value() * other.total_cost().tokens as f64;
        let rhs = other.total_value() * self.total_cost().tokens as f64;
        lhs.partial_cmp(&rhs)
            .unwrap_or(std::cmp::Ordering::Equal)
            // Deterministic tiebreaker: lower group_idx, then stage_kind, then n
            .then_with(|| other.group_idx.cmp(&self.group_idx))
            .then_with(|| other.stage_kind.cmp(&self.stage_kind))
            .then_with(|| other.n.cmp(&self.n))
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

/// Compute the token cost of a single stage for a group.
///
/// For Doc and Body stages, includes the cost delta of standalone truncation
/// markers (`→…`, 1 token each). At Doc/Body(n), symbols with more than n lines
/// get a truncation marker. Advancing from n-1 to n removes markers for symbols
/// that had exactly n-1 remaining lines, so the delta is:
///   markers_at(n) - markers_at(n-1)
/// which is non-positive for n >= 2. The telescoping sum across prerequisites
/// ensures the total cost correctly reflects markers at the final included level.
fn stage_cost(group: &Group, stage: StageKind, n: usize) -> Cost {
    match stage {
        // FilePath has zero own_cost — its cost is handled via file_path_costs
        // on the QueueItem, which correctly accounts for cross-group sharing.
        StageKind::FilePath => Cost::default(),
        StageKind::Names => group.symbols.iter().map(|s| s.name).sum(),
        StageKind::Signatures => group.symbols.iter().map(|s| s.signature).sum(),
        StageKind::Doc => {
            // Suppress truncation marker at the pre-comment/docstring split
            // point. The renderer emits these as two sections separated by
            // the signature; at n == pre_doc_count the pre-comments are fully
            // shown (no marker) and the docstring hasn't started (no marker).
            line_stage_cost(group, n, |s| &s.doc_lines, |s| &s.doc_markers, |s, n| {
                let total = s.doc_lines.len();
                !(total > s.pre_doc_count && n == s.pre_doc_count)
            })
        }
        // Body truncation markers are suppressed for symbols with nested
        // children (e.g., class bodies containing individually-rendered
        // methods). Only count markers for symbols without nested children.
        StageKind::Body => {
            line_stage_cost(group, n, |s| &s.body_lines, |s| &s.body_markers, |s, _n| !s.body_has_nested)
        }
    }
}

/// Shared cost computation for line-based stages (Doc/Body).
///
/// `get_lines` selects which line-cost vector to read from each symbol.
/// `get_markers` selects the parallel marker-cost vector.
/// `show_marker` determines whether a symbol shows a truncation marker at
/// a given line index `n`. For Body, this suppresses markers for symbols
/// with nested children. For Doc, this suppresses the marker at the
/// pre-comment/docstring boundary (see `pre_doc_count`).
fn line_stage_cost(
    group: &Group,
    n: usize,
    get_lines: fn(&SymbolCosts) -> &[Cost],
    get_markers: fn(&SymbolCosts) -> &[Cost],
    show_marker: fn(&SymbolCosts, usize) -> bool,
) -> Cost {
    let line_cost: Cost = group
        .symbols
        .iter()
        .filter_map(|s| get_lines(s).get(n - 1).copied())
        .sum();
    let markers_at_n: Cost = group
        .symbols
        .iter()
        .filter(|s| get_lines(s).len() > n && show_marker(s, n))
        .filter_map(|s| get_markers(s).get(n - 1).copied())
        .sum();
    let markers_at_prev: Cost = if n >= 2 {
        group
            .symbols
            .iter()
            .filter(|s| get_lines(s).len() > (n - 1) && show_marker(s, n - 1))
            .filter_map(|s| get_markers(s).get(n - 2).copied())
            .sum()
    } else {
        Cost::default()
    };
    (line_cost + markers_at_n) - markers_at_prev
}

/// Compute prerequisite costs for an item, accounting for already-included stages.
/// Pass `None` for `included` at initial queue construction (no stages included yet).
fn compute_prereq_costs(
    group: &Group,
    stages: &[StageKind],
    stage_pos: usize,
    stage_kind: StageKind,
    n: usize,
    included: Option<&IncludedStage>,
) -> (f64, Cost) {
    let mut prereq_value: f64 = 0.0;
    let mut prereq_cost = Cost::default();

    let included_pos = included
        .and_then(|inc| stages.iter().position(|&s| s == inc.kind));
    let included_n = included.map(|inc| inc.n_lines).unwrap_or(0);

    for (pos, &sk) in stages.iter().enumerate() {
        if pos >= stage_pos && sk == stage_kind {
            // Include earlier lines of the same stage that aren't yet included
            let already_included_n = if included_pos == Some(pos) {
                included_n
            } else if included_pos.is_some_and(|ip| ip > pos) {
                // This entire stage was included as a prereq of a later stage
                group.max_n(sk)
            } else {
                0
            };
            for earlier_n in (already_included_n + 1)..n {
                prereq_value += compute_value(group, sk, earlier_n);
                prereq_cost += stage_cost(group, sk, earlier_n);
            }
            break;
        }

        if included_pos.is_some_and(|ip| ip > pos) {
            continue; // Fully included as a prereq of a later stage
        }

        // If this stage is the current included stage, only pay for
        // lines beyond what's already included
        let start_n = if included_pos == Some(pos) {
            included_n + 1
        } else {
            1
        };

        for line_n in start_n..=group.max_n(sk) {
            prereq_value += compute_value(group, sk, line_n);
            prereq_cost += stage_cost(group, sk, line_n);
        }
    }

    (prereq_value, prereq_cost)
}

// ---------------------------------------------------------------------------
// Queue population
// ---------------------------------------------------------------------------

/// Enqueue (or re-enqueue) priority queue items for the given groups.
///
/// Computes values, costs, and prerequisite state for each (group, stage, n)
/// triple. Skips items already included (based on `group_stages`) and items
/// that exceed `remaining_budget`. For over-budget items, invalidates their
/// generation entries so stale heap entries are ignored.
///
/// Used for both initial queue construction (with empty state) and updates
/// after an item is accepted (with partial state for affected groups only).
#[allow(clippy::too_many_arguments)]
fn enqueue_group_items(
    group_indices: &[usize],
    groups: &[Group],
    group_stages: &[Option<IncludedStage>],
    fp_costs: &[Cost],
    files_shown: &HashSet<usize>,
    remaining_budget: usize,
    remaining_char_budget: Option<usize>,
    queue: &mut ScheduleQueue,
) {
    for &group_idx in group_indices {
        let group = &groups[group_idx];
        let stages = group.key.kind_category.stage_sequence();
        for (stage_pos, &stage_kind) in stages.iter().enumerate() {
            let max_n = group.max_n(stage_kind);
            if max_n == 0 {
                continue;
            }
            for n in 1..=max_n {
                // Skip items already included
                if group_stages[group_idx]
                    .as_ref()
                    .is_some_and(|inc| inc.covers(stages, stage_kind, n))
                {
                    continue;
                }

                let own_value = compute_value(group, stage_kind, n);
                let own_cost = stage_cost(group, stage_kind, n);
                let (prereq_value, prereq_cost) = compute_prereq_costs(
                    group,
                    stages,
                    stage_pos,
                    stage_kind,
                    n,
                    group_stages[group_idx].as_ref(),
                );
                let fp_cost: Cost = group
                    .file_indices
                    .iter()
                    .filter(|fi| !files_shown.contains(fi))
                    .map(|&fi| fp_costs[fi])
                    .sum();

                let total = own_cost + prereq_cost + fp_cost;

                // Budget pruning: skip items that can't fit either budget.
                // Invalidate their generation so stale heap entries are ignored.
                // For Doc/Body, total_cost is monotonically non-decreasing with n,
                // so we can break the inner loop early.
                let exceeds_budget = total.tokens > remaining_budget
                    || remaining_char_budget.is_some_and(|rcb| total.chars > rcb);
                if exceeds_budget {
                    for invalidate_n in n..=max_n {
                        let item_gen = queue.next_generation();
                        queue.current_gen[group_idx].insert((stage_kind, invalidate_n), item_gen);
                    }
                    break;
                }

                let item_gen = queue.next_generation();
                queue.current_gen[group_idx].insert((stage_kind, n), item_gen);

                queue.heap.push(QueueItem {
                    group_idx,
                    stage_kind,
                    n,
                    own_value,
                    own_cost,
                    prereq_value,
                    prereq_cost,
                    file_path_cost: fp_cost,
                    generation: item_gen,
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Main scheduling algorithm
// ---------------------------------------------------------------------------

/// Run the greedy scheduling algorithm.
pub fn schedule(
    built: &BuiltGroups,
    root: &Path,
    files: &[PathBuf],
    char_budget: Option<usize>,
) -> Schedule {
    let groups = &built.groups;
    let budget = built.budget;
    // Build reverse lookup: (file_idx, symbol_idx) → group_idx
    let mut symbol_to_group: HashMap<(usize, usize), usize> = HashMap::new();
    for (group_idx, group) in groups.iter().enumerate() {
        for sc in &group.symbols {
            symbol_to_group.insert((sc.file_idx, sc.symbol_idx), group_idx);
        }
    }

    // Precompute file path costs (tokens and chars, including separator newline for chars)
    let fp_costs: Vec<Cost> = files
        .iter()
        .map(|f| {
            let relative = f.strip_prefix(root).unwrap_or(f);
            file_path_costs(relative)
        })
        .collect();

    // Reverse index: file_idx → set of group indices that reference this file.
    // Used to efficiently find groups affected when a file's path cost is paid.
    let mut file_to_groups: Vec<Vec<usize>> = vec![Vec::new(); files.len()];
    for (group_idx, group) in groups.iter().enumerate() {
        for &fi in &group.file_indices {
            file_to_groups[fi].push(group_idx);
        }
    }

    // Reserve budget for directory omission markers. Each top-level directory
    // that ends up with no visible files gets a `dirname/\n` marker in the
    // output. We conservatively reserve for all possible markers upfront —
    // directories that become visible won't need markers, but the reserved
    // budget stays consumed rather than being released (avoids cascading
    // re-enqueue overhead from budget fluctuations).
    let mut total_marker_cost = Cost::default();
    {
        let mut seen_dirs = HashSet::new();
        for file in files {
            let relative = file.strip_prefix(root).unwrap_or(file);
            let mut components = relative.components();
            if let Some(first) = components.next()
                && components.next().is_some()
            {
                let top = PathBuf::from(first.as_os_str());
                if seen_dirs.insert(top) {
                    total_marker_cost += Cost::of(&format!("\n{}/\n", first.as_os_str().to_string_lossy()));
                }
            }
        }
    }

    // Track which files have been "shown" (path cost already paid)
    let mut files_shown: HashSet<usize> = HashSet::new();

    // Track which (group, stage, n) items have been included
    // For each group: the highest included stage and line count
    let mut group_stages: Vec<Option<IncludedStage>> = vec![None; groups.len()];

    // Build all queue items
    let mut queue = ScheduleQueue::new(groups.len());

    let mut remaining_budget = budget.saturating_sub(total_marker_cost.tokens);
    let mut remaining_char_budget = char_budget.map(|cb| cb.saturating_sub(total_marker_cost.chars));

    let all_indices: Vec<usize> = (0..groups.len()).collect();
    enqueue_group_items(
        &all_indices,
        groups,
        &group_stages,
        &fp_costs,
        &files_shown,
        remaining_budget,
        remaining_char_budget,
        &mut queue,
    );

    while let Some(item) = queue.heap.pop() {
        // Lazy deletion: skip stale items
        let current = queue.current_gen[item.group_idx]
            .get(&(item.stage_kind, item.n));
        match current {
            Some(&g) if g == item.generation => {}
            _ => continue,
        }

        // Skip zero-value items — these shouldn't consume budget.
        if item.total_value() <= 0.0 {
            continue;
        }

        // Skip if this item's stage is already covered by the group's current
        // included stage. This prevents double-deduction when a high-level item
        // (e.g. Doc(3)) is included before a lower-level item (e.g. Names):
        // the high-level item pays for Names as a prerequisite, but the original
        // Names heap entry still has a valid generation number. Without this
        // check, popping that stale Names entry would deduct its cost again.
        if group_stages[item.group_idx]
            .as_ref()
            .is_some_and(|inc| {
                let stages = groups[item.group_idx].key.kind_category.stage_sequence();
                inc.covers(stages, item.stage_kind, item.n)
            })
        {
            continue;
        }

        let total = item.total_cost();
        let exceeds_budget = total.tokens > remaining_budget
            || remaining_char_budget.is_some_and(|rcb| total.chars > rcb);
        if exceeds_budget {
            // This item doesn't fit. Try the next one.
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

        // Update group stage: include all prerequisites up to this item
        let group = &groups[item.group_idx];
        let stages = group.key.kind_category.stage_sequence();
        // Find the position of this stage in the progression
        let stage_pos = stages.iter().position(|&s| s == item.stage_kind).unwrap();
        for (pos, &sk) in stages.iter().enumerate() {
            let target_n = if sk == item.stage_kind {
                item.n
            } else if pos < stage_pos {
                // Prerequisite stage: include all its lines
                group.max_n(sk)
            } else {
                continue;
            };
            if target_n == 0 {
                continue;
            }
            let should_update = group_stages[item.group_idx]
                .as_ref()
                .is_none_or(|cs| !cs.covers(stages, sk, target_n));
            if should_update {
                group_stages[item.group_idx] = Some(IncludedStage {
                    kind: sk,
                    n_lines: target_n,
                });
            }
        }

        // Update affected items in the queue. Only two kinds of groups need updates:
        // 1. The same group (prerequisite costs changed)
        // 2. Groups sharing *newly shown* files (file path costs just decreased)
        // We use file_to_groups to find case 2 efficiently instead of scanning all groups.
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
            &group_stages,
            &fp_costs,
            &files_shown,
            remaining_budget,
            remaining_char_budget,
            &mut queue,
        );
    }

    Schedule {
        group_stages,
        visible_files: files_shown,
        symbol_to_group,
    }
}
