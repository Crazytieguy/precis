//! Regression baselines: rendered snapshot + divergence report. One
//! artifact per fixture; the scheduler's prefix-monotonicity (see
//! `docs/design-notes.md`) means sub-budget behavior is a prefix of the
//! T_max schedule, so per-budget snapshots are redundant.
//!
//! Three artifact classes, all regeneratable via `UPDATE_BASELINES=1`:
//!
//! - **Rendered snapshot** — `tests/snapshots/rendered/<fixture>.snap`
//!   via insta at `RENDERED_BUDGET` (user-facing default), a human-
//!   readable eyeball of the span-to-rendered-output translation that
//!   atom metrics alone don't cover.
//!
//! - **Divergence report** — `tests/divergence/<fixture>.md`, one per
//!   training fixture with a frozen NS (`tests/north-stars/<fixture>.toml`).
//!   Survey across fixtures with `head -1 tests/divergence/*.md` (each
//!   report's first line is the `Score(3000)=… I=… C=… ns_rows≤3K=…
//!   grid(…)=…` headline); `bash scripts/grid-means.sh` prints the
//!   corpus mean at every grid budget.
//!
//! - **Validation score** — `tests/validation/<fixture>.md`, one per
//!   validation fixture. Contains *only* the `Score(3000)=…` headline
//!   line; no rendered snapshot, no per-row diff.
//!   Validation fixtures are sampled from the GitHub language
//!   distribution and held out: they exist to catch regressions on
//!   real-world codebases the calibration loop has never seen. Walker /
//!   value iteration must not target them — read the `iterate-divergence`
//!   skill for the discipline.
//!
//! Unified regen: `UPDATE_BASELINES=1 cargo t` accepts all three artifact
//! types (sets `INSTA_UPDATE=always` internally for the rendered snapshot).
//! A successful regen run *is* a passing test run — the on-disk baselines
//! and the freshly-computed `actual` are byte-identical when it returns,
//! so a follow-up plain `cargo t` is redundant.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use precis::{
    Schedule, divergence, ns_loader::load_ns_checked, render_schedule, render_with_schedule,
};

/// Walker budget for the scored schedule. Matches the NS cap.
const SCHEDULE_BUDGET: usize = 10_000;
/// User-facing "default precis budget" for the rendered eyeball snapshot.
const RENDERED_BUDGET: usize = 3_000;

const DIVERGENCE_DIR: &str = "tests/divergence";
const VALIDATION_DIR: &str = "tests/validation";
const NS_DIR: &str = "tests/north-stars";

// ---- test-fn generation ------------------------------------------------

macro_rules! per_fixture_tests {
    ($fixture:ident) => {
        per_fixture_tests!($fixture, stringify!($fixture));
    };
    ($fixture:ident, $name:expr) => {
        paste::paste! {
            #[test]
            fn [<fixture_baselines_ $fixture>]() {
                check_fixture_baselines($name);
            }
        }
    };
}

/// Validation-tier counterpart to [`per_fixture_tests!`]: same walker
/// pass, but only the `Scores::headline()` line is committed —
/// fixtures registered here are held out from calibration.
macro_rules! per_validation_fixture_tests {
    ($fixture:ident) => {
        per_validation_fixture_tests!($fixture, stringify!($fixture));
    };
    ($fixture:ident, $name:expr) => {
        paste::paste! {
            #[test]
            fn [<fixture_baselines_validation_ $fixture>]() {
                check_validation_fixture_baselines($name);
            }
        }
    };
}

per_fixture_tests!(log);
per_fixture_tests!(anyhow);
per_fixture_tests!(thiserror);
per_fixture_tests!(mdbook);
per_fixture_tests!(otree);
per_fixture_tests!(sps);
per_fixture_tests!(mitt);
per_fixture_tests!(ts_pattern, "ts-pattern");
per_fixture_tests!(cmdk);
per_fixture_tests!(vaul);
per_fixture_tests!(ky);
per_fixture_tests!(superstruct);
per_fixture_tests!(enclosed);
per_fixture_tests!(toasty);
per_fixture_tests!(d2ts);
per_fixture_tests!(sds);
per_fixture_tests!(tomli);
per_fixture_tests!(xxhash);
per_fixture_tests!(go_multierror, "go-multierror");
per_fixture_tests!(pluggy);
per_fixture_tests!(commander);
per_fixture_tests!(tock);
per_fixture_tests!(typeguard);
per_fixture_tests!(bareiron);
per_fixture_tests!(htmy);
per_fixture_tests!(mcphost);
per_fixture_tests!(krep);
per_fixture_tests!(microbootstrap);
per_fixture_tests!(semver);
per_fixture_tests!(nano_vllm, "nano-vllm");
per_fixture_tests!(chronos_forecasting, "chronos-forecasting");
per_fixture_tests!(py3xui);
per_fixture_tests!(swarm);
per_fixture_tests!(neco);
per_fixture_tests!(xlstm);
per_fixture_tests!(peepdb);
per_fixture_tests!(soluna);
per_fixture_tests!(sqlite_vec, "sqlite-vec");
per_fixture_tests!(cobra);
per_fixture_tests!(gin);
per_fixture_tests!(lo);
per_fixture_tests!(migrate);
per_fixture_tests!(bubbletea);
per_fixture_tests!(p_queue, "p-queue");
per_fixture_tests!(express);
per_fixture_tests!(axios);
per_fixture_tests!(chalk);
per_fixture_tests!(debug);
per_fixture_tests!(flask);
per_fixture_tests!(requests);
per_fixture_tests!(rich);
per_fixture_tests!(click);
per_fixture_tests!(middleclass);
per_fixture_tests!(mkcert);
per_fixture_tests!(act);
per_fixture_tests!(beszel);
per_fixture_tests!(linkwarden);
per_fixture_tests!(json_server, "json-server");
per_fixture_tests!(beets);
per_fixture_tests!(posting);
per_fixture_tests!(linkding);
per_fixture_tests!(htop);
per_fixture_tests!(jq);
per_fixture_tests!(svgo);
per_fixture_tests!(dockly);
per_fixture_tests!(audiobookshelf);
per_fixture_tests!(hyperfine);
per_fixture_tests!(vite);
per_fixture_tests!(monaco_editor, "monaco-editor");
per_fixture_tests!(tinyusb);
per_fixture_tests!(chibicc);

// ---- validation fixtures (held out — see module doc) ------------------

// Python
per_validation_fixture_tests!(aiogram);
per_validation_fixture_tests!(mkdocs);
per_validation_fixture_tests!(healthchecks);
per_validation_fixture_tests!(statsmodels);
per_validation_fixture_tests!(xonsh);
per_validation_fixture_tests!(httpie);
per_validation_fixture_tests!(crawlee_python, "crawlee-python");
// TypeScript
per_validation_fixture_tests!(pretty_ts_errors, "pretty-ts-errors");
per_validation_fixture_tests!(drizzle_orm, "drizzle-orm");
per_validation_fixture_tests!(preact_signals, "preact-signals");
per_validation_fixture_tests!(clack);
per_validation_fixture_tests!(excalidraw);
// JavaScript
per_validation_fixture_tests!(marked);
per_validation_fixture_tests!(handlebars);
per_validation_fixture_tests!(octotree);
per_validation_fixture_tests!(rough_viz, "rough-viz");
// Go
per_validation_fixture_tests!(helm);
per_validation_fixture_tests!(nats_server, "nats-server");
per_validation_fixture_tests!(rqlite);
// C
per_validation_fixture_tests!(mongoose);
per_validation_fixture_tests!(sameboy);
// Rust
per_validation_fixture_tests!(xh);

// ---- paths -------------------------------------------------------------

fn manifest_dir() -> &'static Path {
    static CELL: OnceLock<PathBuf> = OnceLock::new();
    CELL.get_or_init(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn fixture_path(fixture: &str) -> PathBuf {
    manifest_dir().join("tests/fixtures").join(fixture)
}

fn divergence_path(fixture: &str) -> PathBuf {
    manifest_dir()
        .join(DIVERGENCE_DIR)
        .join(format!("{fixture}.md"))
}

fn ns_path(fixture: &str) -> PathBuf {
    manifest_dir().join(NS_DIR).join(format!("{fixture}.toml"))
}

fn validation_path(fixture: &str) -> PathBuf {
    manifest_dir()
        .join(VALIDATION_DIR)
        .join(format!("{fixture}.md"))
}

fn update_baselines() -> bool {
    std::env::var_os("UPDATE_BASELINES").is_some()
}

fn require_fixture(fixture: &str) -> PathBuf {
    let p = fixture_path(fixture);
    assert!(
        p.exists(),
        "fixture `{fixture}` not present at {}; run `cargo run --example clone_fixtures`",
        p.display()
    );
    p
}

// ---- per-fixture baselines ---------------------------------------------

fn check_fixture_baselines(fixture: &str) {
    let fixture_dir = require_fixture(fixture);

    let schedule = render_schedule(&[&fixture_dir], SCHEDULE_BUDGET)
        .unwrap_or_else(|e| panic!("render_schedule({fixture}): {e}"));
    check_rendered(fixture, &fixture_dir, &schedule);
    check_divergence(fixture, &fixture_dir, &schedule);
}

fn check_divergence(fixture: &str, fixture_dir: &Path, schedule: &Schedule) {
    let ns_toml = ns_path(fixture);
    if !ns_toml.exists() {
        return;
    }
    let ns = load_ns_checked(&ns_toml, fixture_dir)
        .unwrap_or_else(|e| panic!("load_ns_checked({fixture}): {e}"));
    let (report, scores) = divergence::generate_divergence_report(&ns, schedule, fixture_dir)
        .unwrap_or_else(|e| panic!("generate_divergence_report({fixture}): {e}"));

    if let Some(first_line) = report.lines().next() {
        println!("{fixture}: {first_line}");
    }
    assert_scores_sane(fixture, &scores);

    compare_or_update(
        "divergence report",
        &divergence_path(fixture),
        report.as_bytes(),
    );
}

fn check_validation_fixture_baselines(fixture: &str) {
    let fixture_dir = require_fixture(fixture);
    let schedule = render_schedule(&[&fixture_dir], SCHEDULE_BUDGET)
        .unwrap_or_else(|e| panic!("render_schedule({fixture}): {e}"));

    let ns_toml = ns_path(fixture);
    assert!(
        ns_toml.exists(),
        "validation fixture `{fixture}` has no NS at {}; author one before registering",
        ns_toml.display(),
    );
    let ns = load_ns_checked(&ns_toml, &fixture_dir)
        .unwrap_or_else(|e| panic!("load_ns_checked({fixture}): {e}"));
    let scores = divergence::score(&ns, &schedule, &fixture_dir)
        .unwrap_or_else(|e| panic!("score({fixture}): {e}"));

    let headline = format!("{}\n", scores.headline());
    println!("{fixture} (validation): {}", scores.headline());
    assert_scores_sane(fixture, &scores);

    compare_or_update(
        "validation score",
        &validation_path(fixture),
        headline.as_bytes(),
    );
}

fn check_rendered(fixture: &str, fixture_dir: &Path, schedule: &Schedule) {
    let rendered = render_with_schedule(schedule, fixture_dir, RENDERED_BUDGET)
        .unwrap_or_else(|e| panic!("render_with_schedule({fixture}): {e}"));
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

/// A zero primary score with no reached/partial rows has historically
/// meant an infrastructure failure (fixture-path canonicalization), not
/// a bad walker — refuse to bake it into a baseline. `ALLOW_ZERO_SCORE=1`
/// overrides for a genuinely degenerate fixture.
fn assert_scores_sane(fixture: &str, scores: &divergence::Scores) {
    if std::env::var_os("ALLOW_ZERO_SCORE").is_some() || !scores.is_degenerate() {
        return;
    }
    panic!(
        "{fixture}: degenerate divergence scores `{}` — \
         likely a path or canonicalization bug rather than a real score; \
         set ALLOW_ZERO_SCORE=1 to accept it as a baseline",
        scores.headline()
    );
}

// ---- comparison + regen ------------------------------------------------

fn remove_sidecar(path: &Path) {
    match fs::remove_file(path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => panic!("removing stale sidecar {}: {e}", path.display()),
    }
}

fn compare_or_update(kind: &str, path: &Path, actual: &[u8]) {
    let new_path = path.with_extension(format!(
        "{}.new",
        path.extension().and_then(|s| s.to_str()).unwrap_or("")
    ));
    let existing = fs::read(path).ok();
    let matches = existing.as_deref().map(|b| b == actual).unwrap_or(false);
    if matches {
        // Drop any stale sidecar from a prior failing run.
        remove_sidecar(&new_path);
        return;
    }
    if update_baselines() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("mkdir {}: {e}", parent.display()));
        }
        fs::write(path, actual).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        remove_sidecar(&new_path);
        return;
    }
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

#[test]
fn fixture_baselines_ns_pins_match_fixture_pins() {
    let ns_dir = manifest_dir().join(NS_DIR);
    let read_dir =
        fs::read_dir(&ns_dir).unwrap_or_else(|e| panic!("reading {}: {e}", ns_dir.display()));
    let mut problems = Vec::new();
    let mut ns_count = 0;
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        ns_count += 1;
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
    assert!(
        ns_count > 0,
        "no NS TOMLs found under {} — directory moved or emptied?",
        ns_dir.display()
    );
}
