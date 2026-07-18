//! Go walker. Per-file package-and-imports plus per-decl batches, plus
//! a `_test.go` test-name surface and a `go.mod` whole-file batch.
//!
//! Per-file keys:
//! - `PackageImports { file }`: `package …` clause + `import (…)` block.
//!   Plumbing batch.
//! - `DeclNames { file }`: surface listing of every top-level
//!   declaration's first line — visibility-blind catastrophic-omission
//!   hedge. Grouped `type ( … )` / `var ( … )` / `const ( … )` blocks
//!   contribute one entry per inner spec, so the hedge surfaces every
//!   exported name even when the whole-group `Decl` batch isn't
//!   scheduled.
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

use std::collections::HashSet;
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
    let (test_files, source_files): (Vec<_>, Vec<_>) = files_with_extension(dir, "go")
        .into_iter()
        .partition(|p| is_test_file(p));
    out.extend(expand_test_files(&test_files, ctx));
    out.extend(expand_source_files(&source_files, ctx));
    out
}

fn expand_gomod(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for (name, kind) in list_dir(dir) {
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
        if let Some(content) =
            single_file_lines_content(file, &source, collect_package_imports(&tree, &source))
        {
            out.push(Batch {
                key: GoKey::PackageImports { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: GoRole::PackageImports.value(file, ctx, entry_factor, 1.0),
            });
        }

        if decls.is_empty() {
            continue;
        }

        // Names-surface chunking: only oversized API files (both many
        // decls AND many lines) chunk. Smaller files emit their full
        // surface so it can land in one batch at small budgets.
        let chunk_count = if decls.len() > GO_DECL_NAMES_CHUNK_THRESHOLD
            && line_count > GO_DECL_NAMES_CHUNK_LINE_THRESHOLD
        {
            decls.len().div_ceil(GO_DECL_NAMES_CHUNK_SIZE)
        } else {
            1
        };
        let names_keys: Vec<GoKey> = (0..chunk_count)
            .map(|chunk_index| GoKey::DeclNames {
                file: file.clone(),
                chunk_index,
            })
            .collect();
        // chunk_count == 1: single batch covers all decls.
        let all_name_lines: HashSet<usize> = decls
            .iter()
            .flat_map(|(_, info)| info.name_lines.iter().copied())
            .collect();
        let names_lines_by_chunk: Vec<FileLines> = if chunk_count == 1 {
            vec![collect_decl_names_from(&decls, &all_name_lines)]
        } else {
            decls
                .chunks(GO_DECL_NAMES_CHUNK_SIZE)
                .map(|chunk| collect_decl_names_from(chunk, &all_name_lines))
                .collect()
        };
        for (chunk_index, names_lines) in names_lines_by_chunk.iter().enumerate() {
            if let Some(content) = single_file_lines_content(file, &source, names_lines.clone()) {
                out.push(Batch {
                    key: names_keys[chunk_index].clone().into(),
                    predecessor: None,
                    content,
                    value: GoRole::DeclNames.value(file, ctx, entry_factor, 1.0)
                        * names_surface_chunk_factor(chunk_index, chunk_count),
                });
            }
        }
        for (decl_index, (node, info)) in decls.iter().enumerate() {
            let chunk_index = decl_index / GO_DECL_NAMES_CHUNK_SIZE;
            let chunk_index = chunk_index.min(chunk_count - 1);
            let names_predecessor = BatchKey::Go(names_keys[chunk_index].clone());
            let chunk_names_lines = &names_lines_by_chunk[chunk_index];
            let decl_key = GoKey::Decl {
                file: file.clone(),
                start_line: info.start_line,
            };
            let decl_lines = collect_decl(info);
            let doc_lines = collect_doc_comments_above(*node, &source);
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
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "function_declaration" => {
                if let Some(info) = func_or_method_info(child, source, DeclKind::Func) {
                    out.push((child, info));
                }
            }
            "method_declaration" => {
                if let Some(info) = func_or_method_info(child, source, DeclKind::Method) {
                    out.push((child, info));
                }
            }
            "type_declaration" => out.push((child, grouped_type_info(child, source))),
            "var_declaration" => {
                out.push((child, grouped_value_info(child, source, DeclKind::Var)))
            }
            "const_declaration" => {
                out.push((child, grouped_value_info(child, source, DeclKind::Const)))
            }
            _ => continue,
        }
    }
    out
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
        struct_field_groups: Vec::new(),
    })
}

fn grouped_type_info(node: Node, source: &str) -> DeclInfo {
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
        let comment_rows = comment_only_rows(struct_body);
        let blank_groups: Vec<(usize, Vec<usize>)> = collect_blank_line_groups(struct_body, source)
            .into_iter()
            .filter_map(|(_, rows)| {
                let rows: Vec<usize> = rows
                    .into_iter()
                    .filter(|row| !comment_rows.contains(&(row - 1)))
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

/// One full + ellipsis pair per name line so a grouped block surfaces
/// every inner spec, not just the `type (` opener.
fn collect_decl_names_from(
    decls: &[(Node, DeclInfo)],
    all_name_lines: &HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for (_, info) in decls {
        for &line in &info.name_lines {
            full.push(line);
            let ellipsis_line = line + 1;
            if !all_name_lines.contains(&ellipsis_line) {
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
        let names = collect_decl_names_from(&decls, &all_name_lines);
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
