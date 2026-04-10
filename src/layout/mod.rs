mod doc;
mod noise;
mod signature;

use crate::parse;
use crate::Lang;

pub(crate) use noise::strip_heading_badges;

/// Pre-computed line ranges for a single symbol. Computed once per symbol,
/// then read by both the scheduler (for token-cost counting) and the renderer
/// (for line emission). All line numbers are 0-indexed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SymbolLayout {
    /// First content line of the doc comment block preceding the symbol.
    /// Skips pure block-comment delimiters (`/**`, `/*`) that carry no
    /// information. Equal to `doc_end` when no doc comment exists.
    pub doc_start: usize,
    /// Exclusive end of doc comment content lines. Skips trailing
    /// block-comment closing delimiters (` */`). Equal to `doc_start`
    /// when no doc comment exists.
    pub doc_end: usize,
    /// The symbol's own first line (`sym.line - 1`).
    pub sym_line_0: usize,
    /// Last line of the signature (inclusive). For single-line symbols this
    /// equals `sym_line_0`; for multi-line sigs it's the line with `{`/`;`/`:`.
    pub sig_end: usize,
    /// Python docstring start (exclusive of signature). Equal to `ds_end`
    /// when no docstring exists.
    pub ds_start: usize,
    /// Python docstring end (exclusive). Equal to `ds_start` when no docstring.
    pub ds_end: usize,
    /// First body line. For code symbols: after signature and any Python
    /// docstring. For markdown sections: first content line after leading
    /// noise (badges, blank lines, link refs).
    pub body_start: usize,
    /// Exclusive end of body. For code symbols with nested children, this is
    /// truncated to the first child symbol's line. For markdown sections, this
    /// is the line of the next heading (or EOF). For symbols without children
    /// this is `sym.end_line.min(lines.len())`.
    pub body_end: usize,
    /// Whether this symbol's body region contains nested child symbols
    /// (e.g. methods inside a class or impl block).
    pub has_children: bool,
}

/// Compute the layout for a single symbol within a file.
pub(crate) fn compute_layout(
    sym: &parse::Symbol,
    sym_idx: usize,
    all_symbols: &[parse::Symbol],
    lines: &[&str],
    lang: Option<Lang>,
) -> SymbolLayout {
    let raw_sym_line_0 = sym.line - 1;
    let raw_doc_start = sym
        .doc_start_line
        .map(|l| l - 1)
        .unwrap_or_else(|| doc::doc_comment_start(lines, raw_sym_line_0, lang));

    // Trim pure block-comment delimiters from doc range so Doc(1) shows
    // actual content instead of a bare `/**` or `/*`.
    let (doc_start, doc_end) = doc::trim_doc_delimiters(lines, raw_doc_start, raw_sym_line_0);

    // For ModuleDoc: advance past leading noise (badges, blank doc lines, HTML,
    // link reference definitions) to find the first meaningful line, which
    // becomes the "name" shown at the Names stage.
    let sym_line_0 = if sym.kind == parse::SymbolKind::ModuleDoc {
        let end = sym.end_line.min(lines.len());
        // Fallback: if all lines are noise, keep the original line so the
        // symbol still renders something rather than vanishing.
        doc::skip_doc_leading_noise(lines, raw_sym_line_0, end, lang).unwrap_or(raw_sym_line_0)
    } else {
        raw_sym_line_0
    };

    let sig_end = if sym.kind == parse::SymbolKind::ModuleDoc {
        sym_line_0
    } else {
        signature::signature_end_line(lines, sym, lang)
    };

    // Python docstring range
    let (ds_start, ds_end) = if lang == Some(Lang::Python) {
        let ds_end = doc::docstring_end(lines, sig_end);
        if ds_end > sig_end + 1 {
            (sig_end + 1, ds_end)
        } else {
            (sig_end + 1, sig_end + 1)
        }
    } else {
        (sig_end + 1, sig_end + 1)
    };

    // Body range: section content for markdown/module docs, code body otherwise.
    let (raw_body_start, raw_body_end) = if sym.kind == parse::SymbolKind::Section {
        let next_heading_line = all_symbols
            .iter()
            .skip(sym_idx + 1)
            .find(|s| s.kind == parse::SymbolKind::Section)
            .map(|s| s.line - 1)
            .unwrap_or(lines.len());
        // Config nodes span the full value; use just the key line as heading.
        let is_config = matches!(lang, Some(Lang::Json | Lang::Toml | Lang::Yaml));
        let heading_end = if is_config {
            sym_line_0 + 1
        } else {
            (sym.end_line - 1).max(sym_line_0 + 1)
        };
        let mut content_start = heading_end;
        while content_start < next_heading_line
            && noise::is_markdown_leading_noise(lines[content_start])
        {
            content_start += 1;
        }
        // Trim trailing noise from section bodies
        let mut section_end = next_heading_line;
        if is_config {
            // Config: closing delimiters, blank lines
            while section_end > content_start
                && noise::is_config_trailing_noise(lines[section_end - 1])
            {
                section_end -= 1;
            }
        } else {
            section_end = noise::trim_trailing_blank_lines(lines, content_start, section_end);
            // Large trailing ToC blocks (5+ consecutive `- [Text](#anchor)` lines)
            // duplicate the heading structure precis already shows. Short navigation
            // lists (< 5 items) are preserved.
            let mut toc_start = section_end;
            while toc_start > content_start
                && noise::is_toc_link(lines[toc_start - 1].trim())
            {
                toc_start -= 1;
            }
            if section_end - toc_start >= 5 && toc_start > content_start {
                section_end = noise::trim_trailing_blank_lines(lines, content_start, toc_start);
            }
        }
        (content_start, section_end)
    } else if sym.kind == parse::SymbolKind::ModuleDoc {
        let mut body_end = sym.end_line.min(lines.len());
        let content_start = doc::skip_doc_leading_noise(lines, sym_line_0 + 1, body_end, lang)
            .unwrap_or(body_end);
        // Trim trailing delimiter-only lines and blank padding (`*/`, `"""`)
        while body_end > content_start {
            let content = doc::strip_doc_line_prefix(lines[body_end - 1], lang);
            if !content.trim().is_empty() {
                break;
            }
            body_end -= 1;
        }
        (content_start, body_end)
    } else {
        let start = if ds_end > ds_start {
            ds_end // skip past Python docstring
        } else {
            sig_end + 1
        };
        (start, sym.end_line.min(lines.len()))
    };

    // Find first child symbol within body (for nesting detection and body truncation).
    // Truncate at the child's doc_start (not sym.line) so the parent doesn't claim
    // the child's doc comment lines. Section-like symbols never have children (their
    // sub-headings are sibling symbols, not nested children).
    let (has_children, first_child_start) = if sym.kind.is_section_like() {
        (false, None)
    } else {
        let first = all_symbols
            .iter()
            .skip(sym_idx + 1)
            .find(|s| {
                let sl = s.line - 1;
                sl >= raw_body_start && sl < raw_body_end
            });
        match first {
            Some(child) => {
                let child_doc = child
                    .doc_start_line
                    .map(|l| l - 1)
                    .unwrap_or_else(|| doc::doc_comment_start(lines, child.line - 1, lang));
                (true, Some(child_doc.max(raw_body_start)))
            }
            None => (false, None),
        }
    };

    // Truncate body at first child — the parent only "owns" lines before children
    let body_start = raw_body_start;
    let body_end = if let Some(child_start) = first_child_start {
        child_start.min(raw_body_end)
    } else {
        raw_body_end
    };

    SymbolLayout {
        doc_start,
        doc_end,
        sym_line_0,
        sig_end,
        ds_start,
        ds_end,
        body_start,
        body_end,
        has_children,
    }
}

/// Compute and assign layouts for all symbols in a single file.
///
/// Uses a two-pass approach: first computes all layouts into a temporary Vec
/// (because computing layout for symbol N reads other symbols for inter-symbol
/// dependencies, requiring shared borrows), then assigns them back to each symbol.
pub fn fill_layouts(symbols: &mut [parse::Symbol], lines: &[&str], lang: Option<Lang>) {
    let layouts: Vec<SymbolLayout> = symbols
        .iter()
        .enumerate()
        .map(|(sym_idx, sym)| compute_layout(sym, sym_idx, symbols, lines, lang))
        .collect();
    for (sym, layout) in symbols.iter_mut().zip(layouts) {
        sym.layout = layout;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to compute body range for a markdown section with the given body lines.
    /// Returns (body_start, body_end) as 0-indexed line numbers.
    fn section_body_range(body_lines: &[&str]) -> (usize, usize) {
        // Build a minimal markdown file: heading + body lines + next heading
        let mut source = String::from("# Heading\n");
        for line in body_lines {
            source.push_str(line);
            source.push('\n');
        }
        source.push_str("## Next\n");
        let lines: Vec<&str> = source.lines().collect();

        let sym = parse::Symbol {
            kind: parse::SymbolKind::Section,
            name: "Heading".to_string(),
            is_public: true,
            is_first_party: false,
            line: 1,
            end_line: 1,
            sig_end_line: None,
            doc_start_line: None,
            is_trait_impl: false,
            is_reexport: false,
            start_byte: 0,
            end_byte: 0,
            composed_prefix_lens: Vec::new(),
            layout: Default::default(),
        };
        let next_sym = parse::Symbol {
            kind: parse::SymbolKind::Section,
            name: "Next".to_string(),
            is_public: true,
            is_first_party: false,
            line: lines.len(),
            end_line: lines.len(),
            sig_end_line: None,
            doc_start_line: None,
            is_trait_impl: false,
            is_reexport: false,
            start_byte: 0,
            end_byte: 0,
            composed_prefix_lens: Vec::new(),
            layout: Default::default(),
        };
        let all_symbols = [sym.clone(), next_sym];
        let layout = compute_layout(&all_symbols[0], 0, &all_symbols, &lines, Some(Lang::Markdown));
        (layout.body_start, layout.body_end)
    }

    #[test]
    fn section_body_trims_trailing_blank_lines() {
        let (start, end) = section_body_range(&[
            "Content here.",
            "",
            "",
        ]);
        // body_start = 1 (content), body_end = 2 (trailing blanks trimmed)
        assert_eq!(start, 1);
        assert_eq!(end, 2);
    }

    #[test]
    fn section_body_trims_large_trailing_toc() {
        let (start, end) = section_body_range(&[
            "A brief introduction.",
            "",
            "- [Section A](#section-a)",
            "- [Section B](#section-b)",
            "- [Section C](#section-c)",
            "- [Section D](#section-d)",
            "- [Section E](#section-e)",
            "",
        ]);
        // body_start = 1 (intro), body_end = 2 (ToC + blanks trimmed)
        assert_eq!(start, 1);
        assert_eq!(end, 2);
    }

    #[test]
    fn section_body_preserves_short_link_list() {
        let (start, end) = section_body_range(&[
            "Content here.",
            "- [Link A](#a)",
            "- [Link B](#b)",
            "- [Link C](#c)",
        ]);
        // Short list (3 items < 5 threshold) preserved
        assert_eq!(start, 1);
        assert_eq!(end, 5); // all 4 content lines
    }

    #[test]
    fn section_body_preserves_link_reference_definitions() {
        let (start, end) = section_body_range(&[
            "Read the [docs][docs-link].",
            "",
            "[docs-link]: https://example.com/docs",
        ]);
        // Link references are NOT trimmed (they don't match is_toc_link)
        assert_eq!(start, 1);
        assert_eq!(end, 4);
    }

    #[test]
    fn section_body_toc_only_already_empty_from_leading_skip() {
        // A section whose ONLY content is ToC links: leading noise skip
        // already removes all lines, so the body is empty before trailing
        // trim even runs. This is pre-existing behavior.
        let (start, end) = section_body_range(&[
            "- [Section A](#section-a)",
            "- [Section B](#section-b)",
            "- [Section C](#section-c)",
            "- [Section D](#section-d)",
            "- [Section E](#section-e)",
            "- [Section F](#section-f)",
        ]);
        // Leading noise skip advances past all ToC lines → empty body
        assert_eq!(start, end);
    }

    #[test]
    fn section_body_toc_after_content_trims_only_toc() {
        // Content followed by a large ToC block: the content is preserved,
        // the ToC is trimmed, and the guard `toc_start > content_start`
        // ensures we don't collapse the body entirely.
        let (start, end) = section_body_range(&[
            "Introduction text.",
            "More details here.",
            "",
            "- [A](#a)",
            "- [B](#b)",
            "- [C](#c)",
            "- [D](#d)",
            "- [E](#e)",
        ]);
        assert_eq!(start, 1);
        assert_eq!(end, 3); // two content lines, ToC + blank trimmed
    }
}
