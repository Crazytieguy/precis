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
        "tsx" | "jsx" => Some((
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            include_str!("../queries/typescript.scm"),
        )),
        "js" => Some((
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
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
            "lexical_declaration" => {
                // Detect arrow functions / function expressions assigned to const/let
                // e.g. `export const foo = () => ...` should be fn, not const
                let is_function_value = symbol_node
                    .named_children(&mut symbol_node.walk())
                    .filter(|c| c.kind() == "variable_declarator")
                    .any(|decl| {
                        decl.child_by_field_name("value")
                            .map(|v| matches!(v.kind(), "arrow_function" | "function_expression" | "generator_function"))
                            .unwrap_or(false)
                    });
                if is_function_value {
                    SymbolKind::Function
                } else {
                    SymbolKind::Const
                }
            }
            "internal_module" => SymbolKind::Module,
            _ => continue,
        };

        // Filter out symbols nested inside function bodies (local helpers, not module-level)
        if is_inside_function(symbol_node) {
            continue;
        }

        // Filter out Rust test code (#[test] functions, #[cfg(test)] modules)
        if ext == "rs" && is_rust_test_code(symbol_node, source) {
            continue;
        }

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

    dedup_overloads(symbols)
}

/// Collapse consecutive symbols with the same name, keeping only the last in each run.
/// This handles TypeScript/JavaScript method overload signatures: the overload declarations
/// appear as `method_signature` nodes before the actual `method_definition` implementation.
fn dedup_overloads(symbols: Vec<Symbol>) -> Vec<Symbol> {
    let mut result: Vec<Symbol> = Vec::with_capacity(symbols.len());
    for sym in symbols {
        if let Some(last) = result.last()
            && last.name == sym.name
        {
            // Replace the previous entry with this one (the later/implementation version)
            *result.last_mut().unwrap() = sym;
            continue;
        }
        result.push(sym);
    }
    result
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
    // TypeScript class methods and interface methods: public if accessibility_modifier
    // is "public", or if no accessibility_modifier (public by default in TS).
    // Interface methods (method_signature) are always public by design.
    if node.kind() == "method_definition" || node.kind() == "method_signature" {
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

/// Check if a node is inside a function body (i.e. it's a local declaration, not a module-level one).
fn is_inside_function(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            // Rust
            "function_item" |
            // TypeScript / JavaScript
            "function_declaration" | "method_definition" | "arrow_function"
            | "function" | "generator_function" | "generator_function_declaration" => {
                return true;
            }
            _ => {}
        }
        current = parent.parent();
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

/// Check if a node's preceding attribute siblings include a specific attribute.
fn has_preceding_attribute(node: tree_sitter::Node, source: &str, needle: &str) -> bool {
    let mut sibling = node.prev_sibling();
    while let Some(sib) = sibling {
        if sib.kind() == "attribute_item" {
            let text = sib.utf8_text(source.as_bytes()).unwrap_or("");
            if text == needle {
                return true;
            }
        } else {
            break;
        }
        sibling = sib.prev_sibling();
    }
    false
}

/// Check if a Rust node is test code that should be filtered from output.
/// Returns true for `#[test]` functions, `#[cfg(test)]` modules, and anything nested inside them.
fn is_rust_test_code(node: tree_sitter::Node, source: &str) -> bool {
    if has_preceding_attribute(node, source, "#[test]")
        || has_preceding_attribute(node, source, "#[cfg(test)]")
    {
        return true;
    }
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "mod_item"
            && has_preceding_attribute(parent, source, "#[cfg(test)]")
        {
            return true;
        }
        if parent.kind() == "function_item"
            && has_preceding_attribute(parent, source, "#[test]")
        {
            return true;
        }
        current = parent.parent();
    }
    false
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

        // Each overloaded method should appear exactly once (the implementation)
        assert_eq!(names.iter().filter(|(n, _)| *n == "andThen").count(), 1);
        assert_eq!(names.iter().filter(|(n, _)| *n == "combine").count(), 1);
        // The kept version should be the implementation (public method_definition)
        assert!(names.contains(&("andThen", true)));
        assert!(names.contains(&("combine", true)));
        // Non-overloaded method still present
        assert!(names.contains(&("simple", true)));
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
        // Nested symbols inside function bodies should be filtered
        assert!(!names.contains(&"nested_helper"));
        assert!(!names.contains(&"local_fn"));
        assert!(!names.contains(&"LOCAL"));
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
        let kinds: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind))
            .collect();

        // Arrow functions and function expressions should be classified as Function
        assert!(kinds.contains(&("greet", SymbolKind::Function)));
        assert!(kinds.contains(&("add", SymbolKind::Function)));
        assert!(kinds.contains(&("helper", SymbolKind::Function)));
        // Regular const values should remain as Const
        assert!(kinds.contains(&("API_URL", SymbolKind::Const)));
        assert!(kinds.contains(&("MAX_RETRIES", SymbolKind::Const)));
    }
}
