//! Scheduler invariants exercised via synthetic inline walkers. The
//! scheduler asserts these invariants in its own hot path; these tests
//! prove the assertions actually fire (and the happy-path cases work) by
//! constructing the relevant cases.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use precis::batch::{BatchKey, FsKey, ResolvedBatch, RustKey, ValueSignals};
use precis::content::{BatchContent, FsEntries, FsGroup, Render, Span};
use precis::render::SourceCache;
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

fn one_child(name: &str) -> FsEntries {
    FsEntries::Listed(vec![PathBuf::from(name)])
}

fn single_span(path: PathBuf, start: usize, end: usize, render: Render) -> Vec<Span> {
    vec![Span {
        path,
        start,
        end,
        render,
    }]
}

// Common stub paths. `/stub` doesn't exist; the synthetic walker answers
// every key's materialize directly, and each test preloads a `SourceCache`
// so the render pipeline materializes spans without touching the real FS.
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

fn preload(cache: &SourceCache, path: &Path, contents: &str) {
    cache.insert(path.to_path_buf(), Arc::from(contents));
}

#[test]
fn scheduler_invariants_override_via_predecessor_chain() {
    // Three batches: a folder listing → a `PubItem` carrying Truncated
    // spans → a `PubItemDocLede` refinement (with PubItem as predecessor)
    // that overrides line 1 with its Full version.
    struct OverrideChain;
    impl Walker for OverrideChain {
        type Key = BatchKey;

        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
            vec![Candidate::new(listing_key(), sig(0.9))]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                let decls = BatchKey::Rust(RustKey::PubItem {
                    file: stub_file("synthetic.rs"),
                    start_line: 1,
                });
                vec![
                    Candidate::new(decls.clone(), sig(0.5)),
                    Candidate::new(
                        BatchKey::Rust(RustKey::PubItemDocLede {
                            file: stub_file("synthetic.rs"),
                            start_line: 1,
                        }),
                        sig(0.3),
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
                    content: BatchContent::Fs {
                        groups: vec![FsGroup {
                            parent: dir.clone(),
                            entries: one_child("synthetic.rs"),
                        }],
                    },
                    signals: sig(0.9),
                }),
                BatchKey::Rust(RustKey::PubItem { .. }) => Some(ResolvedBatch {
                    content: BatchContent::Lines {
                        spans: vec![
                            Span {
                                path: stub_file("synthetic.rs"),
                                start: 1,
                                end: 1,
                                render: Render::Truncated {
                                    pattern: r"^fn [a-z]+".into(),
                                },
                            },
                            Span {
                                path: stub_file("synthetic.rs"),
                                start: 2,
                                end: 2,
                                render: Render::Truncated {
                                    pattern: r"^fn [a-z]+".into(),
                                },
                            },
                        ],
                    },
                    signals: sig(0.5),
                }),
                BatchKey::Rust(RustKey::PubItemDocLede { .. }) => Some(ResolvedBatch {
                    content: BatchContent::Lines {
                        spans: single_span(stub_file("synthetic.rs"), 1, 1, Render::Full),
                    },
                    signals: sig(0.3),
                }),
                _ => None,
            }
        }
    }

    let cache = SourceCache::new();
    preload(
        &cache,
        &stub_file("synthetic.rs"),
        "fn foo(a: i32, b: i32) -> Result<()> {\nfn bar(c: i32) -> Result<()> {\n",
    );
    let scheduler = Scheduler::with_source_cache(stub_dir(), OverrideChain, 100_000, None, cache);
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
fn scheduler_invariants_tiny_budget_truncates_cleanly() {
    struct OneEntry;
    impl Walker for OneEntry {
        type Key = BatchKey;

        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
            vec![Candidate::new(listing_key(), sig(0.9))]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                vec![Candidate::new(
                    BatchKey::Rust(RustKey::PubItem {
                        file: stub_file("synthetic.rs"),
                        start_line: 1,
                    }),
                    sig(0.5),
                )]
            } else {
                Vec::new()
            }
        }
        fn materialize(&mut self, key: &BatchKey, _ctx: &WalkCtx) -> Option<ResolvedBatch> {
            match key {
                BatchKey::Fs(FsKey::DirListing { dir }) => Some(ResolvedBatch {
                    content: BatchContent::Fs {
                        groups: vec![FsGroup {
                            parent: dir.clone(),
                            entries: one_child("synthetic.rs"),
                        }],
                    },
                    signals: sig(0.9),
                }),
                BatchKey::Rust(RustKey::PubItem { .. }) => Some(ResolvedBatch {
                    content: BatchContent::Lines {
                        spans: vec![Span {
                            path: stub_file("synthetic.rs"),
                            start: 1,
                            end: 20,
                            render: Render::Full,
                        }],
                    },
                    signals: sig(0.5),
                }),
                _ => None,
            }
        }
    }

    let cache = SourceCache::new();
    let body: String = (0..20)
        .map(|_| "very long line that will not fit at one token\n")
        .collect();
    preload(&cache, &stub_file("synthetic.rs"), &body);
    let scheduler = Scheduler::with_source_cache(stub_dir(), OneEntry, 1, None, cache);
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
fn scheduler_invariants_non_predecessor_overlap_panics_in_debug() {
    // Two sibling Lines batches (no ancestor relation) target the same
    // line. Release compiles out the assert; this test only asserts debug.
    struct OverlappingWalker;
    impl Walker for OverlappingWalker {
        type Key = BatchKey;

        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
            vec![Candidate::new(listing_key(), sig(0.9))]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Candidate<BatchKey>> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                // Two siblings — neither has the other as predecessor.
                vec![
                    Candidate::new(
                        BatchKey::Rust(RustKey::PubItem {
                            file: stub_file("f.rs"),
                            start_line: 1,
                        }),
                        sig(0.5),
                    ),
                    Candidate::new(
                        BatchKey::Rust(RustKey::MethodSigs {
                            file: stub_file("f.rs"),
                        }),
                        sig(0.5),
                    ),
                ]
            } else {
                Vec::new()
            }
        }
        fn materialize(&mut self, key: &BatchKey, _ctx: &WalkCtx) -> Option<ResolvedBatch> {
            let mk = || ResolvedBatch {
                content: BatchContent::Lines {
                    spans: single_span(stub_file("f.rs"), 1, 1, Render::Full),
                },
                signals: sig(0.5),
            };
            match key {
                BatchKey::Fs(FsKey::DirListing { dir }) => Some(ResolvedBatch {
                    content: BatchContent::Fs {
                        groups: vec![FsGroup {
                            parent: dir.clone(),
                            entries: one_child("f.rs"),
                        }],
                    },
                    signals: sig(0.9),
                }),
                BatchKey::Rust(RustKey::PubItem { .. }) => Some(mk()),
                BatchKey::Rust(RustKey::MethodSigs { .. }) => Some(mk()),
                _ => None,
            }
        }
    }

    let cache = SourceCache::new();
    preload(&cache, &stub_file("f.rs"), "x\n");
    let scheduler =
        Scheduler::with_source_cache(stub_dir(), OverlappingWalker, 10_000, None, cache);
    let _ = scheduler.run();
}
