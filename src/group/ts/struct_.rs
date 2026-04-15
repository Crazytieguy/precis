//! Struct-family kinds.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StructName {
    pub documented: bool,
    pub public: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StructDocFirst;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StructDocRest;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StructBody;

impl TsGroupKindMethods for StructName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_type_children(
            &mut out,
            parent,
            TsGroupKey::StructDocFirst(StructDocFirst),
            Some(TsGroupKey::StructBody(StructBody)),
            self.documented,
        );
        out
    }
}

impl TsGroupKindMethods for StructDocFirst {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_first_lines(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(&mut out, parent, TsGroupKey::StructDocRest(StructDocRest));
        out
    }
}

impl TsGroupKindMethods for StructDocRest {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_rest_lines(item)
    }
}

impl TsGroupKindMethods for StructBody {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_body_lines(item, false, false)
    }
}

const STRUCT_RUST_QUERY: &str = "(struct_item name: (type_identifier) @name) @symbol";
const STRUCT_GO_QUERY: &str =
    "(type_spec name: (type_identifier) @name type: (struct_type)) @symbol";
const STRUCT_C_QUERY: &str = "\
(struct_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol
(union_specifier
  name: (type_identifier) @name
  body: (field_declaration_list)) @symbol
";
const STRUCT_JAVA_QUERY: &str = "(record_declaration name: (identifier) @name) @symbol";

impl TsGroupKindParse for StructName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(STRUCT_RUST_QUERY),
            Lang::Go => KindParseStrategy::Query(STRUCT_GO_QUERY),
            Lang::C | Lang::Cpp => KindParseStrategy::Query(STRUCT_C_QUERY),
            Lang::Java => KindParseStrategy::Query(STRUCT_JAVA_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::simple_named_groups(matches, ctx, |m, ctx| {
            crate::parse::check_c_aggregate_body(m.symbol, ctx.lang)?;
            Some(TsGroupKey::StructName(StructName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
