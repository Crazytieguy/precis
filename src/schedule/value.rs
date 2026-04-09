use std::path::Path;

use super::{
    FileCategory, FileRole, Group, KindCategory, StageKind,
};

/// Compute the effective depth of a path, skipping conventional source root
/// directories (`src`, `lib`, `packages`, `crates`, etc.) that don't represent
/// meaningful hierarchy. For monorepos, also skips an inner root after the
/// package name (e.g., `packages/foo/src/` → effective depth 1, not 3).
fn effective_depth(parent_dir: &Path) -> usize {
    let is_root = |s: &str| matches!(s, "src" | "source" | "lib" | "pkg" | "cmd" | "internal" | "app" | "packages" | "crates");
    let components: Vec<_> = parent_dir.components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    match components.as_slice() {
        [] => 0,
        [first, ..] if is_root(first) => {
            let rest = &components[1..];
            // Monorepo: packages/foo/src/bar → skip packages + src, keep foo + bar
            if rest.len() >= 2 && is_root(rest[1]) {
                rest.len() - 1 // skip the inner root
            } else {
                rest.len()
            }
        }
        _ => components.len(),
    }
}

/// Compute the value of showing a particular stage for a group.
pub(super) fn compute_value(group: &Group, stage: StageKind, n: usize) -> f64 {
    let key = &group.key;

    let visibility = if key.is_public { 1.0 } else { 0.3 };
    // Sections (markdown headings, TOML/JSON/YAML sections) ARE documentation —
    // they don't have doc comments but that doesn't make them less important.
    let documented = if key.is_documented || key.kind_category == KindCategory::Section {
        1.0
    } else {
        0.5
    };

    // Compute effective depth, skipping conventional source root directories
    // that are purely organizational. Without this, files under src/ would be
    // penalized relative to root-level files despite being more valuable.
    let depth = effective_depth(&key.parent_dir);
    let depth_factor = match depth {
        0..=1 => 1.0,
        2..=3 => 0.7,
        _ => 0.4,
    };

    // File role: README and architecture docs are high-signal at the project root.
    // Non-root READMEs (docs/README.md, examples/README.md) are typically
    // setup instructions or theme templates, not the project description.
    let is_root_dir = key.parent_dir.as_os_str().is_empty();
    let file_role_factor = match key.file_role {
        FileRole::Architecture if is_root_dir => 1.5,
        FileRole::Readme if is_root_dir => 1.5,
        FileRole::Architecture | FileRole::Readme => 1.0,
        FileRole::Normal => 1.0,
        FileRole::Translated => 0.1,
        FileRole::Changelog => 0.1,
        FileRole::CommunityHealth => 0.1,
        FileRole::AiConfig => 0.1,
    };

    // Config files (eslint.config.js, pyproject.toml, etc.) are build/tool setup,
    // not core library logic. 0.2 allows section names to appear (project name,
    // dependencies) while keeping body content (classifiers, URLs) deprioritized.
    let config_factor = if key.is_config { 0.2 } else { 1.0 };

    // File category: examples show how to use the library (moderately valuable
    // but shouldn't dominate over core library code), docs site source is
    // supplementary, test/CI infrastructure is low-signal.
    let file_category_factor = match key.file_category {
        FileCategory::Source => 1.0,
        FileCategory::Example => 0.35,
        FileCategory::DocsSite => 0.2,
        FileCategory::Test => 0.15,
        FileCategory::CiConfig => 0.1,
    };

    // TypeScript declaration files (.d.ts) duplicate API signatures already
    // shown from .js/.ts source files. Deprioritize so source files win.
    let type_declaration_factor = if key.is_type_declaration { 0.15 } else { 1.0 };

    // C/C++ header files define the public API. In C, there's no `pub`
    // keyword — the convention is that .h files ARE the public interface.
    // Boost headers significantly so they win over implementation structs
    // from .c files (which can be enormous in amalgamation patterns).
    let header_factor = if key.is_header { 2.5 } else { 1.0 };

    // Heading depth: top-level headings (h1, h2) carry structural value,
    // deeper headings are increasingly detail. Aggressive reduction at h3+
    // prevents large READMEs (with detailed API reference sections) from
    // consuming the entire budget with heading structure.
    let heading_depth_factor = match key.heading_depth {
        Some(1) => 1.0,
        Some(2) => 0.6,
        Some(3) => 0.15,
        Some(_) => 0.08, // h4, h5, h6
        None => 1.0,     // non-section symbols
    };

    // Trait implementation methods (Rust `impl Trait for Type { fn fmt ... }`)
    // implement an interface defined elsewhere. They're lower signal than
    // inherent methods — the trait definition already documents the API, and
    // most trait impls (Display::fmt, From::from, Clone::clone) are boilerplate.
    let trait_impl_factor = if key.is_trait_impl { 0.3 } else { 1.0 };

    // Boilerplate markdown sections (License, Contributing, etc.) appear in
    // almost every README with zero architectural value.
    let boilerplate_factor = if key.is_boilerplate_section { 0.1 } else { 1.0 };

    // Auto-generated files (codegen, protobuf, mockery, auto-generated API docs)
    // duplicate source-of-truth definitions and should be deprioritized.
    let generated_factor = if key.is_generated { 0.1 } else { 1.0 };

    // Rust `pub use` re-exports from child modules (e.g. `pub use self::foo::*`,
    // `pub use bar::Baz` where `mod bar` is declared). These are redundant when
    // precis also shows the submodule's own symbols.
    let reexport_factor = if key.is_reexport { 0.1 } else { 1.0 };

    let base_value = visibility * documented * depth_factor
        * file_role_factor * config_factor * file_category_factor * type_declaration_factor * header_factor
        * heading_depth_factor * trait_impl_factor
        * boilerplate_factor * generated_factor * reexport_factor;

    let stage_value = match key.kind_category {
        // Type/Enum bodies (struct fields, enum variants, interface props)
        // define data schemas — architecturally important content. High Body
        // value + gentle n-decay (below) ensures fields/variants compete
        // with cheap function signatures. Enum variants get slightly higher
        // Body value because they define type taxonomies.
        KindCategory::Type | KindCategory::Enum => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 1.0,
            StageKind::Signatures => 0.7,
            StageKind::Body => if key.kind_category == KindCategory::Enum { 1.5 } else { 1.2 },
            StageKind::Doc => 0.4,
        },
        // Section body is the actual content — README prose, architecture
        // descriptions, heading text. Higher than default Body because section
        // content IS the documentation (vs code body which is implementation).
        KindCategory::Section => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 1.0,
            StageKind::Body => 0.7,
            _ => 0.1,
        },
        // Imports: the Names/Signatures model is a poor fit here (see go.scm).
        // Import "names" are often just keywords (`import`) or truncated module
        // paths (`from ._checkers …`) — not meaningful on their own. Set Names
        // to 0 so imports only appear when Signatures is reached (full line).
        // For 1st-party imports, the full line reveals exported symbols.
        KindCategory::Import => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 0.0,
            StageKind::Signatures => if key.is_first_party { 1.0 } else { 0.1 },
            _ => 0.1,
        },
        // Constants: signature captures the value for short constants;
        // multi-line bodies are usually data literals (large sets, dicts,
        // lookup tables) where the name tells you everything.
        KindCategory::Constant => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 1.0,
            StageKind::Signatures => 0.7,
            StageKind::Doc => 0.5,
            StageKind::Body => 0.05,
        },
        _ => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 1.0,
            StageKind::Signatures => 0.7,
            StageKind::Doc => 0.5,
            StageKind::Body => 0.2,
        },
    };

    // Non-public symbols: deprioritize detail beyond names. At tight budgets,
    // private/internal symbol names are shown (cheap, useful for understanding
    // structure), but budget is reserved for public symbol signatures, bodies,
    // and documentation. Without this, small groups of private symbols reach
    // Signatures/Body before large groups of public symbols reach Names.
    let private_detail_penalty = if !key.is_public
        && matches!(stage, StageKind::Signatures | StageKind::Doc | StageKind::Body)
    {
        0.2
    } else {
        1.0
    };

    // Names and Signatures stages: cost scales linearly with group size
    // (each symbol adds its name/signature tokens). Without compensation,
    // large groups (e.g. a class with 50 methods) get extremely low
    // priority because priority = value / cost, and cost grows with N
    // while value was constant. Scale value by N so all groups have the
    // same per-token priority at Names/Signatures regardless of size.
    //
    // Scale Names/Signatures value by symbol_count^0.75. Each symbol
    // name revealed has value, so larger groups should have higher total
    // value — but sub-linear scaling provides a moderate penalty for
    // large homogeneous groups, reflecting diminishing information as
    // the pattern becomes clear.
    //
    // Impact: 1 → 1.0, 4 → 2.8, 10 → 5.6, 25 → 11.2, 50 → 18.8, 100 → 31.6
    let sym_count = group.symbols.len().max(1) as f64;
    let count_factor = match stage {
        StageKind::Names | StageKind::Signatures => sym_count.powf(0.75),
        _ => 1.0,
    };

    // Enum variant lists and type field definitions: each line is
    // independently valuable. Use very gentle linear decay instead of
    // 1/n so these lines stay competitive with cheap items from other
    // groups (e.g., function signatures at 0.7 stage_value).
    //
    // Section body (README paragraphs, Architecture descriptions): each
    // line carries independent information. Steeper than types (0.1 vs
    // 0.05) since prose is more verbose than struct field definitions,
    // but gentler than 1/n so README intros and Architecture overviews
    // show 5-10 useful lines instead of truncating after 1-2.
    let n_decay = match (key.kind_category, stage) {
        (KindCategory::Enum | KindCategory::Type, StageKind::Body) => {
            1.0 + 0.07 * (n as f64 - 1.0)
        }
        (KindCategory::Section, StageKind::Body) => {
            1.0 + 0.2 * (n as f64 - 1.0)
        }
        _ => n as f64,
    };

    base_value * stage_value * private_detail_penalty * count_factor / n_decay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_depth_skips_source_roots() {
        assert_eq!(effective_depth(Path::new("")), 0);
        assert_eq!(effective_depth(Path::new("src")), 0);
        assert_eq!(effective_depth(Path::new("lib")), 0);
        assert_eq!(effective_depth(Path::new("pkg")), 0);
        assert_eq!(effective_depth(Path::new("cmd")), 0);
        assert_eq!(effective_depth(Path::new("src/pluggy")), 1);
        assert_eq!(effective_depth(Path::new("lib/internal")), 1);
        assert_eq!(effective_depth(Path::new("src/a/b")), 2);
        // Additional conventional roots
        assert_eq!(effective_depth(Path::new("internal")), 0);
        assert_eq!(effective_depth(Path::new("internal/pkg")), 1);
        assert_eq!(effective_depth(Path::new("packages")), 0);
        // Monorepo double-root: packages/ + src/ both skipped
        assert_eq!(effective_depth(Path::new("packages/foo/src")), 1);
        assert_eq!(effective_depth(Path::new("crates/my-crate/src/utils")), 2);
        assert_eq!(effective_depth(Path::new("crates/core")), 1);
        // Non-source-root first components are not skipped
        assert_eq!(effective_depth(Path::new("docs")), 1);
        assert_eq!(effective_depth(Path::new("scripts")), 1);
        assert_eq!(effective_depth(Path::new("docs/conf")), 2);
    }
}
