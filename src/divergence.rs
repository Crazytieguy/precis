//! Divergence metric + report generator. Compares a walker `Schedule` run
//! at `T_max` to a frozen `NorthStar`, producing:
//!
//! - `Scores`: a small tuple (`sim`, counts) with the integral similarity
//!   scalar plus Reached/Early/Late/Missing/Over counts. Goodhart
//!   resistance comes from all three moving in the same direction on a
//!   real improvement.
//! - Markdown `Report` (`tests/divergence/<fixture>.md`): one per fixture,
//!   holistic across budgets. Line 1 is the grep-able score line; body
//!   is an arrival ledger (one row per NS batch with expected vs
//!   observed cum_tokens, credit, status) plus a walker-waste section
//!   for off-NS content. Stable ordering; perfect alignment produces a
//!   short file.
//!
//! Under scheduler prefix-monotonicity (see `docs/design-notes.md`), the
//! walker schedule at any `t ≤ T_max` is exactly the prefix of the
//! `T_max` schedule with `cum_tokens ≤ t`. So we run the walker once
//! and read sub-budget behavior from the trajectory. The NS ordering is
//! authored cumulatively (each batch declares its predecessor / rank);
//! its `exp_t` per batch is its declared cumulative-tokens.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::batch::{BatchContent, Render, Span};
use crate::render::SourceCache;
use crate::schedule_types::{Atom, Schedule, ScheduledBatch};
use crate::schema::{NorthStar, resolve_content};

/// Time-weight half-life (tokens). `w(t) = exp(−t / τ)`. First 2000
/// tokens carry ~63% of the mass; first 6000 ~95%.
const TAU: f64 = 2000.0;

/// Credit threshold for "walker reached this NS batch". NS batch is
/// considered delivered at the first walker step where its average
/// per-atom credit reaches this level.
const REACH_THRESHOLD: f64 = 0.8;

/// Below this credit, an NS batch is counted as `missing` in the
/// headline — walker rendered essentially nothing of what it wanted.
/// Batches in `[MISSING_HEADLINE_FLOOR, REACH_THRESHOLD)` count as
/// reached-partial (neither `missing` nor `reached`) in the headline,
/// and label as `partial` in the arrival ledger.
const MISSING_HEADLINE_FLOOR: f64 = 0.5;
/// Label-only: ledger rows with credit below this and no `seen_t`
/// surface as `missing` rather than `partial`.
const MISSING_LABEL_FLOOR: f64 = 0.01;

/// Classification thresholds (tokens). Used to tag arrival-ledger rows.
const EARLY_FACTOR: f64 = 0.7;
const LATE_FACTOR: f64 = 1.3;

/// Report threshold: walker batches with cost below this are elided from
/// the "walker waste" section.
const UNMAPPED_COST_THRESHOLD: usize = 50;

/// Headline scores.
#[derive(Debug, Clone, Copy)]
pub struct Scores {
    pub sim: f64,
    /// Number of NS batches whose credit in the final walker state
    /// reaches [`REACH_THRESHOLD`].
    pub reached: usize,
    pub total_ns: usize,
    /// NS batches rendered meaningfully earlier than expected.
    pub early: usize,
    /// NS batches rendered meaningfully later than expected.
    pub late: usize,
    /// NS batches not reached by the walker at the cap.
    pub missing: usize,
    /// NS batches where walker's rendered bytes exceed what NS asked for
    /// on at least one line (still fully credited).
    pub over: usize,
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
/// `bytes` semantics:
/// - `Line`: exclusive byte offset within the source line this render
///   makes visible. `Full` = full line length; `Truncated{pattern}` =
///   regex-match byte-end; `Ellipsis` = 0.
/// - `Fs`: always `1`. Fs atoms are boolean.
///
/// Credit between two graded atoms sharing identity (same `Atom`) is
/// `min(walker.bytes, ns.bytes) / max(ns.bytes, 1)`, capped at 1.0.
/// Walker rendering strictly more bytes than NS asks for is fully
/// credited but flagged separately in the report.
#[derive(Debug, Clone)]
struct GradedAtom {
    atom: Atom,
    bytes: usize,
}

/// Derive graded atoms for one batch. Needs the source cache to compute
/// per-line byte_end values — full-line length and regex-match offsets.
fn atoms_from_content(
    content: &BatchContent,
    source_cache: &SourceCache,
    fixture_root: &Path,
) -> Vec<GradedAtom> {
    match content {
        BatchContent::Fs { groups } => groups
            .iter()
            .flat_map(|g| {
                // `g.parent` arrives absolute from the schema/walker. We
                // store the absolute parent for compatibility with walker-
                // emitted schedules.
                let parent = g.parent.clone();
                g.children.keys().map(move |name| GradedAtom {
                    atom: Atom::Fs {
                        parent: parent.clone(),
                        entry: name.clone(),
                    },
                    bytes: 1,
                })
            })
            .collect(),
        BatchContent::Lines { spans } => {
            let by_line = resolve_span_renders(spans);
            let mut out = Vec::with_capacity(by_line.len());
            for ((path, line), render) in by_line {
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
fn byte_end_for(render: &Render, source_line: &str) -> usize {
    match render {
        Render::Full => source_line.len(),
        Render::Ellipsis => 0,
        Render::Truncated { pattern } => regex::Regex::new(pattern)
            .ok()
            .and_then(|re| re.find(source_line).map(|m| m.end()))
            .unwrap_or(0),
    }
}

/// Resolve overlapping spans within one batch to the strongest render per
/// (path, line), matching `RenderedTree::apply_spans`.
fn resolve_span_renders(spans: &[Span]) -> Vec<((PathBuf, usize), Render)> {
    let mut by_key: BTreeMap<(PathBuf, usize), Render> = BTreeMap::new();
    for span in spans {
        for line in span.start..=span.end {
            by_key
                .entry((span.path.clone(), line))
                .and_modify(|r| {
                    if span.render.priority() > r.priority() {
                        *r = span.render.clone();
                    }
                })
                .or_insert_with(|| span.render.clone());
        }
    }
    by_key.into_iter().collect()
}

// ---- context -----------------------------------------------------------

struct BuildCtx<'a> {
    /// NS batches, in declared order. Each entry carries its atoms (with
    /// bytes) and its cumulative cost at that position (`exp_t`).
    ns_rows: Vec<NsRow>,
    /// Walker batches, in scheduled order. Each entry carries its atoms
    /// (with bytes) and its `cum_tokens` at schedule time.
    walker_rows: Vec<WalkerRow<'a>>,
    ns: &'a NorthStar,
    schedule: &'a Schedule,
}

struct NsRow {
    id: String,
    atoms: Vec<GradedAtom>,
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

        let mut ns_rows = Vec::with_capacity(ns.batches.len());
        let mut cum = 0usize;
        for b in &ns.batches {
            let content = resolve_content(&b.content, fixture_root)?;
            let atoms = atoms_from_content(&content, &source_cache, fixture_root);
            let cost = batch_token_cost(&content, fixture_root);
            cum += cost;
            ns_rows.push(NsRow {
                id: b.id.clone(),
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
        })
    }
}

/// Deterministic token-cost estimate for an NS batch. Delegates to
/// `RenderedTree::marginal_cost` so cost accounting matches the scheduler.
fn batch_token_cost(content: &BatchContent, fixture_root: &Path) -> usize {
    let source_cache = crate::render::SourceCache::new();
    let tree = crate::render::RenderedTree::new(fixture_root.to_path_buf(), source_cache);
    let batch = crate::batch::Batch {
        content: content.clone(),
        signals: crate::batch::ValueSignals::default(),
    };
    tree.marginal_cost(&batch).tokens
}

// ---- scoring -----------------------------------------------------------

fn compute_scores(ctx: &BuildCtx) -> Scores {
    let t_max = ctx.schedule.budget.max(ctx.schedule.cumulative_tokens);
    let sim = compute_sim(ctx, t_max);

    let mut reached = 0;
    let mut early = 0;
    let mut late = 0;
    let mut missing = 0;
    let mut over = 0;

    for arrival in arrival_infos(ctx) {
        if arrival.over {
            over += 1;
        }
        if arrival.credit < MISSING_HEADLINE_FLOOR {
            missing += 1;
            continue;
        }
        match arrival.status {
            ArrivalStatus::Aligned | ArrivalStatus::Early | ArrivalStatus::Late => {
                reached += 1;
                match arrival.status {
                    ArrivalStatus::Early => early += 1,
                    ArrivalStatus::Late => late += 1,
                    _ => {}
                }
            }
            // Missing-label rows with credit ≥ MISSING_HEADLINE_FLOOR
            // can't occur (floor > label floor) — defensive.
            ArrivalStatus::Missing | ArrivalStatus::Partial => {}
        }
    }

    Scores {
        sim,
        reached,
        total_ns: ctx.ns_rows.len(),
        early,
        late,
        missing,
        over,
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

/// Per-NS-batch walker observation. Computed once per run and reused by
/// both the headline counters and the arrival-ledger formatter.
struct Arrival {
    seen_t: Option<usize>,
    credit: f64,
    over: bool,
    status: ArrivalStatus,
}

fn arrival_infos(ctx: &BuildCtx) -> Vec<Arrival> {
    ctx.ns_rows
        .iter()
        .map(|row| {
            let seen_t = first_reach_t(ctx, &row.atoms);
            let (credit, over) = credit_for_ns_atoms_at(ctx, &row.atoms, usize::MAX);
            let status = classify(seen_t, row.exp_t, credit);
            Arrival {
                seen_t,
                credit,
                over,
                status,
            }
        })
        .collect()
}

/// `first_reach_t`: cumulative walker tokens at the first walker batch
/// after which this NS batch's credit ≥ `REACH_THRESHOLD`.
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
        // Re-score against NS atoms using the materialized map.
        let (credit, _over) = credit_against_cum_map(ns_atoms, &cumulative);
        if credit >= REACH_THRESHOLD {
            return Some(wr.seen_t);
        }
    }
    None
}

/// Final-state credit (averaged per NS atom) of this NS batch against the
/// walker's entire output. Also returns whether walker over-rendered
/// (bytes > ns bytes) on any line of this batch.
fn credit_for_ns_atoms_at(ctx: &BuildCtx, ns_atoms: &[GradedAtom], budget: usize) -> (f64, bool) {
    let mut cumulative: BTreeMap<&Atom, usize> = BTreeMap::new();
    for wr in &ctx.walker_rows {
        if wr.seen_t > budget {
            break;
        }
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

/// Sim = ∫ w(t)·overlap(t) dt / ∫ w(t) dt over t ∈ [0, T_max], with w(t)
/// = exp(−t/τ). overlap(t) averages credit over NS atoms *reachable at
/// t* (ns batch cum ≤ t) — so low-t reports no longer include
/// unreachable-by-construction NS atoms in the denominator.
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
        let t_mid = pair[1]; // right endpoint — piecewise-constant from the right
        let overlap = overlap_at(ctx, t_mid);
        let w = weighted_segment(a, b);
        num += w * overlap;
        den += w;
    }
    if den > 0.0 { num / den } else { 0.0 }
}

fn overlap_at(ctx: &BuildCtx, t: usize) -> f64 {
    // Walker cumulative byte-max at t.
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
    // NS atoms reachable at t (ns batch's exp_t ≤ t). Average credit.
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
        "scores: Sim={:.3} Reached={}/{} Early={} Late={} Missing={} Over={} Cap={}\n",
        scores.sim,
        scores.reached,
        scores.total_ns,
        scores.early,
        scores.late,
        scores.missing,
        scores.over,
        ctx.schedule.budget,
    ));

    // Arrival ledger — one row per NS batch, sorted numerically by id.
    struct Row<'a> {
        id: &'a str,
        exp_t: usize,
        descriptor: &'a str,
        arrival: Arrival,
    }
    let arrivals = arrival_infos(ctx);
    let mut rows: Vec<Row<'_>> = ctx
        .ns_rows
        .iter()
        .zip(arrivals)
        .enumerate()
        .map(|(i, (ns_row, arrival))| Row {
            id: &ns_row.id,
            exp_t: ns_row.exp_t,
            descriptor: &ctx.ns.batches[i].descriptor,
            arrival,
        })
        .collect();
    rows.sort_by(|a, b| numeric_id_cmp(a.id, b.id));

    // Filter: only show rows that aren't perfectly-aligned (save readers
    // from scrolling through 40 `aligned` rows in a clean fixture).
    let interesting: Vec<&Row> = rows
        .iter()
        .filter(|r| {
            r.arrival.status != ArrivalStatus::Aligned || r.arrival.credit < 1.0 || r.arrival.over
        })
        .collect();

    if !interesting.is_empty() {
        out.push_str("\n## Arrival ledger (non-aligned NS batches)\n\n");
        out.push_str("| id | exp_t | seen_t | credit | status | descriptor |\n");
        out.push_str("|----|------:|-------:|-------:|:-------|:-----------|\n");
        for r in &interesting {
            let seen_cell = match r.arrival.seen_t {
                Some(t) => format!("{t}"),
                None => "—".to_string(),
            };
            let status_cell = if r.arrival.over {
                format!("{}+over", r.arrival.status.label())
            } else {
                r.arrival.status.label().to_string()
            };
            out.push_str(&format!(
                "| {} | {} | {seen_cell} | {:.2} | {status_cell} | {} |\n",
                r.id, r.exp_t, r.arrival.credit, r.descriptor,
            ));
        }
    }

    // Walker waste: walker batches whose content has no NS intersection
    // and whose cost is significant. Sorted by first appearance.
    let ns_atom_set: BTreeSet<Atom> = ctx
        .ns_rows
        .iter()
        .flat_map(|r| r.atoms.iter().map(|a| a.atom.clone()))
        .collect();
    let mut waste: Vec<(usize, &ScheduledBatch)> = Vec::new();
    for wr in &ctx.walker_rows {
        if wr.batch.cost_tokens < UNMAPPED_COST_THRESHOLD {
            continue;
        }
        let any_on = wr.atoms.iter().any(|a| ns_atom_set.contains(&a.atom));
        if !any_on {
            waste.push((wr.seen_t, wr.batch));
        }
    }
    if !waste.is_empty() {
        out.push_str(&format!(
            "\n## Walker waste (cost ≥ {UNMAPPED_COST_THRESHOLD}, no NS intersection)\n\n"
        ));
        out.push_str("| first_t | cost | key |\n");
        out.push_str("|--------:|-----:|:----|\n");
        for (t, b) in &waste {
            out.push_str(&format!("| {t} | {} | {} |\n", b.cost_tokens, b.key));
        }
    }

    out
}

fn classify(seen_t: Option<usize>, exp_t: usize, credit: f64) -> ArrivalStatus {
    let Some(seen) = seen_t else {
        if credit < MISSING_LABEL_FLOOR {
            return ArrivalStatus::Missing;
        }
        return ArrivalStatus::Partial;
    };
    if credit < REACH_THRESHOLD {
        return ArrivalStatus::Partial;
    }
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
