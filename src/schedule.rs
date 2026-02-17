use std::collections::{BinaryHeap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::format;
use crate::parse;
use crate::Lang;

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
        }
    }

    /// Ordered stage progression for this kind category.
    pub fn stage_sequence(&self) -> &'static [StageKind] {
        match self {
            // Types: body before doc (struct fields/enum variants are the useful content)
            KindCategory::Type => &[
                StageKind::Names,
                StageKind::Signatures,
                StageKind::Body,
                StageKind::Doc,
            ],
            // Markdown: just names (headings) and body text
            KindCategory::Section => &[StageKind::Names, StageKind::Body],
            // Everything else: names → signatures → doc → body
            _ => &[
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

/// Detect build/tool configuration files by filename and path convention.
/// These are files like `eslint.config.js`, `jest.config.ts`, `.eslintrc.js`, etc.
/// that configure development tools rather than implementing library/app logic.
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

    false
}

/// Grouping dimensions. All symbols sharing a GroupKey are treated identically.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroupKey {
    pub is_public: bool,
    pub kind_category: KindCategory,
    pub parent_dir: PathBuf,
    pub extension: String,
    pub is_documented: bool,
    pub file_role: FileRole,
    /// Whether the file is a build/tool config file (e.g., eslint.config.js, jest.config.ts).
    pub is_config: bool,
}

/// A symbol's precomputed content costs (word counts per rendering stage).
pub struct SymbolCosts {
    pub file_idx: usize,
    pub symbol_idx: usize,
    pub name_words: usize,
    /// Additional words for full signature beyond the name line.
    pub signature_words: usize,
    /// Words per doc comment line (ordered).
    pub doc_line_words: Vec<usize>,
    /// Words per body line (ordered).
    pub body_line_words: Vec<usize>,
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
    /// Max line count for a Doc/Body stage. Returns 1 for Names/Signatures.
    fn max_n(&self, stage: StageKind) -> usize {
        match stage {
            StageKind::Names | StageKind::Signatures => 1,
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
    /// Total words the scheduler estimated it would use (budget - remaining).
    pub estimated_words: usize,
}

/// What stage a group has been included up to.
#[derive(Debug, Clone, Copy)]
pub struct IncludedStage {
    pub kind: StageKind,
    /// For Doc/Body stages: how many lines to show. Ignored for Names/Signatures.
    pub n_lines: usize,
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
) -> Vec<Group> {
    let mut group_map: HashMap<GroupKey, Group> = HashMap::new();

    for (file_idx, (file, symbols)) in files.iter().zip(all_symbols.iter()).enumerate() {
        let source = match &sources[file_idx] {
            Some(s) => s,
            None => continue,
        };
        let relative = file.strip_prefix(root).unwrap_or(file);
        let lang = Lang::from_path(relative);
        let parent_dir = relative
            .parent()
            .unwrap_or(Path::new(""))
            .to_path_buf();
        let extension = relative
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        let lines: Vec<&str> = source.lines().collect();
        let file_role = FileRole::from_path(relative);
        let is_config = relative.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|name| is_config_file(relative, name));

        for (symbol_idx, sym) in symbols.iter().enumerate() {
            let sym_line_0 = sym.line - 1;
            let kind_category = KindCategory::from_symbol_kind(sym.kind);

            // Detect documentation
            let doc_start = format::doc_comment_start(&lines, sym_line_0, lang);
            let is_documented = doc_start < sym_line_0;

            let key = GroupKey {
                is_public: sym.is_public,
                kind_category,
                parent_dir: parent_dir.clone(),
                extension: extension.clone(),
                is_documented,
                file_role,
                is_config,
            };

            // Compute costs
            let costs = compute_symbol_costs(
                file_idx,
                symbol_idx,
                sym,
                &lines,
                lang,
                doc_start,
                symbols,
            );

            let group = group_map.entry(key.clone()).or_insert_with(|| Group {
                key,
                symbols: Vec::new(),
                file_indices: HashSet::new(),
                max_doc_n: 0,
                max_body_n: 0,
            });
            group.max_doc_n = group.max_doc_n.max(costs.doc_line_words.len());
            group.max_body_n = group.max_body_n.max(costs.body_line_words.len());
            group.symbols.push(costs);
            group.file_indices.insert(file_idx);
        }
    }

    let mut groups: Vec<Group> = group_map.into_values().collect();
    groups.sort_by(|a, b| a.key.cmp(&b.key));
    groups
}

/// Compute word costs for each rendering stage of a single symbol.
fn compute_symbol_costs(
    file_idx: usize,
    symbol_idx: usize,
    sym: &parse::Symbol,
    lines: &[&str],
    lang: Option<Lang>,
    doc_start: usize,
    all_symbols: &[parse::Symbol],
) -> SymbolCosts {
    let sym_line_0 = sym.line - 1;

    // Name: word count of the formatted name line, plus 1 for the " …" suffix
    // that the renderer appends to names-only entries. Including the ellipsis
    // cost here keeps Names-only rendering in sync with the scheduler's budget.
    // When Signatures are also included (no ellipsis rendered), the +1 is offset
    // by a corresponding -1 in signature_words (since signature_words = sig_total
    // - name_words), so the total Names+Signatures cost remains exact.
    let name_line = format::format_symbol_name(sym, lines);
    let name_words = format::count_words(&name_line) + 1;

    // Signature: additional words from showing full signature lines (with line numbers)
    // beyond what the name-only line shows.
    let sig_end = format::signature_end_line(lines, sym, lang);
    let is_section = sym.kind == parse::SymbolKind::Section;
    let mut sig_formatted_words = 0;
    for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
        // Strip trailing badges from markdown heading lines (matches renderer)
        let line = if is_section && i == sym_line_0 {
            format::strip_heading_badges(line)
        } else {
            line
        };
        sig_formatted_words += format::count_words(&format::fmt_line(i, line));
    }
    let signature_words = sig_formatted_words.saturating_sub(name_words);

    // Doc comment lines
    let mut doc_line_words = Vec::new();
    if doc_start < sym_line_0 {
        for (i, line) in lines.iter().enumerate().take(sym_line_0).skip(doc_start) {
            doc_line_words.push(format::count_words(&format::fmt_line(i, line)));
        }
    }
    // Python docstrings: lines after the signature
    if lang == Some(Lang::Python) {
        let ds_end = format::docstring_end(lines, sig_end);
        if ds_end > sig_end + 1 {
            for (i, line) in lines.iter().enumerate().take(ds_end).skip(sig_end + 1) {
                doc_line_words.push(format::count_words(&format::fmt_line(i, line)));
            }
        }
    }

    // Body lines (after signature, up to end_line)
    let mut body_line_words = Vec::new();
    let body_start = sig_end + 1;
    let body_end = sym.end_line.min(lines.len());

    if sym.kind == parse::SymbolKind::Section {
        // Markdown: body is content text after heading, up to next heading
        let next_heading_line = all_symbols
            .iter()
            .skip(symbol_idx + 1)
            .find(|s| s.kind == parse::SymbolKind::Section)
            .map(|s| s.line - 1)
            .unwrap_or(lines.len());
        // Skip leading noise (badges, link refs, blank lines) after heading
        let heading_end = (sym.end_line - 1).max(sym_line_0 + 1);
        let mut content_start = heading_end;
        while content_start < next_heading_line
            && format::is_markdown_leading_noise(lines[content_start])
        {
            content_start += 1;
        }
        for (i, line) in lines.iter().enumerate().take(next_heading_line).skip(content_start) {
            body_line_words.push(format::count_words(&format::fmt_line(i, line)));
        }
    } else {
        // Code: body lines after signature
        // Skip Python docstrings if present (already counted in doc_line_words)
        let actual_body_start = if lang == Some(Lang::Python) {
            let ds_end = format::docstring_end(lines, sig_end);
            if ds_end > sig_end + 1 { ds_end } else { body_start }
        } else {
            body_start
        };
        for (i, line) in lines.iter().enumerate().take(body_end).skip(actual_body_start) {
            body_line_words.push(format::count_words(&format::fmt_line(i, line)));
        }
    }

    // Check if body region contains nested symbols (used to predict
    // whether the renderer will suppress body truncation markers).
    let body_has_nested = if sym.kind == parse::SymbolKind::Section {
        false // Markdown sections don't suppress truncation markers
    } else {
        let body_end = sym.end_line.min(lines.len());
        let actual_body_start = if lang == Some(Lang::Python) {
            let ds_end = format::docstring_end(lines, sig_end);
            if ds_end > sig_end + 1 { ds_end } else { sig_end + 1 }
        } else {
            sig_end + 1
        };
        all_symbols
            .iter()
            .skip(symbol_idx + 1)
            .any(|s| {
                let sl = s.line - 1;
                sl >= actual_body_start && sl < body_end
            })
    };

    SymbolCosts {
        file_idx,
        symbol_idx,
        name_words,
        signature_words,
        doc_line_words,
        body_line_words,
        body_has_nested,
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
    let depth = effective_depth(&key.parent_dir);
    let depth_factor = match depth {
        0..=1 => 1.0,
        2..=3 => 0.7,
        _ => 0.4,
    };

    // Groups with many symbols: each individual symbol is less important.
    // Gentle log decay: 1 sym → 1.0, 2 → 0.94, 5 → 0.86, 10 → 0.81, 50 → 0.72.
    let sibling_count = group.symbols.len().max(1) as f64;
    let sibling_factor = 1.0 / (1.0 + sibling_count.ln() * 0.1);

    // File role: README files are high-signal, changelogs and translations are low-signal.
    let file_role_factor = match key.file_role {
        FileRole::Readme => 1.5,
        FileRole::Normal => 1.0,
        FileRole::Translated => 0.1,
        FileRole::Changelog => 0.1,
        FileRole::CommunityHealth => 0.1,
    };

    // Config files (eslint.config.js, jest.config.ts, etc.) are build/tool setup,
    // not core library logic. Show them only when there's plenty of budget.
    let config_factor = if key.is_config { 0.2 } else { 1.0 };

    let base_value = visibility * documented * depth_factor * sibling_factor * file_role_factor * config_factor;

    let stage_value = match key.kind_category {
        KindCategory::Type => match stage {
            StageKind::Names => 1.0,
            StageKind::Signatures => 0.7,
            StageKind::Body => 0.6,
            StageKind::Doc => 0.4,
        },
        KindCategory::Section => match stage {
            StageKind::Names => 1.0,
            StageKind::Body => 0.5,
            _ => 0.1,
        },
        _ => match stage {
            StageKind::Names => 1.0,
            StageKind::Signatures => 0.7,
            StageKind::Doc => 0.5,
            StageKind::Body => 0.2,
        },
    };

    base_value * stage_value / n as f64
}

// ---------------------------------------------------------------------------
// Priority queue algorithm
// ---------------------------------------------------------------------------

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

/// Compute the word cost of a file path line.
fn file_path_cost(relative_path: &Path) -> usize {
    // Path line is just the relative path as text, plus a newline.
    format::count_words(&format!("{}\n", relative_path.display()))
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
    let mut generation: u64 = 0;
    let mut current_gen: Vec<HashMap<(StageKind, usize), u64>> =
        vec![HashMap::new(); groups.len()];
    let mut heap: BinaryHeap<QueueItem> = BinaryHeap::new();

    for (group_idx, group) in groups.iter().enumerate() {
        let stages = group.key.kind_category.stage_sequence();
        for (stage_pos, &stage_kind) in stages.iter().enumerate() {
            let max_n = group.max_n(stage_kind);
            if max_n == 0 {
                continue;
            }
            for n in 1..=max_n {
                let own_value = compute_value(group, stage_kind, n);
                let own_cost = stage_cost(group, stage_kind, n);

                // Compute prerequisite costs: all earlier stages not yet included
                let (prereq_value, prereq_cost) =
                    compute_prereq_costs(group, group_idx, stages, stage_pos, stage_kind, n);

                // File path costs for files not yet shown
                let fp_cost: usize = group
                    .file_indices
                    .iter()
                    .map(|&fi| file_path_costs[fi])
                    .sum();

                // Budget pruning: skip items that can never fit. For Doc/Body
                // stages, total_cost is monotonically non-decreasing with n
                // (each line adds to prerequisite costs), so we can break early.
                if own_cost + prereq_cost + fp_cost > budget {
                    break;
                }

                let item_gen = generation;
                generation += 1;
                current_gen[group_idx].insert((stage_kind, n), item_gen);

                heap.push(QueueItem {
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

    let mut remaining_budget = budget;

    while let Some(item) = heap.pop() {
        // Lazy deletion: skip stale items
        let current = current_gen[item.group_idx]
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
        if let Some(ref included) = group_stages[item.group_idx] {
            let stages = groups[item.group_idx].key.kind_category.stage_sequence();
            let inc_pos = stages.iter().position(|&s| s == included.kind);
            let this_pos = stages.iter().position(|&s| s == item.stage_kind);
            if let (Some(ip), Some(tp)) = (inc_pos, this_pos)
                && (tp < ip || (tp == ip && item.n <= included.n_lines))
            {
                continue;
            }
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
        for (pos, &sk) in stages.iter().enumerate() {
            let stage_pos = stages.iter().position(|&s| s == item.stage_kind).unwrap();
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
            let current_stage = group_stages[item.group_idx];
            let should_update = match current_stage {
                None => true,
                Some(ref cs) => {
                    let current_pos = stages.iter().position(|&s| s == cs.kind);
                    let new_pos = stages.iter().position(|&s| s == sk);
                    match (current_pos, new_pos) {
                        (Some(cp), Some(np)) => {
                            np > cp || (np == cp && target_n > cs.n_lines)
                        }
                        _ => true,
                    }
                }
            };
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

        for &other_group_idx in &affected_groups {
            let other_group = &groups[other_group_idx];
            let other_stages = other_group.key.kind_category.stage_sequence();
            for &sk in other_stages {
                let max_n = other_group.max_n(sk);
                for n in 1..=max_n {
                    // Skip items already included
                    if let Some(ref included) = group_stages[other_group_idx] {
                        let inc_pos = other_stages.iter().position(|&s| s == included.kind);
                        let this_pos = other_stages.iter().position(|&s| s == sk);
                        if let (Some(ip), Some(tp)) = (inc_pos, this_pos)
                            && (tp < ip || (tp == ip && n <= included.n_lines))
                        {
                            continue;
                        }
                    }

                    let stage_pos = other_stages.iter().position(|&s| s == sk).unwrap();
                    let own_value = compute_value(other_group, sk, n);
                    let own_cost = stage_cost(other_group, sk, n);
                    let (prereq_value, prereq_cost) = compute_prereq_costs_with_state(
                        other_group,
                        other_group_idx,
                        other_stages,
                        stage_pos,
                        sk,
                        n,
                        &group_stages,
                    );
                    let fp_cost: usize = other_group
                        .file_indices
                        .iter()
                        .filter(|fi| !files_shown.contains(fi))
                        .map(|&fi| file_path_costs[fi])
                        .sum();

                    let total_cost = own_cost + prereq_cost + fp_cost;

                    // Budget pruning: skip items that can't fit in remaining
                    // budget. Invalidate their generation so stale heap entries
                    // are ignored. For Doc/Body, total_cost is monotonically
                    // non-decreasing with n, so break the inner loop early.
                    // Safe: if conditions change (file paths shown, prereqs
                    // included), the group will be in affected_groups and
                    // items will be re-evaluated.
                    if total_cost > remaining_budget {
                        // Invalidate any stale heap entries for this and
                        // all remaining n values
                        for invalidate_n in n..=max_n {
                            let item_gen = generation;
                            generation += 1;
                            current_gen[other_group_idx].insert((sk, invalidate_n), item_gen);
                        }
                        break;
                    }

                    let item_gen = generation;
                    generation += 1;
                    current_gen[other_group_idx].insert((sk, n), item_gen);

                    heap.push(QueueItem {
                        group_idx: other_group_idx,
                        stage_kind: sk,
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

    Schedule {
        group_stages,
        visible_files: files_shown,
        symbol_to_group,
        estimated_words: budget - remaining_budget,
    }
}

/// Compute the word cost of a single stage for a group.
///
/// For Doc and Body stages, includes the cost delta of standalone truncation
/// markers (`→…`, 1 word each). At Doc/Body(n), symbols with more than n lines
/// get a truncation marker. Advancing from n-1 to n removes markers for symbols
/// that had exactly n-1 remaining lines, so the delta is:
///   markers_at(n) - markers_at(n-1)
/// which is non-positive for n >= 2. The telescoping sum across prerequisites
/// ensures the total cost correctly reflects markers at the final included level.
fn stage_cost(group: &Group, stage: StageKind, n: usize) -> usize {
    match stage {
        StageKind::Names => group.symbols.iter().map(|s| s.name_words).sum(),
        StageKind::Signatures => group.symbols.iter().map(|s| s.signature_words).sum(),
        StageKind::Doc => {
            let line_cost: usize = group
                .symbols
                .iter()
                .filter_map(|s| s.doc_line_words.get(n - 1))
                .sum();
            // Truncation markers: count symbols with more doc lines than shown
            let markers_at_n = group
                .symbols
                .iter()
                .filter(|s| s.doc_line_words.len() > n)
                .count();
            let markers_at_prev = if n >= 2 {
                group
                    .symbols
                    .iter()
                    .filter(|s| s.doc_line_words.len() > (n - 1))
                    .count()
            } else {
                0
            };
            (line_cost + markers_at_n).saturating_sub(markers_at_prev)
        }
        StageKind::Body => {
            let line_cost: usize = group
                .symbols
                .iter()
                .filter_map(|s| s.body_line_words.get(n - 1))
                .sum();
            // Body truncation markers are suppressed for symbols with nested
            // children (e.g., class bodies containing individually-rendered
            // methods). Only count markers for symbols without nested children.
            let markers_at_n = group
                .symbols
                .iter()
                .filter(|s| s.body_line_words.len() > n && !s.body_has_nested)
                .count();
            let markers_at_prev = if n >= 2 {
                group
                    .symbols
                    .iter()
                    .filter(|s| s.body_line_words.len() > (n - 1) && !s.body_has_nested)
                    .count()
            } else {
                0
            };
            (line_cost + markers_at_n).saturating_sub(markers_at_prev)
        }
    }
}

/// Compute prerequisite costs for an item at initial queue construction
/// (no stages included yet).
fn compute_prereq_costs(
    group: &Group,
    _group_idx: usize,
    stages: &[StageKind],
    stage_pos: usize,
    stage_kind: StageKind,
    n: usize,
) -> (f64, usize) {
    let mut prereq_value: f64 = 0.0;
    let mut prereq_cost = 0usize;

    for (pos, &sk) in stages.iter().enumerate() {
        if pos >= stage_pos && sk == stage_kind {
            // Include earlier lines of the same stage (Doc(1) through Doc(n-1))
            for earlier_n in 1..n {
                prereq_value += compute_value(group, sk, earlier_n);
                prereq_cost += stage_cost(group, sk, earlier_n);
            }
            break;
        }
        // Include all lines of prerequisite stages
        for line_n in 1..=group.max_n(sk) {
            prereq_value += compute_value(group, sk, line_n);
            prereq_cost += stage_cost(group, sk, line_n);
        }
    }

    (prereq_value, prereq_cost)
}

/// Compute prerequisite costs accounting for already-included stages.
fn compute_prereq_costs_with_state(
    group: &Group,
    group_idx: usize,
    stages: &[StageKind],
    stage_pos: usize,
    stage_kind: StageKind,
    n: usize,
    group_stages: &[Option<IncludedStage>],
) -> (f64, usize) {
    let mut prereq_value: f64 = 0.0;
    let mut prereq_cost = 0usize;

    let included = &group_stages[group_idx];
    let included_pos = included
        .as_ref()
        .and_then(|inc| stages.iter().position(|&s| s == inc.kind));
    let included_n = included.as_ref().map(|inc| inc.n_lines).unwrap_or(0);

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
    }
}
