//! Multi-language walker: the top-level [`Walker`] implementation. Seeds
//! with the root FS listing; on expand, unions the contribution of every
//! per-language walker so each one proposes candidates keyed off the same
//! FS listings; on materialize, dispatches by [`BatchKey`] variant.

use crate::batch::{BatchKey, ResolvedBatch};

use super::{Candidate, WalkCtx, Walker, fs, markdown, rust, toml};

#[derive(Default)]
pub struct MultiWalker;

impl Walker for MultiWalker {
    type Key = BatchKey;

    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
        fs::seed(ctx)
    }

    fn expand(&mut self, scheduled: &BatchKey, ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
        let mut out = Vec::new();
        out.extend(fs::expand(scheduled, ctx));
        out.extend(rust::expand(scheduled, ctx));
        out.extend(markdown::expand(scheduled, ctx));
        out.extend(toml::expand(scheduled, ctx));
        out
    }

    fn materialize(&mut self, key: &BatchKey, ctx: &WalkCtx) -> Option<ResolvedBatch> {
        match key {
            BatchKey::Fs(_) => fs::materialize(key, ctx),
            BatchKey::Rust(_) => rust::materialize(key, ctx),
            BatchKey::Markdown(_) => markdown::materialize(key, ctx),
            BatchKey::Toml(_) => toml::materialize(key, ctx),
        }
    }
}
