//! Per-stage profiling for precis.
//!
//! Usage:
//!   cargo run --release --bin profile -- <path> [--budget N]

use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).map(PathBuf::from).unwrap_or_else(|| {
        eprintln!("Usage: profile <path> [--budget N]");
        std::process::exit(1);
    });
    let budget = args
        .iter()
        .position(|a| a == "--budget")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);

    let start = Instant::now();
    let output = precis::render(&path, budget, None);
    let elapsed = start.elapsed();

    let tokens = precis::format::count_tokens(&output);
    eprintln!("path:    {}", path.display());
    eprintln!("budget:  {}", budget);
    eprintln!("tokens:  {}", tokens);
    eprintln!("chars:   {}", output.len());
    eprintln!("time:    {:.1?}", elapsed);
}
