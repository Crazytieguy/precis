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

use precis::ns_simulate::{SimulationReport, Violation, simulate_ns};
use precis::schema::load_ns_checked;

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
            let probe = precis::schema::load_ns(&cli.ns_path)
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
    let cap_note = report.total_tokens > crate_cap();
    let violation_count: usize = report.batches.iter().map(|b| b.violations.len()).sum();

    if violation_count == 0 && !cap_note {
        println!(
            "{}: OK ({} batches, cumulative {} / {})",
            ns_path.display(),
            report.batches.len(),
            report.total_tokens,
            crate_cap(),
        );
    } else {
        println!(
            "{}: {} violation(s) ({} batches, cumulative {} / {})",
            ns_path.display(),
            violation_count,
            report.batches.len(),
            report.total_tokens,
            crate_cap(),
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
        Violation::PredecessorMissing(id) => {
            format!("predecessor {:?} not found in prior batches", id)
        }
        Violation::TwoXRule { largest_preceding } => {
            format!(
                "2× violation: cost exceeds 2 × largest preceding batch ({} tokens)",
                largest_preceding
            )
        }
        Violation::CapExceeded { cumulative, cap } => {
            format!("cap exceeded: cumulative {cumulative} > {cap} tokens")
        }
        Violation::ResolveFailed(msg) => format!("content resolution failed: {msg}"),
    }
}

/// The cap the simulator enforces. Duplicated from `ns_simulate::TOKEN_CAP`
/// (which is private) so this bin can show it in output — kept intentionally
/// in sync; if one changes, both do.
fn crate_cap() -> usize {
    10_000
}
