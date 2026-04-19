use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BatchId(pub u64);

#[derive(Debug, Clone)]
pub struct Batch {
    pub id: BatchId,
    pub descriptor: String,
    pub content: BatchContent,
    /// Structural parent — must be scheduled before this batch enters the frontier.
    pub parent: Option<BatchId>,
    /// Optional ordering predecessor (e.g., public-fns before private-fns).
    pub ordering_pred: Option<BatchId>,
    /// Base value before any size-aware adjustment. First-pass scheduling is linear.
    pub raw_value: f64,
}

impl Batch {
    pub fn predecessors(&self) -> impl Iterator<Item = BatchId> {
        self.parent.into_iter().chain(self.ordering_pred)
    }
}

#[derive(Debug, Clone)]
pub enum BatchContent {
    /// Folder's children listing (subfolders + files by name).
    FolderListing {
        path: PathBuf,
        entries: Vec<FsEntry>,
    },
    /// Lines added to a file. Lines may replace previously rendered lines at
    /// the same line number (line-level override).
    FileContent {
        path: PathBuf,
        lines: Vec<RenderedLine>,
    },
}

impl BatchContent {
    pub fn target_path(&self) -> &PathBuf {
        match self {
            BatchContent::FolderListing { path, .. } => path,
            BatchContent::FileContent { path, .. } => path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FsEntry {
    pub name: String,
    pub is_dir: bool,
}

#[derive(Debug, Clone)]
pub struct RenderedLine {
    pub number: usize,
    pub text: String,
    /// True if `text` is a prefix of the source line and a trailing ellipsis should be rendered.
    pub truncated: bool,
}
