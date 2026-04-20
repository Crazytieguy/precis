//! Markdown content walker. `SUMMARY.md` → whole-file batch (book-style index).
//! Any other markdown → first heading + body slab. Detects ATX (`# Title`) and
//! Setext (`Title\n===`) headings; line-scan only.

use std::path::Path;

use crate::batch::{BatchDraft, RenderedLine};

use super::lines_for_file;

pub fn emit_batches(path: &Path, source: &str) -> Vec<BatchDraft> {
    let is_named = |target: &str| {
        path.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.eq_ignore_ascii_case(target))
    };

    if is_named("SUMMARY.md") {
        return whole_file(path, source, 800.0).into_iter().collect();
    }

    let Some((start, end)) = first_heading_slab(source) else {
        return Vec::new();
    };
    let value = if is_named("README.md") { 800.0 } else { 400.0 };
    lines_in_range(path, source, start..=end, value)
        .into_iter()
        .collect()
}

fn whole_file(path: &Path, source: &str, value: f64) -> Option<BatchDraft> {
    lines_in_range(path, source, 1..=usize::MAX, value)
}

fn lines_in_range(
    path: &Path,
    source: &str,
    range: std::ops::RangeInclusive<usize>,
    value: f64,
) -> Option<BatchDraft> {
    let lines = source
        .lines()
        .enumerate()
        .map(|(i, t)| (i + 1, t))
        .filter(|(n, t)| range.contains(n) && !t.trim().is_empty())
        .map(|(n, t)| (n, RenderedLine::Full(t.to_string())));
    lines_for_file(path, lines, value)
}

/// Find the line range (1-indexed, inclusive) of the first heading and its
/// following body. Returns `None` when the file has no recognizable heading.
/// The slab ends at the next heading of equal-or-higher rank, or end-of-file.
fn first_heading_slab(source: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = source.lines().collect();
    let (start_idx, start_rank) = first_heading(&lines)?;
    let end_idx = (start_idx + 1..lines.len())
        .find(|i| heading_rank_at(&lines, *i).is_some_and(|rank| rank <= start_rank))
        .map(|i| i - 1)
        .unwrap_or(lines.len() - 1);
    Some((start_idx + 1, end_idx + 1))
}

fn first_heading(lines: &[&str]) -> Option<(usize, usize)> {
    (0..lines.len()).find_map(|i| heading_rank_at(lines, i).map(|r| (i, r)))
}

/// ATX (`# Title`) returns rank 1–6. Setext (`Title\n===` or `Title\n---`)
/// returns rank 1 or 2 respectively, attributed to the title line (i).
fn heading_rank_at(lines: &[&str], i: usize) -> Option<usize> {
    let line = *lines.get(i)?;
    if let Some(rank) = atx_rank(line) {
        return Some(rank);
    }
    let next = lines.get(i + 1)?;
    if !line.trim().is_empty() && setext_marker_rank(next).is_some() {
        return setext_marker_rank(next);
    }
    None
}

fn atx_rank(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let rank = trimmed.bytes().take_while(|&b| b == b'#').count();
    let after = trimmed.as_bytes().get(rank).copied();
    (rank > 0 && rank <= 6 && matches!(after, Some(b' ') | Some(b'\t'))).then_some(rank)
}

fn setext_marker_rank(line: &str) -> Option<usize> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }
    if s.bytes().all(|b| b == b'=') {
        Some(1)
    } else if s.bytes().all(|b| b == b'-') {
        Some(2)
    } else {
        None
    }
}
