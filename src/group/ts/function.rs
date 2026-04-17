//! Function-family kinds.

use super::const_;
use super::doc::FunctionDocFirst;
use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionName {
    pub documented: bool,
    pub public: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionSig;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionBody;

impl TsGroupKindMethods for FunctionName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(&mut out, parent, TsGroupKey::FunctionSig(FunctionSig));
        if self.documented {
            super::spawn_simple_child(
                &mut out,
                parent,
                TsGroupKey::FunctionDocFirst(FunctionDocFirst),
            );
        }
        out
    }
}

impl TsGroupKindMethods for FunctionSig {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_function_sig_lines(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(&mut out, parent, TsGroupKey::FunctionBody(FunctionBody));
        out
    }
}

impl TsGroupKindMethods for FunctionBody {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_body_lines(item, true, false)
    }
}

const FUNCTION_RUST_QUERY: &str = "\
(function_item name: (identifier) @name) @symbol
(function_signature_item name: (identifier) @name) @symbol
";
const FUNCTION_TS_QUERY: &str = "\
(function_declaration name: (identifier) @name) @symbol
(method_definition name: (property_identifier) @name) @symbol
(method_definition name: (private_property_identifier) @name) @symbol
(method_signature name: (property_identifier) @name) @symbol
(abstract_method_signature name: (property_identifier) @name) @symbol
(lexical_declaration (variable_declarator name: (identifier) @name)) @symbol
(lexical_declaration (variable_declarator name: (object_pattern))) @symbol
(public_field_definition name: (property_identifier) @name) @symbol
";
const FUNCTION_GO_QUERY: &str = "\
(function_declaration name: (identifier) @name) @symbol
(method_declaration name: (field_identifier) @name) @symbol
";
const FUNCTION_C_QUERY: &str = "\
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol
(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol
(declaration
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol
(declaration
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol
";
const FUNCTION_CPP_QUERY: &str = "\
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol
(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol
(function_definition
  declarator: (function_declarator
    declarator: (qualified_identifier) @name)) @symbol
(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (qualified_identifier) @name))) @symbol
(declaration
  declarator: (function_declarator
    declarator: (identifier) @name)) @symbol
(declaration
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @symbol
";
const FUNCTION_JAVA_QUERY: &str = "\
(method_declaration name: (identifier) @name) @symbol
(constructor_declaration name: (identifier) @name) @symbol
(annotation_type_element_declaration name: (identifier) @name) @symbol
";
const FUNCTION_PYTHON_QUERY: &str = "\
(function_definition name: (identifier) @name) @symbol
";
const FUNCTION_LUA_QUERY: &str = "\
(function_declaration name: (identifier) @name) @symbol
(function_declaration name: (dot_index_expression) @name) @symbol
(function_declaration name: (method_index_expression) @name) @symbol
(variable_declaration
  (assignment_statement
    (variable_list
      name: (identifier) @name))) @symbol
(assignment_statement
  (variable_list
    name: (dot_index_expression) @name)
  (expression_list
    value: (function_definition))) @symbol
(assignment_statement
  (variable_list
    name: (dot_index_expression) @name)
  (expression_list
    value: (_))) @symbol
";

impl TsGroupKindParse for FunctionName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(FUNCTION_RUST_QUERY),
            Lang::TypeScript | Lang::Tsx => KindParseStrategy::Query(FUNCTION_TS_QUERY),
            Lang::Go => KindParseStrategy::Query(FUNCTION_GO_QUERY),
            Lang::C => KindParseStrategy::Query(FUNCTION_C_QUERY),
            Lang::Cpp => KindParseStrategy::Query(FUNCTION_CPP_QUERY),
            Lang::Java => KindParseStrategy::Query(FUNCTION_JAVA_QUERY),
            Lang::Python => KindParseStrategy::Query(FUNCTION_PYTHON_QUERY),
            Lang::Lua => KindParseStrategy::Query(FUNCTION_LUA_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        let mut groups = super::per_match_groups(matches, ctx, |m, ctx| {
            let kind = m.symbol.kind();
            if matches!(ctx.lang, Lang::TypeScript | Lang::Tsx) {
                match kind {
                    "lexical_declaration" => {
                        return crate::parse::classify_js_lexical(
                            m.symbol,
                            ctx.source,
                            ctx.lang,
                            m.is_documented(ctx),
                            &ctx.export_names,
                        );
                    }
                    "public_field_definition" => {
                        let value_kind = m.symbol.child_by_field_name("value").map(|v| v.kind());
                        if !matches!(
                            value_kind,
                            Some("arrow_function" | "function_expression" | "generator_function")
                        ) {
                            return None;
                        }
                    }
                    _ => {}
                }
            }
            if ctx.lang == Lang::Lua
                && matches!(kind, "variable_declaration" | "assignment_statement")
                && !crate::parse::lua_rhs_is_function(m.symbol)
            {
                return Some(TsGroupKey::ConstName(const_::ConstName {
                    documented: m.is_documented(ctx),
                    public: m.is_public(ctx),
                }));
            }
            Some(TsGroupKey::FunctionName(FunctionName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        });
        super::dedup_adjacent_overloads(&mut groups, ctx);
        groups
    }
}
