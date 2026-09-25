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
pub mod value;
pub mod walker;

pub use batch::{Batch, BatchKey};
pub use content::{BatchContent, FsEntries, FsGroup, Render, Span};
pub use fs_util::{DirFilter, EntryKind, list_dir};
pub use render::{Cost, RenderedTree, SourceCache, char_units};

use scheduler::Scheduler;
use walker::FsWalker;

/// Render a precis summary of the directory or file at `path` under the
/// given budgets (`char_budget` in [`char_units`]).
pub fn render(path: &Path, token_budget: usize, char_budget: Option<usize>) -> Result<String> {
    let scheduler = Scheduler::with_filter(walk_scope(path)?, FsWalker, token_budget, char_budget);
    Ok(scheduler.run().render())
}

/// Replay a previously-produced [`Schedule`] against a fresh tree at
/// `budget`. Under prefix-monotone scheduling this matches running
/// `render` at the smaller budget, without re-running the walker.
pub fn render_with_schedule(schedule: &Schedule, budget: usize) -> String {
    let mut tree = RenderedTree::new(schedule.root.clone(), SourceCache::new());
    for (i, sb) in schedule.batches.iter().enumerate() {
        if sb.cum_tokens > budget {
            break;
        }
        tree.apply(&sb.content, batch::BatchId::new(i), |_| true);
    }
    tree.render()
}

/// Complete walker schedule from one [`render_schedule`] run, in
/// scheduling order.
pub struct Schedule {
    /// Canonical fixture root; batch content paths are absolute under it.
    pub root: std::path::PathBuf,
    pub batches: Vec<ScheduledBatch>,
}

pub struct ScheduledBatch {
    pub descriptor: String,
    pub cost_tokens: usize,
    pub cum_tokens: usize,
    pub content: BatchContent,
}

/// Run the walker at `budget` and return a [`Schedule`] — input for
/// regression snapshots and the divergence metric.
pub fn render_schedule(path: &Path, budget: usize) -> Result<Schedule> {
    let filter = walk_scope(path)?;
    let root = filter.root().to_path_buf();
    let report = Scheduler::with_filter(filter, FsWalker, budget, None).run_with_report();
    let batches = report
        .scheduled
        .into_iter()
        .map(|b| ScheduledBatch {
            descriptor: b.key.describe(&root),
            cost_tokens: b.cost.tokens,
            cum_tokens: b.cum_tokens,
            content: b.content,
        })
        .collect();
    Ok(Schedule { root, batches })
}

/// The walk that summarizes `path`: the whole tree for a directory, or
/// for a file, its directory with only that file admitted.
fn walk_scope(path: &Path) -> Result<DirFilter> {
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
    Ok(DirFilter::single_file(&resolved))
}
