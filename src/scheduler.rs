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
//! - **Exact batches** (`self.batches`): materialized, with resolved
//!   content + final signals + exact marginal cost against the current
//!   tree. Ranked by `score(signals) / marginal_cost`.
//!
//! The branch-and-bound loop: peek both tops; if exact-top's ratio ≥
//! speculative-top's upper bound, schedule exact-top (no unmaterialized
//! candidate can beat it). Otherwise materialize the speculative-top and
//! let it join the exact pool. Repeat until neither pool yields a
//! schedulable batch.
//!
//! This gives the lazy-I/O property: files are only read + parsed when a
//! specific [`BatchKey`] earns the right via its FS-only priority. A folder
//! being scheduled never triggers a file read by itself.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::batch::{Batch, BatchId, BatchKey};
use crate::render::{Cost, RenderedTree, SourceCache};
use crate::value::{ratio as score_ratio, score};
use crate::walker::{Candidate, WalkCtx, Walker};

pub struct Scheduler<W: Walker> {
    walker: W,
    ctx: WalkCtx,
    tree: RenderedTree,
    token_budget: usize,
    byte_budget: Option<usize>,
    consumed: Cost,

    /// Materialized batches, indexed by [`BatchId`] (which is the position).
    batches: Vec<Batch>,
    /// Stable key→id lookup for predecessor resolution.
    key_to_id: HashMap<BatchKey, BatchId>,
    /// Scheduled batches.
    scheduled: HashSet<BatchId>,
    /// Speculative candidates (not yet materialized).
    candidates: HashMap<BatchKey, Candidate>,
    /// Keys that failed materialization (`materialize` returned `None`)
    /// and their dependents. Never retried.
    dead: HashSet<BatchKey>,
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
            batches: Vec::new(),
            key_to_id: HashMap::new(),
            scheduled: HashSet::new(),
            candidates: HashMap::new(),
            dead: HashSet::new(),
        }
    }

    pub fn run(mut self) -> RenderedTree {
        for c in self.walker.seed(&self.ctx) {
            self.absorb_candidate(c);
        }

        // Early-stop scheduling: the top-ranked eligible exact batch is
        // considered regardless of fit. If it fits, schedule it. If it
        // doesn't and the speculative pool still has candidates, keep
        // materializing — one of them might resolve to a better-ratio batch
        // that fits (its cost hint is an upper bound, so actual cost can
        // be smaller). Only stop once we've both run out of speculatives
        // and the top exact doesn't fit. This preserves the "best batch
        // must fit else stop" rule while keeping the branch-and-bound
        // invariant from being violated by an oversized-but-high-ratio
        // exact stranding fitting speculatives.
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
                            // Top exact doesn't fit but speculatives exist.
                            // Materialize — a speculative may resolve to
                            // something that fits with a competitive ratio.
                            let _ = self.materialize(&key);
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

        self.tree
    }

    // ---- absorption ----

    fn absorb_candidate(&mut self, c: Candidate) {
        if self.key_to_id.contains_key(&c.key)
            || self.dead.contains(&c.key)
            || self.candidates.contains_key(&c.key)
        {
            return;
        }
        self.candidates.insert(c.key.clone(), c);
    }

    /// Eligibility check: predecessor is scheduled (or no predecessor, or
    /// already in the key→id table but un-scheduled means predecessor is
    /// materialized but not yet picked — not eligible).
    fn eligible(&self, pred: Option<&BatchKey>) -> bool {
        match pred {
            None => true,
            Some(p) => self
                .key_to_id
                .get(p)
                .is_some_and(|id| self.scheduled.contains(id)),
        }
    }

    /// A candidate is dead by transitivity if its predecessor is in `dead`.
    fn predecessor_dead(&self, pred: Option<&BatchKey>) -> bool {
        pred.is_some_and(|p| self.dead.contains(p))
    }

    // ---- speculative pool ----

    fn best_speculative(&self) -> Option<(BatchKey, f64)> {
        let mut best: Option<(f64, &BatchKey)> = None;
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
        let mut best: Option<(f64, BatchId, BatchKey, Cost)> = None;
        for (idx, batch) in self.batches.iter().enumerate() {
            let id = BatchId::new(idx);
            if self.scheduled.contains(&id) {
                continue;
            }
            if !self.eligible(batch.predecessor.as_ref()) {
                continue;
            }
            let cost = self.tree.marginal_cost(batch);
            let value = score(&batch.signals);
            let ratio = score_ratio(value, cost.tokens);
            let better = best
                .as_ref()
                .is_none_or(|(br, _, bk, _)| ratio > *br || (ratio == *br && &batch.key < bk));
            if better {
                best = Some((ratio, id, batch.key.clone(), cost));
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
    fn materialize(&mut self, key: &BatchKey) -> Option<BatchId> {
        let candidate = self.candidates.remove(key)?;
        let Some(resolved) = self.walker.materialize(key, &self.ctx) else {
            self.dead.insert(key.clone());
            return None;
        };
        let id = BatchId::new(self.batches.len());
        self.batches.push(Batch {
            key: key.clone(),
            content: resolved.content,
            predecessor: candidate.predecessor,
            signals: resolved.signals,
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
        let batch_clone = self.batches[id.index()].clone();
        self.tree
            .apply(&batch_clone, id, |i| ancestors.contains(&i));
        self.scheduled.insert(id);
        self.consumed.tokens += cost.tokens;
        self.consumed.bytes += cost.bytes;

        // Walker learns about the new scheduled key; emit successors.
        let key = self.batches[id.index()].key.clone();
        let successors = self.walker.expand(&key, &self.ctx);
        for c in successors {
            self.absorb_candidate(c);
        }
    }

    /// Walk the predecessor chain, resolving keys to ids. Cycles are a
    /// walker bug; debug-assert + break.
    fn ancestors_of(&self, id: BatchId) -> HashSet<BatchId> {
        let mut set = HashSet::new();
        let mut cur = self.batches[id.index()].predecessor.as_ref();
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
            cur = self.batches[pred_id.index()].predecessor.as_ref();
        }
        set
    }
}

fn upper_bound_ratio(c: &Candidate) -> f64 {
    score_ratio(score(&c.signals), c.cost_hint)
}
