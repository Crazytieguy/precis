//! Multi-language walker: emits filesystem listings (folders + files) plus
//! per-file content batches dispatched by extension. The single walker the
//! `Scheduler` runs in production. Per-language emission lives in sibling
//! modules (`rust`, `markdown`, `toml`).

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::Path;

use crate::batch::{Batch, BatchContent, BatchDraft, BatchId, EntryKind};

use super::{Walker, WalkerCtx, markdown, rust, toml};

pub struct MultiWalker;

impl MultiWalker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MultiWalker {
    fn default() -> Self {
        Self::new()
    }
}

impl Walker for MultiWalker {
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
        let mut out = Vec::new();
        for (name, kind) in children {
            let path = parent.join(name);
            match kind {
                EntryKind::Directory => {
                    let depth = ctx.depth_from_root(&path);
                    let sub_children = list_dir(&path);
                    out.push(BatchDraft {
                        content: BatchContent::FileSystemEntries {
                            parent: path,
                            children: sub_children,
                        },
                        value: folder_value(depth),
                    });
                }
                EntryKind::File => {
                    out.extend(file_batches(&path));
                }
            }
        }
        out
    }
}

fn folder_value(depth: usize) -> f64 {
    1000.0 / (1.0 + depth as f64 * 4.0)
}

fn file_batches(path: &Path) -> Vec<BatchDraft> {
    let Ok(source) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => rust::emit_batches(path, &source),
        Some("md") => markdown::emit_batches(path, &source),
        Some("toml") => toml::emit_batches(path, &source),
        _ => Vec::new(),
    }
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
