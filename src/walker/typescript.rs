//! TypeScript / TSX walker. Per-export decl batches plus file-scope
//! module-doc / imports / export-name surface.
//!
//! Per-file keys:
//! - `ModuleDocLede { file }`: top-of-file `/** */` JSDoc block (entrypoints
//!   only — files like `index.ts`, `main.ts`, `mod.ts`)
//! - `Imports { file }`: `import` declarations + bare `export … from`
//!   re-exports — plumbing, not items
//! - `ExportNames { file }`: every top-level export's first line as a
//!   surface listing — catastrophic-omission hedge
//!
//! Per-item keys (keyed by start line):
//! - `Export { file, start_line }`: one top-level export's declaration
//!   (interface fields / type alias / class header + member sigs / fn
//!   signature / const-assignment line; no JSDoc)
//! - `ExportDoc { file, start_line }`: JSDoc above that export,
//!   predecessor = the matching `Export`
//! - `ExportMember { file, start_line, member_start_line }`: one member
//!   surface of an exported JavaScript class, predecessor = the matching
//!   class `Export`.
//! - `ExportBody { file, start_line, body_start_line }`: body slice
//!   (brace-stripped) of a function, class, or `const X = <fn-init>`
//!   export, predecessor = the matching `Export`.
//! - `ModuleItem { file, start_line }`: non-exported top-level declaration
//!   surface for module-private classes, helpers, type aliases, and consts.
//! - `ModuleItemBody { file, start_line, body_start_line }`: body slice of
//!   a non-exported top-level declaration. Large bodies split by top-level
//!   statement so inner regions can schedule independently. Predecessor =
//!   the matching `ModuleItem`.
//!
//! Module-private `const X = <fn-init>` declarations whose name is
//! re-exported via a top-level `export { X }` / `export { X as Y }`
//! clause are *synthesized* into the `Export` triple at the const's
//! own start_line, so callers see the same surface as if `X` had been
//! written `export const X = …`. Type-only re-exports
//! (`export type { X }` / `export { type X }`) are excluded — they
//! don't expose the runtime value.
//!
//! `.ts`, `.tsx`, `.js`, `.mjs`, and `.cjs` are handled through the
//! TypeScript grammar family. Parse trees are cached in [`WalkCtx`].

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, TsKey};
use crate::value::{
    NAMES_SURFACE_CHUNK_SIZE, mix_signals, names_surface_chunk_count, names_surface_chunk_factor,
    names_surface_chunk_index, reexport_import_chunk_factor,
};

use super::import_chunks::{
    ImportGroup, REEXPORT_IMPORT_MAX_OTHER_LINES, REEXPORT_IMPORT_MAX_OTHER_STATEMENTS,
    groups_to_file_lines, node_line_count, push_import_group, should_chunk_import_groups,
};
use super::{
    BodyPart, FileLines, WalkCtx, body_part_value_factor, build_per_file_content, dedup_sorted,
    extend_span, file_depth_factor,
    fs::{JS_MODULE_ENTRYPOINT_FILES, files_with_any_extension, is_source_dir},
    name_of, node_end_row_trimmed, push_rows, signature_end_row, single_file_lines_content,
    statement_block_parts,
};

// TS/JS catalog files can expose many same-file body refinements. The first
// four body segments keep full value because they usually cover the main
// component/function bodies; later segments are commonly nested helper detail.
const FULL_VALUE_BODY_SEGMENTS_PER_FILE: usize = 4;
// Keep late body segments schedulable as last-resort detail, but make their
// value/cost ratio lose to broader structural candidates in budget pressure.
const LATE_BODY_SEGMENT_VALUE_FACTOR: f64 = 0.05;
const JS_CLASS_MEMBER_SPLIT_MIN: usize = 12;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let js_like_files = files_with_any_extension(dir, &["ts", "tsx", "js", "mjs", "cjs"]);
    if js_like_files.is_empty() {
        return Vec::new();
    }
    // Nested source dirs need index gating so child batches do not crowd the parent surface.
    let module_entrypoint = (!is_source_dir(dir) || ctx.depth_from_root(dir) > 1)
        .then(|| module_entrypoint_file(&js_like_files))
        .flatten();
    let module_entrypoint_gate = module_entrypoint
        .as_ref()
        .map(|file| import_gate_for_file(file, ctx));

    let mut out = Vec::new();
    for file in &js_like_files {
        let ep = is_entrypoint_file(file);
        let js_factor = js_value_factor(file, ctx);
        let module_predecessor = module_entrypoint_gate
            .as_ref()
            .filter(|_| module_entrypoint.as_deref() != Some(file.as_path()))
            .cloned();

        if ep
            && let Some(content) =
                build_per_file_content(file, ctx, parse_ts, collect_module_doc_lede)
        {
            out.push(Batch {
                key: TsKey::ModuleDocLede { file: file.clone() }.into(),
                predecessor: module_predecessor.clone(),
                content,
                value: module_doc_lede_value(file, ctx, js_factor),
            });
        }

        if let Some((source, tree)) = parse_ts(ctx, file) {
            if let Some(chunks) = collect_reexport_import_chunks(file, &tree, &source) {
                let chunk_count = chunks.len();
                for (chunk_index, lines) in chunks.into_iter().enumerate() {
                    let Some(content) = single_file_lines_content(file, &source, lines) else {
                        continue;
                    };
                    out.push(Batch {
                        key: TsKey::ImportChunk {
                            file: file.clone(),
                            chunk_index,
                        }
                        .into(),
                        predecessor: module_predecessor.clone(),
                        content,
                        value: imports_chunk_value(file, ctx, chunk_index, chunk_count, js_factor),
                    });
                }
            } else if let Some(content) =
                single_file_lines_content(file, &source, collect_imports(&tree, &source))
            {
                out.push(Batch {
                    key: TsKey::Imports { file: file.clone() }.into(),
                    predecessor: module_predecessor.clone(),
                    content,
                    value: imports_value(file, ctx, js_factor),
                });
            }
        } else if let Some(content) = build_per_file_content(file, ctx, parse_ts, collect_imports) {
            out.push(Batch {
                key: TsKey::Imports { file: file.clone() }.into(),
                predecessor: module_predecessor.clone(),
                content,
                value: imports_value(file, ctx, js_factor),
            });
        }

        let Some((source, tree)) = parse_ts(ctx, file) else {
            continue;
        };
        let src_lines: Vec<&str> = source.lines().collect();
        let exports = find_export_starts(file, &tree, &source, &src_lines);
        let per_export_factor = type_machinery_factor(file, &exports);
        let export_start_lines: HashSet<_> = exports.iter().map(|item| item.start_line).collect();
        let mut body_segment_index = 0usize;
        if !exports.is_empty() {
            let chunk_count = names_surface_chunk_count(exports.len());
            let export_count = exports.len();
            let type_only_export_count = exports.iter().filter(|item| item.is_type_only).count();
            let has_split_js_class_export = is_js_file(file)
                && exports
                    .iter()
                    .any(|item| should_split_js_class_export(file, item));
            let names_predecessors: Vec<_> = (0..chunk_count)
                .map(|chunk_index| {
                    BatchKey::Typescript(TsKey::ExportNames {
                        file: file.clone(),
                        chunk_index,
                        export_count,
                        type_only_export_count,
                    })
                })
                .collect();
            for (chunk_index, chunk) in exports.chunks(NAMES_SURFACE_CHUNK_SIZE).enumerate() {
                let Some(content) = single_file_lines_content(
                    file,
                    &source,
                    collect_export_names_from(chunk, &export_start_lines),
                ) else {
                    continue;
                };
                out.push(Batch {
                    key: names_predecessors[chunk_index].clone(),
                    predecessor: module_predecessor.clone(),
                    content,
                    value: export_names_value(
                        file,
                        ctx,
                        chunk_index,
                        chunk_count,
                        js_factor,
                        has_split_js_class_export,
                    ),
                });
            }
            for (item_index, item) in exports.iter().enumerate() {
                let names_predecessor =
                    names_predecessors[names_surface_chunk_index(item_index)].clone();
                let export_key = TsKey::Export {
                    file: file.clone(),
                    start_line: item.start_line,
                };
                let split_js_class = should_split_js_class_export(file, item);
                if let Some(content) = single_file_lines_content(
                    file,
                    &source,
                    if split_js_class {
                        class_header_surface_lines(item.anchor, item.decl, &source)
                    } else {
                        decl_surface_lines(item.kind, item.anchor, item.decl, &source)
                    },
                ) {
                    out.push(Batch {
                        key: export_key.clone().into(),
                        predecessor: Some(names_predecessor.clone()),
                        content,
                        value: export_value(file, item.kind, ctx, js_factor) * per_export_factor,
                    });
                }
                let export_predecessor = BatchKey::Typescript(export_key);
                if split_js_class {
                    for member in &item.class_members {
                        let member_key = TsKey::ExportMember {
                            file: file.clone(),
                            start_line: item.start_line,
                            member_start_line: member.start_line,
                        };
                        if let Some(content) =
                            single_file_lines_content(file, &source, member.lines.clone())
                        {
                            out.push(Batch {
                                key: member_key.clone().into(),
                                predecessor: Some(export_predecessor.clone()),
                                content,
                                value: export_member_value(file, item.kind, ctx, js_factor)
                                    * per_export_factor,
                            });
                        }
                        let member_predecessor = BatchKey::Typescript(member_key);
                        let mut emit_ctx = ExportBodyEmitCtx {
                            file,
                            source: &source,
                            ctx,
                            js_factor,
                            per_export_factor,
                            body_segment_index: &mut body_segment_index,
                        };
                        emit_export_body_parts(
                            &mut out,
                            &mut emit_ctx,
                            item,
                            member.body_parts.clone(),
                            &member_predecessor,
                        );
                    }
                }
                let mut doc_lines = Vec::new();
                collect_jsdoc_above(
                    item.anchor,
                    &source,
                    &mut doc_lines,
                    is_entrypoint_file(file),
                );
                if let Some(content) = single_file_lines_content(
                    file,
                    &source,
                    FileLines::new(dedup_sorted(doc_lines)),
                ) {
                    out.push(Batch {
                        key: TsKey::ExportDoc {
                            file: file.clone(),
                            start_line: item.start_line,
                        }
                        .into(),
                        predecessor: Some(export_predecessor.clone()),
                        content,
                        value: export_doc_value(file, item.kind, ctx, js_factor)
                            * per_export_factor,
                    });
                }
                if !split_js_class && !item.body_parts.is_empty() {
                    let mut emit_ctx = ExportBodyEmitCtx {
                        file,
                        source: &source,
                        ctx,
                        js_factor,
                        per_export_factor,
                        body_segment_index: &mut body_segment_index,
                    };
                    emit_export_body_parts(
                        &mut out,
                        &mut emit_ctx,
                        item,
                        item.body_parts.clone(),
                        &export_predecessor,
                    );
                }
            }
        }
        let module_items = find_module_items(&tree, &source, &src_lines, &export_start_lines);
        let emit_private_nonclass =
            is_entrypoint_file(file) && (is_tsx_file(file) || is_js_file(file));
        for item in module_items {
            if !emit_private_nonclass && !matches!(item.kind, ItemKind::Class) {
                continue;
            }
            let item_key = TsKey::ModuleItem {
                file: file.clone(),
                start_line: item.start_line,
            };
            if let Some(content) = single_file_lines_content(file, &source, item.lines) {
                out.push(Batch {
                    key: item_key.clone().into(),
                    predecessor: module_predecessor.clone(),
                    content,
                    value: module_item_value(file, item.kind, ctx, js_factor) * per_export_factor,
                });
            }
            if !item.body_parts.is_empty() {
                let item_predecessor = BatchKey::Typescript(item_key);
                let parts = item.body_parts;
                let part_value_factor = body_part_value_factor(parts.len());
                for part in parts {
                    let Some(body_start_line) = part.start_line() else {
                        continue;
                    };
                    let Some(content) =
                        single_file_lines_content(file, &source, FileLines::new(part.lines))
                    else {
                        continue;
                    };
                    out.push(Batch {
                        key: TsKey::ModuleItemBody {
                            file: file.clone(),
                            start_line: item.start_line,
                            body_start_line,
                        }
                        .into(),
                        predecessor: Some(item_predecessor.clone()),
                        content,
                        value: module_item_body_value(file, item.kind, ctx, js_factor)
                            * per_export_factor
                            * part_value_factor
                            * body_segment_value_factor(body_segment_index),
                    });
                    body_segment_index += 1;
                }
            }
        }
    }

    out
}

fn import_gate_for_file(file: &Path, ctx: &WalkCtx) -> BatchKey {
    if would_chunk_reexport_imports(file, ctx) {
        BatchKey::Typescript(TsKey::ImportChunk {
            file: file.to_path_buf(),
            chunk_index: 0,
        })
    } else {
        BatchKey::Typescript(TsKey::Imports {
            file: file.to_path_buf(),
        })
    }
}

fn would_chunk_reexport_imports(file: &Path, ctx: &WalkCtx) -> bool {
    is_entrypoint_file(file)
        && parse_ts(ctx, file)
            .and_then(|(source, tree)| collect_reexport_import_groups(&tree, &source))
            .is_some_and(|groups| should_chunk_import_groups(&groups))
}

struct ExportBodyEmitCtx<'a, 'b> {
    file: &'b Path,
    source: &'b str,
    ctx: &'b WalkCtx,
    js_factor: f64,
    per_export_factor: f64,
    body_segment_index: &'a mut usize,
}

fn emit_export_body_parts(
    out: &mut Vec<Batch<BatchKey>>,
    emit: &mut ExportBodyEmitCtx<'_, '_>,
    item: &ExportInfo<'_>,
    parts: Vec<BodyPart>,
    predecessor: &BatchKey,
) {
    let part_value_factor = body_part_value_factor(parts.len());
    for part in parts {
        let Some(body_start_line) = part.start_line() else {
            continue;
        };
        let Some(content) =
            single_file_lines_content(emit.file, emit.source, FileLines::new(part.lines))
        else {
            continue;
        };
        out.push(Batch {
            key: TsKey::ExportBody {
                file: emit.file.to_path_buf(),
                start_line: item.start_line,
                body_start_line,
            }
            .into(),
            predecessor: Some(predecessor.clone()),
            content,
            value: export_body_value(emit.file, item.kind, emit.ctx, emit.js_factor)
                * emit.per_export_factor
                * part_value_factor
                * body_segment_value_factor(*emit.body_segment_index),
        });
        *emit.body_segment_index += 1;
    }
}

fn body_segment_value_factor(body_segment_index: usize) -> f64 {
    if body_segment_index < FULL_VALUE_BODY_SEGMENTS_PER_FILE {
        1.0
    } else {
        LATE_BODY_SEGMENT_VALUE_FACTOR
    }
}

fn module_entrypoint_file(files: &[PathBuf]) -> Option<PathBuf> {
    files
        .iter()
        .find(|file| is_module_bundle_entrypoint(file))
        .cloned()
}

fn is_module_bundle_entrypoint(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| JS_MODULE_ENTRYPOINT_FILES.contains(&name))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemKind {
    Interface,
    TypeAlias,
    Class,
    Enum,
    Function,
    Const,
    /// `export default …` where the default is an expression (or function
    /// expression / class expression). Treated like a class/function in
    /// weight — it's the most prominent thing the module exports.
    Default,
    /// Local re-export clause without a `from` source: `export { foo }`,
    /// `export type { Pattern }`. Names a local declaration; the
    /// declaration itself isn't resolved (would need a name table), but
    /// the export line still tells the agent the public surface includes
    /// `foo`. Single-line span.
    NamedReexport,
}

impl ItemKind {
    fn kind_weight(self) -> f64 {
        match self {
            ItemKind::Default => 1.2,
            ItemKind::Interface => 1.1,
            ItemKind::Class => 1.05,
            ItemKind::TypeAlias => 1.0,
            ItemKind::Enum => 1.0,
            ItemKind::Function => 0.95,
            ItemKind::Const => 0.85,
            ItemKind::NamedReexport => 0.95,
        }
    }
}

fn should_split_js_class_export(file: &Path, item: &ExportInfo<'_>) -> bool {
    is_js_file(file)
        && matches!(item.kind, ItemKind::Class | ItemKind::Default)
        && is_class_node(item.decl)
        && item.class_members.len() >= JS_CLASS_MEMBER_SPLIT_MIN
}

#[derive(Debug, Clone)]
struct ExportInfo<'a> {
    /// 1-based line of either the wrapping `export_statement` (real export)
    /// or the module-private `lexical_declaration` re-exported by name
    /// (synthetic export).
    start_line: usize,
    kind: ItemKind,
    /// Node used for the declaration surface and JSDoc anchor. Real
    /// exports anchor at the wrapping `export_statement`; synthetic
    /// re-exports anchor at the local declaration itself.
    anchor: Node<'a>,
    decl: Node<'a>,
    body_parts: Vec<BodyPart>,
    class_members: Vec<ClassMemberInfo>,
    /// True when this export carries no runtime value: `Interface` /
    /// `TypeAlias`, or a `NamedReexport` whose `export_statement` has
    /// the statement-level `type` keyword (`export type { Foo }`). Used
    /// by `type_machinery_factor` to flag whole files as type-machinery
    /// internals — the per-export `Export` / `ExportDoc` value is
    /// damped on those files. `Enum` is *not* type-only (TS enums emit
    /// runtime objects).
    is_type_only: bool,
}

#[derive(Debug, Clone)]
struct ClassMemberInfo {
    start_line: usize,
    lines: FileLines,
    body_parts: Vec<BodyPart>,
}

#[derive(Debug, Clone)]
struct ModuleItemInfo {
    start_line: usize,
    kind: ItemKind,
    lines: FileLines,
    body_parts: Vec<BodyPart>,
}

/// Top-level exports in a file. Walks `program` children, looking for
/// `export_statement` nodes and identifying the inner declaration. Re-
/// exports without an inner declaration (`export { foo } from '…'`) are
/// skipped — they're plumbing, picked up by `Imports`. After the real
/// pass, walks `lexical_declaration` siblings looking for module-private
/// `const X = <fn-init>` whose name appears in a top-level
/// `export { X }` value clause; emits a synthesized `ExportInfo` for each
/// (skipping any that share a start_line with a real `export_statement`,
/// since `TsKey::Export` keys disambiguate only by start_line and the
/// scheduler dedupes silently).
fn find_export_starts<'a>(
    file: &Path,
    tree: &'a Tree,
    source: &str,
    src_lines: &[&str],
) -> Vec<ExportInfo<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    let mut real_lines: HashSet<usize> = HashSet::new();
    for child in root.children(&mut cursor) {
        let Some((kind, decl_node)) =
            classify_export(child, source).or_else(|| classify_commonjs_export(child, source))
        else {
            continue;
        };
        let start_line = child.start_position().row + 1;
        real_lines.insert(start_line);
        let is_type_only = is_export_type_only(kind, child, source);
        out.push(make_export_info(
            start_line,
            kind,
            child,
            decl_node,
            file,
            src_lines,
            is_type_only,
        ));
    }

    let reexports = collect_local_value_reexports(tree, source);
    let commonjs_reexports = collect_commonjs_value_reexports(tree, source);
    if !reexports.is_empty() || !commonjs_reexports.is_empty() {
        let mut emitted_lines = real_lines.clone();
        let mut cursor = root.walk();
        for child in root.children(&mut cursor) {
            let start_line = child.start_position().row + 1;
            if emitted_lines.contains(&start_line) {
                continue;
            }

            let kind = if matches!(child.kind(), "lexical_declaration" | "variable_declaration")
                && synthetic_export_name(child, source, &reexports).is_some()
            {
                Some(ItemKind::Const)
            } else {
                synthetic_commonjs_export_kind(child, source, &commonjs_reexports)
            };

            let Some(kind) = kind else { continue };
            out.push(make_export_info(
                start_line, kind, child, child, file, src_lines, false,
            ));
            emitted_lines.insert(start_line);
        }
        out.sort_by_key(|e| e.start_line);
    }

    out
}

fn make_export_info<'a>(
    start_line: usize,
    kind: ItemKind,
    anchor: Node<'a>,
    decl: Node<'a>,
    file: &Path,
    src_lines: &[&str],
    is_type_only: bool,
) -> ExportInfo<'a> {
    let collect_class_members = is_js_file(file)
        && matches!(kind, ItemKind::Class | ItemKind::Default)
        && is_class_node(decl);
    let class_members = if collect_class_members {
        class_member_infos(decl, src_lines)
    } else {
        Vec::new()
    };
    let body_parts = if collect_class_members {
        merged_body_parts(
            class_members
                .iter()
                .flat_map(|member| member.body_parts.clone())
                .collect(),
        )
    } else {
        merged_body_parts(body_parts(decl, kind, src_lines))
    };
    ExportInfo {
        start_line,
        kind,
        anchor,
        decl,
        body_parts,
        class_members,
        is_type_only,
    }
}

/// Top-level declarations that are not already represented by public export
/// batches. These module-private items often hold the real implementation
/// behind a thin exported API.
fn find_module_items(
    tree: &Tree,
    source: &str,
    src_lines: &[&str],
    export_start_lines: &HashSet<usize>,
) -> Vec<ModuleItemInfo> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        let Some(kind) = decl_kind(child) else {
            continue;
        };
        if is_private_props_type(child, kind, source) {
            continue;
        }
        if is_require_declaration(child, source) {
            continue;
        }
        let start_line = child.start_position().row + 1;
        if export_start_lines.contains(&start_line) {
            continue;
        }
        let lines = module_item_lines(kind, child, source);
        let body_parts = body_parts(child, kind, src_lines);
        out.push(ModuleItemInfo {
            start_line,
            kind,
            lines,
            body_parts,
        });
    }
    out
}

fn is_private_props_type(node: Node, kind: ItemKind, source: &str) -> bool {
    if !matches!(kind, ItemKind::Interface | ItemKind::TypeAlias) {
        return false;
    }
    name_of(node, source).is_some_and(|name| name.ends_with("Props"))
}

/// True when this export carries no runtime value. `NamedReexport` is
/// type-only iff the wrapping `export_statement` carries the
/// statement-level `type` keyword (`export type { Foo }`); inline
/// `export { type Foo, valueY }` is conservatively runtime since per-
/// specifier mixing would misclassify. `export declare ...` (ambient)
/// is type-only — the declaration names a runtime value provided by
/// some other environment, but the .ts file itself emits no code.
/// `Enum` without `declare` is runtime (TS enums emit a runtime
/// object).
fn is_export_type_only(kind: ItemKind, stmt: Node, source: &str) -> bool {
    if has_ambient_declaration(stmt) {
        return true;
    }
    match kind {
        ItemKind::Interface | ItemKind::TypeAlias => true,
        ItemKind::NamedReexport => has_export_type_keyword(stmt, source),
        ItemKind::Class
        | ItemKind::Function
        | ItemKind::Const
        | ItemKind::Default
        | ItemKind::Enum => false,
    }
}

/// True when this `export_statement` wraps an `ambient_declaration` —
/// `export declare function` / `export declare class` / `export declare
/// const`, etc. tree-sitter-typescript surfaces this wrapper between
/// `export_statement` and the inner decl node.
fn has_ambient_declaration(stmt: Node) -> bool {
    let mut cursor = stmt.walk();
    stmt.children(&mut cursor)
        .any(|c| c.kind() == "ambient_declaration")
}

/// True for TypeScript declaration files (`.d.ts` / `.d.tsx`). All
/// exports in these files are implicitly ambient — the file emits no
/// runtime code, so per-export batches are deprioritized like other
/// type-machinery files.
pub(crate) fn is_declaration_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.ends_with(".d.ts") || n.ends_with(".d.tsx"))
}

/// `TYPE_MACHINERY_FILE_FACTOR` when this file emits no runtime code,
/// else `1.0`. Two paths qualify: a declaration file (`.d.ts`) — every
/// export is implicitly ambient — or a regular `.ts` file whose every
/// top-level export is type-only (interface / type alias / `export
/// type { ... }` / `export declare ...`). Per-export `Export` /
/// `ExportDoc` / `ExportBody` batches multiply this in so the
/// schedule prefers runtime-bearing files at the same V/C.
/// `ExportNames` is intentionally outside this discount — NS authors
/// of type-heavy public APIs (e.g., `ky`) expect the names surface
/// even when individual lines aren't load-bearing.
fn type_machinery_factor(file: &Path, exports: &[ExportInfo<'_>]) -> f64 {
    if is_declaration_file(file) || (!exports.is_empty() && exports.iter().all(|e| e.is_type_only))
    {
        TYPE_MACHINERY_FILE_FACTOR
    } else {
        1.0
    }
}

const TYPE_MACHINERY_FILE_FACTOR: f64 = 0.7;

/// Local identifier names that appear in any top-level **value**
/// re-export clause (`export { X }`, `export { X as Y }`). Excludes
/// `export type { X }` (statement-level type modifier),
/// `export { type X }` (per-specifier type modifier), and any clause
/// with a `from '…'` source (those are plumbing handled by `Imports`).
fn collect_local_value_reexports(tree: &Tree, source: &str) -> HashSet<String> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = HashSet::new();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() != "export_statement" {
            continue;
        }
        if has_from_source(stmt) {
            continue;
        }
        if has_export_type_keyword(stmt, source) {
            continue;
        }
        let mut sc = stmt.walk();
        for clause in stmt.children(&mut sc) {
            if !matches!(clause.kind(), "export_clause" | "namespace_export") {
                continue;
            }
            let mut cc = clause.walk();
            for spec in clause.children(&mut cc) {
                if spec.kind() != "export_specifier" {
                    continue;
                }
                if has_inline_type_modifier(spec) {
                    continue;
                }
                if let Some(name_node) = first_identifier_child(spec) {
                    let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
                    out.insert(name);
                }
            }
        }
    }
    out
}

/// Local identifier names that appear on the right-hand side of top-level
/// CommonJS export assignments (`exports.Foo = Foo`,
/// `module.exports.Foo = Foo`, or `module.exports = { Foo }`). These mirror
/// ESM value re-exports for JS packages that keep declarations local and
/// publish them at the bottom of the module.
fn collect_commonjs_value_reexports(tree: &Tree, source: &str) -> HashSet<String> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = HashSet::new();
    for stmt in root.children(&mut cursor) {
        let Some((left, right)) = commonjs_assignment_sides(stmt, source) else {
            continue;
        };
        if matches!(
            commonjs_export_target(left, source),
            Some(CommonJsExportTarget::Namespace)
        ) && right.kind() == "object"
        {
            collect_object_export_names(right, source, &mut out);
        } else if let Some(name) = identifier_text(right, source) {
            out.insert(name.to_string());
        }
    }
    out
}

fn collect_object_export_names(node: Node, source: &str, out: &mut HashSet<String>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "shorthand_property_identifier" | "identifier" => {
                out.insert(source[child.start_byte()..child.end_byte()].to_string());
            }
            "pair" => {
                if let Some(value) = child.child_by_field_name("value")
                    && let Some(name) = identifier_text(value, source)
                {
                    out.insert(name.to_string());
                }
            }
            _ => {}
        }
    }
}

/// True when `export_statement` carries a statement-level `type`
/// keyword: `export type { X }` / `export type * as N from '…'` etc.
fn has_export_type_keyword(stmt: Node, source: &str) -> bool {
    let mut cursor = stmt.walk();
    stmt.children(&mut cursor).any(|c| {
        c.kind() == "type"
            || (c.kind() == "keyword" && &source[c.start_byte()..c.end_byte()] == "type")
    })
}

/// True when `export_specifier` carries an inline `type` modifier:
/// `export { type X }` / `export { type X as Y }`.
fn has_inline_type_modifier(spec: Node) -> bool {
    let mut cursor = spec.walk();
    spec.children(&mut cursor).any(|c| c.kind() == "type")
}

fn first_identifier_child(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|c| c.kind() == "identifier")
}

/// Returns the exported local declaration kind when `decl` is named by a
/// top-level CommonJS value export. Function and class declarations are
/// keyed directly by their declared name; const/let declarations keep the
/// narrower fn-init rule used for ESM re-export synthesis.
fn synthetic_commonjs_export_kind(
    decl: Node,
    source: &str,
    reexport_set: &HashSet<String>,
) -> Option<ItemKind> {
    let kind = decl_kind(decl)?;
    match kind {
        ItemKind::Function | ItemKind::Class => {
            let name = name_of(decl, source)?;
            reexport_set.contains(name).then_some(kind)
        }
        ItemKind::Const => synthetic_export_name(decl, source, reexport_set).map(|_| kind),
        ItemKind::Interface | ItemKind::TypeAlias | ItemKind::Enum => None,
        ItemKind::Default | ItemKind::NamedReexport => None,
    }
}

/// Returns the local declared name when `decl` is a single-binding
/// `const X = <fn-init>` whose `X` is in `reexport_set`. The fn-init
/// requirement excludes pure-data consts (objects, primitives) — those
/// don't have a body interior worth eliding behind a marker, and a
/// "synthetic export" of a data const would just duplicate the
/// `NamedReexport` line.
fn synthetic_export_name(
    decl: Node,
    source: &str,
    reexport_set: &HashSet<String>,
) -> Option<String> {
    let mut cursor = decl.walk();
    let declarators: Vec<Node> = decl
        .children(&mut cursor)
        .filter(|c| matches!(c.kind(), "variable_declarator" | "lexical_binding"))
        .collect();
    if declarators.len() != 1 {
        return None;
    }
    let dr = declarators[0];
    let name_node = dr.child_by_field_name("name")?;
    if name_node.kind() != "identifier" {
        return None;
    }
    let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
    if !reexport_set.contains(&name) {
        return None;
    }
    find_fn_init_body(decl)?;
    Some(name)
}

/// Locate the function/class `statement_block` body reachable inside a
/// `lexical_declaration` / `variable_declaration`'s initializer,
/// transparently walking through `parenthesized_expression` and
/// `call_expression` argument lists. Returns `None` for object/array/
/// primitive initializers, or for fn-init shapes whose body is an
/// expression (`() => 1`) rather than a block. Used for both the
/// signature-truncation point in `collect_export_lines` and the body-
/// interior emit set in `export_body_rows`.
fn find_fn_init_body(decl: Node) -> Option<Node> {
    let mut cursor = decl.walk();
    let declarator = decl
        .children(&mut cursor)
        .find(|c| matches!(c.kind(), "variable_declarator" | "lexical_binding"))?;
    let value = declarator.child_by_field_name("value")?;
    descend_for_fn_body(value, FN_BODY_DESCEND_DEPTH)
}

/// Maximum depth `descend_for_fn_body` recurses through call/parenthesis
/// wrappers. Real-world chains (`memo(forwardRef(props => {...}))`) are
/// 2–3 deep; the larger bound is just a defensive guard against
/// pathological nesting blowing the stack.
const FN_BODY_DESCEND_DEPTH: usize = 6;

fn descend_for_fn_body(node: Node, depth: usize) -> Option<Node> {
    if depth == 0 {
        return None;
    }
    match node.kind() {
        "arrow_function"
        | "function_expression"
        | "function_declaration"
        | "generator_function"
        | "generator_function_declaration" => {
            let body = node.child_by_field_name("body")?;
            if body.kind() == "statement_block" {
                Some(body)
            } else {
                None
            }
        }
        "call_expression" => {
            // Only treat the first non-trivial argument as the wrapped
            // value. This is the React.forwardRef / memo / observer
            // shape (`wrapper(callback, ...optional)`); rejecting deeper
            // arg positions avoids `factory(input, () => {...}, opts)`
            // being rendered as if the middle callback were the export's
            // body, which would silently elide the trailing arguments.
            let args = node.child_by_field_name("arguments")?;
            let first = args.named_child(0)?;
            descend_for_fn_body(first, depth - 1)
        }
        "parenthesized_expression" => {
            let inner = node.named_child(0)?;
            descend_for_fn_body(inner, depth - 1)
        }
        _ => None,
    }
}

/// Result of resolving a `start_line` to a top-level export-bearing node.
/// `Real` is the existing `export ...` statement; `Synthetic` is a
/// module-private `const X = <fn-init>` whose name appears in a value
/// re-export clause (see `synthetic_export_name`).
#[cfg(test)]
enum LocatedExport<'a> {
    Real {
        export_stmt: Node<'a>,
        decl: Node<'a>,
        kind: ItemKind,
    },
    Synthetic {
        decl: Node<'a>,
        kind: ItemKind,
    },
}

#[cfg(test)]
impl<'a> LocatedExport<'a> {
    fn kind(&self) -> ItemKind {
        match self {
            LocatedExport::Real { kind, .. } | LocatedExport::Synthetic { kind, .. } => *kind,
        }
    }

    /// The node a collector should anchor at (start_row, prev_sibling
    /// for JSDoc) — the wrapping `export_statement` for real exports,
    /// the lexical_declaration itself for synthetics.
    fn anchor(&self) -> Node<'a> {
        match *self {
            LocatedExport::Real { export_stmt, .. } => export_stmt,
            LocatedExport::Synthetic { decl, .. } => decl,
        }
    }

    /// The inner declaration node — the function/class/lexical_decl
    /// returned by `classify_export` for real exports, the
    /// lexical_declaration itself for synthetics.
    fn decl(&self) -> Node<'a> {
        match *self {
            LocatedExport::Real { decl, .. } | LocatedExport::Synthetic { decl, .. } => decl,
        }
    }
}

/// Single source of truth for "what does this `start_line` point to".
/// First pass returns the `export_statement` at this line if any —
/// that mirrors `find_export_starts`'s same-line collision filter, so a
/// `TsKey::Export { start_line }` for a real export wins even when a
/// module-private `const` lives on the same source line. Second pass
/// looks for a synthetic candidate. Returns `None` for stale keys
/// (source edited since the key was issued) so collectors short-circuit
/// rather than render the wrong content.
#[cfg(test)]
fn locate_export_decl<'a>(
    tree: &'a Tree,
    source: &str,
    start_line: usize,
) -> Option<LocatedExport<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 == start_line {
            let Some((kind, decl)) =
                classify_export(child, source).or_else(|| classify_commonjs_export(child, source))
            else {
                continue;
            };
            return Some(LocatedExport::Real {
                export_stmt: child,
                decl,
                kind,
            });
        }
    }

    let reexports = collect_local_value_reexports(tree, source);
    let commonjs_reexports = collect_commonjs_value_reexports(tree, source);
    if reexports.is_empty() && commonjs_reexports.is_empty() {
        return None;
    }

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        if matches!(child.kind(), "lexical_declaration" | "variable_declaration")
            && synthetic_export_name(child, source, &reexports).is_some()
        {
            return Some(LocatedExport::Synthetic {
                decl: child,
                kind: ItemKind::Const,
            });
        }
        if let Some(kind) = synthetic_commonjs_export_kind(child, source, &commonjs_reexports) {
            return Some(LocatedExport::Synthetic { decl: child, kind });
        }
    }
    None
}

/// Recognize a top-level export-bearing item. Returns `(ItemKind,
/// "declaration node whose end-of-content we measure")` or `None` if this
/// node isn't a meaningful exported declaration.
fn classify_export<'a>(node: Node<'a>, source: &str) -> Option<(ItemKind, Node<'a>)> {
    if node.kind() != "export_statement" {
        return None;
    }
    if has_default_keyword(node, source) {
        // Find a meaningful body node — if the default is a function or
        // class expression, span that; else span the whole stmt.
        let body = first_decl_or_value_child(node).unwrap_or(node);
        return Some((ItemKind::Default, body));
    }
    if let Some(decl) = first_decl_child(node) {
        let kind = decl_kind(decl)?;
        return Some((kind, decl));
    }
    // Local re-export clause: `export { foo }` or `export type { Foo }`
    // without a `from '…'` source. The `from` form is plumbing handled
    // by `Imports`; the local form names a local declaration and earns
    // its own (single-line) export batch.
    if has_export_clause(node) && !has_from_source(node) {
        return Some((ItemKind::NamedReexport, node));
    }
    None
}

/// Recognize a top-level CommonJS export assignment as an export-bearing
/// surface. Local declarations assigned by name are also synthesized by
/// `synthetic_export_kind`; keeping this assignment batch is still useful
/// for entrypoint files where the export line itself is the public factory
/// or alias (`exports.createCommand = ...`, `exports.Command = Command`).
fn classify_commonjs_export<'a>(node: Node<'a>, source: &str) -> Option<(ItemKind, Node<'a>)> {
    let (_, right) = commonjs_assignment_sides(node, source)?;
    Some(match right.kind() {
        "function_expression"
        | "arrow_function"
        | "generator_function"
        | "generator_function_declaration"
        | "function_declaration" => (ItemKind::Function, right),
        "class" | "class_declaration" | "abstract_class_declaration" => (ItemKind::Class, right),
        _ => (ItemKind::Const, node),
    })
}

fn commonjs_assignment_sides<'a>(node: Node<'a>, source: &str) -> Option<(Node<'a>, Node<'a>)> {
    if node.kind() != "expression_statement" {
        return None;
    }
    let expr = node.named_child(0)?;
    if expr.kind() != "assignment_expression" {
        return None;
    }
    let (left, right) = assignment_sides(expr)?;
    commonjs_export_target(left, source).map(|_| (left, right))
}

fn assignment_sides<'a>(assignment: Node<'a>) -> Option<(Node<'a>, Node<'a>)> {
    if let (Some(left), Some(right)) = (
        assignment.child_by_field_name("left"),
        assignment.child_by_field_name("right"),
    ) {
        return Some((left, right));
    }
    let mut cursor = assignment.walk();
    let mut named = assignment.named_children(&mut cursor);
    Some((named.next()?, named.next()?))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommonJsExportTarget {
    Namespace,
    Property,
}

fn commonjs_export_target(node: Node, source: &str) -> Option<CommonJsExportTarget> {
    match node.kind() {
        "member_expression" => {
            let (object, property) = member_object_property(node)?;
            if identifier_eq(object, source, "exports") {
                Some(CommonJsExportTarget::Property)
            } else if identifier_eq(object, source, "module")
                && identifier_eq(property, source, "exports")
            {
                Some(CommonJsExportTarget::Namespace)
            } else if is_module_exports_member(object, source) {
                Some(CommonJsExportTarget::Property)
            } else {
                None
            }
        }
        "subscript_expression" => {
            let object = subscript_object(node)?;
            (identifier_eq(object, source, "exports") || is_module_exports_member(object, source))
                .then_some(CommonJsExportTarget::Property)
        }
        _ => None,
    }
}

fn is_module_exports_member(node: Node, source: &str) -> bool {
    if node.kind() != "member_expression" {
        return false;
    }
    let Some((object, property)) = member_object_property(node) else {
        return false;
    };
    identifier_eq(object, source, "module") && identifier_eq(property, source, "exports")
}

fn member_object_property(node: Node) -> Option<(Node, Node)> {
    if let (Some(object), Some(property)) = (
        node.child_by_field_name("object"),
        node.child_by_field_name("property"),
    ) {
        return Some((object, property));
    }
    let mut cursor = node.walk();
    let mut named = node.named_children(&mut cursor);
    Some((named.next()?, named.next()?))
}

fn subscript_object(node: Node) -> Option<Node> {
    node.child_by_field_name("object")
        .or_else(|| node.named_child(0))
}

fn identifier_eq(node: Node, source: &str, expected: &str) -> bool {
    matches!(
        node.kind(),
        "identifier" | "property_identifier" | "shorthand_property_identifier"
    ) && &source[node.start_byte()..node.end_byte()] == expected
}

fn identifier_text<'a>(node: Node, source: &'a str) -> Option<&'a str> {
    matches!(node.kind(), "identifier" | "shorthand_property_identifier")
        .then_some(&source[node.start_byte()..node.end_byte()])
}

/// Direct decl child of an `export_statement`, transparently unwrapping the
/// `ambient_declaration` wrapper that tree-sitter-typescript inserts for
/// `export declare …` (function / class / interface / namespace / const).
fn first_decl_child(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if decl_kind(child).is_some() {
            return Some(child);
        }
        if child.kind() == "ambient_declaration" {
            let mut inner = child.walk();
            if let Some(inner_decl) = child.children(&mut inner).find(|c| decl_kind(*c).is_some()) {
                return Some(inner_decl);
            }
        }
    }
    None
}

fn has_export_clause(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|c| matches!(c.kind(), "export_clause" | "namespace_export" | "*"))
}

fn has_from_source(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|c| c.kind() == "string")
}

fn first_decl_or_value_child<'a>(node: Node<'a>) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor).find(|c| {
        decl_kind(*c).is_some()
            || matches!(
                c.kind(),
                "function_expression"
                    | "arrow_function"
                    | "class"
                    | "identifier"
                    | "call_expression"
                    | "object"
                    | "new_expression"
            )
    })
}

fn decl_kind(node: Node) -> Option<ItemKind> {
    Some(match node.kind() {
        "function_declaration" | "function_signature" | "generator_function_declaration" => {
            ItemKind::Function
        }
        "class_declaration" | "abstract_class_declaration" => ItemKind::Class,
        "interface_declaration" => ItemKind::Interface,
        "type_alias_declaration" => ItemKind::TypeAlias,
        "enum_declaration" => ItemKind::Enum,
        "lexical_declaration" | "variable_declaration" => ItemKind::Const,
        _ => return None,
    })
}

fn has_default_keyword(node: Node, source: &str) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|c| {
        c.kind() == "default" || {
            let text = &source[c.start_byte()..c.end_byte()];
            c.kind() == "keyword" && text == "default"
        }
    })
}

/// Files whose name signals "module entrypoint / public surface".
fn is_entrypoint_file(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
        matches!(
            n,
            "index.ts"
                | "index.tsx"
                | "index.js"
                | "index.mjs"
                | "index.cjs"
                | "main.ts"
                | "main.js"
                | "main.mjs"
                | "main.cjs"
                | "mod.ts"
                | "mod.js"
                | "mod.mjs"
                | "mod.cjs"
                | "esm.js"
                | "esm.mjs"
        )
    })
}

pub(crate) fn is_ts_or_tsx_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("ts") || e.eq_ignore_ascii_case("tsx"))
}

fn is_tsx_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("tsx"))
}

fn is_js_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| {
        e.eq_ignore_ascii_case("js")
            || e.eq_ignore_ascii_case("mjs")
            || e.eq_ignore_ascii_case("cjs")
    })
}

fn entrypoint_boost(path: &Path) -> f64 {
    if is_entrypoint_file(path) { 1.4 } else { 1.0 }
}

fn ts_depth_factor(path: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(path, ctx, is_entrypoint_file(path))
}

const JS_CONFIG_VALUE_FACTOR: f64 = 0.001;
const PRIMARY_JS_VALUE_FACTOR: f64 = 0.50;
const SECONDARY_JS_VALUE_FACTOR: f64 = 0.05;

fn js_value_factor(path: &Path, ctx: &WalkCtx) -> f64 {
    if is_js_config_file(path) {
        // Dev-tooling JS should remain discoverable without taking budget
        // from source files in TS-first packages.
        JS_CONFIG_VALUE_FACTOR
    } else if is_primary_js_source_path(path, ctx) {
        // Package JS/MJS/CJS entrypoints are often the whole public API, but
        // still need to rank below equivalent TS so TS fixtures stay stable.
        PRIMARY_JS_VALUE_FACTOR
    } else if is_js_file(path) {
        // Secondary JS helpers/scripts are useful fallback context, not the
        // primary surface when source files exist elsewhere.
        SECONDARY_JS_VALUE_FACTOR
    } else {
        1.0
    }
}

fn is_primary_js_source_path(path: &Path, ctx: &WalkCtx) -> bool {
    if !is_js_file(path) {
        return false;
    }
    let Ok(rel) = path.strip_prefix(ctx.root()) else {
        return false;
    };
    let mut component_count = 0;
    for component in rel.components() {
        component_count += 1;
        if is_source_dir(Path::new(component.as_os_str())) {
            return true;
        }
    }
    component_count == 1
}

fn is_js_config_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    is_js_file(path)
        && (name.starts_with('.')
            || ends_with_ignore_ascii_case(name, ".config.js")
            || ends_with_ignore_ascii_case(name, ".config.cjs")
            || ends_with_ignore_ascii_case(name, ".config.mjs")
            || ends_with_ignore_ascii_case(name, "rc.js")
            || ends_with_ignore_ascii_case(name, "rc.cjs")
            || ends_with_ignore_ascii_case(name, "rc.mjs"))
}

fn ends_with_ignore_ascii_case(value: &str, suffix: &str) -> bool {
    let value = value.as_bytes();
    let suffix = suffix.as_bytes();
    value.len() >= suffix.len()
        && value[value.len() - suffix.len()..]
            .iter()
            .zip(suffix)
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
}

fn module_doc_lede_value(file: &Path, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let cat = (0.8 * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.5, 0.9, ts_depth_factor(file, ctx)) * js_factor
}

fn imports_value(file: &Path, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let cat = (0.3 * entrypoint_boost(file)).min(1.0);
    mix_signals(cat, 0.55, 0.3, ts_depth_factor(file, ctx)) * js_factor
}

fn imports_chunk_value(
    file: &Path,
    ctx: &WalkCtx,
    chunk_index: usize,
    chunk_count: usize,
    js_factor: f64,
) -> f64 {
    imports_value(file, ctx, js_factor) * reexport_import_chunk_factor(chunk_index, chunk_count)
}

fn export_names_value(
    file: &Path,
    ctx: &WalkCtx,
    chunk_index: usize,
    chunk_count: usize,
    js_factor: f64,
    has_split_js_class_export: bool,
) -> f64 {
    let cat = (0.8 * entrypoint_boost(file)).min(1.0);
    let class_split_factor = if has_split_js_class_export { 1.12 } else { 1.0 };
    mix_signals(cat, 0.6, 0.35, ts_depth_factor(file, ctx))
        * names_surface_chunk_factor(chunk_index, chunk_count)
        * js_factor
        * class_split_factor
}

fn export_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.70 * k * entrypoint_boost(file)).min(1.0);
    let fu = (0.85 * k).min(1.0);
    mix_signals(cat, fu, 0.65, ts_depth_factor(file, ctx)) * js_factor
}

fn export_doc_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.20 * k * entrypoint_boost(file)).min(1.0);
    let fu = (0.6 * k).min(1.0);
    mix_signals(cat, fu, 0.8, ts_depth_factor(file, ctx)) * js_factor
}

fn export_member_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.62 * k * entrypoint_boost(file)).min(1.0);
    let fu = (0.95 * k).min(1.0);
    mix_signals(cat, fu, 0.55, ts_depth_factor(file, ctx)) * js_factor
}

fn module_item_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let class_boost = if matches!(kind, ItemKind::Class) {
        1.5
    } else {
        1.0
    };
    let cat = (0.38 * class_boost * k * entrypoint_boost(file)).min(1.0);
    let fu = (0.7 * class_boost * k).min(1.0);
    mix_signals(cat, fu, 0.55, ts_depth_factor(file, ctx)) * js_factor
}

fn module_item_body_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.30 * k * entrypoint_boost(file)).min(1.0);
    let fu = (0.82 * k).min(1.0);
    mix_signals(cat, fu, 0.65, ts_depth_factor(file, ctx)) * js_factor
}

// Strictly below `Export.catastrophic` (0.70) — `Export`'s signature already
// hedges existence; the body is a refinement. Strictly above
// `Export.follow_up` (0.85) — body is the prime "don't go grep" signal.
fn export_body_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.45 * k * entrypoint_boost(file)).min(1.0);
    let fu = (0.9 * k).min(1.0);
    mix_signals(cat, fu, 0.8, ts_depth_factor(file, ctx)) * js_factor
}

// --- parser ---

fn parse_ts(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    let language = if is_tsx_file(path) {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    };
    ctx.parse_tree(path, &language)
}

// --- collectors ---

/// Collect lines belonging to the leading `/** … */` JSDoc block at the
/// very top of the file. Skips a leading shebang and arbitrary blank lines
/// but stops at the first non-comment, non-import, non-blank token.
fn collect_module_doc_lede(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "hash_bang_line" {
            continue;
        }
        if child.kind() == "comment" {
            let text = &source[child.start_byte()..child.end_byte()];
            if text.starts_with("/**") {
                extend_span(&mut lines, child, source);
            }
            // Plain `/* */` and `//` comments at the top — skip them but
            // keep walking; license headers commonly precede the JSDoc.
            continue;
        }
        break;
    }
    FileLines::new(lines)
}

/// Collect all `import` and bare-`export … from` lines at the top of the
/// file. Stops at the first declaration node (so trailing re-exports
/// after real code don't get folded in).
fn collect_imports(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "expression_statement" if is_string_directive(child) => {
                extend_span(&mut lines, child, source)
            }
            "import_statement" => extend_span(&mut lines, child, source),
            "lexical_declaration" | "variable_declaration"
                if is_require_declaration(child, source) =>
            {
                extend_span(&mut lines, child, source)
            }
            "export_statement" => {
                if is_bare_reexport(child) {
                    extend_span(&mut lines, child, source);
                } else {
                    break;
                }
            }
            "comment" | "hash_bang_line" => {}
            _ => break,
        }
    }
    FileLines::new(dedup_sorted(lines))
}

fn collect_reexport_import_chunks(
    file: &Path,
    tree: &Tree,
    source: &str,
) -> Option<Vec<FileLines>> {
    if !is_entrypoint_file(file) {
        return None;
    }
    let groups = collect_reexport_import_groups(tree, source)?;
    if !should_chunk_import_groups(&groups) {
        return None;
    }
    Some(groups_to_file_lines(groups))
}

fn collect_reexport_import_groups(tree: &Tree, source: &str) -> Option<Vec<ImportGroup>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut groups = Vec::new();
    let mut other_statements = 0usize;
    let mut other_lines = 0usize;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "expression_statement" if is_string_directive(child) => {
                push_import_group(
                    &mut groups,
                    "__directive__".to_string(),
                    false,
                    child,
                    source,
                );
            }
            "import_statement" => {
                push_import_group(
                    &mut groups,
                    source_literal(child, source).unwrap_or_else(|| {
                        source[child.start_byte()..child.end_byte()].to_string()
                    }),
                    true,
                    child,
                    source,
                );
            }
            "lexical_declaration" | "variable_declaration"
                if is_require_declaration(child, source) =>
            {
                push_import_group(
                    &mut groups,
                    source_literal(child, source).unwrap_or_else(|| {
                        source[child.start_byte()..child.end_byte()].to_string()
                    }),
                    true,
                    child,
                    source,
                );
            }
            "export_statement" if is_bare_reexport(child) => {
                push_import_group(
                    &mut groups,
                    source_literal(child, source).unwrap_or_else(|| {
                        source[child.start_byte()..child.end_byte()].to_string()
                    }),
                    true,
                    child,
                    source,
                );
            }
            "comment" | "hash_bang_line" => {}
            _ if !tolerate_reexport_wall_other(child, &mut other_statements, &mut other_lines) => {
                return None;
            }
            _ => {}
        }
    }
    Some(groups)
}

fn tolerate_reexport_wall_other(
    node: Node,
    other_statements: &mut usize,
    other_lines: &mut usize,
) -> bool {
    *other_statements += 1;
    *other_lines += node_line_count(node);
    *other_statements <= REEXPORT_IMPORT_MAX_OTHER_STATEMENTS
        && *other_lines <= REEXPORT_IMPORT_MAX_OTHER_LINES
}

fn source_literal(node: Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == "string")
        .map(|child| source[child.start_byte()..child.end_byte()].to_string())
}

fn is_require_declaration(node: Node, source: &str) -> bool {
    if !matches!(node.kind(), "lexical_declaration" | "variable_declaration") {
        return false;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|child| matches!(child.kind(), "variable_declarator" | "lexical_binding"))
        .any(|decl| {
            decl.child_by_field_name("value")
                .is_some_and(|value| is_require_call(value, source))
        })
}

fn is_require_call(node: Node, source: &str) -> bool {
    if node.kind() != "call_expression" {
        return false;
    }
    node.child_by_field_name("function")
        .or_else(|| node.named_child(0))
        .is_some_and(|callee| {
            callee.kind() == "identifier"
                && &source[callee.start_byte()..callee.end_byte()] == "require"
        })
}

fn is_string_directive(node: Node) -> bool {
    if node.kind() != "expression_statement" {
        return false;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|child| child.kind() == "string")
}

/// A bare re-export pulls names from another module: it has no inner
/// declaration, has an `export_clause` / `namespace_export` / `*` form,
/// **and** carries a `from '…'` source string. Local export clauses
/// (`export { foo }` with no `from`) are exports of a local declaration,
/// not plumbing — those get their own [`ItemKind::NamedReexport`] batch.
fn is_bare_reexport(node: Node) -> bool {
    first_decl_child(node).is_none() && has_export_clause(node) && has_from_source(node)
}

fn collect_export_names_from(
    items: &[ExportInfo<'_>],
    export_start_lines: &HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for item in items {
        full.push(item.start_line);
        let ellipsis_line = item.start_line + 1;
        if !export_start_lines.contains(&ellipsis_line) {
            ellipses.push(ellipsis_line);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Lines for a single export's decl. For interface/type/class/enum, the
/// whole item. For function, the signature plus a body-elision marker.
/// For const/let, the assignment line(s) — truncated at the inner
/// function body's `{` when the initializer is a fn-init (direct
/// arrow/function or wrapped through `forwardRef(props => {...})` etc.),
/// otherwise the whole declaration.
#[cfg(test)]
fn collect_export_lines(tree: &Tree, source: &str, start_line: usize) -> FileLines {
    let Some(located) = locate_export_decl(tree, source, start_line) else {
        return FileLines::new(Vec::new());
    };
    decl_surface_lines(located.kind(), located.anchor(), located.decl(), source)
}

fn module_item_lines(kind: ItemKind, decl: Node, source: &str) -> FileLines {
    decl_surface_lines(kind, decl, decl, source)
}

fn decl_surface_lines(kind: ItemKind, anchor: Node, decl: Node, source: &str) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let export_start_row = anchor.start_position().row;
    if matches!(kind, ItemKind::NamedReexport) {
        // Local export clause (`export { foo }` / `export type { Foo }`).
        // Span the whole statement; it's typically one line.
        push_rows(
            &mut full,
            export_start_row,
            node_end_row_trimmed(anchor, source),
        );
        return FileLines::new(dedup_sorted(full));
    }
    match decl.kind() {
        "function_declaration"
        | "function_signature"
        | "generator_function_declaration"
        | "function_expression"
        | "arrow_function" => {
            let sig_end = signature_end_row(decl);
            push_rows(&mut full, export_start_row, sig_end);
            if decl
                .child_by_field_name("body")
                .is_some_and(has_multiline_statement_block)
            {
                ellipses.push(sig_end + 2);
            }
        }
        "class_declaration" | "abstract_class_declaration" | "class" => {
            let body = decl.child_by_field_name("body");
            let header_end = body
                .map(|b| b.start_position().row)
                .unwrap_or_else(|| node_end_row_trimmed(decl, source));
            push_rows(&mut full, export_start_row, header_end);
            if let Some(b) = body {
                let mut bcur = b.walk();
                for member in b.children(&mut bcur) {
                    if class_surface_member_kind(member.kind()) {
                        let member_lines = member_header_lines(member).0;
                        full.extend(member_lines.full);
                        ellipses.extend(member_lines.ellipses);
                    }
                }
            }
        }
        "interface_declaration" | "type_alias_declaration" | "enum_declaration" => {
            push_rows(
                &mut full,
                export_start_row,
                node_end_row_trimmed(decl, source),
            );
        }
        "lexical_declaration" | "variable_declaration" => {
            if let Some(body) = find_fn_init_body(decl) {
                let body_start_row = body.start_position().row;
                push_rows(&mut full, export_start_row, body_start_row);
                if has_multiline_statement_block(body) {
                    ellipses.push(body_start_row + 2);
                }
            } else {
                push_rows(
                    &mut full,
                    export_start_row,
                    node_end_row_trimmed(decl, source),
                );
            }
        }
        _ => {
            // Default-export expression with nothing structural —
            // emit the single statement line.
            push_rows(
                &mut full,
                export_start_row,
                node_end_row_trimmed(decl, source),
            );
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

fn class_header_surface_lines(anchor: Node, decl: Node, source: &str) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let export_start_row = anchor.start_position().row;
    let body = decl.child_by_field_name("body");
    let header_end = body
        .map(|body| body.start_position().row)
        .unwrap_or_else(|| node_end_row_trimmed(decl, source));
    push_rows(&mut full, export_start_row, header_end);
    if body.is_some_and(|body| body.end_position().row > body.start_position().row + 1) {
        ellipses.push(header_end + 2);
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(ellipses)
}

fn class_member_infos(class_decl: Node, src_lines: &[&str]) -> Vec<ClassMemberInfo> {
    if !is_class_node(class_decl) {
        return Vec::new();
    }
    let Some(class_body) = class_decl.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut cursor = class_body.walk();
    for member in class_body.children(&mut cursor) {
        if !js_split_class_member_kind(member.kind()) {
            continue;
        }
        let start_line = member.start_position().row + 1;
        let (lines, body) = member_header_lines(member);
        let body_parts = statement_block_parts(body, src_lines, "statement_block");
        out.push(ClassMemberInfo {
            start_line,
            lines,
            body_parts,
        });
    }
    out
}

fn class_surface_member_kind(kind: &str) -> bool {
    matches!(
        kind,
        "method_definition"
            | "method_signature"
            | "abstract_method_signature"
            | "public_field_definition"
            | "property_signature"
    )
}

fn js_split_class_member_kind(kind: &str) -> bool {
    // JS class splitting only sees concrete members; signature kinds are TS-only.
    matches!(kind, "method_definition" | "public_field_definition")
}

fn member_header_lines(member: Node) -> (FileLines, Option<Node>) {
    let sig_end = signature_end_row(member);
    let start_row = member.start_position().row;
    let body = member.child_by_field_name("body");
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    push_rows(&mut full, start_row, sig_end);
    if body.is_some_and(has_multiline_statement_block) {
        ellipses.push(sig_end + 2);
    }
    (
        FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses)),
        body,
    )
}

fn is_class_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "class" | "class_declaration" | "abstract_class_declaration"
    )
}

fn has_multiline_statement_block(body: Node) -> bool {
    body.kind() == "statement_block" && body.end_position().row > body.start_position().row + 1
}

#[cfg(test)]
fn export_body_parts_for_start(tree: &Tree, source: &str, start_line: usize) -> Vec<BodyPart> {
    let Some(located) = locate_export_decl(tree, source, start_line) else {
        return Vec::new();
    };
    let src_lines: Vec<&str> = source.lines().collect();
    merged_body_parts(body_parts(located.decl(), located.kind(), &src_lines))
}

/// Body slices that the materializer emits for a declaration, after applying
/// the same blank-line filter as `build_file_spans`. Large statement blocks
/// split by top-level statement so regions can schedule independently; small
/// blocks stay merged because per-slice atom overhead dominates their benefit.
/// For classes, each method body is considered independently.
fn body_parts(decl: Node, kind: ItemKind, src_lines: &[&str]) -> Vec<BodyPart> {
    match kind {
        ItemKind::Function => statement_block_parts(
            decl.child_by_field_name("body"),
            src_lines,
            "statement_block",
        ),
        ItemKind::Class => class_method_body_parts(decl, src_lines),
        ItemKind::Const => {
            // Const fn-init: arrow / function expression direct, or wrapped
            // through `forwardRef(props => {...})` / `memo(...)`. Pure-data
            // consts (object/array/primitive) yield no body.
            if let Some(body) = find_fn_init_body(decl) {
                statement_block_parts(Some(body), src_lines, "statement_block")
            } else {
                Vec::new()
            }
        }
        ItemKind::Default => match decl.kind() {
            "function_declaration"
            | "function_expression"
            | "arrow_function"
            | "generator_function" => statement_block_parts(
                decl.child_by_field_name("body"),
                src_lines,
                "statement_block",
            ),
            "class" | "class_declaration" | "abstract_class_declaration" => {
                class_method_body_parts(decl, src_lines)
            }
            "call_expression" | "parenthesized_expression" => {
                // `export default forwardRef(props => {...})` etc.
                if let Some(body) = descend_for_fn_body(decl, FN_BODY_DESCEND_DEPTH) {
                    statement_block_parts(Some(body), src_lines, "statement_block")
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn merged_body_parts(parts: Vec<BodyPart>) -> Vec<BodyPart> {
    let lines = dedup_sorted(parts.into_iter().flat_map(|part| part.lines).collect());
    if lines.is_empty() {
        Vec::new()
    } else {
        vec![BodyPart { lines }]
    }
}

/// Walk `class_decl`'s body, accumulating body slices for every
/// `method_definition` member's `statement_block` body. Field
/// initializers (including arrow-init fields) and abstract method
/// signatures are skipped by design — they fall under the v4 deferral
/// (lexical-with-fn-init) noted in `ignore/plan-ts-export-body-v2.md`.
fn class_method_body_parts(class_decl: Node, src_lines: &[&str]) -> Vec<BodyPart> {
    let Some(class_body) = class_decl.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut cursor = class_body.walk();
    for member in class_body.children(&mut cursor) {
        if member.kind() == "method_definition" {
            out.extend(statement_block_parts(
                member.child_by_field_name("body"),
                src_lines,
                "statement_block",
            ));
        }
    }
    out
}

/// JSDoc (`/** */`) immediately above an export at `start_line`. For
/// entrypoint files the leading top-of-file JSDoc is reserved for
/// [`TsKey::ModuleDocLede`] — skip it here so the two batches don't
/// claim the same lines without a predecessor edge.
fn collect_jsdoc_above(node: Node, source: &str, out: &mut Vec<usize>, skip_module_lede: bool) {
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "comment" => {
                let text = &source[prev.start_byte()..prev.end_byte()];
                if text.starts_with("/**") {
                    if skip_module_lede && is_first_top_level_node(prev) {
                        break;
                    }
                    extend_span(out, prev, source);
                    cur = prev.prev_sibling();
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
}

/// True if the leading `/** */` comment at `node` is the same one
/// `ModuleDocLede` would claim — i.e. nothing precedes it except a
/// `hash_bang_line` and/or non-JSDoc comments (license headers etc).
/// Used to avoid double-claiming a file's first JSDoc block as both
/// `ModuleDocLede` and `ExportDoc`. Must mirror the predicate inside
/// `collect_module_doc_lede`.
fn is_first_top_level_node(node: Node) -> bool {
    let mut cur = node.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "hash_bang_line" => cur = prev.prev_sibling(),
            "comment" => {
                // ModuleDocLede walks past plain `/* */` and `//` comments
                // before claiming a `/**` block. Keep walking; a *prior*
                // `/**` block would mean this one isn't the lede, and the
                // outer `collect_jsdoc_above` loop has already accumulated
                // it before reaching here.
                cur = prev.prev_sibling();
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    //! Unit tests for `ExportBody`-related AST analysis. Each test parses a
    //! synthetic TypeScript source string and exercises `find_export_starts`
    //! / `collect_export_body` directly. Test fn names start with
    //! `walker_typescript_` so `cargo t walker_typescript` matches under
    //! nextest.

    use super::*;

    fn parse(source: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        parser.parse(source, None).unwrap()
    }

    fn export_infos<'a>(tree: &'a Tree, source: &str) -> Vec<ExportInfo<'a>> {
        let src_lines: Vec<&str> = source.lines().collect();
        find_export_starts(Path::new("fixture.ts"), tree, source, &src_lines)
    }

    fn body_emit_rows_for(source: &str) -> Vec<usize> {
        let tree = parse(source);
        let exports = export_infos(&tree, source);
        let item = exports.first().unwrap();
        let mut full = Vec::new();
        for part in item.body_parts.clone() {
            full.extend(part.lines);
        }
        full = dedup_sorted(full);
        // Sanity: helper-precomputed parts match actual emit non-emptiness.
        assert_eq!(!item.body_parts.is_empty(), !full.is_empty());
        full
    }

    #[test]
    fn walker_typescript_export_body_function_decl_emits_interior() {
        let src = "export function foo() {\n  let x = 1;\n  return x;\n}\n";
        let rows = body_emit_rows_for(src);
        // Body interior rows = 2..3 (line numbers 2, 3); brace rows 1, 4
        // skipped.
        assert_eq!(rows, vec![2, 3]);
    }

    #[test]
    fn walker_typescript_export_body_default_function_emits_interior() {
        let src =
            "export default function mitt() {\n  let all = new Map();\n  return { all };\n}\n";
        let rows = body_emit_rows_for(src);
        assert_eq!(rows, vec![2, 3]);
    }

    #[test]
    fn walker_typescript_export_body_single_line_function_no_emit() {
        let src = "export function foo() { return 1; }\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        // body.start_row == body.end_row → empty interior. No body
        // candidate fires.
        assert!(exports[0].body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_export_body_empty_body_no_emit() {
        let src = "export function foo() {}\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports[0].body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_export_body_class_walks_method_bodies_only() {
        let src = "\
export class Foo {
  field: number = 1;
  bar() {
    return 1;
  }
  baz(x: number) {
    let y = x + 1;
    return y;
  }
}
";
        let rows = body_emit_rows_for(src);
        // Method `bar` body interior: line 4. Method `baz` body
        // interior: lines 7, 8. Field initializer (`field`) skipped.
        assert_eq!(rows, vec![4, 7, 8]);
    }

    #[test]
    fn walker_typescript_export_body_interface_no_emit() {
        let src = "export interface Foo { bar(): void }\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports[0].body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_export_body_type_alias_no_emit() {
        let src = "export type Foo = { bar: number }\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports[0].body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_export_body_enum_no_emit() {
        let src = "export enum Foo { A, B }\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports[0].body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_export_body_lexical_arrow_emits_interior() {
        // `export const X = () => {...}` now emits the inner arrow body
        // through `Const`'s ExportBody arm. Signature truncates at the
        // body's `{` line.
        let src = "export const X = () => {\n  return 1;\n};\n";
        let rows = body_emit_rows_for(src);
        assert_eq!(rows, vec![2]);
    }

    #[test]
    fn walker_typescript_export_body_expression_arrow_no_emit() {
        let src = "export const X = () => 1;\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports[0].body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_export_body_const_object_no_emit() {
        // Object-literal initializers are pure data — no body to elide.
        let src = "export const X = { a: 1, b: 2 };\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports[0].body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_export_body_const_factory_callback_no_emit() {
        // Codex P1: factory(input, callback, opts) is NOT a wrapper —
        // descending into the middle callback would hide trailing
        // arguments behind an ellipsis. `descend_for_fn_body` only
        // peers at the FIRST argument; trailing-position callbacks
        // get no body emit, and `Export` renders the whole declaration.
        let src = "\
export const cfg = buildConfig(input, () => {
  return 1;
}, options);
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(
            exports[0].body_parts.is_empty(),
            "trailing-position callback must not be treated as the export body"
        );
        // Signature spans the whole declaration (no truncation), since
        // there's no fn-init body to elide behind. The source is three
        // lines long ("export const cfg = buildConfig(input, () => {"
        // / "  return 1;" / "}, options);").
        let lines = collect_export_lines(&tree, src, 1);
        assert_eq!(lines.full, vec![1, 2, 3]);
    }

    #[test]
    fn walker_typescript_export_body_const_memo_two_arg_emits_interior() {
        // memo(component, areEqual) — callback is the FIRST argument,
        // so descent applies. The optional `areEqual` second argument
        // doesn't change the wrapper interpretation.
        let src = "\
export const Cmp = memo((props) => {
  return null;
}, areEqual);
";
        let rows = body_emit_rows_for(src);
        assert_eq!(rows, vec![2]);
    }

    #[test]
    fn walker_typescript_export_body_const_forwardref_emits_interior() {
        // `forwardRef(props => {...})` — body lives inside the call's
        // arguments. `find_fn_init_body` must descend through the call
        // expression to find it.
        let src = "\
export const Item = React.forwardRef<HTMLDivElement, ItemProps>(
  (props, ref) => {
    const x = 1;
    return null;
  },
);
";
        let rows = body_emit_rows_for(src);
        assert_eq!(rows, vec![3, 4]);
    }

    #[test]
    fn walker_typescript_collect_local_value_reexports_excludes_type_only() {
        let src = "\
const X = () => { return 1; };
const Y = () => { return 2; };
const Z = () => { return 3; };
const W = () => { return 4; };
export type { X };
export { type Y };
export { Z };
export { W as Renamed };
";
        let tree = parse(src);
        let names = collect_local_value_reexports(&tree, src);
        assert!(
            !names.contains("X"),
            "type-only stmt-level should be excluded"
        );
        assert!(
            !names.contains("Y"),
            "inline type modifier should be excluded"
        );
        assert!(names.contains("Z"));
        assert!(names.contains("W"));
    }

    #[test]
    fn walker_typescript_synthetic_const_export_via_reexport() {
        // Module-private `const Item = forwardRef(...)` re-exported by
        // name. `find_export_starts` should synthesize an Export at the
        // const's own start_line with body emit rows for the inner body.
        let src = "\
const Item = React.forwardRef((props, ref) => {
  return null;
});
export { Item as CommandItem };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        // Two entries: synthetic Const at line 1, NamedReexport at line 4.
        assert_eq!(exports.len(), 2);
        let synth = exports.iter().find(|e| e.start_line == 1).unwrap();
        assert!(matches!(synth.kind, ItemKind::Const));
        assert!(!synth.body_parts.is_empty());
    }

    #[test]
    fn walker_typescript_synthetic_const_only_when_value_reexport() {
        // `export type { X }` does not synthesize a value Export for X.
        let src = "\
const X = () => { return 1; };
export type { X };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        // Only the export_statement at line 2 (type-only re-export) is
        // recognized as a real export. No synthetic Const.
        assert!(exports.iter().all(|e| e.start_line != 1));
    }

    #[test]
    fn walker_typescript_synthetic_export_skipped_when_real_export_shares_line() {
        // Same-line collision: `const X = () => {...}; export { X };`
        // The synthetic candidate would alias on `(file, start_line)`
        // with the real export_statement; drop the synthetic, keep the
        // real NamedReexport.
        let src = "const X = () => { return 1; }; export { X };\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(exports.len(), 1, "should keep only the real export");
        assert!(matches!(exports[0].kind, ItemKind::NamedReexport));
    }

    #[test]
    fn walker_typescript_locate_export_decl_real_wins_on_same_line() {
        // Materializer-side mirror of the §4c collision filter.
        let src = "const X = () => { return 1; }; export { X };\n";
        let tree = parse(src);
        let located = locate_export_decl(&tree, src, 1).unwrap();
        // Must be the real NamedReexport, not a synthetic Const.
        assert!(matches!(located.kind(), ItemKind::NamedReexport));
        let lines = collect_export_lines(&tree, src, 1);
        // Real export rendering is the whole single-line clause —
        // contains line 1 only.
        assert_eq!(lines.full, vec![1]);
        assert!(
            export_body_parts_for_start(&tree, src, 1).is_empty(),
            "NamedReexport has no body"
        );
    }

    #[test]
    fn walker_typescript_synthetic_const_skips_multi_binding() {
        // `const a = () => {...}, b = () => {...}` — multi-binding form
        // can't be keyed by start_line cleanly; skip even when names
        // are reexported.
        let src = "\
const a = () => { return 1; }, b = () => { return 2; };
export { a, b };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports.iter().all(|e| e.start_line != 1));
    }

    #[test]
    fn walker_typescript_esm_function_reexport_does_not_synthesize_local_function() {
        let src = "\
function composeRefs() {
  return null;
}
function useComposedRefs() {
  return composeRefs();
}
export { composeRefs, useComposedRefs };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(exports.len(), 1);
        assert_eq!(exports[0].start_line, 7);
        assert!(matches!(exports[0].kind, ItemKind::NamedReexport));
    }

    #[test]
    fn walker_typescript_commonjs_synthesizes_exported_class_and_function() {
        let src = "\
class Command {
  parse() {
    return this;
  }
}
function createCommand() {
  return new Command();
}
exports.Command = Command;
exports.createCommand = createCommand;
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        let class_export = exports.iter().find(|e| e.start_line == 1).unwrap();
        let function_export = exports.iter().find(|e| e.start_line == 6).unwrap();
        assert!(matches!(class_export.kind, ItemKind::Class));
        assert!(matches!(function_export.kind, ItemKind::Function));
        assert_eq!(class_export.body_parts[0].lines, vec![3]);
        assert_eq!(function_export.body_parts[0].lines, vec![7]);
        assert!(
            exports.iter().any(|e| e.start_line == 9),
            "CommonJS assignment line should remain as an export surface"
        );
    }

    #[test]
    fn walker_typescript_commonjs_direct_assignment_is_export_surface() {
        let src = "\
const { Command } = require('./command.js');
exports.program = new Command();
exports.createCommand = (name) => new Command(name);
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(exports.len(), 2);
        assert!(
            exports
                .iter()
                .any(|e| { e.start_line == 2 && matches!(e.kind, ItemKind::Const) })
        );
        assert!(
            exports
                .iter()
                .any(|e| { e.start_line == 3 && matches!(e.kind, ItemKind::Function) })
        );
        let lines = collect_export_lines(&tree, src, 3);
        assert_eq!(lines.full, vec![3]);
    }

    #[test]
    fn walker_typescript_commonjs_ignores_exports_prefixed_identifier() {
        let src = "\
function Command() {
  return null;
}
exportsNotReally.Command = Command;
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports.is_empty());
    }

    #[test]
    fn walker_typescript_commonjs_module_exports_object_names_locals() {
        let src = "\
class Help {}
function stripColor() {
  return '';
}
module.exports = { Help, stripColor };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(
            exports
                .iter()
                .any(|e| { e.start_line == 1 && matches!(e.kind, ItemKind::Class) })
        );
        assert!(
            exports
                .iter()
                .any(|e| { e.start_line == 2 && matches!(e.kind, ItemKind::Function) })
        );
        assert!(
            exports.iter().any(|e| e.start_line == 5),
            "module.exports assignment should also be a direct export surface"
        );
    }

    #[test]
    fn walker_typescript_export_body_default_class_method_bodies() {
        let src = "\
export default class C {
  bar() {
    return 1;
  }
}
";
        let rows = body_emit_rows_for(src);
        // Method `bar` body interior: line 3.
        assert_eq!(rows, vec![3]);
    }

    #[test]
    fn walker_typescript_export_body_skips_blank_interior_rows() {
        // Blank source lines inside the body shouldn't be counted —
        // build_file_spans filters them out, so the emit-rows helper must too.
        let src = "\
export function foo() {

  let x = 1;

  return x;
}
";
        let rows = body_emit_rows_for(src);
        // Lines 2 and 4 are blank; only 3 and 5 are emitted.
        assert_eq!(rows, vec![3, 5]);
    }

    #[test]
    fn walker_typescript_type_machinery_pure_type_aliases() {
        // All exports are `type` aliases → file is type-machinery.
        let src = "\
export type A = number;
export type B = string;
export interface C { x: number }
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports.iter().all(|e| e.is_type_only));
        assert_eq!(
            type_machinery_factor(Path::new("foo.ts"), &exports),
            TYPE_MACHINERY_FILE_FACTOR
        );
    }

    #[test]
    fn walker_typescript_type_machinery_excludes_enum() {
        // TS `enum` emits a runtime object — not type-only. A file with
        // any enum is not type-machinery even if every other export is
        // a type alias.
        let src = "\
export type A = number;
export enum E { X, Y }
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(type_machinery_factor(Path::new("foo.ts"), &exports), 1.0);
    }

    #[test]
    fn walker_typescript_type_machinery_excludes_runtime_export() {
        // A single runtime export disqualifies the file.
        let src = "\
export type A = number;
export type B = string;
export const x = 1;
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(type_machinery_factor(Path::new("foo.ts"), &exports), 1.0);
    }

    #[test]
    fn walker_typescript_type_machinery_export_type_clause() {
        // `export type { Foo }` is a NamedReexport with the statement-level
        // `type` keyword — counts as type-only.
        let src = "\
type A = number;
export type { A };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports.iter().all(|e| e.is_type_only));
        assert_eq!(
            type_machinery_factor(Path::new("foo.ts"), &exports),
            TYPE_MACHINERY_FILE_FACTOR
        );
    }

    #[test]
    fn walker_typescript_type_machinery_value_reexport_not_type_only() {
        // Bare `export { X }` (value re-export) is conservatively non-
        // type-only even when X happens to alias a type.
        let src = "\
const X = () => 1;
export { X };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        // The synthetic Const at line 1 plus the NamedReexport at line 2.
        assert_eq!(type_machinery_factor(Path::new("foo.ts"), &exports), 1.0);
    }

    #[test]
    fn walker_typescript_type_machinery_inline_type_modifier_not_type_only() {
        // Inline `export { type Foo }` (per-specifier modifier) is not
        // statement-level — conservatively classified as runtime so a
        // mixed `export { type Foo, valueY }` doesn't get penalized.
        let src = "\
type A = number;
const v = 1;
export { type A, v };
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(type_machinery_factor(Path::new("foo.ts"), &exports), 1.0);
    }

    #[test]
    fn walker_typescript_type_machinery_empty_file_not_type_only() {
        // A file with zero exports is not type-machinery (the multiplier
        // would have nothing to apply to anyway).
        let src = "import { foo } from './bar';\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports.is_empty());
        assert_eq!(type_machinery_factor(Path::new("foo.ts"), &exports), 1.0);
    }

    #[test]
    fn walker_typescript_type_machinery_dts_file_always_type_only() {
        // A `.d.ts` file emits no runtime — even runtime-shaped exports
        // (`export class`, `export const`) are implicitly ambient. Path
        // alone qualifies the file regardless of export kinds.
        let src = "\
export class C { foo(): void; }
export const x: number;
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(
            type_machinery_factor(Path::new("typings/index.d.ts"), &exports),
            TYPE_MACHINERY_FILE_FACTOR
        );
    }

    #[test]
    fn walker_typescript_type_machinery_export_declare_is_type_only() {
        // `export declare ...` in a regular `.ts` file is ambient — the
        // .ts file emits no runtime for the declaration.
        let src = "\
export declare function foo(): void;
export declare const x: number;
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports.iter().all(|e| e.is_type_only));
        assert_eq!(
            type_machinery_factor(Path::new("foo.ts"), &exports),
            TYPE_MACHINERY_FILE_FACTOR
        );
    }
}
