//! Group taxonomy: Folders, Files, and tree-sitter groups.
//!
//! A group is the scheduling and rendering unit. Groups carry items,
//! an inherited_modifier, and optional dependent_siblings.

pub mod files;
pub mod folders;
pub mod ts;

use std::path::{Path, PathBuf};

use crate::classify::FileRole;
use crate::render::CachedGroupRender;
use crate::store::ParseStore;

pub use ts::TsGroupKey;

/// Read-only context needed by group spawning.
pub struct GroupCtx<'s> {
    pub store: &'s ParseStore,
    pub root: PathBuf,
}

impl GroupCtx<'_> {
    pub fn rel_path<'a>(&self, path: &'a Path) -> &'a Path {
        path.strip_prefix(&self.root).unwrap_or(path)
    }
}

/// The three group variants (design §3.1).
pub enum Group<'s> {
    Folders(FoldersGroup),
    Files(FilesGroup),
    Ts(TsGroup<'s>),
}

/// A folder group — represents sub-directories of a parent, scheduled as a unit (D2, A3).
pub struct FoldersGroup {
    pub parent_dir: PathBuf,
    pub items: Vec<PathBuf>,
    pub inherited_modifier: f64,
    /// Per-item folder-line costs, computed once on first probe.
    pub cached_item_costs: Option<Vec<crate::render::FileCost>>,
}

/// A files group — represents files of a given role in a directory.
pub struct FilesGroup {
    pub parent_dir: PathBuf,
    pub role: FileRole,
    pub items: Vec<PathBuf>,
    pub inherited_modifier: f64,
    pub is_deprioritized: bool,
    pub is_type_declaration: bool,
    pub is_header: bool,
    pub is_test_file: bool,
}

/// A tree-sitter group — items extracted from parsed source.
pub struct TsGroup<'s> {
    pub key: TsGroupKey,
    pub items: Vec<TsItem<'s>>,
    pub inherited_modifier: f64,
    pub dependent_siblings: Vec<Group<'s>>,
    /// Cached render + cost, computed on first probe, reused thereafter.
    pub cached_render: Option<CachedGroupRender<'s>>,
}

/// A single item in a tree-sitter group.
pub struct TsItem<'s> {
    pub path: &'s Path,
    pub source: &'s str,
    pub node: tree_sitter::Node<'s>,
    pub end_line: usize,
}

impl TsItem<'_> {
    pub fn start_line(&self) -> usize {
        self.node.start_position().row
    }
}

impl<'s> Group<'s> {
    pub fn value(&self) -> f64 {
        match self {
            Group::Folders(g) => {
                g.inherited_modifier * crate::heuristics::folders_base_value(g.items.len())
            }
            Group::Files(g) => {
                g.inherited_modifier * crate::heuristics::files_base_value(g.items.len(), g.role)
            }
            Group::Ts(g) => {
                g.inherited_modifier * crate::heuristics::ts_base_value(&g.key, g.items.len())
            }
        }
    }

    /// First file path for tiebreaking (A1).
    pub fn first_path(&self) -> &Path {
        match self {
            Group::Folders(g) => g
                .items
                .first()
                .map(|p| p.as_path())
                .unwrap_or(&g.parent_dir),
            Group::Files(g) => g
                .items
                .first()
                .map(|p| p.as_path())
                .unwrap_or(&g.parent_dir),
            Group::Ts(g) => g.items.first().map(|i| i.path).unwrap_or(Path::new("")),
        }
    }

    /// First source line for tiebreaking (A1).
    pub fn first_line(&self) -> usize {
        match self {
            Group::Folders(_) | Group::Files(_) => 0,
            Group::Ts(g) => g.items.first().map(|i| i.start_line()).unwrap_or(0),
        }
    }

    /// Kind ordinal for tiebreaking (A1).
    pub fn kind_ordinal(&self) -> u32 {
        match self {
            Group::Folders(_) => 0,
            Group::Files(_) => 1,
            Group::Ts(g) => g.key.ordinal(),
        }
    }

    /// Produce children when this group is scheduled (A2).
    pub fn children<'a>(&'a mut self, ctx: &GroupCtx<'s>) -> Vec<Group<'s>>
    where
        's: 'a,
    {
        let children = match self {
            Group::Folders(g) => folders::children(g, ctx),
            Group::Files(g) => files::children(g, ctx),
            Group::Ts(g) => ts::children(g),
        };

        debug_assert!(
            children.iter().all(|g| match g {
                Group::Folders(f) => !f.items.is_empty(),
                Group::Files(f) => !f.items.is_empty(),
                Group::Ts(t) => !t.items.is_empty(),
            }),
            "D3: children() produced an empty group"
        );

        children
    }
}
