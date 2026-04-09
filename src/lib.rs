pub mod layout;
pub mod parse;
pub mod render;
pub mod schedule;
pub mod walk;

use std::path::{Path, PathBuf};

use rayon::prelude::*;

// ---------------------------------------------------------------------------
// FileData / Corpus — per-file pipeline data
// ---------------------------------------------------------------------------

/// All per-file data bundled together: source text, extracted symbols (with
/// layouts), and file-level metadata.  Constructed by [`build_file_data`] which
/// fuses symbol extraction, layout computation, and file classification into a
/// single parallel pass per file.
pub struct FileData {
    pub source: Option<String>,
    pub symbols: Vec<parse::Symbol>,
    pub info: schedule::FileInfo,
}

/// Borrowed view of a project's file data, passed through the schedule/render
/// pipeline.  Constructed cheaply from `&[FileData]`.
pub struct Corpus<'a> {
    pub files: &'a [FileData],
}

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
    /// C and C++ source and header files
    C,
    Java,
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
            "c" | "h" | "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "hh" => Some(Lang::C),
            "java" => Some(Lang::Java),
            "lua" => Some(Lang::Lua),
            "md" | "mdx" => Some(Lang::Markdown),
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

/// Check if a file extension indicates a C/C++ header file.
pub fn is_header_extension(ext: &str) -> bool {
    matches!(ext, "h" | "hpp" | "hxx" | "hh")
}

// ---------------------------------------------------------------------------
// Corpus methods
// ---------------------------------------------------------------------------

impl<'a> Corpus<'a> {
    /// Render the corpus within token and optional character budgets.
    pub fn render(&self, budget: usize, char_budget: Option<usize>) -> String {
        let (output, _) = self.render_stats(budget, char_budget);
        output
    }

    /// Render the corpus, returning output and actual token count.
    pub fn render_stats(&self, budget: usize, char_budget: Option<usize>) -> (String, usize) {
        let built = schedule::build_groups(self, budget);
        let sched = schedule::schedule(&built, self, char_budget);
        let output = render::render_scheduled(self, &sched);
        let actual = render::count_tokens(&output);
        (output, actual)
    }
}

// ---------------------------------------------------------------------------
// Pipeline: build FileData from files + sources
// ---------------------------------------------------------------------------

/// Fused pipeline: extract symbols, compute layouts, and classify each file in
/// parallel.  Consumes owned sources (moved into [`FileData`], no cloning).
pub fn build_file_data(
    root: &Path,
    files: &[PathBuf],
    sources: Vec<Option<String>>,
) -> Vec<FileData> {
    let configs = parse::build_language_configs(files);
    files
        .par_iter()
        .zip(sources.into_par_iter())
        .map(|(path, source)| {
            let symbols = match &source {
                Some(s) => parse::extract_file_symbols(path, s, &configs),
                None => vec![],
            };
            let info =
                schedule::compute_single_file_info(root, path, source.as_deref());
            FileData { source, symbols, info }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

/// Render files within token and optional character budgets.
pub fn render_with_budget(
    budget: usize,
    char_budget: Option<usize>,
    root: &Path,
    files: &[PathBuf],
    sources: Vec<Option<String>>,
) -> String {
    let (output, _) = render_with_budget_stats(budget, char_budget, root, files, sources);
    output
}

/// Render files within a token budget, returning output and actual token count.
pub fn render_with_budget_stats(
    budget: usize,
    char_budget: Option<usize>,
    root: &Path,
    files: &[PathBuf],
    sources: Vec<Option<String>>,
) -> (String, usize) {
    let file_data = build_file_data(root, files, sources);
    let corpus = Corpus { files: &file_data };
    corpus.render_stats(budget, char_budget)
}

/// Render a single file within token and optional character budgets.
pub fn render_file_with_budget(
    budget: usize,
    char_budget: Option<usize>,
    path: &Path,
    root: &Path,
    source: &str,
) -> String {
    let files = vec![path.to_path_buf()];
    let sources = vec![Some(source.to_string())];
    render_with_budget(budget, char_budget, root, &files, sources)
}

/// Pre-read source files to avoid repeated disk I/O.
pub fn read_sources(files: &[PathBuf]) -> Vec<Option<String>> {
    files
        .par_iter()
        .map(|f| std::fs::read_to_string(f).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_monotonicity() {
        // More budget should never produce fewer tokens.
        let source = "/// Doc comment\npub fn hello() {}\npub struct Foo { x: i32 }\nfn private() {}\n";
        let root = Path::new("");
        let path = Path::new("test.rs");
        let files = vec![path.to_path_buf()];
        let sources = vec![Some(source.to_string())];
        let file_data = build_file_data(root, &files, sources);
        let corpus = Corpus { files: &file_data };

        let mut prev_tokens = 0;
        for budget in [10, 50, 100, 200, 500, 1000, 5000] {
            let output = corpus.render(budget, None);
            let tokens = render::count_tokens(&output);
            assert!(
                tokens >= prev_tokens,
                "Budget monotonicity violated at budget {}: {} < {}",
                budget,
                tokens,
                prev_tokens,
            );
            prev_tokens = tokens;
        }
    }
}
