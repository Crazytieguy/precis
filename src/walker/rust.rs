//! Rust walker. Per-item pub-declaration batches plus file-scope
//! crate-doc / mod-use / impl-method-groups and cross-file macro surface.
//!
//! Per-file keys:
//! - `CrateDocLede { file }`: `//!` opening paragraph (entrypoints only)
//! - `ModUse { file }`: `use` / `mod` / `pub use` plumbing
//! - `PubItemNames { file }`: every pub item's first line as a surface
//!   listing — a cheap catastrophic-omission hedge when individual item
//!   decls don't all fit
//! - `MethodSigs { file }`: inherent + trait impl headers + method sigs
//!
//! Per-item keys (keyed by start line so each item has a distinct batch):
//! - `PubItem { file, start_line }`: one pub item's declaration (struct
//!   fields / enum variants / trait method sigs / fn signature; no rustdoc)
//! - `PubItemDoc { file, start_line }`: rustdoc above that item,
//!   predecessor = the matching `PubItem`
//!
//! Cross-file keys (scoped by source directory):
//! - `MacroNames { src_dir }`: exported macro name list
//! - `MacroBodies { src_dir }`: full bodies (predecessor: `MacroNames`)
//!
//! Parse trees are cached in [`WalkCtx`]; the same file parsed once powers
//! every Rust batch that touches it.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{BatchKey, FsKey, ResolvedBatch, RustKey, ValueSignals};
use crate::content::{BatchContent, Span};
use crate::value::depth_factor;

use super::{
    Candidate, FileLines, WalkCtx, build_file_spans, dedup_sorted, extend_span,
    fs::files_with_extension, push_rows, signature_end_row, single_file_lines_batch,
};

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
        return Vec::new();
    };
    let rust_files = files_with_extension(dir, "rs");
    if rust_files.is_empty() {
        return Vec::new();
    }
    let dir_depth = ctx.depth_from_root(dir);

    let mut out = Vec::new();

    for file in &rust_files {
        let ep = is_entrypoint_file(file);
        if ep {
            let lede = RustKey::CrateDocLede { file: file.clone() };
            out.push(candidate(
                lede.clone(),
                crate_doc_lede_signals(file, ctx),
                40,
            ));
            out.push(
                candidate(
                    RustKey::CrateDocBody { file: file.clone() },
                    crate_doc_body_signals(file, ctx),
                    200,
                )
                .with_predecessor(BatchKey::Rust(lede)),
            );
            out.push(candidate(
                RustKey::ModUse { file: file.clone() },
                mod_use_signals(file, ctx),
                60,
            ));
            out.push(candidate(
                RustKey::MethodSigs { file: file.clone() },
                method_sigs_signals(file, ctx),
                60,
            ));
        }

        // Per-item pub declarations. We need to read the file to find item
        // start lines — but we cache the parse tree in `WalkCtx`, so this
        // work is shared with later `materialize` calls. If a file has zero
        // pub items, no per-item candidates are emitted.
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let items = find_pub_item_starts(&tree, &source);
        if items.is_empty() {
            continue;
        }
        // Intentionally not modeled as a separate batch in current north stars
        // (they rank full bodies only). Reviewers will flag the resulting
        // "struct header + …" rendering as a partial body, which is correct
        // against the frozen NS. Planned: next NS-author pass will rank a
        // PubItemNames-equivalent as its own tier-1 location hint so
        // reviewer grading matches walker emission. Deferred to batch with
        // other NS-side changes before the next fixture round.
        let names_key = RustKey::PubItemNames { file: file.clone() };
        out.push(candidate(
            names_key.clone(),
            pub_item_names_signals(file, ctx),
            items.len() * 8,
        ));
        for item in &items {
            let key = RustKey::PubItem {
                file: file.clone(),
                start_line: item.start_line,
            };
            // PubItemNames is the parent surface listing; PubItem refines
            // that file's header lines with full content. PubItemDoc
            // refines PubItem with rustdoc.
            out.push(
                candidate(
                    key.clone(),
                    pub_item_signals(file, item.kind, item.surface, ctx),
                    item.estimated_cost(),
                )
                .with_predecessor(BatchKey::Rust(names_key.clone())),
            );
            out.push(
                candidate(
                    RustKey::PubItemDoc {
                        file: file.clone(),
                        start_line: item.start_line,
                    },
                    pub_item_doc_signals(file, item.kind, item.surface, ctx),
                    60,
                )
                .with_predecessor(BatchKey::Rust(key)),
            );
        }
    }

    // Cross-file macro batches, scoped to `dir` (non-recursive).
    let macro_names = RustKey::MacroNames {
        src_dir: dir.clone(),
    };
    out.push(candidate(
        macro_names.clone(),
        macro_names_signals(dir_depth),
        20,
    ));
    out.push(
        candidate(
            RustKey::MacroBodies {
                src_dir: dir.clone(),
            },
            macro_bodies_signals(dir_depth),
            60,
        )
        .with_predecessor(BatchKey::Rust(macro_names)),
    );

    out
}

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Rust(rk) = key else {
        return None;
    };
    match rk {
        RustKey::CrateDocLede { file } => mat_per_file(
            file,
            collect_module_doc_lede,
            crate_doc_lede_signals(file, ctx),
            ctx,
        ),
        RustKey::CrateDocBody { file } => mat_per_file(
            file,
            collect_module_doc_body,
            crate_doc_body_signals(file, ctx),
            ctx,
        ),
        RustKey::ModUse { file } => {
            mat_per_file(file, collect_mod_use, mod_use_signals(file, ctx), ctx)
        }
        RustKey::PubItemNames { file } => mat_per_file(
            file,
            collect_pub_item_names,
            pub_item_names_signals(file, ctx),
            ctx,
        ),
        RustKey::PubItem { file, start_line } => {
            let (source, tree) = parse_rust(ctx, file)?;
            let item = find_item_at(&tree, &source, *start_line)?;
            let lines = collect_pub_item(&tree, &source, *start_line);
            single_file_lines_batch(
                file,
                &source,
                lines,
                pub_item_signals(file, item.kind, item.surface, ctx),
            )
        }
        RustKey::PubItemDoc { file, start_line } => {
            let (source, tree) = parse_rust(ctx, file)?;
            let item = find_item_at(&tree, &source, *start_line)?;
            let lines = collect_pub_item_doc(&tree, &source, *start_line);
            single_file_lines_batch(
                file,
                &source,
                lines,
                pub_item_doc_signals(file, item.kind, item.surface, ctx),
            )
        }
        RustKey::MethodSigs { file } => mat_per_file(
            file,
            collect_method_sigs,
            method_sigs_signals(file, ctx),
            ctx,
        ),
        RustKey::MacroNames { src_dir } => mat_cross_file(
            src_dir,
            collect_macro_name_lines,
            macro_names_signals(ctx.depth_from_root(src_dir)),
            ctx,
        ),
        RustKey::MacroBodies { src_dir } => mat_cross_file(
            src_dir,
            collect_macro_bodies,
            macro_bodies_signals(ctx.depth_from_root(src_dir)),
            ctx,
        ),
    }
}

// --- candidate + signal helpers ---

fn candidate(rk: RustKey, signals: ValueSignals, cost_hint: usize) -> Candidate<BatchKey> {
    Candidate::new(rk.into(), signals, cost_hint)
}

/// Kind of a top-level pub item, used to weight its batch. Traits are the
/// load-bearing abstraction every backend implements; enums and structs
/// carry the data-model; free fns are the call surface. First-pass
/// ordering — calibrate against the north stars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemKind {
    Trait,
    Enum,
    Struct,
    Union,
    Fn,
    TypeAlias,
    Const,
    Static,
}

impl ItemKind {
    fn kind_weight(self) -> f64 {
        match self {
            ItemKind::Trait => 1.15,
            ItemKind::Enum => 1.1,
            ItemKind::Struct | ItemKind::Union => 1.0,
            ItemKind::Fn => 0.95,
            ItemKind::TypeAlias | ItemKind::Const | ItemKind::Static => 0.85,
        }
    }
}

#[derive(Debug, Clone)]
struct PubItemInfo {
    start_line: usize,
    kind: ItemKind,
    surface: ApiSurface,
    /// Rough upper bound of the rendered cost of just this item's decl —
    /// lines × avg 6 tokens, with a floor of 40 for a one-liner decl.
    line_span: usize,
}

impl PubItemInfo {
    fn estimated_cost(&self) -> usize {
        (self.line_span * 6).max(40)
    }
}

/// Whether a syntactically-public item is part of the external crate API.
/// `Public` (plain `pub`) competes for the budget at full weight;
/// `Restricted` (`pub(crate)` / `pub(super)` / `pub(self)` / `pub(in ...)`)
/// is compiler-visible inside the crate but not the external surface.
/// `doc_hidden` is orthogonal — public items can still be opted out of
/// rustdoc with `#[doc(hidden)]`. Together they produce a non-API factor
/// applied uniformly across the value channels in `pub_item_signals` /
/// `pub_item_doc_signals`.
#[derive(Debug, Clone, Copy)]
struct ApiSurface {
    visibility: Visibility,
    doc_hidden: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Visibility {
    Public,
    Restricted,
}

impl ApiSurface {
    /// Multiplier applied to all three value channels for non-API items.
    /// Calibrated against the divergence Sim metric across the 10
    /// fixtures: 0.4 per axis (visibility, doc_hidden) demotes
    /// non-API items meaningfully without dropping load-bearing
    /// internal types out of the schedule entirely. 0.16 stacks for
    /// items that are *both* restricted and `#[doc(hidden)]` (the
    /// most clearly internal class).
    fn factor(self) -> f64 {
        let v = match self.visibility {
            Visibility::Public => 1.0,
            Visibility::Restricted => 0.4,
        };
        let h = if self.doc_hidden { 0.4 } else { 1.0 };
        v * h
    }
}

fn item_kind_of(node: Node) -> Option<ItemKind> {
    Some(match node.kind() {
        "trait_item" => ItemKind::Trait,
        "enum_item" => ItemKind::Enum,
        "struct_item" => ItemKind::Struct,
        "union_item" => ItemKind::Union,
        "function_item" | "function_signature_item" => ItemKind::Fn,
        "type_item" => ItemKind::TypeAlias,
        "const_item" => ItemKind::Const,
        "static_item" => ItemKind::Static,
        _ => return None,
    })
}

fn find_pub_item_starts(tree: &Tree, source: &str) -> Vec<PubItemInfo> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            continue; // Handled by MacroNames/MacroBodies.
        }
        let Some(visibility) = item_visibility(child, source) else {
            continue;
        };
        let Some(kind) = item_kind_of(child) else {
            continue;
        };
        let start_line = child.start_position().row + 1;
        let end_line = item_end_line(child, source);
        out.push(PubItemInfo {
            start_line,
            kind,
            surface: ApiSurface {
                visibility,
                doc_hidden: has_doc_hidden(child, source),
            },
            line_span: end_line.saturating_sub(start_line) + 1,
        });
    }
    out
}

fn find_item_at(tree: &Tree, source: &str, start_line: usize) -> Option<PubItemInfo> {
    find_pub_item_starts(tree, source)
        .into_iter()
        .find(|i| i.start_line == start_line)
}

fn item_end_line(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    let internal = text.trim_end_matches(['\n', '\r']).split('\n').count();
    node.start_position().row + internal.max(1)
}

/// Files whose filename signals "crate entrypoint / main module surface".
fn is_entrypoint_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| matches!(n, "lib.rs" | "main.rs" | "mod.rs"))
}

fn entrypoint_boost(path: &Path) -> f64 {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    match name {
        "lib.rs" | "main.rs" => 1.4,
        "mod.rs" => 1.15,
        _ => 1.0,
    }
}

/// Depth factor for a file. Entrypoints are pinned to depth 1 so
/// `src/lib.rs` isn't penalized relative to root-depth content. All files
/// multiply by a non-essential-directory factor.
fn file_depth_factor(path: &Path, ctx: &WalkCtx) -> f64 {
    let depth = ctx.depth_from_root(path);
    let raw = if is_entrypoint_file(path) {
        depth_factor(depth.min(1))
    } else {
        depth_factor(depth)
    };
    raw * ctx.non_essential_factor(path)
}

fn crate_doc_lede_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.8 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.5,
        zero_tool_call_understanding: 0.9,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn crate_doc_body_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.35 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.75,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn mod_use_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.3 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.55,
        zero_tool_call_understanding: 0.3,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn pub_item_names_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    // Cheap surface listing — catastrophic-omission hedge. Ranks high
    // because missing it means the agent doesn't know items exist.
    // File-level visibility applies the same axis as `ApiSurface::factor`
    // (0.4 for Restricted) — a names listing of items that aren't on the
    // public API is structurally less valuable to the agent.
    let s = file_visibility_factor(file, ctx);
    ValueSignals {
        catastrophic_omission: (0.8 * entrypoint_boost(file) * s).min(1.0),
        follow_up_minimization: 0.6 * s,
        zero_tool_call_understanding: 0.35 * s,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn pub_item_signals(
    file: &Path,
    kind: ItemKind,
    surface: ApiSurface,
    ctx: &WalkCtx,
) -> ValueSignals {
    let k = kind.kind_weight();
    let s = effective_surface(surface, ctx, file).factor();
    ValueSignals {
        catastrophic_omission: (0.70 * k * s * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: (0.85 * k * s).min(1.0),
        zero_tool_call_understanding: 0.65 * s,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn pub_item_doc_signals(
    file: &Path,
    kind: ItemKind,
    surface: ApiSurface,
    ctx: &WalkCtx,
) -> ValueSignals {
    let k = kind.kind_weight();
    let s = effective_surface(surface, ctx, file).factor();
    ValueSignals {
        catastrophic_omission: (0.20 * k * s * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: (0.6 * k * s).min(1.0),
        zero_tool_call_understanding: 0.8 * s,
        depth_factor: file_depth_factor(file, ctx),
    }
}

/// Combine a per-item local `ApiSurface` with the file's effective
/// crate-visibility from `module_visibility`. `doc_hidden` passes through.
fn effective_surface(local: ApiSurface, ctx: &WalkCtx, file: &Path) -> ApiSurface {
    let visibility = match (local.visibility, module_visibility(ctx, file)) {
        (Visibility::Public, Visibility::Public) => Visibility::Public,
        _ => Visibility::Restricted,
    };
    ApiSurface {
        visibility,
        doc_hidden: local.doc_hidden,
    }
}

fn file_visibility_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    match module_visibility(ctx, file) {
        Visibility::Public => 1.0,
        Visibility::Restricted => 0.4,
    }
}

fn method_sigs_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.5 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.8,
        zero_tool_call_understanding: 0.4,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn macro_names_signals(depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.75,
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.4,
        depth_factor: depth_factor(depth),
    }
}

fn macro_bodies_signals(depth: usize) -> ValueSignals {
    // For macro-heavy crates (anyhow, log), the `#[macro_export]` bodies
    // are the crate's public API — on par with `PubItem`. Predecessor
    // edge to MacroNames orders them.
    ValueSignals {
        catastrophic_omission: 0.9,
        follow_up_minimization: 0.9,
        zero_tool_call_understanding: 0.6,
        depth_factor: depth_factor(depth),
    }
}

// --- parser ---

fn parse_rust(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_rust::LANGUAGE.into())
}

// --- shared materializer shapes ---

fn mat_per_file<F>(
    file: &Path,
    collect: F,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch>
where
    F: Fn(&Tree, &str) -> FileLines,
{
    let (source, tree) = parse_rust(ctx, file)?;
    let lines = collect(&tree, &source);
    single_file_lines_batch(file, &source, lines, signals)
}

fn mat_cross_file<F>(
    src_dir: &Path,
    collect: F,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch>
where
    F: Fn(&Tree, &str) -> FileLines,
{
    let rust_files = files_with_extension(src_dir, "rs");
    if rust_files.is_empty() {
        return None;
    }
    let mut all_spans: Vec<Span> = Vec::new();
    for file in &rust_files {
        let Some((source, tree)) = parse_rust(ctx, file) else {
            continue;
        };
        let lines = collect(&tree, &source);
        all_spans.extend(build_file_spans(file, &source, lines));
    }
    if all_spans.is_empty() {
        return None;
    }
    Some(ResolvedBatch {
        content: BatchContent::Lines { spans: all_spans },
        signals,
    })
}

// --- AST collectors ---

fn collect_module_doc_lede(tree: &Tree, source: &str) -> FileLines {
    FileLines::new(collect_module_doc_lines(tree, source, DocSection::Lede))
}

fn collect_module_doc_body(tree: &Tree, source: &str) -> FileLines {
    FileLines::new(collect_module_doc_lines(tree, source, DocSection::Body))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocSection {
    /// First paragraph of `//!` only.
    Lede,
    /// Everything after the first paragraph.
    Body,
}

/// Collect line numbers belonging to the crate-`//!` block, split at the
/// first Markdown heading (`//! #`, `//! ##`, …). Regular license-header
/// `// comments` above the `//!` run are skipped. The lede is everything
/// from the first `//!` up to (but not including) the first heading line;
/// the body is from the heading onwards. If no heading is present, the
/// whole block is the lede.
///
/// Doctest-hidden lines are stripped *before* the lede/body split: a
/// crate doc that opens with a fenced Rust example whose first hidden
/// line happens to read `# use crate::X;` would otherwise have its
/// boundary land on the (invisible) doctest setup, mis-splitting around
/// the real heading.
fn collect_module_doc_lines(tree: &Tree, source: &str, section: DocSection) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut all: Vec<usize> = Vec::new();
    for child in root.children(&mut cursor) {
        if matches!(child.kind(), "line_comment" | "block_comment") {
            if is_module_doc_comment(child, source) {
                extend_span(&mut all, child, source);
            }
            continue;
        }
        break;
    }
    if all.is_empty() {
        return Vec::new();
    }
    let all = strip_hidden_doctest_lines(all, source);
    let src_lines: Vec<&str> = source.lines().collect();
    let heading_pos = all.iter().position(|&n| {
        src_lines.get(n - 1).is_some_and(|t| {
            t.trim_start()
                .trim_start_matches("//!")
                .trim_start()
                .starts_with('#')
        })
    });
    match (section, heading_pos) {
        (DocSection::Lede, Some(idx)) => all[..idx].to_vec(),
        (DocSection::Body, Some(idx)) => all[idx..].to_vec(),
        (DocSection::Lede, None) => all,
        (DocSection::Body, None) => Vec::new(),
    }
}

/// Drop rustdoc doctest-hidden lines. In a `///` or `//!` rustdoc block,
/// lines whose first non-whitespace token is `# ` or a bare `#`, while
/// inside a Rust fenced code block (` ``` ` or ` ~~~ ` with empty/`rust`/
/// `no_run`/`ignore`/`compile_fail`/`should_panic`/`edition*` info-string),
/// are scaffolding rustdoc strips from the rendered HTML. Keeping them in a
/// token-budgeted summary spends real tokens on content the human reader
/// of the docs never sees.
///
/// Operates on a sorted list of 1-based source line numbers, all expected
/// to belong to one contiguous rustdoc block. Walks the lines linearly with
/// a tiny fence state machine. Lines that aren't doc-prefixed (`///` or
/// `//!`) are passed through unchanged so block doc comments (`/** */`) and
/// non-doc inputs are unaffected. Consistent with existing line-level
/// filtering in `mod_use` / `build_file_spans`; preserves honest rendering
/// (output is still a verbatim subset of the source — just a smaller one).
fn strip_hidden_doctest_lines(lines: Vec<usize>, source: &str) -> Vec<usize> {
    if lines.is_empty() {
        return lines;
    }
    let src_lines: Vec<&str> = source.lines().collect();

    let mut state = FenceState::Outside;
    let mut out = Vec::with_capacity(lines.len());

    for n in lines {
        let Some(raw) = src_lines.get(n.saturating_sub(1)) else {
            out.push(n);
            continue;
        };
        let after_ws = raw.trim_start();
        let stripped = after_ws
            .strip_prefix("//!")
            .or_else(|| after_ws.strip_prefix("///"));
        let Some(content) = stripped else {
            out.push(n);
            continue;
        };
        // Strip up to one space after the prefix (rustdoc's leading-space
        // convention for the comment body).
        let content = content.strip_prefix(' ').unwrap_or(content);

        if let Some((kind, info)) = open_fence(content) {
            state = match state {
                FenceState::Outside => FenceState::Inside {
                    kind,
                    hides: is_rust_lang(info),
                },
                FenceState::Inside { kind: open, .. } if open == kind => FenceState::Outside,
                // Mismatched-kind fence inside another fence: rustdoc treats
                // this as literal content. Stay in current state.
                other => other,
            };
            out.push(n);
            continue;
        }

        if let FenceState::Inside { hides: true, .. } = state {
            let trimmed = content.trim_start();
            if trimmed == "#" || trimmed.starts_with("# ") {
                continue;
            }
        }
        out.push(n);
    }
    out
}

#[derive(Copy, Clone, PartialEq)]
enum FenceKind {
    Backtick,
    Tilde,
}

#[derive(Copy, Clone, PartialEq)]
enum FenceState {
    Outside,
    Inside { kind: FenceKind, hides: bool },
}

/// Returns `Some((kind, info_string))` if `content` (the post-prefix
/// remainder of a doc line, with one optional leading space stripped) opens
/// or closes a code fence. The info string is the trailing text after the
/// fence delimiter.
fn open_fence(content: &str) -> Option<(FenceKind, &str)> {
    let trimmed = content.trim_start();
    if let Some(rest) = trimmed.strip_prefix("```") {
        Some((FenceKind::Backtick, rest))
    } else if let Some(rest) = trimmed.strip_prefix("~~~") {
        Some((FenceKind::Tilde, rest))
    } else {
        None
    }
}

/// Whether a code fence's info string identifies a Rust block. Empty info
/// string defaults to Rust (rustdoc convention). The info-string's first
/// comma-separated token decides; case-insensitive.
fn is_rust_lang(info: &str) -> bool {
    let token = info
        .trim()
        .split([',', ' ', '\t'])
        .next()
        .unwrap_or("")
        .trim();
    if token.is_empty() {
        return true;
    }
    const RUST_ATTRS: &[&str] = &[
        "rust",
        "no_run",
        "ignore",
        "compile_fail",
        "should_panic",
        "edition2015",
        "edition2018",
        "edition2021",
        "edition2024",
    ];
    RUST_ATTRS.iter().any(|a| token.eq_ignore_ascii_case(a))
}

fn collect_mod_use(tree: &Tree, _source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "use_declaration" => push_rows(
                &mut full,
                child.start_position().row,
                signature_end_row(child),
            ),
            "mod_item" => {
                let sig_end = signature_end_row(child);
                push_rows(&mut full, child.start_position().row, sig_end);
                if child.child_by_field_name("body").is_some() {
                    ellipses.push(sig_end + 2);
                }
            }
            _ => {}
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Header lines for every top-level pub item (name + first line only, with
/// an ellipsis marker where the body would be). Surface listing — see
/// `PubItemNames`.
fn collect_pub_item_names(tree: &Tree, source: &str) -> FileLines {
    let items = find_pub_item_starts(tree, source);
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for item in &items {
        full.push(item.start_line);
        ellipses.push(item.start_line + 1);
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Lines for a single pub item's decl at `start_line`. For struct/enum/
/// trait/union: whole item (fields/variants/method sigs). For fn: signature
/// with body-elision marker. No outer rustdoc — that's `PubItemDoc`.
fn collect_pub_item(tree: &Tree, source: &str, start_line: usize) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        if item_visibility(child, source).is_none() {
            return FileLines::new(Vec::new());
        }
        let mut full = Vec::new();
        let mut ellipses = Vec::new();
        match child.kind() {
            "function_item" | "function_signature_item" => {
                let sig_end = signature_end_row(child);
                push_rows(&mut full, child.start_position().row, sig_end);
                if child.child_by_field_name("body").is_some() {
                    ellipses.push(sig_end + 2);
                }
            }
            "struct_item" | "enum_item" | "trait_item" | "union_item" | "type_item"
            | "const_item" | "static_item" => {
                extend_span(&mut full, child, source);
            }
            _ => {}
        }
        return FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses));
    }
    FileLines::new(Vec::new())
}

/// Outer rustdoc (`///` / `/** */`) immediately preceding the item at
/// `start_line`. Returns empty when the item has no outer doc.
fn collect_pub_item_doc(tree: &Tree, source: &str, start_line: usize) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        let mut out = Vec::new();
        collect_outer_docs_above(child, source, &mut out);
        let out = strip_hidden_doctest_lines(dedup_sorted(out), source);
        return FileLines::new(out);
    }
    FileLines::new(Vec::new())
}

fn collect_method_sigs(tree: &Tree, _source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "impl_item" {
            continue;
        }
        let sig_end = signature_end_row(child);
        push_rows(&mut full, child.start_position().row, sig_end);
        let Some(body) = child.child_by_field_name("body") else {
            continue;
        };
        let mut body_cursor = body.walk();
        for inner in body.children(&mut body_cursor) {
            if matches!(inner.kind(), "function_item" | "function_signature_item") {
                let inner_end = signature_end_row(inner);
                push_rows(&mut full, inner.start_position().row, inner_end);
                if inner.child_by_field_name("body").is_some() {
                    ellipses.push(inner_end + 2);
                }
            }
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

fn collect_macro_name_lines(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    let mut ellipses = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition" && has_macro_export(child, source) {
            let name_row = child.start_position().row;
            push_rows(&mut out, name_row, name_row);
            ellipses.push(name_row + 2);
        }
    }
    FileLines::new(out).with_ellipses(ellipses)
}

fn collect_macro_bodies(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "macro_definition"
            && has_macro_export(child, source)
            && !is_underscore_private(child, source)
        {
            extend_span(&mut out, child, source);
        }
    }
    FileLines::new(dedup_sorted(out))
}

/// `__` prefix by convention marks an internal dispatcher macro.
fn is_underscore_private(node: Node, source: &str) -> bool {
    if let Some(name) = node.child_by_field_name("name") {
        let text = &source[name.start_byte()..name.end_byte()];
        return text.starts_with("__");
    }
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|c| {
        c.kind() == "identifier" && {
            let text = &source[c.start_byte()..c.end_byte()];
            text.starts_with("__")
        }
    })
}

// --- Cross-file module visibility ---

/// Effective crate-visibility of `file` as a Rust module. `Public` if a
/// chain of `pub mod` declarations connects it back to `src/lib.rs`;
/// `Restricted` otherwise.
///
/// Fallback when `file` isn't in the computed map:
/// - empty map (no `<root>/src/lib.rs`, e.g. binary-only crate or
///   non-Rust seed) → `Public`. The local-syntactic check is the only
///   signal we have.
/// - non-empty map but `file` lives under `<root>/src/` →
///   `Restricted`. The resolver doesn't honor `#[path = ...]` or descend
///   into inline `pub mod` blocks with extern children, so a file
///   under `src/` not in the map is most plausibly behind one of those
///   gaps. Defaulting to `Restricted` keeps private-by-construction
///   items from sliding back to full public weight on a resolver miss.
/// - non-empty map, `file` outside `src/` (tests/, examples/, build.rs)
///   → `Public`. Those paths already get the non-essential-dir discount.
///
/// The map is computed lazily on first access and cached for the run.
/// Re-export tracking handles same-module `pub use self::path::...`
/// declarations: each declared-and-resolvable mod along the chain is
/// lifted to `Public` (gated segment-by-segment on a real `mod x;`
/// declaration in the current file). `pub use crate::...` and
/// `pub use super::...` paths are not modeled.
fn module_visibility(ctx: &WalkCtx, file: &Path) -> Visibility {
    let map = ctx.rust_module_visibility_map(|| compute_module_visibility(ctx));
    if let Some(&v) = map.get(file) {
        return v;
    }
    if !map.is_empty() && file.starts_with(ctx.root().join("src")) {
        Visibility::Restricted
    } else {
        Visibility::Public
    }
}

fn compute_module_visibility(ctx: &WalkCtx) -> HashMap<PathBuf, Visibility> {
    let mut out: HashMap<PathBuf, Visibility> = HashMap::new();
    let lib = ctx.root().join("src/lib.rs");
    if !lib.exists() {
        return out;
    }
    let mut stack: Vec<(PathBuf, Visibility)> = vec![(lib, Visibility::Public)];
    while let Some((file, vis)) = stack.pop() {
        let upgrade = match out.get(&file).copied() {
            None => true,
            Some(Visibility::Public) => false, // already at the best vis
            Some(Visibility::Restricted) => matches!(vis, Visibility::Public),
        };
        if !upgrade {
            continue;
        }
        out.insert(file.clone(), vis);
        let Some((source, tree)) = parse_rust(ctx, &file) else {
            continue;
        };
        for decl in mod_decls(&tree, &source) {
            let child_vis = if decl.is_pub && matches!(vis, Visibility::Public) {
                Visibility::Public
            } else {
                Visibility::Restricted
            };
            let Some(child) = resolve_mod(&file, &decl.name) else {
                continue;
            };
            stack.push((child, child_vis));
        }
        if matches!(vis, Visibility::Public) {
            let parent_decls = mod_decls(&tree, &source);
            for path in pub_use_self_paths(&tree, &source) {
                lift_reexport_chain(&file, &parent_decls, &path, ctx, &mut stack);
            }
        }
    }
    out
}

/// Walk a `pub use self::seg1::seg2::...` re-export chain from
/// `parent_file`, lifting each declared-and-resolvable segment to
/// `Public`. Stops at the first segment that isn't a top-level `mod`
/// declaration in the current file. The mod-declaration gate keeps
/// chain-walked file lifts pinned to real `mod x;` decls — a same-named
/// file that exists for unrelated reasons (orphan, `#[path]`-mounted)
/// must not be promoted just because it's on disk.
fn lift_reexport_chain(
    parent_file: &Path,
    parent_decls: &[ModDecl],
    path: &[String],
    ctx: &WalkCtx,
    stack: &mut Vec<(PathBuf, Visibility)>,
) {
    let mut current = parent_file.to_path_buf();
    let mut decls: Vec<ModDecl> = parent_decls.to_vec();
    let mut first = true;
    for seg in path {
        if !first {
            let Some((src, tree)) = parse_rust(ctx, &current) else {
                return;
            };
            decls = mod_decls(&tree, &src);
        }
        first = false;
        if !decls.iter().any(|d| d.name == *seg) {
            return;
        }
        let Some(child) = resolve_mod(&current, seg) else {
            return;
        };
        stack.push((child.clone(), Visibility::Public));
        current = child;
    }
}

#[derive(Debug, Clone)]
struct ModDecl {
    name: String,
    is_pub: bool,
}

/// Top-level external `mod x;` declarations on this file. Inline
/// `mod x { ... }` blocks (with a body) are skipped — their items live in
/// the same source file and are subject to the per-item local syntactic
/// check, not cross-file resolution.
fn mod_decls(tree: &Tree, source: &str) -> Vec<ModDecl> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "mod_item" {
            continue;
        }
        if child.child_by_field_name("body").is_some() {
            continue;
        }
        let Some(name_node) = child.child_by_field_name("name") else {
            continue;
        };
        let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
        let is_pub = matches!(item_visibility(child, source), Some(Visibility::Public));
        out.push(ModDecl { name, is_pub });
    }
    out
}

/// Top-level `pub use self::...` re-exports as segment-paths *after*
/// the leading `self`. One declaration may yield multiple paths when
/// the use-clause is grouped or wildcarded
/// (`pub use self::{a::X, b::*}` → `[["a", "X"], ["b"]]`). Restricted
/// re-exports (`pub(crate) use`, no leading `self`) are skipped.
fn pub_use_self_paths(tree: &Tree, source: &str) -> Vec<Vec<String>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "use_declaration" {
            continue;
        }
        if !matches!(item_visibility(child, source), Some(Visibility::Public)) {
            continue;
        }
        let mut paths = Vec::new();
        if let Some(clause) = use_declaration_clause(child) {
            collect_use_paths(clause, source, &Vec::new(), &mut paths);
        }
        for p in paths {
            if p.first().is_some_and(|s| s == "self") && p.len() > 1 {
                out.push(p[1..].to_vec());
            }
        }
    }
    out
}

/// The argument clause of a `use_declaration` — the part after `pub?
/// use` and before the trailing `;`. Skips token children.
fn use_declaration_clause(use_decl: Node) -> Option<Node> {
    let mut cur = use_decl.walk();
    use_decl
        .children(&mut cur)
        .find(|c| !matches!(c.kind(), "visibility_modifier" | "use" | ";"))
}

/// Recursively expand a use-clause into one path per enumerated leaf.
/// `prefix` carries the segments accumulated from outer
/// `scoped_use_list` / `scoped_identifier` wrappers; each leaf appends
/// its own segments to `prefix` and pushes one entry into `out`.
fn collect_use_paths(node: Node, source: &str, prefix: &[String], out: &mut Vec<Vec<String>>) {
    match node.kind() {
        "scoped_use_list" => {
            let mut new_prefix = prefix.to_vec();
            let mut path_done = false;
            let mut cur = node.walk();
            for c in node.children(&mut cur) {
                if !path_done && append_path_segment(c, source, &mut new_prefix) {
                    path_done = true;
                } else if c.kind() == "use_list" {
                    expand_use_list(c, source, &new_prefix, out);
                }
            }
        }
        "use_wildcard" => {
            let mut new_prefix = prefix.to_vec();
            let mut cur = node.walk();
            for c in node.children(&mut cur) {
                append_path_segment(c, source, &mut new_prefix);
            }
            out.push(new_prefix);
        }
        "use_as_clause" => {
            let mut cur = node.walk();
            for c in node.children(&mut cur) {
                let mut segments = prefix.to_vec();
                if append_path_segment(c, source, &mut segments) {
                    out.push(segments);
                    return;
                }
            }
        }
        "use_list" => expand_use_list(node, source, prefix, out),
        _ => {
            let mut segments = prefix.to_vec();
            if append_path_segment(node, source, &mut segments) {
                out.push(segments);
            }
        }
    }
}

fn expand_use_list(node: Node, source: &str, prefix: &[String], out: &mut Vec<Vec<String>>) {
    let mut cur = node.walk();
    for item in node.children(&mut cur) {
        if matches!(item.kind(), "{" | "}" | ",") {
            continue;
        }
        collect_use_paths(item, source, prefix, out);
    }
}

/// Append a path-root node's segments to `out`. Handles the four leaf
/// shapes that can appear at the head of a use-clause: `scoped_identifier`
/// (recursed via [`flatten_scoped_id`]), the `self` / `crate` / `super`
/// path roots, and a bare `identifier`. Returns whether anything was
/// appended — callers use the return to track when a `scoped_use_list`'s
/// path prefix has been consumed.
fn append_path_segment(node: Node, source: &str, out: &mut Vec<String>) -> bool {
    match node.kind() {
        "scoped_identifier" => {
            flatten_scoped_id(node, source, out);
            true
        }
        "self" | "crate" | "super" => {
            out.push(node.kind().to_string());
            true
        }
        "identifier" => {
            out.push(source[node.start_byte()..node.end_byte()].to_string());
            true
        }
        _ => false,
    }
}

/// Flatten a `scoped_identifier` chain (`self::a::b::C` /
/// `a::b::C` / `crate::x`) into segment strings.
fn flatten_scoped_id(node: Node, source: &str, out: &mut Vec<String>) {
    let mut cur = node.walk();
    for c in node.children(&mut cur) {
        append_path_segment(c, source, out);
    }
}

/// Resolve a `mod x;` declaration in `parent_file` to the file that
/// `x` lives in, following Rust 2018+ module resolution. `#[path = ...]`
/// is not honored. Returns `None` if neither candidate exists on disk.
fn resolve_mod(parent_file: &Path, name: &str) -> Option<PathBuf> {
    let parent_dir = parent_file.parent()?;
    let lookup_dir = match parent_file.file_name().and_then(|n| n.to_str()) {
        Some("lib.rs" | "main.rs" | "mod.rs") => parent_dir.to_path_buf(),
        _ => {
            let stem = parent_file.file_stem()?.to_str()?;
            parent_dir.join(stem)
        }
    };
    let flat = lookup_dir.join(format!("{name}.rs"));
    if flat.exists() {
        return Some(flat);
    }
    let nested = lookup_dir.join(name).join("mod.rs");
    if nested.exists() {
        return Some(nested);
    }
    None
}

// --- AST predicates ---

/// Classify the visibility of a top-level item. `None` for items without
/// any `visibility_modifier` (i.e. private — these aren't `PubItem`
/// candidates). Plain `pub` → `Public`; any restricted form
/// (`pub(crate)` / `pub(super)` / `pub(self)` / `pub(in path)`) →
/// `Restricted`. Classification is by the trimmed text of the modifier
/// node, so future grammar additions to the restricted forms degrade
/// gracefully (anything that isn't exactly `pub` is treated as
/// restricted).
fn item_visibility(node: Node, source: &str) -> Option<Visibility> {
    let mut cursor = node.walk();
    let modifier = node
        .children(&mut cursor)
        .find(|c| c.kind() == "visibility_modifier")?;
    let text = source[modifier.start_byte()..modifier.end_byte()].trim();
    Some(if text == "pub" {
        Visibility::Public
    } else {
        Visibility::Restricted
    })
}

/// Walk the outer attributes attached to `node` (preceding siblings,
/// skipping comments) and return `true` if any of them satisfies `pred`.
/// `pred` receives the inner `attribute` node — the structured payload —
/// not the wrapping `attribute_item`. Stops at the first non-attr/
/// non-comment sibling.
fn any_outer_attribute<F>(node: Node, pred: F) -> bool
where
    F: Fn(Node) -> bool,
{
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "attribute_item" => {
                let mut acur = prev.walk();
                if prev
                    .children(&mut acur)
                    .any(|c| c.kind() == "attribute" && pred(c))
                {
                    return true;
                }
                cur = prev.prev_sibling();
            }
            "line_comment" | "block_comment" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
    false
}

/// Check whether `#[doc(hidden)]` is attached as a *direct* outer
/// attribute on `node`. Conditional forms like
/// `#[cfg_attr(..., doc(hidden))]` don't count — the outer attribute's
/// path there is `cfg_attr`, not `doc`. Treats conditionally-hidden
/// items as on-surface.
fn has_doc_hidden(node: Node, source: &str) -> bool {
    any_outer_attribute(node, |attr| is_doc_hidden_attribute(attr, source))
}

/// Match an `attribute` node whose path is `doc` and whose token-tree is
/// exactly `(hidden)`. Uses the structured AST instead of substring
/// matching so `cfg_attr(..., doc(hidden))` (whose outer path is
/// `cfg_attr`) doesn't false-fire.
fn is_doc_hidden_attribute(attr: Node, source: &str) -> bool {
    let mut cur = attr.walk();
    let mut children = attr.children(&mut cur);
    let path = children.next();
    let payload = children.next();
    let (Some(path), Some(payload)) = (path, payload) else {
        return false;
    };
    if path.kind() != "identifier" || &source[path.start_byte()..path.end_byte()] != "doc" {
        return false;
    }
    if payload.kind() != "token_tree" {
        return false;
    }
    let mut pcur = payload.walk();
    let idents: Vec<Node> = payload
        .children(&mut pcur)
        .filter(|c| c.kind() == "identifier")
        .collect();
    matches!(
        idents.as_slice(),
        [only] if &source[only.start_byte()..only.end_byte()] == "hidden"
    )
}

fn has_macro_export(node: Node, source: &str) -> bool {
    any_outer_attribute(node, |attr| {
        let mut cur = attr.walk();
        attr.children(&mut cur).any(|c| {
            c.kind() == "identifier" && &source[c.start_byte()..c.end_byte()] == "macro_export"
        })
    })
}

fn is_module_doc_comment(node: Node, source: &str) -> bool {
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("//!") || text.starts_with("/*!")
}

fn is_outer_doc_comment(node: Node, source: &str) -> bool {
    let text = &source[node.start_byte()..node.end_byte()];
    text.starts_with("///") || text.starts_with("/**")
}

fn collect_outer_docs_above(node: Node, source: &str, out: &mut Vec<usize>) {
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "line_comment" | "block_comment" if is_outer_doc_comment(prev, source) => {
                extend_span(out, prev, source);
                cur = prev.prev_sibling();
            }
            "attribute_item" => cur = prev.prev_sibling(),
            _ => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn parse(src: &str) -> Tree {
        let mut p = Parser::new();
        p.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        p.parse(src, None).unwrap()
    }

    fn surfaces(src: &str) -> Vec<(String, ApiSurface)> {
        let tree = parse(src);
        let mut out = Vec::new();
        let mut cursor = tree.root_node().walk();
        for child in tree.root_node().children(&mut cursor) {
            let Some(visibility) = item_visibility(child, src) else {
                continue;
            };
            let surface = ApiSurface {
                visibility,
                doc_hidden: has_doc_hidden(child, src),
            };
            let head = src[child.start_byte()..child.end_byte()]
                .lines()
                .next()
                .unwrap_or("")
                .to_string();
            out.push((head, surface));
        }
        out
    }

    #[test]
    fn rust_visibility_classifies_pub_crate_super_self_in() {
        let src = r#"
pub struct A;
pub(crate) struct B;
pub(super) struct C;
pub(self) struct D;
pub(in crate::x) struct E;
struct F;
"#;
        let s = surfaces(src);
        let by_head: std::collections::HashMap<_, _> = s
            .iter()
            .map(|(h, surf)| (h.clone(), surf.visibility))
            .collect();
        assert_eq!(by_head["pub struct A;"], Visibility::Public);
        assert_eq!(by_head["pub(crate) struct B;"], Visibility::Restricted);
        assert_eq!(by_head["pub(super) struct C;"], Visibility::Restricted);
        assert_eq!(by_head["pub(self) struct D;"], Visibility::Restricted);
        assert_eq!(
            by_head["pub(in crate::x) struct E;"],
            Visibility::Restricted
        );
        // Private items have no visibility_modifier and are filtered out.
        assert!(!by_head.contains_key("struct F;"));
    }

    #[test]
    fn rust_doc_hidden_attaches_only_to_direct_doc_attribute() {
        let src = r#"
#[doc(hidden)]
pub struct A;

#[cfg_attr(feature = "f", doc(hidden))]
pub struct B;

#[doc = "regular doc"]
pub struct C;

#[doc(hidden)]
#[allow(dead_code)]
pub struct D;
"#;
        let surfaces = surfaces(src);
        let by_head: std::collections::HashMap<_, _> = surfaces
            .iter()
            .map(|(h, surf)| (h.clone(), surf.doc_hidden))
            .collect();
        // Direct #[doc(hidden)] fires.
        assert!(by_head["pub struct A;"]);
        // cfg_attr-wrapped doc(hidden) doesn't fire (outer attr is cfg_attr).
        assert!(!by_head["pub struct B;"]);
        // #[doc = "..."] doesn't fire.
        assert!(!by_head["pub struct C;"]);
        // doc(hidden) stacked with another attribute still fires.
        assert!(by_head["pub struct D;"]);
    }

    #[test]
    fn rust_api_surface_factor_composes_visibility_and_doc_hidden() {
        let pub_visible = ApiSurface {
            visibility: Visibility::Public,
            doc_hidden: false,
        };
        let pub_hidden = ApiSurface {
            visibility: Visibility::Public,
            doc_hidden: true,
        };
        let crate_visible = ApiSurface {
            visibility: Visibility::Restricted,
            doc_hidden: false,
        };
        let crate_hidden = ApiSurface {
            visibility: Visibility::Restricted,
            doc_hidden: true,
        };
        assert_eq!(pub_visible.factor(), 1.0);
        assert!((pub_hidden.factor() - 0.4).abs() < 1e-9);
        assert!((crate_visible.factor() - 0.4).abs() < 1e-9);
        assert!((crate_hidden.factor() - 0.16).abs() < 1e-9);
    }

    #[test]
    fn rust_mod_decls_extracts_top_level_external_modules() {
        let src = r#"
mod private_a;
pub mod public_b;
pub(crate) mod restricted_c;
mod inline_d { pub fn x() {} }
pub mod inline_e { }
"#;
        let tree = parse(src);
        let decls = mod_decls(&tree, src);
        let by_name: std::collections::HashMap<_, _> =
            decls.iter().map(|d| (d.name.clone(), d.is_pub)).collect();
        assert_eq!(by_name.get("private_a"), Some(&false));
        assert_eq!(by_name.get("public_b"), Some(&true));
        assert_eq!(by_name.get("restricted_c"), Some(&false)); // pub(crate) ≠ public surface
        // Inline mods (with body) are not external — their items live in
        // the same file and aren't subject to cross-file resolution.
        assert!(!by_name.contains_key("inline_d"));
        assert!(!by_name.contains_key("inline_e"));
    }

    #[test]
    fn rust_resolve_mod_2018_paths() {
        let dir = tempdir();
        let src_dir = dir.path().join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        std::fs::write(src_dir.join("lib.rs"), "").unwrap();
        std::fs::write(src_dir.join("flat.rs"), "").unwrap();
        std::fs::create_dir_all(src_dir.join("nested")).unwrap();
        std::fs::write(src_dir.join("nested").join("mod.rs"), "").unwrap();
        std::fs::create_dir_all(src_dir.join("flat")).unwrap();
        std::fs::write(src_dir.join("flat").join("child.rs"), "").unwrap();

        // From lib.rs: lookup dir is src/; finds flat.rs.
        let r = resolve_mod(&src_dir.join("lib.rs"), "flat").unwrap();
        assert_eq!(r, src_dir.join("flat.rs"));

        // From lib.rs: nested/mod.rs.
        let r = resolve_mod(&src_dir.join("lib.rs"), "nested").unwrap();
        assert_eq!(r, src_dir.join("nested").join("mod.rs"));

        // From flat.rs (a non-mod.rs file): lookup dir is src/flat/.
        let r = resolve_mod(&src_dir.join("flat.rs"), "child").unwrap();
        assert_eq!(r, src_dir.join("flat").join("child.rs"));

        // From nested/mod.rs: lookup dir is src/nested/, child not present.
        assert!(resolve_mod(&src_dir.join("nested").join("mod.rs"), "missing").is_none());
    }

    #[test]
    fn rust_compute_module_visibility_propagates_through_pub_mod_chain() {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "pub mod public_chain;\nmod private_chain;\n",
        )
        .unwrap();
        std::fs::write(
            src.join("public_chain.rs"),
            "pub mod grandchild;\nmod gc_private;\n",
        )
        .unwrap();
        std::fs::create_dir_all(src.join("public_chain")).unwrap();
        std::fs::write(src.join("public_chain").join("grandchild.rs"), "").unwrap();
        std::fs::write(src.join("public_chain").join("gc_private.rs"), "").unwrap();
        std::fs::write(src.join("private_chain.rs"), "pub mod gc_under_private;\n").unwrap();
        std::fs::create_dir_all(src.join("private_chain")).unwrap();
        std::fs::write(src.join("private_chain").join("gc_under_private.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);

        assert_eq!(map.get(&src.join("lib.rs")), Some(&Visibility::Public));
        assert_eq!(
            map.get(&src.join("public_chain.rs")),
            Some(&Visibility::Public)
        );
        assert_eq!(
            map.get(&src.join("public_chain").join("grandchild.rs")),
            Some(&Visibility::Public)
        );
        // Private mod inside a public chain → Restricted.
        assert_eq!(
            map.get(&src.join("public_chain").join("gc_private.rs")),
            Some(&Visibility::Restricted)
        );
        // Private chain itself → Restricted.
        assert_eq!(
            map.get(&src.join("private_chain.rs")),
            Some(&Visibility::Restricted)
        );
        // pub mod inside a private chain doesn't lift visibility back to
        // Public — once Restricted, always Restricted on that path.
        assert_eq!(
            map.get(&src.join("private_chain").join("gc_under_private.rs")),
            Some(&Visibility::Restricted)
        );
    }

    #[test]
    fn rust_compute_module_visibility_skips_when_no_lib_rs() {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("main.rs"), "mod helpers;\n").unwrap();
        std::fs::write(src.join("helpers.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        // No lib.rs → empty map → callers default to Public (preserve
        // local-syntactic-only behavior for binary-only crates).
        assert!(map.is_empty());
    }

    #[test]
    fn rust_module_visibility_falls_back_to_restricted_for_unresolved_files_under_src() {
        // Resolver miss simulator: a `#[path = "actual.rs"]` mod or a
        // private inline `pub mod x { mod y; }` puts files into `src/`
        // that the resolver doesn't follow. Defaulting these to Public
        // would silently restore full public-API weight; the safer
        // fallback is Restricted.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        // Map seeds from lib.rs, which only declares public_child.
        std::fs::write(src.join("lib.rs"), "pub mod public_child;\n").unwrap();
        std::fs::write(src.join("public_child.rs"), "").unwrap();
        // Simulate a #[path]-mounted file the resolver didn't follow.
        std::fs::write(src.join("hidden_via_path_attr.rs"), "").unwrap();
        // Files outside src/ (tests/, examples/, build.rs) should still
        // default to Public — they get the non-essential-dir discount.
        std::fs::write(dir.path().join("build.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        // Reachable via `pub mod` chain → Public.
        assert_eq!(
            module_visibility(&ctx, &src.join("public_child.rs")),
            Visibility::Public
        );
        // Resolver miss under src/ → Restricted (safer default).
        assert_eq!(
            module_visibility(&ctx, &src.join("hidden_via_path_attr.rs")),
            Visibility::Restricted
        );
        // Outside src/ → Public.
        assert_eq!(
            module_visibility(&ctx, &dir.path().join("build.rs")),
            Visibility::Public
        );
    }

    #[test]
    fn rust_module_visibility_no_lib_rs_defaults_public_everywhere() {
        // Binary-only crate: no `<root>/src/lib.rs` → empty map →
        // every file defaults to Public. Preserves the local-syntactic
        // check as the only signal.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("main.rs"), "").unwrap();
        std::fs::write(src.join("helpers.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        assert_eq!(
            module_visibility(&ctx, &src.join("main.rs")),
            Visibility::Public
        );
        assert_eq!(
            module_visibility(&ctx, &src.join("helpers.rs")),
            Visibility::Public
        );
    }

    #[test]
    fn rust_pub_use_self_paths_handles_grammar_shapes() {
        let src = r#"
pub use self::error::Error;
pub use self::key::{Key, ToKey};
pub use self::value::*;
pub use self::source::Visitor as V;
pub use self::a::b::Deep;
pub use self::single_segment;
pub use self::{first_grp::X, second_grp::*};
pub use self::nested::{a::{x, y}, b};
pub use crate::foo::Bar;
pub use other::baz::Qux;
pub(crate) use self::restricted::Hidden;
use self::not_pub::Hidden;
"#;
        let tree = parse(src);
        let mut paths = pub_use_self_paths(&tree, src);
        paths.sort();
        let expected: Vec<Vec<String>> = [
            vec!["a", "b", "Deep"],
            vec!["error", "Error"],
            vec!["first_grp", "X"],
            vec!["key", "Key"],
            vec!["key", "ToKey"],
            vec!["nested", "a", "x"],
            vec!["nested", "a", "y"],
            vec!["nested", "b"],
            vec!["second_grp"],
            vec!["single_segment"],
            vec!["source", "Visitor"],
            vec!["value"],
        ]
        .into_iter()
        .map(|p| p.into_iter().map(String::from).collect())
        .collect();
        let mut expected_sorted = expected;
        expected_sorted.sort();
        assert_eq!(paths, expected_sorted);
    }

    #[test]
    fn rust_pub_use_self_lifts_private_child_to_public() {
        // log/src/kv/mod.rs shape — `mod x;` + `pub use self::x::Item;`
        // is the motivating pattern for cross-file re-export tracking.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "pub mod kv;\n").unwrap();
        std::fs::create_dir_all(src.join("kv")).unwrap();
        std::fs::write(
            src.join("kv").join("mod.rs"),
            "mod error;\npub use self::error::Error;\n",
        )
        .unwrap();
        std::fs::write(src.join("kv").join("error.rs"), "pub struct Error;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(
            map.get(&src.join("kv").join("error.rs")),
            Some(&Visibility::Public),
            "error.rs should be lifted to Public via pub use re-export",
        );
    }

    #[test]
    fn rust_pub_use_self_does_not_lift_when_parent_restricted() {
        // The parent's re-export only crosses to external API when the
        // parent itself is on the external surface.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod parent;\n").unwrap();
        std::fs::create_dir_all(src.join("parent")).unwrap();
        std::fs::write(
            src.join("parent").join("mod.rs"),
            "mod child;\npub use self::child::X;\n",
        )
        .unwrap();
        std::fs::write(src.join("parent").join("child.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(
            map.get(&src.join("parent").join("child.rs")),
            Some(&Visibility::Restricted),
        );
    }

    #[test]
    fn rust_pub_use_self_handles_use_list_and_wildcard() {
        for re_export in ["pub use self::child::{A, B};", "pub use self::child::*;"] {
            let dir = tempdir();
            let src = dir.path().join("src");
            std::fs::create_dir_all(&src).unwrap();
            std::fs::write(src.join("lib.rs"), format!("mod child;\n{re_export}\n")).unwrap();
            std::fs::write(src.join("child.rs"), "pub struct A; pub struct B;\n").unwrap();

            let ctx = WalkCtx::new(dir.path().to_path_buf());
            let map = compute_module_visibility(&ctx);
            assert_eq!(
                map.get(&src.join("child.rs")),
                Some(&Visibility::Public),
                "re-export `{re_export}` should lift child.rs to Public",
            );
        }
    }

    #[test]
    fn rust_pub_use_self_handles_grouped_under_self() {
        // Outer `scoped_use_list` whose path-prefix is the bare `self`
        // token (no `scoped_identifier`) — verifies the path_done logic.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod a;\nmod b;\npub use self::{a::X, b::*};\n",
        )
        .unwrap();
        std::fs::write(src.join("a.rs"), "pub struct X;\n").unwrap();
        std::fs::write(src.join("b.rs"), "pub struct Y;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("a.rs")), Some(&Visibility::Public));
        assert_eq!(map.get(&src.join("b.rs")), Some(&Visibility::Public));
    }

    #[test]
    fn rust_pub_use_self_handles_use_as_clause() {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod child;\npub use self::child::Item as Renamed;\n",
        )
        .unwrap();
        std::fs::write(src.join("child.rs"), "pub struct Item;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("child.rs")), Some(&Visibility::Public),);
    }

    #[test]
    fn rust_pub_use_self_walks_chain_through_private_mods() {
        // The chain walks past private `mod b;` to lift b.rs because
        // `pub use self::a::b::X` makes `b`'s contents publicly reachable
        // even though `a.rs` only declares it privately.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod a;\npub use self::a::b::X;\n").unwrap();
        std::fs::create_dir_all(src.join("a")).unwrap();
        std::fs::write(src.join("a").join("mod.rs"), "mod b;\n").unwrap();
        std::fs::write(src.join("a").join("b.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(
            map.get(&src.join("a").join("mod.rs")),
            Some(&Visibility::Public),
            "a should be lifted",
        );
        assert_eq!(
            map.get(&src.join("a").join("b.rs")),
            Some(&Visibility::Public),
            "b should be lifted via chain walk through private mod",
        );
    }

    #[test]
    fn rust_pub_use_self_does_not_lift_undeclared_same_named_file() {
        // Without the `mod b;` gate, an orphan `a/b.rs` on disk would
        // be silently lifted by `pub use self::a::b::X;` even though
        // it isn't actually a child mod of `a`.
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod a;\npub use self::a::b::X;\n").unwrap();
        std::fs::create_dir_all(src.join("a")).unwrap();
        std::fs::write(src.join("a").join("mod.rs"), "// no mod b\n").unwrap();
        std::fs::write(src.join("a").join("b.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(
            map.get(&src.join("a").join("mod.rs")),
            Some(&Visibility::Public),
        );
        assert!(!map.contains_key(&src.join("a").join("b.rs")));
        assert_eq!(
            module_visibility(&ctx, &src.join("a").join("b.rs")),
            Visibility::Restricted,
        );
    }

    #[test]
    fn rust_pub_use_without_self_prefix_does_not_lift() {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod foo;\npub use crate::foo::Bar;\npub use my_crate::baz::Qux;\n",
        )
        .unwrap();
        std::fs::write(src.join("foo.rs"), "pub struct Bar;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("foo.rs")), Some(&Visibility::Restricted),);
    }

    #[test]
    fn rust_pub_use_with_inner_visibility_does_not_lift() {
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "mod child;\npub(crate) use self::child::X;\n",
        )
        .unwrap();
        std::fs::write(src.join("child.rs"), "pub struct X;\n").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(
            map.get(&src.join("child.rs")),
            Some(&Visibility::Restricted),
        );
    }

    #[test]
    fn rust_compute_module_visibility_dual_decl_merges_to_public() {
        // log/src/kv/mod.rs pattern: cfg-gated `mod x;` + `pub mod x;`
        // pointing at the same file. We don't track cfg gates — taking
        // "any path makes it Public" is the conservative call (preserves
        // surface signal for the kv_unstable feature configuration).
        let dir = tempdir();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "mod child;\npub mod child;\n").unwrap();
        std::fs::write(src.join("child.rs"), "").unwrap();

        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let map = compute_module_visibility(&ctx);
        assert_eq!(map.get(&src.join("child.rs")), Some(&Visibility::Public));
    }

    /// Minimal scratch-dir helper. Avoids pulling in the `tempfile` crate
    /// for two tests; `process::id` keeps the path unique enough across
    /// concurrent test runs.
    fn tempdir() -> TempDir {
        let base = std::env::temp_dir().join(format!(
            "precis-rust-walker-test-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        TempDir(base)
    }

    struct TempDir(PathBuf);
    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    // ---- strip_hidden_doctest_lines ----

    /// Run the filter on every non-blank line of `src` and return only the
    /// surviving lines, joined back into a string. Stable shape for
    /// snapshot-style assertions of multi-fence behavior.
    fn strip_via(src: &str) -> String {
        let all: Vec<usize> = (1..=src.lines().count()).collect();
        let kept = strip_hidden_doctest_lines(all, src);
        let lines: Vec<&str> = src.lines().collect();
        kept.into_iter()
            .map(|n| lines[n - 1])
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn rust_strip_hidden_lines_default_fence() {
        let src =
            "//! ```\n//! # use anyhow::Result;\n//! fn ok() -> Result<()> { Ok(()) }\n//! ```\n";
        assert_eq!(
            strip_via(src),
            "//! ```\n//! fn ok() -> Result<()> { Ok(()) }\n//! ```",
        );
    }

    #[test]
    fn rust_strip_hidden_lines_rust_attrs() {
        for attr in [
            "rust",
            "no_run",
            "ignore",
            "compile_fail",
            "should_panic",
            "edition2018",
            "edition2021",
        ] {
            let src = format!("/// ```{attr}\n/// # let x = 1;\n/// real\n/// ```\n");
            let got = strip_via(&src);
            assert!(
                !got.contains("# let x"),
                "fence `{attr}` should drop hidden line; got: {got}",
            );
            assert!(got.contains("/// real"));
        }
    }

    #[test]
    fn rust_strip_hidden_lines_non_rust_fence_keeps_hash() {
        for lang in ["console", "text", "toml", "bash", "json"] {
            let src = format!("/// ```{lang}\n/// # not hidden\n/// ```\n");
            let got = strip_via(&src);
            assert!(
                got.contains("# not hidden"),
                "fence `{lang}` should keep `# `; got: {got}",
            );
        }
    }

    #[test]
    fn rust_strip_hidden_lines_heading_outside_fence() {
        let src = "//! # Real heading\n//! prose\n";
        assert_eq!(strip_via(src), src.trim_end());
    }

    #[test]
    fn rust_strip_hidden_lines_double_hash_in_rust_fence() {
        // `## foo` inside a Rust fence is rendered (rustdoc consumes one #),
        // so the line must be kept.
        let src = "/// ```\n/// ## foo\n/// ```\n";
        assert_eq!(strip_via(src), src.trim_end());
    }

    #[test]
    fn rust_strip_hidden_lines_attribute_lines_kept() {
        // `#[derive(...)]` and `#![cfg(...)]` don't start with `# ` (no
        // space), so they're code, not hidden.
        let src = "/// ```\n/// #[derive(Debug)]\n/// #![allow(unused)]\n/// struct S;\n/// ```\n";
        assert_eq!(strip_via(src), src.trim_end());
    }

    #[test]
    fn rust_strip_hidden_lines_lone_hash_dropped() {
        let src = "/// ```\n/// #\n/// real\n/// ```\n";
        let got = strip_via(src);
        assert!(!got.contains("/// #\n"), "lone `#` should drop; got: {got}");
        assert!(got.contains("/// real"));
    }

    #[test]
    fn rust_strip_hidden_lines_tilde_fence() {
        let src = "/// ~~~\n/// # let x = 1;\n/// real\n/// ~~~\n";
        let got = strip_via(src);
        assert!(
            !got.contains("# let x"),
            "tilde-fence should drop `# `; got: {got}"
        );
    }

    #[test]
    fn rust_strip_hidden_lines_mismatched_fence_kind_does_not_close() {
        // Inside a backtick Rust fence, a `~~~` line is literal content (not
        // a closer). Subsequent `# ` lines stay hidden.
        let src = "/// ```\n/// # hidden 1\n/// ~~~ inline\n/// # hidden 2\n/// ```\n";
        let got = strip_via(src);
        assert!(!got.contains("# hidden 1"));
        assert!(!got.contains("# hidden 2"));
        assert!(got.contains("~~~ inline"));
    }

    #[test]
    fn rust_strip_hidden_lines_multi_fence_state_machine() {
        // Lede: prose, then a Rust fence, then a console fence, then a
        // tilde-rust fence. Hidden lines in Rust fences only.
        let src = "\
//! intro
//! ```
//! # hidden a
//! visible a
//! ```
//! between
//! ```console
//! # kept-console
//! ```
//! ~~~rust
//! # hidden b
//! visible b
//! ~~~
//! tail
";
        let got = strip_via(src);
        assert!(!got.contains("# hidden a"));
        assert!(!got.contains("# hidden b"));
        assert!(got.contains("# kept-console"));
        assert!(got.contains("visible a"));
        assert!(got.contains("visible b"));
        assert!(got.contains("//! tail"));
    }

    #[test]
    fn rust_strip_hidden_lines_works_with_inner_doc() {
        // Same body for both `///` and `//!` prefix forms.
        let outer = "/// ```\n/// # hidden\n/// real\n/// ```\n";
        let inner = "//! ```\n//! # hidden\n//! real\n//! ```\n";
        assert!(!strip_via(outer).contains("# hidden"));
        assert!(!strip_via(inner).contains("# hidden"));
    }

    #[test]
    fn rust_module_doc_split_handles_fence_before_first_real_heading() {
        // Crate doc opens with a fenced Rust example that has a `# ` hidden
        // line. The first *real* heading is `# Public API` further down. Split
        // must land on the real heading: lede gets the prose and intro fence;
        // body starts at `# Public API`.
        let src = "\
//! Crate one-liner.
//!
//! ```
//! # use crate::Foo;
//! Foo::bar();
//! ```
//!
//! # Public API
//!
//! Detail.

pub fn anchor() {}
";
        let tree = parse(src);
        let lede = collect_module_doc_lines(&tree, src, DocSection::Lede);
        let body = collect_module_doc_lines(&tree, src, DocSection::Body);
        let line_text = |n: usize| src.lines().nth(n - 1).unwrap_or("");

        // Lede contains the prose, the fence delimiters, and the visible
        // body line — but not the `# use crate::Foo;` doctest scaffolding.
        let lede_text: Vec<&str> = lede.iter().map(|&n| line_text(n)).collect();
        assert!(
            lede_text.iter().any(|t| t.contains("Crate one-liner")),
            "lede missing tagline: {lede_text:?}",
        );
        assert!(
            lede_text.iter().any(|t| t.contains("Foo::bar()")),
            "lede missing visible code line: {lede_text:?}",
        );
        assert!(
            !lede_text.iter().any(|t| t.contains("# use crate::Foo")),
            "lede leaked hidden doctest line: {lede_text:?}",
        );
        assert!(
            !lede_text.iter().any(|t| t.contains("# Public API")),
            "lede crossed the real heading: {lede_text:?}",
        );

        // Body starts at the real heading and contains the prose after it.
        let body_text: Vec<&str> = body.iter().map(|&n| line_text(n)).collect();
        assert!(
            body_text
                .first()
                .is_some_and(|t| t.contains("# Public API")),
            "body should start at real heading; got {body_text:?}",
        );
        assert!(body_text.iter().any(|t| t.contains("Detail.")));
    }

    #[test]
    fn rust_strip_hidden_lines_block_comment_passthrough() {
        // Block doc comments `/** */` aren't filtered (helper passes
        // through any line whose prefix isn't `///` or `//!`).
        let src = "/** ```\n# would-be-hidden-but-not-handled\nreal\n``` */\n";
        let got = strip_via(src);
        assert!(got.contains("# would-be-hidden-but-not-handled"));
    }
}
