use std::path::Path;

use crate::{parse, walk};

/// Maximum granularity level.
///
/// Levels:
/// 0 - File paths only
/// 1 - Symbol lines, truncated to symbol name
/// 2 - Symbol lines, full source line (signature)
/// 3 - Full source (all lines)
pub const MAX_LEVEL: u8 = 3;

/// Render a single file at the given granularity level.
pub fn render_file(level: u8, path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    match level {
        0 => format!("{}\n", relative.display()),
        1 => render_symbols(path, root, source, true),
        2 => render_symbols(path, root, source, false),
        _ => render_full_source(path, root, source),
    }
}

/// Count whitespace-delimited words in text.
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Find the highest granularity level whose output fits within the word budget.
/// Uses binary search over `0..=MAX_LEVEL`.
pub fn budget_level(budget: usize, root: &Path) -> u8 {
    let mut low: u8 = 0;
    let mut high: u8 = MAX_LEVEL;
    while low < high {
        let mid = (low + high).div_ceil(2);
        let output = render_directory(mid, root);
        if count_words(&output) <= budget {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

/// Find the highest granularity level whose output for a single file fits within the word budget.
pub fn budget_level_file(budget: usize, path: &Path, root: &Path, source: &str) -> u8 {
    let mut low: u8 = 0;
    let mut high: u8 = MAX_LEVEL;
    while low < high {
        let mid = (low + high).div_ceil(2);
        let output = render_file(mid, path, root, source);
        if count_words(&output) <= budget {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

/// Render all source files in a directory at the given granularity level.
pub fn render_directory(level: u8, root: &Path) -> String {
    let files = walk::discover_source_files(root);
    let mut out = String::new();
    for file in &files {
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
    for sym in &symbols {
        if truncate {
            out.push_str(&format_symbol_name(sym, source));
        } else {
            out.push_str(&format_symbol_line(sym, source));
        }
        out.push('\n');
    }
    out
}

/// Format a symbol line truncated at the symbol name (level 1).
fn format_symbol_name(sym: &parse::Symbol, source: &str) -> String {
    let source_line = source.lines().nth(sym.line - 1).unwrap_or("");
    let trimmed = source_line.trim_start();
    let indent = &source_line[..source_line.len() - trimmed.len()];
    let name_prefix = match find_word(&sym.name, trimmed) {
        Some(pos) => format!("{}{}", indent, &trimmed[..pos + sym.name.len()]),
        None => sym.name.clone(),
    };
    format!("{:>6}→{}", sym.line, name_prefix)
}

/// Format a symbol line showing the full source line (level 2).
fn format_symbol_line(sym: &parse::Symbol, source: &str) -> String {
    let source_line = source.lines().nth(sym.line - 1).unwrap_or("");
    format!("{:>6}→{}", sym.line, source_line)
}

/// Render all lines of a file with line numbers (level 3).
fn render_full_source(path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let mut out = String::new();
    out.push_str(&format!("{}\n", relative.display()));
    for (i, line) in source.lines().enumerate() {
        out.push_str(&format!("{:>6}→{}\n", i + 1, line));
    }
    out
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
