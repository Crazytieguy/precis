//! Plaintext walker — the home for every file precis has no parser
//! for. Two jobs:
//!
//! 1. **Named plaintext files** ([`classify_plaintext`]): license and
//!    ignore files, compact toolchain/build/package manifests, selected
//!    build scripts, requirement lists, version/TODO stamps, man-page
//!    ledes, `Makefile`/`Dockerfile`/dotenv skeletons. These get
//!    class-specific treatment and a per-class value preset.
//! 2. **Every other source-like text file** ([`Class::SourceText`]):
//!    the language-agnostic fallback for the ~90% of file formats no
//!    tree-sitter walker in this crate claims — Java, C++, Ruby, PHP,
//!    Swift, Kotlin, C#, Scala, Elixir, Haskell, Vue, Svelte, CSS,
//!    HTML, reST, plain text and the rest of
//!    [`SOURCE_TEXT_EXTENSIONS`]. Without it those files reach the
//!    output as a bare filename in a directory listing and nothing
//!    else, which is what a repository in any of those languages
//!    renders as.
//!
//! The named whitelist is credential-aware (see [`classify_plaintext`]).
//! Files with format-specific siblings (`.eslintrc.json`,
//! `.prettierrc.js`, `LICENSE.md`) stay with the owning walker, and
//! credential-bearing dotfiles (`.npmrc`, `.netrc`, `.env`, `.pypirc`)
//! are NOT in the whitelist. Checked-in dotenv *samples*
//! (`.env.sample` / `.env.example`) ARE admitted — they carry
//! placeholder values by convention and are the deploy-facing
//! config-key documentation. Shell scripts are only admitted to
//! [`Class::BuildScript`] from build-script locations, and exact
//! env/secret/credential stems ([`is_credential_stem`]) are denied
//! before any `.sh` classification — in the fallback too — because
//! they commonly export tokens for local tooling.
//!
//! Budget protection: `PLAINTEXT_LINE_CAP` bounds a `Whole` batch;
//! a file over the cap is head-sampled rather than dropped, so
//! "slightly too long" never means "renders as nothing".

use std::path::Path;

use crate::batch::{Batch, BatchKey, PlaintextKey};
use crate::value::mix_signals;

use super::{
    FileLines, WalkCtx, dedup_sorted, fs::list_dir, gated_read_source, gated_whole_file_content,
    path_depth_factor, single_file_lines_content,
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

/// Promotion for a small root build file — see
/// [`small_build_file_factor`]. Swept on the full corpus: +0.0013 at
/// ×1.15 and ×1.3, +0.0021 at ×1.5, and +0.0028 from ×1.8 upward, flat
/// out to ×4.5 (the promoted files are cheap enough that once they win
/// their rank race, more value cannot move them further). Set inside
/// that plateau rather than at its edge.
const SMALL_BUILD_FILE_PROMOTION: f64 = 2.0;

/// Line count past which a build file stops being the compact "here is
/// how you build and run this" surface and becomes a build *system*
/// (generated autotools input, a 900-line BSP rules file) that a
/// reader consults rather than reads. Set at the census boundary and
/// to match [`BUILD_ENTRYPOINT_LINE_CAP`]; a 60-line cap measures
/// identically, because the content gates already bound every path
/// that reaches here.
const SMALL_BUILD_FILE_LINE_CAP: usize = 100;

/// Keeps the promotion at the repository root. Depth is counted in
/// `WalkCtx::depth_from_root` terms, where a root-level file is 1.
/// Load-bearing only for [`Class::BuildScript`]: unlike
/// [`Class::BuildEntrypoint`], its location gate admits a `scripts/`
/// directory at *any* depth, and a `packages/foo/scripts/build.sh` is
/// one component's build step rather than the project's.
const SMALL_BUILD_FILE_MAX_DEPTH: usize = 1;

/// Rows in the dotenv head batch — samples lead with the
/// mandatory-settings block by convention (linkwarden's first 11 rows
/// are NextAuth + database; linkding's are container/host/superuser).
const DOTENV_MANDATORY_HEAD_LINES: usize = 12;

/// Selection budget for a [`Class::SourceText`] declaration surface,
/// per line class. Imports and comments are damped so a 40-import
/// Java file or a 15-line license banner cannot consume the whole
/// slice before the first declaration; declarations get the bulk.
const SOURCE_TEXT_IMPORT_LINES: usize = 4;
const SOURCE_TEXT_COMMENT_LINES: usize = 2;
const SOURCE_TEXT_DECL_LINES: usize = 8;

/// Declarations at which a surface stops descending into deeper
/// indentation levels — enough to read as a roster rather than as a
/// single wrapper line.
const SOURCE_TEXT_MIN_DECLS: usize = 4;

/// Hard bound on levels descended, so a file that never reaches
/// [`SOURCE_TEXT_MIN_DECLS`] (a script that is one long block) walks
/// into its statement bodies only so far.
const SOURCE_TEXT_MAX_INDENT_LEVELS: usize = 4;

/// Pre-flight byte gate for the fallback. Generous — only the
/// declaration surface is rendered, not the file — but bounded so a
/// stray data blob is never read.
const SOURCE_TEXT_BYTE_GATE: usize = 512 * 1024;

/// Mean bytes per line above which a file is machine-generated rather
/// than hand-wrapped: minified bundles, single-line JSON-ish dumps and
/// serialized blobs all sit in the thousands, hand-written source and
/// prose in the tens. Rejects them without an extension blocklist.
const SOURCE_TEXT_MAX_MEAN_LINE_BYTES: usize = 200;

/// Characters past which an indentation-zero line stops being a
/// declaration a reader skims and becomes an attribute dump, a data
/// row, or generated output — a Maven `<project xmlns=…>` opener costs
/// ~90 tokens and says nothing. Skipped rather than truncated: a
/// half-line is not more informative than the filename.
const SOURCE_TEXT_MAX_LINE_CHARS: usize = 200;

/// Leading lines scanned for a generated-file banner.
const SOURCE_TEXT_GENERATED_SCAN_LINES: usize = 8;

/// Bytes scanned for a NUL, which no text file contains but a binary
/// misnamed `.txt` (or a UTF-8-decodable data blob) does.
const SOURCE_TEXT_NUL_SCAN_BYTES: usize = 8192;

/// Pre-flight byte gate for dotenv samples — generous (only the head
/// renders) but bounded.
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
    /// Compact build/deploy entrypoints (`Makefile`, `Taskfile`,
    /// `Dockerfile`).
    BuildEntrypoint,
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
    /// A source file in a language no walker parses — the
    /// language-agnostic fallback. Rendered as a declaration surface.
    SourceText,
    /// A prose or flat-config file with no owning walker. Same
    /// extraction (a file with no nesting has every line at
    /// indentation zero, so the surface *is* its head slice), but a
    /// head slice claims much less than a declaration roster does and
    /// is priced for it.
    SourceProse,
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
        // `Taskfile.yaml` is a `Makefile` in YAML clothing — the task
        // runner's target roster. The YAML walker enumerates the
        // extension but classifies only deployment / CI / tooling
        // configs, so it declines this one and ownership stays here.
        "Makefile" | "Taskfile.yaml" | "Taskfile.yml" | "Dockerfile" | "Containerfile" => {
            return Some(Class::BuildEntrypoint);
        }
        ".gitmodules" | "configure.ac" => return Some(Class::BuildScript),
        "setup.cfg" => return Some(Class::PackageConfig),
        _ => {}
    }
    if lower == "requirements.txt" {
        return Some(Class::Requirements);
    }
    if crate::value::is_dotenv_sample_filename(name) {
        return Some(Class::DotenvSample);
    }
    if let Some(stem) = lower.strip_suffix(".sh") {
        if is_credential_stem(stem) {
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

/// File stems that never render regardless of extension — they
/// commonly hold real tokens for local tooling.
fn is_credential_stem(stem: &str) -> bool {
    matches!(
        stem,
        "env" | ".env" | "secret" | "secrets" | "credential" | "credentials" | "creds"
    )
}

/// Programming-language extensions the fallback claims. A directory
/// of these files is a source package whether or not this crate can
/// parse them, so [`crate::walker::fs::is_source_inventory_file`]
/// reads from this list too — without it a `com/google/gson/` full of
/// `.java` does not register as source, its listing loses the ratio
/// race at depth, and the files inside it never even become
/// candidates.
///
/// An allowlist rather than v0.1's "any extension that decodes as
/// UTF-8" rule. SVG, source maps, PO catalogs, CSV, armored keys and
/// notebooks are all valid text and all worthless as a declaration
/// surface, and a blocklist of them is open-ended in a way this list
/// is not — a format missing here renders as it does today (a name in
/// a listing), a wrong entry renders noise.
///
/// Fully-nested markup (XML/POM/XSD, HTML, template dialects) is
/// deliberately absent from all three lists: its only
/// indentation-zero lines are the document declaration and the root
/// element (`<!DOCTYPE html>`, `<project xmlns=…>`), so the fallback
/// has nothing true to say about it and would spend real tokens
/// saying it — ~27 tokens per page across a generated `docs/` tree.
/// Markup needs a walker that understands nesting, not this one.
pub(crate) const SOURCE_TEXT_LANGUAGE_EXTENSIONS: &[&str] = &[
    // JVM / .NET
    "java", "kt", "kts", "scala", "sc", "groovy", "clj", "cljs", "cljc", "cs", "fs", "fsx", "vb",
    // C family (`.c` / `.h` belong to the C walker)
    "cpp", "cc", "cxx", "hpp", "hh", "hxx", "m", "mm", "cu", "cuh",
    // other compiled languages
    "swift", "zig", "dart", "nim", "cr", "d", "hs", "lhs", "ml", "mli", "elm", "erl", "hrl", "ex",
    "exs", // scripting
    "rb", "php", "pl", "pm", "r", "jl", "tcl", "pyi", // component-file web frameworks
    "vue", "svelte", "astro", "jsx",
];

/// Contract-bearing extensions whose indentation-zero lines are real
/// declarations — protobuf messages, GraphQL types, Terraform blocks,
/// Gradle plugin and dependency blocks — but whose *directories* are
/// schema or config trees rather than source packages. Same surface
/// value as a language file, no source-inventory promotion: promoting
/// them buys stacks of asset-tree listings (measured: dockly −0.078
/// when stylesheet dirs were promoted).
const SOURCE_TEXT_DECLARATIVE_EXTENSIONS: &[&str] = &[
    "proto", "thrift", "graphql", "gql", "capnp", "fbs", "tf", "tfvars", "hcl", "nix", "dhall",
    "cue", "gradle", "gemspec", "podspec", "rake",
];

/// Extensions with no *program* structure to surface. `.bat`/`.cmd`
/// are absent on purpose — in practice they are generated wrappers
/// (`gradlew.bat`, `mvnw.cmd`), 158 tokens of argument marshalling.
/// Two shapes, one price:
///
/// - prose, flat config, shell scripts and build glue are sequences
///   of statements, so every line sits at indentation zero and the
///   "surface" is just a head slice;
/// - stylesheets do have declarations at indentation zero, but a
///   selector list describes presentation, not what the program is
///   or does — nine 10-token stylesheet slices displacing a
///   package's Python decl surfaces is a bad trade (measured:
///   linkding −0.044 at language pricing).
///
/// Claimed either way — a slice beats a bare filename — but priced
/// near the floor, and never a source inventory.
const SOURCE_TEXT_FLAT_EXTENSIONS: &[&str] = &[
    "rst",
    "adoc",
    "asciidoc",
    "txt",
    "text",
    "tex",
    "org",
    "mdx",
    "ini",
    "cfg",
    "conf",
    "properties",
    "sh",
    "bash",
    "zsh",
    "fish",
    "ps1",
    "psm1",
    "awk",
    "cmake",
    "mk",
    "mak",
    "bzl",
    "bazel",
    "gyp",
    "gni",
    "ld",
    "css",
    "scss",
    "sass",
    "less",
    "styl",
];

/// Extensionless build manifests the fallback claims by exact name.
const SOURCE_TEXT_FILENAMES: &[&str] = &[
    "Gemfile",
    "Rakefile",
    "Guardfile",
    "Brewfile",
    "Podfile",
    "Procfile",
    "Vagrantfile",
    "Justfile",
    "justfile",
    "Jenkinsfile",
    "Berksfile",
    "Appfile",
    "Fastfile",
    "BUILD",
    "WORKSPACE",
    "SConstruct",
    "meson.build",
];

/// The fallback class `name` falls into on name evidence alone, or
/// `None` when the fallback doesn't own it. Rejects derived artifacts
/// (minified/bundled output, lockfiles, source maps) and credential
/// stems before the extension check — these are the text files whose
/// content makes the output worse, not better.
fn classify_source_text(name: &str) -> Option<Class> {
    let lower = name.to_ascii_lowercase();
    if SOURCE_TEXT_FILENAMES.contains(&name) {
        return Some(Class::SourceText);
    }
    let (stem, ext) = lower.rsplit_once('.')?;
    if is_credential_stem(stem) || is_credential_stem(&lower) {
        return None;
    }
    // Derived siblings of a hand-authored source: `app.min.js`,
    // `bundle.chunk.css`, `pnpm-lock.yaml`, `main.js.map`.
    if stem.ends_with(".min")
        || stem.ends_with("-min")
        || stem.ends_with(".bundle")
        || stem.ends_with(".chunk")
        || stem.ends_with(".generated")
        || stem.ends_with("_generated")
        || stem.ends_with("-lock")
        || stem.ends_with(".lock")
        || ext == "map"
        || ext == "lock"
    {
        return None;
    }
    if SOURCE_TEXT_LANGUAGE_EXTENSIONS.contains(&ext)
        || SOURCE_TEXT_DECLARATIVE_EXTENSIONS.contains(&ext)
    {
        return Some(Class::SourceText);
    }
    if !SOURCE_TEXT_FLAT_EXTENSIONS.contains(&ext) {
        return None;
    }
    // A legal text under a spelling `classify_plaintext`'s exhaustive
    // list misses (`MIT-LICENSE.txt`, `LICENSE-THIRD-PARTY.txt`) is
    // still a license and must be priced as one — at fallback pricing
    // its head slice displaces real code (middleclass −0.008).
    // `*-header` is the one common non-license `license` name: the
    // boilerplate a project prepends to its own sources.
    if (stem.contains("license") || stem.contains("licence") || stem == "copying")
        && !stem.contains("header")
    {
        return Some(Class::License);
    }
    Some(Class::SourceProse)
}

/// Line classes inside a declaration surface. The three get separate
/// selection budgets so a file's declarations survive a long import
/// block or a long comment banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SurfaceLine {
    Import,
    Comment,
    Decl,
}

/// Prefixes of a module/dependency reference in any of the languages
/// the fallback covers. `package` / `namespace` are
/// deliberately absent — they name the unit rather than its
/// dependencies, and are the single most informative line in a Java
/// or C# file, so they rank as declarations.
const SOURCE_TEXT_IMPORT_PREFIXES: &[&str] = &[
    "import",
    "#import",
    "#include",
    "#pragma",
    "using ",
    "require",
    "from ",
    "use ",
    "@use",
    "@import",
    "@forward",
    "open ",
    "extern crate",
    "include ",
    "load(",
    "export * from",
    "export {",
];

/// Substrings that mark a comment line as boilerplate rather than
/// content: legal banners, and pragmas addressed to a compiler or
/// linter. Both open files across whole ecosystems — v0.1's
/// head-slice fallback rendered the license banner and nothing else
/// for most Java, C++ and Swift files, and `# frozen_string_literal:
/// true` is the first line of essentially every modern Ruby file.
const SOURCE_TEXT_BOILERPLATE_MARKERS: &[&str] = &[
    "copyright",
    "spdx-",
    "all rights reserved",
    "licensed under",
    "license at",
    "license, version",
    "permission is hereby granted",
    "warranties",
    "frozen_string_literal",
    "-*-",
    "vim:",
    "coding:",
    "eslint-disable",
    "prettier-ignore",
    "stylelint-disable",
    "clang-format",
    "shellcheck",
    "@ts-nocheck",
    "noqa",
    "type: ignore",
];

/// Classify one trimmed surface line. `None` drops it.
fn classify_surface_line(trimmed: &str) -> Option<SurfaceLine> {
    if is_block_closer(trimmed) || trimmed.starts_with("#!") {
        return None;
    }
    let lower = trimmed.to_ascii_lowercase();
    if SOURCE_TEXT_IMPORT_PREFIXES
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        return Some(SurfaceLine::Import);
    }
    if !is_comment_line(trimmed) {
        return Some(SurfaceLine::Decl);
    }
    if SOURCE_TEXT_BOILERPLATE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
        || comment_text_is_empty(trimmed)
    {
        return None;
    }
    Some(SurfaceLine::Comment)
}

/// A line that only closes a block carries no information the opening
/// line didn't already give.
fn is_block_closer(trimmed: &str) -> bool {
    matches!(trimmed, "end" | "fi" | "done" | "esac" | "#endif" | "*/")
        || trimmed.chars().all(|c| "}])>;,`".contains(c))
}

/// Comment-opener detection across the covered languages. Ambiguous
/// markers require a following space (or end of line) so CSS `#id {`
/// and C `#include` are not read as comments.
fn is_comment_line(trimmed: &str) -> bool {
    for marker in ["//", "/*", "<!--", "\"\"\"", "'''"] {
        if trimmed.starts_with(marker) {
            return true;
        }
    }
    for marker in ["#", "*", "--", ";", "%", "..", "@rem", "rem "] {
        if let Some(rest) = trimmed.strip_prefix(marker)
            && (rest.is_empty()
                || rest.starts_with(char::is_whitespace)
                || rest.starts_with(marker))
        {
            return true;
        }
    }
    false
}

/// True when a comment line carries no words — `/*`, `//`, `# ---`,
/// `****`. These are block punctuation, not content.
fn comment_text_is_empty(trimmed: &str) -> bool {
    !trimmed
        .chars()
        .any(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

/// Number of leading lines occupied by a legal/pragma banner — the
/// license header that opens most Java, C++, Swift and Go-adjacent
/// source files. Zero when the file's opening comment block carries
/// no boilerplate marker, so a genuine file-purpose comment survives.
///
/// Whole-block, not per-line: a marker matches "Copyright (c) 2014"
/// but not the eight continuation lines of the same Apache header,
/// and admitting those is exactly the failure v0.1's head slice had.
fn boilerplate_banner_end(source: &str) -> usize {
    let mut block: Vec<&str> = Vec::new();
    let mut reached_content = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#!") {
            block.push(trimmed);
            continue;
        }
        if !is_comment_line(trimmed) {
            reached_content = true;
            break;
        }
        block.push(trimmed);
    }
    // A file that is comments all the way down is a comment-formatted
    // document, not a banner followed by code; skipping it would leave
    // nothing to render.
    if !reached_content {
        return 0;
    }
    let is_banner = block.iter().any(|line| {
        let lower = line.to_ascii_lowercase();
        SOURCE_TEXT_BOILERPLATE_MARKERS
            .iter()
            .any(|marker| lower.contains(marker))
    });
    if is_banner { block.len() } else { 0 }
}

/// The file's **declaration surface**: the lines at the shallowest
/// indentation levels that together yield a non-trivial declaration
/// roster, capped per line class.
///
/// Indentation is the one structural signal every text format shares,
/// and the shallowest level is where a file's declarations live — in
/// brace languages and indentation languages alike. Column zero alone
/// answers Java, C#, Swift, PHP, CSS and single-file-component web
/// frameworks; in a file with no nesting at all (prose, reST, plain
/// text) every line qualifies, so the same rule degrades to a head
/// slice, which is the right answer for those.
///
/// Descent is what makes it work for the rest. A Ruby file wraps
/// everything in `module Foo`, a C++ header in a `namespace`, a
/// Kotlin file in an `object` — column zero there is one line that is
/// the same in every file of the project. So levels are added,
/// shallowest first, until the roster has
/// [`SOURCE_TEXT_MIN_DECLS`] declarations or
/// [`SOURCE_TEXT_MAX_INDENT_LEVELS`] levels have been consumed.
/// Files that already declare at column zero never descend, so this
/// costs them nothing.
///
/// The result is a *surface*, not a summary: no parse, no signature
/// reconstruction, no bodies. It is priced accordingly in
/// [`class_value`] — strictly below every parsed language walker's
/// declaration roster, and strictly above the nothing that a file in
/// an unsupported language renders as today.
fn declaration_surface(source: &str) -> Vec<usize> {
    let banner_end = boilerplate_banner_end(source);
    let mut rows: Vec<(usize, usize, SurfaceLine)> = Vec::new();
    for (index, line) in source.lines().enumerate().skip(banner_end) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.chars().count() > SOURCE_TEXT_MAX_LINE_CHARS {
            continue;
        }
        let Some(class) = classify_surface_line(trimmed) else {
            continue;
        };
        let indent = line.len() - line.trim_start().len();
        rows.push((indent, index + 1, class));
    }

    let mut levels: Vec<usize> = rows.iter().map(|(indent, ..)| *indent).collect();
    levels.sort_unstable();
    levels.dedup();

    let caps = [
        SOURCE_TEXT_IMPORT_LINES,
        SOURCE_TEXT_COMMENT_LINES,
        SOURCE_TEXT_DECL_LINES,
    ];
    let mut used = [0usize; 3];
    let mut selected: Vec<usize> = Vec::new();
    for level in levels.into_iter().take(SOURCE_TEXT_MAX_INDENT_LEVELS) {
        for &(_, line, class) in rows.iter().filter(|(indent, ..)| *indent == level) {
            let slot = match class {
                SurfaceLine::Import => 0,
                SurfaceLine::Comment => 1,
                SurfaceLine::Decl => 2,
            };
            if used[slot] == caps[slot] {
                continue;
            }
            used[slot] += 1;
            selected.push(line);
        }
        if used[2] >= SOURCE_TEXT_MIN_DECLS || used == caps {
            break;
        }
    }
    selected.sort_unstable();
    selected
}

/// Declaration-surface content for one fallback file, or `None` when
/// the file is unreadable, machine-generated, or has no surface.
fn source_text_content(file: &Path, ctx: &WalkCtx) -> Option<crate::content::BatchContent> {
    let source = gated_read_source(file, ctx, SOURCE_TEXT_BYTE_GATE)?;
    if is_machine_generated_text(&source) {
        return None;
    }
    let selected = declaration_surface(&source);
    if selected.is_empty() {
        return None;
    }
    // Full lines only — the renderer synthesizes a `…` row for every
    // elided non-blank gap from the anchor set on its own.
    single_file_lines_content(file, &source, FileLines::new(selected))
}

/// True for text that is machine-emitted rather than hand-authored:
/// a NUL byte (no text file has one), lines too long to have been
/// wrapped by a human, or a generator banner near the top. Applied
/// after the read because none of it is visible from the filename.
fn is_machine_generated_text(source: &str) -> bool {
    if source
        .as_bytes()
        .iter()
        .take(SOURCE_TEXT_NUL_SCAN_BYTES)
        .any(|byte| *byte == 0)
    {
        return true;
    }
    let line_count = source.lines().count();
    if line_count == 0 || source.len() / line_count > SOURCE_TEXT_MAX_MEAN_LINE_BYTES {
        return true;
    }
    source
        .lines()
        .take(SOURCE_TEXT_GENERATED_SCAN_LINES)
        .any(|line| {
            let lower = line.to_ascii_lowercase();
            lower.contains("@generated")
                || lower.contains("code generated by")
                || lower.contains("do not edit")
                || lower.contains("automatically generated")
                || lower.contains("auto-generated")
                || lower.contains("autogenerated")
        })
}

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let entries = list_dir(dir, ctx.dir_filter());
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
        // A named class whose location gate rejects it (a nested
        // `Makefile`, a `.sh` outside a build-script location) falls
        // through to the fallback rather than out of the output.
        let named = classify_plaintext(&name).filter(|class| match class {
            Class::BuildEntrypoint => dir == ctx.root(),
            Class::BuildScript => is_build_script_location(&file, dir, ctx),
            _ => true,
        });
        // Root `README.rst` belongs to the markdown walker; emitting
        // a second slice of it would overlap its spans.
        let owned_by_markdown = dir == ctx.root() && super::markdown::is_readme_rst(&file);
        let Some(class) = named.or_else(|| {
            (!owned_by_markdown)
                .then(|| classify_source_text(&name))
                .flatten()
        }) else {
            continue;
        };
        if matches!(class, Class::SourceText | Class::SourceProse) {
            if let Some(content) = source_text_content(&file, ctx) {
                out.push(Batch {
                    key: PlaintextKey::DeclSurface { file: file.clone() }.into(),
                    predecessor: None,
                    content,
                    value: class_value(class, &file, ctx),
                });
            }
            continue;
        }
        // Head-sampled, not gated: a file one line over the cap used
        // to render as nothing at all, which is strictly worse than
        // the same file's first `PLAINTEXT_LINE_CAP` lines.
        let content = match class {
            Class::Requirements => {
                head_sampled_content(&file, ctx, PLAINTEXT_BYTE_GATE, REQUIREMENTS_HEAD_LINE_CAP)
            }
            Class::DotenvSample => {
                head_sampled_content(&file, ctx, DOTENV_BYTE_GATE, DOTENV_MANDATORY_HEAD_LINES)
            }
            // Build entrypoints get headroom over the generic cap:
            // real app Dockerfiles / Makefiles routinely run 60–100
            // lines and are exactly the ops surface NS authors anchor
            // on (audiobookshelf 73, linkwarden 70). Gated rather than
            // head-sampled: a Makefile's first 100 lines are usually
            // variable preamble, so a partial head is not the same
            // artifact as the build surface.
            Class::BuildEntrypoint => gated_whole_file_content(
                &file,
                ctx,
                BUILD_ENTRYPOINT_BYTE_GATE,
                BUILD_ENTRYPOINT_LINE_CAP,
            ),
            _ => head_sampled_content(&file, ctx, PLAINTEXT_BYTE_GATE, PLAINTEXT_LINE_CAP),
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
        Class::BuildEntrypoint => (0.70, 0.55, 0.60),
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
        // Unparsed declaration surface. Priced below every parsed
        // walker's names/decl roster (Python's is cat 0.4–0.65, C's
        // and Rust's higher) because a column-zero line slice is a
        // weaker claim about a file than a reconstructed declaration
        // list: it should lose the `value/cost^k` race to any walker
        // that actually understands the file, and win against the
        // bare filename that is the only alternative.
        Class::SourceText => (0.60, 0.55, 0.50),
        // Head slice of a prose/config file. Near the license floor:
        // it carries no declaration semantics at all, and unlike the
        // named plaintext classes above nobody chose this file — it
        // is whatever `.txt`/`.rst`/`.ini` happened to be in the tree.
        Class::SourceProse => (0.22, 0.30, 0.25),
    };
    mix_signals(cat, fu, ztu, path_depth_factor(file, ctx))
        * small_build_file_factor(class, file, ctx)
}

/// Conventional names for the file that says how a project is built
/// and run. Narrow on purpose, in two directions:
///
/// - deploy / CI / linter config describes the contributor's
///   toolchain rather than the project, and boosting that is a
///   measured-dead class — so this is not "config at the root";
/// - it also excludes the sibling classes the promotion reaches
///   without the name check: `.gitmodules` (submodule config) and the
///   `release.sh` / `test.sh` scripts that share `Class::BuildScript`
///   with `build.sh` but are not the build surface.
///
/// Extending the list to the names that price at the `SourceProse`
/// fallback instead (`CMakeLists.txt`, `*.mk`, `meson.build`,
/// `justfile`) requires lifting their class too, and that was measured
/// on the full corpus: net −0.0003 at Score(3000), and −0.055 on
/// microbootstrap when it reached an already-`SourceText` `Justfile`.
///
/// `Taskfile.yaml` is here because the promotion is what makes
/// claiming it worth anything: emitting it alone left the corpus mean
/// exactly flat (bubbletea's landed at cum 3677 against an NS position
/// of 2134), and the promotion is what pulls it inside the budget.
fn is_build_file_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "makefile" | "taskfile.yaml" | "taskfile.yml" | "build.sh" | "configure.ac"
    )
}

/// Mild promotion for a small build file at the repository root. A
/// compact root `Makefile` / `build.sh` is the answer to "how do I
/// build and run this", which NS authors buy in the first screenful —
/// ahead of most of the source it builds — while the class's own
/// preset prices it as one config file among many and it loses the
/// `value/cost^k` race to source. Bounded on both axes so the
/// promotion cannot reach a build *system*: past
/// [`SMALL_BUILD_FILE_LINE_CAP`] the file is reference material, and
/// past [`SMALL_BUILD_FILE_MAX_DEPTH`] it is one component's build
/// step rather than the project's.
fn small_build_file_factor(class: Class, file: &Path, ctx: &WalkCtx) -> f64 {
    if matches!(class, Class::BuildEntrypoint | Class::BuildScript)
        && file
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(is_build_file_name)
        && ctx.depth_from_root(file) <= SMALL_BUILD_FILE_MAX_DEPTH
        && is_small_build_file(file, ctx)
    {
        SMALL_BUILD_FILE_PROMOTION
    } else {
        1.0
    }
}

/// Line count behind the same FS-metadata pre-flight the content gates
/// use, so an oversized file is never read just to measure it. The
/// read itself is cached, so this costs nothing beyond the classified
/// file's own extraction.
fn is_small_build_file(file: &Path, ctx: &WalkCtx) -> bool {
    gated_read_source(file, ctx, SMALL_BUILD_FILE_LINE_CAP * 80)
        .is_some_and(|source| source.lines().count() <= SMALL_BUILD_FILE_LINE_CAP)
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

    /// Line numbers a surface selects, for readable assertions.
    fn surface_of(source: &str) -> Vec<usize> {
        declaration_surface(source)
    }

    #[test]
    fn plaintext_source_text_surface_is_the_declaration_line_in_brace_languages() {
        // Java: license banner dropped whole, `package` kept as a
        // declaration, imports damped, class declaration reached.
        let java = "/*\n * Copyright 2008 Google LLC\n *\n * Licensed under the Apache License.\n */\n\
                    package com.google.gson;\n\n\
                    import java.util.List;\nimport java.util.Map;\nimport java.util.Set;\n\
                    import java.util.Deque;\nimport java.util.Queue;\nimport java.util.Objects;\n\n\
                    public final class Gson {\n  private final List<X> factories;\n\
                    \n  public String toJson(Object src) {\n    return \"\";\n  }\n}\n";
        let selected = surface_of(java);
        let lines: Vec<&str> = java.lines().collect();
        let text: Vec<&str> = selected.iter().map(|n| lines[n - 1]).collect();
        assert!(
            text.iter().all(|line| !line.contains("Copyright")
                && !line.contains("Licensed")
                && *line != "/*"),
            "banner leaked: {text:?}"
        );
        assert!(text.contains(&"package com.google.gson;"), "{text:?}");
        assert!(text.contains(&"public final class Gson {"), "{text:?}");
        assert_eq!(
            text.iter()
                .filter(|line| line.starts_with("import"))
                .count(),
            SOURCE_TEXT_IMPORT_LINES,
            "imports not damped: {text:?}"
        );
    }

    #[test]
    fn plaintext_source_text_surface_descends_past_a_single_wrapper() {
        // Ruby wraps everything in `module`; column zero alone is one
        // line that is identical across the whole project.
        let ruby = "# frozen_string_literal: true\n\nmodule Devise\n  class Mapping\n\
                        def self.find_scope!(obj)\n      obj\n    end\n\
                    \n    def initialize(name)\n      @name = name\n    end\n  end\nend\n";
        let lines: Vec<&str> = ruby.lines().collect();
        let text: Vec<&str> = surface_of(ruby).iter().map(|n| lines[n - 1]).collect();
        assert!(text.contains(&"module Devise"), "{text:?}");
        assert!(text.contains(&"  class Mapping"), "{text:?}");
        assert!(
            text.iter()
                .any(|line| line.contains("def self.find_scope!")),
            "descent stopped too early: {text:?}"
        );
        assert!(
            !text
                .iter()
                .any(|line| line.contains("frozen_string_literal")),
            "tooling pragma leaked: {text:?}"
        );
    }

    #[test]
    fn plaintext_source_text_surface_never_descends_when_column_zero_declares() {
        // CSS selectors sit at column zero, so the surface stays there
        // and never picks up property lines from inside a rule.
        let css = "a {\n  color: red;\n}\n\nh1,\nh2 {\n  margin: 0;\n}\n\n\
                   .card {\n  padding: 1rem;\n}\n\n#main {\n  display: flex;\n}\n";
        let lines: Vec<&str> = css.lines().collect();
        let text: Vec<&str> = surface_of(css).iter().map(|n| lines[n - 1]).collect();
        assert!(
            text.iter().all(|line| !line.starts_with(' ')),
            "descended into rule bodies: {text:?}"
        );
        assert!(
            text.contains(&"#main {"),
            "`#main` read as a comment: {text:?}"
        );
    }

    #[test]
    fn plaintext_source_text_flat_file_degrades_to_a_head_slice() {
        let prose = (1..=40)
            .map(|n| format!("Paragraph line {n}."))
            .collect::<Vec<_>>()
            .join("\n");
        let selected = surface_of(&prose);
        assert_eq!(selected, (1..=SOURCE_TEXT_DECL_LINES).collect::<Vec<_>>());
    }

    #[test]
    fn plaintext_source_text_classification_rejects_derived_and_credential_files() {
        assert_eq!(classify_source_text("Gson.java"), Some(Class::SourceText));
        assert_eq!(
            classify_source_text("Session.swift"),
            Some(Class::SourceText)
        );
        assert_eq!(classify_source_text("Layout.vue"), Some(Class::SourceText));
        assert_eq!(classify_source_text("Gemfile"), Some(Class::SourceText));
        assert_eq!(
            classify_source_text("schema.proto"),
            Some(Class::SourceText)
        );
        assert_eq!(classify_source_text("guide.rst"), Some(Class::SourceProse));
        assert_eq!(classify_source_text("app.css"), Some(Class::SourceProse));
        assert_eq!(classify_source_text("build.sh"), Some(Class::SourceProse));
        // Derived artifacts and credentials never render.
        assert_eq!(classify_source_text("app.min.css"), None);
        assert_eq!(classify_source_text("vendor.bundle.css"), None);
        assert_eq!(classify_source_text("main.js.map"), None);
        assert_eq!(classify_source_text("pnpm-lock.yaml"), None);
        assert_eq!(classify_source_text("secrets.sh"), None);
        assert_eq!(classify_source_text("credentials.txt"), None);
        // Legal texts are licenses, not prose — value-floored.
        assert_eq!(
            classify_source_text("MIT-LICENSE.txt"),
            Some(Class::License)
        );
        assert_eq!(
            classify_source_text("LICENSE-THIRD-PARTY.txt"),
            Some(Class::License)
        );
        assert_eq!(
            classify_source_text("license-header.txt"),
            Some(Class::SourceProse)
        );
        // `.md`/`.rst` legal texts stay with the markdown walker.
        assert_eq!(classify_source_text("LICENSE.md"), None);
        // Formats an owning walker already claims stay with it.
        for owned in [
            "lib.rs",
            "main.py",
            "app.ts",
            "go.mod",
            "index.html",
            "pom.xml",
        ] {
            assert_eq!(classify_source_text(owned), None, "{owned}");
        }
    }

    #[test]
    fn plaintext_source_text_rejects_machine_generated_text() {
        assert!(is_machine_generated_text(
            "// Code generated by protoc.\nx\n"
        ));
        assert!(is_machine_generated_text("/* @generated */\nx\n"));
        assert!(is_machine_generated_text("a\0b\n"));
        // One very long line: a minified bundle, not hand-wrapped text.
        assert!(is_machine_generated_text(&"x".repeat(4096)));
        assert!(!is_machine_generated_text("class Foo {\n  int x;\n}\n"));
    }

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
            ("Taskfile.yaml", Some(Class::BuildEntrypoint)),
            ("Taskfile.yml", Some(Class::BuildEntrypoint)),
            ("Dockerfile", Some(Class::BuildEntrypoint)),
            ("Containerfile", Some(Class::BuildEntrypoint)),
            ("testall.sh", Some(Class::BuildScript)),
            ("build.sh", Some(Class::BuildScript)),
            (".gitmodules", Some(Class::BuildScript)),
            ("configure.ac", Some(Class::BuildScript)),
            ("setup.cfg", Some(Class::PackageConfig)),
            ("requirements.txt", Some(Class::Requirements)),
            ("Requirements.txt", Some(Class::Requirements)),
            ("requirements-dev.txt", None),
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

    /// The root build-file promotion fires on the build classes only,
    /// and each of its three gates (name, depth, size) can veto it.
    #[test]
    fn plaintext_small_build_file_factor_gates() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("packages/api/scripts")).unwrap();
        let short = "all:\n\tcc -o app main.c\n";
        for path in [
            "Makefile",
            "build.sh",
            "release.sh",
            ".gitmodules",
            "packages/api/scripts/build.sh",
        ] {
            std::fs::write(root.join(path), short).unwrap();
        }
        // Over the line cap: a build *system*, not a build surface.
        std::fs::write(
            root.join("configure.ac"),
            "AC_CHECK_HEADERS([x.h])\n".repeat(SMALL_BUILD_FILE_LINE_CAP + 1),
        )
        .unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());

        let factor = |path: &str, class| small_build_file_factor(class, &root.join(path), &ctx);
        assert_eq!(
            factor("Makefile", Class::BuildEntrypoint),
            SMALL_BUILD_FILE_PROMOTION
        );
        assert_eq!(
            factor("build.sh", Class::BuildScript),
            SMALL_BUILD_FILE_PROMOTION
        );
        // Name gate: BuildScript siblings that are not the build surface.
        assert_eq!(factor("release.sh", Class::BuildScript), 1.0);
        assert_eq!(factor(".gitmodules", Class::BuildScript), 1.0);
        // Depth gate: a nested `scripts/` dir still classifies as
        // BuildScript, but is one component's build step.
        assert_eq!(
            factor("packages/api/scripts/build.sh", Class::BuildScript),
            1.0
        );
        // Size gate.
        assert_eq!(factor("configure.ac", Class::BuildScript), 1.0);
        // Class gate: the fallback tiers never receive the promotion.
        assert_eq!(factor("build.sh", Class::SourceProse), 1.0);
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
    fn plaintext_oversized_makefile_is_suppressed_entirely() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let file = root.join("Makefile");
        let mut source = String::from("BUILD_DEPS = common-a common-b\n\nbuild: $(BUILD_DEPS)\n");
        for index in 0..60 {
            source.push_str(&format!(
                "target-{index:02}: dep-{index:02}-a dep-{index:02}-b\n\tRECIPE_{index:02}\n"
            ));
        }
        std::fs::write(&file, &source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        assert!(expand_in_dir(root, &ctx).is_empty());
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

    /// A file whose byte count slips under the gate but whose line
    /// count exceeds `PLAINTEXT_LINE_CAP` renders its first
    /// `PLAINTEXT_LINE_CAP` lines. The cap bounds what is *rendered*,
    /// not whether the file is reachable at all — dropping it made
    /// "one line too long" mean "renders as nothing".
    #[test]
    fn plaintext_too_many_lines_head_sampled_not_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let body: String = (0..(PLAINTEXT_LINE_CAP + 5))
            .map(|i| format!("line {i}\n"))
            .collect();
        std::fs::write(root.join("LICENSE"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        assert_has_plaintext_whole(&report, "LICENSE");
        let rendered = report.tree.render();
        assert!(rendered.contains("line 0"), "{rendered}");
        assert!(
            rendered.contains(&format!("line {}", PLAINTEXT_LINE_CAP - 1)),
            "{rendered}"
        );
        assert!(
            !rendered.contains(&format!("line {PLAINTEXT_LINE_CAP}")),
            "rendered past the cap: {rendered}"
        );
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
