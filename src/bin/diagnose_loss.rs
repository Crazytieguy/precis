//! Corpus-wide `Score(3000)` loss decomposition + oracle ceiling over
//! the training fixtures. Diagnostic-only; prints markdown to stdout.
//! See `src/divergence/diagnose.rs` for bucket semantics.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Parser;

use precis::divergence::diagnose::{LossReport, diagnose};
use precis::ns_loader::load_ns_checked;
use precis::render_schedule;

/// Matches the fixture-baseline harness cap.
const SCHEDULE_BUDGET: usize = 10_000;

#[derive(Parser)]
struct Cli {
    /// Fixtures to diagnose (default: every training fixture).
    fixtures: Vec<String>,
    /// Print each fixture's oracle purchase list (descriptor + tokens).
    #[arg(long)]
    oracle_schedules: bool,
    /// Print per-fixture Score at every grid budget plus corpus means.
    #[arg(long)]
    budgets: bool,
}

fn training_fixtures() -> Result<Vec<String>> {
    let mut names: Vec<String> = std::fs::read_dir("tests/divergence")
        .context("reading tests/divergence")?
        .filter_map(|e| {
            let p = e.ok()?.path();
            (p.extension()? == "md").then(|| p.file_stem()?.to_str().map(str::to_owned))?
        })
        .collect();
    names.sort();
    Ok(names)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let fixtures = if cli.fixtures.is_empty() {
        training_fixtures()?
    } else {
        cli.fixtures
    };

    let mut rows: Vec<(String, LossReport)> = Vec::new();
    for name in &fixtures {
        let fixture_dir = PathBuf::from("tests/fixtures").join(name);
        if !fixture_dir.exists() {
            bail!("fixture `{name}` not present; run `cargo run --bin clone_fixtures`");
        }
        let ns_path = PathBuf::from("tests/north-stars").join(format!("{name}.toml"));
        let ns = load_ns_checked(&ns_path, &fixture_dir)?;
        let start = std::time::Instant::now();
        let schedule = render_schedule(&[&fixture_dir], SCHEDULE_BUDGET)?;
        let report = diagnose(&ns, &schedule, &fixture_dir)?;
        eprintln!(
            "{name}: score={:.3} oracle={:.3} pool={} ({:.1}s)",
            report.baseline.vector[precis::divergence::PRIMARY_BUDGET_INDEX].score,
            report.oracle.score,
            report.pool_batches,
            start.elapsed().as_secs_f64(),
        );
        rows.push((name.clone(), report));
    }

    rows.sort_by(|a, b| {
        let sa = a.1.baseline.vector[precis::divergence::PRIMARY_BUDGET_INDEX].score;
        let sb = b.1.baseline.vector[precis::divergence::PRIMARY_BUDGET_INDEX].score;
        sa.total_cmp(&sb)
    });

    println!("# Score(3000) loss decomposition + oracle ceiling");
    println!();
    println!(
        "Buckets are fractions of |A_3K| (sum with coverage ≈ 1): damp = completion damping on delivered atoms; partial = delivered with fewer bytes than NS; late = delivered past 3K in the capped schedule; unsched = emittable but never scheduled at 10K; absent = not in the full expansion pool (recall gap). Oracle = NS-aware greedy over the full pool at 3000."
    );
    println!();
    println!(
        "| fixture | score | oracle | Δorcl | I | C | damp | partial | late | unsched | absent | pool | orcl_used |"
    );
    println!("|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let mut sums = [0.0f64; 11];
    for (name, r) in &rows {
        let p = &r.baseline.vector[precis::divergence::PRIMARY_BUDGET_INDEX];
        let b = &r.buckets;
        let o = &r.oracle;
        let cols = [
            p.score,
            o.score,
            o.score - p.score,
            p.importance,
            p.coverage,
            b.damping,
            b.partial_render,
            b.late,
            b.unscheduled,
            b.absent,
        ];
        for (s, c) in sums.iter_mut().zip(cols.iter()) {
            *s += c;
        }
        sums[10] += o.used_tokens as f64;
        println!(
            "| {name} | {:.3} | {:.3} | {:+.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {} | {} |",
            cols[0],
            cols[1],
            cols[2],
            cols[3],
            cols[4],
            cols[5],
            cols[6],
            cols[7],
            cols[8],
            cols[9],
            r.pool_batches,
            o.used_tokens,
        );
    }
    let n = rows.len().max(1) as f64;
    println!(
        "| **mean** | **{:.4}** | **{:.4}** | **{:+.4}** | **{:.3}** | **{:.3}** | **{:.3}** | **{:.3}** | **{:.3}** | **{:.3}** | **{:.3}** |  | {:.0} |",
        sums[0] / n,
        sums[1] / n,
        sums[2] / n,
        sums[3] / n,
        sums[4] / n,
        sums[5] / n,
        sums[6] / n,
        sums[7] / n,
        sums[8] / n,
        sums[9] / n,
        sums[10] / n,
    );

    println!();
    println!("## Top per-file losses (per fixture)");
    println!();
    for (name, r) in &rows {
        println!("### {name}");
        for fl in &r.top_file_losses {
            println!("- {:.3} {} `{}`", fl.loss, fl.bucket, fl.path.display());
        }
        println!();
    }

    if cli.budgets {
        println!();
        println!("## Score per grid budget");
        println!();
        let budgets = precis::divergence::BUDGETS;
        let header: Vec<String> = budgets.iter().map(|b| b.to_string()).collect();
        println!("| fixture | {} |", header.join(" | "));
        println!("|:--|{}|", "--:|".repeat(budgets.len()));
        let mut budget_sums = vec![0.0f64; budgets.len()];
        for (name, r) in &rows {
            let cells: Vec<String> = r
                .baseline
                .vector
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    budget_sums[i] += s.score;
                    format!("{:.3}", s.score)
                })
                .collect();
            println!("| {name} | {} |", cells.join(" | "));
        }
        let means: Vec<String> = budget_sums
            .iter()
            .map(|s| format!("**{:.4}**", s / n))
            .collect();
        println!("| **mean** | {} |", means.join(" | "));
    }

    if cli.oracle_schedules {
        println!("## Oracle purchases (per fixture)");
        println!();
        for (name, r) in &rows {
            println!("### {name}");
            for (desc, tokens) in &r.oracle.schedule {
                println!("- {tokens:>5} {desc}");
            }
            println!();
        }
    }
    Ok(())
}
