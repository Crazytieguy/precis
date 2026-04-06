//! Per-stage profiling for precis.
//!
//! Usage:
//!   cargo run --release --bin profile -- <path> [--budget N]

use std::path::Path;
use std::time::Instant;

use precis::{format, layout, schedule, walk};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).unwrap_or_else(|| {
        eprintln!("usage: profile <path> [--budget N]");
        std::process::exit(1);
    });
    let budget = args
        .iter()
        .position(|a| a == "--budget")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000usize);

    let root = Path::new(path).canonicalize().unwrap_or_else(|e| {
        eprintln!("error: {}: {}", path, e);
        std::process::exit(1);
    });

    eprintln!("profiling {} (budget {})\n", root.display(), budget);

    let mut stages: Vec<(&str, std::time::Duration)> = Vec::new();

    // 1. Walk
    let t = Instant::now();
    let files = walk::discover_source_files(&root);
    stages.push(("walk", t.elapsed()));

    // 2. Read
    let t = Instant::now();
    let sources = format::read_sources(&files);
    stages.push(("read", t.elapsed()));

    // 3. Extract symbols
    let t = Instant::now();
    let all_symbols = format::extract_all_symbols(&files, &sources);
    stages.push(("parse", t.elapsed()));

    // 4. Compute layouts
    let t = Instant::now();
    let layouts = layout::compute_all_layouts(&files, &sources, &all_symbols);
    stages.push(("layout", t.elapsed()));

    // 5. Build groups
    let t = Instant::now();
    let groups = schedule::build_groups(&root, &files, &sources, &all_symbols, &layouts);
    stages.push(("groups", t.elapsed()));

    // 6. Schedule
    let t = Instant::now();
    let sched = schedule::schedule(&groups, budget, &root, &files);
    stages.push(("schedule", t.elapsed()));

    // 7. Render
    let t = Instant::now();
    let output = format::render_scheduled(&root, &files, &sources, &all_symbols, &layouts, &groups, &sched);
    stages.push(("render", t.elapsed()));

    // 8. Count tokens
    let t = Instant::now();
    let tokens = format::count_tokens(&output);
    stages.push(("tokens", t.elapsed()));

    // Summary
    let total: std::time::Duration = stages.iter().map(|(_, d)| *d).sum();
    let symbol_count: usize = all_symbols.iter().map(|s| s.len()).sum();

    eprintln!("{:<10} {:>10} {:>6}", "stage", "time", "%");
    eprintln!("{}", "-".repeat(28));
    for (name, dur) in &stages {
        let pct = dur.as_secs_f64() / total.as_secs_f64() * 100.0;
        eprintln!("{:<10} {:>10.1?} {:>5.1}%", name, dur, pct);
    }
    eprintln!("{}", "-".repeat(28));
    eprintln!("{:<10} {:>10.1?}", "total", total);
    eprintln!();
    eprintln!("files:   {}", files.len());
    eprintln!("symbols: {}", symbol_count);
    eprintln!("groups:  {}", groups.len());
    eprintln!("tokens:  {}", tokens);
}
