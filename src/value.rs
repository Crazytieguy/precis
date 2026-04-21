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
/// directory (tests / examples / benches / fixtures / private helpers).
/// These are load-bearing for *using* the crate's infrastructure but rarely
/// for understanding it; they should only appear once the primary-source
/// batches have landed.
pub fn non_essential_factor(path: &std::path::Path) -> f64 {
    for component in path.components() {
        let Some(s) = component.as_os_str().to_str() else {
            continue;
        };
        if matches!(
            s,
            "tests" | "examples" | "benches" | "fixtures" | "rfcs" | "xtask" | "ci"
        ) || s.starts_with("test_")
            || s.starts_with("guide-helper")
        {
            return 0.2;
        }
    }
    // File-level heuristic: `__private_api.rs`, `__internals.rs`, `inner.rs`,
    // etc. — files whose name itself says "not the public surface".
    if let Some(name) = path.file_name().and_then(|n| n.to_str())
        && (name.starts_with("__") || name.starts_with("_") || name == "inner.rs")
    {
        return 0.5;
    }
    1.0
}

/// Convert a value and a marginal token cost into the scheduling ratio.
/// Sublinear in cost — the `sqrt(cost)` denominator means that doubling a
/// batch's size doesn't halve its ratio, only reduces it by √2. This keeps
/// big load-bearing batches (e.g. `lib.rs`'s entire public API) competitive
/// with small cheap batches of similar-per-token value, matching the
/// ontology's "value is sublinear in batch size" principle.
pub fn ratio(value: f64, cost_tokens: usize) -> f64 {
    if cost_tokens == 0 {
        return f64::INFINITY;
    }
    value / (cost_tokens as f64).sqrt()
}
