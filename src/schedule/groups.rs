use std::collections::{HashMap, HashSet};
use std::path::Path;

use rayon::prelude::*;

use crate::Corpus;

use super::classify::{
    detect_heading_depth, is_autogen_api_doc, is_boilerplate_heading, is_config_file,
    is_generated_file, is_generated_filename, FileRole,
};
use super::cost;
use super::{
    BuiltGroups, Group, GroupKey, KindCategory, SymbolCosts,
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

                let costs = cost::compute_name_sig_costs(
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
        cost::fill_doc_body_costs(group, &all_lines, layouts, all_symbols, budget);
    });

    BuiltGroups { groups, budget }
}
