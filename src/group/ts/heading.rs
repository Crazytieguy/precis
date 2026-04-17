//! Markdown heading kinds.

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
        let is_readme_h1 = self.level == 1
            && parent.items.iter().any(|i| {
                crate::classify::FileRole::from_path(i.path)
                    == crate::classify::FileRole::Readme
            });
        let body_modifier = if is_readme_h1 {
            super::README_H1_BODY_BOOST
        } else {
            1.0
        };
        vec![Group::Ts(TsGroup {
            key: body_key,
            items: parent.items.clone(),
            inherited_modifier: parent.inherited_modifier * body_modifier,
            dependent_siblings: vec![],
            cached_render: None,
        })]
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

impl TsGroupKindParse for Heading {
    fn parse_strategy(lang: Lang) -> KindParseStrategy {
        match lang {
            Lang::Markdown => KindParseStrategy::Query(MARKDOWN_HEADING_QUERY),
            _ => KindParseStrategy::None,
        }
    }

    /// Build one single-item `ParseTsGroup` per heading. Each heading's
    /// `end_line` is stretched forward to the next heading's start (section
    /// ownership — atx headings natively span one line). Each heading also
    /// picks its parent as the most recent earlier heading with strictly
    /// lower level, and is attached there as a `dependent_sibling`. Top-level
    /// headings (those with no shallower earlier heading) are the return
    /// value.
    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        struct HeadingInfo<'s> {
            level: u8,
            boilerplate: bool,
            start_line: usize,
            node: tree_sitter::Node<'s>,
        }

        let mut infos: Vec<HeadingInfo<'s>> = matches
            .iter()
            .map(|m| {
                let start = m.symbol.start_position().row;
                let end = crate::parse::compute_end_line(m.symbol);
                let level = crate::classify::detect_heading_depth(&ctx.lines, start, end);
                let stripped = crate::classify::strip_heading_badges(ctx.lines[start]);
                let boilerplate = crate::classify::is_boilerplate_heading(stripped);
                HeadingInfo {
                    level,
                    boilerplate,
                    start_line: start,
                    node: m.range_node,
                }
            })
            .collect();
        infos.sort_by_key(|h| h.start_line);

        let total_lines = ctx.lines.len();
        let mut groups: Vec<Option<ParseTsGroup<'s>>> = infos
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let next_start = infos
                    .get(i + 1)
                    .map(|n| n.start_line)
                    .unwrap_or(total_lines);
                let end_line = next_start.max(crate::parse::compute_end_line(h.node));
                let item = TsItem {
                    path: ctx.display_path,
                    source: ctx.source,
                    node: h.node,
                    end_line,
                };
                Some(ParseTsGroup {
                    key: TsGroupKey::Heading(Heading {
                        level: h.level,
                        boilerplate: h.boilerplate,
                    }),
                    items: vec![item],
                    dependent_siblings: Vec::new(),
                })
            })
            .collect();

        // Per-individual parent: the nearest earlier heading with strictly
        // lower level. Stack holds indices in ascending level.
        let mut parent_of: Vec<Option<usize>> = vec![None; infos.len()];
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..infos.len() {
            while let Some(&top) = stack.last() {
                if infos[top].level >= infos[i].level {
                    stack.pop();
                } else {
                    break;
                }
            }
            parent_of[i] = stack.last().copied();
            stack.push(i);
        }

        // Process in reverse so a child's dependent_siblings stay attached
        // as we lift it into its parent.
        for i in (0..infos.len()).rev() {
            if let Some(p) = parent_of[i] {
                let child = groups[i].take().expect("child slot populated");
                groups[p]
                    .as_mut()
                    .expect("parent slot populated")
                    .dependent_siblings
                    .push(child);
            }
        }

        groups.into_iter().flatten().collect()
    }
}
