//! Divergence metric and report: how closely a walker [`Schedule`] run at
//! the NS cap follows a frozen [`NorthStar`].
//!
//! ## Score
//!
//! `Score(B) = √(I(B) · C(B))`. `A_B` is the NS atoms in NS batches whose
//! cumulative cost `exp_t ≤ B`; every NS atom has a 1-based rank from a
//! flat traversal of the NS batches in order.
//!
//! - `I(B)`, Importance: `min(1, Σ_a damped(a) / rank(a) / Σ_{k ≤ |A_B|} 1/k)`
//!   over all NS atoms, so early delivery of an atom past `A_B` still counts.
//! - `C(B)`, Coverage: `Σ_{rank ≤ |A_B|} damped(a) / |A_B|`.
//! - `damped(a) = credit(a) · completion(batch(a))`, where
//!   `credit(a) = min(walker_bytes, ns_bytes) / ns_bytes` and `completion` is
//!   the byte-weighted credit of the atom's whole NS batch ("finish what you
//!   start").
//!
//! Atoms are source lines `(path, line)` and listing entries
//! `(parent, name)`. An atom's bytes are its rendered footprint: the line
//! length for a full line, the regex match end for a truncated one, 1 for an
//! ellipsis or a listing entry. Walker atoms from batches with
//! `cum_tokens ≤ B`, plus the affordable prefix of the next batch, count
//! toward `Score(B)`, each at the largest footprint the walker delivered.
//! Scheduler prefix-monotonicity makes that the walker's output at budget
//! `B`. NS costs `exp_t` are recomputed through
//! the current renderer and tokenizer, so a render-format change re-prices
//! the answer key.
//!
//! ## Report
//!
//! The first line is the headline, `Score(3000)=… I=… C=… ns_rows≤3K=N/T`
//! (the whole validation baseline), followed on training reports by
//! `grid(1000/…/9000)=…`, `Score(B)` at every [`BUDGETS`] entry.
//! `scripts/grid-means.sh` averages the grid across the training reports.
//!
//! The table merges NS rows (at `ns_cum = exp_t`, `marginal` its cost) and
//! walker rows (at `walker_cum`, `marginal` its cost) on one cumulative
//! token axis, walker rows first on ties. `id` and `predecessor` are the NS
//! batch's fields. `Score(B=cum)` is `Score` at that row's cum, computed
//! once all rows sharing the cum are folded in.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::batch::BatchId;
use crate::content::{BatchContent, FsEntries, Render, explode_spans, with_truncate_regex};
use crate::north_star::NorthStar;
use crate::ns_loader::resolve_content;
use crate::render::{RenderedTree, SourceCache};
use crate::scheduler::{ScheduledBatchRecord, Scheduler};
use crate::walker::FsWalker;

/// Geometric on `[1000, 9000]` (ratio ⁶√9), symmetric around 3000 on the
/// log scale.
const BUDGETS: [usize; 7] = [1000, 1442, 2080, 3000, 4327, 6240, 9000];

const PRIMARY_BUDGET_INDEX: usize = 3;

#[derive(Debug, Clone, Copy, Default)]
pub struct BudgetScore {
    pub importance: f64,
    pub coverage: f64,
    /// `√(importance × coverage)`.
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct Scores {
    /// `Score(B)` for each of [`BUDGETS`].
    grid: [BudgetScore; BUDGETS.len()],
    total_ns_rows: usize,
    /// NS rows with `exp_t ≤ 3000`.
    primary_ns_rows: usize,
}

impl Scores {
    pub fn primary(&self) -> &BudgetScore {
        &self.grid[PRIMARY_BUDGET_INDEX]
    }

    /// The whole validation baseline, and the start of a report's first line.
    pub fn headline(&self) -> String {
        let primary = self.primary();
        format!(
            "Score({})={:.3} I={:.3} C={:.3} ns_rows≤3K={}/{}",
            BUDGETS[PRIMARY_BUDGET_INDEX],
            primary.score,
            primary.importance,
            primary.coverage,
            self.primary_ns_rows,
            self.total_ns_rows,
        )
    }

    /// Training reports only: validation keeps to the primary score so the
    /// held-out set leaks as little as possible.
    fn grid_summary(&self) -> String {
        let budgets: Vec<String> = BUDGETS.iter().map(usize::to_string).collect();
        let scores: Vec<String> = self
            .grid
            .iter()
            .map(|s| format!("{:.3}", s.score))
            .collect();
        format!("grid({})={}", budgets.join("/"), scores.join("/"))
    }
}

/// Complete walker schedule from one [`render_schedule`] run, in
/// scheduling order.
pub struct Schedule {
    /// Canonical fixture root; batch content paths are absolute under it.
    pub root: PathBuf,
    pub batches: Vec<ScheduledBatchRecord>,
}

/// Run the walker at `budget` and return a [`Schedule`] — input for
/// regression snapshots and the divergence metric.
pub fn render_schedule(path: &Path, budget: usize) -> Result<Schedule> {
    let filter = crate::walk_scope(path)?;
    let root = filter.root().to_path_buf();
    let report = Scheduler::with_filter(filter, FsWalker, budget, None).run_with_report();
    Ok(Schedule {
        root,
        batches: report.scheduled,
    })
}

/// Replay a [`Schedule`] against a fresh tree at `budget`. Under
/// prefix-monotone scheduling this matches running the walker at the
/// smaller budget.
pub fn render_with_schedule(schedule: &Schedule, budget: usize) -> String {
    let mut tree = RenderedTree::new(schedule.root.clone(), SourceCache::new());
    replay(schedule, &mut tree, budget);
    tree.render()
}

/// Apply to `tree` what the walker run at `budget` schedules: the batches
/// whose cumulative cost fits, then the affordable prefix of the next
/// one. Returns that prefix.
fn replay(schedule: &Schedule, tree: &mut RenderedTree, budget: usize) -> Option<BatchContent> {
    let mut spent = 0;
    for (i, batch) in schedule.batches.iter().enumerate() {
        if batch.cum_tokens > budget {
            let (prefix, _) =
                tree.affordable_prefix(&batch.content, |cost| spent + cost.tokens <= budget)?;
            tree.apply(&prefix, BatchId::new(i), |_| true);
            return Some(prefix);
        }
        tree.apply(&batch.content, BatchId::new(i), |_| true);
        spent = batch.cum_tokens;
    }
    None
}

/// Markdown report for one fixture plus the scores in its headline.
pub fn generate_divergence_report(
    ns: &NorthStar,
    schedule: &Schedule,
    fixture_root: &Path,
) -> Result<(String, Scores)> {
    Ok(Graded::new(ns, schedule, fixture_root)?.report())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Atom {
    Line { path: PathBuf, line: usize },
    Fs { parent: PathBuf, entry: String },
}

/// Atoms of `content` with their byte footprints (each ≥ 1).
fn graded_atoms(
    content: &BatchContent,
    source_cache: &SourceCache,
    fixture_root: &Path,
) -> Vec<(Atom, usize)> {
    match content {
        BatchContent::Fs { groups } => groups
            .iter()
            .flat_map(|g| {
                let FsEntries::Listed(paths) = &g.entries else {
                    unreachable!("unresolved FsEntries at {}", g.parent.display());
                };
                paths.iter().filter_map(|p| {
                    let entry = p.file_name()?.to_str()?.to_string();
                    let parent = g.parent.clone();
                    Some((Atom::Fs { parent, entry }, 1))
                })
            })
            .collect(),
        BatchContent::Lines { spans } => explode_spans(spans)
            .into_iter()
            .map(|(path, line, render)| {
                let source = source_cache.get(&fixture_root.join(&path));
                let source_line = source
                    .as_deref()
                    .and_then(|s| s.line(line))
                    .unwrap_or("");
                let bytes = match render {
                    Render::Full => source_line.len(),
                    Render::Ellipsis => 1,
                    Render::Truncated { pattern } => with_truncate_regex(&pattern, |re| {
                        re.and_then(|re| re.find(source_line).map(|m| m.end()))
                    })
                    .unwrap_or(0),
                };
                (Atom::Line { path, line }, bytes.max(1))
            })
            .collect(),
    }
}

/// NS and walker content reduced to interned NS atoms.
struct Graded<'a> {
    ns: &'a NorthStar,
    schedule: &'a Schedule,
    ns_rows: Vec<NsRow>,
    /// Per walker batch: `(atom id, bytes)` for each of its atoms the NS
    /// also has. Other walker atoms can never earn credit.
    walker_rows: Vec<Vec<(usize, usize)>>,
    /// Per atom id: `(NS row, NS bytes)` for every NS row holding it.
    occurrences: Vec<Vec<(usize, usize)>>,
    /// Per [`BUDGETS`] entry: the graded atoms of the partial batch the
    /// walker run at that budget ends on.
    partial_rows: Vec<Vec<(usize, usize)>>,
}

struct NsRow {
    /// `(atom id, NS bytes)` in rank order.
    atoms: Vec<(usize, usize)>,
    total_bytes: usize,
    exp_t: usize,
    /// Rank of the row's first atom.
    rank_start: usize,
}

impl<'a> Graded<'a> {
    fn new(ns: &'a NorthStar, schedule: &'a Schedule, fixture_root: &Path) -> Result<Self> {
        // `Schedule::root` is canonical; NS paths must resolve into the
        // same namespace for atoms to match.
        let fixture_root = fixture_root
            .canonicalize()
            .with_context(|| format!("canonicalize fixture_root {}", fixture_root.display()))?;
        let source_cache = SourceCache::new();
        let mut tree = RenderedTree::new(fixture_root.clone(), source_cache.clone());

        let mut ids: HashMap<Atom, usize> = HashMap::new();
        let mut occurrences: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut ns_rows = Vec::with_capacity(ns.batches.len());
        let mut exp_t = 0;
        let mut next_rank = 1;
        for (position, batch) in ns.batches.iter().enumerate() {
            let content = resolve_content(&batch.content, &fixture_root)?;
            exp_t += tree.marginal_cost(&content).tokens;
            // Predecessor-chain conflicts are `simulate_ns`'s concern.
            let _ = tree.apply(&content, BatchId::new(position), |_| true);
            let atoms: Vec<(usize, usize)> = graded_atoms(&content, &source_cache, &fixture_root)
                .into_iter()
                .map(|(atom, bytes)| {
                    let next_id = ids.len();
                    let id = *ids.entry(atom).or_insert(next_id);
                    if id == occurrences.len() {
                        occurrences.push(Vec::new());
                    }
                    occurrences[id].push((ns_rows.len(), bytes));
                    (id, bytes)
                })
                .collect();
            let rank_start = next_rank;
            next_rank += atoms.len();
            ns_rows.push(NsRow {
                total_bytes: atoms.iter().map(|&(_, bytes)| bytes).sum(),
                atoms,
                exp_t,
                rank_start,
            });
        }

        let grade = |content: &BatchContent| -> Vec<(usize, usize)> {
            graded_atoms(content, &source_cache, &fixture_root)
                .into_iter()
                .filter_map(|(atom, bytes)| Some((*ids.get(&atom)?, bytes)))
                .collect()
        };
        let walker_rows = schedule.batches.iter().map(|b| grade(&b.content)).collect();
        let partial_rows = BUDGETS
            .iter()
            .map(|&budget| {
                let mut tree = RenderedTree::new(schedule.root.clone(), source_cache.clone());
                replay(schedule, &mut tree, budget).map_or_else(Vec::new, |prefix| grade(&prefix))
            })
            .collect();
        Ok(Self {
            ns,
            schedule,
            ns_rows,
            walker_rows,
            occurrences,
            partial_rows,
        })
    }

    fn report(&self) -> (String, Scores) {
        let mut out = String::from(
            "\n| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |\n\
             |:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|\n",
        );
        let mut state = Running::new(self);
        let mut grid = [BudgetScore::default(); BUDGETS.len()];
        let mut grid_index = 0;
        let (mut ns_index, mut walker_index) = (0, 0);
        let mut prev_exp_t = 0;
        let mut pending_rows: Vec<String> = Vec::new();
        loop {
            let next_ns = self.ns_rows.get(ns_index).map(|row| row.exp_t);
            let next_walker = self
                .schedule
                .batches
                .get(walker_index)
                .map(|b| b.cum_tokens);
            let Some(cum) = next_ns.into_iter().chain(next_walker).min() else {
                break;
            };
            while grid_index < BUDGETS.len() && BUDGETS[grid_index] < cum {
                grid[grid_index] = state.score_with_partial(&self.partial_rows[grid_index]);
                grid_index += 1;
            }
            // Every row at this cum is folded in before scoring, so tied rows
            // share one Score(B=cum).
            pending_rows.clear();
            while let Some(batch) = self
                .schedule
                .batches
                .get(walker_index)
                .filter(|b| b.cum_tokens == cum)
            {
                state.fold_walker_row(&self.walker_rows[walker_index]);
                pending_rows.push(format!(
                    "| walker |  | {cum} | {} | {} |  |  |",
                    batch.cost.tokens,
                    escape_cell(&batch.key.describe(&self.schedule.root)),
                ));
                walker_index += 1;
            }
            while let Some(row) = self.ns_rows.get(ns_index).filter(|r| r.exp_t == cum) {
                state.add_ns_row(row);
                let batch = &self.ns.batches[ns_index];
                pending_rows.push(format!(
                    "| ns | {cum} |  | {} | {} | {} | {} |",
                    cum - prev_exp_t,
                    escape_cell(&batch.descriptor),
                    escape_cell(&batch.id),
                    escape_cell(batch.predecessor.as_deref().unwrap_or("")),
                ));
                prev_exp_t = cum;
                ns_index += 1;
            }
            let score = state.score().score;
            for row in &pending_rows {
                writeln!(out, "{row} {score:.3} |").unwrap();
            }
        }
        for (index, budget_score) in grid.iter_mut().enumerate().skip(grid_index) {
            *budget_score = state.score_with_partial(&self.partial_rows[index]);
        }

        let scores = Scores {
            grid,
            total_ns_rows: self.ns_rows.len(),
            primary_ns_rows: self
                .ns_rows
                .iter()
                .filter(|row| row.exp_t <= BUDGETS[PRIMARY_BUDGET_INDEX])
                .count(),
        };
        let first_line = format!("{} {}\n", scores.headline(), scores.grid_summary());
        out.insert_str(0, &first_line);
        (out, scores)
    }
}

/// Walker and NS progress along the merged cumulative axis.
#[derive(Clone)]
struct Running<'g> {
    graded: &'g Graded<'g>,
    /// Largest footprint the walker has delivered per atom id.
    walker_bytes: Vec<usize>,
    /// Per NS row: `Σ min(walker, ns)` bytes over its atoms.
    delivered: Vec<usize>,
    /// `|A_B|`.
    a_b_atoms: usize,
    /// `Σ_{k ≤ |A_B|} 1/k`.
    ideal_importance: f64,
}

impl<'g> Running<'g> {
    fn new(graded: &'g Graded<'g>) -> Self {
        Self {
            graded,
            walker_bytes: vec![0; graded.occurrences.len()],
            delivered: vec![0; graded.ns_rows.len()],
            a_b_atoms: 0,
            ideal_importance: 0.0,
        }
    }

    fn fold_walker_row(&mut self, atoms: &[(usize, usize)]) {
        for &(id, bytes) in atoms {
            let old = self.walker_bytes[id];
            if bytes <= old {
                continue;
            }
            self.walker_bytes[id] = bytes;
            for &(row, ns_bytes) in &self.graded.occurrences[id] {
                self.delivered[row] += bytes.min(ns_bytes) - old.min(ns_bytes);
            }
        }
    }

    fn add_ns_row(&mut self, row: &NsRow) {
        for _ in 0..row.atoms.len() {
            self.a_b_atoms += 1;
            self.ideal_importance += 1.0 / self.a_b_atoms as f64;
        }
    }

    /// Score with `partial` — the atoms of the batch a run at this
    /// budget ends on — folded in.
    fn score_with_partial(&self, partial: &[(usize, usize)]) -> BudgetScore {
        let mut with_partial = self.clone();
        with_partial.fold_walker_row(partial);
        with_partial.score()
    }

    fn score(&self) -> BudgetScore {
        if self.a_b_atoms == 0 {
            return BudgetScore::default();
        }
        let mut importance_sum = 0.0;
        let mut coverage_sum = 0.0;
        for (row, &delivered) in self.graded.ns_rows.iter().zip(&self.delivered) {
            if delivered == 0 {
                continue;
            }
            let completion = delivered as f64 / row.total_bytes as f64;
            for (offset, &(id, ns_bytes)) in row.atoms.iter().enumerate() {
                let rank = row.rank_start + offset;
                let credit = self.walker_bytes[id].min(ns_bytes) as f64 / ns_bytes as f64;
                let damped = credit * completion;
                importance_sum += damped / rank as f64;
                if rank <= self.a_b_atoms {
                    coverage_sum += damped;
                }
            }
        }
        let importance = (importance_sum / self.ideal_importance).min(1.0);
        let coverage = coverage_sum / self.a_b_atoms as f64;
        BudgetScore {
            importance,
            coverage,
            score: (importance * coverage).sqrt(),
        }
    }
}

/// Backslashes are doubled before pipes are escaped: in GFM only an
/// odd-length backslash run escapes a pipe.
fn escape_cell(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace(['\n', '\r'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::FsKey;
    use crate::render::Cost;

    #[test]
    fn divergence_escape_cell_keeps_pipes_inside_the_cell() {
        assert_eq!(escape_cell("a || b"), "a \\|\\| b");
        assert_eq!(escape_cell("regex \\| operator"), "regex \\\\\\| operator");
        assert_eq!(escape_cell("two\nlines"), "two lines");
    }

    /// Rows sharing a cum must print one Score, computed after all of them
    /// are folded in: here the walker row alone scores 0 and the NS row
    /// alone has nothing to score against.
    #[test]
    fn divergence_tied_cum_rows_share_one_score() {
        let ns = NorthStar {
            fixture: String::new(),
            revision_pin: String::new(),
            summary: String::new(),
            batches: vec![crate::north_star::NsBatch {
                id: "1.1".into(),
                descriptor: "ns at 100".into(),
                justification: String::new(),
                predecessor: None,
                content: BatchContent::Lines { spans: vec![] },
            }],
        };
        let schedule = Schedule {
            root: PathBuf::new(),
            batches: vec![ScheduledBatchRecord {
                key: FsKey::DirListing {
                    dir: PathBuf::from("walker at 100"),
                }
                .into(),
                cost: Cost {
                    tokens: 100,
                    chars: 0,
                },
                cum_tokens: 100,
                content: BatchContent::Lines { spans: vec![] },
            }],
        };
        let graded = Graded {
            ns: &ns,
            schedule: &schedule,
            ns_rows: vec![NsRow {
                atoms: vec![(0, 10)],
                total_bytes: 10,
                exp_t: 100,
                rank_start: 1,
            }],
            walker_rows: vec![vec![(0, 10)]],
            occurrences: vec![vec![(0, 10)]],
            partial_rows: vec![Vec::new(); BUDGETS.len()],
        };
        let (report, scores) = graded.report();
        let row_scores: Vec<&str> = report
            .lines()
            .filter(|l| l.starts_with("| walker |") || l.starts_with("| ns |"))
            .map(|l| l.rsplit('|').nth(1).unwrap().trim())
            .collect();
        assert_eq!(row_scores, ["1.000", "1.000"], "{report}");
        assert_eq!(scores.primary().score, 1.0);
    }
}
