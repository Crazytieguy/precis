//! JSON walker. `package.json` splits along the same ontology as
//! `Cargo.toml` (identity / scripts ≈ features / dependencies) plus
//! JS-specific `Entry` and `Runtime` batches for entrypoint pointers
//! (`main`/`module`/`exports`/…) and runtime constraints. Other small
//! JSON configs (`tsconfig.json`, `.eslintrc.json`, `jsr.json`,
//! `turbo.json`, …) get a single `Whole` batch; lockfiles and large
//! generated JSONs are skipped. The full key→batch mapping lives in the
//! `is_*_key` predicates below.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, JsonKey};
use crate::content::BatchContent;
use crate::value::mix_signals;

use super::workspace::{WorkspaceMembership, canonical_member, expand_member_entry};
use super::{
    FileLines, WalkCtx, dedup_sorted, first_child_of_kind, fs::files_with_extension,
    gated_whole_file_content, path_depth_factor, single_file_lines_content,
};

/// Hard cap on `Whole` JSON config rendering — generated files
/// (lockfiles, manifests in node_modules) skip the batch entirely.
const WHOLE_LINE_CAP: usize = 60;

/// FS-metadata pre-flight gate (≈200 bytes/line × line cap) — typical
/// generated JSONs are huge; skip without reading.
const WHOLE_BYTE_GATE: usize = WHOLE_LINE_CAP * 200;

/// Damp `Identity` signals on workspace-member `package.json` files —
/// sub-package identity is mostly inherited from the root.
const WORKSPACE_MEMBER_IDENTITY_FACTOR: f64 = 0.4;

/// Per-run JSON-walker state — caches the seed root's JS/TS workspace
/// member set (npm/yarn `workspaces` + `pnpm-workspace.yaml`).
#[derive(Default)]
pub struct JsonState {
    membership: WorkspaceMembership,
}

impl JsonState {
    /// `true` iff `file` is a workspace-member `package.json`.
    pub fn is_workspace_member(&self, file: &Path, root: &Path) -> bool {
        self.membership
            .is_member(file, || collect_workspace_members(root))
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
    let content = gated_whole_file_content(file, ctx, WHOLE_BYTE_GATE, WHOLE_LINE_CAP)?;
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

/// Emit independently purchasable `package.json` surfaces. On ordinary
/// multi-line manifests every section hangs directly off Identity, so appendix
/// metadata cannot gate entrypoints, scripts, or runtime dependencies. Compact
/// manifests whose sections share a physical line retain a local chain because
/// the scheduler only permits line overlap along predecessor ancestry.
fn emit_package_json(file: &Path, ctx: &WalkCtx, out: &mut Vec<Batch<BatchKey>>) {
    let Some((source, tree)) = parse_json(ctx, file) else {
        return;
    };
    let pairs = top_level_pairs(&tree, &source);
    // `*-monorepo` shells are pure orchestration; damp so per-member
    // packages and primary-language anchors win the budget.
    let shell_factor = if file.parent() == Some(ctx.root())
        && package_json_name_ends_with(&tree, &source, "-monorepo")
    {
        0.2
    } else {
        1.0
    };
    let manifest_role = package_json_role(&pairs);
    let scripts_deps_factor = manifest_role.scripts_deps_factor();
    let mut sections = Vec::new();
    let mut collect = |key: JsonKey, value: f64, name_match: fn(&str) -> bool| {
        let Some(content) = section_content(file, &source, &pairs, name_match) else {
            return;
        };
        sections.push((key, content, value * shell_factor));
    };
    let f = file.to_path_buf();
    collect(
        JsonKey::Identity { file: f.clone() },
        identity_value(file, ctx),
        is_identity_key,
    );
    collect(
        JsonKey::Entry { file: f.clone() },
        entry_value(file, ctx),
        is_entry_key,
    );
    collect(
        JsonKey::Runtime { file: f.clone() },
        runtime_value(file, ctx),
        is_runtime_key,
    );
    collect(
        JsonKey::Scripts { file: f.clone() },
        scripts_value(file, ctx) * scripts_deps_factor,
        is_scripts_key,
    );
    collect(
        JsonKey::Dependencies { file: f.clone() },
        dependencies_value(file, ctx) * scripts_deps_factor,
        is_runtime_dependencies_key,
    );
    collect(
        JsonKey::IdentityMeta { file: f.clone() },
        identity_meta_value(file, ctx),
        is_identity_meta_key,
    );
    collect(
        JsonKey::DevDependencies { file: f },
        dev_dependencies_value(file, ctx) * scripts_deps_factor,
        is_dev_dependencies_key,
    );

    let overlap_chain = package_sections_share_lines(&pairs);
    let identity = sections
        .iter()
        .find_map(|(key, _, _)| matches!(key, JsonKey::Identity { .. }).then(|| key.clone()))
        .map(BatchKey::Json);
    let mut previous = None;
    for (key, content, value) in sections {
        let emitted = BatchKey::Json(key.clone());
        let predecessor = if matches!(key, JsonKey::Identity { .. }) {
            None
        } else if overlap_chain {
            previous.clone()
        } else {
            identity.clone()
        };
        out.push(Batch {
            key: emitted.clone(),
            predecessor,
            content,
            value,
        });
        previous = Some(emitted);
    }
}

/// Boost factor for `scripts` / `dependencies` on operational
/// `package.json`s (see [`PackageJsonRole`]).
const APP_SCRIPTS_DEPS_FACTOR: f64 = 1.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackageJsonRole {
    MonorepoRoot,
    AppOrCli,
    Library,
    ImplicitEntryPackage,
}

impl PackageJsonRole {
    fn scripts_deps_factor(self) -> f64 {
        match self {
            PackageJsonRole::AppOrCli => APP_SCRIPTS_DEPS_FACTOR,
            PackageJsonRole::MonorepoRoot
            | PackageJsonRole::Library
            | PackageJsonRole::ImplicitEntryPackage => 1.0,
        }
    }
}

/// Classify a manifest by its own top-level content so app/CLI
/// manifests can price run/dependency surfaces without promoting
/// publishable library manifests wholesale.
fn package_json_role(pairs: &[(String, usize, usize, bool)]) -> PackageJsonRole {
    let private_true = pairs
        .iter()
        .find(|(name, _, _, _)| name == "private")
        .is_some_and(|(_, _, _, value_is_true)| *value_is_true);
    let has_bin = pairs.iter().any(|(name, _, _, _)| name == "bin");
    let has_workspaces = pairs.iter().any(|(name, _, _, _)| name == "workspaces");
    let has_files = pairs.iter().any(|(name, _, _, _)| name == "files");
    let has_entry_metadata = pairs.iter().any(|(name, _, _, _)| {
        matches!(
            name.as_str(),
            "main" | "module" | "browser" | "exports" | "types" | "typings" | "files"
        )
    });

    if private_true && has_workspaces {
        return PackageJsonRole::MonorepoRoot;
    }
    if private_true {
        return PackageJsonRole::AppOrCli;
    }
    if has_bin && has_files {
        return PackageJsonRole::Library;
    }
    if has_entry_metadata {
        return PackageJsonRole::Library;
    }
    if has_bin {
        return PackageJsonRole::AppOrCli;
    }
    PackageJsonRole::ImplicitEntryPackage
}

/// True iff the manifest's top-level `"name"` ends with `suffix`.
fn package_json_name_ends_with(tree: &Tree, source: &str, suffix: &str) -> bool {
    first_child_of_kind(tree.root_node(), "object", false)
        .and_then(|o| object_field_value(o, "name", source))
        .is_some_and(|v| v.kind() == "string" && unquote_string(v, source).ends_with(suffix))
}

fn section_content(
    file: &Path,
    source: &str,
    pairs: &[(String, usize, usize, bool)],
    name_match: fn(&str) -> bool,
) -> Option<BatchContent> {
    let mut lines: Vec<usize> = Vec::new();
    for (name, start, end, _) in pairs {
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

/// Files that never produce JSON batches — lockfiles.
fn is_skipped_json(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(lower.as_str(), "package-lock.json" | "npm-shrinkwrap.json")
        || lower.ends_with(".lock.json")
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

fn is_runtime_dependencies_key(k: &str) -> bool {
    matches!(
        k,
        "dependencies"
            | "optionalDependencies"
            | "bundledDependencies"
            | "bundleDependencies"
            | "overrides"
            | "resolutions"
    )
}

fn is_dev_dependencies_key(k: &str) -> bool {
    matches!(
        k,
        "devDependencies" | "peerDependencies" | "peerDependenciesMeta"
    )
}

fn is_package_section_key(k: &str) -> bool {
    is_identity_key(k)
        || is_identity_meta_key(k)
        || is_entry_key(k)
        || is_runtime_key(k)
        || is_scripts_key(k)
        || is_runtime_dependencies_key(k)
        || is_dev_dependencies_key(k)
}

/// Whether two emitted section classes claim the same physical source line.
/// This is common for one-line JSON, where independent sibling batches would
/// violate the scheduler's ownership contract.
fn package_sections_share_lines(pairs: &[(String, usize, usize, bool)]) -> bool {
    let emitted: Vec<_> = pairs
        .iter()
        .filter(|(name, _, _, _)| is_package_section_key(name))
        .collect();
    emitted.iter().enumerate().any(|(i, (_, start, end, _))| {
        emitted[i + 1..]
            .iter()
            .any(|(_, other_start, other_end, _)| start <= other_end && other_start <= end)
    })
}

// --- value ---

/// Damp `package.json` when a non-JS root manifest (pyproject.toml or
/// Cargo.toml) sits in the same dir — the JS package is almost
/// certainly a docs/tooling site, not the primary surface.
const SECONDARY_PACKAGE_JSON_FACTOR: f64 = 0.05;

/// Damp `package.json` inside scaffold-template directories
/// (`templates/` or `template-*`) — they're starter material, not the
/// repo's own API/workflow.
const SCAFFOLD_TEMPLATE_PACKAGE_JSON_FACTOR: f64 = 0.05;

fn secondary_package_json_factor(file: &Path) -> f64 {
    let Some(parent) = file.parent() else {
        return 1.0;
    };
    if parent.join("pyproject.toml").is_file() || parent.join("Cargo.toml").is_file() {
        return SECONDARY_PACKAGE_JSON_FACTOR;
    }
    if is_scaffold_template_path(file) {
        return SCAFFOLD_TEMPLATE_PACKAGE_JSON_FACTOR;
    }
    1.0
}

/// True iff `file` is under a `templates/` / `template-*` / `cra-template-*`
/// ancestor — npm scaffold template content.
fn is_scaffold_template_path(file: &Path) -> bool {
    file.ancestors().any(|anc| {
        anc.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
            n == "templates" || n.starts_with("template-") || n.starts_with("cra-template-")
        })
    })
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

fn dev_dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.24, 0.48, 0.24, path_depth_factor(file, ctx))
        * secondary_package_json_factor(file)
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

/// `(unquoted_key, start_1based, end_1based, value_is_true)` for each top-level pair.
fn top_level_pairs(tree: &Tree, source: &str) -> Vec<(String, usize, usize, bool)> {
    let root = tree.root_node();
    let Some(object) = first_child_of_kind(root, "object", false) else {
        return Vec::new();
    };
    let mut cur = object.walk();
    let mut out = Vec::new();
    for child in object.children(&mut cur) {
        if child.kind() != "pair" {
            continue;
        }
        let Some(key_node) = first_child_of_kind(child, "string", false) else {
            continue;
        };
        let key = unquote_string(key_node, source);
        let start = child.start_position().row + 1;
        let end = child.end_position().row + 1;
        let value_is_true = child
            .child_by_field_name("value")
            .is_some_and(|value| value.kind() == "true");
        out.push((key, start, end, value_is_true));
    }
    out
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

const PACKAGE_JSON_FILENAME: &str = "package.json";

/// JS/TS workspace members — union of `package.json#workspaces` and
/// `pnpm-workspace.yaml`'s `packages:` list. Only trailing-`/*` globs
/// are honored. Any pnpm `!` negation opts the repo out entirely.
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
            if let Some(member) = canonical_member(&canonical_root, &dir, PACKAGE_JSON_FILENAME) {
                out.insert(member);
            }
        }
    }

    if let Ok(canonical_root_manifest) = root.join(PACKAGE_JSON_FILENAME).canonicalize() {
        out.remove(&canonical_root_manifest);
    }
    out
}

/// Parse `<dir>/package.json` into its source text + tree. The caller
/// re-derives the root object node (tree-sitter nodes borrow the tree,
/// so it can't be returned from here). The one manifest-parse prologue —
/// grammar setup or root-node conventions change here, nowhere else.
fn parse_manifest(dir: &Path) -> Option<(String, tree_sitter::Tree)> {
    let text = std::fs::read_to_string(dir.join(PACKAGE_JSON_FILENAME)).ok()?;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(text.as_bytes(), None)?;
    Some((text, tree))
}

/// The `types` / `typings` target declared by `<root>/package.json`,
/// resolved against `root`. `None` when neither field is a string.
pub(super) fn declared_types_target(root: &Path) -> Option<PathBuf> {
    let (text, tree) = parse_manifest(root)?;
    let object = first_child_of_kind(tree.root_node(), "object", false)?;
    ["types", "typings"].iter().find_map(|field| {
        let value = object_field_value(object, field, &text)?;
        if value.kind() != "string" {
            return None;
        }
        let rel = unquote_string(value, &text);
        let rel = rel.strip_prefix("./").unwrap_or(&rel);
        if rel.is_empty() {
            return None;
        }
        Some(root.join(rel))
    })
}

/// String targets of `<pkg_dir>/package.json`'s entry fields — `main`,
/// `module`, and the string leaves under `exports` / `exports["."]`
/// (conditional-export objects are descended; subpath keys other than
/// `"."` are not).
pub(super) fn package_entry_targets(pkg_dir: &Path) -> Vec<String> {
    let Some((text, tree)) = parse_manifest(pkg_dir) else {
        return Vec::new();
    };
    let Some(object) = first_child_of_kind(tree.root_node(), "object", false) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for field in ["main", "module"] {
        if let Some(value) = object_field_value(object, field, &text)
            && value.kind() == "string"
        {
            out.push(unquote_string(value, &text));
        }
    }
    if let Some(exports) = object_field_value(object, "exports", &text) {
        let dot = if exports.kind() == "object" {
            // `"."` is the root subpath; a conditions-only object (no
            // `"./…"` keys) is itself the root entry. Subpath-only maps
            // contribute nothing — their targets aren't the main entry.
            object_field_value(exports, ".", &text)
                .or_else(|| (!object_has_subpath_key(exports, &text)).then_some(exports))
        } else {
            Some(exports)
        };
        if let Some(dot) = dot {
            collect_string_leaves(dot, &text, 3, &mut out);
        }
    }
    out
}

/// True when a JSON object has any key starting with `.` (an exports
/// subpath key, as opposed to a condition name).
fn object_has_subpath_key(object: Node, source: &str) -> bool {
    let mut cur = object.walk();
    object.children(&mut cur).any(|child| {
        child.kind() == "pair"
            && first_child_of_kind(child, "string", false)
                .is_some_and(|key| unquote_string(key, source).starts_with('.'))
    })
}

/// Push every string leaf of a JSON value, descending objects and
/// arrays up to `depth` levels (conditional-export nesting is shallow).
/// Arrays are descended too so the fallback-array export form
/// (`"exports": {".": ["./a.js", "./b.js"]}` or a top-level array)
/// yields its targets.
fn collect_string_leaves(value: Node, source: &str, depth: usize, out: &mut Vec<String>) {
    if value.kind() == "string" {
        out.push(unquote_string(value, source));
        return;
    }
    if depth == 0 {
        return;
    }
    match value.kind() {
        "object" => {
            let mut cur = value.walk();
            for child in value.children(&mut cur) {
                if child.kind() != "pair" {
                    continue;
                }
                if let Some(inner) = child.child_by_field_name("value") {
                    collect_string_leaves(inner, source, depth - 1, out);
                }
            }
        }
        "array" => {
            let mut cur = value.walk();
            for child in value.children(&mut cur) {
                // Skip array punctuation; recurse on element values.
                if child.is_named() {
                    collect_string_leaves(child, source, depth - 1, out);
                }
            }
        }
        _ => {}
    }
}

/// Raw entries from the `workspaces` field on `<root>/package.json`.
/// Supports both array and object (`{"packages": […]}`) forms.
fn npm_workspaces_entries(root: &Path) -> Vec<String> {
    let Some((text, tree)) = parse_manifest(root) else {
        return Vec::new();
    };
    let Some(object) = first_child_of_kind(tree.root_node(), "object", false) else {
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
        let key_node = first_child_of_kind(child, "string", false)?;
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn write_pkg(dir: &Path, body: &str) {
        fs::write(dir.join("package.json"), body).unwrap();
    }

    fn seed_members(root: &Path, subs: &[&str]) {
        for sub in subs {
            let p = root.join(sub);
            fs::create_dir_all(&p).unwrap();
            write_pkg(&p, r#"{"name": "x"}"#);
        }
    }

    fn member_path(root: &Path, rel: &str) -> PathBuf {
        root.join(rel).join("package.json").canonicalize().unwrap()
    }

    fn role_for_manifest(source: &str) -> PackageJsonRole {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source.as_bytes(), None).unwrap();
        let pairs = top_level_pairs(&tree, source);
        package_json_role(&pairs)
    }

    fn pairs_for_manifest(source: &str) -> Vec<(String, usize, usize, bool)> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source.as_bytes(), None).unwrap();
        top_level_pairs(&tree, source)
    }

    #[test]
    fn walker_json_dependency_classes_are_disjoint() {
        for key in [
            "dependencies",
            "optionalDependencies",
            "bundledDependencies",
            "bundleDependencies",
            "overrides",
            "resolutions",
        ] {
            assert!(is_runtime_dependencies_key(key), "runtime key: {key}");
            assert!(!is_dev_dependencies_key(key), "not a dev key: {key}");
        }
        for key in [
            "devDependencies",
            "peerDependencies",
            "peerDependenciesMeta",
        ] {
            assert!(is_dev_dependencies_key(key), "dev/peer key: {key}");
            assert!(!is_runtime_dependencies_key(key), "not runtime: {key}");
        }
    }

    #[test]
    fn walker_json_only_compact_sections_require_overlap_chain() {
        let multiline = pairs_for_manifest(
            "{\n  \"name\": \"demo\",\n  \"scripts\": {\"test\": \"vitest\"},\n  \"dependencies\": {\"react\": \"19\"}\n}\n",
        );
        assert!(!package_sections_share_lines(&multiline));

        let compact = pairs_for_manifest(
            r#"{"name":"demo","scripts":{"test":"vitest"},"dependencies":{"react":"19"}}"#,
        );
        assert!(package_sections_share_lines(&compact));
    }

    #[test]
    fn walker_json_package_role_private_workspace_root_is_neutral() {
        let role = role_for_manifest(
            r#"{
                "private": true,
                "workspaces": ["packages/*"],
                "scripts": {"build": "pnpm -r build"}
            }"#,
        );
        assert_eq!(role, PackageJsonRole::MonorepoRoot);
        assert_eq!(role.scripts_deps_factor(), 1.0);
    }

    #[test]
    fn walker_json_package_role_private_app_with_start_script_is_boosted() {
        let role = role_for_manifest(
            r#"{
                "private": true,
                "module": "./src/main.ts",
                "scripts": {"start": "vite --host 0.0.0.0"}
            }"#,
        );
        assert_eq!(role, PackageJsonRole::AppOrCli);
        assert_eq!(role.scripts_deps_factor(), APP_SCRIPTS_DEPS_FACTOR);
    }

    #[test]
    fn walker_json_package_role_published_cli_with_files_is_library_leaning() {
        let role = role_for_manifest(
            r#"{
                "bin": "./cli.js",
                "files": ["cli.js", "dist"]
            }"#,
        );
        assert_eq!(role, PackageJsonRole::Library);
        assert_eq!(role.scripts_deps_factor(), 1.0);
    }

    #[test]
    fn walker_json_package_role_implicit_entry_package_is_neutral() {
        let role = role_for_manifest(r#"{"name": "plain-package"}"#);
        assert_eq!(role, PackageJsonRole::ImplicitEntryPackage);
        assert_eq!(role.scripts_deps_factor(), 1.0);
    }

    #[test]
    fn walker_json_package_role_module_field_is_library() {
        let role = role_for_manifest(r#"{"module": "./dist/index.mjs"}"#);
        assert_eq!(role, PackageJsonRole::Library);
        assert_eq!(role.scripts_deps_factor(), 1.0);
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
        seed_members(
            root,
            &["packages/a", "packages/b", "apps/web", "apps/native"],
        );
        let members = collect_workspace_members(root);
        for hit in ["packages/a", "packages/b", "apps/web"] {
            let pkg = member_path(root, hit);
            assert!(members.contains(&pkg), "expected member: {}", pkg.display());
        }
        let miss = member_path(root, "apps/native");
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
        seed_members(root, &["pkg/foo"]);
        let members = collect_workspace_members(root);
        let expected = member_path(root, "pkg/foo");
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
        seed_members(root, &["packages/a", "examples/foo", "examples/bar"]);
        let members = collect_workspace_members(root);
        for hit in ["packages/a", "examples/foo"] {
            let pkg = member_path(root, hit);
            assert!(members.contains(&pkg), "expected member: {}", pkg.display());
        }
        let miss = member_path(root, "examples/bar");
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
        seed_members(root, &["packages/a", "packages/excluded"]);
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
        seed_members(root, &["packages/a", "packages/excluded"]);
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
        seed_members(root, &["packages/mdbook-core"]);
        let members = collect_workspace_members(root);
        assert!(
            members.is_empty(),
            "mid-name globs must not match (got {members:?})"
        );
    }
}
