use std::path::Path;

use anyhow::{Result, anyhow};

pub mod batch;
pub mod render;
pub mod scheduler;
pub mod tokenizer;
pub mod walker;

use scheduler::Scheduler;
use walker::generic::GenericWalker;

/// Render a precis summary of the given path(s) under the given budgets.
///
/// First-pass v0.2 supports a single seed path. The CLI accepts a `Vec<PathBuf>`
/// for forward compatibility, but multi-path scheduling is in the deferred set.
pub fn render(
    paths: &[impl AsRef<Path>],
    token_budget: usize,
    char_budget: Option<usize>,
) -> Result<String> {
    let path = paths
        .first()
        .ok_or_else(|| anyhow!("no path provided"))?
        .as_ref();
    let root = path
        .canonicalize()
        .map_err(|e| anyhow!("failed to canonicalize {}: {}", path.display(), e))?;
    let scheduler = Scheduler::new(root, GenericWalker::new(), token_budget, char_budget);
    let tree = scheduler.run();
    Ok(tree.render())
}
