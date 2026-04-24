//! Divergence metric + report generator. Compares a walker `Schedule` run
//! at `T_max` to a frozen `NorthStar`, producing:
//!
//! - `Scores`: a small tuple — `sim` (integral similarity scalar), plus
//!   per-NS-batch buckets `reached + partial + missing == total_ns` and
//!   timing-breakdown `early` / `late` within the reached bucket.
//! - Markdown `Report` (`tests/divergence/<fixture>.md`): one per fixture,
//!   holistic across budgets. Line 1 is the grep-able score line; body
//!   is a per-tier rollup, an arrival ledger (one row per non-reached /
//!   mistimed NS batch), and a walker-waste section for off-NS content.
//!   Stable ordering; perfect alignment produces a short file.
//!
//! NS `exp_t` for each batch is its marginal cost applied to an evolving
//! `RenderedTree`, matching `simulate_ns` — so predecessor refinements
//! that override ellipsis lines cost only their delta. Under scheduler
//! prefix-monotonicity, the walker schedule at any `t ≤ T_max` is the
//! prefix of the `T_max` schedule with `cum_tokens ≤ t`; we run the
//! walker once and read sub-budget behavior from that trajectory.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::batch::{Batch, BatchId, ValueSignals};
use crate::content::{BatchContent, FsEntries, Render};
use crate::north_star::NorthStar;
use crate::ns_loader::resolve_content;
use crate::render::{RenderedTree, SourceCache};
use crate::schedule_types::{Atom, Schedule, ScheduledBatch};

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
    Ok(compute_scores(&ctx))
}

/// Generate the markdown divergence report (one per fixture).
pub fn generate_divergence_report(
    ns: &NorthStar,
    schedule: &Schedule,
    fixture_root: &Path,
) -> Result<String> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let scores = compute_scores(&ctx);
    Ok(format_report(&scores, &ctx))
}

// ---- graded atoms ------------------------------------------------------

/// One atom of content with its render-level byte footprint.
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
        BatchContent::Lines { spans } => {
            // Spans within a batch must be disjoint on (path, line) —
            // validator enforces it; walker emits disjoint spans by
            // construction. Overlap here is a bug; debug-assert + last
            // write wins for release-build robustness.
            let mut by_key: BTreeMap<(PathBuf, usize), Render> = BTreeMap::new();
            for span in spans {
                for line in span.start..=span.end {
                    let prev = by_key.insert((span.path.clone(), line), span.render.clone());
                    debug_assert!(
                        prev.is_none(),
                        "overlapping spans in divergence at {}:{line}",
                        span.path.display()
                    );
                }
            }
            let mut out = Vec::with_capacity(by_key.len());
            for ((path, line), render) in by_key {
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
                out.push(GradedAtom {
                    atom: Atom::Line { path, line },
                    bytes,
                });
            }
            out
        }
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
    seen_t: usize,
    batch: &'a ScheduledBatch,
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
            let batch = Batch {
                content: content.clone(),
                signals: ValueSignals::default(),
            };
            let marginal = tree.marginal_cost(&batch);
            cum += marginal.tokens;
            let batch_id = BatchId::new(pos);
            // Divergence doesn't care about predecessor-chain conflicts
            // here — that's `simulate_ns`'s job. `|_| true` accepts any
            // existing owner so the tree evolves faithfully regardless.
            let _ = tree.apply(&batch, batch_id, |_| true);
            ns_rows.push(NsRow {
                id: b.id.clone(),
                tier: parse_tier(&b.id),
                atoms,
                exp_t: cum,
            });
        }

        let walker_rows = schedule
            .batches
            .iter()
            .map(|b| {
                let atoms = atoms_from_content(&b.content, &source_cache, fixture_root);
                WalkerRow {
                    atoms,
                    seen_t: b.cum_tokens,
                    batch: b,
                }
            })
            .collect();

        Ok(Self {
            ns_rows,
            walker_rows,
            ns,
            schedule,
            fixture_root: fixture_root_buf,
        })
    }
}

/// Major id prefix: `"1.10"` → `1`. Used for tier rollup; falls back to
/// 0 for non-numeric prefixes.
fn parse_tier(id: &str) -> usize {
    id.split('.')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

// ---- scoring -----------------------------------------------------------

fn compute_scores(ctx: &BuildCtx) -> Scores {
    let t_max = ctx.schedule.budget.max(ctx.schedule.cumulative_tokens);
    let sim = compute_sim(ctx, t_max);

    let mut reached = 0;
    let mut early = 0;
    let mut late = 0;
    let mut partial = 0;
    let mut missing = 0;

    for arrival in arrival_infos(ctx) {
        if arrival.credit < MISSING_FLOOR {
            missing += 1;
        } else if arrival.credit < REACH_THRESHOLD {
            partial += 1;
        } else {
            reached += 1;
            match arrival.status {
                ArrivalStatus::Early => early += 1,
                ArrivalStatus::Late => late += 1,
                _ => {}
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
    let mut cumulative: BTreeMap<&Atom, usize> = BTreeMap::new();
    for wr in &ctx.walker_rows {
        for wa in &wr.atoms {
            let prev = cumulative.get(&wa.atom).copied().unwrap_or(0);
            if wa.bytes > prev {
                cumulative.insert(&wa.atom, wa.bytes);
            }
        }
    }
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
        let overlap = overlap_at(ctx, pair[0]); // left endpoint
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
    let mut cumulative: BTreeMap<&Atom, usize> = BTreeMap::new();
    for wr in &ctx.walker_rows {
        if wr.seen_t > t {
            break;
        }
        for wa in &wr.atoms {
            let prev = cumulative.get(&wa.atom).copied().unwrap_or(0);
            if wa.bytes > prev {
                cumulative.insert(&wa.atom, wa.bytes);
            }
        }
    }
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

fn format_report(scores: &Scores, ctx: &BuildCtx) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "scores: Sim={:.3} Reached={}/{} Early={} Late={} Partial={} Missing={} Used={}/{}\n",
        scores.sim,
        scores.reached,
        scores.total_ns,
        scores.early,
        scores.late,
        scores.partial,
        scores.missing,
        ctx.schedule.cumulative_tokens,
        ctx.schedule.budget,
    ));

    let arrivals = arrival_infos(ctx);

    format_tier_rollup(&mut out, ctx, &arrivals);
    format_arrival_ledger(&mut out, ctx, &arrivals);
    format_walker_waste(&mut out, ctx);

    out
}

fn format_tier_rollup(out: &mut String, ctx: &BuildCtx, arrivals: &[Arrival]) {
    // Aggregate per tier. Rows in NS order; tiers sorted numerically.
    struct TierAgg {
        batches: usize,
        reached: usize,
        partial: usize,
        missing: usize,
        credit_sum: f64,
    }
    let mut by_tier: BTreeMap<usize, TierAgg> = BTreeMap::new();
    for (row, arrival) in ctx.ns_rows.iter().zip(arrivals) {
        let agg = by_tier.entry(row.tier).or_insert(TierAgg {
            batches: 0,
            reached: 0,
            partial: 0,
            missing: 0,
            credit_sum: 0.0,
        });
        agg.batches += 1;
        if arrival.credit < MISSING_FLOOR {
            agg.missing += 1;
        } else if arrival.credit < REACH_THRESHOLD {
            agg.partial += 1;
        } else {
            agg.reached += 1;
        }
        agg.credit_sum += arrival.credit;
    }
    if by_tier.is_empty() {
        return;
    }
    out.push_str("\n## Tier rollup\n\n");
    out.push_str("| tier | batches | reached | partial | missing | avg_credit |\n");
    out.push_str("|-----:|--------:|--------:|--------:|--------:|-----------:|\n");
    for (tier, agg) in &by_tier {
        let avg = agg.credit_sum / agg.batches as f64;
        out.push_str(&format!(
            "| {tier} | {} | {} | {} | {} | {avg:.2} |\n",
            agg.batches, agg.reached, agg.partial, agg.missing,
        ));
    }
}

fn format_arrival_ledger(out: &mut String, ctx: &BuildCtx, arrivals: &[Arrival]) {
    struct Row<'a> {
        id: &'a str,
        exp_t: usize,
        descriptor: &'a str,
        arrival: &'a Arrival,
    }
    let mut rows: Vec<Row<'_>> = ctx
        .ns_rows
        .iter()
        .enumerate()
        .zip(arrivals)
        .map(|((i, ns_row), arrival)| Row {
            id: &ns_row.id,
            exp_t: ns_row.exp_t,
            descriptor: &ctx.ns.batches[i].descriptor,
            arrival,
        })
        .collect();
    rows.sort_by(|a, b| numeric_id_cmp(a.id, b.id));

    // Show rows that aren't perfectly aligned: anything that's not an
    // `Aligned` batch with full credit, OR an aligned-with-full-credit
    // batch where walker over-rendered (the `+over` annotation is its
    // only surface — the headline counter was dropped).
    let interesting: Vec<&Row> = rows
        .iter()
        .filter(|r| {
            r.arrival.status != ArrivalStatus::Aligned || r.arrival.credit < 1.0 || r.arrival.over
        })
        .collect();

    if interesting.is_empty() {
        return;
    }
    out.push_str("\n## Arrival ledger (non-aligned or partial-credit NS batches)\n\n");
    out.push_str("| id | exp_t | reached_t | delta_t | credit | status | descriptor |\n");
    out.push_str("|----|------:|----------:|--------:|-------:|:-------|:-----------|\n");
    for r in &interesting {
        let (reached_cell, delta_cell) = match r.arrival.reached_t {
            Some(t) => {
                let delta = t as isize - r.exp_t as isize;
                let sign = if delta > 0 { "+" } else { "" };
                (format!("{t}"), format!("{sign}{delta}"))
            }
            None => ("—".to_string(), "—".to_string()),
        };
        let status_cell = if r.arrival.over && r.arrival.status == ArrivalStatus::Aligned {
            format!("{}+over", r.arrival.status.label())
        } else {
            r.arrival.status.label().to_string()
        };
        out.push_str(&format!(
            "| {} | {} | {reached_cell} | {delta_cell} | {:.2} | {status_cell} | {} |\n",
            r.id, r.exp_t, r.arrival.credit, r.descriptor,
        ));
    }
}

fn format_walker_waste(out: &mut String, ctx: &BuildCtx) {
    let ns_atom_set: BTreeSet<Atom> = ctx
        .ns_rows
        .iter()
        .flat_map(|r| r.atoms.iter().map(|a| a.atom.clone()))
        .collect();

    // For each walker batch: how much of its spend landed on atoms
    // NS didn't ask for. Pure-waste batches (no NS intersection)
    // contribute their full cost; mixed batches contribute in
    // proportion to their off-NS atom share. Sort by off_tokens
    // descending so the biggest calibration targets are at row 1.
    struct Row<'a> {
        wr: &'a WalkerRow<'a>,
        off_ratio: f64,
        off_tokens: usize,
    }
    let mut rows: Vec<Row<'_>> = ctx
        .walker_rows
        .iter()
        .filter(|wr| wr.batch.cost_tokens >= UNMAPPED_COST_THRESHOLD)
        .filter_map(|wr| {
            if wr.atoms.is_empty() {
                return None;
            }
            let total = wr.atoms.len() as f64;
            let off = wr
                .atoms
                .iter()
                .filter(|a| !ns_atom_set.contains(&a.atom))
                .count() as f64;
            let off_ratio = off / total;
            let off_tokens = (off_ratio * wr.batch.cost_tokens as f64) as usize;
            if off_tokens < UNMAPPED_COST_THRESHOLD {
                return None;
            }
            Some(Row {
                wr,
                off_ratio,
                off_tokens,
            })
        })
        .collect();
    if rows.is_empty() {
        return;
    }
    rows.sort_by(|a, b| {
        b.off_tokens
            .cmp(&a.off_tokens)
            .then_with(|| a.wr.batch.descriptor.cmp(&b.wr.batch.descriptor))
    });

    out.push_str(&format!(
        "\n## Walker waste (off-NS token spend ≥ {UNMAPPED_COST_THRESHOLD})\n\n"
    ));
    out.push_str("| off_tokens | off_ratio | cost | first_t | batch |\n");
    out.push_str("|-----------:|----------:|-----:|--------:|:------|\n");
    for r in &rows {
        let rel = strip_fixture_root(&r.wr.batch.descriptor, &ctx.fixture_root);
        out.push_str(&format!(
            "| {} | {:.2} | {} | {} | {rel} |\n",
            r.off_tokens, r.off_ratio, r.wr.batch.cost_tokens, r.wr.seen_t,
        ));
    }
}

fn strip_fixture_root(descriptor: &str, fixture_root: &Path) -> String {
    // Walker descriptors embed absolute paths (e.g. "crate-doc lede in
    // /Users/.../tests/fixtures/log/src/lib.rs"). Stripping the fixture-
    // root prefix makes reports diff-stable across checkouts.
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

/// Sort NS ids numerically: `2.5 < 2.8 < 3.1 < 10.2`. String-sort would
/// break with `2.10 < 2.2`.
fn numeric_id_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let pa: Vec<usize> = a.split('.').filter_map(|s| s.parse().ok()).collect();
    let pb: Vec<usize> = b.split('.').filter_map(|s| s.parse().ok()).collect();
    pa.cmp(&pb)
}
