use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::batch::{Batch, BatchContent, BatchId, FsEntry, RenderedLine};
use crate::tokenizer;

const INDENT_UNIT: &str = "    ";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cost {
    pub tokens: usize,
    pub chars: usize,
}

#[derive(Debug, Clone)]
struct LineRecord {
    line: RenderedLine,
    owner: BatchId,
}

#[derive(Debug, Default)]
struct TreeNode {
    /// For dirs, child names that have been listed by some scheduled FolderListing.
    listed_children: BTreeMap<String, FsEntry>,
    /// For files, line content keyed by source line number, with the batch id
    /// that contributed the current rendering (so the scheduler can detect
    /// non-predecessor overlap).
    content: BTreeMap<usize, LineRecord>,
}

#[derive(Debug)]
pub struct RenderedTree {
    root: PathBuf,
    nodes: BTreeMap<PathBuf, TreeNode>,
}

impl RenderedTree {
    pub fn new(root: PathBuf) -> Self {
        let mut nodes = BTreeMap::new();
        nodes.insert(root.clone(), TreeNode::default());
        Self { root, nodes }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Compute the marginal cost (in tokens and chars) of applying this batch
    /// against the current tree state. Does not mutate.
    pub fn marginal_cost(&self, batch: &Batch) -> Cost {
        match &batch.content {
            BatchContent::FolderListing { path, entries } => self.cost_folder(path, entries),
            BatchContent::FileContent { path, lines } => self.cost_file(path, lines),
        }
    }

    /// Apply a batch's content to the tree.
    ///
    /// `is_ancestor(id)` should return `true` iff `id` is in the batch's
    /// (transitive) ancestor chain. The scheduler computes this from its
    /// batch graph; the tree uses it to detect non-predecessor overlap on
    /// file content. Mismatches panic in release builds.
    pub fn apply(&mut self, batch: &Batch, is_ancestor: impl Fn(BatchId) -> bool) {
        match &batch.content {
            BatchContent::FolderListing { path, entries } => self.apply_folder(path, entries),
            BatchContent::FileContent { path, lines } => {
                self.apply_file(path, lines, batch.id, &is_ancestor)
            }
        }
    }

    /// Render the tree to its final text representation.
    pub fn render(&self) -> String {
        let mut out = String::new();
        self.render_children(&self.root, 0, &mut out);
        out
    }

    /// Total tokens of the rendered tree (recomputed from current state). Used
    /// for invariant checking, not as the hot-path budget tracker.
    pub fn total_tokens(&self) -> usize {
        tokenizer::count(&self.render())
    }

    /// Total chars of the rendered tree.
    pub fn total_chars(&self) -> usize {
        self.render().len()
    }

    // ---- internal ----

    fn depth_from_root(&self, path: &Path) -> usize {
        path.strip_prefix(&self.root)
            .map(|p| p.components().count())
            .unwrap_or(0)
    }

    fn cost_folder(&self, path: &Path, entries: &[FsEntry]) -> Cost {
        let indent_depth = self.depth_from_root(path);
        let existing = self.nodes.get(path).map(|n| &n.listed_children);
        let mut cost = Cost::default();
        for entry in entries {
            let already = existing.is_some_and(|e| e.contains_key(&entry.name));
            if already {
                continue;
            }
            let row = format_entry_row(entry, indent_depth);
            cost.tokens += tokenizer::count(&row);
            cost.chars += row.len();
        }
        cost
    }

    fn cost_file(&self, path: &Path, lines: &[RenderedLine]) -> Cost {
        let indent_depth = self.depth_from_root(path);
        let existing = self.nodes.get(path).map(|n| &n.content);
        let mut cost = Cost::default();
        for line in lines {
            let new_row = format_line_row(line, indent_depth);
            let new_tokens = tokenizer::count(&new_row);
            let new_chars = new_row.len();
            let old = existing.and_then(|c| c.get(&line.number));
            if let Some(old) = old {
                let old_row = format_line_row(&old.line, indent_depth);
                cost.tokens += new_tokens.saturating_sub(tokenizer::count(&old_row));
                cost.chars += new_chars.saturating_sub(old_row.len());
            } else {
                cost.tokens += new_tokens;
                cost.chars += new_chars;
            }
        }
        cost
    }

    fn apply_folder(&mut self, path: &Path, entries: &[FsEntry]) {
        for entry in entries {
            self.nodes.entry(path.join(&entry.name)).or_default();
        }
        let node = self.nodes.entry(path.to_path_buf()).or_default();
        for entry in entries {
            node.listed_children
                .entry(entry.name.clone())
                .or_insert_with(|| entry.clone());
        }
    }

    fn apply_file(
        &mut self,
        path: &Path,
        lines: &[RenderedLine],
        owner: BatchId,
        is_ancestor: &impl Fn(BatchId) -> bool,
    ) {
        let node = self.nodes.entry(path.to_path_buf()).or_default();
        for line in lines {
            if let Some(existing) = node.content.get(&line.number) {
                assert!(
                    is_ancestor(existing.owner),
                    "non-predecessor overlap: batch {:?} would replace line {} of {} owned by non-ancestor batch {:?}",
                    owner,
                    line.number,
                    path.display(),
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

fn format_entry_row(entry: &FsEntry, indent_depth: usize) -> String {
    let mut s = INDENT_UNIT.repeat(indent_depth);
    s.push_str(&entry.name);
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
