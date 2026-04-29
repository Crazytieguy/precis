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
//!   fixture with a frozen NS (`tests/north-stars/<fixture>.toml`),
//!   plus `tests/divergence/OVERVIEW.md`. Divergence reports are
//!   generated from the in-memory schedules the per-fixture tests already
//!   produce; the overview is assembled once all fixture summaries have
//!   landed in the process-global map.
//!
//! Unified regen: `UPDATE_BASELINES=1 cargo t` accepts all three artifact
//! types (sets `INSTA_UPDATE=always` internally for the rendered snapshot).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use precis::{
    Schedule,
    divergence::{BUDGETS, PRIMARY_BUDGET_INDEX, generate_divergence_report},
    ns_loader::load_ns_checked,
    render_schedule, render_with_schedule,
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
            fn [<fixture_baselines_ $fixture>]() {
                check_fixture_baselines($name);
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

fn divergence_index_path() -> PathBuf {
    manifest_dir().join(DIVERGENCE_DIR).join("OVERVIEW.md")
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

// ---- per-fixture baselines ---------------------------------------------

fn check_fixture_baselines(fixture: &str) {
    let fixture_dir = require_fixture(fixture);

    let schedule = render_schedule(&[&fixture_dir], SCHEDULE_BUDGET)
        .unwrap_or_else(|e| panic!("render_schedule({fixture}): {e}"));
    check_schedule_toml(fixture, &schedule);
    check_rendered(fixture, &fixture_dir, &schedule);
    if let Some(report) = check_divergence(fixture, &fixture_dir, &schedule) {
        record_divergence_summary(fixture, &report);
    }
}

fn check_schedule_toml(fixture: &str, schedule: &Schedule) {
    let serialized = toml::to_string(schedule)
        .unwrap_or_else(|e| panic!("serializing schedule({fixture}): {e}"));
    compare_or_update(
        "schedule TOML",
        &schedule_path(fixture),
        serialized.as_bytes(),
    );
}

fn check_divergence(fixture: &str, fixture_dir: &Path, schedule: &Schedule) -> Option<String> {
    let ns_toml = ns_path(fixture);
    if !ns_toml.exists() {
        return None;
    }
    let ns = load_ns_checked(&ns_toml, fixture_dir)
        .unwrap_or_else(|e| panic!("load_ns_checked({fixture}): {e}"));
    let report = generate_divergence_report(&ns, schedule, fixture_dir)
        .unwrap_or_else(|e| panic!("generate_divergence_report({fixture}): {e}"));

    if let Some(first_line) = report.lines().next() {
        println!("{fixture}: {first_line}");
    }

    compare_or_update(
        "divergence report",
        &divergence_path(fixture),
        report.as_bytes(),
    );
    Some(report)
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

#[derive(Clone)]
struct DivergenceSummaryRow {
    fixture: String,
    /// `Score(B)` for each B in [`BUDGETS`], formatted to 3 decimals. `"—"`
    /// when the report didn't expose that budget (defensive — should
    /// never happen with the current renderer).
    scores: [String; BUDGETS.len()],
    verdict: String,
    primary: String,
    evidence: String,
}

/// Rebuild OVERVIEW.md from the on-disk per-fixture reports. Under
/// nextest's process-per-test model each test runs in its own
/// process, so we can't accumulate summaries through a static mutex.
/// Instead, every per-fixture test re-reads the full set of reports
/// and writes OVERVIEW iff every fixture's report is present —
/// guarantees `OVERVIEW.md` is only ever written from a complete
/// snapshot, regardless of which process finishes last.
fn record_divergence_summary(_fixture: &str, _report: &str) {
    let div_dir = manifest_dir().join(DIVERGENCE_DIR);
    let expected = fixtures_with_north_stars();
    let rows: Vec<DivergenceSummaryRow> = expected
        .iter()
        .filter_map(|fixture| {
            let path = div_dir.join(format!("{fixture}.md"));
            let report = fs::read_to_string(&path).ok()?;
            Some(summary_row_from_report(fixture, &report))
        })
        .collect();
    if rows.len() != expected.len() {
        // Not all fixtures have a report on disk yet — another process
        // (or a later test in this process) will write the complete
        // OVERVIEW.
        return;
    }
    check_divergence_overview(rows);
}

fn summary_row_from_report(fixture: &str, report: &str) -> DivergenceSummaryRow {
    DivergenceSummaryRow {
        fixture: fixture.to_string(),
        scores: extract_score_vector(report)
            .unwrap_or_else(|| std::array::from_fn(|_| "—".to_string())),
        verdict: extract_prefixed_line(report, "Verdict: ")
            .unwrap_or("—")
            .to_string(),
        primary: extract_prefixed_line(report, "Likely primary lever: ")
            .unwrap_or("—")
            .to_string(),
        evidence: extract_prefixed_line(report, "Evidence: ")
            .unwrap_or("—")
            .to_string(),
    }
}

fn check_divergence_overview(mut rows: Vec<DivergenceSummaryRow>) {
    rows.sort_by(|a, b| {
        let score_a = a.scores[PRIMARY_BUDGET_INDEX]
            .parse::<f64>()
            .unwrap_or(f64::INFINITY);
        let score_b = b.scores[PRIMARY_BUDGET_INDEX]
            .parse::<f64>()
            .unwrap_or(f64::INFINITY);
        score_a
            .partial_cmp(&score_b)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.fixture.cmp(&b.fixture))
    });
    let mut index = String::new();
    index.push_str("# Divergence Summary\n\n");
    index.push_str("Sorted by `Score(3000)` ascending. Score columns are `√(Importance × Coverage)` evaluated at each budget on the geometric grid `[1000, 9000]` with ratio ⁶√9 ≈ 1.442.\n\n");
    let mut header = String::from("| fixture |");
    let mut sep = String::from("|:--------|");
    for &b in BUDGETS.iter() {
        header.push_str(&format!(" s@{b} |"));
        sep.push_str("-----:|");
    }
    header.push_str(" verdict | likely primary lever | evidence |\n");
    sep.push_str(":--------|:---------------------|:---------|\n");
    index.push_str(&header);
    index.push_str(&sep);
    for row in rows {
        index.push_str(&format!("| {} |", row.fixture));
        for s in &row.scores {
            index.push_str(&format!(" {s} |"));
        }
        index.push_str(&format!(
            " {} | {} | {} |\n",
            row.verdict, row.primary, row.evidence
        ));
    }

    compare_or_update(
        "divergence summary index",
        &divergence_index_path(),
        index.as_bytes(),
    );
}

fn fixtures_with_north_stars() -> Vec<String> {
    let ns_dir = manifest_dir().join("tests/north-stars");
    let mut fixtures = Vec::new();
    let read_dir =
        fs::read_dir(&ns_dir).unwrap_or_else(|e| panic!("read {}: {e}", ns_dir.display()));
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        let Some(fixture) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        fixtures.push(fixture.to_string());
    }
    fixtures.sort();
    fixtures
}

/// Parse the per-budget table emitted by `format_score_vector`.
/// Each `BUDGETS[i]` row has shape `| <B> | <A_B> | <I> | <C> |
/// <Score> | <walker_used> |`; the 5th pipe-separated cell holds the
/// score string we want, formatted to 3 decimals already by the
/// renderer. We pass that through verbatim so OVERVIEW.md doesn't
/// re-round.
fn extract_score_vector(report: &str) -> Option<[String; BUDGETS.len()]> {
    let mut out: [Option<String>; BUDGETS.len()] = std::array::from_fn(|_| None);
    for line in report.lines() {
        if out.iter().all(Option::is_some) {
            break;
        }
        let stripped = line.trim();
        if !stripped.starts_with("| ") {
            continue;
        }
        let parts: Vec<&str> = stripped.split('|').map(|s| s.trim()).collect();
        // ["", "B", "A_B", "I(B)", "C(B)", "Score(B)", "walker_used", ""]
        if parts.len() < 7 {
            continue;
        }
        let Ok(budget) = parts[1].parse::<usize>() else {
            continue;
        };
        let Some(idx) = BUDGETS.iter().position(|&b| b == budget) else {
            continue;
        };
        out[idx] = Some(parts[5].to_string());
    }
    out.into_iter()
        .collect::<Option<Vec<_>>>()
        .and_then(|v| v.try_into().ok())
}

fn extract_prefixed_line<'a>(report: &'a str, prefix: &str) -> Option<&'a str> {
    report.lines().find_map(|line| line.strip_prefix(prefix))
}

#[test]
fn fixture_baselines_ns_pins_match_fixture_pins() {
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
