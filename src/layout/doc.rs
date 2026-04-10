use crate::Lang;

/// Find the first line (0-indexed) of the doc comment block preceding a symbol.
/// Returns the symbol's own line index if there's no doc comment.
pub(super) fn doc_comment_start(lines: &[&str], symbol_line_0: usize, lang: Option<Lang>) -> usize {
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
pub(super) fn trim_doc_delimiters(lines: &[&str], doc_start: usize, sym_line_0: usize) -> (usize, usize) {
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
pub(super) fn docstring_end(lines: &[&str], sym_line_0: usize) -> usize {
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
pub(super) fn skip_doc_leading_noise(lines: &[&str], start: usize, end: usize, lang: Option<Lang>) -> Option<usize> {
    (start..end).find(|&i| !super::noise::is_markdown_leading_noise(strip_doc_line_prefix(lines[i], lang)))
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
pub(super) fn strip_doc_line_prefix(line: &str, lang: Option<Lang>) -> &str {
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
    use super::super::noise::is_markdown_leading_noise;

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
