use std::path::Path;

use crate::batch::{Batch, BatchId};

pub mod generic;
pub mod stub;

/// Allocator the scheduler hands to walkers when they emit batches, so each
/// new batch gets a fresh monotonic id.
pub struct WalkerCtx {
    next_id: u64,
}

impl WalkerCtx {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn alloc_id(&mut self) -> BatchId {
        let id = BatchId(self.next_id);
        self.next_id += 1;
        id
    }
}

impl Default for WalkerCtx {
    fn default() -> Self {
        Self::new()
    }
}

/// A walker discovers batches lazily: it emits seed batches for the input root
/// and, when a batch is scheduled, may emit successor batches whose structural
/// parent is the scheduled one.
pub trait Walker {
    fn seed(&mut self, root: &Path, ctx: &mut WalkerCtx) -> Vec<Batch>;
    fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch>;
}
