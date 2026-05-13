use super::*;

pub(super) struct ReportRow<'a> {
    pub(super) id: &'a str,
    pub(super) exp_t: usize,
    pub(super) descriptor: &'a str,
    pub(super) atoms: &'a [GradedAtom],
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
                atoms: &ns_row.atoms,
                arrival,
                hint,
                diagnosis,
            })
        })
        .collect()
}

pub(super) struct Opportunity {
    pub(super) intervention: String,
    pub(super) rows: usize,
    /// Σ priority over the opportunity's rows at the low display
    /// budget (1K).
    pub(super) gap_at_low: f64,
    /// Σ priority at the primary budget (3K). Direct proxy for
    /// `Score(3000)` headroom and the primary sort key for
    /// opportunities, so iterators land on interventions that move the
    /// optimization target rather than future-budget rows.
    pub(super) gap_at_primary: f64,
    /// Σ priority at the high display budget (9K).
    pub(super) gap_at_high: f64,
    pub(super) evidence: String,
    pub(super) top_row_ids: String,
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

/// Sum each row's priority at the three display budgets (1K / 3K /
/// 9K).
fn sum_gap_vector(rows: &[&ReportRow<'_>]) -> (f64, f64, f64) {
    rows.iter()
        .fold((0.0, 0.0, 0.0), |(low, primary, high), r| {
            let [l, p, h] = r.arrival.priority_at_b;
            (low + l, primary + p, high + h)
        })
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
        let [low, primary, high] = row.arrival.priority_at_b;
        group.gap_at_low += low;
        group.gap_at_primary += primary;
        group.gap_at_high += high;
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
        b.arrival.priority_at_b[PRIORITY_AT_PRIMARY]
            .partial_cmp(&a.arrival.priority_at_b[PRIORITY_AT_PRIMARY])
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
        let [low, primary, high] = row.arrival.priority_at_b;
        group.gap_at_low += low;
        group.gap_at_primary += primary;
        group.gap_at_high += high;
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
