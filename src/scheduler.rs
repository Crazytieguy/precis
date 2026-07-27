//! Scheduler: greedy value/cost picker with predecessor-gated eligibility.
//!
//! Walkers emit [`Batch<K>`] units directly (key + optional predecessor +
//! content + scalar value). The scheduler absorbs each into a single
//! pool, gates eligibility on whether the batch's predecessor (if any)
//! is already scheduled, and ranks eligible batches by
//! `value / cost^k`. Each iteration narrows the eligible set to a
//! top-K contender pool by *approximate* cost ([`CONTENDER_POOL_K`]),
//! then picks the best contender by exact cost — a batch whose approx
//! ranking falls outside the pool can be picked a round late, which is
//! an accepted approximation. Under prefix-monotone scheduling (see
//! `docs/design-notes.md`), if the top-ranked exact doesn't fit the
//! scheduler stops, no fallback to smaller batches.
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

/// `best_exact` narrows the eligible set to the top-K by approx score
/// before exact tokenization. Corpus boundary is K=66 (htop); 2× margin
/// for off-corpus inputs.
const CONTENDER_POOL_K: usize = 128;

/// A single scheduled batch, captured in order for snapshots and the
/// divergence metric. Generic over the walker's key type.
#[derive(Debug, Clone)]
pub struct ScheduledBatchRecord<K> {
    pub key: K,
    pub content: BatchContent,
    pub cost: Cost,
    pub cum_tokens: usize,
}

/// Scheduler output: rendered tree + ordered log of scheduled batches.
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
    /// Cached exact marginal cost per emitted batch.
    cost_cache: HashMap<BatchId, Cost>,
    /// Cached approx token count per emitted batch (for approx ranking).
    approx_cost_cache: HashMap<BatchId, usize>,
    /// File → span batches touching it. Applying a batch to a file can
    /// change the synthesized `…` marker delta of every other batch
    /// touching that file, so their cached costs are dropped when one
    /// of them is scheduled.
    batches_by_path: HashMap<PathBuf, Vec<BatchId>>,
    /// Memoized transitive predecessor root per batch (the head of its
    /// gated train). Resolved at ranking time, when eligibility
    /// guarantees the chain is fully materialized.
    train_root_memo: HashMap<BatchId, BatchId>,
    /// Non-zero-cost batches scheduled per train root — drives the
    /// breadth-pressure ratio penalty.
    scheduled_per_root: HashMap<BatchId, usize>,
    /// Pool members per train root, counted at absorb time (predecessor
    /// chains materialize in emission order, so absorb-time resolution
    /// is complete for all but pathological cross-expansion chains).
    train_member_counts: HashMap<BatchId, usize>,
    /// Unopened substantial trains (>= TRAIN_SUBSTANTIAL_MEMBERS pool
    /// members, no non-zero-cost schedule yet, non-orientation root) —
    /// breadth pressure is pointless (and measured harmful:
    /// mitt/go-multierror) when there is nothing to redirect the
    /// budget to. Maintained incrementally: absorb adds a root when
    /// its member count crosses the threshold; the first non-zero-cost
    /// schedule under a root removes it.
    substantial_unopened: HashSet<BatchId>,
    /// Dependents absorbed before their predecessor key materialized —
    /// their subtree counts sit under a pseudo-root until the missing
    /// key arrives, then merge (predecessor keys are symbolic, so
    /// emission order is not guaranteed).
    pending_reparent: HashMap<W::Key, Vec<BatchId>>,
    /// Debug-only owner map for FS render cells — overlapping sibling
    /// FS atoms are a walker-contract violation.
    #[cfg(debug_assertions)]
    fs_atom_owners: BTreeMap<(PathBuf, String), W::Key>,
}

/// Breadth-pressure coefficient: past the free allowance, a candidate
/// whose train root already has `n` scheduled non-zero-cost batches
/// ranks at `1/(1 + K*(n - FREE))` of its raw ratio. NS authors
/// schedule breadth-first — every file's surface before any file's
/// depth — while cheap follow-up batches (bodies, docs, members)
/// otherwise out-ratio unopened siblings' surfaces and drive long
/// same-train dives. Restricting the penalty to depth follow-ups is what
/// keeps orientation trains (README headline -> outline -> sections)
/// unpenalized — NS authors sequence those deep by design, and the
/// blanket variant measured -0.004/-0.006.
const TRAIN_PRESSURE_K: f64 = 0.15;
/// Scheduled batches a train may accumulate before pressure applies —
/// normal decl -> doc -> body depth is wanted; 20-batch dives are not.
const TRAIN_PRESSURE_FREE: usize = 4;
/// Pool members for a train to count as substantial breadth.
const TRAIN_SUBSTANTIAL_MEMBERS: usize = 3;
/// Minimum unopened substantial trains for pressure to apply at all —
/// in a small repo whose primary train IS the content, demoting its
/// follow-ups just buys worse batches.
const BREADTH_MIN_TRAINS: usize = 2;

impl<W: Walker> Scheduler<W> {
    pub fn new(root: PathBuf, walker: W, token_budget: usize, byte_budget: Option<usize>) -> Self {
        Self::with_source_cache(root, walker, token_budget, byte_budget, SourceCache::new())
    }

    /// Scheduler sharing an externally-owned `SourceCache` (tests).
    pub fn with_source_cache(
        root: PathBuf,
        walker: W,
        token_budget: usize,
        byte_budget: Option<usize>,
        source_cache: SourceCache,
    ) -> Self {
        let ctx = WalkCtx::with_cache(root.clone(), source_cache.clone());
        let tree = RenderedTree::with_filter(root, source_cache, ctx.dir_filter_handle());
        Self {
            walker,
            ctx,
            tree,
            token_budget,
            byte_budget,
            consumed: Cost::default(),
            entries: Vec::new(),
            key_to_id: HashMap::new(),
            scheduled: HashSet::new(),
            scheduled_log: Vec::new(),
            cost_cache: HashMap::new(),
            approx_cost_cache: HashMap::new(),
            batches_by_path: HashMap::new(),
            train_root_memo: HashMap::new(),
            scheduled_per_root: HashMap::new(),
            train_member_counts: HashMap::new(),
            substantial_unopened: HashSet::new(),
            pending_reparent: HashMap::new(),
            #[cfg(debug_assertions)]
            fs_atom_owners: BTreeMap::new(),
        }
    }

    /// Run the scheduler and return just the rendered tree.
    pub fn run(self) -> RenderedTree {
        self.run_with_report().tree
    }

    /// Run the scheduler and return the tree plus the scheduled-batches log.
    pub fn run_with_report(mut self) -> RunReport<W::Key> {
        crate::time_span!("run_with_report");
        {
            crate::time_span!("walker_seed");
            for batch in self.walker.seed(&self.ctx) {
                self.absorb(batch);
            }
        }

        // Prefix-monotone scheduling — stop on the first top-ranked
        // batch that doesn't fit (no fallback to smaller batches).
        {
            crate::time_span!("scheduler_loop");
            while let Some((id, cost)) = self.best_exact() {
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

        self.key_to_id.insert(batch.key.clone(), id);

        if let BatchContent::Lines { spans } = &batch.content {
            let mut last: Option<&PathBuf> = None;
            for span in spans {
                // Spans arrive grouped by file; dedup consecutively.
                if last != Some(&span.path) {
                    self.batches_by_path
                        .entry(span.path.clone())
                        .or_default()
                        .push(id);
                    last = Some(&span.path);
                }
            }
        }

        // Non-memoized root walk for the member count (the ranking-time
        // memo must only be written once chains are guaranteed
        // complete). A missing predecessor key parks the count under
        // the last resolved id as a pseudo-root and registers it for
        // reparenting when the key materializes.
        let mut root = id;
        let mut cur = self.entries.len(); // guard against cycles
        let mut probe = &batch.predecessor;
        while let Some(pred_key) = probe {
            let Some(&pred_id) = self.key_to_id.get(pred_key) else {
                self.pending_reparent
                    .entry(pred_key.clone())
                    .or_default()
                    .push(root);
                break;
            };
            root = pred_id;
            probe = &self.entries[pred_id.index()].predecessor;
            cur -= 1;
            if cur == 0 {
                break;
            }
        }
        self.bump_train_members(root, 1);

        self.entries.push(batch);

        // The new key may be the missing predecessor of earlier
        // pseudo-roots: merge their parked subtree counts into this
        // batch's own (possibly still pseudo) root.
        let new_key = self.entries[id.index()].key.clone();
        if let Some(orphans) = self.pending_reparent.remove(&new_key) {
            for pseudo in orphans {
                if pseudo == root {
                    continue;
                }
                let parked = self.train_member_counts.remove(&pseudo).unwrap_or(0);
                self.substantial_unopened.remove(&pseudo);
                if let Some(opened) = self.scheduled_per_root.remove(&pseudo) {
                    *self.scheduled_per_root.entry(root).or_insert(0) += opened;
                    self.substantial_unopened.remove(&root);
                }
                self.bump_train_members(root, parked);
            }
        }
    }

    /// Add `n` members to `root`'s train, promoting it into the
    /// substantial-unopened set when it crosses the threshold.
    fn bump_train_members(&mut self, root: BatchId, n: usize) {
        if n == 0 {
            return;
        }
        let members = self.train_member_counts.entry(root).or_insert(0);
        let before = *members;
        *members += n;
        if before < TRAIN_SUBSTANTIAL_MEMBERS
            && *members >= TRAIN_SUBSTANTIAL_MEMBERS
            && !self.scheduled_per_root.contains_key(&root)
            && !self.entries[root.index()].key.is_orientation()
        {
            self.substantial_unopened.insert(root);
        }
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

    /// Eligibility — predecessor scheduled, or no predecessor.
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

    /// Top-ranked eligible batch + its cost.
    fn best_exact(&mut self) -> Option<(BatchId, Cost)> {
        crate::time_counter!(best_exact);

        let eligible: Vec<BatchId> = (0..self.entries.len())
            .map(BatchId::new)
            .filter(|id| {
                !self.scheduled.contains(id)
                    && self.eligible(self.entries[id.index()].predecessor.as_ref())
            })
            .collect();
        if eligible.is_empty() {
            return None;
        }

        let pool = self.select_contender_pool(&eligible);

        crate::time_counter!(best_exact_rank_pass);
        let mut best: Option<(f64, BatchId, Cost)> = None;
        for &id in &pool {
            if !self.cost_cache.contains_key(&id) {
                let content = &self.entries[id.index()].content;
                let c = self.tree.marginal_cost(content);
                self.cost_cache.insert(id, c);
            }
            let pressure = self.train_pressure(id);
            let exact_cost = self.cost_cache[&id];
            let entry = &self.entries[id.index()];
            let ratio = score_ratio(
                entry.value,
                exact_cost.tokens,
                entry.key.concavity_exponent(),
            ) * crate::value::prose_mass_tier_multiplier(
                self.consumed.tokens,
                self.token_budget,
                entry.key.is_deferred_mass_prose(),
            ) * pressure;
            let better = best.as_ref().is_none_or(|(br, b_id, _)| {
                ratio > *br
                    || (ratio == *br
                        && self.entries[id.index()].key < self.entries[b_id.index()].key)
            });
            if better {
                best = Some((ratio, id, exact_cost));
            }
        }
        best.map(|(_, id, cost)| (id, cost))
    }

    /// Narrow `eligible` to the top-K contender pool by approx score
    /// for the exact pass to rerank. Skips the approx pass when the
    /// eligible set already fits in the pool.
    fn select_contender_pool(&mut self, eligible: &[BatchId]) -> Vec<BatchId> {
        if eligible.len() <= CONTENDER_POOL_K {
            return eligible.to_vec();
        }

        {
            crate::time_counter!(best_exact_cost_pass);
            for &id in eligible {
                if !self.approx_cost_cache.contains_key(&id) {
                    let content = &self.entries[id.index()].content;
                    let tokens = self.tree.marginal_cost_approx(content);
                    self.approx_cost_cache.insert(id, tokens);
                }
            }
        }

        let mut candidates: Vec<(f64, BatchId)> = Vec::with_capacity(eligible.len());
        for &id in eligible {
            let pressure = self.train_pressure(id);
            let approx_tokens = self.approx_cost_cache[&id];
            let entry = &self.entries[id.index()];
            let ratio = score_ratio(entry.value, approx_tokens, entry.key.concavity_exponent())
                * crate::value::prose_mass_tier_multiplier(
                    self.consumed.tokens,
                    self.token_budget,
                    entry.key.is_deferred_mass_prose(),
                )
                * pressure;
            candidates.push((ratio, id));
        }

        let k = CONTENDER_POOL_K.min(candidates.len()).max(1);
        let pivot = candidates.len() - k;
        // Partition so positions [pivot, len) hold the k largest
        // (in arbitrary order — the exact pass re-ranks).
        let entries = &self.entries;
        candidates.select_nth_unstable_by(pivot, |a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| entries[b.1.index()].key.cmp(&entries[a.1.index()].key))
        });
        candidates[pivot..].iter().map(|(_, id)| *id).collect()
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
        // Zero-cost batches (already line-covered by an ancestor) don't
        // consume budget, so they don't count toward train pressure.
        if cost.tokens > 0 {
            let root = self.train_root(id);
            *self.scheduled_per_root.entry(root).or_insert(0) += 1;
            self.substantial_unopened.remove(&root);
        }

        // Drop the scheduled batch's cached cost, plus every cached
        // cost for batches touching the same files — the apply may
        // have changed their synthesized-marker deltas. Costs of
        // batches on untouched files stay stable under walker
        // invariants (line-disjoint outside predecessor chains).
        self.cost_cache.remove(&id);
        if let BatchContent::Lines { spans } = &entry_content {
            let mut last: Option<&PathBuf> = None;
            for span in spans {
                if last == Some(&span.path) {
                    continue;
                }
                last = Some(&span.path);
                if let Some(ids) = self.batches_by_path.get(&span.path) {
                    for other in ids {
                        self.cost_cache.remove(other);
                        self.approx_cost_cache.remove(other);
                    }
                }
                // Prune the now-scheduled id from this path's list so
                // future invalidations stop iterating dead ids. Done
                // after the cache-drop above so the current schedule's
                // own invalidation still sees every sibling on the path.
                if let Some(ids) = self.batches_by_path.get_mut(&span.path) {
                    ids.retain(|&other| other != id);
                    if ids.is_empty() {
                        self.batches_by_path.remove(&span.path);
                    }
                }
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

    /// Walk the predecessor chain. Cycles are a walker bug (debug-assert).
    /// Transitive predecessor root of `id` — the head of its gated
    /// train. Memoized; safe to resolve at ranking time because an
    /// eligible batch's chain is fully materialized.
    fn train_root(&mut self, id: BatchId) -> BatchId {
        if let Some(&root) = self.train_root_memo.get(&id) {
            return root;
        }
        let pred_id = self.entries[id.index()]
            .predecessor
            .as_ref()
            .and_then(|pred_key| self.key_to_id.get(pred_key).copied());
        let root = match pred_id {
            Some(pred_id) => self.train_root(pred_id),
            None => id,
        };
        self.train_root_memo.insert(id, root);
        root
    }

    /// Breadth-pressure multiplier for `id`'s ratio. Applies only to
    /// depth follow-up batches (doc/body/member refinements) — surface
    /// batches always rank at their raw ratio.
    fn train_pressure(&mut self, id: BatchId) -> f64 {
        if self.substantial_unopened.len() < BREADTH_MIN_TRAINS {
            return 1.0;
        }
        if !self.entries[id.index()].key.is_depth_follow_up() {
            return 1.0;
        }
        let root = self.train_root(id);
        let n = self.scheduled_per_root.get(&root).copied().unwrap_or(0);
        let over = n.saturating_sub(TRAIN_PRESSURE_FREE);
        1.0 / (1.0 + TRAIN_PRESSURE_K * over as f64)
    }

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
