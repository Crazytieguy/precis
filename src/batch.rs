//! Batch data types. A **batch** is the atomic scheduling unit: a named subset
//! of source content the walker proposes to render. The scheduler picks
//! batches greedily by value/cost ratio within a token budget.
//!
//! Two layers of identity:
//!
//! - [`BatchKey`] is a **semantic, walker-defined** name for a batch
//!   (e.g. `Rust(RustKey::PubDecls { src_dir })`). Walkers use keys to refer
//!   to each other's batches, including predecessor edges that cross files.
//!   Keys are emitted during `expand()` before their content has been parsed;
//!   they're stable through materialization.
//! - [`BatchId`] is the scheduler's internal index assigned when a batch is
//!   *materialized*. Walkers never see `BatchId`s.
//!
//! The key/id split is what makes lazy materialization and cross-file
//! batches possible: a walker can emit "rustdoc refinement has `PubDecls` as
//! predecessor" before either has been parsed, by naming keys.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

/// Scheduler-internal batch index. Assigned at materialization time; walkers
/// never see these.
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

/// Semantic batch identity. Named by the walker that produces it. Stable
/// across materialization — the same key always refers to the same batch.
/// Walkers declare predecessor edges by naming `BatchKey`s, so they can be
/// set up before any file is parsed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BatchKey {
    Fs(FsKey),
    Rust(RustKey),
    Markdown(MarkdownKey),
    Toml(TomlKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FsKey {
    /// Listing of immediate children of `dir`.
    DirListing { dir: PathBuf },
}

impl From<FsKey> for BatchKey {
    fn from(k: FsKey) -> Self {
        BatchKey::Fs(k)
    }
}
impl From<RustKey> for BatchKey {
    fn from(k: RustKey) -> Self {
        BatchKey::Rust(k)
    }
}
impl From<MarkdownKey> for BatchKey {
    fn from(k: MarkdownKey) -> Self {
        BatchKey::Markdown(k)
    }
}
impl From<TomlKey> for BatchKey {
    fn from(k: TomlKey) -> Self {
        BatchKey::Toml(k)
    }
}

/// Rust batches. Per-item for pub type declarations (struct/enum/trait/fn)
/// so the scheduler can individually rank e.g. `pub trait Log` above
/// `pub struct RecordBuilder` when the North Star does. File-scope batches
/// for crate-doc / mod-use / impl-method-groups; cross-file scope for the
/// macro surface (macros cluster in a single `src_dir` and benefit from
/// one name-list batch).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RustKey {
    /// `//!` module-doc lede — first paragraph only, entrypoints
    /// (`lib.rs`, `main.rs`). Priority 1.x.
    CrateDocLede { file: PathBuf },
    /// `//!` module-doc body — everything after the first paragraph.
    /// Predecessor: `CrateDocLede`. Priority 2.x–3.x.
    CrateDocBody { file: PathBuf },
    /// `use` + `mod` + `pub use` plumbing at the top of a file. Priority 2.x.
    ModUse { file: PathBuf },
    /// Surface listing of every top-level `pub` item name in a file. A
    /// catastrophic-omission hedge: when budget can't fit every individual
    /// item's body, this cheap listing still tells the agent that all the
    /// named items exist. Priority 1.x.
    PubItemNames { file: PathBuf },
    /// A single top-level `pub` item's declaration. For struct/enum/trait/
    /// type/const/static, the whole item (fields, variants, method sigs
    /// for traits). For fn/fn-sig, the signature with a `…` body marker.
    /// No rustdoc — that's `PubItemDoc` refinement. Keyed by the item's
    /// start line so each item has a distinct batch. Priority 1.x–4.x.
    PubItem { file: PathBuf, start_line: usize },
    /// Rustdoc (`///` / `/** */`) above a single `pub` item. Predecessor:
    /// the matching `PubItem` at the same `start_line`. Priority 3.x.
    PubItemDoc { file: PathBuf, start_line: usize },
    /// Impl-block headers + method signatures in a single file. Priority 2.x.
    MethodSigs { file: PathBuf },
    /// `#[macro_export] macro_rules!` names across `src_dir` (cross-file
    /// example). Priority 1.x.
    MacroNames { src_dir: PathBuf },
    /// Full `macro_rules!` bodies. Predecessor: `MacroNames`. Priority 2.x.
    MacroBodies { src_dir: PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownKey {
    /// Whole `SUMMARY.md` (mdBook ToC). Priority 1.x.
    SummaryWhole { file: PathBuf },
    /// README headline: first heading + first paragraph. Priority 1.x.
    ReadmeHeadline { file: PathBuf },
    /// One H2-level section of a markdown file, indexed by its 0-based
    /// position. For `README.md`, section 0 is the first section after
    /// the headline (predecessor: `ReadmeHeadline`). For other `.md`
    /// files (changelogs, docs pages), sections are all H2+ sections.
    /// Splitting lets large documents land piece-by-piece rather than
    /// all-or-nothing. Priority 2.x–5.x.
    Section { file: PathBuf, section_index: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TomlKey {
    /// `[package]` or `[workspace.package]` identity block. Priority 1.x.
    Identity { file: PathBuf },
    /// `[features]` table. Priority 1.x.
    Features { file: PathBuf },
    /// `[dependencies]` / `[dev-dependencies]` / `[build-dependencies]` /
    /// `[workspace.dependencies]`. Priority 2.x.
    Dependencies { file: PathBuf },
}

/// Stored batch after materialization. `key` is the semantic name; `content`
/// is the rendered content; `signals` are the value-model inputs.
#[derive(Debug, Clone)]
pub struct Batch {
    pub key: BatchKey,
    pub content: BatchContent,
    pub predecessor: Option<BatchKey>,
    pub signals: ValueSignals,
}

/// What a materializer returns. Scheduler stamps the `BatchId` and absorbs
/// it into the frontier; walkers never see ids.
#[derive(Debug, Clone)]
pub struct ResolvedBatch {
    pub content: BatchContent,
    pub signals: ValueSignals,
}

#[derive(Debug, Clone)]
pub enum BatchContent {
    /// Add named children under `parent` to the rendered tree.
    FileSystemEntries {
        parent: PathBuf,
        children: BTreeMap<OsString, EntryKind>,
    },
    /// Add line content to one or more files. Multi-file map for cross-file
    /// batches (e.g. `PubDecls` across several `.rs` files).
    Lines(BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
}

/// A rendered line. `Full` and `Truncated` carry text; `Ellipsis` is a
/// walker-emitted marker that renders as an indented `…` without a line
/// number prefix. Descendant batches can override any variant by writing to
/// the same (path, line) entry — typically `Ellipsis` → `Full` as refinement
/// fills in elided content.
#[derive(Debug, Clone)]
pub enum RenderedLine {
    Full(String),
    Truncated(String),
    /// Walker-emitted elision marker at a source line number. The number is
    /// not rendered; it exists so descendants can replace the ellipsis with
    /// real content at that exact line without a non-ancestor-overlap panic.
    Ellipsis,
}

impl RenderedLine {
    pub fn text(&self) -> &str {
        match self {
            Self::Full(t) | Self::Truncated(t) => t,
            Self::Ellipsis => "",
        }
    }

    pub fn is_truncated(&self) -> bool {
        matches!(self, Self::Truncated(_))
    }

    pub fn is_ellipsis(&self) -> bool {
        matches!(self, Self::Ellipsis)
    }
}

/// Multi-signal value inputs. The [`ValueModel`](crate::value::ValueModel)
/// composes these into an `f64` value; walkers don't see the weights.
///
/// Each signal is a 0..1 score:
/// - `catastrophic_omission`: harm if the agent never sees this content
///   (e.g. root listing, crate identity, macro-name list).
/// - `follow_up_minimization`: tool calls this saves vs. not including it
///   (e.g. public API decls, README).
/// - `zero_tool_call_understanding`: does seeing this complete a mental
///   model so the agent can reason without a follow-up at all (e.g. lede
///   paragraph, whole SUMMARY.md).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueSignals {
    pub catastrophic_omission: f64,
    pub follow_up_minimization: f64,
    pub zero_tool_call_understanding: f64,
    /// Relative-depth adjustment: multiplied into the final value. Values
    /// under 1.0 down-rank content that's deep in the tree or under
    /// `tests/`/`examples/`; values above 1.0 up-rank entrypoint files.
    pub depth_factor: f64,
}

impl Default for ValueSignals {
    fn default() -> Self {
        Self {
            catastrophic_omission: 0.0,
            follow_up_minimization: 0.0,
            zero_tool_call_understanding: 0.0,
            depth_factor: 1.0,
        }
    }
}
