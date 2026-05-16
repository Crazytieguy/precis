//! Schedule artifact types. `render_schedule` produces a `Schedule`; the
//! divergence test and the regression snapshot both consume it.
//!
//! Mirrors [`ScheduledBatchRecord`] (from the scheduler) into a
//! serde-friendly shape with stringified keys + walker-emitted descriptors
//! for readable diffs. `Schedule` serializes deterministically to TOML for
//! regression snapshots — via [`Schedule::to_toml_normalized`], which
//! rewrites paths as relative to the fixture root so committed snapshots
//! don't bake in absolute paths.

use std::path::{Path, PathBuf};

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
    /// Full discovered candidate pool for diagnostics such as divergence
    /// coverage hints. This is intentionally kept out of schedule TOML
    /// snapshots; fixture baseline tests aggregate divergence reports from
    /// the in-memory schedules they already produce.
    #[serde(skip)]
    pub candidates: Vec<CandidateBatch>,
    /// Fixture root the scheduler was run against. In-memory paths in
    /// `batches`/`candidates` are absolute; this anchor lets
    /// `to_toml_normalized` strip the prefix at serialization time.
    #[serde(skip)]
    pub root: PathBuf,
}

impl Schedule {
    /// Serialize to TOML, rewriting paths under `self.root` as
    /// relative. Used for the regression snapshot so committed
    /// artifacts are portable across worktrees / machines.
    pub fn to_toml_normalized(&self) -> Result<String, toml::ser::Error> {
        let raw = toml::to_string(self)?;
        Ok(make_paths_relative(&raw, &self.root))
    }
}

fn make_paths_relative(s: &str, root: &Path) -> String {
    let root_str = root.to_string_lossy();
    // Order: strip "<root>/" first so the remaining bare "<root>"
    // (the fixture root itself) maps cleanly to ".".
    s.replace(&format!("{root_str}/"), "")
        .replace(root_str.as_ref(), ".")
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

/// One walker-emitted candidate, whether or not it was scheduled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateBatch {
    pub key: String,
    pub predecessor: Option<String>,
    pub descriptor: String,
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
