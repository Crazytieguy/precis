//! Scheduler: greedy value/cost picker with predecessor-gated eligibility.
//!
//! Walkers emit [`Batch<K>`] units directly (key + optional predecessor +
//! content + scalar value). The scheduler absorbs each into a single
//! pool, gates eligibility on whether the batch's predecessor (if any)
//! is already scheduled, and ranks eligible batches by
//! `value / cost^k`. Each iteration picks the best eligible exact
//! batch — under prefix-monotone scheduling
//! (see `docs/design-notes.md`), if the top-ranked exact doesn't fit
//! the scheduler stops, no fallback to smaller batches.
//!
//! Generic over `W: Walker` so the scheduler never names any walker-
//! specific key variant. `W::Key` is an opaque [`WalkerKey`] as far as
//! the scheduler is concerned — it needs identity (`Eq`/`Hash`) and
//! tiebreak order (`Ord`) only.

#[cfg(debug_assertions)]
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::batch::{Batch, BatchId, WalkerKey};
use crate::content::BatchContent;
#[cfg(debug_assertions)]
use crate::content::FsEntries;
use crate::render::{Cost, RenderedTree, SourceCache};
use crate::value::ratio_with_exponent as score_ratio;
use crate::walker::{WalkCtx, Walker};

/// How much recursively-gated descendant value can flow back to a
/// predecessor. The cap keeps wide API surfaces from overwhelming the
/// whole schedule, while still letting a small gate reflect that it
/// unlocks several valuable precise descendants.
const GATED_DESCENDANT_BONUS_MAX_PARENT_MULTIPLE_PER_CHILD: f64 = 0.15;
const GATED_DESCENDANT_BONUS_MAX_PARENT_MULTIPLE_CAP: f64 = 3.0;
/// Only fan-out gates get a scheduling boost. One-off predecessor edges
/// like `Export -> ExportBody` are refinements, not broad unlock points,
/// and boosting every such edge spends budget too aggressively.
const GATED_DESCENDANT_MIN_DIRECT_CHILDREN: usize = 4;
/// Raw descendant value is normalized by this multiple of the parent's
/// own value before applying the saturating curve.
const GATED_DESCENDANT_BONUS_SATURATION_PARENT_MULTIPLE: f64 = 2.0;
/// Grandchildren matter, but less than direct children: the immediate
/// gate must land before any child can compete.
const GATED_DESCENDANT_DEPTH_DECAY: f64 = 0.5;

type ChildrenByParent = HashMap<BatchId, Vec<BatchId>>;

/// A single scheduled batch, captured in order for downstream consumers
/// (schedule snapshots, divergence metric). Generic over the walker's
/// key type; callers that don't want to carry the generic can post-process
/// into a `String`-keyed form (see `lib.rs::render_schedule`).
#[derive(Debug, Clone)]
pub struct ScheduledBatchRecord<K> {
    pub key: K,
    pub content: BatchContent,
    pub cost: Cost,
    pub cum_tokens: usize,
}

/// Everything the scheduler produced: the rendered tree plus the ordered
/// log of scheduled batches. `render()` can be called on the tree; the
/// log drives `render_schedule()` and the divergence metric.
#[derive(Debug)]
pub struct RunReport<K: WalkerKey> {
    pub tree: RenderedTree,
    pub scheduled: Vec<ScheduledBatchRecord<K>>,
    /// All walker-emitted batches discovered during this run, including
    /// batches that never made it into the prefix-monotone schedule.
    pub candidates: Vec<Batch<K>>,
}

pub struct Scheduler<W: Walker> {
    walker: W,
    ctx: WalkCtx,
    tree: RenderedTree,
    token_budget: usize,
    byte_budget: Option<usize>,
    consumed: Cost,

    /// Walker-emitted batches, indexed by [`BatchId`] (which is the position).
    entries: Vec<Batch<W::Key>>,
    /// Stable key→id lookup for predecessor resolution + dedup.
    key_to_id: HashMap<W::Key, BatchId>,
    /// Scheduled batches.
    scheduled: HashSet<BatchId>,
    /// Ordered log of scheduled batch ids + costs for the final report.
    scheduled_log: Vec<(BatchId, Cost)>,
    /// Cached marginal cost per emitted batch. Populated lazily by
    /// `best_exact` on cache miss. Walker invariants keep cached costs
    /// stable: non-ancestor line overlap is rejected during apply, and
    /// debug builds reject overlapping FS atoms at absorb time.
    cost_cache: HashMap<BatchId, Cost>,
    /// Parent → unscheduled children index, maintained incrementally at
    /// `absorb`/`schedule`. `best_exact` rebuilt this from scratch on
    /// every iteration before; that rebuild was the dominant O(N²) cost
    /// on medium-large fixtures.
    ///
    /// Order within each `Vec` matches `entries` order. That ordering
    /// is observable: `raw_gated_descendant_value` iterates this list
    /// and accumulates `f64`, and non-associativity could otherwise
    /// shift ranking ties. Append on absorb, `retain` on schedule.
    children_index: ChildrenByParent,
    /// Children whose predecessor key hasn't been absorbed yet.
    /// Drained into `children_index` when the matching parent is
    /// absorbed. Mirrors the current `key_to_id.get(pred)` miss
    /// behavior in the old rebuild — orphans that never see a parent
    /// stay inert here forever, which is fine (they're also inert in
    /// eligibility today).
    pending_children: HashMap<W::Key, Vec<BatchId>>,
    /// Debug-only verifier toggle. When on, `best_exact` recomputes the
    /// parent→children index from scratch each call and asserts it
    /// matches `children_index`. Off by default; tests turn it on via
    /// [`Self::enable_children_index_verifier`]. Process-local (no env
    /// var) so concurrent libtest threads can't race.
    #[cfg(debug_assertions)]
    verify_children_index: bool,
    /// Debug-only owner map for FS render cells. The production walker
    /// emits one full listing per directory, so overlapping sibling FS
    /// atoms are a walker-contract violation rather than a scheduler case.
    #[cfg(debug_assertions)]
    fs_atom_owners: BTreeMap<(PathBuf, String), W::Key>,
}

impl<W: Walker> Scheduler<W> {
    pub fn new(root: PathBuf, walker: W, token_budget: usize, byte_budget: Option<usize>) -> Self {
        Self::with_source_cache(root, walker, token_budget, byte_budget, SourceCache::new())
    }

    /// Construct a scheduler sharing an externally-owned `SourceCache`. Used
    /// by tests that preload synthetic source content so the render pipeline
    /// can materialize spans against paths that don't exist on disk.
    pub fn with_source_cache(
        root: PathBuf,
        walker: W,
        token_budget: usize,
        byte_budget: Option<usize>,
        source_cache: SourceCache,
    ) -> Self {
        Self {
            walker,
            ctx: WalkCtx::with_cache(root.clone(), source_cache.clone()),
            tree: RenderedTree::new(root, source_cache),
            token_budget,
            byte_budget,
            consumed: Cost::default(),
            entries: Vec::new(),
            key_to_id: HashMap::new(),
            scheduled: HashSet::new(),
            scheduled_log: Vec::new(),
            cost_cache: HashMap::new(),
            children_index: HashMap::new(),
            pending_children: HashMap::new(),
            #[cfg(debug_assertions)]
            verify_children_index: false,
            #[cfg(debug_assertions)]
            fs_atom_owners: BTreeMap::new(),
        }
    }

    /// Turn on the debug-only `children_index` verifier (recompute and
    /// assert on every `best_exact` call). For invariant tests; not
    /// meant for production paths.
    #[cfg(debug_assertions)]
    pub fn enable_children_index_verifier(&mut self) {
        self.verify_children_index = true;
    }

    /// Run the scheduler and return just the rendered tree. Back-compat
    /// entry point used by `precis::render`; new consumers should prefer
    /// [`run_with_report`](Self::run_with_report).
    pub fn run(self) -> RenderedTree {
        self.run_with_report().tree
    }

    /// Run the scheduler and return the tree plus the ordered log of
    /// scheduled batches. Used by `render_schedule` and the divergence
    /// test.
    pub fn run_with_report(mut self) -> RunReport<W::Key> {
        crate::time_span!("run_with_report");
        {
            crate::time_span!("walker_seed");
            for batch in self.walker.seed(&self.ctx) {
                self.absorb(batch);
            }
        }

        // Prefix-monotone scheduling: rank eligible entries by
        // `value / cost^k` and pick the best-fit; if the top-ranked
        // exact doesn't fit, stop (no fallback to smaller batches). This
        // makes the schedule at `T_small` a true prefix of `T_large`'s.
        {
            crate::time_span!("scheduler_loop");
            loop {
                let Some((id, _, cost)) = self.best_exact() else {
                    break;
                };
                if !self.fits(cost) {
                    break;
                }
                self.schedule(id, cost);
            }
        }

        if cfg!(debug_assertions) {
            let total_tokens = self.tree.total_tokens();
            debug_assert!(
                total_tokens <= self.token_budget,
                "rendered output exceeds token budget: {} > {}",
                total_tokens,
                self.token_budget,
            );
            if let Some(bb) = self.byte_budget {
                let total_bytes = self.tree.total_bytes();
                debug_assert!(
                    total_bytes <= bb,
                    "rendered output exceeds byte budget: {} > {}",
                    total_bytes,
                    bb,
                );
            }
        }

        let entries = self.entries;
        let scheduled = self
            .scheduled_log
            .into_iter()
            .scan(0usize, |cum, (id, cost)| {
                *cum += cost.tokens;
                let entry = &entries[id.index()];
                Some(ScheduledBatchRecord {
                    key: entry.key.clone(),
                    content: entry.content.clone(),
                    cost,
                    cum_tokens: *cum,
                })
            })
            .collect();
        RunReport {
            tree: self.tree,
            scheduled,
            candidates: entries,
        }
    }

    // ---- absorption ----

    /// Add a walker-emitted batch to the pool. Idempotent on key.
    fn absorb(&mut self, batch: Batch<W::Key>) {
        if self.key_to_id.contains_key(&batch.key) {
            return;
        }
        #[cfg(debug_assertions)]
        self.assert_disjoint_fs_atoms(&batch);
        let id = BatchId::new(self.entries.len());

        if let Some(pred) = batch.predecessor.as_ref() {
            match self.key_to_id.get(pred) {
                Some(&parent_id) => {
                    self.children_index.entry(parent_id).or_default().push(id);
                }
                None => {
                    self.pending_children
                        .entry(pred.clone())
                        .or_default()
                        .push(id);
                }
            }
        }

        self.key_to_id.insert(batch.key.clone(), id);

        if let Some(waiting) = self.pending_children.remove(&batch.key) {
            // The new id has no prior entry in `children_index` (it was
            // just minted), so direct insert is safe. Pending list is
            // FIFO over absorb order → entries-order is preserved.
            self.children_index.insert(id, waiting);
        }

        self.entries.push(batch);
    }

    #[cfg(debug_assertions)]
    fn assert_disjoint_fs_atoms(&mut self, batch: &Batch<W::Key>) {
        let BatchContent::Fs { groups } = &batch.content else {
            return;
        };
        for group in groups {
            let FsEntries::Listed(paths) = &group.entries else {
                continue;
            };
            for path in paths {
                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };
                let atom = (group.parent.clone(), name.to_string());
                if let Some(owner) = self.fs_atom_owners.get(&atom) {
                    debug_assert_eq!(
                        owner, &batch.key,
                        "overlapping FS atom {:?} emitted by {:?} and {:?}",
                        atom, owner, batch.key,
                    );
                } else {
                    self.fs_atom_owners.insert(atom, batch.key.clone());
                }
            }
        }
    }

    /// Eligibility check: predecessor is scheduled (or no predecessor).
    /// Orphan dependents (predecessor never emitted) stay pending forever
    /// — that's intentional; walkers are responsible for not declaring a
    /// predecessor they aren't going to emit.
    fn eligible(&self, pred: Option<&W::Key>) -> bool {
        match pred {
            None => true,
            Some(p) => self
                .key_to_id
                .get(p)
                .is_some_and(|id| self.scheduled.contains(id)),
        }
    }

    // ---- exact pool ----

    /// Top-ranked eligible batch regardless of fit, with its
    /// already-computed cost. Returning `Cost` here lets the main loop do
    /// the fit check and (on schedule) apply without recomputing —
    /// `cost_spans` is the hot path per iteration.
    ///
    /// Set `PRECIS_VERIFY_COST_CACHE=1` (debug builds only) to recompute
    /// on every cache hit and `debug_assert_eq!` against the cached
    /// value, turning any missed-invalidation bug into a deterministic
    /// panic. Off by default because the recompute roughly 2× the
    /// debug-test runtime; targeted regression coverage lives in
    /// `tests/scheduler_invariants.rs::scheduler_invariants_fs_overlap_invalidates_cached_cost`,
    /// which sets the env var so it always exercises the gate.
    ///
    /// `&mut self` because the cache may be populated mid-call. The body
    /// must mutate only `cost_cache`; no scheduler state transitions
    /// (`scheduled`/`scheduled_log`/`tree.apply`) belong here.
    fn best_exact(&mut self) -> Option<(BatchId, f64, Cost)> {
        crate::time_counter!(best_exact);
        // Two passes so we can mutate `cost_cache` without holding a
        // borrow into `entries`: first compute any missing costs, then
        // rank using the now-populated cache.
        {
            crate::time_counter!(best_exact_cost_pass);
            for idx in 0..self.entries.len() {
                let id = BatchId::new(idx);
                if self.scheduled.contains(&id) {
                    continue;
                }
                let entry = &self.entries[idx];
                if !self.eligible(entry.predecessor.as_ref()) {
                    continue;
                }
                if !self.cost_cache.contains_key(&id) {
                    let fresh = self.tree.marginal_cost(&entry.content);
                    self.cost_cache.insert(id, fresh);
                }
            }
        }

        #[cfg(debug_assertions)]
        if self.verify_children_index {
            let rebuilt = self.children_by_parent_rebuilt();
            debug_assert_eq!(
                rebuilt, self.children_index,
                "children_index out of sync with rebuild"
            );
        }
        let children_by_parent = &self.children_index;
        crate::time_counter!(best_exact_rank_pass);
        let mut descendant_value_cache = HashMap::new();
        let mut best: Option<(f64, BatchId, &W::Key, Cost)> = None;
        for (idx, entry) in self.entries.iter().enumerate() {
            let id = BatchId::new(idx);
            if self.scheduled.contains(&id) {
                continue;
            }
            if !self.eligible(entry.predecessor.as_ref()) {
                continue;
            }
            let cost = self.cost_cache[&id];
            let effective_value =
                self.effective_value(id, children_by_parent, &mut descendant_value_cache);
            let ratio = score_ratio(effective_value, cost.tokens, entry.key.concavity_exponent());
            let better = best
                .as_ref()
                .is_none_or(|(br, _, bk, _)| ratio > *br || (ratio == *br && &entry.key < bk));
            if better {
                best = Some((ratio, id, &entry.key, cost));
            }
        }
        best.map(|(ratio, id, _, cost)| (id, ratio, cost))
    }

    fn effective_value(
        &self,
        id: BatchId,
        children_by_parent: &ChildrenByParent,
        descendant_value_cache: &mut HashMap<BatchId, f64>,
    ) -> f64 {
        let base = self.entries[id.index()].value;
        if base <= 0.0 {
            return base;
        }
        let key_weight = self.entries[id.index()].key.gated_descendant_value_weight();
        if key_weight <= 0.0 {
            return base;
        }
        let direct_child_count = self.direct_unscheduled_child_count(id, children_by_parent);
        if direct_child_count < GATED_DESCENDANT_MIN_DIRECT_CHILDREN {
            return base;
        }
        let raw_descendant_value = self.raw_gated_descendant_value(
            id,
            children_by_parent,
            descendant_value_cache,
            &mut HashSet::new(),
        );
        if raw_descendant_value <= 0.0 {
            return base;
        }
        let max_multiple = (direct_child_count as f64
            * GATED_DESCENDANT_BONUS_MAX_PARENT_MULTIPLE_PER_CHILD)
            .min(GATED_DESCENDANT_BONUS_MAX_PARENT_MULTIPLE_CAP);
        let max_bonus = base * max_multiple * key_weight;
        let saturation =
            (base * GATED_DESCENDANT_BONUS_SATURATION_PARENT_MULTIPLE).max(f64::EPSILON);
        let bonus = max_bonus * (1.0 - (-raw_descendant_value / saturation).exp());
        base + bonus
    }

    /// Reference implementation: rebuild the parent→children index by
    /// scanning entries. Used only by the `PRECIS_VERIFY_CHILDREN_INDEX`
    /// debug verifier; the hot path reads `self.children_index` directly.
    #[cfg(debug_assertions)]
    fn children_by_parent_rebuilt(&self) -> ChildrenByParent {
        let mut children: ChildrenByParent = HashMap::new();
        for (idx, entry) in self.entries.iter().enumerate() {
            let child_id = BatchId::new(idx);
            if self.scheduled.contains(&child_id) {
                continue;
            }
            let Some(pred) = entry.predecessor.as_ref() else {
                continue;
            };
            let Some(parent_id) = self.key_to_id.get(pred) else {
                continue;
            };
            children.entry(*parent_id).or_default().push(child_id);
        }
        children
    }

    fn direct_unscheduled_child_count(
        &self,
        id: BatchId,
        children_by_parent: &ChildrenByParent,
    ) -> usize {
        children_by_parent.get(&id).map_or(0, Vec::len)
    }

    fn raw_gated_descendant_value(
        &self,
        id: BatchId,
        children_by_parent: &ChildrenByParent,
        cache: &mut HashMap<BatchId, f64>,
        visiting: &mut HashSet<BatchId>,
    ) -> f64 {
        if let Some(value) = cache.get(&id) {
            return *value;
        }
        if !visiting.insert(id) {
            if cfg!(debug_assertions) {
                panic!(
                    "predecessor cycle detected while scoring gated descendants at {:?}",
                    self.entries[id.index()].key
                );
            }
            return 0.0;
        }

        let mut total = 0.0;
        for &child_id in children_by_parent.get(&id).into_iter().flatten() {
            let entry = &self.entries[child_id.index()];
            let descendants =
                self.raw_gated_descendant_value(child_id, children_by_parent, cache, visiting);
            total += entry.value + GATED_DESCENDANT_DEPTH_DECAY * descendants;
        }

        visiting.remove(&id);
        cache.insert(id, total);
        total
    }

    fn fits(&self, cost: Cost) -> bool {
        if self.consumed.tokens + cost.tokens > self.token_budget {
            return false;
        }
        if let Some(bb) = self.byte_budget
            && self.consumed.bytes + cost.bytes > bb
        {
            return false;
        }
        true
    }

    // ---- scheduling ----

    fn schedule(&mut self, id: BatchId, cost: Cost) {
        debug_assert!(
            !self.scheduled.contains(&id),
            "batch {:?} scheduled twice",
            id
        );
        debug_assert!(
            self.fits(cost),
            "scheduled batch exceeds token/byte budget; caller should have filtered it"
        );

        let ancestors = self.ancestors_of(id);
        let entry_content = self.entries[id.index()].content.clone();
        let conflicts = {
            crate::time_counter!(schedule_apply);
            self.tree
                .apply(&entry_content, id, |i| ancestors.contains(&i))
        };
        if cfg!(debug_assertions) && !conflicts.is_empty() {
            let current_key = &self.entries[id.index()].key;
            let conflict_details = conflicts
                .iter()
                .map(|conflict| {
                    let owner_key = &self.entries[conflict.existing_owner.index()].key;
                    format!("{conflict:?} owned by {owner_key:?}")
                })
                .collect::<Vec<_>>();
            panic!(
                "walker-emitted batch hit non-ancestor overlap for {current_key:?}: {conflict_details:?}"
            );
        }
        self.scheduled.insert(id);
        self.scheduled_log.push((id, cost));
        self.consumed.tokens += cost.tokens;
        self.consumed.bytes += cost.bytes;

        // Drop the scheduled batch's cached cost. Other cached costs stay
        // stable under walker invariants: siblings don't mutate each
        // other's render cells, and ancestor refinements are scheduled
        // before a dependent first becomes eligible/cached.
        self.cost_cache.remove(&id);

        // Remove this id from its parent's child list. `retain` preserves
        // the order of remaining elements (entries-order matters — see
        // `children_index` field doc).
        let parent_id = self.entries[id.index()]
            .predecessor
            .as_ref()
            .and_then(|p| self.key_to_id.get(p).copied());
        if let Some(parent_id) = parent_id
            && let Some(siblings) = self.children_index.get_mut(&parent_id)
        {
            siblings.retain(|&c| c != id);
            if siblings.is_empty() {
                self.children_index.remove(&parent_id);
            }
        }

        // Walker learns about the new scheduled key; emit successors.
        let key = self.entries[id.index()].key.clone();
        let successors = {
            crate::time_counter!(schedule_expand);
            self.walker.expand(&key, &self.ctx)
        };
        for batch in successors {
            self.absorb(batch);
        }
    }

    /// Walk the predecessor chain, resolving keys to ids. Cycles are a
    /// walker bug; debug-assert + break.
    fn ancestors_of(&self, id: BatchId) -> HashSet<BatchId> {
        let mut set = HashSet::new();
        let mut cur = self.entries[id.index()].predecessor.as_ref();
        while let Some(pred_key) = cur {
            let Some(pred_id) = self.key_to_id.get(pred_key) else {
                break;
            };
            if !set.insert(*pred_id) {
                if cfg!(debug_assertions) {
                    panic!("predecessor cycle detected at {pred_key:?}");
                }
                break;
            }
            cur = self.entries[pred_id.index()].predecessor.as_ref();
        }
        set
    }
}
