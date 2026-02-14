use ignore::Walk;
use std::path::{Path, PathBuf};

/// Known source file extensions, grouped by language.
const SOURCE_EXTENSIONS: &[&str] = &[
    // Rust
    "rs",
    // TypeScript / JavaScript
    "ts", "tsx", "js", "jsx",
    // Python
    "py",
    // Go
    "go",
];

/// Discover source files under `root`, respecting .gitignore.
/// Returns paths sorted for deterministic output.
pub fn discover_source_files(root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Walk::new(root)
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .filter(|entry| is_source_file(entry.path()))
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
}
