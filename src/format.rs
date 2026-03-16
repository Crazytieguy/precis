use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use tiktoken_rs::CoreBPE;

use crate::layout::{self, SymbolLayout};
use crate::parse;
use crate::schedule::{self, IncludedStage, StageKind};

/// Shared BPE tokenizer instance (o200k_base, used by GPT-4o / Claude-class models).
static BPE: LazyLock<CoreBPE> = LazyLock::new(|| tiktoken_rs::o200k_base().unwrap());

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

/// Render files within a token budget using group-based scheduling.
pub fn render_with_budget(
    budget: usize,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> String {
    let (output, _) = render_with_budget_stats(budget, root, files, sources);
    output
}

/// Render files within a token budget, returning output and actual token count.
pub fn render_with_budget_stats(
    budget: usize,
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
) -> (String, usize) {
    let all_symbols = extract_all_symbols(files, sources);
    let layouts = layout::compute_all_layouts(files, sources, &all_symbols);
    let groups = schedule::build_groups(root, files, sources, &all_symbols, &layouts);
    let sched = schedule::schedule(&groups, budget, root, files);
    let output = render_scheduled(root, files, sources, &all_symbols, &layouts, &groups, &sched);
    let actual = count_tokens(&output);
    (output, actual)
}

/// Render a single file within a token budget.
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
    layouts: &[Vec<SymbolLayout>],
    groups: &[schedule::Group],
    sched: &schedule::Schedule,
) -> String {
    let mut out = String::new();
    let mut first_file = true;

    // Build render order: README first, then project manifests, then everything
    // else in alphabetical order. Matches how a human presents a project overview.
    let render_order: Vec<usize> = {
        let mut readme_indices = Vec::new();
        let mut manifest_indices = Vec::new();
        let mut other_indices = Vec::new();
        for (i, file) in files.iter().enumerate() {
            let relative = file.strip_prefix(root).unwrap_or(file);
            let is_root = relative.parent().is_none_or(|p| p.as_os_str().is_empty());
            let role = schedule::FileRole::from_path(relative);
            let filename = relative.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if is_root && matches!(role, schedule::FileRole::Readme | schedule::FileRole::Architecture) {
                readme_indices.push(i);
            } else if is_root && matches!(filename, "Cargo.toml" | "package.json" | "go.mod" | "pyproject.toml" | "setup.cfg") {
                manifest_indices.push(i);
            } else {
                other_indices.push(i);
            }
        }
        readme_indices.extend(manifest_indices);
        readme_indices.extend(other_indices);
        readme_indices
    };

    // Compute top-level directories that are completely invisible (no visible files).
    // Only mark these to avoid cluttering with subdirectory markers.
    let mut dirs_with_files: std::collections::HashMap<PathBuf, bool> = std::collections::HashMap::new();
    for (file_idx, file) in files.iter().enumerate() {
        let relative = file.strip_prefix(root).unwrap_or(file);
        // Get the first path component (top-level directory)
        let top_dir = relative.components().next()
            .map(|c| PathBuf::from(c.as_os_str()));
        if let Some(dir) = top_dir {
            // Only track directories, not root-level files
            if relative.components().count() > 1 {
                let entry = dirs_with_files.entry(dir).or_insert(false);
                if sched.visible_files.contains(&file_idx) {
                    *entry = true; // directory has at least one visible file
                }
            }
        }
    }
    let invisible_dirs: std::collections::BTreeSet<PathBuf> = dirs_with_files
        .into_iter()
        .filter(|(_, has_visible)| !has_visible)
        .map(|(dir, _)| dir)
        .collect();
    let mut dirs_emitted: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    for &file_idx in &render_order {
        if !sched.visible_files.contains(&file_idx) {
            continue;
        }
        let file = &files[file_idx];
        let source = &sources[file_idx];
        let relative = file.strip_prefix(root).unwrap_or(file);

        // Before showing this file, emit omission markers for invisible
        // top-level directories that sort before this file's directory.
        let file_top_dir = relative.components().next()
            .map(|c| PathBuf::from(c.as_os_str()));
        if let Some(ref ftd) = file_top_dir {
            for dir in &invisible_dirs {
                if dir < ftd && dirs_emitted.insert(dir.clone()) {
                    if !first_file { out.push('\n'); }
                    first_file = false;
                    out.push_str(&format!("{}/\n      →…\n", dir.display()));
                }
            }
        }

        if !first_file { out.push('\n'); }
        first_file = false;
        out.push_str(&format!("{}\n", relative.display()));

        let source = match source {
            Some(s) => s,
            None => continue,
        };
        let lines: Vec<&str> = source.lines().collect();
        let symbols = &all_symbols[file_idx];

        // Track the highest source line emitted so far (exclusive) to
        // deduplicate overlapping ranges (e.g. Go grouped const block +
        // individual const_spec symbols sharing the same first line).
        let mut emitted_up_to: usize = 0;

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

            render_symbol(
                &mut out,
                &lines,
                sym,
                &layouts[file_idx][sym_idx],
                included,
                &groups[group_idx],
                &mut emitted_up_to,
            );
        }
    }

    // Emit any remaining invisible top-level directories
    for dir in &invisible_dirs {
        if dirs_emitted.insert(dir.clone()) {
            if !first_file {
                out.push('\n');
            }
            first_file = false;
            out.push_str(&format!("{}/\n      →…\n", dir.display()));
        }
    }

    out
}

/// Render a single symbol at the given included stage.
/// Parent body ranges are truncated at the first child's doc_start, but
/// overlaps can still occur (e.g. Go grouped const blocks). The
/// `emitted_up_to` high-water mark deduplicates within a file.
fn render_symbol(
    out: &mut String,
    lines: &[&str],
    sym: &parse::Symbol,
    layout: &SymbolLayout,
    included: &IncludedStage,
    group: &schedule::Group,
    emitted_up_to: &mut usize,
) {
    let sym_line_0 = layout.sym_line_0;
    let stages = group.key.kind_category.stage_sequence();

    // Determine what to show based on the group's included stage.
    // The included stage is the HIGHEST stage reached. All earlier stages
    // in the progression are implicitly included.
    let show_names = included.covers(stages, StageKind::Names, 1);
    let show_sigs = included.covers(stages, StageKind::Signatures, 1);
    let show_doc = included.covers(stages, StageKind::Doc, 1);
    let show_body = included.covers(stages, StageKind::Body, 1);

    let doc_n = if included.kind == StageKind::Doc { included.n_lines } else if show_doc { usize::MAX } else { 0 };
    let body_n = if included.kind == StageKind::Body { included.n_lines } else if show_body { usize::MAX } else { 0 };

    if !show_names {
        return;
    }

    // Names only
    if !show_sigs && doc_n == 0 && body_n == 0 {
        if sym_line_0 >= *emitted_up_to {
            if sym.kind == parse::SymbolKind::Section {
                // Sections: show the full heading/section line without truncation
                // marker. The heading text IS the name — truncating it looks broken
                // (e.g. `[package …` instead of `[package]`).
                let line = lines.get(sym_line_0).copied().unwrap_or("");
                out.push_str(&fmt_line(sym_line_0, layout::strip_heading_badges(line)));
            } else {
                out.push_str(&format_symbol_name(sym, lines));
                out.push_str(" …\n");
            }
            *emitted_up_to = sym_line_0 + 1;
        }
        return;
    }

    // Signature range from layout
    let sig_end = if show_sigs {
        layout.sig_end
    } else {
        sym_line_0 // just the first line
    };

    // Doc comment lines (before symbol for most languages)
    let doc_lines_shown = render_line_range(out, lines, layout.doc_start, layout.doc_end, doc_n, true, emitted_up_to);

    // Signature lines (strip trailing badges from markdown heading lines)
    let is_section = sym.kind == parse::SymbolKind::Section;
    for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
        if i < *emitted_up_to {
            continue;
        }
        if is_section && i == sym_line_0 {
            out.push_str(&fmt_line(i, layout::strip_heading_badges(line)));
        } else {
            out.push_str(&fmt_line(i, line));
        }
        *emitted_up_to = i + 1;
    }

    // Python docstrings (after signature)
    // doc_n is a cumulative limit across pre-symbol comments and docstrings,
    // matching the scheduler's flat doc_line_tokens vector.
    let doc_n_remaining = doc_n.saturating_sub(doc_lines_shown);
    render_line_range(out, lines, layout.ds_start, layout.ds_end, doc_n_remaining, true, emitted_up_to);

    // Body lines — all ranges from layout
    if body_n > 0 {
        if is_section {
            // Markdown: body is content text between headings
            render_line_range(out, lines, layout.md_content_start, layout.md_section_end, body_n, true, emitted_up_to);
        } else {
            // Code: body lines from layout (already truncated at first child)
            render_line_range(out, lines, layout.body_start, layout.body_end, body_n, !layout.has_children, emitted_up_to);
        }
    }
}

/// Render up to `max_lines` from a line range, with an optional truncation marker.
/// Skips lines already emitted (index < `*emitted_up_to`).
/// Returns the number of lines actually rendered.
fn render_line_range(
    out: &mut String,
    lines: &[&str],
    start: usize,
    end: usize,
    max_lines: usize,
    show_truncation: bool,
    emitted_up_to: &mut usize,
) -> usize {
    if max_lines == 0 || start >= end {
        return 0;
    }
    // Skip lines already emitted by a previous symbol
    let effective_start = start.max(*emitted_up_to);
    if effective_start >= end {
        return 0;
    }
    let available = end - effective_start;
    let to_show = available.min(max_lines);
    let render_end = effective_start + to_show;
    for (i, line) in lines.iter().enumerate().take(render_end).skip(effective_start) {
        out.push_str(&fmt_line(i, line));
    }
    *emitted_up_to = render_end;
    if show_truncation && to_show < available {
        out.push_str(&truncation_marker(lines[render_end - 1]));
    }
    to_show
}

// ---------------------------------------------------------------------------
// Output utilities
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
fn truncation_marker(last_line: &str) -> String {
    let indent_len = last_line.len() - last_line.trim_start().len();
    let indent = &last_line[..indent_len];
    format!("      →{}…\n", indent)
}

/// Count BPE tokens in text using the o200k_base tokenizer.
pub fn count_tokens(text: &str) -> usize {
    BPE.encode_with_special_tokens(text).len()
}

/// Convert rendered output to JSON, splitting into per-file entries.
pub fn to_json(output: &str, budget: usize, tokens: usize) -> String {
    let mut files: Vec<serde_json::Value> = Vec::new();
    let mut current_path: Option<&str> = None;
    let mut current_content = String::new();

    for line in output.lines() {
        if line.is_empty() {
            // Blank separator between file sections — skip
            continue;
        }
        if !line.contains('→') {
            if let Some(path) = current_path.take() {
                let content = current_content.trim_end_matches('\n');
                // Directory omission markers: paths ending with /
                if path.ends_with('/') {
                    files.push(serde_json::json!({"path": path, "omitted": true}));
                } else {
                    files.push(serde_json::json!({"path": path, "content": content}));
                }
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
        if path.ends_with('/') {
            files.push(serde_json::json!({"path": path, "omitted": true}));
        } else {
            let content = current_content.trim_end_matches('\n');
            files.push(serde_json::json!({"path": path, "content": content}));
        }
    }

    let n_files = files.iter().filter(|f| !f.get("omitted").is_some_and(|v| v.as_bool() == Some(true))).count();
    let json = serde_json::json!({
        "budget": budget,
        "tokens": tokens,
        "n_files": n_files,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json_splits_files() {
        let output = "src/main.rs\n     1→fn main() {\nsrc/lib.rs\n     1→pub mod foo\n";
        let json_str = to_json(output, 1000, 5);
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json["budget"], 1000);
        assert_eq!(json["tokens"], 5);
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
    fn budget_monotonicity() {
        // More budget should never produce fewer tokens.
        let source = "/// Doc comment\npub fn hello() {}\npub struct Foo { x: i32 }\nfn private() {}\n";
        let root = Path::new("");
        let path = Path::new("test.rs");
        let files = vec![path.to_path_buf()];
        let sources = vec![Some(source.to_string())];

        let mut prev_tokens = 0;
        for budget in [10, 50, 100, 200, 500, 1000, 5000] {
            let output = render_with_budget(budget, root, &files, &sources);
            let tokens = count_tokens(&output);
            assert!(
                tokens >= prev_tokens,
                "Budget monotonicity violated at budget {}: {} < {}",
                budget,
                tokens,
                prev_tokens,
            );
            prev_tokens = tokens;
        }
    }
}
