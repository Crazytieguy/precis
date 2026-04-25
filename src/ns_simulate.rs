//! NS simulator. Drives a [`RenderedTree`] in NS rank order — the same
//! machinery the scheduler uses, just picking batches off a pre-ranked list
//! instead of greedily. Produces per-batch marginal cost + cumulative +
//! violation data for `validate-ns`.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::batch::{Batch, BatchId, ValueSignals};
use crate::content::{BatchContent, Render, Span};
use crate::north_star::NorthStar;
use crate::ns_loader::resolve_content;
use crate::render::{Cost, RenderedTree, SourceCache};

/// Per-batch simulator output.
#[derive(Debug, Clone)]
pub struct SimulatedBatch {
    pub id: String,
    pub descriptor: String,
    pub marginal_cost: Cost,
    pub cumulative_tokens: usize,
    /// Parent NS-id this batch declared as predecessor. Copied verbatim
    /// from the NS; closure is verified separately.
    pub predecessor: Option<String>,
    pub violations: Vec<Violation>,
}

/// Per-NS-batch violation kinds. The validator emits them as it finds them;
/// each [`SimulatedBatch`] carries the violations that apply to that batch.
/// Keep new variants in sync with `validate-ns`'s formatter.
#[derive(Debug, Clone)]
pub enum Violation {
    /// Two batches share the same NS id. Reported on the duplicate
    /// (second and later) occurrence.
    DuplicateBatchId(String),
    /// Declared predecessor id doesn't refer to a batch ranked earlier.
    PredecessorMissing(String),
    /// Span targets a file that doesn't exist under the fixture root.
    SpanFileMissing(PathBuf),
    /// Span `start > end` or `start == 0`.
    SpanInvertedRange {
        path: PathBuf,
        start: usize,
        end: usize,
    },
    /// Span covers line(s) beyond the file's line count.
    SpanOutOfRange {
        path: PathBuf,
        start: usize,
        end: usize,
        file_lines: usize,
    },
    /// Truncated render's regex didn't compile.
    RegexInvalid {
        path: PathBuf,
        start: usize,
        end: usize,
        pattern: String,
    },
    /// Truncated render's regex produced no/empty match on a source line
    /// in the span.
    RegexNoMatch {
        path: PathBuf,
        line: usize,
        pattern: String,
    },
    /// Truncated render's pattern matches enough of the line that the
    /// rendered `<match>…` is no shorter (in tokens) than the full line —
    /// truncation saves nothing. Pattern should drop the meaningful tail;
    /// if there is no meaningful tail, use `Render::Full` instead.
    TruncationSavesNothing {
        path: PathBuf,
        line: usize,
        pattern: String,
        full_tokens: usize,
        truncated_tokens: usize,
    },
    /// Fs group's parent or listed child doesn't exist.
    FsResolveFailed(String),
    /// A span would overwrite a line owned by a non-ancestor batch.
    NonAncestorOverlap {
        path: PathBuf,
        line: usize,
        existing_batch: String,
    },
    /// `cost > ENV_BASE + ENV_COEF · cumulative_before` — the growth
    /// envelope that replaces the old 2× rule. See
    /// `docs/design-notes.md` for rationale.
    GrowthEnvelope {
        cost: usize,
        cumulative_before: usize,
        max_allowed: usize,
    },
    /// Two spans in the same batch cover the same `(path, line)`. Batch
    /// spans must be disjoint; cross-batch overrides go through
    /// predecessor edges instead.
    OverlappingSpans { path: PathBuf, line: usize },
    /// Cumulative cost exceeds the token cap after this batch.
    CapExceeded { cumulative: usize, cap: usize },
}

/// Result of simulating an NS end-to-end.
#[derive(Debug, Clone)]
pub struct SimulationReport {
    pub batches: Vec<SimulatedBatch>,
    pub total_tokens: usize,
    pub largest_batch: Option<(String, usize)>,
}

/// Token cap shared with the author prompt.
pub const TOKEN_CAP: usize = 10_000;

/// Growth envelope: `max_cost = ENV_BASE + ENV_COEF · cumulative_before`.
pub const ENV_BASE: usize = 100;
const ENV_COEF_NUMERATOR: usize = 3;
const ENV_COEF_DENOMINATOR: usize = 10;

pub fn envelope_max(cumulative_before: usize) -> usize {
    ENV_BASE + cumulative_before * ENV_COEF_NUMERATOR / ENV_COEF_DENOMINATOR
}

/// Simulate applying an NS's batches to a fresh `RenderedTree` in rank
/// order. Returns per-batch cost + collected violations. Does not read the
/// walker — NS content resolves via [`resolve_content`].
pub fn simulate_ns(ns: &NorthStar, fixture_root: &Path) -> Result<SimulationReport> {
    let source_cache = SourceCache::new();
    let mut tree = RenderedTree::new(fixture_root.to_path_buf(), source_cache.clone());

    // Duplicate-id pass. Attach the violation to the later (offending)
    // occurrence so the first copy passes cleanly.
    let mut first_seen: HashMap<&str, usize> = HashMap::new();
    let mut duplicate_at: HashMap<usize, Violation> = HashMap::new();
    for (pos, b) in ns.batches.iter().enumerate() {
        if first_seen.contains_key(b.id.as_str()) {
            duplicate_at.insert(pos, Violation::DuplicateBatchId(b.id.clone()));
        } else {
            first_seen.insert(b.id.as_str(), pos);
        }
    }

    let mut ns_id_to_batch_id: HashMap<String, BatchId> = HashMap::new();
    let mut batch_id_to_ns: HashMap<BatchId, String> = HashMap::new();
    let mut cumulative: usize = 0;
    let mut largest: Option<(String, usize)> = None;
    let mut batches_out: Vec<SimulatedBatch> = Vec::with_capacity(ns.batches.len());

    for (pos, ns_batch) in ns.batches.iter().enumerate() {
        let mut violations = Vec::new();
        if let Some(v) = duplicate_at.remove(&pos) {
            violations.push(v);
        }

        if let Some(pred_id) = &ns_batch.predecessor
            && !ns_id_to_batch_id.contains_key(pred_id)
        {
            violations.push(Violation::PredecessorMissing(pred_id.clone()));
        }

        // Pre-validate Lines content. Fs-content failures surface as
        // FsResolveFailed below via `resolve_content`. If any span fails
        // validation, skip cost/apply — render would panic on bad spans.
        // Render-blocking span errors (out-of-range, bad regex, intra-
        // batch overlap, missing file) prevent applying the batch to the
        // tree at all — skip cost/apply, mark it dead. Quality-only
        // violations (e.g. `TruncationSavesNothing`) still let the batch
        // render fine; record them but keep the batch in the simulation
        // so successors don't see false-positive `PredecessorMissing`.
        let had_render_blocking_error = if let BatchContent::Lines { spans } = &ns_batch.content {
            let span_v = validate_spans(spans, fixture_root, &source_cache);
            let blocking = span_v.iter().any(is_render_blocking);
            violations.extend(span_v);
            blocking
        } else {
            false
        };
        if had_render_blocking_error {
            batches_out.push(skipped_batch(ns_batch, cumulative, violations));
            continue;
        }

        let content = match resolve_content(&ns_batch.content, fixture_root) {
            Ok(c) => c,
            Err(e) => {
                violations.push(Violation::FsResolveFailed(e.to_string()));
                batches_out.push(skipped_batch(ns_batch, cumulative, violations));
                continue;
            }
        };

        let batch_id = BatchId::new(pos);
        let batch = Batch {
            content,
            signals: ValueSignals::default(),
        };

        let cost = tree.marginal_cost(&batch);
        let cumulative_before = cumulative;
        cumulative = cumulative.saturating_add(cost.tokens);

        let max_allowed = envelope_max(cumulative_before);
        if cost.tokens > max_allowed {
            violations.push(Violation::GrowthEnvelope {
                cost: cost.tokens,
                cumulative_before,
                max_allowed,
            });
        }
        if cumulative > TOKEN_CAP {
            violations.push(Violation::CapExceeded {
                cumulative,
                cap: TOKEN_CAP,
            });
        }

        let ancestors = collect_ancestors(&ns_batch.id, &ns.batches, &ns_id_to_batch_id);
        let conflicts = tree.apply(&batch, batch_id, |id| ancestors.contains(&id));
        for c in conflicts {
            let existing_batch = batch_id_to_ns
                .get(&c.existing_owner)
                .cloned()
                .unwrap_or_else(|| format!("{:?}", c.existing_owner));
            violations.push(Violation::NonAncestorOverlap {
                path: c.path,
                line: c.line,
                existing_batch,
            });
        }

        ns_id_to_batch_id.insert(ns_batch.id.clone(), batch_id);
        batch_id_to_ns.insert(batch_id, ns_batch.id.clone());

        if largest.as_ref().is_none_or(|(_, c)| cost.tokens > *c) {
            largest = Some((ns_batch.id.clone(), cost.tokens));
        }

        batches_out.push(SimulatedBatch {
            id: ns_batch.id.clone(),
            descriptor: ns_batch.descriptor.clone(),
            marginal_cost: cost,
            cumulative_tokens: cumulative,
            predecessor: ns_batch.predecessor.clone(),
            violations,
        });
    }

    Ok(SimulationReport {
        batches: batches_out,
        total_tokens: cumulative,
        largest_batch: largest,
    })
}

fn is_render_blocking(v: &Violation) -> bool {
    matches!(
        v,
        Violation::SpanFileMissing(_)
            | Violation::SpanInvertedRange { .. }
            | Violation::SpanOutOfRange { .. }
            | Violation::RegexInvalid { .. }
            | Violation::RegexNoMatch { .. }
            | Violation::OverlappingSpans { .. }
    )
}

/// Record for an NS batch whose cost/apply was skipped — either because
/// span validation flagged it (render would panic) or content resolution
/// failed. Cumulative-tokens carries forward unchanged.
fn skipped_batch(
    ns_batch: &crate::north_star::NsBatch,
    cumulative: usize,
    violations: Vec<Violation>,
) -> SimulatedBatch {
    SimulatedBatch {
        id: ns_batch.id.clone(),
        descriptor: ns_batch.descriptor.clone(),
        marginal_cost: Cost::default(),
        cumulative_tokens: cumulative,
        predecessor: ns_batch.predecessor.clone(),
        violations,
    }
}

fn validate_spans(spans: &[Span], fixture_root: &Path, cache: &SourceCache) -> Vec<Violation> {
    let mut out = Vec::new();
    let mut compiled: HashMap<String, regex::Regex> = HashMap::new();
    let mut seen_lines: HashSet<(PathBuf, usize)> = HashSet::new();
    for span in spans {
        let abs = fixture_root.join(&span.path);
        let Some(source) = cache.get(&abs) else {
            out.push(Violation::SpanFileMissing(span.path.clone()));
            continue;
        };
        if span.start == 0 || span.start > span.end {
            out.push(Violation::SpanInvertedRange {
                path: span.path.clone(),
                start: span.start,
                end: span.end,
            });
            continue;
        }
        let src_lines: Vec<&str> = source.lines().collect();
        if span.end > src_lines.len() {
            out.push(Violation::SpanOutOfRange {
                path: span.path.clone(),
                start: span.start,
                end: span.end,
                file_lines: src_lines.len(),
            });
            continue;
        }
        for ln in span.start..=span.end {
            if !seen_lines.insert((span.path.clone(), ln)) {
                out.push(Violation::OverlappingSpans {
                    path: span.path.clone(),
                    line: ln,
                });
            }
        }
        if let Render::Truncated { pattern } = &span.render {
            if !compiled.contains_key(pattern) {
                match regex::Regex::new(pattern) {
                    Ok(re) => {
                        compiled.insert(pattern.clone(), re);
                    }
                    Err(_) => {
                        out.push(Violation::RegexInvalid {
                            path: span.path.clone(),
                            start: span.start,
                            end: span.end,
                            pattern: pattern.clone(),
                        });
                        continue;
                    }
                }
            }
            let re = &compiled[pattern];
            for ln in span.start..=span.end {
                let line = src_lines[ln - 1];
                let m = re.find(line);
                let Some(m) = m.filter(|m| !m.as_str().is_empty()) else {
                    out.push(Violation::RegexNoMatch {
                        path: span.path.clone(),
                        line: ln,
                        pattern: pattern.clone(),
                    });
                    continue;
                };
                let full_tokens = crate::tokenizer::count(line);
                let truncated_tokens = crate::tokenizer::count(&format!("{}…", m.as_str()));
                if truncated_tokens >= full_tokens {
                    out.push(Violation::TruncationSavesNothing {
                        path: span.path.clone(),
                        line: ln,
                        pattern: pattern.clone(),
                        full_tokens,
                        truncated_tokens,
                    });
                }
            }
        }
    }
    out
}

fn collect_ancestors(
    ns_id: &str,
    all_batches: &[crate::north_star::NsBatch],
    ns_id_to_batch_id: &HashMap<String, BatchId>,
) -> HashSet<BatchId> {
    let mut out = HashSet::new();
    let mut current = ns_id.to_string();
    loop {
        let Some(batch) = all_batches.iter().find(|b| b.id == current) else {
            break;
        };
        let Some(pred_id) = &batch.predecessor else {
            break;
        };
        let Some(&pred_batch_id) = ns_id_to_batch_id.get(pred_id) else {
            break;
        };
        if !out.insert(pred_batch_id) {
            break;
        }
        current = pred_id.clone();
    }
    out
}
