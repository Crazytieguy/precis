//! Per-fixture regression baselines, one test per fixture in
//! `tests/data/fixtures.rs`. Each runs the walker once at `SCHEDULE_BUDGET`;
//! scheduler prefix-monotonicity (see `docs/design-notes.md`) makes every
//! smaller budget a prefix of that schedule plus a head of the next batch.
//!
//! - Training fixtures commit `tests/rendered/<fixture>.txt` (the output at
//!   `RENDERED_BUDGET`) and `tests/divergence/<fixture>.md` (the report
//!   against `tests/north-stars/<fixture>.toml`).
//! - Validation fixtures commit only the score headline to
//!   `tests/validation/<fixture>.md`. They are held out from calibration;
//!   see the `iterate-divergence` skill.
//!
//! A mismatch writes `<file>.new` next to the baseline and fails.
//! `UPDATE_BASELINES=1 cargo t` rewrites the baselines instead and passes.

use std::fs;
use std::path::{Path, PathBuf};

use precis::divergence::{self, Schedule, render_schedule, render_with_schedule};
use precis::ns_loader::load_ns_checked;

/// Walker budget for the scored schedule. Matches the NS cap.
const SCHEDULE_BUDGET: usize = 10_000;
/// The CLI's default budget.
const RENDERED_BUDGET: usize = 3_000;

macro_rules! fixtures {
    (
        training { $($training:ident $training_url:literal $training_rev:literal,)* }
        validation { $($validation:ident $validation_url:literal $validation_rev:literal,)* }
    ) => {
        paste::paste! {
            $(
                #[test]
                fn [<fixture_baselines_ $training>]() {
                    check_training_fixture(stringify!($training), $training_rev);
                }
            )*
            $(
                #[test]
                fn [<fixture_baselines_validation_ $validation>]() {
                    check_validation_fixture(stringify!($validation), $validation_rev);
                }
            )*
        }
    };
}
include!("../data/fixtures.rs");

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn check_training_fixture(name: &str, rev: &str) {
    let (fixture, fixture_dir, schedule) = run_fixture(name, rev);
    let rendered = render_with_schedule(&schedule, RENDERED_BUDGET);
    compare_or_update(
        &repo_path(&format!("tests/rendered/{fixture}.txt")),
        rendered.as_bytes(),
    );

    let ns = load_ns_checked(
        &repo_path(&format!("tests/north-stars/{fixture}.toml")),
        &fixture_dir,
    )
    .unwrap_or_else(|e| panic!("load_ns_checked({fixture}): {e}"));
    let (report, scores) = divergence::generate_divergence_report(&ns, &schedule, &fixture_dir)
        .unwrap_or_else(|e| panic!("generate_divergence_report({fixture}): {e}"));
    assert_scores_sane(&fixture, &scores);
    compare_or_update(
        &repo_path(&format!("tests/divergence/{fixture}.md")),
        report.as_bytes(),
    );
}

fn check_validation_fixture(name: &str, rev: &str) {
    let (fixture, fixture_dir, schedule) = run_fixture(name, rev);
    let ns = load_ns_checked(
        &repo_path(&format!("tests/north-stars/{fixture}.toml")),
        &fixture_dir,
    )
    .unwrap_or_else(|e| panic!("load_ns_checked({fixture}): {e}"));
    let (_, scores) = divergence::generate_divergence_report(&ns, &schedule, &fixture_dir)
        .unwrap_or_else(|e| panic!("generate_divergence_report({fixture}): {e}"));
    assert_scores_sane(&fixture, &scores);
    compare_or_update(
        &repo_path(&format!("tests/validation/{fixture}.md")),
        format!("{}\n", scores.headline()).as_bytes(),
    );
}

/// Checks the clone is at its declared revision, then schedules it.
fn run_fixture(name: &str, rev: &str) -> (String, PathBuf, Schedule) {
    let fixture = name.replace('_', "-");
    let fixture_dir = repo_path(&format!("tests/fixtures/{fixture}"));
    let pin = fs::read_to_string(fixture_dir.join(precis::fs_util::PRECIS_PIN_FILE))
        .unwrap_or_else(|e| {
            panic!("fixture `{fixture}` has no pin ({e}); run `cargo run --example clone_fixtures`")
        });
    assert_eq!(
        pin.trim(),
        rev,
        "fixture `{fixture}` is not at its declared revision; \
         rm -rf {} && cargo run --example clone_fixtures",
        fixture_dir.display()
    );
    let schedule = render_schedule(&fixture_dir, SCHEDULE_BUDGET)
        .unwrap_or_else(|e| panic!("render_schedule({fixture}): {e}"));
    (fixture, fixture_dir, schedule)
}

/// Every fixture scores above zero; zero means the NS and walker atoms
/// never matched, which has been a path bug, not a bad walker.
fn assert_scores_sane(fixture: &str, scores: &divergence::Scores) {
    assert!(
        scores.primary().score > 0.0,
        "{fixture}: `{}` — likely a path or canonicalization bug, not a real score",
        scores.headline()
    );
}

fn compare_or_update(path: &Path, actual: &[u8]) {
    let new_path = PathBuf::from(format!("{}.new", path.display()));
    let _ = fs::remove_file(&new_path);
    if fs::read(path).is_ok_and(|expected| expected == actual) {
        return;
    }
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    if std::env::var_os("UPDATE_BASELINES").is_some() {
        fs::write(path, actual).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        return;
    }
    fs::write(&new_path, actual).unwrap_or_else(|e| panic!("write {}: {e}", new_path.display()));
    panic!(
        "baseline mismatch at {}; review with `git diff --no-index {} {}` \
         or regenerate with `UPDATE_BASELINES=1 cargo t`",
        path.display(),
        path.display(),
        new_path.display(),
    );
}
