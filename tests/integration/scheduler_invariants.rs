//! Scheduler invariants exercised via synthetic inline walkers. The
//! scheduler asserts these invariants in its own hot path; these tests
//! prove the assertions actually fire (and the happy-path cases work) by
//! constructing the relevant cases.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use precis::batch::{Batch, BatchKey, CodeKey, FsKey, Rung};
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

fn fs_listing_batch(value: f64, child: &str) -> Batch {
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

fn code_key(rung: Rung, file: &str) -> BatchKey {
    BatchKey::Code(CodeKey {
        rung,
        file: stub_file(file),
        decl: 1,
        sub: 0,
        line: 1,
    })
}

fn preload(cache: &SourceCache, path: &Path, contents: &str) {
    cache.insert(path.to_path_buf(), Arc::from(contents));
}

#[test]
fn scheduler_invariants_override_via_predecessor_chain() {
    // Three batches: a folder listing → a code `Decl` carrying Truncated
    // spans → a `Doc` refinement (with the `Decl` as predecessor)
    // that overrides line 1 with its Full version.
    struct OverrideChain;
    impl Walker for OverrideChain {
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch> {
            vec![fs_listing_batch(900.0, "synthetic.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                let pub_item_key = code_key(Rung::Decl, "synthetic.rs");
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
                        key: code_key(Rung::Doc, "synthetic.rs"),
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
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch> {
            vec![fs_listing_batch(900.0, "synthetic.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                vec![Batch {
                    key: code_key(Rung::Decl, "synthetic.rs"),
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

#[test]
fn scheduler_invariants_unaffordable_batch_spends_the_rest_on_its_head() {
    struct ListingThenFile;
    impl Walker for ListingThenFile {
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch> {
            vec![fs_listing_batch(900.0, "big.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch> {
            if !matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                return Vec::new();
            }
            vec![Batch {
                key: code_key(Rung::Body, "big.rs"),
                predecessor: None,
                content: BatchContent::Lines {
                    spans: single_span(stub_file("big.rs"), 1, 40, Render::Full),
                },
                value: 500.0,
            }]
        }
    }

    let render_at = |budget: usize| {
        let cache = SourceCache::new();
        let body: String = (1..=40).map(|i| format!("let line_{i} = {i};\n")).collect();
        preload(&cache, &stub_file("big.rs"), &body);
        Scheduler::with_source_cache(stub_dir(), ListingThenFile, budget, None, cache)
            .run()
            .render()
    };
    let whole = render_at(10_000);
    let budget = precis::tokenizer::count(&whole) / 2;
    let partial = render_at(budget);
    assert!(partial.contains("1→let line_1 = 1;"), "{partial}");
    assert!(!partial.contains("40→"), "{partial}");
    assert!(
        partial.ends_with("…\n"),
        "a cut-short file must say so: {partial}"
    );
    assert!(precis::tokenizer::count(&partial) <= budget, "{partial}");
    for row in partial.lines().filter(|row| row.trim() != "…") {
        assert!(whole.lines().any(|whole_row| whole_row == row), "{row}");
    }
}

/// Walker that emits one batch: the listing of `.0`, exactly as
/// `walker::fs` builds it, and nothing else.
struct RootListing(PathBuf);

impl Walker for RootListing {
    fn seed(&mut self, ctx: &WalkCtx) -> Vec<Batch> {
        let dir = self.0.clone();
        vec![Batch {
            key: BatchKey::Fs(FsKey::DirListing { dir: dir.clone() }),
            predecessor: None,
            content: BatchContent::Fs {
                groups: vec![FsGroup {
                    // Bare names, matching what `walker::fs` emits.
                    entries: FsEntries::Listed(
                        precis::fs_util::list_dir(&dir, ctx.dir_filter())
                            .keys()
                            .map(PathBuf::from)
                            .collect(),
                    ),
                    parent: dir,
                }],
            },
            value: 900.0,
        }]
    }
    fn expand(&mut self, _scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch> {
        Vec::new()
    }
}

#[test]
fn scheduler_invariants_unaffordable_seed_listing_degrades_to_a_marked_prefix() {
    // Every batch a walker emits is gated on the seed listing, so a seed
    // too big for the budget used to leave the schedule empty and the
    // caller with an empty string.
    const ENTRIES: usize = 60;
    const BUDGET: usize = 100;
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_path_buf();
    for i in 0..ENTRIES {
        std::fs::write(root.join(format!("entry_{i:03}.txt")), "x").unwrap();
    }

    let scheduler = Scheduler::new(root.clone(), RootListing(root), BUDGET, None);
    let rendered = scheduler.run().render();
    let listed = rendered.lines().filter(|l| l.contains("entry_")).count();
    assert!(listed > 0, "seed listing degraded to nothing: {rendered:?}");
    assert!(
        listed < ENTRIES,
        "whole listing fit — budget no longer exercises the prefix path",
    );
    assert!(
        precis::tokenizer::count(&rendered) <= BUDGET,
        "partial listing broke the budget: {rendered:?}",
    );
    assert!(
        rendered.ends_with("…\n"),
        "a listing cut short must say so: {rendered:?}",
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
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch> {
            vec![fs_listing_batch(900.0, "f.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                let line_content = || BatchContent::Lines {
                    spans: single_span(stub_file("f.rs"), 1, 1, Render::Full),
                };
                vec![
                    Batch {
                        key: code_key(Rung::Decl, "f.rs"),
                        predecessor: None,
                        content: line_content(),
                        value: 500.0,
                    },
                    Batch {
                        key: code_key(Rung::Names, "f.rs"),
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
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch> {
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

        fn expand(&mut self, _scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch> {
            Vec::new()
        }
    }

    let cache = SourceCache::new();
    let scheduler = Scheduler::with_source_cache(stub_dir(), OverlapWalker, 10_000, None, cache);
    let _ = scheduler.run();
}

#[test]
fn scheduler_invariants_dependent_absorbed_before_predecessor() {
    // Predecessor keys are symbolic: a walker may emit a dependent
    // before the batch that owns the predecessor key (absorb order
    // here: doc first, its `Decl` predecessor second).
    struct DependentFirst;
    impl Walker for DependentFirst {
        fn seed(&mut self, _ctx: &WalkCtx) -> Vec<Batch> {
            vec![fs_listing_batch(900.0, "synthetic.rs")]
        }
        fn expand(&mut self, scheduled: &BatchKey, _ctx: &WalkCtx) -> Vec<Batch> {
            if matches!(scheduled, BatchKey::Fs(FsKey::DirListing { .. })) {
                let pub_item_key = code_key(Rung::Decl, "synthetic.rs");
                vec![
                    Batch {
                        key: code_key(Rung::Doc, "synthetic.rs"),
                        predecessor: Some(pub_item_key.clone()),
                        content: BatchContent::Lines {
                            spans: single_span(stub_file("synthetic.rs"), 2, 2, Render::Full),
                        },
                        value: 300.0,
                    },
                    Batch {
                        key: pub_item_key,
                        predecessor: None,
                        content: BatchContent::Lines {
                            spans: single_span(stub_file("synthetic.rs"), 1, 1, Render::Full),
                        },
                        value: 500.0,
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
        "fn foo() {}\n// doc line\n",
    );
    let scheduler = Scheduler::with_source_cache(stub_dir(), DependentFirst, 100_000, None, cache);
    let tree = scheduler.run();
    let rendered = tree.render();
    assert!(rendered.contains("fn foo() {}"), "rendered: {rendered}");
    assert!(rendered.contains("// doc line"), "rendered: {rendered}");
}
