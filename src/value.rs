//! Value-model helpers shared across walkers: depth pricing, non-essential
//! discounts, and the cost-side concavity used by the scheduler. Walkers
//! compute their batch's `value: f64` directly; the scheduler ranks by
//! [`ratio`].
//!
//! [`mix_signals`] is a convenience for walkers that still find it natural
//! to express per-batch tuning as a `(catastrophic, follow-up, zero-call)`
//! triple — the weights live here so per-batch numbers stay comparable
//! across walkers. It's not load-bearing: a walker is free to skip the
//! helper and compute its value however. The weights were re-swept on
//! the full corpus on 2026-07-28 against the v2 answer key (zero point
//! 0.6074) and left unchanged; the measured point grids are recorded in
//! `git show a90ee9b6:docs/design-notes.md` ("Post-refreeze re-sweep
//! curves"). Don't re-sweep without a new answer key.

/// Mix three signal axes — catastrophic-omission,
/// follow-up minimization, zero-tool-call understanding — into a scalar
/// value, scaled by the path-relative depth/non-essential factor.
pub fn mix_signals(cat: f64, fu: f64, ztu: f64, depth: f64) -> f64 {
    (1000.0 * cat + 280.0 * fu + 300.0 * ztu) * depth.max(0.0)
}

/// Value of a manifest's runtime dependency roster, shared by every
/// manifest format. What
/// a package depends on is a primary statement of what it *is* — a
/// database driver, an HTTP client, a template engine — for the manifest
/// that describes the repository (its root, or a workspace's primary
/// member). A sub-package's roster is scaffolding around that.
pub fn dependency_roster_value(describes_repository: bool, depth: f64) -> f64 {
    if describes_repository {
        1068.0 * depth
    } else {
        716.0 * depth
    }
}

/// Value of a manifest's identity block (name, version, description),
/// scaled by how much of the project's identity the manifest carries — a
/// workspace member inherits most of it from the root.
pub fn manifest_identity_value(scale: f64, depth: f64) -> f64 {
    1451.0 * scale * depth
}

/// Value of the sections of a manifest that say how the package runs and
/// ships: entrypoints, console scripts, feature flags, runtime
/// constraints. Shared by every manifest format.
pub fn manifest_operational_value(depth: f64) -> f64 {
    839.0 * depth
}

/// Value of a manifest's appendix: author and URL metadata and every
/// table no other section owns (build systems, profiles, lints, tool
/// config). Shared by every manifest format.
pub fn manifest_appendix_value(depth: f64) -> f64 {
    753.0 * depth
}

/// Roster size at which [`roster_mass_factor`] is neutral; rosters this
/// small already rank acceptably without help.
const ROSTER_MASS_BASELINE: f64 = 11.0;
/// Cap on the roster-mass boost (reached around ~40 entries).
pub const ROSTER_MASS_FACTOR_CAP: f64 = 1.6;

/// Ratio-neutralizing factor for "roster" batches — complete catalogs of
/// N peer entries (directory listings, names surfaces, member catalogs)
/// whose value is otherwise size-invariant while cost grows linearly
/// with N, so `value/cost^k` systematically prefers tiny rosters over
/// the complete catalogs NS authors anchor on. Scaling value by
/// `(N / baseline)^k`, `k` = [`DEFAULT_CONCAVITY_EXPONENT`], pushes back
/// against that. Boost-only (≥ 1) and capped: small rosters keep their
/// existing rank rather than being demoted.
pub fn roster_mass_factor(entries: usize) -> f64 {
    (entries as f64 / ROSTER_MASS_BASELINE)
        .powf(DEFAULT_CONCAVITY_EXPONENT)
        .clamp(1.0, ROSTER_MASS_FACTOR_CAP)
}

/// First-chunk factor (slightly below unchunked) so small unchunked
/// files win comparable rank races.
const CHUNKED_NAMES_FIRST_CHUNK_FACTOR: f64 = 0.9;
/// Falloff per later chunk — 0.25 puts chunk 4 at ~half chunk 1.
const CHUNKED_NAMES_FALLOFF: f64 = 0.25;

/// Head premium over ratio parity (`share_0^k`) inside the conserved
/// total. Pure parity prices the head like the unsplit catalog — which
/// was too big to buy inside the NS windows that motivated splitting
/// (tomli's `_parser.py` roster slipped 1224 → 2726 cum, −0.038).
/// Tuned on the combined tree with the cap below; the aggregate stays
/// exactly 1 — the premium is paid by the tails, not by replication.
const CATALOG_HEAD_PREMIUM: f64 = 1.3;

/// Cap on the head's conserved share — the established first-chunk
/// factor ([`CHUNKED_NAMES_FIRST_CHUNK_FACTOR`]), so a premium-boosted
/// head never prices above the old unconserved head tier.
const CATALOG_HEAD_FACTOR_CAP: f64 = CHUNKED_NAMES_FIRST_CHUNK_FACTOR;

/// Value-conserving per-chunk factors for a partitioned catalog. The
/// unsplit catalog's value is a fixed total: factors sum to exactly 1,
/// so splitting redistributes the catalog's value, it never multiplies
/// it, and a split file cannot out-purchase its unsplit self at any
/// budget. Within that total the head chunk takes `share_0^k` (`k` =
/// `cost_exponent` — the allocation at which its `value/cost^k`
/// scheduling ratio matches the unsplit catalog's) times a bounded
/// premium ([`CATALOG_HEAD_PREMIUM`], capped at
/// [`CATALOG_HEAD_FACTOR_CAP`]). Tails split the remainder
/// proportional to token cost tilted by [`CHUNKED_NAMES_FALLOFF`].
pub fn conserved_catalog_chunk_factors(chunk_costs: &[usize], cost_exponent: f64) -> Vec<f64> {
    if chunk_costs.len() <= 1 {
        return vec![1.0; chunk_costs.len()];
    }
    let total: usize = chunk_costs.iter().sum();
    if total == 0 {
        // Degenerate: no chunk produced measurable content. Equal shares
        // keep the sum-to-1 invariant without dividing by zero.
        return vec![1.0 / chunk_costs.len() as f64; chunk_costs.len()];
    }
    let head_share = chunk_costs[0] as f64 / total as f64;
    let head_factor =
        (head_share.powf(cost_exponent) * CATALOG_HEAD_PREMIUM).min(CATALOG_HEAD_FACTOR_CAP);
    let tail_weights: Vec<f64> = chunk_costs[1..]
        .iter()
        .enumerate()
        .map(|(tail_index, &cost)| {
            cost as f64 / (1.0 + (tail_index + 1) as f64 * CHUNKED_NAMES_FALLOFF)
        })
        .collect();
    let tail_total: f64 = tail_weights.iter().sum();
    let remainder = 1.0 - head_factor;
    let mut factors = Vec::with_capacity(chunk_costs.len());
    factors.push(head_factor);
    if tail_total <= 0.0 {
        factors.extend(std::iter::repeat_n(
            remainder / (chunk_costs.len() - 1) as f64,
            chunk_costs.len() - 1,
        ));
    } else {
        factors.extend(
            tail_weights
                .into_iter()
                .map(|weight| remainder * weight / tail_total),
        );
    }
    factors
}

/// Base value of each code-engine rung (`walker::code`), before the file
/// prior and the chunk share: one value for what a file or declaration
/// is (module doc, declaration), one for its depth (doc, body), and the
/// roster's per-entry value. The engine scales `Names` by `entries^k`
/// (`k` the default concavity exponent), so a roster's scheduling ratio
/// depends on its tokens per entry, not on its length. Tuned on the
/// corpus grid (2026-09-25), where `Names` is by far the most sensitive
/// of the three.
pub fn code_rung_value(rung: crate::batch::Rung) -> f64 {
    use crate::batch::Rung;
    match rung {
        Rung::ModuleDoc | Rung::Decl => 1130.0,
        Rung::Names => 350.0,
        Rung::Doc | Rung::Body => 600.0,
    }
}

/// Down-weight a batch by filesystem depth — depth 0/1 unpenalized.
pub fn depth_factor(depth: usize) -> f64 {
    1.0 / (1.0 + depth.saturating_sub(1) as f64 * 0.35)
}

/// Multiplier for content whose path is under a "non-essential"
/// directory (tests/examples/benches/...) or whose filename signals a
/// test (`*.test.ts`, `*_test.go`). Matched component-wise relative to
/// `root` so the harness's outer `tests/fixtures/` doesn't poison.
pub fn non_essential_factor(path: &std::path::Path, root: &std::path::Path) -> f64 {
    let target = path.strip_prefix(root).unwrap_or(path);
    // Auto-injected docs are already in the model's context. Checked
    // first so SKILL.md / rules.mdc get the stronger 0.1 tier rather
    // than the generic dot-dir 0.2.
    if is_auto_injected_doc_file(path, root) {
        return 0.1;
    }
    // Dot-prefixed dirs at any depth are tooling / CI / admin /
    // docs-site plumbing. Exceptions: `.github/workflows/...` (CI
    // config, paging-relevant) and the auto-injected skill subtrees
    // already handled above.
    let mut comps = target.components();
    if let Some(first) = comps.next().and_then(|c| c.as_os_str().to_str()) {
        if first.eq_ignore_ascii_case(".github") {
            let second = comps.next().and_then(|c| c.as_os_str().to_str());
            if !second.is_some_and(|s| s.eq_ignore_ascii_case("workflows")) {
                return 0.2;
            }
            // The workflows exemption is for what CI says about the
            // project — how it builds, packages and releases. A
            // workflow named for the test suite says only that the
            // test suite runs, which the test tree already said, so it
            // rejoins the tier that tree is on.
            if names_the_test_suite(target) {
                return 0.2;
            }
        } else if first.starts_with('.') && first != "." && !is_dotenv_sample_filename(first) {
            // Dotenv samples are user-facing config documentation,
            // not tooling plumbing — exempt from the dot-prefix damp
            // (cf. the `.github/workflows` exception above).
            return 0.2;
        } else if is_vendor_dir_name(first) {
            // Depth-1-only: a project that vendors *as part of* its
            // own `source/` (chalk) keeps full weight on its vendored
            // modules.
            return 0.2;
        } else if is_docs_site_subtree(first, root) {
            // Separate documentation-site sub-app at the repo root
            // (axios's `docs/package.json`, dockly's `docs/package.json`).
            // The site is build-and-publish plumbing — its sources are
            // peripheral to understanding the parent library. NS authors
            // rank the parent README ahead of the site's own pages,
            // scaffolding, and per-package config. Real user docs
            // (`click/docs/`, `mdbook/docs/` — no nested package.json)
            // keep full weight.
            return 0.2;
        }
    }
    for component in target.components().skip(1) {
        if component
            .as_os_str()
            .to_str()
            .is_some_and(|s| s.starts_with('.') && s != "." && !is_dotenv_sample_filename(s))
        {
            return 0.2;
        }
    }
    // Peripheral admin / release markdown (anywhere in the tree). NS
    // authors treat these as "appendix" content; the walker should not
    // let CHANGELOG, CONTRIBUTING, etc. crowd the primary-source schedule.
    if is_peripheral_doc(target) {
        return 0.2;
    }
    for component in target.components() {
        let Some(raw) = component.as_os_str().to_str() else {
            continue;
        };
        // Case-insensitive: `Tests/`, `Scripts/`, `Examples/` are
        // the spelling in Swift, C#, Objective-C and Java trees,
        // and a role classifier that only knows the lowercase
        // spelling gives those ecosystems' test suites the same
        // weight as their library source.
        let lowered = raw.to_ascii_lowercase();
        let s = dir_role_name(&lowered);
        if matches!(
            s,
            "tests"
                | "test"
                | "testing"
                | "examples"
                | "benches"
                | "benchmark"
                | "benchmarks"
                | "fixtures"
                | "ci"
                | "scripts"
                | "tools"
        ) || s.starts_with("test_")
            || s.starts_with("tests_")
            || s.starts_with("guide-helper")
            || is_scaffold_template_dir_name(s)
        {
            return 0.2;
        }
    }
    // Python under a `docs/` subtree is Sphinx config / site builders /
    // schema validators. (`docs/examples/*.py` survives because the
    // `examples` dir classifier ran first.)
    if path.extension().is_some_and(|ext| ext == "py")
        && target
            .components()
            .any(|c| c.as_os_str().to_str().is_some_and(|s| s == "docs"))
    {
        return 0.3;
    }
    if path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(is_colocated_test_filename)
    {
        return 0.2;
    }
    1.0
}

/// Filename carrying the co-located unit-test convention (`foo.test.js`,
/// `foo.spec.ts`, `foo_test.go`, `test_foo.py`).
fn is_colocated_test_filename(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains(".test.")
        || lower.contains(".test-d.")
        || lower.contains(".spec.")
        || lower.ends_with("_test.go")
        || lower.ends_with("_test.ts")
        || lower.ends_with("_test.js")
        || lower.ends_with("_test.tsx")
        || lower.ends_with("_test.py")
        || (lower.starts_with("test_") && lower.ends_with(".py"))
}

/// True iff `target`'s file stem is exactly `test` / `tests` — the file
/// is named for the test suite and nothing else.
fn names_the_test_suite(target: &std::path::Path) -> bool {
    target
        .file_stem()
        .and_then(|stem| stem.to_str())
        .is_some_and(|stem| stem.eq_ignore_ascii_case("test") || stem.eq_ignore_ascii_case("tests"))
}

/// Checked-in dotenv sample/template filenames. These are config-key
/// documentation with placeholder values — the deploy-facing "what can
/// I configure" surface — distinct from real credential-bearing `.env`
/// files, which stay excluded from every walker.
pub(crate) fn is_dotenv_sample_filename(name: &str) -> bool {
    matches!(
        name,
        ".env.sample" | ".env.example" | ".env.template" | ".env.dist"
    )
}

/// Detect a depth-1 docs-site sub-app — directory named like a docs
/// site that ships its own `package.json` (Docusaurus, VitePress, …).
/// Distinguishes a separate publishing app from inline user docs.
fn is_docs_site_subtree(first_component: &str, root: &std::path::Path) -> bool {
    let lower = first_component.to_ascii_lowercase();
    if !matches!(lower.as_str(), "docs" | "doc" | "site" | "website") {
        return false;
    }
    root.join(first_component).join("package.json").is_file()
}

/// Dirs holding vendored / third-party content. The non-essential
/// discount applies it at depth 1 only, so nested vendored modules under
/// a project's own source tree keep full weight.
pub(crate) fn is_vendor_dir_name(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "deps" | "vendor" | "third_party" | "third-party" | "external" | "3rd" | "sig"
    )
}

/// A directory name read for its *role*, with convention wrapping stripped:
/// `__tests__` reads as `tests`. Dunder wrapping is a naming convention —
/// the JS/TS test tree, its `__snapshots__` and `__mocks__` siblings — not a
/// role of its own.
///
/// Only the wrapping is stripped — a trimmed name still has to *be* a role
/// name, so `_internal` and `__pycache__` go on matching nothing. Case
/// folding stays with the caller: callers matching many role names against
/// one directory fold once, and this borrows from that buffer.
pub(crate) fn dir_role_name(lowercased_name: &str) -> &str {
    lowercased_name.trim_matches('_')
}

/// Starter-template payload directory — the material a project scaffolder
/// copies into a new project (`template-react`, `cra-template-typescript`,
/// as emitted by `create-*` packages). What the scaffolder *is* lives in
/// its own source; the payloads are N near-identical starter projects.
///
/// The prefixed forms only. A bare `templates/` is the view layer in every
/// server framework in the corpus (Django, Flask, Jinja, Helm charts) —
/// demoting that would demote those projects' actual output.
pub(crate) fn is_scaffold_template_dir_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.starts_with("template-") || lower.starts_with("cra-template-")
}

/// Auto-injected agent-instruction files (CLAUDE.md / AGENTS.md /
/// skill docs). Matches anywhere in the tree — CLAUDE.md is recursive.
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
    if let Some(stem) = target.file_stem().and_then(|s| s.to_str())
        && (stem.eq_ignore_ascii_case("AGENTS") || stem.eq_ignore_ascii_case("CLAUDE"))
    {
        return true;
    }
    // Skill / rules subtrees: any text file whose path contains an
    // adjacent `(agent-dir, subdir)` pair, e.g. `.claude/skills`,
    // `.agent/skills`, `.cursor/rules`.
    let parts: Vec<&str> = target
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    parts.windows(2).any(|w| {
        let (a, b) = (w[0], w[1]);
        (a.eq_ignore_ascii_case(".claude") && b.eq_ignore_ascii_case("skills"))
            || (a.eq_ignore_ascii_case(".agent") && b.eq_ignore_ascii_case("skills"))
            || (a.eq_ignore_ascii_case(".cursor") && b.eq_ignore_ascii_case("rules"))
    })
}

/// True for admin/release markdown — CHANGELOG / CONTRIBUTING /
/// SECURITY / NOTICE / RELEASING / etc. —
/// anywhere in the tree (monorepo per-package copies inherit the
/// same admin-doc semantics).
pub fn is_peripheral_doc(target: &std::path::Path) -> bool {
    let Some(ext) = target.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    if !ext.eq_ignore_ascii_case("md") && !ext.eq_ignore_ascii_case("rst") {
        return false;
    }
    let Some(stem) = target.file_stem().and_then(|s| s.to_str()) else {
        return false;
    };
    let stem = stem.replace('-', "_");
    if [
        "CHANGELOG",
        "CHANGES",
        "HISTORY",
        "NEWS",
        "RELEASING",
        "RELEASES",
        "RELEASE_NOTES",
        "CONTRIBUTING",
        "SECURITY",
        "NOTICE",
        "AUTHORS",
        "CODE_OF_CONDUCT",
        "FAQ",
    ]
    .iter()
    .any(|s| stem.eq_ignore_ascii_case(s))
    {
        return true;
    }
    false
}

/// Default cost-side concavity for the scheduling ratio — gentle so
/// big coherent anchor batches stay competitive against many small
/// per-decl batches. Per-key overrides raise this for prose-shaped
/// batches via [`crate::batch::BatchKey::concavity_exponent`].
pub const DEFAULT_CONCAVITY_EXPONENT: f64 = 0.35;

/// Convert a value and a marginal token cost into the scheduling ratio.
/// The scheduler passes `entry.key.concavity_exponent()` so prose-shaped
/// batches see a steeper cost penalty than structural ones.
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

    #[test]
    fn conserved_catalog_chunk_factors_sum_to_one() {
        for costs in [
            vec![120, 130],
            vec![200, 180, 90],
            vec![250, 40, 300, 90, 120],
            vec![100; 8],
        ] {
            let factors = conserved_catalog_chunk_factors(&costs, DEFAULT_CONCAVITY_EXPONENT);
            assert_eq!(factors.len(), costs.len());
            let total: f64 = factors.iter().sum();
            assert!(
                (total - 1.0).abs() < 1e-9,
                "aggregate must be conserved for {costs:?}, got {total}"
            );
            assert!(factors.iter().all(|f| *f > 0.0 && *f < 1.0));
        }
    }

    #[test]
    fn conserved_catalog_chunk_factors_head_premium_is_bounded() {
        let k = DEFAULT_CONCAVITY_EXPONENT;
        let head_share: f64 = 1.0 / 3.0;
        let factors = conserved_catalog_chunk_factors(&[100, 100, 100], k);
        // Head = share^k (ratio parity with the unsplit catalog) times
        // the bounded premium, capped at the first-chunk tier.
        let expected = (head_share.powf(k) * CATALOG_HEAD_PREMIUM).min(CATALOG_HEAD_FACTOR_CAP);
        assert!((factors[0] - expected).abs() < 1e-9);
        assert!(factors[0] >= head_share.powf(k));
        assert!(factors[0] <= CATALOG_HEAD_FACTOR_CAP);
        // Tails decay in train order at equal cost.
        assert!(factors[0] > factors[1] && factors[1] > factors[2]);
    }

    #[test]
    fn conserved_catalog_chunk_factors_degenerate_cases() {
        let k = DEFAULT_CONCAVITY_EXPONENT;
        assert_eq!(conserved_catalog_chunk_factors(&[], k), Vec::<f64>::new());
        assert_eq!(conserved_catalog_chunk_factors(&[500], k), vec![1.0]);
        assert_eq!(conserved_catalog_chunk_factors(&[0, 0], k), vec![0.5, 0.5]);
        // A zero-cost tail set leaves the whole remainder split evenly.
        let head_only = conserved_catalog_chunk_factors(&[200, 0, 0], k);
        assert!((head_only.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert!((head_only[1] - head_only[2]).abs() < 1e-9);
    }

    /// Single table-driven driver. Each row is `(expected, paths)`
    /// where `paths` is space-separated (paths contain no spaces). The
    /// inline comment above each row names the rule under test.
    #[test]
    fn value_non_essential_factor_cases() {
        let cases: &[(f64, &str)] = &[
            // Underscore-prefixed files keep full weight.
            (
                1.0,
                "src/pluggy/_hooks.py src/pluggy/__init__.py src/pkg/__main__.py \
                 pages/_app.tsx packages/x/_routes.json src/_helper.rs",
            ),
            // GitHub: contributor templates demote; workflows keep full
            // weight; dependabot config demotes.
            (
                0.2,
                ".github/PULL_REQUEST_TEMPLATE.md .github/pull_request_template.md \
                 .github/ISSUE_TEMPLATE/bug.md",
            ),
            (1.0, ".github/workflows/ci.yml .github/workflows"),
            (0.2, ".github/dependabot.yml"),
            // Top-level peripheral docs (admin / release / governance).
            (
                0.2,
                "CHANGELOG.md changelog.rst HISTORY.md RELEASE_NOTES.md RELEASING.md \
                 CONTRIBUTING.md SECURITY.md NOTICE.md AUTHORS.md CODE_OF_CONDUCT.md \
                 NEWS.md news.rst FAQ.md",
            ),
            // Monorepo per-package CHANGELOGs etc. inherit admin-doc semantics.
            (
                0.2,
                "docs/changelog.md docs/CONTRIBUTING.md \
                 packages/d2mini/CHANGELOG.md subproject/CHANGELOG.md",
            ),
            // Auto-injected agent docs: recursive (nested copies in monorepos
            // still auto-inject).
            (
                0.1,
                "AGENTS.md agents.md CLAUDE.md claude.rst \
                 .claude/skills/foo/SKILL.md .agent/skills/bar/instructions.md \
                 .cursor/rules/baz.mdc packages/foo/CLAUDE.md crates/bar/AGENTS.md",
            ),
            // Root dot-directories (IDE / tooling / CI / admin / skills).
            (
                0.2,
                ".claude/skills .claude/skills/foo .claude/skills/foo/script.py \
                 .claude/skills/foo/data.json .vscode/settings.json \
                 .devcontainer/devcontainer.json .idea/foo.xml .husky/pre-commit \
                 .circleci/config.yml .cargo/config.toml .yarn/plugins/foo.cjs \
                 .faq/FAQ.md",
            ),
            // Changesets subtree: per-package release-note staging area.
            (
                0.2,
                ".changeset/foo.md .changeset/README.md .changeset/config.json",
            ),
            // `contribute/` is intentionally NOT demoted (mcphost-only,
            // non-markdown subtree).
            (
                1.0,
                "contribute/contribute.md contribute/build.sh contribute/conf/demo.json",
            ),
            // Scaffolder payloads: the prefixed spellings only. A bare
            // `templates/` is the view layer in Django / Flask / Jinja /
            // Helm trees and keeps full weight.
            (
                0.2,
                "packages/create-x/template-react/src/main.tsx \
                 packages/create-x/cra-template-typescript/index.js \
                 packages/create-x/Template-Vue/vite.config.ts",
            ),
            (
                1.0,
                "app/templates/base.html src/template/driver.c pkg/x/templates/deploy.yaml",
            ),
            // Dunder wrapping is read through: the JS test tree lands in
            // the same tier as `test/`. Wrapping alone demotes nothing.
            (
                0.2,
                "src/node/__tests__/serve.ts src/node/__tests__/__snapshots__/x.snap",
            ),
            (1.0, "src/__internal__/queue.ts src/pkg/__pycache__/x.pyc"),
        ];
        let root = Path::new("/repo");
        for (expected, paths) in cases {
            for path in paths.split_ascii_whitespace() {
                assert_eq!(
                    non_essential_factor(&root.join(path), root),
                    *expected,
                    "{path}",
                );
            }
        }
    }
}
