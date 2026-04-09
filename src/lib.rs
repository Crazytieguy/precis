pub mod layout;
pub mod parse;
pub mod render;
pub mod schedule;
pub mod walk;

use std::path::{Path, PathBuf};

use rayon::prelude::*;

// ---------------------------------------------------------------------------
// Corpus — bundled per-file data for the schedule/render pipeline
// ---------------------------------------------------------------------------

/// Borrowed view of all per-file data, passed through the schedule/render pipeline.
///
/// Bundles the parallel arrays (`files`, `sources`, `symbols`, `layouts`) that
/// every pipeline stage needs, eliminating 5-7 parameter function signatures.
/// Constructed cheaply (zero-copy) from the owning data in `render_with_budget_stats`
/// or directly by benchmarks/profiling tools.
pub struct Corpus<'a> {
    pub root: &'a Path,
    pub files: &'a [PathBuf],
    pub sources: &'a [Option<String>],
    pub all_symbols: &'a [Vec<parse::Symbol>],
    pub layouts: &'a [Vec<layout::SymbolLayout>],
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
// Public entry points
// ---------------------------------------------------------------------------

/// Render files within token and optional character budgets.
pub fn render_with_budget(
    budget: usize,
    char_budget: Option<usize>,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
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
    sources: &[Option<String>],
) -> (String, usize) {
    let all_symbols = extract_all_symbols(files, sources);
    let layouts = layout::compute_all_layouts(files, sources, &all_symbols);
    let corpus = Corpus { root, files, sources, all_symbols: &all_symbols, layouts: &layouts };
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
    render_with_budget(budget, char_budget, root, &files, &sources)
}

/// Pre-read source files to avoid repeated disk I/O.
pub fn read_sources(files: &[PathBuf]) -> Vec<Option<String>> {
    files
        .par_iter()
        .map(|f| std::fs::read_to_string(f).ok())
        .collect()
}

/// Pre-extract symbols from all source files.
pub fn extract_all_symbols(
    files: &[PathBuf],
    sources: &[Option<String>],
) -> Vec<Vec<parse::Symbol>> {
    let configs = parse::build_language_configs(files);
    parse::extract_all_symbols_cached(files, sources, &configs)
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

        let mut prev_tokens = 0;
        for budget in [10, 50, 100, 200, 500, 1000, 5000] {
            let output = render_with_budget(budget, None, root, &files, &sources);
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
