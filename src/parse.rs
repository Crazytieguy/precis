use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

/// A symbol extracted from a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: String,
    /// Full text of the symbol node (for signature extraction later).
    pub text: String,
    pub is_public: bool,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        }
    }
}

/// Returns the tree-sitter language and query for a file extension, if supported.
fn language_for_extension(ext: &str) -> Option<(Language, &'static str)> {
    match ext {
        "rs" => Some((
            tree_sitter_rust::LANGUAGE.into(),
            include_str!("../queries/rust.scm"),
        )),
        "ts" => Some((
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            include_str!("../queries/typescript.scm"),
        )),
        "tsx" => Some((
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            include_str!("../queries/typescript.scm"),
        )),
        _ => None,
    }
}

/// Extract symbols from a source file.
pub fn extract_symbols(path: &Path, source: &str) -> Vec<Symbol> {
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext,
        None => return vec![],
    };

    let (language, query_src) = match language_for_extension(ext) {
        Some(pair) => pair,
        None => return vec![],
    };

    let mut parser = Parser::new();
    parser.set_language(&language).expect("language version mismatch");

    let tree = match parser.parse(source, None) {
        Some(tree) => tree,
        None => return vec![],
    };

    let query = Query::new(&language, query_src).expect("invalid query");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    let symbol_idx = query.capture_index_for_name("symbol").expect("missing @symbol capture");
    let name_idx = query.capture_index_for_name("name");

    let mut symbols = Vec::new();

    while let Some(m) = matches.next() {
        let symbol_node = match m.captures.iter().find(|c| c.index == symbol_idx) {
            Some(c) => c.node,
            None => continue,
        };

        let kind = match symbol_node.kind() {
            // Rust
            "function_item" | "function_signature_item" => SymbolKind::Function,
            "struct_item" => SymbolKind::Struct,
            "enum_item" => SymbolKind::Enum,
            "trait_item" => SymbolKind::Trait,
            "impl_item" => SymbolKind::Impl,
            "type_item" => SymbolKind::TypeAlias,
            "const_item" => SymbolKind::Const,
            "static_item" => SymbolKind::Static,
            "macro_definition" => SymbolKind::Macro,
            "mod_item" => SymbolKind::Module,
            // TypeScript
            "function_declaration" | "method_definition" | "method_signature" => {
                SymbolKind::Function
            }
            "class_declaration" => SymbolKind::Class,
            "interface_declaration" => SymbolKind::Interface,
            "enum_declaration" => SymbolKind::Enum,
            "type_alias_declaration" => SymbolKind::TypeAlias,
            "lexical_declaration" => SymbolKind::Const,
            "internal_module" => SymbolKind::Module,
            _ => continue,
        };

        let name = if kind == SymbolKind::Impl {
            impl_name(symbol_node, source)
        } else {
            match name_idx.and_then(|idx| m.captures.iter().find(|c| c.index == idx)) {
                Some(c) => c.node.utf8_text(source.as_bytes()).unwrap_or("?").to_string(),
                None => continue,
            }
        };

        let text = symbol_node
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .to_string();

        let is_public = is_public_symbol(symbol_node, source);

        symbols.push(Symbol {
            kind,
            name,
            text,
            is_public,
            line: symbol_node.start_position().row + 1,
        });
    }

    symbols
}

fn is_public_symbol(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    // Rust: `pub` keyword appears as a visibility_modifier child
    if node
        .children(&mut cursor)
        .any(|child| child.kind() == "visibility_modifier")
    {
        return true;
    }
    // TypeScript: exported symbols are children of export_statement
    if let Some(parent) = node.parent()
        && parent.kind() == "export_statement"
    {
        return true;
    }
    // TypeScript class methods: public if accessibility_modifier is "public",
    // or if no accessibility_modifier (public by default in TS)
    if node.kind() == "method_definition" {
        let mut cursor = node.walk();
        let has_accessor = node
            .children(&mut cursor)
            .any(|child| child.kind() == "accessibility_modifier");
        if !has_accessor {
            return true; // no modifier = public by default
        }
        let mut cursor = node.walk();
        return node.children(&mut cursor).any(|child| {
            child.kind() == "accessibility_modifier"
                && child.utf8_text(source.as_bytes()).unwrap_or("") == "public"
        });
    }
    false
}

/// Build a display name for an impl block, e.g. "Display for Foo" or "Foo".
fn impl_name(node: tree_sitter::Node, source: &str) -> String {
    let type_node = node.child_by_field_name("type");
    let trait_node = node.child_by_field_name("trait");

    let type_name = type_node
        .and_then(|n| n.utf8_text(source.as_bytes()).ok())
        .unwrap_or("?");

    match trait_node.and_then(|n| n.utf8_text(source.as_bytes()).ok()) {
        Some(trait_name) => format!("{trait_name} for {type_name}"),
        None => type_name.to_string(),
    }
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
        let names: Vec<_> = symbols.iter().map(|s| (s.kind, s.name.as_str(), s.is_public)).collect();

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
        assert!(names.contains(&(SymbolKind::Function, "greet", false)));
        assert!(names.contains(&(SymbolKind::Function, "new", true)));
    }

    #[test]
    fn unsupported_extension_returns_empty() {
        let symbols = extract_symbols(Path::new("test.py"), "def foo(): pass");
        assert!(symbols.is_empty());
    }
}
