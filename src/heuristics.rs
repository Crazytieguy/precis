//! All base_value() and contribution() functions for the group taxonomy.
//! This is the single findable location for all value heuristics (design §9).

use crate::classify::{self, FileCategory, FileRole};
use crate::group::TsGroupKey;

use std::path::Path;

// ---------------------------------------------------------------------------
// Base value — computed per group from its items
// ---------------------------------------------------------------------------

/// Compute the base value of a Folders group.
/// Sublinear in item count: additional folders add diminishing information.
pub fn folders_base_value(item_count: usize) -> f64 {
    (item_count as f64).powf(0.75) * 0.3
}

/// Compute the base value of a Files group.
pub fn files_base_value(item_count: usize, role: FileRole) -> f64 {
    let role_factor = match role {
        FileRole::Architecture => 1.5,
        FileRole::Readme => 1.5,
        FileRole::Normal => 1.0,
        FileRole::Translated => 0.1,
        FileRole::Changelog => 0.1,
        FileRole::CommunityHealth => 0.1,
        FileRole::AiConfig => 0.1,
    };
    (item_count as f64).powf(0.75) * 0.3 * role_factor
}

/// Compute the base value of a tree-sitter group.
pub fn ts_base_value(key: &TsGroupKey, item_count: usize) -> f64 {
    use TsGroupKey::*;
    let per_kind_constant = match key {
        // Names: highest priority — these orient the reader
        FunctionName { .. } => 1.0,
        StructName { .. } | ClassName { .. } | InterfaceName { .. } | TraitName { .. } => 1.0,
        EnumName { .. } => 1.0,
        TypeAliasName { .. } => 1.0,
        ConstName { .. } => 1.0,
        MacroName { .. } => 1.0,
        ImplBlock { .. } => 0.8,
        ModuleDocFirst => 1.0,

        // Signatures: moderate — show parameters and return types
        FunctionSig => 0.7,

        // Doc first line: moderate — one-line summary
        FunctionDocFirst | StructDocFirst | EnumDocFirst | ClassDocFirst
        | InterfaceDocFirst | TraitDocFirst | TypeAliasDocFirst | ConstDocFirst
        | MacroDocFirst => 0.4,

        // Body: kind-dependent
        FunctionBody => 0.2,
        StructBody => 1.2,
        EnumBody => 1.5,

        // Doc rest: low — the first line already gives the gist
        ModuleDocRest | FunctionDocRest | StructDocRest | EnumDocRest | ClassDocRest
        | InterfaceDocRest | TraitDocRest | TypeAliasDocRest | ConstDocRest
        | MacroDocRest => 0.3,

        // Imports
        Import { first_party: true, .. } => 0.1,
        Import { first_party: false, .. } => 0.0, // 3rd party imports only via dependent_siblings
        ImportedItems { first_party: true, .. } => 1.0,
        ImportedItems { first_party: false, .. } => 0.1,

        // Headings: depth-dependent
        Heading { level, .. } => match level {
            1 => 1.0,
            2 => 0.6,
            3 => 0.15,
            _ => 0.08,
        },
        HeadingBody => 0.7,

        // Data sections
        DataSection => 0.8,
        DataSectionBody => 0.3,
    };

    (item_count as f64).powf(0.75) * per_kind_constant
}

// ---------------------------------------------------------------------------
// Contribution — modifier passed from parent to child at children() time
// ---------------------------------------------------------------------------

/// Modifier contribution from a Folders group to its child groups.
pub fn folders_contribution(
    parent_dir: &Path,
    category: FileCategory,
) -> f64 {
    let depth = classify::effective_depth(parent_dir);
    let depth_factor = match depth {
        0..=1 => 1.0,
        2..=3 => 0.7,
        _ => 0.4,
    };
    let category_factor = match category {
        FileCategory::Source => 1.0,
        FileCategory::Example => 0.35,
        FileCategory::DocsSite => 0.2,
        FileCategory::Test => 0.15,
        FileCategory::CiConfig => 0.1,
    };
    // Root-level README/Architecture files get a bonus applied in
    // files_contribution, not here.
    depth_factor * category_factor
}

/// Modifier contribution from a Files group to its child tree-sitter groups.
pub fn files_contribution(
    role: FileRole,
    is_root_dir: bool,
    is_deprioritized: bool,
    is_type_declaration: bool,
    is_header: bool,
) -> f64 {
    let role_factor = if is_root_dir {
        match role {
            FileRole::Architecture | FileRole::Readme => 1.5,
            _ => 1.0,
        }
    } else {
        match role {
            FileRole::Translated | FileRole::Changelog | FileRole::CommunityHealth
            | FileRole::AiConfig => 0.1,
            _ => 1.0,
        }
    };
    let deprioritized_factor = if is_deprioritized { 0.2 } else { 1.0 };
    let type_declaration_factor = if is_type_declaration { 0.15 } else { 1.0 };
    let header_factor = if is_header { 2.5 } else { 1.0 };

    role_factor * deprioritized_factor * type_declaration_factor * header_factor
}

/// Modifier for generated files, applied to child TsGroups after source is read.
pub fn generated_contribution(is_generated: bool) -> f64 {
    if is_generated { 0.1 } else { 1.0 }
}

/// Modifier contribution for visibility (public vs private).
/// Applied to the modifier of private-variant groups.
pub fn visibility_contribution(is_public: bool) -> f64 {
    if is_public { 1.0 } else { 0.3 }
}

/// Modifier contribution for documented vs undocumented.
pub fn documented_contribution(is_documented: bool, key: &TsGroupKey) -> f64 {
    // Sections and module docs are inherently documented
    if matches!(key,
        TsGroupKey::Heading { .. } | TsGroupKey::HeadingBody
        | TsGroupKey::ModuleDocFirst | TsGroupKey::ModuleDocRest
        | TsGroupKey::DataSection | TsGroupKey::DataSectionBody
    ) {
        return 1.0;
    }
    if is_documented { 1.0 } else { 0.5 }
}

/// Modifier for boilerplate heading sections.
pub fn boilerplate_heading_contribution() -> f64 {
    0.1
}

/// Modifier for reexport imports.
pub fn reexport_contribution() -> f64 {
    0.1
}
