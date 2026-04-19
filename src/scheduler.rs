use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::batch::{Batch, BatchId};
use crate::render::{Cost, RenderedTree};
use crate::walker::{Walker, WalkerCtx};

pub struct Scheduler<W: Walker> {
    walker: W,
    ctx: WalkerCtx,
    tree: RenderedTree,
    token_budget: usize,
    char_budget: Option<usize>,
    consumed: Cost,
    all_batches: HashMap<BatchId, Batch>,
    scheduled: HashMap<BatchId, ()>,
}

impl<W: Walker> Scheduler<W> {
    pub fn new(root: PathBuf, walker: W, token_budget: usize, char_budget: Option<usize>) -> Self {
        Self {
            walker,
            ctx: WalkerCtx::new(),
            tree: RenderedTree::new(root),
            token_budget,
            char_budget,
            consumed: Cost::default(),
            all_batches: HashMap::new(),
            scheduled: HashMap::new(),
        }
    }

    pub fn run(mut self) -> RenderedTree {
        // Seed
        let seeds = self.walker.seed(self.tree.root(), &mut self.ctx);
        for batch in seeds {
            self.all_batches.insert(batch.id, batch);
        }

        // Greedy loop: pick best ready batch that fits, schedule it, repeat.
        while let Some(id) = self.pick_best() {
            self.schedule(id);
        }

        self.tree
    }

    fn pick_best(&self) -> Option<BatchId> {
        let mut best: Option<(BatchId, f64)> = None;
        for batch in self.all_batches.values() {
            if self.scheduled.contains_key(&batch.id) {
                continue;
            }
            // Predecessors all scheduled?
            if !batch
                .predecessors()
                .all(|p| self.scheduled.contains_key(&p))
            {
                continue;
            }
            // Marginal cost
            let cost = self.tree.marginal_cost(batch);
            // Fits?
            if self.consumed.tokens + cost.tokens > self.token_budget {
                continue;
            }
            if let Some(cb) = self.char_budget
                && self.consumed.chars + cost.chars > cb
            {
                continue;
            }
            // Ratio (free batches always picked; well-defined as INFINITY)
            let ratio = if cost.tokens == 0 {
                f64::INFINITY
            } else {
                batch.raw_value / cost.tokens as f64
            };
            if best.is_none_or(|(_, br)| ratio > br) {
                best = Some((batch.id, ratio));
            }
        }
        best.map(|(id, _)| id)
    }

    fn schedule(&mut self, id: BatchId) {
        let batch = self
            .all_batches
            .get(&id)
            .expect("scheduled id present in batches")
            .clone();

        // Release-enforced invariants protecting output correctness.
        let cost = self.tree.marginal_cost(&batch);
        assert!(
            self.consumed.tokens + cost.tokens <= self.token_budget,
            "scheduled batch exceeds token budget: {} + {} > {}",
            self.consumed.tokens,
            cost.tokens,
            self.token_budget,
        );
        if let Some(cb) = self.char_budget {
            assert!(
                self.consumed.chars + cost.chars <= cb,
                "scheduled batch exceeds char budget: {} + {} > {}",
                self.consumed.chars,
                cost.chars,
                cb,
            );
        }

        // Debug-only consistency checks.
        debug_assert!(
            !self.scheduled.contains_key(&id),
            "batch {:?} scheduled twice",
            id,
        );
        debug_assert!(
            batch
                .predecessors()
                .all(|p| self.scheduled.contains_key(&p)),
            "batch {:?} scheduled before its predecessors",
            id,
        );

        let ancestors = self.ancestors_of(&batch);
        self.tree.apply(&batch, |id| ancestors.contains(&id));
        self.scheduled.insert(id, ());
        self.consumed.tokens += cost.tokens;
        self.consumed.chars += cost.chars;

        // Walker emits successors of this scheduled batch.
        let succs = self.walker.successors(&batch, &mut self.ctx);
        for s in succs {
            debug_assert!(
                !self.all_batches.contains_key(&s.id),
                "successor {:?} duplicates an existing batch id",
                s.id,
            );
            self.all_batches.insert(s.id, s);
        }
    }

    /// Walks the parent chain of `batch` and collects all ancestor ids
    /// (excluding `batch` itself). Used by the tree's overlap check.
    fn ancestors_of(&self, batch: &Batch) -> HashSet<BatchId> {
        let mut set = HashSet::new();
        let mut cur = batch.parent;
        while let Some(p) = cur {
            if !set.insert(p) {
                break;
            }
            cur = self.all_batches.get(&p).and_then(|b| b.parent);
        }
        set
    }
}
