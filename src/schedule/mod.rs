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
    pub(super) names_cost: Cost,
    /// Pre-aggregated cost of the Signatures stage (sum of all symbol
    /// signature-beyond-name costs).
    pub(super) signatures_cost: Cost,
    /// Pre-aggregated net cost per doc layer. `doc_layer_costs[n-1]` is the
    /// incremental cost of showing doc layer n (content + truncation marker
    /// delta). Computed during group construction with budget-aware truncation.
    pub(super) doc_layer_costs: Vec<Cost>,
    /// Pre-aggregated net cost per body layer (same structure as doc).
    pub(super) body_layer_costs: Vec<Cost>,
}

/// Groups with their build-time budget. The budget determines how many doc/body
/// layers were tokenized; scheduling with a larger budget would give wrong results.
pub struct BuiltGroups {
    pub groups: Vec<Group>,
    pub budget: usize,
}

impl Group {
    /// Max line count for a Doc/Body stage. Returns 1 for Names/Signatures/FilePath.
    /// For Doc/Body, this is the number of pre-aggregated layer costs computed
    /// during group construction (capped by budget-aware truncation).
    pub(super) fn max_n(&self, stage: StageKind) -> usize {
        match stage {
            StageKind::FilePath | StageKind::Names | StageKind::Signatures => 1,
            StageKind::Doc => self.doc_layer_costs.len(),
            StageKind::Body => self.body_layer_costs.len(),
        }
    }

    /// Pre-aggregated cost of a stage at level n. O(1) lookup into costs
    /// computed during group construction — the solver never iterates
    /// per-symbol cost arrays.
    pub(super) fn stage_cost(&self, stage: StageKind, n: usize) -> Cost {
        match stage {
            // FilePath has zero own_cost — its cost is handled via file_path_costs
            // on the QueueItem, which correctly accounts for cross-group sharing.
            StageKind::FilePath => Cost::default(),
            StageKind::Names => self.names_cost,
            StageKind::Signatures => self.signatures_cost,
            StageKind::Doc => self.doc_layer_costs.get(n - 1).copied().unwrap_or_default(),
            StageKind::Body => self.body_layer_costs.get(n - 1).copied().unwrap_or_default(),
        }
    }
}

/// Raw output from the greedy solver, before render plan construction.
/// Contains per-group inclusion decisions and which files have content.
pub(super) struct SolverResult {
    /// Per-group inclusion state. `None` = group not included.
    pub group_stages: Vec<Option<IncludedStage>>,
    /// Files that have at least one included symbol (path cost already paid).
    pub files_shown: HashSet<usize>,
}

/// What stage a group has been included up to. Used by the solver to track
/// inclusion state and by the plan builder to resolve [`SymbolRenderSpec`]s.
#[derive(Debug, Clone, Copy)]
pub(super) struct IncludedStage {
    pub(super) kind: StageKind,
    /// For Doc/Body stages: how many lines to show. Ignored for Names/Signatures.
    pub(super) n_lines: usize,
}

impl IncludedStage {
    /// Check if a given (stage_kind, n) is fully covered by this inclusion.
    ///
    /// A stage item is covered if it's earlier in the progression than the
    /// included stage, or if it's the same stage with n <= the included n_lines.
    /// Use `n = 1` to test whether a stage is included at all.
    pub(super) fn covers(&self, stages: &[StageKind], stage_kind: StageKind, n: usize) -> bool {
        let Some(inc_pos) = stages.iter().position(|&s| s == self.kind) else {
            return false;
        };
        let Some(this_pos) = stages.iter().position(|&s| s == stage_kind) else {
            return false;
        };
        this_pos < inc_pos || (this_pos == inc_pos && n <= self.n_lines)
    }
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
