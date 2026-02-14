use std::path::{Path, PathBuf};

use crate::{parse, walk};

/// Maximum granularity level.
///
/// Levels:
/// 0 - File paths only
/// 1 - Symbol lines, truncated to symbol name
/// 2 - Symbol lines, full source line (signature)
/// 3 - Symbol lines with preceding doc comments
/// 4 - Full source (all lines)
pub const MAX_LEVEL: u8 = 4;

/// Render a single file at the given granularity level.
pub fn render_file(level: u8, path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    match level {
        0 => format!("{}\n", relative.display()),
        1 => render_symbols(path, root, source, true),
        2 => render_symbols(path, root, source, false),
        3 => render_symbols_with_docs(path, root, source),
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

/// Find the highest granularity level whose output fits within the word budget.
pub fn budget_level(budget: usize, root: &Path) -> u8 {
    let files = walk::discover_source_files(root);
    search_level(budget, |level| count_words(&render_files(level, root, &files)))
}

/// Find the highest granularity level whose output for a single file fits within the word budget.
pub fn budget_level_file(budget: usize, path: &Path, root: &Path, source: &str) -> u8 {
    search_level(budget, |level| count_words(&render_file(level, path, root, source)))
}

/// Render all source files in a directory at the given granularity level.
pub fn render_directory(level: u8, root: &Path) -> String {
    let files = walk::discover_source_files(root);
    render_files(level, root, &files)
}

/// Render pre-discovered source files at the given granularity level.
pub fn render_files(level: u8, root: &Path, files: &[PathBuf]) -> String {
    let mut out = String::new();
    for file in files {
        if level == 0 {
            let relative = file.strip_prefix(root).unwrap_or(file);
            out.push_str(&format!("{}\n", relative.display()));
            continue;
        }
        let source = match std::fs::read_to_string(file) {
            Ok(s) => s,
            Err(_) => continue,
        };
        out.push_str(&render_file(level, file, root, &source));
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

/// Render all lines of a file with line numbers (level 4).
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
fn render_symbols_with_docs(path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let symbols = parse::extract_symbols(path, source);
    let lines: Vec<&str> = source.lines().collect();
    let mut out = String::new();
    out.push_str(&format!("{}\n", relative.display()));
    if symbols.is_empty() {
        return out;
    }
    for sym in &symbols {
        let sym_line_0 = sym.line - 1;
        let doc_start = doc_comment_start(&lines, sym_line_0);
        for (i, line) in lines.iter().enumerate().take(sym_line_0).skip(doc_start) {
            out.push_str(&format!("{:>6}→{}\n", i + 1, line));
        }
        out.push_str(&format_symbol_line(sym, &lines));
        out.push('\n');
    }
    out
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
        return symbol_line_0;
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
                return symbol_line_0;
            }
            if scan == 0 {
                break;
            }
            scan -= 1;
        }
    }

    symbol_line_0
}

/// Find `needle` in `haystack` at a word boundary (not inside another identifier).
fn find_word(needle: &str, haystack: &str) -> Option<usize> {
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let before_ok = abs == 0
            || !haystack.as_bytes()[abs - 1]
                .is_ascii_alphanumeric()
                && haystack.as_bytes()[abs - 1] != b'_';
        let end = abs + needle.len();
        let after_ok = end == haystack.len()
            || !haystack.as_bytes()[end]
                .is_ascii_alphanumeric()
                && haystack.as_bytes()[end] != b'_';
        if before_ok && after_ok {
            return Some(abs);
        }
        start = abs + 1;
    }
    None
}
