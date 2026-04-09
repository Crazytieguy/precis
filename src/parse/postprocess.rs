use std::collections::HashMap;

use crate::Lang;

use super::{Symbol, SymbolKind, plain_text_symbol};

/// Run all post-extraction processing on the symbol list.
///
/// Order: mark_reexports (Rust only) → dedup_overloads → plain-text fallback
/// (if no symbols remain) → merge_shared_line_symbols.
///
/// Returns the final symbol list (may be a fresh Vec from the fallback path).
pub(super) fn finalize(mut symbols: Vec<Symbol>, lang: Lang, source: &str) -> Vec<Symbol> {
    if lang == Lang::Rust {
        mark_reexports(&mut symbols);
    }
    let mut symbols = dedup_overloads(symbols, lang);
    if symbols.is_empty() {
        return plain_text_symbol(source);
    }
    merge_shared_line_symbols(&mut symbols, source);
    symbols
}

/// Mark `pub use` imports as re-exports when they reference a child module
/// declared in the same file.
fn mark_reexports(symbols: &mut [Symbol]) {
    let module_names: Vec<String> = symbols
        .iter()
        .filter(|s| s.kind == SymbolKind::Module)
        .map(|s| s.name.clone())
        .collect();

    for sym in symbols.iter_mut() {
        if sym.kind != SymbolKind::Import || !sym.is_public {
            continue;
        }
        if sym.name.starts_with("self::") {
            sym.is_reexport = true;
            continue;
        }
        if let Some(first_segment) = sym.name.split("::").next()
            && module_names.iter().any(|m| m == first_segment)
        {
            sym.is_reexport = true;
        }
    }
}

/// Collapse duplicate symbols with the same name and kind, keeping only the first occurrence.
/// Handles three patterns:
///
///   1. TypeScript/C++ function overloads (consecutive, same name)
///   2. C/C++ #ifdef branches (same struct defined for different platforms, may be non-consecutive)
///   3. C amalgamation files where embedded deps define the same struct as the main code
///
/// Go is excluded because it allows multiple `init()` functions in one file.
/// Python is excluded because @property getter/setter pairs share a name.
fn dedup_overloads(symbols: Vec<Symbol>, lang: Lang) -> Vec<Symbol> {
    if matches!(lang, Lang::Go | Lang::Python | Lang::Java) {
        return symbols;
    }
    let mut seen = std::collections::HashSet::new();
    let mut result: Vec<Symbol> = Vec::with_capacity(symbols.len());
    for sym in symbols {
        if let Some(last) = result.last()
            && last.name == sym.name
            && last.kind == sym.kind
        {
            *result.last_mut().unwrap() = sym;
            continue;
        }
        if matches!(sym.kind, SymbolKind::Struct | SymbolKind::Enum | SymbolKind::TypeAlias) {
            let key = (sym.name.clone(), sym.kind);
            if !seen.insert(key) {
                continue;
            }
        }
        result.push(sym);
    }
    result
}

/// Merge symbols that share a source line into composite symbols.
///
/// When multiple symbols of the same kind occupy the same line (e.g., 1,837
/// JSON key-value pairs on a single line in `emojis.json`), collapse them into
/// a single composite symbol. The original symbols become progressive "body
/// lines" — the output is always a valid prefix of the source line, extended
/// one symbol at a time as budget allows.
fn merge_shared_line_symbols(symbols: &mut Vec<Symbol>, source: &str) {
    // Group symbol indices by (line, kind) — only kinds with Body stage
    let mut groups: HashMap<(usize, SymbolKind), Vec<usize>> = HashMap::new();
    for (i, sym) in symbols.iter().enumerate() {
        if sym.kind != SymbolKind::Import {
            groups.entry((sym.line, sym.kind)).or_default().push(i);
        }
    }

    // Pre-compute line start bytes
    let line_starts: Vec<usize> = {
        let mut starts = Vec::new();
        let mut byte_pos = 0;
        for line_text in source.split('\n') {
            starts.push(byte_pos);
            byte_pos += line_text.len() + 1;
        }
        starts
    };

    // Find groups with 2+ symbols on the same line
    let mut to_remove: Vec<usize> = Vec::new();
    for ((line, _kind), indices) in &groups {
        if indices.len() < 2 {
            continue;
        }

        let line_0 = line - 1;
        let line_start_byte = line_starts.get(line_0).copied().unwrap_or(0);

        // Sort by start_byte, then filter to non-overlapping symbols only.
        let mut sorted_indices = indices.clone();
        sorted_indices.sort_by_key(|&i| symbols[i].start_byte);

        let mut non_overlapping: Vec<usize> = Vec::new();
        let mut max_end = 0usize;
        for &idx in &sorted_indices {
            let sym = &symbols[idx];
            if sym.start_byte >= max_end {
                non_overlapping.push(idx);
                max_end = sym.end_byte;
            }
        }

        if non_overlapping.len() < 2 {
            continue;
        }

        // Build composed_prefix_lens as char-boundary-safe prefix lengths.
        let line_text_bytes = source.get(line_start_byte..).unwrap_or("");
        let prefix_lens: Vec<usize> = non_overlapping.iter()
            .map(|&i| {
                let raw = symbols[i].end_byte.saturating_sub(line_start_byte);
                let clamped = raw.min(line_text_bytes.len());
                let mut pos = clamped;
                while pos < line_text_bytes.len() && !line_text_bytes.is_char_boundary(pos) {
                    pos += 1;
                }
                pos
            })
            .collect();

        // Transform the first symbol into the composite
        let first_idx = non_overlapping[0];
        symbols[first_idx].composed_prefix_lens = prefix_lens;

        for &idx in &non_overlapping[1..] {
            to_remove.push(idx);
        }
    }

    // Remove merged symbols (in reverse order to preserve indices)
    to_remove.sort_unstable();
    to_remove.dedup();
    for idx in to_remove.into_iter().rev() {
        symbols.remove(idx);
    }
}
