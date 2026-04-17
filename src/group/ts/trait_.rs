//! Trait-family kinds (Rust).

use super::doc::TraitDocFirst;
use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TraitName {
    pub documented: bool,
    pub public: bool,
}

impl TsGroupKindMethods for TraitName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_type_children(
            &mut out,
            parent,
            TsGroupKey::TraitDocFirst(TraitDocFirst),
            None,
            self.documented,
        );
        super::spawn_method_children(&mut out, parent, 1.0);
        out
    }
}

const TRAIT_RUST_QUERY: &str = "(trait_item name: (type_identifier) @name) @symbol";

impl TsGroupKindParse for TraitName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(TRAIT_RUST_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::per_match_groups(matches, ctx, |m, ctx| {
            Some(TsGroupKey::TraitName(TraitName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
