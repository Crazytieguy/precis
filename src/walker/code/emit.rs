//! [`FileModel`] → batches: normalization, then the ladder per file.
//!
//! Emission order, each batch gated on its predecessor:
//! `ModuleDoc` chunks (no predecessor), `Names` chunks (no predecessor
//! here; `expand_in_dir` then gates the head on its directory's roster
//! chain), then per declaration in normalized order its `Decl` chunks (gated on the
//! owner of its first name row: the `Names` chunk listing it, or for a
//! member the container `Decl` chunk listing it), `Doc` and, for
//! `Callable`, `Body` chunks (gated on the declaration's first `Decl`
//! chunk, or its predecessor when that chunk was covered), then the
//! members of a container. Chunk `i > 0` of any part is gated on the last
//! emitted chunk before it.

use std::collections::HashSet;
use std::path::Path;

use super::chunk::{chunk_ranges, chunk_value_factor, split_oversize_items};
use super::ledger::Ledger;
use super::model::{DeclInfo, FileModel, Item, Shape};
use super::{Language, SourceFile};
use crate::batch::{Batch, BatchKey, CodeKey, Rung};
use crate::content::{BatchContent, Render, Span};
use crate::value::{DEFAULT_CONCAVITY_EXPONENT, code_rung_value, roster_mass};
use crate::walker::{WalkCtx, file_depth_factor};

/// Every batch of one file.
pub(super) fn emit_file(
    language: &Language,
    file: &SourceFile,
    model: FileModel,
    is_entrypoint: bool,
    ctx: &WalkCtx,
) -> Vec<Batch> {
    let model = normalize(model, file);
    let mut emitter = Emitter {
        file,
        file_prior: file_prior(language, &file.path, is_entrypoint, ctx),
        ledger: Ledger::default(),
        out: Vec::new(),
    };
    emitter.emit(&model);
    emitter.out
}

/// Location prior shared by every batch of the file: depth (pinned to 1
/// for an entry file), non-essential discount and the language's file
/// weight.
fn file_prior(language: &Language, path: &Path, is_entrypoint: bool, ctx: &WalkCtx) -> f64 {
    file_depth_factor(path, ctx, is_entrypoint)
        * language.file_weight.map_or(1.0, |weight| weight(path, ctx))
}

/// Re-export rows past this many add nothing to a roster's value. A
/// barrel of hundreds of re-exported names otherwise ranks like a roster
/// of the declarations they name, chunk after chunk, ahead of the modules
/// that implement them.
const MAX_REEXPORT_ENTRIES: usize = 40;

struct Emitter<'a> {
    file: &'a SourceFile,
    file_prior: f64,
    ledger: Ledger,
    out: Vec<Batch>,
}

impl Emitter<'_> {
    fn emit(&mut self, model: &FileModel) {
        let module_doc = self.key(Rung::ModuleDoc, 0, 0);
        self.part(module_doc, &model.module_doc, None, self.file_prior);

        let mut roster: Vec<Item> = model.reexports.clone();
        roster.extend(
            model
                .decls
                .iter()
                .map(|decl| Item::new(decl.name_rows.clone())),
        );
        roster.sort_by_key(|item| item.rows.first().copied());
        let reexport_rows: usize = model.reexports.iter().map(|item| item.rows.len()).sum();
        let decl_rows: usize = model.decls.iter().map(|decl| decl.name_rows.len()).sum();
        let entries = decl_rows + reexport_rows.min(MAX_REEXPORT_ENTRIES);
        let names_value = self.file_prior * (entries as f64).powf(DEFAULT_CONCAVITY_EXPONENT);
        self.part(self.key(Rung::Names, 0, 0), &roster, None, names_value);

        let mut index = 0;
        for decl in &model.decls {
            index += 1;
            let container = self.decl(decl, index, None);
            for member in &decl.members {
                index += 1;
                self.decl(member, index, container.as_ref());
            }
        }
    }

    /// Emits one declaration's `Decl`, `Doc` and `Body` chunks. Returns the
    /// predecessor its members fall back to when no emitted chunk lists
    /// their name row.
    fn decl(
        &mut self,
        decl: &DeclInfo,
        index: u32,
        container: Option<&CodeKey>,
    ) -> Option<CodeKey> {
        let parent = self.ledger.owner(decl.name_rows[0]).or(container).cloned();
        let value = self.file_prior;
        let decl_key = self.key(Rung::Decl, index, decl.head[0]);
        let key = |rung| CodeKey {
            rung,
            ..decl_key.clone()
        };
        let mut head_items = vec![Item::new(decl.head.clone())];
        let mut decl_value = value;
        if decl.shape == Shape::Whole {
            // Head rows past the last entry (the closing `}`) close the
            // last chunk rather than the first.
            let body_end = decl.body.iter().filter_map(|item| item.rows.last()).max();
            let closing = head_items[0].rows.split_off(
                decl.head
                    .partition_point(|row| body_end.is_none_or(|end| row < end)),
            );
            head_items.extend(decl.body.iter().cloned());
            if !closing.is_empty() {
                head_items.push(Item::new(closing));
            }
            decl_value *= roster_mass(decl.body.len());
        }
        let decl_gate = self
            .part(key(Rung::Decl), &head_items, parent.as_ref(), decl_value)
            .or(parent);
        self.part(key(Rung::Doc), &decl.doc, decl_gate.as_ref(), value);
        if decl.shape == Shape::Callable {
            self.part(key(Rung::Body), &decl.body, decl_gate.as_ref(), value);
        }
        decl_gate
    }

    /// The head chunk's key of one part of the file.
    fn key(&self, rung: Rung, decl: u32, line: usize) -> CodeKey {
        CodeKey {
            rung,
            file: self.file.path.clone(),
            decl,
            sub: 0,
            line,
        }
    }

    /// Emits `items` as chained chunks of the part `head` names (its `sub`
    /// is 0), worth `prior × code_rung_value` unsplit. Returns chunk 0's
    /// key when it was emitted.
    fn part(
        &mut self,
        head: CodeKey,
        items: &[Item],
        parent: Option<&CodeKey>,
        prior: f64,
    ) -> Option<CodeKey> {
        let (items, costs) = split_oversize_items(items, self.file);
        let part_cost: usize = costs.iter().sum();
        let ranges = chunk_ranges(&costs);
        let part_value = prior * code_rung_value(head.rung);
        let mut gate = parent.cloned();
        let mut emitted_head = None;
        for (index, range) in ranges.iter().enumerate() {
            let mut rows: Vec<usize> = items[range.clone()]
                .iter()
                .flat_map(|item| item.rows.iter().copied())
                .collect();
            rows.sort_unstable();
            rows.dedup();
            let key = CodeKey {
                sub: index as u32,
                ..head.clone()
            };
            let claim = self.ledger.claim(&key, gate.as_ref(), &rows);
            if claim.covered {
                continue;
            }
            let chunk_cost = costs[range.clone()].iter().sum();
            let value = part_value * chunk_value_factor(chunk_cost, part_cost);
            self.out.push(Batch {
                key: BatchKey::Code(key.clone()),
                predecessor: gate.map(BatchKey::Code),
                content: BatchContent::Lines {
                    spans: spans(self.file, &claim.rows),
                    units: items[range.clone()]
                        .iter()
                        .map(|item| item.rows.clone())
                        .collect(),
                },
                value,
            });
            if index == 0 {
                emitted_head = Some(key.clone());
            }
            gate = Some(key);
        }
        emitted_head
    }
}

/// Sorted non-blank `rows` → spans, bridging gaps that hold only blank
/// rows so the rendered region keeps the source's shape.
fn spans(file: &SourceFile, rows: &[usize]) -> Vec<Span> {
    let mut spans: Vec<Span> = Vec::new();
    for &row in rows {
        if let Some(last) = spans.last_mut()
            && (last.end + 1..row).all(|gap| file.line(gap).trim().is_empty())
        {
            last.end = row;
            continue;
        }
        spans.push(Span {
            path: file.path.clone(),
            start: row,
            end: row,
            render: Render::Full,
        });
    }
    spans
}

/// The engine-side steps of the [`super::model`] contract (its "Rows"
/// and "What the engine does" sections), plus dropping out-of-range rows.
pub(super) fn normalize(mut model: FileModel, file: &SourceFile) -> FileModel {
    let content_row =
        |row: usize| (1..=file.line_count()).contains(&row) && !file.line(row).trim().is_empty();
    clean_items(&mut model.module_doc, &content_row);
    let module_doc_rows: HashSet<usize> = model
        .module_doc
        .iter()
        .flat_map(|item| item.rows.iter().copied())
        .collect();
    let keep = |row: usize| content_row(row) && !module_doc_rows.contains(&row);
    clean_items(&mut model.reexports, &keep);
    model.decls = normalize_siblings(std::mem::take(&mut model.decls), &keep);
    model
}

/// Cleans, orders, merges and trims one sibling list (top-level
/// declarations, or one container's members), and their members.
fn normalize_siblings(decls: Vec<DeclInfo>, keep: &dyn Fn(usize) -> bool) -> Vec<DeclInfo> {
    let mut decls: Vec<(usize, DeclInfo)> = decls
        .into_iter()
        .filter_map(|mut decl| {
            clean_decl(&mut decl, keep);
            Some((first_row(&decl)?, decl))
        })
        .collect();
    decls.sort_by_key(|(first, _)| *first);
    let mut merged: Vec<(usize, DeclInfo)> = Vec::with_capacity(decls.len());
    for (first, decl) in decls {
        match merged.last_mut() {
            Some((previous_first, previous)) if *previous_first == first => {
                merge_into(previous, decl)
            }
            _ => merged.push((first, decl)),
        }
    }
    let firsts: Vec<usize> = merged.iter().map(|(first, _)| *first).collect();
    merged
        .into_iter()
        .enumerate()
        .filter_map(|(index, (_, mut decl))| {
            let next_first = firsts.get(index + 1);
            let before_next = |row: usize| next_first.is_none_or(|&limit| row < limit);
            clean_decl(&mut decl, &before_next);
            let member_keep = |row: usize| keep(row) && before_next(row);
            decl.members = normalize_siblings(std::mem::take(&mut decl.members), &member_keep);
            finish_decl(decl)
        })
        .collect()
}

/// Keeps only `keep` rows in every part, sorted and deduplicated. Members
/// are cleaned by their own sibling pass.
fn clean_decl(decl: &mut DeclInfo, keep: &dyn Fn(usize) -> bool) {
    clean_rows(&mut decl.name_rows, keep);
    clean_rows(&mut decl.head, keep);
    clean_items(&mut decl.doc, keep);
    clean_items(&mut decl.body, keep);
}

fn clean_rows(rows: &mut Vec<usize>, keep: &dyn Fn(usize) -> bool) {
    rows.retain(|&row| keep(row));
    sort_dedup(rows);
}

fn sort_dedup(rows: &mut Vec<usize>) {
    rows.sort_unstable();
    rows.dedup();
}

fn clean_items(items: &mut Vec<Item>, keep: &dyn Fn(usize) -> bool) {
    for item in items.iter_mut() {
        clean_rows(&mut item.rows, keep);
    }
    items.retain(|item| !item.rows.is_empty());
}

/// Makes the parts disjoint (`doc` wins over `head`, `head` over
/// `body`), then restores the non-empty `head` / `name_rows` invariant:
/// an empty head falls back to the name rows and vice versa; a
/// declaration with neither is dropped.
fn finish_decl(mut decl: DeclInfo) -> Option<DeclInfo> {
    let mut claimed: HashSet<usize> = decl
        .doc
        .iter()
        .flat_map(|item| item.rows.iter().copied())
        .collect();
    decl.head.retain(|row| !claimed.contains(row));
    claimed.extend(&decl.head);
    clean_items(&mut decl.body, &|row| !claimed.contains(&row));
    if decl.head.is_empty() {
        decl.head = decl.name_rows.clone();
    }
    if decl.name_rows.is_empty() {
        decl.name_rows = vec![*decl.head.first()?];
    }
    Some(decl)
}

/// The smallest row in any of the declaration's own parts.
fn first_row(decl: &DeclInfo) -> Option<usize> {
    decl.name_rows
        .iter()
        .chain(&decl.head)
        .chain(
            decl.doc
                .iter()
                .chain(&decl.body)
                .flat_map(|item| &item.rows),
        )
        .min()
        .copied()
}

fn merge_into(target: &mut DeclInfo, other: DeclInfo) {
    target.name_rows.extend(other.name_rows);
    target.head.extend(other.head);
    target.doc.extend(other.doc);
    target.body.extend(other.body);
    target.members.extend(other.members);
    sort_dedup(&mut target.name_rows);
    sort_dedup(&mut target.head);
    target.doc.sort_by_key(|item| item.rows[0]);
    target.body.sort_by_key(|item| item.rows[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(range: std::ops::RangeInclusive<usize>) -> Item {
        Item::new(range)
    }

    fn decl(name_row: usize, head: Vec<usize>, shape: Shape) -> DeclInfo {
        DeclInfo::new(vec![name_row], head, shape)
    }

    /// A file of `lines` non-blank rows.
    fn with_file<T>(lines: usize, run: impl FnOnce(&SourceFile) -> T) -> T {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.lua");
        let source: String = (1..=lines).map(|row| format!("x{row} = {row}\n")).collect();
        std::fs::write(&path, source).unwrap();
        let ctx = WalkCtx::new(dir.path().to_path_buf());
        let file = super::super::test_support::parse(&path, &super::super::lua::LANGUAGE, &ctx);
        run(&file)
    }

    /// Emits `model` for a file of `lines` rows, asserting no row was
    /// dropped as a non-ancestor overlap. Returns each batch's key,
    /// predecessor and rendered rows.
    fn emit(lines: usize, model: FileModel) -> Vec<(CodeKey, Option<CodeKey>, Vec<usize>)> {
        with_file(lines, |file| {
            let mut emitter = Emitter {
                file,
                file_prior: 1.0,
                ledger: Ledger::default(),
                out: Vec::new(),
            };
            emitter.emit(&normalize(model, file));
            emitter
                .out
                .into_iter()
                .map(|batch| {
                    let code_key = |key| match key {
                        BatchKey::Code(key) => key,
                        other => panic!("not a code key: {other:?}"),
                    };
                    let BatchContent::Lines { spans, .. } = batch.content else {
                        panic!("not a lines batch");
                    };
                    let rows = spans.iter().flat_map(|span| span.start..=span.end);
                    (
                        code_key(batch.key),
                        batch.predecessor.map(code_key),
                        rows.collect(),
                    )
                })
                .collect()
        })
    }

    fn short(key: &CodeKey) -> (Rung, u32, u32) {
        (key.rung, key.decl, key.sub)
    }

    #[test]
    fn emit_predecessor_table() {
        let mut callable = decl(3, vec![3], Shape::Callable);
        callable.doc = vec![rows(2..=2)];
        callable.body = vec![rows(4..=5)];
        let mut whole = decl(8, vec![7, 8], Shape::Whole);
        whole.body = vec![rows(9..=9)];
        whole.doc = vec![rows(6..=6)];
        let model = FileModel {
            module_doc: vec![rows(1..=1)],
            reexports: Vec::new(),
            decls: vec![whole, callable],
        };
        let batches = emit(10, model);
        let table: Vec<_> = batches
            .iter()
            .map(|(key, predecessor, _)| (short(key), predecessor.as_ref().map(short)))
            .collect();
        use Rung::*;
        assert_eq!(
            table,
            [
                ((ModuleDoc, 0, 0), None),
                ((Names, 0, 0), None),
                // The one-row signature is already on the roster: covered,
                // so its doc and body gate on the roster.
                ((Doc, 1, 0), Some((Names, 0, 0))),
                ((Body, 1, 0), Some((Names, 0, 0))),
                ((Decl, 2, 0), Some((Names, 0, 0))),
                ((Doc, 2, 0), Some((Decl, 2, 0))),
            ]
        );
        assert_eq!(batches[1].2, [3, 8]);
        assert_eq!(batches[4].2, [7, 8, 9]);
    }

    /// An oversize part chains its chunks, and a body that is one huge
    /// statement still renders a prefix: the statement splits into rows,
    /// chunked like any oversize part.
    #[test]
    fn emit_chains_the_chunks_of_an_oversize_part_or_item() {
        let one_item_per_row = (2..=80).map(|row| rows(row..=row)).collect();
        for body in [one_item_per_row, vec![rows(2..=80)]] {
            let mut callable = decl(1, vec![1], Shape::Callable);
            callable.body = body;
            let batches = emit(
                80,
                FileModel {
                    decls: vec![callable],
                    ..FileModel::default()
                },
            );
            let bodies: Vec<_> = batches
                .iter()
                .filter(|(key, _, _)| key.rung == Rung::Body)
                .collect();
            assert!(bodies.len() > 1);
            for pair in bodies.windows(2) {
                assert_eq!(pair[1].1.as_ref(), Some(&pair[0].0));
                assert_eq!(pair[1].0.sub, pair[0].0.sub + 1);
            }
            assert_eq!(bodies[0].2.first(), Some(&2));
            assert_eq!(bodies.last().unwrap().2.last(), Some(&80));
        }
    }

    /// The roster lists in source order, so an oversize trailing re-export
    /// block chunks after the declarations named before it instead of
    /// gating them.
    #[test]
    fn emit_roster_keeps_source_order_ahead_of_a_trailing_reexport_block() {
        let decls = (1..=60).map(|row| decl(row, vec![row], Shape::Whole));
        let batches = emit(
            100,
            FileModel {
                reexports: vec![rows(61..=100)],
                decls: decls.collect(),
                ..FileModel::default()
            },
        );
        let names: Vec<_> = batches
            .iter()
            .filter(|(key, _, _)| key.rung == Rung::Names)
            .collect();
        assert!(names.len() > 1);
        assert_eq!(names[0].1, None);
        assert_eq!(names[0].2.first(), Some(&1));
        let (last, earlier) = names.split_last().unwrap();
        assert_eq!(last.2.last(), Some(&100));
        assert!(
            earlier
                .iter()
                .all(|chunk| chunk.2.iter().all(|&row| row <= 60))
        );
    }

    /// A member hangs under the container chunk that lists its name row;
    /// a member opening on the container's first row (`class A { foo(`)
    /// still gets its own key.
    #[test]
    fn emit_member_ownership_and_key_uniqueness() {
        let mut first = decl(1, vec![1, 2], Shape::Callable);
        first.body = vec![rows(3..=3)];
        let second = decl(5, vec![5], Shape::Callable);
        let mut container = decl(1, vec![1, 7], Shape::Whole);
        container.body = vec![rows(1..=1), rows(4..=4), rows(5..=5)];
        container.members = vec![second, first];
        let batches = emit(
            8,
            FileModel {
                decls: vec![container],
                ..FileModel::default()
            },
        );
        let keys: HashSet<_> = batches.iter().map(|(key, _, _)| key.clone()).collect();
        assert_eq!(keys.len(), batches.len());
        let find = |rung, decl| {
            batches
                .iter()
                .find(|(key, _, _)| key.rung == rung && key.decl == decl)
                .unwrap_or_else(|| panic!("no {rung:?} for decl {decl}"))
        };
        let container_decl = &find(Rung::Decl, 1).0;
        assert_eq!(find(Rung::Decl, 1).2, [1, 4, 5, 7]);
        assert_eq!(find(Rung::Decl, 2).1.as_ref(), Some(container_decl));
        assert_eq!(find(Rung::Decl, 2).0.line, container_decl.line);
        assert_eq!(
            find(Rung::Body, 2).1.as_ref().map(short),
            Some((Rung::Decl, 2, 0))
        );
        // The one-row second member is covered by the container's roster.
        assert!(
            batches
                .iter()
                .all(|(key, _, _)| !(key.rung == Rung::Decl && key.decl == 3))
        );
    }

    #[test]
    fn emit_normalize_merges_same_row_decls_and_trims_at_next_sibling() {
        let mut spilling = decl(1, vec![1, 2, 3, 4], Shape::Whole);
        spilling.body = vec![rows(2..=3)];
        let mut spilling_member = decl(2, vec![2, 3, 4], Shape::Callable);
        spilling_member.doc = vec![rows(1..=1)];
        spilling.members = vec![spilling_member];
        let mut next = decl(4, vec![4], Shape::Whole);
        next.doc = vec![rows(3..=3)];
        let same_row = decl(3, vec![4], Shape::Whole);
        let model = with_file(4, |file| {
            normalize(
                FileModel {
                    decls: vec![next, spilling, same_row],
                    ..FileModel::default()
                },
                file,
            )
        });
        assert_eq!(model.decls.len(), 2);
        assert_eq!(model.decls[0].head, [1, 2]);
        assert!(model.decls[0].body.is_empty(), "head rows leave the body");
        assert_eq!(model.decls[0].members[0].head, [2]);
        assert_eq!(model.decls[0].members[0].doc, [rows(1..=1)]);
        assert_eq!(model.decls[1].name_rows, [3, 4]);
    }
}
