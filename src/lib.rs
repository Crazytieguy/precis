pub mod fixtures;
pub mod format;
pub mod parse;
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
    Markdown,
    /// TypeScript, JavaScript, TSX, JSX
    JsTs,
}

impl Lang {
    /// Map a file extension to its language family.
    pub fn from_extension(ext: &str) -> Option<Lang> {
        match ext {
            "rs" => Some(Lang::Rust),
            "py" => Some(Lang::Python),
            "go" => Some(Lang::Go),
            "md" => Some(Lang::Markdown),
            "ts" | "tsx" | "js" | "jsx" | "mts" | "cts" | "mjs" | "cjs" => Some(Lang::JsTs),
            _ => None,
        }
    }

    pub fn from_path(path: &std::path::Path) -> Option<Lang> {
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(Lang::from_extension)
    }
}
