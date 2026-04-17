//! Rust impl-block kind.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ImplBlock {
    pub is_trait_impl: bool,
    pub is_boilerplate_trait: bool,
}

const IMPL_BLOCK_RUST_QUERY: &str = "(impl_item) @symbol";

impl TsGroupKindParse for ImplBlock {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(IMPL_BLOCK_RUST_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::per_match_groups(matches, ctx, |m, ctx| {
            let trait_node = m.symbol.child_by_field_name("trait");
            let is_boilerplate = trait_node
                .is_some_and(|t| crate::parse::is_boilerplate_trait_impl(t, ctx.source));
            Some(TsGroupKey::ImplBlock(ImplBlock {
                is_trait_impl: trait_node.is_some(),
                is_boilerplate_trait: is_boilerplate,
            }))
        })
    }
}

impl TsGroupKindMethods for ImplBlock {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_impl_block_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let factor = if self.is_boilerplate_trait {
            0.15
        } else if self.is_trait_impl {
            0.5
        } else {
            1.0
        };
        let mut out = Vec::new();
        super::spawn_method_children(&mut out, parent, factor);
        out
    }
}
