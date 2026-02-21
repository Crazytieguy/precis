//! Clone all test fixtures (skips already-cloned ones).
//!
//! Usage: cargo run --bin clone_fixtures

use std::path::Path;
use std::process::Command;

macro_rules! with_fixtures {
    ($(($dir:expr, $url:expr)),* $(,)?) => {
        const FIXTURES: &[(&str, &str)] = &[$(($dir, $url)),*];
    };
}
macro_rules! with_entries { ($($tt:tt)*) => {} } // unused here
include!("../../test/fixtures.rs");

fn main() {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/fixtures");

    let mut cloned = 0;
    let mut skipped = 0;

    for &(dir, url) in FIXTURES {
        let target = fixtures_dir.join(dir);
        if target.exists() {
            skipped += 1;
            continue;
        }
        eprintln!("cloning {} ...", dir);
        let status = Command::new("git")
            .args(["clone", "--depth", "1", url])
            .arg(&target)
            .status()
            .expect("failed to run git");
        if !status.success() {
            eprintln!("  FAILED: {}", dir);
        } else {
            cloned += 1;
        }
    }

    eprintln!("{} cloned, {} already present", cloned, skipped);
}
