//! Const/static-family kinds.

use super::doc::ConstDocFirst;
use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConstName {
    pub documented: bool,
    pub public: bool,
}

impl TsGroupKindMethods for ConstName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        if self.documented {
            super::spawn_simple_child(&mut out, parent, TsGroupKey::ConstDocFirst(ConstDocFirst));
        }
        out
    }
}

const CONST_RUST_QUERY: &str = "\
(const_item name: (identifier) @name) @symbol
(static_item name: (identifier) @name) @symbol
";
const CONST_TS_QUERY: &str = "\
(expression_statement
  (assignment_expression
    left: (member_expression
      object: (identifier)))) @symbol
";
const CONST_GO_QUERY: &str = "\
(const_declaration) @symbol
(var_declaration) @symbol
(const_spec name: (identifier) @name) @symbol
(var_spec name: (identifier) @name) @symbol
";
// C/C++ non-function declarations (e.g. `extern int x;`) are intentionally
// not surfaced — declaration-level constants are low-value at this budget.
const CONST_PYTHON_QUERY: &str = "\
(expression_statement (assignment left: (identifier) @name)) @symbol
";
const CONST_JAVA_QUERY: &str = "\
(field_declaration) @symbol
(constant_declaration) @symbol
";

impl TsGroupKindParse for ConstName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(CONST_RUST_QUERY),
            Lang::TypeScript | Lang::Tsx => KindParseStrategy::Query(CONST_TS_QUERY),
            Lang::Go => KindParseStrategy::Query(CONST_GO_QUERY),
            Lang::Python => KindParseStrategy::Query(CONST_PYTHON_QUERY),
            Lang::Java => KindParseStrategy::Query(CONST_JAVA_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::per_match_groups(matches, ctx, |m, ctx| {
            let kind = m.symbol.kind();
            if matches!(ctx.lang, Lang::TypeScript | Lang::Tsx) && kind == "expression_statement" {
                return crate::parse::classify_js_cjs_const(
                    m.symbol,
                    ctx.source,
                    m.is_documented(ctx),
                );
            }
            if ctx.lang == Lang::Go && matches!(kind, "const_declaration" | "var_declaration") {
                let found_open = (0..m.symbol.child_count())
                    .filter_map(|i| m.symbol.child(i))
                    .any(|c| c.kind() == "(" || c.kind() == "var_spec_list");
                if !found_open {
                    return None;
                }
            }
            if ctx.lang == Lang::Python && kind == "expression_statement" {
                let name = crate::parse::python_module_const_name(m.symbol, ctx.source)?;
                let public =
                    !name.starts_with('_') || (name.starts_with("__") && name.ends_with("__"));
                return Some(TsGroupKey::ConstName(ConstName {
                    documented: m.is_documented(ctx),
                    public,
                }));
            }
            if ctx.lang == Lang::Java
                && kind == "field_declaration"
                && !crate::parse::java_field_is_static_final(m.symbol, ctx.source)
            {
                return None;
            }
            Some(TsGroupKey::ConstName(ConstName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
