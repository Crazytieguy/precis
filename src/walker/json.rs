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
use crate::render::{Source, visible_full_line};
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
        if is_skipped_json(name) || super::plaintext::is_unparsed_manifest(dir, name, ctx) {
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
        && pairs.iter().any(|(name, start, _)| {
            name == "private"
                && source
                    .line(*start)
                    .is_some_and(|line| line.contains("true"))
        });
    let overlap_chain = package_sections_share_lines(&pairs);
    let mut chunked_scripts = if overlap_chain {
        None
    } else {
        scripts_chunks(file, &source, &tree, private_root)
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
            && let Some(chunks) = chunked_scripts.take()
        {
            let lead_value = if chunks.leads_with_entry_points {
                identity_value
            } else {
                value
            };
            push_chained_chunks(out, file, chunks.contents, predecessor, lead_value, value);
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

/// A `scripts` block's rows as a chain of chunks: the `"scripts": {` row
/// leads the first and the closing brace ends the last.
struct ScriptsChunks {
    contents: Vec<BatchContent>,
    /// The first chunk holds exactly the conventional entry points.
    leads_with_entry_points: bool,
}

/// A `scripts` block's rows in chunks (see [`chunk_ranges`]), the
/// conventional entry points first and the rest in source order. With
/// `entry_points_apart`, the entry points are a chunk of their own. `None`
/// when the block stays one batch.
fn scripts_chunks(
    file: &Path,
    source: &Source,
    tree: &Tree,
    entry_points_apart: bool,
) -> Option<ScriptsChunks> {
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
    let entry_point_count = entries
        .iter()
        .take_while(|&&(start, _)| is_entry_point_script(lines[start - 1]))
        .count();
    let mut charged_rows = HashSet::new();
    let costs: Vec<usize> = entries
        .iter()
        .map(|&(start, end)| {
            (start..=end)
                .filter(|&row| charged_rows.insert(row))
                .map(|row| crate::tokenizer::count(visible_full_line(lines[row - 1])))
                .sum()
        })
        .collect();
    let leads_with_entry_points = entry_points_apart && entry_point_count > 0;
    let ranges = if leads_with_entry_points {
        std::iter::once(0..entry_point_count)
            .chain(
                chunk_ranges(&costs[entry_point_count..])
                    .into_iter()
                    .map(|range| range.start + entry_point_count..range.end + entry_point_count),
            )
            .collect()
    } else {
        chunk_ranges(&costs)
    };
    if ranges.len() < 2 && !leads_with_entry_points {
        return None;
    }
    let key_row = scripts.parent()?.start_position().row + 1;
    let last = ranges.len() - 1;
    let contents = ranges
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
        .collect::<Option<_>>()?;
    Some(ScriptsChunks {
        contents,
        leads_with_entry_points,
    })
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

/// Emit `chunks` as `Scripts` then `ScriptsTail`s, each gated on the one
/// before: the first at `lead_value`, the rest at the unsplit block's value.
fn push_chained_chunks(
    out: &mut Vec<Batch>,
    file: &Path,
    chunks: Vec<BatchContent>,
    mut predecessor: Option<BatchKey>,
    lead_value: f64,
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
            value: if chunk == 0 { lead_value } else { value },
        });
    }
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

// --- file-name predicates ---

fn is_package_json(name: &str) -> bool {
    name.eq_ignore_ascii_case("package.json")
}

fn dir_name(path: &Path) -> Option<&std::ffi::OsStr> {
    path.parent().and_then(Path::file_name)
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
/// in a YAML dependency for one field is overkill. Assumes a single
/// top-level mapping (the conventional pnpm-workspace.yaml shape); a
/// nested `packages:` would be ignored or, if mis-indented enough to
/// look top-level, would re-trigger the block scan.
///
/// `None` on any `!`-prefixed entry — half-supported negation parsing is
/// unsafe across the npm/pnpm union, so the caller treats this as a
/// repo-wide opt-out.
fn read_pnpm_workspaces(ctx: &WalkCtx) -> Option<Vec<String>> {
    let Some(text) = ctx.read_source(&ctx.root().join("pnpm-workspace.yaml")) else {
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
    fn json_admits_only_bounded_json_family_sidecars() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("settings.code-workspace"),
            "{\n  \"folders\": []\n}\n",
        )
        .unwrap();
        fs::write(root.join("data.json5"), "{\n  // comment\n  value: 1\n}\n").unwrap();
        fs::write(root.join("not-json.yaml"), "value: 1\n").unwrap();
        // The plaintext walker's root manifest.
        fs::write(root.join("composer.json"), "{\n  \"name\": \"a/b\"\n}\n").unwrap();
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
    /// identity block; a short one, even with many scripts on one line,
    /// stays one batch.
    #[test]
    fn json_long_scripts_block_chains_chunks() {
        let scripts_keys = |script_count: usize, separator: &str| {
            let dir = tempfile::tempdir().unwrap();
            let scripts: Vec<String> = (0..script_count)
                .map(|i| format!("    \"task-{i}\": \"node scripts/run-task.js --step {i}\""))
                .collect();
            let body = format!(
                "{{\n  \"name\": \"x\",\n  \"scripts\": {{\n{}\n  }}\n}}\n",
                scripts.join(separator)
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
        assert_eq!(scripts_keys(3, ",\n").len(), 1);
        assert_eq!(scripts_keys(10, ", ").len(), 1);
        let chunks = scripts_keys(40, ",\n");
        assert!(chunks.len() > 1);
        assert!(matches!(
            chunks[0].1,
            Some(BatchKey::Json(JsonKey::Identity { .. }))
        ));
        for pair in chunks.windows(2) {
            assert_eq!(pair[1].1.as_ref(), Some(&pair[0].0));
        }
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

    #[cfg(unix)]
    #[test]
    fn json_rejects_escaping_family_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("repo");
        fs::create_dir(&root).unwrap();
        let outside = dir.path().join("outside.txt");
        fs::write(&outside, "{\"secret\": \"outside\"}\n").unwrap();

        for extension in ["json", "json5", "code-workspace"] {
            symlink(&outside, root.join(format!("escaping.{extension}"))).unwrap();
        }

        let ctx = WalkCtx::new(root.clone());
        assert!(
            expand_in_dir(&root, &ctx).is_empty(),
            "JSON-family discovery must reject symlinks that escape the root",
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

    #[test]
    fn json_workspace_members_pnpm_yaml_packages_list() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(root, r#"{"name": "monorepo"}"#);
        fs::write(
            root.join("pnpm-workspace.yaml"),
            "packages:\n  - 'packages/*'\n  - examples/foo\n",
        )
        .unwrap();
        seed_members(root, &["packages/a", "examples/foo", "examples/bar"]);
        let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
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
    fn json_workspace_members_pnpm_negation_opts_out() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pkg(root, r#"{"name": "monorepo"}"#);
        fs::write(
            root.join("pnpm-workspace.yaml"),
            "packages:\n  - 'packages/*'\n  - '!packages/excluded'\n",
        )
        .unwrap();
        seed_members(root, &["packages/a", "packages/excluded"]);
        let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
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

    /// Mid-name globs (`packages/foo-*`) are unsupported; resolver
    /// returns no entries from that line rather than mis-matching.
    #[test]
    fn json_workspace_members_unsupported_mid_name_glob() {
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
        let members = collect_workspace_members(&WalkCtx::new(root.to_path_buf()));
        assert!(
            members.is_empty(),
            "mid-name globs must not match (got {members:?})"
        );
    }
}
