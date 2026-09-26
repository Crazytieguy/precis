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
//! higher once that file has been entered. Until the root's identity (a
//! root manifest or build script) has been read, it outranks the listings
//! below the source spine. Under a char budget, `cost` is the larger of a
//! batch's tokens and its chars converted at the two budgets' ratio
//! (`ranking_cost`).

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use crate::batch::{
    Batch, BatchId, BatchKey, CodeKey, FsKey, GoModKey, JsonKey, PlaintextKey, Rung, TomlKey,
};
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
    /// Eligible batches by ratio, best last.
    ranking: BTreeSet<Rc<RankedBatch>>,
    /// Each batch's [`Self::ranking`] entry, by id.
    ranked: Vec<Option<Rc<RankedBatch>>>,
    /// Batches whose ratio must be recomputed before the next pick.
    stale: Vec<BatchId>,
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
    /// Every [root identity](Self::is_root_identity) batch absorbed.
    root_identities: HashSet<BatchId>,
    /// Whether a root identity batch has been scheduled.
    root_identity_read: bool,
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
            ranking: BTreeSet::new(),
            ranked: Vec::new(),
            stale: Vec::new(),
            batches_by_path: HashMap::new(),
            dominant_file_batches: HashSet::new(),
            dominant_file_entered: false,
            code_tokens_per_file: HashMap::new(),
            root_identities: HashSet::new(),
            root_identity_read: false,
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
                self.stale.push(id);
            }
        }
        self.entries.push(batch);
        self.ranked.push(None);
        if self.is_root_identity(id) {
            self.root_identities.insert(id);
        }
    }

    // ---- exact pool ----

    /// Top-ranked eligible batch + its cost.
    fn top_ranked(&mut self) -> Option<(BatchId, Cost)> {
        self.rerank_stale();
        let best = self.ranking.last()?.id;
        let picked = if !self.root_identity_read && self.is_listing_below_spine(best) {
            self.root_identities
                .iter()
                .filter_map(|id| self.ranked[id.index()].as_ref())
                .max()
                .map_or(best, |ranked| ranked.id)
        } else {
            best
        };
        Some((picked, self.cost_cache[&picked]))
    }

    /// Ranks every stale eligible batch at its current ratio, costing
    /// it first if its cost isn't cached.
    fn rerank_stale(&mut self) {
        let mut stale = std::mem::take(&mut self.stale);
        stale.sort_unstable();
        stale.dedup();
        for id in stale {
            if !self.eligible.contains(&id) {
                continue;
            }
            let entry = &self.entries[id.index()];
            self.cost_cache
                .entry(id)
                .or_insert_with(|| self.tree.marginal_cost(&entry.content));
            let ratio = self.ranking_ratio(id);
            if self.ranked[id.index()]
                .as_ref()
                .is_some_and(|ranked| ranked.ratio == ratio)
            {
                continue;
            }
            self.unrank(id);
            let ranked = Rc::new(RankedBatch {
                ratio,
                key: self.entries[id.index()].key.clone(),
                id,
            });
            self.ranking.insert(Rc::clone(&ranked));
            self.ranked[id.index()] = Some(ranked);
        }
    }

    fn unrank(&mut self, id: BatchId) {
        if let Some(ranked) = self.ranked[id.index()].take() {
            self.ranking.remove(&ranked);
        }
    }

    /// `id`'s value per unit of cost, with the adjustments that depend on
    /// what is already scheduled. Its cost must be cached.
    fn ranking_ratio(&self, id: BatchId) -> f64 {
        let entry = &self.entries[id.index()];
        ratio_with_exponent(
            entry.value,
            self.ranking_cost(self.cost_cache[&id]),
            entry.key.concavity_exponent(),
        ) * (self.breadth_pressure(id) * self.dominant_file_boost(id))
    }

    /// Whether `id` lists a directory below the source spine and off it.
    fn is_listing_below_spine(&self, id: BatchId) -> bool {
        let BatchKey::Fs(FsKey::DirListing { dir } | FsKey::DirListingTail { dir }) =
            &self.entries[id.index()].key
        else {
            return false;
        };
        !self.ctx.is_on_source_spine(dir)
            && dir
                .ancestors()
                .skip(1)
                .any(|ancestor| self.ctx.is_on_source_spine(ancestor))
    }

    /// A batch that says what the repository is and how to build it: the
    /// identity block or head of a root manifest, the head of a root file
    /// named `build` with any extension (`build.zig`, `build.gradle.kts`,
    /// `build.sh`), or a root Makefile's build and test rules. A
    /// `Dockerfile` or compose file is not one.
    fn is_root_identity(&self, id: BatchId) -> bool {
        let (BatchKey::Toml(TomlKey::Identity { file })
        | BatchKey::Json(JsonKey::Identity { file })
        | BatchKey::GoMod(GoModKey::Identity { file })
        | BatchKey::Plaintext(
            PlaintextKey::DeclSurface { file } | PlaintextKey::Whole { file },
        )) = &self.entries[id.index()].key
        else {
            return false;
        };
        let (Some(dir), Some(name)) = (file.parent(), file.file_name()) else {
            return false;
        };
        let name = name.to_string_lossy();
        dir == self.ctx.root()
            && (!matches!(self.entries[id.index()].key, BatchKey::Plaintext(_))
                || crate::walker::is_unparsed_manifest(dir, &name, &self.ctx)
                || crate::walker::is_makefile_name(&name)
                || name.split('.').next() == Some("build"))
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
        let total = self.consumed + cost;
        total.tokens <= self.token_budget && self.char_budget.is_none_or(|cap| total.chars <= cap)
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
        debug_assert!(
            conflicts.is_empty(),
            "walker-emitted batch hit non-ancestor overlap for {:?}: {conflicts:?}",
            self.entries[id.index()].key
        );
        self.scheduled.insert(id);
        self.eligible.remove(&id);
        self.unrank(id);
        if let Some(released) = self.waiting.remove(&self.entries[id.index()].key) {
            self.eligible.extend(&released);
            self.stale.extend(released);
        }
        if !self.dominant_file_entered && self.dominant_file_batches.contains(&id) {
            self.dominant_file_entered = true;
            self.stale.extend(&self.dominant_file_batches);
        }
        self.scheduled_log.push((id, cost));
        self.consumed = self.consumed + cost;
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
        if self.root_identities.contains(&id) {
            self.root_identity_read = true;
        }
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
        // invariants (line-disjoint outside predecessor chains). A code
        // batch's spans lie in its key's file, so re-ranking these also
        // covers that file's `Body` batches' new breadth pressure.
        self.cost_cache.remove(&id);
        if let BatchContent::Lines { spans, .. } = &entry_content {
            for run in spans.chunk_by(|a, b| a.path == b.path) {
                for other in self.batches_by_path.get(&run[0].path).into_iter().flatten() {
                    self.cost_cache.remove(other);
                    self.stale.push(*other);
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
                debug_assert!(false, "predecessor cycle detected at {pred_key:?}");
                break;
            }
            cur = self.entries[pred_id.index()].predecessor.as_ref();
        }
        set
    }
}

/// A `Scheduler::ranking` entry. Orders by ratio, then by key with the
/// smaller key ranking higher. Ratios are never NaN, so `total_cmp`
/// agrees with the float order.
struct RankedBatch {
    ratio: f64,
    key: BatchKey,
    id: BatchId,
}

impl Ord for RankedBatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ratio
            .total_cmp(&other.ratio)
            .then_with(|| other.key.cmp(&self.key))
    }
}

impl PartialOrd for RankedBatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RankedBatch {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for RankedBatch {}

#[cfg(test)]
mod tests {
    /// Each of the spine's forty sub-listings outranks the build file's
    /// head, and together they fill the budget.
    #[test]
    fn scheduler_root_identity_outranks_the_spines_sublistings() {
        let cmake = format!(
            "cmake_minimum_required(VERSION 3.20)\nproject(engine C)\n{}",
            "target_sources(engine PRIVATE src/core/alpha.c src/core/beta.c src/core/gamma.c)\n"
                .repeat(40)
        );
        let makefile = format!(
            "{}test: engine\n\t./run-tests --all\n",
            "src/%.o: src/%.c\n\t$(CC) -c $< -o $@ $(CFLAGS) $(EXTRA_FLAGS)\n".repeat(60)
        );
        for (build_file, text, expected) in [
            ("CMakeLists.txt", cmake, "project(engine C)"),
            ("Makefile", makefile, "./run-tests --all"),
        ] {
            let output = render_with_spine(build_file, &text);
            assert!(output.contains(expected), "{output}");
        }
    }

    fn render_with_spine(build_file: &str, text: &str) -> String {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let write = |path: &str, text: &str| {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        };
        write(build_file, text);
        for module in 0..40 {
            for file in ["a", "b"] {
                write(
                    &format!("src/mod{module}/{file}.c"),
                    &"int f(void) { return 0; }\n".repeat(20),
                );
            }
        }
        crate::render(root, 400, None).unwrap()
    }
}
