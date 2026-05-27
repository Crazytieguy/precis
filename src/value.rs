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

/// Chunk size for names-surface gates — roughly one screenful of API.
pub const NAMES_SURFACE_CHUNK_SIZE: usize = 12;

/// First-chunk factor (slightly below unchunked) so small unchunked
/// files win comparable rank races.
const CHUNKED_NAMES_FIRST_CHUNK_FACTOR: f64 = 0.9;
/// Falloff per later chunk — 0.25 puts chunk 4 at ~half chunk 1.
const CHUNKED_NAMES_FALLOFF: f64 = 0.25;

pub fn names_surface_chunk_count(dependent_count: usize) -> usize {
    dependent_count.div_ceil(NAMES_SURFACE_CHUNK_SIZE)
}

pub fn names_surface_chunk_index(item_index: usize) -> usize {
    item_index / NAMES_SURFACE_CHUNK_SIZE
}

/// Per-chunk multiplier for a chunked names-surface batch.
pub fn names_surface_chunk_factor(chunk_index: usize, chunk_count: usize) -> f64 {
    if chunk_count <= 1 {
        1.0
    } else {
        CHUNKED_NAMES_FIRST_CHUNK_FACTOR / (1.0 + chunk_index as f64 * CHUNKED_NAMES_FALLOFF)
    }
}

/// Per-chunk multiplier for re-export-wall chunks. First chunk keeps
/// full value; later groups fall off similar to names-surface chunks.
pub fn reexport_import_chunk_factor(chunk_index: usize, chunk_count: usize) -> f64 {
    if chunk_count <= 1 {
        1.0
    } else {
        1.0 / (1.0 + chunk_index as f64 * 0.2)
    }
}

/// Down-weight a batch by filesystem depth — depth 0/1 unpenalized.
pub fn depth_factor(depth: usize) -> f64 {
    1.0 / (1.0 + depth.saturating_sub(1) as f64 * 0.3)
}

/// Multiplier for content whose path is under a "non-essential"
/// directory (tests/examples/benches/...) or whose filename signals a
/// test (`*.test.ts`, `*_test.go`). Matched component-wise relative to
/// `root` so the harness's outer `tests/fixtures/` doesn't poison.
pub fn non_essential_factor(path: &std::path::Path, root: &std::path::Path) -> f64 {
    non_essential_factor_inner(path, root, false)
}

pub(crate) fn non_essential_factor_inner(
    path: &std::path::Path,
    root: &std::path::Path,
    skip_dir_classifier: bool,
) -> f64 {
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
        } else if first.starts_with('.') && first != "." {
            return 0.2;
        } else if is_root_level_vendor_dir_name(first) {
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
            .is_some_and(|s| s.starts_with('.') && s != ".")
        {
            return 0.2;
        }
    }
    // Peripheral admin / release / translation markdown (anywhere in
    // the tree). NS authors universally treat these as "appendix"
    // content; the walker should not let CHANGELOG, CONTRIBUTING, etc.
    // crowd the primary-source schedule.
    if is_peripheral_doc(target) || is_localized_readme(target) {
        return 0.2;
    }
    if !skip_dir_classifier {
        for component in target.components() {
            let Some(s) = component.as_os_str().to_str() else {
                continue;
            };
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
                    | "rfcs"
                    | "xtask"
                    | "ci"
                    | "website"
                    | "demo"
                    | "playground"
                    | "storybook"
                    | "fuzz"
                    | "fuzzer"
                    | "profiler"
                    | "scripts"
                    | "tools"
                    | "e2e"
                    | "migrations"
            ) || s.starts_with("test_")
                || s.starts_with("tests_")
                || s.starts_with("guide-helper")
                || is_proc_macro_crate_dir_name(s)
            {
                return 0.2;
            }
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
        // Python under a `docs/` subtree is Sphinx config / site
        // builders / schema validators. (`docs/examples/*.py` survives
        // because the `examples` dir classifier ran first.)
        if ext == Some("py")
            && target
                .components()
                .any(|c| c.as_os_str().to_str().is_some_and(|s| s == "docs"))
        {
            return 0.3;
        }
        // Co-located test file conventions: `*.test.*`, `*.spec.*`,
        // `*_test.<go|ts|js|tsx|py>`, `test_*.py`.
        let lower = name.to_ascii_lowercase();
        if lower.contains(".test.")
            || lower.contains(".test-d.")
            || lower.contains(".spec.")
            || lower.ends_with("_test.go")
            || lower.ends_with("_test.ts")
            || lower.ends_with("_test.js")
            || lower.ends_with("_test.tsx")
            || lower.ends_with("_test.py")
            || (lower.starts_with("test_") && lower.ends_with(".py"))
        {
            return 0.2;
        }
    }
    1.0
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

/// Root-level dirs holding vendored / third-party content. Depth-1
/// only so nested vendored modules under a project's own source tree
/// keep full weight.
fn is_root_level_vendor_dir_name(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "deps" | "vendor" | "third_party" | "third-party" | "external" | "3rd" | "sig"
    )
}

/// Rust proc-macro helper-crate convention (`<name>-macros` etc.).
/// Plain `macros`/`derive` excluded — those are usually real modules.
fn is_proc_macro_crate_dir_name(s: &str) -> bool {
    for suffix in ["-macros", "_macros", "-derive", "_derive"] {
        if let Some(stem) = s.strip_suffix(suffix)
            && !stem.is_empty()
        {
            return true;
        }
    }
    false
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
/// SECURITY / NOTICE / RELEASING / migration-guide stems / etc. —
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
    // Migration / upgrade / deprecation docs are content describing
    // historical API changes — necessary at version-bump time but rarely
    // load-bearing for orienting on the current API. NS authors
    // universally treat these as tier-2 reference at best. Matched as
    // substrings so `v3-to-v4-migration-guide.md`, `migrate-from-foo.md`,
    // `deprecated.md`, `deprecation-policy.rst`, `UPGRADE_GUIDE_V2.md`
    // are all caught.
    let lower = stem.to_ascii_lowercase();
    if lower.contains("migration")
        || lower.contains("migrate")
        || lower.contains("deprecated")
        || lower.contains("upgrade")
    {
        return true;
    }
    false
}

/// Project-orientation markdown files — the matklad-style
/// `ARCHITECTURE.md` convention plus the related `OVERVIEW.md` /
/// `DESIGN.md` / `STRUCTURE.md` names, matched at any depth so a
/// `docs/ARCHITECTURE.md` counts the same as a root-level one. NS
/// authors anchor on these as primary orientation alongside the README;
/// the markdown walker depth-pins them and lifts their section cat to
/// match.
pub fn is_orientation_doc(file: &std::path::Path) -> bool {
    let Some(stem) = file.file_stem().and_then(|s| s.to_str()) else {
        return false;
    };
    let Some(ext) = file.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    if !ext.eq_ignore_ascii_case("md") {
        return false;
    }
    ["ARCHITECTURE", "OVERVIEW", "DESIGN", "STRUCTURE"]
        .iter()
        .any(|n| stem.eq_ignore_ascii_case(n))
}

/// True for `README.<locale>.<ext>` or `Readme_<locale>.<ext>`
/// anywhere in the tree, where `<locale>` is a known ISO-639-style
/// code. The whitelist (rather than a
/// `[a-z]{2,3}([-_][A-Z]{2,4})?` regex) is what rules out
/// `README.api.md` / `README.dev.md` / `README.old.md`.
pub fn is_localized_readme(target: &std::path::Path) -> bool {
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
    // We accept both `<lang>-<REGION>` and `<lang>_<REGION>` forms;
    // normalize the separator before matching. Comparison is
    // case-insensitive because the caller already lowercased the stem.
    let normalized = s.replace('_', "-");
    let lang_root = normalized
        .split_once('-')
        .map_or(normalized.as_str(), |(l, r)| {
            if r.is_empty() { normalized.as_str() } else { l }
        });
    is_locale_language(lang_root)
}

/// Accept any 2–3-character ASCII-letter token that isn't a known
/// false-positive stem (`api` / `dev` / `old` / `template` / etc.).
/// A whitelist of ~50 ISO codes can never keep up with new ones in
/// the wild; the blocklist of non-locale README suffixes is small and
/// stable.
fn is_locale_language(s: &str) -> bool {
    let len = s.len();
    if !(2..=3).contains(&len) || !s.bytes().all(|b| b.is_ascii_lowercase()) {
        return false;
    }
    !matches!(
        s,
        "api" | "dev" | "old" | "new" | "min" | "tmp" | "bak" | "pre"
    )
}

/// Default cost-side concavity for the scheduling ratio — gentle so
/// big coherent anchor batches stay competitive against many small
/// per-decl batches. Per-key overrides raise this for prose-shaped
/// batches via [`crate::batch::WalkerKey::concavity_exponent`].
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

    /// Single table-driven driver. Each row is `(expected, paths)`
    /// where `paths` is space-separated (paths contain no spaces). The
    /// inline comment above each row names the rule under test.
    #[test]
    fn value_non_essential_factor_cases() {
        let cases: &[(f64, &str)] = &[
            // Rust underscore-prefix discount: only `inner.rs` and
            // `__*.rs` discount; single-underscore `.rs` and Python/JS
            // framework underscore files keep full weight.
            (0.5, "src/__private_api.rs src/inner.rs"),
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
                 NEWS.md news.rst FAQ.md UPGRADE_GUIDE_V2.md upgrade-guide.md",
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
            // Localized READMEs: `[a-z]{2,3}(-[A-Z]{2,4})?` locale suffix
            // demotes; non-locale suffixes (`api`, `dev`, `old`, `template`)
            // and bare READMEs keep full weight. Subdir applies too.
            (
                0.2,
                "README.zh-CN.md Readme_zh-CN.md README.ja.md README.pt_BR.rst \
                 README.fr.md README.en-US.md README.cn.md README.kr.md README.fa.md \
                 README.de-ch.md README.pt-pt.md README.es-mx.md",
            ),
            (
                1.0,
                "README.api.md README.dev.md README.old.md README_template.md",
            ),
            (1.0, "README.md README.rst Readme.md"),
            (0.2, "docs/README.zh-CN.md"),
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
