//! Snapshot tests for the generic walker against the v0.2 active fixtures.
//!
//! Stage 6 covers folder/file structure only; language-aware content batches
//! land in Stage 7 and will enrich these snapshots organically.
//!
//! Skips silently when a fixture isn't cloned. Run `cargo run --bin
//! clone_fixtures` to populate `test/fixtures/`.

use std::path::PathBuf;

const FIXTURES: &[&str] = &["log", "anyhow", "mdbook"];
const BUDGETS: &[usize] = &[1500, 3000, 6000];

#[test]
fn fixture_snapshots() {
    let fixtures_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test/fixtures");
    for fixture in FIXTURES {
        let path = fixtures_root.join(fixture);
        if !path.exists() {
            eprintln!("skip: fixture `{fixture}` not cloned");
            continue;
        }
        for budget in BUDGETS {
            let rendered = precis::render(&[&path], *budget, None)
                .unwrap_or_else(|e| panic!("render({fixture} @ {budget}): {e}"));
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("snapshots/fixtures");
            settings.set_prepend_module_to_snapshot(false);
            settings.bind(|| {
                insta::assert_snapshot!(format!("{fixture}__{budget}"), rendered);
            });
        }
    }
}
