use std::path::Path;

use crate::batch::{Batch, BatchContent, FsEntry};

use super::{Walker, WalkerCtx};

/// Walks the filesystem and emits one FileSystemEntries batch per directory.
/// Files within a directory are listed by the parent's batch but get no
/// successor batch (no content walker yet — that's Stage 7).
pub struct GenericWalker;

impl GenericWalker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GenericWalker {
    fn default() -> Self {
        Self::new()
    }
}

impl Walker for GenericWalker {
    fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch> {
        let entries = list_dir(ctx.root());
        vec![Batch {
            id: ctx.alloc_id(),
            content: BatchContent::FileSystemEntries(entries),
            predecessor: None,
            value: folder_value(0),
        }]
    }

    fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
        let BatchContent::FileSystemEntries(entries) = &scheduled.content else {
            return vec![];
        };
        entries
            .iter()
            .filter(|e| e.is_dir)
            .map(|entry| {
                let depth = ctx.depth_from_root(&entry.path);
                let sub_entries = list_dir(&entry.path);
                Batch {
                    id: ctx.alloc_id(),
                    content: BatchContent::FileSystemEntries(sub_entries),
                    predecessor: Some(scheduled.id),
                    value: folder_value(depth),
                }
            })
            .collect()
    }
}

/// Heuristic value for a folder-listing batch: depth of the folder being
/// listed. Decreases with depth so the scheduler prefers shallower listings.
/// First-pass placeholder — calibrate later.
fn folder_value(depth: usize) -> f64 {
    1000.0 / (1.0 + depth as f64 * 4.0)
}

fn list_dir(path: &Path) -> Vec<FsEntry> {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut entries: Vec<FsEntry> = read_dir
        .flatten()
        .filter_map(|e| {
            let is_dir = e.file_type().ok()?.is_dir();
            Some(FsEntry {
                path: e.path(),
                is_dir,
            })
        })
        .collect();
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn seed_lists_root_children() {
        let mut walker = GenericWalker::new();
        let mut ctx = WalkerCtx::new(PathBuf::from("tests/fixtures/log"));
        let seeds = walker.seed(&mut ctx);
        assert_eq!(seeds.len(), 1);
        let BatchContent::FileSystemEntries(entries) = &seeds[0].content else {
            panic!("expected FileSystemEntries");
        };
        assert!(
            entries
                .iter()
                .any(|e| e.path.file_name().is_some_and(|n| n == "Cargo.toml"))
        );
    }
}
