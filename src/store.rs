use std::cell::RefCell;
use std::path::{Path, PathBuf};

use elsa::FrozenMap;
use tree_sitter::{Parser, Tree};

use crate::Lang;

/// Pre-compiled tree-sitter configuration for a language.
///
/// Owns a reusable `Parser` so consecutive parses of the same extension
/// don't construct (and `set_language` on) a new parser each time.
pub struct LanguageConfig {
    pub lang: Lang,
    pub ts_language: tree_sitter::Language,
    pub query: tree_sitter::Query,
    pub symbol_idx: u32,
    pub name_idx: Option<u32>,
    parser: RefCell<Parser>,
}

/// Append-only storage for parsed sources and tree-sitter trees.
/// All `&str` slices and `Node` references borrow from here.
pub struct ParseStore {
    sources: FrozenMap<PathBuf, String>,
    trees: FrozenMap<PathBuf, Box<Tree>>,
    /// Lazily-built configs keyed by file extension (lowercase). Each config
    /// owns the parser for its extension, so `.c` and `.h` (both `Lang::C`
    /// but different tree-sitter languages) get distinct parsers.
    configs: FrozenMap<String, Box<LanguageConfig>>,
    /// Interned display paths, deduplicated. Returns `&Path` references
    /// that live as long as the store, eliminating per-item PathBuf clones.
    paths: FrozenMap<PathBuf, Box<PathBuf>>,
}

impl Default for ParseStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseStore {
    pub fn new() -> Self {
        Self {
            sources: FrozenMap::new(),
            trees: FrozenMap::new(),
            configs: FrozenMap::new(),
            paths: FrozenMap::new(),
        }
    }

    /// Get the language config for a path's extension.
    pub fn config_for(&self, path: &Path) -> Option<&LanguageConfig> {
        let ext_raw = path.extension().and_then(|e| e.to_str())?;
        // Fast path: try the raw extension before allocating a lowercased copy.
        if let Some(config) = self.configs.get(ext_raw) {
            return Some(config);
        }
        let ext = ext_raw.to_ascii_lowercase();
        if let Some(config) = self.configs.get(&ext) {
            return Some(config);
        }
        let lang = Lang::from_extension(&ext)?;
        let (ts_language, query_src) = language_for_lang(lang, &ext);
        let query = tree_sitter::Query::new(&ts_language, query_src).ok()?;
        let symbol_idx = query.capture_index_for_name("symbol")?;
        let name_idx = query.capture_index_for_name("name");
        let mut parser = Parser::new();
        parser.set_language(&ts_language).ok()?;
        let config = LanguageConfig {
            lang,
            ts_language,
            query,
            symbol_idx,
            name_idx,
            parser: RefCell::new(parser),
        };
        Some(self.configs.insert(ext, Box::new(config)))
    }

    fn parse_source(&self, path: &Path, source: &str) -> Option<Tree> {
        let config = self.config_for(path)?;
        config.parser.borrow_mut().parse(source, None)
    }

    /// Read, parse, and store a file. Only `FilesGroup::children()` should call this (A5).
    /// Returns `None` if the file fails to read or parse.
    pub fn parse(&self, path: &Path) -> Option<(&str, &Tree)> {
        // Already parsed?
        if let Some(src) = self.sources.get(path) {
            let tree = self.trees.get(path)?;
            return Some((src, tree));
        }

        let source = std::fs::read_to_string(path).ok()?;
        let tree = self.parse_source(path, &source);

        let src_ref = self.sources.insert(path.to_path_buf(), source);

        if let Some(tree) = tree {
            let tree_ref = self.trees.insert(path.to_path_buf(), Box::new(tree));
            Some((src_ref, tree_ref))
        } else {
            // Source stored but no tree (unsupported language or parse failure)
            None
        }
    }

    /// Store a source string directly (for single-file input where we already have it).
    /// Returns the stored reference.
    pub fn store_source(&self, path: &Path, source: String) -> &str {
        if let Some(existing) = self.sources.get(path) {
            return existing;
        }
        self.sources.insert(path.to_path_buf(), source)
    }

    /// Intern a display path, returning a stable `&Path` reference that lives
    /// as long as the store. Repeated calls with equal paths return the same
    /// reference without allocating.
    pub fn intern_path(&self, path: PathBuf) -> &Path {
        if let Some(p) = self.paths.get(&path) {
            return p.as_path();
        }
        self.paths.insert(path.clone(), Box::new(path)).as_path()
    }
}

/// Returns the tree-sitter language and query for a language.
fn language_for_lang(lang: Lang, ext: &str) -> (tree_sitter::Language, &'static str) {
    match (lang, ext) {
        (Lang::Rust, _) => (
            tree_sitter_rust::LANGUAGE.into(),
            include_str!("../queries/rust.scm"),
        ),
        (Lang::JsTs, "tsx" | "jsx") => (
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            include_str!("../queries/typescript.scm"),
        ),
        (Lang::JsTs, _) => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            include_str!("../queries/typescript.scm"),
        ),
        (Lang::Go, _) => (
            tree_sitter_go::LANGUAGE.into(),
            include_str!("../queries/go.scm"),
        ),
        (Lang::C, "c") => (
            tree_sitter_c::LANGUAGE.into(),
            include_str!("../queries/c.scm"),
        ),
        (Lang::C, _) => (
            tree_sitter_cpp::LANGUAGE.into(),
            include_str!("../queries/cpp.scm"),
        ),
        (Lang::Java, _) => (
            tree_sitter_java::LANGUAGE.into(),
            include_str!("../queries/java.scm"),
        ),
        (Lang::Lua, _) => (
            tree_sitter_lua::LANGUAGE.into(),
            include_str!("../queries/lua.scm"),
        ),
        (Lang::Python, _) => (
            tree_sitter_python::LANGUAGE.into(),
            include_str!("../queries/python.scm"),
        ),
        (Lang::Markdown, _) => (
            tree_sitter_md::LANGUAGE.into(),
            include_str!("../queries/markdown.scm"),
        ),
        (Lang::Json, _) => (
            tree_sitter_json::LANGUAGE.into(),
            include_str!("../queries/json.scm"),
        ),
        (Lang::Toml, _) => (
            tree_sitter_toml_ng::LANGUAGE.into(),
            include_str!("../queries/toml.scm"),
        ),
        (Lang::Yaml, _) => (
            tree_sitter_yaml::LANGUAGE.into(),
            include_str!("../queries/yaml.scm"),
        ),
    }
}
