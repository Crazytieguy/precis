//! Macro-family kinds.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::parse::visibility;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct MacroName {
    pub documented: bool,
    pub public: bool,
    pub preproc: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct MacroDocFirst;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct MacroDocRest;

impl TsGroupKindMethods for MacroName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_macro_name(self.preproc, item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        if self.documented {
            super::spawn_simple_child(&mut out, parent, TsGroupKey::MacroDocFirst(MacroDocFirst));
        }
        out
    }
}

impl TsGroupKindMethods for MacroDocFirst {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_first_lines(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(&mut out, parent, TsGroupKey::MacroDocRest(MacroDocRest));
        out
    }
}

impl TsGroupKindMethods for MacroDocRest {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_rest_lines(item)
    }
}

const MACRO_RUST_QUERY: &str = "(macro_definition name: (identifier) @name) @symbol";
const MACRO_C_QUERY: &str = "\
(preproc_def) @symbol
(preproc_function_def) @symbol
";

impl TsGroupKindParse for MacroName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(MACRO_RUST_QUERY),
            Lang::C | Lang::Cpp => KindParseStrategy::Query(MACRO_C_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::simple_named_groups(matches, ctx, |m, ctx| {
            let kind = m.symbol.kind();
            let preproc = matches!(kind, "preproc_def" | "preproc_function_def");
            if kind == "preproc_def"
                && (crate::parse::is_c_header_guard(m.symbol, ctx.source, ctx.display_path)
                    || crate::parse::is_platform_type_define(m.symbol, ctx.source))
            {
                return None;
            }
            Some(TsGroupKey::MacroName(MacroName {
                documented: m.is_documented(ctx),
                public: visibility::macro_visibility(m.symbol, ctx.source, ctx.lang),
                preproc,
            }))
        })
    }
}
