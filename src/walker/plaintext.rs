//! Plaintext walker — the home for every file precis has no parser
//! for. Two jobs:
//!
//! 1. **Named files** ([`classify_plaintext`]): build files, dotenv
//!    samples, version pins and the pnpm workspace, each rendered whole or as
//!    a head slice at one of two value tiers.
//!    Credential-bearing names (`.env`, `.npmrc`, `secrets.sh`) are
//!    never admitted; dotenv *samples* are, since they carry
//!    placeholders and document the deploy-facing config keys.
//! 2. **Every other source-like text file** ([`Class::LanguageSource`],
//!    [`Class::FlatText`]): the language-agnostic fallback for formats no
//!    parser claims (Java, C++, Ruby, PHP, Swift, Kotlin, C#, Vue, CSS,
//!    shell, …), rendered as its [`declaration_surface`] and, when short,
//!    whole behind it. Without it those files show only as a filename.
//!    Markup documents (reST, AsciiDoc, …) are not claimed: like
//!    markdown beyond the root README, they are left to the listing.
//!
//! In a single-file walk it also renders whatever of the named file the
//! other walkers leave unshown ([`named_file_rest`]).

use std::collections::HashSet;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::batch::{Batch, BatchKey, PlaintextKey};
use crate::fs_util::list_dir;
use crate::render::Source;

use super::{
    WalkCtx, gated_read_source, gated_whole_file_content, path_depth_factor,
    single_file_lines_content,
};

/// Line cap on a `Whole` plaintext batch.
const PLAINTEXT_LINE_CAP: usize = 60;

/// FS-metadata pre-flight gate (≈80 bytes/line × line cap).
const PLAINTEXT_BYTE_GATE: usize = PLAINTEXT_LINE_CAP * 80;

/// Line cap on a whole [`Class::Build`] batch.
const BUILD_LINE_CAP: usize = 100;

/// FS-metadata pre-flight gate for build files (same ≈80
/// bytes/line multiplier as [`PLAINTEXT_BYTE_GATE`]).
const BUILD_BYTE_GATE: usize = BUILD_LINE_CAP * 80;

/// Promotion for a small root build file — see
/// [`small_build_file_factor`]. Set inside a flat plateau of a sweep;
/// see `git show 6a5c9887`.
const SMALL_BUILD_FILE_PROMOTION: f64 = 2.0;

/// Rows in the dotenv head batch — samples lead with the
/// mandatory-settings block by convention.
const DOTENV_MANDATORY_HEAD_LINES: usize = 12;

/// Selection budget for a [`Class::LanguageSource`] declaration surface,
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
/// stray data blob is never read. Also how far into a named file
/// [`named_file_rest`] reaches, so a multi-megabyte file is not
/// tokenized whole for a budget that shows its head.
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

/// Bytes scanned by [`has_nul_byte`].
const SOURCE_TEXT_NUL_SCAN_BYTES: usize = 8192;

/// Pre-flight byte gate for dotenv samples — generous (only the head
/// renders) but bounded.
const DOTENV_BYTE_GATE: usize = 64 * 1024;

/// Plaintext file class — drives the (filename → signal preset) table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Class {
    /// Toolchain facts a reader needs before running anything: runtime
    /// version pins and the pnpm workspace's package globs. Ignore
    /// lists, editor / lint / format config and CI or hook YAML are left
    /// to the listing.
    Tooling,
    /// Compact build/deploy entrypoints (`Makefile`, `Taskfile`,
    /// `Dockerfile`, compose files).
    Build,
    /// Checked-in dotenv sample/template (`.env.sample`) — the
    /// deploy-facing config-key documentation, head-sampled when long.
    DotenvSample,
    /// A source file in a language no walker parses — the
    /// language-agnostic fallback. Rendered as a declaration surface.
    LanguageSource,
    /// A flat-config, script or plain-text file with no owning walker. Same
    /// extraction (a file with no nesting has every line at
    /// indentation zero, so the surface *is* its head slice), but a
    /// head slice claims much less than a declaration roster does and
    /// is priced for it.
    FlatText,
}

/// Classify a file by name. `None` for files the walker doesn't own
/// (other walkers' formats, out-of-scope variants, credential names).
pub(crate) fn classify_plaintext(name: &str) -> Option<Class> {
    let lower = name.to_ascii_lowercase();
    match name {
        ".nvmrc" | ".python-version" | ".tool-versions" | "pnpm-workspace.yaml" => {
            return Some(Class::Tooling);
        }
        // `Taskfile.yaml` is a `Makefile` in YAML clothing and a `justfile` one
        // in its own syntax — the task runner's target roster.
        "Makefile" | "Taskfile.yaml" | "Taskfile.yml" | "justfile" | "Justfile" | ".justfile"
        | "Dockerfile" | "Containerfile" => {
            return Some(Class::Build);
        }
        _ => {}
    }
    if is_docker_compose_name(&lower) {
        return Some(Class::Build);
    }
    if crate::value::is_dotenv_sample_filename(name) {
        return Some(Class::DotenvSample);
    }
    None
}

/// True iff `lower` has the exact `docker-compose` / `compose` YAML stem,
/// or adds an environment variant separated by `.` / `-`.
fn is_docker_compose_name(lower: &str) -> bool {
    let Some(stem) = lower
        .strip_suffix(".yaml")
        .or_else(|| lower.strip_suffix(".yml"))
    else {
        return false;
    };
    ["docker-compose", "compose"].into_iter().any(|base| {
        stem.strip_prefix(base).is_some_and(|suffix| {
            suffix.is_empty()
                || suffix
                    .strip_prefix(['.', '-'])
                    .is_some_and(|variant| !variant.is_empty())
        })
    })
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
    "swift", "zig", "dart", "nim", "cr", "hs", "lhs", "ml", "mli", "elm", "erl", "hrl", "ex", "exs",
    "pas", "pp", "dpr", "lpr", "f", "f90", "f95", "f03", "f08", "for", "cob", "cbl", "cpy", "adb",
    "ads", "sol", // hardware description
    "v", "sv", "svh", "vhd", "vhdl", // scripting
    "rb", "php", "pl", "pm", "r", "jl", "tcl", "pyi", "vim", "gd", "coffee", "el", "lisp", "scm",
    "rkt", // TeX classes and packages (macro libraries, not documents)
    "cls", "sty", // component-file web frameworks
    "vue", "svelte", "astro",
];

/// Contract-bearing extensions whose indentation-zero lines are real
/// declarations — protobuf messages, GraphQL types, Terraform blocks,
/// Gradle plugin and dependency blocks — but whose *directories* are
/// schema or config trees rather than source packages. Same surface
/// value as a language file, no source-inventory promotion: promoting
/// them buys stacks of asset-tree listings.
const SOURCE_TEXT_DECLARATIVE_EXTENSIONS: &[&str] = &[
    "proto", "thrift", "graphql", "gql", "capnp", "fbs", "tf", "tfvars", "hcl", "nix", "dhall",
    "cue", "gradle", "gemspec", "podspec", "rake",
];

/// Extensions with no *program* structure to surface. `.bat`/`.cmd`
/// are absent on purpose — in practice they are generated wrappers
/// (`gradlew.bat`, `mvnw.cmd`), 158 tokens of argument marshalling.
/// Two shapes, one price:
///
/// - plain text, flat config, shell scripts and build glue are sequences
///   of statements, so every line sits at indentation zero and the
///   "surface" is just a head slice;
/// - stylesheets do have declarations at indentation zero, but a
///   selector list describes presentation, not what the program is
///   or does — nine 10-token stylesheet slices displacing a
///   package's parsed decl surfaces is a bad trade.
///
/// Claimed either way — a slice beats a bare filename — but priced
/// near the floor, and never a source inventory.
const SOURCE_TEXT_FLAT_EXTENSIONS: &[&str] = &[
    "txt",
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
        return Some(Class::LanguageSource);
    }
    let (stem, ext) = lower.rsplit_once('.')?;
    if is_credential_stem(stem) {
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
        return Some(Class::LanguageSource);
    }
    SOURCE_TEXT_FLAT_EXTENSIONS
        .contains(&ext)
        .then_some(Class::FlatText)
}

/// Build-tool wrapper scripts, written by `gradle wrapper` and
/// `mvn wrapper:wrapper` and the same in every project that has one.
const GENERATED_WRAPPER_SCRIPTS: &[&str] = &["gradlew", "mvnw"];

/// Whether `file` starts with `#!`: an extensionless script (`bin/deploy`,
/// a tool shipped as one executable) names its language on its first line
/// rather than in its name.
fn opens_with_shebang(file: &Path) -> bool {
    let mut head = [0; 2];
    std::fs::File::open(file).is_ok_and(|mut opened| {
        std::io::Read::read_exact(&mut opened, &mut head).is_ok() && &head == b"#!"
    })
}

/// A license text (`LICENSE`, `COPYING.txt`, `MIT-LICENSE.txt`,
/// `LICENSE-APACHE`, `LICENSE.md`). No walker renders one: the listing names
/// the file and the manifest's `license` field names the license, and the
/// text says nothing about the code. A qualifier may follow the legal word
/// only in an all-caps name, so `license-api.md` or `license-server.md` —
/// documentation of a product's own licensing — still renders. `*-header`
/// is the boilerplate a project prepends to its sources, not a license.
pub(crate) fn is_license_file_name(name: &str) -> bool {
    let stem = match name.rsplit_once('.') {
        None => name,
        Some((stem, ext)) if matches!(ext.to_ascii_lowercase().as_str(), "txt" | "md" | "rst") => {
            stem
        }
        Some(_) => return false,
    };
    let lower = stem.to_ascii_lowercase();
    let is_legal_word = |word: &str| {
        matches!(
            word,
            "license" | "licence" | "licenses" | "copying" | "notice"
        )
    };
    let first = lower.split(['-', '_']).next().unwrap_or_default();
    let last = lower.rsplit(['-', '_']).next().unwrap_or_default();
    (is_legal_word(last) || (is_legal_word(first) && !stem.chars().any(|c| c.is_ascii_lowercase())))
        && !lower.contains("header")
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

/// Classify one trimmed surface line, `in_block_comment` when it sits
/// inside a comment opened on an earlier line. `None` drops it.
fn classify_surface_line(trimmed: &str, in_block_comment: bool) -> Option<SurfaceLine> {
    if is_block_closer(trimmed) || trimmed.starts_with("#!") {
        return None;
    }
    let lower = trimmed.to_ascii_lowercase();
    if !in_block_comment
        && SOURCE_TEXT_IMPORT_PREFIXES
            .iter()
            .any(|prefix| lower.starts_with(prefix))
    {
        return Some(SurfaceLine::Import);
    }
    if !in_block_comment && !is_comment_line(trimmed) {
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

/// A line that only closes a block, or only opens one on the line after
/// its declaration, carries no information the declaration line didn't
/// already give.
fn is_block_closer(trimmed: &str) -> bool {
    matches!(trimmed, "end" | "fi" | "done" | "esac" | "#endif" | "*/")
        || trimmed.chars().all(|c| "{}])>;,`".contains(c))
}

/// Comment-opener detection across the covered languages. Ambiguous
/// markers require a following space (or end of line) so CSS `#id {`
/// and C `#include` are not read as comments.
/// `#region` / `#endregion` (C#, Visual Basic, PHP) name a fold for the
/// editor and read as comments too.
fn is_comment_line(trimmed: &str) -> bool {
    let names_fold = |marker: &str| {
        trimmed
            .get(..marker.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(marker))
            && trimmed[marker.len()..]
                .chars()
                .next()
                .is_none_or(char::is_whitespace)
    };
    if names_fold("#region") || names_fold("#endregion") {
        return true;
    }
    for marker in ["//", "/*", "<!--", "\"\"\"", "'''"] {
        if trimmed.starts_with(marker) {
            return true;
        }
    }
    for marker in ["#", "*", "--", ";", "%", "..", "\"", "@rem", "rem "] {
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
fn boilerplate_banner_end(lines: &[&str], in_block_comment: &[bool]) -> usize {
    let mut block: Vec<&str> = Vec::new();
    let mut reached_content = false;
    for (line, &in_block_comment) in lines.iter().zip(in_block_comment) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#!") {
            block.push(trimmed);
            continue;
        }
        if !in_block_comment && !is_comment_line(trimmed) {
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

/// Per line, whether it sits inside a `/* … */` comment opened on an
/// earlier line — interior lines carry no comment marker of their own.
/// Only a line that starts with `/*` opens one, so a glob in a shell
/// script (`rm build/*`) is not read as a comment.
fn block_comment_interiors(lines: &[&str]) -> Vec<bool> {
    let mut inside = false;
    lines
        .iter()
        .map(|line| {
            let trimmed = line.trim();
            let interior = inside;
            if inside {
                inside = !trimmed.contains("*/");
            } else if let Some(rest) = trimmed.strip_prefix("/*") {
                inside = !rest.contains("*/");
            }
            interior
        })
        .collect()
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
/// [`class_value`].
fn declaration_surface(source: &str, class: Class) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().collect();
    let in_block_comment = block_comment_interiors(&lines);
    let banner_end = boilerplate_banner_end(&lines, &in_block_comment);
    let mut rows: Vec<(usize, usize, SurfaceLine)> = Vec::new();
    for (index, (line, &in_block_comment)) in lines
        .iter()
        .zip(&in_block_comment)
        .enumerate()
        .skip(banner_end)
    {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.chars().count() > SOURCE_TEXT_MAX_LINE_CHARS {
            continue;
        }
        let Some(kind) = classify_surface_line(trimmed, in_block_comment) else {
            continue;
        };
        rows.push((indentation(line), index + 1, kind));
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
    // In a language file, a declaration that opens a block (a class, a
    // function, a module) is what the roster is for; statements and
    // directives take the slots it leaves. A flat file's surface stays
    // its head.
    if class == Class::LanguageSource {
        let opens_block = block_openers(&lines);
        rows.sort_by_key(|&(_, line, kind)| kind == SurfaceLine::Decl && !opens_block[line - 1]);
    }
    for level in levels.into_iter().take(SOURCE_TEXT_MAX_INDENT_LEVELS) {
        for &(_, line, kind) in rows.iter().filter(|(indent, ..)| *indent == level) {
            let slot = match kind {
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

fn indentation(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// Per line, whether the next line with content is indented deeper — the
/// line heads a block. A line of only brackets is skipped over, so an
/// Allman-style `{` does not take the heading from the line above it.
fn block_openers(lines: &[&str]) -> Vec<bool> {
    let mut opens = vec![false; lines.len()];
    let mut next_indent: Option<usize> = None;
    for (index, line) in lines.iter().enumerate().rev() {
        let indent = indentation(line);
        opens[index] = next_indent.is_some_and(|next| next > indent);
        let trimmed = line.trim();
        if !trimmed.is_empty() && !is_block_closer(trimmed) {
            next_indent = Some(indent);
        }
    }
    opens
}

/// A fallback file's declaration surface, then — when the surface
/// elides part of a file short enough to render whole — the whole file
/// behind it. Nothing when the file is unreadable, machine-generated, or
/// has no surface. Line length says nothing about prose, which is often
/// written one paragraph per line.
fn push_source_text_batches(out: &mut Vec<Batch>, file: &Path, ctx: &WalkCtx, class: Class) {
    let Some(source) = gated_read_source(file, ctx, SOURCE_TEXT_BYTE_GATE) else {
        return;
    };
    if has_generated_marker(&source)
        || (class == Class::LanguageSource && has_minified_lines(&source))
    {
        return;
    }
    let selected = declaration_surface(&source, class);
    let surface_rows = selected.len();
    let Some(content) = single_file_lines_content(file, &source, selected) else {
        return;
    };
    let surface_key: BatchKey = PlaintextKey::DeclSurface {
        file: file.to_path_buf(),
    }
    .into();
    let value = class_value(class, file, ctx);
    out.push(Batch {
        key: surface_key.clone(),
        predecessor: None,
        content,
        value,
    });
    let content_rows = source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    if surface_rows < content_rows
        && let Some(content) =
            gated_whole_file_content(file, ctx, PLAINTEXT_BYTE_GATE, PLAINTEXT_LINE_CAP)
    {
        out.push(Batch {
            key: PlaintextKey::Whole {
                file: file.to_path_buf(),
            }
            .into(),
            predecessor: Some(surface_key),
            content,
            value,
        });
    }
}

/// True for text that is machine-emitted rather than hand-authored:
/// a NUL byte (no text file has one), lines too long to have been
/// wrapped by a human, or a generator banner near the top. Applied
/// after the read because none of it is visible from the filename.
pub(in crate::walker) fn is_machine_generated_text(source: &str) -> bool {
    has_generated_marker(source) || has_minified_lines(source)
}

fn has_minified_lines(source: &str) -> bool {
    let line_count = source.lines().count();
    line_count == 0 || source.len() / line_count > SOURCE_TEXT_MAX_MEAN_LINE_BYTES
}

/// A NUL byte, which no text file contains but a binary misnamed `.txt`
/// (or a UTF-8-decodable data blob) does.
fn has_nul_byte(source: &str) -> bool {
    source
        .as_bytes()
        .iter()
        .take(SOURCE_TEXT_NUL_SCAN_BYTES)
        .any(|byte| *byte == 0)
}

/// A NUL byte or a generator banner near the top.
fn has_generated_marker(source: &str) -> bool {
    if has_nul_byte(source) {
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

pub fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let entries = list_dir(dir, ctx.dir_filter());
    let mut out = Vec::new();
    for (name, kind) in entries.iter() {
        if !matches!(kind, crate::fs_util::EntryKind::File) {
            continue;
        }
        if is_license_file_name(name) {
            continue;
        }
        let file = dir.join(name);
        let named = classify_plaintext(name);
        // Parsed languages belong to the code engine; a second slice
        // would overlap its spans.
        let owned_elsewhere = super::code::Language::from_path(&file).is_some();
        let Some(class) = named
            .or_else(|| {
                (!owned_elsewhere)
                    .then(|| classify_source_text(name))
                    .flatten()
            })
            .or_else(|| {
                (!name.contains('.')
                    && !GENERATED_WRAPPER_SCRIPTS.contains(&name.as_str())
                    && opens_with_shebang(&file))
                .then_some(Class::FlatText)
            })
        else {
            continue;
        };
        if matches!(class, Class::LanguageSource | Class::FlatText) {
            push_source_text_batches(&mut out, &file, ctx, class);
            continue;
        }
        let content = match class {
            Class::DotenvSample => {
                head_sampled_content(&file, ctx, DOTENV_BYTE_GATE, DOTENV_MANDATORY_HEAD_LINES)
            }
            // Whole, with headroom over the generic cap: a Makefile's
            // head is mostly variable preamble, not its targets.
            Class::Build => gated_whole_file_content(&file, ctx, BUILD_BYTE_GATE, BUILD_LINE_CAP),
            _ => head_sampled_content(&file, ctx, PLAINTEXT_BYTE_GATE, PLAINTEXT_LINE_CAP),
        };
        let (content, value) = match content {
            Some(content) => (
                content,
                class_value(class, &file, ctx) * small_build_file_factor(class, &file, ctx),
            ),
            None => match root_makefile_phony_targets(&file, name, ctx) {
                Some(content) => (content, class_value(class, &file, ctx)),
                None => continue,
            },
        };
        out.push(Batch {
            key: PlaintextKey::Whole { file: file.clone() }.into(),
            predecessor: None,
            content,
            value,
        });
    }
    out
}

/// In a single-file walk, the named file's rows that none of `emitted`
/// shows, worth nothing: it ranks behind every batch a walker chose and
/// spends only the budget they leave. A file that fits renders whole,
/// and a file no walker has anything to say about renders its head
/// rather than its name alone.
pub(super) fn named_file_rest(emitted: &[Batch], ctx: &WalkCtx) -> Option<Batch> {
    let file = ctx.dir_filter().named_file()?;
    let source = named_file_head(file, ctx)?;
    if has_nul_byte(&source) {
        return None;
    }
    let shown: HashSet<usize> = emitted
        .iter()
        .filter_map(|batch| match &batch.content {
            crate::content::BatchContent::Lines { spans } => Some(spans),
            crate::content::BatchContent::Fs { .. } => None,
        })
        .flatten()
        .filter(|span| span.path == file)
        .flat_map(|span| span.start..=span.end)
        .collect();
    let mut bytes_before = 0;
    let rows = (1..=source.line_count())
        .take_while(|&row| {
            bytes_before += source.line(row).map_or(0, str::len) + 1;
            bytes_before <= SOURCE_TEXT_BYTE_GATE
        })
        .filter(|row| !shown.contains(row))
        .collect();
    Some(Batch {
        key: PlaintextKey::Rest {
            file: file.to_path_buf(),
        }
        .into(),
        predecessor: None,
        content: single_file_lines_content(file, &source, rows)?,
        value: 0.0,
    })
}

/// The named file's source as far as [`named_file_rest`] can reach into
/// it: whole when a walker has already read it or it fits within
/// [`SOURCE_TEXT_BYTE_GATE`], else only its head, so a multi-gigabyte
/// log is never read whole. The head runs a few bytes past the gate, the
/// longest UTF-8 character, so the line the gate cuts stays too long to
/// select and still tells the renderer that more of the file follows.
fn named_file_head(file: &Path, ctx: &WalkCtx) -> Option<Arc<Source>> {
    if let Some(source) = ctx.source_cache().cached(file) {
        return Some(source);
    }
    let mut head = Vec::new();
    std::fs::File::open(file)
        .ok()?
        .take((SOURCE_TEXT_BYTE_GATE + 4) as u64)
        .read_to_end(&mut head)
        .ok()?;
    if head.len() > SOURCE_TEXT_BYTE_GATE {
        let last_line_start = head
            .iter()
            .rposition(|&byte| byte == b'\n')
            .map_or(0, |i| i + 1);
        match std::str::from_utf8(&head[last_line_start..]) {
            Err(error) if error.error_len().is_some() => return None,
            Err(error) => head.truncate(last_line_start + error.valid_up_to()),
            Ok(_) => {}
        }
    }
    let text = String::from_utf8(head).ok()?;
    ctx.source_cache()
        .insert(file.to_path_buf(), Arc::from(text));
    ctx.source_cache().cached(file)
}

/// Two tiers. The ops surface (how the project is built, deployed and
/// versioned) and an unparsed language's declaration surface sit at the
/// top, the latter below a parsed declaration unless it is in the
/// repository's primary language ([`is_in_primary_language`]).
/// Contributor tooling and unclassified prose / flat config sit low.
fn class_value(class: Class, file: &Path, ctx: &WalkCtx) -> f64 {
    let tier = match class {
        Class::LanguageSource if is_in_primary_language(file, ctx) => {
            crate::value::code_rung_value(crate::batch::Rung::Decl)
        }
        Class::Build | Class::DotenvSample | Class::LanguageSource => 905.0,
        Class::Tooling | Class::FlatText => 488.0,
    };
    tier * path_depth_factor(file, ctx)
}

/// Whether `file` is in the language the repository is written in. When
/// no walker parses that language, its declaration surface is the best
/// account of the repository's core there is, and prices like a parsed
/// declaration so side clients in parsed languages do not crowd it out.
fn is_in_primary_language(file: &Path, ctx: &WalkCtx) -> bool {
    ctx.primary_language()
        .is_some_and(|primary| super::language_group(file) == Some(primary))
}

/// Mild promotion for a root `Makefile` / `Taskfile` / `justfile`. A
/// compact one is the answer to "how do I build and run this", which
/// NS authors buy in the first screenful — ahead of most of the source
/// it builds — while the class's own preset prices it as one config
/// file among many and it loses the `value/cost^k` race to source.
/// The build class already renders only files of at most
/// [`BUILD_LINE_CAP`] lines; a nested one is one
/// component's build step rather than the project's.
///
/// Narrow on purpose: deploy / CI / linter config describes the
/// contributor's toolchain rather than the project.
fn small_build_file_factor(class: Class, file: &Path, ctx: &WalkCtx) -> f64 {
    let name = file
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if class == Class::Build
        && matches!(
            name,
            "Makefile" | "Taskfile.yaml" | "Taskfile.yml" | "justfile" | "Justfile" | ".justfile"
        )
        && ctx.depth_from_root(file) == 1
    {
        SMALL_BUILD_FILE_PROMOTION
    } else {
        1.0
    }
}

/// A root Makefile too long to render whole still names what it can run:
/// its `.PHONY` declarations, the author's own list of commands, each
/// through its backslash-continued lines.
fn root_makefile_phony_targets(
    file: &Path,
    name: &str,
    ctx: &WalkCtx,
) -> Option<crate::content::BatchContent> {
    if name != "Makefile" || ctx.depth_from_root(file) != 1 {
        return None;
    }
    let source = gated_read_source(file, ctx, SOURCE_TEXT_BYTE_GATE)?;
    let mut rows = Vec::new();
    let mut in_declaration = false;
    for (index, line) in source.lines().enumerate() {
        in_declaration |= line.starts_with(".PHONY");
        if in_declaration {
            rows.push(index + 1);
            in_declaration = line.ends_with('\\');
        }
    }
    single_file_lines_content(file, &source, rows)
}

/// The first `head_line_cap` rows — the whole file when it fits, so a
/// file one line over the cap renders its head rather than nothing.
fn head_sampled_content(
    file: &Path,
    ctx: &WalkCtx,
    byte_gate: usize,
    head_line_cap: usize,
) -> Option<crate::content::BatchContent> {
    let source = gated_read_source(file, ctx, byte_gate)?;
    let rows = (1..=source.line_count().min(head_line_cap)).collect();
    single_file_lines_content(file, &source, rows)
}

#[cfg(test)]
mod tests {
    use crate::batch::BatchKey;
    use crate::scheduler::Scheduler;
    use crate::walker::FsWalker;

    use super::*;

    /// Line numbers a surface selects, for readable assertions.
    fn surface_of(source: &str) -> Vec<usize> {
        declaration_surface(source, Class::LanguageSource)
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

    /// A license inside a `/* … */` whose interior lines carry no `*`, and
    /// inside an editor fold region, is still a banner.
    #[test]
    fn plaintext_source_text_surface_skips_a_folded_block_comment_banner() {
        let csharp = "#region License Information (GPL v3)\n\n/*\n    ShareX - screenshots\n\
                      Copyright (c) 2007-2026 ShareX Team\n\n    This program is free software.\n\
                      */\n\n#endregion License Information (GPL v3)\n\n\
                      using System;\n\nnamespace ShareX\n";
        let lines: Vec<&str> = csharp.lines().collect();
        let text: Vec<&str> = surface_of(csharp).iter().map(|n| lines[n - 1]).collect();
        assert_eq!(text, vec!["using System;", "namespace ShareX"]);
    }

    #[test]
    fn plaintext_source_text_region_marker_is_a_comment_but_a_css_id_is_not() {
        assert!(is_comment_line("#Region \"Fields\""));
        assert!(is_comment_line("#endregion"));
        assert!(!is_comment_line("#region-picker {"));
    }

    /// Column-zero statements ahead of a file's functions do not take the
    /// roster's slots from them, in a language file; a flat file's surface
    /// stays its head.
    #[test]
    fn plaintext_source_text_surface_prefers_lines_that_open_blocks() {
        let setup: String = (0..SOURCE_TEXT_DECL_LINES)
            .map(|n| format!("let g:setting_{n} = {n}\n"))
            .collect();
        let vim = setup + "\nfunction! plug#begin(...)\n  return 1\nendfunction\n";
        let lines: Vec<&str> = vim.lines().collect();
        let text = |class| -> Vec<&str> {
            declaration_surface(&vim, class)
                .iter()
                .map(|n| lines[n - 1])
                .collect()
        };
        assert!(
            text(Class::LanguageSource).contains(&"function! plug#begin(...)"),
            "{:?}",
            text(Class::LanguageSource)
        );
        assert!(!text(Class::FlatText).contains(&"function! plug#begin(...)"));
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
        assert_eq!(
            classify_source_text("Gson.java"),
            Some(Class::LanguageSource)
        );
        assert_eq!(
            classify_source_text("Session.swift"),
            Some(Class::LanguageSource)
        );
        assert_eq!(
            classify_source_text("Layout.vue"),
            Some(Class::LanguageSource)
        );
        assert_eq!(classify_source_text("Gemfile"), Some(Class::LanguageSource));
        assert_eq!(
            classify_source_text("schema.proto"),
            Some(Class::LanguageSource)
        );
        assert_eq!(classify_source_text("guide.rst"), None);
        assert_eq!(classify_source_text("app.css"), Some(Class::FlatText));
        assert_eq!(classify_source_text("build.sh"), Some(Class::FlatText));
        // Derived artifacts and credentials never render.
        assert_eq!(classify_source_text("app.min.css"), None);
        assert_eq!(classify_source_text("vendor.bundle.css"), None);
        assert_eq!(classify_source_text("main.js.map"), None);
        assert_eq!(classify_source_text("pnpm-lock.yaml"), None);
        assert_eq!(classify_source_text("secrets.sh"), None);
        assert_eq!(classify_source_text("credentials.txt"), None);
        // `.md` files stay with the markdown walker.
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

    /// Prose written one paragraph per line has long lines by nature;
    /// it still gets a surface (its short lines — headings, list items).
    #[test]
    fn plaintext_unwrapped_prose_is_not_machine_generated() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let paragraph = "word ".repeat(200);
        let notes = format!("Design\n\n{paragraph}\n\nOpen questions\n\n{paragraph}\n");
        std::fs::write(root.join("notes.txt"), &notes).unwrap();
        std::fs::write(root.join("Notes.java"), &notes).unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let surfaces: Vec<_> = expand_in_dir(root, &ctx)
            .into_iter()
            .filter_map(|batch| match batch.key {
                BatchKey::Plaintext(PlaintextKey::DeclSurface { file }) => Some(file),
                _ => None,
            })
            .collect();
        assert_eq!(surfaces, vec![root.join("notes.txt")]);
    }

    #[test]
    fn plaintext_classify_table_drives_predicate() {
        // (name, expected). `None` rows assert names the walker must
        // refuse — owned by other walkers, out of scope, or
        // credential-bearing.
        let cases: &[(&str, Option<Class>)] = &[
            // Dotfiles by class.
            (".gitignore", None),
            (".editorconfig", None),
            (".nvmrc", Some(Class::Tooling)),
            (".python-version", Some(Class::Tooling)),
            (".tool-versions", Some(Class::Tooling)),
            ("pnpm-workspace.yaml", Some(Class::Tooling)),
            ("Makefile", Some(Class::Build)),
            ("Taskfile.yaml", Some(Class::Build)),
            ("Taskfile.yml", Some(Class::Build)),
            ("justfile", Some(Class::Build)),
            ("Dockerfile", Some(Class::Build)),
            ("Containerfile", Some(Class::Build)),
            ("docker-compose.yml", Some(Class::Build)),
            ("Docker-Compose.YML", Some(Class::Build)),
            ("compose.override.yaml", Some(Class::Build)),
            ("compose-dev.yaml", Some(Class::Build)),
            ("composer.yml", None),
            ("compose-.yml", None),
            ("config.yaml", None),
            (".pre-commit-config.yaml", None),
            ("requirements-dev.txt", None),
            (".env.sample", Some(Class::DotenvSample)),
            (".env.example", Some(Class::DotenvSample)),
            (".env.template", Some(Class::DotenvSample)),
            (".env.dist", Some(Class::DotenvSample)),
            // Owned by other walkers.
            ("LICENSE.md", None),
            (".eslintrc.json", None),
            (".prettierrc.json", None),
            (".eslintrc.js", None),
            (".prettierrc.js", None),
            // Credential-bearing — must not be classified.
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

    /// The root build-file promotion fires on the build class only, and
    /// each of its gates (name, depth) can veto it.
    #[test]
    fn plaintext_small_build_file_factor_gates() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("packages/api")).unwrap();
        let short = "all:\n\tcc -o app main.c\n";
        for path in ["Makefile", "Dockerfile", "packages/api/Makefile"] {
            std::fs::write(root.join(path), short).unwrap();
        }
        let ctx = WalkCtx::new(root.to_path_buf());

        let factor = |path: &str, class| small_build_file_factor(class, &root.join(path), &ctx);
        assert_eq!(factor("Makefile", Class::Build), SMALL_BUILD_FILE_PROMOTION);
        // Name gate: deploy config is not the build surface.
        assert_eq!(factor("Dockerfile", Class::Build), 1.0);
        // Depth gate: a nested Makefile is one component's build step.
        assert_eq!(factor("packages/api/Makefile", Class::Build), 1.0);
        // Class gate: the fallback tiers never receive the promotion.
        assert_eq!(factor("Makefile", Class::FlatText), 1.0);
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
        let source = format!("build: {}\n", "dependency ".repeat(BUILD_BYTE_GATE));
        std::fs::write(&file, source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        assert!(expand_in_dir(root, &ctx).is_empty());
    }

    #[test]
    fn plaintext_oversized_makefile_renders_only_its_phony_targets() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir(root.join("docs")).unwrap();
        let mut source = String::from(
            "BUILD_DEPS = common-a common-b\n.PHONY: build test\n\nbuild: $(BUILD_DEPS)\n",
        );
        for index in 0..60 {
            source.push_str(&format!(
                "target-{index:02}: dep-{index:02}-a dep-{index:02}-b\n\tRECIPE_{index:02}\n"
            ));
        }
        std::fs::write(root.join("Makefile"), &source).unwrap();
        std::fs::write(root.join("docs/Makefile"), &source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        assert_eq!(batches.len(), 1);
        let crate::content::BatchContent::Lines { spans } = &batches[0].content else {
            panic!("expected a lines batch");
        };
        let rows: Vec<_> = spans.iter().map(|span| (span.start, span.end)).collect();
        assert_eq!(rows, [(2, 2)]);
        assert!(expand_in_dir(&root.join("docs"), &ctx).is_empty());
    }

    #[test]
    fn plaintext_oversized_makefile_phony_targets_include_continuation_lines() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let mut source = String::from(
            ".PHONY: build \\\n\ttest lint\nSHELL = /bin/sh\n.PHONY: \\\n\trelease \\\n\tdocs\n\n",
        );
        for index in 0..60 {
            source.push_str(&format!(
                "target-{index:02}: dep-{index:02}\n\tRECIPE_{index:02}\n"
            ));
        }
        std::fs::write(root.join("Makefile"), &source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        let crate::content::BatchContent::Lines { spans } = &batches[0].content else {
            panic!("expected a lines batch");
        };
        let rows: Vec<_> = spans.iter().map(|span| (span.start, span.end)).collect();
        assert_eq!(rows, [(1, 2), (4, 6)]);
    }

    #[test]
    fn plaintext_license_file_names() {
        for name in [
            "LICENSE",
            "license",
            "LICENSE-MIT",
            "LICENSE.txt",
            "LICENSE.md",
            "COPYING",
            "NOTICE",
            "MIT-LICENSE.txt",
            "LICENSE-THIRD-PARTY.txt",
            "DOCKER_LICENSE",
            "arm_license.txt",
            "License.md",
        ] {
            assert!(is_license_file_name(name), "{name}");
        }
        for name in [
            "LICENSE-HEADER",
            "license-header.txt",
            "license.go",
            "licenses.json",
            "license-management.md",
            "license-server.md",
            "license-api.md",
        ] {
            assert!(!is_license_file_name(name), "{name}");
        }
    }

    /// Drive the full `FsWalker` + scheduler against a real
    /// directory (an in-memory `SourceCache`
    /// preload bypasses `read_dir` and never exercises the discovery
    /// path). Asserts the plaintext content lands in the rendered
    /// output and that the scheduler logs a `Plaintext::Whole` batch,
    /// and that a license text renders only as its listing entry.
    #[test]
    fn plaintext_real_dir_renders_seeded_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("LICENSE"),
            "MIT License\n\nCopyright (c) Yoav\n\nSee LICENSE.\n",
        )
        .unwrap();
        std::fs::write(root.join(".tool-versions"), "rust 1.80.0\n").unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();

        assert!(
            !rendered.contains("MIT License"),
            "rendered output carries the LICENSE body:\n{rendered}",
        );
        assert!(
            rendered.contains("rust 1.80.0"),
            "rendered output is missing the .tool-versions body:\n{rendered}",
        );
        assert_no_plaintext_whole(&report, "LICENSE");
        assert_has_plaintext_whole(&report, ".tool-versions");
    }

    /// A file the code engine parses is never also a fallback file —
    /// two walkers claiming its rows would overlap.
    #[test]
    fn plaintext_fallback_skips_files_the_code_engine_parses() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("App.jsx"), "export const X = 1;\n").unwrap();
        std::fs::write(root.join("App.kt"), "class App\n").unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let files: Vec<_> = expand_in_dir(root, &ctx)
            .into_iter()
            .map(|batch| batch.key)
            .collect();
        assert_eq!(
            files,
            vec![BatchKey::from(PlaintextKey::DeclSurface {
                file: root.join("App.kt")
            })]
        );
    }

    /// An extensionless file is a script when it opens with a shebang, and
    /// left to the listing otherwise or when a build tool generated it.
    #[test]
    fn plaintext_fallback_claims_extensionless_shebang_scripts() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("deploy"), "#!/bin/sh\nset -e\nmake release\n").unwrap();
        std::fs::write(root.join("gradlew"), "#!/bin/sh\nexec java \"$@\"\n").unwrap();
        std::fs::write(root.join("AUTHORS"), "Ada\nGrace\n").unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let keys: Vec<_> = expand_in_dir(root, &ctx)
            .into_iter()
            .map(|batch| batch.key)
            .collect();
        assert_eq!(
            keys,
            vec![
                BatchKey::from(PlaintextKey::DeclSurface {
                    file: root.join("deploy")
                }),
                BatchKey::from(PlaintextKey::Whole {
                    file: root.join("deploy")
                }),
            ]
        );
    }

    /// Where no walker parses the language a repository is written in, its
    /// files' surfaces (interfaces included) price like parsed
    /// declarations; a side language's stay below them.
    #[test]
    fn plaintext_primary_language_surface_prices_like_a_parsed_declaration() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("core.ml"), "let step () = ()\n".repeat(50)).unwrap();
        std::fs::write(root.join("core.mli"), "val step : unit -> unit\n").unwrap();
        std::fs::write(root.join("tool.rb"), "def tool\nend\n").unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let decl = crate::value::code_rung_value(crate::batch::Rung::Decl);
        let value = |name: &str| class_value(Class::LanguageSource, &root.join(name), &ctx);
        assert_eq!(value("core.ml"), decl);
        assert_eq!(value("core.mli"), decl);
        assert!(value("tool.rb") < decl);
    }

    /// A small file in an unparsed language renders whole once the
    /// budget reaches past its surface — not four lines and a `…` for
    /// the closing brace.
    #[test]
    fn plaintext_small_source_text_file_renders_whole() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("Foo.java"),
            "package demo;\n\npublic class Foo {\n    int x;\n}\n",
        )
        .unwrap();
        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let rendered = scheduler.run().render();
        assert!(rendered.contains("5→}"), "{rendered}");
        assert!(!rendered.contains('…'), "{rendered}");
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
        std::fs::write(root.join(".tool-versions"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        assert_has_plaintext_whole(&report, ".tool-versions");
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
    fn plaintext_oversized_named_file_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let long_line = "x".repeat(250);
        let body: String = std::iter::repeat_n(long_line.as_str(), 60)
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(root.join(".gitignore"), body).unwrap();

        let scheduler = Scheduler::new(root.to_path_buf(), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        assert_no_plaintext_whole(&report, ".gitignore");
    }

    fn assert_has_plaintext_whole(report: &crate::scheduler::RunReport, suffix: &str) {
        let keys: Vec<_> = report.scheduled.iter().map(|r| r.key.clone()).collect();
        assert!(
            keys.iter().any(|k| matches!(
                k,
                BatchKey::Plaintext(PlaintextKey::Whole { file }) if file.ends_with(suffix),
            )),
            "missing Plaintext::Whole batch ending with {suffix:?}; scheduled keys: {keys:?}",
        );
    }

    fn assert_no_plaintext_whole(report: &crate::scheduler::RunReport, suffix: &str) {
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
