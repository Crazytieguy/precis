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

use crate::batch::{BatchKey, FsKey, JsonKey, ResolvedBatch, ValueSignals};
use crate::value::depth_factor;

use super::{
    Candidate, FileLines, WalkCtx, dedup_sorted, fs::files_with_extension, single_file_lines_batch,
};

/// Hard cap on `Whole` JSON config rendering. Above this, we skip the
/// batch entirely — generated files (lockfiles, .nyc_output, package
/// manifests in node_modules) are pure noise.
const WHOLE_LINE_CAP: usize = 60;

pub fn expand(scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
    let BatchKey::Fs(FsKey::DirListing { dir }) = scheduled else {
        return Vec::new();
    };
    let json_files = files_with_extension(dir, "json");
    if json_files.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in json_files {
        let name = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if is_skipped_json(name) {
            continue;
        }
        if is_package_json(name) {
            // Chain identity → entry → scripts → dependencies so a
            // compact (one-line) package.json — where all four key
            // categories collapse onto the same source line — renders
            // through the predecessor-override path instead of tripping
            // a non-ancestor overlap. Order matches their typical value
            // ranking, so the chain doesn't displace anything in the
            // normal multi-line case.
            let identity = JsonKey::Identity { file: file.clone() };
            out.push(candidate(identity.clone(), identity_signals(&file, ctx)));
            let entry = JsonKey::Entry { file: file.clone() };
            out.push(
                candidate(entry.clone(), entry_signals(&file, ctx))
                    .with_predecessor(BatchKey::Json(identity)),
            );
            let scripts = JsonKey::Scripts { file: file.clone() };
            out.push(
                candidate(scripts.clone(), scripts_signals(&file, ctx))
                    .with_predecessor(BatchKey::Json(entry)),
            );
            out.push(
                candidate(
                    JsonKey::Dependencies { file: file.clone() },
                    dependencies_signals(&file, ctx),
                )
                .with_predecessor(BatchKey::Json(scripts)),
            );
        } else {
            // FS metadata avoids reading the file when the size hint alone
            // already disqualifies it — typical generated JSONs are huge.
            let byte_len = std::fs::metadata(&file)
                .map(|m| m.len() as usize)
                .unwrap_or(usize::MAX);
            if byte_len > WHOLE_LINE_CAP * 200 {
                continue;
            }
            let line_count = ctx
                .read_source(&file)
                .map(|s| s.lines().count())
                .unwrap_or(usize::MAX);
            if line_count > WHOLE_LINE_CAP * 2 {
                continue;
            }
            out.push(candidate(
                JsonKey::Whole { file: file.clone() },
                whole_signals(&file, name, ctx),
            ));
        }
    }
    out
}

pub fn materialize(key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let BatchKey::Json(jk) = key else {
        return None;
    };
    match jk {
        JsonKey::Identity { file } => {
            mat_keys(file, is_identity_key, identity_signals(file, ctx), ctx)
        }
        JsonKey::Entry { file } => mat_keys(file, is_entry_key, entry_signals(file, ctx), ctx),
        JsonKey::Scripts { file } => {
            mat_keys(file, is_scripts_key, scripts_signals(file, ctx), ctx)
        }
        JsonKey::Dependencies { file } => mat_keys(
            file,
            is_dependencies_key,
            dependencies_signals(file, ctx),
            ctx,
        ),
        JsonKey::Whole { file } => mat_whole(file, ctx),
    }
}

fn mat_keys(
    file: &Path,
    matches: impl Fn(&str) -> bool,
    signals: ValueSignals,
    ctx: &WalkCtx,
) -> Option<ResolvedBatch> {
    let (source, tree) = parse_json(ctx, file)?;
    let pairs = top_level_pairs(&tree, &source);
    let mut lines: Vec<usize> = Vec::new();
    for (name, start, end) in pairs {
        if matches(&name) {
            lines.extend(start..=end);
        }
    }
    if lines.is_empty() {
        return None;
    }
    single_file_lines_batch(file, &source, FileLines::new(dedup_sorted(lines)), signals)
}

fn mat_whole(file: &Path, ctx: &WalkCtx) -> Option<ResolvedBatch> {
    let source = ctx.read_source(file)?;
    let line_count = source.lines().count();
    if line_count == 0 || line_count > WHOLE_LINE_CAP {
        return None;
    }
    let lines: Vec<usize> = (1..=line_count).collect();
    let name = file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    single_file_lines_batch(
        file,
        &source,
        FileLines::new(lines),
        whole_signals(file, name, ctx),
    )
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

// --- signals ---

fn candidate(jk: JsonKey, signals: ValueSignals) -> Candidate<BatchKey> {
    Candidate::new(jk.into(), signals)
}

fn signal_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    depth_factor(ctx.depth_from_root(file)) * ctx.non_essential_factor(file)
}

fn identity_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 1.0,
        follow_up_minimization: 0.7,
        zero_tool_call_understanding: 0.85,
        depth_factor: signal_factor(file, ctx),
    }
}

fn entry_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.85,
        follow_up_minimization: 0.85,
        zero_tool_call_understanding: 0.6,
        depth_factor: signal_factor(file, ctx),
    }
}

fn scripts_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.6,
        follow_up_minimization: 0.85,
        zero_tool_call_understanding: 0.55,
        depth_factor: signal_factor(file, ctx),
    }
}

fn dependencies_signals(file: &Path, ctx: &WalkCtx) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: 0.4,
        follow_up_minimization: 0.7,
        zero_tool_call_understanding: 0.4,
        depth_factor: signal_factor(file, ctx),
    }
}

fn whole_signals(file: &Path, name: &str, ctx: &WalkCtx) -> ValueSignals {
    // tsconfig.json sits at the top of the public-facing tooling — rate it
    // just under package identity. Other configs are mid-rank.
    let lower = name.to_ascii_lowercase();
    let is_tsconfig =
        lower == "tsconfig.json" || (lower.starts_with("tsconfig.") && lower.ends_with(".json"));
    let cat = if is_tsconfig { 0.6 } else { 0.35 };
    let zero = if is_tsconfig { 0.7 } else { 0.45 };
    ValueSignals {
        catastrophic_omission: cat,
        follow_up_minimization: 0.55,
        zero_tool_call_understanding: zero,
        depth_factor: signal_factor(file, ctx),
    }
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
