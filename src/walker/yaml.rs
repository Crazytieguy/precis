//! YAML walker. Narrowly scoped to `docker-compose.{yml,yaml}` files —
//! the only YAML shape NS authors consistently anchor on (which services
//! exist, what images they run, which ports / volumes / env they wire).
//!
//! Scope rationale: docker-compose is the deployment-shape document for
//! a wide swath of self-hostable apps; without surfacing it an agent
//! has to make a `Read` call just to answer "how is this packaged".
//! Other YAML configs (GitHub workflows, CI configs, application YAML)
//! aren't unified enough to warrant per-shape walker work and stay
//! reachable via their parent dir listing.
//!
//! Implementation: hand-rolled, no tree-sitter dependency. The file is
//! emitted as a single `Whole` batch capped at [`COMPOSE_LINE_CAP`]
//! lines. Compose files are line-oriented enough that the whole-file
//! Whole batch is sufficient — splitting per-service would either
//! produce many tiny batches or require a real YAML parser to handle
//! block-style nesting; neither pays for itself on the current fixture
//! shape.
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
    FileLines, WalkCtx, fs::files_with_any_extension, path_depth_factor, single_file_lines_content,
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

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let mut out = Vec::new();
    for file in files_with_any_extension(dir, &["yml", "yaml"]) {
        let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !is_docker_compose_name(name) {
            continue;
        }
        let byte_len = std::fs::metadata(&file)
            .map(|m| m.len() as usize)
            .unwrap_or(usize::MAX);
        if byte_len > COMPOSE_BYTE_GATE {
            continue;
        }
        let Some(source) = ctx.read_source(&file) else {
            continue;
        };
        let line_count = source.lines().count();
        if line_count == 0 || line_count > COMPOSE_LINE_CAP {
            continue;
        }
        let lines: Vec<usize> = (1..=line_count).collect();
        let Some(content) = single_file_lines_content(&file, &source, FileLines::new(lines)) else {
            continue;
        };
        out.push(Batch {
            key: YamlKey::Whole { file: file.clone() }.into(),
            predecessor: None,
            content,
            value: compose_value(&file, ctx),
        });
    }
    out
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
    fn yaml_skips_non_compose_yaml() {
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
            "non-compose yaml should not emit Yaml::Whole; scheduled keys: {keys:?}",
        );
    }
}
