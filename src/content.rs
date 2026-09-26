//! Shared content vocabulary for `precis` batches. These types are the
//! data model both the North Star schema (see [`crate::north_star`]) and
//! the walker/render pipeline speak. No walker-implementation details.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use serde::{Deserialize, Deserializer};

/// A batch's content — FS listings or source line ranges. `kind` in TOML.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum BatchContent {
    /// Directory listings.
    Fs { groups: Vec<FsGroup> },
    /// Source line ranges. Spans within a batch are disjoint on
    /// `(path, line)`; cross-batch overrides go through predecessors.
    Lines {
        spans: Vec<Span>,
        /// Row groups a terminal partial takes whole, in the order it
        /// takes them, for a batch whose spans are all in one file. A span
        /// row in no group is a blank bridge, kept only between two taken
        /// rows. Empty: each row is its own group, in `(path, line)` order.
        #[serde(skip)]
        units: Vec<Vec<usize>>,
    },
}

/// One directory listing — parent directory + entries to show.
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
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
///   a one-liner). When in doubt, `Full` is the right choice. A line
///   over 500 characters renders its first 500 and a trailing `…`.
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
///   `start` and `end` must be equal. A multi-line `Ellipsis` span is
///   rejected by the validator: it would render
///   one `…` marker per covered line — redundant noise. Use one
///   single-line `Ellipsis` span per line, or `Full`/`Truncated` if
///   the lines should render.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Render {
    Full,
    Truncated { pattern: String },
    Ellipsis,
}

thread_local! {
    /// Per-thread memo of compiled [`Render::Truncated`] patterns — a
    /// handful of distinct patterns recur across thousands of spans.
    static TRUNCATE_RE_CACHE: RefCell<HashMap<String, Option<regex::Regex>>> =
        RefCell::new(HashMap::new());
}

/// Run `f` with the compiled regex for a [`Render::Truncated`]
/// pattern, memoized per thread. `None` for an invalid pattern —
/// validation rejects those upstream; callers degrade to a zero-width
/// match. Closure-based so the cached `Regex` (and its lazy-DFA
/// scratch pool) is borrowed rather than cloned per call; `f` must not
/// re-enter this cache.
pub(crate) fn with_truncate_regex<R>(
    pattern: &str,
    f: impl FnOnce(Option<&regex::Regex>) -> R,
) -> R {
    TRUNCATE_RE_CACHE.with(|c| {
        let mut map = c.borrow_mut();
        if !map.contains_key(pattern) {
            map.insert(pattern.to_string(), regex::Regex::new(pattern).ok());
        }
        f(map[pattern].as_ref())
    })
}

/// Expand a batch's spans into per-(path, line) entries, sorted by
/// `(path, line)`. Spans in one batch must be disjoint, which the NS
/// validator checks; this fn `debug_assert`s on overlap and falls
/// through to last-write-wins for release-build robustness.
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
