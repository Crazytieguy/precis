//! NS simulator. Drives a [`RenderedTree`] in NS rank order — the same
//! machinery the scheduler uses, just picking batches off a pre-ranked list
//! instead of greedily. Produces per-batch marginal cost + cumulative +
//! violation data for `validate-ns`.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::Result;

use crate::batch::{Batch, BatchId, BatchKey, FsKey, ValueSignals};
use crate::render::{Cost, RenderedTree, SourceCache};
use crate::schema::{NorthStar, resolve_content};

/// Per-batch simulator output.
#[derive(Debug, Clone)]
pub struct SimulatedBatch {
    pub id: String,
    pub descriptor: String,
    pub marginal_cost: Cost,
    pub cumulative_tokens: usize,
    /// Parent NS-id this batch declared as predecessor (copied verbatim from
    /// the NS). None when unset. The simulator verifies closure separately.
    pub predecessor: Option<String>,
    pub violations: Vec<Violation>,
}

#[derive(Debug, Clone)]
pub enum Violation {
    /// Declared predecessor id doesn't exist or isn't ranked earlier.
    PredecessorMissing(String),
    /// `cost > 2 × largest_preceding`.
    TwoXRule { largest_preceding: usize },
    /// Cumulative cost exceeds the 10k cap after this batch.
    CapExceeded { cumulative: usize, cap: usize },
    /// Underlying content resolution failed (file missing, regex bad, etc.).
    ResolveFailed(String),
}

/// Result of simulating an NS end-to-end.
#[derive(Debug, Clone)]
pub struct SimulationReport {
    pub batches: Vec<SimulatedBatch>,
    pub total_tokens: usize,
    pub largest_batch: Option<(String, usize)>,
}

const TOKEN_CAP: usize = 10_000;

/// Simulate applying an NS's batches to a fresh `RenderedTree` in rank
/// order. Returns per-batch cost + collected violations. Does not read the
/// walker — NS content resolves via [`resolve_content`] (which reuses the
/// walker's `list_dir` for filesystem groups).
pub fn simulate_ns(ns: &NorthStar, fixture_root: &Path) -> Result<SimulationReport> {
    let source_cache = SourceCache::new();
    let mut tree = RenderedTree::new(fixture_root.to_path_buf(), source_cache);

    let mut ns_id_to_batch_id: HashMap<String, BatchId> = HashMap::new();
    let mut scheduled: HashSet<BatchId> = HashSet::new();
    let mut cumulative: usize = 0;
    let mut largest_preceding: usize = 0;
    let mut largest: Option<(String, usize)> = None;
    let mut batches_out: Vec<SimulatedBatch> = Vec::with_capacity(ns.batches.len());

    for (pos, ns_batch) in ns.batches.iter().enumerate() {
        let mut violations = Vec::new();

        // Predecessor closure: predecessor id must refer to a batch ranked
        // earlier (i.e. already assigned a BatchId).
        let predecessor_bk = match &ns_batch.predecessor {
            None => None,
            Some(pred_id) => match ns_id_to_batch_id.get(pred_id) {
                Some(_) => Some(synthetic_key(pred_id)),
                None => {
                    violations.push(Violation::PredecessorMissing(pred_id.clone()));
                    None
                }
            },
        };

        let content = match resolve_content(&ns_batch.content, fixture_root) {
            Ok(c) => c,
            Err(e) => {
                violations.push(Violation::ResolveFailed(e.to_string()));
                // Skip applying — we can't cost or schedule it.
                batches_out.push(SimulatedBatch {
                    id: ns_batch.id.clone(),
                    descriptor: ns_batch.descriptor.clone(),
                    marginal_cost: Cost::default(),
                    cumulative_tokens: cumulative,
                    predecessor: ns_batch.predecessor.clone(),
                    violations,
                });
                continue;
            }
        };

        let batch_id = BatchId::new(pos);
        let batch = Batch {
            key: synthetic_key(&ns_batch.id),
            content,
            predecessor: predecessor_bk,
            signals: ValueSignals::default(),
        };

        let cost = tree.marginal_cost(&batch);
        cumulative = cumulative.saturating_add(cost.tokens);

        if largest_preceding > 0 && cost.tokens > 2 * largest_preceding {
            violations.push(Violation::TwoXRule { largest_preceding });
        }
        if cumulative > TOKEN_CAP {
            violations.push(Violation::CapExceeded {
                cumulative,
                cap: TOKEN_CAP,
            });
        }

        let ancestors = collect_ancestors(&ns_batch.id, &ns.batches, &ns_id_to_batch_id);
        tree.apply(&batch, batch_id, |id| ancestors.contains(&id));
        scheduled.insert(batch_id);
        ns_id_to_batch_id.insert(ns_batch.id.clone(), batch_id);

        if cost.tokens > largest_preceding {
            largest_preceding = cost.tokens;
        }
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

/// Build a synthetic `BatchKey` to carry through the simulator. The
/// simulator doesn't need walker-ontology keys; it just needs per-batch
/// unique `BatchKey` values so tree internal bookkeeping has *something*
/// to slot in. The real distinguisher is the `BatchId` assigned at apply
/// time.
fn synthetic_key(ns_id: &str) -> BatchKey {
    BatchKey::Fs(FsKey::DirListing {
        dir: std::path::PathBuf::from(format!("/ns/{ns_id}")),
    })
}

/// Walk the NS-predecessor chain from `ns_id` upward, mapping each NS-id to
/// the assigned `BatchId`. Used to tell the tree's `is_ancestor` predicate
/// which scheduled batches are legitimate ancestors of the one being applied.
fn collect_ancestors(
    ns_id: &str,
    all_batches: &[crate::schema::NsBatch],
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
