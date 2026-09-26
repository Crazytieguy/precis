//! TOML walker. Uses `tree-sitter-toml-ng` to identify top-level `[table]`
//! headers and their line ranges. Emits one batch per ontology-recognized
//! section group (identity / operational / dependencies). Every other
//! table — build systems, profiles, lints, tool config — and the
//! metadata rest of a Python identity table are left to an explicit read.
//!
//! Keys:
//! - `Identity { file }` — `[package]`, `[workspace]`, `[workspace.package]`,
//!   `[project]`, `[tool.poetry]`; the Python tables contribute their lede
//!   only, so the batch stays cheap enough to win an early slot
//! - `Operational { file }` — `[features]`, Cargo's `[lib]` and `[[bin]]`
//!   target declarations, and a Python manifest's
//!   `[project.scripts]` / `[tool.poetry.scripts]`
//! - `Dependencies { file }` — Cargo `[dependencies]` (platform-specific
//!   ones included) / `[workspace.dependencies]`,
//!   `[tool.poetry.dependencies]`, and the PEP
//!   621 dependency arrays under `[project]`

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, TomlKey};
use crate::value::{dependency_roster_value, manifest_identity_value, manifest_operational_value};

use super::workspace::{WORKSPACE_MEMBER_IDENTITY_FACTOR, canonical_member, expand_member_entry};
use super::{WalkCtx, fs::files_with_any_extension, path_depth_factor, single_file_lines_content};

/// A top-level `[table]` or `[[array-of-tables]]` header and its inclusive
/// 1-based row span, which ends at the row before the next header or EOF.
#[derive(Debug, PartialEq)]
struct Section {
    name: String,
    start: usize,
    end: usize,
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let mut out = Vec::new();
    for file in files_with_any_extension(dir, &["toml"], ctx) {
        let Some((source, tree)) = ctx.parse_tree(&file, &tree_sitter_toml_ng::LANGUAGE.into())
        else {
            continue;
        };
        let sections = collect_sections(&tree, &source);
        let python_project_manifest = is_python_project_manifest(&file, &sections);
        let (pairs, identity_residue) = if python_project_manifest {
            (
                collect_table_pairs(&tree, &source),
                python_identity_non_lede_rows(&source, &sections),
            )
        } else {
            Default::default()
        };
        let depth = path_depth_factor(&file, ctx);
        let identity_rows: Vec<usize> = section_rows(&sections, |n| {
            matches!(n, "package" | "workspace" | "workspace.package")
                || (python_project_manifest && is_pyproject_identity_table(n))
        })
        .into_iter()
        .filter(|row| !identity_residue.contains(row))
        .collect();
        if let Some(content) = single_file_lines_content(&file, &source, identity_rows) {
            let scale = if ctx.is_cargo_workspace_member(&file) {
                WORKSPACE_MEMBER_IDENTITY_FACTOR
            } else {
                1.0
            };
            out.push(Batch {
                key: TomlKey::Identity { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: manifest_identity_value(scale, depth),
            });
        }
        let operational_rows = section_rows(&sections, |n| {
            is_operational_section(n, python_project_manifest)
        });
        if let Some(content) = single_file_lines_content(&file, &source, operational_rows) {
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
        if let Some(content) = single_file_lines_content(&file, &source, dependency_rows) {
            let value = dependency_roster_value(file.parent() == Some(ctx.root()), depth);
            out.push(Batch {
                key: TomlKey::Dependencies { file: file.clone() }.into(),
                predecessor: None,
                content,
                value,
            });
        }
    }
    out
}

/// Every row of the sections whose name satisfies `name_match`.
fn section_rows(sections: &[Section], name_match: impl Fn(&str) -> bool) -> Vec<usize> {
    sections
        .iter()
        .filter(|section| name_match(&section.name))
        .flat_map(|section| section.start..=section.end)
        .collect()
}

/// One `key = value` pair written directly under a top-level `[table]`, with
/// the inclusive 1-based row span of the whole pair — a multi-line array or
/// inline table runs through its closing bracket.
struct TablePair {
    table: String,
    key: String,
    start: usize,
    end: usize,
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
            out.push(TablePair {
                table: table.clone(),
                key,
                start: pair.start_position().row + 1,
                end: pair_value_node(pair).unwrap_or(pair).end_position().row + 1,
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

/// `pyproject.toml`, or any TOML declaring a PEP 621 `[project]` table.
fn is_python_project_manifest(file: &Path, sections: &[Section]) -> bool {
    file.file_name().and_then(|n| n.to_str()) == Some("pyproject.toml")
        || sections.iter().any(|section| section.name == "project")
}

fn is_ordinary_dependency_section(name: &str) -> bool {
    let name = untargeted_cargo_table(name);
    matches!(
        name,
        "dependencies"
            | "workspace.dependencies"
            | "tool.poetry.dependencies"
            | "project.optional-dependencies"
    ) || name.starts_with("dependencies.")
        || name.starts_with("workspace.dependencies.")
}

/// A Cargo platform-specific dependency table read as the table it scopes:
/// `target.'cfg(unix)'.dependencies.libc` is `dependencies.libc`. Any other
/// name comes back unchanged.
fn untargeted_cargo_table(name: &str) -> &str {
    let Some(rest) = name.strip_prefix("target.") else {
        return name;
    };
    rest.match_indices('.')
        .map(|(dot, _)| &rest[dot + 1..])
        .find(|tail| {
            matches!(
                tail.split('.').next(),
                Some("dependencies" | "dev-dependencies" | "build-dependencies")
            )
        })
        .unwrap_or(name)
}

/// Feature flags, Cargo's library and binary target declarations, and a
/// Python manifest's console scripts: the package's build and entry surface.
fn is_operational_section(name: &str, python_project_manifest: bool) -> bool {
    matches!(name, "features" | "lib" | "bin")
        || (python_project_manifest && is_scripts_section(name))
}

fn is_scripts_section(name: &str) -> bool {
    matches!(name, "project.scripts" | "tool.poetry.scripts")
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
        .filter(|section| is_pyproject_identity_table(&section.name))
        .flat_map(|section| (section.start + 1)..=section.end)
        .filter(|row| {
            !lines
                .get(row - 1)
                .is_some_and(|line| is_lede_pair_line(line))
        })
        .collect()
}

/// The PEP 621 / Poetry lede: the keys that name, version and describe the
/// package, in their single-line scalar form (a `"""` string continues onto
/// rows the lede does not take, so its opener stays out too).
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
    ) && {
        let value = value.trim_start();
        !value.is_empty()
            && !value.starts_with(['[', '{'])
            && !value.starts_with("\"\"\"")
            && !value.starts_with("'''")
    }
}

/// PEP 621 dependency arrays live inside `[project]` but belong to the
/// dependency batches.
fn is_pep621_dependency_key(key: &str) -> bool {
    matches!(key, "dependencies" | "optional-dependencies")
}

fn pep621_dependency_array_rows(pairs: &[TablePair]) -> impl Iterator<Item = usize> {
    pairs
        .iter()
        .filter(|pair| pair.table == "project" && is_pep621_dependency_key(&pair.key))
        .flat_map(|pair| pair.start..=pair.end)
}

fn is_pyproject_identity_table(name: &str) -> bool {
    matches!(name, "project" | "tool.poetry")
}

// --- section collection ---

fn collect_sections(tree: &Tree, source: &str) -> Vec<Section> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let headers: Vec<(String, usize)> = root
        .children(&mut cursor)
        .filter(|child| matches!(child.kind(), "table" | "table_array_element"))
        .filter_map(|child| {
            Some((
                extract_table_name(child, source)?,
                child.start_position().row,
            ))
        })
        .collect();
    let total_rows = source.lines().count();
    headers
        .iter()
        .enumerate()
        .map(|(i, (name, row))| {
            let end = headers.get(i + 1).map_or(total_rows, |(_, next)| *next);
            Section {
                name: name.clone(),
                start: row + 1,
                end,
            }
        })
        .collect()
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
        let mut config_rows: Vec<usize> = residue.difference(&owned).copied().collect();
        config_rows.sort();
        (
            (1..=end).filter(|row| !residue.contains(row)).collect(),
            config_rows,
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
    fn toml_normalize_key_path_quoted_segments() {
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
    fn toml_sections_include_array_of_tables() {
        let source = "[package]\nname = \"demo\"\nversion = \"1.0\"\n\n\
                      [[bin]]\nname = \"demo-cli\"\n\n\
                      [dependencies]\nserde = \"1\"\n";
        let sections = collect_sections(&parse(source), source);
        assert_eq!(
            sections,
            vec![
                Section {
                    name: "package".to_string(),
                    start: 1,
                    end: 4
                },
                Section {
                    name: "bin".to_string(),
                    start: 5,
                    end: 7
                },
                Section {
                    name: "dependencies".to_string(),
                    start: 8,
                    end: 9
                },
            ],
        );
    }

    /// The lede keeps the single-line scalars that name and describe the
    /// package; every other row of the table is manifest-config residue —
    /// except the PEP 621 dependency arrays, which the dependency batch owns
    /// and which no second batch may claim.
    #[test]
    fn toml_project_identity_splits_lede_from_metadata() {
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

    /// A multi-line string value renders whole in the residue rather than as
    /// a dangling opener in the lede.
    #[test]
    fn toml_multiline_description_stays_out_of_the_lede() {
        let source = "[project]\nname = \"demo\"\ndescription = \"\"\"\nA demo.\n\"\"\"\n";
        let (lede, residue) = identity_partition(source, 5);
        assert_eq!(lede, vec![1, 2]);
        assert_eq!(residue, vec![3, 4, 5]);
    }

    /// `optional-dependencies` belongs to the dependency roster whether it is
    /// written as a `[project.optional-dependencies]` table or inline.
    #[test]
    fn toml_inline_optional_dependencies_join_the_roster() {
        let source = "[project]\nname = \"demo\"\noptional-dependencies = { dev = [\"pytest\"] }\n";
        let owned: Vec<usize> =
            pep621_dependency_array_rows(&collect_table_pairs(&parse(source), source)).collect();
        assert_eq!(owned, vec![3]);
        let (lede, residue) = identity_partition(source, 3);
        assert_eq!(lede, vec![1, 2]);
        assert!(residue.is_empty());
    }

    /// A Cargo manifest has no residue: `[package]` is taken whole, and the
    /// identity split is a Python-manifest rule.
    #[test]
    fn toml_cargo_package_table_has_no_identity_residue() {
        let source = "[package]\nname = \"demo\"\nkeywords = [\"a\"]\nexclude = [\"rfcs/**/*\"]\n";
        let sections = collect_sections(&parse(source), source);
        assert!(python_identity_non_lede_rows(source, &sections).is_empty());
    }

    #[test]
    fn toml_python_manifest_is_pyproject_or_any_project_table() {
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
    /// same lede-detection signal.
    #[test]
    fn toml_poetry_table_treated_as_pyproject_identity() {
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

    /// Platform-specific runtime dependencies ship with the package like
    /// untargeted ones, so they join the dependency roster; platform-specific
    /// dev- and build-dependencies stay out of it, as their untargeted
    /// tables do.
    #[test]
    fn toml_cargo_target_runtime_dependencies_join_the_roster() {
        let source = "[package]\nname = \"demo\"\n\n\
                      [dependencies]\nserde = \"1\"\n\n\
                      [target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\n\n\
                      [target.'cfg(windows)'.dependencies.windows-sys]\nversion = \"0.59\"\n\n\
                      [target.'cfg(unix)'.dev-dependencies]\nnix = \"0.29\"\n";
        let sections = collect_sections(&parse(source), source);
        assert_eq!(
            section_rows(&sections, is_ordinary_dependency_section),
            vec![4, 5, 6, 7, 8, 9, 10, 11, 12],
        );
    }

    #[test]
    fn toml_cargo_targets_are_operational() {
        let source = "[package]\nname = \"demo\"\n\n\
                      [lib]\nproc-macro = true\n\n\
                      [[bin]]\nname = \"demo-cli\"\npath = \"src/cli.rs\"\n\
                      required-features = [\"cli\"]\n\n\
                      [profile.release]\nlto = true\n";
        let sections = collect_sections(&parse(source), source);
        assert_eq!(
            section_rows(&sections, |n| is_operational_section(n, false)),
            vec![4, 5, 6, 7, 8, 9, 10, 11],
        );
    }

    /// mdbook fixture: explicit `crates/*` glob, three literal entries
    /// (`.`, `examples/.../mdbook-remove-emphasis`, `guide/guide-helper`),
    /// plus an unrelated nested Cargo.toml that must NOT be a member.
    #[test]
    fn toml_workspace_members_mdbook_fixture() {
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
    fn toml_workspace_members_no_workspace() {
        let (_dir, members) = members_with("[package]\nname = \"x\"\nversion = \"0.1.0\"\n", &[]);
        assert!(
            members.is_empty(),
            "no [workspace] table → empty member set"
        );
    }

    /// A non-workspace root with a path
    /// dependency must NOT damp the dep's `[package]`. Path-dep
    /// auto-promotion is a workspace-only behavior.
    #[test]
    fn toml_workspace_members_path_dep_without_workspace() {
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
    fn toml_workspace_members_path_dependencies() {
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
    fn toml_workspace_members_exclude_blocks_path_dep() {
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
    fn toml_workspace_members_unsupported_glob() {
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
