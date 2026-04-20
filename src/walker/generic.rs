//! Filesystem-only walker. One `FileSystemEntries` batch per directory; no
//! per-file content batches (the `MultiWalker` does that). Kept around as a
//! thin Walker the scheduler invariant tests' inline walkers can mirror.

use crate::batch::{Batch, BatchContent, BatchDraft, BatchId, EntryKind};

use super::{Walker, WalkerCtx, folder_listing_draft, folder_value, list_dir};

#[derive(Default)]
pub struct GenericWalker;

impl Walker for GenericWalker {
    fn seed(&mut self, ctx: &WalkerCtx) -> Vec<BatchDraft> {
        let parent = ctx.root().to_path_buf();
        let children = list_dir(&parent);
        vec![folder_listing_draft(parent, children, folder_value(0))]
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
            .filter_map(|(name, kind)| {
                if !matches!(kind, EntryKind::Directory) {
                    return None;
                }
                let sub_path = parent.join(name);
                let depth = ctx.depth_from_root(&sub_path);
                let sub_children = list_dir(&sub_path);
                Some(folder_listing_draft(
                    sub_path,
                    sub_children,
                    folder_value(depth),
                ))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn seed_lists_root_children() {
        let mut walker = GenericWalker;
        let ctx = WalkerCtx::new(PathBuf::from("tests/fixtures/log"));
        let seeds = walker.seed(&ctx);
        assert_eq!(seeds.len(), 1);
        let BatchContent::FileSystemEntries { children, .. } = &seeds[0].content else {
            panic!("expected FileSystemEntries");
        };
        assert!(children.contains_key(&OsString::from("Cargo.toml")));
    }
}
