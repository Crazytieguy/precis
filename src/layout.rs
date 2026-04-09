use crate::parse;
use crate::Lang;

// ---------------------------------------------------------------------------
// Symbol layout (shared between scheduler and renderer)
// ---------------------------------------------------------------------------

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
        .unwrap_or_else(|| doc_comment_start(lines, raw_sym_line_0, lang));

    // Trim pure block-comment delimiters from doc range so Doc(1) shows
    // actual content instead of a bare `/**` or `/*`.
    let (doc_start, doc_end) = trim_doc_delimiters(lines, raw_doc_start, raw_sym_line_0);

    // For ModuleDoc: advance past leading noise (badges, blank doc lines, HTML,
    // link reference definitions) to find the first meaningful line, which
    // becomes the "name" shown at the Names stage.
    let sym_line_0 = if sym.kind == parse::SymbolKind::ModuleDoc {
        let end = sym.end_line.min(lines.len());
        // Fallback: if all lines are noise, keep the original line so the
        // symbol still renders something rather than vanishing.
        skip_doc_leading_noise(lines, raw_sym_line_0, end, lang).unwrap_or(raw_sym_line_0)
    } else {
        raw_sym_line_0
    };

    let sig_end = if sym.kind == parse::SymbolKind::ModuleDoc {
        sym_line_0
    } else {
        signature_end_line(lines, sym, lang)
    };

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
            && is_markdown_leading_noise(lines[content_start])
        {
            content_start += 1;
        }
        // Trim trailing noise from config section bodies (closing delimiters, blank lines)
        let mut section_end = next_heading_line;
        if is_config {
            while section_end > content_start
                && is_config_trailing_noise(lines[section_end - 1])
            {
                section_end -= 1;
            }
        }
        (content_start, section_end)
    } else if sym.kind == parse::SymbolKind::ModuleDoc {
        let mut body_end = sym.end_line.min(lines.len());
        let content_start = skip_doc_leading_noise(lines, sym_line_0 + 1, body_end, lang)
            .unwrap_or(body_end);
        // Trim trailing delimiter-only lines and blank padding (`*/`, `"""`)
        while body_end > content_start {
            let content = strip_doc_line_prefix(lines[body_end - 1], lang);
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
/// Strip trailing `# comment` from a Python line, respecting strings.
/// Skips `#` characters inside single- or double-quoted strings to avoid
/// incorrectly stripping hex colors like `"#FF0000"` or format strings.
fn strip_python_line_comment(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut in_single = false;
    let mut in_double = false;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'\'' if !in_double => in_single = !in_single,
            b'"' if !in_single => in_double = !in_double,
            b'#' if !in_single && !in_double => return s[..i].trim_end(),
            _ => {}
        }
    }
    s
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
    // Lua doc comments (-- preceding a declaration)
    let is_line_comment = matches!(lang, Some(Lang::Go | Lang::C)) && prev_trimmed.starts_with("//")
        || matches!(lang, Some(Lang::Lua)) && prev_trimmed.starts_with("--");
    if is_line_comment {
        let comment_prefix = if matches!(lang, Some(Lang::Lua)) { "--" } else { "//" };
        peek -= 1;
        while peek > 0 {
            let above = lines[peek - 1].trim();
            if above.starts_with(comment_prefix) {
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
                // In C/C++, plain /* ... */ comments are the standard doc comment style.
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
    let prefix_len = python_string_prefix_len(trimmed);
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
    // Markdown badge images: [![alt](url)](url) (linked images are always badges)
    if trimmed.starts_with("[![") && trimmed.ends_with(')') {
        return true;
    }
    // Standalone images: ![alt](url) — only skip if alt text is short (badges).
    // Long alt text indicates a screenshot or diagram that has real content value.
    if trimmed.starts_with("![") && trimmed.ends_with(')')
        && let Some(alt_end) = trimmed.find("](")
        && trimmed[2..alt_end].len() <= 30
    {
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
    // Table of contents links: `- [Title](#anchor)` or `* [Title](#anchor)`
    // These are navigational, not content — they duplicate the heading structure.
    if is_toc_link(trimmed) {
        return true;
    }
    // Block-level HTML tags and comments used for layout/badges, not content.
    // Only matches tags that are clearly structural (div, p, img, br, details,
    // table, etc.) — NOT inline tags like <em>, <strong>, <b>, <a> which wrap content.
    if let Some(rest) = trimmed.strip_prefix('<') {
        let tag_start = rest.strip_prefix('/').unwrap_or(rest);
        let tag_end = tag_start.find(|c: char| !c.is_ascii_alphabetic()).unwrap_or(tag_start.len());
        let tag = &tag_start[..tag_end];
        if rest.starts_with('!')  // HTML comments: <!-- ... -->
            || tag.eq_ignore_ascii_case("div") || tag.eq_ignore_ascii_case("p")
            || tag.eq_ignore_ascii_case("img") || tag.eq_ignore_ascii_case("br")
            || tag.eq_ignore_ascii_case("hr") || tag.eq_ignore_ascii_case("table")
            || tag.eq_ignore_ascii_case("details") || tag.eq_ignore_ascii_case("summary")
            || tag.eq_ignore_ascii_case("picture") || tag.eq_ignore_ascii_case("figure")
            || tag.eq_ignore_ascii_case("center")
        {
            return true;
        }
    }
    // Horizontal rules: 3+ of the same character (-, *, _) with optional spaces
    if is_horizontal_rule(trimmed) {
        return true;
    }
    false
}

/// Check if a line is a table-of-contents link: `- [Title](#anchor)` etc.
fn is_toc_link(trimmed: &str) -> bool {
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
        let rest = trimmed[2..].trim_start();
        return rest.starts_with('[') && rest.contains("](#") && rest.ends_with(')');
    }
    false
}

/// Check if a line is trailing noise in config format section bodies.
/// Matches blank lines and lines that are purely closing delimiters
/// (`}`, `]`, with optional trailing commas and whitespace).
fn is_config_trailing_noise(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }
    // Closing delimiters with optional trailing comma: }, ], },, ],
    matches!(trimmed, "}" | "}," | "]" | "],")
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

/// Length of the optional Python string prefix before triple quotes.
/// Valid prefixes (case-insensitive): r, u, f, b, rb, br, rf, fr.
fn python_string_prefix_len(trimmed: &str) -> usize {
    let bytes = trimmed.as_bytes();
    match bytes {
        [b'r' | b'R', b'b' | b'B', ..] | [b'b' | b'B', b'r' | b'R', ..]
        | [b'r' | b'R', b'f' | b'F', ..] | [b'f' | b'F', b'r' | b'R', ..] => 2,
        [b'r' | b'R', ..] | [b'u' | b'U', ..] | [b'f' | b'F', ..] | [b'b' | b'B', ..] => 1,
        _ => 0,
    }
}

/// Find the first non-noise line in a doc comment range.
/// Returns `None` if all lines in `start..end` are noise.
fn skip_doc_leading_noise(lines: &[&str], start: usize, end: usize, lang: Option<Lang>) -> Option<usize> {
    (start..end).find(|&i| !is_markdown_leading_noise(strip_doc_line_prefix(lines[i], lang)))
}

/// Strip doc comment line prefixes so the underlying content can be checked
/// for leading noise (badges, link references, HTML, etc.).
///
/// Handles per-line prefixes for all languages that support ModuleDoc:
/// - Rust: `//!`, `///`, `/*!`, block comment continuation `* `
/// - Go: `//`, block comment continuation `* `
/// - Java: `/**`, block comment continuation `* `
/// - Python: `"""` / `'''` (with optional string prefix: r, u, f, b, rb, br, rf, fr)
///
/// Lines that are purely delimiters (`/*`, `*/`, `/**`, `/*!`, bare `*`) return
/// empty — these are always noise. For Python, a bare `"""` or `'''` (with optional
/// prefix) returns empty; `"""content` returns `content`.
fn strip_doc_line_prefix(line: &str, lang: Option<Lang>) -> &str {
    let trimmed = line.trim_start();

    // Block comment delimiters: always noise (any language)
    if matches!(trimmed, "/*" | "/**" | "/*!" | "*/" | "*") {
        return "";
    }

    match lang {
        Some(Lang::Rust) => {
            trimmed
                .strip_prefix("//!")
                .or_else(|| trimmed.strip_prefix("///"))
                .or_else(|| trimmed.strip_prefix("/*!"))
                .or_else(|| trimmed.strip_prefix("* "))
                .map(|s| s.strip_prefix(' ').unwrap_or(s))
                .unwrap_or(trimmed)
        }
        Some(Lang::Go) => {
            trimmed
                .strip_prefix("//")
                .or_else(|| trimmed.strip_prefix("* "))
                .map(|s| s.strip_prefix(' ').unwrap_or(s))
                .unwrap_or(trimmed)
        }
        Some(Lang::Java) => {
            trimmed
                .strip_prefix("/**")
                .or_else(|| trimmed.strip_prefix("* "))
                .map(|s| s.strip_prefix(' ').unwrap_or(s))
                .unwrap_or(trimmed)
        }
        Some(Lang::Python) => {
            let after_prefix = &trimmed[python_string_prefix_len(trimmed)..];
            if let Some(rest) = after_prefix.strip_prefix("\"\"\"") {
                rest
            } else if let Some(rest) = after_prefix.strip_prefix("'''") {
                rest
            } else {
                trimmed
            }
        }
        _ => trimmed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_python_comment_respects_strings() {
        assert_eq!(strip_python_line_comment("x = 1  # comment"), "x = 1");
        assert_eq!(strip_python_line_comment("no comment"), "no comment");
        // # inside double-quoted strings should not be stripped
        assert_eq!(
            strip_python_line_comment("color = \"#FF0000\"  # hex color"),
            "color = \"#FF0000\"",
        );
        // # inside single-quoted strings should not be stripped
        assert_eq!(
            strip_python_line_comment("x = '#tag'"),
            "x = '#tag'",
        );
    }

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
            start_byte: 0,
            end_byte: 0,
            composed_prefix_lens: Vec::new(),
            layout: Default::default(),
        };
        assert_eq!(signature_end_line(&lines, &sym, Some(Lang::JsTs)), 0);
    }

    #[test]
    fn leading_noise_badge_vs_screenshot() {
        // Short alt text = badge (noise)
        assert!(is_markdown_leading_noise("![build](https://img.shields.io/badge.svg)"));
        assert!(is_markdown_leading_noise("![npm](https://badge.fury.io/js/pkg.svg)"));
        // Linked badge
        assert!(is_markdown_leading_noise("[![CI](url)](link)"));
        // Long alt text = screenshot (not noise)
        assert!(!is_markdown_leading_noise("![Screenshot showing the dashboard with live metrics](screenshot.png)"));
        // TOC links
        assert!(is_markdown_leading_noise("- [Installation](#installation)"));
        assert!(is_markdown_leading_noise("-   [Usage](#usage)"));
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

    #[test]
    fn strip_doc_prefix_rust() {
        // Inner doc comments
        assert_eq!(strip_doc_line_prefix("//! content", Some(Lang::Rust)), "content");
        assert_eq!(strip_doc_line_prefix("//!  extra space", Some(Lang::Rust)), " extra space");
        assert_eq!(strip_doc_line_prefix("//!", Some(Lang::Rust)), "");
        // Block inner doc opener
        assert_eq!(strip_doc_line_prefix("/*!", Some(Lang::Rust)), "");
        assert_eq!(strip_doc_line_prefix("/*! content", Some(Lang::Rust)), "content");
        // Block comment continuation
        assert_eq!(strip_doc_line_prefix(" * content", Some(Lang::Rust)), "content");
        assert_eq!(strip_doc_line_prefix(" */", Some(Lang::Rust)), "");
        assert_eq!(strip_doc_line_prefix(" *", Some(Lang::Rust)), "");
    }

    #[test]
    fn strip_doc_prefix_go() {
        assert_eq!(strip_doc_line_prefix("// content", Some(Lang::Go)), "content");
        assert_eq!(strip_doc_line_prefix("//", Some(Lang::Go)), "");
        // Block comment
        assert_eq!(strip_doc_line_prefix("/*", Some(Lang::Go)), "");
        assert_eq!(strip_doc_line_prefix(" * content", Some(Lang::Go)), "content");
        assert_eq!(strip_doc_line_prefix(" */", Some(Lang::Go)), "");
    }

    #[test]
    fn strip_doc_prefix_java() {
        assert_eq!(strip_doc_line_prefix("/** content", Some(Lang::Java)), "content");
        assert_eq!(strip_doc_line_prefix("/**", Some(Lang::Java)), "");
        assert_eq!(strip_doc_line_prefix(" * content", Some(Lang::Java)), "content");
        assert_eq!(strip_doc_line_prefix(" */", Some(Lang::Java)), "");
    }

    #[test]
    fn strip_doc_prefix_python() {
        // Bare triple quotes (delimiter-only)
        assert_eq!(strip_doc_line_prefix("\"\"\"", Some(Lang::Python)), "");
        assert_eq!(strip_doc_line_prefix("'''", Some(Lang::Python)), "");
        // Triple quotes with content (content preserved)
        assert_eq!(strip_doc_line_prefix("\"\"\"Module docs.", Some(Lang::Python)), "Module docs.");
        // With string prefix
        assert_eq!(strip_doc_line_prefix("r\"\"\"", Some(Lang::Python)), "");
        assert_eq!(strip_doc_line_prefix("r\"\"\"Raw docs.", Some(Lang::Python)), "Raw docs.");
        assert_eq!(strip_doc_line_prefix("rb\"\"\"", Some(Lang::Python)), "");
        // Regular content lines (no prefix to strip)
        assert_eq!(strip_doc_line_prefix("Regular content.", Some(Lang::Python)), "Regular content.");
    }

    #[test]
    fn strip_doc_prefix_noise_integration() {
        // Rust badge link ref: after stripping //!, is_markdown_leading_noise should detect it
        let content = strip_doc_line_prefix("//! [github]: https://img.shields.io/badge", Some(Lang::Rust));
        assert!(is_markdown_leading_noise(content));
        // Rust HTML: after stripping //!, is_markdown_leading_noise should detect it
        let content = strip_doc_line_prefix("//! <br>", Some(Lang::Rust));
        assert!(is_markdown_leading_noise(content));
        // Rust empty line
        let content = strip_doc_line_prefix("//!", Some(Lang::Rust));
        assert!(is_markdown_leading_noise(content));
        // Rust actual content: NOT noise
        let content = strip_doc_line_prefix("//! This library provides error handling.", Some(Lang::Rust));
        assert!(!is_markdown_leading_noise(content));
    }
}
