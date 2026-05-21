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

macro_rules! impl_batchkey_from {
    ($($variant:ident => $key:ident),* $(,)?) => {
        $(
            impl From<$key> for BatchKey {
                fn from(k: $key) -> Self { BatchKey::$variant(k) }
            }
        )*
    };
}

impl_batchkey_from! {
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

/// Rust batches. Per-item for pub type declarations so the scheduler can
/// individually rank them. File-scope for crate-doc / mod-use / impl-method
/// groups; cross-file scope for the `#[macro_export]` name surface.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RustKey {
    /// `//!` module-doc lede — first paragraph only, entrypoint files.
    CrateDocLede { file: PathBuf },
    /// `//!` module-doc body — after the first paragraph.
    /// Predecessor: `CrateDocLede`.
    CrateDocBody { file: PathBuf },
    /// `use` + `mod` + `pub use` plumbing at the top of a file.
    ModUse { file: PathBuf },
    /// Surface listing of every top-level `pub` item name in a file —
    /// catastrophic-omission hedge.
    PubItemNames { file: PathBuf },
    /// One top-level `pub` item's declaration. Whole item for
    /// struct/enum/trait/type/const/static; signature with body marker
    /// for fn. Keyed by start line.
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
    /// First paragraph of the rustdoc above a `pub` item (up to first
    /// `# Heading`, or whole doc when headless). Predecessor: matching
    /// `PubItem`.
    PubItemDocLede { file: PathBuf, start_line: usize },
    /// Rustdoc body from the first `# Heading` onward. Predecessor:
    /// matching `PubItemDocLede` if any, else `PubItem` (avoids
    /// dead-keying when Lede would be empty).
    PubItemDocBody { file: PathBuf, start_line: usize },
    /// Impl-block headers + method signatures in a single file.
    MethodSigs { file: PathBuf },
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
    /// Every H1/H2/H3 heading line (H2+H3 only for READMEs, where the
    /// H1 stays under `ReadmeHeadline`). Predecessor of every same-file
    /// `Section` when emitted, so heading-row overlap is permitted as
    /// ancestor overlap.
    HeadingsOutline { file: PathBuf },
    /// One scheduling unit of a markdown body, indexed by 0-based
    /// position in the walker's logical-section list. Granularity is
    /// variable — H2s may be whole, bullet-split (one batch per top-level
    /// item), or H3-split (one `Intro` plus one batch per H3 child); the
    /// split classification lives in the walker, so `section_index` is
    /// post-split. For `README.md`, predecessor is `ReadmeHeadline`
    /// (or `HeadingsOutline` when it's emitted for the file).
    Section { file: PathBuf, section_index: usize },
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
    /// Surface listing of every top-level export's first line —
    /// catastrophic-omission hedge.
    ExportNames {
        file: PathBuf,
        chunk_index: usize,
        export_count: usize,
        type_only_export_count: usize,
    },
    /// One top-level export's declaration. Whole item for
    /// interface/type/class/enum; signature with body marker for fn;
    /// assignment line for const/let. Keyed by start line.
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
    /// Body slice of an export with a `statement_block` body (fn,
    /// generator, class methods, `export default <fn|class>`). Outer
    /// braces stripped. Predecessor: matching `Export`. Sibling of
    /// `ExportDoc` under `Export`; the two cover disjoint lines.
    ExportBody {
        file: PathBuf,
        /// Parent export line — keeps body slices tied to their
        /// predecessor when two bodies share a start line in different
        /// declarations.
        start_line: usize,
        /// First emitted line of this body slice; disambiguates siblings.
        body_start_line: usize,
    },
    /// Top-level non-exported declaration surface — module-private
    /// classes, helper fns, type aliases, constants that exported APIs
    /// depend on.
    ModuleItem { file: PathBuf, start_line: usize },
    /// Body slice of a non-exported top-level decl, split by top-level
    /// statement. Predecessor: matching `ModuleItem`.
    ModuleItemBody {
        file: PathBuf,
        /// Parent module item line.
        start_line: usize,
        /// First emitted line of this body slice; disambiguates siblings.
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
    /// `package.json` entrypoint pointers: `main`, `module`, `browser`,
    /// `exports`, `types`/`typings`, `source`, `bin`, `unpkg`, `umd:main`,
    /// `jsnext:main`, `react-native`, `files`.
    Entry { file: PathBuf },
    /// `package.json` runtime/toolchain constraints: `engines`,
    /// `engineStrict`, `packageManager`.
    Runtime { file: PathBuf },
    /// `package.json` `scripts` block.
    Scripts { file: PathBuf },
    /// `package.json` dependency blocks (`dependencies`,
    /// `devDependencies`, `peerDependencies`, `optionalDependencies`,
    /// `overrides`, `resolutions`).
    Dependencies { file: PathBuf },
    /// Whole-file render of a small JSON config. Skipped for
    /// `package.json` (use the split batches) and for large/generated files.
    Whole { file: PathBuf },
}

/// Plaintext config / license file batches. Whitelist lives in
/// [`crate::walker::plaintext`]. Capped on line + token cost to bound
/// render-time displacement of richer walker batches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlaintextKey {
    /// Whole-file render. Skipped when line count or rendered token
    /// cost exceeds the walker's caps.
    Whole { file: PathBuf },
}

/// YAML batches. Narrowly scoped to `docker-compose.{yml,yaml}` —
/// other YAML configs are out of scope; the agent can `Read` them
/// after seeing the dir listing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum YamlKey {
    /// Whole-file render of a `docker-compose.{yml,yaml}` file. Skipped
    /// when the source exceeds the walker's line cap.
    Whole { file: PathBuf },
}

/// Prisma schema batches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrismaKey {
    /// One line per top-level `model` / `enum` / `datasource` /
    /// `generator` declaration in a `schema.prisma` — catastrophic-
    /// omission hedge.
    Toc { file: PathBuf },
}

/// C / C-header batches. "Public" rule: top-level
/// `function_definition` / `declaration` / `type_definition` /
/// `preproc_def` / `preproc_function_def` that aren't `static` (in `.c`
/// files), plus `static inline` fn defs in `.h` files. Header-guard
/// `#ifndef`/`#define`/`#endif` is descended transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CKey {
    /// Top-of-file `/* */` banner comment (license / brief).
    HeaderBanner { file: PathBuf },
    /// `#include` directives.
    Includes { file: PathBuf },
    /// Surface listing of every top-level public declaration's first
    /// line — catastrophic-omission hedge. Chunked in
    /// `NAMES_SURFACE_CHUNK_SIZE` groups so a large surface doesn't
    /// lose the value/cost race against per-decl batches.
    DeclNames { file: PathBuf, chunk_index: usize },
    /// One top-level public declaration. Whole statement for typedefs /
    /// prototypes / `extern` / `#define`; whole specifier for
    /// struct/enum/union; signature with body marker for fn defs.
    /// Keyed by start line.
    Decl { file: PathBuf, start_line: usize },
    /// Body interior of a fn definition. Predecessor: matching `Decl`.
    DeclBody { file: PathBuf, start_line: usize },
    /// Doc comment(s) immediately above a declaration. Predecessor:
    /// matching `Decl`.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Blank-line-separated field group inside a big struct/union body,
    /// or a sized chunk of enumerators inside a big enum body. Emitted
    /// only when a struct/union has ≥3 blank-line groups, or an enum
    /// has ≥`AGGREGATE_ENUM_CHUNK_MIN` enumerators. Predecessor:
    /// matching `Decl` — line overlap with the Decl is allowed as
    /// ancestor overlap; the Decl's span is reduced to the type header
    /// + closer.
    AggregateMemberGroup {
        file: PathBuf,
        start_line: usize,
        group_start_line: usize,
    },
}

/// Go batches. Emits **all** top-level decls regardless of export
/// status — NS authors anchor on intentionally-unexported types; a
/// `visibility_factor` discount ranks exported names higher rather than
/// hard-filtering. Grouped decls (`type ( … )`, `var ( … )`, `const ( … )`)
/// stay as one batch so iota / inherited-type / shared-comment semantics
/// survive. `*_test.go` files emit only `TestNames` plus a non-essential
/// discount on per-decl bodies.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GoKey {
    /// `// Package foo …` doc comment immediately above `package`.
    /// Emitted separately from `PackageImports` so a long package
    /// comment can fire without dragging the import block.
    PackageDocLede { file: PathBuf },
    /// Package clause + import block at the top of a `.go` file.
    PackageImports { file: PathBuf },
    /// Surface listing of every top-level decl's first line.
    /// Catastrophic-omission hedge. Visibility-blind. Chunked at
    /// `GO_DECL_NAMES_CHUNK_SIZE` only above
    /// `GO_DECL_NAMES_CHUNK_THRESHOLD` decls.
    DeclNames { file: PathBuf, chunk_index: usize },
    /// One top-level declaration. Signature with body marker for
    /// fn/method defs; whole decl (including grouped specs) for
    /// type/var/const. Keyed by start line.
    Decl { file: PathBuf, start_line: usize },
    /// Body interior of a fn or method definition. Predecessor:
    /// matching `Decl`.
    DeclBody { file: PathBuf, start_line: usize },
    /// Run of `//` (or `/* */`) comments above a decl with no
    /// blank-line gap. Predecessor: matching `Decl`.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Blank-line-separated field-group within a big
    /// `type X struct { … }`. Emitted only for type-decls whose struct
    /// body has ≥3 blank-line groups and ≥60 body lines. Predecessor:
    /// matching `Decl` — line overlap allowed as ancestor overlap; the
    /// Decl's span is reduced to the struct header + closer.
    StructFieldGroup {
        file: PathBuf,
        start_line: usize,
        group_start_line: usize,
    },
    /// Surface listing of every `Test*` / `Benchmark*` / `Example*`
    /// fn's first line in a `_test.go` file.
    TestNames { file: PathBuf },
    /// Identity slice of a `go.mod` / `go.work`: `module` path, `go`
    /// version floor, optional `toolchain` lines. Predecessor of
    /// `GoMod` so the cheap identity slice can land without the whole
    /// require block.
    GoModIdentity { file: PathBuf },
    /// Whole-file render of a `go.mod` / `go.work`. Line-capped.
    /// Predecessor: matching `GoModIdentity`.
    GoMod { file: PathBuf },
}

/// Python batches. Per-decl items at the top level and per-method
/// inside top-level classes (methods are first-class scheduling units).
///
/// Decorated defs / classes use the `decorated_definition` wrapper as
/// the unit, so `start_line` is the `@decorator` row and the span
/// includes decorator lines.
///
/// Visibility: emits all top-level + class-body items; a
/// `visibility_factor` ranks public-by-PEP-8 names above leading-`_`
/// rather than hard-filtering (NSes anchor on intentionally-private
/// names). `test_*.py` / `*_test.py` files emit only `TestNames` plus a
/// non-essential discount on per-decl content.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PythonKey {
    /// `import` / `from … import …` statements, optional module
    /// docstring, `__all__`, and module-level dunder assignments.
    Imports { file: PathBuf },
    /// Chunked `Imports` for large `__init__.py` re-export walls.
    ImportChunk { file: PathBuf, chunk_index: usize },
    /// Surface listing of every top-level class / def (sync or async) /
    /// non-dunder simple-assignment first line.
    DeclNames { file: PathBuf, chunk_index: usize },
    /// One top-level item. For class/def: header (+ decorators if any) +
    /// up to two non-blank rows of the docstring's first paragraph
    /// (PEP 257 summary line) so the per-decl batch carries the usual
    /// NS anchor as one unit. For constant: the assignment line(s).
    /// Keyed by start line (decorator row when decorated).
    Decl { file: PathBuf, start_line: usize },
    /// Docstring of a top-level def or class — the
    /// `expression_statement(string)` at the start of its body, after
    /// any leading comments. Predecessor: matching `Decl`.
    DeclDoc { file: PathBuf, start_line: usize },
    /// Body slice of a top-level def, split by top-level statement;
    /// skips the leading docstring. Predecessor: matching `Decl`.
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
    /// Surface listing of every method def first line across every
    /// top-level class. Decorator-aware. Catastrophic-omission hedge.
    MethodSigs { file: PathBuf, chunk_index: usize },
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
    /// Whole-file rendering for LuaCATS spec files (`---@meta` at top,
    /// or majority-LuaCATS-tag comment density). Gated to small files.
    MetaFileWhole { file: PathBuf },
    /// Surface listing of every top-level fn name + table-method
    /// assignment first line.
    DeclNames { file: PathBuf, chunk_index: usize },
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
    /// `[features]` table.
    Features { file: PathBuf },
    /// `[dependencies]` / `[dev-dependencies]` / `[build-dependencies]` /
    /// `[workspace.dependencies]`.
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
            BatchKey::Prisma(k) => k.describe(fixture_root),
            BatchKey::C(k) => k.describe(fixture_root),
            BatchKey::Go(k) => k.describe(fixture_root),
            BatchKey::Python(k) => k.describe(fixture_root),
            BatchKey::Lua(k) => k.describe(fixture_root),
            BatchKey::Yaml(k) => k.describe(fixture_root),
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
            BatchKey::Lua(k) => k.concavity_exponent(),
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
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
    pub fn describe(&self, root: &Path) -> String {
        let FsKey::DirListing { dir } = self;
        let shown = display_path(dir, root);
        if shown.is_empty() {
            "listing of '.'".to_string()
        } else {
            format!("listing of '{shown}'")
        }
    }
}

impl RustKey {
    /// `PubItemDocBody` steepens to `0.45` — rustdoc prose past the
    /// first heading grows in cost without proportional structural value.
    /// `CrateDocBody` stays at the default (its bullets carry credit).
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            RustKey::PubItemDocBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, root: &Path) -> String {
        match self {
            RustKey::CrateDocLede { file } => describe_in("crate-doc lede", file, root),
            RustKey::CrateDocBody { file } => describe_in("crate-doc body", file, root),
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
            RustKey::MacroNames { src_dir } => {
                format!("macro_export names across {}", display_path(src_dir, root))
            }
            RustKey::MacroBody { file, start_line } => {
                describe_at("macro_export body", file, *start_line, root)
            }
        }
    }
}

impl MarkdownKey {
    pub fn describe(&self, root: &Path) -> String {
        match self {
            MarkdownKey::SummaryWhole { file } => {
                format!("mdBook SUMMARY at {}", display_path(file, root))
            }
            MarkdownKey::ReadmeHeadline { file } => describe_in("README headline", file, root),
            MarkdownKey::HeadingsOutline { file } => describe_in("headings outline", file, root),
            MarkdownKey::Section {
                file,
                section_index,
            } => format!("{} section #{section_index}", display_path(file, root)),
        }
    }

    /// `Section` at index ≥1 steepens to `0.45` to demote prose body
    /// against structural anchors of the same value; index 0 keeps the
    /// default since READMEs often lead with their canonical claim.
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
    /// `ExportNames` / `ImportChunk` for TS/TSX impl files use a mild
    /// `0.38` (flatter than per-decl, steeper than coherent anchors).
    /// Declaration files and JS runtime exports keep the default —
    /// flattening them demotes load-bearing anchors. `ExportMember`
    /// uses `0.45` (per-decl tier).
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
        // Broad, mostly type-only export-name surfaces are real gates.
        // Runtime-heavy catalogs and tiny type files keep standalone rank.
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

    pub fn describe(&self, root: &Path) -> String {
        match self {
            TsKey::ModuleDocLede { file } => describe_in("module-doc lede", file, root),
            TsKey::Imports { file } => describe_in("imports", file, root),
            TsKey::ImportChunk { file, chunk_index } => {
                describe_chunked_surface("imports", file, *chunk_index, root)
            }
            TsKey::ExportNames {
                file, chunk_index, ..
            } => describe_chunked_surface("export names surface", file, *chunk_index, root),
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
            TsKey::ExportBody {
                file,
                start_line,
                body_start_line,
            } => describe_at_body("export body", file, *start_line, *body_start_line, root),
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

impl TomlKey {
    pub fn describe(&self, root: &Path) -> String {
        match self {
            TomlKey::Identity { file } => describe_in("[package]", file, root),
            TomlKey::Features { file } => describe_in("[features]", file, root),
            TomlKey::Dependencies { file } => describe_in("[dependencies]", file, root),
        }
    }
}

impl JsonKey {
    /// `Whole` steepens to `0.45` — verbatim JSON config bodies grow
    /// in cost without proportional structural value. Other variants
    /// (package.json sections) stay at the default; they're short and
    /// structural.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            JsonKey::Whole { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, root: &Path) -> String {
        match self {
            JsonKey::Identity { file } => describe_in("package identity", file, root),
            JsonKey::IdentityMeta { file } => describe_in("package identity metadata", file, root),
            JsonKey::Entry { file } => describe_in("package entrypoints", file, root),
            JsonKey::Runtime { file } => describe_in("package runtime metadata", file, root),
            JsonKey::Scripts { file } => describe_in("package scripts", file, root),
            JsonKey::Dependencies { file } => describe_in("package dependencies", file, root),
            JsonKey::Whole { file } => format!("json config {}", display_path(file, root)),
        }
    }
}

impl PlaintextKey {
    pub fn describe(&self, root: &Path) -> String {
        let PlaintextKey::Whole { file } = self;
        format!("plaintext config {}", display_path(file, root))
    }
}

impl PrismaKey {
    pub fn describe(&self, root: &Path) -> String {
        let PrismaKey::Toc { file } = self;
        describe_in("Prisma schema TOC", file, root)
    }
}

impl YamlKey {
    pub fn describe(&self, root: &Path) -> String {
        let YamlKey::Whole { file } = self;
        format!("docker-compose at {}", display_path(file, root))
    }
}

impl GoKey {
    /// Per-decl batches steepen to `0.45` (matches the C walker) —
    /// short decls plus dozens per file would otherwise dominate the
    /// rank against larger anchors at the default 0.35.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            GoKey::Decl { .. } | GoKey::DeclBody { .. } | GoKey::StructFieldGroup { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, root: &Path) -> String {
        match self {
            GoKey::PackageDocLede { file } => describe_in("go package doc lede", file, root),
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

impl PythonKey {
    /// `DeclNames` + `ImportChunk` use a mild `0.37` (broad
    /// catalog-shaped surfaces). Per-decl / per-method batches use
    /// `0.45` (matches C / Go) for the same reason — short decls
    /// emitted in bulk. `ClassBody` keeps the default; field listings
    /// tie structurally to the class.
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

    pub fn describe(&self, root: &Path) -> String {
        match self {
            PythonKey::Imports { file } => describe_in("python imports", file, root),
            PythonKey::ImportChunk { file, chunk_index } => {
                describe_chunked_surface("python imports", file, *chunk_index, root)
            }
            PythonKey::DeclNames { file, chunk_index } => {
                describe_chunked_surface("python decl names surface", file, *chunk_index, root)
            }
            PythonKey::Decl { file, start_line } => {
                describe_at("python decl", file, *start_line, root)
            }
            PythonKey::DeclDoc { file, start_line } => {
                describe_at("python decl doc", file, *start_line, root)
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
            PythonKey::MethodSigs { file, chunk_index } => {
                describe_chunked_surface("python method sigs", file, *chunk_index, root)
            }
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
            PythonKey::TestNames { file } => describe_in("python test names surface", file, root),
        }
    }
}

impl CKey {
    /// Per-decl batches steepen to `0.45` — typedef / prototype lines
    /// are short and headers emit dozens; the default 0.35 lets the
    /// stack dominate larger anchor batches.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            CKey::Decl { .. } | CKey::DeclBody { .. } | CKey::AggregateMemberGroup { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, root: &Path) -> String {
        match self {
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

impl LuaKey {
    /// Per-decl batches steepen to `0.45` (matches C / Python).
    /// `MetaFileWhole` keeps the default — LuaCATS specs are
    /// load-bearing and shouldn't be pushed later.
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            LuaKey::Decl { .. } | LuaKey::DeclBody { .. } => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }

    pub fn describe(&self, root: &Path) -> String {
        match self {
            LuaKey::Banner { file } => describe_in("lua banner", file, root),
            LuaKey::MetaFileWhole { file } => {
                format!("lua meta-file at {}", display_path(file, root))
            }
            LuaKey::DeclNames { file, chunk_index } => {
                describe_chunked_surface("lua decl names surface", file, *chunk_index, root)
            }
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
