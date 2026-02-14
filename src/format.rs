use std::path::{Path, PathBuf};

use crate::parse;

/// Maximum granularity level.
///
/// Levels:
/// 0 - File paths only
/// 1 - Symbol lines, truncated to symbol name
/// 2 - Symbol lines, full source line (signature)
/// 3 - Symbol lines with preceding doc comments
/// 4 - Like level 3, but type definition bodies (struct/enum/trait/interface/class) shown in full
/// 5 - Full source (all lines)
pub const MAX_LEVEL: u8 = 5;

/// Render a single file at the given granularity level.
pub fn render_file(level: u8, path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    match level {
        0 => format!("{}\n", relative.display()),
        1 => render_symbols(path, root, source, true),
        2 => render_symbols(path, root, source, false),
        3 => render_symbols_with_docs(path, root, source, false),
        4 => render_symbols_with_docs(path, root, source, true),
        _ => render_full_source(path, root, source),
    }
}

/// Count whitespace-delimited words in text.
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Find the highest level where `cost(level) <= budget`.
/// Uses binary search over `0..=MAX_LEVEL`.
pub fn search_level(budget: usize, cost: impl Fn(u8) -> usize) -> u8 {
    let mut low: u8 = 0;
    let mut high: u8 = MAX_LEVEL;
    while low < high {
        let mid = (low + high).div_ceil(2);
        if cost(mid) <= budget {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

/// Pre-read source files to avoid repeated disk I/O.
pub fn read_sources(files: &[PathBuf]) -> Vec<Option<String>> {
    files
        .iter()
        .map(|f| std::fs::read_to_string(f).ok())
        .collect()
}

/// Find the highest granularity level whose output fits within the word budget.
/// Accepts pre-discovered files and pre-read sources to avoid redundant I/O.
pub fn budget_level(
    budget: usize,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> u8 {
    search_level(budget, |level| {
        count_words(&render_files(level, root, files, sources))
    })
}

/// Find the highest granularity level whose output for a single file fits within the word budget.
pub fn budget_level_file(budget: usize, path: &Path, root: &Path, source: &str) -> u8 {
    search_level(budget, |level| {
        count_words(&render_file(level, path, root, source))
    })
}

/// Render pre-discovered source files at the given granularity level.
/// Uses pre-read sources to avoid redundant disk I/O.
pub fn render_files(
    level: u8,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> String {
    let mut out = String::new();
    for (file, source) in files.iter().zip(sources) {
        if level == 0 {
            out.push_str(&render_file(0, file, root, ""));
        } else if let Some(s) = source {
            out.push_str(&render_file(level, file, root, s));
        }
    }
    out
}

/// Render symbol lines for a file. If `truncate` is true (level 1), truncates
/// each line at the symbol name. If false (level 2), shows the full source line.
fn render_symbols(path: &Path, root: &Path, source: &str, truncate: bool) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let symbols = parse::extract_symbols(path, source);
    let mut out = String::new();
    out.push_str(&format!("{}\n", relative.display()));
    if symbols.is_empty() {
        return out;
    }
    let lines: Vec<&str> = source.lines().collect();
    for sym in &symbols {
        if truncate {
            out.push_str(&format_symbol_name(sym, &lines));
        } else {
            out.push_str(&format_symbol_line(sym, &lines));
        }
        out.push('\n');
    }
    out
}

/// Format a symbol line truncated at the symbol name (level 1).
fn format_symbol_name(sym: &parse::Symbol, lines: &[&str]) -> String {
    let source_line = lines.get(sym.line - 1).copied().unwrap_or("");
    let trimmed = source_line.trim_start();
    let indent = &source_line[..source_line.len() - trimmed.len()];
    let name_prefix = match find_word(&sym.name, trimmed) {
        Some(pos) => format!("{}{}", indent, &trimmed[..pos + sym.name.len()]),
        None => sym.name.clone(),
    };
    format!("{:>6}→{}", sym.line, name_prefix)
}

/// Format a symbol line showing the full source line (level 2).
fn format_symbol_line(sym: &parse::Symbol, lines: &[&str]) -> String {
    let source_line = lines.get(sym.line - 1).copied().unwrap_or("");
    format!("{:>6}→{}", sym.line, source_line)
}

/// Render all lines of a file with line numbers (level 5).
fn render_full_source(path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let mut out = String::new();
    out.push_str(&format!("{}\n", relative.display()));
    for (i, line) in source.lines().enumerate() {
        out.push_str(&format!("{:>6}→{}\n", i + 1, line));
    }
    out
}

/// Render symbol lines with preceding doc comments (level 3).
/// If `expand_types` is true (level 4), show full bodies for struct/enum/trait/interface.
fn render_symbols_with_docs(path: &Path, root: &Path, source: &str, expand_types: bool) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let symbols = parse::extract_symbols(path, source);
    let lines: Vec<&str> = source.lines().collect();
    let is_python = path.extension().and_then(|e| e.to_str()) == Some("py");
    let mut out = String::new();
    out.push_str(&format!("{}\n", relative.display()));
    if symbols.is_empty() {
        return out;
    }
    // Track which lines have been emitted to avoid duplication when type bodies
    // overlap with nested symbols (e.g. trait methods inside a trait body).
    let mut emitted_up_to: usize = 0; // 0-indexed, exclusive
    for sym in &symbols {
        let sym_line_0 = sym.line - 1;
        let doc_start = doc_comment_start(&lines, sym_line_0);
        let should_expand = expand_types && is_type_definition(sym.kind);
        if should_expand {
            let body_end = sym.end_line; // 1-indexed, inclusive
            // Start from doc_start or emitted_up_to, whichever is later
            let start = doc_start.max(emitted_up_to);
            let end = body_end; // 1-indexed inclusive → 0-indexed exclusive
            for (i, line) in lines.iter().enumerate().take(end).skip(start) {
                out.push_str(&format!("{:>6}→{}\n", i + 1, line));
            }
            emitted_up_to = emitted_up_to.max(end);
        } else if sym_line_0 >= emitted_up_to {
            // Non-expanded symbol: show doc comments + signature line
            let start = doc_start.max(emitted_up_to);
            for (i, line) in lines.iter().enumerate().take(sym_line_0).skip(start) {
                out.push_str(&format!("{:>6}→{}\n", i + 1, line));
            }
            out.push_str(&format_symbol_line(sym, &lines));
            out.push('\n');
            // Python docstrings: include triple-quoted string after the symbol line
            if is_python {
                let ds_end = docstring_end(&lines, sym_line_0);
                for (i, line) in lines.iter().enumerate().take(ds_end).skip(sym_line_0 + 1) {
                    out.push_str(&format!("{:>6}→{}\n", i + 1, line));
                }
                emitted_up_to = emitted_up_to.max(ds_end);
            }
        }
    }
    out
}

/// Whether a symbol kind represents a type definition whose body should be expanded.
fn is_type_definition(kind: parse::SymbolKind) -> bool {
    matches!(
        kind,
        parse::SymbolKind::Struct
            | parse::SymbolKind::Enum
            | parse::SymbolKind::Trait
            | parse::SymbolKind::Interface
            | parse::SymbolKind::Class
    )
}

/// Find the first line (0-indexed) of the doc comment block preceding a symbol.
/// Returns the symbol's own line index if there's no doc comment.
fn doc_comment_start(lines: &[&str], symbol_line_0: usize) -> usize {
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

    let prev_trimmed = lines[idx - 1].trim();

    // Rust-style line doc comments (/// or //!)
    if prev_trimmed.starts_with("///") || prev_trimmed.starts_with("//!") {
        idx -= 1;
        while idx > 0 {
            let above = lines[idx - 1].trim();
            if above.starts_with("///") || above.starts_with("//!") {
                idx -= 1;
            } else {
                break;
            }
        }
        return idx;
    }

    // Block doc comments (/** ... */ — JSDoc or Rust)
    if prev_trimmed.ends_with("*/") {
        let mut scan = idx - 1;
        loop {
            let line = lines[scan].trim();
            if line.starts_with("/**") {
                return scan;
            }
            if line.starts_with("/*") {
                // Regular block comment, not a doc comment
                break;
            }
            if scan == 0 {
                break;
            }
            scan -= 1;
        }
    }

    // Python-style # comments (but not Rust #[attributes], already handled above)
    if prev_trimmed.starts_with('#') && !prev_trimmed.starts_with("#[") {
        idx -= 1;
        while idx > 0 {
            let above = lines[idx - 1].trim();
            if above.starts_with('#') && !above.starts_with("#[") {
                idx -= 1;
            } else {
                break;
            }
        }
        return idx;
    }

    // Return idx (which may be < symbol_line_0 if decorators/attributes were found)
    idx
}

/// For Python files, find the end of a docstring following a symbol definition.
/// Returns the end position (0-indexed, exclusive) of the docstring lines.
/// Returns `sym_line_0 + 1` if no docstring is found.
fn docstring_end(lines: &[&str], sym_line_0: usize) -> usize {
    let mut idx = sym_line_0 + 1;
    // Skip blank lines
    while idx < lines.len() && lines[idx].trim().is_empty() {
        idx += 1;
    }
    if idx >= lines.len() {
        return sym_line_0 + 1;
    }
    let trimmed = lines[idx].trim();
    // Detect triple-quote opener (""" or ''', optionally with r prefix)
    let (quote, open_len) = if trimmed.starts_with("\"\"\"") {
        ("\"\"\"", 3)
    } else if trimmed.starts_with("'''") {
        ("'''", 3)
    } else if trimmed.starts_with("r\"\"\"") {
        ("\"\"\"", 4)
    } else if trimmed.starts_with("r'''") {
        ("'''", 4)
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

/// Find `needle` in `haystack` at a word boundary (not inside another identifier).
fn find_word(needle: &str, haystack: &str) -> Option<usize> {
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let before_ok = abs == 0
            || !haystack.as_bytes()[abs - 1].is_ascii_alphanumeric()
                && haystack.as_bytes()[abs - 1] != b'_';
        let end = abs + needle.len();
        let after_ok = end == haystack.len()
            || !haystack.as_bytes()[end].is_ascii_alphanumeric()
                && haystack.as_bytes()[end] != b'_';
        if before_ok && after_ok {
            return Some(abs);
        }
        start = abs + 1;
    }
    None
}
