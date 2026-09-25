//! The shared code engine: one declaration ladder for every source
//! language, fed by a thin extraction module per language. Each
//! `code/<lang>.rs` exports one [`Language`] (plus, if it keeps per-run
//! caches, a `RunState` reachable as `ctx.code.<lang>`); the engine
//! (`emit`, `chunk`, `ledger`) owns keys, predecessors, chunking, row
//! ownership and value, and a language module never builds a batch.

mod c;
mod chunk;
mod emit;
mod go;
mod ledger;
mod lua;
pub(crate) mod model;
mod python;
mod rust;
mod typescript;

use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

use self::model::{FileModel, Item};
use super::fs::files_with_any_extension;
use super::{WalkCtx, node_end_row_trimmed};
use crate::batch::Batch;
use crate::render::Source;

/// One source language: what its extraction module gives the engine.
pub(crate) struct Language {
    /// File extensions (matched case-insensitively) the language claims.
    /// No two languages share one.
    extensions: &'static [&'static str],
    /// The grammar that parses a path.
    grammar: fn(&Path) -> tree_sitter::Language,
    /// The file's declarations, split into parts per the contract in
    /// [`model`].
    extract: fn(&SourceFile, &WalkCtx) -> FileModel,
    /// The language's entry files (`lib.rs`, `__init__.py`, …), which the
    /// engine prices as depth ≤ 1.
    is_entrypoint: Option<fn(&Path, &WalkCtx) -> bool>,
    /// A file-role multiplier on every batch of the file.
    file_weight: Option<fn(&Path, &WalkCtx) -> f64>,
}

/// The languages the engine can walk: a closed set.
const LANGUAGES: [&Language; 6] = [
    &rust::LANGUAGE,
    &typescript::LANGUAGE,
    &python::LANGUAGE,
    &go::LANGUAGE,
    &c::LANGUAGE,
    &lua::LANGUAGE,
];

impl Language {
    /// The language whose extensions include `path`'s extension.
    fn from_path(path: &Path) -> Option<&'static Language> {
        let extension = path.extension()?.to_str()?;
        LANGUAGES.into_iter().find(|language| {
            language
                .extensions
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        })
    }
}

/// Per-run state of the language modules that keep any.
#[derive(Default)]
pub(crate) struct CodeState {
    typescript: typescript::RunState,
}

/// One parsed source file, built once and shared by extraction and
/// emission.
pub(crate) struct SourceFile {
    pub path: PathBuf,
    pub source: Arc<Source>,
    pub tree: Arc<Tree>,
}

impl SourceFile {
    fn parse(path: &Path, language: &Language, ctx: &WalkCtx) -> Option<Self> {
        let (source, tree) = ctx.parse_tree(path, &(language.grammar)(path))?;
        Some(Self {
            path: path.to_path_buf(),
            source,
            tree,
        })
    }

    /// Text of 1-based `row` without its line terminator; empty past the
    /// end of the file.
    pub(crate) fn line(&self, row: usize) -> &str {
        self.source.line(row).unwrap_or("")
    }

    pub(crate) fn line_count(&self) -> usize {
        self.source.line_count()
    }

    pub(crate) fn text(&self, node: Node) -> &str {
        &self.source[node.byte_range()]
    }

    /// 1-based rows `node` covers, without a final row it reaches only
    /// with whitespace.
    pub(crate) fn node_rows(&self, node: Node) -> RangeInclusive<usize> {
        node.start_position().row + 1..=node_end_row_trimmed(node, &self.source) + 1
    }

    /// Whether `node` is the first token on its row.
    pub(crate) fn starts_own_row(&self, node: Node) -> bool {
        let position = node.start_position();
        self.line(position.row + 1)
            .get(..position.column)
            .is_some_and(|before| before.trim().is_empty())
    }

    /// Rows of the comments directly above `node`: a run of `comment`
    /// siblings, each on its own rows and ending at most one row above
    /// the next, stopping at the first one that starts at or before
    /// `after_row`.
    pub(crate) fn comment_rows_above(&self, node: Node, after_row: usize) -> Vec<usize> {
        let mut rows = Vec::new();
        let mut next_start = node.start_position().row;
        let mut previous = node.prev_sibling();
        while let Some(comment) = previous {
            if comment.kind() != "comment"
                || next_start.saturating_sub(comment.end_position().row) > 1
                || !self.starts_own_row(comment)
                || comment.start_position().row < after_row
            {
                break;
            }
            rows.extend(self.node_rows(comment));
            next_start = comment.start_position().row;
            previous = comment.prev_sibling();
        }
        rows.sort_unstable();
        rows
    }

    /// One [`Item`] per node of `nodes` (statements, fields, specs), in
    /// order, keeping only rows past `after_row`. A comment joins the
    /// item after it (a leading comment), or the item before it when it
    /// starts on that item's last row (a trailing comment); comments
    /// after the last item form their own.
    pub(crate) fn node_items<'tree>(
        &self,
        nodes: impl IntoIterator<Item = Node<'tree>>,
        after_row: usize,
    ) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        let mut pending = Vec::new();
        for node in nodes {
            let rows: Vec<usize> = self
                .node_rows(node)
                .filter(|&row| row > after_row)
                .collect();
            let Some(&first) = rows.first() else {
                continue;
            };
            if !node.kind().contains("comment") {
                pending.extend(rows);
                items.push(Item::new(std::mem::take(&mut pending)));
            } else if pending.is_empty()
                && let Some(last) = items.last_mut()
                && last.rows.last() == Some(&first)
            {
                last.rows.extend(&rows[1..]);
            } else {
                pending.extend(rows);
            }
        }
        if !pending.is_empty() {
            items.push(Item::new(pending));
        }
        items
    }

    /// `rows` split into one [`Item`] per paragraph: runs of non-blank
    /// rows, broken at blank rows and at gaps.
    pub(crate) fn paragraphs(&self, rows: impl IntoIterator<Item = usize>) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        let mut previous = None;
        for row in rows {
            if self.line(row).trim().is_empty() {
                previous = None;
                continue;
            }
            match items.last_mut() {
                Some(item) if previous == Some(row - 1) => item.rows.push(row),
                _ => items.push(Item::new([row])),
            }
            previous = Some(row);
        }
        items
    }
}

/// Batches for every source file in `dir`. Called by `FsWalker` once per
/// scheduled directory listing.
pub(crate) fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let extensions: Vec<&str> = LANGUAGES
        .into_iter()
        .flat_map(|language| language.extensions.iter().copied())
        .collect();
    let files: Vec<(PathBuf, &Language)> = files_with_any_extension(dir, &extensions, ctx)
        .into_iter()
        .filter_map(|path| Language::from_path(&path).map(|language| (path, language)))
        .collect();
    ctx.parse_trees(
        files
            .iter()
            .map(|(path, language)| (path.as_path(), (language.grammar)(path))),
    );
    let mut out = Vec::new();
    for (path, language) in files {
        let Some(file) = SourceFile::parse(&path, language, ctx) else {
            continue;
        };
        let model = (language.extract)(&file, ctx);
        out.extend(emit::emit_file(language, &file, model, ctx));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_mod_language_from_path_matches_extensions_case_insensitively() {
        let claimed =
            |path: &str| Language::from_path(Path::new(path)).map(|language| language.extensions);
        assert_eq!(claimed("x.h"), Some(c::LANGUAGE.extensions));
        assert_eq!(
            claimed("web/App.JSX"),
            Some(typescript::LANGUAGE.extensions)
        );
        assert_eq!(claimed("x.cpp"), None);
    }
}
