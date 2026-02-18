use std::path::Path;

struct SnapshotEntry {
    filename: String,
    budget: usize,
    tokens: usize,
}

fn parse_snapshot(path: &Path) -> Option<SnapshotEntry> {
    let content = std::fs::read_to_string(path).ok()?;
    let filename = path.file_name()?.to_str()?.to_string();

    // Find the budget line after the YAML front matter.
    // Format: "budget: 1000 (692 tokens)"
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("budget: ") {
            // "1000 (692 tokens)"
            let (budget_str, rest) = rest.split_once(' ')?;
            let budget: usize = budget_str.parse().ok()?;
            // "(692 tokens)"
            let tokens_str = rest.trim_start_matches('(').split_once(' ')?;
            let tokens: usize = tokens_str.0.parse().ok()?;
            return Some(SnapshotEntry {
                filename,
                budget,
                tokens,
            });
        }
    }
    None
}

fn main() {
    let snap_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots");

    let mut entries: Vec<SnapshotEntry> = Vec::new();

    let Ok(dir) = std::fs::read_dir(&snap_dir) else {
        eprintln!("cannot read snapshot directory: {}", snap_dir.display());
        std::process::exit(1);
    };

    for entry in dir.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if !name.starts_with("snapshots__budget_") || !name.ends_with(".snap") {
            continue;
        }
        if let Some(snap) = parse_snapshot(&path) {
            entries.push(snap);
        }
    }

    if entries.is_empty() {
        eprintln!("no budget snapshots found in {}", snap_dir.display());
        std::process::exit(1);
    }

    // Compute utilization
    let utils: Vec<(f64, &SnapshotEntry)> = entries
        .iter()
        .map(|e| ((e.tokens as f64 / e.budget as f64).min(1.0), e))
        .collect();

    // Sort by utilization ascending (worst first)
    let mut sorted: Vec<(f64, &SnapshotEntry)> = utils;
    sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Print all
    println!("=== Budget Utilization (from snapshots) ===");
    println!();
    println!(
        "{:>6}  {:>6}  {:>6}  Snapshot",
        "Util%", "Budget", "Tokens"
    );
    for (util, entry) in &sorted {
        println!(
            "{:>5.1}%  {:>6}  {:>6}  {}",
            util * 100.0,
            entry.budget,
            entry.tokens,
            entry.filename,
        );
    }

    // Summary
    let n = sorted.len();
    let mean_util: f64 = sorted.iter().map(|(u, _)| u).sum::<f64>() / n as f64;
    let below_50 = sorted.iter().filter(|(u, _)| *u < 0.5).count();

    println!();
    println!("=== Summary ===");
    println!("Snapshots: {}", n);
    println!("Mean utilization: {:.1}%", mean_util * 100.0);
    println!(
        "Below 50%: {} ({:.1}%)",
        below_50,
        below_50 as f64 / n as f64 * 100.0
    );
}
