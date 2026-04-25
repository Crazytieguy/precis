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
//! - `ExportBody { file, start_line }`: body interior (brace-stripped) of
//!   a function or class export, predecessor = the matching `Export`.
//!   Sibling of `ExportDoc` under `Export`. Only emitted when the inner
//!   decl has a multi-line `statement_block` body. See the v4 plan note in
//!   `ignore/plan-ts-export-body-v2.md` for what's deferred (lexical-with-
//!   fn-init, expression arrows, forwardRef-wrapped callbacks).
//!
//! `.ts` and `.tsx` are both handled; the grammar is dispatched by
//! extension. Parse trees are cached in [`WalkCtx`].

use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{BatchKey, FsKey, ResolvedBatch, TsKey, ValueSignals};
use crate::value::depth_factor;

use super::{
    Candidate, FileLines, WalkCtx, dedup_sorted, extend_span, fs::files_with_any_extension,
    push_rows, signature_end_row, single_file_lines_batch,
};

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
        return Vec::new();
    };
    let ts_files = files_with_any_extension(dir, &["ts", "tsx"]);
    if ts_files.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for file in &ts_files {
        let ep = is_entrypoint_file(file);

        if ep {
            out.push(candidate(
                TsKey::ModuleDocLede { file: file.clone() },
                module_doc_lede_signals(file, ctx),
                40,
            ));
        }

        out.push(candidate(
            TsKey::Imports { file: file.clone() },
            imports_signals(file, ctx),
            60,
        ));

        let Some((source, tree)) = parse_ts(ctx, file) else {
            continue;
        };
        let exports = find_export_starts(&tree, &source);
        if exports.is_empty() {
            continue;
        }
        let names_key = TsKey::ExportNames { file: file.clone() };
        out.push(candidate(
            names_key.clone(),
            export_names_signals(file, ctx),
            exports.len() * 8,
        ));
        for item in &exports {
            let key = TsKey::Export {
                file: file.clone(),
                start_line: item.start_line,
            };
            out.push(
                candidate(
                    key.clone(),
                    export_signals(file, item.kind, ctx),
                    item.estimated_cost(),
                )
                .with_predecessor(BatchKey::Typescript(names_key.clone())),
            );
            out.push(
                candidate(
                    TsKey::ExportDoc {
                        file: file.clone(),
                        start_line: item.start_line,
                    },
                    export_doc_signals(file, item.kind, ctx),
                    60,
                )
                .with_predecessor(BatchKey::Typescript(key.clone())),
            );
            if item.body_emit_rows > 0 {
                out.push(
                    candidate(
                        TsKey::ExportBody {
                            file: file.clone(),
                            start_line: item.start_line,
                        },
                        export_body_signals(file, item.kind, ctx),
                        (item.body_emit_rows * 6).max(60),
                    )
                    .with_predecessor(BatchKey::Typescript(key)),
                );
            }
        }
    }

    out
}

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Typescript(tk) = key else {
        return None;
    };
    match tk {
        TsKey::ModuleDocLede { file } => mat_per_file(
            file,
            collect_module_doc_lede,
            module_doc_lede_signals(file, ctx),
            ctx,
        ),
        TsKey::Imports { file } => {
            mat_per_file(file, collect_imports, imports_signals(file, ctx), ctx)
        }
        TsKey::ExportNames { file } => mat_per_file(
            file,
            collect_export_names,
            export_names_signals(file, ctx),
            ctx,
        ),
        TsKey::Export { file, start_line } => {
            let (source, tree) = parse_ts(ctx, file)?;
            let kind = classify_export_at(&tree, &source, *start_line)?;
            let lines = collect_export_lines(&tree, *start_line);
            single_file_lines_batch(file, &source, lines, export_signals(file, kind, ctx))
        }
        TsKey::ExportDoc { file, start_line } => {
            let (source, tree) = parse_ts(ctx, file)?;
            let kind = classify_export_at(&tree, &source, *start_line)?;
            let lines = collect_export_doc_lines(&tree, &source, file, *start_line);
            single_file_lines_batch(file, &source, lines, export_doc_signals(file, kind, ctx))
        }
        TsKey::ExportBody { file, start_line } => {
            let (source, tree) = parse_ts(ctx, file)?;
            let kind = classify_export_at(&tree, &source, *start_line)?;
            let lines = collect_export_body(&tree, &source, *start_line);
            single_file_lines_batch(file, &source, lines, export_body_signals(file, kind, ctx))
        }
    }
}

// --- candidate + signal helpers ---

fn candidate(tk: TsKey, signals: ValueSignals, cost_hint: usize) -> Candidate<BatchKey> {
    Candidate::new(tk.into(), signals, cost_hint)
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

#[derive(Debug, Clone)]
struct ExportInfo {
    /// 1-based line of the wrapping `export_statement`. Walker only emits
    /// items wrapped by `export_statement`; bare top-level declarations
    /// without an `export` keyword are module-private and ignored.
    start_line: usize,
    kind: ItemKind,
    line_span: usize,
    /// Number of *non-blank* source rows the body materializer would emit
    /// for `ExportBody` if scheduled. 0 means no `ExportBody` candidate
    /// fires. Computed once at expand-time so the cost hint matches the
    /// actual emit count after `build_file_spans`'s blank-line filter.
    body_emit_rows: usize,
}

impl ExportInfo {
    fn estimated_cost(&self) -> usize {
        (self.line_span * 6).max(40)
    }
}

/// Top-level exports in a file. Walks `program` children, looking for
/// `export_statement` nodes and identifying the inner declaration. Re-
/// exports without an inner declaration (`export { foo } from '…'`) are
/// skipped — they're plumbing, picked up by `Imports`.
fn find_export_starts(tree: &Tree, source: &str) -> Vec<ExportInfo> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        let Some((kind, decl_node)) = classify_export(child, source) else {
            continue;
        };
        let start_line = child.start_position().row + 1;
        let end_line = item_end_line(decl_node, source);
        let body_emit_rows = export_body_rows(decl_node, kind, source).len();
        out.push(ExportInfo {
            start_line,
            kind,
            line_span: end_line.saturating_sub(start_line) + 1,
            body_emit_rows,
        });
    }
    out
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

/// Materializer-path lookup: classify the export at `start_line` without
/// re-running `find_export_starts` (which would recompute every export's
/// `body_emit_rows` AST walk just to discard the count). All callers
/// downstream of materialize need only the `ItemKind` for signal weighting.
fn classify_export_at(tree: &Tree, source: &str, start_line: usize) -> Option<ItemKind> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        return classify_export(child, source).map(|(k, _)| k);
    }
    None
}

fn item_end_line(node: Node, source: &str) -> usize {
    let text = &source[node.start_byte()..node.end_byte()];
    let internal = text.trim_end_matches(['\n', '\r']).split('\n').count();
    node.start_position().row + internal.max(1)
}

/// Files whose name signals "module entrypoint / public surface".
fn is_entrypoint_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| matches!(n, "index.ts" | "index.tsx" | "main.ts" | "mod.ts"))
}

fn entrypoint_boost(path: &Path) -> f64 {
    if is_entrypoint_file(path) { 1.4 } else { 1.0 }
}

fn file_depth_factor(path: &Path, ctx: &WalkCtx) -> f64 {
    let depth = ctx.depth_from_root(path);
    let raw = if is_entrypoint_file(path) {
        depth_factor(depth.min(1))
    } else {
        depth_factor(depth)
    };
    raw * ctx.non_essential_factor(path)
}

fn module_doc_lede_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.8 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.5,
        zero_tool_call_understanding: 0.9,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn imports_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.3 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.55,
        zero_tool_call_understanding: 0.3,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn export_names_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: (0.8 * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.35,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn export_signals(file: &Path, kind: ItemKind, ctx: &WalkCtx) -> ValueSignals {
    let k = kind.kind_weight();
    ValueSignals {
        catastrophic_omission: (0.70 * k * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: (0.85 * k).min(1.0),
        zero_tool_call_understanding: 0.65,
        depth_factor: file_depth_factor(file, ctx),
    }
}

fn export_doc_signals(file: &Path, kind: ItemKind, ctx: &WalkCtx) -> ValueSignals {
    let k = kind.kind_weight();
    ValueSignals {
        catastrophic_omission: (0.20 * k * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: (0.6 * k).min(1.0),
        zero_tool_call_understanding: 0.8,
        depth_factor: file_depth_factor(file, ctx),
    }
}

// Strictly below `Export.catastrophic` (0.70) — `Export`'s signature already
// hedges existence; the body is a refinement. Strictly above
// `Export.follow_up` (0.85) — body is the prime "don't go grep" signal.
fn export_body_signals(file: &Path, kind: ItemKind, ctx: &WalkCtx) -> ValueSignals {
    let k = kind.kind_weight();
    ValueSignals {
        catastrophic_omission: (0.45 * k * entrypoint_boost(file)).min(1.0),
        follow_up_minimization: (0.9 * k).min(1.0),
        zero_tool_call_understanding: 0.8,
        depth_factor: file_depth_factor(file, ctx),
    }
}

// --- parser ---

fn parse_ts(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    let language = if path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("tsx"))
    {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    };
    ctx.parse_tree(path, &language)
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
    let (source, tree) = parse_ts(ctx, file)?;
    let lines = collect(&tree, &source);
    single_file_lines_batch(file, &source, lines, signals)
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
            "import_statement" => extend_span(&mut lines, child, source),
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

/// A bare re-export pulls names from another module: it has no inner
/// declaration, has an `export_clause` / `namespace_export` / `*` form,
/// **and** carries a `from '…'` source string. Local export clauses
/// (`export { foo }` with no `from`) are exports of a local declaration,
/// not plumbing — those get their own [`ItemKind::NamedReexport`] batch.
fn is_bare_reexport(node: Node) -> bool {
    first_decl_child(node).is_none() && has_export_clause(node) && has_from_source(node)
}

/// True if `decl` is a `lexical_declaration` / `variable_declaration` whose
/// initialiser is a function-, class-, object-, or array-literal worth
/// hiding behind a body-elision marker.
fn has_value_expression_body(decl: Node) -> bool {
    let mut cur = decl.walk();
    decl.children(&mut cur).any(|c| {
        matches!(c.kind(), "variable_declarator" | "lexical_binding") && {
            let mut vc = c.walk();
            c.children(&mut vc).any(|v| {
                matches!(
                    v.kind(),
                    "function_expression" | "arrow_function" | "class" | "object" | "array"
                )
            })
        }
    })
}

fn collect_export_names(tree: &Tree, source: &str) -> FileLines {
    let items = find_export_starts(tree, source);
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for item in &items {
        full.push(item.start_line);
        ellipses.push(item.start_line + 1);
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Lines for a single export's decl. For interface/type/class/enum, the
/// whole item. For function, the signature plus a body-elision marker.
/// For const/let, the assignment line(s) up through the `=`.
fn collect_export_lines(tree: &Tree, start_line: usize) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        if child.kind() != "export_statement" {
            return FileLines::new(Vec::new());
        }
        let mut full = Vec::new();
        let mut ellipses = Vec::new();
        let export_start_row = child.start_position().row;
        let Some(decl) = first_decl_or_value_child(child) else {
            // No inner decl/value — must be a local export clause
            // (`export { foo }` / `export type { Foo }`). Span the
            // whole statement; it's typically one line.
            push_rows(&mut full, export_start_row, child.end_position().row);
            return FileLines::new(dedup_sorted(full));
        };
        match decl.kind() {
            "function_declaration"
            | "function_signature"
            | "generator_function_declaration"
            | "function_expression"
            | "arrow_function" => {
                let sig_end = signature_end_row(decl);
                push_rows(&mut full, export_start_row, sig_end);
                if decl.child_by_field_name("body").is_some() {
                    ellipses.push(sig_end + 2);
                }
            }
            "class_declaration" | "abstract_class_declaration" | "class" => {
                let body = decl.child_by_field_name("body");
                let header_end = body
                    .map(|b| b.start_position().row)
                    .unwrap_or_else(|| decl.end_position().row);
                push_rows(&mut full, export_start_row, header_end);
                if let Some(b) = body {
                    let mut bcur = b.walk();
                    for member in b.children(&mut bcur) {
                        if matches!(
                            member.kind(),
                            "method_definition"
                                | "method_signature"
                                | "abstract_method_signature"
                                | "public_field_definition"
                                | "property_signature"
                        ) {
                            let m_sig_end = signature_end_row(member);
                            push_rows(&mut full, member.start_position().row, m_sig_end);
                            if member.child_by_field_name("body").is_some() {
                                ellipses.push(m_sig_end + 2);
                            }
                        }
                    }
                }
            }
            "interface_declaration" | "type_alias_declaration" | "enum_declaration" => {
                push_rows(&mut full, export_start_row, decl.end_position().row);
            }
            "lexical_declaration" | "variable_declaration" => {
                let end_row = signature_end_row(decl);
                push_rows(&mut full, export_start_row, end_row);
                if has_value_expression_body(decl) && end_row < decl.end_position().row {
                    ellipses.push(end_row + 2);
                }
            }
            _ => {
                // Default-export expression with nothing structural —
                // emit the single statement line.
                push_rows(&mut full, export_start_row, decl.end_position().row);
            }
        }
        return FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses));
    }
    FileLines::new(Vec::new())
}

/// Body interior lines for an export at `start_line`. Brace-strip rule:
/// `body.start_row` (the `{`) and `body.end_row` (the `}`) are excluded;
/// rows in between are emitted. For class exports, every member with a
/// `statement_block` body contributes its own interior; the merged set
/// is returned as a single `FileLines`. Only emits for the export shapes
/// covered by [`export_body_rows`] — others return empty (and the
/// scheduler propagates as a dead key).
fn collect_export_body(tree: &Tree, source: &str, start_line: usize) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        let Some((kind, decl)) = classify_export(child, source) else {
            return FileLines::new(Vec::new());
        };
        let rows = export_body_rows(decl, kind, source);
        return FileLines::new(rows);
    }
    FileLines::new(Vec::new())
}

/// 1-based source rows that the body materializer would emit for an
/// export's `(decl, kind)` pair, *after* applying the same blank-line
/// filter as `build_file_spans`. Used both to gate emission (returns
/// empty when there is no `statement_block` body, when the body is
/// single-line, or when interior is all blank) and to size the
/// speculative cost hint without re-walking the tree.
fn export_body_rows(decl: Node, kind: ItemKind, source: &str) -> Vec<usize> {
    let src_lines: Vec<&str> = source.lines().collect();
    let mut out = Vec::new();
    match kind {
        ItemKind::Function => {
            push_statement_block_interior(&mut out, decl.child_by_field_name("body"), &src_lines);
        }
        ItemKind::Class => {
            push_class_method_interiors(&mut out, decl, &src_lines);
        }
        ItemKind::Default => match decl.kind() {
            "function_declaration"
            | "function_expression"
            | "arrow_function"
            | "generator_function" => {
                push_statement_block_interior(
                    &mut out,
                    decl.child_by_field_name("body"),
                    &src_lines,
                );
            }
            "class" | "class_declaration" | "abstract_class_declaration" => {
                push_class_method_interiors(&mut out, decl, &src_lines);
            }
            _ => {}
        },
        _ => {}
    }
    dedup_sorted(out)
}

/// Push the interior rows (1-based) of `body` if it's a multi-line
/// `statement_block`. Skips blank source lines so the count matches the
/// post-`build_file_spans` emit count exactly — that's what
/// `body_emit_rows` is wired to in the cost hint, and a mismatch would
/// silently inflate the speculative ratio.
fn push_statement_block_interior(out: &mut Vec<usize>, body: Option<Node>, src_lines: &[&str]) {
    let Some(b) = body else { return };
    if b.kind() != "statement_block" {
        return;
    }
    let s = b.start_position().row;
    let e = b.end_position().row;
    if e <= s + 1 {
        return;
    }
    for row in (s + 1)..e {
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(row + 1);
        }
    }
}

/// Walk `class_decl`'s body, accumulating interior rows of every
/// `method_definition` member's `statement_block` body. Field
/// initializers (including arrow-init fields) and abstract method
/// signatures are skipped by design — they fall under the v4 deferral
/// (lexical-with-fn-init) noted in `ignore/plan-ts-export-body-v2.md`.
fn push_class_method_interiors(out: &mut Vec<usize>, class_decl: Node, src_lines: &[&str]) {
    let Some(class_body) = class_decl.child_by_field_name("body") else {
        return;
    };
    let mut cursor = class_body.walk();
    for member in class_body.children(&mut cursor) {
        if member.kind() == "method_definition" {
            push_statement_block_interior(out, member.child_by_field_name("body"), src_lines);
        }
    }
}

/// JSDoc (`/** */`) immediately above an export at `start_line`. For
/// entrypoint files the leading top-of-file JSDoc is reserved for
/// [`TsKey::ModuleDocLede`] — skip it here so the two batches don't
/// claim the same lines without a predecessor edge.
fn collect_export_doc_lines(
    tree: &Tree,
    source: &str,
    file: &Path,
    start_line: usize,
) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_position().row + 1 != start_line {
            continue;
        }
        let mut out = Vec::new();
        collect_jsdoc_above(child, source, &mut out, is_entrypoint_file(file));
        return FileLines::new(dedup_sorted(out));
    }
    FileLines::new(Vec::new())
}

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

    fn body_emit_rows_for(source: &str) -> Vec<usize> {
        let tree = parse(source);
        let exports = find_export_starts(&tree, source);
        let item = exports.first().unwrap();
        let FileLines { full, .. } = collect_export_body(&tree, source, item.start_line);
        // Sanity: helper-precomputed count matches actual emit rows.
        assert_eq!(item.body_emit_rows, full.len());
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
        let exports = find_export_starts(&tree, src);
        // body.start_row == body.end_row → empty interior. body_emit_rows
        // is 0; no candidate fires.
        assert_eq!(exports[0].body_emit_rows, 0);
    }

    #[test]
    fn walker_typescript_export_body_empty_body_no_emit() {
        let src = "export function foo() {}\n";
        let tree = parse(src);
        let exports = find_export_starts(&tree, src);
        assert_eq!(exports[0].body_emit_rows, 0);
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
        let exports = find_export_starts(&tree, src);
        assert_eq!(exports[0].body_emit_rows, 0);
    }

    #[test]
    fn walker_typescript_export_body_type_alias_no_emit() {
        let src = "export type Foo = { bar: number }\n";
        let tree = parse(src);
        let exports = find_export_starts(&tree, src);
        assert_eq!(exports[0].body_emit_rows, 0);
    }

    #[test]
    fn walker_typescript_export_body_enum_no_emit() {
        let src = "export enum Foo { A, B }\n";
        let tree = parse(src);
        let exports = find_export_starts(&tree, src);
        assert_eq!(exports[0].body_emit_rows, 0);
    }

    #[test]
    fn walker_typescript_export_body_lexical_arrow_no_emit_v4() {
        // v4 deferred: `Export` already emits the whole lexical_declaration
        // including the body. Adding `ExportBody` here would overlap.
        let src = "export const X = () => {\n  return 1;\n};\n";
        let tree = parse(src);
        let exports = find_export_starts(&tree, src);
        assert_eq!(exports[0].body_emit_rows, 0);
    }

    #[test]
    fn walker_typescript_export_body_expression_arrow_no_emit() {
        let src = "export const X = () => 1;\n";
        let tree = parse(src);
        let exports = find_export_starts(&tree, src);
        assert_eq!(exports[0].body_emit_rows, 0);
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
        // build_file_spans filters them out, so body_emit_rows must too.
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
}
