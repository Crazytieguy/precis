//! Snapshot tests for the multi-walker against the v0.2 active fixtures.
//!
//! Missing fixtures **fail** the test — silent skips previously hid bugs
//! when worktrees didn't have the fixtures cloned. Run `cargo run --bin
//! clone_fixtures` to populate `tests/fixtures/`.

use std::path::PathBuf;

const FIXTURES: &[&str] = &["log", "anyhow", "mdbook", "otree"];
const BUDGETS: &[usize] = &[1500, 3000, 6000];

#[test]
fn snapshots_fixtures() {
    let fixtures_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for fixture in FIXTURES {
        let path = fixtures_root.join(fixture);
        assert!(
            path.exists(),
            "fixture `{fixture}` not present at {}; run `cargo run --bin clone_fixtures`",
            path.display()
        );
        for budget in BUDGETS {
            let rendered = precis::render(&[&path], *budget, None)
                .unwrap_or_else(|e| panic!("render({fixture} @ {budget}): {e}"));
            insta::with_settings!(
                {
                    snapshot_path => "snapshots/fixtures",
                    prepend_module_to_snapshot => false,
                },
                {
                    insta::assert_snapshot!(format!("{fixture}__{budget}"), rendered);
                }
            );
        }
    }
}
