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

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchId};
use crate::content::BatchContent;
use crate::render::{Cost, RenderedTree, SourceCache};
use crate::value::ratio as score_ratio;
use crate::walker::{WalkCtx, Walker};

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
pub struct RunReport<K> {
    pub tree: RenderedTree,
    pub scheduled: Vec<ScheduledBatchRecord<K>>,
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
    /// `best_exact` on cache miss; selectively cleared by `schedule`
    /// when the just-applied batch mutates render-tree state any other
    /// batch's marginal cost depends on.
    cost_cache: HashMap<BatchId, Cost>,
    /// Reverse index: each path → batches whose marginal cost depends
    /// on render-tree state at that path. Populated at absorb time from
    /// the batch's content (Lines: span paths; Fs: group parents).
    /// Lines and Fs use disjoint cells of the render tree, so a single
    /// `PathBuf`-keyed index suffices — even on a hypothetical
    /// file-vs-dir collision the worst case is over-invalidation.
    path_to_batches: HashMap<PathBuf, Vec<BatchId>>,
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
            path_to_batches: HashMap::new(),
        }
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
        for batch in self.walker.seed(&self.ctx) {
            self.absorb(batch);
        }

        // Prefix-monotone scheduling: rank eligible entries by
        // `value / cost^k` and pick the best-fit; if the top-ranked
        // exact doesn't fit, stop (no fallback to smaller batches). This
        // makes the schedule at `T_small` a true prefix of `T_large`'s.
        loop {
            let Some((id, _, cost)) = self.best_exact() else {
                break;
            };
            if !self.fits(cost) {
                break;
            }
            self.schedule(id, cost);
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

        let scheduled = self
            .scheduled_log
            .into_iter()
            .scan(0usize, |cum, (id, cost)| {
                *cum += cost.tokens;
                let entry = &self.entries[id.index()];
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
        }
    }

    // ---- absorption ----

    /// Add a walker-emitted batch to the pool. Idempotent on key.
    fn absorb(&mut self, batch: Batch<W::Key>) {
        if self.key_to_id.contains_key(&batch.key) {
            return;
        }
        let id = BatchId::new(self.entries.len());
        for path in relevant_paths(&batch.content) {
            self.path_to_batches.entry(path).or_default().push(id);
        }
        self.key_to_id.insert(batch.key.clone(), id);
        self.entries.push(batch);
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
        // Two passes so we can mutate `cost_cache` without holding a
        // borrow into `entries`: first compute any missing costs, then
        // rank using the now-populated cache.
        let verify_hits = cfg!(debug_assertions) && verify_cost_cache_enabled();
        for idx in 0..self.entries.len() {
            let id = BatchId::new(idx);
            if self.scheduled.contains(&id) {
                continue;
            }
            let entry = &self.entries[idx];
            if !self.eligible(entry.predecessor.as_ref()) {
                continue;
            }
            if let Some(&cached) = self.cost_cache.get(&id) {
                if verify_hits {
                    let fresh = self.tree.marginal_cost(&entry.content);
                    debug_assert_eq!(
                        fresh, cached,
                        "stale cost_cache entry for batch {id:?} — invalidation logic missed a dependency",
                    );
                }
            } else {
                let fresh = self.tree.marginal_cost(&entry.content);
                self.cost_cache.insert(id, fresh);
            }
        }

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
            let ratio = score_ratio(entry.value, cost.tokens);
            let better = best
                .as_ref()
                .is_none_or(|(br, _, bk, _)| ratio > *br || (ratio == *br && &entry.key < bk));
            if better {
                best = Some((ratio, id, &entry.key, cost));
            }
        }
        best.map(|(ratio, id, _, cost)| (id, ratio, cost))
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
        let conflicts = self
            .tree
            .apply(&entry_content, id, |i| ancestors.contains(&i));
        debug_assert!(
            conflicts.is_empty(),
            "walker-emitted batch hit non-ancestor overlap: {:?}",
            conflicts
        );
        self.scheduled.insert(id);
        self.scheduled_log.push((id, cost));
        self.consumed.tokens += cost.tokens;
        self.consumed.bytes += cost.bytes;

        // Invalidate cached marginal costs for any other emitted batch
        // whose marginal cost depends on render-tree state at a path the
        // just-applied batch mutated. Also lazily prune already-scheduled
        // ids from the dependents list — they never re-enter `best_exact`,
        // so keeping them inflates the invalidation loop on hot files
        // like `src/lib.rs`.
        self.cost_cache.remove(&id);
        let scheduled = &self.scheduled;
        for path in relevant_paths(&entry_content) {
            if let Some(dependents) = self.path_to_batches.get_mut(&path) {
                dependents.retain(|d| !scheduled.contains(d));
                for dep in dependents {
                    self.cost_cache.remove(dep);
                }
            }
        }

        // Walker learns about the new scheduled key; emit successors.
        let key = self.entries[id.index()].key.clone();
        let successors = self.walker.expand(&key, &self.ctx);
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

/// `PRECIS_VERIFY_COST_CACHE=1` opt-in for the recompute-on-hit
/// assertion in `best_exact`. Read once per process — toggling at
/// runtime isn't supported, but the value of an env var rarely changes
/// inside a test run anyway.
fn verify_cost_cache_enabled() -> bool {
    use std::sync::OnceLock;
    static FLAG: OnceLock<bool> = OnceLock::new();
    *FLAG.get_or_init(|| {
        std::env::var("PRECIS_VERIFY_COST_CACHE")
            .map(|v| matches!(v.as_str(), "1" | "true"))
            .unwrap_or(false)
    })
}

/// Paths whose render-tree state the marginal cost of `content`
/// depends on, used as the invalidation key for `Scheduler::cost_cache`.
///
/// - `Lines`: each unique span path (Lines marginal cost reads
///   `nodes[path].content` to compute refinement deltas).
/// - `Fs`: each unique group parent (Fs marginal cost reads
///   `nodes[parent].children` to skip already-listed entries).
///
/// Cross-variant invalidation is unnecessary: `apply_fs_group` only
/// mutates `Dir.children` (and inserts empty `File`/`Dir` nodes whose
/// content is empty — Lines `existing.get(line) == None` either way),
/// and `apply_spans` only mutates `File.content`. Lines paths and Fs
/// parents thus invalidate disjoint cells of the render tree even
/// when sharing a `PathBuf` (which can't happen in practice — file
/// vs. dir paths differ).
fn relevant_paths(content: &BatchContent) -> Vec<PathBuf> {
    let mut paths: Vec<&Path> = match content {
        BatchContent::Lines { spans } => spans.iter().map(|s| s.path.as_path()).collect(),
        BatchContent::Fs { groups } => groups.iter().map(|g| g.parent.as_path()).collect(),
    };
    paths.sort();
    paths.dedup();
    paths.into_iter().map(PathBuf::from).collect()
}
