//! Stage 6 scheduler invariants exercised in language-agnostic terms via
//! synthetic walkers defined inline. The scheduler asserts these invariants
//! in its own hot path; these tests prove the assertions actually fire by
//! constructing the relevant cases.

use std::path::PathBuf;

use precis::batch::{Batch, BatchContent, FileLineSet, FsEntry, RenderedLine};
use precis::scheduler::Scheduler;
use precis::walker::{Walker, WalkerCtx};

#[test]
fn override_via_predecessor_chain() {
    // Three batches: a folder listing → a file's truncated lines → a full line
    // for the same line number (overrides the truncated one). All connected
    // by a predecessor chain. Override must succeed and the rendered output
    // must show the full form.
    struct OverrideChain;
    impl Walker for OverrideChain {
        fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch> {
            vec![Batch {
                id: ctx.alloc_id(),
                content: BatchContent::FileSystemEntries(vec![FsEntry {
                    path: PathBuf::from("/stub/synthetic.rs"),
                    is_dir: false,
                }]),
                predecessor: None,
                value: 100.0,
            }]
        }
        fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
            match &scheduled.content {
                BatchContent::FileSystemEntries(_) => vec![Batch {
                    id: ctx.alloc_id(),
                    content: BatchContent::Lines(vec![FileLineSet {
                        path: PathBuf::from("/stub/synthetic.rs"),
                        lines: vec![
                            RenderedLine {
                                number: 1,
                                text: "fn foo".into(),
                                truncated: true,
                            },
                            RenderedLine {
                                number: 2,
                                text: "fn bar".into(),
                                truncated: true,
                            },
                        ],
                    }]),
                    predecessor: Some(scheduled.id),
                    value: 50.0,
                }],
                BatchContent::Lines(sets) if sets[0].lines.iter().any(|l| l.truncated) => {
                    vec![Batch {
                        id: ctx.alloc_id(),
                        content: BatchContent::Lines(vec![FileLineSet {
                            path: PathBuf::from("/stub/synthetic.rs"),
                            lines: vec![RenderedLine {
                                number: 1,
                                text: "fn foo(a: i32, b: i32) -> Result<()> {".into(),
                                truncated: false,
                            }],
                        }]),
                        predecessor: Some(scheduled.id),
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
        fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch> {
            vec![Batch {
                id: ctx.alloc_id(),
                content: BatchContent::FileSystemEntries(vec![FsEntry {
                    path: PathBuf::from("/stub/synthetic.rs"),
                    is_dir: false,
                }]),
                predecessor: None,
                value: 100.0,
            }]
        }
        fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
            // Always emits one large content batch. At a 1-token budget the
            // listing fits but the content batch doesn't.
            if matches!(scheduled.content, BatchContent::FileSystemEntries(_)) {
                vec![Batch {
                    id: ctx.alloc_id(),
                    content: BatchContent::Lines(vec![FileLineSet {
                        path: PathBuf::from("/stub/synthetic.rs"),
                        lines: (1..=20)
                            .map(|n| RenderedLine {
                                number: n,
                                text: "very long line that will not fit at one token".into(),
                                truncated: false,
                            })
                            .collect(),
                    }]),
                    predecessor: Some(scheduled.id),
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

#[test]
#[should_panic(expected = "non-ancestor overlap")]
fn non_predecessor_overlap_panics_in_debug() {
    // Two Lines batches with no ancestor relation that target the same line.
    struct OverlappingWalker;
    impl Walker for OverlappingWalker {
        fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch> {
            vec![Batch {
                id: ctx.alloc_id(),
                content: BatchContent::FileSystemEntries(vec![FsEntry {
                    path: PathBuf::from("/synth/f.rs"),
                    is_dir: false,
                }]),
                predecessor: None,
                value: 100.0,
            }]
        }
        fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
            if !matches!(scheduled.content, BatchContent::FileSystemEntries(_)) {
                return vec![];
            }
            let line = || RenderedLine {
                number: 1,
                text: "x".into(),
                truncated: false,
            };
            vec![
                Batch {
                    id: ctx.alloc_id(),
                    content: BatchContent::Lines(vec![FileLineSet {
                        path: PathBuf::from("/synth/f.rs"),
                        lines: vec![line()],
                    }]),
                    predecessor: Some(scheduled.id),
                    value: 50.0,
                },
                Batch {
                    id: ctx.alloc_id(),
                    content: BatchContent::Lines(vec![FileLineSet {
                        path: PathBuf::from("/synth/f.rs"),
                        lines: vec![RenderedLine {
                            number: 1,
                            text: "y".into(),
                            truncated: false,
                        }],
                    }]),
                    predecessor: Some(scheduled.id),
                    value: 50.0,
                },
            ]
        }
    }

    let scheduler = Scheduler::new(PathBuf::from("/synth"), OverlappingWalker, 10_000, None);
    let _ = scheduler.run();
}
