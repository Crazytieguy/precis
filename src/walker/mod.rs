use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchId};

pub mod generic;

/// Per-run state the scheduler hands to walkers when they emit batches:
/// monotonic id allocation and the seed root the run was started from.
pub struct WalkerCtx {
    next_id: usize,
    root: PathBuf,
}

impl WalkerCtx {
    pub fn new(root: PathBuf) -> Self {
        Self { next_id: 0, root }
    }

    pub fn alloc_id(&mut self) -> BatchId {
        let id = BatchId(self.next_id);
        self.next_id += 1;
        id
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

/// A walker discovers batches lazily: it emits seed batches for the run's root
/// and, when a batch is scheduled, may emit successor batches whose predecessor
/// is the scheduled one.
pub trait Walker {
    fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch>;
    fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch>;
}
