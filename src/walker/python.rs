//! Python walker. Per-file orientation (imports + module docstring +
//! `__all__` + dunder assignments) plus per-decl batches for top-level
//! classes, defs (sync / async), and module-level non-dunder
//! assignments. Per-method batches inside top-level classes mirror
//! Rust's per-item granularity so NS targets like
//! `pluggy._hooks.HookCaller._add_hookimpl` body land naturally.
//!
//! Per-file keys:
//! - [`PythonKey::Imports`]: `import` / `from … import …` directives
//!   plus the module docstring and module-level dunder assignments
//!   (`__all__`, `__version__`, `__author__`).
//! - [`PythonKey::DeclNames`]: surface listing of every top-level
//!   class, def, and module-level non-dunder simple-assignment first
//!   line — catastrophic-omission hedge.
//! - [`PythonKey::MethodSigs`]: surface listing of every method's inner
//!   `def` line across every top-level class. Decorator rows are owned by
//!   the per-method batch. `@overload` stubs are excluded, as in
//!   [`PythonKey::DeclNames`] — an overload stack is one method, and the
//!   return annotation that discriminates its variants is not on the
//!   `def` line.
//! - [`PythonKey::TestNames`]: in `test_*.py` / `*_test.py` only,
//!   surface listing of `def test_*` first lines.
//! - [`PythonKey::SetupManifest`]: in `setup.py` only, the top-level
//!   `setup(...)` call that carries legacy package metadata.
//!
//! Per-decl keys (keyed by start line):
//! - [`PythonKey::Decl`]: one top-level class / def / non-dunder
//!   constant. For decorated forms, the span starts at the
//!   `@decorator` row.
//! - [`PythonKey::DeclDoc`]: top-level def / class docstring.
//! - [`PythonKey::DeclBody`]: top-level def body slices, sans
//!   leading docstring.
//! - [`PythonKey::ClassBody`]: top-level class body excluding methods
//!   and the leading docstring — TypedDict / dataclass / Pydantic
//!   fields, `__slots__`, class constants.
//!
//! Per-method keys (keyed by the method's start line):
//! - [`PythonKey::Method`] / [`PythonKey::MethodDoc`] /
//!   [`PythonKey::MethodBody`]: same shape as the per-decl trio, with
//!   bodies split into top-level statement slices.
//!
//! Visibility: emits everything. A `visibility_factor` discount
//! (1.0 unprefixed, 0.6 leading-`_`, 1.0 dunder) ranks public-by-PEP-8
//! names above leading-`_` "internal" ones rather than hard-filtering
//! — NSes anchor on intentionally-private names
//! (`pluggy._callers._multicall`, `pluggy._hooks.HookCaller._add_hookimpl`).
//!
//! Parse trees are cached in [`WalkCtx`].

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, PythonKey};
use crate::value::{
    CATALOG_ROSTER_CONCAVITY_EXPONENT, DEFAULT_CONCAVITY_EXPONENT, ROSTER_MASS_FACTOR_CAP,
    conserved_catalog_chunk_factors, depth_factor, mix_signals, reexport_import_chunk_factor,
};

use super::import_chunks::{
    ImportGroup, REEXPORT_IMPORT_MAX_OTHER_LINES, REEXPORT_IMPORT_MAX_OTHER_STATEMENTS,
    groups_to_file_lines, node_line_count, push_import_group, should_chunk_import_groups,
};
use super::{
    BodyPart, FileLines, WalkCtx, body_part_value_factor, budget_chunk_ranges, dedup_sorted,
    extend_nonblank_rows, extend_span, file_depth_factor, file_lines_covered_by,
    first_child_of_kind, fs::files_with_extension, name_of, push_rows, signature_end_row,
    single_file_lines_content, statement_block_parts,
};

const VISIBILITY_PUBLIC: f64 = 1.0;
const VISIBILITY_UNDERSCORE: f64 = 0.6;

/// Neutral roster size for Python decl/method/field surfaces: surfaces
/// this big are priced as-is, bigger ones are lifted and smaller ones
/// demoted, because a roster's value is flat while its cost grows with
/// entry count. Calibrated: a baseline of 2 lifts nearly every
/// multi-decl file and floods orientation content (README sections,
/// re-export walls) out of the early budget; 8 is measured flat at the
/// primary budget (better at 2080, worse at 3000).
const PYTHON_ROSTER_MASS_BASELINE: f64 = 6.0;

/// Preserve the historically winning unified names surface until it is
/// too large to remain purchasable in the early budget window. Oversize
/// catalogs are split near the target, and a final chunk below half the
/// target folds back into its predecessor rather than becoming a
/// trailing crumb — the last chunk gates the whole per-decl train, so a
/// 30-token crumb gate costs a purchase for nothing.
///
/// What the split delivers at ≤10K is the *head* chunk, not the tail:
/// no `DeclNamesChunk` row is scheduled at any budget ≤10K in the
/// corpus, so the live effect is that an oversize roster reaches the
/// frontier as a cheaper head slice repriced by
/// [`conserved_catalog_chunk_factors`]. That is not the same as inert:
/// unifying every roster regardless of size measures −0.0007 at 3K
/// (tomli −0.053), so the target is a live knob.
const DECL_NAMES_CHUNK_TARGET_TOKENS: usize = 450;
const DECL_NAMES_TINY_TAIL_TOKENS: usize = DECL_NAMES_CHUNK_TARGET_TOKENS / 2;

/// Two-sided, unlike the shared [`crate::value::roster_mass_factor`]:
/// the boost-only form prices a one-decl module's 11-token roster at the
/// same size-invariant value as a 40-decl module's 600-token one, so the
/// early budget goes on enumerating trinket modules one name at a time
/// while a flagship module's roster — the predecessor of every per-decl
/// batch in the file — never clears the frontier and the whole file
/// contributes nothing. Demoting under-baseline rosters is what makes
/// the factor actually roster-size-neutral in the direction that
/// decides the early-budget race.
///
/// `__init__.py` rosters are exempt: the entrypoint depth pin already
/// privileges them, and re-pricing on top floods nested-package
/// `__init__` surfaces ahead of the re-export walls NS authors rank.
fn python_roster_mass_factor(file: &Path, entries: usize) -> f64 {
    if is_python_entrypoint(file) {
        return 1.0;
    }
    (entries.max(1) as f64 / PYTHON_ROSTER_MASS_BASELINE)
        .powf(DEFAULT_CONCAVITY_EXPONENT)
        .min(ROSTER_MASS_FACTOR_CAP)
}

/// Per-run Python walker state: spine-module sets cached per package
/// root (computed once from the root `__init__.py`'s re-exports).
#[derive(Default)]
pub(in crate::walker) struct PythonState {
    spine_modules: RefCell<HashMap<PathBuf, Arc<HashSet<PathBuf>>>>,
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let py_files = files_with_extension(dir, "py", ctx);
    if py_files.is_empty() {
        return Vec::new();
    }
    let (test_files, source_files): (Vec<_>, Vec<_>) =
        py_files.into_iter().partition(|p| is_test_file(p));
    let mut out = Vec::new();
    out.extend(expand_test_files(&test_files, ctx));
    out.extend(expand_source_files(&source_files, ctx));
    out
}

fn expand_test_files(test_files: &[PathBuf], ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in test_files {
        let Some((source, tree)) = parse_python(ctx, file) else {
            continue;
        };
        let starts = collect_test_function_starts(&tree, &source);
        if starts.is_empty() {
            continue;
        }
        let ellipses = starts.iter().map(|&l| l + 1).collect();
        let lines = FileLines::new(starts).with_ellipses(ellipses);
        let Some(content) = single_file_lines_content(file, &source, lines) else {
            continue;
        };
        out.push(Batch {
            key: PythonKey::TestNames { file: file.clone() }.into(),
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
        let Some((source, tree)) = parse_python(ctx, file) else {
            continue;
        };
        let src_lines: Vec<&str> = source.lines().collect();
        let decls = find_top_level_decls(&tree, &source);

        if let Some(chunks) = collect_reexport_import_chunks(file, &tree, &source) {
            let chunk_count = chunks.len();
            for (chunk_index, lines) in chunks.into_iter().enumerate() {
                let Some(content) = single_file_lines_content(file, &source, lines) else {
                    continue;
                };
                out.push(Batch {
                    key: PythonKey::ImportChunk {
                        file: file.clone(),
                        chunk_index,
                    }
                    .into(),
                    predecessor: None,
                    content,
                    value: imports_chunk_value(file, ctx, chunk_index, chunk_count),
                });
            }
        } else if let Some(content) =
            single_file_lines_content(file, &source, collect_imports(&tree, &source))
        {
            out.push(Batch {
                key: PythonKey::Imports { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: imports_value(file, ctx),
            });
        }

        if let Some((start_line, lines)) = collect_setup_manifest(file, &tree, &source)
            && let Some(content) = single_file_lines_content(file, &source, lines)
        {
            out.push(Batch {
                key: PythonKey::SetupManifest {
                    file: file.clone(),
                    start_line,
                }
                .into(),
                predecessor: None,
                content,
                value: setup_manifest_value(file, ctx),
            });
        }

        if decls.is_empty() {
            continue;
        }

        let methods_by_class = collect_methods_by_class(&decls, &source);
        // The same exclusion [`names_roster`] applies to top-level
        // defs, for the same reason: an overload stack is one method,
        // and what discriminates its variants is the return annotation
        // — which a roster row, being the `def` line alone, cannot
        // show. Listing every stub therefore overstates the method
        // count and, when the signature wraps, repeats a bare
        // `def name(` that implies distinctions the roster has hidden.
        // The implementation def's row stands for the stack; each stub
        // still gets its own batch, where the full signature is
        // visible.
        let collapsed_stub_lines: HashSet<usize> = methods_by_class
            .iter()
            .flat_map(|(_, methods)| {
                methods
                    .iter()
                    .filter(|m| collapsed_overload_stub(m, methods, &source))
                    .map(signature_line)
            })
            .collect();
        let flat_methods: Vec<(usize, DeclInfo)> = methods_by_class
            .into_iter()
            .flat_map(|(class_index, methods)| methods.into_iter().map(move |m| (class_index, m)))
            .collect();
        let all_name_lines = collect_all_name_lines(&decls, &flat_methods);

        // Keep the historically winning unified names surface unless its
        // rendered cost exceeds the chunk target; continuations are
        // chained so they do not become independently schedulable crumbs.
        // Roster-mass pricing remains based on the complete catalog, and
        // the catalog's value is a conserved total allocated across the
        // chunks.
        let roster = names_roster(&decls, &source);
        let roster_decls: Vec<_> = roster.iter().map(|&i| decls[i]).collect();
        let names_lines = collect_decl_names_from(&roster_decls, &all_name_lines);
        let names_chunk_ranges =
            decl_names_chunk_ranges(file, ctx, &source, &roster_decls, &all_name_lines);
        let names_base_value =
            decl_names_value(file, ctx) * python_roster_mass_factor(file, roster.len());
        let mut chunk_contents = Vec::with_capacity(names_chunk_ranges.len());
        for (chunk_index, range) in names_chunk_ranges.iter().enumerate() {
            let chunk_decls = &roster_decls[range.clone()];
            let chunk_lines = collect_decl_names_from(chunk_decls, &all_name_lines);
            if let Some(content) = single_file_lines_content(file, &source, chunk_lines) {
                chunk_contents.push((chunk_index, content));
            }
        }
        let chunk_factors = if chunk_contents.len() > 1 {
            let costs: Vec<usize> = chunk_contents
                .iter()
                .map(|(_, content)| ctx.marginal_tokens(content))
                .collect();
            conserved_catalog_chunk_factors(&costs, CATALOG_ROSTER_CONCAVITY_EXPONENT)
        } else {
            vec![1.0; chunk_contents.len()]
        };
        let mut names_keys = Vec::with_capacity(chunk_contents.len());
        for ((chunk_index, content), chunk_factor) in chunk_contents.into_iter().zip(chunk_factors)
        {
            let key = BatchKey::Python(if chunk_index == 0 {
                PythonKey::DeclNames { file: file.clone() }
            } else {
                PythonKey::DeclNamesChunk {
                    file: file.clone(),
                    chunk_index,
                }
            });
            let predecessor = names_keys.last().cloned();
            out.push(Batch {
                key: key.clone(),
                predecessor,
                content,
                value: names_base_value * chunk_factor,
            });
            names_keys.push(key);
        }
        let names_gate = names_keys.last().cloned();

        // Method-signature catalog — likewise one unified batch. Gated
        // on the names surface: the `Full+Ellipsis` pair can share the
        // class header's following ellipsis row, so gating keeps overlap
        // ancestry local.
        let mut method_sigs_gate: Option<BatchKey> = None;
        {
            let full: Vec<_> = flat_methods
                .iter()
                .map(|(_, m)| signature_line(m))
                .filter(|line| !collapsed_stub_lines.contains(line))
                .collect();
            let ellipses: Vec<_> = full
                .iter()
                .map(|line| *line + 1)
                .filter(|line| !all_name_lines.contains(line))
                .collect();
            let lines = FileLines::new(full).with_ellipses(ellipses);
            if let Some(content) = single_file_lines_content(file, &source, lines) {
                let key = BatchKey::Python(PythonKey::MethodSigs { file: file.clone() });
                out.push(Batch {
                    key: key.clone(),
                    predecessor: names_gate.clone(),
                    content,
                    value: method_sigs_value(file, ctx)
                        * python_roster_mass_factor(file, flat_methods.len()),
                });
                method_sigs_gate = Some(key);
            }
        }

        for decl in decls.iter() {
            let decl_key = PythonKey::Decl {
                file: file.clone(),
                start_line: decl.start_line,
            };
            let decl_lines = collect_decl(decl);
            if (!matches!(decl.kind, DeclKind::Const)
                || !file_lines_covered_by(&decl_lines, &names_lines))
                && let Some(content) = single_file_lines_content(file, &source, decl_lines)
            {
                out.push(Batch {
                    key: decl_key.clone().into(),
                    predecessor: names_gate.clone(),
                    content,
                    value: decl_value(file, decl, ctx),
                });
            }
            let decl_predecessor = BatchKey::Python(decl_key);

            let (doc_lede, doc_rest) =
                split_doc_lede(collect_doc_for(decl.inner_node, &source), &src_lines);
            if let Some(content) = single_file_lines_content(file, &source, doc_lede) {
                let doc_key = PythonKey::DeclDoc {
                    file: file.clone(),
                    start_line: decl.start_line,
                };
                out.push(Batch {
                    key: doc_key.clone().into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: decl_doc_value(file, decl, ctx),
                });
                if let Some(rest_lines) = doc_rest
                    && let Some(content) = single_file_lines_content(file, &source, rest_lines)
                {
                    out.push(Batch {
                        key: PythonKey::DeclDocRest {
                            file: file.clone(),
                            start_line: decl.start_line,
                        }
                        .into(),
                        predecessor: Some(BatchKey::Python(doc_key)),
                        content,
                        value: decl_doc_value(file, decl, ctx) * DOC_REST_VALUE_FACTOR,
                    });
                }
            }

            match decl.kind {
                DeclKind::Function => {
                    let parts = def_body_parts(decl.inner_node, &src_lines);
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
                            key: PythonKey::DeclBody {
                                file: file.clone(),
                                start_line: decl.start_line,
                                body_start_line,
                            }
                            .into(),
                            predecessor: Some(decl_predecessor.clone()),
                            value: decl_body_value(file, decl, ctx) * part_value_factor,
                            content,
                        });
                    }
                }
                DeclKind::Class => {
                    let class_body_lines = collect_class_body(decl.inner_node, &src_lines);
                    let field_row_count = class_body_lines.full.len();
                    if let Some(content) =
                        single_file_lines_content(file, &source, class_body_lines)
                    {
                        out.push(Batch {
                            key: PythonKey::ClassBody {
                                file: file.clone(),
                                start_line: decl.start_line,
                            }
                            .into(),
                            predecessor: Some(decl_predecessor.clone()),
                            content,
                            value: class_body_value(file, decl, ctx)
                                * python_roster_mass_factor(file, field_row_count),
                        });
                    }
                    out.extend(emit_methods(
                        file,
                        ctx,
                        &source,
                        &src_lines,
                        decl,
                        method_sigs_gate.as_ref(),
                        &decl_predecessor,
                    ));
                }
                DeclKind::Const => {}
            }
        }
    }
    out
}

fn collect_setup_manifest(file: &Path, tree: &Tree, source: &str) -> Option<(usize, FileLines)> {
    if file.file_name().and_then(|n| n.to_str()) != Some("setup.py") {
        return None;
    }
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() != "expression_statement" {
            continue;
        }
        let Some(call) = first_child_of_kind(child, "call", true) else {
            continue;
        };
        let Some(function) = call.child_by_field_name("function") else {
            continue;
        };
        if function.kind() != "identifier"
            || source[function.start_byte()..function.end_byte()].trim() != "setup"
        {
            continue;
        }
        if function.start_position().row != call.start_position().row {
            continue;
        }
        let lines = setup_install_requires_lines(call, source)?;
        return Some((lines.full[0], lines));
    }
    None
}

fn setup_install_requires_lines(call: Node, source: &str) -> Option<FileLines> {
    let args = first_child_of_kind(call, "argument_list", true)?;
    let mut cursor = args.walk();
    for child in args.children(&mut cursor) {
        if child.kind() != "keyword_argument" {
            continue;
        }
        let Some(name) = keyword_argument_name(child, source) else {
            continue;
        };
        if name != "install_requires" {
            continue;
        }
        let start = child.start_position().row + 1;
        let end = child.end_position().row + 1;
        return Some(FileLines::new((start..=end).collect()));
    }
    None
}

fn keyword_argument_name<'a>(keyword: Node<'a>, source: &'a str) -> Option<&'a str> {
    let name = keyword
        .child_by_field_name("name")
        .or_else(|| first_child_of_kind(keyword, "identifier", true))?;
    Some(source[name.start_byte()..name.end_byte()].trim())
}

fn emit_methods(
    file: &Path,
    ctx: &WalkCtx,
    source: &str,
    src_lines: &[&str],
    class_decl: &DeclInfo,
    method_sigs_gate: Option<&BatchKey>,
    decl_predecessor: &BatchKey,
) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for method in collect_methods_in_class(class_decl, source) {
        let method_key = PythonKey::Method {
            file: file.to_path_buf(),
            start_line: method.start_line,
        };
        if let Some(content) = single_file_lines_content(file, source, collect_method_decl(&method))
        {
            out.push(Batch {
                key: method_key.clone().into(),
                predecessor: Some(method_sigs_gate.unwrap_or(decl_predecessor).clone()),
                content,
                value: method_value(file, &method, ctx),
            });
        }
        let method_predecessor = BatchKey::Python(method_key);

        if let Some(content) =
            single_file_lines_content(file, source, collect_doc_for(method.inner_node, source))
        {
            out.push(Batch {
                key: PythonKey::MethodDoc {
                    file: file.to_path_buf(),
                    start_line: method.start_line,
                }
                .into(),
                predecessor: Some(method_predecessor.clone()),
                content,
                value: method_doc_value(file, &method, ctx),
            });
        }

        let parts = def_body_parts(method.inner_node, src_lines);
        let part_value_factor = body_part_value_factor(parts.len());
        for part in parts {
            let Some(body_start_line) = part.start_line() else {
                continue;
            };
            let Some(content) = single_file_lines_content(file, source, FileLines::new(part.lines))
            else {
                continue;
            };
            out.push(Batch {
                key: PythonKey::MethodBody {
                    file: file.to_path_buf(),
                    start_line: method.start_line,
                    body_start_line,
                }
                .into(),
                predecessor: Some(method_predecessor.clone()),
                value: method_body_value(file, &method, ctx) * part_value_factor,
                content,
            });
        }
    }
    out
}

// --- decl model ---------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclKind {
    Class,
    Function,
    Const,
}

impl DeclKind {
    fn kind_weight(self) -> f64 {
        match self {
            DeclKind::Class => 1.10,
            DeclKind::Function => 1.00,
            DeclKind::Const => 0.85,
        }
    }
}

/// One top-level item or method. `unit_node` wraps decorators;
/// `inner_node` is the underlying def/class/expression_statement.
#[derive(Debug, Clone, Copy)]
struct DeclInfo<'a> {
    kind: DeclKind,
    unit_node: Node<'a>,
    inner_node: Node<'a>,
    start_line: usize,
    underscore_private: bool,
}

impl<'a> DeclInfo<'a> {
    fn visibility_factor(&self) -> f64 {
        if self.underscore_private {
            VISIBILITY_UNDERSCORE
        } else {
            VISIBILITY_PUBLIC
        }
    }
}

/// Returns `(unit_node, inner_node)` for a `function_definition` /
/// `class_definition`, unwrapping `decorated_definition` if present.
fn function_or_decorated<'a>(node: Node<'a>) -> Option<(Node<'a>, Node<'a>)> {
    match node.kind() {
        "function_definition" | "class_definition" => Some((node, node)),
        "decorated_definition" => {
            // The wrapped def/class is the "definition" field.
            let inner = node.child_by_field_name("definition")?;
            if matches!(inner.kind(), "function_definition" | "class_definition") {
                Some((node, inner))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn find_top_level_decls<'a>(tree: &'a Tree, source: &str) -> Vec<DeclInfo<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if let Some((unit, inner)) = function_or_decorated(child) {
            out.push(decl_from_def(unit, inner, source));
        } else if child.kind() == "expression_statement"
            && let Some(target) = const_assignment_target(child, source)
            && !is_dunder(target)
        {
            out.push(DeclInfo {
                kind: DeclKind::Const,
                unit_node: child,
                inner_node: child,
                start_line: child.start_position().row + 1,
                underscore_private: is_underscore_private(target),
            });
        }
    }
    out
}

/// Methods of one class (direct body children, decorator-unwrapped).
fn collect_methods_in_class<'a>(class_decl: &DeclInfo<'a>, source: &str) -> Vec<DeclInfo<'a>> {
    let Some(body) = class_decl.inner_node.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    let mut out = Vec::new();
    for child in body.children(&mut cursor) {
        let Some((unit, inner)) = function_or_decorated(child) else {
            continue;
        };
        if inner.kind() != "function_definition" {
            continue;
        }
        out.push(decl_from_def(unit, inner, source));
    }
    out
}

fn decl_from_def<'a>(unit: Node<'a>, inner: Node<'a>, source: &str) -> DeclInfo<'a> {
    let kind = match inner.kind() {
        "class_definition" => DeclKind::Class,
        _ => DeclKind::Function,
    };
    let name = name_of(inner, source).unwrap_or("");
    DeclInfo {
        kind,
        unit_node: unit,
        inner_node: inner,
        start_line: unit.start_position().row + 1,
        underscore_private: is_underscore_private(name),
    }
}

fn is_underscore_private(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    if is_dunder(name) {
        return false;
    }
    name.starts_with('_')
}

fn is_dunder(name: &str) -> bool {
    name.starts_with("__") && name.ends_with("__") && name.len() >= 4
}

/// Target name of a simple `NAME = …` or `NAME: TYPE = …`. Rejects
/// multi-target, tuple, and unpack assignments.
fn const_assignment_target<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let mut cursor = node.walk();
    let inner = node
        .children(&mut cursor)
        .find(|c| matches!(c.kind(), "assignment"))?;
    let left = inner.child_by_field_name("left")?;
    match left.kind() {
        "identifier" => Some(&source[left.start_byte()..left.end_byte()]),
        _ => None,
    }
}

// --- collectors ---------------------------------------------------------

fn collect_imports(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    let mut first_real_statement_seen = false;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "import_statement" | "import_from_statement" | "future_import_statement" => {
                extend_span(&mut lines, child, source);
                first_real_statement_seen = true;
            }
            "expression_statement" => {
                // Module docstring: only the very first statement in the
                // file qualifies. A post-assignment `"""…"""` "variable
                // docstring" elsewhere in the module is not a module
                // docstring and stays out of `Imports`.
                if !first_real_statement_seen && is_docstring_statement(child) {
                    extend_span(&mut lines, child, source);
                } else if let Some(target) = const_assignment_target(child, source)
                    && is_dunder(target)
                {
                    extend_span(&mut lines, child, source);
                }
                first_real_statement_seen = true;
            }
            "if_statement" | "try_statement" => {
                first_real_statement_seen = true;
            }
            "comment" => {
                // Leading-comment block before any real statement: shebang
                // (`#!/usr/bin/env python`), file-level directive (`# ruff:
                // noqa`, `# type: ignore`), or a brief "what this module
                // is" header. Same shape as the module docstring slot —
                // module-prelude context that orients the file alongside
                // imports / `__all__`. Trailing comments between imports
                // stay out.
                if !first_real_statement_seen {
                    extend_span(&mut lines, child, source);
                }
            }
            _ => {
                first_real_statement_seen = true;
            }
        }
    }
    FileLines::new(dedup_sorted(lines))
}

fn collect_reexport_import_chunks(
    file: &Path,
    tree: &Tree,
    source: &str,
) -> Option<Vec<FileLines>> {
    // `__init__.py` is the canonical Python package export surface. Other
    // aggregator-shaped modules keep ordinary import behavior to avoid
    // over-classifying implementation files as package entrypoints.
    if !is_init_py(file) {
        return None;
    }
    let mut groups = collect_import_groups(tree, source)?;
    if !should_chunk_import_groups(&groups) {
        return None;
    }
    // `__all__` is the package's explicit public-API declaration — strictly
    // more orienting than the individual `from .mod import Name` groups that
    // feed it. Float it to the front so it lands at the lowest chunk index
    // (best `reexport_import_chunk_factor`) instead of dead last — but keep a
    // leading module docstring (`__doc__`) ahead of it, since the docstring is
    // itself prime orientation.
    if let Some(pos) = groups.iter().position(|group| group.source == "__all__") {
        let all_group = groups.remove(pos);
        let insert_at = usize::from(groups.first().is_some_and(|g| g.source == "__doc__"));
        groups.insert(insert_at, all_group);
    }
    Some(groups_to_file_lines(groups))
}

fn collect_import_groups(tree: &Tree, source: &str) -> Option<Vec<ImportGroup>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut groups = Vec::new();
    let mut first_real_statement_seen = false;
    let mut other_statements = 0usize;
    let mut other_lines = 0usize;
    // Tail tolerance: once we leave the contiguous imports prefix, allow the
    // tail to be carried as a top-level Decl batch (handled elsewhere) and
    // stop collecting import groups. Common case: `def __getattr__` shims
    // for deprecated-name handling at the end of an __init__.py. The prefix
    // itself is still a re-export wall.
    let mut imports_prefix_ended = false;
    for child in root.children(&mut cursor) {
        match child.kind() {
            "import_statement" | "import_from_statement" | "future_import_statement" => {
                if imports_prefix_ended {
                    return None;
                }
                push_import_group(
                    &mut groups,
                    import_source_key(child, source),
                    true,
                    child,
                    source,
                );
                first_real_statement_seen = true;
            }
            "expression_statement" => {
                if !first_real_statement_seen && is_docstring_statement(child) {
                    push_import_group(&mut groups, "__doc__".to_string(), false, child, source);
                } else if let Some(target) = const_assignment_target(child, source)
                    && is_dunder(target)
                {
                    if imports_prefix_ended {
                        return None;
                    }
                    push_import_group(&mut groups, target.to_string(), false, child, source);
                } else if !tolerate_reexport_wall_other(
                    child,
                    &mut other_statements,
                    &mut other_lines,
                ) {
                    // Significant tail content — stop collecting but keep
                    // the imports prefix we've already seen.
                    imports_prefix_ended = true;
                }
                first_real_statement_seen = true;
            }
            "if_statement" | "try_statement" => {
                imports_prefix_ended = true;
                first_real_statement_seen = true;
            }
            "comment" => {}
            _ if !tolerate_reexport_wall_other(child, &mut other_statements, &mut other_lines) => {
                imports_prefix_ended = true;
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

fn import_source_key(node: Node, source: &str) -> String {
    let text = source[node.start_byte()..node.end_byte()].trim();
    if node.kind() == "import_from_statement"
        && let Some(rest) = text.strip_prefix("from ")
        && let Some((module, _)) = rest.split_once(" import")
    {
        return module.trim().to_string();
    }
    if let Some(rest) = text.strip_prefix("import ") {
        let first = rest.lines().next().unwrap_or(rest).trim();
        if !first.contains(',') {
            return first
                .split(" as ")
                .next()
                .unwrap_or(first)
                .trim()
                .to_string();
        }
    }
    text.lines().next().unwrap_or(text).trim().to_string()
}

/// Surface listing of every top-level decl's first line. Class/def
/// entries emit Full + Ellipsis (body-elision marker); consts emit
/// Full only to avoid overlapping the next statement.
/// Indices of `decls` in names-surface order: classes and functions
/// first (file order), module consts trailing — a const/TypeVar wall
/// at the top of the file must not occupy chunk 0 and push the real
/// API into falloff-damped tail chunks. `@overload` stubs whose
/// implementation is in the file are excluded: their names row would
/// render the bare `@overload` decorator line, so the stack is
/// represented by the implementation def's row.
fn names_roster(decls: &[DeclInfo], source: &str) -> Vec<usize> {
    let mut roster: Vec<usize> = (0..decls.len())
        .filter(|&i| {
            matches!(decls[i].kind, DeclKind::Class | DeclKind::Function)
                && !collapsed_overload_stub(&decls[i], decls, source)
        })
        .collect();
    roster.extend((0..decls.len()).filter(|&i| decls[i].kind == DeclKind::Const));
    roster
}

fn collapsed_overload_stub(decl: &DeclInfo, decls: &[DeclInfo], source: &str) -> bool {
    is_overload_stub(decl, source) && overload_implementation(decl, decls, source).is_some()
}

/// Index of the non-stub function sharing the stub's name, if any.
fn overload_implementation(stub: &DeclInfo, decls: &[DeclInfo], source: &str) -> Option<usize> {
    let stub_name = name_of(stub.inner_node, source)?;
    decls.iter().position(|d| {
        d.kind == DeclKind::Function
            && !is_overload_stub(d, source)
            && name_of(d.inner_node, source) == Some(stub_name)
    })
}

/// `@overload` / `@typing.overload` / `@t.overload`-decorated def.
fn is_overload_stub(decl: &DeclInfo, source: &str) -> bool {
    if decl.kind != DeclKind::Function || decl.unit_node.kind() != "decorated_definition" {
        return false;
    }
    let mut cursor = decl.unit_node.walk();
    decl.unit_node.children(&mut cursor).any(|child| {
        child.kind() == "decorator" && {
            let text = source[child.start_byte()..child.end_byte()].trim();
            text == "@overload" || text.ends_with(".overload")
        }
    })
}

fn collect_all_name_lines(
    decls: &[DeclInfo],
    flat_methods: &[(usize, DeclInfo)],
) -> HashSet<usize> {
    decls
        .iter()
        .map(|decl| decl.start_line)
        .chain(
            flat_methods
                .iter()
                .map(|(_, method)| signature_line(method)),
        )
        .collect()
}

fn collect_decl_names_from(decls: &[DeclInfo], all_name_lines: &HashSet<usize>) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for decl in decls {
        full.push(decl.start_line);
        if decl.kind == DeclKind::Function {
            let ellipsis_line = decl.start_line + 1;
            if !all_name_lines.contains(&ellipsis_line) {
                ellipses.push(ellipsis_line);
            }
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Source-roster-order ranges for a Python names surface: greedily cut
/// once a range reaches the target, with a tail below half-target folded
/// into the preceding range. A roster whose whole cost is under the
/// target yields the single range `0..len` — i.e. the established
/// unified surface, untouched.
fn decl_names_chunk_ranges(
    file: &Path,
    ctx: &WalkCtx,
    source: &str,
    decls: &[DeclInfo],
    all_name_lines: &HashSet<usize>,
) -> Vec<Range<usize>> {
    if decls.is_empty() {
        return Vec::new();
    }

    let range_cost = |range: Range<usize>| {
        let lines = collect_decl_names_from(&decls[range], all_name_lines);
        single_file_lines_content(file, source, lines)
            .map(|content| ctx.marginal_tokens(&content))
            .unwrap_or(0)
    };

    budget_chunk_ranges(
        decls.len(),
        range_cost,
        DECL_NAMES_CHUNK_TARGET_TOKENS,
        DECL_NAMES_TINY_TAIL_TOKENS,
        |_| true,
        |_| true,
    )
}

fn collect_methods_by_class<'a>(
    decls: &[DeclInfo<'a>],
    source: &str,
) -> Vec<(usize, Vec<DeclInfo<'a>>)> {
    decls
        .iter()
        .enumerate()
        .filter(|(_, d)| d.kind == DeclKind::Class)
        .map(|(index, d)| (index, collect_methods_in_class(d, source)))
        .filter(|(_, ms)| !ms.is_empty())
        .collect()
}

fn signature_line(info: &DeclInfo) -> usize {
    info.inner_node.start_position().row + 1
}

/// Last row of `inner`'s signature. [`signature_end_row`] reports the
/// row the *body* starts on, which is one past the signature only when
/// the body starts on its own line. A stub written
/// `def f(\n    x,\n) -> T: ...` puts the body on the same row as the
/// closing paren, so stopping one row short there drops the return
/// annotation — precisely what distinguishes one `@overload` variant
/// from the next.
fn signature_last_row(inner: Node) -> usize {
    let before_body = signature_end_row(inner).saturating_sub(1);
    inner
        .child_by_field_name("return_type")
        .or_else(|| inner.child_by_field_name("parameters"))
        .map(|node| node.end_position().row.max(before_body))
        .unwrap_or(before_body)
}

fn collect_decl(info: &DeclInfo) -> FileLines {
    let unit_start = info.unit_node.start_position().row;
    let end_row = match info.kind {
        DeclKind::Function | DeclKind::Class => signature_last_row(info.inner_node).max(unit_start),
        DeclKind::Const => info.unit_node.end_position().row,
    };
    let mut lines = Vec::new();
    push_rows(&mut lines, unit_start, end_row);
    FileLines::new(dedup_sorted(lines))
}

fn collect_method_decl(info: &DeclInfo) -> FileLines {
    let unit_start = info.unit_node.start_position().row;
    let signature_start = info.inner_node.start_position().row;
    let end_row = signature_last_row(info.inner_node).max(signature_start);
    let mut lines = Vec::new();
    push_rows(&mut lines, unit_start, end_row);
    FileLines::new(dedup_sorted(lines))
}

/// Rows of the first docstring in `inner`'s body, or empty if none.
fn collect_doc_for(inner: Node, source: &str) -> FileLines {
    let Some(body) = inner.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let Some(doc_stmt) = first_docstring_statement(body) else {
        return FileLines::new(Vec::new());
    };
    let mut lines = Vec::new();
    extend_span(&mut lines, doc_stmt, source);
    FileLines::new(dedup_sorted(lines))
}

/// Minimum remainder size (lines) for splitting a docstring into lede
/// and rest batches. High on purpose: splitting a doc the scheduler
/// can buy whole converts a complete NS delivery into
/// lede-now/rest-never (quadratically damped partial credit; measured
/// -0.03 on tomli and pluggy at a threshold of 4). Only the docs too
/// fat to ever clear a frontier whole should split.
const DOC_LEDE_SPLIT_MIN_REST_LINES: usize = 20;

/// Value factor for the post-lede remainder of a split docstring —
/// parameter docs and examples rank below the summary paragraph.
const DOC_REST_VALUE_FACTOR: f64 = 0.6;

/// Split a docstring span at its first paragraph break. NS rows quote
/// a long class docstring as lede + detail rows; an all-or-nothing
/// 700-token DeclDoc batch is unbuyable at any frontier. `rest` is
/// `None` when the docstring is one paragraph or the remainder is
/// trivial.
fn split_doc_lede(doc: FileLines, src_lines: &[&str]) -> (FileLines, Option<FileLines>) {
    let blank = |row: &usize| {
        src_lines
            .get(row - 1)
            .is_some_and(|line| line.trim().is_empty())
    };
    let Some(split_position) = doc.full.iter().position(blank) else {
        return (doc, None);
    };
    let rest_rows: Vec<usize> = doc.full[split_position..]
        .iter()
        .copied()
        .skip_while(blank)
        .collect();
    if rest_rows.len() < DOC_LEDE_SPLIT_MIN_REST_LINES {
        return (doc, None);
    }
    let lede_rows = doc.full[..split_position].to_vec();
    (FileLines::new(lede_rows), Some(FileLines::new(rest_rows)))
}

/// The first named child of `body` that's a docstring statement, or
/// `None`. Skips comments.
fn first_docstring_statement<'a>(body: Node<'a>) -> Option<Node<'a>> {
    let mut cursor = body.walk();
    for child in body.named_children(&mut cursor) {
        if child.kind() == "comment" {
            continue;
        }
        if is_docstring_statement(child) {
            return Some(child);
        }
        // First non-comment, non-docstring statement → no docstring.
        return None;
    }
    None
}

/// True iff `node` is a docstring `expression_statement(string)`.
fn is_docstring_statement(node: Node) -> bool {
    if node.kind() != "expression_statement" {
        return false;
    }
    let mut cursor = node.walk();
    let Some(first) = node.named_children(&mut cursor).next() else {
        return false;
    };
    matches!(first.kind(), "string" | "concatenated_string")
}

/// Body parts of a def, split by top-level statement, sans leading
/// docstring + blank lines. When the whole body collapses into one
/// part containing the docstring, the `block_child_parts` fallback
/// splits per-statement regardless of `BODY_SPLIT_MIN_LINES` — so a
/// documented def splits below the threshold an undocumented one
/// respects. Calibrated-in: the threshold-respecting variant is a
/// walker-machinery re-calibration judged not worth the risk.
fn def_body_parts(inner: Node, src_lines: &[&str]) -> Vec<BodyPart> {
    let Some(body) = inner.child_by_field_name("body") else {
        return Vec::new();
    };
    let docstring_rows =
        first_docstring_statement(body).map(|n| (n.start_position().row, n.end_position().row));
    let mut parts = statement_block_parts(Some(body), src_lines, "block");
    if parts.is_empty() {
        return Vec::new();
    }
    if let Some((doc_start, doc_end)) = docstring_rows {
        let doc_start_line = doc_start + 1;
        let doc_end_line = doc_end + 1;
        if parts.len() == 1
            && parts[0]
                .start_line()
                .is_some_and(|line| (doc_start_line..=doc_end_line).contains(&line))
        {
            parts = block_child_parts(body, src_lines);
        }
        parts.retain(|part| {
            !part
                .start_line()
                .is_some_and(|line| (doc_start_line..=doc_end_line).contains(&line))
        });
    }
    parts
}

fn block_child_parts(body: Node, src_lines: &[&str]) -> Vec<BodyPart> {
    let mut parts = Vec::new();
    let mut cursor = body.walk();
    // First-claimer wins on shared rows — a trailing end-of-line
    // comment is a named sibling starting on the previous statement's
    // final row, and sibling parts double-claiming that row is a
    // walker-contract overlap (non-ancestor sibling batches).
    let mut claimed_lines = std::collections::HashSet::new();
    for child in body.named_children(&mut cursor) {
        let mut lines = Vec::new();
        extend_nonblank_rows(
            &mut lines,
            src_lines,
            child.start_position().row,
            child.end_position().row,
        );
        let lines: Vec<usize> = dedup_sorted(lines)
            .into_iter()
            .filter(|line| claimed_lines.insert(*line))
            .collect();
        if !lines.is_empty() {
            parts.push(BodyPart { lines });
        }
    }
    parts
}

/// Class body rows, skipping methods and the leading docstring.
fn collect_class_body(inner: Node, src_lines: &[&str]) -> FileLines {
    let Some(body) = inner.child_by_field_name("body") else {
        return FileLines::new(Vec::new());
    };
    let docstring = first_docstring_statement(body);
    let mut method_ranges = Vec::new();
    let mut cursor = body.walk();
    for child in body.children(&mut cursor) {
        if let Some((unit, inner_node)) = function_or_decorated(child)
            && inner_node.kind() == "function_definition"
        {
            method_ranges.push((unit.start_position().row, unit.end_position().row));
        }
    }
    let body_start = body.start_position().row;
    let body_end = body.end_position().row;
    let mut excluded = method_ranges;
    if let Some(d) = docstring {
        excluded.push((d.start_position().row, d.end_position().row));
    }
    excluded.sort_by_key(|(start, _)| *start);

    let mut out = Vec::new();
    let mut range_start = body_start;
    for (skip_start, skip_end) in excluded {
        if range_start < skip_start {
            extend_nonblank_rows(&mut out, src_lines, range_start, skip_start - 1);
        }
        if range_start <= skip_end {
            range_start = skip_end + 1;
        }
    }
    if range_start <= body_end {
        extend_nonblank_rows(&mut out, src_lines, range_start, body_end);
    }
    FileLines::new(out)
}

/// `def test_*` first lines (top-level + class-body). The inner `def`
/// row, not the decorator row — so `@pytest.mark.parametrize(...)`
/// stacks don't hide the signature.
fn collect_test_function_starts(tree: &Tree, source: &str) -> Vec<usize> {
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if let Some((_, inner)) = function_or_decorated(child) {
            match inner.kind() {
                "function_definition" => push_test_fn_row(inner, source, &mut out),
                "class_definition" => collect_test_methods_in_class(inner, source, &mut out),
                _ => {}
            }
        }
    }
    dedup_sorted(out)
}

fn push_test_fn_row(inner: Node, source: &str, out: &mut Vec<usize>) {
    if let Some(name) = name_of(inner, source)
        && name.starts_with("test_")
    {
        out.push(inner.start_position().row + 1);
    }
}

fn collect_test_methods_in_class(class_node: Node, source: &str, out: &mut Vec<usize>) {
    let Some(body) = class_node.child_by_field_name("body") else {
        return;
    };
    let mut cursor = body.walk();
    for child in body.children(&mut cursor) {
        if let Some((_, inner)) = function_or_decorated(child)
            && inner.kind() == "function_definition"
        {
            push_test_fn_row(inner, source, out);
        }
    }
}

// --- spine modules -------------------------------------------------------

/// Depth that spine modules rank at — between the package-root anchors
/// (entrypoint pin, depth ≤ 1) and ordinary implementation modules.
const SPINE_MODULE_DEPTH_PIN: usize = 2;

/// A re-export wall naming more modules than this carries no curation
/// signal — the `__init__.py` is just assembling the whole package, so
/// pinning its sources lifts most files at once and displaces
/// orientation rows. Measured on the training corpus: curated walls
/// (pluggy 4, chronos 5 spine modules) win from the pin; assembly
/// walls (requests/click 8, typeguard 9, flask 10) regress. No package
/// sits at 6-7, so the boundary inside that span is unmeasured.
const SPINE_MODULES_MAX: usize = 6;

/// True when the package root `__init__.py` imports names from `file` —
/// the module is a re-export source implementing the package's public
/// API. NS authors rank these spine modules right after orientation;
/// the depth/cat priors otherwise demote them (a src-layout module is
/// always depth ≥ 3).
fn is_package_spine_module(file: &Path, ctx: &WalkCtx) -> bool {
    if is_python_entrypoint(file) {
        return false;
    }
    let Some(package_root) = python_package_root(file, ctx) else {
        return false;
    };
    package_spine_modules(ctx, &package_root).contains(file)
}

/// Topmost ancestor package dir of `file` (contiguous `__init__.py`
/// chain, clipped to the walk root). `None` when the containing dir is
/// not a package.
fn python_package_root(file: &Path, ctx: &WalkCtx) -> Option<PathBuf> {
    let mut dir = file.parent()?;
    if !dir.join("__init__.py").is_file() {
        return None;
    }
    while let Some(parent) = dir.parent() {
        if !parent.starts_with(ctx.root()) || !parent.join("__init__.py").is_file() {
            break;
        }
        dir = parent;
    }
    Some(dir.to_path_buf())
}

fn package_spine_modules(ctx: &WalkCtx, package_root: &Path) -> Arc<HashSet<PathBuf>> {
    if let Some(cached) = ctx.python_state().spine_modules.borrow().get(package_root) {
        return Arc::clone(cached);
    }
    let computed = Arc::new(collect_spine_modules_uncached(ctx, package_root));
    ctx.python_state()
        .spine_modules
        .borrow_mut()
        .insert(package_root.to_path_buf(), Arc::clone(&computed));
    computed
}

fn collect_spine_modules_uncached(ctx: &WalkCtx, package_root: &Path) -> HashSet<PathBuf> {
    let mut out = HashSet::new();
    let init = package_root.join("__init__.py");
    let Some((source, tree)) = parse_python(ctx, &init) else {
        return out;
    };
    let package_name = package_root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() != "import_from_statement" {
            continue;
        }
        let Some(module_node) = child.child_by_field_name("module_name") else {
            continue;
        };
        let text = source[module_node.start_byte()..module_node.end_byte()].trim();
        let module_path = match text.strip_prefix('.') {
            // `from .mod import …` / `from .pkg.mod import …`. Deeper
            // relative levels (`..`) cannot resolve at the package root.
            Some(rest) if !rest.starts_with('.') => rest,
            Some(_) => continue,
            // Absolute self-import (`from pkg.mod import …`).
            None => match text
                .strip_prefix(package_name)
                .filter(|_| !package_name.is_empty())
            {
                Some("") => "",
                Some(rest) => match rest.strip_prefix('.') {
                    Some(suffix) => suffix,
                    None => continue,
                },
                None => continue,
            },
        };
        if module_path.is_empty() {
            // `from . import a, b` — each imported name is a module.
            for name in import_from_statement_names(child, &source) {
                if let Some(path) = resolve_package_module(package_root, &name) {
                    out.insert(path);
                }
            }
        } else if let Some(path) = resolve_package_module(package_root, module_path) {
            out.insert(path);
        }
    }
    if out.len() > SPINE_MODULES_MAX {
        out.clear();
    }
    out
}

/// Imported names of a `from … import a, b as c` statement (alias
/// sources, first dotted component).
fn import_from_statement_names(node: Node, source: &str) -> Vec<String> {
    let mut cursor = node.walk();
    let mut out = Vec::new();
    for name_node in node.children_by_field_name("name", &mut cursor) {
        let target = if name_node.kind() == "aliased_import" {
            name_node.child_by_field_name("name")
        } else {
            Some(name_node)
        };
        let Some(target) = target else { continue };
        let text = source[target.start_byte()..target.end_byte()].trim();
        let first = text.split('.').next().unwrap_or(text);
        if !first.is_empty() {
            out.push(first.to_string());
        }
    }
    out
}

/// Resolve a dotted module path relative to the package root to a real
/// file: `pkg/a/b.py`, falling back to `pkg/a/b/__init__.py`.
fn resolve_package_module(package_root: &Path, dotted: &str) -> Option<PathBuf> {
    let mut path = package_root.to_path_buf();
    for part in dotted.split('.') {
        if part.is_empty() {
            return None;
        }
        path.push(part);
    }
    let module_file = path.with_extension("py");
    if module_file.is_file() {
        return Some(module_file);
    }
    let init = path.join("__init__.py");
    init.is_file().then_some(init)
}

// --- value functions ----------------------------------------------------

fn python_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if is_python_entrypoint(file) {
        return file_depth_factor(file, ctx, true);
    }
    let depth = ctx.depth_from_root(file);
    let pinned_depth = if is_package_spine_module(file, ctx) {
        depth.min(SPINE_MODULE_DEPTH_PIN)
    } else {
        depth
    };
    depth_factor(pinned_depth) * ctx.non_essential_factor(file)
}

fn is_python_entrypoint(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| matches!(n, "__init__.py" | "__main__.py"))
}

fn imports_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // `__init__.py` carries the package's public surface (`from .X
    // import Y as Y` re-exports + `__all__` + `__version__`) — both
    // catastrophic-omission and follow-up axes are pinned high so a
    // hundreds-of-tokens import block still beats individual per-decl
    // batches in cost^0.35-penalised ratio. `__main__.py` is not
    // boosted (its imports are plumbing for a CLI body, not a
    // re-export anchor); the depth pin in `python_depth_factor`
    // already keeps it visible at small budgets.
    let (cat, fu) = if is_init_py(file) {
        (0.70, 1.0)
    } else {
        (0.25, 0.45)
    };
    mix_signals(cat, fu, 0.30, python_depth_factor(file, ctx))
        * top_level_package_init_factor(file, ctx)
}

fn imports_chunk_value(file: &Path, ctx: &WalkCtx, chunk_index: usize, chunk_count: usize) -> f64 {
    imports_value(file, ctx) * reexport_import_chunk_factor(chunk_index, chunk_count)
}

fn setup_manifest_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.45, 0.55, 0.45, python_depth_factor(file, ctx))
}

fn is_init_py(file: &Path) -> bool {
    file.file_name().and_then(|n| n.to_str()) == Some("__init__.py")
}

/// True iff `file` is `<name>` at the root of its package hierarchy —
/// i.e. its grandparent does not contain `__init__.py`. Excludes
/// `examples/<topic>/<pkg>/<name>` via `non_essential_factor` so the
/// boost stays scoped to real public-API anchors.
fn is_top_level_package_file(file: &Path, ctx: &WalkCtx, name: &str) -> bool {
    if file.file_name().and_then(|n| n.to_str()) != Some(name) {
        return false;
    }
    let Some(parent) = file.parent() else {
        return false;
    };
    let Some(grandparent) = parent.parent() else {
        return true;
    };
    if grandparent.join("__init__.py").is_file() {
        return false;
    }
    ctx.non_essential_factor(file) >= 1.0
}

fn top_level_package_init_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if is_top_level_package_file(file, ctx, "__init__.py") {
        3.0
    } else {
        1.0
    }
}

fn decl_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // `__init__.py` carries the package's public surface; non-init
    // modules are implementation detail and their names surface should
    // not crowd README / public-export batches in the early budget.
    let cat = if is_init_py(file) {
        0.65
    } else if ctx.depth_from_root(file) >= 3 && !is_package_spine_module(file, ctx) {
        0.4
    } else {
        0.5
    };
    mix_signals(cat, 0.55, 0.35, python_depth_factor(file, ctx))
        * concrete_impl_sibling_factor(file).max(1.0)
}

/// 1.5× boost for `base.py`, 0.6× damp on its concrete siblings.
///
/// Role split (chronos NS order is the canonical evidence): NS authors
/// schedule breadth-first — base's surface, then every sibling's
/// surface, then base's bodies. So surface roles (names, method sigs)
/// take the boost but never the damp (`.max(1.0)` — a core file's
/// location roster is the anti-omission hedge regardless of siblings),
/// while depth roles (class body, method) take the damp but never the
/// boost (`.min(1.0)` — a boosted body outranks sibling surfaces and
/// funds a deep dive the NS ranks last).
fn concrete_impl_sibling_factor(file: &Path) -> f64 {
    let Some(parent) = file.parent() else {
        return 1.0;
    };
    let Some(stem) = file.file_stem().and_then(|s| s.to_str()) else {
        return 1.0;
    };
    if stem == "__init__" {
        return 1.0;
    }
    if stem == "base" && parent.join("base.py").is_file() {
        return 1.5;
    }
    if parent.join("base.py").is_file() {
        0.6
    } else {
        1.0
    }
}

fn method_sigs_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.55, 0.50, 0.30, python_depth_factor(file, ctx))
        * concrete_impl_sibling_factor(file).max(1.0)
}

fn decl_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.70 * kv).min(1.0);
    let fu = (0.85 * kv).min(1.0);
    mix_signals(cat, fu, 0.65, python_depth_factor(file, ctx))
        * concrete_impl_sibling_factor(file).max(1.0)
}

fn decl_doc_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.20 * kv).min(1.0);
    let fu = (0.60 * kv).min(1.0);
    mix_signals(cat, fu, 0.80, python_depth_factor(file, ctx))
}

fn decl_body_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.30 * kv).min(1.0);
    let fu = (0.80 * kv).min(1.0);
    mix_signals(cat, fu, 0.70, python_depth_factor(file, ctx))
}

fn class_body_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.35 * v).min(1.0);
    let fu = (0.70 * v).min(1.0);
    mix_signals(cat, fu, 0.55, python_depth_factor(file, ctx))
        * concrete_impl_sibling_factor(file).min(1.0)
}

fn method_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.55 * v).min(1.0);
    let fu = (0.75 * v).min(1.0);
    mix_signals(cat, fu, 0.55, python_depth_factor(file, ctx))
        * concrete_impl_sibling_factor(file).min(1.0)
}

fn method_doc_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.15 * v).min(1.0);
    let fu = (0.50 * v).min(1.0);
    mix_signals(cat, fu, 0.70, python_depth_factor(file, ctx))
}

fn method_body_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let v = info.visibility_factor();
    let cat = (0.20 * v).min(1.0);
    let fu = (0.65 * v).min(1.0);
    mix_signals(cat, fu, 0.55, python_depth_factor(file, ctx))
}

fn test_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.40, 0.0, 0.30, python_depth_factor(file, ctx))
}

// --- helpers ------------------------------------------------------------

fn is_test_file(file: &Path) -> bool {
    file.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with("test_") || n.ends_with("_test.py"))
}

fn parse_python(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_python::LANGUAGE.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::BatchContent;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    fn parse(source: &str) -> (String, Tree) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("load python grammar");
        let tree = parser.parse(source, None).expect("parse");
        (source.to_string(), tree)
    }

    /// Re-collects methods using the source string so tests can assert
    /// on names. The runtime walker doesn't need the names — it only
    /// reads them for visibility classification, which is done after
    /// the fact below.
    fn methods_with_names<'a>(decls: &[DeclInfo<'a>], source: &str) -> Vec<(usize, String, bool)> {
        let mut out = Vec::new();
        for d in decls {
            if d.kind != DeclKind::Class {
                continue;
            }
            for m in collect_methods_in_class(d, source) {
                let name = name_of(m.inner_node, source).unwrap_or("").to_string();
                out.push((m.start_line, name.clone(), is_underscore_private(&name)));
            }
        }
        out
    }

    fn assert_python_scheduler_overlap_free(src: &str) {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("catalog.py");
        std::fs::write(&file, src).unwrap();

        let scheduler = Scheduler::new(dir.path().to_path_buf(), FsWalker, 1_000_000, None);
        let _ = scheduler.run_with_report();
    }

    fn decl_name_batches(batches: &[Batch<BatchKey>]) -> Vec<&Batch<BatchKey>> {
        batches
            .iter()
            .filter(|batch| {
                matches!(
                    batch.key,
                    BatchKey::Python(
                        PythonKey::DeclNames { .. } | PythonKey::DeclNamesChunk { .. }
                    )
                )
            })
            .collect()
    }

    #[test]
    fn python_decl_names_only_splits_oversize_surfaces() {
        let dir = tempfile::tempdir().unwrap();
        let small_file = dir.path().join("small.py");
        std::fs::write(&small_file, "def one(): pass\ndef two(): pass\n").unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let small = expand_source_files(std::slice::from_ref(&small_file), &ctx);
        let small_names = decl_name_batches(&small);
        assert_eq!(small_names.len(), 1);
        assert!(matches!(
            small_names[0].key,
            BatchKey::Python(PythonKey::DeclNames { .. })
        ));

        let large_file = dir.path().join("large.py");
        let mut source = String::new();
        for i in 0..48 {
            source.push_str(&format!(
                "def public_function_{i:02}(first_argument: SomeLongProtocol, second_argument: AnotherLongProtocol) -> ReturnProtocol:\n    return first_argument\n\n"
            ));
        }
        std::fs::write(&large_file, source).unwrap();
        let large = expand_source_files(std::slice::from_ref(&large_file), &ctx);
        let large_names = decl_name_batches(&large);
        assert!(large_names.len() > 1, "expected an oversize split");
        assert!(matches!(
            large_names[0].key,
            BatchKey::Python(PythonKey::DeclNames { .. })
        ));
        for (chunk_index, pair) in large_names.windows(2).enumerate() {
            assert_eq!(pair[1].predecessor.as_ref(), Some(&pair[0].key));
            assert!(matches!(
                pair[1].key,
                BatchKey::Python(PythonKey::DeclNamesChunk {
                    chunk_index: actual,
                    ..
                }) if actual == chunk_index + 1
            ));
            // Value density decreases along the chain (the conserved
            // allocation tilts head-ward); absolute chunk values track
            // chunk cost, so they need not decrease monotonically.
            let density = |batch: &Batch<BatchKey>| {
                batch.value / ctx.marginal_tokens(&batch.content).max(1) as f64
            };
            assert!(density(pair[1]) < density(pair[0]));
        }
        let total: f64 = large_names.iter().map(|batch| batch.value).sum();
        let unsplit =
            decl_names_value(&large_file, &ctx) * python_roster_mass_factor(&large_file, 48);
        assert!(
            (total - unsplit).abs() < 1e-9,
            "split catalog must conserve the unsplit value: {total} vs {unsplit}"
        );

        let mut covered = HashSet::new();
        for batch in large_names {
            let BatchContent::Lines { spans } = &batch.content else {
                panic!("decl names must emit line spans");
            };
            for (path, line, _) in crate::content::explode_spans(spans) {
                assert!(
                    covered.insert((path, line)),
                    "names chunks must be disjoint"
                );
            }
        }
        assert_eq!(covered.len(), 96, "48 signatures plus 48 ellipses");
    }

    /// A small docstring-led method body splits via `block_child_parts`;
    /// a trailing end-of-line comment on a multi-line statement's final
    /// row is a named sibling starting on that same row, and both parts
    /// claiming it panics the scheduler's overlap assert (xonsh
    /// events.py at 1M).
    #[test]
    fn python_body_parts_trailing_comment_row_claimed_once() {
        assert_python_scheduler_overlap_free(
            "class Events:\n    def method(self):\n        \"\"\"Doc line.\"\"\"\n        x = call(\n            1,\n        )  # trailing note\n",
        );
    }

    #[test]
    fn python_setup_manifest_captures_full_install_requires_keyword() {
        let src = "\
from setuptools import setup

setup(
    name=\"demo\",
    install_requires=[
        \"a\",
        \"b\",
    ],
    extras_require={\"dev\": [\"pytest\"]},
)
";
        let (source, tree) = parse(src);
        let (start_line, lines) =
            collect_setup_manifest(Path::new("setup.py"), &tree, &source).unwrap();
        assert_eq!(start_line, 5);
        assert_eq!(lines.full, vec![5, 6, 7, 8]);
    }

    #[test]
    fn python_setup_manifest_single_line_install_requires_stops_at_keyword() {
        let src = "\
from setuptools import setup

setup(
    name=\"demo\",
    install_requires=[\"a\", \"b\"],
    extras_require={\"dev\": [\"pytest\"]},
)
";
        let (source, tree) = parse(src);
        let (start_line, lines) =
            collect_setup_manifest(Path::new("setup.py"), &tree, &source).unwrap();
        assert_eq!(start_line, 5);
        assert_eq!(lines.full, vec![5]);
    }

    #[test]
    fn python_classify_top_level_decls() {
        let src = "\
\"\"\"module doc.\"\"\"

import os

__version__ = \"1.0\"
ENV_PREFIX = \"X\"

def helper():
    return 1

async def fetch():
    return 2

class Foo:
    pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let lines: Vec<_> = decls.iter().map(|d| (d.kind, d.start_line)).collect();
        // Module-doc + import + __version__ go into Imports. ENV_PREFIX is the
        // first Const decl. helper / fetch / Foo follow. (Line numbers are taken
        // from tree-sitter — verify against the source.)
        let kinds_only: Vec<_> = lines.iter().map(|(k, _)| *k).collect();
        assert_eq!(
            kinds_only,
            vec![
                DeclKind::Const,
                DeclKind::Function,
                DeclKind::Function,
                DeclKind::Class,
            ],
        );
    }

    #[test]
    fn python_decorator_span_includes_decorator_lines() {
        let src = "\
@final
@deprecated
class X:
    pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].start_line, 1, "decorator row is start_line");
        let lines = collect_decl(&decls[0]);
        // Span: decorators (1, 2) + class header (3) — 4 lines for "X:" included.
        assert!(lines.full.contains(&1));
        assert!(lines.full.contains(&2));
        assert!(lines.full.contains(&3));
    }

    #[test]
    fn python_decorated_methods_surface_in_method_sigs() {
        let src = "\
class C:
    @property
    def excinfo(self):
        return self._x

    @classmethod
    def from_call(cls):
        return cls()

    def plain(self):
        pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let methods = methods_with_names(&decls, &source);
        let names: Vec<&str> = methods.iter().map(|(_, n, _)| n.as_str()).collect();
        assert_eq!(names, vec!["excinfo", "from_call", "plain"]);
        // Decorator rows are start lines for decorated methods.
        let starts: Vec<_> = methods.iter().map(|(l, _, _)| *l).collect();
        assert_eq!(starts, vec![2, 6, 10]);
    }

    #[test]
    fn python_method_decl_includes_decorator_lines() {
        let src = "\
class C:
    @property
    @cached_property
    def value(self):
        return self._value
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let by_class = collect_methods_by_class(&decls, &source);
        let method = by_class[0].1[0];

        let lines = collect_method_decl(&method);
        assert_eq!(lines.full, vec![2, 3, 4]);
        assert_eq!(signature_line(&method), 4);
    }

    #[test]
    fn python_method_decl_keeps_a_wrapped_signature_s_return_annotation() {
        let src = "\
class C:
    @overload
    def get(
        self, key: str, default: None = None
    ) -> str | None: ...
    @overload
    def get(self, key: str, default: int) -> str | int: ...
    def get(
        self, key: str, default: object = None
    ) -> object:
        return default
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let by_class = collect_methods_by_class(&decls, &source);
        let methods = &by_class[0].1;

        assert_eq!(
            collect_method_decl(&methods[0]).full,
            vec![2, 3, 4, 5],
            "the `) -> str | None: ...` row is signature, not body — it is what \
             discriminates this overload from the next"
        );
        assert_eq!(collect_method_decl(&methods[1]).full, vec![6, 7]);
        assert_eq!(
            collect_method_decl(&methods[2]).full,
            vec![8, 9, 10],
            "a body on its own line still stops the decl at the signature"
        );

        let stub_rows: Vec<_> = methods
            .iter()
            .filter(|m| collapsed_overload_stub(m, methods, &source))
            .map(signature_line)
            .collect();
        assert_eq!(
            stub_rows,
            vec![3, 7],
            "both stubs drop off the roster; the implementation's row stands for the stack"
        );
    }

    #[test]
    fn python_decorated_methods_remain_overlap_free() {
        let src = "\
class C:
    @property
    def one(self): pass
    @property
    def two(self): pass
";
        assert_python_scheduler_overlap_free(src);
    }

    #[test]
    fn python_visibility_factor_underscore_vs_dunder() {
        assert!(!is_underscore_private("Foo"));
        assert!(!is_underscore_private("foo"));
        assert!(is_underscore_private("_helper"));
        assert!(!is_underscore_private("__init__"));
        assert!(!is_underscore_private("__call__"));
        assert!(is_underscore_private("__private_helper"));
    }

    #[test]
    fn python_method_sigs_lists_each_method() {
        let src = "\
class A:
    def one(self): pass
    def two(self): pass
    def three(self): pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let by_class = collect_methods_by_class(&decls, &source);
        let starts: Vec<_> = by_class
            .iter()
            .flat_map(|(_, methods)| methods.iter().map(|m| m.start_line))
            .collect();
        assert_eq!(starts, vec![2, 3, 4]);
    }

    #[test]
    fn python_decl_names_ellipsis_skips_next_decl_line() {
        let mut src = String::new();
        for i in 0..14 {
            src.push_str(&format!("def f{i:02}(): pass\n"));
        }

        assert_python_scheduler_overlap_free(&src);
    }

    #[test]
    fn python_method_sigs_ellipsis_skips_following_decl_line() {
        let mut src = String::new();
        for class_index in 0..12 {
            src.push_str(&format!("class C{class_index:02}:\n"));
            for method_index in 0..3 {
                src.push_str(&format!("    def m{method_index}(self): pass\n"));
            }
        }
        src.push_str("X = 1\n");

        assert_python_scheduler_overlap_free(&src);
    }

    #[test]
    fn python_class_body_emits_fields_skipping_methods() {
        let src = "\
class HookOpts:
    \"\"\"Options.\"\"\"
    firstresult: bool
    historic: bool
    warn_on_impl: Warning | None

    def __init__(self):
        pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let class = decls.iter().find(|d| d.kind == DeclKind::Class).unwrap();
        let src_lines: Vec<&str> = source.lines().collect();
        let body = collect_class_body(class.inner_node, &src_lines);
        assert_eq!(
            body.full,
            vec![3, 4, 5],
            "fields kept; docstring + method dropped"
        );
    }

    #[test]
    fn python_class_with_only_methods_emits_no_class_body() {
        let src = "\
class A:
    \"\"\"Just methods.\"\"\"
    def one(self):
        pass
    def two(self):
        pass
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let class = decls.iter().find(|d| d.kind == DeclKind::Class).unwrap();
        let src_lines: Vec<&str> = source.lines().collect();
        let body = collect_class_body(class.inner_node, &src_lines);
        assert!(body.full.is_empty(), "got {:?}", body.full);
    }

    #[test]
    fn python_test_file_surfaces_only_test_funcs() {
        let src = "\
def helper():
    pass

@pytest.mark.parametrize(
    \"x\",
    [1, 2],
)
def test_first(x):
    assert x

class TestThing:
    def test_inner(self):
        pass
    def helper(self):
        pass

@pytest.mark.skip
class TestDecorated:
    def test_dec(self):
        pass

def test_top():
    pass
";
        let (source, tree) = parse(src);
        let starts = collect_test_function_starts(&tree, &source);
        // Emitted line is the inner `def test_*` row, not the decorator
        // row — keeps the signature visible at small budgets.
        assert_eq!(starts, vec![8, 12, 19, 22], "got {starts:?}");
    }

    #[test]
    fn python_module_docstring_in_imports() {
        let src = "\
\"\"\"This is the module.

It does things.
\"\"\"

import os
";
        let (source, tree) = parse(src);
        let lines = collect_imports(&tree, &source);
        // The 4-line docstring on rows 1-4 plus the import on row 6.
        for r in [1, 2, 3, 4, 6] {
            assert!(lines.full.contains(&r), "missing {r}: {:?}", lines.full);
        }
    }

    #[test]
    fn python_all_assignment_in_imports() {
        let src = "\
import os

__all__ = [
    \"foo\",
    \"bar\",
]
";
        let (source, tree) = parse(src);
        let lines = collect_imports(&tree, &source);
        // Multi-line __all__ literal lines 3-6 plus import on line 1.
        for r in [1, 3, 4, 5, 6] {
            assert!(lines.full.contains(&r), "missing {r}: {:?}", lines.full);
        }
    }

    #[test]
    fn python_module_constant_emits_decl() {
        let src = "ENV_PREFIX = \"x\"\n";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].kind, DeclKind::Const);
        assert_eq!(decls[0].start_line, 1);
        let all_name_lines = collect_all_name_lines(&decls, &[]);
        let names = collect_decl_names_from(&decls, &all_name_lines);
        assert_eq!(names.full, vec![1]);
    }

    #[test]
    fn python_docstring_extracted_from_expression_statement() {
        let src = "\
def f():
    \"\"\"docstring.\"\"\"
    return 1
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let f = &decls[0];
        let doc = collect_doc_for(f.inner_node, &source);
        assert_eq!(doc.full, vec![2]);
        let src_lines: Vec<&str> = source.lines().collect();
        let body = def_body_parts(f.inner_node, &src_lines);
        assert_eq!(body.len(), 1);
        assert_eq!(body[0].lines, vec![3], "body skips docstring on row 2");
    }

    #[test]
    fn python_large_def_body_splits_by_top_level_statement() {
        let src = "\
def f():
    \"\"\"docstring.\"\"\"
    x0 = 0
    x1 = 1
    x2 = 2
    x3 = 3
    x4 = 4
    x5 = 5
    x6 = 6
    x7 = 7
    x8 = 8
    x9 = 9
    x10 = 10
    x11 = 11
    x12 = 12
";
        let (source, tree) = parse(src);
        let decls = find_top_level_decls(&tree, &source);
        let src_lines: Vec<&str> = source.lines().collect();
        let parts = def_body_parts(decls[0].inner_node, &src_lines);
        assert_eq!(parts.len(), 13);
        assert_eq!(parts[0].lines, vec![3]);
        assert_eq!(parts[12].lines, vec![15]);
    }

    #[test]
    fn python_real_dir_renders_seeded_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("__init__.py"),
            "from .core import greet\n\n__all__ = [\"greet\"]\n",
        )
        .unwrap();
        std::fs::write(
            root.join("core.py"),
            "\"\"\"Core module.\"\"\"\n\ndef greet(name):\n    \"\"\"Say hi.\"\"\"\n    return f\"hello {name}\"\n\nclass Person:\n    \"\"\"A person.\"\"\"\n    name: str\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            rendered.contains("def greet(name)"),
            "missing function signature:\n{rendered}",
        );
        assert!(
            rendered.contains("class Person"),
            "missing class header:\n{rendered}",
        );
        assert!(
            rendered.contains("__all__"),
            "missing __all__ in imports surface:\n{rendered}",
        );

        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Python(PythonKey::Decl { .. }))),
            "expected a Python::Decl batch; keys: {keys:?}"
        );
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Python(PythonKey::Imports { .. }))),
            "expected a Python::Imports batch; keys: {keys:?}"
        );
    }

    #[test]
    fn python_test_file_only_emits_test_names() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("test_something.py"),
            "def test_one():\n    pass\n\ndef helper():\n    pass\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        let test_decl_count = keys
            .iter()
            .filter(|k| matches!(k, BatchKey::Python(PythonKey::Decl { .. })))
            .count();
        assert_eq!(
            test_decl_count, 0,
            "expected no Python::Decl batches in a test file; keys: {keys:?}"
        );
    }
}
