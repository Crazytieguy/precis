//! Scheduler: greedy value/cost picker with predecessor-gated eligibility.
//!
//! Walkers emit [`Batch`] units directly (key + optional predecessor +
//! content + scalar value). The scheduler absorbs each into a single
//! pool, gates eligibility on whether the batch's predecessor (if any)
//! is already scheduled, and ranks eligible batches by
//! `value / cost^k`. Under prefix-monotone scheduling (see
//! `docs/design-notes.md`), if the top-ranked batch doesn't fit the
//! scheduler spends what is left on the longest affordable prefix of
//! that batch and stops — no fallback to smaller batches
//! (`Scheduler::schedule_partial`).
//!
//! Two ratio adjustments depend on what is already scheduled, so they
//! live here rather than in a batch's value: a code `Body` batch ranks
//! lower the more of its file's code is scheduled (breadth pressure),
//! and a batch drawn only from the tree's dominant source file ranks
//! higher once that file has been entered. Under a char budget, `cost`
//! is the larger of a batch's tokens and its chars converted at the two
//! budgets' ratio (`ranking_cost`).

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use crate::batch::{Batch, BatchId, BatchKey, CodeKey, Rung};
use crate::content::BatchContent;
use crate::render::{Cost, RenderedTree};
use crate::value::ratio_with_exponent;
use crate::walker::{WalkCtx, Walker};

/// A single scheduled batch, captured in order for snapshots and the
/// divergence metric.
#[derive(Debug, Clone)]
pub struct ScheduledBatchRecord {
    pub key: BatchKey,
    pub content: BatchContent,
    pub cost: Cost,
    pub cum_tokens: usize,
}

/// Scheduler output: rendered tree + ordered log of scheduled batches.
#[derive(Debug)]
pub struct RunReport {
    pub tree: RenderedTree,
    pub scheduled: Vec<ScheduledBatchRecord>,
}

pub struct Scheduler<W: Walker> {
    walker: W,
    ctx: WalkCtx,
    tree: RenderedTree,
    token_budget: usize,
    char_budget: Option<usize>,
    consumed: Cost,

    /// Walker-emitted batches, indexed by [`BatchId`] (which is the position).
    entries: Vec<Batch>,
    /// Stable key→id lookup for predecessor resolution + dedup.
    key_to_id: HashMap<BatchKey, BatchId>,
    /// Scheduled batches.
    scheduled: HashSet<BatchId>,
    /// Unscheduled batches whose predecessor (if any) is scheduled.
    eligible: BTreeSet<BatchId>,
    /// Batches gated on a predecessor key not yet scheduled, by that key.
    waiting: HashMap<BatchKey, Vec<BatchId>>,
    /// Ordered log of scheduled batch ids + costs for the final report.
    scheduled_log: Vec<(BatchId, Cost)>,
    /// Cached exact marginal cost per emitted batch.
    cost_cache: HashMap<BatchId, Cost>,
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
    /// Code tokens scheduled per source file — drives the
    /// breadth-pressure ratio penalty.
    code_tokens_per_file: HashMap<PathBuf, usize>,
}

/// Breadth-pressure scale: a `Body` batch of a source file that already
/// has `t` tokens scheduled ranks at `1/(1 + t/SCALE)` of its raw ratio.
/// NS authors schedule breadth-first — every file's surface before any
/// file's depth — while cheap function bodies otherwise out-ratio
/// unopened siblings' surfaces and drive long same-file dives.
const BREADTH_PRESSURE_TOKEN_SCALE: f64 = 1000.0;

/// Ranking premium for content drawn from the tree's dominant source
/// file ([`WalkCtx::dominant_source_file`]). When one file holds most of
/// a repository's implementation, "what is this repo" and "what is in
/// that file" are the same question, and the marginal token buys more
/// inside that file than on another lap of breadth. Gated on the file
/// having already been entered on its own merits, so the premium
/// escalates depth rather than pulling one file in front of the
/// repository's orientation.
const DOMINANT_FILE_RATIO_BOOST: f64 = 1.35;

impl<W: Walker> Scheduler<W> {
    pub fn new(ctx: WalkCtx, walker: W, token_budget: usize, char_budget: Option<usize>) -> Self {
        let tree = RenderedTree::with_filter(
            ctx.root().to_path_buf(),
            ctx.source_cache().clone(),
            Rc::clone(ctx.dir_filter()),
        );
        Self {
            walker,
            ctx,
            tree,
            token_budget,
            char_budget,
            consumed: Cost::default(),
            entries: Vec::new(),
            key_to_id: HashMap::new(),
            scheduled: HashSet::new(),
            eligible: BTreeSet::new(),
            waiting: HashMap::new(),
            scheduled_log: Vec::new(),
            cost_cache: HashMap::new(),
            batches_by_path: HashMap::new(),
            dominant_file_batches: HashSet::new(),
            dominant_file_entered: false,
            code_tokens_per_file: HashMap::new(),
        }
    }

    /// Run the scheduler and return just the rendered tree.
    pub fn run(self) -> RenderedTree {
        self.run_with_report().tree
    }

    /// Run the scheduler and return the tree plus the scheduled-batches log.
    pub fn run_with_report(mut self) -> RunReport {
        // The first cost probe needs the tokenizer; build it while the
        // tree's one whole-walk scan runs.
        crate::tokenizer::warm_up();
        self.ctx.dominant_source_file();
        for batch in self.walker.seed(&self.ctx) {
            self.absorb(batch);
        }

        // Prefix-monotone scheduling — stop on the first top-ranked
        // batch that doesn't fit (no fallback to smaller batches). A pool
        // that runs dry is refilled once from the walker's floor.
        let mut floor_absorbed = false;
        loop {
            let Some((id, cost)) = self.top_ranked() else {
                if std::mem::replace(&mut floor_absorbed, true) {
                    break;
                }
                for batch in self.walker.floor(&self.entries, &self.ctx) {
                    self.absorb(batch);
                }
                continue;
            };
            if !self.fits(cost) {
                self.schedule_partial(id);
                break;
            }
            self.schedule(id, cost);
        }

        if cfg!(debug_assertions) {
            let rendered = self.tree.render();
            let total_tokens = crate::tokenizer::count(&rendered);
            debug_assert!(
                total_tokens <= self.token_budget,
                "rendered output exceeds token budget: {total_tokens} > {}",
                self.token_budget,
            );
            let total_chars = crate::render::char_units(&rendered);
            debug_assert!(
                self.char_budget.is_none_or(|cap| total_chars <= cap),
                "rendered output exceeds char budget: {total_chars} > {:?}",
                self.char_budget,
            );
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
        }
    }

    // ---- absorption ----

    /// Add a walker-emitted batch to the pool. Idempotent on key.
    fn absorb(&mut self, batch: Batch) {
        if self.key_to_id.contains_key(&batch.key) {
            return;
        }
        let id = BatchId::new(self.entries.len());

        self.key_to_id.insert(batch.key.clone(), id);

        if let BatchContent::Lines { spans, .. } = &batch.content {
            // Spans arrive grouped by file.
            for run in spans.chunk_by(|a, b| a.path == b.path) {
                self.batches_by_path
                    .entry(run[0].path.clone())
                    .or_default()
                    .push(id);
            }
            let in_dominant_file = self
                .ctx
                .dominant_source_file()
                .is_some_and(|dominant| spans.iter().all(|span| span.path == dominant));
            if in_dominant_file {
                self.dominant_file_batches.insert(id);
            }
        }

        match &batch.predecessor {
            Some(pred)
                if !self
                    .key_to_id
                    .get(pred)
                    .is_some_and(|pred_id| self.scheduled.contains(pred_id)) =>
            {
                self.waiting.entry(pred.clone()).or_default().push(id);
            }
            _ => {
                self.eligible.insert(id);
            }
        }
        self.entries.push(batch);
    }

    // ---- exact pool ----

    /// Top-ranked eligible batch + its cost.
    fn top_ranked(&mut self) -> Option<(BatchId, Cost)> {
        let eligible: Vec<BatchId> = self.eligible.iter().copied().collect();
        let mut best: Option<(f64, BatchId, Cost)> = None;
        for id in eligible {
            if !self.cost_cache.contains_key(&id) {
                let content = &self.entries[id.index()].content;
                let c = self.tree.marginal_cost(content);
                self.cost_cache.insert(id, c);
            }
            let pressure = self.breadth_pressure(id) * self.dominant_file_boost(id);
            let exact_cost = self.cost_cache[&id];
            let entry = &self.entries[id.index()];
            let ratio = ratio_with_exponent(
                entry.value,
                self.ranking_cost(exact_cost),
                entry.key.concavity_exponent(),
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

    /// Cost a batch ranks at, in tokens. Under a char budget, its chars
    /// are converted at the budgets' own chars-per-token rate and the
    /// larger of the two counts: a batch is priced in whichever budget
    /// it draws down faster.
    fn ranking_cost(&self, cost: Cost) -> usize {
        match self.char_budget {
            Some(cap) if cap > 0 => cost
                .tokens
                .max(cost.chars.saturating_mul(self.token_budget).div_ceil(cap)),
            _ => cost.tokens,
        }
    }

    fn fits(&self, cost: Cost) -> bool {
        if self.consumed.tokens + cost.tokens > self.token_budget {
            return false;
        }
        if let Some(cap) = self.char_budget
            && self.consumed.chars + cost.chars > cap
        {
            return false;
        }
        true
    }

    // ---- scheduling ----

    /// Apply `id`'s content to the tree and log it as scheduled.
    /// Returns the applied content. Bookkeeping that only pays off in a
    /// *later* round — cost invalidation, breadth-pressure counters, walker
    /// expansion — lives in [`Self::schedule`]; a terminal partial
    /// schedule skips it.
    fn apply_and_record(&mut self, id: BatchId, cost: Cost) -> BatchContent {
        let ancestors = self.ancestors_of(id);
        let entry_content = self.entries[id.index()].content.clone();
        let conflicts = self
            .tree
            .apply(&entry_content, id, |i| ancestors.contains(&i));
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
        self.eligible.remove(&id);
        if let Some(released) = self.waiting.remove(&self.entries[id.index()].key) {
            self.eligible.extend(released);
        }
        self.dominant_file_entered |= self.dominant_file_batches.contains(&id);
        self.scheduled_log.push((id, cost));
        self.consumed.tokens += cost.tokens;
        self.consumed.chars += cost.chars;
        entry_content
    }

    /// Terminal step once the top-ranked batch doesn't fit: schedule
    /// the longest affordable prefix of its entries, source lines or line
    /// units, so the budget left over is spent on the head of the batch
    /// every larger budget shows in full. The prefix is a function of the
    /// batch and the tree alone, so the output at a smaller budget stays
    /// a subset of the output at a larger one.
    fn schedule_partial(&mut self, id: BatchId) {
        let content = &self.entries[id.index()].content;
        let Some((prefix, cost)) = self.tree.affordable_prefix(content, |cost| self.fits(cost))
        else {
            return;
        };
        self.entries[id.index()].content = prefix;
        self.apply_and_record(id, cost);
    }

    fn schedule(&mut self, id: BatchId, cost: Cost) {
        debug_assert!(
            !self.scheduled.contains(&id),
            "batch {:?} scheduled twice",
            id
        );
        debug_assert!(
            self.fits(cost),
            "scheduled batch exceeds token/char budget; caller should have filtered it"
        );

        let entry_content = self.apply_and_record(id, cost);
        if let BatchKey::Code(key) = &self.entries[id.index()].key {
            *self
                .code_tokens_per_file
                .entry(key.file.clone())
                .or_default() += cost.tokens;
        }

        // Drop the scheduled batch's cached cost, plus every cached
        // cost for batches touching the same files — the apply may
        // have changed their synthesized-marker deltas. Costs of
        // batches on untouched files stay stable under walker
        // invariants (line-disjoint outside predecessor chains).
        self.cost_cache.remove(&id);
        if let BatchContent::Lines { spans, .. } = &entry_content {
            for run in spans.chunk_by(|a, b| a.path == b.path) {
                for other in self.batches_by_path.get(&run[0].path).into_iter().flatten() {
                    self.cost_cache.remove(other);
                }
            }
        }

        // Walker learns about the new scheduled key; emit successors.
        let key = self.entries[id.index()].key.clone();
        for batch in self.walker.expand(&key, &self.ctx) {
            self.absorb(batch);
        }
    }

    /// Ratio premium for `id` when it draws only on the dominant source
    /// file.
    fn dominant_file_boost(&self, id: BatchId) -> f64 {
        if self.dominant_file_entered && self.dominant_file_batches.contains(&id) {
            DOMINANT_FILE_RATIO_BOOST
        } else {
            1.0
        }
    }

    /// Breadth-pressure multiplier for `id`'s ratio. Applies only to
    /// function bodies — surfaces always rank at their raw ratio.
    fn breadth_pressure(&self, id: BatchId) -> f64 {
        let BatchKey::Code(CodeKey {
            rung: Rung::Body,
            file,
            ..
        }) = &self.entries[id.index()].key
        else {
            return 1.0;
        };
        let scheduled = self.code_tokens_per_file.get(file).copied().unwrap_or(0);
        1.0 / (1.0 + scheduled as f64 / BREADTH_PRESSURE_TOKEN_SCALE)
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
