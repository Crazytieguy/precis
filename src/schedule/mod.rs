mod classify;
mod cost;
mod groups;
mod solver;
mod value;

use std::collections::HashSet;
use std::path::PathBuf;

use crate::parse;
use crate::render;

pub use classify::{FileCategory, FileRole};
pub use groups::build_groups;
pub use solver::schedule;

// ---------------------------------------------------------------------------
// Render plan — pre-resolved output structure for the renderer
// ---------------------------------------------------------------------------

/// Pre-resolved rendering parameters for a single symbol.
/// Computed by the scheduler, consumed by the renderer. The renderer doesn't
/// need to know about stages, groups, or kind categories — just these four
/// concrete decisions about what to show for each symbol.
#[derive(Debug, Clone, Copy)]
pub struct SymbolRenderSpec {
    /// Show the symbol name (false only for FilePath-only groups).
    pub show_name: bool,
    /// Show full signature beyond the name line.
    pub show_sig: bool,
    /// Number of doc comment lines to show. 0=none, usize::MAX=all.
    pub doc_lines: usize,
    /// Number of body lines to show. 0=none, usize::MAX=all.
    pub body_lines: usize,
}

/// An item in the render plan — the ordered sequence of things to output.
#[derive(Debug)]
pub enum RenderPlanItem {
    /// Omission marker for an invisible top-level directory.
    DirectoryMarker(PathBuf),
    /// A visible file (header line + its symbols).
    File(usize),
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
    Import, // use/import statements
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
            // Markdown: just names (headings) and body text
            KindCategory::Section => &[StageKind::FilePath, StageKind::Names, StageKind::Body],
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

/// A symbol's precomputed content costs per rendering stage.
#[derive(Clone)]
pub struct SymbolCosts {
    pub file_idx: usize,
    pub symbol_idx: usize,
    pub name: Cost,
    /// Additional cost for full signature beyond the name line.
    pub signature: Cost,
    /// Cost per doc comment line (ordered). For Python symbols with both
    /// pre-symbol `#` comments and post-signature docstrings, this is the
    /// concatenation of both sections (pre-comments first, then docstring lines).
    pub doc_lines: Vec<Cost>,
    /// Cost of the truncation marker after each doc line. Parallel to
    /// `doc_lines` — `doc_markers[i]` is the cost of `"      →{indent}…\n"`
    /// where indent matches `doc_lines[i]`'s source line.
    pub doc_markers: Vec<Cost>,
    /// Number of pre-symbol comment lines in `doc_lines`. The renderer
    /// emits pre-comments and docstrings as separate sections with a signature
    /// in between, so truncation markers must account for this split point.
    pub pre_doc_count: usize,
    /// Cost per body line (ordered).
    pub body_lines: Vec<Cost>,
    /// Cost of the truncation marker after each body line. Parallel to
    /// `body_lines`.
    pub body_markers: Vec<Cost>,
    /// Whether the symbol's body contains nested symbols (e.g., class methods).
    /// When true, body truncation markers are suppressed by the renderer.
    pub body_has_nested: bool,
}

/// A group of similarly-valued symbols that always receive the same treatment.
pub struct Group {
    pub key: GroupKey,
    pub(super) symbols: Vec<SymbolCosts>,
    pub(super) file_indices: HashSet<usize>,
    /// Pre-computed product of all static value factors (file role, depth,
    /// visibility, documented, etc.) — everything that depends only on
    /// `GroupKey` properties, not on stage or line number. Computed once
    /// in `build_groups`, read by `compute_value` on every call.
    pub(super) base_importance: f64,
    /// Cached: max doc lines the scheduler should consider. Capped by budget-aware
    /// truncation — may be less than the true max doc lines across symbols.
    pub(super) max_doc_n: usize,
    /// Cached: max body lines the scheduler should consider. Same capping applies.
    pub(super) max_body_n: usize,
}

/// Groups with their build-time budget. The budget determines how many doc/body
/// layers were tokenized; scheduling with a larger budget would give wrong results.
pub struct BuiltGroups {
    pub groups: Vec<Group>,
    pub budget: usize,
}

impl Group {
    /// Max line count for a Doc/Body stage. Returns 1 for Names/Signatures/FilePath.
    pub(super) fn max_n(&self, stage: StageKind) -> usize {
        match stage {
            StageKind::FilePath | StageKind::Names | StageKind::Signatures => 1,
            StageKind::Doc => self.max_doc_n,
            StageKind::Body => self.max_body_n,
        }
    }
}

/// The result of scheduling: a render plan (ordered output structure) and
/// per-symbol render specs (concrete rendering decisions).
pub struct Schedule {
    /// Ordered list of files and directory markers to render.
    pub render_plan: Vec<RenderPlanItem>,
    /// Per-symbol render specs, indexed as `[file_idx][sym_idx]`.
    /// `None` means the symbol is not rendered.
    pub symbol_specs: Vec<Vec<Option<SymbolRenderSpec>>>,
}

/// What stage a group has been included up to. Internal to the scheduler;
/// the renderer receives pre-resolved [`SymbolRenderSpec`]s instead.
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
