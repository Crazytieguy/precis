//! TOML walker. Uses `tree-sitter-toml-ng` to identify top-level `[table]`
//! headers and their line ranges. Emits one batch per ontology-recognized
//! section group (identity / features / dependencies).
//!
//! Keys:
//! - `Identity { file }` — `[package]`, `[workspace]`, `[workspace.package]`
//! - `Features { file }` — `[features]`
//! - `Dependencies { file }` — `[dependencies]`, `[dev-dependencies]`,
//!   `[build-dependencies]`, `[workspace.dependencies]`

use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{BatchKey, FsKey, ResolvedBatch, TomlKey, ValueSignals};
use crate::value::{depth_factor, non_essential_factor};

use super::{Candidate, FileLines, WalkCtx, fs::files_with_extension, single_file_lines_batch};

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
        return Vec::new();
    };
    let toml_files = files_with_extension(dir, "toml");
    if toml_files.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in toml_files {
        let depth = ctx.depth_from_root(&file);
        out.push(candidate(
            TomlKey::Identity { file: file.clone() },
            identity_signals(&file, depth),
            60,
        ));
        out.push(candidate(
            TomlKey::Features { file: file.clone() },
            features_signals(&file, depth),
            40,
        ));
        out.push(candidate(
            TomlKey::Dependencies { file: file.clone() },
            dependencies_signals(&file, depth),
            80,
        ));
    }
    out
}

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Toml(tk) = key else {
        return None;
    };
    match tk {
        TomlKey::Identity { file } => mat_sections(
            file,
            |n| matches!(n, "package" | "workspace" | "workspace.package"),
            identity_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        TomlKey::Features { file } => mat_sections(
            file,
            |n| n == "features",
            features_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
        TomlKey::Dependencies { file } => mat_sections(
            file,
            |n| {
                matches!(
                    n,
                    "dependencies"
                        | "dev-dependencies"
                        | "build-dependencies"
                        | "workspace.dependencies"
                )
            },
            dependencies_signals(file, ctx.depth_from_root(file)),
            ctx,
        ),
    }
}

fn mat_sections(
    file: &Path,
    name_match: impl Fn(&str) -> bool,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch> {
    let (source, tree) = parse_toml(ctx, file)?;
    let sections = collect_sections(&tree, &source);
    let mut line_numbers: Vec<usize> = Vec::new();
    for (name, start_line, end_line) in sections {
        if name_match(&name) {
            line_numbers.extend(start_line..=end_line);
        }
    }
    if line_numbers.is_empty() {
        return None;
    }
    line_numbers.sort();
    line_numbers.dedup();
    single_file_lines_batch(file, &source, FileLines::new(line_numbers), signals)
}

// --- candidate helpers ---

fn candidate(tk: TomlKey, signals: ValueSignals, cost_hint: usize) -> Candidate<BatchKey> {
    Candidate::new(tk.into(), signals, cost_hint)
}

fn identity_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 1.0,
        follow_up_minimization: 0.7,
        zero_tool_call_understanding: 0.85,
        depth_factor: depth_factor(depth) * non_essential_factor(file),
    }
}

fn features_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.75,
        follow_up_minimization: 0.6,
        zero_tool_call_understanding: 0.5,
        depth_factor: depth_factor(depth) * non_essential_factor(file),
    }
}

fn dependencies_signals(file: &Path, depth: usize) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.4,
        follow_up_minimization: 0.7,
        zero_tool_call_understanding: 0.4,
        depth_factor: depth_factor(depth) * non_essential_factor(file),
    }
}

// --- parser ---

fn parse_toml(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_toml_ng::LANGUAGE.into())
}

// --- section collection ---

/// Returns `(header_name, start_line_1based, end_line_1based)` for every
/// top-level `table` node. A table's range is its header line through the
/// row before the next table (or EOF).
fn collect_sections(tree: &Tree, source: &str) -> Vec<(String, usize, usize)> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut raw: Vec<(String, usize)> = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "table" {
            continue;
        }
        let Some(name) = extract_table_name(child, source) else {
            continue;
        };
        let start_row = child.start_position().row;
        raw.push((name, start_row));
    }
    let total_rows = source.lines().count();
    let mut out = Vec::with_capacity(raw.len());
    for i in 0..raw.len() {
        let start = raw[i].1 + 1;
        let end = if i + 1 < raw.len() {
            raw[i + 1].1 // next table's row (0-indexed) → last row of this section is one before (0-indexed → stays same 1-indexed)
        } else {
            total_rows
        };
        out.push((raw[i].0.clone(), start, end));
    }
    out
}

fn extract_table_name(node: Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if matches!(child.kind(), "bare_key" | "dotted_key" | "quoted_key") {
            let text = &source[child.start_byte()..child.end_byte()];
            return Some(text.trim().to_string());
        }
    }
    None
}
