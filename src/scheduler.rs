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
//! scheduler stops, no fallback to smaller batches — with one
//! exception at round 0, where stopping means returning nothing at all
//! (`Scheduler::schedule_partial_seed`).
//!
//! Generic over `W: Walker` so the scheduler never names any walker-
//! specific key variant. `W::Key` is an opaque [`WalkerKey`] as far as
//! the scheduler is concerned — it needs identity (`Eq`/`Hash`) and
//! tiebreak order (`Ord`) only.

#[cfg(debug_assertions)]
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchId, WalkerKey};
use crate::content::{BatchContent, FsEntries, FsGroup};
use crate::fs_util::{DirFilter, EntryKind, list_dir};
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
    /// batches that never made it into the prefix-monotone schedule. A
    /// seed degraded by `Scheduler::schedule_partial_seed` appears in
    /// the form it was scheduled in, not the form the walker emitted.
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
    /// Batches drawn entirely from the tree's dominant source file —
    /// they rank at a [`DOMINANT_FILE_RATIO_BOOST`] premium.
    dominant_file_batches: HashSet<BatchId>,
    /// Whether any dominant-file batch has been scheduled yet — the
    /// premium escalates depth into a file already entered rather than
    /// pulling it in front of the repository's orientation.
    dominant_file_entered: bool,
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

/// Ranking premium for content drawn from the tree's dominant source
/// file ([`WalkCtx::dominant_source_file`]). When one file holds most of
/// a repository's implementation, "what is this repo" and "what is in
/// that file" are the same question, and the marginal token buys more
/// inside that file than on another lap of breadth. Gated on the file
/// having already been entered on its own merits, so the premium
/// escalates depth rather than pulling one file in front of the
/// repository's orientation. Swept full-corpus; see
/// `docs/design-notes.md` ("Dominant source file").
const DOMINANT_FILE_RATIO_BOOST: f64 = 1.45;

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
            dominant_file_batches: HashSet::new(),
            dominant_file_entered: false,
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
        // batch that doesn't fit (no fallback to smaller batches),
        // except at round 0 where stopping yields nothing at all.
        {
            crate::time_span!("scheduler_loop");
            while let Some((id, cost)) = self.best_exact() {
                if !self.fits(cost) {
                    self.schedule_partial_seed(id);
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
            let in_dominant_file = self
                .ctx
                .dominant_source_file()
                .is_some_and(|dominant| spans.iter().all(|span| span.path == dominant));
            if in_dominant_file {
                self.dominant_file_batches.insert(id);
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
            let pressure = self.train_pressure(id) * self.dominant_file_boost(id);
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
            let pressure = self.train_pressure(id) * self.dominant_file_boost(id);
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

    /// Apply `id`'s content to the tree and log it as scheduled.
    /// Returns the applied content. Bookkeeping that only pays off in a
    /// *later* round — cost invalidation, train counters, walker
    /// expansion — lives in [`Self::schedule`]; a terminal partial
    /// schedule skips it.
    fn apply_and_record(&mut self, id: BatchId, cost: Cost) -> BatchContent {
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
        self.dominant_file_entered |= self.dominant_file_batches.contains(&id);
        self.scheduled_log.push((id, cost));
        self.consumed.tokens += cost.tokens;
        self.consumed.bytes += cost.bytes;
        entry_content
    }

    /// Terminal degradation for a budget too small to open the walk.
    /// Every batch a walker emits is gated, directly or transitively, on
    /// the seed listing, so a seed that doesn't fit leaves the pool
    /// permanently empty and the caller with an empty string — no
    /// output at all, and no indication why. Schedule the longest
    /// affordable prefix of the seed's entries instead; the renderer
    /// marks the shortened listing `…`, and the budget buys the head of
    /// the repo map rather than nothing.
    ///
    /// Only listings degrade this way: entry rows are independent, so a
    /// prefix of one is a smaller listing, where a prefix of a line
    /// batch is a severed piece of source. Restricting this to the
    /// still-empty schedule keeps the stop-on-first-ill-fit rule (and
    /// the prefix-monotone schedule it buys) intact everywhere else.
    fn schedule_partial_seed(&mut self, id: BatchId) {
        if !self.scheduled_log.is_empty() {
            return;
        }
        let Some((prefix, cost)) = self.affordable_fs_prefix(&self.entries[id.index()].content)
        else {
            return;
        };
        self.entries[id.index()].content = prefix;
        self.apply_and_record(id, cost);
    }

    /// Longest prefix of a listing's entries — taken in
    /// [`rank_seed_entries`] order, not name order — that fits the
    /// remaining budget, with its exact cost. A seed is one directory's
    /// listing; multi-group FS content comes only from NS TOML, which
    /// never reaches the scheduler.
    fn affordable_fs_prefix(&self, content: &BatchContent) -> Option<(BatchContent, Cost)> {
        let BatchContent::Fs { groups } = content else {
            return None;
        };
        let [
            FsGroup {
                parent,
                entries: FsEntries::Listed(paths),
            },
        ] = groups.as_slice()
        else {
            return None;
        };
        let ranked = rank_seed_entries(parent, paths, self.ctx.dir_filter());
        let prefix = |k: usize| {
            // Back to name order: a degraded listing is otherwise an
            // ordinary listing of a subset, and every consumer of batch
            // content expects the sorted form `list_dir` produces.
            let mut kept = ranked[..k].to_vec();
            kept.sort();
            BatchContent::Fs {
                groups: vec![FsGroup {
                    parent: parent.clone(),
                    entries: FsEntries::Listed(kept),
                }],
            }
        };
        // Entry rows are independent, so cost climbs with the prefix
        // length — binary-search the boundary, keeping the longest
        // prefix that fit. Nothing downstream depends on the search
        // finding the exact boundary: whatever it returns was measured.
        let (mut lo, mut hi) = (0usize, ranked.len());
        let mut affordable = None;
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let candidate = prefix(mid);
            let cost = self.tree.marginal_cost(&candidate);
            if self.fits(cost) {
                lo = mid;
                affordable = Some((candidate, cost));
            } else {
                hi = mid;
            }
        }
        affordable
    }

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

        let entry_content = self.apply_and_record(id, cost);
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

    /// Ratio premium for `id` when it draws only on the dominant source
    /// file. Surface batches only: the spine file's roster is what the
    /// scheduler under-buys, while its docs and bodies already rank
    /// locally once the train is open, and boosting those front-loads
    /// one file's depth over the rest of the repository's orientation.
    fn dominant_file_boost(&self, id: BatchId) -> f64 {
        if self.dominant_file_entered
            && self.dominant_file_batches.contains(&id)
            && !self.entries[id.index()].key.is_depth_follow_up()
        {
            DOMINANT_FILE_RATIO_BOOST
        } else {
            1.0
        }
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

/// Order a listing's entries by how much a bare name row tells a reader
/// meeting the repository, most informative first.
///
/// Only [`Scheduler::affordable_fs_prefix`] consults this. A listing
/// that fits renders every one of its rows, in name order, whatever
/// this returns; the ranking decides only *which* entries a budget too
/// small for the whole listing keeps. The order is a function of the
/// listing alone, never of the budget, so a smaller budget's prefix
/// stays a subset of a larger one's.
///
/// Three tiers, each a naming or filesystem convention rather than a
/// list of names, so the rule holds on any repository:
///
/// - **Hidden entries last.** A leading `.` is the filesystem's own
///   "not part of the ordinary view of this directory", and what lives
///   behind it at a repository root is tooling and editor configuration
///   rather than the project.
/// - **Directories before files.** A directory row stands for a whole
///   subtree and is the only kind of row that says how the repository
///   is organized, where a file row names one leaf.
/// - **All-caps documents after other files.** The convention exists so
///   that `README` / `LICENSE` / `CHANGELOG` / `CONTRIBUTING` are
///   recognizable everywhere, which is exactly what makes their names
///   uninformative — a reader assumes they are there. A row naming the
///   manifest, the entrypoint or the build file does not. Within the
///   documents the unqualified name outranks the variants that add a
///   component to it, so `README.md` outranks `README.ja.md`.
///
/// Name order breaks ties, so the result is total and deterministic.
fn rank_seed_entries(parent: &Path, paths: &[PathBuf], filter: &DirFilter) -> Vec<PathBuf> {
    // Re-probe to recover kinds. Uses the walk's own filter rather than
    // an unfiltered listing: its caches are already warm, and it is the
    // filter that decides whether a symlinked entry is a directory at
    // all, so asking anything else here could rank an entry as a file
    // that the listing itself resolved to a directory.
    let kinds = list_dir(parent, filter);
    let mut ranked = paths.to_vec();
    ranked.sort_by_cached_key(|path| {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let hidden = name.starts_with('.');
        let is_file = !matches!(kinds.get(&name), Some(EntryKind::Directory));
        let root_document = is_file && is_root_document_name(&name);
        let doc_variant = if root_document {
            name.matches('.').count()
        } else {
            0
        };
        (hidden, is_file, root_document, doc_variant, name)
    });
    ranked
}

/// Whether `name` follows the all-caps root-document convention —
/// `README.md`, `LICENSE`, `CHANGELOG.md`, `CONTRIBUTING.md`, `AUTHORS`,
/// `COPYING`, and their translated variants.
fn is_root_document_name(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name);
    stem.chars().any(char::is_uppercase) && !stem.chars().any(char::is_lowercase)
}
