//! LineEntry types, RenderedEntry, per-file cache, assembly, and override resolution.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use crate::format;
use crate::group::{Group, GroupCtx};

// ---------------------------------------------------------------------------
// LineEntry (design §3.4)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum LineEntry<'src> {
    /// Full source line.
    Complete { line: u32, content: &'src str },
    /// Prefix of a source line, the rest omitted.
    Truncated { line: u32, content: &'src str },
    /// Ellipsis placeholder at a specific source line.
    Ellipsis { line: u32 },
}

impl<'src> LineEntry<'src> {
    pub fn line(&self) -> u32 {
        match self {
            LineEntry::Complete { line, .. }
            | LineEntry::Truncated { line, .. }
            | LineEntry::Ellipsis { line } => *line,
        }
    }

    /// Content rank for override resolution (R4).
    pub fn content_rank(&self) -> u8 {
        match self {
            LineEntry::Ellipsis { .. } => 0,
            LineEntry::Truncated { .. } => 1,
            LineEntry::Complete { .. } => 2,
        }
    }

    pub fn content_len(&self) -> usize {
        match self {
            LineEntry::Complete { content, .. } | LineEntry::Truncated { content, .. } => {
                content.len()
            }
            LineEntry::Ellipsis { .. } => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// RenderedEntry — a LineEntry with its pre-computed formatted string and costs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RenderedEntry<'src> {
    pub entry: LineEntry<'src>,
    pub formatted: String,
    pub tokens: usize,
    pub chars: usize,
}

/// Format a LineEntry and compute its token/char cost. Called once per entry.
pub fn render_entry<'s>(entry: &LineEntry<'s>) -> RenderedEntry<'s> {
    let formatted = match entry {
        LineEntry::Complete { line, content } => format::fmt_line(*line as usize, content),
        LineEntry::Truncated { line, content } => {
            let base = format::fmt_line(*line as usize, content);
            format!("{} …\n", base.trim_end())
        }
        LineEntry::Ellipsis { .. } => format::TRUNCATION_MARKER.to_string(),
    };
    let tokens = format::count_tokens(&formatted);
    let chars = formatted.len();
    RenderedEntry {
        entry: entry.clone(),
        formatted,
        tokens,
        chars,
    }
}

// ---------------------------------------------------------------------------
// CachedGroupRender — stored on the group after first probe
// ---------------------------------------------------------------------------

/// The result of rendering a group: per-file rendered entries and the total
/// marginal cost. Computed once on first probe, reused on every subsequent
/// probe and at commit time.
#[derive(Debug, Clone)]
pub struct CachedGroupRender<'s> {
    pub per_file: Vec<(PathBuf, Vec<RenderedEntry<'s>>)>,
    pub marginal_cost: FileCost,
}

// ---------------------------------------------------------------------------
// Scheduler renderer boundary
// ---------------------------------------------------------------------------

/// Renderer contract used by the scheduler.
///
/// The scheduler decides which group wins next; the renderer decides what that
/// group costs, how a commit affects the rendered state, and how to assemble
/// the final output.
pub trait SchedulerRenderer<'s> {
    fn prepare(&mut self, group: &mut Group<'s>, ctx: &GroupCtx<'s>);
    fn probe_cost(&mut self, group: &Group<'s>, ctx: &GroupCtx<'s>) -> FileCost;
    fn commit(&mut self, group: &mut Group<'s>, ctx: &GroupCtx<'s>) -> BudgetDelta;
    fn assemble(&self, char_budget: Option<usize>) -> String;
}

#[derive(Copy, Clone, Default, Debug)]
pub struct BudgetDelta {
    pub charge: FileCost,
    pub refund: FileCost,
}

impl BudgetDelta {
    pub fn charge(charge: FileCost) -> Self {
        Self {
            charge,
            refund: FileCost::default(),
        }
    }

    pub fn apply_to(&self, remaining_tokens: &mut usize, remaining_chars: &mut Option<usize>) {
        *remaining_tokens += self.refund.tokens;
        *remaining_tokens = remaining_tokens.saturating_sub(self.charge.tokens);

        if let Some(rc) = remaining_chars {
            *rc += self.refund.chars;
            *rc = rc.saturating_sub(self.charge.chars);
        }
    }
}

pub struct TextRenderer {
    cache: FileCache,
    // Folders currently planned in the output and already charged to the
    // budget. When a child group commits under a folder, the folder is removed
    // from the map and its cost refunded.
    childless_folders: HashMap<PathBuf, FileCost>,
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            cache: FileCache::new(),
            childless_folders: HashMap::new(),
        }
    }

    /// Compute per-item folder costs for a FoldersGroup, caching the result.
    fn ensure_folders_cached(g: &mut crate::group::FoldersGroup, ctx: &GroupCtx<'_>) {
        if g.cached_item_costs.is_some() {
            return;
        }
        let costs: Vec<FileCost> = g
            .items
            .iter()
            .map(|item_dir| {
                let rel = ctx.rel_path(item_dir);
                let header = format::folder_line(rel);
                FileCost {
                    tokens: format::count_tokens(&header),
                    chars: header.len(),
                }
            })
            .collect();
        g.cached_item_costs = Some(costs);
    }
}

impl<'s> SchedulerRenderer<'s> for TextRenderer {
    fn prepare(&mut self, group: &mut Group<'s>, ctx: &GroupCtx<'s>) {
        if let Group::Folders(g) = group {
            Self::ensure_folders_cached(g, ctx);
            return;
        }
        let Group::Ts(g) = group else { return };
        if g.cached_render.is_some() {
            return;
        }

        let raw = crate::group::ts::render_entries(g);

        let mut per_file: Vec<(PathBuf, Vec<RenderedEntry<'s>>)> = Vec::new();
        for (path, entries) in raw {
            let rendered: Vec<RenderedEntry<'s>> =
                entries.iter().map(|e| render_entry(e)).collect();
            per_file.push((path, rendered));
        }

        let marginal_cost = self.cache.marginal_cost(&per_file);

        g.cached_render = Some(CachedGroupRender {
            per_file,
            marginal_cost,
        });
    }

    fn probe_cost(&mut self, group: &Group<'s>, ctx: &GroupCtx<'s>) -> FileCost {
        match group {
            Group::Ts(g) => g.cached_render.as_ref().unwrap().marginal_cost,
            Group::Folders(g) => {
                let costs = g.cached_item_costs.as_ref().unwrap();
                let mut fc = FileCost::default();
                for c in costs {
                    fc.tokens += c.tokens;
                    fc.chars += c.chars;
                }
                fc
            }
            Group::Files(g) => {
                let mut fc = FileCost::default();
                for file_path in &g.items {
                    let rel = ctx.rel_path(file_path);
                    if !self.cache.has_file(rel) {
                        let hcost = self.cache.header_cost_for(rel);
                        fc.tokens += hcost.tokens;
                        fc.chars += hcost.chars;
                    }
                }
                // Discount: if parent folder is still childless, committing
                // this Files group will remove that placeholder from output.
                let parent_rel = ctx.rel_path(&g.parent_dir);
                if let Some(folder_cost) = self.childless_folders.get(parent_rel) {
                    fc.tokens = fc.tokens.saturating_sub(folder_cost.tokens);
                    fc.chars = fc.chars.saturating_sub(folder_cost.chars);
                }
                fc
            }
        }
    }

    fn commit(&mut self, group: &mut Group<'s>, ctx: &GroupCtx<'s>) -> BudgetDelta {
        match group {
            Group::Ts(g) => {
                let cached = g.cached_render.take().unwrap();
                let cost = cached.marginal_cost;
                self.cache.commit(cached.per_file, cost);
                BudgetDelta::charge(cost)
            }
            Group::Folders(g) => {
                let costs = g.cached_item_costs.take().unwrap();
                let mut total_cost = FileCost::default();
                for (item_dir, cost) in g.items.iter().zip(&costs) {
                    let rel = ctx.rel_path(item_dir);
                    self.childless_folders.insert(rel.to_path_buf(), *cost);
                    total_cost.tokens += cost.tokens;
                    total_cost.chars += cost.chars;
                }
                BudgetDelta::charge(total_cost)
            }
            Group::Files(g) => {
                let mut charge = FileCost::default();
                for file_path in &g.items {
                    let rel = ctx.rel_path(file_path);
                    if !self.cache.has_file(rel) {
                        let hcost = self.cache.header_cost_for(rel);
                        charge.tokens += hcost.tokens;
                        charge.chars += hcost.chars;
                        self.cache.register_file(rel);
                    }
                }

                let parent_rel = ctx.rel_path(&g.parent_dir);
                let refund = self
                    .childless_folders
                    .remove(parent_rel)
                    .unwrap_or_default();
                BudgetDelta { charge, refund }
            }
        }
    }

    fn assemble(&self, char_budget: Option<usize>) -> String {
        let mut output = self.cache.assemble();

        // Append childless folder entries (P2). These are already paid for in
        // the budget, so no token check is needed.
        let mut folder_entries: Vec<_> = self.childless_folders.iter().collect();
        folder_entries.sort_by(|a, b| a.0.cmp(b.0));
        for (folder_path, _cost) in folder_entries {
            let line = format::folder_line(folder_path);
            if output.is_empty() {
                output.push_str(&line);
            } else {
                output.push('\n');
                output.push_str(&line);
            }
        }

        if let Some(cb) = char_budget
            && output.len() > cb
        {
            output.truncate(cb);
            if let Some(pos) = output.rfind('\n') {
                output.truncate(pos + 1);
            }
        }

        output
    }
}

// ---------------------------------------------------------------------------
// Per-file cache
// ---------------------------------------------------------------------------

/// Tracks committed entries per file, keyed by line number for O(1) override.
pub struct FileCache {
    /// Per-file: line → committed RenderedEntry.
    files: HashMap<PathBuf, BTreeMap<u32, CommittedEntry>>,
    /// Per-file header string and cost, computed once on first appearance.
    headers: HashMap<PathBuf, (String, FileCost)>,
    /// Insertion-ordered paths for deterministic final assembly.
    path_order: Vec<PathBuf>,
    /// Running totals across all committed files.
    pub total_tokens: usize,
    pub total_chars: usize,
}

/// An entry committed to a file's line map.
struct CommittedEntry {
    formatted: String,
    tokens: usize,
    chars: usize,
    content_rank: u8,
    content_len: usize,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct FileCost {
    pub tokens: usize,
    pub chars: usize,
}

impl Default for FileCache {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCache {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            headers: HashMap::new(),
            path_order: Vec::new(),
            total_tokens: 0,
            total_chars: 0,
        }
    }

    /// Get or compute the header cost for a path. Tokenized once.
    /// Includes the `\n` separator that precedes each file in the assembled output.
    pub fn header_cost_for(&mut self, path: &Path) -> FileCost {
        if let Some((_, cost)) = self.headers.get(path) {
            return *cost;
        }
        let header_line = format::header_line(path);
        // Include the separator newline in the cost — it's always emitted
        // before a file header in the assembled output (except the first file,
        // but overcounting by 1 token is safer than undercounting).
        let with_sep = format!("\n{header_line}");
        let tokens = format::count_tokens(&with_sep);
        let chars = with_sep.len();
        let cost = FileCost { tokens, chars };
        self.headers.insert(path.to_path_buf(), (header_line, cost));
        cost
    }

    /// Whether a path already has committed entries (or a header reservation).
    pub fn has_file(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    /// Register a file in the path order without any entries (for file headers).
    pub fn register_file(&mut self, path: &Path) {
        if !self.files.contains_key(path) {
            self.files.insert(path.to_path_buf(), BTreeMap::new());
            self.path_order.push(path.to_path_buf());
            let hcost = self.header_cost_for(path);
            self.total_tokens += hcost.tokens;
            self.total_chars += hcost.chars;
        }
    }

    /// Compute the marginal cost of committing a set of rendered entries,
    /// accounting for descendant overrides of already-committed lines.
    /// Does NOT mutate the cache — this is a read-only probe.
    pub fn marginal_cost(&mut self, per_file: &[(PathBuf, Vec<RenderedEntry<'_>>)]) -> FileCost {
        // Pre-compute header costs for new files to avoid borrow conflict
        for (path, _) in per_file {
            if !self.files.contains_key(path) {
                self.header_cost_for(path);
            }
        }

        let mut total = FileCost::default();

        for (path, entries) in per_file {
            let file_map = self.files.get(path);
            let is_new_file = file_map.is_none();

            if is_new_file {
                let hcost = self
                    .headers
                    .get(path)
                    .map_or(FileCost::default(), |(_, c)| *c);
                total.tokens += hcost.tokens;
                total.chars += hcost.chars;
            }

            for re in entries {
                let line = re.entry.line();
                if let Some(map) = file_map
                    && let Some(existing) = map.get(&line)
                {
                    // Override: subtract old cost, add new cost
                    assert!(
                        re.entry.content_rank() > existing.content_rank
                            || (re.entry.content_rank() == existing.content_rank
                                && re.entry.content_rank() == 2
                                && re.entry.content_len() == existing.content_len)
                            || (re.entry.content_rank() == existing.content_rank
                                && re.entry.content_rank() == 1
                                && re.entry.content_len() >= existing.content_len),
                        "R4 violation: descendant entry has less content at line {}",
                        line,
                    );
                    total.tokens += re.tokens;
                    total.tokens = total.tokens.saturating_sub(existing.tokens);
                    total.chars += re.chars;
                    total.chars = total.chars.saturating_sub(existing.chars);
                    continue;
                }
                // New line: full cost
                total.tokens += re.tokens;
                total.chars += re.chars;
            }
        }

        total
    }

    /// Commit rendered entries to the cache. Updates running totals.
    pub fn commit(
        &mut self,
        per_file: Vec<(PathBuf, Vec<RenderedEntry<'_>>)>,
        marginal_cost: FileCost,
    ) {
        for (path, entries) in per_file {
            let map = self.files.entry(path.to_path_buf()).or_insert_with(|| {
                self.path_order.push(path.to_path_buf());
                BTreeMap::new()
            });
            for re in entries {
                let line = re.entry.line();
                map.insert(
                    line,
                    CommittedEntry {
                        formatted: re.formatted,
                        tokens: re.tokens,
                        chars: re.chars,
                        content_rank: re.entry.content_rank(),
                        content_len: re.entry.content_len(),
                    },
                );
            }
        }
        self.total_tokens += marginal_cost.tokens;
        self.total_chars += marginal_cost.chars;
    }

    /// Assemble the final output string from all committed files.
    pub fn assemble(&self) -> String {
        let mut output = String::new();
        for path in &self.path_order {
            let map = match self.files.get(path) {
                Some(m) if !m.is_empty() => m,
                _ => continue,
            };

            if !output.is_empty() {
                output.push('\n');
            }
            let (header_line, _) = self
                .headers
                .get(path)
                .expect("header must be cached before assembly");
            output.push_str(header_line);

            // Entries are in a BTreeMap keyed by line number — already sorted.
            for entry in map.values() {
                output.push_str(&entry.formatted);
            }
        }
        output
    }
}
