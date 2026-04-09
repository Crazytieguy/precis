use std::sync::LazyLock;

use tiktoken_rs::CoreBPE;

use crate::parse;

/// Shared BPE tokenizer instance (o200k_base, used by GPT-4o / Claude-class models).
static BPE: LazyLock<CoreBPE> = LazyLock::new(|| tiktoken_rs::o200k_base().unwrap());

// ---------------------------------------------------------------------------
// Output format primitives
// ---------------------------------------------------------------------------

/// Format a single source line with its line number.
/// Line numbers are 1-indexed; `line_idx_0` is the 0-based index.
pub(crate) fn fmt_line(line_idx_0: usize, line: &str) -> String {
    format!("{:>6}→{}\n", line_idx_0 + 1, line)
}

/// Truncation marker indented to match the content being truncated.
/// `last_line` is the source line just before the truncation point —
/// the marker inherits its leading whitespace so nested content reads
/// naturally (e.g. struct fields get an indented `…`).
pub(crate) fn truncation_marker(last_line: &str) -> String {
    let indent_len = last_line.len() - last_line.trim_start().len();
    let indent = &last_line[..indent_len];
    format!("      →{}…\n", indent)
}

/// Count BPE tokens in text using the o200k_base tokenizer.
pub fn count_tokens(text: &str) -> usize {
    BPE.encode_with_special_tokens(text).len()
}

// ---------------------------------------------------------------------------
// Symbol content helpers (used by both schedule and format)
// ---------------------------------------------------------------------------

/// Format a symbol line truncated at the symbol name.
pub(crate) fn format_symbol_name(sym: &parse::Symbol, lines: &[&str]) -> String {
    let source_line = lines.get(sym.line - 1).copied().unwrap_or("");
    let trimmed = source_line.trim_start();
    let indent = &source_line[..source_line.len() - trimmed.len()];
    let name_prefix = match find_word(&sym.name, trimmed) {
        Some(pos) => format!("{}{}", indent, &trimmed[..pos + sym.name.len()]),
        None => {
            if sym.kind == parse::SymbolKind::Import {
                // Multi-line import: the module path is on a later line,
                // so show the source first line (e.g. `import type {`) to
                // preserve the import keyword context.
                format!("{}{}", indent, trimmed)
            } else {
                sym.name.clone()
            }
        }
    };
    format!("{:>6}→{}", sym.line, name_prefix)
}

/// Find `needle` in `haystack` at a word boundary (not inside another identifier).
/// Prefers matches outside parentheses to handle Go methods where the receiver
/// type name matches the method name (e.g. `func (f *Field) Field(...)`).
fn find_word(needle: &str, haystack: &str) -> Option<usize> {
    let mut start = 0;
    let mut first_match = None;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let before_ok = abs == 0
            || (!haystack.as_bytes()[abs - 1].is_ascii_alphanumeric()
                && haystack.as_bytes()[abs - 1] != b'_');
        let end = abs + needle.len();
        let after_ok = end == haystack.len()
            || (!haystack.as_bytes()[end].is_ascii_alphanumeric()
                && haystack.as_bytes()[end] != b'_');
        if before_ok && after_ok {
            if first_match.is_none() {
                first_match = Some(abs);
            }
            // Prefer match outside parentheses (paren depth == 0)
            let depth: i32 = haystack[..abs]
                .bytes()
                .fold(0, |d, b| match b {
                    b'(' => d + 1,
                    b')' => d - 1,
                    _ => d,
                });
            if depth == 0 {
                return Some(abs);
            }
        }
        start = abs + 1;
    }
    first_match
}
