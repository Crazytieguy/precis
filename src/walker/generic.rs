use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchContent, FsEntry};

use super::{Walker, WalkerCtx};

/// Walks the filesystem and emits one folder-listing batch per directory.
/// Files within a directory are listed by the parent's batch but get no
/// successor batch (no content walker yet — that's Stage 7).
pub struct GenericWalker {
    root: Option<PathBuf>,
}

impl GenericWalker {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn depth_from_root(&self, path: &Path) -> usize {
        match &self.root {
            Some(root) => path
                .strip_prefix(root)
                .map(|p| p.components().count())
                .unwrap_or(0),
            None => 0,
        }
    }
}

impl Default for GenericWalker {
    fn default() -> Self {
        Self::new()
    }
}

impl Walker for GenericWalker {
    fn seed(&mut self, root: &Path, ctx: &mut WalkerCtx) -> Vec<Batch> {
        self.root = Some(root.to_path_buf());
        let entries = list_dir(root);
        vec![Batch {
            id: ctx.alloc_id(),
            descriptor: format!("listing of {}", root.display()),
            content: BatchContent::FolderListing {
                path: root.to_path_buf(),
                entries,
            },
            parent: None,
            ordering_pred: None,
            raw_value: folder_value(0),
        }]
    }

    fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
        let BatchContent::FolderListing { path, entries } = &scheduled.content else {
            return vec![];
        };
        let depth = self.depth_from_root(path);
        let mut out = Vec::new();
        for entry in entries {
            if !entry.is_dir {
                continue;
            }
            let sub_path = path.join(&entry.name);
            let sub_entries = list_dir(&sub_path);
            out.push(Batch {
                id: ctx.alloc_id(),
                descriptor: format!("listing of {}", sub_path.display()),
                content: BatchContent::FolderListing {
                    path: sub_path,
                    entries: sub_entries,
                },
                parent: Some(scheduled.id),
                ordering_pred: None,
                raw_value: folder_value(depth + 1),
            });
        }
        out
    }
}

/// Heuristic value for a folder-listing batch at a given depth from root.
/// Decreases with depth so the scheduler prefers shallower listings before
/// deeper ones, all else equal. First-pass placeholder — calibrate later.
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
            let name = e.file_name().to_string_lossy().to_string();
            let is_dir = e.file_type().ok()?.is_dir();
            Some(FsEntry { name, is_dir })
        })
        .collect();
    entries.sort();
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_lists_root_children() {
        let mut walker = GenericWalker::new();
        let mut ctx = WalkerCtx::new();
        let seeds = walker.seed(Path::new("test/fixtures/log"), &mut ctx);
        assert_eq!(seeds.len(), 1);
        let BatchContent::FolderListing { entries, .. } = &seeds[0].content else {
            panic!("expected FolderListing");
        };
        assert!(entries.iter().any(|e| e.name == "Cargo.toml"));
    }
}
