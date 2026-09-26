//! Value-model helpers shared across walkers: depth pricing, non-essential
//! discounts, and the cost-side concavity used by the scheduler. Walkers
//! compute their batch's `value: f64` directly; the scheduler ranks by
//! [`ratio_with_exponent`].

/// Value of a manifest's runtime dependency roster, shared by every
/// manifest format. What a package depends on is a primary statement of
/// what it *is* — a database driver, an HTTP client, a template engine.
pub fn dependency_roster_value(depth: f64) -> f64 {
    854.0 * depth
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

/// Roster size at which [`roster_mass`] is neutral; rosters this small
/// already rank acceptably without help.
const ROSTER_MASS_BASELINE: f64 = 11.0;

/// Ratio-neutralizing factor for "roster" batches — complete catalogs of
/// N peer entries (directory listings, names surfaces, member catalogs)
/// whose value is otherwise size-invariant while cost grows linearly
/// with N, so `value/cost^k` systematically prefers tiny rosters over
/// the complete catalogs NS authors anchor on. Scaling value by
/// `(N / baseline)^k`, `k` = [`DEFAULT_CONCAVITY_EXPONENT`], pushes back
/// against that. Boost-only (≥ 1): small rosters keep their existing
/// rank rather than being demoted.
pub fn roster_mass(entries: usize) -> f64 {
    (entries as f64 / ROSTER_MASS_BASELINE)
        .powf(DEFAULT_CONCAVITY_EXPONENT)
        .max(1.0)
}

/// Base value of each code-engine rung (`walker::code`), before the file
/// prior and the chunk share: one value for what a file or declaration
/// is (module doc, declaration), one for its depth (doc, body), and the
/// roster's per-row value. The engine scales `Names` by `rows^k` (`k`
/// the default concavity exponent), so a roster's scheduling ratio
/// depends on its tokens per row, not on its length, and a `Whole`
/// declaration's `Decl` (fields, entries, a container's member roster) by
/// [`roster_mass`] of its body entries. `Names` is by far the most
/// grid-sensitive of the three.
pub fn code_rung_value(rung: crate::batch::Rung) -> f64 {
    use crate::batch::Rung;
    match rung {
        Rung::ModuleDoc | Rung::Decl => 1130.0,
        Rung::Names => 350.0,
        Rung::Doc | Rung::Body => 360.0,
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
    let names: Vec<Option<&str>> = target
        .components()
        .map(|component| component.as_os_str().to_str())
        .collect();
    for (index, name) in names.iter().enumerate() {
        let Some(name) = *name else {
            continue;
        };
        // Dot-prefixed entries at any depth are tooling / CI / admin /
        // docs-site plumbing, except `.github/workflows/...` (CI config).
        let is_ci_config = index == 0
            && name.eq_ignore_ascii_case(".github")
            && names
                .get(1)
                .copied()
                .flatten()
                .is_some_and(|second| second.eq_ignore_ascii_case("workflows"));
        let is_dotted = name.starts_with('.') && name != "." && !is_ci_config;
        // Case-insensitive: `Tests/`, `Scripts/`, `Examples/` are
        // the spelling in Swift, C#, Objective-C and Java trees,
        // and a role classifier that only knows the lowercase
        // spelling gives those ecosystems' test suites the same
        // weight as their library source.
        let lowered = name.to_ascii_lowercase();
        let role = dir_role_name(&lowered);
        let is_non_essential_role = matches!(
            role,
            "tests"
                | "test"
                | "testing"
                | "examples"
                | "benches"
                | "bench"
                | "spec"
                | "specs"
                | "benchmark"
                | "benchmarks"
                | "fixtures"
                | "testdata"
                | "testutil"
                | "testutils"
                | "mock"
                | "mocks"
                | "harness"
                | "harnesses"
                | "smoketest"
                | "smoketests"
                | "perf"
                | "ci"
                | "scripts"
                | "tools"
                | "archived"
                | "locales"
                | "l10n"
                | "translations"
        ) || is_third_party_role(role)
            || role.starts_with("test_")
            || role.starts_with("tests_")
            || is_scaffold_template_dir_name(&lowered)
            // Python under `docs/` is Sphinx config and site builders.
            || name == "docs" && path.extension().is_some_and(|ext| ext == "py");
        // Past `com/` or `org/` a JVM package is named for its publisher:
        // `com.example` is the templates' default, `com.google.samples`
        // an organization, and neither is a sample tree.
        let is_sample_tree = matches!(role, "example" | "sample" | "samples")
            && !names[..index]
                .iter()
                .any(|name| matches!(name, Some("com" | "org")));
        // A separate documentation-site sub-app at the repo root
        // (axios's `docs/package.json`, dockly's `docs/package.json`) is
        // build-and-publish plumbing, peripheral to the parent library.
        // Real user docs (`click/docs/`, `mdbook/docs/` — no nested
        // package.json) keep full weight. A root `support/` holds release
        // scripts, editor syntaxes and test certificates; further down it
        // can name a module of the project's own (`src/support/`).
        if is_dotted
            || is_non_essential_role
            || is_sample_tree
            || index == 0
                && (is_vendor_dir_name(name)
                    || lowered == "support"
                    || is_docs_site_subtree(name, root))
        {
            return 0.2;
        }
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

/// Filename carrying a co-located unit-test convention (`foo.test.js`,
/// `foo.spec.ts`, `foo_test.go`, `foo_spec.rb`, `test_foo.py`,
/// `FooTest.java`, `FooTests.cs`, `FooSpec.scala`).
fn is_colocated_test_filename(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let stem = name.rsplit_once('.').map_or(name, |(stem, _)| stem);
    let camel_case_suffix = ["Test", "Tests", "Spec", "Specs"].iter().any(|suffix| {
        stem.strip_suffix(suffix).is_some_and(|subject| {
            subject
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
    });
    lower.contains(".test.")
        || lower.contains(".test-d.")
        || lower.contains(".spec.")
        || stem.to_ascii_lowercase().ends_with("_test")
        || stem.to_ascii_lowercase().ends_with("_spec")
        || (lower.starts_with("test_") && lower.ends_with(".py"))
        || camel_case_suffix
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

/// A directory of other projects' code: a third-party directory at any
/// depth, or a vendor directory at the root.
pub(crate) fn is_third_party_dir(dir: &std::path::Path, root: &std::path::Path) -> bool {
    let Ok(relative) = dir.strip_prefix(root) else {
        return false;
    };
    let Some(name) = relative.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    is_third_party_role(dir_role_name(&name.to_ascii_lowercase()))
        || relative.components().count() == 1 && is_vendor_dir_name(name)
}

fn is_third_party_role(role: &str) -> bool {
    matches!(
        role,
        "third_party" | "third-party" | "thirdparty" | "3rdparty"
    )
}

/// Dirs that hold vendored content at the root but can name a project's
/// own module further down (a USB `vendor` class, chalk's
/// `source/vendor`), so the non-essential discount applies them at depth
/// 1 only. Unambiguous third-party names apply at any depth.
fn is_vendor_dir_name(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "deps" | "vendor" | "external" | "3rd" | "sig" | "dependencies"
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
fn dir_role_name(lowercased_name: &str) -> &str {
    lowercased_name.trim_matches('_')
}

/// Starter-template payload directory — the material a project scaffolder
/// copies into a new project (`template-react`, `cra-template-typescript`,
/// as emitted by `create-*` packages, or a Mason brick's `__brick__`).
/// What the scaffolder *is* lives in its own source; the payloads are N
/// near-identical starter projects, and not always in the language their
/// names claim.
///
/// The prefixed forms only. A bare `templates/` is the view layer in every
/// server framework in the corpus (Django, Flask, Jinja, Helm charts) —
/// demoting that would demote those projects' actual output.
fn is_scaffold_template_dir_name(lowered: &str) -> bool {
    lowered.starts_with("template-")
        || lowered.starts_with("cra-template-")
        || lowered == "__brick__"
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
            // Root dot-directories (IDE / tooling / CI / admin / skills).
            (
                0.2,
                ".claude/skills/foo/SKILL.md .cursor/rules/baz.mdc \
                 .claude/skills .claude/skills/foo .claude/skills/foo/script.py \
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
            // Example and sample trees, in either case, and a root
            // `support/`.
            (
                0.2,
                "Example/AppDelegate.m samples/Polly.Samples/Program.cs \
                 sample/app.go \
                 app/src/test/java/com/example/FooTest.java support/release.py",
            ),
            // A JVM package named for its publisher.
            (
                1.0,
                "app/src/main/kotlin/com/google/samples/apps/Main.kt \
                 src/main/java/org/springframework/samples/petclinic/Owner.java \
                 src/main/java/com/example/demo/DemoApplication.java src/support/mod.rs",
            ),
            // Scaffolder payloads: the prefixed spellings only. A bare
            // `templates/` is the view layer in Django / Flask / Jinja /
            // Helm trees and keeps full weight.
            (
                0.2,
                "packages/create-x/template-react/src/main.tsx \
                 packages/create-x/cra-template-typescript/index.js \
                 packages/create-x/Template-Vue/vite.config.ts \
                 bricks/bloc/__brick__/{{name}}_bloc.dart",
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
            // Test-file and test-tree conventions of languages the
            // fallback renders.
            (
                0.2,
                "db/table_test.cc src/aof_test.zig sinatra-contrib/spec/x.rb lib/cache_spec.rb \
                 src/EmailValidatorTest.java src/PollyTests.cs src/FunctorSpec.scala \
                 bench/Polly.Benchmarks/Program.cs",
            ),
            (
                1.0,
                "src/Latest.java src/Contest.kt src/Test.java lib/specification.rb src/Spec.hs",
            ),
            // Archived, translated and third-party copies, at any depth.
            (
                0.2,
                "_archived/guestbook/app.yaml samples/archived/x.cs \
                 apps/web/public/locales/en/common.json lessons/1/translations/README.es.md \
                 src/third_party/zlib/zlib.h lib/3rdParty/x.c \
                 dependencies/camlzip/zip.ml",
            ),
            // `vendor/`, `locale/` and `*-master/` below the root can be
            // the project's own module.
            (
                1.0,
                "src/class/vendor/vendor_device.c source/vendor/ansi-styles/index.js \
                 racket/src/io/locale/parameter.rkt drivers/i2c-master/i2c.c",
            ),
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
