//! Import-family kinds.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Import {
    pub first_party: bool,
    pub reexport: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ImportedItems {
    pub first_party: bool,
    pub reexport: bool,
}

impl TsGroupKindMethods for Import {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_import_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(
            &mut out,
            parent,
            TsGroupKey::ImportedItems(ImportedItems {
                first_party: self.first_party,
                reexport: self.reexport,
            }),
        );
        out
    }
}

impl TsGroupKindMethods for ImportedItems {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_imported_items_lines(item)
    }
}

const IMPORT_RUST_QUERY: &str = "\
(use_declaration) @symbol
(mod_item name: (identifier) @name) @symbol
";
const IMPORT_TS_QUERY: &str = "\
(import_statement) @symbol
(export_statement) @symbol
(expression_statement
  (assignment_expression
    left: (member_expression
      object: (identifier)))) @symbol
";
const IMPORT_PYTHON_QUERY: &str = "\
(import_statement) @symbol
(import_from_statement) @symbol
";
const IMPORT_GO_QUERY: &str = "(import_declaration) @symbol";
const IMPORT_JAVA_QUERY: &str = "(import_declaration) @symbol";
const IMPORT_C_QUERY: &str = "(preproc_include) @symbol";

impl TsGroupKindParse for Import {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(IMPORT_RUST_QUERY),
            Lang::TypeScript | Lang::Tsx => KindParseStrategy::Query(IMPORT_TS_QUERY),
            Lang::Python => KindParseStrategy::Query(IMPORT_PYTHON_QUERY),
            Lang::Go => KindParseStrategy::Query(IMPORT_GO_QUERY),
            Lang::Java => KindParseStrategy::Query(IMPORT_JAVA_QUERY),
            Lang::C | Lang::Cpp => KindParseStrategy::Query(IMPORT_C_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::per_match_groups(matches, ctx, |m, ctx| {
            let kind = m.symbol.kind();
            match (ctx.lang, kind) {
                (Lang::Rust, "use_declaration") => {
                    let is_crate_root =
                        ctx.display_path.file_name().is_some_and(|f| f == "lib.rs");
                    Some(TsGroupKey::Import(Import {
                        first_party: crate::parse::is_first_party_import(
                            m.symbol, ctx.source, ctx.lang,
                        ),
                        reexport: !is_crate_root
                            && crate::parse::is_rust_reexport(
                                m.symbol,
                                ctx.source,
                                &ctx.mod_names,
                            ),
                    }))
                }
                (Lang::Rust, "mod_item") => {
                    if m.symbol.child_by_field_name("body").is_some() {
                        return None;
                    }
                    Some(TsGroupKey::Import(Import {
                        first_party: true,
                        reexport: false,
                    }))
                }
                (Lang::TypeScript | Lang::Tsx, "import_statement") => {
                    Some(TsGroupKey::Import(Import {
                        first_party: crate::parse::is_first_party_import(
                            m.symbol, ctx.source, ctx.lang,
                        ),
                        reexport: false,
                    }))
                }
                (Lang::TypeScript | Lang::Tsx, "export_statement") => {
                    crate::parse::classify_js_export(m.symbol, ctx.source)
                }
                // `module.exports = require(...)` is a re-export; the
                // ConstName path (which also captures expression_statement)
                // rejects the match in that case.
                (Lang::TypeScript | Lang::Tsx, "expression_statement") => {
                    crate::parse::classify_js_cjs_reexport(m.symbol, ctx.source)
                }
                (Lang::Python, "import_statement") => Some(TsGroupKey::Import(Import {
                    first_party: crate::parse::is_first_party_import(
                        m.symbol, ctx.source, ctx.lang,
                    ),
                    reexport: false,
                })),
                (Lang::Python, "import_from_statement") => Some(TsGroupKey::Import(Import {
                    first_party: crate::parse::is_first_party_import(
                        m.symbol, ctx.source, ctx.lang,
                    ),
                    reexport: crate::parse::is_python_reexport(m.symbol, ctx.source),
                })),
                (_, "import_declaration") => Some(TsGroupKey::Import(Import {
                    first_party: crate::parse::is_first_party_import(
                        m.symbol, ctx.source, ctx.lang,
                    ),
                    reexport: false,
                })),
                (Lang::C | Lang::Cpp, "preproc_include") => Some(TsGroupKey::Import(Import {
                    first_party: crate::parse::is_first_party_import(
                        m.symbol, ctx.source, ctx.lang,
                    ),
                    reexport: false,
                })),
                _ => None,
            }
        })
    }
}
