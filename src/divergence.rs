//! Divergence metric + report generator. Compares a walker `Schedule`
//! run at `T_max` to a frozen `NorthStar`. Two artifacts:
//!
//! - `Scores`: `sim` (integral similarity scalar) + per-NS-batch
//!   buckets `reached + partial + missing == total_ns` and
//!   `early` / `late` within `reached`.
//! - Markdown `Report` (`tests/divergence/<fixture>.md`): line 1 is the
//!   grep-able score line; body is per-tier rollup, arrival ledger,
//!   and walker-waste. Stable ordering; perfect alignment ⇒ 1-line file.
//!
//! NS `exp_t` is the cumulative marginal cost of applying NS batches in
//! rank order to one shared `RenderedTree` — same accounting as
//! `simulate_ns`, so predecessor refinements over ellipsis lines cost
//! only their delta. Under scheduler prefix-monotonicity, walker
//! sub-budget behavior is the prefix of the T_max schedule with
//! `cum_tokens ≤ t`, so we run the walker once.
//!
//! ## Score line
//!
//! `Sim=X.XXX Reached=R/T Early=E Late=L Partial=P Missing=M Used=U/B`
//!
//! - `Sim ∈ [0,1]` — `∫ w(t)·overlap(t) dt / ∫ w(t) dt`,
//!   `w(t) = exp(−t/τ)`, `τ=2000`. `overlap(t)` averages graded credit
//!   over NS atoms *reachable at t* (denominator excludes
//!   unreachable-by-construction NS atoms, so low-t is meaningful).
//! - `R/T` reached / total NS batches; `E`, `L`, `P`, `M` are subsets.
//! - `U/B` walker tokens consumed / token budget. Gap = budget the
//!   prefix-monotone scheduler left on the table.
//!
//! ## Atoms + credit
//!
//! Atoms are `Line(path, line)` or `Fs(parent, entry)`. Each has a
//! `bytes` footprint per render (Full = source-line length, Truncated =
//! regex match end, Ellipsis = 1, Fs = 1; floored at 1 so "rendered" is
//! distinguishable from "absent"). Credit between matched walker + NS
//! atoms is `min(walker, ns) / max(ns, 1)`, capped at 1.0. Walker
//! showing strictly more bytes than NS asked → fully credited but
//! flagged via the `+over` row annotation.
//!
//! ## Tier rollup
//!
//! Per major-id prefix (`1.x`, `2.x`, …): `batches | reached | partial
//! | missing | avg_credit`. Lets a reader spot which tier the walker
//! falls off.
//!
//! ## Arrival ledger
//!
//! One row per non-aligned-or-partial NS batch:
//! `id | exp_t | reached_t | delta_t | credit | status | descriptor`.
//!
//! - `exp_t` — NS-cumulative tokens at that batch (when NS expects it).
//! - `reached_t` — walker `cum_tokens` at the batch where this NS
//!   batch's credit first reaches `REACH_THRESHOLD = 0.8`. `—` if
//!   never reached.
//! - `delta_t` — `reached_t - exp_t` with sign. Negative = early,
//!   positive = late.
//! - `credit` — final-state byte-range credit averaged over NS atoms.
//! - `status` ∈ `{aligned, early, late, partial, missing}` with
//!   optional `+over` annotation (only on `aligned` rows). Bands:
//!   `credit < 0.5` → missing, `< 0.8` → partial, else reached;
//!   within reached, timing classifies via `EARLY_FACTOR = 0.7` /
//!   `LATE_FACTOR = 1.3` on `delta_t / exp_t`.
//!
//! ## Walker waste
//!
//! Walker batches whose `off_tokens` exceeds `UNMAPPED_COST_THRESHOLD`,
//! sorted descending. Surfaces both pure-waste (`off_ratio = 1.00`)
//! and mixed-intersection batches (some on-NS atoms but most spend
//! off-script).
//!
//! Off-NS attribution is **per-atom marginal**: each atom carries the
//! token delta it actually contributed to the batch's marginal cost
//! (refinement-over-ancestor lines pay the truncated delta;
//! already-listed FS entries pay 0), captured by driving a parallel
//! walker tree forward in schedule order. The waste columns sum
//! exactly the off-NS atoms' marginal contributions — no
//! ratio-times-cost approximation.
//!
//! - `off_tokens` — sum of off-NS atoms' marginal token costs.
//! - `off_ratio` — `off_tokens / total_marginal_tokens`. Fraction of
//!   the batch's marginal token spend not paired with any NS atom.
//! - `cost` — total marginal cost of the walker batch (= sum of all
//!   atoms' marginal contributions, on- and off-NS).
//! - `first_t` — walker `cum_tokens` when this batch was scheduled.
//! - `batch` — descriptor with fixture root stripped.
//!
//! Preceded by a *rollup* table grouping waste rows by descriptor
//! pattern (e.g. `pub-item doc at src/lib.rs:<n>` collapses 14 per-line
//! rows into one). Surfaces systemic walker over-spend that the
//! per-batch table buries; elided when no pattern groups two-or-more
//! rows.

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
use render::off_ns_attribution;
use synthesis::{
    ReportRow, predecessor_groups_from_refs, ranking_loss, report_rows, report_summary, row_ids,
    sim_gap_weight, top_opportunities,
};

/// Time-weight half-life (tokens). `w(t) = exp(−t / τ)`. First 2000
/// tokens carry ~63% of the mass; first 6000 ~95%.
const TAU: f64 = 2000.0;

/// Credit threshold: NS batch counted as `reached` iff final-state
/// credit ≥ this.
const REACH_THRESHOLD: f64 = 0.8;

/// Credit threshold: NS batch counted as `missing` iff final-state
/// credit < this. Rows in `[MISSING_FLOOR, REACH_THRESHOLD)` are
/// `partial` everywhere (headline + row label).
const MISSING_FLOOR: f64 = 0.5;

/// Timing classification within the `reached` bucket: a reached batch
/// is `early` / `late` when `(seen_t - exp_t) / exp_t` passes these
/// thresholds. Otherwise `aligned`.
const EARLY_FACTOR: f64 = 0.7;
const LATE_FACTOR: f64 = 1.3;

/// Report threshold: walker-waste rows elide batches below this cost.
const UNMAPPED_COST_THRESHOLD: usize = 50;
const WASTE_DETAIL_LIMIT: usize = 10;

/// Headline scores. `reached + partial + missing == total_ns`.
#[derive(Debug, Clone, Copy)]
pub struct Scores {
    pub sim: f64,
    pub total_ns: usize,
    /// Final credit ≥ [`REACH_THRESHOLD`].
    pub reached: usize,
    /// Reached and walker delivered meaningfully earlier than expected.
    pub early: usize,
    /// Reached and walker delivered meaningfully later than expected.
    pub late: usize,
    /// Final credit in `[MISSING_FLOOR, REACH_THRESHOLD)` — present but
    /// diluted.
    pub partial: usize,
    /// Final credit < [`MISSING_FLOOR`].
    pub missing: usize,
}

/// Compute scores. `schedule` is expected to be a full-cap walker run.
pub fn score(ns: &NorthStar, schedule: &Schedule, fixture_root: &Path) -> Result<Scores> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let arrivals = arrival_infos(&ctx);
    Ok(compute_scores(&ctx, &arrivals))
}

/// Generate the markdown divergence report (one per fixture).
pub fn generate_divergence_report(
    ns: &NorthStar,
    schedule: &Schedule,
    fixture_root: &Path,
) -> Result<String> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let arrivals = arrival_infos(&ctx);
    let scores = compute_scores(&ctx, &arrivals);
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
    /// Major id prefix, parsed from `id`. Used for the per-tier rollup.
    tier: usize,
    atoms: Vec<GradedAtom>,
    /// Cumulative token cost up through this batch, computed as the
    /// marginal cost of applying each batch in order to a shared
    /// `RenderedTree` — same accounting as `simulate_ns`.
    exp_t: usize,
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
            ns_rows.push(NsRow {
                id: b.id.clone(),
                tier: parse_tier(&b.id),
                atoms,
                exp_t: cum,
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

/// Numeric components of a major.minor NS id (`"2.10"` → `[2, 10]`).
fn id_components(id: &str) -> Vec<usize> {
    id.split('.').filter_map(|s| s.parse().ok()).collect()
}

/// Major id prefix: `"1.10"` → `1`. Used for tier rollup; falls back to
/// 0 for non-numeric prefixes.
fn parse_tier(id: &str) -> usize {
    id_components(id).first().copied().unwrap_or(0)
}

// ---- scoring -----------------------------------------------------------

fn compute_scores(ctx: &BuildCtx, arrivals: &[Arrival]) -> Scores {
    let t_max = ctx.schedule.budget.max(ctx.schedule.cumulative_tokens);
    let sim = compute_sim(ctx, t_max);

    let mut reached = 0;
    let mut early = 0;
    let mut late = 0;
    let mut partial = 0;
    let mut missing = 0;

    for arrival in arrivals {
        match arrival.status {
            ArrivalStatus::Missing => missing += 1,
            ArrivalStatus::Partial => partial += 1,
            ArrivalStatus::Aligned => reached += 1,
            ArrivalStatus::Early => {
                reached += 1;
                early += 1;
            }
            ArrivalStatus::Late => {
                reached += 1;
                late += 1;
            }
        }
    }

    Scores {
        sim,
        total_ns: ctx.ns_rows.len(),
        reached,
        early,
        late,
        partial,
        missing,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrivalStatus {
    Aligned,
    Early,
    Late,
    Partial,
    Missing,
}

impl ArrivalStatus {
    fn label(self) -> &'static str {
        match self {
            ArrivalStatus::Aligned => "aligned",
            ArrivalStatus::Early => "early",
            ArrivalStatus::Late => "late",
            ArrivalStatus::Partial => "partial",
            ArrivalStatus::Missing => "missing",
        }
    }
}

struct Arrival {
    reached_t: Option<usize>,
    credit: f64,
    over: bool,
    status: ArrivalStatus,
}

fn arrival_infos(ctx: &BuildCtx) -> Vec<Arrival> {
    ctx.ns_rows
        .iter()
        .map(|row| {
            let reached_t = first_reach_t(ctx, &row.atoms);
            let (credit, over) = credit_for_ns_atoms(ctx, &row.atoms);
            let status = classify(reached_t, row.exp_t, credit);
            Arrival {
                reached_t,
                credit,
                over,
                status,
            }
        })
        .collect()
}

/// Walker cumulative byte-max map up to (and including) walker batches
/// with `seen_t ≤ t_max`. Pass `usize::MAX` for the full final state.
/// Single shared helper for `credit_for_ns_atoms` and `overlap_at`.
fn walker_cum_at<'a>(ctx: &'a BuildCtx<'_>, t_max: usize) -> BTreeMap<&'a Atom, usize> {
    let mut cumulative: BTreeMap<&Atom, usize> = BTreeMap::new();
    for wr in &ctx.walker_rows {
        if wr.seen_t > t_max {
            break;
        }
        for wa in &wr.atoms {
            let prev = cumulative.get(&wa.atom).copied().unwrap_or(0);
            if wa.bytes > prev {
                cumulative.insert(&wa.atom, wa.bytes);
            }
        }
    }
    cumulative
}

/// Cumulative walker tokens at the first walker batch after which this
/// NS batch's credit reaches [`REACH_THRESHOLD`]. `None` if the batch
/// is never reached.
fn first_reach_t(ctx: &BuildCtx, ns_atoms: &[GradedAtom]) -> Option<usize> {
    if ns_atoms.is_empty() {
        return None;
    }
    let mut cumulative: BTreeMap<&Atom, usize> = BTreeMap::new();
    for wr in &ctx.walker_rows {
        for wa in &wr.atoms {
            let prev = cumulative.get(&wa.atom).copied().unwrap_or(0);
            if wa.bytes > prev {
                cumulative.insert(&wa.atom, wa.bytes);
            }
        }
        let (credit, _over) = credit_against_cum_map(ns_atoms, &cumulative);
        if credit >= REACH_THRESHOLD {
            return Some(wr.seen_t);
        }
    }
    None
}

fn credit_for_ns_atoms(ctx: &BuildCtx, ns_atoms: &[GradedAtom]) -> (f64, bool) {
    let cumulative = walker_cum_at(ctx, usize::MAX);
    credit_against_cum_map(ns_atoms, &cumulative)
}

fn credit_against_cum_map(
    ns_atoms: &[GradedAtom],
    walker_cum: &BTreeMap<&Atom, usize>,
) -> (f64, bool) {
    if ns_atoms.is_empty() {
        return (0.0, false);
    }
    let mut total = 0.0;
    let mut over = false;
    for na in ns_atoms {
        let walker_bytes = walker_cum.get(&na.atom).copied().unwrap_or(0);
        let ns_bytes = na.bytes.max(1);
        let c = (walker_bytes.min(ns_bytes) as f64) / (ns_bytes as f64);
        total += c.min(1.0);
        if walker_bytes > na.bytes {
            over = true;
        }
    }
    (total / ns_atoms.len() as f64, over)
}

// ---- Sim (integral) ----------------------------------------------------

/// Sim = ∫ w(t)·overlap(t) dt / ∫ w(t) dt over t ∈ [0, T_max], with
/// `w(t) = exp(−t/τ)`. `overlap(t)` averages credit over NS atoms
/// *reachable at t* (ns batch `exp_t ≤ t`) — unreachable-by-construction
/// atoms don't inflate the denominator at low `t`. Eval is at the
/// *left* endpoint of each boundary segment: overlap is a right-
/// continuous step function, constant on `[a, b)` after any jump at
/// `a`, so the left endpoint captures the right integrand for `(a, b)`.
fn compute_sim(ctx: &BuildCtx, t_max: usize) -> f64 {
    let mut boundaries: BTreeSet<usize> = BTreeSet::new();
    boundaries.insert(0);
    boundaries.insert(t_max);
    for row in &ctx.ns_rows {
        if row.exp_t <= t_max {
            boundaries.insert(row.exp_t);
        }
    }
    for wr in &ctx.walker_rows {
        if wr.seen_t <= t_max {
            boundaries.insert(wr.seen_t);
        }
    }
    let bs: Vec<usize> = boundaries.into_iter().collect();

    let mut num = 0.0;
    let mut den = 0.0;
    for pair in bs.windows(2) {
        let a = pair[0] as f64;
        let b = pair[1] as f64;
        let overlap = overlap_at(ctx, pair[0]);
        let w = weighted_segment(a, b);
        num += w * overlap;
        den += w;
    }
    if den > 0.0 { num / den } else { 0.0 }
}

/// `overlap(t)` has jumps at both walker batch boundaries (numerator
/// grows as walker delivers atoms) and NS batch boundaries (denominator
/// grows as atoms become reachable). It's right-continuous at both —
/// the left-endpoint eval in `compute_sim` captures the correct
/// piecewise-constant value for `(a, b]`.
fn overlap_at(ctx: &BuildCtx, t: usize) -> f64 {
    let cumulative = walker_cum_at(ctx, t);
    let mut total_atoms = 0.0;
    let mut total_credit = 0.0;
    for row in &ctx.ns_rows {
        if row.exp_t > t {
            continue;
        }
        for na in &row.atoms {
            total_atoms += 1.0;
            let walker_bytes = cumulative.get(&na.atom).copied().unwrap_or(0);
            let ns_bytes = na.bytes.max(1);
            let c = (walker_bytes.min(ns_bytes) as f64) / (ns_bytes as f64);
            total_credit += c.min(1.0);
        }
    }
    if total_atoms == 0.0 {
        0.0
    } else {
        total_credit / total_atoms
    }
}

/// ∫_a^b exp(−t/τ) dt = τ · (e^(−a/τ) − e^(−b/τ))
fn weighted_segment(a: f64, b: f64) -> f64 {
    TAU * ((-a / TAU).exp() - (-b / TAU).exp())
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
    descriptor.to_string()
}

/// Walker descriptors embed absolute paths (e.g. `"crate-doc lede in
/// /abs/.../tests/fixtures/log/src/lib.rs"`); stripping the fixture
/// root makes reports diff-stable across checkouts.
fn strip_fixture_root(descriptor: &str, fixture_root: &Path) -> String {
    let prefix = format!("{}/", fixture_root.display());
    descriptor.replace(&prefix, "")
}

fn classify(reached_t: Option<usize>, exp_t: usize, credit: f64) -> ArrivalStatus {
    if credit < MISSING_FLOOR {
        return ArrivalStatus::Missing;
    }
    if credit < REACH_THRESHOLD {
        return ArrivalStatus::Partial;
    }
    let Some(seen) = reached_t else {
        // Unreachable in practice: credit ≥ 0.8 means at least one walker
        // batch crossed the threshold, so `first_reach_t` returns Some.
        return ArrivalStatus::Aligned;
    };
    let exp = exp_t.max(1) as f64;
    let ratio = (seen as f64 - exp_t as f64) / exp;
    if ratio <= EARLY_FACTOR - 1.0 {
        ArrivalStatus::Early
    } else if ratio >= LATE_FACTOR - 1.0 {
        ArrivalStatus::Late
    } else {
        ArrivalStatus::Aligned
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
        NsRow, WalkerRow, candidate_hint, off_ns_attribution,
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
            tier: 1,
            atoms,
            exp_t: 1,
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
}
