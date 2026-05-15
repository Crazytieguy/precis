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
//! - `GoMod { file }`: line-set batch for `go.mod` / `go.work`. Drops
//!   `// indirect` lines from inside `require ( … )` blocks; module /
//!   `go` / `toolchain` / `replace` / `exclude` / `retract` / `use`
//!   directives stay.
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

use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, GoKey};
use crate::content::BatchContent;
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, build_per_file_content, collect_doc_comments_above, dedup_sorted,
    extend_span, file_depth_factor, file_lines_covered_by,
    fs::{files_with_extension, list_dir},
    push_rows, signature_end_row, single_file_lines_content,
};

/// Cap on `GoMod` line emission when the file has no `// indirect`
/// markers (older / hand-written go.mods). With the indirect filter,
/// even monorepo go.mods render under this cap.
const GOMOD_LINE_CAP: usize = 80;

const VISIBILITY_FACTOR_EXPORTED: f64 = 1.0;
const VISIBILITY_FACTOR_UNEXPORTED: f64 = 0.6;

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
        let Some(content) = build_gomod_content(&path, ctx) else {
            continue;
        };
        out.push(Batch {
            key: GoKey::GoMod { file: path.clone() }.into(),
            predecessor: None,
            content,
            value: gomod_value(&path, ctx),
        });
    }
    out
}

/// Build `GoMod` content as a line set. Drops `// indirect` lines
/// inside `require ( … )` blocks; keeps every directive (`module`,
/// `go`, `toolchain`, `replace`, `exclude`, `retract`, `use`) and
/// the structural `require (` / `)` delimiters.
fn build_gomod_content(file: &Path, ctx: &WalkCtx) -> Option<BatchContent> {
    let source = ctx.read_source(file)?;
    let total_lines = source.lines().count();
    if total_lines == 0 {
        return None;
    }

    let mut lines = Vec::with_capacity(total_lines);
    let mut indirect_seen = false;
    let mut in_require_block = false;
    for (i, raw) in source.lines().enumerate() {
        let trimmed = raw.trim();
        let line_no = i + 1;
        if !in_require_block && trimmed.starts_with("require") && trimmed.ends_with('(') {
            in_require_block = true;
            lines.push(line_no);
            continue;
        }
        if in_require_block && trimmed == ")" {
            in_require_block = false;
            lines.push(line_no);
            continue;
        }
        if in_require_block && raw.contains("// indirect") {
            indirect_seen = true;
            continue;
        }
        lines.push(line_no);
    }

    if !indirect_seen && lines.len() > GOMOD_LINE_CAP {
        return None;
    }
    single_file_lines_content(file, &source, FileLines::new(lines))
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
        if let Some(content) = build_per_file_content(file, ctx, parse_go, collect_package_imports)
        {
            out.push(Batch {
                key: GoKey::PackageImports { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: package_imports_value(file, ctx),
            });
        }

        let Some((source, tree)) = parse_go(ctx, file) else {
            continue;
        };
        let decls = find_decls(&tree, &source);
        if decls.is_empty() {
            continue;
        }

        let names_key = GoKey::DeclNames { file: file.clone() };
        let parent_names_lines = collect_decl_names_from(&decls);
        if let Some(content) = single_file_lines_content(file, &source, parent_names_lines.clone())
        {
            out.push(Batch {
                key: names_key.clone().into(),
                predecessor: None,
                content,
                value: decl_names_value(file, ctx),
            });
        }
        let names_predecessor = BatchKey::Go(names_key);
        let src_lines: Vec<&str> = source.lines().collect();
        for (node, info) in &decls {
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
            if (!file_lines_covered_by(&decl_lines, &parent_names_lines) || decl_has_descendants)
                && let Some(content) = single_file_lines_content(file, &source, decl_lines)
            {
                out.push(Batch {
                    key: decl_key.clone().into(),
                    predecessor: Some(names_predecessor.clone()),
                    content,
                    value: decl_value(file, info, ctx),
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
                    value: decl_doc_value(file, info, ctx),
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
                    predecessor: Some(decl_predecessor),
                    content,
                    value: decl_body_value(file, info, ctx),
                });
            }
        }
    }
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
    /// One entry per name the names surface should list. For a
    /// single-spec decl, just `[start_line]`. For a grouped
    /// `type/var/const ( … )` block, one entry per inner spec.
    name_lines: Vec<usize>,
    body_rows: Option<(usize, usize)>,
    exported: bool,
}

impl DeclInfo {
    fn visibility_factor(&self) -> f64 {
        if self.exported {
            VISIBILITY_FACTOR_EXPORTED
        } else {
            VISIBILITY_FACTOR_UNEXPORTED
        }
    }
}

fn find_decls<'a>(tree: &'a Tree, source: &str) -> Vec<(Node<'a>, DeclInfo)> {
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "function_declaration" => {
                if let Some(info) = function_info(child, source) {
                    out.push((child, info));
                }
            }
            "method_declaration" => {
                if let Some(info) = method_info(child, source) {
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

fn function_info(node: Node, source: &str) -> Option<DeclInfo> {
    let name_node = node.child_by_field_name("name")?;
    let name = &source[name_node.start_byte()..name_node.end_byte()];
    let sig_end = signature_end_row(node);
    let mut decl_lines = Vec::new();
    push_rows(&mut decl_lines, node.start_position().row, sig_end);
    let start_line = node.start_position().row + 1;
    Some(DeclInfo {
        kind: DeclKind::Func,
        start_line,
        decl_lines,
        name_lines: vec![start_line],
        body_rows: body_interior_rows(node),
        exported: is_exported(name),
    })
}

fn method_info(node: Node, source: &str) -> Option<DeclInfo> {
    let name_node = node.child_by_field_name("name")?;
    let name = &source[name_node.start_byte()..name_node.end_byte()];
    let sig_end = signature_end_row(node);
    let mut decl_lines = Vec::new();
    push_rows(&mut decl_lines, node.start_position().row, sig_end);
    let start_line = node.start_position().row + 1;
    Some(DeclInfo {
        kind: DeclKind::Method,
        start_line,
        decl_lines,
        name_lines: vec![start_line],
        body_rows: body_interior_rows(node),
        // Method counts as exported only when both its name and the
        // receiver type are exported. A lowercase method on an exported
        // type stays package-private from the agent's "what's the
        // public API" view.
        exported: is_exported(name) && receiver_type_exported(node, source),
    })
}

fn grouped_type_info(node: Node, source: &str) -> DeclInfo {
    let mut cursor = node.walk();
    let mut name_lines = Vec::new();
    let mut exported = false;
    for spec in node.children(&mut cursor) {
        if !matches!(spec.kind(), "type_spec" | "type_alias") {
            continue;
        }
        name_lines.push(spec.start_position().row + 1);
        if let Some(name_node) = spec.child_by_field_name("name")
            && is_exported(&source[name_node.start_byte()..name_node.end_byte()])
        {
            exported = true;
        }
    }
    let mut decl_lines = Vec::new();
    push_rows(
        &mut decl_lines,
        node.start_position().row,
        node.end_position().row,
    );
    DeclInfo {
        kind: DeclKind::Type,
        start_line: node.start_position().row + 1,
        decl_lines,
        name_lines,
        body_rows: None,
        exported,
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

/// Walk a receiver / parameter type back to its root `type_identifier`.
/// Strips `*T` and `T[U]` wrappers; gives up on anything more exotic.
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

fn package_imports_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.30, 0.55, 0.3, go_depth_factor(file, ctx)) * go_aux_factor(file)
}

/// Sits below per-decl `Type`-kind value so the scheduler favours
/// structural anchors in load-bearing files over a blanket name
/// surface in every `.go` file.
fn decl_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.65, 0.55, 0.35, go_depth_factor(file, ctx)) * go_aux_factor(file)
}

fn decl_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.70 * kv).min(1.0);
    let fu = (0.85 * kv).min(1.0);
    mix_signals(cat, fu, 0.65, go_depth_factor(file, ctx)) * go_aux_factor(file)
}

fn decl_doc_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.20 * kv).min(1.0);
    let fu = (0.6 * kv).min(1.0);
    mix_signals(cat, fu, 0.8, go_depth_factor(file, ctx)) * go_aux_factor(file)
}

fn decl_body_value(file: &Path, info: &DeclInfo, ctx: &WalkCtx) -> f64 {
    let kv = info.kind.kind_weight() * info.visibility_factor();
    let cat = (0.30 * kv).min(1.0);
    let fu = (0.80 * kv).min(1.0);
    mix_signals(cat, fu, 0.7, go_depth_factor(file, ctx)) * go_aux_factor(file)
}

/// Damp Go files whose stem carries a build-tag suffix or matches
/// the cobra-style shell-completion generator convention. Both
/// classes are aux implementation behind a portable interface; NS
/// authors anchor on the un-suffixed sibling.
fn go_aux_factor(file: &Path) -> f64 {
    let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
        return 1.0;
    };
    let lower = name.to_ascii_lowercase();
    let stem = lower.strip_suffix(".go").unwrap_or(&lower);
    if is_go_completion_stem(stem) || is_go_build_variant_stem(stem) {
        return 0.5;
    }
    1.0
}

fn is_go_completion_stem(stem: &str) -> bool {
    stem == "active_help" || stem.contains("completion")
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
        "aix"
            | "android"
            | "darwin"
            | "dragonfly"
            | "freebsd"
            | "hurd"
            | "illumos"
            | "ios"
            | "js"
            | "linux"
            | "nacl"
            | "netbsd"
            | "openbsd"
            | "plan9"
            | "solaris"
            | "wasip1"
            | "windows"
            | "zos"
            | "unix"
            | "bsd"
            | "other"
            | "notwin"
            | "nonwin"
            | "win"
            | "amd64"
            | "arm"
            | "arm64"
            | "386"
            | "loong64"
            | "mips"
            | "mips64"
            | "ppc64"
            | "riscv64"
            | "s390x"
            | "wasm"
    )
}

fn test_names_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.40, 0.0, 0.3, go_depth_factor(file, ctx))
}

fn gomod_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.85, 0.65, 0.5, go_depth_factor(file, ctx))
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

/// One full + ellipsis pair per name line so a grouped block surfaces
/// every inner spec, not just the `type (` opener.
fn collect_decl_names_from(decls: &[(Node, DeclInfo)]) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for (_, info) in decls {
        for &line in &info.name_lines {
            full.push(line);
            ellipses.push(line + 1);
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
    for row in (s + 1)..e {
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(row + 1);
        }
    }
    FileLines::new(out)
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let names = collect_decl_names_from(&decls);
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
    fn go_mod_indirect_filter_keeps_replace_and_retract() {
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
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("go.mod");
        std::fs::write(&path, src).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let content = build_gomod_content(&path, &ctx).expect("emits content");
        let BatchContent::Lines { spans } = content else {
            panic!("expected Lines content");
        };
        let mut lines = Vec::new();
        for span in &spans {
            for l in span.start..=span.end {
                lines.push(l);
            }
        }
        // Indirect line (line 7) dropped; everything else kept.
        assert!(lines.contains(&1), "module clause kept");
        assert!(lines.contains(&5), "require ( kept");
        assert!(lines.contains(&6), "direct require kept");
        assert!(!lines.contains(&7), "indirect require dropped");
        assert!(lines.contains(&8), "require ) kept");
        assert!(lines.contains(&10), "replace kept");
        assert!(lines.contains(&12), "retract kept");
    }

    #[test]
    fn go_mod_falls_back_to_capped_whole_file_when_no_indirect() {
        let src = "module example.com/foo\n\ngo 1.22\n";
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("go.mod");
        std::fs::write(&path, src).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let content = build_gomod_content(&path, &ctx).expect("emits content");
        let BatchContent::Lines { spans } = content else {
            panic!("expected Lines content");
        };
        assert!(!spans.is_empty());
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
