use super::*;
use crate::divergence::synthesis::plural;

pub(super) fn format_report(scores: &Scores, ctx: &BuildCtx, arrivals: &[Arrival]) -> String {
    let mut out = String::new();
    let report_rows = report_rows(ctx, arrivals);
    let waste_rows = compute_walker_waste_rows(ctx);

    out.push_str(&format!(
        "scores: Score(3000)={:.3} ns_rows≤3K={}/{} (reached={} partial={} missing={})\n",
        scores.primary(),
        scores.rows_in_primary,
        scores.total_ns,
        scores.reached,
        scores.partial,
        scores.missing,
    ));

    format_score_vector(&mut out, scores);
    format_top_opportunities(&mut out, &report_rows, &waste_rows);
    format_arrival_ledger(&mut out, &report_rows, ctx);
    format_walker_waste(&mut out, &waste_rows, ctx);

    out
}

fn format_score_vector(out: &mut String, scores: &Scores) {
    out.push_str("\n## Per-budget scores\n\n");
    out.push_str("| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |\n");
    out.push_str("|--:|----:|-----:|-----:|---------:|---------:|------------:|\n");
    for s in scores.vector.iter() {
        let compl = if s.completion.is_nan() {
            "—".to_string()
        } else {
            format!("{:.3}", s.completion)
        };
        out.push_str(&format!(
            "| {} | {} | {:.3} | {:.3} | {} | {:.3} | {} |\n",
            s.budget, s.a_b_atoms, s.importance, s.coverage, compl, s.score, s.walker_used,
        ));
    }
}

fn format_top_opportunities(
    out: &mut String,
    rows: &[ReportRow<'_>],
    waste_rows: &[WalkerWasteRow<'_>],
) {
    let additive = top_opportunities(rows, 5);
    // Pass ALL waste rows; per-column `walker_t <= B` gating happens
    // inside `top_suppression_opportunities`.
    let subtractive = top_suppression_opportunities(waste_rows, 5);
    if additive.is_empty() && subtractive.is_empty() {
        return;
    }

    out.push_str("\n## Top opportunities\n\n");

    let additive_emitted = !additive.is_empty();
    if additive_emitted {
        out.push_str("### Additive (close partial / missing rows)\n\n");
        out.push_str(
            "| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |\n",
        );
        out.push_str(
            "|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|\n",
        );
        for opp in additive {
            out.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} | {:.2} | {} | {} |\n",
                opp.intervention,
                opp.rows,
                opp.gap_at_low,
                opp.gap_at_primary,
                opp.gap_at_high,
                opp.evidence,
                opp.top_row_ids
            ));
        }
    }

    if !subtractive.is_empty() {
        if additive_emitted {
            out.push('\n');
        }
        out.push_str("### Subtractive (suppress consistently off-NS batches)\n\n");
        out.push_str(
            "| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |\n",
        );
        out.push_str(
            "|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|\n",
        );
        for opp in subtractive {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                opp.pattern,
                opp.batches,
                opp.freed_at_low,
                opp.freed_at_primary,
                opp.freed_at_high,
                opp.evidence,
                opp.top_batch_ids
            ));
        }
    }
}

pub(super) struct SuppressionOpportunity {
    pub(super) pattern: String,
    pub(super) batches: usize,
    pub(super) freed_at_low: usize,
    pub(super) freed_at_primary: usize,
    pub(super) freed_at_high: usize,
    pub(super) evidence: String,
    pub(super) top_batch_ids: String,
}

/// Group waste rows by `pattern_template(descriptor)` — the same key
/// the existing waste rollup uses. Each pattern becomes one
/// suppression-intervention candidate.
///
/// Each `freed@B` column sums `off_any_tokens` over rows with
/// `walker_t ≤ B`. We use `off_any` (atoms truly never in NS at any
/// budget), not `off_3k`, because atoms NS wants past `B` still
/// contribute to `Score(B)` via the no-rank-cap Importance numerator
/// in `compute_score_at` — suppressing them would lose that
/// contribution. `off_any` is the strictly-safe lever. The `evidence`
/// column shows `off_3k` for context: a pattern with `off_3k`
/// noticeably above `freed@3k` carries "premature" NS-after-3K
/// content that the iterator may want to keep.
///
/// Patterns are dropped if `freed@3k == 0` — no truly off-NS spend
/// within the 3K prefix.
///
/// Sorted by `freed@3k` descending, capped at `limit`. `batches` and
/// `top_batch_ids` describe the rows that contributed to `freed@3k`.
pub(super) fn top_suppression_opportunities(
    waste_rows: &[WalkerWasteRow<'_>],
    limit: usize,
) -> Vec<SuppressionOpportunity> {
    let low_budget = BUDGETS[LOW_BUDGET_INDEX];
    let high_budget = BUDGETS[HIGH_BUDGET_INDEX];
    struct Group<'a> {
        primary_rows: Vec<&'a WalkerWasteRow<'a>>,
        freed_at_low: usize,
        freed_at_primary: usize,
        freed_at_high: usize,
        off_3k_primary: usize,
    }
    let mut groups: BTreeMap<String, Group<'_>> = BTreeMap::new();
    for r in waste_rows {
        if r.walker_t > high_budget {
            continue;
        }
        let pattern = pattern_template(&r.descriptor_rel);
        let g = groups.entry(pattern).or_insert(Group {
            primary_rows: Vec::new(),
            freed_at_low: 0,
            freed_at_primary: 0,
            freed_at_high: 0,
            off_3k_primary: 0,
        });
        if r.walker_t <= low_budget {
            g.freed_at_low += r.off_any_tokens;
        }
        if r.walker_t <= PRIMARY_BUDGET {
            g.freed_at_primary += r.off_any_tokens;
            g.off_3k_primary += r.off_3k_tokens;
            g.primary_rows.push(r);
        }
        g.freed_at_high += r.off_any_tokens;
    }
    let mut out: Vec<SuppressionOpportunity> = groups
        .into_iter()
        .filter(|(_, g)| g.freed_at_primary > 0)
        .map(|(pattern, g)| {
            // g.primary_rows preserves insertion order from
            // compute_walker_waste_rows — descending off_3k_tokens.
            let top_ids = g
                .primary_rows
                .iter()
                .take(3)
                .map(|r| r.descriptor_rel.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let top_batch_ids = if g.primary_rows.len() > 3 {
                format!("{top_ids}, ...")
            } else {
                top_ids
            };
            SuppressionOpportunity {
                pattern,
                batches: g.primary_rows.len(),
                freed_at_low: g.freed_at_low,
                freed_at_primary: g.freed_at_primary,
                freed_at_high: g.freed_at_high,
                evidence: format!("off_3k={}", g.off_3k_primary),
                top_batch_ids,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.freed_at_primary
            .cmp(&a.freed_at_primary)
            .then_with(|| b.freed_at_high.cmp(&a.freed_at_high))
            .then_with(|| b.batches.cmp(&a.batches))
            .then_with(|| a.pattern.cmp(&b.pattern))
    });
    out.truncate(limit);
    out
}

fn format_arrival_ledger(out: &mut String, rows: &[ReportRow<'_>], ctx: &BuildCtx) {
    if rows.is_empty() {
        return;
    }
    format_top_missed_paths(out, rows, &ctx.fixture_root);
    out.push_str("\n## Arrival ledger by diagnosis\n");

    let order = [
        DiagnosisKind::RankingRecoverable,
        DiagnosisKind::WrongSlice,
        DiagnosisKind::NoDiscoveredCandidate,
        DiagnosisKind::FsListing,
        DiagnosisKind::MixedUnknown,
    ];
    for diagnosis in order {
        let mut group: Vec<&ReportRow<'_>> =
            rows.iter().filter(|r| r.diagnosis == diagnosis).collect();
        if group.is_empty() {
            continue;
        }
        group.sort_by_key(|r| r.exp_t);
        out.push_str(&format!("\n### {}\n\n", diagnosis.label()));
        out.push_str("| id | ns_t | credit | status | descriptor | candidate hint |\n");
        out.push_str("|----|-----:|-------:|:-------|:-----------|:---------------|\n");
        let grouped_ids = if diagnosis == DiagnosisKind::RankingRecoverable {
            format_parent_gated_groups(out, &group)
        } else {
            BTreeSet::new()
        };
        for r in group {
            if grouped_ids.contains(r.id) {
                continue;
            }
            format_arrival_row(out, r);
        }
    }
}

fn format_parent_gated_groups(out: &mut String, rows: &[&ReportRow<'_>]) -> BTreeSet<String> {
    let groups = predecessor_groups_from_refs(rows);
    let mut grouped_ids = BTreeSet::new();
    for group in groups {
        if group.rows.len() < 2 {
            continue;
        }
        let ids = row_ids(&group.rows, 12);
        for row in &group.rows {
            grouped_ids.insert(row.id.to_string());
        }
        let ns_t = group.rows.iter().map(|row| row.exp_t).min().unwrap_or(0);
        let avg_credit =
            group.rows.iter().map(|row| row.arrival.credit).sum::<f64>() / group.rows.len() as f64;
        out.push_str(&format!(
            "| group | {ns_t} | {avg_credit:.2} | predecessor-gated | {} children of `{}` | exact total={}/{}; rows: {ids} |\n",
            group.rows.len(),
            group.predecessor,
            group.hits,
            group.total,
        ));
    }
    grouped_ids
}

fn format_arrival_row(out: &mut String, r: &ReportRow<'_>) {
    out.push_str(&format!(
        "| {} | {} | {:.2} | {} | {} | {} |\n",
        r.id,
        r.exp_t,
        r.arrival.credit,
        r.arrival.status.label(),
        r.descriptor,
        r.hint.cell,
    ));
}

/// Per-batch off-NS attribution. Returns `(off_ratio, off_tokens)` where
/// both are weighted by per-atom marginal token cost (not atom count).
/// `None` for empty batches and for pure-refinement no-ops where every
/// atom contributed zero marginal cost (an ancestor already rendered
/// the same content).
pub(super) fn off_ns_attribution(
    atoms: &[GradedAtom],
    atom_token_costs: &[usize],
    ns_atom_set: &BTreeSet<Atom>,
) -> Option<(f64, usize)> {
    if atoms.is_empty() {
        return None;
    }
    debug_assert_eq!(atoms.len(), atom_token_costs.len());
    let total_marginal: usize = atom_token_costs.iter().sum();
    if total_marginal == 0 {
        return None;
    }
    let off_marginal: usize = atom_token_costs
        .iter()
        .zip(atoms)
        .filter(|(_, a)| !ns_atom_set.contains(&a.atom))
        .map(|(c, _)| *c)
        .sum();
    Some((off_marginal as f64 / total_marginal as f64, off_marginal))
}

pub(super) struct WalkerWasteRow<'a> {
    pub(super) wr: &'a WalkerRow<'a>,
    pub(super) descriptor_rel: String,
    /// Atoms not in any NS batch (any budget). Truly off-NS content —
    /// suppressing it strictly reduces noise without losing any
    /// `Score(B)` contribution at any budget. The subtractive
    /// subtable's `freed@B` is built from this.
    pub(super) off_any_tokens: usize,
    /// Atoms not in `A_3K`'s NS atom set: either absent from NS
    /// entirely (= `off_any`) or in NS but at `exp_t > 3000`. Includes
    /// "premature" content that, although outside `A_3K`, still
    /// contributes to `Score(3000)` via the no-rank-cap Importance
    /// numerator. Primary admission/sort key for the detail table.
    pub(super) off_3k_ratio: f64,
    pub(super) off_3k_tokens: usize,
    /// Walker `cum_tokens` *before* this batch was applied —
    /// `seen_t - cost_tokens`. The gate value for "primary-actionable":
    /// `walker_t ≤ 3000` includes boundary-crossers (where
    /// `walker_t ≤ 3000 < walker_t + cost`) — these are the batches
    /// that *stopped* the 3K prefix under the no-fallback scheduler,
    /// so demoting one can let the next-best candidate fit and lift
    /// `Score(3000)`.
    pub(super) walker_t: usize,
}

/// Build NS atom set for atoms in batches with `exp_t ≤ budget`.
/// `usize::MAX` → all-NS set (atoms in any batch, any budget).
fn ns_atom_set_at(ctx: &BuildCtx, budget: usize) -> BTreeSet<Atom> {
    ctx.ns_rows
        .iter()
        .filter(|r| budget == usize::MAX || r.exp_t <= budget)
        .flat_map(|r| r.atoms.iter().map(|a| a.atom.clone()))
        .collect()
}

/// Compute walker-waste rows once, populated with off-NS attribution
/// against both the all-NS set (`off_any_tokens` — truly off-NS) and
/// `A_3K` (`off_3k_tokens` — admission/sort key). Shared by the
/// Top-opportunities subtractive subtable and the walker-waste section
/// at the bottom.
///
/// Primary admission gate is `off_3k_tokens ≥ UNMAPPED_COST_THRESHOLD`
/// — every atom outside `A_3K` is at most a weak contributor to
/// `Score(3000)`. Sorted by `off_3k_tokens` descending.
pub(super) fn compute_walker_waste_rows<'a>(ctx: &'a BuildCtx<'a>) -> Vec<WalkerWasteRow<'a>> {
    let ns_atom_set_all = ns_atom_set_at(ctx, usize::MAX);
    let ns_atom_set_3k = ns_atom_set_at(ctx, PRIMARY_BUDGET);

    // For each walker batch: how much of its marginal token spend landed
    // on atoms NS didn't ask for. Per-atom marginal costs were captured
    // alongside `atoms` (1:1) when the parallel walker tree was driven
    // forward, so a refinement-over-ancestor on-NS line costs only its
    // delta and an already-listed FS entry costs zero — the off_*
    // values reflect true marginal share, not uniform-across-atoms.
    let mut rows: Vec<WalkerWasteRow<'_>> = ctx
        .walker_rows
        .iter()
        .filter(|wr| wr.batch.cost_tokens >= UNMAPPED_COST_THRESHOLD)
        .filter_map(|wr| {
            let (_, off_any_tokens) =
                off_ns_attribution(&wr.atoms, &wr.atom_token_costs, &ns_atom_set_all)?;
            let (off_3k_ratio, off_3k_tokens) =
                off_ns_attribution(&wr.atoms, &wr.atom_token_costs, &ns_atom_set_3k)?;
            if off_3k_tokens < UNMAPPED_COST_THRESHOLD {
                return None;
            }
            let descriptor_rel = strip_fixture_root(&wr.batch.descriptor, &ctx.fixture_root);
            let walker_t = wr.seen_t.saturating_sub(wr.batch.cost_tokens);
            Some(WalkerWasteRow {
                wr,
                descriptor_rel,
                off_any_tokens,
                off_3k_ratio,
                off_3k_tokens,
                walker_t,
            })
        })
        .collect();
    rows.sort_by(|a, b| {
        b.off_3k_tokens
            .cmp(&a.off_3k_tokens)
            .then_with(|| a.descriptor_rel.cmp(&b.descriptor_rel))
    });
    rows
}

fn format_walker_waste(out: &mut String, rows: &[WalkerWasteRow<'_>], ctx: &BuildCtx) {
    // Use `walker_t` (cumulative before the batch was applied) so a
    // boundary-crossing batch — the one that stopped the 3K prefix
    // under the scheduler's no-fallback rule — counts as primary-
    // actionable. Demoting that batch can let the next-best candidate
    // fit and lift `Score(3000)`.
    let primary: Vec<&WalkerWasteRow<'_>> = rows
        .iter()
        .filter(|r| r.walker_t <= PRIMARY_BUDGET)
        .collect();
    if primary.is_empty() {
        return;
    }

    format_top_wasted_paths(out, &primary, &ctx.fixture_root);
    format_walker_waste_rollup(
        out,
        primary
            .iter()
            .map(|r| (r.descriptor_rel.as_str(), r.off_3k_tokens)),
    );
    format_walker_waste_table(out, &primary);
}

fn format_walker_waste_table(out: &mut String, rows: &[&WalkerWasteRow<'_>]) {
    out.push_str(&format!(
        "\n## Walker waste (walker_t ≤ {PRIMARY_BUDGET}, off_3k ≥ {UNMAPPED_COST_THRESHOLD})\n\n"
    ));
    out.push_str("| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |\n");
    out.push_str("|-------:|-------------:|--------:|-----:|---------:|:------|\n");
    for r in rows.iter().take(WASTE_DETAIL_LIMIT) {
        out.push_str(&format!(
            "| {} | {:.2} | {} | {} | {} | {} |\n",
            r.off_3k_tokens,
            r.off_3k_ratio,
            r.off_any_tokens,
            r.wr.batch.cost_tokens,
            r.walker_t,
            r.descriptor_rel,
        ));
    }
    if rows.len() > WASTE_DETAIL_LIMIT {
        let tail = &rows[WASTE_DETAIL_LIMIT..];
        let tail_3k_tokens: usize = tail.iter().map(|r| r.off_3k_tokens).sum();
        out.push_str(&format!(
            "| {tail_3k_tokens} | — | — | — | — | +{} more row{} |\n",
            tail.len(),
            plural(tail.len()),
        ));
    }
}

/// Rollup of waste rows by descriptor pattern: same descriptor with
/// position-bearing suffix (`:<line>` for Rust/TS bodies, ` #<index>` for
/// markdown sections) collapsed to `<n>`. Surfaces systemic walker
/// over-spend — e.g. "14 `pub-item doc at src/lib.rs:<n>` rows totaling
/// 3.6k tokens" — that the per-row table buries. Elide patterns with
/// `n == 1` (no rollup benefit) and the whole section if no pattern has
/// `n ≥ 2`.
fn format_walker_waste_rollup<'a>(
    out: &mut String,
    rows: impl IntoIterator<Item = (&'a str, usize)>,
) {
    struct Group {
        n: usize,
        off_3k_tokens_total: usize,
    }
    let mut by_pattern: BTreeMap<String, Group> = BTreeMap::new();
    for (descriptor_rel, off_3k_tokens) in rows {
        let pattern = pattern_template(descriptor_rel);
        let g = by_pattern.entry(pattern).or_insert(Group {
            n: 0,
            off_3k_tokens_total: 0,
        });
        g.n += 1;
        g.off_3k_tokens_total += off_3k_tokens;
    }
    let mut interesting: Vec<(&String, &Group)> =
        by_pattern.iter().filter(|(_, g)| g.n >= 2).collect();
    if interesting.is_empty() {
        return;
    }
    interesting.sort_by(|a, b| {
        b.1.off_3k_tokens_total
            .cmp(&a.1.off_3k_tokens_total)
            .then_with(|| a.0.cmp(b.0))
    });
    out.push_str("\n## Walker waste rollup (by descriptor pattern)\n\n");
    out.push_str("| n | off_3k_total | pattern |\n");
    out.push_str("|--:|-------------:|:--------|\n");
    for (pattern, g) in interesting {
        out.push_str(&format!(
            "| {} | {} | {pattern} |\n",
            g.n, g.off_3k_tokens_total
        ));
    }
}

/// Dominant source path for an atom — the `path` for `Line` atoms,
/// the `parent` directory for `Fs` atoms (FS-listing batches focus on
/// directories). Stripped relative to fixture root for diff-stable
/// reports.
fn atom_dominant_path(atom: &Atom, fixture_root: &Path) -> PathBuf {
    let abs = match atom {
        Atom::Line { path, .. } => path,
        Atom::Fs { parent, .. } => parent,
    };
    abs.strip_prefix(fixture_root)
        .map(PathBuf::from)
        .unwrap_or_else(|_| abs.clone())
}

/// Per-NS-row dominant file: path with the most atoms in the row.
/// Most NS rows touch a single file; this picks that file. Ties break
/// by lexicographic path order for stability.
fn dominant_path_for_atoms(atoms: &[GradedAtom], fixture_root: &Path) -> Option<PathBuf> {
    let mut counts: BTreeMap<PathBuf, usize> = BTreeMap::new();
    for atom in atoms {
        let p = atom_dominant_path(&atom.atom, fixture_root);
        *counts.entry(p).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by(|(p_a, c_a), (p_b, c_b)| c_a.cmp(c_b).then_with(|| p_b.cmp(p_a)))
        .map(|(p, _)| p)
}

const PATH_ROLLUP_LIMIT: usize = 8;

/// Render a one-line `<header>: path1 (...), path2 (...), +N more`
/// summary from per-path `(primary, secondary)` aggregates. Sort is
/// `(primary desc, secondary desc, path asc)`. Caller supplies the
/// per-row formatter so labels can vary across call sites.
fn format_path_rollup<F>(
    out: &mut String,
    header: &str,
    by_path: BTreeMap<PathBuf, (usize, usize)>,
    format_row: F,
) where
    F: Fn(&Path, usize, usize) -> String,
{
    if by_path.is_empty() {
        return;
    }
    let mut items: Vec<(PathBuf, usize, usize)> = by_path
        .into_iter()
        .map(|(p, (primary, secondary))| (p, primary, secondary))
        .collect();
    items.sort_by(|(p_a, pri_a, sec_a), (p_b, pri_b, sec_b)| {
        pri_b
            .cmp(pri_a)
            .then_with(|| sec_b.cmp(sec_a))
            .then_with(|| p_a.cmp(p_b))
    });
    let summary: Vec<String> = items
        .iter()
        .take(PATH_ROLLUP_LIMIT)
        .map(|(p, primary, secondary)| format_row(p, *primary, *secondary))
        .collect();
    let suffix = if items.len() > PATH_ROLLUP_LIMIT {
        format!(", +{} more", items.len() - PATH_ROLLUP_LIMIT)
    } else {
        String::new()
    };
    out.push_str(&format!("\n{header}: {}{}\n", summary.join(", "), suffix));
}

/// Top missed paths: A_3K NS rows the walker didn't deliver, grouped
/// by dominant source path. Pairs with Top wasted paths to make
/// path-mismatch (walker spending budget on file X while NS wants file
/// Y) visible at a glance.
fn format_top_missed_paths(out: &mut String, rows: &[ReportRow<'_>], fixture_root: &Path) {
    let mut by_path: BTreeMap<PathBuf, (usize, usize)> = BTreeMap::new();
    for row in rows {
        if row.exp_t > PRIMARY_BUDGET {
            continue;
        }
        let Some(path) = dominant_path_for_atoms(row.atoms, fixture_root) else {
            continue;
        };
        let agg = by_path.entry(path).or_insert((0, 0));
        agg.0 += row.atoms.len(); // primary: atoms (score-relevant magnitude)
        agg.1 += 1; // secondary: rows
    }
    format_path_rollup(
        out,
        "Top missed paths (NS rows ≤ 3K)",
        by_path,
        |p, atoms, rows| {
            format!(
                "{} ({rows} row{}, {atoms} atom{})",
                p.display(),
                plural(rows),
                plural(atoms),
            )
        },
    );
}

/// Top wasted paths: primary-actionable waste batches grouped by
/// dominant source path. Companion to Top missed paths.
fn format_top_wasted_paths(out: &mut String, rows: &[&WalkerWasteRow<'_>], fixture_root: &Path) {
    let mut by_path: BTreeMap<PathBuf, (usize, usize)> = BTreeMap::new();
    for r in rows {
        let Some(path) = dominant_path_for_atoms(&r.wr.atoms, fixture_root) else {
            continue;
        };
        let agg = by_path.entry(path).or_insert((0, 0));
        agg.0 += r.off_3k_tokens; // primary: off_3k tokens
        agg.1 += 1; // secondary: batches
    }
    format_path_rollup(
        out,
        "Top wasted paths (off-NS at 3K)",
        by_path,
        |p, off_3k, batches| {
            format!(
                "{} ({off_3k}t, {batches} batch{})",
                p.display(),
                if batches == 1 { "" } else { "es" },
            )
        },
    );
}
