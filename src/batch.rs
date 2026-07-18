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

/// Default walker-key sum. Scheduler/render code depends on the
/// [`WalkerKey`] trait, not this enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BatchKey {
    Fs(FsKey),
    Rust(RustKey),
    Markdown(MarkdownKey),
    Toml(TomlKey),
    Typescript(TsKey),
    Json(JsonKey),
    Plaintext(PlaintextKey),
    Prisma(PrismaKey),
    C(CKey),
    Go(GoKey),
    Python(PythonKey),
    Lua(LuaKey),
    Yaml(YamlKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FsKey {
    /// Listing of immediate children of `dir`.
    DirListing { dir: PathBuf },
}

/// Generates the `From<XKey> for BatchKey` forwarders and the
/// `WalkerKey for BatchKey` dispatch from a single variant list. Each
/// inner key implements [`InnerKey`] for the dispatch body.
macro_rules! impl_batchkey {
    ($($variant:ident => $key:ident),* $(,)?) => {
        $(
            impl From<$key> for BatchKey {
                fn from(k: $key) -> Self { BatchKey::$variant(k) }
            }
        )*

        impl WalkerKey for BatchKey {
            fn describe(&self, fixture_root: &Path) -> String {
                match self { $(BatchKey::$variant(k) => InnerKey::describe(k, fixture_root),)* }
            }
            fn concavity_exponent(&self) -> f64 {
                match self { $(BatchKey::$variant(k) => InnerKey::concavity_exponent(k),)* }
            }
            fn is_orientation(&self) -> bool {
                match self { $(BatchKey::$variant(k) => InnerKey::is_orientation(k),)* }
            }
            fn is_deferred_mass_prose(&self) -> bool {
                match self { $(BatchKey::$variant(k) => InnerKey::is_deferred_mass_prose(k),)* }
            }
            fn is_depth_follow_up(&self) -> bool {
                match self { $(BatchKey::$variant(k) => InnerKey::is_depth_follow_up(k),)* }
            }
        }
    };
}

impl_batchkey! {
    Fs => FsKey,
    Rust => RustKey,
    Markdown => MarkdownKey,
    Toml => TomlKey,
    Typescript => TsKey,
    Json => JsonKey,
    Plaintext => PlaintextKey,
    Prisma => PrismaKey,
    C => CKey,
    Go => GoKey,
    Python => PythonKey,
    Lua => LuaKey,
    Yaml => YamlKey,
}

/// Per-walker contributions to the [`WalkerKey`] dispatch on
/// [`BatchKey`]. Defaults match [`WalkerKey`]'s defaults so walkers
/// only implement the methods they override.
trait InnerKey {
    fn describe(&self, fixture_root: &Path) -> String;
    fn concavity_exponent(&self) -> f64 {
        crate::value::DEFAULT_CONCAVITY_EXPONENT
    }
    fn is_orientation(&self) -> bool {
        false
    }
    fn is_deferred_mass_prose(&self) -> bool {
        false
    }
    /// True for depth follow-up batches — doc/body/member refinements
    /// of an already-delivered surface. Drives the scheduler's
    /// breadth-pressure penalty; surfaces and orientation never
    /// qualify.
    fn is_depth_follow_up(&self) -> bool {
        false
    }
}

/// Rust batches — per-item for pub types, file-scope for crate-doc /
/// mod-use / methods, cross-file for the `#[macro_export]` surface.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RustKey {
    /// `//!` module-doc lede — first paragraph only, entrypoint files.
    CrateDocLede { file: PathBuf },
    /// `//!` module-doc body — after the first paragraph. Predecessor:
    /// matching `CrateDocLede`, or `None` when the crate doc opens with
    /// a heading and no Lede candidate is emitted. When the body is
    /// oversize it shrinks to the first ~200-token chunk, followed by
    /// chained `CrateDocTail` chunks.
    CrateDocBody { file: PathBuf },
    /// One ~200-token continuation chunk of an oversize crate-doc
    /// body, cut at blank doc lines outside doc code fences.
    /// Predecessor: the previous chunk (`CrateDocBody` for the first
    /// tail). Scheduler-trait defaults (flat concavity, no breadth
    /// pressure) are the initial shipped state, not yet swept —
    /// `PubItemDocBody` steepens to 0.45 and markdown's `OversizeTail`
    /// chunks steepen via `Section` index ≥ 1, so those are the
    /// candidates if the tail train over-buys at higher budgets.
    CrateDocTail { file: PathBuf, start_line: usize },
    /// Contiguous top-of-file `#![…]` inner-attribute block (with its
    /// interleaved comment lines), entrypoint files only.
    CrateAttrs { file: PathBuf },
    /// `use` + `mod` + `pub use` plumbing at the top of a file.
    ModUse { file: PathBuf },
    /// Surface listing of every top-level `pub` item name in a file —
    /// catastrophic-omission hedge.
    PubItemNames { file: PathBuf },
    /// Whole top-level `pub` item (sig with body marker for fn).
    PubItem { file: PathBuf, start_line: usize },
    /// Body slice of a public fn, split by top-level statement.
    /// Predecessor: matching `PubItem`.
    PubItemBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
    /// Private top-level item in a Rust entrypoint file. Functions render
    /// as sig with body ellipses; non-functions render whole.
    EntryItem { file: PathBuf, start_line: usize },
    /// Body slice of a private entrypoint fn, split by top-level statement.
    /// Predecessor: matching `EntryItem`.
    EntryItemBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
    /// Rustdoc up to the first `# Heading`. Predecessor: `PubItem`.
    PubItemDocLede { file: PathBuf, start_line: usize },
    /// Rustdoc body from the first `# Heading` onward. Predecessor:
    /// `PubItemDocLede` if any, else `PubItem`.
    PubItemDocBody { file: PathBuf, start_line: usize },
    /// Impl-block headers + method signatures in a single file.
    MethodSigs { file: PathBuf },
    /// Private function signature plus same-constructor registration call anchors.
    RegistrationRoster { file: PathBuf, start_line: usize },
    /// `#[macro_export] macro_rules!` names across `src_dir`.
    MacroNames { src_dir: PathBuf },
    /// Full body of one `#[macro_export] macro_rules!`. Predecessor:
    /// `MacroNames` for the enclosing `src_dir`.
    MacroBody { file: PathBuf, start_line: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownKey {
    /// Whole `SUMMARY.md` (mdBook ToC).
    SummaryWhole { file: PathBuf },
    /// README headline: first heading + first paragraph.
    ReadmeHeadline { file: PathBuf },
    /// Every H1/H2/H3 heading line (H2+H3 only on READMEs). Predecessor
    /// of every same-file `Section`.
    HeadingsOutline { file: PathBuf },
    /// One scheduling unit of a markdown body, indexed by post-split
    /// position. May be whole H2 / per-bullet / Intro+per-H3 child.
    /// `reference_shaped` marks README ranges dominated by list / table
    /// / fence rows — they keep the default concavity instead of the
    /// steeper prose exponent.
    /// `deferred_mass_prose` marks operationally dense prose that only
    /// competes in the late scheduler window.
    Section {
        file: PathBuf,
        section_index: usize,
        reference_shaped: bool,
        deferred_mass_prose: bool,
    },
}

/// TypeScript / TSX batches. "Public" = top-level with `export` (or
/// `default` export). Body-less re-exports (`export { foo } from '…'`)
/// fold into `Imports`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TsKey {
    /// Module-level JSDoc lede. Entrypoint files only.
    ModuleDocLede { file: PathBuf },
    /// `import`, side-effect imports, and bare `export … from` re-exports.
    Imports { file: PathBuf },
    /// Chunked `Imports` for entrypoint files that are mostly re-export
    /// walls; each chunk groups consecutive imports from the same source.
    ImportChunk { file: PathBuf, chunk_index: usize },
    /// Surface listing of every top-level export's first line — one
    /// unified catalog per file; catastrophic-omission hedge.
    ExportNames { file: PathBuf },
    /// Top-level export's declaration (sig with body marker for fn).
    Export { file: PathBuf, start_line: usize },
    /// JSDoc above a single export. Predecessor: matching `Export`.
    ExportDoc { file: PathBuf, start_line: usize },
    /// One member of an exported JS class. Predecessor: matching class
    /// `Export`.
    ExportMember {
        file: PathBuf,
        /// Parent export line.
        start_line: usize,
        /// First line of the class member surface.
        member_start_line: usize,
    },
    /// Whole member-name catalog of one big exported declaration
    /// (interface / object-type alias / class above the per-member
    /// split range). Predecessor: the matching `Export` header.
    ExportMemberNames {
        file: PathBuf,
        /// Parent export line.
        start_line: usize,
    },
    /// One source-order partition of an oversized member-name catalog.
    /// Predecessor: the file's `ExportNames` surface.
    ExportMemberNamesChunk {
        file: PathBuf,
        /// Parent export line.
        start_line: usize,
        /// Zero-based source-order partition index.
        chunk_index: usize,
    },
    /// Continuation chunk of one oversized exported class declaration.
    /// Predecessor: the matching `Export` head or previous tail chunk.
    ExportTail {
        file: PathBuf,
        /// Parent export line.
        start_line: usize,
        /// Zero-based tail index (the `Export` head is implicit chunk 0).
        chunk_index: usize,
    },
    /// Body slice of an export with a `statement_block` body (outer
    /// braces stripped). Predecessor: matching `Export`.
    ExportBody {
        file: PathBuf,
        start_line: usize,
        /// Disambiguates sibling body slices.
        body_start_line: usize,
    },
    /// Unified first-line catalog of a private-emitting entrypoint's
    /// module items — one roster instead of a per-item train.
    /// Predecessor of each `ModuleItem`.
    ModuleItemNames { file: PathBuf },
    /// Top-level non-exported decl — module-private classes, helpers.
    ModuleItem { file: PathBuf, start_line: usize },
    /// Body slice of a non-exported decl. Predecessor: matching
    /// `ModuleItem`.
    ModuleItemBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
}

/// JSON batches. `package.json` splits along the `Cargo.toml` ontology
/// (identity / scripts / deps) plus a JS entrypoint-pointer batch. Other
/// small JSON configs get a single `Whole` batch.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JsonKey {
    /// `package.json` identity scalars: `name`, `version`, `description`,
    /// `type`, `private`, `license`/`licenses`.
    Identity { file: PathBuf },
    /// Auxiliary `package.json` metadata: authorship, repository/homepage,
    /// bugs, keywords, publish config, funding.
    IdentityMeta { file: PathBuf },
    /// `package.json` entrypoint pointers (`main`/`module`/`exports`/
    /// `bin`/`types`/`files`/...).
    Entry { file: PathBuf },
    /// `package.json` runtime/toolchain constraints (`engines`,
    /// `packageManager`).
    Runtime { file: PathBuf },
    /// `package.json` `scripts` block.
    Scripts { file: PathBuf },
    /// Runtime `package.json` dependency blocks (`dependencies`, optional /
    /// bundled dependencies, overrides, and resolutions).
    Dependencies { file: PathBuf },
    /// Development and consumer-contract dependency blocks
    /// (`devDependencies`, `peerDependencies`, `peerDependenciesMeta`).
    DevDependencies { file: PathBuf },
    /// Whole-file render of a small JSON config. Skipped for
    /// `package.json` and for large/generated files.
    Whole { file: PathBuf },
}

/// Plaintext config / license file batches. Whitelist lives in
/// [`crate::walker::plaintext`]. Capped on line + token cost to bound
/// render-time displacement of richer walker batches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlaintextKey {
    /// Whole-file render. Skipped when line count or rendered token
    /// cost exceeds the walker's caps. For split classes (dotenv
    /// samples, long Dockerfiles) this key carries the head slice
    /// instead, with the rest under [`PlaintextKey::Tail`].
    Whole { file: PathBuf },
    /// Rows complementary to a head-shaped `Whole` batch — dotenv
    /// samples: the optional-settings roster after the mandatory head;
    /// Dockerfiles: the build-mechanics body interleaved between the
    /// head's contract lines.
    Tail { file: PathBuf },
    /// `NAME` + `DESCRIPTION`-lede slice of a troff man page — the
    /// "what is this tool" answer for a CLI shipping a `*.1` / `*.5`
    /// (or autotools `*.1.in`) manual.
    ManLede { file: PathBuf },
}

/// YAML batches. Narrowly scoped to operational configs and root
/// reference/spec maps whose top-level keys are useful orientation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum YamlKey {
    /// Whole-file render of a `docker-compose.{yml,yaml}` file. Skipped
    /// when the source exceeds the walker's line cap.
    Whole { file: PathBuf },
    /// Root and child keys in a root reference/API/spec map.
    TopLevelKeys { file: PathBuf },
}

/// Prisma schema batches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrismaKey {
    /// One line per top-level `model` / `enum` / `datasource` /
    /// `generator` declaration in a `schema.prisma` — catastrophic-
    /// omission hedge.
    Toc { file: PathBuf },
    /// Full brace-block body of one top-level `model` / `enum` /
    /// `datasource` / `generator` declaration. Predecessor: the
    /// enclosing `Toc`. A very large `model` is split into a
    /// head + tail at its row midpoint so high-value identity /
    /// relation fields schedule ahead of archival-default fields.
    Decl { file: PathBuf, start_line: usize },
    /// Tail slice of a split large `model` body. Predecessor: the
    /// matching head `Decl` at `start_line`.
    DeclTail {
        file: PathBuf,
        start_line: usize,
        tail_start_line: usize,
    },
}

/// C / C-header batches. "Public" rule: top-level
/// `function_definition` / `declaration` / `type_definition` /
/// `preproc_def` / `preproc_function_def` that aren't `static` (in `.c`
/// files), plus `static inline` fn defs in `.h` files. Header-guard
/// `#ifndef`/`#define`/`#endif` is descended transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CKey {
    /// Whole small header rendered verbatim in one batch — replaces the
    /// `HeaderBanner`/`Includes`/`DeclNames`/per-`Decl` decomposition for
    /// headers small enough that the decomposition only fragments their
    /// public surface and strips macro values / `#ifdef` shape.
    WholeFile { file: PathBuf },
    /// Top-of-file `/* */` banner comment (license / brief).
    HeaderBanner { file: PathBuf },
    /// `#include` directives.
    Includes { file: PathBuf },
    /// Names-surface chunk for top-level public decls.
    DeclNames { file: PathBuf, chunk_index: usize },
    /// One top-level public declaration (sig with body marker for fn).
    Decl { file: PathBuf, start_line: usize },
    /// Body interior of a fn definition. Predecessor: matching `Decl`.
    DeclBody { file: PathBuf, start_line: usize },
    /// Doc comment(s) above a decl. Predecessor: matching `Decl`.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Blank-line-separated field group inside a big struct/union body,
    /// or a sized chunk of a big enum body. Predecessor: `Decl` (the
    /// Decl span is trimmed to header + closer).
    AggregateMemberGroup {
        file: PathBuf,
        start_line: usize,
        group_start_line: usize,
    },
}

/// Go batches. Emits all top-level decls regardless of export status
/// — visibility is a value discount, not a filter. Grouped decls
/// stay as one batch so iota / shared-comment semantics survive.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GoKey {
    /// `// Package foo …` doc comment above `package`.
    PackageDocLede { file: PathBuf },
    /// Remainder of a `/* */` package godoc past the lede paragraph —
    /// conventionally the package's canonical usage example.
    /// Predecessor: matching `PackageDocLede`.
    PackageDocBody { file: PathBuf },
    /// Package clause + import block at the top of a `.go` file.
    PackageImports { file: PathBuf },
    /// Names-surface chunk for top-level decls (visibility-blind).
    DeclNames { file: PathBuf, chunk_index: usize },
    /// One top-level declaration (sig with body marker for fn/method).
    Decl { file: PathBuf, start_line: usize },
    /// Body interior of a fn/method def. Predecessor: matching `Decl`.
    DeclBody { file: PathBuf, start_line: usize },
    /// `//`/`/* */` run above a decl. Predecessor: matching `Decl`.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Blank-line-separated field group in a big struct. Predecessor:
    /// `Decl` (Decl span is trimmed to header + closer).
    StructFieldGroup {
        file: PathBuf,
        start_line: usize,
        group_start_line: usize,
    },
    /// Names surface for `Test*`/`Benchmark*`/`Example*` in `*_test.go`.
    TestNames { file: PathBuf },
    /// Identity slice of a `go.mod` / `go.work` — `module`, `go`,
    /// `toolchain`. Predecessor of `GoMod`.
    GoModIdentity { file: PathBuf },
    /// Whole-file render of a `go.mod` / `go.work`. Line-capped.
    /// Predecessor: matching `GoModIdentity`.
    GoMod { file: PathBuf },
}

/// Python batches. All top-level + class-body items emit; visibility
/// is a value discount, not a filter. Decorated defs use the
/// `decorated_definition` wrapper as the unit (decorator rows included).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PythonKey {
    /// `import` / `from … import …` + module docstring + `__all__` +
    /// module-level dunder assignments.
    Imports { file: PathBuf },
    /// Chunked `Imports` for large `__init__.py` re-export walls.
    ImportChunk { file: PathBuf, chunk_index: usize },
    /// Names surface for top-level class/def/non-dunder consts — one
    /// unified catalog per file unless the rendered surface exceeds the
    /// Python walker's oversize threshold. For an oversize catalog this
    /// remains the first chunk, preserving the established key/value.
    DeclNames { file: PathBuf },
    /// Continuation of an oversize top-level names surface. Chained after
    /// `DeclNames` (and then the preceding continuation) so the catalog
    /// retains source-roster order without creating independently
    /// schedulable crumbs.
    DeclNamesChunk { file: PathBuf, chunk_index: usize },
    /// One top-level item — header + up to 2 docstring-summary rows
    /// for class/def, or assignment line(s) for const.
    Decl { file: PathBuf, start_line: usize },
    /// Top-level def/class docstring — the lede paragraph when the
    /// docstring is split. Predecessor: matching `Decl`.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Remainder of a split docstring (parameter docs, examples).
    /// Predecessor: matching `DeclDoc`.
    DeclDocRest { file: PathBuf, start_line: usize },
    /// Body slice of a top-level def (docstring excluded). Predecessor:
    /// matching `Decl`.
    DeclBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
    /// Class body excluding method-def signatures and the leading
    /// docstring — TypedDict / dataclass / Protocol / Pydantic fields,
    /// `__slots__`, class-level constants. Predecessor: matching
    /// class `Decl`.
    ClassBody { file: PathBuf, start_line: usize },
    /// Surface listing of every method's inner `def` line across every
    /// top-level class; decorator rows are owned by the per-method batch.
    /// Catastrophic-omission hedge.
    MethodSigs { file: PathBuf },
    /// Method-level `Decl` analog for a method inside a top-level
    /// class. Predecessor: enclosing class's `Decl`.
    Method { file: PathBuf, start_line: usize },
    /// Method's docstring. Predecessor: matching `Method`.
    MethodDoc { file: PathBuf, start_line: usize },
    /// Method body slice, split by top-level statement, sans leading
    /// docstring. Predecessor: matching `Method`.
    MethodBody {
        file: PathBuf,
        start_line: usize,
        body_start_line: usize,
    },
    /// Legacy packaging dependency metadata: the `install_requires`
    /// keyword argument span within the top-level `setup(...)` call.
    SetupManifest { file: PathBuf, start_line: usize },
    /// Surface listing of every `def test_*` first line in a `test_*.py`
    /// / `*_test.py` file (top-level + class-body, decorator-aware).
    TestNames { file: PathBuf },
}

/// Lua batches. LuaCATS spec files (`---@meta`, `---@class`,
/// `---@alias`) get whole-file rendering when small; other Lua sources
/// get the C/Python-style per-decl breakdown.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LuaKey {
    /// Top-of-file comment block (license / brief).
    Banner { file: PathBuf },
    /// Top-of-file module identity table — a `local M = { _VERSION =
    /// …, _DESCRIPTION = …, _URL = … }` metadata block (the standard
    /// Lua library "what is this" idiom), truncated before long-string
    /// fields like `_LICENSE = [[`.
    ModuleIdentity { file: PathBuf },
    /// Whole-file rendering for LuaCATS spec files (`---@meta` at top,
    /// or majority-LuaCATS-tag comment density). Gated to small files.
    MetaFileWhole { file: PathBuf },
    /// Surface listing of every top-level fn name + table-method
    /// assignment first line — one unified catalog per file.
    DeclNames { file: PathBuf },
    /// One top-level fn-like declaration's signature/header. Covers
    /// `function foo()`, `local function foo()`, and `M.foo = function(...)`.
    /// Keyed by start line.
    Decl { file: PathBuf, start_line: usize },
    /// LuaCATS `---@` comment block immediately above a decl.
    /// Predecessor: matching `Decl`.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Body interior of a fn decl. Predecessor: matching `Decl`.
    DeclBody { file: PathBuf, start_line: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TomlKey {
    /// `[package]` or `[workspace.package]` identity block.
    Identity { file: PathBuf },
    /// Entry-point console scripts: `[project.scripts]` (PEP 621) or
    /// `[tool.poetry.scripts]` — the "how do I run this" surface.
    Scripts { file: PathBuf },
    /// `[features]` table.
    Features { file: PathBuf },
    /// Ordinary `[dependencies]` / `[workspace.dependencies]` tables,
    /// plus Python-manifest dependency sections.
    Dependencies { file: PathBuf },
    /// Cargo `[dev-dependencies]`, `[build-dependencies]`, and target-
    /// conditional dependency tables.
    DevelopmentDependencies { file: PathBuf },
    /// One top-level Python-manifest `tool.<name>` family, including its
    /// descendants, or a compact family of adjacent small tool tables.
    ToolConfig { file: PathBuf, tool: String },
    /// Manifest-level operational config outside Python `tool.*` families:
    /// build systems, package metadata, Cargo targets, and profiles.
    Config { file: PathBuf },
}

/// Opaque walker-key contract — scheduler + renderer depend on this
/// trait so a new walker doesn't touch them.
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
    /// One-line human descriptor for snapshots and divergence reports.
    /// `fixture_root` is stripped from embedded paths.
    fn describe(&self, fixture_root: &Path) -> String;

    /// Per-key cost concavity for the scheduling ratio
    /// (`value / cost^exponent`). Raise on prose-shaped batches.
    fn concavity_exponent(&self) -> f64 {
        crate::value::DEFAULT_CONCAVITY_EXPONENT
    }

    /// True for orientation-class batches — README / man-page
    /// orientation prose, manifest identity — as opposed to
    /// source-code bodies. Directory listings (`FsKey`) are
    /// deliberately *not* orientation: boosting the cheap dir-listing
    /// flood measured negative (see
    /// [`crate::value::ORIENTATION_TIER_WINDOW`]'s doc). The scheduler
    /// gives these an early-budget ratio boost (see
    /// [`crate::value::orientation_tier_multiplier`]) so high-rank
    /// orientation atoms aren't crowded out of the first ~1K tokens by
    /// cheap high-ratio code chunks.
    fn is_orientation(&self) -> bool {
        false
    }

    /// True for prose batches that should only receive their ratio
    /// lift after the protected early source window.
    fn is_deferred_mass_prose(&self) -> bool {
        false
    }

    /// True for depth follow-up batches — doc/body/member refinements
    /// of an already-delivered surface. Drives the scheduler's
    /// breadth-pressure penalty.
    fn is_depth_follow_up(&self) -> bool {
        false
    }
}

impl InnerKey for FsKey {
    fn describe(&self, root: &Path) -> String {
        let FsKey::DirListing { dir } = self;
        let shown = display_path(dir, root);
        if shown.is_empty() {
            "listing of '.'".to_string()
        } else {
            format!("listing of '{shown}'")
        }
    }
}

impl InnerKey for RustKey {
    fn is_depth_follow_up(&self) -> bool {
        // `EntryItemBody` is deliberately absent: entrypoint internals
        // are the "how does this app work" spine, and NS authors rank
        // that dive as wanted depth (otree main.rs args->config parts:
        // -0.131 with it pressured).
        matches!(
            self,
            RustKey::PubItemBody { .. }
                | RustKey::PubItemDocLede { .. }
                | RustKey::PubItemDocBody { .. }
                | RustKey::MacroBody { .. }
        )
    }

    /// `PubItemDocBody` steepens to `0.45` — rustdoc prose past the
    /// first heading grows in cost without proportional structural value.
    /// `CrateDocBody` stays at the default (its bullets carry credit).
    fn concavity_exponent(&self) -> f64 {
        match self {
            RustKey::PubItemDocBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            RustKey::CrateDocLede { file } => describe_in("crate-doc lede", file, root),
            RustKey::CrateDocBody { file } => describe_in("crate-doc body", file, root),
            RustKey::CrateDocTail { file, start_line } => {
                describe_at("crate-doc tail", file, *start_line, root)
            }
            RustKey::CrateAttrs { file } => describe_in("crate attributes", file, root),
            RustKey::ModUse { file } => describe_in("mod/use plumbing", file, root),
            RustKey::PubItemNames { file } => describe_in("pub-item names surface", file, root),
            RustKey::PubItem { file, start_line } => {
                describe_at("pub item", file, *start_line, root)
            }
            RustKey::PubItemBody {
                file,
                start_line,
                body_start_line,
            } => describe_at_body("pub item body", file, *start_line, *body_start_line, root),
            RustKey::EntryItem { file, start_line } => {
                describe_at("entry item", file, *start_line, root)
            }
            RustKey::EntryItemBody {
                file,
                start_line,
                body_start_line,
            } => describe_at_body("entry item body", file, *start_line, *body_start_line, root),
            RustKey::PubItemDocLede { file, start_line } => {
                describe_at("pub-item doc lede", file, *start_line, root)
            }
            RustKey::PubItemDocBody { file, start_line } => {
                describe_at("pub-item doc body", file, *start_line, root)
            }
            RustKey::MethodSigs { file } => describe_in("impl method sigs", file, root),
            RustKey::RegistrationRoster { file, start_line } => {
                describe_at("registration roster", file, *start_line, root)
            }
            RustKey::MacroNames { src_dir } => {
                format!("macro_export names across {}", display_path(src_dir, root))
            }
            RustKey::MacroBody { file, start_line } => {
                describe_at("macro_export body", file, *start_line, root)
            }
        }
    }
}

impl InnerKey for MarkdownKey {
    fn is_orientation(&self) -> bool {
        !matches!(
            self,
            MarkdownKey::Section {
                deferred_mass_prose: true,
                ..
            }
        )
    }
    fn describe(&self, root: &Path) -> String {
        match self {
            MarkdownKey::SummaryWhole { file } => {
                format!("mdBook SUMMARY at {}", display_path(file, root))
            }
            MarkdownKey::ReadmeHeadline { file } => describe_in("README headline", file, root),
            MarkdownKey::HeadingsOutline { file } => describe_in("headings outline", file, root),
            MarkdownKey::Section {
                file,
                section_index,
                ..
            } => format!("{} section #{section_index}", display_path(file, root)),
        }
    }

    /// `Section` at index ≥1 steepens to `0.45` to demote prose body
    /// against structural anchors of the same value; index 0 keeps the
    /// default since READMEs often lead with their canonical claim.
    /// Reference-shaped ranges (list / table / fence dominant) are
    /// catalogs, not prose — they keep the default exponent.
    fn concavity_exponent(&self) -> f64 {
        match self {
            MarkdownKey::Section {
                section_index,
                reference_shaped,
                ..
            } if *section_index >= 1 && !reference_shaped => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    fn is_deferred_mass_prose(&self) -> bool {
        matches!(
            self,
            MarkdownKey::Section {
                deferred_mass_prose: true,
                ..
            }
        )
    }
}

impl InnerKey for TsKey {
    fn is_depth_follow_up(&self) -> bool {
        matches!(
            self,
            TsKey::ExportBody { .. }
                | TsKey::ExportMember { .. }
                | TsKey::ExportTail { .. }
                | TsKey::ModuleItemBody { .. }
        )
    }

    /// `ExportNames` / `ImportChunk` for TS/TSX impl files use a mild
    /// `0.38` (flatter than per-decl, steeper than coherent anchors).
    /// Declaration files and JS runtime exports keep the default —
    /// flattening them demotes load-bearing anchors. `ExportMember`
    /// uses `0.45` (per-decl tier).
    fn concavity_exponent(&self) -> f64 {
        match self {
            TsKey::ExportNames { file, .. } | TsKey::ImportChunk { file, .. }
                if crate::walker::typescript::is_ts_or_tsx_file(file)
                    && !crate::walker::typescript::is_declaration_file(file) =>
            {
                0.38
            }
            TsKey::ExportMember { .. } => 0.45,
            // Roster tier for the unified member catalog — measured:
            // dropping it to the default leaves axios flat and costs
            // commander -0.212 (2026-07-06).
            TsKey::ExportMemberNames { .. }
            | TsKey::ExportMemberNamesChunk { .. }
            | TsKey::ModuleItemNames { .. } => 0.37,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            TsKey::ModuleDocLede { file } => describe_in("module-doc lede", file, root),
            TsKey::Imports { file } => describe_in("imports", file, root),
            TsKey::ImportChunk { file, chunk_index } => {
                describe_chunked_surface("imports", file, *chunk_index, root)
            }
            TsKey::ExportNames { file } => describe_in("export names surface", file, root),
            TsKey::Export { file, start_line } => describe_at("export", file, *start_line, root),
            TsKey::ExportDoc { file, start_line } => {
                describe_at("export doc", file, *start_line, root)
            }
            TsKey::ExportMember {
                file,
                start_line,
                member_start_line,
            } => format!(
                "export member at {}:{start_line} member {member_start_line}",
                display_path(file, root)
            ),
            TsKey::ExportMemberNames { file, start_line } => format!(
                "export member names at {}:{start_line}",
                display_path(file, root)
            ),
            TsKey::ExportMemberNamesChunk {
                file,
                start_line,
                chunk_index,
            } => format!(
                "export member names #{} at {}:{start_line}",
                chunk_index + 1,
                display_path(file, root)
            ),
            TsKey::ExportTail {
                file,
                start_line,
                chunk_index,
            } => format!(
                "export tail #{} at {}:{start_line}",
                chunk_index + 1,
                display_path(file, root)
            ),
            TsKey::ExportBody {
                file,
                start_line,
                body_start_line,
            } => describe_at_body("export body", file, *start_line, *body_start_line, root),
            TsKey::ModuleItemNames { file } => describe_in("module item names surface", file, root),
            TsKey::ModuleItem { file, start_line } => {
                describe_at("module item", file, *start_line, root)
            }
            TsKey::ModuleItemBody {
                file,
                start_line,
                body_start_line,
            } => describe_at_body(
                "module item body",
                file,
                *start_line,
                *body_start_line,
                root,
            ),
        }
    }
}

impl InnerKey for TomlKey {
    fn is_orientation(&self) -> bool {
        !matches!(self, TomlKey::ToolConfig { .. } | TomlKey::Config { .. })
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            TomlKey::Identity { file } => describe_in("[package]", file, root),
            TomlKey::Scripts { file } => describe_in("entry-point scripts", file, root),
            TomlKey::Features { file } => describe_in("[features]", file, root),
            TomlKey::Dependencies { file } => describe_in("[dependencies]", file, root),
            TomlKey::DevelopmentDependencies { file } => {
                describe_in("dev/build/target dependencies", file, root)
            }
            TomlKey::ToolConfig { file, tool } => {
                describe_in(&format!("tool.{tool} config"), file, root)
            }
            TomlKey::Config { file } => describe_in("manifest config", file, root),
        }
    }
}

impl InnerKey for JsonKey {
    /// `Whole` steepens to `0.45` — verbatim JSON config bodies grow
    /// in cost without proportional structural value. Other variants
    /// (package.json sections) stay at the default; they're short and
    /// structural.
    fn concavity_exponent(&self) -> f64 {
        match self {
            JsonKey::Whole { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    /// Manifest sections (identity / entrypoints / scripts / deps) are
    /// orientation; a verbatim `Whole` JSON config dump is not.
    fn is_orientation(&self) -> bool {
        !matches!(self, JsonKey::Whole { .. })
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            JsonKey::Identity { file } => describe_in("package identity", file, root),
            JsonKey::IdentityMeta { file } => describe_in("package identity metadata", file, root),
            JsonKey::Entry { file } => describe_in("package entrypoints", file, root),
            JsonKey::Runtime { file } => describe_in("package runtime metadata", file, root),
            JsonKey::Scripts { file } => describe_in("package scripts", file, root),
            JsonKey::Dependencies { file } => {
                describe_in("package runtime dependencies", file, root)
            }
            JsonKey::DevDependencies { file } => {
                describe_in("package dev/peer dependencies", file, root)
            }
            JsonKey::Whole { file } => format!("json config {}", display_path(file, root)),
        }
    }
}

impl InnerKey for PlaintextKey {
    /// A man-page NAME/DESCRIPTION lede is orientation; a verbatim
    /// plaintext config dump is not.
    fn is_orientation(&self) -> bool {
        matches!(self, PlaintextKey::ManLede { .. })
    }
    fn describe(&self, root: &Path) -> String {
        match self {
            PlaintextKey::Whole { file } => {
                format!("plaintext config {}", display_path(file, root))
            }
            PlaintextKey::Tail { file } => {
                format!("plaintext config tail of {}", display_path(file, root))
            }
            PlaintextKey::ManLede { file } => {
                format!(
                    "man-page NAME + DESCRIPTION in {}",
                    display_path(file, root)
                )
            }
        }
    }
}

impl InnerKey for PrismaKey {
    fn describe(&self, root: &Path) -> String {
        match self {
            PrismaKey::Toc { file } => describe_in("Prisma schema TOC", file, root),
            PrismaKey::Decl { file, start_line } => {
                describe_at("Prisma decl", file, *start_line, root)
            }
            PrismaKey::DeclTail {
                file,
                start_line,
                tail_start_line,
            } => describe_at_body(
                "Prisma decl tail",
                file,
                *start_line,
                *tail_start_line,
                root,
            ),
        }
    }
}

impl InnerKey for YamlKey {
    fn describe(&self, root: &Path) -> String {
        match self {
            YamlKey::Whole { file } => format!("YAML config at {}", display_path(file, root)),
            YamlKey::TopLevelKeys { file } => {
                format!(
                    "YAML reference map key roster in {}",
                    display_path(file, root)
                )
            }
        }
    }
}

impl InnerKey for GoKey {
    fn is_depth_follow_up(&self) -> bool {
        // `DeclDoc` is deliberately absent: godoc comments are the API
        // documentation in Go convention, and NS authors rank a
        // primary file's doc train as wanted depth (bubbletea tea.go:
        // -0.048 with it pressured).
        matches!(self, GoKey::DeclBody { .. })
    }

    /// Per-decl batches steepen to `0.45` (matches the C walker) —
    /// short decls plus dozens per file would otherwise dominate the
    /// rank against larger anchors at the default 0.35.
    fn concavity_exponent(&self) -> f64 {
        match self {
            GoKey::Decl { .. } | GoKey::DeclBody { .. } | GoKey::StructFieldGroup { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            GoKey::PackageDocLede { file } => describe_in("go package doc lede", file, root),
            GoKey::PackageDocBody { file } => describe_in("go package doc body", file, root),
            GoKey::PackageImports { file } => describe_in("go package + imports", file, root),
            GoKey::DeclNames { file, chunk_index } => {
                describe_chunked_surface("go decl names surface", file, *chunk_index, root)
            }
            GoKey::Decl { file, start_line } => describe_at("go decl", file, *start_line, root),
            GoKey::DeclBody { file, start_line } => {
                describe_at("go decl body", file, *start_line, root)
            }
            GoKey::DeclDoc { file, start_line } => {
                describe_at("go decl doc", file, *start_line, root)
            }
            GoKey::StructFieldGroup {
                file,
                start_line,
                group_start_line,
            } => format!(
                "go struct field group at {}:{start_line} group {group_start_line}",
                display_path(file, root)
            ),
            GoKey::TestNames { file } => describe_in("go test names surface", file, root),
            GoKey::GoModIdentity { file } => describe_in("go module identity", file, root),
            GoKey::GoMod { file } => format!("go module file {}", display_path(file, root)),
        }
    }
}

impl InnerKey for PythonKey {
    fn is_depth_follow_up(&self) -> bool {
        matches!(
            self,
            PythonKey::DeclDocRest { .. }
                | PythonKey::DeclBody { .. }
                | PythonKey::MethodBody { .. }
        )
    }

    /// `DeclNames` / `DeclNamesChunk` + `ImportChunk` use a mild `0.37` (broad
    /// catalog-shaped surfaces). Per-decl / per-method batches use
    /// `0.45` (matches C / Go) for the same reason — short decls
    /// emitted in bulk. `ClassBody` keeps the default; field listings
    /// tie structurally to the class.
    fn concavity_exponent(&self) -> f64 {
        match self {
            PythonKey::ImportChunk { .. }
            | PythonKey::DeclNames { .. }
            | PythonKey::DeclNamesChunk { .. } => 0.37,
            PythonKey::Decl { .. }
            | PythonKey::DeclBody { .. }
            | PythonKey::Method { .. }
            | PythonKey::MethodBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            PythonKey::Imports { file } => describe_in("python imports", file, root),
            PythonKey::ImportChunk { file, chunk_index } => {
                describe_chunked_surface("python imports", file, *chunk_index, root)
            }
            PythonKey::DeclNames { file } => describe_in("python decl names surface", file, root),
            PythonKey::DeclNamesChunk { file, chunk_index } => {
                describe_chunked_surface("python decl names surface", file, *chunk_index, root)
            }
            PythonKey::Decl { file, start_line } => {
                describe_at("python decl", file, *start_line, root)
            }
            PythonKey::DeclDoc { file, start_line } => {
                describe_at("python decl doc", file, *start_line, root)
            }
            PythonKey::DeclDocRest { file, start_line } => {
                describe_at("python decl doc rest", file, *start_line, root)
            }
            PythonKey::DeclBody {
                file,
                start_line,
                body_start_line,
            } => describe_at_body(
                "python decl body",
                file,
                *start_line,
                *body_start_line,
                root,
            ),
            PythonKey::ClassBody { file, start_line } => {
                describe_at("python class body", file, *start_line, root)
            }
            PythonKey::MethodSigs { file } => describe_in("python method sigs", file, root),
            PythonKey::Method { file, start_line } => {
                describe_at("python method", file, *start_line, root)
            }
            PythonKey::MethodDoc { file, start_line } => {
                describe_at("python method doc", file, *start_line, root)
            }
            PythonKey::MethodBody {
                file,
                start_line,
                body_start_line,
            } => describe_at_body(
                "python method body",
                file,
                *start_line,
                *body_start_line,
                root,
            ),
            PythonKey::SetupManifest { file, start_line } => {
                describe_at("python setup manifest", file, *start_line, root)
            }
            PythonKey::TestNames { file } => describe_in("python test names surface", file, root),
        }
    }
}

impl InnerKey for CKey {
    fn is_depth_follow_up(&self) -> bool {
        matches!(self, CKey::DeclDoc { .. } | CKey::DeclBody { .. })
    }

    /// Per-decl batches steepen to `0.45` — typedef / prototype lines
    /// are short and headers emit dozens; the default 0.35 lets the
    /// stack dominate larger anchor batches. `DeclDoc` joins them: a
    /// one-line doc comment above a decl is the same short-and-numerous
    /// shape, and at 0.35 the doc-scrap stack out-ranks the big coherent
    /// name catalogs / struct batches the NS wants first (krep, bareiron).
    fn concavity_exponent(&self) -> f64 {
        match self {
            CKey::Decl { .. }
            | CKey::DeclBody { .. }
            | CKey::AggregateMemberGroup { .. }
            | CKey::DeclDoc { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            CKey::WholeFile { file } => describe_in("c whole header", file, root),
            CKey::HeaderBanner { file } => describe_in("c header banner", file, root),
            CKey::Includes { file } => describe_in("c includes", file, root),
            CKey::DeclNames { file, chunk_index } => {
                describe_chunked_surface("c decl names surface", file, *chunk_index, root)
            }
            CKey::Decl { file, start_line } => describe_at("c decl", file, *start_line, root),
            CKey::DeclBody { file, start_line } => {
                describe_at("c decl body", file, *start_line, root)
            }
            CKey::DeclDoc { file, start_line } => {
                describe_at("c decl doc", file, *start_line, root)
            }
            CKey::AggregateMemberGroup {
                file,
                start_line,
                group_start_line,
            } => format!(
                "c aggregate member group at {}:{start_line} group {group_start_line}",
                display_path(file, root)
            ),
        }
    }
}

impl InnerKey for LuaKey {
    /// Per-decl batches steepen to `0.45` (matches C / Python).
    /// `MetaFileWhole` keeps the default — LuaCATS specs are
    /// load-bearing and shouldn't be pushed later.
    fn concavity_exponent(&self) -> f64 {
        match self {
            LuaKey::Decl { .. } | LuaKey::DeclBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    /// The module identity table is orientation ("what is this library").
    fn is_orientation(&self) -> bool {
        matches!(self, LuaKey::ModuleIdentity { .. })
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            LuaKey::Banner { file } => describe_in("lua banner", file, root),
            LuaKey::ModuleIdentity { file } => describe_in("lua module identity", file, root),
            LuaKey::MetaFileWhole { file } => {
                format!("lua meta-file at {}", display_path(file, root))
            }
            LuaKey::DeclNames { file } => describe_in("lua decl names surface", file, root),
            LuaKey::Decl { file, start_line } => describe_at("lua decl", file, *start_line, root),
            LuaKey::DeclDoc { file, start_line } => {
                describe_at("lua decl doc", file, *start_line, root)
            }
            LuaKey::DeclBody { file, start_line } => {
                describe_at("lua decl body", file, *start_line, root)
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

/// `"<label> in <path>"`.
fn describe_in(label: &str, file: &Path, root: &Path) -> String {
    format!("{label} in {}", display_path(file, root))
}

/// `"<label> at <path>:<line>"`.
fn describe_at(label: &str, file: &Path, line: usize, root: &Path) -> String {
    format!("{label} at {}:{line}", display_path(file, root))
}

/// `"<label> at <path>:<line> body <body>"`.
fn describe_at_body(label: &str, file: &Path, line: usize, body: usize, root: &Path) -> String {
    format!("{label} at {}:{line} body {body}", display_path(file, root))
}

fn describe_chunked_surface(label: &str, file: &Path, chunk_index: usize, root: &Path) -> String {
    if chunk_index == 0 {
        describe_in(label, file, root)
    } else {
        format!("{label} #{chunk_index} in {}", display_path(file, root))
    }
}

/// A walker-emitted scheduling unit. Carries the walker's key (so other
/// candidates can name it as predecessor), the rendered content, and the
/// scalar value used for ranking. The scheduler stamps a [`BatchId`] when
/// the batch enters the pool but the walker never sees it.
///
/// Walkers compute `value` directly: it's the scalar input to
/// [`crate::value::ratio_with_exponent`], on a shared cross-walker
/// scale (calibration across walkers is a divergence-reports problem,
/// not a code-level invariant — see `docs/design-notes.md`).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_deferred_mass_prose_is_not_orientation() {
        let ordinary = MarkdownKey::Section {
            file: PathBuf::from("README.md"),
            section_index: 1,
            reference_shaped: false,
            deferred_mass_prose: false,
        };
        let deferred = MarkdownKey::Section {
            file: PathBuf::from("README.md"),
            section_index: 2,
            reference_shaped: false,
            deferred_mass_prose: true,
        };

        assert!(ordinary.is_orientation());
        assert!(!deferred.is_orientation());
        assert!(deferred.is_deferred_mass_prose());
    }
}
