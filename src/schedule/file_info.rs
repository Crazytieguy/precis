use std::path::{Path, PathBuf};

use crate::Lang;

use super::classify::{
    self, is_autogen_api_doc, is_config_file, is_generated_file, is_generated_filename,
    is_type_declaration_file, FileCategory, FileRole,
};

/// Pre-computed per-file metadata derived from the file path and source content.
///
/// Computed once per file by [`compute_single_file_info`] and stored in [`crate::FileData`].
/// Downstream consumers (groups, solver, plan, render) read from this instead of
/// recomputing path-derived properties independently.
pub struct FileInfo {
    /// Path relative to the project root.
    pub relative_path: PathBuf,
    /// Detected language family from the file extension.
    pub lang: Option<Lang>,
    /// Structural role: README, Changelog, Architecture, etc.
    pub file_role: FileRole,
    /// Category relative to core source: Source, Test, Example, etc.
    pub file_category: FileCategory,
    /// Whether the file is build/tool configuration (file-level component).
    /// Per-symbol overrides (e.g. TOML `[tool.*]` sections) are applied in groups.
    pub is_config: bool,
    /// Whether the file is a TypeScript declaration file (.d.ts, .d.mts, .d.cts).
    pub is_type_declaration: bool,
    /// Whether the file is a C/C++ header (.h, .hpp, etc.).
    pub is_header: bool,
    /// Whether the file is auto-generated (codegen markers, protobuf output, etc.).
    pub is_generated: bool,
}

/// Compute metadata for a single file.  Used by [`crate::build_file_data`] in
/// its per-file parallel loop.
pub fn compute_single_file_info(
    root: &Path,
    file: &Path,
    source: Option<&str>,
) -> FileInfo {
    let relative = file.strip_prefix(root).unwrap_or(file).to_path_buf();
    let lang = Lang::from_path(&relative);
    let file_role = FileRole::from_path(&relative);
    let file_category = classify::classify_file(&relative);
    let is_config = relative
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| is_config_file(&relative, name));
    let is_type_declaration = is_type_declaration_file(&relative);
    let is_header = relative
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| crate::is_header_extension(&ext.to_ascii_lowercase()));
    let is_generated = source.is_some_and(|src| {
        is_autogen_api_doc(src, file_role) || is_generated_file(src)
    }) || is_generated_filename(&relative);

    FileInfo {
        relative_path: relative,
        lang,
        file_role,
        file_category,
        is_config,
        is_type_declaration,
        is_header,
        is_generated,
    }
}
