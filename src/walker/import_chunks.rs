use tree_sitter::Node;

use super::{FileLines, dedup_sorted, extend_span};

/// Only split large re-export walls: about a screenful of public-surface
/// lines and at least several source modules. Smaller entrypoints keep the
/// old single import batch to avoid needless scheduler churn.
pub(crate) const REEXPORT_IMPORT_MIN_LINES: usize = 30;
pub(crate) const REEXPORT_IMPORT_MIN_SOURCES: usize = 5;

/// A "mostly re-export wall" may have a small amount of local metadata
/// next to the wall. More than this is treated as implementation shape and
/// left as the ordinary single import batch.
pub(crate) const REEXPORT_IMPORT_MAX_OTHER_STATEMENTS: usize = 3;
pub(crate) const REEXPORT_IMPORT_MAX_OTHER_LINES: usize = 4;

#[derive(Debug)]
pub(crate) struct ImportGroup {
    pub source: String,
    pub lines: Vec<usize>,
    /// True only for real imports/re-exports. Docstrings, directives, and
    /// dunder metadata can ride along, but do not make a file a broad wall.
    pub counts_as_source: bool,
}

pub(crate) fn push_import_group(
    groups: &mut Vec<ImportGroup>,
    source_key: String,
    counts_as_source: bool,
    node: Node,
    source: &str,
) {
    let lines = groups
        .last_mut()
        .filter(|group| group.source == source_key && group.counts_as_source == counts_as_source)
        .map(|group| &mut group.lines);
    if let Some(lines) = lines {
        extend_span(lines, node, source);
    } else {
        let mut lines = Vec::new();
        extend_span(&mut lines, node, source);
        groups.push(ImportGroup {
            source: source_key,
            lines,
            counts_as_source,
        });
    }
}

pub(crate) fn should_chunk_import_groups(groups: &[ImportGroup]) -> bool {
    if groups.len() <= 1 {
        return false;
    }
    let line_count: usize = groups.iter().map(|group| group.lines.len()).sum();
    let mut sources = Vec::new();
    for group in groups.iter().filter(|group| group.counts_as_source) {
        if !sources
            .iter()
            .any(|source| source == &group.source.as_str())
        {
            sources.push(group.source.as_str());
        }
    }
    line_count >= REEXPORT_IMPORT_MIN_LINES && sources.len() >= REEXPORT_IMPORT_MIN_SOURCES
}

pub(crate) fn groups_to_file_lines(groups: Vec<ImportGroup>) -> Vec<FileLines> {
    groups
        .into_iter()
        .map(|group| FileLines::new(dedup_sorted(group.lines)))
        .collect()
}

pub(crate) fn node_line_count(node: Node) -> usize {
    node.end_position()
        .row
        .saturating_sub(node.start_position().row)
        + 1
}
