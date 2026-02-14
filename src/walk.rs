use ignore::Walk;
use std::path::{Path, PathBuf};

/// Known source file extensions — only languages with tree-sitter queries in queries/.
const SOURCE_EXTENSIONS: &[&str] = &[
    // Rust
    "rs",
    // TypeScript / JavaScript
    "ts", "tsx", "js", "jsx",
    // Python
    "py",
];

/// Discover source files under `root`, respecting .gitignore.
/// Returns paths sorted for deterministic output.
pub fn discover_source_files(root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Walk::new(root)
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .filter(|entry| is_source_file(entry.path()))
        .filter(|entry| !is_test_file(entry.path()))
        .map(|entry| entry.into_path())
        .collect();
    files.sort();
    files
}

fn is_source_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| SOURCE_EXTENSIONS.contains(&ext))
}

/// Check if a file is a test file that should be excluded from output.
/// Matches common JS/TS test patterns: *.test.ts, *.spec.tsx, __tests__/, etc.
fn is_test_file(path: &Path) -> bool {
    // Check for __tests__ directory anywhere in the path
    if path.components().any(|c| c.as_os_str() == "__tests__") {
        return true;
    }

    let stem = match path.file_stem().and_then(|s| s.to_str()) {
        Some(s) => s,
        None => return false,
    };

    // Match *.test.* and *.spec.* (e.g. foo.test.ts, bar.spec.tsx)
    // Match test_*.py and *_test.py (Python pytest conventions)
    stem.ends_with(".test") || stem.ends_with(".spec")
        || stem.starts_with("test_") || stem.ends_with("_test")
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
    }

    #[test]
    fn filters_non_source_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("lib.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("readme.md"), "# hi").unwrap();
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
        fs::write(dir.path().join("app.test.tsx"), "test('renders', () => {});").unwrap();
        // __tests__ directory
        let tests_dir = dir.path().join("__tests__");
        fs::create_dir(&tests_dir).unwrap();
        fs::write(tests_dir.join("helper.ts"), "export function h() {}").unwrap();

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
    }
}
