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
            if !is_leading_trivia(node) {
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

    /// One [`Item`] per named child of `list` that `admit` accepts, with
    /// the own-row comments and attributes directly above it (see
    /// [`Self::node_items`]); those above a child `admit` rejects are
    /// dropped with it.
    pub(crate) fn admitted_items(
        &self,
        list: Node,
        after_row: usize,
        admit: impl Fn(Node) -> bool,
    ) -> Vec<Item> {
        let mut nodes = Vec::new();
        let mut leading = Vec::new();
        let mut cursor = list.walk();
        for child in list.named_children(&mut cursor) {
            if is_leading_trivia(child) {
                if self.starts_own_row(child) {
                    leading.push(child);
                }
            } else if admit(child) {
                nodes.append(&mut leading);
                nodes.push(child);
            } else {
                leading.clear();
            }
        }
        nodes.append(&mut leading);
        self.node_items(nodes, after_row)
    }

    /// A `Callable` spanning `rows` whose body holds `statements`: the
    /// head runs through the row before the first statement, and at least
    /// through `floor` (the name row, or the row opening the body); each
    /// statement past it is one body item. Without statements, all head.
    pub(crate) fn callable<'tree>(
        &self,
        name_rows: Vec<usize>,
        rows: RangeInclusive<usize>,
        statements: impl IntoIterator<Item = Node<'tree>>,
        floor: usize,
    ) -> DeclInfo {
        let mut statements = statements.into_iter().peekable();
        let head_end = statements
            .peek()
            .map_or(*rows.end(), |first| first.start_position().row.max(floor));
        DeclInfo {
            body: self.node_items(statements, head_end),
            ..DeclInfo::new(
                name_rows,
                (*rows.start()..=head_end).collect(),
                Shape::Callable,
            )
        }
    }

    /// A `Whole` spanning `rows` whose entries are the named children of
    /// `block`: one body item per entry (see [`Self::node_items`]), and a
    /// head of the rows through the one opening `block` plus those after
    /// the last entry. Without a block, all head.
    pub(crate) fn whole(
        &self,
        name_rows: Vec<usize>,
        rows: RangeInclusive<usize>,
        block: Option<Node>,
    ) -> DeclInfo {
        let Some(block) = block else {
            return DeclInfo::new(name_rows, rows.collect(), Shape::Whole);
        };
        let open_row = block.start_position().row + 1;
        let body = self.node_items(block.named_children(&mut block.walk()), open_row);
        let content_end = body
            .last()
            .map_or(open_row, |item| item.rows[item.rows.len() - 1]);
        DeclInfo {
            body,
            ..DeclInfo::new(
                name_rows,
                block_head(rows, open_row, content_end),
                Shape::Whole,
            )
        }
    }

    /// Consecutive `rows` split into one [`Item`] per paragraph, broken at
    /// blank rows.
    pub(crate) fn paragraphs(&self, rows: impl IntoIterator<Item = usize>) -> Vec<Item> {
        self.paragraphs_by(rows, |line| line.trim().is_empty())
    }

    /// `rows` split into one [`Item`] per paragraph: a row whose line
    /// `is_break` accepts ends the paragraph above it, which keeps it.
    pub(crate) fn paragraphs_by(
        &self,
        rows: impl IntoIterator<Item = usize>,
        is_break: impl Fn(&str) -> bool,
    ) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        let mut after_break = true;
        for row in rows {
            let breaks = is_break(self.line(row));
            match items.last_mut() {
                Some(item) if breaks || !after_break => item.rows.push(row),
                _ => items.push(Item::new([row])),
            }
            after_break = breaks;
        }
        items
    }
}

/// The named children of `node`; none without a node.
fn named_children<'tree>(node: Option<Node<'tree>>) -> Vec<Node<'tree>> {
    node.map_or_else(Vec::new, |node| {
        node.named_children(&mut node.walk()).collect()
    })
}

/// A comment, or a Rust attribute: it belongs to the node after it.
fn is_leading_trivia(node: Node) -> bool {
    node.kind().contains("comment") || node.kind() == "attribute_item"
}

/// The head of a `Whole` with an entry block: `rows` through `open_row`,
/// then those after `content_end`.
fn block_head(rows: RangeInclusive<usize>, open_row: usize, content_end: usize) -> Vec<usize> {
    (*rows.start()..=open_row)
        .chain(content_end.max(open_row) + 1..=*rows.end())
        .collect()
}

fn file_stem(path: &Path) -> Option<&str> {
    path.file_stem()?.to_str()
}

fn file_name(path: &Path) -> Option<&str> {
    path.file_name()?.to_str()
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
    /// `language`: the normalized model the engine emits from, asserting
    /// the [`model`] invariants on it.
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
        let model = emit::normalize((language.extract)(&file, &ctx), &file);
        if let Err(violation) = check_contract(&model) {
            panic!("{target}: {violation}");
        }
        (file, model)
    }

    pub(crate) fn extract_source(language: &Language, relative: &str, source: &str) -> FileModel {
        extract_in(language, &[(relative, source)], relative).1
    }

    pub(crate) fn rows(items: &[Item]) -> Vec<Vec<usize>> {
        items.iter().map(|item| item.rows.clone()).collect()
    }

    /// One line per declaration (members indented): shape, name rows,
    /// head, doc and body.
    pub(crate) fn describe(model: &FileModel) -> Vec<String> {
        fn line(decl: &DeclInfo, indent: &str) -> String {
            format!(
                "{indent}{:?} name {:?} head {:?} doc {:?} body {:?}",
                decl.shape,
                decl.name_rows,
                decl.head,
                rows(&decl.doc),
                rows(&decl.body),
            )
        }
        model
            .decls
            .iter()
            .flat_map(|decl| {
                std::iter::once(line(decl, "")).chain(decl.members.iter().map(|m| line(m, "  ")))
            })
            .collect()
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
    pub(crate) fn check_contract(model: &FileModel) -> Result<(), String> {
        let mut owned = part_rows(&model.module_doc);
        for row in part_rows(&model.reexports) {
            if !owned.insert(row) {
                return Err(format!("re-export row {row} owned twice"));
            }
        }
        for decl in &model.decls {
            let container_rows = decl_rows(decl)?;
            let mut all_rows = container_rows.clone();
            for member in &decl.members {
                if !member.members.is_empty() {
                    return Err(format!("member {:?} has members", member.name_rows));
                }
                let member_rows = decl_rows(member)?;
                let shared: Vec<_> = member_rows
                    .intersection(&container_rows)
                    .filter(|row| !member.name_rows.contains(row))
                    .collect();
                if !shared.is_empty() {
                    return Err(format!("member rows {shared:?} also in its container"));
                }
                all_rows.extend(member_rows);
            }
            for row in all_rows {
                if !owned.insert(row) {
                    return Err(format!("declaration row {row} owned twice"));
                }
            }
        }
        Ok(())
    }

    /// Every row in `decl`'s parts, once its own invariants hold.
    fn decl_rows(decl: &DeclInfo) -> Result<HashSet<usize>, String> {
        let name_rows = &decl.name_rows;
        if name_rows.is_empty() || decl.head.is_empty() {
            return Err(format!(
                "declaration {name_rows:?} has no name or head rows"
            ));
        }
        let head: HashSet<usize> = decl.head.iter().copied().collect();
        let doc = part_rows(&decl.doc);
        let body = part_rows(&decl.body);
        if !(head.is_disjoint(&doc) && head.is_disjoint(&body) && doc.is_disjoint(&body)) {
            return Err(format!("declaration {name_rows:?} has overlapping parts"));
        }
        let rendered: HashSet<usize> = match decl.shape {
            Shape::Callable => head.clone(),
            Shape::Whole => head.union(&body).copied().collect(),
        };
        if !name_rows.iter().all(|row| rendered.contains(row)) {
            return Err(format!("name rows {name_rows:?} outside the rendered rows"));
        }
        Ok(head.union(&doc).chain(body.iter()).copied().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! fixtures {
        (
            training { $($training:ident $training_url:literal $training_rev:literal,)* }
            validation { $($validation:tt)* }
        ) => {
            const TRAINING_FIXTURES: &[&str] = &[$(stringify!($training)),*];
        };
    }
    include!("../../../tests/data/fixtures.rs");

    /// Files the contract sweep checks per fixture, spread evenly over its
    /// sorted paths.
    const SWEEP_FILES_PER_FIXTURE: usize = 40;

    /// Real source holds shapes no hand-written case anticipates, so the
    /// training fixtures' parsed files must extract within the [`model`]
    /// contract. Machine-generated files are exempt: a minified bundle puts
    /// declarations and an `export { … }` on one row, which rows cannot
    /// split, and the engine drops the doubly claimed row from the later
    /// batch.
    #[test]
    fn code_mod_training_fixtures_extract_within_the_contract() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
        let mut files_per_language = [0; LANGUAGES.len()];
        let mut violations = Vec::new();
        for name in TRAINING_FIXTURES {
            let root = fixtures.join(name.replace('_', "-"));
            assert!(
                root.is_dir(),
                "fixture `{name}` missing; run `cargo run --example clone_fixtures`"
            );
            let mut files: Vec<(PathBuf, &Language)> = ignore::WalkBuilder::new(&root)
                .build()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_some_and(|kind| kind.is_file()))
                .filter_map(|entry| {
                    let language = Language::from_path(entry.path())?;
                    Some((entry.into_path(), language))
                })
                .collect();
            files.sort_by(|left, right| left.0.cmp(&right.0));
            let stride = files.len().div_ceil(SWEEP_FILES_PER_FIXTURE).max(1);
            let files: Vec<_> = files.into_iter().step_by(stride).collect();
            let grammars: Vec<(&Path, tree_sitter::Language)> = files
                .iter()
                .map(|(path, language)| (path.as_path(), (language.grammar)(path)))
                .collect();
            let ctx = WalkCtx::new(root);
            ctx.parse_each(&grammars, |index, source, tree| {
                let (path, language) = &files[index];
                let file = SourceFile {
                    path: path.clone(),
                    source,
                    tree,
                };
                let model = emit::normalize((language.extract)(&file, &ctx), &file);
                if let Err(violation) = test_support::check_contract(&model)
                    && !super::super::plaintext::is_machine_generated_text(&file.source)
                {
                    violations.push(format!("{}: {violation}", path.display()));
                }
                let slot = LANGUAGES
                    .iter()
                    .position(|candidate| candidate.extensions == language.extensions)
                    .unwrap();
                files_per_language[slot] += 1;
            });
        }
        assert!(
            files_per_language.iter().all(|&count| count > 0),
            "{files_per_language:?}"
        );
        assert!(violations.is_empty(), "{}", violations.join("\n"));
    }

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
