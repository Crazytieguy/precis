//! Shared content vocabulary for `precis` batches. These types are the
//! data model both the North Star schema (see [`crate::north_star`]) and
//! the walker/render pipeline speak. No walker-implementation details.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A batch's content — FS listings or source line ranges. `kind` in TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum BatchContent {
    /// Directory listings.
    Fs { groups: Vec<FsGroup> },
    /// Source line ranges. Spans within a batch are disjoint on
    /// `(path, line)`; cross-batch overrides go through predecessors.
    Lines { spans: Vec<Span> },
}

/// One directory listing — parent directory + entries to show.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsGroup {
    pub parent: PathBuf,
    pub entries: FsEntries,
}

/// Children under an [`FsGroup`]'s parent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsEntries {
    /// Sentinel for "every immediate child" — `ns_loader` expands to
    /// `Listed`; walker output never carries `All`.
    All,
    Listed(Vec<PathBuf>),
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
///   Use this when the line is itself the content (a signature, a header,
///   a one-liner). When in doubt, `Full` is the right choice.
/// - `Truncated { pattern }`: emit only the regex match of `pattern`
///   against the source line, followed by a trailing `…`. The pattern
///   must match at least one character on every line the span covers.
///   The point is to *drop the meaningful tail* of the line — only use
///   when the match is materially shorter than the full line (e.g. show
///   `pub fn foo` from `pub fn foo(arg: Type) -> Result<…>`). If the
///   pattern matches all or most of the line, use `Full` instead;
///   `Truncated` saves no tokens there and reads as misuse.
/// - `Ellipsis`: emit a bare `…` marker at one line, with no line
///   number and no content. A later batch can replace it with `Full`
///   or `Truncated` at the same `(path, line)`. **Single-line only**:
///   `start` and `end` must be equal. A multi-line `Ellipsis` span
///   renders as one `…` marker at `start` and produces nothing for the
///   remaining lines — almost certainly not what the author intended.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Render {
    Full,
    Truncated { pattern: String },
    Ellipsis,
}

/// Expand a batch's spans into per-(path, line) entries, sorted by
/// `(path, line)`. Within one batch, spans must be disjoint —
/// `OverlappingSpans` is a validator-caught violation; this fn
/// `debug_assert`s on overlap and falls through to last-write-wins for
/// release-build robustness. Shared by `render::cost_spans` /
/// `render::apply_spans` / `divergence::atoms_from_content`.
pub(crate) fn explode_spans(spans: &[Span]) -> Vec<(PathBuf, usize, Render)> {
    let mut by_key: BTreeMap<(PathBuf, usize), Render> = BTreeMap::new();
    for span in spans {
        for line in span.start..=span.end {
            let prev = by_key.insert((span.path.clone(), line), span.render.clone());
            debug_assert!(
                prev.is_none(),
                "overlapping spans within one batch at {}:{line} — validator should have caught this",
                span.path.display()
            );
        }
    }
    by_key.into_iter().map(|((p, l), r)| (p, l, r)).collect()
}
