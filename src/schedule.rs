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

/// Grouping dimensions. All symbols sharing a GroupKey are treated identically.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GroupKey {
    pub is_public: bool,
    pub kind_category: KindCategory,
    pub parent_dir: PathBuf,
    pub extension: String,
    pub is_documented: bool,
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
}

/// A group of similarly-valued symbols that always receive the same treatment.
pub struct Group {
    pub key: GroupKey,
    pub symbols: Vec<SymbolCosts>,
    pub file_indices: HashSet<usize>,
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
            });
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

    // Name: word count of the formatted name line
    let name_line = format::format_symbol_name(sym, lines);
    let name_words = format::count_words(&name_line);

    // Signature: additional words from showing full signature lines (with line numbers)
    // beyond what the name-only line shows.
    let sig_end = format::signature_end_line(lines, sym, lang);
    let mut sig_formatted_words = 0;
    for (i, line) in lines.iter().enumerate().take(sig_end + 1).skip(sym_line_0) {
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
        // Skip blank lines after heading, then include content
        let heading_end = (sym.end_line - 1).max(sym_line_0 + 1);
        for (i, line) in lines.iter().enumerate().take(next_heading_line).skip(heading_end) {
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

    SymbolCosts {
        file_idx,
        symbol_idx,
        name_words,
        signature_words,
        doc_line_words,
        body_line_words,
    }
}

// ---------------------------------------------------------------------------
// Value computation
// ---------------------------------------------------------------------------

/// Compute the value of showing a particular stage for a group.
fn compute_value(group: &Group, stage: StageKind, n: usize) -> f64 {
    let key = &group.key;

    let visibility = if key.is_public { 1.0 } else { 0.3 };
    let documented = if key.is_documented { 1.0 } else { 0.5 };

    let depth = key.parent_dir.components().count();
    let depth_factor = match depth {
        0..=1 => 1.0,
        2..=3 => 0.7,
        _ => 0.4,
    };

    // Groups with many symbols: each individual symbol is less important.
    // Gentle log decay: 1 sym → 1.0, 2 → 0.94, 5 → 0.86, 10 → 0.81, 50 → 0.72.
    let sibling_count = group.symbols.len().max(1) as f64;
    let sibling_factor = 1.0 / (1.0 + sibling_count.ln() * 0.1);

    let base_value = visibility * documented * depth_factor * sibling_factor;

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
            let max_n = match stage_kind {
                StageKind::Names | StageKind::Signatures => 1,
                StageKind::Doc => group
                    .symbols
                    .iter()
                    .map(|s| s.doc_line_words.len())
                    .max()
                    .unwrap_or(0),
                StageKind::Body => group
                    .symbols
                    .iter()
                    .map(|s| s.body_line_words.len())
                    .max()
                    .unwrap_or(0),
            };
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
            let target_n = if sk == item.stage_kind {
                item.n
            } else if matches!(sk, StageKind::Doc | StageKind::Body) {
                // Prerequisites: if this stage is before our target in the sequence,
                // we need to include all its lines
                let stage_pos = stages.iter().position(|&s| s == item.stage_kind).unwrap();
                if pos < stage_pos {
                    match sk {
                        StageKind::Doc => group
                            .symbols
                            .iter()
                            .map(|s| s.doc_line_words.len())
                            .max()
                            .unwrap_or(0),
                        StageKind::Body => group
                            .symbols
                            .iter()
                            .map(|s| s.body_line_words.len())
                            .max()
                            .unwrap_or(0),
                        _ => 1,
                    }
                } else {
                    continue;
                }
            } else {
                let stage_pos = stages.iter().position(|&s| s == item.stage_kind).unwrap();
                if pos <= stage_pos {
                    1
                } else {
                    continue;
                }
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
                let max_n = match sk {
                    StageKind::Names | StageKind::Signatures => 1,
                    StageKind::Doc => other_group
                        .symbols
                        .iter()
                        .map(|s| s.doc_line_words.len())
                        .max()
                        .unwrap_or(0),
                    StageKind::Body => other_group
                        .symbols
                        .iter()
                        .map(|s| s.body_line_words.len())
                        .max()
                        .unwrap_or(0),
                };
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
fn stage_cost(group: &Group, stage: StageKind, n: usize) -> usize {
    match stage {
        StageKind::Names => group.symbols.iter().map(|s| s.name_words).sum(),
        StageKind::Signatures => group.symbols.iter().map(|s| s.signature_words).sum(),
        StageKind::Doc => {
            // Cost of showing doc line `n` (1-indexed) for all symbols that have it
            group
                .symbols
                .iter()
                .filter_map(|s| s.doc_line_words.get(n - 1))
                .sum()
        }
        StageKind::Body => {
            group
                .symbols
                .iter()
                .filter_map(|s| s.body_line_words.get(n - 1))
                .sum()
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
        let max_n = match sk {
            StageKind::Names | StageKind::Signatures => 1,
            StageKind::Doc => group
                .symbols
                .iter()
                .map(|s| s.doc_line_words.len())
                .max()
                .unwrap_or(0),
            StageKind::Body => group
                .symbols
                .iter()
                .map(|s| s.body_line_words.len())
                .max()
                .unwrap_or(0),
        };
        for line_n in 1..=max_n {
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
                match sk {
                    StageKind::Names | StageKind::Signatures => 1,
                    StageKind::Doc => group
                        .symbols
                        .iter()
                        .map(|s| s.doc_line_words.len())
                        .max()
                        .unwrap_or(0),
                    StageKind::Body => group
                        .symbols
                        .iter()
                        .map(|s| s.body_line_words.len())
                        .max()
                        .unwrap_or(0),
                }
            } else {
                0
            };
            for earlier_n in (already_included_n + 1)..n {
                prereq_value += compute_value(group, sk, earlier_n);
                prereq_cost += stage_cost(group, sk, earlier_n);
            }
            break;
        }

        // Check if this prerequisite stage is already included
        let max_n = match sk {
            StageKind::Names | StageKind::Signatures => 1,
            StageKind::Doc => group
                .symbols
                .iter()
                .map(|s| s.doc_line_words.len())
                .max()
                .unwrap_or(0),
            StageKind::Body => group
                .symbols
                .iter()
                .map(|s| s.body_line_words.len())
                .max()
                .unwrap_or(0),
        };

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

        for line_n in start_n..=max_n {
            prereq_value += compute_value(group, sk, line_n);
            prereq_cost += stage_cost(group, sk, line_n);
        }
    }

    (prereq_value, prereq_cost)
}
