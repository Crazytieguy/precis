//! Clone test fixtures (skips already-cloned ones).
//!
//! Clones at a pinned revision and removes .git so fixtures are static snapshots.
//!
//! Usage:
//!   cargo run --bin clone_fixtures              — clone regular test fixtures
//!   cargo run --bin clone_fixtures -- --perf    — clone large perf fixtures (shallow)
//!   cargo run --bin clone_fixtures -- --perf django — clone a single perf fixture

use std::path::Path;
use std::process::Command;

macro_rules! with_fixtures {
    ($(($dir:expr, $url:expr, $rev:expr)),* $(,)?) => {
        const FIXTURES: &[(&str, &str, &str)] = &[$(($dir, $url, $rev)),*];
    };
}
macro_rules! with_entries { ($($tt:tt)*) => {} }
include!("../../test/fixtures.rs");

macro_rules! with_perf_fixtures {
    ($(($dir:expr, $url:expr, $tag:expr)),* $(,)?) => {
        const PERF_FIXTURES: &[(&str, &str, &str)] = &[$(($dir, $url, $tag)),*];
    };
}
include!("../../test/perf_fixtures.rs");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let perf = args.iter().any(|a| a == "--perf");

    if perf {
        let filter = args
            .iter()
            .skip_while(|a| *a != "--perf")
            .nth(1)
            .filter(|a| !a.starts_with('-'));
        clone_perf(filter.map(|s| s.as_str()));
    } else {
        clone_regular();
    }
}

fn clone_regular() {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/fixtures");
    let mut cloned = 0;
    let mut skipped = 0;

    for &(dir, url, rev) in FIXTURES {
        let target = fixtures_dir.join(dir);
        if target.exists() {
            skipped += 1;
            continue;
        }
        eprintln!("cloning {} @ {} ...", dir, rev);
        let ok = Command::new("git")
            .args(["clone", url])
            .arg(&target)
            .status()
            .expect("failed to run git")
            .success()
            && Command::new("git")
                .args(["checkout", rev])
                .current_dir(&target)
                .status()
                .expect("failed to run git")
                .success();
        if !ok {
            eprintln!("  FAILED: {}", dir);
            continue;
        }
        std::fs::remove_dir_all(target.join(".git")).ok();
        cloned += 1;
    }

    eprintln!("{} cloned, {} already present", cloned, skipped);
}

fn clone_perf(filter: Option<&str>) {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/perf-fixtures");
    std::fs::create_dir_all(&fixtures_dir).expect("failed to create perf-fixtures dir");

    let mut cloned = 0;
    let mut skipped = 0;
    let mut failed = 0;

    for &(dir, url, tag) in PERF_FIXTURES {
        if let Some(name) = filter {
            if dir != name {
                continue;
            }
        }
        let target = fixtures_dir.join(dir);
        if target.exists() {
            skipped += 1;
            continue;
        }
        eprintln!("cloning {} @ {} (shallow) ...", dir, tag);
        let ok = Command::new("git")
            .args(["clone", "--depth", "1", "--branch", tag, url])
            .arg(&target)
            .status()
            .expect("failed to run git")
            .success();
        if !ok {
            eprintln!("  FAILED: {}", dir);
            failed += 1;
            continue;
        }
        std::fs::remove_dir_all(target.join(".git")).ok();
        cloned += 1;
    }

    eprintln!("{} cloned, {} already present, {} failed", cloned, skipped, failed);
    if failed > 0 {
        std::process::exit(1);
    }
}
