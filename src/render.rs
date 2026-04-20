use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchContent, BatchId, FsEntry, RenderedLine};
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

#[derive(Debug, Default)]
struct TreeNode {
    /// For dirs: name → entry that some scheduled FileSystemEntries batch added.
    listed_children: BTreeMap<String, FsEntry>,
    /// For files: line content keyed by source line number, with the batch
    /// that contributed the current rendering.
    content: BTreeMap<usize, LineRecord>,
}

#[derive(Debug)]
pub struct RenderedTree {
    root: PathBuf,
    nodes: HashMap<PathBuf, TreeNode>,
}

impl RenderedTree {
    pub fn new(root: PathBuf) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(root.clone(), TreeNode::default());
        Self { root, nodes }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Compute the marginal cost of applying this batch against the current
    /// state (without mutating).
    pub fn marginal_cost(&self, batch: &Batch) -> Cost {
        match &batch.content {
            BatchContent::FileSystemEntries(entries) => self.cost_entries(entries),
            BatchContent::Lines(sets) => self.cost_lines(sets),
        }
    }

    /// Apply a batch's content. `is_ancestor(id)` returns true iff `id` is in
    /// the batch's transitive predecessor chain — used to detect non-ancestor
    /// override of file lines (debug-assert).
    pub fn apply(&mut self, batch: &Batch, is_ancestor: impl Fn(BatchId) -> bool) {
        match &batch.content {
            BatchContent::FileSystemEntries(entries) => self.apply_entries(entries),
            BatchContent::Lines(sets) => self.apply_lines(sets, batch.id, &is_ancestor),
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        self.render_children(&self.root, 0, &mut out);
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

    fn cost_entries(&self, entries: &[FsEntry]) -> Cost {
        let mut cost = Cost::default();
        for entry in entries {
            let parent = entry.path.parent().unwrap_or(&self.root);
            let indent_depth = self.depth_from_root(parent);
            let already = self
                .nodes
                .get(parent)
                .is_some_and(|n| n.listed_children.contains_key(basename(&entry.path)));
            if already {
                continue;
            }
            let row = format_entry_row(entry, indent_depth);
            cost.tokens += tokenizer::count(&row);
            cost.bytes += row.len();
        }
        cost
    }

    fn cost_lines(&self, sets: &[crate::batch::FileLineSet]) -> Cost {
        let mut cost = Cost::default();
        for set in sets {
            let indent_depth = self.depth_from_root(&set.path);
            let existing = self.nodes.get(&set.path).map(|n| &n.content);
            for line in &set.lines {
                let new_row = format_line_row(line, indent_depth);
                let new_tokens = tokenizer::count(&new_row);
                let new_bytes = new_row.len();
                if let Some(existing) = existing
                    && let Some(old) = existing.get(&line.number)
                {
                    let old_row = format_line_row(&old.line, indent_depth);
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

    fn apply_entries(&mut self, entries: &[FsEntry]) {
        for entry in entries {
            self.nodes.entry(entry.path.clone()).or_default();
        }
        // Group entries by parent so each parent's TreeNode is fetched once.
        let mut by_parent: HashMap<PathBuf, Vec<&FsEntry>> = HashMap::new();
        for entry in entries {
            let parent = entry.path.parent().unwrap_or(&self.root).to_path_buf();
            by_parent.entry(parent).or_default().push(entry);
        }
        for (parent_path, entries) in by_parent {
            let node = self.nodes.entry(parent_path).or_default();
            for entry in entries {
                node.listed_children
                    .entry(basename(&entry.path).to_string())
                    .or_insert_with(|| entry.clone());
            }
        }
    }

    fn apply_lines(
        &mut self,
        sets: &[crate::batch::FileLineSet],
        owner: BatchId,
        is_ancestor: &impl Fn(BatchId) -> bool,
    ) {
        for set in sets {
            let node = self.nodes.entry(set.path.clone()).or_default();
            for line in &set.lines {
                if let Some(existing) = node.content.get(&line.number) {
                    debug_assert!(
                        is_ancestor(existing.owner),
                        "non-ancestor overlap: batch {:?} would replace line {} of {} owned by non-ancestor batch {:?}",
                        owner,
                        line.number,
                        set.path.display(),
                        existing.owner,
                    );
                }
                node.content.insert(
                    line.number,
                    LineRecord {
                        line: line.clone(),
                        owner,
                    },
                );
            }
        }
    }

    fn render_children(&self, parent: &Path, indent_depth: usize, out: &mut String) {
        let indent = INDENT_UNIT.repeat(indent_depth);
        let Some(parent_node) = self.nodes.get(parent) else {
            return;
        };
        for (name, entry) in &parent_node.listed_children {
            let child_path = parent.join(name);
            out.push_str(&indent);
            out.push_str(name);
            if entry.is_dir {
                out.push_str("/\n");
                self.render_children(&child_path, indent_depth + 1, out);
            } else {
                out.push('\n');
                self.render_file_content(&child_path, indent_depth + 1, out);
            }
        }
    }

    fn render_file_content(&self, path: &Path, indent_depth: usize, out: &mut String) {
        let Some(node) = self.nodes.get(path) else {
            return;
        };
        let indent = INDENT_UNIT.repeat(indent_depth);
        for (num, record) in &node.content {
            out.push_str(&indent);
            out.push_str(&num.to_string());
            out.push('→');
            out.push_str(&record.line.text);
            if record.line.truncated {
                out.push('…');
            }
            out.push('\n');
        }
    }
}

fn basename(path: &Path) -> &str {
    path.file_name().and_then(|n| n.to_str()).unwrap_or("")
}

fn format_entry_row(entry: &FsEntry, indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push_str(basename(&entry.path));
    if entry.is_dir {
        s.push('/');
    }
    s.push('\n');
    s
}

fn format_line_row(line: &RenderedLine, indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push_str(&line.number.to_string());
    s.push('→');
    s.push_str(&line.text);
    if line.truncated {
        s.push('…');
    }
    s.push('\n');
    s
}
