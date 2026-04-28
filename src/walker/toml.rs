//! TOML walker. Uses `tree-sitter-toml-ng` to identify top-level `[table]`
//! headers and their line ranges. Emits one batch per ontology-recognized
//! section group (identity / features / dependencies).
//!
//! Keys:
//! - `Identity { file }` — `[package]`, `[workspace]`, `[workspace.package]`,
//!   `[project]`
//! - `Features { file }` — `[features]`
//! - `Dependencies { file }` — `[dependencies]`, `[dev-dependencies]`,
//!   `[build-dependencies]`, `[workspace.dependencies]`

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, TomlKey};
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, dedup_sorted, fs::files_with_extension, path_depth_factor,
    single_file_lines_content,
};

/// Multiplier applied to `[package]` Identity signals on workspace-member
/// Cargo.tomls. A sub-crate's identity is mostly inherited from the
/// workspace root (`edition.workspace = true` etc.); same axis as the
/// Rust walker's `pub(crate)` damping.
const WORKSPACE_MEMBER_IDENTITY_FACTOR: f64 = 0.4;

const PYPROJECT_LEDE_IDENTITY_FACTOR: f64 = 0.5;
const PYPROJECT_HYBRID_LEDE_IDENTITY_FACTOR: f64 = 0.4;
const PYPROJECT_NON_LEDE_IDENTITY_FACTOR: f64 = 0.1;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let toml_files = files_with_extension(dir, "toml");
    if toml_files.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in toml_files {
        if let Some(content) = build_section_content(&file, ctx, |n| {
            matches!(n, "package" | "workspace" | "workspace.package" | "project")
        }) {
            out.push(Batch {
                key: TomlKey::Identity { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: identity_value(&file, ctx),
            });
        }
        if let Some(content) = build_section_content(&file, ctx, |n| n == "features") {
            out.push(Batch {
                key: TomlKey::Features { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: features_value(&file, ctx),
            });
        }
        if let Some(content) = build_section_content(&file, ctx, |n| {
            matches!(
                n,
                "dependencies"
                    | "dev-dependencies"
                    | "build-dependencies"
                    | "workspace.dependencies"
            )
        }) {
            out.push(Batch {
                key: TomlKey::Dependencies { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: dependencies_value(&file, ctx),
            });
        }
    }
    out
}

fn build_section_content(
    file: &Path,
    ctx: &WalkCtx,
    name_match: impl Fn(&str) -> bool,
) -> Option<crate::content::BatchContent> {
    let (source, tree) = parse_toml(ctx, file)?;
    let sections = collect_sections(&tree, &source);
    let mut line_numbers: Vec<usize> = Vec::new();
    for (name, start_line, end_line) in sections {
        if name_match(&name) {
            if name == "project" {
                line_numbers.extend(project_identity_lines(&source, start_line, end_line));
            } else {
                line_numbers.extend(start_line..=end_line);
            }
        }
    }
    if line_numbers.is_empty() {
        return None;
    }
    single_file_lines_content(file, &source, FileLines::new(dedup_sorted(line_numbers)))
}

fn project_identity_lines(source: &str, start_line: usize, end_line: usize) -> Vec<usize> {
    let mut out = vec![start_line];
    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        if line_no <= start_line || line_no > end_line {
            continue;
        }
        if is_project_scalar_pair_line(line) {
            out.push(line_no);
        }
    }
    out
}

fn is_project_scalar_pair_line(line: &str) -> bool {
    let Some((key, value)) = line.trim_start().split_once('=') else {
        return false;
    };
    let key = key.trim();
    matches!(key, "name" | "description")
        && value
            .trim_start()
            .chars()
            .next()
            .is_some_and(|ch| ch != '[')
}

fn identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let m = if let Some(factor) = pyproject_identity_factor(file, ctx) {
        factor
    } else if ctx.is_workspace_member(file) {
        WORKSPACE_MEMBER_IDENTITY_FACTOR
    } else {
        1.0
    };
    mix_signals(m, 0.7 * m, 0.85 * m, path_depth_factor(file, ctx))
}

fn is_pyproject_manifest(file: &Path) -> bool {
    file.file_name().and_then(|name| name.to_str()) == Some("pyproject.toml")
}

fn pyproject_identity_factor(file: &Path, ctx: &WalkCtx) -> Option<f64> {
    if !is_pyproject_manifest(file) {
        return None;
    }
    let project_is_lede = parse_toml(ctx, file)
        .map(|(source, tree)| {
            collect_sections(&tree, &source)
                .first()
                .is_some_and(|(name, _, _)| name == "project")
        })
        .unwrap_or(false);
    if !project_is_lede {
        return Some(PYPROJECT_NON_LEDE_IDENTITY_FACTOR);
    }
    let has_package_json = file
        .parent()
        .is_some_and(|parent| parent.join("package.json").exists());
    Some(if has_package_json {
        PYPROJECT_HYBRID_LEDE_IDENTITY_FACTOR
    } else {
        PYPROJECT_LEDE_IDENTITY_FACTOR
    })
}

fn features_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.75, 0.6, 0.5, path_depth_factor(file, ctx))
}

fn dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.4, 0.7, 0.4, path_depth_factor(file, ctx))
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

// --- workspace-member resolution ---

/// Resolve the seed-root `Cargo.toml`'s declared workspace members
/// (including auto-promoted local path-dependencies) and return their
/// absolute Cargo.toml paths.
///
/// The set is the **union** of two sources:
/// - explicit `[workspace].members` entries (literals + trailing-`/*`
///   globs); and
/// - top-level `[dependencies]` / `[dev-dependencies]` /
///   `[build-dependencies]` entries with `path = "..."` resolving under
///   `<root>` (Cargo auto-promotes these as members; see
///   <https://doc.rust-lang.org/cargo/reference/workspaces.html#the-members-and-exclude-fields>).
///
/// `[workspace].exclude` is applied **once at the end** to the unified
/// set, so it blocks both explicit members and auto-promoted path deps.
///
/// Honest scope (intentional false-negatives — a missed member just
/// means we don't damp; we never damp a non-member):
/// - Only trailing-`/*` globs are honored. `crates/mdbook-*`,
///   `**/Cargo.toml`, `?` patterns are not.
/// - `[workspace]` is only read from `<root>/Cargo.toml`. Workspace
///   roots elsewhere on disk aren't considered.
/// - Path entries with `..` or absolute paths are skipped.
/// - Returns empty set on any TOML parse error or missing root file.
pub(super) fn collect_workspace_members(root: &Path) -> HashSet<PathBuf> {
    let root_manifest = root.join("Cargo.toml");
    let Ok(text) = std::fs::read_to_string(&root_manifest) else {
        return HashSet::new();
    };
    let Ok(value) = toml::from_str::<toml::Value>(&text) else {
        return HashSet::new();
    };
    // Path-dep auto-promotion only applies inside a Cargo workspace. Without
    // a [workspace] table the manifest is just a regular crate, and damping
    // its path-dep sub-crates would deprioritize legitimate package identity.
    let Some(workspace) = value.get("workspace").and_then(|v| v.as_table()) else {
        return HashSet::new();
    };
    let Ok(canonical_root) = root.canonicalize() else {
        return HashSet::new();
    };

    let collect = |key: &str| -> HashSet<PathBuf> {
        let Some(arr) = workspace.get(key).and_then(|v| v.as_array()) else {
            return HashSet::new();
        };
        let mut out = HashSet::new();
        for entry in arr.iter().filter_map(|v| v.as_str()) {
            for path in expand_member_entry(root, entry) {
                if let Some(member) = canonical_member(&canonical_root, &path) {
                    out.insert(member);
                }
            }
        }
        out
    };

    let mut candidates = collect("members");

    for table_name in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(table) = value.get(table_name).and_then(|v| v.as_table()) else {
            continue;
        };
        for (_dep_name, dep_value) in table.iter() {
            let Some(path_str) = dep_value
                .as_table()
                .and_then(|t| t.get("path"))
                .and_then(|v| v.as_str())
            else {
                continue;
            };
            if let Some(member) = canonical_member(&canonical_root, &root.join(path_str)) {
                candidates.insert(member);
            }
        }
    }

    let excluded = collect("exclude");
    for ex in &excluded {
        candidates.remove(ex);
    }
    if let Ok(canonical_root_manifest) = root_manifest.canonicalize() {
        candidates.remove(&canonical_root_manifest);
    }
    candidates
}

/// Expand a single `members`/`exclude` entry against `<root>`. Supports
/// literal entries (`./crates/foo`, `examples/bar`) and trailing-`/*`
/// globs (`crates/*`). Returns directory paths whose `Cargo.toml` may
/// then exist; existence is checked downstream by `canonical_member`.
fn expand_member_entry(root: &Path, entry: &str) -> Vec<PathBuf> {
    if let Some(prefix) = entry.strip_suffix("/*") {
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
    } else if entry.contains('*') || entry.contains('?') {
        // Unsupported glob shape; honest no-op.
        Vec::new()
    } else {
        vec![root.join(entry)]
    }
}

/// Resolve a member directory to its canonical `Cargo.toml`, requiring
/// the file to exist and to live under `canonical_root` (no `..` escape,
/// no absolute override).
fn canonical_member(canonical_root: &Path, dir: &Path) -> Option<PathBuf> {
    let canonical_manifest = dir.join("Cargo.toml").canonicalize().ok()?;
    canonical_manifest
        .starts_with(canonical_root)
        .then_some(canonical_manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture_path(rel: &str) -> PathBuf {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root.join("tests/fixtures").join(rel)
    }

    #[test]
    fn walker_toml_project_identity_filters_array_values() {
        let source = r#"[project]
name = "demo"
dynamic = ["version"]
description = "Demo package"
authors = [{ name = "Ada" }]
classifiers = [
    "Programming Language :: Python :: 3",
]
requires-python = ">=3.10"

[project.urls]
Homepage = "https://example.com"
"#;

        let lines = project_identity_lines(source, 1, 9);
        assert_eq!(lines, vec![1, 2, 4]);
    }

    /// mdbook fixture: explicit `crates/*` glob, three literal entries
    /// (`.`, `examples/.../mdbook-remove-emphasis`, `guide/guide-helper`),
    /// plus an unrelated nested Cargo.toml that must NOT be a member.
    #[test]
    fn walker_toml_workspace_members_mdbook_fixture() {
        let root = fixture_path("mdbook");
        let members = collect_workspace_members(&root);

        // All 9 crates/* directories are members.
        let expected_crates = [
            "mdbook-compare",
            "mdbook-core",
            "mdbook-driver",
            "mdbook-html",
            "mdbook-markdown",
            "mdbook-preprocessor",
            "mdbook-renderer",
            "mdbook-summary",
            "xtask",
        ];
        for crate_name in expected_crates {
            let p = root
                .join("crates")
                .join(crate_name)
                .join("Cargo.toml")
                .canonicalize()
                .unwrap();
            assert!(members.contains(&p), "expected member: {}", p.display());
        }

        // Literal entries.
        let example = root
            .join("examples/remove-emphasis/mdbook-remove-emphasis/Cargo.toml")
            .canonicalize()
            .unwrap();
        assert!(members.contains(&example), "example crate must be a member");
        let guide_helper = root
            .join("guide/guide-helper/Cargo.toml")
            .canonicalize()
            .unwrap();
        assert!(
            members.contains(&guide_helper),
            "guide-helper must be a member"
        );

        // Independent nested Cargo.toml under workspace root: NOT a member.
        let wordcount = root
            .join("guide/src/for_developers/mdbook-wordcount/Cargo.toml")
            .canonicalize()
            .unwrap();
        assert!(
            !members.contains(&wordcount),
            "mdbook-wordcount is independent, not a workspace member"
        );

        // Workspace root itself: NOT in the member set.
        let root_manifest = root.join("Cargo.toml").canonicalize().unwrap();
        assert!(
            !members.contains(&root_manifest),
            "workspace root must be excluded from member set"
        );
    }

    #[test]
    fn walker_toml_workspace_members_no_workspace() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("Cargo.toml"),
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        let members = collect_workspace_members(dir.path());
        assert!(
            members.is_empty(),
            "no [workspace] table → empty member set"
        );
    }

    /// Codex no-ship regression: a non-workspace root with a path
    /// dependency must NOT damp the dep's `[package]`. Path-dep
    /// auto-promotion is a workspace-only behavior.
    #[test]
    fn walker_toml_workspace_members_path_dep_without_workspace() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("Cargo.toml"),
            r#"[package]
name = "root"
version = "0.1.0"

[dependencies]
foo = { path = "deps/foo" }
"#,
        )
        .unwrap();
        let foo_dir = dir.path().join("deps/foo");
        fs::create_dir_all(&foo_dir).unwrap();
        fs::write(
            foo_dir.join("Cargo.toml"),
            "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        let members = collect_workspace_members(dir.path());
        assert!(
            members.is_empty(),
            "no [workspace] table → path-dep must not be auto-promoted"
        );
    }

    #[test]
    fn walker_toml_workspace_members_path_dependencies() {
        let dir = tempfile::tempdir().unwrap();
        // Root manifest with a [workspace] (otherwise no auto-members) and
        // a path dependency.
        fs::write(
            dir.path().join("Cargo.toml"),
            r#"[workspace]
members = []

[package]
name = "root"
version = "0.1.0"

[dependencies]
foo = { path = "deps/foo" }
"#,
        )
        .unwrap();
        let foo_dir = dir.path().join("deps/foo");
        fs::create_dir_all(&foo_dir).unwrap();
        fs::write(
            foo_dir.join("Cargo.toml"),
            "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        let members = collect_workspace_members(dir.path());
        let expected = foo_dir.join("Cargo.toml").canonicalize().unwrap();
        assert!(
            members.contains(&expected),
            "path-dependency Cargo.toml must be auto-promoted"
        );
    }

    /// Regression guard: `[workspace].exclude` must apply *after* the
    /// candidate set is built from members ∪ path-deps.
    #[test]
    fn walker_toml_workspace_members_exclude_blocks_path_dep() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("Cargo.toml"),
            r#"[workspace]
members = []
exclude = ["deps/foo"]

[package]
name = "root"
version = "0.1.0"

[dependencies]
foo = { path = "deps/foo" }
"#,
        )
        .unwrap();
        let foo_dir = dir.path().join("deps/foo");
        fs::create_dir_all(&foo_dir).unwrap();
        fs::write(
            foo_dir.join("Cargo.toml"),
            "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        let members = collect_workspace_members(dir.path());
        let candidate = foo_dir.join("Cargo.toml").canonicalize().unwrap();
        assert!(
            !members.contains(&candidate),
            "exclude must block path-dep auto-promotion"
        );
    }

    /// Honest scope: `crates/foo-*` (mid-name globs) are not supported;
    /// resolver returns no members rather than silently mis-matching.
    #[test]
    fn walker_toml_workspace_members_unsupported_glob() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("Cargo.toml"),
            r#"[workspace]
members = ["crates/mdbook-*"]
"#,
        )
        .unwrap();
        // Even with matching dirs on disk, unsupported globs return empty.
        let crates_dir = dir.path().join("crates/mdbook-core");
        fs::create_dir_all(&crates_dir).unwrap();
        fs::write(
            crates_dir.join("Cargo.toml"),
            "[package]\nname = \"mdbook-core\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        let members = collect_workspace_members(dir.path());
        assert!(
            members.is_empty(),
            "mid-name glob shape is unsupported and must not match"
        );
    }
}
