//! Interface-family kinds.

use super::doc::InterfaceDocFirst;
use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct InterfaceName {
    pub documented: bool,
    pub public: bool,
}

impl TsGroupKindMethods for InterfaceName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_type_children(
            &mut out,
            parent,
            TsGroupKey::InterfaceDocFirst(InterfaceDocFirst),
            None,
            self.documented,
        );
        super::spawn_method_children(&mut out, parent, 1.0);
        out
    }
}

const INTERFACE_TS_QUERY: &str =
    "(interface_declaration name: (type_identifier) @name) @symbol";
const INTERFACE_GO_QUERY: &str =
    "(type_spec name: (type_identifier) @name type: (interface_type)) @symbol";
const INTERFACE_JAVA_QUERY: &str = "\
(interface_declaration name: (identifier) @name) @symbol
(annotation_type_declaration name: (identifier) @name) @symbol
";

impl TsGroupKindParse for InterfaceName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::TypeScript | Lang::Tsx => KindParseStrategy::Query(INTERFACE_TS_QUERY),
            Lang::Go => KindParseStrategy::Query(INTERFACE_GO_QUERY),
            Lang::Java => KindParseStrategy::Query(INTERFACE_JAVA_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::per_match_groups(matches, ctx, |m, ctx| {
            Some(TsGroupKey::InterfaceName(InterfaceName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
