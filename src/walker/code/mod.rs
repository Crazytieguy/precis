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

use self::model::{DeclInfo, FileModel, Item, Shape};
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
    pub(crate) fn from_path(path: &Path) -> Option<&'static Language> {
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
    pub tree: Tree,
}

impl SourceFile {
    #[cfg(test)]
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
    /// the next.
    pub(crate) fn comment_rows_above(&self, node: Node) -> Vec<usize> {
        let mut rows = Vec::new();
        let mut next_start = node.start_position().row;
        let mut previous = node.prev_sibling();
        while let Some(comment) = previous {
            if comment.kind() != "comment"
                || next_start.saturating_sub(comment.end_position().row) > 1
                || !self.starts_own_row(comment)
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
    /// order, holding only rows past `after_row` and past every earlier
    /// node. A comment joins the item after it (a leading comment), or
    /// the item before it when it starts on that item's last row (a
    /// trailing comment); comments after the last item form their own.
    pub(crate) fn node_items<'tree>(
        &self,
        nodes: impl IntoIterator<Item = Node<'tree>>,
        after_row: usize,
    ) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        let mut pending = Vec::new();
        let mut claimed_through = after_row;
        for node in nodes {
            let rows = self.node_rows(node);
            let (start, end) = (*rows.start(), *rows.end());
            let new_rows = start.max(claimed_through + 1)..=end;
            let trails_last_item = start == claimed_through && pending.is_empty();
            claimed_through = claimed_through.max(end);
            if new_rows.is_empty() {
                continue;
            }
            if !node.kind().contains("comment") {
                pending.extend(new_rows);
                items.push(Item::new(std::mem::take(&mut pending)));
            } else if trails_last_item && let Some(last) = items.last_mut() {
                last.rows.extend(new_rows);
            } else {
                pending.extend(new_rows);
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

/// Whether `path`'s stem is `dir`'s name, ignoring case and reading `-`
/// as `_`: the file a project names after itself (`lib/express.js` in
/// `express`, `sds.h` in `sds`), conventionally its front door.
fn is_named_after(path: &Path, dir: &Path) -> bool {
    let normalized = |name: &std::ffi::OsStr| {
        name.to_str()
            .map(|name| name.to_ascii_lowercase().replace('-', "_"))
    };
    match (path.file_stem(), dir.file_name()) {
        (Some(stem), Some(dir_name)) => {
            normalized(stem).is_some_and(|stem| Some(stem) == normalized(dir_name))
        }
        _ => false,
    }
}

/// A top-level function of a program's entry file: its index in the
/// file's `decls`, its closing row, and whether it is `main`.
type ProgramFunction = (usize, usize, bool);

/// A program's control flow is what its entry file is about: its
/// `main`, and when `main` is short (at most two top-level statements,
/// however large), the file's other functions. They render with their
/// bodies, as `Whole` declarations closed by their last row.
fn show_program_flow(decls: &mut [DeclInfo], functions: &[ProgramFunction]) {
    let Some(&(main, _, _)) = functions.iter().find(|(_, _, is_main)| *is_main) else {
        return;
    };
    let delegates = decls[main].body.len() <= 2;
    for &(index, closing_row, is_main) in functions {
        if is_main || delegates {
            decls[index].shape = Shape::Whole;
            decls[index].head.push(closing_row);
        }
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
    let grammars: Vec<(&Path, tree_sitter::Language)> = files
        .iter()
        .map(|(path, language)| (path.as_path(), (language.grammar)(path)))
        .collect();
    let parsed = ctx.parse_trees(&grammars);
    let mut out = Vec::new();
    for ((path, language), parsed) in files.into_iter().zip(parsed) {
        let Some((source, tree)) = parsed else {
            continue;
        };
        let file = SourceFile { path, source, tree };
        let model = (language.extract)(&file, ctx);
        out.extend(emit::emit_file(language, &file, model, ctx));
    }
    out
}

#[cfg(test)]
pub(super) mod test_support {
    use std::collections::HashSet;

    use super::model::{DeclInfo, Shape};
    use super::*;

    /// Writes `files` under a fresh root and extracts `target` with
    /// `language`, asserting the [`model`] invariants on the normalized
    /// result.
    pub(crate) fn extract_in(
        language: &Language,
        files: &[(&str, &str)],
        target: &str,
    ) -> (SourceFile, FileModel) {
        let dir = tempfile::tempdir().unwrap();
        for (relative, content) in files {
            let path = dir.path().join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        }
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let file = SourceFile::parse(&dir.path().join(target), language, &ctx).unwrap();
        let model = (language.extract)(&file, &ctx);
        assert_contract(&emit::normalize(model.clone(), &file));
        (file, model)
    }

    pub(crate) fn extract_source(language: &Language, relative: &str, source: &str) -> FileModel {
        extract_in(language, &[(relative, source)], relative).1
    }

    pub(crate) fn rows(items: &[Item]) -> Vec<Vec<usize>> {
        items.iter().map(|item| item.rows.clone()).collect()
    }

    fn part_rows(items: &[Item]) -> HashSet<usize> {
        items
            .iter()
            .flat_map(|item| item.rows.iter().copied())
            .collect()
    }

    /// The [`model`] invariants: non-empty head and name rows, name rows
    /// inside the rendered decl, disjoint parts, one level of members
    /// sharing only their name rows with the container, and no row in
    /// two file-level owners.
    fn assert_contract(model: &FileModel) {
        let mut owned = part_rows(&model.module_doc);
        for row in part_rows(&model.reexports) {
            assert!(owned.insert(row), "re-export row {row} owned twice");
        }
        let check_decl = |decl: &DeclInfo| {
            assert!(!decl.name_rows.is_empty() && !decl.head.is_empty());
            let head: HashSet<usize> = decl.head.iter().copied().collect();
            let doc = part_rows(&decl.doc);
            let body = part_rows(&decl.body);
            assert!(head.is_disjoint(&doc) && head.is_disjoint(&body) && doc.is_disjoint(&body));
            let rendered: HashSet<usize> = match decl.shape {
                Shape::Callable => head.clone(),
                Shape::Whole => head.union(&body).copied().collect(),
            };
            assert!(decl.name_rows.iter().all(|row| rendered.contains(row)));
            head.union(&doc)
                .chain(body.iter())
                .copied()
                .collect::<HashSet<_>>()
        };
        for decl in &model.decls {
            let container_rows = check_decl(decl);
            let mut decl_rows = container_rows.clone();
            for member in &decl.members {
                assert!(member.members.is_empty());
                let member_rows = check_decl(member);
                let shared: Vec<_> = member_rows.intersection(&container_rows).collect();
                assert!(
                    shared.iter().all(|row| member.name_rows.contains(row)),
                    "member rows {shared:?} also in its container"
                );
                decl_rows.extend(member_rows);
            }
            for row in decl_rows {
                assert!(owned.insert(row), "declaration row {row} owned twice");
            }
        }
    }
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
