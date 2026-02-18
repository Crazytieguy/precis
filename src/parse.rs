use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::Lang;

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
    /// text heuristics in `format::signature_end_line`).
    pub sig_end_line: Option<usize>,
    /// First line of the doc comment block preceding the symbol (1-indexed),
    /// computed from tree-sitter AST by walking previous sibling comment nodes.
    /// `None` when tree-sitter couldn't find a doc comment (fallback to
    /// text heuristics in `format::doc_comment_start`).
    pub doc_start_line: Option<usize>,
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

/// Build a display name for an import statement.
/// Extracts the module/crate path from language-specific import syntax.
fn import_name(node: tree_sitter::Node, source: &str, lang: Lang) -> String {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("?").trim();
    match lang {
        Lang::Rust => {
            // `use path;` or `pub use path;`
            let rest = text
                .strip_prefix("pub")
                .map(|s| s.trim_start())
                .unwrap_or(text);
            let rest = rest
                .strip_prefix("use")
                .map(|s| s.trim_start())
                .unwrap_or(rest);
            rest.trim_end_matches(';').trim().to_string()
        }
        Lang::JsTs => {
            // `import { foo } from 'bar';` → `bar`
            // `import './styles.css';` → `./styles.css`
            if let Some(from_pos) = text.rfind(" from ") {
                let after = &text[from_pos + 6..];
                after
                    .trim()
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == ';')
                    .to_string()
            } else {
                let rest = text
                    .strip_prefix("import")
                    .map(|s| s.trim_start())
                    .unwrap_or(text);
                rest.trim_matches(|c: char| c == '\'' || c == '"' || c == ';' || c.is_whitespace())
                    .to_string()
            }
        }
        Lang::Go => {
            // Single: `import "fmt"` → `fmt`
            // Grouped: `import (\n"fmt"\n"os"\n)` → `import`
            if text.contains('(') {
                "import".to_string()
            } else if let Some(start) = text.find('"') {
                let rest = &text[start + 1..];
                rest.find('"')
                    .map(|end| rest[..end].to_string())
                    .unwrap_or_else(|| "import".to_string())
            } else {
                "import".to_string()
            }
        }
        Lang::Python => {
            // `from os.path import join` → `os.path`
            // `from . import foo` → `.`
            // `import os` → `os`
            if let Some(rest) = text.strip_prefix("from") {
                let rest = rest.trim_start();
                if let Some(import_pos) = rest.find(" import") {
                    rest[..import_pos].trim().to_string()
                } else {
                    rest.split_whitespace()
                        .next()
                        .unwrap_or("?")
                        .to_string()
                }
            } else {
                let rest = text
                    .strip_prefix("import")
                    .map(|s| s.trim_start())
                    .unwrap_or(text);
                rest.split(&[',', ' ', '\n'][..])
                    .next()
                    .unwrap_or("?")
                    .trim()
                    .to_string()
            }
        }
        Lang::C => {
            // Preserve brackets vs quotes to distinguish system vs local includes:
            // `#include <stdio.h>` → `<stdio.h>` (system)
            // `#include "myheader.h"` → `"myheader.h"` (local/1st-party)
            text.strip_prefix("#include")
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| text.to_string())
        }
        Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => "?".to_string(),
    }
}

/// Determine if an import is 1st-party (local/relative) based on its name.
///
/// 1st-party imports reference code within the same project and are higher
/// signal for understanding a file's role and dependencies.
fn is_first_party_import(name: &str, lang: Lang) -> bool {
    match lang {
        // Rust: `crate::`, `self::`, `super::` are local.
        Lang::Rust => {
            name.starts_with("crate::") || name.starts_with("self::") || name.starts_with("super::")
        }
        // JS/TS: relative paths (`./`, `../`) are local.
        Lang::JsTs => name.starts_with("./") || name.starts_with("../"),
        // Go: imports containing only one path segment (e.g. "fmt", "os")
        // are stdlib; anything with a dot in the first segment (e.g.
        // "github.com/...") is external. We can't distinguish the project's
        // own module path without reading go.mod, so all multi-segment
        // imports are treated as 3rd-party for now.
        Lang::Go => false,
        // Python: relative imports (leading dot) are local.
        Lang::Python => name.starts_with('.'),
        // C: `#include "header.h"` (quoted) is local, `#include <header.h>` (angle) is system.
        Lang::C => name.starts_with('"'),
        Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => false,
    }
}

/// Check if a file extension is supported for symbol extraction.
pub fn is_supported_extension(ext: &str) -> bool {
    Lang::from_extension(ext).is_some()
}

/// Returns the tree-sitter language and query for a file extension, if supported.
/// Uses [`Lang::from_extension`] as the canonical extension check, then selects
/// the appropriate tree-sitter grammar (e.g. TSX vs TypeScript for `.tsx`).
fn language_for_extension(ext: &str) -> Option<(Language, &'static str)> {
    let lang = Lang::from_extension(ext)?;
    Some(match (lang, ext) {
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
        (Lang::C, _) => (
            tree_sitter_c::LANGUAGE.into(),
            include_str!("../queries/c.scm"),
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
    })
}

/// Extract symbols from a source file.
pub fn extract_symbols(path: &Path, source: &str) -> Vec<Symbol> {
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext,
        None => return vec![],
    };

    let lang = match Lang::from_extension(ext) {
        Some(l) => l,
        None => return vec![],
    };

    let (language, query_src) = match language_for_extension(ext) {
        Some(pair) => pair,
        None => return vec![],
    };

    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("language version mismatch");

    let tree = match parser.parse(source, None) {
        Some(tree) => tree,
        None => return vec![],
    };

    let query = Query::new(&language, query_src).expect("invalid query");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    let symbol_idx = query
        .capture_index_for_name("symbol")
        .expect("missing @symbol capture");
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
            // TypeScript / Go
            "function_declaration" | "method_definition" | "method_signature"
            | "abstract_method_signature" | "method_declaration" => SymbolKind::Function,
            "class_declaration" | "abstract_class_declaration" => SymbolKind::Class,
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
                if matches!(
                    value_kind,
                    Some("arrow_function" | "function_expression" | "generator_function")
                ) {
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
                let value_kind = symbol_node.child_by_field_name("value").map(|v| v.kind());

                if matches!(
                    value_kind,
                    Some("arrow_function" | "function_expression" | "generator_function")
                ) {
                    SymbolKind::Function
                } else {
                    // Skip plain data fields (not functions)
                    continue;
                }
            }
            "internal_module" => SymbolKind::Module,
            // Go
            "type_spec" => {
                // Determine kind from the type child (struct_type, interface_type, or other)
                let type_child = symbol_node.child_by_field_name("type").map(|t| t.kind());
                match type_child {
                    Some("struct_type") => SymbolKind::Struct,
                    Some("interface_type") => SymbolKind::Interface,
                    _ => SymbolKind::TypeAlias,
                }
            }
            "type_alias" => SymbolKind::TypeAlias,
            "const_declaration" | "var_declaration" => {
                // Only capture grouped declarations (const (...) / var (...)).
                // Standalone declarations are captured via inner const_spec/var_spec.
                // Detect grouping via AST: grouped const blocks have a "(" child,
                // grouped var blocks wrap entries in a "var_spec_list" child.
                let is_grouped = (0..symbol_node.child_count())
                    .filter_map(|i| symbol_node.child(i))
                    .any(|c| c.kind() == "(" || c.kind() == "var_spec_list");
                if !is_grouped {
                    continue;
                }
                if symbol_node.kind() == "const_declaration" {
                    SymbolKind::Const
                } else {
                    SymbolKind::Static
                }
            }
            "const_spec" => SymbolKind::Const,
            "var_spec" => SymbolKind::Static,
            // C
            "function_definition" if lang == Lang::C => SymbolKind::Function,
            // C type specifiers: only capture definitions (with body), not forward
            // declarations. Skip specifiers inside typedef — the typedef node
            // captures the whole thing.
            "struct_specifier" | "union_specifier" | "enum_specifier" => {
                if symbol_node.child_by_field_name("body").is_none() {
                    continue;
                }
                if symbol_node
                    .parent()
                    .is_some_and(|p| p.kind() == "type_definition")
                {
                    continue;
                }
                if symbol_node.kind() == "enum_specifier" {
                    SymbolKind::Enum
                } else {
                    SymbolKind::Struct
                }
            }
            "type_definition" => SymbolKind::TypeAlias,
            "preproc_def" | "preproc_function_def" => SymbolKind::Macro,
            "preproc_include" => SymbolKind::Import,
            // C: declaration nodes can be function prototypes or global variables
            "declaration" if lang == Lang::C => {
                // Check if this is a function prototype (has function_declarator)
                if has_descendant_kind(symbol_node, "function_declarator") {
                    SymbolKind::Function
                } else {
                    SymbolKind::Static
                }
            }
            // Python
            "function_definition" => SymbolKind::Function,
            "class_definition" => SymbolKind::Class,
            // Markdown / JSON / TOML / YAML
            "atx_heading" | "setext_heading" | "pair" | "table" | "table_array_element"
            | "block_mapping_pair" => SymbolKind::Section,
            // Imports
            "use_declaration" => SymbolKind::Import,              // Rust
            "import_statement" => {
                // Python import_statement or TypeScript import_statement
                if lang == Lang::Python || lang == Lang::JsTs {
                    SymbolKind::Import
                } else {
                    continue;
                }
            }
            "import_from_statement" => SymbolKind::Import,        // Python
            "import_declaration" => SymbolKind::Import,            // Go
            // Python module-level assignments (constants, type variables, dunder attrs)
            "expression_statement" => {
                if lang != Lang::Python {
                    continue;
                }
                // Must be at module level (direct child of module node)
                if symbol_node.parent().map(|p| p.kind()) != Some("module") {
                    continue;
                }
                // Check if the assignment has a type annotation
                let assignment = symbol_node
                    .named_children(&mut symbol_node.walk())
                    .find(|c| c.kind() == "assignment");
                let has_type_annotation = assignment
                    .map(|a| {
                        let mut cursor = a.walk();
                        a.children(&mut cursor).any(|c| c.kind() == "type")
                    })
                    .unwrap_or(false);
                if !has_type_annotation {
                    // No type annotation — only keep UPPER_CASE or dunder names
                    let name_text = name_idx
                        .and_then(|idx| m.captures.iter().find(|c| c.index == idx))
                        .and_then(|c| c.node.utf8_text(source.as_bytes()).ok())
                        .unwrap_or("");
                    let is_upper = !name_text.is_empty()
                        && name_text.bytes().all(|b| b.is_ascii_uppercase() || b == b'_')
                        && name_text.bytes().any(|b| b.is_ascii_uppercase());
                    let is_dunder =
                        name_text.starts_with("__") && name_text.ends_with("__");
                    if !is_upper && !is_dunder {
                        continue;
                    }
                }
                SymbolKind::Const
            }
            _ => continue,
        };

        // Filter out symbols nested inside function bodies (local helpers, not module-level)
        if is_inside_function(symbol_node) {
            continue;
        }

        // Filter out Rust test code (#[test] functions, #[cfg(test)] modules)
        if lang == Lang::Rust && is_rust_test_code(symbol_node, source) {
            continue;
        }

        // Filter out symbols nested inside Rust `const _: T = { ... }` blocks
        // (build probes, type assertions — nothing inside is accessible)
        if lang == Lang::Rust && is_inside_rust_anon_const(symbol_node, source) {
            continue;
        }

        let name = if kind == SymbolKind::Impl {
            impl_name(symbol_node, source)
        } else if kind == SymbolKind::Import {
            import_name(symbol_node, source, lang)
        } else if matches!(symbol_node.kind(), "const_declaration" | "var_declaration") {
            // Grouped const/var block: use keyword as display name
            (if symbol_node.kind() == "const_declaration" { "const" } else { "var" }).to_string()
        } else if symbol_node.kind() == "type_definition" {
            // C typedef: extract the declarator name (last type_identifier in the declarator)
            typedef_name(symbol_node, source)
        } else if symbol_node.kind() == "declaration" && lang == Lang::C {
            // C declaration: extract name from the declarator
            c_declaration_name(symbol_node, source)
        } else {
            match name_idx.and_then(|idx| m.captures.iter().find(|c| c.index == idx)) {
                Some(c) => c
                    .node
                    .utf8_text(source.as_bytes())
                    .unwrap_or("?")
                    .trim()
                    .to_string(),
                None => continue,
            }
        };

        // Strip trailing badge markdown from section heading names.
        // Many READMEs have `# Project [![badge](url)](link)` — the badge URLs
        // waste token budget and provide no useful information.
        let name = if kind == SymbolKind::Section {
            crate::format::strip_heading_badges(&name).to_string()
        } else {
            name
        };

        // Filter out blank identifier `_` in Go and Rust.
        // Go: `_ = iota` in const blocks (skips iota values), `var _ error = ...` (interface checks).
        // Rust: `const _: () = { ... }` (build probes, type assertions, compile-time checks).
        if matches!(lang, Lang::Go | Lang::Rust) && name == "_" {
            continue;
        }

        let is_public = if kind == SymbolKind::Import {
            // Only Rust `pub use` re-exports are public; all other imports are private
            lang == Lang::Rust && is_public_symbol(symbol_node, source)
        } else {
            match lang {
                Lang::Go => name.starts_with(|c: char| c.is_ascii_uppercase()),
                Lang::Python => !name.starts_with('_') || (name.starts_with("__") && name.ends_with("__")),
                Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml => true,
                // C: non-static symbols are public; underscore-prefixed names are conventionally private
                Lang::C => !is_c_static(symbol_node, source) && !name.starts_with('_'),
                _ => is_public_symbol(symbol_node, source),
            }
        };

        // Rust: #[doc(hidden)] items are technically `pub` (needed for macro
        // internals, proc-macro bridges, etc.) but the crate author explicitly
        // doesn't want them shown to users. Treat as non-public so the
        // scheduler deprioritizes them.
        let is_public = if is_public
            && lang == Lang::Rust
            && has_preceding_attribute(symbol_node, source, "#[doc(hidden)]")
        {
            false
        } else {
            is_public
        };

        let is_first_party = if kind == SymbolKind::Import {
            is_first_party_import(&name, lang)
        } else {
            false
        };

        symbols.push(Symbol {
            kind,
            name,
            is_public,
            is_first_party,
            line: symbol_node.start_position().row + 1,
            // C preprocessor directives include trailing newlines, so their
            // end_position is at column 0 of the next line. Adjust to avoid
            // claiming an extra line that causes overlapping output.
            end_line: if lang == Lang::C
                && matches!(symbol_node.kind(), "preproc_include" | "preproc_def" | "preproc_function_def")
                && symbol_node.end_position().column == 0
                && symbol_node.end_position().row > symbol_node.start_position().row
            {
                symbol_node.end_position().row
            } else {
                symbol_node.end_position().row + 1
            },
            sig_end_line: compute_sig_end_line(symbol_node, lang),
            doc_start_line: compute_doc_start_line(symbol_node, source, lang),
        });
    }

    dedup_overloads(symbols, lang)
}

/// Collapse consecutive function symbols with the same name, keeping only the last in each run.
/// Go is excluded because it allows multiple `init()` functions in one file.
fn dedup_overloads(symbols: Vec<Symbol>, lang: Lang) -> Vec<Symbol> {
    if lang == Lang::Go {
        return symbols;
    }
    let mut result: Vec<Symbol> = Vec::with_capacity(symbols.len());
    for sym in symbols {
        if let Some(last) = result.last()
            && last.name == sym.name
            && last.kind == SymbolKind::Function
            && sym.kind == SymbolKind::Function
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
    // Rust: `pub` keyword appears as a visibility_modifier child.
    // Restricted visibility (`pub(crate)`, `pub(super)`, `pub(in ...)`) is NOT
    // considered public — these are internal items, not part of the crate's API.
    if let Some(vis) = node
        .children(&mut cursor)
        .find(|child| child.kind() == "visibility_modifier")
    {
        let vis_text = vis.utf8_text(source.as_bytes()).unwrap_or("");
        return vis_text == "pub";
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
    if matches!(
        node.kind(),
        "method_definition" | "method_signature" | "abstract_method_signature" | "public_field_definition"
    ) {
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
            // Convention: _prefix means private for concrete methods and fields.
            // Interface signatures and abstract methods are always public (part of the type contract).
            if matches!(node.kind(), "method_definition" | "public_field_definition") {
                let is_underscore_prefixed = node
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .is_some_and(|name| name.starts_with('_'));
                if is_underscore_prefixed {
                    return false;
                }
            }
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
        Some(v) if v.kind() == "call_expression" => {
            v.child_by_field_name("function")
                .and_then(|f| f.utf8_text(source.as_bytes()).ok())
                == Some("require")
        }
        Some(v) if v.kind() == "member_expression" => {
            v.child_by_field_name("object").is_some_and(|obj| {
                obj.kind() == "call_expression"
                    && obj
                        .child_by_field_name("function")
                        .and_then(|f| f.utf8_text(source.as_bytes()).ok())
                        == Some("require")
            })
        }
        _ => false,
    }
}

/// Check if any descendant of a node has the given kind.
fn has_descendant_kind(node: tree_sitter::Node, kind: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == kind {
            return true;
        }
        if has_descendant_kind(child, kind) {
            return true;
        }
    }
    false
}

/// Extract the typedef name from a C `type_definition` node.
/// Handles `typedef struct { ... } Name;`, `typedef int Name;`,
/// `typedef int (*Name)(...)`, etc.
fn typedef_name(node: tree_sitter::Node, source: &str) -> String {
    // The `declarator` field of type_definition contains the name.
    // For simple typedefs: declarator is type_identifier
    // For pointer typedefs: declarator is pointer_declarator wrapping type_identifier
    // For function pointer typedefs: deeply nested
    if let Some(decl) = node.child_by_field_name("declarator")
        && let Some(name) = find_first_of_kind(decl, "type_identifier", source)
    {
        return name;
    }
    "?".to_string()
}

/// Recursively find the first descendant of the given kind in a subtree.
fn find_first_of_kind(node: tree_sitter::Node, target_kind: &str, source: &str) -> Option<String> {
    if node.kind() == target_kind {
        return node.utf8_text(source.as_bytes()).ok().map(|s| s.to_string());
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(name) = find_first_of_kind(child, target_kind, source) {
            return Some(name);
        }
    }
    None
}

/// Extract the name from a C `declaration` node (global variable or function prototype).
fn c_declaration_name(node: tree_sitter::Node, source: &str) -> String {
    // The declarator field contains the name, possibly nested in pointer_declarator
    // or function_declarator.
    if let Some(decl) = node.child_by_field_name("declarator")
        && let Some(name) = find_first_of_kind(decl, "identifier", source)
    {
        return name;
    }
    "?".to_string()
}

/// Check if a C symbol has `static` storage class (file-scoped, not public).
fn is_c_static(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|child| {
        child.kind() == "storage_class_specifier"
            && child.utf8_text(source.as_bytes()).unwrap_or("") == "static"
    })
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
            // Go
            "method_declaration" | "func_literal" |
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

/// Check if a node is inside a Rust `const _: T = { ... }` block.
/// These anonymous constants are used for build probes, type assertions, and compile-time checks.
/// Nothing inside them is accessible from outside — all nested types, impls, and functions
/// are internal to the constant expression and should be filtered from output.
fn is_inside_rust_anon_const(node: tree_sitter::Node, source: &str) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "const_item" {
            // Check if this const_item has name `_`
            let mut cursor = parent.walk();
            let is_anon = parent.children(&mut cursor).any(|child| {
                child.kind() == "identifier"
                    && child.utf8_text(source.as_bytes()).unwrap_or("") == "_"
            });
            if is_anon {
                return true;
            }
        }
        current = parent.parent();
    }
    false
}

/// Compute the signature end line (1-indexed) from the tree-sitter AST.
///
/// For C-like languages (Rust, TS/JS, Go, C), the body child (block,
/// field_declaration_list, etc.) starts at `{`, so `sig_end = body.start.row + 1`.
///
/// For Python, the body (block) starts at the first statement after `:`.
/// We find the `:` token that's a direct child of the function/class definition
/// and use its row.
///
/// For Go `type_spec`, the body braces are inside the `type` child (struct_type,
/// interface_type), so we look for `{` there.
///
/// Returns `None` for nodes without a detectable body (type aliases, constants,
/// method signatures, etc.) — the caller should fall back to text heuristics.
fn compute_sig_end_line(node: tree_sitter::Node, lang: Lang) -> Option<usize> {
    if lang == Lang::Python {
        let body = node.child_by_field_name("body")?;
        // Walk backwards from body to find the ':' token
        let body_id = body.id();
        let mut body_idx = None;
        for i in 0..node.child_count() {
            if node.child(i).is_some_and(|c| c.id() == body_id) {
                body_idx = Some(i);
                break;
            }
        }
        if let Some(idx) = body_idx {
            for i in (0..idx).rev() {
                if let Some(child) = node.child(i)
                    && child.kind() == ":"
                {
                    return Some(child.start_position().row + 1);
                }
            }
        }
        return None;
    }

    // C-like languages: body field starts at '{'
    if let Some(body) = node.child_by_field_name("body") {
        return Some(body.start_position().row + 1);
    }

    // Go type_spec: the type child (struct_type, interface_type) contains the body.
    // Look for '{' token inside the type child.
    if node.kind() == "type_spec"
        && let Some(type_child) = node.child_by_field_name("type")
    {
        for i in 0..type_child.child_count() {
            if let Some(child) = type_child.child(i)
                && child.kind() == "{"
            {
                return Some(child.start_position().row + 1);
            }
        }
    }

    None
}

/// Compute the first line (1-indexed) of the doc comment block preceding a symbol,
/// using tree-sitter AST sibling navigation. Returns `None` when no doc comment is
/// found (fallback to text heuristics in `format::doc_comment_start`).
///
/// Walks backwards through previous named siblings looking for comment nodes that
/// qualify as doc comments for the given language. For symbols wrapped in
/// `export_statement` (TypeScript) or `decorated_definition` (Python), checks the
/// wrapper's siblings when the symbol's own siblings don't have comments.
fn compute_doc_start_line(symbol_node: tree_sitter::Node, source: &str, lang: Lang) -> Option<usize> {
    if matches!(lang, Lang::Markdown | Lang::Json | Lang::Toml | Lang::Yaml) {
        return None;
    }

    // Find the nearest doc comment above this symbol.
    // Check the symbol's own prev sibling first, then wrapper parent nodes
    // (export_statement for TypeScript, decorated_definition for Python decorators).
    // Gap limit: allow at most 1 blank line between doc comment and symbol/wrapper.
    // Larger gaps indicate section separators, not doc comments.
    // Max gap of 2 rows (allows 1 blank line between doc comment and symbol/wrapper).
    // Larger gaps indicate section separators, not doc comments.
    let max_gap = 2;

    let first_doc = symbol_node
        .prev_named_sibling()
        .filter(|n| {
            is_doc_comment_node(*n, source, lang)
                && symbol_node.start_position().row.saturating_sub(n.end_position().row) <= max_gap
        })
        .or_else(|| {
            let parent = symbol_node.parent()?;
            if matches!(parent.kind(), "export_statement" | "decorated_definition") {
                parent.prev_named_sibling().filter(|n| {
                    is_doc_comment_node(*n, source, lang)
                        && parent.start_position().row.saturating_sub(n.end_position().row)
                            <= max_gap
                })
            } else {
                None
            }
        })?;

    // Walk backwards through contiguous doc comment siblings to find the block start
    let mut doc_start_row = first_doc.start_position().row;
    let mut current = first_doc;

    while let Some(prev) = current.prev_named_sibling() {
        if !is_doc_comment_node(prev, source, lang) {
            break;
        }
        // Require contiguity within the doc block (no blank lines between comments)
        if current.start_position().row.saturating_sub(prev.end_position().row) > 1 {
            break;
        }
        doc_start_row = prev.start_position().row;
        current = prev;
    }

    Some(doc_start_row + 1) // 1-indexed
}

/// Check whether a tree-sitter node is a doc comment for the given language.
fn is_doc_comment_node(node: tree_sitter::Node, source: &str, lang: Lang) -> bool {
    if !matches!(node.kind(), "comment" | "line_comment" | "block_comment") {
        return false;
    }
    let text = match node.utf8_text(source.as_bytes()) {
        Ok(t) => t.trim(),
        Err(_) => return false,
    };
    match lang {
        Lang::Rust => {
            // Only outer doc comments (/// and /**) document the next item.
            // Inner doc comments (//! and /*!) document the containing module.
            text.starts_with("///") || text.starts_with("/**")
        }
        Lang::Go | Lang::C => true, // godoc / doxygen: any comment preceding a declaration
        Lang::JsTs => text.starts_with("/**"), // JSDoc only
        Lang::Python => text.starts_with('#'),
        _ => false,
    }
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
        if parent.kind() == "mod_item" && has_preceding_attribute(parent, source, "#[cfg(test)]") {
            return true;
        }
        if parent.kind() == "function_item" && has_preceding_attribute(parent, source, "#[test]") {
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
        // Both trait method signature and trait impl method are public
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
        // Anonymous constants and everything inside them should be filtered
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
        let kinds: Vec<_> = symbols.iter().map(|s| (s.name.as_str(), s.kind)).collect();

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
    _internal() {}
    _prepareForParse() {}
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
        // JS _prefix convention methods should be captured as non-public
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

        // Interface methods are always public, even with _ prefix
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

        // Module-level functions
        assert!(info.contains(&("greet", SymbolKind::Function, true)));
        assert!(info.contains(&("_private_helper", SymbolKind::Function, false)));

        // Classes
        assert!(info.contains(&("Animal", SymbolKind::Class, true)));
        assert!(info.contains(&("_PrivateClass", SymbolKind::Class, false)));

        // Methods inside classes
        assert!(info.contains(&("__init__", SymbolKind::Function, true)));
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

        // Type-annotated assignments are always captured
        assert!(info.contains(&("VERSION", SymbolKind::Const, true)));
        assert!(info.contains(&("MAX_SIZE", SymbolKind::Const, true)));

        // UPPER_CASE untyped assignments are captured
        assert!(info.contains(&("T", SymbolKind::Const, true)));
        assert!(info.contains(&("UPPER_CASE", SymbolKind::Const, true)));

        // Dunder names are captured and treated as public
        assert!(info.contains(&("__all__", SymbolKind::Const, true)));
        assert!(info.contains(&("__version__", SymbolKind::Const, true)));

        // Private UPPER_CASE is captured but marked private
        assert!(info.contains(&("_PRIVATE_CONST", SymbolKind::Const, false)));

        // lower_case without type annotation is NOT captured
        assert!(!info.iter().any(|(name, _, _)| *name == "lower_case"));
        assert!(!info.iter().any(|(name, _, _)| *name == "logger"));

        // Constants inside functions are NOT captured (not module-level)
        assert!(!info.iter().any(|(name, _, _)| *name == "LOCAL_CONST"));

        // Functions should still be captured
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

        // Arrow function class fields should be captured as Function
        assert!(info.contains(&("subscribe", SymbolKind::Function)));
        assert!(info.contains(&("publish", SymbolKind::Function)));
        // Regular method should also be captured
        assert!(info.contains(&("normalMethod", SymbolKind::Function)));
        assert!(info.contains(&("constructor", SymbolKind::Function)));
        // Plain data fields should NOT be captured
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

        // Abstract class should be captured as Class
        assert!(info.contains(&("Base", SymbolKind::Class, true)));
        // Abstract method signature should be captured as Function
        assert!(info.contains(&("method", SymbolKind::Function, true)));
        // Concrete method inside abstract class
        assert!(info.contains(&("concrete", SymbolKind::Function, true)));
        // Derived class
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

        // All headings should be extracted as Section kind
        assert_eq!(symbols.len(), 7);
        assert!(info.contains(&("Introduction", SymbolKind::Section, true, 1)));
        assert!(info.contains(&("Getting Started", SymbolKind::Section, true, 5)));
        assert!(info.contains(&("Installation", SymbolKind::Section, true, 9)));
        assert!(info.contains(&("API Reference", SymbolKind::Section, true, 13)));
        assert!(info.contains(&("`parse(source)`", SymbolKind::Section, true, 15)));
        assert!(info.contains(&("Parameters", SymbolKind::Section, true, 19)));
        assert!(info.contains(&("Contributing", SymbolKind::Section, true, 25)));

        // All headings should be public
        assert!(symbols.iter().all(|s| s.is_public));

        // ATX headings: end_line == line + 1 (node includes trailing newline)
        assert!(symbols.iter().all(|s| s.end_line == s.line + 1));
    }

    #[test]
    fn extracts_setext_headings() {
        let source = "Introduction\n============\n\nSome text.\n\nGetting Started\n---------------\n\nMore text.\n";
        let symbols = extract_symbols(Path::new("test.md"), source);
        assert_eq!(symbols.len(), 2);
        // Setext headings: names are trimmed, end_line extends past the underline
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

        // Struct
        assert!(info.contains(&("Token", SymbolKind::Struct, true)));
        // Interface
        assert!(info.contains(&("Stringer", SymbolKind::Interface, true)));
        // Type alias
        assert!(info.contains(&("Span", SymbolKind::TypeAlias, true)));
        // Const
        assert!(info.contains(&("MaxTokens", SymbolKind::Const, true)));
        // Var (exported)
        assert!(info.contains(&("Version", SymbolKind::Static, true)));
        // Var (unexported)
        assert!(info.contains(&("internal", SymbolKind::Static, false)));
        // Exported function
        assert!(info.contains(&("Process", SymbolKind::Function, true)));
        // Unexported function
        assert!(info.contains(&("helper", SymbolKind::Function, false)));
        // Exported method
        assert!(info.contains(&("String", SymbolKind::Function, true)));
        // Unexported method
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

        // Grouped const block should be captured with keyword name
        assert!(names.contains(&"const"));
        // Named constants should be kept
        assert!(names.contains(&"Reset"));
        assert!(names.contains(&"Bold"));
        assert!(names.contains(&"Faint"));
        assert!(names.contains(&"Underline"));
        // Blank identifier _ should be filtered out (iota skip, interface check)
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

        // Standalone const: captured via const_spec only (no duplicate from const_declaration)
        assert_eq!(
            info.iter().filter(|(n, _)| *n == "MaxItems").count(),
            1,
            "standalone const should appear once"
        );
        // Grouped const block: const_declaration captured as "const"
        assert!(info.contains(&("const", SymbolKind::Const)));
        // Individual entries still captured
        assert!(info.contains(&("Red", SymbolKind::Const)));
        assert!(info.contains(&("Green", SymbolKind::Const)));
        assert!(info.contains(&("Blue", SymbolKind::Const)));

        // Standalone var: captured via var_spec only
        assert_eq!(
            info.iter().filter(|(n, _)| *n == "Version").count(),
            1,
            "standalone var should appear once"
        );
        // Grouped var block: var_declaration captured as "var"
        assert!(info.contains(&("var", SymbolKind::Static)));
        // Individual entries still captured
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

        // Package-level symbols should be captured
        assert!(names.contains(&"Handler"));
        assert!(names.contains(&"Process"));
        assert!(names.contains(&"MaxItems"));
        // Symbols inside func_literal should be filtered out
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
        // Other functions still captured
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

        // Spans: "scripts" should span from its line to "dependencies" line
        let scripts = &symbols[2];
        assert_eq!(scripts.line, 4);
        assert_eq!(scripts.name, "scripts");
        let deps = &symbols[3];
        assert_eq!(deps.line, 8);
    }

    #[test]
    fn json_empty_and_array_roots() {
        // Empty object
        assert!(extract_symbols(Path::new("empty.json"), "{}").is_empty());
        // Array root — no top-level keys
        assert!(extract_symbols(Path::new("arr.json"), "[1, 2, 3]").is_empty());
        // Empty source
        assert!(extract_symbols(Path::new("empty.json"), "").is_empty());
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

        // [package] spans from line 1 to [dependencies] at line 6
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
        // Comments only
        assert!(extract_symbols(Path::new("comments.toml"), "# just a comment\n").is_empty());
    }
}
