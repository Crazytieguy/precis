//! JSON / TOML / YAML data section kinds.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataSection {
    pub level: u8,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataSectionBody {
    pub level: u8,
}

impl TsGroupKindMethods for DataSection {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_heading_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let body_key = TsGroupKey::DataSectionBody(DataSectionBody { level: self.level });
        // DataSection aggregates across files; a merged group can mix TOML,
        // JSON, and YAML items from the same directory. Each language needs
        // its own body-spawn shape: TOML gets one group per table (flat
        // namespace), JSON/YAML get one body group covering all items.
        let (toml_items, flat_items): (Vec<TsItem<'s>>, Vec<TsItem<'s>>) = parent
            .items
            .iter()
            .copied()
            .partition(|i| crate::Lang::from_path(i.path) == Some(crate::Lang::Toml));

        let mut out = Vec::new();
        for item in toml_items {
            out.push(Group::Ts(TsGroup {
                key: body_key,
                items: vec![item],
                inherited_modifier: parent.inherited_modifier,
                dependent_siblings: vec![],
                cached_render: None,
            }));
        }
        if !flat_items.is_empty() {
            out.push(Group::Ts(TsGroup {
                key: body_key,
                items: flat_items,
                inherited_modifier: parent.inherited_modifier,
                dependent_siblings: vec![],
                cached_render: None,
            }));
        }
        out
    }
}

impl TsGroupKindMethods for DataSectionBody {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_heading_body_lines(self.level, item)
    }
}

const TOML_DATA_QUERY: &str = "\
(table (bare_key) @name) @symbol
(table (dotted_key) @name) @symbol
(table (quoted_key) @name) @symbol
(table_array_element (bare_key) @name) @symbol
(table_array_element (dotted_key) @name) @symbol
(table_array_element (quoted_key) @name) @symbol
";

const JSON_DATA_QUERY: &str = "\
(document
  (object
    (pair
      key: (string (string_content) @name)) @symbol))
";

const YAML_DATA_QUERY: &str = "\
(stream
  (document
    (block_node
      (block_mapping
        (block_mapping_pair
          key: (flow_node
            (plain_scalar
              (string_scalar) @name))) @symbol))))
";

impl TsGroupKindParse for DataSection {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Toml => KindParseStrategy::Query(TOML_DATA_QUERY),
            Lang::Json => KindParseStrategy::Query(JSON_DATA_QUERY),
            Lang::Yaml => KindParseStrategy::Query(YAML_DATA_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        super::per_match_groups(matches, ctx, |m, ctx| {
            let level = match ctx.lang {
                Lang::Toml => {
                    let start = m.symbol.start_position().row;
                    let key_text = ctx.lines[start]
                        .trim()
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .trim();
                    key_text.chars().filter(|&c| c == '.').count() as u8 + 1
                }
                Lang::Json | Lang::Yaml => 1,
                _ => return None,
            };
            Some(TsGroupKey::DataSection(DataSection { level }))
        })
    }
}
