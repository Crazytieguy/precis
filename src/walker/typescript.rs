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

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, TsKey};
use crate::content::{BatchContent, Render, Span};
use crate::value::{
    NAMES_SURFACE_CHUNK_SIZE, mix_signals, names_surface_chunk_count, names_surface_chunk_factor,
    reexport_import_chunk_factor, roster_mass_factor,
};

use super::import_chunks::{
    ImportGroup, REEXPORT_IMPORT_MAX_OTHER_LINES, REEXPORT_IMPORT_MAX_OTHER_STATEMENTS,
    groups_to_file_lines, node_line_count, push_import_group, should_chunk_import_groups,
};
use super::{
    BodyPart, FileLines, WalkCtx, body_part_value_factor, build_per_file_content, dedup_sorted,
    extend_nonblank_rows, extend_span, file_depth_factor, file_lines_covered_by,
    fs::{JS_MODULE_ENTRYPOINT_FILES, files_with_any_extension, is_source_dir},
    name_of, node_end_row_trimmed, push_rows, signature_end_row, single_file_lines_content,
    statement_block_parts,
};

/// First N body segments per file keep full value; later segments
/// drop to `LATE_BODY_SEGMENT_VALUE_FACTOR`.
const FULL_VALUE_BODY_SEGMENTS_PER_FILE: usize = 4;
const LATE_BODY_SEGMENT_VALUE_FACTOR: f64 = 0.05;
const JS_CLASS_MEMBER_SPLIT_MIN: usize = 12;
/// Upper bound on class member count for the per-method split — above
/// this, the per-method `ExportMember` batches dominate the early
/// budget on concavity without satisfying any catalog-shape NS row.
const JS_CLASS_MEMBER_SPLIT_MAX: usize = 40;
/// Minimum prototype-method assignments in a JS file for the synthesized
/// per-method exports to fire. Below this floor a couple incidental
/// `thing.helper = function …` lines shouldn't hijack the file shape.
const JS_PROTOTYPE_METHOD_MIN: usize = 3;

/// Per-run TypeScript-walker state. Caches the project's "public
/// surface" (entrypoint files + transitively re-exported targets) so
/// items in non-surface files can be damped as internal.
#[derive(Default)]
pub struct TypescriptState {
    public_surface: OnceCell<PublicSurface>,
    in_surface_lookup: RefCell<HashMap<PathBuf, bool>>,
    nearest_subpackage_dir_lookup: RefCell<HashMap<PathBuf, Option<PathBuf>>>,
    declared_api_contract: OnceCell<Option<PathBuf>>,
    pinned_entrypoint_lookup: RefCell<HashMap<PathBuf, bool>>,
    api_spine_lookup: RefCell<HashMap<PathBuf, bool>>,
    /// `package_entry_targets` results per package dir — the manifest
    /// is immutable for the run, and the uncached form re-parses it
    /// once per entrypoint-named file in the package.
    entry_targets_lookup: RefCell<HashMap<PathBuf, Vec<String>>>,
}

/// Reachability results over the TS/JS import graph, canonicalized.
#[derive(Default)]
struct PublicSurface {
    /// Entrypoints plus everything transitively reachable from them.
    files: HashSet<PathBuf>,
    /// Files reached via a genuine re-export edge — excludes the
    /// entrypoint self-seeding and the app-fallback import chain.
    reexport_targets: HashSet<PathBuf>,
}

impl TypescriptState {
    pub fn new() -> Self {
        Self::default()
    }

    /// `true` iff `file` is in the project's TS/JS public surface.
    pub fn is_in_public_surface(&self, file: &Path, ctx: &WalkCtx) -> bool {
        if let Some(&hit) = self.in_surface_lookup.borrow().get(file) {
            return hit;
        }
        let surface = self
            .public_surface
            .get_or_init(|| compute_public_surface(ctx));
        let canonical = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
        let hit = surface.files.contains(&canonical);
        self.in_surface_lookup
            .borrow_mut()
            .insert(file.to_path_buf(), hit);
        hit
    }

    /// `true` iff `file` is re-exported by another file in the public
    /// surface (a genuine re-export edge, not entrypoint self-seeding).
    fn is_reexport_target(&self, file: &Path, ctx: &WalkCtx) -> bool {
        let surface = self
            .public_surface
            .get_or_init(|| compute_public_surface(ctx));
        let canonical = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
        surface.reexport_targets.contains(&canonical)
    }

    /// The package's declared API contract — the root `package.json`'s
    /// `types` / `typings` target, when it is a root-level declaration
    /// file that exists in the repo. Canonicalized. Root-level only:
    /// nested targets (`source/index.d.ts`, `typings/index.d.ts`,
    /// generated `dist/…`) are type plumbing next to real source, and
    /// promoting them measured as a regression (see design-notes
    /// "Entrypoint-named `.d.ts` promotion").
    pub(in crate::walker) fn declared_api_contract(&self, root: &Path) -> Option<&PathBuf> {
        self.declared_api_contract
            .get_or_init(|| {
                let target = super::json::declared_types_target(root)?;
                if !is_declaration_file(&target) || target.parent() != Some(root) {
                    return None;
                }
                target.canonicalize().ok()
            })
            .as_ref()
    }

    /// Walk up from `file` to the nearest dir under `root` that contains
    /// its own `package.json`. Memoized.
    pub(in crate::walker) fn nearest_subpackage_dir(
        &self,
        file: &Path,
        root: &Path,
    ) -> Option<PathBuf> {
        let key = file.to_path_buf();
        if let Some(hit) = self.nearest_subpackage_dir_lookup.borrow().get(&key) {
            return hit.clone();
        }
        let mut result = None;
        let mut dir = file.parent();
        while let Some(current) = dir {
            if current == root {
                break;
            }
            if !current.starts_with(root) {
                break;
            }
            if current.join("package.json").is_file() {
                result = Some(current.to_path_buf());
                break;
            }
            dir = current.parent();
        }
        self.nearest_subpackage_dir_lookup
            .borrow_mut()
            .insert(key, result.clone());
        result
    }
}

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
        let js_factor = js_value_factor(file, ctx)
            * public_surface_factor(file, ctx)
            * secondary_ts_workspace_member_factor(file, ctx);
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

        // Lines claimed by an imports / re-export-chunk batch on this
        // file. Threaded into `collect_export_names_from` so the
        // names-surface ellipsis marker never lands on a line owned by
        // another peer batch — peers don't form an ancestor chain, so
        // an overlap would be a scheduler conflict.
        let mut import_owned_lines: HashSet<usize> = HashSet::new();
        if let Some((source, tree)) = parse_ts(ctx, file) {
            let api_spine = ep && is_api_spine_entrypoint(file, ctx);
            if let Some(chunks) = collect_reexport_import_chunks(file, &tree, &source) {
                let chunk_count = chunks.len();
                for (chunk_index, (lines, is_reexport_wall)) in chunks.into_iter().enumerate() {
                    import_owned_lines.extend(lines.full.iter().copied());
                    let Some(content) = single_file_lines_content(file, &source, lines) else {
                        continue;
                    };
                    let base_value = if is_reexport_wall && api_spine {
                        reexport_wall_value(file, ctx, js_factor)
                    } else {
                        imports_value(file, ctx, js_factor)
                    };
                    out.push(Batch {
                        key: TsKey::ImportChunk {
                            file: file.clone(),
                            chunk_index,
                        }
                        .into(),
                        predecessor: module_predecessor.clone(),
                        content,
                        value: base_value * reexport_import_chunk_factor(chunk_index, chunk_count),
                    });
                }
            } else {
                let lines = collect_imports(&tree, &source);
                import_owned_lines.extend(lines.full.iter().copied());
                let is_reexport_wall = is_entrypoint_file(file)
                    && api_spine
                    && is_mostly_reexport(&lines, &collect_bare_reexport_lines(&tree, &source));
                if let Some(content) = single_file_lines_content(file, &source, lines) {
                    out.push(Batch {
                        key: TsKey::Imports { file: file.clone() }.into(),
                        predecessor: module_predecessor.clone(),
                        content,
                        value: if is_reexport_wall {
                            reexport_wall_value(file, ctx, js_factor)
                        } else {
                            imports_value(file, ctx, js_factor)
                        },
                    });
                }
            }
        }

        let Some((source, tree)) = parse_ts(ctx, file) else {
            continue;
        };
        let src_lines: Vec<&str> = source.lines().collect();
        let exports = find_export_starts(file, &tree, &source, &src_lines);
        // The declared API contract's roster shapes (names surface,
        // member-chunked decls and their catalogs) are exempt from the
        // machinery damp: a root-level `.d.ts` the manifest's `types`
        // field names is the package's public API surface, and its big
        // member catalogs are what NS authors anchor on. Small
        // non-roster exports keep the damp — exempting them measured as
        // a cost-ascending flood of type aliases that displaces
        // NS-credited orientation without earning catalog credit.
        let is_api_contract = is_declared_api_contract(file, ctx);
        let per_export_factor = type_machinery_factor(file, &exports);
        let contract_roster_factor = |has_member_chunks: bool| {
            if is_api_contract && has_member_chunks {
                1.0
            } else {
                per_export_factor
            }
        };
        let export_start_lines: HashSet<_> = exports.iter().map(|item| item.start_line).collect();
        let mut body_segment_index = 0usize;
        if !exports.is_empty() {
            // Prototype-style JS files surface their public API as one
            // catalog ("Application prototype — method names"); the
            // method assignments are semantically a single class. Skip
            // chunking so the catalog lands as one orientation batch
            // rather than splitting an inherently-coherent name list.
            let has_prototype_method = exports.iter().any(|item| item.is_prototype_method);
            let chunk_count = if has_prototype_method {
                1
            } else {
                names_surface_chunk_count(exports.len())
            };
            let chunk_size = if has_prototype_method {
                exports.len().max(NAMES_SURFACE_CHUNK_SIZE)
            } else {
                NAMES_SURFACE_CHUNK_SIZE
            };
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
            let names_lines_by_chunk: Vec<_> = exports
                .chunks(chunk_size)
                .map(|chunk| {
                    collect_export_names_from(chunk, &export_start_lines, &import_owned_lines)
                })
                .collect();
            for (chunk_index, names_lines) in names_lines_by_chunk.iter().enumerate() {
                let Some(content) = single_file_lines_content(file, &source, names_lines.clone())
                else {
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
                    ) * contract_roster_factor(true),
                });
            }
            for (item_index, item) in exports.iter().enumerate() {
                let chunk_index = item_index / chunk_size;
                let names_predecessor = names_predecessors[chunk_index].clone();
                let export_key = TsKey::Export {
                    file: file.clone(),
                    start_line: item.start_line,
                };
                let split_js_class = should_split_js_class_export(file, item);
                let member_names_chunks = if split_js_class {
                    None
                } else {
                    member_names_chunk_lines(item.kind, item.decl, &source)
                };
                let export_lines = if split_js_class || member_names_chunks.is_some() {
                    // Chunked declarations trade the whole-member surface
                    // for a cheap header; the members arrive via the
                    // gated `ExportMemberNames` chunks instead.
                    header_surface_lines(
                        item.anchor,
                        member_surface_body(item.kind, item.decl),
                        item.decl,
                        &source,
                    )
                } else {
                    decl_surface_lines(item.kind, item.anchor, item.decl, &source, true)
                };
                let mut doc_lines = Vec::new();
                collect_jsdoc_above(
                    item.anchor,
                    &source,
                    &mut doc_lines,
                    is_entrypoint_file(file),
                );
                let export_has_descendants = !doc_lines.is_empty()
                    || (split_js_class && !item.class_members.is_empty())
                    || member_names_chunks.is_some()
                    || (!split_js_class && !item.body_parts.is_empty());
                if (!file_lines_covered_by(&export_lines, &names_lines_by_chunk[chunk_index])
                    || export_has_descendants)
                    && let Some(content) = single_file_lines_content(file, &source, export_lines)
                {
                    out.push(Batch {
                        key: export_key.clone().into(),
                        predecessor: Some(names_predecessor.clone()),
                        content,
                        value: export_value(file, item.kind, ctx, js_factor)
                            * contract_roster_factor(member_names_chunks.is_some()),
                    });
                }
                let export_predecessor = BatchKey::Typescript(export_key);
                // Body parts of a chunked declaration hang behind the
                // final chunk: the member catalog is the better buy at
                // every budget, and a sibling body batch could collide
                // with chunk ellipsis markers (non-ancestor overlap).
                let mut body_parts_predecessor = export_predecessor.clone();
                if let Some(member_chunks) = member_names_chunks {
                    let chunk_count = member_chunks.chunks.len();
                    let member_count: usize = member_chunks
                        .chunks
                        .iter()
                        .map(|chunk| chunk.full.len())
                        .sum();
                    let mut chunk_predecessor = export_predecessor.clone();
                    for (chunk_index, lines) in member_chunks.chunks.into_iter().enumerate() {
                        let content = if member_chunks.truncate_to_name {
                            truncated_member_names_content(file, &source, &lines)
                        } else {
                            single_file_lines_content(file, &source, lines)
                        };
                        let Some(content) = content else {
                            continue;
                        };
                        let key = TsKey::ExportMemberNames {
                            file: file.clone(),
                            start_line: item.start_line,
                            chunk_index,
                        };
                        out.push(Batch {
                            key: key.clone().into(),
                            predecessor: Some(chunk_predecessor),
                            content,
                            value: export_member_names_value(
                                file,
                                item.kind,
                                ctx,
                                js_factor,
                                chunk_index,
                                chunk_count,
                                member_count,
                            ) * contract_roster_factor(true),
                        });
                        chunk_predecessor = BatchKey::Typescript(key);
                    }
                    body_parts_predecessor = chunk_predecessor;
                }
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
                        &body_parts_predecessor,
                    );
                }
            }
        }
        let module_items = find_module_items(&tree, &source, &src_lines, &export_start_lines);
        // README-cited JS files (canonical example scripts referenced from
        // the root README) emit private statements as module items even
        // though they aren't entrypoints — those statements ARE the
        // example's content the NS author anchored on.
        let emit_private_nonclass = (is_entrypoint_file(file) || ctx.is_readme_cited(file))
            && (is_tsx_file(file) || is_js_file(file));
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
                // Module-level class method bodies are sibling units (one
                // per method), each independently relevant — unlike
                // function body fragments which compete as alternatives.
                // No per-part damping so a method body can compete on
                // its own merit against orientation batches.
                let part_value_factor = if matches!(item.kind, ItemKind::Class) {
                    1.0
                } else {
                    body_part_value_factor(parts.len())
                };
                for part in parts {
                    let Some(body_start_line) = part.start_line() else {
                        continue;
                    };
                    let Some(content) =
                        single_file_lines_content(file, &source, FileLines::new(part.lines))
                    else {
                        continue;
                    };
                    // Class method bodies are peer units (one per method);
                    // bypass the late-body decay that targets long
                    // function-body chains in catalog files.
                    let segment_factor = if matches!(item.kind, ItemKind::Class) {
                        1.0
                    } else {
                        body_segment_value_factor(body_segment_index)
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
                            * segment_factor,
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
    // Factory body parts are sibling anchors; skip the per-partition
    // value damping.
    let part_value_factor = if item.factory_sibling_body_parts {
        1.0
    } else {
        body_part_value_factor(parts.len())
    };
    // Class method bodies are peer units — each is its own semantic
    // unit so the late-body decay doesn't apply.
    let is_class_peer = matches!(item.kind, ItemKind::Class | ItemKind::Default);
    for part in parts {
        let Some(body_start_line) = part.start_line() else {
            continue;
        };
        let Some(content) =
            single_file_lines_content(emit.file, emit.source, FileLines::new(part.lines))
        else {
            continue;
        };
        // Factory siblings and class peers skip late-segment damping.
        let segment_factor = if item.factory_sibling_body_parts || is_class_peer {
            1.0
        } else {
            body_segment_value_factor(*emit.body_segment_index)
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
                * segment_factor,
        });
        if !is_class_peer {
            *emit.body_segment_index += 1;
        }
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
        .find(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| JS_MODULE_ENTRYPOINT_FILES.contains(&name))
        })
        .cloned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemKind {
    Interface,
    TypeAlias,
    Class,
    Enum,
    Function,
    Const,
    /// `export default …` (expression / class expr / function expr).
    Default,
    /// `export { foo }` / `export type { Pattern }` — single-line span,
    /// declaration target not resolved.
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
        && (JS_CLASS_MEMBER_SPLIT_MIN..=JS_CLASS_MEMBER_SPLIT_MAX)
            .contains(&item.class_members.len())
}

#[derive(Debug, Clone)]
struct ExportInfo<'a> {
    /// 1-based line of the wrapping `export_statement`, or the local
    /// `lexical_declaration` for synthetic re-exports.
    start_line: usize,
    kind: ItemKind,
    /// Anchor for the declaration surface + JSDoc.
    anchor: Node<'a>,
    decl: Node<'a>,
    body_parts: Vec<BodyPart>,
    class_members: Vec<ClassMemberInfo>,
    /// True for type-only exports — flags whole files as type-machinery
    /// for damping. `Enum` is NOT type-only.
    is_type_only: bool,
    /// True when synthesized from a CommonJS prototype-style method
    /// assignment.
    is_prototype_method: bool,
    /// True when `body_parts` are sibling anchors from a factory match
    /// — the emitter skips per-part value damping.
    factory_sibling_body_parts: bool,
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

/// Top-level exports in a file. Walks `program` for `export_statement`
/// then for module-private `const X = <fn-init>` re-exported by name —
/// emits a synthesized `ExportInfo` for each. Bare `export { foo }
/// from '…'` is skipped (picked up by `Imports`).
fn find_export_starts<'a>(
    file: &Path,
    tree: &'a Tree,
    source: &str,
    src_lines: &[&str],
) -> Vec<ExportInfo<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    let mut emitted_lines: HashSet<usize> = HashSet::new();
    for child in root.children(&mut cursor) {
        let Some((kind, decl_node)) =
            classify_export(child, source).or_else(|| classify_commonjs_export(child, source))
        else {
            continue;
        };
        let start_line = child.start_position().row + 1;
        emitted_lines.insert(start_line);
        let is_type_only = is_export_type_only(kind, child, source);
        out.push(make_export_info(
            start_line,
            kind,
            child,
            decl_node,
            file,
            source,
            src_lines,
            is_type_only,
            false,
        ));
    }

    let reexports = collect_local_value_reexports(tree, source);
    let commonjs_reexports = collect_commonjs_value_reexports(tree, source);
    let mut needs_sort = false;
    if !reexports.is_empty() || !commonjs_reexports.is_empty() {
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
                start_line, kind, child, child, file, source, src_lines, false, false,
            ));
            emitted_lines.insert(start_line);
            needs_sort = true;
        }
    }

    // CommonJS prototype-style method assignments — JS only.
    if is_js_file(file) {
        let receivers = collect_module_exports_receivers(tree, source);
        if !receivers.is_empty() {
            let methods = collect_prototype_method_assignments(tree, source, &receivers);
            if methods.len() >= JS_PROTOTYPE_METHOD_MIN {
                for method in methods {
                    if emitted_lines.contains(&method.start_line) {
                        continue;
                    }
                    out.push(make_export_info(
                        method.start_line,
                        ItemKind::Function,
                        method.anchor,
                        method.fn_expr,
                        file,
                        source,
                        src_lines,
                        false,
                        true,
                    ));
                    emitted_lines.insert(method.start_line);
                    needs_sort = true;
                }
            }
        }
    }

    if needs_sort {
        out.sort_by_key(|e| e.start_line);
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn make_export_info<'a>(
    start_line: usize,
    kind: ItemKind,
    anchor: Node<'a>,
    decl: Node<'a>,
    file: &Path,
    source: &str,
    src_lines: &[&str],
    is_type_only: bool,
    is_prototype_method: bool,
) -> ExportInfo<'a> {
    let collect_class_members = is_js_file(file)
        && matches!(kind, ItemKind::Class | ItemKind::Default)
        && is_class_node(decl);
    let class_members = if collect_class_members {
        class_member_infos(decl, src_lines)
    } else {
        Vec::new()
    };
    let factory_sibling_body_parts =
        !collect_class_members && is_factory_body_match(decl, kind, source, src_lines);
    let body_parts = if collect_class_members {
        merged_body_parts(
            class_members
                .iter()
                .flat_map(|member| member.body_parts.clone())
                .collect(),
        )
    } else {
        let parts = body_parts(decl, kind, source, src_lines);
        // Factory bodies expose their public surface as two distinct
        // anchors (receiver table + inner-function locations); keep
        // those parts as separate ExportBody batches rather than
        // merging the table into the catalog (which would inflate the
        // body cost out of the auto-injection budget).
        if factory_sibling_body_parts {
            parts
        } else {
            merged_body_parts(parts)
        }
    };
    ExportInfo {
        start_line,
        kind,
        anchor,
        decl,
        body_parts,
        class_members,
        is_type_only,
        is_prototype_method,
        factory_sibling_body_parts,
    }
}

/// Top-level decls not already covered by `find_export_starts` — the
/// module-private items behind a thin exported API.
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
        let body_parts = body_parts(child, kind, source, src_lines);
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

/// True when an export carries no runtime value. `Enum` and
/// `Class`/`Function`/`Const`/`Default` are runtime; `Interface`/
/// `TypeAlias` always type-only; `NamedReexport` only when the
/// statement carries the `type` keyword. `export declare …` is
/// type-only (ambient).
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

/// True when this `export_statement` wraps an `ambient_declaration`
/// (`export declare …`).
fn has_ambient_declaration(stmt: Node) -> bool {
    let mut cursor = stmt.walk();
    stmt.children(&mut cursor)
        .any(|c| c.kind() == "ambient_declaration")
}

/// True for TypeScript declaration files (`.d.ts` / `.d.tsx`).
pub(crate) fn is_declaration_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.ends_with(".d.ts") || n.ends_with(".d.tsx"))
}

/// `TYPE_MACHINERY_FILE_FACTOR` when the file emits no runtime code —
/// `.d.ts` or every top-level export is type-only. `ExportNames` is
/// intentionally NOT damped so the names surface stays visible.
/// Callers exempt the declared API contract (see
/// [`is_declared_api_contract`]) before applying this factor.
fn type_machinery_factor(file: &Path, exports: &[ExportInfo<'_>]) -> f64 {
    if is_declaration_file(file) || (!exports.is_empty() && exports.iter().all(|e| e.is_type_only))
    {
        TYPE_MACHINERY_FILE_FACTOR
    } else {
        1.0
    }
}

/// True iff `file` is the package's declared API contract (see
/// [`TypescriptState::declared_api_contract`]).
fn is_declared_api_contract(file: &Path, ctx: &WalkCtx) -> bool {
    let Some(contract) = ctx.typescript_state().declared_api_contract(ctx.root()) else {
        return false;
    };
    file.canonicalize().is_ok_and(|c| &c == contract)
}

const TYPE_MACHINERY_FILE_FACTOR: f64 = 0.35;

/// Local identifier names in top-level value re-export clauses
/// (`export { X }` / `export { X as Y }`). Excludes type-only and
/// any clause with a `from` source.
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

/// Local identifier names on the RHS of top-level CommonJS export
/// assignments (`exports.Foo = Foo` / `module.exports = { Foo }`).
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
            out.extend(object_value_names(right, source));
        } else if let Some(name) = identifier_text(right, source) {
            out.insert(name.to_string());
        }
    }
    out
}

/// Local identifier names bound on the value side of an `object` literal:
/// shorthand `{ A }` yields `A`; `{ key: A }` yields `A`; computed/spread
/// children are skipped. Used by `module.exports = { … }` collectors that
/// need to map each entry to its local declaration name.
fn object_value_names(node: Node, source: &str) -> Vec<String> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter_map(|child| match child.kind() {
            "shorthand_property_identifier" | "identifier" => {
                Some(source[child.start_byte()..child.end_byte()].to_string())
            }
            "pair" => child
                .child_by_field_name("value")
                .and_then(|v| identifier_text(v, source).map(str::to_string)),
            _ => None,
        })
        .collect()
}

/// Identifier names bound to `module.exports` at file scope —
/// receivers whose `.X = function …` assignments become synthesized
/// exported methods. Includes constructors whose `.prototype` is the
/// receiver value.
fn collect_module_exports_receivers(tree: &Tree, source: &str) -> HashSet<String> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut receivers: HashSet<String> = HashSet::new();
    for stmt in root.children(&mut cursor) {
        match stmt.kind() {
            "expression_statement" => {
                if let Some(expr) = stmt.named_child(0)
                    && expr.kind() == "assignment_expression"
                {
                    collect_module_exports_receivers_from_assignment(expr, source, &mut receivers);
                }
            }
            "lexical_declaration" | "variable_declaration" => {
                let mut dc = stmt.walk();
                for declarator in stmt.children(&mut dc) {
                    if !matches!(declarator.kind(), "variable_declarator" | "lexical_binding") {
                        continue;
                    }
                    let Some(value) = declarator.child_by_field_name("value") else {
                        continue;
                    };
                    if !chain_contains_module_exports(value, source) {
                        continue;
                    }
                    let Some(name) = declarator
                        .child_by_field_name("name")
                        .and_then(|n| identifier_text(n, source))
                    else {
                        continue;
                    };
                    receivers.insert(name.to_string());
                }
            }
            _ => {}
        }
    }
    receivers
}

/// Extract receivers from a top-level `assignment_expression`. A
/// chain like `X = exports = module.exports = {}` admits every LHS
/// whose right-tail reaches `module.exports`.
fn collect_module_exports_receivers_from_assignment(
    assignment: Node,
    source: &str,
    receivers: &mut HashSet<String>,
) {
    let Some((left, right)) = assignment_sides(assignment) else {
        return;
    };
    // `module.exports = X` — X is the receiver.
    if matches!(
        commonjs_export_target(left, source),
        Some(CommonJsExportTarget::Namespace)
    ) && let Some(name) = identifier_text(right, source)
    {
        receivers.insert(name.to_string());
        return;
    }
    // `X = …` chain where `…` eventually assigns to `module.exports`.
    if chain_contains_module_exports(right, source)
        && let Some(name) = identifier_text(left, source)
    {
        receivers.insert(name.to_string());
        if right.kind() == "assignment_expression" {
            collect_module_exports_receivers_from_assignment(right, source, receivers);
        }
    }
}

/// True if `node` is `module.exports` or transitively assigns to it.
fn chain_contains_module_exports(node: Node, source: &str) -> bool {
    if is_module_exports_member(node, source) {
        return true;
    }
    if node.kind() != "assignment_expression" {
        return false;
    }
    let Some((left, right)) = assignment_sides(node) else {
        return false;
    };
    is_module_exports_member(left, source) || chain_contains_module_exports(right, source)
}

/// One `Receiver.member = function …` (or `.prototype.member = …`)
/// assignment on a tracked `module.exports`-aliased receiver.
#[derive(Debug, Clone)]
struct PrototypeMethodAssignment<'a> {
    start_line: usize,
    anchor: Node<'a>,
    fn_expr: Node<'a>,
}

/// Top-level prototype-style method assignments on any receiver in
/// `receivers`. Chained assignments surface once at the statement.
fn collect_prototype_method_assignments<'a>(
    tree: &'a Tree,
    source: &str,
    receivers: &HashSet<String>,
) -> Vec<PrototypeMethodAssignment<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() != "expression_statement" {
            continue;
        }
        let Some(expr) = stmt.named_child(0) else {
            continue;
        };
        if expr.kind() != "assignment_expression" {
            continue;
        }
        let Some(fn_expr) = prototype_method_fn_value(expr, source, receivers) else {
            continue;
        };
        out.push(PrototypeMethodAssignment {
            start_line: stmt.start_position().row + 1,
            anchor: stmt,
            fn_expr,
        });
    }
    out
}

/// If `assignment` is `R.m = function …` (or `R.prototype.m = function
/// …`, or a chain whose RHS ultimately resolves to a function expression
/// and whose innermost left targets a receiver), return that
/// function-valued node. Walks the right-spine across chained
/// assignments. Conservative: only function/arrow/generator expressions
/// count — pure-data assignments aren't methods.
fn prototype_method_fn_value<'a>(
    assignment: Node<'a>,
    source: &str,
    receivers: &HashSet<String>,
) -> Option<Node<'a>> {
    let (left, right) = assignment_sides(assignment)?;
    if !is_prototype_method_target(left, source, receivers) {
        return None;
    }
    let mut current = right;
    while current.kind() == "assignment_expression" {
        // Each intermediate left must also be a receiver-targeted member.
        // `req.get = req.header = function …` — both lefts qualify.
        // `app.x = somethingElse = function …` — bail unless the
        // something-else also looks like a receiver method target.
        let (inner_left, inner_right) = assignment_sides(current)?;
        if !is_prototype_method_target(inner_left, source, receivers) {
            return None;
        }
        current = inner_right;
    }
    matches!(
        current.kind(),
        "function_expression" | "arrow_function" | "generator_function"
    )
    .then_some(current)
}

/// True if `node` is a member-expression chain targeting one of the
/// tracked receivers — either `R.member` directly or `R.prototype.member`.
fn is_prototype_method_target(node: Node, source: &str, receivers: &HashSet<String>) -> bool {
    if node.kind() != "member_expression" {
        return false;
    }
    let Some((object, _property)) = member_object_property(node) else {
        return false;
    };
    if let Some(name) = identifier_text(object, source) {
        return receivers.contains(name);
    }
    // `R.prototype.member` — `object` is the inner `R.prototype`
    // member_expression.
    if object.kind() == "member_expression"
        && let Some((inner_object, inner_property)) = member_object_property(object)
        && identifier_eq(inner_property, source, "prototype")
        && let Some(name) = identifier_text(inner_object, source)
    {
        return receivers.contains(name);
    }
    false
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
    let mut expr = node.named_child(0)?;
    // Chained export assignments (`exports = module.exports = X`,
    // `module.exports = exports = X`) export X through whichever link
    // is the module.exports target: descend past bare-`exports` alias
    // links on the left, and unwrap assignment links on the right down
    // to the exported value.
    loop {
        if expr.kind() != "assignment_expression" {
            return None;
        }
        let (left, right) = assignment_sides(expr)?;
        if commonjs_export_target(left, source).is_some() {
            let mut value = right;
            while value.kind() == "assignment_expression" {
                let Some((_, inner)) = assignment_sides(value) else {
                    break;
                };
                value = inner;
            }
            return Some((left, value));
        }
        if !identifier_eq(left, source, "exports") {
            return None;
        }
        expr = right;
    }
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
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let Some((stem, ext)) = name.rsplit_once('.') else {
        return false;
    };
    matches!(stem, "index" | "main" | "mod" | "esm")
        && matches!(ext, "ts" | "tsx" | "js" | "mjs" | "cjs")
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

fn entrypoint_boost(path: &Path, ctx: &WalkCtx) -> f64 {
    if is_pinned_entrypoint(path, ctx) {
        1.4
    } else {
        1.0
    }
}

fn ts_depth_factor(path: &Path, ctx: &WalkCtx) -> f64 {
    file_depth_factor(path, ctx, is_pinned_entrypoint(path, ctx))
}

/// Entrypoint-named files get the depth pin + 1.4 boost only when they
/// plausibly speak for their package: within two components of the
/// package root (`index.ts`, `src/index.ts`) or on the export spine
/// (re-exported by another surface file / the declared entry's source
/// twin). A deep `index.ts` that is merely an exports-map subpath
/// (d2ts `electric/`, vite `module-runner/`) otherwise prices as the
/// package root and its NS-mid-rank content floods the early budget
/// ahead of the package's core modules. Memoized — this sits on every
/// per-batch value path.
fn is_pinned_entrypoint(path: &Path, ctx: &WalkCtx) -> bool {
    if !is_entrypoint_file(path) {
        return false;
    }
    let state = ctx.typescript_state();
    if let Some(&hit) = state.pinned_entrypoint_lookup.borrow().get(path) {
        return hit;
    }
    let package_dir_depth = state
        .nearest_subpackage_dir(path, ctx.root())
        .map(|dir| ctx.depth_from_root(&dir))
        .unwrap_or(0);
    let relative_depth = ctx.depth_from_root(path).saturating_sub(package_dir_depth);
    let hit = relative_depth <= 2 || is_api_spine_entrypoint(path, ctx);
    state
        .pinned_entrypoint_lookup
        .borrow_mut()
        .insert(path.to_path_buf(), hit);
    hit
}

const JS_CONFIG_VALUE_FACTOR: f64 = 0.001;
const PRIMARY_JS_VALUE_FACTOR: f64 = 0.85;
const SECONDARY_JS_VALUE_FACTOR: f64 = 0.05;

/// Multiplier applied to every per-file TS/JS batch: full weight for
/// files in the project's public surface (entrypoint-reachable via
/// re-export chains), partial weight otherwise. Acts as a public-vs-
/// private visibility axis — internal-only modules (`src/types/*` in
/// ts-pattern, `lib/helpers.js` not referenced by `index.js`) still
/// surface but rank below modules consumers can name from the
/// package entrypoint. Entrypoint files themselves are always in the
/// surface so the project's `index.ts` keeps its full boost.
fn public_surface_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if ctx.is_ts_public_surface(file) {
        1.0
    } else {
        0.5
    }
}

fn js_value_factor(path: &Path, ctx: &WalkCtx) -> f64 {
    if is_js_config_file(path) {
        // Dev-tooling JS should remain discoverable without taking budget
        // from source files in TS-first packages.
        JS_CONFIG_VALUE_FACTOR
    } else if is_primary_js_source_path(path, ctx) || ctx.is_readme_cited(path) {
        // Package JS/MJS/CJS entrypoints are often the whole public API, but
        // still need to rank below equivalent TS so TS fixtures stay stable.
        // README-cited JS files (e.g. canonical example scripts) are
        // primary by author intent — promote them out of the secondary tier.
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
    let cat = (0.8 * entrypoint_boost(file, ctx)).min(1.0);
    mix_signals(cat, 0.5, 0.9, ts_depth_factor(file, ctx)) * js_factor
}

fn imports_value(file: &Path, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let cat = (0.3 * entrypoint_boost(file, ctx)).min(1.0);
    mix_signals(cat, 0.55, 0.3, ts_depth_factor(file, ctx)) * js_factor
}

/// Damp factor for TS/JS files in a "secondary" sub-package — a
/// directory under root with its own `package.json` whose basename
/// doesn't match the root basename. The convention `secondary_factor`
/// keys off is *the primary sub-package shares the repo basename*:
/// `d2ts/packages/d2ts/`, `vite/packages/vite/`, `cmdk/cmdk/`. Anything
/// else under the root (`monaco-editor/webpack-plugin/`, `vite/packages/
/// plugin-legacy/`, `linkwarden/apps/worker/`) is auxiliary from the
/// repo's perspective — its imports/exports/per-item batches should
/// rank below the primary package's surface. Mirrors Rust's
/// `secondary_workspace_member_factor`.
///
/// Sub-package detection uses the nearest-enclosing-non-root
/// `package.json` (see `TypescriptState::nearest_subpackage_dir`). This
/// covers npm/yarn `workspaces`-declared layouts, pnpm-declared layouts,
/// and undeclared multi-package monorepos (monaco-editor) uniformly.
const SECONDARY_TS_SUBPACKAGE_FACTOR: f64 = 0.5;

fn secondary_ts_workspace_member_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    let Some(subpackage_dir) = ctx
        .typescript_state()
        .nearest_subpackage_dir(file, ctx.root())
    else {
        return 1.0;
    };
    let root_basename = ctx
        .root()
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    let member_basename = subpackage_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    if member_basename == root_basename {
        1.0
    } else {
        SECONDARY_TS_SUBPACKAGE_FACTOR
    }
}

/// An entrypoint's bare `export … from` wall is the package's public
/// API roster — the NS anchors on it as "the API in one screen" — not
/// plumbing imports. Priced on the names-surface axes instead of the
/// imports axes; the per-chunk falloff stays with the caller.
fn reexport_wall_value(file: &Path, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let cat = (0.8 * entrypoint_boost(file, ctx)).min(1.0);
    mix_signals(cat, 0.6, 0.35, ts_depth_factor(file, ctx)) * js_factor
}

/// Roster pricing applies only to walls on the package's export spine:
/// the file is re-exported by another surface file, or it is the
/// source twin of its package's declared entry. Subpath adapters
/// (`electric/`, `sqlite/` export paths) and secondary workspace
/// members keep imports pricing — boosting their walls measured
/// d2ts −0.023 / linkwarden −0.028 while the spine walls won.
fn is_api_spine_entrypoint(file: &Path, ctx: &WalkCtx) -> bool {
    let state = ctx.typescript_state();
    if let Some(&hit) = state.api_spine_lookup.borrow().get(file) {
        return hit;
    }
    let hit = secondary_ts_workspace_member_factor(file, ctx) >= 1.0
        && (state.is_reexport_target(file, ctx) || is_declared_package_entry_source(file, ctx));
    state
        .api_spine_lookup
        .borrow_mut()
        .insert(file.to_path_buf(), hit);
    hit
}

/// Generated-output prefixes a manifest entry path may carry; stripped
/// when mapping the entry back to its source twin.
const GENERATED_ENTRY_DIR_PREFIXES: &[&str] = &["dist", "build", "out", "output", "lib", "esm"];
/// Source-tree prefixes tried when re-rooting a generated entry path.
const SOURCE_ENTRY_DIR_PREFIXES: &[&str] = &["", "src", "source"];
const ENTRY_SOURCE_EXTS: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];

/// True when `file` is the source file behind its package's declared
/// entry (`main` / `module` / `exports["."]`) — either named directly,
/// or via the conventional generated-dir mapping (`./dist/node/index.js`
/// → `src/node/index.ts`).
fn is_declared_package_entry_source(file: &Path, ctx: &WalkCtx) -> bool {
    let root = ctx.root();
    let pkg_dir = ctx
        .typescript_state()
        .nearest_subpackage_dir(file, root)
        .unwrap_or_else(|| root.to_path_buf());
    let Ok(canonical_file) = file.canonicalize() else {
        return false;
    };
    let state = ctx.typescript_state();
    let targets = state
        .entry_targets_lookup
        .borrow_mut()
        .entry(pkg_dir.clone())
        .or_insert_with(|| super::json::package_entry_targets(&pkg_dir))
        .clone();
    for target in targets {
        let rel = target.trim_start_matches("./");
        if rel.is_empty() {
            continue;
        }
        let stem = Path::new(rel).with_extension("");
        let mut variants = vec![stem.clone()];
        for prefix in GENERATED_ENTRY_DIR_PREFIXES {
            if let Ok(stripped) = stem.strip_prefix(prefix) {
                variants.push(stripped.to_path_buf());
            }
        }
        for variant in &variants {
            for source_prefix in SOURCE_ENTRY_DIR_PREFIXES {
                let base = if source_prefix.is_empty() {
                    pkg_dir.join(variant)
                } else {
                    pkg_dir.join(source_prefix).join(variant)
                };
                for ext in ENTRY_SOURCE_EXTS {
                    if base.with_extension(ext).canonicalize().ok().as_ref()
                        == Some(&canonical_file)
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn export_names_value(
    file: &Path,
    ctx: &WalkCtx,
    chunk_index: usize,
    chunk_count: usize,
    js_factor: f64,
    has_split_js_class_export: bool,
) -> f64 {
    let cat = (0.8 * entrypoint_boost(file, ctx)).min(1.0);
    let class_split_factor = if has_split_js_class_export { 1.12 } else { 1.0 };
    mix_signals(cat, 0.6, 0.35, ts_depth_factor(file, ctx))
        * names_surface_chunk_factor(chunk_index, chunk_count)
        * js_factor
        * class_split_factor
}

fn export_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.70 * k * entrypoint_boost(file, ctx)).min(1.0);
    let fu = (0.85 * k).min(1.0);
    mix_signals(cat, fu, 0.65, ts_depth_factor(file, ctx)) * js_factor
}

fn export_doc_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.20 * k * entrypoint_boost(file, ctx)).min(1.0);
    let fu = (0.6 * k).min(1.0);
    mix_signals(cat, fu, 0.8, ts_depth_factor(file, ctx)) * js_factor
}

fn export_member_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.62 * k * entrypoint_boost(file, ctx)).min(1.0);
    let fu = (0.95 * k).min(1.0);
    mix_signals(cat, fu, 0.55, ts_depth_factor(file, ctx)) * js_factor
}

/// One chunk of a big declaration's member-name catalog — the
/// per-member surface priced as a roster slice: names-surface falloff
/// across chunks, roster-mass boost for the catalog's total size so
/// complete catalogs stay ratio-competitive with tiny exports.
fn export_member_names_value(
    file: &Path,
    kind: ItemKind,
    ctx: &WalkCtx,
    js_factor: f64,
    chunk_index: usize,
    chunk_count: usize,
    member_count: usize,
) -> f64 {
    export_member_value(file, kind, ctx, js_factor)
        * names_surface_chunk_factor(chunk_index, chunk_count)
        * roster_mass_factor(member_count)
}

/// Module-private classes carry per-method query value (constructors,
/// member sigs) — lift cat so each surface can compete against
/// peer-level orientation batches in the early budget.
fn module_item_class_boost(kind: ItemKind) -> f64 {
    if matches!(kind, ItemKind::Class) {
        1.5
    } else {
        1.0
    }
}

fn module_item_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight() * module_item_class_boost(kind);
    let cat = (0.38 * k * entrypoint_boost(file, ctx)).min(1.0);
    let fu = (0.7 * k).min(1.0);
    mix_signals(cat, fu, 0.55, ts_depth_factor(file, ctx)) * js_factor
}

fn module_item_body_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight() * module_item_class_boost(kind);
    let cat = (0.30 * k * entrypoint_boost(file, ctx)).min(1.0);
    let fu = (0.82 * k).min(1.0);
    mix_signals(cat, fu, 0.65, ts_depth_factor(file, ctx)) * js_factor
}

// Strictly below `Export.catastrophic` (0.70) — `Export`'s signature already
// hedges existence; the body is a refinement. Strictly above
// `Export.follow_up` (0.85) — body is the prime "don't go grep" signal.
fn export_body_value(file: &Path, kind: ItemKind, ctx: &WalkCtx, js_factor: f64) -> f64 {
    let k = kind.kind_weight();
    let cat = (0.45 * k * entrypoint_boost(file, ctx)).min(1.0);
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
/// after real code don't get folded in). Top-of-file side-effect
/// expression statements (`EventEmitter.defaultMaxListeners = 50`,
/// `process.env.X = …`, `Sentry.init(…)`) are skipped without
/// breaking — CommonJS entrypoints commonly interleave such setup
/// between requires.
fn collect_imports(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "expression_statement" if is_string_directive(child) => {
                extend_span(&mut lines, child, source)
            }
            "expression_statement" => {}
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
) -> Option<Vec<(FileLines, bool)>> {
    if !is_entrypoint_file(file) {
        return None;
    }
    let groups = collect_reexport_import_groups(tree, source)?;
    if !should_chunk_import_groups(&groups) {
        return None;
    }
    let reexport_lines = collect_bare_reexport_lines(tree, source);
    Some(
        groups_to_file_lines(groups)
            .into_iter()
            .map(|lines| {
                let is_wall = is_mostly_reexport(&lines, &reexport_lines);
                (lines, is_wall)
            })
            .collect(),
    )
}

/// 1-based lines of top-level bare `export … from` statements.
fn collect_bare_reexport_lines(tree: &Tree, source: &str) -> HashSet<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "export_statement" && is_bare_reexport(child) {
            extend_span(&mut lines, child, source);
        }
    }
    lines.into_iter().collect()
}

/// True when at least half of `lines` belong to bare re-export
/// statements — the batch is publishing the API roster, not importing.
fn is_mostly_reexport(lines: &FileLines, reexport_lines: &HashSet<usize>) -> bool {
    if lines.full.is_empty() {
        return false;
    }
    let hits = lines
        .full
        .iter()
        .filter(|line| reexport_lines.contains(line))
        .count();
    hits * 2 >= lines.full.len()
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
                    import_source_key(child, source),
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
                    import_source_key(child, source),
                    true,
                    child,
                    source,
                );
            }
            "export_statement" if is_bare_reexport(child) => {
                push_import_group(
                    &mut groups,
                    import_source_key(child, source),
                    true,
                    child,
                    source,
                );
            }
            // Single-line surface aliases sitting next to the wall — local
            // re-export clauses (`export [type] { X }` without a `from`)
            // and inline one-line `export const X = …` / `export type X
            // = …`. These are structurally wall items (forwarding /
            // aliasing locally-named identifiers) and shouldn't be
            // counted against the "other implementation" tolerance — the
            // pattern is common at the head and middle of TS package
            // entrypoints (e.g. vite's `node/index.ts` mixes 200+ bare
            // reexports with a handful of one-line `export const`
            // aliases). Their lines aren't pulled into any chunk —
            // `find_export_starts` already covers them.
            "export_statement" if is_wall_compatible_inline_export(child) => {}
            "comment" | "hash_bang_line" => {}
            _ => {
                other_statements += 1;
                other_lines += node_line_count(child);
                if other_statements > REEXPORT_IMPORT_MAX_OTHER_STATEMENTS
                    || other_lines > REEXPORT_IMPORT_MAX_OTHER_LINES
                {
                    return None;
                }
            }
        }
    }
    Some(groups)
}

fn source_literal(node: Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == "string")
        .map(|child| source[child.start_byte()..child.end_byte()].to_string())
}

fn import_source_key(node: Node, source: &str) -> String {
    source_literal(node, source)
        .unwrap_or_else(|| source[node.start_byte()..node.end_byte()].to_string())
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

/// True when an `export_statement` is a single-line surface alias / local
/// forward that should not interrupt a re-export wall — `export [type] {
/// X }` without a `from`, or one-line `export const X = …` / `export
/// type X = …` aliases. Multi-line bodies are real implementation and
/// stop the wall.
fn is_wall_compatible_inline_export(node: Node) -> bool {
    if node_line_count(node) > 1 {
        return false;
    }
    if has_export_clause(node) && !has_from_source(node) {
        return true;
    }
    first_decl_child(node).is_some()
}

fn collect_export_names_from(
    items: &[ExportInfo<'_>],
    export_start_lines: &HashSet<usize>,
    import_owned_lines: &HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for item in items {
        full.push(item.start_line);
        let ellipsis_line = item.start_line + 1;
        // Suppress the courtesy ellipsis when the next source line is
        // structural — another real export start, or a line owned by
        // the imports / re-export-chunk batch. The latter only matters
        // when an inline `export const` / `export type { … }` sits
        // adjacent to a bare re-export wall (TS entrypoint pattern):
        // peer batches can't overlap line ownership.
        if !export_start_lines.contains(&ellipsis_line)
            && !import_owned_lines.contains(&ellipsis_line)
        {
            ellipses.push(ellipsis_line);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

fn module_item_lines(kind: ItemKind, decl: Node, source: &str) -> FileLines {
    // Module-private items aren't part of the public API surface; their
    // private-vs-public split inside the declaration isn't load-bearing
    // for orientation, so the full member surface stays.
    let mut lines = decl_surface_lines(kind, decl, decl, source, false);
    // Module-private classes commonly carry the type's documentation
    // ("This class represents X. It follows the builder pattern...") in
    // a JSDoc block immediately above. NS authors anchor on the doc +
    // signature together (ts-pattern NS 2.5 = "MatchExpression class doc
    // + constructor"); without this, the walker emits the class surface
    // with the doc-block lines missing.
    if matches!(kind, ItemKind::Class) {
        let mut doc_lines = Vec::new();
        collect_jsdoc_above(decl, source, &mut doc_lines, false);
        if !doc_lines.is_empty() {
            lines.full = dedup_sorted(lines.full.into_iter().chain(doc_lines).collect());
        }
    }
    lines
}

fn decl_surface_lines(
    kind: ItemKind,
    anchor: Node,
    decl: Node,
    source: &str,
    public_surface: bool,
) -> FileLines {
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
                    if class_surface_member_kind(member.kind())
                        && !(public_surface && is_non_public_class_member(member, source))
                    {
                        let member_lines = member_header_lines(member).0;
                        full.extend(member_lines.full);
                        ellipses.extend(member_lines.ellipses);
                    }
                }
            }
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
        // interface / type alias / enum span their whole declaration;
        // default-export expressions with nothing structural fall here too.
        _ => push_rows(
            &mut full,
            export_start_row,
            node_end_row_trimmed(decl, source),
        ),
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Declaration surface truncated at its member body — the cheap
/// "header + ellipsis" render shared by split JS classes and
/// member-names-chunked declarations.
fn header_surface_lines(anchor: Node, body: Option<Node>, decl: Node, source: &str) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let export_start_row = anchor.start_position().row;
    let header_end = body
        .map(|body| body.start_position().row)
        .unwrap_or_else(|| node_end_row_trimmed(decl, source));
    push_rows(&mut full, export_start_row, header_end);
    if body.is_some_and(|body| body.end_position().row > body.start_position().row + 1) {
        ellipses.push(header_end + 2);
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(ellipses)
}

/// Body node holding a declaration's member list — class body,
/// interface body, or the object-literal type of a type alias.
fn member_surface_body<'a>(kind: ItemKind, decl: Node<'a>) -> Option<Node<'a>> {
    match kind {
        ItemKind::Interface => decl.child_by_field_name("body"),
        ItemKind::TypeAlias => decl
            .child_by_field_name("value")
            .filter(|value| value.kind() == "object_type"),
        ItemKind::Class | ItemKind::Default if is_class_node(decl) => {
            decl.child_by_field_name("body")
        }
        _ => None,
    }
}

/// `(first_line, end_line)` of every member surfaced by a declaration's
/// member catalog — interface/object-type signatures, or public class
/// members. Underscore-prefixed names are conventionally private and
/// excluded: NS catalogs list the consumer-facing surface only.
fn member_surface_spans(kind: ItemKind, decl: Node, source: &str) -> Vec<(usize, usize)> {
    let Some(body) = member_surface_body(kind, decl) else {
        return Vec::new();
    };
    let is_class = is_class_node(decl);
    let mut cursor = body.walk();
    body.children(&mut cursor)
        .filter(|member| {
            let surfaced = if is_class {
                class_surface_member_kind(member.kind())
                    && !is_non_public_class_member(*member, source)
            } else {
                matches!(
                    member.kind(),
                    "property_signature"
                        | "method_signature"
                        | "call_signature"
                        | "construct_signature"
                        | "index_signature"
                )
            };
            surfaced && !name_of(*member, source).is_some_and(|name| name.starts_with('_'))
        })
        .map(|member| {
            (
                member.start_position().row + 1,
                member.end_position().row + 1,
            )
        })
        .collect()
}

/// Member-name catalog chunks for one big declaration.
struct MemberNamesChunks {
    chunks: Vec<FileLines>,
    /// Class catalogs render truncated-to-name (the catalog is *which
    /// methods exist*); interface/object-type catalogs keep full lines
    /// (the field's type IS the content).
    truncate_to_name: bool,
}

/// Chunked member-first-line surfaces for a big declaration — the
/// "header then one line per member" catalog shape NS authors anchor
/// on. `None` below one chunk's worth of members; multi-line members
/// keep an ellipsis marker unless the next line is another member.
///
/// Classes only chunk above the per-member split range: a mid-size
/// class's whole-surface slab is affordable and NS rows want its full
/// signature lines, while an oversize class's slab never schedules —
/// the truncated name catalog is the only deliverable shape.
fn member_names_chunk_lines(kind: ItemKind, decl: Node, source: &str) -> Option<MemberNamesChunks> {
    let spans = member_surface_spans(kind, decl, source);
    let min_members = if is_class_node(decl) {
        JS_CLASS_MEMBER_SPLIT_MAX + 1
    } else {
        NAMES_SURFACE_CHUNK_SIZE
    };
    if spans.len() < min_members {
        return None;
    }
    let truncate_to_name = is_class_node(decl);
    let first_lines: HashSet<usize> = spans.iter().map(|&(first, _)| first).collect();
    let chunks = spans
        .chunks(NAMES_SURFACE_CHUNK_SIZE)
        .map(|chunk| {
            let mut full = Vec::new();
            let mut ellipses = Vec::new();
            for &(first, end) in chunk {
                full.push(first);
                if !truncate_to_name && end > first && !first_lines.contains(&(first + 1)) {
                    ellipses.push(first + 1);
                }
            }
            FileLines::new(full).with_ellipses(ellipses)
        })
        .collect();
    Some(MemberNamesChunks {
        chunks,
        truncate_to_name,
    })
}

/// Regex for truncated member-name renders — everything before the
/// parameter list, mirroring the NS authoring convention for method
/// catalogs.
const MEMBER_NAME_TRUNCATE_PATTERN: &str = "^[^(]+";

/// Lines content rendering each member line truncated to its name.
/// Lines without a leading non-`(` prefix fall back to `Full` so the
/// truncate regex always matches non-empty.
fn truncated_member_names_content(
    file: &Path,
    source: &str,
    lines: &FileLines,
) -> Option<BatchContent> {
    let src_lines: Vec<&str> = source.lines().collect();
    let mut spans = Vec::new();
    for &line in &lines.full {
        let Some(text) = src_lines.get(line - 1) else {
            continue;
        };
        if text.trim().is_empty() {
            continue;
        }
        let render = if text.starts_with('(') {
            Render::Full
        } else {
            Render::Truncated {
                pattern: MEMBER_NAME_TRUNCATE_PATTERN.to_string(),
            }
        };
        spans.push(Span {
            path: file.to_path_buf(),
            start: line,
            end: line,
            render,
        });
    }
    if spans.is_empty() {
        None
    } else {
        Some(BatchContent::Lines { spans })
    }
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

/// True when a class member is intentionally non-public — either an
/// ECMAScript `#privateName` (runtime-enforced, syntactically inaccessible
/// from outside) or a TypeScript `private`/`protected` modifier. Such
/// members aren't part of the class's public API, so they don't belong
/// in the orientation surface NS authors anchor on ("public method
/// signatures"). Filtering them shrinks dense classes like `PQueue`
/// (~50 members, most `#private`) from a budget-blocking signature
/// dump to a focused public-surface listing.
fn is_non_public_class_member(member: Node, source: &str) -> bool {
    if let Some(name) = name_of(member, source)
        && name.starts_with('#')
    {
        return true;
    }
    let mut cursor = member.walk();
    for child in member.children(&mut cursor) {
        if child.kind() == "accessibility_modifier" {
            let text = &source[child.start_byte()..child.end_byte()];
            if matches!(text, "private" | "protected") {
                return true;
            }
        }
    }
    false
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
    let src_lines: Vec<&str> = source.lines().collect();
    find_export_starts(Path::new("fixture.ts"), tree, source, &src_lines)
        .into_iter()
        .find(|e| e.start_line == start_line)
        .map(|e| e.body_parts)
        .unwrap_or_default()
}

/// Body slices that the materializer emits for a declaration, after applying
/// the same blank-line filter as `build_file_spans`. Large statement blocks
/// split by top-level statement so regions can schedule independently; small
/// blocks stay merged because per-slice atom overhead dominates their benefit.
/// For classes, each method body is considered independently.
fn body_parts(decl: Node, kind: ItemKind, source: &str, src_lines: &[&str]) -> Vec<BodyPart> {
    // `Class` always routes to method-body splitting; `Default` does when the
    // underlying expression is a class expression / declaration.
    if matches!(kind, ItemKind::Class) || (matches!(kind, ItemKind::Default) && is_class_node(decl))
    {
        return class_method_body_parts(decl, src_lines);
    }
    function_body_parts(decl_fn_body(decl, kind), source, src_lines)
}

/// Statement block of a top-level fn-bearing declaration. Returns `None` for
/// declarations with no reachable function body (pure-data consts, named
/// reexports, `export default <object>`, etc.) — callers fall through to
/// the empty `body_parts` result.
fn decl_fn_body(decl: Node, kind: ItemKind) -> Option<Node> {
    match kind {
        ItemKind::Function => decl.child_by_field_name("body"),
        ItemKind::Const => find_fn_init_body(decl),
        ItemKind::Default => match decl.kind() {
            "function_declaration"
            | "function_expression"
            | "arrow_function"
            | "generator_function" => decl.child_by_field_name("body"),
            "call_expression" | "parenthesized_expression" => {
                descend_for_fn_body(decl, FN_BODY_DESCEND_DEPTH)
            }
            _ => None,
        },
        _ => None,
    }
}

/// Body slices for a function/arrow body. Recognises the CommonJS factory
/// idiom — `function setup(...) { R.foo = ...; R.bar = ...; ...; return
/// R; }` — and substitutes two purpose-built `BodyPart`s for the standard
/// per-statement split: the receiver-table prefix, and an inner-function
/// locations surface (first line of every nested `function X(…)`
/// declaration in the factory body).
///
/// Without this hook all body lines go through `statement_block_parts`
/// and `merged_body_parts` collapses them into one `BodyPart` carrying
/// the entire body interior — a 200-300-line ExportBody for `debug`'s
/// `src/common.js` (~750 tokens) that loses every budget race. The
/// factory parts ship the small contract slices that NS authors anchor
/// on (the `module.exports = setup` shape where `setup` returns and
/// wires a shared receiver, and the catalog of inner helpers the
/// factory declares).
///
/// **Tradeoff**: the inner-function *bodies* still don't surface
/// through ExportBody on matched factories — only their location
/// lines do. In practice the unsplit whole-body slice also fails to
/// schedule (too expensive against typical budgets and later-segment
/// damping), so this is an honest trade: ship the small location
/// catalog or ship nothing. Inner-function bodies remain follow-up
/// reads off the location pointers.
fn function_body_parts(body: Option<Node>, source: &str, src_lines: &[&str]) -> Vec<BodyPart> {
    if let Some(b) = body
        && let Some(table) = factory_receiver_table_part(b, source, src_lines)
    {
        let mut parts = vec![table];
        // Inner-function locations surface — the catalog of nested
        // helpers the factory closes over. Distinct from the table
        // (the host contract); NS authors anchor on each separately.
        if let Some(locations) = factory_inner_function_locations_part(b, src_lines) {
            parts.push(locations);
        }
        // Per-inner-function body parts — each named helper's body
        // interior emitted as its own sibling BodyPart so small ones
        // (`disable`, `coerce`, `destroy`, `extend`) can schedule into
        // the auto-injection budget on their own merit. Without this
        // pass NS rows anchored on inner-helper bodies (debug's 3.x
        // tier) have no walker atoms at all.
        parts.extend(factory_inner_function_body_parts(b, src_lines));
        return parts;
    }
    statement_block_parts(body, src_lines, "statement_block")
}

/// Body slices for each nested `function_declaration` inside a factory
/// body. Reuses [`statement_block_parts`] so large inner-helper bodies
/// (createDebug's 88-line interior with its own nested `function
/// debug(...)` plus per-instance setup) split per top-level statement,
/// letting small regions schedule independently — same rule the
/// per-statement split uses for top-level function bodies. Caller
/// emits each part as its own sibling `ExportBody` batch.
fn factory_inner_function_body_parts(body: Node, src_lines: &[&str]) -> Vec<BodyPart> {
    let mut cursor = body.walk();
    let mut out = Vec::new();
    for child in body.named_children(&mut cursor) {
        if child.kind() != "function_declaration" {
            continue;
        }
        let inner_body = child.child_by_field_name("body");
        out.extend(statement_block_parts(
            inner_body,
            src_lines,
            "statement_block",
        ));
    }
    out
}

/// True when `function_body_parts` would split a top-level function
/// body into multiple purpose-built parts (the factory case). The
/// caller uses this to bypass `merged_body_parts` and emit each part
/// as its own `ExportBody` batch. Other function bodies route through
/// the merge so a single ExportBody covers the interior — the
/// existing materializer contract.
fn is_factory_body_match(decl: Node, kind: ItemKind, source: &str, src_lines: &[&str]) -> bool {
    decl_fn_body(decl, kind)
        .is_some_and(|body| factory_receiver_table_part(body, source, src_lines).is_some())
}

/// Minimum count for the factory-shape folds — receiver-property head
/// assignments and inner `function_declaration` helpers.  Below this
/// floor the pattern is just a couple incidental statements, not a
/// public-API wiring table.  Mirrors `JS_PROTOTYPE_METHOD_MIN`.
const FACTORY_PATTERN_MIN: usize = 3;

/// If `body` is a `statement_block` matching the factory-table idiom
/// — its tail contains `return R;` and its head is a run of
/// `R.<prop> = <expr>` assignments to the same identifier `R` —
/// return the head-run lines as a single `BodyPart`. Otherwise `None`.
///
/// The return-identifier gate keeps the fold tied to actual factories
/// (the receiver IS the returned value): regular handlers that happen
/// to start with property assignments on some object don't match
/// because their return shape doesn't name that object.
fn factory_receiver_table_part(body: Node, source: &str, src_lines: &[&str]) -> Option<BodyPart> {
    if body.kind() != "statement_block" {
        return None;
    }
    let mut cursor = body.walk();
    let named: Vec<Node> = body.named_children(&mut cursor).collect();
    let receiver = factory_return_identifier(&named, source)?;

    let mut table_rows: Vec<usize> = Vec::new();
    let mut count = 0usize;
    for child in &named {
        if !is_receiver_property_assignment(*child, source, receiver) {
            break;
        }
        // BodyPart lines are 1-based; `extend_nonblank_rows` does the
        // 0-based → 1-based conversion and applies the blank-line filter.
        let start_row = child.start_position().row;
        let end_row = child.end_position().row;
        extend_nonblank_rows(&mut table_rows, src_lines, start_row, end_row);
        count += 1;
    }
    if count < FACTORY_PATTERN_MIN {
        return None;
    }
    let lines = dedup_sorted(table_rows);
    (!lines.is_empty()).then_some(BodyPart { lines })
}

/// Inner-function locations surface for a factory body. Returns the
/// first-line list of every `function_declaration` directly nested in
/// the factory's `statement_block`, packed as one `BodyPart`. NS
/// authors anchor on this surface ("debug's `setup()` declares
/// selectColor, createDebug, debug, extend, enable, matchesTemplate,
/// disable, enabled, coerce, destroy") as the catalog of helpers the
/// factory closes over, distinct from the receiver-table contract.
///
/// Only `function_declaration` children count — `lexical_declaration`
/// arrow-init helpers don't carry the same "named inner helper"
/// semantics, and surfacing them would dilute the catalog. Below the
/// floor it's not a true catalog and the part is suppressed.
fn factory_inner_function_locations_part(body: Node, src_lines: &[&str]) -> Option<BodyPart> {
    let mut cursor = body.walk();
    let lines: Vec<usize> = body
        .named_children(&mut cursor)
        .filter(|child| child.kind() == "function_declaration")
        .map(|child| child.start_position().row + 1)
        .filter(|&line| {
            // Blank lines never have content; `extend_nonblank_rows` would
            // skip them too. Cheap guard against degenerate parses.
            line.saturating_sub(1) < src_lines.len() && !src_lines[line - 1].trim().is_empty()
        })
        .collect();
    if lines.len() < FACTORY_PATTERN_MIN {
        return None;
    }
    Some(BodyPart {
        lines: dedup_sorted(lines),
    })
}

/// Return the identifier text of the first top-level `return
/// <Identifier>;` statement in `named` (the function body's named
/// children). Real factory bodies have exactly one bottom return;
/// the gate also rejects `return obj.foo;` and `return;` (no
/// argument or non-identifier argument).
fn factory_return_identifier<'a>(named: &[Node<'a>], source: &'a str) -> Option<&'a str> {
    named
        .iter()
        .find(|child| child.kind() == "return_statement")
        .and_then(|ret| ret.named_child(0))
        .and_then(|arg| identifier_text(arg, source))
}

/// True when `stmt` is a top-level `<receiver>.<prop> = <expr>;`
/// assignment. Member-expression LHSs only — subscript and computed
/// keys don't count toward the factory-table fold.
fn is_receiver_property_assignment(stmt: Node, source: &str, receiver: &str) -> bool {
    if stmt.kind() != "expression_statement" {
        return false;
    }
    let Some(expr) = stmt.named_child(0) else {
        return false;
    };
    if expr.kind() != "assignment_expression" {
        return false;
    }
    let Some((left, _right)) = assignment_sides(expr) else {
        return false;
    };
    if left.kind() != "member_expression" {
        return false;
    }
    let Some((object, _property)) = member_object_property(left) else {
        return false;
    };
    identifier_text(object, source).is_some_and(|name| name == receiver)
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

// --- public-surface reachability ----------------------------------------

/// TS/JS extensions in precedence order. Declaration extensions come
/// last so TS source wins over generated `.d.ts`.
const TS_JS_EXTS: &[&str] = &["ts", "tsx", "js", "mjs", "cjs", "d.ts", "d.mts", "d.cts"];

/// Sorted recursive walk for entrypoint-named files. Skips heavy /
/// generated trees.
fn find_all_entrypoints(root: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(read_dir) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if file_type.is_dir() {
                let name = entry.file_name();
                if !super::fs::should_skip_dir(&name.to_string_lossy()) {
                    walk(&path, out);
                }
            } else if file_type.is_file() && is_entrypoint_file(&path) {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, &mut out);
    out.sort();
    out
}

/// Project's public surface — every TS/JS file transitively reachable
/// from an entrypoint via re-export chains. Canonicalized.
fn compute_public_surface(ctx: &WalkCtx) -> PublicSurface {
    let mut entrypoints = find_all_entrypoints(ctx.root());
    // The declared API contract is entrypoint-named-adjacent (`types`
    // field) but its `.d.ts` extension escapes the stem match above.
    entrypoints.extend(
        ctx.typescript_state()
            .declared_api_contract(ctx.root())
            .cloned(),
    );
    let mut surface = PublicSurface::default();
    let mut frontier: VecDeque<PathBuf> = VecDeque::new();
    let mut from_app_entrypoint: HashSet<PathBuf> = HashSet::new();
    for ep in entrypoints {
        let canonical = ep.canonicalize().unwrap_or_else(|_| ep.clone());
        if surface.files.insert(canonical) {
            frontier.push_back(ep);
        }
    }
    while let Some(file) = frontier.pop_front() {
        let Some((source, tree)) = parse_ts(ctx, &file) else {
            continue;
        };
        let dir = file.parent().unwrap_or_else(|| Path::new(""));
        let is_entrypoint = is_entrypoint_file(&file);
        let canonical_self = file.canonicalize().unwrap_or_else(|_| file.clone());
        let app_seed = is_entrypoint || from_app_entrypoint.contains(&canonical_self);
        let mut rel_paths = collect_reexported_source_paths(&tree, &source);
        // App-style entrypoint fallback: a script entrypoint that has no
        // named exports (no `export const/function/class`, no
        // `exports.X = …`, no `module.exports = { … }` namespace) still
        // defines the project's runtime surface via its `require` /
        // `import` chain. Treat its imported source paths as surface so
        // internal modules — which a library entrypoint would re-export,
        // but an app entrypoint just invokes — rank as public rather
        // than internal. A `module.exports = local-identifier` (CLI
        // default-style) doesn't count as named: the file exposes one
        // thing, and its imports are still part of the runtime chain.
        // Propagates one hop along the chain so the app's immediate
        // collaborators expand by the same rule.
        let used_app_fallback =
            rel_paths.is_empty() && app_seed && !file_has_named_exports(&tree, &source);
        if used_app_fallback {
            rel_paths = collect_import_source_paths(&tree, &source);
        }
        for rel in rel_paths {
            let Some(target) = resolve_ts_relative_path(dir, &rel) else {
                continue;
            };
            let canonical = target.canonicalize().unwrap_or_else(|_| target.clone());
            if used_app_fallback {
                from_app_entrypoint.insert(canonical.clone());
            } else {
                surface.reexport_targets.insert(canonical.clone());
            }
            if surface.files.insert(canonical) {
                frontier.push_back(target);
            }
        }
    }
    surface
}

/// Source paths exposed through the file's public surface — `export …
/// from`/re-exported `import` (ESM) plus `require(...)`-bound locals
/// that get re-assigned to `exports` (CommonJS).
fn collect_reexported_source_paths(tree: &Tree, source: &str) -> Vec<String> {
    let root = tree.root_node();
    let mut out: Vec<String> = Vec::new();
    let mut import_bindings: HashMap<String, String> = HashMap::new();

    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() == "import_statement" {
            let Some(src) = source_literal(stmt, source) else {
                continue;
            };
            let src = strip_quotes(&src).to_string();
            for name in import_clause_bindings(stmt, source) {
                import_bindings.insert(name, src.clone());
            }
        } else if is_require_declaration(stmt, source) {
            collect_require_bindings(stmt, source, &mut import_bindings);
        }
    }

    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() != "export_statement" {
            continue;
        }
        if has_from_source(stmt) {
            if let Some(src) = source_literal(stmt, source) {
                out.push(strip_quotes(&src).to_string());
            }
            continue;
        }
        for local_name in local_reexport_local_names(stmt, source) {
            if let Some(src) = import_bindings.get(&local_name) {
                out.push(src.clone());
            }
        }
    }
    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        let Some((left, right)) = commonjs_assignment_sides(stmt, source) else {
            continue;
        };
        match commonjs_export_target(left, source) {
            Some(CommonJsExportTarget::Namespace) if right.kind() == "object" => {
                for name in object_value_names(right, source) {
                    if let Some(src) = import_bindings.get(&name) {
                        out.push(src.clone());
                    }
                }
            }
            Some(_) => {
                if let Some(name) = identifier_text(right, source)
                    && let Some(src) = import_bindings.get(name)
                {
                    out.push(src.clone());
                }
            }
            None => {}
        }
    }

    out
}

/// True when the file emits any named export — `export …`, `exports.X`,
/// or `module.exports = { … }`. Library signal for the surface graph;
/// `module.exports = local-id` (default-style) doesn't count.
fn file_has_named_exports(tree: &Tree, source: &str) -> bool {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() == "export_statement" {
            return true;
        }
        let Some((left, right)) = commonjs_assignment_sides(stmt, source) else {
            continue;
        };
        match commonjs_export_target(left, source) {
            // `exports.X = …` always names a key.
            Some(CommonJsExportTarget::Property) => return true,
            // `module.exports = { … }` declares a namespace; a single
            // `module.exports = local-identifier` is default-style and
            // doesn't.
            Some(CommonJsExportTarget::Namespace) if right.kind() == "object" => return true,
            _ => {}
        }
    }
    false
}

/// Source paths from top-level `import` / `require` declarations —
/// surface-fallback for script-style entrypoints with no re-exports.
fn collect_import_source_paths(tree: &Tree, source: &str) -> Vec<String> {
    let root = tree.root_node();
    let mut out: Vec<String> = Vec::new();
    let mut bindings: HashMap<String, String> = HashMap::new();
    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() == "import_statement" {
            if let Some(src) = source_literal(stmt, source) {
                out.push(strip_quotes(&src).to_string());
            }
        } else if is_require_declaration(stmt, source) {
            bindings.clear();
            collect_require_bindings(stmt, source, &mut bindings);
            for src in bindings.values() {
                out.push(src.clone());
            }
        }
    }
    out
}

/// Walk a `const X = require('./foo')` or `const { X, Y } =
/// require('./foo')` statement and add each bound name → source path
/// to `bindings`.
fn collect_require_bindings(stmt: Node, source: &str, bindings: &mut HashMap<String, String>) {
    let mut cursor = stmt.walk();
    for child in stmt.children(&mut cursor) {
        if !matches!(child.kind(), "variable_declarator" | "lexical_binding") {
            continue;
        }
        let Some(value) = child.child_by_field_name("value") else {
            continue;
        };
        if !is_require_call(value, source) {
            continue;
        }
        let Some(args) = value.child_by_field_name("arguments") else {
            continue;
        };
        let mut ac = args.walk();
        let Some(src_node) = args.children(&mut ac).find(|c| c.kind() == "string") else {
            continue;
        };
        let src = strip_quotes(&source[src_node.start_byte()..src_node.end_byte()]).to_string();
        let Some(name_node) = child.child_by_field_name("name") else {
            continue;
        };
        match name_node.kind() {
            "identifier" => {
                let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
                bindings.insert(name, src);
            }
            "object_pattern" => {
                let mut pc = name_node.walk();
                for pat in name_node.named_children(&mut pc) {
                    let bound = match pat.kind() {
                        "shorthand_property_identifier_pattern" => {
                            Some(source[pat.start_byte()..pat.end_byte()].to_string())
                        }
                        "pair_pattern" => pat
                            .child_by_field_name("value")
                            .and_then(|v| identifier_text(v, source).map(str::to_string)),
                        _ => None,
                    };
                    if let Some(name) = bound {
                        bindings.insert(name, src.clone());
                    }
                }
            }
            _ => {}
        }
    }
}

/// Local names (before `as`) of each specifier in an `export_clause`
/// or `namespace_export`. Includes type-only specifiers.
fn local_reexport_local_names(stmt: Node, source: &str) -> Vec<String> {
    let mut out = Vec::new();
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
            if let Some(name_node) = first_identifier_child(spec) {
                let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
                out.push(name);
            }
        }
    }
    out
}

/// Names bound by an `import_statement`'s import clause:
/// - `import Foo from '...'` → `["Foo"]`
/// - `import { A, B as C } from '...'` → `["A", "C"]`
/// - `import * as N from '...'` → `["N"]`
/// - `import Foo, { A } from '...'` → `["Foo", "A"]`
/// - `import Foo, * as N from '...'` → `["Foo", "N"]`
fn import_clause_bindings(stmt: Node, source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cursor = stmt.walk();
    for child in stmt.children(&mut cursor) {
        match child.kind() {
            "import_clause" => collect_import_clause_names(child, source, &mut out),
            "identifier" => {
                // Bare `import Foo = require('...')` rare; just in case.
                let name = source[child.start_byte()..child.end_byte()].to_string();
                out.push(name);
            }
            _ => {}
        }
    }
    out
}

fn collect_import_clause_names(clause: Node, source: &str, out: &mut Vec<String>) {
    let mut cursor = clause.walk();
    for child in clause.children(&mut cursor) {
        match child.kind() {
            "identifier" => {
                let name = source[child.start_byte()..child.end_byte()].to_string();
                out.push(name);
            }
            "namespace_import" => {
                if let Some(id) = first_identifier_child(child) {
                    let name = source[id.start_byte()..id.end_byte()].to_string();
                    out.push(name);
                }
            }
            "named_imports" => {
                let mut nc = child.walk();
                for spec in child.children(&mut nc) {
                    if spec.kind() != "import_specifier" {
                        continue;
                    }
                    // `import { name as alias }`: the LOCAL binding is `alias` if
                    // present, else `name` — the field-name contract from the
                    // tree-sitter-typescript grammar.
                    let bound = spec
                        .child_by_field_name("alias")
                        .or_else(|| spec.child_by_field_name("name"));
                    if let Some(b) = bound {
                        out.push(source[b.start_byte()..b.end_byte()].to_string());
                    }
                }
            }
            _ => {}
        }
    }
}

fn strip_quotes(s: &str) -> &str {
    let s = s.trim();
    s.strip_prefix('\'')
        .or_else(|| s.strip_prefix('"'))
        .or_else(|| s.strip_prefix('`'))
        .and_then(|s| {
            s.strip_suffix('\'')
                .or_else(|| s.strip_suffix('"'))
                .or_else(|| s.strip_suffix('`'))
        })
        .unwrap_or(s)
}

/// Resolve a relative TS/JS specifier against `dir`. Tries the literal,
/// then a `.js`→`.ts`-style ext substitution, then `<rel>/index.<ext>`.
/// `None` for bare specifiers.
fn resolve_ts_relative_path(dir: &Path, rel: &str) -> Option<PathBuf> {
    if !(rel.starts_with("./") || rel.starts_with("../") || rel == "." || rel == "..") {
        return None;
    }
    let candidate = dir.join(rel);
    if candidate.is_file() {
        return Some(candidate);
    }
    let stripped_base = strip_ts_js_ext(rel).map(|s| dir.join(s));
    let bases: &[&PathBuf] = match &stripped_base {
        Some(s) => &[s, &candidate],
        None => &[&candidate],
    };
    for base in bases {
        for ext in TS_JS_EXTS {
            let with_ext = append_extension(base, ext);
            if with_ext.is_file() {
                return Some(with_ext);
            }
        }
    }
    for ext in TS_JS_EXTS {
        let idx = candidate.join(format!("index.{ext}"));
        if idx.is_file() {
            return Some(idx);
        }
    }
    None
}

fn strip_ts_js_ext(rel: &str) -> Option<&str> {
    for ext in TS_JS_EXTS {
        let with_dot = format!(".{ext}");
        if let Some(stripped) = rel.strip_suffix(&with_dot) {
            return Some(stripped);
        }
    }
    None
}

/// Append `.<ext>` to `base` — `Path::with_extension` would replace,
/// and the resolver needs append.
fn append_extension(base: &Path, ext: &str) -> PathBuf {
    let mut s = base.as_os_str().to_owned();
    s.push(".");
    s.push(ext);
    PathBuf::from(s)
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

    /// Lines for a single export's decl. For interface/type/class/enum, the
    /// whole item. For function, the signature plus a body-elision marker.
    /// For const/let, the assignment line(s) — truncated at the inner
    /// function body's `{` when the initializer is a fn-init (direct
    /// arrow/function or wrapped through `forwardRef(props => {...})` etc.),
    /// otherwise the whole declaration.
    fn collect_export_lines(tree: &Tree, source: &str, start_line: usize) -> FileLines {
        let src_lines: Vec<&str> = source.lines().collect();
        let exports = find_export_starts(Path::new("fixture.ts"), tree, source, &src_lines);
        let Some(item) = exports.iter().find(|e| e.start_line == start_line) else {
            return FileLines::new(Vec::new());
        };
        decl_surface_lines(item.kind, item.anchor, item.decl, source, true)
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
    fn walker_typescript_export_body_emit_rows() {
        const CASES: &[(&str, &[usize])] = &[
            // Body interior rows = 2..3 (line numbers 2, 3); brace rows 1, 4
            // skipped.
            (
                "export function foo() {\n  let x = 1;\n  return x;\n}\n",
                &[2, 3],
            ),
            (
                "export default function mitt() {\n  let all = new Map();\n  return { all };\n}\n",
                &[2, 3],
            ),
            // Method `bar` body interior: line 4. Method `baz` body
            // interior: lines 7, 8. Field initializer (`field`) skipped.
            (
                "\
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
",
                &[4, 7, 8],
            ),
            // `export const X = () => {...}` now emits the inner arrow body
            // through `Const`'s ExportBody arm. Signature truncates at the
            // body's `{` line.
            ("export const X = () => {\n  return 1;\n};\n", &[2]),
            // memo(component, areEqual) — callback is the FIRST argument,
            // so descent applies. The optional `areEqual` second argument
            // doesn't change the wrapper interpretation.
            (
                "\
export const Cmp = memo((props) => {
  return null;
}, areEqual);
",
                &[2],
            ),
            // `forwardRef(props => {...})` — body lives inside the call's
            // arguments. `find_fn_init_body` must descend through the call
            // expression to find it.
            (
                "\
export const Item = React.forwardRef<HTMLDivElement, ItemProps>(
  (props, ref) => {
    const x = 1;
    return null;
  },
);
",
                &[3, 4],
            ),
            // Method `bar` body interior: line 3.
            (
                "\
export default class C {
  bar() {
    return 1;
  }
}
",
                &[3],
            ),
            // Blank source lines inside the body shouldn't be counted —
            // collected rows stay non-blank (`build_file_spans` re-bridges
            // interior blanks at span build), so the emit-rows helper
            // filters them too. Lines 2 and 4 are blank; only 3 and 5 are
            // collected.
            (
                "\
export function foo() {

  let x = 1;

  return x;
}
",
                &[3, 5],
            ),
        ];

        for (src, expected) in CASES {
            assert_eq!(body_emit_rows_for(src), *expected, "{src}");
        }
    }

    #[test]
    fn walker_typescript_export_body_no_emit() {
        const NO_EMIT: &[&str] = &[
            // body.start_row == body.end_row → empty interior. No body
            // candidate fires.
            "export function foo() { return 1; }\n",
            "export function foo() {}\n",
            "export interface Foo { bar(): void }\n",
            "export type Foo = { bar: number }\n",
            "export enum Foo { A, B }\n",
            "export const X = () => 1;\n",
            // Object-literal initializers are pure data — no body to elide.
            "export const X = { a: 1, b: 2 };\n",
        ];

        for src in NO_EMIT {
            assert!(
                export_infos(&parse(src), src)[0].body_parts.is_empty(),
                "{src}"
            );
        }
    }

    #[test]
    fn walker_typescript_factory_receiver_table_part_matches() {
        // CommonJS factory idiom: `function setup(env) { R.x = ...; R.y =
        // ...; R.z = ...; return R; }`. Without the factory-table fold
        // body_parts splits per-statement and `body_part_value_factor`
        // damps each one by `1 / parts.len()`; the merged single-part
        // form keeps the table at full value so the ExportBody can
        // schedule into the auto-injection budget.
        let src = "\
function setup(env) {
  createDebug.debug = createDebug;
  createDebug.default = createDebug;
  createDebug.coerce = coerce;
  function helper() { return 1; }
  return createDebug;
}
module.exports = setup;
";
        let tree = parse(src);
        let src_lines: Vec<&str> = src.lines().collect();
        let exports = find_export_starts(Path::new("fixture.js"), &tree, src, &src_lines);
        let setup = exports.iter().find(|e| e.start_line == 1).unwrap();
        // One merged body part = the three R.x assignments (lines 2..4),
        // *not* the rest of the body (helper line 5).
        assert_eq!(setup.body_parts.len(), 1);
        assert_eq!(setup.body_parts[0].lines, vec![2, 3, 4]);
    }

    #[test]
    fn walker_typescript_factory_inner_function_locations_emitted_as_sibling_part() {
        // CommonJS factory with 3+ inner function declarations: the
        // receiver-table fold fires (3 R.x assignments at lines 2-4),
        // a locations surface lists the inner-function start lines
        // (5, 6, 7), and each inner helper's body interior emits as
        // its own sibling part. NS authors anchor on each surface
        // independently (debug NS 2.1, 2.2, 3.x).
        let src = "\
function setup(env) {
  createDebug.debug = createDebug;
  createDebug.default = createDebug;
  createDebug.coerce = coerce;
  function selectColor() { return 1; }
  function enable() { return 2; }
  function disable() { return 3; }
  return createDebug;
}
module.exports = setup;
";
        let tree = parse(src);
        let src_lines: Vec<&str> = src.lines().collect();
        let exports = find_export_starts(Path::new("fixture.js"), &tree, src, &src_lines);
        let setup = exports.iter().find(|e| e.start_line == 1).unwrap();
        assert!(setup.factory_sibling_body_parts);
        // Two pre-existing parts (table, locations) plus three
        // single-line bodies for each inner helper (lines 5/6/7 are
        // each one-statement bodies, so their interior is also their
        // sole line) — but the inner-body extractor skips bodies that
        // collapse to ≤ 1 line, so only nontrivial bodies emit. With
        // single-line bodies, no inner-body parts emit.
        assert_eq!(setup.body_parts.len(), 2);
        // First part: receiver-table prefix.
        assert_eq!(setup.body_parts[0].lines, vec![2, 3, 4]);
        // Second part: inner-function start lines.
        assert_eq!(setup.body_parts[1].lines, vec![5, 6, 7]);
    }

    #[test]
    fn walker_typescript_factory_inner_function_bodies_emitted_as_sibling_parts() {
        // Each inner helper with a multi-line body emits its body
        // interior as a separate sibling BodyPart. NS authors anchor
        // on inner-helper bodies (debug NS 3.x) as the meaningful
        // semantic unit inside the factory; this lets small bodies
        // schedule into the auto-injection budget independently.
        let src = "\
function setup(env) {
  R.a = 1;
  R.b = 2;
  R.c = 3;
  function selectColor(namespace) {
    let hash = 0;
    return hash;
  }
  function enable(namespaces) {
    R.namespaces = namespaces;
    return true;
  }
  function disable() {
    R.namespaces = '';
    return '';
  }
  return R;
}
module.exports = setup;
";
        let tree = parse(src);
        let src_lines: Vec<&str> = src.lines().collect();
        let exports = find_export_starts(Path::new("fixture.js"), &tree, src, &src_lines);
        let setup = exports.iter().find(|e| e.start_line == 1).unwrap();
        // 1 receiver table + 1 locations + 3 inner-fn bodies = 5 parts.
        assert_eq!(setup.body_parts.len(), 5);
        assert_eq!(setup.body_parts[0].lines, vec![2, 3, 4]); // table
        assert_eq!(setup.body_parts[1].lines, vec![5, 9, 13]); // locations
        // Inner-function bodies (lines inside braces, blank-line-filtered).
        assert_eq!(setup.body_parts[2].lines, vec![6, 7]); // selectColor body
        assert_eq!(setup.body_parts[3].lines, vec![10, 11]); // enable body
        assert_eq!(setup.body_parts[4].lines, vec![14, 15]); // disable body
    }

    #[test]
    fn walker_typescript_factory_receiver_table_no_match_below_floor() {
        // Two assignments is below `FACTORY_PATTERN_MIN` — fall
        // back to the standard split (regular body interior emit).
        let src = "\
function setup() {
  createDebug.a = 1;
  createDebug.b = 2;
  return createDebug;
}
module.exports = setup;
";
        let tree = parse(src);
        let src_lines: Vec<&str> = src.lines().collect();
        let exports = find_export_starts(Path::new("fixture.js"), &tree, src, &src_lines);
        let setup = exports.iter().find(|e| e.start_line == 1).unwrap();
        // Falls back to statement_block_parts — merged via
        // `merged_body_parts` in `make_export_info` to a single part
        // containing every interior line.
        let lines = &setup.body_parts[0].lines;
        assert!(lines.contains(&4), "fallback should include return line");
    }

    #[test]
    fn walker_typescript_factory_receiver_table_no_match_no_return() {
        // No `return <Identifier>;` — the fold requires the receiver IS
        // the returned value.
        let src = "\
function init() {
  obj.a = 1;
  obj.b = 2;
  obj.c = 3;
}
module.exports = init;
";
        let tree = parse(src);
        let src_lines: Vec<&str> = src.lines().collect();
        let exports = find_export_starts(Path::new("fixture.js"), &tree, src, &src_lines);
        let init = exports.iter().find(|e| e.start_line == 1).unwrap();
        let lines = &init.body_parts[0].lines;
        // Without a return-identifier the fold doesn't fire — fallback
        // covers every interior line.
        assert!(lines.contains(&2) && lines.contains(&3) && lines.contains(&4));
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
    fn walker_typescript_export_lines_real_wins_on_same_line() {
        // §4c collision filter: when a real export_statement shares a line
        // with a `const X = …` whose name is re-exported, the synthetic
        // candidate is dropped (real NamedReexport wins); also checks the
        // rendered export line is the single-line clause and that a
        // NamedReexport has no body parts.
        let src = "const X = () => { return 1; }; export { X };\n";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert_eq!(exports.len(), 1);
        assert!(matches!(exports[0].kind, ItemKind::NamedReexport));
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
    fn walker_typescript_type_machinery_factor_cases() {
        const CASES: &[(&str, &str, f64, bool, bool)] = &[
            // All exports are `type` aliases → file is type-machinery.
            (
                "\
export type A = number;
export type B = string;
export interface C { x: number }
",
                "foo.ts",
                TYPE_MACHINERY_FILE_FACTOR,
                true,
                false,
            ),
            // TS `enum` emits a runtime object — not type-only. A file with
            // any enum is not type-machinery even if every other export is
            // a type alias.
            (
                "\
export type A = number;
export enum E { X, Y }
",
                "foo.ts",
                1.0,
                false,
                false,
            ),
            // A single runtime export disqualifies the file.
            (
                "\
export type A = number;
export type B = string;
export const x = 1;
",
                "foo.ts",
                1.0,
                false,
                false,
            ),
            // `export type { Foo }` is a NamedReexport with the statement-level
            // `type` keyword — counts as type-only.
            (
                "\
type A = number;
export type { A };
",
                "foo.ts",
                TYPE_MACHINERY_FILE_FACTOR,
                true,
                false,
            ),
            // Bare `export { X }` (value re-export) is conservatively non-
            // type-only even when X happens to alias a type.
            // The synthetic Const at line 1 plus the NamedReexport at line 2.
            (
                "\
const X = () => 1;
export { X };
",
                "foo.ts",
                1.0,
                false,
                false,
            ),
            // Inline `export { type Foo }` (per-specifier modifier) is not
            // statement-level — conservatively classified as runtime so a
            // mixed `export { type Foo, valueY }` doesn't get penalized.
            (
                "\
type A = number;
const v = 1;
export { type A, v };
",
                "foo.ts",
                1.0,
                false,
                false,
            ),
            // A file with zero exports is not type-machinery (the multiplier
            // would have nothing to apply to anyway).
            ("import { foo } from './bar';\n", "foo.ts", 1.0, false, true),
            // A `.d.ts` file emits no runtime — even runtime-shaped exports
            // (`export class`, `export const`) are implicitly ambient. Path
            // alone qualifies the file regardless of export kinds.
            (
                "\
export class C { foo(): void; }
export const x: number;
",
                "typings/index.d.ts",
                TYPE_MACHINERY_FILE_FACTOR,
                false,
                false,
            ),
            // `export declare ...` in a regular `.ts` file is ambient — the
            // .ts file emits no runtime for the declaration.
            (
                "\
export declare function foo(): void;
export declare const x: number;
",
                "foo.ts",
                TYPE_MACHINERY_FILE_FACTOR,
                true,
                false,
            ),
        ];

        for (src, path, expected, expect_all_type_only, expect_empty) in CASES {
            let tree = parse(src);
            let exports = export_infos(&tree, src);
            if *expect_all_type_only {
                assert!(exports.iter().all(|e| e.is_type_only), "{src}");
            }
            if *expect_empty {
                assert!(exports.is_empty(), "{src}");
            }
            assert_eq!(
                type_machinery_factor(Path::new(path), &exports),
                *expected,
                "{src}"
            );
        }
    }
}
