//! README walker: the one root README (see [`root_readme`]). Markdown is
//! parsed with `tree-sitter-md`'s block grammar; reST and AsciiDoc (and an
//! extensionless README without ATX headings) are line-scanned into the
//! same shapes (see [`line_scanned_readme`]). Every other document is left
//! to the listing, which names it.
//!
//! In document order:
//! - `ReadmeHeadline` — the first heading plus the lede: the first
//!   substantive block before it and the blocks under it through the
//!   first paragraph, stepping over chrome (badges, logos, nav rows).
//! - `Prelude` — the rest of the text above the first heading, chrome
//!   excluded. Predecessor: the headline.
//! - `HeadingsOutline` — every H1–H3 heading row the headline doesn't
//!   cover (H1–H2 when that is too many), when there are
//!   2..=[`MAX_OUTLINE_HEADINGS`] of them.
//!   Predecessor: the headline.
//! - `CommandBlock`s — per top-level section, the heading and leading
//!   shell blocks of a build/test/run section inside it (see
//!   [`command_block`]). Predecessor: the outline, else the headline.
//! - `Section`s — one per top-level H2 (an H1-only document unwraps to
//!   an intro plus its H2s); an oversize section splits into a head
//!   chunk plus chained `OversizeTail` chunks. Predecessor: the
//!   section's command block, else the outline, else the headline.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, MarkdownKey};
use crate::content::BatchContent;
use crate::fs_util::{EntryKind, list_dir};
use crate::render::{Source, visible_full_line};
use crate::tokenizer;

use super::{
    WalkCtx, budget_chunk_ranges, first_child_of_kind, node_end_row_trimmed,
    single_file_lines_content,
};

mod line_scanned;
use line_scanned::line_scanned_readme;

/// Upper bound on collectable heading rows before `HeadingsOutline`
/// suppresses itself — the outline predecesses every section, so an
/// oversize outline would block the whole file.
const MAX_OUTLINE_HEADINGS: usize = 30;

/// Per-chunk token target for the oversize head-split; a section at or
/// above it is split. NSes are authored to a growth envelope
/// (`cost ≤ 100 + 0.3·cumulative`, see `src/ns_simulate.rs`), so an
/// early-rankable batch is ~100–400 tokens; a lump beyond that cannot
/// win the early purchase race no matter its value. Low enough that a
/// fence-heavy chunk pair doesn't overshoot it before the first cut
/// candidate; measured in tokens because code tokenizes denser per
/// byte than prose.
const OVERSIZE_CHUNK_TARGET_TOKENS: usize = 300;

/// Token target for the *first* chunk of a head-split section — the
/// section lede, so a section's entry price is its opening paragraph's
/// rather than a generic chunk's. Both halves price off the section's
/// own value, undiscounted: splitting the value by row share would make
/// the ratio scale as `cost^(1-k)`, i.e. make entry strictly worse.
const LEDE_TARGET_TOKENS: usize = 140;

/// Minimum tokens that must remain after a cut — a smaller remainder
/// folds into the current chunk instead of spawning a micro-tail.
const OVERSIZE_CHUNK_MIN_TAIL_TOKENS: usize = 100;

/// Emergency source-character ceiling between natural blank-line cuts.
/// Long unbroken paragraphs may have no structural split point at all;
/// safe row boundaries outside fences keep those ranges purchasable.
const OVERSIZE_HARD_CHUNK_CHAR_CAP: usize = 4_096;

/// A lede block larger than this stays out of `ReadmeHeadline`, so a
/// pathological README paragraph cannot turn it into a multi-megabyte
/// batch.
const HEADLINE_BLOCK_BYTE_GATE: usize = 16 * 1024;

/// Maximum character count for the stripped content of a "tagline"
/// block, beyond which the extension is suppressed because the block
/// is itself the substantive lede.
const HEADLINE_TAGLINE_MAX_CHARS: usize = 90;

/// Source-byte ceiling on the `Prelude` batch. The hero region of a
/// normal README is well under this; the cap only stops a README that
/// puts its whole body above the first heading from shipping as one
/// unsplittable early-budget lump.
const PRELUDE_MAX_BYTES: usize = 2_500;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let mut out = Vec::new();
    if dir != ctx.root() {
        return out;
    }
    let Some((file, markup)) = root_readme(dir, ctx) else {
        return out;
    };
    // reST and AsciiDoc: line-scanned headings, no tree-sitter parse.
    if markup != ReadmeMarkup::Markdown {
        let Some(source) = ctx.read_source(&file) else {
            return out;
        };
        let (headline, ranges) = line_scanned_readme(&source, markup);
        let headline_emitted = push_headline(&mut out, &file, &source, &headline);
        push_sections(&mut out, &file, &source, &ranges, None, headline_emitted);
        return out;
    }

    let Some((source, tree)) = ctx.parse_tree(&file, &tree_sitter_md::LANGUAGE.into()) else {
        return out;
    };
    let headline = headline_rows(&tree, &source);
    let outline_rows = outline_rows(&tree, &source, headline.as_ref());
    let outline_emits = !outline_rows.is_empty();

    let headline_emitted = headline
        .as_ref()
        .and_then(|spec| push_headline(&mut out, &file, &source, spec));
    if let Some(spec) = &headline {
        let rows = prelude_remainder_rows(&tree, &source, spec);
        if let Some(content) = single_file_lines_content(&file, &source, rows) {
            out.push(Batch {
                key: MarkdownKey::Prelude { file: file.clone() }.into(),
                predecessor: headline_emitted.clone(),
                content,
                value: README_SECTION_VALUE,
            });
        }
    }
    let mut outline_emitted: Option<BatchKey> = None;
    let outline_content = single_file_lines_content(
        &file,
        &source,
        outline_rows
            .iter()
            .flat_map(|&(start, end)| start..=end)
            .collect(),
    );
    if let Some(content) = outline_content {
        let key = MarkdownKey::HeadingsOutline { file: file.clone() };
        out.push(Batch {
            key: key.clone().into(),
            predecessor: headline_emitted.clone(),
            content,
            value: HEADINGS_OUTLINE_VALUE,
        });
        outline_emitted = Some(BatchKey::Markdown(key));
    }

    push_sections(
        &mut out,
        &file,
        &source,
        &logical_sections(&tree, &source, outline_emits),
        headline.as_ref(),
        outline_emitted.or(headline_emitted),
    );
    out
}

/// Emit the `ReadmeHeadline` batch over `rows`, returning its key.
fn push_headline(
    out: &mut Vec<Batch>,
    file: &Path,
    source: &Source,
    rows: &BTreeSet<usize>,
) -> Option<BatchKey> {
    let content = single_file_lines_content(file, source, rows.iter().copied().collect())?;
    let key = BatchKey::from(MarkdownKey::ReadmeHeadline {
        file: file.to_path_buf(),
    });
    out.push(Batch {
        key: key.clone(),
        predecessor: None,
        content,
        value: README_HEADLINE_VALUE,
    });
    Some(key)
}

/// Emit one `Section` batch per range, each head preceded by its
/// `CommandBlock` when it has one past the headline. A head gates on its
/// command block, else on `section_predecessor`; oversize tails deliver
/// in source order behind the chunk before them.
fn push_sections(
    out: &mut Vec<Batch>,
    file: &Path,
    source: &Source,
    ranges: &[SectionRange],
    headline: Option<&BTreeSet<usize>>,
    section_predecessor: Option<BatchKey>,
) {
    let headline_end = headline.and_then(|spec| spec.iter().next_back().copied());
    let mut chain_key = section_predecessor.clone();
    for (idx, range) in ranges.iter().enumerate() {
        if !range.chained_to_previous {
            chain_key = section_predecessor.clone();
        }
        if let Some(CommandBlockRows { heading, block }) = range.command_block
            && headline_end.is_none_or(|end| end < block.0)
            && let Some(content) = single_file_lines_content(
                file,
                source,
                (heading.0..=heading.1).chain(block.0..=block.1).collect(),
            )
        {
            let key = BatchKey::from(MarkdownKey::CommandBlock {
                file: file.to_path_buf(),
                row: block.0,
            });
            out.push(Batch {
                key: key.clone(),
                predecessor: chain_key.replace(key),
                content,
                value: README_SECTION_VALUE,
            });
        }
        if let Some(content) = build_section_content(file, source, range, headline) {
            let key = BatchKey::from(MarkdownKey::Section {
                file: file.to_path_buf(),
                section_index: idx,
                keeps_default_concavity: range.is_reference_usage_section,
            });
            out.push(Batch {
                key: key.clone(),
                predecessor: chain_key.replace(key),
                content,
                value: section_value(range),
            });
        }
    }
}

// --- value ---

const README_HEADLINE_VALUE: f64 = 2616.0;

const HEADINGS_OUTLINE_VALUE: f64 = 974.0;

/// Value of a section before its range's own factors, and of the
/// `Prelude`, which is the top of the README body just above the first
/// heading.
const README_SECTION_VALUE: f64 = 1181.0;

/// Boost for README usage/reference sections (see
/// [`SectionRange::is_reference_usage_section`]) so they clear the
/// early budget instead of sinking below the README index decay.
const REFERENCE_USAGE_SECTION_FACTOR: f64 = 1.3;

/// Index decay for README sections: `(idx + 1)^-0.15`, counting from the
/// first real H2.
fn readme_index_decay(range: &SectionRange) -> f64 {
    (range.h2_index as f64 + 1.0).powf(-0.15)
}

fn section_value(range: &SectionRange) -> f64 {
    let mut value = README_SECTION_VALUE * readme_index_decay(range);
    if range.is_reference_usage_section {
        value *= REFERENCE_USAGE_SECTION_FACTOR;
    }
    value
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

/// Heading row ranges for `HeadingsOutline` — levels 1–3, or 1–2 when
/// that is too many, with any headline-covered headings dropped — or
/// none when there are fewer than two or more than
/// [`MAX_OUTLINE_HEADINGS`] of them.
fn outline_rows(
    tree: &Tree,
    source: &str,
    headline: Option<&BTreeSet<usize>>,
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
        if headline.is_some_and(|spec| spec.range(start_row..=end_row).next().is_some()) {
            continue;
        }
        out.push((level, start_row, end_row));
    }
    if out.len() > MAX_OUTLINE_HEADINGS {
        out.retain(|&(level, _, _)| level <= 2);
    }
    if !(2..=MAX_OUTLINE_HEADINGS).contains(&out.len()) {
        out.clear();
    }
    out.into_iter()
        .map(|(_, start, end)| (start, end))
        .collect()
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

fn build_section_content(
    file: &Path,
    source: &Source,
    range: &SectionRange,
    headline: Option<&BTreeSet<usize>>,
) -> Option<BatchContent> {
    let (start, end) = (range.start, range.end);

    // A README section starts past the last row `ReadmeHeadline`
    // covers — else those rows' marginal cost goes to 0 and
    // `ratio(value, 0) = ∞`. (`headline` is `Some` only for README.md.)
    // Rows the headline stepped *over* (chrome) are dropped with them.
    let effective_start = match headline.and_then(|spec| spec.iter().next_back()) {
        Some(max_row) => (max_row + 1).max(start),
        None => start,
    };
    if effective_start > end {
        return None;
    }

    let lines: Vec<usize> = (effective_start..=end).collect();
    single_file_lines_content(file, source, lines)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ReadmeMarkup {
    Markdown,
    Rst,
    AsciiDoc,
}

/// The root README, preferring Markdown, then reST, then AsciiDoc, and a
/// name with an extension over a bare `README` of the same markup. A
/// document precis was pointed at directly reads as its README. An
/// extensionless `README` is Markdown when it has an ATX heading, else
/// it is scanned for reST-style underlined headings.
fn root_readme(dir: &Path, ctx: &WalkCtx) -> Option<(PathBuf, ReadmeMarkup)> {
    let single_file = ctx.dir_filter().named_file().is_some();
    list_dir(dir, ctx.dir_filter())
        .iter()
        .filter(|(_, kind)| matches!(kind, EntryKind::File))
        .filter_map(|(name, _)| {
            let (stem, extension) = name.split_once('.').unwrap_or((name, ""));
            if !single_file && !stem.eq_ignore_ascii_case("readme") {
                return None;
            }
            let file = dir.join(name);
            let markup = match extension.to_ascii_lowercase().as_str() {
                "md" | "markdown" | "mkdn" | "mdown" => ReadmeMarkup::Markdown,
                "rst" => ReadmeMarkup::Rst,
                "adoc" | "asciidoc" | "asc" => ReadmeMarkup::AsciiDoc,
                "" if !single_file => {
                    let source = ctx.read_source(&file)?;
                    if source.lines().any(|line| line.starts_with("# ")) {
                        ReadmeMarkup::Markdown
                    } else {
                        ReadmeMarkup::Rst
                    }
                }
                _ => return None,
            };
            Some((file, markup))
        })
        .min_by_key(|(file, markup)| (*markup, file.extension().is_none()))
}

/// Lowercased leading alphanumeric/whitespace run of a heading title.
fn title_core(title: &str) -> String {
    let core: String = title
        .trim()
        .to_ascii_lowercase()
        .chars()
        .skip_while(|c| !c.is_ascii_alphanumeric())
        .take_while(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect();
    core.trim().to_string()
}

// --- headline ---

/// Block kinds that bound a section — `ReadmeHeadline` never reaches
/// past these.
fn is_section_boundary(kind: &str) -> bool {
    matches!(kind, "section" | "atx_heading" | "setext_heading")
}

fn headline_rows(tree: &Tree, source: &str) -> Option<BTreeSet<usize>> {
    let section = headed_sections(tree.root_node()).next()?;
    let heading = first_heading_child(section)?;

    let mut spec = BTreeSet::new();
    // Prelude content: README opens with HTML title blocks / badges /
    // lede paragraph before the first heading.
    extend_lede(
        &mut spec,
        &prelude_blocks(tree.root_node(), section),
        source,
        true,
    );
    extend_rows_inclusive(&mut spec, heading, source);
    let heading_first_row = heading.start_position().row + 1;

    // The "tagline + lede" extension only fires under the project's
    // title heading (H1). For non-H1 first-headed sections (`### Usage`,
    // `## About`) the first substantive paragraph IS the section body
    // and shouldn't pull in further content.
    let post: Vec<Node> = children_after(section, heading);
    extend_lede(&mut spec, &post, source, heading_level(heading) == 1);

    debug_assert!(
        spec.contains(&heading_first_row),
        "headline covered_rows missing heading row"
    );
    Some(spec)
}

/// Append every 1-based row covered by `node` to `out`, trimming a
/// trailing newline tree-sitter-md sometimes includes in a node's span.
fn extend_rows_inclusive(out: &mut BTreeSet<usize>, node: Node, source: &str) {
    let last = node_end_row_trimmed(node, source);
    for row in node.start_position().row..=last {
        out.insert(row + 1);
    }
}

fn extend_headline_block_rows(rows: &mut BTreeSet<usize>, node: Node, source: &str) {
    if node.end_byte() - node.start_byte() <= HEADLINE_BLOCK_BYTE_GATE {
        rows.extend(block_text_rows(node, source));
    }
}

/// 1-based rows of `block` that carry text. An HTML block keeps only
/// the rows with a letter or digit outside its tags and comments, so a
/// centered title keeps its heading row but not the logo and badge rows
/// wrapped in the same element.
fn block_text_rows(block: Node, source: &str) -> Vec<usize> {
    let first_row = block.start_position().row;
    let last_row = node_end_row_trimmed(block, source);
    if block.kind() != "html_block" {
        return (first_row + 1..=last_row + 1).collect();
    }
    let mut rows = Vec::new();
    let mut in_tag = false;
    let text = &source[block.start_byte()..block.end_byte()];
    for (offset, line) in text.lines().enumerate().take(last_row - first_row + 1) {
        let mut has_text = false;
        for c in strip_html_entities(line).chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                c if !in_tag && c.is_alphanumeric() => has_text = true,
                _ => {}
            }
        }
        if has_text {
            rows.push(first_row + offset + 1);
        }
    }
    rows
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
            let Some(link_text) = first_child_of_kind(node, "link_text") else {
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

/// Paragraph is decorative iff every named inline child is decorative
/// or skippable, at least one is decorative, and no plain text sits
/// between or around them. Plain-text paragraphs return `false`.
fn is_decorative_paragraph(para: Node, source: &str) -> bool {
    let Some(inline_block) = first_child_of_kind(para, "inline") else {
        return false;
    };
    let inline_text = &source[inline_block.start_byte()..inline_block.end_byte()];
    let Some(tree) = parse_inline(inline_text) else {
        return false;
    };
    let root = tree.root_node();
    let mut cur = root.walk();
    let mut any_decorative = false;
    let mut cursor = 0;
    for node in root.children(&mut cur).filter(|c| c.is_named()) {
        if is_decorative_inline(node, inline_text) {
            any_decorative = true;
        } else if !is_skippable_inline(node, inline_text) {
            return false;
        }
        if !inline_text[cursor..node.start_byte()].trim().is_empty() {
            return false;
        }
        cursor = node.end_byte();
    }
    any_decorative && inline_text[cursor..].trim().is_empty()
}

fn is_decorative_block(block: Node, source: &str) -> bool {
    match block.kind() {
        "paragraph" => is_decorative_paragraph(block, source),
        // `[label]: url` definitions render nothing on their own.
        "link_reference_definition" => true,
        "html_block" => is_decorative_html_block(block, source),
        _ => false,
    }
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

/// A navigation row: three or more links and nothing else but separator
/// punctuation (`•`, `·`, `|`, `/`, `,`, dashes) — multi-language
/// READMEs' `[English](url) • [中文](url) • ...`.
fn is_nav_link_paragraph(para: Node, source: &str) -> bool {
    let Some(inline_block) = first_child_of_kind(para, "inline") else {
        return false;
    };
    let inline_text = &source[inline_block.start_byte()..inline_block.end_byte()];
    let Some(tree) = parse_inline(inline_text) else {
        return false;
    };
    let root = tree.root_node();
    let mut cur = root.walk();
    let named: Vec<Node> = root
        .children(&mut cur)
        .filter(|c| c.is_named() && !is_skippable_inline(*c, inline_text))
        .collect();
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

/// True iff the headline block reads as a "tagline" (a bold one-liner,
/// a centered `<h1>`) — short enough that the next non-decorative block
/// is plausibly the prose lede. Measured on stripped text, not rows: a
/// single long sentence is the lede itself, not a tagline preceding one.
fn is_short_substantive_block(block: Node, source: &str) -> bool {
    let raw = &source[block.start_byte()..block.end_byte()];
    let stripped = strip_block_for_length(raw);
    stripped.chars().count() <= HEADLINE_TAGLINE_MAX_CHARS
}

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
    /// [`is_reference_usage_section`]. Earns
    /// [`REFERENCE_USAGE_SECTION_FACTOR`] and the default concavity.
    is_reference_usage_section: bool,
    /// Oversize tail chunks gate on the previous chunk so the section
    /// delivers as an in-order prefix.
    chained_to_previous: bool,
    /// The section's [`command_block`]; on its first chunk only.
    command_block: Option<CommandBlockRows>,
}

/// 1-based `(first, last)` rows of a command section's heading and of its
/// shell blocks (see [`command_block`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CommandBlockRows {
    heading: (usize, usize),
    block: (usize, usize),
}

impl SectionRange {
    fn new(start: usize, end: usize, h2_index: usize) -> Self {
        Self {
            start,
            end,
            h2_index,
            is_reference_usage_section: false,
            chained_to_previous: false,
            command_block: None,
        }
    }
}

/// Section ranges for batching: one per top-level entry — or, for a
/// headingless file, one for its whole text — head-split when oversize.
fn logical_sections(tree: &Tree, source: &str, outline_emits: bool) -> Vec<SectionRange> {
    let src_lines: Vec<&str> = source.lines().collect();
    let entries = top_level_entries(tree.root_node(), source);
    let mut out = Vec::with_capacity(entries.len());
    if entries.is_empty() {
        let nonblank = |line: &&str| !line.trim().is_empty();
        if let (Some(start), Some(end)) = (
            src_lines.iter().position(nonblank),
            src_lines.iter().rposition(nonblank),
        ) {
            push_whole_or_head_split(
                &mut out,
                &src_lines,
                SectionRange::new(start + 1, end + 1, 0),
            );
        }
        return out;
    }
    // The first real H2 is index 0; an H1-unwrap intro shares it.
    let h2_offset = usize::from(matches!(
        entries.first(),
        Some(TopLevelEntry::SyntheticIntro { .. })
    ));

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
            TopLevelEntry::Section { node, start, end } => {
                if is_appendix_title_core(&section_title_core(*node, source)) {
                    continue;
                }
                let reference_h2 = is_reference_usage_section(*node, source);
                // Back-matter subsections closing the section are dropped too.
                let subsections: Vec<Node> = headed_sections(*node).collect();
                let first_appendix = subsections
                    .iter()
                    .rev()
                    .take_while(|sub| is_appendix_title_core(&section_title_core(**sub, source)))
                    .last()
                    .copied();
                if first_appendix.is_some()
                    && section_body(*node, first_appendix, source)
                        .trim()
                        .is_empty()
                {
                    continue;
                }
                let end = first_appendix.map_or(*end, |first| first.start_position().row);
                push_whole_or_head_split(
                    &mut out,
                    &src_lines,
                    SectionRange {
                        is_reference_usage_section: reference_h2,
                        command_block: command_block(*node, None, source, &src_lines),
                        ..SectionRange::new(*start, end, h2_idx)
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
/// then refined into a lede plus body.
fn oversize_chunk_bounds(src_lines: &[&str], start: usize, end: usize) -> Vec<(usize, usize)> {
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
    ranges
        .into_iter()
        .map(|range| (start + range.start, start + range.end - 1))
        .collect()
}

/// Per-row token count (row is 1-based; includes the newline). Rides
/// the tokenizer's per-line memoization, so repeated sweeps are cheap.
fn row_tokens(src_lines: &[&str], row: usize) -> usize {
    src_lines
        .get(row - 1)
        .map(|l| tokenizer::count(&format!("{}\n", visible_full_line(l))))
        .unwrap_or(0)
}

/// The fence delimiter at the start of an already-trimmed line — its
/// marker char and run length (≥ 3) — if any.
fn fence_marker(trimmed: &str) -> Option<(char, usize)> {
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
fn fence_closes(trimmed: &str, open: (char, usize)) -> bool {
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
/// [`OVERSIZE_CHUNK_TARGET_TOKENS`] and splits at natural boundaries —
/// shrink `head` to the first chunk (keeping its value flags; the head
/// includes the section heading, so no outline is required to preserve
/// it) and follow it with predecessor-chained tail chunks priced like
/// the head: a tail is the direct continuation of content whose head
/// just won purchase.
fn push_whole_or_head_split(out: &mut Vec<SectionRange>, src_lines: &[&str], head: SectionRange) {
    let (start, end) = (head.start, head.end);
    let tokens: usize = (start..=end).map(|r| row_tokens(src_lines, r)).sum();
    if tokens < OVERSIZE_CHUNK_TARGET_TOKENS {
        out.push(head);
        return;
    }
    for (i, (chunk_start, chunk_end)) in oversize_chunk_bounds(src_lines, start, end)
        .into_iter()
        .enumerate()
    {
        out.push(SectionRange {
            start: chunk_start,
            end: chunk_end,
            chained_to_previous: i > 0,
            command_block: head.command_block.filter(|_| i == 0),
            ..head
        });
    }
}

/// One entry in the un-split top-level section list. `SyntheticIntro`
/// is the row range carved out by H1-unwrap to preserve the H1 heading
/// and the prelude before the first H2 (a virtual section inside the
/// H1's node). `Section` is any other top-level section (usually an
/// H2) and carries its tree-sitter node.
#[derive(Debug, Clone, Copy)]
enum TopLevelEntry<'a> {
    SyntheticIntro {
        /// Nothing but the H1 heading before the first H2.
        heading_only: bool,
        start: usize,
        end: usize,
    },
    Section {
        node: Node<'a>,
        start: usize,
        end: usize,
    },
}

/// Top-level section list with H1-unwrap. If the doc has exactly one
/// top-level section and it's an H1, descend into its H2 children and
/// synthesize an intro range for the H1 heading + pre-first-H2
/// prelude. Without this, a README styled `# Title` would collapse into
/// one multi-KB blob.
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
                out.push(TopLevelEntry::Section {
                    node: h2,
                    start: h2.start_position().row + 1,
                    end: node_end_row_trimmed(h2, source) + 1,
                });
            }
            return out;
        }
    }
    top.into_iter()
        .map(|s| TopLevelEntry::Section {
            node: s,
            start: s.start_position().row + 1,
            end: node_end_row_trimmed(s, source) + 1,
        })
        .collect()
}

/// README sections worth [`REFERENCE_USAGE_SECTION_FACTOR`]: a usage
/// demo (a usage title over a body that is nearly all code) or a
/// reference section (an options / configuration / API title over a
/// compact body that lists, tabulates or shows code). The same titles
/// over prose are introductions, and over a long body a tutorial the
/// index decay keeps back.
fn is_reference_usage_section(section: Node, source: &str) -> bool {
    let title = section_title_core(section, source);
    let body = section_body(section, None, source);
    (is_canonical_usage_title_core(&title) && is_code_dominant(section, body))
        || (is_reference_usage_title_core(&title) && reference_usage_body_ok(body))
}

fn section_title_core(section: Node, source: &str) -> String {
    first_heading_child(section)
        .and_then(|heading| first_child_of_kind(heading, "inline"))
        .map(|inline| title_core(&source[inline.start_byte()..inline.end_byte()]))
        .unwrap_or_default()
}

/// A README's back matter: who wrote, funds, maintains or may contribute
/// to the project, and under what license — never what the code does.
/// These sections emit no batch; the outline still names them. A title
/// may name its object (`Contributing to X`, `Support this project`).
fn is_appendix_title_core(core: &str) -> bool {
    let core = core.split_once(" to ").map_or(core, |(head, _)| head);
    let core = core.strip_suffix(" this project").unwrap_or(core);
    matches!(
        core,
        "license"
            | "licence"
            | "licensing"
            | "contributing"
            | "contribute"
            | "contributors"
            | "contribution"
            | "sponsors"
            | "backers"
            | "donate"
            | "donation"
            | "donations"
            | "support"
            | "funding"
            | "acknowledgements"
            | "acknowledgments"
            | "credits"
            | "thanks"
            | "authors"
            | "author"
            | "maintainers"
            | "star history"
            | "code of conduct"
            | "security"
            | "citation"
            | "contact"
    )
}

/// Title words naming how to build, test, run or develop the project.
const COMMAND_TITLE_WORDS: [&str; 12] = [
    "build",
    "building",
    "compile",
    "compiling",
    "compilation",
    "test",
    "tests",
    "testing",
    "develop",
    "development",
    "run",
    "running",
];

fn is_command_title_core(core: &str) -> bool {
    core.split_whitespace()
        .any(|word| COMMAND_TITLE_WORDS.contains(&word))
}

/// The first shell block (see [`is_shell_block`]) that sits under a
/// heading titled by [`is_command_title_core`] in `section`, extended
/// through the shell blocks after it before the next subsection, all
/// within [`OVERSIZE_CHUNK_TARGET_TOKENS`]; paired with the innermost
/// such heading (`command_heading` is the enclosing one). Back matter is
/// skipped.
fn command_block(
    section: Node,
    command_heading: Option<Node>,
    source: &str,
    src_lines: &[&str],
) -> Option<CommandBlockRows> {
    fn first_shell_block<'a>(node: Node<'a>, source: &str) -> Option<Node<'a>> {
        if matches!(node.kind(), "fenced_code_block" | "indented_code_block") {
            return is_shell_block(node, source).then_some(node);
        }
        let mut cursor = node.walk();
        let children: Vec<Node> = node.children(&mut cursor).collect();
        children
            .into_iter()
            .find_map(|child| first_shell_block(child, source))
    }
    let title = section_title_core(section, source);
    if is_appendix_title_core(&title) {
        return None;
    }
    let command_heading = first_heading_child(section)
        .filter(|_| is_command_title_core(&title))
        .or(command_heading);
    let rows = |node: Node| {
        (
            node.start_position().row + 1,
            node_end_row_trimmed(node, source) + 1,
        )
    };
    let mut cursor = section.walk();
    let children: Vec<Node> = section.children(&mut cursor).collect();
    let tokens = |start: usize, end: usize| -> usize {
        (start..=end).map(|row| row_tokens(src_lines, row)).sum()
    };
    for (idx, child) in children.iter().enumerate() {
        if child.kind() == "section" {
            if let Some(found) = command_block(*child, command_heading, source, src_lines) {
                return Some(found);
            }
            continue;
        }
        let (Some(heading), Some(block)) = (command_heading, first_shell_block(*child, source))
        else {
            continue;
        };
        let (start, mut end) = rows(block);
        if tokens(start, end) > OVERSIZE_CHUNK_TARGET_TOKENS {
            continue;
        }
        for sibling in children[idx + 1..]
            .iter()
            .take_while(|sibling| sibling.kind() != "section")
        {
            if let Some(next) = first_shell_block(*sibling, source) {
                let next_end = rows(next).1;
                if tokens(start, next_end) > OVERSIZE_CHUNK_TARGET_TOKENS {
                    break;
                }
                end = next_end;
            }
        }
        return Some(CommandBlockRows {
            heading: rows(heading),
            block: (start, end),
        });
    }
    None
}

/// Fence languages of shell commands.
const SHELL_LANGUAGES: [&str; 10] = [
    "sh",
    "bash",
    "shell",
    "console",
    "zsh",
    "fish",
    "powershell",
    "pwsh",
    "cmd",
    "bat",
];

/// A code block that holds shell commands: indented, untagged, or tagged
/// with a shell language.
fn is_shell_block(block: Node, source: &str) -> bool {
    let Some(info) = first_child_of_kind(block, "info_string") else {
        return true;
    };
    first_child_of_kind(info, "language").is_some_and(|language| {
        SHELL_LANGUAGES.contains(&source[language.byte_range()].to_ascii_lowercase().as_str())
    })
}

/// Usage-demo titles; shared with the RST heading path.
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

/// Reference titles; shared with the RST heading path. Bare "usage" is
/// a usage-demo title only: over prose it is a walkthrough.
fn is_reference_usage_title_core(core: &str) -> bool {
    matches!(
        core,
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
            | "environment variables"
    )
}

/// Upper bound (source bytes, heading excluded) on a reference
/// section's body.
const REFERENCE_USAGE_MAX_BODY_BYTES: usize = 1500;

/// Share of a usage demo's body (heading excluded) that must be code
/// blocks.
const CANONICAL_USAGE_CODE_MIN_FRACTION: f64 = 0.85;

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

/// Non-empty, at most [`REFERENCE_USAGE_MAX_BODY_BYTES`], and holding a
/// list item, table row or code fence.
fn reference_usage_body_ok(body: &str) -> bool {
    let is_structure_line = |line: &str| {
        let t = line.trim_start();
        t.starts_with(['-', '*', '+']) && t[1..].starts_with(' ')
            || t.starts_with('|')
            || t.starts_with("```")
            || t.starts_with("~~~")
            || t.split_once(". ")
                .is_some_and(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
    };
    !body.trim().is_empty()
        && body.len() <= REFERENCE_USAGE_MAX_BODY_BYTES
        && body.lines().any(is_structure_line)
}

fn is_code_dominant(section: Node, body: &str) -> bool {
    let mut cursor = section.walk();
    let code_bytes: usize = section
        .children(&mut cursor)
        .filter(|c| matches!(c.kind(), "fenced_code_block" | "indented_code_block"))
        .map(|c| c.end_byte() - c.start_byte())
        .sum();
    !body.is_empty() && code_bytes as f64 / body.len() as f64 >= CANONICAL_USAGE_CODE_MIN_FRACTION
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
/// links point at page sections (`href="#…"`), e.g. an
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
    spec: &mut BTreeSet<usize>,
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
fn prelude_remainder_rows(tree: &Tree, source: &str, headline: &BTreeSet<usize>) -> Vec<usize> {
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
        for row in block_text_rows(block, source) {
            if headline.contains(&row) {
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
/// wrappers, in-page nav menus and tables of contents. Everything else
/// above the first heading is substance. Both readers of that region use
/// this — `ReadmeHeadline` and [`prelude_remainder_rows`].
fn is_prelude_chrome_block(block: Node, source: &str) -> bool {
    is_decorative_block(block, source)
        || is_html_nav_block(block, source)
        || (block.kind() == "paragraph" && is_nav_link_paragraph(block, source))
        || is_table_of_contents(block, source)
}

/// A list whose every item is a bare in-document link (`- [Install](#install)`).
fn is_table_of_contents(block: Node, source: &str) -> bool {
    block.kind() == "list"
        && source[block.start_byte()..block.end_byte()]
            .lines()
            .filter(|line| !line.trim().is_empty())
            .all(|line| {
                line.trim()
                    .trim_start_matches(|c: char| c.is_ascii_digit())
                    .trim_start_matches(['-', '*', '+', '.', ')'])
                    .trim_start()
                    .strip_prefix('[')
                    .and_then(|link| link.split_once("](#"))
                    .is_some_and(|(text, target)| {
                        !text.contains(']')
                            && target.strip_suffix(')').is_some_and(|t| !t.contains(')'))
                    })
            })
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
        oversize_chunk_bounds(&src_lines, 1, end)
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
        let spec = headline_rows(&tree, source).expect("headline");
        assert!(spec.contains(&5), "lede row in headline");
        assert_eq!(prelude_remainder_rows(&tree, source, &spec), vec![9]);
    }

    #[test]
    fn markdown_table_of_contents_is_bare_section_links_only() {
        let toc = "- [Install](#install)\n  - [From source](#from-source)\n1. [Usage](#usage)\n";
        let features =
            "- **Fast** — see [benchmarks](#benchmarks)\n- **Tiny** — see [size](#size)\n";
        for (source, expected) in [(toc, true), (features, false)] {
            let tree = parse(source);
            let block = tree.root_node().child(0).and_then(|s| s.child(0)).unwrap();
            assert_eq!(is_table_of_contents(block, source), expected, "{source}");
        }
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

        let scheduler = Scheduler::new(WalkCtx::new(dir.path().to_path_buf()), FsWalker, 200, None);
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
        assert_eq!(covered(&source).into_iter().collect::<Vec<_>>(), vec![1]);
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
        headline_rows(&parse(source), source).expect("headline rows")
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
        // A centered HTML title block keeps only its text rows; an in-page
        // table of contents under the title is chrome.
        (
            "markdown_html_title_block_keeps_text_rows_and_skips_toc",
            "<div align=\"center\">\n\
                   <img src=\"logo.png\">\n\
                   <h1>Tool</h1>\n\
                   </div>\n\
                   \n\
                   # Tool\n\
                   \n\
                   * [Install](#install)\n\
                   * [Usage](#usage)\n\
                   \n\
                   Tool does the useful thing for you.\n",
            &[3, 6, 11],
            &[2, 8, 9],
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

    fn outline_rows_of(source: &str) -> Vec<(usize, usize)> {
        let tree = parse(source);
        let headline = headline_rows(&tree, source);
        outline_rows(&tree, source, headline.as_ref())
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
        let rows = outline_rows_of(src);
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
        let rows = outline_rows_of(src);
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
        let rows = outline_rows_of(src);
        assert!(
            rows.is_empty(),
            "outline must not include H4+; got {rows:?}"
        );
    }

    #[test]
    fn markdown_outline_over_the_cap_keeps_h1_h2() {
        let mut src = String::from("# Tool\n\nTool does things.\n");
        for section in 0..3 {
            src.push_str(&format!("\n## Part {section}\n"));
            for sub in 0..MAX_OUTLINE_HEADINGS / 2 {
                src.push_str(&format!("\n### Step {sub}\n"));
            }
        }
        let rows = outline_rows_of(&src);
        let lines: Vec<&str> = src.lines().collect();
        assert_eq!(rows.len(), 3);
        assert!(
            rows.iter()
                .all(|&(start, _)| lines[start - 1].starts_with("## "))
        );
    }

    // --- logical_sections tests ---

    fn sections(source: &str) -> Vec<SectionRange> {
        let tree = parse(source);
        let outline_emits = !outline_rows_of(source).is_empty();
        logical_sections(&tree, source, outline_emits)
    }

    /// Back-matter sections (license, contributing, sponsors, …) emit no
    /// batch, whatever emoji or markup leads their title.
    #[test]
    fn markdown_readme_back_matter_emits_no_section() {
        let src = "# Tool\n\nDoes things.\n\n## Usage\n\nRun it.\n\n\
                   ## 📄 License\n\nMIT.\n\n## Contributing\n\nSend PRs.\n";
        let ranges = sections(src);
        let bounds: Vec<_> = ranges.iter().map(|r| (r.start, r.end)).collect();
        assert_eq!(bounds, [(1, 4), (5, 7)]);

        let src = "# Tool\n\nDoes things.\n\n## About\n\nIt is small.\n\n\
                   ### Author\n\nMe.\n\n### License to use\n\nMIT.\n";
        let bounds: Vec<_> = sections(src).iter().map(|r| (r.start, r.end)).collect();
        assert_eq!(bounds, [(1, 4), (5, 8)]);

        let src =
            "# Tool\n\nDoes things.\n\n## About\n\n### Author\n\nMe.\n\n### License\n\nMIT.\n";
        let bounds: Vec<_> = sections(src).iter().map(|r| (r.start, r.end)).collect();
        assert_eq!(bounds, [(1, 4)]);
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
            // → NOT flagged.
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
            let ranges = sections(&src);
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
        let ranges = sections(src);
        // Three ranges: synthetic intro, ## Install, ## Use.
        let indices: Vec<usize> = ranges.iter().map(|r| r.h2_index).collect();
        assert_eq!(indices, vec![0, 0, 1], "got {ranges:?}");
        assert_eq!(readme_index_decay(&ranges[1]), 1.0);
        let f = readme_index_decay(&ranges[2]);
        assert!(f < 1.0, "second real H2 should decay; got {f}");
    }

    /// READMEs without an H1 wrap (no synthetic intro) — first H2 is
    /// index 0 and gets factor 1.0 directly.
    #[test]
    fn markdown_readme_index_decay_no_synthetic_intro() {
        let src = "## Install\n\nbody\n\n## Use\n\nbody\n";
        let ranges = sections(src);
        let indices: Vec<usize> = ranges.iter().map(|r| r.h2_index).collect();
        assert_eq!(indices, vec![0, 1]);
        assert_eq!(readme_index_decay(&ranges[0]), 1.0);
    }

    /// Only the root README is read; every other document, a nested
    /// README included, is left to the listing.
    #[test]
    fn markdown_walker_reads_only_the_root_readme() {
        use std::fs;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let body = "# Project\n\n\
                    Top-level instructions go here.\n\n\
                    ## Setup\n\nrun `cargo build`.\n\n\
                    ## Conventions\n\nuse rustfmt.\n";
        fs::create_dir(root.join("docs")).unwrap();
        for name in ["README.md", "notes.md", "docs/README.md"] {
            fs::write(root.join(name), body).unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        let root_batches = expand_in_dir(root, &ctx);
        assert!(root_batches.iter().any(|b| matches!(
            &b.key,
            BatchKey::Markdown(MarkdownKey::Section { file, .. }) if *file == root.join("README.md")
        )));
        assert!(root_batches.iter().all(|b| matches!(
            &b.key,
            BatchKey::Markdown(
                MarkdownKey::ReadmeHeadline { file }
                    | MarkdownKey::Prelude { file }
                    | MarkdownKey::HeadingsOutline { file }
                    | MarkdownKey::Section { file, .. }
            ) if *file == root.join("README.md")
        )));
        assert!(expand_in_dir(&root.join("docs"), &ctx).is_empty());
    }

    /// A build/test/run section yields one `CommandBlock` per top-level
    /// section — its shell, untagged or indented blocks, never a code sample —
    /// and the section holding it gates on it.
    #[test]
    fn markdown_command_block_per_section() {
        use std::fs;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let body = "# Tool\n\nDoes things.\n\n\
                    ## Usage\n\n```sh\ntool run\n```\n\n\
                    ## Building\n\nNeeds a compiler.\n\n\
                    ```rust\nfn main() {}\n```\n\n\
                    ```sh\ncargo build\n```\n\n\
                    ### Running tests\n\n```\ncargo test\n```\n\n\
                    ## Development\n\n```console\n$ make dev\n```\n";
        fs::write(root.join("README.md"), body).unwrap();
        let batches = expand_in_dir(root, &WalkCtx::new(root.to_path_buf()));
        let command_rows: Vec<usize> = batches
            .iter()
            .filter_map(|b| match &b.key {
                BatchKey::Markdown(MarkdownKey::CommandBlock { row, .. }) => Some(*row),
                _ => None,
            })
            .collect();
        assert_eq!(command_rows, vec![19, 31]);
        let building = batches
            .iter()
            .find(|b| {
                matches!(
                    &b.key,
                    BatchKey::Markdown(MarkdownKey::Section {
                        section_index: 2,
                        ..
                    })
                )
            })
            .unwrap();
        assert!(matches!(
            &building.predecessor,
            Some(BatchKey::Markdown(MarkdownKey::CommandBlock {
                row: 19,
                ..
            }))
        ));
    }

    /// A root README in any recognized markup is read — Markdown first,
    /// and an extensionless one by its content.
    #[test]
    fn markdown_root_readme_recognizes_markups() {
        use std::fs;
        type Case<'a> = (&'a [(&'a str, &'a str)], Option<(&'a str, ReadmeMarkup)>);
        let cases: [Case; 5] = [
            (
                &[("README.adoc", "= T\n"), ("README.rst", "T\n=\n")],
                Some(("README.rst", ReadmeMarkup::Rst)),
            ),
            (
                &[("README.asciidoc", "= T\n")],
                Some(("README.asciidoc", ReadmeMarkup::AsciiDoc)),
            ),
            (
                &[("README", "# T\n"), ("README.md", "# T\n")],
                Some(("README.md", ReadmeMarkup::Markdown)),
            ),
            (
                &[("README", "Title\n=====\n")],
                Some(("README", ReadmeMarkup::Rst)),
            ),
            (
                &[("README.html", "<h1>T</h1>\n"), ("README-dev.md", "# T\n")],
                None,
            ),
        ];
        for (files, expected) in cases {
            let dir = tempfile::tempdir().unwrap();
            for (name, body) in files {
                fs::write(dir.path().join(name), body).unwrap();
            }
            let ctx = WalkCtx::new(dir.path().to_path_buf());
            let found = root_readme(dir.path(), &ctx);
            assert_eq!(
                found,
                expected.map(|(name, markup)| (dir.path().join(name), markup))
            );
        }
    }

    #[test]
    fn markdown_title_core_reads_past_leading_symbols() {
        assert_eq!(title_core("🚀 Quick Start"), "quick start");
        assert_eq!(title_core("**Usage**"), "usage");
        assert_eq!(title_core("Usage: CLI"), "usage");
    }

    /// A non-ASCII tagline whose UTF-8 byte length exceeds the tagline
    /// max but whose char count is under it must measure as short — length
    /// is counted in chars, not bytes.
    #[test]
    fn markdown_strip_block_for_length_counts_chars_not_bytes() {
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
