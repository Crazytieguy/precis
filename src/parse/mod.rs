//! Tree-sitter query execution and classification into (TsGroupKey, TsItem).

pub(crate) mod ast;
pub mod file_ctx;
pub(crate) mod module_doc;
pub(crate) mod visibility;

use std::collections::HashSet;
use std::path::Path;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, QueryCursor};

use crate::Lang;
use crate::group::TsGroupKey;
use crate::group::ts::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindParse,
};
use crate::group::ts::{
    class, const_, data_section, enum_, function, heading, impl_block, import, interface, macro_,
    module, struct_, trait_, type_alias,
};
use crate::parse::file_ctx::FileCtx;
use crate::store::LanguageConfig;

/// Extract top-level items from a parsed source file, returning
/// pre-calibration `ParseTsGroup` values. Heading-style kinds may embed
/// nested `dependent_siblings` that must survive all downstream passes.
pub fn extract_items<'t>(
    display_path: &'t Path,
    source: &'t str,
    tree: &'t tree_sitter::Tree,
    config: &'t LanguageConfig,
) -> Vec<ParseTsGroup<'t>> {
    let lang = config.lang;
    let root = tree.root_node();

    // `lines` is only consulted by Heading and DataSection arms.
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

    let ctx = FileCtx {
        config,
        lang,
        display_path,
        source,
        root,
        lines,
        mod_names,
        export_names,
    };

    let mut groups = dispatch_kinds(tree, &ctx);
    filter_nested_items(&mut groups);
    groups
}

// ---------------------------------------------------------------------------
// Dispatch: per-kind from_parse loop
// ---------------------------------------------------------------------------

use crate::store::CombinedKindQuery;

/// Hand-written list of every Query-strategy kind. The dispatcher iterates
/// this list in `build_combined_query` (to assign pattern ranges) and in the
/// `dispatch_kinds` invoke loop (to hand each bucket to its kind). Each entry
/// pairs the kind with an explicit 0-based ordinal used as the bucket index.
macro_rules! query_kinds {
    ($macro:ident) => {
        $macro!(0, impl_block::ImplBlock);
        $macro!(1, heading::Heading);
        $macro!(2, trait_::TraitName);
        $macro!(3, enum_::EnumName);
        $macro!(4, class::ClassName);
        $macro!(5, macro_::MacroName);
        $macro!(6, struct_::StructName);
        $macro!(7, interface::InterfaceName);
        $macro!(8, type_alias::TypeAliasName);
        $macro!(9, function::FunctionName);
        $macro!(10, const_::ConstName);
        $macro!(11, import::Import);
        $macro!(12, data_section::DataSection);
    };
}

const NUM_QUERY_KINDS: usize = 13;

// Compile-time check: every ordinal in `query_kinds!` must fit within
// `NUM_QUERY_KINDS`. Adding a new kind without bumping `NUM_QUERY_KINDS`
// fails the build here.
macro_rules! check_kind_ord {
    ($ord:literal, $kind:path) => {
        const _: () = assert!($ord < NUM_QUERY_KINDS);
    };
}
query_kinds!(check_kind_ord);

/// Run every migrated kind's `from_parse` for this file. SourceOnly kinds run
/// individually; Query kinds share one tree walk via the combined query.
fn dispatch_kinds<'s>(tree: &'s tree_sitter::Tree, ctx: &FileCtx<'s>) -> Vec<ParseTsGroup<'s>> {
    let mut out: Vec<ParseTsGroup<'s>> = Vec::new();

    // SourceOnly kinds first so their output is in declaration order.
    run_source_only::<module::ModuleDocFirst>(ctx, &mut out);

    // All Query kinds share one combined query over the tree.
    let Some(combined) = ctx
        .config
        .combined_kind_query(|ts_lang| build_combined_query(ts_lang, ctx.lang))
    else {
        return out;
    };
    let symbol_idx = match combined.query.capture_index_for_name("symbol") {
        Some(i) => i,
        None => return out,
    };

    let mut buckets: [Vec<OwnedQueryMatch<'s>>; NUM_QUERY_KINDS] = Default::default();
    let mut cursor = QueryCursor::new();
    let mut stream = cursor.matches(&combined.query, tree.root_node(), ctx.source.as_bytes());
    while let Some(m) = stream.next() {
        let ord = combined.pattern_to_kind[m.pattern_index] as usize;
        let symbol_node = match m.captures.iter().find(|c| c.index == symbol_idx) {
            Some(c) => c.node,
            None => continue,
        };
        if !accept_top_level_symbol(symbol_node, ctx) {
            continue;
        }
        let range_node = compute_range_node(symbol_node, ctx.lang);
        buckets[ord].push(OwnedQueryMatch {
            symbol: symbol_node,
            range_node,
        });
    }

    macro_rules! invoke {
        ($ord:literal, $kind:path) => {
            if matches!(<$kind>::parse_strategy(ctx.lang), KindParseStrategy::Query(_)) {
                let matches = std::mem::take(&mut buckets[$ord]);
                out.extend(<$kind>::from_parse(&matches, ctx));
            }
        };
    }
    query_kinds!(invoke);

    out
}

fn run_source_only<'s, K: TsGroupKindParse>(
    ctx: &FileCtx<'s>,
    out: &mut Vec<ParseTsGroup<'s>>,
) {
    if matches!(K::parse_strategy(ctx.lang), KindParseStrategy::SourceOnly) {
        out.extend(K::from_parse(&[], ctx));
    }
}

/// Compile one tree-sitter `Query` spanning every Query-strategy kind for
/// `lang` and record each pattern's owning kind ordinal in `pattern_to_kind`.
/// Pattern counts are inferred from each kind query's `@symbol` occurrences,
/// since every pattern has exactly one `@symbol` capture by convention.
fn build_combined_query(
    ts_language: &tree_sitter::Language,
    lang: Lang,
) -> Option<CombinedKindQuery> {
    let mut combined_src = String::new();
    let mut pattern_to_kind: Vec<u8> = Vec::new();

    macro_rules! push {
        ($ord:literal, $kind:path) => {
            if let KindParseStrategy::Query(src) = <$kind>::parse_strategy(lang) {
                let count = src.matches("@symbol").count();
                for _ in 0..count {
                    pattern_to_kind.push($ord);
                }
                combined_src.push_str(src);
                combined_src.push('\n');
            }
        };
    }
    query_kinds!(push);

    if pattern_to_kind.is_empty() {
        return None;
    }
    let query = tree_sitter::Query::new(ts_language, &combined_src).ok()?;
    debug_assert_eq!(query.pattern_count(), pattern_to_kind.len());
    Some(CombinedKindQuery {
        query,
        pattern_to_kind,
    })
}

fn accept_top_level_symbol(symbol_node: tree_sitter::Node, ctx: &FileCtx<'_>) -> bool {
    if ast::is_inside_scope_boundary(symbol_node) {
        return false;
    }
    if ctx.lang == Lang::Rust && ast::is_rust_test_code(symbol_node, ctx.source) {
        return false;
    }
    if ctx.lang == Lang::Rust && ast::is_inside_rust_anon_const(symbol_node, ctx.source) {
        return false;
    }
    if matches!(ctx.lang, Lang::Go | Lang::Rust) {
        let ident = symbol_node
            .child_by_field_name("name")
            .and_then(|n| n.utf8_text(ctx.source.as_bytes()).ok())
            .map(str::trim);
        if ident == Some("_") {
            return false;
        }
    }
    true
}

/// Language visibility rules first, then a TS/TSX fallback against the file's
/// `export { ... }` names.
pub(crate) fn symbol_is_public(node: tree_sitter::Node, ctx: &FileCtx<'_>) -> bool {
    if visibility::symbol_visibility(node, ctx.source, ctx.lang) {
        return true;
    }
    if matches!(ctx.lang, Lang::TypeScript | Lang::Tsx)
        && !ctx.export_names.is_empty()
        && let Some(name_node) = node.child_by_field_name("name")
        && let Ok(name) = name_node.utf8_text(ctx.source.as_bytes())
    {
        return ctx.export_names.contains(name);
    }
    false
}

/// C++ `template_declaration` wraps the inner item; classify on the inner
/// node but use the outer for range + doc detection so the template
/// parameters are captured in the rendered range. Other languages just use
/// the symbol node itself.
fn compute_range_node(symbol_node: tree_sitter::Node<'_>, lang: Lang) -> tree_sitter::Node<'_> {
    if lang == Lang::Cpp
        && symbol_node
            .parent()
            .is_some_and(|p| p.kind() == "template_declaration")
    {
        symbol_node.parent().unwrap()
    } else {
        symbol_node
    }
}

pub(crate) fn classify_js_lexical(
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
        return Some(TsGroupKey::Import(import::Import {
            first_party: is_require_first_party(declarator, source),
            reexport: false,
        }));
    }

    let keyword = node.child(0).map(|c| c.kind());
    if keyword != Some("const") {
        return None;
    }

    let base_public = visibility::symbol_visibility(node, source, lang);
    let public = base_public
        || (matches!(lang, Lang::TypeScript | Lang::Tsx)
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
        Some(FunctionName(function::FunctionName { documented, public }))
    } else {
        Some(ConstName(const_::ConstName { documented, public }))
    }
}

/// Classify a JS/TS `export_statement`. Returns `None` for declaration
/// exports (the inner declaration is captured separately); returns an Import
/// group for re-exports (`export * from`, `export { } from`, `export { }`).
pub(crate) fn classify_js_export(node: Node, source: &str) -> Option<TsGroupKey> {
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
        is_first_party_import(node, source, Lang::TypeScript)
    } else {
        true
    };
    Some(TsGroupKey::Import(import::Import {
        first_party,
        reexport: true,
    }))
}

/// Classify a CJS `exports.X = …` / `module.exports = …` as a ConstName.
/// Returns `None` for `module.exports = require(…)` (handled by
/// `classify_js_cjs_reexport`) or anything that isn't a CJS export assignment.
pub(crate) fn classify_js_cjs_const(node: Node, source: &str, documented: bool) -> Option<TsGroupKey> {
    let (obj_text, rhs) = cjs_export_target(node, source)?;
    match obj_text {
        "exports" => Some(TsGroupKey::ConstName(const_::ConstName {
            documented,
            public: true,
        })),
        "module" if !is_rhs_require(rhs, source) => Some(TsGroupKey::ConstName(const_::ConstName {
            documented,
            public: true,
        })),
        _ => None,
    }
}

/// Classify a CJS `module.exports = require('X')` as a re-export Import.
/// Returns `None` for anything that isn't a module.exports = require(...)
/// re-export.
pub(crate) fn classify_js_cjs_reexport(node: Node, source: &str) -> Option<TsGroupKey> {
    let (obj_text, rhs) = cjs_export_target(node, source)?;
    if obj_text != "module" || !is_rhs_require(rhs, source) {
        return None;
    }
    Some(TsGroupKey::Import(import::Import {
        first_party: is_rhs_require_first_party(rhs, source),
        reexport: true,
    }))
}

/// Walk a CJS export assignment (`exports.X = …`, `module.exports = …`),
/// returning `(left-hand object name, right-hand side node)`.
fn cjs_export_target<'a, 's: 'a>(
    node: Node<'a>,
    source: &'s str,
) -> Option<(&'s str, Node<'a>)> {
    let assign = node
        .named_children(&mut node.walk())
        .find(|c| c.kind() == "assignment_expression")?;
    let left = assign.child_by_field_name("left")?;
    if left.kind() != "member_expression" {
        return None;
    }
    let obj = left.child_by_field_name("object")?;
    let obj_text = obj.utf8_text(source.as_bytes()).ok()?;
    if obj_text == "module" {
        let prop = left.child_by_field_name("property")?;
        if prop.utf8_text(source.as_bytes()).ok()? != "exports" {
            return None;
        }
    }
    let rhs = assign.child_by_field_name("right")?;
    Some((obj_text, rhs))
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

pub(crate) fn java_field_is_static_final(node: Node, source: &str) -> bool {
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
pub(crate) fn python_module_const_name<'a>(node: Node, source: &'a str) -> Option<&'a str> {
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

pub(crate) fn lua_rhs_is_function(node: Node) -> bool {
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

/// C/C++ struct/union/enum/class filter: aggregate must have a body (so
/// anonymous and forward declarations are excluded) and must not be wrapped
/// in a `typedef` (those are surfaced via typedef handling, not via the
/// aggregate's own name).
pub(crate) fn check_c_aggregate_body(node: Node, lang: Lang) -> Option<()> {
    if !matches!(lang, Lang::C | Lang::Cpp) {
        return Some(());
    }
    node.child_by_field_name("body")?;
    if node
        .parent()
        .is_some_and(|p| p.kind() == "type_definition")
    {
        return None;
    }
    Some(())
}

pub(crate) fn is_simple_typedef_alias(node: Node) -> bool {
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

pub(crate) fn is_c_header_guard(node: Node, source: &str, path: &Path) -> bool {
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

pub(crate) fn is_platform_type_define(node: Node, source: &str) -> bool {
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
    if !matches!(lang, Lang::TypeScript | Lang::Tsx) {
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
pub(crate) fn is_python_reexport(node: Node, source: &str) -> bool {
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

pub(crate) fn is_rust_reexport(node: Node, source: &str, mod_names: &[&str]) -> bool {
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

pub(crate) fn is_first_party_import(node: Node, source: &str, lang: Lang) -> bool {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("");
    match lang {
        Lang::Rust => {
            let path = rust_use_path(text);
            path.starts_with("crate::") || path.starts_with("self::") || path.starts_with("super::")
        }
        Lang::TypeScript | Lang::Tsx => {
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
        Lang::C | Lang::Cpp => text
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

pub(crate) fn compute_end_line(node: Node) -> usize {
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

/// Cross-kind nesting filter: drop any top-level group whose single item is
/// fully contained in another group's single item byte range (D4). Relies on
/// the invariant that every `ParseTsGroup` coming out of `dispatch_kinds` has
/// exactly one `items` entry; items nested under `dependent_siblings` already
/// belong to a containing group and are left alone.
fn filter_nested_items(groups: &mut Vec<ParseTsGroup<'_>>) {
    if groups.len() <= 1 {
        return;
    }

    groups.sort_by(|a, b| {
        let na = a.items[0].node;
        let nb = b.items[0].node;
        na.start_byte()
            .cmp(&nb.start_byte())
            .then(nb.end_byte().cmp(&na.end_byte()))
    });

    let mut keep = vec![true; groups.len()];
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for (i, g) in groups.iter().enumerate() {
        let node = g.items[0].node;
        let start = node.start_byte();
        let end = node.end_byte();

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

    let mut idx = 0;
    groups.retain(|_| {
        let k = keep[idx];
        idx += 1;
        k
    });
}

pub(crate) fn item_identifier<'a>(node: Node, source: &'a str) -> &'a str {
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

pub(crate) fn is_boilerplate_trait_impl(trait_node: Node, source: &str) -> bool {
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
