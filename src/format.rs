use std::path::Path;

use crate::{parse, walk};

/// Format a single symbol as a line of output (without trailing newline).
///
/// Extracts a prefix from the actual source line (up to and including the symbol name)
/// to satisfy the substring constraint: output lines are substrings of source lines.
fn format_symbol(sym: &parse::Symbol, source: &str) -> String {
    let source_line = source.lines().nth(sym.line - 1).unwrap_or("");
    let trimmed = source_line.trim_start();
    let prefix = match trimmed.find(&sym.name) {
        Some(pos) => &trimmed[..pos + sym.name.len()],
        None => &sym.name,
    };
    format!("  {} :{}", prefix, sym.line)
}

/// Format all symbols from a single file, with the file path header.
pub fn format_file_symbols(path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let symbols = parse::extract_symbols(path, source);
    if symbols.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push_str(&format!("{}:\n", relative.display()));
    for sym in &symbols {
        out.push_str(&format_symbol(sym, source));
        out.push('\n');
    }
    out
}

/// Format all source files in a directory.
pub fn format_directory(root: &Path) -> String {
    let files = walk::discover_source_files(root);
    let mut out = String::new();
    for file in &files {
        let source = match std::fs::read_to_string(file) {
            Ok(s) => s,
            Err(_) => continue,
        };
        out.push_str(&format_file_symbols(file, root, &source));
    }
    out
}
