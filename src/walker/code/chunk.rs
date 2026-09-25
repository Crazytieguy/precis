//! Splitting an oversize part (a list of [`Item`]s) into chained chunks,
//! and pricing each chunk as a share of the unsplit part.
//!
//! Cost is a source-token proxy (the tokens of the part's rows), not the
//! scheduler's rendered marginal cost, which also counts gutters and gap
//! markers and zeroes rows an ancestor already rendered. Chunk ranking
//! parity with the unsplit part is therefore approximate.

use std::ops::Range;

use super::SourceFile;
use super::model::Item;
use crate::value::DEFAULT_CONCAVITY_EXPONENT;
use crate::walker::budget_chunk_ranges;

/// A part costing more tokens than this is split.
pub(super) const SPLIT_AT: usize = 300;
/// Token size a chunk grows to before the next item starts a new one.
pub(super) const TARGET: usize = 120;
/// A final chunk cheaper than this folds into the one before it.
pub(super) const MIN_TAIL: usize = 75;

/// Token cost of one item's rows.
pub(super) fn item_cost(item: &Item, file: &SourceFile) -> usize {
    item.rows
        .iter()
        .map(|&row| crate::tokenizer::count(file.line(row)))
        .sum()
}

/// Consecutive ranges of item indices, one per chunk, never splitting an
/// item: one range when the part costs at most [`SPLIT_AT`], otherwise cut
/// at [`TARGET`] / [`MIN_TAIL`]. Empty for an empty part.
pub(in crate::walker) fn chunk_ranges(item_costs: &[usize]) -> Vec<Range<usize>> {
    let mut prefix = Vec::with_capacity(item_costs.len() + 1);
    prefix.push(0);
    for cost in item_costs {
        prefix.push(prefix[prefix.len() - 1] + cost);
    }
    let target = if prefix[item_costs.len()] > SPLIT_AT {
        TARGET
    } else {
        usize::MAX
    };
    budget_chunk_ranges(
        item_costs.len(),
        |range| prefix[range.end] - prefix[range.start],
        target,
        MIN_TAIL,
        |_| true,
    )
}

/// Value multiplier for a chunk costing `chunk_cost` of its part's
/// `part_cost`: `share^k` with `k` the default concavity exponent, so
/// each chunk ranks like the unsplit part.
pub(super) fn chunk_value_factor(chunk_cost: usize, part_cost: usize) -> f64 {
    if chunk_cost == part_cost {
        return 1.0;
    }
    (chunk_cost as f64 / part_cost as f64).powf(DEFAULT_CONCAVITY_EXPONENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_ranges_split_only_oversize_parts() {
        assert!(chunk_ranges(&[]).is_empty());
        assert_eq!(chunk_ranges(&[100, 100, 100]).len(), 1);
        assert_eq!(chunk_ranges(&[100, 100, 100, 100]), [0..2, 2..4]);
        assert_eq!(chunk_ranges(&[160, 160, 20]), [0..1, 1..3]);
    }
}
