use std::path::PathBuf;

/// Index into the scheduler's `batches: Vec<Batch>`. Walkers receive a
/// monotonic allocator from `WalkerCtx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BatchId(pub usize);

/// A batch is the atomic scheduling unit. Either-or-nothing: the scheduler
/// places its full content in the rendered tree, or none of it.
#[derive(Debug, Clone)]
pub struct Batch {
    pub id: BatchId,
    pub content: BatchContent,
    /// Logical predecessor: must be scheduled before this batch becomes
    /// eligible. First-pass keeps this single (covers both "structural parent"
    /// and "ordering" cases — to enforce A-before-B between siblings, make B's
    /// predecessor be A directly). Stage 7 may reintroduce a richer model.
    pub predecessor: Option<BatchId>,
    /// Heuristic value. The scheduler picks by `value / marginal_cost.tokens`.
    pub value: f64,
}

#[derive(Debug, Clone)]
pub enum BatchContent {
    /// Declare these filesystem entries (files and/or folders) as part of the
    /// rendered tree. Each entry is added to its parent path's listed-children
    /// set; the parent must already be in the tree (typically via a prior
    /// FileSystemEntries batch on the parent folder).
    FileSystemEntries(Vec<FsEntry>),
    /// Add line content to one or more files. The file paths must already be
    /// declared via a prior FileSystemEntries batch (otherwise the content is
    /// orphaned and won't render — debug-asserted).
    Lines(Vec<FileLineSet>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsEntry {
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Debug, Clone)]
pub struct FileLineSet {
    pub path: PathBuf,
    pub lines: Vec<RenderedLine>,
}

#[derive(Debug, Clone)]
pub struct RenderedLine {
    pub number: usize,
    pub text: String,
    /// Render with a trailing ellipsis (the line was truncated at a syntactic
    /// boundary; the marker invites the reader to follow up with a Read).
    pub truncated: bool,
}
