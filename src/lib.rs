pub mod format;
pub mod layout;
pub mod parse;
pub mod schedule;
pub mod walk;

/// Language family for rendering and parsing heuristics (comment styles, delimiters).
///
/// This is the single source of truth for which file extensions map to which
/// language family. Both the parser and renderer derive their extension handling
/// from [`Lang::from_extension`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Rust,
    Python,
    Go,
    C,
    Lua,
    Markdown,
    /// TypeScript, JavaScript, TSX, JSX
    JsTs,
    /// JSON config files (package.json, tsconfig.json, etc.)
    Json,
    /// TOML config files (Cargo.toml, pyproject.toml, etc.)
    Toml,
    /// YAML config files (docker-compose.yml, CI configs, etc.)
    Yaml,
}

impl Lang {
    /// Map a file extension to its language family.
    pub fn from_extension(ext: &str) -> Option<Lang> {
        match ext {
            "rs" => Some(Lang::Rust),
            "py" => Some(Lang::Python),
            "go" => Some(Lang::Go),
            "c" | "h" => Some(Lang::C),
            "lua" => Some(Lang::Lua),
            "md" => Some(Lang::Markdown),
            "ts" | "tsx" | "js" | "jsx" | "mts" | "cts" | "mjs" | "cjs" => Some(Lang::JsTs),
            "json" => Some(Lang::Json),
            "toml" => Some(Lang::Toml),
            "yaml" | "yml" => Some(Lang::Yaml),
            _ => None,
        }
    }

    pub fn from_path(path: &std::path::Path) -> Option<Lang> {
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(|ext| Lang::from_extension(&ext.to_ascii_lowercase()))
    }
}
