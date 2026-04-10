use crate::parse;
use crate::Lang;

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
pub(super) fn signature_end_line(lines: &[&str], sym: &parse::Symbol, lang: Option<Lang>) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout;

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
            layout: layout::SymbolLayout::default(),
        };
        assert_eq!(signature_end_line(&lines, &sym, Some(Lang::JsTs)), 0);
    }
}
