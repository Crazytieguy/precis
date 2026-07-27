//! TOML walker. Uses `tree-sitter-toml-ng` to identify top-level `[table]`
//! headers and their line ranges. Emits one batch per ontology-recognized
//! section group (identity / scripts / features / ordinary dependencies /
//! development dependencies / per-tool config / other config).
//!
//! Keys:
//! - `Identity { file }` — `[package]`, `[workspace]`, `[workspace.package]`,
//!   `[project]`, `[tool.poetry]`
//! - `Scripts { file }` — `[project.scripts]`, `[tool.poetry.scripts]`
//! - `Features { file }` — `[features]`
//! - `Dependencies { file }` — Cargo `[dependencies]` /
//!   `[workspace.dependencies]`, `[tool.poetry.dependencies]`, and the PEP
//!   621 dependency arrays under `[project]`
//! - `DevelopmentDependencies { file }` — Cargo `[dev-dependencies]`,
//!   `[build-dependencies]`, and target-conditional dependency tables
//! - `ToolConfig { file, tool }` — one Python-manifest `tool.<name>` family,
//!   with adjacent small tables packed into compact families
//! - `Config { file }` — other manifest-level build-system, package metadata,
//!   Cargo profiles/targets, and packaging config

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use crate::batch::{Batch, BatchKey, TomlKey};
use crate::value::mix_signals;

use super::workspace::{canonical_member, expand_member_entry};
use super::{
    FileLines, WalkCtx, dedup_sorted, fs::files_with_extension, path_depth_factor,
    single_file_lines_content,
};

/// Damp `[package]` Identity on workspace-member Cargo.tomls — sub-
/// crate identity is mostly inherited from the workspace root.
const WORKSPACE_MEMBER_IDENTITY_FACTOR: f64 = 0.4;

/// In a primary-name collision, manifests that definitely are not a
/// primary candidate stay behind the equally damped candidates. This is
/// not promotion: every candidate retains the normal member damp.
const AMBIGUOUS_SECONDARY_IDENTITY_FACTOR: f64 = WORKSPACE_MEMBER_IDENTITY_FACTOR * 0.7;

const PYPROJECT_LEDE_IDENTITY_FACTOR: f64 = 0.5;
const PYPROJECT_HYBRID_LEDE_IDENTITY_FACTOR: f64 = 0.4;

/// Per-tool tables below this source-token size are packed with adjacent
/// small tables. This avoids turning a large config roster into a swarm of
/// 10--40 token scheduler trinkets while keeping useful config slices near
/// the 100--250 token target once line labels and gap markers are rendered.
const TOOL_CONFIG_FAMILY_MIN_TOKENS: usize = 120;
const TOOL_CONFIG_FAMILY_MAX_TOKENS: usize = 240;

type Section = (String, usize, usize);

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let toml_files = files_with_extension(dir, "toml", ctx);
    if toml_files.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in toml_files {
        let Some((source, tree)) = parse_toml(ctx, &file) else {
            continue;
        };
        let sections = collect_sections(&tree, &source);
        let python_project_manifest = is_python_project_manifest(&file, &sections, &source);
        if let Some(content) = build_section_content(
            &file,
            &source,
            &sections,
            |n| {
                matches!(n, "package" | "workspace" | "workspace.package")
                    || (python_project_manifest && matches!(n, "project" | "tool.poetry"))
            },
            python_project_manifest,
        ) {
            out.push(Batch {
                key: TomlKey::Identity { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: identity_value(&file, ctx),
            });
        }
        if let Some(content) = build_section_content(
            &file,
            &source,
            &sections,
            |n| is_scripts_section(n) && python_project_manifest,
            python_project_manifest,
        ) {
            out.push(Batch {
                key: TomlKey::Scripts { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: scripts_value(&file, ctx),
            });
        }
        if let Some(content) = build_section_content(
            &file,
            &source,
            &sections,
            |n| n == "features",
            python_project_manifest,
        ) {
            out.push(Batch {
                key: TomlKey::Features { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: features_value(&file, ctx),
            });
        }
        if let Some(content) =
            build_dependencies_content(&file, &source, &tree, &sections, python_project_manifest)
        {
            out.push(Batch {
                key: TomlKey::Dependencies { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: if is_cargo_manifest(&file) {
                    cargo_dependencies_value(&file, ctx)
                } else {
                    dependencies_value(&file, ctx)
                },
            });
        }
        if let Some(content) = build_development_dependencies_content(
            &file,
            &source,
            &sections,
            python_project_manifest,
        ) {
            out.push(Batch {
                key: TomlKey::DevelopmentDependencies { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: development_dependencies_value(&file, ctx),
            });
        }
        for (tool, content) in
            build_tool_config_contents(&file, &source, &sections, python_project_manifest)
        {
            out.push(Batch {
                key: TomlKey::ToolConfig {
                    file: file.clone(),
                    tool,
                }
                .into(),
                predecessor: None,
                content,
                value: config_value(&file, ctx),
            });
        }
        if let Some(content) =
            build_config_content(&file, &source, &sections, python_project_manifest)
        {
            out.push(Batch {
                key: TomlKey::Config { file: file.clone() }.into(),
                predecessor: None,
                content,
                value: config_value(&file, ctx),
            });
        }
    }
    out
}

/// Dependency content for Cargo (table-based) and pyproject (array
/// under `[project]`).
fn build_dependencies_content(
    file: &Path,
    source: &str,
    tree: &Tree,
    sections: &[Section],
    python_project_manifest: bool,
) -> Option<crate::content::BatchContent> {
    let cargo_manifest = is_cargo_manifest(file);
    let mut line_numbers: Vec<usize> = Vec::new();
    for (name, start, end) in sections {
        if (python_project_manifest && is_dependency_section(name))
            || (cargo_manifest && is_ordinary_dependency_section(name))
            // Non-manifest TOMLs keep the pre-split exclusion of
            // pyproject-shaped sections: a poetry/PEP-621 dep table in a
            // template or sample file is not a manifest roster.
            || (!python_project_manifest
                && !cargo_manifest
                && is_dependency_section(name)
                && !(name.starts_with("project.") || name.starts_with("tool.poetry.")))
        {
            line_numbers.extend(*start..=*end);
        }
    }
    if python_project_manifest {
        for key in ["dependencies", "optional-dependencies"] {
            if let Some((start, end)) = project_pair_array_rows(tree, source, key) {
                line_numbers.extend(start..=end);
            }
        }
    }
    if line_numbers.is_empty() {
        return None;
    }
    single_file_lines_content(file, source, FileLines::new(dedup_sorted(line_numbers)))
}

/// Cargo dependency classes that describe tests, build-time tooling, or a
/// platform-specific edge. They are useful context, but should not make the
/// ordinary runtime dependency roster unaffordable.
fn build_development_dependencies_content(
    file: &Path,
    source: &str,
    sections: &[Section],
    python_project_manifest: bool,
) -> Option<crate::content::BatchContent> {
    if python_project_manifest || !is_cargo_manifest(file) {
        return None;
    }
    let mut line_numbers = Vec::new();
    for (name, start, end) in sections {
        if is_cargo_development_dependency_section(name) {
            line_numbers.extend(*start..=*end);
        }
    }
    if line_numbers.is_empty() {
        return None;
    }
    single_file_lines_content(file, source, FileLines::new(dedup_sorted(line_numbers)))
}

fn build_tool_config_contents(
    file: &Path,
    source: &str,
    sections: &[Section],
    python_project_manifest: bool,
) -> Vec<(String, crate::content::BatchContent)> {
    if !should_partition_tool_config(file, source, sections, python_project_manifest) {
        return Vec::new();
    }
    pack_small_tool_config_families(source, collect_tool_config_lines(file, sections))
        .into_iter()
        .filter_map(|(tool, lines)| {
            single_file_lines_content(file, source, FileLines::new(dedup_sorted(lines)))
                .map(|content| (tool, content))
        })
        .collect()
}

fn pack_small_tool_config_families(
    source: &str,
    families: BTreeMap<String, Vec<usize>>,
) -> Vec<(String, Vec<usize>)> {
    let source_lines: Vec<&str> = source.lines().collect();
    let mut ordered: Vec<_> = families.into_iter().collect();
    ordered.sort_by_key(|(_, lines)| lines.first().copied().unwrap_or(usize::MAX));

    let mut packed = Vec::new();
    let mut small_family: Option<(Vec<String>, Vec<usize>, usize)> = None;
    for (tool, lines) in ordered {
        let tokens = tool_config_source_tokens(&source_lines, &lines);
        if tokens >= TOOL_CONFIG_FAMILY_MIN_TOKENS {
            // Flushing the small accumulator on a large family can
            // strand a lone sub-minimum pack (an interleaved layout's
            // leading small family stays solo). Measured better than
            // merging across large families: the small solo pack is a
            // cheap early buy, the merged pack schedules later
            // (htmy/tomli, 2026-07-18).
            flush_small_tool_family(&mut packed, &mut small_family);
            packed.push((tool, lines));
            continue;
        }

        if small_family
            .as_ref()
            .is_some_and(|(_, _, total)| total + tokens > TOOL_CONFIG_FAMILY_MAX_TOKENS)
        {
            flush_small_tool_family(&mut packed, &mut small_family);
        }
        let (tools, packed_lines, total) =
            small_family.get_or_insert_with(|| (Vec::new(), Vec::new(), 0));
        tools.push(tool);
        packed_lines.extend(lines);
        *total += tokens;
    }
    flush_small_tool_family(&mut packed, &mut small_family);
    packed
}

fn should_partition_tool_config(
    file: &Path,
    source: &str,
    sections: &[Section],
    python_project_manifest: bool,
) -> bool {
    if !python_project_manifest {
        return false;
    }
    let source_lines: Vec<&str> = source.lines().collect();
    let tool_tokens: usize = collect_tool_config_lines(file, sections)
        .values()
        .map(|lines| tool_config_source_tokens(&source_lines, lines))
        .sum();
    tool_tokens > TOOL_CONFIG_FAMILY_MAX_TOKENS
}

fn tool_config_source_tokens(source_lines: &[&str], lines: &[usize]) -> usize {
    let mut text = String::new();
    for line_number in lines {
        if let Some(line) = source_lines.get(line_number.saturating_sub(1)) {
            text.push_str(line);
            text.push('\n');
        }
    }
    crate::tokenizer::count(&text)
}

fn flush_small_tool_family(
    packed: &mut Vec<(String, Vec<usize>)>,
    small_family: &mut Option<(Vec<String>, Vec<usize>, usize)>,
) {
    if let Some((tools, lines, _)) = small_family.take() {
        packed.push((tools.join("+"), lines));
    }
}

fn collect_tool_config_lines(file: &Path, sections: &[Section]) -> BTreeMap<String, Vec<usize>> {
    let mut families: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (name, start, end) in sections {
        if !is_config_section(file, name) {
            continue;
        }
        let Some(tool) = top_level_tool_name(name) else {
            continue;
        };
        families.entry(tool).or_default().extend(*start..=*end);
    }
    families
}

fn build_config_content(
    file: &Path,
    source: &str,
    sections: &[Section],
    python_project_manifest: bool,
) -> Option<crate::content::BatchContent> {
    if !is_manifest_toml(file, python_project_manifest) {
        return None;
    }
    let partition_tool_config =
        should_partition_tool_config(file, source, sections, python_project_manifest);
    let mut line_numbers: Vec<usize> = Vec::new();
    for (name, start, end) in sections {
        if is_config_section(file, name)
            && (!partition_tool_config || top_level_tool_name(name).is_none())
        {
            line_numbers.extend(*start..=*end);
        }
    }
    if line_numbers.is_empty() {
        return None;
    }
    single_file_lines_content(file, source, FileLines::new(dedup_sorted(line_numbers)))
}

/// Row span of `<key> = [...]` inside the `[project]` table.
fn project_pair_array_rows(tree: &Tree, source: &str, key: &str) -> Option<(usize, usize)> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() != "table" {
            continue;
        }
        let Some(name) = extract_table_name(child, source) else {
            continue;
        };
        if name != "project" {
            continue;
        }
        let mut pair_cursor = child.walk();
        for pair in child.children(&mut pair_cursor) {
            if pair.kind() != "pair" {
                continue;
            }
            if pair_key_matches(pair, source, key) {
                let value_node = pair_value_node(pair)?;
                if value_node.kind() != "array" {
                    return None;
                }
                return Some((
                    pair.start_position().row + 1,
                    value_node.end_position().row + 1,
                ));
            }
        }
    }
    None
}

fn pair_key_matches(pair: Node, source: &str, key: &str) -> bool {
    let mut cursor = pair.walk();
    for child in pair.children(&mut cursor) {
        if matches!(child.kind(), "bare_key" | "dotted_key" | "quoted_key") {
            return normalize_key_path(source[child.start_byte()..child.end_byte()].trim()) == key;
        }
    }
    false
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

fn build_section_content(
    file: &Path,
    source: &str,
    sections: &[Section],
    name_match: impl Fn(&str) -> bool,
    python_project_manifest: bool,
) -> Option<crate::content::BatchContent> {
    let mut line_numbers: Vec<usize> = Vec::new();
    for (name, start_line, end_line) in sections {
        if name_match(name) {
            if is_pyproject_identity_table(name) {
                line_numbers.extend(project_identity_lines(
                    source,
                    *start_line,
                    *end_line,
                    python_project_manifest,
                ));
            } else {
                line_numbers.extend(*start_line..=*end_line);
            }
        }
    }
    if line_numbers.is_empty() {
        return None;
    }
    single_file_lines_content(file, source, FileLines::new(dedup_sorted(line_numbers)))
}

fn is_cargo_manifest(file: &Path) -> bool {
    file.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml")
}

fn is_python_project_manifest(file: &Path, sections: &[Section], source: &str) -> bool {
    match file.file_name().and_then(|n| n.to_str()) {
        Some("pyproject.toml") => true,
        Some("project.toml") => project_table_has_pep621_key(sections, source),
        _ => false,
    }
}

fn is_manifest_toml(file: &Path, python_project_manifest: bool) -> bool {
    matches!(
        file.file_name().and_then(|n| n.to_str()),
        Some("Cargo.toml" | "pyproject.toml")
    ) || (file.file_name().and_then(|n| n.to_str()) == Some("project.toml")
        && python_project_manifest)
}

fn project_table_has_pep621_key(sections: &[Section], source: &str) -> bool {
    let Some((_, start, end)) = sections.iter().find(|(name, _, _)| name == "project") else {
        return false;
    };
    source
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line_no = idx + 1;
            (line_no > *start && line_no <= *end).then_some(line)
        })
        .filter_map(|line| line.trim_start().split_once('=').map(|(key, _)| key.trim()))
        .any(is_pep621_project_key)
}

fn is_pep621_project_key(key: &str) -> bool {
    matches!(
        key,
        "name"
            | "version"
            | "description"
            | "readme"
            | "requires-python"
            | "license"
            | "authors"
            | "maintainers"
            | "keywords"
            | "classifiers"
            | "dependencies"
            | "dynamic"
    )
}

fn is_dependency_section(name: &str) -> bool {
    is_ordinary_dependency_section(name) || is_cargo_development_dependency_section(name)
}

fn is_ordinary_dependency_section(name: &str) -> bool {
    matches!(
        name,
        "dependencies"
            | "workspace.dependencies"
            | "tool.poetry.dependencies"
            | "project.optional-dependencies"
    ) || name.starts_with("dependencies.")
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

/// The first dotted segment after `tool.`, preserving quotes around a segment
/// that itself contains a literal dot. Input has already passed through
/// [`normalize_key_path`].
fn top_level_tool_name(name: &str) -> Option<String> {
    let rest = name.strip_prefix("tool.")?;
    if rest.is_empty() {
        return None;
    }
    let mut quote = None;
    for (index, ch) in rest.char_indices() {
        match quote {
            Some(q) if ch == q => quote = None,
            Some(_) => {}
            None if matches!(ch, '\'' | '"') => quote = Some(ch),
            None if ch == '.' => return Some(rest[..index].to_string()),
            None => {}
        }
    }
    Some(rest.to_string())
}

fn is_config_section(file: &Path, name: &str) -> bool {
    if is_pyproject_identity_table(name) || is_scripts_section(name) || is_dependency_section(name)
    {
        return false;
    }
    name == "build-system"
        || name == "project.urls"
        || name == "project.entry-points"
        || name.starts_with("tool.")
        || name.starts_with("package.metadata.")
        || name.starts_with("profile.")
        || (file.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml")
            && matches!(name, "bin" | "example" | "test" | "bench"))
}

fn project_identity_lines(
    source: &str,
    start_line: usize,
    end_line: usize,
    broad_scalars: bool,
) -> Vec<usize> {
    let mut out = vec![start_line];
    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        if line_no <= start_line || line_no > end_line {
            continue;
        }
        if is_project_scalar_pair_line(line, broad_scalars) {
            out.push(line_no);
        }
    }
    out
}

/// `broad_scalars` widens the captured keys from {name, description}
/// to the full PEP 621 lede for Python project manifests.
fn is_project_scalar_pair_line(line: &str, broad_scalars: bool) -> bool {
    let Some((key, value)) = line.trim_start().split_once('=') else {
        return false;
    };
    let key = key.trim();
    let in_set = if broad_scalars {
        matches!(
            key,
            "name" | "version" | "description" | "requires-python" | "license" | "readme"
        )
    } else {
        matches!(key, "name" | "description")
    };
    in_set
        && value
            .trim_start()
            .chars()
            .next()
            .is_some_and(|ch| ch != '[' && ch != '{')
}

fn identity_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let m = if let Some(factor) = pyproject_identity_factor(file, ctx) {
        factor
    } else if ctx.is_workspace_member(file) {
        if ctx
            .rust_state()
            .is_definite_secondary_member(file, ctx.root())
        {
            AMBIGUOUS_SECONDARY_IDENTITY_FACTOR
        } else {
            WORKSPACE_MEMBER_IDENTITY_FACTOR
        }
    } else {
        1.0
    };
    mix_signals(m, 0.7 * m, 0.85 * m, path_depth_factor(file, ctx))
}

fn pyproject_identity_factor(file: &Path, ctx: &WalkCtx) -> Option<f64> {
    // Pyproject shape — any TOML with `[project]` or `[tool.poetry]`,
    // regardless of filename. Cargo.toml uses `[package]` and won't match.
    let (source, tree) = parse_toml(ctx, file)?;
    let sections = collect_sections(&tree, &source);
    if !sections
        .iter()
        .any(|(name, _, _)| is_pyproject_identity_table(name))
    {
        return None;
    }
    // Reaching this point already proves the file is a Python project
    // manifest with an identity table. PEP 517 conventionally places
    // `[build-system]` first, so table position is not an identity signal.
    let has_package_json = file
        .parent()
        .is_some_and(|parent| parent.join("package.json").exists());
    Some(if has_package_json {
        PYPROJECT_HYBRID_LEDE_IDENTITY_FACTOR
    } else {
        PYPROJECT_LEDE_IDENTITY_FACTOR
    })
}

fn is_pyproject_identity_table(name: &str) -> bool {
    matches!(name, "project" | "tool.poetry")
}

fn features_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.75, 0.6, 0.5, path_depth_factor(file, ctx))
}

fn scripts_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Console entry points answer "how do I run this" — orientation that
    // a reader otherwise has to reconstruct from the source tree.
    mix_signals(0.70, 0.6, 0.65, path_depth_factor(file, ctx))
}

fn dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Pyproject lede manifests get a cat-axis bump — reuses the same
    // signal as `identity_value` so the two stay co-classified.
    let cat = match pyproject_identity_factor(file, ctx) {
        Some(PYPROJECT_LEDE_IDENTITY_FACTOR | PYPROJECT_HYBRID_LEDE_IDENTITY_FACTOR) => 0.55,
        _ => 0.4,
    };
    mix_signals(cat, 0.7, 0.4, path_depth_factor(file, ctx))
}

fn cargo_dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let (catastrophic, follow_up, zero_tool_call) = if file.parent() == Some(ctx.root()) {
        (0.75, 0.6, 0.5)
    } else {
        (0.4, 0.7, 0.4)
    };
    mix_signals(
        catastrophic,
        follow_up,
        zero_tool_call,
        path_depth_factor(file, ctx),
    )
}

fn development_dependencies_value(file: &Path, ctx: &WalkCtx) -> f64 {
    let (catastrophic, follow_up, zero_tool_call) = if file.parent() == Some(ctx.root()) {
        (0.4, 0.7, 0.4)
    } else {
        (0.32, 0.58, 0.32)
    };
    mix_signals(
        catastrophic,
        follow_up,
        zero_tool_call,
        path_depth_factor(file, ctx),
    )
}

fn config_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.45, 0.6, 0.45, path_depth_factor(file, ctx))
}

// --- parser ---

fn parse_toml(ctx: &WalkCtx, path: &Path) -> Option<(Arc<str>, Arc<Tree>)> {
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

/// Everything primary-member selection needs from the workspace members'
/// own manifests, gathered in a single read pass over them.
pub(super) struct MemberFacts {
    /// Members whose `[package].name` matches the repository basename
    /// case-insensitively.
    basename_matches: HashSet<PathBuf>,
    /// Member manifest → how many *sibling* members depend on it.
    in_degrees: HashMap<PathBuf, usize>,
    /// `false` when a member manifest could not be read or declares no
    /// package. The skipped member's edges are exactly the ones that could
    /// have changed the answer, so both questions below refuse to answer on
    /// facts known to be partial.
    complete: bool,
}

impl MemberFacts {
    /// The colliding candidates when more than one member's package name
    /// matches the repository basename. Callers use this explicit ambiguity
    /// state to fail closed instead of letting an exact-case basename
    /// shortcut select one candidate or a definite secondary win a cost tie.
    pub(super) fn ambiguous_primary(&self) -> Option<&HashSet<PathBuf>> {
        (self.complete && self.basename_matches.len() > 1).then_some(&self.basename_matches)
    }

    /// The single member whose package name matches the repository basename,
    /// when exactly one does. Breaks ties between several members whose
    /// *directory* shares the repo basename, since Cargo keeps package names
    /// unique within a workspace.
    pub(super) fn name_matched_member(&self) -> Option<&PathBuf> {
        let mut matches = self.basename_matches.iter();
        let only = matches.next()?;
        matches.next().is_none().then_some(only)
    }

    /// Whether any sibling member depends on `manifest` — i.e. whether it is
    /// inside the workspace's dependency fabric at all.
    pub(super) fn has_sibling_dependents(&self, manifest: &Path) -> bool {
        self.complete && self.in_degrees.get(manifest).is_some_and(|&d| d > 0)
    }

    /// The member the rest of the workspace is built on: the one with a
    /// strictly greater in-degree than every sibling, provided it clears
    /// `min_in_degree`. `None` for a flat workspace of independent crates,
    /// a tie at the top, or a dependency graph known to be incomplete.
    pub(super) fn dependency_hub(&self, min_in_degree: usize) -> Option<&PathBuf> {
        if !self.complete {
            return None;
        }
        let top = self.in_degrees.values().copied().max()?;
        if top < min_in_degree {
            return None;
        }
        let mut at_top = self
            .in_degrees
            .iter()
            .filter_map(|(manifest, &degree)| (degree == top).then_some(manifest));
        let hub = at_top.next()?;
        at_top.next().is_none().then_some(hub)
    }
}

/// The `path = "..."` of a dependency-style entry, if it has one.
fn dep_path(spec: &toml::Value) -> Option<&str> {
    spec.as_table()?.get("path")?.as_str()
}

fn package_name(manifest: &toml::Value) -> Option<&str> {
    manifest.get("package")?.as_table()?.get("name")?.as_str()
}

fn parse_manifest(path: &Path) -> Option<toml::Value> {
    toml::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

/// The workspace member whose crate dir is `dir`, if `dir` holds one.
fn member_at(canonical_root: &Path, members: &HashSet<PathBuf>, dir: &Path) -> Option<PathBuf> {
    let manifest = canonical_member(canonical_root, dir, CARGO_MANIFEST_FILENAME)?;
    members.contains(&manifest).then_some(manifest)
}

/// The root-manifest tables a member's `[dependencies]` entry may have to be
/// resolved through before it can be called an intra-workspace edge.
struct WorkspaceLinks<'a> {
    canonical_root: &'a Path,
    members: &'a HashSet<PathBuf>,
    /// Root `[workspace.dependencies]`, which `workspace = true` inherits.
    workspace_deps: Option<&'a toml::Table>,
    /// Crate name → the member a root `[patch.*]` table redirects it to.
    patched: HashMap<String, PathBuf>,
}

impl<'a> WorkspaceLinks<'a> {
    fn new(
        canonical_root: &'a Path,
        members: &'a HashSet<PathBuf>,
        root_manifest: Option<&'a toml::Value>,
    ) -> Self {
        let table = |value: Option<&'a toml::Value>, key| value?.as_table()?.get(key);
        let patch_registries = table(root_manifest, "patch").and_then(|v| v.as_table());
        let patched = patch_registries
            .into_iter()
            .flat_map(|registries| registries.values().filter_map(|v| v.as_table()))
            .flatten()
            .filter_map(|(name, spec)| {
                let dir = canonical_root.join(dep_path(spec)?);
                Some((name.clone(), member_at(canonical_root, members, &dir)?))
            })
            .collect();
        Self {
            canonical_root,
            members,
            workspace_deps: table(table(root_manifest, "workspace"), "dependencies")
                .and_then(|v| v.as_table()),
            patched,
        }
    }

    /// The sibling member a `[dependencies]` entry resolves to, or `None`
    /// when the dependency leaves the workspace.
    ///
    /// A registry entry resolves *only* when a root `[patch]` table proves
    /// the redirection. Matching a bare version requirement on package name
    /// alone would count a genuine crates.io dependency that happens to
    /// share a member's name as an internal edge, and two such consumers are
    /// enough to elect a hub nothing in the workspace depends on.
    fn resolve(&self, dependent_dir: &Path, key: &str, dep: &toml::Value) -> Option<PathBuf> {
        if let Some(path) = dep_path(dep) {
            return self.member_at(&dependent_dir.join(path));
        }
        let field = |name: &str| dep.as_table()?.get(name);
        let package = field("package").and_then(|v| v.as_str()).unwrap_or(key);
        if field("workspace").and_then(|v| v.as_bool()) == Some(true)
            && let Some(path) = self
                .workspace_deps
                .and_then(|deps| deps.get(package))
                .and_then(dep_path)
        {
            return self.member_at(&self.canonical_root.join(path));
        }
        self.patched.get(package).cloned()
    }

    fn member_at(&self, dir: &Path) -> Option<PathBuf> {
        member_at(self.canonical_root, self.members, dir)
    }
}

/// Read every workspace member's manifest once. An intra-workspace dependency
/// edge is a `[dependencies]` entry that *resolves* to a sibling member — via
/// its own `path`, via `workspace = true` through the root
/// `[workspace.dependencies]` table, or via a root `[patch]` redirection.
/// Each dependent→dependee pair counts once however many times it is declared.
pub(super) fn read_member_facts(root: &Path, members: &HashSet<PathBuf>) -> MemberFacts {
    let target = root.file_name().and_then(|n| n.to_str());
    let mut basename_matches = HashSet::new();
    let mut parsed: Vec<(&PathBuf, toml::Value)> = Vec::new();
    let mut complete = true;

    for manifest in members {
        let Some(value) = parse_manifest(manifest) else {
            complete = false;
            continue;
        };
        let Some(name) = package_name(&value) else {
            complete = false;
            continue;
        };
        if target.is_some_and(|target| name.eq_ignore_ascii_case(target)) {
            basename_matches.insert(manifest.clone());
        }
        parsed.push((manifest, value));
    }

    MemberFacts {
        basename_matches,
        in_degrees: member_in_degrees(root, members, &parsed),
        complete,
    }
}

/// How many *sibling* members depend on each member, over the members that
/// parsed. Members with no dependents are present with a zero.
fn member_in_degrees(
    root: &Path,
    members: &HashSet<PathBuf>,
    parsed: &[(&PathBuf, toml::Value)],
) -> HashMap<PathBuf, usize> {
    let mut in_degrees: HashMap<PathBuf, usize> =
        parsed.iter().map(|(m, _)| ((*m).clone(), 0)).collect();
    let Ok(canonical_root) = root.canonicalize() else {
        return in_degrees;
    };
    let root_manifest = parse_manifest(&root.join(CARGO_MANIFEST_FILENAME));
    let links = WorkspaceLinks::new(&canonical_root, members, root_manifest.as_ref());
    for (manifest, value) in parsed {
        let (Some(dir), Some(deps)) = (
            manifest.parent(),
            value.get("dependencies").and_then(|v| v.as_table()),
        ) else {
            continue;
        };
        let dependees: HashSet<PathBuf> = deps
            .iter()
            .filter_map(|(key, dep)| links.resolve(dir, key, dep))
            .filter(|dependee| &dependee != manifest)
            .collect();
        for dependee in dependees {
            *in_degrees.entry(dependee).or_default() += 1;
        }
    }
    in_degrees
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture_path(rel: &str) -> PathBuf {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root.join("tests/fixtures").join(rel)
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

    fn member(name: &str, deps: &str) -> String {
        format!("[package]\nname='{name}'\nversion='0.1.0'\n[dependencies]\n{deps}")
    }

    fn in_degree_of(dir: &Path, members: &HashSet<PathBuf>, rel: &str) -> usize {
        let manifest = dir.join(rel).join("Cargo.toml").canonicalize().unwrap();
        read_member_facts(dir, members)
            .in_degrees
            .get(&manifest)
            .copied()
            .unwrap_or(0)
    }

    /// Entries that name a member without resolving to it are not edges: an
    /// unpatched registry requirement is resolved from crates.io however much
    /// the name matches, and a `path` may point outside the member set. Left
    /// uncounted, two such consumers would elect a hub nothing depends on.
    #[test]
    fn walker_toml_member_in_degrees_ignore_unresolvable_lookalikes() {
        let (dir, members) = members_with(
            "[workspace]\nmembers=['acme','one','two','three']\nexclude=['vendored']\n",
            &[
                ("acme", &member("acme", "")),
                ("one", &member("one", "acme = '1.0'\n")),
                ("two", &member("two", "acme = { version = '1.0' }\n")),
                (
                    "three",
                    &member("three", "acme = { path = '../vendored' }\n"),
                ),
                ("vendored", &member("acme", "")),
            ],
        );
        assert_eq!(in_degree_of(dir.path(), &members, "acme"), 0);
        assert!(
            read_member_facts(dir.path(), &members)
                .dependency_hub(2)
                .is_none()
        );
    }

    /// The three ways a member really can name a sibling, plus the rule that a
    /// dependent counts once however many entries it routes through.
    #[test]
    fn walker_toml_member_in_degrees_resolve_path_workspace_and_patch_edges() {
        let (dir, members) = members_with(
            "[workspace]\nmembers=['hub','viapath','viaws','viapatch','twice']\n\
             [workspace.dependencies]\nhub = { path = 'hub' }\n\
             [patch.crates-io]\nhub = { path = 'hub' }\n",
            &[
                ("hub", &member("hub", "")),
                ("viapath", &member("viapath", "hub = { path = '../hub' }\n")),
                ("viaws", &member("viaws", "hub.workspace = true\n")),
                ("viapatch", &member("viapatch", "hub = '1.0'\n")),
                (
                    "twice",
                    &member(
                        "twice",
                        "hub = { path = '../hub' }\naliased = { path = '../hub', package = 'hub' }\n",
                    ),
                ),
            ],
        );
        assert_eq!(in_degree_of(dir.path(), &members, "hub"), 4);
    }

    /// An unreadable member manifest means the graph is missing exactly the
    /// edges that might have changed the answer, so no hub is elected.
    #[test]
    fn walker_toml_dependency_hub_fails_closed_on_an_unparseable_member() {
        let (dir, members) = members_with(
            "[workspace]\nmembers=['hub','one','two','broken']\n",
            &[
                ("hub", &member("hub", "")),
                ("one", &member("one", "hub = { path = '../hub' }\n")),
                ("two", &member("two", "hub = { path = '../hub' }\n")),
                ("broken", "[package\nname='broken'\n"),
            ],
        );
        let manifests = read_member_facts(dir.path(), &members);
        assert!(manifests.dependency_hub(2).is_none());
    }

    fn case_collision_workspace() -> (tempfile::TempDir, PathBuf, Vec<PathBuf>) {
        let outer = tempfile::tempdir().unwrap();
        let root = outer.path().join("acme");
        fs::create_dir(&root).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers=['crates/acme','crates/upper','crates/mixed','crates/other']\n",
        )
        .unwrap();
        let packages = [
            ("crates/acme", "acme"),
            ("crates/upper", "ACME"),
            ("crates/mixed", "Acme"),
            ("crates/other", "other"),
        ];
        let manifests = packages
            .iter()
            .map(|(rel, name)| {
                let dir = root.join(rel);
                fs::create_dir_all(dir.join("src")).unwrap();
                let manifest = dir.join("Cargo.toml");
                fs::write(
                    &manifest,
                    format!("[package]\nname='{name}'\nversion='0.1.0'\n"),
                )
                .unwrap();
                fs::write(dir.join("src/lib.rs"), "").unwrap();
                manifest.canonicalize().unwrap()
            })
            .collect();
        (outer, root, manifests)
    }

    #[test]
    fn walker_toml_primary_member_case_collision_fails_closed_without_inversion() {
        let (_outer, root, manifests) = case_collision_workspace();
        let members = collect_workspace_members(&root);
        let member_facts = read_member_facts(&root, &members);
        let ambiguous = member_facts.ambiguous_primary().unwrap();

        assert_eq!(ambiguous.len(), 3);
        assert!(manifests[..3].iter().all(|path| ambiguous.contains(path)));
        assert!(!ambiguous.contains(&manifests[3]));

        let ctx = WalkCtx::new(root.clone());
        let candidate_values: Vec<f64> = manifests[..3]
            .iter()
            .map(|manifest| identity_value(manifest, &ctx))
            .collect();
        assert!(candidate_values.windows(2).all(|pair| pair[0] == pair[1]));
        assert!(identity_value(&manifests[3], &ctx) < candidate_values[0]);

        let output = crate::render(&[root], 200, None).unwrap();
        assert!(
            ["acme", "ACME", "Acme"]
                .iter()
                .any(|name| output.contains(&format!("name='{name}'"))),
            "a colliding candidate must win before a definite secondary:\n{output}"
        );
        assert!(!output.contains("name='other'"), "output:\n{output}");
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
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_toml_ng::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let sections = collect_sections(&tree, source);
        assert_eq!(
            sections,
            vec![
                ("package".to_string(), 1, 4),
                ("bin".to_string(), 5, 7),
                ("dependencies".to_string(), 8, 9),
            ],
        );
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

        let broad = project_identity_lines(source, 1, 9, true);
        // Broad set: header + name (line 2) + description (line 4) +
        // requires-python (line 9). Array-valued pairs (dynamic /
        // authors / classifiers) and array-interior rows are excluded
        // by the `!= '['` value check and the "must contain `=`"
        // split, respectively.
        assert_eq!(broad, vec![1, 2, 4, 9]);

        let narrow = project_identity_lines(source, 1, 9, false);
        // Narrow set: header + name + description.
        assert_eq!(narrow, vec![1, 2, 4]);
    }

    #[test]
    fn walker_toml_project_toml_requires_pep621_project_table() {
        let parse_sections = |source: &str| {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&tree_sitter_toml_ng::LANGUAGE.into())
                .unwrap();
            let tree = parser.parse(source, None).unwrap();
            collect_sections(&tree, source)
        };

        let generic_project = "[project]\nowner = \"infra\"\n";
        let generic_sections = parse_sections(generic_project);
        assert!(!is_python_project_manifest(
            &PathBuf::from("project.toml"),
            &generic_sections,
            generic_project,
        ));

        let pep621_project = "[project]\nname = \"demo\"\ndependencies = [\"click\"]\n";
        let pep621_sections = parse_sections(pep621_project);
        assert!(is_python_project_manifest(
            &PathBuf::from("project.toml"),
            &pep621_sections,
            pep621_project,
        ));

        let pyproject_without_project = "[tool.ruff]\nline-length = 100\n";
        let pyproject_sections = parse_sections(pyproject_without_project);
        assert!(is_python_project_manifest(
            &PathBuf::from("pyproject.toml"),
            &pyproject_sections,
            pyproject_without_project,
        ));
    }

    /// Poetry-style pyproject (`[tool.poetry]` as lede table) is
    /// classified the same as PEP 621 `[project]`: same scalar filter,
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
documentation = "https://rich.readthedocs.io/en/latest/"
version = "15.0.0"
description = "Render rich text"
authors = ["Will McGugan <willmcgugan@gmail.com>"]
license = "MIT"
readme = "README.md"
"#;
        let broad = project_identity_lines(source, 1, 9, true);
        // Header (1) + name (2) + version (5) + description (6) +
        // license (8) + readme (9). homepage / documentation are not
        // in the broad set; authors is array-valued.
        assert_eq!(broad, vec![1, 2, 5, 6, 8, 9]);
    }

    #[test]
    fn walker_toml_config_sections_do_not_overlap_owned_sections() {
        let file = PathBuf::from("pyproject.toml");
        let owned = [
            "tool.poetry",
            "tool.poetry.dependencies",
            "tool.poetry.scripts",
            "dependencies.foo",
            "dev-dependencies.foo",
            "build-dependencies.foo",
            "target.'cfg(unix)'.dependencies",
            "target.'cfg(windows)'.dev-dependencies",
        ];
        for name in owned {
            assert!(
                !is_config_section(&file, name),
                "{name} must stay with its owning TOML batch"
            );
        }
        assert!(is_config_section(&file, "tool.ruff"));
        assert!(!is_config_section(&file, "test"));
        assert!(is_config_section(&PathBuf::from("Cargo.toml"), "test"));
    }

    #[test]
    fn walker_toml_tool_config_partitions_by_top_level_family() {
        assert_eq!(top_level_tool_name("tool.ruff.lint"), Some("ruff".into()));
        assert_eq!(
            top_level_tool_name("tool.pytest.ini_options"),
            Some("pytest".into())
        );
        assert_eq!(
            top_level_tool_name("tool.\"vendor.tool\".lint"),
            Some("\"vendor.tool\"".into())
        );
        assert_eq!(top_level_tool_name("project.urls"), None);

        let source = r#"[build-system]
requires = ["setuptools"]

[tool.pytest.ini_options]
xfail_strict = true

[tool.ruff]
src = ["src"]

[tool.ruff.lint]
select = ["E"]

[tool.mypy]
strict = true

[tool.poetry]
name = "demo"

[tool.poetry.dependencies]
python = ">=3.11"

[tool.poetry.group.test.dependencies]
pytest = "*"
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_toml_ng::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let sections = collect_sections(&tree, source);
        let families = collect_tool_config_lines(&PathBuf::from("pyproject.toml"), &sections);

        assert!(!should_partition_tool_config(
            &PathBuf::from("pyproject.toml"),
            source,
            &sections,
            true
        ));

        assert_eq!(
            families.keys().cloned().collect::<Vec<_>>(),
            ["mypy", "poetry", "pytest", "ruff"]
        );
        assert!(families["ruff"].contains(&7));
        assert!(families["ruff"].contains(&10));
        assert_eq!(families["poetry"], vec![22, 23]);

        let packed = pack_small_tool_config_families(source, families);
        assert_eq!(
            packed
                .iter()
                .map(|(tools, _)| tools.as_str())
                .collect::<Vec<_>>(),
            ["pytest+ruff+mypy+poetry"]
        );

        let oversized = format!(
            "[tool.ruff]\nselect = [{}]\n",
            std::iter::repeat_n("\"RULE\"", 300)
                .collect::<Vec<_>>()
                .join(", ")
        );
        let oversized_tree = parser.parse(&oversized, None).unwrap();
        let oversized_sections = collect_sections(&oversized_tree, &oversized);
        assert!(should_partition_tool_config(
            &PathBuf::from("pyproject.toml"),
            &oversized,
            &oversized_sections,
            true
        ));
    }

    #[test]
    fn walker_toml_cargo_dependency_classes_are_disjoint() {
        assert!(is_cargo_manifest(&PathBuf::from("Cargo.toml")));
        assert!(!is_cargo_manifest(&PathBuf::from("pyproject.toml")));
        assert!(!is_cargo_manifest(&PathBuf::from("config.toml")));

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

    /// The two real declaration styles the corpus exercises, pinned against
    /// the actual manifests: mdbook routes siblings through `workspace = true`
    /// plus the root `[workspace.dependencies]` table, while sps declares them
    /// by version and redirects with `[patch.crates-io]`.
    #[test]
    fn walker_toml_dependency_hub_on_real_workspaces() {
        for (fixture, expected) in [("mdbook", "crates/mdbook-core"), ("sps", "sps-common")] {
            let root = fixture_path(fixture);
            let members = collect_workspace_members(&root);
            let hub = read_member_facts(&root, &members)
                .dependency_hub(2)
                .cloned();
            let expected = root
                .join(expected)
                .join("Cargo.toml")
                .canonicalize()
                .unwrap();
            assert_eq!(hub, Some(expected), "{fixture}");
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
