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

/// Chunk size for names-surface gates. Twelve declarations is roughly one
/// screenful of API anchors: enough context to orient, small enough that a
/// later chunk does not make the first chunk too expensive for descendants.
pub const NAMES_SURFACE_CHUNK_SIZE: usize = 12;

/// First chunks are deliberately a little below the previous unchunked
/// surface value so small unchunked files still win comparable rank races.
const CHUNKED_NAMES_FIRST_CHUNK_FACTOR: f64 = 0.9;
/// A 0.25 falloff puts the fourth chunk at about half the first chunk,
/// keeping source-order tails available without letting giant catalogs win
/// every early scheduling slot.
const CHUNKED_NAMES_FALLOFF: f64 = 0.25;

pub fn names_surface_chunk_count(dependent_count: usize) -> usize {
    dependent_count.div_ceil(NAMES_SURFACE_CHUNK_SIZE)
}

pub fn names_surface_chunk_index(item_index: usize) -> usize {
    item_index / NAMES_SURFACE_CHUNK_SIZE
}

/// Value multiplier for a names-surface batch that gates per-item
/// descendants. The surface carries its own orientation value, but it
/// also unlocks the scheduler's ability to choose precise child batches
/// later; dense public API files should therefore beat unrelated
/// follow-up batches without letting one enormous catalog dominate the
/// whole prefix.
pub fn names_surface_chunk_factor(chunk_index: usize, chunk_count: usize) -> f64 {
    if chunk_count <= 1 {
        1.0
    } else {
        CHUNKED_NAMES_FIRST_CHUNK_FACTOR / (1.0 + chunk_index as f64 * CHUNKED_NAMES_FALLOFF)
    }
}

/// Value multiplier for import/re-export-wall chunks. The first chunk keeps
/// full import-batch value because it unlocks the package surface; later
/// source groups fall off faster than names surfaces so tail plumbing does
/// not crowd more precise semantic anchors at small budgets.
pub fn reexport_import_chunk_factor(chunk_index: usize, chunk_count: usize) -> f64 {
    if chunk_count <= 1 {
        1.0
    } else {
        1.0 / (1.0 + chunk_index as f64 * 0.5)
    }
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
/// contributor automation / showcase websites). These are load-bearing for
/// *using* the crate's infrastructure but rarely for understanding it; they
/// should only appear once the primary-source batches have landed.
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
    let mut comps = target.components();
    if let Some(first) = comps.next().and_then(|c| c.as_os_str().to_str())
        && first.eq_ignore_ascii_case(".github")
    {
        // CI workflows are load-bearing operational config; everything
        // else under `.github/` is contributor templates and admin docs.
        let second = comps.next().and_then(|c| c.as_os_str().to_str());
        if !second.is_some_and(|s| s.eq_ignore_ascii_case("workflows")) {
            return 0.2;
        }
    }
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
        // File-level heuristic for Rust private-helper conventions:
        // `__private_api.rs`, `__internals.rs`, `inner.rs`. The
        // discount is intentionally Rust-only — `_hooks.py` /
        // `__init__.py` are load-bearing Python (PEP 8 internal-but-
        // public convention), and `_app.tsx` / `_routes.json` are
        // framework orientation files (Next.js, Cloudflare). A blanket
        // `_*` rule would penalize all of those.
        let ext = path.extension().and_then(|e| e.to_str());
        if name == "inner.rs" || (ext == Some("rs") && name.starts_with("__")) {
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

/// Default cost-side concavity exponent for the scheduling ratio. The
/// ontology's principle is "value is sublinear in batch size"; `sqrt`
/// alone is too aggressive on cost — a 1500-token `lib.rs` PubDecls
/// batch holding the full crate API gets beaten by two dozen 80-token
/// batches with similar per-token ratio, but losing that one coherent
/// batch is a catastrophic-omission outcome. The gentler default keeps
/// big anchor batches competitive. Per-key overrides on
/// [`crate::batch::WalkerKey::concavity_exponent`] raise this for
/// prose-shaped batches whose token count grows without proportional
/// structural value.
pub const DEFAULT_CONCAVITY_EXPONENT: f64 = 0.35;

/// Convert a value and a marginal token cost into the scheduling ratio
/// at the default concavity exponent. Thin wrapper over
/// [`ratio_with_exponent`] for callers that don't need a per-key exponent.
pub fn ratio(value: f64, cost_tokens: usize) -> f64 {
    ratio_with_exponent(value, cost_tokens, DEFAULT_CONCAVITY_EXPONENT)
}

/// Like [`ratio`] but with caller-supplied concavity exponent. The
/// scheduler calls this with `entry.key.concavity_exponent()` so prose-
/// shaped batches see a steeper cost penalty than structural ones.
pub fn ratio_with_exponent(value: f64, cost_tokens: usize, cost_exponent: f64) -> f64 {
    if cost_tokens == 0 {
        return f64::INFINITY;
    }
    value / (cost_tokens as f64).powf(cost_exponent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// The file-level discount for "private helper" filenames is
    /// Rust-only — Python `_*.py` files (PEP 8 internal-but-load-bearing
    /// convention) and framework underscore files (Next.js `_app.tsx`,
    /// Cloudflare `_routes.json`) keep their full weight. The literal
    /// `inner.rs` and `__-prefixed *.rs` patterns still discount.
    #[test]
    fn value_underscore_filename_discount_is_rust_only() {
        let root = Path::new("/repo");
        // Rust private helper conventions: still 0.5.
        assert_eq!(
            non_essential_factor(&root.join("src/__private_api.rs"), root),
            0.5,
        );
        assert_eq!(non_essential_factor(&root.join("src/inner.rs"), root), 0.5,);
        // Python load-bearing files: full weight.
        assert_eq!(
            non_essential_factor(&root.join("src/pluggy/_hooks.py"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join("src/pluggy/__init__.py"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join("src/pkg/__main__.py"), root),
            1.0,
        );
        // Framework underscore files in JS/TS land: full weight.
        assert_eq!(
            non_essential_factor(&root.join("pages/_app.tsx"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join("packages/x/_routes.json"), root),
            1.0,
        );
        // Single-underscore `*.rs` files no longer get a discount —
        // prefer to specialize per file with a literal allowlist if a
        // regression appears.
        assert_eq!(
            non_essential_factor(&root.join("src/_helper.rs"), root),
            1.0,
        );
    }

    #[test]
    fn value_github_contributor_templates_are_discounted() {
        let root = Path::new("/repo");
        assert_eq!(
            non_essential_factor(&root.join(".github/PULL_REQUEST_TEMPLATE.md"), root),
            0.2,
        );
        assert_eq!(
            non_essential_factor(&root.join(".github/pull_request_template.md"), root),
            0.2,
        );
        assert_eq!(
            non_essential_factor(&root.join(".github/ISSUE_TEMPLATE/bug.md"), root),
            0.2,
        );
    }

    #[test]
    fn value_github_workflows_keep_full_weight() {
        let root = Path::new("/repo");
        assert_eq!(
            non_essential_factor(&root.join(".github/workflows/ci.yml"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join(".github/workflows"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join(".github/dependabot.yml"), root),
            0.2,
        );
    }
}
