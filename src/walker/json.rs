//! JSON walker. `package.json` splits along the same ontology as
//! `Cargo.toml` (identity / scripts ≈ features / dependencies) plus
//! JS-specific `Entry` and `Runtime` batches for entrypoint pointers
//! (`main`/`module`/`exports`/…) and runtime constraints. Other small
//! root JSON configs (`tsconfig.json`, `.eslintrc.json`, …) and
//! `*.json5` / `*.code-workspace` files anywhere get a single `Whole`
//! batch; nested `.json` is data (locale tables, fixtures, logs) far more
//! often than config and is left to the listing. Lockfiles and large
//! generated files are skipped. The full `package.json` key→batch mapping
//! lives in the `is_*_key` predicates below.

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, JsonKey};
use crate::content::BatchContent;
use crate::render::Source;
use crate::value::{
    dependency_roster_value, dependency_table_mass_factor, manifest_appendix_value,
    manifest_identity_value, manifest_operational_value,
};

use super::workspace::{
    WORKSPACE_MEMBER_IDENTITY_FACTOR, WorkspaceMembership, canonical_member, expand_member_entry,
    member_named_after_root,
};
use super::{
    FileLines, WalkCtx, dedup_sorted, first_child_of_kind, fs::files_with_any_extension,
    gated_whole_file_content, lines_content_tokens, path_depth_factor, single_file_lines_content,
};

/// Hard cap on `Whole` JSON config rendering — generated files
/// (lockfiles, manifests in node_modules) skip the batch entirely.
const WHOLE_LINE_CAP: usize = 60;

/// FS-metadata pre-flight gate (≈200 bytes/line × line cap) — typical
/// generated JSONs are huge; skip without reading.
const WHOLE_BYTE_GATE: usize = WHOLE_LINE_CAP * 200;

/// Per-run JSON-walker state — caches the seed root's JS/TS workspace
/// member set (npm/yarn `workspaces` + `pnpm-workspace.yaml`).
#[derive(Default)]
pub struct JsonState {
    membership: WorkspaceMembership,
    primary_member: OnceCell<Option<PathBuf>>,
    /// Per-file answers for [`Self::is_primary_workspace_member`] — the
    /// question is asked once per priced section and each miss costs a
    /// `canonicalize` syscall chain.
    primary_member_files: RefCell<HashMap<PathBuf, bool>>,
}

impl JsonState {
    /// `true` iff `file` is a workspace-member `package.json`.
    pub fn is_workspace_member(&self, file: &Path, root: &Path) -> bool {
        self.membership
            .is_member(file, || collect_workspace_members(root))
    }

    /// `true` iff `file` is the primary workspace member — see
    /// [`member_named_after_root`].
    pub fn is_primary_workspace_member(&self, file: &Path, root: &Path) -> bool {
        let primary = self.primary_member.get_or_init(|| {
            member_named_after_root(
                root,
                self.membership.members(|| collect_workspace_members(root)),
            )
        });
        let Some(primary) = primary else {
            return false;
        };
        if let Some(&cached) = self.primary_member_files.borrow().get(file) {
            return cached;
        }
        let is_primary = file.canonicalize().is_ok_and(|file| file == *primary);
        self.primary_member_files
            .borrow_mut()
            .insert(file.to_path_buf(), is_primary);
        is_primary
    }
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    // Keep admission extension-bounded: these are explicit JSON-family
    // formats, not files guessed to be JSON from their contents.
    let json_family_files =
        files_with_any_extension(dir, &["json", "json5", "code-workspace"], ctx);
    if json_family_files.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in json_family_files {
        let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if is_skipped_json(name) {
            continue;
        }
        if is_package_json(name) {
            emit_package_json(&file, ctx, &mut out);
        } else if (dir == ctx.root() || !name.to_ascii_lowercase().ends_with(".json"))
            && let Some(batch) = whole_json_batch(&file, name, ctx)
        {
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
/// multi-line manifests every section hangs directly off Identity, so
/// appendix metadata cannot gate entrypoints, scripts, or dependencies.
/// Compact manifests whose sections share a physical line retain a local
/// chain because the scheduler only permits line overlap along predecessor
/// ancestry. The `devDependencies` / `peerDependencies` rosters are left
/// for an explicit read.
fn emit_package_json(file: &Path, ctx: &WalkCtx, out: &mut Vec<Batch<BatchKey>>) {
    let Some((source, tree)) = parse_json(ctx, file) else {
        return;
    };
    let pairs = top_level_pairs(&tree, &source);
    let mut sections = Vec::new();
    // `mass_graded` marks the dependency rosters, the one section class
    // whose value stops tracking its size — see
    // `crate::value::dependency_table_mass_factor`.
    let mut collect =
        |key: JsonKey, value: f64, name_match: fn(&str) -> bool, mass_graded: bool| {
            let Some(content) = section_content(file, &source, &pairs, name_match) else {
                return;
            };
            let grade = if mass_graded {
                dependency_table_mass_factor(lines_content_tokens(&source, &content))
            } else {
                1.0
            };
            sections.push((key, content, value * grade));
        };
    let f = file.to_path_buf();
    collect(
        JsonKey::Identity { file: f.clone() },
        identity_value(file, ctx),
        is_identity_key,
        false,
    );
    collect(
        JsonKey::Entry { file: f.clone() },
        manifest_operational_value(manifest_depth_factor(file, ctx))
            * secondary_package_json_factor(file),
        is_entry_key,
        false,
    );
    collect(
        JsonKey::Runtime { file: f.clone() },
        manifest_operational_value(manifest_depth_factor(file, ctx))
            * secondary_package_json_factor(file),
        is_runtime_key,
        false,
    );
    collect(
        JsonKey::Scripts { file: f.clone() },
        manifest_operational_value(manifest_depth_factor(file, ctx))
            * secondary_package_json_factor(file),
        is_scripts_key,
        false,
    );
    collect(
        JsonKey::Dependencies { file: f.clone() },
        dependencies_value(file, ctx),
        is_runtime_dependencies_key,
        true,
    );
    collect(
        JsonKey::IdentityMeta { file: f.clone() },
        manifest_appendix_value(path_depth_factor(file, ctx)) * secondary_package_json_factor(file),
        is_identity_meta_key,
        false,
    );
    let overlap_chain = package_sections_share_lines(&pairs);
    // The collect() calls above push Identity first, so it can only be
    // the head of `sections` — the emission/chain order below relies
    // on that ordering.
    let identity = sections
        .first()
        .filter(|(key, _, _)| matches!(key, JsonKey::Identity { .. }))
        .map(|(key, _, _)| BatchKey::Json(key.clone()));
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

fn section_content(
    file: &Path,
    source: &Source,
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
    k == "scripts"
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

fn is_package_section_key(k: &str) -> bool {
    is_identity_key(k)
        || is_identity_meta_key(k)
        || is_entry_key(k)
        || is_runtime_key(k)
        || is_scripts_key(k)
        || is_runtime_dependencies_key(k)
}

/// Whether two emitted section classes claim the same physical source line.
/// This is common for one-line JSON, where independent sibling batches would
/// violate the scheduler's ownership contract.
fn package_sections_share_lines(pairs: &[(String, usize, usize)]) -> bool {
    let emitted: Vec<_> = pairs
        .iter()
        .filter(|(name, _, _)| is_package_section_key(name))
        .collect();
    emitted.iter().enumerate().any(|(i, (_, start, end))| {
        emitted[i + 1..]
            .iter()
            .any(|(_, other_start, other_end)| start <= other_end && other_start <= end)
    })
}

// --- value ---

/// Damp `package.json` when a non-JS root manifest (pyproject.toml or
/// Cargo.toml) sits in the same dir — the JS package is almost
/// certainly a docs/tooling site, not the primary surface.
const SECONDARY_PACKAGE_JSON_FACTOR: f64 = 0.05;

fn secondary_package_json_factor(file: &Path) -> f64 {
    let Some(parent) = file.parent() else {
        return 1.0;
    };
    if parent.join("pyproject.toml").is_file() || parent.join("Cargo.toml").is_file() {
        return SECONDARY_PACKAGE_JSON_FACTOR;
    }
    1.0
}

fn identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let scale = if ctx.is_js_workspace_member(file) && !ctx.is_primary_js_workspace_member(file) {
        WORKSPACE_MEMBER_IDENTITY_FACTOR
    } else {
        1.0
    };
    manifest_identity_value(scale, manifest_depth_factor(file, ctx))
        * secondary_package_json_factor(file)
}

fn dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    dependency_roster_value(
        describes_repository(file, ctx),
        manifest_depth_factor(file, ctx),
    ) * secondary_package_json_factor(file)
}

/// True iff this manifest describes the repository itself — the root
/// `package.json`, or the one publishable workspace member whose name
/// matches the repo.
fn describes_repository(file: &Path, ctx: &WalkCtx) -> bool {
    file.parent() == Some(ctx.root()) || ctx.is_primary_js_workspace_member(file)
}

/// Primary publishable members rank like root manifests for the operational
/// surfaces that establish what the package is and how it ships. Appendix
/// metadata and runtime constraints retain ordinary path-depth pricing.
fn manifest_depth_factor(file: &Path, ctx: &WalkCtx) -> f64 {
    if ctx.is_primary_js_workspace_member(file) {
        1.0
    } else {
        path_depth_factor(file, ctx)
    }
}

fn whole_value(file: &Path, name: &str, ctx: &WalkCtx) -> f64 {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".code-workspace") || lower.ends_with(".json5") {
        // These explicitly admitted sidecars are otherwise the only copy of
        // workspace topology or non-JSON data/config. Their strict size cap
        // keeps this identity-like admission value away from generated data.
        1451.0 * path_depth_factor(file, ctx)
    } else {
        589.0 * path_depth_factor(file, ctx)
    }
}

// --- parser + AST helpers ---

fn parse_json(ctx: &WalkCtx, path: &Path) -> Option<(Arc<Source>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_json::LANGUAGE.into())
}

/// `(unquoted_key, start_1based, end_1based)` for each top-level pair.
fn top_level_pairs(tree: &Tree, source: &str) -> Vec<(String, usize, usize)> {
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
        out.push((key, start, end));
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
    let Some(pnpm) = read_pnpm_workspaces(root) else {
        return HashSet::new();
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

/// Read the top-level `packages:` list from `<root>/pnpm-workspace.yaml`.
/// Hand-rolled scanner — the file is a 2–10-line YAML list and pulling
/// in a YAML dependency for one field is overkill. Assumes a single
/// top-level mapping (the conventional pnpm-workspace.yaml shape); a
/// nested `packages:` would be ignored or, if mis-indented enough to
/// look top-level, would re-trigger the block scan.
///
/// `None` on any `!`-prefixed entry — half-supported negation parsing is
/// unsafe across the npm/pnpm union, so the caller treats this as a
/// repo-wide opt-out.
fn read_pnpm_workspaces(root: &Path) -> Option<Vec<String>> {
    let Ok(text) = std::fs::read_to_string(root.join("pnpm-workspace.yaml")) else {
        return Some(Vec::new());
    };
    let mut entries = Vec::new();
    let mut in_packages_block = false;
    for raw_line in text.lines() {
        // A `#` inside a quoted scalar isn't a comment, but workspace
        // entries are paths/globs without one.
        let line = raw_line.split_once('#').map_or(raw_line, |(head, _)| head);
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }
        if !trimmed.starts_with(char::is_whitespace) {
            in_packages_block = trimmed.starts_with("packages:");
            continue;
        }
        if !in_packages_block {
            continue;
        }
        let Some(rest) = trimmed.trim_start().strip_prefix('-') else {
            continue;
        };
        let value = rest.trim().trim_matches(['"', '\'']);
        if value.starts_with('!') {
            return None;
        }
        if !value.is_empty() {
            entries.push(value.to_string());
        }
    }
    Some(entries)
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

    fn pairs_for_manifest(source: &str) -> Vec<(String, usize, usize)> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source.as_bytes(), None).unwrap();
        top_level_pairs(&tree, source)
    }

    #[test]
    fn walker_json_admits_only_bounded_json_family_sidecars() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("settings.code-workspace"),
            "{\n  \"folders\": []\n}\n",
        )
        .unwrap();
        fs::write(root.join("data.json5"), "{\n  // comment\n  value: 1\n}\n").unwrap();
        fs::write(root.join("not-json.yaml"), "value: 1\n").unwrap();
        fs::write(
            root.join("large.json5"),
            "value\n".repeat(WHOLE_LINE_CAP + 1),
        )
        .unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        let whole_files: HashSet<PathBuf> = batches
            .into_iter()
            .filter_map(|batch| match batch.key {
                BatchKey::Json(JsonKey::Whole { file }) => Some(file),
                _ => None,
            })
            .collect();

        assert_eq!(
            whole_files,
            HashSet::from([
                root.join("data.json5"),
                root.join("settings.code-workspace"),
            ])
        );
    }

    #[cfg(unix)]
    #[test]
    fn walker_json_rejects_in_root_and_escaping_family_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("repo");
        fs::create_dir(&root).unwrap();
        let in_root = root.join("payload.txt");
        let outside = dir.path().join("outside.txt");
        fs::write(&in_root, "{\"secret\": \"inside\"}\n").unwrap();
        fs::write(&outside, "{\"secret\": \"outside\"}\n").unwrap();

        for extension in ["json", "json5", "code-workspace"] {
            symlink(&in_root, root.join(format!("in-root.{extension}"))).unwrap();
            symlink(&outside, root.join(format!("escaping.{extension}"))).unwrap();
        }

        let ctx = WalkCtx::new(root.clone());
        assert!(
            expand_in_dir(&root, &ctx).is_empty(),
            "JSON-family discovery must reject both contained and escaping symlinks",
        );
    }

    #[test]
    fn walker_json_dependency_keys_exclude_dev_and_peer_rosters() {
        for key in [
            "dependencies",
            "optionalDependencies",
            "bundledDependencies",
            "bundleDependencies",
            "overrides",
            "resolutions",
        ] {
            assert!(is_runtime_dependencies_key(key), "runtime key: {key}");
        }
        for key in [
            "devDependencies",
            "peerDependencies",
            "peerDependenciesMeta",
        ] {
            assert!(!is_package_section_key(key), "not emitted: {key}");
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
    fn walker_json_primary_member_is_named_after_the_repository() {
        let outer = tempfile::tempdir().unwrap();
        let root = outer.path().join("primary-package");
        fs::create_dir(&root).unwrap();
        write_pkg(
            &root,
            r#"{"name":"workspace-shell","workspaces":["packages/*", "apps/*"]}"#,
        );
        seed_members(&root, &["packages/primary-package", "packages/satellite"]);
        let members = collect_workspace_members(&root);
        assert_eq!(
            member_named_after_root(&root, &members),
            Some(member_path(&root, "packages/primary-package"))
        );

        seed_members(&root, &["apps/primary-package"]);
        let members = collect_workspace_members(&root);
        assert_eq!(member_named_after_root(&root, &members), None);
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
