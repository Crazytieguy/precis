//! North Star document types. A frozen NS is the ground-truth ranking
//! that the walker is judged against via the divergence metric.
//!
//! **Public schema surface** — these are the types an NS-author agent
//! (or any external TOML producer) needs to understand. The rest of the
//! schema vocabulary lives adjacent to the walker/render types it reuses:
//!
//! - [`crate::batch::BatchContent`] — `Fs { groups }` | `Lines { spans }`
//! - [`crate::batch::FsGroup`] + [`crate::batch::FsEntries`] — filesystem
//!   listings with the `All` / `Names(...)` / `Listed(...)` input shapes
//! - [`crate::batch::Span`] — a line range with a render spec
//! - [`crate::batch::Render`] — `Full` | `Truncated { pattern }` | `Ellipsis`
//! - [`crate::batch::EntryKind`] — `File` | `Dir`
//!
//! Loading + resolution (expanding `FsEntries::All`/`Names` to `Listed`,
//! checking the fixture's revision pin) lives in [`crate::ns_loader`].

use serde::{Deserialize, Serialize};

use crate::batch::BatchContent;

/// A frozen North Star document for one fixture: ranked batches with
/// declarative spans + render specs.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor: Option<String>,
    /// The batch's content: filesystem listings (possibly with
    /// `FsEntries::All` / `Names` sentinels resolved at load time) or
    /// source line spans with render specs.
    pub content: BatchContent,
}
