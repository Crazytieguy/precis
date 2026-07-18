//! Plaintext walker. Emits `Whole` content batches for small known
//! plaintext files that none of the format-aware walkers (Rust,
//! Markdown, TOML, JSON, TypeScript, YAML, etc.) cover: license and
//! ignore files, compact toolchain/build/package manifests, selected
//! build scripts, requirement lists, version/TODO stamps, and man-page
//! ledes. Without this walker, these files only appear in directory
//! listings and their content is unreachable from the scheduler.
//!
//! The whitelist remains credential-aware (see [`classify_plaintext`]).
//! Files with format-specific siblings (`.eslintrc.json`,
//! `.prettierrc.js`, `LICENSE.md`) stay with the owning walker, and
//! credential-bearing dotfiles (`.npmrc`, `.netrc`, `.env`, `.pypirc`)
//! are NOT in the whitelist. Checked-in dotenv *samples*
//! (`.env.sample` / `.env.example`) ARE admitted — they carry
//! placeholder values by convention and are the deploy-facing
//! config-key documentation. Shell scripts are only admitted from
//! build-script locations, and exact env/secret/credential stems
//! (`env`, `.env`, `secret`, `secrets`, `credential`, `credentials`,
//! `creds`) are denied before `.sh` classification because they
//! commonly export tokens for local tooling.
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
    single_file_lines_content, whole_file_lines_content,
};

/// Line cap on a `Whole` plaintext batch.
const PLAINTEXT_LINE_CAP: usize = 60;

/// FS-metadata pre-flight gate (≈80 bytes/line × line cap).
const PLAINTEXT_BYTE_GATE: usize = PLAINTEXT_LINE_CAP * 80;

/// Head rows retained from long `requirements.txt` files.
const REQUIREMENTS_HEAD_LINE_CAP: usize = 8;

/// Line cap on `Makefile` / `Dockerfile` whole batches.
const BUILD_ENTRYPOINT_LINE_CAP: usize = 100;

/// Tail-batch value factor for the Dockerfile split — the `RUN` /
/// `COPY` bodies are follow-up to the stage + contract skeleton, but
/// close follow-up: NSes that rank a Dockerfile at all want the build
/// commands within the same budget window as the skeleton.
const DOCKERFILE_TAIL_FACTOR: f64 = 0.85;

/// Minimum line count before a Dockerfile is split. Short Dockerfiles
/// are already their own skeleton and NSes rank them whole; only past
/// this size does the whole-file lump price the stage/contract
/// skeleton out of the early budget.
const DOCKERFILE_SPLIT_MIN_LINES: usize = 40;

/// Rows in the dotenv head batch — samples lead with the
/// mandatory-settings block by convention (linkwarden's first 11 rows
/// are NextAuth + database; linkding's are container/host/superuser).
const DOTENV_MANDATORY_HEAD_LINES: usize = 12;

/// Total rows retained from long dotenv samples (head + tail batch) —
/// enough to carry the leading config-key roster.
const DOTENV_HEAD_LINE_CAP: usize = 60;

/// Tail-batch value factor relative to the head — the optional-settings
/// roster is a follow-up, not the anchor.
const DOTENV_TAIL_FACTOR: f64 = 0.6;

/// Pre-flight byte gate for dotenv samples — generous (they're
/// head-sampled, not rendered whole) but bounded.
const DOTENV_BYTE_GATE: usize = 64 * 1024;

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
    /// Compact build/deploy entrypoints.
    BuildEntrypoint,
    /// Container build file (`Dockerfile` / `Containerfile`), split
    /// into a stage/contract head + gated body tail.
    Dockerfile,
    /// Compact build/test plumbing scripts and manifests.
    BuildScript,
    /// Legacy Python packaging metadata (`setup.cfg`).
    PackageConfig,
    /// Python requirements freeze/list files, sampled when long.
    Requirements,
    /// Checked-in dotenv sample/template (`.env.sample`) — the
    /// deploy-facing config-key documentation, head-sampled when long.
    DotenvSample,
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
        "Makefile" => return Some(Class::BuildEntrypoint),
        "Dockerfile" | "Containerfile" => return Some(Class::Dockerfile),
        ".gitmodules" | "configure.ac" => return Some(Class::BuildScript),
        "setup.cfg" => return Some(Class::PackageConfig),
        "requirements.txt" => return Some(Class::Requirements),
        _ => {}
    }
    if crate::value::is_dotenv_sample_filename(name) {
        return Some(Class::DotenvSample);
    }
    if let Some(stem) = lower.strip_suffix(".sh") {
        if matches!(
            stem,
            "env" | ".env" | "secret" | "secrets" | "credential" | "credentials" | "creds"
        ) {
            return None;
        }
        return Some(Class::BuildScript);
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
        if matches!(class, Class::BuildEntrypoint | Class::Dockerfile) && dir != ctx.root() {
            continue;
        }
        if matches!(class, Class::BuildScript) && !is_build_script_location(&file, dir, ctx) {
            continue;
        }
        if matches!(class, Class::DotenvSample) {
            push_dotenv_batches(&file, class, ctx, &mut out);
            continue;
        }
        if matches!(class, Class::Dockerfile) {
            push_dockerfile_batches(&file, class, ctx, &mut out);
            continue;
        }
        let content = match class {
            Class::Requirements => {
                head_sampled_content(&file, ctx, PLAINTEXT_BYTE_GATE, REQUIREMENTS_HEAD_LINE_CAP)
            }
            // Build entrypoints get headroom over the generic cap:
            // real app Dockerfiles / Makefiles routinely run 60–100
            // lines and are exactly the ops surface NS authors anchor
            // on (audiobookshelf 73, linkwarden 70).
            Class::BuildEntrypoint => gated_whole_file_content(
                &file,
                ctx,
                BUILD_ENTRYPOINT_LINE_CAP * 80,
                BUILD_ENTRYPOINT_LINE_CAP,
            ),
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
        Class::BuildEntrypoint | Class::Dockerfile => (0.70, 0.55, 0.60),
        Class::BuildScript => (0.60, 0.50, 0.55),
        Class::PackageConfig => (0.45, 0.55, 0.45),
        Class::Requirements => (0.35, 0.45, 0.35),
        // Dotenv sample: the deploy-facing config-key roster of a
        // self-hosted app — NS authors rank it alongside Dockerfile /
        // compose as tier-1 ops orientation (linkwarden, linkding).
        Class::DotenvSample => (0.60, 0.50, 0.55),
        // Version stamp: a single short line answers "what version is
        // this?" — high orientation value relative to the trivial cost.
        Class::Version => (0.55, 0.40, 0.45),
        // TODO backlog: short header items are tier-1 orientation for
        // "what's pending / known limitations"; rest is appendix.
        Class::Todo => (0.40, 0.50, 0.40),
    };
    mix_signals(cat, fu, ztu, path_depth_factor(file, ctx))
}

fn is_build_script_location(file: &Path, dir: &Path, ctx: &WalkCtx) -> bool {
    let Some(name) = file.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    if matches!(name, ".gitmodules" | "configure.ac") {
        return dir == ctx.root();
    }
    name.ends_with(".sh")
        && (dir == ctx.root()
            || dir
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == "scripts"))
}

/// Dotenv samples ship as a cheap mandatory-settings head plus a
/// gated tail batch — a single head-sampled lump prices the mandatory
/// block out of the early budget NS authors rank it in.
fn push_dotenv_batches(file: &Path, class: Class, ctx: &WalkCtx, out: &mut Vec<Batch<BatchKey>>) {
    let byte_len = std::fs::metadata(file)
        .map(|m| m.len() as usize)
        .unwrap_or(usize::MAX);
    if byte_len > DOTENV_BYTE_GATE {
        return;
    }
    let Some(source) = ctx.read_source(file) else {
        return;
    };
    let line_count = source.lines().count();
    if line_count == 0 {
        return;
    }
    let head_end = DOTENV_MANDATORY_HEAD_LINES.min(line_count);
    let mut head = FileLines::new((1..=head_end).collect());
    if head_end < line_count {
        head = head.with_ellipses(vec![head_end + 1]);
    }
    let head_key = PlaintextKey::Whole {
        file: file.to_path_buf(),
    };
    let head_value = class_value(class, file, ctx);
    if let Some(content) = single_file_lines_content(file, &source, head) {
        out.push(Batch {
            key: head_key.clone().into(),
            predecessor: None,
            content,
            value: head_value,
        });
    }
    let tail_end = DOTENV_HEAD_LINE_CAP.min(line_count);
    if tail_end <= head_end {
        return;
    }
    let mut tail = FileLines::new((head_end + 1..=tail_end).collect());
    if tail_end < line_count {
        tail = tail.with_ellipses(vec![tail_end + 1]);
    }
    if let Some(content) = single_file_lines_content(file, &source, tail) {
        out.push(Batch {
            key: PlaintextKey::Tail {
                file: file.to_path_buf(),
            }
            .into(),
            predecessor: Some(BatchKey::Plaintext(head_key)),
            content,
            value: head_value * DOTENV_TAIL_FACTOR,
        });
    }
}

/// Dockerfile instructions that form the container's operational
/// contract: stage boundaries plus what the finished image exposes
/// and runs. Per-stage build environment (`ARG`/`ENV`/`WORKDIR`) and
/// build mechanics (`RUN`/`COPY`/`ADD`, labels) are body, not
/// skeleton — NS authors consistently elide them from the early rank
/// and pick them up with the stage bodies.
fn is_dockerfile_contract_instruction(instruction: &str) -> bool {
    matches!(
        instruction,
        "FROM" | "EXPOSE" | "VOLUME" | "USER" | "ENTRYPOINT" | "CMD" | "HEALTHCHECK" | "STOPSIGNAL"
    )
}

/// 1-based lines of the Dockerfile contract skeleton: every contract
/// instruction with its backslash-continuation lines, plus the
/// contiguous comment block directly above each `FROM` (stage-label
/// comments by convention).
fn dockerfile_contract_lines(source: &str) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().collect();
    let mut selected = Vec::new();
    let mut in_continuation = false;
    let mut current_selected = false;
    for (i, raw) in lines.iter().enumerate() {
        let line = raw.trim();
        if in_continuation {
            if current_selected {
                selected.push(i + 1);
            }
            // Comment lines inside a continuation don't end it.
            if !line.starts_with('#') {
                in_continuation = line.ends_with('\\');
            }
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let instruction = line
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_uppercase();
        in_continuation = line.ends_with('\\');
        current_selected = is_dockerfile_contract_instruction(&instruction);
        if !current_selected {
            continue;
        }
        if instruction == "FROM" {
            // Attach the stage-label comment block.
            let mut j = i;
            while j > 0 && lines[j - 1].trim_start().starts_with('#') {
                j -= 1;
                selected.push(j + 1);
            }
        }
        selected.push(i + 1);
    }
    selected.sort_unstable();
    selected.dedup();
    selected
}

/// Ellipsis markers for each maximal gap in `selected` within
/// `1..=line_count` (first line of every gap).
fn gap_ellipses(selected: &[usize], line_count: usize) -> Vec<usize> {
    let mut ellipses = Vec::new();
    let mut prev = 0usize;
    for &line in selected.iter().chain(std::iter::once(&(line_count + 1))) {
        if line > prev + 1 {
            ellipses.push(prev + 1);
        }
        prev = line;
    }
    ellipses
}

/// Dockerfiles ship as a compact stage/contract head plus a gated
/// body tail — NS authors rank the stage markers + operational
/// contract inside the early budget and the `RUN`/`COPY` build
/// mechanics as follow-up; a whole-file lump prices the contract out
/// (same shape as the dotenv-sample split).
fn push_dockerfile_batches(
    file: &Path,
    class: Class,
    ctx: &WalkCtx,
    out: &mut Vec<Batch<BatchKey>>,
) {
    let byte_len = std::fs::metadata(file)
        .map(|m| m.len() as usize)
        .unwrap_or(usize::MAX);
    if byte_len > BUILD_ENTRYPOINT_LINE_CAP * 80 {
        return;
    }
    let Some(source) = ctx.read_source(file) else {
        return;
    };
    let line_count = source.lines().count();
    if line_count == 0 || line_count > BUILD_ENTRYPOINT_LINE_CAP {
        return;
    }
    let head_lines = dockerfile_contract_lines(&source);
    let tail_lines: Vec<usize> = (1..=line_count)
        .filter(|line| !head_lines.contains(line))
        .collect();
    let value = class_value(class, file, ctx);
    let head_key = PlaintextKey::Whole {
        file: file.to_path_buf(),
    };
    if line_count <= DOCKERFILE_SPLIT_MIN_LINES || head_lines.is_empty() || tail_lines.is_empty() {
        // Short, not Dockerfile-shaped, or all-contract: ship whole.
        if let Some(content) = whole_file_lines_content(file, &source) {
            out.push(Batch {
                key: head_key.into(),
                predecessor: None,
                content,
                value,
            });
        }
        return;
    }
    let ellipses = gap_ellipses(&head_lines, line_count);
    let head = FileLines::new(head_lines).with_ellipses(ellipses);
    if let Some(content) = single_file_lines_content(file, &source, head) {
        out.push(Batch {
            key: head_key.clone().into(),
            predecessor: None,
            content,
            value,
        });
    }
    let ellipses = gap_ellipses(&tail_lines, line_count);
    let tail = FileLines::new(tail_lines).with_ellipses(ellipses);
    if let Some(content) = single_file_lines_content(file, &source, tail) {
        out.push(Batch {
            key: PlaintextKey::Tail {
                file: file.to_path_buf(),
            }
            .into(),
            predecessor: Some(BatchKey::Plaintext(head_key)),
            content,
            value: value * DOCKERFILE_TAIL_FACTOR,
        });
    }
}

/// Whole file when it fits `head_line_cap`, else the head rows with a
/// trailing ellipsis. Shared by the requirements and dotenv-sample
/// classes, whose long-file tails are low-value but whose heads carry
/// the roster the file exists for.
fn head_sampled_content(
    file: &Path,
    ctx: &WalkCtx,
    byte_gate: usize,
    head_line_cap: usize,
) -> Option<crate::content::BatchContent> {
    let byte_len = std::fs::metadata(file)
        .map(|m| m.len() as usize)
        .unwrap_or(usize::MAX);
    if byte_len > byte_gate {
        return None;
    }
    let source = ctx.read_source(file)?;
    let line_count = source.lines().count();
    if line_count == 0 {
        return None;
    }
    if line_count <= head_line_cap {
        return single_file_lines_content(
            file,
            &source,
            FileLines::new((1..=line_count).collect()),
        );
    }
    single_file_lines_content(file, &source, head_lines(line_count, head_line_cap))
}

fn head_lines(line_count: usize, head_line_cap: usize) -> FileLines {
    let full: Vec<usize> = (1..=head_line_cap.min(line_count)).collect();
    let mut ellipses = Vec::new();
    let mut boundaries = full.clone();
    boundaries.push(line_count + 1);
    for pair in boundaries.windows(2) {
        if pair[1] > pair[0] + 1 {
            ellipses.push(pair[0] + 1);
        }
    }
    FileLines::new(full).with_ellipses(dedup_sorted(ellipses))
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
            ("Makefile", Some(Class::BuildEntrypoint)),
            ("Dockerfile", Some(Class::Dockerfile)),
            ("Containerfile", Some(Class::Dockerfile)),
            ("testall.sh", Some(Class::BuildScript)),
            ("build.sh", Some(Class::BuildScript)),
            (".gitmodules", Some(Class::BuildScript)),
            ("configure.ac", Some(Class::BuildScript)),
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
            ("env.sh", None),
            (".env.sh", None),
            ("secrets.sh", None),
            ("secret.sh", None),
            ("credentials.sh", None),
            ("creds.sh", None),
            // Out of scope by design.
            ("LICENSE-HEADER", None),
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

    #[test]
    fn plaintext_dockerfile_contract_lines_select_stages_and_contract() {
        let src = "\
# Stage: builder\n\
# uses rust\n\
FROM rust:1.86 AS builder\n\
\n\
RUN cargo build \\\n\
    # comment inside continuation\n\
    --release\n\
\n\
FROM node:20 AS app\n\
ENV FOO=1 \\\n\
    BAR=2\n\
COPY . .\n\
HEALTHCHECK --interval=30s \\\n\
    CMD [\"curl\", \"http://localhost/\"]\n\
EXPOSE 3000\n\
CMD [\"node\", \"index.js\"]\n";
        let selected = dockerfile_contract_lines(src);
        // Stage comments + FROMs, HEALTHCHECK with continuation,
        // EXPOSE, CMD — but not ENV (build environment) and not RUN
        // (with its interior comment + continuation) or COPY.
        assert_eq!(selected, vec![1, 2, 3, 9, 13, 14, 15, 16]);
        assert_eq!(gap_ellipses(&selected, 16), vec![4, 10]);
        // Leading gap gets a marker too.
        assert_eq!(gap_ellipses(&[3, 4], 6), vec![1, 5]);
    }

    #[test]
    fn plaintext_requirements_samples_head_not_url_lines() {
        let lines = head_lines(12, REQUIREMENTS_HEAD_LINE_CAP);
        assert_eq!(lines.full, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(lines.ellipses, vec![9]);
    }

    #[test]
    fn plaintext_oversized_requirements_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let long_line = "x".repeat(250);
        let body: String = std::iter::repeat_n(long_line.as_str(), 60)
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(root.join("requirements.txt"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        assert_no_plaintext_whole(&report, "requirements.txt");
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
