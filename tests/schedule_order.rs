//! Regression baselines: schedule TOML + rendered snapshot + divergence
//! report. One artifact per fixture; the scheduler's prefix-monotonicity
//! (see `docs/design-notes.md`) means sub-budget behavior is a prefix of
//! the T_max schedule, so per-budget snapshots are redundant.
//!
//! Three artifact classes, all regeneratable via `UPDATE_BASELINES=1`:
//!
//! - **Schedule TOML** — `tests/snapshots/schedule/<fixture>.toml`,
//!   produced by `render_schedule` at `SCHEDULE_BUDGET` (the cap). Byte-
//!   equal comparison to on-disk file.
//!
//! - **Rendered snapshot** — `tests/snapshots/rendered/<fixture>.snap`
//!   via insta at `RENDERED_BUDGET` (user-facing default), a human-
//!   readable eyeball of the span-to-rendered-output translation that
//!   atom metrics alone don't cover.
//!
//! - **Divergence report** — `tests/divergence/<fixture>.md`, one per
//!   fixture with a frozen NS (`tests/north-stars/<fixture>.toml`).
//!   Line 1 is echoed to stdout so calibration iterations see all
//!   scores without opening files. Fixtures without an NS yield no
//!   report and the test passes vacuously.
//!
//! Unified regen: `UPDATE_BASELINES=1 cargo t` accepts all three artifact
//! types (sets `INSTA_UPDATE=always` internally for the rendered snapshot).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use precis::{
    divergence::generate_divergence_report, ns_loader::load_ns_checked, render as render_precis,
    render_schedule,
};

/// Walker budget for the canonical schedule snapshot. Matches the NS cap.
const SCHEDULE_BUDGET: usize = 10_000;
/// User-facing "default precis budget" for the rendered eyeball snapshot.
const RENDERED_BUDGET: usize = 3_000;

const SCHEDULE_DIR: &str = "tests/snapshots/schedule";
const DIVERGENCE_DIR: &str = "tests/divergence";

// ---- test-fn generation ------------------------------------------------

macro_rules! per_fixture_tests {
    ($fixture:ident) => {
        per_fixture_tests!($fixture, stringify!($fixture));
    };
    ($fixture:ident, $name:expr) => {
        paste::paste! {
            #[test]
            fn [<schedule_order_snapshot_ $fixture>]() {
                check_schedule_snapshot($name);
            }
            #[test]
            fn [<schedule_order_divergence_ $fixture>]() {
                check_divergence_report($name);
            }
            #[test]
            fn [<schedule_order_rendered_ $fixture>]() {
                check_rendered_snapshot($name);
            }
        }
    };
}

per_fixture_tests!(log);
per_fixture_tests!(anyhow);
per_fixture_tests!(mdbook);
per_fixture_tests!(otree);
per_fixture_tests!(mitt);
per_fixture_tests!(ts_pattern, "ts-pattern");
per_fixture_tests!(cmdk);
per_fixture_tests!(vaul);
per_fixture_tests!(ky);
per_fixture_tests!(superstruct);

// ---- paths -------------------------------------------------------------

fn manifest_dir() -> &'static Path {
    static CELL: OnceLock<PathBuf> = OnceLock::new();
    CELL.get_or_init(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn fixture_path(fixture: &str) -> PathBuf {
    manifest_dir().join("tests/fixtures").join(fixture)
}

fn schedule_path(fixture: &str) -> PathBuf {
    manifest_dir()
        .join(SCHEDULE_DIR)
        .join(format!("{fixture}.toml"))
}

fn divergence_path(fixture: &str) -> PathBuf {
    manifest_dir()
        .join(DIVERGENCE_DIR)
        .join(format!("{fixture}.md"))
}

fn ns_path(fixture: &str) -> PathBuf {
    manifest_dir().join(format!("tests/north-stars/{fixture}.toml"))
}

fn update_baselines() -> bool {
    std::env::var_os("UPDATE_BASELINES").is_some()
}

fn require_fixture(fixture: &str) -> PathBuf {
    let p = fixture_path(fixture);
    assert!(
        p.exists(),
        "fixture `{fixture}` not present at {}; run `cargo run --bin clone_fixtures`",
        p.display()
    );
    p
}

// ---- schedule TOML -----------------------------------------------------

fn check_schedule_snapshot(fixture: &str) {
    let fixture_dir = require_fixture(fixture);
    let schedule = render_schedule(&[&fixture_dir], SCHEDULE_BUDGET)
        .unwrap_or_else(|e| panic!("render_schedule({fixture}): {e}"));
    let serialized = toml::to_string(&schedule)
        .unwrap_or_else(|e| panic!("serializing schedule({fixture}): {e}"));
    compare_or_update(
        "schedule TOML",
        &schedule_path(fixture),
        serialized.as_bytes(),
    );
}

// ---- divergence report -------------------------------------------------

fn check_divergence_report(fixture: &str) {
    let fixture_dir = require_fixture(fixture);
    let ns_toml = ns_path(fixture);
    if !ns_toml.exists() {
        return; // no NS for this fixture yet
    }
    let ns = load_ns_checked(&ns_toml, &fixture_dir)
        .unwrap_or_else(|e| panic!("load_ns_checked({fixture}): {e}"));
    let schedule = render_schedule(&[&fixture_dir], SCHEDULE_BUDGET)
        .unwrap_or_else(|e| panic!("render_schedule({fixture}): {e}"));
    let report = generate_divergence_report(&ns, &schedule, &fixture_dir)
        .unwrap_or_else(|e| panic!("generate_divergence_report({fixture}): {e}"));

    if let Some(first_line) = report.lines().next() {
        println!("{fixture}: {first_line}");
    }

    compare_or_update(
        "divergence report",
        &divergence_path(fixture),
        report.as_bytes(),
    );
}

// ---- rendered snapshot -------------------------------------------------

fn check_rendered_snapshot(fixture: &str) {
    let fixture_dir = require_fixture(fixture);
    let rendered = render_precis(&[&fixture_dir], RENDERED_BUDGET, None)
        .unwrap_or_else(|e| panic!("render({fixture}): {e}"));
    if update_baselines() {
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
            insta::assert_snapshot!(format!("{fixture}"), rendered);
        }
    );
}

// ---- comparison + regen ------------------------------------------------

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

// ---- invariants --------------------------------------------------------

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
        let ns = match precis::ns_loader::load_ns(&path) {
            Ok(ns) => ns,
            Err(e) => {
                problems.push(format!("{}: {e}", path.display()));
                continue;
            }
        };
        let fixture_root = fixture_path(&ns.fixture);
        if !fixture_root.exists() {
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
