//! Trait-family kinds (Rust).

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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TraitDocFirst;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TraitDocRest;

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

impl TsGroupKindMethods for TraitDocFirst {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_first_lines(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(&mut out, parent, TsGroupKey::TraitDocRest(TraitDocRest));
        out
    }
}

impl TsGroupKindMethods for TraitDocRest {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_rest_lines(item)
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
        super::simple_named_groups(matches, ctx, |m, ctx| {
            Some(TsGroupKey::TraitName(TraitName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
