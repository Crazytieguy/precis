mod classify;
mod cost;
mod groups;
mod plan;
mod solver;
mod value;

mod file_info;

use std::collections::HashSet;
use std::path::PathBuf;

use crate::parse;
use crate::render;
use crate::Corpus;

pub use classify::{FileCategory, FileRole};
pub use file_info::{FileInfo, compute_single_file_info};
pub use groups::build_groups;
pub(crate) use plan::directory_marker_text;
pub use plan::{RenderPlanItem, Schedule, SymbolRenderSpec};

// ---------------------------------------------------------------------------
// Public scheduling entry point
// ---------------------------------------------------------------------------

/// Run the full scheduling pipeline: greedy optimization followed by render
/// plan construction. Returns an ordered render plan with per-symbol specs.
pub fn schedule(built: &BuiltGroups, corpus: &Corpus<'_>, char_budget: Option<usize>) -> Schedule {
    let result = solver::solve(built, corpus, char_budget);
    plan::build_render_plan(&result, built, corpus)
}

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// Category of symbol for grouping and stage progression.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum KindCategory {
    Function, // Function, methods
    Type,     // Struct, Trait, Interface, Class
    Enum,     // Enum variant lists are high-value body content
    Constant, // Const, Static
    Module,   // Module, Namespace
    Section,  // Markdown headings
    Macro,
    Impl,
    Import,    // use/import statements
    ModuleDoc, // Module-level documentation (//!, docstrings, package comments)
}

impl KindCategory {
    pub fn from_symbol_kind(kind: parse::SymbolKind) -> Self {
        match kind {
            parse::SymbolKind::Function => KindCategory::Function,
            parse::SymbolKind::Struct
            | parse::SymbolKind::Trait
            | parse::SymbolKind::Interface
            | parse::SymbolKind::Class
            | parse::SymbolKind::TypeAlias => KindCategory::Type,
            parse::SymbolKind::Enum => KindCategory::Enum,
            parse::SymbolKind::Const | parse::SymbolKind::Static => KindCategory::Constant,
            parse::SymbolKind::Module => KindCategory::Module,
            parse::SymbolKind::Section => KindCategory::Section,
            parse::SymbolKind::Macro => KindCategory::Macro,
            parse::SymbolKind::Impl => KindCategory::Impl,
            parse::SymbolKind::Import => KindCategory::Import,
            parse::SymbolKind::ModuleDoc => KindCategory::ModuleDoc,
        }
    }

    /// Ordered stage progression for this kind category.
    pub fn stage_sequence(&self) -> &'static [StageKind] {
        match self {
            // Types: body before doc (struct fields are useful content)
            KindCategory::Type => &[
                StageKind::FilePath,
                StageKind::Names,
                StageKind::Signatures,
                StageKind::Body,
                StageKind::Doc,
            ],
            // Enums: same progression as types, but Body gets higher value
            // in compute_value (variant lists define type taxonomies)
            KindCategory::Enum => &[
                StageKind::FilePath,
                StageKind::Names,
                StageKind::Signatures,
                StageKind::Body,
                StageKind::Doc,
            ],
            // Sections and module docs: names (headings/first line) and body text
            KindCategory::Section | KindCategory::ModuleDoc => &[StageKind::FilePath, StageKind::Names, StageKind::Body],
            // Imports: names (truncated) → full signature line(s)
            KindCategory::Import => &[StageKind::FilePath, StageKind::Names, StageKind::Signatures],
            // Everything else: names → signatures → doc → body
            _ => &[
                StageKind::FilePath,
                StageKind::Names,
                StageKind::Signatures,
                StageKind::Doc,
                StageKind::Body,
            ],
        }
    }
}

/// The kind of a rendering stage. Doc and Body are expanded line-by-line
/// (Doc(1), Doc(2), ... up to the symbol's actual doc length).
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum StageKind {
    FilePath, // Show file paths only (no symbol content)
    Names,
    Signatures,
    Doc, // Each increment = one more line of doc across the group
    Body, // Each increment = one more line of body across the group
}

/// Grouping dimensions. All symbols sharing a GroupKey are treated identically.
/// Symbols from different files in the same directory with matching properties
/// are pooled together. This ensures similar symbols are shown or hidden as a
/// unit — if we can't distinguish their value, showing an arbitrary subset is
/// more confusing than showing all or none.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroupKey {
    pub is_public: bool,
    pub kind_category: KindCategory,
    /// Parent directory (relative to project root). Symbols from different
    /// files in the same directory are grouped together.
    pub parent_dir: PathBuf,
    pub is_documented: bool,
    pub file_role: FileRole,
    pub file_category: FileCategory,
    pub is_type_declaration: bool,
    /// Whether the file is a C/C++ header (.h, .hpp, etc.). Headers define
    /// public API and should be prioritized over implementation files.
    pub is_header: bool,
    /// Whether the file is auto-generated (codegen markers, protobuf output,
    /// mockery mocks, or auto-generated API doc READMEs).
    pub is_generated: bool,
    /// Whether this group is build/tool config. Per-symbol because TOML
    /// `[tool.*]` sections are config while `[package]` is not.
    pub is_config: bool,
    /// Heading depth for markdown sections (1 = h1, 2 = h2, etc.).
    /// None for non-section symbols.
    pub heading_depth: Option<u8>,
    /// Whether the imports in this group are 1st-party (local/relative).
    pub is_first_party: bool,
    /// Whether symbols are trait implementation methods (Rust only).
    pub is_trait_impl: bool,
    /// Whether this is a boilerplate markdown section (License, Contributing, etc.).
    pub is_boilerplate_section: bool,
    /// Whether the imports are `pub use` re-exports from child modules (Rust only).
    pub is_reexport: bool,
}

/// Token and character cost of a rendered element.
#[derive(Clone, Copy, Debug, Default)]
pub struct Cost {
    pub tokens: usize,
    pub chars: usize,
}

impl Cost {
    pub(super) fn new(tokens: usize, chars: usize) -> Self {
        Self { tokens, chars }
    }

    /// Compute token and character cost of a rendered text string.
    pub(super) fn of(text: &str) -> Self {
        Self { tokens: render::count_tokens(text), chars: text.len() }
    }
}

impl std::ops::Add for Cost {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { tokens: self.tokens + rhs.tokens, chars: self.chars + rhs.chars }
    }
}

impl std::ops::AddAssign for Cost {
    fn add_assign(&mut self, rhs: Self) {
        self.tokens += rhs.tokens;
        self.chars += rhs.chars;
    }
}

impl std::ops::Sub for Cost {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            tokens: self.tokens.saturating_sub(rhs.tokens),
            chars: self.chars.saturating_sub(rhs.chars),
        }
    }
}

impl std::iter::Sum for Cost {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

/// Reference to a symbol within the corpus, used by groups to track membership
/// without carrying per-symbol cost data.
#[derive(Clone, Copy)]
pub struct SymbolRef {
    pub file_idx: usize,
    pub symbol_idx: usize,
}

/// One position in a group's cumulative prefix-sum array.
#[derive(Clone, Copy)]
pub(super) struct CumulativeEntry {
    /// What (stage, n) this position represents.
    pub stage: StageKind,
    pub n: usize,
    /// Prefix sum of cost up to and including this entry.
    pub cost: Cost,
    /// Prefix sum of value up to and including this entry.
    pub value: f64,
}

/// Precomputed cumulative cost/value prefix sums for a group's linear stage
/// progression. Each position represents one schedulable item (e.g. Names(1),
/// Signatures(1), Doc(1), Doc(2), Body(1), etc.).
///
/// The solver uses these to compute incremental cost/value from the current
/// inclusion state to any target position in O(1) via subtraction, eliminating
/// the need for on-the-fly prerequisite cost computation.
#[derive(Default)]
pub(super) struct StageCumulatives {
    pub entries: Vec<CumulativeEntry>,
}

/// A group of similarly-valued symbols that always receive the same treatment.
pub struct Group {
    pub key: GroupKey,
    pub(super) symbols: Vec<SymbolRef>,
    pub(super) file_indices: HashSet<usize>,
    /// Pre-computed product of all static value factors (file role, depth,
    /// visibility, documented, etc.) — everything that depends only on
    /// `GroupKey` properties, not on stage or line number. Computed once
    /// in `build_groups`, read by `compute_value` on every call.
    pub(super) base_importance: f64,
    /// Pre-aggregated cost of the Names stage (sum of all symbol name costs).
    /// Accumulated during phase 1 (parallel), consumed by `build_cumulatives`.
    pub(super) names_cost: Cost,
    /// Pre-aggregated cost of the Signatures stage (sum of all symbol
    /// signature-beyond-name costs).
    /// Accumulated during phase 1 (parallel), consumed by `build_cumulatives`.
    pub(super) signatures_cost: Cost,
    /// Cumulative prefix sums for the group's linear stage progression.
    /// Built during phase 2 by `cost::build_cumulatives`.
    pub(super) cumulatives: StageCumulatives,
}

/// Groups with their build-time budget. The budget determines how many doc/body
/// layers were tokenized; scheduling with a larger budget would give wrong results.
pub struct BuiltGroups {
    pub groups: Vec<Group>,
    pub budget: usize,
}

impl StageCumulatives {
    /// Number of schedulable positions in this group's progression.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Incremental cost from the current inclusion position to a target position.
    /// `current` is `None` (nothing included) or `Some(pos)`.
    pub fn incremental_cost(&self, current: Option<usize>, target: usize) -> Cost {
        let target_cost = self.entries[target].cost;
        match current {
            Some(c) => target_cost - self.entries[c].cost,
            None => target_cost,
        }
    }

    /// Incremental value from the current inclusion position to a target position.
    pub fn incremental_value(&self, current: Option<usize>, target: usize) -> f64 {
        let target_value = self.entries[target].value;
        match current {
            Some(c) => target_value - self.entries[c].value,
            None => target_value,
        }
    }
}

/// Raw output from the greedy solver, before render plan construction.
/// Contains per-group inclusion decisions and which files have content.
pub(super) struct SolverResult {
    /// Per-group inclusion position in the cumulative array.
    /// `None` = group not included.
    pub group_positions: Vec<Option<usize>>,
    /// Files that have at least one included symbol (path cost already paid).
    pub files_shown: HashSet<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Validate that Import is the only KindCategory without Body in its
    /// stage sequence. parse::merge_shared_line_symbols relies on this
    /// invariant via an inline `sym.kind != SymbolKind::Import` check.
    #[test]
    fn import_is_only_kind_without_body() {
        let all_kinds = [
            KindCategory::Function,
            KindCategory::Type,
            KindCategory::Enum,
            KindCategory::Constant,
            KindCategory::Module,
            KindCategory::Section,
            KindCategory::Macro,
            KindCategory::Impl,
            KindCategory::Import,
            KindCategory::ModuleDoc,
        ];
        for &kind in &all_kinds {
            let has_body = kind.stage_sequence().contains(&StageKind::Body);
            if kind == KindCategory::Import {
                assert!(!has_body, "Import should NOT have Body stage");
            } else {
                assert!(has_body, "{kind:?} should have Body stage");
            }
        }
    }
}
