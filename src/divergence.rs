//! Divergence metric + report generator. Compares a walker `Schedule`
//! run at `T_max` to a frozen `NorthStar`. Two artifacts:
//!
//! - `Scores`: per-budget `Score(B)` vector across a 7-budget grid +
//!   `A_3K`-gated bucket counts `reached + partial + missing ==
//!   ns_rows≤3K`. Primary objective is `Score(3000)` — the auto-
//!   injection budget every session hits.
//! - Per-fixture Markdown report (`tests/divergence/<fixture>.md`):
//!   Score line + per-budget table, Top opportunities (additive +
//!   subtractive subtables), Top missed paths + arrival ledger
//!   grouped by diagnosis, Top wasted paths + walker-waste pattern
//!   rollup + per-batch detail.
//!   Reached rows (damped credit ≥ 0.8 at the primary budget) are
//!   filtered out — the optimization target only depends on partial /
//!   missing rows. Stable ordering; perfect alignment ⇒ very short
//!   file.
//!
//! Cross-fixture survey is via shell, not a generated index file:
//! `head -1 tests/divergence/*.md` for filename-ordered score lines, or
//! `grep -H '^scores:' tests/divergence/*.md | sort -t= -k2 -g` for a
//! Score(3000)-ascending index.
//!
//! NS `exp_t` is the cumulative marginal cost of applying NS batches in
//! rank order to one shared `RenderedTree` — same accounting as
//! `simulate_ns`, so predecessor refinements over ellipsis lines cost
//! only their delta. Under scheduler prefix-monotonicity, walker
//! sub-budget behavior is the prefix of the T_max schedule with
//! `cum_tokens ≤ t`, so we run the walker once and slice it for each
//! budget in the grid.
//!
//! ## Score line + per-budget table
//!
//! Headline: `Score(3000)=X.XXX ns_rows≤3K=N/T (reached=R partial=P
//! missing=M)`. `N` is the number of NS *rows* (batches) with
//! `exp_t ≤ 3000`; `T` is total NS rows. Bucket counts are gated to
//! those rows — under per-budget the optimization target only
//! depends on rows in `A_3K`. (Distinct from `A_B` in the per-budget
//! table, which counts NS *atoms* in `A_B`, not rows.)
//!
//! Followed by a 7-row table:
//! `| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |`
//!
//! - `B ∈ {1000, 1442, 2080, 3000, 4327, 6240, 9000}` — geometric grid
//!   on `[1000, 9000]` with ratio ⁶√9 ≈ 1.442, symmetric around 3000
//!   on the log scale.
//! - `A_B` — number of NS atoms in batches with `cum_tokens ≤ B`.
//! - `I(B)` — Importance, rank-weighted recall: `min(1, Σ r(rank(a)) ·
//!   damped_credit(a) / Σ_{rank ≤ |A_B|} r(rank))`. Atoms past `A_B`
//!   contribute to the numerator (with their NS rank) but the
//!   denominator is fixed to A_B and the result caps at 1 — perfect is
//!   "delivered all of A_B".
//! - `C(B)` — Coverage, rank-uniform recall over A_B: `Σ_{rank ≤ |A_B|}
//!   damped_credit(a) / |A_B|`.
//! - `compl(B)` — mean per-row completion across A_B rows the walker
//!   delivered any atom of (rank-order misses excluded; they're
//!   already visible in low I/C). Isolates the orthogonal "finish what
//!   you started" signal — `f64::NAN` ⇒ `—` when no A_B row was
//!   delivered.
//! - `Score(B) = √(I(B) · C(B))`.
//! - `walker_used` — walker `cum_tokens` at the last batch fitting in B.
//!
//! ## Atoms + credit + completion
//!
//! Atoms are `Line(path, line)` or `Fs(parent, entry)`. Each has a
//! `bytes` footprint per render (Full = source-line length, Truncated =
//! regex match end, Ellipsis = 1, Fs = 1; floored at 1 so "rendered" is
//! distinguishable from "absent"). Credit between matched walker + NS
//! atoms is `min(walker, ns) / max(ns, 1)`, capped at 1.0.
//!
//! NS atoms carry a 1-indexed `rank` from a flat traversal of NS
//! batches in schedule order, and `r(rank) = 1/rank` (α = 1) is the
//! per-atom Importance weight.
//!
//! `completion(B_i) = Σ_{a ∈ B_i} min(walker_bytes(a), ns_bytes(a)) /
//! Σ_{a ∈ B_i} ns_bytes(a)` — byte-weighted fraction of NS batch `B_i`
//! the walker delivered. Per-atom `damped_credit(a) = credit(a) ·
//! completion(B(a))` gates each atom by the completeness of its
//! enclosing NS batch — a walker covering 50% of every batch scores
//! worse than one completing fewer batches fully ("finish what you
//! start", with no thresholds).
//!
//! ## Primary-budget anchor
//!
//! Every per-row signal in the report — `credit`, `status`, the
//! headline counts, and the diagnosis bucketing — is computed against
//! the walker state at the **primary budget** (`B = 3000`), not at
//! T_max. This is the contract that makes the report honest: a row
//! expected before 3K but first delivered at t=8000 contributes 0 to
//! `Score(3000)`, so it must show as `missing` (not `reached`) and
//! remain visible to the iterator. Per-budget priority weights use
//! each budget's own `walker_cum`, so the vector context (`gap@1k`,
//! `gap@9k`) is honestly per-budget rather than collapsing to
//! final-state.
//!
//! ## Top opportunities
//!
//! The brief: two subtables (Additive, Subtractive) under one header.
//! Each row is one intervention with the rows or batches it would
//! address. Capped at 5 rows per subtable.
//!
//! ### Additive (close partial / missing rows)
//!
//! Columns: `intervention | rows | gap@1k | gap@3k | gap@9k | evidence
//! | top row ids`.
//!
//! - `intervention` — what to change (e.g. `promote go decl signature
//!   batches`, `split wrong-slice walker batches`, `add walker
//!   candidates for no-discovered rows`, `tune ranking for
//!   high-overlap unscheduled candidates`).
//! - `gap@B` — **non-additive priority score** `Σ over atoms in row:
//!   (1 − damped_credit(a)) / rank(a)` at budget `B`'s walker state.
//!   Approximates the row's headroom on `Score(B)` via the Importance
//!   numerator (`compute_score_at` has no rank cap, so atoms past
//!   `|A_B|` contribute via `1/rank` until Importance saturates at 1).
//!   `gap@3k` is the primary sort key. Monotone non-increasing in `B`
//!   (walker has more budget at higher `B`). Rows can overlap between
//!   opportunities, so sums are upper bounds on `Score(B)` impact,
//!   not exact deltas. Opportunities with `gap@3k = 0` are dropped.
//! - `evidence` — short rationale (file count for predecessor-kind
//!   groups, exact-atom totals, etc.).
//! - `top row ids` — up to 5 ids sorted by `gap@3k` descending.
//!
//! Predecessor-kind grouping collapses parent-gated rows by walker-key
//! class (`go decl at <file>` predecessors → one `promote go decl
//! signature batches` opportunity) — the calibration-relevant frame
//! since `value.rs` is tuned by walker-key class. `DiscoveredUnscheduled`
//! and `TooExpensiveAtFinalMargin` collapse into one ranking-race
//! intervention (`TooExpensive` is post-hoc; we can't disambiguate
//! "fit at eligibility, lost rank race" from "never fit" without
//! scheduler instrumentation, but both subsets share the rank-tuning
//! lever).
//!
//! ### Subtractive (suppress consistently off-NS batches)
//!
//! Columns: `pattern | batches | freed@1k | freed@3k | freed@9k |
//! evidence | top batch ids`.
//!
//! - `freed@B` — marginal tokens the walker spent on atoms **not in
//!   any NS batch** (`off_any`), summed over batches with `walker_t
//!   ≤ B`. We use `off_any` rather than "atoms not in `A_B`" because
//!   atoms NS wants past `B` still contribute to `Score(B)` via the
//!   no-rank-cap Importance numerator — counting them as "freed"
//!   would steer iteration toward suppressing batches that aren't
//!   truly waste. Different units from additive `gap@B`
//!   (dimensionless rank-weighted priority); both are priority
//!   signals, neither is an exact `Score(B)` delta — the freed slot
//!   refills with whatever's next-best. Sorted by `freed@3k`
//!   descending; patterns with `freed@3k = 0` are dropped.
//! - `evidence` shows `off_3k` for context: the looser "off `A_3K`"
//!   measure that includes both truly off-NS atoms and NS-after-3K
//!   atoms. `off_3k ≈ freed@3k` ⇒ purely irrelevant content;
//!   `off_3k > freed@3k` ⇒ pattern also carries premature NS-late
//!   content (visible in the walker-waste detail table — useful for
//!   the timing-tradeoff iterator, not safe to fold into a suppress
//!   recommendation).
//!
//! ## Arrival ledger by diagnosis
//!
//! One section per diagnosis bucket in fixed order:
//! `ranking-recoverable`, `wrong-slice / granularity`,
//! `no discovered candidate`, `fs/listing`, `mixed/unknown`.
//! Within each section rows sort by `ns_t` ascending. Per-row
//! columns: `id | ns_t | credit | status | descriptor | candidate
//! hint`.
//!
//! Diagnoses are bucket labels assigned per row from `candidate_hint`
//! plus per-row state — they describe the row's symptom shape:
//!
//! - `ranking-recoverable`: an unscheduled candidate has high/full
//!   exact overlap on the row's atoms. Lever: rank tuning, or
//!   promoting a parent batch when the candidate is predecessor-
//!   gated.
//! - `wrong-slice / granularity`: the scheduled (or nearest) walker
//!   batch overlaps the row's source region but with low exact atom
//!   overlap. Lever: split the walker's batch shape to match NS slices.
//! - `no discovered candidate`: no walker-emitted batch has any line
//!   atom on the row's paths. Lever: add walker candidates.
//! - `fs/listing`: NS row carries only Fs atoms; no line bbox to score.
//! - `mixed/unknown`: partial-state rows that don't fit cleanly into
//!   the others — inspect manually.
//!
//! Per-row fields:
//!
//! - `ns_t` — NS cumulative tokens at this batch (when NS expects it).
//! - `credit` — primary-budget byte-credit averaged over NS atoms
//!   (atom-count weighted).
//! - `status` ∈ `{partial, missing}` — banded on `credit × completion`
//!   (the damped credit `Score(B)` consumes): `< 0.5` → missing,
//!   `< 0.8` → partial. Reached rows (damped credit ≥ 0.8) are filtered
//!   out — no actionable gap on `Score(3000)`.
//! - `candidate hint` — `[<hint kind> exact=H/T] <descriptor> (...,
//!   <loss>)`. Hint kinds: `scheduled bbox` (a scheduled walker
//!   batch's atoms fall inside the NS row's per-file line bbox),
//!   `unscheduled bbox` (same but candidate is unscheduled — often
//!   ranking-recoverable at high overlap), `scheduled / unscheduled
//!   same-file`, `fs-only`, `no discovered candidate`. When the
//!   chosen hint is `scheduled bbox` and a non-ancestor unscheduled
//!   candidate has higher exact overlap, the hint appends
//!   `; better unscheduled exact=H'/T': ...` to surface hidden ranking
//!   failures. **Caveat**: candidates are walker batches discovered
//!   this run; descendants behind unscheduled predecessors may not be
//!   present, so `no discovered candidate` isn't proof that no emit
//!   path exists. Per-row loss labels: `predecessor not scheduled`,
//!   `too expensive at final margin`, `discovered unscheduled`. The
//!   `too expensive` label is post-hoc — it conflates "fit at
//!   eligibility, lost rank race" with "never fit" (see
//!   `docs/design-notes.md`).
//!
//! Within `ranking-recoverable`, predecessor-gated children that share
//! a parent collapse into a single `group` row in the ledger
//! (`<n> children of <predecessor>`) — keeps the section scannable
//! when one parent gates many children.
//!
//! Two one-line path rollups (inline preambles, not their own
//! sections) frame the ledger / waste tables by dominant source path:
//!
//! - **Top missed paths** (top of arrival ledger): A_3K rows the
//!   walker didn't deliver, grouped by file. Pairs with...
//! - **Top wasted paths** (top of walker waste): primary-actionable
//!   waste batches grouped by file. Side-by-side these surface
//!   path-mismatch (walker spending on file X while NS wants file Y).
//!
//! ## Walker waste
//!
//! Walker batches with `off_3k_tokens ≥ UNMAPPED_COST_THRESHOLD` and
//! `walker_t ≤ 3000` (within the primary prefix or boundary-crossing
//! it), sorted by `off_3k_tokens` descending. Surfaces both pure-
//! waste (`off_3k_ratio = 1.00`) and mixed-intersection batches.
//!
//! Two tables:
//!
//! - **Walker waste rollup (by descriptor pattern)** groups detail
//!   rows by descriptor pattern (e.g. `pub-item doc at src/lib.rs:<n>`
//!   collapses 14 per-line rows into one). Surfaces systemic walker
//!   over-spend; elided when no pattern groups two-or-more rows.
//! - **Walker waste** (per-batch detail) capped at `WASTE_DETAIL_LIMIT`
//!   rows with a `+N more rows` tail summarizing the remainder.
//!
//! Columns on the per-batch detail: `off_3k | off_3k_ratio | off_any
//! | cost | walker_t | batch`.
//!
//! - `off_3k` / `off_3k_ratio` — marginal tokens (and their fraction)
//!   on atoms not in `A_3K`'s NS atom set, i.e. contributing zero to
//!   `Score(3000)`. Includes both atoms NS never wants AND atoms NS
//!   wants only past 3K. The primary sort and admission key.
//! - `off_any` — marginal tokens on atoms absent from any NS batch
//!   (any budget). Distinguishes "purely irrelevant" (`off_any ≈
//!   off_3k` — suppress globally) from "premature" (`off_any <
//!   off_3k` — would help at higher budgets, just not 3K).
//! - `cost` — total marginal cost of the walker batch.
//! - `walker_t` — walker `cum_tokens` *before* this batch was applied.
//!   The gate value: `walker_t ≤ 3000` includes boundary-crossers
//!   (`walker_t ≤ 3000 < walker_t + cost`) — these are batches that
//!   *stopped* the 3K prefix under the no-fallback scheduler, so
//!   demoting one can let the next-best candidate fit and lift
//!   `Score(3000)`.
//! - `batch` — descriptor with fixture root stripped.
//!
//! Off-NS attribution is **per-atom marginal**: each atom carries the
//! token delta it actually contributed to the batch's marginal cost
//! (refinement-over-ancestor lines pay the truncated delta;
//! already-listed FS entries pay 0), captured by driving a parallel
//! walker tree forward in schedule order. The waste columns sum
//! exactly the off-NS atoms' marginal contributions — no
//! ratio-times-cost approximation.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::batch::BatchId;
use crate::content::{BatchContent, FsEntries, Render, explode_spans};
use crate::north_star::NorthStar;
use crate::ns_loader::resolve_content;
use crate::render::{Cost, RenderedTree, SourceCache};
use crate::schedule_types::{Atom, CandidateBatch, Schedule, ScheduledBatch};

mod diagnosis;
mod render;
mod synthesis;
#[cfg(test)]
use diagnosis::candidate_hint;
use diagnosis::{
    CandidateHint, CandidateHintKind, CandidateLoss, CandidateLossReason, DiagnosisKind,
    ExactOverlap, ExactOverlapBucket, candidate_hint_for_ctx, diagnose_row,
};
use render::format_report;
#[cfg(test)]
use render::{WalkerWasteRow, off_ns_attribution, top_suppression_opportunities};
use synthesis::{ReportRow, predecessor_groups_from_refs, report_rows, row_ids, top_opportunities};

/// Per-budget Score grid: geometric on `[1000, 9000]` with ratio
/// `⁶√9 ≈ 1.442`, symmetric around 3000 on the log scale. The vector
/// at these budgets is the headline; `Score(3000)` is the primary
/// objective (auto-injection budget every session hits).
pub const BUDGETS: [usize; 7] = [1000, 1442, 2080, 3000, 4327, 6240, 9000];

/// Index of the primary budget within [`BUDGETS`]. `Score(3000)` is
/// the single number that drives sort order and headline framing.
pub const PRIMARY_BUDGET_INDEX: usize = 3;

/// Low-end budget shown alongside the primary in opportunity tables —
/// surfaces the "front-loaded vs. trailing" walker shape directly in
/// the priority columns rather than only the headline vector.
pub(crate) const LOW_BUDGET_INDEX: usize = 0;

/// High-end budget shown alongside the primary in opportunity tables.
/// `|A_9K|` covers nearly all NS atoms in practice, so this approximates
/// the "all atoms" total we previously surfaced as `rank×gap`.
pub(crate) const HIGH_BUDGET_INDEX: usize = BUDGETS.len() - 1;

/// Damped-credit threshold: NS batch counted as `reached` iff
/// `credit × completion` at the primary budget ≥ this. Same
/// quantity Score(B) consumes per atom, so the bucketing reflects
/// what the metric rewards.
const REACH_THRESHOLD: f64 = 0.8;

/// Damped-credit threshold: NS batch counted as `missing` iff
/// `credit × completion` at the primary budget < this. Rows in
/// `[MISSING_FLOOR, REACH_THRESHOLD)` are `partial`.
const MISSING_FLOOR: f64 = 0.5;

/// Report threshold: walker-waste rows elide batches below this cost.
const UNMAPPED_COST_THRESHOLD: usize = 50;
const WASTE_DETAIL_LIMIT: usize = 10;

/// One row of the per-budget score table.
#[derive(Debug, Clone, Copy)]
pub struct ScoreAtBudget {
    /// Token budget this row evaluates at (one of [`BUDGETS`]).
    pub budget: usize,
    /// `|A_B|` — NS atoms in batches with `cum_tokens ≤ budget`.
    pub a_b_atoms: usize,
    /// Importance — rank-weighted recall, capped at 1.
    pub importance: f64,
    /// Coverage — rank-uniform recall over A_B.
    pub coverage: f64,
    /// Mean per-row completion across A_B rows (i.e. `exp_t ≤ budget`)
    /// **the walker delivered any atom of**. Restriction matters:
    /// `completion_for_row` returns 0.0 for fully missing rows, so a
    /// naive mean would conflate "row never delivered" (rank-order
    /// miss, already visible in low importance/coverage) with "row
    /// partially delivered" (the orthogonal partial-delivery signal
    /// this column isolates). `f64::NAN` when no A_B row was
    /// delivered; the renderer formats this as `—`.
    pub completion: f64,
    /// `√(importance × coverage)`.
    pub score: f64,
    /// Walker `cum_tokens` at the last batch fitting in `budget`.
    pub walker_used: usize,
}

/// Headline scores. Bucket counts are gated to atoms reachable at the
/// primary budget (`A_3K`) — they describe the rows the optimization
/// target actually depends on, not whole-NS noise.
/// `reached + partial + missing == rows_in_a_primary`.
#[derive(Debug, Clone)]
pub struct Scores {
    /// Per-budget vector, one entry per [`BUDGETS`] slot.
    pub vector: [ScoreAtBudget; BUDGETS.len()],
    /// Total NS batches across the whole NS (all budgets). For "X of Y"
    /// framing where Y captures the full ground truth.
    pub total_ns: usize,
    /// NS batches with `exp_t ≤ PRIMARY_BUDGET` — the rows whose status
    /// drives `Score(3000)`. Equal to `reached + partial + missing`.
    pub rows_in_primary: usize,
    /// In `A_3K`: final credit ≥ [`REACH_THRESHOLD`].
    pub reached: usize,
    /// In `A_3K`: final credit in `[MISSING_FLOOR, REACH_THRESHOLD)` —
    /// present but diluted.
    pub partial: usize,
    /// In `A_3K`: final credit < [`MISSING_FLOOR`].
    pub missing: usize,
}

impl Scores {
    /// `Score(3000)` — the primary objective.
    pub fn primary(&self) -> f64 {
        self.vector[PRIMARY_BUDGET_INDEX].score
    }
}

/// Primary budget for headline counts and per-row attention direction.
/// Sourced from [`BUDGETS`] at [`PRIMARY_BUDGET_INDEX`].
pub(crate) const PRIMARY_BUDGET: usize = BUDGETS[PRIMARY_BUDGET_INDEX];

/// Compute scores. `schedule` is expected to be a full-cap walker run.
pub fn score(ns: &NorthStar, schedule: &Schedule, fixture_root: &Path) -> Result<Scores> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let walker = WalkerSnapshots::build(&ctx);
    let arrivals = arrival_infos(&ctx, &walker);
    Ok(build_scores(&ctx, &arrivals, &walker))
}

/// Generate the markdown divergence report (one per fixture).
pub fn generate_divergence_report(
    ns: &NorthStar,
    schedule: &Schedule,
    fixture_root: &Path,
) -> Result<String> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let walker = WalkerSnapshots::build(&ctx);
    let arrivals = arrival_infos(&ctx, &walker);
    let scores = build_scores(&ctx, &arrivals, &walker);
    Ok(format_report(&scores, &ctx, &arrivals))
}

// ---- graded atoms ------------------------------------------------------

/// One atom of content with the byte footprint used for **credit
/// accounting only**. This is *not* a render-cost weight — see
/// `WalkerRow::atom_token_costs` for that.
///
/// `bytes` semantics (minimum 1 for any present atom — 0 is reserved for
/// "walker never rendered this atom"):
/// - `Line::Full` = source line length.
/// - `Line::Truncated{pattern}` = regex-match byte-end (validator
///   guarantees ≥ 1 on every covered line).
/// - `Line::Ellipsis` = `1`.
/// - `Fs`: always `1`. Fs atoms are boolean.
///
/// Credit between two graded atoms sharing identity (same `Atom`) is
/// `min(walker.bytes, ns.bytes) / max(ns.bytes, 1)`, capped at 1.0.
/// Over-rendering (walker shows strictly more bytes than NS asks) is
/// fully credited but flagged as a row annotation when the batch is
/// otherwise aligned.
#[derive(Debug, Clone)]
struct GradedAtom {
    atom: Atom,
    bytes: usize,
}

fn atoms_from_content(
    content: &BatchContent,
    source_cache: &SourceCache,
    fixture_root: &Path,
) -> Vec<GradedAtom> {
    match content {
        BatchContent::Fs { groups } => {
            let mut out = Vec::new();
            for g in groups {
                let FsEntries::Listed(paths) = &g.entries else {
                    debug_assert!(
                        false,
                        "unresolved FsEntries in divergence at {}",
                        g.parent.display()
                    );
                    continue;
                };
                for p in paths {
                    let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
                        continue;
                    };
                    out.push(GradedAtom {
                        atom: Atom::Fs {
                            parent: g.parent.clone(),
                            entry: name.to_string(),
                        },
                        bytes: 1,
                    });
                }
            }
            out
        }
        BatchContent::Lines { spans } => explode_spans(spans)
            .into_iter()
            .map(|(path, line, render)| {
                let abs = if path.is_absolute() {
                    path.clone()
                } else {
                    fixture_root.join(&path)
                };
                let source = source_cache.get(&abs);
                let src_line = source
                    .as_deref()
                    .and_then(|s| s.lines().nth(line.saturating_sub(1)))
                    .unwrap_or("");
                let bytes = byte_end_for(&render, src_line);
                GradedAtom {
                    atom: Atom::Line { path, line },
                    bytes,
                }
            })
            .collect(),
    }
}

/// Per-render byte_end: where this render stops within the source line.
/// `Ellipsis` is a 1-byte sentinel (not 0) so NS-ellipsis vs walker-
/// ellipsis on the same line scores full credit. All render kinds floor
/// at 1 so "present" is strictly distinguishable from "not rendered".
fn byte_end_for(render: &Render, source_line: &str) -> usize {
    match render {
        Render::Full => source_line.len().max(1),
        Render::Ellipsis => 1,
        Render::Truncated { pattern } => regex::Regex::new(pattern)
            .ok()
            .and_then(|re| re.find(source_line).map(|m| m.end()))
            .unwrap_or(0)
            .max(1),
    }
}

// ---- context -----------------------------------------------------------

struct BuildCtx<'a> {
    ns_rows: Vec<NsRow>,
    walker_rows: Vec<WalkerRow<'a>>,
    candidate_rows: Vec<CandidateRow<'a>>,
    scheduled_keys: BTreeSet<String>,
    ns: &'a NorthStar,
    schedule: &'a Schedule,
    fixture_root: PathBuf,
}

struct NsRow {
    id: String,
    atoms: Vec<GradedAtom>,
    /// Cumulative token cost up through this batch, computed as the
    /// marginal cost of applying each batch in order to a shared
    /// `RenderedTree` — same accounting as `simulate_ns`.
    exp_t: usize,
    /// 1-indexed rank of this row's first atom in the flat NS schedule
    /// order. `r(rank_start + i) = 1 / (rank_start + i)` is atom `i`'s
    /// Importance weight. NS batches arrive in rank order so atoms in
    /// `A_B` are exactly ranks `1..=|A_B|`.
    rank_start: usize,
}

struct WalkerRow<'a> {
    atoms: Vec<GradedAtom>,
    /// Per-atom marginal token cost at the moment this batch was scheduled
    /// — 1:1 with `atoms`. Computed against a parallel `RenderedTree`
    /// driven forward in scheduling order so each atom's contribution is
    /// the actual delta the scheduler would have paid (refinement-over-
    /// ancestor lines pay the truncated delta; already-listed FS entries
    /// pay 0). Used by `format_walker_waste` to attribute off-NS spend
    /// in true-marginal terms rather than uniformly across atoms.
    atom_token_costs: Vec<usize>,
    seen_t: usize,
    batch: &'a ScheduledBatch,
}

struct CandidateRow<'a> {
    atoms: Vec<GradedAtom>,
    final_cost: Cost,
    scheduled: bool,
    batch: &'a CandidateBatch,
}

impl<'a> BuildCtx<'a> {
    fn new(ns: &'a NorthStar, schedule: &'a Schedule, fixture_root: &Path) -> Result<Self> {
        let source_cache = SourceCache::new();
        let fixture_root_buf = fixture_root.to_path_buf();
        let mut tree = RenderedTree::new(fixture_root_buf.clone(), source_cache.clone());

        let mut ns_rows = Vec::with_capacity(ns.batches.len());
        let mut cum = 0usize;
        let mut next_rank = 1usize;
        for (pos, b) in ns.batches.iter().enumerate() {
            let content = resolve_content(&b.content, fixture_root)?;
            let atoms = atoms_from_content(&content, &source_cache, fixture_root);
            let marginal = tree.marginal_cost(&content);
            cum += marginal.tokens;
            let batch_id = BatchId::new(pos);
            // Divergence doesn't care about predecessor-chain conflicts
            // here — that's `simulate_ns`'s job. `|_| true` accepts any
            // existing owner so the tree evolves faithfully regardless.
            let _ = tree.apply(&content, batch_id, |_| true);
            let rank_start = next_rank;
            next_rank += atoms.len();
            ns_rows.push(NsRow {
                id: b.id.clone(),
                atoms,
                exp_t: cum,
                rank_start,
            });
        }

        // Parallel walker tree, driven forward in schedule order. Per-atom
        // marginal costs are read off this tree at scheduling time (so
        // refinement-over-ancestor lines see the truncated delta and
        // already-listed FS entries cost 0), then the batch is applied so
        // later rows see it as ancestor content. Mirrors what
        // `Scheduler::run_with_report` does for the real tree.
        let mut walker_tree = RenderedTree::new(fixture_root_buf.clone(), source_cache.clone());
        let walker_rows: Vec<WalkerRow<'_>> = schedule
            .batches
            .iter()
            .enumerate()
            .map(|(pos, b)| {
                let atoms = atoms_from_content(&b.content, &source_cache, fixture_root);
                let per_atom = walker_tree.marginal_cost_per_atom(&b.content);
                debug_assert_eq!(
                    per_atom.len(),
                    atoms.len(),
                    "marginal_cost_per_atom and atoms_from_content must agree on atom count and order — order invariant"
                );
                let atom_token_costs = per_atom.into_iter().map(|c| c.tokens).collect();
                let _ = walker_tree.apply(&b.content, BatchId::new(pos), |_| true);
                WalkerRow {
                    atoms,
                    atom_token_costs,
                    seen_t: b.cum_tokens,
                    batch: b,
                }
            })
            .collect();
        let scheduled_keys: BTreeSet<String> =
            schedule.batches.iter().map(|b| b.key.clone()).collect();
        let candidate_rows = schedule
            .candidates
            .iter()
            .map(|b| CandidateRow {
                atoms: atoms_from_content(&b.content, &source_cache, fixture_root),
                final_cost: walker_tree.marginal_cost(&b.content),
                scheduled: scheduled_keys.contains(&b.key),
                batch: b,
            })
            .collect();

        Ok(Self {
            ns_rows,
            walker_rows,
            candidate_rows,
            scheduled_keys,
            ns,
            schedule,
            fixture_root: fixture_root_buf,
        })
    }
}

// ---- scoring -----------------------------------------------------------

/// Per-budget walker state shared across `arrival_infos` and
/// `build_scores`. Building once removes the redundant 7× walker-row
/// passes the previous structure had, and shifts every credit /
/// completion / priority computation onto borrowed `&Atom` keys (the
/// owned-key version cloned the embedded `PathBuf` on every insert).
struct WalkerSnapshots<'a> {
    /// Per-budget byte-max maps, indexed by [`BUDGETS`].
    cums: [BTreeMap<&'a Atom, usize>; BUDGETS.len()],
    /// Per-budget atom counts of `A_B` (NS atoms in batches with
    /// `exp_t ≤ B`).
    a_b_atoms: [usize; BUDGETS.len()],
    /// Per-budget walker `cum_tokens` at the last batch fitting in B.
    walker_used: [usize; BUDGETS.len()],
}

impl<'a> WalkerSnapshots<'a> {
    fn build(ctx: &'a BuildCtx<'_>) -> Self {
        let cums = BUDGETS.map(|budget| walker_cum_at(ctx, budget));
        let a_b_atoms = BUDGETS.map(|budget| {
            ctx.ns_rows
                .iter()
                .filter(|r| r.exp_t <= budget)
                .map(|r| r.atoms.len())
                .sum::<usize>()
        });
        let walker_used = BUDGETS.map(|budget| {
            ctx.walker_rows
                .iter()
                .filter(|wr| wr.seen_t <= budget)
                .map(|wr| wr.seen_t)
                .max()
                .unwrap_or(0)
        });
        Self {
            cums,
            a_b_atoms,
            walker_used,
        }
    }
}

fn build_scores(ctx: &BuildCtx, arrivals: &[Arrival], walker: &WalkerSnapshots) -> Scores {
    let vector: [ScoreAtBudget; BUDGETS.len()] =
        std::array::from_fn(|i| compute_score_at(ctx, i, walker));

    let mut reached = 0;
    let mut partial = 0;
    let mut missing = 0;
    let mut rows_in_primary = 0;

    for (row, arrival) in ctx.ns_rows.iter().zip(arrivals) {
        if row.exp_t > PRIMARY_BUDGET {
            continue;
        }
        rows_in_primary += 1;
        match arrival.status {
            ArrivalStatus::Reached => reached += 1,
            ArrivalStatus::Partial => partial += 1,
            ArrivalStatus::Missing => missing += 1,
        }
    }

    Scores {
        vector,
        total_ns: ctx.ns_rows.len(),
        rows_in_primary,
        reached,
        partial,
        missing,
    }
}

fn compute_score_at(ctx: &BuildCtx, budget_idx: usize, walker: &WalkerSnapshots) -> ScoreAtBudget {
    let budget = BUDGETS[budget_idx];
    let walker_cum = &walker.cums[budget_idx];
    let a_b_atoms = walker.a_b_atoms[budget_idx];
    let walker_used = walker.walker_used[budget_idx];

    if a_b_atoms == 0 {
        return ScoreAtBudget {
            budget,
            a_b_atoms: 0,
            importance: 0.0,
            coverage: 0.0,
            completion: f64::NAN,
            score: 0.0,
            walker_used,
        };
    }

    let ideal_denom: f64 = (1..=a_b_atoms).map(|k| 1.0 / k as f64).sum();

    let mut importance_num = 0.0;
    let mut coverage_sum = 0.0;
    let mut delivered_completion_sum = 0.0;
    let mut delivered_completion_n = 0usize;
    for row in &ctx.ns_rows {
        let completion = completion_for_row(&row.atoms, walker_cum);
        if completion == 0.0 {
            continue;
        }
        if row.exp_t <= budget {
            delivered_completion_sum += completion;
            delivered_completion_n += 1;
        }
        for (i, atom) in row.atoms.iter().enumerate() {
            let rank = row.rank_start + i;
            let damped = atom_credit(atom, walker_cum) * completion;
            importance_num += damped / rank as f64;
            if rank <= a_b_atoms {
                coverage_sum += damped;
            }
        }
    }

    let importance = (importance_num / ideal_denom).min(1.0);
    let coverage = coverage_sum / a_b_atoms as f64;
    let completion = if delivered_completion_n == 0 {
        f64::NAN
    } else {
        delivered_completion_sum / delivered_completion_n as f64
    };
    let score = (importance * coverage).sqrt();
    ScoreAtBudget {
        budget,
        a_b_atoms,
        importance,
        coverage,
        completion,
        score,
        walker_used,
    }
}

/// Byte-weighted completion of an NS batch under a walker state.
/// `Σ min(walker, ns) / Σ ns` over the batch's atoms — the
/// `completion(B_i)` factor in `damped_credit`. Returns 0 when nothing
/// is delivered, 1 when the batch is fully covered.
fn completion_for_row(ns_atoms: &[GradedAtom], walker_cum: &BTreeMap<&Atom, usize>) -> f64 {
    if ns_atoms.is_empty() {
        return 0.0;
    }
    let mut delivered = 0usize;
    let mut total = 0usize;
    for atom in ns_atoms {
        let walker_bytes = walker_cum.get(&atom.atom).copied().unwrap_or(0);
        let ns_bytes = atom.bytes.max(1);
        delivered += walker_bytes.min(ns_bytes);
        total += ns_bytes;
    }
    if total == 0 {
        return 0.0;
    }
    delivered as f64 / total as f64
}

fn atom_credit(atom: &GradedAtom, walker_cum: &BTreeMap<&Atom, usize>) -> f64 {
    let walker_bytes = walker_cum.get(&atom.atom).copied().unwrap_or(0);
    let ns_bytes = atom.bytes.max(1);
    ((walker_bytes.min(ns_bytes) as f64) / (ns_bytes as f64)).min(1.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrivalStatus {
    /// Primary-budget damped credit ≥ [`REACH_THRESHOLD`].
    Reached,
    /// Primary-budget damped credit in `[MISSING_FLOOR, REACH_THRESHOLD)`.
    Partial,
    /// Primary-budget damped credit < [`MISSING_FLOOR`].
    Missing,
}

impl ArrivalStatus {
    fn label(self) -> &'static str {
        match self {
            ArrivalStatus::Reached => "reached",
            ArrivalStatus::Partial => "partial",
            ArrivalStatus::Missing => "missing",
        }
    }
}

struct Arrival {
    /// Primary-budget byte-credit averaged over NS atoms (atom-count
    /// weighted). Uses `walker_cum` at `PRIMARY_BUDGET` — describes
    /// the row's state at the optimization target, not at T_max.
    credit: f64,
    /// Status derived from the primary-budget damped credit (`credit
    /// × completion`) — the same quantity `Score(B)` consumes per
    /// atom.
    status: ArrivalStatus,
    /// Per-row priority at the three reported budgets, in order:
    /// 1K (low), 3K (primary), 9K (high). Each entry: `Σ over atoms
    /// in row: r(rank(a)) × (1 − damped_credit(a))` evaluated at that
    /// budget's `walker_cum` and `completion`. No rank cap — atoms
    /// past `|A_B|` enter via `1/rank`, matching the Importance
    /// numerator in `compute_score_at`. `[PRIORITY_AT_PRIMARY]` is
    /// the sort key throughout opportunity / row-id rollups; `[0]`
    /// and `[2]` surface as `gap@1k` / `gap@9k` columns.
    priority_at_b: [f64; 3],
}

/// Index into [`Arrival::priority_at_b`] for the primary-budget tier.
/// The low / high tiers are accessed by destructuring (their indices
/// only matter at the destructure site).
pub(crate) const PRIORITY_AT_PRIMARY: usize = 1;

fn arrival_infos(ctx: &BuildCtx, walker: &WalkerSnapshots) -> Vec<Arrival> {
    let primary_cum = &walker.cums[PRIMARY_BUDGET_INDEX];
    let reported_cums = [
        &walker.cums[LOW_BUDGET_INDEX],
        &walker.cums[PRIMARY_BUDGET_INDEX],
        &walker.cums[HIGH_BUDGET_INDEX],
    ];
    ctx.ns_rows
        .iter()
        .map(|row| {
            let credit = credit_for_ns_atoms(&row.atoms, primary_cum);
            let completion = completion_for_row(&row.atoms, primary_cum);
            // Status anchors on damped credit (= credit × completion),
            // matching what `Score(B)` actually consumes. A row with
            // many small atoms covered but one large atom missing has
            // high `credit` and low `completion`; classifying on
            // `credit` alone called it `Reached` and dropped it from
            // every rollup despite Score depressing it heavily.
            let status = classify(credit * completion);
            let priority_at_b: [f64; 3] = std::array::from_fn(|i| {
                let cum = reported_cums[i];
                let row_completion = completion_for_row(&row.atoms, cum);
                priority_for_row(row, cum, row_completion)
            });
            Arrival {
                credit,
                status,
                priority_at_b,
            }
        })
        .collect()
}

/// Walker cumulative byte-max map up to (and including) walker batches
/// with `seen_t ≤ t_max`. Borrowed `&Atom` keys — keys live in
/// `ctx.walker_rows`, so the returned map can't outlive `ctx`.
fn walker_cum_at<'a>(ctx: &'a BuildCtx<'_>, t_max: usize) -> BTreeMap<&'a Atom, usize> {
    let mut cumulative: BTreeMap<&Atom, usize> = BTreeMap::new();
    for wr in &ctx.walker_rows {
        if wr.seen_t > t_max {
            break;
        }
        for wa in &wr.atoms {
            let entry = cumulative.entry(&wa.atom).or_insert(0);
            if wa.bytes > *entry {
                *entry = wa.bytes;
            }
        }
    }
    cumulative
}

fn credit_for_ns_atoms(ns_atoms: &[GradedAtom], walker_cum: &BTreeMap<&Atom, usize>) -> f64 {
    if ns_atoms.is_empty() {
        return 0.0;
    }
    let total: f64 = ns_atoms.iter().map(|na| atom_credit(na, walker_cum)).sum();
    total / ns_atoms.len() as f64
}

/// Per-row priority = `Σ over atoms in row: r(rank(a)) × (1 −
/// damped_credit(a))` evaluated at the given budget's `walker_cum`
/// and `completion`. No rank cap: every atom contributes via
/// `1/rank`, including atoms past `|A_B|` — they enter `Score(B)`
/// through the Importance numerator (which `compute_score_at` does
/// **not** rank-cap), so closing their gap can still lift `Score(B)`
/// until Importance saturates at 1. The `1/rank` weighting fades
/// late-atom contributions naturally.
fn priority_for_row(row: &NsRow, walker_cum: &BTreeMap<&Atom, usize>, completion: f64) -> f64 {
    row.atoms
        .iter()
        .enumerate()
        .map(|(i, atom)| {
            let rank = row.rank_start + i;
            let damped = atom_credit(atom, walker_cum) * completion;
            (1.0 - damped).max(0.0) / rank as f64
        })
        .sum()
}

// ---- report ------------------------------------------------------------

/// Collapse position-bearing suffixes in a descriptor down to `<n>` so
/// semantically-similar batches group under one pattern. Shape-specific
/// matching: only known walker descriptor templates with positional
/// suffixes are touched. Path-only descriptors (e.g.
/// `crate-doc lede in src/lib.rs`) pass through verbatim — their
/// trailing characters come from user-controlled paths and shouldn't be
/// rewritten.
///
/// Recognized shapes (extend if a new walker adds a positional descriptor):
/// - `(pub item|pub-item doc lede|pub-item doc body|export|export doc)
///    at <path>:<line>` → `… at <path>:<n>`
/// - `<path>.md section #<index>` → `<path>.md section #<n>`
fn pattern_template(descriptor: &str) -> String {
    const LINE_PREFIXES: &[&str] = &[
        "pub item at ",
        "pub-item doc lede at ",
        "pub-item doc body at ",
        "export at ",
        "export doc at ",
    ];
    for prefix in LINE_PREFIXES {
        if let Some(rest) = descriptor.strip_prefix(prefix)
            && let Some((path_part, last)) = rest.rsplit_once(':')
            && !last.is_empty()
            && last.bytes().all(|b| b.is_ascii_digit())
        {
            return format!("{prefix}{path_part}:<n>");
        }
    }
    if let Some((before, n)) = descriptor.rsplit_once(" section #")
        && let Some((_, ext)) = before.rsplit_once('.')
        && ext.eq_ignore_ascii_case("md")
        && !n.is_empty()
        && n.bytes().all(|b| b.is_ascii_digit())
    {
        return format!("{before} section #<n>");
    }
    // Collapse " #<digits> " mid-descriptor (e.g. chunked names-surface
    // batches: "export names surface #2 in src/types/helpers.ts" →
    // "export names surface #<n> in src/types/helpers.ts"). Restricted to
    // middle-of-string (digits followed by a space) so the markdown-only
    // " section #N" suffix handler above keeps owning the trailing case.
    if let Some((idx, _)) = descriptor.match_indices(" #").find(|(idx, _)| {
        let after = &descriptor[idx + 2..];
        let digits_end = after.bytes().take_while(|b| b.is_ascii_digit()).count();
        digits_end > 0 && after.as_bytes().get(digits_end) == Some(&b' ')
    }) {
        let after = &descriptor[idx + 2..];
        let digits_end = after.bytes().take_while(|b| b.is_ascii_digit()).count();
        let before = &descriptor[..idx];
        let rest = &after[digits_end..];
        return format!("{before} #<n>{rest}");
    }
    descriptor.to_string()
}

/// Walker descriptors embed absolute paths (e.g. `"crate-doc lede in
/// /abs/.../tests/fixtures/log/src/lib.rs"`); stripping the fixture
/// root makes reports diff-stable across checkouts.
fn strip_fixture_root(descriptor: &str, fixture_root: &Path) -> String {
    let prefix = format!("{}/", fixture_root.display());
    descriptor.replace(&prefix, "")
}

fn classify(credit: f64) -> ArrivalStatus {
    if credit < MISSING_FLOOR {
        ArrivalStatus::Missing
    } else if credit < REACH_THRESHOLD {
        ArrivalStatus::Partial
    } else {
        ArrivalStatus::Reached
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use crate::content::BatchContent;
    use crate::render::Cost;
    use crate::schedule_types::{Atom, CandidateBatch, ScheduledBatch};

    use super::{
        CandidateHintKind, CandidateLossReason, CandidateRow, ExactOverlapBucket, GradedAtom,
        NsRow, WalkerRow, WalkerWasteRow, candidate_hint, off_ns_attribution,
        top_suppression_opportunities,
    };

    fn line_atom(line: usize, bytes: usize) -> GradedAtom {
        GradedAtom {
            atom: Atom::Line {
                path: PathBuf::from("src/lib.rs"),
                line,
            },
            bytes,
        }
    }

    fn path_line_atom(path: &str, line: usize) -> GradedAtom {
        GradedAtom {
            atom: Atom::Line {
                path: PathBuf::from(path),
                line,
            },
            bytes: 1,
        }
    }

    fn fs_atom(parent: &str, entry: &str) -> GradedAtom {
        GradedAtom {
            atom: Atom::Fs {
                parent: PathBuf::from(parent),
                entry: entry.to_string(),
            },
            bytes: 1,
        }
    }

    fn ns_row(atoms: Vec<GradedAtom>) -> NsRow {
        NsRow {
            id: "1.1".to_string(),
            atoms,
            exp_t: 1,
            rank_start: 1,
        }
    }

    fn scheduled_batch(descriptor: &str) -> ScheduledBatch {
        ScheduledBatch {
            position: 1,
            key: descriptor.to_string(),
            descriptor: descriptor.to_string(),
            cost_tokens: 1,
            cum_tokens: 10,
            content: BatchContent::Lines { spans: vec![] },
        }
    }

    fn candidate_batch(descriptor: &str) -> CandidateBatch {
        CandidateBatch {
            key: descriptor.to_string(),
            predecessor: None,
            descriptor: descriptor.to_string(),
            content: BatchContent::Lines { spans: vec![] },
        }
    }

    fn candidate_batch_with_predecessor(descriptor: &str, predecessor: &str) -> CandidateBatch {
        CandidateBatch {
            key: descriptor.to_string(),
            predecessor: Some(predecessor.to_string()),
            descriptor: descriptor.to_string(),
            content: BatchContent::Lines { spans: vec![] },
        }
    }

    fn candidate_row<'a>(
        batch: &'a CandidateBatch,
        atoms: Vec<GradedAtom>,
        scheduled: bool,
    ) -> CandidateRow<'a> {
        CandidateRow {
            atoms,
            final_cost: Cost {
                tokens: 1,
                bytes: 1,
            },
            scheduled,
            batch,
        }
    }

    fn hint<'a>(
        ns: &NsRow,
        walker_rows: &[WalkerRow<'a>],
        candidate_rows: &[CandidateRow<'a>],
    ) -> super::CandidateHint {
        candidate_hint(
            ns,
            walker_rows,
            candidate_rows,
            &BTreeSet::new(),
            100,
            PathBuf::from(".").as_path(),
        )
    }

    /// Heterogeneous batch: one Full atom of a long line + four Ellipsis
    /// atoms of short lines. Atom-count attribution would say off_ratio
    /// = 4/5 = 0.80 (and off_tokens scaled to ~80% of cost). Marginal
    /// attribution credits each atom with its real marginal cost: the
    /// Full line dominates, so the off_ratio is much lower. Concrete
    /// numbers used here mirror the rendered token weights you'd get
    /// from `format_line_row` (long-line ≈ 50 tokens, ellipsis ≈ 1
    /// token).
    #[test]
    fn divergence_off_ns_attribution_marginal_weighted() {
        // Atom 1 (line 1) is on-NS, costs 50 tokens; atoms 2..=5 are
        // off-NS, each costs 1 token.
        let atoms = vec![
            line_atom(1, 50),
            line_atom(2, 1),
            line_atom(3, 1),
            line_atom(4, 1),
            line_atom(5, 1),
        ];
        let atom_token_costs = vec![50usize, 1, 1, 1, 1];
        let ns_atom_set: BTreeSet<Atom> = [Atom::Line {
            path: PathBuf::from("src/lib.rs"),
            line: 1,
        }]
        .into_iter()
        .collect();

        let (off_ratio, off_tokens) =
            off_ns_attribution(&atoms, &atom_token_costs, &ns_atom_set).expect("non-empty");

        // Off atoms = lines 2..=5, marginal cost 4. Total marginal 54.
        assert_eq!(off_tokens, 4);
        assert!(
            (off_ratio - 4.0 / 54.0).abs() < 1e-9,
            "off_ratio = {off_ratio}, expected ≈ {}",
            4.0 / 54.0
        );
        // Sanity: atom-count attribution would be 4/5 = 0.80, so the
        // marginal-weighted ratio is ~10× lower — the Full atom carries
        // the spend.
        assert!(off_ratio < 0.5);
    }

    /// Refinement-over-ancestor case: line 1 is in NS but the walker
    /// batch refines a previously-rendered line, so its marginal cost
    /// is tiny. The off-NS atom (line 2) carries most of the marginal
    /// spend. Without per-atom marginals, the on-NS atom's *fresh*
    /// cost would dominate the denominator and skew off_ratio toward
    /// zero — codex round-2 finding. This asserts the fix.
    #[test]
    fn divergence_off_ns_attribution_handles_refinement() {
        // Line 1 (on-NS): refinement contributes only 5 tokens delta.
        // Line 2 (off-NS): fresh full line contributes 30 tokens.
        let atoms = vec![line_atom(1, 50), line_atom(2, 30)];
        let atom_token_costs = vec![5usize, 30];
        let ns_atom_set: BTreeSet<Atom> = [Atom::Line {
            path: PathBuf::from("src/lib.rs"),
            line: 1,
        }]
        .into_iter()
        .collect();

        let (off_ratio, off_tokens) =
            off_ns_attribution(&atoms, &atom_token_costs, &ns_atom_set).expect("non-empty");

        // Off-NS atom contributed 30 tokens of marginal spend; total 35.
        assert_eq!(off_tokens, 30);
        let expected = 30.0 / 35.0;
        assert!(
            (off_ratio - expected).abs() < 1e-9,
            "off_ratio = {off_ratio}, expected ≈ {expected}"
        );
        // Most of the batch's marginal spend was off-NS — the rollup
        // should rank this batch high. A fresh-cost-weighted formula
        // would have given off_ratio = 30/80 ≈ 0.375; marginal gives
        // ≈ 0.857.
        assert!(off_ratio > 0.8);
    }

    /// All atoms are pure refinements (every cost is zero) — no marginal
    /// spend means there's nothing to attribute. Returns `None`.
    #[test]
    fn divergence_off_ns_attribution_skips_zero_marginal() {
        let atoms = vec![line_atom(1, 10), line_atom(2, 10)];
        let atom_token_costs = vec![0usize, 0];
        let ns_atom_set: BTreeSet<Atom> = BTreeSet::new();
        assert!(off_ns_attribution(&atoms, &atom_token_costs, &ns_atom_set).is_none());
    }

    #[test]
    fn divergence_candidate_hint_prioritizes_scheduled_bbox() {
        let ns = ns_row(vec![path_line_atom("src/lib.rs", 10)]);
        let scheduled_batch = scheduled_batch("scheduled exact");
        let candidate_batch = candidate_batch("unscheduled exact");
        let walker_rows = vec![WalkerRow {
            atoms: vec![path_line_atom("src/lib.rs", 10)],
            atom_token_costs: vec![1],
            seen_t: 10,
            batch: &scheduled_batch,
        }];
        let candidate_rows = vec![candidate_row(
            &candidate_batch,
            vec![path_line_atom("src/lib.rs", 10)],
            false,
        )];

        let hint = hint(&ns, &walker_rows, &candidate_rows);

        assert_eq!(hint.kind, CandidateHintKind::ScheduledBbox);
        assert_eq!(
            hint.exact_overlap.expect("bbox").bucket(),
            ExactOverlapBucket::Full
        );
        assert!(hint.cell.contains("[scheduled bbox exact=1/1]"));
    }

    #[test]
    fn divergence_candidate_hint_uses_unscheduled_bbox_before_same_file() {
        let ns = ns_row(vec![path_line_atom("src/lib.rs", 10)]);
        let scheduled_batch = scheduled_batch("scheduled same-file");
        let candidate_batch = candidate_batch("unscheduled exact");
        let walker_rows = vec![WalkerRow {
            atoms: vec![path_line_atom("src/lib.rs", 99)],
            atom_token_costs: vec![1],
            seen_t: 10,
            batch: &scheduled_batch,
        }];
        let candidate_rows = vec![candidate_row(
            &candidate_batch,
            vec![path_line_atom("src/lib.rs", 10)],
            false,
        )];

        let hint = hint(&ns, &walker_rows, &candidate_rows);

        assert_eq!(hint.kind, CandidateHintKind::UnscheduledBbox);
        assert_eq!(
            hint.exact_overlap.expect("bbox").bucket(),
            ExactOverlapBucket::Full
        );
        assert!(hint.cell.contains("[unscheduled bbox exact=1/1]"));
    }

    #[test]
    fn divergence_candidate_hint_reports_same_file_when_bbox_misses() {
        let ns = ns_row(vec![path_line_atom("src/lib.rs", 10)]);
        let scheduled_batch = scheduled_batch("scheduled same-file");
        let walker_rows = vec![WalkerRow {
            atoms: vec![path_line_atom("src/lib.rs", 99)],
            atom_token_costs: vec![1],
            seen_t: 10,
            batch: &scheduled_batch,
        }];

        let hint = hint(&ns, &walker_rows, &[]);

        assert_eq!(hint.kind, CandidateHintKind::ScheduledSameFile);
        assert!(hint.exact_overlap.is_none());
        assert!(hint.cell.contains("[scheduled same-file]"));
    }

    #[test]
    fn divergence_candidate_hint_splits_bbox_exact_overlap() {
        let ns = ns_row(vec![
            path_line_atom("src/lib.rs", 10),
            path_line_atom("src/lib.rs", 12),
        ]);
        let scheduled_batch = scheduled_batch("scheduled wrong slice");
        let walker_rows = vec![WalkerRow {
            // Inside the NS bbox (10..=12), but not an exact NS atom.
            atoms: vec![path_line_atom("src/lib.rs", 11)],
            atom_token_costs: vec![1],
            seen_t: 10,
            batch: &scheduled_batch,
        }];

        let hint = hint(&ns, &walker_rows, &[]);

        assert_eq!(hint.kind, CandidateHintKind::ScheduledBbox);
        assert_eq!(
            hint.exact_overlap.expect("bbox").bucket(),
            ExactOverlapBucket::None
        );
        assert!(hint.cell.contains("exact=0/2"));
    }

    #[test]
    fn divergence_candidate_hint_surfaces_better_unscheduled_bbox() {
        let ns = ns_row(vec![
            path_line_atom("src/lib.rs", 10),
            path_line_atom("src/lib.rs", 12),
        ]);
        let scheduled_batch = scheduled_batch("scheduled wrong slice");
        let candidate_batch = candidate_batch("unscheduled exact");
        let walker_rows = vec![WalkerRow {
            atoms: vec![path_line_atom("src/lib.rs", 11)],
            atom_token_costs: vec![1],
            seen_t: 10,
            batch: &scheduled_batch,
        }];
        let candidate_rows = vec![candidate_row(
            &candidate_batch,
            vec![
                path_line_atom("src/lib.rs", 10),
                path_line_atom("src/lib.rs", 12),
            ],
            false,
        )];

        let hint = hint(&ns, &walker_rows, &candidate_rows);

        assert_eq!(hint.kind, CandidateHintKind::ScheduledBbox);
        assert!(hint.cell.contains("better unscheduled exact=2/2"));
        assert!(hint.better_unscheduled.is_some());
    }

    #[test]
    fn divergence_candidate_hint_names_unscheduled_predecessor() {
        let ns = ns_row(vec![path_line_atom("src/lib.rs", 10)]);
        let parent = candidate_batch("parent names surface");
        let child = candidate_batch_with_predecessor("child body", "parent names surface");
        let candidate_rows = vec![
            candidate_row(&parent, vec![path_line_atom("src/lib.rs", 1)], false),
            candidate_row(&child, vec![path_line_atom("src/lib.rs", 10)], false),
        ];

        let hint = hint(&ns, &[], &candidate_rows);

        assert_eq!(hint.kind, CandidateHintKind::UnscheduledBbox);
        let loss = hint.loss.expect("unscheduled loss");
        assert_eq!(loss.reason, CandidateLossReason::PredecessorNotScheduled);
        assert_eq!(loss.predecessor.as_deref(), Some("parent names surface"));
        assert!(
            hint.cell
                .contains("predecessor not scheduled: parent names surface")
        );
    }

    #[test]
    fn divergence_candidate_hint_separates_fs_only_and_no_discovered() {
        let fs = ns_row(vec![fs_atom(".", "src")]);
        let no_candidate = ns_row(vec![path_line_atom("src/lib.rs", 10)]);

        let fs_hint = hint(&fs, &[], &[]);
        let no_candidate_hint = hint(&no_candidate, &[], &[]);

        assert_eq!(fs_hint.kind, CandidateHintKind::FsOnly);
        assert_eq!(fs_hint.cell, "fs-only");
        assert_eq!(
            no_candidate_hint.kind,
            CandidateHintKind::NoDiscoveredCandidate
        );
        assert_eq!(no_candidate_hint.cell, "no discovered line candidate");
    }

    use super::pattern_template;

    #[test]
    fn divergence_pattern_template_collapses_known_shapes() {
        assert_eq!(
            pattern_template("pub item at src/lib.rs:475"),
            "pub item at src/lib.rs:<n>"
        );
        assert_eq!(
            pattern_template("pub-item doc lede at src/lib.rs:1478"),
            "pub-item doc lede at src/lib.rs:<n>"
        );
        assert_eq!(
            pattern_template("pub-item doc body at src/lib.rs:1478"),
            "pub-item doc body at src/lib.rs:<n>"
        );
        assert_eq!(
            pattern_template("export at source/types/hooks.ts:48"),
            "export at source/types/hooks.ts:<n>"
        );
        assert_eq!(
            pattern_template("export doc at source/errors/NonError.ts:6"),
            "export doc at source/errors/NonError.ts:<n>"
        );
        assert_eq!(
            pattern_template("README.md section #3"),
            "README.md section #<n>"
        );
        assert_eq!(
            pattern_template("docs/changelog.md section #5"),
            "docs/changelog.md section #<n>"
        );
    }

    #[test]
    fn divergence_pattern_template_handles_mixed_case_md() {
        assert_eq!(
            pattern_template("README.MD section #1"),
            "README.MD section #<n>"
        );
        assert_eq!(
            pattern_template("readme.Md section #0"),
            "readme.Md section #<n>"
        );
    }

    #[test]
    fn divergence_pattern_template_passes_through_path_only_descriptors() {
        assert_eq!(
            pattern_template("crate-doc lede in src/lib.rs"),
            "crate-doc lede in src/lib.rs"
        );
        assert_eq!(
            pattern_template("[package] in Cargo.toml"),
            "[package] in Cargo.toml"
        );
        assert_eq!(
            pattern_template("README headline in README.md"),
            "README headline in README.md"
        );
        // Section discriminator only fires for `.md` paths — a non-md
        // path with the same trailing shape passes through.
        assert_eq!(
            pattern_template("not-markdown.txt section #3"),
            "not-markdown.txt section #3"
        );
    }

    fn waste_row<'a>(
        wr: &'a WalkerRow<'a>,
        descriptor_rel: &str,
        walker_t: usize,
        off_3k_tokens: usize,
        off_any_tokens: usize,
    ) -> WalkerWasteRow<'a> {
        WalkerWasteRow {
            wr,
            descriptor_rel: descriptor_rel.to_string(),
            off_any_tokens,
            off_3k_ratio: 1.0,
            off_3k_tokens,
            walker_t,
        }
    }

    fn empty_walker_row(batch: &ScheduledBatch, seen_t: usize) -> WalkerRow<'_> {
        WalkerRow {
            atoms: vec![],
            atom_token_costs: vec![],
            seen_t,
            batch,
        }
    }

    /// Each `freed@B` column must only count rows actually in B's
    /// walker prefix. A batch scheduled at `walker_t = 2500` doesn't
    /// belong in `freed@1k`. Conversely, waste at `walker_t = 5000`
    /// still contributes to `freed@9k`.
    #[test]
    fn divergence_top_suppression_opportunities_gates_by_budget_prefix() {
        let batch_a = scheduled_batch("X in foo");
        let batch_b = scheduled_batch("X in foo");
        let batch_c = scheduled_batch("X in foo");
        let walker_a = empty_walker_row(&batch_a, 600);
        let walker_b = empty_walker_row(&batch_b, 2600);
        let walker_c = empty_walker_row(&batch_c, 5100);
        let rows = vec![
            // walker_t=500: in 1K, 3K, 9K prefixes
            waste_row(&walker_a, "X in foo", 500, 100, 100),
            // walker_t=2500: NOT in 1K prefix, in 3K and 9K
            waste_row(&walker_b, "X in foo", 2500, 200, 200),
            // walker_t=5000: NOT in 1K or 3K, in 9K
            waste_row(&walker_c, "X in foo", 5000, 50, 50),
        ];

        let opps = top_suppression_opportunities(&rows, 5);

        assert_eq!(opps.len(), 1);
        let opp = &opps[0];
        assert_eq!(opp.pattern, "X in foo");
        // Only batch A is in the 1K prefix.
        assert_eq!(opp.freed_at_low, 100);
        // Batches A and B are in the 3K prefix.
        assert_eq!(opp.freed_at_primary, 300);
        // All three are in the 9K prefix.
        assert_eq!(opp.freed_at_high, 350);
        // `batches` counts only primary-actionable rows (in 3K prefix)
        // — A and B.
        assert_eq!(opp.batches, 2);
    }

    /// A boundary-crossing batch (walker_t < 3K < walker_t + cost)
    /// is the row that *stopped* the 3K prefix under the no-fallback
    /// scheduler. Suppressing it lets the next-best batch fit, so it
    /// counts toward `freed@3k` — same convention as the walker-
    /// waste detail table.
    #[test]
    fn divergence_top_suppression_opportunities_includes_boundary_crossers() {
        let in_prefix_batch = scheduled_batch("X in foo");
        let crosser_batch = scheduled_batch("X in foo");
        let in_prefix = empty_walker_row(&in_prefix_batch, 600);
        // walker_t = 2950, seen_t = 3050: straddles the 3K boundary.
        let crosser = empty_walker_row(&crosser_batch, 3050);
        let rows = vec![
            waste_row(&in_prefix, "X in foo", 500, 100, 100),
            waste_row(&crosser, "X in foo", 2950, 200, 200),
        ];

        let opps = top_suppression_opportunities(&rows, 5);

        assert_eq!(opps.len(), 1);
        let opp = &opps[0];
        // Both in-prefix and crosser contribute to freed@3k; crosser
        // is excluded from freed@1k since walker_t > 1000.
        assert_eq!(opp.freed_at_low, 100);
        assert_eq!(opp.freed_at_primary, 300);
        assert_eq!(opp.batches, 2);
        assert_eq!(opp.freed_at_high, 300);
    }

    /// A pattern whose only waste rows ship past 9K is dropped — no
    /// budget in the report's vector can be freed by suppressing it.
    #[test]
    fn divergence_top_suppression_opportunities_drops_post_9k_waste() {
        let batch = scheduled_batch("Y in bar");
        let walker = empty_walker_row(&batch, 9500);
        let rows = vec![waste_row(&walker, "Y in bar", 9300, 0, 0)];
        let opps = top_suppression_opportunities(&rows, 5);
        assert!(opps.is_empty());
    }

    /// `freed@B` must only credit truly off-NS atoms (`off_any`), not
    /// atoms NS wants past `B` (`off_3k` minus `off_any`). The score
    /// formula has no rank cap on the Importance numerator, so
    /// delivered NS-past-3K atoms still contribute to `Score(3000)`
    /// — counting them as "freed" would steer iterators toward
    /// suppressing batches that aren't actually waste.
    #[test]
    fn divergence_top_suppression_opportunities_freed_counts_only_off_any() {
        let mixed_batch = scheduled_batch("M in foo");
        let pure_batch = scheduled_batch("P in foo");
        let mixed = empty_walker_row(&mixed_batch, 600);
        let pure = empty_walker_row(&pure_batch, 700);
        let rows = vec![
            // Mixed: 200 off-3K marginal tokens, but only 50 are
            // truly off-NS (150 are NS-past-3K — delivered early).
            waste_row(&mixed, "M in foo", 500, 200, 50),
            // Pure: all 100 off-3K tokens are truly off-NS.
            waste_row(&pure, "P in foo", 600, 100, 100),
        ];

        let opps = top_suppression_opportunities(&rows, 5);

        let mixed_opp = opps
            .iter()
            .find(|o| o.pattern == "M in foo")
            .expect("mixed pattern should surface — it has off_any > 0");
        assert_eq!(
            mixed_opp.freed_at_primary, 50,
            "freed@3k credits only off_any (50), not off_3k (200)"
        );
        assert_eq!(mixed_opp.evidence, "off_3k=200");

        let pure_opp = opps
            .iter()
            .find(|o| o.pattern == "P in foo")
            .expect("pure pattern should surface");
        assert_eq!(pure_opp.freed_at_primary, 100);
        assert_eq!(pure_opp.evidence, "off_3k=100");

        // Pure pattern sorts ahead — it has 100 freed vs mixed's 50.
        assert_eq!(opps[0].pattern, "P in foo");
    }
}
