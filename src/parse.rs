use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

/// A symbol extracted from a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: String,
    pub is_public: bool,
    pub line: usize,
    pub end_line: usize,
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
        "py" => Some((
            tree_sitter_python::LANGUAGE.into(),
            include_str!("../queries/python.scm"),
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
                // Filter out `let` and `var` — only `const` declarations are symbols.
                // `let`/`var` are mutable runtime state, not API-level definitions.
                let keyword = symbol_node.child(0).map(|c| c.kind());
                if keyword != Some("const") {
                    continue;
                }

                let declarator = symbol_node
                    .named_children(&mut symbol_node.walk())
                    .find(|c| c.kind() == "variable_declarator");
                let value_kind = declarator
                    .and_then(|d| d.child_by_field_name("value"))
                    .map(|v| v.kind());

                // Detect arrow functions / function expressions assigned to const
                // e.g. `export const foo = () => ...` should be fn, not const
                if matches!(value_kind, Some("arrow_function" | "function_expression" | "generator_function")) {
                    SymbolKind::Function
                }
                // Filter out CommonJS require() calls — these are imports, not definitions
                // e.g. `const SemVer = require('./semver')`
                else if is_require_call(declarator, source) {
                    continue;
                } else {
                    SymbolKind::Const
                }
            }
            "public_field_definition" => {
                let value_kind = symbol_node
                    .child_by_field_name("value")
                    .map(|v| v.kind());

                if matches!(value_kind, Some("arrow_function" | "function_expression" | "generator_function")) {
                    SymbolKind::Function
                } else {
                    // Skip plain data fields (not functions)
                    continue;
                }
            }
            "internal_module" => SymbolKind::Module,
            // Python
            "function_definition" => SymbolKind::Function,
            "class_definition" => SymbolKind::Class,
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

        let is_public = if ext == "py" {
            // Python: underscore prefix = private (convention)
            !name.starts_with('_')
        } else {
            is_public_symbol(symbol_node, source)
        };

        symbols.push(Symbol {
            kind,
            name,
            is_public,
            line: symbol_node.start_position().row + 1,
            end_line: symbol_node.end_position().row + 1,
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
    // Rust: trait methods (both signatures and default implementations) are always public
    if matches!(node.kind(), "function_signature_item" | "function_item")
        && let Some(parent) = node.parent()
        && parent.kind() == "declaration_list"
        && let Some(grandparent) = parent.parent()
        && grandparent.kind() == "trait_item"
    {
        return true;
    }
    // Rust: items in trait implementations are public (they implement a public interface)
    if let Some(parent) = node.parent()
        && parent.kind() == "declaration_list"
        && let Some(grandparent) = parent.parent()
        && grandparent.kind() == "impl_item"
        && grandparent.child_by_field_name("trait").is_some()
    {
        return true;
    }
    // TypeScript class methods and interface methods: public if accessibility_modifier
    // is "public", or if no accessibility_modifier (public by default in TS).
    // Interface methods (method_signature) are always public by design.
    // JS private methods (#method) use private_property_identifier and are always private.
    if matches!(node.kind(), "method_definition" | "method_signature" | "public_field_definition") {
        // JS #private methods are always private
        if node
            .child_by_field_name("name")
            .is_some_and(|n| n.kind() == "private_property_identifier")
        {
            return false;
        }
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

/// Check if a variable_declarator's value is a `require()` call (CommonJS import).
/// Handles both `const x = require('...')` and `const x = require('...').member`.
fn is_require_call(declarator: Option<tree_sitter::Node>, source: &str) -> bool {
    let value = declarator.and_then(|d| d.child_by_field_name("value"));
    match value {
        Some(v) if v.kind() == "call_expression" => v
            .child_by_field_name("function")
            .and_then(|f| f.utf8_text(source.as_bytes()).ok())
            == Some("require"),
        Some(v) if v.kind() == "member_expression" => v
            .child_by_field_name("object")
            .is_some_and(|obj| {
                obj.kind() == "call_expression"
                    && obj
                        .child_by_field_name("function")
                        .and_then(|f| f.utf8_text(source.as_bytes()).ok())
                        == Some("require")
            }),
        _ => false,
    }
}

/// Check if a node is inside a function body (i.e. it's a local declaration, not a module-level one).
fn is_inside_function(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            // Rust
            "function_item" |
            // TypeScript / JavaScript
            "function_declaration" | "function_expression" | "method_definition" | "arrow_function"
            | "generator_function" | "generator_function_declaration" |
            // Python
            "function_definition" => {
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
        // Both trait method signature and trait impl method are public
        assert_eq!(names.iter().filter(|&&(k, n, p)| k == SymbolKind::Function && n == "greet" && p).count(), 2);
        assert!(!names.contains(&(SymbolKind::Function, "greet", false)));
        assert!(names.contains(&(SymbolKind::Function, "new", true)));
    }

    #[test]
    fn unsupported_extension_returns_empty() {
        let symbols = extract_symbols(Path::new("test.rb"), "def foo(): pass");
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
        // Variables inside function expressions should be filtered
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

        // const declarations should be kept
        assert!(names.contains(&"API_URL"));
        assert!(names.contains(&"greet"));
        // let and var declarations should be filtered out
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

        // require() calls should be filtered out (both direct and member access)
        assert!(!names.contains(&"debug"));
        assert!(!names.contains(&"SemVer"));
        assert!(!names.contains(&"parseOptions"));
        assert!(!names.contains(&"EventEmitter"));
        // Regular const values should be kept
        assert!(names.contains(&"ANY"));
        assert!(names.contains(&"MAX_RETRIES"));
        // Arrow functions should still be kept
        assert!(names.contains(&"helper"));
    }

    #[test]
    fn captures_js_private_methods() {
        let source = r#"
export class Parser {
    parse() { return []; }
    #advance() {}
    #reset() {}
}
"#;
        let symbols = extract_symbols(Path::new("test.js"), source);
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind, s.is_public))
            .collect();

        // Public methods
        assert!(info.contains(&("parse", SymbolKind::Function, true)));
        // JS #private methods should be captured as non-public
        assert!(info.contains(&("#advance", SymbolKind::Function, false)));
        assert!(info.contains(&("#reset", SymbolKind::Function, false)));
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

        // Module-level functions
        assert!(info.contains(&("greet", SymbolKind::Function, true)));
        assert!(info.contains(&("_private_helper", SymbolKind::Function, false)));

        // Classes
        assert!(info.contains(&("Animal", SymbolKind::Class, true)));
        assert!(info.contains(&("_PrivateClass", SymbolKind::Class, false)));

        // Methods inside classes
        assert!(info.contains(&("__init__", SymbolKind::Function, false)));
        assert!(info.contains(&("speak", SymbolKind::Function, true)));
        assert!(info.contains(&("_internal", SymbolKind::Function, false)));

        // Nested functions inside methods should be filtered
        // (none in this example, but the mechanism is tested)
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
        // Nested functions inside function bodies should be filtered
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
        let info: Vec<_> = symbols
            .iter()
            .map(|s| (s.name.as_str(), s.kind))
            .collect();

        // Arrow function class fields should be captured as Function
        assert!(info.contains(&("subscribe", SymbolKind::Function)));
        assert!(info.contains(&("publish", SymbolKind::Function)));
        // Regular method should also be captured
        assert!(info.contains(&("normalMethod", SymbolKind::Function)));
        assert!(info.contains(&("constructor", SymbolKind::Function)));
        // Plain data fields should NOT be captured
        assert!(!info.iter().any(|(name, _)| *name == "subscribers"));
    }
}
