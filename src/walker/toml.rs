//! TOML walker. Uses `tree-sitter-toml-ng` to identify top-level `[table]`
//! headers and their line ranges. Emits one batch per ontology-recognized
//! section group (identity / package metadata / operational /
//! dependencies / config).
//!
//! Keys:
//! - `Identity { file }` — `[package]`, `[workspace]`, `[workspace.package]`,
//!   `[project]`, `[tool.poetry]`; the Python tables contribute their lede
//!   only, so the batch stays cheap enough to win an early slot
//! - `PackageMetadata { file }` — the rest of a Python identity table: author
//!   and maintainer rosters, project URLs, keywords
//! - `Operational { file }` — `[features]`, and a Python manifest's
//!   `[project.scripts]` / `[tool.poetry.scripts]`
//! - `Dependencies { file }` — Cargo `[dependencies]` /
//!   `[workspace.dependencies]`, `[tool.poetry.dependencies]`, and the PEP
//!   621 dependency arrays under `[project]`
//! - `Config { file }` — every other table of a manifest, whatever it is
//!   named: build systems, targets, profiles, lints, patches, packaging;
//!   predecessor: `Identity` on the same file when it has one

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, TomlKey};
use crate::render::Source;
use crate::value::{
    dependency_roster_value, manifest_appendix_value, manifest_identity_value,
    manifest_operational_value,
};

use super::workspace::{WORKSPACE_MEMBER_IDENTITY_FACTOR, canonical_member, expand_member_entry};
use super::{
    FileLines, WalkCtx, dedup_sorted, fs::files_with_extension, path_depth_factor,
    single_file_lines_content,
};

type Section = (String, usize, usize);

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let mut out = Vec::new();
    for file in files_with_extension(dir, "toml", ctx) {
        let Some((source, tree)) = parse_toml(ctx, &file) else {
            continue;
        };
        let sections = collect_sections(&tree, &source);
        let python_project_manifest = is_python_project_manifest(&file, &sections);
        // Pair-level detail is only consulted for Python identity tables.
        let pairs = if python_project_manifest {
            collect_table_pairs(&tree, &source)
        } else {
            Vec::new()
        };
        let depth = path_depth_factor(&file, ctx);
        let identity_residue = python_identity_non_lede_rows(&source, &sections);
        let identity_rows: Vec<usize> = section_rows(&sections, |n| {
            matches!(n, "package" | "workspace" | "workspace.package")
                || (python_project_manifest && is_pyproject_identity_table(n))
        })
        .into_iter()
        .filter(|row| !identity_residue.contains(row))
        .collect();
        let identity_key: BatchKey = TomlKey::Identity { file: file.clone() }.into();
        let identity = rows_content(&file, &source, identity_rows).map(|content| {
            let scale = if ctx.is_workspace_member(&file) {
                WORKSPACE_MEMBER_IDENTITY_FACTOR
            } else {
                1.0
            };
            out.push(Batch {
                key: identity_key.clone(),
                predecessor: None,
                content,
                value: manifest_identity_value(scale, depth),
            });
            identity_key
        });
        if let Some(identity) = &identity
            && let Some(content) = rows_content(
                &file,
                &source,
                package_metadata_rows(&pairs, &identity_residue),
            )
        {
            out.push(Batch {
                key: TomlKey::PackageMetadata { file: file.clone() }.into(),
                predecessor: Some(identity.clone()),
                content,
                value: manifest_appendix_value(depth),
            });
        }
        let operational_rows = section_rows(&sections, |n| {
            n == "features" || (python_project_manifest && is_scripts_section(n))
        });
        if let Some(content) = rows_content(&file, &source, operational_rows) {
            out.push(Batch {
                key: TomlKey::Operational { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: manifest_operational_value(depth),
            });
        }
        let mut dependency_rows = section_rows(&sections, is_ordinary_dependency_section);
        if python_project_manifest {
            dependency_rows.extend(pep621_dependency_array_rows(&pairs));
        }
        if let Some(content) = rows_content(&file, &source, dependency_rows) {
            let value = dependency_roster_value(file.parent() == Some(ctx.root()), depth);
            out.push(Batch {
                key: TomlKey::Dependencies { file: file.clone() }.into(),
                predecessor: None,
                content,
                value,
            });
        }
        // The config appendix of a manifest gates behind that manifest's
        // identity block: linter settings and build-backend tables are
        // qualifiers on a package the reader has not been told the name of
        // yet. The load-bearing sections (operational, dependency
        // rosters) stay ungated — they answer what the project is on their
        // own, and gating them costs more than it buys.
        if is_manifest_toml(&sections, python_project_manifest)
            && let Some(content) =
                rows_content(&file, &source, section_rows(&sections, is_config_section))
        {
            out.push(Batch {
                key: TomlKey::Config { file: file.clone() }.into(),
                predecessor: identity,
                content,
                value: manifest_appendix_value(depth),
            });
        }
    }
    out
}

/// Every row of the sections whose name satisfies `name_match`.
fn section_rows(sections: &[Section], name_match: impl Fn(&str) -> bool) -> Vec<usize> {
    sections
        .iter()
        .filter(|(name, _, _)| name_match(name))
        .flat_map(|(_, start, end)| *start..=*end)
        .collect()
}

fn rows_content(
    file: &Path,
    source: &Source,
    rows: Vec<usize>,
) -> Option<crate::content::BatchContent> {
    if rows.is_empty() {
        return None;
    }
    single_file_lines_content(file, source, FileLines::new(dedup_sorted(rows)))
}

/// The identity-table residue, minus the dependency arrays the dependency
/// batch owns — no two peer batches may claim the same row.
fn package_metadata_rows(pairs: &[TablePair], identity_residue: &HashSet<usize>) -> Vec<usize> {
    let dropped: HashSet<usize> = pep621_dependency_array_rows(pairs)
        .chain(packaging_mechanics_rows(pairs))
        .collect();
    identity_residue.difference(&dropped).copied().collect()
}

/// One `key = value` pair written directly under a top-level `[table]`, with
/// the inclusive 1-based row span of the whole pair — a multi-line array or
/// inline table runs through its closing bracket.
struct TablePair {
    table: String,
    key: String,
    start: usize,
    end: usize,
    value_is_array: bool,
}

fn collect_table_pairs(tree: &Tree, source: &str) -> Vec<TablePair> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for child in root.children(&mut cursor) {
        if child.kind() != "table" {
            continue;
        }
        let Some(table) = extract_table_name(child, source) else {
            continue;
        };
        let mut pair_cursor = child.walk();
        for pair in child.children(&mut pair_cursor) {
            if pair.kind() != "pair" {
                continue;
            }
            let Some(key) = pair_key(pair, source) else {
                continue;
            };
            let value = pair_value_node(pair);
            out.push(TablePair {
                table: table.clone(),
                key,
                start: pair.start_position().row + 1,
                end: value.unwrap_or(pair).end_position().row + 1,
                value_is_array: value.is_some_and(|v| v.kind() == "array"),
            });
        }
    }
    out
}

fn pair_key(pair: Node, source: &str) -> Option<String> {
    let mut cursor = pair.walk();
    pair.children(&mut cursor)
        .find(|child| matches!(child.kind(), "bare_key" | "dotted_key" | "quoted_key"))
        .map(|child| normalize_key_path(source[child.start_byte()..child.end_byte()].trim()))
}

fn pair_value_node(pair: Node) -> Option<Node> {
    let mut cursor = pair.walk();
    let mut seen_key = false;
    for child in pair.children(&mut cursor) {
        if matches!(child.kind(), "bare_key" | "dotted_key" | "quoted_key") {
            seen_key = true;
            continue;
        }
        if seen_key && !matches!(child.kind(), "=" | "comment") {
            return Some(child);
        }
    }
    None
}

/// `pyproject.toml`, or any TOML declaring a PEP 621 `[project]` table —
/// the Python counterpart of [`is_manifest_toml`]'s name-independent rule.
fn is_python_project_manifest(file: &Path, sections: &[Section]) -> bool {
    file.file_name().and_then(|n| n.to_str()) == Some("pyproject.toml")
        || sections.iter().any(|(name, _, _)| name == "project")
}

/// A TOML that declares package identity is a manifest whatever it is named —
/// `sqlite-dist.toml`, `uv.toml` and friends carry the same class of content
/// as `Cargo.toml`, and a filename list can only ever recognize the dialects
/// that already existed when it was written.
fn is_manifest_toml(sections: &[Section], python_project_manifest: bool) -> bool {
    python_project_manifest
        || sections.iter().any(|(name, _, _)| {
            matches!(name.as_str(), "package" | "workspace" | "workspace.package")
        })
}

fn is_dependency_section(name: &str) -> bool {
    is_ordinary_dependency_section(name)
        || is_cargo_development_dependency_section(name)
        || is_dependency_group_section(name)
}

/// Named test / lint / docs rosters, the Python analogue of Cargo's
/// `[dev-dependencies]`: PEP 735 `[dependency-groups]` and Poetry's
/// `[tool.poetry.group.*]` / `[tool.poetry.dev-dependencies]`.
fn is_dependency_group_section(name: &str) -> bool {
    name == "dependency-groups"
        || name.starts_with("dependency-groups.")
        || name.starts_with("tool.poetry.group.")
        || name == "tool.poetry.dev-dependencies"
}

fn is_ordinary_dependency_section(name: &str) -> bool {
    matches!(
        name,
        "dependencies"
            | "workspace.dependencies"
            | "tool.poetry.dependencies"
            | "project.optional-dependencies"
    ) || name.starts_with("dependencies.")
        || name.starts_with("workspace.dependencies.")
}

fn is_cargo_development_dependency_section(name: &str) -> bool {
    matches!(name, "dev-dependencies" | "build-dependencies")
        || name.starts_with("dev-dependencies.")
        || name.starts_with("build-dependencies.")
        || (name.starts_with("target.")
            && name.split('.').any(|segment| {
                matches!(
                    segment,
                    "dependencies" | "dev-dependencies" | "build-dependencies"
                )
            }))
}

fn is_scripts_section(name: &str) -> bool {
    matches!(name, "project.scripts" | "tool.poetry.scripts")
}

/// Every table of a manifest that no other batch claims. A manifest is
/// author-written declaration throughout: a `[lib]`, a `[lints.*]` block or a
/// `[patch.*]` redirect states a decision about the project as much as a
/// `[profile.*]` does, and enumerating the tables worth keeping only ever
/// produces a list that the next manifest falls outside of.
fn is_config_section(name: &str) -> bool {
    !(matches!(
        name,
        "package" | "workspace" | "workspace.package" | "features"
    ) || is_pyproject_identity_table(name)
        || is_scripts_section(name)
        || is_dependency_section(name))
}

/// Rows of a Python identity table that its lede does not take: the author and
/// maintainer rosters, project URLs, keywords, trove classifiers, packaging
/// globs — and the PEP 621 dependency arrays, which the dependency batch owns.
///
/// A Cargo `[package]` table has no such residue: it is short enough that the
/// whole table is the lede. A PEP 621 `[project]` table routinely declares
/// four times its lede in metadata, and the Identity batch competes for its
/// early slot on `value / cost^k` — carrying that metadata costs the lede the
/// slot outright, so the residue is priced as manifest config instead.
fn python_identity_non_lede_rows(source: &str, sections: &[Section]) -> HashSet<usize> {
    let lines: Vec<&str> = source.lines().collect();
    sections
        .iter()
        .filter(|(name, _, _)| is_pyproject_identity_table(name))
        .flat_map(|(_, start, end)| (start + 1)..=*end)
        .filter(|row| {
            !lines
                .get(row - 1)
                .is_some_and(|line| is_lede_pair_line(line))
        })
        .collect()
}

/// The PEP 621 / Poetry lede: the keys that name, version and describe the
/// package, in their single-line scalar form.
///
/// Row-ownership invariant: this line predicate is the only thing keeping
/// `Identity` and `Dependencies` from claiming the same row — `Identity`
/// takes the lede rows out of `[project]`, `Dependencies` takes the
/// dependency-array rows, and the two stay disjoint only because no row of
/// a dependency array can satisfy this test (the array's own `dependencies
/// = [` opener keys on a name that is not in the list below, and its
/// element rows are quoted strings whose `split_once('=')` key retains a
/// quote). Adding a key here — or relaxing the scalar-value test — must be
/// checked against that: peer batches sharing a row is a scheduler panic.
fn is_lede_pair_line(line: &str) -> bool {
    let Some((key, value)) = line.trim_start().split_once('=') else {
        return false;
    };
    matches!(
        key.trim(),
        "name" | "version" | "description" | "requires-python" | "license" | "readme"
    ) && value
        .trim_start()
        .chars()
        .next()
        .is_some_and(|ch| ch != '[' && ch != '{')
}

/// PEP 621 dependency arrays live inside `[project]` but belong to the
/// dependency batches.
fn is_pep621_dependency_key(key: &str) -> bool {
    matches!(key, "dependencies" | "optional-dependencies")
}

/// Rows of the identity-table keys that no batch emits at any budget: the
/// trove-classifier list, which restates in a fixed registry vocabulary what
/// `license`, `requires-python` and `description` already say, and the
/// archive-selection globs, which describe how the package is built rather
/// than what it is. Both are among the longest keys a manifest declares.
fn packaging_mechanics_rows(pairs: &[TablePair]) -> impl Iterator<Item = usize> {
    pairs
        .iter()
        .filter(|pair| {
            is_pyproject_identity_table(&pair.table)
                && matches!(
                    pair.key.as_str(),
                    "classifiers" | "packages" | "include" | "exclude"
                )
        })
        .flat_map(|pair| pair.start..=pair.end)
}

fn pep621_dependency_array_rows(pairs: &[TablePair]) -> impl Iterator<Item = usize> {
    pairs
        .iter()
        .filter(|pair| {
            pair.table == "project" && pair.value_is_array && is_pep621_dependency_key(&pair.key)
        })
        .flat_map(|pair| pair.start..=pair.end)
}

fn is_pyproject_identity_table(name: &str) -> bool {
    matches!(name, "project" | "tool.poetry")
}

// --- parser ---

fn parse_toml(ctx: &WalkCtx, path: &Path) -> Option<(Arc<Source>, Arc<Tree>)> {
    ctx.parse_tree(path, &tree_sitter_toml_ng::LANGUAGE.into())
}

// --- section collection ---

/// `(header_name, start_1based, end_1based)` for every top-level
/// `table` or `[[array-of-tables]]`. End is the row before the next
/// table/array block or EOF.
fn collect_sections(tree: &Tree, source: &str) -> Vec<(String, usize, usize)> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut raw: Vec<(String, usize)> = Vec::new();
    for child in root.children(&mut cursor) {
        let name = match child.kind() {
            "table" | "table_array_element" => {
                let Some(name) = extract_table_name(child, source) else {
                    continue;
                };
                name
            }
            _ => continue,
        };
        raw.push((name, child.start_position().row));
    }
    let total_rows = source.lines().count();
    let mut out = Vec::new();
    for i in 0..raw.len() {
        let name = raw[i].0.clone();
        let start = raw[i].1 + 1;
        let end = if i + 1 < raw.len() {
            raw[i + 1].1
        } else {
            total_rows
        };
        out.push((name, start, end));
    }
    out
}

fn extract_table_name(node: Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if matches!(child.kind(), "bare_key" | "dotted_key" | "quoted_key") {
            let text = &source[child.start_byte()..child.end_byte()];
            return Some(normalize_key_path(text.trim()));
        }
    }
    None
}

/// Strip quotes from dotted-key segments so `[tool."poetry".scripts]`
/// classifies the same as `[tool.poetry.scripts]`. Splits only on dots
/// *outside* quotes, and a segment whose content contains a literal dot
/// keeps its quotes — `["tool.poetry".scripts]` names a different table
/// than `[tool.poetry.scripts]` and must not normalize into it.
fn normalize_key_path(text: &str) -> String {
    let mut segments: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut push_segment = |segment: &mut String| {
        let trimmed = segment.trim();
        segments.push(if trimmed.contains('.') {
            format!("\"{trimmed}\"")
        } else {
            trimmed.to_string()
        });
        segment.clear();
    };
    for c in text.chars() {
        match quote {
            Some(q) if c == q => quote = None,
            Some(_) => current.push(c),
            None => match c {
                '"' | '\'' => quote = Some(c),
                '.' => push_segment(&mut current),
                _ => current.push(c),
            },
        }
    }
    push_segment(&mut current);
    segments.join(".")
}

// --- workspace-member resolution ---

const CARGO_MANIFEST_FILENAME: &str = "Cargo.toml";

/// Declared + auto-promoted workspace members from `<root>/Cargo.toml`.
/// Union of `[workspace].members` (literals + trailing-`/*` globs) and
/// `[dependencies]`-table `path = "..."` entries; `[workspace].exclude`
/// applies to the union. Empty on parse error.
pub(super) fn collect_workspace_members(root: &Path) -> HashSet<PathBuf> {
    let root_manifest = root.join(CARGO_MANIFEST_FILENAME);
    let Some(value) = parse_manifest(&root_manifest) else {
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
                if let Some(member) =
                    canonical_member(&canonical_root, &path, CARGO_MANIFEST_FILENAME)
                {
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
        for dep in table.values() {
            if let Some(path) = dep_path(dep)
                && let Some(member) =
                    canonical_member(&canonical_root, &root.join(path), CARGO_MANIFEST_FILENAME)
            {
                candidates.insert(member);
            }
        }
    }

    for ex in collect("exclude") {
        candidates.remove(&ex);
    }
    if let Ok(canonical_root_manifest) = root_manifest.canonicalize() {
        candidates.remove(&canonical_root_manifest);
    }
    candidates
}

/// The `path = "..."` of a dependency-style entry, if it has one.
fn dep_path(spec: &toml::Value) -> Option<&str> {
    spec.as_table()?.get("path")?.as_str()
}

fn parse_manifest(path: &Path) -> Option<toml::Value> {
    toml::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture_path(rel: &str) -> PathBuf {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root.join("tests/fixtures").join(rel)
    }

    fn parse(source: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_toml_ng::LANGUAGE.into())
            .unwrap();
        parser.parse(source, None).unwrap()
    }

    /// `(identity lede rows, manifest-config residue rows)` for a source whose
    /// only identity table spans 1..=`end`.
    fn identity_partition(source: &str, end: usize) -> (Vec<usize>, Vec<usize>) {
        let sections = collect_sections(&parse(source), source);
        let residue = python_identity_non_lede_rows(source, &sections);
        let owned: HashSet<usize> =
            pep621_dependency_array_rows(&collect_table_pairs(&parse(source), source)).collect();
        (
            (1..=end).filter(|row| !residue.contains(row)).collect(),
            dedup_sorted(residue.difference(&owned).copied().collect()),
        )
    }

    fn members_with(
        root_toml: &str,
        nested: &[(&str, &str)],
    ) -> (tempfile::TempDir, HashSet<PathBuf>) {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), root_toml).unwrap();
        for (rel_dir, body) in nested {
            let manifest_dir = dir.path().join(rel_dir);
            fs::create_dir_all(&manifest_dir).unwrap();
            fs::write(manifest_dir.join("Cargo.toml"), body).unwrap();
        }
        let members = collect_workspace_members(dir.path());
        (dir, members)
    }

    #[test]
    fn walker_toml_normalize_key_path_quoted_segments() {
        // Quoted segment without a literal dot normalizes into the
        // dotted path; a literal-dot segment keeps its quotes so it
        // can't be confused with the structurally-dotted table.
        assert_eq!(
            normalize_key_path("tool.\"poetry\".scripts"),
            "tool.poetry.scripts"
        );
        assert_eq!(normalize_key_path("'tool'.poetry"), "tool.poetry");
        assert_eq!(
            normalize_key_path("\"tool.poetry\".scripts"),
            "\"tool.poetry\".scripts"
        );
        assert_eq!(normalize_key_path("dependencies"), "dependencies");
        assert_eq!(normalize_key_path("\"dependencies\""), "dependencies");
    }

    #[test]
    fn walker_toml_sections_include_array_of_tables() {
        let source = "[package]\nname = \"demo\"\nversion = \"1.0\"\n\n\
                      [[bin]]\nname = \"demo-cli\"\n\n\
                      [dependencies]\nserde = \"1\"\n";
        let sections = collect_sections(&parse(source), source);
        assert_eq!(
            sections,
            vec![
                ("package".to_string(), 1, 4),
                ("bin".to_string(), 5, 7),
                ("dependencies".to_string(), 8, 9),
            ],
        );
    }

    /// The lede keeps the single-line scalars that name and describe the
    /// package; every other row of the table is manifest-config residue —
    /// except the PEP 621 dependency arrays, which the dependency batch owns
    /// and which no second batch may claim.
    #[test]
    fn walker_toml_project_identity_splits_lede_from_metadata() {
        let source = r#"[project]
name = "demo"
dynamic = ["version"]
description = "Demo package"
authors = [{ name = "Ada" }]
license = { text = "MIT" }
classifiers = [
    "Programming Language :: Python :: 3",
]
dependencies = [
    "click",
]
requires-python = ">=3.10"
"#;
        let (lede, residue) = identity_partition(source, 13);
        assert_eq!(lede, vec![1, 2, 4, 13]);
        assert_eq!(residue, vec![3, 5, 6, 7, 8, 9]);
    }

    /// A Cargo manifest has no residue: `[package]` is taken whole, and the
    /// identity split is a Python-manifest rule.
    #[test]
    fn walker_toml_cargo_package_table_has_no_identity_residue() {
        let source = "[package]\nname = \"demo\"\nkeywords = [\"a\"]\nexclude = [\"rfcs/**/*\"]\n";
        let sections = collect_sections(&parse(source), source);
        assert!(python_identity_non_lede_rows(source, &sections).is_empty());
    }

    #[test]
    fn walker_toml_python_manifest_is_pyproject_or_any_project_table() {
        let parse_sections = |source: &str| collect_sections(&parse(source), source);

        let project = parse_sections("[project]\nname = \"demo\"\n");
        assert!(is_python_project_manifest(
            &PathBuf::from("project.toml"),
            &project
        ));

        let tool_only = parse_sections("[tool.ruff]\nline-length = 100\n");
        assert!(is_python_project_manifest(
            &PathBuf::from("pyproject.toml"),
            &tool_only
        ));
        assert!(!is_python_project_manifest(
            &PathBuf::from("ruff.toml"),
            &tool_only
        ));
    }

    /// Poetry-style pyproject (`[tool.poetry]` as lede table) is
    /// classified the same as PEP 621 `[project]`: same lede rule,
    /// same lede-detection signal. Both rich and beets ship Poetry
    /// pyprojects in the corpus.
    #[test]
    fn walker_toml_poetry_table_treated_as_pyproject_identity() {
        assert!(is_pyproject_identity_table("tool.poetry"));
        assert!(is_pyproject_identity_table("project"));
        assert!(!is_pyproject_identity_table("tool.poetry.dependencies"));
        assert!(!is_pyproject_identity_table("package"));

        let source = r#"[tool.poetry]
name = "rich"
homepage = "https://github.com/Textualize/rich"
version = "15.0.0"
authors = ["Will McGugan <willmcgugan@gmail.com>"]
"#;
        let (lede, residue) = identity_partition(source, 5);
        assert_eq!(lede, vec![1, 2, 4]);
        assert_eq!(residue, vec![3, 5]);
    }

    /// Config takes every table no other batch owns; the owned ones are the
    /// whole exclusion list, since a peer batch may not re-claim their lines.
    #[test]
    fn walker_toml_config_sections_do_not_overlap_owned_sections() {
        let owned = [
            "package",
            "workspace",
            "workspace.package",
            "features",
            "project",
            "project.scripts",
            "tool.poetry",
            "tool.poetry.dependencies",
            "tool.poetry.scripts",
            "dependencies.foo",
            "dev-dependencies.foo",
            "build-dependencies.foo",
            "dependency-groups",
            "target.'cfg(unix)'.dependencies",
            "target.'cfg(windows)'.dev-dependencies",
        ];
        for name in owned {
            assert!(
                !is_config_section(name),
                "{name} must stay with its owning TOML batch"
            );
        }
        for name in [
            "tool.ruff",
            "lib",
            "lints.clippy",
            "patch.crates-io",
            "test",
        ] {
            assert!(
                is_config_section(name),
                "{name} is unclaimed manifest config"
            );
        }
    }

    #[test]
    fn walker_toml_cargo_dependency_classes_are_disjoint() {
        for name in [
            "dependencies",
            "dependencies.serde",
            "workspace.dependencies",
        ] {
            assert!(is_ordinary_dependency_section(name), "ordinary: {name}");
            assert!(
                !is_cargo_development_dependency_section(name),
                "not development: {name}"
            );
        }
        for name in [
            "dev-dependencies",
            "dev-dependencies.proptest",
            "build-dependencies",
            "build-dependencies.cc",
            "target.'cfg(unix)'.dependencies",
            "target.'cfg(windows)'.dev-dependencies",
            "target.'cfg(target_os = \"macos\")'.build-dependencies.bindgen",
        ] {
            assert!(
                is_cargo_development_dependency_section(name),
                "development/build/target: {name}"
            );
            assert!(
                !is_ordinary_dependency_section(name),
                "not ordinary: {name}"
            );
        }
        for name in ["profile.release", "bin", "example", "test", "bench"] {
            assert!(!is_dependency_section(name), "config/target only: {name}");
        }
    }

    /// A manifest's config appendix is a qualifier on the package the
    /// identity block names, so it waits for it. A manifest that declares
    /// no identity table at all — a bare `[build-system]` pyproject — has
    /// nothing to wait for and stays ungated.
    #[test]
    fn walker_toml_config_appendix_gates_behind_identity() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname='demo'\nversion='0.1.0'\n[profile.release]\nlto=true\n",
        )
        .unwrap();
        fs::write(
            root.join("pyproject.toml"),
            "[build-system]\nrequires = ['setuptools']\nbuild-backend = 'setuptools.build_meta'\n",
        )
        .unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        for (rel, expected) in [
            (
                "Cargo.toml",
                Some(BatchKey::Toml(TomlKey::Identity {
                    file: root.join("Cargo.toml"),
                })),
            ),
            ("pyproject.toml", None),
        ] {
            let file = root.join(rel);
            let config = batches
                .iter()
                .find(
                    |b| matches!(&b.key, BatchKey::Toml(TomlKey::Config { file: f }) if *f == file),
                )
                .expect("config batch");
            assert_eq!(config.predecessor, expected, "{rel}");
        }
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
        let (_dir, members) = members_with("[package]\nname = \"x\"\nversion = \"0.1.0\"\n", &[]);
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
        let (_dir, members) = members_with(
            r#"[package]
name = "root"
version = "0.1.0"

[dependencies]
foo = { path = "deps/foo" }
"#,
            &[(
                "deps/foo",
                "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
            )],
        );
        assert!(
            members.is_empty(),
            "no [workspace] table → path-dep must not be auto-promoted"
        );
    }

    #[test]
    fn walker_toml_workspace_members_path_dependencies() {
        // Root manifest with a [workspace] (otherwise no auto-members) and
        // a path dependency.
        let (dir, members) = members_with(
            r#"[workspace]
members = []

[package]
name = "root"
version = "0.1.0"

[dependencies]
foo = { path = "deps/foo" }
"#,
            &[(
                "deps/foo",
                "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
            )],
        );
        let expected = dir
            .path()
            .join("deps/foo/Cargo.toml")
            .canonicalize()
            .unwrap();
        assert!(
            members.contains(&expected),
            "path-dependency Cargo.toml must be auto-promoted"
        );
    }

    /// Regression guard: `[workspace].exclude` must apply *after* the
    /// candidate set is built from members ∪ path-deps.
    #[test]
    fn walker_toml_workspace_members_exclude_blocks_path_dep() {
        let (dir, members) = members_with(
            r#"[workspace]
members = []
exclude = ["deps/foo"]

[package]
name = "root"
version = "0.1.0"

[dependencies]
foo = { path = "deps/foo" }
"#,
            &[(
                "deps/foo",
                "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
            )],
        );
        let candidate = dir
            .path()
            .join("deps/foo/Cargo.toml")
            .canonicalize()
            .unwrap();
        assert!(
            !members.contains(&candidate),
            "exclude must block path-dep auto-promotion"
        );
    }

    /// Honest scope: `crates/foo-*` (mid-name globs) are not supported;
    /// resolver returns no members rather than silently mis-matching.
    #[test]
    fn walker_toml_workspace_members_unsupported_glob() {
        let (_dir, members) = members_with(
            r#"[workspace]
members = ["crates/mdbook-*"]
"#,
            &[(
                "crates/mdbook-core",
                "[package]\nname = \"mdbook-core\"\nversion = \"0.1.0\"\n",
            )],
        );
        assert!(
            members.is_empty(),
            "mid-name glob shape is unsupported and must not match"
        );
    }
}
