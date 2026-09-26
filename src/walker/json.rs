//! JSON walker. `package.json` splits along the same ontology as
//! `Cargo.toml` (identity / scripts ≈ features / dependencies) plus
//! JS-specific `Entry` and `Runtime` batches for entrypoint pointers
//! (`main`/`module`/`exports`/…) and runtime constraints. Other JSON —
//! tool config (`tsconfig.json`, `.eslintrc.json`, …) and data — is left
//! to the listing and the floor. The full `package.json` key→batch
//! mapping lives in the `is_*_key` predicates below.

use std::cell::OnceCell;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, JsonKey};
use crate::content::BatchContent;
use crate::render::Source;
use crate::value::{dependency_roster_value, manifest_identity_value, manifest_operational_value};

use super::workspace::{
    WORKSPACE_MEMBER_IDENTITY_FACTOR, WorkspaceMembership, canonical_member, expand_member_entry,
    member_named_after_root,
};
use super::{
    WalkCtx, first_child_of_kind, fs::files_with_any_extension, path_depth_factor,
    single_file_lines_content,
};

/// Per-run JSON-walker state — caches the seed root's JS/TS workspace
/// member set (npm/yarn `workspaces` + `pnpm-workspace.yaml`).
#[derive(Default)]
pub struct JsonState {
    membership: WorkspaceMembership,
    primary_member: OnceCell<Option<PathBuf>>,
}

impl JsonState {
    /// `true` iff `file` is a workspace-member `package.json`.
    pub fn is_workspace_member(&self, file: &Path, ctx: &WalkCtx) -> bool {
        self.membership
            .is_member(file, || collect_workspace_members(ctx))
    }

    /// `true` iff `file` is the primary workspace member — see
    /// [`member_named_after_root`]. That member is the only one in a
    /// directory of its name, so the name and membership identify it.
    pub fn is_primary_workspace_member(&self, file: &Path, ctx: &WalkCtx) -> bool {
        let primary = self.primary_member.get_or_init(|| {
            member_named_after_root(
                ctx.root(),
                self.membership.members(|| collect_workspace_members(ctx)),
            )
        });
        primary.as_deref().is_some_and(|primary| {
            dir_name(file) == dir_name(primary) && self.is_workspace_member(file, ctx)
        })
    }
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let mut out = Vec::new();
    for file in files_with_any_extension(dir, &["json"], ctx) {
        if file
            .file_name()
            .is_some_and(|name| name.eq_ignore_ascii_case(PACKAGE_JSON_FILENAME))
        {
            emit_package_json(&file, ctx, &mut out);
        }
    }
    out
}

/// Emit independently purchasable `package.json` surfaces. On ordinary
/// multi-line manifests every section hangs directly off Identity.
/// Compact manifests whose sections share a physical line retain a local
/// chain because the scheduler only permits line overlap along predecessor
/// ancestry. The `devDependencies` / `peerDependencies` rosters are left
/// for an explicit read.
fn emit_package_json(file: &Path, ctx: &WalkCtx, out: &mut Vec<Batch>) {
    let Some((source, tree)) = ctx.parse_tree(file, &tree_sitter_json::LANGUAGE.into()) else {
        return;
    };
    let pairs = top_level_pairs(&tree, &source);
    // The primary workspace member prices like the root manifest.
    let primary = ctx.is_primary_js_workspace_member(file);
    let depth = if primary {
        1.0
    } else {
        path_depth_factor(file, ctx)
    };
    let identity_scale = if ctx.is_js_workspace_member(file) && !primary {
        WORKSPACE_MEMBER_IDENTITY_FACTOR
    } else {
        1.0
    };
    let identity_value = manifest_identity_value(identity_scale, depth);
    let operational = manifest_operational_value(depth);
    let f = file.to_path_buf();
    let section_kinds = [
        (
            JsonKey::Identity { file: f.clone() },
            identity_value,
            is_identity_key as fn(&str) -> bool,
        ),
        (
            JsonKey::Entry { file: f.clone() },
            operational,
            is_entry_key,
        ),
        (
            JsonKey::Runtime { file: f.clone() },
            operational,
            is_runtime_key,
        ),
        (
            JsonKey::Scripts { file: f.clone() },
            operational,
            is_scripts_key,
        ),
        (
            JsonKey::Dependencies { file: f },
            dependency_roster_value(depth),
            is_runtime_dependencies_key,
        ),
    ];
    // A private root manifest publishes nothing, so its identity says
    // little; its entry-point scripts say how to build, test and run the
    // project, and ride at the identity's value.
    let private_root = ctx.depth_from_root(file) == 1
        && first_child_of_kind(tree.root_node(), "object")
            .and_then(|object| object_field_value(object, "private", &source))
            .is_some_and(|value| value.kind() == "true");
    let overlap_chain = package_sections_share_lines(&pairs);
    let mut split_scripts = if overlap_chain {
        None
    } else {
        entry_point_scripts_split(file, &source, &tree)
    };
    // Identity is first in `section_kinds`, so every later section sees
    // whether it was emitted.
    let mut identity = None;
    let mut previous = None;
    for (key, value, name_match) in section_kinds {
        let is_identity = matches!(key, JsonKey::Identity { .. });
        let emitted = BatchKey::Json(key.clone());
        let predecessor = if is_identity {
            None
        } else if overlap_chain {
            previous.clone()
        } else {
            identity.clone()
        };
        if matches!(key, JsonKey::Scripts { .. })
            && let Some((entry_points, rest)) = split_scripts.take()
        {
            out.push(Batch {
                key: emitted.clone(),
                predecessor,
                content: entry_points,
                value: if private_root { identity_value } else { value },
            });
            out.push(Batch {
                key: JsonKey::ScriptsTail {
                    file: file.to_path_buf(),
                }
                .into(),
                predecessor: Some(emitted.clone()),
                content: rest,
                value,
            });
        } else if let Some(content) = section_content(file, &source, &pairs, name_match) {
            out.push(Batch {
                key: emitted.clone(),
                predecessor,
                content,
                value,
            });
        } else {
            continue;
        }
        if is_identity {
            identity = Some(emitted.clone());
        }
        previous = Some(emitted);
    }
}

/// A `scripts` block split in two: the `"scripts": {` row and the
/// conventional entry points, then the other scripts through the closing
/// brace. `None` when either half has no script or the halves share a row.
fn entry_point_scripts_split(
    file: &Path,
    source: &Source,
    tree: &Tree,
) -> Option<(BatchContent, BatchContent)> {
    let object = first_child_of_kind(tree.root_node(), "object")?;
    let scripts = object_field_value(object, "scripts", source)?;
    let mut entry_points = vec![scripts.parent()?.start_position().row + 1];
    let mut rest = vec![scripts.end_position().row + 1];
    let mut cursor = scripts.walk();
    for pair in scripts
        .children(&mut cursor)
        .filter(|child| child.kind() == "pair")
    {
        let (start, end) = (pair.start_position().row + 1, pair.end_position().row + 1);
        if is_entry_point_script(source.line(start)?) {
            entry_points.extend(start..=end);
        } else {
            rest.extend(start..=end);
        }
    }
    if entry_points.len() == 1
        || rest.len() == 1
        || entry_points.iter().any(|row| rest.contains(row))
    {
        return None;
    }
    Some((
        single_file_lines_content(file, source, entry_points)?,
        single_file_lines_content(file, source, rest)?,
    ))
}

/// A script row named for one of the conventional entry points.
fn is_entry_point_script(row: &str) -> bool {
    row.trim_start()
        .strip_prefix('"')
        .and_then(|rest| rest.split_once('"'))
        .is_some_and(|(name, _)| is_entry_point_script_name(name))
}

/// A task or script name for one of the conventional entry points, which
/// say how to build, test and run a project.
pub(super) fn is_entry_point_script_name(name: &str) -> bool {
    matches!(
        name,
        "build" | "test" | "lint" | "dev" | "start" | "check" | "typecheck" | "format"
    )
}

fn section_content(
    file: &Path,
    source: &Source,
    pairs: &[(String, usize, usize)],
    name_match: fn(&str) -> bool,
) -> Option<BatchContent> {
    let lines = pairs
        .iter()
        .filter(|(name, _, _)| name_match(name))
        .flat_map(|(_, start, end)| *start..=*end)
        .collect();
    single_file_lines_content(file, source, lines)
}

fn dir_name(path: &Path) -> Option<&std::ffi::OsStr> {
    path.parent().and_then(Path::file_name)
}

// --- key classification ---

fn is_identity_key(k: &str) -> bool {
    matches!(
        k,
        "name" | "version" | "description" | "license" | "licenses" | "type" | "private"
    )
}

fn is_entry_key(k: &str) -> bool {
    #[rustfmt::skip]
    const KEYS: &[&str] = &[
        "main", "module", "browser", "exports", "imports", "types", "typings", "typesVersions",
        "source", "bin", "files", "directories", "unpkg", "jsdelivr", "umd:main", "jsnext:main",
        "react-native", "svelte", "sideEffects", "workspaces",
    ];
    KEYS.contains(&k)
}

fn is_runtime_key(k: &str) -> bool {
    matches!(k, "engines" | "engineStrict" | "packageManager")
}

fn is_scripts_key(k: &str) -> bool {
    k == "scripts"
}

fn is_runtime_dependencies_key(k: &str) -> bool {
    #[rustfmt::skip]
    const KEYS: &[&str] = &[
        "dependencies", "optionalDependencies", "bundledDependencies", "bundleDependencies",
        "overrides", "resolutions",
    ];
    KEYS.contains(&k)
}

fn is_package_section_key(k: &str) -> bool {
    is_identity_key(k)
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

// --- AST helpers ---

/// `(unquoted_key, start_1based, end_1based)` for each top-level pair.
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

fn unquote_string(node: Node, source: &str) -> String {
    let raw = source[node.byte_range()].trim();
    raw.strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .unwrap_or(raw)
        .to_string()
}

// --- JS/TS workspace-member resolution ---

const PACKAGE_JSON_FILENAME: &str = "package.json";

/// JS/TS workspace members — union of `package.json#workspaces` and
/// `pnpm-workspace.yaml`'s `packages:` list. Only trailing-`/*` globs
/// are honored. Any pnpm `!` negation opts the repo out entirely.
pub(super) fn collect_workspace_members(ctx: &WalkCtx) -> HashSet<PathBuf> {
    let root = ctx.root();
    let Ok(canonical_root) = root.canonicalize() else {
        return HashSet::new();
    };
    let Some(pnpm) = read_pnpm_workspaces(ctx) else {
        return HashSet::new();
    };
    let mut out = HashSet::new();
    for entry in npm_workspaces_entries(ctx).into_iter().chain(pnpm) {
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

/// Raw entries from the `workspaces` field on `<root>/package.json`.
/// Supports both array and object (`{"packages": […]}`) forms.
fn npm_workspaces_entries(ctx: &WalkCtx) -> Vec<String> {
    let Some((text, tree)) = ctx.parse_tree(
        &ctx.root().join(PACKAGE_JSON_FILENAME),
        &tree_sitter_json::LANGUAGE.into(),
    ) else {
        return Vec::new();
    };
    let Some(object) = first_child_of_kind(tree.root_node(), "object") else {
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

/// Read the top-level `packages:` list from `<root>/pnpm-workspace.yaml`.
/// Hand-rolled scanner — the file is a 2–10-line YAML list and pulling
/// in a YAML dependency for one field is overkill. Only an unindented
/// `packages:` key counts; its block list runs to the next unindented
/// key, and a flow list (`[a, b]`) sits on the key's own line.
///
/// `None` on any `!`-prefixed entry — half-supported negation parsing is
/// unsafe across the npm/pnpm union, so the caller treats this as a
/// repo-wide opt-out.
fn read_pnpm_workspaces(ctx: &WalkCtx) -> Option<Vec<String>> {
    let Some(text) = ctx.read_source(&ctx.root().join("pnpm-workspace.yaml")) else {
        return Some(Vec::new());
    };
    let mut values = Vec::new();
    let mut in_packages_block = false;
    for raw_line in text.lines() {
        // A `#` inside a quoted scalar isn't a comment, but workspace
        // entries are paths/globs without one.
        let line = raw_line.split_once('#').map_or(raw_line, |(head, _)| head);
        let trimmed = line.trim_end();
        // A block sequence may sit at its key's own indentation.
        if let Some(item) = trimmed.trim_start().strip_prefix('-') {
            if in_packages_block {
                values.push(item);
            }
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with(char::is_whitespace) {
            continue;
        }
        let packages = trimmed.strip_prefix("packages:").map(str::trim);
        in_packages_block = packages.is_some();
        if let Some(flow) = packages
            .and_then(|value| value.strip_prefix('['))
            .and_then(|value| value.strip_suffix(']'))
        {
            values.extend(flow.split(','));
        }
    }
    let mut entries = Vec::new();
    for value in values {
        let value = value.trim().trim_matches(['"', '\'']);
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

    /// A `scripts` block with both entry-point and other scripts delivers
    /// its entry points first, the rest chained behind them; a block of
    /// one kind, or with both kinds on one row, stays one batch.
    #[test]
    fn json_scripts_split_entry_points_from_the_rest() {
        let scripts_keys = |scripts: &str| {
            let dir = tempfile::tempdir().unwrap();
            write_pkg(
                dir.path(),
                &format!("{{\n  \"name\": \"x\",\n  \"scripts\": {{\n{scripts}\n  }}\n}}\n"),
            );
            let ctx = WalkCtx::new(dir.path().to_path_buf());
            expand_in_dir(dir.path(), &ctx)
                .into_iter()
                .filter(|batch| {
                    matches!(
                        batch.key,
                        BatchKey::Json(JsonKey::Scripts { .. } | JsonKey::ScriptsTail { .. })
                    )
                })
                .map(|batch| (batch.key, batch.predecessor))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            scripts_keys("    \"release\": \"x\",\n    \"bump\": \"y\"").len(),
            1
        );
        assert_eq!(
            scripts_keys("    \"build\": \"x\", \"release\": \"y\"").len(),
            1
        );
        let split = scripts_keys("    \"release\": \"x\",\n    \"build\": \"y\"");
        assert_eq!(split.len(), 2);
        assert!(matches!(
            split[0].1,
            Some(BatchKey::Json(JsonKey::Identity { .. }))
        ));
        assert_eq!(split[1].1.as_ref(), Some(&split[0].0));
    }

    /// A private root manifest's entry-point scripts are a batch of their
    /// own at the identity's value; the other scripts follow at the
    /// operational value.
    #[test]
    fn json_private_root_entry_point_scripts_ride_at_identity_value() {
        let dir = tempfile::tempdir().unwrap();
        write_pkg(
            dir.path(),
            "{\n  \"private\": true,\n  \"scripts\": {\n    \"release\": \"changeset publish\",\n    \"build\": \"tsc -b\",\n    \"test\": \"vitest\"\n  }\n}\n",
        );
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let batches = expand_in_dir(dir.path(), &ctx);
        let value_of = |wanted: fn(&JsonKey) -> bool| {
            batches
                .iter()
                .find_map(|batch| match &batch.key {
                    BatchKey::Json(key) if wanted(key) => Some(batch.value),
                    _ => None,
                })
                .unwrap()
        };
        let identity = value_of(|key| matches!(key, JsonKey::Identity { .. }));
        assert_eq!(
            value_of(|key| matches!(key, JsonKey::Scripts { .. })),
            identity
        );
        assert!(value_of(|key| matches!(key, JsonKey::ScriptsTail { .. })) < identity);
    }

    /// `private` is read from its value, not from whatever else shares
    /// its row.
    #[test]
    fn json_private_false_beside_a_true_value_is_not_private() {
        let dir = tempfile::tempdir().unwrap();
        write_pkg(
            dir.path(),
            "{\"private\": false, \"x-internal\": true,\n  \"scripts\": {\n    \"release\": \"changeset publish\",\n    \"build\": \"tsc -b\",\n    \"test\": \"vitest\"\n  }\n}\n",
        );
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let batches = expand_in_dir(dir.path(), &ctx);
        let scripts = batches
            .iter()
            .find(|batch| matches!(&batch.key, BatchKey::Json(JsonKey::Scripts { .. })))
            .unwrap();
        let tail = batches
            .iter()
            .find(|batch| matches!(&batch.key, BatchKey::Json(JsonKey::ScriptsTail { .. })))
            .unwrap();
        assert_eq!(scripts.value, tail.value);
    }

    #[cfg(unix)]
    #[test]
    fn json_rejects_escaping_family_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("repo");
        fs::create_dir(&root).unwrap();
        let outside = dir.path().join("outside.txt");
        fs::write(&outside, "{\"secret\": \"outside\"}\n").unwrap();

        symlink(&outside, root.join("package.json")).unwrap();

        let ctx = WalkCtx::new(root.clone());
        assert!(
            expand_in_dir(&root, &ctx).is_empty(),
            "package.json discovery must reject symlinks that escape the root",
        );
    }

    #[test]
    fn json_dependency_keys_exclude_dev_and_peer_rosters() {
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
    fn json_only_compact_sections_require_overlap_chain() {
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
    fn json_workspace_members_npm_array_glob_and_literal() {
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
        let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
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
    fn json_primary_member_is_named_after_the_repository() {
        let outer = tempfile::tempdir().unwrap();
        let root = outer.path().join("primary-package");
        fs::create_dir(&root).unwrap();
        write_pkg(
            &root,
            r#"{"name":"workspace-shell","workspaces":["packages/*", "apps/*"]}"#,
        );
        seed_members(&root, &["packages/primary-package", "packages/satellite"]);
        let members = collect_workspace_members(&WalkCtx::new(root.clone()));
        assert_eq!(
            member_named_after_root(&root, &members),
            Some(member_path(&root, "packages/primary-package"))
        );

        seed_members(&root, &["apps/primary-package"]);
        let members = collect_workspace_members(&WalkCtx::new(root.clone()));
        assert_eq!(member_named_after_root(&root, &members), None);
    }

    #[test]
    fn json_workspace_members_yarn_object_form() {
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
        let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
        let expected = member_path(root, "pkg/foo");
        assert!(members.contains(&expected));
    }

    /// Indented, unindented and flow sequences all spell the list.
    #[test]
    fn json_workspace_members_pnpm_yaml_packages_list() {
        for packages in [
            "packages:\n  - 'packages/*'\n  - examples/foo\n",
            "packages:\n- 'packages/*'\n- examples/foo\n",
            "packages: ['packages/*', \"examples/foo\"]\n",
        ] {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path();
            write_pkg(root, r#"{"name": "monorepo"}"#);
            fs::write(
                root.join("pnpm-workspace.yaml"),
                format!("{packages}onlyBuiltDependencies:\n- examples/bar\n"),
            )
            .unwrap();
            seed_members(root, &["packages/a", "examples/foo", "examples/bar"]);
            let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
            for hit in ["packages/a", "examples/foo"] {
                let pkg = member_path(root, hit);
                assert!(
                    members.contains(&pkg),
                    "expected member: {} in {packages:?}",
                    pkg.display()
                );
            }
            let miss = member_path(root, "examples/bar");
            assert!(
                !members.contains(&miss),
                "examples/bar must not be a member in {packages:?}"
            );
        }
    }

    /// pnpm negation (`!packages/foo`) is partially supported only —
    /// rather than expand the listed packages and silently skip the
    /// negation, we opt the entire repo out of JS workspace damping — even
    /// when `package.json#workspaces` lists the excluded package too.
    #[test]
    fn json_workspace_members_pnpm_negation_overrides_npm_workspaces() {
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
        let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
        assert!(
            members.is_empty(),
            "mixed npm+pnpm-with-negation must opt out fully (got {members:?})"
        );
    }

    #[test]
    fn json_workspace_members_no_workspace_declaration() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(root, r#"{"name": "single-package"}"#);
        let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
        assert!(members.is_empty(), "no workspaces field → empty member set");
    }
}
