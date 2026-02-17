use crate::parse;
use ignore::Walk;
use std::path::{Path, PathBuf};

/// Discover source files under `root`, respecting .gitignore.
/// Returns paths sorted for deterministic output.
pub fn discover_source_files(root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Walk::new(root)
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .filter(|entry| is_source_file(entry.path()))
        .filter(|entry| {
            let relative = entry.path().strip_prefix(root).unwrap_or(entry.path());
            !is_test_file(relative)
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
}

/// Check if a file should be excluded from output (test/benchmark/example infrastructure).
/// Matches test directories, benchmark directories, example directories,
/// and test file naming conventions.
fn is_test_file(path: &Path) -> bool {
    // Check for non-source directories anywhere in the path:
    // __tests__/ (Jest), tests/ (Rust/Python/JS), test/ (JS/TS), testing/ (Python),
    // benches/ (Rust), benchmark/ and benchmarks/ (cross-language),
    // testdata/ (Go convention, excluded by go build),
    // vendor/ (vendored third-party dependencies: Go, PHP, Ruby),
    // examples/ and example/ (usage demonstrations, not core API: Rust, Go, JS)
    if path.components().any(|c| {
        let s = c.as_os_str();
        s == "__tests__"
            || s == "tests"
            || s == "test"
            || s == "testing"
            || s == "benches"
            || s == "benchmark"
            || s == "benchmarks"
            || s == "testdata"
            || s == "vendor"
            || s == "examples"
            || s == "example"
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
        // tests/ should be excluded as a test directory
        assert!(!relative.iter().any(|p| p.starts_with("tests/")));
    }

    #[test]
    fn filters_non_source_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("lib.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("readme.txt"), "hi").unwrap();
        fs::write(dir.path().join("data.json"), "{}").unwrap();

        let files = discover_source_files(dir.path());
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("lib.rs"));
    }

    #[test]
    fn filters_test_files() {
        let dir = tempfile::tempdir().unwrap();
        // Real source files
        fs::write(dir.path().join("index.ts"), "export const x = 1;").unwrap();
        fs::write(dir.path().join("utils.ts"), "export function f() {}").unwrap();
        // Test files that should be excluded
        fs::write(dir.path().join("index.test.ts"), "describe('x', () => {});").unwrap();
        fs::write(dir.path().join("utils.spec.ts"), "it('works', () => {});").unwrap();
        fs::write(
            dir.path().join("app.test.tsx"),
            "test('renders', () => {});",
        )
        .unwrap();
        // __tests__ directory
        let tests_dir = dir.path().join("__tests__");
        fs::create_dir(&tests_dir).unwrap();
        fs::write(tests_dir.join("helper.ts"), "export function h() {}").unwrap();
        // tests/ directory (Rust integration tests, Python tests)
        let tests_dir2 = dir.path().join("tests");
        fs::create_dir(&tests_dir2).unwrap();
        fs::write(tests_dir2.join("integration.rs"), "fn test_it() {}").unwrap();
        // test/ directory (common JS/TS convention)
        let test_dir = dir.path().join("test");
        fs::create_dir(&test_dir).unwrap();
        fs::write(test_dir.join("setup.ts"), "export const setup = {};").unwrap();
        // benches/ directory (Rust benchmarks)
        let benches_dir = dir.path().join("benches");
        fs::create_dir(&benches_dir).unwrap();
        fs::write(benches_dir.join("bench.rs"), "fn bench_it() {}").unwrap();
        // testing/ directory (Python convention)
        let testing_dir = dir.path().join("testing");
        fs::create_dir(&testing_dir).unwrap();
        fs::write(testing_dir.join("helpers.py"), "def h(): pass").unwrap();
        // benchmark/ directory
        let benchmark_dir = dir.path().join("benchmark");
        fs::create_dir(&benchmark_dir).unwrap();
        fs::write(benchmark_dir.join("run.py"), "def bench(): pass").unwrap();
        // conftest.py (pytest infrastructure)
        fs::write(dir.path().join("conftest.py"), "import pytest").unwrap();
        // TypeScript type test definitions (tsd)
        fs::write(
            dir.path().join("index.test-d.ts"),
            "expectType<string>(fn());",
        )
        .unwrap();
        // testdata/ directory (Go convention)
        let testdata_dir = dir.path().join("testdata");
        fs::create_dir(&testdata_dir).unwrap();
        fs::write(testdata_dir.join("fixture.go"), "package testdata").unwrap();
        // vendor/ directory (vendored third-party dependencies)
        let vendor_dir = dir.path().join("vendor");
        fs::create_dir_all(vendor_dir.join("github.com/pkg/errors")).unwrap();
        fs::write(
            vendor_dir.join("github.com/pkg/errors/errors.go"),
            "package errors",
        )
        .unwrap();
        // examples/ directory (usage demonstrations)
        let examples_dir = dir.path().join("examples");
        fs::create_dir(&examples_dir).unwrap();
        fs::write(examples_dir.join("basic.rs"), "fn main() {}").unwrap();
        // example/ directory (singular variant)
        let example_dir = dir.path().join("example");
        fs::create_dir(&example_dir).unwrap();
        fs::write(example_dir.join("demo.ts"), "export const x = 1;").unwrap();

        let files = discover_source_files(dir.path());
        let names: Vec<_> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert_eq!(files.len(), 2);
        assert!(names.contains(&"index.ts".to_string()));
        assert!(names.contains(&"utils.ts".to_string()));
        // Test files should be excluded
        assert!(!names.contains(&"index.test.ts".to_string()));
        assert!(!names.contains(&"utils.spec.ts".to_string()));
        assert!(!names.contains(&"app.test.tsx".to_string()));
        assert!(!names.contains(&"helper.ts".to_string()));
        assert!(!names.contains(&"integration.rs".to_string()));
        assert!(!names.contains(&"setup.ts".to_string()));
        assert!(!names.contains(&"bench.rs".to_string()));
        assert!(!names.contains(&"helpers.py".to_string()));
        assert!(!names.contains(&"run.py".to_string()));
        assert!(!names.contains(&"conftest.py".to_string()));
        // testdata/, vendor/, examples/, example/ should be excluded
        assert!(!names.contains(&"fixture.go".to_string()));
        assert!(!names.contains(&"errors.go".to_string()));
        assert!(!names.contains(&"basic.rs".to_string()));
        assert!(!names.contains(&"demo.ts".to_string()));
        assert!(!names.contains(&"index.test-d.ts".to_string()));
    }
}
