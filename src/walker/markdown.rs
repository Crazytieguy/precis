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
//!   subdivides via one of two rules when applicable. *H3 split*
//!   (content-heavy H2 with ≥2 H3 children) becomes one `Intro`
//!   (when its body is substantive) plus one `H3Child` per H3.
//!   *Body-block split* refines large H3 children into direct
//!   paragraph/code/list blocks, and can split list-only H2 sections
//!   into one body block per item (anyhow's `## Details` shape).
//!   All per-child kinds carry a global signal scale
//!   (`SUB_SECTION_SIGNAL_SCALE` / `BODY_BLOCK_SIGNAL_SCALE`) to keep
//!   them from over-ranking once the marginal cost drops to
//!   per-sub-section size. Root-README sections neither rule catches
//!   that still exceed `OVERSIZE_SECTION_SPLIT_TOKENS` get the
//!   *oversize head-split*: a head chunk (kind `Whole`, keeps the
//!   section's value flags, includes the heading) plus
//!   predecessor-chained `OversizeTail` chunks (valued at
//!   `OVERSIZE_TAIL_FACTOR`), cut at blank-line boundaries outside
//!   code fences. Changelog-class files (CHANGELOG
//!   / CONTRIBUTING / CHANGES) are gated out of splitting. H3/prose
//!   body-block splitting also requires `HeadingsOutline` so heading
//!   context is preserved; list-only H2 splits are allowed without an
//!   outline because each item is self-contained.
//!   Predecessor (when emitted): outline → headline → none, picking
//!   the deepest available so all heading-row overlaps are
//!   ancestor-overlaps. Enabling either split rule shifts
//!   `section_index` numbering relative to a non-split version of
//!   the same file; snapshots / divergence reports reflect the
//!   post-split indexing. For `README.rst` (line-scanned, no
//!   tree-sitter) `Section`s are the body sections after the title,
//!   one per setext/overline heading — see [`rst_body_sections`].

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, MarkdownKey};
use crate::content::{BatchContent, Render, Span};
use crate::tokenizer;
use crate::value::{is_orientation_doc, mix_signals, roster_mass_factor};

use super::{
    FileLines, WalkCtx, budget_chunk_ranges, extend_nonblank_rows, first_child_of_kind,
    fs::files_with_extension, node_end_row_trimmed, path_depth_factor, single_file_lines_content,
    whole_file_lines_content,
};

/// Upper bound on collectable heading rows before `HeadingsOutline`
/// suppresses itself — the outline predecesses every section, so an
/// oversize outline would block the whole file.
const MAX_OUTLINE_HEADINGS: usize = 30;

/// Source-byte cap on the outline's heading content (~400 tokens).
const MAX_OUTLINE_HEADING_BYTES: usize = 1500;

/// Minimum H2 source bytes to split into H3/bullet sub-sections.
const H2_SPLIT_BYTES: usize = 600;

/// Multiplier on the three value signals for `H3Child` ranges —
/// compensates for smaller marginal cost.
const SUB_SECTION_SIGNAL_SCALE: f64 = 0.45;

/// Multiplier for `BodyBlock` (paragraph / list item) ranges.
const BODY_BLOCK_SIGNAL_SCALE: f64 = 0.60;

/// Minimum source bytes before a section is split into body blocks.
/// Lower than `H2_SPLIT_BYTES` since it can apply after H2 splitting.
const BODY_BLOCK_SPLIT_BYTES: usize = 350;

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

/// Tail-chunk value factor relative to the parent section. Above the
/// generic `BODY_BLOCK_SIGNAL_SCALE`: a tail is the direct
/// continuation of content whose head just won purchase, and the NS
/// ranks the continuation right behind it — pricing tails as fan-out
/// noise strands them past the window their head opened. (Tails still
/// take the steeper index-≥1 `Section` concavity, and as orientation
/// batches they are exempt from train breadth pressure.)
const OVERSIZE_TAIL_FACTOR: f64 = 0.85;

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

    // RST README: emit a ReadmeHeadline batch (title + first
    // substantive paragraph) plus one `Section` batch per body section
    // — parity with the .md path, which subdivides the README body. No
    // tree-sitter parse — line-scan headings via `is_rst_underline`,
    // dropping `.. directive::` blocks. Skip nested README.rst — the
    // root-level file is the only anchor.
    for file in super::fs::files_with_extension(dir, "rst", ctx) {
        if !is_readme_rst(&file) || dir != ctx.root() {
            continue;
        }
        let Some(source) = ctx.read_source(&file) else {
            continue;
        };
        let mut headline_emitted: Option<BatchKey> = None;
        if let Some(content) = build_rst_readme_content(&file, &source) {
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
        for (idx, section) in rst_body_sections(&source).into_iter().enumerate() {
            let value = rst_section_value(&file, &section, ctx);
            let reference_shaped = section.reference_shaped;
            if let Some(content) =
                single_file_lines_content(&file, &source, FileLines::new(section.rows))
            {
                out.push(Batch {
                    key: MarkdownKey::Section {
                        file: file.clone(),
                        section_index: idx,
                        keeps_default_concavity: reference_shaped,
                    }
                    .into(),
                    predecessor: headline_emitted.clone(),
                    content,
                    value,
                });
            }
        }
    }

    if md_files.is_empty() {
        return out;
    }
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
        let gates = derive_outline_gates(&file, &tree, &source);
        let (headline, outline_rows, mut outline_emits) = (gates.headline, gates.rows, gates.emits);
        let root_readme = is_readme(&file) && dir == ctx.root();
        let nav = NavDensity {
            sibling_md_count,
            root_readme,
        };
        let mut ranges = logical_sections(
            &file,
            &tree,
            &source,
            &outlined_rows(outline_emits, &outline_rows),
            gates.truncated,
            root_readme,
        );
        // The mega-doc fallback (truncated H2-skeleton outline + the
        // structural splits it unlocks) earns its early-budget cost
        // only on the reference-README shape, where it carries the
        // roster index. On a merely long doc it floods the window with
        // a skeleton + micro-H3 stubs the NS doesn't rank — keep the
        // pre-existing suppression there.
        if gates.truncated && !ranges.iter().any(|r| r.roster_entries > 0) {
            outline_emits = false;
            ranges = logical_sections(&file, &tree, &source, &BTreeSet::new(), false, root_readme);
        }
        if ranges.is_empty() && !markdown_has_heading(&tree) {
            ranges = headingless_fallback_ranges(&file, &source);
        }
        if ranges.is_empty() {
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

        let src_lines: Vec<&str> = source.lines().collect();
        let mut prev_section_key: Option<BatchKey> = None;
        for (idx, range) in ranges.iter().enumerate() {
            if let Some(content) =
                build_section_content(&file, &source, idx, range, headline.as_ref())
            {
                let key = MarkdownKey::Section {
                    file: file.clone(),
                    section_index: idx,
                    keeps_default_concavity: range.reference_shaped
                        || range.kind == SectionKind::LedeBody,
                };
                // Roster chunks deliver in source order: each chunk
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
                    value: section_value(&file, range, &src_lines, root_readme, ctx),
                });
            }
        }
    }
    out
}

/// Essential source mass below which the root README stops reading as
/// peripheral prose beside the code and becomes the repository's primary
/// orientation: there is not enough source to learn the project from, so
/// the document that explains it competes with the source rather than
/// behind it. Doubling this admits addressable-reference READMEs whose
/// valuable sections are late-index and measured worse.
const THIN_SOURCE_BYTES: u64 = 24_000;

/// Section size, in tokens, that prices at par under
/// [`thin_source_orientation_factor`]. At 400 the par point lands above
/// a whole README's sections and the shape becomes a flat damp.
const THIN_SOURCE_PAR_TOKENS: f64 = 200.0;

/// Exponent on the size ratio. Positive: in a thin-source repository the
/// substantive sections are the orientation and the heading-sized stubs
/// between them are not, which is the split an NS-aware oracle makes on
/// these READMEs. Measured flat against 1.0 at the primary budget; the
/// milder shape is kept.
const THIN_SOURCE_MASS_EXPONENT: f64 = 0.5;

/// Size-shaped premium for root-README sections of a thin-source
/// repository. Sections at [`THIN_SOURCE_PAR_TOKENS`] are unchanged;
/// larger sections rise and stub sections fall.
fn thin_source_orientation_factor(
    range: &SectionRange,
    src_lines: &[&str],
    root_readme: bool,
    ctx: &WalkCtx,
) -> f64 {
    if !root_readme || ctx.essential_source_bytes() >= THIN_SOURCE_BYTES {
        return 1.0;
    }
    let tokens: usize = (range.start..=range.end)
        .map(|row| row_tokens(src_lines, row))
        .sum();
    (tokens as f64 / THIN_SOURCE_PAR_TOKENS).powf(THIN_SOURCE_MASS_EXPONENT)
}

/// True if the outline batch should be emitted — bounded by both row
/// count and total source bytes.
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

/// A whole-file table of contents is a navigation index whose entries
/// are the filenames the parent directory listing already emitted, so it
/// prices as the listing it duplicates rather than as an orientation
/// doc. Measured alone this is worth +0.0009 at the 3K mean on the one
/// corpus carrier; it is fully absorbed once
/// [`NavDensity::factor`] frees the same window (see the lane note in
/// `git show a90ee9b6:docs/design-notes.md`).
fn summary_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let (cat, fu, ztu) = super::fs::PLAIN_LISTING_SIGNALS;
    mix_signals(cat, fu, ztu, path_depth_factor(file, ctx))
}

fn readme_headline_value(file: &Path, ctx: &WalkCtx, nav: NavDensity) -> f64 {
    mix_signals(0.9, 0.6, 0.8, path_depth_factor(file, ctx)) * nav.factor(file)
}

/// `Prelude` prices as the README's index-0 section: it is the top of
/// the README body, just above the first heading rather than below it.
fn prelude_value(file: &Path, ctx: &WalkCtx, nav: NavDensity) -> f64 {
    mix_signals(0.55, 0.8, 0.7, path_depth_factor(file, ctx))
        * PRELUDE_VALUE_FACTOR
        * nav.factor(file)
}

/// Premium over the index-0 [`readme_section_value`] — same signal mix,
/// no index decay, times this. Swept: per-fixture rows are identical
/// everywhere over [1.3, 1.6], and at 1.0 cobra's 211-token prelude
/// prices out of the 3K frontier for −0.0004. Shipped at the low end of
/// the measured-flat plateau; there is no argument for the exact value
/// beyond that.
const PRELUDE_VALUE_FACTOR: f64 = 1.3;

fn headings_outline_value(file: &Path, ctx: &WalkCtx, nav: NavDensity) -> f64 {
    mix_signals(
        0.7,
        0.55,
        0.4,
        super::file_depth_factor(file, ctx, is_orientation_doc(file)),
    ) * nav.factor(file)
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
    /// The root README and orientation docs are exempt. The root README's
    /// exemption is load-bearing rather than cosmetic: its outline is the
    /// hard predecessor of every root README section, so damping it
    /// delays the whole README body, which is the largest credited
    /// early-budget purchase on repos that have one. A README deeper in
    /// the tree carries no such stream and is just one more page in a
    /// docs directory the listing already enumerated.
    fn factor(self, file: &Path) -> f64 {
        if self.root_readme || is_orientation_doc(file) {
            return 1.0;
        }
        if self.sibling_md_count <= DENSE_MD_SIBLINGS {
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

fn readme_section_value(file: &Path, range: &SectionRange, ctx: &WalkCtx) -> f64 {
    mix_signals(0.55, 0.8, 0.7, path_depth_factor(file, ctx))
        * readme_index_decay(range)
        * canonical_usage_section_factor(range)
}

/// Boost for README H2 `Whole` sections whose title is a canonical-
/// usage marker (see [`is_canonical_usage_h2_title`]) — the demo
/// fence inside is the highest-value follow-up to the headline.
const CANONICAL_USAGE_SECTION_FACTOR: f64 = 2.2;

/// Structural operational sections are useful orientation, but less reliably
/// the single canonical demo than an exact usage-title/code-dominant match.
const STRUCTURAL_OPERATIONAL_SECTION_FACTOR: f64 = 2.1;

/// Operational orientation must remain a single early-budget purchase. Larger
/// reference sections can be valid, but promoting them displaces source/API
/// surfaces before their README material is useful.
const STRUCTURAL_OPERATIONAL_MAX_TOKENS: usize = 240;

/// Modest parallel boost for README reference/usage sections whose
/// title matches the broader vocabulary (see
/// [`is_reference_usage_title`]) — applies REGARDLESS of code fraction,
/// so prose/list/table reference sections (`## Options`, `### Colors`,
/// `## Environment Variables`) clear the early budget instead of
/// sinking below the README index decay. Smaller than the code-dominant
/// canonical factor since these sections are less reliably the single
/// highest-value follow-up.
const REFERENCE_USAGE_SECTION_FACTOR: f64 = 1.3;

/// Combined README section boost: the strongest applicable factor among the
/// code-dominant canonical demo, the compact structural operational class,
/// and the modest reference/usage title class. Factors never stack.
fn canonical_usage_section_factor(range: &SectionRange) -> f64 {
    // `OversizeTail` is the continuation of a boosted `Whole` head —
    // it inherits the boost so the tail prices at head * tail factor.
    let canonical = if range.parent_is_canonical_usage_h2
        && matches!(
            range.kind,
            SectionKind::Whole | SectionKind::OversizeTail | SectionKind::LedeBody
        ) {
        CANONICAL_USAGE_SECTION_FACTOR
    } else {
        1.0
    };
    let operational = if range.is_canonical_operational_section {
        STRUCTURAL_OPERATIONAL_SECTION_FACTOR
    } else {
        1.0
    };
    let reference = if range.is_reference_usage_section {
        REFERENCE_USAGE_SECTION_FACTOR
    } else {
        1.0
    };
    canonical.max(reference).max(operational)
}

/// Index decay for README sections. (An adaptive steeper falloff for
/// long READMEs (≥18 H2s) was tuned on the pre-refreeze keys and
/// measured obsolete on the frozen ones — un-shipped 2026-07-06.)
fn readme_index_decay(range: &SectionRange) -> f64 {
    let h2_idx = if range.synthetic_intro_present {
        range.parent_index.saturating_sub(1)
    } else {
        range.parent_index
    };
    index_decay(h2_idx, 0.15, 0.7)
}

/// `(idx + 1)^-exp`, floored at `floor` — shared decay shape.
fn index_decay(idx: usize, exp: f64, floor: f64) -> f64 {
    ((idx as f64 + 1.0).powf(-exp)).max(floor)
}

fn heading_slab_value(file: &Path, parent_index: usize, ctx: &WalkCtx) -> f64 {
    let is_guide = is_changelog_class(file);
    let is_orientation = is_orientation_doc(file);
    // Root-level (depth 1) orientation already wins; only nested
    // orientation gets the cat bump.
    let is_nested_orientation = is_orientation && ctx.depth_from_root(file) > 1;
    // Changelog index decay (newest-first) — floor lets deep sections
    // still fire when budget permits.
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

fn dev_workflow_section_value(file: &Path, parent_index: usize, ctx: &WalkCtx) -> f64 {
    mix_signals(1.15, 1.0, 0.85, path_depth_factor(file, ctx))
        * index_decay(parent_index, 0.2, 0.75)
}

/// Per-section value. Child ranges scale the parent's value so they
/// don't over-rank once cost drops. `Intro` keeps full weight.
fn section_value(
    file: &Path,
    range: &SectionRange,
    src_lines: &[&str],
    root_readme: bool,
    ctx: &WalkCtx,
) -> f64 {
    section_signal_value(file, range, ctx)
        * thin_source_orientation_factor(range, src_lines, root_readme, ctx)
}

fn section_signal_value(file: &Path, range: &SectionRange, ctx: &WalkCtx) -> f64 {
    // Link-index roster chunks are API rosters, not prose — the NS
    // calls lo's helper index "the highest does-X-exist yield per
    // token in the repo". Names-surface tier cat with the roster-mass
    // ratio neutralizer, under the same README index decay.
    if range.roster_entries > 0 {
        return mix_signals(0.65, 0.8, 0.7, path_depth_factor(file, ctx))
            * readme_index_decay(range)
            * roster_mass_factor(range.roster_entries);
    }
    let parent = if range.dev_workflow_section {
        dev_workflow_section_value(file, range.parent_index, ctx)
    } else if is_readme(file) {
        readme_section_value(file, range, ctx)
    } else {
        heading_slab_value(file, range.parent_index, ctx)
    };
    let sub_scale = if is_readme(file) {
        // A root-README reference-vocabulary H3 (`### Colors`,
        // `### Modifiers`) is a top-rank catalog row in its own right,
        // not H3 fan-out noise — it skips the sub-section scale.
        if range.is_reference_usage_section && ctx.depth_from_root(file) <= 1 {
            1.0
        } else {
            README_SUB_SECTION_SIGNAL_SCALE
        }
    } else {
        SUB_SECTION_SIGNAL_SCALE
    };
    match range.kind {
        SectionKind::Whole | SectionKind::Intro => parent,
        SectionKind::H3Child => parent * sub_scale,
        SectionKind::BodyBlock => parent * BODY_BLOCK_SIGNAL_SCALE,
        SectionKind::OversizeTail => parent * OVERSIZE_TAIL_FACTOR,
        SectionKind::LedeBody => parent,
    }
}

/// README H3 children are often canonical concept rows in their own
/// right; bump above the generic `SUB_SECTION_SIGNAL_SCALE`.
const README_SUB_SECTION_SIGNAL_SCALE: f64 = 0.55;

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

/// Parse `text` with the inline grammar — surfaces named `image` /
/// `inline_link` / `html_tag` children that the block grammar leaves
/// as opaque bytes.
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
    whole_file_lines_content(file, &source)
}

fn build_headline_content(file: &Path, source: &str, spec: &HeadlineSpec) -> Option<BatchContent> {
    let spans = build_headline_spans(file, source, spec);
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
        // `end + 1` is either a row of this heading's own section (a
        // later `Section` batch gates on the outline, so its `full` is a
        // descendant upgrade) or the next heading's row, which the
        // outline renders itself and so drops the marker. Fragile seam:
        // the outline is a DESCENDANT of `ReadmeHeadline`, and a
        // descendant `…` on an ancestor-rendered row silently demotes
        // the paid row. Unreachable only because the decorative-title
        // fallback ([`headline_spec`]) covers a contiguous run from the
        // H1, and every heading it covers is dropped from `rows`.
        // Widening that fallback past the first subsection arms this.
        ellipses.push(end + 1);
    }
    single_file_lines_content(file, source, FileLines::new(full).with_ellipses(ellipses))
}

/// Rows `HeadingsOutline` will render — empty when it is suppressed.
/// The split paths read this to decide whether a heading row they would
/// drop already has an owner.
fn outlined_rows(outline_emits: bool, rows: &[(usize, usize)]) -> BTreeSet<usize> {
    if !outline_emits {
        return BTreeSet::new();
    }
    rows.iter().flat_map(|(s, e)| *s..=*e).collect()
}

/// Per-file derivation chain shared by `expand_in_dir` and the unit-test
/// helpers, so the one question every split path asks — "which heading
/// rows does the outline own?" — is answered in exactly one place:
/// headline spec (READMEs only) → outline rows → outline gate.
/// Outline gates plus the mega-doc flag: `truncated` is true when the
/// file's full H1-H3 heading set blew [`MAX_OUTLINE_HEADINGS`] and the
/// emitted rows (if any) are the H1/H2 skeleton instead — i.e. the
/// outline does NOT name the file's H3 set, so an intra-doc link index
/// is not a duplicate of it.
struct OutlineGates {
    headline: Option<HeadlineSpec>,
    rows: Vec<(usize, usize)>,
    emits: bool,
    truncated: bool,
}

fn derive_outline_gates(file: &Path, tree: &Tree, source: &str) -> OutlineGates {
    let headline = is_readme(file)
        .then(|| headline_spec(tree, source))
        .flatten();
    let outline_rows = collectable_outline_rows(tree, source, headline.as_ref(), 3);
    let outline_emits = outline_emits_for(&outline_rows, source);
    if outline_emits || outline_rows.len() <= MAX_OUTLINE_HEADINGS {
        // Emitting, or suppressed on bytes alone — no mega-doc fallback.
        return OutlineGates {
            headline,
            rows: outline_rows,
            emits: outline_emits,
            truncated: false,
        };
    }
    // Mega-doc: too many headings to name them all. Fall back to the
    // H1/H2 skeleton (prefix-truncated to the same caps) so the file
    // keeps an outline — and the structural splits it gates — instead
    // of emitting one giant un-splittable Whole section (lo's 112KB
    // `## Spec`).
    let h2_rows = truncate_outline_rows(
        collectable_outline_rows(tree, source, headline.as_ref(), 2),
        source,
    );
    let emits = outline_emits_for(&h2_rows, source);
    OutlineGates {
        headline,
        rows: h2_rows,
        emits,
        truncated: true,
    }
}

/// Prefix of `rows` within both outline caps (row count + total bytes).
fn truncate_outline_rows(rows: Vec<(usize, usize)>, source: &str) -> Vec<(usize, usize)> {
    let src_lines: Vec<&str> = source.lines().collect();
    let mut bytes = 0usize;
    let mut out = Vec::new();
    for (s, e) in rows {
        bytes += (s..=e)
            .filter_map(|r| src_lines.get(r - 1))
            .map(|l| l.len())
            .sum::<usize>();
        if out.len() >= MAX_OUTLINE_HEADINGS || bytes > MAX_OUTLINE_HEADING_BYTES {
            break;
        }
        out.push((s, e));
    }
    out
}

/// Heading row ranges for `HeadingsOutline` — levels `1..=max_level`
/// (3 normally; 2 for the mega-doc skeleton fallback), with any
/// headline-covered headings dropped (`headline` is `Some` only for
/// READMEs; the caller derives it once per file).
fn collectable_outline_rows(
    tree: &Tree,
    source: &str,
    headline: Option<&HeadlineSpec>,
    max_level: usize,
) -> Vec<(usize, usize)> {
    let headline_covered: BTreeSet<usize> = headline
        .map(|s| s.covered_rows.iter().copied().collect())
        .unwrap_or_default();
    let mut nodes = Vec::new();
    collect_heading_nodes(tree.root_node(), &mut nodes);
    let mut out = Vec::new();
    for node in nodes {
        let level = heading_level(node);
        if !(1..=max_level).contains(&level) {
            continue;
        }
        let start_row = node.start_position().row + 1;
        let end_row = node_end_row_trimmed(node, source) + 1;
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

fn markdown_has_heading(tree: &Tree) -> bool {
    let mut nodes = Vec::new();
    collect_heading_nodes(tree.root_node(), &mut nodes);
    !nodes.is_empty()
}

fn headingless_fallback_ranges(file: &Path, source: &str) -> Vec<SectionRange> {
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
        SectionRange {
            start: start + 1,
            end: end + 1,
            // A headingless README is still the project's orientation
            // lede. Other headingless Markdown (licenses, generated
            // fragments, footer snippets) remains reachable without
            // competing at the same value as a named section.
            kind: if is_readme(file) {
                SectionKind::Whole
            } else {
                SectionKind::BodyBlock
            },
            parent_index: 0,
            synthetic_intro_present: false,
            parent_is_canonical_usage_h2: false,
            is_canonical_operational_section: false,
            is_reference_usage_section: false,
            reference_shaped: false,
            dev_workflow_section: false,
            roster_entries: 0,
            chained_to_previous: false,
        },
        true,
    );
    out
}

fn build_section_content(
    file: &Path,
    source: &str,
    section_index: usize,
    range: &SectionRange,
    headline: Option<&HeadlineSpec>,
) -> Option<BatchContent> {
    let (start, end) = (range.start, range.end);

    // For README section 0, skip lines `ReadmeHeadline` covers — else
    // their marginal cost goes to 0 and `ratio(value, 0) = ∞`.
    // (`headline` is `Some` only for READMEs.) Rows the headline
    // stepped *over* are dropped with them: admitting that chrome
    // measured −0.0033 corpus mean.
    let effective_start = if section_index == 0
        && let Some(max_row) = headline.and_then(|spec| spec.covered_rows.iter().next_back())
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

/// Root `README.rst` is the one `.rst` this walker owns; the plaintext
/// fallback must skip it so the two never emit overlapping spans.
pub(crate) fn is_readme_rst(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("README.rst"))
}

/// RST `ReadmeHeadline` content — title + first substantive paragraph,
/// stopping at the next setext heading. Skips `.. directive::` blocks
/// and their indented continuations.
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
        if i + 2 < src_lines.len()
            && is_rst_underline(src_lines[i], 1)
            && is_rst_underline(src_lines[i + 2], 1)
            && src_lines[i].trim() == src_lines[i + 2].trim()
            && !src_lines[i + 1].trim().is_empty()
        {
            if seen_title {
                break;
            }
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
    // If only the title + badges/decoratives were captured (no lede prose —
    // common when an admin section like `Sponsors` sits directly under the
    // title), descend to the first intro/overview-class section and capture
    // its first prose paragraph as the "what is this" lede.
    let has_prose = keep
        .iter()
        .any(|&row| src_lines.get(row - 1).is_some_and(|l| is_rst_prose_line(l)));
    if !has_prose {
        for row in rst_intro_section_prose_rows(&src_lines) {
            keep.push(row);
        }
        keep.sort_unstable();
        keep.dedup();
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

/// 1-based rows of the first prose paragraph under the first
/// intro/overview-class RST section (`Overview` / `Introduction` /
/// `About`), skipping its leading directives/figures. Empty if none.
fn rst_intro_section_prose_rows(src_lines: &[&str]) -> Vec<usize> {
    let headings = scan_rst_headings(src_lines);
    for h in headings.iter().skip(1) {
        let title = src_lines
            .get(h.title_row - 1)
            .map(|l| l.trim().to_ascii_lowercase())
            .unwrap_or_default();
        if !matches!(
            title.as_str(),
            "overview" | "introduction" | "about" | "summary"
        ) {
            continue;
        }
        let mut out = Vec::new();
        let mut idx = h.underline_row; // 0-based index of the line after the underline
        while idx < src_lines.len() {
            let line = src_lines[idx];
            let t = line.trim_start();
            if t.is_empty() {
                if !out.is_empty() {
                    break;
                }
                idx += 1;
                continue;
            }
            if t.starts_with("..") {
                let indent = line.len() - t.len();
                idx += 1;
                while idx < src_lines.len() {
                    let n = src_lines[idx];
                    if n.trim().is_empty() || n.len() - n.trim_start().len() > indent {
                        idx += 1;
                    } else {
                        break;
                    }
                }
                continue;
            }
            if is_rst_underline(t, 1) {
                break;
            }
            if is_rst_prose_line(line) {
                out.push(idx + 1);
            } else if !out.is_empty() {
                break;
            }
            idx += 1;
        }
        return out;
    }
    Vec::new()
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

/// Max body sections emitted from an RST README — keeps the tail off
/// the early budget the way `MAX_OUTLINE_HEADINGS` bounds the .md
/// outline. Generous enough for the typical orientation-bearing
/// sections (overview, layout, feature lists) at the top of the file.
const RST_MAX_BODY_SECTIONS: usize = 8;

/// Source-byte cap per RST body section — a directive-heavy or
/// reference-table section is truncated rather than dumped whole. About
/// 500 tokens; comparable to the .md `H2_SPLIT_BYTES` ceiling on a
/// single un-split section.
const RST_MAX_SECTION_BYTES: usize = 1800;

/// One RST README body section: the heading row + body rows (directive
/// blocks already stripped), 1-based, plus its post-title index and
/// whether its title is a canonical-usage marker. RST stays out of the
/// Markdown-only deferred prose tier for now: the line scanner does not have
/// the same README/dev-workflow calibration that stamps Markdown sections.
struct RstSection {
    rows: Vec<usize>,
    /// Position among body sections (0-based), used for index decay —
    /// the title section is excluded so the first body section is 0.
    index: usize,
    is_canonical_usage: bool,
    /// Title matches the shared reference/usage vocabulary
    /// ([`is_reference_usage_title_core`]).
    is_reference_usage: bool,
    /// Kept rows are catalog-dominant (same shape test as the .md
    /// reference-shaped flag, underline rows excluded) — carried into
    /// the `Section` key for the default concavity.
    reference_shaped: bool,
}

/// A scanned RST heading: the 1-based row of the title text and the
/// 1-based row of the underline that closes it. `start_row` is the
/// heading's first row — the overline row for overline form, the title
/// row for setext form — so a following section can bound itself at
/// `start_row - 1` and not swallow this heading's overline punctuation.
struct RstHeading {
    start_row: usize,
    title_row: usize,
    underline_row: usize,
    is_canonical_usage: bool,
}

/// Line-scan all setext/overline RST headings (same detection as
/// [`build_rst_readme_content`]). The first heading is the document
/// title; subsequent ones bound body sections.
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
                is_canonical_usage: is_rst_canonical_usage_title(src_lines[i + 1]),
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
                is_canonical_usage: is_rst_canonical_usage_title(title),
            });
            i += 2;
            continue;
        }
        i += 1;
    }
    headings
}

/// RST README body sections — every heading after the title, capped at
/// [`RST_MAX_BODY_SECTIONS`]. Each section spans its heading through the
/// row before the next heading, with `.. directive::` blocks and
/// leading/trailing blank rows stripped, then truncated to
/// [`RST_MAX_SECTION_BYTES`].
fn rst_body_sections(source: &str) -> Vec<RstSection> {
    let src_lines: Vec<&str> = source.lines().collect();
    let headings = scan_rst_headings(&src_lines);
    // headings[0] is the document title (covered by the headline batch).
    let mut out = Vec::new();
    for (body_idx, heading) in headings.iter().skip(1).enumerate() {
        if body_idx >= RST_MAX_BODY_SECTIONS {
            break;
        }
        let start_row = heading.title_row; // 1-based
        let end_row = headings
            .get(body_idx + 2) // +1 to undo skip(1), +1 for the next heading
            .map(|next| next.start_row - 1)
            .unwrap_or(src_lines.len());
        let rows = collect_rst_section_rows(&src_lines, start_row, end_row, heading.underline_row);
        if rows.is_empty() {
            continue;
        }
        let title = src_lines.get(heading.title_row - 1).copied().unwrap_or("");
        let reference_shaped = rst_rows_are_reference_shaped(&src_lines, &rows);
        out.push(RstSection {
            rows,
            index: body_idx,
            is_canonical_usage: heading.is_canonical_usage,
            is_reference_usage: is_reference_usage_title_core(&rst_title_core(title)),
            reference_shaped,
        });
    }
    out
}

/// RST analog of [`range_is_reference_shaped`] over an already-filtered
/// row list (directive blocks stripped). Heading underline rows are
/// pure punctuation and excluded from the dominance fraction — an RST
/// section carries two heading rows where .md carries one.
fn rst_rows_are_reference_shaped(src_lines: &[&str], rows: &[usize]) -> bool {
    let lines = rows.iter().filter_map(|&r| src_lines.get(r - 1).copied());
    lines_are_reference_shaped(lines.filter(|l| !is_rst_underline(l.trim(), 2)))
}

/// Rows (1-based) of one RST section: heading + body, skipping
/// non-content `.. directive::` blocks (figure / image / raw / toctree
/// / substitution / hyperlink-target comments) and their indented
/// continuations, plus leading/trailing blanks, bounded by
/// [`RST_MAX_SECTION_BYTES`]. Content directives (`code-block`,
/// `literalinclude`, …) are kept — their indented body is the example
/// the NS wants. The underline row is always kept so the heading
/// renders.
fn collect_rst_section_rows(
    src_lines: &[&str],
    start_row: usize,
    end_row: usize,
    underline_row: usize,
) -> Vec<usize> {
    let mut rows = Vec::new();
    let mut bytes = 0usize;
    let mut i = start_row; // 1-based
    while i <= end_row {
        let Some(line) = src_lines.get(i - 1) else {
            break;
        };
        let trimmed = line.trim_start();
        // Skip non-content directive blocks (but never the underline
        // row, whose `----` can't start with `..`). Content directives
        // fall through and render their indented body.
        if i != underline_row && trimmed.starts_with("..") && !is_rst_content_directive(trimmed) {
            let directive_indent = line.len() - trimmed.len();
            i += 1;
            while i <= end_row {
                let Some(next) = src_lines.get(i - 1) else {
                    break;
                };
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
        if bytes >= RST_MAX_SECTION_BYTES {
            break;
        }
        bytes += line.len();
        rows.push(i);
        i += 1;
    }
    // Drop leading/trailing blank rows so the marginal cost is body, not
    // whitespace.
    while rows
        .first()
        .is_some_and(|&r| src_lines.get(r - 1).is_none_or(|l| l.trim().is_empty()))
    {
        rows.remove(0);
    }
    while rows
        .last()
        .is_some_and(|&r| src_lines.get(r - 1).is_none_or(|l| l.trim().is_empty()))
    {
        rows.pop();
    }
    rows
}

/// True iff `trimmed` (a line already known to start with `..`) opens a
/// content-bearing directive whose indented body is substantive (code /
/// included source / literal blocks). These are kept; all other
/// directives (figure, image, raw, toctree, `.. _ref:` targets,
/// `.. |sub|` substitutions, plain `..` comments) are stripped.
fn is_rst_content_directive(trimmed: &str) -> bool {
    let Some(rest) = trimmed.strip_prefix("..") else {
        return false;
    };
    let rest = rest.trim_start();
    let name: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    matches!(
        name.as_str(),
        "code" | "code-block" | "sourcecode" | "literalinclude" | "parsed-literal"
    )
}

/// True iff an RST heading title is a canonical usage/example marker —
/// the RST analog of [`is_canonical_usage_h2_title`].
/// Lowercased leading alphanumeric/whitespace run of an RST heading
/// title — the string-level analog of `h2_title_core`.
fn rst_title_core(title: &str) -> String {
    let core: String = title
        .trim()
        .to_ascii_lowercase()
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect();
    core.trim().to_string()
}

fn is_rst_canonical_usage_title(title: &str) -> bool {
    matches!(
        rst_title_core(title).as_str(),
        "usage"
            | "sample usage"
            | "basic usage"
            | "example"
            | "examples"
            | "a definitive example"
            | "definitive example"
            | "usage example"
            | "usage examples"
            | "quick start"
            | "quickstart"
            | "getting started"
            | "demo"
    )
}

/// Down-weight for non-canonical, non-reference RST body sections.
/// The reliable wins are the canonical example/demo section and
/// reference catalogs (see below); remaining body sections (install /
/// contribute / contact prose) are supplementary and, on code-centric
/// repos, otherwise displace higher-value NS code surface — a blanket
/// 1.0 promoted beets' prose sections over its python surface.
const RST_NON_USAGE_SECTION_FACTOR: f64 = 0.5;

/// Value for an RST README body section — parity with the .md
/// [`readme_section_value`]: the same base signal mix and index decay,
/// the canonical-usage boost when the heading is an example/usage
/// marker, and the modest reference boost for reference-vocabulary
/// titles (`Key Features`) or catalog-shaped bodies (tinyusb's device/
/// host class-support lists — the NS's top README tier there; they
/// also ride the flat reference concavity via the key flag). RST has
/// no synthetic-intro wrap, so the first body section is index 0
/// directly. Plain prose keeps the non-usage damp.
fn rst_section_value(file: &Path, section: &RstSection, ctx: &WalkCtx) -> f64 {
    let base = mix_signals(0.55, 0.8, 0.7, path_depth_factor(file, ctx));
    let decay = index_decay(section.index, 0.15, 0.7);
    let usage = if section.is_canonical_usage || section.reference_shaped {
        // Catalog-shaped bodies take the full canonical factor: the
        // class-support matrix is the RST README's highest-yield
        // follow-up (tinyusb NS tier 1), and at the reference factor
        // its largest section still missed the schedule frontier.
        CANONICAL_USAGE_SECTION_FACTOR
    } else if section.is_reference_usage {
        REFERENCE_USAGE_SECTION_FACTOR
    } else {
        RST_NON_USAGE_SECTION_FACTOR
    };
    base * decay * usage
}

// --- headline spec + span construction ---

/// Computed shape of `ReadmeHeadline` — rendered source rows plus an
/// optional per-row truncation override for the heading line.
#[derive(Debug, Clone)]
struct HeadlineSpec {
    covered_rows: BTreeSet<usize>,
    truncate: Option<TruncatedRow>,
}

/// Per-row truncation override — paired so "rows without pattern" is
/// unrepresentable.
#[derive(Debug, Clone)]
struct TruncatedRow {
    row: usize,
    pattern: String,
}

/// Block kinds that bound a section — `ReadmeHeadline` never reaches
/// past these.
fn is_section_boundary(kind: &str) -> bool {
    matches!(kind, "section" | "atx_heading" | "setext_heading")
}

fn headline_spec(tree: &Tree, source: &str) -> Option<HeadlineSpec> {
    let section = headed_sections(tree.root_node()).next()?;
    let heading = first_heading_child(section)?;

    let mut covered: BTreeSet<usize> = BTreeSet::new();
    // Prelude content: README opens with HTML title blocks / badges /
    // lede paragraph before the first heading. Surface up to one
    // substantive paragraph, skipping decoratives.
    extend_prelude_lede(&mut covered, tree.root_node(), section, source);
    extend_rows_inclusive(&mut covered, heading, source);
    let heading_first_row = heading.start_position().row + 1;

    // Skip leading decorative paragraphs / image-only HTML / admin
    // block_quotes, then include blocks until the first substantive
    // paragraph. Short H1 taglines get one more non-decorative block.
    let post: Vec<Node> = children_after(section, heading);
    let mut i = 0;
    while i < post.len() {
        let block = post[i];
        if is_section_boundary(block.kind()) {
            break;
        }
        if !is_decorative_block(block, source) {
            break;
        }
        i += 1;
    }
    let mut state = HeadlineExtend::SeekFirst;
    let mut captured_lede = false;
    while i < post.len() {
        let block = post[i];
        if is_section_boundary(block.kind()) {
            break;
        }
        if is_decorative_block(block, source) {
            i += 1;
            continue;
        }
        extend_headline_block_rows(&mut covered, block, source);
        // Any substantive post-H1 block (paragraph, block_quote, list,
        // code lede) counts as a lede — the decorative-title fallback
        // below must only fire for a genuinely bare image/badge title,
        // not displace a non-paragraph lede.
        captured_lede = true;
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

    // Decorative-title fallback: an image/badge-only H1 yields no lede,
    // with the real "what is this" sentence pushed under the first
    // subsection. When that subsection is intro-class (`## Introduction`
    // / `## Overview` / `## About`), descend one level and capture its
    // first substantive paragraph as the lede.
    if !captured_lede
        && heading_level(heading) == 1
        && let Some(sub) = post.iter().find(|b| b.kind() == "section")
        && let Some(sub_heading) = first_heading_child(*sub)
        && is_intro_section_title(sub_heading, source)
    {
        extend_rows_inclusive(&mut covered, sub_heading, source);
        for inner in children_after(*sub, sub_heading) {
            if is_section_boundary(inner.kind()) {
                break;
            }
            if is_decorative_block(inner, source) {
                continue;
            }
            extend_headline_block_rows(&mut covered, inner, source);
            if inner.kind() == "paragraph" {
                break;
            }
        }
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
    let src_lines: Vec<&str> = source.lines().collect();
    let oversize_rows: Vec<usize> = spec
        .covered_rows
        .iter()
        .copied()
        .filter(|row| {
            Some(*row) != trunc_row
                && src_lines
                    .get(*row - 1)
                    .is_some_and(|line| line.chars().count() > HEADLINE_OVERSIZE_LINE_CHARS)
        })
        .collect();
    let full_rows: Vec<usize> = spec
        .covered_rows
        .iter()
        .copied()
        .filter(|row| Some(*row) != trunc_row && !oversize_rows.contains(row))
        .collect();

    // Full rows go through `build_file_spans` (blank-filter + merge);
    // splice the truncated-row span in afterwards.
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
    }
    spans.extend(oversize_rows.into_iter().map(|row| Span {
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

fn extend_headline_block_rows(out: &mut BTreeSet<usize>, node: Node, source: &str) {
    if node.end_byte() - node.start_byte() <= HEADLINE_BLOCK_BYTE_GATE {
        extend_rows_inclusive(out, node, source);
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
        out.insert(row + 1);
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

/// Inline children with no semantic content — whitespace and breaks.
fn is_skippable_inline(node: Node, source: &str) -> bool {
    match node.kind() {
        "text" => source[node.start_byte()..node.end_byte()].trim().is_empty(),
        "hard_line_break" | "soft_line_break" => true,
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
        // A paragraph can be a badge wall written as raw HTML
        // (`<a …><img …></a>` per line) rather than markdown images —
        // same decoration, so the same tag-stripping test applies.
        "paragraph" => {
            is_decorative_paragraph(block, source)
                || is_raw_html_markup(&source[block.start_byte()..block.end_byte()])
        }
        "html_block" => is_decorative_html_block(block, source),
        "block_quote" => is_admin_block_quote(block, source),
        _ => false,
    }
}

/// True iff every fragment (named + plain-text gaps) is decorative or
/// whitespace, and at least one fragment exists.
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

/// True when the text is nothing but HTML element tags and blank
/// filler — a badge wall or hero written as raw `<a …><img …></a>`
/// rather than markdown images. Unlike the `html_block` test this must
/// see a real element tag, so a markdown autolink (`<https://…>`) or
/// e-mail (`<a@b.c>`) still counts as content.
fn is_raw_html_markup(raw: &str) -> bool {
    let mut rest = raw;
    let mut saw_tag = false;
    while let Some(open) = rest.find('<') {
        let Some(close) = rest[open..].find('>').map(|i| open + i) else {
            return false;
        };
        if !is_blank_filler(&rest[..open]) || !is_html_element_tag(&rest[open + 1..close]) {
            return false;
        }
        saw_tag = true;
        rest = &rest[close + 1..];
    }
    saw_tag && is_blank_filler(rest)
}

/// A tag body naming an HTML element, as opposed to a markdown
/// autolink's URL (`https://…`) or e-mail address (`a@b.c`), whose
/// first token is not a bare element name.
fn is_html_element_tag(body: &str) -> bool {
    let name = body
        .trim_start_matches(['/', '!', '?', '-', ' '])
        .split([' ', '\t', '\n', '/', '='])
        .next()
        .unwrap_or_default();
    name.starts_with(|c: char| c.is_ascii_alphabetic())
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Whitespace once HTML character references (`&nbsp;`, `&#8226;`) are
/// removed — layout padding, not words.
fn is_blank_filler(text: &str) -> bool {
    strip_html_entities(text).trim().is_empty()
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

/// A "navigation paragraph" is a paragraph whose substantive content is
/// nothing but cross-reference links separated by separator punctuation
/// (`•`, `·`, `|`, `/`, `,`, dashes). Common in multi-language READMEs
/// (`[English](url) • [中文](url) • ...`) and in tagline-rich docs that
/// list related projects. The links carry no orientation value at the
/// budget where the headline lives — treat as decorative so the
/// headline walker doesn't burn its prelude on them.
fn is_nav_link_paragraph(para: Node, source: &str) -> bool {
    let Some(inline_block) = first_child_of_kind(para, "inline", false) else {
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
    strip_html_tags(raw)
        .chars()
        .filter(|c| !matches!(c, '>' | '*' | '_' | '`'))
        .collect()
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
    let inline_block = first_child_of_kind(heading, "inline", false)?;
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

    // Pattern includes the heading marker (`# `, `## `, …) because
    // `Render::Truncated` only renders matched bytes.
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
    /// Parent H2 title matches a canonical-usage marker (see
    /// [`is_canonical_usage_h2_title`]). README-only.
    parent_is_canonical_usage_h2: bool,
    /// This range independently has a canonical operational shape: a
    /// flag-first option table, a fenced CLI synopsis, a compact "How ...
    /// Works" section, or a structurally populated CLI-reference H2.
    /// README-only.
    is_canonical_operational_section: bool,
    /// This range's own (or parent H2's, for `Whole`/`Intro`) title
    /// matches the broader reference/usage vocabulary (see
    /// [`is_reference_usage_title`]) and the section has non-trivial
    /// body bytes. README-only; earns the modest
    /// [`REFERENCE_USAGE_SECTION_FACTOR`].
    is_reference_usage_section: bool,
    /// This range's body is structurally reference-shaped — dominated
    /// by list items / table rows / code-fence lines (see
    /// [`range_is_reference_shaped`]). README-only. Carried into the
    /// `Section` key so the scheduler prices the range at the default
    /// concavity instead of the steeper prose exponent.
    reference_shaped: bool,
    /// Dev-workflow doc section whose body carries repo-ops mechanics
    /// (commands, setup/test/debug steps, env vars, or concrete repo
    /// paths). Valued above peripheral-doc prose.
    dev_workflow_section: bool,
    /// Non-zero for a link-index roster chunk: the count of intra-doc
    /// link entries this chunk catalogs. Valued as a names surface
    /// (cat lift + [`roster_mass_factor`]) instead of section prose.
    roster_entries: usize,
    /// Roster chunks after the first gate on the previous chunk so the
    /// index delivers as an in-order prefix.
    chained_to_previous: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SectionKind {
    /// Un-split H2 (or synthetic H1-unwrap intro).
    Whole,
    /// H2 heading + prelude before its first sub-section.
    Intro,
    /// One H3 sub-section under a split H2.
    H3Child,
    /// One direct block inside a long split section.
    BodyBlock,
    /// Predecessor-chained tail chunk of an oversize head-split.
    OversizeTail,
    /// The section body directly behind a carved lede — the rest of
    /// what the unsplit section would have delivered.
    LedeBody,
}

/// Minimum fraction of a range's non-blank out-of-fence rows that must
/// be catalog rows (list / table) for [`range_is_reference_shaped`].
const REFERENCE_SHAPED_MIN_FRACTION: f64 = 0.6;

/// Minimum count of catalog rows — keeps two-line stubs from earning
/// the flatter concavity.
const REFERENCE_SHAPED_MIN_CATALOG_ROWS: usize = 3;

/// Source-byte bounds for a reference-shaped range. Below the floor the
/// flattened concavity just pulls micro-sections into the orientation
/// window (mkcert's 20-50-token stubs); above the cap the range is a
/// mega-catalog (sqlite-vec's 900-token install-matrix table) whose
/// early purchase evicts NS-ranked content wholesale.
const REFERENCE_SHAPED_MIN_BYTES: usize = 400;
const REFERENCE_SHAPED_MAX_BYTES: usize = 1600;

/// True iff the row range is dominated by catalog rows — list items and
/// table rows, measured outside code fences (fence delimiters and
/// interiors are neutral: excluded from both sides of the fraction) —
/// and its total source bytes sit inside the reference-shaped bounds.
/// Catalog sections (option tables, color/modifier lists, helper
/// indexes) are roster-like: their per-row information density doesn't
/// fall off the way prose does, so they shouldn't pay the steeper prose
/// concavity. Fence dominance deliberately does NOT qualify — compact
/// demo snippets on code-first repos are exactly what the prose
/// exponent exists to demote.
fn range_is_reference_shaped(src_lines: &[&str], start: usize, end: usize) -> bool {
    let last = end.min(src_lines.len());
    if start > last {
        return false;
    }
    lines_are_reference_shaped(src_lines[start - 1..last].iter().copied())
}

/// Shape test shared by the .md row-range and RST row-list callers —
/// see [`range_is_reference_shaped`] for the semantics.
fn lines_are_reference_shaped<'a>(lines: impl Iterator<Item = &'a str>) -> bool {
    let mut in_fence = false;
    let mut prose_or_catalog = 0usize;
    let mut catalog = 0usize;
    let mut bytes = 0usize;
    for line in lines {
        bytes += line.len() + 1;
        let t = line.trim_start();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("```") || t.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        prose_or_catalog += 1;
        if is_catalog_line(line) {
            catalog += 1;
        }
    }
    (REFERENCE_SHAPED_MIN_BYTES..=REFERENCE_SHAPED_MAX_BYTES).contains(&bytes)
        && catalog >= REFERENCE_SHAPED_MIN_CATALOG_ROWS
        && prose_or_catalog > 0
        && catalog as f64 / prose_or_catalog as f64 >= REFERENCE_SHAPED_MIN_FRACTION
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

/// True when a range is dominated by a Markdown option table whose first
/// column contains CLI flags. This is intentionally title-independent: many
/// READMEs put the synopsis and option catalog under project-specific H3s.
fn range_has_flag_option_table(src_lines: &[&str], start: usize, end: usize) -> bool {
    let last = end.min(src_lines.len());
    if start > last {
        return false;
    }
    let mut content_rows = 0usize;
    let mut table_rows = 0usize;
    let mut flag_rows = 0usize;
    // Marker-matched fence state — see `fence_closes`.
    let mut open_fence: Option<(char, usize)> = None;
    for line in &src_lines[start - 1..last] {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if let Some(open) = open_fence {
            if fence_closes(t, open) {
                open_fence = None;
            }
            continue;
        }
        if let Some(marker) = fence_marker(t) {
            open_fence = Some(marker);
            continue;
        }
        if is_markdown_rule_row(t) {
            continue;
        }
        content_rows += 1;
        if !t.contains('|') {
            continue;
        }
        table_rows += 1;
        let first_cell = t
            .trim_matches('|')
            .split('|')
            .next()
            .unwrap_or_default()
            .trim();
        if cell_starts_with_cli_flag(first_cell) {
            flag_rows += 1;
        }
    }
    flag_rows >= 3 && flag_rows * 5 >= table_rows * 3 && flag_rows * 2 >= content_rows
}

fn cell_starts_with_cli_flag(cell: &str) -> bool {
    let bytes = cell.as_bytes();
    bytes.iter().enumerate().any(|(idx, byte)| {
        if *byte != b'-'
            || idx
                .checked_sub(1)
                .is_some_and(|prev| bytes[prev].is_ascii_alphanumeric())
        {
            return false;
        }
        let mut next = idx + 1;
        if bytes.get(next) == Some(&b'-') {
            next += 1;
        }
        bytes.get(next).is_some_and(u8::is_ascii_alphanumeric)
    })
}

/// True when a fenced block begins with a command synopsis rather than source
/// code: an executable-like first token followed by flags or metavariables.
fn range_has_cli_synopsis(src_lines: &[&str], start: usize, end: usize) -> bool {
    let last = end.min(src_lines.len());
    if start > last {
        return false;
    }
    // Marker-matched fence state — see `fence_closes`.
    let mut open_fence: Option<(char, usize)> = None;
    let mut awaiting_first_line = false;
    for line in &src_lines[start - 1..last] {
        let t = line.trim();
        if let Some(open) = open_fence {
            if fence_closes(t, open) {
                open_fence = None;
            } else if awaiting_first_line && !t.is_empty() {
                if looks_like_cli_synopsis_line(t) {
                    return true;
                }
                awaiting_first_line = false;
            }
            continue;
        }
        if let Some(marker) = fence_marker(t) {
            open_fence = Some(marker);
            awaiting_first_line = true;
        }
    }
    false
}

fn looks_like_cli_synopsis_line(line: &str) -> bool {
    let mut line = line.trim_start_matches(['$', '>']).trim_start();
    if let Some(rest) = line
        .strip_prefix("Usage:")
        .or_else(|| line.strip_prefix("usage:"))
    {
        line = rest.trim_start();
    }
    let mut tokens = line.split_whitespace();
    let Some(command) = tokens.next() else {
        return false;
    };
    let command = command.trim_matches(|c: char| matches!(c, '`' | '"' | '\''));
    if command.is_empty()
        || !command
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '/' | '_' | '-' | '+'))
        || matches!(
            command,
            "const"
                | "let"
                | "var"
                | "function"
                | "class"
                | "def"
                | "fn"
                | "import"
                | "from"
                | "use"
                | "pub"
        )
    {
        return false;
    }
    let mut metavariables = 0usize;
    let mut explicit_options = false;
    for token in tokens {
        let token = token.trim_matches(|c: char| matches!(c, '`' | ',' | ';'));
        let lower = token.to_ascii_lowercase();
        explicit_options |= lower.starts_with("[option")
            || lower.starts_with("<option")
            || lower.starts_with("[flag")
            || lower.starts_with("<flag");
        let core = token.trim_matches(|c: char| matches!(c, '[' | ']' | '<' | '>' | '.'));
        if token.starts_with('[')
            || token.starts_with('<')
            || (core.len() > 1
                && core
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
        {
            metavariables += 1;
        }
    }
    explicit_options || metavariables >= 2
}

fn is_dev_workflow_doc(file: &Path) -> bool {
    file.file_stem().and_then(|s| s.to_str()).is_some_and(|s| {
        matches!(
            s.to_ascii_uppercase().as_str(),
            "CONTRIBUTING" | "DEVELOPING" | "DEVELOPMENT" | "HACKING"
        )
    })
}

const DEV_WORKFLOW_MIN_SIGNALS: usize = 2;
const DEV_WORKFLOW_MAX_BYTES: usize = 2200;

fn range_has_dev_workflow_signal(src_lines: &[&str], start: usize, end: usize) -> bool {
    let last = end.min(src_lines.len());
    if start > last {
        return false;
    }
    let mut bytes = 0usize;
    let mut signals = 0usize;
    let mut fence_has_signal = false;
    let mut in_fence = false;
    for line in &src_lines[start - 1..last] {
        bytes += line.len() + 1;
        let t = line.trim_start();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("```") || t.starts_with("~~~") {
            if in_fence && fence_has_signal {
                signals += 1;
            }
            in_fence = !in_fence;
            fence_has_signal = false;
            continue;
        }
        if in_fence {
            if line_has_density_signal(t, true) {
                fence_has_signal = true;
            }
            continue;
        }
        if line_has_density_signal(t, false) {
            signals += 1;
        }
    }
    if in_fence && fence_has_signal {
        signals += 1;
    }
    bytes <= DEV_WORKFLOW_MAX_BYTES && signals >= DEV_WORKFLOW_MIN_SIGNALS
}

fn line_has_density_signal(t: &str, in_fence: bool) -> bool {
    if in_fence {
        return is_dev_command_line(t)
            || is_dev_config_line(t)
            || is_dev_config_snippet_line(t)
            || is_repo_path_line(t);
    }
    is_dev_command_line(t)
        || is_dev_config_line(t)
        || is_repo_path_line(t)
        || (is_numbered_step_line(t) && contains_dev_action(t))
}

fn is_markdown_rule_row(t: &str) -> bool {
    let stripped = t.trim_matches('|').trim();
    !stripped.is_empty()
        && stripped
            .bytes()
            .all(|b| matches!(b, b'-' | b':' | b' ' | b'\t' | b'|'))
}

fn is_config_key_token(token: &str) -> bool {
    matches!(
        token,
        "bin"
            | "browser"
            | "dependencies"
            | "devDependencies"
            | "exports"
            | "files"
            | "main"
            | "module"
            | "scripts"
            | "types"
    ) || token.ends_with(".json")
        || token.ends_with(".toml")
        || token.ends_with(".yaml")
        || token.ends_with(".yml")
        || token.contains("config")
}

fn is_dev_config_snippet_line(t: &str) -> bool {
    let Some((key, _)) = t.split_once([':', '=']) else {
        return false;
    };
    let key = key
        .trim()
        .trim_matches(|c: char| matches!(c, '"' | '\'' | '`' | '{' | '[' | ',' | ' ' | '\t'));
    matches!(
        key,
        "compilerOptions"
            | "dependencies"
            | "devDependencies"
            | "extends"
            | "module"
            | "options"
            | "parser"
            | "plugins"
            | "presets"
            | "rules"
            | "scripts"
    ) || is_config_key_token(key)
}

fn is_dev_command_line(t: &str) -> bool {
    const DEV_COMMAND_TOKENS: &[&str] = &[
        "npm",
        "pnpm",
        "yarn",
        "cargo",
        "rustup",
        "go",
        "make",
        "git",
        "pip",
        "pip3",
        "python",
        "python3",
        "pytest",
        "poetry",
        "uv",
        "docker",
        "docker-compose",
        "gradle",
        "mvn",
    ];
    let Some(token) = first_command_token(t) else {
        return false;
    };
    token.starts_with("./") || DEV_COMMAND_TOKENS.contains(&token)
}

fn first_command_token(t: &str) -> Option<&str> {
    let mut code = t.trim_start();
    while let Some(rest) = code.strip_prefix('`') {
        code = rest.trim_start();
    }
    if let Some(rest) = code.strip_prefix("$ ") {
        code = rest.trim_start();
    } else if let Some(rest) = code.strip_prefix("> ") {
        code = rest.trim_start();
    }
    code.split_whitespace()
        .next()
        .map(|token| token.trim_end_matches('`'))
}

fn is_dev_config_line(t: &str) -> bool {
    t.contains("package.json")
        || t.contains("pyproject.toml")
        || t.contains("setup.py")
        || t.contains("requirements.txt")
        || t.contains("Makefile")
        || t.contains("Dockerfile")
        || t.contains("pnpm-workspace")
        || t.contains("vitest")
        || t.contains("playwright")
        || t.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .any(is_env_var_token)
}

fn is_repo_path_line(t: &str) -> bool {
    t.contains("src/")
        || t.contains("packages/")
        || t.contains("crates/")
        || t.contains("tests/")
        || t.contains(".github/")
        || t.contains(".ts")
        || t.contains(".js")
        || t.contains(".rs")
        || t.contains(".py")
        || t.contains(".json")
}

fn is_numbered_step_line(t: &str) -> bool {
    t.split_once(". ")
        .is_some_and(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

fn contains_dev_action(t: &str) -> bool {
    let lower = t.to_ascii_lowercase();
    lower.contains("run ")
        || lower.contains("test")
        || lower.contains("build")
        || lower.contains("debug")
        || lower.contains("install")
        || lower.contains("clone")
        || lower.contains("create")
        || lower.contains("edit")
        || lower.contains("update")
}

fn is_env_var_token(token: &str) -> bool {
    token.len() >= 5
        && token.contains('_')
        && token
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
}

/// Section ranges for batching. H2s that satisfy a split rule expand
/// to an optional `Intro` plus per-child sub-ranges (bullet split,
/// H3 split, or body-block split). Other top-level entries emit one
/// `Whole`.
///
/// Splits drop the section's heading row — sub-ranges are built from
/// body blocks, and headings are scaffolding to that walk. `outlined`
/// is the set of rows `HeadingsOutline` will actually render, and it is
/// the only place a dropped heading row can find an owner: a heading in
/// neither the outline nor a section range is unrenderable at every
/// budget, so the split paths keep any heading row `outlined` does not
/// cover. Empty when the outline is suppressed, and — on the mega-doc
/// H2-only skeleton — short of the file's full heading set.
fn logical_sections(
    file: &Path,
    tree: &Tree,
    source: &str,
    outlined: &BTreeSet<usize>,
    outline_truncated: bool,
    root_readme: bool,
) -> Vec<SectionRange> {
    let outline_will_emit = !outlined.is_empty();
    let entries = top_level_entries(tree.root_node(), source);
    let split_eligible_file = !is_changelog_class(file);
    // Oversize head-split scope: the root README only. Its early
    // content is what NSes rank inside the early-budget envelope, so
    // unlocking early purchase there is recall; peripheral docs
    // (upgrade guides, nested docs) are NS-ranked late, where lump
    // size is not a purchase barrier — splitting them only hands a
    // cheap full-value head to content the schedule shouldn't buy
    // early.
    let oversize_split_eligible = split_eligible_file && root_readme;
    let synthetic_intro_present =
        matches!(entries.first(), Some(TopLevelEntry::SyntheticIntro { .. }));

    let readme = is_readme(file);
    let src_lines: Vec<&str> = source.lines().collect();
    let mut out = Vec::with_capacity(entries.len());
    let mut mechanics_h2_parents = BTreeSet::new();
    let mut cli_reference_h2_parents = BTreeSet::new();
    for (parent_idx, entry) in entries.iter().enumerate() {
        match entry {
            TopLevelEntry::SyntheticIntro { start, end } => {
                push_whole_or_head_split(
                    &mut out,
                    &src_lines,
                    SectionRange {
                        start: *start,
                        end: *end,
                        kind: SectionKind::Whole,
                        parent_index: parent_idx,
                        synthetic_intro_present,
                        parent_is_canonical_usage_h2: false,
                        is_canonical_operational_section: false,
                        is_reference_usage_section: false,
                        reference_shaped: false,
                        dev_workflow_section: false,
                        roster_entries: 0,
                        chained_to_previous: false,
                    },
                    oversize_split_eligible,
                );
            }
            TopLevelEntry::H2Section { node, start, end } => {
                let bytes = node.end_byte() - node.start_byte();
                let structural_split_gate =
                    split_eligible_file && outline_will_emit && bytes >= H2_SPLIT_BYTES;
                let body_block_split_gate = split_eligible_file && bytes >= H2_SPLIT_BYTES;
                let usage_h2 = readme && is_canonical_usage_h2(*node, source);
                // README H2 whose title is in the reference/usage
                // vocabulary, regardless of code fraction. Gates: a
                // non-trivial but compact body (so stub H2s and large
                // prose/demo blobs are excluded) AND structural
                // reference content (list / table / code), so a
                // prose-only intro under a reference title is skipped.
                let reference_h2 = readme
                    && is_reference_usage_title(*node, source)
                    && reference_usage_body_ok(*node)
                    && reference_usage_has_structure(*node, source);
                if root_readme && reference_h2 && is_cli_reference_title(*node, source) {
                    cli_reference_h2_parents.insert(parent_idx);
                }

                let h3s = if structural_split_gate {
                    direct_h3_children(*node)
                } else {
                    Vec::new()
                };
                if root_readme
                    && is_mechanics_h2_title(*node, source)
                    && compact_mechanics_body_ok(*node)
                {
                    mechanics_h2_parents.insert(parent_idx);
                }
                if h3s.len() >= 2 {
                    // Mega-README reference H2 (lo's `## Spec`, with a
                    // per-API-symbol H3 for every index entry): the
                    // prelude before the first H3 is the navigation
                    // index over the H3 set the truncated outline can't
                    // name — deliver it as chained roster chunks. The
                    // H3-count gate keeps ordinary TOC sections (whose
                    // entries mirror the outline's own heading set —
                    // pure double-buying) at prose pricing.
                    let intro_end = (h3s[0].start_position().row + 1).saturating_sub(1);
                    let indexed = readme
                        && outline_truncated
                        && h3s.len() >= LINK_INDEX_MIN_H3_CHILDREN
                        && intro_end >= *start
                        && push_link_index_chunks(
                            &mut out,
                            *start,
                            intro_end,
                            parent_idx,
                            synthetic_intro_present,
                            source,
                        );
                    if !indexed {
                        push_intro(
                            &mut out,
                            *node,
                            h3s[0],
                            outlined.contains(start),
                            parent_idx,
                            synthetic_intro_present,
                            source,
                        );
                    }
                    for h3 in &h3s {
                        push_h3_child_or_body_blocks(
                            &mut out,
                            *h3,
                            outlined,
                            parent_idx,
                            synthetic_intro_present,
                            source,
                        );
                    }
                } else {
                    let did_body_split = body_block_split_gate
                        && push_list_body_blocks(
                            &mut out,
                            *node,
                            outlined.contains(start),
                            parent_idx,
                            synthetic_intro_present,
                            source,
                        );
                    let did_fence_split = !did_body_split
                        && root_readme
                        && usage_h2
                        && bytes >= H2_SPLIT_BYTES
                        && push_canonical_usage_fence_split(
                            &mut out,
                            *node,
                            *start,
                            *end,
                            parent_idx,
                            synthetic_intro_present,
                            source,
                        );
                    if !did_body_split && !did_fence_split {
                        push_whole_or_head_split(
                            &mut out,
                            &src_lines,
                            SectionRange {
                                start: *start,
                                end: *end,
                                kind: SectionKind::Whole,
                                parent_index: parent_idx,
                                synthetic_intro_present,
                                parent_is_canonical_usage_h2: usage_h2,
                                is_canonical_operational_section: false,
                                is_reference_usage_section: reference_h2,
                                reference_shaped: false,
                                dev_workflow_section: false,
                                roster_entries: 0,
                                chained_to_previous: false,
                            },
                            oversize_split_eligible,
                        );
                    }
                }
            }
        }
    }
    // Root-README-only: mark catalog-dominant ranges so the scheduler
    // prices them at the default concavity (the steeper `Section` prose
    // exponent exists to demote prose, not catalogs). Gated on the
    // reference-usage title vocabulary: structure alone misfires —
    // sponsor tables, TOCs, "Supported root stores" / "Security"
    // bullet lists are list-dominant but NS-tier-2, and flattening
    // them displaced NS-ranked code surfaces (mkcert/peepdb/d2ts).
    // Nested READMEs stay at prose pricing — flattening them re-fed
    // the per-driver README flood the NS treats as catalog-listing
    // material.
    if root_readme {
        for range in &mut out {
            if range.roster_entries > 0 {
                continue; // link-index chunks set the flag themselves
            }
            let range_tokens: usize = (range.start..=range.end)
                .map(|row| row_tokens(&src_lines, row))
                .sum();
            range.is_canonical_operational_section = range_tokens
                <= STRUCTURAL_OPERATIONAL_MAX_TOKENS
                && (range_has_flag_option_table(&src_lines, range.start, range.end)
                    || range_has_cli_synopsis(&src_lines, range.start, range.end)
                    || mechanics_h2_parents.contains(&range.parent_index)
                    || cli_reference_h2_parents.contains(&range.parent_index));
            range.reference_shaped = range.is_reference_usage_section
                && range_is_reference_shaped(&src_lines, range.start, range.end);
        }
    }
    if is_dev_workflow_doc(file) {
        for range in &mut out {
            range.dev_workflow_section =
                range_has_dev_workflow_signal(&src_lines, range.start, range.end);
            range.reference_shaped |= range.dev_workflow_section;
        }
    }
    out
}

/// Minimum intra-doc link entries for a body to count as a link index,
/// and the dominance fraction those entries must hold among non-blank
/// body rows. Tighter than the generic catalog test — a navigation
/// index is a wall of `- [Name](#anchor)` bullets.
const LINK_INDEX_MIN_ENTRIES: usize = 20;
const LINK_INDEX_MIN_FRACTION: f64 = 0.6;
/// Entries a chunk must hold before a prose row may close it — keeps
/// family-header prose rows (`Supported helpers for maps:`) glued to
/// the run they introduce without spawning micro-chunks.
const LINK_INDEX_MIN_CHUNK_ENTRIES: usize = 8;
/// Direct H3 children a reference H2 must have before its intro is
/// treated as a roster index — one H3 per API symbol is the
/// reference-README shape; a TOC over a normal H2/H3 set is not.
const LINK_INDEX_MIN_H3_CHILDREN: usize = 12;

/// A bullet whose payload is an intra-doc anchor link —
/// `- [Filter](#filter)`. Indented sub-bullets count.
fn is_link_index_row(line: &str) -> bool {
    let t = line.trim_start();
    let Some(rest) = t
        .strip_prefix("- ")
        .or_else(|| t.strip_prefix("* "))
        .or_else(|| t.strip_prefix("+ "))
    else {
        return false;
    };
    rest.trim_start().starts_with('[') && rest.contains("](#")
}

/// Split `[start, end]` into `(start, end, entries)` roster chunks at
/// prose boundaries, or `None` when the range isn't link-index shaped
/// (see the `LINK_INDEX_*` gates). Chunks break where a prose row
/// follows a full run — for lo this is exactly the per-family
/// (`slices` / `maps` / `math`) boundaries the NS chunks at.
fn link_index_chunks(
    src_lines: &[&str],
    start: usize,
    end: usize,
) -> Option<Vec<(usize, usize, usize)>> {
    let mut entries = 0usize;
    let mut nonblank = 0usize;
    for row in start..=end {
        let Some(line) = src_lines.get(row - 1) else {
            break;
        };
        if line.trim().is_empty() {
            continue;
        }
        nonblank += 1;
        if is_link_index_row(line) {
            entries += 1;
        }
    }
    if entries < LINK_INDEX_MIN_ENTRIES
        || (entries as f64) < LINK_INDEX_MIN_FRACTION * nonblank as f64
    {
        return None;
    }
    let mut chunks = Vec::new();
    let mut chunk_start = start;
    let mut chunk_entries = 0usize;
    for row in start..=end {
        let Some(line) = src_lines.get(row - 1) else {
            break;
        };
        if line.trim().is_empty() {
            continue;
        }
        if is_link_index_row(line) {
            chunk_entries += 1;
        } else if chunk_entries >= LINK_INDEX_MIN_CHUNK_ENTRIES {
            chunks.push((chunk_start, row - 1, chunk_entries));
            chunk_start = row;
            chunk_entries = 0;
        }
    }
    if chunk_entries > 0 {
        chunks.push((chunk_start, end, chunk_entries));
    } else if let Some(last) = chunks.last_mut() {
        // A prose-only tail is not a roster; fold it into the previous
        // chunk without inflating that chunk's entry count. (`chunks`
        // can't be empty here: the entry gate above guarantees ≥20
        // entries, and `chunk_entries` only resets when a chunk is
        // pushed.)
        last.1 = end;
    }
    Some(chunks)
}

/// Emit a link-index body as chained roster chunks (see
/// [`link_index_chunks`]); false when the body isn't index-shaped.
fn push_link_index_chunks(
    out: &mut Vec<SectionRange>,
    start: usize,
    end: usize,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) -> bool {
    let src_lines: Vec<&str> = source.lines().collect();
    let Some(chunks) = link_index_chunks(&src_lines, start, end) else {
        return false;
    };
    for (i, (chunk_start, chunk_end, entries)) in chunks.into_iter().enumerate() {
        out.push(SectionRange {
            start: chunk_start,
            end: chunk_end,
            kind: SectionKind::Intro,
            parent_index: parent_idx,
            synthetic_intro_present,
            parent_is_canonical_usage_h2: false,
            is_canonical_operational_section: false,
            is_reference_usage_section: false,
            reference_shaped: true,
            dev_workflow_section: false,
            roster_entries: entries,
            chained_to_previous: i > 0,
        });
    }
    true
}

/// Split a large canonical-usage H2 (no H3 children, no list-only
/// body) at the end of its first code fence: the heading + prelude +
/// first fence is the canonical snippet — buyable separately from the
/// demo blob that follows. The snippet keeps the canonical-usage boost
/// as a `Whole`; the remainder is a `BodyBlock` (demo-blob tier).
/// Returns false (no ranges pushed) when there's no direct fence or
/// the remainder lacks substantive content.
fn push_canonical_usage_fence_split(
    out: &mut Vec<SectionRange>,
    h2_section: Node<'_>,
    start: usize,
    end: usize,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) -> bool {
    let mut cur = h2_section.walk();
    let Some(first_fence) = h2_section
        .children(&mut cur)
        .find(|c| is_code_block(c.kind()))
    else {
        return false;
    };
    let (_, fence_end) = node_row_range(first_fence, source);
    if fence_end >= end {
        return false;
    }
    let src_lines: Vec<&str> = source.lines().collect();
    let rest_start = fence_end + 1;
    let rest_has_content =
        (rest_start..=end).any(|r| src_lines.get(r - 1).is_some_and(|l| !l.trim().is_empty()));
    if !rest_has_content {
        return false;
    }
    out.push(SectionRange {
        start,
        end: fence_end,
        kind: SectionKind::Whole,
        parent_index: parent_idx,
        synthetic_intro_present,
        parent_is_canonical_usage_h2: true,
        is_canonical_operational_section: false,
        is_reference_usage_section: false,
        reference_shaped: false,
        dev_workflow_section: false,
        roster_entries: 0,
        chained_to_previous: false,
    });
    out.push(SectionRange {
        start: rest_start,
        end,
        kind: SectionKind::BodyBlock,
        parent_index: parent_idx,
        synthetic_intro_present,
        parent_is_canonical_usage_h2: false,
        is_canonical_operational_section: false,
        is_reference_usage_section: false,
        reference_shaped: false,
        dev_workflow_section: false,
        roster_entries: 0,
        chained_to_previous: false,
    });
    true
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
/// Shared with the Rust walker's crate-doc chunking and attr gate.
pub(in crate::walker) fn row_tokens(src_lines: &[&str], row: usize) -> usize {
    src_lines
        .get(row - 1)
        .map(|l| tokenizer::count(&format!("{l}\n")))
        .unwrap_or(0)
}

/// The fence delimiter at the start of an already-trimmed line — its
/// marker char and run length (≥ 3) — if any. Shared with the Rust
/// walker's crate-doc chunking (rustdoc is markdown).
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
/// shrink `head` to the first chunk (keeping its kind and value flags;
/// the head includes the section heading, so no outline is required
/// to preserve it) and follow it with predecessor-chained
/// `OversizeTail` chunks. Tails carry the head's boost flags so a
/// boosted section's continuation is priced off the same base the head
/// won its rank with ([`OVERSIZE_TAIL_FACTOR`]'s rationale); the other
/// fields stay positional. When the head itself carves a lede
/// ([`LEDE_TARGET_TOKENS`]), the chunk behind it is a
/// [`SectionKind::LedeBody`] rather than a tail.
fn push_whole_or_head_split(
    out: &mut Vec<SectionRange>,
    src_lines: &[&str],
    head: SectionRange,
    split_eligible: bool,
) {
    let (start, end) = (head.start, head.end);
    if !split_eligible {
        out.push(head);
        return;
    }
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
        } else if i == 1 && lede {
            out.push(SectionRange {
                start: chunk_start,
                end: chunk_end,
                kind: SectionKind::LedeBody,
                chained_to_previous: true,
                ..head
            });
        } else {
            out.push(SectionRange {
                start: chunk_start,
                end: chunk_end,
                kind: SectionKind::OversizeTail,
                parent_index: head.parent_index,
                synthetic_intro_present: head.synthetic_intro_present,
                parent_is_canonical_usage_h2: head.parent_is_canonical_usage_h2,
                is_canonical_operational_section: head.is_canonical_operational_section,
                is_reference_usage_section: head.is_reference_usage_section,
                reference_shaped: false,
                dev_workflow_section: false,
                roster_entries: 0,
                chained_to_previous: true,
            });
        }
    }
}

/// Append an `Intro` range covering the H2 heading + prelude before
/// `first_child` (the first H3 or the section's bullet list). A prelude
/// with no substantive non-heading content is skipped as a zero-cost
/// duplicate of the outline's own heading row — but only when the
/// outline really renders that row (`heading_outlined`). It does not on
/// the mega-doc H2-only skeleton, which keeps a prefix of the H2 set:
/// for a late H2 this Intro is the heading row's only owner.
fn push_intro<'a>(
    out: &mut Vec<SectionRange>,
    h2_section: Node<'a>,
    first_child: Node<'a>,
    heading_outlined: bool,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) {
    let h2_start = h2_section.start_position().row + 1;
    let intro_end = (first_child.start_position().row + 1).saturating_sub(1);
    if intro_end < h2_start {
        return;
    }
    if heading_outlined && !has_substantive_body(h2_section, h2_start, intro_end, source) {
        return;
    }
    // The H2 title carries the reference/usage match; the prelude
    // before the first H3 inherits it. The compact-body gate measures
    // just the prelude rows (the whole split H2 is large by
    // construction). README-only — the caller is gated.
    let reference_h2 = is_reference_usage_title(h2_section, source)
        && reference_usage_row_range_ok(h2_section, h2_start, intro_end, source);
    out.push(SectionRange {
        start: h2_start,
        end: intro_end,
        kind: SectionKind::Intro,
        parent_index: parent_idx,
        synthetic_intro_present,
        parent_is_canonical_usage_h2: false,
        is_canonical_operational_section: false,
        is_reference_usage_section: reference_h2,
        reference_shaped: false,
        dev_workflow_section: false,
        roster_entries: 0,
        chained_to_previous: false,
    });
}

fn push_h3_child_or_body_blocks(
    out: &mut Vec<SectionRange>,
    h3_section: Node<'_>,
    outlined: &BTreeSet<usize>,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) {
    let (h3_start, h3_end) = node_row_range(h3_section, source);
    let heading_outlined = outlined.contains(&h3_start);
    if !has_substantive_body(h3_section, h3_start, h3_end, source) {
        // An H3 with nothing under its title (an undocumented entry in
        // a generated API index) is left to the outline, which renders
        // that row already — an H3Child here would be a zero-cost
        // duplicate. The mega-doc skeleton names H2s only, though, so
        // under truncation this batch is the row's only possible owner.
        if heading_outlined {
            return;
        }
        if let Some(heading) = first_heading_child(h3_section) {
            push_h3_child(
                out,
                h3_section,
                node_end_row_trimmed(heading, source) + 1,
                parent_idx,
                synthetic_intro_present,
                source,
            );
        }
        return;
    }
    let h3_bytes = h3_section.end_byte() - h3_section.start_byte();
    if h3_bytes >= BODY_BLOCK_SPLIT_BYTES {
        let mut ranges = body_block_ranges(h3_section, source);
        // The mega-doc skeleton names H2s only, so an H3 heading split
        // into body blocks has no outline row to fall back on.
        if !heading_outlined {
            keep_heading_row(&mut ranges, h3_start);
        }
        if push_body_block_ranges(out, ranges, parent_idx, synthetic_intro_present) {
            return;
        }
    }
    push_h3_child(
        out,
        h3_section,
        h3_end,
        parent_idx,
        synthetic_intro_present,
        source,
    );
}

fn push_h3_child(
    out: &mut Vec<SectionRange>,
    h3_section: Node<'_>,
    end: usize,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) {
    // The H3's OWN title carries the reference/usage match (e.g.
    // `### Colors`, `### Default preset`). Same gates as the H2 path:
    // compact body + structural reference content.
    let reference_h3 = is_reference_usage_title(h3_section, source)
        && reference_usage_body_ok(h3_section)
        && reference_usage_has_structure(h3_section, source);
    out.push(SectionRange {
        start: h3_section.start_position().row + 1,
        end,
        kind: SectionKind::H3Child,
        parent_index: parent_idx,
        synthetic_intro_present,
        parent_is_canonical_usage_h2: false,
        is_canonical_operational_section: false,
        is_reference_usage_section: reference_h3,
        reference_shaped: false,
        dev_workflow_section: false,
        roster_entries: 0,
        chained_to_previous: false,
    });
}

fn push_body_block_ranges(
    out: &mut Vec<SectionRange>,
    ranges: Vec<(usize, usize)>,
    parent_idx: usize,
    synthetic_intro_present: bool,
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
        parent_is_canonical_usage_h2: false,
        is_canonical_operational_section: false,
        is_reference_usage_section: false,
        reference_shaped: false,
        dev_workflow_section: false,
        roster_entries: 0,
        chained_to_previous: false,
    }));
    true
}

fn push_list_body_blocks(
    out: &mut Vec<SectionRange>,
    section: Node<'_>,
    heading_outlined: bool,
    parent_idx: usize,
    synthetic_intro_present: bool,
    source: &str,
) -> bool {
    let Some(mut ranges) = list_only_body_block_ranges(section, source) else {
        return false;
    };
    // Unlike the structural split, this one does not require the outline
    // to emit at all — a single-heading list doc splits with no outline
    // behind it, and then only the first item can carry the title.
    if !heading_outlined {
        keep_heading_row(&mut ranges, section.start_position().row + 1);
    }
    push_body_block_ranges(out, ranges, parent_idx, synthetic_intro_present)
}

/// Extend the first block range back over the section's heading row, so
/// a split section whose heading no outline renders still has exactly
/// one owner for its own title. No-op on an empty range list (no split
/// follows, and the un-split `Whole` already covers the heading).
fn keep_heading_row(ranges: &mut [(usize, usize)], heading_start: usize) {
    if let Some((start, _)) = ranges.first_mut() {
        *start = heading_start.min(*start);
    }
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

/// Title half of the compact mechanics-section classifier. The call site also
/// requires a mid-sized body below the reference-section byte cap, keeping
/// both a generic stub and a large essay out of the early orientation tier.
fn is_mechanics_h2_title(h2_section: Node<'_>, source: &str) -> bool {
    let Some(core) = h2_title_core(h2_section, source) else {
        return false;
    };
    let core = core.trim();
    core == "how it works" || (core.starts_with("how ") && core.ends_with(" works"))
}

fn compact_mechanics_body_ok(section: Node<'_>) -> bool {
    const MIN_BODY_BYTES: usize = 400;
    let Some(heading) = first_heading_child(section) else {
        return false;
    };
    let heading_bytes = heading.end_byte() - heading.start_byte();
    let body_bytes = (section.end_byte() - section.start_byte()).saturating_sub(heading_bytes);
    (MIN_BODY_BYTES..=REFERENCE_USAGE_MAX_BODY_BYTES).contains(&body_bytes)
}

/// README reference/usage sections worth the modest
/// [`REFERENCE_USAGE_SECTION_FACTOR`]: title (H2 or H3, taken from the
/// section's own first heading) matches a tight reference/usage
/// vocabulary. Unlike [`is_canonical_usage_h2`] this is NOT gated on
/// code dominance — the point is to lift prose/list/table reference
/// sections (option tables, color/modifier lists, environment-variable
/// docs) above the README index decay so they clear the early budget.
/// The vocabulary is kept tight and the body-bytes gate is enforced at
/// the call sites; a too-broad list would over-promote trivial
/// sections. README-only (gated at the call sites).
fn is_reference_usage_title(section: Node<'_>, source: &str) -> bool {
    let Some(core) = h2_title_core(section, source) else {
        return false;
    };
    is_reference_usage_title_core(&core)
}

fn is_cli_reference_title(section: Node<'_>, source: &str) -> bool {
    let Some(core) = h2_title_core(section, source) else {
        return false;
    };
    let core = core.trim();
    matches!(
        core,
        "command line usage"
            | "command-line usage"
            | "cli usage"
            | "command line options"
            | "command-line options"
            | "options"
            | "flags"
    )
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

/// True iff `section`'s non-heading body is non-empty and at most
/// [`REFERENCE_USAGE_MAX_BODY_BYTES`] — the body-size gate for the
/// `Whole` (unsplit H2) and `H3Child` reference/usage boost. Measured
/// over the whole node; the split-H2 `Intro` uses
/// [`reference_usage_row_range_ok`] over just the prelude rows.
fn reference_usage_body_ok(section: Node<'_>) -> bool {
    let Some(heading) = first_heading_child(section) else {
        return false;
    };
    let heading_bytes = heading.end_byte() - heading.start_byte();
    let body_bytes = (section.end_byte() - section.start_byte()).saturating_sub(heading_bytes);
    body_bytes > 0 && body_bytes <= REFERENCE_USAGE_MAX_BODY_BYTES
}

/// Like [`reference_usage_body_ok`] but over a 1-based source row range
/// `[start, end]` (heading rows excluded). Used for a split H2's
/// `Intro`, whose body is just the prelude before the first H3 — the
/// whole-H2 byte count would always exceed the cap (the H2 split only
/// because it's large), so svgo's `## Configuration` prelude would be
/// wrongly rejected.
fn reference_usage_row_range_ok(section: Node, start: usize, end: usize, source: &str) -> bool {
    let Some(heading) = first_heading_child(section) else {
        return false;
    };
    let heading_first_row = heading.start_position().row + 1;
    let heading_last_row = node_end_row_trimmed(heading, source) + 1;
    let body: Vec<&str> = source
        .lines()
        .enumerate()
        .skip(start.saturating_sub(1))
        .take(end.saturating_sub(start) + 1)
        .filter(|(idx, _)| !(heading_first_row..=heading_last_row).contains(&(idx + 1)))
        .map(|(_, line)| line)
        .collect();
    // +1 for the newline each line carried in the source.
    let body_bytes: usize = body.iter().map(|l| l.len() + 1).sum();
    body_bytes > 0
        && body_bytes <= REFERENCE_USAGE_MAX_BODY_BYTES
        && body.iter().any(|l| is_reference_structure_line(l))
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

/// True iff the section node's body (heading excluded) contains
/// structural reference content — see [`is_reference_structure_line`].
/// Node-based variant for the `Whole` / `H3Child` paths.
fn reference_usage_has_structure(section: Node<'_>, source: &str) -> bool {
    let start = section.start_position().row + 1;
    let end = node_end_row_trimmed(section, source) + 1;
    let Some(heading) = first_heading_child(section) else {
        return false;
    };
    let heading_first_row = heading.start_position().row + 1;
    let heading_last_row = node_end_row_trimmed(heading, source) + 1;
    source
        .lines()
        .enumerate()
        .skip(start.saturating_sub(1))
        .take(end.saturating_sub(start) + 1)
        .filter(|(idx, _)| !(heading_first_row..=heading_last_row).contains(&(idx + 1)))
        .any(|(_, line)| is_reference_structure_line(line))
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
/// found. Used by [`is_canonical_usage_h2_title`].
fn h2_title_core(h2_section: Node<'_>, source: &str) -> Option<String> {
    let heading = first_heading_child(h2_section)?;
    let inline = first_child_of_kind(heading, "inline", false)?;
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
    let heading_last_row = node_end_row_trimmed(heading, source) + 1;
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

/// True for an admin/migration warning paragraph (`⚠️ …`, deprecation /
/// breaking-change notices) READMEs place above the lede. These are
/// appendix, not the project's "what is this" sentence — skip them so the
/// real lede is what the headline captures.
fn is_admin_warning_paragraph(para: Node, source: &str) -> bool {
    let text = source[para.start_byte()..para.end_byte()].trim_start();
    if ["⚠", "🚨", "❗", "‼", "🛑"]
        .iter()
        .any(|m| text.starts_with(m))
    {
        return true;
    }
    let lower = text.to_ascii_lowercase();
    let head = lower.trim_start_matches(['*', '>', '_', ' ']);
    head.starts_with("warning")
        || head.starts_with("note:")
        || head.starts_with("caution")
        || head.starts_with("deprecated")
        || head.starts_with("important:")
        || head.starts_with("breaking change")
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
    let prelude_blocks = prelude_blocks(root, first_headed);

    let mut i = 0;
    while i < prelude_blocks.len() {
        if !is_headline_skippable_block(prelude_blocks[i], source) {
            break;
        }
        i += 1;
    }
    // Include the first substantive prelude block. Short taglines
    // extend to one more non-decorative block (skipping decoratives
    // between).
    if let Some(block) = prelude_blocks.get(i) {
        extend_rows_inclusive(covered, *block, source);
        if is_short_substantive_block(*block, source) {
            let mut j = i + 1;
            while j < prelude_blocks.len() && is_headline_skippable_block(prelude_blocks[j], source)
            {
                j += 1;
            }
            if let Some(extra) = prelude_blocks.get(j) {
                extend_rows_inclusive(covered, *extra, source);
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

/// Blocks [`headline_spec`] steps over when looking for the lede: the
/// chrome of [`is_prelude_chrome_block`], plus admin/deprecation
/// warnings — real prose, but not the "what is this" sentence.
fn is_headline_skippable_block(block: Node, source: &str) -> bool {
    is_prelude_chrome_block(block, source)
        || (block.kind() == "paragraph" && is_admin_warning_paragraph(block, source))
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
/// `ReadmeHeadline` via [`is_headline_skippable_block`] (which adds
/// admin warnings) and [`prelude_remainder_rows`] directly.
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

/// True iff `heading`'s title text names an orientation/intro section
/// (`Introduction` / `Overview` / `About` / …) — the subsection a
/// decorative-title README puts its "what is this" sentence under.
fn is_intro_section_title(heading: Node, source: &str) -> bool {
    let raw = &source[heading.start_byte()..heading.end_byte()];
    let text = raw
        .lines()
        .next()
        .unwrap_or("")
        .trim_start_matches('#')
        .trim()
        .to_ascii_lowercase();
    matches!(
        text.as_str(),
        "introduction" | "overview" | "about" | "summary" | "synopsis"
    ) || text.starts_with("what is")
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
    fn markdown_headingless_non_readme_uses_body_value() {
        let ranges = headingless_fallback_ranges(
            &PathBuf::from("LICENSE.md"),
            "license prose without a heading\n",
        );
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].kind, SectionKind::BodyBlock);
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
        build_headline_spans(&PathBuf::from("README.md"), source, &spec)
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

    #[test]
    fn markdown_dev_command_line_uses_first_token() {
        for line in [
            "npm test",
            "`$ pnpm install`",
            "> cargo test",
            "python3 -m pytest",
            "uv sync",
            "docker-compose up",
            "./scripts/check",
        ] {
            assert!(is_dev_command_line(line), "{line}");
        }

        for line in [
            "Install with npm after cloning.",
            "The `cargo test` command is useful.",
            "Run this in your shell.",
        ] {
            assert!(!is_dev_command_line(line), "{line}");
        }
    }

    #[test]
    fn markdown_dev_signal_lines_cover_python_configs() {
        for line in [
            "pyproject.toml",
            "setup.py",
            "requirements.txt",
            "Makefile",
            "Dockerfile",
        ] {
            assert!(is_dev_config_line(line), "{line}");
        }
        assert!(is_repo_path_line("src/package/module.py"));
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

    /// Above the count cap, the mega-doc fallback emits a truncated
    /// H1/H2 skeleton instead of suppressing the outline outright.
    #[test]
    fn markdown_outline_above_count_cap_truncates_to_h2_skeleton() {
        let mut src = String::from("# Big File\n\nIntro.\n\n");
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("## Section {i}\n\nbody.\n\n"));
        }
        let tree = parse(&src);
        let gates = derive_outline_gates(&PathBuf::from("README.md"), &tree, &src);
        assert!(gates.truncated, "count blowout must take the fallback");
        assert!(gates.emits, "the truncated skeleton must emit");
        assert_eq!(gates.rows.len(), MAX_OUTLINE_HEADINGS);
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
        let file = PathBuf::from(file);
        let gates = derive_outline_gates(&file, &tree, source);
        logical_sections(
            &file,
            &tree,
            source,
            &outlined_rows(gates.emits, &gates.rows),
            gates.truncated,
            is_readme(&file),
        )
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

    #[test]
    fn markdown_logical_sections_kinds_table() {
        let cases: &[(&str, &str, String, Vec<SectionKind>)] = &[
            // README with one H1 wrapping one H2 with two H3 children, body
            // large enough to clear `H2_SPLIT_BYTES`. Should return one
            // `Whole` (the H1-unwrap intro) plus two H3Child ranges. The H2
            // intro range is dropped (heading-only after the H2).
            (
                "markdown_h2_split_intro_plus_h3_subsections",
                "README.md",
                make_split_h2_source("# Title\n\nTagline.\n\n## Usage", 2, 6),
                vec![
                    SectionKind::Whole,   // H1-unwrap intro
                    SectionKind::H3Child, // ### Sub 0
                    SectionKind::H3Child, // ### Sub 1
                ],
            ),
            // Substantive intro body — H2 heading followed by a real paragraph
            // before the first H3 — must keep the Intro range.
            (
                "markdown_h2_intro_kept_when_body_substantive",
                "README.md",
                make_split_h2_source(
                    "# Title\n\nTagline.\n\n## Setup\n\n\
                      Real prose intro before any subheading.\n\
                      A second sentence makes it substantive.",
                    2,
                    6,
                ),
                vec![
                    SectionKind::Whole,
                    SectionKind::Intro,
                    SectionKind::H3Child,
                    SectionKind::H3Child,
                ],
            ),
            // H2 with only one H3 child stays a `Whole`. The split rule
            // requires ≥2 H3 children.
            (
                "markdown_h2_no_split_one_h3",
                "README.md",
                make_split_h2_source("# Title\n\nTagline.\n\n## Usage", 1, 6),
                vec![SectionKind::Whole, SectionKind::Whole],
            ),
            // Splittable shape but section bytes < `H2_SPLIT_BYTES` stays one
            // `Whole` range.
            (
                "markdown_h2_no_split_under_threshold",
                "README.md",
                make_split_h2_source("# Title\n\nT.\n\n## Usage", 2, 0),
                vec![SectionKind::Whole, SectionKind::Whole],
            ),
            // `CHANGELOG.md` shape with H3 children stays `Whole` — the
            // changelog index-decay needs a stable per-H2 mapping.
            (
                "markdown_h2_split_skipped_for_changelog",
                "CHANGELOG.md",
                make_split_h2_source("## v1.0", 2, 6),
                vec![SectionKind::Whole],
            ),
            // Doc page (non-README, non-changelog) with H3 children does
            // split. Gate is changelog-only, not readme-only.
            (
                "markdown_h2_split_non_readme_doc_page",
                "docs/setup.md",
                make_split_h2_source(
                    "## Setup\n\nIntro paragraph that is real prose.\n\
                      A second line so the intro body counts as substantive.",
                    2,
                    6,
                ),
                vec![
                    SectionKind::Intro,
                    SectionKind::H3Child,
                    SectionKind::H3Child,
                ],
            ),
        ];

        for (row_no, (case, file, src, expected)) in cases.iter().enumerate() {
            if *case == "markdown_h2_no_split_under_threshold" {
                assert!(
                    src.len() < 600,
                    "{case} row {}: test fixture must be under threshold; got {} bytes",
                    row_no + 1,
                    src.len()
                );
            }
            let ranges = sections(file, src);
            let kinds: Vec<SectionKind> = ranges.iter().map(|r| r.kind).collect();
            assert_eq!(
                kinds.as_slice(),
                expected.as_slice(),
                "{case} row {}: section kinds mismatch; ranges={ranges:?}",
                row_no + 1
            );
        }
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

    /// Mega-README (heading count past the outline cap): the truncated
    /// H2-skeleton outline still gates structural splits, so a
    /// splittable H2 expands instead of riding as one giant Whole
    /// larger than any schedule budget (lo's 112KB `## Spec`).
    #[test]
    fn markdown_h2_split_proceeds_under_truncated_outline() {
        // One splittable H2 (with two H3 children + filler), then enough
        // additional H2 stubs to push past `MAX_OUTLINE_HEADINGS`.
        let mut src = String::from("# Title\n\nTagline.\n\n");
        src.push_str(&make_split_h2_source("## Usage", 2, 6));
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("\n## Other {i}\n\nbody.\n"));
        }
        let ranges = sections("README.md", &src);
        let h3_children = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::H3Child)
            .count();
        assert!(
            h3_children >= 2,
            "mega-README splittable H2 must split under the truncated outline; got {ranges:?}",
        );
    }

    /// A mega-README reference H2 whose prelude is an intra-doc link
    /// index emits chained roster chunks at prose (family) boundaries.
    #[test]
    fn markdown_link_index_intro_chunks_into_rosters() {
        let mut src =
            String::from("# lo\n\nTagline.\n\n## Spec\n\nSupported helpers for slices:\n\n");
        for i in 0..30 {
            src.push_str(&format!("- [Helper{i}](#helper{i})\n"));
        }
        src.push_str("\nSupported helpers for maps:\n\n");
        for i in 30..50 {
            src.push_str(&format!("- [Helper{i}](#helper{i})\n"));
        }
        src.push('\n');
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("\n### Helper{i}\n\nDoes thing {i}.\n"));
        }
        // A second H2 so the truncated H2 skeleton clears the outline's
        // 2-row minimum (which gates the structural split).
        src.push_str("\n## License\n\nMIT.\n");
        let ranges = sections("README.md", &src);
        let rosters: Vec<&SectionRange> = ranges.iter().filter(|r| r.roster_entries > 0).collect();
        assert_eq!(
            rosters.len(),
            2,
            "expected one roster chunk per family; got {ranges:?}",
        );
        assert_eq!(rosters[0].roster_entries, 30);
        assert!(!rosters[0].chained_to_previous);
        assert_eq!(rosters[1].roster_entries, 20);
        assert!(rosters[1].chained_to_previous);
        assert!(rosters.iter().all(|r| r.reference_shaped));
    }

    #[test]
    fn markdown_density_signals_ignore_bare_fence_delimiters() {
        let src = [
            "```",
            "plain text",
            "```",
            "~~~",
            "more prose",
            "~~~",
            "```",
            "npm test",
            "```",
            "```",
            "cargo build",
            "```",
        ];
        // Prose fences contribute nothing; only the command fences do,
        // and one alone is below the dev-workflow signal minimum.
        assert!(!range_has_dev_workflow_signal(&src, 1, 6));
        assert!(!range_has_dev_workflow_signal(&src, 1, 9));
        assert!(range_has_dev_workflow_signal(&src, 1, 12));
    }

    #[test]
    fn markdown_in_fence_config_snippet_is_a_density_signal() {
        assert!(line_has_density_signal("\"plugins\": [\"svgo\"]", true));
        assert!(!line_has_density_signal("\"plugins\": [\"svgo\"]", false));
    }

    #[test]
    fn markdown_link_index_trailing_prose_folds_into_last_roster() {
        let mut src =
            String::from("# lo\n\nTagline.\n\n## Spec\n\nSupported helpers for slices:\n\n");
        for i in 0..30 {
            src.push_str(&format!("- [Helper{i}](#helper{i})\n"));
        }
        src.push_str("\nDeprecated aliases are listed in the wiki.\n");
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("\n### Helper{i}\n\nDoes thing {i}.\n"));
        }
        src.push_str("\n## License\n\nMIT.\n");
        let ranges = sections("README.md", &src);
        let rosters: Vec<&SectionRange> = ranges.iter().filter(|r| r.roster_entries > 0).collect();
        assert_eq!(
            rosters.len(),
            1,
            "prose-only tail must not become its own roster chunk; got {ranges:?}",
        );
        assert_eq!(rosters[0].roster_entries, 30);
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
    /// `<br>` html_block before the next H2. Must split into BodyBlock
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
        let body_blocks: Vec<&SectionRange> = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::BodyBlock)
            .collect();
        assert_eq!(
            body_blocks.len(),
            3,
            "expected 3 BodyBlock ranges; got {ranges:?}"
        );
        // Trailing `<br>` row sits after the last bullet's range.
        let br_row = src
            .lines()
            .position(|l| l.trim() == "<br>")
            .map(|i| i + 1)
            .expect("test source must contain <br>");
        assert!(
            body_blocks.iter().all(|b| b.end < br_row),
            "<br> must not be inside any body-block range"
        );
    }

    /// Real anyhow `## Details` text (rows 21-125 of fixture README,
    /// trailing `<br>` included) must produce one BodyBlock per real
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
        let body_blocks: Vec<&SectionRange> = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::BodyBlock)
            .collect();
        // anyhow's ## Details has 6 top-level bullets; expect each to survive.
        assert_eq!(
            body_blocks.len(),
            6,
            "expected 6 BodyBlock ranges from real anyhow ## Details; got {ranges:?}"
        );
    }

    /// A long list-only section splits into one body block per item.
    #[test]
    fn markdown_h2_body_block_split_short_bullets() {
        let prefix = "# Title\n\nTagline.\n\n## Features";
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
            "long bullet list must split into body blocks; got {ranges:?}"
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

    #[test]
    fn markdown_structural_operational_sections_join_canonical_class() {
        assert!(cell_starts_with_cli_flag("<code>--all</code>"));
        assert!(!looks_like_cli_synopsis_line("npm install --save package"));
        assert!(!looks_like_cli_synopsis_line("node app.js <input>"));
        assert!(looks_like_cli_synopsis_line(
            "project [OPTIONS] PATTERN [FILE...]"
        ));

        let table = "# Project\n\nTagline.\n\n## Reference\n\n\
                     Flag | Meaning\n\
                     ---|---\n\
                     `-a`, `--all` | all values\n\
                     `-q`, `--quiet` | quiet output\n\
                     `--color` | color mode\n";
        let ranges = sections("README.md", table);
        assert!(ranges.iter().any(|r| r.is_canonical_operational_section));

        let synopsis = "# Project\n\nTagline.\n\n## Invocation\n\n\
                        ```text\nproject [OPTIONS] PATTERN [FILE...]\n```\n";
        let ranges = sections("README.md", synopsis);
        assert!(ranges.iter().any(|r| r.is_canonical_operational_section));

        let source_code = "# Project\n\nTagline.\n\n## Internals\n\n\
                           ```js\nconst value = call();\n```\n";
        let ranges = sections("README.md", source_code);
        assert!(ranges.iter().all(|r| !r.is_canonical_operational_section));

        let mechanics = make_split_h2_source("# Project\n\nTagline.\n\n## How Project Works", 2, 3);
        let ranges = sections("README.md", &mechanics);
        let children: Vec<&SectionRange> = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::H3Child)
            .collect();
        assert!(
            children.len() >= 2,
            "expected split mechanics children: {ranges:?}"
        );
        assert!(
            children.iter().all(|r| r.is_canonical_operational_section),
            "mechanics children must inherit the structural class: {ranges:?}",
        );

        let prose = "# Project\n\nTagline.\n\n## How Project Works\n\nOne plain paragraph.\n";
        let ranges = sections("README.md", prose);
        assert!(ranges.iter().all(|r| !r.is_canonical_operational_section));
    }

    /// Trailing decorative HTML block after the list (anyhow's
    /// `<br>` shape, or an image-only `<p><img/></p>`) is tolerated
    /// by the predicate and falls outside every emitted range. This
    /// is the same trade-off `ReadmeHeadline` and the rest of the
    /// markdown walker make for image-only / badge-only content:
    /// there's no semantic value to preserve, so dropping it is
    /// fine.
    #[test]
    fn markdown_h2_split_bullets_trailing_decorative_html_block() {
        let prefix = "# Title\n\nTagline.\n\n## Details";
        let mut src = make_bullet_section(prefix, 3, 4);
        src.push_str(
            "\n<p align=\"center\"><img src=\"./assets/footer.png\"/></p>\n\n## Next\n\nbody.\n",
        );
        let ranges = sections("README.md", &src);
        let body_blocks: Vec<&SectionRange> = ranges
            .iter()
            .filter(|r| r.kind == SectionKind::BodyBlock)
            .collect();
        assert_eq!(
            body_blocks.len(),
            3,
            "trailing decorative html_block must not block the body-block split; got {ranges:?}"
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

    /// Section has prose paragraph then list — list-only predicate
    /// rejects because the paragraph is a non-decorative non-list
    /// child, so no body-block split fires.
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
            !ranges.iter().any(|r| r.kind == SectionKind::BodyBlock),
            "prose-then-list section must not body-block-split; got {ranges:?}"
        );
    }

    /// Changelog file class is gated out of body-block split — index-
    /// decay needs a stable per-H2 mapping.
    #[test]
    fn markdown_h2_no_split_bullets_changelog() {
        let src = make_bullet_section("## v1.0", 3, 4);
        let ranges = sections("CHANGELOG.md", &src);
        assert!(
            !ranges.iter().any(|r| r.kind == SectionKind::BodyBlock),
            "changelog must not split; got {ranges:?}"
        );
    }

    /// Outline-omitted file still uses the body-block fallback for
    /// list-only H2 sections.
    #[test]
    fn markdown_h2_body_block_bullets_when_outline_omitted() {
        let mut src = String::from("# Title\n\nTagline.\n\n");
        src.push_str(&make_bullet_section("## Details", 3, 4));
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            src.push_str(&format!("\n## Other {i}\n\nbody.\n"));
        }
        let tree = parse(&src);
        let gates = derive_outline_gates(&PathBuf::from("README.md"), &tree, &src);
        assert!(
            gates.truncated,
            "outline must be count-capped for the test premise to hold"
        );
        let ranges = sections("README.md", &src);
        assert!(
            ranges.iter().any(|r| r.kind == SectionKind::BodyBlock),
            "outline-capped long list should fall back to body blocks; got {ranges:?}",
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
        assert_eq!(readme_index_decay(&ranges[0]), 1.0);
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

    /// Adjacent top-level bullets with nested sub-bullets must split
    /// into line-disjoint per-item sections. tree-sitter-markdown list
    /// items swallow the next sibling's leading indentation, so a
    /// newline-only end trim let item N's range claim item N+1's first
    /// row — sibling Section batches then hit the scheduler's
    /// non-ancestor overlap panic (tinyusb SEGGER_RTT README at 1M).
    #[test]
    fn walker_markdown_adjacent_nested_bullets_split_disjoint() {
        let pad = "x".repeat(60);
        let mut src =
            String::from("Title\n=====\n\nIntro paragraph prose.\n\n## Included files\n\n");
        for name in ["alpha", "beta", "gamma", "delta"] {
            src.push_str(&format!("  * `{name}/`\n"));
            src.push_str(&format!("    * `{name}.c` - {pad}\n"));
            src.push_str(&format!("    * `{name}.h` - {pad}\n"));
        }
        let tree = parse(&src);
        let file = PathBuf::from("/x/README.md");
        let gates = derive_outline_gates(&file, &tree, &src);
        let ranges = logical_sections(
            &file,
            &tree,
            &src,
            &outlined_rows(gates.emits, &gates.rows),
            gates.truncated,
            false,
        );
        let items: Vec<(usize, usize)> = ranges
            .iter()
            .filter(|r| matches!(r.kind, SectionKind::BodyBlock))
            .map(|r| (r.start, r.end))
            .collect();
        assert!(
            items.len() >= 4,
            "expected per-bullet split, got {ranges:?}"
        );
        for pair in items.windows(2) {
            assert!(
                pair[0].1 < pair[1].0,
                "sibling item ranges overlap: {items:?}"
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
        let sections = rst_body_sections(src);
        assert_eq!(sections.len(), 2, "Overview + Details");
        for section in &sections {
            let own_start = section.rows.first().copied();
            for &row in &section.rows {
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

    /// A single-heading list doc splits into per-item body blocks with no
    /// outline behind it (the outline needs two heading rows to emit), so
    /// nothing else can carry the section's own title. Without an owner
    /// that row is unrenderable at every budget, 1M included.
    #[test]
    fn markdown_list_split_without_an_outline_keeps_the_heading_row() {
        let mut src = String::from("## Supported functions\n\n");
        for i in 0..12 {
            src.push_str(&format!(
                "- `helper{i}(input)` — returns the transformed input for case {i}.\n"
            ));
        }
        assert!(
            src.len() >= H2_SPLIT_BYTES,
            "shape must clear the body-block split gate"
        );
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("guide.md"), &src).unwrap();

        let scheduler = Scheduler::new(dir.path().to_path_buf(), FsWalker, 100_000, None);
        let rendered = scheduler.run().render();
        assert!(
            rendered.contains("helper0(input)"),
            "expected the per-item split to render: {rendered}"
        );
        assert!(
            rendered.contains("Supported functions"),
            "section title has no owner at any budget: {rendered}"
        );
    }

    /// The mega-doc fallback outline is an H2-only skeleton, so it can
    /// never name an H3. An H3 large enough to split into body blocks is
    /// then the only possible owner of its own heading row.
    #[test]
    fn markdown_h3_body_block_split_keeps_its_heading_row_under_truncation() {
        let mut src =
            String::from("# lo\n\nTagline.\n\n## Spec\n\nSupported helpers for slices:\n\n");
        for i in 0..30 {
            src.push_str(&format!("- [Helper{i}](#helper{i})\n"));
        }
        src.push('\n');
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            // Prose + fence, then a second prose block: two body blocks,
            // with the H3 past `BODY_BLOCK_SPLIT_BYTES`.
            let h3 = format!(
                "\n### Helper{i}\n\nDoes thing {i} to every element of the input slice, \
                 preserving the original order of the elements it keeps and \
                 allocating exactly once.\n\n\
                 ```go\nresult := lo.Helper{i}(input, predicate)\n```\n\n\
                 Returns a new slice; the input is never mutated, the zero value \
                 is returned when the input is empty, and a nil predicate panics \
                 rather than silently matching everything.\n"
            );
            assert!(
                h3.len() >= BODY_BLOCK_SPLIT_BYTES,
                "each H3 must clear the body-block split gate"
            );
            src.push_str(&h3);
        }
        src.push_str("\n## License\n\nMIT.\n");
        let ranges = sections("README.md", &src);
        assert!(
            ranges.iter().any(|r| r.kind == SectionKind::BodyBlock),
            "expected the H3 body-block split; got {ranges:?}"
        );
        for (idx, line) in src.lines().enumerate() {
            if !line.starts_with("### ") {
                continue;
            }
            let row = idx + 1;
            assert!(
                ranges.iter().any(|r| r.start <= row && row <= r.end),
                "H3 heading row {row} ({line}) has no owner; the skeleton outline names H2s only"
            );
        }
    }

    /// Same skeleton, one path over: an H3 with nothing under its title
    /// is normally left to the outline, but the H2-only skeleton never
    /// names an H3 — so the bare heading needs a batch of its own.
    #[test]
    fn markdown_empty_h3_keeps_its_heading_row_under_truncation() {
        // The link index is what keeps the mega-doc fallback alive past
        // `expand_in_dir`'s roster check, so the skeleton outline (and
        // the split it gates) survives to reach the H3 children.
        let mut src =
            String::from("# lo\n\nTagline.\n\n## Spec\n\nSupported helpers for slices:\n\n");
        for i in 0..30 {
            src.push_str(&format!("- [Helper{i}](#helper{i})\n"));
        }
        src.push('\n');
        for i in 0..(MAX_OUTLINE_HEADINGS + 5) {
            // Every H3 documented but one: the bare entry is the shape a
            // generated index produces for an unannotated symbol.
            if i == 3 {
                src.push_str("\n### Undocumented\n");
                continue;
            }
            src.push_str(&format!(
                "\n### Helper{i}\n\nDoes thing {i} to every element of the input slice, \
                 preserving the original order of the elements it keeps.\n"
            ));
        }
        src.push_str("\n## License\n\nMIT.\n");
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("README.md"), &src).unwrap();

        let scheduler = Scheduler::new(dir.path().to_path_buf(), FsWalker, 1_000_000, None);
        let rendered = scheduler.run().render();
        assert!(
            rendered.contains("### Helper0"),
            "expected the H3 split to render documented children: {rendered}"
        );
        assert!(
            rendered.contains("### Undocumented"),
            "empty H3's heading row has no owner at any budget: {rendered}"
        );
    }
}
