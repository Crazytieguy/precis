use std::collections::HashSet;
use std::path::PathBuf;

use crate::batch::{Batch, BatchId};
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
        let seeds = self.walker.seed(&mut self.ctx);
        for batch in seeds {
            self.absorb_batch(batch);
        }
        while let Some((id, cost)) = self.pick_best() {
            self.schedule(id, cost);
        }

        // End-of-run cross-check (debug-only, per design philosophy: in
        // release we'd rather emit a possibly-over-budget output than panic).
        debug_assert!(
            self.tree.total_tokens() <= self.token_budget,
            "rendered output exceeds token budget: {} > {}",
            self.tree.total_tokens(),
            self.token_budget,
        );
        if let Some(bb) = self.byte_budget {
            debug_assert!(
                self.tree.total_bytes() <= bb,
                "rendered output exceeds byte budget: {} > {}",
                self.tree.total_bytes(),
                bb,
            );
        }

        self.tree
    }

    fn absorb_batch(&mut self, batch: Batch) {
        debug_assert_eq!(
            batch.id.0,
            self.batches.len(),
            "walker emitted batch id {} out of order; expected {}",
            batch.id.0,
            self.batches.len(),
        );
        self.batches.push(batch);
        self.scheduled.push(false);
    }

    fn pick_best(&self) -> Option<(BatchId, Cost)> {
        // Tracks (ratio, id, cost). Tie-break on lower BatchId for determinism
        // independent of iteration order (here it's already deterministic
        // since batches is a Vec, but the rule keeps semantics stable across
        // future refactors).
        let mut best: Option<(f64, BatchId, Cost)> = None;
        for (idx, batch) in self.batches.iter().enumerate() {
            if self.scheduled[idx] {
                continue;
            }
            if let Some(p) = batch.predecessor
                && !self.scheduled[p.0]
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
                Some((br, bid, _)) => ratio > br || (ratio == br && batch.id < bid),
            };
            if better {
                best = Some((ratio, batch.id, cost));
            }
        }
        best.map(|(_, id, cost)| (id, cost))
    }

    fn schedule(&mut self, id: BatchId, cost: Cost) {
        debug_assert!(!self.scheduled[id.0], "batch {:?} scheduled twice", id);
        debug_assert!(
            self.batches[id.0]
                .predecessor
                .is_none_or(|p| self.scheduled[p.0]),
            "batch {:?} scheduled before its predecessor",
            id,
        );
        debug_assert!(
            self.consumed.tokens + cost.tokens <= self.token_budget,
            "scheduled batch exceeds token budget"
        );

        let ancestors = self.ancestors_of(id);
        // Need to satisfy borrow checker: clone the batch out so we can pass
        // &self.tree mutably below. The clone is the one place this matters;
        // a longer-term refactor would split self into independently-borrowed
        // pieces, but that's more code than the win warrants right now.
        let batch = self.batches[id.0].clone();
        self.tree.apply(&batch, |id| ancestors.contains(&id));
        self.scheduled[id.0] = true;
        self.consumed.tokens += cost.tokens;
        self.consumed.bytes += cost.bytes;

        let succs = self.walker.successors(&batch, &mut self.ctx);
        for s in succs {
            self.absorb_batch(s);
        }
    }

    /// Walks the predecessor chain of batches[id] (transitively) and returns
    /// the set of ancestor ids. The visited set makes the walk safe against
    /// cycles a buggy walker could introduce.
    fn ancestors_of(&self, id: BatchId) -> HashSet<BatchId> {
        let mut set = HashSet::new();
        let mut cur = self.batches[id.0].predecessor;
        while let Some(p) = cur {
            if !set.insert(p) {
                break;
            }
            cur = self.batches[p.0].predecessor;
        }
        set
    }
}
