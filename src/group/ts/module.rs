//! Module-doc kinds.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ModuleDocFirst;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ModuleDocRest;

impl TsGroupKindMethods for ModuleDocFirst {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_module_doc_first_lines(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(&mut out, parent, TsGroupKey::ModuleDocRest(ModuleDocRest));
        out
    }
}

impl TsGroupKindMethods for ModuleDocRest {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_module_doc_rest_lines(item)
    }
}

impl TsGroupKindParse for ModuleDocFirst {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust | Lang::Python | Lang::Go | Lang::Java | Lang::Markdown => {
                KindParseStrategy::SourceOnly
            }
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        _matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        let Some((node, end_row)) =
            crate::parse::module_doc::detect_module_doc(ctx.root, ctx.source, ctx.lang, ctx.display_path)
        else {
            return Vec::new();
        };
        let item = TsItem {
            path: ctx.display_path,
            source: ctx.source,
            node,
            end_line: end_row + 1,
        };
        vec![ParseTsGroup {
            key: TsGroupKey::ModuleDocFirst(ModuleDocFirst),
            items: vec![item],
            dependent_siblings: Vec::new(),
        }]
    }
}
