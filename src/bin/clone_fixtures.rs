//! Clone all test fixtures (skips already-cloned ones).
//!
//! Clones at a pinned commit and removes .git so fixtures are static snapshots.
//!
//! Usage: cargo run --bin clone_fixtures

use std::path::Path;
use std::process::Command;

macro_rules! with_fixtures {
    ($(($dir:expr, $url:expr, $rev:expr)),* $(,)?) => {
        const FIXTURES: &[(&str, &str, &str)] = &[$(($dir, $url, $rev)),*];
    };
}
macro_rules! with_entries { ($($tt:tt)*) => {} } // unused here
include!("../../test/fixtures.rs");

fn main() {
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
        // Remove .git to make it a static snapshot
        std::fs::remove_dir_all(target.join(".git")).ok();
        cloned += 1;
    }

    eprintln!("{} cloned, {} already present", cloned, skipped);
}
