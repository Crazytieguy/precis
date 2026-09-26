//! NS simulator for `validate_ns`. Applies an NS's batches to a
//! [`RenderedTree`] in rank order, the same machinery the scheduler uses,
//! and reports each batch's marginal cost, the running total, and every
//! authoring-rule violation.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::batch::BatchId;
use crate::content::{BatchContent, FsEntries, Render, Span, with_truncate_regex};
use crate::north_star::{NorthStar, NsBatch};
use crate::ns_loader::{path_escapes_root, resolve_content};
use crate::render::{RenderedTree, SourceCache};

pub struct SimulatedBatch {
    pub id: String,
    pub descriptor: String,
    pub cost_tokens: usize,
    pub cumulative_tokens: usize,
    pub violations: Vec<String>,
}

/// Token cap shared with the author prompt.
pub const TOKEN_CAP: usize = 10_000;

/// Growth envelope: a batch may cost at most `100 + 0.3 · cumulative_before`.
fn envelope_max(cumulative_before: usize) -> usize {
    100 + cumulative_before * 3 / 10
}

pub fn simulate_ns(ns: &NorthStar, fixture_root: &Path) -> Vec<SimulatedBatch> {
    let source_cache = SourceCache::new();
    let mut tree = RenderedTree::new(fixture_root.to_path_buf(), source_cache.clone());
    let mut seen_ids: HashSet<&str> = HashSet::new();
    let mut applied: HashMap<&str, BatchId> = HashMap::new();
    let mut applied_by: HashMap<BatchId, &str> = HashMap::new();
    let mut fs_owners: BTreeMap<(PathBuf, String), &str> = BTreeMap::new();
    let mut cumulative = 0;
    let mut out = Vec::with_capacity(ns.batches.len());

    for (position, batch) in ns.batches.iter().enumerate() {
        let mut violations = Vec::new();
        if !seen_ids.insert(&batch.id) {
            violations.push(format!(
                "duplicate batch id {:?} (first occurrence passes; this is the clash)",
                batch.id
            ));
        }
        if let Some(predecessor) = &batch.predecessor
            && !applied.contains_key(predecessor.as_str())
        {
            violations.push(format!(
                "predecessor {predecessor:?} not found in prior batches"
            ));
        }
        let record = |violations, cost_tokens, cumulative_tokens| SimulatedBatch {
            id: batch.id.clone(),
            descriptor: batch.descriptor.clone(),
            cost_tokens,
            cumulative_tokens,
            violations,
        };

        // A span the renderer can't apply (missing file, bad range or
        // regex) leaves the batch out of the simulation. Quality-only
        // violations keep it in, so its successors still find it.
        if let BatchContent::Lines { spans, .. } = &batch.content
            && validate_spans(spans, fixture_root, &source_cache, &mut violations)
        {
            out.push(record(violations, 0, cumulative));
            continue;
        }
        let content = match resolve_content(&batch.content, fixture_root) {
            Ok(content) => content,
            Err(e) => {
                violations.push(format!("fs content resolution failed: {e}"));
                out.push(record(violations, 0, cumulative));
                continue;
            }
        };
        if is_zero_atom(&content) {
            violations.push(
                "batch resolves to zero atoms — it renders as a no-op, so divergence can never credit it. Delete the batch or point it at real content.".to_string(),
            );
        }
        check_fs_entries(&content, &batch.id, &mut fs_owners, &mut violations);

        let cost = tree.marginal_cost(&content).tokens;
        let max_allowed = envelope_max(cumulative);
        if cost > max_allowed {
            violations.push(format!(
                "growth envelope: batch cost {cost} > {max_allowed} tokens (cumulative so far: {cumulative}; envelope = 100 + 0.3·cumulative). Split the batch, or rank smaller batches earlier."
            ));
        }
        cumulative += cost;
        if cumulative > TOKEN_CAP {
            violations.push(format!(
                "cap exceeded: cumulative {cumulative} > {TOKEN_CAP} tokens"
            ));
        }

        let batch_id = BatchId::new(position);
        let ancestors = collect_ancestors(&batch.id, &ns.batches, &applied);
        for conflict in tree.apply(&content, batch_id, |id| ancestors.contains(&id)) {
            let owner = applied_by.get(&conflict.existing_owner).map_or_else(
                || format!("{:?}", conflict.existing_owner),
                |id| id.to_string(),
            );
            violations.push(format!(
                "non-ancestor overlap: {}:{} already owned by {owner} (add a predecessor edge or move the span)",
                conflict.path.display(),
                conflict.line
            ));
        }
        applied.insert(&batch.id, batch_id);
        applied_by.insert(batch_id, &batch.id);
        out.push(record(violations, cost, cumulative));
    }
    out
}

/// Resolved content with no atoms: no spans, or listings that list nothing.
fn is_zero_atom(content: &BatchContent) -> bool {
    match content {
        BatchContent::Lines { spans, .. } => spans.is_empty(),
        BatchContent::Fs { groups } => groups
            .iter()
            .all(|g| matches!(&g.entries, FsEntries::Listed(paths) if paths.is_empty())),
    }
}

fn check_fs_entries<'a>(
    content: &BatchContent,
    batch_id: &'a str,
    owners: &mut BTreeMap<(PathBuf, String), &'a str>,
    violations: &mut Vec<String>,
) {
    let BatchContent::Fs { groups } = content else {
        return;
    };
    for group in groups {
        let FsEntries::Listed(paths) = &group.entries else {
            continue;
        };
        for name in paths.iter().filter_map(|p| p.file_name()?.to_str()) {
            let atom = (group.parent.clone(), name.to_string());
            if let Some(owner) = owners.get(&atom) {
                violations.push(format!(
                    "overlapping fs entry: {} lists {name:?}, already owned by {owner} (split-listing rows must partition entries)",
                    group.parent.display()
                ));
            } else {
                owners.insert(atom, batch_id);
            }
        }
    }
}

/// Appends span violations; returns whether any makes the batch
/// unrenderable.
fn validate_spans(
    spans: &[Span],
    fixture_root: &Path,
    cache: &SourceCache,
    violations: &mut Vec<String>,
) -> bool {
    let mut blocking = false;
    let mut seen_lines: HashSet<(&Path, usize)> = HashSet::new();
    for span in spans {
        let path = span.path.display();
        let (start, end) = (span.start, span.end);
        if path_escapes_root(&span.path) {
            violations.push(format!(
                "span path {path} escapes the fixture root (span paths must be fixture-root-relative — no absolute paths or `..`)"
            ));
            blocking = true;
            continue;
        }
        let Some(source) = cache.get(&fixture_root.join(&span.path)) else {
            violations.push(format!("span file missing: {path}"));
            blocking = true;
            continue;
        };
        if start == 0 || start > end {
            violations.push(format!(
                "span range inverted or zero-indexed at {path}: start={start}, end={end}"
            ));
            blocking = true;
            continue;
        }
        if end > source.line_count() {
            violations.push(format!(
                "span out of range at {path}: {start}..={end} (file has {} lines)",
                source.line_count()
            ));
            blocking = true;
            continue;
        }
        for line in start..=end {
            if !seen_lines.insert((&span.path, line)) {
                violations.push(format!(
                    "overlapping spans within one batch at {path}:{line} (batch spans must be disjoint — cross-batch overrides go through predecessor edges)"
                ));
                blocking = true;
            }
        }
        match &span.render {
            Render::Full => {}
            Render::Ellipsis if start == end => {}
            Render::Ellipsis => violations.push(format!(
                "multi-line Ellipsis span at {path}:{start}..={end} renders one `…` per line — redundant + visually indistinguishable from a single marker. Use one Ellipsis span per line, or `Full`/`Truncated` if the lines should render verbatim."
            )),
            Render::Truncated { pattern } => with_truncate_regex(pattern, |re| {
                let Some(re) = re else {
                    violations.push(format!(
                        "invalid Truncated regex `{pattern}` at {path}:{start}..={end}"
                    ));
                    blocking = true;
                    return;
                };
                let mut flagged = false;
                let mut elides_a_word = false;
                for line in start..=end {
                    let text = source.line(line).unwrap_or_default();
                    let Some(m) = re.find(text).filter(|m| !m.as_str().is_empty()) else {
                        violations.push(format!(
                            "Truncated regex `{pattern}` produced no/empty match at {path}:{line}"
                        ));
                        blocking = true;
                        flagged = true;
                        continue;
                    };
                    elides_a_word |= text[m.end()..].contains(char::is_alphanumeric);
                    let full_tokens = crate::tokenizer::count(text);
                    let truncated_tokens = crate::tokenizer::count(&format!("{}…", m.as_str()));
                    if truncated_tokens >= full_tokens {
                        violations.push(format!(
                            "Truncated render saves no tokens at {path}:{line} with pattern `{pattern}` (full line: {full_tokens} tokens, truncated `<match>…`: {truncated_tokens} tokens). Use `Full` here, or pick a pattern that drops the meaningful tail."
                        ));
                        flagged = true;
                    }
                }
                if !elides_a_word && !flagged {
                    violations.push(format!(
                        "Truncated render at {path}:{start}..={end} with pattern `{pattern}` never elides a word character — every line's dropped tail is punctuation/whitespace, so the `…` implies substance that isn't there. Use `Full`."
                    ));
                }
            }),
        }
    }
    blocking
}

/// Applied batches on `ns_id`'s predecessor chain.
fn collect_ancestors(
    ns_id: &str,
    batches: &[NsBatch],
    applied: &HashMap<&str, BatchId>,
) -> HashSet<BatchId> {
    let mut out = HashSet::new();
    let mut current = ns_id;
    while let Some(predecessor) = batches
        .iter()
        .find(|b| b.id == current)
        .and_then(|b| b.predecessor.as_deref())
        && let Some(&id) = applied.get(predecessor)
        && out.insert(id)
    {
        current = predecessor;
    }
    out
}
