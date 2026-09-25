//! YAML walker. Emits bounded whole-file batches for compact operational
//! YAML whose shape is more informative than its filename: compose
//! deployment files, CI workflows, lint/hook configs, and docs-site
//! configs.
//!
//! Implementation: hand-rolled, no tree-sitter dependency. The file is
//! emitted as a single `Whole` batch capped by class. These operational
//! configs are line-oriented enough that the whole-file `Whole` batch is
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

use std::path::Path;

use crate::batch::{Batch, BatchKey, YamlKey};
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, fs::files_with_any_extension, gated_read_source, gated_whole_file_content,
    path_depth_factor, single_file_lines_content,
};

/// Hard cap on the number of source lines a docker-compose file may
/// have to be considered for a whole-file body. Typical real-world
/// compose files are well under this (the linkwarden fixture is 28
/// lines, beszel's 18, audiobookshelf's 27); above the cap the file is
/// left for an explicit read.
const COMPOSE_LINE_CAP: usize = 80;

/// FS-metadata pre-flight gate: skip files whose raw byte size is
/// obviously past the line cap before opening them. 100 bytes/line is a
/// generous bound for indented YAML (the linkwarden fixture averages
/// ~30 chars/line).
const COMPOSE_BYTE_GATE: usize = COMPOSE_LINE_CAP * 100;

/// Compact CI/tooling configs render whole; longer ones render a
/// capped head rather than nothing.
const CONFIG_LINE_CAP: usize = 80;
const CONFIG_HEAD_LINE_CAP: usize = 60;
const CONFIG_BYTE_GATE: usize = CONFIG_LINE_CAP * 200;

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let in_workflow_dir = is_github_workflow_dir(dir, ctx);
    let mut out = Vec::new();
    for file in files_with_any_extension(dir, &["yml", "yaml"], ctx) {
        let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let (content, value) = if is_docker_compose_name(name) {
            (
                gated_whole_file_content(&file, ctx, COMPOSE_BYTE_GATE, COMPOSE_LINE_CAP),
                compose_value(&file, ctx),
            )
        } else if in_workflow_dir || is_tooling_config(name, dir == ctx.root()) {
            (config_content(&file, ctx), config_value(&file, ctx))
        } else {
            continue;
        };
        if let Some(content) = content {
            out.push(Batch {
                key: YamlKey::Whole { file: file.clone() }.into(),
                predecessor: None,
                content,
                value,
            });
        }
    }
    out
}

fn is_tooling_config(name: &str, at_root: bool) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        ".travis.yml" | ".golangci.yml" | ".golangci.yaml" | ".pre-commit-config.yaml"
    ) || (at_root && lower == "mkdocs.yml")
}

fn config_content(file: &Path, ctx: &WalkCtx) -> Option<crate::content::BatchContent> {
    let source = gated_read_source(file, ctx, CONFIG_BYTE_GATE)?;
    let line_count = source.lines().count();
    if line_count == 0 {
        return None;
    }
    if line_count <= CONFIG_LINE_CAP {
        return super::whole_file_lines_content(file, &source);
    }
    single_file_lines_content(
        file,
        &source,
        FileLines::new((1..=CONFIG_HEAD_LINE_CAP).collect())
            .with_ellipses(vec![CONFIG_HEAD_LINE_CAP + 1]),
    )
}

fn is_github_workflow_dir(dir: &Path, ctx: &WalkCtx) -> bool {
    dir.strip_prefix(ctx.root())
        .is_ok_and(|rel| rel == Path::new(".github/workflows"))
}

/// True iff `name` has the exact `docker-compose` / `compose` stem, or adds
/// an environment variant separated by `.` / `-`, case-insensitively.
fn is_docker_compose_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let Some(stem) = lower
        .strip_suffix(".yaml")
        .or_else(|| lower.strip_suffix(".yml"))
    else {
        return false;
    };
    ["docker-compose", "compose"].into_iter().any(|base| {
        stem == base
            || stem
                .strip_prefix(base)
                .and_then(|suffix| {
                    suffix
                        .strip_prefix('.')
                        .or_else(|| suffix.strip_prefix('-'))
                })
                .is_some_and(|variant| !variant.is_empty())
    })
}

fn compose_value(file: &Path, ctx: &WalkCtx) -> f64 {
    // docker-compose is the deployment-shape anchor for any
    // self-hostable app: which services exist, what images they run,
    // which ports/volumes/env they wire. For a Linkwarden- /
    // Audiobookshelf- / Linkding-shaped repo the compose file *is*
    // the canonical answer to "how do I run this," with no equally
    // structured substitute in source.
    mix_signals(0.6, 0.45, 0.65, path_depth_factor(file, ctx))
}

/// CI workflows and lint/hook/docs-site configs describe the
/// contributor's toolchain rather than the project, so they price as
/// peripheral config.
fn config_value(file: &Path, ctx: &WalkCtx) -> f64 {
    mix_signals(0.25, 0.20, 0.20, path_depth_factor(file, ctx))
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
            "docker-compose.prod.yml",
            "docker-compose-prod.yml",
            "compose.override.yaml",
            "compose-dev.yaml",
        ] {
            assert!(is_docker_compose_name(name), "{name} should match");
        }
        for name in [
            // Other YAML filenames we don't claim.
            "ci.yml",
            "pnpm-workspace.yaml",
            "config.yaml",
            "composer.yml",
            "composefile.yml",
            "composed.yaml",
            "compose-.yml",
            "docker-compose..yaml",
        ] {
            assert!(!is_docker_compose_name(name), "{name} should not match");
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
        let body: String = (1..=(CONFIG_LINE_CAP + 10))
            .map(|i| format!("rule_{i}: true\n"))
            .collect();
        std::fs::write(root.join(".golangci.yml"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            rendered.contains(&format!("rule_{CONFIG_HEAD_LINE_CAP}: true")),
            "rendered output is missing capped tooling head:\n{rendered}",
        );
        assert!(
            !rendered.contains(&format!("rule_{}: true", CONFIG_HEAD_LINE_CAP + 1)),
            "rendered output should not include lines past the capped tooling head:\n{rendered}",
        );
    }

    /// A repo whose CI lives in one long workflow must not render just
    /// the filename at every budget.
    #[test]
    fn yaml_emits_head_for_long_workflow() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let workflow_dir = root.join(".github/workflows");
        std::fs::create_dir_all(&workflow_dir).unwrap();
        let body: String = std::iter::once("name: ci\n".to_string())
            .chain((1..=200).map(|i| format!("  step_{i}: run some reasonably long command\n")))
            .collect();
        assert!(body.len() <= CONFIG_BYTE_GATE);
        for name in ["ci.yml", "stale.yml", "codeql.yml"] {
            std::fs::write(workflow_dir.join(name), &body).unwrap();
        }

        let report = Scheduler::new(root.to_path_buf(), FsWalker, 100_000, None).run_with_report();
        let rendered = report.tree.render();

        assert!(rendered.contains("step_1: run"), "{rendered}");
        assert!(
            rendered.contains(&format!("step_{}: run", CONFIG_HEAD_LINE_CAP - 1)),
            "{rendered}",
        );
        assert!(
            !rendered.contains(&format!("step_{}: run", CONFIG_HEAD_LINE_CAP + 1)),
            "head must stay bounded by the line cap:\n{rendered}",
        );
    }
}
