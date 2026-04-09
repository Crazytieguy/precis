use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use crate::Corpus;

use super::{BuiltGroups, FileRole, StageCumulatives, StageKind};

// ---------------------------------------------------------------------------
// Render plan — pre-resolved output structure for the renderer
// ---------------------------------------------------------------------------

/// Pre-resolved rendering parameters for a single symbol.
/// Computed by the scheduler, consumed by the renderer. The renderer doesn't
/// need to know about stages, groups, or kind categories — just these four
/// concrete decisions about what to show for each symbol.
#[derive(Debug, Clone, Copy)]
pub struct SymbolRenderSpec {
    /// Show the symbol name (false only for FilePath-only groups).
    pub show_name: bool,
    /// Show full signature beyond the name line.
    pub show_sig: bool,
    /// Number of doc comment lines to show. 0=none, usize::MAX=all.
    pub doc_lines: usize,
    /// Number of body lines to show. 0=none, usize::MAX=all.
    pub body_lines: usize,
}

/// An item in the render plan — the ordered sequence of things to output.
#[derive(Debug)]
pub enum RenderPlanItem {
    /// Omission marker for an invisible top-level directory.
    DirectoryMarker(PathBuf),
    /// A visible file (header line + its symbols).
    File(usize),
}

/// The result of scheduling: a render plan (ordered output structure) and
/// per-symbol render specs (concrete rendering decisions).
pub struct Schedule {
    /// Ordered list of files and directory markers to render.
    pub render_plan: Vec<RenderPlanItem>,
    /// Per-symbol render specs, indexed as `[file_idx][sym_idx]`.
    /// `None` means the symbol is not rendered.
    pub symbol_specs: Vec<Vec<Option<SymbolRenderSpec>>>,
}

// ---------------------------------------------------------------------------
// Directory marker format — single source of truth
// ---------------------------------------------------------------------------

/// Format the content line for a directory omission marker.
/// Does NOT include the leading separator newline — callers add that
/// following the same convention as file headers (the renderer adds
/// `\n` conditionally, the solver includes it unconditionally for cost).
pub(crate) fn directory_marker_text(dir: &Path) -> String {
    format!("{}/\n", dir.display())
}

/// Extract the top-level directory of a path relative to the project root.
/// Returns `None` for root-level files (no parent directory).
pub(super) fn top_level_dir(relative: &Path) -> Option<PathBuf> {
    let mut components = relative.components();
    let first = components.next()?;
    // Must have at least one more component to be in a subdirectory
    components.next()?;
    Some(PathBuf::from(first.as_os_str()))
}

// ---------------------------------------------------------------------------
// Render plan construction
// ---------------------------------------------------------------------------

/// Build a concrete render plan from solver decisions.
///
/// Takes the raw solver output (per-group inclusion positions and shown files)
/// and converts it into an ordered render plan with per-symbol specs. The
/// solver knows nothing about file ordering or directory marker placement —
/// those presentation concerns are handled entirely here.
pub(super) fn build_render_plan(
    result: &super::SolverResult,
    built: &BuiltGroups,
    corpus: &Corpus<'_>,
) -> Schedule {
    let groups = &built.groups;
    let files = corpus.files;

    // 1. Resolve per-symbol render specs from group positions.
    let mut symbol_specs: Vec<Vec<Option<SymbolRenderSpec>>> = files
        .iter()
        .map(|f| vec![None; f.symbols.len()])
        .collect();

    for (group_idx, group) in groups.iter().enumerate() {
        let pos = match result.group_positions[group_idx] {
            Some(p) => p,
            None => continue,
        };
        let stages = group.key.kind_category.stage_sequence();
        let spec = resolve_render_spec(pos, &group.cumulatives, stages);
        for sc in &group.symbols {
            symbol_specs[sc.file_idx][sc.symbol_idx] = Some(spec);
        }
    }

    // 2. Compute file render order: README first, manifests second, alphabetical.
    let mut render_order: Vec<usize> = result.files_shown.iter().copied().collect();
    render_order.sort_by_key(|&i| {
        let fi = &files[i].info;
        let is_root = fi.relative_path.parent().is_none_or(|p| p.as_os_str().is_empty());
        let filename = fi.relative_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let priority = if is_root && matches!(fi.file_role, FileRole::Readme | FileRole::Architecture) {
            0
        } else if is_root
            && matches!(
                filename,
                "Cargo.toml" | "package.json" | "go.mod" | "pyproject.toml" | "setup.cfg"
            )
        {
            1
        } else {
            2
        };
        // Within each priority tier, sort alphabetically by relative path
        // for deterministic output.
        (priority, &fi.relative_path)
    });

    // 3. Compute invisible directory markers (two-set approach).
    let invisible_dirs: BTreeSet<PathBuf> = {
        let mut all_dirs = HashSet::new();
        let mut visible_dirs = HashSet::new();
        for (i, f) in files.iter().enumerate() {
            if let Some(top) = top_level_dir(&f.info.relative_path) {
                all_dirs.insert(top.clone());
                if result.files_shown.contains(&i) {
                    visible_dirs.insert(top);
                }
            }
        }
        all_dirs.difference(&visible_dirs).cloned().collect()
    };

    // 4. Merge file order and directory markers into render plan.
    let mut render_plan: Vec<RenderPlanItem> = Vec::new();
    let mut dirs_emitted: HashSet<PathBuf> = HashSet::new();

    for &file_idx in &render_order {
        let relative = &files[file_idx].info.relative_path;
        // Emit invisible directory markers that sort before this file's top-level dir.
        if let Some(ftd) = relative.components().next().map(|c| PathBuf::from(c.as_os_str())) {
            for dir in &invisible_dirs {
                if dir < &ftd && dirs_emitted.insert(dir.clone()) {
                    render_plan.push(RenderPlanItem::DirectoryMarker(dir.clone()));
                }
            }
        }
        render_plan.push(RenderPlanItem::File(file_idx));
    }
    // Emit any remaining invisible directories after the last file.
    for dir in &invisible_dirs {
        if dirs_emitted.insert(dir.clone()) {
            render_plan.push(RenderPlanItem::DirectoryMarker(dir.clone()));
        }
    }

    Schedule {
        render_plan,
        symbol_specs,
    }
}

/// Resolve a cumulative position into a concrete `SymbolRenderSpec`.
///
/// A stage is covered if it appears at or before the final entry's stage
/// in the kind's stage sequence. For Doc/Body: if the final entry IS that
/// stage, render at `n` lines; if it's a prerequisite (before the final
/// stage), render fully (`usize::MAX`); otherwise 0.
fn resolve_render_spec(
    pos: usize,
    cumulatives: &StageCumulatives,
    stages: &[StageKind],
) -> SymbolRenderSpec {
    let entry = &cumulatives.entries[pos];
    let final_kind = entry.stage;
    let final_n = entry.n;
    let final_seq_pos = stages.iter().position(|&s| s == final_kind).unwrap();

    let is_covered = |sk: StageKind| -> bool {
        stages.iter().position(|&s| s == sk)
            .is_some_and(|p| p <= final_seq_pos)
    };

    let show_name = is_covered(StageKind::Names);
    let show_sig = is_covered(StageKind::Signatures);

    let doc_lines = if final_kind == StageKind::Doc {
        final_n
    } else if is_covered(StageKind::Doc) {
        usize::MAX
    } else {
        0
    };

    let body_lines = if final_kind == StageKind::Body {
        final_n
    } else if is_covered(StageKind::Body) {
        usize::MAX
    } else {
        0
    };

    SymbolRenderSpec { show_name, show_sig, doc_lines, body_lines }
}
