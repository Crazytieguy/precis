//! Clone test fixtures (skips already-cloned ones).
//!
//! Clones at a pinned revision and removes .git so fixtures are static snapshots.
//!
//! Usage: cargo run --bin clone_fixtures

use std::path::Path;
use std::process::Command;

macro_rules! with_fixtures {
    ($(($dir:expr, $url:expr, $rev:expr)),* $(,)?) => {
        const FIXTURES: &[(&str, &str, &str)] = &[$(($dir, $url, $rev)),*];
    };
}
include!("../../tests/data/fixtures.rs");

fn main() {
    let fixtures_dir = match std::env::var_os("PRECIS_FIXTURE_DIR") {
        Some(p) => std::path::PathBuf::from(p),
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"),
    };
    let mut cloned = 0;
    let mut skipped = 0;

    for &(dir, url, rev) in FIXTURES {
        let target = fixtures_dir.join(dir);
        if target.exists() {
            let pin = target.join(".precis-pin");
            match std::fs::read_to_string(&pin) {
                Err(_) => {
                    // Backfill pre-pin-tracking clones so they satisfy the staleness test.
                    std::fs::write(&pin, rev).expect("write .precis-pin");
                }
                Ok(existing) if existing.trim() != rev => {
                    eprintln!(
                        "  STALE: {} pinned at {} but fixtures.rs declares {} (rm -rf {} && re-run to update)",
                        dir,
                        existing.trim(),
                        rev,
                        target.display()
                    );
                }
                Ok(_) => {}
            }
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
            eprintln!("  FAILED: {dir} (removing partial clone)");
            let _ = std::fs::remove_dir_all(&target);
            continue;
        }
        std::fs::remove_dir_all(target.join(".git")).ok();
        std::fs::write(target.join(".precis-pin"), rev).expect("write .precis-pin");
        cloned += 1;
    }

    eprintln!("{} cloned, {} already present", cloned, skipped);
}
