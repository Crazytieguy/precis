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

use super::{
    FileLines, WalkCtx, dedup_sorted, fs::list_dir, gated_whole_file_content, path_depth_factor,
    single_file_lines_content,
};

/// Line cap on a `Whole` plaintext batch.
const PLAINTEXT_LINE_CAP: usize = 60;

/// FS-metadata pre-flight gate (≈80 bytes/line × line cap).
const PLAINTEXT_BYTE_GATE: usize = PLAINTEXT_LINE_CAP * 80;

/// Plaintext file class — drives the (filename → signal preset) table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Class {
    /// LICENSE / LICENSE-MIT / COPYING / NOTICE etc.
    License,
    /// .gitignore / .dockerignore.
    IgnoreList,
    /// .editorconfig / .eslintrc / .prettierrc (extensionless).
    EditorConfig,
    /// .nvmrc / .python-version / .tool-versions / pnpm-workspace.yaml.
    Toolchain,
    /// Legacy Python packaging metadata (`setup.cfg`).
    PackageConfig,
    /// Python requirements freeze/list files, sampled when long.
    Requirements,
    /// One-line version stamp.
    Version,
    /// Plain-text backlog.
    Todo,
}

/// Classify a file by name. `None` for files the walker doesn't own
/// (other walkers' formats, out-of-scope variants, credential names).
pub(crate) fn classify_plaintext(name: &str) -> Option<Class> {
    // Licenses match case-insensitively; dotfiles case-sensitively.
    // The list is exhaustive on purpose — `LICENSE-*` would catch
    // `LICENSE-HEADER` etc.
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
        "setup.cfg" => return Some(Class::PackageConfig),
        "requirements.txt" => return Some(Class::Requirements),
        _ => {}
    }
    // Orientation stamps matched case-insensitively by exact name —
    // exact equality (no stem matching) is what keeps `version.h` and
    // similar source headers out. `version.txt` is the de-facto
    // Python-project variant when a project ships its canonical version
    // stamp as a sibling of `pyproject.toml` rather than baking it into
    // the `[project].version` scalar (linkding).
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
        let file = dir.join(&name);
        if is_man_page_name(&name) {
            if let Some(batch) = man_page_batch(&file, ctx) {
                out.push(batch);
            }
            continue;
        }
        let Some(class) = classify_plaintext(&name) else {
            continue;
        };
        let content = match class {
            Class::Requirements => requirements_content(&file, ctx),
            _ => gated_whole_file_content(&file, ctx, PLAINTEXT_BYTE_GATE, PLAINTEXT_LINE_CAP),
        };
        let Some(content) = content else {
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
        Class::PackageConfig => (0.45, 0.55, 0.45),
        Class::Requirements => (0.35, 0.45, 0.35),
        // Version stamp: a single short line answers "what version is
        // this?" — high orientation value relative to the trivial cost.
        Class::Version => (0.55, 0.40, 0.45),
        // TODO backlog: short header items are tier-1 orientation for
        // "what's pending / known limitations"; rest is appendix.
        Class::Todo => (0.40, 0.50, 0.40),
    };
    mix_signals(cat, fu, ztu, path_depth_factor(file, ctx))
}

fn requirements_content(file: &Path, ctx: &WalkCtx) -> Option<crate::content::BatchContent> {
    let source = ctx.read_source(file)?;
    let lines: Vec<&str> = source.lines().collect();
    if lines.is_empty() {
        return None;
    }
    if lines.len() <= 8 {
        return single_file_lines_content(
            file,
            &source,
            FileLines::new((1..=lines.len()).collect()),
        );
    }
    let mut full = vec![1];
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("-e ") || trimmed.starts_with("git+") || trimmed.contains("://") {
            full.push(idx + 1);
        }
    }
    full.sort_unstable();
    full.dedup();
    let mut ellipses = Vec::new();
    for pair in full.windows(2) {
        if pair[1] > pair[0] + 1 {
            ellipses.push(pair[0] + 1);
        }
    }
    if full.last().is_some_and(|last| *last < lines.len()) {
        ellipses.push(full.last().copied().unwrap_or(1) + 1);
    }
    single_file_lines_content(
        file,
        &source,
        FileLines::new(full).with_ellipses(dedup_sorted(ellipses)),
    )
}

// --- man pages ----------------------------------------------------------

/// Lines scanned for the man-page `NAME` / `DESCRIPTION` sections — both
/// live near the top of any troff page, so a bounded scan suffices.
const MAN_SCAN_LINES: usize = 150;
/// Content lines taken from the `DESCRIPTION` section (the lede), beyond
/// the heading itself.
const MAN_DESC_LEDE_LINES: usize = 5;

/// True for a troff man-page filename: `<base>.<1-9>` optionally with an
/// autotools `.in` suffix (`htop.1`, `foo.5`, `htop.1.in`).
fn is_man_page_name(name: &str) -> bool {
    let stem = name.strip_suffix(".in").unwrap_or(name);
    match stem.rsplit_once('.') {
        Some((base, section)) => {
            !base.is_empty()
                && section.len() == 1
                && section.chars().all(|c| ('1'..='9').contains(&c))
        }
        None => false,
    }
}

fn man_page_batch(file: &Path, ctx: &WalkCtx) -> Option<Batch<BatchKey>> {
    let source = ctx.read_source(file)?;
    let lines = man_lede_lines(&source);
    if lines.is_empty() {
        return None;
    }
    let content = single_file_lines_content(file, &source, FileLines::new(lines))?;
    Some(Batch {
        key: PlaintextKey::ManLede {
            file: file.to_path_buf(),
        }
        .into(),
        predecessor: None,
        content,
        // NAME + DESCRIPTION lede is the canonical "what is this tool"
        // answer — high catastrophic-omission and zero-tool-call value,
        // like a README headline, for any CLI shipping a man page.
        value: mix_signals(0.65, 0.45, 0.7, path_depth_factor(file, ctx)),
    })
}

/// 1-based line numbers of the `NAME` section (heading + body to the next
/// `.SH`) and the `DESCRIPTION` lede (heading + first
/// [`MAN_DESC_LEDE_LINES`] content rows). Empty if no `NAME` section.
fn man_lede_lines(source: &str) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().take(MAN_SCAN_LINES).collect();
    let mut out = Vec::new();
    let mut saw_name = false;
    let mut i = 0;
    while i < lines.len() {
        if is_man_section_heading(lines[i], "NAME") {
            saw_name = true;
            out.push(i + 1);
            let mut j = i + 1;
            while j < lines.len() && !is_sh_directive(lines[j]) {
                if !lines[j].trim().is_empty() {
                    out.push(j + 1);
                }
                j += 1;
            }
        } else if is_man_section_heading(lines[i], "DESCRIPTION") {
            out.push(i + 1);
            let mut j = i + 1;
            let mut taken = 0;
            while j < lines.len() && taken < MAN_DESC_LEDE_LINES && !is_sh_directive(lines[j]) {
                out.push(j + 1);
                taken += 1;
                j += 1;
            }
        }
        i += 1;
    }
    out.sort_unstable();
    out.dedup();
    if saw_name { out } else { Vec::new() }
}

fn is_sh_directive(line: &str) -> bool {
    line.trim_start().starts_with(".SH")
}

/// True iff `line` is a `.SH NAME` / `.SH "DESCRIPTION"` section heading
/// for `name` (quoting and case tolerated).
fn is_man_section_heading(line: &str, name: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix(".SH") else {
        return false;
    };
    rest.trim()
        .trim_matches('"')
        .trim()
        .eq_ignore_ascii_case(name)
}

#[cfg(test)]
mod tests {
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    use super::*;

    #[test]
    fn plaintext_man_page_name_and_lede() {
        assert!(is_man_page_name("htop.1"));
        assert!(is_man_page_name("htop.1.in"));
        assert!(is_man_page_name("foo.5"));
        assert!(!is_man_page_name("foo.h"));
        assert!(!is_man_page_name("foo.cpp"));
        assert!(!is_man_page_name("README.md"));
        assert!(!is_man_page_name("1"));

        let src = ".TH FOO 1\n.SH \"NAME\"\nfoo \\- does things\n.SH \"SYNOPSIS\"\n\
                   .B foo\n.SH \"DESCRIPTION\"\n.B foo\nis a thing.\n.LP\nMore.\n";
        let lines = man_lede_lines(src);
        // NAME heading (2) + body (3); DESCRIPTION heading (6) + lede (7..).
        assert!(lines.contains(&2) && lines.contains(&3), "NAME: {lines:?}");
        assert!(lines.contains(&6) && lines.contains(&7), "DESC: {lines:?}");
        // A page with no NAME section yields nothing.
        assert!(man_lede_lines(".TH FOO 1\n.SH SYNOPSIS\n.B foo\n").is_empty());
    }

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
            ("setup.cfg", Some(Class::PackageConfig)),
            ("requirements.txt", Some(Class::Requirements)),
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
