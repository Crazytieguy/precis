//! North Star document types. A frozen NS is the ground-truth ranking
//! that the walker is judged against via the divergence metric.
//!
//! **Public schema surface** — these are the types an NS-author agent
//! (or any external TOML producer) needs to understand:
//! - `NorthStar` / `NsBatch` (this file) — document + per-batch shape.
//! - [`crate::content`] — `BatchContent`, `FsGroup`, `FsEntries`, `Span`,
//!   `Render`. Shared vocabulary with the walker/render pipeline.
//!
//! Loading + resolution (expanding `FsEntries::All` to `Listed`, checking
//! the fixture's revision pin) lives in [`crate::ns_loader`].

use serde::Deserialize;

use crate::content::BatchContent;

/// A frozen North Star document for one fixture: ranked batches with
/// declarative spans + render specs.
#[derive(Debug, Clone, Deserialize)]
pub struct NorthStar {
    /// Fixture directory name under `tests/fixtures/`, e.g. `"log"`.
    pub fixture: String,
    /// Fixture revision this NS was authored against. Must equal
    /// `<fixture_root>/.precis-pin` at load time; mismatch is a hard
    /// error (the NS's spans could be off-by-N against a re-pinned
    /// fixture).
    pub revision_pin: String,
    /// Free-form prose describing the crate's concept + the query
    /// distribution the ranking serves. Opaque to tooling.
    #[serde(default)]
    pub summary: String,
    /// Ranked batches. Position = rank. IDs are major.minor strings
    /// (`"1.1"`, `"2.10"`) with numeric-aware sort for diff stability.
    pub batches: Vec<NsBatch>,
}

/// One ranked batch in a North Star.
#[derive(Debug, Clone, Deserialize)]
pub struct NsBatch {
    /// Major.minor id (`"1.1"`, `"2.10"`). Numeric-sortable per `.`-
    /// separated component; don't mix with pure-numeric or string ids.
    pub id: String,
    /// Short human-readable label, e.g. `"Crate-doc lede"`.
    pub descriptor: String,
    /// Free-form prose on why this batch is ranked here / what query
    /// distribution it serves. Opaque to tooling; helpful in diffs.
    pub justification: String,
    /// ID of an earlier-ranked batch this one depends on (e.g. a fn
    /// body after its signature, or a refinement that overrides the
    /// parent's ellipsis lines with real content). A batch may overlap
    /// another's rendered lines only if one is the transitive predecessor
    /// of the other. The simulator validates ancestor closure.
    #[serde(default)]
    pub predecessor: Option<String>,
    /// The batch's content: filesystem listings (possibly with
    /// `FsEntries::All` sentinel expanded at load time) or
    /// source line spans with render specs.
    pub content: BatchContent,
}
