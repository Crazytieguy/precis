use std::path::Path;

use crate::{parse, walk};

/// Format a single symbol as a line of output (without trailing newline).
fn format_symbol(sym: &parse::Symbol) -> String {
    let vis = if sym.is_public { "pub " } else { "" };
    format!("  {vis}{} {}", sym.kind, sym.name)
}

/// Format all symbols from a single file, with the file path header.
pub fn format_file_symbols(path: &Path, root: &Path, source: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let symbols = parse::extract_symbols(path, source);
    let mut out = String::new();
    if symbols.is_empty() {
        out.push_str(&format!("{}\n", relative.display()));
    } else {
        out.push_str(&format!("{}:\n", relative.display()));
        for sym in &symbols {
            out.push_str(&format_symbol(sym));
            out.push('\n');
        }
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
