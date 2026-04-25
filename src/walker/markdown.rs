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
use crate::value::depth_factor;

use super::{Candidate, FileLines, WalkCtx, fs::files_with_extension, single_file_lines_batch};

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
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
        if name.eq_ignore_ascii_case("SUMMARY.md") {
            out.push(candidate(
                MarkdownKey::SummaryWhole { file: file.clone() },
                summary_signals(&file, ctx),
                200,
            ));
            continue;
        }

        let Some(section_count) = section_count_for(ctx, &file) else {
            continue;
        };

        let is_readme = is_readme(&file);
        let predecessor = if is_readme {
            let headline = MarkdownKey::ReadmeHeadline { file: file.clone() };
            out.push(candidate(
                headline.clone(),
                readme_headline_signals(&file, ctx),
                60,
            ));
            Some(BatchKey::Markdown(headline))
        } else {
            None
        };

        for idx in 0..section_count {
            let signals = if is_readme {
                readme_section_signals(&file, ctx)
            } else {
                heading_slab_signals(&file, idx, ctx)
            };
            let cost = if is_readme { 100 } else { 80 };
            let mut cand = candidate(
                MarkdownKey::Section {
                    file: file.clone(),
                    section_index: idx,
                },
                signals,
                cost,
            );
            if let Some(p) = predecessor.clone() {
                cand = cand.with_predecessor(p);
            }
            out.push(cand);
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

fn candidate(mk: MarkdownKey, signals: ValueSignals, cost_hint: usize) -> Candidate<BatchKey> {
    Candidate::new(mk.into(), signals, cost_hint)
}

// --- signals ---

fn signal_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    depth_factor(ctx.depth_from_root(file)) * ctx.non_essential_factor(file)
}

fn summary_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.9,
        follow_up_minimization: 0.8,
        zero_tool_call_understanding: 0.7,
        depth_factor: signal_factor(file, ctx),
    }
}

fn readme_headline_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.9,
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.8,
        depth_factor: signal_factor(file, ctx),
    }
}

fn readme_section_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    // Match the former monolithic ReadmeBody's catastrophic weight — a
    // single section is still a piece of README body; it just fits more
    // often when split. Follow-up/zero-call slightly reduced because a
    // single section alone answers fewer potential questions than the
    // whole body would.
    ValueSignals {
        catastrophic_omission: 0.55,
        follow_up_minimization: 0.8,
        zero_tool_call_understanding: 0.7,
        depth_factor: signal_factor(file, ctx),
    }
}

fn heading_slab_signals(file: &Path, section_index: usize, ctx: &WalkCtx) -> ValueSignals {
    let is_guide = file.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
        matches!(
            n.to_ascii_uppercase().as_str(),
            "CHANGELOG.MD" | "CONTRIBUTING.MD" | "CHANGES.MD"
        )
    });
    // Changelogs are conventionally sorted newest-first, so later
    // sections are ancient release notes of decreasing relevance. Apply
    // an index-based decay only to guide-shape files; for general docs
    // the section order doesn't imply relevance. Floored at 0.35 so a
    // deep section can still fire if budget permits, just not displace
    // higher-tier content.
    let index_decay = if is_guide {
        ((section_index as f64 + 1.0).powf(-0.3)).max(0.35)
    } else {
        1.0
    };
    ValueSignals {
        catastrophic_omission: if is_guide { 0.5 } else { 0.3 } * index_decay,
        follow_up_minimization: 0.5 * index_decay,
        zero_tool_call_understanding: 0.5 * index_decay,
        depth_factor: signal_factor(file, ctx),
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
        summary_signals(file, ctx),
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
        readme_headline_signals(file, ctx),
    )
}

fn mat_section(file: &Path, section_index: usize, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let (source, tree) = parse_md(ctx, file)?;
    let (start, end) = nth_section_range(&tree, &source, section_index)?;

    // For README section 0, exclude lines already covered by
    // `ReadmeHeadline` (heading + first paragraph). Otherwise the two
    // batches overlap exactly on short READMEs, Section 0's marginal
    // cost drops to zero after dedupe, and `ratio(value, 0) = INFINITY`
    // gives it unconditional scheduling priority — a smell even though
    // the duplicate apply is a no-op.
    let effective_start = if section_index == 0
        && is_readme(file)
        && let Some((_, _, para_end)) = first_section_headline(&tree, &source)
    {
        (para_end + 1).max(start)
    } else {
        start
    };
    if effective_start > end {
        return None;
    }

    let lines: Vec<usize> = (effective_start..=end).collect();
    let signals = if is_readme(file) {
        readme_section_signals(file, ctx)
    } else {
        heading_slab_signals(file, section_index, ctx)
    };
    single_file_lines_batch(file, &source, FileLines::new(lines), signals)
}

fn is_readme(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("README.md"))
}

// --- tree-sitter-md helpers ---

fn nth_section_range(tree: &Tree, source: &str, n: usize) -> Option<(usize, usize)> {
    logical_sections(tree.root_node(), source)
        .into_iter()
        .nth(n)
}

fn section_count_for(ctx: &WalkCtx, file: &Path) -> Option<usize> {
    let (source, tree) = parse_md(ctx, file)?;
    Some(logical_sections(tree.root_node(), &source).len())
}

/// Section ranges (1-based start, 1-based end inclusive) for batching.
///
/// Top-level sections by default. Special case: if the doc has exactly
/// one top-level section *and* it's an H1 (i.e. `# Title` wrapping
/// everything), descend into its H2 children so the agent gets
/// per-H2 batches instead of one multi-KB blob; the H1's prelude before
/// the first H2 becomes a synthesized intro section #0. Without this,
/// READMEs styled `# Title` (mitt, mdbook, otree) collapse into one
/// section that rarely fits at the user's budget.
fn logical_sections(root: Node, source: &str) -> Vec<(usize, usize)> {
    let top: Vec<Node> = headed_sections(root).collect();
    if top.len() == 1
        && let Some(heading) = first_heading_child(top[0])
        && heading_level(heading) == 1
    {
        let h1 = top[0];
        let h2s: Vec<Node> = headed_sections(h1).collect();
        if !h2s.is_empty() {
            let intro_start = h1.start_position().row + 1;
            let intro_end = h2s[0].start_position().row; // 1-based row before first H2
            let mut out = Vec::with_capacity(h2s.len() + 1);
            if intro_end >= intro_start {
                out.push((intro_start, intro_end));
            }
            for h2 in h2s {
                out.push((h2.start_position().row + 1, span_last_row(h2, source) + 1));
            }
            return out;
        }
    }
    top.into_iter()
        .map(|s| (s.start_position().row + 1, span_last_row(s, source) + 1))
        .collect()
}

/// 1-based level of an `atx_heading` / `setext_heading` (`# → 1`,
/// `## → 2`, …). Returns 0 if no marker child is recognized.
fn heading_level(heading: Node) -> usize {
    let mut cur = heading.walk();
    for c in heading.children(&mut cur) {
        match c.kind() {
            "atx_h1_marker" | "setext_h1_underline" => return 1,
            "atx_h2_marker" | "setext_h2_underline" => return 2,
            "atx_h3_marker" => return 3,
            "atx_h4_marker" => return 4,
            "atx_h5_marker" => return 5,
            "atx_h6_marker" => return 6,
            _ => {}
        }
    }
    0
}

fn first_section_headline(tree: &Tree, source: &str) -> Option<(usize, usize, usize)> {
    let section = headed_sections(tree.root_node()).next()?;
    let heading = first_heading_child(section)?;
    let heading_start = heading.start_position().row + 1;
    let heading_end = span_last_row(heading, source) + 1;
    let paragraph_end = first_paragraph_after(section, heading)
        .map(|p| span_last_row(p, source) + 1)
        .unwrap_or(heading_end);
    Some((heading_start, heading_end, paragraph_end))
}

/// Top-level `section` children of `node` that have a heading. Tree-sitter-md
/// wraps a leading `html_block` (or other heading-less prelude) in its own
/// `section` node — those don't represent a navigable doc section, so we
/// skip them everywhere section indices are counted or addressed.
fn headed_sections(node: Node<'_>) -> impl Iterator<Item = Node<'_>> {
    let mut cursor = node.walk();
    let children: Vec<Node> = node.children(&mut cursor).collect();
    children
        .into_iter()
        .filter(|c| c.kind() == "section" && first_heading_child(*c).is_some())
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
