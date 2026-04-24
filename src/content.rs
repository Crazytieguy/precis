//! Shared content vocabulary for `precis` batches. These types are the
//! data model both the North Star schema (see [`crate::north_star`]) and
//! the walker/render pipeline speak. No walker-implementation details.

use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A batch's content: a set of filesystem listings, or a set of source
/// line ranges. Tagged `kind` in TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum BatchContent {
    /// Directory listings. Each group names a parent and the entries to
    /// show underneath it.
    Fs { groups: Vec<FsGroup> },
    /// Source line ranges with render specs. Within one batch, if two
    /// spans overlap on the same `(path, line)` the stronger render
    /// wins (see [`Render::priority`]).
    Lines { spans: Vec<Span> },
}

/// One directory listing: a parent directory and the entries to show
/// underneath it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsGroup {
    /// The parent directory.
    pub parent: PathBuf,
    /// Which children under `parent` to include.
    pub entries: FsEntries,
}

/// Selection of children under an [`FsGroup`]'s parent.
///
/// - `All` — every immediate child of `parent`.
/// - `Listed(names)` — the named children only. Each entry is a path
///   component relative to `parent` (typically just a filename).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsEntries {
    All,
    Listed(Vec<PathBuf>),
}

impl FsEntries {
    /// Access the concrete entry list. Returns `None` for `All`, which
    /// is a pre-resolution sentinel; post-resolution (NS load or walker
    /// output) callers should only see `Listed`.
    pub fn as_listed(&self) -> Option<&[PathBuf]> {
        match self {
            FsEntries::Listed(v) => Some(v),
            FsEntries::All => None,
        }
    }
}

impl Serialize for FsEntries {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            FsEntries::All => s.serialize_str("all"),
            FsEntries::Listed(paths) => paths.serialize(s),
        }
    }
}

impl<'de> Deserialize<'de> for FsEntries {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Str(String),
            Listed(Vec<PathBuf>),
        }
        match Repr::deserialize(d)? {
            Repr::Str(s) if s == "all" => Ok(FsEntries::All),
            Repr::Str(s) => Err(serde::de::Error::custom(format!(
                "invalid entries sentinel {s:?}; expected \"all\" or an array of paths"
            ))),
            Repr::Listed(v) => Ok(FsEntries::Listed(v)),
        }
    }
}

/// A contiguous range of source lines in one file plus how to render them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// Source file. Relative in NS input; absolutized at load time.
    pub path: PathBuf,
    /// Inclusive, 1-indexed.
    pub start: usize,
    /// Inclusive, 1-indexed. For a single line, equal to `start`.
    pub end: usize,
    /// How the lines are rendered.
    pub render: Render,
}

/// How to render a line.
///
/// - `Full`: emit the source line verbatim, with its line-number prefix.
/// - `Truncated { pattern }`: emit only the regex match of `pattern`
///   against the source line, followed by a trailing `…`. The pattern
///   must match at least one character on every line the span covers.
/// - `Ellipsis`: emit a bare `…` marker at that line, with no line
///   number and no content. A later batch can replace it with `Full`
///   or `Truncated` at the same `(path, line)`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Render {
    Full,
    Truncated { pattern: String },
    Ellipsis,
}

impl Render {
    /// Priority when two spans overlap on the same `(path, line)`:
    /// `Full` > `Truncated` > `Ellipsis`.
    pub(crate) fn priority(&self) -> u8 {
        match self {
            Render::Full => 3,
            Render::Truncated { .. } => 2,
            Render::Ellipsis => 1,
        }
    }
}
