use std::path::{Path, PathBuf};

use crate::parse;
use crate::schedule::{self, IncludedStage, StageKind};
use crate::Lang;

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

/// Render files within a word budget using group-based scheduling.
pub fn render_with_budget(
    budget: usize,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> String {
    let (output, _) = render_with_budget_stats(budget, root, files, sources);
    output
}

/// Render files within a word budget, returning output and actual word count.
///
/// Compensates for the scheduler/renderer cost mismatch: the scheduler charges
/// per-symbol but the renderer deduplicates at the line level (`emitted_up_to`),
/// causing nested symbols to be skipped. A small budget inflation absorbs the
/// typical mismatch without an extra scheduling pass.
pub fn render_with_budget_stats(
    budget: usize,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> (String, usize) {
    let all_symbols = extract_all_symbols(files, sources);
    let groups = schedule::build_groups(root, files, sources, &all_symbols);

    // Inflate budget by ~3% to compensate for renderer-side line deduplication.
    // Nested symbols (e.g., methods inside class bodies) are charged by the
    // scheduler but skipped by the renderer when their lines are already emitted.
    let effective_budget = budget + budget / 33;
    let sched = schedule::schedule(&groups, effective_budget, root, files);
    let mut output = render_scheduled(root, files, sources, &all_symbols, &groups, &sched);
    let mut actual = count_words(&output);

    // If still significantly underutilized (>5%), retry with larger inflation.
    let underuse = budget.saturating_sub(actual);
    if underuse > budget / 20 {
        let retry_budget = effective_budget + underuse;
        let sched = schedule::schedule(&groups, retry_budget, root, files);
        output = render_scheduled(root, files, sources, &all_symbols, &groups, &sched);
        actual = count_words(&output);
    }

    (output, actual)
}

/// Render a single file within a word budget.
pub fn render_file_with_budget(
    budget: usize,
    path: &Path,
    root: &Path,
    source: &str,
) -> String {
    let files = vec![path.to_path_buf()];
    let sources = vec![Some(source.to_string())];
    render_with_budget(budget, root, &files, &sources)
}

/// Pre-read source files to avoid repeated disk I/O.
pub fn read_sources(files: &[PathBuf]) -> Vec<Option<String>> {
    files
        .iter()
        .map(|f| std::fs::read_to_string(f).ok())
        .collect()
}

/// Pre-extract symbols from all source files.
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

// ---------------------------------------------------------------------------
// Rendering from a schedule
// ---------------------------------------------------------------------------

/// Render output from a computed schedule.
fn render_scheduled(
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
    all_symbols: &[Vec<parse::Symbol>],
    groups: &[schedule::Group],
    sched: &schedule::Schedule,
) -> String {
    let mut out = String::new();

    for (file_idx, (file, source)) in files.iter().zip(sources.iter()).enumerate() {
        if !sched.visible_files.contains(&file_idx) {
            continue;
        }
        let relative = file.strip_prefix(root).unwrap_or(file);
        out.push_str(&format!("{}\n", relative.display()));

        let source = match source {
            Some(s) => s,
            None => continue,
        };
        let lines: Vec<&str> = source.lines().collect();
        let lang = Lang::from_path(relative);
        let symbols = &all_symbols[file_idx];

        // Track emitted lines to avoid duplication when ranges overlap
        let mut emitted_up_to: usize = 0; // 0-indexed, exclusive

        for (sym_idx, sym) in symbols.iter().enumerate() {
            let group_idx = match sched.symbol_to_group.get(&(file_idx, sym_idx)) {
                Some(&gi) => gi,
                None => continue,
            };
            let included = match &sched.group_stages[group_idx] {
                Some(stage) => stage,
                None => continue, // group is hidden
            };

            // FilePath stage: file path is shown but no symbol content
            if included.kind == StageKind::FilePath {
                continue;
            }

            let sym_line_0 = sym.line - 1;
            if sym_line_0 < emitted_up_to {
                continue;
            }

            render_symbol(
                &mut out,
                &lines,
                sym,
                sym_idx,
                symbols,
                lang,
                included,
                &groups[group_idx],
                &mut emitted_up_to,
            );
        }
    }

    out
}

/// Render a single symbol at the given included stage.
#[allow(clippy::too_many_arguments)]
fn render_symbol(
    out: &mut String,
    lines: &[&str],
    sym: &parse::Symbol,
    sym_idx: usize,
    all_symbols: &[parse::Symbol],
    lang: Option<Lang>,
    included: &IncludedStage,
    group: &schedule::Group,
    emitted_up_to: &mut usize,
) {
    let sym_line_0 = sym.line - 1;
    let stages = group.key.kind_category.stage_sequence();

    // Determine what to show based on the group's included stage.
    // The included stage is the HIGHEST stage reached. All earlier stages
    // in the progression are implicitly included.
    let stage_pos = stages
        .iter()
        .position(|&s| s == included.kind)
        .unwrap_or(0);

    let show_names = stages.iter().position(|&s| s == StageKind::Names).is_some_and(|p| p <= stage_pos);
    let show_sigs = stages.iter().position(|&s| s == StageKind::Signatures).is_some_and(|p| p <= stage_pos);
    let show_doc = stages.iter().position(|&s| s == StageKind::Doc).is_some_and(|p| p <= stage_pos);
    let show_body = stages.iter().position(|&s| s == StageKind::Body).is_some_and(|p| p <= stage_pos);

    let doc_n = if included.kind == StageKind::Doc { included.n_lines } else if show_doc { usize::MAX } else { 0 };
    let body_n = if included.kind == StageKind::Body { included.n_lines } else if show_body { usize::MAX } else { 0 };

    if !show_names {
        return;
    }

    // Names only: truncated symbol name with inline ellipsis
    if !show_sigs && doc_n == 0 && body_n == 0 {
        out.push_str(&format_symbol_name(sym, lines));
        out.push_str(" …\n");
        return;
    }

    // Signature lines
    let sig_end = if show_sigs {
        signature_end_line(lines, sym, lang)
    } else {
        sym_line_0 // just the first line
    };

    // Doc comment lines (before symbol for most languages)
    let doc_start = doc_comment_start(lines, sym_line_0, lang);
    let mut doc_lines_shown = 0;
    if doc_n > 0 && doc_start < sym_line_0 {
        let doc_lines_available = sym_line_0 - doc_start;
        let doc_lines_to_show = doc_lines_available.min(doc_n);
        doc_lines_shown = doc_lines_to_show;
        let start = doc_start.max(*emitted_up_to);
        let end = (doc_start + doc_lines_to_show).min(sym_line_0);
        for (i, line) in lines.iter().enumerate().take(end).skip(start) {
            out.push_str(&fmt_line(i, line));
        }
        if doc_lines_to_show < doc_lines_available {
            out.push_str(TRUNCATION_MARKER);
        }
    }

    // Signature lines (strip trailing badges from markdown heading lines)
    let is_section = sym.kind == parse::SymbolKind::Section;
    for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
        if i >= *emitted_up_to {
            if is_section && i == sym_line_0 {
                out.push_str(&fmt_line(i, strip_heading_badges(line)));
            } else {
                out.push_str(&fmt_line(i, line));
            }
        }
    }
    *emitted_up_to = (*emitted_up_to).max(sig_end + 1);

    // Python docstrings (after signature)
    // doc_n is a cumulative limit across pre-symbol comments and docstrings,
    // matching the scheduler's flat doc_line_words vector.
    let doc_n_remaining = doc_n.saturating_sub(doc_lines_shown);
    if doc_n_remaining > 0 && lang == Some(Lang::Python) {
        let ds_end = docstring_end(lines, sig_end);
        if ds_end > sig_end + 1 {
            let ds_lines_available = ds_end - (sig_end + 1);
            let ds_lines_to_show = ds_lines_available.min(doc_n_remaining);
            let end = sig_end + 1 + ds_lines_to_show;
            for (i, line) in lines.iter().enumerate().take(end).skip(sig_end + 1) {
                out.push_str(&fmt_line(i, line));
            }
            if ds_lines_to_show < ds_lines_available {
                out.push_str(TRUNCATION_MARKER);
            }
            *emitted_up_to = (*emitted_up_to).max(end);
        }
    }

    // Body lines
    if body_n > 0 {
        if sym.kind == parse::SymbolKind::Section {
            // Markdown: body is content text after heading
            let heading_end = (sym.end_line - 1).max(sym_line_0 + 1);
            let next_heading_line = all_symbols
                .iter()
                .skip(sym_idx + 1)
                .find(|s| s.kind == parse::SymbolKind::Section)
                .map(|s| s.line - 1)
                .unwrap_or(lines.len());
            // Skip leading noise (badges, link refs, blank lines)
            let mut content_start = heading_end;
            while content_start < next_heading_line
                && is_markdown_leading_noise(lines[content_start])
            {
                content_start += 1;
            }
            let body_lines_available = next_heading_line.saturating_sub(content_start);
            let body_lines_to_show = body_lines_available.min(body_n);
            let start = content_start.max(*emitted_up_to);
            let end = content_start + body_lines_to_show;
            for (i, line) in lines.iter().enumerate().take(end).skip(start) {
                out.push_str(&fmt_line(i, line));
            }
            if body_lines_to_show < body_lines_available {
                out.push_str(TRUNCATION_MARKER);
            }
            *emitted_up_to = (*emitted_up_to).max(end);
        } else {
            // Code: body lines after signature (skip Python docstrings)
            let body_start = if lang == Some(Lang::Python) {
                let ds_end = docstring_end(lines, sig_end);
                if ds_end > sig_end + 1 { ds_end } else { sig_end + 1 }
            } else {
                sig_end + 1
            };
            let body_end = sym.end_line.min(lines.len());
            let body_lines_available = body_end.saturating_sub(body_start);
            let body_lines_to_show = body_lines_available.min(body_n);
            let start = body_start.max(*emitted_up_to);
            let end = body_start + body_lines_to_show;
            for (i, line) in lines.iter().enumerate().take(end).skip(start) {
                out.push_str(&fmt_line(i, line));
            }
            if body_lines_to_show < body_lines_available {
                // Skip marker when truncated lines overlap with nested symbols
                // (e.g., class body containing methods that render individually)
                let trunc_start = body_start + body_lines_to_show;
                let has_nested = all_symbols.iter().skip(sym_idx + 1).any(|s| {
                    let sl = s.line - 1;
                    sl >= trunc_start && sl < body_end
                });
                if !has_nested {
                    out.push_str(TRUNCATION_MARKER);
                }
            }
            *emitted_up_to = (*emitted_up_to).max(end);
        }
    }
}

// ---------------------------------------------------------------------------
// Output utilities
// ---------------------------------------------------------------------------

/// Format a single source line with its line number.
/// Line numbers are 1-indexed; `line_idx_0` is the 0-based index.
pub(crate) fn fmt_line(line_idx_0: usize, line: &str) -> String {
    format!("{:>6}→{}\n", line_idx_0 + 1, line)
}

/// Standalone truncation marker line indicating omitted content.
const TRUNCATION_MARKER: &str = "      →…\n";

/// Count whitespace-delimited words in text.
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Check if a markdown line is leading "noise" that should be skipped at
/// the start of section bodies. Matches:
/// - Blank or whitespace-only lines
/// - Markdown images: `![alt](url)` (badges/shields)
/// - Linked images: `[![alt](url)](url)` (clickable badges)
/// - Link reference definitions: `[label]: http...`
/// - HTML tags: `<div>`, `<p align=...>`, `<img .../>`, `</div>`, etc.
/// - HTML comments: `<!-- ... -->`
/// - Horizontal rules: `---`, `***`, `___`, `* * *`, etc.
///
/// Only used to skip contiguous noise at the start of a section body,
/// so mid-section images and links are still rendered normally.
pub(crate) fn is_markdown_leading_noise(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }
    // Markdown images (badges): ![alt](url) or [![alt](url)](url)
    if (trimmed.starts_with("![") || trimmed.starts_with("[![")) && trimmed.ends_with(')') {
        return true;
    }
    // Link reference definitions: [label]: URL
    if trimmed.starts_with('[')
        && let Some(pos) = trimmed.find("]: ")
    {
        let after = trimmed[pos + 3..].trim();
        if after.starts_with("http") || after.starts_with('/') || after.starts_with('#') {
            return true;
        }
    }
    // HTML tags: <div>, </div>, <p align=...>, <img .../>, <br>, <br/>, etc.
    // Also matches HTML comments: <!-- ... -->
    if let Some(rest) = trimmed.strip_prefix('<')
        && (rest.starts_with('/')
            || rest.starts_with('!')
            || rest.as_bytes().first().is_some_and(|b| b.is_ascii_alphabetic()))
    {
        return true;
    }
    // Horizontal rules: 3+ of the same character (-, *, _) with optional spaces
    if is_horizontal_rule(trimmed) {
        return true;
    }
    false
}

/// Check if a trimmed line is a markdown horizontal rule (thematic break).
/// Matches 3+ of the same character (`-`, `*`, or `_`), optionally
/// separated by spaces.
fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < 3 {
        return false;
    }
    let mut rule_char = None;
    let mut count = 0;
    for b in trimmed.bytes() {
        match b {
            b'-' | b'*' | b'_' => {
                if let Some(rc) = rule_char {
                    if b != rc {
                        return false;
                    }
                } else {
                    rule_char = Some(b);
                }
                count += 1;
            }
            b' ' => {}
            _ => return false,
        }
    }
    count >= 3
}

/// Strip trailing badge/image markdown from a markdown heading line.
///
/// Many README files include CI/coverage/version badges inline in the top-level
/// heading: `# project [![build](url)](url) [![version](url)](url)`.
/// These badge URLs waste word budget while adding no useful information for
/// codebase understanding.
///
/// Returns the prefix of the line before the first trailing ` [![` pattern.
/// Only matches the linked-image pattern (`[![`) which is specifically used
/// for badges; plain `![` images in headings are left intact.
pub(crate) fn strip_heading_badges(line: &str) -> &str {
    // Look for ` [![` — linked image (badge) preceded by a space.
    // Only strip if there's meaningful heading text before the badge.
    if let Some(pos) = line.find(" [![") {
        // Ensure there's at least one non-whitespace char of heading text
        // before the badge (skip the `# ` prefix).
        let before = line[..pos].trim();
        if !before.is_empty() && before != "#" {
            return line[..pos].trim_end();
        }
    }
    line
}

/// Convert rendered output to JSON, splitting into per-file entries.
pub fn to_json(output: &str, budget: usize, words: usize) -> String {
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
        "budget": budget,
        "words": words,
        "files": files,
    });
    serde_json::to_string_pretty(&json).unwrap()
}

// ---------------------------------------------------------------------------
// Symbol content helpers (used by both schedule and render)
// ---------------------------------------------------------------------------

/// Format a symbol line truncated at the symbol name.
pub(crate) fn format_symbol_name(sym: &parse::Symbol, lines: &[&str]) -> String {
    let source_line = lines.get(sym.line - 1).copied().unwrap_or("");
    let trimmed = source_line.trim_start();
    let indent = &source_line[..source_line.len() - trimmed.len()];
    let name_prefix = match find_word(&sym.name, trimmed) {
        Some(pos) => format!("{}{}", indent, &trimmed[..pos + sym.name.len()]),
        None => sym.name.clone(),
    };
    format!("{:>6}→{}", sym.line, name_prefix)
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
pub(crate) fn signature_end_line(lines: &[&str], sym: &parse::Symbol, lang: Option<Lang>) -> usize {
    let sym_line_0 = sym.line - 1;
    // Type aliases and imports: the entire declaration IS the signature,
    // so show all lines. For type aliases, this handles multi-line TypeScript
    // types. For imports, this shows the full import statement including
    // multi-line Rust `use foo::{A, B, C};` or Go grouped imports.
    if matches!(sym.kind, parse::SymbolKind::TypeAlias | parse::SymbolKind::Import) {
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

/// Strip a trailing `#` comment from a Python line, returning the code portion.
fn strip_python_line_comment(s: &str) -> &str {
    match s.find('#') {
        Some(pos) => s[..pos].trim_end(),
        None => s,
    }
}

/// Find the first line (0-indexed) of the doc comment block preceding a symbol.
/// Returns the symbol's own line index if there's no doc comment.
pub(crate) fn doc_comment_start(lines: &[&str], symbol_line_0: usize, lang: Option<Lang>) -> usize {
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
pub(crate) fn docstring_end(lines: &[&str], sym_line_0: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json_splits_files() {
        let output = "src/main.rs\n     1→fn main() {\nsrc/lib.rs\n     1→pub mod foo\n";
        let json_str = to_json(output, 1000, 5);
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json["budget"], 1000);
        assert_eq!(json["words"], 5);
        let files = json["files"].as_array().unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0]["path"], "src/main.rs");
        assert_eq!(files[0]["content"], "     1→fn main() {");
        assert_eq!(files[1]["path"], "src/lib.rs");
        assert_eq!(files[1]["content"], "     1→pub mod foo");
    }

    #[test]
    fn to_json_empty_content() {
        let output = "src/main.rs\nsrc/lib.rs\n";
        let json_str = to_json(output, 100, 2);
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        let files = json["files"].as_array().unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0]["content"], "");
        assert_eq!(files[1]["content"], "");
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
            line: 1,
            end_line: 3,
        };
        assert_eq!(signature_end_line(&lines, &sym, Some(Lang::JsTs)), 0);
    }

    #[test]
    fn strip_heading_badges_removes_trailing_badges() {
        // Linked images (badges) after heading text
        assert_eq!(
            strip_heading_badges("# color [![build](url)](link)"),
            "# color",
        );
        assert_eq!(
            strip_heading_badges("# Structs [![GoDoc](url)](link) [![Build](url)](link)"),
            "# Structs",
        );
        // Name-only text (without # prefix)
        assert_eq!(
            strip_heading_badges("color [![build](url)](link)"),
            "color",
        );
        // No badges — unchanged
        assert_eq!(strip_heading_badges("# Introduction"), "# Introduction");
        assert_eq!(strip_heading_badges("## API Reference"), "## API Reference");
        // Badge without preceding text — unchanged (just `#` prefix)
        assert_eq!(
            strip_heading_badges("# [![badge](url)](link)"),
            "# [![badge](url)](link)",
        );
        // Plain image (not linked) — not stripped (only [![ is targeted)
        assert_eq!(
            strip_heading_badges("# Project ![icon](url)"),
            "# Project ![icon](url)",
        );
    }

    #[test]
    fn budget_monotonicity() {
        // More budget should never produce fewer words.
        let source = "/// Doc comment\npub fn hello() {}\npub struct Foo { x: i32 }\nfn private() {}\n";
        let root = Path::new("");
        let path = Path::new("test.rs");
        let files = vec![path.to_path_buf()];
        let sources = vec![Some(source.to_string())];

        let mut prev_words = 0;
        for budget in [10, 50, 100, 200, 500, 1000, 5000] {
            let output = render_with_budget(budget, root, &files, &sources);
            let words = count_words(&output);
            assert!(
                words >= prev_words,
                "Budget monotonicity violated at budget {}: {} < {}",
                budget,
                words,
                prev_words,
            );
            prev_words = words;
        }
    }
}
