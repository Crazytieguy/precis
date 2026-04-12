//! Group taxonomy: Folders, Files, and tree-sitter groups.
//!
//! A group is the scheduling and rendering unit. Groups carry items,
//! an inherited_modifier, and optional dependent_siblings.

pub mod files;
pub mod folders;
pub mod ts;

use std::path::{Path, PathBuf};

use crate::classify::{FileCategory, FileRole};
use crate::render::{CachedGroupRender, LineEntry};
use crate::schedule::ScheduleCtx;

pub use ts::TsGroupKey;

/// The three group variants (design §3.1).
pub enum Group<'s> {
    Folders(FoldersGroup),
    Files(FilesGroup),
    Ts(TsGroup<'s>),
}

/// A folder group — represents sub-directories of a parent.
pub struct FoldersGroup {
    pub parent_dir: PathBuf,
    pub inherited_modifier: f64,
    pub category: FileCategory,
}

/// A files group — represents files of a given role in a directory.
pub struct FilesGroup {
    pub parent_dir: PathBuf,
    pub role: FileRole,
    pub items: Vec<PathBuf>,
    pub inherited_modifier: f64,
    pub is_config: bool,
    pub is_type_declaration: bool,
    pub is_header: bool,
    pub is_generated: bool,
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
    pub path: PathBuf,
    pub source: &'s str,
    pub node: tree_sitter::Node<'s>,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
}

impl<'s> Group<'s> {
    pub fn value(&self) -> f64 {
        match self {
            Group::Folders(g) => {
                g.inherited_modifier * crate::heuristics::folders_base_value()
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
            Group::Folders(g) => &g.parent_dir,
            Group::Files(g) => g.items.first().map(|p| p.as_path()).unwrap_or(&g.parent_dir),
            Group::Ts(g) => g
                .items
                .first()
                .map(|i| i.path.as_path())
                .unwrap_or(Path::new("")),
        }
    }

    /// First source line for tiebreaking (A1).
    pub fn first_line(&self) -> usize {
        match self {
            Group::Folders(_) | Group::Files(_) => 0,
            Group::Ts(g) => g.items.first().map(|i| i.start_line).unwrap_or(0),
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
    pub fn children<'a>(&'a mut self, ctx: &ScheduleCtx<'s>) -> Vec<Group<'s>>
    where
        's: 'a,
    {
        match self {
            Group::Folders(g) => folders::children(g, ctx),
            Group::Files(g) => files::children(g, ctx),
            Group::Ts(g) => ts::children(g, ctx),
        }
    }

    /// Render this group's contribution as LineEntries.
    /// For TsGroups, this is called once and cached.
    pub fn render_entries(&mut self, ctx: &ScheduleCtx<'s>) -> Vec<(PathBuf, Vec<LineEntry<'s>>)> {
        match self {
            Group::Folders(g) => folders::render(g),
            Group::Files(g) => files::render(g),
            Group::Ts(g) => ts::render_entries(g, ctx),
        }
    }
}
