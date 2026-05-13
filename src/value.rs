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
    if let Some(first) = comps.next().and_then(|c| c.as_os_str().to_str()) {
        if first.eq_ignore_ascii_case(".github") {
            // CI workflows are load-bearing operational config; everything
            // else under `.github/` is contributor templates and admin docs.
            let second = comps.next().and_then(|c| c.as_os_str().to_str());
            if !second.is_some_and(|s| s.eq_ignore_ascii_case("workflows")) {
                return 0.2;
            }
        }
        // Changesets release-tooling subtree: every file is release
        // metadata, uniformly low-priority next to source.
        if first.eq_ignore_ascii_case(".changeset") {
            return 0.2;
        }
    }
    // Auto-injected agent docs: AGENTS.md / CLAUDE.md at the root, plus
    // text-content files under .claude/skills/, .agent/skills/,
    // .cursor/rules/. The harness already loads these into the model's
    // context, so spending precis budget on their headings/lede is
    // pure waste. (Section + SummaryWhole batches are dropped entirely
    // in `walker::markdown`; this discount is defense-in-depth for the
    // residual HeadingsOutline / ReadmeHeadline.)
    if is_auto_injected_doc_file(path, root) {
        return 0.1;
    }
    // Peripheral admin / release / translation markdown at the root.
    // NS authors universally treat these as "appendix" content; the
    // walker should not let CHANGELOG, CONTRIBUTING, etc. crowd the
    // primary-source schedule.
    if is_top_level_peripheral_doc(target) || is_top_level_localized_readme(target) {
        return 0.2;
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

/// Auto-injected agent-instruction files: their bodies are already in
/// the model's context, so precis budget spent on them is waste. The
/// extension filter is what keeps directory listings from matching —
/// `.claude/skills` (no extension) reaches the predicate but returns
/// false, so its fs-listing batch keeps full weight and the file paths
/// stay discoverable.
///
/// Takes the original `path` + `root` (mirroring [`non_essential_factor`])
/// rather than a pre-stripped target, so call sites don't have to
/// duplicate the strip-prefix dance.
pub fn is_auto_injected_doc_file(path: &std::path::Path, root: &std::path::Path) -> bool {
    let target = path.strip_prefix(root).unwrap_or(path);
    let Some(ext) = target.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    if !["md", "rst", "txt", "mdc"]
        .iter()
        .any(|e| ext.eq_ignore_ascii_case(e))
    {
        return false;
    }
    let mut comps = target.components();
    let Some(first) = comps.next().and_then(|c| c.as_os_str().to_str()) else {
        return false;
    };
    let second = comps.next().and_then(|c| c.as_os_str().to_str());
    if second.is_none()
        && let Some(stem) = target.file_stem().and_then(|s| s.to_str())
        && (stem.eq_ignore_ascii_case("AGENTS") || stem.eq_ignore_ascii_case("CLAUDE"))
    {
        return true;
    }
    let Some(second) = second else { return false };
    (first.eq_ignore_ascii_case(".claude") && second.eq_ignore_ascii_case("skills"))
        || (first.eq_ignore_ascii_case(".agent") && second.eq_ignore_ascii_case("skills"))
        || (first.eq_ignore_ascii_case(".cursor") && second.eq_ignore_ascii_case("rules"))
}

/// True iff `target` (already root-relative) is a single component —
/// a file directly at the repo root. Used by predicates that should
/// only fire on top-level admin/translation markdown, leaving the
/// same basenames in subdirectories at full weight.
fn is_top_level(target: &std::path::Path) -> bool {
    let mut comps = target.components();
    comps.next().is_some() && comps.next().is_none()
}

/// True for top-level admin / release markdown — CHANGELOG /
/// CONTRIBUTING / SECURITY / NOTICE / RELEASING / etc. — only when
/// at the repo root. Subdir occurrences (e.g. otree's
/// `docs/changelog.md`) are deliberately untouched because NS authors
/// sometimes promote them.
pub fn is_top_level_peripheral_doc(target: &std::path::Path) -> bool {
    if !is_top_level(target) {
        return false;
    }
    let Some(ext) = target.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    if !ext.eq_ignore_ascii_case("md") && !ext.eq_ignore_ascii_case("rst") {
        return false;
    }
    let Some(stem) = target.file_stem().and_then(|s| s.to_str()) else {
        return false;
    };
    [
        "CHANGELOG",
        "CHANGES",
        "HISTORY",
        "RELEASING",
        "RELEASES",
        "RELEASE_NOTES",
        "CONTRIBUTING",
        "SECURITY",
        "NOTICE",
        "AUTHORS",
        "MAINTAINERS",
        "CODE_OF_CONDUCT",
        "CODEOWNERS",
        "SUPPORT",
        "GOVERNANCE",
    ]
    .iter()
    .any(|s| stem.eq_ignore_ascii_case(s))
}

/// True for `README.<locale>.<ext>` or `Readme_<locale>.<ext>` at the
/// repo root, where `<locale>` is a known ISO-639-style code. The
/// whitelist (rather than a `[a-z]{2,3}([-_][A-Z]{2,4})?` regex) is
/// what rules out `README.api.md` / `README.dev.md` / `README.old.md`.
pub fn is_top_level_localized_readme(target: &std::path::Path) -> bool {
    if !is_top_level(target) {
        return false;
    }
    let Some(name) = target.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    let stem_lower = lower
        .strip_suffix(".md")
        .or_else(|| lower.strip_suffix(".rst"));
    let Some(stem_lower) = stem_lower else {
        return false;
    };
    let rest = stem_lower
        .strip_prefix("readme.")
        .or_else(|| stem_lower.strip_prefix("readme_"));
    let Some(rest) = rest else { return false };
    if rest.is_empty() {
        return false;
    }
    is_known_locale(rest)
}

fn is_known_locale(s: &str) -> bool {
    // Compare case-insensitively against the whitelist. We accept both
    // `<lang>-<REGION>` and `<lang>_<REGION>` forms; normalize the
    // separator before matching.
    let normalized = s.replace('_', "-");
    matches!(
        normalized.as_str(),
        "zh" | "zh-cn"
            | "zh-tw"
            | "zh-hk"
            | "ja"
            | "ko"
            | "fr"
            | "de"
            | "es"
            | "it"
            | "pt"
            | "pt-br"
            | "ru"
            | "ar"
            | "hi"
            | "nl"
            | "pl"
            | "tr"
            | "sv"
            | "no"
            | "da"
            | "fi"
            | "cs"
            | "vi"
            | "th"
            | "id"
            | "he"
            | "uk"
            | "ro"
            | "hu"
            | "el"
            | "en"
            | "en-us"
            | "en-gb"
    )
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

    #[test]
    fn value_top_level_peripheral_docs_are_discounted() {
        let root = Path::new("/repo");
        for name in [
            "CHANGELOG.md",
            "changelog.rst",
            "HISTORY.md",
            "RELEASE_NOTES.md",
            "RELEASING.md",
            "CONTRIBUTING.md",
            "SECURITY.md",
            "NOTICE.md",
            "AUTHORS.md",
            "CODE_OF_CONDUCT.md",
            "CODEOWNERS.md",
            "SUPPORT.md",
            "GOVERNANCE.md",
        ] {
            assert_eq!(
                non_essential_factor(&root.join(name), root),
                0.2,
                "top-level {name}",
            );
        }
    }

    /// Same basenames in subdirectories keep full weight — NS authors
    /// sometimes promote `docs/changelog.md` (otree) as primary
    /// content. The peripheral-doc predicate is intentionally
    /// root-scoped.
    #[test]
    fn value_subdir_peripheral_basenames_keep_full_weight() {
        let root = Path::new("/repo");
        // `docs/` is not in the non-essential dir list, so these files
        // stay at 1.0.
        assert_eq!(
            non_essential_factor(&root.join("docs/changelog.md"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join("docs/CONTRIBUTING.md"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join("subproject/CHANGELOG.md"), root),
            1.0,
        );
    }

    #[test]
    fn value_auto_injected_agent_docs_are_strongly_discounted() {
        let root = Path::new("/repo");
        for path in [
            "AGENTS.md",
            "agents.md",
            "CLAUDE.md",
            "claude.rst",
            ".claude/skills/foo/SKILL.md",
            ".agent/skills/bar/instructions.md",
            ".cursor/rules/baz.mdc",
        ] {
            assert_eq!(
                non_essential_factor(&root.join(path), root),
                0.1,
                "auto-injected {path}",
            );
        }
    }

    /// Directories under the skill subtrees keep full weight so file
    /// paths remain discoverable via fs listings. Non-text files (real
    /// code / data) under the same subtrees are walked normally.
    #[test]
    fn value_auto_injected_dirs_and_non_text_keep_full_weight() {
        let root = Path::new("/repo");
        // Directories — no extension.
        assert_eq!(
            non_essential_factor(&root.join(".claude/skills"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join(".claude/skills/foo"), root),
            1.0,
        );
        // Non-text content under a skill dir is treated normally.
        assert_eq!(
            non_essential_factor(&root.join(".claude/skills/foo/script.py"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join(".claude/skills/foo/data.json"), root),
            1.0,
        );
    }

    #[test]
    fn value_changeset_subtree_is_discounted() {
        let root = Path::new("/repo");
        assert_eq!(
            non_essential_factor(&root.join(".changeset/foo.md"), root),
            0.2,
        );
        assert_eq!(
            non_essential_factor(&root.join(".changeset/README.md"), root),
            0.2,
        );
        assert_eq!(
            non_essential_factor(&root.join(".changeset/config.json"), root),
            0.2,
        );
    }

    /// `contribute/` is **not** in the demotion set: only mcphost uses
    /// it, the savings are tiny, and the subtree contains non-markdown
    /// content (`build.sh`, `conf/demo.json`) where demotion would be
    /// unjustified.
    #[test]
    fn value_contribute_subtree_keeps_full_weight() {
        let root = Path::new("/repo");
        assert_eq!(
            non_essential_factor(&root.join("contribute/contribute.md"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join("contribute/build.sh"), root),
            1.0,
        );
        assert_eq!(
            non_essential_factor(&root.join("contribute/conf/demo.json"), root),
            1.0,
        );
    }

    #[test]
    fn value_localized_readme_is_discounted() {
        let root = Path::new("/repo");
        for name in [
            "README.zh-CN.md",
            "Readme_zh-CN.md",
            "README.ja.md",
            "README.pt_BR.rst",
            "README.fr.md",
            "README.en-US.md",
        ] {
            assert_eq!(
                non_essential_factor(&root.join(name), root),
                0.2,
                "localized {name}",
            );
        }
    }

    /// The whitelist eliminates false positives a loose
    /// `[a-z]{2,3}(-[A-Z]{2,4})?` regex would catch: `README.api.md`,
    /// `README.dev.md`, etc. are not localized copies, they're
    /// auxiliary docs whose contents matter.
    /// (`README.test.md` is excluded from this list — it's caught
    /// by the pre-existing co-located-test rule, which discounts to
    /// 0.2 for a different reason.)
    #[test]
    fn value_non_locale_readme_suffixes_keep_full_weight() {
        let root = Path::new("/repo");
        for name in [
            "README.api.md",
            "README.dev.md",
            "README.old.md",
            "README_template.md",
        ] {
            assert_eq!(
                non_essential_factor(&root.join(name), root),
                1.0,
                "non-locale {name}",
            );
        }
    }

    #[test]
    fn value_bare_readme_keeps_full_weight() {
        let root = Path::new("/repo");
        assert_eq!(non_essential_factor(&root.join("README.md"), root), 1.0,);
        assert_eq!(non_essential_factor(&root.join("README.rst"), root), 1.0,);
        assert_eq!(non_essential_factor(&root.join("Readme.md"), root), 1.0,);
    }

    /// Localized READMEs in subdirs keep full weight — only top-level
    /// localized copies are demoted.
    #[test]
    fn value_subdir_localized_readme_keeps_full_weight() {
        let root = Path::new("/repo");
        assert_eq!(
            non_essential_factor(&root.join("docs/README.zh-CN.md"), root),
            1.0,
        );
    }
}
