//! Schedule artifact types. `render_schedule` produces a `Schedule`; the
//! divergence test and the regression snapshot both consume it.
//!
//! Mirrors [`ScheduledBatchRecord`] (from the scheduler) into a
//! serde-friendly shape with stringified keys + walker-emitted descriptors
//! for readable diffs. `Schedule` serializes deterministically to TOML for
//! regression snapshots.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::content::BatchContent;

/// Complete scheduling result for one `render_schedule(fixture, budget)`
/// invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub fixture: String,
    pub budget: usize,
    pub cumulative_tokens: usize,
    pub batch_count: usize,
    pub batches: Vec<ScheduledBatch>,
}

/// One scheduled batch's public-facing record. `content` is the resolved
/// library `BatchContent` (filesystem groups carry explicit children; line
/// batches carry their spans verbatim).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledBatch {
    pub position: usize,
    pub key: String,
    pub descriptor: String,
    pub cost_tokens: usize,
    pub cum_tokens: usize,
    pub content: BatchContent,
}

/// Root-relative atom — identity only. Paired with a per-render
/// `byte_end` inside the divergence metric for credit computation, so the
/// same (path, line) atom from walker + NS compare byte-ranges rather
/// than being treated as different identities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Atom {
    Line { path: PathBuf, line: usize },
    Fs { parent: PathBuf, entry: String },
}
