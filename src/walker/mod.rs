use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchContent, BatchDraft, BatchId, EntryKind, RenderedLine};

pub mod generic;
pub mod markdown;
pub mod multi;
pub mod rust;
pub mod toml;

/// Per-run, walker-visible state. Only carries the seed root and depth helper —
/// no id allocation: the scheduler stamps ids and predecessors when it
/// absorbs walker-emitted drafts.
pub struct WalkerCtx {
    root: PathBuf,
}

impl WalkerCtx {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Depth of `path` relative to the seed root (root itself = 0). Returns 0
    /// when `path` is not under root — that's a walker bug, but the contract
    /// is to make it visible via wrong indentation rather than panic mid-run.
    pub fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }
}

/// A walker discovers batches lazily. Walkers return drafts; the scheduler
/// assigns ids and stamps `predecessor` (None for seeds, Some(scheduled_id)
/// for successors), so a walker cannot emit a forward reference, a cycle,
/// or a successor with the wrong parent.
pub trait Walker {
    fn seed(&mut self, ctx: &WalkerCtx) -> Vec<BatchDraft>;
    fn successors(
        &mut self,
        scheduled_id: BatchId,
        scheduled: &Batch,
        ctx: &WalkerCtx,
    ) -> Vec<BatchDraft>;
}

// ---------- shared helpers ----------

/// Heuristic value for a folder-listing batch at a given depth from root.
/// Decreases with depth so the scheduler prefers shallower listings before
/// deeper ones. First-pass placeholder — calibrate later.
pub(crate) fn folder_value(depth: usize) -> f64 {
    1000.0 / (1.0 + depth as f64 * 4.0)
}

/// Read a directory's immediate children into a name-keyed map.
pub(crate) fn list_dir(path: &Path) -> BTreeMap<OsString, EntryKind> {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return BTreeMap::new();
    };
    read_dir
        .flatten()
        .filter_map(|e| {
            let kind = if e.file_type().ok()?.is_dir() {
                EntryKind::Directory
            } else {
                EntryKind::File
            };
            Some((e.file_name(), kind))
        })
        .collect()
}

/// Build a per-file Lines `BatchDraft` from `(line_number, RenderedLine)` pairs.
/// Empty-text lines are dropped (they'd trip the renderer's debug-assert and
/// rendering them as just a numbered prefix is noise). Returns `None` when
/// the resulting line set is empty.
pub(crate) fn lines_for_file(
    path: &Path,
    lines: impl IntoIterator<Item = (usize, RenderedLine)>,
    value: f64,
) -> Option<BatchDraft> {
    let inner: BTreeMap<usize, RenderedLine> = lines
        .into_iter()
        .filter(|(_, l)| !l.text().is_empty())
        .collect();
    if inner.is_empty() {
        return None;
    }
    let mut file_map: BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>> = BTreeMap::new();
    file_map.insert(path.to_path_buf(), inner);
    Some(BatchDraft {
        content: BatchContent::Lines(file_map),
        value,
    })
}

/// Build a folder-listing `BatchDraft` for `path` with the given children and
/// value. Convenience that matches how walkers actually emit folder batches.
pub(crate) fn folder_listing_draft(
    path: PathBuf,
    children: BTreeMap<OsString, EntryKind>,
    value: f64,
) -> BatchDraft {
    BatchDraft {
        content: BatchContent::FileSystemEntries {
            parent: path,
            children,
        },
        value,
    }
}
