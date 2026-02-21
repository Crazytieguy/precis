use std::path::PathBuf;

use crate::parse;
use crate::Lang;

// ---------------------------------------------------------------------------
// Symbol layout (shared between scheduler and renderer)
// ---------------------------------------------------------------------------

/// Pre-computed line ranges for a single symbol. Computed once per symbol,
/// then read by both the scheduler (for token-cost counting) and the renderer
/// (for line emission). All line numbers are 0-indexed.
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
    /// First body line (after signature and any Python docstring).
    pub body_start: usize,
    /// Exclusive end of body. For code symbols with nested children, this is
    /// truncated to the first child symbol's line. For symbols without children
    /// this is `sym.end_line.min(lines.len())`.
    pub body_end: usize,
    /// For markdown sections: first content line after leading noise (badges,
    /// blank lines, link refs). Zero for non-section symbols.
    pub md_content_start: usize,
    /// For markdown sections: the line of the next heading (or EOF). Zero for
    /// non-section symbols.
    pub md_section_end: usize,
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
    let sym_line_0 = sym.line - 1;
    let raw_doc_start = sym
        .doc_start_line
        .map(|l| l - 1)
        .unwrap_or_else(|| doc_comment_start(lines, sym_line_0, lang));

    // Trim pure block-comment delimiters from doc range so Doc(1) shows
    // actual content instead of a bare `/**` or `/*`.
    let (doc_start, doc_end) = trim_doc_delimiters(lines, raw_doc_start, sym_line_0);

    let sig_end = signature_end_line(lines, sym, lang);

    // Python docstring range
    let (ds_start, ds_end) = if lang == Some(Lang::Python) {
        let ds_end = docstring_end(lines, sig_end);
        if ds_end > sig_end + 1 {
            (sig_end + 1, ds_end)
        } else {
            (sig_end + 1, sig_end + 1)
        }
    } else {
        (sig_end + 1, sig_end + 1)
    };

    // Markdown section body range
    let (md_content_start, md_section_end) = if sym.kind == parse::SymbolKind::Section {
        let next_heading_line = all_symbols
            .iter()
            .skip(sym_idx + 1)
            .find(|s| s.kind == parse::SymbolKind::Section)
            .map(|s| s.line - 1)
            .unwrap_or(lines.len());
        let heading_end = (sym.end_line - 1).max(sym_line_0 + 1);
        let mut content_start = heading_end;
        while content_start < next_heading_line
            && is_markdown_leading_noise(lines[content_start])
        {
            content_start += 1;
        }
        (content_start, next_heading_line)
    } else {
        (0, 0)
    };

    // Code body range
    let raw_body_start = if sym.kind == parse::SymbolKind::Section {
        // For markdown, body_start/body_end aren't used (md_content_start/md_section_end are)
        0
    } else if ds_end > ds_start {
        ds_end // skip past Python docstring
    } else {
        sig_end + 1
    };
    let raw_body_end = if sym.kind == parse::SymbolKind::Section {
        0
    } else {
        sym.end_line.min(lines.len())
    };

    // Find first child symbol within body (for nesting detection and body truncation).
    // Truncate at the child's doc_start (not sym.line) so the parent doesn't claim
    // the child's doc comment lines.
    let (has_children, first_child_start) = if sym.kind == parse::SymbolKind::Section {
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
                    .unwrap_or_else(|| doc_comment_start(lines, child.line - 1, lang));
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
        md_content_start,
        md_section_end,
        has_children,
    }
}

/// Compute layouts for all symbols across all files.
pub fn compute_all_layouts(
    files: &[PathBuf],
    sources: &[Option<String>],
    all_symbols: &[Vec<parse::Symbol>],
) -> Vec<Vec<SymbolLayout>> {
    files
        .iter()
        .zip(sources.iter())
        .zip(all_symbols.iter())
        .map(|((file, source), symbols)| {
            let source = match source {
                Some(s) => s,
                None => return Vec::new(),
            };
            let lines: Vec<&str> = source.lines().collect();
            let lang = Lang::from_path(file);
            symbols
                .iter()
                .enumerate()
                .map(|(sym_idx, sym)| compute_layout(sym, sym_idx, symbols, &lines, lang))
                .collect()
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Signature detection
// ---------------------------------------------------------------------------

/// Whether a symbol kind can have a multi-line signature (parameters, `where` clauses, bounds).
fn has_multiline_signature(kind: parse::SymbolKind) -> bool {
    matches!(
        kind,
        parse::SymbolKind::Function
            | parse::SymbolKind::Impl
            | parse::SymbolKind::Trait
            | parse::SymbolKind::Struct
            | parse::SymbolKind::Enum
            | parse::SymbolKind::Class
            | parse::SymbolKind::Interface
    )
}

/// Find the last line of a function's signature (0-indexed).
///
/// Prefers the tree-sitter-computed `sig_end_line` from [`parse::Symbol`] when
/// available, falling back to text heuristics for symbols where tree-sitter
/// couldn't determine the boundary (type aliases, method signatures, constants).
///
/// The text fallback scans forward for the opening body delimiter (`{` or `;`
/// for C-like languages, `:` for Python). Strips trailing line comments (`//`
/// or `#`) before checking delimiters, so `interface Foo { // eslint-disable`
/// correctly matches on `{`.
///
/// Returns sym.line - 1 for single-line or non-function symbols.
pub(crate) fn signature_end_line(lines: &[&str], sym: &parse::Symbol, lang: Option<Lang>) -> usize {
    let sym_line_0 = sym.line - 1;
    // Imports: the entire declaration IS the signature (multi-line
    // Rust `use foo::{A, B, C};` or Go grouped imports).
    if sym.kind == parse::SymbolKind::Import {
        return sym.end_line.min(lines.len()) - 1;
    }
    // Tree-sitter computed sig_end takes precedence over text heuristics.
    // Converted from 1-indexed to 0-indexed.
    if let Some(sig_end) = sym.sig_end_line {
        return (sig_end - 1).min(lines.len().saturating_sub(1));
    }
    // --- Text heuristic fallback for nodes without tree-sitter body data ---
    // Type aliases: scan for `{` to find the end of the signature.
    // Simple aliases (`type Foo = Bar;`) use the entire declaration.
    // Complex aliases (`type Foo = { ... }`) stop at `{` so the body
    // with nested method signatures isn't part of the signature.
    if sym.kind == parse::SymbolKind::TypeAlias {
        let max_line = sym.end_line.min(lines.len());
        for (i, line) in lines.iter().enumerate().take(max_line).skip(sym_line_0) {
            let code = strip_c_line_comment(line.trim());
            if code.ends_with('{') {
                return i;
            }
        }
        return sym.end_line.min(lines.len()) - 1;
    }
    if !has_multiline_signature(sym.kind) {
        return sym_line_0;
    }
    let max_line = sym.end_line.min(lines.len());
    for (i, line) in lines.iter().enumerate().take(max_line).skip(sym_line_0) {
        let trimmed = line.trim();
        if lang == Some(Lang::Python) {
            let code = strip_python_line_comment(trimmed);
            if code.ends_with(':') {
                return i;
            }
        } else {
            let code = strip_c_line_comment(trimmed);
            if code.ends_with('{') || code.ends_with(';') {
                return i;
            }
        }
    }
    sym_line_0
}

/// Strip a trailing `//` line comment, returning the code portion.
/// Skips `://` sequences (URLs like `https://...`) to avoid false positives.
fn strip_c_line_comment(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'/' {
            // Skip :// (URLs like https://)
            if i > 0 && bytes[i - 1] == b':' {
                i += 2;
                continue;
            }
            return s[..i].trim_end();
        }
        i += 1;
    }
    s
}

/// Strip a trailing `#` comment from a Python line, returning the code portion.
fn strip_python_line_comment(s: &str) -> &str {
    match s.find('#') {
        Some(pos) => s[..pos].trim_end(),
        None => s,
    }
}

// ---------------------------------------------------------------------------
// Doc comment detection
// ---------------------------------------------------------------------------

/// Find the first line (0-indexed) of the doc comment block preceding a symbol.
/// Returns the symbol's own line index if there's no doc comment.
pub(crate) fn doc_comment_start(lines: &[&str], symbol_line_0: usize, lang: Option<Lang>) -> usize {
    if symbol_line_0 == 0 {
        return symbol_line_0;
    }

    // Walk backwards over Rust attributes (#[...]) and TS/JS decorators (@...)
    let mut idx = symbol_line_0;
    while idx > 0 {
        let prev = lines[idx - 1].trim();
        if prev.starts_with("#[") || prev.starts_with("@") {
            idx -= 1;
        } else {
            break;
        }
    }

    if idx == 0 {
        return idx;
    }

    // Skip blank lines between doc comment and symbol/decorators.
    // Many JS/TS codebases put a blank line after `*/` before the symbol.
    let mut peek = idx;
    while peek > 0 && lines[peek - 1].trim().is_empty() {
        peek -= 1;
    }

    if peek == 0 {
        return idx;
    }

    let prev_trimmed = lines[peek - 1].trim();

    // Rust-style line doc comments (///).
    // Note: //! (inner doc comments) document the containing module, not the next item.
    if prev_trimmed.starts_with("///") {
        peek -= 1;
        while peek > 0 {
            let above = lines[peek - 1].trim();
            if above.starts_with("///") {
                peek -= 1;
            } else {
                break;
            }
        }
        return peek;
    }

    // Go and C doc comments (plain // preceding a declaration)
    if matches!(lang, Some(Lang::Go | Lang::C)) && prev_trimmed.starts_with("//") {
        peek -= 1;
        while peek > 0 {
            let above = lines[peek - 1].trim();
            if above.starts_with("//") {
                peek -= 1;
            } else {
                break;
            }
        }
        return peek;
    }

    // Block doc comments (/** ... */ — JSDoc/Doxygen or Rust; /* ... */ for C)
    if prev_trimmed.ends_with("*/") {
        let mut scan = peek - 1;
        loop {
            let line = lines[scan].trim();
            if line.starts_with("/**") {
                return scan;
            }
            if line.starts_with("/*") {
                // In C, plain /* ... */ comments are the standard doc comment style.
                // In other languages, only /** ... */ counts as a doc comment.
                if lang == Some(Lang::C) {
                    return scan;
                }
                break;
            }
            if scan == 0 {
                break;
            }
            scan -= 1;
        }
    }

    // Python-style # comments (but not Rust #[attributes], already handled above).
    // Only match for Python — C preprocessor directives (#include, #define) also
    // start with # but are not doc comments.
    if lang == Some(Lang::Python) && prev_trimmed.starts_with('#') && !prev_trimmed.starts_with("#[") {
        peek -= 1;
        while peek > 0 {
            let above = lines[peek - 1].trim();
            if above.starts_with('#') && !above.starts_with("#[") {
                peek -= 1;
            } else {
                break;
            }
        }
        return peek;
    }

    // No doc comment found — return idx (includes decorators but not blank lines above)
    idx
}

/// Trim pure block-comment delimiter lines from the doc comment range.
///
/// Block comments like `/** ... */` have delimiter-only lines (`/**`, ` */`)
/// that carry no information. Showing these as Doc(1) wastes budget on noise
/// instead of actual description text. This trims leading openers and trailing
/// closers so Doc(1) shows the first real content line.
///
/// Returns `(trimmed_start, trimmed_end)` where the range is `start..end`.
fn trim_doc_delimiters(lines: &[&str], doc_start: usize, sym_line_0: usize) -> (usize, usize) {
    if doc_start >= sym_line_0 {
        return (sym_line_0, sym_line_0);
    }

    let mut start = doc_start;
    let mut end = sym_line_0;

    // Trim leading block-comment opener: `/**`, `/*`
    let first = lines[start].trim();
    if first == "/**" || first == "/*" {
        start += 1;
    }

    // Trim trailing blank lines and block-comment closer (`*/`)
    while end > start && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    if end > start && lines[end - 1].trim() == "*/" {
        end -= 1;
    }

    // If trimming eliminated all lines, report no doc comment
    if start >= end {
        (sym_line_0, sym_line_0)
    } else {
        (start, end)
    }
}

/// For Python files, find the end of a docstring following a symbol definition.
/// Returns the end position (0-indexed, exclusive) of the docstring lines.
/// Returns `sym_line_0 + 1` if no docstring is found.
pub(crate) fn docstring_end(lines: &[&str], sym_line_0: usize) -> usize {
    let mut idx = sym_line_0 + 1;
    // Skip blank lines
    while idx < lines.len() && lines[idx].trim().is_empty() {
        idx += 1;
    }
    if idx >= lines.len() {
        return sym_line_0 + 1;
    }
    let trimmed = lines[idx].trim();
    // Detect triple-quote opener (""" or '''), with optional Python string prefix
    // Valid prefixes: r, u, f, b, rb, br, rf, fr (case-insensitive)
    let prefix_len = {
        let lower: String = trimmed.chars().take(3).flat_map(|c| c.to_lowercase()).collect();
        if lower.starts_with("rb")
            || lower.starts_with("br")
            || lower.starts_with("rf")
            || lower.starts_with("fr")
        {
            2
        } else if lower.starts_with('r')
            || lower.starts_with('u')
            || lower.starts_with('f')
            || lower.starts_with('b')
        {
            1
        } else {
            0
        }
    };
    let after_prefix = &trimmed[prefix_len..];
    let (quote, open_len) = if after_prefix.starts_with("\"\"\"") {
        ("\"\"\"", prefix_len + 3)
    } else if after_prefix.starts_with("'''") {
        ("'''", prefix_len + 3)
    } else {
        return sym_line_0 + 1;
    };
    // Check if docstring closes on the same line (after the opening quotes)
    let after_open = &trimmed[open_len..];
    if after_open.contains(quote) {
        return idx + 1;
    }
    // Multi-line: scan until closing triple quote
    idx += 1;
    while idx < lines.len() {
        if lines[idx].contains(quote) {
            return idx + 1;
        }
        idx += 1;
    }
    sym_line_0 + 1 // No closing quote found, skip
}

// ---------------------------------------------------------------------------
// Markdown helpers
// ---------------------------------------------------------------------------

/// Check if a markdown line is leading "noise" that should be skipped at
/// the start of section bodies. Matches:
/// - Blank or whitespace-only lines
/// - Markdown images: `![alt](url)` (badges/shields)
/// - Linked images: `[![alt](url)](url)` (clickable badges)
/// - Link reference definitions: `[label]: http...`
/// - HTML tags: `<div>`, `<p align=...>`, `<img .../>`, `</div>`, etc.
/// - HTML comments: `<!-- ... -->`
/// - Horizontal rules: `---`, `***`, `___`, `* * *`, etc.
///
/// Only used to skip contiguous noise at the start of a section body,
/// so mid-section images and links are still rendered normally.
pub(crate) fn is_markdown_leading_noise(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }
    // Markdown images (badges): ![alt](url) or [![alt](url)](url)
    if (trimmed.starts_with("![") || trimmed.starts_with("[![")) && trimmed.ends_with(')') {
        return true;
    }
    // Link reference definitions: [label]: URL
    if trimmed.starts_with('[')
        && let Some(pos) = trimmed.find("]: ")
    {
        let after = trimmed[pos + 3..].trim();
        if after.starts_with("http") || after.starts_with('/') || after.starts_with('#') {
            return true;
        }
    }
    // HTML tags: <div>, </div>, <p align=...>, <img .../>, <br>, <br/>, etc.
    // Also matches HTML comments: <!-- ... -->
    if let Some(rest) = trimmed.strip_prefix('<')
        && (rest.starts_with('/')
            || rest.starts_with('!')
            || rest.as_bytes().first().is_some_and(|b| b.is_ascii_alphabetic()))
    {
        return true;
    }
    // Horizontal rules: 3+ of the same character (-, *, _) with optional spaces
    if is_horizontal_rule(trimmed) {
        return true;
    }
    false
}

/// Check if a trimmed line is a markdown horizontal rule (thematic break).
/// Matches 3+ of the same character (`-`, `*`, or `_`), optionally
/// separated by spaces.
fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < 3 {
        return false;
    }
    let mut rule_char = None;
    let mut count = 0;
    for b in trimmed.bytes() {
        match b {
            b'-' | b'*' | b'_' => {
                if let Some(rc) = rule_char {
                    if b != rc {
                        return false;
                    }
                } else {
                    rule_char = Some(b);
                }
                count += 1;
            }
            b' ' => {}
            _ => return false,
        }
    }
    count >= 3
}

/// Strip trailing badge/image markdown from a markdown heading line.
///
/// Many README files include CI/coverage/version badges inline in the top-level
/// heading: `# project [![build](url)](url) [![version](url)](url)`.
/// These badge URLs waste token budget while adding no useful information for
/// codebase understanding.
///
/// Returns the prefix of the line before the first trailing ` [![` pattern.
/// Only matches the linked-image pattern (`[![`) which is specifically used
/// for badges; plain `![` images in headings are left intact.
pub(crate) fn strip_heading_badges(line: &str) -> &str {
    // Look for ` [![` — linked image (badge) preceded by a space.
    // Only strip if there's meaningful heading text before the badge.
    if let Some(pos) = line.find(" [![") {
        // Ensure there's at least one non-whitespace char of heading text
        // before the badge (skip the `# ` prefix).
        let before = line[..pos].trim();
        if !before.is_empty() && before != "#" {
            return line[..pos].trim_end();
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_c_line_comment_skips_urls() {
        assert_eq!(strip_c_line_comment("code // comment"), "code");
        assert_eq!(strip_c_line_comment("no comment"), "no comment");
        assert_eq!(
            strip_c_line_comment(r#"function fetch(url = "https://example.com") {"#),
            r#"function fetch(url = "https://example.com") {"#,
        );
        assert_eq!(
            strip_c_line_comment(r#"const API = "http://localhost:3000"; // dev"#),
            r#"const API = "http://localhost:3000";"#,
        );
    }

    #[test]
    fn signature_end_detects_brace_after_url() {
        let source = r#"function fetch(url: string = "https://example.com") {
    return axios.get(url);
}"#;
        let lines: Vec<&str> = source.lines().collect();
        let sym = parse::Symbol {
            kind: parse::SymbolKind::Function,
            name: "fetch".to_string(),
            is_public: true,
            is_first_party: false,
            line: 1,
            end_line: 3,
            sig_end_line: None, // test text fallback
            doc_start_line: None,
            is_trait_impl: false,
            is_reexport: false,
        };
        assert_eq!(signature_end_line(&lines, &sym, Some(Lang::JsTs)), 0);
    }

    #[test]
    fn strip_heading_badges_removes_trailing_badges() {
        // Linked images (badges) after heading text
        assert_eq!(
            strip_heading_badges("# color [![build](url)](link)"),
            "# color",
        );
        assert_eq!(
            strip_heading_badges("# Structs [![GoDoc](url)](link) [![Build](url)](link)"),
            "# Structs",
        );
        // Name-only text (without # prefix)
        assert_eq!(
            strip_heading_badges("color [![build](url)](link)"),
            "color",
        );
        // No badges — unchanged
        assert_eq!(strip_heading_badges("# Introduction"), "# Introduction");
        assert_eq!(strip_heading_badges("## API Reference"), "## API Reference");
        // Badge without preceding text — unchanged (just `#` prefix)
        assert_eq!(
            strip_heading_badges("# [![badge](url)](link)"),
            "# [![badge](url)](link)",
        );
        // Plain image (not linked) — not stripped (only [![ is targeted)
        assert_eq!(
            strip_heading_badges("# Project ![icon](url)"),
            "# Project ![icon](url)",
        );
    }
}
