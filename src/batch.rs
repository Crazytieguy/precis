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

use std::path::{Path, PathBuf};

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
    Go(GoKey),
    Python(PythonKey),
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
impl From<GoKey> for BatchKey {
    fn from(k: GoKey) -> Self {
        BatchKey::Go(k)
    }
}
impl From<PythonKey> for BatchKey {
    fn from(k: PythonKey) -> Self {
        BatchKey::Python(k)
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
    /// Body slice of a public function item, split by top-level statement.
    /// Predecessor: the matching `PubItem`.
    PubItemBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
    /// A private top-level item in a Rust entrypoint file. Functions render
    /// as signatures with body ellipses; non-functions render whole so example
    /// `main.rs` usage flows can still surface without `pub` items.
    EntryItem { file: PathBuf, start_line: usize },
    /// Body slice of a private entrypoint function, split by top-level
    /// statement. Predecessor: the matching `EntryItem`.
    EntryItemBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
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
    /// Chunked `Imports` for entrypoint files that are mostly re-export walls.
    /// Each chunk groups consecutive imports/re-exports from the same source.
    ImportChunk { file: PathBuf, chunk_index: usize },
    /// Surface listing of every top-level export's first line — a
    /// catastrophic-omission hedge when individual decls don't all fit.
    /// Priority 1.x.
    ExportNames {
        file: PathBuf,
        chunk_index: usize,
        export_count: usize,
        type_only_export_count: usize,
    },
    /// One top-level export's declaration. For interface/type/class/enum,
    /// the whole item. For function, signature with body marker. For
    /// const/let, the assignment line. Keyed by start line so each
    /// export has a distinct batch. Priority 1.x–4.x.
    Export { file: PathBuf, start_line: usize },
    /// JSDoc (`/** … */`) above a single export. Predecessor: the matching
    /// `Export` at the same `start_line`. Priority 3.x.
    ExportDoc { file: PathBuf, start_line: usize },
    /// Surface for one member of an exported JavaScript class. Predecessor:
    /// the matching class `Export` at the same `start_line`. Priority 2.x–3.x.
    ExportMember {
        file: PathBuf,
        /// Parent export line.
        start_line: usize,
        /// First line of the class member surface.
        member_start_line: usize,
    },
    /// Body slice of an export with a `statement_block` body — function,
    /// generator, class methods, or `export default <fn|class>`. Brace-strip
    /// rule: outer `{` and `}` rows omitted, interior rows emitted.
    /// Predecessor: the matching `Export` at the same `start_line`. Sibling
    /// of `ExportDoc` under `Export`; the two cover disjoint lines. Priority
    /// 2.x–3.x.
    ExportBody {
        file: PathBuf,
        /// Parent export line. Kept in the key so body slices remain tied to
        /// their predecessor even when two bodies start on the same line in
        /// different declarations.
        start_line: usize,
        /// First emitted line of this body slice; disambiguates siblings
        /// within the parent export.
        body_start_line: usize,
    },
    /// Top-level non-exported TypeScript declaration surface. This catches
    /// module-private classes, helper functions, type aliases, and constants
    /// that exported APIs depend on but do not export directly.
    ModuleItem { file: PathBuf, start_line: usize },
    /// Body slice of a top-level non-exported declaration. Large bodies split
    /// by top-level statement so methods/regions can schedule independently.
    /// Predecessor: the matching `ModuleItem`.
    ModuleItemBody {
        file: PathBuf,
        /// Parent module item line. Kept in the key so the predecessor edge
        /// and sibling body slices share the same declaration identity.
        start_line: usize,
        /// First emitted line of this body slice; disambiguates siblings
        /// within the parent item.
        body_start_line: usize,
    },
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
    /// `type`, `private`, `license`/`licenses`. Priority 1.x.
    Identity { file: PathBuf },
    /// Auxiliary `package.json` metadata: authorship, repository/homepage,
    /// bugs, keywords, publish config, funding. Priority 1.x-2.x.
    IdentityMeta { file: PathBuf },
    /// `package.json` entrypoint pointers: `main`, `module`, `browser`,
    /// `exports`, `types`/`typings`, `source`, `bin`, `unpkg`, `umd:main`,
    /// `jsnext:main`, `react-native`, `files`. Priority 1.x–2.x.
    Entry { file: PathBuf },
    /// `package.json` runtime/toolchain constraints: `engines`,
    /// `engineStrict`, `packageManager`. Priority 1.x-2.x.
    Runtime { file: PathBuf },
    /// `package.json` `scripts` block. Priority 2.x.
    Scripts { file: PathBuf },
    /// `package.json` dependency blocks (`dependencies`,
    /// `devDependencies`, `peerDependencies`, `optionalDependencies`,
    /// `overrides`, `resolutions`). Priority 2.x–4.x.
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
    /// Large headers/source files (`krep.h` exposes ~80 decls) chunk
    /// the surface in `NAMES_SURFACE_CHUNK_SIZE`-sized groups so a
    /// 1000-token monolith doesn't lose the value/cost race against
    /// per-decl batches. Priority 1.x.
    DeclNames { file: PathBuf, chunk_index: usize },
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

/// Go batches. Mirrors the C walker shape — per-file orientation
/// batches (package + imports, decl-name surface) plus per-decl item
/// batches with optional doc / body refinement.
///
/// Visibility: emits **all** top-level declarations, exported and
/// unexported. NS authors regularly anchor on intentionally-unexported
/// types (`go-multierror`'s `chain`, `tock`'s `repository` /
/// `twInterval`). A `visibility_factor` discount ranks exported names
/// above unexported ones rather than hard-filtering.
///
/// Grouped declarations (`type ( … )`, `var ( … )`, `const ( … )`)
/// are emitted as **one** batch covering the whole block — splitting
/// per-spec would lose iota / inherited-type / shared-comment
/// semantics, which is critical for Go's enum-via-iota idiom.
///
/// `*_test.go` files surface a separate [`GoKey::TestNames`] batch
/// listing `Test*` / `Benchmark*` / `Example*` first lines only —
/// `go test`'s lookup contract — while skipping per-decl bodies. The
/// 0.2 multiplier from [`crate::value::non_essential_factor`] keeps the
/// test surface deprioritized vs. ordinary source.
///
/// `go.mod` (and `go.work`) get a single whole-file batch
/// ([`GoKey::GoMod`]); NS authors split the file into logical sections
/// by line range, but the walker needs only to make the file
/// reachable in one schedule slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GoKey {
    /// Package clause + import block at the top of a `.go` file.
    /// Plumbing batch. Priority 2.x.
    PackageImports { file: PathBuf },
    /// Surface listing of every top-level declaration's first line —
    /// funcs, methods, types, vars, consts. Catastrophic-omission
    /// hedge. Visibility-blind — lists everything in the file
    /// regardless of export status. Priority 1.x.
    DeclNames { file: PathBuf },
    /// One top-level declaration. For function / method definitions,
    /// the signature with a body marker. For type / var / const, the
    /// whole declaration including grouped specs. Keyed by start line.
    /// Priority 1.x–4.x.
    Decl { file: PathBuf, start_line: usize },
    /// Body interior of a function or method definition. Predecessor:
    /// matching [`GoKey::Decl`] at the same `start_line`. Priority 2.x–3.x.
    DeclBody { file: PathBuf, start_line: usize },
    /// Run of `//` (or `/* */`) comments immediately above a decl, with
    /// no blank-line gap. Predecessor: matching [`GoKey::Decl`].
    /// Priority 3.x.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Surface listing of every `Test*` / `Benchmark*` / `Example*`
    /// function's first line in a `_test.go` file. Skipped for
    /// non-test files. Priority 3.x–5.x.
    TestNames { file: PathBuf },
    /// Whole-file render of a `go.mod` (or `go.work`) file. Capped
    /// at a small line count; larger module files are skipped. Priority 1.x.
    GoMod { file: PathBuf },
}

/// Python batches. Mirrors the Go / C walkers' shape — per-file
/// orientation (imports + `__all__` + module docstring + module-level
/// dunder assignments fold into [`PythonKey::Imports`]) plus per-decl
/// item batches (top-level def / class / non-dunder constant), and
/// per-method batches inside top-level classes (Rust precedent —
/// methods are first-class scheduling units).
///
/// **Decorator handling**: tree-sitter Python wraps a decorated def /
/// class in `decorated_definition`. The walker treats that wrapper as
/// the unit, so a `Decl`'s `start_line` is the `@decorator` row and
/// the span includes the decorator lines.
///
/// **Visibility**: emits all top-level + class-body items. A
/// `visibility_factor` discount (1.0 unprefixed, 0.6 leading-`_`,
/// 1.0 dunder) ranks public-by-PEP-8 names above leading-`_`-prefixed
/// "internal" ones rather than hard-filtering. NSes anchor on
/// intentionally-private names (`pluggy._callers._multicall`,
/// `pluggy._hooks.HookCaller._add_hookimpl`).
///
/// `test_*.py` / `*_test.py` files surface a separate
/// [`PythonKey::TestNames`] batch listing every `def test_*` first
/// line (decorator-aware) and skip per-decl bodies. The 0.2
/// multiplier from [`crate::value::non_essential_factor`] keeps the
/// test surface deprioritized vs. ordinary source.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PythonKey {
    /// Top-of-file `import` / `from … import …` statements, optional
    /// module docstring, `__all__`, and module-level dunder
    /// assignments (`__version__`, `__author__`). Plumbing batch.
    /// Priority 2.x.
    Imports { file: PathBuf },
    /// Chunked `Imports` for large `__init__.py` re-export walls. Each chunk
    /// groups consecutive imports/re-exports from the same source.
    ImportChunk { file: PathBuf, chunk_index: usize },
    /// Surface listing of every top-level class, def (sync or async),
    /// and non-dunder simple-assignment first line.
    /// Catastrophic-omission hedge. Priority 1.x.
    DeclNames { file: PathBuf, chunk_index: usize },
    /// One top-level item. For class, the `class Foo(Base):` header
    /// (decorator lines included if decorated). For def, signature
    /// with body marker. For constant, the assignment line(s). Keyed
    /// by start line (the decorator row when decorated). Priority
    /// 1.x–4.x.
    Decl { file: PathBuf, start_line: usize },
    /// Docstring of a top-level def or class — the
    /// `expression_statement(string)` at the start of its body, after
    /// any leading comments. Predecessor: matching [`PythonKey::Decl`].
    /// Priority 3.x.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Body slice of a top-level def, split by top-level statement.
    /// Skips the leading docstring (covered by [`PythonKey::DeclDoc`]).
    /// Predecessor: matching [`PythonKey::Decl`]. Priority 2.x–3.x.
    DeclBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
    /// Class body excluding method def signatures and the leading
    /// docstring — covers TypedDict / dataclass / Protocol / Pydantic
    /// fields, `__slots__`, class-level constants. Predecessor:
    /// matching class [`PythonKey::Decl`]. Priority 3.x–4.x.
    ClassBody { file: PathBuf, start_line: usize },
    /// Surface listing of every method def first line across every
    /// top-level class in this file — Rust [`RustKey::MethodSigs`]
    /// analog. Decorator-aware. Catastrophic-omission hedge for class
    /// APIs. Priority 2.x.
    MethodSigs { file: PathBuf, chunk_index: usize },
    /// Per-method version of [`PythonKey::Decl`] for a method inside
    /// a top-level class. Predecessor: enclosing class's
    /// [`PythonKey::Decl`]. Priority 2.x–4.x.
    Method { file: PathBuf, start_line: usize },
    /// Method's docstring. Predecessor: matching
    /// [`PythonKey::Method`]. Priority 3.x.
    MethodDoc { file: PathBuf, start_line: usize },
    /// Method body slice, split by top-level statement, sans leading
    /// docstring. Predecessor: matching [`PythonKey::Method`]. Priority 3.x–4.x.
    MethodBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
    /// Surface listing of every `def test_*` first line in a `test_*.py`
    /// / `*_test.py` file (top-level + class-body, decorator-aware).
    /// Skipped for non-test files. Priority 3.x–5.x.
    TestNames { file: PathBuf },
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
    /// `fixture_root` is stripped from any embedded paths so descriptors
    /// stay relative and diff-stable across checkouts.
    fn describe(&self, fixture_root: &Path) -> String;

    /// Per-key cost concavity exponent for the scheduling ratio
    /// (`value / cost^exponent`). Defaults to
    /// [`crate::value::DEFAULT_CONCAVITY_EXPONENT`]; raise on prose-shaped
    /// batches (doc bodies, README sections) whose token count grows
    /// without proportional structural value.
    fn concavity_exponent(&self) -> f64 {
        crate::value::DEFAULT_CONCAVITY_EXPONENT
    }

    /// Weight for routing discovered descendant value back into this
    /// key's scheduling score. Defaults to off: most predecessor edges
    /// are ordinary refinements, not broad gates whose children should
    /// affect the parent's rank.
    fn gated_descendant_value_weight(&self) -> f64 {
        0.0
    }
}

impl WalkerKey for BatchKey {
    fn describe(&self, fixture_root: &Path) -> String {
        match self {
            BatchKey::Fs(k) => k.describe(fixture_root),
            BatchKey::Rust(k) => k.describe(fixture_root),
            BatchKey::Markdown(k) => k.describe(fixture_root),
            BatchKey::Toml(k) => k.describe(fixture_root),
            BatchKey::Typescript(k) => k.describe(fixture_root),
            BatchKey::Json(k) => k.describe(fixture_root),
            BatchKey::Plaintext(k) => k.describe(fixture_root),
            BatchKey::C(k) => k.describe(fixture_root),
            BatchKey::Go(k) => k.describe(fixture_root),
            BatchKey::Python(k) => k.describe(fixture_root),
        }
    }

    fn concavity_exponent(&self) -> f64 {
        match self {
            BatchKey::Markdown(k) => k.concavity_exponent(),
            BatchKey::C(k) => k.concavity_exponent(),
            BatchKey::Go(k) => k.concavity_exponent(),
            BatchKey::Json(k) => k.concavity_exponent(),
            BatchKey::Python(k) => k.concavity_exponent(),
            BatchKey::Rust(k) => k.concavity_exponent(),
            BatchKey::Typescript(k) => k.concavity_exponent(),
            BatchKey::Fs(_) | BatchKey::Toml(_) | BatchKey::Plaintext(_) => {
                crate::value::DEFAULT_CONCAVITY_EXPONENT
            }
        }
    }

    fn gated_descendant_value_weight(&self) -> f64 {
        match self {
            BatchKey::Typescript(k) => k.gated_descendant_value_weight(),
            _ => 0.0,
        }
    }
}

impl FsKey {
    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            FsKey::DirListing { dir } => {
                let shown = display_path(dir, fixture_root);
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
    /// `PubItemDocBody` carries a steeper `0.45`: rustdoc prose after
    /// the first `# Heading` grows token cost without proportional
    /// structural value, so at the default it out-ranks cheaper
    /// anchors (`PubItemNames`, `ModUse`, sibling `PubItem`s).
    /// `CrateDocBody` stays at the default — its bullets are where
    /// fixture NSes credit the crate-orientation prose. Matches the
    /// `MarkdownKey::Section` precedent of demoting prose bodies only.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            RustKey::PubItemDocBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            RustKey::CrateDocLede { file } => {
                format!("crate-doc lede in {}", display_path(file, fixture_root))
            }
            RustKey::CrateDocBody { file } => {
                format!("crate-doc body in {}", display_path(file, fixture_root))
            }
            RustKey::ModUse { file } => {
                format!("mod/use plumbing in {}", display_path(file, fixture_root))
            }
            RustKey::PubItemNames { file } => {
                format!(
                    "pub-item names surface in {}",
                    display_path(file, fixture_root)
                )
            }
            RustKey::PubItem { file, start_line } => {
                format!(
                    "pub item at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            RustKey::PubItemBody {
                file,
                start_line,
                body_start_line,
            } => {
                format!(
                    "pub item body at {}:{} body {}",
                    display_path(file, fixture_root),
                    start_line,
                    body_start_line
                )
            }
            RustKey::EntryItem { file, start_line } => {
                format!(
                    "entry item at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            RustKey::EntryItemBody {
                file,
                start_line,
                body_start_line,
            } => {
                format!(
                    "entry item body at {}:{} body {}",
                    display_path(file, fixture_root),
                    start_line,
                    body_start_line
                )
            }
            RustKey::PubItemDocLede { file, start_line } => {
                format!(
                    "pub-item doc lede at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            RustKey::PubItemDocBody { file, start_line } => {
                format!(
                    "pub-item doc body at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            RustKey::MethodSigs { file } => {
                format!("impl method sigs in {}", display_path(file, fixture_root))
            }
            RustKey::MacroNames { src_dir } => {
                format!(
                    "macro_export names across {}",
                    display_path(src_dir, fixture_root)
                )
            }
            RustKey::MacroBody { file, start_line } => {
                format!(
                    "macro_export body at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
        }
    }
}

impl MarkdownKey {
    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            MarkdownKey::SummaryWhole { file } => {
                format!("mdBook SUMMARY at {}", display_path(file, fixture_root))
            }
            MarkdownKey::ReadmeHeadline { file } => {
                format!("README headline in {}", display_path(file, fixture_root))
            }
            MarkdownKey::HeadingsOutline { file } => {
                format!("headings outline in {}", display_path(file, fixture_root))
            }
            MarkdownKey::Section {
                file,
                section_index,
            } => format!(
                "{} section #{section_index}",
                display_path(file, fixture_root)
            ),
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
    /// TypeScript / TSX implementation export-name catalogs carry a mild
    /// `0.38` concavity: flatter than coherent anchors, but not as steep as
    /// prose bodies or tiny per-decl batches. Declaration files keep the
    /// default because their names surface is often the useful API anchor.
    /// JavaScript runtime export gates also keep the default; treating them
    /// as flat catalogs demotes load-bearing anchors. Large TS/TSX
    /// re-export-wall import chunks use the same catalog-shaped exponent.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            TsKey::ExportNames { file, .. }
                if crate::walker::typescript::is_ts_or_tsx_file(file)
                    && !crate::walker::typescript::is_declaration_file(file) =>
            {
                0.38
            }
            TsKey::ImportChunk { file, .. }
                if crate::walker::typescript::is_ts_or_tsx_file(file)
                    && !crate::walker::typescript::is_declaration_file(file) =>
            {
                0.38
            }
            TsKey::ExportMember { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn gated_descendant_value_weight(&self) -> f64 {
        // Broad, mostly type-only export-name surfaces are real gates:
        // individual exports can be high-value descendants, but none can
        // compete until the names surface lands. Runtime-heavy catalogs and
        // tiny type files keep their normal standalone rank.
        let TsKey::ExportNames {
            export_count,
            type_only_export_count,
            ..
        } = self
        else {
            return 0.0;
        };
        if *export_count < 10 {
            0.0
        } else {
            let type_only_ratio = *type_only_export_count as f64 / *export_count as f64;
            if type_only_ratio >= 0.75 {
                type_only_ratio
            } else {
                0.0
            }
        }
    }

    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            TsKey::ModuleDocLede { file } => {
                format!("module-doc lede in {}", display_path(file, fixture_root))
            }
            TsKey::Imports { file } => format!("imports in {}", display_path(file, fixture_root)),
            TsKey::ImportChunk { file, chunk_index } => {
                describe_chunked_surface("imports", file, *chunk_index, fixture_root)
            }
            TsKey::ExportNames {
                file, chunk_index, ..
            } => describe_chunked_surface("export names surface", file, *chunk_index, fixture_root),
            TsKey::Export { file, start_line } => {
                format!(
                    "export at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            TsKey::ExportDoc { file, start_line } => {
                format!(
                    "export doc at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            TsKey::ExportMember {
                file,
                start_line,
                member_start_line,
            } => {
                format!(
                    "export member at {}:{} member {}",
                    display_path(file, fixture_root),
                    start_line,
                    member_start_line
                )
            }
            TsKey::ExportBody {
                file,
                start_line,
                body_start_line,
            } => {
                format!(
                    "export body at {}:{} body {}",
                    display_path(file, fixture_root),
                    start_line,
                    body_start_line
                )
            }
            TsKey::ModuleItem { file, start_line } => {
                format!(
                    "module item at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            TsKey::ModuleItemBody {
                file,
                start_line,
                body_start_line,
            } => {
                format!(
                    "module item body at {}:{} body {}",
                    display_path(file, fixture_root),
                    start_line,
                    body_start_line
                )
            }
        }
    }
}

impl TomlKey {
    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            TomlKey::Identity { file } => {
                format!("[package] in {}", display_path(file, fixture_root))
            }
            TomlKey::Features { file } => {
                format!("[features] in {}", display_path(file, fixture_root))
            }
            TomlKey::Dependencies { file } => {
                format!("[dependencies] in {}", display_path(file, fixture_root))
            }
        }
    }
}

impl JsonKey {
    /// `Whole` carries a steeper concavity than the default for the same
    /// reason Markdown non-leading sections do: a verbatim 200-line
    /// `tsconfig.json` (or any non-package JSON config) has token cost
    /// that grows without proportional structural value. 0.45 matches the
    /// `MarkdownKey::Section` non-zero-index, `GoKey::Decl`, and
    /// `CKey::Decl` precedent — calibrated against the d2ts (5 nested
    /// configs) and cmdk (242-token tsconfig) divergence reports. The
    /// other JsonKey variants stay at the default — the package.json
    /// section batches are short and structural.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            JsonKey::Whole { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            JsonKey::Identity { file } => {
                format!("package identity in {}", display_path(file, fixture_root))
            }
            JsonKey::IdentityMeta { file } => {
                format!(
                    "package identity metadata in {}",
                    display_path(file, fixture_root)
                )
            }
            JsonKey::Entry { file } => format!(
                "package entrypoints in {}",
                display_path(file, fixture_root)
            ),
            JsonKey::Runtime { file } => format!(
                "package runtime metadata in {}",
                display_path(file, fixture_root)
            ),
            JsonKey::Scripts { file } => {
                format!("package scripts in {}", display_path(file, fixture_root))
            }
            JsonKey::Dependencies { file } => {
                format!(
                    "package dependencies in {}",
                    display_path(file, fixture_root)
                )
            }
            JsonKey::Whole { file } => format!("json config {}", display_path(file, fixture_root)),
        }
    }
}

impl PlaintextKey {
    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            PlaintextKey::Whole { file } => {
                format!("plaintext config {}", display_path(file, fixture_root))
            }
        }
    }
}

impl GoKey {
    /// `Decl` and `DeclBody` carry a steeper concavity than the default
    /// for the same reason as the C walker — Go top-level decls (a single
    /// type-spec line, a function signature, a `var Foo = expr`) are
    /// short, source files emit dozens of them, and the default 0.35
    /// exponent runs them up the rank against larger anchors. 0.45
    /// matches the C walker's calibrated value.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            GoKey::Decl { .. } | GoKey::DeclBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            GoKey::PackageImports { file } => {
                format!(
                    "go package + imports in {}",
                    display_path(file, fixture_root)
                )
            }
            GoKey::DeclNames { file } => {
                format!(
                    "go decl names surface in {}",
                    display_path(file, fixture_root)
                )
            }
            GoKey::Decl { file, start_line } => {
                format!(
                    "go decl at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            GoKey::DeclBody { file, start_line } => {
                format!(
                    "go decl body at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            GoKey::DeclDoc { file, start_line } => {
                format!(
                    "go decl doc at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            GoKey::TestNames { file } => {
                format!(
                    "go test names surface in {}",
                    display_path(file, fixture_root)
                )
            }
            GoKey::GoMod { file } => format!("go module file {}", display_path(file, fixture_root)),
        }
    }
}

impl PythonKey {
    /// `DeclNames` is the broad predecessor names surface and carries a
    /// milder tuned `0.37` concavity; large re-export-wall import chunks use
    /// the same catalog-shaped exponent. Per-decl / per-method batches carry
    /// the same 0.45 concavity as
    /// the C / Go walkers — Python decls are short (a single `def
    /// name(...):`, a single `class X(Base):` line), source files
    /// emit dozens of them, and the default 0.35 lets every tiny one
    /// out-rank larger anchors. `ClassBody` keeps the default because
    /// field listings have a structural tie to the class.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            PythonKey::ImportChunk { .. } => 0.37,
            PythonKey::DeclNames { .. } => 0.37,
            PythonKey::Decl { .. }
            | PythonKey::DeclBody { .. }
            | PythonKey::Method { .. }
            | PythonKey::MethodBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            PythonKey::Imports { file } => {
                format!("python imports in {}", display_path(file, fixture_root))
            }
            PythonKey::ImportChunk { file, chunk_index } => {
                describe_chunked_surface("python imports", file, *chunk_index, fixture_root)
            }
            PythonKey::DeclNames { file, chunk_index } => describe_chunked_surface(
                "python decl names surface",
                file,
                *chunk_index,
                fixture_root,
            ),
            PythonKey::Decl { file, start_line } => {
                format!(
                    "python decl at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            PythonKey::DeclDoc { file, start_line } => {
                format!(
                    "python decl doc at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            PythonKey::DeclBody {
                file,
                start_line,
                body_start_line,
            } => {
                format!(
                    "python decl body at {}:{} body {}",
                    display_path(file, fixture_root),
                    start_line,
                    body_start_line
                )
            }
            PythonKey::ClassBody { file, start_line } => {
                format!(
                    "python class body at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            PythonKey::MethodSigs { file, chunk_index } => {
                describe_chunked_surface("python method sigs", file, *chunk_index, fixture_root)
            }
            PythonKey::Method { file, start_line } => {
                format!(
                    "python method at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            PythonKey::MethodDoc { file, start_line } => {
                format!(
                    "python method doc at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            PythonKey::MethodBody {
                file,
                start_line,
                body_start_line,
            } => {
                format!(
                    "python method body at {}:{} body {}",
                    display_path(file, fixture_root),
                    start_line,
                    body_start_line
                )
            }
            PythonKey::TestNames { file } => {
                format!(
                    "python test names surface in {}",
                    display_path(file, fixture_root)
                )
            }
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

    pub fn describe(&self, fixture_root: &Path) -> String {
        match self {
            CKey::HeaderBanner { file } => {
                format!("c header banner in {}", display_path(file, fixture_root))
            }
            CKey::Includes { file } => {
                format!("c includes in {}", display_path(file, fixture_root))
            }
            CKey::DeclNames { file, chunk_index } => {
                describe_chunked_surface("c decl names surface", file, *chunk_index, fixture_root)
            }
            CKey::Decl { file, start_line } => {
                format!(
                    "c decl at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            CKey::DeclBody { file, start_line } => {
                format!(
                    "c decl body at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
            CKey::DeclDoc { file, start_line } => {
                format!(
                    "c decl doc at {}:{}",
                    display_path(file, fixture_root),
                    start_line
                )
            }
        }
    }
}

fn display_path(path: &Path, fixture_root: &Path) -> String {
    path.strip_prefix(fixture_root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn describe_chunked_surface(
    label: &str,
    file: &Path,
    chunk_index: usize,
    fixture_root: &Path,
) -> String {
    if chunk_index == 0 {
        format!("{label} in {}", display_path(file, fixture_root))
    } else {
        format!(
            "{label} #{chunk_index} in {}",
            display_path(file, fixture_root)
        )
    }
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
