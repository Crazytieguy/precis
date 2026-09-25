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
    Markdown(MarkdownKey),
    Toml(TomlKey),
    Json(JsonKey),
    Plaintext(PlaintextKey),
    Prisma(PrismaKey),
    GoMod(GoModKey),
    Code(CodeKey),
    Yaml(YamlKey),
    Sql(SqlKey),
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
            fn is_depth_follow_up(&self) -> bool {
                match self { $(BatchKey::$variant(k) => InnerKey::is_depth_follow_up(k),)* }
            }
            fn is_dominant_file_surface(&self) -> bool {
                match self { $(BatchKey::$variant(k) => InnerKey::is_dominant_file_surface(k),)* }
            }
        }
    };
}

impl_batchkey! {
    Fs => FsKey,
    Markdown => MarkdownKey,
    Toml => TomlKey,
    Json => JsonKey,
    Plaintext => PlaintextKey,
    Prisma => PrismaKey,
    GoMod => GoModKey,
    Code => CodeKey,
    Yaml => YamlKey,
    Sql => SqlKey,
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
    /// True for depth follow-up batches — doc/body/member refinements
    /// of an already-delivered surface. Drives the scheduler's
    /// breadth-pressure penalty; surfaces and orientation never
    /// qualify.
    fn is_depth_follow_up(&self) -> bool {
        false
    }
    /// See [`WalkerKey::is_dominant_file_surface`].
    fn is_dominant_file_surface(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownKey {
    /// Whole `SUMMARY.md` (mdBook ToC).
    SummaryWhole { file: PathBuf },
    /// README headline: first heading + first paragraph.
    ReadmeHeadline { file: PathBuf },
    /// The rest of a README's pre-heading prelude — hero/logo block,
    /// badges, and every lede block past the one the headline took.
    /// Predecessor: `ReadmeHeadline`.
    Prelude { file: PathBuf },
    /// Every H1/H2/H3 heading line (H2+H3 only on READMEs). Predecessor
    /// of every same-file `Section`.
    HeadingsOutline { file: PathBuf },
    /// One scheduling unit of a markdown body, indexed by post-split
    /// position. May be whole H2 / per-bullet / Intro+per-H3 child.
    /// `keeps_default_concavity` exempts a range from the steeper
    /// index-≥1 prose exponent: README ranges dominated by list /
    /// table / fence rows (catalogs, not prose), and the body behind a
    /// carved section lede (the body is the same content the unsplit
    /// section priced at index 0 — the lede taking slot 0 must not
    /// re-price it).
    Section {
        file: PathBuf,
        section_index: usize,
        keeps_default_concavity: bool,
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
    /// instead, with the rest under [`PlaintextKey::DotenvChunk`] or
    /// [`PlaintextKey::Tail`].
    Whole { file: PathBuf },
    /// Dockerfile build-mechanics rows complementary to the contract
    /// lines carried by a head-shaped `Whole` batch.
    Tail { file: PathBuf },
    /// Source-ordered chunk of a long dotenv sample's optional-settings
    /// tail. Chunks form a predecessor chain after [`PlaintextKey::Whole`]
    /// so later config groups cannot render before earlier ones.
    DotenvChunk { file: PathBuf, chunk_index: usize },
    /// `NAME` + `DESCRIPTION`-lede slice of a troff man page — the
    /// "what is this tool" answer for a CLI shipping a `*.1` / `*.5`
    /// (or autotools `*.1.in`) manual.
    ManLede { file: PathBuf },
    /// Indentation-zero declaration surface of a source-like text file
    /// no format-aware walker claims (Java, C++, Ruby, PHP, Swift,
    /// Vue, CSS, reST, …). The language-agnostic fallback.
    DeclSurface { file: PathBuf },
}

/// Root-contract SQL batches. Migration forests and other incidental SQL
/// stay unreachable: the SQL walker admits only root files or exact paths
/// cited from a root README/build file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SqlKey {
    /// Source-ordered, semicolon-aligned DDL statements.
    SchemaChunk { file: PathBuf, chunk_index: usize },
    /// A bounded set of representative `SELECT ... WHERE` contracts.
    QueryChunk { file: PathBuf, chunk_index: usize },
}

/// YAML batches. Narrowly scoped to operational configs and root
/// reference/spec maps whose top-level keys are useful orientation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum YamlKey {
    /// Whole-file render of a compact operational config.
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

/// `go.mod` / `go.work` batches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GoModKey {
    /// The `module`, `go` and `toolchain` directives. Predecessor of
    /// `File`.
    Identity { file: PathBuf },
    /// The module file, whole when small, otherwise without its indirect
    /// requires. Predecessor: matching `Identity`.
    File { file: PathBuf },
}

/// Batches of the shared code engine (`walker::code`): one key shape for
/// every language it has ported. Identity is `(rung, file, decl, sub)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodeKey {
    pub rung: Rung,
    pub file: PathBuf,
    /// Per-file declaration index in the engine's normalized order
    /// (top-level declarations by first row, each container directly
    /// followed by its members). 0 for the file-level rungs.
    pub decl: u32,
    /// Chunk index within the part; 0 is the head chunk.
    pub sub: u32,
    /// The declaration's first source row (1-based), shown by
    /// [`WalkerKey::describe`]. `decl` already determines it, so it never
    /// decides identity or order; a container and its first member can
    /// share it. 0 for the file-level rungs.
    pub line: usize,
}

/// The rungs of the per-file declaration ladder, in emission order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rung {
    /// File-level documentation: crate/package/module doc or docstring.
    ModuleDoc,
    /// The file's roster: every top-level declaration's name rows and
    /// every re-export, in source order.
    Names,
    /// One declaration's head, plus its body when the declaration is
    /// `Whole` (a container's body is its member roster).
    Decl,
    /// One declaration's doc comment or docstring.
    Doc,
    /// One `Callable` declaration's body statements.
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TomlKey {
    /// `[package]` or `[workspace.package]` identity block.
    Identity { file: PathBuf },
    /// The rest of a Python identity table once the lede is taken: author and
    /// maintainer rosters, project URLs, keywords, trove classifiers and
    /// packaging globs. Predecessor: `Identity` on the same file.
    PackageMetadata { file: PathBuf },
    /// Entry-point console scripts: `[project.scripts]` (PEP 621) or
    /// `[tool.poetry.scripts]` — the "how do I run this" surface.
    Scripts { file: PathBuf },
    /// `[features]` table.
    Features { file: PathBuf },
    /// Ordinary `[dependencies]` / `[workspace.dependencies]` tables,
    /// plus Python-manifest dependency sections.
    Dependencies { file: PathBuf },
    /// Cargo `[dev-dependencies]`, `[build-dependencies]`, and target-
    /// conditional dependency tables. Predecessor: `Dependencies` on the
    /// same file, when that manifest declares a runtime roster.
    DevelopmentDependencies { file: PathBuf },
    /// One top-level Python-manifest `tool.<name>` family, including its
    /// descendants, or a compact family of adjacent small tool tables.
    /// Predecessor: `Identity` on the same file, when it has one.
    ToolConfig { file: PathBuf, tool: String },
    /// Manifest-level operational config outside Python `tool.*` families:
    /// build systems, package metadata, Cargo targets, and profiles.
    /// Predecessor: `Identity` on the same file, when it has one.
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

    /// True for orientation content (README/manifest surfaces). A train
    /// rooted here counts as a scheduler redirect target by its
    /// *unbought* members rather than by being unentered — orientation
    /// chains are read a section at a time, so entering one does not
    /// spend the breadth it still holds.
    fn is_orientation(&self) -> bool {
        false
    }

    /// True for depth follow-up batches — doc/body/member refinements
    /// of an already-delivered surface. Drives the scheduler's
    /// breadth-pressure penalty.
    fn is_depth_follow_up(&self) -> bool {
        false
    }

    /// True for the batch classes that earn the scheduler's
    /// dominant-source-file premium: declaration/name rosters and
    /// catalogs, public-surface item heads, and the imports-level
    /// top-of-file surface. Opt-in per walker, defaulting false —
    /// bodies, tails, doc prose, and member/field groups are the depth
    /// the premium is meant to *reach*, not the depth it front-loads.
    fn is_dominant_file_surface(&self) -> bool {
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

impl InnerKey for MarkdownKey {
    fn is_orientation(&self) -> bool {
        true
    }
    fn describe(&self, root: &Path) -> String {
        match self {
            MarkdownKey::SummaryWhole { file } => {
                format!("mdBook SUMMARY at {}", display_path(file, root))
            }
            MarkdownKey::ReadmeHeadline { file } => describe_in("README headline", file, root),
            MarkdownKey::Prelude { file } => describe_in("README prelude", file, root),
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
    /// See `keeps_default_concavity` for the exemptions.
    fn concavity_exponent(&self) -> f64 {
        match self {
            MarkdownKey::Section {
                section_index,
                keeps_default_concavity,
                ..
            } if *section_index >= 1 && !keeps_default_concavity => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }
}

impl InnerKey for TomlKey {
    fn is_orientation(&self) -> bool {
        !matches!(
            self,
            TomlKey::ToolConfig { .. } | TomlKey::Config { .. } | TomlKey::PackageMetadata { .. }
        )
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            TomlKey::Identity { file } => describe_in("[package]", file, root),
            TomlKey::PackageMetadata { file } => describe_in("package metadata", file, root),
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

    /// The language-agnostic declaration surface is how an unparsed
    /// source language (Ruby, Swift, C++, …) presents its roster.
    fn is_dominant_file_surface(&self) -> bool {
        matches!(self, PlaintextKey::DeclSurface { .. })
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            PlaintextKey::Whole { file } => {
                format!("plaintext config {}", display_path(file, root))
            }
            PlaintextKey::Tail { file } => {
                format!("plaintext config tail of {}", display_path(file, root))
            }
            PlaintextKey::DotenvChunk { file, chunk_index } => {
                format!(
                    "plaintext dotenv tail chunk #{} of {}",
                    chunk_index + 1,
                    display_path(file, root),
                )
            }
            PlaintextKey::ManLede { file } => {
                format!(
                    "man-page NAME + DESCRIPTION in {}",
                    display_path(file, root)
                )
            }
            PlaintextKey::DeclSurface { file } => {
                format!("declaration surface of {}", display_path(file, root))
            }
        }
    }
}

impl InnerKey for SqlKey {
    fn is_orientation(&self) -> bool {
        true
    }

    fn describe(&self, root: &Path) -> String {
        match self {
            SqlKey::SchemaChunk { file, chunk_index } => format!(
                "SQL schema contracts chunk #{} in {}",
                chunk_index + 1,
                display_path(file, root),
            ),
            SqlKey::QueryChunk { file, chunk_index } => format!(
                "SQL SELECT/WHERE contracts chunk #{} in {}",
                chunk_index + 1,
                display_path(file, root),
            ),
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

impl InnerKey for GoModKey {
    fn describe(&self, root: &Path) -> String {
        match self {
            GoModKey::Identity { file } => describe_in("go module identity", file, root),
            GoModKey::File { file } => format!("go module file {}", display_path(file, root)),
        }
    }
}

impl InnerKey for CodeKey {
    fn is_depth_follow_up(&self) -> bool {
        matches!(self.rung, Rung::Doc | Rung::Body)
    }

    fn is_dominant_file_surface(&self) -> bool {
        matches!(self.rung, Rung::Names | Rung::Decl)
    }

    /// `"<lang> <rung> <path>[:<line>][ #<sub>]"`, e.g. `go decl pkg/a.go:42`
    /// or `rust names src/lib.rs #1`.
    fn describe(&self, root: &Path) -> String {
        let language = crate::walker::code::Language::from_path(&self.file)
            .map_or("code", |language| language.label());
        let rung = match self.rung {
            Rung::ModuleDoc => "module doc",
            Rung::Names => "names",
            Rung::Decl => "decl",
            Rung::Doc => "doc",
            Rung::Body => "body",
        };
        let mut out = format!("{language} {rung} {}", display_path(&self.file, root));
        if matches!(self.rung, Rung::Decl | Rung::Doc | Rung::Body) {
            out.push_str(&format!(":{}", self.line));
        }
        if self.sub > 0 {
            out.push_str(&format!(" #{}", self.sub));
        }
        out
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

/// A walker-emitted scheduling unit. Carries the walker's key (so other
/// candidates can name it as predecessor), the rendered content, and the
/// scalar value used for ranking. The scheduler stamps a [`BatchId`] when
/// the batch enters the pool but the walker never sees it.
///
/// Walkers compute `value` directly: it's the scalar input to
/// [`crate::value::ratio_with_exponent`], on a shared cross-walker
/// scale (calibration across walkers is a divergence-reports problem,
/// not a code-level invariant).
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
