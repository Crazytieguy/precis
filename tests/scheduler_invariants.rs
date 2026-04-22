//! Scheduler invariants exercised via synthetic inline walkers. The
//! scheduler asserts these invariants in its own hot path; these tests
//! prove the assertions actually fire (and the happy-path cases work) by
//! constructing the relevant cases.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use precis::batch::{
    BatchContent, BatchKey, EntryKind, FsKey, RenderedLine, ResolvedBatch, RustKey, ValueSignals,
};
use precis::scheduler::Scheduler;
use precis::walker::{Candidate, WalkCtx, Walker};

fn sig(n: f64) -> ValueSignals {
    ValueSignals {
        catastrophic_omission: n,
        follow_up_minimization: n,
        zero_tool_call_understanding: n,
        depth_factor: 1.0,
    }
}

fn one_child(name: &str, kind: EntryKind) -> BTreeMap<OsString, EntryKind> {
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

// Common stub paths. `/stub` doesn't exist; the synthetic walker answers
// every key's materialize directly, so the scheduler never touches the
// real filesystem.
const STUB_DIR: &str = "/stub";

fn stub_dir() -> PathBuf {
    PathBuf::from(STUB_DIR)
}

fn stub_file(name: &str) -> PathBuf {
    Path::new(STUB_DIR).join(name)
}

fn listing_key() -> BatchKey {
    BatchKey::Fs(FsKey::DirListing { dir: stub_dir() })
}

#[test]
fn override_via_predecessor_chain() {
    // Three batches: a folder listing → a `PubDecls` carrying truncated
    // lines → a `PubDocs` (refinement, with PubDecls as predecessor) that
    // overrides line 1 with its full version.
    struct OverrideChain;
    impl Walker for OverrideChain {
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Candidate> {
            vec![Candidate::new(listing_key(), sig(0.9), 40)]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Candidate> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                let decls = BatchKey::Rust(RustKey::PubItem {
                    file: stub_file("synthetic.rs"),
                    start_line: 1,
                });
                vec![
                    Candidate::new(decls.clone(), sig(0.5), 50),
                    Candidate::new(
                        BatchKey::Rust(RustKey::PubItemDoc {
                            file: stub_file("synthetic.rs"),
                            start_line: 1,
                        }),
                        sig(0.3),
                        50,
                    )
                    .with_predecessor(decls),
                ]
            } else {
                Vec::new()
            }
        }
        fn materialize(&mut self, key: &BatchKey, _ctx: &WalkCtx) -> Option<ResolvedBatch> {
            match key {
                BatchKey::Fs(FsKey::DirListing { dir }) => Some(ResolvedBatch {
                    content: BatchContent::FileSystemEntries {
                        parent: dir.clone(),
                        children: one_child("synthetic.rs", EntryKind::File),
                    },
                    signals: sig(0.9),
                }),
                BatchKey::Rust(RustKey::PubItem { .. }) => Some(ResolvedBatch {
                    content: BatchContent::Lines(line_set(
                        stub_file("synthetic.rs"),
                        vec![
                            (1, RenderedLine::Truncated("fn foo".into())),
                            (2, RenderedLine::Truncated("fn bar".into())),
                        ],
                    )),
                    signals: sig(0.5),
                }),
                BatchKey::Rust(RustKey::PubItemDoc { .. }) => Some(ResolvedBatch {
                    content: BatchContent::Lines(line_set(
                        stub_file("synthetic.rs"),
                        vec![(
                            1,
                            RenderedLine::Full("fn foo(a: i32, b: i32) -> Result<()> {".into()),
                        )],
                    )),
                    signals: sig(0.3),
                }),
                _ => None,
            }
        }
    }

    let scheduler = Scheduler::new(stub_dir(), OverrideChain, 100_000, None);
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
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Candidate> {
            vec![Candidate::new(listing_key(), sig(0.9), 40)]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Candidate> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                vec![Candidate::new(
                    BatchKey::Rust(RustKey::PubItem {
                        file: stub_file("synthetic.rs"),
                        start_line: 1,
                    }),
                    sig(0.5),
                    50,
                )]
            } else {
                Vec::new()
            }
        }
        fn materialize(&mut self, key: &BatchKey, _ctx: &WalkCtx) -> Option<ResolvedBatch> {
            match key {
                BatchKey::Fs(FsKey::DirListing { dir }) => Some(ResolvedBatch {
                    content: BatchContent::FileSystemEntries {
                        parent: dir.clone(),
                        children: one_child("synthetic.rs", EntryKind::File),
                    },
                    signals: sig(0.9),
                }),
                BatchKey::Rust(RustKey::PubItem { .. }) => {
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
                    Some(ResolvedBatch {
                        content: BatchContent::Lines(line_set(
                            stub_file("synthetic.rs"),
                            lines,
                        )),
                        signals: sig(0.5),
                    })
                }
                _ => None,
            }
        }
    }

    let scheduler = Scheduler::new(stub_dir(), OneEntry, 1, None);
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
    // Two sibling Lines batches (no ancestor relation) target the same
    // line. Release compiles out the assert; this test only asserts debug.
    struct OverlappingWalker;
    impl Walker for OverlappingWalker {
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Candidate> {
            vec![Candidate::new(listing_key(), sig(0.9), 40)]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Candidate> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                // Two siblings — neither has the other as predecessor.
                vec![
                    Candidate::new(
                        BatchKey::Rust(RustKey::PubItem {
                            file: stub_file("synthetic.rs"),
                            start_line: 1,
                        }),
                        sig(0.5),
                        20,
                    ),
                    Candidate::new(
                        BatchKey::Rust(RustKey::MethodSigs {
                            file: stub_file("synthetic.rs"),
                        }),
                        sig(0.5),
                        20,
                    ),
                ]
            } else {
                Vec::new()
            }
        }
        fn materialize(&mut self, key: &BatchKey, _ctx: &WalkCtx) -> Option<ResolvedBatch> {
            let mk = |text: &str| ResolvedBatch {
                content: BatchContent::Lines(line_set(
                    stub_file("f.rs"),
                    vec![(1, RenderedLine::Full(text.into()))],
                )),
                signals: sig(0.5),
            };
            match key {
                BatchKey::Fs(FsKey::DirListing { dir }) => Some(ResolvedBatch {
                    content: BatchContent::FileSystemEntries {
                        parent: dir.clone(),
                        children: one_child("f.rs", EntryKind::File),
                    },
                    signals: sig(0.9),
                }),
                BatchKey::Rust(RustKey::PubItem { .. }) => Some(mk("x")),
                BatchKey::Rust(RustKey::MethodSigs { .. }) => Some(mk("y")),
                _ => None,
            }
        }
    }

    let scheduler = Scheduler::new(stub_dir(), OverlappingWalker, 10_000, None);
    let _ = scheduler.run();
}
