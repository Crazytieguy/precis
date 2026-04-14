//! Tree-sitter query execution and classification into (TsGroupKey, TsItem).

pub(crate) mod ast;
pub(crate) mod module_doc;
pub(crate) mod visibility;

use std::collections::HashSet;
use std::mem::discriminant;
use std::path::Path;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, QueryCursor};

use crate::Lang;
use crate::group::TsGroupKey;
use crate::group::TsItem;
use crate::store::LanguageConfig;

/// Extract top-level items from a parsed source file, classifying each
/// directly into a `TsGroupKey` paired with a `TsItem`.
pub fn extract_items<'t>(
    display_path: &'t Path,
    source: &'t str,
    tree: &'t tree_sitter::Tree,
    config: &LanguageConfig,
) -> Vec<(TsGroupKey, TsItem<'t>)> {
    let lang = config.lang;
    let root = tree.root_node();

    // `lines` is only consulted by Heading arms (markdown/toml/json/yaml).
    // Skipping the collect() for other languages saves an allocation
    // proportional to file line count on the hot path.
    let lines: Vec<&str> = if matches!(lang, Lang::Markdown | Lang::Toml | Lang::Json | Lang::Yaml)
    {
        source.lines().collect()
    } else {
        Vec::new()
    };
    let mod_names = collect_top_level_mod_names(root, source, lang);
    let export_names = collect_js_export_names(root, source, lang);

    let mut items: Vec<(TsGroupKey, TsItem<'t>)> = Vec::new();

    if let Some((node, end_row)) = module_doc::detect_module_doc(root, source, lang, display_path) {
        items.push((
            TsGroupKey::ModuleDocFirst,
            TsItem {
                path: display_path,
                source,
                node,
                end_line: end_row + 1,
            },
        ));
    }

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&config.query, root, source.as_bytes());

    let symbol_idx = config.symbol_idx;

    while let Some(m) = matches.next() {
        let symbol_node = match m.captures.iter().find(|c| c.index == symbol_idx) {
            Some(c) => c.node,
            None => continue,
        };

        if ast::is_inside_function(symbol_node) {
            continue;
        }
        if lang == Lang::Rust && ast::is_rust_test_code(symbol_node, source) {
            continue;
        }
        if lang == Lang::Rust && ast::is_inside_rust_anon_const(symbol_node, source) {
            continue;
        }

        if matches!(lang, Lang::Go | Lang::Rust) {
            let ident = symbol_node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .map(str::trim);
            if ident == Some("_") {
                continue;
            }
        }

        // C++ template_declaration wraps the inner item; classify the inner
        // node but use the outer for range + doc detection so the template
        // parameters are captured in the rendered range.
        let range_node = if lang == Lang::C
            && symbol_node
                .parent()
                .is_some_and(|p| p.kind() == "template_declaration")
        {
            symbol_node.parent().unwrap()
        } else {
            symbol_node
        };

        let documented = ast::is_documented(range_node, source, lang);

        let Some(key) = classify(
            symbol_node,
            source,
            lang,
            display_path,
            &lines,
            &mod_names,
            &export_names,
            documented,
        ) else {
            continue;
        };

        let end_line = compute_end_line(range_node);

        items.push((
            key,
            TsItem {
                path: display_path,
                source,
                node: range_node,
                end_line,
            },
        ));
    }

    filter_nested_items(&mut items);
    dedup_line_overlaps(&mut items);
    extend_section_ranges(&mut items, source);
    dedup_overloads(&mut items, lang, source);

    assert!(
        items
            .windows(2)
            .all(|w| w[0].1.end_line <= w[1].1.start_line()),
        "D4 violation: top-level items overlap after nesting filter in {}",
        display_path.display()
    );

    items
}

// ---------------------------------------------------------------------------
// Classification: node -> Option<TsGroupKey>
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn classify<'t>(
    node: Node<'t>,
    source: &str,
    lang: Lang,
    path: &Path,
    lines: &[&str],
    mod_names: &[&str],
    export_names: &HashSet<&str>,
    documented: bool,
) -> Option<TsGroupKey> {
    use TsGroupKey::*;

    let public = || {
        if visibility::symbol_visibility(node, source, lang) {
            return true;
        }
        if lang == Lang::JsTs
            && !export_names.is_empty()
            && let Some(name_node) = node.child_by_field_name("name")
            && let Ok(name) = name_node.utf8_text(source.as_bytes())
        {
            return export_names.contains(name);
        }
        false
    };

    match node.kind() {
        // ---- Rust ----
        "function_item" | "function_signature_item" => Some(FunctionName {
            documented,
            public: public(),
        }),
        "struct_item" => Some(StructName {
            documented,
            public: public(),
        }),
        "enum_item" => Some(EnumName {
            documented,
            public: public(),
        }),
        "trait_item" => Some(TraitName {
            documented,
            public: public(),
        }),
        "impl_item" => {
            let trait_node = node.child_by_field_name("trait");
            let is_boilerplate = trait_node.is_some_and(|t| is_boilerplate_trait_impl(t, source));
            Some(ImplBlock {
                is_trait_impl: trait_node.is_some(),
                is_boilerplate_trait: is_boilerplate,
            })
        }
        "type_item" => Some(TypeAliasName {
            documented,
            public: public(),
        }),
        "const_item" | "static_item" => Some(ConstName {
            documented,
            public: public(),
        }),
        "macro_definition" => Some(MacroName {
            documented,
            public: visibility::macro_visibility(node, source, lang),
            preproc: false,
        }),
        "mod_item" if node.child_by_field_name("body").is_none() => Some(Import {
            first_party: true,
            reexport: false,
        }),
        "mod_item" => None,

        // ---- TypeScript / JavaScript ----
        "function_declaration"
        | "method_definition"
        | "method_signature"
        | "abstract_method_signature"
        | "method_declaration" => Some(FunctionName {
            documented,
            public: public(),
        }),
        "class_declaration" | "abstract_class_declaration" => Some(ClassName {
            documented,
            public: public(),
        }),
        "interface_declaration" => Some(InterfaceName {
            documented,
            public: public(),
        }),
        "enum_declaration" => Some(EnumName {
            documented,
            public: public(),
        }),
        "type_alias_declaration" => Some(TypeAliasName {
            documented,
            public: public(),
        }),
        "lexical_declaration" => classify_js_lexical(node, source, lang, documented, export_names),
        "public_field_definition" => {
            let value_kind = node.child_by_field_name("value").map(|v| v.kind());
            if matches!(
                value_kind,
                Some("arrow_function" | "function_expression" | "generator_function")
            ) {
                Some(FunctionName {
                    documented,
                    public: public(),
                })
            } else {
                None
            }
        }
        "internal_module" => None,
        "export_statement" if lang == Lang::JsTs => classify_js_export(node, source),
        "expression_statement" if lang == Lang::JsTs => {
            classify_js_cjs_export(node, source, documented)
        }

        // ---- Go ----
        "type_spec" => {
            let type_child = node.child_by_field_name("type").map(|t| t.kind());
            match type_child {
                Some("struct_type") => Some(StructName {
                    documented,
                    public: public(),
                }),
                Some("interface_type") => Some(InterfaceName {
                    documented,
                    public: public(),
                }),
                _ => Some(TypeAliasName {
                    documented,
                    public: public(),
                }),
            }
        }
        "type_alias" => Some(TypeAliasName {
            documented,
            public: public(),
        }),
        "const_declaration" | "var_declaration" => {
            let is_grouped = (0..node.child_count())
                .filter_map(|i| node.child(i))
                .any(|c| c.kind() == "(" || c.kind() == "var_spec_list");
            if !is_grouped {
                return None;
            }
            Some(ConstName {
                documented,
                public: public(),
            })
        }
        "const_spec" | "var_spec" => Some(ConstName {
            documented,
            public: public(),
        }),

        // ---- C / C++ ----
        "function_definition" => Some(FunctionName {
            documented,
            public: public(),
        }),
        "class_specifier" | "struct_specifier" | "union_specifier" | "enum_specifier" => {
            node.child_by_field_name("body")?;
            if node.parent().is_some_and(|p| p.kind() == "type_definition") {
                return None;
            }
            match node.kind() {
                "enum_specifier" => Some(EnumName {
                    documented,
                    public: public(),
                }),
                "class_specifier" => Some(ClassName {
                    documented,
                    public: public(),
                }),
                _ => Some(StructName {
                    documented,
                    public: public(),
                }),
            }
        }
        "type_definition" => {
            if is_simple_typedef_alias(node) {
                return None;
            }
            Some(TypeAliasName {
                documented,
                public: public(),
            })
        }
        "preproc_def" => {
            if is_c_header_guard(node, source, path) {
                return None;
            }
            if is_platform_type_define(node, source) {
                return None;
            }
            Some(MacroName {
                documented,
                public: visibility::macro_visibility(node, source, lang),
                preproc: true,
            })
        }
        "preproc_function_def" => Some(MacroName {
            documented,
            public: visibility::macro_visibility(node, source, lang),
            preproc: true,
        }),
        "preproc_include" => Some(Import {
            first_party: is_first_party_import(node, source, lang),
            reexport: false,
        }),
        "declaration" if lang == Lang::C => {
            if ast::find_descendant_of_kind(node, "function_declarator").is_some() {
                Some(FunctionName {
                    documented,
                    public: public(),
                })
            } else {
                Some(ConstName {
                    documented,
                    public: public(),
                })
            }
        }
        "namespace_definition" => None,
        "alias_declaration" => Some(TypeAliasName {
            documented,
            public: public(),
        }),

        // ---- Java ----
        "record_declaration" => Some(StructName {
            documented,
            public: public(),
        }),
        "annotation_type_declaration" => Some(InterfaceName {
            documented,
            public: public(),
        }),
        "constructor_declaration" | "annotation_type_element_declaration" => Some(FunctionName {
            documented,
            public: public(),
        }),
        "module_declaration" => None,
        "field_declaration" if lang == Lang::Java => {
            if !java_field_is_static_final(node, source) {
                return None;
            }
            Some(ConstName {
                documented,
                public: public(),
            })
        }
        "constant_declaration" if lang == Lang::Java => Some(ConstName {
            documented,
            public: public(),
        }),

        // ---- Python ----
        "class_definition" => Some(ClassName {
            documented,
            public: public(),
        }),
        "expression_statement" if lang == Lang::Python => {
            let name = python_module_const_name(node, source)?;
            let public = !name.starts_with('_') || (name.starts_with("__") && name.ends_with("__"));
            Some(ConstName { documented, public })
        }

        // ---- Markdown / JSON / TOML / YAML ----
        "atx_heading" | "setext_heading" => {
            let start = node.start_position().row;
            let end = compute_end_line(node);
            let level = crate::classify::detect_heading_depth(lines, start, end);
            let heading_line = lines.get(start).copied().unwrap_or("");
            let stripped = crate::classify::strip_heading_badges(heading_line);
            let boilerplate = crate::classify::is_boilerplate_heading(stripped);
            Some(Heading { level, boilerplate })
        }
        "table" | "table_array_element" => {
            // TOML: level from dot count in [a.b.c] header.
            let start = node.start_position().row;
            let line = lines.get(start).copied().unwrap_or("");
            let key = line
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim();
            let level = key.chars().filter(|&c| c == '.').count() as u8 + 1;
            Some(Heading {
                level,
                boilerplate: false,
            })
        }
        "pair" | "block_mapping_pair" => Some(Heading {
            level: 1,
            boilerplate: false,
        }),

        // ---- Imports ----
        "use_declaration" => {
            let is_crate_root = path.file_name().is_some_and(|f| f == "lib.rs");
            Some(Import {
                first_party: is_first_party_import(node, source, lang),
                reexport: !is_crate_root && is_rust_reexport(node, source, mod_names),
            })
        }
        "import_statement" if lang == Lang::Python || lang == Lang::JsTs => Some(Import {
            first_party: is_first_party_import(node, source, lang),
            reexport: false,
        }),
        "import_from_statement" => Some(Import {
            first_party: is_first_party_import(node, source, lang),
            reexport: is_python_reexport(node, source),
        }),
        "import_declaration" => Some(Import {
            first_party: is_first_party_import(node, source, lang),
            reexport: false,
        }),

        // ---- Lua ----
        "variable_declaration" | "assignment_statement" if lang == Lang::Lua => {
            let is_function = lua_rhs_is_function(node);
            if is_function {
                Some(FunctionName {
                    documented,
                    public: public(),
                })
            } else {
                Some(ConstName {
                    documented,
                    public: public(),
                })
            }
        }

        _ => None,
    }
}

fn classify_js_lexical(
    node: Node,
    source: &str,
    lang: Lang,
    documented: bool,
    export_names: &HashSet<&str>,
) -> Option<TsGroupKey> {
    use TsGroupKey::*;

    let declarator = node
        .named_children(&mut node.walk())
        .find(|c| c.kind() == "variable_declarator");

    if is_require_call(declarator, source) {
        return Some(TsGroupKey::Import {
            first_party: is_require_first_party(declarator, source),
            reexport: false,
        });
    }

    let keyword = node.child(0).map(|c| c.kind());
    if keyword != Some("const") {
        return None;
    }

    let base_public = visibility::symbol_visibility(node, source, lang);
    let public = base_public
        || (lang == Lang::JsTs
            && !export_names.is_empty()
            && declarator
                .and_then(|d| d.child_by_field_name("name"))
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .is_some_and(|name| export_names.contains(name)));

    let value_kind = declarator
        .and_then(|d| d.child_by_field_name("value"))
        .map(|v| v.kind());
    if matches!(
        value_kind,
        Some("arrow_function" | "function_expression" | "generator_function")
    ) {
        Some(FunctionName { documented, public })
    } else {
        Some(ConstName { documented, public })
    }
}

/// Classify a JS/TS `export_statement`. Returns `None` for declaration
/// exports (the inner declaration is captured separately); returns an Import
/// group for re-exports (`export * from`, `export { } from`, `export { }`).
fn classify_js_export(node: Node, source: &str) -> Option<TsGroupKey> {
    let mut cursor = node.walk();
    let has_declaration = node.children(&mut cursor).any(|c| {
        matches!(
            c.kind(),
            "function_declaration"
                | "class_declaration"
                | "abstract_class_declaration"
                | "lexical_declaration"
                | "type_alias_declaration"
                | "interface_declaration"
                | "enum_declaration"
                | "internal_module"
        )
    });
    if has_declaration {
        return None;
    }

    // `export default expr` — not a re-export
    cursor = node.walk();
    let has_default = node.children(&mut cursor).any(|c| c.kind() == "default");
    if has_default {
        return None;
    }

    // Remaining cases: `export * from '...'`, `export { } from '...'`, `export { }`
    cursor = node.walk();
    let has_source = node.children(&mut cursor).any(|c| c.kind() == "string");
    let first_party = if has_source {
        is_first_party_import(node, source, Lang::JsTs)
    } else {
        true
    };
    Some(TsGroupKey::Import {
        first_party,
        reexport: true,
    })
}

fn classify_js_cjs_export(node: Node, source: &str, documented: bool) -> Option<TsGroupKey> {
    let assign = node
        .named_children(&mut node.walk())
        .find(|c| c.kind() == "assignment_expression")?;
    let left = assign.child_by_field_name("left")?;
    if left.kind() != "member_expression" {
        return None;
    }
    let obj = left.child_by_field_name("object")?;
    let obj_text = obj.utf8_text(source.as_bytes()).ok()?;
    match obj_text {
        "exports" => Some(TsGroupKey::ConstName {
            documented,
            public: true,
        }),
        "module" => {
            let prop = left.child_by_field_name("property")?;
            let prop_text = prop.utf8_text(source.as_bytes()).ok()?;
            if prop_text != "exports" {
                return None;
            }
            let rhs = assign.child_by_field_name("right")?;
            if is_rhs_require(rhs, source) {
                Some(TsGroupKey::Import {
                    first_party: is_rhs_require_first_party(rhs, source),
                    reexport: true,
                })
            } else {
                Some(TsGroupKey::ConstName {
                    documented,
                    public: true,
                })
            }
        }
        _ => None,
    }
}

fn is_rhs_require(rhs: Node, source: &str) -> bool {
    if rhs.kind() == "call_expression" {
        return rhs
            .child_by_field_name("function")
            .and_then(|f| f.utf8_text(source.as_bytes()).ok())
            == Some("require");
    }
    false
}

fn is_rhs_require_first_party(rhs: Node, source: &str) -> bool {
    if rhs.kind() != "call_expression" {
        return false;
    }
    let args = match rhs.child_by_field_name("arguments") {
        Some(a) => a,
        None => return false,
    };
    let first_arg = match args.named_child(0) {
        Some(a) if a.kind() == "string" => a,
        _ => return false,
    };
    let text = first_arg.utf8_text(source.as_bytes()).unwrap_or("");
    let unquoted = text.trim_matches(|c: char| c == '\'' || c == '"');
    unquoted.starts_with("./") || unquoted.starts_with("../")
}

fn is_require_call(declarator: Option<Node>, source: &str) -> bool {
    require_argument(declarator, source).is_some()
}

fn require_argument<'a>(declarator: Option<Node>, source: &'a str) -> Option<&'a str> {
    let value = declarator.and_then(|d| d.child_by_field_name("value"));
    let call = match value {
        Some(v) if v.kind() == "call_expression" => Some(v),
        Some(v) if v.kind() == "member_expression" => {
            let obj = v.child_by_field_name("object")?;
            if obj.kind() == "call_expression" {
                Some(obj)
            } else {
                None
            }
        }
        _ => None,
    };
    let call = call?;
    let func = call.child_by_field_name("function")?;
    if func.utf8_text(source.as_bytes()).ok() != Some("require") {
        return None;
    }
    let args = call.child_by_field_name("arguments")?;
    let first_arg = args.named_child(0)?;
    if first_arg.kind() == "string" {
        let text = first_arg.utf8_text(source.as_bytes()).ok()?;
        Some(text.trim_matches(|c| c == '\'' || c == '"'))
    } else {
        Some("")
    }
}

fn is_require_first_party(declarator: Option<Node>, source: &str) -> bool {
    match require_argument(declarator, source) {
        Some(arg) => arg.starts_with("./") || arg.starts_with("../"),
        None => false,
    }
}

fn java_field_is_static_final(node: Node, source: &str) -> bool {
    let modifiers = match node
        .children(&mut node.walk())
        .find(|c| c.kind() == "modifiers")
    {
        Some(m) => m,
        None => return false,
    };
    let mut has_static = false;
    let mut has_final = false;
    let mut cursor = modifiers.walk();
    for child in modifiers.children(&mut cursor) {
        match child.utf8_text(source.as_bytes()).ok() {
            Some("static") => has_static = true,
            Some("final") => has_final = true,
            _ => {}
        }
    }
    has_static && has_final
}

/// If `node` is a Python module-level assignment that qualifies as a
/// constant, return its left-hand identifier text. A type-annotated
/// assignment always qualifies; otherwise the name must be ALL_CAPS or
/// `__dunder__`.
fn python_module_const_name<'a>(node: Node, source: &'a str) -> Option<&'a str> {
    if node.parent().map(|p| p.kind()) != Some("module") {
        return None;
    }
    let assignment = node
        .named_children(&mut node.walk())
        .find(|c| c.kind() == "assignment")?;
    let name_text = assignment
        .child_by_field_name("left")
        .and_then(|n| n.utf8_text(source.as_bytes()).ok())
        .unwrap_or("");

    let has_type_annotation = {
        let mut cursor = assignment.walk();
        assignment.children(&mut cursor).any(|c| c.kind() == "type")
    };
    if has_type_annotation {
        return Some(name_text);
    }

    let is_upper = !name_text.is_empty()
        && name_text
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b == b'_')
        && name_text.bytes().any(|b| b.is_ascii_uppercase());
    let is_dunder = name_text.starts_with("__") && name_text.ends_with("__");
    (is_upper || is_dunder).then_some(name_text)
}

fn lua_rhs_is_function(node: Node) -> bool {
    let assign = if node.kind() == "variable_declaration" {
        node.named_children(&mut node.walk())
            .find(|c| c.kind() == "assignment_statement")
    } else {
        Some(node)
    };
    let value_kind = assign
        .and_then(|a| {
            a.named_children(&mut a.walk())
                .find(|c| c.kind() == "expression_list")
        })
        .and_then(|el| el.named_child(0))
        .map(|v| v.kind());
    value_kind == Some("function_definition")
}

fn is_simple_typedef_alias(node: Node) -> bool {
    let type_child = match node.child_by_field_name("type") {
        Some(t) => t,
        None => return false,
    };
    if !matches!(
        type_child.kind(),
        "type_identifier" | "primitive_type" | "sized_type_specifier"
    ) {
        return false;
    }
    let declarator = match node.child_by_field_name("declarator") {
        Some(d) => d,
        None => return false,
    };
    matches!(declarator.kind(), "type_identifier" | "primitive_type")
}

fn is_c_header_guard(node: Node, source: &str, path: &Path) -> bool {
    if !crate::classify::is_header_file(path) {
        return false;
    }
    if node.child_by_field_name("value").is_some() {
        return false;
    }
    let stem = match path.file_stem().and_then(|s| s.to_str()) {
        Some(s) if !s.is_empty() => s.to_ascii_uppercase(),
        _ => return false,
    };
    let name = match node.child_by_field_name("name") {
        Some(n) => n
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .to_ascii_uppercase(),
        None => return false,
    };
    name.contains(&stem)
}

fn is_platform_type_define(node: Node, source: &str) -> bool {
    let name = match node.child_by_field_name("name") {
        Some(n) => n.utf8_text(source.as_bytes()).unwrap_or(""),
        None => return false,
    };
    if !name.ends_with("_TYPE") {
        return false;
    }
    let value = match node.child_by_field_name("value") {
        Some(v) => v.utf8_text(source.as_bytes()).unwrap_or(""),
        None => return false,
    };
    let value = value.trim();
    !value.is_empty()
        && value.split_whitespace().all(|token| {
            token.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
}

// ---------------------------------------------------------------------------
// Top-level state built once per file
// ---------------------------------------------------------------------------

/// Collect the names of top-level `mod foo { ... }` / `mod foo;` declarations
/// at the crate root (Rust only). Used to flag `pub use foo::...` as a
/// re-export when `foo` is a local submodule.
fn collect_top_level_mod_names<'t>(root: Node<'t>, source: &'t str, lang: Lang) -> Vec<&'t str> {
    if lang != Lang::Rust {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "mod_item"
            && let Some(name) = child.child_by_field_name("name")
            && let Ok(text) = name.utf8_text(source.as_bytes())
        {
            out.push(text);
        }
    }
    out
}

fn collect_js_export_names<'t>(root: Node<'t>, source: &'t str, lang: Lang) -> HashSet<&'t str> {
    if lang != Lang::JsTs {
        return HashSet::new();
    }
    let mut out = HashSet::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() != "export_statement" {
            continue;
        }
        let mut inner = child.walk();
        let has_source = child.children(&mut inner).any(|c| c.kind() == "string");
        if has_source {
            continue;
        }
        let mut inner = child.walk();
        for grandchild in child.children(&mut inner) {
            if grandchild.kind() != "export_clause" {
                continue;
            }
            let mut clause_cursor = grandchild.walk();
            for spec in grandchild.children(&mut clause_cursor) {
                if spec.kind() == "export_specifier"
                    && let Some(name_node) = spec.child_by_field_name("name")
                    && let Ok(text) = name_node.utf8_text(source.as_bytes())
                {
                    out.insert(text);
                }
            }
        }
    }
    out
}

/// Strip `pub` (including scoped forms like `pub(crate)` and
/// `pub(in foo::bar)`), `use`, and trailing `;` from a Rust `use`
/// declaration's source text, yielding just the module path
/// (e.g. `crate::foo::Bar`).
fn rust_use_path(text: &str) -> &str {
    let trimmed = text.trim();
    let rest = trimmed.strip_prefix("pub").unwrap_or(trimmed);
    let rest = if let Some(after_paren) = rest
        .strip_prefix('(')
        .and_then(|inner| inner.find(')').map(|end| &inner[end + 1..]))
    {
        after_paren
    } else {
        rest
    };
    let rest = rest.trim_start();
    let rest = rest.strip_prefix("use").unwrap_or(rest).trim_start();
    rest.trim_end_matches(';').trim()
}

/// Detect Python `from X import Y as Y` — the PEP 484 explicit re-export convention.
fn is_python_reexport(node: Node, source: &str) -> bool {
    let mut cursor = node.walk();
    let mut has_aliased = false;
    for child in node.children(&mut cursor) {
        if child.kind() == "aliased_import" {
            let name = child
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok());
            let alias = child
                .child_by_field_name("alias")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok());
            match (name, alias) {
                (Some(n), Some(a)) if n == a => has_aliased = true,
                (Some(_), Some(_)) => return false,
                _ => {}
            }
        }
    }
    has_aliased
}

fn is_rust_reexport(node: Node, source: &str, mod_names: &[&str]) -> bool {
    if !visibility::import_visibility(node, source, Lang::Rust) {
        return false;
    }
    let path = rust_use_path(node.utf8_text(source.as_bytes()).unwrap_or(""));
    if path.starts_with("crate::") || path.starts_with("self::") || path.starts_with("super::") {
        return true;
    }
    if let Some(first_segment) = path.split("::").next() {
        return mod_names.contains(&first_segment);
    }
    false
}

fn is_first_party_import(node: Node, source: &str, lang: Lang) -> bool {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("");
    match lang {
        Lang::Rust => {
            let path = rust_use_path(text);
            path.starts_with("crate::") || path.starts_with("self::") || path.starts_with("super::")
        }
        Lang::JsTs => {
            if let Some(from_pos) = text.rfind(" from ") {
                let after = text[from_pos + 6..].trim();
                after.starts_with("'./")
                    || after.starts_with("\"./")
                    || after.starts_with("'../")
                    || after.starts_with("\"../")
            } else {
                let rest = text.trim().strip_prefix("import").unwrap_or("").trim();
                rest.starts_with("'./")
                    || rest.starts_with("\"./")
                    || rest.starts_with("'../")
                    || rest.starts_with("\"../")
            }
        }
        Lang::Python => text.trim().starts_with("from ."),
        Lang::C => text
            .trim()
            .strip_prefix("#include")
            .unwrap_or("")
            .trim_start()
            .starts_with('"'),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Range construction
// ---------------------------------------------------------------------------

fn compute_end_line(node: Node) -> usize {
    let end = node.end_position();
    if end.column == 0 && end.row > node.start_position().row {
        end.row
    } else {
        end.row + 1
    }
}

// ---------------------------------------------------------------------------
// Post-extraction filters
// ---------------------------------------------------------------------------

/// Cross-kind nesting filter: drop any item whose node byte range is fully
/// contained within another item's node byte range (D4).
fn filter_nested_items(items: &mut Vec<(TsGroupKey, TsItem<'_>)>) {
    if items.len() <= 1 {
        return;
    }

    items.sort_by(|a, b| {
        a.1.node
            .start_byte()
            .cmp(&b.1.node.start_byte())
            .then(b.1.node.end_byte().cmp(&a.1.node.end_byte()))
    });

    let mut keep = vec![true; items.len()];
    let mut stack: Vec<(usize, usize)> = Vec::new();

    for (i, (_, item)) in items.iter().enumerate() {
        let start = item.node.start_byte();
        let end = item.node.end_byte();

        while let Some(&(_, parent_end)) = stack.last() {
            if parent_end <= start {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&(parent_start, parent_end)) = stack.last()
            && start >= parent_start
            && end <= parent_end
        {
            keep[i] = false;
            continue;
        }

        stack.push((start, end));
    }

    let mut write = 0;
    for (read, &kept) in keep.iter().enumerate() {
        if kept {
            if write != read {
                items.swap(write, read);
            }
            write += 1;
        }
    }
    items.truncate(write);

    items.sort_by_key(|(_, item)| item.start_line());
}

/// Drop items whose start line falls inside a previous item's range.
/// Handles e.g. Lua's `local x = {}; x.foo = bar` (two independent items on one line).
fn dedup_line_overlaps(items: &mut Vec<(TsGroupKey, TsItem<'_>)>) {
    if items.len() <= 1 {
        return;
    }
    items.sort_by_key(|(_, i)| (i.start_line(), i.node.start_byte()));
    let mut max_end: usize = 0;
    let mut first = true;
    items.retain(|(_, item)| {
        if first || item.start_line() >= max_end {
            max_end = item.end_line;
            first = false;
            true
        } else {
            max_end = max_end.max(item.end_line);
            false
        }
    });
}

/// Stretch Heading items whose grammar only captures the heading line forward
/// to the next item's start (or EOF).
fn extend_section_ranges(items: &mut [(TsGroupKey, TsItem<'_>)], source: &str) {
    let is_heading = |k: &TsGroupKey| matches!(k, TsGroupKey::Heading { .. });
    if !items.iter().any(|(k, _)| is_heading(k)) {
        return;
    }
    let total_lines = source.lines().count();
    let heading_indices: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, (k, _))| is_heading(k))
        .map(|(idx, _)| idx)
        .collect();

    for &idx in &heading_indices {
        let next_start = items
            .get(idx + 1)
            .map(|(_, item)| item.start_line())
            .unwrap_or(total_lines);
        items[idx].1.end_line = next_start;
    }
}

/// Drop duplicate overloads (C decl+defn, TS overload signatures) and
/// duplicate type declarations (Rust cfg'd dupes, etc.).
fn dedup_overloads(items: &mut Vec<(TsGroupKey, TsItem<'_>)>, lang: Lang, source: &str) {
    if !matches!(lang, Lang::Rust | Lang::C | Lang::JsTs) {
        return;
    }
    let mut seen: HashSet<(&str, std::mem::Discriminant<TsGroupKey>)> = HashSet::new();
    let mut to_remove = Vec::new();
    let mut prev: Option<(&str, std::mem::Discriminant<TsGroupKey>)> = None;

    for (i, (key, item)) in items.iter().enumerate() {
        let ident = item_identifier(item.node, source);
        let tag = discriminant(key);

        if let Some((pname, ptag)) = prev
            && pname == ident
            && ptag == tag
        {
            to_remove.push(i - 1);
            continue;
        }
        if matches!(
            key,
            TsGroupKey::StructName { .. }
                | TsGroupKey::EnumName { .. }
                | TsGroupKey::TypeAliasName { .. }
                | TsGroupKey::MacroName { .. }
        ) && !seen.insert((ident, tag))
        {
            to_remove.push(i);
            continue;
        }
        prev = Some((ident, tag));
    }

    to_remove.sort_unstable();
    to_remove.dedup();
    for idx in to_remove.into_iter().rev() {
        items.remove(idx);
    }
}

fn item_identifier<'a>(node: Node, source: &'a str) -> &'a str {
    if let Some(name_node) = node.child_by_field_name("name")
        && let Ok(text) = name_node.utf8_text(source.as_bytes())
    {
        return text.trim();
    }
    if let Some(decl) = node.child_by_field_name("declarator") {
        // For C/C++ function declarations, the declarator tree may be
        // pointer_declarator → function_declarator → identifier, with
        // type_identifiers hiding in the parameter_list. Extract the
        // name from function_declarator's own declarator field to avoid
        // picking up a parameter type as the function name.
        let func_name = ast::find_descendant_of_kind(decl, "function_declarator")
            .and_then(|fd| fd.child_by_field_name("declarator"));
        let found = func_name
            .or_else(|| ast::find_descendant_of_kind(decl, "type_identifier"))
            .or_else(|| ast::find_descendant_of_kind(decl, "identifier"));
        if let Some(n) = found
            && let Ok(text) = n.utf8_text(source.as_bytes())
        {
            return text.trim();
        }
    }
    if node.kind() == "lexical_declaration" {
        let mut cursor = node.walk();
        if let Some(vd) = node
            .children(&mut cursor)
            .find(|c| c.kind() == "variable_declarator")
            && let Some(name_node) = vd.child_by_field_name("name")
            && let Ok(text) = name_node.utf8_text(source.as_bytes())
        {
            return text.trim();
        }
    }
    if node.kind() == "impl_item"
        && let Ok(text) = node.utf8_text(source.as_bytes())
    {
        return text.lines().next().unwrap_or("").trim();
    }
    node.utf8_text(source.as_bytes())
        .map(|s| s.lines().next().unwrap_or("").trim())
        .unwrap_or("")
}

fn is_boilerplate_trait_impl(trait_node: Node, source: &str) -> bool {
    let name = trait_type_name(trait_node, source);
    matches!(
        name,
        // Marker traits (no methods)
        "Send" | "Sync" | "Unpin" | "UnwindSafe" | "RefUnwindSafe" | "Sized"
        // Derive-like traits (mechanical implementations)
        | "Copy" | "Clone" | "Debug" | "Display"
        | "Default" | "Drop"
        // Comparison traits (usually derived)
        | "PartialEq" | "Eq" | "Hash" | "PartialOrd" | "Ord"
        // Sealed trait pattern
        | "Sealed"
    )
}

fn trait_type_name<'a>(node: Node, source: &'a str) -> &'a str {
    if node.kind() == "type_identifier" {
        return node.utf8_text(source.as_bytes()).unwrap_or("");
    }
    if (node.kind() == "scoped_type_identifier" || node.kind() == "generic_type")
        && let Some(name_node) = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("type"))
    {
        return trait_type_name(name_node, source);
    }
    node.utf8_text(source.as_bytes())
        .unwrap_or("")
        .split('<')
        .next()
        .unwrap_or("")
        .rsplit("::")
        .next()
        .unwrap_or("")
        .trim()
}

#[cfg(test)]
mod rust_use_path_tests {
    use super::rust_use_path;

    #[test]
    fn plain_use() {
        assert_eq!(rust_use_path("use crate::foo::Bar;"), "crate::foo::Bar");
    }

    #[test]
    fn pub_use() {
        assert_eq!(rust_use_path("pub use crate::foo::Bar;"), "crate::foo::Bar");
    }

    #[test]
    fn pub_crate_use() {
        assert_eq!(
            rust_use_path("pub(crate) use crate::foo::Bar;"),
            "crate::foo::Bar"
        );
    }

    #[test]
    fn pub_super_use() {
        assert_eq!(
            rust_use_path("pub(super) use self::impls::Thing;"),
            "self::impls::Thing"
        );
    }

    #[test]
    fn pub_in_path_use() {
        assert_eq!(
            rust_use_path("pub(in crate::api) use self::impls::*;"),
            "self::impls::*"
        );
    }

    #[test]
    fn use_with_braces() {
        assert_eq!(
            rust_use_path("pub(crate) use crate::{a, b};"),
            "crate::{a, b}"
        );
    }
}
