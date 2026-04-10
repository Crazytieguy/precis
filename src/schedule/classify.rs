use std::collections::HashSet;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// File role classification
// ---------------------------------------------------------------------------

/// Role of the file a symbol lives in, for separating high-value files
/// (README) from low-value files (CHANGELOG) in markdown grouping.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum FileRole {
    /// README.md, readme.md, etc.
    Readme,
    /// CHANGELOG.md, CHANGES.md, HISTORY.md, NEWS.md, etc.
    Changelog,
    /// Translated/localized files (e.g., README_zh-CN.md, README-es.md).
    Translated,
    /// Community health files: CONTRIBUTING.md, LICENSE.md, SECURITY.md, CODE_OF_CONDUCT.md, etc.
    CommunityHealth,
    /// AI coding assistant config files: CLAUDE.md, AGENTS.md, COPILOT.md, etc.
    AiConfig,
    /// Architecture/design documentation: ARCHITECTURE.md, DESIGN.md.
    /// Body content of these files is the most valuable information in a repo
    /// (design philosophy, discarded approaches, code patterns).
    Architecture,
    /// Everything else.
    Normal,
}

impl FileRole {
    pub fn from_filename(name: &str) -> Self {
        let lower = name.to_ascii_lowercase();
        // Strip document extension for matching
        let (stem, is_doc) = lower.strip_suffix(".md").map(|s| (s, true))
            .or_else(|| lower.strip_suffix(".markdown").map(|s| (s, true)))
            .or_else(|| lower.strip_suffix(".rst").map(|s| (s, true)))
            .or_else(|| lower.strip_suffix(".txt").map(|s| (s, true)))
            .unwrap_or((&lower, false));
        match stem {
            "readme" => FileRole::Readme,
            "changelog" | "changes" | "history" | "news" | "releases"
            | "breaking_changes" | "breaking-changes" | "migration" | "upgrading"
            | "release_notes" | "release-notes" => FileRole::Changelog,
            "contributing" | "contributors" | "security" | "license" | "licence"
            | "code_of_conduct" | "codeowners" | "releasing" | "support"
            | "governance" | "authors" | "maintainers"
            | "tidelift" | "sponsors" | "funding"
            | "notice" | "citation" => FileRole::CommunityHealth,
            "claude" | "agents" | "copilot" | "copilot-instructions"
            | "cursor" | "windsurf" => FileRole::AiConfig,
            "architecture" | "design" => FileRole::Architecture,
            _ if is_doc && has_locale_suffix(stem) => FileRole::Translated,
            _ => FileRole::Normal,
        }
    }

    /// Determine file role from the full relative path.
    /// Checks both the filename and directory components for locale patterns.
    /// Locale directories override filename-based detection (a translated README
    /// is low value, not high value).
    pub fn from_path(path: &Path) -> Self {
        let ext_owned = path.extension().and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase());
        let ext = ext_owned.as_deref().unwrap_or("");
        let is_doc = matches!(ext, "md" | "markdown" | "rst" | "txt");
        let is_data = matches!(ext, "json" | "yaml" | "yml" | "toml" | "properties" | "strings" | "xlf" | "po" | "pot");
        // For doc and data files, check if any parent directory is a locale directory
        // (e.g., docs/zh-CN/guide.md, i18n/es/readme.md, locales/ar.json).
        // This takes priority over filename-based detection.
        if is_doc || is_data {
            for component in path.components() {
                if let std::path::Component::Normal(seg) = component
                    && let Some(s) = seg.to_str()
                    && is_locale_dir(s)
                {
                    return FileRole::Translated;
                }
            }
        }
        path.file_name()
            .and_then(|n| n.to_str())
            .map(FileRole::from_filename)
            .unwrap_or(FileRole::Normal)
    }
}

/// Detect locale suffixes like `_zh-CN`, `-es`, `.ja`, `_pt-BR`.
/// Matches `[_.-]xx` or `[_.-]xx[-_]yy` at end of stem, where xx/yy are
/// 2-letter ASCII alpha codes (ISO 639-1 language / ISO 3166-1 region).
fn has_locale_suffix(stem: &str) -> bool {
    let bytes = stem.as_bytes();
    let len = bytes.len();
    // Minimum: separator + 2-letter code = 3 chars, plus at least 1 char before
    if len < 4 {
        return false;
    }
    let is_sep = |b: u8| b == b'_' || b == b'-' || b == b'.';
    let is_alpha = |b: u8| b.is_ascii_lowercase();

    // Try `[sep]xx[-_]yy` at end (6 chars)
    if len >= 7 && is_sep(bytes[len - 6]) && is_alpha(bytes[len - 5])
        && is_alpha(bytes[len - 4]) && (bytes[len - 3] == b'-' || bytes[len - 3] == b'_')
        && is_alpha(bytes[len - 2]) && is_alpha(bytes[len - 1])
    {
        return true;
    }
    // Try `[sep]xx` at end (3 chars)
    if is_sep(bytes[len - 3]) && is_alpha(bytes[len - 2]) && is_alpha(bytes[len - 1]) {
        return true;
    }
    false
}

/// Detect locale directory names like `zh-CN`, `pt-BR`, `en-US`,
/// or well-known i18n directories like `i18n`, `l10n`, `locales`, `translations`.
/// Only matches the `xx-YY` / `xx_YY` pattern (not bare 2-letter codes, which
/// have too many false positives like `go`, `js`, `ci`).
fn is_locale_dir(name: &str) -> bool {
    let bytes = name.as_bytes();
    // Well-known i18n directory names
    let lower = name.to_ascii_lowercase();
    if matches!(lower.as_str(), "i18n" | "l10n" | "locales" | "locale" | "translations") {
        return true;
    }
    // xx-YY or xx_YY (5 chars): e.g. zh-CN, pt-BR, en-US
    if bytes.len() == 5
        && bytes[0].is_ascii_alphabetic()
        && bytes[1].is_ascii_alphabetic()
        && (bytes[2] == b'-' || bytes[2] == b'_')
        && bytes[3].is_ascii_alphabetic()
        && bytes[4].is_ascii_alphabetic()
    {
        return true;
    }
    false
}

// ---------------------------------------------------------------------------
// Content classification heuristics
// ---------------------------------------------------------------------------

/// Detect heading depth from source lines for a markdown section symbol.
/// ATX headings (`# h1`, `## h2`, etc.) are detected by counting leading `#` chars.
/// Setext headings (underline with `=` or `-`) are detected by checking the underline.
pub(super) fn detect_heading_depth(lines: &[&str], sym_line_0: usize, end_line: usize) -> u8 {
    let trimmed = lines[sym_line_0].trim_start();
    if trimmed.starts_with('#') {
        let depth = trimmed.bytes().take_while(|&b| b == b'#').count();
        (depth as u8).clamp(1, 6)
    } else {
        // Setext heading: check underline character
        let underline_idx = end_line.min(lines.len()) - 1;
        if underline_idx > sym_line_0
            && lines[underline_idx].trim_start().starts_with('=')
        {
            1
        } else {
            2 // '-' underline = h2
        }
    }
}

/// Detect auto-generated API documentation READMEs (pdoc, sphinx, etc.).
/// These files contain structured class/method listings that duplicate the
/// signatures already extracted from the source code files they document.
/// Detection: README files whose first non-blank lines contain HTML anchor
/// tags (`<a id=`, `<a name=`), which doc generators insert for navigation.
pub(super) fn is_autogen_api_doc(source: &str, file_role: FileRole) -> bool {
    if file_role != FileRole::Readme {
        return false;
    }
    source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(3)
        .any(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("<a id=") || trimmed.starts_with("<a name=")
        })
}

/// Detect auto-generated files by checking for common generator markers
/// in the first few lines (e.g., "Code generated by", "DO NOT EDIT",
/// "auto-generated", "This file is generated").
pub(super) fn is_generated_file(source: &str) -> bool {
    source
        .lines()
        .take(5)
        .any(|line| {
            let lower = line.to_ascii_lowercase();
            (lower.contains("generated") || lower.contains("auto-generated"))
                && (lower.contains("do not edit")
                    || lower.contains("do not modify")
                    || lower.contains("generated by")
                    || lower.contains("auto-generated"))
        })
}

/// Detect generated files by filename patterns (protobuf, codegen, etc.).
pub(super) fn is_generated_filename(path: &Path) -> bool {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    // Protobuf: *_pb2.py, *.pb.go, *_grpc.py
    stem.ends_with("_pb2") || stem.ends_with("_pb2_grpc")
        || name.ends_with(".pb.go") || name.ends_with(".pb.cc") || name.ends_with(".pb.h")
        // Codegen: *.generated.ts, *.g.dart
        || stem.ends_with(".generated") || stem.ends_with(".g")
        // Mock generators: mock_*.go (mockery)
        || (stem.starts_with("mock_") && name.ends_with(".go"))
}

/// Detect boilerplate markdown section headings that convey no architectural value.
/// These are headings like "## License", "## Contributing", "## Acknowledgements"
/// that appear in READMEs across many repos with identical content.
pub(super) fn is_boilerplate_heading(name: &str) -> bool {
    // Strip leading emoji/punctuation to handle "📃 License", "🤝 Contributing", etc.
    let stripped = name.trim_start_matches(|c: char| !c.is_ascii_alphanumeric());
    let lower = stripped.trim().to_ascii_lowercase();
    // Also handle "License - MIT" style suffixed headings
    let stem = lower.split(['-', '—', '–']).next().unwrap_or(&lower).trim();
    matches!(
        stem,
        "license" | "licence"
            | "contribute" | "contributing" | "contributors"
            | "code of conduct"
            | "acknowledgments" | "acknowledgements" | "credits"
            | "author" | "authors" | "maintainers"
            | "support" | "governance" | "security"
            | "sponsors" | "backers" | "funding" | "donate" | "donations"
            | "changelog" | "release notes" | "releases" | "history"
            | "related" | "alternatives"
            | "faq"
            | "table of contents" | "contents"
            | "star history" | "stargazers"
            | "development" | "developing"
            | "community"
    )
}

/// Whether a directory name is a conventional documentation root (`docs`, `doc`).
fn is_docs_dir_name(name: &str) -> bool {
    matches!(name.to_ascii_lowercase().as_str(), "docs" | "doc")
}

/// Detect directories that are documentation site roots (Sphinx, MkDocs,
/// Docusaurus) by scanning for doc generator config files. Returns a set of
/// relative directory paths whose contents should be classified as `DocsSite`.
///
/// Inferred directories (e.g. `docs/` from a root `mkdocs.yml`) may not
/// actually exist in the file list — callers use `starts_with` matching,
/// so phantom entries are harmless.
pub(crate) fn detect_doc_site_dirs<'a>(
    relative_paths: impl Iterator<Item = &'a Path>,
) -> HashSet<PathBuf> {
    let mut dirs = HashSet::new();
    for relative in relative_paths {
        let filename = match relative.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let parent = relative.parent().unwrap_or(Path::new(""));

        // Sphinx: conf.py inside a docs/ or doc/ directory
        if filename == "conf.py"
            && let Some(parent_name) = parent.file_name().and_then(|n| n.to_str())
            && is_docs_dir_name(parent_name)
        {
            dirs.insert(parent.to_path_buf());
        }

        // MkDocs: mkdocs.yml/yaml at root infers docs/ as the default content
        // dir. Non-root mkdocs.yml (monorepo sub-packages) is still classified
        // as config by is_config_file, but we don't infer a sibling docs/ dir
        // since the content directory may be configured differently.
        if matches!(filename, "mkdocs.yml" | "mkdocs.yaml")
            && parent.as_os_str().is_empty()
        {
            dirs.insert(PathBuf::from("docs"));
        }

        // Docusaurus: docusaurus.config.{js,ts,mjs} → sibling docs/
        if filename.starts_with("docusaurus.config.") {
            dirs.insert(parent.join("docs"));
        }
    }
    dirs
}

/// Detect files that are not core library/application source code.
/// Includes build/tool configuration, stylesheets, HTML templates, web assets,
/// and files in conventional tooling directories. These get a reduced value
/// factor (0.2) so they appear when budget allows but don't displace source code.
pub(super) fn is_config_file(relative_path: &Path, filename: &str) -> bool {
    let lower = filename.to_ascii_lowercase();
    let is_root = relative_path.parent().is_none_or(|p| p.as_os_str().is_empty());
    let ext = lower.rsplit_once('.').map(|(_, e)| e);

    // Build scripts and packaging files — only at project root.
    // build.rs: Rust build scripts (compile-time codegen, feature probing)
    // setup.py: Python setuptools packaging
    // These filenames are generic enough that in subdirs they may be regular source files
    // (e.g., src/cmd/build.rs implements a "build" CLI subcommand).
    if is_root {
        match lower.as_str() {
            "build.rs" | "setup.py" | "setup.cfg" | "tox.ini" | "pytest.ini" => return true,
            _ => {}
        }
    }

    // Package metadata and compiler config — always metadata/tooling, never architecture.
    // package.json: npm manifest (scripts, devDependencies, repository, keywords, etc.)
    // tsconfig*.json: TypeScript compiler configuration (compilerOptions, include, exclude)
    // jsconfig*.json: JavaScript project configuration
    // These consume significant budget in small libraries while providing zero
    // architectural value — the source code and README are far more informative.
    match lower.as_str() {
        "package.json" => return true,
        _ if (lower.starts_with("tsconfig") || lower.starts_with("jsconfig"))
            && ext == Some("json") => return true,
        _ => {}
    }

    // Conda/pip environment files (environment_*.yaml, environment.yml, etc.)
    // These are dependency manifests with zero architectural value.
    if (lower.starts_with("environment") || lower.starts_with("requirements"))
        && matches!(ext, Some("yaml" | "yml" | "txt"))
    {
        return true;
    }

    // Build/task runners — matched by filename anywhere (unambiguous names).
    match lower.as_str() {
        "gulpfile.js" | "gruntfile.js" | "jakefile.js"
        | "make.lua" | "premake5.lua" | "cmake.lua"
        | "cmakelists.txt" => return true,
        _ => {}
    }

    // Doc generator configs contain theme settings, plugin lists, and build
    // options — tooling setup with no architectural signal. Without this,
    // mkdocs.yml body text about theme palettes wastes budget.
    match lower.as_str() {
        "mkdocs.yml" | "mkdocs.yaml" => return true,
        "conf.py" if relative_path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .is_some_and(is_docs_dir_name)
        => return true,
        _ if lower.starts_with("docusaurus.config.") => return true,
        _ => {}
    }

    // TOML files that aren't Cargo.toml are tool/service configuration.
    // pyproject.toml: section names are useful but body content (classifiers,
    // dependency-groups, URLs, build-system) wastes budget vs source code.
    // Cargo.toml is the exception — [dependencies] body reveals the tech stack.
    if ext == Some("toml") && lower != "cargo.toml" {
        return true;
    }

    // Dotfiles with data-format extensions are tool/service configuration
    // (e.g., .travis.yml, .codecov.yml, .release-please-manifest.json).
    if lower.starts_with('.') && matches!(ext, Some("json" | "yml" | "yaml")) {
        return true;
    }

    // *.config.{ext} — catches eslint.config.js, jest.config.ts, vite.config.mjs,
    // next.config.js, tailwind.config.ts, postcss.config.js, tsup.config.ts,
    // playwright.config.ts, rollup.config.js, webpack.config.js, babel.config.js, etc.
    if let Some(stem) = lower.rsplit_once('.').map(|(s, _)| s) {
        if stem.ends_with(".config") {
            return true;
        }
        // .{name}rc.{ext} — catches .eslintrc.js, .babelrc.js, .prettierrc.mjs, etc.
        if stem.starts_with('.') && stem.ends_with("rc") {
            return true;
        }
        // *-config.{data-ext} and *_config.{data-ext} — catches
        // release-please-config.json and similar tool/service config files.
        // Restricted to data-format extensions to avoid matching code files
        // like my_config.py which may contain configuration logic.
        if (stem.ends_with("-config") || stem.ends_with("_config"))
            && matches!(ext, Some("json" | "yml" | "yaml"))
        {
            return true;
        }
    }

    // Container orchestration and CI service config files.
    if lower.starts_with("docker-compose") || lower.starts_with("compose.") {
        return true;
    }

    // Dockerfiles (with or without extension variants like Dockerfile.prod)
    if lower.starts_with("dockerfile") {
        return true;
    }

    // Web asset and metadata files — committed but not source code.
    match lower.as_str() {
        "robots.txt" | "humans.txt" | "sitemap.xml" | "favicon.ico"
        | "site.webmanifest" | "manifest.json" | "browserconfig.xml" => return true,
        _ => {}
    }

    // Stylesheets, HTML templates, and SVG assets are not source code architecture.
    if matches!(ext, Some("css" | "scss" | "sass" | "less" | "html" | "htm" | "svg")) {
        return true;
    }

    // Specific well-known repo management config files that don't match
    // the patterns above (not TOML, not dotfiles, no *-config pattern).
    match lower.as_str() {
        "codecov.yml" | "codecov.yaml" | "renovate.json" | "package-support.json"
        | "biome.json" | "biome.jsonc" | "deno.json" | "deno.jsonc"
        | "taskfile.yml" | "taskfile.yaml" => return true,
        _ => {}
    }

    // Files in conventional tooling directories are development utilities
    // (release scripts, build helpers, CI glue), not core library/app code.
    // Only `scripts/` and `tools/` — these are unambiguous across ecosystems.
    for component in relative_path.components() {
        if let std::path::Component::Normal(seg) = component
            && let Some(name) = seg.to_str()
        {
            match name.to_ascii_lowercase().as_str() {
                "scripts" | "script" | "tools" | "tool" => return true,
                _ => {}
            }
        }
    }

    false
}

// ---------------------------------------------------------------------------
// File category classification
// ---------------------------------------------------------------------------

/// Role of a file relative to the core library/app code.
/// Files in different categories get different value factors — examples
/// demonstrate library usage (moderately valuable), while test infrastructure
/// and CI config are low-signal.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum FileCategory {
    /// Core library/application source code.
    Source,
    /// Example/demo code showing how to use the library.
    Example,
    /// Test files, benchmarks, fixtures, mocks.
    Test,
    /// Documentation site source (Docusaurus, Sphinx, etc.).
    DocsSite,
    /// CI/CD configuration (.github/, .circleci/).
    CiConfig,
}

/// Classify a file's role relative to core library/app code.
/// Used by the scheduler to apply different value factors per category.
pub fn classify_file(path: &Path) -> FileCategory {
    // Check directory components for category signals (single pass).
    for component in path.components() {
        let s = component.as_os_str();

        // Examples and experiments — demonstrate usage, moderately valuable
        if s == "examples" || s == "example" || s == "experiments" || s == "experiment" {
            return FileCategory::Example;
        }

        // Documentation sites, supplementary docs, and changelog fragments.
        if s == "website" || s == "site" || s == "rfcs" || s == "rfc"
            || s == "changelog" || s == "changelogs"
        {
            return FileCategory::DocsSite;
        }

        // CI/CD configuration
        if s == ".github" || s == ".circleci" || s == ".gitlab" {
            return FileCategory::CiConfig;
        }

        // Test/build infrastructure — tests, benchmarks, fixtures, mocks, changelogs
        if s == "__tests__" || s == "tests" || s == "test" || s == "testing"
            || s == "benches" || s == "benchmark" || s == "benchmarks"
            || s == "fixtures" || s == "fixture"
            || s == "mocks" || s == "__mocks__"
            || s == "stories" || s == "__stories__" || s == ".storybook"
        {
            return FileCategory::Test;
        }

        // Compound directory names with test-related segments
        // (workspace crates like "foo-test-utils", "bench-helpers")
        if let Some(name) = s.to_str()
            && (name.contains('-') || name.contains('_'))
            && name.split(['-', '_']).any(|seg| {
                matches!(
                    seg,
                    "test" | "tests" | "testing" | "bench" | "benches"
                    | "benchmark" | "benchmarks" | "mock" | "mocks"
                    | "fixture" | "fixtures"
                )
            })
        {
            return FileCategory::Test;
        }
    }

    // Check filename conventions for test files
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        && (stem.ends_with(".test")
            || stem.ends_with(".test-d")
            || stem.ends_with(".spec")
            || stem.ends_with(".stories")
            || stem.starts_with("test_")
            || stem.ends_with("_test")
            || stem == "conftest")
    {
        return FileCategory::Test;
    }

    FileCategory::Source
}

/// Check if a file is a TypeScript declaration file (.d.ts, .d.mts, .d.cts).
/// These contain type signatures that duplicate the API already shown from
/// .js/.ts source files, so they are deprioritized by the scheduler.
pub fn is_type_declaration_file(path: &Path) -> bool {
    let name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return false,
    };
    name.ends_with(".d.ts") || name.ends_with(".d.mts") || name.ends_with(".d.cts")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_file_detection() {
        assert!(is_generated_file("// Code generated by mockery; DO NOT EDIT.\npackage mocks"));
        assert!(is_generated_file("# auto-generated by protoc\n# DO NOT EDIT"));
        assert!(is_generated_file("/* Generated by the gRPC code generator. Do not modify. */"));
        // Normal files should not match
        assert!(!is_generated_file("// This is a normal Go file\npackage main"));
        assert!(!is_generated_file("# Generated thoughtfully by a human\ndef foo(): pass"));
        assert!(!is_generated_file("pub fn main() {}"));
    }

    #[test]
    fn file_role_community_health() {
        assert_eq!(FileRole::from_filename("CONTRIBUTING.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("contributing.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("CONTRIBUTORS.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("SECURITY.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("LICENSE.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("LICENCE.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("License.txt"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("CODE_OF_CONDUCT.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("CODEOWNERS"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("RELEASING.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("SUPPORT.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("GOVERNANCE.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("AUTHORS"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("AUTHORS.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("MAINTAINERS.md"), FileRole::CommunityHealth);
    }

    #[test]
    fn generated_filename_detection() {
        assert!(is_generated_filename(Path::new("service_pb2.py")));
        assert!(is_generated_filename(Path::new("api.pb.go")));
        assert!(is_generated_filename(Path::new("mock_repository.go")));
        assert!(is_generated_filename(Path::new("types.generated.ts")));
        assert!(is_generated_filename(Path::new("model.g.dart")));
        // gRPC generated files
        assert!(is_generated_filename(Path::new("api_pb2_grpc.py")));
        assert!(is_generated_filename(Path::new("service.pb.cc")));
        assert!(is_generated_filename(Path::new("types.pb.h")));
        // Normal files should not match
        assert!(!is_generated_filename(Path::new("main.go")));
        assert!(!is_generated_filename(Path::new("utils.py")));
        assert!(!is_generated_filename(Path::new("mock.ts"))); // no underscore
        assert!(!is_generated_filename(Path::new("mockup.go"))); // no underscore after mock
        assert!(!is_generated_filename(Path::new("generator.ts"))); // "generated" not in stem
    }

    #[test]
    fn boilerplate_heading_detection() {
        assert!(is_boilerplate_heading("License"));
        assert!(is_boilerplate_heading("## License"));
        assert!(is_boilerplate_heading("License - MIT"));
        assert!(is_boilerplate_heading("📃 License"));
        assert!(is_boilerplate_heading("Contributing"));
        assert!(is_boilerplate_heading("Acknowledgements"));
        assert!(is_boilerplate_heading("FAQ"));
        assert!(is_boilerplate_heading("Changelog"));
        assert!(is_boilerplate_heading("Alternatives"));
        // Non-boilerplate headings
        assert!(!is_boilerplate_heading("Installation"));
        assert!(!is_boilerplate_heading("Usage"));
        assert!(!is_boilerplate_heading("API"));
        assert!(!is_boilerplate_heading("Architecture"));
        assert!(!is_boilerplate_heading("Features"));
    }

    #[test]
    fn file_role_architecture() {
        assert_eq!(FileRole::from_filename("ARCHITECTURE.md"), FileRole::Architecture);
        assert_eq!(FileRole::from_filename("architecture.md"), FileRole::Architecture);
        assert_eq!(FileRole::from_filename("DESIGN.md"), FileRole::Architecture);
        assert_eq!(FileRole::from_filename("design.md"), FileRole::Architecture);
        // CONTEXT.md is ambiguous — could be AI context or project docs.
        // Classified as Normal (not AiConfig) to avoid hiding genuine documentation.
        assert_eq!(FileRole::from_filename("CONTEXT.md"), FileRole::Normal);
        assert_eq!(FileRole::from_filename("context.md"), FileRole::Normal);
        // Non-doc extensions should not match
        assert_eq!(FileRole::from_filename("architecture.rs"), FileRole::Normal);
        assert_eq!(FileRole::from_filename("context.py"), FileRole::Normal);
        // RST extension should be recognized
        assert_eq!(FileRole::from_filename("README.rst"), FileRole::Readme);
        assert_eq!(FileRole::from_filename("CHANGELOG.rst"), FileRole::Changelog);
    }

    #[test]
    fn file_role_existing_roles() {
        assert_eq!(FileRole::from_filename("README.md"), FileRole::Readme);
        assert_eq!(FileRole::from_filename("CHANGELOG.md"), FileRole::Changelog);
        assert_eq!(FileRole::from_filename("CHANGES.md"), FileRole::Changelog);
        assert_eq!(FileRole::from_filename("README_zh-CN.md"), FileRole::Translated);
        assert_eq!(FileRole::from_filename("README-es.md"), FileRole::Translated);
        assert_eq!(FileRole::from_filename("lib.rs"), FileRole::Normal);
        assert_eq!(FileRole::from_filename("main.py"), FileRole::Normal);
    }

    #[test]
    fn file_role_ai_config() {
        assert_eq!(FileRole::from_filename("CLAUDE.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("claude.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("AGENTS.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("agents.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("COPILOT.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("copilot-instructions.md"), FileRole::AiConfig);
        // Non-doc extensions should not match
        assert_eq!(FileRole::from_filename("claude.toml"), FileRole::Normal);
        assert_eq!(FileRole::from_filename("agents.json"), FileRole::Normal);
    }

    #[test]
    fn file_role_from_path_locale_dirs() {
        // Locale directory patterns (xx-YY) detect translated docs
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/guide.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("docs/pt-BR/readme.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("docs/en_US/intro.md")), FileRole::Translated);
        // Well-known i18n directory names
        assert_eq!(FileRole::from_path(Path::new("i18n/guide.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("l10n/guide.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("locales/readme.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("translations/guide.md")), FileRole::Translated);
        // Data files in locale dirs are also Translated
        assert_eq!(FileRole::from_path(Path::new("locales/ar.json")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("src/locales/en.json")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("i18n/de.yaml")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("locales/fr.yml")), FileRole::Translated);
        // Non-doc non-data files in locale dirs stay Normal (e.g., source code)
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/lib.rs")), FileRole::Normal);
        assert_eq!(FileRole::from_path(Path::new("locales/index.ts")), FileRole::Normal);
        // Plain 2-letter dirs are NOT matched (too many false positives)
        assert_eq!(FileRole::from_path(Path::new("docs/go/guide.md")), FileRole::Normal);
        assert_eq!(FileRole::from_path(Path::new("docs/js/guide.md")), FileRole::Normal);
        // Locale directory overrides filename-based roles (translated README is low value)
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/README.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/CHANGELOG.md")), FileRole::Translated);
        // Root-level files work normally
        assert_eq!(FileRole::from_path(Path::new("README.md")), FileRole::Readme);
        assert_eq!(FileRole::from_path(Path::new("guide.md")), FileRole::Normal);
    }

    #[test]
    fn config_file_detection() {
        // Helper: root-level file (no parent dir)
        fn at_root(name: &str) -> bool {
            is_config_file(Path::new(name), name)
        }
        // Helper: file in a subdirectory
        fn at_path(path: &str) -> bool {
            let p = Path::new(path);
            let name = p.file_name().unwrap().to_str().unwrap();
            is_config_file(p, name)
        }

        // *.config.{ext} pattern (matched anywhere)
        assert!(at_root("eslint.config.js"));
        assert!(at_root("jest.config.ts"));
        assert!(at_root("vite.config.mjs"));
        assert!(at_root("next.config.js"));
        assert!(at_root("next.config.mjs"));
        assert!(at_root("tailwind.config.ts"));
        assert!(at_root("postcss.config.js"));
        assert!(at_root("tsup.config.ts"));
        assert!(at_root("playwright.config.ts"));
        assert!(at_root("rollup.config.js"));
        assert!(at_root("webpack.config.js"));
        assert!(at_root("babel.config.js"));
        assert!(at_root("vitest.config.ts"));
        assert!(at_root("theme.config.jsx"));
        // Case insensitive
        assert!(at_root("ESLint.Config.JS"));
        // .{name}rc.{ext} pattern
        assert!(at_root(".eslintrc.js"));
        assert!(at_root(".babelrc.js"));
        assert!(at_root(".prettierrc.mjs"));
        // Build scripts at project root
        assert!(at_root("build.rs"));
        assert!(at_root("Build.rs")); // case insensitive
        assert!(at_root("setup.py"));
        assert!(at_root("Setup.py"));
        // Build scripts in subdirs are NOT config files (e.g., src/cmd/build.rs)
        assert!(!at_path("src/cmd/build.rs"));
        assert!(!at_path("lib/setup.py"));
        // JS task runners (matched anywhere)
        assert!(at_root("Gruntfile.js"));
        assert!(at_root("gruntfile.js"));
        assert!(at_root("Gulpfile.js"));
        assert!(at_root("gulpfile.js"));
        assert!(at_root("Jakefile.js"));
        assert!(at_path("tools/Gulpfile.js")); // task runners match in subdirs too
        // TOML files that aren't project manifests
        assert!(at_root("triagebot.toml"));
        assert!(at_root("rust-toolchain.toml"));
        assert!(at_root("netlify.toml"));
        assert!(at_root("fly.toml"));
        assert!(at_path("guide/book.toml")); // tool config in subdirs too
        assert!(at_path("packages/app-server/wrangler.toml"));
        // Cargo.toml is NOT config (dependencies body is useful)
        assert!(!at_root("Cargo.toml"));
        assert!(!at_path("crates/core/Cargo.toml"));
        // pyproject.toml IS config (body content wastes budget)
        assert!(at_root("pyproject.toml"));
        // Dotfiles with data-format extensions
        assert!(at_root(".travis.yml"));
        assert!(at_root(".gitlab-ci.yml"));
        assert!(at_root(".codecov.yml"));
        assert!(at_root(".release-please-manifest.json"));
        assert!(at_root(".renovaterc.json")); // also caught by .{name}rc pattern
        // *-config and *_config patterns (data-format extensions only)
        assert!(at_root("release-please-config.json"));
        assert!(at_root("app-config.yaml"));
        assert!(at_root("my_config.json"));
        assert!(!at_root("my_config.py")); // code file, not data config
        // Specific well-known repo management files
        assert!(at_root("codecov.yml"));
        assert!(at_root("codecov.yaml"));
        assert!(at_root("renovate.json"));
        // Package metadata and compiler config
        assert!(at_root("package.json"));
        assert!(at_path("packages/my-lib/package.json")); // monorepo sub-packages too
        assert!(at_root("tsconfig.json"));
        assert!(at_root("tsconfig.build.json"));
        assert!(at_root("tsconfig.node.json"));
        assert!(at_root("jsconfig.json"));
        assert!(at_path("packages/app/tsconfig.json")); // monorepo sub-packages too
        // Conda/pip environment files
        assert!(at_root("environment.yaml"));
        assert!(at_root("environment_pt220cu121.yaml"));
        assert!(at_root("environment.yml"));
        assert!(at_root("requirements.txt"));
        assert!(at_root("requirements-dev.txt"));
        // Not config files
        assert!(!at_root("main.js"));
        assert!(!at_root("lib.rs"));
        assert!(!at_root("index.ts"));
        assert!(!at_root("config.js")); // no *.config.* pattern (code file)
        assert!(!at_root("README.md"));
        assert!(!at_root("build.go")); // build.rs is Rust-specific
        assert!(!at_root("setup.rs")); // setup.py is Python-specific
        assert!(at_root("compose.yaml")); // container orchestration = config
        assert!(!at_root("pnpm-workspace.yaml")); // monorepo structure
        assert!(!at_root("reference.yaml")); // data file
        // Files in scripts/ and tools/ directories
        assert!(at_path("scripts/release.py"));
        assert!(at_path("scripts/build.sh"));
        assert!(at_path("tools/lint.py"));
        assert!(at_path("tool/gen.rs"));
        assert!(at_path("script/deploy.js"));
        // Nested scripts dir also matches
        assert!(at_path("ci/scripts/test.sh"));
        // Case insensitive
        assert!(at_path("Scripts/release.py"));
        // Normal source dirs are NOT matched
        assert!(!at_path("src/main.rs"));
        assert!(!at_path("lib/index.js"));
        assert!(!at_path("pkg/server.go"));
    }

    #[test]
    fn classify_file_detection() {
        // Test directories
        assert_eq!(classify_file(Path::new("__tests__/helper.ts")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("tests/integration.rs")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("test/setup.ts")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("testing/helpers.py")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("benches/bench.rs")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("benchmark/run.py")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("benchmarks/perf.rs")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("fixtures/setup.py")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("fixture/helpers.ts")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("mocks/mock_repo.go")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("__mocks__/utils.ts")), FileCategory::Test);
        // Example directories
        assert_eq!(classify_file(Path::new("examples/basic.rs")), FileCategory::Example);
        assert_eq!(classify_file(Path::new("example/demo.ts")), FileCategory::Example);
        assert_eq!(classify_file(Path::new("experiments/train.py")), FileCategory::Example);
        assert_eq!(classify_file(Path::new("experiment/run.py")), FileCategory::Example);
        // Documentation site directories
        assert_eq!(classify_file(Path::new("website/src/App.tsx")), FileCategory::DocsSite);
        assert_eq!(classify_file(Path::new("site/pages/index.tsx")), FileCategory::DocsSite);
        // docs/ and doc/ are Source — they often contain valuable API reference
        assert_eq!(classify_file(Path::new("docs/conf.py")), FileCategory::Source);
        assert_eq!(classify_file(Path::new("doc/guide.md")), FileCategory::Source);
        // CI/CD directories
        assert_eq!(classify_file(Path::new(".github/workflows/ci.yml")), FileCategory::CiConfig);
        assert_eq!(classify_file(Path::new(".circleci/config.yml")), FileCategory::CiConfig);
        // Test file naming conventions
        assert_eq!(classify_file(Path::new("index.test.ts")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("utils.spec.ts")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("test_utils.py")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("conftest.py")), FileCategory::Test);
        // Compound directory names with test-related segments
        assert_eq!(classify_file(Path::new("crates/foo-test-utils/src/lib.rs")), FileCategory::Test);
        // "integration" alone is not a test signal (could be "api-integration" source)
        assert_eq!(classify_file(Path::new("crates/my-integration-suite/src/setup.rs")), FileCategory::Source);
        // But "integration-test" IS a test signal
        assert_eq!(classify_file(Path::new("crates/integration-test-utils/src/lib.rs")), FileCategory::Test);
        assert_eq!(classify_file(Path::new("crates/my-mock-server/src/lib.rs")), FileCategory::Test);
        // Changelog directories
        assert_eq!(classify_file(Path::new("changelog/README.rst")), FileCategory::DocsSite);
        assert_eq!(classify_file(Path::new("changelogs/1234.md")), FileCategory::DocsSite);
        // Storybook directories
        assert_eq!(classify_file(Path::new("stories/Button.stories.tsx")), FileCategory::Test);
        assert_eq!(classify_file(Path::new(".storybook/config.js")), FileCategory::Test);
        // contribute/ is NOT deprioritized (too generic a name)
        assert_eq!(classify_file(Path::new("contribute/demo.py")), FileCategory::Source);
        // RFC directories
        assert_eq!(classify_file(Path::new("rfcs/0001-design.md")), FileCategory::DocsSite);
        // GitLab CI
        assert_eq!(classify_file(Path::new(".gitlab/ci.yml")), FileCategory::CiConfig);
        // Normal source files
        assert_eq!(classify_file(Path::new("src/main.rs")), FileCategory::Source);
        assert_eq!(classify_file(Path::new("index.ts")), FileCategory::Source);
        assert_eq!(classify_file(Path::new("lib/utils.py")), FileCategory::Source);
        assert_eq!(classify_file(Path::new("crates/toasty-core/src/lib.rs")), FileCategory::Source);
    }

    #[test]
    fn type_declaration_file_detection() {
        // TypeScript declaration files
        assert!(is_type_declaration_file(Path::new("index.d.ts")));
        assert!(is_type_declaration_file(Path::new("typings/index.d.ts")));
        assert!(is_type_declaration_file(Path::new("lib/types.d.mts")));
        assert!(is_type_declaration_file(Path::new("utils.d.cts")));
        // Normal source files (not declarations)
        assert!(!is_type_declaration_file(Path::new("index.ts")));
        assert!(!is_type_declaration_file(Path::new("index.js")));
        assert!(!is_type_declaration_file(Path::new("test.d.py"))); // wrong extension
        assert!(!is_type_declaration_file(Path::new("src/main.rs")));
    }

    #[test]
    fn doc_site_dir_detection() {
        let detect = |paths: &[&str]| -> HashSet<PathBuf> {
            let owned: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
            detect_doc_site_dirs(owned.iter().map(|p| p.as_path()))
        };

        // Sphinx: conf.py in docs/ or doc/
        let dirs = detect(&["docs/conf.py", "docs/api_reference.rst", "src/main.py"]);
        assert!(dirs.contains(Path::new("docs")));
        assert!(!dirs.contains(Path::new("src")));

        // conf.py NOT in a docs-like directory → no detection
        assert!(detect(&["src/conf.py"]).is_empty());

        // MkDocs: mkdocs.yml at root → docs/ is detected
        assert!(detect(&["mkdocs.yml", "docs/guide.md"]).contains(Path::new("docs")));

        // Docusaurus: docusaurus.config.js → sibling docs/
        assert!(detect(&["docusaurus.config.js"]).contains(Path::new("docs")));

        // Docusaurus in a subdirectory → docs/ sibling
        assert!(detect(&["website/docusaurus.config.ts"]).contains(Path::new("website/docs")));

        // No doc generator config → empty
        assert!(detect(&["docs/guide.md", "src/main.rs"]).is_empty());
    }

    #[test]
    fn config_file_doc_generators() {
        fn at_root(name: &str) -> bool {
            is_config_file(Path::new(name), name)
        }
        fn at_path(path: &str) -> bool {
            let p = Path::new(path);
            let name = p.file_name().unwrap().to_str().unwrap();
            is_config_file(p, name)
        }

        // MkDocs config
        assert!(at_root("mkdocs.yml"));
        assert!(at_root("mkdocs.yaml"));
        assert!(at_path("packages/site/mkdocs.yml"));
        // Docusaurus config
        assert!(at_root("docusaurus.config.js"));
        assert!(at_root("docusaurus.config.ts"));
        assert!(at_root("docusaurus.config.mjs"));
        // Sphinx conf.py in docs/ directories
        assert!(at_path("docs/conf.py"));
        assert!(at_path("doc/conf.py"));
        // conf.py NOT in docs/ is NOT config (could be regular Python module)
        assert!(!at_path("src/conf.py"));
        assert!(!at_root("conf.py"));
    }
}
