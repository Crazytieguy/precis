use crate::layout;
use crate::parse;
use crate::render;

use super::{Cost, Group, StageKind, SymbolRef};

// ---------------------------------------------------------------------------
// Source-line mapping
// ---------------------------------------------------------------------------

/// Map a layer index `n` (1-based) to the source line index and total line count
/// for a symbol at that layer. Returns `None` if the symbol has no content at layer `n`.
fn layer_source_line(
    sym: &parse::Symbol,
    is_doc: bool,
    n: usize,
) -> Option<(usize, usize)> {
    let layout = &sym.layout;
    if is_doc {
        let pre = layout.doc_end.saturating_sub(layout.doc_start);
        let total = pre + layout.ds_end.saturating_sub(layout.ds_start);
        if n - 1 < pre {
            Some((layout.doc_start + (n - 1), total))
        } else if n - 1 < total {
            Some((layout.ds_start + (n - 1 - pre), total))
        } else {
            None
        }
    } else {
        let count = layout.body_end.saturating_sub(layout.body_start);
        if n - 1 < count {
            Some((layout.body_start + (n - 1), count))
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Token cost helpers
// ---------------------------------------------------------------------------

/// Token cost of the truncation marker that would appear after a given source line.
fn truncation_marker_cost(source_line: &str) -> Cost {
    let text = render::truncation_marker(source_line);
    Cost::of(&text)
}

/// Compute name + signature token costs for a symbol. Returns a `SymbolRef`
/// plus the name and signature costs as separate `Cost` values.
pub(super) fn compute_name_sig_costs(
    file_idx: usize,
    symbol_idx: usize,
    sym: &parse::Symbol,
    lines: &[&str],
) -> (SymbolRef, Cost, Cost) {
    let layout = &sym.layout;
    let sym_line_0 = layout.sym_line_0;
    let is_section = sym.kind.is_section_like();

    // Composite symbols: name cost is the first prefix + " …", no signature.
    if !sym.composed_prefix_lens.is_empty() {
        let line = lines.get(sym_line_0).copied().unwrap_or("");
        let prefix_len = sym.composed_prefix_lens[0].min(line.len());
        let has_more = sym.composed_prefix_lens.len() > 1;
        let name_text = if has_more {
            format!("{} …\n", render::fmt_line(sym_line_0, &line[..prefix_len]).trim_end())
        } else {
            render::fmt_line(sym_line_0, &line[..prefix_len])
        };
        return (
            SymbolRef { file_idx, symbol_idx },
            Cost::of(&name_text),
            Cost::default(),
        );
    }

    let sig_end = layout.sig_end;

    let name = if is_section {
        let line = layout::strip_heading_badges(lines.get(layout.sym_line_0).copied().unwrap_or(""));
        let text = render::fmt_line(layout.sym_line_0, line);
        Cost::of(&text)
    } else {
        let name_line = render::format_symbol_name(sym, lines);
        let text = format!("{} …\n", name_line);
        Cost::of(&text)
    };

    let mut sig_formatted = Cost::default();
    for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
        let line = if is_section && i == sym_line_0 {
            layout::strip_heading_badges(line)
        } else {
            line
        };
        let text = render::fmt_line(i, line);
        sig_formatted += Cost::of(&text);
    }
    let signature = sig_formatted - name;

    (SymbolRef { file_idx, symbol_idx }, name, signature)
}

// ---------------------------------------------------------------------------
// Phase 2: aggregate layer costs (budget-aware)
// ---------------------------------------------------------------------------

/// Compute pre-aggregated doc/body layer costs for a group.
///
/// For each layer n, computes the net incremental cost of showing that layer
/// across all symbols in the group, including truncation marker deltas. The
/// result is stored directly in `group.doc_layer_costs` and
/// `group.body_layer_costs`. Layers whose cumulative cost exceeds `budget`
/// are skipped (budget-aware truncation).
pub(super) fn fill_layer_costs(
    group: &mut Group,
    all_lines: &[Vec<&str>],
    files: &[crate::FileData],
    budget: usize,
) {
    // Determine true max doc/body line counts across symbols.
    let mut true_max_doc = 0usize;
    let mut true_max_body = 0usize;
    for sref in &group.symbols {
        let sym = &files[sref.file_idx].symbols[sref.symbol_idx];

        // Composite symbols: body lines = additional symbols on the shared line.
        let (doc_count, body_count) = if !sym.composed_prefix_lens.is_empty() {
            (0, sym.composed_prefix_lens.len().saturating_sub(1))
        } else {
            let layout = &sym.layout;
            let doc = layout.doc_end.saturating_sub(layout.doc_start)
                + layout.ds_end.saturating_sub(layout.ds_start);
            (doc, layout.body_end.saturating_sub(layout.body_start))
        };

        true_max_doc = true_max_doc.max(doc_count);
        true_max_body = true_max_body.max(body_count);
    }

    // Base cost: names + signatures (already pre-aggregated in phase 1).
    let base_cost: usize = group.names_cost.tokens + group.signatures_cost.tokens;
    let mut cumulative = base_cost;

    let stages = group.key.kind_category.stage_sequence();
    for &stage_kind in stages {
        match stage_kind {
            StageKind::FilePath | StageKind::Names | StageKind::Signatures => continue,
            StageKind::Doc | StageKind::Body => {}
        }

        if cumulative > budget {
            break;
        }

        let true_max = match stage_kind {
            StageKind::Doc => true_max_doc,
            StageKind::Body => true_max_body,
            _ => unreachable!(),
        };

        let is_doc = stage_kind == StageKind::Doc;
        let mut prev_markers = Cost::default();

        // Byte-count threshold for skipping BPE: bytes/8 underestimates
        // tokens by ~2-4x (real ratio is ~3-4 bytes/token for o200k_base on
        // formatted code). If even this underestimate exceeds remaining budget,
        // the layer definitely exceeds it.
        const BYTES_PER_TOKEN: usize = 8;

        for n in 1..=true_max {
            let remaining = budget.saturating_sub(cumulative);
            let mut layer_content = Cost::default();
            let mut markers_at_n = Cost::default();
            let mut layer_bytes = 0usize;
            let mut byte_estimate_exceeded = false;

            for sref in &group.symbols {
                let sym = &files[sref.file_idx].symbols[sref.symbol_idx];

                // Composite symbols: no doc, body lines extend the prefix to
                // include one more original symbol on the shared source line.
                if !sym.composed_prefix_lens.is_empty() {
                    if is_doc { continue; }
                    let pl = &sym.composed_prefix_lens;
                    let body_count = pl.len() - 1;
                    if n > body_count {
                        continue;
                    }
                    let file_lines = &all_lines[sref.file_idx];
                    let line = file_lines.get(sym.line - 1).copied().unwrap_or("");

                    // Boundary-window correction for accurate incremental cost.
                    // Window covers the longest possible BPE token merge span.
                    const BPE_WINDOW: usize = 40;

                    let prev_end = pl[n - 1].min(line.len());
                    let curr_end = pl[n].min(line.len());
                    let slice = &line[prev_end..curr_end];
                    let slice_tokens = render::count_tokens(slice);

                    let left_start = line.floor_char_boundary(prev_end.saturating_sub(BPE_WINDOW));
                    let right_end = line.ceil_char_boundary((prev_end + BPE_WINDOW).min(curr_end));
                    let left = &line[left_start..prev_end];
                    let right = &line[prev_end..right_end];
                    let joint = [left, right].concat();
                    let correction = (render::count_tokens(left) + render::count_tokens(right))
                        .saturating_sub(render::count_tokens(&joint));

                    let lt = slice_tokens.saturating_sub(correction);

                    layer_content += Cost::new(lt, curr_end - prev_end);
                    // body_has_nested is effectively true for composites,
                    // so no truncation marker is added.
                    continue;
                }

                // Derive per-symbol metadata from layout (not stored in Group).
                let layout = &sym.layout;
                let pre_doc_count = layout.doc_end.saturating_sub(layout.doc_start);
                let body_has_nested = layout.has_children;

                let (src_line_idx, true_len) = match layer_source_line(sym, is_doc, n) {
                    Some(result) => result,
                    None => continue,
                };

                let file_lines = &all_lines[sref.file_idx];
                let line = file_lines.get(src_line_idx).copied().unwrap_or("");

                // fmt_line overhead: 6-char line number + 3-byte → + newline
                layer_bytes += 10 + line.len();
                if layer_bytes / BYTES_PER_TOKEN > remaining {
                    byte_estimate_exceeded = true;
                    break;
                }

                let fmt = render::fmt_line(src_line_idx, line);
                let line_cost = Cost::of(&fmt);
                let marker_cost = truncation_marker_cost(line);

                layer_content += line_cost;

                // Truncation marker: shown when there are more lines beyond n.
                let show_marker = if is_doc {
                    // Suppress marker at the pre-comment/docstring split point.
                    !(true_len > pre_doc_count && n == pre_doc_count)
                } else {
                    // Suppress marker for symbols with nested children.
                    !body_has_nested
                };
                if true_len > n && show_marker {
                    markers_at_n += marker_cost;
                }
            }

            if byte_estimate_exceeded {
                break;
            }

            let net_cost = (layer_content + markers_at_n) - prev_markers;
            prev_markers = markers_at_n;

            if is_doc {
                group.doc_layer_costs.push(net_cost);
            } else {
                group.body_layer_costs.push(net_cost);
            }

            cumulative += net_cost.tokens;
            if cumulative > budget {
                break;
            }
        }
    }

}
