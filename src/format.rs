use std::path::{Path, PathBuf};

use crate::parse;
use crate::Lang;

/// Maximum granularity level.
///
/// The nominal level is a global parameter. The render function computes an
/// effective level per file by subtracting depth and size penalties, so
/// different files receive different rendering at the same nominal level.
///
/// Rendering policies (applied at the effective level for each file):
/// 0 - File paths only
/// 1 - Symbol lines, truncated to symbol name
/// 2 - Symbol lines, full multi-line signatures
/// 3 - Like 2, but public symbols also get preceding doc comments
/// 4 - Symbol lines with preceding doc comments (all symbols)
/// 5 - Like 4, but public type definition bodies shown in full
/// 6 - Like 5, but all type definition bodies shown in full
/// 7 - Full source (all lines)
pub const MAX_LEVEL: u8 = 7;

/// Render a single file at the given granularity level.
pub fn render_file(level: u8, path: &Path, root: &Path, source: &str) -> String {
    // Extract symbols for any level >= 1. Even at nominal level 5 (full source),
    // depth penalty may reduce the effective level to 1–4 which needs symbols.
    let symbols = if level >= 1 {
        parse::extract_symbols(path, source)
    } else {
        vec![]
    };
    render_with_symbols(level, path, root, source, &symbols)
}

/// Render a single file at the given level using pre-extracted symbols.
/// Used by budget search to avoid redundant symbol extraction across levels.
///
/// The effective level is adjusted by file depth and file size: deeper and
/// larger files are rendered with less detail so that shallow, focused files
/// are prioritised when a budget is tight. See [`depth_penalty`] and
/// [`size_penalty`] for the formulas.
fn render_with_symbols(
    level: u8,
    path: &Path,
    root: &Path,
    source: &str,
    symbols: &[parse::Symbol],
) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    // Apply depth penalty first, then size penalty only if the result is
    // level 3+. At levels 1–2 (names/signatures), output already scales with
    // symbol count rather than file size, so penalising large files is unhelpful.
    let after_depth = level.saturating_sub(depth_penalty(relative));
    let effective = after_depth.saturating_sub(
        if after_depth >= 3 { size_penalty(source) } else { 0 },
    );
    let mut out = format!("{}\n", relative.display());
    match effective {
        0 => {}
        1 => render_symbols(&mut out, relative, source, symbols, true),
        2 => render_symbols(&mut out, relative, source, symbols, false),
        3 => render_symbols_with_docs(&mut out, relative, source, symbols, false, true, false),
        4 => render_symbols_with_docs(&mut out, relative, source, symbols, false, false, false),
        5 => render_symbols_with_docs(&mut out, relative, source, symbols, true, false, true),
        6 => render_symbols_with_docs(&mut out, relative, source, symbols, true, false, false),
        _ => render_full_source(&mut out, source),
    }
    out
}

/// Compute a depth penalty for a file path.
///
/// Files at the root or one directory deep get full detail. Deeper files
/// have their effective level reduced so that shallow files are prioritised.
///
/// Depth 0–1: no penalty, Depth 2–3: −1 level, Depth 4–5: −2 levels, etc.
fn depth_penalty(relative: &Path) -> u8 {
    let depth = relative.components().count().saturating_sub(1);
    (depth as u8) / 2
}

/// Compute a size penalty for a file based on its line count.
///
/// Small files get full detail. Larger files have their effective level
/// reduced so that concise files are prioritised over verbose ones.
///
/// Files with 1000+ lines get −1 level when the effective level is 3+.
/// Capped at 1 to preserve the monotonicity invariant (level 3 with
/// penalty 1 = effective 2, which equals level 2 with no penalty).
fn size_penalty(source: &str) -> u8 {
    if source.lines().count() >= 1000 { 1 } else { 0 }
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

/// Convert rendered output to JSON, splitting into per-file entries.
pub fn to_json(output: &str, level: u8, words: usize) -> String {
    let mut files: Vec<serde_json::Value> = Vec::new();
    let mut current_path: Option<&str> = None;
    let mut current_content = String::new();

    for line in output.lines() {
        if !line.contains('→') {
            if let Some(path) = current_path.take() {
                let content = current_content.trim_end_matches('\n');
                files.push(serde_json::json!({"path": path, "content": content}));
                current_content.clear();
            }
            current_path = Some(line);
        } else {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }
    if let Some(path) = current_path {
        let content = current_content.trim_end_matches('\n');
        files.push(serde_json::json!({"path": path, "content": content}));
    }

    let json = serde_json::json!({
        "level": level,
        "words": words,
        "files": files,
    });
    serde_json::to_string_pretty(&json).unwrap()
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
    let all_symbols = extract_all_symbols(files, sources);
    render_files_with_symbols(level, root, files, sources, &all_symbols)
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
    let lang = Lang::from_path(path);
    let lines: Vec<&str> = source.lines().collect();
    // Track emitted lines to avoid duplication when type alias bodies
    // encompass nested symbols (e.g. method signatures inside a TS type literal).
    let mut emitted_up_to: usize = 0; // 0-indexed, exclusive
    for sym in symbols {
        if truncate {
            out.push_str(&format_symbol_name(sym, &lines));
            out.push('\n');
        } else {
            let sym_line_0 = sym.line - 1;
            if sym_line_0 < emitted_up_to {
                continue;
            }
            let sig_end = signature_end_line(&lines, sym, lang);
            for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
                out.push_str(&fmt_line(i, line));
            }
            emitted_up_to = emitted_up_to.max(sig_end + 1);
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

/// Render all lines of a file with line numbers (level 6).
fn render_full_source(out: &mut String, source: &str) {
    for (i, line) in source.lines().enumerate() {
        out.push_str(&fmt_line(i, line));
    }
}

/// Render symbol lines with preceding doc comments.
/// If `expand_types` is true (level 5+), show full bodies for struct/enum/trait/interface.
/// If `public_types_only` is true (level 5), only expand public type bodies.
/// If `public_only` is true (level 3), only show doc comments for public symbols;
/// private symbols still get their full signatures but no doc comments.
/// For Markdown: level 4 shows first paragraph after each heading, level 5+ shows all content.
/// `path` is relative to the project root (used for language detection).
fn render_symbols_with_docs(
    out: &mut String,
    path: &Path,
    source: &str,
    symbols: &[parse::Symbol],
    expand_types: bool,
    public_only: bool,
    public_types_only: bool,
) {
    if symbols.is_empty() {
        return;
    }
    let lines: Vec<&str> = source.lines().collect();
    let lang = Lang::from_path(path);
    // Track which lines have been emitted to avoid duplication when type bodies
    // overlap with nested symbols (e.g. trait methods inside a trait body).
    let mut emitted_up_to: usize = 0; // 0-indexed, exclusive
    for (sym_idx, sym) in symbols.iter().enumerate() {
        let sym_line_0 = sym.line - 1;
        let show_docs = !public_only || sym.is_public;
        let doc_start = if show_docs {
            doc_comment_start(&lines, sym_line_0, lang)
        } else {
            sym_line_0
        };
        // Expand type definitions (structs, enums, etc.) and Go grouped
        // const/var blocks (identified by their keyword name "const"/"var").
        let is_grouped_block =
            (sym.name == "const" || sym.name == "var") && sym.end_line > sym.line;
        let should_expand = expand_types
            && (is_type_definition(sym.kind) || is_grouped_block)
            && (!public_types_only || sym.is_public);
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
            let sig_end = signature_end_line(&lines, sym, lang);
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
            if show_docs && lang == Some(Lang::Python) {
                let ds_end = docstring_end(&lines, sig_end);
                for (i, line) in lines.iter().enumerate().take(ds_end).skip(sig_end + 1) {
                    out.push_str(&fmt_line(i, line));
                }
                emitted_up_to = emitted_up_to.max(ds_end);
            }
            // Markdown: include body text after headings.
            // Use max of sym_line + 1 and end_line - 1 to handle both cases:
            //   ATX headings (end_line may equal line if no trailing newline): sym_line + 1
            //   Setext headings (end_line > line + 1): end_line - 1 skips the underline
            // Note: Markdown headings are always public, so this is not affected by public_only.
            if show_docs && lang == Some(Lang::Markdown) {
                let content_end = markdown_content_end(symbols, sym_idx, &lines, expand_types);
                let heading_end = (sym.end_line - 1).max(sym_line_0 + 1);
                for (i, line) in lines
                    .iter()
                    .enumerate()
                    .take(content_end)
                    .skip(heading_end)
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
fn signature_end_line(lines: &[&str], sym: &parse::Symbol, lang: Option<Lang>) -> usize {
    let sym_line_0 = sym.line - 1;
    // Type aliases: the entire declaration IS the signature (no hidden body),
    // so show all lines. This handles multi-line TypeScript type aliases like
    // `export type Transpose<A, B, C> = ...` which span multiple lines.
    // Rust type aliases already work via `;` detection, but this is correct for both.
    if sym.kind == parse::SymbolKind::TypeAlias {
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
/// This is still a heuristic: `//` inside string literals may be incorrectly
/// stripped, but in signature context this is rare and the fallback is acceptable.
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
/// Same heuristic caveat as `strip_c_line_comment`.
fn strip_python_line_comment(s: &str) -> &str {
    match s.find('#') {
        Some(pos) => s[..pos].trim_end(),
        None => s,
    }
}

/// Find the first line (0-indexed) of the doc comment block preceding a symbol.
/// Returns the symbol's own line index if there's no doc comment.
fn doc_comment_start(lines: &[&str], symbol_line_0: usize, lang: Option<Lang>) -> usize {
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
    if lang == Some(Lang::Go) && prev_trimmed.starts_with("//") {
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
    let next_heading = symbols
        .get(sym_idx + 1)
        .map(|s| s.line - 1)
        .unwrap_or(lines.len());

    if expand_types {
        // Level 4: all content until next heading
        next_heading
    } else {
        // Level 3: first paragraph after heading.
        // Use max of sym_line + 1 and end_line - 1 to handle both cases:
        //   ATX headings (end_line may equal line if no trailing newline): sym_line + 1
        //   Setext headings (end_line > line + 1): end_line - 1 skips the underline
        let sym_line_0 = symbols[sym_idx].line - 1;
        let mut idx = (symbols[sym_idx].end_line - 1).max(sym_line_0 + 1);
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
/// Prefers matches outside parentheses to handle Go methods where the receiver
/// type name matches the method name (e.g. `func (f *Field) Field(...)`).
fn find_word(needle: &str, haystack: &str) -> Option<usize> {
    let mut start = 0;
    let mut first_match = None;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json_splits_files() {
        let output = "src/main.rs\n     1→fn main() {\nsrc/lib.rs\n     1→pub mod foo\n";
        let json_str = to_json(output, 1, 5);
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json["level"], 1);
        assert_eq!(json["words"], 5);
        let files = json["files"].as_array().unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0]["path"], "src/main.rs");
        assert_eq!(files[0]["content"], "     1→fn main() {");
        assert_eq!(files[1]["path"], "src/lib.rs");
        assert_eq!(files[1]["content"], "     1→pub mod foo");
    }

    #[test]
    fn to_json_level0_empty_content() {
        let output = "src/main.rs\nsrc/lib.rs\n";
        let json_str = to_json(output, 0, 2);
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        let files = json["files"].as_array().unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0]["content"], "");
        assert_eq!(files[1]["content"], "");
    }

    #[test]
    fn depth_penalty_values() {
        use std::path::Path;
        // Depth 0 (file at root): no penalty
        assert_eq!(depth_penalty(Path::new("main.rs")), 0);
        // Depth 1 (one directory): no penalty
        assert_eq!(depth_penalty(Path::new("src/main.rs")), 0);
        // Depth 2: penalty 1
        assert_eq!(depth_penalty(Path::new("src/utils/helpers.rs")), 1);
        // Depth 3: penalty 1
        assert_eq!(depth_penalty(Path::new("src/utils/internal/core.rs")), 1);
        // Depth 4: penalty 2
        assert_eq!(depth_penalty(Path::new("a/b/c/d/file.rs")), 2);
        // Depth 5: penalty 2
        assert_eq!(depth_penalty(Path::new("a/b/c/d/e/file.rs")), 2);
    }

    #[test]
    fn depth_aware_reduces_level_for_deep_files() {
        let source = "pub fn hello() {}\n";
        let root = Path::new("");

        // Shallow file at level 2: should show full signature
        let shallow = render_file(2, Path::new("lib.rs"), root, source);
        assert!(shallow.contains("→"), "shallow file should have symbol lines");

        // Deep file at level 2: penalty = 2/2 = 1, effective level 1
        let deep = render_file(2, Path::new("a/b/lib.rs"), root, source);
        assert!(deep.contains("→"), "deep file at effective level 1 should have symbol names");

        // Even deeper file at level 2: penalty = 3/2 = 1, effective level 1
        let deeper = render_file(2, Path::new("a/b/c/lib.rs"), root, source);
        assert!(deeper.contains("→"), "deeper file at effective level 1 should have symbol names");

        // Very deep file at level 2: penalty = 4/2 = 2, effective level 0
        let very_deep = render_file(2, Path::new("a/b/c/d/lib.rs"), root, source);
        assert!(!very_deep.contains("→"), "very deep file at effective level 0 should be path-only");

        // Very deep file at higher level can still show content
        let very_deep_l4 = render_file(4, Path::new("a/b/c/d/lib.rs"), root, source);
        assert!(very_deep_l4.contains("→"), "very deep file at level 4 (effective 2) should have content");
    }

    #[test]
    fn depth_aware_monotonicity() {
        // For any given file at any depth, increasing the nominal level
        // must never decrease the word count.
        let source = "/// Doc comment\npub fn hello() {}\npub struct Foo { x: i32 }\n";
        let root = Path::new("");

        for path in &[
            Path::new("lib.rs"),
            Path::new("src/lib.rs"),
            Path::new("a/b/lib.rs"),
            Path::new("a/b/c/d/lib.rs"),
        ] {
            let mut prev_words = 0;
            for level in 0..=MAX_LEVEL {
                let output = render_file(level, path, root, source);
                let words = count_words(&output);
                assert!(
                    words >= prev_words,
                    "Depth-aware monotonicity violated for {:?} at level {}: {} < {}",
                    path,
                    level,
                    words,
                    prev_words,
                );
                prev_words = words;
            }
        }
    }

    #[test]
    fn size_penalty_values() {
        // Short file: no penalty
        let short = "line\n".repeat(500);
        assert_eq!(size_penalty(&short), 0);

        // 999 lines: no penalty
        let just_under = "line\n".repeat(999);
        assert_eq!(size_penalty(&just_under), 0);

        // 1000 lines: penalty 1
        let threshold = "line\n".repeat(1000);
        assert_eq!(size_penalty(&threshold), 1);

        // 2000 lines: still penalty 1 (capped)
        let large = "line\n".repeat(2000);
        assert_eq!(size_penalty(&large), 1);
    }

    #[test]
    fn size_aware_reduces_level_for_large_files() {
        let root = Path::new("");
        // Generate a small source (< 1000 lines) and a large source (> 1000 lines)
        let small_source = "/// Doc comment\npub fn hello() {}\npub struct Foo { x: i32 }\n";
        let large_source = {
            let mut s = String::new();
            for i in 0..200 {
                s.push_str(&format!("/// doc {i}\npub fn func_{i}() {{}}\n"));
            }
            // Pad to > 1000 lines
            for i in 0..700 {
                s.push_str(&format!("// padding line {i}\n"));
            }
            s
        };
        assert!(large_source.lines().count() >= 1000);

        // Small file at level 3: should show doc comments
        let small_l3 = render_file(3, Path::new("small.rs"), root, small_source);
        assert!(small_l3.contains("Doc comment"), "small file at level 3 should include docs");

        // Large file at level 3: effective level 2 (signatures only, no docs)
        let large_l3 = render_file(3, Path::new("large.rs"), root, &large_source);
        assert!(!large_l3.contains("doc "), "large file at level 3 should not include docs");

        // Large file at level 4: effective level 3 (docs, not bodies)
        let large_l4 = render_file(4, Path::new("large.rs"), root, &large_source);
        assert!(large_l4.contains("doc "), "large file at level 4 should include docs");
    }

    #[test]
    fn size_aware_monotonicity() {
        // For files of various sizes, monotonicity must hold.
        let root = Path::new("");
        let small_source = "/// Doc comment\npub fn hello() {}\npub struct Foo { x: i32 }\n";
        let large_source = {
            let mut s = String::new();
            for i in 0..200 {
                s.push_str(&format!("/// doc {i}\npub fn func_{i}() {{}}\n"));
            }
            for i in 0..700 {
                s.push_str(&format!("// padding line {i}\n"));
            }
            s
        };

        for (path, source) in [
            (Path::new("small.rs"), small_source),
            (Path::new("large.rs"), large_source.as_str()),
        ] {
            let mut prev_words = 0;
            for level in 0..=MAX_LEVEL {
                let output = render_file(level, path, root, source);
                let words = count_words(&output);
                assert!(
                    words >= prev_words,
                    "Size-aware monotonicity violated for {:?} ({} lines) at level {}: {} < {}",
                    path,
                    source.lines().count(),
                    level,
                    words,
                    prev_words,
                );
                prev_words = words;
            }
        }
    }

    #[test]
    fn strip_c_line_comment_skips_urls() {
        // Plain comment stripping
        assert_eq!(strip_c_line_comment("code // comment"), "code");
        assert_eq!(strip_c_line_comment("no comment"), "no comment");
        // URL should NOT be stripped
        assert_eq!(
            strip_c_line_comment(r#"function fetch(url = "https://example.com") {"#),
            r#"function fetch(url = "https://example.com") {"#,
        );
        // http:// in string literal should be preserved
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
            line: 1,
            end_line: 3,
        };
        // Should detect `{` on line 0 (first line), not scan into the body
        assert_eq!(signature_end_line(&lines, &sym, Some(Lang::JsTs)), 0);
    }

    #[test]
    fn setext_heading_rendering() {
        let source = "Introduction\n============\n\nSome text here.\n\nGetting Started\n---------------\n\nMore content.\n";
        let root = Path::new("");
        let path = Path::new("test.md");

        // Level 1: symbol names only — underline should not appear
        let l1 = render_file(1, path, root, source);
        assert!(l1.contains("Introduction"), "level 1 should show heading name");
        assert!(!l1.contains("===="), "level 1 should not show underline");

        // Level 3: heading + first paragraph — underline should not appear as content
        let l3 = render_file(3, path, root, source);
        assert!(l3.contains("Introduction"), "level 3 should show heading name");
        assert!(!l3.contains("===="), "level 3 should not show underline");
        assert!(l3.contains("Some text here"), "level 3 should show first paragraph");
        assert!(!l3.contains("----"), "level 3 should not show underline");
        assert!(l3.contains("More content"), "level 3 should show second heading content");

        // Level 4: heading + all content — underline should not appear
        let l4 = render_file(4, path, root, source);
        assert!(!l4.contains("===="), "level 4 should not show underline");
        assert!(!l4.contains("----"), "level 4 should not show underline");
        assert!(l4.contains("Some text here"), "level 4 should show content");
        assert!(l4.contains("More content"), "level 4 should show content");

        // Monotonicity: levels should produce non-decreasing word counts
        let mut prev_words = 0;
        for level in 0..=MAX_LEVEL {
            let output = render_file(level, path, root, source);
            let words = count_words(&output);
            assert!(
                words >= prev_words,
                "Setext heading monotonicity violated at level {}: {} < {}",
                level, words, prev_words,
            );
            prev_words = words;
        }
    }
}
