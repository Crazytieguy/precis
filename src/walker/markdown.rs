//! Markdown content walker. First-pass Tier 1 scope:
//! - `SUMMARY.md`: emit the whole file as one batch (book-style index).
//! - Any markdown file: emit the first heading + its body slab as one batch.
//!
//! Uses simple line scanning (no markdown parser) — Tier 1 only needs heading
//! detection and basename matching, both line-identifiable.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::batch::{BatchContent, BatchDraft, RenderedLine};

pub fn emit_batches(path: &Path, source: &str) -> Vec<BatchDraft> {
    let basename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if basename.eq_ignore_ascii_case("SUMMARY.md") {
        return vec![whole_file(path, source, 800.0)];
    }

    // README-style: pull the first heading + its slab (lines until the next
    // heading or a blank-line fence). Skip files whose first non-blank line
    // isn't a heading — they're prose dumps, not structured docs.
    let Some((start, end)) = first_heading_slab(source) else {
        return Vec::new();
    };
    let mut lines = BTreeMap::new();
    for (i, text) in source
        .lines()
        .enumerate()
        .skip(start - 1)
        .take(end - start + 1)
    {
        if text.trim().is_empty() {
            continue;
        }
        lines.insert(i + 1, RenderedLine::Full(text.to_string()));
    }
    let mut file_map: BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>> = BTreeMap::new();
    file_map.insert(path.to_path_buf(), lines);
    let value = if basename.eq_ignore_ascii_case("README.md") {
        800.0
    } else {
        400.0
    };
    vec![BatchDraft {
        content: BatchContent::Lines(file_map),
        value,
    }]
}

fn whole_file(path: &Path, source: &str, value: f64) -> BatchDraft {
    let mut lines = BTreeMap::new();
    for (i, text) in source.lines().enumerate() {
        if text.trim().is_empty() {
            continue;
        }
        lines.insert(i + 1, RenderedLine::Full(text.to_string()));
    }
    let mut file_map: BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>> = BTreeMap::new();
    file_map.insert(path.to_path_buf(), lines);
    BatchDraft {
        content: BatchContent::Lines(file_map),
        value,
    }
}

/// Find the line range (1-indexed, inclusive) of the first heading and its
/// following body. The slab ends at the next heading of equal-or-higher
/// rank, or at end-of-file.
fn first_heading_slab(source: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = source.lines().collect();
    let (start_idx, start_rank) = lines.iter().enumerate().find_map(|(i, l)| {
        let rank = heading_rank(l)?;
        Some((i, rank))
    })?;
    // Find next heading of rank <= start_rank (or EOF).
    let end_idx = lines
        .iter()
        .enumerate()
        .skip(start_idx + 1)
        .find_map(|(i, l)| {
            let rank = heading_rank(l)?;
            (rank <= start_rank).then_some(i)
        })
        .map(|i| i - 1)
        .unwrap_or(lines.len() - 1);
    Some((start_idx + 1, end_idx + 1))
}

fn heading_rank(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let rank = trimmed.bytes().take_while(|&b| b == b'#').count();
    if rank > 0
        && rank <= 6
        && trimmed
            .as_bytes()
            .get(rank)
            .is_some_and(|&b| b == b' ' || b == b'\t')
    {
        Some(rank)
    } else {
        None
    }
}
