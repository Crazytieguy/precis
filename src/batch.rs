//! Walker/scheduler internals. A **batch** is the atomic scheduling
//! unit: a named subset of source content the walker proposes to render.
//! Two layers of identity:
//!
//! - [`BatchKey`] is a walker-defined semantic name. Walkers declare
//!   predecessor edges by naming keys, so edges can be set up before
//!   any file is parsed. Stable through materialization.
//! - [`BatchId`] is the scheduler's internal index assigned at
//!   materialization time. Walkers never see ids.
//!
//! Content vocabulary shared with the NS schema lives in
//! [`crate::content`]; this file is only walker/scheduler-internal.

use std::path::PathBuf;

use crate::content::BatchContent;

/// Scheduler-internal batch index. Assigned at materialization time;
/// walkers never see these.
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

/// Default walker-key sum used by [`crate::walker::multi::MultiWalker`].
/// Scheduler/render code depends on the [`WalkerKey`] trait, not on
/// this enum — a new walker implementing [`WalkerKey`] can be added
/// without touching either.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BatchKey {
    Fs(FsKey),
    Rust(RustKey),
    Markdown(MarkdownKey),
    Toml(TomlKey),
    Typescript(TsKey),
    Json(JsonKey),
    Plaintext(PlaintextKey),
    C(CKey),
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
impl From<TsKey> for BatchKey {
    fn from(k: TsKey) -> Self {
        BatchKey::Typescript(k)
    }
}
impl From<JsonKey> for BatchKey {
    fn from(k: JsonKey) -> Self {
        BatchKey::Json(k)
    }
}
impl From<PlaintextKey> for BatchKey {
    fn from(k: PlaintextKey) -> Self {
        BatchKey::Plaintext(k)
    }
}
impl From<CKey> for BatchKey {
    fn from(k: CKey) -> Self {
        BatchKey::C(k)
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
    /// No rustdoc — that's the `PubItemDocLede` / `PubItemDocBody`
    /// refinement. Keyed by the item's start line so each item has a
    /// distinct batch. Priority 1.x–4.x.
    PubItem { file: PathBuf, start_line: usize },
    /// First paragraph of the rustdoc (`///` / `/** */`) above a single
    /// `pub` item — everything up to the first `# Heading` line, or
    /// the whole doc when no heading is present. Predecessor: the
    /// matching `PubItem` at the same `start_line`. Priority 3.x.
    PubItemDocLede { file: PathBuf, start_line: usize },
    /// Body of the rustdoc above a single `pub` item — from the first
    /// `# Heading` onward. Predecessor: the matching `PubItemDocLede`
    /// when one exists, otherwise the `PubItem` (for docs whose first
    /// non-doctest-hidden line is already a heading — empty Lede would
    /// otherwise dead-key the body). Priority 3.x–4.x.
    PubItemDocBody { file: PathBuf, start_line: usize },
    /// Impl-block headers + method signatures in a single file. Priority 2.x.
    MethodSigs { file: PathBuf },
    /// `#[macro_export] macro_rules!` names across `src_dir` (cross-file
    /// example). Priority 1.x.
    MacroNames { src_dir: PathBuf },
    /// Full body of one `#[macro_export] macro_rules!` definition.
    /// Per-macro splitting (vs. the previous cross-file `MacroBodies`
    /// aggregate) lets the scheduler rank user-facing macros above
    /// dispatch helpers and keeps a single oversized body from blocking
    /// the prefix-monotone schedule. Predecessor: `MacroNames` for the
    /// enclosing `src_dir`. Priority 2.x.
    MacroBody { file: PathBuf, start_line: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownKey {
    /// Whole `SUMMARY.md` (mdBook ToC). Priority 1.x.
    SummaryWhole { file: PathBuf },
    /// README headline: first heading + first paragraph. Priority 1.x.
    ReadmeHeadline { file: PathBuf },
    /// Cheap navigation hedge: every H1/H2/H3 heading line in the file,
    /// nothing else. Analog of [`RustKey::PubItemNames`]. For READMEs,
    /// the H1 line stays under [`MarkdownKey::ReadmeHeadline`] so the
    /// headline's truncation render isn't overridden; outline collects
    /// H2+H3 only. Predecessor of every [`MarkdownKey::Section`] in the
    /// file when emitted (so the section's heading-row overlap is
    /// permitted as ancestor overlap). Priority 1.x.
    HeadingsOutline { file: PathBuf },
    /// One scheduling unit of a markdown file's body, indexed by its
    /// 0-based position in the walker's logical-section list. The
    /// granularity is variable: most H2s stay as a single `Whole`
    /// range; content-heavy H2s (and the file's outline emitted) are
    /// subdivided either as a *bullet split* (an H2 whose
    /// non-decorative content is a single bullet list, one batch per
    /// top-level item — anyhow `## Details` shape) or as an *H3
    /// split* (an H2 with ≥2 H3 children, one `Intro` plus one batch
    /// per H3 child). Either rule lets the scheduler pick relevant
    /// sub-sections instead of all-or-nothing committing to the
    /// whole H2. The split classification lives in the walker (not
    /// on this key) — `section_index` is the post-split logical
    /// index, so enabling or changing a split rule shifts the
    /// numbering. For `README.md`, section 0 is the first section
    /// after the headline (predecessor: `ReadmeHeadline`). When
    /// [`MarkdownKey::HeadingsOutline`] is emitted for the same file
    /// the outline becomes Section's predecessor. Priority 2.x–5.x.
    Section { file: PathBuf, section_index: usize },
}

/// TypeScript / TSX batches. Mirrors the Rust walker shape: per-file
/// orientation batches (module-doc lede, imports, top-level export-name
/// surface) plus per-export item batches with optional JSDoc refinement.
///
/// "Public" in TS = a top-level declaration with the `export` keyword (or
/// a `default` export). Re-exports without a body (`export { foo } from '…'`)
/// are folded into the `Imports` batch since they're plumbing, not items.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TsKey {
    /// Top-of-file `/** … */` block — module-level JSDoc lede. Priority 1.x.
    /// Only emitted for entrypoint files (`index.ts`, `main.ts`, `mod.ts`).
    ModuleDocLede { file: PathBuf },
    /// `import` + side-effect imports + bare `export … from` re-exports at
    /// the top of the file. Plumbing batch. Priority 2.x.
    Imports { file: PathBuf },
    /// Surface listing of every top-level export's first line — a
    /// catastrophic-omission hedge when individual decls don't all fit.
    /// Priority 1.x.
    ExportNames { file: PathBuf },
    /// One top-level export's declaration. For interface/type/class/enum,
    /// the whole item. For function, signature with body marker. For
    /// const/let, the assignment line. Keyed by start line so each
    /// export has a distinct batch. Priority 1.x–4.x.
    Export { file: PathBuf, start_line: usize },
    /// JSDoc (`/** … */`) above a single export. Predecessor: the matching
    /// `Export` at the same `start_line`. Priority 3.x.
    ExportDoc { file: PathBuf, start_line: usize },
    /// Body interior of an export with a `statement_block` body — function,
    /// generator, class methods, or `export default <fn|class>`. Brace-strip
    /// rule: outer `{` and `}` rows omitted, interior rows emitted. For
    /// classes, body interiors of every member with a `statement_block`
    /// body are merged into one batch. Predecessor: the matching `Export`
    /// at the same `start_line`. Sibling of `ExportDoc` under `Export`;
    /// the two cover disjoint lines. Not emitted for interface / type-alias
    /// / enum / re-export / lexical-with-fn-init (deferred — see the v4
    /// plan in `ignore/plan-ts-export-body-v2.md`). Priority 2.x–3.x.
    ExportBody { file: PathBuf, start_line: usize },
}

/// JSON batches. `package.json` is split along the same ontology as
/// `Cargo.toml` (identity / scripts ≈ features / dependencies) plus a
/// JS-specific entrypoint-pointer batch (`main`/`module`/`exports`/etc.).
/// Other small JSON configs (`tsconfig.json`, `.eslintrc.json`,
/// `jsr.json`, …) get a single `Whole` batch when they're small enough
/// to pay for outright.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JsonKey {
    /// `package.json` identity scalars: `name`, `version`, `description`,
    /// `license`, `author`/`authors`, `repository`, `homepage`, `keywords`,
    /// `type`. Priority 1.x.
    Identity { file: PathBuf },
    /// `package.json` entrypoint pointers: `main`, `module`, `browser`,
    /// `exports`, `types`/`typings`, `source`, `bin`, `unpkg`, `umd:main`,
    /// `jsnext:main`, `react-native`, `files`. Priority 1.x–2.x.
    Entry { file: PathBuf },
    /// `package.json` `scripts` block. Priority 2.x.
    Scripts { file: PathBuf },
    /// `package.json` dependency blocks (`dependencies`,
    /// `devDependencies`, `peerDependencies`, `optionalDependencies`,
    /// `engines`, `packageManager`). Priority 2.x–4.x.
    Dependencies { file: PathBuf },
    /// Whole-file render of a small JSON config (`tsconfig.json`,
    /// `.eslintrc.json`, `jsr.json`, etc.). Skipped for `package.json`
    /// (use the split batches instead) and for large/generated files.
    Whole { file: PathBuf },
}

/// Plaintext config / license file batches. One whole-file `Whole`
/// variant per supported filename (see [`crate::walker::plaintext`] for
/// the whitelist). These files would otherwise only appear in dir
/// listings — the plaintext walker emits a content batch capped at a
/// small line + token budget so render-time displacement of richer
/// walker batches stays bounded.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlaintextKey {
    /// Whole-file render of a small, known-plaintext config or license
    /// file. Skipped when the file's line count or rendered token cost
    /// exceeds the walker's caps.
    Whole { file: PathBuf },
}

/// C / C-header batches. Mirrors the Rust walker shape: per-file
/// orientation batches (top-of-file banner, includes, decl-name surface)
/// plus per-decl item batches with optional doc / body refinement.
///
/// "Public" rule: top-level `function_definition` / `declaration` /
/// `type_definition` / `preproc_def` / `preproc_function_def` whose
/// declarator is not `static` (in `.c` files), plus `static inline`
/// function definitions in `.h` files (header-only inline accessors are
/// part of the header's public API expansion). The single wrapping
/// header-guard `#ifndef X` / `#define X` / `#endif` is descended into
/// transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CKey {
    /// Top-of-file `/* */` banner comment (license / brief). Priority
    /// 1.x for headers, 4.x for `.c` files.
    HeaderBanner { file: PathBuf },
    /// `#include` directives — the file's structural dependencies.
    /// Priority 2.x.
    Includes { file: PathBuf },
    /// Surface listing of every top-level public declaration's first
    /// line — typedefs, function prototypes, struct/enum names, public
    /// `#define`s, function definitions. Catastrophic-omission hedge.
    /// Priority 1.x.
    DeclNames { file: PathBuf },
    /// One top-level public declaration. For typedefs / function
    /// prototypes / `extern` decls / `#define`s, the whole statement.
    /// For struct / enum / union, the whole specifier. For function
    /// definitions, the signature with a body marker. Keyed by start
    /// line. Priority 1.x–4.x.
    Decl { file: PathBuf, start_line: usize },
    /// Body interior of a function definition. Predecessor: matching
    /// [`CKey::Decl`] at the same `start_line`. Priority 2.x–3.x.
    DeclBody { file: PathBuf, start_line: usize },
    /// Doc comment(s) immediately above a declaration. Predecessor:
    /// matching [`CKey::Decl`]. Priority 3.x.
    DeclDoc { file: PathBuf, start_line: usize },
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

/// Opaque walker-key contract. Scheduler + renderer depend on this trait
/// instead of any concrete walker-specific enum, so a new walker can be
/// added without touching them.
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

    /// Per-key cost concavity exponent for the scheduling ratio
    /// (`value / cost^exponent`). Defaults to
    /// [`crate::value::DEFAULT_CONCAVITY_EXPONENT`]; raise on prose-shaped
    /// batches (doc bodies, README sections) whose token count grows
    /// without proportional structural value.
    fn concavity_exponent(&self) -> f64 {
        crate::value::DEFAULT_CONCAVITY_EXPONENT
    }
}

impl WalkerKey for BatchKey {
    fn describe(&self) -> String {
        match self {
            BatchKey::Fs(k) => k.describe(),
            BatchKey::Rust(k) => k.describe(),
            BatchKey::Markdown(k) => k.describe(),
            BatchKey::Toml(k) => k.describe(),
            BatchKey::Typescript(k) => k.describe(),
            BatchKey::Json(k) => k.describe(),
            BatchKey::Plaintext(k) => k.describe(),
            BatchKey::C(k) => k.describe(),
        }
    }

    fn concavity_exponent(&self) -> f64 {
        match self {
            BatchKey::Markdown(k) => k.concavity_exponent(),
            BatchKey::C(k) => k.concavity_exponent(),
            BatchKey::Fs(_)
            | BatchKey::Rust(_)
            | BatchKey::Toml(_)
            | BatchKey::Typescript(_)
            | BatchKey::Json(_)
            | BatchKey::Plaintext(_) => crate::value::DEFAULT_CONCAVITY_EXPONENT,
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
            RustKey::PubItemDocLede { file, start_line } => {
                format!("pub-item doc lede at {}:{}", display_path(file), start_line)
            }
            RustKey::PubItemDocBody { file, start_line } => {
                format!("pub-item doc body at {}:{}", display_path(file), start_line)
            }
            RustKey::MethodSigs { file } => format!("impl method sigs in {}", display_path(file)),
            RustKey::MacroNames { src_dir } => {
                format!("macro_export names across {}", display_path(src_dir))
            }
            RustKey::MacroBody { file, start_line } => {
                format!("macro_export body at {}:{}", display_path(file), start_line)
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
            MarkdownKey::HeadingsOutline { file } => {
                format!("headings outline in {}", display_path(file))
            }
            MarkdownKey::Section {
                file,
                section_index,
            } => format!("{} section #{section_index}", display_path(file)),
        }
    }

    /// Sections at index ≥1 get a steeper `0.45` so prose body grows more
    /// expensive than structural anchors of the same value. Index 0 keeps
    /// the default — many READMEs lead with their canonical claim there.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            MarkdownKey::Section {
                section_index: 0, ..
            } => crate::value::DEFAULT_CONCAVITY_EXPONENT,
            MarkdownKey::Section { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }
}

impl TsKey {
    pub fn describe(&self) -> String {
        match self {
            TsKey::ModuleDocLede { file } => format!("module-doc lede in {}", display_path(file)),
            TsKey::Imports { file } => format!("imports in {}", display_path(file)),
            TsKey::ExportNames { file } => {
                format!("export names surface in {}", display_path(file))
            }
            TsKey::Export { file, start_line } => {
                format!("export at {}:{}", display_path(file), start_line)
            }
            TsKey::ExportDoc { file, start_line } => {
                format!("export doc at {}:{}", display_path(file), start_line)
            }
            TsKey::ExportBody { file, start_line } => {
                format!("export body at {}:{}", display_path(file), start_line)
            }
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

impl JsonKey {
    pub fn describe(&self) -> String {
        match self {
            JsonKey::Identity { file } => format!("package identity in {}", display_path(file)),
            JsonKey::Entry { file } => format!("package entrypoints in {}", display_path(file)),
            JsonKey::Scripts { file } => format!("package scripts in {}", display_path(file)),
            JsonKey::Dependencies { file } => {
                format!("package dependencies in {}", display_path(file))
            }
            JsonKey::Whole { file } => format!("json config {}", display_path(file)),
        }
    }
}

impl PlaintextKey {
    pub fn describe(&self) -> String {
        match self {
            PlaintextKey::Whole { file } => format!("plaintext config {}", display_path(file)),
        }
    }
}

impl CKey {
    /// `Decl` and `DeclBody` carry a steeper concavity than the default
    /// because C decls are typically very short (a single typedef /
    /// prototype line) and a header file emits dozens of them. Under
    /// the default 0.35 exponent each tiny batch has a runaway
    /// value/cost^0.35 ratio and the scheduler picks the whole stack
    /// of them before any larger anchor batch (README section, RustKey
    /// PubItem). 0.45 (matching `MarkdownKey::Section` for non-zero
    /// indices) tames that without dropping headers out of the schedule
    /// — calibrated against the divergence reports for sds, bareiron,
    /// and krep.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            CKey::Decl { .. } | CKey::DeclBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self) -> String {
        match self {
            CKey::HeaderBanner { file } => format!("c header banner in {}", display_path(file)),
            CKey::Includes { file } => format!("c includes in {}", display_path(file)),
            CKey::DeclNames { file } => format!("c decl names surface in {}", display_path(file)),
            CKey::Decl { file, start_line } => {
                format!("c decl at {}:{}", display_path(file), start_line)
            }
            CKey::DeclBody { file, start_line } => {
                format!("c decl body at {}:{}", display_path(file), start_line)
            }
            CKey::DeclDoc { file, start_line } => {
                format!("c decl doc at {}:{}", display_path(file), start_line)
            }
        }
    }
}

fn display_path(path: &std::path::Path) -> String {
    path.display().to_string()
}

/// A walker-emitted scheduling unit. Carries the walker's key (so other
/// candidates can name it as predecessor), the rendered content, and the
/// scalar value used for ranking. The scheduler stamps a [`BatchId`] when
/// the batch enters the pool but the walker never sees it.
///
/// Walkers compute `value` directly: it's the scalar input to
/// [`crate::value::ratio`], on a shared cross-walker scale (calibration
/// across walkers is a divergence-reports problem, not a code-level
/// invariant — see `docs/design-notes.md`).
#[derive(Debug, Clone)]
pub struct Batch<K: WalkerKey> {
    pub key: K,
    /// Optional predecessor edge. The batch stays pending until its
    /// predecessor is scheduled; line overlap with earlier batches is only
    /// permitted along this chain (see [`crate::content`] / `render`).
    pub predecessor: Option<K>,
    pub content: BatchContent,
    pub value: f64,
}
