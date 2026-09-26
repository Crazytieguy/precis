//! The shared code engine: one declaration ladder for every source
//! language, fed by a thin extraction module per language. Each
//! `code/<lang>.rs` exports one [`Language`] (plus, if it keeps per-run
//! caches, a `RunState` reachable as `ctx.code.<lang>`); the engine
//! (`emit`, `chunk`, `ledger`) owns keys, predecessors, chunking, row
//! ownership and value, and a language module never builds a batch.

mod c;
pub(in crate::walker) mod chunk;
mod emit;
mod go;
mod ledger;
mod lua;
pub(crate) mod model;
mod python;
mod rust;
mod typescript;

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tree_sitter::{Node, Tree};

pub(in crate::walker) use self::c::is_cpp_header;
use self::model::{DeclInfo, FileModel, Item, Shape};
use super::fs::files_with_any_extension;
use super::{WalkCtx, node_end_row_trimmed};
use crate::batch::{Batch, BatchKey, Rung};
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
    /// The names the file uses for the sibling files it depends on,
    /// matched against each sibling's [`Self::sibling_names`]. `None`
    /// orders the language's roster chain by size alone.
    sibling_mentions: Option<fn(&SourceFile) -> HashSet<String>>,
    /// The names siblings use for the file; `None` for its file stem.
    sibling_names: Option<fn(&SourceFile) -> Vec<String>>,
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
    pub(crate) fn is_entrypoint(&self, path: &Path, ctx: &WalkCtx) -> bool {
        self.is_entrypoint.is_some_and(|test| test(path, ctx))
    }

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

/// Every extension some [`Language`] claims.
pub(crate) fn parsed_extensions() -> impl Iterator<Item = &'static str> {
    LANGUAGES
        .into_iter()
        .flat_map(|language| language.extensions.iter().copied())
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

    /// The rows of each comment directly above `node`, in source order: a
    /// run of `comment` siblings that `accept` admits, each on its own
    /// rows and ending 1 to `max_gap` rows above the next.
    pub(crate) fn comments_above(
        &self,
        node: Node,
        max_gap: usize,
        accept: impl Fn(Node) -> bool,
    ) -> Vec<RangeInclusive<usize>> {
        let mut comments = Vec::new();
        let mut next_start = *self.node_rows(node).start();
        let mut previous = node.prev_sibling();
        while let Some(comment) = previous {
            let rows = self.node_rows(comment);
            let gap = next_start.saturating_sub(*rows.end());
            if comment.kind() != "comment"
                || !(1..=max_gap).contains(&gap)
                || !self.starts_own_row(comment)
                || !accept(comment)
            {
                break;
            }
            next_start = *rows.start();
            comments.push(rows);
            previous = comment.prev_sibling();
        }
        comments.reverse();
        comments
    }

    /// The comments directly above `node`, one [`Item`] per paragraph.
    pub(crate) fn comment_paragraphs_above(&self, node: Node) -> Vec<Item> {
        self.paragraphs(self.comments_above(node, 1, |_| true).into_iter().flatten())
    }

    /// One [`Item`] per node of `nodes` (statements, fields, specs), in
    /// order, holding only rows past `after_row` and past every earlier
    /// node. A comment or Rust attribute joins the item after it (leading),
    /// or the item before it when it starts on that item's last row
    /// (trailing); ones after the last item form their own.
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
            if !(node.kind().contains("comment") || node.kind() == "attribute_item") {
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

fn file_stem(path: &Path) -> Option<&str> {
    path.file_stem()?.to_str()
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

/// A top-level function of a program's entry file.
struct ProgramFunction {
    /// Its index in the file's `decls`.
    index: usize,
    closing_row: usize,
    is_main: bool,
}

impl ProgramFunction {
    /// The function declared by `node`, extracted as `decls[index]`.
    fn new(index: usize, node: Node, file: &SourceFile) -> Self {
        Self {
            index,
            closing_row: *file.node_rows(node).end(),
            is_main: node
                .child_by_field_name("name")
                .is_some_and(|name| file.text(name) == "main"),
        }
    }
}

/// A program's control flow is what its entry file is about: its
/// `main`, and when `main` is short (at most two top-level statements,
/// however large), the file's other functions. They render with their
/// bodies, as `Whole` declarations closed by their last row.
fn show_program_flow(decls: &mut [DeclInfo], functions: &[ProgramFunction]) {
    let Some(main) = functions.iter().find(|function| function.is_main) else {
        return;
    };
    let delegates = decls[main.index].body.len() <= 2;
    for function in functions {
        if function.is_main || delegates {
            let decl = &mut decls[function.index];
            decl.shape = Shape::Whole;
            decl.head.push(function.closing_row);
        }
    }
}

/// Batches for every source file in `dir`. Called by `FsWalker` once per
/// scheduled directory listing.
pub(crate) fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let extensions: Vec<&str> = parsed_extensions().collect();
    let files: Vec<(PathBuf, &Language)> = files_with_any_extension(dir, &extensions, ctx)
        .into_iter()
        .filter_map(|path| Language::from_path(&path).map(|language| (path, language)))
        .collect();
    let grammars: Vec<(&Path, tree_sitter::Language)> = files
        .iter()
        .map(|(path, language)| (path.as_path(), (language.grammar)(path)))
        .collect();
    let mut emitted = Vec::new();
    ctx.parse_each(&grammars, |index, source, tree| {
        let (path, language) = &files[index];
        let bytes = source.len();
        let file = SourceFile {
            path: path.clone(),
            source,
            tree,
        };
        let model = (language.extract)(&file, ctx);
        let names = match language.sibling_names {
            Some(names) => names(&file),
            None => file_stem(path).into_iter().map(str::to_owned).collect(),
        };
        emitted.push(EmittedFile {
            batches: emit::emit_file(language, &file, model, ctx),
            chained: !language.is_entrypoint(path, ctx),
            non_essential: ctx.non_essential_factor(path) < 1.0,
            bytes,
            names,
            mentions: language
                .sibling_mentions
                .map_or_else(HashSet::new, |mentions| mentions(&file)),
        });
    });
    chain_rosters(&mut emitted);
    emitted.into_iter().flat_map(|file| file.batches).collect()
}

/// One file's batches, and what places its roster on its directory's
/// roster chain.
struct EmittedFile {
    batches: Vec<Batch>,
    /// Entry files stay off the chain: their rosters are neither gated nor
    /// a gate.
    chained: bool,
    non_essential: bool,
    bytes: usize,
    /// See [`Language::sibling_names`].
    names: Vec<String>,
    /// See [`Language::sibling_mentions`].
    mentions: HashSet<String>,
}

/// Gates each chained file's `Names` head chunk on the previous chained
/// file's, most central file first and non-essential files last. Every
/// roster of a directory is priced alike by row length, so without the
/// chain the files with the shortest rows open first, whatever their role.
/// A file is central when many siblings depend on it (a base type the
/// others are written against) or it depends on many (the one composing
/// them); ties, and languages that don't say, fall back to size.
fn chain_rosters(files: &mut [EmittedFile]) {
    let chain: Vec<&mut EmittedFile> = files.iter_mut().filter(|file| file.chained).collect();
    let mut declarers: HashMap<&str, Vec<usize>> = HashMap::new();
    for (index, file) in chain.iter().enumerate() {
        for name in &file.names {
            let files = declarers.entry(name.as_str()).or_default();
            if files.last() != Some(&index) {
                files.push(index);
            }
        }
    }
    let mut degrees = vec![0; chain.len()];
    for (dependent, file) in chain.iter().enumerate() {
        if file.mentions.is_empty() {
            continue;
        }
        let own: HashSet<&str> = file.names.iter().map(String::as_str).collect();
        let dependencies: HashSet<usize> = file
            .mentions
            .iter()
            .filter(|name| !own.contains(name.as_str()))
            .filter_map(|name| declarers.get(name.as_str()))
            .flatten()
            .copied()
            .filter(|&dependency| dependency != dependent)
            .collect();
        degrees[dependent] += dependencies.len();
        for dependency in dependencies {
            degrees[dependency] += 1;
        }
    }
    let mut ranked: Vec<(usize, &mut EmittedFile)> = degrees.into_iter().zip(chain).collect();
    ranked.sort_by_key(|(degree, file)| (file.non_essential, Reverse((*degree, file.bytes))));
    let mut gate: Option<BatchKey> = None;
    for (_, file) in ranked {
        let Some(head) = file.batches.iter_mut().find(|batch| {
            matches!(&batch.key, BatchKey::Code(key) if key.rung == Rung::Names && key.sub == 0)
        }) else {
            continue;
        };
        head.predecessor = gate.replace(head.key.clone());
    }
}

#[cfg(test)]
pub(super) mod test_support {
    use std::collections::HashSet;

    use super::model::{DeclInfo, Shape};
    use super::*;

    pub(crate) fn parse(path: &Path, language: &Language, ctx: &WalkCtx) -> SourceFile {
        let (source, tree) = ctx.parse_tree(path, &(language.grammar)(path)).unwrap();
        SourceFile {
            path: path.to_path_buf(),
            source,
            tree,
        }
    }

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
        let file = parse(&dir.path().join(target), language, &ctx);
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

    /// Each file's `Names` head chunk and its predecessor's file name, for
    /// `files` written to a fresh directory.
    fn roster_chain(files: &[(&str, &str)]) -> Vec<(String, Option<String>)> {
        let dir = tempfile::tempdir().unwrap();
        for (name, content) in files {
            std::fs::write(dir.path().join(name), content).unwrap();
        }
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let name = |key: &BatchKey| match key {
            BatchKey::Code(key) => key.file.file_name().unwrap().to_string_lossy().into_owned(),
            other => panic!("not a code key: {other:?}"),
        };
        let mut heads: Vec<_> = expand_in_dir(dir.path(), &ctx)
            .into_iter()
            .filter(|batch| {
                matches!(&batch.key, BatchKey::Code(key) if key.rung == Rung::Names && key.sub == 0)
            })
            .map(|batch| (name(&batch.key), batch.predecessor.as_ref().map(name)))
            .collect();
        heads.sort();
        heads
    }

    fn link(file: &str, gate: Option<&str>) -> (String, Option<String>) {
        (file.to_owned(), gate.map(str::to_owned))
    }

    #[test]
    fn code_mod_roster_chain_puts_the_most_referenced_module_first() {
        let chain = roster_chain(&[
            ("__init__.py", "from .core import Engine\n"),
            ("core.py", "class Engine:\n    pass\n"),
            (
                "cli.py",
                "from .core import Engine\n\ndef main():\n    pass\n",
            ),
            (
                "helpers.py",
                "from . import core\n\ndef helper_with_a_long_name():\n    pass\n",
            ),
            ("test_core.py", "def test_engine():\n    pass\n"),
        ]);
        assert_eq!(
            chain,
            [
                link("__init__.py", None),
                link("cli.py", Some("helpers.py")),
                link("core.py", None),
                link("helpers.py", Some("core.py")),
                link("test_core.py", Some("cli.py")),
            ]
        );
    }

    /// `variant.go` redeclares `New`, as a build-tagged platform variant
    /// does, so naming it is not a dependency on `engine.go`.
    #[test]
    fn code_mod_roster_chain_links_go_files_by_the_names_they_use() {
        let chain = roster_chain(&[
            (
                "engine.go",
                "package gin\n\ntype Engine struct{}\n\nfunc New() *Engine { return nil }\n",
            ),
            (
                "mode.go",
                "package gin\n\nfunc SetModeWithAVeryLongName(value string) {}\n",
            ),
            (
                "group.go",
                "package gin\n\ntype Group struct{ engine *Engine }\n",
            ),
            (
                "variant.go",
                "package gin\n\ntype Engine2 struct{}\n\nfunc New() {}\n",
            ),
        ]);
        assert_eq!(
            chain,
            [
                link("engine.go", None),
                link("group.go", Some("engine.go")),
                link("mode.go", Some("group.go")),
                link("variant.go", Some("mode.go")),
            ]
        );
    }
}
