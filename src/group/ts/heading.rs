//! Markdown / TOML / JSON / YAML heading kinds.
//!
//! TOML/JSON/YAML still route through `Heading` here as a stage 6 holdover;
//! stage 7 splits them into a dedicated `DataSection` kind.

use super::kind::{
    KindParseStrategy, OwnedQueryMatch, ParseTsGroup, TsGroupKindMethods, TsGroupKindParse,
};
use crate::Lang;
use crate::group::{Group, TsGroup, TsGroupKey, TsItem};
use crate::parse::file_ctx::FileCtx;
use crate::render::LineEntry;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Heading {
    pub level: u8,
    pub boilerplate: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HeadingBody {
    pub level: u8,
}

impl TsGroupKindMethods for Heading {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_heading_line(item)
    }

    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> {
        let body_key = TsGroupKey::HeadingBody(HeadingBody { level: self.level });
        let (toml_items, other_items): (Vec<TsItem<'s>>, Vec<TsItem<'s>>) = parent
            .items
            .iter()
            .copied()
            .partition(|i| crate::Lang::from_path(i.path) == Some(crate::Lang::Toml));

        let mut out = Vec::new();
        if !other_items.is_empty() {
            let is_readme_h1 = self.level == 1
                && other_items.iter().any(|i| {
                    crate::classify::FileRole::from_path(i.path)
                        == crate::classify::FileRole::Readme
                });
            let body_modifier = if is_readme_h1 {
                super::README_H1_BODY_BOOST
            } else {
                1.0
            };
            out.push(Group::Ts(TsGroup {
                key: body_key,
                items: other_items,
                inherited_modifier: parent.inherited_modifier * body_modifier,
                dependent_siblings: vec![],
                cached_render: None,
            }));
        }
        for item in toml_items {
            out.push(Group::Ts(TsGroup {
                key: body_key,
                items: vec![item],
                inherited_modifier: parent.inherited_modifier,
                dependent_siblings: vec![],
                cached_render: None,
            }));
        }
        out
    }
}

impl TsGroupKindMethods for HeadingBody {
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>> {
        super::render_heading_body_lines(self.level, item)
    }
}

const MARKDOWN_HEADING_QUERY: &str = "\
(atx_heading (inline) @name) @symbol
(setext_heading (paragraph) @name) @symbol
";

const TOML_HEADING_QUERY: &str = "\
(table (bare_key) @name) @symbol
(table (dotted_key) @name) @symbol
(table (quoted_key) @name) @symbol
(table_array_element (bare_key) @name) @symbol
(table_array_element (dotted_key) @name) @symbol
(table_array_element (quoted_key) @name) @symbol
";

const JSON_HEADING_QUERY: &str = "\
(document
  (object
    (pair
      key: (string (string_content) @name)) @symbol))
";

const YAML_HEADING_QUERY: &str = "\
(stream
  (document
    (block_node
      (block_mapping
        (block_mapping_pair
          key: (flow_node
            (plain_scalar
              (string_scalar) @name))) @symbol))))
";

impl TsGroupKindParse for Heading {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Markdown => KindParseStrategy::Query(MARKDOWN_HEADING_QUERY),
            Lang::Toml => KindParseStrategy::Query(TOML_HEADING_QUERY),
            Lang::Json => KindParseStrategy::Query(JSON_HEADING_QUERY),
            Lang::Yaml => KindParseStrategy::Query(YAML_HEADING_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        let mut out = Vec::with_capacity(matches.len());
        for m in matches {
            let key = match ctx.lang {
                Lang::Markdown => {
                    let start = m.symbol.start_position().row;
                    let end = crate::parse::compute_end_line(m.symbol);
                    let level = crate::classify::detect_heading_depth(&ctx.lines, start, end);
                    let stripped = crate::classify::strip_heading_badges(ctx.lines[start]);
                    let boilerplate = crate::classify::is_boilerplate_heading(stripped);
                    TsGroupKey::Heading(Heading { level, boilerplate })
                }
                Lang::Toml => {
                    let start = m.symbol.start_position().row;
                    let key_text = ctx.lines[start]
                        .trim()
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .trim();
                    let level = key_text.chars().filter(|&c| c == '.').count() as u8 + 1;
                    TsGroupKey::Heading(Heading {
                        level,
                        boilerplate: false,
                    })
                }
                Lang::Json | Lang::Yaml => TsGroupKey::Heading(Heading {
                    level: 1,
                    boilerplate: false,
                }),
                _ => continue,
            };
            let end_line = crate::parse::compute_end_line(m.range_node);
            let item = TsItem {
                path: ctx.display_path,
                source: ctx.source,
                node: m.range_node,
                end_line,
            };
            out.push(ParseTsGroup {
                key,
                items: vec![item],
                dependent_siblings: Vec::new(),
            });
        }
        out
    }
}
