//! Scheduler invariants exercised via synthetic inline walkers. The
//! scheduler asserts these invariants in its own hot path; these tests
//! prove the assertions actually fire (and the happy-path cases work) by
//! constructing the relevant cases.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use precis::batch::{Batch, BatchKey, FsKey, RustKey};
use precis::content::{BatchContent, FsEntries, FsGroup, Render, Span};
use precis::render::SourceCache;
use precis::scheduler::Scheduler;
use precis::walker::{WalkCtx, Walker};

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

// Common stub paths. `/stub` doesn't exist; the synthetic walker emits
// every batch directly, and each test preloads a `SourceCache` so the
// render pipeline materializes spans without touching the real FS.
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

fn fs_listing_batch(value: f64, child: &str) -> Batch<BatchKey> {
    Batch {
        key: listing_key(),
        predecessor: None,
        content: BatchContent::Fs {
            groups: vec![FsGroup {
                parent: stub_dir(),
                entries: one_child(child),
            }],
        },
        value,
    }
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

        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            vec![fs_listing_batch(900.0, "synthetic.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                let pub_item_key = BatchKey::Rust(RustKey::PubItem {
                    file: stub_file("synthetic.rs"),
                    start_line: 1,
                });
                vec![
                    Batch {
                        key: pub_item_key.clone(),
                        predecessor: None,
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
                        value: 500.0,
                    },
                    Batch {
                        key: BatchKey::Rust(RustKey::PubItemDocLede {
                            file: stub_file("synthetic.rs"),
                            start_line: 1,
                        }),
                        predecessor: Some(pub_item_key),
                        content: BatchContent::Lines {
                            spans: single_span(stub_file("synthetic.rs"), 1, 1, Render::Full),
                        },
                        value: 300.0,
                    },
                ]
            } else {
                Vec::new()
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

        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            vec![fs_listing_batch(900.0, "synthetic.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                vec![Batch {
                    key: BatchKey::Rust(RustKey::PubItem {
                        file: stub_file("synthetic.rs"),
                        start_line: 1,
                    }),
                    predecessor: None,
                    content: BatchContent::Lines {
                        spans: vec![Span {
                            path: stub_file("synthetic.rs"),
                            start: 1,
                            end: 20,
                            render: Render::Full,
                        }],
                    },
                    value: 500.0,
                }]
            } else {
                Vec::new()
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

        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            vec![fs_listing_batch(900.0, "f.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                let line_content = || BatchContent::Lines {
                    spans: single_span(stub_file("f.rs"), 1, 1, Render::Full),
                };
                vec![
                    Batch {
                        key: BatchKey::Rust(RustKey::PubItem {
                            file: stub_file("f.rs"),
                            start_line: 1,
                        }),
                        predecessor: None,
                        content: line_content(),
                        value: 500.0,
                    },
                    Batch {
                        key: BatchKey::Rust(RustKey::MethodSigs {
                            file: stub_file("f.rs"),
                        }),
                        predecessor: None,
                        content: line_content(),
                        value: 500.0,
                    },
                ]
            } else {
                Vec::new()
            }
        }
    }

    let cache = SourceCache::new();
    preload(&cache, &stub_file("f.rs"), "x\n");
    let scheduler =
        Scheduler::with_source_cache(stub_dir(), OverlappingWalker, 10_000, None, cache);
    let _ = scheduler.run();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "overlapping FS atom")]
fn scheduler_invariants_overlapping_fs_atoms_panic_in_debug() {
    // Two sibling DirListing batches share `/stub` as parent and both
    // claim `a.rs`. The production FS walker emits one full listing per
    // directory, so overlapping FS atoms are a walker-contract violation
    // rejected at absorb time rather than a scheduler-supported case.
    fn key_a() -> BatchKey {
        // Distinct directory paths so the keys hash differently —
        // both batches still emit groups parented at `/stub`, which is
        // the render cell whose ownership must be unique.
        BatchKey::Fs(FsKey::DirListing {
            dir: PathBuf::from("/stub-a"),
        })
    }
    fn key_b() -> BatchKey {
        BatchKey::Fs(FsKey::DirListing {
            dir: PathBuf::from("/stub-b"),
        })
    }

    struct OverlapWalker;
    impl Walker for OverlapWalker {
        type Key = BatchKey;

        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            vec![
                Batch {
                    key: key_a(),
                    predecessor: None,
                    content: BatchContent::Fs {
                        groups: vec![FsGroup {
                            parent: stub_dir(),
                            entries: FsEntries::Listed(vec![PathBuf::from("a.rs")]),
                        }],
                    },
                    value: 900.0,
                },
                Batch {
                    key: key_b(),
                    predecessor: None,
                    content: BatchContent::Fs {
                        groups: vec![FsGroup {
                            parent: stub_dir(),
                            entries: FsEntries::Listed(vec![
                                PathBuf::from("a.rs"),
                                PathBuf::from("b.rs"),
                            ]),
                        }],
                    },
                    value: 400.0,
                },
            ]
        }

        fn expand(&mut self, _scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch<BatchKey>> {
            Vec::new()
        }
    }

    let cache = SourceCache::new();
    let scheduler = Scheduler::with_source_cache(stub_dir(), OverlapWalker, 10_000, None, cache);
    let _ = scheduler.run();
}
