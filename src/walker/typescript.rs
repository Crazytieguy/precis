//! TypeScript / TSX walker. Per-export decl batches plus file-scope
//! module-doc / imports / export-name surface.
//!
//! Per-file keys:
//! - `ModuleDocLede { file }`: top-of-file `/** */` JSDoc block (entrypoints
//!   only — files like `index.ts`, `main.ts`, `mod.ts`)
//! - `Imports { file }`: `import` declarations + bare `export … from`
//!   re-exports — plumbing, not items. Star re-exports are the
//!   exception; see `ExportNames`.
//! - `ExportNames { file }`: every top-level export's first line as a
//!   surface listing — catastrophic-omission hedge. Two qualifiers ride
//!   here rather than in the batch that would normally own them,
//!   because a roster is frequently the only place an export appears
//!   and it is `Export`'s predecessor: `export * from '…'` (which says
//!   the surface is larger than the roster lists) and a JSDoc
//!   `@deprecated` / `@internal` / `@private` marker (which says a
//!   listed export is not one to reach for).
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
use crate::fs_util::DirFilter;
use crate::value::{
    DEFAULT_CONCAVITY_EXPONENT, conserved_catalog_chunk_factors, mix_signals,
    reexport_import_chunk_factor, roster_mass_factor,
};

use super::import_chunks::{
    ImportGroup, REEXPORT_IMPORT_MAX_OTHER_LINES, REEXPORT_IMPORT_MAX_OTHER_STATEMENTS,
    groups_to_file_lines, node_line_count, push_import_group, should_chunk_import_groups,
};
use super::{
    BodyPart, FileLines, WalkCtx, body_part_value_factor, budget_chunk_ranges,
    build_per_file_content, dedup_sorted, disjoint_body_parts, extend_nonblank_rows, extend_span,
    file_depth_factor, file_lines_covered_by,
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
/// Hard affordability ceiling on a member-name catalog, in rendered
/// tokens. This is a safety valve, not a calibration knob: the
/// scheduler stops on the first top-ranked batch that does not fit and
/// only a seed listing degrades to a prefix, so an exact batch bigger
/// than the whole budget strands every token behind it. A roster's
/// cost is proportional to its member count, which is unbounded in
/// machine-generated `.d.ts` and schema surfaces, so the roster is the
/// one batch class that can exceed any budget.
///
/// 1500 sits ~50% above the largest catalog any fixture in the corpus
/// produces (1009 tokens, a hand-written `.d.ts` roster) and well
/// under the smallest budget precis is normally asked to fill, so it
/// never fires on hand-written code and always leaves a slice that a
/// real budget can buy.
const MEMBER_CATALOG_MAX_TOKENS: usize = 1500;
/// Rendered class surfaces above this cost are outside the early NS
/// purchase envelope and split at member boundaries.
const OVERSIZE_EXPORT_SPLIT_TOKENS: usize = 400;
/// Mirrors markdown's oversize-section chunk target: small enough to
/// compete in the NS authoring envelope without becoming a crumb train.
const OVERSIZE_EXPORT_CHUNK_TARGET_TOKENS: usize = 200;
const OVERSIZE_EXPORT_CHUNK_MIN_TAIL_TOKENS: usize = 100;
/// Direct continuation of an export whose head already won purchase.
const OVERSIZE_EXPORT_TAIL_FACTOR: f64 = 0.85;
/// Exported class bodies above this rendered-cost envelope are partitioned at
/// method-statement boundaries. A batch that cannot honor the envelope at a
/// natural boundary is suppressed rather than recreating a prefix blocker.
const CLASS_BODY_CHUNK_MAX_TOKENS: usize = 1_200;
const CLASS_BODY_CHUNK_TARGET_TOKENS: usize = 800;
const CLASS_BODY_CHUNK_MIN_TAIL_TOKENS: usize = 300;
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
    /// Per package dir: the manifest-published top-level directories that
    /// act as the package's source tree when it has no source-dir wrapper.
    /// Empty whenever a wrapper exists — the wrapper is then the answer.
    flat_source_dirs_lookup: RefCell<HashMap<PathBuf, Vec<String>>>,
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
    /// `types` / `typings` target, when it is a declaration file that
    /// exists in the repo, at whatever depth the manifest names it.
    /// Canonicalized. Generated targets (`dist/…`) name themselves out
    /// of this set by not being checked in.
    pub(in crate::walker) fn declared_api_contract(&self, root: &Path) -> Option<&PathBuf> {
        self.declared_api_contract
            .get_or_init(|| {
                let target = super::json::declared_types_target(root)?;
                if !is_declaration_file(&target) {
                    return None;
                }
                target.canonicalize().ok()
            })
            .as_ref()
    }

    /// The declared API contract when it also has to *seed* the public
    /// surface walk. A contract sitting next to the package's sources
    /// has an implementation sibling that is already an entrypoint, so
    /// seeding it there only adds its own type imports — which are
    /// plumbing, not published surface.
    pub(in crate::walker) fn declared_api_contract_entrypoint(
        &self,
        root: &Path,
    ) -> Option<&PathBuf> {
        let canonical_root = root.canonicalize().ok()?;
        self.declared_api_contract(root)
            .filter(|contract| contract.parent() == Some(canonical_root.as_path()))
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
    let js_like_files =
        files_with_any_extension(dir, &["ts", "tsx", "mts", "cts", "js", "mjs", "cjs"], ctx);
    if js_like_files.is_empty() {
        return Vec::new();
    }
    // Nested source dirs need index gating so child batches do not crowd the parent surface.
    let module_entrypoint = (!is_source_dir(dir) || ctx.depth_from_root(dir) > 1)
        .then(|| module_entrypoint_file(&js_like_files))
        .flatten();
    let module_entrypoint_gate = module_entrypoint
        .as_ref()
        .and_then(|file| emitted_import_gate_for_file(file, ctx));

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

        // File-surface batches this file actually emits, in the order a
        // reader meets them. Script statements gate on the first of them
        // — see `first_surface_gate`.
        let mut doc_lede_gate: Option<BatchKey> = None;
        let mut imports_gate: Option<BatchKey> = None;
        if ep
            && let Some(content) =
                build_per_file_content(file, ctx, parse_ts, collect_module_doc_lede)
        {
            let key = BatchKey::Typescript(TsKey::ModuleDocLede { file: file.clone() });
            out.push(Batch {
                key: key.clone(),
                predecessor: module_predecessor.clone(),
                content,
                value: module_doc_lede_value(file, ctx, js_factor),
            });
            doc_lede_gate = Some(key);
        }

        // Lines the imports / re-export-chunk batches would claim on
        // this file. Threaded into `collect_export_names_from` so the
        // names-surface ellipsis marker never lands on a line owned by
        // another peer batch — peers don't form an ancestor chain, so
        // an overlap would be a scheduler conflict. Recorded before the
        // star re-exports are subtracted below: an ellipsis has no
        // business on those rows either, since the roster renders them.
        let mut import_owned_lines: HashSet<usize> = HashSet::new();
        // Star re-exports handed to the export roster — see
        // [`roster_star_reexport_lines`].
        let mut roster_star_lines: Vec<usize> = Vec::new();
        // Re-export block past the import prologue — see
        // [`collect_reexport_tail`]. Emitted below, once the roster it
        // hangs off exists.
        let mut reexport_tail_lines = FileLines::default();
        if let Some((source, tree)) = parse_ts(ctx, file) {
            let api_spine = ep && is_api_spine_entrypoint(file, ctx);
            roster_star_lines = roster_star_reexport_lines(&tree, &source);
            if let Some(chunks) = collect_reexport_import_chunks(file, &tree, &source) {
                let chunk_count = chunks.len();
                for (chunk_index, (mut lines, is_reexport_wall)) in chunks.into_iter().enumerate() {
                    import_owned_lines.extend(lines.full.iter().copied());
                    lines.full.retain(|line| !roster_star_lines.contains(line));
                    let Some(content) = single_file_lines_content(file, &source, lines) else {
                        continue;
                    };
                    let base_value = if is_reexport_wall && api_spine {
                        reexport_wall_value(file, ctx, js_factor)
                    } else {
                        imports_value(file, ctx, js_factor)
                    };
                    let key = BatchKey::Typescript(TsKey::ImportChunk {
                        file: file.clone(),
                        chunk_index,
                    });
                    out.push(Batch {
                        key: key.clone(),
                        predecessor: module_predecessor.clone(),
                        content,
                        value: base_value * reexport_import_chunk_factor(chunk_index, chunk_count),
                    });
                    imports_gate.get_or_insert(key);
                }
            } else {
                let mut lines = collect_imports(&tree, &source);
                import_owned_lines.extend(lines.full.iter().copied());
                let is_reexport_wall = is_entrypoint_file(file)
                    && api_spine
                    && is_mostly_reexport(&lines, &collect_bare_reexport_lines(&tree, &source));
                lines.full.retain(|line| !roster_star_lines.contains(line));
                if let Some(content) = single_file_lines_content(file, &source, lines) {
                    let key = BatchKey::Typescript(TsKey::Imports { file: file.clone() });
                    out.push(Batch {
                        key: key.clone(),
                        predecessor: module_predecessor.clone(),
                        content,
                        value: if is_reexport_wall {
                            reexport_wall_value(file, ctx, js_factor)
                        } else {
                            imports_value(file, ctx, js_factor)
                        },
                    });
                    imports_gate = Some(key);
                }
            }
            let mut claimed = import_owned_lines.clone();
            claimed.extend(roster_star_lines.iter().copied());
            reexport_tail_lines = collect_reexport_tail(&tree, &source, &claimed);
            // The tail is a peer of the roster, so the roster's courtesy
            // ellipsis must not land on one of its rows.
            import_owned_lines.extend(reexport_tail_lines.full.iter().copied());
        }

        let Some((source, tree)) = parse_ts(ctx, file) else {
            continue;
        };
        let src_lines: Vec<&str> = source.lines().collect();
        let local_reexports = collect_local_reexports(&tree, &source);
        let commonjs_value_reexports = collect_commonjs_value_reexports(&tree, &source);
        let default_implementation_exports = collect_default_implementation_exports(&tree, &source);
        let mut exports = find_export_starts(
            file,
            &tree,
            &source,
            &src_lines,
            &local_reexports,
            &commonjs_value_reexports,
            &default_implementation_exports,
        );
        let top_level_decl_start_lines = top_level_decl_start_lines(&tree);
        // Module items are discovered before the exports are emitted: a
        // member-defining statement resolves its predecessor against them,
        // and the script-flow statements have to be known while the names
        // surface is choosing where to put its courtesy ellipses.
        let mut reexported_local_names = local_reexports.value.clone();
        reexported_local_names.extend(commonjs_value_reexports.iter().cloned());
        let module_items = find_module_items(
            &tree,
            &source,
            &src_lines,
            &exports.iter().map(|item| item.start_line).collect(),
            &reexported_local_names,
            commonjs_published_local_name(&tree, &source),
        );
        // README-cited JS files (canonical example scripts referenced from
        // the root README) emit private statements as module items even
        // though they aren't entrypoints — those statements ARE the
        // example's content the NS author anchored on.
        let emit_private_nonclass = (is_entrypoint_file(file) || ctx.is_readme_cited(file))
            && (is_tsx_file(file) || is_js_file(file));
        let emits_item = |item: &ModuleItemInfo| {
            emit_private_nonclass || matches!(item.kind, ItemKind::Class) || item.is_published
        };
        let emitted_items: Vec<&ModuleItemInfo> = module_items
            .iter()
            .filter(|item| emits_item(item))
            .collect();
        let emitted_item_start_lines: HashSet<usize> =
            emitted_items.iter().map(|item| item.start_line).collect();
        // A member-defining statement is only surface if the receiver it
        // installs onto actually renders. Chaining it to the roster instead
        // measured dead — the roster is the gate for the whole file's train,
        // so widening it prices out everything behind it — hence: resolve a
        // real predecessor batch or drop the statement entirely.
        let real_export_start_lines: HashSet<usize> = exports
            .iter()
            .filter(|item| item.member_definition_receiver_line.is_none())
            .map(|item| item.start_line)
            .collect();
        exports.retain(|item| {
            let Some(receiver_line) = item.member_definition_receiver_line else {
                return true;
            };
            emitted_item_start_lines.contains(&receiver_line)
                || real_export_start_lines.contains(&receiver_line)
        });
        // The declared API contract's roster shapes (names surface,
        // member-chunked decls and their catalogs) are exempt from the
        // machinery damp: the `.d.ts` the manifest's `types` field
        // names is the package's public API surface, and its big
        // member catalogs are what NS authors anchor on. Small
        // non-roster exports keep the damp — exempting them measured as
        // a cost-ascending flood of type aliases that displaces
        // NS-credited orientation without earning catalog credit.
        let is_api_contract = is_declared_api_contract(file, ctx);
        let per_export_factor = type_machinery_factor(file, &exports);
        // Member docs ship only from runtime modules: on a type-only
        // module the surface a doc would hang off is itself damped and
        // enters late or not at all, and a doc slice below an
        // unadmitted surface collects nothing.
        let runtime_module = is_runtime_module(file, &exports);
        let contract_roster_factor = |has_member_chunks: bool| {
            if is_api_contract && has_member_chunks {
                1.0
            } else {
                per_export_factor
            }
        };
        let export_start_lines: HashSet<_> = exports.iter().map(|item| item.start_line).collect();
        // Script flow: the run-directly half of a script. Scripts only —
        // in a compiled component module, module scope carries
        // registration trivia rather than the program's flow. Admission
        // is decided at emission time, where the file's first surface
        // batch is known; see `first_surface_gate`.
        let script_statements = if emit_private_nonclass && is_js_file(file) {
            let declaration_owned = declaration_owned_rows(&tree, &source);
            script_flow_statement_lines(&tree, &source)
                .into_iter()
                // A statement sharing a physical line with a declaration or
                // an export (`const s = {}; run(s);`) is already rendered by
                // that declaration's batch, and a second peer batch over the
                // row would be a scheduler conflict.
                .filter(|lines| {
                    lines
                        .full
                        .iter()
                        .all(|line| !declaration_owned.contains(line))
                })
                .collect()
        } else {
            Vec::new()
        };
        let script_statement_rows: HashSet<usize> = script_statements
            .iter()
            .flat_map(|lines| lines.full.iter().copied())
            .collect();
        let mut body_segment_index = 0usize;
        let mut names_gate: Option<BatchKey> = None;
        let mut names_owned_rows: HashSet<usize> = HashSet::new();
        if !exports.is_empty() {
            // One pass over the exports: the catalog/chunk computations
            // are token-cost probes, too expensive to redo for the
            // ExportNames flags and again per item.
            let export_split_plans: Vec<ExportSplitPlan> = exports
                .iter()
                .map(|item| {
                    let split_js_class = should_split_js_class_export(file, item);
                    let member_names_catalog = if split_js_class {
                        None
                    } else {
                        member_names_catalog_lines(item.kind, item.decl, &source)
                    };
                    let oversized_export_chunks =
                        if split_js_class || member_names_catalog.is_some() {
                            None
                        } else {
                            oversized_export_class_chunks(file, item, &source, ctx)
                        };
                    ExportSplitPlan {
                        split_js_class,
                        member_names_catalog,
                        oversized_export_chunks,
                    }
                })
                .collect();
            let has_split_js_class_export =
                export_split_plans.iter().any(|plan| plan.split_js_class);
            // `should_split_js_class_export` (js-only) and
            // `oversized_export_class_chunks` (ts/tsx-only) are
            // file-type disjoint, so the gated per-item chunks match
            // the ungated any-export probe this flag used to run.
            let has_oversized_ts_class_export = export_split_plans
                .iter()
                .any(|plan| plan.oversized_export_chunks.is_some());
            // One unified names surface per file — NS authors anchor on
            // the complete catalog as a single unit (chunking measured
            // against unified on the post-refreeze keys: unified wins).
            let mut names_lines = collect_export_names_from(
                &exports,
                &export_start_lines,
                &import_owned_lines,
                &top_level_decl_start_lines,
                &script_statement_rows,
            );
            // A roster claims "these exports exist". A JSDoc tag that
            // disavows one contradicts that claim, and it lives in
            // `ExportDoc` — a batch that competes with the roster
            // rather than riding with it, so a disavowed export
            // routinely renders listed clean among live siblings. The
            // roster is `Export`'s predecessor, so binding the marker
            // here makes it impossible to render the export without it.
            let disavowal_lines: HashSet<usize> = exports
                .iter()
                .filter(|item| item.predecessor_start_line.is_none())
                .flat_map(|item| {
                    disavowal_doc_lines(item.anchor, &source, &src_lines, is_entrypoint_file(file))
                })
                .collect();
            if !roster_star_lines.is_empty() || !disavowal_lines.is_empty() {
                names_lines.full = dedup_sorted(
                    names_lines
                        .full
                        .into_iter()
                        .chain(roster_star_lines.iter().copied())
                        .chain(disavowal_lines.iter().copied())
                        .collect(),
                );
                // A hoisted line can be the courtesy ellipsis row of the
                // export above it; Full wins, and the batch's own line
                // sets must stay disjoint.
                names_lines.ellipses.retain(|line| {
                    !disavowal_lines.contains(line) && !roster_star_lines.contains(line)
                });
            }
            if let Some(content) = single_file_lines_content(file, &source, names_lines.clone()) {
                let key = BatchKey::Typescript(TsKey::ExportNames { file: file.clone() });
                out.push(Batch {
                    key: key.clone(),
                    predecessor: module_predecessor.clone(),
                    content,
                    value: export_names_value(
                        file,
                        ctx,
                        js_factor,
                        has_split_js_class_export,
                        has_oversized_ts_class_export,
                    ) * contract_roster_factor(true),
                });
                names_gate = Some(key);
                names_owned_rows = names_lines
                    .full
                    .iter()
                    .chain(names_lines.ellipses.iter())
                    .copied()
                    .collect();
            }
            let dependent_export_start_lines: HashSet<_> = exports
                .iter()
                .filter_map(|item| item.predecessor_start_line)
                .collect();
            for (item, plan) in exports.iter().zip(export_split_plans) {
                let export_key = TsKey::Export {
                    file: file.clone(),
                    start_line: item.start_line,
                };
                let ExportSplitPlan {
                    split_js_class,
                    member_names_catalog,
                    oversized_export_chunks,
                } = plan;
                // A member-defining statement whose receiver is a private
                // declaration chains to that declaration's `ModuleItem`;
                // `retain` above already dropped the ones with neither an
                // export nor a module item to ride.
                let export_surface_predecessor = item
                    .predecessor_start_line
                    .map(|start_line| {
                        BatchKey::Typescript(
                            if item.member_definition_receiver_line == Some(start_line)
                                && !export_start_lines.contains(&start_line)
                            {
                                TsKey::ModuleItem {
                                    file: file.clone(),
                                    start_line,
                                }
                            } else {
                                TsKey::Export {
                                    file: file.clone(),
                                    start_line,
                                }
                            },
                        )
                    })
                    .or_else(|| names_gate.clone());
                let export_lines = if let Some(chunks) = &oversized_export_chunks {
                    chunks[0].clone()
                } else if split_js_class || member_names_catalog.is_some() {
                    // Catalogued declarations trade the whole-member
                    // surface for a cheap header; the members arrive via
                    // the gated `ExportMemberNames` catalog instead.
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
                // The disavowal marker rides with the roster; dropping
                // it here keeps the two batches' line sets disjoint.
                doc_lines.retain(|line| !disavowal_lines.contains(line));
                let export_has_descendants = !doc_lines.is_empty()
                    || (split_js_class && !item.class_members.is_empty())
                    || member_names_catalog.is_some()
                    || oversized_export_chunks.is_some()
                    || (!split_js_class && !item.body_parts.is_empty())
                    || dependent_export_start_lines.contains(&item.start_line);
                let export_surface_lines = export_lines.clone();
                if (!file_lines_covered_by(&export_lines, &names_lines) || export_has_descendants)
                    && let Some(content) = single_file_lines_content(file, &source, export_lines)
                {
                    out.push(Batch {
                        key: export_key.clone().into(),
                        predecessor: export_surface_predecessor,
                        content,
                        value: export_value(file, item.kind, ctx, js_factor)
                            * contract_roster_factor(member_names_catalog.is_some()),
                    });
                }
                let export_predecessor = BatchKey::Typescript(export_key);
                let mut oversized_tail_predecessor = export_predecessor.clone();
                if let Some(chunks) = &oversized_export_chunks {
                    let mut predecessor = export_predecessor.clone();
                    for (chunk_index, lines) in chunks.iter().skip(1).enumerate() {
                        let Some(content) = single_file_lines_content(file, &source, lines.clone())
                        else {
                            continue;
                        };
                        let key = TsKey::ExportTail {
                            file: file.clone(),
                            start_line: item.start_line,
                            chunk_index,
                        };
                        out.push(Batch {
                            key: key.clone().into(),
                            predecessor: Some(predecessor),
                            content,
                            value: export_value(file, item.kind, ctx, js_factor)
                                * per_export_factor
                                * OVERSIZE_EXPORT_TAIL_FACTOR,
                        });
                        predecessor = BatchKey::Typescript(key);
                    }
                    oversized_tail_predecessor = predecessor;
                }
                // Body parts of a catalogued declaration hang behind the
                // member catalog: the catalog is the better buy at every
                // budget, and a sibling body batch could collide with
                // the catalog's ellipsis markers (non-ancestor overlap).
                let mut body_parts_predecessor = oversized_tail_predecessor;
                // The batch that renders the members' signature rows —
                // the roster itself, or its last slice when the roster
                // breached the affordability ceiling.
                let mut member_row_owner: Option<BatchKey> = None;
                if let Some(member_catalog) = &member_names_catalog {
                    let content = member_names_catalog_content(
                        file,
                        &source,
                        &member_catalog.lines,
                        member_catalog.truncate_to_name,
                    );
                    if let Some(content) = content {
                        let member_count = member_catalog.lines.full.len();
                        let roster_value = export_member_names_value(
                            file,
                            item.kind,
                            ctx,
                            js_factor,
                            member_count,
                        ) * contract_roster_factor(true);
                        let whole_cost = ctx.marginal_tokens(&content);
                        if whole_cost <= MEMBER_CATALOG_MAX_TOKENS {
                            let key = TsKey::ExportMemberNames {
                                file: file.clone(),
                                start_line: item.start_line,
                            };
                            out.push(Batch {
                                key: key.clone().into(),
                                predecessor: Some(export_predecessor.clone()),
                                content,
                                value: roster_value,
                            });
                            member_row_owner = Some(BatchKey::Typescript(key));
                        } else {
                            let slices =
                                member_names_catalog_cap_chunks(file, &source, member_catalog, ctx);
                            let sliced: Vec<(BatchContent, usize)> = slices
                                .iter()
                                .filter_map(|lines| {
                                    member_names_catalog_content(
                                        file,
                                        &source,
                                        lines,
                                        member_catalog.truncate_to_name,
                                    )
                                    .map(|content| {
                                        let cost = ctx.marginal_tokens(&content);
                                        (content, cost)
                                    })
                                })
                                .collect();
                            // Value is split by each slice's share of the
                            // whole roster's cost. Ranking is
                            // `value / cost^k` with `k < 1`, so a slice
                            // scores `(c_i / C)^(1-k)` of the unsplit
                            // roster's ratio — strictly less than it for
                            // every proper slice. Slicing therefore never
                            // buys the reader an earlier, cheaper way in
                            // than the whole roster would have been; it
                            // only makes the roster affordable at all.
                            let total: usize = sliced.iter().map(|(_, cost)| *cost).sum();
                            let emitted_any = !sliced.is_empty();
                            let mut predecessor = export_predecessor.clone();
                            for (chunk_index, (content, cost)) in sliced.into_iter().enumerate() {
                                let key = TsKey::ExportMemberNamesChunk {
                                    file: file.clone(),
                                    start_line: item.start_line,
                                    chunk_index,
                                };
                                let share = if total == 0 {
                                    0.0
                                } else {
                                    cost as f64 / total as f64
                                };
                                out.push(Batch {
                                    key: key.clone().into(),
                                    predecessor: Some(predecessor),
                                    content,
                                    value: roster_value * share,
                                });
                                predecessor = BatchKey::Typescript(key);
                            }
                            member_row_owner = emitted_any.then_some(predecessor);
                        }
                    }
                }
                if let Some(owner) = &member_row_owner {
                    body_parts_predecessor = owner.clone();
                }
                if let Some((roster, element_count)) = data_literal_roster(item.decl, item.kind)
                    .filter(|(roster, _)| !file_lines_covered_by(roster, &export_surface_lines))
                    && let Some(content) = single_file_lines_content(file, &source, roster)
                {
                    let key = TsKey::LiteralRoster {
                        file: file.clone(),
                        start_line: item.start_line,
                    };
                    out.push(Batch {
                        key: key.clone().into(),
                        predecessor: Some(body_parts_predecessor.clone()),
                        content,
                        value: literal_roster_value(file, item.kind, ctx, js_factor, element_count)
                            * per_export_factor,
                    });
                    body_parts_predecessor = BatchKey::Typescript(key);
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
                if runtime_module {
                    // Each documented member's doc rides the batch that
                    // renders that member's signature row — the same law
                    // as a module statement riding the file's first
                    // admitted surface, one level down: a member's doc is
                    // the cheap content of the member's surface. A member
                    // whose signature no batch renders gets no doc batch;
                    // a root-level crumb below an unadmitted surface is
                    // credit the reader can never collect.
                    let member_doc_owner = |sig_line: usize| -> Option<BatchKey> {
                        // A chunked surface is one reading of the
                        // declaration, split only for affordability, so
                        // its docs ride the *last* chunk. Riding the
                        // member's own chunk instead lets an early
                        // chunk's cheap docs outrank the later chunk
                        // carrying the other members' signatures —
                        // measured: that inversion strands six of one
                        // fixture's eight member-doc rows.
                        if let Some(chunks) = &oversized_export_chunks {
                            if !chunks.iter().any(|chunk| chunk.full.contains(&sig_line)) {
                                return None;
                            }
                            return Some(BatchKey::Typescript(match chunks.len().checked_sub(2) {
                                Some(chunk_index) => TsKey::ExportTail {
                                    file: file.clone(),
                                    start_line: item.start_line,
                                    chunk_index,
                                },
                                // The head chunk is the `Export` batch.
                                None => TsKey::Export {
                                    file: file.clone(),
                                    start_line: item.start_line,
                                },
                            }));
                        }
                        if split_js_class {
                            return item
                                .class_members
                                .iter()
                                .any(|member| member.start_line == sig_line)
                                .then(|| {
                                    BatchKey::Typescript(TsKey::ExportMember {
                                        file: file.clone(),
                                        start_line: item.start_line,
                                        member_start_line: sig_line,
                                    })
                                });
                        }
                        // A name-only catalog does not deliver the
                        // member's signature, only that the member
                        // exists — so a doc block under it is not the
                        // cheap completion of a surface already paid
                        // for, it is the whole member arriving as
                        // prose, against a declaration whose own
                        // catalog chose to spend nothing on detail.
                        if member_names_catalog
                            .as_ref()
                            .is_some_and(|catalog| catalog.truncate_to_name)
                        {
                            return None;
                        }
                        if member_names_catalog.is_some() {
                            // The roster, or — when it breached the
                            // affordability ceiling — its last slice.
                            // The slices are a chain, so the last one
                            // being scheduled means every member's row
                            // is rendered; this is the same last-chunk
                            // gate the split export surface uses above.
                            // `None` when the roster rendered nothing:
                            // a member whose signature no batch renders
                            // gets no doc batch.
                            return member_row_owner.clone();
                        }
                        // The undivided export surface renders every
                        // member header itself.
                        Some(export_predecessor.clone())
                    };
                    for member in documented_member_nodes(item, &source) {
                        let sig_line = member.start_position().row + 1;
                        let Some(lines) = member_doc_lines(member, &source, &src_lines) else {
                            continue;
                        };
                        // A declaration whose surface spans its whole
                        // body already delivers its members' docs; a
                        // second batch over the same rows adds nothing.
                        if file_lines_covered_by(&lines, &export_surface_lines) {
                            continue;
                        }
                        let Some(predecessor) = member_doc_owner(sig_line) else {
                            continue;
                        };
                        let Some(content) = single_file_lines_content(file, &source, lines) else {
                            continue;
                        };
                        out.push(Batch {
                            key: TsKey::ExportMemberDoc {
                                file: file.clone(),
                                start_line: item.start_line,
                                member_start_line: sig_line,
                            }
                            .into(),
                            predecessor: Some(predecessor),
                            content,
                            value: export_doc_value(file, item.kind, ctx, js_factor),
                        });
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
                    let body_parts = disjoint_body_parts(item.body_parts.clone());
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
                        body_parts,
                        &body_parts_predecessor,
                    );
                }
            }
        }
        // Outside the `exports` block on purpose: a file can publish its
        // whole surface through a trailing block and declare nothing
        // locally, and that surface has to render. The tail chains
        // behind the roster when there is one so the two halves of the
        // surface arrive in order, and otherwise roots at the module
        // gate like every other file-surface batch here.
        if !reexport_tail_lines.full.is_empty()
            && let Some(content) =
                single_file_lines_content(file, &source, reexport_tail_lines.clone())
        {
            out.push(Batch {
                key: TsKey::ReexportTail { file: file.clone() }.into(),
                predecessor: names_gate.clone().or_else(|| module_predecessor.clone()),
                content,
                value: reexport_wall_value(file, ctx, js_factor),
            });
        }
        // A wide private surface ships as one first-line catalog with
        // the per-item batches gated (and line-covered) behind it —
        // NS authors anchor on the roster as a unit, and without it
        // the items schedule as a ~15-token-apiece train that eats the
        // early budget ahead of orientation content (cmdk index.tsx:
        // ~40 items, ~1.5K tokens before the workspace manifests).
        let mut module_items_gate: Option<BatchKey> = None;
        // When a file has too few declarations for a catalog, its first
        // declaration batch is the admitted declaration content instead.
        let mut first_module_item_gate: Option<BatchKey> = None;
        if emitted_items.len() >= MODULE_ITEM_CATALOG_MIN {
            let names_lines =
                FileLines::new(emitted_items.iter().map(|item| item.start_line).collect());
            if let Some(content) = single_file_lines_content(file, &source, names_lines) {
                let key = BatchKey::Typescript(TsKey::ModuleItemNames { file: file.clone() });
                out.push(Batch {
                    key: key.clone(),
                    predecessor: module_predecessor.clone(),
                    content,
                    value: module_item_names_value(file, ctx, js_factor, emitted_items.len())
                        * per_export_factor,
                });
                module_items_gate = Some(key);
            }
        }
        // Below the catalog minimum there is no catalog to ride, so each
        // item pays the same admission price a statement does: the file's
        // first admitted surface. Without it a 4–5-declaration file in a
        // directory with no entrypoint puts its declarations at the root
        // of the schedule, ahead of every gated orientation batch. Every
        // rung already chains to `module_predecessor`, so this only ever
        // tightens the ordering; the fallback covers a file whose
        // declarations *are* its first surface.
        for item in module_items.iter() {
            if !emits_item(item) {
                continue;
            }
            let item_key = TsKey::ModuleItem {
                file: file.clone(),
                start_line: item.start_line,
            };
            if let Some(content) = single_file_lines_content(file, &source, item.lines.clone()) {
                out.push(Batch {
                    key: item_key.clone().into(),
                    predecessor: first_surface_gate(
                        module_items_gate.as_ref(),
                        doc_lede_gate.as_ref(),
                        names_gate.as_ref(),
                        imports_gate.as_ref(),
                    )
                    .or_else(|| module_predecessor.clone()),
                    content,
                    value: module_item_value(file, item.kind, ctx, js_factor) * per_export_factor,
                });
                first_module_item_gate
                    .get_or_insert_with(|| BatchKey::Typescript(item_key.clone()));
            }
            let mut item_predecessor = BatchKey::Typescript(item_key.clone());
            if let Some((roster, element_count)) = &item.literal_roster
                && let Some(content) = single_file_lines_content(file, &source, roster.clone())
            {
                let key = TsKey::LiteralRoster {
                    file: file.clone(),
                    start_line: item.start_line,
                };
                out.push(Batch {
                    key: key.clone().into(),
                    predecessor: Some(item_predecessor.clone()),
                    content,
                    value: literal_roster_value(file, item.kind, ctx, js_factor, *element_count)
                        * per_export_factor,
                });
                item_predecessor = BatchKey::Typescript(key);
            }
            if !item.body_parts.is_empty() {
                let parts = disjoint_body_parts(item.body_parts.clone());
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
        // The declarations are covered above; what is left is the statements
        // that actually execute, which belong to no declaration and would
        // otherwise have no batch at all. Membership was decided with the
        // rest of the file's line ownership — see `script_statements`.
        {
            for lines in script_statements {
                let Some(start_line) = lines.full.first().copied() else {
                    continue;
                };
                // A member-defining call is an expression statement too, and
                // it already renders as an export. Two peer batches over the
                // same rows would be a scheduler conflict.
                if export_start_lines.contains(&start_line) {
                    continue;
                }
                // The names surface is the only gate that can own one of
                // these rows (its courtesy ellipsis). Rows are kept disjoint
                // above, so this is the belt to that braces: when it ever
                // does overlap, chain under the owner rather than beside it.
                let overlaps_names_surface = lines
                    .full
                    .iter()
                    .any(|line| names_owned_rows.contains(line));
                let Some(gate) = (if overlaps_names_surface {
                    names_gate.clone()
                } else {
                    first_surface_gate(
                        module_items_gate
                            .as_ref()
                            .or(first_module_item_gate.as_ref()),
                        doc_lede_gate.as_ref(),
                        names_gate.as_ref(),
                        imports_gate.as_ref(),
                    )
                }) else {
                    continue;
                };
                let Some(content) = single_file_lines_content(file, &source, lines) else {
                    continue;
                };
                out.push(Batch {
                    key: TsKey::ModuleStatements {
                        file: file.clone(),
                        start_line,
                    }
                    .into(),
                    predecessor: Some(gate),
                    content,
                    value: module_item_value(file, ItemKind::Const, ctx, js_factor)
                        * per_export_factor,
                });
            }
        }
    }

    out
}

/// The file's first admitted surface — the batch a reader necessarily
/// meets before its module-scope content, and therefore the price of
/// admission for it. A file that publishes no surface at all emits no
/// statement batches rather than root-level crumbs that jump the queue.
fn first_surface_gate(
    declaration_gate: Option<&BatchKey>,
    doc_lede_gate: Option<&BatchKey>,
    names_gate: Option<&BatchKey>,
    imports_gate: Option<&BatchKey>,
) -> Option<BatchKey> {
    declaration_gate
        .or(doc_lede_gate)
        .or(names_gate)
        .or(imports_gate)
        .cloned()
}

/// Every row spanned by a top-level declaration or export statement —
/// the rows some declaration-shaped batch may claim. A script-flow
/// statement sharing one of them (`const s = {}; run(s);` on a single
/// line) is already rendered there, and a peer batch over the same row
/// is a scheduler conflict.
fn declaration_owned_rows(tree: &Tree, source: &str) -> HashSet<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut rows = HashSet::new();
    for child in root.children(&mut cursor) {
        if decl_kind(child).is_none() && child.kind() != "export_statement" {
            continue;
        }
        for row in child.start_position().row..=node_end_row_trimmed(child, source) {
            rows.insert(row + 1);
        }
    }
    rows
}

/// Statement kinds that make up a script's flow — everything a program
/// does at module scope that isn't a declaration, an import, or an
/// export.
fn is_script_flow_statement(node: Node) -> bool {
    matches!(
        node.kind(),
        "expression_statement"
            | "if_statement"
            | "for_statement"
            | "for_in_statement"
            | "while_statement"
            | "do_statement"
            | "switch_statement"
            | "try_statement"
            | "throw_statement"
            | "labeled_statement"
    )
}

/// Module-scope script-flow statements, one entry apiece. Per statement
/// rather than per contiguous run: a run is the whole tail of a bin
/// script and prices itself out of the early budget as one slab, while
/// each statement (a dispatch guard, a bootstrap chain) is already an
/// NS-sized unit. String directives (`'use strict'`) are import prologue
/// and stay with the imports batch.
fn script_flow_statement_lines(tree: &Tree, source: &str) -> Vec<FileLines> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    root.children(&mut cursor)
        .filter(|child| is_script_flow_statement(*child) && !is_string_directive(*child))
        .map(|child| {
            let mut lines = Vec::new();
            extend_span(&mut lines, child, source);
            FileLines::new(dedup_sorted(lines))
        })
        .collect()
}

/// Key of the import-shaped batch the module entrypoint will actually
/// emit, or `None` when it emits none (no import / re-export lines, or
/// unparseable). Sibling batches must never be gated on a key that is
/// never absorbed — such a predecessor leaves every dependent batch
/// permanently ineligible, silently starving the whole directory.
fn emitted_import_gate_for_file(file: &Path, ctx: &WalkCtx) -> Option<BatchKey> {
    let (source, tree) = parse_ts(ctx, file)?;
    if collect_reexport_import_chunks(file, &tree, &source).is_some() {
        return Some(BatchKey::Typescript(TsKey::ImportChunk {
            file: file.to_path_buf(),
            chunk_index: 0,
        }));
    }
    let lines = collect_imports(&tree, &source);
    single_file_lines_content(file, &source, lines)?;
    Some(BatchKey::Typescript(TsKey::Imports {
        file: file.to_path_buf(),
    }))
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
    let parts = disjoint_body_parts(parts);
    // Class method bodies skip late-body decay. Oversized class aggregates
    // become one value-conserved train: splitting their shape must not
    // multiply the declaration's total value.
    let is_class_peer = matches!(item.kind, ItemKind::Class)
        || (matches!(item.kind, ItemKind::Default) && is_class_node(item.decl));
    let valued_parts = if is_class_peer {
        bounded_class_body_parts(emit.file, emit.source, parts, emit.ctx)
    } else {
        // Factory body parts are sibling anchors; skip the per-partition
        // value damping.
        let factor = if item.factory_sibling_body_parts {
            1.0
        } else {
            body_part_value_factor(parts.len())
        };
        parts
            .into_iter()
            .map(|part| ValuedClassBodyPart {
                part,
                value_factor: factor,
            })
            .collect()
    };
    let mut class_predecessor = predecessor.clone();
    for valued_part in valued_parts {
        let part = valued_part.part;
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
        let key = TsKey::ExportBody {
            file: emit.file.to_path_buf(),
            start_line: item.start_line,
            body_start_line,
        };
        out.push(Batch {
            key: key.clone().into(),
            predecessor: Some(if is_class_peer {
                class_predecessor.clone()
            } else {
                predecessor.clone()
            }),
            content,
            value: export_body_value(emit.file, item.kind, emit.ctx, emit.js_factor)
                * emit.per_export_factor
                * valued_part.value_factor
                * segment_factor,
        });
        if is_class_peer {
            class_predecessor = BatchKey::Typescript(key);
        } else {
            *emit.body_segment_index += 1;
        }
    }
}

struct ValuedClassBodyPart {
    part: BodyPart,
    value_factor: f64,
}

/// Bound exported-class body batches at a fixed rendered-cost envelope.
/// Source-ordered method-statement parts are packed with
/// [`budget_chunk_ranges`], then one declaration-level value is conserved
/// across the resulting predecessor chain. If one indivisible statement is
/// itself over the envelope, suppress the class body: emitting it would
/// recreate the exact prefix blocker this shaping rule exists to prevent.
fn bounded_class_body_parts(
    file: &Path,
    source: &str,
    parts: Vec<BodyPart>,
    ctx: &WalkCtx,
) -> Vec<ValuedClassBodyPart> {
    if parts.is_empty() {
        return Vec::new();
    }
    let lines_for = |range: std::ops::Range<usize>| BodyPart {
        lines: dedup_sorted(
            parts[range]
                .iter()
                .flat_map(|part| part.lines.iter().copied())
                .collect(),
        ),
    };
    let cost = |part: &BodyPart| {
        single_file_lines_content(file, source, FileLines::new(part.lines.clone()))
            .map(|content| ctx.marginal_tokens(&content))
            .unwrap_or(0)
    };
    let whole = lines_for(0..parts.len());
    if cost(&whole) <= CLASS_BODY_CHUNK_MAX_TOKENS {
        return vec![ValuedClassBodyPart {
            part: whole,
            value_factor: 1.0,
        }];
    }
    let ranges = budget_chunk_ranges(
        parts.len(),
        |range| cost(&lines_for(range)),
        CLASS_BODY_CHUNK_TARGET_TOKENS,
        CLASS_BODY_CHUNK_MIN_TAIL_TOKENS,
        |_| true,
        |range| cost(&lines_for(range)) <= CLASS_BODY_CHUNK_MAX_TOKENS,
    );
    let chunks: Vec<BodyPart> = ranges.into_iter().map(lines_for).collect();
    let costs: Vec<usize> = chunks.iter().map(cost).collect();
    if costs
        .iter()
        .any(|&tokens| tokens > CLASS_BODY_CHUNK_MAX_TOKENS)
    {
        return Vec::new();
    }
    let factors = conserved_catalog_chunk_factors(&costs, DEFAULT_CONCAVITY_EXPONENT);
    chunks
        .into_iter()
        .zip(factors)
        .map(|(part, value_factor)| ValuedClassBodyPart { part, value_factor })
        .collect()
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

/// Per-export split decisions, computed once per file pass: the
/// catalog / chunk probes are token-cost renders shared by the
/// `ExportNames` flags and the per-item emission.
struct ExportSplitPlan {
    split_js_class: bool,
    member_names_catalog: Option<MemberNamesCatalog>,
    oversized_export_chunks: Option<Vec<FileLines>>,
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
    /// True when `body_parts` are sibling anchors from a factory match
    /// — the emitter skips per-part value damping.
    factory_sibling_body_parts: bool,
    /// Export surface line that this synthesized local implementation
    /// refines. Used for thin `export default localName` entrypoints.
    predecessor_start_line: Option<usize>,
    /// Set only for a member-defining statement: the line publishing the
    /// receiver it installs onto. The statement is surface only while
    /// that line renders, and it never joins the names surface — see the
    /// `retain` in `expand_in_dir`.
    member_definition_receiver_line: Option<usize>,
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
    /// Key roster of the data literal this declaration binds, with its
    /// element count — see [`data_literal_roster`].
    literal_roster: Option<(FileLines, usize)>,
    /// This declaration is what the file's `module.exports` publishes —
    /// see [`commonjs_published_local_name`]. It renders wherever the
    /// file's own surface does, like a module-level `class`.
    is_published: bool,
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
    reexports: &LocalReexports,
    commonjs_reexports: &HashSet<String>,
    default_identifier_reexports: &HashMap<String, usize>,
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
        ));
    }

    let mut needs_sort = false;
    if !reexports.is_empty()
        || !commonjs_reexports.is_empty()
        || !default_identifier_reexports.is_empty()
    {
        let mut cursor = root.walk();
        for child in root.children(&mut cursor) {
            let start_line = child.start_position().row + 1;
            if emitted_lines.contains(&start_line) {
                continue;
            }

            // The rules key off the declaration, but the rendered
            // surface is the wrapper — it carries the `declare`.
            let ambient_inner = ambient_inner_decl(child);
            let decl = ambient_inner.unwrap_or(child);

            let default_predecessor = local_decl_name(decl, source)
                .and_then(|name| default_identifier_reexports.get(name).copied());
            let kind = if default_predecessor.is_some() {
                decl_kind(decl)
            } else if let Some(kind) = synthetic_local_export_kind(decl, source, &reexports.value) {
                Some(kind)
            } else if let Some(kind) =
                synthetic_local_type_export_kind(decl, source, &reexports.type_only)
            {
                Some(kind)
            } else {
                synthetic_commonjs_export_kind(decl, source, commonjs_reexports)
            };

            let Some(kind) = kind else { continue };
            // An ambient declaration carries no runtime value, same as
            // the `export declare …` arm of `is_export_type_only`.
            let is_type_only = ambient_inner.is_some()
                || matches!(kind, ItemKind::Interface | ItemKind::TypeAlias);
            let mut info = make_export_info(
                start_line,
                kind,
                child,
                decl,
                file,
                source,
                src_lines,
                is_type_only,
            );
            info.predecessor_start_line = default_predecessor;
            out.push(info);
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
                    ));
                    emitted_lines.insert(method.start_line);
                    needs_sort = true;
                }
            }
        }
        // Member-defining call statements — the dynamic half of the same
        // surface (`defineGetter(r, 'x', fn)`, `list.forEach(m => r[m] = …)`).
        // They always refine the batch that renders the receiver, which
        // keeps them off the file's names surface: a roster that grew by
        // every dynamic definition would price itself out of the early
        // budget and starve the whole file's train behind it (measured
        // dead). `expand_in_dir` drops the ones whose receiver has no
        // rendering batch rather than letting them fall back to the roster.
        let exported_receivers = collect_exported_receivers(tree, source);
        for stmt in collect_member_defining_statements(tree, source, &exported_receivers) {
            if emitted_lines.contains(&stmt.start_line) {
                continue;
            }
            let mut info = make_export_info(
                stmt.start_line,
                ItemKind::Function,
                stmt.anchor,
                stmt.definer,
                file,
                source,
                src_lines,
                false,
            );
            info.predecessor_start_line = Some(stmt.receiver_line);
            info.member_definition_receiver_line = Some(stmt.receiver_line);
            out.push(info);
            emitted_lines.insert(stmt.start_line);
            needs_sort = true;
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
    let class_body = matches!(kind, ItemKind::Class | ItemKind::Default) && is_class_node(decl);
    let body_parts = if collect_class_members {
        class_members
            .iter()
            .flat_map(|member| member.body_parts.clone())
            .collect()
    } else {
        let parts = body_parts(decl, kind, source, src_lines);
        // Factory bodies expose their public surface as two distinct
        // anchors (receiver table + inner-function locations); keep
        // those parts as separate ExportBody batches rather than
        // merging the table into the catalog (which would inflate the
        // body cost out of the auto-injection budget).
        if factory_sibling_body_parts || class_body {
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
        factory_sibling_body_parts,
        predecessor_start_line: None,
        member_definition_receiver_line: None,
    }
}

/// Top-level decls not already covered by `find_export_starts` — the
/// module-private items behind a thin exported API.
fn find_module_items(
    tree: &Tree,
    source: &str,
    src_lines: &[&str],
    export_start_lines: &HashSet<usize>,
    reexported_local_names: &HashSet<String>,
    published_local_name: Option<&str>,
) -> Vec<ModuleItemInfo> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        let Some(kind) = decl_kind(child) else {
            continue;
        };
        if is_require_declaration(child, source) {
            continue;
        }
        let start_line = child.start_position().row + 1;
        if export_start_lines.contains(&start_line) {
            continue;
        }
        if synthetic_commonjs_export_kind(child, source, reexported_local_names).is_some() {
            continue;
        }
        let lines = module_item_lines(kind, child, source);
        let body_parts = body_parts(child, kind, source, src_lines);
        let literal_roster = data_literal_roster(child, kind)
            .filter(|(roster, _)| !file_lines_covered_by(roster, &lines));
        let is_published = published_local_name
            .is_some_and(|published| local_decl_name(child, source) == Some(published));
        out.push(ModuleItemInfo {
            start_line,
            kind,
            lines,
            body_parts,
            literal_roster,
            is_published,
        });
    }
    out
}

fn local_decl_name<'a>(node: Node, source: &'a str) -> Option<&'a str> {
    if matches!(node.kind(), "lexical_declaration" | "variable_declaration") {
        let mut cursor = node.walk();
        let declarator = node
            .children(&mut cursor)
            .find(|c| matches!(c.kind(), "variable_declarator" | "lexical_binding"))?;
        return declarator
            .child_by_field_name("name")
            .and_then(|name| identifier_text(name, source));
    }
    name_of(node, source)
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

/// Surface profile for the dominant-file detector: `(surface item
/// count, is type machinery)`. Deliberately NOT built on
/// `find_export_starts` — that list is render-oriented: it skips
/// `from`-sourced re-exports and synthesizes receiver methods into it
/// past a threshold, so reusing it either misses type-only barrels or
/// double-counts CommonJS surface. Here every top-level statement is
/// counted at most once, by AST span.
pub(super) fn dominant_surface_profile(file: &Path, source: &str) -> Option<(u64, bool)> {
    let language = if is_tsx_file(file) {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language).ok()?;
    let tree = parser.parse(source, None)?;
    let root = tree.root_node();
    let local_type_names = collect_top_level_type_names(&tree, source);
    let module_receivers = collect_module_exports_receivers(&tree, source);
    let receiver_methods = collect_prototype_method_assignments(&tree, source, &module_receivers);

    let mut surface_spans: HashSet<(usize, usize)> = HashSet::new();
    let mut export_statements = 0u64;
    let mut runtime_export_statements = 0u64;
    let mut commonjs_exports = 0u64;
    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() == "export_statement" {
            surface_spans.insert((stmt.start_byte(), stmt.end_byte()));
            export_statements += 1;
            if !export_statement_is_type_only(stmt, source, &local_type_names) {
                runtime_export_statements += 1;
            }
        } else if classify_commonjs_export(stmt, source).is_some() {
            surface_spans.insert((stmt.start_byte(), stmt.end_byte()));
            commonjs_exports += 1;
        }
    }
    for method in &receiver_methods {
        surface_spans.insert((method.anchor.start_byte(), method.anchor.end_byte()));
    }
    let type_machinery = is_declaration_file(file)
        || (commonjs_exports == 0
            && receiver_methods.is_empty()
            && export_statements > 0
            && runtime_export_statements == 0);
    Some((surface_spans.len() as u64, type_machinery))
}

/// Whether a top-level `export_statement` exposes only types. Handles
/// the forms `find_export_starts` doesn't model: `export type { X }
/// from '…'`, per-specifier modifiers (`export { type X }`), and bare
/// local clauses (`export { Foo }`) whose every name resolves to a
/// top-level type declaration or type-only import. `export * from`,
/// default exports, and value declarations are runtime; an
/// unresolvable bare name is conservatively runtime.
fn export_statement_is_type_only(
    stmt: Node,
    source: &str,
    local_type_names: &HashSet<String>,
) -> bool {
    if has_export_type_keyword(stmt, source) {
        return true;
    }
    if let Some(decl) = first_decl_child(stmt) {
        return matches!(
            decl.kind(),
            "interface_declaration" | "type_alias_declaration"
        );
    }
    if has_default_keyword(stmt, source) {
        return false;
    }
    let from_clause = has_from_source(stmt);
    let mut saw_specifier = false;
    let mut sc = stmt.walk();
    for clause in stmt.children(&mut sc) {
        if clause.kind() == "namespace_export" {
            return false;
        }
        if clause.kind() != "export_clause" {
            continue;
        }
        let mut cc = clause.walk();
        for spec in clause.children(&mut cc) {
            if spec.kind() != "export_specifier" {
                continue;
            }
            saw_specifier = true;
            if has_inline_type_modifier(spec) {
                continue;
            }
            if from_clause {
                return false;
            }
            let Some(name_node) = first_identifier_child(spec) else {
                return false;
            };
            if !local_type_names.contains(&source[name_node.start_byte()..name_node.end_byte()]) {
                return false;
            }
        }
    }
    saw_specifier
}

/// Names that are types in this module's top level: interface / type
/// alias declarations (exported or not) and type-only import bindings
/// (`import type { A }` / `import { type A }`), keyed by local name
/// (the alias when one is present).
fn collect_top_level_type_names(tree: &Tree, source: &str) -> HashSet<String> {
    let root = tree.root_node();
    let mut out = HashSet::new();
    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        let decl = if stmt.kind() == "export_statement" {
            first_decl_child(stmt)
        } else {
            Some(stmt)
        };
        if let Some(decl) = decl
            && matches!(
                decl.kind(),
                "interface_declaration" | "type_alias_declaration"
            )
        {
            if let Some(name) = decl.child_by_field_name("name") {
                out.insert(source[name.start_byte()..name.end_byte()].to_string());
            }
            continue;
        }
        if stmt.kind() != "import_statement" {
            continue;
        }
        let whole_import_is_type = has_export_type_keyword(stmt, source)
            || stmt
                .child_by_field_name("import_clause")
                .is_some_and(|c| has_export_type_keyword(c, source));
        let mut sc = stmt.walk();
        for clause in stmt.children(&mut sc) {
            if clause.kind() != "import_clause" {
                continue;
            }
            let mut nc = clause.walk();
            for named in clause.children(&mut nc) {
                if named.kind() != "named_imports" {
                    continue;
                }
                let mut ic = named.walk();
                for spec in named.children(&mut ic) {
                    if spec.kind() != "import_specifier" {
                        continue;
                    }
                    if !(whole_import_is_type || has_inline_type_modifier(spec)) {
                        continue;
                    }
                    let local = spec
                        .child_by_field_name("alias")
                        .or_else(|| spec.child_by_field_name("name"));
                    if let Some(local) = local {
                        out.insert(source[local.start_byte()..local.end_byte()].to_string());
                    }
                }
            }
        }
    }
    out
}

/// True for TypeScript declaration files (`.d.ts` / `.d.tsx`).
pub(crate) fn is_declaration_file(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
        n.ends_with(".d.ts")
            || n.ends_with(".d.tsx")
            || n.ends_with(".d.mts")
            || n.ends_with(".d.cts")
    })
}

/// `TYPE_MACHINERY_FILE_FACTOR` when the file emits no runtime code —
/// `.d.ts` or every top-level export is type-only. `ExportNames` is
/// intentionally NOT damped so the names surface stays visible.
/// Callers exempt the declared API contract (see
/// [`is_declared_api_contract`]) before applying this factor.
fn type_machinery_factor(file: &Path, exports: &[ExportInfo<'_>]) -> f64 {
    if is_runtime_module(file, exports) {
        1.0
    } else {
        TYPE_MACHINERY_FILE_FACTOR
    }
}

/// True when the file emits runtime code — neither a declaration file
/// nor a module whose every export is type-only.
fn is_runtime_module(file: &Path, exports: &[ExportInfo<'_>]) -> bool {
    !is_declaration_file(file) && !(!exports.is_empty() && exports.iter().all(|e| e.is_type_only))
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

/// Local identifier names in top-level re-export clauses (`export { X }`
/// / `export { X as Y }`), split by what the clause publishes. Clauses
/// with a `from` source are plumbing and appear in neither set.
#[derive(Default)]
struct LocalReexports {
    /// Names published as runtime values.
    value: HashSet<String>,
    /// Names published type-only — either a statement-level `export type
    /// { … }` or a per-specifier `export { type … }` modifier.
    type_only: HashSet<String>,
}

impl LocalReexports {
    fn is_empty(&self) -> bool {
        self.value.is_empty() && self.type_only.is_empty()
    }
}

fn collect_local_reexports(tree: &Tree, source: &str) -> LocalReexports {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = LocalReexports::default();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() != "export_statement" || has_from_source(stmt) {
            continue;
        }
        let statement_type_only = has_export_type_keyword(stmt, source);
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
                let Some(name_node) = first_identifier_child(spec) else {
                    continue;
                };
                let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
                if statement_type_only || has_inline_type_modifier(spec) {
                    out.type_only.insert(name);
                } else {
                    out.value.insert(name);
                }
            }
        }
    }
    out
}

/// Implementation declarations behind default-export aliases. Handles
/// both `export default fnName` and the thin instance shape
/// `export default instance; const instance = factory();`, plus single-hop
/// direct calls / constructors (`export default factory()` and
/// `const instance = new LocalClass(); export default instance`).
fn collect_default_implementation_exports(tree: &Tree, source: &str) -> HashMap<String, usize> {
    let root = tree.root_node();
    let mut decls: HashMap<String, Node> = HashMap::new();
    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        if let Some(name) = local_decl_name(stmt, source) {
            decls.insert(name.to_string(), stmt);
        }
    }

    let mut out = HashMap::new();
    let mut cursor = root.walk();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() != "export_statement" || !has_default_keyword(stmt, source) {
            continue;
        }
        let Some(value) = first_decl_or_value_child(stmt) else {
            continue;
        };
        let export_line = stmt.start_position().row + 1;
        if let Some(default_name) = identifier_text(value, source)
            && let Some(default_decl) = decls.get(default_name).copied()
        {
            // A thin `const instance = factory()` alias is a handle on the
            // implementation, not the implementation — chase one hop so the
            // export lands on the declaration a reader wants. Everything
            // else the name refers to *is* the module's value, whatever its
            // shape: a data const, an enum and a function are equally the
            // thing the file publishes.
            if !local_decl_has_implementation_body(default_decl)
                && let Some(factory_name) = const_factory_callee_name(default_decl, source)
                && decls
                    .get(factory_name)
                    .is_some_and(|decl| local_decl_has_implementation_body(*decl))
            {
                out.insert(factory_name.to_string(), export_line);
            } else {
                out.insert(default_name.to_string(), export_line);
            }
        } else if let Some(factory_name) = expression_callee_name(value, source)
            && decls
                .get(factory_name)
                .is_some_and(|decl| local_decl_has_implementation_body(*decl))
        {
            out.insert(factory_name.to_string(), export_line);
        }
    }
    out
}

fn local_decl_has_implementation_body(decl: Node) -> bool {
    match decl_kind(decl) {
        Some(ItemKind::Function | ItemKind::Class) => decl.child_by_field_name("body").is_some(),
        Some(ItemKind::Const) => find_fn_init_body(decl).is_some(),
        _ => false,
    }
}

fn const_factory_callee_name<'a>(decl: Node, source: &'a str) -> Option<&'a str> {
    if !matches!(decl.kind(), "lexical_declaration" | "variable_declaration") {
        return None;
    }
    let mut cursor = decl.walk();
    let declarator = decl
        .children(&mut cursor)
        .find(|c| matches!(c.kind(), "variable_declarator" | "lexical_binding"))?;
    let value = declarator.child_by_field_name("value")?;
    expression_callee_name(value, source)
}

fn expression_callee_name<'a>(expr: Node, source: &'a str) -> Option<&'a str> {
    match expr.kind() {
        "call_expression" => expr
            .child_by_field_name("function")
            .or_else(|| expr.named_child(0))
            .and_then(|callee| identifier_text(callee, source)),
        "new_expression" => expr
            .child_by_field_name("constructor")
            .or_else(|| expr.named_child(0))
            .and_then(|callee| identifier_text(callee, source)),
        _ => None,
    }
}

/// Local name a top-level `module.exports = …` publishes as the file's
/// whole value — `= X`, `= new X()`, `= makeX()`. That declaration is
/// what the file exists to export, so it is not module-private however
/// the assignment spells it. At most one per file, so recognizing it
/// cannot flood a file's schedule the way a surface-set gate can.
fn commonjs_published_local_name<'a>(tree: &Tree, source: &'a str) -> Option<&'a str> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    root.children(&mut cursor)
        .filter_map(|stmt| commonjs_assignment_sides(stmt, source))
        .filter(|(left, _)| {
            matches!(
                commonjs_export_target(*left, source),
                Some(CommonJsExportTarget::Namespace)
            )
        })
        .find_map(|(_, right)| {
            identifier_text(right, source).or_else(|| expression_callee_name(right, source))
        })
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
        collect_module_exports_receivers_from_statement(stmt, source, &mut receivers);
    }
    receivers
}

fn collect_module_exports_receivers_from_statement(
    stmt: Node,
    source: &str,
    receivers: &mut HashSet<String>,
) {
    {
        match stmt.kind() {
            "expression_statement" => {
                if let Some(expr) = stmt.named_child(0)
                    && expr.kind() == "assignment_expression"
                {
                    collect_module_exports_receivers_from_assignment(expr, source, receivers);
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
    // `module.exports = X` — X is the receiver. When the module exports
    // an instance (`module.exports = new Cli()`) or a factory result,
    // the constructor/factory carries the surface, so it is the receiver.
    if matches!(
        commonjs_export_target(left, source),
        Some(CommonJsExportTarget::Namespace)
    ) && let Some(name) =
        identifier_text(right, source).or_else(|| expression_callee_name(right, source))
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

/// Every name the file publishes, mapped to the 1-based line of the
/// statement that publishes it: `module.exports` aliases plus locally
/// declared names an ESM `export` statement re-publishes. A member-defining
/// statement only counts as surface when it installs onto one of these —
/// members of a purely local object are implementation detail — and the
/// publishing line is where its batch chains.
fn collect_exported_receivers(tree: &Tree, source: &str) -> HashMap<String, usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut receivers: HashMap<String, usize> = HashMap::new();
    for stmt in root.children(&mut cursor) {
        let start_line = stmt.start_position().row + 1;
        let mut names: HashSet<String> = HashSet::new();
        collect_module_exports_receivers_from_statement(stmt, source, &mut names);
        if stmt.kind() == "export_statement" {
            // `export default X` / `export = X`.
            if let Some(name) = stmt
                .child_by_field_name("value")
                .and_then(|value| identifier_text(value, source))
            {
                names.insert(name.to_string());
            }
            // `export class X` / `export function X` / `export const X = …`.
            if let Some(name) =
                first_decl_child(stmt).and_then(|decl| local_decl_name(decl, source))
            {
                names.insert(name.to_string());
            }
            // `export { X, Y as Z }` — the local name is what statements target.
            let mut clause_cursor = stmt.walk();
            for clause in stmt
                .children(&mut clause_cursor)
                .filter(|child| child.kind() == "export_clause")
            {
                let mut spec_cursor = clause.walk();
                for spec in clause.named_children(&mut spec_cursor) {
                    if let Some(name) = spec
                        .child_by_field_name("name")
                        .or_else(|| spec.named_child(0))
                        .and_then(|n| identifier_text(n, source))
                    {
                        names.insert(name.to_string());
                    }
                }
            }
        }
        for name in names {
            receivers.entry(name).or_insert(start_line);
        }
    }
    receivers
}

/// One module-scope call statement that installs members on an exported
/// receiver — the dynamic counterpart of a `Receiver.m = function …`
/// assignment.
#[derive(Debug, Clone)]
struct MemberDefiningStatement<'a> {
    start_line: usize,
    anchor: Node<'a>,
    /// The function argument that carries the definition — the surface
    /// line and body slices are taken from it.
    definer: Node<'a>,
    /// Line that publishes the receiver, so the statement can chain onto
    /// the receiver's own export rather than the file's names surface.
    receiver_line: usize,
}

/// Top-level call statements that define members on a tracked receiver.
/// Two shapes qualify, both requiring a function argument:
/// - the receiver is the call's first argument, so the call is a
///   property-definition helper (`defineGetter(req, 'ip', fn)`,
///   `Object.defineProperty(exports, 'x', fn)`);
/// - the function argument's body assigns to a receiver-rooted member,
///   so the call is an installation loop (`methods.forEach(m => app[m] = …)`).
fn collect_member_defining_statements<'a>(
    tree: &'a Tree,
    source: &str,
    receivers: &HashMap<String, usize>,
) -> Vec<MemberDefiningStatement<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for stmt in root.children(&mut cursor) {
        if stmt.kind() != "expression_statement" {
            continue;
        }
        let Some(call) = stmt
            .named_child(0)
            .filter(|e| e.kind() == "call_expression")
        else {
            continue;
        };
        let Some(arguments) = call.child_by_field_name("arguments") else {
            continue;
        };
        let mut arg_cursor = arguments.walk();
        let args: Vec<Node> = arguments.named_children(&mut arg_cursor).collect();
        let Some(definer) = args
            .iter()
            .rev()
            .find(|arg| is_function_node(**arg))
            .copied()
        else {
            continue;
        };
        let receiver = args
            .first()
            .and_then(|arg| identifier_text(*arg, source))
            .filter(|name| receivers.contains_key(*name))
            .or_else(|| installed_receiver_name(definer, source, receivers));
        let Some(receiver_line) = receiver.and_then(|name| receivers.get(name)).copied() else {
            continue;
        };
        out.push(MemberDefiningStatement {
            start_line: stmt.start_position().row + 1,
            anchor: stmt,
            definer,
            receiver_line,
        });
    }
    out
}

fn is_function_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "function_expression" | "arrow_function" | "generator_function"
    )
}

/// Name of the tracked receiver some assignment under `node` installs a
/// member on — `R.m = …`, `R[m] = …`, `R.prototype[m] = …`.
fn installed_receiver_name<'a>(
    node: Node,
    source: &'a str,
    receivers: &HashMap<String, usize>,
) -> Option<&'a str> {
    // A callback parameter or local can shadow the module-scope receiver
    // (`values.forEach(api => { api.local = … })`). Assigning to the
    // shadow mutates a local object, not the published one, so treating
    // it as surface would promote implementation-only code as public API.
    let shadowed = locally_bound_names(node, source);
    let mut cursor = node.walk();
    let mut pending = vec![node];
    while let Some(current) = pending.pop() {
        if current.kind() == "assignment_expression"
            && let Some((left, _)) = assignment_sides(current)
            && let Some(name) = receiver_member_target_name(left, source, receivers)
            && !shadowed.contains(name)
        {
            return Some(name);
        }
        pending.extend(current.named_children(&mut cursor));
    }
    None
}

/// Every name bound anywhere inside `node` — parameters of the function
/// and of any nested function, declared variables, function and class
/// declaration names, and `catch` bindings. Deliberately scope-flat: a
/// name bound anywhere under the callback cannot be relied on to mean the
/// module-scope receiver at the assignment site.
fn locally_bound_names(node: Node, source: &str) -> HashSet<String> {
    let mut cursor = node.walk();
    let mut names = HashSet::new();
    let mut pending = vec![node];
    while let Some(current) = pending.pop() {
        let binder = match current.kind() {
            "formal_parameters" | "object_pattern" | "array_pattern" => Some(current),
            "variable_declarator" | "lexical_binding" => current.child_by_field_name("name"),
            "function_declaration"
            | "generator_function_declaration"
            | "class_declaration"
            | "arrow_function"
            | "function_expression" => current.child_by_field_name("name").or_else(|| {
                // `x => …` binds its single parameter without a
                // `formal_parameters` wrapper.
                current
                    .child_by_field_name("parameter")
                    .filter(|_| current.kind() == "arrow_function")
            }),
            "catch_clause" => current.child_by_field_name("parameter"),
            _ => None,
        };
        if let Some(binder) = binder {
            collect_pattern_identifiers(binder, source, &mut names);
        }
        pending.extend(current.named_children(&mut cursor));
    }
    names
}

/// Identifier names introduced by a binding form — the node itself when
/// it is a plain identifier, otherwise every identifier under it that is
/// not a property key or a default-value expression.
fn collect_pattern_identifiers(node: Node, source: &str, names: &mut HashSet<String>) {
    if let Some(name) = identifier_text(node, source) {
        names.insert(name.to_string());
        return;
    }
    let mut cursor = node.walk();
    let mut pending = vec![node];
    while let Some(current) = pending.pop() {
        if matches!(
            current.kind(),
            "identifier" | "shorthand_property_identifier_pattern"
        ) {
            names.insert(source[current.start_byte()..current.end_byte()].to_string());
            continue;
        }
        // `{ key: local }` binds `local`; `key` is a property name.
        if current.kind() == "pair_pattern" {
            if let Some(value) = current.child_by_field_name("value") {
                pending.push(value);
            }
            continue;
        }
        // `x = fallback` binds `x`; the fallback is an expression.
        if current.kind() == "assignment_pattern" {
            if let Some(left) = current.child_by_field_name("left") {
                pending.push(left);
            }
            continue;
        }
        pending.extend(current.named_children(&mut cursor));
    }
}

/// Receiver name when `node` is a member or subscript access rooted at a
/// tracked receiver, directly or through its `prototype`.
fn receiver_member_target_name<'a>(
    node: Node,
    source: &'a str,
    receivers: &HashMap<String, usize>,
) -> Option<&'a str> {
    let object = match node.kind() {
        "member_expression" => member_object_property(node).map(|(object, _)| object),
        "subscript_expression" => subscript_object(node),
        _ => None,
    }?;
    let direct = identifier_text(object, source).filter(|name| receivers.contains_key(*name));
    if direct.is_some() {
        return direct;
    }
    let (inner_object, inner_property) = member_object_property(object)?;
    (object.kind() == "member_expression" && identifier_eq(inner_property, source, "prototype"))
        .then(|| identifier_text(inner_object, source))
        .flatten()
        .filter(|name| receivers.contains_key(*name))
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
/// top-level re-export clause (`export { X }`) whose names are in
/// `reexport_set`. Named declarations are keyed directly by their
/// declared name; const/let declarations keep the narrower fn-init rule
/// (see [`synthetic_export_name`]), which excludes pure-data consts.
fn synthetic_local_export_kind(
    decl: Node,
    source: &str,
    reexport_set: &HashSet<String>,
) -> Option<ItemKind> {
    let kind = decl_kind(decl)?;
    match kind {
        ItemKind::Const => synthetic_export_name(decl, source, reexport_set).map(|_| kind),
        ItemKind::Function
        | ItemKind::Class
        | ItemKind::Interface
        | ItemKind::TypeAlias
        | ItemKind::Enum => {
            let name = name_of(decl, source)?;
            reexport_set.contains(name).then_some(kind)
        }
        ItemKind::Default | ItemKind::NamedReexport => None,
    }
}

/// [`synthetic_local_export_kind`] for a type-only clause (`export type
/// { X }` / `export { type X }`). Only declarations that exist purely in
/// the type world qualify: such a clause deliberately withholds the
/// runtime binding, so promoting a const / function / class / enum
/// behind one would overstate the module's runtime surface.
fn synthetic_local_type_export_kind(
    decl: Node,
    source: &str,
    reexport_set: &HashSet<String>,
) -> Option<ItemKind> {
    let kind = decl_kind(decl)?;
    if !matches!(kind, ItemKind::Interface | ItemKind::TypeAlias) {
        return None;
    }
    let name = name_of(decl, source)?;
    reexport_set.contains(name).then_some(kind)
}

/// [`synthetic_local_export_kind`] restricted to runtime declarations —
/// a CommonJS `exports.X = X` assignment can only name a runtime value.
fn synthetic_commonjs_export_kind(
    decl: Node,
    source: &str,
    reexport_set: &HashSet<String>,
) -> Option<ItemKind> {
    let kind = synthetic_local_export_kind(decl, source, reexport_set)?;
    (!matches!(
        kind,
        ItemKind::Interface | ItemKind::TypeAlias | ItemKind::Enum
    ))
    .then_some(kind)
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
    node.children(&mut cursor).find_map(|child| {
        decl_kind(child)
            .is_some()
            .then_some(child)
            .or_else(|| ambient_inner_decl(child))
    })
}

/// The declaration inside an `ambient_declaration` wrapper. Tree-sitter
/// interposes that node for `declare function …` / `declare class …` /
/// `declare const …`, which is how nearly everything in a `.d.ts` is
/// spelled; callers that reason about declaration kinds have to see
/// through it.
fn ambient_inner_decl(node: Node) -> Option<Node> {
    if node.kind() != "ambient_declaration" {
        return None;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor).find(|c| decl_kind(*c).is_some())
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
        && matches!(ext, "ts" | "tsx" | "mts" | "cts" | "js" | "mjs" | "cjs")
}

pub(crate) fn is_ts_or_tsx_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| {
        e.eq_ignore_ascii_case("ts")
            || e.eq_ignore_ascii_case("tsx")
            || e.eq_ignore_ascii_case("mts")
            || e.eq_ignore_ascii_case("cts")
    })
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
    // When `path` isn't under the root, `depth_from_root` fails open to
    // 0, which reads as relative_depth 0 and would re-pin every deep
    // entrypoint. Fail closed: with depth unknown, only the export-spine
    // check can grant the pin.
    let hit = if path.starts_with(ctx.root()) {
        let relative_depth = ctx.depth_from_root(path).saturating_sub(package_dir_depth);
        relative_depth <= 2 || is_api_spine_entrypoint(path, ctx)
    } else {
        is_api_spine_entrypoint(path, ctx)
    };
    state
        .pinned_entrypoint_lookup
        .borrow_mut()
        .insert(path.to_path_buf(), hit);
    hit
}

const JS_CONFIG_VALUE_FACTOR: f64 = 0.001;
const PRIMARY_JS_VALUE_FACTOR: f64 = 0.85;
const SECONDARY_JS_VALUE_FACTOR: f64 = 0.05;
/// Own tier between the two: a flat-layout package's published
/// directories are the source tree, but they are not the wrapper the
/// primary tier was calibrated on, so they rank between the two.
const FLAT_LAYOUT_JS_VALUE_FACTOR: f64 = 0.35;

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
    } else if is_flat_layout_source_path(path, ctx) {
        FLAT_LAYOUT_JS_VALUE_FACTOR
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

/// A package with no `src`/`lib`/`source` wrapper keeps its source at the
/// top level, so the directories its manifest publishes *are* the source
/// tree — not secondary helpers next to one. Restricted to the published
/// set so unpublished siblings (tests, benchmarks, examples) stay
/// secondary, and inert wherever a wrapper exists.
fn is_flat_layout_source_path(path: &Path, ctx: &WalkCtx) -> bool {
    // Fail closed for non-JS: this is a tier *within* the JS value
    // ladder, and it sits above the `is_js_file` branch in
    // `js_value_factor`, so a path-only answer would demote a
    // wrapper-less package's TypeScript off the 1.0 default.
    if !is_js_file(path) {
        return false;
    }
    let root = ctx.root();
    let state = ctx.typescript_state();
    let pkg_dir = state
        .nearest_subpackage_dir(path, root)
        .unwrap_or_else(|| root.to_path_buf());
    let Ok(rel) = path.strip_prefix(&pkg_dir) else {
        return false;
    };
    let Some(std::path::Component::Normal(top)) = rel.components().next() else {
        return false;
    };
    let Some(top) = top.to_str() else {
        return false;
    };
    let mut cache = state.flat_source_dirs_lookup.borrow_mut();
    cache
        .entry(pkg_dir.clone())
        .or_insert_with(|| flat_layout_source_dirs(&pkg_dir))
        .iter()
        .any(|dir| dir == top)
}

fn flat_layout_source_dirs(pkg_dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(pkg_dir) else {
        return Vec::new();
    };
    let mut child_dirs = Vec::new();
    for entry in entries.flatten() {
        if !entry.path().is_dir() {
            continue;
        }
        if is_source_dir(&entry.path()) {
            return Vec::new();
        }
        if let Some(name) = entry.file_name().to_str() {
            child_dirs.push(name.to_string());
        }
    }
    super::json::published_top_level_dir_names(pkg_dir)
        .into_iter()
        .filter(|name| {
            // A published *build output* directory is the package's
            // shipped artifact, not the source that produced it.
            !GENERATED_ENTRY_DIR_PREFIXES.contains(&name.as_str())
                && child_dirs.iter().any(|dir| dir == name)
        })
        .collect()
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
/// `workspace_member_value_factor`.
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
const GENERATED_ENTRY_DIR_PREFIXES: &[&str] =
    &["dist", "build", "out", "output", "lib", "esm", "cjs"];
/// Source-tree prefixes tried when re-rooting a generated entry path.
const SOURCE_ENTRY_DIR_PREFIXES: &[&str] = &["", "src", "source"];
const ENTRY_SOURCE_EXTS: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];

/// True when `file` is the source file behind its package's declared
/// entry (`main` / `module` / `exports["."]`) — either named directly,
/// or via the conventional generated-dir mapping (`./dist/node/index.js`
/// → `src/node/index.ts`). Declaration files are never entry sources:
/// the `types` condition under `exports["."]` names the package's type
/// contract, not what the runtime loads, and the contract is priced by
/// [`TypescriptState::declared_api_contract`] instead.
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
    declared_package_entry_sources_from_targets(&pkg_dir, targets)
        .any(|candidate| candidate.canonicalize().ok().as_ref() == Some(&canonical_file))
}

fn declared_package_entry_sources(pkg_dir: &Path) -> Vec<PathBuf> {
    declared_package_entry_sources_from_targets(
        pkg_dir,
        super::json::package_entry_targets(pkg_dir),
    )
    .filter(|candidate| candidate.is_file())
    .collect()
}

fn declared_package_entry_sources_from_targets(
    pkg_dir: &Path,
    targets: Vec<String>,
) -> impl Iterator<Item = PathBuf> {
    let mut candidates = Vec::new();
    for target in targets {
        let rel = target.trim_start_matches("./");
        if rel.is_empty() {
            continue;
        }
        // Manifest targets are repository-controlled: reject absolute paths
        // and parent components so a crafted `main`/`exports` entry cannot
        // make the walker probe files outside the package directory.
        if Path::new(rel)
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            continue;
        }
        let stem = Path::new(rel).with_extension("");
        let mut variants = vec![stem.clone()];
        // Strip nested generated dirs level by level (`dist/esm/index`,
        // `dist/cjs/index`) until no leading generated prefix remains.
        let mut current = stem.clone();
        while let Some(stripped) = GENERATED_ENTRY_DIR_PREFIXES
            .iter()
            .find_map(|prefix| current.strip_prefix(prefix).ok())
        {
            current = stripped.to_path_buf();
            variants.push(current.clone());
        }
        for variant in &variants {
            for source_prefix in SOURCE_ENTRY_DIR_PREFIXES {
                let base = if source_prefix.is_empty() {
                    pkg_dir.join(variant)
                } else {
                    pkg_dir.join(source_prefix).join(variant)
                };
                for ext in ENTRY_SOURCE_EXTS {
                    // `append_extension`, not `with_extension`: a dotted
                    // stem like `foo.config` would have its `.config`
                    // treated as an extension and replaced (→ `foo.ts`).
                    let candidate = append_extension(&base, ext);
                    if !is_declaration_file(&candidate) {
                        candidates.push(candidate);
                    }
                }
            }
        }
    }
    candidates.into_iter()
}

fn export_names_value(
    file: &Path,
    ctx: &WalkCtx,
    js_factor: f64,
    has_split_js_class_export: bool,
    has_oversized_ts_class_export: bool,
) -> f64 {
    let cat = (0.8 * entrypoint_boost(file, ctx)).min(1.0);
    let class_split_factor = if has_oversized_ts_class_export {
        1.25
    } else if has_split_js_class_export {
        1.12
    } else {
        1.0
    };
    mix_signals(cat, 0.6, 0.35, ts_depth_factor(file, ctx)) * js_factor * class_split_factor
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

/// A big declaration's member-name catalog — the per-member surface
/// with a roster-mass boost for the catalog's total size so complete
/// catalogs stay ratio-competitive with tiny exports.
fn export_member_names_value(
    file: &Path,
    kind: ItemKind,
    ctx: &WalkCtx,
    js_factor: f64,
    member_count: usize,
) -> f64 {
    export_member_value(file, kind, ctx, js_factor) * roster_mass_factor(member_count)
}

/// The keys of a value a declaration binds are that value's members, so
/// a complete key roster prices at the member-catalog tier rather than
/// at the host declaration's own tier: an option name or a table key is
/// not derivable from anything else in the file, which is the
/// catastrophic-omission class. The host's reach still enters through
/// `js_factor` and the depth factor.
fn literal_roster_value(
    file: &Path,
    kind: ItemKind,
    ctx: &WalkCtx,
    js_factor: f64,
    element_count: usize,
) -> f64 {
    export_member_names_value(file, kind, ctx, js_factor, element_count)
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

/// Minimum emitted module items for the unified first-line catalog —
/// below this the per-item batches are not a flood worth a roster.
const MODULE_ITEM_CATALOG_MIN: usize = 6;

/// Names-surface tier for the module-item catalog, roster-mass
/// neutralized like the other unified catalogs.
fn module_item_names_value(file: &Path, ctx: &WalkCtx, js_factor: f64, item_count: usize) -> f64 {
    let cat = (0.5 * entrypoint_boost(file, ctx)).min(1.0);
    mix_signals(cat, 0.55, 0.35, ts_depth_factor(file, ctx))
        * js_factor
        * roster_mass_factor(item_count)
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

/// Bare `export … from` statements past the file's import prologue.
///
/// [`collect_imports`] stops at the first statement that isn't import
/// plumbing, and the chunked path bails once the file carries more than
/// a few lines of other code, so a re-export block placed *after* a
/// module's implementation is claimed by nothing. On that shape the
/// block is the module's declared public surface — commonly its whole
/// type surface — rather than plumbing, which is why it gets its own
/// roster-priced key instead of widening the imports batch.
///
/// `claimed` carries the lines the imports batch and the export roster
/// already own; peer batches may not overlap.
fn collect_reexport_tail(tree: &Tree, source: &str, claimed: &HashSet<usize>) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "export_statement" && is_bare_reexport(child) {
            extend_span(&mut lines, child, source);
        }
    }
    lines.retain(|line| !claimed.contains(line));
    FileLines::new(dedup_sorted(lines))
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

/// Star re-exports (`export * from '…'` / `export * as ns from '…'`)
/// that belong on the file's export roster rather than in the plumbing
/// batch. Unlike a named re-export, a star does not enumerate what it
/// publishes — it says the module's surface is *larger* than whatever
/// is listed beside it. Left in the imports batch it competes with the
/// roster and routinely loses, and the surviving `export { … }` sibling
/// then reads as the module's complete surface.
///
/// Empty when the plumbing surface is itself a re-export wall: there
/// the stars *are* the content, they are already priced as a roster,
/// and hoisting a wall's worth of them would swamp the names surface.
fn roster_star_reexport_lines(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "export_statement" && is_star_reexport(child) {
            extend_span(&mut lines, child, source);
        }
    }
    if lines.is_empty()
        || is_mostly_reexport(
            &collect_imports(tree, source),
            &collect_bare_reexport_lines(tree, source),
        )
    {
        return Vec::new();
    }
    dedup_sorted(lines)
}

/// A bare re-export in the `*` / `* as ns` form, as opposed to the
/// name-enumerating `export { a, b } from '…'`.
fn is_star_reexport(node: Node) -> bool {
    is_bare_reexport(node) && {
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .any(|child| matches!(child.kind(), "namespace_export" | "*"))
    }
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
    top_level_decl_start_lines: &HashSet<usize>,
    script_statement_rows: &HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for item in items {
        if item.predecessor_start_line.is_some() {
            continue;
        }
        full.push(item.start_line);
        let ellipsis_line = item.start_line + 1;
        // Suppress the courtesy ellipsis when the next source line is
        // structural — another real export start, or a line owned by
        // the imports / re-export-chunk batch, or a row a script-flow
        // statement batch claims. The import case only matters when an
        // inline `export const` / `export type { … }` sits adjacent to a
        // bare re-export wall (TS entrypoint pattern): peer batches can't
        // overlap line ownership.
        if !export_start_lines.contains(&ellipsis_line)
            && !import_owned_lines.contains(&ellipsis_line)
            && !top_level_decl_start_lines.contains(&ellipsis_line)
            && !script_statement_rows.contains(&ellipsis_line)
        {
            ellipses.push(ellipsis_line);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

fn top_level_decl_start_lines(tree: &Tree) -> HashSet<usize> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    root.children(&mut cursor)
        .filter(|child| decl_kind(*child).is_some())
        .map(|child| child.start_position().row + 1)
        .collect()
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

/// Minimum top-level elements before a data literal is worth a roster.
/// Below it the literal is small enough that its own body batch is the
/// cheap read.
const DATA_LITERAL_ROSTER_MIN: usize = 4;

/// The multi-line array/object literal `decl` binds, plus the row that
/// binds it: either the declaration's own initializer, or the largest
/// literal a statement in its body assigns to a name
/// (`function Cli () { this.cliOpts = […] }` — a data table wearing a
/// constructor's clothes). Nested callbacks are out of reach on purpose:
/// only statements directly in the declaration's own body, or in one of
/// its members' bodies, define the declaration's data surface.
fn bound_data_literal<'a>(decl: Node<'a>, kind: ItemKind) -> Option<(Node<'a>, usize)> {
    let literal = declared_data_literal(decl).or_else(|| body_assigned_data_literal(decl, kind))?;
    (literal.end_position().row > literal.start_position().row)
        .then(|| (literal, literal.start_position().row + 1))
}

/// Largest multi-line data literal assigned by a statement sitting
/// directly in one of `decl`'s bodies.
fn body_assigned_data_literal<'a>(decl: Node<'a>, kind: ItemKind) -> Option<Node<'a>> {
    let bodies: Vec<Node<'a>> = match decl_fn_body(decl, kind) {
        Some(body) => vec![body],
        None => match decl.child_by_field_name("body") {
            Some(class_body) => {
                let mut member_cursor = class_body.walk();
                class_body
                    .children(&mut member_cursor)
                    .filter_map(|member| member.child_by_field_name("body"))
                    .collect()
            }
            None => Vec::new(),
        },
    };
    bodies
        .into_iter()
        .flat_map(|body| {
            let mut cursor = body.walk();
            body.named_children(&mut cursor).collect::<Vec<_>>()
        })
        .filter_map(|statement| {
            let assignment = statement
                .named_child(0)
                .filter(|expr| expr.kind() == "assignment_expression")?;
            data_literal_node(assignment.child_by_field_name("right")?)
        })
        .max_by_key(|literal| literal.end_position().row - literal.start_position().row)
}

fn data_literal_node(node: Node) -> Option<Node> {
    matches!(node.kind(), "array" | "object").then_some(node)
}

fn declared_data_literal(decl: Node) -> Option<Node> {
    if let Some(literal) = data_literal_node(decl) {
        return Some(literal);
    }
    let mut cursor = decl.walk();
    let declarator = decl
        .children(&mut cursor)
        .find(|child| matches!(child.kind(), "variable_declarator" | "lexical_binding"))?;
    data_literal_node(declarator.child_by_field_name("value")?)
}

/// One row per top-level element of `literal`. An element that is itself
/// a multi-line object is named by its first property row: the row
/// carrying its opening brace identifies nothing, while
/// `{ name: 'socketPath', …` does.
fn data_literal_element_rows(literal: Node) -> Vec<usize> {
    let mut cursor = literal.walk();
    literal
        .named_children(&mut cursor)
        .filter(|element| element.kind() != "comment")
        .map(|element| {
            let named = (element.kind() == "object"
                && element.end_position().row > element.start_position().row)
                .then(|| {
                    let mut inner = element.walk();
                    element
                        .named_children(&mut inner)
                        .find(|child| child.kind() != "comment")
                })
                .flatten()
                .unwrap_or(element);
            named.start_position().row + 1
        })
        .collect()
}

/// Key roster of the data literal `decl` binds: the binding row plus one
/// row per top-level element, everything between elided. `None` unless
/// the roster is a real abbreviation of the literal — a roster as long as
/// the literal is the literal, and the literal already has a body batch.
fn data_literal_roster(decl: Node, kind: ItemKind) -> Option<(FileLines, usize)> {
    let (literal, binding_row) = bound_data_literal(decl, kind)?;
    let literal_rows = literal.end_position().row - literal.start_position().row + 1;
    let element_rows = data_literal_element_rows(literal);
    if element_rows.len() < DATA_LITERAL_ROSTER_MIN || element_rows.len() * 2 > literal_rows {
        return None;
    }
    let count = element_rows.len();
    let rows = std::iter::once(binding_row).chain(element_rows).collect();
    Some((FileLines::new(dedup_sorted(rows)), count))
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

/// Split one oversized exported TS class surface into a structural head and
/// predecessor-chained tails. The head keeps the class signature, fields,
/// and constructor signature together; tails cut only between later member
/// headers. The chunks are disjoint and exactly cover the old surface slab.
fn oversized_export_class_chunks(
    file: &Path,
    item: &ExportInfo<'_>,
    source: &str,
    ctx: &WalkCtx,
) -> Option<Vec<FileLines>> {
    if !is_ts_or_tsx_file(file)
        || is_declaration_file(file)
        || !matches!(item.kind, ItemKind::Class | ItemKind::Default)
        || !is_class_node(item.decl)
    {
        return None;
    }
    let body = item.decl.child_by_field_name("body")?;
    let surface = decl_surface_lines(item.kind, item.anchor, item.decl, source, true);
    let cost = |lines: &FileLines| {
        single_file_lines_content(file, source, lines.clone())
            .map(|content| ctx.marginal_tokens(&content))
            .unwrap_or(0)
    };
    if cost(&surface) < OVERSIZE_EXPORT_SPLIT_TOKENS {
        return None;
    }

    let mut cursor = body.walk();
    let members: Vec<Node<'_>> = body
        .named_children(&mut cursor)
        .filter(|member| {
            class_surface_member_kind(member.kind()) && !is_non_public_class_member(*member, source)
        })
        .collect();
    if members.len() < 2 {
        return None;
    }
    let head_last_index = members
        .iter()
        .position(|member| name_of(*member, source) == Some("constructor"))
        .or_else(|| {
            members
                .iter()
                .take_while(|member| member.kind() == "public_field_definition")
                .count()
                .checked_sub(1)
        })
        .unwrap_or(0);
    if head_last_index + 1 >= members.len() {
        return None;
    }

    let header_end = body.start_position().row;
    let mut header_lines = Vec::new();
    push_rows(
        &mut header_lines,
        item.anchor.start_position().row,
        header_end,
    );
    let mut head = FileLines::new(header_lines);
    for member in members.iter().take(head_last_index + 1) {
        let member_lines = member_header_lines(*member).0;
        head.full.extend(member_lines.full);
        head.ellipses.extend(member_lines.ellipses);
    }
    let tail_members = &members[head_last_index + 1..];
    let lines_for = |range: std::ops::Range<usize>| {
        let mut lines = FileLines::default();
        for member in &tail_members[range] {
            let member_lines = member_header_lines(*member).0;
            lines.full.extend(member_lines.full);
            lines.ellipses.extend(member_lines.ellipses);
        }
        lines
    };
    let tail_ranges = budget_chunk_ranges(
        tail_members.len(),
        |range| cost(&lines_for(range)),
        OVERSIZE_EXPORT_CHUNK_TARGET_TOKENS,
        OVERSIZE_EXPORT_CHUNK_MIN_TAIL_TOKENS,
        |_| true,
        |_| true,
    );
    let mut chunks = vec![head];
    chunks.extend(tail_ranges.into_iter().map(lines_for));
    if chunks.len() == 2 && cost(&chunks[1]) < OVERSIZE_EXPORT_CHUNK_MIN_TAIL_TOKENS {
        let tail = chunks.pop().expect("two chunks checked above");
        chunks[0].full.extend(tail.full);
        chunks[0].ellipses.extend(tail.ellipses);
    }
    (chunks.len() > 1).then_some(chunks)
}

/// Body node holding a declaration's member list — class body,
/// interface body, or the object-literal type of a type alias.
fn member_surface_body<'a>(kind: ItemKind, decl: Node<'a>) -> Option<Node<'a>> {
    match kind {
        ItemKind::Interface => decl.child_by_field_name("body"),
        ItemKind::TypeAlias => decl.child_by_field_name("value").and_then(own_object_type),
        ItemKind::Class | ItemKind::Default if is_class_node(decl) => {
            decl.child_by_field_name("body")
        }
        _ => None,
    }
}

/// The member block an alias declares in its own right. A mixin
/// intersection (`Base & Other & { … }`) states the alias's own members
/// in its trailing object literal — the earlier operands name types
/// declared elsewhere and surface nothing here.
fn own_object_type(value: Node<'_>) -> Option<Node<'_>> {
    match value.kind() {
        "object_type" => Some(value),
        "intersection_type" => {
            let mut cursor = value.walk();
            value
                .named_children(&mut cursor)
                .filter(|operand| operand.kind() == "object_type")
                .last()
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
    let mut cursor = body.walk();
    body.children(&mut cursor)
        .filter(|member| is_surfaced_member(kind, decl, *member, source))
        .map(|member| {
            (
                member.start_position().row + 1,
                member.end_position().row + 1,
            )
        })
        .collect()
}

/// The members of an exported declaration whose signature row some
/// batch renders — the same set the member surfaces and catalogs are
/// built from, which is what makes a member's doc gateable on its own
/// signature's owner.
fn documented_member_nodes<'a>(item: &ExportInfo<'a>, source: &str) -> Vec<Node<'a>> {
    let Some(body) = member_surface_body(item.kind, item.decl) else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    let members: Vec<Node<'a>> = body
        .children(&mut cursor)
        .filter(|member| is_surfaced_member(item.kind, item.decl, *member, source))
        .collect();
    members
}

fn is_surfaced_member(kind: ItemKind, decl: Node, member: Node, source: &str) -> bool {
    let surfaced = if is_class_node(decl) {
        class_surface_member_kind(member.kind()) && !is_non_public_class_member(member, source)
    } else if matches!(kind, ItemKind::Interface | ItemKind::TypeAlias) {
        matches!(
            member.kind(),
            "property_signature"
                | "method_signature"
                | "call_signature"
                | "construct_signature"
                | "index_signature"
        )
    } else {
        false
    };
    surfaced && !name_of(member, source).is_some_and(|name| name.starts_with('_'))
}

/// Member-name catalog chunks for one big declaration.
#[derive(Clone)]
struct MemberNamesCatalog {
    lines: FileLines,
    /// Class catalogs render truncated-to-name (the catalog is *which
    /// methods exist*); interface/object-type catalogs keep full lines
    /// (the field's type IS the content).
    truncate_to_name: bool,
}

/// Minimum member count for a big interface / object-type alias to
/// trade its whole-member surface for a header + member-name catalog.
const MEMBER_CATALOG_MIN_MEMBERS: usize = 12;

/// Unified member-first-line surface for a big declaration — the
/// "header then one line per member" catalog shape NS authors anchor
/// on. `None` below the catalog minimum; multi-line members keep an
/// ellipsis marker unless the next line is another member.
///
/// Classes only catalog above the per-member split range: a mid-size
/// class's whole-surface slab is affordable and NS rows want its full
/// signature lines, while an oversize class's slab never schedules —
/// the truncated name catalog is the only deliverable shape.
fn member_names_catalog_lines(
    kind: ItemKind,
    decl: Node,
    source: &str,
) -> Option<MemberNamesCatalog> {
    let spans = member_surface_spans(kind, decl, source);
    let min_members = if is_class_node(decl) {
        JS_CLASS_MEMBER_SPLIT_MAX + 1
    } else {
        MEMBER_CATALOG_MIN_MEMBERS
    };
    if spans.len() < min_members {
        return None;
    }
    member_names_catalog_from_spans(spans, is_class_node(decl))
}

fn member_names_catalog_from_spans(
    spans: Vec<(usize, usize)>,
    truncate_to_name: bool,
) -> Option<MemberNamesCatalog> {
    if spans.is_empty() {
        return None;
    }
    let first_lines: HashSet<usize> = spans.iter().map(|&(first, _)| first).collect();
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for &(first, end) in &spans {
        full.push(first);
        if !truncate_to_name && end > first && !first_lines.contains(&(first + 1)) {
            ellipses.push(first + 1);
        }
    }
    Some(MemberNamesCatalog {
        lines: FileLines::new(full).with_ellipses(ellipses),
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

fn member_names_catalog_content(
    file: &Path,
    source: &str,
    lines: &FileLines,
    truncate_to_name: bool,
) -> Option<BatchContent> {
    if truncate_to_name {
        truncated_member_names_content(file, source, lines)
    } else {
        single_file_lines_content(file, source, lines.clone())
    }
}

/// Slice a catalog that breaches [`MEMBER_CATALOG_MAX_TOKENS`] at member
/// boundaries in source order, so every member appears exactly once.
/// Each slice costs at most the ceiling plus one member row — a member
/// row is the atom, so this bounds the roster's cost by the source's
/// longest single member line instead of by its member count, which is
/// the property the scheduler needs.
fn member_names_catalog_cap_chunks(
    file: &Path,
    source: &str,
    catalog: &MemberNamesCatalog,
    ctx: &WalkCtx,
) -> Vec<FileLines> {
    let lines_for = |range: std::ops::Range<usize>| {
        let full = catalog.lines.full[range].to_vec();
        let ellipses = catalog
            .lines
            .ellipses
            .iter()
            .copied()
            .filter(|ellipsis| full.iter().any(|line| *ellipsis == line + 1))
            .collect();
        FileLines::new(full).with_ellipses(ellipses)
    };
    let cost = |range: std::ops::Range<usize>| {
        member_names_catalog_content(file, source, &lines_for(range), catalog.truncate_to_name)
            .map(|content| ctx.marginal_tokens(&content))
            .unwrap_or(0)
    };
    // `min_tail` 0 disables tail merging: merging is what would let a
    // slice grow past the ceiling after the fact.
    budget_chunk_ranges(
        catalog.lines.full.len(),
        cost,
        MEMBER_CATALOG_MAX_TOKENS,
        0,
        |_| true,
        |_| true,
    )
    .into_iter()
    .map(lines_for)
    .collect()
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
    let file = Path::new("fixture.ts");
    let src_lines: Vec<&str> = source.lines().collect();
    let reexports = collect_local_reexports(tree, source);
    let commonjs_reexports = collect_commonjs_value_reexports(tree, source);
    let default_identifier_reexports = collect_default_implementation_exports(tree, source);
    find_export_starts(
        file,
        tree,
        source,
        &src_lines,
        &reexports,
        &commonjs_reexports,
        &default_identifier_reexports,
    )
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

/// The JSDoc block documenting one member of a declaration, rendered as
/// the member-doc batch renders it — the comment rows only, never the
/// member's signature row, which the batch's predecessor owns.
///
/// A doc block is prose plus, often, a worked example costing several
/// times the prose. The prose states the member's contract, so the
/// batch renders down to the first example and marks the rest elided.
/// Measured against rendering the whole block, and against the same
/// lede with no marker: this shape wins at every budget.
fn member_doc_lines(member: Node, source: &str, src_lines: &[&str]) -> Option<FileLines> {
    let doc = member_jsdoc_block(member, source)?;
    let start = doc.start_position().row + 1;
    let end = node_end_row_trimmed(doc, source) + 1;
    let example = (start + 1..=end).find(|line| {
        src_lines
            .get(line - 1)
            .is_some_and(|text| is_worked_example_jsdoc_line(text))
    });
    let Some(example) = example else {
        return Some(FileLines::new((start..=end).collect()));
    };
    // The blank separator line before the example belongs to neither
    // half; the lede ends at the last row that renders prose.
    let lede_end = (start..example).rev().find(|line| {
        src_lines
            .get(line - 1)
            .is_some_and(|t| !is_blank_jsdoc_line(t))
    })?;
    Some(FileLines::new((start..=lede_end).collect()).with_ellipses(vec![example]))
}

/// A doc-comment row carrying no prose — empty, or the bare `*`
/// continuation that separates paragraphs in conventional JSDoc.
fn is_blank_jsdoc_line(text: &str) -> bool {
    text.trim_start().trim_start_matches('*').trim().is_empty()
}

/// The `/** */` block documenting `member`.
///
/// Two node kinds routinely sit between a member and its doc block, and
/// neither can belong to anything but that member, so the walk crosses
/// both: the line-comment directives that annotate the declaration
/// below them (`// eslint-disable-next-line …`), and **decorators** —
/// the grammar seats each `@Decorator(…)` as its own sibling ahead of
/// the member, so in an annotated codebase the doc block is never the
/// member's immediate previous sibling. Anything else, including the
/// preceding member, is the boundary: past it a doc block documents
/// something other than this member.
fn member_jsdoc_block<'a>(member: Node<'a>, source: &str) -> Option<Node<'a>> {
    let mut cur = member.prev_sibling();
    while let Some(prev) = cur {
        match prev.kind() {
            "comment" if source[prev.start_byte()..prev.end_byte()].starts_with("/**") => {
                return Some(prev);
            }
            "comment" | "decorator" => cur = prev.prev_sibling(),
            _ => return None,
        }
    }
    None
}

/// A doc-comment row that opens a worked example — the `@example` tag
/// or a fenced code block. Both are the documentation vocabulary for
/// "here is the same thing again, spelled out"; the prose above is what
/// states the contract.
fn is_worked_example_jsdoc_line(text: &str) -> bool {
    let body = text.trim_start().trim_start_matches('*').trim_start();
    body.starts_with("```")
        || body
            .strip_prefix("@example")
            .is_some_and(|rest| !rest.starts_with(|c: char| c.is_alphanumeric()))
}

/// Lines of the JSDoc above `node` that disavow what it documents.
/// Empty for a live export — a disavowal is rare, so a clean roster
/// pays nothing for this.
fn disavowal_doc_lines(
    node: Node,
    source: &str,
    src_lines: &[&str],
    skip_module_lede: bool,
) -> Vec<usize> {
    let mut doc_lines = Vec::new();
    collect_jsdoc_above(node, source, &mut doc_lines, skip_module_lede);
    doc_lines
        .into_iter()
        .filter(|line| {
            src_lines
                .get(line - 1)
                .is_some_and(|text| is_disavowal_jsdoc_line(text))
        })
        .collect()
}

/// A JSDoc body line whose tag *contradicts* the item's presence on an
/// export roster. The rest of the tag vocabulary — `@param`,
/// `@returns`, `@public`, `@example` — describes or affirms the item,
/// so it keeps competing as ordinary documentation.
fn is_disavowal_jsdoc_line(text: &str) -> bool {
    let body = text
        .trim_start()
        .trim_start_matches(['/', '*'])
        .trim_start();
    ["@deprecated", "@internal", "@private"].iter().any(|tag| {
        body.strip_prefix(tag)
            .is_some_and(|rest| !rest.starts_with(|c: char| c.is_alphanumeric()))
    })
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
/// generated trees and anything the ignore rules hide — this walk is
/// uncapped and every hit gets parsed and followed through its
/// re-export chain, so an ignored generated tree here is both wasted
/// work and a vote in the public-surface classification.
fn find_all_entrypoints(root: &Path, filter: &DirFilter) -> Vec<PathBuf> {
    fn walk(dir: &Path, filter: &DirFilter, out: &mut Vec<PathBuf>) {
        let Ok(read_dir) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if filter.excludes(&path, file_type.is_dir()) {
                continue;
            }
            if file_type.is_dir() {
                let name = entry.file_name();
                if !crate::fs_util::should_skip_dir(&name.to_string_lossy()) {
                    walk(&path, filter, out);
                }
            } else if file_type.is_file() && is_entrypoint_file(&path) {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, filter, &mut out);
    out.sort();
    out.dedup();
    out
}

/// Project's public surface — every TS/JS file transitively reachable
/// from an entrypoint via re-export chains. Canonicalized.
fn compute_public_surface(ctx: &WalkCtx) -> PublicSurface {
    let mut entrypoints = find_all_entrypoints(ctx.root(), ctx.dir_filter());
    entrypoints.extend(declared_package_entry_sources(ctx.root()));
    // The declared API contract is entrypoint-named-adjacent (`types`
    // field) but its `.d.ts` extension escapes the stem match above.
    entrypoints.extend(
        ctx.typescript_state()
            .declared_api_contract_entrypoint(ctx.root())
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
    // The contract is published surface wherever it sits, but a nested
    // one does not seed the walk (see
    // `declared_api_contract_entrypoint`) — it joins the surface as a
    // leaf.
    if let Some(contract) = ctx.typescript_state().declared_api_contract(ctx.root()) {
        surface.files.insert(contract.clone());
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
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    fn parse(source: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        parser.parse(source, None).unwrap()
    }

    fn export_infos_for_path<'a>(file: &Path, tree: &'a Tree, source: &str) -> Vec<ExportInfo<'a>> {
        let src_lines: Vec<&str> = source.lines().collect();
        let reexports = collect_local_reexports(tree, source);
        let commonjs_reexports = collect_commonjs_value_reexports(tree, source);
        let default_identifier_reexports = collect_default_implementation_exports(tree, source);
        find_export_starts(
            file,
            tree,
            source,
            &src_lines,
            &reexports,
            &commonjs_reexports,
            &default_identifier_reexports,
        )
    }

    fn module_items_for(source: &str, tree: &Tree) -> Vec<ModuleItemInfo> {
        let src_lines: Vec<&str> = source.lines().collect();
        find_module_items(
            tree,
            source,
            &src_lines,
            &HashSet::new(),
            &HashSet::new(),
            commonjs_published_local_name(tree, source),
        )
    }

    const PUBLISHED_CONSTRUCTOR_WITH_TABLE: &str = "\
function Cli () {
  this.cliOpts = [
    {
      name: 'socketPath',
      alias: 's'
    },
    {
      name: 'host',
      alias: 'H'
    },
    {
      name: 'port',
      alias: 'P'
    },
    {
      name: 'theme',
      alias: 't'
    }
  ]
}

module.exports = new Cli()
";

    #[test]
    fn walker_typescript_commonjs_published_constructor_is_not_module_private() {
        let tree = parse(PUBLISHED_CONSTRUCTOR_WITH_TABLE);
        let items = module_items_for(PUBLISHED_CONSTRUCTOR_WITH_TABLE, &tree);
        let published = items.iter().find(|item| item.start_line == 1).unwrap();
        assert!(
            published.is_published,
            "`module.exports = new Cli()` publishes the declaration it constructs"
        );
    }

    #[test]
    fn walker_typescript_literal_roster_names_each_element_of_a_body_table() {
        let tree = parse(PUBLISHED_CONSTRUCTOR_WITH_TABLE);
        let items = module_items_for(PUBLISHED_CONSTRUCTOR_WITH_TABLE, &tree);
        let (roster, element_count) = items
            .iter()
            .find(|item| item.start_line == 1)
            .unwrap()
            .literal_roster
            .clone()
            .expect("a constructor-bound table has a key roster");
        assert_eq!(element_count, 4);
        assert_eq!(
            roster.full,
            vec![2, 4, 8, 12, 16],
            "the binding row plus each element's first property row — never its brace row"
        );
    }

    #[test]
    fn walker_typescript_literal_roster_skips_a_literal_its_surface_renders() {
        let source = "\
const LEVELS = {
  info: 0,
  warn: 1,
  error: 2,
  fatal: 3,
};

module.exports = LEVELS;
";
        let tree = parse(source);
        let items = module_items_for(source, &tree);
        let published = items.iter().find(|item| item.start_line == 1).unwrap();
        assert!(
            published.literal_roster.is_none(),
            "a const's surface already renders its whole literal; a roster would duplicate it"
        );
    }

    /// Receiver names a member-defining statement resolves against, for
    /// a JS source parsed as a module.
    fn member_definition_receiver_lines(source: &str) -> Vec<usize> {
        let tree = parse(source);
        let receivers = collect_exported_receivers(&tree, source);
        collect_member_defining_statements(&tree, source, &receivers)
            .into_iter()
            .map(|stmt| stmt.start_line)
            .collect()
    }

    #[test]
    fn walker_typescript_member_definition_matches_the_exported_receiver() {
        let source = "\
export const api = {};

values.forEach(function (name) {
  api[name] = () => name;
});
";
        assert_eq!(
            member_definition_receiver_lines(source),
            vec![3],
            "a callback installing onto the exported receiver is the shape this recalls"
        );
    }

    #[test]
    fn walker_typescript_member_definition_rejects_a_shadowing_parameter() {
        let source = "\
export const api = {};

values.forEach(function (api) {
  api.local = 1;
});
";
        assert!(
            member_definition_receiver_lines(source).is_empty(),
            "the parameter shadows the module-scope receiver, so the assignment \
             mutates a local object and is not public surface"
        );
    }

    #[test]
    fn walker_typescript_member_definition_rejects_a_shadow_bound_in_a_nested_function() {
        let source = "\
export const api = {};

register(function () {
  function inner(api) {
    api.local = 1;
  }
  return inner;
});
";
        assert!(
            member_definition_receiver_lines(source).is_empty(),
            "a name bound anywhere under the callback cannot be relied on to mean \
             the module-scope receiver at the assignment site"
        );
    }

    #[test]
    fn walker_typescript_member_definition_rejects_a_shadowing_local_declaration() {
        let source = "\
export const api = {};

register(function () {
  const { api } = getContext();
  api.local = 1;
});
";
        assert!(
            member_definition_receiver_lines(source).is_empty(),
            "a destructured local shadows the receiver just as a parameter does"
        );
    }

    #[test]
    fn walker_typescript_star_reexport_rides_with_the_export_roster() {
        let entry = "\
import { css } from './languages';
import './features';

export * from './editor';
export { css };
";
        let tree = parse(entry);
        assert_eq!(
            roster_star_reexport_lines(&tree, entry),
            vec![4],
            "a star says the surface is larger than the `export {{ … }}` beside it, \
             so it belongs on the roster rather than in the plumbing batch"
        );
        assert_eq!(
            collect_imports(&tree, entry).full,
            vec![1, 2, 4],
            "the imports batch still claims it until the caller subtracts the roster lines"
        );

        let wall = "\
export * from './a';
export * from './b';
export const VERSION = '1';
";
        let wall_tree = parse(wall);
        assert!(
            roster_star_reexport_lines(&wall_tree, wall).is_empty(),
            "in a re-export wall the stars *are* the content and are already \
             priced as a roster — hoisting them would swamp the names surface"
        );

        let named = "import a from './a';\n\nexport { b } from './b';\nexport const C = 1;\n";
        let named_tree = parse(named);
        assert!(
            roster_star_reexport_lines(&named_tree, named).is_empty(),
            "a named re-export enumerates what it publishes, so it does not \
             contradict the roster's completeness"
        );
    }

    #[test]
    fn walker_typescript_reexport_tail_claims_what_the_prologue_left_behind() {
        let src = "\
import {Ky} from './core/Ky.js';

const ky = createInstance();

export default ky;

export type {KyInstance} from './types/ky.js';
export {
\tHTTPError,
\tTimeoutError,
} from './errors/index.js';
";
        let tree = parse(src);
        let imports = collect_imports(&tree, src);
        assert_eq!(
            imports.full,
            vec![1],
            "the prologue still stops at the first implementation statement"
        );
        let claimed: HashSet<usize> = imports.full.iter().copied().collect();
        assert_eq!(
            collect_reexport_tail(&tree, src, &claimed).full,
            vec![7, 8, 9, 10, 11]
        );
    }

    #[test]
    fn walker_typescript_reexport_tail_never_double_claims_the_prologue() {
        // A file whose re-exports are all inside the prologue leaves the
        // tail empty — `Imports` already renders them.
        let src = "\
import a from './a.js';
export {b} from './b.js';
export * from './c.js';

const local = 1;
";
        let tree = parse(src);
        let imports = collect_imports(&tree, src);
        assert_eq!(imports.full, vec![1, 2, 3]);
        let mut claimed: HashSet<usize> = imports.full.iter().copied().collect();
        claimed.extend(roster_star_reexport_lines(&tree, src));
        assert!(collect_reexport_tail(&tree, src, &claimed).full.is_empty());
    }

    #[test]
    fn walker_typescript_disavowing_jsdoc_tags_are_the_only_ones_hoisted() {
        let src = "\
/**
 * Old thing.
 *
 * @deprecated Use `next` instead.
 * @public
 */
export function old() {}
";
        let tree = parse(src);
        let src_lines: Vec<&str> = src.lines().collect();
        let root = tree.root_node();
        let mut cursor = root.walk();
        let export = root
            .children(&mut cursor)
            .find(|child| child.kind() == "export_statement")
            .expect("export statement");
        assert_eq!(
            disavowal_doc_lines(export, src, &src_lines, false),
            vec![4],
            "only the tag that contradicts the roster's claim moves; the rest of \
             the block keeps competing as documentation"
        );

        assert!(is_disavowal_jsdoc_line("/** @internal */"));
        assert!(is_disavowal_jsdoc_line(" * @private"));
        assert!(
            !is_disavowal_jsdoc_line(" * @privateRemarks notes for maintainers"),
            "tag matching must respect word boundaries"
        );
        assert!(!is_disavowal_jsdoc_line(" * @public"));
    }

    fn export_infos<'a>(tree: &'a Tree, source: &str) -> Vec<ExportInfo<'a>> {
        export_infos_for_path(Path::new("fixture.ts"), tree, source)
    }

    /// Lines for a single export's decl. For interface/type/class/enum, the
    /// whole item. For function, the signature plus a body-elision marker.
    /// For const/let, the assignment line(s) — truncated at the inner
    /// function body's `{` when the initializer is a fn-init (direct
    /// arrow/function or wrapped through `forwardRef(props => {...})` etc.),
    /// otherwise the whole declaration.
    fn collect_export_lines(tree: &Tree, source: &str, start_line: usize) -> FileLines {
        let exports = export_infos(tree, source);
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
    fn walker_typescript_manifest_entry_seeds_nested_public_surface() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let lib = root.join("lib");
        std::fs::create_dir(&lib).unwrap();
        std::fs::write(
            root.join("package.json"),
            r#"{"exports":{".":{"import":"./lib/package-node.js"}}}"#,
        )
        .unwrap();
        let entry = lib.join("package-node.js");
        let core = lib.join("core.js");
        std::fs::write(&entry, "export * from './core.js';\n").unwrap();
        std::fs::write(&core, "export const run = () => {};\n").unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let surface = compute_public_surface(&ctx);
        for file in [&entry, &core] {
            let canonical = file.canonicalize().unwrap();
            assert!(
                surface.files.contains(&canonical),
                "manifest entry and its re-export target must be public: {}",
                file.display()
            );
        }
        assert!(
            surface
                .reexport_targets
                .contains(&core.canonicalize().unwrap())
        );
    }

    #[test]
    fn walker_typescript_declaration_contract_never_seeds_the_surface_walk() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let typings = root.join("typings");
        std::fs::create_dir(&typings).unwrap();
        // The modern conditional-exports shape: the contract is named
        // both by `types` and by the `types` condition under `.`.
        std::fs::write(
            root.join("package.json"),
            r#"{"types":"./typings/index.d.ts","main":"./index.js","exports":{".":{"require":{"types":"./typings/index.d.ts","default":"./index.js"},"default":"./index.js"}}}"#,
        )
        .unwrap();
        std::fs::write(root.join("index.js"), "export const run = () => {};\n").unwrap();
        let contract = typings.join("index.d.ts");
        let plumbing = typings.join("internal.js");
        std::fs::write(&contract, "export * from './internal.js';\n").unwrap();
        std::fs::write(&plumbing, "export const helper = () => {};\n").unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let surface = compute_public_surface(&ctx);
        assert!(
            surface.files.contains(&contract.canonicalize().unwrap()),
            "the declared contract is published surface wherever it sits"
        );
        let plumbing = plumbing.canonicalize().unwrap();
        assert!(
            !surface.files.contains(&plumbing),
            "a declaration file is never a runtime entry seed — its imports are type plumbing"
        );
        assert!(!surface.reexport_targets.contains(&plumbing));
    }

    #[test]
    fn walker_typescript_flat_layout_published_dirs_are_primary_source() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("package.json"),
            r#"{"main":"index.js","files":["classes/","dist","index.js"]}"#,
        )
        .unwrap();
        for sub in ["classes", "dist", "test"] {
            std::fs::create_dir(root.join(sub)).unwrap();
            std::fs::write(root.join(sub).join("a.js"), "module.exports = 1;\n").unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        assert!(is_flat_layout_source_path(&root.join("classes/a.js"), &ctx));
        assert!(
            !is_flat_layout_source_path(&root.join("test/a.js"), &ctx),
            "unpublished siblings stay secondary"
        );
        assert!(
            !is_flat_layout_source_path(&root.join("dist/a.js"), &ctx),
            "published build output is not source"
        );
    }

    #[test]
    fn walker_typescript_flat_layout_tier_is_js_only() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("package.json"),
            r#"{"main":"index.js","files":["classes"]}"#,
        )
        .unwrap();
        std::fs::create_dir(root.join("classes")).unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        for ext in ["ts", "tsx", "mts", "cts"] {
            let file = root.join("classes").join(format!("a.{ext}"));
            assert_eq!(
                js_value_factor(&file, &ctx),
                1.0,
                "the flat-layout tier is a JS tier; TypeScript keeps full weight: {}",
                file.display()
            );
        }
        assert_eq!(
            js_value_factor(&root.join("classes/a.js"), &ctx),
            FLAT_LAYOUT_JS_VALUE_FACTOR
        );
    }

    #[test]
    fn walker_typescript_flat_layout_honors_files_negations() {
        for (files, promoted) in [
            (r#"["classes","!classes"]"#, false),
            (r#"["!classes","classes"]"#, false),
            (r#"["classes","!classes/fixtures/**"]"#, false),
            (r#"["!classes/fixtures/**","classes"]"#, false),
            (r#"["classes","!.DS_Store"]"#, true),
        ] {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path();
            std::fs::write(
                root.join("package.json"),
                format!(r#"{{"main":"index.js","files":{files}}}"#),
            )
            .unwrap();
            std::fs::create_dir(root.join("classes")).unwrap();
            let ctx = WalkCtx::new(root.to_path_buf());
            assert_eq!(
                is_flat_layout_source_path(&root.join("classes/a.js"), &ctx),
                promoted,
                "files {files}"
            );
        }
    }

    #[test]
    fn walker_typescript_source_wrapper_suppresses_flat_layout_promotion() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("package.json"),
            r#"{"main":"index.js","files":["widgets","lib"]}"#,
        )
        .unwrap();
        for sub in ["widgets", "lib"] {
            std::fs::create_dir(root.join(sub)).unwrap();
            std::fs::write(root.join(sub).join("a.js"), "module.exports = 1;\n").unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());
        assert!(
            !is_flat_layout_source_path(&root.join("widgets/a.js"), &ctx),
            "a package with a source-dir wrapper keeps the wrapper as its source tree"
        );
    }

    #[test]
    fn walker_typescript_manifest_entry_escaping_targets_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let outside = dir.path().join("outside.js");
        std::fs::write(&outside, "export const secret = 1;\n").unwrap();
        let root = dir.path().join("repo");
        std::fs::create_dir(&root).unwrap();
        std::fs::write(
            root.join("package.json"),
            format!(
                r#"{{"main":"../outside.js","module":"{}","exports":{{".":"../outside.js"}}}}"#,
                outside.display()
            ),
        )
        .unwrap();
        assert!(
            declared_package_entry_sources(&root).is_empty(),
            "absolute and parent-traversing manifest targets must be ignored"
        );
    }

    #[test]
    fn walker_typescript_mixin_intersection_alias_surfaces_its_own_members() {
        // `Base & Other & { … }` declares its members in the trailing
        // object literal; the named operands are declared elsewhere.
        let source = "\
type Props = Children &
  DivProps & {
    label?: string
    progress?: number
  }
";
        let tree = parse(source);
        let alias = tree.root_node().child(0).unwrap();
        assert_eq!(
            member_surface_spans(ItemKind::TypeAlias, alias, source),
            vec![(3, 3), (4, 4)]
        );
    }

    #[test]
    fn walker_typescript_module_private_type_declarations_are_module_items() {
        // Every module-private declaration is roster material; nothing
        // about a declaration's *name* decides whether it exists.
        let source = "\
type ItemProps = { disabled?: boolean }
type Store = { emit: () => void }
export function run(): void {}
";
        let tree = parse(source);
        let items = find_module_items(
            &tree,
            source,
            &source.lines().collect::<Vec<_>>(),
            &HashSet::from([3]),
            &HashSet::new(),
            None,
        );
        assert_eq!(
            items.iter().map(|item| item.start_line).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn walker_typescript_oversize_js_class_gets_member_catalog() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut src = String::from("export class Command {\n");
        for index in 0..=JS_CLASS_MEMBER_SPLIT_MAX {
            src.push_str(&format!("  method_{index}() {{ return {index}; }}\n"));
        }
        src.push_str("}\n");
        let file = root.join("catalog.js");
        std::fs::write(&file, src).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 10_000, None);
        let report = scheduler.run_with_report();
        let export_key = BatchKey::Typescript(TsKey::Export {
            file: file.clone(),
            start_line: 1,
        });
        let catalog_key = BatchKey::Typescript(TsKey::ExportMemberNames {
            file: file.clone(),
            start_line: 1,
        });
        let catalog = report
            .candidates
            .iter()
            .find(|b| b.key == catalog_key)
            .expect("expected unified member-name catalog");
        assert_eq!(catalog.predecessor.as_ref(), Some(&export_key));
    }

    #[test]
    fn walker_typescript_large_member_catalog_stays_unified() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut src = String::from("export class Command {\n");
        for index in 0..80 {
            src.push_str(&format!(
                "  configure_with_a_descriptive_public_method_name_{index}() {{ return {index}; }}\n"
            ));
        }
        src.push_str("}\n");
        let file = root.join("catalog.js");
        std::fs::write(&file, src).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 100_000, None);
        let report = scheduler.run_with_report();
        let catalog_key = BatchKey::Typescript(TsKey::ExportMemberNames {
            file: file.clone(),
            start_line: 1,
        });
        let catalogs: Vec<_> = report
            .candidates
            .iter()
            .filter(|batch| batch.key == catalog_key)
            .collect();
        assert_eq!(catalogs.len(), 1, "a member roster is a single batch");

        let BatchContent::Lines { spans } = &catalogs[0].content else {
            panic!("member catalog must contain lines");
        };
        let mut covered = HashSet::new();
        for span in spans {
            for line in span.start..=span.end {
                assert!(covered.insert(line), "roster overlap at line {line}");
            }
        }
        assert_eq!(covered.len(), 80, "the roster must cover every member");
    }

    #[test]
    fn walker_typescript_unaffordable_member_catalog_does_not_abandon_the_budget() {
        // The scheduler stops on the first top-ranked batch that does
        // not fit, and only a seed listing degrades to a prefix. An
        // exact batch larger than the whole budget therefore strands
        // every token behind it, so a machine-generated roster must not
        // be allowed to become one.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut generated = String::from("export interface GeneratedSchema {\n");
        for field in 0..4000 {
            generated.push_str(&format!(
                "  descriptive_generated_field_name_{field}: string;\n"
            ));
        }
        generated.push_str("}\n");
        std::fs::write(root.join("generated.ts"), generated).unwrap();
        // Independent, cheap, lower-ranked content in its own file: it
        // shares no predecessor with the roster, so the only thing that
        // can keep it out of the schedule is the run being abandoned.
        for module in 0..6 {
            std::fs::write(
                root.join(format!("helper_{module}.ts")),
                format!(
                    "/** Formats a {module}-style value for display. */\nexport function format_helper_{module}(value: string): string {{\n  return value;\n}}\n"
                ),
            )
            .unwrap();
        }

        let budget = 3000;
        let report = Scheduler::new(root.to_path_buf(), FsWalker, budget, None).run_with_report();
        let rendered = report.tree.render();
        let scheduled_helpers = (0..6)
            .filter(|module| rendered.contains(&format!("format_helper_{module}")))
            .count();
        assert!(
            scheduled_helpers > 0,
            "independent helper content must still schedule behind an oversized roster"
        );
        let used = report.tree.total_tokens();
        assert!(
            used * 2 > budget,
            "the run abandoned the budget: {used} of {budget} tokens used"
        );

        // The roster was sliced, the slices chain, and each one is
        // bounded by the ceiling plus the single member row that
        // crossed it.
        let generated = root.join("generated.ts");
        assert!(
            !report.candidates.iter().any(|batch| matches!(
                &batch.key,
                BatchKey::Typescript(TsKey::ExportMemberNames { file, .. }) if *file == generated
            )),
            "an over-ceiling roster must not also be emitted whole"
        );
        let ctx = WalkCtx::new(root.to_path_buf());
        let mut slices: Vec<_> = report
            .candidates
            .iter()
            .filter_map(|batch| match &batch.key {
                BatchKey::Typescript(TsKey::ExportMemberNamesChunk {
                    file, chunk_index, ..
                }) if *file == generated => Some((*chunk_index, batch)),
                _ => None,
            })
            .collect();
        slices.sort_by_key(|(chunk_index, _)| *chunk_index);
        assert!(slices.len() > 1, "the roster must be sliced");
        for (chunk_index, batch) in &slices {
            let cost = ctx.marginal_tokens(&batch.content);
            assert!(
                cost <= MEMBER_CATALOG_MAX_TOKENS + 64,
                "slice {chunk_index} costs {cost}, past the ceiling plus one member row"
            );
            let expected = match chunk_index.checked_sub(1) {
                Some(previous) => BatchKey::Typescript(TsKey::ExportMemberNamesChunk {
                    file: generated.clone(),
                    start_line: 1,
                    chunk_index: previous,
                }),
                None => BatchKey::Typescript(TsKey::Export {
                    file: generated.clone(),
                    start_line: 1,
                }),
            };
            assert_eq!(
                batch.predecessor.as_ref(),
                Some(&expected),
                "slice {chunk_index} must chain onto its predecessor"
            );
        }
    }

    /// A documented class big enough to split its surface into chunks:
    /// every method carries a doc block whose second paragraph is a
    /// worked example.
    fn documented_oversize_class() -> String {
        let mut source = String::from(
            "export class Large {\n  field: string = 'value';\n  constructor() {\n    this.field = 'constructed';\n  }\n",
        );
        for method in 0..30 {
            source.push_str(&format!(
                "  /**\n   * Runs step {method}.\n   *\n   * @example\n   * large.method_{method}();\n   */\n  method_{method}(argument: {{ first: string; second: number; third: boolean }}): string {{\n    return this.field + argument.first;\n  }}\n"
            ));
        }
        source.push_str("}\n");
        source
    }

    /// `(member_start_line, predecessor)` of every member-doc batch a
    /// single-file directory produces.
    fn member_doc_batches(name: &str, source: &str) -> Vec<(usize, Option<BatchKey>)> {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(name), source).unwrap();
        let report =
            Scheduler::new(dir.path().to_path_buf(), FsWalker, 100_000, None).run_with_report();
        report
            .candidates
            .iter()
            .filter_map(|batch| match &batch.key {
                BatchKey::Typescript(TsKey::ExportMemberDoc {
                    member_start_line, ..
                }) => Some((*member_start_line, batch.predecessor.clone())),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn walker_typescript_member_doc_rides_the_last_chunk_of_a_split_surface() {
        // Every member's doc gates on the final tail, not on the chunk
        // holding its own signature: a chunked surface is one reading of
        // the declaration, and letting an early chunk's cheap docs
        // outrank a later chunk inverts the gate.
        let source = documented_oversize_class();
        let docs = member_doc_batches("fixture.ts", &source);
        assert_eq!(docs.len(), 30, "one doc batch per documented method");
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("fixture.ts");
        std::fs::write(&file, &source).unwrap();
        let tree = parse(&source);
        let exports = export_infos_for_path(&file, &tree, &source);
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let chunks =
            oversized_export_class_chunks(&file, exports.first().unwrap(), &source, &ctx).unwrap();
        assert!(chunks.len() > 2, "surface splits into head + several tails");
        for (member_start_line, predecessor) in docs {
            let Some(BatchKey::Typescript(TsKey::ExportTail { chunk_index, .. })) = predecessor
            else {
                panic!("member {member_start_line} doc must ride a tail chunk");
            };
            assert_eq!(chunk_index, chunks.len() - 2, "the last tail");
        }
    }

    #[test]
    fn walker_typescript_member_doc_elides_the_worked_example() {
        let source = "\
export class Small {
  /**
   * States the contract.
   *
   * @example
   * small.run();
   */
  run(): void {}
}
";
        let tree = parse(source);
        let src_lines: Vec<&str> = source.lines().collect();
        let exports = export_infos_for_path(Path::new("fixture.ts"), &tree, source);
        let members = documented_member_nodes(exports.first().unwrap(), source);
        let lines = member_doc_lines(members[0], source, &src_lines).expect("documented member");
        assert_eq!(
            lines.full,
            vec![2, 3],
            "the prose, and not the signature row"
        );
        assert_eq!(lines.ellipses, vec![5], "the example is marked, not spent");
    }

    /// Rows of the member-doc batch for the sole documented member of
    /// a one-class file, as `(full, ellipses)`.
    fn sole_member_doc_rows(source: &str) -> (Vec<usize>, Vec<usize>) {
        let tree = parse(source);
        let src_lines: Vec<&str> = source.lines().collect();
        let exports = export_infos_for_path(Path::new("fixture.ts"), &tree, source);
        let members = documented_member_nodes(exports.first().unwrap(), source);
        let lines = member_doc_lines(*members.last().unwrap(), source, &src_lines)
            .expect("the documented member");
        (lines.full, lines.ellipses)
    }

    #[test]
    fn walker_typescript_member_doc_reaches_past_a_decorator() {
        // The grammar seats a decorator as its own class-body sibling
        // ahead of the member, so a doc block is not the member's
        // immediate previous sibling in any annotated codebase.
        let source = "\
export class Controller {
  /** Lists the items. */
  @Get('/items')
  listItems(): void {}
}
";
        assert_eq!(sole_member_doc_rows(source), (vec![2], Vec::new()));
    }

    #[test]
    fn walker_typescript_member_doc_reaches_past_stacked_decorators() {
        let source = "\
export class Controller {
  /** Lists the items. */
  @Get('/items')
  @UseGuards(AuthGuard)
  @ApiResponse({ status: 200 })
  listItems(): void {}
}
";
        assert_eq!(sole_member_doc_rows(source), (vec![2], Vec::new()));
    }

    #[test]
    fn walker_typescript_member_doc_reaches_past_a_directive_above_decorators() {
        let source = "\
export class Controller {
  /** Lists the items. */
  // eslint-disable-next-line @typescript-eslint/no-unsafe-call
  @Get('/items')
  listItems(): void {}
}
";
        assert_eq!(sole_member_doc_rows(source), (vec![2], Vec::new()));
    }

    #[test]
    fn walker_typescript_member_doc_stops_at_a_preceding_members_trailing_comment() {
        // An ordinary comment that is not a directive belongs to what
        // precedes it; crossing it would attach an unrelated member's
        // doc block to this one.
        let source = "\
export class Controller {
  /** Lists the items. */
  listItems(): void {}

  // Everything below is internal bookkeeping.
  refresh(): void {}
}
";
        let tree = parse(source);
        let src_lines: Vec<&str> = source.lines().collect();
        let exports = export_infos_for_path(Path::new("fixture.ts"), &tree, source);
        let members = documented_member_nodes(exports.first().unwrap(), source);
        assert!(
            member_doc_lines(*members.last().unwrap(), source, &src_lines).is_none(),
            "the second member is undocumented"
        );
    }

    #[test]
    fn walker_typescript_member_doc_rides_the_unified_catalog() {
        // However large the roster, it is one batch, so every
        // documented member's doc rides that single surface — the one
        // batch that renders its signature row.
        let mut source =
            String::from("export function run(): void {}\n\nexport interface Options {\n");
        for field in 0..40 {
            source.push_str(&format!(
                "  /** Controls facet {field}. */\n  facet_{field}: {{ first: string; second: number; third: boolean }};\n"
            ));
        }
        source.push_str("}\n");
        let docs = member_doc_batches("fixture.ts", &source);
        assert!(!docs.is_empty(), "documented fields get doc batches");
        for (member_start_line, predecessor) in &docs {
            assert!(
                matches!(
                    predecessor,
                    Some(BatchKey::Typescript(TsKey::ExportMemberNames {
                        start_line: 3,
                        ..
                    }))
                ),
                "member {member_start_line} doc must ride the unified catalog, got {predecessor:?}"
            );
        }
    }

    #[test]
    fn walker_typescript_member_doc_skips_a_type_only_module() {
        // The surface a doc would hang off is damped as type machinery
        // and enters late or never, so a doc slice under it is credit
        // nobody can collect.
        let source = "\
export interface Contract {
  /** Runs the step. */
  run(): void;
}
";
        assert!(member_doc_batches("contract.ts", source).is_empty());
    }

    #[test]
    fn walker_typescript_oversized_export_class_gets_chained_member_tails() {
        let mut source = String::from(
            "export class Large {\n  field: string = 'value';\n  constructor() {\n    this.field = 'constructed';\n  }\n",
        );
        for method in 0..30 {
            source.push_str(&format!(
                "  method_{method}(argument: {{ first: string; second: number; third: boolean }}): string {{\n    return this.field + argument.first;\n  }}\n"
            ));
        }
        source.push_str("}\n");

        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("fixture.ts");
        std::fs::write(&file, &source).unwrap();
        let tree = parse(&source);
        let exports = export_infos_for_path(&file, &tree, &source);
        let item = exports.first().unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let chunks = oversized_export_class_chunks(&file, item, &source, &ctx)
            .expect("expected oversized class chunks");
        assert!(chunks.len() > 1);
        let covered_full: Vec<_> = chunks
            .iter()
            .flat_map(|chunk| chunk.full.iter().copied())
            .collect();
        let covered_ellipses: Vec<_> = chunks
            .iter()
            .flat_map(|chunk| chunk.ellipses.iter().copied())
            .collect();
        let original = decl_surface_lines(item.kind, item.anchor, item.decl, &source, true);
        assert_eq!(covered_full, original.full);
        assert_eq!(covered_ellipses, original.ellipses);

        let report =
            Scheduler::new(dir.path().to_path_buf(), FsWalker, 100_000, None).run_with_report();
        let export_key = BatchKey::Typescript(TsKey::Export {
            file: file.clone(),
            start_line: 1,
        });
        let tails: Vec<_> = report
            .candidates
            .iter()
            .filter(|batch| matches!(batch.key, BatchKey::Typescript(TsKey::ExportTail { .. })))
            .collect();
        assert_eq!(tails.len(), chunks.len() - 1);
        let mut predecessor = export_key;
        for (chunk_index, tail) in tails.iter().enumerate() {
            assert_eq!(tail.predecessor.as_ref(), Some(&predecessor));
            predecessor = BatchKey::Typescript(TsKey::ExportTail {
                file: file.clone(),
                start_line: 1,
                chunk_index,
            });
        }
    }

    #[test]
    fn walker_typescript_oversized_class_body_is_bounded_and_value_conserved() {
        let mut source = String::from("export class Large {\n");
        for method in 0..60 {
            source.push_str(&format!(
                "  method_{method}(input: string): string {{\n    const value = input + 'a deliberately long semantic payload for chunk-cost coverage';\n    return value + 'with enough source mass to require several bounded class body chunks';\n  }}\n"
            ));
        }
        source.push_str("}\n");

        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("fixture.ts");
        std::fs::write(&file, &source).unwrap();
        let tree = parse(&source);
        let exports = export_infos_for_path(&file, &tree, &source);
        let item = exports.first().unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let chunks = bounded_class_body_parts(&file, &source, item.body_parts.clone(), &ctx);

        assert!(chunks.len() > 1, "expected an oversized body to split");
        for chunk in &chunks {
            let content =
                single_file_lines_content(&file, &source, FileLines::new(chunk.part.lines.clone()))
                    .unwrap();
            assert!(ctx.marginal_tokens(&content) <= CLASS_BODY_CHUNK_MAX_TOKENS);
        }
        let total_factor: f64 = chunks.iter().map(|chunk| chunk.value_factor).sum();
        assert!((total_factor - 1.0).abs() < 1e-9);
        let covered: Vec<_> = chunks
            .iter()
            .flat_map(|chunk| chunk.part.lines.iter().copied())
            .collect();
        let original: Vec<_> = item
            .body_parts
            .iter()
            .flat_map(|part| part.lines.iter().copied())
            .collect();
        assert_eq!(covered, original);

        let report =
            Scheduler::new(dir.path().to_path_buf(), FsWalker, 100_000, None).run_with_report();
        let body_batches: Vec<_> = report
            .candidates
            .iter()
            .filter(|batch| {
                matches!(
                    batch.key,
                    BatchKey::Typescript(TsKey::ExportBody { start_line: 1, .. })
                )
            })
            .collect();
        assert_eq!(body_batches.len(), chunks.len());
        for pair in body_batches.windows(2) {
            assert_eq!(pair[1].predecessor.as_ref(), Some(&pair[0].key));
        }
    }

    #[test]
    fn walker_typescript_affordable_export_class_stays_whole() {
        let source = "export class Small {\n  field = 1;\n  constructor() {}\n  method() { return this.field; }\n}\n";
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("fixture.ts");
        std::fs::write(&file, source).unwrap();
        let tree = parse(source);
        let exports = export_infos_for_path(&file, &tree, source);
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        assert!(
            oversized_export_class_chunks(&file, exports.first().unwrap(), source, &ctx,).is_none()
        );
    }

    #[test]
    fn walker_typescript_importless_entrypoint_does_not_starve_siblings() {
        // A nested module dir whose index.ts has no import / re-export
        // lines used to gate every sibling batch on an `Imports` key
        // that was never emitted, leaving the siblings permanently
        // ineligible at any budget.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let util = root.join("src").join("util");
        std::fs::create_dir_all(&util).unwrap();
        std::fs::write(root.join("package.json"), "{\"name\":\"repro\"}\n").unwrap();
        std::fs::write(
            root.join("src").join("index.ts"),
            "export * from \"./util\";\n",
        )
        .unwrap();
        std::fs::write(util.join("index.ts"), "export const ANSWER = 42;\n").unwrap();
        std::fs::write(
            util.join("helper.ts"),
            "export function helperAlpha(): number {\n  return 1;\n}\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 50_000, None);
        let report = scheduler.run_with_report();
        let keys: std::collections::HashSet<_> =
            report.candidates.iter().map(|b| b.key.clone()).collect();
        for batch in &report.candidates {
            if let Some(pred) = &batch.predecessor {
                assert!(
                    keys.contains(pred),
                    "dangling predecessor {pred:?} on {:?}",
                    batch.key
                );
            }
        }
        let rendered = report.tree.render();
        assert!(rendered.contains("helperAlpha"), "rendered:\n{rendered}");
    }

    #[test]
    fn walker_typescript_reexport_tail_is_the_surface_when_nothing_else_is_exported() {
        // Implementation plus a trailing re-export block and no local
        // exports at all: the block is the file's entire public surface,
        // so it must render even though there is no roster to gate it.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let src = root.join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(root.join("package.json"), "{\"name\":\"repro\"}\n").unwrap();
        std::fs::write(
            src.join("index.ts"),
            "\
import {register} from './registry.js';

const instance = register();
instance.start();

export {PublicAlpha} from './alpha.js';
export {PublicBeta} from './beta.js';
",
        )
        .unwrap();
        std::fs::write(
            src.join("alpha.ts"),
            "export function PublicAlpha(): number {\n  return 1;\n}\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 50_000, None);
        let report = scheduler.run_with_report();
        let keys: std::collections::HashSet<_> =
            report.candidates.iter().map(|b| b.key.clone()).collect();
        for batch in &report.candidates {
            if let Some(pred) = &batch.predecessor {
                assert!(
                    keys.contains(pred),
                    "dangling predecessor {pred:?} on {:?}",
                    batch.key
                );
            }
        }
        let rendered = report.tree.render();
        assert!(rendered.contains("PublicBeta"), "rendered:\n{rendered}");
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
        let exports = export_infos_for_path(Path::new("fixture.js"), &tree, src);
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
        let exports = export_infos_for_path(Path::new("fixture.js"), &tree, src);
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
        let exports = export_infos_for_path(Path::new("fixture.js"), &tree, src);
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
        let exports = export_infos_for_path(Path::new("fixture.js"), &tree, src);
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
        let exports = export_infos_for_path(Path::new("fixture.js"), &tree, src);
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
    fn walker_typescript_collect_local_reexports_splits_value_from_type_only() {
        let src = "\
const X = () => { return 1; };
const Y = () => { return 2; };
const Z = () => { return 3; };
const W = () => { return 4; };
export type { X };
export { type Y };
export { Z };
export { W as Renamed };
export { A } from './a';
";
        let tree = parse(src);
        let names = collect_local_reexports(&tree, src);
        assert!(
            !names.value.contains("X"),
            "type-only stmt-level should not be a value export"
        );
        assert!(
            !names.value.contains("Y"),
            "inline type modifier should not be a value export"
        );
        assert!(names.value.contains("Z"));
        assert!(names.value.contains("W"));
        assert!(names.type_only.contains("X"));
        assert!(names.type_only.contains("Y"));
        assert!(
            !names.value.contains("A") && !names.type_only.contains("A"),
            "a clause with a `from` source is plumbing, not a local re-export"
        );
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
    fn walker_typescript_synthetic_export_covers_named_declarations() {
        // A local `export { … }` clause naming a function / class / type
        // declaration synthesizes the Export at the declaration, not just
        // the one-line NamedReexport clause.
        let src = "\
export { createNote, Store, NoteId };

async function createNote(opts) {
  return opts;
}

class Store {
  get(id) { return id; }
}

type NoteId = string;
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        let kind_at = |line: usize| {
            exports
                .iter()
                .find(|e| e.start_line == line)
                .map(|e| e.kind)
        };
        assert_eq!(kind_at(3), Some(ItemKind::Function));
        assert_eq!(kind_at(7), Some(ItemKind::Class));
        assert_eq!(kind_at(11), Some(ItemKind::TypeAlias));
        assert!(
            exports
                .iter()
                .find(|e| e.start_line == 11)
                .is_some_and(|e| e.is_type_only),
            "a synthesized type declaration must still count as type-only \
             so the type-machinery damp sees it"
        );
    }

    #[test]
    fn walker_typescript_synthetic_export_covers_type_only_clauses() {
        // `export type { … }` and `export { type … }` are the idiomatic
        // spellings for publishing a type; both must reach the type
        // declarations they name.
        let src = "\
export type { Options, Handler };
export { type Result, run };

interface Options {
  strict: boolean;
}

type Handler = (input: string) => void;

type Result = {ok: boolean};

function run() {
  return 1;
}
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        let kind_at = |line: usize| {
            exports
                .iter()
                .find(|e| e.start_line == line)
                .map(|e| e.kind)
        };
        assert_eq!(kind_at(4), Some(ItemKind::Interface));
        assert_eq!(kind_at(8), Some(ItemKind::TypeAlias));
        assert_eq!(kind_at(10), Some(ItemKind::TypeAlias));
        assert_eq!(
            kind_at(12),
            Some(ItemKind::Function),
            "a value specifier in a mixed clause still publishes its runtime declaration"
        );
        for line in [4, 8, 10] {
            assert!(
                exports
                    .iter()
                    .find(|e| e.start_line == line)
                    .is_some_and(|e| e.is_type_only),
                "line {line} is a type declaration"
            );
        }
    }

    #[test]
    fn walker_typescript_synthetic_export_type_clause_withholds_runtime_decls() {
        // A type-only clause deliberately withholds the runtime binding,
        // so it must not promote a const / function / class / enum.
        let src = "\
export type { Value, helper, Widget };

const Value = () => { return 1; };

function helper() {
  return 2;
}

class Widget {
}
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(
            exports.iter().all(|e| e.start_line == 1),
            "only the clause itself is an export: {:?}",
            exports.iter().map(|e| e.start_line).collect::<Vec<_>>()
        );
    }

    #[test]
    fn walker_typescript_synthetic_export_sees_through_ambient_declarations() {
        // `.d.ts` spells nearly everything as `declare …`, which
        // tree-sitter wraps in an `ambient_declaration`. The synthesis
        // has to unwrap it or a declaration file's whole re-exported
        // surface is invisible.
        let src = "\
export {parse, Parser, Options};

declare function parse(input: string): Options;

declare class Parser {
}

declare const VERSION: string;

type Options = {strict: boolean};
";
        let tree = parse(src);
        let exports = export_infos_for_path(Path::new("index.d.ts"), &tree, src);
        let kind_at = |line: usize| {
            exports
                .iter()
                .find(|e| e.start_line == line)
                .map(|e| e.kind)
        };
        assert_eq!(kind_at(3), Some(ItemKind::Function));
        assert_eq!(kind_at(5), Some(ItemKind::Class));
        assert_eq!(kind_at(10), Some(ItemKind::TypeAlias));
        assert!(
            exports
                .iter()
                .filter(|e| e.start_line != 1)
                .all(|e| e.is_type_only),
            "an ambient declaration carries no runtime value"
        );
        assert_eq!(
            kind_at(8),
            None,
            "`declare const` has no fn initializer, so the const rule still declines it"
        );
    }

    #[test]
    fn walker_typescript_synthetic_export_needs_the_reexport_name() {
        // An unmentioned sibling declaration stays module-private.
        let src = "\
export { kept };

function kept() { return 1; }

function dropped() { return 2; }
";
        let tree = parse(src);
        let exports = export_infos(&tree, src);
        assert!(exports.iter().any(|e| e.start_line == 3));
        assert!(exports.iter().all(|e| e.start_line != 5));
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
    fn walker_typescript_esm_function_reexport_synthesizes_local_functions() {
        // Every name in the clause is public surface, so each declaration
        // gets its own Export alongside the one-line clause.
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
        assert_eq!(
            exports
                .iter()
                .map(|e| (e.start_line, e.kind))
                .collect::<Vec<_>>(),
            vec![
                (1, ItemKind::Function),
                (4, ItemKind::Function),
                (7, ItemKind::NamedReexport),
            ]
        );
    }

    #[test]
    fn walker_typescript_default_direct_call_synthesizes_local_function() {
        let src = "\
function createInstance() {
  return {};
}
export default createInstance();
";
        let tree = parse(src);
        let exports = export_infos_for_path(Path::new("index.ts"), &tree, src);
        let local = exports.iter().find(|e| e.start_line == 1).unwrap();
        assert!(matches!(local.kind, ItemKind::Function));
        assert_eq!(local.predecessor_start_line, Some(4));
        assert!(local.body_parts.iter().any(|part| part.lines == vec![2]));
    }

    #[test]
    fn walker_typescript_default_new_initializer_synthesizes_local_class() {
        let src = "\
class Client {
  run() {
    return 1;
  }
}
const instance = new Client();
export default instance;
";
        let tree = parse(src);
        let exports = export_infos_for_path(Path::new("index.ts"), &tree, src);
        let local = exports.iter().find(|e| e.start_line == 1).unwrap();
        assert!(matches!(local.kind, ItemKind::Class));
        assert_eq!(local.predecessor_start_line, Some(7));
        assert!(local.body_parts.iter().any(|part| part.lines == vec![3]));
    }

    #[test]
    fn walker_typescript_default_data_const_synthesizes_the_declaration() {
        let src = "\
const config = {
  retries: 3,
};
export default config;
";
        let tree = parse(src);
        let exports = export_infos_for_path(Path::new("source/options.ts"), &tree, src);
        let local = exports.iter().find(|e| e.start_line == 1).unwrap();
        assert!(matches!(local.kind, ItemKind::Const));
        assert_eq!(local.predecessor_start_line, Some(4));
    }

    #[test]
    fn walker_typescript_default_primitive_const_synthesizes_the_declaration() {
        let src = "\
const VERSION = '1.2.3';
export default VERSION;
";
        let tree = parse(src);
        let exports = export_infos_for_path(Path::new("source/version.ts"), &tree, src);
        let local = exports.iter().find(|e| e.start_line == 1).unwrap();
        assert!(matches!(local.kind, ItemKind::Const));
        assert_eq!(local.predecessor_start_line, Some(2));
    }

    #[test]
    fn walker_typescript_default_enum_synthesizes_the_declaration() {
        let src = "\
enum Level {
  Info,
  Warn,
}
export default Level;
";
        let tree = parse(src);
        let exports = export_infos_for_path(Path::new("source/level.ts"), &tree, src);
        let local = exports.iter().find(|e| e.start_line == 1).unwrap();
        assert_eq!(local.predecessor_start_line, Some(5));
    }

    #[test]
    fn walker_typescript_default_identifier_alias_synthesizes_the_named_declaration() {
        let src = "\
const base = {
  retries: 3,
};
const config = base;
export default config;
";
        let tree = parse(src);
        let exports = export_infos_for_path(Path::new("source/options.ts"), &tree, src);
        let local = exports.iter().find(|e| e.start_line == 4).unwrap();
        assert!(matches!(local.kind, ItemKind::Const));
        assert_eq!(local.predecessor_start_line, Some(5));
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
    fn walker_typescript_commonjs_instance_export_names_its_constructor() {
        // `module.exports = new Cli()` exports the instance, so `Cli` is
        // the receiver whose prototype methods are the public surface.
        let src = "\
function Cli () {
}
Cli.prototype.showVersion = function () {
  return 1;
};
Cli.prototype.showUsage = function () {
  return 2;
};
Cli.prototype.cliParse = function () {
  return 3;
};
module.exports = new Cli()
";
        let tree = parse(src);
        assert!(collect_module_exports_receivers(&tree, src).contains("Cli"));
        let exports = export_infos_for_path(Path::new("cli.js"), &tree, src);
        let method_lines: Vec<usize> = exports
            .iter()
            .filter(|e| matches!(e.kind, ItemKind::Function))
            .map(|e| e.start_line)
            .collect();
        assert_eq!(method_lines, vec![3, 6, 9]);
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
