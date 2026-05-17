//! Plaintext walker. Emits `Whole` content batches for small,
//! known-plaintext config / license files that none of the format-aware
//! walkers (Rust, Markdown, TOML, JSON, TypeScript) cover. Today these
//! files only appear in directory listings; without this walker their
//! content is unreachable from the scheduler.
//!
//! Whitelist is deliberately narrow (see [`classify_plaintext`]). Files
//! with format-specific siblings (`.eslintrc.json`, `.prettierrc.js`,
//! `LICENSE.md`) stay with the owning walker — only the extensionless or
//! plain-text variants land here. **Credential-bearing dotfiles
//! (`.npmrc`, `.netrc`, `.env`, `.pypirc`) are NOT in the whitelist** —
//! a project-local `.npmrc` commonly carries `_authToken` or registry
//! passwords, and `precis` output is intended for downstream agents /
//! logs.
//!
//! Budget protection: `PLAINTEXT_LINE_CAP` skips any file whose source
//! line count exceeds the cap. Plaintext files this walker owns are
//! intentionally short — anything bigger should either be a `Read` call
//! by the agent or land in a format-aware walker.

use std::path::Path;

use crate::batch::{Batch, BatchKey, PlaintextKey};
use crate::value::mix_signals;

use super::{FileLines, WalkCtx, fs::list_dir, path_depth_factor, single_file_lines_content};

/// Hard cap on the number of source lines a plaintext file may have to
/// be considered for a `Whole` batch. Larger files are skipped wholesale.
const PLAINTEXT_LINE_CAP: usize = 60;

/// FS-metadata pre-flight gate: skip files whose raw byte size is
/// obviously past the cap before opening them. 80 bytes/line bounds
/// the worst-case rendered cost (line-cap × ~80 chars × token-overhead)
/// to roughly 1500 tokens — comfortably below any single budget chunk
/// the scheduler would let plaintext claim. Files denser than this
/// are agent-`Read` territory, not precis output.
const PLAINTEXT_BYTE_GATE: usize = PLAINTEXT_LINE_CAP * 80;

/// What kind of plaintext file this is. Drives the signal preset and
/// keeps the (filename → preset) mapping in one table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Class {
    /// LICENSE / LICENSE-MIT / LICENSE-APACHE / COPYING / NOTICE etc.
    /// Conveys legal status; rarely affects how to use the project.
    License,
    /// .gitignore / .dockerignore. Hints at generated artifacts and
    /// involved tooling.
    IgnoreList,
    /// .editorconfig / .eslintrc / .prettierrc (extensionless).
    /// Formatting + lint conventions; affects code edits.
    EditorConfig,
    /// .nvmrc / .python-version / .tool-versions / pnpm-workspace.yaml.
    /// Toolchain or workspace-topology pinning — orientation files for
    /// "what does this project assume about its environment / shape".
    /// `.npmrc` is intentionally absent (auth-token risk — see module
    /// doc).
    Toolchain,
    /// VERSION file: a one-line version stamp (`0.1.7-alpha.10`). NS
    /// authors anchor on this when no `pyproject.toml` / `package.json`
    /// / `Cargo.toml` carries the canonical version (sqlite-vec, act).
    Version,
    /// TODO file: an open backlog in plain text. NS authors anchor on
    /// the header item as a "what's pending" orientation signal
    /// (sqlite-vec).
    Todo,
}

/// Classify a file by its name. Returns `None` for any file the walker
/// does not own — including format-aware siblings (`.eslintrc.json`,
/// `LICENSE.md`) that other walkers handle, and out-of-scope variants
/// (`LICENSE-HEADER`, `Makefile`) that we deliberately don't claim.
///
/// Credential-bearing names (`.npmrc`, `.netrc`, `.env`, `.pypirc`)
/// are NOT classified — see the module doc for the safety rationale.
pub(crate) fn classify_plaintext(name: &str) -> Option<Class> {
    // License names are matched case-insensitively (some repos use
    // lowercase `license`); dotfile names case-sensitively (Unix
    // convention). Listing names exhaustively is on purpose: an
    // open-ended `LICENSE-*` predicate would also pick up things like
    // `LICENSE-HEADER` (tomli) that aren't actually a license header.
    let lower = name.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "license"
            | "license-mit"
            | "license-apache"
            | "license.txt"
            | "copying"
            | "copying.txt"
            | "notice"
            | "notice.txt"
    ) {
        return Some(Class::License);
    }
    match name {
        ".gitignore" | ".dockerignore" => return Some(Class::IgnoreList),
        ".editorconfig" | ".eslintrc" | ".prettierrc" => return Some(Class::EditorConfig),
        ".nvmrc" | ".python-version" | ".tool-versions" | "pnpm-workspace.yaml" => {
            return Some(Class::Toolchain);
        }
        _ => {}
    }
    // Extensionless orientation files matched case-insensitively. Kept
    // separate from the `LICENSE` block so we don't accidentally match
    // `version.h` or similar — `lower` is only used here. `version.txt`
    // is the de-facto Python-project variant when a project ships its
    // canonical version stamp as a sibling of `pyproject.toml` rather
    // than baking it into the `[project].version` scalar (linkding).
    if lower == "version" || lower == "version.txt" {
        return Some(Class::Version);
    }
    if lower == "todo" {
        return Some(Class::Todo);
    }
    None
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let entries = list_dir(dir);
    let mut out = Vec::new();
    for (name, kind) in entries {
        if !matches!(kind, crate::fs_util::EntryKind::File) {
            continue;
        }
        let Some(class) = classify_plaintext(&name) else {
            continue;
        };
        let file = dir.join(&name);
        let byte_len = std::fs::metadata(&file)
            .map(|m| m.len() as usize)
            .unwrap_or(usize::MAX);
        if byte_len > PLAINTEXT_BYTE_GATE {
            continue;
        }
        let Some(source) = ctx.read_source(&file) else {
            continue;
        };
        let line_count = source.lines().count();
        if line_count == 0 || line_count > PLAINTEXT_LINE_CAP {
            continue;
        }
        let lines: Vec<usize> = (1..=line_count).collect();
        let Some(content) = single_file_lines_content(&file, &source, FileLines::new(lines)) else {
            continue;
        };
        out.push(Batch {
            key: PlaintextKey::Whole { file: file.clone() }.into(),
            predecessor: None,
            content,
            value: class_value(class, &file, ctx),
        });
    }
    out
}

fn class_value(class: Class, file: &Path, ctx: &WalkCtx) -> f64 {
    // Tuned against frozen-NS divergence baselines: License gets the
    // floor because pure boilerplate rarely shifts how an agent uses
    // the code, and at higher weights it displaced one tier-tail
    // batch in anyhow/superstruct. mitt's NS 5.10 (.editorconfig +
    // .gitignore) is the load-bearing fixture target.
    let (cat, fu, ztu) = match class {
        Class::License => (0.05, 0.10, 0.10),
        Class::IgnoreList => (0.20, 0.30, 0.25),
        Class::EditorConfig => (0.25, 0.35, 0.30),
        Class::Toolchain => (0.30, 0.35, 0.30),
        // Version stamp: a single short line answers "what version is
        // this?" — high orientation value relative to the trivial cost.
        Class::Version => (0.55, 0.40, 0.45),
        // TODO backlog: short header items are tier-1 orientation for
        // "what's pending / known limitations"; rest is appendix.
        Class::Todo => (0.40, 0.50, 0.40),
    };
    mix_signals(cat, fu, ztu, path_depth_factor(file, ctx))
}

#[cfg(test)]
mod tests {
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    use super::*;

    #[test]
    fn plaintext_classify_table_drives_predicate() {
        // (name, expected). `None` rows assert names the walker must
        // refuse — owned by other walkers, out of scope, or
        // credential-bearing.
        let cases: &[(&str, Option<Class>)] = &[
            // License names — case-insensitive.
            ("LICENSE", Some(Class::License)),
            ("license", Some(Class::License)),
            ("LICENSE-MIT", Some(Class::License)),
            ("LICENSE-APACHE", Some(Class::License)),
            ("LICENSE.txt", Some(Class::License)),
            ("COPYING", Some(Class::License)),
            ("NOTICE", Some(Class::License)),
            // Dotfiles by class.
            (".gitignore", Some(Class::IgnoreList)),
            (".dockerignore", Some(Class::IgnoreList)),
            (".editorconfig", Some(Class::EditorConfig)),
            (".eslintrc", Some(Class::EditorConfig)),
            (".prettierrc", Some(Class::EditorConfig)),
            (".nvmrc", Some(Class::Toolchain)),
            (".python-version", Some(Class::Toolchain)),
            (".tool-versions", Some(Class::Toolchain)),
            ("pnpm-workspace.yaml", Some(Class::Toolchain)),
            // Extensionless orientation files (case-insensitive on the
            // stem). `VERSION` is a one-line version stamp common in
            // C-shaped projects; `TODO` is a plain backlog file. The
            // `version.txt` variant is Python convention.
            ("VERSION", Some(Class::Version)),
            ("version", Some(Class::Version)),
            ("version.txt", Some(Class::Version)),
            ("VERSION.txt", Some(Class::Version)),
            ("TODO", Some(Class::Todo)),
            // Owned by other walkers.
            ("LICENSE.md", None),
            (".eslintrc.json", None),
            (".prettierrc.json", None),
            (".eslintrc.js", None),
            (".prettierrc.js", None),
            // Credential-bearing — must not be classified (see module
            // doc + Codex adversarial review).
            (".npmrc", None),
            (".netrc", None),
            (".env", None),
            (".pypirc", None),
            // Out of scope by design.
            ("LICENSE-HEADER", None),
            ("Makefile", None),
            ("Dockerfile", None),
            ("README", None),
            ("notes.txt", None),
        ];
        for (name, expected) in cases {
            assert_eq!(
                classify_plaintext(name),
                *expected,
                "classify_plaintext({name:?})",
            );
        }
    }

    /// Drive the full `FsWalker` + scheduler against a real
    /// directory (per Codex round-1 P2: in-memory `SourceCache`
    /// preload bypasses `read_dir` and never exercises the discovery
    /// path). Asserts the plaintext content lands in the rendered
    /// output and that the scheduler logs a `Plaintext::Whole` batch.
    #[test]
    fn plaintext_real_dir_renders_seeded_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("LICENSE"),
            "MIT License\n\nCopyright (c) Yoav\n\nSee LICENSE.\n",
        )
        .unwrap();
        std::fs::write(root.join(".gitignore"), "target/\n*.tmp\n").unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            rendered.contains("MIT License"),
            "rendered output is missing the LICENSE body:\n{rendered}",
        );
        assert!(
            rendered.contains("target/"),
            "rendered output is missing the .gitignore body:\n{rendered}",
        );
        assert_has_plaintext_whole(&report, "LICENSE");
        assert_has_plaintext_whole(&report, ".gitignore");
    }

    /// A LICENSE that exceeds `PLAINTEXT_BYTE_GATE` is dropped at
    /// `expand` time via the FS-metadata gate.
    #[test]
    fn plaintext_oversized_license_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        // Each line is ~250 bytes; 60 lines pushes byte count past
        // PLAINTEXT_BYTE_GATE without needing an actually huge file.
        let long_line = "x".repeat(250);
        let body: String = std::iter::repeat_n(long_line.as_str(), 60)
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(root.join("LICENSE"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        assert_no_plaintext_whole(&report, "LICENSE");
    }

    /// A LICENSE whose byte count slips under the gate but whose line
    /// count exceeds `PLAINTEXT_LINE_CAP` is dropped at materialize
    /// time.
    #[test]
    fn plaintext_too_many_lines_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let body: String = (0..(PLAINTEXT_LINE_CAP + 5))
            .map(|i| format!("line {i}\n"))
            .collect();
        std::fs::write(root.join("LICENSE"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        assert_no_plaintext_whole(&report, "LICENSE");
    }

    fn assert_has_plaintext_whole(report: &crate::scheduler::RunReport<BatchKey>, suffix: &str) {
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter().any(|k| matches!(
                k,
                BatchKey::Plaintext(PlaintextKey::Whole { file }) if file.ends_with(suffix),
            )),
            "missing Plaintext::Whole batch ending with {suffix:?}; scheduled keys: {keys:?}",
        );
    }

    fn assert_no_plaintext_whole(report: &crate::scheduler::RunReport<BatchKey>, suffix: &str) {
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            !keys.iter().any(|k| matches!(
                k,
                BatchKey::Plaintext(PlaintextKey::Whole { file }) if file.ends_with(suffix),
            )),
            "unexpected Plaintext::Whole batch ending with {suffix:?}; scheduled keys: {keys:?}",
        );
    }
}
