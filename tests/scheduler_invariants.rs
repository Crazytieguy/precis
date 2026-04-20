//! Stage 6 scheduler invariants exercised in language-agnostic terms via
//! synthetic walkers defined inline. The scheduler asserts these invariants
//! in its own hot path; these tests prove the assertions actually fire by
//! constructing the relevant cases.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

use precis::batch::{Batch, BatchContent, BatchDraft, BatchId, EntryKind, RenderedLine};
use precis::scheduler::Scheduler;
use precis::walker::{Walker, WalkerCtx};

fn one_file(name: &str, kind: EntryKind) -> BTreeMap<OsString, EntryKind> {
    let mut m = BTreeMap::new();
    m.insert(OsString::from(name), kind);
    m
}

fn line_set(
    path: PathBuf,
    lines: Vec<(usize, RenderedLine)>,
) -> BTreeMap<PathBuf, BTreeMap<usize, RenderedLine>> {
    let mut inner = BTreeMap::new();
    for (n, l) in lines {
        inner.insert(n, l);
    }
    let mut outer = BTreeMap::new();
    outer.insert(path, inner);
    outer
}

#[test]
fn override_via_predecessor_chain() {
    // Three batches: a folder listing → a file's truncated lines → a full line
    // for the same line number (overrides the truncated one).
    struct OverrideChain;
    impl Walker for OverrideChain {
        fn seed(&mut self, ctx: &WalkerCtx) -> Vec<BatchDraft> {
            vec![BatchDraft {
                content: BatchContent::FileSystemEntries {
                    parent: ctx.root().to_path_buf(),
                    children: one_file("synthetic.rs", EntryKind::File),
                },
                value: 100.0,
            }]
        }
        fn successors(
            &mut self,
            _id: BatchId,
            scheduled: &Batch,
            ctx: &WalkerCtx,
        ) -> Vec<BatchDraft> {
            match &scheduled.content {
                BatchContent::FileSystemEntries { .. } => {
                    let path = ctx.root().join("synthetic.rs");
                    vec![BatchDraft {
                        content: BatchContent::Lines(line_set(
                            path,
                            vec![
                                (1, RenderedLine::Truncated("fn foo".into())),
                                (2, RenderedLine::Truncated("fn bar".into())),
                            ],
                        )),
                        value: 50.0,
                    }]
                }
                BatchContent::Lines(file_map)
                    if file_map
                        .values()
                        .any(|m| m.values().any(|l| l.is_truncated())) =>
                {
                    let path = ctx.root().join("synthetic.rs");
                    vec![BatchDraft {
                        content: BatchContent::Lines(line_set(
                            path,
                            vec![(
                                1,
                                RenderedLine::Full("fn foo(a: i32, b: i32) -> Result<()> {".into()),
                            )],
                        )),
                        value: 30.0,
                    }]
                }
                _ => vec![],
            }
        }
    }

    let scheduler = Scheduler::new(PathBuf::from("/stub"), OverrideChain, 100_000, None);
    let tree = scheduler.run();
    let rendered = tree.render();
    assert!(rendered.contains("synthetic.rs"), "rendered: {rendered}");
    assert!(rendered.contains("fn bar"), "rendered: {rendered}");
    assert!(rendered.contains('…'), "rendered: {rendered}");
    assert!(
        rendered.contains("fn foo(a: i32, b: i32) -> Result<()> {"),
        "rendered: {rendered}",
    );
}

#[test]
fn tiny_budget_truncates_cleanly() {
    struct OneEntry;
    impl Walker for OneEntry {
        fn seed(&mut self, ctx: &WalkerCtx) -> Vec<BatchDraft> {
            vec![BatchDraft {
                content: BatchContent::FileSystemEntries {
                    parent: ctx.root().to_path_buf(),
                    children: one_file("synthetic.rs", EntryKind::File),
                },
                value: 100.0,
            }]
        }
        fn successors(
            &mut self,
            _id: BatchId,
            scheduled: &Batch,
            ctx: &WalkerCtx,
        ) -> Vec<BatchDraft> {
            if matches!(scheduled.content, BatchContent::FileSystemEntries { .. }) {
                let path = ctx.root().join("synthetic.rs");
                let lines = (1..=20)
                    .map(|n| {
                        (
                            n,
                            RenderedLine::Full(
                                "very long line that will not fit at one token".into(),
                            ),
                        )
                    })
                    .collect::<Vec<_>>();
                vec![BatchDraft {
                    content: BatchContent::Lines(line_set(path, lines)),
                    value: 50.0,
                }]
            } else {
                vec![]
            }
        }
    }

    let scheduler = Scheduler::new(PathBuf::from("/stub"), OneEntry, 1, None);
    let tree = scheduler.run();
    let rendered = tree.render();
    assert!(
        !rendered.contains("very long line"),
        "tiny budget shouldn't fit content"
    );
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "non-ancestor overlap")]
fn non_predecessor_overlap_panics_in_debug() {
    // Two Lines batches with no ancestor relation that target the same line.
    // In release builds the assert is compiled out and the second write
    // silently overrides; this test only asserts the debug behavior.
    struct OverlappingWalker;
    impl Walker for OverlappingWalker {
        fn seed(&mut self, ctx: &WalkerCtx) -> Vec<BatchDraft> {
            vec![BatchDraft {
                content: BatchContent::FileSystemEntries {
                    parent: ctx.root().to_path_buf(),
                    children: one_file("f.rs", EntryKind::File),
                },
                value: 100.0,
            }]
        }
        fn successors(
            &mut self,
            _id: BatchId,
            scheduled: &Batch,
            ctx: &WalkerCtx,
        ) -> Vec<BatchDraft> {
            if !matches!(scheduled.content, BatchContent::FileSystemEntries { .. }) {
                return vec![];
            }
            let path = ctx.root().join("f.rs");
            vec![
                BatchDraft {
                    content: BatchContent::Lines(line_set(
                        path.clone(),
                        vec![(1, RenderedLine::Full("x".into()))],
                    )),
                    value: 50.0,
                },
                BatchDraft {
                    content: BatchContent::Lines(line_set(
                        path,
                        vec![(1, RenderedLine::Full("y".into()))],
                    )),
                    value: 50.0,
                },
            ]
        }
    }

    let scheduler = Scheduler::new(PathBuf::from("/synth"), OverlappingWalker, 10_000, None);
    let _ = scheduler.run();
}
