use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};

pub mod batch;
pub mod ns_simulate;
pub mod render;
pub mod scheduler;
pub mod schema;
pub mod tokenizer;
pub mod value;
pub mod walker;

// Public API surface for external binaries (validate-ns, divergence-report
// generator, etc.) and for tests that need to assemble batches directly.
// The schema, simulator, and render_schedule entry points land in later
// commits as their consumers (validate-ns bin, tests/schedule_order.rs)
// are added.
pub use batch::{
    Batch, BatchContent, BatchKey, EntryKind, FsGroup, FsKey, MarkdownKey, Render, ResolvedBatch,
    RustKey, Span, TomlKey, ValueSignals,
};
pub use render::{Cost, RenderedTree, SourceCache};
pub use walker::fs::list_dir;

use scheduler::Scheduler;
use walker::multi::MultiWalker;

/// Render a precis summary of the given path(s) under the given budgets.
///
/// First-pass v0.2 supports a single seed path that must be a directory.
/// Multi-path inputs and file-as-seed are in the deferred set.
pub fn render(
    paths: &[impl AsRef<Path>],
    token_budget: usize,
    byte_budget: Option<usize>,
) -> Result<String> {
    let path = paths
        .first()
        .ok_or_else(|| anyhow!("no path provided"))?
        .as_ref();
    let root = path
        .canonicalize()
        .with_context(|| format!("failed to canonicalize {}", path.display()))?;
    if !root.is_dir() {
        bail!(
            "{} is not a directory; v0.2 first pass only supports directory roots",
            root.display()
        );
    }
    let scheduler = Scheduler::new(root, MultiWalker, token_budget, byte_budget);
    let tree = scheduler.run();
    Ok(tree.render())
}
