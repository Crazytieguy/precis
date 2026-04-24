use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};

pub mod batch;
pub mod content;
pub mod divergence;
pub mod fs_util;
pub mod north_star;
pub mod ns_loader;
pub mod ns_simulate;
pub mod render;
pub mod schedule_types;
pub mod scheduler;
pub mod tokenizer;
pub mod value;
pub mod walker;

pub use batch::{
    Batch, BatchKey, FsKey, MarkdownKey, ResolvedBatch, RustKey, TomlKey, ValueSignals, WalkerKey,
};
pub use content::{BatchContent, FsEntries, FsGroup, Render, Span};
pub use fs_util::{EntryKind, list_dir};
pub use render::{Cost, RenderedTree, SourceCache};
pub use schedule_types::{Atom, Schedule, ScheduledBatch};

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
    let root = canonicalize_dir(path)?;
    let scheduler = Scheduler::new(root, MultiWalker, token_budget, byte_budget);
    let tree = scheduler.run();
    Ok(tree.render())
}

/// Run the walker at `budget` and return a structured `Schedule` — the
/// input both the regression-snapshot test and `compare-ns` (divergence
/// metric) consume. Strings out the ordered batch log: keys are formatted
/// via `BatchKey::describe()` for readable diffs; content is the resolved
/// library `BatchContent` (fs groups with explicit children, lines with
/// spans).
pub fn render_schedule(paths: &[impl AsRef<Path>], budget: usize) -> Result<Schedule> {
    let path = paths
        .first()
        .ok_or_else(|| anyhow!("no path provided"))?
        .as_ref();
    let root = canonicalize_dir(path)?;
    let fixture = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let scheduler = Scheduler::new(root, MultiWalker, budget, None);
    let report = scheduler.run_with_report();

    let cumulative_tokens = report.scheduled.last().map(|b| b.cum_tokens).unwrap_or(0);
    let batches = report
        .scheduled
        .into_iter()
        .enumerate()
        .map(|(i, b)| ScheduledBatch {
            position: i + 1,
            key: format!("{:?}", &b.key),
            descriptor: WalkerKey::describe(&b.key),
            cost_tokens: b.cost.tokens,
            cum_tokens: b.cum_tokens,
            content: b.content,
        })
        .collect::<Vec<_>>();
    Ok(Schedule {
        fixture,
        budget,
        cumulative_tokens,
        batch_count: batches.len(),
        batches,
    })
}

fn canonicalize_dir(path: &Path) -> Result<std::path::PathBuf> {
    let root = path
        .canonicalize()
        .with_context(|| format!("failed to canonicalize {}", path.display()))?;
    if !root.is_dir() {
        bail!(
            "{} is not a directory; v0.2 first pass only supports directory roots",
            root.display()
        );
    }
    Ok(root)
}
