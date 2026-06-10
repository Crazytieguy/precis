//! `validate-ns`: load a North Star TOML, simulate its batches in rank
//! order against the fixture source, print per-batch costs + violations,
//! and exit non-zero if anything fails.
//!
//! Usage:
//!   cargo run --bin validate-ns -- tests/north-stars/log.toml
//!
//! The fixture root is derived from the NS's `fixture = "…"` field,
//! resolved as `tests/fixtures/<name>/` relative to the current workdir
//! (canonical for this repo layout). A `--fixture-root` override is
//! supported for out-of-tree experimentation.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;

use precis::ns_loader::load_ns_checked;
use precis::ns_simulate::{SimulationReport, TOKEN_CAP, Violation, simulate_ns};

#[derive(Parser, Debug)]
#[command(about = "Validate a North Star TOML against a fixture.")]
struct Cli {
    /// Path to the NS TOML.
    ns_path: PathBuf,
    /// Override the fixture-root path. Default: `tests/fixtures/<ns.fixture>`
    /// relative to the current working directory.
    #[arg(long)]
    fixture_root: Option<PathBuf>,
}

fn main() -> ExitCode {
    match run() {
        Ok(exit) => exit,
        Err(e) => {
            eprintln!("validate-ns: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();

    let fixture_root = match &cli.fixture_root {
        Some(p) => p.clone(),
        None => {
            // Parse just enough of the TOML to learn the fixture name.
            let probe = precis::ns_loader::load_ns(&cli.ns_path)
                .with_context(|| format!("loading {} (pre-pin-check)", cli.ns_path.display()))?;
            PathBuf::from("tests/fixtures").join(&probe.fixture)
        }
    };

    let ns = load_ns_checked(&cli.ns_path, &fixture_root)?;
    let report = simulate_ns(&ns, &fixture_root)?;

    let had_violations = print_report(&cli.ns_path, &ns.fixture, &report);
    if had_violations {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Print per-batch table + any violations. Returns `true` iff any batch
/// had violations (so main can exit non-zero).
fn print_report(ns_path: &std::path::Path, fixture: &str, report: &SimulationReport) -> bool {
    let violation_count: usize = report.batches.iter().map(|b| b.violations.len()).sum();

    if violation_count == 0 {
        println!(
            "{}: OK ({} batches, cumulative {} / {})",
            ns_path.display(),
            report.batches.len(),
            report.total_tokens,
            TOKEN_CAP,
        );
    } else {
        println!(
            "{}: {} violation(s) ({} batches, cumulative {} / {})",
            ns_path.display(),
            violation_count,
            report.batches.len(),
            report.total_tokens,
            TOKEN_CAP,
        );
    }

    println!("fixture: {fixture}");
    if let Some((id, cost)) = &report.largest_batch {
        println!("largest batch: {cost} tokens @ #{id}");
    }
    println!();
    println!("  pos  id{:20}  cost   cum    descriptor", "");
    for (pos, b) in report.batches.iter().enumerate() {
        let id_cell = format!("{:20}", b.id);
        let flag = if b.violations.is_empty() { " " } else { "!" };
        println!(
            "  {pos:>3}  {id_cell}{flag} {:>5}  {:>5}  {}",
            b.marginal_cost.tokens, b.cumulative_tokens, b.descriptor,
        );
    }

    let mut printed_header = false;
    for b in &report.batches {
        for v in &b.violations {
            if !printed_header {
                println!();
                println!("Violations:");
                printed_header = true;
            }
            println!("  [{}] {}", b.id, format_violation(v));
        }
    }

    violation_count > 0
}

fn format_violation(v: &Violation) -> String {
    match v {
        Violation::DuplicateBatchId(id) => {
            format!("duplicate batch id {id:?} (first occurrence passes; this is the clash)")
        }
        Violation::PredecessorMissing(id) => {
            format!("predecessor {id:?} not found in prior batches")
        }
        Violation::SpanFileMissing(path) => {
            format!("span file missing: {}", path.display())
        }
        Violation::SpanInvertedRange { path, start, end } => {
            format!(
                "span range inverted or zero-indexed at {}: start={start}, end={end}",
                path.display()
            )
        }
        Violation::SpanOutOfRange {
            path,
            start,
            end,
            file_lines,
        } => {
            format!(
                "span out of range at {}: {start}..={end} (file has {file_lines} lines)",
                path.display()
            )
        }
        Violation::RegexInvalid {
            path,
            start,
            end,
            pattern,
        } => format!(
            "invalid Truncated regex `{pattern}` at {}:{start}..={end}",
            path.display()
        ),
        Violation::RegexNoMatch {
            path,
            line,
            pattern,
        } => format!(
            "Truncated regex `{pattern}` produced no/empty match at {}:{line}",
            path.display()
        ),
        Violation::TruncationSavesNothing {
            path,
            line,
            pattern,
            full_tokens,
            truncated_tokens,
        } => format!(
            "Truncated render saves no tokens at {}:{line} with pattern `{pattern}` (full line: {full_tokens} tokens, truncated `<match>…`: {truncated_tokens} tokens). Use `Full` here, or pick a pattern that drops the meaningful tail.",
            path.display()
        ),
        Violation::EllipsisMultiLine { path, start, end } => format!(
            "multi-line Ellipsis span at {}:{start}..={end} renders one `…` per line — redundant + visually indistinguishable from a single marker. Use one Ellipsis span per line, or `Full`/`Truncated` if the lines should render verbatim.",
            path.display()
        ),
        Violation::FsResolveFailed(msg) => format!("fs content resolution failed: {msg}"),
        Violation::NonAncestorOverlap {
            path,
            line,
            existing_batch,
        } => format!(
            "non-ancestor overlap: {}:{line} already owned by {existing_batch} (add a predecessor edge or move the span)",
            path.display()
        ),
        Violation::GrowthEnvelope {
            cost,
            cumulative_before,
            max_allowed,
        } => format!(
            "growth envelope: batch cost {cost} > {max_allowed} tokens (cumulative so far: {cumulative_before}; envelope = 100 + 0.3·cumulative). Split the batch, or rank smaller batches earlier."
        ),
        Violation::OverlappingSpans { path, line } => format!(
            "overlapping spans within one batch at {}:{line} (batch spans must be disjoint — cross-batch overrides go through predecessor edges)",
            path.display()
        ),
        Violation::OverlappingFsEntry {
            parent,
            entry,
            existing_batch,
        } => format!(
            "overlapping fs entry: {} lists {entry:?}, already owned by {existing_batch} (split-listing rows must partition entries)",
            parent.display()
        ),
        Violation::CapExceeded { cumulative, cap } => {
            format!("cap exceeded: cumulative {cumulative} > {cap} tokens")
        }
        Violation::EmptyBatch => {
            "batch resolves to zero atoms — it renders as a no-op, so divergence can never credit it and it is counted missing forever. Delete the batch or point it at real content.".to_string()
        }
        Violation::SpanPathEscapesRoot { path } => format!(
            "span path {} escapes the fixture root (span paths must be fixture-root-relative — no absolute paths or `..`)",
            path.display()
        ),
    }
}
