//! Divergence metric + report generator. Compares a walker `Schedule`
//! run at `T_max` to a frozen `NorthStar`. Two artifacts:
//!
//! - [`Scores`]: per-budget `Score(B)` vector across a 7-budget grid
//!   plus `A_3K`-gated bucket counts `reached + partial + missing ==
//!   ns_rows≤3K`. Primary objective is `Score(3000)` — the
//!   auto-injection budget every session hits.
//! - Per-fixture Markdown report (`tests/divergence/<fixture>.md`):
//!   one headline + one interleaved schedule table. NS rows and
//!   walker rows merged onto a shared `cum_tokens` axis, with
//!   `Score(B = cum)` computed at every row. Where Score climbs the
//!   schedule is contributing; where it stalls or falls, that row
//!   (or the absence around it) is the problem. Replaces an earlier
//!   derived-signals layer (Top opportunities, walker waste,
//!   diagnosis labels) — the per-row Score curve carries the same
//!   signal more faithfully. See `.claude/skills/iterate-divergence/`
//!   for how to read it.
//!
//! Cross-fixture survey is via shell, not a generated index file:
//! `head -1 tests/divergence/*.md` for the per-fixture score
//! headlines.
//!
//! ## Score formula
//!
//! `Score(B) = √(I(B) · C(B))`, where
//!
//! - `I(B)` — **Importance**, rank-weighted recall over A_B (atoms in
//!   NS batches with `exp_t ≤ B`): `min(1, Σ r(rank(a)) ·
//!   damped_credit(a) / Σ_{rank ≤ |A_B|} r(rank))`. Atoms past `A_B`
//!   contribute via `1/rank` but the denominator is fixed to A_B and
//!   the result caps at 1.
//! - `C(B)` — **Coverage**, rank-uniform recall over A_B:
//!   `Σ_{rank ≤ |A_B|} damped_credit(a) / |A_B|`.
//! - `r(rank) = 1 / rank` — per-atom Importance weight.
//! - `BUDGETS = [1000, 1442, 2080, 3000, 4327, 6240, 9000]` — geometric
//!   grid on `[1000, 9000]` with ratio ⁶√9 ≈ 1.442, symmetric around
//!   3000 on the log scale. `PRIMARY_BUDGET = 3000`.
//!
//! Atoms are `Line(path, line)` or `Fs(parent, entry)`, each with a
//! `bytes` footprint per render (Full = source-line length, Truncated
//! = regex match end, Ellipsis = 1, Fs = 1; floored at 1). Credit
//! between matched walker + NS atoms is `min(walker, ns) / max(ns, 1)`,
//! capped at 1. NS atoms carry a 1-indexed rank from a flat traversal
//! of NS batches in schedule order.
//!
//! `completion(B_i) = Σ_{a ∈ B_i} min(walker, ns) / Σ_{a ∈ B_i} ns` —
//! byte-weighted fraction of NS batch `B_i` the walker delivered.
//! Per-atom `damped_credit(a) = credit(a) · completion(B(a))` gates
//! each atom by the completeness of its enclosing NS batch ("finish
//! what you start").
//!
//! Under scheduler prefix-monotonicity, walker sub-budget behavior is
//! the prefix of the T_max schedule with `cum_tokens ≤ t`, so we run
//! the walker once and slice it for each budget in the grid.
//!
//! ## Headline + counts
//!
//! Each report's first line:
//! `Score(3000)=X.XXX I=X.XXX C=X.XXX ns_rows≤3K=N/T (reached=R
//! partial=P missing=M)`.
//!
//! `N` is NS rows (batches) with `exp_t ≤ 3000`, `T` is total NS rows.
//! `reached / partial / missing` are gated to those N rows and banded
//! on `credit × completion` at the primary budget: `< MISSING_FLOOR`
//! → missing, `< REACH_THRESHOLD` → partial, else reached.
//!
//! ## Schedule table
//!
//! `| source | ns_cum | walker_cum | marginal | descriptor | id |
//! predecessor | Score(B=cum) |`
//!
//! Rows are emitted via a two-pointer merge over NS batches (sorted
//! by `exp_t`) and walker batches (sorted by `cum_tokens`),
//! tie-breaking on equal cum by `source = walker first`. One row per
//! NS or walker batch; exactly `ns_rows + walker_rows` rows total.
//!
//! - `ns_cum` / `walker_cum` — only one is filled per row; the other
//!   is blank. `source` (`ns` or `walker`) disambiguates.
//! - `marginal` — `exp_t − prev_exp_t` for NS rows; `cost_tokens` for
//!   walker rows.
//! - `descriptor` — NS descriptor or walker descriptor (both
//!   root-relative — walker descriptors get the fixture-root strip
//!   inside [`crate::batch::WalkerKey::describe`]).
//! - `id` — `NsBatch.id` (e.g. `1.1`, `2.10`) for NS rows; blank for
//!   walker rows. Lets the reader grep
//!   `tests/north-stars/<fixture>.toml` for `id = "..."`.
//! - `predecessor` — `NsBatch.predecessor` field verbatim for NS rows;
//!   blank otherwise. Declares a logical dependency on an earlier NS
//!   batch — either a refinement chain (e.g. signature → body, where
//!   the later batch fills in lines the earlier one ellipsised) or a
//!   semantic prerequisite (the later batch only makes sense after the
//!   predecessor's concept is in). NS metadata, not a walker-scheduling
//!   gate.
//! - `Score(B=cum)` — Score(B) at `B = whichever cum is filled on this
//!   row`, three decimals. Computed by [`compute_score_at_running`]
//!   in a single forward pass that maintains running `walker_cum`
//!   and `a_b_atoms` state — the same function the snapshot path
//!   (`build_scores`) calls at each grid budget, so the per-row curve
//!   and the grid-aligned snapshot agree by construction.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};

use crate::batch::BatchId;
use crate::content::{BatchContent, FsEntries, Render, explode_spans, truncate_regex};
use crate::north_star::NorthStar;
use crate::ns_loader::resolve_content;
use crate::render::{RenderedTree, SourceCache};
use crate::schedule_types::{Atom, Schedule, ScheduledBatch};

mod render;
use render::format_report;

/// Per-budget Score grid — geometric on `[1000, 9000]` (ratio ⁶√9 ≈
/// 1.442), symmetric around 3000 on the log scale.
pub const BUDGETS: [usize; 7] = [1000, 1442, 2080, 3000, 4327, 6240, 9000];

/// Index of the primary budget. `Score(3000)` drives sort order.
pub const PRIMARY_BUDGET_INDEX: usize = 3;

/// `reached` threshold on damped credit at the primary budget.
const REACH_THRESHOLD: f64 = 0.8;

/// `missing` threshold on damped credit. `[MISSING_FLOOR, REACH_THRESHOLD)`
/// is `partial`.
const MISSING_FLOOR: f64 = 0.5;

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
    /// `√(importance × coverage)`.
    pub score: f64,
}

/// Headline scores. Bucket counts gate on NS rows with `exp_t ≤
/// PRIMARY_BUDGET` (`reached + partial + missing == rows_in_primary`).
#[derive(Debug, Clone)]
pub struct Scores {
    pub vector: [ScoreAtBudget; BUDGETS.len()],
    /// Total NS batches across all budgets.
    pub total_ns: usize,
    /// NS batches with `exp_t ≤ PRIMARY_BUDGET`.
    pub rows_in_primary: usize,
    /// Damped credit (credit × completion) ≥ [`REACH_THRESHOLD`].
    pub reached: usize,
    /// Damped credit in `[MISSING_FLOOR, REACH_THRESHOLD)`.
    pub partial: usize,
    /// Damped credit < [`MISSING_FLOOR`].
    pub missing: usize,
}

impl Scores {
    /// First line of any divergence report; full contents of a
    /// validation-tier baseline.
    pub fn headline(&self) -> String {
        let primary = &self.vector[PRIMARY_BUDGET_INDEX];
        format!(
            "Score(3000)={:.3} I={:.3} C={:.3} ns_rows≤3K={}/{} (reached={} partial={} missing={})",
            primary.score,
            primary.importance,
            primary.coverage,
            self.rows_in_primary,
            self.total_ns,
            self.reached,
            self.partial,
            self.missing,
        )
    }
}

pub(crate) const PRIMARY_BUDGET: usize = BUDGETS[PRIMARY_BUDGET_INDEX];

/// Compute scores. `schedule` is expected to be a full-cap walker run.
pub fn score(ns: &NorthStar, schedule: &Schedule, fixture_root: &Path) -> Result<Scores> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let walker = WalkerSnapshots::build(&ctx);
    Ok(build_scores(&ctx, &walker))
}

/// Generate the markdown divergence report (one per fixture).
pub fn generate_divergence_report(
    ns: &NorthStar,
    schedule: &Schedule,
    fixture_root: &Path,
) -> Result<String> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let walker = WalkerSnapshots::build(&ctx);
    let scores = build_scores(&ctx, &walker);
    Ok(format_report(&scores, &ctx))
}

// ---- graded atoms ------------------------------------------------------

/// One content atom with byte footprint for credit accounting (not
/// render-cost). `bytes ≥ 1` for any present atom; 0 means "walker
/// never rendered this atom". Credit between matching atoms is
/// `min(walker, ns) / max(ns, 1)`, capped at 1.0.
#[derive(Debug, Clone)]
pub(super) struct GradedAtom {
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

/// Per-render byte_end. Ellipsis is a 1-byte sentinel; all kinds
/// floor at 1 to keep "present" distinguishable from "not rendered".
fn byte_end_for(render: &Render, source_line: &str) -> usize {
    match render {
        Render::Full => source_line.len().max(1),
        Render::Ellipsis => 1,
        Render::Truncated { pattern } => truncate_regex(pattern)
            .and_then(|re| re.find(source_line).map(|m| m.end()))
            .unwrap_or(0)
            .max(1),
    }
}

// ---- context -----------------------------------------------------------

pub(super) struct BuildCtx<'a> {
    ns_rows: Vec<NsRow>,
    walker_rows: Vec<WalkerRow<'a>>,
    ns: &'a NorthStar,
}

struct NsRow {
    atoms: Vec<GradedAtom>,
    /// Cumulative tokens through this batch (same accounting as
    /// `simulate_ns`).
    exp_t: usize,
    /// 1-indexed rank of this row's first atom in the flat NS schedule.
    /// Atom `i`'s Importance weight is `1 / (rank_start + i)`.
    rank_start: usize,
}

struct WalkerRow<'a> {
    atoms: Vec<GradedAtom>,
    seen_t: usize,
    batch: &'a ScheduledBatch,
}

impl<'a> BuildCtx<'a> {
    fn new(ns: &'a NorthStar, schedule: &'a Schedule, fixture_root: &Path) -> Result<Self> {
        // Canonicalize so NS atoms and walker atoms land in the same
        // path namespace (`Schedule::root` is canonicalized too).
        let fixture_root = fixture_root
            .canonicalize()
            .with_context(|| format!("canonicalize fixture_root {}", fixture_root.display()))?;
        let fixture_root = fixture_root.as_path();
        let source_cache = SourceCache::new();
        let mut tree = RenderedTree::new(fixture_root.to_path_buf(), source_cache.clone());

        let mut ns_rows = Vec::with_capacity(ns.batches.len());
        let mut cum = 0usize;
        let mut next_rank = 1usize;
        for (pos, b) in ns.batches.iter().enumerate() {
            let content = resolve_content(&b.content, fixture_root)?;
            let atoms = atoms_from_content(&content, &source_cache, fixture_root);
            let marginal = tree.marginal_cost(&content);
            cum += marginal.tokens;
            let batch_id = BatchId::new(pos);
            // Predecessor-chain conflicts are `simulate_ns`'s concern.
            let _ = tree.apply(&content, batch_id, |_| true);
            let rank_start = next_rank;
            next_rank += atoms.len();
            ns_rows.push(NsRow {
                atoms,
                exp_t: cum,
                rank_start,
            });
        }

        let walker_rows: Vec<WalkerRow<'_>> = schedule
            .batches
            .iter()
            .map(|b| WalkerRow {
                atoms: atoms_from_content(&b.content, &source_cache, fixture_root),
                seen_t: b.cum_tokens,
                batch: b,
            })
            .collect();
        Ok(Self {
            ns_rows,
            walker_rows,
            ns,
        })
    }
}

// ---- scoring -----------------------------------------------------------

/// Per-budget walker state for `build_scores`.
struct WalkerSnapshots<'a> {
    cums: [BTreeMap<&'a Atom, usize>; BUDGETS.len()],
    a_b_atoms: [usize; BUDGETS.len()],
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
        Self { cums, a_b_atoms }
    }
}

fn build_scores(ctx: &BuildCtx, walker: &WalkerSnapshots) -> Scores {
    let vector: [ScoreAtBudget; BUDGETS.len()] = std::array::from_fn(|i| {
        compute_score_at_running(ctx, BUDGETS[i], &walker.cums[i], walker.a_b_atoms[i])
    });

    // reached/partial/missing on damped credit at the primary budget.
    let primary_cum = &walker.cums[PRIMARY_BUDGET_INDEX];
    let mut reached = 0;
    let mut partial = 0;
    let mut missing = 0;
    let mut rows_in_primary = 0;
    for row in &ctx.ns_rows {
        if row.exp_t > PRIMARY_BUDGET {
            continue;
        }
        rows_in_primary += 1;
        let credit = if row.atoms.is_empty() {
            0.0
        } else {
            let sum: f64 = row.atoms.iter().map(|a| atom_credit(a, primary_cum)).sum();
            sum / row.atoms.len() as f64
        };
        let completion = completion_for_row(&row.atoms, primary_cum);
        let damped = credit * completion;
        if damped < MISSING_FLOOR {
            missing += 1;
        } else if damped < REACH_THRESHOLD {
            partial += 1;
        } else {
            reached += 1;
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

/// Compute `ScoreAtBudget` from running state — the per-budget snapshot
/// path (`build_scores`) and the per-row schedule-table renderer both
/// route through here, so the per-row Score(B=cum) curve and the
/// grid-aligned snapshot agree by construction.
pub(super) fn compute_score_at_running(
    ctx: &BuildCtx,
    budget: usize,
    walker_cum: &BTreeMap<&Atom, usize>,
    a_b_atoms: usize,
) -> ScoreAtBudget {
    if a_b_atoms == 0 {
        return ScoreAtBudget {
            budget,
            a_b_atoms: 0,
            importance: 0.0,
            coverage: 0.0,
            score: 0.0,
        };
    }

    let ideal_denom: f64 = (1..=a_b_atoms).map(|k| 1.0 / k as f64).sum();

    let mut importance_num = 0.0;
    let mut coverage_sum = 0.0;
    for row in &ctx.ns_rows {
        let completion = completion_for_row(&row.atoms, walker_cum);
        if completion == 0.0 {
            continue;
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
    let score = (importance * coverage).sqrt();
    ScoreAtBudget {
        budget,
        a_b_atoms,
        importance,
        coverage,
        score,
    }
}

/// Byte-weighted completion: `Σ min(walker, ns) / Σ ns` over the
/// batch's atoms — the `completion(B_i)` factor in `damped_credit`.
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
    delivered as f64 / total as f64
}

fn atom_credit(atom: &GradedAtom, walker_cum: &BTreeMap<&Atom, usize>) -> f64 {
    let walker_bytes = walker_cum.get(&atom.atom).copied().unwrap_or(0);
    let ns_bytes = atom.bytes.max(1);
    ((walker_bytes.min(ns_bytes) as f64) / (ns_bytes as f64)).min(1.0)
}

/// Fold one walker batch's atoms into a cumulative byte-max map.
pub(super) fn fold_walker_atoms<'a>(cum: &mut BTreeMap<&'a Atom, usize>, atoms: &'a [GradedAtom]) {
    for wa in atoms {
        let entry = cum.entry(&wa.atom).or_insert(0);
        if wa.bytes > *entry {
            *entry = wa.bytes;
        }
    }
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
        fold_walker_atoms(&mut cumulative, &wr.atoms);
    }
    cumulative
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use crate::content::BatchContent;
    use crate::north_star::NorthStar;
    use crate::schedule_types::{Atom, ScheduledBatch};

    use super::{
        BUDGETS, BuildCtx, GradedAtom, NsRow, WalkerRow, WalkerSnapshots, compute_score_at_running,
    };

    fn line_atom(path: &str, line: usize, bytes: usize) -> GradedAtom {
        GradedAtom {
            atom: Atom::Line {
                path: PathBuf::from(path),
                line,
            },
            bytes,
        }
    }

    fn scheduled_batch(cum_tokens: usize) -> ScheduledBatch {
        ScheduledBatch {
            position: 0,
            key: String::new(),
            descriptor: String::new(),
            cost_tokens: 0,
            cum_tokens,
            content: BatchContent::Lines { spans: vec![] },
        }
    }

    /// A manual two-pointer forward pass over NS + walker rows
    /// (mirroring `format_schedule_table`'s per-row state advance) must
    /// produce the same `walker_cum` / `a_b_atoms` state at each grid
    /// budget as the snapshot path (`WalkerSnapshots::build`). Guards
    /// against off-by-one in `a_b_atoms`, missed `walker_cum` updates,
    /// or wrong tie-break ordering in the schedule-table renderer.
    #[test]
    fn divergence_running_score_matches_snapshot_at_each_budget() {
        // NorthStar reference only — `BuildCtx.ns` is read by the report
        // renderer, not by the scoring path under test.
        let ns = NorthStar {
            fixture: String::new(),
            revision_pin: String::new(),
            summary: String::new(),
            batches: vec![],
        };
        let ns_rows = vec![
            NsRow {
                atoms: vec![line_atom("src/a.rs", 1, 50), line_atom("src/a.rs", 2, 80)],
                exp_t: 200,
                rank_start: 1,
            },
            NsRow {
                atoms: vec![line_atom("src/b.rs", 1, 40)],
                exp_t: 2500,
                rank_start: 3,
            },
            NsRow {
                atoms: vec![line_atom("src/c.rs", 1, 60), line_atom("src/c.rs", 2, 30)],
                exp_t: 4500,
                rank_start: 4,
            },
            NsRow {
                atoms: vec![line_atom("src/d.rs", 1, 20)],
                exp_t: 8500,
                rank_start: 6,
            },
        ];

        let walker_batches: Vec<ScheduledBatch> = [50usize, 250, 1500, 3500, 5000, 8200]
            .into_iter()
            .map(scheduled_batch)
            .collect();
        // Walker delivers a subset of the NS atoms at varying budgets,
        // mixed with off-NS atoms so coverage / importance differ across
        // budgets. byte counts deliberately differ from NS atom bytes
        // (partial / full / over-delivery) to exercise atom_credit min().
        let walker_atoms: Vec<Vec<GradedAtom>> = vec![
            vec![line_atom("src/a.rs", 1, 30)],
            vec![line_atom("off.rs", 1, 100)],
            vec![line_atom("src/a.rs", 2, 80), line_atom("src/b.rs", 1, 40)],
            vec![line_atom("src/c.rs", 1, 200)],
            vec![line_atom("off.rs", 2, 50)],
            vec![line_atom("src/d.rs", 1, 20)],
        ];
        let walker_rows: Vec<WalkerRow<'_>> = walker_batches
            .iter()
            .zip(walker_atoms)
            .map(|(b, atoms)| WalkerRow {
                atoms,
                seen_t: b.cum_tokens,
                batch: b,
            })
            .collect();

        let ctx = BuildCtx {
            ns_rows,
            walker_rows,
            ns: &ns,
        };
        let walker = WalkerSnapshots::build(&ctx);

        // Manual forward pass mirroring `format_schedule_table` state
        // advance: at each grid budget, advance NS + walker rows with
        // cum ≤ budget, then parity-check the running score against the
        // precomputed snapshot.
        let mut walker_cum: BTreeMap<&Atom, usize> = BTreeMap::new();
        let mut a_b_atoms: usize = 0;
        let mut ns_idx = 0;
        let mut walker_idx = 0;

        for (i, &budget) in BUDGETS.iter().enumerate() {
            while ns_idx < ctx.ns_rows.len() && ctx.ns_rows[ns_idx].exp_t <= budget {
                a_b_atoms += ctx.ns_rows[ns_idx].atoms.len();
                ns_idx += 1;
            }
            while walker_idx < ctx.walker_rows.len() && ctx.walker_rows[walker_idx].seen_t <= budget
            {
                let wr = &ctx.walker_rows[walker_idx];
                super::fold_walker_atoms(&mut walker_cum, &wr.atoms);
                walker_idx += 1;
            }
            let snap = compute_score_at_running(&ctx, budget, &walker.cums[i], walker.a_b_atoms[i]);
            let run = compute_score_at_running(&ctx, budget, &walker_cum, a_b_atoms);
            assert_eq!(snap.budget, run.budget, "budget at i={i}");
            assert_eq!(snap.a_b_atoms, run.a_b_atoms, "a_b_atoms at i={i}");
            assert!(
                (snap.importance - run.importance).abs() < 1e-12,
                "importance at i={i}: snap={} run={}",
                snap.importance,
                run.importance,
            );
            assert!(
                (snap.coverage - run.coverage).abs() < 1e-12,
                "coverage at i={i}: snap={} run={}",
                snap.coverage,
                run.coverage,
            );
            assert!(
                (snap.score - run.score).abs() < 1e-12,
                "score at i={i}: snap={} run={}",
                snap.score,
                run.score,
            );
        }
    }
}
