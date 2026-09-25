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

use std::cell::OnceCell;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, JsonKey};
use crate::content::BatchContent;
use crate::render::Source;
use crate::value::{dependency_roster_value, manifest_identity_value, manifest_operational_value};

use super::code::chunk::chunk_ranges;
use super::workspace::{
    WORKSPACE_MEMBER_IDENTITY_FACTOR, WorkspaceMembership, canonical_member, expand_member_entry,
    member_named_after_root,
};
use super::{
    WalkCtx, first_child_of_kind, fs::files_with_any_extension, gated_whole_file_content,
    path_depth_factor, single_file_lines_content,
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
}

impl JsonState {
    /// `true` iff `file` is a workspace-member `package.json`.
    pub fn is_workspace_member(&self, file: &Path, root: &Path) -> bool {
        self.membership
            .is_member(file, || collect_workspace_members(root))
    }

    /// `true` iff `file` is the primary workspace member — see
    /// [`member_named_after_root`]. That member is the only one in a
    /// directory of its name, so the name and membership identify it.
    pub fn is_primary_workspace_member(&self, file: &Path, root: &Path) -> bool {
        let primary = self.primary_member.get_or_init(|| {
            member_named_after_root(
                root,
                self.membership.members(|| collect_workspace_members(root)),
            )
        });
        fn dir_name(path: &Path) -> Option<&std::ffi::OsStr> {
            path.parent().and_then(Path::file_name)
        }
        primary.as_deref().is_some_and(|primary| {
            dir_name(file) == dir_name(primary) && self.is_workspace_member(file, root)
        })
    }
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
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
            && let Some(batch) = whole_json_batch(&file, ctx)
        {
            out.push(batch);
        }
    }
    out
}

fn whole_json_batch(file: &Path, ctx: &WalkCtx) -> Option<Batch> {
    let content = gated_whole_file_content(file, ctx, WHOLE_BYTE_GATE, WHOLE_LINE_CAP)?;
    Some(Batch {
        key: JsonKey::Whole {
            file: file.to_path_buf(),
        }
        .into(),
        predecessor: None,
        content,
        value: 589.0 * path_depth_factor(file, ctx),
    })
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
    let describes_repository = primary || file.parent() == Some(ctx.root());
    let operational = manifest_operational_value(depth);
    let f = file.to_path_buf();
    let section_kinds = [
        (
            JsonKey::Identity { file: f.clone() },
            manifest_identity_value(identity_scale, depth),
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
            dependency_roster_value(describes_repository, depth),
            is_runtime_dependencies_key,
        ),
    ];
    let sections: Vec<_> = section_kinds
        .into_iter()
        .filter_map(|(key, value, name_match)| {
            section_content(file, &source, &pairs, name_match).map(|content| (key, content, value))
        })
        .collect();
    let overlap_chain = package_sections_share_lines(&pairs);
    // Identity is first in `section_kinds`, so it can only be the head of
    // `sections` — the emission/chain order below relies on that ordering.
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
        if matches!(key, JsonKey::Scripts { .. })
            && !overlap_chain
            && let Some(chunks) = scripts_chunks(file, &source, &tree)
        {
            push_chained_chunks(out, file, chunks, predecessor, value);
            previous = Some(emitted);
            continue;
        }
        out.push(Batch {
            key: emitted.clone(),
            predecessor,
            content,
            value,
        });
        previous = Some(emitted);
    }
}

/// A long `scripts` block's rows in chunks (see [`chunk_ranges`]), the
/// conventional entry points first and the rest in source order: the
/// `"scripts": {` row leads the first chunk and the closing brace ends
/// the last. `None` when it stays one batch.
fn scripts_chunks(file: &Path, source: &Source, tree: &Tree) -> Option<Vec<BatchContent>> {
    let object = first_child_of_kind(tree.root_node(), "object")?;
    let scripts = object_field_value(object, "scripts", source)?;
    let mut cursor = scripts.walk();
    let mut entries: Vec<(usize, usize)> = scripts
        .children(&mut cursor)
        .filter(|child| child.kind() == "pair")
        .map(|pair| (pair.start_position().row + 1, pair.end_position().row + 1))
        .collect();
    let lines: Vec<&str> = source.lines().collect();
    entries.sort_by_key(|&(start, _)| !is_entry_point_script(lines[start - 1]));
    let costs: Vec<usize> = entries
        .iter()
        .map(|&(start, end)| {
            (start..=end)
                .map(|row| crate::tokenizer::count(lines[row - 1]))
                .sum()
        })
        .collect();
    let ranges = chunk_ranges(&costs);
    if ranges.len() < 2 {
        return None;
    }
    let key_row = scripts.parent()?.start_position().row + 1;
    let last = ranges.len() - 1;
    ranges
        .into_iter()
        .enumerate()
        .map(|(index, range)| {
            let mut rows: Vec<usize> = entries[range]
                .iter()
                .flat_map(|&(start, end)| start..=end)
                .collect();
            if index == 0 {
                rows.insert(0, key_row);
            }
            if index == last {
                rows.push(scripts.end_position().row + 1);
            }
            rows.sort_unstable();
            rows.dedup();
            single_file_lines_content(file, source, rows)
        })
        .collect()
}

/// A script row named for one of the conventional entry points.
fn is_entry_point_script(row: &str) -> bool {
    let name = row.trim_start().trim_start_matches('"');
    [
        "build",
        "test",
        "lint",
        "dev",
        "start",
        "check",
        "typecheck",
        "format",
    ]
    .iter()
    .any(|entry_point| {
        name.strip_prefix(entry_point)
            .is_some_and(|rest| rest.starts_with('"'))
    })
}

/// Emit `chunks` as `Scripts` then `ScriptsTail`s, each gated on the one
/// before, all at the unsplit block's value.
fn push_chained_chunks(
    out: &mut Vec<Batch>,
    file: &Path,
    chunks: Vec<BatchContent>,
    mut predecessor: Option<BatchKey>,
    value: f64,
) {
    for (chunk, content) in chunks.into_iter().enumerate() {
        let file = file.to_path_buf();
        let key = BatchKey::Json(if chunk == 0 {
            JsonKey::Scripts { file }
        } else {
            JsonKey::ScriptsTail { file, chunk }
        });
        out.push(Batch {
            key: key.clone(),
            predecessor: predecessor.replace(key),
            content,
            value,
        });
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
    single_file_lines_content(file, source, lines)
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

    /// A long `scripts` block delivers as a chain of chunks behind the
    /// identity block; a short one stays one batch.
    #[test]
    fn walker_json_long_scripts_block_chains_chunks() {
        let scripts_keys = |script_count: usize| {
            let dir = tempfile::tempdir().unwrap();
            let scripts: Vec<String> = (0..script_count)
                .map(|i| format!("    \"task-{i}\": \"node scripts/run-task.js --step {i}\""))
                .collect();
            let body = format!(
                "{{\n  \"name\": \"x\",\n  \"scripts\": {{\n{}\n  }}\n}}\n",
                scripts.join(",\n")
            );
            write_pkg(dir.path(), &body);
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
        assert_eq!(scripts_keys(3).len(), 1);
        let chunks = scripts_keys(40);
        assert!(chunks.len() > 1);
        assert!(matches!(
            chunks[0].1,
            Some(BatchKey::Json(JsonKey::Identity { .. }))
        ));
        for pair in chunks.windows(2) {
            assert_eq!(pair[1].1.as_ref(), Some(&pair[0].0));
        }
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

    /// Mixed declaration:
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
