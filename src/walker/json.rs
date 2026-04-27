//! JSON walker. `package.json` splits along the same ontology as
//! `Cargo.toml` (identity / scripts ≈ features / dependencies) plus a
//! JS-specific `Entry` batch for the entrypoint pointers
//! (`main`/`module`/`exports`/…). Other small JSON configs
//! (`tsconfig.json`, `.eslintrc.json`, `jsr.json`, `turbo.json`, …) get
//! a single `Whole` batch; lockfiles and large generated JSONs are
//! skipped. The full key→batch mapping lives in the `is_*_key`
//! predicates below.

use std::path::Path;
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, JsonKey};
use crate::content::BatchContent;
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, dedup_sorted, fs::files_with_extension, path_depth_factor,
    single_file_lines_content,
};

/// Hard cap on `Whole` JSON config rendering. Above this, we skip the
/// batch entirely — generated files (lockfiles, .nyc_output, package
/// manifests in node_modules) are pure noise.
const WHOLE_LINE_CAP: usize = 60;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let json_files = files_with_extension(dir, "json");
    if json_files.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in json_files {
        let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if is_skipped_json(name) {
            continue;
        }
        if is_package_json(name) {
            emit_package_json(&file, ctx, &mut out);
        } else if let Some(batch) = whole_json_batch(&file, name, ctx) {
            out.push(batch);
        }
    }
    out
}

fn whole_json_batch(file: &Path, name: &str, ctx: &WalkCtx) -> Option<Batch<BatchKey>> {
    // FS metadata avoids reading the file when the size hint alone
    // already disqualifies it — typical generated JSONs are huge.
    let byte_len = std::fs::metadata(file)
        .map(|m| m.len() as usize)
        .unwrap_or(usize::MAX);
    if byte_len > WHOLE_LINE_CAP * 200 {
        return None;
    }
    let source = ctx.read_source(file)?;
    let line_count = source.lines().count();
    if line_count == 0 || line_count > WHOLE_LINE_CAP {
        return None;
    }
    let lines: Vec<usize> = (1..=line_count).collect();
    let content = single_file_lines_content(file, &source, FileLines::new(lines))?;
    Some(Batch {
        key: JsonKey::Whole {
            file: file.to_path_buf(),
        }
        .into(),
        predecessor: None,
        content,
        value: whole_value(file, name, ctx),
    })
}

/// Emit the four `package.json` batches chained as
/// `Identity ← Entry ← Scripts ← Dependencies`. The chain exists so a
/// compact one-line `package.json` (where all four key groups collapse
/// onto the same source line) renders through the predecessor-override
/// path instead of tripping a non-ancestor overlap. For typical
/// multi-line files the spans are disjoint and the chain costs nothing.
///
/// Predecessor advances only when the prior section actually emitted —
/// a `package.json` missing identity keys still chains Entry → Scripts →
/// Dependencies cleanly rather than orphaning them on an unscheduled
/// Identity batch.
fn emit_package_json(file: &Path, ctx: &WalkCtx, out: &mut Vec<Batch<BatchKey>>) {
    let Some((source, tree)) = parse_json(ctx, file) else {
        return;
    };
    let pairs = top_level_pairs(&tree, &source);
    let mut prev: Option<BatchKey> = None;
    let mut push = |key: JsonKey, value: f64, name_match: fn(&str) -> bool| {
        let Some(content) = section_content(file, &source, &pairs, name_match) else {
            return;
        };
        let emitted = BatchKey::Json(key.clone());
        out.push(Batch {
            key: key.into(),
            predecessor: prev.clone(),
            content,
            value,
        });
        prev = Some(emitted);
    };
    let f = file.to_path_buf();
    push(
        JsonKey::Identity { file: f.clone() },
        identity_value(file, ctx),
        is_identity_key,
    );
    push(
        JsonKey::Entry { file: f.clone() },
        entry_value(file, ctx),
        is_entry_key,
    );
    push(
        JsonKey::Scripts { file: f.clone() },
        scripts_value(file, ctx),
        is_scripts_key,
    );
    push(
        JsonKey::Dependencies { file: f },
        dependencies_value(file, ctx),
        is_dependencies_key,
    );
}

fn section_content(
    file: &Path,
    source: &str,
    pairs: &[(String, usize, usize)],
    name_match: fn(&str) -> bool,
) -> Option<BatchContent> {
    let mut lines: Vec<usize> = Vec::new();
    for (name, start, end) in pairs {
        if name_match(name) {
            lines.extend(*start..=*end);
        }
    }
    if lines.is_empty() {
        return None;
    }
    single_file_lines_content(file, source, FileLines::new(dedup_sorted(lines)))
}

// --- file-name predicates ---

fn is_package_json(name: &str) -> bool {
    name.eq_ignore_ascii_case("package.json")
}

/// Files that should never produce JSON batches: lockfiles, generated
/// metadata, anything noisy enough that a full or partial render is
/// almost always wasted budget.
fn is_skipped_json(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(lower.as_str(), "package-lock.json" | "npm-shrinkwrap.json")
        || lower.ends_with(".lock.json")
        || lower.ends_with(".tsbuildinfo")
}

// --- key classification ---

fn is_identity_key(k: &str) -> bool {
    matches!(
        k,
        "name"
            | "version"
            | "description"
            | "license"
            | "licenses"
            | "author"
            | "authors"
            | "contributors"
            | "repository"
            | "homepage"
            | "bugs"
            | "keywords"
            | "type"
            | "private"
            | "publishConfig"
            | "funding"
    )
}

fn is_entry_key(k: &str) -> bool {
    matches!(
        k,
        "main"
            | "module"
            | "browser"
            | "exports"
            | "imports"
            | "types"
            | "typings"
            | "typesVersions"
            | "source"
            | "bin"
            | "files"
            | "directories"
            | "unpkg"
            | "jsdelivr"
            | "umd:main"
            | "jsnext:main"
            | "react-native"
            | "svelte"
            | "sideEffects"
            | "workspaces"
    )
}

fn is_scripts_key(k: &str) -> bool {
    matches!(k, "scripts" | "bin-scripts")
}

fn is_dependencies_key(k: &str) -> bool {
    matches!(
        k,
        "dependencies"
            | "devDependencies"
            | "peerDependencies"
            | "peerDependenciesMeta"
            | "optionalDependencies"
            | "bundledDependencies"
            | "bundleDependencies"
            | "engines"
            | "engineStrict"
            | "packageManager"
            | "overrides"
            | "resolutions"
    )
}

// --- value ---

fn identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(1.0, 0.7, 0.85, path_depth_factor(file, ctx))
}

fn entry_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.85, 0.85, 0.6, path_depth_factor(file, ctx))
}

fn scripts_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.6, 0.85, 0.55, path_depth_factor(file, ctx))
}

fn dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.4, 0.7, 0.4, path_depth_factor(file, ctx))
}

fn whole_value(file: &Path, name: &str, ctx: &WalkCtx) -> f64 {
    // tsconfig.json sits at the top of the public-facing tooling — rate it
    // just under package identity. Other configs are mid-rank.
    let lower = name.to_ascii_lowercase();
    let is_tsconfig =
        lower == "tsconfig.json" || (lower.starts_with("tsconfig.") && lower.ends_with(".json"));
    let cat = if is_tsconfig { 0.6 } else { 0.35 };
    let ztu = if is_tsconfig { 0.7 } else { 0.45 };
    mix_signals(cat, 0.55, ztu, path_depth_factor(file, ctx))
}

// --- parser + AST helpers ---

fn parse_json(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_json::LANGUAGE.into())
}

/// Top-level `pair` nodes inside the document's root object. Returns
/// `(unquoted_key, start_line_1based, end_line_1based)` for each. If the
/// document root isn't an object (rare but legal), returns empty.
fn top_level_pairs(tree: &Tree, source: &str) -> Vec<(String, usize, usize)> {
    let root = tree.root_node();
    let Some(object) = first_child_of_kind(root, "object") else {
        return Vec::new();
    };
    let mut cur = object.walk();
    let mut out = Vec::new();
    for child in object.children(&mut cur) {
        if child.kind() != "pair" {
            continue;
        }
        let Some(key_node) = first_child_of_kind(child, "string") else {
            continue;
        };
        let key = unquote_string(key_node, source);
        let start = child.start_position().row + 1;
        let end = child.end_position().row + 1;
        out.push((key, start, end));
    }
    out
}

fn first_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cur = node.walk();
    node.children(&mut cur).find(|c| c.kind() == kind)
}

fn unquote_string(node: Node, source: &str) -> String {
    let raw = &source[node.start_byte()..node.end_byte()];
    let trimmed = raw.trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() >= 2 && bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"' {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}
