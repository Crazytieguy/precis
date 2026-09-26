//! Simulate a North Star against its fixture (`tests/fixtures/<fixture>`),
//! print per-batch costs and violations, and exit 1 if there are any, 2
//! if the NS can't be loaded or its revision pin doesn't match the
//! fixture's.
//!
//! Usage: cargo run --example validate_ns -- tests/north-stars/log.toml

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};

use precis::ns_loader::{check_pin, load_ns};
use precis::ns_simulate::{TOKEN_CAP, simulate_ns};

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("validate_ns: {e:#}");
            ExitCode::from(2)
        }
    }
}

/// Returns whether the NS is clean.
fn run() -> Result<bool> {
    let ns_path = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .context("usage: validate_ns <north-star.toml>")?,
    );
    let ns = load_ns(&ns_path)?;
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(&ns.fixture);
    check_pin(&ns, &ns_path, &fixture_root)?;
    let batches = simulate_ns(&ns, &fixture_root);

    let violation_count: usize = batches.iter().map(|b| b.violations.len()).sum();
    let total = batches.last().map_or(0, |b| b.cumulative_tokens);
    let status = match violation_count {
        0 => "OK".to_string(),
        n => format!("{n} violation(s)"),
    };
    println!(
        "{}: {status} ({} batches, cumulative {total} / {TOKEN_CAP})",
        ns_path.display(),
        batches.len(),
    );
    println!("fixture: {}", ns.fixture);
    if let Some(largest) = batches.iter().rev().max_by_key(|b| b.cost_tokens) {
        println!(
            "largest batch: {} tokens (id {})",
            largest.cost_tokens, largest.id
        );
    }
    println!();
    println!(
        "  pos  {:20}  {:>5}  {:>5}  descriptor",
        "id", "cost", "cum"
    );
    for (position, b) in batches.iter().enumerate() {
        let flag = if b.violations.is_empty() { " " } else { "!" };
        println!(
            "  {position:>3}  {:20}{flag} {:>5}  {:>5}  {}",
            b.id, b.cost_tokens, b.cumulative_tokens, b.descriptor,
        );
    }
    if violation_count > 0 {
        println!();
        println!("Violations:");
        for b in &batches {
            for violation in &b.violations {
                println!("  [{}] {violation}", b.id);
            }
        }
    }
    Ok(violation_count == 0)
}
