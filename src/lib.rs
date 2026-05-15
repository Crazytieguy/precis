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
#[cfg(feature = "timing")]
pub mod timing;
pub mod tokenizer;
pub mod value;
pub mod walker;

// Zero-cost without the `timing` feature: both macros expand to nothing.
// One `time_span!` or `time_counter!` per `{}` block — the underscore-prefixed
// binding makes a second invocation in the same block shadow (and prematurely
// drop) the first. Nest into a child block to time two adjacent phases.
macro_rules! time_span {
    ($name:literal) => {
        #[cfg(feature = "timing")]
        let _timing_phase_timer = $crate::timing::PhaseTimer::new($name);
    };
}

macro_rules! time_counter {
    ($slot:ident) => {
        #[cfg(feature = "timing")]
        let _timing_counter_guard = $crate::timing::CounterGuard::new(|c| &mut c.$slot);
    };
}

pub(crate) use {time_counter, time_span};

pub use batch::{Batch, BatchKey, FsKey, MarkdownKey, RustKey, TomlKey, WalkerKey};
pub use content::{BatchContent, FsEntries, FsGroup, Render, Span};
pub use fs_util::{EntryKind, list_dir};
pub use render::{Cost, RenderedTree, SourceCache};
pub use schedule_types::{Atom, CandidateBatch, Schedule, ScheduledBatch};

use scheduler::Scheduler;
use walker::FsWalker;

/// Render a precis summary of the given path(s) under the given budgets.
///
/// First-pass v0.2 supports a single seed path that must be a directory.
/// Multi-path inputs and file-as-seed are in the deferred set.
pub fn render(
    paths: &[impl AsRef<Path>],
    token_budget: usize,
    byte_budget: Option<usize>,
) -> Result<String> {
    time_span!("render_total");
    let path = paths
        .first()
        .ok_or_else(|| anyhow!("no path provided"))?
        .as_ref();
    let root = canonicalize_dir(path)?;
    let scheduler = Scheduler::new(root, FsWalker, token_budget, byte_budget);
    let tree = scheduler.run();
    let out = {
        time_span!("final_render");
        tree.render()
    };
    #[cfg(feature = "timing")]
    timing::dump_and_reset();
    Ok(out)
}

/// Replay a previously-produced [`Schedule`] against a fresh tree at
/// `budget` (≤ the schedule's budget). Stops once cumulative tokens
/// exceed `budget`; under prefix-monotone scheduling this matches what
/// [`render`] would have produced at the smaller budget directly,
/// without re-running the walker.
pub fn render_with_schedule(
    schedule: &Schedule,
    root: impl AsRef<Path>,
    budget: usize,
) -> Result<String> {
    let root = canonicalize_dir(root.as_ref())?;
    let mut tree = RenderedTree::new(root, SourceCache::new());
    for (i, sb) in schedule.batches.iter().enumerate() {
        if sb.cum_tokens > budget {
            break;
        }
        tree.apply(&sb.content, batch::BatchId::new(i), |_| true);
    }
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

    let scheduler = Scheduler::new(root.clone(), FsWalker, budget, None);
    let report = scheduler.run_with_report();

    let cumulative_tokens = report.scheduled.last().map(|b| b.cum_tokens).unwrap_or(0);
    let batches = report
        .scheduled
        .into_iter()
        .enumerate()
        .map(|(i, b)| ScheduledBatch {
            position: i + 1,
            key: format!("{:?}", &b.key),
            descriptor: WalkerKey::describe(&b.key, &root),
            cost_tokens: b.cost.tokens,
            cum_tokens: b.cum_tokens,
            content: b.content,
        })
        .collect::<Vec<_>>();
    let candidates = report
        .candidates
        .into_iter()
        .map(|b| CandidateBatch {
            key: format!("{:?}", &b.key),
            predecessor: b.predecessor.as_ref().map(|p| format!("{p:?}")),
            descriptor: WalkerKey::describe(&b.key, &root),
            content: b.content,
        })
        .collect();
    Ok(Schedule {
        fixture,
        budget,
        cumulative_tokens,
        batch_count: batches.len(),
        batches,
        candidates,
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
