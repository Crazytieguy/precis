//! Violation + growth-envelope tests for `ns_simulate::simulate_ns`.
//! Each test constructs a synthetic NS TOML pointed at the `log` fixture
//! and checks that the expected violation surfaces. Covers the three
//! NS-authoring pathologies the growth envelope + load-time validation
//! are meant to catch.

use std::path::Path;

use precis::ns_simulate::{ENV_BASE, Violation, envelope_max, simulate_ns};
use precis::schema::NorthStar;

const LOG_FIXTURE: &str = "tests/fixtures/log";

fn load_ns_toml(toml: &str) -> NorthStar {
    toml::from_str(toml).expect("valid ns toml")
}

fn fixture_pin(path: &str) -> String {
    std::fs::read_to_string(Path::new(path).join(".precis-pin"))
        .unwrap()
        .trim()
        .to_string()
}

#[test]
fn ns_simulate_envelope_math() {
    assert_eq!(envelope_max(0), ENV_BASE);
    assert_eq!(envelope_max(100), 130);
    assert_eq!(envelope_max(1000), 400);
    assert_eq!(envelope_max(10000), 3100);
}

/// Mutation #1 — one batch's cost exceeds the 100 + 0.3·C_{i-1} envelope.
/// Expected: `GrowthEnvelope` violation on that batch, no others.
#[test]
fn ns_simulate_detects_growth_envelope_violation() {
    let pin = fixture_pin(LOG_FIXTURE);
    // Batch 1 is tiny (lines 1-3 of lib.rs, ~30 tokens). Batch 2 covers
    // ~400 lines of full rustdoc — thousands of tokens — which blows past
    // envelope_max(~30) = 100 + 9 = 109.
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "tiny"
justification = "anchor"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 1, end = 3, render = {{ kind = "full" }} }}]

[[batches]]
id = "2"
descriptor = "oversized"
justification = "blows envelope"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 10, end = 400, render = {{ kind = "full" }} }}]
"#
    ));

    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert_eq!(report.batches.len(), 2);
    assert!(
        report.batches[0].violations.is_empty(),
        "batch 1 unexpected violations: {:?}",
        report.batches[0].violations
    );
    let violated = &report.batches[1].violations;
    assert!(
        violated
            .iter()
            .any(|v| matches!(v, Violation::GrowthEnvelope { .. })),
        "expected GrowthEnvelope, got {violated:?}"
    );
}

/// Mutation #2 — two batches share an id. Expected: `DuplicateBatchId`
/// on the second occurrence.
#[test]
fn ns_simulate_detects_duplicate_batch_id() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1.1"
descriptor = "first"
justification = "anchor"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 1, end = 3, render = {{ kind = "full" }} }}]

[[batches]]
id = "1.1"
descriptor = "second with same id"
justification = "clash"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 5, end = 7, render = {{ kind = "full" }} }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0].violations.is_empty(),
        "first batch should pass"
    );
    assert!(
        report.batches[1]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::DuplicateBatchId(id) if id == "1.1")),
        "expected DuplicateBatchId on second, got {:?}",
        report.batches[1].violations
    );
}

/// Mutation #2b — first batch exceeds the envelope (envelope_max(0) = 100).
/// Regression for a bug where the validator skipped the envelope check
/// when `cumulative_before == 0`, letting oversized first batches pass.
#[test]
fn ns_simulate_first_batch_obeys_envelope() {
    let pin = fixture_pin(LOG_FIXTURE);
    // Single batch spanning ~400 lines of rustdoc — well over 100 tokens.
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "oversized first"
justification = "should fail envelope_max(0)=100"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 1, end = 400, render = {{ kind = "full" }} }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::GrowthEnvelope { .. })),
        "expected GrowthEnvelope on first batch, got {:?}",
        report.batches[0].violations
    );
}

/// Mutation #3 — span covers lines beyond the file's line count.
/// Matches the `schema load should have caught this` panic observed last
/// session. Expected: `SpanOutOfRange` (not a panic).
#[test]
fn ns_simulate_detects_span_out_of_range() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "line 99999"
justification = "past EOF"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 1, end = 99999, render = {{ kind = "full" }} }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::SpanOutOfRange { .. })),
        "expected SpanOutOfRange, got {:?}",
        report.batches[0].violations
    );
}
