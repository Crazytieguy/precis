//! Plaintext walker — the home for every file precis has no parser
//! for. Two jobs:
//!
//! 1. **Named files** ([`classify_plaintext`]): build files, dotenv
//!    samples, version pins and the pnpm workspace, each rendered whole or as
//!    a head slice and priced by class ([`class_value`]).
//!    Credential-bearing names (`.env`, `.npmrc`, `secrets.sh`) are
//!    never admitted; dotenv *samples* are, since they carry
//!    placeholders and document the deploy-facing config keys.
//!    At a package root, the project's manifest in a format no walker
//!    parses ([`is_unparsed_manifest_name`]) is claimed too.
//! 2. **Every other source-like text file** ([`Class::LanguageSource`],
//!    [`Class::FlatText`]): the language-agnostic fallback for formats no
//!    parser claims, rendered as its [`declaration_surface`] and, when
//!    short, whole behind it. Markup documents (reST, AsciiDoc, …) are left
//!    to the listing and the floor.
//!
//! Once the walkers' batches are all scheduled, it also renders the head
//! of every listed file they leave untouched ([`floor_batches`]).

use std::collections::{HashMap, HashSet};
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
/// [`floor_batches`] reaches, so a multi-megabyte file is not
/// tokenized whole for a budget that shows its head.
const SOURCE_TEXT_BYTE_GATE: usize = 512 * 1024;

/// Mean bytes per line above which a file is machine-generated rather
/// than hand-wrapped: minified bundles and serialized blobs sit in the
/// thousands, hand-written source and prose in the tens.
const SOURCE_TEXT_MAX_MEAN_LINE_BYTES: usize = 200;

/// Characters past which a surface line is an attribute dump, a data row
/// or generated output (a Maven `<project xmlns=…>` opener) rather than a
/// declaration. Skipped rather than truncated.
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
    /// `Dockerfile`, compose files). A root one too long to render whole
    /// shows its flat surface, as [`Class::FlatText`] does.
    Build,
    /// Checked-in dotenv sample/template (`.env.sample`) — the
    /// deploy-facing config-key documentation, head-sampled when long.
    DotenvSample,
    /// The project's manifest or build script in a format no walker parses
    /// ([`is_unparsed_manifest_name`]). Rendered like [`Class::FlatText`],
    /// so its head fields (the project's name, version and description)
    /// lead, and priced as a manifest's identity block.
    Manifest,
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
/// (other walkers' formats, out-of-scope variants).
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

/// A repository's own manifest or build script in a format no walker
/// parses: its identity, dependencies and build entry points. Claimed
/// only at a [package root](is_package_root), where it describes the
/// project rather than one module of it.
pub(crate) fn is_unparsed_manifest_name(name: &str) -> bool {
    #[rustfmt::skip]
    const NAMES: &[&str] = &[
        "pom.xml", "composer.json", "build.sbt", "CMakeLists.txt", "action.yml", "action.yaml",
        "DESCRIPTION", "rebar.config", "dune-project", "deps.edn", "pubspec.yaml", "shard.yml",
    ];
    NAMES.contains(&name) || name.ends_with(".cabal") || name.ends_with(".nimble")
}

/// The root, or a first-level directory that holds the project itself:
/// `src/`, or one named after the repository.
fn is_package_root(dir: &Path, ctx: &WalkCtx) -> bool {
    dir == ctx.root()
        || (dir.parent() == Some(ctx.root())
            && (dir.file_name() == Some("src".as_ref())
                || dir.file_name() == ctx.root().file_name()))
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

/// A file named for the credentials it holds: a dotenv file (`.env`,
/// `.env.local`, `production.env`), a name led by a credential word
/// (`secrets.yml`, `credentials-dev.ini`, `creds_staging.conf`, an
/// extensionless `secrets` script), a tool's auth file, Terraform
/// variable values, or a service-account key. Samples (`.env.example`,
/// `secrets.yml.sample`) hold placeholders, and source code and
/// documents (`credentials.py`, `secrets.md`) are about credentials
/// rather than holding them.
pub(crate) fn is_credential_name(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    #[rustfmt::skip]
    const EXEMPT_SUFFIXES: &[&str] =
        &[".example", ".sample", ".template", ".dist", ".md", ".mdx", ".rst", ".adoc"];
    let is_exempt = EXEMPT_SUFFIXES.iter().any(|suffix| lower.ends_with(suffix))
        || super::language_group(path).is_some();
    let leading = lower.split(['.', '-', '_']).next().unwrap_or_default();
    !is_exempt
        && (matches!(
            leading,
            "env" | "secret" | "secrets" | "credential" | "credentials" | "creds"
        ) || lower.starts_with(".env.")
            || lower.ends_with(".env")
            || lower.ends_with(".tfvars")
            || lower.ends_with(".tfvars.json")
            || (lower.ends_with(".json")
                && ["service-account", "service_account", "serviceaccount"]
                    .iter()
                    .any(|prefix| lower.starts_with(prefix)))
            || matches!(
                lower.as_str(),
                ".npmrc" | ".netrc" | ".pypirc" | ".git-credentials" | ".htpasswd"
            ))
}

/// Programming-language extensions the fallback claims. A directory
/// of these files is a source package whether or not this crate can
/// parse them, so the source inventory reads from this list too.
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
#[rustfmt::skip]
pub(crate) const SOURCE_TEXT_LANGUAGE_EXTENSIONS: &[&str] = &[
    // JVM / .NET
    "java", "kt", "kts", "scala", "sc", "groovy", "clj", "cljs", "cljc", "cs", "fs", "fsx", "vb",
    // C family (`.c` / `.h` belong to the C walker, but for a `.h` written in C++)
    "cpp", "cc", "cxx", "hpp", "hh", "hxx", "m", "mm", "cu", "cuh",
    // other compiled languages
    "swift", "zig", "dart", "nim", "cr", "hs", "lhs", "ml", "mli", "elm", "erl", "hrl", "ex", "exs",
    "pas", "pp", "dpr", "lpr", "f", "f90", "f95", "f03", "f08", "for", "cob", "cbl", "cpy", "adb",
    "ads", "sol",
    // hardware description
    "v", "sv", "svh", "vhd", "vhdl",
    // scripting
    "rb", "php", "pl", "pm", "r", "jl", "tcl", "vim", "gd", "coffee", "el", "lisp", "scm", "rkt",
    // TeX classes and packages (macro libraries, not documents)
    "cls", "sty",
    // component-file web frameworks
    "vue", "svelte", "astro",
];

/// Contract-bearing extensions whose indentation-zero lines are real
/// declarations — protobuf messages, GraphQL types, Terraform blocks,
/// Gradle plugin and dependency blocks — but whose *directories* are
/// schema or config trees rather than source packages. Same surface
/// value as a language file, no source-inventory promotion: promoting
/// them buys stacks of asset-tree listings.
const SOURCE_TEXT_DECLARATIVE_EXTENSIONS: &[&str] = &[
    "proto", "thrift", "graphql", "gql", "capnp", "fbs", "tf", "hcl", "nix", "dhall", "cue",
    "gradle", "gemspec", "podspec", "rake",
];

/// Extensions with no *program* structure to surface: plain text, flat
/// config, scripts and build glue, and stylesheets, whose selectors
/// describe presentation rather than what the program does. `.bat`/`.cmd`
/// are absent on purpose — in practice they are generated wrappers
/// (`gradlew.bat`, `mvnw.cmd`), 158 tokens of argument marshalling.
#[rustfmt::skip]
const SOURCE_TEXT_FLAT_EXTENSIONS: &[&str] = &[
    "txt", "ini", "cfg", "conf", "properties", "sh", "bash", "zsh", "fish", "ps1", "psm1", "awk",
    "cmake", "mk", "mak", "bzl", "bazel", "gyp", "gni", "ld", "css", "scss", "sass", "less", "styl",
];

/// Extensionless build manifests the fallback claims by exact name.
#[rustfmt::skip]
const SOURCE_TEXT_FILENAMES: &[&str] = &[
    "Gemfile", "Rakefile", "Guardfile", "Brewfile", "Podfile", "Procfile", "Vagrantfile",
    "Jenkinsfile", "Berksfile", "Appfile", "Fastfile", "BUILD", "WORKSPACE", "SConstruct",
    "meson.build",
];

/// The fallback class `name` falls into on name evidence alone, or
/// `None` when the fallback doesn't own it. Rejects derived artifacts
/// (minified/bundled output, lockfiles, source maps) before the
/// extension check — these are the text files whose content makes the
/// output worse, not better.
fn classify_source_text(name: &str) -> Option<Class> {
    let lower = name.to_ascii_lowercase();
    if SOURCE_TEXT_FILENAMES.contains(&name) {
        return Some(Class::LanguageSource);
    }
    let (_, ext) = lower.rsplit_once('.')?;
    if is_derived_artifact_name(&lower) {
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

/// A derived sibling of a hand-authored file, by its lowercased name:
/// `app.min.js`, `bundle.chunk.css`, `pnpm-lock.yaml`, `main.js.map`.
fn is_derived_artifact_name(lower: &str) -> bool {
    let Some((stem, ext)) = lower.rsplit_once('.') else {
        return false;
    };
    #[rustfmt::skip]
    const STEM_SUFFIXES: &[&str] =
        &[".min", "-min", ".bundle", ".chunk", ".generated", "_generated", "-lock", ".lock"];
    STEM_SUFFIXES.iter().any(|suffix| stem.ends_with(suffix)) || matches!(ext, "map" | "lock")
}

/// Whether `file` starts with `#!`: an extensionless script (`bin/deploy`,
/// a tool shipped as one executable) names its language on its first line
/// rather than in its name.
fn opens_with_shebang(file: &Path) -> bool {
    let mut head = [0; 2];
    std::fs::File::open(file)
        .is_ok_and(|mut opened| opened.read_exact(&mut head).is_ok() && &head == b"#!")
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
#[rustfmt::skip]
const SOURCE_TEXT_IMPORT_PREFIXES: &[&str] = &[
    "import", "#import", "#include", "using ", "require", "from ", "use ", "@use", "@import",
    "@forward", "open ", "extern crate", "include ", "load(", "export * from", "export {",
];

/// Substrings that mark a comment line as boilerplate rather than
/// content: legal banners, and pragmas addressed to a compiler or
/// linter. Both open files across whole ecosystems — v0.1's
/// head-slice fallback rendered the license banner and nothing else
/// for most Java, C++ and Swift files, and `# frozen_string_literal:
/// true` is the first line of essentially every modern Ruby file.
#[rustfmt::skip]
const SOURCE_TEXT_BOILERPLATE_MARKERS: &[&str] = &[
    "copyright", "spdx-", "all rights reserved", "licensed under", "license at",
    "license, version", "permission is hereby granted", "warranties", "frozen_string_literal",
    "-*-", "vim:", "coding:", "eslint-disable", "prettier-ignore", "stylelint-disable",
    "clang-format", "shellcheck", "@ts-nocheck", "noqa", "type: ignore", "// mark:",
];

/// Classify one trimmed surface line, `in_block_comment` when it sits
/// inside a comment opened on an earlier line. `None` drops it.
fn classify_surface_line(trimmed: &str, in_block_comment: bool) -> Option<SurfaceLine> {
    if is_block_closer(trimmed) || trimmed.starts_with("#!") || is_compiler_directive(trimmed) {
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

/// A conditional-compilation or compiler-pragma line (`#if`, `#else`,
/// `#pragma warning disable`): it says how the file builds, not what it
/// declares, and shown without its `#endif` it reads as an open block.
const COMPILER_DIRECTIVES: &str =
    "if ifdef ifndef else elif elseif endif pragma error warning nullable line";

fn is_compiler_directive(trimmed: &str) -> bool {
    trimmed.strip_prefix('#').is_some_and(|rest| {
        let word = rest.split(|c: char| !c.is_ascii_alphabetic()).next();
        word.is_some_and(|word| has_word(COMPILER_DIRECTIVES, word))
    })
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
    for marker in ["//", "/*", "{-", "<!--", "\"\"\"", "'''"] {
        if trimmed.starts_with(marker) {
            return true;
        }
    }
    for marker in ["#", "*", "--", "(*", ";", "%", "..", "\"", "@rem", "rem "] {
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

/// Number of leading lines occupied by a legal/pragma banner. Zero when
/// the file's opening comment block carries no boilerplate marker, so a
/// genuine file-purpose comment survives. Whole-block, not per-line: a
/// marker matches "Copyright (c) 2014" but not the continuation lines of
/// the same header. A doc comment set off from the banner by a blank line
/// is the file's documentation, not banner.
fn boilerplate_banner_end(lines: &[&str], in_block_comment: &[bool]) -> usize {
    let mut is_banner = false;
    let mut after_blank = false;
    let mut doc_start = None;
    for (index, (line, &in_block_comment)) in lines.iter().zip(in_block_comment).enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#!") || trimmed.starts_with("<?php") {
            after_blank |= trimmed.is_empty() && !in_block_comment;
            continue;
        }
        if !in_block_comment && !is_comment_line(trimmed) {
            return if is_banner {
                doc_start.unwrap_or(index)
            } else {
                0
            };
        }
        if is_banner
            && after_blank
            && doc_start.is_none()
            && DOC_COMMENT_OPENERS
                .iter()
                .any(|opener| trimmed.starts_with(opener))
        {
            doc_start = Some(index);
        }
        after_blank = false;
        let lower = trimmed.to_ascii_lowercase();
        is_banner |= SOURCE_TEXT_BOILERPLATE_MARKERS
            .iter()
            .any(|marker| lower.contains(marker));
    }
    // A file that is comments all the way down is a comment-formatted
    // document, not a banner followed by code; skipping it would leave
    // nothing to render.
    0
}

/// Openers of a documentation comment, as opposed to a plain one.
const DOC_COMMENT_OPENERS: &[&str] = &["/**", "///", "//!", "/*!", "-- |", "{-|", "(**"];

/// Block comment delimiters: the C family's, Haskell's, the ML family's
/// and Pascal's, and XML's.
const BLOCK_COMMENTS: &[(&str, &str)] =
    &[("/*", "*/"), ("{-", "-}"), ("(*", "*)"), ("<!--", "-->")];

/// Per line, whether it sits inside a block comment opened on an earlier
/// line — interior lines carry no comment marker of their own. Only a
/// comment line that starts with an opener opens one, so a glob in a
/// shell script (`rm build/*`) or a C dereference (`(*fn)(x)`) does not.
fn block_comment_interiors(lines: &[&str]) -> Vec<bool> {
    let mut open_until: Option<&str> = None;
    lines
        .iter()
        .map(|line| {
            let trimmed = line.trim();
            let interior = open_until.is_some();
            if let Some(end) = open_until {
                if trimmed.contains(end) {
                    open_until = None;
                }
            } else if is_comment_line(trimmed)
                && let Some((start, end)) = BLOCK_COMMENTS
                    .iter()
                    .find(|(start, _)| trimmed.starts_with(start))
                && !trimmed[start.len()..].contains(end)
            {
                open_until = Some(end);
            }
            interior
        })
        .collect()
}

/// The file's **declaration surface** (its selected rows) and the rows its
/// [boilerplate banner](boilerplate_banner_end) spans: the lines at the
/// shallowest indentation levels that together yield a non-trivial
/// declaration roster, capped per line class.
///
/// Indentation is the one structural signal every text format shares, and
/// the shallowest level is where a file's declarations live. In a file
/// with no nesting every line qualifies, so the rule degrades to a head
/// slice. Where column zero is one wrapper line (`module Foo`, a
/// `namespace`), levels are added, shallowest first, until the roster has
/// [`SOURCE_TEXT_MIN_DECLS`] declarations or
/// [`SOURCE_TEXT_MAX_INDENT_LEVELS`] levels have been consumed.
///
/// `decl_cap` bounds the declarations kept: [`SOURCE_TEXT_DECL_LINES`] for
/// a roster among many files, unbounded when the file is the whole walk
/// and the surface is its outline.
fn declaration_surface(source: &str, class: Class, decl_cap: usize) -> (Vec<usize>, usize) {
    let lines: Vec<&str> = source.lines().collect();
    let in_block_comment = block_comment_interiors(&lines);
    let banner_end = boilerplate_banner_end(&lines, &in_block_comment);
    let is_language = class == Class::LanguageSource;
    let continues = if is_language {
        continuation_lines(&lines, &in_block_comment)
    } else {
        vec![false; lines.len()]
    };
    let opens_block = if is_language {
        block_openers(&lines, &continues)
    } else {
        vec![false; lines.len()]
    };
    let mut rows: Vec<(usize, usize, SurfaceLine, DeclarationRank)> = Vec::new();
    for (index, (line, &in_block_comment)) in lines
        .iter()
        .zip(&in_block_comment)
        .enumerate()
        .skip(banner_end)
    {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.chars().count() > SOURCE_TEXT_MAX_LINE_CHARS
            || continues[index]
            || (is_language && is_annotation_only(trimmed))
        {
            continue;
        }
        let Some(kind) = classify_surface_line(trimmed, in_block_comment) else {
            continue;
        };
        let rank = if is_language && kind == SurfaceLine::Decl {
            declaration_rank(trimmed, opens_block[index])
        } else {
            DeclarationRank::Heading
        };
        rows.push((indentation(line), index + 1, kind, rank));
    }

    let mut levels: Vec<usize> = rows.iter().map(|(indent, ..)| *indent).collect();
    levels.sort_unstable();
    levels.dedup();

    let caps = [
        SOURCE_TEXT_IMPORT_LINES,
        SOURCE_TEXT_COMMENT_LINES,
        decl_cap,
    ];
    let mut used = [0usize; 3];
    let mut selected: Vec<usize> = Vec::new();
    // In a language file the roster is for types and functions;
    // statements and directives take the slots they leave. A flat file's
    // surface stays its head.
    if is_language {
        rows.sort_by_key(|&(.., rank)| rank);
    }
    for (depth, level) in levels
        .into_iter()
        .take(SOURCE_TEXT_MAX_INDENT_LEVELS)
        .enumerate()
    {
        // A deeper level whose code lines are all statements is function
        // bodies; a level of only comments or imports does not decide.
        let mut level_declarations = rows
            .iter()
            .filter(|&&(indent, _, kind, _)| indent == level && kind == SurfaceLine::Decl)
            .peekable();
        let is_statement_body = level_declarations.peek().is_some()
            && level_declarations.all(|&(.., rank)| rank >= DeclarationRank::Statement);
        if is_language && depth > 0 && is_statement_body {
            break;
        }
        for &(_, line, kind, _) in rows.iter().filter(|(indent, ..)| *indent == level) {
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
    (selected, banner_end)
}

fn indentation(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// Per line, whether the next line with content is indented deeper — the
/// line heads a block. A line of only brackets is skipped over, so an
/// Allman-style `{` does not take the heading from the line above it, and
/// so are `continues` lines, so a wrapped signature heads the block its
/// last line opens while a multi-line annotation heads nothing.
fn block_openers(lines: &[&str], continues: &[bool]) -> Vec<bool> {
    let mut opens = vec![false; lines.len()];
    let mut next_indent: Option<usize> = None;
    for (index, line) in lines.iter().enumerate().rev() {
        let indent = indentation(line);
        opens[index] = next_indent.is_some_and(|next| next > indent);
        let trimmed = line.trim();
        if !trimmed.is_empty() && !is_block_closer(trimmed) && !continues[index] {
            next_indent = Some(indent);
        }
    }
    opens
}

/// Roster order of a declaration line, first to last.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DeclarationRank {
    /// Heads a block, or declares with no body: an interface or protocol
    /// requirement, a type signature, a forward declaration.
    Heading,
    /// A declaration whose body follows on the same line.
    OneLiner,
    Statement,
    /// Private members, test cases and control flow: not the file's API.
    Internal,
}

/// Words that declare the name after them, across the covered languages.
const DECLARATION_KEYWORDS: &str = "class interface trait object struct enum protocol extension \
    module record union namespace defmodule type typealias data newtype instance def fun func \
    function fn sub defmacro proc val var let const";

/// First words of a line that is not part of a file's API: a private or
/// file-local member, a test case, or a control-flow statement.
const INTERNAL_LEADERS: &str = "private fileprivate defp static test begin rescue ensure else \
    elsif elif comptime if unless for foreach while until switch match when try catch finally do \
    return throw raise new await yield";

/// Whether `word` is one of the space-separated words of `table`.
fn has_word(table: &str, word: &str) -> bool {
    table.split_whitespace().any(|entry| entry == word)
}

/// Where a declaration line falls in the roster. A line reads as a
/// declaration by a keyword followed by a name, by a Haskell `name ::`
/// signature, or by the C-family `Type name(` shape; it has a body when
/// an `=` follows at bracket depth zero.
fn declaration_rank(trimmed: &str, opens_block: bool) -> DeclarationRank {
    let head_end = trimmed
        .find(['(', '=', '{', '<', '['])
        .unwrap_or(trimmed.len());
    let words: Vec<&str> = trimmed[..head_end].split_whitespace().collect();
    if words
        .first()
        .is_some_and(|first| has_word(INTERNAL_LEADERS, first))
    {
        return DeclarationRank::Internal;
    }
    if opens_block {
        return DeclarationRank::Heading;
    }
    let declares = words
        .iter()
        .rev()
        .skip(1)
        .any(|word| has_word(DECLARATION_KEYWORDS, word))
        || words.get(1) == Some(&"::")
        || (words.len() >= 2 && trimmed[head_end..].starts_with('('));
    if !declares {
        DeclarationRank::Statement
    } else if has_same_line_body(trimmed) {
        DeclarationRank::OneLiner
    } else {
        DeclarationRank::Heading
    }
}

/// Whether an assignment `=` (not `==`, `=>`, `<=`, …) sits outside every
/// round and square bracket and literal on the line.
fn has_same_line_body(trimmed: &str) -> bool {
    let bytes = trimmed.as_bytes();
    let operator_char = |at: Option<&u8>| at.is_some_and(|c| b"=<>!:+-*/".contains(c));
    bytes.iter().enumerate().any(|(index, &byte)| {
        byte == b'='
            && !operator_char(index.checked_sub(1).and_then(|before| bytes.get(before)))
            && !operator_char(bytes.get(index + 1))
            && bracket_balance(&trimmed[..index]) == 0
    })
}

/// Leading characters of a line indented under the one above it that
/// carries on that line's expression — a method chain, a wrapped
/// assignment or type, a guard, a pipe.
const CONTINUATION_OPERATORS: &[char] = &['.', '=', '?', ':', '|', '&', '+', '-', ','];

/// Trailing characters of a line whose expression carries on to the
/// deeper line below it: an assignment, a binary operator, a list.
const CONTINUED_OPERATORS: &[char] = &['=', '+', '-', '&', ','];

/// Per line, whether it continues a logical line begun above it: the rest
/// of a `(` or `[` left open there (wrapped parameters, a multi-line
/// annotation, a literal), a line led by a closing bracket, or a deeper
/// line led by an operator or below one that ends in one. It declares
/// nothing on its own. A line back at or above the indentation that opened
/// the bracket ends it, so a miscounted bracket cannot swallow the rest of
/// the file.
fn continuation_lines(lines: &[&str], in_block_comment: &[bool]) -> Vec<bool> {
    let mut continues = string_interiors(lines);
    let mut depth = 0;
    let mut head_indent = 0;
    let mut previous_carries_on = false;
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if continues[index]
            || trimmed.is_empty()
            || in_block_comment[index]
            || is_comment_line(trimmed)
        {
            continue;
        }
        let indent = indentation(line);
        let led_by_closer = trimmed.starts_with([')', ']', '}']);
        if depth > 0 && indent <= head_indent && !led_by_closer {
            depth = 0;
        }
        continues[index] = depth > 0
            || led_by_closer
            || (indent > head_indent
                && (previous_carries_on || trimmed.starts_with(CONTINUATION_OPERATORS)));
        if depth == 0 && !continues[index] {
            head_indent = indent;
        }
        depth = (depth + bracket_balance(trimmed)).max(0);
        previous_carries_on = trimmed.ends_with(CONTINUED_OPERATORS);
    }
    continues
}

/// Per line, whether it sits inside a multi-line string opened on an
/// earlier line — a `"""` or `'''` literal, or a heredoc (`<<<'EOF'`,
/// `<<~SQL`) — through the line that closes it. Its text is data, however
/// much it looks like code. A string that never closes marks nothing.
fn string_interiors(lines: &[&str]) -> Vec<bool> {
    let mut rows_by_leading_identifier: Option<HashMap<&str, Vec<usize>>> = None;
    let mut inside = vec![false; lines.len()];
    let mut index = 0;
    while index < lines.len() {
        let closing_row = match multiline_string_end(lines[index]) {
            Some((end, true)) => rows_by_leading_identifier
                .get_or_insert_with(|| index_rows_by_leading_identifier(lines))
                .get(end)
                .and_then(|rows| rows.get(rows.partition_point(|&row| row <= index)))
                .copied(),
            Some((end, false)) => lines[index + 1..]
                .iter()
                .position(|line| line.contains(end))
                .map(|offset| index + 1 + offset),
            None => None,
        };
        if let Some(last) = closing_row {
            inside[index + 1..=last].fill(true);
            index = last;
        }
        index += 1;
    }
    inside
}

/// The rows each identifier leads, in order — where a heredoc opened with
/// that identifier can close. Looked up rather than scanned for, so a line
/// that only looks like an opener (`1<<BITS`) costs no pass over the rest
/// of the file.
fn index_rows_by_leading_identifier<'a>(lines: &[&'a str]) -> HashMap<&'a str, Vec<usize>> {
    let mut rows: HashMap<&str, Vec<usize>> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        let identifier = leading_identifier(line.trim_start());
        if !identifier.is_empty() {
            rows.entry(identifier).or_default().push(row);
        }
    }
    rows
}

fn leading_identifier(text: &str) -> &str {
    let end = text
        .find(|c: char| !is_identifier_char(c))
        .unwrap_or(text.len());
    &text[..end]
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// For a line that opens a multi-line string, the token that ends it and
/// whether that token is a heredoc identifier, which ends it only at the
/// start of a line. A heredoc's identifier must start upper-case, so a
/// shift (`a << b`) or a stream insertion is not read as one.
fn multiline_string_end(line: &str) -> Option<(&str, bool)> {
    if let Some(delimiter) = ["\"\"\"", "'''"]
        .into_iter()
        .find(|delimiter| line.matches(delimiter).count() % 2 == 1)
    {
        return Some((delimiter, false));
    }
    let (_, rest) = line.split_once("<<")?;
    let rest = rest
        .trim_start_matches(['<', '~', '-'])
        .trim_start_matches(['\'', '"']);
    let identifier = leading_identifier(rest);
    identifier
        .starts_with(|c: char| c.is_ascii_uppercase())
        .then_some((identifier, true))
}

/// Opening minus closing round and square brackets on a line, outside
/// string and character literals and a trailing `//` comment. A `'` after
/// a letter or digit is a Haskell prime or a digit separator, not a quote,
/// and so is one no later `'` on the line closes: a type variable
/// (`(x : 'a)`) or a Lisp quote (`(provide 'widget)`).
fn bracket_balance(trimmed: &str) -> isize {
    let mut balance = 0;
    let mut quote: Option<char> = None;
    let mut previous = ' ';
    let mut chars = trimmed.chars().peekable();
    while let Some(c) = chars.next() {
        match quote {
            Some(_) if c == '\\' => {
                chars.next();
            }
            Some(open) if c == open => quote = None,
            Some(_) => {}
            None => match c {
                '"' | '`' => quote = Some(c),
                '\'' if !previous.is_alphanumeric() && chars.clone().any(|c| c == '\'') => {
                    quote = Some(c);
                }
                '/' if chars.peek() == Some(&'/') => break,
                '(' | '[' => balance += 1,
                ')' | ']' => balance -= 1,
                _ => {}
            },
        }
        previous = c;
    }
    balance
}

/// A line of annotations or attributes alone (`@Deprecated(`,
/// `@MainActor`, `[Fact]`, `#[Route("/")]`): it qualifies the declaration
/// below it and names nothing itself. Whitespace outside brackets and
/// literals means a declaration follows on the same line (`@Override
/// public void run()`, Objective-C's `@interface Foo`).
fn is_annotation_only(trimmed: &str) -> bool {
    let body = trimmed
        .strip_prefix('#')
        .filter(|rest| rest.starts_with('['))
        .unwrap_or(trimmed);
    let mut chars = body.chars();
    if !matches!(chars.next(), Some('@' | '[')) || !chars.next().is_some_and(char::is_alphabetic) {
        return false;
    }
    !body
        .char_indices()
        .any(|(index, c)| c.is_whitespace() && bracket_balance(&body[..index]) <= 0)
}

/// A fallback file's declaration surface, then — when the surface
/// elides part of a file short enough to render whole — the file past its
/// license banner behind it. Nothing when the file is unreadable,
/// machine-generated, or has no surface. Line length says nothing about
/// prose, which is often written one paragraph per line.
fn push_source_text_batches(out: &mut Vec<Batch>, file: &Path, ctx: &WalkCtx, class: Class) {
    let Some(source) = gated_read_source(file, ctx, SOURCE_TEXT_BYTE_GATE) else {
        return;
    };
    if has_generated_marker(&source)
        || (class == Class::LanguageSource && has_minified_lines(&source))
    {
        return;
    }
    let decl_cap = if ctx.dir_filter().named_file().is_some() {
        usize::MAX
    } else {
        SOURCE_TEXT_DECL_LINES
    };
    let (selected, banner_end) = declaration_surface(&source, class, decl_cap);
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
        .skip(banner_end)
        .filter(|line| !line.trim().is_empty())
        .count();
    if surface_rows < content_rows
        && source.len() <= PLAINTEXT_BYTE_GATE
        && source.line_count() <= PLAINTEXT_LINE_CAP
        && let Some(content) = single_file_lines_content(
            file,
            &source,
            (banner_end + 1..=source.line_count()).collect(),
        )
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
    #[rustfmt::skip]
    const MARKERS: &[&str] = &[
        "@generated", "code generated by", "do not edit", "automatically generated",
        "auto-generated", "autogenerated",
    ];
    has_nul_byte(source)
        || source
            .lines()
            .take(SOURCE_TEXT_GENERATED_SCAN_LINES)
            .any(|line| {
                let lower = line.to_ascii_lowercase();
                MARKERS.iter().any(|marker| lower.contains(marker))
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
        let named = classify_plaintext(name).or_else(|| {
            (is_package_root(dir, ctx) && is_unparsed_manifest_name(name))
                .then_some(Class::Manifest)
        });
        // Parsed languages belong to the code engine, except a C++
        // header the C grammar can't parse; a second slice would overlap
        // its spans.
        let owned_elsewhere = super::code::Language::from_path(&file).is_some();
        let Some(class) = named
            .or_else(|| {
                (!owned_elsewhere)
                    .then(|| classify_source_text(name))
                    .flatten()
            })
            .or_else(|| {
                // `gradlew` and `mvnw` are generated build-tool wrappers, the
                // same in every project that has one.
                (!name.contains('.')
                    && !matches!(name.as_str(), "gradlew" | "mvnw")
                    && opens_with_shebang(&file))
                .then_some(Class::FlatText)
            })
            .or_else(|| {
                owned_elsewhere
                    .then(|| gated_read_source(&file, ctx, SOURCE_TEXT_BYTE_GATE))
                    .flatten()
                    .is_some_and(|source| super::code::is_cpp_header(&file, &source))
                    .then_some(Class::LanguageSource)
            })
        else {
            continue;
        };
        if matches!(
            class,
            Class::Manifest | Class::LanguageSource | Class::FlatText
        ) {
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
                None => {
                    if class == Class::Build && ctx.depth_from_root(&file) == 1 {
                        push_source_text_batches(&mut out, &file, ctx, class);
                    }
                    continue;
                }
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

/// Once every batch a walker emitted is scheduled, the head of each listed
/// file none of `emitted` touches. They spend only the budget the walkers
/// leave, so a repository that runs dry renders its documents, templates
/// and configs rather than their names alone. The head is
/// [`PLAINTEXT_BYTE_GATE`] long. The file a single-file walk names instead
/// renders whatever of its first [`SOURCE_TEXT_BYTE_GATE`] no batch shows;
/// any other file stays a name when it is hidden, a license text, derived
/// or machine-generated (and, like every read, when it is
/// [refused](super::is_refused)).
pub(super) fn floor_batches(emitted: &[Batch], ctx: &WalkCtx) -> Vec<Batch> {
    let mut shown: HashMap<&Path, HashSet<usize>> = HashMap::new();
    let mut listed: Vec<(&Path, &str)> = Vec::new();
    for batch in emitted {
        match &batch.content {
            crate::content::BatchContent::Lines { spans, .. } => {
                for span in spans {
                    shown
                        .entry(&span.path)
                        .or_default()
                        .extend(span.start..=span.end);
                }
            }
            crate::content::BatchContent::Fs { groups } => {
                for group in groups {
                    if let crate::content::FsEntries::Listed(names) = &group.entries {
                        listed.extend(
                            names
                                .iter()
                                .filter_map(|name| Some((group.parent.as_path(), name.to_str()?))),
                        );
                    }
                }
            }
        }
    }
    let named_file = ctx.dir_filter().named_file();
    let mut out = Vec::new();
    for (dir, name) in listed {
        let kind = list_dir(dir, ctx.dir_filter()).get(name).copied();
        if kind != Some(crate::fs_util::EntryKind::File) {
            continue;
        }
        let file = dir.join(name);
        let is_named = named_file == Some(file.as_path());
        if !is_named
            && (shown.contains_key(file.as_path())
                || is_hidden(&file, ctx)
                || is_license_file_name(name)
                || is_derived_artifact_name(&name.to_ascii_lowercase()))
        {
            continue;
        }
        let head_bytes = if is_named {
            SOURCE_TEXT_BYTE_GATE
        } else {
            PLAINTEXT_BYTE_GATE
        };
        let Some(source) = file_head(&file, ctx, head_bytes) else {
            continue;
        };
        if has_nul_byte(&source) || (!is_named && is_machine_generated_text(&source)) {
            continue;
        }
        let shown = shown.get(file.as_path());
        let mut bytes_before = 0;
        let rows = (1..=source.line_count())
            .take_while(|&row| {
                bytes_before += source.line(row).map_or(0, str::len) + 1;
                bytes_before <= head_bytes
            })
            .filter(|row| shown.is_none_or(|shown| !shown.contains(row)))
            .collect();
        let Some(content) = single_file_lines_content(&file, &source, rows) else {
            continue;
        };
        let value = path_depth_factor(&file, ctx);
        out.push(Batch {
            key: PlaintextKey::Rest { file }.into(),
            predecessor: None,
            content,
            value,
        });
    }
    out
}

/// Whether `file` or a directory above it (below the root) is dot-prefixed:
/// CI, editor, linter and ignore-list configuration, which says nothing
/// about what the project does.
fn is_hidden(file: &Path, ctx: &WalkCtx) -> bool {
    file.strip_prefix(ctx.root()).is_ok_and(|relative| {
        relative
            .components()
            .any(|component| component.as_os_str().as_encoded_bytes().starts_with(b"."))
    })
}

/// `file`'s source as far as `head_bytes` reach into it: whole when a
/// walker has already read it or it fits, else only its head. The head runs
/// a few bytes past `head_bytes` so the line the cut falls in stays too long
/// to select. A head is cached under `file`'s own path, where the renderer
/// reads it; no walker reads it as the whole file only because the floor
/// runs once, after the pool has run dry and every expansion is done.
fn file_head(file: &Path, ctx: &WalkCtx, head_bytes: usize) -> Option<Arc<Source>> {
    if let Some(source) = ctx.source_cache().cached(file) {
        return Some(source);
    }
    let mut head = Vec::new();
    std::fs::File::open(file)
        .ok()?
        .take((head_bytes + 4) as u64)
        .read_to_end(&mut head)
        .ok()?;
    if head.len() > head_bytes {
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

/// The ops surface (how the project is built, deployed and versioned) and
/// an unparsed language's declaration surface sit high, the latter at a
/// parsed declaration's value when it is in the repository's primary
/// language ([`is_in_primary_language`]); an unparsed manifest prices as a
/// manifest's identity block. Contributor tooling and unclassified prose /
/// flat config sit low.
fn class_value(class: Class, file: &Path, ctx: &WalkCtx) -> f64 {
    let tier = match class {
        Class::LanguageSource if is_in_primary_language(file, ctx) => {
            crate::value::code_rung_value(crate::batch::Rung::Decl)
        }
        Class::Manifest => crate::value::manifest_identity_value(1.0, 1.0),
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

/// Mild promotion for a root `Makefile` / `Taskfile` / `justfile`: a
/// compact one answers "how do I build and run this", which NS authors buy
/// in the first screenful, while the class's own preset prices it as one
/// config file among many. A nested one is one component's build step, and
/// deploy / CI / linter config describes the contributor's toolchain.
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

    /// The lines a surface selects, for readable assertions.
    fn surface_text(source: &str, class: Class) -> Vec<&str> {
        let lines: Vec<&str> = source.lines().collect();
        let (rows, _) = declaration_surface(source, class, SOURCE_TEXT_DECL_LINES);
        rows.iter().map(|n| lines[n - 1]).collect()
    }

    fn surface(source: &str) -> Vec<&str> {
        surface_text(source, Class::LanguageSource)
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
        let text = surface(java);
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
        let text = surface(csharp);
        assert_eq!(text, vec!["using System;", "namespace ShareX"]);
    }

    /// A license in a Haskell `{- -}` block, or after PHP's `<?php` opener,
    /// is a banner too.
    #[test]
    fn plaintext_source_text_surface_skips_banners_in_every_comment_syntax() {
        let haskell = "{-\n    Copyright 2012 Vidar Holen\n\n    GNU General Public License\n-}\n\
                       module ShellCheck.AST where\n\ndata Token = Token\n";
        let text = surface(haskell);
        assert_eq!(
            text,
            vec!["module ShellCheck.AST where", "data Token = Token"]
        );

        let php = "<?php declare(strict_types=1);\n\n/*\n * This file is part of Composer.\n *\n\
                   * (c) Nils Adermann\n *\n * For the full copyright and license information\n */\n\n\
                   namespace Composer;\n\nclass Cache\n{\n}\n";
        let text = surface(php);
        assert_eq!(text, vec!["namespace Composer;", "class Cache"]);

        assert!(!is_comment_line("(*fn)(argument);"));
        assert!(block_comment_interiors(&["(* a", "b *)", "c"]) == [false, true, false]);
    }

    /// Conditional compilation, pragmas and editor section marks say how a
    /// file builds or folds, not what it declares.
    #[test]
    fn plaintext_source_text_surface_skips_compiler_directives_and_section_marks() {
        let swift = "#if canImport(Darwin)\nimport Darwin\n#elseif canImport(Glibc)\nimport Glibc\n#endif\n\n\
                     // MARK: - Instant\n\n#pragma warning disable CA1815\npublic struct Instant {\n}\n";
        let text = surface(swift);
        assert_eq!(
            text,
            vec!["import Darwin", "import Glibc", "public struct Instant {"]
        );
        assert!(!is_compiler_directive("#include <stdio.h>"));
        assert!(!is_compiler_directive("#define MAX 3"));
        assert!(is_comment_line("#Region \"Fields\""));
        assert!(is_comment_line("#endregion"));
        assert!(!is_comment_line("#region-picker {"));
    }

    /// The interior of a multi-line `<!-- … -->` is comment, not a roster
    /// of declarations ahead of the elements after it.
    #[test]
    fn plaintext_source_text_xml_comment_interior_is_comment() {
        let xml = "<?xml version=\"1.0\"?>\n<!--\n  ~ one\n  ~ two\n  ~ three\n  ~ four\n-->\n\
                   <project>\n  <groupId>org.example</groupId>\n  <artifactId>app</artifactId>\n\
                   </project>\n";
        let text = surface_text(xml, Class::Manifest);
        assert!(text.contains(&"  <artifactId>app</artifactId>"), "{text:?}");
        assert!(
            text.iter().filter(|line| line.contains('~')).count() <= SOURCE_TEXT_COMMENT_LINES,
            "{text:?}"
        );
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
        let text = surface_text(&vim, Class::LanguageSource);
        assert!(text.contains(&"function! plug#begin(...)"), "{text:?}");
        assert!(!surface_text(&vim, Class::FlatText).contains(&"function! plug#begin(...)"));
    }

    /// A flat config's indented value list is content, not a wrapped
    /// signature: every value stays on the surface.
    #[test]
    fn plaintext_source_text_flat_surface_keeps_indented_values() {
        let tox = "[testenv]\ndeps =\n    pytest\n    coverage\ncommands =\n    python -m pip check\n    python -m pytest\n";
        let text = surface_text(tox, Class::FlatText);
        for line in [
            "[testenv]",
            "    pytest",
            "    python -m pip check",
            "    python -m pytest",
        ] {
            assert!(text.contains(&line), "{line:?} missing from {text:?}");
        }
    }

    /// A multi-line annotation, a wrapped initializer and the tail of a
    /// wrapped signature continue the line above them and never take a slot
    /// from a declaration. Brackets inside literals do not count.
    #[test]
    fn plaintext_source_text_surface_skips_continuation_lines() {
        let kotlin = [
            "class A(",
            "  val x: Int,",
            ") {",
            "  @Deprecated(",
            "    message = \"x\",",
            "  )",
            "  fun old(): Int = 1",
            "  private val RULE: Predicate",
            "      = Rules::check",
            "  fun keep(): Int = 2",
            "  fun wrapped(",
            "    y: Int,",
            "  ): Int {",
            "    return y",
            "  }",
            "}",
        ]
        .join("\n");
        let lines: Vec<&str> = kotlin.lines().collect();
        let text = surface(&kotlin);
        assert_eq!(
            text,
            vec![
                "class A(",
                "  fun old(): Int = 1",
                "  private val RULE: Predicate",
                "  fun keep(): Int = 2",
                "  fun wrapped(",
            ]
        );
        let opens_block = block_openers(
            &lines,
            &continuation_lines(&lines, &vec![false; lines.len()]),
        );
        assert!(opens_block[0] && opens_block[10], "{opens_block:?}");
        assert!(!opens_block[3], "{opens_block:?}");
        assert_eq!(bracket_balance("foo(\"(\", ')', bar( // (("), 2);
        assert_eq!(bracket_balance("foldl' (+) 0 xs"), 0);
        assert_eq!(bracket_balance("let singleton (x : 'a) = Node (x)"), 0);
        assert_eq!(bracket_balance("(provide 'widget)"), 0);
        assert_eq!(bracket_balance("(defvar modes '(text-mode prog-mode)"), 1);
        assert!(is_annotation_only("@Deprecated("));
        assert!(is_annotation_only(
            "[UnconditionalSuppressMessage(\"x\", \"y\")]"
        ));
        assert!(!is_annotation_only("@Override public void run() {"));
        assert!(!is_annotation_only("@interface Foo : NSObject"));
    }

    /// An abstract member is the API a trait or interface defines and
    /// outranks members with bodies; private members, test cases and
    /// control flow come last.
    #[test]
    fn plaintext_source_text_surface_ranks_signatures_before_bodies_and_internals() {
        let mut scala = vec!["trait Functor {".to_string()];
        for n in 0..SOURCE_TEXT_DECL_LINES {
            scala.push(format!("  def helper{n}: Int ="));
            scala.push(format!("    {n}"));
        }
        scala.extend(
            [
                "  private def hidden(): Int = 0",
                "  def map[A, B](fa: F[A])(f: A => B): F[B]",
                "  boolean isValid(String email);",
                "}",
            ]
            .map(String::from),
        );
        let scala = scala.join("\n");
        let text = surface(&scala);
        assert!(
            text.contains(&"  def map[A, B](fa: F[A])(f: A => B): F[B]"),
            "{text:?}"
        );
        assert!(
            text.contains(&"  boolean isValid(String email);"),
            "{text:?}"
        );
        assert!(
            !text.contains(&"  private def hidden(): Int = 0"),
            "{text:?}"
        );

        for (line, rank) in [
            ("rescue LoadError", DeclarationRank::Internal),
            ("comptime {", DeclarationRank::Internal),
            (
                "fun noCache(): Boolean = noCache",
                DeclarationRank::OneLiner,
            ),
            (
                "treeChecks :: [Parameters -> Token]",
                DeclarationRank::Heading,
            ),
            ("return foo(x);", DeclarationRank::Internal),
            ("foo(bar);", DeclarationRank::Statement),
            ("val url: HttpUrl =", DeclarationRank::OneLiner),
        ] {
            assert_eq!(declaration_rank(line, false), rank, "{line}");
        }
    }

    /// Code inside a heredoc or a triple-quoted string is data, not the
    /// file's declarations.
    #[test]
    fn plaintext_source_text_surface_skips_multiline_string_interiors() {
        let php = "<?php\nclass A {\n    function f() { return <<<'EOF'\nif (x) {\n    y();\n}\nEOF;\n    }\n}\n";
        let text = surface(php);
        assert_eq!(
            text,
            vec!["<?php", "class A {", "    function f() { return <<<'EOF'"]
        );

        let elixir = "defmodule Router do\n  @doc \"\"\"\n  ## Examples\n\n  from any scope.\n  \"\"\"\n  def pipeline(plug) do\n    plug\n  end\nend\n";
        let text = surface(elixir);
        assert!(!text.contains(&"  ## Examples"), "{text:?}");
        assert!(text.contains(&"  def pipeline(plug) do"), "{text:?}");

        let shift = "int mask = 1<<BITS;\nint next() {\n  return 1;\n}\n";
        assert!(
            string_interiors(&shift.lines().collect::<Vec<_>>())
                .iter()
                .all(|inside| !inside)
        );

        let closer_above_opener = "EOF\nx = <<EOF\n  EOF_NOT\nEOF;\nEOF\n";
        assert_eq!(
            string_interiors(&closer_above_opener.lines().collect::<Vec<_>>()),
            vec![false, false, true, true, false]
        );
    }

    /// Descent reaches past one wrapper line (a Ruby `module`) and a doc
    /// comment's ` * ` level to the members below, one-liners included, but
    /// not into statement bodies, and never below column zero when column
    /// zero already declares (CSS selectors, whose `#main` is no comment).
    #[test]
    fn plaintext_source_text_surface_descent() {
        let ruby = "# frozen_string_literal: true\n\nmodule Devise\n  class Mapping\n\
                        def self.find_scope!(obj)\n      obj\n    end\n\
                    \n    def initialize(name)\n      @name = name\n    end\n  end\nend\n";
        let cpp = "namespace leveldb {\n\nStatus BuildTable(const std::string& dbname, Env* env,\n\
                   \x20                 Iterator* iter) {\n  Status s;\n  if (iter->Valid()) {\n\
                   \x20   s = Write();\n  }\n  return s;\n}\n\n}  // namespace leveldb\n";
        let kotlin = "object Config {\n    fun host() = \"localhost\"\n    fun port() = 8080\n}\n";
        let java = "/**\n * A client.\n */\npublic class Client {\n    public void open() {\n\
                    \x20       connect();\n    }\n}\n";
        let css = "a {\n  color: red;\n}\n\nh1,\nh2 {\n  margin: 0;\n}\n\n\
                   .card {\n  padding: 1rem;\n}\n\n#main {\n  display: flex;\n}\n";
        let cases: &[(&str, &[&str], &[&str])] = &[
            (
                ruby,
                &["module Devise", "class Mapping", "def self.find_scope!"],
                &["frozen_string_literal"],
            ),
            (
                cpp,
                &["namespace leveldb {", "Status BuildTable("],
                &["Status s;", "iter->Valid()", "return s;"],
            ),
            (
                kotlin,
                &["object Config {", "fun host()", "fun port()"],
                &[],
            ),
            (java, &["public void open() {"], &[]),
            (
                css,
                &["#main {"],
                &["color", "margin", "padding", "display"],
            ),
        ];
        for (source, present, absent) in cases {
            let text = surface(source);
            let has = |needle: &str| text.iter().any(|line| line.contains(needle));
            for needle in *present {
                assert!(has(needle), "missing {needle:?}: {text:?}");
            }
            for needle in *absent {
                assert!(!has(needle), "leaked {needle:?}: {text:?}");
            }
        }
    }

    #[test]
    fn plaintext_source_text_flat_file_degrades_to_a_head_slice() {
        let prose = (1..=40)
            .map(|n| format!("Paragraph line {n}."))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(
            surface(&prose),
            prose
                .lines()
                .take(SOURCE_TEXT_DECL_LINES)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn plaintext_source_text_classification_rejects_derived_files() {
        let language = Some(Class::LanguageSource);
        let flat = Some(Class::FlatText);
        #[rustfmt::skip]
        let cases = [
            ("Gson.java", language), ("Session.swift", language), ("Layout.vue", language),
            ("Gemfile", language), ("schema.proto", language), ("guide.rst", None),
            ("app.css", flat), ("build.sh", flat),
            // Derived artifacts never render.
            ("app.min.css", None), ("vendor.bundle.css", None), ("main.js.map", None),
            ("pnpm-lock.yaml", None),
            // `.md` files stay with the markdown walker, and formats an owning
            // walker already claims stay with it.
            ("LICENSE.md", None), ("lib.rs", None), ("main.py", None), ("app.ts", None),
            ("go.mod", None), ("index.html", None), ("pom.xml", None),
        ];
        for (name, expected) in cases {
            assert_eq!(classify_source_text(name), expected, "{name}");
        }
    }

    #[test]
    fn plaintext_credential_names_are_data_and_config_files() {
        #[rustfmt::skip]
        let cases = [
            (".env", true), (".env.local", true), (".env.local.sh", true), ("production.env", true),
            ("docker.env", true), ("env.sh", true), ("secrets.yml", true), ("secrets.prod.sh", true),
            ("secret.txt", true), ("credentials.json", true), ("credentials-dev.ini", true),
            ("creds_staging.conf", true), ("secrets", true), ("prod.tfvars", true),
            ("prod.tfvars.json", true), ("service-account.json", true),
            ("serviceAccountKey.json", true), (".npmrc", true), (".netrc", true),
            (".pypirc", true), (".git-credentials", true), (".htpasswd", true),
            // Samples document keys with placeholder values.
            (".env.example", false), (".env.sample", false), (".env.template", false),
            (".env.dist", false),
            // Code and docs about credentials hold none.
            ("credentials.py", false), ("secrets.rs", false), ("credentials.go", false),
            ("env.d.ts", false), ("secrets.md", false), ("credentials.rst", false),
            (".env.local.example", false), ("secrets.yml.sample", false),
            ("service-account.yaml", false), ("environment.yml", false), ("config.json", false),
        ];
        for (name, expected) in cases {
            assert_eq!(is_credential_name(Path::new(name)), expected, "{name}");
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
        // refuse — owned by other walkers, or out of scope.
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

    /// The rows each build file renders at the root and one level down. A
    /// small Makefile renders whole anywhere; one too long, or too wide for
    /// the byte gate, renders at the root only its `.PHONY` declarations
    /// (through their continuation lines), and a long root Dockerfile only
    /// its head.
    #[test]
    fn plaintext_build_file_rows() {
        let recipes = |prefix: &str| -> String {
            (0..60)
                .map(|index| format!("target-{index:02}: dep-{index:02}\n\tRECIPE_{index:02}\n"))
                .fold(prefix.to_string(), |source, recipe| source + &recipe)
        };
        let dockerfile = (0..BUILD_LINE_CAP)
            .map(|index| format!("RUN step-{index}\n"))
            .fold("ARG BASE=ubuntu:24.04\n".to_string(), |source, step| {
                source + &step
            });
        let cases = [
            (
                "Makefile",
                "FLAGS = --all\n\nbuild:\n\ttool $(FLAGS)\n".to_string(),
                vec![(1, 4)],
                vec![(1, 4)],
            ),
            (
                "Makefile",
                format!("build: {}\n", "dependency ".repeat(BUILD_BYTE_GATE)),
                vec![],
                vec![],
            ),
            (
                "Makefile",
                recipes(
                    "BUILD_DEPS = common-a common-b\n.PHONY: build test\n\nbuild: $(BUILD_DEPS)\n",
                ),
                vec![(2, 2)],
                vec![],
            ),
            (
                "Makefile",
                recipes(
                    ".PHONY: build \\\n\ttest lint\nSHELL = /bin/sh\n.PHONY: \\\n\trelease \\\n\tdocs\n\n",
                ),
                vec![(1, 2), (4, 6)],
                vec![],
            ),
            (
                "Dockerfile",
                dockerfile,
                vec![(1, SOURCE_TEXT_DECL_LINES)],
                vec![],
            ),
        ];
        for (name, source, root_rows, nested_rows) in cases {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path();
            std::fs::create_dir(root.join("nested")).unwrap();
            std::fs::write(root.join(name), &source).unwrap();
            std::fs::write(root.join("nested").join(name), &source).unwrap();
            let ctx = WalkCtx::new(root.to_path_buf());
            let rows = |dir: &Path| -> Vec<(usize, usize)> {
                expand_in_dir(dir, &ctx)
                    .iter()
                    .flat_map(|batch| match &batch.content {
                        crate::content::BatchContent::Lines { spans, .. } => spans.clone(),
                        crate::content::BatchContent::Fs { .. } => Vec::new(),
                    })
                    .map(|span| (span.start, span.end))
                    .collect()
            };
            assert_eq!(rows(root), root_rows, "{name}: {source:.40}");
            assert_eq!(rows(&root.join("nested")), nested_rows, "nested {name}");
        }
    }

    /// A root `pom.xml` leads with its coordinates, not the `<project
    /// xmlns=…>` opener or the blocks after them; a module's is left to
    /// the listing unless it is the project's `src/`.
    #[test]
    fn plaintext_root_manifest_surface_is_its_head_fields() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir(root.join("module")).unwrap();
        std::fs::create_dir(root.join("src")).unwrap();
        let mut source = format!(
            "<?xml version=\"1.0\"?>\n<project xmlns=\"{}\">\n  <modelVersion>4.0.0</modelVersion>\n\
             \n  <groupId>org.example</groupId>\n  <artifactId>app</artifactId>\n\
             \n  <dependencies>\n",
            "x".repeat(SOURCE_TEXT_MAX_LINE_CHARS)
        );
        for index in 0..PLAINTEXT_LINE_CAP {
            source.push_str(&format!(
                "    <dependency>\n      <artifactId>lib-{index}</artifactId>\n    </dependency>\n"
            ));
        }
        source.push_str("  </dependencies>\n</project>\n");
        std::fs::write(root.join("pom.xml"), &source).unwrap();
        std::fs::write(root.join("module/pom.xml"), &source).unwrap();
        std::fs::write(root.join("src/pom.xml"), &source).unwrap();

        let ctx = WalkCtx::new(root.to_path_buf());
        let batches = expand_in_dir(root, &ctx);
        assert_eq!(batches.len(), 1);
        let crate::content::BatchContent::Lines { spans, .. } = &batches[0].content else {
            panic!("expected a lines batch");
        };
        let rows: Vec<_> = spans.iter().map(|span| (span.start, span.end)).collect();
        let closing = source.lines().count();
        assert_eq!(rows, [(1, 1), (3, 8), (closing - 1, closing)]);
        assert!(expand_in_dir(&root.join("module"), &ctx).is_empty());
        assert_eq!(expand_in_dir(&root.join("src"), &ctx).len(), 1);
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

    /// Drive the full `FsWalker` + scheduler against a real directory. A
    /// named file renders whole, its first `PLAINTEXT_LINE_CAP` lines when
    /// it runs longer, and never when its bytes overflow the gate. Once the
    /// walkers' batches are all scheduled, a file no walker claims renders
    /// its head; a license text, credentials and hidden files stay names,
    /// and a sidecar the listing leaves out stays unnamed.
    #[test]
    fn plaintext_real_dir_renders_named_files_and_the_floor() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let tool_versions: String = std::iter::once("rust 1.80.0\n".to_string())
            .chain((1..PLAINTEXT_LINE_CAP + 5).map(|i| format!("line {i}\n")))
            .collect();
        let files = [
            (".tool-versions", tool_versions.as_str()),
            (".gitignore", &format!("{}\n", "x".repeat(250)).repeat(60)),
            ("chapter.tex", "\\section{Intro}\nFirst words.\n"),
            ("LICENSE", "MIT License\n\nCopyright (c) Yoav\n"),
            ("secrets.yml", "api_token: abc\n"),
            ("secrets.production.yaml", "db_password: hunter2\n"),
            ("config/credentials.local.json", "{\"token\": \"t0k\"}\n"),
            (
                "deploy_key",
                &format!(
                    "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n",
                    "MIIEv".repeat(13)
                ),
            ),
            (
                "signing-key.asc",
                &format!(
                    "-----BEGIN PGP PRIVATE KEY BLOCK-----\n\nlQVYBGXpayload{}\n=ab12\n\
                     -----END PGP PRIVATE KEY BLOCK-----\n",
                    "A".repeat(50)
                ),
            ),
            (".github/labels.yml", "- name: bug\n"),
            ("sprite.png.meta", "guid: 0123\n"),
        ];
        for (path, text) in files {
            std::fs::create_dir_all(root.join(path).parent().unwrap()).unwrap();
            std::fs::write(root.join(path), text).unwrap();
        }
        std::fs::write(root.join("sprite.png"), [0x89, b'P', b'N', b'G', 0]).unwrap();

        let scheduler = Scheduler::new(WalkCtx::new(root.to_path_buf()), FsWalker, 4_000, None);
        let report = scheduler.run_with_report();
        let rendered = report.tree.render();
        for shown in ["rust 1.80.0", "First words."] {
            assert!(rendered.contains(shown), "{rendered}");
        }
        for hidden in [
            "MIT License",
            "api_token",
            "hunter2",
            "t0k",
            "BEGIN PRIVATE KEY",
            "PGP PRIVATE KEY",
            "payload",
            "name: bug",
            "guid",
            "xxxx",
        ] {
            assert!(!rendered.contains(hidden), "{hidden:?} in {rendered}");
        }
        let whole_rows = |suffix: &str| -> Vec<(usize, usize)> {
            report
                .scheduled
                .iter()
                .filter(|record| {
                    matches!(&record.key, BatchKey::Plaintext(PlaintextKey::Whole { file })
                        if file.ends_with(suffix))
                })
                .flat_map(|record| match &record.content {
                    crate::content::BatchContent::Lines { spans, .. } => spans.clone(),
                    crate::content::BatchContent::Fs { .. } => Vec::new(),
                })
                .map(|span| (span.start, span.end))
                .collect()
        };
        assert_eq!(whole_rows(".tool-versions"), [(1, PLAINTEXT_LINE_CAP)]);
        assert_eq!(whole_rows(".gitignore"), []);
        assert_eq!(whole_rows("LICENSE"), []);
    }

    /// A file the code engine parses is never also a fallback file —
    /// two walkers claiming its rows would overlap.
    #[test]
    fn plaintext_fallback_skips_files_the_code_engine_parses() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("App.jsx"), "export const X = 1;\n").unwrap();
        std::fs::write(root.join("App.kt"), "class App\n").unwrap();
        std::fs::write(root.join("api.h"), "int api(void);\n").unwrap();
        std::fs::write(root.join("widget.h"), "namespace lib { class Widget; }\n").unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let mut files: Vec<_> = expand_in_dir(root, &ctx)
            .into_iter()
            .map(|batch| batch.key)
            .collect();
        files.sort();
        assert_eq!(
            files,
            ["App.kt", "widget.h"].map(|name| BatchKey::from(PlaintextKey::DeclSurface {
                file: root.join(name)
            }))
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

    /// A small file in an unparsed language renders whole once the budget
    /// reaches past its surface — through its closing brace, not four lines
    /// and a `…` — past its license banner, keeping the doc comment that
    /// follows the banner.
    #[test]
    fn plaintext_small_source_text_file_renders_whole_past_its_banner() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(
            root.join("Foo.java"),
            "/*\n * Copyright (C) 2012 The Authors\n */\n\n/**\n * Escapers and encoders.\n */\n\
             package demo;\n\npublic class Foo {\n    int x;\n}\n",
        )
        .unwrap();
        let scheduler = Scheduler::new(WalkCtx::new(root.to_path_buf()), FsWalker, 4_000, None);
        let rendered = scheduler.run().render();
        assert!(!rendered.contains("Copyright"), "{rendered}");
        for line in ["6→ * Escapers and encoders.", "8→package demo;", "12→}"] {
            assert!(rendered.contains(line), "{rendered}");
        }
    }
}
