//! Regression baselines + divergence regression + rendered-output
//! regression, unified under one `cargo t` flow.
//!
//! Three artifact classes, one unified regen flag:
//!
//! - **Schedule TOML** (`tests/snapshots/schedule/<fixture>__<budget>.toml`):
//!   machine-readable, per-(fixture,budget). Produced by `render_schedule`;
//!   compared by byte-equality to the on-disk file. Multi-budget because
//!   the scheduler is budget-sensitive — a single-10k regression would
//!   miss low-budget shifts.
//!
//! - **Divergence report** (`tests/divergence/<fixture>__<budget>.md`):
//!   human-readable markdown, one per (fixture,budget) for every fixture
//!   that has a frozen NS (`tests/north-stars/<fixture>.toml`). Line 1
//!   is echoed to stdout so a calibration iteration can eyeball all
//!   scores without opening files. Fixtures without an NS yield no
//!   report and the test passes vacuously.
//!
//! - **Rendered snapshot** (`tests/snapshots/rendered/<fixture>__3000.snap`):
//!   insta-based, at the default `precis` budget. Covers the span-to-
//!   rendered-output translation that atoms-only metrics ignore.
//!
//! Unified regen: `UPDATE_BASELINES=1 cargo t` accepts all three artifact
//! types simultaneously (the wrapper also sets `INSTA_UPDATE=always`
//! internally). Run it from the project root.
//!
//! Per-(fixture,budget) test fns are generated via `paste` so nextest's
//! test-name substring filter resolves — e.g. `cargo t log__3000` targets
//! exactly the three regressions for log at 3000 (schedule + divergence +
//! rendered if 3000 is the rendered-snapshot budget).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use precis::{
    divergence::generate_divergence_report, render as render_precis, render_schedule,
    schema::load_ns_checked,
};

const RENDERED_BUDGET: usize = 3000;

const SCHEDULE_DIR: &str = "tests/snapshots/schedule";
const DIVERGENCE_DIR: &str = "tests/divergence";

// ---- macros to generate one test fn per (fixture, budget) ---------------

macro_rules! per_fixture_tests {
    ($fixture:ident) => {
        paste::paste! {
            // Schedule TOML regression per budget.
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_snapshot_ $fixture __1500>]() { check_schedule_snapshot(stringify!($fixture), 1500); }
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_snapshot_ $fixture __3000>]() { check_schedule_snapshot(stringify!($fixture), 3000); }
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_snapshot_ $fixture __6000>]() { check_schedule_snapshot(stringify!($fixture), 6000); }
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_snapshot_ $fixture __10000>]() { check_schedule_snapshot(stringify!($fixture), 10000); }

            // Divergence report regression per budget.
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_divergence_ $fixture __1500>]() { check_divergence_report(stringify!($fixture), 1500); }
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_divergence_ $fixture __3000>]() { check_divergence_report(stringify!($fixture), 3000); }
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_divergence_ $fixture __6000>]() { check_divergence_report(stringify!($fixture), 6000); }
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_divergence_ $fixture __10000>]() { check_divergence_report(stringify!($fixture), 10000); }

            // Rendered snapshot at the default budget.
            #[test] #[allow(non_snake_case)]
            fn [<schedule_order_rendered_ $fixture __3000>]() { check_rendered_snapshot(stringify!($fixture)); }
        }
    };
}

per_fixture_tests!(log);
per_fixture_tests!(anyhow);
per_fixture_tests!(mdbook);
per_fixture_tests!(otree);

// ---- helpers ------------------------------------------------------------

fn manifest_dir() -> &'static Path {
    static CELL: OnceLock<PathBuf> = OnceLock::new();
    CELL.get_or_init(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn fixture_path(fixture: &str) -> PathBuf {
    manifest_dir().join("tests/fixtures").join(fixture)
}

fn schedule_path(fixture: &str, budget: usize) -> PathBuf {
    manifest_dir()
        .join(SCHEDULE_DIR)
        .join(format!("{fixture}__{budget}.toml"))
}

fn divergence_path(fixture: &str, budget: usize) -> PathBuf {
    manifest_dir()
        .join(DIVERGENCE_DIR)
        .join(format!("{fixture}__{budget}.md"))
}

fn ns_path(fixture: &str) -> PathBuf {
    manifest_dir().join(format!("tests/north-stars/{fixture}.toml"))
}

fn update_baselines() -> bool {
    std::env::var_os("UPDATE_BASELINES").is_some()
}

/// Require a fixture checkout. Missing fixtures fail — silent skips used
/// to hide bugs when worktrees didn't have everything cloned.
fn require_fixture(fixture: &str) -> PathBuf {
    let p = fixture_path(fixture);
    assert!(
        p.exists(),
        "fixture `{fixture}` not present at {}; run `cargo run --bin clone_fixtures`",
        p.display()
    );
    p
}

// ---- schedule TOML ------------------------------------------------------

fn check_schedule_snapshot(fixture: &str, budget: usize) {
    let fixture_dir = require_fixture(fixture);
    let schedule = render_schedule(&[&fixture_dir], budget)
        .unwrap_or_else(|e| panic!("render_schedule({fixture} @ {budget}): {e}"));
    let serialized = toml::to_string(&schedule)
        .unwrap_or_else(|e| panic!("serializing schedule({fixture} @ {budget}): {e}"));
    compare_or_update(
        "schedule TOML",
        &schedule_path(fixture, budget),
        serialized.as_bytes(),
    );
}

// ---- divergence report --------------------------------------------------

fn check_divergence_report(fixture: &str, budget: usize) {
    let fixture_dir = require_fixture(fixture);
    let ns_toml = ns_path(fixture);
    if !ns_toml.exists() {
        // No NS for this fixture yet — the regression passes vacuously.
        // An NS-less fixture can still produce schedule + rendered
        // snapshots; divergence just requires a frozen NS.
        return;
    }
    let ns = load_ns_checked(&ns_toml, &fixture_dir)
        .unwrap_or_else(|e| panic!("load_ns_checked({fixture}): {e}"));
    let schedule = render_schedule(&[&fixture_dir], budget)
        .unwrap_or_else(|e| panic!("render_schedule({fixture} @ {budget}): {e}"));
    let report = generate_divergence_report(&ns, &schedule, &fixture_dir)
        .unwrap_or_else(|e| panic!("generate_divergence_report({fixture} @ {budget}): {e}"));

    if let Some(first_line) = report.lines().next() {
        println!("{fixture}__{budget}: {first_line}");
    }

    compare_or_update(
        "divergence report",
        &divergence_path(fixture, budget),
        report.as_bytes(),
    );
}

// ---- rendered snapshot --------------------------------------------------

fn check_rendered_snapshot(fixture: &str) {
    let fixture_dir = require_fixture(fixture);
    let rendered = render_precis(&[&fixture_dir], RENDERED_BUDGET, None)
        .unwrap_or_else(|e| panic!("render({fixture} @ {RENDERED_BUDGET}): {e}"));
    if update_baselines() {
        // Let insta regenerate by setting its env var for this assertion.
        unsafe {
            std::env::set_var("INSTA_UPDATE", "always");
        }
    }
    insta::with_settings!(
        {
            snapshot_path => "snapshots/rendered",
            prepend_module_to_snapshot => false,
        },
        {
            insta::assert_snapshot!(format!("{fixture}__{RENDERED_BUDGET}"), rendered);
        }
    );
}

// ---- comparison + regen -------------------------------------------------

fn compare_or_update(kind: &str, path: &Path, actual: &[u8]) {
    let existing = fs::read(path).ok();
    let matches = existing.as_deref().map(|b| b == actual).unwrap_or(false);
    if matches {
        return;
    }
    if update_baselines() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("mkdir {}: {e}", parent.display()));
        }
        fs::write(path, actual).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        return;
    }
    // Write .new sidecar so a diff is inspectable before accepting.
    let new_path = path.with_extension(format!(
        "{}.new",
        path.extension().and_then(|s| s.to_str()).unwrap_or("")
    ));
    if let Some(parent) = new_path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|e| panic!("mkdir {}: {e}", parent.display()));
    }
    fs::write(&new_path, actual).unwrap_or_else(|e| panic!("write {}: {e}", new_path.display()));
    panic!(
        "{kind} mismatch at {}\n  expected: {}\n  wrote new output to: {}\n  review with `git diff --no-index {} {}` or regenerate with `UPDATE_BASELINES=1 cargo t`",
        path.display(),
        path.display(),
        new_path.display(),
        path.display(),
        new_path.display(),
    );
}

// ---- invariants ---------------------------------------------------------

#[test]
fn schedule_order_ns_pins_match_fixture_pins() {
    let ns_dir = manifest_dir().join("tests/north-stars");
    let Ok(read_dir) = fs::read_dir(&ns_dir) else {
        return;
    };
    let mut problems = Vec::new();
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        let ns = match precis::schema::load_ns(&path) {
            Ok(ns) => ns,
            Err(e) => {
                problems.push(format!("{}: {e}", path.display()));
                continue;
            }
        };
        let fixture_root = fixture_path(&ns.fixture);
        if !fixture_root.exists() {
            // Fixture not cloned locally — can't check pin, can't verify.
            // `reviews_fresh_fixture_pins_match_declarations` handles the
            // declared-vs-cloned side.
            continue;
        }
        let pin_path = fixture_root.join(".precis-pin");
        let pin = match fs::read_to_string(&pin_path) {
            Ok(s) => s.trim().to_string(),
            Err(e) => {
                problems.push(format!("{}: reading pin: {e}", path.display()));
                continue;
            }
        };
        if ns.revision_pin != pin {
            problems.push(format!(
                "{}: NS pins {} but fixture pins {} — re-author NS or re-clone fixture",
                path.display(),
                ns.revision_pin,
                pin
            ));
        }
    }
    if !problems.is_empty() {
        panic!("NS pin mismatches:\n  {}", problems.join("\n  "));
    }
}
