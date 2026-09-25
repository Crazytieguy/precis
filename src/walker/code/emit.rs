//! [`FileModel`] → batches: normalization, then the ladder per file.
//!
//! Emission order, each batch gated on its predecessor:
//! `ModuleDoc` chunks (no predecessor), `Names` chunks (no predecessor),
//! then per declaration in normalized order its `Decl` chunks (gated on the
//! owner of its first name row: the `Names` chunk listing it, or for a
//! member the container `Decl` chunk listing it), `Doc` and, for
//! `Callable`, `Body` chunks (gated on the declaration's first `Decl`
//! chunk, or its owner when that chunk was covered). Chunk `i > 0` of any
//! part is gated on chunk `i - 1`.

use super::model::FileModel;
use super::{Language, SourceFile};
use crate::batch::{Batch, BatchKey};
use crate::walker::WalkCtx;

/// Every batch of one file.
pub(super) fn emit_file(
    _language: Language,
    _file: &SourceFile,
    _model: FileModel,
    _ctx: &WalkCtx,
) -> Vec<Batch<BatchKey>> {
    Vec::new()
}

/// The engine-side steps of the [`super::model`] contract: order by first
/// row, merge same-first-row declarations, trim at the next sibling, drop
/// blank rows, sort and dedup rows, strip `module_doc` rows from every
/// other part.
pub(super) fn normalize(model: FileModel, _file: &SourceFile) -> FileModel {
    model
}
