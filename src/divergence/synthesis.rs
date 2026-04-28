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
            if arrival.status == ArrivalStatus::Aligned && arrival.credit >= 1.0 && !arrival.over {
                return None;
            }
            let hint = candidate_hint_for_ctx(ns_row, ctx);
            let diagnosis = diagnose_row(arrival, &hint);
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
    pub(super) loss_reason_line: String,
    pub(super) top_rows: String,
}

pub(super) struct Opportunity {
    pub(super) intervention: String,
    pub(super) rows: usize,
    pub(super) gap_weight: f64,
    pub(super) bands: BudgetBands,
    pub(super) evidence: String,
    pub(super) top_row_ids: String,
}

/// Default-budget canonical (3k) and a moderately-common higher tier (6k).
/// These bound the bands shown in the opportunities table — exp_t past
/// `BAND_HI` contributes nothing extractable to default-user output, so
/// it lands in `total` only.
const BAND_DEFAULT: usize = 3_000;
const BAND_HI: usize = 6_000;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct BudgetBands {
    le3k: usize,
    le6k: usize,
    total: usize,
}

impl BudgetBands {
    fn add(&mut self, exp_t: usize) {
        if exp_t <= BAND_DEFAULT {
            self.le3k += 1;
        }
        if exp_t <= BAND_HI {
            self.le6k += 1;
        }
        self.total += 1;
    }

    pub(super) fn label(self) -> String {
        format!("{}/{}/{}", self.le3k, self.le6k, self.total)
    }
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
    let budget_count = losses
        .get(&CandidateLossReason::TooExpensiveAtFinalMargin)
        .copied()
        .unwrap_or(0);
    let pred_weight = loss_weights
        .get(&CandidateLossReason::PredecessorNotScheduled)
        .copied()
        .unwrap_or(0.0);
    let budget_weight = loss_weights
        .get(&CandidateLossReason::TooExpensiveAtFinalMargin)
        .copied()
        .unwrap_or(0.0);
    let race_weight = loss_weights
        .get(&CandidateLossReason::DiscoveredUnscheduled)
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
        if pred_weight > 0.0 && pred_weight >= budget_weight && pred_weight >= race_weight {
            Some(CandidateLossReason::PredecessorNotScheduled)
        } else if budget_weight > 0.0 && budget_weight >= race_weight {
            Some(CandidateLossReason::TooExpensiveAtFinalMargin)
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
    } else if primary_loss == Some(CandidateLossReason::TooExpensiveAtFinalMargin) {
        (
            "budget-pressure bound".to_string(),
            "free final budget / demote late low-value spend".to_string(),
        )
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

    let secondary_intervention = if primary_loss
        != Some(CandidateLossReason::TooExpensiveAtFinalMargin)
        && budget_count > 0
    {
        Some(format!(
            "free final budget for {budget_count} too-expensive candidate{}",
            plural(budget_count)
        ))
    } else if primary_loss != Some(CandidateLossReason::PredecessorNotScheduled) && pred_count > 0 {
        Some(format!(
            "promote predecessors for {pred_count} gated candidate{}",
            plural(pred_count)
        ))
    } else if wrong_slice > ranking && wrong_slice > 0 {
        Some(format!(
            "split wrong-slice batches for {wrong_slice} row{}",
            plural(wrong_slice)
        ))
    } else if no_discovered > 0 {
        Some(format!(
            "investigate {no_discovered} no-discovered row{}",
            plural(no_discovered)
        ))
    } else {
        None
    };

    let evidence = format!(
        "{ranking} ranking-recoverable (w×gap={ranking_weight:.2}), {wrong_slice} wrong-slice/granularity (w×gap={wrong_slice_weight:.2}), {no_discovered} no-discovered (w×gap={no_discovered_weight:.2})"
    );
    let loss_reason_line = format_loss_reason_line(&losses);
    let top_rows = top_opportunities(rows, 1)
        .first()
        .map(|opp| opp.top_row_ids.clone())
        .unwrap_or_else(|| {
            rows.iter()
                .filter(|r| r.diagnosis != DiagnosisKind::TimingOnly)
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
        loss_reason_line,
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

fn loss_reason_counts(rows: &[ReportRow<'_>]) -> BTreeMap<CandidateLossReason, usize> {
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

fn loss_reason_weights(rows: &[ReportRow<'_>]) -> BTreeMap<CandidateLossReason, f64> {
    let mut weights = BTreeMap::new();
    for row in rows {
        if row.diagnosis != DiagnosisKind::RankingRecoverable {
            continue;
        }
        if let Some(loss) = ranking_loss(row) {
            *weights.entry(loss.reason).or_default() += sim_gap_weight(row);
        }
    }
    weights
}

fn diagnosis_weights(rows: &[ReportRow<'_>]) -> BTreeMap<DiagnosisKind, f64> {
    let mut weights = BTreeMap::new();
    for row in rows {
        *weights.entry(row.diagnosis).or_default() += sim_gap_weight(row);
    }
    weights
}

pub(super) fn sim_weight(exp_t: usize) -> f64 {
    (-(exp_t as f64) / TAU).exp()
}

pub(super) fn sim_gap_weight(row: &ReportRow<'_>) -> f64 {
    sim_weight(row.exp_t) * (1.0 - row.arrival.credit).clamp(0.0, 1.0)
}

fn format_loss_reason_line(counts: &BTreeMap<CandidateLossReason, usize>) -> String {
    let pred = counts
        .get(&CandidateLossReason::PredecessorNotScheduled)
        .copied()
        .unwrap_or(0);
    let budget = counts
        .get(&CandidateLossReason::TooExpensiveAtFinalMargin)
        .copied()
        .unwrap_or(0);
    let race = counts
        .get(&CandidateLossReason::DiscoveredUnscheduled)
        .copied()
        .unwrap_or(0);
    format!("{pred} predecessor-gated, {budget} too-expensive, {race} discovered-unscheduled")
}

pub(super) struct PredecessorGroup<'a> {
    pub(super) predecessor: String,
    pub(super) rows: Vec<&'a ReportRow<'a>>,
    pub(super) hits: usize,
    pub(super) total: usize,
    gap_weight: f64,
    bands: BudgetBands,
}

struct PredecessorKindGroup<'a> {
    kind: String,
    rows: Vec<&'a ReportRow<'a>>,
    files: BTreeSet<String>,
    hits: usize,
    total: usize,
    gap_weight: f64,
    bands: BudgetBands,
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
            gap_weight: 0.0,
            bands: BudgetBands::default(),
        });
        group.rows.push(row);
        group.gap_weight += sim_gap_weight(row);
        group.bands.add(row.exp_t);
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
        b.gap_weight
            .partial_cmp(&a.gap_weight)
            .unwrap_or(std::cmp::Ordering::Equal)
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
            gap_weight: group.gap_weight,
            bands: group.bands,
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

    push_loss_opportunity(
        &mut opportunities,
        rows,
        CandidateLossReason::TooExpensiveAtFinalMargin,
        "free final budget / demote late waste",
        "high-overlap candidates exceed final remaining budget",
    );
    push_loss_opportunity(
        &mut opportunities,
        rows,
        CandidateLossReason::DiscoveredUnscheduled,
        "tune ranking for discovered unscheduled candidates",
        "high-overlap candidates fit but did not win",
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

    opportunities.sort_by(|a, b| {
        b.gap_weight
            .partial_cmp(&a.gap_weight)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.rows.cmp(&a.rows))
            .then_with(|| a.intervention.cmp(&b.intervention))
    });
    opportunities.truncate(limit);
    opportunities
}

fn push_loss_opportunity(
    opportunities: &mut Vec<Opportunity>,
    rows: &[ReportRow<'_>],
    reason: CandidateLossReason,
    intervention: &str,
    evidence: &str,
) {
    let matching: Vec<&ReportRow<'_>> = rows
        .iter()
        .filter(|row| ranking_loss(row).is_some_and(|loss| loss.reason == reason))
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
    let gap_weight: f64 = matching.iter().map(|row| sim_gap_weight(row)).sum();
    let bands = budget_bands(&matching);
    opportunities.push(Opportunity {
        intervention: intervention.to_string(),
        rows: matching.len(),
        gap_weight,
        bands,
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
    let gap_weight: f64 = matching.iter().map(|row| sim_gap_weight(row)).sum();
    let bands = budget_bands(&matching);
    opportunities.push(Opportunity {
        intervention: intervention.to_string(),
        rows: matching.len(),
        gap_weight,
        bands,
        evidence: evidence.to_string(),
        top_row_ids: row_ids(&matching, 5),
    });
}

pub(super) fn row_ids(rows: &[&ReportRow<'_>], limit: usize) -> String {
    let mut weighted_rows = rows.to_vec();
    weighted_rows.sort_by(|a, b| {
        sim_gap_weight(b)
            .partial_cmp(&sim_gap_weight(a))
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

fn budget_bands(rows: &[&ReportRow<'_>]) -> BudgetBands {
    let mut bands = BudgetBands::default();
    for row in rows {
        bands.add(row.exp_t);
    }
    bands
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
                gap_weight: 0.0,
                bands: BudgetBands::default(),
            });
        group.rows.push(row);
        group.gap_weight += sim_gap_weight(row);
        group.bands.add(row.exp_t);
        if let Some(overlap) = overlap {
            group.hits += overlap.hits;
            group.total += overlap.total;
        }
    }
    let mut out: Vec<_> = groups.into_values().collect();
    out.sort_by(|a, b| {
        b.gap_weight
            .partial_cmp(&a.gap_weight)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.rows.len().cmp(&a.rows.len()))
            .then_with(|| b.hits.cmp(&a.hits))
            .then_with(|| a.predecessor.cmp(&b.predecessor))
    });
    out
}

pub(super) fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}
