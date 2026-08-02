//! Go walker. Per-file package-and-imports plus per-decl batches, plus
//! a `_test.go` test-name surface and a `go.mod` whole-file batch.
//!
//! Per-file keys:
//! - `PackageImports { file }`: `package …` clause + `import (…)` block.
//!   Plumbing batch. Predecessor: the file's `DeclNames` head chunk when
//!   the file declares anything — imports refine a summarized file, they
//!   don't open an unsummarized one.
//! - `DeclNames { file }`: surface listing of every top-level
//!   declaration's first line — visibility-blind catastrophic-omission
//!   hedge. Grouped `type ( … )` / `var ( … )` / `const ( … )` blocks
//!   contribute one entry per inner spec, so the hedge surfaces every
//!   exported name even when the whole-group `Decl` batch isn't
//!   scheduled. A decl's `Deprecated:` doc line rides here rather than
//!   in `DeclDoc`: the roster is often the only place a decl appears,
//!   and listing a deprecated decl among live siblings without its
//!   marker steers the reader onto the API the package disowned.
//! - `TestNames { file }`: in `_test.go` files only, surface listing of
//!   `Test*` / `Benchmark*` / `Example*` first lines (Go's `go test`
//!   lookup contract). No bodies / docs from test files.
//! - `GoModIdentity { file }`: small (~25–40 tok) identity slice of
//!   `go.mod` / `go.work` covering the `module`, `go`, and `toolchain`
//!   directive lines. Predecessor of the whole-file `GoMod` batch so
//!   the cheap lede can land first at small budgets.
//! - `GoMod { file }`: line-set batch for `go.mod` / `go.work`.
//!   Small module files are kept whole. Large files retain identity
//!   directives, direct `require` rows, and `replace` / `exclude` /
//!   `retract` / `use` directives, with indirect-only tails elided.
//!   Predecessor: matching `GoModIdentity` (identity lines are an
//!   ancestor subset).
//!
//! Per-decl keys (keyed by start line):
//! - `Decl { file, start_line }`: one top-level declaration's
//!   signature/header. For function and method definitions, signature
//!   with body marker. For type / var / const, the whole declaration
//!   including any grouped `( … )` block — splitting per-spec would
//!   lose iota / inherited-type / shared-comment semantics.
//! - `DeclDoc { file, start_line }`: doc comment block immediately above
//!   a decl. Predecessor: matching `Decl`.
//! - `DeclBody { file, start_line }`: body interior of a function or
//!   method definition. Predecessor: matching `Decl`.
//!
//! Visibility: emits **all** top-level decls. A `visibility_factor`
//! (1.0 exported, 0.6 unexported) discounts unexported decls'
//! catastrophic axis so exported anchors win on ratio. Hard-filtering
//! lowercase decls would miss NS-load-bearing internal anchors that
//! the fixture NSes call out (`go-multierror`'s `chain`, `tock`'s
//! `repository` / `twInterval` / `timeLayout`). For grouped
//! declarations, the group is exported iff any inner spec is exported.
//!
//! Parse trees are cached in [`WalkCtx`].

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, GoKey};
use crate::content::BatchContent;
use crate::value::{mix_signals, names_surface_chunk_factor};

use super::{
    FileLines, WalkCtx, collect_blank_line_groups, collect_doc_comments_above, comment_only_rows,
    dedup_sorted, extend_nonblank_rows, extend_span, file_depth_factor, file_lines_covered_by,
    fs::{files_with_extension, list_dir},
    push_rows, signature_end_row, single_file_lines_content,
};

const VISIBILITY_FACTOR_EXPORTED: f64 = 1.0;
const VISIBILITY_FACTOR_UNEXPORTED: f64 = 0.6;
const GOMOD_WHOLE_LINE_CAP: usize = 72;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    out.extend(expand_gomod(dir, ctx));
    let (test_files, source_files): (Vec<_>, Vec<_>) = files_with_extension(dir, "go", ctx)
        .into_iter()
        .partition(|p| is_test_file(p));
    out.extend(expand_test_files(&test_files, ctx));
    out.extend(expand_source_files(&source_files, ctx));
    out
}

fn expand_gomod(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for (name, kind) in list_dir(dir, ctx.dir_filter()) {
        if !matches!(kind, crate::fs_util::EntryKind::File) {
            continue;
        }
        if !matches!(name.as_str(), "go.mod" | "go.work") {
            continue;
        }
        let path = dir.join(name);
        let identity_emitted = if let Some(content) = build_gomod_identity_content(&path, ctx) {
            out.push(Batch {
                key: GoKey::GoModIdentity { file: path.clone() }.into(),
                predecessor: None,
                content,
                value: gomod_identity_value(&path, ctx),
            });
            true
        } else {
            false
        };
        let Some(content) = build_gomod_content(&path, ctx) else {
            continue;
        };
        // Only declare the identity batch as predecessor when it was
        // actually emitted — otherwise the GoMod batch would orphan
        // itself on a never-resolved predecessor key.
        let predecessor =
            identity_emitted.then(|| BatchKey::Go(GoKey::GoModIdentity { file: path.clone() }));
        out.push(Batch {
            key: GoKey::GoMod { file: path.clone() }.into(),
            predecessor,
            content,
            value: gomod_value(&path, ctx),
        });
    }
    out
}

/// Identity slice of `go.mod` / `go.work` — `module`/`go`/`toolchain`
/// directives only; block bodies are skipped.
fn build_gomod_identity_content(file: &Path, ctx: &WalkCtx) -> Option<BatchContent> {
    let source = ctx.read_source(file)?;
    let mut lines = Vec::new();
    let mut in_block = false;
    for (i, raw) in source.lines().enumerate() {
        let trimmed = raw.trim();
        // Skip block bodies (`require ( … )`, etc.) so block entries
        // can't shadow identity keywords; go.mod allows directives in
        // any order, so module/go/toolchain are collected wherever
        // they appear at top level.
        if trimmed.ends_with('(') && !trimmed.starts_with("//") {
            in_block = true;
            continue;
        }
        if in_block {
            if trimmed == ")" {
                in_block = false;
            }
            continue;
        }
        let first = trimmed.split_whitespace().next().unwrap_or("");
        if matches!(first, "module" | "go" | "toolchain") {
            lines.push(i + 1);
        }
    }
    if lines.is_empty() {
        return None;
    }
    single_file_lines_content(file, &source, FileLines::new(lines))
}

/// `GoMod` content — whole for compact module files; sampled for large
/// generated dependency closures.
fn build_gomod_content(file: &Path, ctx: &WalkCtx) -> Option<BatchContent> {
    let source = ctx.read_source(file)?;
    let total_lines = source.lines().count();
    if total_lines == 0 {
        return None;
    }
    if total_lines <= GOMOD_WHOLE_LINE_CAP {
        return single_file_lines_content(
            file,
            &source,
            FileLines::new((1..=total_lines).collect()),
        );
    }

    single_file_lines_content(file, &source, bounded_gomod_lines(&source))
}

fn bounded_gomod_lines(source: &str) -> FileLines {
    let src_lines: Vec<&str> = source.lines().collect();
    let mut full = Vec::new();
    let mut i = 0;
    while i < src_lines.len() {
        let line_no = i + 1;
        let trimmed = src_lines[i].trim();
        if let Some(block) = gomod_block_start(trimmed) {
            let start_line = line_no;
            let mut body = Vec::new();
            i += 1;
            while i < src_lines.len() && src_lines[i].trim() != ")" {
                if keep_gomod_block_entry(block, src_lines[i].trim()) {
                    body.push(i + 1);
                }
                i += 1;
            }
            let close_line = (i < src_lines.len() && src_lines[i].trim() == ")").then_some(i + 1);
            if !body.is_empty() {
                full.push(start_line);
                full.extend(body);
                if let Some(close_line) = close_line {
                    full.push(close_line);
                }
            }
            if close_line.is_some() {
                i += 1;
            }
            continue;
        }
        if keep_gomod_directive_line(trimmed) {
            full.push(line_no);
        }
        i += 1;
    }
    full.sort_unstable();
    full.dedup();
    let ellipses = gomod_ellipses_for_gaps(&full, &src_lines);
    FileLines::new(full).with_ellipses(ellipses)
}

fn gomod_ellipses_for_gaps(full: &[usize], src_lines: &[&str]) -> Vec<usize> {
    if full.is_empty() {
        return Vec::new();
    }
    let mut boundaries = full.to_vec();
    boundaries.push(src_lines.len() + 1);
    let mut ellipses = Vec::new();
    for pair in boundaries.windows(2) {
        let omitted_start = pair[0] + 1;
        let omitted_end = pair[1].saturating_sub(1);
        if omitted_start > omitted_end {
            continue;
        }
        let has_substantive_omission = (omitted_start..=omitted_end).any(|line_no| {
            src_lines
                .get(line_no - 1)
                .is_some_and(|line| is_substantive_gomod_line(line.trim()))
        });
        if has_substantive_omission {
            ellipses.push(omitted_start);
        }
    }
    ellipses
}

fn is_substantive_gomod_line(trimmed: &str) -> bool {
    !trimmed.is_empty() && !trimmed.starts_with("//") && trimmed != ")"
}

fn gomod_block_start(trimmed: &str) -> Option<&str> {
    let (first, rest) = trimmed.split_once(char::is_whitespace)?;
    (rest.trim() == "(" && matches!(first, "require" | "replace" | "exclude" | "retract" | "use"))
        .then_some(first)
}

fn keep_gomod_directive_line(trimmed: &str) -> bool {
    let Some(first) = trimmed.split_whitespace().next() else {
        return false;
    };
    matches!(
        first,
        "module" | "go" | "toolchain" | "replace" | "exclude" | "retract" | "use"
    ) || (first == "require" && !trimmed.contains("// indirect"))
}

fn keep_gomod_block_entry(block: &str, trimmed: &str) -> bool {
    if trimmed.is_empty() || trimmed.starts_with("//") {
        return false;
    }
    block != "require" || !trimmed.contains("// indirect")
}

fn expand_test_files(test_files: &[PathBuf], ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in test_files {
        let Some((source, tree)) = parse_go(ctx, file) else {
            continue;
        };
        let starts = collect_test_function_lines(&tree, &source);
        if starts.is_empty() {
            continue;
        }
        let ellipses = starts.iter().map(|&l| l + 1).collect();
        let lines = FileLines::new(starts).with_ellipses(ellipses);
        let Some(content) = single_file_lines_content(file, &source, lines) else {
            continue;
        };
        out.push(Batch {
            key: GoKey::TestNames { file: file.clone() }.into(),
            predecessor: None,
            content,
            value: test_names_value(file, ctx),
        });
    }
    out
}

fn expand_source_files(source_files: &[PathBuf], ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    let spine = spine_file(source_files, ctx);
    for file in source_files {
        let Some((source, tree)) = parse_go(ctx, file) else {
            continue;
        };
        let src_lines: Vec<&str> = source.lines().collect();
        let line_count = src_lines.len();
        let pkg = package_name(&tree, &source);
        let decls = find_decls(&tree, &source);
        // Computed once per file and threaded through value functions
        // — the previous design recomputed inside each value call,
        // re-walking the tree per decl.
        let entry_factor = go_entry_factor_for(file, ctx, pkg.as_deref(), &decls);
        // Exportedness is meaningless in `package main` — nothing can
        // import it, and NS authors rank entrypoint internals (flag
        // tables, `func main` wiring) without a visibility discount.
        //
        // Tried and reverted alongside this: a `cmd/`-subtree surface
        // boost (1.4× and 1.2×, names/decl/struct-group roles only).
        // act's NS anchors its cmd/ files (+0.037 at 1.4×) but
        // mcphost's equally-conventional cmd/ subcommand files are
        // NS-peripheral (−0.087 at 1.4×, −0.032 at 1.2× with act's
        // gain gone) — same layout, same cobra idioms, no walk-time
        // signal separating them.
        let package_main = pkg.as_deref() == Some("main");

        // `PackageDocLede` fires for entry-shaped files (see
        // `go_entry_factor_for`) — internal subpackage ledes carry low
        // orientation value relative to cost — *or* for a dedicated
        // `doc.go` package-documentation file. A `doc.go` has no decls
        // and a non-eponymous stem, so the entry-factor gate alone
        // would skip it even though its godoc lede IS the package's
        // identity sentence. The lede ships first-paragraph-only; the
        // truncated remainder of a `/* */` godoc (conventionally the
        // package's hello-world example — a canonical-usage demo)
        // ships as a separate gated batch so it is at least
        // purchasable rather than absent from the pool.
        if entry_factor > 1.0 || is_package_doc_file(file, &decls) {
            let (lede_lines, body_lines) = collect_package_doc_parts(&tree, &source);
            let lede_emitted =
                if let Some(content) = single_file_lines_content(file, &source, lede_lines) {
                    out.push(Batch {
                        key: GoKey::PackageDocLede { file: file.clone() }.into(),
                        predecessor: None,
                        content,
                        value: GoRole::PackageDocLede.value(file, ctx, entry_factor, 1.0),
                    });
                    true
                } else {
                    false
                };
            if lede_emitted
                && let Some(content) = single_file_lines_content(file, &source, body_lines)
            {
                out.push(Batch {
                    key: GoKey::PackageDocBody { file: file.clone() }.into(),
                    predecessor: Some(BatchKey::Go(GoKey::PackageDocLede { file: file.clone() })),
                    content,
                    value: GoRole::PackageDocBody.value(file, ctx, entry_factor, 1.0),
                });
            }
        }
        let all_name_lines: HashSet<usize> = decls
            .iter()
            .flat_map(|(_, info)| {
                info.name_lines
                    .iter()
                    .copied()
                    .chain(info.deprecation_marker)
            })
            .collect();
        let row_claimants = decl_row_claimants(&decls, &source, &src_lines);
        let roster = RosterCtx {
            file,
            ctx,
            source: &source,
            decls: &decls,
            all_name_lines: &all_name_lines,
            co_location: co_location_representatives(decls.len(), &row_claimants),
            row_claimants: &row_claimants,
        };
        let names_groups = decl_names_chunk_groups(
            &roster,
            line_count,
            spine.as_deref() == Some(file.as_path()),
        );
        let chunk_count = names_groups.len();
        let mut chunk_of_decl = vec![0usize; decls.len()];
        for (chunk_index, group) in names_groups.iter().enumerate() {
            for &decl_index in &group.decls {
                chunk_of_decl[decl_index] = chunk_index;
            }
        }
        let names_keys: Vec<GoKey> = (0..chunk_count)
            .map(|chunk_index| GoKey::DeclNames {
                file: file.clone(),
                chunk_index,
            })
            .collect();
        let mut names_lines_by_chunk: Vec<FileLines> = names_groups
            .iter()
            .map(|group| roster.lines(&group.decls))
            .collect();
        // A `//go:build` constraint negates the roster's claim that
        // these declarations are the package's API: they exist only for
        // the builds it names. It sits above the `package` clause, in no
        // batch at all, so the roster — routinely the only place these
        // declarations appear — presented conditional code as universal.
        // The head chunk is the right anchor for the same reason the
        // `Deprecated:` marker rides here: it is the file's first
        // admitted content and the predecessor of everything else in the
        // file, so the constraint cannot be separated from what it
        // qualifies.
        let constraint_rows = build_constraint_rows(&src_lines);
        if let Some(head) = names_lines_by_chunk.first_mut()
            && !head.full.is_empty()
            && !constraint_rows.is_empty()
        {
            head.full = dedup_sorted(head.full.iter().copied().chain(constraint_rows).collect());
        }
        let mut names_head_emitted = false;
        for (chunk_index, names_lines) in names_lines_by_chunk.iter().enumerate() {
            if let Some(content) = single_file_lines_content(file, &source, names_lines.clone()) {
                names_head_emitted = names_head_emitted || chunk_index == 0;
                out.push(Batch {
                    key: names_keys[chunk_index].clone().into(),
                    predecessor: None,
                    content,
                    value: GoRole::DeclNames.value(file, ctx, entry_factor, 1.0)
                        * names_groups[chunk_index].value_factor,
                });
            }
        }
        // The import block is a refinement of the file's declaration
        // roster, not an entry point into the file. Ungated it is the
        // cheapest batch any source file offers, so the first token ever
        // spent on a file buys `package X` + a list of dependencies —
        // which renders as a confident "examined, it's plumbing" where a
        // bare filename honestly rendered "not covered". Gating behind
        // the roster keeps imports purchasable (they really do carry the
        // dependency surface) while making a declaration listing the
        // file's first admitted content. Files with no declarations have
        // no roster to gate on and stay ungated.
        if let Some(content) =
            single_file_lines_content(file, &source, collect_package_imports(&tree, &source))
        {
            out.push(Batch {
                key: GoKey::PackageImports { file: file.clone() }.into(),
                predecessor: names_head_emitted.then(|| BatchKey::Go(names_keys[0].clone())),
                content,
                value: GoRole::PackageImports.value(file, ctx, entry_factor, 1.0),
            });
        }
        for (decl_index, (node, info)) in decls.iter().enumerate() {
            // Gate each decl's train on the chunk that OWNS its name
            // line, not on a positional chunk: the entry slice is
            // rank-chosen, so a decl's owner is not a function of its
            // source position, and gating on a non-owner would make the
            // decl schedulable while a non-ancestor chunk still holds
            // its roster row.
            let chunk_index = chunk_of_decl[decl_index];
            let names_predecessor = BatchKey::Go(names_keys[chunk_index].clone());
            let chunk_names_lines = &names_lines_by_chunk[chunk_index];
            let decl_key = GoKey::Decl {
                file: file.clone(),
                start_line: info.start_line,
            };
            let decl_lines = collect_decl(info);
            // The marker line belongs to the names surface; dropping it
            // here keeps the two batches' line sets disjoint.
            let mut doc_lines = collect_doc_comments_above(*node, &source);
            doc_lines
                .full
                .retain(|l| Some(*l) != info.deprecation_marker);
            let body_lines = if info.kind.has_body() {
                collect_decl_body(info, &src_lines)
            } else {
                FileLines::new(Vec::new())
            };
            let decl_has_descendants = !doc_lines.full.is_empty() || !body_lines.full.is_empty();
            if (!file_lines_covered_by(&decl_lines, chunk_names_lines) || decl_has_descendants)
                && let Some(content) = single_file_lines_content(file, &source, decl_lines)
            {
                out.push(Batch {
                    key: decl_key.clone().into(),
                    predecessor: Some(names_predecessor.clone()),
                    content,
                    value: GoRole::Decl.value(file, ctx, entry_factor, info.kv(package_main)),
                });
            }
            let decl_predecessor = BatchKey::Go(decl_key);
            if let Some(content) = single_file_lines_content(file, &source, doc_lines) {
                out.push(Batch {
                    key: GoKey::DeclDoc {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: GoRole::DeclDoc.value(file, ctx, entry_factor, info.kv(package_main)),
                });
            }
            if info.kind.has_body()
                && let Some(content) = single_file_lines_content(file, &source, body_lines)
            {
                out.push(Batch {
                    key: GoKey::DeclBody {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: GoRole::DeclBody.value(file, ctx, entry_factor, info.kv(package_main)),
                });
            }
            // Big-struct field-group split. Each blank-line-separated
            // group inside `type X struct { … }` fires independently
            // gated on the parent `Decl`; the parent Decl's own span
            // was trimmed in `grouped_type_info` to the type-header /
            // closer rows so the FieldGroup spans don't overlap.
            for (group_start_line, rows) in &info.struct_field_groups {
                let lines = FileLines::new(rows.clone());
                if let Some(content) = single_file_lines_content(file, &source, lines) {
                    out.push(Batch {
                        key: GoKey::StructFieldGroup {
                            file: file.clone(),
                            start_line: info.start_line,
                            group_start_line: *group_start_line,
                        }
                        .into(),
                        predecessor: Some(decl_predecessor.clone()),
                        content,
                        value: GoRole::StructFieldGroup.value(
                            file,
                            ctx,
                            entry_factor,
                            info.kv(package_main),
                        ),
                    });
                }
            }
        }
    }
    out
}

/// Chunk size for Go decl-name surfaces. Smaller than Python/TS's 12
/// because method-decl rows render long (`func (c *Command) Foo(...)`).
const GO_DECL_NAMES_CHUNK_SIZE: usize = 8;

const GO_DECL_NAMES_CHUNK_THRESHOLD: usize = 30;

/// Co-gate — only chunk when the file's full names surface plausibly
/// won't fit at 3K.
const GO_DECL_NAMES_CHUNK_LINE_THRESHOLD: usize = 800;

/// A names surface costing more than this gets an entry slice split off
/// its head. See [`decl_names_chunk_groups`].
const GO_ENTRY_SLICE_SPLIT_TOKENS: usize = 250;

/// Ceiling on the entry slice's decl count.
const GO_ENTRY_SLICE_DECLS: usize = 8;

/// Ceiling on the entry slice's rendered cost — the knob that actually
/// controls whether the file enters, since the gate's rank is
/// `value / cost^k`.
const GO_ENTRY_SLICE_TOKENS: usize = 80;

/// The gate slice's share of the file's roster value. Held just under
/// an unsplit roster's so a small file whose whole surface lands in one
/// batch still wins a comparable rank race.
const GO_ENTRY_SLICE_VALUE_FACTOR: f64 = 0.9;

/// Rank premium for a declaration carrying a godoc comment. Below the
/// gap between the exported and unexported [`visibility_factor`]s on
/// purpose: Go's capitalization rule is the stronger statement about
/// what the package offers outward, and a doc comment only orders
/// declarations *within* a visibility class.
const GO_ENTRY_SLICE_DOC_PREMIUM: f64 = 1.15;

/// Within-file prominence rank used to choose the entry slice. Only
/// signals the language itself defines are used: the exported-name
/// rule, the presence of a godoc comment, and the declaration kind.
///
/// Unlike [`DeclInfo::kv`], the visibility discount is **not** waived in
/// `package main`. `kv` waives it because nothing can import a `main`
/// package, so exportedness carries no cross-package signal; but this
/// rank orders declarations against each other inside one file, where a
/// capitalized name is still the author's own marking of the file's
/// outward-facing surface (`func Execute` against its unexported
/// helpers).
fn entry_slice_rank(info: &DeclInfo, has_doc: bool) -> f64 {
    let doc = if has_doc {
        GO_ENTRY_SLICE_DOC_PREMIUM
    } else {
        1.0
    };
    info.kind.kind_weight() * info.visibility_factor() * doc
}

/// Partition a file's declarations into names-surface chunks, group 0
/// being the file's **entry unit** — the batch every other batch in the
/// file gates on, and therefore the price of admission to the file's
/// whole train.
///
/// A fat roster makes that price ruinous. Roster value is near
/// size-invariant while cost grows with the declaration count, so within
/// one file class the ratio decays and the *spine* file — the one with
/// the most declarations, and the one an NS author ranks first — is the
/// last to be admitted, if ever. Splitting a small, rank-chosen gate
/// slice off the head buys entry at a fraction of the cost while the
/// remainder keeps carrying the catastrophic-omission hedge.
///
/// The slice must be rank-chosen rather than source-ordered: a
/// source-order head is an arbitrary prefix of the API, so it neither
/// names what the file is for nor completes any roster an NS asks for.
/// Ranking by [`entry_slice_rank`] puts the exported, documented
/// declarations in the gate.
///
/// Below [`GO_ENTRY_SLICE_SPLIT_TOKENS`] no split happens: an entry unit
/// that already fits is its own best gate, and slicing it would only
/// strand the remainder. Nor does it happen off the directory's spine
/// file ([`spine_file`]) — a cheap gate admits the whole file's depth
/// train, so handing one to every fat file in a package trades the
/// package's breadth for a scattering of half-read files.
fn decl_names_chunk_groups(
    roster: &RosterCtx,
    line_count: usize,
    is_spine: bool,
) -> Vec<NamesChunk> {
    let decls = roster.decls;
    if decls.is_empty() {
        return Vec::new();
    }
    let source_order_chunks = |indices: &[usize]| -> Vec<Vec<usize>> {
        if decls.len() > GO_DECL_NAMES_CHUNK_THRESHOLD
            && line_count > GO_DECL_NAMES_CHUNK_LINE_THRESHOLD
        {
            indices
                .chunks(GO_DECL_NAMES_CHUNK_SIZE)
                .map(<[usize]>::to_vec)
                .collect()
        } else {
            vec![indices.to_vec()]
        }
    };
    let priced = |groups: Vec<Vec<usize>>| -> Vec<NamesChunk> {
        let count = groups.len();
        groups
            .into_iter()
            .enumerate()
            .map(|(chunk_index, decls)| NamesChunk {
                decls,
                value_factor: names_surface_chunk_factor(chunk_index, count),
            })
            .collect()
    };
    let all: Vec<usize> = (0..decls.len()).collect();
    let legacy = source_order_chunks(&all);
    // Only a roster the chunker left whole gets a gate slice. Where the
    // surface is already chunked the head chunk *is* an entry slice, one
    // the chunker sized; carving a second, smaller gate out of it only
    // lengthens a train that already opens cheaply, and it displaces the
    // source-order roster halves an NS asks for as units.
    if !is_spine || legacy.len() > 1 || roster.cost(&legacy[0]) <= GO_ENTRY_SLICE_SPLIT_TOKENS {
        return priced(legacy);
    }
    let Some(entry) = entry_slice(roster) else {
        return priced(legacy);
    };
    let remainder: Vec<usize> = all
        .iter()
        .copied()
        .filter(|index| !entry.contains(index))
        .collect();
    // The remainder keeps the factors it would have carried had no slice
    // been taken. Re-indexing it behind the gate would demote the
    // catalog the gate was supposed to make reachable: on a file whose
    // roster *did* fit, the split would then buy a cheap gate at the
    // price of pushing the complete roster out of budget, which is a
    // strictly worse trade than not splitting at all.
    let legacy_count = legacy.len();
    let mut groups = vec![NamesChunk {
        decls: entry,
        value_factor: GO_ENTRY_SLICE_VALUE_FACTOR,
    }];
    groups.extend(source_order_chunks(&remainder).into_iter().enumerate().map(
        |(chunk_index, decls)| NamesChunk {
            decls,
            value_factor: names_surface_chunk_factor(chunk_index, legacy_count),
        },
    ));
    groups
}

/// One names-surface batch's declarations and its share of the
/// file's roster value.
struct NamesChunk {
    decls: Vec<usize>,
    value_factor: f64,
}

/// Everything needed to render and price an arbitrary slice of one
/// file's declaration roster. The partitioner measures candidate slices
/// with the same code that later emits them, so a slice can never be
/// chosen on a cost the emitted batch does not have.
struct RosterCtx<'a, 'tree> {
    file: &'a Path,
    ctx: &'a WalkCtx,
    source: &'a str,
    decls: &'a [(Node<'tree>, DeclInfo)],
    all_name_lines: &'a HashSet<usize>,
    row_claimants: &'a HashMap<usize, Vec<usize>>,
    co_location: Vec<usize>,
}

impl RosterCtx<'_, '_> {
    fn lines(&self, indices: &[usize]) -> FileLines {
        collect_decl_names_from(
            indices.iter().map(|&i| (i, &self.decls[i].1)),
            self.all_name_lines,
            self.row_claimants,
        )
    }

    fn cost(&self, indices: &[usize]) -> usize {
        single_file_lines_content(self.file, self.source, self.lines(indices))
            .map(|content| self.ctx.marginal_tokens(&content))
            .unwrap_or(0)
    }

    /// `indices` grown to whole co-location sets, in source order, so a
    /// chunk cut along this boundary keeps every claimant of a shared
    /// row on the same side of it.
    fn co_located_closure(&self, indices: &[usize]) -> Vec<usize> {
        let sets: HashSet<usize> = indices
            .iter()
            .map(|&index| self.co_location[index])
            .collect();
        (0..self.decls.len())
            .filter(|&index| sets.contains(&self.co_location[index]))
            .collect()
    }
}

/// The head slice of [`decl_names_chunk_groups`], in source order, or
/// `None` when no slice within the ceilings can serve as a gate.
fn entry_slice(roster: &RosterCtx) -> Option<Vec<usize>> {
    let decls = roster.decls;
    if decls.len() <= GO_ENTRY_SLICE_DECLS {
        return None;
    }
    let ranks: Vec<f64> = decls
        .iter()
        .map(|(node, info)| {
            let has_doc = !collect_doc_comments_above(*node, roster.source)
                .full
                .is_empty();
            entry_slice_rank(info, has_doc)
        })
        .collect();
    let mut by_rank: Vec<usize> = (0..decls.len()).collect();
    by_rank.sort_by(|&a, &b| ranks[b].total_cmp(&ranks[a]).then(a.cmp(&b)));
    by_rank.truncate(GO_ENTRY_SLICE_DECLS);
    // Trim the lowest-ranked members until the gate fits its ceiling.
    // Descending rank order means popping the tail; the survivors go
    // back into source order so the slice renders as a reading of the
    // file rather than of the ranking.
    let candidate = |chosen: &[usize]| roster.co_located_closure(&sorted(chosen));
    while !by_rank.is_empty() && roster.cost(&candidate(&by_rank)) > GO_ENTRY_SLICE_TOKENS {
        by_rank.pop();
    }
    // The ceiling is the whole mechanism: a gate priced like the roster
    // it was carved from admits nothing the roster would not have
    // admitted on its own, and still costs the remainder a chunk seam.
    // So when even the top-ranked declaration renders over the ceiling,
    // fall back to the best-ranked declaration that does fit — and when
    // none does, decline to split.
    if by_rank.is_empty() {
        let fits = (0..decls.len())
            .filter(|&index| roster.cost(&candidate(&[index])) <= GO_ENTRY_SLICE_TOKENS)
            .max_by(|&a, &b| ranks[a].total_cmp(&ranks[b]).then(b.cmp(&a)))?;
        by_rank.push(fits);
    }
    Some(candidate(&by_rank))
}

/// The directory's **spine**: the one source file declaring strictly
/// more top-level names than any of its siblings.
///
/// Roster value is near size-invariant while roster cost grows with the
/// declaration count, so inside one directory the entry ratio decays
/// with `N` and the file with the most declarations is the last of its
/// siblings to be admitted — usually never, at the budgets that matter.
/// That inversion is what the gate slice exists to undo, and confining
/// it to one file per directory is what keeps a package's breadth from
/// being spent on a scattering of half-read files.
///
/// Declaration count is a coarse stand-in for importance — a generated
/// or repetitive file (`Zip2`…`Zip9`) can out-declare the file the
/// package is actually about. It is used only to pick *which* file may
/// carve a gate, never to price anything, and the caller applies further
/// conditions before carving. Returns `None` on a tie, when no file
/// stands out from its siblings.
fn spine_file(source_files: &[PathBuf], ctx: &WalkCtx) -> Option<PathBuf> {
    let mut best: Option<(&PathBuf, usize)> = None;
    let mut tied = false;
    for file in source_files {
        let Some((source, tree)) = parse_go(ctx, file) else {
            continue;
        };
        let count = find_decls(&tree, &source).len();
        match best {
            Some((_, best_count)) if count < best_count => {}
            Some((_, best_count)) if count == best_count => tied = true,
            _ => {
                best = Some((file, count));
                tied = false;
            }
        }
    }
    match best {
        Some((file, _)) if !tied => Some(file.clone()),
        _ => None,
    }
}

fn sorted(indices: &[usize]) -> Vec<usize> {
    let mut out = indices.to_vec();
    out.sort_unstable();
    out
}

fn is_test_file(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.ends_with("_test.go"))
}

// --- decl classification ------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclKind {
    Type,
    Func,
    Method,
    Var,
    Const,
}

impl DeclKind {
    fn kind_weight(self) -> f64 {
        match self {
            DeclKind::Type => 1.10,
            DeclKind::Func => 1.00,
            DeclKind::Method => 0.95,
            DeclKind::Const => 0.90,
            DeclKind::Var => 0.85,
        }
    }

    fn has_body(self) -> bool {
        matches!(self, DeclKind::Func | DeclKind::Method)
    }
}

#[derive(Debug, Clone)]
struct DeclInfo {
    kind: DeclKind,
    start_line: usize,
    decl_lines: Vec<usize>,
    /// One entry per name on the names surface — one per inner spec
    /// for grouped `type/var/const ( … )` blocks.
    name_lines: Vec<usize>,
    body_rows: Option<(usize, usize)>,
    exported: bool,
    /// Line of the doc comment's `Deprecated:` marker, if any. Rides
    /// with the names surface rather than the doc batch — see
    /// [`collect_decl_names_from`].
    deprecation_marker: Option<usize>,
    /// Blank-line-separated field groups inside a big struct body;
    /// when present, `decl_lines` covers only header + closing brace.
    struct_field_groups: Vec<(usize, Vec<usize>)>,
}

impl DeclInfo {
    fn visibility_factor(&self) -> f64 {
        if self.exported {
            VISIBILITY_FACTOR_EXPORTED
        } else {
            VISIBILITY_FACTOR_UNEXPORTED
        }
    }

    /// `package_main`: exportedness carries no signal in `package main`
    /// (nothing imports it), so the visibility discount is waived.
    fn kv(&self, package_main: bool) -> f64 {
        let visibility = if package_main {
            VISIBILITY_FACTOR_EXPORTED
        } else {
            self.visibility_factor()
        };
        self.kind.kind_weight() * visibility
    }
}

fn find_decls<'a>(tree: &'a Tree, source: &str) -> Vec<(Node<'a>, DeclInfo)> {
    let src_lines: Vec<&str> = source.lines().collect();
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        let info = match child.kind() {
            "function_declaration" => func_or_method_info(child, source, DeclKind::Func),
            "method_declaration" => func_or_method_info(child, source, DeclKind::Method),
            "type_declaration" => Some(grouped_type_info(child, source, &src_lines)),
            "var_declaration" => Some(grouped_value_info(child, source, DeclKind::Var)),
            "const_declaration" => Some(grouped_value_info(child, source, DeclKind::Const)),
            _ => None,
        };
        let Some(mut info) = info else { continue };
        info.deprecation_marker = deprecation_marker_line(child, source, &src_lines);
        out.push((child, info));
    }
    out
}

/// 1-based line of the `Deprecated:` marker in `node`'s doc comment —
/// Go's documented deprecation convention. `None` for a live decl.
fn deprecation_marker_line(node: Node, source: &str, src_lines: &[&str]) -> Option<usize> {
    collect_doc_comments_above(node, source)
        .full
        .into_iter()
        .find(|line| {
            src_lines
                .get(line - 1)
                .is_some_and(|text| is_deprecation_marker(text))
        })
}

/// A comment row opening Go's documented deprecation paragraph.
fn is_deprecation_marker(text: &str) -> bool {
    text.trim_start_matches(['/', '\t', ' '])
        .starts_with("Deprecated:")
}

/// Rows of the file's `//go:build` constraint header. A constraint
/// decides whether the file's contents exist at all for a given build,
/// so it contradicts the roster below it in the strongest available
/// sense — rendered without it, platform- or tag-specific declarations
/// read as the package's universal API.
///
/// Go requires the constraint block to be followed by a blank line
/// before the `package` clause; a comment that touches the clause is a
/// doc comment, not a constraint. The pre-1.17 `// +build` mirror is
/// dropped whenever the modern form is present — it restates the same
/// constraint and would cost a second row to say so.
fn build_constraint_rows(src_lines: &[&str]) -> Vec<usize> {
    let mut modern = Vec::new();
    let mut legacy = Vec::new();
    let mut header_end = 0;
    for (i, raw) in src_lines.iter().enumerate() {
        let trimmed = raw.trim();
        if trimmed.starts_with("package ") {
            break;
        }
        if trimmed.starts_with("//go:build") {
            modern.push(i + 1);
        } else if trimmed.starts_with("// +build") {
            legacy.push(i + 1);
        } else {
            continue;
        }
        header_end = i + 1;
    }
    if !src_lines
        .get(header_end)
        .is_some_and(|l| l.trim().is_empty())
    {
        return Vec::new();
    }
    if modern.is_empty() { legacy } else { modern }
}

fn func_or_method_info(node: Node, source: &str, kind: DeclKind) -> Option<DeclInfo> {
    let name_node = node.child_by_field_name("name")?;
    let name = &source[name_node.start_byte()..name_node.end_byte()];
    let sig_end = signature_end_row(node);
    let mut decl_lines = Vec::new();
    push_rows(&mut decl_lines, node.start_position().row, sig_end);
    let start_line = node.start_position().row + 1;
    // Methods additionally require the receiver type to be exported.
    let exported = is_exported(name)
        && (!matches!(kind, DeclKind::Method) || receiver_type_exported(node, source));
    Some(DeclInfo {
        kind,
        start_line,
        decl_lines,
        name_lines: vec![start_line],
        body_rows: body_interior_rows(node),
        exported,
        deprecation_marker: None,
        struct_field_groups: Vec::new(),
    })
}

fn grouped_type_info(node: Node, source: &str, src_lines: &[&str]) -> DeclInfo {
    let mut cursor = node.walk();
    let mut name_lines = Vec::new();
    let mut exported = false;
    let mut single_type_spec: Option<Node> = None;
    let mut type_spec_count = 0;
    for spec in node.children(&mut cursor) {
        if !matches!(spec.kind(), "type_spec" | "type_alias") {
            continue;
        }
        type_spec_count += 1;
        single_type_spec = Some(spec);
        name_lines.push(spec.start_position().row + 1);
        if let Some(name_node) = spec.child_by_field_name("name")
            && is_exported(&source[name_node.start_byte()..name_node.end_byte()])
        {
            exported = true;
        }
    }
    let start_row = node.start_position().row;
    let end_row = node.end_position().row;
    let mut decl_lines = Vec::new();
    push_rows(&mut decl_lines, start_row, end_row);

    let mut struct_field_groups = Vec::new();
    // Only chunk single-spec struct decls; multi-spec groups stay whole.
    if type_spec_count == 1
        && let Some(spec) = single_type_spec
        && let Some(struct_body) = find_struct_body(spec)
    {
        let body_start = struct_body.start_position().row;
        let body_end = struct_body.end_position().row;
        let body_lines = body_end.saturating_sub(body_start) + 1;
        // Below the main threshold, only near-monolithic bodies (a
        // 40+-line struct with ≤2 blank-line groups, i.e. no internal
        // structure for the group split to key on) chunk — they ride
        // as one unschedulable batch otherwise (act's `Input`).
        // Well-blank-separated mid-size structs stay whole: chunking
        // them mints a flood of tiny high-ratio groups (measured:
        // migrate −0.153 via `migration.go`'s 12 tiny groups).
        // Standalone full-line comment rows are elided — NS struct
        // renders skip them, and comment-per-field styles (cobra's
        // `Command`) otherwise cost ~5x the field lines alone.
        // Same-line trailing comments share a row with their field and
        // stay; groups left empty (pure comment dividers) are dropped.
        //
        // The one comment row that survives elision is a `Deprecated:`
        // marker. Every other doc row elaborates a field the group
        // already lists, so dropping it costs detail; this one
        // contradicts the listing, and dropping it renders a field the
        // package disowned as indistinguishable from its live siblings.
        let comment_rows = comment_only_rows(struct_body);
        let blank_groups: Vec<(usize, Vec<usize>)> = collect_blank_line_groups(struct_body, source)
            .into_iter()
            .filter_map(|(_, rows)| {
                let rows: Vec<usize> = rows
                    .into_iter()
                    .filter(|row| {
                        !comment_rows.contains(&(row - 1))
                            || src_lines
                                .get(row - 1)
                                .is_some_and(|text| is_deprecation_marker(text))
                    })
                    .collect();
                let start = *rows.first()?;
                Some((start, rows))
            })
            .collect();
        let chunkable = body_lines >= STRUCT_FIELD_GROUP_MIN_LINES
            || (body_lines >= STRUCT_FIELD_GROUP_MONOLITH_MIN_LINES && blank_groups.len() <= 2);
        if chunkable {
            let groups = split_oversized_groups(blank_groups);
            if !groups.is_empty() {
                // Trim decl_lines to the type header row + the
                // struct's closing-brace row. Body rows in between
                // now belong to per-`StructFieldGroup` batches, and
                // any rows after the struct (uncommon) stay with the
                // header so the Decl renders the full structural
                // framing in one batch.
                decl_lines.clear();
                decl_lines.push(start_row + 1);
                decl_lines.push(body_end + 1);
                if end_row > body_end {
                    push_rows(&mut decl_lines, body_end + 1, end_row);
                }
                decl_lines.sort_unstable();
                decl_lines.dedup();
                struct_field_groups = groups;
            }
        }
    }

    DeclInfo {
        kind: DeclKind::Type,
        start_line: start_row + 1,
        decl_lines,
        name_lines,
        body_rows: None,
        exported,
        deprecation_marker: None,
        struct_field_groups,
    }
}

/// Minimum struct-body span (lines) for field-group chunking.
const STRUCT_FIELD_GROUP_MIN_LINES: usize = 60;

/// Lower chunking threshold for near-monolithic struct bodies (≤2
/// blank-line groups) — see the gate comment in `grouped_type_info`.
const STRUCT_FIELD_GROUP_MONOLITH_MIN_LINES: usize = 40;

/// Blank-line groups longer than this split into
/// [`STRUCT_FIELD_GROUP_SPLIT_ROWS`]-row runs. Without it a struct
/// body with no interior blank lines (act's `cmd/input.go` `Input`)
/// rides as one monolithic group whose cost keeps the whole batch —
/// and with it the NS-anchored field roster — unschedulable.
const STRUCT_FIELD_GROUP_MAX_ROWS: usize = 16;
const STRUCT_FIELD_GROUP_SPLIT_ROWS: usize = 12;

/// Split any group longer than [`STRUCT_FIELD_GROUP_MAX_ROWS`] into
/// fixed-size runs; a short trailing run merges into the previous one
/// (same rationale as the C walker's names-chunk remainder merge).
fn split_oversized_groups(groups: Vec<(usize, Vec<usize>)>) -> Vec<(usize, Vec<usize>)> {
    let mut out = Vec::with_capacity(groups.len());
    for (start, rows) in groups {
        if rows.len() <= STRUCT_FIELD_GROUP_MAX_ROWS {
            out.push((start, rows));
            continue;
        }
        let mut chunks: Vec<Vec<usize>> = rows
            .chunks(STRUCT_FIELD_GROUP_SPLIT_ROWS)
            .map(<[usize]>::to_vec)
            .collect();
        if let Some(last) = chunks.last()
            && last.len() < STRUCT_FIELD_GROUP_SPLIT_ROWS / 2
            && chunks.len() >= 2
        {
            let tail = chunks.pop().expect("len checked");
            chunks.last_mut().expect("len checked").extend(tail);
        }
        for chunk in chunks {
            let chunk_start = *chunk.first().expect("chunks are non-empty");
            out.push((chunk_start, chunk));
        }
    }
    out
}

/// `struct_type` node from a struct-typed `type_spec`.
fn find_struct_body(spec: Node) -> Option<Node> {
    let ty = spec.child_by_field_name("type")?;
    if ty.kind() == "struct_type" {
        Some(ty)
    } else {
        None
    }
}

fn grouped_value_info(node: Node, source: &str, kind: DeclKind) -> DeclInfo {
    fn walk_specs<'a>(parent: Node<'a>, specs: &mut Vec<Node<'a>>) {
        let mut cursor = parent.walk();
        for child in parent.children(&mut cursor) {
            match child.kind() {
                "var_spec" | "const_spec" => specs.push(child),
                "var_spec_list" => walk_specs(child, specs),
                _ => continue,
            }
        }
    }
    let mut specs = Vec::new();
    walk_specs(node, &mut specs);
    let mut name_lines = Vec::new();
    let mut exported = false;
    for spec in &specs {
        name_lines.push(spec.start_position().row + 1);
        let mut name_cursor = spec.walk();
        for n in spec.children_by_field_name("name", &mut name_cursor) {
            if is_exported(&source[n.start_byte()..n.end_byte()]) {
                exported = true;
                break;
            }
        }
    }
    let mut decl_lines = Vec::new();
    push_rows(
        &mut decl_lines,
        node.start_position().row,
        node.end_position().row,
    );
    DeclInfo {
        kind,
        start_line: node.start_position().row + 1,
        decl_lines,
        name_lines,
        body_rows: None,
        exported,
        deprecation_marker: None,
        struct_field_groups: Vec::new(),
    }
}

/// True iff `name`'s first character is an uppercase Unicode letter
/// (Go's exported-name rule).
fn is_exported(name: &str) -> bool {
    name.chars().next().is_some_and(|c| c.is_uppercase())
}

fn receiver_type_exported(method: Node, source: &str) -> bool {
    let Some(receiver) = method.child_by_field_name("receiver") else {
        return false;
    };
    let mut cursor = receiver.walk();
    for child in receiver.children(&mut cursor) {
        if child.kind() == "parameter_declaration" {
            let Some(ty) = child.child_by_field_name("type") else {
                continue;
            };
            return type_root_identifier(ty, source).is_some_and(is_exported);
        }
    }
    false
}

/// Root `type_identifier` of a type, stripping `*T`/`T[U]` wrappers.
fn type_root_identifier<'a>(node: Node<'_>, source: &'a str) -> Option<&'a str> {
    match node.kind() {
        "type_identifier" => Some(&source[node.start_byte()..node.end_byte()]),
        "pointer_type" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.is_named() {
                    return type_root_identifier(child, source);
                }
            }
            None
        }
        "generic_type" => node
            .child_by_field_name("type")
            .and_then(|t| type_root_identifier(t, source)),
        "qualified_type" => node
            .child_by_field_name("name")
            .and_then(|t| type_root_identifier(t, source)),
        _ => None,
    }
}

fn body_interior_rows(node: Node) -> Option<(usize, usize)> {
    let body = node.child_by_field_name("body")?;
    let s = body.start_position().row;
    let e = body.end_position().row;
    if e <= s + 1 { None } else { Some((s, e)) }
}

fn collect_test_function_lines(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() != "function_declaration" {
            continue;
        }
        let Some(name_node) = child.child_by_field_name("name") else {
            continue;
        };
        let name = &source[name_node.start_byte()..name_node.end_byte()];
        if name.starts_with("Test") || name.starts_with("Benchmark") || name.starts_with("Example")
        {
            out.push(child.start_position().row + 1);
        }
    }
    out
}

// --- value functions ----------------------------------------------------

fn go_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(file, ctx, false)
}

/// Per-role `(cat, fu, ztu)` signal triples sharing the entry-factor /
/// aux-factor pipeline.
#[derive(Clone, Copy)]
enum GoRole {
    PackageDocLede,
    PackageDocBody,
    PackageImports,
    DeclNames,
    Decl,
    DeclDoc,
    DeclBody,
    StructFieldGroup,
}

impl GoRole {
    /// `kv` is `kind_weight * visibility_factor` for decl-bearing
    /// roles, `1.0` for the no-decl roles.
    fn value(self, file: &Path, ctx: &WalkCtx, entry_factor: f64, kv: f64) -> f64 {
        let (cat, fu, ztu) = match self {
            GoRole::PackageDocLede => (0.60, 0.55, 0.55),
            // Canonical-usage demo (the Go analog of a README
            // quickstart fence) — high zero-tool-call yield.
            GoRole::PackageDocBody => (0.60, 0.70, 0.60),
            GoRole::PackageImports => (0.30, 0.55, 0.30),
            GoRole::DeclNames => (0.65, 0.55, 0.35),
            GoRole::Decl => (0.70, 0.85, 0.65),
            GoRole::DeclDoc => (0.20, 0.60, 0.80),
            GoRole::DeclBody => (0.30, 0.80, 0.70),
            GoRole::StructFieldGroup => (0.55, 0.70, 0.55),
        };
        mix_signals(
            (cat * kv).min(1.0),
            (fu * kv).min(1.0),
            ztu,
            go_depth_factor(file, ctx),
        ) * go_aux_factor(file)
            * entry_factor
    }
}

/// Boost for entry-shaped Go surfaces (package-name match or exported
/// chunked struct, root-level only — see `go_entry_factor_for`).
const GO_ENTRY_FACTOR: f64 = 1.4;

/// 1.4× boost for files anchoring the package API surface —
/// package-name match or an exported chunked struct. Root-level only.
///
/// Tried and reverted: extending the eponymous (`stem == package`)
/// boost to *nested* subpackages (`internal/config/config.go`,
/// `pkg/runner/runner.go`). The boost multiplies every decl/doc/body
/// of the file, so it floods the 3K budget with one directory's whole
/// interior. At 1.4× it regressed every Go fixture (mcphost −0.086,
/// gin −0.077); even a gentle 1.15× moved no target (the NS rows the
/// boost would surface — mcphost `config.go` 2.1/2.2/2.4 — are big
/// enough that their fidelity never recovers within budget) while
/// still pulling tock's out-of-budget `internal/config/config.go`
/// ahead of its own in-budget content (−0.043). There's no walk-time
/// signal distinguishing "NS wants this nested file in budget" from
/// "it doesn't", so the nested boost is net-negative corpus-wide.
fn go_entry_factor_for(
    file: &Path,
    ctx: &WalkCtx,
    pkg: Option<&str>,
    decls: &[(Node, DeclInfo)],
) -> f64 {
    if ctx.depth_from_root(file) > 1 {
        return 1.0;
    }
    let Some(stem) = file.file_stem().and_then(|s| s.to_str()) else {
        return 1.0;
    };
    let big_struct_anchor = decls
        .iter()
        .any(|(_, info)| info.exported && !info.struct_field_groups.is_empty());
    if pkg == Some(stem) || big_struct_anchor {
        GO_ENTRY_FACTOR
    } else {
        1.0
    }
}

fn package_name(tree: &Tree, source: &str) -> Option<String> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "package_clause" {
            let mut inner = child.walk();
            for n in child.children(&mut inner) {
                if matches!(n.kind(), "package_identifier" | "identifier") {
                    return Some(source[n.start_byte()..n.end_byte()].to_string());
                }
            }
        }
    }
    None
}

/// Damp Go files whose stem carries a build-tag suffix — aux
/// implementation behind a portable interface; NS authors anchor on
/// the un-suffixed sibling.
fn go_aux_factor(file: &Path) -> f64 {
    let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    let lower = name.to_ascii_lowercase();
    let stem = lower.strip_suffix(".go").unwrap_or(&lower);
    if is_go_build_variant_stem(stem) {
        return 0.5;
    }
    1.0
}

/// Go build-tag suffix on a filename stem. The Go toolchain auto-
/// applies a build constraint matching the trailing `_<os>` /
/// `_<arch>` / `_<os>_<arch>` segment; the `_unix` / `_bsd` /
/// `_other` / `_notwin` informal variants follow the same shape.
fn is_go_build_variant_stem(stem: &str) -> bool {
    let Some((_, suffix)) = stem.rsplit_once('_') else {
        return false;
    };
    matches!(
        suffix,
        // GOOS values (subset of `go tool dist list`).
        "darwin" | "linux" | "windows" | "freebsd" | "openbsd" | "netbsd"
        | "dragonfly" | "plan9" | "solaris" | "ios" | "android" | "aix"
        | "illumos" | "js" | "wasm" | "wasip1" | "zos"
        // GOARCH values.
        | "amd64" | "arm" | "arm64" | "386" | "ppc64" | "riscv64" | "s390x"
        // Informal multi-OS variants.
        | "unix" | "bsd" | "other" | "win" | "notwin" | "nonwin"
    )
}

fn test_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.40, 0.0, 0.3, go_depth_factor(file, ctx))
}

fn gomod_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Identity directives split into [`GoKey::GoModIdentity`]; this
    // batch reflects the residual require / replace / exclude /
    // retract content.
    mix_signals(0.80, 0.60, 0.5, go_depth_factor(file, ctx))
}

fn gomod_identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.90, 0.75, 0.35, go_depth_factor(file, ctx))
}

// --- parser -------------------------------------------------------------

fn parse_go(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_go::LANGUAGE.into())
}

// --- collectors ---------------------------------------------------------

fn collect_package_imports(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "package_clause" | "import_declaration" => extend_span(&mut lines, child, source),
            _ => continue,
        }
    }
    FileLines::new(dedup_sorted(lines))
}

/// The contiguous comment block immediately above the file's
/// `package` clause — Go's "package comment" convention. Emitted as a
/// separate batch (vs folding into `PackageImports`) so the lede can
/// fire standalone at low cost.
///
/// `/* … */` block comments span the whole godoc body in a single
/// tree-sitter node, so we truncate to the first paragraph (stop at
/// first blank source row). `//`-style ledes are unaffected since
/// `collect_doc_comments_above` already stops at any row gap.
/// `(lede, body)` split of the package doc comment: the lede is the
/// first paragraph (rows before the first blank source row — only
/// `/* */` blocks span blank rows); the body is every remaining row of
/// the comment. The body is empty for `//`-style ledes.
fn collect_package_doc_parts(tree: &Tree, source: &str) -> (FileLines, FileLines) {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "package_clause" {
            let lines = collect_doc_comments_above(child, source);
            let lede = truncate_at_first_blank_row(lines.clone(), source);
            let body_rows: Vec<usize> = lines
                .full
                .into_iter()
                .filter(|row| !lede.full.contains(row))
                .collect();
            return (lede, FileLines::new(body_rows));
        }
    }
    (FileLines::new(Vec::new()), FileLines::new(Vec::new()))
}

/// True iff `file` is a dedicated package-documentation file: the Go
/// convention is a decl-less `doc.go` whose only top-level content is
/// the `package` clause and its leading godoc comment. We require the
/// `doc.go` name (the convention is explicit) and zero top-level
/// declarations so a `doc.go` carrying real code is never mistaken for
/// doc-only.
fn is_package_doc_file(file: &Path, decls: &[(Node, DeclInfo)]) -> bool {
    let stem = file.file_stem().and_then(|s| s.to_str());
    stem == Some("doc") && decls.is_empty()
}

/// Drop any row at or after the first blank source line in `lines.full`.
/// `FileLines` row numbers are 1-based.
fn truncate_at_first_blank_row(lines: FileLines, source: &str) -> FileLines {
    let src_lines: Vec<&str> = source.lines().collect();
    let mut kept = Vec::new();
    for row in lines.full {
        if src_lines
            .get(row.saturating_sub(1))
            .is_some_and(|t| t.trim().is_empty())
        {
            break;
        }
        kept.push(row);
    }
    FileLines::new(kept)
}

/// Every row any single declaration's batches can render, mapped to the
/// declarations that claim it, ascending. The first claimant is the
/// row's owner; a row with more than one claimant is *co-located*.
///
/// Roster chunks consult this to stay disjoint. Two declarations can
/// share a source row (`var a = 1; var b = 2`), and the row after a
/// body-less declaration is routinely the *next* declaration's doc
/// comment — so without an owner map, one chunk's roster row or
/// continuation ellipsis lands on a row another declaration's train
/// later renders. While chunks were cut in source order that was almost
/// always the same chunk, hence an ancestor; a rank-chosen gate slice
/// scatters neighbours across chunks and makes it a non-ancestor
/// overlap: a debug panic, and a silently dropped row in release.
fn decl_row_claimants(
    decls: &[(Node, DeclInfo)],
    source: &str,
    src_lines: &[&str],
) -> HashMap<usize, Vec<usize>> {
    let mut claimants: HashMap<usize, Vec<usize>> = HashMap::new();
    for (decl_index, (node, info)) in decls.iter().enumerate() {
        let rows = info
            .name_lines
            .iter()
            .copied()
            .chain(info.decl_lines.iter().copied())
            .chain(collect_doc_comments_above(*node, source).full)
            .chain(collect_decl_body(info, src_lines).full)
            .chain(
                info.struct_field_groups
                    .iter()
                    .flat_map(|(_, rows)| rows.iter().copied()),
            );
        for row in rows {
            let row_claimants = claimants.entry(row).or_default();
            if row_claimants.last() != Some(&decl_index) {
                row_claimants.push(decl_index);
            }
        }
    }
    claimants
}

/// One representative declaration index per set of declarations that
/// share a source row, transitively.
///
/// A roster chunk renders a shared row once, but every co-located
/// declaration's own `Decl` batch renders it too — so the row is only
/// safe if all of its claimants gate on the same chunk. Splitting them
/// is the same non-ancestor overlap [`decl_row_claimants`] exists to
/// prevent, one the row-owner filter cannot reach because the offending
/// batch belongs to the declaration train rather than the roster.
fn co_location_representatives(
    decl_count: usize,
    claimants: &HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    let mut representatives: Vec<usize> = (0..decl_count).collect();
    for row_claimants in claimants.values().filter(|shared| shared.len() > 1) {
        let merged = row_claimants
            .iter()
            .map(|&decl| representatives[decl])
            .min()
            .unwrap_or_default();
        let replaced: Vec<usize> = row_claimants
            .iter()
            .map(|&decl| representatives[decl])
            .collect();
        for representative in &mut representatives {
            if replaced.contains(representative) {
                *representative = merged;
            }
        }
    }
    representatives
}

/// One full + ellipsis pair per name line so a grouped block surfaces
/// every inner spec, not just the `type (` opener.
///
/// Both are filtered through [`decl_row_claimants`]: a declaration only
/// renders the roster rows it owns, and only trails an ellipsis onto a
/// row no *other* declaration claims. The ellipsis rule is deliberately
/// blind to which chunk the other declaration landed in — that keeps
/// the rows a chunk renders independent of how the file was partitioned,
/// which is what lets the partitioner price candidate slices with the
/// same function that emits them.
fn collect_decl_names_from<'a>(
    decls: impl IntoIterator<Item = (usize, &'a DeclInfo)>,
    all_name_lines: &HashSet<usize>,
    row_claimants: &HashMap<usize, Vec<usize>>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let owns = |decl_index: usize, row: usize| {
        row_claimants
            .get(&row)
            .is_some_and(|claimants| claimants.first() == Some(&decl_index))
    };
    for (decl_index, info) in decls {
        // The roster is often the *only* place a decl appears, and a
        // deprecated decl listed among live siblings steers the reader
        // onto the API the package told them not to use. The marker
        // rides with the roster line instead of competing as `DeclDoc`,
        // so it cannot be separated from what it negates.
        full.extend(
            info.deprecation_marker
                .filter(|&marker| owns(decl_index, marker)),
        );
        for &line in &info.name_lines {
            if !owns(decl_index, line) {
                continue;
            }
            full.push(line);
            let ellipsis_line = line + 1;
            // A declaration's own body row is the normal target — that
            // ellipsis is the marker saying the body was elided. Only
            // somebody else's row is off limits.
            if !all_name_lines.contains(&ellipsis_line)
                && row_claimants
                    .get(&ellipsis_line)
                    .is_none_or(|claimants| claimants.iter().all(|&claim| claim == decl_index))
            {
                ellipses.push(ellipsis_line);
            }
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

fn collect_decl(info: &DeclInfo) -> FileLines {
    FileLines::new(info.decl_lines.clone())
}

fn collect_decl_body(info: &DeclInfo, src_lines: &[&str]) -> FileLines {
    let Some((s, e)) = info.body_rows else {
        return FileLines::new(Vec::new());
    };
    let mut out = Vec::new();
    extend_nonblank_rows(&mut out, src_lines, s + 1, e - 1);
    FileLines::new(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Render;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    fn parse(source: &str) -> (String, Tree) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_go::LANGUAGE.into())
            .expect("load go grammar");
        let tree = parser.parse(source, None).expect("parse");
        (source.to_string(), tree)
    }

    fn gomod_lines(src: &str) -> Vec<usize> {
        gomod_rendered_lines(src)
            .into_iter()
            .map(|(line, _)| line)
            .collect()
    }

    fn gomod_rendered_lines(src: &str) -> Vec<(usize, Render)> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("go.mod");
        std::fs::write(&path, src).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let content = build_gomod_content(&path, &ctx).expect("emits content");
        let BatchContent::Lines { spans } = content else {
            panic!("expected Lines content");
        };
        spans
            .iter()
            .flat_map(|span| (span.start..=span.end).map(|line| (line, span.render.clone())))
            .collect()
    }

    fn gomod_identity_lines(src: &str) -> Vec<usize> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("go.mod");
        std::fs::write(&path, src).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let content = build_gomod_identity_content(&path, &ctx).expect("emits content");
        let BatchContent::Lines { spans } = content else {
            panic!("expected Lines content");
        };
        spans
            .iter()
            .flat_map(|span| span.start..=span.end)
            .collect()
    }

    #[test]
    fn go_struct_field_groups_elide_comment_only_rows() {
        // Comment-per-field style (cobra's `Command`): each blank-line
        // group is two doc rows + one field row. Elision keeps only
        // the field rows, so the group batches render as the name-only
        // roster the NS convention wants.
        let mut src = String::from("package foo\n\ntype Doc struct {\n");
        for g in 0..20 {
            src.push_str(&format!(
                "\t// Field{g} does a thing.\n\t// More detail.\n\tField{g} string\n\n"
            ));
        }
        src.push_str("}\n");
        let (source, tree) = parse(&src);
        let decls = find_decls(&tree, &source);
        assert_eq!(decls.len(), 1);
        let groups = &decls[0].1.struct_field_groups;
        assert_eq!(groups.len(), 20, "one group per blank-line field run");
        let src_lines: Vec<&str> = src.lines().collect();
        for (_, rows) in groups {
            assert_eq!(rows.len(), 1, "doc rows are elided from the group");
            for &row in rows {
                assert!(
                    src_lines[row - 1].trim_start().starts_with("Field"),
                    "comment-only rows are elided, got {:?}",
                    src_lines[row - 1]
                );
            }
        }
    }

    #[test]
    fn go_struct_field_group_keeps_a_disavowed_field_s_marker() {
        let mut src = String::from("package foo\n\ntype Doc struct {\n");
        for g in 0..20 {
            src.push_str(&format!("\t// Field{g} does a thing.\n"));
            if g == 3 {
                src.push_str("\t// Deprecated: use Field4 instead.\n");
            } else {
                src.push_str("\t// More detail.\n");
            }
            src.push_str(&format!("\tField{g} string\n\n"));
        }
        src.push_str("}\n");
        let (source, tree) = parse(&src);
        let decls = find_decls(&tree, &source);
        let groups = &decls[0].1.struct_field_groups;
        let src_lines: Vec<&str> = src.lines().collect();
        let rendered: Vec<Vec<&str>> = groups
            .iter()
            .map(|(_, rows)| rows.iter().map(|&row| src_lines[row - 1].trim()).collect())
            .collect();
        assert_eq!(
            rendered[3],
            vec!["// Deprecated: use Field4 instead.", "Field3 string"],
            "the row that contradicts the listing survives elision"
        );
        assert_eq!(
            rendered[4],
            vec!["Field4 string"],
            "rows that merely elaborate a listed field are still elided"
        );
    }

    #[test]
    fn go_emits_both_exported_and_unexported_decls() {
        let src = "package foo\n\nfunc Public() {}\nfunc private() {}\n";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        assert_eq!(decls.len(), 2);
        assert_eq!(decls[0].1.start_line, 3);
        assert!(decls[0].1.exported, "Public should be exported");
        assert_eq!(decls[1].1.start_line, 4);
        assert!(!decls[1].1.exported, "private should be unexported");
    }

    #[test]
    fn go_build_constraint_rows_are_the_header_go_would_honor() {
        let both = "\
//go:build linux && amd64
// +build linux,amd64

package foo
";
        assert_eq!(
            build_constraint_rows(&both.lines().collect::<Vec<_>>()),
            vec![1],
            "the pre-1.17 mirror restates the modern form and costs a second row"
        );

        let legacy = "// +build linux\n\npackage foo\n";
        assert_eq!(
            build_constraint_rows(&legacy.lines().collect::<Vec<_>>()),
            vec![1]
        );

        // Go only honors a constraint separated from the clause by a
        // blank line; touching it, the comment is package documentation
        // and already belongs to another batch.
        let touching = "//go:build linux\npackage foo\n";
        assert!(
            build_constraint_rows(&touching.lines().collect::<Vec<_>>()).is_empty(),
            "a comment touching the package clause is a doc comment"
        );

        let unconstrained = "// Package foo does things.\npackage foo\n";
        assert!(build_constraint_rows(&unconstrained.lines().collect::<Vec<_>>()).is_empty());
    }

    #[test]
    fn go_method_visibility_requires_both_name_and_receiver_type() {
        let src = "\
package foo

type Public struct{}
type private struct{}

func (p *Public) Method()  {}
func (p *Public) helper()  {}
func (p *private) Method() {}
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        assert_eq!(decls.len(), 5);
        let by_line: std::collections::HashMap<_, _> =
            decls.iter().map(|(_, d)| (d.start_line, d)).collect();
        assert!(by_line[&3].exported, "type Public exported");
        assert!(!by_line[&4].exported, "type private unexported");
        assert!(by_line[&6].exported, "Public.Method exported");
        assert!(
            !by_line[&7].exported,
            "Public.helper not exported (lowercase method)"
        );
        assert!(
            !by_line[&8].exported,
            "private.Method not exported (unexported receiver)"
        );
    }

    #[test]
    fn go_grouped_decls_become_one_batch_per_block() {
        let src = "\
package foo

type (
    Public  struct{}
    private struct{}
)

const (
    Format12Hour = 1 + iota
    Format24Hour
    formatInternal
)

var (
    Baz = 3
    qux = 4
)
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        let kinds: Vec<_> = decls.iter().map(|(_, d)| d.kind).collect();
        assert_eq!(kinds, vec![DeclKind::Type, DeclKind::Const, DeclKind::Var]);
        assert!(decls.iter().all(|(_, d)| d.exported));
        let const_decl = &decls[1].1;
        assert!(const_decl.decl_lines.contains(&9), "iota line included");
        assert!(const_decl.decl_lines.contains(&10), "Format24Hour included");
    }

    #[test]
    fn go_grouped_decl_names_surface_each_inner_spec() {
        let src = "\
package foo

type (
    Public  struct{}
    Other   struct{}
)

const (
    Format12Hour = 1 + iota
    Format24Hour
)
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        assert_eq!(decls.len(), 2);
        let all_name_lines: HashSet<usize> = decls
            .iter()
            .flat_map(|(_, info)| info.name_lines.iter().copied())
            .collect();
        let row_claimants =
            decl_row_claimants(&decls, &source, &source.lines().collect::<Vec<_>>());
        let names = collect_decl_names_from(
            decls
                .iter()
                .enumerate()
                .map(|(index, (_, info))| (index, info)),
            &all_name_lines,
            &row_claimants,
        );
        // Inner spec lines: Public@4, Other@5, Format12Hour@9, Format24Hour@10.
        assert_eq!(names.full, vec![4, 5, 9, 10]);
    }

    #[test]
    fn go_unexported_only_group_still_emits_with_low_visibility() {
        let src = "\
package foo

type (
    a struct{}
    b struct{}
)
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        assert_eq!(decls.len(), 1);
        assert!(!decls[0].1.exported);
        assert!((decls[0].1.visibility_factor() - VISIBILITY_FACTOR_UNEXPORTED).abs() < 1e-9);
    }

    #[test]
    fn go_doc_comment_above_decl_is_collected() {
        let src = "\
package foo

// Foo does a thing.
// Second doc line.
func Foo() {}

// gap doc

func Bar() {}
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        assert_eq!(decls.len(), 2);

        let foo_doc = collect_doc_comments_above(decls[0].0, &source);
        assert_eq!(foo_doc.full, vec![3, 4]);

        let bar_doc = collect_doc_comments_above(decls[1].0, &source);
        assert!(
            bar_doc.full.is_empty(),
            "blank-line gap separates the comment from Bar's decl; got {bar_doc:?}"
        );
    }

    #[test]
    fn go_deprecation_marker_rides_with_the_names_roster() {
        let src = "\
package foo

// Old does a thing.
//
// Deprecated: Use New instead.
func Old() {}

// New does a thing.
func New() {}
";
        let (source, tree) = parse(src);
        let src_lines: Vec<&str> = source.lines().collect();
        let decls = find_decls(&tree, &source);
        assert_eq!(decls[0].1.deprecation_marker, Some(5));
        assert_eq!(decls[1].1.deprecation_marker, None);

        let all_name_lines = decls
            .iter()
            .flat_map(|(_, info)| {
                info.name_lines
                    .iter()
                    .copied()
                    .chain(info.deprecation_marker)
            })
            .collect();
        let row_claimants =
            decl_row_claimants(&decls, &source, &source.lines().collect::<Vec<_>>());
        let names = collect_decl_names_from(
            decls
                .iter()
                .enumerate()
                .map(|(index, (_, info))| (index, info)),
            &all_name_lines,
            &row_claimants,
        );
        assert_eq!(
            names.full,
            vec![5, 6, 9],
            "the roster carries the marker, so `Old` can never render without it"
        );
        assert!(
            !names.ellipses.contains(&5),
            "the marker line is real content, not an elision marker"
        );

        // The doc batch keeps the prose and drops the marker line, so
        // the two batches' line sets stay disjoint.
        let mut doc_lines = collect_doc_comments_above(decls[0].0, &source);
        doc_lines
            .full
            .retain(|l| Some(*l) != decls[0].1.deprecation_marker);
        assert_eq!(doc_lines.full, vec![3, 4]);
        assert_eq!(
            deprecation_marker_line(decls[1].0, &source, &src_lines),
            None
        );
    }

    #[test]
    fn go_package_doc_lede_truncates_block_comment_at_first_blank_line() {
        // Block-comment ledes (gin's `doc.go`) often pack an entire
        // `Example:` body into the `/* … */` node. The lede batch should
        // ship only the first paragraph; the rest is example content
        // that's high cost and not the identity-level signal.
        let src = "\
/*
Package foo summarises the package in one sentence.

Example:

\tx := foo.New()
\tx.Run()
*/
package foo
";
        let (source, tree) = parse(src);
        let (lede, _) = collect_package_doc_parts(&tree, &source);
        assert_eq!(lede.full, vec![1, 2]);
    }

    #[test]
    fn go_is_package_doc_file_requires_doc_stem_and_no_decls() {
        let doc = Path::new("doc.go");
        let other = Path::new("config.go");
        let no_decls: Vec<(Node, DeclInfo)> = Vec::new();
        assert!(is_package_doc_file(doc, &no_decls));
        assert!(!is_package_doc_file(other, &no_decls));
        // A `doc.go` carrying real decls is not a doc-only file.
        let src = "package foo\n\nfunc Foo() {}\n";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        assert!(!is_package_doc_file(doc, &decls));
    }

    #[test]
    fn go_package_doc_lede_preserves_line_comment_block() {
        // `//`-style ledes don't have blank source rows inside the
        // contiguous comment run, so truncation is a no-op.
        let src = "\
// Package foo summarises the package.
// Continues onto a second line.
package foo
";
        let (source, tree) = parse(src);
        let (lede, _) = collect_package_doc_parts(&tree, &source);
        assert_eq!(lede.full, vec![1, 2]);
    }

    /// `PackageImports` predecessor for the sole `.go` file in a temp dir.
    fn package_imports_predecessor(src: &str) -> Option<BatchKey> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("subject.go");
        std::fs::write(&path, src).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        expand_source_files(&[path], &ctx)
            .into_iter()
            .find(|b| matches!(b.key, BatchKey::Go(GoKey::PackageImports { .. })))
            .expect("emits a PackageImports batch")
            .predecessor
    }

    #[test]
    fn go_imports_gate_on_the_files_declaration_roster() {
        let pred = package_imports_predecessor(
            "package foo\n\nimport \"strings\"\n\nfunc Foo(s string) string { return s }\n",
        );
        assert!(
            matches!(
                pred,
                Some(BatchKey::Go(GoKey::DeclNames { chunk_index: 0, .. }))
            ),
            "imports should refine the decl roster, not open the file: {pred:?}"
        );
    }

    #[test]
    fn go_imports_stay_ungated_when_the_file_declares_nothing() {
        // No roster exists to gate on, so gating would strand the batch.
        let pred = package_imports_predecessor("package foo\n\nimport \"strings\"\n");
        assert_eq!(pred, None);
    }

    /// Twelve long declarations: enough rendered cost to clear the split
    /// threshold, with only `Alpha` exported and only `Alpha` /
    /// `zulu` documented, so the entry-slice rank has something to sort.
    fn oversize_roster_source() -> String {
        let mut src = String::from(
            "package subject\n\n// Alpha is the documented entry point.\nfunc Alpha(ctx context.Context, name string, options map[string]string) (*Result, error) { return nil, nil }\n",
        );
        for index in 0..10 {
            src.push_str(&format!(
                "func helper{index}(ctx context.Context, name string, options map[string]string) (*Result, error) {{ return nil, nil }}\n"
            ));
        }
        src.push_str("// zulu is documented but unexported.\nfunc zulu(ctx context.Context, name string, options map[string]string) (*Result, error) { return nil, nil }\n");
        src
    }

    fn names_batches(files: &[(&str, String)]) -> Vec<(usize, Vec<usize>)> {
        let dir = tempfile::tempdir().unwrap();
        let paths: Vec<PathBuf> = files
            .iter()
            .map(|(name, src)| {
                let path = dir.path().join(name);
                std::fs::write(&path, src).unwrap();
                path
            })
            .collect();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let mut out: Vec<(usize, Vec<usize>)> = expand_source_files(&paths, &ctx)
            .into_iter()
            .filter_map(|batch| match (&batch.key, &batch.content) {
                (
                    BatchKey::Go(GoKey::DeclNames { chunk_index, .. }),
                    BatchContent::Lines { spans },
                ) => Some((
                    *chunk_index,
                    spans.iter().map(|span| span.start).collect::<Vec<_>>(),
                )),
                _ => None,
            })
            .collect();
        out.sort();
        out
    }

    /// Ten long unexported helpers — enough rendered cost to clear the
    /// split threshold. `lead` and `tail` bracket them so a caller can
    /// place a seam at either end of the roster.
    fn roster_with(lead: &str, tail: &str) -> String {
        let mut src = format!("package subject\n\n{lead}");
        for index in 0..10 {
            src.push_str(&format!(
                "func helper{index}(ctx context.Context, name string, options map[string]string) (*Result, error) {{ return nil, nil }}\n"
            ));
        }
        src.push_str(tail);
        src
    }

    /// Schedule a one-file repo at a budget large enough to reach every
    /// batch. In debug builds the scheduler panics when a batch writes a
    /// row a non-ancestor already owns, so this is the seam harness.
    fn schedule_go_source(src: &str) -> String {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("subject.go"), src).unwrap();
        std::fs::write(
            root.join("go.mod"),
            "module example.com/subject\n\ngo 1.22\n",
        )
        .unwrap();
        Scheduler::new(root.to_path_buf(), FsWalker, 100_000, None)
            .run_with_report()
            .tree
            .render()
    }

    #[test]
    fn go_roster_ellipsis_never_lands_on_a_later_decls_doc() {
        // Two documented exported funcs fill the gate; `Charlie` still
        // fits under the token ceiling but `helper0` does not, so the
        // body-less `Charlie` ends up in the gate with the row after its
        // roster line belonging to `helper0`'s doc train — reached
        // through a different chunk.
        let rendered = schedule_go_source(&roster_with(
            "// Alpha is exported and documented.
func Alpha(ctx context.Context, name string, options map[string]string) (*Result, error) { return nil, nil }
// Bravo is exported and documented.
func Bravo(ctx context.Context, name string, options map[string]string) (*Result, error) { return nil, nil }
var Charlie = 1
// helper0 explains itself.
",
            "",
        ));
        assert!(
            rendered.contains("var Charlie = 1"),
            "gate row must survive:\n{rendered}"
        );
        assert!(
            rendered.contains("helper0 explains itself"),
            "the remainder decl's doc row must survive:\n{rendered}"
        );
    }

    #[test]
    fn go_roster_ellipsis_never_lands_on_a_gate_decls_doc() {
        // The reverse seam: a body-less *remainder* declaration whose
        // next row is the gate declaration's doc comment.
        let rendered = schedule_go_source(&roster_with(
            "",
            "var quiet = 1\n// Alpha is the exported knob.\nvar Alpha = 2\n",
        ));
        assert!(
            rendered.contains("var Alpha = 2"),
            "gate row must survive:\n{rendered}"
        );
        assert!(
            rendered.contains("Alpha is the exported knob"),
            "the gate decl's doc row must survive:\n{rendered}"
        );
    }

    #[test]
    fn go_roster_rows_stay_disjoint_when_two_decls_share_a_row() {
        // Two top-level declarations on one source row land in different
        // chunks; only one roster may claim the row.
        let rendered = schedule_go_source(&roster_with("var Alpha = 1; var beta = 2\n", ""));
        assert!(
            rendered.contains("var Alpha = 1; var beta = 2"),
            "the shared row must render once:\n{rendered}"
        );
    }

    #[test]
    fn go_entry_slice_gates_an_oversize_roster_on_its_highest_rank_decls() {
        let batches = names_batches(&[("subject.go", oversize_roster_source())]);
        assert_eq!(batches.len(), 2, "roster splits into gate + remainder");
        // Exported beats documented-unexported beats bare unexported, so
        // the gate takes `Alpha` (line 4) and `zulu` (line 16) — not the
        // source-order prefix, and not a contiguous run.
        assert_eq!(batches[0].1, vec![4, 16], "gate slice: {batches:?}");
        assert!(
            batches[1].1.contains(&5) && !batches[1].1.contains(&4),
            "remainder holds the rest: {batches:?}"
        );
    }

    #[test]
    fn go_entry_slice_skips_a_top_ranked_decl_that_blows_the_ceiling() {
        // `Alpha` outranks everything — exported and documented — but its
        // signature alone renders over the gate ceiling, so the gate has
        // to fall through to the best-ranked declaration that fits.
        let params: String = (0..40)
            .map(|index| format!("argument{index} map[string]string, "))
            .collect();
        let mut src = format!(
            "package subject\n\n// Alpha is the documented entry point.\nfunc Alpha({params}) (*Result, error) {{ return nil, nil }}\n// Bravo is documented too.\nfunc Bravo(ctx context.Context) error {{ return nil }}\n"
        );
        for index in 0..10 {
            src.push_str(&format!(
                "func helper{index}(ctx context.Context, name string, options map[string]string) (*Result, error) {{ return nil, nil }}\n"
            ));
        }
        let batches = names_batches(&[("subject.go", src)]);
        assert_eq!(batches.len(), 2, "roster still splits: {batches:?}");
        assert_eq!(
            batches[0].1,
            vec![6],
            "gate is `Bravo`, not the over-ceiling `Alpha`: {batches:?}"
        );
        assert!(
            batches[1].1.contains(&4),
            "`Alpha` rides the remainder: {batches:?}"
        );
    }

    #[test]
    fn go_entry_slice_leaves_a_roster_that_already_fits_whole() {
        let batches = names_batches(&[(
            "subject.go",
            "package subject\n\nfunc Alpha() {}\nfunc Bravo() {}\n".to_string(),
        )]);
        assert_eq!(
            batches.len(),
            1,
            "small roster stays one batch: {batches:?}"
        );
    }

    #[test]
    fn go_entry_slice_is_confined_to_the_directorys_spine_file() {
        // Two files with identical oversize rosters: neither out-declares
        // the other, so no spine stands out and neither carves a gate.
        let batches = names_batches(&[
            ("subject.go", oversize_roster_source()),
            ("sibling.go", oversize_roster_source()),
        ]);
        assert_eq!(
            batches.len(),
            2,
            "one whole roster per file, no gate: {batches:?}"
        );
        assert!(
            batches.iter().all(|(chunk_index, _)| *chunk_index == 0),
            "no remainder chunks: {batches:?}"
        );
    }

    #[test]
    fn go_test_file_surfaces_only_test_benchmark_example_names() {
        let src = "\
package foo

import \"testing\"

func TestFoo(t *testing.T)        {}
func helperHelper(t *testing.T)   {}
func BenchmarkFoo(b *testing.B)   {}
func ExampleFoo()                 {}
";
        let (source, tree) = parse(src);
        let starts = collect_test_function_lines(&tree, &source);
        assert_eq!(starts, vec![5, 7, 8], "got {starts:?}");
    }

    #[test]
    fn go_mod_keeps_indirect_requires_replace_and_retract() {
        let src = "\
module example.com/foo

go 1.22

require (
\tgithub.com/x/y v1.0.0
\tgithub.com/x/z v0.5.0 // indirect
)

replace github.com/x/y => github.com/forked/y v2.0.0

retract v0.1.0
";
        let lines = gomod_lines(src);
        assert!(lines.contains(&1), "module clause kept");
        assert!(lines.contains(&5), "require ( kept");
        assert!(lines.contains(&6), "direct require kept");
        assert!(lines.contains(&7), "indirect require kept");
        assert!(lines.contains(&8), "require ) kept");
        assert!(lines.contains(&10), "replace kept");
        assert!(lines.contains(&12), "retract kept");
    }

    #[test]
    fn go_mod_identity_collects_module_go_and_toolchain_directives() {
        let src = "\
module example.com/foo

go 1.22

toolchain go1.22.5

require (
\tgithub.com/x/y v1.0.0
)

replace github.com/x/y => github.com/forked/y v2.0.0
";
        let lines = gomod_identity_lines(src);
        assert!(lines.contains(&1), "module clause kept");
        assert!(lines.contains(&3), "go version kept");
        assert!(lines.contains(&5), "toolchain kept");
        assert!(!lines.contains(&7), "require ( excluded from identity");
        assert!(!lines.contains(&8), "require body excluded from identity");
        assert!(
            !lines.contains(&11),
            "replace excluded from identity (not an identity directive)"
        );
    }

    #[test]
    fn go_mod_identity_handles_minimal_module() {
        let src = "module example.com/foo\n\ngo 1.22\n";
        let lines = gomod_identity_lines(src);
        assert!(lines.contains(&1));
        assert!(lines.contains(&3));
    }

    #[test]
    fn go_mod_emits_whole_file_when_no_indirect_lines_present() {
        let src = "module example.com/foo\n\ngo 1.22\n";
        let lines = gomod_lines(src);
        assert!(!lines.is_empty());
    }

    #[test]
    fn go_mod_over_cap_elides_indirect_require_tail() {
        let mut src = String::from(
            "\
module example.com/foo

go 1.22

require (
\tgithub.com/direct/a v1.0.0
",
        );
        let first_indirect_line = src.lines().count() + 1;
        for i in 0..65 {
            src.push_str(&format!("\tgithub.com/indirect/{i} v0.0.1 // indirect\n"));
        }
        let close_line = src.lines().count() + 1;
        src.push_str(")\n\n");
        let replace_line = src.lines().count() + 1;
        src.push_str("replace github.com/direct/a => ../a\n\n");
        let exclude_line = src.lines().count() + 1;
        src.push_str("exclude github.com/bad/module v1.0.0\n");

        let rendered = gomod_rendered_lines(&src);
        assert!(rendered.contains(&(1, Render::Full)), "module kept");
        assert!(rendered.contains(&(3, Render::Full)), "go directive kept");
        assert!(
            rendered.contains(&(5, Render::Full)),
            "require block opener kept"
        );
        assert!(rendered.contains(&(6, Render::Full)), "direct require kept");
        assert!(
            rendered.contains(&(close_line, Render::Full)),
            "require block closer kept"
        );
        assert!(
            rendered.contains(&(replace_line, Render::Full)),
            "replace directive kept"
        );
        assert!(
            rendered.contains(&(exclude_line, Render::Full)),
            "exclude directive kept"
        );
        assert!(
            !rendered.contains(&(first_indirect_line, Render::Full)),
            "indirect require must not render in full"
        );
        assert!(
            rendered.contains(&(first_indirect_line, Render::Ellipsis)),
            "indirect require tail should be represented by one ellipsis"
        );
    }

    #[test]
    fn go_real_dir_renders_seeded_files_and_skips_test_bodies() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("foo.go"),
            "package foo\n\n// Foo greets.\nfunc Foo(name string) string { return \"hi \" + name }\n",
        )
        .unwrap();
        std::fs::write(
            root.join("foo_test.go"),
            "package foo\n\nfunc TestFoo(t *testing.T) { _ = Foo(\"x\") }\n",
        )
        .unwrap();
        std::fs::write(root.join("go.mod"), "module example.com/foo\n\ngo 1.22\n").unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();
        assert!(
            rendered.contains("func Foo(name string)"),
            "expected the Go signature in rendered output:\n{rendered}",
        );

        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Go(GoKey::Decl { .. }))),
            "expected a Go::Decl batch; keys: {keys:?}"
        );
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Go(GoKey::GoMod { .. }))),
            "expected a GoMod batch; keys: {keys:?}"
        );

        let test_decls = keys
            .iter()
            .filter(|k| {
                matches!(k, BatchKey::Go(GoKey::Decl { file, .. }) if file.ends_with("foo_test.go"))
            })
            .count();
        assert_eq!(
            test_decls, 0,
            "expected no Go decl batches from *_test.go; keys: {keys:?}"
        );
    }
}
