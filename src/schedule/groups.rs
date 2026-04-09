use std::collections::{HashMap, HashSet};
use std::path::Path;

use rayon::prelude::*;

use crate::layout;
use crate::parse;
use crate::render;
use crate::Corpus;

use super::classify::{
    detect_heading_depth, is_autogen_api_doc, is_boilerplate_heading, is_config_file,
    is_generated_file, is_generated_filename, FileRole,
};
use super::{
    BuiltGroups, Cost, Group, GroupKey, KindCategory, StageKind, SymbolCosts,
};

/// Build groups from extracted symbols, computing per-symbol costs.
///
/// Doc/body line costs are computed with budget-aware truncation: layers whose
/// cumulative cost exceeds `budget` are skipped (they can never be scheduled).
/// The returned `BuiltGroups` carries the budget so `schedule()` can enforce it.
pub fn build_groups(
    corpus: &Corpus<'_>,
    budget: usize,
) -> BuiltGroups {
    let &Corpus { root, files, sources, all_symbols, layouts } = corpus;

    // Pre-compute lines for all files (needed for deferred doc/body tokenization).
    let all_lines: Vec<Vec<&str>> = sources
        .iter()
        .map(|s| {
            s.as_ref()
                .map(|s| s.lines().collect())
                .unwrap_or_default()
        })
        .collect();

    // Phase 1 (parallel): compute GroupKey + name/signature costs per file.
    let file_results: Vec<Vec<(GroupKey, SymbolCosts)>> = (0..files.len())
        .into_par_iter()
        .map(|file_idx| {
            let source = match sources[file_idx].as_ref() {
                Some(s) => s,
                None => return vec![],
            };
            let file = &files[file_idx];
            let symbols = &all_symbols[file_idx];
            let lines = &all_lines[file_idx];
            let file_layouts = &layouts[file_idx];
            let relative = file.strip_prefix(root).unwrap_or(file);
            let parent_dir = relative.parent().unwrap_or(Path::new("")).to_path_buf();
            let lang = crate::Lang::from_path(relative);
            let file_role = FileRole::from_path(relative);
            let is_file_config = relative.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| is_config_file(relative, name));
            let file_category = super::classify::classify_file(relative);
            let is_type_declaration = super::classify::is_type_declaration_file(relative);
            let is_header = relative.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| crate::is_header_extension(&ext.to_ascii_lowercase()));
            let is_generated = is_autogen_api_doc(source, file_role)
                || is_generated_file(source)
                || is_generated_filename(relative);

            symbols.iter().enumerate().map(|(symbol_idx, sym)| {
                let sym_line_0 = sym.line - 1;
                let kind_category = KindCategory::from_symbol_kind(sym.kind);
                let layout = &file_layouts[symbol_idx];

                // For imports, doc comments don't change value — the import line
                // itself is what matters. Force is_documented=false to keep all
                // imports in one group (avoids splitting __init__.py re-exports
                // into documented/undocumented subsets).
                let is_documented = kind_category != KindCategory::Import
                    && layout.doc_start < layout.doc_end;

                let heading_depth = if kind_category == KindCategory::Section {
                    if matches!(lang, Some(crate::Lang::Toml)) {
                        // Count dot-separated segments: [project] → 1, [tool.ruff] → 2
                        let depth = sym.name.chars().filter(|&c| c == '.').count() as u8 + 1;
                        Some(depth)
                    } else if matches!(lang, Some(crate::Lang::Markdown)) {
                        Some(detect_heading_depth(lines, sym_line_0, sym.end_line))
                    } else {
                        Some(1) // JSON/YAML/unsupported: all top-level
                    }
                } else {
                    None
                };

                let is_boilerplate_section = kind_category == KindCategory::Section
                    && matches!(lang, Some(crate::Lang::Markdown))
                    && is_boilerplate_heading(&sym.name);

                // TOML [tool.*] sections are tool configuration (linters, formatters,
                // type checkers, test runners) embedded in project manifests. They're
                // equivalent to dedicated config files like eslint.config.js, and should
                // be deprioritized similarly.
                let is_config = is_file_config
                    || (kind_category == KindCategory::Section
                        && matches!(lang, Some(crate::Lang::Toml))
                        && sym.name.starts_with("tool."));

                let key = GroupKey {
                    is_public: sym.is_public,
                    kind_category,
                    parent_dir: parent_dir.clone(),
                    is_documented,
                    file_role,
                    file_category,
                    is_type_declaration,
                    is_header,
                    is_generated,
                    is_config,
                    heading_depth,
                    is_first_party: sym.is_first_party,
                    is_trait_impl: sym.is_trait_impl,
                    is_boilerplate_section,
                    is_reexport: sym.is_reexport,
                };

                let costs = compute_name_sig_costs(
                    file_idx,
                    symbol_idx,
                    sym,
                    lines,
                    layout,
                );

                (key, costs)
            }).collect()
        })
        .collect();

    // Sequential reduce: merge into group map.
    let mut group_map: HashMap<GroupKey, Group> = HashMap::new();
    for file_result in file_results {
        for (key, costs) in file_result {
            let group = group_map.entry(key.clone()).or_insert_with(|| Group {
                key,
                symbols: Vec::new(),
                file_indices: HashSet::new(),
                max_doc_n: 0,
                max_body_n: 0,
            });
            group.file_indices.insert(costs.file_idx);
            group.symbols.push(costs);
        }
    }

    let mut groups: Vec<Group> = group_map.into_values().collect();
    groups.sort_by(|a, b| a.key.cmp(&b.key));

    // Phase 2 (parallel): compute doc/body line costs per group with budget-aware truncation.
    groups.par_iter_mut().for_each(|group| {
        fill_doc_body_costs(group, &all_lines, layouts, all_symbols, budget);
    });

    BuiltGroups { groups, budget }
}

/// Map a layer index `n` (1-based) to the source line index and total line count
/// for a symbol at that layer. Returns `None` if the symbol has no content at layer `n`.
fn layer_source_line(
    layout: &layout::SymbolLayout,
    sym_kind: parse::SymbolKind,
    is_doc: bool,
    n: usize,
) -> Option<(usize, usize)> {
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
        let (start, end) = if sym_kind == parse::SymbolKind::Section {
            (layout.md_content_start, layout.md_section_end)
        } else {
            (layout.body_start, layout.body_end)
        };
        let count = end.saturating_sub(start);
        if n - 1 < count {
            Some((start + (n - 1), count))
        } else {
            None
        }
    }
}

/// Token cost of the truncation marker that would appear after a given source line.
fn truncation_marker_cost(source_line: &str) -> Cost {
    let text = render::truncation_marker(source_line);
    Cost::of(&text)
}

/// Compute name + signature token costs for a symbol. Doc/body vectors are
/// left empty (filled per-group by `fill_doc_body_costs`).
fn compute_name_sig_costs(
    file_idx: usize,
    symbol_idx: usize,
    sym: &parse::Symbol,
    lines: &[&str],
    layout: &layout::SymbolLayout,
) -> SymbolCosts {
    let sym_line_0 = layout.sym_line_0;
    let is_section = sym.kind == parse::SymbolKind::Section;

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
        let pre_doc_count = layout.doc_end.saturating_sub(layout.doc_start);
        return SymbolCosts {
            file_idx,
            symbol_idx,
            name: Cost::of(&name_text),
            signature: Cost::default(),
            doc_lines: Vec::new(),
            doc_markers: Vec::new(),
            pre_doc_count,
            body_lines: Vec::new(),
            body_markers: Vec::new(),
            // Suppress separate truncation markers — composite " …" is inline
            // and already accounted for in name cost.
            body_has_nested: true,
        };
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

    let pre_doc_count = layout.doc_end.saturating_sub(layout.doc_start);

    SymbolCosts {
        file_idx,
        symbol_idx,
        name,
        signature,
        doc_lines: Vec::new(),
        doc_markers: Vec::new(),
        pre_doc_count,
        body_lines: Vec::new(),
        body_markers: Vec::new(),
        body_has_nested: layout.has_children,
    }
}

/// Phase 2: compute doc/body line token costs per group with budget-aware
/// truncation. Layers whose cumulative cost exceeds the budget are skipped
/// (their vector entries remain 0). `max_doc_n`/`max_body_n` are capped to
/// the last computed layer.
fn fill_doc_body_costs(
    group: &mut Group,
    all_lines: &[Vec<&str>],
    layouts: &[Vec<layout::SymbolLayout>],
    all_symbols: &[Vec<parse::Symbol>],
    budget: usize,
) {
    // Pre-allocate vectors to true lengths (0-filled). True `.len()` is needed
    // for truncation marker detection in `line_stage_cost`.
    let mut true_max_doc = 0usize;
    let mut true_max_body = 0usize;
    for sc in &mut group.symbols {
        let sym = &all_symbols[sc.file_idx][sc.symbol_idx];

        // Composite symbols: body lines = additional symbols on the shared line.
        let (doc_count, body_count) = if !sym.composed_prefix_lens.is_empty() {
            (0, sym.composed_prefix_lens.len().saturating_sub(1))
        } else {
            let layout = &layouts[sc.file_idx][sc.symbol_idx];
            let doc = layout.doc_end.saturating_sub(layout.doc_start)
                + layout.ds_end.saturating_sub(layout.ds_start);
            let (body_start, body_end) = if sym.kind == parse::SymbolKind::Section {
                (layout.md_content_start, layout.md_section_end)
            } else {
                (layout.body_start, layout.body_end)
            };
            (doc, body_end.saturating_sub(body_start))
        };

        sc.doc_lines = vec![Cost::default(); doc_count];
        sc.doc_markers = vec![Cost::default(); doc_count];
        sc.body_lines = vec![Cost::default(); body_count];
        sc.body_markers = vec![Cost::default(); body_count];
        true_max_doc = true_max_doc.max(doc_count);
        true_max_body = true_max_body.max(body_count);
    }

    // Base cost: names + signatures (already computed in phase 1).
    let base_cost: usize = group
        .symbols
        .iter()
        .map(|s| s.name.tokens + s.signature.tokens)
        .sum();
    let mut cumulative = base_cost;
    let mut computed_doc_n = 0usize;
    let mut computed_body_n = 0usize;

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
        let mut prev_markers = 0usize;

        // Byte-count threshold for skipping BPE: bytes/8 underestimates
        // tokens by ~2-4x (real ratio is ~3-4 bytes/token for o200k_base on
        // formatted code). If even this underestimate exceeds remaining budget,
        // the layer definitely exceeds it.
        const BYTES_PER_TOKEN: usize = 8;

        for n in 1..=true_max {
            let remaining = budget.saturating_sub(cumulative);
            let mut layer_token_cost = 0usize;
            let mut markers_at_n = 0usize;
            let mut layer_bytes = 0usize;
            let mut byte_estimate_exceeded = false;

            for sc in group.symbols.iter_mut() {
                let sym = &all_symbols[sc.file_idx][sc.symbol_idx];

                // Composite symbols: no doc, body lines extend the prefix to
                // include one more original symbol on the shared source line.
                if !sym.composed_prefix_lens.is_empty() {
                    if is_doc { continue; }
                    let pl = &sym.composed_prefix_lens;
                    let body_count = pl.len() - 1;
                    if n > body_count {
                        continue;
                    }
                    let file_lines = &all_lines[sc.file_idx];
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

                    sc.body_lines[n - 1] = Cost::new(lt, curr_end - prev_end);
                    // body_markers stays default: the inline " …" is already
                    // counted in name cost, and body_has_nested=true suppresses
                    // marker handling in line_stage_cost.

                    layer_token_cost += lt;
                    continue;
                }

                let layout = &layouts[sc.file_idx][sc.symbol_idx];
                let (src_line_idx, true_len) = match layer_source_line(layout, sym.kind, is_doc, n) {
                    Some(result) => result,
                    None => continue,
                };

                let file_lines = &all_lines[sc.file_idx];
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

                if is_doc {
                    sc.doc_lines[n - 1] = line_cost;
                    sc.doc_markers[n - 1] = marker_cost;
                } else {
                    sc.body_lines[n - 1] = line_cost;
                    sc.body_markers[n - 1] = marker_cost;
                }

                layer_token_cost += line_cost.tokens;

                // Truncation marker: shown when there are more lines beyond n.
                let show_marker = if is_doc {
                    // Suppress marker at the pre-comment/docstring split point.
                    !(true_len > sc.pre_doc_count && n == sc.pre_doc_count)
                } else {
                    // Suppress marker for symbols with nested children.
                    !sc.body_has_nested
                };
                if true_len > n && show_marker {
                    markers_at_n += marker_cost.tokens;
                }
            }

            if byte_estimate_exceeded {
                break;
            }

            let incremental = (layer_token_cost + markers_at_n).saturating_sub(prev_markers);
            cumulative += incremental;
            prev_markers = markers_at_n;

            if is_doc {
                computed_doc_n = n;
            } else {
                computed_body_n = n;
            }

            if cumulative > budget {
                break;
            }
        }
    }

    // Cap max_doc_n/max_body_n to the last computed layer. The scheduler uses
    // these to bound iteration; true vector lengths are preserved for marker
    // detection in line_stage_cost.
    group.max_doc_n = computed_doc_n;
    group.max_body_n = computed_body_n;
}
