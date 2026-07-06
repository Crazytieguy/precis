//! YAML walker. Emits bounded whole-file batches for compact operational
//! YAML whose shape is more informative than its filename: compose
//! deployment files, CI workflows, lint/hook configs, and docs-site
//! configs.
//!
//! Implementation: hand-rolled, no tree-sitter dependency. The file is
//! emitted as a single `Whole` batch capped by class. These operational
//! configs are line-oriented enough that the whole-file Whole batch is
//! sufficient — splitting per-service/job/hook would either produce many
//! tiny batches or require a real YAML parser to handle block-style
//! nesting.
//!
//! **Secrets safety**: env values inlined in `environment:` blocks
//! (`DATABASE_URL=postgresql://postgres:${POSTGRES_PASSWORD}@…`) are
//! emitted verbatim. This mirrors how `precis` already treats every
//! other file: it's a "value-per-token summary," not a credential
//! scrubber. Compose files conventionally use `${VAR}` interpolation
//! rather than baking secrets directly; baked-in credentials are an
//! upstream-fixture choice the agent would also see on `Read`.

use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchKey, YamlKey};
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, fs::files_with_any_extension, gated_whole_file_content, path_depth_factor,
    single_file_lines_content,
};

/// Hard cap on the number of source lines a docker-compose file may
/// have to be considered for a `Whole` batch. Typical real-world
/// compose files are well under this (the linkwarden fixture is 28
/// lines, beszel's 18, audiobookshelf's 27); above the cap the file is
/// dropped wholesale rather than partially rendered. A long compose
/// file with many services is `Read`-territory anyway — the whole
/// point of the Whole batch is the cheap orientation hit.
const COMPOSE_LINE_CAP: usize = 80;

/// FS-metadata pre-flight gate: skip files whose raw byte size is
/// obviously past the line cap before opening them. 100 bytes/line is a
/// generous bound for indented YAML (the linkwarden fixture averages
/// ~30 chars/line).
const COMPOSE_BYTE_GATE: usize = COMPOSE_LINE_CAP * 100;

/// Compact CI/tooling configs stay cheap enough to render whole. Larger
/// workflows/configs are intentionally left for explicit reads rather
/// than partially summarized by a YAML parser we do not have.
const TOOLING_LINE_CAP: usize = 80;
const TOOLING_BYTE_GATE: usize = TOOLING_LINE_CAP * 120;
const TOOLING_HEAD_LINE_CAP: usize = 80;
const TOOLING_HEAD_BYTE_GATE: usize = TOOLING_HEAD_LINE_CAP * 200;
const WORKFLOW_HEAD_LINE_CAP: usize = 60;
const WORKFLOW_HEAD_BYTE_GATE: usize = 3_000;
const REFERENCE_MAP_BYTE_GATE: usize = 80_000;
const REFERENCE_MAP_KEY_LINE_CAP: usize = 80;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    let yaml_files = files_with_any_extension(dir, &["yml", "yaml"]);
    let workflow_file_count = is_github_workflow_dir(dir, ctx).then_some(yaml_files.len());
    let primary_workflow = workflow_file_count
        .filter(|count| *count > 2)
        .and_then(|_| primary_ci_workflow(&yaml_files));
    for file in yaml_files {
        let Some(class) = yaml_class(&file, ctx, workflow_file_count, primary_workflow.as_ref())
        else {
            continue;
        };
        let Some(content) = class.content(&file, ctx) else {
            continue;
        };
        let key = match class {
            YamlClass::ReferenceMap => YamlKey::TopLevelKeys { file: file.clone() },
            _ => YamlKey::Whole { file: file.clone() },
        };
        out.push(Batch {
            key: key.into(),
            predecessor: None,
            content,
            value: class.value(&file, ctx),
        });
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum YamlClass {
    Compose,
    Workflow,
    WorkflowPeripheral,
    Travis,
    Lint,
    Hook,
    DocsSite,
    ReferenceMap,
}

impl YamlClass {
    fn content(self, file: &Path, ctx: &WalkCtx) -> Option<crate::content::BatchContent> {
        match self {
            YamlClass::Compose => {
                gated_whole_file_content(file, ctx, COMPOSE_BYTE_GATE, COMPOSE_LINE_CAP)
            }
            YamlClass::Workflow | YamlClass::WorkflowPeripheral => head_capped_yaml_content(
                file,
                ctx,
                TOOLING_BYTE_GATE,
                TOOLING_LINE_CAP,
                WORKFLOW_HEAD_BYTE_GATE,
                WORKFLOW_HEAD_LINE_CAP,
            ),
            YamlClass::Travis | YamlClass::Lint | YamlClass::Hook | YamlClass::DocsSite => {
                head_capped_yaml_content(
                    file,
                    ctx,
                    TOOLING_BYTE_GATE,
                    TOOLING_LINE_CAP,
                    TOOLING_HEAD_BYTE_GATE,
                    TOOLING_HEAD_LINE_CAP,
                )
            }
            YamlClass::ReferenceMap => reference_map_key_content(file, ctx),
        }
    }

    fn value(self, file: &Path, ctx: &WalkCtx) -> f64 {
        match self {
            YamlClass::Compose => compose_value(file, ctx),
            YamlClass::Workflow | YamlClass::Travis => ci_value(file, ctx),
            YamlClass::WorkflowPeripheral => peripheral_ci_value(file, ctx),
            YamlClass::Lint | YamlClass::Hook => lint_hook_value(file, ctx),
            YamlClass::DocsSite => docs_site_value(file, ctx),
            YamlClass::ReferenceMap => reference_map_value(file, ctx),
        }
    }
}

fn reference_map_key_content(file: &Path, ctx: &WalkCtx) -> Option<crate::content::BatchContent> {
    let byte_len = std::fs::metadata(file)
        .map(|m| m.len() as usize)
        .unwrap_or(usize::MAX);
    if byte_len > REFERENCE_MAP_BYTE_GATE {
        return None;
    }
    let source = ctx.read_source(file)?;
    let lines = reference_map_key_lines(&source);
    if lines.full.len() > REFERENCE_MAP_KEY_LINE_CAP {
        return None;
    }
    single_file_lines_content(file, &source, lines)
}

fn head_capped_yaml_content(
    file: &Path,
    ctx: &WalkCtx,
    whole_byte_gate: usize,
    whole_line_cap: usize,
    head_byte_gate: usize,
    head_line_cap: usize,
) -> Option<crate::content::BatchContent> {
    if let Some(content) = gated_whole_file_content(file, ctx, whole_byte_gate, whole_line_cap) {
        return Some(content);
    }

    let byte_len = std::fs::metadata(file)
        .map(|m| m.len() as usize)
        .unwrap_or(usize::MAX);
    if byte_len > head_byte_gate {
        return None;
    }
    let source = ctx.read_source(file)?;
    let line_count = source.lines().count();
    if line_count == 0 || line_count <= whole_line_cap {
        return None;
    }
    let head_line_count = head_line_cap.min(line_count);
    single_file_lines_content(
        file,
        &source,
        FileLines::new((1..=head_line_count).collect()).with_ellipses(vec![head_line_count + 1]),
    )
}

fn yaml_class(
    file: &Path,
    ctx: &WalkCtx,
    workflow_file_count: Option<usize>,
    primary_workflow: Option<&PathBuf>,
) -> Option<YamlClass> {
    let name = file.file_name()?.to_str()?;
    if is_docker_compose_name(name) {
        return Some(YamlClass::Compose);
    }
    if is_github_workflow(file, ctx) {
        let count = workflow_file_count.unwrap_or(usize::MAX);
        if count <= 2 {
            return Some(workflow_class_for_name(name));
        }
        if primary_workflow.is_some_and(|primary| primary == file) {
            return Some(YamlClass::WorkflowPeripheral);
        }
    }
    if name.eq_ignore_ascii_case(".travis.yml") {
        return Some(YamlClass::Travis);
    }
    if name.eq_ignore_ascii_case(".golangci.yml") || name.eq_ignore_ascii_case(".golangci.yaml") {
        return Some(YamlClass::Lint);
    }
    if name.eq_ignore_ascii_case(".pre-commit-config.yaml") {
        return Some(YamlClass::Hook);
    }
    if name.eq_ignore_ascii_case("mkdocs.yml") && is_root_file(file, ctx) {
        return Some(YamlClass::DocsSite);
    }
    if is_root_file(file, ctx) && is_reference_map_name(name) {
        return Some(YamlClass::ReferenceMap);
    }
    None
}

fn is_reference_map_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let stem = lower
        .strip_suffix(".yaml")
        .or_else(|| lower.strip_suffix(".yml"))
        .unwrap_or(&lower);
    stem == "reference"
        || stem == "references"
        || stem == "api"
        || stem == "api-reference"
        || stem == "api_reference"
        || stem == "openapi"
        || stem == "swagger"
        || stem == "spec"
        || stem == "schema"
}

fn reference_map_key_lines(source: &str) -> FileLines {
    let mut full = Vec::new();
    let mut ellipses = Vec::new();
    let mut previous_kept = None;
    for (idx, raw) in source.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with('-')
            || !trimmed.contains(':')
        {
            continue;
        }
        if raw
            .chars()
            .next()
            .is_some_and(|c| c.is_whitespace() && c != ' ')
        {
            continue;
        }
        let indent = raw.chars().take_while(|c| *c == ' ').count();
        if indent > 2 {
            continue;
        }
        if is_yaml_document_marker(trimmed) {
            continue;
        }
        if let Some(prev) = previous_kept
            && line_no > prev + 1
        {
            ellipses.push(prev + 1);
        }
        full.push(line_no);
        previous_kept = Some(line_no);
    }
    FileLines::new(full).with_ellipses(ellipses)
}

fn is_yaml_document_marker(trimmed: &str) -> bool {
    matches!(trimmed, "---" | "...")
}

fn workflow_name_rank(name: &str) -> Option<usize> {
    let lower = name.to_ascii_lowercase();
    let stem = lower
        .strip_suffix(".yaml")
        .or_else(|| lower.strip_suffix(".yml"))
        .unwrap_or(&lower);
    if stem.contains("release") || stem.contains("publish") || stem.contains("deploy") {
        return None;
    }
    if stem == "ci" {
        Some(0)
    } else if matches!(stem, "test" | "tests") {
        Some(1)
    } else if stem == "build" {
        Some(2)
    } else if stem == "lint" {
        Some(3)
    } else if stem == "main" {
        Some(4)
    } else if matches!(stem, "node.js" | "node") {
        Some(5)
    } else if stem.ends_with("-test") || stem.ends_with("-tests") {
        Some(6)
    } else {
        None
    }
}

fn legacy_workflow_name_rank(name: &str) -> Option<usize> {
    let lower = name.to_ascii_lowercase();
    let stem = lower
        .strip_suffix(".yaml")
        .or_else(|| lower.strip_suffix(".yml"))
        .unwrap_or(&lower);
    if stem == "ci" {
        Some(0)
    } else if matches!(stem, "test" | "tests") {
        Some(1)
    } else if stem == "main" {
        Some(4)
    } else if matches!(stem, "node.js" | "node") {
        Some(5)
    } else if stem.ends_with("-test") || stem.ends_with("-tests") {
        Some(6)
    } else {
        None
    }
}

fn workflow_class_for_name(name: &str) -> YamlClass {
    if legacy_workflow_name_rank(name).is_some() {
        YamlClass::Workflow
    } else {
        YamlClass::WorkflowPeripheral
    }
}

fn primary_ci_workflow(files: &[PathBuf]) -> Option<PathBuf> {
    files
        .iter()
        .filter_map(|file| {
            let name = file.file_name()?.to_str()?;
            workflow_name_rank(name).map(|rank| (rank, name.to_ascii_lowercase(), file.clone()))
        })
        .min_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)))
        .map(|(_, _, file)| file)
}

fn is_github_workflow(file: &Path, ctx: &WalkCtx) -> bool {
    let Ok(rel) = file.strip_prefix(ctx.root()) else {
        return false;
    };
    let mut components = rel.components().filter_map(|c| c.as_os_str().to_str());
    components.next().is_some_and(|c| c == ".github")
        && components.next().is_some_and(|c| c == "workflows")
        && components.next().is_some()
        && components.next().is_none()
}

fn is_github_workflow_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    let Ok(rel) = dir.strip_prefix(ctx.root()) else {
        return false;
    };
    let mut components = rel.components().filter_map(|c| c.as_os_str().to_str());
    components.next().is_some_and(|c| c == ".github")
        && components.next().is_some_and(|c| c == "workflows")
        && components.next().is_none()
}

fn is_root_file(file: &Path, ctx: &WalkCtx) -> bool {
    file.parent().is_some_and(|p| p == ctx.root())
}

/// True iff `name` is a docker-compose filename. Matches the base
/// `docker-compose.{yml,yaml}` plus the conventional `compose.{yml,yaml}`
/// shorthand introduced in Compose Spec v2. The dotted-variant pattern
/// (`docker-compose.prod.yml`, `compose.override.yaml`) is intentionally
/// not matched — those overlay files describe environment-specific
/// overrides that are rarely the canonical deployment shape.
fn is_docker_compose_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "docker-compose.yml" | "docker-compose.yaml" | "compose.yml" | "compose.yaml"
    )
}

fn compose_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // docker-compose is the deployment-shape anchor for any
    // self-hostable app: which services exist, what images they run,
    // which ports/volumes/env they wire. For a Linkwarden- /
    // Audiobookshelf- / Linkding-shaped repo the compose file *is*
    // the canonical answer to "how do I run this," with no equally
    // structured substitute in source. Weighted near `package.json`
    // Runtime — catastrophic-omission signal is high (an agent
    // asking "what's the postgres image" or "what ports does this
    // expose" should not need a `Read` call), zero-tool-call value
    // is high (the rendered compose file directly answers
    // deployment questions), follow-up-minimization is mid (the
    // file rarely points the agent at downstream files).
    mix_signals(0.6, 0.45, 0.65, path_depth_factor(file, ctx))
}

fn ci_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // CI YAML answers the operational "what versions/platforms/checks
    // gate this project" question once the workflow directory is known.
    // The strong value is reserved for CI-like primary names; compact
    // arbitrary workflow stems use `peripheral_ci_value`.
    mix_signals(3.2, 1.2, 2.0, path_depth_factor(file, ctx))
}

fn peripheral_ci_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Compact workflow directories can contain release/publish/docs
    // automation whose filename is not a CI signal. Admit them for
    // recall, but keep them below source/API anchors.
    mix_signals(0.25, 0.20, 0.20, path_depth_factor(file, ctx))
}

fn lint_hook_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Lint/hook configs are compact policy rosters: enabled linters,
    // exclusions, and hook ids. They are broad operational context but
    // usually secondary to source APIs and runtime manifests.
    mix_signals(1.3, 0.65, 1.0, path_depth_factor(file, ctx))
}

fn docs_site_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // Root docs-site YAML names the published documentation structure
    // and plugins. It is useful orientation for docs-heavy repos, but
    // less universally load-bearing than CI or lint policy.
    mix_signals(1.0, 0.55, 0.85, path_depth_factor(file, ctx))
}

fn reference_map_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // A root reference/spec map is often the structured source of truth
    // for public API docs. Its root and child keys give the agent a
    // compact "what domains and entries exist here" catalog without
    // spending budget on each nested entry's prose.
    mix_signals(1.9, 1.0, 1.45, path_depth_factor(file, ctx))
}

#[cfg(test)]
mod tests {
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    use super::*;

    #[test]
    fn yaml_recognizes_docker_compose_filenames() {
        for name in [
            "docker-compose.yml",
            "docker-compose.yaml",
            "compose.yml",
            "compose.yaml",
            "Docker-Compose.YML",
        ] {
            assert!(is_docker_compose_name(name), "{name} should match");
        }
        for name in [
            // Overlay/override variants — out of scope by design.
            "docker-compose.prod.yml",
            "compose.override.yaml",
            // Other YAML filenames we don't claim.
            "ci.yml",
            "pnpm-workspace.yaml",
            "config.yaml",
        ] {
            assert!(!is_docker_compose_name(name), "{name} should not match");
        }
    }

    #[test]
    fn yaml_classifies_tooling_configs() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let ctx = WalkCtx::new(root.to_path_buf());

        let workflow = root.join(".github/workflows/test.yml");
        std::fs::create_dir_all(workflow.parent().unwrap()).unwrap();
        std::fs::write(&workflow, "name: test\non: [push]\n").unwrap();

        for (path, class) in [
            (workflow, YamlClass::Workflow),
            (root.join(".travis.yml"), YamlClass::Travis),
            (root.join(".golangci.yml"), YamlClass::Lint),
            (root.join(".golangci.yaml"), YamlClass::Lint),
            (root.join(".pre-commit-config.yaml"), YamlClass::Hook),
            (root.join("mkdocs.yml"), YamlClass::DocsSite),
        ] {
            if !path.exists() {
                std::fs::write(&path, "key: value\n").unwrap();
            }
            let workflow_file_count = is_github_workflow(&path, &ctx).then_some(1);
            assert_eq!(
                yaml_class(&path, &ctx, workflow_file_count, None),
                Some(class),
                "{path:?}"
            );
        }
    }

    #[test]
    fn yaml_compact_workflow_dir_accepts_any_stem() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let ctx = WalkCtx::new(root.to_path_buf());
        let workflow = root.join(".github/workflows/release.yml");
        std::fs::create_dir_all(workflow.parent().unwrap()).unwrap();
        std::fs::write(&workflow, "name: release\non: [push]\n").unwrap();

        assert_eq!(
            yaml_class(&workflow, &ctx, Some(1), None),
            Some(YamlClass::WorkflowPeripheral)
        );
    }

    #[test]
    fn yaml_large_workflow_dir_picks_one_ci_primary() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let ctx = WalkCtx::new(root.to_path_buf());
        let workflow_dir = root.join(".github/workflows");
        std::fs::create_dir_all(&workflow_dir).unwrap();
        let files = ["release.yml", "publish.yml", "lint.yml", "build.yml"]
            .into_iter()
            .map(|name| {
                let path = workflow_dir.join(name);
                std::fs::write(&path, "name: x\non: [push]\n").unwrap();
                path
            })
            .collect::<Vec<_>>();
        let primary = primary_ci_workflow(&files);
        let expected_primary = workflow_dir.join("build.yml");

        assert_eq!(primary.as_deref(), Some(expected_primary.as_path()));
        for file in files {
            let expected = if file.ends_with("build.yml") {
                Some(YamlClass::WorkflowPeripheral)
            } else {
                None
            };
            assert_eq!(
                yaml_class(&file, &ctx, Some(4), primary.as_ref()),
                expected,
                "{file:?}"
            );
        }
    }

    #[test]
    fn yaml_emits_whole_batch_for_compose_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("docker-compose.yml"),
            "services:\n  postgres:\n    image: postgres:16-alpine\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            rendered.contains("postgres:16-alpine"),
            "rendered output is missing docker-compose body:\n{rendered}",
        );
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Yaml(YamlKey::Whole { .. }))),
            "missing Yaml::Whole batch; scheduled keys: {keys:?}",
        );
    }

    #[test]
    fn yaml_oversized_compose_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // > COMPOSE_LINE_CAP lines, each short enough to keep the byte
        // count under the byte gate so the line-count check fires.
        let body: String = (0..(COMPOSE_LINE_CAP + 10))
            .map(|i| format!("  svc{i}: image\n"))
            .collect();
        std::fs::write(root.join("docker-compose.yml"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            !keys
                .iter()
                .any(|k| matches!(k, BatchKey::Yaml(YamlKey::Whole { .. }))),
            "unexpected Yaml::Whole batch for oversized compose; scheduled keys: {keys:?}",
        );
    }

    #[test]
    fn yaml_skips_unclassified_yaml() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("config.yaml"), "key: value\n").unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            !keys
                .iter()
                .any(|k| matches!(k, BatchKey::Yaml(YamlKey::Whole { .. }))),
            "unclassified yaml should not emit Yaml::Whole; scheduled keys: {keys:?}",
        );
    }

    #[test]
    fn yaml_emits_root_reference_map_top_level_keys() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("reference.yaml"),
            "constructors:\n  vec_f32:\n    desc: Vector constructors\n    functions:\n      - vec_f32\nmeta:\n  vec_version:\n    params: []\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(rendered.contains("constructors:"), "{rendered}");
        assert!(rendered.contains("meta:"), "{rendered}");
        assert!(rendered.contains("vec_f32:"), "{rendered}");
        assert!(rendered.contains("vec_version:"), "{rendered}");
        assert!(!rendered.contains("params"), "{rendered}");
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter()
                .any(|k| matches!(k, BatchKey::Yaml(YamlKey::TopLevelKeys { .. }))),
            "missing Yaml::TopLevelKeys batch; scheduled keys: {keys:?}",
        );
    }

    #[test]
    fn yaml_reference_map_rule_is_root_only() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let nested = root.join("docs/reference.yaml");
        std::fs::create_dir_all(nested.parent().unwrap()).unwrap();
        std::fs::write(nested, "constructors:\n  - vec_f32\n").unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            !keys
                .iter()
                .any(|k| matches!(k, BatchKey::Yaml(YamlKey::TopLevelKeys { .. }))),
            "nested reference map should not emit Yaml::TopLevelKeys; scheduled keys: {keys:?}",
        );
    }

    #[test]
    fn yaml_emits_whole_batch_for_small_github_workflow() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let workflow_dir = root.join(".github/workflows");
        std::fs::create_dir_all(&workflow_dir).unwrap();
        std::fs::write(
            workflow_dir.join("test.yml"),
            "name: test\non: [push]\njobs:\n  test:\n    runs-on: ubuntu-latest\n",
        )
        .unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            rendered.contains("runs-on: ubuntu-latest"),
            "rendered output is missing workflow body:\n{rendered}",
        );
    }

    #[test]
    fn yaml_emits_head_for_long_tooling_config() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let body: String = (1..=(TOOLING_HEAD_LINE_CAP + 10))
            .map(|i| format!("rule_{i}: true\n"))
            .collect();
        std::fs::write(root.join(".golangci.yml"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            rendered.contains("rule_80: true"),
            "rendered output is missing capped tooling head:\n{rendered}",
        );
        assert!(
            !rendered.contains("rule_90: true"),
            "rendered output should not include lines past the capped tooling head:\n{rendered}",
        );
    }
}
