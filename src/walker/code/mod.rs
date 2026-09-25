//! The shared code engine: one declaration ladder for every source
//! language, fed by a thin extraction module per language.
//!
//! Each `code/<lang>.rs` exports exactly this interface, and nothing else
//! reaches into it:
//!
//! - `PORTED: bool`: whether [`expand_in_dir`] handles the language. False
//!   until the language's port commit, which flips it and deletes the old
//!   `walker/<lang>.rs` in the same commit.
//! - `EXTENSIONS: &[&str]`: file extensions (matched case-insensitively)
//!   the language claims. No two languages share one.
//! - `grammar(path) -> tree_sitter::Language`: the grammar that parses
//!   `path`.
//! - `extract(file, ctx) -> FileModel`: the file's declarations, split into
//!   parts per the contract in [`model`]. The one big hook.
//! - `is_entrypoint(path, ctx) -> bool`: the language's entry files
//!   (`lib.rs`, `__init__.py`, …), which the engine prices as depth ≤ 1.
//! - `file_weight(path, ctx) -> f64`: a language-specific file-role
//!   multiplier on every batch of the file; 1.0 unless a measured rule
//!   says otherwise.
//! - `RunState`: per-run caches the language needs, reachable as
//!   `ctx.code.<lang>`; empty unless needed.
//!
//! The engine (`emit`, `chunk`, `ledger`) owns keys, predecessors,
//! chunking, row ownership and value; a language module never builds a
//! batch.

mod c;
mod chunk;
mod emit;
mod go;
mod ledger;
mod lua;
pub(crate) mod model;
mod python;
mod rust;
mod typescript;

use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use self::model::FileModel;
use super::fs::files_with_any_extension;
use super::{WalkCtx, node_end_row_trimmed};
use crate::batch::{Batch, BatchKey};
use crate::render::Source;

/// The languages the engine can walk: a closed set, dispatched by `match`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Language {
    Rust,
    TypeScript,
    Python,
    Go,
    C,
    Lua,
}

impl Language {
    const ALL: [Language; 6] = [
        Language::Rust,
        Language::TypeScript,
        Language::Python,
        Language::Go,
        Language::C,
        Language::Lua,
    ];

    /// The language whose `EXTENSIONS` include `path`'s extension.
    pub(crate) fn from_path(path: &Path) -> Option<Language> {
        let extension = path.extension()?.to_str()?;
        Self::ALL.into_iter().find(|language| {
            language
                .extensions()
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        })
    }

    /// Short name leading every `CodeKey` descriptor.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::TypeScript => "ts",
            Language::Python => "python",
            Language::Go => "go",
            Language::C => "c",
            Language::Lua => "lua",
        }
    }

    fn is_ported(self) -> bool {
        match self {
            Language::Rust => rust::PORTED,
            Language::TypeScript => typescript::PORTED,
            Language::Python => python::PORTED,
            Language::Go => go::PORTED,
            Language::C => c::PORTED,
            Language::Lua => lua::PORTED,
        }
    }

    fn extensions(self) -> &'static [&'static str] {
        match self {
            Language::Rust => rust::EXTENSIONS,
            Language::TypeScript => typescript::EXTENSIONS,
            Language::Python => python::EXTENSIONS,
            Language::Go => go::EXTENSIONS,
            Language::C => c::EXTENSIONS,
            Language::Lua => lua::EXTENSIONS,
        }
    }

    fn grammar(self, path: &Path) -> tree_sitter::Language {
        match self {
            Language::Rust => rust::grammar(path),
            Language::TypeScript => typescript::grammar(path),
            Language::Python => python::grammar(path),
            Language::Go => go::grammar(path),
            Language::C => c::grammar(path),
            Language::Lua => lua::grammar(path),
        }
    }

    fn extract(self, file: &SourceFile, ctx: &WalkCtx) -> FileModel {
        match self {
            Language::Rust => rust::extract(file, ctx),
            Language::TypeScript => typescript::extract(file, ctx),
            Language::Python => python::extract(file, ctx),
            Language::Go => go::extract(file, ctx),
            Language::C => c::extract(file, ctx),
            Language::Lua => lua::extract(file, ctx),
        }
    }

    pub(crate) fn is_entrypoint(self, path: &Path, ctx: &WalkCtx) -> bool {
        match self {
            Language::Rust => rust::is_entrypoint(path, ctx),
            Language::TypeScript => typescript::is_entrypoint(path, ctx),
            Language::Python => python::is_entrypoint(path, ctx),
            Language::Go => go::is_entrypoint(path, ctx),
            Language::C => c::is_entrypoint(path, ctx),
            Language::Lua => lua::is_entrypoint(path, ctx),
        }
    }

    pub(crate) fn file_weight(self, path: &Path, ctx: &WalkCtx) -> f64 {
        match self {
            Language::Rust => rust::file_weight(path, ctx),
            Language::TypeScript => typescript::file_weight(path, ctx),
            Language::Python => python::file_weight(path, ctx),
            Language::Go => go::file_weight(path, ctx),
            Language::C => c::file_weight(path, ctx),
            Language::Lua => lua::file_weight(path, ctx),
        }
    }
}

/// Per-run state of the language modules, one field per language.
#[derive(Default)]
#[allow(
    dead_code,
    reason = "a language reads its field once its port needs run state"
)]
pub(crate) struct CodeState {
    rust: rust::RunState,
    typescript: typescript::RunState,
    python: python::RunState,
    go: go::RunState,
    c: c::RunState,
    lua: lua::RunState,
}

/// One parsed source file, built once and shared by extraction and
/// emission.
pub(crate) struct SourceFile {
    pub path: PathBuf,
    pub source: Arc<Source>,
    pub tree: Arc<Tree>,
}

impl SourceFile {
    fn parse(path: &Path, language: Language, ctx: &WalkCtx) -> Option<Self> {
        let (source, tree) = ctx.parse_tree(path, &language.grammar(path))?;
        Some(Self {
            path: path.to_path_buf(),
            source,
            tree,
        })
    }

    /// Text of 1-based `row` without its line terminator; empty past the
    /// end of the file.
    pub(crate) fn line(&self, row: usize) -> &str {
        self.source.line(row).unwrap_or("")
    }

    pub(crate) fn line_count(&self) -> usize {
        self.source.line_count()
    }

    #[allow(dead_code, reason = "extraction helper for the unported languages")]
    pub(crate) fn text(&self, node: Node) -> &str {
        &self.source[node.byte_range()]
    }

    /// 1-based rows `node` covers, without a final row it reaches only
    /// with whitespace.
    pub(crate) fn node_rows(&self, node: Node) -> RangeInclusive<usize> {
        node.start_position().row + 1..=node_end_row_trimmed(node, &self.source) + 1
    }
}

/// Batches for every file in `dir` whose language is ported. Called by
/// `FsWalker` once per scheduled directory listing.
pub(crate) fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
    let extensions: Vec<&str> = Language::ALL
        .into_iter()
        .filter(|language| language.is_ported())
        .flat_map(|language| language.extensions().iter().copied())
        .collect();
    if extensions.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for path in files_with_any_extension(dir, &extensions, ctx) {
        let Some(language) = Language::from_path(&path) else {
            continue;
        };
        let Some(file) = SourceFile::parse(&path, language, ctx) else {
            continue;
        };
        let model = language.extract(&file, ctx);
        out.extend(emit::emit_file(language, &file, model, ctx));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::{CodeKey, Rung, WalkerKey};

    #[test]
    fn code_mod_describe_names_language_rung_line_and_chunk() {
        let root = Path::new("/repo");
        let describe = |rung, file: &str, sub, line| {
            BatchKey::from(CodeKey {
                rung,
                file: root.join(file),
                decl: 3,
                sub,
                line,
            })
            .describe(root)
        };
        assert_eq!(
            describe(Rung::Decl, "pkg/a.go", 0, 42),
            "go decl pkg/a.go:42"
        );
        assert_eq!(
            describe(Rung::Names, "src/lib.rs", 1, 0),
            "rust names src/lib.rs #1"
        );
        assert_eq!(
            describe(Rung::Body, "web/App.JSX", 2, 7),
            "ts body web/App.JSX:7 #2"
        );
        assert_eq!(Language::from_path(Path::new("x.h")), Some(Language::C));
        assert_eq!(Language::from_path(Path::new("x.cpp")), None);
    }
}
