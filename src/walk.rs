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
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(parse::is_supported_extension)
        && !is_lockfile(path)
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
        "package-lock.json" | "npm-shrinkwrap.json" | "pnpm-lock.yaml"
    )
}

/// Check if a file is vendored third-party code or test fixture data that should
/// be completely excluded from output. These are never authored by the project.
fn is_vendored_or_fixture(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str();
        // vendor/ (vendored third-party dependencies: Go, PHP, Ruby)
        // testdata/ (Go convention for test fixture data, excluded by go build)
        s == "vendor" || s == "testdata"
    })
}

/// Check if a file is test/benchmark/example/docs-site infrastructure.
/// These files are included in output but deprioritized by the scheduler.
/// Matches test directories, benchmark directories, example directories,
/// documentation site directories, and test file naming conventions.
pub fn is_test_file(path: &Path) -> bool {
    // Check for deprioritized directories anywhere in the path:
    // __tests__/ (Jest), tests/ (Rust/Python/JS), test/ (JS/TS), testing/ (Python),
    // benches/ (Rust), benchmark/ and benchmarks/ (cross-language),
    // examples/ and example/ (usage demonstrations, not core API: Rust, Go, JS),
    // website/ and site/ (documentation sites: Docusaurus, Next.js, etc.),
    // docs/ and doc/ (supplementary documentation, Sphinx configs, GitHub Pages),
    // mocks/ and __mocks__/ (auto-generated test mocks: Go mockery, Jest, etc.)
    if path.components().any(|c| {
        let s = c.as_os_str();
        s == "__tests__"
            || s == "tests"
            || s == "test"
            || s == "testing"
            || s == "benches"
            || s == "benchmark"
            || s == "benchmarks"
            || s == "examples"
            || s == "example"
            || s == "website"
            || s == "site"
            || s == "docs"
            || s == "doc"
            || s == "mocks"
            || s == "__mocks__"
    }) {
        return true;
    }

    let stem = match path.file_stem().and_then(|s| s.to_str()) {
        Some(s) => s,
        None => return false,
    };

    // Match *.test.* and *.spec.* (e.g. foo.test.ts, bar.spec.tsx)
    // Match *.test-d.ts (TypeScript type test definitions, used by tsd)
    // Match test_*.py and *_test.py (Python pytest conventions)
    // Match conftest.py (pytest configuration/fixtures)
    stem.ends_with(".test")
        || stem.ends_with(".test-d")
        || stem.ends_with(".spec")
        || stem.starts_with("test_")
        || stem.ends_with("_test")
        || stem == "conftest"
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

        let files = discover_source_files(dir.path());
        let names: Vec<_> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert_eq!(files.len(), 1);
        assert!(names.contains(&"index.ts".to_string()));
        // vendor/ and testdata/ should be completely excluded
        assert!(!names.contains(&"fixture.go".to_string()));
        assert!(!names.contains(&"errors.go".to_string()));
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
    fn is_test_file_detection() {
        // Test directories
        assert!(is_test_file(Path::new("__tests__/helper.ts")));
        assert!(is_test_file(Path::new("tests/integration.rs")));
        assert!(is_test_file(Path::new("test/setup.ts")));
        assert!(is_test_file(Path::new("testing/helpers.py")));
        assert!(is_test_file(Path::new("benches/bench.rs")));
        assert!(is_test_file(Path::new("benchmark/run.py")));
        assert!(is_test_file(Path::new("benchmarks/perf.rs")));
        assert!(is_test_file(Path::new("examples/basic.rs")));
        assert!(is_test_file(Path::new("example/demo.ts")));
        // Documentation site directories
        assert!(is_test_file(Path::new("website/src/App.tsx")));
        assert!(is_test_file(Path::new("site/pages/index.tsx")));
        assert!(is_test_file(Path::new("docs/conf.py")));
        assert!(is_test_file(Path::new("doc/guide.md")));
        // Mock directories
        assert!(is_test_file(Path::new("mocks/mock_repo.go")));
        assert!(is_test_file(Path::new("internal/ports/mocks/mock_service.go")));
        assert!(is_test_file(Path::new("__mocks__/utils.ts")));
        // Test file naming conventions
        assert!(is_test_file(Path::new("index.test.ts")));
        assert!(is_test_file(Path::new("utils.spec.ts")));
        assert!(is_test_file(Path::new("index.test-d.ts")));
        assert!(is_test_file(Path::new("test_utils.py")));
        assert!(is_test_file(Path::new("utils_test.py")));
        assert!(is_test_file(Path::new("conftest.py")));
        // Normal source files
        assert!(!is_test_file(Path::new("src/main.rs")));
        assert!(!is_test_file(Path::new("index.ts")));
        assert!(!is_test_file(Path::new("lib/utils.py")));
    }
}
