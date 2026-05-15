//! Lua walker. LuaCATS-meta files (`---@meta` / dense `---@` tags) get
//! a whole-file batch; other Lua sources use per-decl breakdown
//! mirroring the C/Python shape. Three top-level decl shapes are
//! recognized: `function_declaration`, and `assignment_statement` /
//! `variable_declaration` whose RHS is a `function_definition`.
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
    extend_span, file_depth_factor, file_lines_covered_by, fs::files_with_extension,
    node_end_row_trimmed, path_depth_factor, push_rows, single_file_lines_content,
    trim_end_before_next_decl,
};

/// Token cap for `MetaFileWhole`: above this, fall back to per-decl
/// emission so a large LuaCATS spec doesn't crowd the whole budget with
/// a single batch.
const META_FILE_TOKEN_CAP: usize = 400;
/// LuaCATS-tag density threshold for treating a file as a meta-file
/// even without a `---@meta` leader: > 60% of comment lines starting
/// with `---@`.
const META_TAG_DENSITY_THRESHOLD: f64 = 0.60;

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
            && let Some(content) = collect_meta_file_whole(file, &source)
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

        let decls = find_decls(&tree, &source);
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
            let decl_lines = collect_decl(*node, &source, &all_starts);
            let doc_lines = collect_doc_comments_above(*node, &source);
            let body_lines = collect_decl_body(*node, &src_lines);
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

/// Walk the top-level (`chunk`) children and emit a `DeclInfo` for each
/// function-like declaration we recognize. Sorted by start_line; dups
/// (rare; e.g. when `declaration` wraps a `function_declaration` that
/// also surfaces directly) are coalesced.
fn find_decls<'a>(tree: &'a Tree, _source: &str) -> Vec<(Node<'a>, DeclInfo)> {
    let root = tree.root_node();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        let included = match child.kind() {
            "function_declaration" => true,
            "variable_declaration" | "assignment_statement" => {
                function_definition_rhs(child).is_some()
            }
            _ => false,
        };
        if included {
            out.push((
                child,
                DeclInfo {
                    start_line: child.start_position().row + 1,
                },
            ));
        }
    }
    out.sort_by_key(|(_, d)| d.start_line);
    out.dedup_by_key(|(_, d)| d.start_line);
    out
}

/// The `function_definition` node that's the RHS of an `assignment_statement`
/// or `variable_declaration`. Returns `None` for plain `function_declaration`.
fn function_definition_rhs<'a>(node: Node<'a>) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "expression_list" => {
                let mut inner = child.walk();
                for expr in child.children(&mut inner) {
                    if expr.kind() == "function_definition" {
                        return Some(expr);
                    }
                }
            }
            "assignment_statement" => {
                if let Some(n) = function_definition_rhs(child) {
                    return Some(n);
                }
            }
            _ => {}
        }
    }
    None
}

// --- collectors ---------------------------------------------------------

/// Top-of-file `--` comment block. Stops at the first non-comment,
/// non-blank token.
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

/// Signature lines for a decl. For `function_declaration` and the
/// function-RHS-wrappers (`local foo = function(...)`,
/// `M.foo = function(...)`), the rows from the statement's start through
/// the row before the function's body. Trimmed at the next sibling
/// decl's start_line so we never claim a row that's another decl's
/// anchor (parallel to the C walker's adjacency trim).
fn collect_decl(
    node: Node,
    source: &str,
    all_starts: &std::collections::HashSet<usize>,
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

    // Body-elision marker: a `…` on the line right after the signature
    // so descendant `DeclBody` batches can override it with real
    // content. Only when the body has interior rows to elide.
    if let Some(body) = body_node {
        let bs = body.start_position().row;
        let be = body.end_position().row;
        let ellipsis_line = sig_end + 2;
        if be > bs + 1 && !all_starts.contains(&ellipsis_line) {
            ellipses.push(ellipsis_line);
        }
    }
    FileLines::new(dedup_sorted(full)).with_ellipses(dedup_sorted(ellipses))
}

/// Body interior of a function-like decl: rows strictly between the
/// body's first and last rows, blank source rows skipped. Returns empty
/// when the body has no interior to render.
fn collect_decl_body(node: Node, src_lines: &[&str]) -> FileLines {
    let Some(body) = body_node_for_decl(node) else {
        return FileLines::new(Vec::new());
    };
    let s = body.start_position().row;
    let e = body.end_position().row;
    if e <= s + 1 {
        return FileLines::new(Vec::new());
    }
    let mut out = Vec::new();
    for row in (s + 1)..e {
        if src_lines.get(row).is_some_and(|t| !t.trim().is_empty()) {
            out.push(row + 1);
        }
    }
    FileLines::new(out)
}

/// The function body's `block` node, regardless of which decl shape
/// holds it.
fn body_node_for_decl<'a>(node: Node<'a>) -> Option<Node<'a>> {
    match node.kind() {
        "function_declaration" => node.child_by_field_name("body"),
        "assignment_statement" | "variable_declaration" => {
            let fn_def = function_definition_rhs(node)?;
            fn_def.child_by_field_name("body")
        }
        _ => None,
    }
}

// --- meta-file detection ------------------------------------------------

/// Does this Lua source look like a LuaCATS spec / `---@meta` file?
/// True when either the first non-blank line is `---@meta` *or* >60%
/// of comment lines (`-- ...`) start with `---@`.
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
    // Density check: count `---`-prefixed comment lines vs. plain
    // comment lines. LuaCATS uses `---` (three dashes); ordinary
    // comments use `--`.
    let mut tagged = 0usize;
    let mut commented = 0usize;
    for line in source.lines() {
        let t = line.trim_start();
        if t.starts_with("---") {
            tagged += 1;
            commented += 1;
        } else if t.starts_with("--") {
            commented += 1;
        }
    }
    commented > 0 && (tagged as f64 / commented as f64) >= META_TAG_DENSITY_THRESHOLD
}

fn collect_meta_file_whole(file: &Path, source: &str) -> Option<crate::content::BatchContent> {
    let line_count = source.lines().count();
    if line_count == 0 {
        return None;
    }
    let lines: Vec<usize> = (1..=line_count).collect();
    single_file_lines_content(file, source, FileLines::new(lines))
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
    fn lua_finds_top_level_function_declaration() {
        let src = "\
function foo(x)
  return x + 1
end
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
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
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
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
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
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
    fn lua_decl_spans_do_not_overlap_for_adjacent_decls() {
        // Two adjacent `local function` decls. Tree-sitter is unlikely to
        // produce overlapping nodes here, but the adjacency trim should
        // still guard the catch-all path.
        let src = "\
local function first(x) return x end
local function second(x) return x end
";
        let (source, tree) = parse(src);
        let decls = find_decls(&tree, &source);
        let all_starts: std::collections::HashSet<usize> =
            decls.iter().map(|(_, i)| i.start_line).collect();
        let mut claimed: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (node, info) in &decls {
            let lines = collect_decl(*node, &source, &all_starts);
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
