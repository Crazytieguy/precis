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
//!
//! **Content model — spans, not materialized text.** A `BatchContent::Lines`
//! value carries `Vec<Span>` where each span declares *which* source lines
//! to include and *how* to render them (`Full`, `Truncated { pattern }`,
//! `Ellipsis`). Materialization happens at render/cost time by reading the
//! source file through the `RenderedTree`'s shared source cache. This keeps
//! the NS schema and the walker on the same expressive substrate: both
//! produce `Vec<Span>` values, both materialize identically.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

/// Opaque walker-key contract. The scheduler + renderer depend on this
/// trait instead of any concrete walker-specific enum, so a new walker
/// can be added without touching them. Concrete `Walker::Key` types
/// (typically an enum the walker defines) implement this; `BatchKey`
/// below is the default sum used by [`crate::walker::multi::MultiWalker`].
///
/// `Clone + Eq + Hash` support the scheduler's key-to-id map and dead
/// set; `Ord` gives deterministic tiebreaks; `Debug` is for panic
/// messages; `Send + Sync + 'static` keep the type usable across
/// threads / `Arc`s if downstream ever needs it.
pub trait WalkerKey:
    Clone
    + std::fmt::Debug
    + std::hash::Hash
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + 'static
{
    /// One-line human descriptor (e.g. `"crate-doc lede in src/lib.rs"`).
    /// Shown in schedule snapshots + divergence reports so diffs read
    /// as content-shape rather than `Rust(CrateDocLede(PathBuf(...)))`.
    fn describe(&self) -> String;
}

impl WalkerKey for BatchKey {
    fn describe(&self) -> String {
        match self {
            BatchKey::Fs(k) => k.describe(),
            BatchKey::Rust(k) => k.describe(),
            BatchKey::Markdown(k) => k.describe(),
            BatchKey::Toml(k) => k.describe(),
        }
    }
}

impl FsKey {
    pub fn describe(&self) -> String {
        match self {
            FsKey::DirListing { dir } => {
                let shown = display_path(dir);
                if shown.is_empty() {
                    "listing of '.'".to_string()
                } else {
                    format!("listing of '{shown}'")
                }
            }
        }
    }
}

impl RustKey {
    pub fn describe(&self) -> String {
        match self {
            RustKey::CrateDocLede { file } => format!("crate-doc lede in {}", display_path(file)),
            RustKey::CrateDocBody { file } => format!("crate-doc body in {}", display_path(file)),
            RustKey::ModUse { file } => format!("mod/use plumbing in {}", display_path(file)),
            RustKey::PubItemNames { file } => {
                format!("pub-item names surface in {}", display_path(file))
            }
            RustKey::PubItem { file, start_line } => {
                format!("pub item at {}:{}", display_path(file), start_line)
            }
            RustKey::PubItemDoc { file, start_line } => {
                format!("pub-item doc at {}:{}", display_path(file), start_line)
            }
            RustKey::MethodSigs { file } => format!("impl method sigs in {}", display_path(file)),
            RustKey::MacroNames { src_dir } => {
                format!("macro_export names across {}", display_path(src_dir))
            }
            RustKey::MacroBodies { src_dir } => {
                format!("macro_export bodies across {}", display_path(src_dir))
            }
        }
    }
}

impl MarkdownKey {
    pub fn describe(&self) -> String {
        match self {
            MarkdownKey::SummaryWhole { file } => {
                format!("mdBook SUMMARY at {}", display_path(file))
            }
            MarkdownKey::ReadmeHeadline { file } => {
                format!("README headline in {}", display_path(file))
            }
            MarkdownKey::Section {
                file,
                section_index,
            } => format!("{} section #{section_index}", display_path(file)),
        }
    }
}

impl TomlKey {
    pub fn describe(&self) -> String {
        match self {
            TomlKey::Identity { file } => format!("[package] in {}", display_path(file)),
            TomlKey::Features { file } => format!("[features] in {}", display_path(file)),
            TomlKey::Dependencies { file } => format!("[dependencies] in {}", display_path(file)),
        }
    }
}

/// Render a path as it should appear in a descriptor — lossy UTF-8 of the
/// path string. For absolute fixture paths we show only the last few
/// components to keep descriptors short. Kept local to batch.rs; the
/// schedule-snapshot serializer will typically post-process for its own
/// needs.
fn display_path(path: &std::path::Path) -> String {
    path.display().to_string()
}

/// A materialized batch: just the rendered content + the walker-supplied
/// value signals. The scheduler stores its own key/predecessor bookkeeping
/// alongside (see `scheduler::BatchEntry`); they're not fields of `Batch`
/// because the renderer/cost/divergence paths only ever read
/// `content`+`signals`, and the validator/divergence previously had to
/// forge fake keys to populate those fields. Dropping them eliminates
/// that kludge entirely.
#[derive(Debug, Clone)]
pub struct Batch {
    pub content: BatchContent,
    pub signals: ValueSignals,
}

/// What a materializer returns. Scheduler stamps the `BatchId` and absorbs
/// it into the frontier; walkers never see ids.
#[derive(Debug, Clone)]
pub struct ResolvedBatch {
    pub content: BatchContent,
    pub signals: ValueSignals,
}

/// Batch content. Either filesystem-level entries (one or more directory
/// listings grouped together) or a set of line-range spans with render
/// specs. Same type used by the walker (always fully-resolved) and the
/// NS schema (may carry [`FsEntries::All`] / `FsEntries::Names` sentinels
/// that get expanded to `Listed` at NS load time).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum BatchContent {
    /// One or more filesystem listings. Authored NS batches can bundle
    /// multiple parents together (e.g. `docs/ + config/ + config/themes/`);
    /// the walker's filesystem walker emits single-parent batches today.
    Fs { groups: Vec<FsGroup> },
    /// Source line ranges. Each span declares lines and a render spec;
    /// the render pipeline materializes by reading source at cost/apply/
    /// render time. Within one batch's spans, if two overlap on the same
    /// `(path, line)` the stronger render spec wins (Full > Truncated > Ellipsis).
    Lines { spans: Vec<Span> },
}

/// A single filesystem listing: one parent directory and its children.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsGroup {
    /// Parent directory path. Absolute for walker-emitted groups; NS
    /// groups are relativized in TOML and absolutized at load time.
    pub parent: PathBuf,
    /// Children under `parent`. See [`FsEntries`] for the three shapes
    /// this can take.
    pub entries: FsEntries,
}

/// Filesystem-listing contents. Two input shapes (from NS TOML) and one
/// post-resolution shape; walker always emits the post-resolution form.
///
/// - `All` — NS sentinel meaning "every child under `parent`". Expanded
///   to `Listed` at NS load time via `fs_util::list_dir`. Never emitted
///   by the walker; post-resolution invariant is no `All` variants.
/// - `Names` — NS input listing a subset of child names by string.
///   Each name must exist under `parent`; load-time resolution probes
///   and attaches the kind to produce `Listed`.
/// - `Listed` — concrete: child-name → kind. Walker-emitted and
///   post-resolution NS form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsEntries {
    All,
    Names(Vec<String>),
    Listed(BTreeMap<String, EntryKind>),
}

impl FsEntries {
    /// Convenience constructor for walker code that always produces a
    /// resolved listing.
    pub fn listed(children: BTreeMap<String, EntryKind>) -> Self {
        FsEntries::Listed(children)
    }

    /// Access the resolved child map. Returns `None` for unresolved
    /// (`All` / `Names`) variants — callers past NS load should never
    /// see these.
    pub fn as_listed(&self) -> Option<&BTreeMap<String, EntryKind>> {
        match self {
            FsEntries::Listed(m) => Some(m),
            _ => None,
        }
    }
}

impl Serialize for FsEntries {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            FsEntries::All => s.serialize_str("all"),
            FsEntries::Names(names) => names.serialize(s),
            FsEntries::Listed(map) => map.serialize(s),
        }
    }
}

impl<'de> Deserialize<'de> for FsEntries {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Str(String),
            Names(Vec<String>),
            Listed(BTreeMap<String, EntryKind>),
        }
        match Repr::deserialize(d)? {
            Repr::Str(s) if s == "all" => Ok(FsEntries::All),
            Repr::Str(s) => Err(serde::de::Error::custom(format!(
                "invalid entries sentinel {s:?}; expected \"all\" or a map/list"
            ))),
            Repr::Names(v) => Ok(FsEntries::Names(v)),
            Repr::Listed(m) => Ok(FsEntries::Listed(m)),
        }
    }
}

/// A contiguous range of source lines in one file, plus how to render them.
/// Both walker and NS schema produce these.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub path: PathBuf,
    /// Inclusive, 1-indexed.
    pub start: usize,
    /// Inclusive, 1-indexed. For a single line, `start == end`.
    pub end: usize,
    pub render: Render,
}

/// How to render a line. The render pipeline materializes by reading the
/// source file and applying the spec per line.
///
/// - `Full`: emit the source line verbatim with its line-number prefix.
/// - `Truncated { pattern }`: emit only the regex match of `pattern` against
///   the source line, followed by a trailing `…`. The pattern must match
///   at least one character on every line the span covers (validated at
///   schema load for NS spans; walker spans don't use `Truncated` today).
/// - `Ellipsis`: emit a `…` marker at that source-line position with no
///   line-number prefix. A descendant batch can later replace this line
///   with a `Full` or `Truncated` span at the same `(path, line)`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Render {
    Full,
    Truncated { pattern: String },
    Ellipsis,
}

impl Render {
    /// Priority when two spans overlap on the same (path, line):
    /// Full > Truncated > Ellipsis.
    pub(crate) fn priority(&self) -> u8 {
        match self {
            Render::Full => 3,
            Render::Truncated { .. } => 2,
            Render::Ellipsis => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    File,
    #[serde(rename = "dir")]
    Directory,
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
