use super::*;

pub(super) struct ReportRow<'a> {
    pub(super) id: &'a str,
    pub(super) exp_t: usize,
    pub(super) descriptor: &'a str,
    pub(super) arrival: &'a Arrival,
    pub(super) hint: CandidateHint,
    pub(super) diagnosis: DiagnosisKind,
}

pub(super) fn report_rows<'a>(ctx: &'a BuildCtx, arrivals: &'a [Arrival]) -> Vec<ReportRow<'a>> {
    ctx.ns_rows
        .iter()
        .enumerate()
        .zip(arrivals)
        .filter_map(|((i, ns_row), arrival)| {
            // Reached rows (primary-budget damped credit ≥
            // REACH_THRESHOLD) are reached at the optimization
            // target — no actionable gap on `Score(3000)`. Drop them
            // so the ledger and rollups stay partial-or-missing only.
            if arrival.status == ArrivalStatus::Reached {
                return None;
            }
            let hint = candidate_hint_for_ctx(ns_row, ctx);
            let diagnosis = diagnose_row(&hint);
            Some(ReportRow {
                id: &ns_row.id,
                exp_t: ns_row.exp_t,
                descriptor: &ctx.ns.batches[i].descriptor,
                arrival,
                hint,
                diagnosis,
            })
        })
        .collect()
}

pub(super) struct ReportSummary {
    pub(super) verdict: String,
    pub(super) primary_intervention: String,
    pub(super) secondary_intervention: Option<String>,
    pub(super) evidence: String,
    pub(super) top_rows: String,
}

pub(super) struct Opportunity {
    pub(super) intervention: String,
    pub(super) rows: usize,
    /// Σ priority over the opportunity's rows at B=[`BUDGETS[LOW_BUDGET_INDEX]`].
    pub(super) gap_at_low: f64,
    /// Σ priority at B=[`PRIMARY_BUDGET`]. Direct proxy for
    /// `Score(3000)` headroom and the primary sort key for
    /// opportunities, so iterators land on interventions that move the
    /// optimization target rather than future-budget rows.
    pub(super) gap_at_primary: f64,
    /// Σ priority at B=[`BUDGETS[HIGH_BUDGET_INDEX]`].
    pub(super) gap_at_high: f64,
    pub(super) evidence: String,
    pub(super) top_row_ids: String,
}

pub(super) fn report_summary(rows: &[ReportRow<'_>]) -> ReportSummary {
    let diagnosis = diagnosis_counts(rows);
    let losses = loss_reason_counts(rows);
    let diagnosis_weights = diagnosis_weights(rows);
    let loss_weights = loss_reason_weights(rows);
    let pred_groups = predecessor_groups(rows);
    let no_discovered = diagnosis
        .get(&DiagnosisKind::NoDiscoveredCandidate)
        .copied()
        .unwrap_or(0);
    let wrong_slice = diagnosis
        .get(&DiagnosisKind::WrongSlice)
        .copied()
        .unwrap_or(0);
    let ranking = diagnosis
        .get(&DiagnosisKind::RankingRecoverable)
        .copied()
        .unwrap_or(0);
    let pred_count = losses
        .get(&CandidateLossReason::PredecessorNotScheduled)
        .copied()
        .unwrap_or(0);
    let pred_weight = loss_weights
        .get(&CandidateLossReason::PredecessorNotScheduled)
        .copied()
        .unwrap_or(0.0);
    // `DiscoveredUnscheduled` + `TooExpensiveAtFinalMargin` collapse
    // into one ranking-race bucket. Rationale on the module-level
    // doc; the short version is that `TooExpensive` is a post-hoc
    // label conflating two cases that share the same `tune ranking`
    // lever.
    let race_weight = loss_weights
        .get(&CandidateLossReason::DiscoveredUnscheduled)
        .copied()
        .unwrap_or(0.0)
        + loss_weights
            .get(&CandidateLossReason::TooExpensiveAtFinalMargin)
            .copied()
            .unwrap_or(0.0);
    let ranking_weight = diagnosis_weights
        .get(&DiagnosisKind::RankingRecoverable)
        .copied()
        .unwrap_or(0.0);
    let wrong_slice_weight = diagnosis_weights
        .get(&DiagnosisKind::WrongSlice)
        .copied()
        .unwrap_or(0.0);
    let no_discovered_weight = diagnosis_weights
        .get(&DiagnosisKind::NoDiscoveredCandidate)
        .copied()
        .unwrap_or(0.0);

    let primary_diagnosis = [
        (DiagnosisKind::WrongSlice, wrong_slice_weight),
        (DiagnosisKind::RankingRecoverable, ranking_weight),
        (DiagnosisKind::NoDiscoveredCandidate, no_discovered_weight),
    ]
    .into_iter()
    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    .filter(|(_, weight)| *weight > 0.0)
    .map(|(diagnosis, _)| diagnosis);

    let primary_loss = if primary_diagnosis == Some(DiagnosisKind::RankingRecoverable) {
        if pred_weight > 0.0 && pred_weight >= race_weight {
            Some(CandidateLossReason::PredecessorNotScheduled)
        } else if race_weight > 0.0 {
            Some(CandidateLossReason::DiscoveredUnscheduled)
        } else {
            None
        }
    } else {
        None
    };

    let (verdict, primary_intervention) = if primary_diagnosis == Some(DiagnosisKind::WrongSlice) {
        (
            "wrong-slice bound".to_string(),
            "split walker batches to match NS semantic slices".to_string(),
        )
    } else if primary_diagnosis == Some(DiagnosisKind::NoDiscoveredCandidate) {
        (
            "coverage-gap bound".to_string(),
            "add walker candidates for no-discovered NS rows".to_string(),
        )
    } else if primary_loss == Some(CandidateLossReason::PredecessorNotScheduled) {
        if let Some(top) = predecessor_kind_groups(rows).first() {
            let file_text = if top.files.is_empty() {
                String::new()
            } else {
                format!(
                    " across {} file{}",
                    top.files.len(),
                    plural(top.files.len())
                )
            };
            (
                "parent-gating bound".to_string(),
                format!(
                    "promote {} for {} gated row{}{}",
                    top.kind,
                    top.rows.len(),
                    plural(top.rows.len()),
                    file_text
                ),
            )
        } else if let Some(top) = pred_groups.first().filter(|g| g.rows.len() >= 2) {
            (
                "parent-gating bound".to_string(),
                format!("promote `{}`", top.predecessor),
            )
        } else {
            (
                "parent-gating bound".to_string(),
                format!("promote predecessor candidates for {pred_count} gated rows"),
            )
        }
    } else if primary_loss == Some(CandidateLossReason::DiscoveredUnscheduled) {
        (
            "ranking-race bound".to_string(),
            "raise high-overlap discovered candidates over competing batches".to_string(),
        )
    } else {
        (
            "timing-only / low-action".to_string(),
            "inspect timing rows only if score movement matters".to_string(),
        )
    };

    // Secondary intervention only fires when it's distinct from the
    // primary diagnosis — e.g. a wrong-slice fixture with a few
    // gated predecessors surfaces both. If the candidate
    // intervention overlaps with the verdict's primary lever, drop
    // it; restating the same recommendation is noise.
    let secondary_intervention = if primary_loss
        != Some(CandidateLossReason::PredecessorNotScheduled)
        && pred_count > 0
    {
        Some(format!(
            "promote predecessors for {pred_count} gated candidate{}",
            plural(pred_count)
        ))
    } else if primary_diagnosis != Some(DiagnosisKind::WrongSlice)
        && wrong_slice > ranking
        && wrong_slice > 0
    {
        Some(format!(
            "split wrong-slice batches for {wrong_slice} row{}",
            plural(wrong_slice)
        ))
    } else if primary_diagnosis != Some(DiagnosisKind::NoDiscoveredCandidate) && no_discovered > 0 {
        Some(format!(
            "investigate {no_discovered} no-discovered row{}",
            plural(no_discovered)
        ))
    } else {
        None
    };

    let evidence = format!(
        "{ranking} ranking-recoverable (gap@3k={ranking_weight:.2}), {wrong_slice} wrong-slice/granularity (gap@3k={wrong_slice_weight:.2}), {no_discovered} no-discovered (gap@3k={no_discovered_weight:.2})"
    );
    let top_rows = top_opportunities(rows, 1)
        .first()
        .map(|opp| opp.top_row_ids.clone())
        .unwrap_or_else(|| {
            rows.iter()
                .take(5)
                .map(|r| r.id)
                .collect::<Vec<_>>()
                .join(", ")
        });

    ReportSummary {
        verdict,
        primary_intervention,
        secondary_intervention,
        evidence,
        top_rows: if top_rows.is_empty() {
            "none".to_string()
        } else {
            top_rows
        },
    }
}

fn diagnosis_counts(rows: &[ReportRow<'_>]) -> BTreeMap<DiagnosisKind, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.diagnosis).or_default() += 1;
    }
    counts
}

pub(super) fn loss_reason_counts(rows: &[ReportRow<'_>]) -> BTreeMap<CandidateLossReason, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        if row.diagnosis != DiagnosisKind::RankingRecoverable {
            continue;
        }
        if let Some(loss) = ranking_loss(row) {
            *counts.entry(loss.reason).or_default() += 1;
        }
    }
    counts
}

pub(super) fn loss_reason_weights(rows: &[ReportRow<'_>]) -> BTreeMap<CandidateLossReason, f64> {
    let mut weights = BTreeMap::new();
    for row in rows {
        if row.diagnosis != DiagnosisKind::RankingRecoverable {
            continue;
        }
        if let Some(loss) = ranking_loss(row) {
            *weights.entry(loss.reason).or_default() +=
                priority_at_budget(row, PRIMARY_BUDGET_INDEX);
        }
    }
    weights
}

fn diagnosis_weights(rows: &[ReportRow<'_>]) -> BTreeMap<DiagnosisKind, f64> {
    let mut weights = BTreeMap::new();
    for row in rows {
        *weights.entry(row.diagnosis).or_default() += priority_at_budget(row, PRIMARY_BUDGET_INDEX);
    }
    weights
}

/// Per-row priority weight at a specific budget index in
/// [`BUDGETS`]: `Σ over atoms with rank ≤ |A_B|: (1 −
/// damped_credit(a)) / rank(a)` evaluated at that budget's walker
/// snapshot. Slot [`PRIMARY_BUDGET_INDEX`] is the sort key throughout
/// opportunity / loss / diagnosis rollups; [`LOW_BUDGET_INDEX`] and
/// [`HIGH_BUDGET_INDEX`] surface the per-budget shape in the
/// opportunity table.
pub(super) fn priority_at_budget(row: &ReportRow<'_>, b_index: usize) -> f64 {
    row.arrival.priority_at_b[b_index]
}

pub(super) struct PredecessorGroup<'a> {
    pub(super) predecessor: String,
    pub(super) rows: Vec<&'a ReportRow<'a>>,
    pub(super) hits: usize,
    pub(super) total: usize,
    gap_at_low: f64,
    gap_at_primary: f64,
    gap_at_high: f64,
}

struct PredecessorKindGroup<'a> {
    kind: String,
    rows: Vec<&'a ReportRow<'a>>,
    files: BTreeSet<String>,
    hits: usize,
    total: usize,
    gap_at_low: f64,
    gap_at_primary: f64,
    gap_at_high: f64,
}

/// Sum each row's per-budget priority at the three display indices
/// (`LOW_BUDGET_INDEX`, `PRIMARY_BUDGET_INDEX`, `HIGH_BUDGET_INDEX`).
fn sum_gap_vector(rows: &[&ReportRow<'_>]) -> (f64, f64, f64) {
    let low: f64 = rows
        .iter()
        .map(|r| priority_at_budget(r, LOW_BUDGET_INDEX))
        .sum();
    let primary: f64 = rows
        .iter()
        .map(|r| priority_at_budget(r, PRIMARY_BUDGET_INDEX))
        .sum();
    let high: f64 = rows
        .iter()
        .map(|r| priority_at_budget(r, HIGH_BUDGET_INDEX))
        .sum();
    (low, primary, high)
}

fn predecessor_groups<'a>(rows: &'a [ReportRow<'a>]) -> Vec<PredecessorGroup<'a>> {
    let refs = rows.iter().collect::<Vec<_>>();
    predecessor_groups_from_refs(&refs)
}

fn predecessor_kind_groups<'a>(rows: &'a [ReportRow<'a>]) -> Vec<PredecessorKindGroup<'a>> {
    let mut groups: BTreeMap<String, PredecessorKindGroup<'a>> = BTreeMap::new();
    for row in rows {
        let Some(loss) = ranking_loss(row) else {
            continue;
        };
        if loss.reason != CandidateLossReason::PredecessorNotScheduled {
            continue;
        }
        let Some(predecessor) = loss.predecessor.as_deref() else {
            continue;
        };
        let kind = predecessor_kind_label(predecessor);
        let overlap = ranking_overlap(row);
        let group = groups.entry(kind.clone()).or_insert(PredecessorKindGroup {
            kind,
            rows: Vec::new(),
            files: BTreeSet::new(),
            hits: 0,
            total: 0,
            gap_at_low: 0.0,
            gap_at_primary: 0.0,
            gap_at_high: 0.0,
        });
        group.rows.push(row);
        group.gap_at_low += priority_at_budget(row, LOW_BUDGET_INDEX);
        group.gap_at_primary += priority_at_budget(row, PRIMARY_BUDGET_INDEX);
        group.gap_at_high += priority_at_budget(row, HIGH_BUDGET_INDEX);
        if let Some(file) = predecessor_file(predecessor) {
            group.files.insert(file);
        }
        if let Some(overlap) = overlap {
            group.hits += overlap.hits;
            group.total += overlap.total;
        }
    }
    let mut out: Vec<_> = groups.into_values().collect();
    out.sort_by(|a, b| {
        b.gap_at_primary
            .partial_cmp(&a.gap_at_primary)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                b.gap_at_high
                    .partial_cmp(&a.gap_at_high)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| b.rows.len().cmp(&a.rows.len()))
            .then_with(|| b.hits.cmp(&a.hits))
            .then_with(|| a.kind.cmp(&b.kind))
    });
    out
}

fn predecessor_kind_label(descriptor: &str) -> String {
    if descriptor.starts_with("go decl at ") {
        return "go decl signature batches".to_string();
    }
    if descriptor.starts_with("c decl at ") {
        return "c decl signature batches".to_string();
    }
    if descriptor.starts_with("export at ") {
        return "export batches".to_string();
    }
    if descriptor.starts_with("go decl names surface in ") {
        return "go decl names surfaces".to_string();
    }
    if descriptor.starts_with("pub-item names surface in ") {
        return "pub-item names surfaces".to_string();
    }
    if descriptor.starts_with("pub item at ") {
        return "pub item signature batches".to_string();
    }
    pattern_template(descriptor)
}

fn predecessor_file(descriptor: &str) -> Option<String> {
    const IN_PREFIXES: &[&str] = &[
        "go decl names surface in ",
        "pub-item names surface in ",
        "impl method sigs in ",
        "crate-doc body in ",
        "crate-doc lede in ",
    ];
    for prefix in IN_PREFIXES {
        if let Some(rest) = descriptor.strip_prefix(prefix) {
            return Some(rest.to_string());
        }
    }
    const AT_PREFIXES: &[&str] = &["go decl at ", "c decl at ", "export at ", "pub item at "];
    for prefix in AT_PREFIXES {
        if let Some(rest) = descriptor.strip_prefix(prefix) {
            return Some(
                rest.rsplit_once(':')
                    .map_or(rest, |(path, _)| path)
                    .to_string(),
            );
        }
    }
    None
}

pub(super) fn top_opportunities(rows: &[ReportRow<'_>], limit: usize) -> Vec<Opportunity> {
    let mut opportunities = Vec::new();
    // Predecessor-kind grouping is the calibration-relevant frame (you
    // tune by walker-key class in `value.rs`, not by individual batch).
    // Exact-predecessor groupings cover the same rows and would duplicate
    // gap_weight, crowding the limit; that detail still surfaces per-row
    // in the arrival ledger via `format_parent_gated_groups`.
    for group in predecessor_kind_groups(rows) {
        opportunities.push(Opportunity {
            intervention: format!("promote {}", group.kind),
            rows: group.rows.len(),
            gap_at_low: group.gap_at_low,
            gap_at_primary: group.gap_at_primary,
            gap_at_high: group.gap_at_high,
            evidence: format!(
                "{} file{}, exact total={}/{}",
                group.files.len(),
                plural(group.files.len()),
                group.hits,
                group.total
            ),
            top_row_ids: row_ids(&group.rows, 5),
        });
    }

    // One ranking-race opportunity covers both
    // `DiscoveredUnscheduled` (genuinely lost the rank race) and
    // `TooExpensiveAtFinalMargin` (post-hoc — see module doc; the
    // rank-race subset is recoverable, the never-fit subset is not).
    // The intervention text doesn't claim rank-race for these rows;
    // the per-row loss tag in the arrival ledger preserves which is
    // which.
    push_loss_opportunity(
        &mut opportunities,
        rows,
        &[
            CandidateLossReason::DiscoveredUnscheduled,
            CandidateLossReason::TooExpensiveAtFinalMargin,
        ],
        "tune ranking for high-overlap unscheduled candidates",
        "high-overlap candidates not in the schedule by T_max",
    );
    push_diagnosis_opportunity(
        &mut opportunities,
        rows,
        DiagnosisKind::NoDiscoveredCandidate,
        "add walker candidates for no-discovered rows",
        "NS rows have no discovered line candidate",
    );
    push_diagnosis_opportunity(
        &mut opportunities,
        rows,
        DiagnosisKind::WrongSlice,
        "split wrong-slice walker batches",
        "nearby candidates have low exact atom overlap",
    );

    // Drop opportunities with zero `gap_at_primary` — under the new
    // sort contract these can't move `Score(3000)`. They were
    // surfacing as `0.00` rows that violated "Top opportunities = what
    // to fix at the primary budget."
    opportunities.retain(|o| o.gap_at_primary > 0.0);
    opportunities.sort_by(|a, b| {
        b.gap_at_primary
            .partial_cmp(&a.gap_at_primary)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                b.gap_at_high
                    .partial_cmp(&a.gap_at_high)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| b.rows.cmp(&a.rows))
            .then_with(|| a.intervention.cmp(&b.intervention))
    });
    opportunities.truncate(limit);
    opportunities
}

fn push_loss_opportunity(
    opportunities: &mut Vec<Opportunity>,
    rows: &[ReportRow<'_>],
    reasons: &[CandidateLossReason],
    intervention: &str,
    evidence: &str,
) {
    let matching: Vec<&ReportRow<'_>> = rows
        .iter()
        .filter(|row| ranking_loss(row).is_some_and(|loss| reasons.contains(&loss.reason)))
        .collect();
    if matching.is_empty() {
        return;
    }
    let exact = matching
        .iter()
        .filter_map(|row| ranking_overlap(row))
        .fold((0usize, 0usize), |(hits, total), overlap| {
            (hits + overlap.hits, total + overlap.total)
        });
    let (gap_at_low, gap_at_primary, gap_at_high) = sum_gap_vector(&matching);
    opportunities.push(Opportunity {
        intervention: intervention.to_string(),
        rows: matching.len(),
        gap_at_low,
        gap_at_primary,
        gap_at_high,
        evidence: format!("{evidence}, exact total={}/{}", exact.0, exact.1),
        top_row_ids: row_ids(&matching, 5),
    });
}

fn push_diagnosis_opportunity(
    opportunities: &mut Vec<Opportunity>,
    rows: &[ReportRow<'_>],
    diagnosis: DiagnosisKind,
    intervention: &str,
    evidence: &str,
) {
    let matching: Vec<&ReportRow<'_>> = rows
        .iter()
        .filter(|row| row.diagnosis == diagnosis)
        .collect();
    if matching.is_empty() {
        return;
    }
    let (gap_at_low, gap_at_primary, gap_at_high) = sum_gap_vector(&matching);
    opportunities.push(Opportunity {
        intervention: intervention.to_string(),
        rows: matching.len(),
        gap_at_low,
        gap_at_primary,
        gap_at_high,
        evidence: evidence.to_string(),
        top_row_ids: row_ids(&matching, 5),
    });
}

pub(super) fn row_ids(rows: &[&ReportRow<'_>], limit: usize) -> String {
    let mut weighted_rows = rows.to_vec();
    weighted_rows.sort_by(|a, b| {
        priority_at_budget(b, PRIMARY_BUDGET_INDEX)
            .partial_cmp(&priority_at_budget(a, PRIMARY_BUDGET_INDEX))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.exp_t.cmp(&b.exp_t))
            .then_with(|| a.id.cmp(b.id))
    });
    let mut ids = weighted_rows
        .iter()
        .take(limit)
        .map(|row| row.id)
        .collect::<Vec<_>>();
    if rows.len() > limit {
        ids.push("...");
    }
    ids.join(", ")
}

pub(super) fn ranking_loss<'a>(row: &'a ReportRow<'_>) -> Option<&'a CandidateLoss> {
    row.hint
        .better_unscheduled
        .as_ref()
        .filter(|b| b.exact_overlap.bucket() >= ExactOverlapBucket::High)
        .map(|b| &b.loss)
        .or_else(|| {
            if row.hint.kind == CandidateHintKind::UnscheduledBbox
                && row
                    .hint
                    .exact_overlap
                    .is_some_and(|o| o.bucket() >= ExactOverlapBucket::High)
            {
                row.hint.loss.as_ref()
            } else {
                None
            }
        })
}

pub(super) fn ranking_overlap(row: &ReportRow<'_>) -> Option<ExactOverlap> {
    row.hint
        .better_unscheduled
        .as_ref()
        .filter(|b| b.exact_overlap.bucket() >= ExactOverlapBucket::High)
        .map(|b| b.exact_overlap)
        .or_else(|| {
            if row.hint.kind == CandidateHintKind::UnscheduledBbox {
                row.hint.exact_overlap
            } else {
                None
            }
        })
}

pub(super) fn predecessor_groups_from_refs<'a>(
    rows: &[&'a ReportRow<'a>],
) -> Vec<PredecessorGroup<'a>> {
    let mut groups: BTreeMap<String, PredecessorGroup<'a>> = BTreeMap::new();
    for row in rows {
        let Some(loss) = ranking_loss(row) else {
            continue;
        };
        if loss.reason != CandidateLossReason::PredecessorNotScheduled {
            continue;
        }
        let Some(predecessor) = loss.predecessor.clone() else {
            continue;
        };
        let overlap = ranking_overlap(row);
        let group = groups
            .entry(predecessor.clone())
            .or_insert(PredecessorGroup {
                predecessor,
                rows: Vec::new(),
                hits: 0,
                total: 0,
                gap_at_low: 0.0,
                gap_at_primary: 0.0,
                gap_at_high: 0.0,
            });
        group.rows.push(row);
        group.gap_at_low += priority_at_budget(row, LOW_BUDGET_INDEX);
        group.gap_at_primary += priority_at_budget(row, PRIMARY_BUDGET_INDEX);
        group.gap_at_high += priority_at_budget(row, HIGH_BUDGET_INDEX);
        if let Some(overlap) = overlap {
            group.hits += overlap.hits;
            group.total += overlap.total;
        }
    }
    let mut out: Vec<_> = groups.into_values().collect();
    out.sort_by(|a, b| {
        b.gap_at_primary
            .partial_cmp(&a.gap_at_primary)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                b.gap_at_high
                    .partial_cmp(&a.gap_at_high)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| b.rows.len().cmp(&a.rows.len()))
            .then_with(|| b.hits.cmp(&a.hits))
            .then_with(|| a.predecessor.cmp(&b.predecessor))
    });
    out
}

pub(super) fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}
