use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use elsa::FrozenMap;
use tree_sitter::{Parser, Query, Tree};

use crate::Lang;

/// Combined per-kind query for a language. Built by concatenating every
/// kind's query string for the language and tracking which `pattern_index`
/// belongs to which kind.
pub struct CombinedKindQuery {
    pub query: Query,
    /// `pattern_to_kind[i]` is the 0-based declaration position of the kind
    /// that owns the `i`th pattern in `query`. Both `build_combined_query`
    /// and the dispatcher iterate `query_kinds!` in declaration order, so
    /// this indexes into a fixed-size bucket vector sized to the number of
    /// query kinds.
    pub pattern_to_kind: Vec<u8>,
}

/// Pre-compiled tree-sitter configuration for a language.
///
/// Owns a reusable `Parser` so consecutive parses of the same extension
/// don't construct (and `set_language` on) a new parser each time.
pub struct LanguageConfig {
    pub lang: Lang,
    pub ts_language: tree_sitter::Language,
    parser: RefCell<Parser>,
    /// Cached combined per-kind query, lazily built on first use.
    combined_kind_query: OnceLock<Option<CombinedKindQuery>>,
}

impl LanguageConfig {
    /// Get (or build) the combined per-kind query for this language. The
    /// builder closure runs at most once.
    pub fn combined_kind_query(
        &self,
        build: impl FnOnce(&tree_sitter::Language) -> Option<CombinedKindQuery>,
    ) -> Option<&CombinedKindQuery> {
        self.combined_kind_query
            .get_or_init(|| build(&self.ts_language))
            .as_ref()
    }
}

/// Append-only storage for parsed sources and tree-sitter trees.
/// All `&str` slices and `Node` references borrow from here.
pub struct ParseStore {
    sources: FrozenMap<PathBuf, String>,
    trees: FrozenMap<PathBuf, Box<Tree>>,
    /// Lazily-built configs keyed by file extension (lowercase). Each config
    /// owns the parser for its extension.
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
        let ts_language = ts_language_for(lang);
        let mut parser = Parser::new();
        parser.set_language(&ts_language).ok()?;
        let config = LanguageConfig {
            lang,
            ts_language,
            parser: RefCell::new(parser),
            combined_kind_query: OnceLock::new(),
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

/// Returns the tree-sitter language for a `Lang`.
fn ts_language_for(lang: Lang) -> tree_sitter::Language {
    match lang {
        Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
        Lang::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        Lang::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
        Lang::Go => tree_sitter_go::LANGUAGE.into(),
        Lang::C => tree_sitter_c::LANGUAGE.into(),
        Lang::Cpp => tree_sitter_cpp::LANGUAGE.into(),
        Lang::Java => tree_sitter_java::LANGUAGE.into(),
        Lang::Lua => tree_sitter_lua::LANGUAGE.into(),
        Lang::Python => tree_sitter_python::LANGUAGE.into(),
        Lang::Markdown => tree_sitter_md::LANGUAGE.into(),
        Lang::Json => tree_sitter_json::LANGUAGE.into(),
        Lang::Toml => tree_sitter_toml_ng::LANGUAGE.into(),
        Lang::Yaml => tree_sitter_yaml::LANGUAGE.into(),
    }
}

