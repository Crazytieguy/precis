use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum CandidateHintKind {
    ScheduledBbox,
    UnscheduledBbox,
    ScheduledSameFile,
    UnscheduledSameFile,
    FsOnly,
    NoDiscoveredCandidate,
}

impl CandidateHintKind {
    pub(super) fn label(self) -> &'static str {
        match self {
            CandidateHintKind::ScheduledBbox => "scheduled bbox",
            CandidateHintKind::UnscheduledBbox => "unscheduled bbox",
            CandidateHintKind::ScheduledSameFile => "scheduled same-file",
            CandidateHintKind::UnscheduledSameFile => "unscheduled same-file",
            CandidateHintKind::FsOnly => "fs-only",
            CandidateHintKind::NoDiscoveredCandidate => "no discovered candidate",
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct CandidateHint {
    pub(super) kind: CandidateHintKind,
    pub(super) cell: String,
    pub(super) exact_overlap: Option<ExactOverlap>,
    pub(super) loss: Option<CandidateLoss>,
    pub(super) better_unscheduled: Option<UnscheduledCandidateHint>,
}

#[derive(Debug, Clone)]
pub(super) struct UnscheduledCandidateHint {
    pub(super) descriptor: String,
    pub(super) exact_overlap: ExactOverlap,
    pub(super) bbox_atoms: usize,
    pub(super) loss: CandidateLoss,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CandidateLoss {
    pub(super) reason: CandidateLossReason,
    pub(super) predecessor: Option<String>,
}

impl CandidateLoss {
    fn label(&self) -> String {
        match (&self.reason, &self.predecessor) {
            (CandidateLossReason::PredecessorNotScheduled, Some(pred)) => {
                format!("{}: {pred}", self.reason.label())
            }
            _ => self.reason.label().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum CandidateLossReason {
    PredecessorNotScheduled,
    TooExpensiveAtFinalMargin,
    DiscoveredUnscheduled,
}

impl CandidateLossReason {
    pub(super) fn label(self) -> &'static str {
        match self {
            CandidateLossReason::PredecessorNotScheduled => "predecessor not scheduled",
            CandidateLossReason::TooExpensiveAtFinalMargin => "too expensive at final margin",
            CandidateLossReason::DiscoveredUnscheduled => "discovered unscheduled",
        }
    }

    /// `TooExpensiveAtFinalMargin` shares `tune ranking` with
    /// `DiscoveredUnscheduled` — see the divergence module doc on
    /// the ranking-race bucket for why.
    pub(super) fn intervention_label(self) -> &'static str {
        match self {
            CandidateLossReason::PredecessorNotScheduled => "promote predecessor",
            CandidateLossReason::TooExpensiveAtFinalMargin
            | CandidateLossReason::DiscoveredUnscheduled => "tune ranking",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum DiagnosisKind {
    RankingRecoverable,
    WrongSlice,
    NoDiscoveredCandidate,
    FsListing,
    MixedUnknown,
}

impl DiagnosisKind {
    pub(super) fn label(self) -> &'static str {
        match self {
            DiagnosisKind::RankingRecoverable => "ranking-recoverable",
            DiagnosisKind::WrongSlice => "wrong-slice / granularity",
            DiagnosisKind::NoDiscoveredCandidate => "no discovered candidate",
            DiagnosisKind::FsListing => "fs/listing",
            DiagnosisKind::MixedUnknown => "mixed/unknown",
        }
    }

    pub(super) fn likely_lever(self) -> &'static str {
        match self {
            DiagnosisKind::RankingRecoverable => "value/ranking",
            DiagnosisKind::WrongSlice => "walker granularity / wrong slice",
            DiagnosisKind::NoDiscoveredCandidate => "walker coverage or predecessor-gated emit",
            DiagnosisKind::FsListing => "filesystem/listing value",
            DiagnosisKind::MixedUnknown => "inspect row",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ExactOverlapBucket {
    None,
    Low,
    High,
    Full,
}

impl ExactOverlapBucket {
    pub(super) fn label(self) -> &'static str {
        match self {
            ExactOverlapBucket::None => "none",
            ExactOverlapBucket::Low => "low",
            ExactOverlapBucket::High => "high",
            ExactOverlapBucket::Full => "full",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ExactOverlap {
    pub(super) hits: usize,
    pub(super) total: usize,
}

impl ExactOverlap {
    pub(super) fn bucket(self) -> ExactOverlapBucket {
        if self.hits == 0 {
            return ExactOverlapBucket::None;
        }
        if self.hits >= self.total {
            return ExactOverlapBucket::Full;
        }
        if (self.hits as f64) / (self.total as f64) >= 0.8 {
            ExactOverlapBucket::High
        } else {
            ExactOverlapBucket::Low
        }
    }

    pub(super) fn label(self) -> String {
        format!("{}/{}", self.hits, self.total)
    }
}

pub(super) fn candidate_hint_for_ctx(ns_row: &NsRow, ctx: &BuildCtx) -> CandidateHint {
    candidate_hint(
        ns_row,
        &ctx.walker_rows,
        &ctx.candidate_rows,
        &ctx.scheduled_keys,
        ctx.schedule
            .budget
            .saturating_sub(ctx.schedule.cumulative_tokens),
        &ctx.fixture_root,
    )
}

pub(super) fn diagnose_row(hint: &CandidateHint) -> DiagnosisKind {
    if hint
        .better_unscheduled
        .as_ref()
        .is_some_and(|b| b.exact_overlap.bucket() >= ExactOverlapBucket::High)
    {
        return DiagnosisKind::RankingRecoverable;
    }

    match hint.kind {
        CandidateHintKind::UnscheduledBbox => match hint.exact_overlap.map(|o| o.bucket()) {
            Some(ExactOverlapBucket::High | ExactOverlapBucket::Full) => {
                DiagnosisKind::RankingRecoverable
            }
            _ => DiagnosisKind::WrongSlice,
        },
        CandidateHintKind::ScheduledBbox => match hint.exact_overlap.map(|o| o.bucket()) {
            Some(ExactOverlapBucket::High | ExactOverlapBucket::Full) => {
                DiagnosisKind::MixedUnknown
            }
            _ => DiagnosisKind::WrongSlice,
        },
        CandidateHintKind::ScheduledSameFile | CandidateHintKind::UnscheduledSameFile => {
            DiagnosisKind::WrongSlice
        }
        CandidateHintKind::NoDiscoveredCandidate => DiagnosisKind::NoDiscoveredCandidate,
        CandidateHintKind::FsOnly => DiagnosisKind::FsListing,
    }
}

pub(super) fn candidate_hint(
    ns_row: &NsRow,
    walker_rows: &[WalkerRow<'_>],
    candidate_rows: &[CandidateRow<'_>],
    scheduled_keys: &BTreeSet<String>,
    remaining_tokens: usize,
    fixture_root: &Path,
) -> CandidateHint {
    let mut ns_box: BTreeMap<PathBuf, (usize, usize)> = BTreeMap::new();
    let mut ns_line_atoms: BTreeSet<Atom> = BTreeSet::new();
    let mut has_fs = false;
    for ga in &ns_row.atoms {
        match &ga.atom {
            Atom::Line { path, line } => {
                ns_line_atoms.insert(ga.atom.clone());
                ns_box
                    .entry(path.clone())
                    .and_modify(|(lo, hi)| {
                        *lo = (*lo).min(*line);
                        *hi = (*hi).max(*line);
                    })
                    .or_insert((*line, *line));
            }
            Atom::Fs { .. } => has_fs = true,
        }
    }
    if ns_box.is_empty() {
        let cell = if has_fs { "fs-only" } else { "no atoms" };
        return CandidateHint {
            kind: CandidateHintKind::FsOnly,
            cell: cell.to_string(),
            exact_overlap: None,
            loss: None,
            better_unscheduled: None,
        };
    }

    if let Some((wr, count)) = best_scheduled_bbox(walker_rows, &ns_box) {
        let desc = strip_fixture_root(&wr.batch.descriptor, fixture_root);
        let exact_overlap = exact_overlap(&wr.atoms, &ns_line_atoms);
        let better_unscheduled =
            best_unscheduled_bbox_by_exact(candidate_rows, &ns_box, &ns_line_atoms).and_then(
                |(cr, candidate_count, candidate_overlap)| {
                    is_better_overlap(candidate_overlap, exact_overlap).then(|| {
                        let descriptor = strip_fixture_root(&cr.batch.descriptor, fixture_root);
                        UnscheduledCandidateHint {
                            descriptor,
                            exact_overlap: candidate_overlap,
                            bbox_atoms: candidate_count,
                            loss: candidate_loss(
                                cr,
                                candidate_rows,
                                scheduled_keys,
                                remaining_tokens,
                                fixture_root,
                            ),
                        }
                    })
                },
            );
        let better_cell = better_unscheduled
            .as_ref()
            .map(|b| {
                format!(
                    "; better unscheduled exact={}: {} ({} atoms, {})",
                    b.exact_overlap.label(),
                    b.descriptor,
                    b.bbox_atoms,
                    b.loss.label()
                )
            })
            .unwrap_or_default();
        return CandidateHint {
            kind: CandidateHintKind::ScheduledBbox,
            cell: format!(
                "[scheduled bbox exact={}] {desc} (t={}, {count} atoms){better_cell}",
                exact_overlap.label(),
                wr.seen_t
            ),
            exact_overlap: Some(exact_overlap),
            loss: None,
            better_unscheduled,
        };
    }

    if let Some((cr, count, exact_overlap)) =
        best_unscheduled_bbox_by_exact(candidate_rows, &ns_box, &ns_line_atoms)
    {
        let desc = strip_fixture_root(&cr.batch.descriptor, fixture_root);
        let loss = candidate_loss(
            cr,
            candidate_rows,
            scheduled_keys,
            remaining_tokens,
            fixture_root,
        );
        let loss_label = loss.label();
        return CandidateHint {
            kind: CandidateHintKind::UnscheduledBbox,
            cell: format!(
                "[unscheduled bbox exact={}] {desc} ({count} atoms, {})",
                exact_overlap.label(),
                loss_label
            ),
            exact_overlap: Some(exact_overlap),
            loss: Some(loss),
            better_unscheduled: None,
        };
    }

    let ns_files: BTreeSet<&PathBuf> = ns_box.keys().collect();
    if let Some((wr, count)) = best_scheduled_same_file(walker_rows, &ns_files) {
        let desc = strip_fixture_root(&wr.batch.descriptor, fixture_root);
        return CandidateHint {
            kind: CandidateHintKind::ScheduledSameFile,
            cell: format!(
                "[scheduled same-file] {desc} (t={}, {count} atoms)",
                wr.seen_t
            ),
            exact_overlap: None,
            loss: None,
            better_unscheduled: None,
        };
    }

    if let Some((cr, count)) = best_unscheduled_candidate_same_file(candidate_rows, &ns_files) {
        let desc = strip_fixture_root(&cr.batch.descriptor, fixture_root);
        let loss = candidate_loss(
            cr,
            candidate_rows,
            scheduled_keys,
            remaining_tokens,
            fixture_root,
        );
        let loss_label = loss.label();
        return CandidateHint {
            kind: CandidateHintKind::UnscheduledSameFile,
            cell: format!(
                "[unscheduled same-file] {desc} ({count} atoms, {})",
                loss_label
            ),
            exact_overlap: None,
            loss: Some(loss),
            better_unscheduled: None,
        };
    }

    CandidateHint {
        kind: CandidateHintKind::NoDiscoveredCandidate,
        cell: "no discovered line candidate".to_string(),
        exact_overlap: None,
        loss: None,
        better_unscheduled: None,
    }
}

fn exact_overlap(atoms: &[GradedAtom], ns_line_atoms: &BTreeSet<Atom>) -> ExactOverlap {
    let candidate_atoms: BTreeSet<&Atom> = atoms.iter().map(|a| &a.atom).collect();
    ExactOverlap {
        hits: ns_line_atoms
            .iter()
            .filter(|atom| candidate_atoms.contains(atom))
            .count(),
        total: ns_line_atoms.len(),
    }
}

fn is_better_overlap(candidate: ExactOverlap, current: ExactOverlap) -> bool {
    let candidate_key = (candidate.bucket(), candidate.hits);
    let current_key = (current.bucket(), current.hits);
    candidate_key > current_key
}

fn candidate_loss(
    row: &CandidateRow<'_>,
    candidate_rows: &[CandidateRow<'_>],
    scheduled_keys: &BTreeSet<String>,
    remaining_tokens: usize,
    fixture_root: &Path,
) -> CandidateLoss {
    if let Some(pred) = &row.batch.predecessor
        && !scheduled_keys.contains(pred)
    {
        let predecessor = candidate_rows
            .iter()
            .find(|candidate| &candidate.batch.key == pred)
            .map(|candidate| strip_fixture_root(&candidate.batch.descriptor, fixture_root))
            .or_else(|| Some(pred.clone()));
        return CandidateLoss {
            reason: CandidateLossReason::PredecessorNotScheduled,
            predecessor,
        };
    }
    if row.final_cost.tokens > remaining_tokens {
        return CandidateLoss {
            reason: CandidateLossReason::TooExpensiveAtFinalMargin,
            predecessor: None,
        };
    }
    CandidateLoss {
        reason: CandidateLossReason::DiscoveredUnscheduled,
        predecessor: None,
    }
}

fn bbox_overlap_count(atoms: &[GradedAtom], ns_box: &BTreeMap<PathBuf, (usize, usize)>) -> usize {
    atoms
        .iter()
        .filter(|a| {
            matches!(
                &a.atom,
                Atom::Line { path, line }
                    if ns_box
                        .get(path)
                        .is_some_and(|(lo, hi)| *lo <= *line && *line <= *hi)
            )
        })
        .count()
}

fn same_file_count(atoms: &[GradedAtom], ns_files: &BTreeSet<&PathBuf>) -> usize {
    atoms
        .iter()
        .filter(|a| matches!(&a.atom, Atom::Line { path, .. } if ns_files.contains(path)))
        .count()
}

/// Pick the best-scoring row by `count_fn`, where higher count = better and
/// zero count drops the row. Generic over row type so walker rows and
/// candidate rows share the same implementation.
fn best_by_count<'a, R, F>(
    rows: impl IntoIterator<Item = &'a R>,
    count_fn: F,
) -> Option<(&'a R, usize)>
where
    R: 'a,
    F: Fn(&'a R) -> usize,
{
    rows.into_iter()
        .filter_map(|row| {
            let count = count_fn(row);
            (count > 0).then_some((row, count))
        })
        .max_by_key(|(_, count)| *count)
}

fn best_scheduled_bbox<'a>(
    rows: &'a [WalkerRow<'a>],
    ns_box: &BTreeMap<PathBuf, (usize, usize)>,
) -> Option<(&'a WalkerRow<'a>, usize)> {
    best_by_count(rows.iter(), |row| bbox_overlap_count(&row.atoms, ns_box))
}

/// Rank unscheduled bbox candidates by exact-atom overlap (then bbox count)
/// so multi-span NS rows pick the candidate that matches their specific
/// atoms, not just the one with the most lines in the collapsed bbox.
fn best_unscheduled_bbox_by_exact<'a>(
    rows: &'a [CandidateRow<'a>],
    ns_box: &BTreeMap<PathBuf, (usize, usize)>,
    ns_line_atoms: &BTreeSet<Atom>,
) -> Option<(&'a CandidateRow<'a>, usize, ExactOverlap)> {
    rows.iter()
        .filter(|row| !row.scheduled)
        .filter_map(|row| {
            let count = bbox_overlap_count(&row.atoms, ns_box);
            if count == 0 {
                return None;
            }
            let overlap = exact_overlap(&row.atoms, ns_line_atoms);
            Some((row, count, overlap))
        })
        .max_by_key(|(_, count, overlap)| (overlap.bucket(), overlap.hits, *count))
}

fn best_scheduled_same_file<'a>(
    rows: &'a [WalkerRow<'a>],
    ns_files: &BTreeSet<&PathBuf>,
) -> Option<(&'a WalkerRow<'a>, usize)> {
    best_by_count(rows.iter(), |row| same_file_count(&row.atoms, ns_files))
}

fn best_unscheduled_candidate_same_file<'a>(
    rows: &'a [CandidateRow<'a>],
    ns_files: &BTreeSet<&PathBuf>,
) -> Option<(&'a CandidateRow<'a>, usize)> {
    best_by_count(rows.iter().filter(|row| !row.scheduled), |row| {
        same_file_count(&row.atoms, ns_files)
    })
}
