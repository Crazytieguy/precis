//! Violation + growth-envelope tests for `ns_simulate::simulate_ns`.
//! Each test constructs a synthetic NS TOML pointed at the `log` fixture
//! and checks that the expected violation surfaces. Covers the three
//! NS-authoring pathologies the growth envelope + load-time validation
//! are meant to catch.

use std::path::Path;

use precis::north_star::NorthStar;
use precis::ns_simulate::{ENV_BASE, Violation, envelope_max, simulate_ns};

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

/// Two spans in one batch cover the same `(path, line)`. Batch spans
/// must be disjoint; cross-batch overrides use predecessor edges.
/// Expected: `OverlappingSpans` violation.
#[test]
fn ns_simulate_detects_overlapping_spans_within_batch() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "overlapping"
justification = "two spans cover line 5"
[batches.content]
kind = "lines"
spans = [
  {{ path = "src/lib.rs", start = 1, end = 5, render = {{ kind = "full" }} }},
  {{ path = "src/lib.rs", start = 5, end = 10, render = {{ kind = "full" }} }},
]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::OverlappingSpans { line, .. } if *line == 5)),
        "expected OverlappingSpans at line 5, got {:?}",
        report.batches[0].violations
    );
}

#[test]
fn ns_simulate_detects_overlapping_fs_entries_across_batches() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "root listing"
justification = "orientation"
[batches.content]
kind = "fs"
groups = [{{ parent = ".", entries = ["src"] }}]

[[batches]]
id = "2"
descriptor = "duplicate root listing"
justification = "duplicate fs atom"
[batches.content]
kind = "fs"
groups = [{{ parent = ".", entries = ["src", "Cargo.toml"] }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0].violations.is_empty(),
        "first owner should pass, got {:?}",
        report.batches[0].violations
    );
    assert!(
        report.batches[1].violations.iter().any(|v| matches!(
            v,
            Violation::OverlappingFsEntry {
                entry,
                existing_batch,
                ..
            } if entry == "src" && existing_batch == "1"
        )),
        "expected OverlappingFsEntry for src, got {:?}",
        report.batches[1].violations
    );
}

/// Multi-line `Render::Ellipsis` span renders one `…` per covered line
/// — visually indistinguishable from a single marker (no line numbers
/// to differentiate them) but costs N× the tokens. The schema doc on
/// `Render::Ellipsis` calls this single-line-only; the validator
/// should flag it. Expected: `EllipsisMultiLine` violation, batch
/// still simulates (quality-only, not render-blocking).
#[test]
fn ns_simulate_detects_multi_line_ellipsis() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "ellipsis 5..=8"
justification = "should fail single-line invariant"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 5, end = 8, render = {{ kind = "ellipsis" }} }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert_eq!(report.batches.len(), 1);
    let violations = &report.batches[0].violations;
    assert!(
        violations.iter().any(|v| matches!(
            v,
            Violation::EllipsisMultiLine { start, end, .. } if *start == 5 && *end == 8
        )),
        "expected EllipsisMultiLine 5..=8, got {violations:?}"
    );
}

/// Single-line `Render::Ellipsis` is valid — the validator must not
/// fire on `start == end`.
#[test]
fn ns_simulate_accepts_single_line_ellipsis() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "ellipsis at line 5"
justification = "single-line ellipsis is valid"
[batches.content]
kind = "lines"
spans = [{{ path = "src/lib.rs", start = 5, end = 5, render = {{ kind = "ellipsis" }} }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    let violations = &report.batches[0].violations;
    assert!(
        !violations
            .iter()
            .any(|v| matches!(v, Violation::EllipsisMultiLine { .. })),
        "single-line Ellipsis should not fire EllipsisMultiLine, got {violations:?}"
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

/// A `lines` batch with no spans resolves to zero atoms — it renders as a
/// no-op and can never be credited by divergence. Expected: `EmptyBatch`.
#[test]
fn ns_simulate_detects_empty_lines_batch() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "no spans"
justification = "zero-atom no-op"
[batches.content]
kind = "lines"
spans = []
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::EmptyBatch)),
        "expected EmptyBatch, got {:?}",
        report.batches[0].violations
    );
}

/// An `fs` batch whose only group lists nothing resolves to zero atoms.
/// Expected: `EmptyBatch`.
#[test]
fn ns_simulate_detects_empty_fs_batch() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "empty listing"
justification = "zero-atom no-op"
[batches.content]
kind = "fs"
groups = [{{ parent = ".", entries = [] }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::EmptyBatch)),
        "expected EmptyBatch, got {:?}",
        report.batches[0].violations
    );
}

/// An absolute span path escapes the fixture root. Expected:
/// `SpanPathEscapesRoot` (render-blocking — the batch is skipped).
#[test]
fn ns_simulate_detects_absolute_span_path() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "absolute path"
justification = "escapes fixture root"
[batches.content]
kind = "lines"
spans = [{{ path = "/etc/passwd", start = 1, end = 1, render = {{ kind = "full" }} }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::SpanPathEscapesRoot { .. })),
        "expected SpanPathEscapesRoot, got {:?}",
        report.batches[0].violations
    );
}

/// A `..`-traversing span path escapes the fixture root. Expected:
/// `SpanPathEscapesRoot`.
#[test]
fn ns_simulate_detects_parent_traversal_span_path() {
    let pin = fixture_pin(LOG_FIXTURE);
    let ns = load_ns_toml(&format!(
        r#"fixture = "log"
revision_pin = "{pin}"

[[batches]]
id = "1"
descriptor = "parent traversal"
justification = "escapes fixture root"
[batches.content]
kind = "lines"
spans = [{{ path = "../Cargo.toml", start = 1, end = 1, render = {{ kind = "full" }} }}]
"#
    ));
    let report = simulate_ns(&ns, Path::new(LOG_FIXTURE)).expect("simulate");
    assert!(
        report.batches[0]
            .violations
            .iter()
            .any(|v| matches!(v, Violation::SpanPathEscapesRoot { .. })),
        "expected SpanPathEscapesRoot, got {:?}",
        report.batches[0].violations
    );
}
