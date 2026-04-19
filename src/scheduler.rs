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
    scheduled: HashSet<BatchId>,
}

impl<W: Walker> Scheduler<W> {
    pub fn new(root: PathBuf, walker: W, token_budget: usize, char_budget: Option<usize>) -> Self {
        Self {
            walker,
            ctx: WalkerCtx::new(root.clone()),
            tree: RenderedTree::new(root),
            token_budget,
            char_budget,
            consumed: Cost::default(),
            all_batches: HashMap::new(),
            scheduled: HashSet::new(),
        }
    }

    pub fn run(mut self) -> RenderedTree {
        let seeds = self.walker.seed(&mut self.ctx);
        for batch in seeds {
            self.all_batches.insert(batch.id, batch);
        }
        while let Some((id, cost)) = self.pick_best() {
            self.schedule(id, cost);
        }

        // End-of-run cross-check against the actually-rendered text. Catches
        // marginal-cost accounting bugs that incremental tracking might miss
        // (BPE non-additivity across rows is monotone in our favor, so this
        // can only fire if some other accounting drift snuck in).
        let final_tokens = self.tree.total_tokens();
        assert!(
            final_tokens <= self.token_budget,
            "rendered output exceeds token budget: {} > {}",
            final_tokens,
            self.token_budget,
        );
        if let Some(cb) = self.char_budget {
            let final_chars = self.tree.total_chars();
            assert!(
                final_chars <= cb,
                "rendered output exceeds char budget: {} > {}",
                final_chars,
                cb,
            );
        }

        self.tree
    }

    fn pick_best(&self) -> Option<(BatchId, Cost)> {
        // Tracks the best (ratio, batch id, cost). Tie-break on lower BatchId
        // so the schedule order is deterministic regardless of HashMap iteration.
        let mut best: Option<(f64, BatchId, Cost)> = None;
        for batch in self.all_batches.values() {
            if self.scheduled.contains(&batch.id) {
                continue;
            }
            if !batch.predecessors().all(|p| self.scheduled.contains(&p)) {
                continue;
            }
            let cost = self.tree.marginal_cost(batch);
            if self.consumed.tokens + cost.tokens > self.token_budget {
                continue;
            }
            if let Some(cb) = self.char_budget
                && self.consumed.chars + cost.chars > cb
            {
                continue;
            }
            let ratio = if cost.tokens == 0 {
                f64::INFINITY
            } else {
                batch.raw_value / cost.tokens as f64
            };
            let better = match best {
                None => true,
                Some((br, bid, _)) => ratio > br || (ratio == br && batch.id < bid),
            };
            if better {
                best = Some((ratio, batch.id, cost));
            }
        }
        best.map(|(_, id, cost)| (id, cost))
    }

    fn schedule(&mut self, id: BatchId, cost: Cost) {
        let batch = self
            .all_batches
            .get(&id)
            .expect("scheduled id present in batches")
            .clone();

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

        debug_assert!(
            !self.scheduled.contains(&id),
            "batch {:?} scheduled twice",
            id
        );
        debug_assert!(
            batch.predecessors().all(|p| self.scheduled.contains(&p)),
            "batch {:?} scheduled before its predecessors",
            id,
        );

        let ancestors = self.ancestors_of(&batch);
        self.tree.apply(&batch, |id| ancestors.contains(&id));
        self.scheduled.insert(id);
        self.consumed.tokens += cost.tokens;
        self.consumed.chars += cost.chars;

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

    /// Walks the full predecessor graph of `batch` (parent + ordering_pred,
    /// transitively) and returns the set of ancestor ids. The visited set
    /// also makes the walk safe against cycles, which a buggy walker could
    /// otherwise introduce. Used by the tree's overlap check.
    fn ancestors_of(&self, batch: &Batch) -> HashSet<BatchId> {
        let mut set = HashSet::new();
        let mut stack: Vec<BatchId> = batch.predecessors().collect();
        while let Some(p) = stack.pop() {
            if !set.insert(p) {
                continue;
            }
            if let Some(b) = self.all_batches.get(&p) {
                stack.extend(b.predecessors());
            }
        }
        set
    }
}
