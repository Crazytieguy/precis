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

use std::collections::VecDeque;
use std::ops::Range;
use std::path::Path;

use crate::batch::{Batch, BatchKey, PlaintextKey};
use crate::value::{
    DEFAULT_CONCAVITY_EXPONENT, conserved_catalog_chunk_factors, mix_signals, roster_mass_factor,
};

use super::{
    FileLines, WalkCtx, budget_chunk_ranges, dedup_sorted, fs::list_dir, gap_ellipses,
    gated_read_source, gated_whole_file_content, path_depth_factor, single_file_lines_content,
    whole_file_lines_content,
};

/// Line cap on a `Whole` plaintext batch.
const PLAINTEXT_LINE_CAP: usize = 60;

/// FS-metadata pre-flight gate (≈80 bytes/line × line cap).
const PLAINTEXT_BYTE_GATE: usize = PLAINTEXT_LINE_CAP * 80;

/// Head rows retained from long `requirements.txt` files.
const REQUIREMENTS_HEAD_LINE_CAP: usize = 8;

/// Line cap on `Makefile` / `Dockerfile` whole batches.
const BUILD_ENTRYPOINT_LINE_CAP: usize = 100;

/// FS-metadata pre-flight gate for build entrypoints (same ≈80
/// bytes/line multiplier as [`PLAINTEXT_BYTE_GATE`]).
const BUILD_ENTRYPOINT_BYTE_GATE: usize = BUILD_ENTRYPOINT_LINE_CAP * 80;

/// Oversized Makefiles are parsed into a bounded structural skeleton,
/// so they can tolerate realistic recipe-heavy files without admitting
/// arbitrarily large plaintext inputs.
const MAKEFILE_SKELETON_BYTE_GATE: usize = 256 * 1024;

/// Target cost for source-ordered Makefile skeleton chunks. Skeleton
/// rows are individually cheap, but a large target catalog should not
/// become another whole-file-sized scheduling lump.
const MAKEFILE_SKELETON_CHUNK_TARGET_TOKENS: usize = 100;
const MAKEFILE_SKELETON_CHUNK_MIN_TOKENS: usize = 60;

/// Variable context is supporting evidence, not a second config dump.
/// Only compact blocks directly adjacent to build/test targets or
/// referenced by admitted target headers survive this cap.
const MAKEFILE_VARIABLE_BLOCK_MAX_LINES: usize = 10;

/// Literal-target count at which an oversized Makefile skeleton carries
/// the class's full aggregate value. Sparse skeletons still provide a
/// useful reachability hedge, but should not rank like a broad build/test
/// surface merely because excluding recipes made them very cheap.
const MAKEFILE_TARGET_MASS_BASELINE: f64 = 18.0;
const MAKEFILE_TARGET_MASS_FLOOR: f64 = 0.35;

/// Tail-batch value factor for the Dockerfile split — the `RUN` /
/// `COPY` bodies are follow-up to the stage + contract skeleton, but
/// close follow-up: NSes that rank a Dockerfile at all want the build
/// commands within the same budget window as the skeleton.
const DOCKERFILE_TAIL_FACTOR: f64 = 0.85;

/// Minimum line count before a Dockerfile is split. Short Dockerfiles
/// are already their own skeleton and NSes rank them whole; only past
/// this size does the whole-file lump price the stage/contract
/// skeleton out of the early budget.
const DOCKERFILE_SPLIT_MIN_LINES: usize = 50;

/// Rows in the dotenv head batch — samples lead with the
/// mandatory-settings block by convention (linkwarden's first 11 rows
/// are NextAuth + database; linkding's are container/host/superuser).
const DOTENV_MANDATORY_HEAD_LINES: usize = 12;

/// Target marginal cost for source-ordered dotenv tail chunks. Cuts
/// prefer blank/comment-group boundaries; an individually oversized
/// group can also split between adjacent dotenv entries.
const DOTENV_TAIL_CHUNK_TARGET_TOKENS: usize = 150;

/// Avoid a tiny final dotenv chunk when it can fold into its predecessor.
const DOTENV_TAIL_CHUNK_MIN_TOKENS: usize = 100;

/// Optional dotenv keys are deploy-facing contract, not an ordinary
/// names catalog. Apply one modest ops premium to the conserved train;
/// this is aggregate, never repeated per chunk.
const DOTENV_TAIL_OPS_FACTOR: f64 = 1.25;

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
        if matches!(class, Class::BuildEntrypoint) {
            push_makefile_batches(&file, class, ctx, &mut out);
            continue;
        }
        let content = match class {
            Class::Requirements => {
                head_sampled_content(&file, ctx, PLAINTEXT_BYTE_GATE, REQUIREMENTS_HEAD_LINE_CAP)
            }
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

#[derive(Debug)]
enum MakefileStatement {
    Assignment {
        lines: Vec<usize>,
        name: String,
    },
    Target {
        lines: Vec<usize>,
        names: Vec<String>,
        text: String,
    },
    Trivia,
    Other,
}

fn makefile_logical_ranges(lines: &[&str]) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0;
    while start < lines.len() {
        let mut end = start + 1;
        while end < lines.len() && lines[end - 1].trim_end().ends_with('\\') {
            end += 1;
        }
        ranges.push(start..end);
        start = end;
    }
    ranges
}

fn makefile_assignment_name(line: &str) -> Option<String> {
    if line.starts_with(char::is_whitespace) {
        return None;
    }
    let line = line.split('#').next().unwrap_or_default().trim();
    let operator_at = [":=", "?=", "+=", "!=", "="]
        .into_iter()
        .filter_map(|operator| line.find(operator))
        .min()?;
    let before = line[..operator_at].trim();
    if before.is_empty() || before.contains(':') {
        return None;
    }
    let name = before.split_whitespace().last()?;
    if name.is_empty()
        || name
            .chars()
            .any(|ch| ch.is_whitespace() || matches!(ch, '$' | '%' | '/' | '\\'))
    {
        return None;
    }
    Some(name.to_owned())
}

fn is_literal_make_target(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with('.')
        && !name.contains('.')
        && !name.chars().any(|ch| {
            matches!(
                ch,
                '$' | '%' | '/' | '\\' | '*' | '?' | '[' | ']' | '(' | ')' | '&' | '='
            )
        })
}

fn makefile_target(line: &str) -> Option<(Vec<String>, String)> {
    if line.starts_with(char::is_whitespace) || line.trim_start().starts_with('#') {
        return None;
    }
    let text = line.split('#').next().unwrap_or_default().trim();
    let colon = text.find(':')?;
    let lhs = text[..colon].trim();
    let rhs = text[colon + 1..].trim();
    // Target-specific assignments, static-pattern rules, and inline
    // recipes are not target/dependency skeleton rows.
    if rhs.contains(':')
        || rhs.contains(';')
        || [":=", "?=", "+=", "!=", "="]
            .into_iter()
            .any(|operator| rhs.contains(operator))
    {
        return None;
    }
    let names: Vec<String> = lhs.split_whitespace().map(str::to_owned).collect();
    (!names.is_empty() && names.iter().all(|name| is_literal_make_target(name)))
        .then_some((names, text.to_owned()))
}

fn is_build_test_target(name: &str) -> bool {
    name == "all"
        || ["build", "compile", "test", "check", "lint", "bench"]
            .into_iter()
            .any(|prefix| name == prefix || name.starts_with(&format!("{prefix}-")))
}

fn makefile_statements(source: &str) -> Vec<MakefileStatement> {
    let lines: Vec<&str> = source.lines().collect();
    makefile_logical_ranges(&lines)
        .into_iter()
        .map(|range| {
            let first = lines[range.start];
            let physical_lines: Vec<usize> = (range.start + 1..=range.end).collect();
            if first.trim().is_empty() || first.trim_start().starts_with('#') {
                return MakefileStatement::Trivia;
            }
            if let Some(name) = makefile_assignment_name(first) {
                return MakefileStatement::Assignment {
                    lines: physical_lines,
                    name,
                };
            }
            if let Some((names, text)) = makefile_target(first) {
                return MakefileStatement::Target {
                    lines: physical_lines,
                    names,
                    text,
                };
            }
            MakefileStatement::Other
        })
        .collect()
}

/// Structural rows for an oversized Makefile. Literal user-invokable
/// targets are the primary surface. Compact assignment blocks ride along
/// only when the next substantive statement is a build/test target or a
/// variable from the block appears in an admitted target header.
fn makefile_skeleton_items(source: &str) -> Vec<Vec<usize>> {
    let statements = makefile_statements(source);
    let mut selected: Vec<Vec<usize>> = Vec::new();
    let mut attached_targets: Vec<(usize, usize)> = Vec::new();
    let mut index = 0;
    while index < statements.len() {
        if !matches!(statements[index], MakefileStatement::Assignment { .. }) {
            index += 1;
            continue;
        }
        let block_start = index;
        let mut block_end = index;
        let mut block_line_count = 0;
        let mut block_names = Vec::new();
        while block_end < statements.len()
            && matches!(
                statements[block_end],
                MakefileStatement::Assignment { .. } | MakefileStatement::Trivia
            )
        {
            if let MakefileStatement::Assignment { lines, name } = &statements[block_end] {
                block_line_count += lines.len();
                block_names.push(name.as_str());
            }
            block_end += 1;
        }
        let next_build_test = statements
            .get(block_end)
            .and_then(|statement| match statement {
                MakefileStatement::Target { names, .. }
                    if names.iter().any(|name| is_build_test_target(name)) =>
                {
                    Some(block_end)
                }
                _ => None,
            });
        let referenced_target =
            statements
                .iter()
                .enumerate()
                .find_map(|(target_index, statement)| match statement {
                    MakefileStatement::Target { text, .. }
                        if block_names.iter().any(|name| {
                            text.contains(&format!("$({name})"))
                                || text.contains(&format!("${{{name}}}"))
                        }) =>
                    {
                        Some(target_index)
                    }
                    _ => None,
                });
        if block_line_count <= MAKEFILE_VARIABLE_BLOCK_MAX_LINES
            && let Some(target_index) = next_build_test.or(referenced_target)
        {
            let mut block_lines: Vec<usize> = statements[block_start..block_end]
                .iter()
                .filter_map(|statement| match statement {
                    MakefileStatement::Assignment { lines, .. } => Some(lines.iter().copied()),
                    _ => None,
                })
                .flatten()
                .collect();
            if let Some((_, selected_index)) = attached_targets
                .iter()
                .find(|(attached_target, _)| *attached_target == target_index)
            {
                selected[*selected_index].append(&mut block_lines);
                selected[*selected_index].sort_unstable();
                selected[*selected_index].dedup();
            } else if let MakefileStatement::Target { lines, .. } = &statements[target_index] {
                block_lines.extend(lines);
                block_lines.sort_unstable();
                block_lines.dedup();
                let selected_index = selected.len();
                selected.push(block_lines);
                attached_targets.push((target_index, selected_index));
            }
        }
        index = block_end.max(index + 1);
    }
    selected.extend(
        statements.into_iter().enumerate().filter_map(
            |(target_index, statement)| match statement {
                MakefileStatement::Target { lines, .. }
                    if !attached_targets
                        .iter()
                        .any(|(attached_target, _)| *attached_target == target_index) =>
                {
                    Some(lines)
                }
                _ => None,
            },
        ),
    );
    selected.sort_unstable_by_key(|lines| lines[0]);
    selected
}

fn makefile_skeleton_contents(
    file: &Path,
    source: &str,
    ctx: &WalkCtx,
) -> (Vec<crate::content::BatchContent>, usize) {
    let items = makefile_skeleton_items(source);
    if items.is_empty() {
        return (Vec::new(), 0);
    }
    let target_count = items.len();
    let line_count = source.lines().count();
    let mut all_selected: Vec<usize> = items
        .iter()
        .flat_map(|lines| lines.iter().copied())
        .collect();
    all_selected.sort_unstable();
    all_selected.dedup();
    let all_ellipses = gap_ellipses(&all_selected, line_count);
    let lines_for = |range: Range<usize>| {
        let is_first_chunk = range.start == 0;
        let mut selected: Vec<usize> = items[range]
            .iter()
            .flat_map(|lines| lines.iter().copied())
            .collect();
        selected.sort_unstable();
        selected.dedup();
        // Partition the global skeleton's gap markers with the source
        // rows: a leading marker belongs to the first chunk, and every
        // other marker belongs to the chunk containing the selected row
        // immediately before that gap. Chunks therefore never claim the
        // same line even when the scheduler buys them out of order.
        let ellipses = all_ellipses
            .iter()
            .copied()
            .filter(|ellipsis| {
                (*ellipsis == 1 && is_first_chunk)
                    || ellipsis
                        .checked_sub(1)
                        .is_some_and(|previous| selected.binary_search(&previous).is_ok())
            })
            .collect();
        FileLines::new(selected).with_ellipses(ellipses)
    };
    let cost = |range: Range<usize>| {
        single_file_lines_content(file, source, lines_for(range))
            .map(|content| ctx.marginal_tokens(&content))
            .unwrap_or(0)
    };
    let contents = budget_chunk_ranges(
        items.len(),
        cost,
        MAKEFILE_SKELETON_CHUNK_TARGET_TOKENS,
        MAKEFILE_SKELETON_CHUNK_MIN_TOKENS,
        |_| true,
        |range| cost(range) <= MAKEFILE_SKELETON_CHUNK_TARGET_TOKENS + 50,
    )
    .into_iter()
    .filter_map(|range| single_file_lines_content(file, source, lines_for(range)))
    .collect();
    (contents, target_count)
}

fn makefile_target_mass_factor(target_count: usize) -> f64 {
    (target_count as f64 / MAKEFILE_TARGET_MASS_BASELINE).clamp(MAKEFILE_TARGET_MASS_FLOOR, 1.0)
}

fn push_makefile_batches(file: &Path, class: Class, ctx: &WalkCtx, out: &mut Vec<Batch<BatchKey>>) {
    let Some(source) = gated_read_source(file, ctx, MAKEFILE_SKELETON_BYTE_GATE) else {
        return;
    };
    let line_count = source.lines().count();
    if line_count == 0 {
        return;
    }
    let value = class_value(class, file, ctx);
    if line_count <= BUILD_ENTRYPOINT_LINE_CAP {
        // Preserve the pre-skeleton whole-file contract exactly: a
        // short-but-extremely-wide Makefile still fails the original
        // 8KB metadata gate rather than riding the larger structural
        // parser gate into a huge Whole batch.
        if let Some(content) = gated_whole_file_content(
            file,
            ctx,
            BUILD_ENTRYPOINT_BYTE_GATE,
            BUILD_ENTRYPOINT_LINE_CAP,
        ) {
            out.push(Batch {
                key: PlaintextKey::Whole {
                    file: file.to_path_buf(),
                }
                .into(),
                predecessor: None,
                content,
                value,
            });
        }
        return;
    }
    let (contents, target_count) = makefile_skeleton_contents(file, &source, ctx);
    let costs: Vec<usize> = contents
        .iter()
        .map(|content| ctx.marginal_tokens(content))
        .collect();
    let factors = conserved_catalog_chunk_factors(&costs, DEFAULT_CONCAVITY_EXPONENT);
    let skeleton_value = value * makefile_target_mass_factor(target_count);
    for (chunk_index, (content, factor)) in contents.into_iter().zip(factors).enumerate() {
        out.push(Batch {
            key: PlaintextKey::MakefileSkeletonChunk {
                file: file.to_path_buf(),
                chunk_index,
            }
            .into(),
            predecessor: None,
            content,
            value: skeleton_value * factor,
        });
    }
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

/// Source ranges for dotenv tail groups. A group begins after a blank
/// separator or at the start of a comment block; consecutive comment
/// rows stay attached, as do the config rows documented by that block.
/// Returned ranges are zero-based and end-exclusive over `lines`.
fn dotenv_tail_groups(lines: &[&str], head_end: usize) -> Vec<Range<usize>> {
    if head_end >= lines.len() {
        return Vec::new();
    }
    let mut starts = vec![head_end];
    for index in head_end + 1..lines.len() {
        let current = lines[index].trim();
        if current.is_empty() {
            continue;
        }
        let previous = lines[index - 1].trim();
        let starts_comment_block = current.starts_with('#') && !previous.starts_with('#');
        if previous.is_empty() || starts_comment_block {
            starts.push(index);
        }
    }
    starts.dedup();
    starts
        .iter()
        .enumerate()
        .map(|(index, &start)| start..starts.get(index + 1).copied().unwrap_or(lines.len()))
        .collect()
}

fn is_dotenv_entry_line(line: &str) -> bool {
    let line = line.trim();
    !line.is_empty() && !line.starts_with('#') && line.contains('=')
}

/// Partition the dotenv tail into source-ordered, comment-group-aligned
/// chunks. Each non-final chunk carries an ellipsis on the next chunk's
/// first row; scheduling that descendant replaces the marker with full
/// content, so every purchased prefix remains visibly incomplete.
fn dotenv_tail_chunks(file: &Path, source: &str, head_end: usize, ctx: &WalkCtx) -> Vec<FileLines> {
    let lines: Vec<&str> = source.lines().collect();
    let groups = dotenv_tail_groups(&lines, head_end);
    let tail_len = lines.len().saturating_sub(head_end);
    let lines_for = |range: Range<usize>| {
        let start = head_end + range.start;
        let end = head_end + range.end;
        let mut chunk = FileLines::new((start + 1..=end).collect());
        if end < lines.len() {
            chunk = chunk.with_ellipses(vec![end + 1]);
        }
        chunk
    };
    let cost = |range: Range<usize>| {
        single_file_lines_content(file, source, lines_for(range))
            .map(|content| ctx.marginal_tokens(&content))
            .unwrap_or(0)
    };
    budget_chunk_ranges(
        tail_len,
        cost,
        DOTENV_TAIL_CHUNK_TARGET_TOKENS,
        DOTENV_TAIL_CHUNK_MIN_TOKENS,
        |end| {
            let source_end = head_end + end;
            let group_boundary = groups.iter().any(|group| group.end == source_end);
            if group_boundary {
                return true;
            }
            // A very large comment group (for example, a long roster
            // under one header) still needs a purchasable shape. Fall
            // back to a cut between adjacent dotenv assignments; never
            // strand a comment or blank separator from what follows.
            is_dotenv_entry_line(lines[source_end - 1]) && is_dotenv_entry_line(lines[source_end])
        },
        |range| cost(range) <= DOTENV_TAIL_CHUNK_TARGET_TOKENS + 50,
    )
    .into_iter()
    .map(lines_for)
    .collect()
}

/// Dotenv samples ship as a cheap mandatory-settings head plus chained,
/// independently purchasable optional-setting chunks. A capped roster-
/// mass factor recognizes that a hundreds-entry config surface carries
/// more value than a tiny sample; that one aggregate allocation is then
/// conserved across the chunks rather than multiplied per chunk.
fn push_dotenv_batches(file: &Path, class: Class, ctx: &WalkCtx, out: &mut Vec<Batch<BatchKey>>) {
    let Some(source) = gated_read_source(file, ctx, DOTENV_BYTE_GATE) else {
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
    let Some(head_content) = single_file_lines_content(file, &source, head) else {
        return;
    };
    out.push(Batch {
        key: head_key.clone().into(),
        predecessor: None,
        content: head_content,
        value: head_value,
    });
    let chunks = dotenv_tail_chunks(file, &source, head_end, ctx);
    if chunks.is_empty() {
        return;
    }
    let chunk_contents: Vec<_> = chunks
        .into_iter()
        .filter_map(|lines| single_file_lines_content(file, &source, lines))
        .collect();
    let chunk_costs: Vec<_> = chunk_contents
        .iter()
        .map(|content| ctx.marginal_tokens(content))
        .collect();
    let factors = conserved_catalog_chunk_factors(&chunk_costs, DEFAULT_CONCAVITY_EXPONENT);
    let tail_entries = source
        .lines()
        .skip(head_end)
        .filter(|line| is_dotenv_entry_line(line))
        .count();
    let tail_factor = roster_mass_factor(tail_entries) * DOTENV_TAIL_OPS_FACTOR;
    let mut predecessor = BatchKey::Plaintext(head_key);
    for (chunk_index, (content, factor)) in chunk_contents.into_iter().zip(factors).enumerate() {
        let key = PlaintextKey::DotenvChunk {
            file: file.to_path_buf(),
            chunk_index,
        };
        out.push(Batch {
            key: key.clone().into(),
            predecessor: Some(predecessor),
            content,
            value: head_value * tail_factor * factor,
        });
        predecessor = BatchKey::Plaintext(key);
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

/// Escape character for line continuations: `\` unless a leading
/// `# escape=` parser directive picks the backtick (Windows-style
/// Dockerfiles).
fn dockerfile_escape_char(lines: &[&str]) -> char {
    // Parser directives are `# key=value` comments before any other
    // content; the first non-directive line ends the block.
    for raw in lines {
        let Some(rest) = raw.trim().strip_prefix('#') else {
            break;
        };
        let Some((key, value)) = rest.split_once('=') else {
            break;
        };
        if key.trim().eq_ignore_ascii_case("escape") && value.trim() == "`" {
            return '`';
        }
    }
    '\\'
}

/// Append the delimiters of any heredocs opened on `line` (`<<EOF`,
/// `<<-EOF`, quoted variants), each with whether `<<-` permits an
/// indented closing delimiter. `<<` must sit at the start of a shell
/// word (start of line or after whitespace), which keeps operators
/// embedded in program text like `cout<<msg` out; delimiters must
/// start with a letter or underscore, which keeps shell arithmetic
/// like `1<<2` out.
fn collect_heredoc_openers(line: &str, out: &mut VecDeque<(String, bool)>) {
    let mut rest = line;
    let mut offset = 0;
    while let Some(pos) = rest.find("<<") {
        let at_word_start = offset + pos == 0
            || rest[..pos]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_whitespace());
        offset += pos + 2;
        rest = &rest[pos + 2..];
        if !at_word_start {
            continue;
        }
        let allow_indented_close = rest.starts_with('-');
        let mut s = rest.strip_prefix('-').unwrap_or(rest);
        if let Some(unquoted) = s.strip_prefix(['"', '\'']) {
            s = unquoted;
        }
        if !s.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
            continue;
        }
        let delimiter: String = s
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        out.push_back((delimiter, allow_indented_close));
    }
}

/// 1-based lines of the Dockerfile contract skeleton: every contract
/// instruction with its continuation lines and heredoc body, the
/// contiguous comment block directly above each `FROM` (stage-label
/// comments by convention), and pre-`FROM` global `ARG`s (they feed
/// `FROM ${...}` references). `ONBUILD` routes by its payload
/// instruction.
fn dockerfile_contract_lines(source: &str) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().collect();
    let escape = dockerfile_escape_char(&lines);
    let mut selected = Vec::new();
    // `Some(sel)` while inside a continuation of an instruction whose
    // selection state is `sel`.
    let mut continuation: Option<bool> = None;
    // Delimiters of heredocs whose bodies are still pending; body
    // lines inherit the opening instruction's selection state.
    let mut heredocs: VecDeque<(String, bool)> = VecDeque::new();
    let mut heredoc_selected = false;
    let mut seen_from = false;
    for (i, raw) in lines.iter().enumerate() {
        let line = raw.trim();
        if let Some(sel) = continuation {
            if sel {
                selected.push(i + 1);
            }
            // Comment and blank lines inside a continuation don't end
            // it (Docker skips them).
            if !line.is_empty() && !line.starts_with('#') {
                collect_heredoc_openers(line, &mut heredocs);
                if !line.ends_with(escape) {
                    continuation = None;
                }
            }
            continue;
        }
        if let Some((delimiter, allow_indented_close)) = heredocs.front() {
            if heredoc_selected {
                selected.push(i + 1);
            }
            // Plain `<<` closes only on an unindented delimiter line;
            // `<<-` also accepts an indented one.
            let closes = if *allow_indented_close {
                line == delimiter
            } else {
                raw.trim_end() == delimiter
            };
            if closes {
                heredocs.pop_front();
            }
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut instruction = line
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_uppercase();
        if instruction == "ONBUILD" {
            instruction = line
                .split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_ascii_uppercase();
        }
        let is_selected = is_dockerfile_contract_instruction(&instruction)
            || (instruction == "ARG" && !seen_from);
        if instruction == "FROM" {
            seen_from = true;
        }
        collect_heredoc_openers(line, &mut heredocs);
        heredoc_selected = is_selected;
        if line.ends_with(escape) {
            continuation = Some(is_selected);
        }
        if !is_selected {
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
    let Some(source) = gated_read_source(file, ctx, BUILD_ENTRYPOINT_BYTE_GATE) else {
        return;
    };
    let line_count = source.lines().count();
    if line_count == 0 || line_count > BUILD_ENTRYPOINT_LINE_CAP {
        return;
    }
    let value = class_value(class, file, ctx);
    let head_key = PlaintextKey::Whole {
        file: file.to_path_buf(),
    };
    let split = (line_count > DOCKERFILE_SPLIT_MIN_LINES)
        .then(|| {
            let head_lines = dockerfile_contract_lines(&source);
            let tail_lines: Vec<usize> = (1..=line_count)
                .filter(|line| head_lines.binary_search(line).is_err())
                .collect();
            (head_lines, tail_lines)
        })
        .filter(|(head_lines, tail_lines)| !head_lines.is_empty() && !tail_lines.is_empty());
    let Some((head_lines, tail_lines)) = split else {
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
    };
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
    // No tail ellipses: head ∪ tail covers every line, and a tail
    // Ellipsis record on a head-owned line would replace the head's
    // already-rendered row (`apply_spans` overwrites ancestor records
    // unconditionally).
    if let Some(content) = single_file_lines_content(file, &source, FileLines::new(tail_lines)) {
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
            (".env.sample", Some(Class::DotenvSample)),
            (".env.example", Some(Class::DotenvSample)),
            (".env.template", Some(Class::DotenvSample)),
            (".env.dist", Some(Class::DotenvSample)),
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

    #[test]
    fn plaintext_makefile_skeleton_selects_literal_targets_and_small_variable_blocks() {
        let source = "\
BUILD_FLAGS = --release
TEST_DEPS := unit integration

build: $(BUILD_FLAGS)
\tcargo build
check: unit \\
 integration
.PHONY: build check
%.o: %.c
$(OUTPUT): input.c
artifact.bin: input.c
inline: dep ; echo recipe
lint: SHELL:=/bin/bash

AUX = helper

test: $(TEST_DEPS)
\tcargo test
";
        let selected: Vec<usize> = makefile_skeleton_items(source)
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(selected, vec![1, 2, 4, 6, 7, 15, 17]);
    }

    #[test]
    fn plaintext_small_makefile_stays_whole() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let file = root.join("Makefile");
        std::fs::write(&file, "FLAGS = --all\n\nbuild:\n\ttool $(FLAGS)\n").unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        assert_eq!(batches.len(), 1);
        assert!(matches!(
            &batches[0].key,
            BatchKey::Plaintext(PlaintextKey::Whole { file: batch_file })
                if batch_file == &file
        ));
    }

    #[test]
    fn plaintext_small_but_wide_makefile_keeps_original_byte_gate() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let file = root.join("Makefile");
        let source = format!(
            "build: {}\n",
            "dependency ".repeat(BUILD_ENTRYPOINT_BYTE_GATE)
        );
        std::fs::write(&file, source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        assert!(expand_in_dir(root, &ctx).is_empty());
    }

    #[test]
    fn plaintext_oversized_makefile_chunks_conserve_value_and_exclude_recipes() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let file = root.join("Makefile");
        let mut source = String::from("BUILD_DEPS = common-a common-b\n\nbuild: $(BUILD_DEPS)\n");
        for index in 0..60 {
            source.push_str(&format!(
                "target-{index:02}: dep-{index:02}-a dep-{index:02}-b\n\tRECIPE_SHOULD_NOT_RENDER_{index:02}\n"
            ));
        }
        source.push_str("%.o: %.c\n\tPATTERN_RECIPE\n$(OUTPUT): generated.c\n\tGENERATED_RECIPE\n");
        std::fs::write(&file, &source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        assert!(batches.len() > 1, "expected several skeleton chunks");
        assert!(batches.iter().all(|batch| matches!(
            &batch.key,
            BatchKey::Plaintext(PlaintextKey::MakefileSkeletonChunk {
                file: batch_file,
                ..
            }) if batch_file == &file
        )));
        let total_value: f64 = batches.iter().map(|batch| batch.value).sum();
        let expected =
            class_value(Class::BuildEntrypoint, &file, &ctx) * makefile_target_mass_factor(61);
        assert!(
            (total_value - expected).abs() < 1e-9,
            "skeleton allocation must be conserved: total={total_value}, expected={expected}",
        );

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 1_000_000, None);
        let rendered = scheduler.run_with_report().tree.render();
        assert!(rendered.contains("build: $(BUILD_DEPS)"));
        assert!(rendered.contains("target-59: dep-59-a dep-59-b"));
        assert!(!rendered.contains("RECIPE_SHOULD_NOT_RENDER"));
        assert!(!rendered.contains("%.o: %.c"));
        assert!(!rendered.contains("$(OUTPUT): generated.c"));
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
    fn plaintext_dotenv_groups_cut_before_comment_blocks() {
        let lines = [
            "HEAD=1",
            "",
            "# Group A",
            "A=1",
            "B=2",
            "# Group B without a blank separator",
            "C=3",
            "",
            "# Group C",
            "D=4",
        ];
        assert_eq!(dotenv_tail_groups(&lines, 2), vec![2..5, 5..8, 8..10]);
        assert!(is_dotenv_entry_line("FOO=bar"));
        assert!(!is_dotenv_entry_line("# Example: FOO=bar"));
    }

    #[test]
    fn plaintext_dotenv_chunks_chain_and_conserve_tail_value() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let file = root.join(".env.sample");
        let mut source = String::new();
        for index in 0..DOTENV_MANDATORY_HEAD_LINES {
            source.push_str(&format!("MANDATORY_{index:02}=value_{index:02}\n"));
        }
        for group in 0..16 {
            source.push_str(&format!("# Optional group {group:02}\n"));
            for setting in 0..6 {
                source.push_str(&format!(
                    "OPTIONAL_{group:02}_{setting:02}=value_{group:02}_{setting:02}\n"
                ));
            }
            source.push('\n');
        }
        std::fs::write(&file, &source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        let head = batches
            .iter()
            .find(|batch| {
                matches!(
                    &batch.key,
                    BatchKey::Plaintext(PlaintextKey::Whole { file: batch_file })
                        if batch_file == &file
                )
            })
            .expect("dotenv head batch");
        let chunks: Vec<_> = batches
            .iter()
            .filter(|batch| {
                matches!(
                    &batch.key,
                    BatchKey::Plaintext(PlaintextKey::DotenvChunk { file: batch_file, .. })
                        if batch_file == &file
                )
            })
            .collect();
        assert!(
            chunks.len() > 2,
            "expected several dotenv chunks: {chunks:?}"
        );

        let total_tail_value: f64 = chunks.iter().map(|batch| batch.value).sum();
        let tail_entries = source
            .lines()
            .skip(DOTENV_MANDATORY_HEAD_LINES)
            .filter(|line| is_dotenv_entry_line(line))
            .count();
        assert!(
            (total_tail_value
                - head.value * roster_mass_factor(tail_entries) * DOTENV_TAIL_OPS_FACTOR)
                .abs()
                < 1e-9,
            "tail allocation must be conserved: head={}, tail={total_tail_value}",
            head.value,
        );
        let mut expected_predecessor = head.key.clone();
        for (chunk_index, batch) in chunks.iter().enumerate() {
            assert_eq!(batch.predecessor.as_ref(), Some(&expected_predecessor));
            expected_predecessor = BatchKey::Plaintext(PlaintextKey::DotenvChunk {
                file: file.clone(),
                chunk_index,
            });
            assert_eq!(batch.key, expected_predecessor);
        }

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 1_000_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();
        for line in source.lines().filter(|line| !line.is_empty()) {
            assert_eq!(
                rendered.matches(line).count(),
                1,
                "source line {line:?} should render exactly once:\n{rendered}",
            );
        }
    }

    #[test]
    fn plaintext_dockerfile_contract_lines_parser_edges() {
        // Pre-FROM global ARG feeds `FROM ${...}` and is contract;
        // post-FROM ARG is per-stage build environment.
        let src = "ARG BASE=alpine\nFROM ${BASE} AS app\nARG DEBUG=0\nCMD [\"app\"]\n";
        assert_eq!(dockerfile_contract_lines(src), vec![1, 2, 4]);
        // ONBUILD routes by its payload instruction.
        let src = "FROM base\nONBUILD EXPOSE 80\nONBUILD RUN make\n";
        assert_eq!(dockerfile_contract_lines(src), vec![1, 2]);
        // Heredoc body lines belong to the opening instruction even
        // when one starts with a contract keyword.
        let src = "FROM base\nRUN <<EOF\nFROM not-an-instruction\nEOF\nEXPOSE 80\n";
        assert_eq!(dockerfile_contract_lines(src), vec![1, 5]);
        // Blank lines inside a continuation don't end it.
        let src = "FROM base\nENTRYPOINT [\"sh\", \\\n\n  \"-c\", \"run.sh\"]\nRUN make\n";
        assert_eq!(dockerfile_contract_lines(src), vec![1, 2, 3, 4]);
        // A `# escape=` directive switches the continuation character
        // (the directive line itself rides along as the comment block
        // above the first FROM).
        let src =
            "# escape=`\nFROM base\nHEALTHCHECK --interval=30s `\n  CMD curl localhost\nRUN make\n";
        assert_eq!(dockerfile_contract_lines(src), vec![1, 2, 3, 4]);
    }

    /// A long Dockerfile schedules as head + gated tail; with both
    /// batches paid for, the render reconstructs every source line
    /// exactly once. Regression for the tail's gap ellipses landing on
    /// head-owned lines and deleting rendered contract rows (Codex
    /// adversarial review).
    #[test]
    fn plaintext_dockerfile_split_head_plus_tail_renders_whole_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut src = String::from("# Stage: builder\nFROM rust:1.86 AS builder\n");
        for i in 0..DOCKERFILE_SPLIT_MIN_LINES {
            src.push_str(&format!("RUN echo step-{i:02}\n"));
        }
        src.push_str("# Stage: app\nFROM alpine AS app\nEXPOSE 3000\nENTRYPOINT [\"app\"]\n");
        std::fs::write(root.join("Dockerfile"), &src).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 1_000_000, None);
        let report = scheduler.run_with_report();
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter().any(|k| matches!(
                k,
                BatchKey::Plaintext(PlaintextKey::Tail { file }) if file.ends_with("Dockerfile"),
            )),
            "expected a scheduled Dockerfile tail batch; scheduled keys: {keys:?}",
        );
        let rendered = report.tree.render();
        for line in src.lines() {
            assert_eq!(
                rendered.matches(line).count(),
                1,
                "source line {line:?} should render exactly once:\n{rendered}",
            );
        }
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
