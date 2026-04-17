//! Class-family kinds.

use super::doc::ClassDocFirst;
use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;
use crate::Lang;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClassName {
    pub documented: bool,
    pub public: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClassBody;

impl TsGroupKindMethods for ClassName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        let lang = parent.items.first().and_then(|i| Lang::from_path(i.path));
        // Java: spawn_method_children extracts methods but not fields, so
        // without ClassBody the data shape of a class is never shown.
        let body_key = if matches!(lang, Some(Lang::Python | Lang::Java)) {
            Some(TsGroupKey::ClassBody(ClassBody))
        } else {
            None
        };
        super::spawn_type_children(
            &mut out,
            parent,
            TsGroupKey::ClassDocFirst(ClassDocFirst),
            body_key,
            self.documented,
        );
        super::spawn_method_children(&mut out, parent, 1.0);
        out
    }
}

impl TsGroupKindMethods for ClassBody {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_body_lines(item, true, true)
    }
}

const CLASS_TS_QUERY: &str = "\
(class_declaration name: (type_identifier) @name) @symbol
(abstract_class_declaration name: (type_identifier) @name) @symbol
";
const CLASS_JAVA_QUERY: &str = "(class_declaration name: (identifier) @name) @symbol";
const CLASS_PYTHON_QUERY: &str = "(class_definition name: (identifier) @name) @symbol";
const CLASS_CPP_QUERY: &str = "\
(class_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol
";

impl TsGroupKindParse for ClassName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::TypeScript | Lang::Tsx => KindParseStrategy::Query(CLASS_TS_QUERY),
            Lang::Java => KindParseStrategy::Query(CLASS_JAVA_QUERY),
            Lang::Python => KindParseStrategy::Query(CLASS_PYTHON_QUERY),
            Lang::Cpp => KindParseStrategy::Query(CLASS_CPP_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::per_match_groups(matches, ctx, |m, ctx| {
            crate::parse::check_c_aggregate_body(m.symbol, ctx.lang)?;
            Some(TsGroupKey::ClassName(ClassName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
