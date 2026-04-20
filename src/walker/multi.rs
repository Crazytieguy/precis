//! Multi-language walker: emits filesystem listings (folders + files) plus
//! per-file content batches dispatched by file extension.

use std::path::Path;

use crate::batch::{Batch, BatchContent, BatchDraft, BatchId};

use super::{
    Walker, WalkerCtx, folder_listing_draft, folder_value, list_dir, markdown, rust, toml,
};

#[derive(Default)]
pub struct MultiWalker;

impl Walker for MultiWalker {
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
        let mut out = Vec::new();
        for (name, kind) in children {
            let path = parent.join(name);
            match kind {
                crate::batch::EntryKind::Directory => {
                    let depth = ctx.depth_from_root(&path);
                    let sub_children = list_dir(&path);
                    out.push(folder_listing_draft(
                        path,
                        sub_children,
                        folder_value(depth),
                    ));
                }
                crate::batch::EntryKind::File => out.extend(file_batches(&path)),
            }
        }
        out
    }
}

/// Dispatch a single file to its language walker. Extension match happens
/// before reading the file so unsupported types (`Cargo.lock`, `LICENSE`,
/// images) don't pay an I/O + UTF-8 cost. Match is case-insensitive so
/// uppercase variants (`README.MD`, `Cargo.TOML`) work.
fn file_batches(path: &Path) -> Vec<BatchDraft> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);
    let walker: fn(&Path, &str) -> Vec<BatchDraft> = match ext.as_deref() {
        Some("rs") => rust::emit_batches,
        Some("md") => markdown::emit_batches,
        Some("toml") => toml::emit_batches,
        _ => return Vec::new(),
    };
    let Ok(source) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    walker(path, &source)
}
