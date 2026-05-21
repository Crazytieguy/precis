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
//!   subdivides via one of three rules when applicable. *Bullet split*
//!   (anyhow's `## Details` shape) — an H2 whose non-decorative
//!   content is a single `list` block becomes one `BulletItem` per
//!   substantive top-level item. *H3 split* (content-heavy H2 with
//!   ≥2 H3 children) becomes one `Intro` (when its body is
//!   substantive) plus one `H3Child` per H3. *Body-block split*
//!   refines large H3 children into direct paragraph/code/list blocks,
//!   and can split list-only H2 sections into one body block per item.
//!   All per-child kinds carry a global signal scale
//!   (`SUB_SECTION_SIGNAL_SCALE` / `BODY_BLOCK_SIGNAL_SCALE`) to keep
//!   them from over-ranking once the marginal cost drops to
//!   per-sub-section size. Changelog-class files (CHANGELOG /
//!   CONTRIBUTING / CHANGES) are gated out of splitting. H3/prose
//!   body-block splitting also requires `HeadingsOutline` so heading
//!   context is preserved; list-only H2 splits are allowed without an
//!   outline because each item is self-contained.
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
use crate::value::{is_orientation_doc, mix_signals};

use super::{
    FileLines, WalkCtx, extend_nonblank_rows, fs::files_with_extension, node_end_row_trimmed,
    path_depth_factor, single_file_lines_content,
};

/// Upper bound on collectable heading rows before `HeadingsOutline`
/// suppresses itself. The outline is the predecessor of every section,
/// so an oversize outline that fails to fit near the budget tail would
/// block the whole file.
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

/// Multiplier for `SectionKind::BodyBlock`, which can be as small as a
/// single paragraph or list item. Body blocks are useful budget fillers
/// but should not outrank intact declarations / larger doc sections just
/// because their marginal cost is tiny.
const BODY_BLOCK_SIGNAL_SCALE: f64 = 0.60;

/// Minimum source-byte length before a section or sub-section is split
/// into body blocks. WHY: below this, the current divergence corpus mostly
/// gains schedule churn rather than useful budget relief; it stays lower
/// than `H2_SPLIT_BYTES` because it can apply after H2 splitting too.
const BODY_BLOCK_SPLIT_BYTES: usize = 350;

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
    let mut out = Vec::new();

    // RST README: emit a single `ReadmeHeadline` batch with the
    // file's non-decorative content. No tree-sitter parse — we
    // line-scan, drop `.. directive::` blocks (image / badge /
    // hyperlink targets), and render the rest. This is enough
    // to unblock typeguard / pluggy whose NS pins the README's
    // lede + brief mode/feature paragraphs (a single substantive
    // block in source order). Larger RST files get the same
    // single-batch treatment capped by `RST_README_LINE_CAP`;
    // beyond that the file is treated as too big for one slot
    // and skipped. We do NOT emit `HeadingsOutline` or per-section
    // `Section` batches for RST (tree-sitter-md can't parse the
    // setext-style `===` / `---` underlines), so the budget for
    // RST READMEs is one anchor batch.
    for file in super::fs::files_with_extension(dir, "rst") {
        if !is_readme_rst(&file) {
            continue;
        }
        // Skip README.rst inside subdirectories (changelog/,
        // downstream/, etc.). The root-level README is the only
        // ReadmeHeadline anchor; nested READMEs are admin files
        // describing the subtree's content convention, and crowding
        // the schedule with them displaces real source content.
        if dir != ctx.root() {
            continue;
        }
        let Some(source) = ctx.read_source(&file) else {
            continue;
        };
        if let Some(content) = build_rst_readme_content(&file, &source) {
            out.push(Batch {
                key: MarkdownKey::ReadmeHeadline { file: file.clone() }.into(),
                predecessor: None,
                content,
                // RST headlines carry the *entire* substantive README in
                // one batch (no `Section` split — see the block-comment
                // above). They're the RST equivalent of the README's
                // ReadmeHeadline + every `## …` section combined, so the
                // value tier should match the broader-anchor role. The
                // long-RST cases (beets's 550-token headline) sit near
                // the auto-injection budget edge; the boost keeps them
                // inside the budget rather than displaced behind
                // peripheral per-file batches.
                value: readme_headline_value(&file, ctx) * RST_README_HEADLINE_FACTOR,
            });
        }
    }

    if md_files.is_empty() {
        return out;
    }
    let sibling_md_count = md_files.len();
    for file in md_files {
        let name = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        // Auto-injected agent docs (AGENTS.md / CLAUDE.md / skill
        // files) are loaded into the model's context by the harness,
        // so emitting their bodies is pure waste. Skip the prose-body
        // batches (SummaryWhole + Section); the structural batches
        // (HeadingsOutline + ReadmeHeadline) still emit so the file's
        // shape stays discoverable at large budgets, riding the 0.1×
        // value discount applied via `non_essential_factor`.
        let suppress_body = ctx.is_auto_injected_doc_file(&file);

        if name.eq_ignore_ascii_case("SUMMARY.md") {
            if suppress_body {
                continue;
            }
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
                value: headings_outline_value(&file, ctx, sibling_md_count),
            });
            outline_emitted = Some(BatchKey::Markdown(o.clone()));
        }

        let section_predecessor = outline_emitted.or(headline_emitted);

        if suppress_body {
            continue;
        }

        let total_h2_count = section_h2_count(&ranges);
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
                    value: section_value(&file, range, ctx, total_h2_count),
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

/// Multiplier applied to the RST `ReadmeHeadline` batch. RST READMEs
/// don't get `Section` / `HeadingsOutline` companion batches (no
/// tree-sitter-md parse), so the lone `ReadmeHeadline` batch carries
/// the *entire* substantive README in one chunk — far more NS
/// coverage per batch than a Markdown headline (which is paired with
/// per-section content). Lift its value tier to reflect the broader
/// anchor role; otherwise long-RST READMEs (beets's 550-token
/// headline) sit past the auto-injection budget because their cost
/// loses the V/C race despite the per-token coverage being high.
const RST_README_HEADLINE_FACTOR: f64 = 1.5;

fn headings_outline_value(file: &Path, ctx: &WalkCtx, sibling_md_count: usize) -> f64 {
    mix_signals(
        0.7,
        0.55,
        0.4,
        super::file_depth_factor(file, ctx, is_orientation_doc(file)),
    ) * dense_md_sibling_factor(file, sibling_md_count)
        * non_anchor_outline_factor(file, ctx)
}

/// Saturate the per-file outline value when the file sits in a dir
/// with many `.md` siblings: the directory listing already names
/// every file, so emitting a ranked outline per file crowds source
/// content. README and orientation docs are exempt — they're the
/// anchor, not the noise.
fn dense_md_sibling_factor(file: &Path, sibling_md_count: usize) -> f64 {
    if is_readme(file) || is_orientation_doc(file) {
        return 1.0;
    }
    const DENSE_THRESHOLD: usize = 6;
    if sibling_md_count <= DENSE_THRESHOLD {
        return 1.0;
    }
    ((DENSE_THRESHOLD as f64) / (sibling_md_count as f64)).sqrt()
}

/// Damp the outline value for loose `docs/<file>.md` pages — markdown
/// files at depth 2 that aren't README or orientation docs
/// (ARCHITECTURE / OVERVIEW / DESIGN / STRUCTURE). NS authors anchor
/// the outline batch on the README, on a project-orientation doc, or
/// on the entries of a curated docs site (a `docs/guides/`,
/// `docs/reference/`, `site/content/`, or `guide/src/` subdirectory);
/// standalone pages like `docs/installation.md`, `docs/usage.md`, or
/// `docs/index.md` rarely appear in NSes yet still consume budget at
/// the per-file outline rank.
///
/// Depth-based gating distinguishes the two cases. Depth 1
/// (CONTRIBUTING.md, IMAGES.md) keeps full value — root-level admin /
/// topical docs are rare and occasionally NS-anchored. Depth ≥ 3
/// (docs/reference/X.md, docs/guides/X.md, guide/src/Y.md) also keeps
/// full value — a subdirectory under `docs/` is a curation signal
/// that NS authors do reference. The damp targets exactly the
/// in-between case: `docs/X.md` with no further organization.
const NON_ANCHOR_OUTLINE_FACTOR: f64 = 0.4;

fn non_anchor_outline_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if is_readme(file) || is_orientation_doc(file) {
        return 1.0;
    }
    if ctx.depth_from_root(file) != 2 {
        return 1.0;
    }
    NON_ANCHOR_OUTLINE_FACTOR
}

fn readme_section_value(
    file: &Path,
    range: &SectionRange,
    ctx: &WalkCtx,
    total_h2_count: usize,
) -> f64 {
    mix_signals(0.55, 0.8, 0.7, path_depth_factor(file, ctx))
        * readme_index_decay(range, total_h2_count)
        * canonical_usage_section_factor(range)
        * features_section_factor(range)
}

/// Boost for README H2 `Whole` sections whose title is a canonical-
/// usage marker (see [`is_canonical_usage_h2_title`]) — the demo
/// fence inside is the highest-value follow-up to the headline.
const CANONICAL_USAGE_SECTION_FACTOR: f64 = 1.5;

fn canonical_usage_section_factor(range: &SectionRange) -> f64 {
    if range.parent_is_canonical_usage_h2 && matches!(range.kind, SectionKind::Whole) {
        CANONICAL_USAGE_SECTION_FACTOR
    } else {
        1.0
    }
}

/// Boost for README H2 `Whole` sections whose title is a features-list
/// marker (see [`is_features_h2_title`]) — the high-density capability
/// inventory that anchors many NS rows.
const FEATURES_SECTION_FACTOR: f64 = 1.6;

fn features_section_factor(range: &SectionRange) -> f64 {
    if range.parent_is_features_h2 && matches!(range.kind, SectionKind::Whole) {
        FEATURES_SECTION_FACTOR
    } else {
        1.0
    }
}

/// Index decay for README sections. Long READMEs (>= 18 H2s) switch
/// to a steeper falloff so kitchen-sink documentation projects' tail
/// sections don't crowd source-code anchors.
fn readme_index_decay(range: &SectionRange, total_h2_count: usize) -> f64 {
    let h2_idx = if range.synthetic_intro_present {
        range.parent_index.saturating_sub(1)
    } else {
        range.parent_index
    };
    if total_h2_count >= 18 {
        index_decay(h2_idx, 0.35, 0.4)
    } else {
        index_decay(h2_idx, 0.15, 0.7)
    }
}

/// Count of real H2 sections — `parent_index` is monotonic per
/// `logical_sections`, so `max(parent_index) + 1` gives the H2
/// cardinality (minus one when a synthetic H1-unwrap intro occupies
/// `parent_idx 0`).
fn section_h2_count(ranges: &[SectionRange]) -> usize {
    let synthetic_intro_present = ranges.first().is_some_and(|r| r.synthetic_intro_present);
    let max_parent = ranges.iter().map(|r| r.parent_index).max();
    match max_parent {
        Some(max) if synthetic_intro_present => max,
        Some(max) => max + 1,
        None => 0,
    }
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
    let is_orientation = is_orientation_doc(file);
    // Root-level orientation docs (depth 1) don't take the cat bump —
    // depth-1 already wins the cost race, and bumping them regresses
    // fixtures whose NS anchors only specific sections rather than the
    // whole doc.
    let is_nested_orientation = is_orientation && ctx.depth_from_root(file) > 1;
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
    let cat = if is_guide {
        0.5
    } else if is_nested_orientation {
        0.65
    } else {
        0.3
    };
    mix_signals(
        cat,
        0.5,
        0.5,
        super::file_depth_factor(file, ctx, is_orientation),
    ) * scale
}

/// Per-section value. Child ranges scale the parent's value — identical
/// weights would over-rank them on the value/cost ratio once the cost
/// drops to per-sub-section size. `Intro` keeps full weight (it carries
/// the H2 heading + topic prelude).
fn section_value(file: &Path, range: &SectionRange, ctx: &WalkCtx, total_h2_count: usize) -> f64 {
    let parent = if is_readme(file) {
        readme_section_value(file, range, ctx, total_h2_count)
    } else {
        heading_slab_value(file, range.parent_index, ctx)
    };
    let h3_scale = if range.parent_is_concept_h2 {
        README_CONCEPT_H3_SIGNAL_SCALE
    } else if is_readme(file) {
        README_SUB_SECTION_SIGNAL_SCALE
    } else {
        SUB_SECTION_SIGNAL_SCALE
    };
    let bullet_scale = if is_readme(file) {
        README_SUB_SECTION_SIGNAL_SCALE
    } else {
        SUB_SECTION_SIGNAL_SCALE
    };
    // BodyBlocks of a concept H3 (monaco-editor's Providers split into
    // two paragraphs) are still concept content; lift them to the same
    // scale as a single-paragraph concept H3 (Editors). The marker is
    // gated on H3 byte size in `push_h3_child_or_body_blocks`, so this
    // doesn't fire for the long-topical-section H3s that happen to
    // live under a concept H2 (htmy's `### Components`).
    let body_block_scale = if range.parent_is_concept_h2 {
        README_CONCEPT_H3_SIGNAL_SCALE
    } else {
        BODY_BLOCK_SIGNAL_SCALE
    };
    match range.kind {
        SectionKind::Whole | SectionKind::Intro => parent,
        SectionKind::H3Child => parent * h3_scale,
        SectionKind::BulletItem => parent * bullet_scale,
        SectionKind::BodyBlock => parent * body_block_scale,
    }
}

/// README H3 children often ARE the canonical concept rows NS authors
/// anchor on ("Models", "URIs", "Editors" in monaco-editor), not
/// elaboration sub-sections of a larger H2. Bump above the generic
/// `SUB_SECTION_SIGNAL_SCALE` so these compete with per-method class
/// body fragments in the early budget. Non-README docs (CHANGELOG,
/// ARCHITECTURE, /docs pages) keep the conservative discount.
const README_SUB_SECTION_SIGNAL_SCALE: f64 = 0.55;

/// Stronger scale for H3 children under a README H2 whose title is an
/// orientation-concept marker (`## Concepts`, `## Architecture`, …; see
/// [`is_concept_h2_title`]). Such H3s are the named concept definitions
/// NS authors anchor on; the generic
/// `README_SUB_SECTION_SIGNAL_SCALE` keeps them behind per-decl
/// surfaces in the early budget. The boost only applies when the
/// parent H2 title matches the marker set, so it doesn't lift
/// elaboration H3s under arbitrary topical H2s.
const README_CONCEPT_H3_SIGNAL_SCALE: f64 = 1.0;

/// Upper byte size for an H3 to still propagate the concept boost
/// (`README_CONCEPT_H3_SIGNAL_SCALE`) onto its `BodyBlock`s when split.
/// A single-paragraph concept H3 stays as one `H3Child` (no split,
/// the threshold doesn't apply). A multi-paragraph concept *definition*
/// — monaco-editor's Providers (~500 B, 2 paragraphs) — falls in the
/// window and keeps the boost on each paragraph. A multi-KB H3 living
/// under a concept H2 (htmy's `### Components` at ~4 KB) is a long
/// topical section rather than a concept definition; treating every
/// paragraph as a top-tier concept row crowds NS-anchored content.
const CONCEPT_H3_BODY_BLOCK_MAX_BYTES: usize = 700;

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

fn is_readme_rst(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("README.rst"))
}

/// Render the RST README's title + first substantive paragraph as a
/// single `ReadmeHeadline` batch. Stops at the first setext-style
/// heading after the title (a row of `=` or `-` whose width covers the
/// preceding non-blank line) — analogous to how
/// [`headline_spec`] stops at the first H2 of an MD README.
///
/// "Decorative" lines (top-level `.. directive::` blocks: `.. image::`,
/// `.. _ref:`, `.. |substitution| image::`, bare `.. badges` comments)
/// are skipped along with their indented continuation. The setext H1
/// underline directly under the title is kept (it identifies the
/// title); subsequent setext underlines after the lede end the
/// headline.
fn build_rst_readme_content(file: &Path, source: &str) -> Option<BatchContent> {
    let src_lines: Vec<&str> = source.lines().collect();
    if src_lines.is_empty() {
        return None;
    }
    let mut keep: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut seen_title = false;
    while i < src_lines.len() {
        let line = src_lines[i];
        let trimmed = line.trim_start();
        if trimmed.starts_with("..") {
            let directive_indent = line.len() - trimmed.len();
            i += 1;
            while i < src_lines.len() {
                let next = src_lines[i];
                if next.trim().is_empty() {
                    i += 1;
                    continue;
                }
                let next_indent = next.len() - next.trim_start().len();
                if next_indent > directive_indent {
                    i += 1;
                } else {
                    break;
                }
            }
            continue;
        }
        // Setext-style heading detection: a non-blank line followed by
        // an underline of `=`, `-`, `~`, `^`, `*`, `+`, or `#` whose
        // width is at least the line's. The first such heading is the
        // title; subsequent ones bound the headline.
        if i + 1 < src_lines.len()
            && let Some(prev) = src_lines.get(i).filter(|l| !l.trim().is_empty())
            && is_rst_underline(src_lines[i + 1], prev.trim_end().chars().count())
        {
            if seen_title {
                break;
            }
            keep.push(i + 1);
            keep.push(i + 2);
            seen_title = true;
            i += 2;
            continue;
        }
        // Overline form (title surrounded by underline rows). Detect:
        // a punctuation row followed by a title row followed by the
        // same punctuation row.
        if !seen_title
            && i + 2 < src_lines.len()
            && is_rst_underline(src_lines[i], 1)
            && is_rst_underline(src_lines[i + 2], 1)
            && src_lines[i].trim() == src_lines[i + 2].trim()
            && !src_lines[i + 1].trim().is_empty()
        {
            keep.push(i + 1);
            keep.push(i + 2);
            keep.push(i + 3);
            seen_title = true;
            i += 3;
            continue;
        }
        keep.push(i + 1);
        i += 1;
    }
    while keep
        .first()
        .is_some_and(|&row| src_lines.get(row - 1).is_none_or(|l| l.trim().is_empty()))
    {
        keep.remove(0);
    }
    while keep
        .last()
        .is_some_and(|&row| src_lines.get(row - 1).is_none_or(|l| l.trim().is_empty()))
    {
        keep.pop();
    }
    if keep.is_empty() {
        return None;
    }
    single_file_lines_content(file, source, FileLines::new(keep))
}

fn is_rst_underline(line: &str, min_width: usize) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < min_width || trimmed.is_empty() {
        return false;
    }
    let first = trimmed.chars().next().unwrap();
    matches!(first, '=' | '-' | '~' | '^' | '*' | '+' | '#') && trimmed.chars().all(|c| c == first)
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
    // Prelude content: a README that opens with HTML title blocks /
    // badges / a one-paragraph lede before the first heading
    // (microbootstrap, many JS/TS libs) wraps that material in a
    // heading-less section that `headed_sections` skips. Surface up to
    // one substantive paragraph (skipping leading decorative
    // paragraphs / image-only HTML) so the lede's content lines are
    // covered. The heading and post-heading walk below still apply.
    extend_prelude_lede(&mut covered, tree.root_node(), section, source);
    extend_rows_inclusive(&mut covered, heading, source);
    let heading_first_row = heading.start_position().row + 1;

    // Walk siblings after the heading, skipping leading decorative
    // paragraphs / image-only HTML blocks / admin block_quotes
    // (`> [!WARNING]` callouts, long deprecation notices); then include
    // subsequent blocks until we hit the first substantive paragraph.
    // If that paragraph is a short tagline under an H1 (posting's
    // bold-tagline shape), take one more non-decorative block — the
    // prose lede that follows it.
    let post: Vec<Node> = children_after(section, heading);
    let mut i = 0;
    while i < post.len() {
        let block = post[i];
        if is_section_boundary(block.kind()) {
            break;
        }
        match block.kind() {
            "paragraph" if is_decorative_paragraph(block, source) => {}
            "paragraph" if is_admin_emoji_paragraph(block, source) => {}
            "html_block" if is_decorative_html_block(block, source) => {}
            "block_quote" if is_admin_block_quote(block, source) => {}
            _ => break,
        }
        i += 1;
    }
    let mut state = HeadlineExtend::SeekFirst;
    while i < post.len() {
        let block = post[i];
        if is_section_boundary(block.kind()) {
            break;
        }
        let is_decorative_block = match block.kind() {
            "paragraph" => is_decorative_paragraph(block, source),
            "html_block" => is_decorative_html_block(block, source),
            "block_quote" => is_admin_block_quote(block, source),
            _ => false,
        };
        if is_decorative_block {
            i += 1;
            continue;
        }
        if block.kind() == "paragraph" && is_admin_emoji_paragraph(block, source) {
            i += 1;
            continue;
        }
        extend_rows_inclusive(&mut covered, block, source);
        match state {
            HeadlineExtend::SeekFirst => {
                if block.kind() == "paragraph" {
                    // The "tagline + lede" extension only fires under
                    // the project's title heading (H1). For non-H1
                    // first-headed sections (`### Usage`, `## About`)
                    // the first substantive paragraph IS the section
                    // body and shouldn't pull in further content.
                    if heading_level(heading) == 1 && is_short_substantive_block(block, source) {
                        state = HeadlineExtend::SeekExtension;
                    } else {
                        break;
                    }
                }
            }
            HeadlineExtend::SeekExtension => break,
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

/// True when a `block_quote` is an admin / warning / callout — a
/// GitHub-flavored callout (`> [!WARNING]`, `> [!NOTE]`, etc.) OR a
/// multi-paragraph block_quote (4+ source rows). Short single-paragraph
/// block_quotes (typically taglines — `> Ky is a tiny and elegant HTTP
/// client...`) are NOT classified as admin and stay in the headline.
fn is_admin_block_quote(block: Node, source: &str) -> bool {
    let raw = &source[block.start_byte()..block.end_byte()];
    // GitHub-style callouts open with `> [!TYPE]`.
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
    // Multi-paragraph or long block_quote (4+ lines including the
    // line numbers of the first row through the last). Single-line
    // taglines stay.
    let start = block.start_position().row;
    let end = block.end_position().row;
    end.saturating_sub(start) >= 3
}

/// A "navigation paragraph" is a paragraph whose substantive content is
/// nothing but cross-reference links separated by separator punctuation
/// (`•`, `·`, `|`, `/`, `,`, dashes). Common in multi-language READMEs
/// (`[English](url) • [中文](url) • ...`) and in tagline-rich docs that
/// list related projects. The links carry no orientation value at the
/// budget where the headline lives — treat as decorative so the
/// headline walker doesn't burn its prelude on them.
fn is_nav_link_paragraph(para: Node, source: &str) -> bool {
    let Some(inline_block) = first_child_of_kind(para, "inline") else {
        return false;
    };
    let inline_text = &source[inline_block.start_byte()..inline_block.end_byte()];
    let Some(tree) = parse_inline(inline_text) else {
        return false;
    };
    let root = tree.root_node();
    let named = named_decorative_candidates(root, inline_text);
    if named.len() < 3 {
        return false;
    }
    if !named.iter().all(|n| {
        matches!(
            n.kind(),
            "inline_link" | "full_reference_link" | "collapsed_reference_link" | "shortcut_link"
        )
    }) {
        return false;
    }
    let mut cursor = 0usize;
    for n in &named {
        if !is_separator_gap(&inline_text[cursor..n.start_byte()]) {
            return false;
        }
        cursor = n.end_byte();
    }
    is_separator_gap(&inline_text[cursor..])
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
    let mut out = String::with_capacity(raw.len());
    let bytes = raw.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'<' {
            while i < bytes.len() && bytes[i] != b'>' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        if matches!(b, b'>' | b'*' | b'_' | b'`') {
            i += 1;
            continue;
        }
        out.push(b as char);
        i += 1;
    }
    out
}

/// State for the post-heading walk inside [`headline_spec`].
/// Transitions: `SeekFirst` (taking blocks until the first substantive
/// paragraph) → `SeekExtension` (only if that paragraph was a short
/// tagline under an H1, take one more non-decorative block then stop).
#[derive(Copy, Clone)]
enum HeadlineExtend {
    SeekFirst,
    SeekExtension,
}

/// An html_block is a "navigation block" iff it contains 3+ anchor
/// tags AND the text between them (after stripping all tags) is
/// dominantly separator punctuation. Matches superstruct's
/// `<p align="center"> <a href="#usage">Usage</a> • ...</p>` shape.
/// Decorative for headline purposes — the link labels alone add no
/// orientation value the FS listing doesn't already imply.
fn is_nav_link_html_block(block: Node, source: &str) -> bool {
    let raw = &source[block.start_byte()..block.end_byte()];
    let anchor_count = raw.matches("<a ").count();
    if anchor_count < 3 {
        return false;
    }
    let stripped = strip_html_tags(raw);
    let separator_tokens = stripped
        .split_whitespace()
        .filter(|w| {
            !w.is_empty()
                && w.chars().all(|c| {
                    matches!(
                        c,
                        '\u{2022}'
                            | '\u{00B7}'
                            | '|'
                            | '/'
                            | '\\'
                            | ','
                            | '-'
                            | '\u{2014}'
                            | '\u{2013}'
                    )
                })
        })
        .count();
    separator_tokens + 1 >= anchor_count
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
    /// True when this range's parent H2's title is an orientation-concept
    /// marker (`Concepts`, `Architecture`, `Overview`, …; see
    /// [`is_concept_h2_title`]). Lets `section_value` boost `H3Child`
    /// and concept-`BodyBlock` ranges under such H2s — those are the
    /// canonical concept definitions NS authors anchor on. Always false
    /// for non-README files. Always false for `BulletItem` / `Whole` /
    /// `Intro` (those kinds aren't concept-row content even when they
    /// live under a matching H2). `BodyBlock`s inherit the flag only
    /// when produced by `push_h3_child_or_body_blocks` splitting a
    /// concept H3 — paragraphs of a long concept H3 like Providers
    /// are still concept content.
    parent_is_concept_h2: bool,
    /// True when this range's parent H2 is a README canonical-usage
    /// section: a `## Usage` / `## Sample usage` / `## Example(s)` /
    /// `## Quick start` / `## Getting started` / `## Basic usage` /
    /// `## Demo` header (see [`is_canonical_usage_h2_title`]). NS
    /// authors anchor on the canonical demo snippet inside these
    /// sections (sqlite-vec's `## Sample usage` covers NS 1.5, 1.9,
    /// 1.10); the section's value gets a boost so it competes with
    /// the cheaper trailing `## See Also`-style bullet sections that
    /// otherwise displace it on pure cost. Always false for non-README
    /// files.
    parent_is_canonical_usage_h2: bool,
    /// True when this range's parent H2 is a README features-list
    /// section: `## Features` / `## Key features` / `## Feature
    /// highlights` (see [`is_features_h2_title`]). The bullet list
    /// inside such a section is the README's high-density capability
    /// inventory — NS authors regularly anchor on it across Rust /
    /// Go / Python / TS projects. Always false for non-README files.
    parent_is_features_h2: bool,
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
    /// One direct body block (paragraph, code block, nested section, or
    /// list item) inside a long split section.
    BodyBlock,
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
///   H3 child, or smaller `BodyBlock`s inside that H3 when it is still
///   large and block-structured.
/// - **Body-block split** refines large H3 children into direct
///   paragraph/code/list blocks, and also fires as a fallback for
///   list-only H2s whose items are too small for the specialized
///   bullet split.
///
/// All other top-level entries emit one `Whole` range. H3/prose splits
/// require the outline to preserve heading rows — Intro ranges with
/// empty bodies are elided to avoid `ratio(value, 0) = INFINITY` no-op
/// batches, so without an outline the heading would otherwise be lost.
fn logical_sections(file: &Path, tree: &Tree, source: &str) -> Vec<SectionRange> {
    let entries = top_level_entries(tree.root_node(), source);
    let outline_will_emit = {
        let rows = collectable_outline_rows(file, tree, source);
        outline_emits_for(&rows, source)
    };
    let split_eligible_file = !is_changelog_class(file);
    let synthetic_intro_present =
        matches!(entries.first(), Some(TopLevelEntry::SyntheticIntro { .. }));

    let readme = is_readme(file);
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
                    parent_is_concept_h2: false,
                    parent_is_canonical_usage_h2: false,
                    parent_is_features_h2: false,
                });
            }
            TopLevelEntry::H2Section { node, start, end } => {
                let bytes = node.end_byte() - node.start_byte();
                let structural_split_gate =
                    split_eligible_file && outline_will_emit && bytes >= H2_SPLIT_BYTES;
                let body_block_split_gate = split_eligible_file && bytes >= H2_SPLIT_BYTES;
                let usage_h2 = readme && is_canonical_usage_h2(*node, source);
                let features_h2 = readme && is_features_h2_title(*node, source);

                let bullet_items = structural_split_gate
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
                    for item in items {
                        let (item_start, item_end) = node_row_range(item, source);
                        out.push(SectionRange {
                            start: item_start,
                            end: item_end,
                            kind: SectionKind::BulletItem,
                            parent_index: parent_idx,
                            synthetic_intro_present,
                            parent_is_concept_h2: false,
                            parent_is_canonical_usage_h2: false,
                            parent_is_features_h2: false,
                        });
                    }
                    continue;
                }

                let h3s = if structural_split_gate {
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
                    let concept_h2 = readme && is_concept_h2_title(*node, source);
                    for h3 in &h3s {
                        push_h3_child_or_body_blocks(
                            &mut out,
                            *h3,
                            parent_idx,
                            synthetic_intro_present,
                            concept_h2,
                            source,
                        );
                    }
                } else {
                    let did_body_split = body_block_split_gate
                        && push_list_body_blocks(
                            &mut out,
                            *node,
                            parent_idx,
                            synthetic_intro_present,
                            source,
                        );
                    if !did_body_split {
                        out.push(SectionRange {
                            start: *start,
                            end: *end,
                            kind: SectionKind::Whole,
                            parent_index: parent_idx,
                            synthetic_intro_present,
                            parent_is_concept_h2: false,
                            parent_is_canonical_usage_h2: usage_h2,
                            parent_is_features_h2: features_h2,
                        });
                    }
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
        parent_is_concept_h2: false,
        parent_is_canonical_usage_h2: false,
        parent_is_features_h2: false,
    });
}

fn push_h3_child_or_body_blocks(
    out: &mut Vec<SectionRange>,
    h3_section: Node<'_>,
    parent_idx: usize,
    synthetic_intro_present: bool,
    parent_is_concept_h2: bool,
    source: &str,
) {
    let (h3_start, h3_end) = node_row_range(h3_section, source);
    if !has_substantive_body(h3_section, h3_start, h3_end, source) {
        return;
    }
    let h3_bytes = h3_section.end_byte() - h3_section.start_byte();
    if h3_bytes >= BODY_BLOCK_SPLIT_BYTES {
        // See `CONCEPT_H3_BODY_BLOCK_MAX_BYTES` — the boost only
        // propagates to BodyBlocks when the H3 is a tight definition.
        let body_block_concept =
            parent_is_concept_h2 && h3_bytes <= CONCEPT_H3_BODY_BLOCK_MAX_BYTES;
        let ranges = body_block_ranges(h3_section, source);
        if push_body_block_ranges(
            out,
            ranges,
            parent_idx,
            synthetic_intro_present,
            body_block_concept,
        ) {
            return;
        }
    }
    out.push(SectionRange {
        start: h3_start,
        end: h3_end,
        kind: SectionKind::H3Child,
        parent_index: parent_idx,
        synthetic_intro_present,
        parent_is_concept_h2,
        parent_is_canonical_usage_h2: false,
        parent_is_features_h2: false,
    });
}

fn push_body_block_ranges(
    out: &mut Vec<SectionRange>,
    ranges: Vec<(usize, usize)>,
    parent_idx: usize,
    synthetic_intro_present: bool,
    parent_is_concept_h2: bool,
) -> bool {
    if ranges.len() < 2 {
        return false;
    }
    out.extend(ranges.into_iter().map(|(start, end)| SectionRange {
        start,
        end,
        kind: SectionKind::BodyBlock,
        parent_index: parent_idx,
        synthetic_intro_present,
        parent_is_concept_h2,
        parent_is_canonical_usage_h2: false,
        parent_is_features_h2: false,
    }));
    true
}

fn push_list_body_blocks(
    out: &mut Vec<SectionRange>,
    section: Node<'_>,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) -> bool {
    let Some(ranges) = list_only_body_block_ranges(section, source) else {
        return false;
    };
    // List-only body-block split is the H2-direct fallback (no H3
    // children). The concept-H3 boost doesn't apply at this level.
    push_body_block_ranges(out, ranges, parent_idx, synthetic_intro_present, false)
}

fn list_only_body_block_ranges(section: Node<'_>, source: &str) -> Option<Vec<(usize, usize)>> {
    Some(substantive_item_ranges(
        single_list_with_decorative_siblings(section, source)?,
        source,
    ))
}

fn single_list_with_decorative_siblings<'a>(section: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cur = section.walk();
    let mut list: Option<Node<'a>> = None;
    for child in section.children(&mut cur) {
        match child.kind() {
            kind if is_section_scaffolding(kind) => continue,
            "list" => {
                if list.is_some() {
                    return None;
                }
                list = Some(child);
            }
            "paragraph" if is_decorative_paragraph(child, source) => continue,
            "html_block" if is_decorative_html_block(child, source) => continue,
            _ => return None,
        }
    }
    list
}

fn substantive_item_ranges(list: Node<'_>, source: &str) -> Vec<(usize, usize)> {
    top_level_list_items(list)
        .into_iter()
        .filter(|item| has_substantive_list_item(*item))
        .map(|item| node_row_range(item, source))
        .collect()
}

fn body_block_ranges(section: Node<'_>, source: &str) -> Vec<(usize, usize)> {
    let src_lines: Vec<&str> = source.lines().collect();
    let mut cur = section.walk();
    let children: Vec<Node> = section.children(&mut cur).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < children.len() {
        let child = children[i];
        match child.kind() {
            kind if is_section_scaffolding(kind) => {
                i += 1;
            }
            "paragraph" if is_decorative_paragraph(child, source) => {
                i += 1;
            }
            "html_block" if is_decorative_html_block(child, source) => {
                i += 1;
            }
            "paragraph"
                if children
                    .get(i + 1)
                    .is_some_and(|n| n.kind() == "list" || is_code_block(n.kind())) =>
            {
                let next = children[i + 1];
                let (start, _) = node_row_range(child, source);
                let (_, end) = node_row_range(next, source);
                out.push((start, end));
                i += 2;
            }
            "list" => {
                out.extend(substantive_item_ranges(child, source));
                i += 1;
            }
            _ => {
                if let Some(range) = nonblank_node_row_range(child, &src_lines, source) {
                    out.push(range);
                }
                i += 1;
            }
        }
    }
    out
}

fn node_row_range(node: Node, source: &str) -> (usize, usize) {
    (
        node.start_position().row + 1,
        node_end_row_trimmed(node, source) + 1,
    )
}

fn nonblank_node_row_range(node: Node, src_lines: &[&str], source: &str) -> Option<(usize, usize)> {
    let (start, end) = node_row_range(node, source);
    let mut rows = Vec::new();
    extend_nonblank_rows(&mut rows, src_lines, start - 1, end - 1);
    (!rows.is_empty()).then_some((start, end))
}

fn is_section_scaffolding(kind: &str) -> bool {
    matches!(
        kind,
        "atx_heading" | "setext_heading" | "block_continuation"
    )
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
    let items: Vec<Node<'a>> =
        top_level_list_items(single_list_with_decorative_siblings(h2_section, source)?)
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

fn is_code_block(kind: &str) -> bool {
    matches!(kind, "fenced_code_block" | "indented_code_block")
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

/// True iff the H2 section's heading text is an orientation-concept
/// marker. The H3 children of such an H2 are typically the canonical
/// concept definitions an NS author would anchor on (monaco-editor's
/// `## Concepts` → Models / URIs / Editors / Providers / Disposables)
/// rather than elaboration sub-sections of a longer topic. Used by
/// `section_value` to lift those H3 rows above the generic
/// `README_SUB_SECTION_SIGNAL_SCALE` so they compete with per-decl
/// surfaces in the early budget.
///
/// Matching is case-insensitive and on the heading's plain text only
/// (badges / decorative inlines are tolerated because the title is
/// matched as a prefix word). The set is intentionally narrow:
/// "Getting Started" / "Installation" H3s tend to be procedural steps,
/// not concepts, so they are excluded.
fn is_concept_h2_title(h2_section: Node<'_>, source: &str) -> bool {
    let Some(core) = h2_title_core(h2_section, source) else {
        return false;
    };
    matches!(
        core.as_str(),
        "concepts"
            | "core concepts"
            | "key concepts"
            | "architecture"
            | "overview"
            | "fundamentals"
            | "primitives"
            | "building blocks"
            | "glossary"
            | "terminology"
            | "theory of operation"
    )
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
    let Some(core) = h2_title_core(h2_section, source) else {
        return false;
    };
    matches!(
        core.as_str(),
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

/// README H2 sections worth a features-list boost: title is one of
/// the features-list markers. Used by `section_value` via
/// `features_section_factor`. Matching is the same shape as
/// `is_concept_h2_title` / `is_canonical_usage_h2_title` (lowercased
/// alphanumeric prefix of the heading's inline text).
fn is_features_h2_title(h2_section: Node<'_>, source: &str) -> bool {
    let Some(core) = h2_title_core(h2_section, source) else {
        return false;
    };
    matches!(
        core.as_str(),
        "features" | "key features" | "feature highlights" | "highlights"
    )
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

fn section_is_code_dominant(section: Node<'_>, _source: &str) -> bool {
    let Some(heading) = first_heading_child(section) else {
        return false;
    };
    let heading_bytes = heading.end_byte() - heading.start_byte();
    let body_bytes = (section.end_byte() - section.start_byte()).saturating_sub(heading_bytes);
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

/// Plain-text core of an H2's title (lowercased, alphanumeric +
/// whitespace prefix only). Returns `None` when no `inline` child is
/// found. Shared by [`is_concept_h2_title`] and
/// [`is_canonical_usage_h2_title`].
fn h2_title_core(h2_section: Node<'_>, source: &str) -> Option<String> {
    let heading = first_heading_child(h2_section)?;
    let inline = first_child_of_kind(heading, "inline")?;
    let text = source[inline.start_byte()..inline.end_byte()]
        .trim()
        .to_ascii_lowercase();
    let core: String = text
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect();
    Some(core.trim().to_string())
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

/// Collect prelude lede rows for [`headline_spec`]: walk the `section`
/// children of the root that appear *before* `first_headed` (i.e. the
/// heading-less prelude tree-sitter-md wraps when the README opens with
/// HTML title blocks or badges), skip leading decorative paragraphs /
/// image-only HTML, then include blocks until the first substantive
/// paragraph (inclusive) or end of prelude. Mirrors the
/// post-heading-skip-then-include walk inside `headline_spec`.
fn extend_prelude_lede(
    covered: &mut BTreeSet<usize>,
    root: Node<'_>,
    first_headed: Node<'_>,
    source: &str,
) {
    let mut cursor = root.walk();
    let prelude_blocks: Vec<Node> = root
        .children(&mut cursor)
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
        .collect();

    let mut i = 0;
    while i < prelude_blocks.len() {
        let block = prelude_blocks[i];
        match block.kind() {
            "paragraph"
                if is_decorative_paragraph(block, source)
                    || is_nav_link_paragraph(block, source)
                    || is_admin_emoji_paragraph(block, source) => {}
            "html_block"
                if is_decorative_html_block(block, source)
                    || is_nav_link_html_block(block, source) => {}
            "block_quote" if is_admin_block_quote(block, source) => {}
            _ => break,
        }
        i += 1;
    }
    // Include the first substantive prelude block. If it's a short
    // tagline (ts-pattern's `<h1>TS-Pattern</h1>`, microbootstrap's
    // `<b>name</b> assists you...`), also include the next non-
    // decorative block — the actual prose lede or the canonical code
    // example. Decoratives are skipped between the two without being
    // included. Larger preludes stop at one block to keep batch size
    // bounded.
    if let Some(block) = prelude_blocks.get(i) {
        extend_rows_inclusive(covered, *block, source);
        if is_short_substantive_block(*block, source) {
            let mut j = i + 1;
            while j < prelude_blocks.len() {
                let next = prelude_blocks[j];
                match next.kind() {
                    "paragraph"
                        if is_decorative_paragraph(next, source)
                            || is_nav_link_paragraph(next, source)
                            || is_admin_emoji_paragraph(next, source) => {}
                    "html_block"
                        if is_decorative_html_block(next, source)
                            || is_nav_link_html_block(next, source) => {}
                    "block_quote" if is_admin_block_quote(next, source) => {}
                    _ => break,
                }
                j += 1;
            }
            if let Some(extra) = prelude_blocks.get(j) {
                extend_rows_inclusive(covered, *extra, source);
            }
        }
    }
}

/// True when a paragraph opens with a callout-style emoji
/// (⚠️ / 🚨 / ⛔ / ❗) — the prose-equivalent of a
/// `> [!WARNING]` block_quote. NS authors anchor on the project
/// lede, not on a deprecation / migration / security notice typeset
/// as a plain paragraph; treating it as admin lets the
/// prelude-skip phase keep looking for the substantive lede when a
/// project uses emoji callouts instead of GitHub-flavored block
/// quotes.
fn is_admin_emoji_paragraph(para: Node, source: &str) -> bool {
    let raw = source[para.start_byte()..para.end_byte()].trim_start();
    starts_with_admin_emoji(raw)
}

fn starts_with_admin_emoji(s: &str) -> bool {
    // ⚠ U+26A0 (with or without U+FE0F variation selector), 🚨 U+1F6A8,
    // ⛔ U+26D4, ❗ U+2757. Match the codepoint, not the byte sequence,
    // so the variation selector (`⚠️` = U+26A0 U+FE0F) and the bare
    // form both classify the same.
    let mut chars = s.chars();
    matches!(
        chars.next(),
        Some('\u{26A0}' | '\u{1F6A8}' | '\u{26D4}' | '\u{2757}')
    )
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
    node_end_row_trimmed(node, source)
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

    /// py3xui shape: prelude HTML wrapper, ⚠️-led admin notice
    /// paragraph (a plain-paragraph callout instead of `> [!WARNING]`),
    /// then the real lede. The admin paragraph must be skipped so the
    /// substantive lede gets surfaced.
    #[test]
    fn markdown_admin_emoji_paragraph_skipped_py3xui_shape() {
        let src = "\u{26A0}\u{FE0F} The secret token feature was removed in v2.6.0. \u{26A0}\u{FE0F}\n\
                   \n\
                   Sync and Async Object-oriented Python SDK for the 3x-ui API.\n\
                   \n\
                   ## Overview\n";
        let rows = covered(src);
        assert!(
            rows.contains(&3),
            "lede row 3 missing — admin emoji paragraph should have been skipped"
        );
        assert!(
            !rows.contains(&1),
            "admin emoji paragraph row 1 should be skipped"
        );
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

    /// Multi-language nav paragraph (`[English](url) • ...`) is
    /// decorative — the headline must not burn its prelude on it.
    #[test]
    fn markdown_nav_link_paragraph_is_decorative() {
        let src = "[English](https://e.x/a) • [中文](https://e.x/b) • [Fr](https://e.x/c)\n\
                   \n\
                   # Project\n\
                   \n\
                   Actual lede paragraph.\n";
        let rows = covered(src);
        assert!(rows.contains(&3), "H1 row must be present");
        assert!(rows.contains(&5), "actual lede must be present");
        assert!(
            !rows.contains(&1),
            "nav-link prelude paragraph must be skipped as decorative"
        );
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

    /// Outline gated out by row-count cap → no structural H3/body split.
    /// Without the outline carrying the H2 heading, dropping a
    /// heading-only intro would lose it.
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
            "outline-omitted files must keep non-list H2s as Whole; got {usage:?}",
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

    /// Short bullets (each well under `BULLET_LARGE_ITEM_BYTES`) miss the
    /// specialized bullet split, but a long list still falls through to the
    /// generic body-block split.
    #[test]
    fn markdown_h2_body_block_split_short_bullets() {
        let prefix = "# Title\n\nTagline.\n\n## Features";
        // Make the section large enough overall that the byte gate
        // can't single-handedly suppress; the specialized bullet-item
        // gate should reject, then generic body blocks should recover
        // one range per top-level item.
        let mut src = String::from(prefix);
        for i in 0..40 {
            src.push_str(&format!("\n- short item {i}\n"));
        }
        src.push_str("\n## Next\n\nbody.\n");
        let ranges = sections("README.md", &src);
        let body_blocks = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::BodyBlock)
            .count();
        assert_eq!(
            body_blocks, 40,
            "long short-bullet list must split into body blocks; got {ranges:?}"
        );
    }

    /// H3 split is not always fine-grained enough: a long H3 child with
    /// several body blocks should refine to `BodyBlock` ranges while a
    /// sibling H3 with one body block remains an `H3Child`.
    #[test]
    fn markdown_h3_child_body_block_split() {
        let filler = "Additional prose keeps this sub-section large enough for \
                      body-block splitting while still representing ordinary \
                      markdown documentation text.\n"
            .repeat(3);
        let src = "# Title\n\nTagline.\n\n## Parts\n\n### One\n\n\
                   Intro paragraph before the example.\n"
            .to_owned()
            + &filler
            + "\n\
                   ```tsx\nconst one = 1\n```\n\n\
                   Follow-up paragraph with details.\n"
            + &filler
            + "\n\
                   ### Two\n\n\
                   Single compact paragraph.\n";
        assert!(
            src.len() >= H2_SPLIT_BYTES,
            "test source must clear H2 split gate"
        );
        let ranges = sections("README.md", &src);
        let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
        assert!(
            kinds.contains(&SectionKind::BodyBlock),
            "long H3 child must split into body blocks; got {ranges:?}"
        );
        assert!(
            kinds.contains(&SectionKind::H3Child),
            "single-block H3 child must remain whole; got {ranges:?}"
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

    /// Outline-omitted file doesn't use the specialized `BulletItem`
    /// split, but the generic body-block fallback can still emit the
    /// substantive list items.
    #[test]
    fn markdown_h2_body_block_bullets_when_outline_omitted() {
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
        assert!(
            ranges.iter().any(|r| r.kind == SectionKind::BodyBlock),
            "outline-omitted long list should fall back to body blocks; got {ranges:?}",
        );
        assert!(
            !ranges.iter().any(|r| r.kind == SectionKind::BulletItem),
            "outline-omitted file must not use BulletItem split; got {ranges:?}"
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
            readme_index_decay(install, 2),
            1.0,
            "first real H2 must be unscaled (readme h2_idx = 0)"
        );

        let use_ = &ranges[2];
        assert_eq!(use_.parent_index, 2);
        let f = readme_index_decay(use_, 2);
        assert!(
            (0.7..1.0).contains(&f),
            "second real H2 should decay; got {f}"
        );
    }

    /// READMEs without an H1 wrap (no synthetic intro) — first H2 is
    /// `parent_index = 0` and gets factor 1.0 directly.
    #[test]
    fn markdown_readme_index_decay_no_synthetic_intro() {
        let src = "## Install\n\nbody\n\n## Use\n\nbody\n";
        let ranges = sections("README.md", src);
        assert_eq!(ranges.len(), 2);
        assert!(!ranges[0].synthetic_intro_present);
        assert_eq!(readme_index_decay(&ranges[0], 2), 1.0);
    }

    /// AGENTS.md / CLAUDE.md / skill bodies are already loaded into
    /// the model's context by the harness, so emitting their per-H2
    /// `Section` batches is pure waste. The walker must skip
    /// `MarkdownKey::Section` and `MarkdownKey::SummaryWhole` for
    /// these files while still emitting structural batches
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
                    MarkdownKey::SummaryWhole { file } if file == &agents_md => Some("summary"),
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
        assert!(
            !agents_keys.contains(&"summary"),
            "AGENTS.md must not emit SummaryWhole; got {agents_keys:?}",
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
