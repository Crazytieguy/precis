//! TypeScript / JavaScript extraction for the code engine. Not ported
//! yet: `walker::typescript` still walks these files.

use std::path::Path;

use super::SourceFile;
use super::model::FileModel;
use crate::walker::WalkCtx;

pub(super) const PORTED: bool = false;
pub(super) const EXTENSIONS: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];

/// The TypeScript grammar for `.ts` / `.mts` / `.cts`; the TSX grammar,
/// which also parses JSX, for everything else.
pub(super) fn grammar(path: &Path) -> tree_sitter::Language {
    let is_typescript = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            ["ts", "mts", "cts"]
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        });
    if is_typescript {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    }
}

pub(super) fn extract(_file: &SourceFile, _ctx: &WalkCtx) -> FileModel {
    FileModel::default()
}

pub(super) fn is_entrypoint(_path: &Path, _ctx: &WalkCtx) -> bool {
    false
}

pub(super) fn file_weight(_path: &Path, _ctx: &WalkCtx) -> f64 {
    1.0
}

#[derive(Default)]
pub(crate) struct RunState {}
