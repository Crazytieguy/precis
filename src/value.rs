//! Value-model helpers shared across walkers: depth pricing, non-essential
//! discounts, and the cost-side concavity used by the scheduler. Walkers
//! compute their batch's `value: f64` directly; the scheduler ranks by
//! [`ratio`].
//!
//! [`mix_signals`] is a convenience for walkers that still find it natural
//! to express per-batch tuning as a `(catastrophic, follow-up, zero-call)`
//! triple — the weights live here so per-batch numbers stay comparable
//! across walkers. It's not load-bearing: a walker is free to skip the
//! helper and compute its value however. First-pass weights; calibration
//! is experimentation territory (see `docs/design-notes.md`).

/// Mix three first-pass signal axes — catastrophic-omission,
/// follow-up minimization, zero-tool-call understanding — into a scalar
/// value, scaled by the path-relative depth/non-essential factor.
pub fn mix_signals(cat: f64, fu: f64, ztu: f64, depth: f64) -> f64 {
    (1000.0 * cat + 400.0 * fu + 300.0 * ztu) * depth.max(0.0)
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
