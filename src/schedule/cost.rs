use crate::format;
use crate::layout;
use crate::parse;

use super::value::compute_value;
use super::{Cost, CumulativeEntry, Group, StageCumulatives, StageKind, SymbolRef};

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
    let text = format::truncation_marker(source_line);
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
            format!("{} …\n", format::fmt_line(sym_line_0, &line[..prefix_len]).trim_end())
        } else {
            format::fmt_line(sym_line_0, &line[..prefix_len])
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
        let text = format::fmt_line(layout.sym_line_0, line);
        Cost::of(&text)
    } else {
        let name_line = format::format_symbol_name(sym, lines);
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
        let text = format::fmt_line(i, line);
        sig_formatted += Cost::of(&text);
    }
    let signature = sig_formatted - name;

    (SymbolRef { file_idx, symbol_idx }, name, signature)
}

// ---------------------------------------------------------------------------
// Phase 2: build cumulative prefix sums (budget-aware)
// ---------------------------------------------------------------------------

/// Build the cumulative cost/value prefix sums for a group's linear stage
/// progression.
///
/// Iterates stages in sequence order (e.g. FilePath → Names → Sig → Doc(1..max)
/// → Body(1..max)), computing per-layer costs with budget-aware truncation and
/// values via `compute_value`. The result is stored in `group.cumulatives`.
pub(super) fn build_cumulatives(
    group: &mut Group,
    all_lines: &[Vec<&str>],
    files: &[crate::FileData],
    budget: usize,
) {
    let stages = group.key.kind_category.stage_sequence();

    // Determine true max doc/body line counts across symbols.
    let mut true_max_doc = 0usize;
    let mut true_max_body = 0usize;
    for sref in &group.symbols {
        let sym = &files[sref.file_idx].symbols[sref.symbol_idx];
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

    let mut entries: Vec<CumulativeEntry> = Vec::new();
    let mut running_cost = Cost::default();
    let mut running_value: f64 = 0.0;

    // Track cumulative token cost for budget-aware truncation of doc/body.
    let mut token_cumulative: usize = 0;

    // Helper: append one entry to the cumulative array.
    let push = |stage, n, cost: Cost, entries: &mut Vec<CumulativeEntry>,
                    running_cost: &mut Cost, running_value: &mut f64,
                    token_cumulative: &mut usize| {
        let value = compute_value(group, stage, n);
        *running_cost += cost;
        *running_value += value;
        *token_cumulative += cost.tokens;
        entries.push(CumulativeEntry { stage, n, cost: *running_cost, value: *running_value });
    };

    for &stage_kind in stages {
        match stage_kind {
            // FilePath/Names/Signatures: single entry each. FilePath has zero
            // own cost (handled via file_path_costs on the QueueItem).
            StageKind::FilePath | StageKind::Names | StageKind::Signatures => {
                let cost = match stage_kind {
                    StageKind::Names => group.names_cost,
                    StageKind::Signatures => group.signatures_cost,
                    _ => Cost::default(),
                };
                push(stage_kind, 1, cost, &mut entries, &mut running_cost,
                     &mut running_value, &mut token_cumulative);
            }
            StageKind::Doc | StageKind::Body => {
                if token_cumulative > budget {
                    continue;
                }

                let true_max = if stage_kind == StageKind::Doc {
                    true_max_doc
                } else {
                    true_max_body
                };
                let is_doc = stage_kind == StageKind::Doc;
                let mut prev_markers = Cost::default();

                const BYTES_PER_TOKEN: usize = 8;

                for n in 1..=true_max {
                    let remaining = budget.saturating_sub(token_cumulative);
                    let mut layer_content = Cost::default();
                    let mut markers_at_n = Cost::default();
                    let mut layer_bytes = 0usize;
                    let mut byte_estimate_exceeded = false;

                    for sref in &group.symbols {
                        let sym = &files[sref.file_idx].symbols[sref.symbol_idx];

                        // Composite symbols: no doc, body lines extend prefix.
                        if !sym.composed_prefix_lens.is_empty() {
                            if is_doc { continue; }
                            let pl = &sym.composed_prefix_lens;
                            let body_count = pl.len() - 1;
                            if n > body_count { continue; }
                            let file_lines = &all_lines[sref.file_idx];
                            let line = file_lines.get(sym.line - 1).copied().unwrap_or("");

                            const BPE_WINDOW: usize = 40;
                            let prev_end = pl[n - 1].min(line.len());
                            let curr_end = pl[n].min(line.len());
                            let slice = &line[prev_end..curr_end];
                            let slice_tokens = format::count_tokens(slice);

                            let left_start = line.floor_char_boundary(prev_end.saturating_sub(BPE_WINDOW));
                            let right_end = line.ceil_char_boundary((prev_end + BPE_WINDOW).min(curr_end));
                            let left = &line[left_start..prev_end];
                            let right = &line[prev_end..right_end];
                            let joint = [left, right].concat();
                            let correction = (format::count_tokens(left) + format::count_tokens(right))
                                .saturating_sub(format::count_tokens(&joint));
                            let lt = slice_tokens.saturating_sub(correction);

                            layer_content += Cost::new(lt, curr_end - prev_end);
                            continue;
                        }

                        let layout = &sym.layout;
                        let pre_doc_count = layout.doc_end.saturating_sub(layout.doc_start);
                        let body_has_nested = layout.has_children;

                        let (src_line_idx, true_len) = match layer_source_line(sym, is_doc, n) {
                            Some(result) => result,
                            None => continue,
                        };

                        let file_lines = &all_lines[sref.file_idx];
                        let line = file_lines.get(src_line_idx).copied().unwrap_or("");

                        layer_bytes += 10 + line.len();
                        if layer_bytes / BYTES_PER_TOKEN > remaining {
                            byte_estimate_exceeded = true;
                            break;
                        }

                        let fmt = format::fmt_line(src_line_idx, line);
                        let line_cost = Cost::of(&fmt);
                        let marker_cost = truncation_marker_cost(line);

                        layer_content += line_cost;

                        let show_marker = if is_doc {
                            !(true_len > pre_doc_count && n == pre_doc_count)
                        } else {
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

                    push(stage_kind, n, net_cost, &mut entries, &mut running_cost,
                         &mut running_value, &mut token_cumulative);

                    if token_cumulative > budget {
                        break;
                    }
                }
            }
        }
    }

    group.cumulatives = StageCumulatives { entries };
}
