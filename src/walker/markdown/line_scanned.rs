//! The line-scanned README path: reST and AsciiDoc headings and ledes
//! found by scanning lines, split into the same `ReadmeHeadline` and
//! `Section` shapes the Markdown path emits.

use std::collections::BTreeSet;

use super::{
    ReadmeMarkup, SectionRange, is_appendix_title_core, is_canonical_usage_title_core,
    is_reference_usage_title_core, push_whole_or_head_split, title_core,
};

/// True for a real reST/AsciiDoc prose line — not a heading underline,
/// badge row (`|Build Status| …`, `image:…[…]`), directive (`.. figure::`,
/// `toc::[]`, `ifdef::…`), link macro, or attribute entry (`:toc: macro`).
fn is_rst_prose_line(line: &str) -> bool {
    let t = line.trim();
    if t.is_empty() || t.starts_with("..") || t.starts_with('|') || is_rst_underline(t, 1) {
        return false;
    }
    let first_word = t.split_whitespace().next().unwrap_or_default();
    if first_word.contains("::")
        || t.starts_with("image:")
        || t.starts_with("link:")
        || t.strip_prefix(':')
            .and_then(|rest| rest.split_once(':'))
            .is_some_and(|(name, _)| {
                !name.is_empty()
                    && name
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            })
    {
        return false;
    }
    t.split_whitespace()
        .filter(|w| w.chars().any(|c| c.is_alphabetic()))
        .count()
        >= 3
}

fn is_rst_underline(line: &str, min_width: usize) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < min_width || trimmed.is_empty() {
        return false;
    }
    let first = trimmed.chars().next().unwrap();
    matches!(first, '=' | '-' | '~' | '^' | '*' | '+' | '#') && trimmed.chars().all(|c| c == first)
}

/// A scanned RST heading: the 1-based row of the title text and the
/// 1-based row of the underline that closes it. `start_row` is the
/// heading's first row — the overline row for overline form, the title
/// row for setext form — so a following section can bound itself at
/// `start_row - 1` and not swallow this heading's overline punctuation.
struct RstHeading {
    start_row: usize,
    title_row: usize,
    underline_row: usize,
}

/// Line-scan all setext/overline RST headings. The first heading is the
/// document title; subsequent ones bound body sections.
fn scan_rst_headings(src_lines: &[&str]) -> Vec<RstHeading> {
    let mut headings = Vec::new();
    let mut i = 0;
    while i < src_lines.len() {
        // Overline form: punctuation row / title / matching punctuation.
        if i + 2 < src_lines.len()
            && is_rst_underline(src_lines[i], 1)
            && is_rst_underline(src_lines[i + 2], 1)
            && src_lines[i].trim() == src_lines[i + 2].trim()
            && !src_lines[i + 1].trim().is_empty()
        {
            headings.push(RstHeading {
                start_row: i + 1,
                title_row: i + 2,
                underline_row: i + 3,
            });
            i += 3;
            continue;
        }
        // Setext form: a non-blank title row followed by an underline at
        // least as wide as the title.
        if i + 1 < src_lines.len()
            && let Some(title) = src_lines.get(i).filter(|l| !l.trim().is_empty())
            && is_rst_underline(src_lines[i + 1], title.trim_end().chars().count())
        {
            headings.push(RstHeading {
                start_row: i + 1,
                title_row: i + 1,
                underline_row: i + 2,
            });
            i += 2;
            continue;
        }
        i += 1;
    }
    headings
}

/// Line-scan AsciiDoc section titles (`= Title`, `== Section`, …, and the
/// two-line form: a title underlined by `=`, `-`, `~`, `^` or `+` within
/// one character of its width) outside delimited blocks (`----` listings,
/// `....` literals, `====` examples, …), whose lines are content even when
/// they start with `= `.
fn scan_asciidoc_headings(src_lines: &[&str]) -> Vec<RstHeading> {
    let mut open_delimiter: Option<&str> = None;
    let mut headings = Vec::new();
    for (i, line) in src_lines.iter().enumerate() {
        let trimmed = line.trim_end();
        if open_delimiter.is_none()
            && i > 0
            && is_asciidoc_two_line_title(src_lines[i - 1].trim_end(), trimmed)
        {
            headings.push(RstHeading {
                start_row: i,
                title_row: i,
                underline_row: i + 1,
            });
            continue;
        }
        if is_asciidoc_block_delimiter(trimmed) {
            match open_delimiter {
                None => open_delimiter = Some(trimmed),
                Some(open) if open == trimmed => open_delimiter = None,
                Some(_) => {}
            }
            continue;
        }
        let rest = line.trim_start_matches('=');
        if open_delimiter.is_none()
            && (1..=6).contains(&(line.len() - rest.len()))
            && rest.starts_with(' ')
        {
            headings.push(RstHeading {
                start_row: i + 1,
                title_row: i + 1,
                underline_row: i + 1,
            });
        }
    }
    headings
}

/// `title` over `underline` is a two-line section title: a title that is
/// not a block title (`.Title`), attribute list (`[source]`) or delimiter,
/// over a run of one underline character within one of its width.
fn is_asciidoc_two_line_title(title: &str, underline: &str) -> bool {
    let title_width = title.chars().count();
    underline.chars().next().is_some_and(|first| {
        matches!(first, '=' | '-' | '~' | '^' | '+')
            && underline.chars().all(|c| c == first)
            && underline.len().abs_diff(title_width) <= 1
    }) && !title.starts_with(['.', '['])
        && title.chars().any(char::is_alphanumeric)
        && !is_asciidoc_block_delimiter(title)
}

/// A run of four or more of one AsciiDoc block-delimiter character alone
/// on its line; the same run closes the block.
fn is_asciidoc_block_delimiter(line: &str) -> bool {
    line.chars().next().is_some_and(|first| {
        matches!(first, '-' | '.' | '=' | '*' | '_' | '+' | '/')
            && line.len() >= 4
            && line.chars().all(|c| c == first)
    })
}

/// A reST or AsciiDoc README split into its `ReadmeHeadline` and
/// `Section` ranges. The first heading is the title unless prose
/// precedes it. The headline is the title plus the first prose paragraph
/// under it — or, when there is none, the first prose paragraph of an
/// intro-titled section (`Overview`, `Introduction`, …). The text before
/// the first body heading is a synthetic intro, and every body heading
/// opens one section, each through the oversize head-split.
pub(super) fn line_scanned_readme(
    source: &str,
    markup: ReadmeMarkup,
) -> (BTreeSet<usize>, Vec<SectionRange>) {
    let src_lines: Vec<&str> = source.lines().collect();
    let headings = if markup == ReadmeMarkup::AsciiDoc {
        scan_asciidoc_headings(&src_lines)
    } else {
        scan_rst_headings(&src_lines)
    };
    let has_title = headings.first().is_some_and(|first| {
        let above = rst_content_rows(&src_lines, 1, first.start_row - 1);
        rst_lede(&src_lines, &above).is_none()
    });
    let body = &headings[usize::from(has_title)..];
    let section_end = |i: usize| {
        body.get(i + 1)
            .map_or(src_lines.len(), |next| next.start_row - 1)
    };
    let intro_end = body
        .first()
        .map_or(src_lines.len(), |next| next.start_row - 1);
    let mut covered_rows = BTreeSet::new();
    let mut intro_start = 1;
    if has_title {
        let title = &headings[0];
        covered_rows.extend(title.start_row..=title.underline_row);
        intro_start = title.underline_row + 1;
    }
    let intro = rst_content_rows(&src_lines, intro_start, intro_end);
    // `lede_section` is `None` for the intro, else the body heading index.
    let mut lede_section = None;
    if let Some(lede) = rst_lede(&src_lines, &intro) {
        covered_rows.extend(&intro[lede]);
    } else if let Some((i, rows, lede)) = body.iter().enumerate().find_map(|(i, h)| {
        let title = title_core(src_lines[h.title_row - 1]);
        if !matches!(
            title.as_str(),
            "overview" | "introduction" | "about" | "summary"
        ) {
            return None;
        }
        let rows = rst_content_rows(&src_lines, h.underline_row + 1, section_end(i));
        rst_lede(&src_lines, &rows).map(|lede| (i, rows, lede))
    }) {
        covered_rows.extend(&rows[lede]);
        lede_section = Some(i);
    }
    // The section holding the lede starts past it, dropping the rows the
    // headline stepped over; every other section stays whole.
    let lede_end = covered_rows.last().copied().unwrap_or(0);
    let start_for = |section: Option<usize>, start: usize| {
        if lede_section == section {
            (lede_end + 1).max(start)
        } else {
            start
        }
    };
    let mut ranges = Vec::new();
    let intro_start = start_for(None, intro_start);
    if intro_start <= intro_end {
        let intro = SectionRange::new(intro_start, intro_end, 0);
        push_whole_or_head_split(&mut ranges, &src_lines, intro);
    }
    for (i, heading) in body.iter().enumerate() {
        let (start, end) = (start_for(Some(i), heading.title_row), section_end(i));
        if start > end {
            continue;
        }
        let title = title_core(src_lines[heading.title_row - 1]);
        if is_appendix_title_core(&title) {
            continue;
        }
        push_whole_or_head_split(
            &mut ranges,
            &src_lines,
            SectionRange {
                is_reference_usage_section: is_canonical_usage_title_core(&title)
                    || is_reference_usage_title_core(&title),
                ..SectionRange::new(start, end, i)
            },
        );
    }
    (covered_rows, ranges)
}

/// Index range in `rows` of the first prose paragraph: the first prose
/// line and the source-contiguous rows after it.
fn rst_lede(src_lines: &[&str], rows: &[usize]) -> Option<std::ops::Range<usize>> {
    let start = rows
        .iter()
        .position(|&row| is_rst_prose_line(src_lines[row - 1]))?;
    let len = rows[start..]
        .iter()
        .zip(rows[start]..)
        .take_while(|(row, expected)| **row == *expected)
        .count();
    Some(start..start + len)
}

/// 1-based rows of reST decoration, which no `Section` batch shows:
/// comments, hyperlink targets, substitution definitions and `image`,
/// `figure`, `raw` and `contents` directives with their indented bodies,
/// and lines of bare substitution references (`|build| |docs|`) with the
/// grid-table borders around them.
pub(super) fn rst_chrome_rows(source: &str) -> BTreeSet<usize> {
    let src_lines: Vec<&str> = source.lines().collect();
    let is_reference_line = |index: Option<usize>| {
        index
            .and_then(|i| src_lines.get(i))
            .is_some_and(|line| is_substitution_reference_line(line))
    };
    let mut rows = BTreeSet::new();
    let mut open_block_indent: Option<usize> = None;
    for (i, line) in src_lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if open_block_indent.is_some_and(|open| indent > open) {
            rows.insert(i + 1);
            continue;
        }
        open_block_indent = None;
        let markup_body = trimmed
            .strip_prefix("..")
            .filter(|rest| rest.is_empty() || rest.starts_with(' '))
            .map(str::trim);
        if markup_body.is_some_and(|body| match body.split_once("::") {
            _ if body.starts_with(['|', '_']) => true,
            Some((name, _)) if !name.contains(char::is_whitespace) => {
                matches!(name, "image" | "figure" | "raw" | "contents")
            }
            _ => !body.starts_with('['),
        }) {
            open_block_indent = Some(indent);
            rows.insert(i + 1);
        } else if is_reference_line(Some(i))
            || (trimmed.starts_with('+')
                && trimmed.chars().all(|c| matches!(c, '+' | '-' | '='))
                && (is_reference_line(i.checked_sub(1)) || is_reference_line(Some(i + 1))))
        {
            rows.insert(i + 1);
        }
    }
    rows
}

/// `|Build Status| |Docs|` or a grid-table row of them (`| |1| |2| |`):
/// text between pipes is blank or a whole substitution name.
fn is_substitution_reference_line(line: &str) -> bool {
    let line = line.trim();
    line.starts_with('|')
        && line.split('|').any(|piece| !piece.trim().is_empty())
        && line
            .split('|')
            .all(|piece| piece.trim().is_empty() || piece.trim() == piece)
}

/// Non-blank rows (1-based) of `[start, end]` outside `.. directive::`
/// blocks and their indented continuations — where a lede can sit.
fn rst_content_rows(src_lines: &[&str], start: usize, end: usize) -> Vec<usize> {
    let mut rows = Vec::new();
    let mut directive_indent: Option<usize> = None;
    for row in start..=end.min(src_lines.len()) {
        let line = src_lines[row - 1];
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let indent = line.len() - trimmed.len();
        if directive_indent.is_some_and(|open| indent > open) {
            continue;
        }
        directive_indent = None;
        if trimmed.starts_with("..") {
            directive_indent = Some(indent);
            continue;
        }
        rows.push(row);
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::{BatchKey, MarkdownKey};
    use crate::content::BatchContent;
    use crate::walker::WalkCtx;

    #[test]
    fn line_scanned_asciidoc_readme_title_heads_the_headline() {
        let src = "= Tool\n\nTool does the useful thing for you.\n\n== Install\n\nrun make\n";
        let (headline, sections) = line_scanned_readme(src, ReadmeMarkup::AsciiDoc);
        assert_eq!(headline.into_iter().collect::<Vec<_>>(), vec![1, 3]);
        let last = sections.last().unwrap();
        assert_eq!((last.start, last.end), (5, 7));
    }

    #[test]
    fn line_scanned_asciidoc_headings_skip_delimited_blocks() {
        let src = "\
= Tool

== Usage

----
$ tool --sum
= 42
----

....
== not a section either
....

== License
";
        let rows: Vec<usize> = scan_asciidoc_headings(&src.lines().collect::<Vec<_>>())
            .iter()
            .map(|heading| heading.title_row)
            .collect();
        assert_eq!(rows, vec![1, 3, 14]);
    }

    #[test]
    fn line_scanned_asciidoc_two_line_titles_are_headings_not_delimiters() {
        let src = "\
Project
=======

.Run
----
make
----

Install
-------

== Usage

run make
";
        let rows: Vec<usize> = scan_asciidoc_headings(&src.lines().collect::<Vec<_>>())
            .iter()
            .map(|heading| heading.title_row)
            .collect();
        assert_eq!(rows, vec![1, 9, 12]);
    }

    #[test]
    fn line_scanned_asciidoc_readme_lede_skips_badges_and_attributes() {
        let src = "\
image:https://img.shields.io/badge/a-b-c.svg[Badge above the title, link=x]

= Tool
:description: A fast tool for the useful thing.
:toc: macro

image:https://github.com/x/tool/workflows/CI/badge.svg[CI, link=x] image:https://img.shields.io/crates/v/tool.svg[Crates.io, link=x]

toc::[]

Tool is a small utility that does the useful thing for you.
";
        let (headline, _) = line_scanned_readme(src, ReadmeMarkup::AsciiDoc);
        assert_eq!(headline.into_iter().collect::<Vec<_>>(), vec![3, 11]);
    }

    /// Overline-form headings must not leak their overline punctuation row
    /// into the *preceding* section's span: a section bounds at the next
    /// heading's `start_row - 1` (the overline row), not its title row.
    #[test]
    fn line_scanned_rst_overline_section_excludes_next_overline_row() {
        let src = "\
======
Title
======

Intro prose for the title.

========
Overview
========

Overview prose paragraph one.

=======
Details
=======

Details prose paragraph one.
";
        let src_lines: Vec<&str> = src.lines().collect();
        let headings = scan_rst_headings(&src_lines);
        // title + Overview + Details
        assert_eq!(headings.len(), 3);
        let sections = line_scanned_readme(src, ReadmeMarkup::Rst).1;
        assert_eq!(sections.len(), 3, "intro + Overview + Details");
        for section in &sections {
            let own_start = Some(section.start);
            for row in section.start..=section.end {
                // A section legitimately starts at its own overline row;
                // it must not contain any *other* heading's overline row.
                let foreign_overline = headings
                    .iter()
                    .any(|h| h.start_row == row && Some(row) != own_start);
                assert!(
                    !foreign_overline,
                    "section row {row} landed on another heading's overline punctuation"
                );
            }
        }
    }

    /// When the RST lede comes from a later `Overview`, the sections
    /// between the title and it stay whole; only the Overview section
    /// starts past the lede.
    #[test]
    fn line_scanned_rst_fallback_lede_keeps_preceding_sections() {
        let src = "\
Title
=====

Installation
------------

Run pip install the package now.

Overview
--------

Overview prose paragraph one here.

Overview trailing details stay here.
";
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("README.rst"), src).unwrap();
        let ctx = WalkCtx::new(root.to_path_buf());
        let mut headline_spans = Vec::new();
        let mut section_spans = Vec::new();
        for batch in super::super::expand_in_dir(root, &ctx) {
            let BatchContent::Lines { spans, .. } = batch.content else {
                continue;
            };
            let rows = spans.iter().map(|span| (span.start, span.end));
            match batch.key {
                BatchKey::Markdown(MarkdownKey::ReadmeHeadline { .. }) => {
                    headline_spans.extend(rows);
                }
                BatchKey::Markdown(MarkdownKey::Section { .. }) => section_spans.extend(rows),
                _ => {}
            }
        }
        assert_eq!(headline_spans, vec![(1, 2), (12, 12)]);
        assert_eq!(section_spans, vec![(4, 7), (14, 14)]);
    }

    #[test]
    fn line_scanned_rst_chrome_is_badges_images_and_targets() {
        let src = "\
Tool
====

+---------+
| |1| |2| |
+---------+

.. |1| image:: https://ci.example/badge
    :target: https://ci.example
.. |2| image:: https://docs.example/badge

|Build Status| |Docs|

Tool does the useful thing. See the `guide`_.

| A line block keeps
| its prose.

.. _guide: https://guide.example
.. links

.. [1] A footnote stays.

.. code-block:: sh

    make

.. figure:: logo.svg
   :alt: Logo
";
        let rows: Vec<usize> = rst_chrome_rows(src).into_iter().collect();
        assert_eq!(rows, vec![4, 5, 6, 8, 9, 10, 12, 19, 20, 28, 29]);
    }
}
