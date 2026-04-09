use std::collections::{HashMap, HashSet};
use std::path::Path;

use rayon::prelude::*;

use crate::Corpus;

use super::classify::{detect_heading_depth, is_boilerplate_heading};
use super::cost;
use super::value;
use super::{
    BuiltGroups, Cost, Group, GroupKey, KindCategory, SymbolRef,
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
    let files = corpus.files;

    // Pre-compute lines for all files (needed for deferred doc/body tokenization).
    let all_lines: Vec<Vec<&str>> = files
        .iter()
        .map(|f| {
            f.source
                .as_ref()
                .map(|s| s.lines().collect())
                .unwrap_or_default()
        })
        .collect();

    // Phase 1 (parallel): compute GroupKey + name/signature costs per file.
    let file_results: Vec<Vec<(GroupKey, SymbolRef, Cost, Cost)>> = (0..files.len())
        .into_par_iter()
        .map(|file_idx| {
            let fd = &files[file_idx];
            if fd.source.is_none() {
                return vec![];
            }
            let symbols = &fd.symbols;
            let lines = &all_lines[file_idx];
            let fi = &fd.info;

            symbols.iter().enumerate().map(|(symbol_idx, sym)| {
                let sym_line_0 = sym.line - 1;
                let kind_category = KindCategory::from_symbol_kind(sym.kind);

                // For imports, doc comments don't change value — the import line
                // itself is what matters. Force is_documented=false to keep all
                // imports in one group (avoids splitting __init__.py re-exports
                // into documented/undocumented subsets).
                let is_documented = kind_category != KindCategory::Import
                    && sym.layout.doc_start < sym.layout.doc_end;

                let heading_depth = if kind_category == KindCategory::Section {
                    if matches!(fi.lang, Some(crate::Lang::Toml)) {
                        // Count dot-separated segments: [project] → 1, [tool.ruff] → 2
                        let depth = sym.name.chars().filter(|&c| c == '.').count() as u8 + 1;
                        Some(depth)
                    } else if matches!(fi.lang, Some(crate::Lang::Markdown)) {
                        Some(detect_heading_depth(lines, sym_line_0, sym.end_line))
                    } else {
                        Some(1) // JSON/YAML/unsupported: all top-level
                    }
                } else {
                    None
                };

                let is_boilerplate_section = kind_category == KindCategory::Section
                    && matches!(fi.lang, Some(crate::Lang::Markdown))
                    && is_boilerplate_heading(&sym.name);

                // TOML [tool.*] sections are tool configuration (linters, formatters,
                // type checkers, test runners) embedded in project manifests. They're
                // equivalent to dedicated config files like eslint.config.js, and should
                // be deprioritized similarly.
                let is_config = fi.is_config
                    || (kind_category == KindCategory::Section
                        && matches!(fi.lang, Some(crate::Lang::Toml))
                        && sym.name.starts_with("tool."));

                let key = GroupKey {
                    is_public: sym.is_public,
                    kind_category,
                    parent_dir: fi.relative_path.parent().unwrap_or(Path::new("")).to_path_buf(),
                    is_documented,
                    file_role: fi.file_role,
                    file_category: fi.file_category,
                    is_type_declaration: fi.is_type_declaration,
                    is_header: fi.is_header,
                    is_generated: fi.is_generated,
                    is_config,
                    heading_depth,
                    is_first_party: sym.is_first_party,
                    is_trait_impl: sym.is_trait_impl,
                    is_boilerplate_section,
                    is_reexport: sym.is_reexport,
                };

                let (sref, name_cost, sig_cost) = cost::compute_name_sig_costs(
                    file_idx,
                    symbol_idx,
                    sym,
                    lines,
                );

                (key, sref, name_cost, sig_cost)
            }).collect()
        })
        .collect();

    // Sequential reduce: merge into group map, accumulating name/sig costs.
    let mut group_map: HashMap<GroupKey, Group> = HashMap::new();
    for file_result in file_results {
        for (key, sref, name_cost, sig_cost) in file_result {
            let group = group_map.entry(key.clone()).or_insert_with(|| {
                let base_importance = value::compute_base_importance(&key);
                Group {
                    key,
                    symbols: Vec::new(),
                    file_indices: HashSet::new(),
                    base_importance,
                    names_cost: Cost::default(),
                    signatures_cost: Cost::default(),
                    cumulatives: Default::default(),
                }
            });
            group.file_indices.insert(sref.file_idx);
            group.names_cost += name_cost;
            group.signatures_cost += sig_cost;
            group.symbols.push(sref);
        }
    }

    let mut groups: Vec<Group> = group_map.into_values().collect();
    groups.sort_by(|a, b| a.key.cmp(&b.key));

    // Phase 2 (parallel): build cumulative prefix sums per group with budget-aware truncation.
    groups.par_iter_mut().for_each(|group| {
        cost::build_cumulatives(group, &all_lines, files, budget);
    });

    BuiltGroups { groups, budget }
}
