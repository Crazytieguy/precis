//! Value model: composes [`ValueSignals`] into a scalar `f64` for the
//! scheduler's greedy pick. Walkers contribute *signals*, not values — the
//! mixing weights live here so every walker's output is comparable on one
//! scale.
//!
//! The three mixing weights (`W_CATASTROPHIC`, `W_FOLLOW_UP`, `W_ZERO_CALL`)
//! are the ontology's three failure modes we're trying to avoid. First-pass
//! calibration; tune against north-star divergence reports.

use crate::batch::ValueSignals;

/// Weight on the catastrophic-omission signal. Highest — missing this
/// content means the agent forms a wrong mental model.
pub const W_CATASTROPHIC: f64 = 1000.0;

/// Weight on the follow-up-minimization signal. Each 1.0 saves one tool
/// call's worth of work.
pub const W_FOLLOW_UP: f64 = 400.0;

/// Weight on the zero-tool-call-understanding signal. How much this content
/// contributes to answering a question *without* any follow-up.
pub const W_ZERO_CALL: f64 = 300.0;

/// Compose a set of signals into a scalar value. Linear in signals,
/// multiplied by the depth/boost factor. Value is always >= 0.
pub fn score(signals: &ValueSignals) -> f64 {
    let base = W_CATASTROPHIC * signals.catastrophic_omission
        + W_FOLLOW_UP * signals.follow_up_minimization
        + W_ZERO_CALL * signals.zero_tool_call_understanding;
    base * signals.depth_factor.max(0.0)
}

/// Down-weight a batch by filesystem depth. Depth 0 (root) and depth 1
/// (files directly in root, e.g. Cargo.toml, README.md) are unpenalized;
/// penalty grows for deeper content. First-pass placeholder; calibrate
/// against north-star reports.
pub fn depth_factor(depth: usize) -> f64 {
    1.0 / (1.0 + depth.saturating_sub(1) as f64 * 0.3)
}

/// Multiplier applied to content whose path is under a "non-essential"
/// directory (tests / examples / benches / fixtures / private helpers /
/// showcase websites). These are load-bearing for *using* the crate's
/// infrastructure but rarely for understanding it; they should only
/// appear once the primary-source batches have landed.
///
/// File-level test-file naming (`*.test.ts`, `*.spec.ts`, `*_test.go`)
/// is also caught — many JS/TS projects keep tests next to source rather
/// than in a `tests/` directory.
///
/// `path` is matched component-wise *relative to* `root` (so the outer
/// `tests/fixtures/` of the test harness doesn't poison every fixture
/// path). When `path` isn't under `root` (a walker bug; not panicked on
/// for release-mode robustness), the absolute path is used as-is.
pub fn non_essential_factor(path: &std::path::Path, root: &std::path::Path) -> f64 {
    let target = path.strip_prefix(root).unwrap_or(path);
    for component in target.components() {
        let Some(s) = component.as_os_str().to_str() else {
            continue;
        };
        if matches!(
            s,
            "tests"
                | "test"
                | "examples"
                | "example"
                | "benches"
                | "bench"
                | "benchmark"
                | "benchmarks"
                | "fixtures"
                | "rfcs"
                | "xtask"
                | "ci"
                | "website"
                | "docs-site"
                | "demo"
                | "demos"
                | "playground"
                | "showcase"
                | "storybook"
                | "fuzz"
        ) || s.starts_with("test_")
            || s.starts_with("guide-helper")
        {
            return 0.2;
        }
    }
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        // File-level heuristic: `__private_api.rs`, `__internals.rs`,
        // `inner.rs`, etc. — files whose name itself says "not the
        // public surface".
        if name.starts_with("__") || name.starts_with("_") || name == "inner.rs" {
            return 0.5;
        }
        // Co-located test files: `foo.test.ts`, `foo.spec.ts`,
        // `foo_test.go`, `foo.test.tsx`, `foo.test.js`, etc.
        let lower = name.to_ascii_lowercase();
        if lower.contains(".test.")
            || lower.contains(".spec.")
            || lower.ends_with("_test.go")
            || lower.ends_with("_test.ts")
            || lower.ends_with("_test.js")
            || lower.ends_with("_test.tsx")
        {
            return 0.2;
        }
    }
    1.0
}

/// Convert a value and a marginal token cost into the scheduling ratio.
/// Sublinear in cost via `cost^0.35`: the ontology's principle that "value
/// is sublinear in batch size" plus the empirical observation that `sqrt`
/// alone is too aggressive on cost — a 1500-token `lib.rs` PubDecls batch
/// holding the full crate API gets beaten by two dozen 80-token batches
/// with similar per-token ratio, but losing that one coherent batch is a
/// catastrophic-omission outcome. The gentler exponent keeps big anchor
/// batches competitive.
pub fn ratio(value: f64, cost_tokens: usize) -> f64 {
    if cost_tokens == 0 {
        return f64::INFINITY;
    }
    value / (cost_tokens as f64).powf(0.35)
}
