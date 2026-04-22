//! Markdown walker. Uses `tree-sitter-md`'s block grammar for heading and
//! section detection.
//!
//! Keys:
//! - `SummaryWhole { file }` — `SUMMARY.md`, whole file (mdBook ToC)
//! - `ReadmeHeadline { file }` — `README.md`, first section's heading +
//!   opening paragraph
//! - `Section { file, section_index }` — one H2 section, 0-indexed. For
//!   READMEs, this is the split replacement of the old monolithic
//!   "body" batch (predecessor: headline). For other markdown files
//!   (changelogs, doc pages), we emit one per top-level section so the
//!   file can land piece-by-piece.

use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{BatchKey, FsKey, MarkdownKey, ResolvedBatch, ValueSignals};
use crate::value::{depth_factor, non_essential_factor};

use super::{Candidate, FileLines, WalkCtx, fs::files_with_extension, single_file_lines_batch};

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
        return Vec::new();
    };
    let md_files = files_with_extension(dir, "md");
    if md_files.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in md_files {
        let name = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        let depth = ctx.depth_from_root(&file);
        if name.eq_ignore_ascii_case("SUMMARY.md") {
            out.push(candidate(
                MarkdownKey::SummaryWhole { file: file.clone() },
                summary_signals(&file, depth),
                200,
            ));
            continue;
        }

        // Count top-level sections so we can emit one batch per section.
        // `None` means the file couldn't be parsed or has no sections —
        // the walker simply emits nothing in that case.
        let Some(section_count) = section_count_for(ctx, &file) else {
            continue;
        };

        if name.eq_ignore_ascii_case("README.md") {
            // README gets a cheap "headline" batch (title + opening
            // paragraph) plus one Section batch per top-level section,
            // with the headline as predecessor for ordering.
            //
            // Note: section 0 covers the *whole* first section, which
            // overlaps the headline's heading+paragraph lines. The
            // scheduler dedupes per line, so this is idempotent — the
            // section batch contributes only the body paragraphs the
            // headline didn't already render.
            let headline = MarkdownKey::ReadmeHeadline { file: file.clone() };
            out.push(candidate(
                headline.clone(),
                readme_headline_signals(&file, depth),
                60,
            ));
            for idx in 0..section_count {
                out.push(
                    candidate(
                        MarkdownKey::Section {
                            file: file.clone(),
                            section_index: idx,
                        },
                        readme_section_signals(&file, depth),
                        100,
                    )
                    .with_predecessor(BatchKey::Markdown(headline.clone())),
                );
            }
        } else {
            // Other `.md` files: emit one batch per top-level section.
            for idx in 0..section_count {
                out.push(candidate(
                    MarkdownKey::Section {
                        file: file.clone(),
                        section_index: idx,
                    },
                    heading_slab_signals(depth, &file),
                    80,
                ));
            }
        }
    }
    out
}

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Markdown(mk) = key else {
        return None;
    };
    match mk {
        MarkdownKey::SummaryWhole { file } => mat_summary(file, ctx),
        MarkdownKey::ReadmeHeadline { file } => mat_readme_headline(file, ctx),
        MarkdownKey::Section {
            file,
            section_index,
        } => mat_section(file, *section_index, ctx),
    }
}

// --- candidate helpers ---

fn candidate(mk: MarkdownKey, signals: ValueSignals, cost_hint: usize) -> Candidate {
    Candidate::new(mk.into(), signals, cost_hint)
}

// --- signals ---

fn summary_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.9,
        follow_up_minimization: 0.8,
        zero_tool_call_understanding: 0.7,
        depth_factor: depth_factor(depth) * non_essential_factor(file),
    }
}

fn readme_headline_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.9,
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.8,
        depth_factor: depth_factor(depth) * non_essential_factor(file),
    }
}

fn readme_section_signals(file: &Path, depth: usize) -> ValueSignals {
    // Match the former monolithic ReadmeBody's catastrophic weight — a
    // single section is still a piece of README body; it just fits more
    // often when split. Follow-up/zero-call slightly reduced because a
    // single section alone answers fewer potential questions than the
    // whole body would.
    ValueSignals {
        catastrophic_omission: 0.55,
        follow_up_minimization: 0.8,
        zero_tool_call_understanding: 0.7,
        depth_factor: depth_factor(depth) * non_essential_factor(file),
    }
}

fn heading_slab_signals(depth: usize, file: &Path) -> ValueSignals {
    let is_guide = file.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
        matches!(
            n.to_ascii_uppercase().as_str(),
            "CHANGELOG.MD" | "CONTRIBUTING.MD" | "CHANGES.MD"
        )
    });
    ValueSignals {
        catastrophic_omission: if is_guide { 0.5 } else { 0.3 },
        follow_up_minimization: 0.5,
        zero_tool_call_understanding: 0.5,
        depth_factor: depth_factor(depth) * non_essential_factor(file),
    }
}

// --- parser ---

fn parse_md(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_md::LANGUAGE.into())
}

// --- materializers ---

fn mat_summary(file: &Path, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let source = ctx.read_source(file)?;
    let line_count = source.lines().count();
    let lines: Vec<usize> = (1..=line_count).collect();
    single_file_lines_batch(
        file,
        &source,
        FileLines::new(lines),
        summary_signals(file, ctx.depth_from_root(file)),
    )
}

fn mat_readme_headline(file: &Path, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let (source, tree) = parse_md(ctx, file)?;
    let (heading_start, heading_end, para_end) = first_section_headline(&tree, &source)?;
    let mut lines: Vec<usize> = (heading_start..=heading_end).collect();
    lines.extend(heading_end + 1..=para_end);
    single_file_lines_batch(
        file,
        &source,
        FileLines::new(lines),
        readme_headline_signals(file, ctx.depth_from_root(file)),
    )
}

fn mat_section(file: &Path, section_index: usize, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let (source, tree) = parse_md(ctx, file)?;
    let (start, end) = nth_section_range(&tree, &source, section_index)?;
    let lines: Vec<usize> = (start..=end).collect();
    let name = file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    let signals = if name.eq_ignore_ascii_case("README.md") {
        readme_section_signals(file, ctx.depth_from_root(file))
    } else {
        heading_slab_signals(ctx.depth_from_root(file), file)
    };
    single_file_lines_batch(file, &source, FileLines::new(lines), signals)
}

// --- tree-sitter-md helpers ---

fn nth_section_range(tree: &Tree, source: &str, n: usize) -> Option<(usize, usize)> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut sections = root.children(&mut cursor).filter(|c| c.kind() == "section");
    let section = sections.nth(n)?;
    let start = section.start_position().row + 1;
    let end_row = span_last_row(section, source);
    Some((start, end_row + 1))
}

fn section_count_for(ctx: &WalkCtx, file: &Path) -> Option<usize> {
    let (_source, tree) = parse_md(ctx, file)?;
    let root = tree.root_node();
    let mut cursor = root.walk();
    let n = root
        .children(&mut cursor)
        .filter(|c| c.kind() == "section")
        .count();
    Some(n)
}

fn first_section_headline(tree: &Tree, source: &str) -> Option<(usize, usize, usize)> {
    let section = find_first_section(tree.root_node())?;
    let heading = first_heading_child(section)?;
    let heading_start = heading.start_position().row + 1;
    let heading_end = span_last_row(heading, source) + 1;
    let paragraph_end = first_paragraph_after(section, heading)
        .map(|p| span_last_row(p, source) + 1)
        .unwrap_or(heading_end);
    Some((heading_start, heading_end, paragraph_end))
}

fn find_first_section(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    node.children(&mut cursor).find(|c| c.kind() == "section")
}

fn first_heading_child(section: Node) -> Option<Node> {
    let mut cursor = section.walk();
    for child in section.children(&mut cursor) {
        if matches!(child.kind(), "atx_heading" | "setext_heading") {
            return Some(child);
        }
    }
    None
}

fn first_paragraph_after<'a>(section: Node<'a>, heading: Node<'a>) -> Option<Node<'a>> {
    let mut cursor = section.walk();
    let mut seen_heading = false;
    for child in section.children(&mut cursor) {
        if child.id() == heading.id() {
            seen_heading = true;
            continue;
        }
        if seen_heading && child.kind() == "paragraph" {
            return Some(child);
        }
    }
    None
}

/// Last source row (0-indexed) covered by `node`, trimming a trailing empty
/// line tree-sitter-md sometimes includes in a node's span.
fn span_last_row(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    let trimmed = text.trim_end_matches(['\n', '\r']);
    let internal = trimmed.split('\n').count();
    node.start_position().row + internal.saturating_sub(1)
}
