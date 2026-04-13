use std::path::Path;


// ---------------------------------------------------------------------------
// File role classification
// ---------------------------------------------------------------------------

/// Role of the file a symbol lives in, for separating high-value files
/// (README) from low-value files (CHANGELOG) in markdown grouping.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum FileRole {
    Readme,
    Changelog,
    Translated,
    CommunityHealth,
    AiConfig,
    Architecture,
    Normal,
}

impl FileRole {
    pub fn from_filename(name: &str) -> Self {
        let lower = name.to_ascii_lowercase();
        let (stem, is_doc) = lower
            .strip_suffix(".md")
            .map(|s| (s, true))
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
            | "cursor" | "windsurf" | "context" => FileRole::AiConfig,
            "architecture" | "design" => FileRole::Architecture,
            _ if is_doc && has_locale_suffix(stem) => FileRole::Translated,
            _ => FileRole::Normal,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let ext_owned = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase());
        let ext = ext_owned.as_deref().unwrap_or("");
        let is_doc = matches!(ext, "md" | "markdown" | "rst" | "txt");
        let is_data = matches!(
            ext,
            "json" | "yaml" | "yml" | "toml" | "properties" | "strings" | "xlf" | "po" | "pot"
        );
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

fn has_locale_suffix(stem: &str) -> bool {
    let bytes = stem.as_bytes();
    let len = bytes.len();
    if len < 4 {
        return false;
    }
    let is_sep = |b: u8| b == b'_' || b == b'-' || b == b'.';
    let is_alpha = |b: u8| b.is_ascii_lowercase();

    if len >= 7
        && is_sep(bytes[len - 6])
        && is_alpha(bytes[len - 5])
        && is_alpha(bytes[len - 4])
        && (bytes[len - 3] == b'-' || bytes[len - 3] == b'_')
        && is_alpha(bytes[len - 2])
        && is_alpha(bytes[len - 1])
    {
        return true;
    }
    if is_sep(bytes[len - 3]) && is_alpha(bytes[len - 2]) && is_alpha(bytes[len - 1]) {
        return true;
    }
    false
}

fn is_locale_dir(name: &str) -> bool {
    let bytes = name.as_bytes();
    let lower = name.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "i18n" | "l10n" | "locales" | "locale" | "translations"
    ) {
        return true;
    }
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
// File category classification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum FileCategory {
    Source,
    Example,
    Test,
    DocsSite,
    CiConfig,
}

fn classify_by_components(path: &Path) -> Option<FileCategory> {
    for component in path.components() {
        let s = component.as_os_str();

        if s == "examples" || s == "example" || s == "experiments" || s == "experiment" {
            return Some(FileCategory::Example);
        }
        if s == "website"
            || s == "site"
            || s == "docs"
            || s == "doc"
            || s == "rfcs"
            || s == "rfc"
            || s == "changelog"
            || s == "changelogs"
        {
            return Some(FileCategory::DocsSite);
        }
        if s == ".github" || s == ".circleci" || s == ".gitlab" {
            return Some(FileCategory::CiConfig);
        }
        if s == "__tests__"
            || s == "tests"
            || s == "test"
            || s == "testing"
            || s == "benches"
            || s == "benchmark"
            || s == "benchmarks"
            || s == "fixtures"
            || s == "fixture"
            || s == "mocks"
            || s == "__mocks__"
            || s == "stories"
            || s == "__stories__"
            || s == ".storybook"
        {
            return Some(FileCategory::Test);
        }
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
            return Some(FileCategory::Test);
        }
    }
    None
}

pub fn classify_dir(path: &Path) -> FileCategory {
    classify_by_components(path).unwrap_or(FileCategory::Source)
}

// ---------------------------------------------------------------------------
// Content classification heuristics
// ---------------------------------------------------------------------------

pub fn detect_heading_depth(lines: &[&str], sym_line_0: usize, end_line: usize) -> u8 {
    let trimmed = lines[sym_line_0].trim_start();
    if trimmed.starts_with('#') {
        let depth = trimmed.bytes().take_while(|&b| b == b'#').count();
        (depth as u8).clamp(1, 6)
    } else {
        let underline_idx = end_line.min(lines.len()) - 1;
        if underline_idx > sym_line_0 && lines[underline_idx].trim_start().starts_with('=') {
            1
        } else {
            2
        }
    }
}

pub fn is_autogen_api_doc(source: &str, file_role: FileRole) -> bool {
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

pub fn is_generated_file(source: &str) -> bool {
    source.lines().take(5).any(|line| {
        let lower = line.to_ascii_lowercase();
        (lower.contains("generated") || lower.contains("auto-generated"))
            && (lower.contains("do not edit")
                || lower.contains("do not modify")
                || lower.contains("generated by")
                || lower.contains("auto-generated"))
    })
}

pub fn is_generated_filename(path: &Path) -> bool {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    stem.ends_with("_pb2")
        || stem.ends_with("_pb2_grpc")
        || name.ends_with(".pb.go")
        || name.ends_with(".pb.cc")
        || name.ends_with(".pb.h")
        || stem.ends_with(".generated")
        || stem.ends_with(".g")
        || (stem.starts_with("mock_") && name.ends_with(".go"))
}

pub fn is_boilerplate_heading(name: &str) -> bool {
    let stripped = name.trim_start_matches(|c: char| !c.is_ascii_alphanumeric());
    let lower = stripped.trim().to_ascii_lowercase();

    let full_match = |s: &str| {
        matches!(
            s,
            "license"
                | "licence"
                | "contribute"
                | "contributing"
                | "contributors"
                | "code of conduct"
                | "acknowledgments"
                | "acknowledgements"
                | "credits"
                | "author"
                | "authors"
                | "maintainers"
                | "support"
                | "governance"
                | "security"
                | "sponsors"
                | "backers"
                | "funding"
                | "donate"
                | "donations"
                | "changelog"
                | "release notes"
                | "releases"
                | "history"
                | "related"
                | "alternatives"
                | "faq"
                | "table of contents"
                | "contents"
                | "star history"
                | "stargazers"
                | "development"
                | "developing"
                | "community"
        )
    };

    let stem = lower.split(['-', '—', '–']).next().unwrap_or(&lower).trim();
    if full_match(stem) {
        return true;
    }
    let first_word = lower.split_whitespace().next().unwrap_or(&lower);
    full_match(first_word)
}

fn is_docs_dir_name(name: &str) -> bool {
    matches!(name.to_ascii_lowercase().as_str(), "docs" | "doc")
}

pub fn is_deprioritized_file(relative_path: &Path, filename: &str) -> bool {
    let lower = filename.to_ascii_lowercase();
    let is_root = relative_path
        .parent()
        .is_none_or(|p| p.as_os_str().is_empty());
    let ext = lower.rsplit_once('.').map(|(_, e)| e);

    if is_root {
        match lower.as_str() {
            "build.rs" | "setup.py" | "setup.cfg" | "tox.ini" | "pytest.ini" => return true,
            _ => {}
        }
    }

    match lower.as_str() {
        "package.json" => return true,
        _ if (lower.starts_with("tsconfig") || lower.starts_with("jsconfig"))
            && ext == Some("json") =>
        {
            return true
        }
        _ => {}
    }

    if (lower.starts_with("environment") || lower.starts_with("requirements"))
        && matches!(ext, Some("yaml" | "yml" | "txt"))
    {
        return true;
    }

    match lower.as_str() {
        "gulpfile.js" | "gruntfile.js" | "jakefile.js" | "make.lua" | "premake5.lua"
        | "cmake.lua" | "cmakelists.txt" => return true,
        _ => {}
    }

    match lower.as_str() {
        "mkdocs.yml" | "mkdocs.yaml" => return true,
        "conf.py"
            if relative_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .is_some_and(is_docs_dir_name) =>
        {
            return true
        }
        _ if lower.starts_with("docusaurus.config.") => return true,
        _ => {}
    }

    if ext == Some("toml") && lower != "cargo.toml" {
        return true;
    }

    if lower.starts_with('.') && matches!(ext, Some("json" | "yml" | "yaml")) {
        return true;
    }

    if let Some(stem) = lower.rsplit_once('.').map(|(s, _)| s) {
        if stem.ends_with(".config") {
            return true;
        }
        if stem.starts_with('.') && stem.ends_with("rc") {
            return true;
        }
        if (stem.ends_with("-config") || stem.ends_with("_config"))
            && matches!(ext, Some("json" | "yml" | "yaml"))
        {
            return true;
        }
    }

    if lower.starts_with("docker-compose") || lower.starts_with("compose.") {
        return true;
    }
    if lower.starts_with("dockerfile") {
        return true;
    }

    match lower.as_str() {
        "robots.txt" | "humans.txt" | "sitemap.xml" | "favicon.ico" | "site.webmanifest"
        | "manifest.json" | "browserconfig.xml" => return true,
        _ => {}
    }

    if matches!(
        ext,
        Some("css" | "scss" | "sass" | "less" | "html" | "htm" | "svg")
    ) {
        return true;
    }

    match lower.as_str() {
        "codecov.yml" | "codecov.yaml" | "renovate.json" | "package-support.json"
        | "biome.json" | "biome.jsonc" | "deno.json" | "deno.jsonc" | "taskfile.yml"
        | "taskfile.yaml" => return true,
        _ => {}
    }

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

pub fn is_type_declaration_file(path: &Path) -> bool {
    let name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return false,
    };
    name.ends_with(".d.ts") || name.ends_with(".d.mts") || name.ends_with(".d.cts")
}

/// Check if a file extension indicates a C/C++ header file.
pub fn is_header_extension(ext: &str) -> bool {
    matches!(ext, "h" | "hpp" | "hxx" | "hh")
}

/// Check if a file path has a C/C++ header extension.
pub fn is_header_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| is_header_extension(&ext.to_ascii_lowercase()))
}

/// Compute per-file modifier properties used to partition files into groups.
pub fn file_modifier_properties(relative_path: &Path) -> (bool, bool, bool, bool) {
    let filename = relative_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    (
        is_deprioritized_file(relative_path, filename),
        is_type_declaration_file(relative_path),
        is_header_file(relative_path),
        is_colocated_test_file(filename),
    )
}

pub fn is_colocated_test_file(filename: &str) -> bool {
    let lower = filename.to_ascii_lowercase();
    if lower.ends_with("_test.go") {
        return true;
    }
    if lower.starts_with("test_") && lower.ends_with(".py") {
        return true;
    }
    if let Some(stem) = lower.strip_suffix(".py")
        && stem.ends_with("_test")
    {
        return true;
    }
    let test_infixes = [".test.", ".spec.", "_test.", "_spec."];
    for infix in test_infixes {
        if lower.contains(infix) {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// File discovery helpers (migrated from walk.rs)
// ---------------------------------------------------------------------------

/// Returns true for files likely to be text-readable source, excluding
/// known binary formats and generated/minified artifacts.
pub fn is_source_file(path: &Path) -> bool {
    if is_lockfile(path) {
        return false;
    }
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        return !is_binary_extension(&ext.to_ascii_lowercase());
    }
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| {
            matches!(
                name,
                "Makefile"
                    | "Dockerfile"
                    | "Containerfile"
                    | "Justfile"
                    | "Gemfile"
                    | "Rakefile"
            )
        })
}

fn is_binary_extension(ext: &str) -> bool {
    matches!(
        ext,
        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" | "tiff" | "tif"
        | "psd" | "ai" | "eps" | "raw" | "cr2" | "nef" | "heic" | "avif"
        // Fonts
        | "ttf" | "otf" | "woff" | "woff2" | "eot"
        // Audio / video
        | "mp3" | "wav" | "ogg" | "flac" | "aac" | "wma" | "m4a"
        | "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm"
        // Archives / compressed
        | "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst"
        // Executables / libraries
        | "exe" | "dll" | "so" | "dylib" | "a" | "o" | "obj" | "lib"
        | "wasm" | "class" | "pyc" | "pyo"
        // Documents (binary)
        | "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt"
        // Data / ML (binary)
        | "db" | "sqlite" | "sqlite3"
        | "bin" | "dat" | "npy" | "npz" | "h5" | "hdf5" | "parquet"
        | "safetensors" | "onnx" | "pt" | "pth" | "pkl" | "pickle"
        // Design / 3D
        | "sketch" | "fig" | "blend" | "fbx" | "stl" | "gltf" | "glb"
        // Disk images / packages
        | "iso" | "dmg" | "deb" | "rpm" | "msi" | "apk" | "ipa"
        // Generated (lockfiles, etc.)
        | "lock" | "lockb"
    )
}

fn is_lockfile(path: &Path) -> bool {
    let name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return false,
    };
    let lower = name.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "package-lock.json" | "npm-shrinkwrap.json" | "pnpm-lock.yaml"
    ) || lower.ends_with(".min.js")
        || lower.ends_with(".min.css")
        || lower.ends_with(".bundle.js")
        || lower.ends_with(".chunk.js")
}

pub fn strip_heading_badges(line: &str) -> &str {
    if let Some(pos) = line.find(" [![") {
        let before = line[..pos].trim();
        if !before.is_empty() && before != "#" {
            return line[..pos].trim_end();
        }
    }
    line
}

pub fn is_vendored_or_fixture(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str();
        s == "vendor" || s == "node_modules" || s == "deps" || s == "testdata"
    })
}

// ---------------------------------------------------------------------------
// Effective depth
// ---------------------------------------------------------------------------

pub fn effective_depth(parent_dir: &Path) -> usize {
    let is_root =
        |s: &str| matches!(s, "src" | "source" | "lib" | "pkg" | "cmd" | "internal" | "app" | "packages" | "crates");
    let components: Vec<_> = parent_dir
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    match components.as_slice() {
        [] => 0,
        [first, ..] if is_root(first) => {
            let rest = &components[1..];
            if rest.len() >= 2 && is_root(rest[1]) {
                rest.len() - 1
            } else {
                rest.len()
            }
        }
        _ => components.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_depth_skips_source_roots() {
        assert_eq!(effective_depth(Path::new("")), 0);
        assert_eq!(effective_depth(Path::new("src")), 0);
        assert_eq!(effective_depth(Path::new("lib")), 0);
        assert_eq!(effective_depth(Path::new("pkg")), 0);
        assert_eq!(effective_depth(Path::new("cmd")), 0);
        assert_eq!(effective_depth(Path::new("src/pluggy")), 1);
        assert_eq!(effective_depth(Path::new("lib/internal")), 1);
        assert_eq!(effective_depth(Path::new("src/a/b")), 2);
        assert_eq!(effective_depth(Path::new("internal")), 0);
        assert_eq!(effective_depth(Path::new("internal/pkg")), 1);
        assert_eq!(effective_depth(Path::new("packages")), 0);
        assert_eq!(effective_depth(Path::new("packages/foo/src")), 1);
        assert_eq!(effective_depth(Path::new("crates/my-crate/src/utils")), 2);
        assert_eq!(effective_depth(Path::new("crates/core")), 1);
        assert_eq!(effective_depth(Path::new("docs")), 1);
        assert_eq!(effective_depth(Path::new("scripts")), 1);
        assert_eq!(effective_depth(Path::new("docs/conf")), 2);
    }

    #[test]
    fn generated_file_detection() {
        assert!(is_generated_file(
            "// Code generated by mockery; DO NOT EDIT.\npackage mocks"
        ));
        assert!(is_generated_file(
            "# auto-generated by protoc\n# DO NOT EDIT"
        ));
        assert!(!is_generated_file(
            "// This is a normal Go file\npackage main"
        ));
        assert!(!is_generated_file("pub fn main() {}"));
    }
}
