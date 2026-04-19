//! Stage 6 scheduler invariants exercised in language-agnostic terms via the
//! StubWalker (and one targeted synthetic walker for non-predecessor overlap).
//!
//! The scheduler asserts these invariants in its own hot path; these tests
//! prove the assertions actually fire by constructing the relevant cases.

use std::path::PathBuf;

use precis::batch::{Batch, BatchContent, FsEntry, RenderedLine};
use precis::scheduler::Scheduler;
use precis::walker::stub::StubWalker;
use precis::walker::{Walker, WalkerCtx};

#[test]
fn stub_walker_at_large_budget_schedules_all_and_overrides_line() {
    let scheduler = Scheduler::new(PathBuf::from("/stub"), StubWalker, 100_000, None);
    let tree = scheduler.run();
    let rendered = tree.render();

    // Folder listing produced the synthetic file.
    assert!(rendered.contains("synthetic.rs"), "rendered: {rendered}");

    // Truncated form of `bar` survives (no override of line 2).
    assert!(rendered.contains("fn bar"), "rendered: {rendered}");
    assert!(rendered.contains('…'), "rendered: {rendered}");

    // Override applied to line 1: full signature replaces the truncated `fn foo`.
    assert!(
        rendered.contains("fn foo(a: i32, b: i32) -> Result<()> {"),
        "rendered: {rendered}",
    );
}

#[test]
fn stub_walker_at_tiny_budget_fits_only_seed() {
    // 1 token can fit only the empty seed (no batch lines).
    let scheduler = Scheduler::new(PathBuf::from("/stub"), StubWalker, 1, None);
    let tree = scheduler.run();
    let rendered = tree.render();
    // The seed itself is the folder listing; at 1 token even the listing's
    // single entry might not fit. Either way we should see no override line.
    assert!(
        !rendered.contains("fn foo(a: i32"),
        "tiny budget shouldn't fit override line"
    );
}

#[test]
#[should_panic(expected = "non-predecessor overlap")]
fn non_predecessor_overlap_panics() {
    // Two file-content batches with no ancestor relation that target the same
    // line. The seed listing contains the file, then two siblings (each a child
    // of the seed) both write to line 1 — the second triggers the assert.
    struct OverlappingWalker;
    impl Walker for OverlappingWalker {
        fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch> {
            vec![Batch {
                id: ctx.alloc_id(),
                descriptor: "seed".into(),
                content: BatchContent::FolderListing {
                    path: PathBuf::from("/synth"),
                    entries: vec![FsEntry {
                        name: "f.rs".into(),
                        is_dir: false,
                    }],
                },
                parent: None,
                ordering_pred: None,
                raw_value: 100.0,
            }]
        }
        fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
            if scheduled.descriptor != "seed" {
                return vec![];
            }
            // Two sibling batches — both children of seed, neither an ancestor
            // of the other — that write to the same line.
            let line = || RenderedLine {
                number: 1,
                text: "x".into(),
                truncated: false,
            };
            vec![
                Batch {
                    id: ctx.alloc_id(),
                    descriptor: "first writer".into(),
                    content: BatchContent::FileContent {
                        path: PathBuf::from("/synth/f.rs"),
                        lines: vec![line()],
                    },
                    parent: Some(scheduled.id),
                    ordering_pred: None,
                    raw_value: 50.0,
                },
                Batch {
                    id: ctx.alloc_id(),
                    descriptor: "second writer".into(),
                    content: BatchContent::FileContent {
                        path: PathBuf::from("/synth/f.rs"),
                        lines: vec![RenderedLine {
                            number: 1,
                            text: "y".into(),
                            truncated: false,
                        }],
                    },
                    parent: Some(scheduled.id),
                    ordering_pred: None,
                    raw_value: 50.0,
                },
            ]
        }
    }

    let scheduler = Scheduler::new(PathBuf::from("/synth"), OverlappingWalker, 10_000, None);
    let _ = scheduler.run(); // expected to panic during the second write
}

#[test]
fn override_via_parent_chain_does_not_panic() {
    // A 3-batch chain: folder → truncated lines → full line. The full-line batch
    // (child of truncated) overriding line 1 must not panic — its predecessor
    // owns the line being replaced.
    let scheduler = Scheduler::new(PathBuf::from("/stub"), StubWalker, 100_000, None);
    let _ = scheduler.run(); // panicking would fail the test
}

#[test]
fn override_via_ordering_predecessor_does_not_panic() {
    // A and B are siblings under a folder. B has ordering_pred = A and
    // overrides line 1 that A wrote. The overlap check must treat ordering
    // predecessors as legal owners, not just structural parents.
    struct OrderedSiblings;
    impl Walker for OrderedSiblings {
        fn seed(&mut self, ctx: &mut WalkerCtx) -> Vec<Batch> {
            vec![Batch {
                id: ctx.alloc_id(),
                descriptor: "seed".into(),
                content: BatchContent::FolderListing {
                    path: PathBuf::from("/synth"),
                    entries: vec![FsEntry {
                        name: "f.rs".into(),
                        is_dir: false,
                    }],
                },
                parent: None,
                ordering_pred: None,
                raw_value: 100.0,
            }]
        }
        fn successors(&mut self, scheduled: &Batch, ctx: &mut WalkerCtx) -> Vec<Batch> {
            if scheduled.descriptor != "seed" {
                return vec![];
            }
            let a_id = ctx.alloc_id();
            let b_id = ctx.alloc_id();
            vec![
                Batch {
                    id: a_id,
                    descriptor: "A: writes line 1".into(),
                    content: BatchContent::FileContent {
                        path: PathBuf::from("/synth/f.rs"),
                        lines: vec![RenderedLine {
                            number: 1,
                            text: "fn foo".into(),
                            truncated: true,
                        }],
                    },
                    parent: Some(scheduled.id),
                    ordering_pred: None,
                    raw_value: 50.0,
                },
                Batch {
                    id: b_id,
                    descriptor: "B: overrides line 1 after A".into(),
                    content: BatchContent::FileContent {
                        path: PathBuf::from("/synth/f.rs"),
                        lines: vec![RenderedLine {
                            number: 1,
                            text: "fn foo() -> i32 {".into(),
                            truncated: false,
                        }],
                    },
                    parent: Some(scheduled.id),
                    ordering_pred: Some(a_id),
                    raw_value: 40.0,
                },
            ]
        }
    }

    let scheduler = Scheduler::new(PathBuf::from("/synth"), OrderedSiblings, 10_000, None);
    let tree = scheduler.run();
    let rendered = tree.render();
    assert!(
        rendered.contains("fn foo() -> i32 {"),
        "ordered override should land: {rendered}"
    );
}
