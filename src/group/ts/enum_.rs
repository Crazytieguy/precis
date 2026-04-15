//! Enum-family kinds.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct EnumName {
    pub documented: bool,
    pub public: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct EnumDocFirst;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct EnumDocRest;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct EnumBody;

impl TsGroupKindMethods for EnumName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_type_children(
            &mut out,
            parent,
            TsGroupKey::EnumDocFirst(EnumDocFirst),
            Some(TsGroupKey::EnumBody(EnumBody)),
            self.documented,
        );
        out
    }
}

impl TsGroupKindMethods for EnumDocFirst {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_first_lines(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(&mut out, parent, TsGroupKey::EnumDocRest(EnumDocRest));
        out
    }
}

impl TsGroupKindMethods for EnumDocRest {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_rest_lines(item)
    }
}

impl TsGroupKindMethods for EnumBody {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_body_lines(item, false, false)
    }
}

const ENUM_RUST_QUERY: &str = "(enum_item name: (type_identifier) @name) @symbol";
const ENUM_TS_QUERY: &str = "(enum_declaration name: (identifier) @name) @symbol";
const ENUM_JAVA_QUERY: &str = "(enum_declaration name: (identifier) @name) @symbol";
const ENUM_C_QUERY: &str = "\
(enum_specifier
  name: (type_identifier) @name
  body: (enumerator_list)) @symbol
";

impl TsGroupKindParse for EnumName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(ENUM_RUST_QUERY),
            Lang::TypeScript | Lang::Tsx => KindParseStrategy::Query(ENUM_TS_QUERY),
            Lang::Java => KindParseStrategy::Query(ENUM_JAVA_QUERY),
            Lang::C | Lang::Cpp => KindParseStrategy::Query(ENUM_C_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::simple_named_groups(matches, ctx, |m, ctx| {
            crate::parse::check_c_aggregate_body(m.symbol, ctx.lang)?;
            Some(TsGroupKey::EnumName(EnumName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
