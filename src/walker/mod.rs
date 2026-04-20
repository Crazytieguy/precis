use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchDraft, BatchId};

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
