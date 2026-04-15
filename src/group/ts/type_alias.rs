//! Type-alias-family kinds.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TypeAliasName {
    pub documented: bool,
    pub public: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TypeAliasDocFirst;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TypeAliasDocRest;

impl TsGroupKindMethods for TypeAliasName {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_name_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        if self.documented {
            super::spawn_simple_child(
                &mut out,
                parent,
                TsGroupKey::TypeAliasDocFirst(TypeAliasDocFirst),
            );
        }
        out
    }
}

impl TsGroupKindMethods for TypeAliasDocFirst {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_first_lines(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let mut out = Vec::new();
        super::spawn_simple_child(
            &mut out,
            parent,
            TsGroupKey::TypeAliasDocRest(TypeAliasDocRest),
        );
        out
    }
}

impl TsGroupKindMethods for TypeAliasDocRest {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_doc_rest_lines(item)
    }
}

const TYPE_ALIAS_RUST_QUERY: &str = "(type_item name: (type_identifier) @name) @symbol";
const TYPE_ALIAS_TS_QUERY: &str =
    "(type_alias_declaration name: (type_identifier) @name) @symbol";
const TYPE_ALIAS_GO_QUERY: &str = "\
(type_spec name: (type_identifier) @name) @symbol
(type_alias name: (type_identifier) @name) @symbol
";
const TYPE_ALIAS_C_QUERY: &str = "(type_definition) @symbol";
const TYPE_ALIAS_CPP_QUERY: &str = "\
(type_definition) @symbol
(alias_declaration name: (type_identifier) @name) @symbol
";

impl TsGroupKindParse for TypeAliasName {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Rust => KindParseStrategy::Query(TYPE_ALIAS_RUST_QUERY),
            Lang::TypeScript | Lang::Tsx => KindParseStrategy::Query(TYPE_ALIAS_TS_QUERY),
            Lang::Go => KindParseStrategy::Query(TYPE_ALIAS_GO_QUERY),
            Lang::C => KindParseStrategy::Query(TYPE_ALIAS_C_QUERY),
            Lang::Cpp => KindParseStrategy::Query(TYPE_ALIAS_CPP_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::simple_named_groups(matches, ctx, |m, ctx| {
            let kind = m.symbol.kind();
            // Go type_spec is polymorphic — only the non-struct/non-interface
            // variant is a type alias here.
            if kind == "type_spec" {
                let type_child = m.symbol.child_by_field_name("type").map(|t| t.kind());
                if matches!(type_child, Some("struct_type") | Some("interface_type")) {
                    return None;
                }
            }
            // C/C++ type_definition: simple typedef aliases (e.g. `typedef int Foo;`)
            // are skipped; only complex types become alias kinds.
            if kind == "type_definition" && crate::parse::is_simple_typedef_alias(m.symbol) {
                return None;
            }
            Some(TsGroupKey::TypeAliasName(TypeAliasName {
                documented: m.is_documented(ctx),
                public: m.is_public(ctx),
            }))
        })
    }
}
