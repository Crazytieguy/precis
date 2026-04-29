use super::*;
use crate::divergence::synthesis::priority_at_budget;

pub(super) fn format_report(scores: &Scores, ctx: &BuildCtx, arrivals: &[Arrival]) -> String {
    let mut out = String::new();
    let report_rows = report_rows(ctx, arrivals);
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
    format_verdict_block(&mut out, &report_rows);
    format_top_opportunities(&mut out, &report_rows);
    format_diagnosis_rollup(&mut out, &report_rows);
    format_loss_reason_rollup(&mut out, &report_rows);
    format_candidate_coverage_note(&mut out, &report_rows);
    format_candidate_exact_overlap_rollup(&mut out, &report_rows);
    format_arrival_ledger(&mut out, &report_rows);
    format_walker_waste(&mut out, ctx);

    out
}

fn format_score_vector(out: &mut String, scores: &Scores) {
    out.push_str("\n## Per-budget scores\n\n");
    out.push_str("| B | A_B | I(B) | C(B) | Score(B) | walker_used |\n");
    out.push_str("|--:|----:|-----:|-----:|---------:|------------:|\n");
    for s in scores.vector.iter() {
        out.push_str(&format!(
            "| {} | {} | {:.3} | {:.3} | {:.3} | {} |\n",
            s.budget, s.a_b_atoms, s.importance, s.coverage, s.score, s.walker_used,
        ));
    }
}

fn format_verdict_block(out: &mut String, rows: &[ReportRow<'_>]) {
    if rows.is_empty() {
        return;
    }
    let summary = report_summary(rows);

    out.push_str("\n## Verdict\n\n");
    out.push_str(&format!("Verdict: {}\n", summary.verdict));
    out.push_str(&format!(
        "Likely primary lever: {}\n",
        summary.primary_intervention
    ));
    out.push_str(&format!("Evidence: {}\n", summary.evidence));
    if let Some(secondary) = summary.secondary_intervention {
        out.push_str(&format!("Secondary intervention: {secondary}\n"));
    }
    out.push_str(&format!("Top rows: {}\n", summary.top_rows));
}

fn format_top_opportunities(out: &mut String, rows: &[ReportRow<'_>]) {
    let opportunities = top_opportunities(rows, 5);
    if opportunities.is_empty() {
        return;
    }

    out.push_str("\n## Top opportunities\n\n");
    out.push_str("_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._\n\n");
    out.push_str("| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |\n");
    out.push_str("|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|\n");
    for opp in opportunities {
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

fn format_candidate_coverage_note(out: &mut String, rows: &[ReportRow<'_>]) {
    let mut counts: BTreeMap<CandidateHintKind, usize> = BTreeMap::new();
    for row in rows {
        *counts.entry(row.hint.kind).or_default() += 1;
    }
    if counts.is_empty() {
        return;
    }

    let coverage = counts
        .into_iter()
        .map(|(kind, count)| format!("{}={count}", kind.label()))
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(
        "\n_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._\n",
    );
    out.push_str(&format!("Candidate hint kinds: {coverage}\n"));
}

fn format_diagnosis_rollup(out: &mut String, rows: &[ReportRow<'_>]) {
    struct Agg {
        rows: usize,
        missing: usize,
        partial: usize,
    }
    let mut counts: BTreeMap<DiagnosisKind, Agg> = BTreeMap::new();
    for row in rows {
        let agg = counts.entry(row.diagnosis).or_insert(Agg {
            rows: 0,
            missing: 0,
            partial: 0,
        });
        agg.rows += 1;
        match row.arrival.status {
            ArrivalStatus::Missing => agg.missing += 1,
            ArrivalStatus::Partial => agg.partial += 1,
            ArrivalStatus::Reached => debug_assert!(false, "reached rows are filtered out"),
        }
    }
    if counts.is_empty() {
        return;
    }

    out.push_str("\n## Diagnosis rollup\n\n");
    out.push_str("| diagnosis | rows | missing | partial | likely lever |\n");
    out.push_str("|:----------|-----:|--------:|--------:|:-------------|\n");
    for (diagnosis, agg) in counts {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            diagnosis.label(),
            agg.rows,
            agg.missing,
            agg.partial,
            diagnosis.likely_lever()
        ));
    }
}

fn format_loss_reason_rollup(out: &mut String, rows: &[ReportRow<'_>]) {
    let mut counts: BTreeMap<CandidateLossReason, usize> = BTreeMap::new();
    let mut weights: BTreeMap<CandidateLossReason, f64> = BTreeMap::new();
    for row in rows {
        if row.diagnosis != DiagnosisKind::RankingRecoverable {
            continue;
        }
        if let Some(loss) = ranking_loss(row) {
            *counts.entry(loss.reason).or_default() += 1;
            *weights.entry(loss.reason).or_default() +=
                priority_at_budget(row, PRIMARY_BUDGET_INDEX);
        }
    }
    if counts.is_empty() {
        return;
    }

    out.push_str("\n## Loss reason rollup (ranking-recoverable rows)\n\n");
    out.push_str("| loss reason | rows | gap@3k | likely lever |\n");
    out.push_str("|:------------|-----:|-------:|:-------------|\n");
    for (reason, count) in counts {
        out.push_str(&format!(
            "| {} | {} | {:.2} | {} |\n",
            reason.label(),
            count,
            weights.get(&reason).copied().unwrap_or(0.0),
            reason.intervention_label()
        ));
    }
}

fn format_candidate_exact_overlap_rollup(out: &mut String, rows: &[ReportRow<'_>]) {
    let mut counts: BTreeMap<(CandidateHintKind, &'static str, ExactOverlapBucket), usize> =
        BTreeMap::new();
    for row in rows {
        if let Some(overlap) = row.hint.exact_overlap {
            *counts
                .entry((row.hint.kind, row.arrival.status.label(), overlap.bucket()))
                .or_default() += 1;
        }
    }
    if counts.is_empty() {
        return;
    }

    out.push_str("\n## Exact atom overlap rollup (bbox hints)\n\n");
    out.push_str("| kind | status | exact_overlap | rows |\n");
    out.push_str("|:-----|:-------|:--------------|-----:|\n");
    for ((kind, status, bucket), count) in counts {
        out.push_str(&format!(
            "| {} | {status} | {} | {count} |\n",
            kind.label(),
            bucket.label()
        ));
    }
}

fn format_arrival_ledger(out: &mut String, rows: &[ReportRow<'_>]) {
    if rows.is_empty() {
        return;
    }
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
        out.push_str("| id | exp_t | credit | comp | status | descriptor | candidate hint |\n");
        out.push_str(
            "|----|------:|-------:|-----:|:-------|:-----------|:--------------------|\n",
        );
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
        let exp_t = group.rows.iter().map(|row| row.exp_t).min().unwrap_or(0);
        let avg_credit =
            group.rows.iter().map(|row| row.arrival.credit).sum::<f64>() / group.rows.len() as f64;
        let avg_completion = group
            .rows
            .iter()
            .map(|row| row.arrival.completion)
            .sum::<f64>()
            / group.rows.len() as f64;
        out.push_str(&format!(
            "| group | {exp_t} | {avg_credit:.2} | {avg_completion:.2} | predecessor-gated | {} children of `{}` | exact total={}/{}; rows: {ids} |\n",
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
        "| {} | {} | {:.2} | {:.2} | {} | {} | {} |\n",
        r.id,
        r.exp_t,
        r.arrival.credit,
        r.arrival.completion,
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

fn format_walker_waste(out: &mut String, ctx: &BuildCtx) {
    let ns_atom_set: BTreeSet<Atom> = ctx
        .ns_rows
        .iter()
        .flat_map(|r| r.atoms.iter().map(|a| a.atom.clone()))
        .collect();

    // For each walker batch: how much of its marginal token spend landed
    // on atoms NS didn't ask for. Per-atom marginal costs were captured
    // alongside `atoms` (1:1) when the parallel walker tree was driven
    // forward, so a refinement-over-ancestor on-NS line costs only its
    // delta and an already-listed FS entry costs zero — the off_ratio
    // reflects true marginal share, not uniform-across-atoms.
    // Sort by off_tokens descending so the biggest calibration targets
    // are at row 1.
    struct Row<'a> {
        wr: &'a WalkerRow<'a>,
        descriptor_rel: String,
        off_ratio: f64,
        off_tokens: usize,
    }
    let mut rows: Vec<Row<'_>> = ctx
        .walker_rows
        .iter()
        .filter(|wr| wr.batch.cost_tokens >= UNMAPPED_COST_THRESHOLD)
        .filter_map(|wr| {
            let (off_ratio, off_tokens) =
                off_ns_attribution(&wr.atoms, &wr.atom_token_costs, &ns_atom_set)?;
            if off_tokens < UNMAPPED_COST_THRESHOLD {
                return None;
            }
            let descriptor_rel = strip_fixture_root(&wr.batch.descriptor, &ctx.fixture_root);
            Some(Row {
                wr,
                descriptor_rel,
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
            .then_with(|| a.descriptor_rel.cmp(&b.descriptor_rel))
    });

    format_walker_waste_rollup(
        out,
        rows.iter()
            .map(|r| (r.descriptor_rel.as_str(), r.off_tokens)),
    );

    out.push_str(&format!(
        "\n## Walker waste (off-NS token spend ≥ {UNMAPPED_COST_THRESHOLD})\n\n"
    ));
    out.push_str("| off_tokens | off_ratio | cost | first_t | batch |\n");
    out.push_str("|-----------:|----------:|-----:|--------:|:------|\n");
    for r in rows.iter().take(WASTE_DETAIL_LIMIT) {
        out.push_str(&format!(
            "| {} | {:.2} | {} | {} | {} |\n",
            r.off_tokens, r.off_ratio, r.wr.batch.cost_tokens, r.wr.seen_t, r.descriptor_rel,
        ));
    }
    if rows.len() > WASTE_DETAIL_LIMIT {
        let tail = &rows[WASTE_DETAIL_LIMIT..];
        let tail_tokens: usize = tail.iter().map(|r| r.off_tokens).sum();
        out.push_str(&format!(
            "| {tail_tokens} | — | — | — | +{} more rows |\n",
            tail.len()
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
        off_tokens_total: usize,
    }
    let mut by_pattern: BTreeMap<String, Group> = BTreeMap::new();
    for (descriptor_rel, off_tokens) in rows {
        let pattern = pattern_template(descriptor_rel);
        let g = by_pattern.entry(pattern).or_insert(Group {
            n: 0,
            off_tokens_total: 0,
        });
        g.n += 1;
        g.off_tokens_total += off_tokens;
    }
    let mut interesting: Vec<(&String, &Group)> =
        by_pattern.iter().filter(|(_, g)| g.n >= 2).collect();
    if interesting.is_empty() {
        return;
    }
    interesting.sort_by(|a, b| {
        b.1.off_tokens_total
            .cmp(&a.1.off_tokens_total)
            .then_with(|| a.0.cmp(b.0))
    });
    out.push_str("\n## Walker waste rollup (by descriptor pattern)\n\n");
    out.push_str("| n | off_tokens_total | pattern |\n");
    out.push_str("|--:|-----------------:|:--------|\n");
    for (pattern, g) in interesting {
        out.push_str(&format!(
            "| {} | {} | {pattern} |\n",
            g.n, g.off_tokens_total
        ));
    }
}
