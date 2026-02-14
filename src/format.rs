use std::path::{Path, PathBuf};

use crate::parse;

/// Maximum granularity level.
///
/// Levels:
/// 0 - File paths only
/// 1 - Symbol lines, truncated to symbol name
/// 2 - Symbol lines, full multi-line signatures
/// 3 - Symbol lines with preceding doc comments
/// 4 - Like level 3, but type definition bodies (struct/enum/trait/interface/class) shown in full
/// 5 - Full source (all lines)
pub const MAX_LEVEL: u8 = 5;

/// Render a single file at the given granularity level.
pub fn render_file(level: u8, path: &Path, root: &Path, source: &str) -> String {
    let symbols = if matches!(level, 1..=4) {
        parse::extract_symbols(path, source)
    } else {
        vec![]
    };
    render_with_symbols(level, path, root, source, &symbols)
}

/// Render a single file at the given level using pre-extracted symbols.
/// Used by budget search to avoid redundant symbol extraction across levels.
fn render_with_symbols(
    level: u8,
    path: &Path,
    root: &Path,
    source: &str,
    symbols: &[parse::Symbol],
) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let mut out = format!("{}\n", relative.display());
    match level {
        0 => {}
        1 => render_symbols(&mut out, relative, source, symbols, true),
        2 => render_symbols(&mut out, relative, source, symbols, false),
        3 => render_symbols_with_docs(&mut out, relative, source, symbols, false),
        4 => render_symbols_with_docs(&mut out, relative, source, symbols, true),
        _ => render_full_source(&mut out, source),
    }
    out
}

/// Format a single source line with its line number.
/// Line numbers are 1-indexed; `line_idx_0` is the 0-based index.
fn fmt_line(line_idx_0: usize, line: &str) -> String {
    format!("{:>6}→{}\n", line_idx_0 + 1, line)
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

/// Pre-extract symbols from all source files.
/// Used to share symbol extraction between budget search and final rendering.
pub fn extract_all_symbols(
    files: &[PathBuf],
    sources: &[Option<String>],
) -> Vec<Vec<parse::Symbol>> {
    files
        .iter()
        .zip(sources.iter())
        .map(|(f, s)| {
            s.as_ref()
                .map(|s| parse::extract_symbols(f, s))
                .unwrap_or_default()
        })
        .collect()
}

/// Find the highest granularity level whose output fits within the word budget.
/// Accepts pre-discovered files and pre-read sources to avoid redundant I/O.
/// Returns both the level and the pre-extracted symbols (for reuse during rendering).
pub fn budget_level(
    budget: usize,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> (u8, Vec<Vec<parse::Symbol>>) {
    let all_symbols = extract_all_symbols(files, sources);

    let level = search_level(budget, |level| {
        count_words(&render_files_with_symbols(
            level,
            root,
            files,
            sources,
            &all_symbols,
        ))
    });

    (level, all_symbols)
}

/// Find the highest granularity level whose output for a single file fits within the word budget.
/// Returns both the level and the pre-extracted symbols (for reuse during rendering).
pub fn budget_level_file(
    budget: usize,
    path: &Path,
    root: &Path,
    source: &str,
) -> (u8, Vec<parse::Symbol>) {
    let symbols = parse::extract_symbols(path, source);
    let level = search_level(budget, |level| {
        count_words(&render_with_symbols(level, path, root, source, &symbols))
    });
    (level, symbols)
}

/// Render pre-discovered source files at the given granularity level.
/// Uses pre-read sources to avoid redundant disk I/O.
/// Files with unreadable source (None) fall back to path-only output at all levels
/// to maintain the monotonicity invariant.
pub fn render_files(
    level: u8,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> String {
    let mut out = String::new();
    for (file, source) in files.iter().zip(sources) {
        match source {
            Some(s) if level > 0 => {
                out.push_str(&render_file(level, file, root, s));
            }
            _ => {
                // Level 0 or unreadable file: show path only
                out.push_str(&render_file(0, file, root, ""));
            }
        }
    }
    out
}

/// Render pre-discovered source files using pre-extracted symbols.
/// Avoids redundant symbol extraction when symbols have already been computed
/// (e.g. from budget_level).
/// Files with unreadable source (None) fall back to path-only output at all levels
/// to maintain the monotonicity invariant.
pub fn render_files_with_symbols(
    level: u8,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
    all_symbols: &[Vec<parse::Symbol>],
) -> String {
    let mut out = String::new();
    for (i, (file, source)) in files.iter().zip(sources).enumerate() {
        match source {
            Some(s) if level > 0 => {
                out.push_str(&render_with_symbols(level, file, root, s, &all_symbols[i]));
            }
            _ => {
                // Level 0 or unreadable file: show path only
                out.push_str(&render_with_symbols(0, file, root, "", &[]));
            }
        }
    }
    out
}

/// Render a single file using pre-extracted symbols.
/// Avoids redundant symbol extraction when symbols have already been computed.
pub fn render_file_with_symbols(
    level: u8,
    path: &Path,
    root: &Path,
    source: &str,
    symbols: &[parse::Symbol],
) -> String {
    render_with_symbols(level, path, root, source, symbols)
}

/// Render symbol lines for a file. If `truncate` is true (level 1), truncates
/// each line at the symbol name. If false (level 2), shows full multi-line signatures.
/// `path` is relative to the project root (used for language detection).
fn render_symbols(
    out: &mut String,
    path: &Path,
    source: &str,
    symbols: &[parse::Symbol],
    truncate: bool,
) {
    if symbols.is_empty() {
        return;
    }
    let is_python = path.extension().and_then(|e| e.to_str()) == Some("py");
    let lines: Vec<&str> = source.lines().collect();
    for sym in symbols {
        if truncate {
            out.push_str(&format_symbol_name(sym, &lines));
            out.push('\n');
        } else {
            let sig_end = signature_end_line(&lines, sym, is_python);
            for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym.line - 1) {
                out.push_str(&fmt_line(i, line));
            }
        }
    }
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

/// Render all lines of a file with line numbers (level 5).
fn render_full_source(out: &mut String, source: &str) {
    for (i, line) in source.lines().enumerate() {
        out.push_str(&fmt_line(i, line));
    }
}

/// Render symbol lines with preceding doc comments (level 3).
/// If `expand_types` is true (level 4), show full bodies for struct/enum/trait/interface.
/// For Markdown: level 3 shows first paragraph after each heading, level 4 shows all content.
/// `path` is relative to the project root (used for language detection).
fn render_symbols_with_docs(
    out: &mut String,
    path: &Path,
    source: &str,
    symbols: &[parse::Symbol],
    expand_types: bool,
) {
    if symbols.is_empty() {
        return;
    }
    let lines: Vec<&str> = source.lines().collect();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let is_python = ext == "py";
    let is_markdown = ext == "md";
    let is_go = ext == "go";
    // Track which lines have been emitted to avoid duplication when type bodies
    // overlap with nested symbols (e.g. trait methods inside a trait body).
    let mut emitted_up_to: usize = 0; // 0-indexed, exclusive
    for (sym_idx, sym) in symbols.iter().enumerate() {
        let sym_line_0 = sym.line - 1;
        let doc_start = doc_comment_start(&lines, sym_line_0, is_go);
        let should_expand = expand_types && is_type_definition(sym.kind);
        if should_expand {
            let body_end = sym.end_line; // 1-indexed, inclusive
            // Start from doc_start or emitted_up_to, whichever is later
            let start = doc_start.max(emitted_up_to);
            let end = body_end; // 1-indexed inclusive → 0-indexed exclusive
            for (i, line) in lines.iter().enumerate().take(end).skip(start) {
                out.push_str(&fmt_line(i, line));
            }
            emitted_up_to = emitted_up_to.max(end);
        } else if sym_line_0 >= emitted_up_to {
            // Non-expanded symbol: show doc comments + multi-line signature
            let start = doc_start.max(emitted_up_to);
            let sig_end = signature_end_line(&lines, sym, is_python);
            // Doc comment lines before signature
            for (i, line) in lines.iter().enumerate().take(sym_line_0).skip(start) {
                out.push_str(&fmt_line(i, line));
            }
            // Signature lines (may span multiple lines for functions)
            for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
                if i >= emitted_up_to {
                    out.push_str(&fmt_line(i, line));
                }
            }
            emitted_up_to = emitted_up_to.max(sig_end + 1);
            // Python docstrings: include triple-quoted string after the signature
            if is_python {
                let ds_end = docstring_end(&lines, sig_end);
                for (i, line) in lines.iter().enumerate().take(ds_end).skip(sig_end + 1) {
                    out.push_str(&fmt_line(i, line));
                }
                emitted_up_to = emitted_up_to.max(ds_end);
            }
            // Markdown: include body text after headings
            if is_markdown {
                let content_end = markdown_content_end(symbols, sym_idx, &lines, expand_types);
                for (i, line) in lines
                    .iter()
                    .enumerate()
                    .take(content_end)
                    .skip(sym_line_0 + 1)
                {
                    out.push_str(&fmt_line(i, line));
                }
                emitted_up_to = emitted_up_to.max(content_end);
            }
        }
    }
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
/// For multi-line signatures, scans forward for the opening body delimiter
/// (`{` or `;` for C-like languages, `:` for Python).
/// Strips trailing line comments (`//` or `#`) before checking delimiters,
/// so `interface Foo { // eslint-disable` correctly matches on `{`.
/// Returns sym.line - 1 for single-line or non-function symbols.
fn signature_end_line(lines: &[&str], sym: &parse::Symbol, is_python: bool) -> usize {
    let sym_line_0 = sym.line - 1;
    if !has_multiline_signature(sym.kind) {
        return sym_line_0;
    }
    let max_line = sym.end_line.min(lines.len());
    for (i, line) in lines.iter().enumerate().take(max_line).skip(sym_line_0) {
        let trimmed = line.trim();
        if is_python {
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
/// This is a heuristic: `//` inside string literals may be incorrectly stripped,
/// but in signature context (function/class/interface declarations) this is rare
/// and the fallback (not matching the delimiter) is acceptable.
fn strip_c_line_comment(s: &str) -> &str {
    match s.find("//") {
        Some(pos) => s[..pos].trim_end(),
        None => s,
    }
}

/// Strip a trailing `#` comment from a Python line, returning the code portion.
/// Same heuristic caveat as `strip_c_line_comment`.
fn strip_python_line_comment(s: &str) -> &str {
    match s.find('#') {
        Some(pos) => s[..pos].trim_end(),
        None => s,
    }
}

/// Find the first line (0-indexed) of the doc comment block preceding a symbol.
/// Returns the symbol's own line index if there's no doc comment.
fn doc_comment_start(lines: &[&str], symbol_line_0: usize, is_go: bool) -> usize {
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

    // Rust-style line doc comments (/// or //!)
    if prev_trimmed.starts_with("///") || prev_trimmed.starts_with("//!") {
        peek -= 1;
        while peek > 0 {
            let above = lines[peek - 1].trim();
            if above.starts_with("///") || above.starts_with("//!") {
                peek -= 1;
            } else {
                break;
            }
        }
        return peek;
    }

    // Go doc comments (plain // preceding a declaration)
    if is_go && prev_trimmed.starts_with("//") {
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

    // Block doc comments (/** ... */ — JSDoc or Rust)
    if prev_trimmed.ends_with("*/") {
        let mut scan = peek - 1;
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

/// For Markdown files, determine how many content lines to include after a heading.
/// At level 3 (expand_types=false): first paragraph (until blank line or next heading).
/// At level 4 (expand_types=true): all content until next heading.
/// Returns end position (0-indexed, exclusive).
fn markdown_content_end(
    symbols: &[parse::Symbol],
    sym_idx: usize,
    lines: &[&str],
    expand_types: bool,
) -> usize {
    let sym_line_0 = symbols[sym_idx].line - 1;
    let next_heading = symbols
        .get(sym_idx + 1)
        .map(|s| s.line - 1)
        .unwrap_or(lines.len());

    if expand_types {
        // Level 4: all content until next heading
        next_heading
    } else {
        // Level 3: first paragraph after heading
        let mut idx = sym_line_0 + 1;
        // Skip blank lines immediately after heading
        while idx < next_heading && lines[idx].trim().is_empty() {
            idx += 1;
        }
        // Include non-blank lines until blank line or next heading
        while idx < next_heading && !lines[idx].trim().is_empty() {
            idx += 1;
        }
        idx
    }
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
