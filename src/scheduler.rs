use std::collections::HashSet;
use std::path::PathBuf;

use crate::batch::{Batch, BatchDraft, BatchId};
use crate::render::{Cost, RenderedTree};
use crate::walker::{Walker, WalkerCtx};

pub struct Scheduler<W: Walker> {
    walker: W,
    ctx: WalkerCtx,
    tree: RenderedTree,
    token_budget: usize,
    byte_budget: Option<usize>,
    consumed: Cost,
    batches: Vec<Batch>,
    scheduled: Vec<bool>,
}

impl<W: Walker> Scheduler<W> {
    pub fn new(root: PathBuf, walker: W, token_budget: usize, byte_budget: Option<usize>) -> Self {
        Self {
            walker,
            ctx: WalkerCtx::new(root.clone()),
            tree: RenderedTree::new(root),
            token_budget,
            byte_budget,
            consumed: Cost::default(),
            batches: Vec::new(),
            scheduled: Vec::new(),
        }
    }

    pub fn run(mut self) -> RenderedTree {
        let seeds = self.walker.seed(&self.ctx);
        for d in seeds {
            self.absorb_draft(d, None);
        }
        while let Some((id, cost)) = self.pick_best() {
            self.schedule(id, cost);
            // Walker emits successors of the just-scheduled batch. Collect
            // first so the borrow on self.walker / self.batches / self.ctx
            // ends before we mutate self.batches via absorb_draft.
            let successors = self
                .walker
                .successors(id, &self.batches[id.index()], &self.ctx);
            for d in successors {
                self.absorb_draft(d, Some(id));
            }
        }

        // End-of-run cross-check (debug-only; in release we'd rather emit a
        // possibly-out-of-budget output than panic). Bind once — total_tokens
        // re-renders the whole tree, so re-evaluating it inside the assert
        // message would render twice.
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

    /// Allocate a new id and store the batch. `predecessor` is None for seeds,
    /// `Some(scheduled_id)` for successors — set by the scheduler, not the
    /// walker, so a walker cannot attach the wrong parent.
    fn absorb_draft(&mut self, draft: BatchDraft, predecessor: Option<BatchId>) {
        self.batches.push(Batch {
            content: draft.content,
            predecessor,
            value: draft.value,
        });
        self.scheduled.push(false);
    }

    fn pick_best(&self) -> Option<(BatchId, Cost)> {
        // Tracks (ratio, idx, cost). Tie-break on lower idx for determinism.
        let mut best: Option<(f64, usize, Cost)> = None;
        for (idx, batch) in self.batches.iter().enumerate() {
            if self.scheduled[idx] {
                continue;
            }
            if let Some(p) = batch.predecessor
                && !self.scheduled[p.index()]
            {
                continue;
            }
            let cost = self.tree.marginal_cost(batch);
            if self.consumed.tokens + cost.tokens > self.token_budget {
                continue;
            }
            if let Some(bb) = self.byte_budget
                && self.consumed.bytes + cost.bytes > bb
            {
                continue;
            }
            let ratio = if cost.tokens == 0 {
                f64::INFINITY
            } else {
                batch.value / cost.tokens as f64
            };
            let better = match best {
                None => true,
                Some((br, bidx, _)) => ratio > br || (ratio == br && idx < bidx),
            };
            if better {
                best = Some((ratio, idx, cost));
            }
        }
        best.map(|(_, idx, cost)| (BatchId::new(idx), cost))
    }

    fn schedule(&mut self, id: BatchId, cost: Cost) {
        debug_assert!(
            !self.scheduled[id.index()],
            "batch {:?} scheduled twice",
            id
        );
        debug_assert!(
            self.batches[id.index()]
                .predecessor
                .is_none_or(|p| self.scheduled[p.index()]),
            "batch {:?} scheduled before its predecessor",
            id,
        );
        debug_assert!(
            self.consumed.tokens + cost.tokens <= self.token_budget,
            "scheduled batch exceeds token budget"
        );

        let ancestors = self.ancestors_of(id);
        self.tree
            .apply(&self.batches[id.index()], id, |i| ancestors.contains(&i));
        self.scheduled[id.index()] = true;
        self.consumed.tokens += cost.tokens;
        self.consumed.bytes += cost.bytes;
    }

    /// Walks the predecessor chain of `id` (transitively). Cycles are
    /// impossible by construction (walkers can't forge ids; predecessors
    /// always reference an already-allocated batch), so no visited-set guard.
    fn ancestors_of(&self, id: BatchId) -> HashSet<BatchId> {
        let mut set = HashSet::new();
        let mut cur = self.batches[id.index()].predecessor;
        while let Some(p) = cur {
            set.insert(p);
            cur = self.batches[p.index()].predecessor;
        }
        set
    }
}
