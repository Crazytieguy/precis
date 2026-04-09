mod ast;
mod classify;
mod name;
mod postprocess;
mod visibility;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::Lang;

/// Pre-compiled tree-sitter configuration for a language.
pub struct LanguageConfig {
    lang: Lang,
    ts_language: Language,
    query: Query,
    symbol_idx: u32,
    name_idx: Option<u32>,
}

fn normalized_ext(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
}

/// A symbol extracted from a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: String,
    pub is_public: bool,
    /// For imports: whether the import refers to a 1st-party (local) module.
    /// 1st-party imports are higher signal for understanding a file's role.
    pub is_first_party: bool,
    pub line: usize,
    pub end_line: usize,
    /// End line of the symbol's signature (1-indexed), computed from tree-sitter
    /// AST by locating the body/block child. For C-like languages this is the
    /// line containing `{`; for Python it's the line containing `:`.
    /// `None` when tree-sitter couldn't determine the boundary (fallback to
    /// text heuristics in `layout::signature_end_line`).
    pub sig_end_line: Option<usize>,
    /// First line of the doc comment block preceding the symbol (1-indexed),
    /// computed from tree-sitter AST by walking previous sibling comment nodes.
    /// `None` when tree-sitter couldn't find a doc comment (fallback to
    /// text heuristics in `layout::doc_comment_start`).
    pub doc_start_line: Option<usize>,
    /// Whether this symbol is a method inside a trait implementation block
    /// (Rust `impl Trait for Type { ... }`). Trait impl methods implement
    /// an interface defined elsewhere and are typically boilerplate (fmt,
    /// from, clone, deref, etc.). Deprioritized relative to inherent methods
    /// and standalone functions.
    pub is_trait_impl: bool,
    /// Whether this is a `pub use` re-export from a child module (Rust only).
    /// Detected when a public import references `self::` or a bare path whose
    /// first segment matches a `mod` declaration in the same file. These are
    /// redundant when the submodule's own symbols are already in the output.
    pub is_reexport: bool,
    /// Byte offset of the symbol's start in the source string.
    pub start_byte: usize,
    /// Byte offset of the symbol's end (exclusive) in the source string.
    pub end_byte: usize,
    /// For composite symbols (multiple symbols sharing a source line): the
    /// prefix length from the line start to each original symbol's end_byte,
    /// in source order. Slicing `lines[sym_line_0][..prefix_lens[n]]` gives a
    /// valid prefix of the source line including the first n+1 original symbols.
    /// Empty for non-composite symbols.
    pub composed_prefix_lens: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    TypeAlias,
    Const,
    Static,
    Macro,
    Module,
    Class,
    Interface,
    Section,
    Import,
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Function => write!(f, "fn"),
            SymbolKind::Struct => write!(f, "struct"),
            SymbolKind::Enum => write!(f, "enum"),
            SymbolKind::Trait => write!(f, "trait"),
            SymbolKind::Impl => write!(f, "impl"),
            SymbolKind::TypeAlias => write!(f, "type"),
            SymbolKind::Const => write!(f, "const"),
            SymbolKind::Static => write!(f, "static"),
            SymbolKind::Macro => write!(f, "macro"),
            SymbolKind::Module => write!(f, "mod"),
            SymbolKind::Class => write!(f, "class"),
            SymbolKind::Interface => write!(f, "interface"),
            SymbolKind::Section => write!(f, "section"),
            SymbolKind::Import => write!(f, "import"),
        }
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Check if a file extension is supported for symbol extraction.
pub fn is_supported_extension(ext: &str) -> bool {
    Lang::from_extension(ext).is_some()
}

/// Create a single Section symbol for files in unsupported languages.
/// The section spans just the first meaningful line (the "heading"), with body
/// content extending to EOF via the layout system's next-heading detection.
/// Skips shebang lines (`#!`) since they convey no structural information.
pub(crate) fn plain_text_symbol(source: &str) -> Vec<Symbol> {
    let mut lines = source.lines().enumerate();
    let first = loop {
        match lines.next() {
            Some((_, line)) if line.starts_with("#!") => continue,
            Some((i, _)) => break i + 1,
            None => return vec![],
        }
    };
    vec![Symbol {
        kind: SymbolKind::Section,
        name: String::new(),
        is_public: true,
        is_first_party: false,
        line: first,
        end_line: first,
        sig_end_line: None,
        doc_start_line: None,
        is_trait_impl: false,
        is_reexport: false,
        start_byte: 0,
        end_byte: 0,
        composed_prefix_lens: Vec::new(),
    }]
}

/// Returns the tree-sitter language and query for a file extension, if supported.
fn language_for_extension(ext: &str) -> Option<(Language, &'static str)> {
    let lang = Lang::from_extension(ext)?;
    Some(match (lang, ext) {
        (Lang::Rust, _) => (
            tree_sitter_rust::LANGUAGE.into(),
            include_str!("../../queries/rust.scm"),
        ),
        (Lang::JsTs, "tsx" | "jsx") => (
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            include_str!("../../queries/typescript.scm"),
        ),
        (Lang::JsTs, _) => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            include_str!("../../queries/typescript.scm"),
        ),
        (Lang::Go, _) => (
            tree_sitter_go::LANGUAGE.into(),
            include_str!("../../queries/go.scm"),
        ),
        // Pure .c files use the C grammar; everything else (including .h)
        // uses the C++ grammar which is a superset and handles both C and C++.
        (Lang::C, "c") => (
            tree_sitter_c::LANGUAGE.into(),
            include_str!("../../queries/c.scm"),
        ),
        (Lang::C, _) => (
            tree_sitter_cpp::LANGUAGE.into(),
            include_str!("../../queries/cpp.scm"),
        ),
        (Lang::Lua, _) => (
            tree_sitter_lua::LANGUAGE.into(),
            include_str!("../../queries/lua.scm"),
        ),
        (Lang::Python, _) => (
            tree_sitter_python::LANGUAGE.into(),
            include_str!("../../queries/python.scm"),
        ),
        (Lang::Markdown, _) => (
            tree_sitter_md::LANGUAGE.into(),
            include_str!("../../queries/markdown.scm"),
        ),
        (Lang::Json, _) => (
            tree_sitter_json::LANGUAGE.into(),
            include_str!("../../queries/json.scm"),
        ),
        (Lang::Toml, _) => (
            tree_sitter_toml_ng::LANGUAGE.into(),
            include_str!("../../queries/toml.scm"),
        ),
        (Lang::Yaml, _) => (
            tree_sitter_yaml::LANGUAGE.into(),
            include_str!("../../queries/yaml.scm"),
        ),
    })
}

/// Build pre-compiled language configs from the extensions present in `files`.
/// One config per unique extension (~20 total), each with a pre-compiled `Query`.
pub fn build_language_configs(files: &[PathBuf]) -> HashMap<String, LanguageConfig> {
    let mut configs: HashMap<String, LanguageConfig> = HashMap::new();

    for file in files {
        let ext_owned = match normalized_ext(file) {
            Some(ext) => ext,
            None => continue,
        };
        if configs.contains_key(&ext_owned) {
            continue;
        }
        let lang = match Lang::from_extension(&ext_owned) {
            Some(l) => l,
            None => continue,
        };
        let (ts_language, query_src) = match language_for_extension(&ext_owned) {
            Some(pair) => pair,
            None => continue,
        };

        let query = Query::new(&ts_language, query_src).expect("invalid query");
        let symbol_idx = query
            .capture_index_for_name("symbol")
            .expect("missing @symbol capture");
        let name_idx = query.capture_index_for_name("name");
        configs.insert(ext_owned, LanguageConfig {
            lang,
            ts_language,
            query,
            symbol_idx,
            name_idx,
        });
    }
    configs
}

/// Extract symbols from all files using pre-compiled language configs.
pub fn extract_all_symbols_cached(
    files: &[PathBuf],
    sources: &[Option<String>],
    configs: &HashMap<String, LanguageConfig>,
) -> Vec<Vec<Symbol>> {
    files
        .par_iter()
        .zip(sources.par_iter())
        .map(|(f, s)| {
            let source = match s.as_ref() {
                Some(s) => s,
                None => return vec![],
            };
            match normalized_ext(f).as_deref().and_then(|e| configs.get(e)) {
                Some(config) => extract_symbols_with_config(f, source, config),
                None => plain_text_symbol(source),
            }
        })
        .collect()
}

/// Extract symbols from a source file.
pub fn extract_symbols(path: &Path, source: &str) -> Vec<Symbol> {
    let files = [path.to_path_buf()];
    let configs = build_language_configs(&files);
    match normalized_ext(path).as_deref().and_then(|e| configs.get(e)) {
        Some(config) => extract_symbols_with_config(path, source, config),
        None => plain_text_symbol(source),
    }
}

// ---------------------------------------------------------------------------
// Core extraction loop
// ---------------------------------------------------------------------------

/// Extract symbols from a source file using a pre-compiled language config.
///
/// The loop delegates to focused sub-modules for each concern:
/// - [`classify`]: node kind → SymbolKind (with inline filtering)
/// - [`name`]: symbol name extraction
/// - [`visibility`]: public/private determination
/// - [`ast`]: signature end line, doc start line, trait impl detection, etc.
/// - [`postprocess`]: mark_reexports, dedup_overloads, merge_shared_line_symbols
pub fn extract_symbols_with_config(
    path: &Path,
    source: &str,
    config: &LanguageConfig,
) -> Vec<Symbol> {
    let lang = config.lang;

    let mut parser = Parser::new();
    parser
        .set_language(&config.ts_language)
        .expect("language version mismatch");

    let tree = match parser.parse(source, None) {
        Some(tree) => tree,
        None => return vec![],
    };

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&config.query, tree.root_node(), source.as_bytes());

    let symbol_idx = config.symbol_idx;
    let name_idx = config.name_idx;

    let mut symbols = Vec::new();

    while let Some(m) = matches.next() {
        let symbol_node = match m.captures.iter().find(|c| c.index == symbol_idx) {
            Some(c) => c.node,
            None => continue,
        };

        // 1. Classify: what kind of symbol is this node?
        let kind = match classify::classify_node(symbol_node, m.captures, name_idx, source, lang, path) {
            Some(k) => k,
            None => continue,
        };

        // 2. Filter: skip nodes nested inside function bodies, test code, anon const blocks
        if ast::is_inside_function(symbol_node) {
            continue;
        }
        if lang == Lang::Rust && ast::is_rust_test_code(symbol_node, source) {
            continue;
        }
        if lang == Lang::Rust && ast::is_inside_rust_anon_const(symbol_node, source) {
            continue;
        }

        // 3. Name: extract the display name
        let name_node = name_idx
            .and_then(|idx| m.captures.iter().find(|c| c.index == idx))
            .map(|c| c.node);
        let name = match name::extract_name(symbol_node, kind, name_node, source, lang) {
            Some(n) => n,
            None => continue,
        };

        // 4. Visibility
        let is_public = visibility::determine_visibility(symbol_node, kind, &name, source, lang);

        // 5. Import metadata
        let is_first_party = if kind == SymbolKind::Import {
            name::is_first_party_import(&name, lang)
        } else {
            false
        };

        // 6. Rust trait impl detection
        let is_trait_impl = lang == Lang::Rust
            && matches!(kind, SymbolKind::Function | SymbolKind::Const | SymbolKind::TypeAlias)
            && ast::is_in_trait_impl(symbol_node);

        // 7. AST metadata
        // C++ template handling: when a symbol is a direct child of
        // template_declaration, use the template node for line range,
        // signature, and doc detection so `template<...>` is included.
        let effective_node = if lang == Lang::C
            && symbol_node.parent().is_some_and(|p| p.kind() == "template_declaration")
        {
            symbol_node.parent().unwrap()
        } else {
            symbol_node
        };

        let line = effective_node.start_position().row + 1;
        let end_line = if lang == Lang::C
            && matches!(effective_node.kind(), "preproc_include" | "preproc_def" | "preproc_function_def")
            && effective_node.end_position().column == 0
            && effective_node.end_position().row > effective_node.start_position().row
        {
            effective_node.end_position().row
        } else {
            effective_node.end_position().row + 1
        };

        symbols.push(Symbol {
            kind,
            name,
            is_public,
            is_first_party,
            line,
            end_line,
            sig_end_line: ast::compute_sig_end_line(effective_node, lang),
            doc_start_line: ast::compute_doc_start_line(effective_node, source, lang),
            is_trait_impl,
            is_reexport: false,
            start_byte: effective_node.start_byte(),
            end_byte: effective_node.end_byte(),
            composed_prefix_lens: Vec::new(),
        });
    }

    postprocess::finalize(symbols, lang, source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn extracts_rust_symbols() {
        let source = r#"
pub fn hello(name: &str) -> String {
    format!("Hello, {name}")
}

struct Point {
    x: f64,
    y: f64,
}

pub enum Color {
    Red,
    Green,
    Blue,
}

pub trait Greet {
    fn greet(&self) -> String;
}

impl Greet for Point {
    fn greet(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

pub type Name = String;

const MAX: usize = 100;

pub static GLOBAL: &str = "hi";

macro_rules! say {
    ($e:expr) => { println!("{}", $e) };
}

pub mod utils;
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source);
        let names: Vec<_> = symbols
            .iter()
            .map(|s| (s.kind, s.name.as_str(), s.is_public))
            .collect();

        assert!(names.contains(&(SymbolKind::Function, "hello", true)));
        assert!(names.contains(&(SymbolKind::Struct, "Point", false)));
        assert!(names.contains(&(SymbolKind::Enum, "Color", true)));
        assert!(names.contains(&(SymbolKind::Trait, "Greet", true)));
        assert!(names.contains(&(SymbolKind::Impl, "Greet for Point", false)));
        assert!(names.contains(&(SymbolKind::Impl, "Point", false)));
        assert!(names.contains(&(SymbolKind::TypeAlias, "Name", true)));
        assert!(names.contains(&(SymbolKind::Const, "MAX", false)));
        assert!(names.contains(&(SymbolKind::Static, "GLOBAL", true)));
        assert!(names.contains(&(SymbolKind::Macro, "say", false)));
        assert!(names.contains(&(SymbolKind::Module, "utils", true)));

        // Functions inside impl blocks should also be found
        assert_eq!(
            names
                .iter()
                .filter(|&&(k, n, p)| k == SymbolKind::Function && n == "greet" && p)
                .count(),
            2
        );
        assert!(!names.contains(&(SymbolKind::Function, "greet", false)));
        assert!(names.contains(&(SymbolKind::Function, "new", true)));
    }

    #[test]
    fn rust_restricted_visibility_is_not_public() {
        let source = r#"
pub fn fully_public() {}
pub(crate) fn crate_visible() {}
pub(super) fn super_visible() {}
fn private() {}
pub struct PubStruct {}
pub(crate) struct CrateStruct {}
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source);
        let names: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.is_public))
            .collect();

        assert!(names.contains(&("fully_public", true)));
        assert!(names.contains(&("crate_visible", false)));
        assert!(names.contains(&("super_visible", false)));
        assert!(names.contains(&("private", false)));
        assert!(names.contains(&("PubStruct", true)));
        assert!(names.contains(&("CrateStruct", false)));
    }

    #[test]
    fn rust_doc_hidden_is_not_public() {
        let source = r#"
pub fn visible() {}

#[doc(hidden)]
pub fn hidden_fn() {}

#[doc(hidden)]
pub struct HiddenStruct {}

#[doc(hidden)]
pub mod __private {}

pub struct NormalStruct {}
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source);
        let names: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.is_public))
            .collect();

        assert!(names.contains(&("visible", true)));
        assert!(names.contains(&("hidden_fn", false)));
        assert!(names.contains(&("HiddenStruct", false)));
        assert!(names.contains(&("__private", false)));
        assert!(names.contains(&("NormalStruct", true)));
    }

    #[test]
    fn rust_macro_export_is_public() {
        let source = r#"
macro_rules! private_macro {
    () => {};
}

#[macro_export]
macro_rules! public_macro {
    () => {};
}

/// Documented macro
#[macro_export]
macro_rules! documented_public_macro {
    () => {};
}
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source);
        let names: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.is_public))
            .collect();

        assert!(names.contains(&("private_macro", false)));
        assert!(names.contains(&("public_macro", true)));
        assert!(names.contains(&("documented_public_macro", true)));

        let source_hidden = r#"
#[doc(hidden)]
#[macro_export]
macro_rules! __internal_helper {
    () => {};
}
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source_hidden);
        let names: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.is_public))
            .collect();
        assert!(names.contains(&("__internal_helper", false)));
    }

    #[test]
    fn unsupported_extension_returns_plain_text_section() {
        let symbols = extract_symbols(Path::new("test.rb"), "def foo(): pass");
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].kind, SymbolKind::Section);
        assert_eq!(symbols[0].line, 1);
    }

    #[test]
    fn empty_file_returns_no_symbols() {
        let symbols = extract_symbols(Path::new("test.rb"), "");
        assert!(symbols.is_empty());
    }

    #[test]
    fn filters_rust_test_code() {
        let source = r#"
pub fn real_function() {}

#[test]
fn test_something() {}

#[cfg(test)]
mod tests {
    fn helper() {}

    #[test]
    fn another_test() {}
}
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"real_function"));
        assert!(!names.contains(&"test_something"));
        assert!(!names.contains(&"tests"));
        assert!(!names.contains(&"helper"));
        assert!(!names.contains(&"another_test"));
    }

    #[test]
    fn filters_rust_anon_const_blocks() {
        let source = r#"
pub fn real_function() {}

const _: () = {
    use core::fmt::Debug;

    struct ProbeType;

    impl Debug for ProbeType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            Ok(())
        }
    }
};

const _: Option<&str> = option_env!("RUSTC_BOOTSTRAP");

pub const REAL_CONST: usize = 42;
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"real_function"));
        assert!(names.contains(&"REAL_CONST"));
        assert!(!names.contains(&"_"));
        assert!(!names.contains(&"ProbeType"));
        assert!(!names.contains(&"fmt"));
    }

    #[test]
    fn deduplicates_typescript_overloads() {
        let source = r#"
export class Example {
    andThen(f: (val: number) => string): string;
    andThen(f: (val: number) => number): number;
    andThen(f: (val: number) => any): any {
        return f(42);
    }

    simple(): void {
        // no overloads
    }

    combine(a: number): number;
    combine(a: string): string;
    combine(a: any): any {
        return a;
    }
}
"#;
        let symbols = extract_symbols(Path::new("test.ts"), source);
        let names: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.is_public))
            .collect();

        assert_eq!(names.iter().filter(|(n, _)| *n == "andThen").count(), 1);
        assert_eq!(names.iter().filter(|(n, _)| *n == "combine").count(), 1);
        assert!(names.contains(&("andThen", true)));
        assert!(names.contains(&("combine", true)));
        assert!(names.contains(&("simple", true)));
    }

    #[test]
    fn extracts_lua_symbols() {
        let source = r#"
local M = {}

M.CONSTANT = 42

function M.init()
    -- Initialize
end

function M:method(arg)
    return self.value + arg
end

local function helper()
    -- Private
end
"#;
        let symbols = extract_symbols(Path::new("test.lua"), source);
        let names: Vec<_> = symbols.iter().map(|s| (s.name.as_str(), s.kind)).collect();
        assert!(names.iter().any(|(n, k)| *n == "M.init" && *k == SymbolKind::Function));
        assert!(names.iter().any(|(n, k)| *n == "M:method" && *k == SymbolKind::Function));
        assert!(names.iter().any(|(n, _)| *n == "M.CONSTANT"));
        assert!(names.iter().any(|(n, k)| *n == "helper" && *k == SymbolKind::Function));
        assert!(names.iter().any(|(n, _)| *n == "M"));
    }

    #[test]
    fn deduplicates_non_consecutive_c_structs() {
        let source = r#"
struct asmctx {
    int x;
};

struct other {
    int y;
};

struct asmctx {
    long x;
};
"#;
        let symbols = extract_symbols(Path::new("test.c"), source);
        let struct_names: Vec<_> = symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Struct)
            .map(|s| s.name.as_str())
            .collect();

        assert_eq!(
            struct_names.iter().filter(|&&n| n == "asmctx").count(),
            1,
            "non-consecutive struct duplicates should be deduped"
        );
        assert!(struct_names.contains(&"other"));
    }

    #[test]
    fn filters_nested_functions() {
        let source = r#"
pub fn outer() {
    fn nested_helper() {}
    const LOCAL: usize = 42;
}

impl Foo {
    pub fn method() {
        fn local_fn() {}
    }
}

fn top_level() {}
"#;
        let symbols = extract_symbols(Path::new("test.rs"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"outer"));
        assert!(names.contains(&"top_level"));
        assert!(names.contains(&"method"));
        assert!(!names.contains(&"nested_helper"));
        assert!(!names.contains(&"local_fn"));
        assert!(!names.contains(&"LOCAL"));
    }

    #[test]
    fn filters_nested_in_function_expressions() {
        let source = r#"
export const Outer = forwardRef(function Outer(props, ref) {
    const localVar = useRef(null);
    const anotherLocal = useMemo(() => 42);
    function localHelper() {}
});

export const TopLevel = 42;
"#;
        let symbols = extract_symbols(Path::new("test.tsx"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"Outer"));
        assert!(names.contains(&"TopLevel"));
        assert!(!names.contains(&"localVar"));
        assert!(!names.contains(&"anotherLocal"));
        assert!(!names.contains(&"localHelper"));
    }

    #[test]
    fn reclassifies_arrow_functions_as_fn() {
        let source = r#"
export const greet = (name: string): string => {
    return `Hello, ${name}`;
};

export const add = (a: number, b: number) => a + b;

const helper = function(x: number) { return x * 2; };

export const API_URL = "https://example.com";

export const MAX_RETRIES = 3;
"#;
        let symbols = extract_symbols(Path::new("test.ts"), source);
        let kinds: Vec<_> = symbols.iter().map(|s| (s.name.as_str(), s.kind)).collect();

        assert!(kinds.contains(&("greet", SymbolKind::Function)));
        assert!(kinds.contains(&("add", SymbolKind::Function)));
        assert!(kinds.contains(&("helper", SymbolKind::Function)));
        assert!(kinds.contains(&("API_URL", SymbolKind::Const)));
        assert!(kinds.contains(&("MAX_RETRIES", SymbolKind::Const)));
    }

    #[test]
    fn filters_let_and_var_declarations() {
        let source = r#"
const API_URL = "https://example.com";
let counter = 0;
var legacy = "old";
const greet = (name) => `Hello, ${name}`;
let mutableFn = () => {};
"#;
        let symbols = extract_symbols(Path::new("test.js"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"API_URL"));
        assert!(names.contains(&"greet"));
        assert!(!names.contains(&"counter"));
        assert!(!names.contains(&"legacy"));
        assert!(!names.contains(&"mutableFn"));
    }

    #[test]
    fn filters_require_calls() {
        let source = r#"
const debug = require('../internal/debug')
const SemVer = require('./semver')
const parseOptions = require('../internal/parse-options')
const EventEmitter = require('node:events').EventEmitter;

const ANY = Symbol('SemVer ANY')
const MAX_RETRIES = 3;

export const helper = (x) => x * 2;
"#;
        let symbols = extract_symbols(Path::new("test.js"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(!names.contains(&"debug"));
        assert!(!names.contains(&"SemVer"));
        assert!(!names.contains(&"parseOptions"));
        assert!(!names.contains(&"EventEmitter"));
        assert!(names.contains(&"ANY"));
        assert!(names.contains(&"MAX_RETRIES"));
        assert!(names.contains(&"helper"));
    }

    #[test]
    fn captures_js_private_methods() {
        let source = r#"
export class Parser {
    parse() { return []; }
    #advance() {}
    #reset() {}
    _internal() {}
    _prepareForParse() {}
}
"#;
        let symbols = extract_symbols(Path::new("test.js"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public))
            .collect();

        assert!(info.contains(&("parse", SymbolKind::Function, true)));
        assert!(info.contains(&("#advance", SymbolKind::Function, false)));
        assert!(info.contains(&("#reset", SymbolKind::Function, false)));
        assert!(info.contains(&("_internal", SymbolKind::Function, false)));
        assert!(info.contains(&("_prepareForParse", SymbolKind::Function, false)));
    }

    #[test]
    fn interface_underscore_methods_stay_public() {
        let source = r#"
interface IResult<T, E> {
    isOk(): boolean;
    _unsafeUnwrap(): T;
    _unsafeUnwrapErr(): E;
}
"#;
        let symbols = extract_symbols(Path::new("test.ts"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public))
            .collect();

        assert!(info.contains(&("isOk", SymbolKind::Function, true)));
        assert!(info.contains(&("_unsafeUnwrap", SymbolKind::Function, true)));
        assert!(info.contains(&("_unsafeUnwrapErr", SymbolKind::Function, true)));
    }

    #[test]
    fn extracts_python_symbols() {
        let source = r#"
import os
from typing import Optional

def greet(name: str) -> str:
    """Say hello."""
    return f"Hello, {name}"

def _private_helper(x):
    return x * 2

class Animal:
    """An animal."""

    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return "..."

    def _internal(self):
        pass

class _PrivateClass:
    pass

MAX_SIZE: int = 100
"#;
        let symbols = extract_symbols(Path::new("test.py"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public))
            .collect();

        assert!(info.contains(&("greet", SymbolKind::Function, true)));
        assert!(info.contains(&("_private_helper", SymbolKind::Function, false)));
        assert!(info.contains(&("Animal", SymbolKind::Class, true)));
        assert!(info.contains(&("_PrivateClass", SymbolKind::Class, false)));
        assert!(info.contains(&("__init__", SymbolKind::Function, true)));
        assert!(info.contains(&("speak", SymbolKind::Function, true)));
        assert!(info.contains(&("_internal", SymbolKind::Function, false)));
    }

    #[test]
    fn filters_python_nested_functions() {
        let source = r#"
def outer():
    def inner_helper():
        pass
    return inner_helper()

class Foo:
    def method(self):
        def local():
            pass
        return local()

def top_level():
    pass
"#;
        let symbols = extract_symbols(Path::new("test.py"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"outer"));
        assert!(names.contains(&"Foo"));
        assert!(names.contains(&"method"));
        assert!(names.contains(&"top_level"));
        assert!(!names.contains(&"inner_helper"));
        assert!(!names.contains(&"local"));
    }

    #[test]
    fn python_decorated_functions() {
        let source = r#"
from functools import lru_cache

class MyClass:
    @property
    def name(self) -> str:
        return self._name

    @staticmethod
    def create() -> "MyClass":
        return MyClass()

    @classmethod
    def from_dict(cls, data: dict) -> "MyClass":
        return cls()

@lru_cache(maxsize=128)
def cached_compute(x: int) -> int:
    return x * x
"#;
        let symbols = extract_symbols(Path::new("test.py"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"MyClass"));
        assert!(names.contains(&"name"));
        assert!(names.contains(&"create"));
        assert!(names.contains(&"from_dict"));
        assert!(names.contains(&"cached_compute"));
    }

    #[test]
    fn extracts_python_module_constants() {
        let source = r#"
from typing import TypeVar

T = TypeVar("T")

VERSION: str = "0.1.0"

MAX_SIZE: int = 100

__all__ = ["foo", "bar"]

__version__ = "1.0.0"

UPPER_CASE = 42

lower_case = "not a constant"

logger = get_logger(__name__)

_PRIVATE_CONST = 99

def some_function():
    LOCAL_CONST = 1
"#;
        let symbols = extract_symbols(Path::new("test.py"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public))
            .collect();

        assert!(info.contains(&("VERSION", SymbolKind::Const, true)));
        assert!(info.contains(&("MAX_SIZE", SymbolKind::Const, true)));
        assert!(info.contains(&("T", SymbolKind::Const, true)));
        assert!(info.contains(&("UPPER_CASE", SymbolKind::Const, true)));
        assert!(info.contains(&("__all__", SymbolKind::Const, true)));
        assert!(info.contains(&("__version__", SymbolKind::Const, true)));
        assert!(info.contains(&("_PRIVATE_CONST", SymbolKind::Const, false)));
        assert!(!info.iter().any(|(name, _, _)| *name == "lower_case"));
        assert!(!info.iter().any(|(name, _, _)| *name == "logger"));
        assert!(!info.iter().any(|(name, _, _)| *name == "LOCAL_CONST"));
        assert!(info.contains(&("some_function", SymbolKind::Function, true)));
    }

    #[test]
    fn captures_arrow_function_class_fields() {
        let source = r#"
class Observer {
    subscribers: Array<string>;

    constructor() {
        this.subscribers = [];
    }

    subscribe = (subscriber: string) => {
        this.subscribers.push(subscriber);
    };

    publish = (data: string) => {
        console.log(data);
    };

    normalMethod() {
        return true;
    }
}
"#;
        let symbols = extract_symbols(Path::new("test.ts"), source);
        let info: Vec<_> = symbols.iter().map(|s| (s.name.as_str(), s.kind)).collect();

        assert!(info.contains(&("subscribe", SymbolKind::Function)));
        assert!(info.contains(&("publish", SymbolKind::Function)));
        assert!(info.contains(&("normalMethod", SymbolKind::Function)));
        assert!(info.contains(&("constructor", SymbolKind::Function)));
        assert!(!info.iter().any(|(name, _)| *name == "subscribers"));
    }

    #[test]
    fn extracts_abstract_classes() {
        let source = r#"
export abstract class Base {
    abstract method(): void;
    concrete(): string { return ""; }
}

export class Derived extends Base {
    method(): void {}
}
"#;
        let symbols = extract_symbols(Path::new("test.ts"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public))
            .collect();

        assert!(info.contains(&("Base", SymbolKind::Class, true)));
        assert!(info.contains(&("method", SymbolKind::Function, true)));
        assert!(info.contains(&("concrete", SymbolKind::Function, true)));
        assert!(info.contains(&("Derived", SymbolKind::Class, true)));
    }

    #[test]
    fn extracts_markdown_headings() {
        let source = r#"# Introduction

Some introductory text.

## Getting Started

Instructions here.

### Installation

Steps to install.

## API Reference

### `parse(source)`

Parse the source code.

#### Parameters

| Name | Type |
|------|------|
| source | string |

## Contributing
"#;
        let symbols = extract_symbols(Path::new("test.md"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public, s.line))
            .collect();

        assert_eq!(symbols.len(), 7);
        assert!(info.contains(&("Introduction", SymbolKind::Section, true, 1)));
        assert!(info.contains(&("Getting Started", SymbolKind::Section, true, 5)));
        assert!(info.contains(&("Installation", SymbolKind::Section, true, 9)));
        assert!(info.contains(&("API Reference", SymbolKind::Section, true, 13)));
        assert!(info.contains(&("`parse(source)`", SymbolKind::Section, true, 15)));
        assert!(info.contains(&("Parameters", SymbolKind::Section, true, 19)));
        assert!(info.contains(&("Contributing", SymbolKind::Section, true, 25)));
        assert!(symbols.iter().all(|s| s.is_public));
        assert!(symbols.iter().all(|s| s.end_line == s.line + 1));
    }

    #[test]
    fn extracts_setext_headings() {
        let source = "Introduction\n============\n\nSome text.\n\nGetting Started\n---------------\n\nMore text.\n";
        let symbols = extract_symbols(Path::new("test.md"), source);
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "Introduction");
        assert_eq!(symbols[0].line, 1);
        assert_eq!(symbols[0].end_line, 3);
        assert_eq!(symbols[1].name, "Getting Started");
        assert_eq!(symbols[1].line, 6);
        assert_eq!(symbols[1].end_line, 8);
    }

    #[test]
    fn extracts_go_symbols() {
        let source = r#"
package token

import "fmt"

// Token represents a lexical token.
type Token struct {
	Kind TokenKind
	Span Span
}

// Stringer interface for custom formatting.
type Stringer interface {
	String() string
}

type Span = [2]int

const MaxTokens = 1024

var Version = "0.1.0"

var internal = "hidden"

// Process processes input and returns tokens.
func Process(input string) ([]Token, error) {
	return nil, nil
}

func helper() {}

// String implements Stringer for Token.
func (t Token) String() string {
	return fmt.Sprintf("%v", t.Kind)
}

func (t *Token) reset() {
	t.Kind = 0
}
"#;
        let symbols = extract_symbols(Path::new("test.go"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public))
            .collect();

        assert!(info.contains(&("Token", SymbolKind::Struct, true)));
        assert!(info.contains(&("Stringer", SymbolKind::Interface, true)));
        assert!(info.contains(&("Span", SymbolKind::TypeAlias, true)));
        assert!(info.contains(&("MaxTokens", SymbolKind::Const, true)));
        assert!(info.contains(&("Version", SymbolKind::Static, true)));
        assert!(info.contains(&("internal", SymbolKind::Static, false)));
        assert!(info.contains(&("Process", SymbolKind::Function, true)));
        assert!(info.contains(&("helper", SymbolKind::Function, false)));
        assert!(info.contains(&("String", SymbolKind::Function, true)));
        assert!(info.contains(&("reset", SymbolKind::Function, false)));
    }

    #[test]
    fn filters_go_blank_identifier() {
        let source = r#"
package color

type Attribute int

const (
	Reset Attribute = iota
	Bold
	Faint
	_
	Underline
)

var _ error = (*MyError)(nil)
"#;
        let symbols = extract_symbols(Path::new("test.go"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"const"));
        assert!(names.contains(&"Reset"));
        assert!(names.contains(&"Bold"));
        assert!(names.contains(&"Faint"));
        assert!(names.contains(&"Underline"));
        assert!(!names.contains(&"_"));
    }

    #[test]
    fn captures_go_grouped_const_var_blocks() {
        let source = r#"
package example

const MaxItems = 100

const (
	Red   = iota
	Green
	Blue
)

var Version = "1.0"

var (
	Debug   bool
	Verbose bool
)
"#;
        let symbols = extract_symbols(Path::new("test.go"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind))
            .collect();

        assert_eq!(
            info.iter().filter(|(n, _)| *n == "MaxItems").count(),
            1,
            "standalone const should appear once"
        );
        assert!(info.contains(&("const", SymbolKind::Const)));
        assert!(info.contains(&("Red", SymbolKind::Const)));
        assert!(info.contains(&("Green", SymbolKind::Const)));
        assert!(info.contains(&("Blue", SymbolKind::Const)));
        assert_eq!(
            info.iter().filter(|(n, _)| *n == "Version").count(),
            1,
            "standalone var should appear once"
        );
        assert!(info.contains(&("var", SymbolKind::Static)));
        assert!(info.contains(&("Debug", SymbolKind::Static)));
        assert!(info.contains(&("Verbose", SymbolKind::Static)));
    }

    #[test]
    fn filters_go_func_literal_nested_symbols() {
        let source = r#"
package example

var Handler = func() {
	const bufSize = 4096
	var temp = "x"
}

func Process() {}

const MaxItems = 100
"#;
        let symbols = extract_symbols(Path::new("test.go"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert!(names.contains(&"Handler"));
        assert!(names.contains(&"Process"));
        assert!(names.contains(&"MaxItems"));
        assert!(!names.contains(&"bufSize"));
        assert!(!names.contains(&"temp"));
    }

    #[test]
    fn preserves_go_multiple_init_functions() {
        let source = r#"
package example

func init() {
	registerA()
}

func init() {
	registerB()
}

func Process() {}
"#;
        let symbols = extract_symbols(Path::new("test.go"), source);
        let init_count = symbols.iter().filter(|s| s.name == "init").count();
        assert_eq!(
            init_count, 2,
            "Go allows multiple init() functions; both must be preserved"
        );
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"Process"));
    }

    #[test]
    fn extracts_json_top_level_keys() {
        let source = r#"{
  "name": "my-project",
  "version": "1.0.0",
  "scripts": {
    "build": "tsc",
    "test": "jest"
  },
  "dependencies": {
    "react": "^18.0.0"
  }
}"#;
        let symbols = extract_symbols(Path::new("package.json"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert_eq!(names, &["name", "version", "scripts", "dependencies"]);
        assert!(symbols.iter().all(|s| s.kind == SymbolKind::Section));
        assert!(symbols.iter().all(|s| s.is_public));

        let scripts = &symbols[2];
        assert_eq!(scripts.line, 4);
        assert_eq!(scripts.name, "scripts");
        let deps = &symbols[3];
        assert_eq!(deps.line, 8);
    }

    #[test]
    fn json_empty_and_array_roots() {
        assert!(extract_symbols(Path::new("empty.json"), "").is_empty());
        let syms = extract_symbols(Path::new("empty.json"), "{}");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].kind, SymbolKind::Section);
        let syms = extract_symbols(Path::new("arr.json"), "[1, 2, 3]");
        assert_eq!(syms.len(), 1);
    }

    #[test]
    fn extracts_toml_sections() {
        let source = r#"[package]
name = "precis"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4" }

[[bin]]
name = "precis"
path = "src/main.rs"
"#;
        let symbols = extract_symbols(Path::new("Cargo.toml"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert_eq!(names, &["package", "dependencies", "bin"]);
        assert!(symbols.iter().all(|s| s.kind == SymbolKind::Section));
        assert!(symbols.iter().all(|s| s.is_public));
        assert_eq!(symbols[0].line, 1);
        assert_eq!(symbols[0].end_line, 6);
    }

    #[test]
    fn extracts_yaml_top_level_keys() {
        let source = r#"name: my-project
version: 1.0.0
scripts:
  build: tsc
  test: jest
dependencies:
  react: "^18.0.0"
"#;
        let symbols = extract_symbols(Path::new("config.yml"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert_eq!(names, &["name", "version", "scripts", "dependencies"]);
        assert!(symbols.iter().all(|s| s.kind == SymbolKind::Section));
        assert!(symbols.iter().all(|s| s.is_public));
    }

    #[test]
    fn yaml_skips_comments_and_markers() {
        let source = r#"---
# This is a comment
name: project
version: 1.0
..."#;
        let symbols = extract_symbols(Path::new("config.yaml"), source);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();

        assert_eq!(names, &["name", "version"]);
    }

    #[test]
    fn toml_empty_file() {
        assert!(extract_symbols(Path::new("empty.toml"), "").is_empty());
        let syms = extract_symbols(Path::new("comments.toml"), "# just a comment\n");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].kind, SymbolKind::Section);
    }
}
