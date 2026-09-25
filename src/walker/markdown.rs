//! Markdown walker. Uses `tree-sitter-md`'s block grammar for heading and
//! section detection; the root `README.rst` is line-scanned into the same
//! shapes (see [`rst_readme`]).
//!
//! Per file, in document order:
//! - `ReadmeHeadline` (READMEs) — the first heading plus the lede: the
//!   first substantive block before it and the blocks under it through
//!   the first paragraph, stepping over chrome (badges, logos, nav
//!   rows).
//! - `Prelude` (READMEs) — the rest of the text above the first
//!   heading, chrome excluded. Predecessor: the headline.
//! - `HeadingsOutline` — every H1–H3 heading row the headline doesn't
//!   cover, when there are 2..=[`MAX_OUTLINE_HEADINGS`] of them.
//!   Predecessor: the headline.
//! - `Section`s — one per top-level H2 (an H1-only document unwraps to
//!   an intro plus its H2s); an oversize section splits into a head
//!   chunk plus chained `OversizeTail` chunks. Predecessor: the
//!   outline, else the headline.
//!
//! Peripheral and auto-injected docs emit only their structural
//! batches.

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, MarkdownKey};
use crate::content::{BatchContent, Render, Span};
use crate::render::Source;
use crate::tokenizer;
use crate::value::{is_peripheral_doc, mix_signals};

use super::{
    FileLines, WalkCtx, budget_chunk_ranges, first_child_of_kind, fs::files_with_extension,
    node_end_row_trimmed, path_depth_factor, single_file_lines_content,
};

/// Upper bound on collectable heading rows before `HeadingsOutline`
/// suppresses itself — the outline predecesses every section, so an
/// oversize outline would block the whole file.
const MAX_OUTLINE_HEADINGS: usize = 30;

/// Token threshold above which an otherwise-unsplit section is
/// emitted as a head chunk plus predecessor-chained tail chunks.
/// NSes are authored to a growth envelope
/// (`cost ≤ 100 + 0.3·cumulative`, see `src/ns_simulate.rs`), so an
/// early-rankable batch is ~100–400 tokens; a prose lump beyond that
/// structurally cannot win the early purchase race no matter its
/// value. The head can; the tails follow through the existing
/// predecessor/train machinery. Measured in tokens (not bytes) —
/// code-heavy sections tokenize markedly denser per byte than prose,
/// so a byte gate mis-sizes exactly the fence-rich sections this
/// split targets.
const OVERSIZE_SECTION_SPLIT_TOKENS: usize = 500;

/// Greedy per-chunk token target for the oversize head-split — inside
/// the NS early-batch envelope, low enough that a fence-heavy chunk
/// pair doesn't overshoot it before the first cut candidate.
const OVERSIZE_CHUNK_TARGET_TOKENS: usize = 300;

/// Token target for the *first* chunk of a head-split section — the
/// section lede. A section's opening paragraph is what an NS credits
/// when it wants the section's subject rather than its detail, so the
/// entry price for a section should be its lede's, not a generic
/// chunk's. Both halves price off the section's own value, undiscounted
/// — a carve that split the value by row share would make ratio scale
/// as `cost^(1-k)`, i.e. make entry strictly *worse*, which is the same
/// arithmetic that killed conservation in the Go and Python
/// entry-slice families; a 0.7 lede discount measured −0.0027 at 3000
/// here and lost both carriers.
const LEDE_TARGET_TOKENS: usize = 140;

/// Minimum tokens that must remain after a cut — a smaller remainder
/// folds into the current chunk instead of spawning a micro-tail.
const OVERSIZE_CHUNK_MIN_TAIL_TOKENS: usize = 100;

/// Emergency source-character ceiling between natural blank-line cuts.
/// Long unbroken paragraphs may have no structural split point at all;
/// safe row boundaries outside fences keep those ranges purchasable.
const OVERSIZE_HARD_CHUNK_CHAR_CAP: usize = 4_096;

/// A pathological README paragraph must not turn `ReadmeHeadline` into
/// a multi-megabyte batch. Normal ledes stay untouched; only a very
/// large block is reduced to a bounded prefix, and individually huge
/// rows in that prefix are truncated further for early purchase.
const HEADLINE_BLOCK_BYTE_GATE: usize = 16 * 1024;
const HEADLINE_OVERSIZE_LEDE_BYTES: usize = 640;
const HEADLINE_OVERSIZE_LINE_CHARS: usize = 320;

/// Source-byte ceiling on the `Prelude` batch. The hero region of a
/// normal README is well under this; the cap only stops a README that
/// puts its whole body above the first heading from shipping as one
/// unsplittable early-budget lump.
const PRELUDE_MAX_BYTES: usize = 2_500;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let md_files = files_with_extension(dir, "md", ctx);
    let sibling_md_count = md_files.len();
    let mut out = Vec::new();

    // RST README: emit a ReadmeHeadline batch plus `Section` batches
    // (see [`rst_readme`]) — parity with the .md path. No tree-sitter
    // parse — line-scan headings via `is_rst_underline`, dropping
    // `.. directive::` blocks. Skip nested README.rst — the root-level
    // file is the only anchor.
    let rst_files = if dir == ctx.root() {
        super::fs::files_with_extension(dir, "rst", ctx)
    } else {
        Vec::new()
    };
    for file in rst_files {
        if !is_readme_rst(&file) {
            continue;
        }
        let Some(source) = ctx.read_source(&file) else {
            continue;
        };
        let (headline, ranges) = rst_readme(&source);
        let mut headline_emitted: Option<BatchKey> = None;
        if let Some(content) = build_headline_content(&file, &source, &headline) {
            let key = MarkdownKey::ReadmeHeadline { file: file.clone() };
            out.push(Batch {
                key: key.clone().into(),
                predecessor: None,
                content,
                value: readme_headline_value(
                    &file,
                    ctx,
                    NavDensity {
                        sibling_md_count,
                        root_readme: true,
                    },
                ),
            });
            headline_emitted = Some(BatchKey::Markdown(key));
        }
        push_sections(
            &mut out,
            &file,
            &source,
            &ranges,
            Some(&headline),
            headline_emitted,
            ctx,
        );
    }

    if md_files.is_empty() {
        return out;
    }
    for file in md_files {
        // Auto-injected agent docs (AGENTS.md / CLAUDE.md / skill
        // files) are loaded into the model's context by the harness,
        // so emitting their bodies is pure waste; peripheral admin docs
        // (changelogs, contributing guides, …) are appendix material
        // whose bodies an orientation summary never reaches. Skip the
        // prose-body batches (Prelude + Section); the structural batches
        // (HeadingsOutline + ReadmeHeadline) still emit so the file's
        // shape stays discoverable at large budgets, riding the value
        // discount applied via `non_essential_factor`.
        let suppress_body = ctx.is_auto_injected_doc_file(&file) || is_peripheral_doc(&file);

        let Some((source, tree)) = parse_md(ctx, &file) else {
            continue;
        };
        let gates = derive_outline_gates(&file, &tree, &source);
        let (headline, outline_rows, outline_emits) = (gates.headline, gates.rows, gates.emits);
        let root_readme = is_readme(&file) && dir == ctx.root();
        let nav = NavDensity {
            sibling_md_count,
            root_readme,
        };
        let ranges = if suppress_body {
            Vec::new()
        } else {
            let ranges = logical_sections(&file, &tree, &source, outline_emits);
            if ranges.is_empty() && headed_sections(tree.root_node()).next().is_none() {
                headingless_fallback_ranges(&source)
            } else {
                ranges
            }
        };
        if ranges.is_empty() && !suppress_body {
            continue;
        }

        let mut headline_emitted: Option<BatchKey> = None;
        if let Some(spec) = &headline
            && let Some(content) = build_headline_content(&file, &source, spec)
        {
            let key = MarkdownKey::ReadmeHeadline { file: file.clone() };
            out.push(Batch {
                key: key.clone().into(),
                predecessor: None,
                content,
                value: readme_headline_value(&file, ctx, nav),
            });
            headline_emitted = Some(BatchKey::Markdown(key));
        }
        if let Some(spec) = &headline
            && !suppress_body
        {
            let rows = prelude_remainder_rows(&tree, &source, spec);
            if let Some(content) = single_file_lines_content(&file, &source, FileLines::new(rows)) {
                out.push(Batch {
                    key: MarkdownKey::Prelude { file: file.clone() }.into(),
                    predecessor: headline_emitted.clone(),
                    content,
                    value: prelude_value(&file, ctx, nav),
                });
            }
        }
        let mut outline_emitted: Option<BatchKey> = None;
        if outline_emits && let Some(content) = build_outline_content(&file, &source, &outline_rows)
        {
            let key = MarkdownKey::HeadingsOutline { file: file.clone() };
            out.push(Batch {
                key: key.clone().into(),
                predecessor: headline_emitted.clone(),
                content,
                value: headings_outline_value(&file, ctx, nav),
            });
            outline_emitted = Some(BatchKey::Markdown(key));
        }

        let section_predecessor = outline_emitted.or(headline_emitted);

        if suppress_body {
            continue;
        }

        push_sections(
            &mut out,
            &file,
            &source,
            &ranges,
            headline.as_ref(),
            section_predecessor,
            ctx,
        );
    }
    out
}

/// Emit one `Section` batch per range. Each gates on
/// `section_predecessor`, except oversize tails, which deliver in source
/// order behind the chunk before them.
fn push_sections(
    out: &mut Vec<Batch<BatchKey>>,
    file: &Path,
    source: &Source,
    ranges: &[SectionRange],
    headline: Option<&HeadlineSpec>,
    section_predecessor: Option<BatchKey>,
    ctx: &WalkCtx,
) {
    let base = section_base_value(file, ctx);
    let readme = is_readme(file);
    let mut prev_section_key: Option<BatchKey> = None;
    for (idx, range) in ranges.iter().enumerate() {
        if let Some(content) = build_section_content(file, source, range, headline) {
            let key = MarkdownKey::Section {
                file: file.to_path_buf(),
                section_index: idx,
                keeps_default_concavity: range.is_reference_usage_section || range.is_lede_body,
            };
            // Oversize tails deliver in source order: each chunk
            // gates on its predecessor chunk.
            let predecessor = if range.chained_to_previous {
                prev_section_key
                    .clone()
                    .or_else(|| section_predecessor.clone())
            } else {
                section_predecessor.clone()
            };
            prev_section_key = Some(BatchKey::Markdown(key.clone()));
            out.push(Batch {
                key: key.into(),
                predecessor,
                content,
                value: section_value(base, readme, range),
            });
        }
    }
}

// --- value ---

fn readme_headline_value(file: &Path, ctx: &WalkCtx, nav: NavDensity) -> f64 {
    mix_signals(0.9, 0.6, 0.8, path_depth_factor(file, ctx)) * nav.factor()
}

/// `Prelude` prices as the README's index-0 section: it is the top of
/// the README body, just above the first heading rather than below it.
fn prelude_value(file: &Path, ctx: &WalkCtx, nav: NavDensity) -> f64 {
    section_base_value(file, ctx) * nav.factor()
}

fn headings_outline_value(file: &Path, ctx: &WalkCtx, nav: NavDensity) -> f64 {
    mix_signals(0.7, 0.55, 0.4, path_depth_factor(file, ctx)) * nav.factor()
}

/// How crowded the directory is that a markdown file's navigation
/// batches (`HeadingsOutline` / `ReadmeHeadline` / `Prelude`) sit in,
/// plus whether this file is the repo's root README.
#[derive(Clone, Copy)]
struct NavDensity {
    sibling_md_count: usize,
    root_readme: bool,
}

impl NavDensity {
    /// Saturate a file's navigation value in dirs with many .md siblings
    /// — the dir listing already names them.
    ///
    /// The root README is exempt, and not cosmetically: its outline is the
    /// hard predecessor of every root README section, so damping it
    /// delays the whole README body, which is the largest credited
    /// early-budget purchase on repos that have one. A README deeper in
    /// the tree carries no such stream and is just one more page in a
    /// docs directory the listing already enumerated.
    fn factor(self) -> f64 {
        if self.root_readme || self.sibling_md_count <= DENSE_MD_SIBLINGS {
            return 1.0;
        }
        ((DENSE_MD_SIBLINGS as f64) / (self.sibling_md_count as f64)).sqrt()
    }
}

/// Markdown-file count above which a directory reads as a docs
/// directory rather than as a couple of loose notes, so its listing
/// stands in for the individual pages' navigation batches. Swept on the
/// 3K corpus mean at 6 / 4 / 3 / 2 → +0.0000 / +0.0019 / +0.0031 /
/// +0.0032; the response saturates between 3 and 2, and 3 is the
/// gentlest setting on the flat part. No fixture moves down at any grid
/// budget at any of these settings.
const DENSE_MD_SIBLINGS: usize = 3;

/// Boost for README usage/reference sections (see
/// [`SectionRange::is_reference_usage_section`]) so they clear the
/// early budget instead of sinking below the README index decay.
const REFERENCE_USAGE_SECTION_FACTOR: f64 = 1.3;

/// Index decay for README sections: `(idx + 1)^-0.15`, floored at 0.7,
/// counting from the first real H2. (An adaptive steeper falloff for
/// long READMEs (≥18 H2s) was tuned on the pre-refreeze keys and
/// measured obsolete on the frozen ones — un-shipped 2026-07-06.)
fn readme_index_decay(range: &SectionRange) -> f64 {
    (range.h2_index as f64 + 1.0).powf(-0.15).max(0.7)
}

/// Value of a section of `file` before its range's own factors: the
/// README section tier, or the heading-slab tier for any other doc.
fn section_base_value(file: &Path, ctx: &WalkCtx) -> f64 {
    if is_readme(file) {
        return mix_signals(0.55, 0.8, 0.7, path_depth_factor(file, ctx));
    }
    mix_signals(0.3, 0.5, 0.5, path_depth_factor(file, ctx))
}

/// Per-section value from the file's [`section_base_value`].
fn section_value(base: f64, readme: bool, range: &SectionRange) -> f64 {
    let mut value = base;
    if readme {
        value *= readme_index_decay(range);
    }
    if range.is_reference_usage_section {
        value *= REFERENCE_USAGE_SECTION_FACTOR;
    }
    value
}

// --- parser ---

fn parse_md(ctx: &WalkCtx, path: &Path) -> Option<(Arc<Source>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_md::LANGUAGE.into())
}

/// Parse `text` with the inline grammar — surfaces named `image` /
/// `inline_link` / `html_tag` children that the block grammar leaves
/// as opaque bytes.
fn parse_inline(text: &str) -> Option<Tree> {
    thread_local! {
        static INLINE_PARSER: std::cell::RefCell<Option<tree_sitter::Parser>> = {
            let mut parser = tree_sitter::Parser::new();
            let loaded = parser.set_language(&tree_sitter_md::INLINE_LANGUAGE.into());
            std::cell::RefCell::new(loaded.ok().map(|()| parser))
        };
    }
    INLINE_PARSER.with_borrow_mut(|parser| parser.as_mut()?.parse(text, None))
}

// --- content builders ---

fn build_headline_content(
    file: &Path,
    source: &Source,
    spec: &HeadlineSpec,
) -> Option<BatchContent> {
    let spans = build_headline_spans(file, source, spec);
    if spans.is_empty() {
        return None;
    }
    Some(BatchContent::Lines { spans })
}

fn build_outline_content(
    file: &Path,
    source: &Source,
    rows: &[(usize, usize)],
) -> Option<BatchContent> {
    if rows.len() < 2 {
        return None;
    }
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for (start, end) in rows {
        for r in *start..=*end {
            full.push(r);
        }
        // `end + 1` is either a row of this heading's own section (a
        // later `Section` batch gates on the outline, so its `full` is a
        // descendant upgrade) or the next heading's row, which the
        // outline renders itself and so drops the marker. Fragile seam:
        // the outline is a DESCENDANT of `ReadmeHeadline`, and a
        // descendant `…` on an ancestor-rendered row silently demotes
        // the paid row. Unreachable only because [`headline_spec`]
        // covers the first heading and blocks before the next one, and
        // that heading is dropped from `rows`.
        ellipses.push(end + 1);
    }
    single_file_lines_content(file, source, FileLines::new(full).with_ellipses(ellipses))
}

/// Per-file derivation chain shared by `expand_in_dir` and the unit-test
/// helpers, so the one question every split path asks — "which heading
/// rows does the outline own?" — is answered in exactly one place:
/// headline spec (READMEs only) → outline rows → outline gate.
struct OutlineGates {
    headline: Option<HeadlineSpec>,
    rows: Vec<(usize, usize)>,
    emits: bool,
}

fn derive_outline_gates(file: &Path, tree: &Tree, source: &str) -> OutlineGates {
    let headline = is_readme(file)
        .then(|| headline_spec(tree, source))
        .flatten();
    let rows = collectable_outline_rows(tree, source, headline.as_ref());
    let emits = (2..=MAX_OUTLINE_HEADINGS).contains(&rows.len());
    OutlineGates {
        headline,
        rows,
        emits,
    }
}

/// Heading row ranges for `HeadingsOutline` — levels 1–3, with any
/// headline-covered headings dropped (`headline` is `Some` only for
/// READMEs; the caller derives it once per file).
fn collectable_outline_rows(
    tree: &Tree,
    source: &str,
    headline: Option<&HeadlineSpec>,
) -> Vec<(usize, usize)> {
    let mut nodes = Vec::new();
    collect_heading_nodes(tree.root_node(), &mut nodes);
    let mut out = Vec::new();
    for node in nodes {
        let level = heading_level(node);
        if !(1..=3).contains(&level) {
            continue;
        }
        let start_row = node.start_position().row + 1;
        let end_row = node_end_row_trimmed(node, source) + 1;
        if headline.is_some_and(|spec| {
            spec.covered_rows
                .range(start_row..=end_row)
                .next()
                .is_some()
        }) {
            continue;
        }
        out.push((start_row, end_row));
    }
    out
}

/// Section headings in document order — tree-sitter-md nests every
/// section's heading as a direct child of its `section` node.
fn collect_heading_nodes<'a>(node: Node<'a>, out: &mut Vec<Node<'a>>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if matches!(child.kind(), "atx_heading" | "setext_heading") {
            out.push(child);
        } else if child.kind() == "section" {
            collect_heading_nodes(child, out);
        }
    }
}

fn headingless_fallback_ranges(source: &str) -> Vec<SectionRange> {
    let src_lines: Vec<&str> = source.lines().collect();
    let Some(start) = src_lines.iter().position(|line| !line.trim().is_empty()) else {
        return Vec::new();
    };
    let end = src_lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .expect("non-empty start implies non-empty end");
    let mut out = Vec::new();
    push_whole_or_head_split(
        &mut out,
        &src_lines,
        SectionRange::new(start + 1, end + 1, 0),
    );
    out
}

fn build_section_content(
    file: &Path,
    source: &Source,
    range: &SectionRange,
    headline: Option<&HeadlineSpec>,
) -> Option<BatchContent> {
    let (start, end) = (range.start, range.end);

    // A README section starts past the last row `ReadmeHeadline`
    // covers — else those rows' marginal cost goes to 0 and
    // `ratio(value, 0) = ∞`. (`headline` is `Some` only for READMEs.)
    // Rows the headline stepped *over* are dropped with them: admitting
    // that chrome measured −0.0033 corpus mean.
    let effective_start = match headline.and_then(|spec| spec.covered_rows.iter().next_back()) {
        Some(max_row) => (max_row + 1).max(start),
        None => start,
    };
    if effective_start > end {
        return None;
    }

    let lines: Vec<usize> = (effective_start..=end).collect();
    single_file_lines_content(file, source, FileLines::new(lines))
}

fn is_readme(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("README.md"))
        || is_readme_rst(file)
}

/// Root `README.rst` is the one `.rst` this walker owns; the plaintext
/// fallback must skip it so the two never emit overlapping spans.
pub(crate) fn is_readme_rst(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("README.rst"))
}

/// True for a real RST prose line — not a heading underline, badge row
/// (`|Build Status| …`), or directive (`.. figure::`).
fn is_rst_prose_line(line: &str) -> bool {
    let t = line.trim();
    if t.is_empty() || t.starts_with("..") || t.starts_with('|') || is_rst_underline(t, 1) {
        return false;
    }
    t.split_whitespace()
        .filter(|w| w.chars().any(|c| c.is_alphabetic()))
        .count()
        >= 3
}

fn is_rst_underline(line: &str, min_width: usize) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < min_width || trimmed.is_empty() {
        return false;
    }
    let first = trimmed.chars().next().unwrap();
    matches!(first, '=' | '-' | '~' | '^' | '*' | '+' | '#') && trimmed.chars().all(|c| c == first)
}

// --- RST body sections ---

/// A scanned RST heading: the 1-based row of the title text and the
/// 1-based row of the underline that closes it. `start_row` is the
/// heading's first row — the overline row for overline form, the title
/// row for setext form — so a following section can bound itself at
/// `start_row - 1` and not swallow this heading's overline punctuation.
struct RstHeading {
    start_row: usize,
    title_row: usize,
    underline_row: usize,
}

/// Line-scan all setext/overline RST headings. The first heading is the
/// document title; subsequent ones bound body sections.
fn scan_rst_headings(src_lines: &[&str]) -> Vec<RstHeading> {
    let mut headings = Vec::new();
    let mut i = 0;
    while i < src_lines.len() {
        // Overline form: punctuation row / title / matching punctuation.
        if i + 2 < src_lines.len()
            && is_rst_underline(src_lines[i], 1)
            && is_rst_underline(src_lines[i + 2], 1)
            && src_lines[i].trim() == src_lines[i + 2].trim()
            && !src_lines[i + 1].trim().is_empty()
        {
            headings.push(RstHeading {
                start_row: i + 1,
                title_row: i + 2,
                underline_row: i + 3,
            });
            i += 3;
            continue;
        }
        // Setext form: a non-blank title row followed by an underline at
        // least as wide as the title.
        if i + 1 < src_lines.len()
            && let Some(title) = src_lines.get(i).filter(|l| !l.trim().is_empty())
            && is_rst_underline(src_lines[i + 1], title.trim_end().chars().count())
        {
            headings.push(RstHeading {
                start_row: i + 1,
                title_row: i + 1,
                underline_row: i + 2,
            });
            i += 2;
            continue;
        }
        i += 1;
    }
    headings
}

/// An RST README split into its `ReadmeHeadline` and `Section` ranges.
/// The headline is the title plus the first prose paragraph under it —
/// or, when the title carries none, the first prose paragraph of an
/// intro-titled section (`Overview`, `Introduction`, …). The text
/// before the first body heading is a synthetic intro, and every later
/// heading opens one section, each through the oversize head-split.
fn rst_readme(source: &str) -> (HeadlineSpec, Vec<SectionRange>) {
    let src_lines: Vec<&str> = source.lines().collect();
    let headings = scan_rst_headings(&src_lines);
    let intro_end = headings
        .get(1)
        .map_or(src_lines.len(), |next| next.start_row - 1);
    let mut covered_rows = BTreeSet::new();
    let mut intro_start = 1;
    if let Some(title) = headings.first() {
        covered_rows.extend(title.start_row..=title.underline_row);
        intro_start = title.underline_row + 1;
    }
    let section_end = |i: usize| {
        headings
            .get(i + 1)
            .map_or(src_lines.len(), |next| next.start_row - 1)
    };
    let intro = rst_content_rows(&src_lines, intro_start, intro_end);
    if let Some(lede) = rst_lede(&src_lines, &intro) {
        covered_rows.extend(&intro[lede]);
    } else if let Some((rows, lede)) = headings.iter().enumerate().skip(1).find_map(|(i, h)| {
        let title = title_core(src_lines[h.title_row - 1]);
        if !matches!(
            title.as_str(),
            "overview" | "introduction" | "about" | "summary"
        ) {
            return None;
        }
        let rows = rst_content_rows(&src_lines, h.title_row, section_end(i));
        rst_lede(&src_lines, &rows).map(|lede| (rows, lede))
    }) {
        covered_rows.extend(&rows[lede]);
    }
    let mut ranges = Vec::new();
    if intro_start <= intro_end {
        let intro = SectionRange::new(intro_start, intro_end, 0);
        push_whole_or_head_split(&mut ranges, &src_lines, intro);
    }
    for (i, heading) in headings.iter().enumerate().skip(1) {
        let title = title_core(src_lines[heading.title_row - 1]);
        push_whole_or_head_split(
            &mut ranges,
            &src_lines,
            SectionRange {
                is_reference_usage_section: is_canonical_usage_title_core(&title)
                    || is_reference_usage_title_core(&title),
                ..SectionRange::new(heading.title_row, section_end(i), i - 1)
            },
        );
    }
    (
        HeadlineSpec {
            covered_rows,
            truncated_rows: BTreeSet::new(),
        },
        ranges,
    )
}

/// Index range in `rows` of the first prose paragraph: the first prose
/// line and the source-contiguous rows after it.
fn rst_lede(src_lines: &[&str], rows: &[usize]) -> Option<std::ops::Range<usize>> {
    let start = rows
        .iter()
        .position(|&row| is_rst_prose_line(src_lines[row - 1]))?;
    let len = rows[start..]
        .iter()
        .zip(rows[start]..)
        .take_while(|(row, expected)| **row == *expected)
        .count();
    Some(start..start + len)
}

/// Non-blank rows (1-based) of `[start, end]` outside `.. directive::`
/// blocks and their indented continuations — where a lede can sit.
fn rst_content_rows(src_lines: &[&str], start: usize, end: usize) -> Vec<usize> {
    let mut rows = Vec::new();
    let mut directive_indent: Option<usize> = None;
    for row in start..=end.min(src_lines.len()) {
        let line = src_lines[row - 1];
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let indent = line.len() - trimmed.len();
        if directive_indent.is_some_and(|open| indent > open) {
            continue;
        }
        directive_indent = None;
        if trimmed.starts_with("..") {
            directive_indent = Some(indent);
            continue;
        }
        rows.push(row);
    }
    rows
}

/// Lowercased leading alphanumeric/whitespace run of a heading title.
fn title_core(title: &str) -> String {
    let core: String = title
        .trim()
        .to_ascii_lowercase()
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect();
    core.trim().to_string()
}

// --- headline spec + span construction ---

/// Computed shape of `ReadmeHeadline` — rendered source rows, and the
/// long rows of an oversize lede block, which render as a prefix.
#[derive(Debug, Clone, Default)]
struct HeadlineSpec {
    covered_rows: BTreeSet<usize>,
    truncated_rows: BTreeSet<usize>,
}

/// Block kinds that bound a section — `ReadmeHeadline` never reaches
/// past these.
fn is_section_boundary(kind: &str) -> bool {
    matches!(kind, "section" | "atx_heading" | "setext_heading")
}

fn headline_spec(tree: &Tree, source: &str) -> Option<HeadlineSpec> {
    let section = headed_sections(tree.root_node()).next()?;
    let heading = first_heading_child(section)?;

    let mut spec = HeadlineSpec::default();
    // Prelude content: README opens with HTML title blocks / badges /
    // lede paragraph before the first heading.
    extend_lede(
        &mut spec,
        &prelude_blocks(tree.root_node(), section),
        source,
        true,
    );
    extend_rows_inclusive(&mut spec.covered_rows, heading, source);
    let heading_first_row = heading.start_position().row + 1;

    // The "tagline + lede" extension only fires under the project's
    // title heading (H1). For non-H1 first-headed sections (`### Usage`,
    // `## About`) the first substantive paragraph IS the section body
    // and shouldn't pull in further content.
    let post: Vec<Node> = children_after(section, heading);
    extend_lede(&mut spec, &post, source, heading_level(heading) == 1);

    debug_assert!(
        spec.covered_rows.contains(&heading_first_row),
        "headline covered_rows missing heading row"
    );
    Some(spec)
}

fn build_headline_spans(file: &Path, source: &Source, spec: &HeadlineSpec) -> Vec<Span> {
    let full_rows: Vec<usize> = spec
        .covered_rows
        .iter()
        .copied()
        .filter(|row| !spec.truncated_rows.contains(row))
        .collect();
    // Full rows go through `build_file_spans` (blank-filter + merge);
    // splice the truncated-row spans in afterwards.
    let mut spans = super::build_file_spans(file, source, FileLines::new(full_rows));
    spans.extend(spec.truncated_rows.iter().map(|&row| Span {
        path: file.to_path_buf(),
        start: row,
        end: row,
        render: Render::Truncated {
            pattern: format!(r"(?s)^.{{1,{HEADLINE_OVERSIZE_LINE_CHARS}}}"),
        },
    }));
    spans.sort_by_key(|span| span.start);
    spans
}

/// Append every 1-based row covered by `node` to `out`, trimming a
/// trailing newline tree-sitter-md sometimes includes in a node's span.
fn extend_rows_inclusive(out: &mut BTreeSet<usize>, node: Node, source: &str) {
    let last = node_end_row_trimmed(node, source);
    for row in node.start_position().row..=last {
        out.insert(row + 1);
    }
}

fn extend_headline_block_rows(spec: &mut HeadlineSpec, node: Node, source: &str) {
    if node.end_byte() - node.start_byte() <= HEADLINE_BLOCK_BYTE_GATE {
        extend_rows_inclusive(&mut spec.covered_rows, node, source);
        return;
    }
    let src_lines: Vec<&str> = source.lines().collect();
    let start = node.start_position().row;
    let end = node_end_row_trimmed(node, source);
    let mut bytes = 0usize;
    for row in start..=end {
        let line = src_lines.get(row).copied().unwrap_or_default();
        if line.trim().is_empty() {
            continue;
        }
        spec.covered_rows.insert(row + 1);
        if line.chars().count() > HEADLINE_OVERSIZE_LINE_CHARS {
            spec.truncated_rows.insert(row + 1);
        }
        bytes += line.len() + 1;
        if bytes >= HEADLINE_OVERSIZE_LEDE_BYTES {
            break;
        }
    }
}

fn children_after<'a>(parent: Node<'a>, after: Node<'a>) -> Vec<Node<'a>> {
    let mut cur = parent.walk();
    let mut seen = false;
    let mut out = Vec::new();
    for child in parent.children(&mut cur) {
        if seen {
            out.push(child);
        }
        if child.id() == after.id() {
            seen = true;
        }
    }
    out
}

// --- decorative classifiers ---

/// Inline children with no semantic content — whitespace, breaks, and
/// HTML tags other than `<img>` (a tag's text sits outside it).
fn is_skippable_inline(node: Node, source: &str) -> bool {
    match node.kind() {
        "text" => source[node.start_byte()..node.end_byte()].trim().is_empty(),
        "hard_line_break" | "soft_line_break" => true,
        "html_tag" => !is_img_html_tag(node, source),
        _ => false,
    }
}

/// Image / badge / `<img>` inline. Plain text-links are NOT decorative.
fn is_decorative_inline(node: Node, source: &str) -> bool {
    match node.kind() {
        "image" => true,
        "inline_link" | "full_reference_link" | "collapsed_reference_link" | "shortcut_link" => {
            let Some(link_text) = first_child_of_kind(node, "link_text", false) else {
                return false;
            };
            link_text_is_image_only_direct(link_text, source)
        }
        "html_tag" => is_img_html_tag(node, source),
        _ => false,
    }
}

fn is_img_html_tag(node: Node, source: &str) -> bool {
    source[node.start_byte()..node.end_byte()]
        .trim_start()
        .to_ascii_lowercase()
        .starts_with("<img")
}

/// True iff every direct child of `link_text` is skippable or image-
/// shaped — direct only (recursion would catch image alt-text).
fn link_text_is_image_only_direct(link_text: Node, source: &str) -> bool {
    let mut cur = link_text.walk();
    let mut had_any = false;
    for child in link_text.children(&mut cur) {
        if is_skippable_inline(child, source) {
            continue;
        }
        had_any = true;
        match child.kind() {
            "image" => continue,
            "html_tag" if is_img_html_tag(child, source) => continue,
            _ => return false,
        }
    }
    had_any
}

/// Paragraph is decorative iff every non-skippable inline child is
/// decorative AND there's at least one. Plain-text paragraphs (no
/// named inline children) return `false`.
fn is_decorative_paragraph(para: Node, source: &str) -> bool {
    let Some(inline_block) = first_child_of_kind(para, "inline", false) else {
        return false;
    };
    let inline_text = &source[inline_block.start_byte()..inline_block.end_byte()];
    let Some(tree) = parse_inline(inline_text) else {
        return false;
    };
    let root = tree.root_node();
    inline_root_is_all_decorative(root, inline_text)
}

fn is_decorative_block(block: Node, source: &str) -> bool {
    match block.kind() {
        "paragraph" => is_decorative_paragraph(block, source),
        // `[label]: url` definitions render nothing on their own.
        "link_reference_definition" => true,
        "html_block" => is_decorative_html_block(block, source),
        "block_quote" => is_admin_block_quote(block, source),
        _ => false,
    }
}

/// True iff every fragment (named + plain-text gaps) is decorative or
/// whitespace, and at least one fragment exists.
fn inline_root_is_all_decorative(root: Node, inline_text: &str) -> bool {
    let mut cur = root.walk();
    let named: Vec<Node> = root.children(&mut cur).filter(|c| c.is_named()).collect();
    let mut any_decorative = false;
    for node in &named {
        if is_decorative_inline(*node, inline_text) {
            any_decorative = true;
        } else if !is_skippable_inline(*node, inline_text) {
            return false;
        }
    }
    let mut cursor = 0;
    for node in &named {
        if !inline_text[cursor..node.start_byte()].trim().is_empty() {
            return false;
        }
        cursor = node.end_byte();
    }
    any_decorative && inline_text[cursor..].trim().is_empty()
}

fn named_decorative_candidates<'a>(root: Node<'a>, inline_text: &str) -> Vec<Node<'a>> {
    let mut cur = root.walk();
    root.children(&mut cur)
        .filter(|c| c.is_named() && !is_skippable_inline(*c, inline_text))
        .collect()
}

/// An `html_block` is decorative iff its source text contains nothing
/// but tags + whitespace. Strips `<…>` runs and checks whether the
/// remainder is blank. Catches `<p align="center"><img …/></p>` while
/// preserving blocks that contain real prose.
fn is_decorative_html_block(block: Node, source: &str) -> bool {
    let raw = &source[block.start_byte()..block.end_byte()];
    let stripped = strip_html_tags(raw);
    stripped.trim().is_empty()
}

/// True when a `block_quote` is a GitHub-flavored admin callout
/// (`> [!WARNING]`, `> [!NOTE]`, etc.). Short single-paragraph
/// block_quotes (typically taglines — `> Ky is a tiny and elegant HTTP
/// client...`) are NOT classified as admin and stay in the headline.
fn is_admin_block_quote(block: Node, source: &str) -> bool {
    let raw = &source[block.start_byte()..block.end_byte()];
    for line in raw.lines().take(2) {
        let trimmed = line.trim_start_matches('>').trim();
        if let Some(rest) = trimmed.strip_prefix("[!")
            && rest
                .split_once(']')
                .is_some_and(|(_, after)| after.trim().is_empty())
        {
            return true;
        }
    }
    false
}

/// A navigation row: three or more links and nothing else but separator
/// punctuation (`•`, `·`, `|`, `/`, `,`, dashes) — multi-language
/// READMEs' `[English](url) • [中文](url) • ...`.
fn is_nav_link_paragraph(para: Node, source: &str) -> bool {
    let Some(inline_block) = first_child_of_kind(para, "inline", false) else {
        return false;
    };
    let inline_text = &source[inline_block.start_byte()..inline_block.end_byte()];
    let Some(tree) = parse_inline(inline_text) else {
        return false;
    };
    let named = named_decorative_candidates(tree.root_node(), inline_text);
    let mut cursor = 0usize;
    for n in &named {
        if !matches!(
            n.kind(),
            "inline_link" | "full_reference_link" | "collapsed_reference_link" | "shortcut_link"
        ) || !is_separator_gap(&inline_text[cursor..n.start_byte()])
        {
            return false;
        }
        cursor = n.end_byte();
    }
    named.len() >= 3 && is_separator_gap(&inline_text[cursor..])
}

fn is_separator_gap(s: &str) -> bool {
    s.chars().all(|c| {
        c.is_whitespace()
            || matches!(
                c,
                '\u{2022}' | '\u{00B7}' | '|' | '/' | '\\' | ',' | '-' | '\u{2014}' | '\u{2013}'
            )
    })
}

/// True iff the headline block reads as a "tagline" — short enough
/// that the next non-decorative block is plausibly the actual prose
/// lede the reader needs (posting's `**A powerful HTTP client...**`,
/// ts-pattern's `<h1 align="center">TS-Pattern</h1>`). Bounded by the
/// stripped-text length of the block: row count alone treats a single
/// long sentence ("D2TS is a TypeScript implementation of differential
/// dataflow ...") as short, but its ~250 chars of prose is the lede
/// itself, not a tagline preceding one.
fn is_short_substantive_block(block: Node, source: &str) -> bool {
    let raw = &source[block.start_byte()..block.end_byte()];
    let stripped = strip_block_for_length(raw);
    stripped.chars().count() <= HEADLINE_TAGLINE_MAX_CHARS
}

/// Maximum character count for the stripped content of a "tagline"
/// block, beyond which the extension is suppressed because the block
/// is itself the substantive lede.
const HEADLINE_TAGLINE_MAX_CHARS: usize = 90;

/// Strip markdown / HTML markup from a block's raw source for the
/// purposes of measuring its "content length". Removes `<...>` HTML
/// tags, leading `>` block-quote markers, and common emphasis markup
/// (`**`, `*`, `_`, `` ` ``) so a bolded tagline measures by its
/// underlying prose rather than its punctuation.
fn strip_block_for_length(raw: &str) -> String {
    strip_html_tags(raw)
        .chars()
        .filter(|c| !matches!(c, '>' | '*' | '_' | '`'))
        .collect()
}

fn strip_html_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '<' {
            // Skip the tag through its closing '>', then drop the '>'.
            for inner in chars.by_ref() {
                if inner == '>' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

// --- tree-sitter-md helpers ---

/// One scheduling unit for a markdown file: 1-based inclusive row range
/// plus `h2_index`, the README index decay's input — the
/// position of its parent H2 in the *un-split* top-level section list,
/// counting the first real H2 as 0 (an H1-unwrap intro shares index 0
/// with it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SectionRange {
    start: usize,
    end: usize,
    h2_index: usize,
    /// README-only: this range's section is a code-dominant canonical usage demo
    /// ([`is_canonical_usage_h2`]), or is titled in the reference/usage
    /// vocabulary ([`is_reference_usage_title`]) with a compact,
    /// structured body. Earns [`REFERENCE_USAGE_SECTION_FACTOR`] and
    /// the default concavity.
    is_reference_usage_section: bool,
    /// Oversize tail chunks gate on the previous chunk so the section
    /// delivers as an in-order prefix.
    chained_to_previous: bool,
    /// The section body directly behind a carved lede — the rest of what
    /// the unsplit section would have delivered, so it keeps the index-0
    /// concavity.
    is_lede_body: bool,
}

impl SectionRange {
    fn new(start: usize, end: usize, h2_index: usize) -> Self {
        Self {
            start,
            end,
            h2_index,
            is_reference_usage_section: false,
            chained_to_previous: false,
            is_lede_body: false,
        }
    }
}

/// A list item, numbered item, or table row — the catalog subset of
/// [`is_reference_structure_line`] (fences excluded).
fn is_catalog_line(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("- ")
        || t.starts_with("* ")
        || t.starts_with("+ ")
        || t.starts_with('|')
        || t.split_once(". ")
            .is_some_and(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// Section ranges for batching: one `Whole` per top-level entry,
/// head-split when oversize.
fn logical_sections(
    file: &Path,
    tree: &Tree,
    source: &str,
    outline_emits: bool,
) -> Vec<SectionRange> {
    let entries = top_level_entries(tree.root_node(), source);
    // The first real H2 is index 0; an H1-unwrap intro shares it.
    let h2_offset = usize::from(matches!(
        entries.first(),
        Some(TopLevelEntry::SyntheticIntro { .. })
    ));

    let readme = is_readme(file);
    let src_lines: Vec<&str> = source.lines().collect();
    let mut out = Vec::with_capacity(entries.len());
    for (entry_idx, entry) in entries.iter().enumerate() {
        let h2_idx = entry_idx.saturating_sub(h2_offset);
        match entry {
            TopLevelEntry::SyntheticIntro {
                heading_only,
                start,
                end,
            } => {
                // A heading-only intro would re-sell the outline's H1 row.
                if outline_emits && *heading_only {
                    continue;
                }
                push_whole_or_head_split(
                    &mut out,
                    &src_lines,
                    SectionRange::new(*start, *end, h2_idx),
                );
            }
            TopLevelEntry::H2Section { node, start, end } => {
                // README H2 that is either a code-dominant canonical
                // usage demo, or titled in the reference/usage
                // vocabulary with a non-trivial but compact body (so
                // stub H2s and large prose/demo blobs are excluded) AND
                // structural reference content (list / table / code),
                // so a prose-only intro under a reference title is
                // skipped.
                let reference_h2 = readme
                    && (is_canonical_usage_h2(*node, source)
                        || (is_reference_usage_title(*node, source)
                            && reference_usage_body_ok(section_body(*node, None, source))));
                push_whole_or_head_split(
                    &mut out,
                    &src_lines,
                    SectionRange {
                        is_reference_usage_section: reference_h2,
                        ..SectionRange::new(*start, *end, h2_idx)
                    },
                );
            }
        }
    }
    out
}

/// Contiguous chunk bounds for the oversize head-split: cover
/// `[start, end]` exactly, cutting at blank rows outside code fences
/// once the running chunk reaches [`OVERSIZE_CHUNK_TARGET_TOKENS`]. A
/// cut is skipped when the next non-blank row opens a fence (the
/// fence binds to the paragraph introducing it) or when the remainder
/// would fall below [`OVERSIZE_CHUNK_MIN_TAIL_TOKENS`]. Chunk 0 is
/// then refined into a lede plus body; the returned flag says whether
/// that lede cut was taken.
fn oversize_chunk_bounds(
    src_lines: &[&str],
    start: usize,
    end: usize,
) -> (Vec<(usize, usize)>, bool) {
    let item_count = end - start + 1;
    let mut token_prefix = Vec::with_capacity(item_count + 1);
    token_prefix.push(0);
    let mut char_prefix = Vec::with_capacity(item_count + 1);
    char_prefix.push(0);
    let mut legal_split = vec![false; item_count];
    let mut safe_split = vec![false; item_count];
    let mut open_fence: Option<(char, usize)> = None;
    let mut open_html: Option<RawHtmlBlock> = None;
    for (index, row) in (start..=end).enumerate() {
        token_prefix.push(token_prefix[index] + row_tokens(src_lines, row));
        let raw = src_lines.get(row - 1).copied().unwrap_or("");
        let t = raw.trim_start();
        char_prefix.push(char_prefix[index] + raw.chars().count() + 1);
        // Marker-matched fence state (see `fence_closes`) and raw-HTML
        // block state (see `RawHtmlBlock`): a blank row is only a block
        // boundary outside both, so neither a fenced example nor a
        // `<script>` body can be cut through the middle.
        if let Some(open) = open_fence {
            if fence_closes(t, open) {
                open_fence = None;
            }
            if index + 1 < item_count {
                safe_split[index + 1] = open_fence.is_none();
            }
            continue;
        }
        if let Some(block) = open_html {
            if block.closed_by(t) {
                open_html = None;
            }
            if index + 1 < item_count {
                safe_split[index + 1] = open_html.is_none();
            }
            continue;
        }
        if let Some(marker) = fence_marker(t) {
            open_fence = Some(marker);
            continue;
        }
        if let Some(block) = RawHtmlBlock::opened_by(t)
            && !block.closed_by(t)
        {
            open_html = Some(block);
            continue;
        }
        if index + 1 < item_count {
            safe_split[index + 1] = true;
        }
        if !t.is_empty() {
            continue;
        }
        if index + 1 < item_count {
            legal_split[index + 1] = !next_nonblank_opens_fence(src_lines, row + 1, end);
        }
    }
    let mut chunk_start = 0usize;
    for index in 1..item_count {
        if legal_split[index] {
            chunk_start = index;
        } else if safe_split[index]
            && char_prefix[index] - char_prefix[chunk_start] >= OVERSIZE_HARD_CHUNK_CHAR_CAP
        {
            legal_split[index] = true;
            chunk_start = index;
        }
    }
    let mut ranges = budget_chunk_ranges(
        item_count,
        |range| token_prefix[range.end] - token_prefix[range.start],
        OVERSIZE_CHUNK_TARGET_TOKENS,
        OVERSIZE_CHUNK_MIN_TAIL_TOKENS,
        |index| legal_split[index],
        |_| true,
    );
    // The lede cut refines chunk 0 only: the earliest legal boundary
    // past `LEDE_TARGET_TOKENS`, taken when enough of that chunk
    // remains behind it to be worth its own batch. Later chunk bounds
    // are untouched, so carving a lede never makes the section's body
    // more expensive than it was unsplit.
    let head = ranges[0].clone();
    let lede = (head.start + 1..head.end).find(|&index| {
        legal_split[index]
            && token_prefix[index] - token_prefix[head.start] >= LEDE_TARGET_TOKENS
            && token_prefix[head.end] - token_prefix[index] >= OVERSIZE_CHUNK_MIN_TAIL_TOKENS
    });
    if let Some(index) = lede {
        ranges[0] = index..head.end;
        ranges.insert(0, head.start..index);
    }
    let bounds = ranges
        .into_iter()
        .map(|range| (start + range.start, start + range.end - 1))
        .collect();
    (bounds, lede.is_some())
}

/// Per-row token count (row is 1-based; includes the newline). Rides
/// the tokenizer's per-line memoization, so repeated sweeps are cheap.
fn row_tokens(src_lines: &[&str], row: usize) -> usize {
    src_lines
        .get(row - 1)
        .map(|l| tokenizer::count(&format!("{l}\n")))
        .unwrap_or(0)
}

/// The fence delimiter at the start of an already-trimmed line — its
/// marker char and run length (≥ 3) — if any. Shared with Rust doc
/// comment extraction (rustdoc is markdown).
pub(in crate::walker) fn fence_marker(trimmed: &str) -> Option<(char, usize)> {
    let c = trimmed.chars().next()?;
    if c != '`' && c != '~' {
        return None;
    }
    let run = trimmed.chars().take_while(|&x| x == c).count();
    (run >= 3).then_some((c, run))
}

/// Whether an already-trimmed line closes the fence opened by `open`.
/// CommonMark requires a closing fence to be a run of the *same*
/// character, at least as long as the opener, with nothing but
/// whitespace after it — so neither a ``` line inside a ~~~ fence nor
/// a ``` line carrying a trailing word leaves the fence. Getting the
/// second half wrong desynchronizes the fence state and exposes the
/// blank rows behind it as cut points.
pub(in crate::walker) fn fence_closes(trimmed: &str, open: (char, usize)) -> bool {
    let (open_char, open_run) = open;
    fence_marker(trimmed).is_some_and(|(c, run)| {
        c == open_char && run >= open_run && trimmed[run..].trim().is_empty()
    })
}

/// Tags whose raw-HTML block (CommonMark type 1) runs verbatim to its
/// closing tag.
const RAW_HTML_VERBATIM_TAGS: [&str; 4] = ["script", "pre", "style", "textarea"];
const RAW_HTML_VERBATIM_CLOSERS: [&str; 4] = ["</script>", "</pre>", "</style>", "</textarea>"];

/// A raw-HTML block whose end condition is a closing token rather than
/// a blank line — CommonMark block types 1–5. Types 6 and 7 do end at
/// a blank line and need no tracking. Inside one of these a blank row
/// is *not* a block boundary, so cutting a chunk there severs the
/// construct, silently, at render time.
#[derive(Clone, Copy)]
enum RawHtmlBlock {
    /// `<script` / `<pre` / `<style` / `<textarea` (type 1).
    Verbatim,
    /// `<!--` (2), `<?` (3), `<!DECL` (4), `<![CDATA[` (5).
    Token(&'static str),
}

impl RawHtmlBlock {
    /// The block an already-trimmed line opens, if any.
    fn opened_by(trimmed: &str) -> Option<Self> {
        if trimmed.starts_with("<!--") {
            return Some(Self::Token("-->"));
        }
        if trimmed.starts_with("<?") {
            return Some(Self::Token("?>"));
        }
        if trimmed.starts_with("<![CDATA[") {
            return Some(Self::Token("]]>"));
        }
        if let Some(rest) = trimmed.strip_prefix("<!")
            && rest.starts_with(|c: char| c.is_ascii_alphabetic())
        {
            return Some(Self::Token(">"));
        }
        let rest = trimmed.strip_prefix('<')?.as_bytes();
        RAW_HTML_VERBATIM_TAGS
            .iter()
            .any(|tag| {
                let tag = tag.as_bytes();
                rest.len() >= tag.len()
                    && rest[..tag.len()].eq_ignore_ascii_case(tag)
                    && rest
                        .get(tag.len())
                        .is_none_or(|b| b.is_ascii_whitespace() || *b == b'>')
            })
            .then_some(Self::Verbatim)
    }

    /// Whether an already-trimmed line meets the block's end condition.
    /// The opening line can meet it itself.
    fn closed_by(self, trimmed: &str) -> bool {
        let lowered = trimmed.to_ascii_lowercase();
        match self {
            Self::Verbatim => RAW_HTML_VERBATIM_CLOSERS
                .iter()
                .any(|closer| lowered.contains(closer)),
            Self::Token(end) => lowered.contains(end),
        }
    }
}

fn next_nonblank_opens_fence(src_lines: &[&str], from: usize, end: usize) -> bool {
    for row in from..=end {
        let t = src_lines.get(row - 1).copied().unwrap_or("").trim_start();
        if t.is_empty() {
            continue;
        }
        return fence_marker(t).is_some();
    }
    false
}

/// Emit `head` as-is, or — when its row range exceeds
/// [`OVERSIZE_SECTION_SPLIT_TOKENS`] and splits at natural boundaries —
/// shrink `head` to the first chunk (keeping its value flags; the head
/// includes the section heading, so no outline is required to preserve
/// it) and follow it with predecessor-chained tail chunks priced like
/// the head: a tail is the direct continuation of content whose head
/// just won purchase. When the head itself carves a lede
/// ([`LEDE_TARGET_TOKENS`]), the chunk behind it is the lede body.
fn push_whole_or_head_split(out: &mut Vec<SectionRange>, src_lines: &[&str], head: SectionRange) {
    let (start, end) = (head.start, head.end);
    let tokens: usize = (start..=end).map(|r| row_tokens(src_lines, r)).sum();
    if tokens < OVERSIZE_SECTION_SPLIT_TOKENS {
        out.push(head);
        return;
    }
    let (bounds, lede) = oversize_chunk_bounds(src_lines, start, end);
    for (i, (chunk_start, chunk_end)) in bounds.into_iter().enumerate() {
        // Chunk 0 is the lede when one was carved; the chunk directly
        // behind it is the section remainder and keeps the section's
        // own price (the entry-slice law: re-pricing the remainder
        // demotes the content the carve exists to reach). Only further
        // chunks are continuations.
        if i == 0 {
            out.push(SectionRange {
                start: chunk_start,
                end: chunk_end,
                ..head
            });
        } else {
            out.push(SectionRange {
                start: chunk_start,
                end: chunk_end,
                chained_to_previous: true,
                is_lede_body: i == 1 && lede,
                ..head
            });
        }
    }
}

/// One entry in the un-split top-level section list. `SyntheticIntro`
/// is the row range carved out by H1-unwrap to preserve the H1 heading
/// and the prelude before the first H2 (a virtual section inside the
/// H1's node). `H2Section` carries the tree-sitter node so
/// [`direct_h3_children`] and source-byte length can be derived without
/// re-walking from the root.
#[derive(Debug, Clone, Copy)]
enum TopLevelEntry<'a> {
    SyntheticIntro {
        /// Nothing but the H1 heading before the first H2.
        heading_only: bool,
        start: usize,
        end: usize,
    },
    H2Section {
        node: Node<'a>,
        start: usize,
        end: usize,
    },
}

/// Top-level section list with H1-unwrap. If the doc has exactly one
/// top-level section and it's an H1, descend into its H2 children and
/// synthesize an intro range for the H1 heading + pre-first-H2
/// prelude. Without this, READMEs styled `# Title` (mitt, mdbook,
/// otree) would collapse into one multi-KB blob.
fn top_level_entries<'a>(root: Node<'a>, source: &'a str) -> Vec<TopLevelEntry<'a>> {
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
                out.push(TopLevelEntry::SyntheticIntro {
                    heading_only: section_body(h1, Some(h2s[0]), source).trim().is_empty(),
                    start: intro_start,
                    end: intro_end,
                });
            }
            for h2 in h2s {
                out.push(TopLevelEntry::H2Section {
                    node: h2,
                    start: h2.start_position().row + 1,
                    end: node_end_row_trimmed(h2, source) + 1,
                });
            }
            return out;
        }
    }
    top.into_iter()
        .map(|s| TopLevelEntry::H2Section {
            node: s,
            start: s.start_position().row + 1,
            end: node_end_row_trimmed(s, source) + 1,
        })
        .collect()
}

fn is_code_block(kind: &str) -> bool {
    matches!(kind, "fenced_code_block" | "indented_code_block")
}

/// README H2 sections worth a canonical-usage boost: title is one of
/// the canonical-demo markers (`## Usage` / `## Sample usage` /
/// `## Example(s)` / `## Quick start` / `## Getting started` /
/// `## Basic usage` / `## Demo`), AND the body is dominated by code
/// blocks. The body-shape gate matters: a bullet-list-of-links
/// `### Examples` (superstruct) shares the title pattern but isn't a
/// canonical demo — boosting it displaces NS-anchored content for no
/// gain. Gated on the READMEs-only call site, so unrelated `## Usage`
/// headings inside a changelog or docs page don't trigger.
fn is_canonical_usage_h2(h2_section: Node<'_>, source: &str) -> bool {
    is_canonical_usage_h2_title(h2_section, source) && section_is_code_dominant(h2_section, source)
}

fn is_canonical_usage_h2_title(h2_section: Node<'_>, source: &str) -> bool {
    h2_title_core(h2_section, source).is_some_and(|core| is_canonical_usage_title_core(&core))
}

/// Vocabulary half of [`is_canonical_usage_h2_title`], shared with the
/// RST heading path.
fn is_canonical_usage_title_core(core: &str) -> bool {
    matches!(
        core,
        "usage"
            | "sample usage"
            | "basic usage"
            | "example"
            | "examples"
            | "usage example"
            | "usage examples"
            | "quick start"
            | "quickstart"
            | "getting started"
            | "demo"
    )
}

/// README reference/usage sections worth the modest
/// [`REFERENCE_USAGE_SECTION_FACTOR`]: title (taken from the section's
/// own first heading) matches a tight reference/usage
/// vocabulary. Unlike [`is_canonical_usage_h2`] this is NOT gated on
/// code dominance — the point is to lift prose/list/table reference
/// sections (option tables, color/modifier lists, environment-variable
/// docs) above the README index decay so they clear the early budget.
/// The vocabulary is kept tight and the body-bytes gate is enforced at
/// the call sites; a too-broad list would over-promote trivial
/// sections. README-only (gated at the call sites).
fn is_reference_usage_title(section: Node<'_>, source: &str) -> bool {
    h2_title_core(section, source).is_some_and(|core| is_reference_usage_title_core(&core))
}

/// Vocabulary half of [`is_reference_usage_title`], shared with the
/// RST heading path (which has no tree-sitter node to extract from).
fn is_reference_usage_title_core(core: &str) -> bool {
    matches!(
        core,
        // Bare "usage" is deliberately excluded: code-dominant usage
        // demos are already handled by the canonical path, and a
        // prose/demo `## Usage` blob (json-server) only displaces source
        // NS when promoted. The *specific* usage titles below name
        // genuine CLI/API reference sections.
        "command line usage"
            | "command-line usage"
            | "cli usage"
            | "command line options"
            | "command-line options"
            | "options"
            | "flags"
            | "key features"
            | "features"
            | "configuration"
            | "config"
            | "api"
            | "api usage"
            | "colors"
            | "modifiers"
            | "styles"
            | "background colors"
            | "environment variables"
            | "formatters"
            | "default preset"
    )
}

/// Upper bound (source bytes, heading excluded) on a reference/usage
/// section's body for it to earn [`REFERENCE_USAGE_SECTION_FACTOR`].
/// Reference tables/lists (option tables, color/modifier lists,
/// feature bullets) are compact — krep's `## Command Line Options`
/// (~1.2 KB) and chalk's `## 256 and Truecolor` (~1.3 KB) clear it. The
/// cap excludes large `## Usage` prose/demo blobs (debug ~2.1 KB,
/// superstruct `### Usage` ~2.3 KB) that the README index decay
/// correctly keeps off the early budget — promoting them displaces
/// NS-anchored source content for no gain.
const REFERENCE_USAGE_MAX_BODY_BYTES: usize = 1500;

/// Source text of `section` after its heading, up to the start of
/// `until` (a child the body stops at, e.g. an H1's first H2) or the
/// section's end. Empty for a section without a heading.
fn section_body<'a>(section: Node, until: Option<Node>, source: &'a str) -> &'a str {
    let Some(heading) = first_heading_child(section) else {
        return "";
    };
    let end = until.map_or(section.end_byte(), |node| node.start_byte());
    &source[heading.end_byte().min(end)..end]
}

/// The body gate of the reference/usage class: non-empty, at most
/// [`REFERENCE_USAGE_MAX_BODY_BYTES`], and holding structural reference
/// content (see [`is_reference_structure_line`]).
fn reference_usage_body_ok(body: &str) -> bool {
    !body.trim().is_empty()
        && body.len() <= REFERENCE_USAGE_MAX_BODY_BYTES
        && body.lines().any(is_reference_structure_line)
}

/// True iff `line` is structural reference content: a bullet / numbered
/// list item, a table row, or a code-fence line. Reference sections
/// worth the boost catalog options / colors / flags as lists or tables
/// (or hold a config code block); a section whose body is only prose —
/// e.g. enclosed's `### Configuration`, a one-line pointer to external
/// docs — is an intro, not a reference, and is correctly skipped.
fn is_reference_structure_line(line: &str) -> bool {
    let t = line.trim_start();
    is_catalog_line(line) || t.starts_with("```") || t.starts_with("~~~")
}

/// True iff `section`'s direct children include at least one
/// `fenced_code_block` whose source bytes are at least
/// [`CANONICAL_USAGE_CODE_MIN_FRACTION`] of the section's total body
/// bytes (the heading is excluded from the denominator since titles
/// are tiny and would otherwise tilt every section toward "code
/// dominant"). The fraction threshold is conservative — well above
/// what a "few code snippets among prose" tutorial section can clear,
/// but below the typical "headline + one code fence + a paragraph"
/// canonical-demo shape (sqlite-vec's `## Sample usage` is ~75 % code
/// by source bytes).
const CANONICAL_USAGE_CODE_MIN_FRACTION: f64 = 0.85;

fn section_is_code_dominant(section: Node<'_>, source: &str) -> bool {
    let body_bytes = section_body(section, None, source).len();
    if body_bytes == 0 {
        return false;
    }
    let mut cur = section.walk();
    let code_bytes: usize = section
        .children(&mut cur)
        .filter(|c| is_code_block(c.kind()))
        .map(|c| c.end_byte() - c.start_byte())
        .sum();
    (code_bytes as f64 / body_bytes as f64) >= CANONICAL_USAGE_CODE_MIN_FRACTION
}

/// [`title_core`] of a section's heading. `None` when no `inline`
/// child is found.
fn h2_title_core(h2_section: Node<'_>, source: &str) -> Option<String> {
    let heading = first_heading_child(h2_section)?;
    let inline = first_child_of_kind(heading, "inline", false)?;
    Some(title_core(&source[inline.start_byte()..inline.end_byte()]))
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

/// True for an HTML nav / table-of-contents block — a `<p>`/`<div>` whose
/// links point at page sections (`href="#…"`), e.g. py3xui's
/// `Overview • Quick Start • Examples` menu. Decorative chrome, not lede.
fn is_html_nav_block(block: Node, source: &str) -> bool {
    if block.kind() != "html_block" {
        return false;
    }
    let text = &source[block.start_byte()..block.end_byte()];
    if text.matches("href=\"#").count() + text.matches("href='#").count() >= 2 {
        return true;
    }
    // Same construct with absolute URLs (`<a>Demo</a> • <a>Docs</a> •
    // <a>CLI</a>`): two or more anchors with nothing but separator
    // punctuation between them once the anchors themselves are removed.
    let (anchors, rest) = strip_anchor_elements(text);
    anchors >= 2 && is_separator_gap(&strip_html_entities(&strip_html_tags(&rest)))
}

/// Remove whole `<a …>…</a>` elements, returning the anchor count and
/// the surrounding text.
fn strip_anchor_elements(html: &str) -> (usize, String) {
    let mut count = 0;
    let mut rest = String::with_capacity(html.len());
    let mut cursor = 0usize;
    while let Some(open) = html[cursor..].find("<a ").map(|i| cursor + i) {
        let Some(close) = html[open..].find("</a>").map(|i| open + i + "</a>".len()) else {
            break;
        };
        rest.push_str(&html[cursor..open]);
        count += 1;
        cursor = close;
    }
    rest.push_str(&html[cursor..]);
    (count, rest)
}

/// Replace `&nbsp;`-style character references with a space so entity
/// padding doesn't read as content.
fn strip_html_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let tail = &rest[amp + 1..];
        match tail.find(';').filter(|end| {
            tail[..*end]
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '#')
        }) {
            Some(end) => {
                out.push(' ');
                rest = &tail[end + 1..];
            }
            None => {
                out.push('&');
                rest = tail;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Include `blocks` through the first substantive paragraph, stepping
/// over chrome ([`is_prelude_chrome_block`]); after a short tagline
/// paragraph, take one more block.
fn extend_lede(
    spec: &mut HeadlineSpec,
    blocks: &[Node],
    source: &str,
    allow_tagline_extension: bool,
) {
    let mut extending = false;
    for &block in blocks {
        if is_section_boundary(block.kind()) {
            break;
        }
        if is_prelude_chrome_block(block, source) {
            continue;
        }
        extend_headline_block_rows(spec, block, source);
        if extending {
            break;
        }
        if block.kind() == "paragraph" {
            if allow_tagline_extension && is_short_substantive_block(block, source) {
                extending = true;
            } else {
                break;
            }
        }
    }
}

/// Top-level blocks above the first headed section.
fn prelude_blocks<'a>(root: Node<'a>, first_headed: Node<'a>) -> Vec<Node<'a>> {
    let mut cursor = root.walk();
    root.children(&mut cursor)
        .take_while(|c| *c != first_headed)
        .flat_map(|c| {
            // The prelude is itself a `section` node wrapping the
            // pre-heading blocks; descend into it. Top-level blocks
            // outside any section (rare) are walked as-is.
            if c.kind() == "section" {
                let mut inner = c.walk();
                c.children(&mut inner).collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}

/// Prelude rows `ReadmeHeadline` left behind — the substantive blocks
/// above the first heading that the headline's one-lede walk did not
/// take. The headline reads this region but claims at most two blocks
/// of it, and no section range reaches above the first heading, so
/// without this the rest of the lede is unreachable at any budget.
///
/// [`is_prelude_chrome_block`] stays excluded: chrome tokenizes almost
/// entirely as URLs, and it sits at the very top of the README, so
/// buying it displaces the earliest-ranked content in the schedule.
fn prelude_remainder_rows(tree: &Tree, source: &str, headline: &HeadlineSpec) -> Vec<usize> {
    let Some(first_headed) = headed_sections(tree.root_node()).next() else {
        return Vec::new();
    };
    let mut rows: Vec<usize> = Vec::new();
    let mut bytes = 0usize;
    let src_lines: Vec<&str> = source.lines().collect();
    for block in prelude_blocks(tree.root_node(), first_headed) {
        if is_prelude_chrome_block(block, source) {
            continue;
        }
        for row in block.start_position().row..=node_end_row_trimmed(block, source) {
            let row = row + 1;
            if headline.covered_rows.contains(&row) {
                continue;
            }
            let line = src_lines.get(row - 1).copied().unwrap_or_default();
            if line.trim().is_empty() {
                continue;
            }
            bytes += line.len() + 1;
            if bytes > PRELUDE_MAX_BYTES {
                return rows;
            }
            rows.push(row);
        }
    }
    rows
}

/// The chrome/substance line for the whole pre-heading region, stated
/// once: decoration is image/badge-only paragraphs, tag-only HTML
/// wrappers, and in-page nav menus. Everything else above the first
/// heading is substance. Both readers of that region use this —
/// `ReadmeHeadline` and [`prelude_remainder_rows`].
fn is_prelude_chrome_block(block: Node, source: &str) -> bool {
    is_decorative_block(block, source)
        || is_html_nav_block(block, source)
        || (block.kind() == "paragraph" && is_nav_link_paragraph(block, source))
}

/// Top-level `section` children with a heading — skips tree-sitter-md's
/// heading-less prelude `section` wrapper.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;
    use std::path::PathBuf;
    use tree_sitter::Parser;

    fn parse(source: &str) -> Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_md::LANGUAGE.into())
            .unwrap();
        parser.parse(source, None).unwrap()
    }

    /// Chunk bounds over the whole of `source`.
    fn chunk_bounds_of(source: &str) -> Vec<(usize, usize)> {
        let src_lines: Vec<&str> = source.lines().collect();
        let end = src_lines.len();
        oversize_chunk_bounds(&src_lines, 1, end).0
    }

    /// A blank-separated prose paragraph of ~`tokens` tokens.
    fn prose_block(tokens: usize) -> String {
        let mut s = String::new();
        for i in 0..tokens {
            s.push_str(&format!("word{i} "));
        }
        s.push('\n');
        s
    }

    /// The prelude batch takes the lede blocks the headline stopped
    /// short of, and none of the chrome around them.
    #[test]
    fn markdown_prelude_takes_lede_remainder_not_chrome() {
        let source = concat!(
            "<p align=\"center\"><img src=\"logo.png\"></p>\n",
            "\n",
            "<a href=\"https://a\">Docs</a> &nbsp;&middot;&nbsp; <a href=\"https://b\">Demo</a>\n",
            "\n",
            "Widget is a tiny thing.\n",
            "\n",
            "[![build](https://img.shields.io/build.svg)](https://ci.example)\n",
            "\n",
            "It also does the other thing, at length, in a second paragraph.\n",
            "\n",
            "## Install\n",
        );
        let tree = parse(source);
        let spec = headline_spec(&tree, source).expect("headline");
        assert!(spec.covered_rows.contains(&5), "lede row in headline");
        assert_eq!(prelude_remainder_rows(&tree, source, &spec), vec![9]);
    }

    /// A badge wall written as raw HTML inside a markdown paragraph is
    /// decoration, exactly like the markdown-image form.
    #[test]
    fn markdown_raw_html_badge_paragraph_is_decorative() {
        let source = "<a href=\"https://x\"><img src=\"https://b.svg\" alt=\"b\"></a>\n\n# Title\n";
        let tree = parse(source);
        let block = tree.root_node().child(0).and_then(|s| s.child(0)).unwrap();
        assert_eq!(block.kind(), "paragraph");
        assert!(is_decorative_block(block, source));
    }

    #[test]
    fn markdown_oversize_chunks_cover_range_and_stay_disjoint() {
        let source = format!(
            "## Big\n\n{}\n{}\n{}\n",
            prose_block(220),
            prose_block(220),
            prose_block(220)
        );
        let bounds = chunk_bounds_of(&source);
        assert!(bounds.len() >= 2, "expected a split, got {bounds:?}");
        let end = source.lines().count();
        assert_eq!(bounds.first().unwrap().0, 1);
        assert_eq!(bounds.last().unwrap().1, end);
        for pair in bounds.windows(2) {
            assert_eq!(pair[0].1 + 1, pair[1].0, "gap/overlap in {bounds:?}");
        }
    }

    #[test]
    fn markdown_oversize_chunks_split_unbroken_prose_at_safe_row_boundaries() {
        let long_line = "word ".repeat(360);
        let source = std::iter::repeat_n(long_line.as_str(), 12)
            .collect::<Vec<_>>()
            .join("\n");
        let bounds = chunk_bounds_of(&source);
        assert!(bounds.len() > 1, "expected hard splits, got {bounds:?}");
        let end = source.lines().count();
        assert_eq!(bounds.first().unwrap().0, 1);
        assert_eq!(bounds.last().unwrap().1, end);
        for pair in bounds.windows(2) {
            assert_eq!(pair[0].1 + 1, pair[1].0, "gap/overlap in {bounds:?}");
        }
    }

    /// A raw HTML block of CommonMark type 1 (`<script>` / `<pre>` /
    /// `<style>` / `<textarea>`) ends at its closing tag, not at the
    /// first blank line inside it. No chunk boundary may land in its
    /// interior, or a budget that buys only the lede renders a severed
    /// construct.
    #[test]
    fn markdown_oversize_chunks_never_cut_inside_a_raw_html_block() {
        for (open, close) in [("<script>", "</script>"), ("<pre>", "</pre>")] {
            let source = format!(
                "## Big\n\n{}{open}\n{}\n{}\n{close}\n\n{}",
                prose_block(150),
                prose_block(60),
                prose_block(60),
                prose_block(300),
            );
            let open_row = source
                .lines()
                .position(|l| l.trim() == open)
                .expect("open row")
                + 1;
            let close_row = source
                .lines()
                .position(|l| l.trim() == close)
                .expect("close row")
                + 1;
            for (chunk_start, _) in chunk_bounds_of(&source) {
                assert!(
                    !(open_row < chunk_start && chunk_start <= close_row),
                    "{open} block rows {open_row}..={close_row} cut at {chunk_start}",
                );
            }
        }
    }

    /// A line whose backtick run is followed by non-whitespace is not a
    /// closing fence — treating it as one desynchronizes the fence
    /// state and exposes the blank lines after it as cut points.
    #[test]
    fn markdown_oversize_chunks_reject_fence_closers_with_a_trailing_word() {
        let source = format!(
            "## Big\n\n{}```text\nin fence\n``` still-in-fence\n\n{}\n```\n\n{}",
            prose_block(150),
            prose_block(60),
            prose_block(300),
        );
        let open_row = source
            .lines()
            .position(|l| l.trim() == "```text")
            .expect("open row")
            + 1;
        let close_row = source
            .lines()
            .collect::<Vec<_>>()
            .iter()
            .rposition(|l| l.trim() == "```")
            .expect("close row")
            + 1;
        for (chunk_start, _) in chunk_bounds_of(&source) {
            assert!(
                !(open_row < chunk_start && chunk_start <= close_row),
                "fence rows {open_row}..={close_row} cut at {chunk_start}",
            );
        }
    }

    /// A long blank-line-free table has no legal boundary at all, so
    /// only the emergency character cap may split it — the raw-HTML and
    /// fence tracking must not strand it as one unpurchasable lump.
    #[test]
    fn markdown_oversize_chunks_split_a_long_table_only_at_the_hard_cap() {
        let row = format!("| {} | {} |\n", "cell ".repeat(40), "cell ".repeat(40));
        let source = format!("## Big\n\n| a | b |\n| - | - |\n{}", row.repeat(24));
        let bounds = chunk_bounds_of(&source);
        assert!(bounds.len() > 1, "expected hard-cap splits, got {bounds:?}");
        let end = source.lines().count();
        assert_eq!(bounds.first().unwrap().0, 1);
        assert_eq!(bounds.last().unwrap().1, end);
        for pair in bounds.windows(2) {
            assert_eq!(pair[0].1 + 1, pair[1].0, "gap/overlap in {bounds:?}");
        }
    }

    #[test]
    fn markdown_headingless_readme_emits_fallback_at_small_budget() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("README.md"),
            "just two lines of prose without any heading at all.\nsecond line here.\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(dir.path().to_path_buf(), FsWalker, 200, None);
        let rendered = scheduler.run().render();
        assert!(rendered.contains("just two lines of prose"), "{rendered}");
        assert!(rendered.contains("second line here"), "{rendered}");
    }

    #[test]
    fn markdown_headline_bounds_a_giant_unbroken_lede() {
        let long_line = "word ".repeat(400);
        let source = format!(
            "# Giant Doc\n\n{}\n",
            std::iter::repeat_n(long_line.as_str(), 10)
                .collect::<Vec<_>>()
                .join("\n")
        );
        let spans = rendered_spans(&source);
        assert_eq!(spans.len(), 2, "unexpected headline spans: {spans:?}");
        assert_eq!((spans[0].start, spans[0].end), (1, 1));
        assert_eq!((spans[1].start, spans[1].end), (3, 3));
        assert!(matches!(spans[1].render, Render::Truncated { .. }));
    }

    #[test]
    fn markdown_oversize_chunks_never_cut_inside_mismatched_fences() {
        // A ~~~ fence whose body contains a ``` line (markdown
        // tutorial shape), followed by a ```` fence containing a
        // ``` line. Neither inner marker may close its fence, so no
        // cut can land inside either fence.
        let mut fence_a = String::from("~~~\n");
        for i in 0..120 {
            fence_a.push_str(&format!("```line {i} of embedded example\n"));
        }
        fence_a.push_str("~~~\n");
        let mut fence_b = String::from("````\n");
        for i in 0..120 {
            fence_b.push_str(&format!("```inner {i}\n\n"));
        }
        fence_b.push_str("````\n");
        let source = format!(
            "## Big\n\n{fence_a}\n{}\n{fence_b}\n{}",
            prose_block(150),
            prose_block(150)
        );
        let src_lines: Vec<&str> = source.lines().collect();
        let fence_rows: Vec<(usize, usize)> = {
            // Row ranges of the two fences (1-based, inclusive).
            let a_start = 3;
            let a_end = a_start + 121;
            let b_start = src_lines
                .iter()
                .position(|l| l.starts_with("````"))
                .unwrap()
                + 1;
            let b_end = b_start
                + src_lines[b_start..]
                    .iter()
                    .position(|l| l.starts_with("````"))
                    .unwrap()
                + 1;
            vec![(a_start, a_end), (b_start, b_end)]
        };
        for (_, cut_end) in chunk_bounds_of(&source) {
            for &(fs, fe) in &fence_rows {
                assert!(
                    cut_end < fs || cut_end >= fe,
                    "cut at row {cut_end} lands inside fence {fs}..{fe}"
                );
            }
        }
    }

    fn covered(source: &str) -> BTreeSet<usize> {
        let tree = parse(source);
        let spec = headline_spec(&tree, source).expect("headline spec");
        spec.covered_rows
    }

    fn rendered_spans(source: &str) -> Vec<Span> {
        let tree = parse(source);
        let spec = headline_spec(&tree, source).expect("headline spec");
        build_headline_spans(
            &PathBuf::from("README.md"),
            &Source::new(source.into()),
            &spec,
        )
    }

    const HEADLINE_COVERED_CASES: &[(&str, &str, &[usize], &[usize])] = &[
        // anyhow shape: H1 setext, blank, four badge image-link lines as one
        // paragraph, blank, prose paragraph. The headline must skip the
        // badge paragraph.
        (
            "markdown_decorative_paragraph_skipped_anyhow_shape",
            "Anyhow\n\
                   ======\n\
                   \n\
                   [![github](https://example/badge1.svg)](https://example/repo)\n\
                   [![crates.io](https://example/badge2.svg)](https://example/crate)\n\
                   \n\
                   This library provides anyhow::Error, a trait object based error type.\n",
            &[1, 7],
            &[4, 5],
        ),
        // otree shape: H1 + blank + bare-image paragraph + blank + tagline.
        (
            "markdown_decorative_image_only_paragraph_skipped_otree_shape",
            "# OTree - Object Tree TUI Viewer\n\
                   \n\
                   ![screenshot](assets/screenshot.png)\n\
                   \n\
                   A command line tool to view objects in TUI tree widget.\n",
            &[1, 5],
            &[3],
        ),
        // soluna shape: H1 + blank + plain text-link paragraph + blank +
        // prose. Plain text-link paragraphs are not badges and must NOT
        // be skipped.
        (
            "markdown_link_only_paragraph_kept_soluna_shape",
            "# Soluna\n\
                   \n\
                   [Live Examples](https://example/demo)\n\
                   \n\
                   A framework for 2D games in Lua.\n",
            &[1, 3],
            &[],
        ),
        // mitt shape: H1, block_quote tagline, list of features, prose
        // paragraph. Headline includes all four blocks (regression guard).
        (
            "markdown_mitt_shape_unchanged",
            "# Mitt\n\
                   \n\
                   > Tiny 200b functional event emitter / pubsub.\n\
                   \n\
                   - **Microscopic:** weighs less than 200 bytes\n\
                   - **Useful:** wildcard event types\n\
                   \n\
                   Mitt was made for the browser, but works in any JavaScript runtime.\n\
                   \n\
                   ## Table of Contents\n",
            &[1, 3, 5, 6, 8],
            &[10],
        ),
        // Block_quote between H1 and H2 — include block_quote, stop
        // before the H2 sub-section.
        (
            "markdown_blockquote_then_subsection",
            "# Title\n\
                   \n\
                   > Tagline.\n\
                   \n\
                   ## Sub\n\
                   \n\
                   sub body\n",
            &[1, 3],
            &[5, 7],
        ),
        // HTML block with only an `<img>` tag (cmdk-style centered hero).
        // The block is decorative and should be skipped.
        (
            "markdown_decorative_html_block_skipped",
            "# Title\n\
                   \n\
                   <p align=\"center\">\n\
                   <img src=\"hero.png\" />\n\
                   </p>\n\
                   \n\
                   Tagline.\n",
            &[1, 7],
            &[3, 4],
        ),
        // A badge wall broken with inline `<br>` is still decorative.
        (
            "markdown_badge_wall_with_br_is_decorative",
            "# Title\n\
                   \n\
                   [![a](https://e.x/a.svg)](https://e.x)<br>\n\
                   [![b](https://e.x/b.svg)](https://e.x)\n\
                   \n\
                   Tagline.\n",
            &[1, 6],
            &[3, 4],
        ),
        // Plain-text autolink paragraph is NOT decorative.
        (
            "markdown_autolink_paragraph_kept",
            "# Title\n\
                   \n\
                   <https://example.com/docs>\n\
                   \n\
                   Tagline.\n",
            &[3],
            &[],
        ),
        // Multi-language nav paragraph (`[English](url) • ...`) is
        // decorative — the headline must not burn its prelude on it.
        (
            "markdown_nav_link_paragraph_is_decorative",
            "[English](https://e.x/a) • [中文](https://e.x/b) • [Fr](https://e.x/c)\n\
                   \n\
                   # Project\n\
                   \n\
                   Actual lede paragraph.\n",
            &[3, 5],
            &[1],
        ),
    ];

    #[test]
    fn markdown_headline_covered_rows_table() {
        for (row_no, (case, src, present, absent)) in HEADLINE_COVERED_CASES.iter().enumerate() {
            let rows = covered(src);
            for row in *present {
                assert!(
                    rows.contains(row),
                    "{case} row {}: expected source row {row} to be present; rows={rows:?}",
                    row_no + 1
                );
            }
            for row in *absent {
                assert!(
                    !rows.contains(row),
                    "{case} row {}: expected source row {row} to be absent; rows={rows:?}",
                    row_no + 1
                );
            }
        }
    }

    // Nested H1→H2 immediately. Headline must NOT pull the H2 body into
    // itself; only the H1 heading row is covered (exact set, not just membership).
    #[test]
    fn markdown_nested_subsection_not_pulled_in() {
        let src = "# Title\n\
                   \n\
                   ## Sub\n\
                   \n\
                   sub body\n";
        assert_eq!(covered(src).into_iter().collect::<Vec<_>>(), vec![1]);
    }

    /// Short bold tagline followed by a prose lede: the extension
    /// takes the prose paragraph so the headline carries the full
    /// "what is this" snippet (posting shape).
    #[test]
    fn markdown_post_h1_short_tagline_extends_to_prose_lede() {
        let src = "# Posting\n\
                   \n\
                   **A powerful HTTP client that lives in your terminal.**\n\
                   \n\
                   Posting is an HTTP client, not unlike Postman.\n\
                   \n\
                   ## Install\n";
        let rows = covered(src);
        assert!(rows.contains(&1), "H1 missing");
        assert!(rows.contains(&3), "bold tagline missing");
        assert!(
            rows.contains(&5),
            "prose-lede paragraph must be picked up by extension"
        );
    }

    /// A long first paragraph (stripped content > tagline threshold)
    /// is the lede itself — the extension must NOT pull in a
    /// second paragraph.
    #[test]
    fn markdown_post_h1_long_first_paragraph_no_extension() {
        let src = "# D2TS\n\
                   \n\
                   D2TS is a TypeScript implementation of differential dataflow with a long prose lede that runs past the tagline threshold.\n\
                   \n\
                   A second paragraph the headline must NOT pull in.\n\
                   \n\
                   ## Install\n";
        let rows = covered(src);
        assert!(rows.contains(&1), "H1 missing");
        assert!(rows.contains(&3), "first paragraph missing");
        assert!(!rows.contains(&5), "second paragraph must NOT be pulled in");
    }

    /// Sub-section heading (`### Usage`) is NOT the project's
    /// title — extension must not fire under non-H1 first-headed
    /// sections (superstruct shape).
    #[test]
    fn markdown_non_h1_first_section_no_extension() {
        let src = "<p>tagline html block</p>\n\
                   \n\
                   ### Usage\n\
                   \n\
                   Short body para.\n\
                   \n\
                   A second body para that must NOT land in headline.\n";
        let rows = covered(src);
        assert!(rows.contains(&3), "heading missing");
        assert!(rows.contains(&5), "first paragraph missing");
        assert!(
            !rows.contains(&7),
            "second paragraph must NOT be pulled in under non-H1 first heading"
        );
    }

    // --- HeadingsOutline tests ---

    fn outline_rows(file: &str, source: &str) -> Vec<(usize, usize)> {
        let tree = parse(source);
        derive_outline_gates(&PathBuf::from(file), &tree, source).rows
    }

    /// README with H1 + 4 H2s. Outline collects only the H2 rows; the
    /// H1 row stays under `ReadmeHeadline`.
    #[test]
    fn markdown_outline_strips_readme_h1() {
        let src = "# Project\n\
                   \n\
                   Tagline paragraph.\n\
                   \n\
                   ## Install\n\
                   \n\
                   prose\n\
                   \n\
                   ## Usage\n\
                   \n\
                   prose\n\
                   \n\
                   ## API\n\
                   \n\
                   prose\n\
                   \n\
                   ## License\n\
                   \n\
                   prose\n";
        let rows = outline_rows("README.md", src);
        let starts: Vec<usize> = rows.iter().map(|(s, _)| *s).collect();
        assert_eq!(
            starts,
            vec![5, 9, 13, 17],
            "outline must collect only H2 rows"
        );
    }

    /// Setext H2 (`-----` underline) covers two source rows; outline
    /// includes both.
    #[test]
    fn markdown_outline_setext_h2_two_rows() {
        let src = "# Project\n\
                   \n\
                   Tagline.\n\
                   \n\
                   First section\n\
                   -------------\n\
                   \n\
                   body\n\
                   \n\
                   Second section\n\
                   --------------\n\
                   \n\
                   body\n";
        let rows = outline_rows("README.md", src);
        assert_eq!(
            rows,
            vec![(5, 6), (10, 11)],
            "setext H2 outline rows include the underline"
        );
    }

    /// File with only H4 headings. `heading_level` returns 4, so the
    /// outline collects no rows — emission is skipped, downstream
    /// `Section` batches keep their existing predecessor.
    #[test]
    fn markdown_outline_h4_only_collects_nothing() {
        let src = "#### Subsubsection A\n\
                   \n\
                   body A\n\
                   \n\
                   #### Subsubsection B\n\
                   \n\
                   body B\n";
        let rows = outline_rows("docs/page.md", src);
        assert!(
            rows.is_empty(),
            "outline must not include H4+; got {rows:?}"
        );
    }

    // --- logical_sections tests ---

    fn sections(file: &str, source: &str) -> Vec<SectionRange> {
        let tree = parse(source);
        let file = PathBuf::from(file);
        let gates = derive_outline_gates(&file, &tree, source);
        logical_sections(&file, &tree, source, gates.emits)
    }

    #[test]
    fn markdown_reference_usage_section_flagged() {
        // (title, body, expect_flag_on_some_whole_range)
        let cases: &[(&str, &str, bool)] = &[
            // Reference title + list body → flagged.
            ("## Options", "- `-a` first\n- `-b` second\n", true),
            // Reference title + table body → flagged.
            (
                "## Environment Variables",
                "| Name | Meaning |\n|------|---------|\n| `X` | a thing |\n",
                true,
            ),
            // Reference title but prose-only body (no list/table/code)
            // → NOT flagged (enclosed's pointer shape).
            (
                "## Configuration",
                "See the configuration docs for details.\n",
                false,
            ),
            // Reference title but body over the compact cap → NOT
            // flagged (large `## Usage`-style blob).
            (
                "## Features",
                &("- bullet of moderately long reference text here\n".repeat(40)),
                false,
            ),
            // Non-vocabulary title with a list body → NOT flagged.
            ("## Random Notes", "- one\n- two\n", false),
            // Bare `## Usage` is deliberately excluded from the broad
            // vocab (canonical path handles code-dominant usage).
            ("## Usage", "- run it\n- profit\n", false),
        ];
        for (title, body, expect) in cases {
            let src = format!("# Project\n\nTagline.\n\n{title}\n\n{body}");
            let ranges = sections("README.md", &src);
            let any_flagged = ranges.iter().any(|r| r.is_reference_usage_section);
            assert_eq!(
                any_flagged, *expect,
                "title={title:?} body={body:?} expected flag={expect}, got {any_flagged}; ranges={ranges:?}"
            );
        }
    }

    /// `# Title` README's first real H2 must get readme-index decay
    /// factor 1.0 (i.e. be unscaled): the synthetic H1-unwrap intro
    /// shares index 0 with it rather than pushing "## Install" to index
    /// 1 (~0.90).
    #[test]
    fn markdown_readme_index_decay_skips_synthetic_intro() {
        let src = "# Title\n\nTagline.\n\n## Install\n\nbody\n\n## Use\n\nbody\n";
        let ranges = sections("README.md", src);
        // Three ranges: synthetic intro, ## Install, ## Use.
        let indices: Vec<usize> = ranges.iter().map(|r| r.h2_index).collect();
        assert_eq!(indices, vec![0, 0, 1], "got {ranges:?}");
        assert_eq!(readme_index_decay(&ranges[1]), 1.0);
        let f = readme_index_decay(&ranges[2]);
        assert!(
            (0.7..1.0).contains(&f),
            "second real H2 should decay; got {f}"
        );
    }

    /// READMEs without an H1 wrap (no synthetic intro) — first H2 is
    /// index 0 and gets factor 1.0 directly.
    #[test]
    fn markdown_readme_index_decay_no_synthetic_intro() {
        let src = "## Install\n\nbody\n\n## Use\n\nbody\n";
        let ranges = sections("README.md", src);
        let indices: Vec<usize> = ranges.iter().map(|r| r.h2_index).collect();
        assert_eq!(indices, vec![0, 1]);
        assert_eq!(readme_index_decay(&ranges[0]), 1.0);
    }

    /// AGENTS.md / CLAUDE.md / skill bodies are already loaded into
    /// the model's context by the harness, so emitting their per-H2
    /// `Section` batches is pure waste. The walker must skip
    /// `MarkdownKey::Section` for these files while still emitting structural batches
    /// (`HeadingsOutline`, `ReadmeHeadline`) so file shape stays
    /// discoverable at large budgets.
    #[test]
    fn markdown_walker_suppresses_body_for_auto_injected_docs() {
        use std::fs;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let body = "# Project AGENTS\n\n\
                    Top-level instructions go here.\n\n\
                    ## Setup\n\nrun `cargo build`.\n\n\
                    ## Conventions\n\nuse rustfmt.\n";
        fs::write(root.join("AGENTS.md"), body).unwrap();
        fs::write(root.join("notes.md"), body).unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);

        let agents_md = root.join("AGENTS.md");
        let notes_md = root.join("notes.md");
        let agents_keys: Vec<_> = batches
            .iter()
            .filter_map(|b| match &b.key {
                BatchKey::Markdown(k) => match k {
                    MarkdownKey::Section { file, .. } if file == &agents_md => Some("section"),
                    MarkdownKey::HeadingsOutline { file } if file == &agents_md => Some("outline"),
                    MarkdownKey::ReadmeHeadline { file } if file == &agents_md => Some("headline"),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert!(
            !agents_keys.contains(&"section"),
            "AGENTS.md must not emit Section batches; got {agents_keys:?}",
        );

        // The companion notes.md (same content, normal filename) must
        // still produce Section batches — the suppression is targeted,
        // not blanket.
        let notes_has_section = batches.iter().any(|b| {
            matches!(
                &b.key,
                BatchKey::Markdown(MarkdownKey::Section { file, .. }) if file == &notes_md
            )
        });
        assert!(
            notes_has_section,
            "control file notes.md should still emit Section batches",
        );
    }

    /// The root README's `HeadingsOutline` is the hard predecessor of
    /// every section in that file, so the dense-siblings damp must never
    /// reach it however crowded the root directory is — damping it would
    /// delay the whole README body, the largest credited early-budget
    /// purchase on repos that have one. A README at depth carries no such
    /// stream and does take the damp.
    #[test]
    fn markdown_dense_siblings_never_damp_the_root_readme() {
        use std::fs;

        let body = "# Project\n\nA tagline paragraph about the project.\n\n\
                    Some further prelude prose worth a batch.\n\n\
                    ## Install\n\nrun it.\n\n\
                    ## Use\n\nuse it.\n\n\
                    ## Configure\n\nconfigure it.\n";
        let crowd = ["a.md", "b.md", "c.md", "d.md", "e.md", "f.md", "g.md"];

        // Two roots differing only in how many .md siblings sit beside
        // the README — sparse (under the threshold) and crowded.
        let build = |extra: &[&str]| {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().to_path_buf();
            fs::write(root.join("README.md"), body).unwrap();
            let docs = root.join("docs");
            fs::create_dir(&docs).unwrap();
            fs::write(docs.join("README.md"), body).unwrap();
            for n in extra {
                fs::write(root.join(n), body).unwrap();
                fs::write(docs.join(n), body).unwrap();
            }
            let ctx = WalkCtx::new(root.clone());
            let batches: Vec<_> = expand_in_dir(&root, &ctx)
                .into_iter()
                .chain(expand_in_dir(&docs, &ctx))
                .collect();
            // `dir` owns the temp tree; keep it alive past the walk.
            (dir, root, batches)
        };
        let (_sparse_guard, sparse_root, sparse) = build(&[]);
        let (_dense_guard, dense_root, dense) = build(&crowd);

        // Every nav batch either walk emitted for `file`, keyed by kind.
        // Gathered rather than looked up by key so the assertions cover
        // whichever of the three this README shape produces.
        let nav_values = |batches: &[Batch<BatchKey>], file: &Path| {
            let mut found: Vec<(&'static str, f64)> = batches
                .iter()
                .filter_map(|b| {
                    let kind = match &b.key {
                        BatchKey::Markdown(MarkdownKey::HeadingsOutline { file: f })
                            if f == file =>
                        {
                            "outline"
                        }
                        BatchKey::Markdown(MarkdownKey::ReadmeHeadline { file: f })
                            if f == file =>
                        {
                            "headline"
                        }
                        BatchKey::Markdown(MarkdownKey::Prelude { file: f }) if f == file => {
                            "prelude"
                        }
                        _ => return None,
                    };
                    Some((kind, b.value))
                })
                .collect();
            found.sort_by_key(|(kind, _)| *kind);
            found
        };

        let sparse_root_nav = nav_values(&sparse, &sparse_root.join("README.md"));
        let dense_root_nav = nav_values(&dense, &dense_root.join("README.md"));
        assert!(
            sparse_root_nav.iter().any(|(k, _)| *k == "outline"),
            "test shape must emit a root README outline to pin",
        );
        assert_eq!(
            sparse_root_nav,
            dense_root_nav,
            "root README nav must price identically however many .md siblings the root \
             holds (1 vs {})",
            1 + crowd.len(),
        );

        // The damp is real, not vacuous: the same batches on a nested
        // README do move once its directory is crowded.
        let sparse_docs_nav = nav_values(&sparse, &sparse_root.join("docs/README.md"));
        let dense_docs_nav = nav_values(&dense, &dense_root.join("docs/README.md"));
        assert_eq!(sparse_docs_nav.len(), dense_docs_nav.len());
        for ((kind, sparse_value), (_, dense_value)) in sparse_docs_nav.iter().zip(&dense_docs_nav)
        {
            assert!(
                dense_value < sparse_value,
                "nested README {kind} must take the dense-siblings damp \
                 ({sparse_value} sparse vs {dense_value} crowded)",
            );
        }
    }

    /// Overline-form headings must not leak their overline punctuation row
    /// into the *preceding* section's span: a section bounds at the next
    /// heading's `start_row - 1` (the overline row), not its title row.
    #[test]
    fn walker_markdown_rst_overline_section_excludes_next_overline_row() {
        let src = "\
======
Title
======

Intro prose for the title.

========
Overview
========

Overview prose paragraph one.

=======
Details
=======

Details prose paragraph one.
";
        let src_lines: Vec<&str> = src.lines().collect();
        let headings = scan_rst_headings(&src_lines);
        // title + Overview + Details
        assert_eq!(headings.len(), 3);
        let sections = rst_readme(src).1;
        assert_eq!(sections.len(), 3, "intro + Overview + Details");
        for section in &sections {
            let own_start = Some(section.start);
            for row in section.start..=section.end {
                // A section legitimately starts at its own overline row;
                // it must not contain any *other* heading's overline row.
                let foreign_overline = headings
                    .iter()
                    .any(|h| h.start_row == row && Some(row) != own_start);
                assert!(
                    !foreign_overline,
                    "section row {row} landed on another heading's overline punctuation"
                );
            }
        }
    }

    /// A non-ASCII tagline whose UTF-8 byte length exceeds the tagline
    /// max but whose char count is under it must measure as short — length
    /// is counted in chars, not bytes.
    #[test]
    fn walker_markdown_strip_block_for_length_counts_chars_not_bytes() {
        // 40 CJK chars = 120 bytes (> 90), but 40 chars (< 90). Wrapped in
        // bold markup the stripper must remove.
        let tagline: String = "字".repeat(40);
        let raw = format!("**{tagline}**");
        assert!(raw.len() > 90, "fixture must exceed byte threshold");
        let stripped = strip_block_for_length(&raw);
        assert_eq!(
            stripped.chars().count(),
            40,
            "char count must ignore markup and multi-byte width"
        );
        assert!(
            stripped.chars().count() <= HEADLINE_TAGLINE_MAX_CHARS,
            "tagline is short by char count despite exceeding byte threshold"
        );
    }
}
