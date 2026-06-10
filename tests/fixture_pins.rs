//! Fixture-pin staleness check: every active fixture's `.precis-pin` file
//! must equal the SHA in `tests/data/fixtures.rs`. Catches the case where
//! someone re-pins a fixture in `fixtures.rs` without re-cloning.

use std::path::PathBuf;

macro_rules! with_fixtures {
    ($(($dir:expr, $url:expr, $rev:expr)),* $(,)?) => {
        const DECLARED_FIXTURES: &[(&str, &str, &str)] = &[$(($dir, $url, $rev)),*];
    };
}
include!("data/fixtures.rs");

#[test]
fn fixture_pins_match_declarations() {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut problems = Vec::new();
    for &(name, _url, expected) in DECLARED_FIXTURES {
        let dir = fixtures_dir.join(name);
        if !dir.exists() {
            continue; // Not all declared fixtures are cloned locally — fine.
        }
        let pin_path = dir.join(".precis-pin");
        let actual = match std::fs::read_to_string(&pin_path) {
            Ok(s) => s.trim().to_string(),
            Err(_) => {
                problems.push(format!(
                    "{name}: missing {} (re-run `cargo run --bin clone_fixtures`)",
                    pin_path.display()
                ));
                continue;
            }
        };
        if actual != expected {
            problems.push(format!(
                "{name}: pin {actual} != fixtures.rs {expected} (re-clone: rm -rf {} && cargo run --bin clone_fixtures)",
                dir.display()
            ));
        }
    }
    if !problems.is_empty() {
        panic!("Fixture pin mismatches:\n  {}", problems.join("\n  "));
    }
}
