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

/// Walker-key sum: one variant per walker, each wrapping that walker's
/// own key.
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FsKey {
    /// Listing of immediate children of `dir`: all of them, or a long
    /// listing's head.
    DirListing { dir: PathBuf },
    /// The rest of a long listing. Predecessor: its `DirListing`.
    DirListingTail { dir: PathBuf },
}

/// `From<XKey> for BatchKey` for every per-walker key.
macro_rules! impl_from_walker_keys {
    ($($variant:ident => $key:ident),* $(,)?) => {
        $(
            impl From<$key> for BatchKey {
                fn from(k: $key) -> Self { BatchKey::$variant(k) }
            }
        )*
    };
}

impl_from_walker_keys! {
    Fs => FsKey,
    Markdown => MarkdownKey,
    Toml => TomlKey,
    Json => JsonKey,
    Plaintext => PlaintextKey,
    Prisma => PrismaKey,
    GoMod => GoModKey,
    Code => CodeKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownKey {
    /// README headline: first heading + first paragraph.
    ReadmeHeadline { file: PathBuf },
    /// The rest of a README's pre-heading prelude: every lede block past
    /// the one the headline took, chrome left out. Predecessor:
    /// `ReadmeHeadline`.
    Prelude { file: PathBuf },
    /// Every H1/H2/H3 heading line the headline doesn't cover.
    /// Predecessor of every same-file `Section`.
    HeadingsOutline { file: PathBuf },
    /// One scheduling unit of a markdown body, indexed by post-split
    /// position: a whole top-level section, an H1 intro, a headingless
    /// body, or an oversize chunk. `keeps_default_concavity` exempts a
    /// README reference section (usage demo, options, API) from the
    /// steeper index-≥1 prose exponent.
    Section {
        file: PathBuf,
        section_index: usize,
        keeps_default_concavity: bool,
    },
    /// A build/test/run section's heading plus its leading shell blocks,
    /// keyed by the block's first row. Predecessor: `HeadingsOutline`,
    /// else `ReadmeHeadline`; the `Section` holding it gates on it.
    CommandBlock { file: PathBuf, row: usize },
}

/// `package.json` batches, split along the shared manifest ontology
/// (identity / operational / dependencies).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JsonKey {
    /// `package.json` identity scalars: `name`, `version`, `description`,
    /// `type`, `private`, `license`/`licenses`.
    Identity { file: PathBuf },
    /// `package.json` entrypoint pointers (`main`/`module`/`exports`/
    /// `bin`/`types`/`files`/...).
    Entry { file: PathBuf },
    /// `package.json` runtime/toolchain constraints (`engines`,
    /// `packageManager`).
    Runtime { file: PathBuf },
    /// `package.json` `scripts` block, or its first chunk when it is long.
    Scripts { file: PathBuf },
    /// A later chunk of a long `scripts` block, in source order.
    ScriptsTail { file: PathBuf, chunk: usize },
    /// Runtime `package.json` dependency blocks (`dependencies`, optional /
    /// bundled dependencies, overrides, and resolutions).
    Dependencies { file: PathBuf },
}

/// Batches for files no parser claims — named config/ops files and the
/// language-agnostic declaration surface. See `walker::plaintext`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlaintextKey {
    /// Whole-file render, or a head slice of a longer file. For a
    /// fallback file, the whole of a short one; predecessor: its
    /// `DeclSurface`.
    Whole { file: PathBuf },
    /// Indentation-zero declaration surface of a source-like text file
    /// no format-aware walker claims (Java, C++, Ruby, PHP, Swift,
    /// Vue, CSS, reST, …). The language-agnostic fallback.
    DeclSurface { file: PathBuf },
    /// Once every other batch is scheduled, the head of a file no other
    /// batch touches, or in a single-file walk the named file's rows no
    /// other batch shows.
    Rest { file: PathBuf },
}

/// Prisma schema batches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrismaKey {
    /// One line per top-level `model` / `enum` / `datasource` /
    /// `generator` declaration in a `schema.prisma`.
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
    /// [`BatchKey::describe`]. `decl` already determines it, so it never
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
    /// How the package runs: `[features]`, and a Python manifest's console
    /// scripts (`[project.scripts]`, `[tool.poetry.scripts]`).
    Operational { file: PathBuf },
    /// Ordinary `[dependencies]` / `[workspace.dependencies]` tables,
    /// plus Python-manifest dependency sections.
    Dependencies { file: PathBuf },
}

impl BatchKey {
    /// One-line descriptor for divergence reports: the key's `Debug` form
    /// with `fixture_root` stripped from its paths, e.g.
    /// `Markdown::Section { file: README.md, section_index: 2, .. }`.
    pub fn describe(&self, fixture_root: &Path) -> String {
        let root = format!("{fixture_root:?}");
        let root = root.trim_matches('"');
        let debug = format!("{self:?}")
            .replace(&format!("{root}/"), "")
            .replace(root, ".")
            .replace('"', "");
        match debug.split_once('(') {
            Some((walker, inner)) => {
                format!("{walker}::{}", inner.strip_suffix(')').unwrap_or(inner))
            }
            None => debug,
        }
    }

    /// Cost concavity for the scheduling ratio (`value / cost^exponent`).
    /// `0.45` for prose-shaped batches whose cost grows without
    /// proportional structural value: markdown sections past the first
    /// (index 0 is where READMEs lead with their canonical claim; see
    /// `keeps_default_concavity` for the exemptions).
    pub fn concavity_exponent(&self) -> f64 {
        match self {
            BatchKey::Markdown(MarkdownKey::Section {
                section_index,
                keeps_default_concavity,
                ..
            }) if *section_index >= 1 && !keeps_default_concavity => 0.45,
            _ => crate::value::DEFAULT_CONCAVITY_EXPONENT,
        }
    }
}

/// A walker-emitted scheduling unit. Carries the walker's key (so other
/// candidates can name it as predecessor), the rendered content, and the
/// scalar value used for ranking. The scheduler stamps a [`BatchId`] when
/// the batch enters the pool but the walker never sees it.
///
/// Walkers compute `value` directly: it's the scalar input to
/// `value::ratio_with_exponent`, on a shared cross-walker
/// scale (calibration across walkers is a divergence-reports problem,
/// not a code-level invariant).
#[derive(Debug, Clone)]
pub struct Batch {
    pub key: BatchKey,
    /// Optional predecessor edge. The batch stays pending until its
    /// predecessor is scheduled; line overlap with earlier batches is only
    /// permitted along this chain (see [`crate::content`] / `render`).
    pub predecessor: Option<BatchKey>,
    pub content: BatchContent,
    pub value: f64,
}
