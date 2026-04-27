//! Markdown walker. Uses `tree-sitter-md`'s block grammar for heading and
//! section detection.
//!
//! Keys:
//! - `SummaryWhole { file }` — `SUMMARY.md`, whole file (mdBook ToC)
//! - `ReadmeHeadline { file }` — `README.md`, the heading's project-name
//!   line plus the first non-decorative content. Decorative paragraphs
//!   (image-only / badge-only) and `<img>`-only HTML blocks immediately
//!   after the heading are skipped, and a heading line whose tail is
//!   nothing but badges is rendered with `Render::Truncated` to drop it.
//! - `HeadingsOutline { file }` — every H1/H2/H3 heading line (full
//!   row, no body). Cheap navigation hedge analogous to Rust's
//!   `PubItemNames`. For READMEs the headline-covered H1 row is
//!   skipped so the headline's `Truncated` render survives. Only
//!   emitted when 2..=`MAX_OUTLINE_HEADINGS` collectable rows exist.
//! - `Section { file, section_index }` — one scheduling unit of a
//!   markdown file's body, 0-indexed. Default granularity is one
//!   H2-level top-level section per batch; `logical_sections`
//!   subdivides via one of two rules when applicable. *Bullet split*
//!   (anyhow's `## Details` shape) — an H2 whose non-decorative
//!   content is a single `list` block becomes one `BulletItem` per
//!   substantive top-level item. *H3 split* (content-heavy H2 with
//!   ≥2 H3 children) becomes one `Intro` (when its body is
//!   substantive) plus one `H3Child` per H3. Both kinds carry a
//!   global signal scale (`SUB_SECTION_SIGNAL_SCALE`) on the
//!   per-child ranges to keep them from over-ranking once the
//!   marginal cost drops to per-sub-section size. Changelog-class
//!   files (CHANGELOG / CONTRIBUTING / CHANGES) and files whose
//!   outline isn't emitted are gated out of splitting (the H2
//!   heading is preserved by `HeadingsOutline` only when it emits).
//!   Predecessor (when emitted): outline → headline → none, picking
//!   the deepest available so all heading-row overlaps are
//!   ancestor-overlaps. Enabling either split rule shifts
//!   `section_index` numbering relative to a non-split version of
//!   the same file; snapshots / divergence reports reflect the
//!   post-split indexing.

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, MarkdownKey};
use crate::content::{BatchContent, Render, Span};
use crate::tokenizer;
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, fs::files_with_extension, path_depth_factor, single_file_lines_content,
};

/// Upper bound on collectable heading rows before `HeadingsOutline`
/// suppresses itself. The outline is the predecessor of every section,
/// so an oversize outline that fails to fit near the budget tail would
/// block the whole file. See `docs/design-notes.md` "Sub-section
/// markdown splitting" for the deferred decoupling.
const MAX_OUTLINE_HEADINGS: usize = 30;

/// Upper bound (in source bytes) on the outline's heading content. The
/// row-count cap alone wouldn't catch a file with 5 very long headings
/// — render bytes drive token cost, so we also cap by source bytes.
/// 1500 chars ≈ 400 tokens, comfortably small at any reasonable budget.
const MAX_OUTLINE_HEADING_BYTES: usize = 1500;

/// Minimum H2 source-byte length required to subdivide it into per-H3
/// or per-bullet `SectionRange`s. Below this, the H2 fits in one
/// batch and splitting just adds scheduling overhead with no
/// waste-reduction payoff.
const H2_SPLIT_BYTES: usize = 600;

/// Multiplier on the three value signals for `SectionKind::H3Child`
/// and `SectionKind::BulletItem` ranges. Compensates for the smaller
/// marginal cost — identical signals would over-rank a sub-section
/// relative to other walker batches at the parent H2's calibration
/// level.
const SUB_SECTION_SIGNAL_SCALE: f64 = 0.45;

/// Bullet-list-split predicate parameters.
///
/// `BULLET_MIN_ITEMS` — minimum count of top-level list items the
/// section's single list block must contain. Below this the gain from
/// splitting is negligible.
///
/// `BULLET_MIN_LARGE_ITEMS` and `BULLET_LARGE_ITEM_BYTES` — at least
/// `BULLET_MIN_LARGE_ITEMS` of those items must individually exceed
/// `BULLET_LARGE_ITEM_BYTES` source bytes. Calibrated to anyhow's
/// `## Details` (6 items, each ~150-300 source bytes); below the
/// threshold the items are short one-liners (e.g. ts-pattern's
/// `## Features`) where splitting adds scheduling overhead without
/// reducing waste. Pair gates the predicate together.
const BULLET_MIN_ITEMS: usize = 3;
const BULLET_MIN_LARGE_ITEMS: usize = 2;
const BULLET_LARGE_ITEM_BYTES: usize = 200;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
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
            if let Some(content) = build_summary_content(&file, ctx) {
                out.push(Batch {
                    key: MarkdownKey::SummaryWhole { file: file.clone() }.into(),
                    predecessor: None,
                    content,
                    value: summary_value(&file, ctx),
                });
            }
            continue;
        }

        let Some((source, tree)) = parse_md(ctx, &file) else {
            continue;
        };
        let ranges = logical_sections(&file, &tree, &source);
        if ranges.is_empty() {
            continue;
        }

        let is_readme = is_readme(&file);
        let headline_key = is_readme.then(|| MarkdownKey::ReadmeHeadline { file: file.clone() });
        let outline_rows = collectable_outline_rows(&file, &tree, &source);
        let outline_emits = outline_emits_for(&outline_rows, &source);
        let outline_key =
            outline_emits.then(|| MarkdownKey::HeadingsOutline { file: file.clone() });

        let mut headline_emitted: Option<BatchKey> = None;
        if let Some(h) = &headline_key
            && let Some(content) = build_headline_content(&file, &source, &tree)
        {
            out.push(Batch {
                key: h.clone().into(),
                predecessor: None,
                content,
                value: readme_headline_value(&file, ctx),
            });
            headline_emitted = Some(BatchKey::Markdown(h.clone()));
        }
        let mut outline_emitted: Option<BatchKey> = None;
        if let Some(o) = &outline_key
            && let Some(content) = build_outline_content(&file, &source, &outline_rows)
        {
            out.push(Batch {
                key: o.clone().into(),
                predecessor: headline_emitted.clone(),
                content,
                value: headings_outline_value(&file, ctx),
            });
            outline_emitted = Some(BatchKey::Markdown(o.clone()));
        }

        let section_predecessor = outline_emitted.or(headline_emitted);

        for (idx, range) in ranges.iter().enumerate() {
            if let Some(content) = build_section_content(&file, &source, &tree, idx, range) {
                out.push(Batch {
                    key: MarkdownKey::Section {
                        file: file.clone(),
                        section_index: idx,
                    }
                    .into(),
                    predecessor: section_predecessor.clone(),
                    content,
                    value: section_value(&file, range, ctx),
                });
            }
        }
    }
    out
}

/// True if the outline batch should be emitted for a file with these
/// heading rows. Bounded both by row count and total source bytes —
/// long heading lines can blow past `MAX_OUTLINE_HEADINGS * tokens`
/// even when the row count looks safe.
fn outline_emits_for(rows: &[(usize, usize)], source: &str) -> bool {
    if rows.len() < 2 || rows.len() > MAX_OUTLINE_HEADINGS {
        return false;
    }
    let src_lines: Vec<&str> = source.lines().collect();
    let bytes: usize = rows
        .iter()
        .flat_map(|(s, e)| {
            (*s..=*e)
                .filter_map(|r| src_lines.get(r - 1))
                .map(|l| l.len())
        })
        .sum();
    bytes <= MAX_OUTLINE_HEADING_BYTES
}

// --- value ---

fn summary_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.9, 0.8, 0.7, path_depth_factor(file, ctx))
}

fn readme_headline_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.9, 0.6, 0.8, path_depth_factor(file, ctx))
}

fn headings_outline_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.7, 0.55, 0.4, path_depth_factor(file, ctx))
}

fn readme_section_value(file: &Path, range: &SectionRange, ctx: &WalkCtx) -> f64 {
    // README sections share a flat base (catastrophic 0.55,
    // follow-up 0.8, ztu 0.7) — a single section is a piece of
    // README body that just fits more often when split. The mild
    // index decay tilts toward earlier sections (install / quick-
    // start / "how it works") without pushing late ones (License,
    // Contributing, FAQ appendix) out of the schedule. Floored at
    // 0.7 — section 5 keeps ≈76% weight, section 10+ keeps 70%.
    // Index counts real H2 sections only (the H1-unwrap synthetic
    // intro at parent_index=0 doesn't count), so a `# Title` README's
    // first real H2 gets factor 1.0.
    mix_signals(0.55, 0.8, 0.7, path_depth_factor(file, ctx)) * readme_index_decay(range)
}

/// Mild index decay for README sections. Counts real H2 sections only
/// (skipping the synthetic H1-unwrap intro). Returns 1.0 for the first
/// real H2; floors at 0.7.
fn readme_index_decay(range: &SectionRange) -> f64 {
    let h2_idx = if range.synthetic_intro_present {
        range.parent_index.saturating_sub(1)
    } else {
        range.parent_index
    };
    index_decay(h2_idx, 0.15, 0.7)
}

/// Index-based signal-channel scale factor: `(idx + 1)^-exp`, floored
/// at `floor`. Shared by `readme_index_decay` and the changelog decay
/// in `heading_slab_signals`. Both decay the same shape; differ only
/// in `exp` and `floor`.
fn index_decay(idx: usize, exp: f64, floor: f64) -> f64 {
    ((idx as f64 + 1.0).powf(-exp)).max(floor)
}

fn heading_slab_value(file: &Path, parent_index: usize, ctx: &WalkCtx) -> f64 {
    let is_guide = is_changelog_class(file);
    // Changelogs are conventionally sorted newest-first, so later
    // sections are ancient release notes of decreasing relevance. Apply
    // an index-based decay only to guide-shape files; for general docs
    // the section order doesn't imply relevance. Floored at 0.35 so a
    // deep section can still fire if budget permits, just not displace
    // higher-tier content. The decay uses `parent_index` (un-split
    // top-level position) so a future H3-split of a changelog still
    // inherits its parent H2's decay tier.
    let scale = if is_guide {
        index_decay(parent_index, 0.3, 0.35)
    } else {
        1.0
    };
    let cat = if is_guide { 0.5 } else { 0.3 };
    mix_signals(cat, 0.5, 0.5, path_depth_factor(file, ctx)) * scale
}

/// Per-section value. `H3Child` and `BulletItem` ranges scale the parent's
/// value — identical weights would over-rank them on `value / cost^0.35`
/// once the cost drops to per-sub-section size. `Intro` keeps full weight
/// (it carries the H2 heading + topic prelude).
fn section_value(file: &Path, range: &SectionRange, ctx: &WalkCtx) -> f64 {
    let parent = if is_readme(file) {
        readme_section_value(file, range, ctx)
    } else {
        heading_slab_value(file, range.parent_index, ctx)
    };
    match range.kind {
        SectionKind::Whole | SectionKind::Intro => parent,
        SectionKind::H3Child | SectionKind::BulletItem => parent * SUB_SECTION_SIGNAL_SCALE,
    }
}

fn is_changelog_class(file: &Path) -> bool {
    file.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
        matches!(
            n.to_ascii_uppercase().as_str(),
            "CHANGELOG.MD" | "CONTRIBUTING.MD" | "CHANGES.MD"
        )
    })
}

// --- parser ---

fn parse_md(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_md::LANGUAGE.into())
}

/// Parse `text` with the inline grammar. tree-sitter-md's block grammar
/// produces a flat `inline` node holding raw bytes; the inline grammar
/// is what turns those bytes into named `image` / `inline_link` /
/// `html_tag` children.
fn parse_inline(text: &str) -> Option<Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_md::INLINE_LANGUAGE.into())
        .ok()?;
    parser.parse(text, None)
}

// --- content builders ---

fn build_summary_content(file: &Path, ctx: &WalkCtx) -> Option<BatchContent> {
    let source = ctx.read_source(file)?;
    let line_count = source.lines().count();
    let lines: Vec<usize> = (1..=line_count).collect();
    single_file_lines_content(file, &source, FileLines::new(lines))
}

fn build_headline_content(file: &Path, source: &str, tree: &Tree) -> Option<BatchContent> {
    let spec = headline_spec(tree, source)?;
    let spans = build_headline_spans(file, source, &spec);
    if spans.is_empty() {
        return None;
    }
    Some(BatchContent::Lines { spans })
}

fn build_outline_content(
    file: &Path,
    source: &str,
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
        ellipses.push(end + 1);
    }
    single_file_lines_content(file, source, FileLines::new(full).with_ellipses(ellipses))
}

/// Heading-row ranges (1-based, inclusive) the `HeadingsOutline` batch
/// would render. Walks every `atx_heading` / `setext_heading` reachable
/// in the parse tree, keeps only level 1-3, and (for READMEs) drops any
/// heading already covered by `ReadmeHeadline` so the outline never
/// overrides the headline's `Render::Truncated` with `Render::Full`.
fn collectable_outline_rows(file: &Path, tree: &Tree, source: &str) -> Vec<(usize, usize)> {
    let headline_covered: BTreeSet<usize> = if is_readme(file) {
        headline_spec(tree, source)
            .map(|s| s.covered_rows.iter().copied().collect())
            .unwrap_or_default()
    } else {
        BTreeSet::new()
    };
    let mut nodes = Vec::new();
    collect_heading_nodes(tree.root_node(), &mut nodes);
    let mut out = Vec::new();
    for node in nodes {
        let level = heading_level(node);
        if !(1..=3).contains(&level) {
            continue;
        }
        let start_row = node.start_position().row + 1;
        let end_row = span_last_row(node, source) + 1;
        if (start_row..=end_row).any(|r| headline_covered.contains(&r)) {
            continue;
        }
        out.push((start_row, end_row));
    }
    out
}

fn collect_heading_nodes<'a>(node: Node<'a>, out: &mut Vec<Node<'a>>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if matches!(child.kind(), "atx_heading" | "setext_heading") {
            out.push(child);
        } else {
            collect_heading_nodes(child, out);
        }
    }
}

fn build_section_content(
    file: &Path,
    source: &str,
    tree: &Tree,
    section_index: usize,
    range: &SectionRange,
) -> Option<BatchContent> {
    let (start, end) = (range.start, range.end);

    // For README section 0, exclude lines already covered by
    // `ReadmeHeadline`. Otherwise the two batches overlap on short
    // READMEs, Section 0's marginal cost drops to zero after dedupe,
    // and `ratio(value, 0) = INFINITY` gives it unconditional
    // scheduling priority — a smell even though the duplicate apply
    // is a no-op.
    let effective_start = if section_index == 0
        && is_readme(file)
        && let Some(spec) = headline_spec(tree, source)
        && let Some(max_row) = spec.last_covered_row()
    {
        (max_row + 1).max(start)
    } else {
        start
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
}

// --- headline spec + span construction ---

/// Computed shape of `ReadmeHeadline`: the set of source rows the batch
/// renders, plus an optional per-row truncation override for the
/// heading line. Rows are 1-based.
#[derive(Debug, Clone)]
struct HeadlineSpec {
    covered_rows: BTreeSet<usize>,
    truncate: Option<TruncatedRow>,
}

/// Per-row truncation override: render row `row` as
/// `Render::Truncated { pattern }` instead of `Render::Full`. Storing
/// the pair together makes "row set without pattern" unrepresentable.
#[derive(Debug, Clone)]
struct TruncatedRow {
    row: usize,
    pattern: String,
}

impl HeadlineSpec {
    fn last_covered_row(&self) -> Option<usize> {
        self.covered_rows.iter().next_back().copied()
    }
}

/// Block kinds that bound a section's content. Hitting one means we've
/// reached a sibling sub-section — `ReadmeHeadline` must never reach
/// into nested H2/H3 bodies.
fn is_section_boundary(kind: &str) -> bool {
    matches!(kind, "section" | "atx_heading" | "setext_heading")
}

fn headline_spec(tree: &Tree, source: &str) -> Option<HeadlineSpec> {
    let section = headed_sections(tree.root_node()).next()?;
    let heading = first_heading_child(section)?;

    let mut covered: BTreeSet<usize> = BTreeSet::new();
    extend_rows_inclusive(&mut covered, heading, source);
    let heading_first_row = heading.start_position().row + 1;

    // Walk siblings after the heading, skipping leading decorative
    // paragraphs / image-only HTML blocks; then include subsequent
    // blocks until we hit a non-decorative paragraph or a section
    // boundary.
    let post: Vec<Node> = children_after(section, heading);
    let mut i = 0;
    while i < post.len() {
        let block = post[i];
        if is_section_boundary(block.kind()) {
            break;
        }
        match block.kind() {
            "paragraph" if is_decorative_paragraph(block, source) => {}
            "html_block" if is_decorative_html_block(block, source) => {}
            _ => break,
        }
        i += 1;
    }
    while i < post.len() {
        let block = post[i];
        if is_section_boundary(block.kind()) {
            break;
        }
        extend_rows_inclusive(&mut covered, block, source);
        if block.kind() == "paragraph" && !is_decorative_paragraph(block, source) {
            break;
        }
        i += 1;
    }

    let truncate = compute_heading_truncation(heading, source);

    if covered.is_empty() {
        return None;
    }
    debug_assert!(
        covered.contains(&heading_first_row),
        "headline covered_rows missing heading row"
    );

    Some(HeadlineSpec {
        covered_rows: covered,
        truncate,
    })
}

fn build_headline_spans(file: &Path, source: &str, spec: &HeadlineSpec) -> Vec<Span> {
    let trunc_row = spec.truncate.as_ref().map(|t| t.row);
    let full_rows: Vec<usize> = spec
        .covered_rows
        .iter()
        .copied()
        .filter(|n| Some(*n) != trunc_row)
        .collect();

    // build_file_spans handles blank-row filtering + contiguous-range
    // merging for the Full rows; we only need to splice in the
    // truncated-row span (if any) to assemble the final list.
    let mut spans = super::build_file_spans(file, source, FileLines::new(full_rows));
    if let Some(t) = &spec.truncate {
        spans.push(Span {
            path: file.to_path_buf(),
            start: t.row,
            end: t.row,
            render: Render::Truncated {
                pattern: t.pattern.clone(),
            },
        });
        spans.sort_by_key(|s| s.start);
    }
    spans
}

/// Append every 1-based row covered by `node` to `out`, trimming a
/// trailing newline tree-sitter-md sometimes includes in a node's span.
fn extend_rows_inclusive(out: &mut BTreeSet<usize>, node: Node, source: &str) {
    let last = span_last_row(node, source);
    for row in node.start_position().row..=last {
        out.insert(row + 1);
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

/// Inline children with no semantic content — whitespace text and
/// line-breaks. Skipping these from inline iteration leaves only the
/// "real" inline children.
fn is_skippable_inline(node: Node, source: &str) -> bool {
    match node.kind() {
        "text" => source[node.start_byte()..node.end_byte()].trim().is_empty(),
        "hard_line_break" | "soft_line_break" => true,
        _ => false,
    }
}

/// Image / badge / `<img>`-tag inline. We deliberately do NOT classify
/// plain text-link / autolink / email-autolink as decorative: a
/// paragraph of just `[Live Examples](...)` is real navigation content
/// for the reader.
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

/// True iff every direct child of `link_text` is either skippable or
/// an image-shaped node. Direct children only — DO NOT recurse into
/// `image`'s `image_description` (the alt text would otherwise count
/// as prose and defeat badge detection).
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

/// A paragraph is decorative iff every non-skippable inline child is
/// decorative AND there's at least one such child. The block grammar's
/// `inline` node is opaque bytes — we re-parse with the inline grammar
/// to see named children like `image` / `inline_link` / `html_tag`.
/// Returns `false` (treat as non-decorative) when the inline grammar
/// doesn't surface any named children — e.g., a plain-text paragraph
/// produces only an `inline` root with raw `text` content.
fn is_decorative_paragraph(para: Node, source: &str) -> bool {
    let Some(inline_block) = first_child_of_kind(para, "inline") else {
        return false;
    };
    let inline_text = &source[inline_block.start_byte()..inline_block.end_byte()];
    let Some(tree) = parse_inline(inline_text) else {
        return false;
    };
    let root = tree.root_node();
    inline_root_is_all_decorative(root, inline_text)
}

/// True iff the inline content is non-empty AND every fragment
/// (named children + plain-text gaps between them — the inline grammar
/// leaves plain text outside named nodes) is decorative or whitespace.
fn inline_root_is_all_decorative(root: Node, inline_text: &str) -> bool {
    let named = named_decorative_candidates(root, inline_text);
    if named.is_empty() {
        return false;
    }
    named.iter().all(|n| is_decorative_inline(*n, inline_text))
        && plain_text_gaps_are_blank(&named, inline_text, 0)
}

fn named_decorative_candidates<'a>(root: Node<'a>, inline_text: &str) -> Vec<Node<'a>> {
    let mut cur = root.walk();
    root.children(&mut cur)
        .filter(|c| c.is_named() && !is_skippable_inline(*c, inline_text))
        .collect()
}

/// True iff the byte range `[gap_start, named[0].start)` plus the gaps
/// between consecutive named children plus the tail after the last
/// named child are all whitespace-only. Used to enforce "no real prose
/// between badges" in both decorative-paragraph and heading-truncation
/// classifications.
fn plain_text_gaps_are_blank(named: &[Node], inline_text: &str, gap_start: usize) -> bool {
    let mut cursor = gap_start;
    for n in named {
        if !inline_text[cursor..n.start_byte()].trim().is_empty() {
            return false;
        }
        cursor = n.end_byte();
    }
    inline_text[cursor..].trim().is_empty()
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

fn strip_html_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            while i < bytes.len() && bytes[i] != b'>' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

// --- heading truncation ---

/// Returns the truncation override if the heading should render its
/// row as `Render::Truncated { pattern }` to drop a trailing badge run.
/// Pre-conditions:
/// * `inline` child exists (atx_heading or setext_heading)
/// * the inline opens with non-empty plain-text content (the project
///   name) before its first named child
/// * every named inline child is decorative AND no non-whitespace
///   plain text sits between or after them
/// * the truncated render saves at least one token vs `Render::Full`.
///
/// Returns `None` whenever any condition fails — caller renders the
/// heading row verbatim.
fn compute_heading_truncation(heading: Node, source: &str) -> Option<TruncatedRow> {
    let inline_block = first_child_of_kind(heading, "inline")?;
    let inline_text = &source[inline_block.start_byte()..inline_block.end_byte()];
    let tree = parse_inline(inline_text)?;
    let named = named_decorative_candidates(tree.root_node(), inline_text);
    if named.is_empty() {
        return None;
    }

    // Plain-text gap before the first named child = the project name.
    // Empty means the heading opens with a link/image (e.g.
    // `# [Project](url)`) — don't truncate, would erase the name.
    if inline_text[..named[0].start_byte()].trim().is_empty() {
        return None;
    }
    if !named.iter().all(|n| is_decorative_inline(*n, inline_text)) {
        return None;
    }
    if !plain_text_gaps_are_blank(&named[1..], inline_text, named[0].end_byte()) {
        return None;
    }

    // The pattern runs from the heading marker (`# `, `## `, …) through
    // the project-name text. Including the heading marker matters
    // because `Render::Truncated` only renders the matched bytes —
    // without `#` the rendered line would lose its heading marker.
    let abs_trim = inline_block.start_byte() + named[0].start_byte();
    let prefix = source[heading.start_byte()..abs_trim].trim_end();
    if prefix.contains('\n') || prefix.is_empty() {
        return None;
    }

    let row = inline_block.start_position().row + 1;
    let line = source.lines().nth(row.saturating_sub(1)).unwrap_or("");
    let full_tokens = tokenizer::count(&format!("{line}\n"));
    let truncated_tokens = tokenizer::count(&format!("{prefix}…\n"));
    if truncated_tokens >= full_tokens {
        return None;
    }

    Some(TruncatedRow {
        row,
        pattern: regex::escape(prefix),
    })
}

// --- tree-sitter-md helpers ---

/// One scheduling unit for a markdown file: 1-based inclusive row range
/// plus its kind and the index of its parent H2 in the *un-split*
/// top-level section list. `parent_index` lets `heading_slab_signals`
/// apply changelog index-decay relative to the parent H2 instead of
/// the post-split logical-section index. `synthetic_intro_present`
/// is true iff the file's `top_level_entries[0]` is a
/// `SyntheticIntro` (the H1-unwrap virtual section); README/changelog
/// decay subtracts 1 from `parent_index` in that case so the first
/// real H2 is treated as index 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SectionRange {
    start: usize,
    end: usize,
    kind: SectionKind,
    parent_index: usize,
    synthetic_intro_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SectionKind {
    /// Whole top-level section (un-split H2 or H1-unwrap synthetic intro).
    Whole,
    /// H2 heading + prelude before its first H3 child or bullet list.
    Intro,
    /// One H3 sub-section under a split H2.
    H3Child,
    /// One top-level bullet item under a split H2 (see
    /// `should_split_by_bullets`). Preserves the H2 heading via
    /// `HeadingsOutline`, same gating discipline as H3 splitting.
    BulletItem,
}

/// Section ranges for batching. The "un-split top-level list" — one
/// entry per top-level section (including the synthetic H1-unwrap
/// intro) — is the basis for `parent_index`. H2s that satisfy a split
/// rule expand to one optional `Intro` range (only when its body
/// before the first sub-section has substantive content) plus one
/// sub-range per child:
///
/// - **Bullet split** fires when the H2's content (modulo decorative
///   blocks) is exactly one `list` block satisfying
///   [`should_split_by_bullets`] — emits one `BulletItem` per
///   substantive top-level list item.
/// - **H3 split** fires when the H2 has ≥2 direct H3 children and
///   meets the byte threshold — emits one `H3Child` per substantive
///   H3 child.
///
/// All other top-level entries emit one `Whole` range. Skipping the
/// split when the file's outline isn't emitted preserves the H2
/// heading via the `Whole` range — Intro ranges with empty bodies
/// are elided to avoid `ratio(value, 0) = INFINITY` no-op batches,
/// so without an outline the H2 heading would otherwise be lost.
fn logical_sections(file: &Path, tree: &Tree, source: &str) -> Vec<SectionRange> {
    let entries = top_level_entries(tree.root_node(), source);
    let outline_will_emit = {
        let rows = collectable_outline_rows(file, tree, source);
        outline_emits_for(&rows, source)
    };
    let split_eligible_file = !is_changelog_class(file);
    let synthetic_intro_present =
        matches!(entries.first(), Some(TopLevelEntry::SyntheticIntro { .. }));

    let mut out = Vec::with_capacity(entries.len());
    for (parent_idx, entry) in entries.iter().enumerate() {
        match entry {
            TopLevelEntry::SyntheticIntro { start, end } => {
                out.push(SectionRange {
                    start: *start,
                    end: *end,
                    kind: SectionKind::Whole,
                    parent_index: parent_idx,
                    synthetic_intro_present,
                });
            }
            TopLevelEntry::H2Section { node, start, end } => {
                let bytes = node.end_byte() - node.start_byte();
                let split_gate =
                    split_eligible_file && outline_will_emit && bytes >= H2_SPLIT_BYTES;

                let bullet_items = split_gate
                    .then(|| should_split_by_bullets(*node, source))
                    .flatten();

                if let Some(items) = bullet_items {
                    push_intro(
                        &mut out,
                        *node,
                        *start,
                        items[0],
                        parent_idx,
                        synthetic_intro_present,
                        source,
                    );
                    for item in &items {
                        let item_start = item.start_position().row + 1;
                        let item_end = span_last_row(*item, source) + 1;
                        out.push(SectionRange {
                            start: item_start,
                            end: item_end,
                            kind: SectionKind::BulletItem,
                            parent_index: parent_idx,
                            synthetic_intro_present,
                        });
                    }
                    continue;
                }

                let h3s = if split_gate {
                    direct_h3_children(*node)
                } else {
                    Vec::new()
                };
                if h3s.len() >= 2 {
                    push_intro(
                        &mut out,
                        *node,
                        *start,
                        h3s[0],
                        parent_idx,
                        synthetic_intro_present,
                        source,
                    );
                    for h3 in &h3s {
                        let h3_start = h3.start_position().row + 1;
                        let h3_end = span_last_row(*h3, source) + 1;
                        if has_substantive_body(*h3, h3_start, h3_end, source) {
                            out.push(SectionRange {
                                start: h3_start,
                                end: h3_end,
                                kind: SectionKind::H3Child,
                                parent_index: parent_idx,
                                synthetic_intro_present,
                            });
                        }
                    }
                } else {
                    out.push(SectionRange {
                        start: *start,
                        end: *end,
                        kind: SectionKind::Whole,
                        parent_index: parent_idx,
                        synthetic_intro_present,
                    });
                }
            }
        }
    }
    out
}

/// Append an `Intro` range covering the H2 heading + prelude before
/// `first_child` (the first H3 or the section's bullet list), but only
/// when that prelude has substantive non-heading content. Skipping the
/// heading-only case avoids a zero-cost duplicate batch — the H2
/// heading is preserved by `HeadingsOutline` (gated on by the split
/// rule's `outline_will_emit` requirement).
fn push_intro<'a>(
    out: &mut Vec<SectionRange>,
    h2_section: Node<'a>,
    h2_start: usize,
    first_child: Node<'a>,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) {
    let intro_end = (first_child.start_position().row + 1).saturating_sub(1);
    if intro_end < h2_start {
        return;
    }
    if !has_substantive_body(h2_section, h2_start, intro_end, source) {
        return;
    }
    out.push(SectionRange {
        start: h2_start,
        end: intro_end,
        kind: SectionKind::Intro,
        parent_index: parent_idx,
        synthetic_intro_present,
    });
}

/// One entry in the un-split top-level section list. `SyntheticIntro`
/// is the row range carved out by H1-unwrap to preserve the H1 heading
/// and the prelude before the first H2 (no tree-sitter node — it's a
/// virtual section). `H2Section` carries the tree-sitter node so
/// [`direct_h3_children`] and source-byte length can be derived without
/// re-walking from the root.
#[derive(Debug, Clone, Copy)]
enum TopLevelEntry<'a> {
    SyntheticIntro {
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
                    start: intro_start,
                    end: intro_end,
                });
            }
            for h2 in h2s {
                out.push(TopLevelEntry::H2Section {
                    node: h2,
                    start: h2.start_position().row + 1,
                    end: span_last_row(h2, source) + 1,
                });
            }
            return out;
        }
    }
    top.into_iter()
        .map(|s| TopLevelEntry::H2Section {
            node: s,
            start: s.start_position().row + 1,
            end: span_last_row(s, source) + 1,
        })
        .collect()
}

/// True iff the H2 section's only structural content is a single
/// `list` block (decorative blocks like trailing `<br>` or
/// image-only paragraphs are tolerated). Returns the substantive
/// list items that the caller should emit as `BulletItem` ranges, so
/// the same item set drives both the size gate and emission —
/// otherwise a section with many raw items but few substantive ones
/// could pass the gate and then emit too few ranges, suppressing a
/// legitimate H3 split.
///
/// Decorative siblings around the list (e.g. anyhow's trailing
/// `<br>`) are tolerated but not preserved by any emitted range.
/// That's load-bearing on the decorative classifiers
/// ([`is_decorative_html_block`] / [`is_decorative_paragraph`]) being
/// strict: image-only / badge-only blocks have no rendered content
/// worth scheduling.
fn should_split_by_bullets<'a>(h2_section: Node<'a>, source: &str) -> Option<Vec<Node<'a>>> {
    let mut cur = h2_section.walk();
    let mut list: Option<Node<'a>> = None;
    for child in h2_section.children(&mut cur) {
        match child.kind() {
            "atx_heading" | "setext_heading" => continue,
            "list" => {
                if list.is_some() {
                    return None;
                }
                list = Some(child);
            }
            "html_block" if is_decorative_html_block(child, source) => continue,
            "paragraph" if is_decorative_paragraph(child, source) => continue,
            // `block_continuation` is a tree-sitter-md scaffolding node
            // with no rendered content; safe to ignore.
            "block_continuation" => continue,
            _ => return None,
        }
    }
    let items: Vec<Node<'a>> = top_level_list_items(list?)
        .into_iter()
        .filter(|i| has_substantive_list_item(*i))
        .collect();
    if items.len() < BULLET_MIN_ITEMS {
        return None;
    }
    let large = items
        .iter()
        .filter(|i| i.end_byte() - i.start_byte() >= BULLET_LARGE_ITEM_BYTES)
        .count();
    if large < BULLET_MIN_LARGE_ITEMS {
        return None;
    }
    Some(items)
}

/// Top-level `list_item` children of a `list` node. Excludes nested
/// list items inside an item's body — those would be returned by
/// recursing into `list` grandchildren, which we deliberately don't
/// do (nested-list splitting is out of scope).
fn top_level_list_items<'a>(list: Node<'a>) -> Vec<Node<'a>> {
    let mut cur = list.walk();
    list.children(&mut cur)
        .filter(|c| c.kind() == "list_item")
        .collect()
}

/// True iff a `list_item` has at least one non-marker, non-continuation
/// child — i.e. a paragraph, code block, nested list, etc. An empty
/// `- ` item has only marker children, so it's filtered to avoid
/// scheduling a no-op batch (`ratio(value, 0) = INFINITY` smell).
/// Distinct from [`has_substantive_body`] because a list item has no
/// heading.
fn has_substantive_list_item(item: Node) -> bool {
    let mut cur = item.walk();
    item.children(&mut cur).any(|c| {
        !matches!(
            c.kind(),
            "list_marker_minus"
                | "list_marker_plus"
                | "list_marker_star"
                | "list_marker_dot"
                | "list_marker_parenthesis"
                | "task_list_marker_checked"
                | "task_list_marker_unchecked"
                | "block_continuation"
        )
    })
}

/// Direct H3-section children of an H2 section node. Tree-sitter-md
/// nests sections by heading level, so an H2's H3 children are direct
/// `section` children whose first heading is level 3.
fn direct_h3_children<'a>(h2_section: Node<'a>) -> Vec<Node<'a>> {
    let mut cur = h2_section.walk();
    h2_section
        .children(&mut cur)
        .filter(|c| c.kind() == "section")
        .filter(|c| first_heading_child(*c).is_some_and(|h| heading_level(h) == 3))
        .collect()
}

/// True iff the source-row range `[start, end]` of `section` has any
/// non-blank rows outside the section's heading. Used to drop an Intro
/// or H3Child sub-range whose only content is the heading itself —
/// without that filter, the post-outline marginal cost is 0 and
/// `ratio(value, 0) = INFINITY` would unconditionally schedule a no-op
/// batch.
fn has_substantive_body(section: Node, start: usize, end: usize, source: &str) -> bool {
    let Some(heading) = first_heading_child(section) else {
        return false;
    };
    let heading_first_row = heading.start_position().row + 1;
    let heading_last_row = span_last_row(heading, source) + 1;
    source
        .lines()
        .enumerate()
        .skip(start.saturating_sub(1))
        .take(end.saturating_sub(start) + 1)
        .any(|(idx, line)| {
            let row = idx + 1;
            !(heading_first_row..=heading_last_row).contains(&row) && !line.trim().is_empty()
        })
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

fn first_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cur = node.walk();
    node.children(&mut cur).find(|c| c.kind() == kind)
}

/// Last source row (0-indexed) covered by `node`, trimming a trailing empty
/// line tree-sitter-md sometimes includes in a node's span.
fn span_last_row(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    let trimmed = text.trim_end_matches(['\n', '\r']);
    let internal = trimmed.split('\n').count();
    node.start_position().row + internal.saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tree_sitter::Parser;

    fn parse(source: &str) -> Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_md::LANGUAGE.into())
            .unwrap();
        parser.parse(source, None).unwrap()
    }

    fn covered(source: &str) -> BTreeSet<usize> {
        let tree = parse(source);
        let spec = headline_spec(&tree, source).expect("headline spec");
        spec.covered_rows
    }

    fn rendered_spans(source: &str) -> Vec<Span> {
        let tree = parse(source);
        let spec = headline_spec(&tree, source).expect("headline spec");
        build_headline_spans(&PathBuf::from("README.md"), source, &spec)
    }

    /// anyhow shape: H1 setext, blank, four badge image-link lines as one
    /// paragraph, blank, prose paragraph. The headline must skip the
    /// badge paragraph.
    #[test]
    fn markdown_decorative_paragraph_skipped_anyhow_shape() {
        let src = "Anyhow\n\
                   ======\n\
                   \n\
                   [![github](https://example/badge1.svg)](https://example/repo)\n\
                   [![crates.io](https://example/badge2.svg)](https://example/crate)\n\
                   \n\
                   This library provides anyhow::Error, a trait object based error type.\n";
        let rows = covered(src);
        // Heading on rows 1-2; prose on row 7. Badges (rows 4-5) skipped.
        assert!(rows.contains(&1), "heading row 1 missing");
        assert!(rows.contains(&7), "prose row 7 missing");
        assert!(!rows.contains(&4), "badge row 4 should be skipped");
        assert!(!rows.contains(&5), "badge row 5 should be skipped");
    }

    /// otree shape: H1 + blank + bare-image paragraph + blank + tagline.
    #[test]
    fn markdown_decorative_image_only_paragraph_skipped_otree_shape() {
        let src = "# OTree - Object Tree TUI Viewer\n\
                   \n\
                   ![screenshot](assets/screenshot.png)\n\
                   \n\
                   A command line tool to view objects in TUI tree widget.\n";
        let rows = covered(src);
        assert!(rows.contains(&1), "heading row missing");
        assert!(rows.contains(&5), "tagline row missing");
        assert!(!rows.contains(&3), "image-only paragraph should be skipped");
    }

    /// soluna shape: H1 + blank + plain text-link paragraph + blank +
    /// prose. Plain text-link paragraphs are not badges and must NOT
    /// be skipped.
    #[test]
    fn markdown_link_only_paragraph_kept_soluna_shape() {
        let src = "# Soluna\n\
                   \n\
                   [Live Examples](https://example/demo)\n\
                   \n\
                   A framework for 2D games in Lua.\n";
        let rows = covered(src);
        assert!(rows.contains(&1));
        assert!(
            rows.contains(&3),
            "plain link paragraph must NOT be skipped"
        );
    }

    /// mitt shape: H1, block_quote tagline, list of features, prose
    /// paragraph. Headline includes all four blocks (regression guard).
    #[test]
    fn markdown_mitt_shape_unchanged() {
        let src = "# Mitt\n\
                   \n\
                   > Tiny 200b functional event emitter / pubsub.\n\
                   \n\
                   - **Microscopic:** weighs less than 200 bytes\n\
                   - **Useful:** wildcard event types\n\
                   \n\
                   Mitt was made for the browser, but works in any JavaScript runtime.\n\
                   \n\
                   ## Table of Contents\n";
        let rows = covered(src);
        assert!(rows.contains(&1), "heading row missing");
        assert!(rows.contains(&3), "block_quote tagline missing");
        assert!(rows.contains(&5), "list row 5 missing");
        assert!(rows.contains(&6), "list row 6 missing");
        assert!(rows.contains(&8), "closer paragraph missing");
        assert!(!rows.contains(&10), "must not pull in ## Table of Contents");
    }

    /// Nested H1→H2 immediately. Headline must NOT pull H2 body into
    /// itself; only the H1 heading row is covered.
    #[test]
    fn markdown_nested_subsection_not_pulled_in() {
        let src = "# Title\n\
                   \n\
                   ## Sub\n\
                   \n\
                   sub body\n";
        let rows = covered(src);
        assert_eq!(rows.iter().copied().collect::<Vec<_>>(), vec![1]);
    }

    /// Block_quote between H1 and H2 — include block_quote, stop
    /// before the H2 sub-section.
    #[test]
    fn markdown_blockquote_then_subsection() {
        let src = "# Title\n\
                   \n\
                   > Tagline.\n\
                   \n\
                   ## Sub\n\
                   \n\
                   sub body\n";
        let rows = covered(src);
        assert!(rows.contains(&1));
        assert!(rows.contains(&3), "block_quote row missing");
        assert!(!rows.contains(&5), "must not pull in ## Sub heading");
        assert!(!rows.contains(&7), "must not pull in sub body");
    }

    /// cmdk shape: H1 with project name then trailing image-link badges.
    /// Heading line emits Render::Truncated.
    #[test]
    fn markdown_heading_with_inline_badges_truncates_cmdk_shape() {
        let src = "# Project [![badge1](https://example/b1.svg)](https://example/r1) [![badge2](https://example/b2.svg)](https://example/r2)\n\
                   \n\
                   The actual tagline content.\n";
        let spans = rendered_spans(src);
        // Find the span at row 1 — it must be Truncated.
        let heading_span = spans
            .iter()
            .find(|s| s.start <= 1 && s.end >= 1)
            .expect("no span covers row 1");
        match &heading_span.render {
            Render::Truncated { pattern } => {
                assert!(
                    pattern.contains("Project"),
                    "pattern should keep project name: got {pattern}"
                );
                assert!(
                    !pattern.contains("badge"),
                    "pattern must not include badge text"
                );
                assert!(
                    pattern.starts_with("\\#"),
                    "pattern must include heading marker: got {pattern}"
                );
            }
            other => panic!("expected Render::Truncated, got {other:?}"),
        }
        // Tagline at row 3 still rendered.
        assert!(spans.iter().any(|s| s.start <= 3 && s.end >= 3));
    }

    /// Heading whose sole content is a link — must NOT truncate (the
    /// project-name text isn't there to keep).
    #[test]
    fn markdown_heading_with_only_link_not_truncated() {
        let src = "# [Project](https://example/repo)\n\
                   \n\
                   Tagline.\n";
        let spans = rendered_spans(src);
        let heading_span = spans
            .iter()
            .find(|s| s.start <= 1 && s.end >= 1)
            .expect("no span covers row 1");
        assert!(matches!(heading_span.render, Render::Full));
    }

    /// HTML block with only an `<img>` tag (cmdk-style centered hero).
    /// The block is decorative and should be skipped.
    #[test]
    fn markdown_decorative_html_block_skipped() {
        let src = "# Title\n\
                   \n\
                   <p align=\"center\">\n\
                   <img src=\"hero.png\" />\n\
                   </p>\n\
                   \n\
                   Tagline.\n";
        let rows = covered(src);
        assert!(rows.contains(&1));
        assert!(rows.contains(&7));
        assert!(!rows.contains(&3), "html_block row 3 should be skipped");
        assert!(!rows.contains(&4), "html_block row 4 should be skipped");
    }

    /// Plain-text autolink paragraph is NOT decorative.
    #[test]
    fn markdown_autolink_paragraph_kept() {
        let src = "# Title\n\
                   \n\
                   <https://example.com/docs>\n\
                   \n\
                   Tagline.\n";
        let rows = covered(src);
        assert!(rows.contains(&3), "autolink paragraph must be kept");
    }

    // --- HeadingsOutline tests ---

    fn outline_rows(file: &str, source: &str) -> Vec<(usize, usize)> {
        let tree = parse(source);
        collectable_outline_rows(&PathBuf::from(file), &tree, source)
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

    /// `collectable_outline_rows` returns every row regardless of cap;
    /// the count + byte caps are enforced by `outline_emits_for` at
    /// the `expand` site.
    #[test]
    fn markdown_outline_above_count_cap_gated_by_outline_emits_for() {
        let mut src = String::from("# Big File\n\nIntro.\n\n");
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("## Section {i}\n\nbody.\n\n"));
        }
        let rows = outline_rows("README.md", &src);
        assert_eq!(rows.len(), MAX_OUTLINE_HEADINGS + 5);
        assert!(!outline_emits_for(&rows, &src));
    }

    /// Long heading lines can exceed the byte cap even when the row
    /// count is safe. The codex adversarial review of v0.2's outline
    /// flagged this exact failure mode (5 100-char H2s would otherwise
    /// pass the count cap but produce a ~500-token outline that becomes
    /// a hard predecessor for every section).
    #[test]
    fn markdown_outline_byte_cap_catches_long_headings() {
        let long = "x".repeat(400);
        let src = format!(
            "# Title\n\nIntro.\n\n## {long}\n\nbody.\n\n## {long}\n\nbody.\n\n## {long}\n\nbody.\n\n## {long}\n\nbody.\n",
        );
        let rows = outline_rows("README.md", &src);
        assert!(rows.len() <= MAX_OUTLINE_HEADINGS);
        assert!(
            !outline_emits_for(&rows, &src),
            "byte cap must reject long-heading outlines"
        );
    }

    // --- H3-splitting tests (logical_sections) ---

    fn sections(file: &str, source: &str) -> Vec<SectionRange> {
        let tree = parse(source);
        logical_sections(&PathBuf::from(file), &tree, source)
    }

    /// Build a `## Heading\n\n### Sub\n<filler>` shape sized to clear
    /// `H2_SPLIT_BYTES`. Returns the source string and the row of the
    /// first `### Sub` heading.
    fn make_split_h2_source(prefix: &str, h3_count: usize, filler_per_h3: usize) -> String {
        let mut s = String::from(prefix);
        for i in 0..h3_count {
            s.push_str(&format!("\n### Sub {i}\n\n"));
            for _ in 0..filler_per_h3 {
                s.push_str(
                    "Some prose content with substance to it. \
                            More words to fill out the section body. \
                            Even more words. Plenty of bytes here.\n",
                );
            }
        }
        s
    }

    /// README with one H1 wrapping one H2 with two H3 children, body
    /// large enough to clear `H2_SPLIT_BYTES`. Should return one
    /// `Whole` (the H1-unwrap intro) plus two H3Child ranges. The H2
    /// intro range is dropped (heading-only after the H2).
    #[test]
    fn markdown_h2_split_intro_plus_h3_subsections() {
        let prefix = "# Title\n\nTagline.\n\n## Usage";
        let src = make_split_h2_source(prefix, 2, 6);
        let ranges = sections("README.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert_eq!(
            kinds,
            vec![
                SectionKind::Whole,   // H1-unwrap intro
                SectionKind::H3Child, // ### Sub 0
                SectionKind::H3Child, // ### Sub 1
            ],
            "split H2 must drop the heading-only intro and emit per-H3 ranges; got {ranges:?}",
        );
    }

    /// Substantive intro body — H2 heading followed by a real paragraph
    /// before the first H3 — must keep the Intro range.
    #[test]
    fn markdown_h2_intro_kept_when_body_substantive() {
        let prefix = "# Title\n\nTagline.\n\n## Setup\n\n\
                      Real prose intro before any subheading.\n\
                      A second sentence makes it substantive.";
        let src = make_split_h2_source(prefix, 2, 6);
        let ranges = sections("README.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert_eq!(
            kinds,
            vec![
                SectionKind::Whole,
                SectionKind::Intro,
                SectionKind::H3Child,
                SectionKind::H3Child,
            ],
            "intro with body must be kept; got {ranges:?}",
        );
    }

    /// H2 with only one H3 child stays a `Whole`. The split rule
    /// requires ≥2 H3 children.
    #[test]
    fn markdown_h2_no_split_one_h3() {
        let prefix = "# Title\n\nTagline.\n\n## Usage";
        let src = make_split_h2_source(prefix, 1, 6);
        let ranges = sections("README.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert_eq!(kinds, vec![SectionKind::Whole, SectionKind::Whole]);
    }

    /// Splittable shape but section bytes < `H2_SPLIT_BYTES` stays one
    /// `Whole` range.
    #[test]
    fn markdown_h2_no_split_under_threshold() {
        let prefix = "# Title\n\nT.\n\n## Usage";
        let src = make_split_h2_source(prefix, 2, 0);
        assert!(
            src.len() < 600,
            "test fixture must be under threshold; got {} bytes",
            src.len()
        );
        let ranges = sections("README.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert_eq!(kinds, vec![SectionKind::Whole, SectionKind::Whole]);
    }

    /// `CHANGELOG.md` shape with H3 children stays `Whole` — the
    /// changelog index-decay needs a stable per-H2 mapping.
    #[test]
    fn markdown_h2_split_skipped_for_changelog() {
        let prefix = "## v1.0";
        let src = make_split_h2_source(prefix, 2, 6);
        let ranges = sections("CHANGELOG.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert_eq!(
            kinds,
            vec![SectionKind::Whole],
            "changelog must not split; got {ranges:?}",
        );
    }

    /// Doc page (non-README, non-changelog) with H3 children does
    /// split. Gate is changelog-only, not readme-only.
    #[test]
    fn markdown_h2_split_non_readme_doc_page() {
        let prefix = "## Setup\n\nIntro paragraph that is real prose.\n\
                      A second line so the intro body counts as substantive.";
        let src = make_split_h2_source(prefix, 2, 6);
        let ranges = sections("docs/setup.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert_eq!(
            kinds,
            vec![
                SectionKind::Intro,
                SectionKind::H3Child,
                SectionKind::H3Child,
            ],
            "non-changelog doc page must split; got {ranges:?}",
        );
    }

    /// Non-split changelog: every `Whole` range's `parent_index` equals
    /// its position in the returned list. Regression guard for the
    /// index-decay contract.
    #[test]
    fn markdown_h2_parent_index_consistent_for_changelog() {
        let mut src = String::new();
        for v in 0..3 {
            src.push_str(&format!(
                "## v{v}.0\n\n### Added\n\nbullet\n\n### Fixed\n\nbullet\n\n"
            ));
        }
        let ranges = sections("CHANGELOG.md", &src);
        for (i, r) in ranges.iter().enumerate() {
            assert_eq!(
                r.parent_index, i,
                "Whole ranges must have parent_index == position; got {r:?}"
            );
            assert_eq!(r.kind, SectionKind::Whole);
        }
    }

    /// Outline gated out by row-count cap → no split, even for a
    /// splittable H2 inside the file. Without the outline carrying
    /// the H2 heading, dropping the heading-only intro would lose it.
    #[test]
    fn markdown_h2_no_split_when_outline_omitted() {
        // One splittable H2 (with two H3 children + filler), then enough
        // additional H2 stubs to push past `MAX_OUTLINE_HEADINGS`.
        let mut src = String::from("# Title\n\nTagline.\n\n");
        src.push_str(&make_split_h2_source("## Usage", 2, 6));
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("\n## Other {i}\n\nbody.\n"));
        }
        // Sanity: outline gate must reject this file.
        let outline = outline_rows("README.md", &src);
        assert!(
            !outline_emits_for(&outline, &src),
            "outline must be omitted for the test premise to hold"
        );
        let ranges = sections("README.md", &src);
        let usage = ranges
            .iter()
            .find(|r| {
                src.lines()
                    .nth(r.start.saturating_sub(1))
                    .is_some_and(|l| l.starts_with("## Usage"))
            })
            .expect("Usage section must appear in ranges");
        assert_eq!(
            usage.kind,
            SectionKind::Whole,
            "outline-omitted files must keep H2s as Whole; got {usage:?}",
        );
    }

    // --- bullet-splitting tests (logical_sections) ---

    /// Build a section with `n_items` bullet items each filled with
    /// `filler` lines of body text. Returns the source string.
    fn make_bullet_section(prefix: &str, n_items: usize, filler_per_item: usize) -> String {
        let mut s = String::from(prefix);
        for i in 0..n_items {
            s.push_str(&format!("\n- Bullet item {i} headline.\n"));
            for _ in 0..filler_per_item {
                s.push_str(
                    "  Some prose body for the item, multi-line. \
                            Enough characters to clear the per-item byte gate.\n",
                );
            }
        }
        s
    }

    /// anyhow `## Details` shape — heading + bulleted list of items
    /// each containing a code-block-ish prose blob, then a trailing
    /// `<br>` html_block before the next H2. Must split into BulletItem
    /// ranges; trailing decorative html_block is tolerated; no Intro
    /// (heading-only prelude).
    #[test]
    fn markdown_h2_split_bullets_anyhow_details_shape() {
        let prefix = "# Title\n\nTagline.\n\n## Details";
        let mut src = make_bullet_section(prefix, 3, 4);
        src.push_str("\n<br>\n\n## Next\n\nbody.\n");
        let ranges = sections("README.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert!(
            !kinds.contains(&SectionKind::Intro),
            "no Intro for heading-only prelude; got {ranges:?}"
        );
        let bullets: Vec<&SectionRange> = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::BulletItem)
            .collect();
        assert_eq!(
            bullets.len(),
            3,
            "expected 3 BulletItem ranges; got {ranges:?}"
        );
        // Trailing `<br>` row sits after the last bullet's range.
        let br_row = src
            .lines()
            .position(|l| l.trim() == "<br>")
            .map(|i| i + 1)
            .expect("test source must contain <br>");
        assert!(
            bullets.iter().all(|b| b.end < br_row),
            "<br> must not be inside any bullet range"
        );
    }

    /// Real anyhow `## Details` text (rows 21-125 of fixture README,
    /// trailing `<br>` included) must produce one BulletItem per real
    /// item. Catches "predicate filters out the item" regressions
    /// against the data the feature is sized for.
    #[test]
    fn markdown_h2_split_bullets_real_anyhow_details() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/anyhow/README.md");
        let body = std::fs::read_to_string(&path).expect("read anyhow README fixture");
        let lines: Vec<&str> = body.lines().collect();
        let details_start = lines
            .iter()
            .position(|l| l.starts_with("## Details"))
            .expect("anyhow fixture has ## Details");
        let next_h2 = lines[details_start + 1..]
            .iter()
            .position(|l| l.starts_with("## "))
            .map(|i| details_start + 1 + i)
            .expect("anyhow fixture has H2 after Details");
        let details = lines[details_start..next_h2].join("\n");
        let src = format!("# Title\n\nTagline.\n\n{details}\n## Next\n\nbody.\n");
        let ranges = sections("README.md", &src);
        let bullets: Vec<&SectionRange> = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::BulletItem)
            .collect();
        // anyhow's ## Details has 6 top-level bullets; expect each to survive.
        assert_eq!(
            bullets.len(),
            6,
            "expected 6 BulletItem ranges from real anyhow ## Details; got {ranges:?}"
        );
    }

    /// Short bullets (each well under `BULLET_LARGE_ITEM_BYTES`) →
    /// no split, falls through to Whole.
    #[test]
    fn markdown_h2_no_split_short_bullets() {
        let prefix = "# Title\n\nTagline.\n\n## Features";
        // Make the section large enough overall that the byte gate
        // can't single-handedly suppress; per-item gate must do it.
        let mut src = String::from(prefix);
        for i in 0..40 {
            src.push_str(&format!("\n- short item {i}\n"));
        }
        src.push_str("\n## Next\n\nbody.\n");
        let ranges = sections("README.md", &src);
        assert!(
            ranges
                .iter()
                .all(|r| r.kind != SectionKind::BulletItem && r.kind != SectionKind::Intro),
            "short bullets must not split; got {ranges:?}"
        );
    }

    /// Trailing decorative HTML block after the list (anyhow's
    /// `<br>` shape, or an image-only `<p><img/></p>`) is tolerated
    /// by the predicate and falls outside every emitted range. This
    /// is the same trade-off `ReadmeHeadline` and the rest of the
    /// markdown walker make for image-only / badge-only content:
    /// there's no semantic value to preserve, so dropping it is
    /// fine.
    ///
    /// (A leading decorative paragraph before the list is a
    /// theoretical case but doesn't occur in any current fixture; if
    /// one ever does, the Intro range will pick it up via
    /// `has_substantive_body`, which currently classifies any
    /// non-blank row as substantive. The decorative classifier is
    /// only consulted by the bullet-split predicate, not by the
    /// Intro construction.)
    #[test]
    fn markdown_h2_split_bullets_trailing_decorative_html_block() {
        let prefix = "# Title\n\nTagline.\n\n## Details";
        let mut src = make_bullet_section(prefix, 3, 4);
        src.push_str(
            "\n<p align=\"center\"><img src=\"./assets/footer.png\"/></p>\n\n## Next\n\nbody.\n",
        );
        let ranges = sections("README.md", &src);
        let bullets: Vec<&SectionRange> = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::BulletItem)
            .collect();
        assert_eq!(
            bullets.len(),
            3,
            "trailing decorative html_block must not block the bullet split; got {ranges:?}"
        );
        let footer_row = src
            .lines()
            .position(|l| l.contains("./assets/footer.png"))
            .map(|i| i + 1)
            .expect("test source must contain footer marker");
        let row_in_any_range =
            |row: usize| -> bool { ranges.iter().any(|r| (r.start..=r.end).contains(&row)) };
        assert!(
            !row_in_any_range(footer_row),
            "trailing image row {footer_row} leaked into a range; got {ranges:?}"
        );
    }

    /// Section has prose paragraph then list — predicate REJECTS
    /// because the paragraph is a non-decorative non-list child.
    #[test]
    fn markdown_h2_no_split_bullets_with_prose() {
        let prose = "Here are the details:\n\
                     This paragraph keeps adding bytes so the gate \
                     conditions are met but the predicate must reject.\n";
        let mut src = String::from("# Title\n\nTagline.\n\n## Details\n\n");
        src.push_str(prose);
        // Append substantive bullets too.
        for i in 0..3 {
            src.push_str(&format!(
                "\n- Bullet item {i} with extra body text long enough to clear the per-item \
                 gate without the help of any code blocks etc.\n"
            ));
        }
        src.push_str("\n## Next\n\nbody.\n");
        let ranges = sections("README.md", &src);
        assert!(
            !ranges.iter().any(|r| r.kind == SectionKind::BulletItem),
            "prose-then-list section must not bullet-split; got {ranges:?}"
        );
    }

    /// Changelog file class is gated out of bullet split (same gate as
    /// H3 split — index-decay needs a stable per-H2 mapping).
    #[test]
    fn markdown_h2_no_split_bullets_changelog() {
        let src = make_bullet_section("## v1.0", 3, 4);
        let ranges = sections("CHANGELOG.md", &src);
        assert!(
            !ranges.iter().any(|r| r.kind == SectionKind::BulletItem),
            "changelog must not bullet-split; got {ranges:?}"
        );
    }

    /// Outline-omitted file doesn't bullet-split (otherwise the H2
    /// heading would be lost, since Intro is heading-only).
    #[test]
    fn markdown_h2_no_split_bullets_when_outline_omitted() {
        let mut src = String::from("# Title\n\nTagline.\n\n");
        src.push_str(&make_bullet_section("## Details", 3, 4));
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("\n## Other {i}\n\nbody.\n"));
        }
        let outline = outline_rows("README.md", &src);
        assert!(
            !outline_emits_for(&outline, &src),
            "outline must be omitted for the test premise to hold"
        );
        let ranges = sections("README.md", &src);
        let details = ranges
            .iter()
            .find(|r| {
                src.lines()
                    .nth(r.start.saturating_sub(1))
                    .is_some_and(|l| l.starts_with("## Details"))
            })
            .expect("Details section must appear");
        assert_eq!(
            details.kind,
            SectionKind::Whole,
            "outline-omitted file must keep H2 as Whole; got {details:?}"
        );
    }

    /// `# Title` README's first real H2 must get readme-index decay
    /// factor 1.0 (i.e. be unscaled). The synthetic H1-unwrap intro
    /// sits at `parent_index = 0`, so naïvely keying the decay off
    /// `parent_index` would scale "## Install" to ~0.90 — the exact
    /// codex-flagged regression this test guards against.
    #[test]
    fn markdown_readme_index_decay_skips_synthetic_intro() {
        let src = "# Title\n\nTagline.\n\n## Install\n\nbody\n\n## Use\n\nbody\n";
        let ranges = sections("README.md", src);
        // Three ranges: synthetic intro, ## Install, ## Use.
        assert_eq!(ranges.len(), 3, "got {ranges:?}");
        let intro = &ranges[0];
        assert_eq!(intro.parent_index, 0);
        assert!(intro.synthetic_intro_present);

        let install = &ranges[1];
        assert_eq!(install.parent_index, 1);
        assert!(install.synthetic_intro_present);
        assert_eq!(
            readme_index_decay(install),
            1.0,
            "first real H2 must be unscaled (readme h2_idx = 0)"
        );

        let use_ = &ranges[2];
        assert_eq!(use_.parent_index, 2);
        let f = readme_index_decay(use_);
        assert!(f < 1.0 && f > 0.7, "second real H2 should decay; got {f}");
    }

    /// READMEs without an H1 wrap (no synthetic intro) — first H2 is
    /// `parent_index = 0` and gets factor 1.0 directly.
    #[test]
    fn markdown_readme_index_decay_no_synthetic_intro() {
        let src = "## Install\n\nbody\n\n## Use\n\nbody\n";
        let ranges = sections("README.md", src);
        assert_eq!(ranges.len(), 2);
        assert!(!ranges[0].synthetic_intro_present);
        assert_eq!(readme_index_decay(&ranges[0]), 1.0);
    }

    /// cmdk shape: H1 with badge tail + 3 H2s. Headline truncates row
    /// 1 to drop the badges; outline must skip row 1 so the headline's
    /// `Render::Truncated` isn't overridden.
    #[test]
    fn markdown_outline_preserves_cmdk_headline_truncation() {
        let src = "# cmdk [![badge1](https://example/b1.svg)](https://example/r1) [![badge2](https://example/b2.svg)](https://example/r2)\n\
                   \n\
                   The actual tagline content.\n\
                   \n\
                   ## Install\n\
                   \n\
                   body\n\
                   \n\
                   ## Use\n\
                   \n\
                   body\n\
                   \n\
                   ## Parts\n\
                   \n\
                   body\n";
        let rows = outline_rows("README.md", src);
        let starts: Vec<usize> = rows.iter().map(|(s, _)| *s).collect();
        assert!(
            !starts.contains(&1),
            "outline must not claim H1 row; would override headline truncation. got {starts:?}"
        );
        assert_eq!(starts, vec![5, 9, 13]);
    }
}
