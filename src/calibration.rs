//! All base_value() and contribution() functions for the group taxonomy.
//! This is the single findable location for all calibration values (design §9).

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
        FunctionName(_) => 1.0,
        StructName(_) | ClassName(_) | InterfaceName(_) | TraitName(_) => 1.0,
        EnumName(_) => 1.0,
        TypeAliasName(_) => 1.0,
        ConstName(_) => 1.0,
        MacroName(_) => 1.0,
        ImplBlock(b) if b.is_boilerplate_trait => 0.15,
        ImplBlock(_) => 0.8,
        ModuleDocFirst(_) => 1.0,

        FunctionSig(_) => 0.7,

        FunctionDocFirst(_) | StructDocFirst(_) | EnumDocFirst(_) | ClassDocFirst(_)
        | InterfaceDocFirst(_) | TraitDocFirst(_) | TypeAliasDocFirst(_) | ConstDocFirst(_)
        | MacroDocFirst(_) => 0.4,

        FunctionBody(_) => 0.2,
        StructBody(_) => 1.2,
        EnumBody(_) => 1.5,
        ClassBody(_) => 1.0,
        // TypeAliasBody and ConstBody are "value-reveal" bodies: they upgrade
        // the name line from Truncated to Complete, usually adding one or two
        // more tokens per item. Kept low so they compete only via the
        // auto-commit budget-phase hatch, not against bigger content.
        TypeAliasBody(_) => 0.3,
        ConstBody(_) => 0.3,

        ModuleDocRest(_) | FunctionDocRest(_) | StructDocRest(_) | EnumDocRest(_)
        | ClassDocRest(_) | InterfaceDocRest(_) | TraitDocRest(_) | TypeAliasDocRest(_)
        | ConstDocRest(_) | MacroDocRest(_) => 0.3,

        Import(i) if i.first_party => 0.1,
        Import(_) => 0.0,
        ImportedItems(i) if i.first_party => 1.0,
        ImportedItems(_) => 0.1,

        Heading(h) => match h.level {
            1 => 1.0,
            2 => 0.6,
            3 => 0.15,
            _ => 0.08,
        },
        HeadingBody(h) => match h.level {
            1 => 1.2,
            2 => 0.5,
            3 => 0.1,
            _ => 0.05,
        },

        DataSection(d) => match d.level {
            1 => 1.0,
            2 => 0.6,
            3 => 0.15,
            _ => 0.08,
        },
        DataSectionBody(d) => match d.level {
            1 => 1.2,
            2 => 0.5,
            3 => 0.1,
            _ => 0.05,
        },
    };

    (item_count as f64).powf(0.75) * per_kind_constant
}

// ---------------------------------------------------------------------------
// Contribution — modifier passed from parent to child at children() time
// ---------------------------------------------------------------------------

/// Modifier contribution from a Folders group to its child groups.
pub fn folders_contribution(parent_dir: &Path, category: FileCategory) -> f64 {
    let depth = classify::effective_depth(parent_dir);
    let depth_factor = match depth {
        0..=1 => 1.0,
        2..=3 => 0.7,
        4 => 0.55,
        _ => 0.4,
    };
    let category_factor = match category {
        FileCategory::Source => 1.0,
        FileCategory::Example => 0.35,
        FileCategory::DocsSite => 0.2,
        FileCategory::Test => 0.15,
        FileCategory::CiConfig => 0.1,
        FileCategory::Artifact => 0.1,
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
    is_test_file: bool,
    has_companion_header: bool,
) -> f64 {
    let role_factor = match role {
        FileRole::Architecture | FileRole::Readme if is_root_dir => 1.5,
        FileRole::Translated
        | FileRole::Changelog
        | FileRole::CommunityHealth
        | FileRole::AiConfig => 0.1,
        _ => 1.0,
    };
    let deprioritized_factor = if is_deprioritized { 0.2 } else { 1.0 };
    let type_declaration_factor = if is_type_declaration { 0.15 } else { 1.0 };
    let header_factor = if is_header { 2.5 } else { 1.0 };
    let test_file_factor = if is_test_file { 0.15 } else { 1.0 };
    let companion_header_factor = if has_companion_header { 0.3 } else { 1.0 };

    role_factor
        * deprioritized_factor
        * type_declaration_factor
        * header_factor
        * test_file_factor
        * companion_header_factor
}

pub const PRIVATE_FACTOR: f64 = 0.3;
pub const UNDOCUMENTED_FACTOR: f64 = 0.5;
pub const BOILERPLATE_HEADING_FACTOR: f64 = 0.1;
pub const REEXPORT_FACTOR: f64 = 0.1;

pub const COMPACT_BODY_LINE_LIMIT: usize = 25;

/// Compute the inherited modifier for a TsGroup based on its key and parent modifier.
pub fn compute_item_modifier(key: &TsGroupKey, parent_modifier: f64) -> f64 {
    use TsGroupKey::*;

    let (doc_factor, vis_factor) = match key.name_doc_visibility() {
        Some((documented, public)) => {
            let doc = if documented { 1.0 } else { UNDOCUMENTED_FACTOR };
            let vis = if public { 1.0 } else { PRIVATE_FACTOR };
            (doc, vis)
        }
        None => (1.0, 1.0),
    };

    let boilerplate_factor = match key {
        Heading(h) if h.boilerplate => BOILERPLATE_HEADING_FACTOR,
        _ => 1.0,
    };

    let reexport_factor = match key {
        Import(i) if i.reexport => REEXPORT_FACTOR,
        ImportedItems(i) if i.reexport => REEXPORT_FACTOR,
        _ => 1.0,
    };

    parent_modifier * vis_factor * doc_factor * boilerplate_factor * reexport_factor
}
