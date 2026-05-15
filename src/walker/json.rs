//! JSON walker. `package.json` splits along the same ontology as
//! `Cargo.toml` (identity / scripts ≈ features / dependencies) plus
//! JS-specific `Entry` and `Runtime` batches for entrypoint pointers
//! (`main`/`module`/`exports`/…) and runtime constraints. Other small
//! JSON configs (`tsconfig.json`, `.eslintrc.json`, `jsr.json`,
//! `turbo.json`, …) get a single `Whole` batch; lockfiles and large
//! generated JSONs are skipped. The full key→batch mapping lives in the
//! `is_*_key` predicates below.

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
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

/// Multiplier applied to `Identity` signals on workspace-member
/// `package.json` files. Same axis as the TOML walker's
/// `WORKSPACE_MEMBER_IDENTITY_FACTOR`: in a workspace, a sub-package's
/// `name`/`version`/`description` is mostly inherited or trivially
/// derivable from the root, so it shouldn't crowd out the root manifest
/// or load-bearing source.
const WORKSPACE_MEMBER_IDENTITY_FACTOR: f64 = 0.4;

/// Per-run JSON-walker state owned by [`WalkCtx`]. Caches the seed-root's
/// JS/TS workspace member set so we resolve it at most once per run.
/// Members are determined from the union of `<root>/package.json#workspaces`
/// (npm/yarn) and `<root>/pnpm-workspace.yaml` (pnpm). Pnpm declarations
/// containing a `!`-prefixed (negation) entry are treated as opt-out for
/// pnpm-based damping — see `collect_workspace_members` for the safety
/// rationale.
#[derive(Default)]
pub struct JsonState {
    members: OnceCell<HashSet<PathBuf>>,
    member_lookup: RefCell<HashMap<PathBuf, bool>>,
}

impl JsonState {
    /// `true` iff `file` is a `package.json` declared as a workspace
    /// member by the seed root. Lookups are memoized — one canonicalize
    /// per file across the run.
    pub fn is_workspace_member(&self, file: &Path, root: &Path) -> bool {
        let members = self.members.get_or_init(|| collect_workspace_members(root));
        if members.is_empty() {
            return false;
        }
        if let Some(&hit) = self.member_lookup.borrow().get(file) {
            return hit;
        }
        let hit = file
            .canonicalize()
            .map(|c| members.contains(&c))
            .unwrap_or(false);
        self.member_lookup
            .borrow_mut()
            .insert(file.to_path_buf(), hit);
        hit
    }
}

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

/// Emit the six `package.json` batches chained as
/// `Identity ← IdentityMeta ← Entry ← Runtime ← Scripts ← Dependencies`.
/// The chain exists so a compact one-line `package.json` (where all six
/// key groups collapse onto the same source line) renders through the
/// predecessor-override path instead of tripping a non-ancestor overlap.
/// For typical multi-line files the spans are disjoint and the chain
/// costs nothing.
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
    // Workspace shells named `*-monorepo` are pure orchestration: the
    // root package's name/scripts/deps describe the monorepo as a
    // build target, not any project's API. Damp every batch so the
    // per-member packages and primary-language anchors win the budget.
    let shell_factor =
        if file.parent() == Some(ctx.root()) && package_json_name_ends_with(&source, "-monorepo") {
            0.2
        } else {
            1.0
        };
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
            value: value * shell_factor,
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
        JsonKey::IdentityMeta { file: f.clone() },
        identity_meta_value(file, ctx),
        is_identity_meta_key,
    );
    push(
        JsonKey::Entry { file: f.clone() },
        entry_value(file, ctx),
        is_entry_key,
    );
    push(
        JsonKey::Runtime { file: f.clone() },
        runtime_value(file, ctx),
        is_runtime_key,
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

/// Look up the `"name"` field's string value in a parsed package.json
/// source and check whether it ends with the given suffix. Cheap
/// inline parsing — sufficient for well-formed JSON.
fn package_json_name_ends_with(source: &str, suffix: &str) -> bool {
    let Some(idx) = source.find("\"name\"") else {
        return false;
    };
    let rest = &source[idx + "\"name\"".len()..];
    let Some(colon) = rest.find(':') else {
        return false;
    };
    let after = &rest[colon + 1..];
    let Some(qs) = after.find('"') else {
        return false;
    };
    let value_start = qs + 1;
    let Some(qe) = after[value_start..].find('"') else {
        return false;
    };
    let value = &after[value_start..value_start + qe];
    value.ends_with(suffix)
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
        "name" | "version" | "description" | "license" | "licenses" | "type" | "private"
    )
}

fn is_identity_meta_key(k: &str) -> bool {
    matches!(
        k,
        "author"
            | "authors"
            | "contributors"
            | "repository"
            | "homepage"
            | "bugs"
            | "keywords"
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

fn is_runtime_key(k: &str) -> bool {
    matches!(k, "engines" | "engineStrict" | "packageManager")
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
            | "overrides"
            | "resolutions"
    )
}

// --- value ---

/// A `package.json` is secondary metadata when a Python project
/// manifest (pyproject.toml) or a Rust manifest sits in the same dir.
/// In those layouts the JS package is almost always a docs/tooling
/// site (microbootstrap's `microbootstrap-docs` Vuepress shell, etc.)
/// rather than the primary project surface, so each `package.json`
/// batch should rank below the primary-language batches in the same
/// repo.
const SECONDARY_PACKAGE_JSON_FACTOR: f64 = 0.05;

fn secondary_package_json_factor(file: &Path) -> f64 {
    let Some(parent) = file.parent() else {
        return 1.0;
    };
    if parent.join("pyproject.toml").is_file() || parent.join("Cargo.toml").is_file() {
        SECONDARY_PACKAGE_JSON_FACTOR
    } else {
        1.0
    }
}

fn identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let m = if ctx.is_js_workspace_member(file) {
        WORKSPACE_MEMBER_IDENTITY_FACTOR
    } else {
        1.0
    };
    let s = secondary_package_json_factor(file);
    mix_signals(m, 0.7 * m, 0.85 * m, path_depth_factor(file, ctx)) * s
}

fn identity_meta_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // IdentityMeta (author/homepage/keywords/license/repository) is
    // appendix-shape: NS authors anchor on it occasionally, but most
    // anchor only on the core identity block. Dropped cat 0.6 → 0.4
    // so primary-source batches reliably win the early budget across
    // fixtures.
    let m = if ctx.is_js_workspace_member(file) {
        WORKSPACE_MEMBER_IDENTITY_FACTOR
    } else {
        1.0
    };
    let s = secondary_package_json_factor(file);
    mix_signals(0.4 * m, 0.5 * m, 0.5 * m, path_depth_factor(file, ctx)) * s
}

fn entry_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Lowered against ts-pattern + cmdk + d2ts divergence evidence:
    // the `exports` / `main` / `module` / `types` keys were arriving
    // ahead of README sections the NS values, but rarely match an NS
    // anchor themselves (cmdk NS 1.4 / 1.8 anchor on Dependencies +
    // Scripts, not Entry). A missed Entry block is not a
    // catastrophic-omission risk — `precis` users can re-read the
    // file at trivial cost.
    mix_signals(0.55, 0.55, 0.45, path_depth_factor(file, ctx))
        * secondary_package_json_factor(file)
}

fn runtime_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.53, 0.55, 0.48, path_depth_factor(file, ctx))
        * secondary_package_json_factor(file)
}

fn scripts_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Same calibration story as `entry_value`. Build / test / lint
    // commands describe the package's workflow rather than its public
    // surface, so they don't earn a top-rank slot — but cmdk NS 1.8
    // (Root scripts) does pin Scripts as tier-1, so we don't drop the
    // weight as far as Entry.
    mix_signals(0.5, 0.6, 0.45, path_depth_factor(file, ctx)) * secondary_package_json_factor(file)
}

fn dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.25, 0.5, 0.25, path_depth_factor(file, ctx)) * secondary_package_json_factor(file)
}

fn whole_value(file: &Path, name: &str, ctx: &WalkCtx) -> f64 {
    // tsconfig.json sits at the top of the public-facing tooling — rate it
    // just under package identity. Other configs are mid-rank. Sub-flavor
    // tsconfigs (tsconfig.ts.json, tsconfig.build.json) are build-specific
    // and don't carry the project's TS dialect like the root tsconfig
    // does, so they get the lower mid-rank weight.
    let lower = name.to_ascii_lowercase();
    let is_root_tsconfig = lower == "tsconfig.json";
    let cat = if is_root_tsconfig { 0.55 } else { 0.3 };
    let ztu = if is_root_tsconfig { 0.7 } else { 0.45 };
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

// --- JS/TS workspace-member resolution ---

/// Resolve the seed-root's declared JS/TS workspace members (npm/yarn
/// `workspaces` field on the root `package.json`, plus pnpm's
/// `pnpm-workspace.yaml`) and return their absolute `package.json`
/// paths. The set is the union across both sources.
///
/// Honest scope (intentional false-negatives — a missed member just
/// means we don't damp; we never damp a non-member):
/// - Only trailing-`/*` globs are honored. Mid-name (`packages/foo-*`)
///   and `?` patterns are not.
/// - `pnpm-workspace.yaml` is read with a tiny hand-rolled scanner
///   (top-level `packages:` list). If any entry begins with `!` —
///   pnpm's negation syntax — we **opt the entire repo out of JS
///   workspace damping** (return an empty set, regardless of any
///   `package.json#workspaces` declaration). Pnpm's negation is the
///   source of truth for which packages are excluded; expanding the
///   non-negated globs while skipping the negation would silently
///   reinstate excluded packages via the npm union path.
/// - Path entries with `..` or absolute paths are skipped.
/// - Returns empty set on any read/parse error.
pub(super) fn collect_workspace_members(root: &Path) -> HashSet<PathBuf> {
    let Ok(canonical_root) = root.canonicalize() else {
        return HashSet::new();
    };
    let pnpm = match read_pnpm_workspaces(root) {
        PnpmWorkspaces::Negated => return HashSet::new(),
        PnpmWorkspaces::Entries(v) => v,
    };
    let mut out = HashSet::new();
    for entry in npm_workspaces_entries(root).into_iter().chain(pnpm) {
        for dir in expand_member_entry(root, &entry) {
            if let Some(member) = canonical_member_manifest(&canonical_root, &dir) {
                out.insert(member);
            }
        }
    }

    if let Ok(canonical_root_manifest) = root.join("package.json").canonicalize() {
        out.remove(&canonical_root_manifest);
    }
    out
}

/// Read the npm/yarn `workspaces` field from `<root>/package.json`.
/// Supports both array form (`"workspaces": ["packages/*"]`) and object
/// form (`"workspaces": { "packages": [...] }`). Returns the list of
/// raw entry strings; resolution is downstream.
///
/// Parses with the same `tree-sitter-json` grammar the rest of the
/// walker uses. Reads the file directly rather than going through
/// `WalkCtx` because the resolver runs at first-query time and
/// shouldn't pollute the source/parse caches with the workspace root
/// (which already has its own batches scheduled by then).
fn npm_workspaces_entries(root: &Path) -> Vec<String> {
    let manifest = root.join("package.json");
    let Ok(text) = std::fs::read_to_string(&manifest) else {
        return Vec::new();
    };
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .is_err()
    {
        return Vec::new();
    }
    let Some(tree) = parser.parse(text.as_bytes(), None) else {
        return Vec::new();
    };
    let root_node = tree.root_node();
    let Some(object) = first_child_of_kind(root_node, "object") else {
        return Vec::new();
    };
    let Some(workspaces_value) = object_field_value(object, "workspaces", &text) else {
        return Vec::new();
    };
    // Array form: ["pkg/a", "pkg/b"].
    if workspaces_value.kind() == "array" {
        return collect_string_array(workspaces_value, &text);
    }
    // Object form: { "packages": [...] }.
    if workspaces_value.kind() == "object"
        && let Some(packages) = object_field_value(workspaces_value, "packages", &text)
        && packages.kind() == "array"
    {
        return collect_string_array(packages, &text);
    }
    Vec::new()
}

/// Find the `value` node of an object child `pair` whose unquoted key
/// matches `name`.
fn object_field_value<'a>(object: Node<'a>, name: &str, source: &str) -> Option<Node<'a>> {
    let mut cur = object.walk();
    for child in object.children(&mut cur) {
        if child.kind() != "pair" {
            continue;
        }
        let key_node = first_child_of_kind(child, "string")?;
        if unquote_string(key_node, source) != name {
            continue;
        }
        return child.child_by_field_name("value");
    }
    None
}

fn collect_string_array(array: Node, source: &str) -> Vec<String> {
    let mut cur = array.walk();
    array
        .children(&mut cur)
        .filter(|c| c.kind() == "string")
        .map(|c| unquote_string(c, source))
        .collect()
}

/// Outcome of reading `pnpm-workspace.yaml`. Distinguishes "no negation,
/// here are the entries" from "negation present, opt the repo out of
/// damping" so [`collect_workspace_members`] can short-circuit before
/// it would otherwise union pnpm's positive globs with `package.json#workspaces`.
enum PnpmWorkspaces {
    Entries(Vec<String>),
    Negated,
}

/// Read the top-level `packages:` list from `<root>/pnpm-workspace.yaml`.
/// Hand-rolled scanner — the file is a 2–10-line YAML list and pulling
/// in a YAML dependency for one field is overkill. Assumes a single
/// top-level mapping (the conventional pnpm-workspace.yaml shape); a
/// nested `packages:` would be ignored or, if mis-indented enough to
/// look top-level, would re-trigger the block scan.
///
/// Returns [`PnpmWorkspaces::Negated`] on any `!`-prefixed entry —
/// half-supported negation parsing is unsafe across the npm/pnpm
/// union, so the caller treats this as a repo-wide opt-out.
fn read_pnpm_workspaces(root: &Path) -> PnpmWorkspaces {
    let path = root.join("pnpm-workspace.yaml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return PnpmWorkspaces::Entries(Vec::new());
    };
    let mut entries = Vec::new();
    let mut in_packages_block = false;
    for raw_line in text.lines() {
        let line = strip_yaml_comment(raw_line);
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }
        let indented = trimmed.starts_with(char::is_whitespace);
        if !indented {
            in_packages_block = trimmed.trim_start().starts_with("packages:");
            continue;
        }
        if !in_packages_block {
            continue;
        }
        let item = trimmed.trim_start();
        let Some(rest) = item.strip_prefix('-') else {
            continue;
        };
        let value = unquote_yaml_scalar(rest.trim());
        if value.is_empty() {
            continue;
        }
        if value.starts_with('!') {
            return PnpmWorkspaces::Negated;
        }
        entries.push(value);
    }
    PnpmWorkspaces::Entries(entries)
}

fn strip_yaml_comment(line: &str) -> &str {
    // Honest cap: a `#` inside a quoted scalar isn't a comment, but
    // pnpm-workspace.yaml entries are paths/globs without `#` and we
    // don't try to parse arbitrary YAML.
    line.split_once('#').map(|(head, _)| head).unwrap_or(line)
}

fn unquote_yaml_scalar(raw: &str) -> String {
    let bytes = raw.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return raw[1..raw.len() - 1].to_string();
        }
    }
    raw.to_string()
}

/// Expand a single workspace entry against `<root>`. Supports literal
/// paths (`./packages/foo`) and trailing-`/*` globs (`packages/*`).
/// Returns directory paths whose `package.json` may then exist;
/// existence is checked downstream by `canonical_member_manifest`.
fn expand_member_entry(root: &Path, entry: &str) -> Vec<PathBuf> {
    let trimmed = entry.trim_start_matches("./");
    if let Some(prefix) = trimmed.strip_suffix("/*") {
        let parent = root.join(prefix);
        let Ok(read) = std::fs::read_dir(&parent) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for entry in read.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.push(path);
            }
        }
        out
    } else if trimmed.contains('*') || trimmed.contains('?') {
        // Unsupported glob shape; honest no-op.
        Vec::new()
    } else {
        vec![root.join(trimmed)]
    }
}

/// Resolve a member directory to its canonical `package.json`,
/// requiring the file to exist and to live under `canonical_root`
/// (no `..` escape, no absolute override).
fn canonical_member_manifest(canonical_root: &Path, dir: &Path) -> Option<PathBuf> {
    let canonical_manifest = dir.join("package.json").canonicalize().ok()?;
    canonical_manifest
        .starts_with(canonical_root)
        .then_some(canonical_manifest)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn write_pkg(dir: &Path, body: &str) {
        fs::write(dir.join("package.json"), body).unwrap();
    }

    #[test]
    fn walker_json_workspace_members_npm_array_glob_and_literal() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(
            root,
            r#"{
                "name": "monorepo",
                "private": true,
                "workspaces": ["packages/*", "apps/web"]
            }"#,
        );
        for sub in ["packages/a", "packages/b", "apps/web", "apps/native"] {
            let p = root.join(sub);
            fs::create_dir_all(&p).unwrap();
            write_pkg(&p, r#"{"name": "x"}"#);
        }
        let members = collect_workspace_members(root);
        for hit in ["packages/a", "packages/b", "apps/web"] {
            let pkg = root.join(hit).join("package.json").canonicalize().unwrap();
            assert!(members.contains(&pkg), "expected member: {}", pkg.display());
        }
        let miss = root
            .join("apps/native/package.json")
            .canonicalize()
            .unwrap();
        assert!(
            !members.contains(&miss),
            "apps/native must not be a member (not listed)"
        );
        let root_pkg = root.join("package.json").canonicalize().unwrap();
        assert!(
            !members.contains(&root_pkg),
            "root package.json must be excluded from member set"
        );
    }

    #[test]
    fn walker_json_workspace_members_yarn_object_form() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(
            root,
            r#"{
                "name": "monorepo",
                "workspaces": { "packages": ["pkg/*"] }
            }"#,
        );
        let sub = root.join("pkg/foo");
        fs::create_dir_all(&sub).unwrap();
        write_pkg(&sub, r#"{"name": "foo"}"#);
        let members = collect_workspace_members(root);
        let expected = sub.join("package.json").canonicalize().unwrap();
        assert!(members.contains(&expected));
    }

    #[test]
    fn walker_json_workspace_members_pnpm_yaml_packages_list() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(root, r#"{"name": "monorepo"}"#);
        fs::write(
            root.join("pnpm-workspace.yaml"),
            "packages:\n  - 'packages/*'\n  - examples/foo\n",
        )
        .unwrap();
        for sub in ["packages/a", "examples/foo", "examples/bar"] {
            let p = root.join(sub);
            fs::create_dir_all(&p).unwrap();
            write_pkg(&p, r#"{"name": "x"}"#);
        }
        let members = collect_workspace_members(root);
        for hit in ["packages/a", "examples/foo"] {
            let pkg = root.join(hit).join("package.json").canonicalize().unwrap();
            assert!(members.contains(&pkg), "expected member: {}", pkg.display());
        }
        let miss = root
            .join("examples/bar/package.json")
            .canonicalize()
            .unwrap();
        assert!(
            !members.contains(&miss),
            "examples/bar must not be a member"
        );
    }

    /// pnpm negation (`!packages/foo`) is partially supported only —
    /// rather than expand the listed packages and silently skip the
    /// negation, we opt the entire repo out of JS workspace damping.
    #[test]
    fn walker_json_workspace_members_pnpm_negation_opts_out() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(root, r#"{"name": "monorepo"}"#);
        fs::write(
            root.join("pnpm-workspace.yaml"),
            "packages:\n  - 'packages/*'\n  - '!packages/excluded'\n",
        )
        .unwrap();
        for sub in ["packages/a", "packages/excluded"] {
            let p = root.join(sub);
            fs::create_dir_all(&p).unwrap();
            write_pkg(&p, r#"{"name": "x"}"#);
        }
        let members = collect_workspace_members(root);
        assert!(
            members.is_empty(),
            "pnpm negation present → empty member set (got {members:?})"
        );
    }

    /// Mixed declaration (Codex adversarial review regression guard):
    /// `package.json#workspaces` lists `packages/*` and
    /// `pnpm-workspace.yaml` excludes one of them. The npm-source union
    /// path must not silently re-include the excluded package.
    #[test]
    fn walker_json_workspace_members_pnpm_negation_overrides_npm_workspaces() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(
            root,
            r#"{
                "name": "monorepo",
                "workspaces": ["packages/*"]
            }"#,
        );
        fs::write(
            root.join("pnpm-workspace.yaml"),
            "packages:\n  - 'packages/*'\n  - '!packages/excluded'\n",
        )
        .unwrap();
        for sub in ["packages/a", "packages/excluded"] {
            let p = root.join(sub);
            fs::create_dir_all(&p).unwrap();
            write_pkg(&p, r#"{"name": "x"}"#);
        }
        let members = collect_workspace_members(root);
        assert!(
            members.is_empty(),
            "mixed npm+pnpm-with-negation must opt out fully (got {members:?})"
        );
    }

    #[test]
    fn walker_json_workspace_members_no_workspace_declaration() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(root, r#"{"name": "single-package"}"#);
        let members = collect_workspace_members(root);
        assert!(members.is_empty(), "no workspaces field → empty member set");
    }

    /// Mid-name globs (`packages/foo-*`) are unsupported; resolver
    /// returns no entries from that line rather than mis-matching.
    #[test]
    fn walker_json_workspace_members_unsupported_mid_name_glob() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(
            root,
            r#"{
                "name": "monorepo",
                "workspaces": ["packages/mdbook-*"]
            }"#,
        );
        let p = root.join("packages/mdbook-core");
        fs::create_dir_all(&p).unwrap();
        write_pkg(&p, r#"{"name": "x"}"#);
        let members = collect_workspace_members(root);
        assert!(
            members.is_empty(),
            "mid-name globs must not match (got {members:?})"
        );
    }
}
