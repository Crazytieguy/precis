//! Divergence metric + report generator. Compares a walker `Schedule` to
//! a frozen `NorthStar`, producing:
//!
//! - `Scores`: headline scalars (`Sim`, `OffScriptFrac`, `PredViolations`,
//!   `Coverage`) derived from graded-atom intersection over token-indexed
//!   coverage curves.
//! - Markdown `Report`: committed artifact with the score line + missed-NS-
//!   batch and unmapped-walker-batch sections. Designed to diff cleanly
//!   across runs — stable identifiers, NS-id-sorted, empty sections elided.
//!
//! See the plan (`great-yeah-actually-k-to-k-keen-kahn.md`) for the metric
//! definition. Single-file module — keeping atoms / weighting / grading /
//! formatting together makes constants easy to find and adjust.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use anyhow::Result;

use crate::batch::{BatchContent, Render, Span};
use crate::schedule_types::{Atom, Schedule, ScheduledBatch};
use crate::schema::{NorthStar, NsBatch, resolve_content};

/// Time-weight half-life in tokens. `w(t) = exp(−t / τ)`. First 2000 tokens
/// carry ~63% of the mass, first 6000 ~95%.
const TAU: f64 = 2000.0;

/// Per-render content grade. Full > Truncated > Ellipsis > Absent.
/// Filesystem atoms are boolean (grade 1.0 when present).
const GRADE_FULL: f64 = 1.0;
const GRADE_TRUNCATED: f64 = 0.6;
const GRADE_ELLIPSIS: f64 = 0.3;

/// Report threshold: NS batches with credit below this show up in the
/// "missed" section. Tunable; current value elides near-fully-covered
/// batches to keep reports short under small walker perturbations.
const MISSED_THRESHOLD: f64 = 0.8;

/// Report threshold: walker batches with cost below this are elided from
/// the "unmapped" section (prevents every trivial off-script batch from
/// cluttering the diff). Tunable.
const UNMAPPED_COST_THRESHOLD: usize = 50;

/// Headline scalars.
#[derive(Debug, Clone, Copy)]
pub struct Scores {
    pub sim: f64,
    pub off_script_frac: f64,
    pub pred_violations: usize,
    pub coverage: f64,
}

/// Compute scores for an NS against a walker schedule. Requires a fixture
/// root because NS spans are authored relative to it.
pub fn score(
    ns: &NorthStar,
    schedule: &Schedule,
    fixture_root: &std::path::Path,
) -> Result<Scores> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    Ok(compute_scores(&ctx))
}

/// Generate the markdown divergence report. Line 1 is the grep-able score
/// line; subsequent sections list missed NS batches + unmapped walker
/// batches + predecessor violations, each elided when empty.
pub fn generate_divergence_report(
    ns: &NorthStar,
    schedule: &Schedule,
    fixture_root: &std::path::Path,
) -> Result<String> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let scores = compute_scores(&ctx);
    Ok(format_report(&scores, &ctx))
}

// ---- internals ----

/// Grade of a rendered atom.
fn grade(render: &Render) -> f64 {
    match render {
        Render::Full => GRADE_FULL,
        Render::Truncated { .. } => GRADE_TRUNCATED,
        Render::Ellipsis => GRADE_ELLIPSIS,
    }
}

/// An atom with its grade. Fs atoms are always `1.0`; lines vary with
/// the render spec at that (path, line).
#[derive(Debug, Clone)]
struct GradedAtom {
    atom: Atom,
    grade: f64,
}

/// Build per-batch atom sets + cumulative-cost indexing for both sides.
/// NS batches are indexed by declared id; walker batches by `position`.
struct BuildCtx<'a> {
    ns_atoms: Vec<(String, Vec<GradedAtom>, usize)>,
    walker_atoms: Vec<(usize, Vec<GradedAtom>, usize, &'a ScheduledBatch)>,
    ns: &'a NorthStar,
    schedule: &'a Schedule,
}

impl<'a> BuildCtx<'a> {
    fn new(
        ns: &'a NorthStar,
        schedule: &'a Schedule,
        fixture_root: &std::path::Path,
    ) -> Result<Self> {
        let mut ns_atoms = Vec::with_capacity(ns.batches.len());
        let mut cum = 0usize;
        for b in &ns.batches {
            let content = resolve_content(&b.content, fixture_root)?;
            let atoms = atoms_from_content(&content);
            let cost = batch_token_cost(&content, fixture_root);
            cum += cost;
            ns_atoms.push((b.id.clone(), atoms, cum));
        }

        let walker_atoms = schedule
            .batches
            .iter()
            .map(|b| {
                let atoms = atoms_from_content(&b.content);
                (b.position, atoms, b.cum_tokens, b)
            })
            .collect();

        Ok(Self {
            ns_atoms,
            walker_atoms,
            ns,
            schedule,
        })
    }
}

fn atoms_from_content(content: &BatchContent) -> Vec<GradedAtom> {
    match content {
        BatchContent::Fs { groups } => groups
            .iter()
            .flat_map(|g| {
                g.children.keys().map(|name| GradedAtom {
                    atom: Atom::Fs {
                        parent: g.parent.clone(),
                        entry: name.clone(),
                    },
                    grade: GRADE_FULL,
                })
            })
            .collect(),
        BatchContent::Lines { spans } => {
            let resolved = resolve_span_grades(spans);
            resolved
                .into_iter()
                .map(|(path, line, r)| GradedAtom {
                    atom: Atom::Line { path, line },
                    grade: grade(&r),
                })
                .collect()
        }
    }
}

/// Same priority-based span resolution `RenderedTree::apply_spans` uses:
/// overlapping spans within a single batch resolve to the stronger render.
fn resolve_span_grades(spans: &[Span]) -> Vec<(PathBuf, usize, Render)> {
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
    by_key.into_iter().map(|((p, l), r)| (p, l, r)).collect()
}

/// Deterministic token-cost estimate for an NS batch at its declared
/// position. Mirrors `RenderedTree::cost_spans` / `cost_fs_groups` logic
/// but doesn't require threading a `RenderedTree` through — the metric
/// only needs cumulative-token ordering, not exact override-accounting.
fn batch_token_cost(content: &BatchContent, fixture_root: &std::path::Path) -> usize {
    // Delegate to a fresh RenderedTree so cost accounting exactly matches
    // the scheduler's. Slight overhead per NS batch but avoids duplicating
    // the logic.
    let source_cache = crate::render::SourceCache::new();
    let tree = crate::render::RenderedTree::new(fixture_root.to_path_buf(), source_cache);
    let batch = crate::batch::Batch {
        content: content.clone(),
        signals: crate::batch::ValueSignals::default(),
    };
    tree.marginal_cost(&batch).tokens
}

/// Compute `Sim` = ∫ w(t) overlap(t) dt / ∫ w(t) dt over t ∈ [0, budget].
/// `overlap(t)` is graded-atom credit — for each NS atom reachable at t,
/// sum `min(walker_grade, ns_grade) / ns_grade` over all NS atoms / |A(t)|.
/// Piecewise-constant integrand; finite sum over segment boundaries.
fn compute_scores(ctx: &BuildCtx) -> Scores {
    let budget_end = ctx.schedule.budget.max(ctx.schedule.cumulative_tokens);

    // Build walker's per-atom grade map at final state (t = end). Later
    // we'll subset this by cum_tokens to compute B(t).
    let walker_final: BTreeMap<Atom, f64> = {
        let mut map: BTreeMap<Atom, f64> = BTreeMap::new();
        for (_, atoms, _, _) in &ctx.walker_atoms {
            for a in atoms {
                map.entry(a.atom.clone())
                    .and_modify(|g| {
                        if a.grade > *g {
                            *g = a.grade;
                        }
                    })
                    .or_insert(a.grade);
            }
        }
        map
    };

    // Integrate over budget-t. Piecewise boundaries are the cum_tokens of
    // each NS batch and each walker batch — between boundaries, overlap(t)
    // is constant.
    let mut boundaries: Vec<usize> = Vec::new();
    boundaries.push(0);
    for (_, _, cum) in &ctx.ns_atoms {
        boundaries.push(*cum);
    }
    for (_, _, cum, _) in &ctx.walker_atoms {
        boundaries.push(*cum);
    }
    boundaries.push(budget_end);
    boundaries.sort();
    boundaries.dedup();

    let mut num = 0.0;
    let mut den = 0.0;
    let mut last_t = 0.0;
    for &t in boundaries.iter().skip(1) {
        let t_f = t as f64;
        let overlap = overlap_at(ctx, t);
        let w = weighted_segment(last_t, t_f);
        num += w * overlap;
        den += w;
        last_t = t_f;
    }

    let sim = if den > 0.0 { num / den } else { 0.0 };
    let off_script_frac = off_script_fraction(ctx);
    let pred_violations = count_pred_violations(ctx, &walker_final);
    let coverage = coverage_at_full(ctx, &walker_final);

    Scores {
        sim,
        off_script_frac,
        pred_violations,
        coverage,
    }
}

/// `overlap(t)`: for NS batches whose cumulative-declared-cost ≤ t, sum
/// graded credit of each NS atom against walker's atom set at t. Walker's
/// atom set at t = union of walker batches with cum_tokens ≤ t.
fn overlap_at(ctx: &BuildCtx, t: usize) -> f64 {
    let walker_at_t: BTreeMap<Atom, f64> = walker_atoms_at(ctx, t);
    let mut total_credit = 0.0;
    let mut total_atoms = 0.0;
    for (_, atoms, cum) in &ctx.ns_atoms {
        if *cum > t {
            break;
        }
        for a in atoms {
            total_atoms += 1.0;
            let g_walker = walker_at_t.get(&a.atom).copied().unwrap_or(0.0);
            let credit = (g_walker.min(a.grade) / a.grade).min(1.0);
            total_credit += credit;
        }
    }
    if total_atoms == 0.0 {
        0.0
    } else {
        total_credit / total_atoms
    }
}

fn walker_atoms_at(ctx: &BuildCtx, t: usize) -> BTreeMap<Atom, f64> {
    let mut map: BTreeMap<Atom, f64> = BTreeMap::new();
    for (_, atoms, cum, _) in &ctx.walker_atoms {
        if *cum > t {
            break;
        }
        for a in atoms {
            map.entry(a.atom.clone())
                .and_modify(|g| {
                    if a.grade > *g {
                        *g = a.grade;
                    }
                })
                .or_insert(a.grade);
        }
    }
    map
}

/// ∫_{a}^{b} exp(−t/τ) dt = τ · (exp(−a/τ) − exp(−b/τ))
fn weighted_segment(a: f64, b: f64) -> f64 {
    TAU * ((-a / TAU).exp() - (-b / TAU).exp())
}

fn off_script_fraction(ctx: &BuildCtx) -> f64 {
    let ns_atoms: BTreeSet<Atom> = ctx
        .ns_atoms
        .iter()
        .flat_map(|(_, atoms, _)| atoms.iter().map(|a| a.atom.clone()))
        .collect();
    let mut total = 0.0;
    let mut off = 0.0;
    for (_, atoms, _, _) in &ctx.walker_atoms {
        for a in atoms {
            total += 1.0;
            if !ns_atoms.contains(&a.atom) {
                off += 1.0;
            }
        }
    }
    if total > 0.0 { off / total } else { 0.0 }
}

fn count_pred_violations(ctx: &BuildCtx, walker_final: &BTreeMap<Atom, f64>) -> usize {
    // For each NS batch with a declared predecessor, check that the NS
    // batch itself is rendered (≥50% credit) AND its predecessor is
    // rendered (≥50%). If rendered but predecessor isn't, that's a
    // violation.
    let mut violations = 0;
    let ns_by_id: BTreeMap<&str, &NsBatch> =
        ctx.ns.batches.iter().map(|b| (b.id.as_str(), b)).collect();
    let ns_atoms_by_id: BTreeMap<&str, &Vec<GradedAtom>> = ctx
        .ns_atoms
        .iter()
        .map(|(id, atoms, _)| (id.as_str(), atoms))
        .collect();

    for b in &ctx.ns.batches {
        let Some(pred_id) = b.predecessor.as_deref() else {
            continue;
        };
        let Some(self_atoms) = ns_atoms_by_id.get(b.id.as_str()) else {
            continue;
        };
        let self_coverage = batch_credit(self_atoms, walker_final);
        if self_coverage < 0.5 {
            continue; // NS batch not meaningfully rendered; no pred check
        }
        let Some(pred_atoms) = ns_atoms_by_id.get(pred_id) else {
            continue;
        };
        let pred_coverage = batch_credit(pred_atoms, walker_final);
        if pred_coverage < 0.5 {
            violations += 1;
        }
        // reference to keep ns_by_id from being unused
        let _ = ns_by_id.get(b.id.as_str());
    }
    violations
}

fn batch_credit(atoms: &[GradedAtom], walker_final: &BTreeMap<Atom, f64>) -> f64 {
    if atoms.is_empty() {
        return 0.0;
    }
    let mut total = 0.0;
    for a in atoms {
        let g_walker = walker_final.get(&a.atom).copied().unwrap_or(0.0);
        total += (g_walker.min(a.grade) / a.grade).min(1.0);
    }
    total / atoms.len() as f64
}

fn coverage_at_full(ctx: &BuildCtx, walker_final: &BTreeMap<Atom, f64>) -> f64 {
    let mut total = 0.0;
    let mut count = 0.0;
    for (_, atoms, _) in &ctx.ns_atoms {
        for a in atoms {
            count += 1.0;
            let g = walker_final.get(&a.atom).copied().unwrap_or(0.0);
            total += (g.min(a.grade) / a.grade).min(1.0);
        }
    }
    if count > 0.0 { total / count } else { 0.0 }
}

// ---- report formatting ----

fn format_report(scores: &Scores, ctx: &BuildCtx) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "scores: Sim={:.3} OffScript={:.2} Pred={} Coverage={:.2}\n",
        scores.sim, scores.off_script_frac, scores.pred_violations, scores.coverage
    ));

    let walker_final = walker_atoms_at(ctx, usize::MAX);
    let ns_atoms_by_id: BTreeMap<&str, &Vec<GradedAtom>> = ctx
        .ns_atoms
        .iter()
        .map(|(id, atoms, _)| (id.as_str(), atoms))
        .collect();
    let ns_all_atoms: BTreeSet<Atom> = ctx
        .ns_atoms
        .iter()
        .flat_map(|(_, atoms, _)| atoms.iter().map(|a| a.atom.clone()))
        .collect();

    // Missed NS batches — sorted by numeric-aware id.
    let mut missed: Vec<(String, f64, &NsBatch)> = Vec::new();
    for b in &ctx.ns.batches {
        let Some(atoms) = ns_atoms_by_id.get(b.id.as_str()) else {
            continue;
        };
        let cov = batch_credit(atoms, &walker_final);
        if cov < MISSED_THRESHOLD {
            missed.push((b.id.clone(), cov, b));
        }
    }
    missed.sort_by(|a, b| numeric_id_cmp(&a.0, &b.0));
    if !missed.is_empty() {
        out.push_str(&format!(
            "\n## Missed NS batches (credit < {MISSED_THRESHOLD})\n\n"
        ));
        for (id, credit, b) in &missed {
            out.push_str(&format!(
                "- [{id} {descriptor}] credit={credit:.2}\n",
                descriptor = b.descriptor
            ));
        }
    }

    // Unmapped walker batches — sorted lexicographically by key.
    let mut unmapped: Vec<&ScheduledBatch> = Vec::new();
    for b in &ctx.schedule.batches {
        if b.cost_tokens < UNMAPPED_COST_THRESHOLD {
            continue;
        }
        let atoms = atoms_from_content(&b.content);
        let on_script = atoms.iter().any(|a| ns_all_atoms.contains(&a.atom));
        if !on_script {
            unmapped.push(b);
        }
    }
    unmapped.sort_by(|a, b| a.key.cmp(&b.key));
    if !unmapped.is_empty() {
        out.push_str(&format!(
            "\n## Unmapped walker batches (cost ≥ {UNMAPPED_COST_THRESHOLD})\n\n"
        ));
        for b in &unmapped {
            out.push_str(&format!("- {} cost={}\n", b.key, b.cost_tokens));
        }
    }

    if scores.pred_violations > 0 {
        out.push_str("\n## Predecessor violations\n\n");
        for b in &ctx.ns.batches {
            let Some(pred_id) = b.predecessor.as_deref() else {
                continue;
            };
            let Some(self_atoms) = ns_atoms_by_id.get(b.id.as_str()) else {
                continue;
            };
            let Some(pred_atoms) = ns_atoms_by_id.get(pred_id) else {
                continue;
            };
            let self_cov = batch_credit(self_atoms, &walker_final);
            let pred_cov = batch_credit(pred_atoms, &walker_final);
            if self_cov >= 0.5 && pred_cov < 0.5 {
                out.push_str(&format!("- {} (requires {pred_id}, not rendered)\n", b.id));
            }
        }
    }

    out
}

/// Sort NS ids numerically: `2.5 < 2.8 < 3.1 < 10.2`. String-sort would
/// break with `2.10 < 2.2`.
fn numeric_id_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let pa: Vec<usize> = a.split('.').filter_map(|s| s.parse().ok()).collect();
    let pb: Vec<usize> = b.split('.').filter_map(|s| s.parse().ok()).collect();
    pa.cmp(&pb)
}
