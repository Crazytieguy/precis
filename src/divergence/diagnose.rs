//! Diagnostic-only decomposition of `Score(3000)` loss, plus an
//! NS-aware oracle ceiling. Driven by `src/bin/diagnose_loss.rs`; not
//! part of the shipped CLI.
//!
//! Two questions this answers, per fixture:
//!
//! 1. **Where does the coverage loss at the primary budget come from?**
//!    Every A_3K atom's shortfall is attributed to exactly one bucket:
//!    - `damping` — atom delivered, but its enclosing NS batch is
//!      incomplete ("finish what you start" multiplier).
//!    - `partial_render` — delivered with fewer bytes than the NS render
//!      (truncated / ellipsis vs full line).
//!    - `late` — absent at 3000, delivered later in the capped schedule.
//!    - `unscheduled` — absent from the capped schedule, but the walker
//!      *can* emit it (present in the full expansion-fixpoint pool).
//!    - `absent` — not emitted by any reachable walker batch: a recall
//!      gap that no scheduler change can fix.
//!
//! 2. **How much could a perfect scheduler recover?** The oracle greedily
//!    schedules walker batches from the full expansion pool to maximize
//!    the real `Score(3000)` (predecessor chains respected, real
//!    marginal costs). `oracle − actual` is the re-ranking headroom;
//!    `1 − oracle` is roughly the walker-content gap (recall + render
//!    fidelity), unreachable by scheduling alone.
//!
//! The oracle is a lower bound on the true ceiling (greedy, and chain
//! costs are estimated as sums of standalone marginal costs, which
//! overcounts intra-chain overlap when ranking candidates).

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::batch::{Batch, BatchId, BatchKey, WalkerKey};
use crate::north_star::NorthStar;
use crate::render::{RenderedTree, SourceCache};
use crate::schedule_types::{Atom, Schedule};
use crate::walker::{FsWalker, WalkCtx, Walker};

use super::{
    BuildCtx, GradedAtom, PRIMARY_BUDGET, Scores, WalkerSnapshots, atom_credit, atoms_from_content,
    build_scores, completion_for_row, walker_cum_at,
};

/// Coverage-loss attribution at the primary budget. All fields are
/// fractions of `|A_3K|`; the delivered share plus `damping +
/// partial_render + late + unscheduled + absent ≈ 1`.
#[derive(Debug, Clone, Copy)]
pub struct Buckets {
    pub damping: f64,
    pub partial_render: f64,
    pub late: f64,
    pub unscheduled: f64,
    pub absent: f64,
}

/// NS-aware greedy schedule over the full walker pool.
#[derive(Debug, Clone)]
pub struct OracleResult {
    pub score: f64,
    pub used_tokens: usize,
    /// The oracle's purchases in order: (descriptor, exact tokens).
    pub schedule: Vec<(String, usize)>,
}

/// One file's contribution to a loss bucket (for lever hunting).
#[derive(Debug, Clone)]
pub struct FileLoss {
    pub path: PathBuf,
    pub bucket: &'static str,
    /// Fraction of `|A_3K|` lost to this (file, bucket) pair.
    pub loss: f64,
}

#[derive(Debug)]
pub struct LossReport {
    pub baseline: Scores,
    pub buckets: Buckets,
    pub oracle: OracleResult,
    pub pool_batches: usize,
    /// Largest per-file loss contributors, descending.
    pub top_file_losses: Vec<FileLoss>,
}

/// Cap on the expansion fixpoint — a runaway-walker backstop, far above
/// any corpus fixture's real pool.
const POOL_CAP: usize = 300_000;

pub fn diagnose(ns: &NorthStar, schedule: &Schedule, fixture_root: &Path) -> Result<LossReport> {
    let ctx = BuildCtx::new(ns, schedule, fixture_root)?;
    let baseline = build_scores(&ctx, &WalkerSnapshots::build(&ctx));

    let canon_root = fixture_root
        .canonicalize()
        .with_context(|| format!("canonicalize {}", fixture_root.display()))?;

    let source_cache = SourceCache::new();
    let pool = full_pool(&canon_root, &source_cache);
    let pool_atoms: Vec<Vec<GradedAtom>> = pool
        .iter()
        .map(|b| atoms_from_content(&b.content, &source_cache, &canon_root))
        .collect();
    let pool_atom_set: HashSet<&Atom> = pool_atoms
        .iter()
        .flat_map(|atoms| atoms.iter().map(|a| &a.atom))
        .collect();

    let cum_3k = walker_cum_at(&ctx, PRIMARY_BUDGET);
    let cum_cap = walker_cum_at(&ctx, usize::MAX);

    let (buckets, top_file_losses) = decompose(&ctx, &cum_3k, &cum_cap, &pool_atom_set);
    let oracle = run_oracle(&ctx, &pool, &pool_atoms, &canon_root, &source_cache);

    Ok(LossReport {
        baseline,
        buckets,
        oracle,
        pool_batches: pool.len(),
        top_file_losses,
    })
}

/// Expansion fixpoint: absorb seeds, expand every absorbed batch
/// regardless of scheduling. The full inventory the walker can emit.
fn full_pool(canon_root: &Path, source_cache: &SourceCache) -> Vec<Batch<BatchKey>> {
    let ctx = WalkCtx::with_cache(canon_root.to_path_buf(), source_cache.clone());
    let mut walker = FsWalker;
    let mut seen: HashSet<BatchKey> = HashSet::new();
    let mut queue: VecDeque<Batch<BatchKey>> = walker.seed(&ctx).into();
    let mut out = Vec::new();
    while let Some(batch) = queue.pop_front() {
        if !seen.insert(batch.key.clone()) {
            continue;
        }
        if out.len() >= POOL_CAP {
            eprintln!(
                "warning: pool cap {POOL_CAP} hit at {}",
                canon_root.display()
            );
            break;
        }
        queue.extend(walker.expand(&batch.key, &ctx));
        out.push(batch);
    }
    out
}

fn decompose(
    ctx: &BuildCtx,
    cum_3k: &std::collections::BTreeMap<&Atom, usize>,
    cum_cap: &std::collections::BTreeMap<&Atom, usize>,
    pool_atom_set: &HashSet<&Atom>,
) -> (Buckets, Vec<FileLoss>) {
    let mut damping = 0.0;
    let mut partial_render = 0.0;
    let mut late = 0.0;
    let mut unscheduled = 0.0;
    let mut absent = 0.0;
    let mut a_3k_atoms = 0usize;
    // (path, bucket) → summed loss.
    let mut file_losses: HashMap<(PathBuf, &'static str), f64> = HashMap::new();

    for row in &ctx.ns_rows {
        if row.exp_t > PRIMARY_BUDGET {
            continue;
        }
        a_3k_atoms += row.atoms.len();
        let completion = completion_for_row(&row.atoms, cum_3k);
        for atom in &row.atoms {
            let credit = atom_credit(atom, cum_3k);
            damping += credit * (1.0 - completion);
            let credit_loss = 1.0 - credit;
            if credit_loss <= 0.0 {
                continue;
            }
            let bucket = if credit > 0.0 {
                partial_render += credit_loss;
                "partial_render"
            } else if cum_cap.get(&atom.atom).copied().unwrap_or(0) > 0 {
                late += credit_loss;
                "late"
            } else if pool_atom_set.contains(&atom.atom) {
                unscheduled += credit_loss;
                "unscheduled"
            } else {
                absent += credit_loss;
                "absent"
            };
            let path = match &atom.atom {
                Atom::Line { path, .. } => path.clone(),
                Atom::Fs { parent, .. } => parent.clone(),
            };
            *file_losses.entry((path, bucket)).or_insert(0.0) += credit_loss;
        }
    }

    let n = a_3k_atoms.max(1) as f64;
    let mut top: Vec<FileLoss> = file_losses
        .into_iter()
        .map(|((path, bucket), loss)| FileLoss {
            path,
            bucket,
            loss: loss / n,
        })
        .collect();
    top.sort_by(|a, b| b.loss.total_cmp(&a.loss));
    top.truncate(8);

    (
        Buckets {
            damping: damping / n,
            partial_render: partial_render / n,
            late: late / n,
            unscheduled: unscheduled / n,
            absent: absent / n,
        },
        top,
    )
}

// ---- oracle ------------------------------------------------------------

/// Per-NS-row scoring state for incremental oracle evaluation.
struct RowState {
    /// (atom identity index into `atom_ns_bytes`, rank, ns_bytes).
    atoms: Vec<(Atom, usize, usize)>,
    total_ns_bytes: usize,
    in_a: bool,
}

struct OracleEval {
    rows: Vec<RowState>,
    /// Atom → rows containing it (row_idx).
    atom_rows: HashMap<Atom, Vec<usize>>,
    /// Current delivered bytes per atom (max-folded).
    delivered: HashMap<Atom, usize>,
    /// Cached per-row (importance contribution, coverage contribution).
    row_contrib: Vec<(f64, f64)>,
    importance_num: f64,
    coverage_num: f64,
    ideal_denom: f64,
    a_b_atoms: usize,
}

impl OracleEval {
    fn new(ctx: &BuildCtx) -> Self {
        let mut rows = Vec::with_capacity(ctx.ns_rows.len());
        let mut atom_rows: HashMap<Atom, Vec<usize>> = HashMap::new();
        let mut a_b_atoms = 0usize;
        for (row_idx, row) in ctx.ns_rows.iter().enumerate() {
            let in_a = row.exp_t <= PRIMARY_BUDGET;
            if in_a {
                a_b_atoms += row.atoms.len();
            }
            let mut atoms = Vec::with_capacity(row.atoms.len());
            let mut total = 0usize;
            for (i, a) in row.atoms.iter().enumerate() {
                let ns_bytes = a.bytes.max(1);
                total += ns_bytes;
                atoms.push((a.atom.clone(), row.rank_start + i, ns_bytes));
                atom_rows.entry(a.atom.clone()).or_default().push(row_idx);
            }
            rows.push(RowState {
                atoms,
                total_ns_bytes: total.max(1),
                in_a,
            });
        }
        let ideal_denom: f64 = (1..=a_b_atoms).map(|k| 1.0 / k as f64).sum();
        let row_contrib = vec![(0.0, 0.0); rows.len()];
        Self {
            rows,
            atom_rows,
            delivered: HashMap::new(),
            row_contrib,
            importance_num: 0.0,
            coverage_num: 0.0,
            ideal_denom,
            a_b_atoms,
        }
    }

    /// Row contributions under an optional delivered-bytes overlay.
    fn row_contrib_with(
        &self,
        row_idx: usize,
        overlay: Option<&HashMap<Atom, usize>>,
    ) -> (f64, f64) {
        let row = &self.rows[row_idx];
        let mut delivered_min = 0usize;
        let mut cr_rank = 0.0;
        let mut cr_flat = 0.0;
        for (atom, rank, ns_bytes) in &row.atoms {
            let cur = overlay
                .and_then(|o| o.get(atom).copied())
                .or_else(|| self.delivered.get(atom).copied())
                .unwrap_or(0);
            let capped = cur.min(*ns_bytes);
            delivered_min += capped;
            let credit = capped as f64 / *ns_bytes as f64;
            cr_rank += credit / *rank as f64;
            cr_flat += credit;
        }
        let completion = delivered_min as f64 / row.total_ns_bytes as f64;
        let imp = completion * cr_rank;
        let cov = if row.in_a { completion * cr_flat } else { 0.0 };
        (imp, cov)
    }

    fn score_from(&self, importance_num: f64, coverage_num: f64) -> f64 {
        if self.a_b_atoms == 0 {
            return 0.0;
        }
        let importance = (importance_num / self.ideal_denom).min(1.0);
        let coverage = coverage_num / self.a_b_atoms as f64;
        (importance * coverage).sqrt()
    }

    fn current_score(&self) -> f64 {
        self.score_from(self.importance_num, self.coverage_num)
    }

    /// Score if `new_atoms` (atom → candidate bytes) were max-folded in.
    fn score_with(&self, new_atoms: &[(Atom, usize)]) -> f64 {
        let mut overlay: HashMap<Atom, usize> = HashMap::new();
        let mut affected: HashSet<usize> = HashSet::new();
        for (atom, bytes) in new_atoms {
            let cur = self.delivered.get(atom).copied().unwrap_or(0);
            if *bytes <= cur {
                continue;
            }
            let entry = overlay.entry(atom.clone()).or_insert(cur);
            if *bytes > *entry {
                *entry = *bytes;
            }
            if let Some(rows) = self.atom_rows.get(atom) {
                affected.extend(rows.iter().copied());
            }
        }
        if affected.is_empty() {
            return self.current_score();
        }
        let mut imp = self.importance_num;
        let mut cov = self.coverage_num;
        for &row_idx in &affected {
            let (old_i, old_c) = self.row_contrib[row_idx];
            let (new_i, new_c) = self.row_contrib_with(row_idx, Some(&overlay));
            imp += new_i - old_i;
            cov += new_c - old_c;
        }
        self.score_from(imp, cov)
    }

    /// Commit atoms (max-fold) and refresh affected rows.
    fn commit(&mut self, new_atoms: &[(Atom, usize)]) {
        let mut affected: HashSet<usize> = HashSet::new();
        for (atom, bytes) in new_atoms {
            let entry = self.delivered.entry(atom.clone()).or_insert(0);
            if *bytes > *entry {
                *entry = *bytes;
                if let Some(rows) = self.atom_rows.get(atom) {
                    affected.extend(rows.iter().copied());
                }
            }
        }
        for row_idx in affected {
            let (old_i, old_c) = self.row_contrib[row_idx];
            let (new_i, new_c) = self.row_contrib_with(row_idx, None);
            self.importance_num += new_i - old_i;
            self.coverage_num += new_c - old_c;
            self.row_contrib[row_idx] = (new_i, new_c);
        }
    }

    fn final_result(&self, used_tokens: usize, schedule: Vec<(String, usize)>) -> OracleResult {
        let importance = if self.a_b_atoms == 0 {
            0.0
        } else {
            (self.importance_num / self.ideal_denom).min(1.0)
        };
        let coverage = if self.a_b_atoms == 0 {
            0.0
        } else {
            self.coverage_num / self.a_b_atoms as f64
        };
        OracleResult {
            score: (importance * coverage).sqrt(),
            used_tokens,
            schedule,
        }
    }
}

/// Pool batches worth considering: those whose atoms touch any NS row,
/// plus their predecessor closure.
fn relevant_indices(
    ctx: &BuildCtx,
    pool: &[Batch<BatchKey>],
    pool_atoms: &[Vec<GradedAtom>],
) -> Vec<usize> {
    let ns_atoms: HashSet<&Atom> = ctx
        .ns_rows
        .iter()
        .flat_map(|r| r.atoms.iter().map(|a| &a.atom))
        .collect();
    let key_to_idx: HashMap<&BatchKey, usize> =
        pool.iter().enumerate().map(|(i, b)| (&b.key, i)).collect();
    let mut relevant: HashSet<usize> = HashSet::new();
    for (i, atoms) in pool_atoms.iter().enumerate() {
        if atoms.iter().any(|a| ns_atoms.contains(&a.atom)) {
            relevant.insert(i);
            // Predecessor closure.
            let mut cur = pool[i].predecessor.as_ref();
            while let Some(pred) = cur {
                let Some(&pi) = key_to_idx.get(pred) else {
                    break;
                };
                if !relevant.insert(pi) {
                    break;
                }
                cur = pool[pi].predecessor.as_ref();
            }
        }
    }
    let mut out: Vec<usize> = relevant.into_iter().collect();
    out.sort_unstable();
    out
}

fn run_oracle(
    ctx: &BuildCtx,
    pool: &[Batch<BatchKey>],
    pool_atoms: &[Vec<GradedAtom>],
    canon_root: &Path,
    source_cache: &SourceCache,
) -> OracleResult {
    let relevant = relevant_indices(ctx, pool, pool_atoms);
    let key_to_idx: HashMap<&BatchKey, usize> =
        pool.iter().enumerate().map(|(i, b)| (&b.key, i)).collect();

    let mut eval = OracleEval::new(ctx);
    let mut tree = RenderedTree::new(canon_root.to_path_buf(), source_cache.clone());
    let mut scheduled: HashSet<usize> = HashSet::new();
    let mut purchases: Vec<(String, usize)> = Vec::new();
    let mut used = 0usize;

    loop {
        let remaining = PRIMARY_BUDGET.saturating_sub(used);
        if remaining == 0 {
            break;
        }
        // Standalone marginal costs are recomputed each step — applied
        // content changes overlapping batches' marginals along chains.
        let mut cost_cache: HashMap<usize, usize> = HashMap::new();
        let cur_score = eval.current_score();
        let mut best: Option<(f64, f64, Vec<usize>, usize)> = None; // (ratio, gain, chain, cost)

        for &i in &relevant {
            if scheduled.contains(&i) {
                continue;
            }
            // Build the unscheduled predecessor chain, root-first.
            let mut chain = vec![i];
            let mut cur = pool[i].predecessor.as_ref();
            let mut broken = false;
            while let Some(pred) = cur {
                let Some(&pi) = key_to_idx.get(pred) else {
                    broken = true;
                    break;
                };
                if scheduled.contains(&pi) {
                    break;
                }
                chain.push(pi);
                cur = pool[pi].predecessor.as_ref();
            }
            if broken {
                continue;
            }
            chain.reverse();

            let mut cost = 0usize;
            for &m in &chain {
                let c = *cost_cache
                    .entry(m)
                    .or_insert_with(|| tree.marginal_cost(&pool[m].content).tokens);
                cost += c;
            }
            if cost > remaining {
                continue;
            }
            let new_atoms: Vec<(Atom, usize)> = chain
                .iter()
                .flat_map(|&m| pool_atoms[m].iter().map(|a| (a.atom.clone(), a.bytes)))
                .collect();
            let gain = eval.score_with(&new_atoms) - cur_score;
            if gain <= 1e-12 {
                continue;
            }
            let ratio = gain / cost.max(1) as f64;
            if best.as_ref().is_none_or(|(br, ..)| ratio > *br) {
                best = Some((ratio, gain, chain, cost));
            }
        }

        let Some((_, _, chain, _)) = best else {
            break;
        };
        for &m in &chain {
            let exact = tree.marginal_cost(&pool[m].content).tokens;
            if used + exact > PRIMARY_BUDGET {
                break;
            }
            tree.apply(&pool[m].content, BatchId::new(m), |_| true);
            used += exact;
            scheduled.insert(m);
            purchases.push((pool[m].key.describe(canon_root), exact));
            let atoms: Vec<(Atom, usize)> = pool_atoms[m]
                .iter()
                .map(|a| (a.atom.clone(), a.bytes))
                .collect();
            eval.commit(&atoms);
        }
    }

    eval.final_result(used, purchases)
}
