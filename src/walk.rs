use crate::parse;
use ignore::Walk;
use std::path::{Path, PathBuf};

/// Discover source files under `root`, respecting .gitignore.
/// Returns paths sorted for deterministic output.
/// Includes test/benchmark/example files (deprioritized by the scheduler)
/// but excludes vendored dependencies and test fixture data.
pub fn discover_source_files(root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Walk::new(root)
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .filter(|entry| is_source_file(entry.path()))
        .filter(|entry| {
            let relative = entry.path().strip_prefix(root).unwrap_or(entry.path());
            !is_vendored_or_fixture(relative)
        })
        .map(|entry| entry.into_path())
        .collect();
    files.sort();
    files
}

fn is_source_file(path: &Path) -> bool {
    // Check extension-based matching first (covers most files)
    let ext_match = path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| parse::is_supported_extension(ext) || is_unsupported_code_extension(ext));
    if ext_match {
        return !is_lockfile(path);
    }
    // Well-known extensionless files (Makefile, Dockerfile, etc.)
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| matches!(name, "Makefile" | "Dockerfile" | "Vagrantfile" | "Rakefile" | "Gemfile" | "Brewfile" | "Procfile" | "Justfile" | "Earthfile" | "Containerfile"))
}

/// Common programming language extensions without tree-sitter support.
/// Files with these extensions are included in discovery and rendered as
/// plain text (first line + body), giving visibility into multi-language
/// codebases where only some languages have full parser support.
///
/// Deliberately excludes non-code text files that cause noise:
/// shell scripts (.sh), stylesheets (.css/.scss), HTML templates,
/// LaTeX (.tex), and shader code (.glsl).
/// RST is included because README.rst and other project docs are
/// valuable even without tree-sitter parsing (rendered as plain text).
fn is_unsupported_code_extension(ext: &str) -> bool {
    matches!(
        ext,
        // Systems languages
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" | "cs" | "m" | "mm" | "swift"
        // JVM languages
        | "java" | "kt" | "kts" | "scala" | "groovy" | "clj" | "cljs"
        // Scripting languages
        | "lua" | "rb" | "pl" | "pm" | "php"
        // ML/functional languages
        | "hs" | "ml" | "mli" | "fs" | "fsi" | "ex" | "exs" | "erl"
        // Modern languages
        | "dart" | "zig" | "nim" | "r" | "jl"
        // GPU languages
        | "cu" | "cuh"
        // Web component languages (contain logic + template)
        | "vue" | "svelte"
        // Query/schema languages
        | "sql" | "graphql" | "gql" | "proto"
        // Documentation markup (README.rst etc. are valuable as plain text)
        | "rst"
        // Go module file (go.mod lists dependencies and Go version)
        | "mod"
        // Config files with informative metadata (setup.cfg, .ini)
        | "cfg" | "ini"
    )
}

/// Check if a file is an auto-generated lockfile that should be excluded.
/// Lockfiles are machine-generated, often huge, and contain no human-authored
/// information. Only lists lockfiles whose extensions are in the supported set
/// (json, yaml); other lockfiles use `.lock`/`.lockb` extensions that are
/// already filtered out by `is_source_file`.
fn is_lockfile(path: &Path) -> bool {
    let name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return false,
    };
    matches!(
        name,
        "package-lock.json" | "npm-shrinkwrap.json" | "pnpm-lock.yaml" | "composer.lock"
    )
}

/// Check if a file is vendored third-party code or test fixture data that should
/// be completely excluded from output. These are never authored by the project.
fn is_vendored_or_fixture(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str();
        // vendor/ (vendored third-party dependencies: Go, PHP, Ruby)
        // node_modules/ (npm dependencies, usually gitignored but just in case)
        // deps/ (vendored C/C++ dependencies, e.g. amalgamated single-file libs)
        // testdata/ (Go convention for test fixture data, excluded by go build)
        s == "vendor" || s == "node_modules" || s == "deps" || s == "testdata"
    })
}

/// Classify a file's role relative to core library/app code.
/// Used by the scheduler to apply different value factors per category.
pub fn classify_file(path: &Path) -> crate::schedule::FileCategory {
    use crate::schedule::FileCategory;

    // Check directory components for category signals (single pass).
    for component in path.components() {
        let s = component.as_os_str();

        // Examples and experiments — demonstrate usage, moderately valuable
        if s == "examples" || s == "example" || s == "experiments" || s == "experiment" {
            return FileCategory::Example;
        }

        // Documentation sites and supplementary design docs.
        // docs/ and doc/ are ambiguous (often contain valuable API reference),
        // so only website/, site/, and rfcs/ are classified as doc-site source.
        if s == "website" || s == "site" || s == "rfcs" || s == "rfc" {
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
            || s == "changelog" || s == "changelogs"
            || s == "contribute"
            || s == "stories" || s == "__stories__" || s == ".storybook"
        {
            return FileCategory::Test;
        }

        // Compound directory names with test-related segments
        // (workspace crates like "foo-test-utils", "my-integration-suite")
        if let Some(name) = s.to_str()
            && (name.contains('-') || name.contains('_'))
            && name.split(['-', '_']).any(|seg| {
                matches!(
                    seg,
                    "test" | "tests" | "testing" | "bench" | "benches"
                    | "benchmark" | "benchmarks" | "mock" | "mocks"
                    | "fixture" | "fixtures" | "integration"
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
            || stem.starts_with("test_")
            || stem.ends_with("_test")
            || stem == "conftest")
    {
        return FileCategory::Test;
    }

    FileCategory::Source
}

/// Check if a file extension indicates a C/C++ header file.
pub fn is_header_extension(ext: &str) -> bool {
    matches!(ext, "h" | "hpp" | "hxx" | "hh")
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
    use std::fs;

    #[test]
    fn discovers_files_in_own_project() {
        let files = discover_source_files(Path::new(env!("CARGO_MANIFEST_DIR")));
        let relative: Vec<_> = files
            .iter()
            .map(|p| {
                p.strip_prefix(env!("CARGO_MANIFEST_DIR"))
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();
        assert!(relative.contains(&"src/main.rs".to_string()));
        assert!(relative.contains(&"src/walk.rs".to_string()));
        // target/ should be excluded via .gitignore
        assert!(!relative.iter().any(|p| p.starts_with("target/")));
        // tests/ are now included (deprioritized by scheduler, not excluded)
        assert!(relative.iter().any(|p| p.starts_with("tests/")));
    }

    #[test]
    fn filters_non_source_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("lib.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("readme.txt"), "hi").unwrap();
        fs::write(dir.path().join("data.csv"), "a,b,c").unwrap();

        let files = discover_source_files(dir.path());
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("lib.rs"));
    }

    #[test]
    fn includes_config_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("lib.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(dir.path().join("config.toml"), "[section]").unwrap();
        fs::write(dir.path().join("app.yml"), "key: value").unwrap();

        let files = discover_source_files(dir.path());
        assert_eq!(files.len(), 4);
    }

    #[test]
    fn excludes_lockfiles() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(dir.path().join("package-lock.json"), "{}").unwrap();
        fs::write(dir.path().join("npm-shrinkwrap.json"), "{}").unwrap();
        fs::write(dir.path().join("pnpm-lock.yaml"), "lockfileVersion: 6").unwrap();

        let files = discover_source_files(dir.path());
        let names: Vec<_> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(names.contains(&"package.json".to_string()));
        assert!(!names.contains(&"package-lock.json".to_string()));
        assert!(!names.contains(&"npm-shrinkwrap.json".to_string()));
        assert!(!names.contains(&"pnpm-lock.yaml".to_string()));
    }

    #[test]
    fn lockfile_detection_no_false_positives() {
        // Ensure files with "lock" as a substring are NOT excluded
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("spinlock.h"), "typedef struct {} spinlock_t;").unwrap();
        fs::write(dir.path().join("block.json"), r#"{"height": 42}"#).unwrap();
        fs::write(dir.path().join("clock.yaml"), "timezone: UTC").unwrap();

        let files = discover_source_files(dir.path());
        let names: Vec<_> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(names.contains(&"spinlock.h".to_string()));
        assert!(names.contains(&"block.json".to_string()));
        assert!(names.contains(&"clock.yaml".to_string()));
    }

    #[test]
    fn filters_vendored_and_fixture_files() {
        let dir = tempfile::tempdir().unwrap();
        // Real source files
        fs::write(dir.path().join("index.ts"), "export const x = 1;").unwrap();
        // testdata/ directory (Go convention for fixtures — excluded)
        let testdata_dir = dir.path().join("testdata");
        fs::create_dir(&testdata_dir).unwrap();
        fs::write(testdata_dir.join("fixture.go"), "package testdata").unwrap();
        // vendor/ directory (vendored third-party dependencies — excluded)
        let vendor_dir = dir.path().join("vendor");
        fs::create_dir_all(vendor_dir.join("github.com/pkg/errors")).unwrap();
        fs::write(
            vendor_dir.join("github.com/pkg/errors/errors.go"),
            "package errors",
        )
        .unwrap();
        // deps/ directory (vendored C/C++ dependencies — excluded)
        let deps_dir = dir.path().join("deps");
        fs::create_dir(&deps_dir).unwrap();
        fs::write(deps_dir.join("sco.c"), "void sco_start() {}").unwrap();

        let files = discover_source_files(dir.path());
        let names: Vec<_> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert_eq!(files.len(), 1);
        assert!(names.contains(&"index.ts".to_string()));
        // vendor/, deps/, and testdata/ should be completely excluded
        assert!(!names.contains(&"fixture.go".to_string()));
        assert!(!names.contains(&"errors.go".to_string()));
        assert!(!names.contains(&"sco.c".to_string()));
    }

    #[test]
    fn includes_test_files_in_discovery() {
        let dir = tempfile::tempdir().unwrap();
        // Real source files
        fs::write(dir.path().join("index.ts"), "export const x = 1;").unwrap();
        // Test files — now included (deprioritized by scheduler)
        fs::write(dir.path().join("index.test.ts"), "describe('x', () => {});").unwrap();
        fs::write(dir.path().join("utils.spec.ts"), "it('works', () => {});").unwrap();
        let tests_dir = dir.path().join("__tests__");
        fs::create_dir(&tests_dir).unwrap();
        fs::write(tests_dir.join("helper.ts"), "export function h() {}").unwrap();
        let tests_dir2 = dir.path().join("tests");
        fs::create_dir(&tests_dir2).unwrap();
        fs::write(tests_dir2.join("integration.rs"), "fn test_it() {}").unwrap();
        let benches_dir = dir.path().join("benches");
        fs::create_dir(&benches_dir).unwrap();
        fs::write(benches_dir.join("bench.rs"), "fn bench_it() {}").unwrap();
        let examples_dir = dir.path().join("examples");
        fs::create_dir(&examples_dir).unwrap();
        fs::write(examples_dir.join("basic.rs"), "fn main() {}").unwrap();

        let files = discover_source_files(dir.path());
        let names: Vec<_> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        // All files should be discovered (test files are no longer excluded)
        assert!(names.contains(&"index.ts".to_string()));
        assert!(names.contains(&"index.test.ts".to_string()));
        assert!(names.contains(&"utils.spec.ts".to_string()));
        assert!(names.contains(&"helper.ts".to_string()));
        assert!(names.contains(&"integration.rs".to_string()));
        assert!(names.contains(&"bench.rs".to_string()));
        assert!(names.contains(&"basic.rs".to_string()));
    }

    #[test]
    fn classify_file_detection() {
        use crate::schedule::FileCategory::*;

        // Test directories
        assert_eq!(classify_file(Path::new("__tests__/helper.ts")), Test);
        assert_eq!(classify_file(Path::new("tests/integration.rs")), Test);
        assert_eq!(classify_file(Path::new("test/setup.ts")), Test);
        assert_eq!(classify_file(Path::new("testing/helpers.py")), Test);
        assert_eq!(classify_file(Path::new("benches/bench.rs")), Test);
        assert_eq!(classify_file(Path::new("benchmark/run.py")), Test);
        assert_eq!(classify_file(Path::new("benchmarks/perf.rs")), Test);
        assert_eq!(classify_file(Path::new("fixtures/setup.py")), Test);
        assert_eq!(classify_file(Path::new("fixture/helpers.ts")), Test);
        assert_eq!(classify_file(Path::new("mocks/mock_repo.go")), Test);
        assert_eq!(classify_file(Path::new("__mocks__/utils.ts")), Test);
        // Example directories
        assert_eq!(classify_file(Path::new("examples/basic.rs")), Example);
        assert_eq!(classify_file(Path::new("example/demo.ts")), Example);
        assert_eq!(classify_file(Path::new("experiments/train.py")), Example);
        assert_eq!(classify_file(Path::new("experiment/run.py")), Example);
        // Documentation site directories
        assert_eq!(classify_file(Path::new("website/src/App.tsx")), DocsSite);
        assert_eq!(classify_file(Path::new("site/pages/index.tsx")), DocsSite);
        // docs/ and doc/ are Source — they often contain valuable API reference
        assert_eq!(classify_file(Path::new("docs/conf.py")), Source);
        assert_eq!(classify_file(Path::new("doc/guide.md")), Source);
        // CI/CD directories
        assert_eq!(classify_file(Path::new(".github/workflows/ci.yml")), CiConfig);
        assert_eq!(classify_file(Path::new(".circleci/config.yml")), CiConfig);
        // Test file naming conventions
        assert_eq!(classify_file(Path::new("index.test.ts")), Test);
        assert_eq!(classify_file(Path::new("utils.spec.ts")), Test);
        assert_eq!(classify_file(Path::new("test_utils.py")), Test);
        assert_eq!(classify_file(Path::new("conftest.py")), Test);
        // Compound directory names with test-related segments
        assert_eq!(classify_file(Path::new("crates/foo-test-utils/src/lib.rs")), Test);
        assert_eq!(classify_file(Path::new("crates/my-integration-suite/src/setup.rs")), Test);
        assert_eq!(classify_file(Path::new("crates/my-mock-server/src/lib.rs")), Test);
        // Changelog directories
        assert_eq!(classify_file(Path::new("changelog/README.rst")), Test);
        assert_eq!(classify_file(Path::new("changelogs/1234.md")), Test);
        // Storybook directories
        assert_eq!(classify_file(Path::new("stories/Button.stories.tsx")), Test);
        assert_eq!(classify_file(Path::new(".storybook/config.js")), Test);
        // Contribute directories
        assert_eq!(classify_file(Path::new("contribute/demo.py")), Test);
        // RFC directories
        assert_eq!(classify_file(Path::new("rfcs/0001-design.md")), DocsSite);
        // GitLab CI
        assert_eq!(classify_file(Path::new(".gitlab/ci.yml")), CiConfig);
        // Normal source files
        assert_eq!(classify_file(Path::new("src/main.rs")), Source);
        assert_eq!(classify_file(Path::new("index.ts")), Source);
        assert_eq!(classify_file(Path::new("lib/utils.py")), Source);
        assert_eq!(classify_file(Path::new("crates/toasty-core/src/lib.rs")), Source);
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
}
