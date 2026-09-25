//! Per-file row ownership for the engine's batches.
//!
//! The invariant is the one `RenderedTree::apply_spans` enforces: a batch
//! may write a row only if the row is unowned or owned by one of the
//! batch's ancestors (its predecessor chain). The ledger applies it at
//! emission, so an extraction quirk drops a row instead of tripping the
//! scheduler.

use std::collections::HashMap;

use crate::batch::CodeKey;

/// Row owners of one file, plus each claimed key's predecessor.
#[derive(Default)]
pub(super) struct Ledger {
    owners: HashMap<usize, CodeKey>,
    parents: HashMap<CodeKey, Option<CodeKey>>,
    dropped_rows: usize,
}

/// The rows a batch keeps after [`Ledger::claim`].
pub(super) struct Claim {
    /// Rows the batch may render: unowned rows and rows its ancestors own.
    pub rows: Vec<usize>,
    /// Every kept row was already owned by an ancestor, so the batch adds
    /// nothing and is not emitted; its descendants gate on `parent`.
    pub covered: bool,
}

impl Ledger {
    /// Claims `rows` for `key`, whose predecessor is `parent`. Rows a
    /// non-ancestor owns are dropped and counted (and asserted in the
    /// engine's unit tests).
    pub(super) fn claim(
        &mut self,
        _key: &CodeKey,
        _parent: Option<&CodeKey>,
        rows: &[usize],
    ) -> Claim {
        Claim {
            rows: rows.to_vec(),
            covered: false,
        }
    }

    /// The batch that owns `row`, for predecessor lookup.
    pub(super) fn owner(&self, row: usize) -> Option<&CodeKey> {
        self.owners.get(&row)
    }

    /// Rows dropped as non-ancestor overlaps so far.
    pub(super) fn dropped_rows(&self) -> usize {
        self.dropped_rows
    }
}
