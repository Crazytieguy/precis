//! Lua walker. LuaCATS-meta files (`---@meta` / dense `---` LuaDoc
//! comments) get a whole-file batch; other Lua sources use per-decl
//! breakdown mirroring the C/Python shape. Recognized decl shapes:
//!  - `function_declaration` (incl. `local function`),
//!  - `assignment_statement` / `variable_declaration` with a
//!    `function_definition` RHS, and
//!  - function-valued `field`s inside a top-level table-constructor RHS
//!    (the tables-as-classes idiom: `local M = { foo = function() ... end }`),
//!    surfaced one level deep so a `static = { ... }` sub-table resolves.
//!
//! Batch variants are documented on [`crate::batch::LuaKey`].

use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, LuaKey};
use crate::value::{
    NAMES_SURFACE_CHUNK_SIZE, mix_signals, names_surface_chunk_count, names_surface_chunk_factor,
    names_surface_chunk_index,
};

use super::{
    FileLines, WalkCtx, build_per_file_content, collect_doc_comments_above, dedup_sorted,
    extend_nonblank_rows, extend_span, file_depth_factor, file_lines_covered_by,
    fs::files_with_extension, node_end_row_trimmed, path_depth_factor, push_rows,
    single_file_lines_content, trim_end_before_next_decl, whole_file_lines_content,
};

/// Token cap for `MetaFileWhole` — above, fall back to per-decl.
const META_FILE_TOKEN_CAP: usize = 400;
/// Treat as meta-file when ≥60% of comment lines are `---`-prefixed
/// (LuaDoc-style, tagged or prose).
const META_DOC_DENSITY_THRESHOLD: f64 = 0.60;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let lua_files = files_with_extension(dir, "lua");
    if lua_files.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for file in &lua_files {
        let Some((source, tree)) = parse_lua(ctx, file) else {
            continue;
        };

        // LuaCATS-spec files render whole. Per-decl emission is
        // skipped — sibling batches would non-ancestor-overlap with
        // the whole-file span.
        if is_meta_file(&source)
            && crate::tokenizer::count(&source) <= META_FILE_TOKEN_CAP
            && let Some(content) = whole_file_lines_content(file, &source)
        {
            out.push(Batch {
                key: LuaKey::MetaFileWhole { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: meta_file_whole_value(file, ctx),
            });
            continue;
        }

        if let Some(content) = build_per_file_content(file, ctx, parse_lua, collect_header_banner) {
            out.push(Batch {
                key: LuaKey::Banner { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: banner_value(file, ctx),
            });
        }

        let identity_lines = collect_module_identity_lines(&source);
        if !identity_lines.is_empty()
            && let Some(content) =
                single_file_lines_content(file, &source, FileLines::new(identity_lines))
        {
            out.push(Batch {
                key: LuaKey::ModuleIdentity { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: module_identity_value(file, ctx),
            });
        }

        let decls = find_decls(&tree);
        if decls.is_empty() {
            continue;
        }

        let names_chunk_count = names_surface_chunk_count(decls.len()).max(1);
        let names_predecessors: Vec<_> = (0..names_chunk_count)
            .map(|chunk_index| {
                BatchKey::Lua(LuaKey::DeclNames {
                    file: file.clone(),
                    chunk_index,
                })
            })
            .collect();
        let all_starts: std::collections::HashSet<usize> =
            decls.iter().map(|(_, i)| i.start_line).collect();
        let names_lines_by_chunk: Vec<FileLines> = decls
            .chunks(NAMES_SURFACE_CHUNK_SIZE)
            .map(|c| collect_decl_names_from_with_global_starts(c, &all_starts))
            .collect();
        for (chunk_index, names_lines) in names_lines_by_chunk.iter().enumerate() {
            let Some(content) = single_file_lines_content(file, &source, names_lines.clone())
            else {
                continue;
            };
            out.push(Batch {
                key: names_predecessors[chunk_index].clone(),
                predecessor: None,
                content,
                value: decl_names_value(file, ctx, chunk_index, names_chunk_count),
            });
        }

        let src_lines: Vec<&str> = source.lines().collect();
        for (decl_index, (node, info)) in decls.iter().enumerate() {
            let names_chunk_index = names_surface_chunk_index(decl_index);
            let names_predecessor = names_predecessors[names_chunk_index].clone();
            let chunk_names_lines = &names_lines_by_chunk[names_chunk_index];
            let decl_key = LuaKey::Decl {
                file: file.clone(),
                start_line: info.start_line,
            };
            let doc_lines = collect_doc_comments_above(*node, &source);
            let body_lines = collect_decl_body(*node, &src_lines);
            let decl_lines = collect_decl(*node, &source, &all_starts, !body_lines.full.is_empty());
            let decl_has_descendants = !doc_lines.full.is_empty() || !body_lines.full.is_empty();
            if (!file_lines_covered_by(&decl_lines, chunk_names_lines) || decl_has_descendants)
                && let Some(content) = single_file_lines_content(file, &source, decl_lines)
            {
                out.push(Batch {
                    key: decl_key.clone().into(),
                    predecessor: Some(names_predecessor.clone()),
                    content,
                    value: decl_value(file, ctx),
                });
            }
            let decl_predecessor = BatchKey::Lua(decl_key);
            if let Some(content) = single_file_lines_content(file, &source, doc_lines) {
                out.push(Batch {
                    key: LuaKey::DeclDoc {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor.clone()),
                    content,
                    value: decl_doc_value(file, ctx),
                });
            }
            if let Some(content) = single_file_lines_content(file, &source, body_lines) {
                out.push(Batch {
                    key: LuaKey::DeclBody {
                        file: file.clone(),
                        start_line: info.start_line,
                    }
                    .into(),
                    predecessor: Some(decl_predecessor),
                    content,
                    value: decl_body_value(file, ctx),
                });
            }
        }
    }

    out
}

// --- parser -------------------------------------------------------------

fn parse_lua(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_lua::LANGUAGE.into())
}

// --- decl classification ------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct DeclInfo {
    start_line: usize,
}

/// Top-level fn-like declarations. Tables-as-classes
/// (`local M = { foo = function … }`) surface one nesting level deep.
fn find_decls<'a>(tree: &'a Tree) -> Vec<(Node<'a>, DeclInfo)> {
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "function_declaration" => {
                push_decl(&mut out, child);
            }
            "variable_declaration" | "assignment_statement" => {
                if rhs_of_kind(child, "function_definition").is_some() {
                    push_decl(&mut out, child);
                } else if let Some(tc) = rhs_of_kind(child, "table_constructor") {
                    collect_function_fields(tc, &mut out, 1);
                }
            }
            _ => {}
        }
    }
    out.sort_by_key(|(_, d)| d.start_line);
    out.dedup_by_key(|(_, d)| d.start_line);
    out
}

fn push_decl<'a>(out: &mut Vec<(Node<'a>, DeclInfo)>, node: Node<'a>) {
    out.push((
        node,
        DeclInfo {
            start_line: node.start_position().row + 1,
        },
    ));
}

/// Emit fields with function values from a table constructor, recursing
/// into nested constructors up to `remaining_depth` more levels.
fn collect_function_fields<'a>(
    tc: Node<'a>,
    out: &mut Vec<(Node<'a>, DeclInfo)>,
    remaining_depth: u8,
) {
    let mut cursor = tc.walk();
    for field in tc.children(&mut cursor) {
        if field.kind() != "field" {
            continue;
        }
        let Some(value) = field.child_by_field_name("value") else {
            continue;
        };
        match value.kind() {
            "function_definition" => push_decl(out, field),
            "table_constructor" if remaining_depth > 0 => {
                // Only surface the parent if a descendant would surface.
                let before = out.len();
                collect_function_fields(value, out, remaining_depth - 1);
                if out.len() > before {
                    push_decl(out, field);
                }
            }
            _ => {}
        }
    }
}

/// The node of `kind` that's the RHS of an `assignment_statement` /
/// `variable_declaration`. Used to recognise `local Foo = function(...)`
/// and `local Foo = {...}` shapes.
fn rhs_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "expression_list" => {
                let mut inner = child.walk();
                for expr in child.children(&mut inner) {
                    if expr.kind() == kind {
                        return Some(expr);
                    }
                }
            }
            "assignment_statement" => {
                if let Some(n) = rhs_of_kind(child, kind) {
                    return Some(n);
                }
            }
            _ => {}
        }
    }
    None
}

// --- collectors ---------------------------------------------------------

/// Identity-table metadata keys — the standard Lua library convention
/// (`local M = { _VERSION = …, _DESCRIPTION = … }`).
const IDENTITY_META_KEYS: [&str; 7] = [
    "_VERSION",
    "_DESCRIPTION",
    "_NAME",
    "_URL",
    "_AUTHOR",
    "_LICENSE",
    "_COPYRIGHT",
];

fn line_has_identity_meta_key(line: &str) -> bool {
    IDENTITY_META_KEYS.iter().any(|k| line.contains(k))
}

/// Lines (1-based) of a top-of-file module identity table: a
/// `local M = { _VERSION = …, _DESCRIPTION = … }` metadata block (the
/// standard Lua "what is this library" idiom). Returns the table opener
/// plus its leading single-line `_KEY = …` fields, stopping before a
/// long-string field (`[[`) or the table close — so the multi-line
/// `_LICENSE = [[ … ]]` body is excluded. Empty unless an identity table
/// (≥1 metadata key) sits at the top of the file.
fn collect_module_identity_lines(source: &str) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().collect();
    // First non-blank, non-comment line.
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim_start();
        if t.is_empty() || t.starts_with("--") {
            i += 1;
        } else {
            break;
        }
    }
    if i >= lines.len() {
        return Vec::new();
    }
    // Must open a table assignment: `local NAME = {` or `NAME = {`.
    let opener = lines[i].trim_start();
    let lhs = opener.strip_prefix("local ").unwrap_or(opener);
    let is_table_open = opener.contains('{')
        && lhs.split('=').next().is_some_and(|name| {
            let name = name.trim();
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
        });
    if !is_table_open {
        return Vec::new();
    }
    let mut out = vec![i + 1];
    let mut j = i + 1;
    while j < lines.len() {
        let t = lines[j].trim();
        if t.contains("[[") || t.starts_with('}') {
            break;
        }
        if t.starts_with('_') {
            out.push(j + 1);
            j += 1;
        } else {
            break;
        }
    }
    if out
        .iter()
        .any(|&ln| line_has_identity_meta_key(lines[ln - 1]))
    {
        out
    } else {
        Vec::new()
    }
}

fn module_identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // "What is this library" — `_VERSION` / `_DESCRIPTION` / `_URL`. A
    // primary orientation surface, valued like a crate-doc lede.
    mix_signals(0.80, 0.55, 0.75, path_depth_factor(file, ctx))
}

/// Top-of-file `--` comment block.
fn collect_header_banner(tree: &Tree, source: &str) -> FileLines {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut lines = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() == "comment" {
            extend_span(&mut lines, child, source);
            continue;
        }
        break;
    }
    FileLines::new(lines)
}

fn collect_decl_names_from_with_global_starts(
    decls: &[(Node, DeclInfo)],
    all_starts: &std::collections::HashSet<usize>,
) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    for (_, info) in decls {
        full.push(info.start_line);
        if !all_starts.contains(&(info.start_line + 1)) {
            ellipses.push(info.start_line + 1);
        }
    }
    FileLines::new(full).with_ellipses(ellipses)
}

/// Signature lines for a decl — statement start through pre-body row,
/// trimmed at the next sibling decl's start_line (no anchor overlap).
/// `body_has_rows` is whether `collect_decl_body` delivered rows for
/// this decl — the body-elision `…` marker only makes sense when a
/// DeclBody batch will later replace it.
fn collect_decl(
    node: Node,
    source: &str,
    all_starts: &std::collections::HashSet<usize>,
    body_has_rows: bool,
) -> FileLines {
    let start_row = node.start_position().row;
    let body_node = body_node_for_decl(node);
    let sig_end = match body_node {
        Some(body) => body.start_position().row.saturating_sub(1).max(start_row),
        None => node_end_row_trimmed(node, source),
    };
    let sig_end = trim_end_before_next_decl(sig_end, start_row, all_starts);

    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    push_rows(&mut full, start_row, sig_end);

    let ellipsis_line = sig_end + 2;
    if body_has_rows && !all_starts.contains(&ellipsis_line) {
        ellipses.push(ellipsis_line);
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Non-blank rows of a function-like decl's body block. A Lua `block`
/// node spans exactly the statement rows (no brace rows to exclude —
/// `function`/`end` belong to the parent), so every row is content;
/// only a block starting on the signature row itself is clamped past
/// the signature.
fn collect_decl_body(node: Node, src_lines: &[&str]) -> FileLines {
    let Some(body) = body_node_for_decl(node) else {
        return FileLines::new(Vec::new());
    };
    let first = body.start_position().row.max(node.start_position().row + 1);
    let mut out = Vec::new();
    extend_nonblank_rows(&mut out, src_lines, first, body.end_position().row);
    FileLines::new(out)
}

/// The function body's `block` node for any decl shape.
fn body_node_for_decl<'a>(node: Node<'a>) -> Option<Node<'a>> {
    match node.kind() {
        "function_declaration" => node.child_by_field_name("body"),
        "assignment_statement" | "variable_declaration" => {
            let fn_def = rhs_of_kind(node, "function_definition")?;
            fn_def.child_by_field_name("body")
        }
        "field" => {
            let value = node.child_by_field_name("value")?;
            if value.kind() == "function_definition" {
                value.child_by_field_name("body")
            } else {
                None
            }
        }
        _ => None,
    }
}

// --- meta-file detection ------------------------------------------------

/// True iff `source` is a LuaCATS `---@meta` spec — by leader line
/// or by density of `---`-prefixed (LuaDoc-style) comment lines.
fn is_meta_file(source: &str) -> bool {
    // Quick check for `---@meta` leader.
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("---@meta") {
            return true;
        }
        break;
    }
    // Density: `---`-prefixed comments vs plain `--` comments.
    let mut luadoc = 0usize;
    let mut commented = 0usize;
    for line in source.lines() {
        let t = line.trim_start();
        if t.starts_with("---") {
            luadoc += 1;
            commented += 1;
        } else if t.starts_with("--") {
            commented += 1;
        }
    }
    commented > 0 && (luadoc as f64 / commented as f64) >= META_DOC_DENSITY_THRESHOLD
}

// --- value functions ----------------------------------------------------

fn banner_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Banners are scaffolding; modest standalone value, mostly bait so
    // the descendant decls get value reflected back.
    mix_signals(0.20, 0.35, 0.35, path_depth_factor(file, ctx))
}

fn meta_file_whole_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // LuaCATS specs ARE the API contract — load-bearing for soluna's
    // tier-1/2 NS rows. Treat like a primary surface.
    mix_signals(0.85, 0.85, 0.55, file_depth_factor(file, ctx, false))
}

fn decl_names_value(file: &Path, ctx: &WalkCtx, chunk_index: usize, chunk_count: usize) -> f64 {
    mix_signals(0.65, 0.70, 0.45, path_depth_factor(file, ctx))
        * names_surface_chunk_factor(chunk_index, chunk_count)
}

fn decl_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.65, 0.80, 0.55, path_depth_factor(file, ctx))
}

fn decl_doc_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.30, 0.55, 0.75, path_depth_factor(file, ctx))
}

fn decl_body_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.30, 0.70, 0.60, path_depth_factor(file, ctx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    fn parse(source: &str) -> (String, Tree) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_lua::LANGUAGE.into())
            .expect("load lua grammar");
        let tree = parser.parse(source, None).expect("parse");
        (source.to_string(), tree)
    }

    #[test]
    fn lua_finds_table_as_class_function_fields() {
        let src = "\
local M = {
  foo = function(self) return 1 end,
  bar = function(self) return 2 end,
  static = {
    baz = function(self) return 3 end,
  },
  data = 42,
}
return M
";
        let (_source, tree) = parse(src);
        let decls = find_decls(&tree);
        let starts: Vec<usize> = decls.iter().map(|(_, i)| i.start_line).collect();
        assert!(
            starts.contains(&2) && starts.contains(&3) && starts.contains(&5),
            "decls: {decls:?}"
        );
        assert!(
            !starts.contains(&7),
            "data=42 must not be a decl: {decls:?}"
        );
    }

    #[test]
    fn lua_decl_body_covers_first_and_last_statement_rows() {
        let src = "\
local function foo(x)
  local y = x + 1
  if y > 2 then
    y = y - 1
  end
  return y
end
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree);
        assert_eq!(decls.len(), 1);
        let src_lines: Vec<&str> = source.lines().collect();
        let body = collect_decl_body(decls[0].0, &src_lines);
        assert_eq!(body.full, vec![2, 3, 4, 5, 6], "body rows: {:?}", body.full);
    }

    #[test]
    fn lua_decl_body_single_statement_is_nonempty() {
        let src = "\
local function foo(x)
  return x + 1
end
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree);
        let src_lines: Vec<&str> = source.lines().collect();
        let body = collect_decl_body(decls[0].0, &src_lines);
        assert_eq!(body.full, vec![2], "body rows: {:?}", body.full);
    }

    #[test]
    fn lua_finds_top_level_function_declaration() {
        let src = "\
function foo(x)
  return x + 1
end
";
        let (_source, tree) = parse(src);
        let decls = find_decls(&tree);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].1.start_line, 1);
    }

    #[test]
    fn lua_finds_local_function_declaration() {
        let src = "\
local function foo(x)
  return x + 1
end
";
        let (_source, tree) = parse(src);
        let decls = find_decls(&tree);
        assert_eq!(decls.len(), 1, "decls: {:?}", decls);
    }

    #[test]
    fn lua_finds_table_method_assignment() {
        let src = "\
local M = {}
function M.bar(x) return x end
M.baz = function(x) return x end
return M
";
        let (_source, tree) = parse(src);
        let decls = find_decls(&tree);
        // Expect: M.bar (function_declaration) and M.baz (assignment).
        // `local M = {}` is a variable_declaration WITHOUT function RHS,
        // so it's not recognized.
        let starts: Vec<usize> = decls.iter().map(|(_, i)| i.start_line).collect();
        assert!(
            starts.contains(&2) && starts.contains(&3),
            "decls: {decls:?}"
        );
    }

    #[test]
    fn lua_is_meta_file_detects_meta_leader() {
        let src = "---@meta\n\nlocal M = {}\nreturn M\n";
        assert!(is_meta_file(src));
    }

    #[test]
    fn lua_is_meta_file_detects_high_tag_density() {
        let src = "\
---Module doc
---@class Soluna
---@field platform string
---@field version string
local soluna = {}
return soluna
";
        assert!(is_meta_file(src));
    }

    #[test]
    fn lua_is_meta_file_rejects_plain_module() {
        let src = "\
-- A plain module
-- with ordinary comments
local M = {}
function M.foo() return 1 end
return M
";
        assert!(!is_meta_file(src));
    }

    #[test]
    fn lua_module_identity_captures_metadata_and_stops_before_license() {
        // middleclass-style identity table: opener + single-line `_KEY`
        // fields, truncated before the multi-line `_LICENSE = [[`.
        let src = "\
local middleclass = {
  _VERSION     = 'middleclass v4.1.1',
  _DESCRIPTION = 'Object Orientation for Lua',
  _URL         = 'https://example.com',
  _LICENSE     = [[
    MIT LICENSE
  ]]
}
";
        assert_eq!(collect_module_identity_lines(src), vec![1, 2, 3, 4]);
    }

    #[test]
    fn lua_module_identity_ignores_plain_tables_and_requires() {
        // No `_VERSION`-style metadata key ⇒ not an identity table.
        assert!(
            collect_module_identity_lines("local cfg = {\n  host = 'x',\n  port = 80,\n}\n")
                .is_empty()
        );
        // A bare module-return table is not an assignment opener.
        assert!(collect_module_identity_lines("return {\n  _VERSION = '1.0',\n}\n").is_empty());
    }

    #[test]
    fn lua_decl_spans_do_not_overlap_for_adjacent_decls() {
        // Two adjacent `local function` decls. Tree-sitter is unlikely to
        // produce overlapping nodes here, but the adjacency trim should
        // still guard the catch-all path.
        let src = "\
local function first(x) return x end
local function second(x) return x end
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree);
        let all_starts: std::collections::HashSet<usize> =
            decls.iter().map(|(_, i)| i.start_line).collect();
        let mut claimed: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let src_lines: Vec<&str> = source.lines().collect();
        for (node, info) in &decls {
            let body_lines = collect_decl_body(*node, &src_lines);
            let lines = collect_decl(*node, &source, &all_starts, !body_lines.full.is_empty());
            for line in &lines.full {
                let prev = claimed.insert(*line, info.start_line);
                assert!(prev.is_none(), "overlap at line {line}");
            }
        }
    }

    #[test]
    fn lua_real_dir_emits_meta_file_for_luacats_spec() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("api.lua"),
            "\
---@meta

---@class Engine
---@field version string

---Returns the engine version.
---@return string version
function Engine:version() end
",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Lua(LuaKey::MetaFileWhole { .. }))),
            "expected a Lua::MetaFileWhole batch; keys: {keys:?}"
        );
    }
}
