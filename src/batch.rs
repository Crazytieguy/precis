use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

/// Opaque batch identity: an index into the scheduler's `batches: Vec<Batch>`.
/// The constructor is crate-private — only the scheduler hands these out — so
/// walkers cannot forge dangling, duplicate, or self-referential ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BatchId(usize);

impl BatchId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

/// Stored batch. `id` is the position in the scheduler's `Vec<Batch>` — it's
/// not a field on the struct because two storage sites for the same fact
/// would admit a "Batch::id ≠ index" mismatch.
#[derive(Debug, Clone)]
pub struct Batch {
    pub content: BatchContent,
    pub predecessor: Option<BatchId>,
    pub value: f64,
}

/// What walkers return: a Batch without identity. The scheduler stamps the
/// id (= next index) when it absorbs the draft. Successor drafts also have
/// their `predecessor` stamped automatically by the scheduler so a walker
/// cannot attach the wrong parent.
#[derive(Debug, Clone)]
pub struct BatchDraft {
    pub content: BatchContent,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub enum BatchContent {
    /// Add named children under `parent` to the rendered tree. `parent` must
    /// already exist in the tree (root, or a folder that some prior
    /// FileSystemEntries batch declared). The children-by-name map dedupes
    /// at the type level.
    FileSystemEntries {
        parent: PathBuf,
        children: BTreeMap<OsString, EntryKind>,
    },
    /// Add line content to one or more files. Each file path must already be
    /// declared as a `File` entry by a prior FileSystemEntries batch
    /// (debug-asserted at apply). Nested map dedupes both file paths and
    /// line numbers at the type level.
    Lines(BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
}

/// A rendered line: either the full source line or a prefix of it that was
/// truncated at a syntactic boundary. The variant carries the text; an empty
/// text string is debug-asserted at apply time.
#[derive(Debug, Clone)]
pub enum RenderedLine {
    Full(String),
    Truncated(String),
}

impl RenderedLine {
    pub fn text(&self) -> &str {
        match self {
            Self::Full(t) | Self::Truncated(t) => t,
        }
    }

    pub fn is_truncated(&self) -> bool {
        matches!(self, Self::Truncated(_))
    }
}
