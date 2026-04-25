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
//! - `Section { file, section_index }` — one H2 section, 0-indexed. For
//!   READMEs, this is the split replacement of the old monolithic
//!   "body" batch. For other markdown files (changelogs, doc pages),
//!   we emit one per top-level section so the file can land
//!   piece-by-piece. Predecessor (when emitted): outline → headline →
//!   none, picking the deepest available so all heading-row overlaps
//!   are ancestor-overlaps.

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{BatchKey, FsKey, MarkdownKey, ResolvedBatch, ValueSignals};
use crate::content::{BatchContent, Render, Span};
use crate::tokenizer;
use crate::value::depth_factor;

use super::{Candidate, FileLines, WalkCtx, fs::files_with_extension, single_file_lines_batch};

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

const TOKENS_PER_HEADING_ROW: usize = 12;

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

        let Some((source, tree)) = parse_md(ctx, &file) else {
            continue;
        };
        let section_count = logical_sections(tree.root_node(), &source).len();
        if section_count == 0 {
            continue;
        }
        let outline_rows = collectable_outline_rows(&file, &tree, &source);
        let outline_emits = outline_emits_for(&outline_rows, &source);

        let is_readme = is_readme(&file);
        let headline_key = is_readme.then(|| MarkdownKey::ReadmeHeadline { file: file.clone() });
        let outline_key =
            outline_emits.then(|| MarkdownKey::HeadingsOutline { file: file.clone() });

        if let Some(h) = &headline_key {
            out.push(candidate(
                h.clone(),
                readme_headline_signals(&file, ctx),
                60,
            ));
        }
        if let Some(o) = &outline_key {
            let mut cand = candidate(
                o.clone(),
                headings_outline_signals(&file, ctx),
                (outline_rows.len() * TOKENS_PER_HEADING_ROW).max(30),
            );
            if let Some(h) = &headline_key {
                cand = cand.with_predecessor(BatchKey::Markdown(h.clone()));
            }
            out.push(cand);
        }

        let section_predecessor = outline_key
            .as_ref()
            .or(headline_key.as_ref())
            .cloned()
            .map(BatchKey::Markdown);

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
            if let Some(p) = section_predecessor.clone() {
                cand = cand.with_predecessor(p);
            }
            out.push(cand);
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

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Markdown(mk) = key else {
        return None;
    };
    match mk {
        MarkdownKey::SummaryWhole { file } => mat_summary(file, ctx),
        MarkdownKey::ReadmeHeadline { file } => mat_readme_headline(file, ctx),
        MarkdownKey::HeadingsOutline { file } => mat_headings_outline(file, ctx),
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

fn headings_outline_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.7,
        follow_up_minimization: 0.55,
        zero_tool_call_understanding: 0.4,
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
    let spec = headline_spec(&tree, &source)?;
    let spans = build_headline_spans(file, &source, &spec);
    if spans.is_empty() {
        return None;
    }
    Some(ResolvedBatch {
        content: BatchContent::Lines { spans },
        signals: readme_headline_signals(file, ctx),
    })
}

fn mat_headings_outline(file: &Path, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let (source, tree) = parse_md(ctx, file)?;
    let rows = collectable_outline_rows(file, &tree, &source);
    if rows.len() < 2 {
        return None;
    }
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for (start, end) in &rows {
        for r in *start..=*end {
            full.push(r);
        }
        ellipses.push(end + 1);
    }
    single_file_lines_batch(
        file,
        &source,
        FileLines::new(full).with_ellipses(ellipses),
        headings_outline_signals(file, ctx),
    )
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

fn mat_section(file: &Path, section_index: usize, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let (source, tree) = parse_md(ctx, file)?;
    let (start, end) = nth_section_range(&tree, &source, section_index)?;

    // For README section 0, exclude lines already covered by
    // `ReadmeHeadline`. Otherwise the two batches overlap on short
    // READMEs, Section 0's marginal cost drops to zero after dedupe,
    // and `ratio(value, 0) = INFINITY` gives it unconditional
    // scheduling priority — a smell even though the duplicate apply
    // is a no-op.
    let effective_start = if section_index == 0
        && is_readme(file)
        && let Some(spec) = headline_spec(&tree, &source)
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

fn nth_section_range(tree: &Tree, source: &str, n: usize) -> Option<(usize, usize)> {
    logical_sections(tree.root_node(), source)
        .into_iter()
        .nth(n)
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
