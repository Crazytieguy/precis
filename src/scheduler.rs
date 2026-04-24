//! Scheduler: greedy value/cost picker with a two-tier frontier.
//!
//! The scheduler maintains two pools at any time:
//!
//! - **Speculative candidates** (`self.candidates`): walker-emitted
//!   [`Candidate`]s whose content hasn't been read or parsed yet. Ranked by
//!   `score(signals) / cost_hint` — both of which the walker supplies from
//!   FS-only evidence and must be OPTIMISTIC (signals high, cost low) so
//!   the ratio is a correct upper bound on the post-materialization value.
//!
//! - **Exact batches** (`self.entries`): materialized, with resolved
//!   content + final signals + exact marginal cost against the current
//!   tree. Ranked by `score(signals) / marginal_cost`.
//!
//! Generic over `W: Walker` so the scheduler never names any walker-
//! specific key variant. `W::Key` is an opaque [`WalkerKey`] as far as
//! the scheduler is concerned — it needs identity (`Eq`/`Hash`) and
//! tiebreak order (`Ord`) only.
//!
//! The branch-and-bound loop: peek both tops; if exact-top's ratio ≥
//! speculative-top's upper bound, schedule exact-top (no unmaterialized
//! candidate can beat it). Otherwise materialize the speculative-top and
//! let it join the exact pool. Under prefix-monotone scheduling (see
//! `docs/design-notes.md`), if the top-ratio exact doesn't fit the
//! scheduler stops — no fallback to a smaller batch.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::batch::{Batch, BatchId, WalkerKey};
use crate::content::BatchContent;
use crate::render::{Cost, RenderedTree, SourceCache};
use crate::value::{ratio as score_ratio, score};
use crate::walker::{Candidate, WalkCtx, Walker};

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

/// Scheduler-private record holding a materialized batch + its walker-
/// declared identity. `Batch` itself is key-free; `BatchEntry` carries
/// the scheduler bookkeeping that was previously inlined into `Batch`.
struct BatchEntry<K: WalkerKey> {
    key: K,
    predecessor: Option<K>,
    batch: Batch,
}

pub struct Scheduler<W: Walker> {
    walker: W,
    ctx: WalkCtx,
    tree: RenderedTree,
    token_budget: usize,
    byte_budget: Option<usize>,
    consumed: Cost,

    /// Materialized batches, indexed by [`BatchId`] (which is the position).
    entries: Vec<BatchEntry<W::Key>>,
    /// Stable key→id lookup for predecessor resolution.
    key_to_id: HashMap<W::Key, BatchId>,
    /// Scheduled batches.
    scheduled: HashSet<BatchId>,
    /// Ordered log of scheduled batch ids + costs for the final report.
    scheduled_log: Vec<(BatchId, Cost)>,
    /// Speculative candidates (not yet materialized).
    candidates: HashMap<W::Key, Candidate<W::Key>>,
    /// Keys that failed materialization (`materialize` returned `None`)
    /// and their dependents. Never retried.
    dead: HashSet<W::Key>,
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
            candidates: HashMap::new(),
            dead: HashSet::new(),
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
        for c in self.walker.seed(&self.ctx) {
            self.absorb_candidate(c);
        }

        // Prefix-monotone scheduling: the top-ranked eligible batch (across
        // both exact and speculative pools) is considered at each step. If
        // it's speculative, materialize it (no budget cost). If it's an
        // exact that fits, schedule it. If it's an exact that doesn't fit,
        // stop — do not fall back to a smaller batch. This makes the
        // schedule at T_small a true prefix of T_large's schedule (every
        // decision up to the stopping point at T_small also holds at
        // T_large, and T_large simply continues past it).
        loop {
            let best_exact = self.best_exact();
            let best_spec = self.best_speculative();
            match (best_exact, best_spec) {
                (None, None) => break,
                (Some((id, _, cost)), None) => {
                    if self.fits(cost) {
                        self.schedule(id, cost);
                    } else {
                        break;
                    }
                }
                (None, Some((key, _))) => {
                    let _ = self.materialize(&key);
                }
                (Some((id, ex_ratio, cost)), Some((key, spec_bound))) => {
                    if ex_ratio >= spec_bound {
                        if self.fits(cost) {
                            self.schedule(id, cost);
                        } else {
                            break;
                        }
                    } else {
                        let _ = self.materialize(&key);
                    }
                }
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

        let scheduled = self
            .scheduled_log
            .into_iter()
            .scan(0usize, |cum, (id, cost)| {
                *cum += cost.tokens;
                let entry = &self.entries[id.index()];
                Some(ScheduledBatchRecord {
                    key: entry.key.clone(),
                    content: entry.batch.content.clone(),
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

    fn absorb_candidate(&mut self, c: Candidate<W::Key>) {
        if self.key_to_id.contains_key(&c.key)
            || self.dead.contains(&c.key)
            || self.candidates.contains_key(&c.key)
        {
            return;
        }
        self.candidates.insert(c.key.clone(), c);
    }

    /// Eligibility check: predecessor is scheduled (or no predecessor).
    /// If the predecessor has been materialized but not yet scheduled,
    /// the candidate is not yet eligible.
    fn eligible(&self, pred: Option<&W::Key>) -> bool {
        match pred {
            None => true,
            Some(p) => self
                .key_to_id
                .get(p)
                .is_some_and(|id| self.scheduled.contains(id)),
        }
    }

    /// A candidate is dead by transitivity if its predecessor is in `dead`.
    fn predecessor_dead(&self, pred: Option<&W::Key>) -> bool {
        pred.is_some_and(|p| self.dead.contains(p))
    }

    // ---- speculative pool ----

    fn best_speculative(&self) -> Option<(W::Key, f64)> {
        let mut best: Option<(f64, &W::Key)> = None;
        for (key, c) in &self.candidates {
            if self.predecessor_dead(c.predecessor.as_ref()) {
                continue;
            }
            if !self.eligible(c.predecessor.as_ref()) {
                continue;
            }
            let ratio = upper_bound_ratio(c);
            let better = best.is_none_or(|(br, bk)| ratio > br || (ratio == br && key < bk));
            if better {
                best = Some((ratio, key));
            }
        }
        best.map(|(ratio, k)| (k.clone(), ratio))
    }

    // ---- exact pool ----

    /// Top-ranked eligible exact batch regardless of fit, with its
    /// already-computed cost. Returning `Cost` here lets the main loop do
    /// the fit check and (on schedule) apply without recomputing —
    /// `cost_spans` is the hot path per iteration.
    fn best_exact(&self) -> Option<(BatchId, f64, Cost)> {
        let mut best: Option<(f64, BatchId, &W::Key, Cost)> = None;
        for (idx, entry) in self.entries.iter().enumerate() {
            let id = BatchId::new(idx);
            if self.scheduled.contains(&id) {
                continue;
            }
            if !self.eligible(entry.predecessor.as_ref()) {
                continue;
            }
            let cost = self.tree.marginal_cost(&entry.batch);
            let value = score(&entry.batch.signals);
            let ratio = score_ratio(value, cost.tokens);
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

    // ---- materialization ----

    /// Materialize `key`: remove it from the candidate pool, call the
    /// walker, and either record it as dead or absorb the resolved batch
    /// into the exact pool. Returns the assigned [`BatchId`] on success.
    fn materialize(&mut self, key: &W::Key) -> Option<BatchId> {
        let candidate = self.candidates.remove(key)?;
        let Some(resolved) = self.walker.materialize(key, &self.ctx) else {
            self.dead.insert(key.clone());
            return None;
        };
        let id = BatchId::new(self.entries.len());
        self.entries.push(BatchEntry {
            key: key.clone(),
            predecessor: candidate.predecessor,
            batch: Batch {
                content: resolved.content,
                signals: resolved.signals,
            },
        });
        self.key_to_id.insert(key.clone(), id);
        Some(id)
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
        let batch_clone = self.entries[id.index()].batch.clone();
        let conflicts = self
            .tree
            .apply(&batch_clone, id, |i| ancestors.contains(&i));
        debug_assert!(
            conflicts.is_empty(),
            "walker-emitted batch hit non-ancestor overlap: {:?}",
            conflicts
        );
        self.scheduled.insert(id);
        self.scheduled_log.push((id, cost));
        self.consumed.tokens += cost.tokens;
        self.consumed.bytes += cost.bytes;

        // Walker learns about the new scheduled key; emit successors.
        let key = self.entries[id.index()].key.clone();
        let successors = self.walker.expand(&key, &self.ctx);
        for c in successors {
            self.absorb_candidate(c);
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

fn upper_bound_ratio<K: WalkerKey>(c: &Candidate<K>) -> f64 {
    score_ratio(score(&c.signals), c.cost_hint)
}
