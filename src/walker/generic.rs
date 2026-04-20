use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::Path;

use crate::batch::{Batch, BatchContent, BatchDraft, BatchId, EntryKind};

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
    fn seed(&mut self, ctx: &WalkerCtx) -> Vec<BatchDraft> {
        let parent = ctx.root().to_path_buf();
        let children = list_dir(&parent);
        vec![BatchDraft {
            content: BatchContent::FileSystemEntries { parent, children },
            value: folder_value(0),
        }]
    }

    fn successors(
        &mut self,
        _scheduled_id: BatchId,
        scheduled: &Batch,
        ctx: &WalkerCtx,
    ) -> Vec<BatchDraft> {
        let BatchContent::FileSystemEntries { parent, children } = &scheduled.content else {
            return Vec::new();
        };
        children
            .iter()
            .filter_map(|(name, kind)| match kind {
                EntryKind::Directory => {
                    let sub_path = parent.join(name);
                    let depth = ctx.depth_from_root(&sub_path);
                    let sub_children = list_dir(&sub_path);
                    Some(BatchDraft {
                        content: BatchContent::FileSystemEntries {
                            parent: sub_path,
                            children: sub_children,
                        },
                        value: folder_value(depth),
                    })
                }
                EntryKind::File => None,
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

fn list_dir(path: &Path) -> BTreeMap<OsString, EntryKind> {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn seed_lists_root_children() {
        let mut walker = GenericWalker::new();
        let ctx = WalkerCtx::new(PathBuf::from("tests/fixtures/log"));
        let seeds = walker.seed(&ctx);
        assert_eq!(seeds.len(), 1);
        let BatchContent::FileSystemEntries { children, .. } = &seeds[0].content else {
            panic!("expected FileSystemEntries");
        };
        assert!(children.contains_key(&OsString::from("Cargo.toml")));
    }
}
