use std::collections::{BTreeMap, HashMap};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchContent, BatchId, EntryKind, RenderedLine};
use crate::tokenizer;

const INDENT_UNIT: &str = "    ";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cost {
    pub tokens: usize,
    pub bytes: usize,
}

#[derive(Debug, Clone)]
struct LineRecord {
    line: RenderedLine,
    owner: BatchId,
}

/// A node in the rendered tree is either a directory (with named children)
/// or a file (with line content). The enum makes "file with children" or
/// "directory with line content" unconstructible.
#[derive(Debug)]
enum TreeNode {
    Dir {
        children: BTreeMap<OsString, EntryKind>,
    },
    File {
        content: BTreeMap<usize, LineRecord>,
    },
}

impl TreeNode {
    fn empty_dir() -> Self {
        Self::Dir {
            children: BTreeMap::new(),
        }
    }

    fn empty_file() -> Self {
        Self::File {
            content: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct RenderedTree {
    root: PathBuf,
    nodes: HashMap<PathBuf, TreeNode>,
}

impl RenderedTree {
    pub fn new(root: PathBuf) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(root.clone(), TreeNode::empty_dir());
        Self { root, nodes }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Compute the marginal cost of applying this batch against the current
    /// state (without mutating).
    pub fn marginal_cost(&self, batch: &Batch) -> Cost {
        match &batch.content {
            BatchContent::FileSystemEntries { parent, children } => {
                self.cost_entries(parent, children)
            }
            BatchContent::Lines(file_map) => self.cost_lines(file_map),
        }
    }

    /// Apply a batch's content. `owner` is the batch's id (the scheduler's
    /// loop index); `is_ancestor(id)` returns true iff `id` is in the
    /// batch's transitive predecessor chain — used to detect non-ancestor
    /// overrides of file lines.
    pub fn apply(&mut self, batch: &Batch, owner: BatchId, is_ancestor: impl Fn(BatchId) -> bool) {
        match &batch.content {
            BatchContent::FileSystemEntries { parent, children } => {
                self.apply_entries(parent, children);
            }
            BatchContent::Lines(file_map) => {
                self.apply_lines(file_map, owner, &is_ancestor);
            }
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        self.render_dir(&self.root, 0, &mut out);
        out
    }

    /// Total tokens of the rendered tree (recomputed). Used for end-of-run
    /// cross-checks; not the hot-path budget tracker.
    pub fn total_tokens(&self) -> usize {
        tokenizer::count(&self.render())
    }

    /// Total bytes of the rendered tree.
    pub fn total_bytes(&self) -> usize {
        self.render().len()
    }

    // ---- internal ----

    fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    fn cost_entries(&self, parent: &Path, children: &BTreeMap<OsString, EntryKind>) -> Cost {
        let indent_depth = self.depth_from_root(parent);
        let already_listed = match self.nodes.get(parent) {
            Some(TreeNode::Dir { children }) => Some(children),
            _ => None,
        };
        let mut cost = Cost::default();
        for (name, kind) in children {
            if already_listed.is_some_and(|c| c.contains_key(name)) {
                continue;
            }
            let row = format_entry_row(name, *kind, indent_depth);
            cost.tokens += tokenizer::count(&row);
            cost.bytes += row.len();
        }
        cost
    }

    fn cost_lines(&self, file_map: &BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>>) -> Cost {
        // Conservative ellipsis-overhead headroom per file to keep the
        // end-of-run cross-check happy when this batch introduces new gaps
        // (rendered as bare-ellipsis lines). ~2 markers × ~3 tokens each;
        // slight over-estimate trades a few tokens of unused budget for the
        // invariant that consumed >= rendered.
        const ELLIPSIS_OVERHEAD_TOKENS: usize = 8;
        const ELLIPSIS_OVERHEAD_BYTES: usize = 16;

        let mut cost = Cost::default();
        for (path, lines) in file_map {
            let indent_depth = self.depth_from_root(path);
            let existing = match self.nodes.get(path) {
                Some(TreeNode::File { content }) => Some(content),
                _ => None,
            };
            cost.tokens += ELLIPSIS_OVERHEAD_TOKENS;
            cost.bytes += ELLIPSIS_OVERHEAD_BYTES;
            for (number, line) in lines {
                let new_row = format_line_row(*number, line, indent_depth);
                let new_tokens = tokenizer::count(&new_row);
                let new_bytes = new_row.len();
                if let Some(existing) = existing
                    && let Some(old) = existing.get(number)
                {
                    let old_row = format_line_row(*number, &old.line, indent_depth);
                    cost.tokens += new_tokens.saturating_sub(tokenizer::count(&old_row));
                    cost.bytes += new_bytes.saturating_sub(old_row.len());
                } else {
                    cost.tokens += new_tokens;
                    cost.bytes += new_bytes;
                }
            }
        }
        cost
    }

    fn apply_entries(&mut self, parent: &Path, children: &BTreeMap<OsString, EntryKind>) {
        // Pre-create child nodes so future apply()s can target them.
        for (name, kind) in children {
            let child_path = parent.join(name);
            self.nodes.entry(child_path).or_insert_with(|| match kind {
                EntryKind::Directory => TreeNode::empty_dir(),
                EntryKind::File => TreeNode::empty_file(),
            });
        }
        let parent_node = self
            .nodes
            .entry(parent.to_path_buf())
            .or_insert_with(TreeNode::empty_dir);
        let TreeNode::Dir {
            children: parent_children,
        } = parent_node
        else {
            debug_assert!(
                false,
                "FileSystemEntries parent {} is a File node",
                parent.display()
            );
            return;
        };
        for (name, kind) in children {
            parent_children.entry(name.clone()).or_insert(*kind);
        }
    }

    fn apply_lines(
        &mut self,
        file_map: &BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>>,
        owner: BatchId,
        is_ancestor: &impl Fn(BatchId) -> bool,
    ) {
        for (path, lines) in file_map {
            // The file path must already be declared as a File node by a
            // prior FileSystemEntries batch. Orphaned content would render
            // invisibly but consume budget.
            let node = self.nodes.get_mut(path);
            debug_assert!(
                matches!(node, Some(TreeNode::File { .. })),
                "Lines target {} not declared as a File node",
                path.display()
            );
            let Some(TreeNode::File { content }) = node else {
                continue;
            };
            for (number, line) in lines {
                debug_assert!(
                    !line.text().is_empty(),
                    "RenderedLine with empty text for {}:{}",
                    path.display(),
                    number
                );
                if let Some(existing) = content.get(number) {
                    debug_assert!(
                        is_ancestor(existing.owner),
                        "non-ancestor overlap: batch {:?} would replace line {} of {} owned by non-ancestor batch {:?}",
                        owner,
                        number,
                        path.display(),
                        existing.owner,
                    );
                }
                content.insert(
                    *number,
                    LineRecord {
                        line: line.clone(),
                        owner,
                    },
                );
            }
        }
    }

    fn render_dir(&self, path: &Path, indent_depth: usize, out: &mut String) {
        let Some(TreeNode::Dir { children }) = self.nodes.get(path) else {
            return;
        };
        let indent = INDENT_UNIT.repeat(indent_depth);
        for (name, kind) in children {
            out.push_str(&indent);
            out.push_str(&name.to_string_lossy());
            match kind {
                EntryKind::Directory => {
                    out.push_str("/\n");
                    self.render_dir(&path.join(name), indent_depth + 1, out);
                }
                EntryKind::File => {
                    out.push('\n');
                    self.render_file(&path.join(name), indent_depth + 1, out);
                }
            }
        }
    }

    fn render_file(&self, path: &Path, indent_depth: usize, out: &mut String) {
        let Some(TreeNode::File { content }) = self.nodes.get(path) else {
            return;
        };
        let indent = INDENT_UNIT.repeat(indent_depth);
        // Track the previous rendered line number so we can insert a bare-
        // ellipsis marker on any gap. Gaps represent content the walkers chose
        // not to surface at this budget; the marker invites the agent to fetch
        // it via a Read. A gap at the start of the file (first rendered line
        // isn't 1) also gets a marker, since content before the first shown
        // line is likewise elided.
        let mut prev: Option<usize> = None;
        for (number, record) in content {
            let expected_next = prev.map_or(1, |p| p + 1);
            if *number > expected_next {
                out.push_str(&indent);
                out.push('…');
                out.push('\n');
            }
            out.push_str(&indent);
            out.push_str(&number.to_string());
            out.push('→');
            out.push_str(record.line.text());
            if record.line.is_truncated() {
                out.push('…');
            }
            out.push('\n');
            prev = Some(*number);
        }
    }
}

fn format_entry_row(name: &OsString, kind: EntryKind, indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push_str(&name.to_string_lossy());
    if matches!(kind, EntryKind::Directory) {
        s.push('/');
    }
    s.push('\n');
    s
}

fn format_line_row(number: usize, line: &RenderedLine, indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push_str(&number.to_string());
    s.push('→');
    s.push_str(line.text());
    if line.is_truncated() {
        s.push('…');
    }
    s.push('\n');
    s
}
