//! The shared code engine: one declaration ladder for every source
//! language, fed by a thin extraction module per language. Each
//! `code/<lang>.rs` exports one [`Language`]; the engine
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
use super::plaintext::{has_minified_lines, is_derived_artifact_name};
use super::{WalkCtx, node_end_row_trimmed, parse_each};
use crate::batch::{Batch, BatchKey, Rung};
use crate::fs_util::{EntryKind, list_dir};
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
    /// The language whose extensions include `path`'s extension.
    pub(crate) fn from_path(path: &Path) -> Option<&'static Language> {
        LANGUAGES
            .into_iter()
            .find(|language| has_extension(path, language.extensions))
    }
}

/// Whether `path`'s extension is one of `extensions`, ignoring case.
fn has_extension(path: &Path, extensions: &[&str]) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extensions
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        })
}

/// Every extension some [`Language`] claims.
pub(crate) fn parsed_extensions() -> impl Iterator<Item = &'static str> {
    LANGUAGES
        .into_iter()
        .flat_map(|language| language.extensions.iter().copied())
}

/// How deeply the scopes an extraction descends into (C feature gates
/// and `extern "C"` blocks, Rust inline modules) may
/// nest; deeper ones are not descended. Tree-sitter finds a node's parent
/// and siblings by walking down from the root, so the work per descended
/// node grows with its depth, and generated input can nest hundreds of
/// thousands deep. C99 requires compilers to support 63 nested levels of
/// conditional inclusion.
const MAX_SCOPE_NESTING: usize = 63;

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
        for child in list.named_children(&mut list.walk()) {
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
    /// statement past it is one body item. Without a statement past the
    /// head, all head.
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
        let body = self.node_items(statements, head_end);
        let head_end = if body.is_empty() {
            *rows.end()
        } else {
            head_end
        };
        DeclInfo {
            body,
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
    let normalized = |name: &str| name.to_ascii_lowercase().replace('-', "_");
    file_stem(path)
        .zip(file_name(dir))
        .is_some_and(|(stem, dir_name)| normalized(stem) == normalized(dir_name))
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

/// The language that extracts `path`, and its source read for the parse,
/// or `None` when the file is left to its listing row. Derived artifacts
/// (`app.min.js`, `main.bundle.js`, `api.generated.ts`), files of lines
/// too long to be hand-wrapped and files under a generator's banner are
/// left there, as the plaintext fallback leaves them: extracting one
/// renders machine output. They are caught before they parse, since
/// their tree would be built only to be discarded. So is a Python file
/// whose comment runs would stall the parse
/// ([`python::has_costly_comment_runs`]).
fn extracting_language(path: &Path, ctx: &WalkCtx) -> Option<(&'static Language, Arc<Source>)> {
    let language = Language::from_path(path)?;
    let name = path.file_name()?.to_string_lossy().to_ascii_lowercase();
    if is_derived_artifact_name(&name) {
        return None;
    }
    let source = ctx.read_for_parse(path)?;
    let stalls_the_parser = language.extensions == python::LANGUAGE.extensions
        && python::has_costly_comment_runs(&source);
    (!has_minified_lines(&source) && !has_generator_banner(&source) && !stalls_the_parser)
        .then_some((language, source))
}

/// Rows from the top of a file that [`has_generator_banner`] reads
/// whatever they hold; past them it reads only as far as the file's
/// leading comment rows run, counting every row of a `/* … */` block
/// (a `#include` or `#define` row ends them).
const GENERATOR_BANNER_ROWS: usize = 12;

/// Whether the comment rows near the top of `source` (or anywhere in the
/// comment block a file opens with, so a banner under a long license
/// header counts) say a tool wrote the file: a row tagged `@generated` or
/// Argument Clinic's `[clinic start generated code]`, a row mentioning a
/// file or code as auto-generated (`# This file is auto-generated`), a
/// row near the top opening with "auto-generated" (`<auto-generated>`),
/// or rows saying both "generated" and "do not edit" (`// Code generated
/// by stringer; DO NOT EDIT.`). Only rows that are wholly comment count: a
/// hand-written `__version__ = "1.0"  # DO NOT EDIT` is not a banner, nor
/// is prose about generated things (`The auto-generated primary key …`,
/// an automatically generated `profile`) or a bare "do not edit" on a
/// hand-written header.
pub(in crate::walker) fn has_generator_banner(source: &str) -> bool {
    const COMMENT_OPENERS: &[&str] = &["//", "/*", "*", "#", "--", "\"\"\"", "'''"];
    const CLINIC_MARKER: &str = "[clinic start generated code]";
    #[rustfmt::skip]
    const AUTO_GENERATED: &[&str] =
        &["auto-generated", "autogenerated", "auto generated", "automatically generated"];
    let mut says_generated = false;
    let mut says_do_not_edit = false;
    let mut in_leading_comments = true;
    let mut in_block_comment = false;
    for (index, line) in source.lines().enumerate() {
        if index >= GENERATOR_BANNER_ROWS && !in_leading_comments {
            break;
        }
        let line = line.trim_start().to_ascii_lowercase();
        if line.contains(CLINIC_MARKER) {
            return true;
        }
        let is_comment = in_block_comment
            || COMMENT_OPENERS
                .iter()
                .any(|opener| line.starts_with(opener));
        let is_preprocessor = !in_block_comment
            && line.starts_with('#')
            && line[1..].starts_with(char::is_alphabetic);
        in_leading_comments &= line.is_empty() || is_comment && !is_preprocessor;
        if !is_comment {
            continue;
        }
        in_block_comment = ends_in_block_comment(&line, in_block_comment);
        let text = line.trim_start_matches(|character: char| !character.is_ascii_alphanumeric());
        let names_file_or_code = text
            .split(|character: char| !character.is_ascii_alphanumeric())
            .any(|word| matches!(word, "file" | "files" | "code"));
        let is_auto_generated_banner = AUTO_GENERATED.iter().any(|marker| {
            index < GENERATOR_BANNER_ROWS && text.starts_with(marker)
                || text.contains(marker) && names_file_or_code
        });
        if line.contains("@generated") || is_auto_generated_banner {
            return true;
        }
        says_generated |= text.contains("generated");
        says_do_not_edit |= text.contains("do not edit");
    }
    says_generated && says_do_not_edit
}

/// Whether a `/* … */` comment is open at the end of `line`, given
/// whether one was open at its start.
fn ends_in_block_comment(line: &str, starts_in_block_comment: bool) -> bool {
    let mut in_block_comment = starts_in_block_comment;
    let mut rest = line;
    loop {
        if in_block_comment {
            let Some(close) = rest.find("*/") else {
                return true;
            };
            rest = &rest[close + 2..];
            in_block_comment = false;
        } else {
            match rest.find("/*") {
                Some(open) if !rest[..open].contains("//") => {
                    rest = &rest[open + 2..];
                    in_block_comment = true;
                }
                _ => return false,
            }
        }
    }
}

/// Batches for every source file in `dir`, as [`extracting_language`]
/// selects them. Called by `FsWalker` once per scheduled directory
/// listing.
pub(crate) fn expand_in_dir(dir: &Path, ctx: &WalkCtx) -> Vec<Batch> {
    let files: Vec<(PathBuf, &Language, Arc<Source>)> = list_dir(dir, ctx.dir_filter())
        .iter()
        .filter(|(_, kind)| **kind == EntryKind::File)
        .filter_map(|(name, _)| {
            let path = dir.join(name);
            let (language, source) = extracting_language(&path, ctx)?;
            Some((path, language, source))
        })
        .collect();
    let grammars: Vec<(Arc<Source>, tree_sitter::Language)> = files
        .iter()
        .map(|(path, language, source)| (source.clone(), (language.grammar)(path)))
        .collect();
    let mut emitted = Vec::new();
    parse_each(&grammars, |index, source, tree| {
        let (path, language, _) = &files[index];
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
        let is_entrypoint = language.is_entrypoint.is_some_and(|test| test(path, ctx));
        let non_essential = model.non_essential || ctx.non_essential_factor(path) < 1.0;
        let reexported_words = if is_entrypoint && model.decls.is_empty() {
            model
                .reexports
                .iter()
                .flat_map(|item| &item.rows)
                .flat_map(|&row| {
                    file.line(row)
                        .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-'))
                })
                .filter(|word| !word.is_empty())
                .map(str::to_owned)
                .collect()
        } else {
            HashSet::new()
        };
        emitted.push(EmittedFile {
            batches: emit::emit_file(language, &file, model, is_entrypoint, ctx),
            chained: !is_entrypoint,
            reexported_words,
            non_essential,
            bytes,
            names,
            mentions: language
                .sibling_mentions
                .map_or_else(HashSet::new, |mentions| mentions(&file)),
        });
    });
    let depth = ctx.depth_from_root(&dir.join("_"));
    let entry_lift = crate::value::depth_factor(depth.min(1)) / crate::value::depth_factor(depth);
    chain_rosters(&mut emitted, entry_lift);
    emitted.into_iter().flat_map(|file| file.batches).collect()
}

/// One file's batches, and what places its roster on its directory's
/// roster chain.
struct EmittedFile {
    batches: Vec<Batch>,
    /// Entry files stay off the chain: their rosters are neither gated nor
    /// a gate.
    chained: bool,
    /// The words of a re-export-only entry file's re-export rows, which
    /// name the sibling modules it re-exports; empty for any other file.
    reexported_words: HashSet<String>,
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
///
/// Beside a re-export-only entry file (a barrel), the modules it re-exports
/// lead the chain, and the first one's head chunk is gated on the barrel's
/// last roster chunk and priced at the barrel's pinned depth: otherwise the
/// barrel, priced as an entry file, is all a package's source shows.
fn chain_rosters(files: &mut [EmittedFile], entry_lift: f64) {
    let barrel_end = files
        .iter()
        .filter(|file| !file.reexported_words.is_empty())
        .flat_map(|file| &file.batches)
        .filter(|batch| matches!(&batch.key, BatchKey::Code(key) if key.rung == Rung::Names))
        .map(|batch| batch.key.clone())
        .next_back();
    let reexported: HashSet<String> = files
        .iter_mut()
        .flat_map(|file| std::mem::take(&mut file.reexported_words))
        .collect();
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
    let mut ranked: Vec<(bool, usize, &mut EmittedFile)> = degrees
        .into_iter()
        .zip(chain)
        .map(|(degree, file)| {
            let is_reexported = file.names.iter().any(|name| reexported.contains(name));
            (is_reexported, degree, file)
        })
        .collect();
    ranked.sort_by_key(|(is_reexported, degree, file)| {
        (
            file.non_essential,
            !is_reexported,
            Reverse((*degree, file.bytes)),
        )
    });
    let mut gate: Option<BatchKey> = None;
    for (is_reexported, _, file) in ranked {
        let Some(head) = file.batches.iter_mut().find(|batch| {
            matches!(&batch.key, BatchKey::Code(key) if key.rung == Rung::Names && key.sub == 0)
        }) else {
            continue;
        };
        if gate.is_none() && is_reexported {
            head.value *= entry_lift;
            gate.clone_from(&barrel_end);
        }
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
    /// training fixtures' files the engine extracts (see [`expand_in_dir`])
    /// must extract within the [`model`] contract.
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
            let ctx = WalkCtx::new(root.clone());
            // The sweep reads every file, more than one run parses.
            ctx.parse_bytes_left.set(usize::MAX);
            let mut files: Vec<(PathBuf, &Language, Arc<Source>)> = ignore::WalkBuilder::new(&root)
                .build()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_some_and(|kind| kind.is_file()))
                .filter_map(|entry| {
                    let (language, source) = extracting_language(entry.path(), &ctx)?;
                    Some((entry.into_path(), language, source))
                })
                .collect();
            files.sort_by(|left, right| left.0.cmp(&right.0));
            let stride = files.len().div_ceil(SWEEP_FILES_PER_FIXTURE).max(1);
            let files: Vec<_> = files.into_iter().step_by(stride).collect();
            let grammars: Vec<(Arc<Source>, tree_sitter::Language)> = files
                .iter()
                .map(|(path, language, source)| (source.clone(), (language.grammar)(path)))
                .collect();
            parse_each(&grammars, |index, source, tree| {
                let (path, language, _) = &files[index];
                let file = SourceFile {
                    path: path.clone(),
                    source,
                    tree,
                };
                let model = emit::normalize((language.extract)(&file, &ctx), &file);
                if let Err(violation) = test_support::check_contract(&model) {
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

    #[test]
    fn code_mod_generator_banner_is_a_comment_row_near_the_top() {
        assert!(has_generator_banner(
            "package t\n\n// Code generated by \"go run gen.go\"; DO NOT EDIT.\n"
        ));
        assert!(has_generator_banner(
            "// This file is @generated by prost-build.\npub struct A {}\n"
        ));
        assert!(has_generator_banner(
            "/*[clinic input]\npreserve\n[clinic start generated code]*/\n"
        ));
        assert!(has_generator_banner(
            "# This file is auto-generated by utils/gen.py.\nclass A: ...\n"
        ));
        assert!(!has_generator_banner(
            "__version__ = \"1.0\"  # DO NOT EDIT THIS LINE MANUALLY\n"
        ));
        assert!(has_generator_banner("// <auto-generated>\nclass A {}\n"));
        assert!(has_generator_banner(
            "/*\n * WARNING: do not edit!\n * Generated by Makefile from buildinf.h.in\n */\n"
        ));
        assert!(has_generator_banner(
            "/** This is an auto-generated file **/\nint x;\n"
        ));
        for handwritten in [
            "\"\"\"The auto-generated primary key can't be passed to the constructor.\"\"\"\n",
            "# Plain and simple local endpoint with an auto-generated certificate:\n",
            "// NOTE: Assertions have been autogenerated by utils/update_cc_test_checks.py\n",
            "// DO NOT EDIT this file to adjust your configuration. Create your own\n",
            "//     * docs is the murex online documentation. Contents in here are autogenerated\n",
        ] {
            assert!(!has_generator_banner(handwritten), "{handwritten}");
        }
        let license = "// Licensed under the Apache License.\n".repeat(GENERATOR_BANNER_ROWS);
        let under_license =
            format!("{license}\n// Code generated by conversion-gen. DO NOT EDIT.\n");
        assert!(has_generator_banner(&under_license));
        let block_license = format!(
            "/*\n{}*/\n\n// Code generated by conversion-gen. DO NOT EDIT.\n\npackage v1\n",
            "Licensed under the Apache License.\n".repeat(GENERATOR_BANNER_ROWS)
        );
        assert!(has_generator_banner(&block_license));
        let after_block = format!(
            "/*\nLicense.\n*/\n{}// Code generated. DO NOT EDIT.\n",
            "x = 1\n".repeat(GENERATOR_BANNER_ROWS)
        );
        assert!(!has_generator_banner(&after_block));
        let late = format!("{}// @generated\n", "x = 1\n".repeat(GENERATOR_BANNER_ROWS));
        assert!(!has_generator_banner(&late));
        let past_includes = format!("{license}#include <a.h>\n// Helpers for autogenerated code\n");
        assert!(!has_generator_banner(&past_includes));
        for usage_prose in [
            " *\tunconfined - special automatically generated unconfined profile\n",
            " *      auto-generated wakeup in preemptive monitor.\n",
        ] {
            let usage = format!("/*\n{}{usage_prose} */\nint x;\n", " * Usage.\n".repeat(20));
            assert!(!has_generator_banner(&usage), "{usage_prose}");
        }
    }

    #[test]
    fn code_mod_no_two_languages_claim_one_extension() {
        let mut seen = HashSet::new();
        for extension in parsed_extensions() {
            assert!(seen.insert(extension.to_ascii_lowercase()), "{extension}");
        }
    }

    /// A container renders its closing row with its opening row, so an
    /// excerpt of its first entries still shows where it ends: a long
    /// program `main`, and a `type (` group of structs.
    #[test]
    fn code_mod_partly_shown_container_keeps_its_closer() {
        let dir = tempfile::tempdir().unwrap();
        let statements: String = (0..400)
            .map(|index| format!("\tfmt.Println(\"step {index} of the program\")\n"))
            .collect();
        let source = format!("package main\n\nimport \"fmt\"\n\nfunc main() {{\n{statements}}}\n");
        std::fs::write(dir.path().join("main.go"), source).unwrap();
        let out = crate::render(dir.path(), 3000, None).unwrap();
        assert!(out.contains("5→func main() {"), "{out}");
        assert!(out.contains("step 0 of the program"), "{out}");
        assert!(!out.contains("step 399 of the program"), "{out}");
        assert!(out.contains("406→}"), "{out}");

        let dir = tempfile::tempdir().unwrap();
        let structs: String = (0..40)
            .map(|node| {
                let fields: String = (0..6)
                    .map(|field| format!("\t\tField{field} int // field {field} of node {node}\n"))
                    .collect();
                format!("\t// A Node{node} node is node number {node}.\n\tNode{node} struct {{\n{fields}\t}}\n")
            })
            .collect();
        let source = format!("package ast\n\n// Nodes.\ntype (\n{structs})\n");
        let closer = source.lines().count();
        std::fs::write(dir.path().join("ast.go"), source).unwrap();
        let out = crate::render(dir.path(), 1000, None).unwrap();
        assert!(out.contains("4→type ("), "{out}");
        assert!(!out.contains("field 5 of node 39"), "{out}");
        assert!(out.contains(&format!("{closer}→)")), "{out}");
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
                link("core.py", Some("__init__.py")),
                link("helpers.py", Some("core.py")),
                link("test_core.py", Some("cli.py")),
            ]
        );
    }

    #[test]
    fn code_mod_roster_chain_opens_a_barrels_modules_after_it() {
        let chain = roster_chain(&[
            (
                "index.ts",
                "export { parse } from './parser'\nexport * from './ast'\n",
            ),
            ("parser.ts", "export function parse() {}\n"),
            ("ast.ts", "export interface Node {}\n"),
            (
                "tokenizer.ts",
                "export class TokenizerWithAMuchLongerNameThanTheOthers {}\n",
            ),
        ]);
        assert_eq!(
            chain,
            [
                link("ast.ts", Some("parser.ts")),
                link("index.ts", None),
                link("parser.ts", Some("index.ts")),
                link("tokenizer.ts", Some("ast.ts")),
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
