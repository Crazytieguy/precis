use std::collections::{BinaryHeap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::format;
use crate::parse;
use crate::walk;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// Category of symbol for grouping and stage progression.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum KindCategory {
    Function, // Function, methods
    Type,     // Struct, Enum, Trait, Interface, Class
    Constant, // Const, Static, TypeAlias
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
            | parse::SymbolKind::Enum
            | parse::SymbolKind::Trait
            | parse::SymbolKind::Interface
            | parse::SymbolKind::Class => KindCategory::Type,
            parse::SymbolKind::Const | parse::SymbolKind::Static | parse::SymbolKind::TypeAlias => {
                KindCategory::Constant
            }
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
            // Types: body before doc (struct fields/enum variants are the useful content)
            KindCategory::Type => &[
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

/// Role of the file a symbol lives in, for separating high-value files
/// (README) from low-value files (CHANGELOG) in markdown grouping.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum FileRole {
    /// README.md, readme.md, etc.
    Readme,
    /// CHANGELOG.md, CHANGES.md, HISTORY.md, NEWS.md, etc.
    Changelog,
    /// Translated/localized files (e.g., README_zh-CN.md, README-es.md).
    Translated,
    /// Community health files: CONTRIBUTING.md, LICENSE.md, SECURITY.md, CODE_OF_CONDUCT.md, etc.
    CommunityHealth,
    /// AI coding assistant config files: CLAUDE.md, AGENTS.md, COPILOT.md, etc.
    AiConfig,
    /// Everything else.
    Normal,
}

impl FileRole {
    pub fn from_filename(name: &str) -> Self {
        let lower = name.to_ascii_lowercase();
        // Strip document extension for matching
        let (stem, is_doc) = lower.strip_suffix(".md").map(|s| (s, true))
            .or_else(|| lower.strip_suffix(".markdown").map(|s| (s, true)))
            .or_else(|| lower.strip_suffix(".txt").map(|s| (s, true)))
            .unwrap_or((&lower, false));
        match stem {
            "readme" => FileRole::Readme,
            "changelog" | "changes" | "history" | "news" | "releases" => FileRole::Changelog,
            "contributing" | "contributors" | "security" | "license" | "licence"
            | "code_of_conduct" | "codeowners" | "releasing" | "support"
            | "governance" | "authors" | "maintainers" => FileRole::CommunityHealth,
            "claude" | "agents" | "copilot" | "copilot-instructions" => FileRole::AiConfig,
            _ if is_doc && has_locale_suffix(stem) => FileRole::Translated,
            _ => FileRole::Normal,
        }
    }

    /// Determine file role from the full relative path.
    /// Checks both the filename and directory components for locale patterns.
    /// Locale directories override filename-based detection (a translated README
    /// is low value, not high value).
    pub fn from_path(path: &Path) -> Self {
        let is_doc = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| matches!(e, "md" | "markdown" | "txt"))
            .unwrap_or(false);
        // For doc files, check if any parent directory is a locale directory
        // (e.g., docs/zh-CN/guide.md, i18n/es/readme.md).
        // This takes priority over filename-based detection.
        if is_doc {
            for component in path.components() {
                if let std::path::Component::Normal(seg) = component
                    && let Some(s) = seg.to_str()
                    && is_locale_dir(s)
                {
                    return FileRole::Translated;
                }
            }
        }
        path.file_name()
            .and_then(|n| n.to_str())
            .map(FileRole::from_filename)
            .unwrap_or(FileRole::Normal)
    }
}

/// Detect locale suffixes like `_zh-CN`, `-es`, `.ja`, `_pt-BR`.
/// Matches `[_.-]xx` or `[_.-]xx[-_]yy` at end of stem, where xx/yy are
/// 2-letter ASCII alpha codes (ISO 639-1 language / ISO 3166-1 region).
fn has_locale_suffix(stem: &str) -> bool {
    let bytes = stem.as_bytes();
    let len = bytes.len();
    // Minimum: separator + 2-letter code = 3 chars, plus at least 1 char before
    if len < 4 {
        return false;
    }
    let is_sep = |b: u8| b == b'_' || b == b'-' || b == b'.';
    let is_alpha = |b: u8| b.is_ascii_lowercase();

    // Try `[sep]xx[-_]yy` at end (6 chars)
    if len >= 7 && is_sep(bytes[len - 6]) && is_alpha(bytes[len - 5])
        && is_alpha(bytes[len - 4]) && (bytes[len - 3] == b'-' || bytes[len - 3] == b'_')
        && is_alpha(bytes[len - 2]) && is_alpha(bytes[len - 1])
    {
        return true;
    }
    // Try `[sep]xx` at end (3 chars)
    if is_sep(bytes[len - 3]) && is_alpha(bytes[len - 2]) && is_alpha(bytes[len - 1]) {
        return true;
    }
    false
}

/// Detect locale directory names like `zh-CN`, `pt-BR`, `en-US`,
/// or well-known i18n directories like `i18n`, `l10n`, `locales`, `translations`.
/// Only matches the `xx-YY` / `xx_YY` pattern (not bare 2-letter codes, which
/// have too many false positives like `go`, `js`, `ci`).
fn is_locale_dir(name: &str) -> bool {
    let bytes = name.as_bytes();
    // Well-known i18n directory names
    let lower = name.to_ascii_lowercase();
    if matches!(lower.as_str(), "i18n" | "l10n" | "locales" | "locale" | "translations") {
        return true;
    }
    // xx-YY or xx_YY (5 chars): e.g. zh-CN, pt-BR, en-US
    if bytes.len() == 5
        && bytes[0].is_ascii_alphabetic()
        && bytes[1].is_ascii_alphabetic()
        && (bytes[2] == b'-' || bytes[2] == b'_')
        && bytes[3].is_ascii_alphabetic()
        && bytes[4].is_ascii_alphabetic()
    {
        return true;
    }
    false
}

/// Detect heading depth from source lines for a markdown section symbol.
/// ATX headings (`# h1`, `## h2`, etc.) are detected by counting leading `#` chars.
/// Setext headings (underline with `=` or `-`) are detected by checking the underline.
fn detect_heading_depth(lines: &[&str], sym_line_0: usize, end_line: usize) -> u8 {
    let trimmed = lines[sym_line_0].trim_start();
    if trimmed.starts_with('#') {
        let depth = trimmed.bytes().take_while(|&b| b == b'#').count();
        (depth as u8).clamp(1, 6)
    } else {
        // Setext heading: check underline character
        let underline_idx = end_line.min(lines.len()) - 1;
        if underline_idx > sym_line_0
            && lines[underline_idx].trim_start().starts_with('=')
        {
            1
        } else {
            2 // '-' underline = h2
        }
    }
}

/// Detect build/tool configuration files by filename and path convention.
/// These are files like `eslint.config.js`, `jest.config.ts`, `.eslintrc.js`, etc.
/// that configure development tools rather than implementing library/app logic.
/// Also detects files in conventional tooling directories (`scripts/`, `tools/`).
/// `relative_path` is the path relative to the project root.
fn is_config_file(relative_path: &Path, filename: &str) -> bool {
    let lower = filename.to_ascii_lowercase();
    let is_root = relative_path.parent().is_none_or(|p| p.as_os_str().is_empty());

    // Build scripts and packaging files — only at project root.
    // build.rs: Rust build scripts (compile-time codegen, feature probing)
    // setup.py: Python setuptools packaging
    // These filenames are generic enough that in subdirs they may be regular source files
    // (e.g., src/cmd/build.rs implements a "build" CLI subcommand).
    if is_root {
        match lower.as_str() {
            "build.rs" | "setup.py" => return true,
            _ => {}
        }
    }

    // JS task runners — matched by filename anywhere (these names are unambiguous).
    match lower.as_str() {
        "gulpfile.js" | "gruntfile.js" | "jakefile.js" => return true,
        _ => {}
    }

    // *.config.{ext} — catches eslint.config.js, jest.config.ts, vite.config.mjs,
    // next.config.js, tailwind.config.ts, postcss.config.js, tsup.config.ts,
    // playwright.config.ts, rollup.config.js, webpack.config.js, babel.config.js, etc.
    if let Some(stem) = lower.rsplit_once('.').map(|(s, _)| s) {
        if stem.ends_with(".config") {
            return true;
        }
        // .{name}rc.{ext} — catches .eslintrc.js, .babelrc.js, .prettierrc.mjs, etc.
        if stem.starts_with('.') && stem.ends_with("rc") {
            return true;
        }
    }

    // Files in conventional tooling directories are development utilities
    // (release scripts, build helpers, CI glue), not core library/app code.
    // Only `scripts/` and `tools/` — these are unambiguous across ecosystems.
    for component in relative_path.components() {
        if let std::path::Component::Normal(seg) = component
            && let Some(name) = seg.to_str()
        {
            match name.to_ascii_lowercase().as_str() {
                "scripts" | "script" | "tools" | "tool" => return true,
                _ => {}
            }
        }
    }

    false
}

/// Grouping dimensions. All symbols sharing a GroupKey are treated identically.
/// Groups are per-file: symbols from different files are never pooled together,
/// even if they share the same directory, kind, and visibility. This prevents
/// large directories (many files with many methods) from creating oversized
/// groups whose Names cost outweighs their per-token priority.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroupKey {
    pub is_public: bool,
    pub kind_category: KindCategory,
    /// Relative file path (from project root). Per-file grouping ensures
    /// each file's symbols compete independently in the scheduler.
    pub file_path: PathBuf,
    pub is_documented: bool,
    pub file_role: FileRole,
    /// Whether the file is a build/tool config file (e.g., eslint.config.js, jest.config.ts).
    pub is_config: bool,
    /// Whether the file is test/benchmark/example infrastructure.
    pub is_test: bool,
    /// Heading depth for markdown sections (1 = h1, 2 = h2, etc.).
    /// None for non-section symbols. Separates top-level headings from
    /// detail subsections so h1/h2 content gets priority over h3+ detail.
    pub heading_depth: Option<u8>,
    /// Whether the imports in this group are 1st-party (local/relative).
    /// Only meaningful for Import kind; false for all other kinds.
    pub is_first_party: bool,
}

/// A symbol's precomputed content costs (token counts per rendering stage).
pub struct SymbolCosts {
    pub file_idx: usize,
    pub symbol_idx: usize,
    pub name_tokens: usize,
    /// Additional tokens for full signature beyond the name line.
    pub signature_tokens: usize,
    /// Tokens per doc comment line (ordered). For Python symbols with both
    /// pre-symbol `#` comments and post-signature docstrings, this is the
    /// concatenation of both sections (pre-comments first, then docstring lines).
    pub doc_line_tokens: Vec<usize>,
    /// Number of pre-symbol comment lines in `doc_line_tokens`. The renderer
    /// emits pre-comments and docstrings as separate sections with a signature
    /// in between, so truncation markers must account for this split point.
    pub pre_doc_count: usize,
    /// Tokens per body line (ordered).
    pub body_line_tokens: Vec<usize>,
    /// Whether the symbol's body contains nested symbols (e.g., class methods).
    /// When true, body truncation markers are suppressed by the renderer.
    pub body_has_nested: bool,
}

/// A group of similarly-valued symbols that always receive the same treatment.
pub struct Group {
    pub key: GroupKey,
    pub symbols: Vec<SymbolCosts>,
    pub file_indices: HashSet<usize>,
    /// Cached: max doc lines across all symbols in this group.
    pub max_doc_n: usize,
    /// Cached: max body lines across all symbols in this group.
    pub max_body_n: usize,
}

impl Group {
    /// Max line count for a Doc/Body stage. Returns 1 for Names/Signatures/FilePath.
    fn max_n(&self, stage: StageKind) -> usize {
        match stage {
            StageKind::FilePath | StageKind::Names | StageKind::Signatures => 1,
            StageKind::Doc => self.max_doc_n,
            StageKind::Body => self.max_body_n,
        }
    }
}

/// The result of scheduling: for each group, the highest included stage
/// and the number of Doc/Body lines to show.
pub struct Schedule {
    /// For each group index: (stage_kind, n_lines) where n_lines applies to Doc/Body.
    /// None means the group is hidden.
    pub group_stages: Vec<Option<IncludedStage>>,
    /// Files whose paths should be shown (indices into the files array).
    pub visible_files: HashSet<usize>,
    /// Reverse lookup: for a (file_idx, symbol_idx) pair, which group index it belongs to.
    pub symbol_to_group: HashMap<(usize, usize), usize>,
}

/// What stage a group has been included up to.
#[derive(Debug, Clone, Copy)]
pub struct IncludedStage {
    pub kind: StageKind,
    /// For Doc/Body stages: how many lines to show. Ignored for Names/Signatures.
    pub n_lines: usize,
}

impl IncludedStage {
    /// Check if a given (stage_kind, n) is fully covered by this inclusion.
    ///
    /// A stage item is covered if it's earlier in the progression than the
    /// included stage, or if it's the same stage with n <= the included n_lines.
    /// Use `n = 1` to test whether a stage is included at all.
    pub fn covers(&self, stages: &[StageKind], stage_kind: StageKind, n: usize) -> bool {
        let Some(inc_pos) = stages.iter().position(|&s| s == self.kind) else {
            return false;
        };
        let Some(this_pos) = stages.iter().position(|&s| s == stage_kind) else {
            return false;
        };
        this_pos < inc_pos || (this_pos == inc_pos && n <= self.n_lines)
    }
}

// ---------------------------------------------------------------------------
// Group construction
// ---------------------------------------------------------------------------

/// Build groups from extracted symbols, computing per-symbol costs.
pub fn build_groups(
    root: &Path,
    files: &[PathBuf],
    sources: &[Option<String>],
    all_symbols: &[Vec<parse::Symbol>],
    layouts: &[Vec<format::SymbolLayout>],
) -> Vec<Group> {
    let mut group_map: HashMap<GroupKey, Group> = HashMap::new();

    for (file_idx, (file, symbols)) in files.iter().zip(all_symbols.iter()).enumerate() {
        let source = match &sources[file_idx] {
            Some(s) => s,
            None => continue,
        };
        let relative = file.strip_prefix(root).unwrap_or(file);
        let file_path = relative.to_path_buf();
        let lines: Vec<&str> = source.lines().collect();
        let file_role = FileRole::from_path(relative);
        let is_config = relative.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|name| is_config_file(relative, name));
        let is_test = walk::is_test_file(relative);

        for (symbol_idx, sym) in symbols.iter().enumerate() {
            let sym_line_0 = sym.line - 1;
            let kind_category = KindCategory::from_symbol_kind(sym.kind);
            let layout = &layouts[file_idx][symbol_idx];

            // Detect documentation from layout (uses trimmed range)
            let is_documented = layout.doc_start < layout.doc_end;

            let lang = crate::Lang::from_path(relative);
            let heading_depth = if kind_category == KindCategory::Section {
                if matches!(lang, Some(crate::Lang::Toml)) {
                    // Count dot-separated segments: [project] → 1, [tool.ruff] → 2
                    let depth = sym.name.chars().filter(|&c| c == '.').count() as u8 + 1;
                    Some(depth)
                } else if matches!(lang, Some(crate::Lang::Json | crate::Lang::Yaml)) {
                    Some(1) // JSON/YAML sections are all top-level
                } else {
                    Some(detect_heading_depth(&lines, sym_line_0, sym.end_line))
                }
            } else {
                None
            };

            let key = GroupKey {
                is_public: sym.is_public,
                kind_category,
                file_path: file_path.clone(),
                is_documented,
                file_role,
                is_config,
                is_test,
                heading_depth,
                is_first_party: sym.is_first_party,
            };

            // Compute costs
            let costs = compute_symbol_costs(
                file_idx,
                symbol_idx,
                sym,
                &lines,
                layout,
            );

            let group = group_map.entry(key.clone()).or_insert_with(|| Group {
                key,
                symbols: Vec::new(),
                file_indices: HashSet::new(),
                max_doc_n: 0,
                max_body_n: 0,
            });
            group.max_doc_n = group.max_doc_n.max(costs.doc_line_tokens.len());
            group.max_body_n = group.max_body_n.max(costs.body_line_tokens.len());
            group.symbols.push(costs);
            group.file_indices.insert(file_idx);
        }
    }

    let mut groups: Vec<Group> = group_map.into_values().collect();
    groups.sort_by(|a, b| a.key.cmp(&b.key));
    groups
}

/// Compute token costs for each rendering stage of a single symbol.
/// Reads all line ranges from the pre-computed layout.
fn compute_symbol_costs(
    file_idx: usize,
    symbol_idx: usize,
    sym: &parse::Symbol,
    lines: &[&str],
    layout: &format::SymbolLayout,
) -> SymbolCosts {
    let sym_line_0 = layout.sym_line_0;

    let sig_end = layout.sig_end;
    let is_section = sym.kind == parse::SymbolKind::Section;

    let name_tokens = if is_section {
        // Sections show the full heading/section line at Names stage (no
        // truncation marker). Cost matches the signature line since headings
        // are single-line.
        let line = format::strip_heading_badges(lines.get(layout.sym_line_0).copied().unwrap_or(""));
        format::count_tokens(&format::fmt_line(layout.sym_line_0, line))
    } else {
        // Token count of the formatted name line, plus 1 for the " …" suffix
        // that the renderer appends to names-only entries. Including the
        // ellipsis cost keeps Names-only rendering in sync with the budget.
        // When Signatures are also included (no ellipsis rendered), the +1 is
        // offset by a corresponding -1 in signature_tokens (sig_total -
        // name_tokens), so the total Names+Signatures cost remains exact.
        let name_line = format::format_symbol_name(sym, lines);
        format::count_tokens(&name_line) + 1
    };

    // Signature: additional tokens from showing full signature lines (with line numbers)
    // beyond what the name-only line shows.
    let mut sig_formatted_tokens = 0;
    for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
        // Strip trailing badges from markdown heading lines (matches renderer)
        let line = if is_section && i == sym_line_0 {
            format::strip_heading_badges(line)
        } else {
            line
        };
        sig_formatted_tokens += format::count_tokens(&format::fmt_line(i, line));
    }
    let signature_tokens = sig_formatted_tokens.saturating_sub(name_tokens);

    // Doc comment lines (pre-symbol comments + Python docstrings)
    let mut doc_line_tokens = Vec::new();
    if layout.doc_start < layout.doc_end {
        for (i, line) in lines.iter().enumerate().take(layout.doc_end).skip(layout.doc_start) {
            doc_line_tokens.push(format::count_tokens(&format::fmt_line(i, line)));
        }
    }
    let pre_doc_count = doc_line_tokens.len();
    // Python docstrings
    if layout.ds_end > layout.ds_start {
        for (i, line) in lines.iter().enumerate().take(layout.ds_end).skip(layout.ds_start) {
            doc_line_tokens.push(format::count_tokens(&format::fmt_line(i, line)));
        }
    }

    // Body lines — ranges come from layout (already truncated at first child)
    let mut body_line_tokens = Vec::new();
    if is_section {
        // Markdown: body is content text between headings
        for (i, line) in lines.iter().enumerate().take(layout.md_section_end).skip(layout.md_content_start) {
            body_line_tokens.push(format::count_tokens(&format::fmt_line(i, line)));
        }
    } else {
        // Code: body_start..body_end (truncated at first child by layout)
        for (i, line) in lines.iter().enumerate().take(layout.body_end).skip(layout.body_start) {
            body_line_tokens.push(format::count_tokens(&format::fmt_line(i, line)));
        }
    }

    SymbolCosts {
        file_idx,
        symbol_idx,
        name_tokens,
        signature_tokens,
        doc_line_tokens,
        pre_doc_count,
        body_line_tokens,
        body_has_nested: layout.has_children,
    }
}

// ---------------------------------------------------------------------------
// Value computation
// ---------------------------------------------------------------------------

/// Compute the effective depth of a path, skipping the first component if it's
/// a conventional source root directory (`src`, `lib`, `pkg`, `cmd`). These
/// directories are language-mandated or conventional organizational roots that
/// don't represent meaningful project hierarchy.
fn effective_depth(parent_dir: &Path) -> usize {
    let mut components = parent_dir.components();
    let total = components.clone().count();
    if total == 0 {
        return 0;
    }
    let first = components.next().and_then(|c| c.as_os_str().to_str());
    if matches!(first, Some("src" | "lib" | "pkg" | "cmd")) {
        total - 1
    } else {
        total
    }
}

/// Compute the value of showing a particular stage for a group.
fn compute_value(group: &Group, stage: StageKind, n: usize) -> f64 {
    let key = &group.key;

    let visibility = if key.is_public { 1.0 } else { 0.3 };
    let documented = if key.is_documented { 1.0 } else { 0.5 };

    // Compute effective depth, skipping conventional source root directories
    // (src/, lib/, pkg/, cmd/) that are purely organizational and don't represent
    // meaningful hierarchy. Without this, `src/pluggy/_manager.py` (depth 2, factor
    // 0.7) would be penalized relative to `docs/conf.py` (depth 1, factor 1.0),
    // even though the source code is more valuable than build configuration.
    let parent_dir = key.file_path.parent().unwrap_or(Path::new(""));
    let depth = effective_depth(parent_dir);
    let depth_factor = match depth {
        0..=1 => 1.0,
        2..=3 => 0.7,
        _ => 0.4,
    };

    // Groups with many symbols: each individual symbol is less important.
    // Gentle log decay: 1 sym → 1.0, 2 → 0.94, 5 → 0.86, 10 → 0.81, 50 → 0.72.
    let sibling_count = group.symbols.len().max(1) as f64;
    let sibling_factor = 1.0 / (1.0 + sibling_count.ln() * 0.1);

    // File role: README files are high-signal, changelogs/translations/metadata are low-signal.
    let file_role_factor = match key.file_role {
        FileRole::Readme => 1.5,
        FileRole::Normal => 1.0,
        FileRole::Translated => 0.1,
        FileRole::Changelog => 0.1,
        FileRole::CommunityHealth => 0.1,
        FileRole::AiConfig => 0.1,
    };

    // Config files (eslint.config.js, jest.config.ts, etc.) are build/tool setup,
    // not core library logic. Show them only when there's plenty of budget.
    let config_factor = if key.is_config { 0.2 } else { 1.0 };

    // Test/benchmark/example files are infrastructure, not core API.
    // At tight budgets they appear as file paths only (structural context);
    // at generous budgets their symbols are rendered normally.
    let test_factor = if key.is_test { 0.15 } else { 1.0 };

    // Heading depth: top-level headings (h1, h2) are more important than
    // subsections (h3+). This lets the scheduler show body content for
    // h1/h2 headings before filling in h3/h4 detail sections.
    let heading_depth_factor = match key.heading_depth {
        Some(1) => 1.0,
        Some(2) => 0.8,
        Some(3) => 0.5,
        Some(_) => 0.3, // h4, h5, h6
        None => 1.0,    // non-section symbols
    };

    // 1st-party imports tell you about internal module structure and are
    // higher signal than 3rd-party dependency imports.
    let first_party_factor = if key.kind_category == KindCategory::Import && key.is_first_party {
        2.0
    } else {
        1.0
    };

    let base_value = visibility * documented * depth_factor * sibling_factor * file_role_factor * config_factor * test_factor * heading_depth_factor * first_party_factor;

    let stage_value = match key.kind_category {
        KindCategory::Type => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 1.0,
            StageKind::Signatures => 0.7,
            StageKind::Body => 0.6,
            StageKind::Doc => 0.4,
        },
        KindCategory::Section => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 1.0,
            StageKind::Body => 0.3,
            _ => 0.1,
        },
        // Imports: supplementary context for understanding file dependencies.
        // Lower base value since they're not API surface, but still useful.
        KindCategory::Import => match stage {
            StageKind::FilePath => 0.3,
            StageKind::Names => 1.0,
            StageKind::Signatures => 0.5,
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

    // Non-public symbols: deprioritize body/doc content beyond signatures.
    // At tight budgets, private/internal symbol names and signatures are shown
    // (cheap, useful for understanding structure), but budget is reserved for
    // public symbol bodies and documentation. Without this, small groups of
    // private symbols reach Body stage before large groups of public symbols
    // reach Names — showing private implementation detail before API surface.
    let private_detail_penalty = if !key.is_public
        && matches!(stage, StageKind::Doc | StageKind::Body)
    {
        0.15
    } else {
        1.0
    };

    // Names and Signatures stages: cost scales linearly with group size
    // (each symbol adds its name/signature tokens). Without compensation,
    // large groups (e.g. a class with 50 methods) get extremely low
    // priority because priority = value / cost, and cost grows with N
    // while value was constant. Scale value by N so all groups have the
    // same per-token priority at Names/Signatures regardless of size.
    // This ensures breadth-first scheduling: all groups reach Names before
    // any group reaches Doc/Body. The sibling_factor still provides a mild
    // log-based penalty (28% at N=50) so larger groups don't dominate over
    // smaller groups with higher base value.
    let count_factor = if matches!(stage, StageKind::Names | StageKind::Signatures) {
        group.symbols.len().max(1) as f64
    } else {
        1.0
    };

    base_value * stage_value * private_detail_penalty * count_factor / n as f64
}

// ---------------------------------------------------------------------------
// Priority queue algorithm
// ---------------------------------------------------------------------------

/// Mutable state for the lazy-deletion priority queue.
struct ScheduleQueue {
    /// Monotonic counter; each enqueued/invalidated item gets a unique value.
    generation: u64,
    /// Per-group map from (stage_kind, n) to the latest generation. Heap
    /// entries whose generation doesn't match are stale and get skipped.
    current_gen: Vec<HashMap<(StageKind, usize), u64>>,
    heap: BinaryHeap<QueueItem>,
}

impl ScheduleQueue {
    fn new(num_groups: usize) -> Self {
        Self {
            generation: 0,
            current_gen: vec![HashMap::new(); num_groups],
            heap: BinaryHeap::new(),
        }
    }

    fn next_generation(&mut self) -> u64 {
        let g = self.generation;
        self.generation += 1;
        g
    }
}

/// An item in the scheduling priority queue.
#[derive(Debug)]
struct QueueItem {
    group_idx: usize,
    stage_kind: StageKind,
    /// For Doc/Body: which line number (1-indexed) this item represents.
    /// For Names/Signatures: always 1.
    n: usize,
    own_value: f64,
    own_cost: usize,
    /// Sum of values from unmet prerequisite stages.
    prereq_value: f64,
    /// Sum of costs from unmet prerequisite stages.
    prereq_cost: usize,
    /// Cost of file paths not yet shown (for files this group touches).
    file_path_cost: usize,
    /// Generation counter for lazy deletion.
    generation: u64,
}

impl QueueItem {
    fn total_value(&self) -> f64 {
        self.own_value + self.prereq_value
    }

    fn total_cost(&self) -> usize {
        self.own_cost + self.prereq_cost + self.file_path_cost
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for QueueItem {}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Cross-multiplication avoids float division precision loss:
        // self_value / self_cost vs other_value / other_cost
        // becomes self_value * other_cost vs other_value * self_cost
        let lhs = self.total_value() * other.total_cost() as f64;
        let rhs = other.total_value() * self.total_cost() as f64;
        lhs.partial_cmp(&rhs)
            .unwrap_or(std::cmp::Ordering::Equal)
            // Deterministic tiebreaker: lower group_idx, then stage_kind, then n
            .then_with(|| other.group_idx.cmp(&self.group_idx))
            .then_with(|| other.stage_kind.cmp(&self.stage_kind))
            .then_with(|| other.n.cmp(&self.n))
    }
}

/// Compute the token cost of a file path line.
fn file_path_cost(relative_path: &Path) -> usize {
    // Path line is just the relative path as text, plus a newline.
    format::count_tokens(&format!("{}\n", relative_path.display()))
}

/// Enqueue (or re-enqueue) priority queue items for the given groups.
///
/// Computes values, costs, and prerequisite state for each (group, stage, n)
/// triple. Skips items already included (based on `group_stages`) and items
/// that exceed `remaining_budget`. For over-budget items, invalidates their
/// generation entries so stale heap entries are ignored.
///
/// Used for both initial queue construction (with empty state) and updates
/// after an item is accepted (with partial state for affected groups only).
fn enqueue_group_items(
    group_indices: &[usize],
    groups: &[Group],
    group_stages: &[Option<IncludedStage>],
    file_path_costs: &[usize],
    files_shown: &HashSet<usize>,
    remaining_budget: usize,
    queue: &mut ScheduleQueue,
) {
    for &group_idx in group_indices {
        let group = &groups[group_idx];
        let stages = group.key.kind_category.stage_sequence();
        for (stage_pos, &stage_kind) in stages.iter().enumerate() {
            let max_n = group.max_n(stage_kind);
            if max_n == 0 {
                continue;
            }
            for n in 1..=max_n {
                // Skip items already included
                if group_stages[group_idx]
                    .as_ref()
                    .is_some_and(|inc| inc.covers(stages, stage_kind, n))
                {
                    continue;
                }

                let own_value = compute_value(group, stage_kind, n);
                let own_cost = stage_cost(group, stage_kind, n);
                let (prereq_value, prereq_cost) = compute_prereq_costs(
                    group,
                    stages,
                    stage_pos,
                    stage_kind,
                    n,
                    group_stages[group_idx].as_ref(),
                );
                let fp_cost: usize = group
                    .file_indices
                    .iter()
                    .filter(|fi| !files_shown.contains(fi))
                    .map(|&fi| file_path_costs[fi])
                    .sum();

                let total_cost = own_cost + prereq_cost + fp_cost;

                // Budget pruning: skip items that can't fit. Invalidate their
                // generation so stale heap entries are ignored. For Doc/Body,
                // total_cost is monotonically non-decreasing with n, so we
                // can break the inner loop early. Safe: if conditions change
                // (file paths shown, prereqs included), the group will be in
                // affected_groups and items will be re-evaluated.
                if total_cost > remaining_budget {
                    for invalidate_n in n..=max_n {
                        let item_gen = queue.next_generation();
                        queue.current_gen[group_idx].insert((stage_kind, invalidate_n), item_gen);
                    }
                    break;
                }

                let item_gen = queue.next_generation();
                queue.current_gen[group_idx].insert((stage_kind, n), item_gen);

                queue.heap.push(QueueItem {
                    group_idx,
                    stage_kind,
                    n,
                    own_value,
                    own_cost,
                    prereq_value,
                    prereq_cost,
                    file_path_cost: fp_cost,
                    generation: item_gen,
                });
            }
        }
    }
}

/// Run the greedy scheduling algorithm.
pub fn schedule(
    groups: &[Group],
    budget: usize,
    root: &Path,
    files: &[PathBuf],
) -> Schedule {
    // Build reverse lookup: (file_idx, symbol_idx) → group_idx
    let mut symbol_to_group: HashMap<(usize, usize), usize> = HashMap::new();
    for (group_idx, group) in groups.iter().enumerate() {
        for sc in &group.symbols {
            symbol_to_group.insert((sc.file_idx, sc.symbol_idx), group_idx);
        }
    }

    // Precompute file path costs
    let file_path_costs: Vec<usize> = files
        .iter()
        .map(|f| {
            let relative = f.strip_prefix(root).unwrap_or(f);
            file_path_cost(relative)
        })
        .collect();

    // Reverse index: file_idx → set of group indices that reference this file.
    // Used to efficiently find groups affected when a file's path cost is paid.
    let mut file_to_groups: Vec<Vec<usize>> = vec![Vec::new(); files.len()];
    for (group_idx, group) in groups.iter().enumerate() {
        for &fi in &group.file_indices {
            file_to_groups[fi].push(group_idx);
        }
    }

    // Track which files have been "shown" (path cost already paid)
    let mut files_shown: HashSet<usize> = HashSet::new();

    // Track which (group, stage, n) items have been included
    // For each group: the highest included stage and line count
    let mut group_stages: Vec<Option<IncludedStage>> = vec![None; groups.len()];

    // Build all queue items
    let mut queue = ScheduleQueue::new(groups.len());

    let all_indices: Vec<usize> = (0..groups.len()).collect();
    enqueue_group_items(
        &all_indices,
        groups,
        &group_stages,
        &file_path_costs,
        &files_shown,
        budget,
        &mut queue,
    );

    let mut remaining_budget = budget;

    while let Some(item) = queue.heap.pop() {
        // Lazy deletion: skip stale items
        let current = queue.current_gen[item.group_idx]
            .get(&(item.stage_kind, item.n));
        match current {
            Some(&g) if g == item.generation => {}
            _ => continue,
        }

        // Skip if this item's stage is already covered by the group's current
        // included stage. This prevents double-deduction when a high-level item
        // (e.g. Doc(3)) is included before a lower-level item (e.g. Names):
        // the high-level item pays for Names as a prerequisite, but the original
        // Names heap entry still has a valid generation number. Without this
        // check, popping that stale Names entry would deduct its cost again.
        if group_stages[item.group_idx]
            .as_ref()
            .is_some_and(|inc| {
                let stages = groups[item.group_idx].key.kind_category.stage_sequence();
                inc.covers(stages, item.stage_kind, item.n)
            })
        {
            continue;
        }

        let total_cost = item.total_cost();
        if total_cost > remaining_budget {
            // This item doesn't fit. Try the next one.
            // (Items are ordered by priority, not cost, so a cheaper item
            // might still fit later.)
            continue;
        }

        // Include this item and all its prerequisites
        remaining_budget -= total_cost;

        // Mark file paths as shown, tracking which are newly shown
        let mut newly_shown_files: Vec<usize> = Vec::new();
        for &fi in &groups[item.group_idx].file_indices {
            if files_shown.insert(fi) {
                newly_shown_files.push(fi);
            }
        }

        // Update group stage: include all prerequisites up to this item
        let group = &groups[item.group_idx];
        let stages = group.key.kind_category.stage_sequence();
        // Find the position of this stage in the progression
        let stage_pos = stages.iter().position(|&s| s == item.stage_kind).unwrap();
        for (pos, &sk) in stages.iter().enumerate() {
            let target_n = if sk == item.stage_kind {
                item.n
            } else if pos < stage_pos {
                // Prerequisite stage: include all its lines
                group.max_n(sk)
            } else {
                continue;
            };
            if target_n == 0 {
                continue;
            }
            let should_update = group_stages[item.group_idx]
                .as_ref()
                .is_none_or(|cs| !cs.covers(stages, sk, target_n));
            if should_update {
                group_stages[item.group_idx] = Some(IncludedStage {
                    kind: sk,
                    n_lines: target_n,
                });
            }
        }

        // Update affected items in the queue. Only two kinds of groups need updates:
        // 1. The same group (prerequisite costs changed)
        // 2. Groups sharing *newly shown* files (file path costs just decreased)
        // We use file_to_groups to find case 2 efficiently instead of scanning all groups.
        let mut affected_groups: HashSet<usize> = HashSet::new();
        affected_groups.insert(item.group_idx);
        for &fi in &newly_shown_files {
            for &gi in &file_to_groups[fi] {
                affected_groups.insert(gi);
            }
        }

        let affected: Vec<usize> = affected_groups.into_iter().collect();
        enqueue_group_items(
            &affected,
            groups,
            &group_stages,
            &file_path_costs,
            &files_shown,
            remaining_budget,
            &mut queue,
        );
    }

    Schedule {
        group_stages,
        visible_files: files_shown,
        symbol_to_group,
    }
}

/// Compute the token cost of a single stage for a group.
///
/// For Doc and Body stages, includes the cost delta of standalone truncation
/// markers (`→…`, 1 token each). At Doc/Body(n), symbols with more than n lines
/// get a truncation marker. Advancing from n-1 to n removes markers for symbols
/// that had exactly n-1 remaining lines, so the delta is:
///   markers_at(n) - markers_at(n-1)
/// which is non-positive for n >= 2. The telescoping sum across prerequisites
/// ensures the total cost correctly reflects markers at the final included level.
fn stage_cost(group: &Group, stage: StageKind, n: usize) -> usize {
    match stage {
        // FilePath has zero own_cost — its cost is handled via file_path_cost
        // on the QueueItem, which correctly accounts for cross-group sharing.
        StageKind::FilePath => 0,
        StageKind::Names => group.symbols.iter().map(|s| s.name_tokens).sum(),
        StageKind::Signatures => group.symbols.iter().map(|s| s.signature_tokens).sum(),
        StageKind::Doc => {
            // Suppress truncation marker at the pre-comment/docstring split
            // point. The renderer emits these as two sections separated by
            // the signature; at n == pre_doc_count the pre-comments are fully
            // shown (no marker) and the docstring hasn't started (no marker).
            line_stage_cost(group, n, |s| &s.doc_line_tokens, |s, n| {
                let total = s.doc_line_tokens.len();
                !(total > s.pre_doc_count && n == s.pre_doc_count)
            })
        }
        // Body truncation markers are suppressed for symbols with nested
        // children (e.g., class bodies containing individually-rendered
        // methods). Only count markers for symbols without nested children.
        StageKind::Body => {
            line_stage_cost(group, n, |s| &s.body_line_tokens, |s, _n| !s.body_has_nested)
        }
    }
}

/// Shared cost computation for line-based stages (Doc/Body).
///
/// `get_lines` selects which line-cost vector to read from each symbol.
/// `show_marker` determines whether a symbol shows a truncation marker at
/// a given line index `n`. For Body, this suppresses markers for symbols
/// with nested children. For Doc, this suppresses the marker at the
/// pre-comment/docstring boundary (see `pre_doc_count`).
fn line_stage_cost(
    group: &Group,
    n: usize,
    get_lines: fn(&SymbolCosts) -> &Vec<usize>,
    show_marker: fn(&SymbolCosts, usize) -> bool,
) -> usize {
    let line_cost: usize = group
        .symbols
        .iter()
        .filter_map(|s| get_lines(s).get(n - 1))
        .sum();
    let markers_at_n = group
        .symbols
        .iter()
        .filter(|s| get_lines(s).len() > n && show_marker(s, n))
        .count();
    let markers_at_prev = if n >= 2 {
        group
            .symbols
            .iter()
            .filter(|s| get_lines(s).len() > (n - 1) && show_marker(s, n - 1))
            .count()
    } else {
        0
    };
    (line_cost + markers_at_n).saturating_sub(markers_at_prev)
}

/// Compute prerequisite costs for an item, accounting for already-included stages.
/// Pass `None` for `included` at initial queue construction (no stages included yet).
fn compute_prereq_costs(
    group: &Group,
    stages: &[StageKind],
    stage_pos: usize,
    stage_kind: StageKind,
    n: usize,
    included: Option<&IncludedStage>,
) -> (f64, usize) {
    let mut prereq_value: f64 = 0.0;
    let mut prereq_cost = 0usize;

    let included_pos = included
        .and_then(|inc| stages.iter().position(|&s| s == inc.kind));
    let included_n = included.map(|inc| inc.n_lines).unwrap_or(0);

    for (pos, &sk) in stages.iter().enumerate() {
        if pos >= stage_pos && sk == stage_kind {
            // Include earlier lines of the same stage that aren't yet included
            let already_included_n = if included_pos == Some(pos) {
                included_n
            } else if included_pos.is_some_and(|ip| ip > pos) {
                // This entire stage was included as a prereq of a later stage
                group.max_n(sk)
            } else {
                0
            };
            for earlier_n in (already_included_n + 1)..n {
                prereq_value += compute_value(group, sk, earlier_n);
                prereq_cost += stage_cost(group, sk, earlier_n);
            }
            break;
        }

        if included_pos.is_some_and(|ip| ip > pos) {
            continue; // Fully included as a prereq of a later stage
        }

        // If this stage is the current included stage, only pay for
        // lines beyond what's already included
        let start_n = if included_pos == Some(pos) {
            included_n + 1
        } else {
            1
        };

        for line_n in start_n..=group.max_n(sk) {
            prereq_value += compute_value(group, sk, line_n);
            prereq_cost += stage_cost(group, sk, line_n);
        }
    }

    (prereq_value, prereq_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_role_community_health() {
        assert_eq!(FileRole::from_filename("CONTRIBUTING.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("contributing.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("CONTRIBUTORS.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("SECURITY.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("LICENSE.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("LICENCE.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("License.txt"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("CODE_OF_CONDUCT.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("CODEOWNERS"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("RELEASING.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("SUPPORT.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("GOVERNANCE.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("AUTHORS"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("AUTHORS.md"), FileRole::CommunityHealth);
        assert_eq!(FileRole::from_filename("MAINTAINERS.md"), FileRole::CommunityHealth);
    }

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
        // Non-source-root first components are not skipped
        assert_eq!(effective_depth(Path::new("docs")), 1);
        assert_eq!(effective_depth(Path::new("scripts")), 1);
        assert_eq!(effective_depth(Path::new("docs/conf")), 2);
        assert_eq!(effective_depth(Path::new("internal/pkg")), 2);
        // Source root deeper in path is not skipped
        assert_eq!(effective_depth(Path::new("packages/foo/src")), 3);
    }

    #[test]
    fn file_role_existing_roles() {
        assert_eq!(FileRole::from_filename("README.md"), FileRole::Readme);
        assert_eq!(FileRole::from_filename("CHANGELOG.md"), FileRole::Changelog);
        assert_eq!(FileRole::from_filename("CHANGES.md"), FileRole::Changelog);
        assert_eq!(FileRole::from_filename("README_zh-CN.md"), FileRole::Translated);
        assert_eq!(FileRole::from_filename("README-es.md"), FileRole::Translated);
        assert_eq!(FileRole::from_filename("lib.rs"), FileRole::Normal);
        assert_eq!(FileRole::from_filename("main.py"), FileRole::Normal);
    }

    #[test]
    fn file_role_ai_config() {
        assert_eq!(FileRole::from_filename("CLAUDE.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("claude.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("AGENTS.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("agents.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("COPILOT.md"), FileRole::AiConfig);
        assert_eq!(FileRole::from_filename("copilot-instructions.md"), FileRole::AiConfig);
        // Non-doc extensions should not match
        assert_eq!(FileRole::from_filename("claude.toml"), FileRole::Normal);
        assert_eq!(FileRole::from_filename("agents.json"), FileRole::Normal);
    }

    #[test]
    fn file_role_from_path_locale_dirs() {
        // Locale directory patterns (xx-YY) detect translated docs
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/guide.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("docs/pt-BR/readme.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("docs/en_US/intro.md")), FileRole::Translated);
        // Well-known i18n directory names
        assert_eq!(FileRole::from_path(Path::new("i18n/guide.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("l10n/guide.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("locales/readme.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("translations/guide.md")), FileRole::Translated);
        // Non-doc files in locale dirs stay Normal
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/lib.rs")), FileRole::Normal);
        // Plain 2-letter dirs are NOT matched (too many false positives)
        assert_eq!(FileRole::from_path(Path::new("docs/go/guide.md")), FileRole::Normal);
        assert_eq!(FileRole::from_path(Path::new("docs/js/guide.md")), FileRole::Normal);
        // Locale directory overrides filename-based roles (translated README is low value)
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/README.md")), FileRole::Translated);
        assert_eq!(FileRole::from_path(Path::new("docs/zh-CN/CHANGELOG.md")), FileRole::Translated);
        // Root-level files work normally
        assert_eq!(FileRole::from_path(Path::new("README.md")), FileRole::Readme);
        assert_eq!(FileRole::from_path(Path::new("guide.md")), FileRole::Normal);
    }

    #[test]
    fn config_file_detection() {
        // Helper: root-level file (no parent dir)
        fn at_root(name: &str) -> bool {
            is_config_file(Path::new(name), name)
        }
        // Helper: file in a subdirectory
        fn at_path(path: &str) -> bool {
            let p = Path::new(path);
            let name = p.file_name().unwrap().to_str().unwrap();
            is_config_file(p, name)
        }

        // *.config.{ext} pattern (matched anywhere)
        assert!(at_root("eslint.config.js"));
        assert!(at_root("jest.config.ts"));
        assert!(at_root("vite.config.mjs"));
        assert!(at_root("next.config.js"));
        assert!(at_root("next.config.mjs"));
        assert!(at_root("tailwind.config.ts"));
        assert!(at_root("postcss.config.js"));
        assert!(at_root("tsup.config.ts"));
        assert!(at_root("playwright.config.ts"));
        assert!(at_root("rollup.config.js"));
        assert!(at_root("webpack.config.js"));
        assert!(at_root("babel.config.js"));
        assert!(at_root("vitest.config.ts"));
        assert!(at_root("theme.config.jsx"));
        // Case insensitive
        assert!(at_root("ESLint.Config.JS"));
        // .{name}rc.{ext} pattern
        assert!(at_root(".eslintrc.js"));
        assert!(at_root(".babelrc.js"));
        assert!(at_root(".prettierrc.mjs"));
        // Build scripts at project root
        assert!(at_root("build.rs"));
        assert!(at_root("Build.rs")); // case insensitive
        assert!(at_root("setup.py"));
        assert!(at_root("Setup.py"));
        // Build scripts in subdirs are NOT config files (e.g., src/cmd/build.rs)
        assert!(!at_path("src/cmd/build.rs"));
        assert!(!at_path("lib/setup.py"));
        // JS task runners (matched anywhere)
        assert!(at_root("Gruntfile.js"));
        assert!(at_root("gruntfile.js"));
        assert!(at_root("Gulpfile.js"));
        assert!(at_root("gulpfile.js"));
        assert!(at_root("Jakefile.js"));
        assert!(at_path("tools/Gulpfile.js")); // task runners match in subdirs too
        // Not config files
        assert!(!at_root("main.js"));
        assert!(!at_root("lib.rs"));
        assert!(!at_root("index.ts"));
        assert!(!at_root("config.js")); // no *.config.* pattern
        assert!(!at_root("my_config.py"));
        assert!(!at_root("README.md"));
        assert!(!at_root("build.go")); // build.rs is Rust-specific
        assert!(!at_root("setup.rs")); // setup.py is Python-specific
        // Files in scripts/ and tools/ directories
        assert!(at_path("scripts/release.py"));
        assert!(at_path("scripts/build.sh"));
        assert!(at_path("tools/lint.py"));
        assert!(at_path("tool/gen.rs"));
        assert!(at_path("script/deploy.js"));
        // Nested scripts dir also matches
        assert!(at_path("ci/scripts/test.sh"));
        // Case insensitive
        assert!(at_path("Scripts/release.py"));
        // Normal source dirs are NOT matched
        assert!(!at_path("src/main.rs"));
        assert!(!at_path("lib/index.js"));
        assert!(!at_path("pkg/server.go"));
    }
}
