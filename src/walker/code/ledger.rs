//! Per-file row ownership for the engine's batches.
//!
//! The invariant is the one `RenderedTree::apply_spans` enforces: a batch
//! may write a row only if the row is unowned or owned by one of the
//! batch's ancestors (its predecessor chain), and writing it makes the
//! batch its owner. The ledger applies it at emission, so an extraction
//! quirk drops a row instead of tripping the scheduler.

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
    /// No kept row is new: every one is already rendered by an ancestor,
    /// so the batch adds nothing and is not emitted; its descendants gate
    /// on its predecessor instead.
    pub covered: bool,
}

impl Ledger {
    /// Claims `rows` (sorted, deduplicated) for `key`, whose predecessor is
    /// `parent`. Rows a non-ancestor owns are dropped and counted. A
    /// covered claim records nothing.
    pub(super) fn claim(
        &mut self,
        key: &CodeKey,
        parent: Option<&CodeKey>,
        rows: &[usize],
    ) -> Claim {
        let mut kept = Vec::with_capacity(rows.len());
        let mut adds_row = false;
        for &row in rows {
            match self.owners.get(&row) {
                None => {
                    adds_row = true;
                    kept.push(row);
                }
                Some(owner) if self.is_ancestor(owner, parent) => kept.push(row),
                Some(_) => self.dropped_rows += 1,
            }
        }
        if adds_row {
            for &row in &kept {
                self.owners.insert(row, key.clone());
            }
            self.parents.insert(key.clone(), parent.cloned());
        }
        Claim {
            rows: kept,
            covered: !adds_row,
        }
    }

    /// Whether `candidate` is `parent` or one of its predecessors.
    fn is_ancestor(&self, candidate: &CodeKey, parent: Option<&CodeKey>) -> bool {
        let mut cursor = parent;
        while let Some(key) = cursor {
            if key == candidate {
                return true;
            }
            cursor = self.parents.get(key).and_then(Option::as_ref);
        }
        false
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::Rung;
    use std::path::PathBuf;

    fn key(rung: Rung, decl: u32) -> CodeKey {
        CodeKey {
            rung,
            file: PathBuf::from("a.lua"),
            decl,
            sub: 0,
            line: 0,
        }
    }

    #[test]
    fn ledger_keeps_ancestor_rows_and_drops_non_ancestor_rows() {
        let mut ledger = Ledger::default();
        let names = key(Rung::Names, 0);
        let first = key(Rung::Decl, 1);
        let second = key(Rung::Decl, 2);
        assert!(!ledger.claim(&names, None, &[1, 5]).covered);
        let claim = ledger.claim(&first, Some(&names), &[1, 2, 3]);
        assert_eq!((claim.rows, claim.covered), (vec![1, 2, 3], false));
        assert_eq!(ledger.owner(1), Some(&first));
        assert_eq!(ledger.dropped_rows(), 0);

        let claim = ledger.claim(&second, Some(&names), &[3, 5, 6]);
        assert_eq!(claim.rows, [5, 6]);
        assert_eq!(ledger.dropped_rows(), 1);
    }

    #[test]
    fn ledger_covered_claim_records_nothing() {
        let mut ledger = Ledger::default();
        let names = key(Rung::Names, 0);
        let decl = key(Rung::Decl, 1);
        ledger.claim(&names, None, &[1]);
        let claim = ledger.claim(&decl, Some(&names), &[1]);
        assert!(claim.covered);
        assert_eq!(ledger.owner(1), Some(&names));
    }
}
