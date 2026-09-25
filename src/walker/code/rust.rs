//! Rust extraction for the code engine. Not ported yet: `walker::rust` still
//! walks these files.

use std::path::Path;

use super::SourceFile;
use super::model::FileModel;
use crate::walker::WalkCtx;

pub(super) const PORTED: bool = false;
pub(super) const EXTENSIONS: &[&str] = &["rs"];

pub(super) fn grammar(_path: &Path) -> tree_sitter::Language {
    tree_sitter_rust::LANGUAGE.into()
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
