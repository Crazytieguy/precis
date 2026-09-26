use std::path::Path;

use anyhow::{Context, Result, bail};

pub mod batch;
pub mod content;
pub mod divergence;
pub mod fs_util;
pub mod north_star;
pub mod ns_loader;
pub mod ns_simulate;
pub mod render;
pub mod scheduler;
pub mod tokenizer;
mod value;
pub mod walker;

pub use render::char_units;

use fs_util::DirFilter;
use render::SourceCache;
use scheduler::Scheduler;
use walker::{FsWalker, WalkCtx};

/// Render a precis summary of the directory or file at `path` under the
/// given budgets (`char_budget` in [`char_units`]).
pub fn render(path: &Path, token_budget: usize, char_budget: Option<usize>) -> Result<String> {
    let ctx = WalkCtx::with_filter(walk_scope(path)?, SourceCache::new());
    let scheduler = Scheduler::new(ctx, FsWalker, token_budget, char_budget);
    Ok(scheduler.run().render())
}

/// The walk that summarizes `path`: the whole tree for a directory, or
/// for a file, its directory with only that file admitted.
pub(crate) fn walk_scope(path: &Path) -> Result<DirFilter> {
    let resolved = path
        .canonicalize()
        .with_context(|| format!("failed to canonicalize {}", path.display()))?;
    if resolved.is_dir() {
        // Without this an unreadable root walks to nothing, and the caller
        // cannot tell a permission error from an empty repository.
        std::fs::read_dir(&resolved)
            .with_context(|| format!("failed to read directory {}", resolved.display()))?;
        return Ok(DirFilter::new(&resolved));
    }
    if !resolved.is_file() {
        bail!(
            "{} is neither a directory nor a regular file",
            path.display()
        );
    }
    // A named file that is a link gets the containment rule a listing
    // applies to a linked entry: it must resolve inside its own directory.
    let named_dir = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    let named_dir = named_dir
        .canonicalize()
        .with_context(|| format!("failed to canonicalize {}", named_dir.display()))?;
    if !resolved.starts_with(&named_dir) {
        bail!(
            "{} resolves to {}, outside {}",
            path.display(),
            resolved.display(),
            named_dir.display()
        );
    }
    // A link named like a credential file shows by that name only,
    // whatever its target is named.
    match path.file_name() {
        Some(name) if walker::is_credential_name(path) => {
            Ok(DirFilter::single_file(&named_dir.join(name)))
        }
        _ => Ok(DirFilter::single_file(&resolved)),
    }
}
