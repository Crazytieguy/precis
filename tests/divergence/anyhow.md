Score(3000)=0.676 I=0.875 C=0.522 ns_rows≤3K=18/46 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 61 | 27 | [features] in Cargo.toml |  |  | 1.000 |
| ns | 83 |  | 49 | src/ module listing | 1.2 |  | 0.606 |
| walker |  | 128 | 67 | README headline in README.md |  |  | 0.608 |
| ns | 173 |  | 90 | tests/ top-level listing | 1.3 |  | 0.419 |
| walker |  | 177 | 49 | listing of 'src' |  |  | 0.696 |
| walker |  | 185 | 8 | listing of '.github' |  |  | 0.696 |
| walker |  | 189 | 4 | listing of '.github/workflows' |  |  | 0.696 |
| ns | 277 |  | 104 | test-support subdirectory listings | 1.4 |  | 0.549 |
| walker |  | 279 | 90 | pub-item names surface in src/lib.rs |  |  | 0.550 |
| walker |  | 279 | 0 | pub item at src/lib.rs:468 |  |  | 0.550 |
| walker |  | 292 | 13 | pub item at src/lib.rs:650 |  |  | 0.550 |
| ns | 304 |  | 27 | Cargo.toml feature flags | 1.5 |  | 0.569 |
| walker |  | 318 | 26 | pub item at src/lib.rs:390 |  |  | 0.569 |
| walker |  | 347 | 29 | pub item at src/lib.rs:415 |  |  | 0.570 |
| walker |  | 357 | 10 | pub item body at src/lib.rs:650 body 651 |  |  | 0.570 |
| walker |  | 401 | 44 | headings outline in README.md |  |  | 0.570 |
| ns | 469 |  | 165 | Cargo.toml package identity | 1.6 |  | 0.512 |
| walker |  | 491 | 90 | listing of 'tests' |  |  | 0.721 |
| ns | 510 |  | 41 | README pitch sentence | 1.7 |  | 0.726 |
| ns | 620 |  | 110 | Cargo.toml dependencies | 1.8 |  | 0.694 |
| walker |  | 651 | 160 | pub item at src/lib.rs:616 |  |  | 0.695 |
| ns | 673 |  | 53 | Misc root config: toolchain, gitignore, funding, pin | 1.9 |  | 0.666 |
| walker |  | 816 | 165 | [package] in Cargo.toml |  |  | 0.764 |
| walker |  | 898 | 82 | macro_export names across src |  |  | 0.692 |
| ns | 898 |  | 225 | Cargo.toml dev-dependencies + docs.rs metadata | 1.10 |  | 0.692 |
| ns | 933 |  | 35 | README install snippet | 1.11 |  | 0.676 |
| walker |  | 972 | 74 | README.md section #0 |  |  | 0.703 |
| ns | 1071 |  | 138 | lib.rs module declarations + extern crate | 1.12 |  | 0.641 |
| walker |  | 1082 | 110 | [dependencies] in Cargo.toml |  |  | 0.678 |
| ns | 1291 |  | 220 | README no-std support section | 1.13 |  | 0.634 |
| walker |  | 1356 | 274 | crate-doc lede in src/lib.rs |  |  | 0.634 |
| walker |  | 1546 | 190 | mod/use plumbing in src/lib.rs |  |  | 0.665 |
| walker |  | 1643 | 97 | dev/build/target dependencies in Cargo.toml |  |  | 0.678 |
| ns | 1692 |  | 401 | lib.rs crate-level attributes | 1.14 |  | 0.607 |
| walker |  | 1766 | 123 | manifest config in Cargo.toml |  |  | 0.665 |
| ns | 1856 |  | 164 | README license section | 1.15 |  | 0.635 |
| walker |  | 1907 | 141 | impl method sigs in src/context.rs |  |  | 0.635 |
| ns | 1968 |  | 112 | lib.rs top-level public API roster | 2.1 |  | 0.645 |
| walker |  | 2308 | 401 | crate attributes in src/lib.rs |  |  | 0.751 |
| walker |  | 2331 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.751 |
| walker |  | 2335 | 4 | listing of 'tests/common' |  |  | 0.752 |
| ns | 2366 |  | 398 | error.rs Error impl method roster | 2.2 |  | 0.696 |
| walker |  | 2376 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.696 |
| walker |  | 2441 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.697 |
| walker |  | 2579 | 138 | macro_export body at src/macros.rs:58 |  |  | 0.698 |
| walker |  | 2753 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.698 |
| walker |  | 2757 | 4 | listing of 'tests/drop' |  |  | 0.699 |
| walker |  | 2839 | 82 | listing of 'tests/ui' |  |  | 0.754 |
| ns | 2971 |  | 605 | Rest-of-crate public/crate-visible item roster | 2.3 |  | 0.676 |
| walker |  | 3061 | 222 | macro_export body at src/macros.rs:204 |  |  | 0.678 |
| walker |  | 3098 | 37 | impl method sigs in src/ensure.rs |  |  | 0.678 |
| walker |  | 3098 | 0 | impl method at src/ensure.rs:41 |  |  | 0.678 |
| walker |  | 3098 | 0 | impl method at src/ensure.rs:48 |  |  | 0.678 |
| walker |  | 3496 | 398 | impl method sigs in src/ptr.rs |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:32 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:38 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:44 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:48 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:55 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:87 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:94 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:101 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:108 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:115 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:119 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:148 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:155 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:162 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:169 |  |  | 0.695 |
| walker |  | 3496 | 0 | impl method at src/ptr.rs:175 |  |  | 0.695 |
| walker |  | 3529 | 33 | pub-item names surface in src/chain.rs |  |  | 0.695 |
| walker |  | 3546 | 17 | pub item at src/chain.rs:11 |  |  | 0.695 |
| ns | 3703 |  | 732 | Context trait doc (prose, code examples elided) | 3.1 | 2.1 | 0.626 |
| ns | 4012 |  | 309 | macros.rs bail! doc + expansion | 3.2 |  | 0.611 |
| walker |  | 4226 | 680 | impl method sigs in src/error.rs |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:140 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:432 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:459 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:622 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:675 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:686 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:958 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:967 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:974 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:985 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:1002 |  |  | 0.671 |
| walker |  | 4226 | 0 | impl method at src/error.rs:1010 |  |  | 0.671 |
| walker |  | 4452 | 226 | pub-item doc lede at src/lib.rs:468 |  |  | 0.673 |
| ns | 4461 |  | 449 | macros.rs anyhow! doc + expansion | 3.3 |  | 0.654 |
| walker |  | 4486 | 34 | pub-item names surface in src/error.rs |  |  | 0.654 |
| walker |  | 4517 | 31 | pub item at src/error.rs:952 |  |  | 0.654 |
| ns | 4691 |  | 230 | macros.rs ensure! doc summary | 3.4 |  | 0.640 |
| walker |  | 4801 | 284 | pub-item doc lede at src/lib.rs:650 |  |  | 0.640 |
| walker |  | 4841 | 40 | impl method sigs in src/chain.rs |  |  | 0.641 |
| walker |  | 4841 | 0 | impl method at src/chain.rs:28 |  |  | 0.641 |
| ns | 5275 |  | 584 | kind.rs tagged-dispatch explanation (top comment) | 3.5 | 2.3 | 0.603 |
| walker |  | 5369 | 528 | crate-doc body in src/lib.rs |  |  | 0.603 |
| walker |  | 5506 | 137 | README.md section #8 |  |  | 0.603 |
| walker |  | 5556 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.608 |
| walker |  | 5569 | 13 | pub item at src/ptr.rs:181 |  |  | 0.608 |
| walker |  | 5616 | 47 | pub item at src/ptr.rs:6 |  |  | 0.609 |
| ns | 5624 |  | 349 | Error struct: Display/Debug output forms | 3.6 |  | 0.588 |
| walker |  | 5677 | 61 | pub item at src/ptr.rs:64 |  |  | 0.589 |
| walker |  | 5700 | 23 | pub-item names surface in tests/drop/mod.rs |  |  | 0.589 |
| ns | 5711 |  | 87 | Chain doc + struct def | 3.7 | 2.1 | 0.590 |
| walker |  | 5724 | 24 | pub item at tests/drop/mod.rs:26 |  |  | 0.590 |
| walker |  | 5749 | 25 | pub item at tests/drop/mod.rs:9 |  |  | 0.590 |
| walker |  | 5811 | 62 | pub item at src/ptr.rs:125 |  |  | 0.592 |
| walker |  | 5865 | 54 | pub-item names surface in src/wrapper.rs |  |  | 0.599 |
| walker |  | 5874 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.599 |
| walker |  | 5883 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.599 |
| walker |  | 5892 | 9 | pub item at src/wrapper.rs:58 |  |  | 0.599 |
| walker |  | 5905 | 13 | impl method body at src/ptr.rs:115 body 116 |  |  | 0.599 |
| ns | 5951 |  | 240 | Result alias doc | 3.8 | 2.1 | 0.608 |
| walker |  | 5977 | 72 | impl method sigs in src/fmt.rs |  |  | 0.612 |
| walker |  | 5977 | 0 | impl method at src/fmt.rs:7 |  |  | 0.612 |
| walker |  | 5977 | 0 | impl method at src/fmt.rs:20 |  |  | 0.612 |
| walker |  | 6195 | 218 | README.md section #7 |  |  | 0.639 |
| walker |  | 6294 | 99 | pub item at src/chain.rs:16 |  |  | 0.639 |
| ns | 6324 |  | 373 | error.rs unsafe construct() — the type-erasure trick | 4.1 | 2.2 | 0.619 |
| ns | 6540 |  | 216 | error.rs object_downcast — concrete unsafe cast | 4.2 |  | 0.607 |
| walker |  | 6663 | 369 | crate-doc tail at src/lib.rs:95 |  |  | 0.607 |
| ns | 6692 |  | 152 | ptr.rs Own/Ref/Mut struct shapes | 4.3 | 2.3 | 0.620 |
| ns | 6816 |  | 124 | chain.rs Iterator::next impl | 4.4 | 2.3 | 0.613 |
| ns | 7040 |  | 224 | fmt.rs ErrorImpl::debug impl | 4.5 | 2.3 | 0.601 |
| walker |  | 7044 | 381 | crate-doc tail at src/lib.rs:142 |  |  | 0.601 |
| ns | 7279 |  | 239 | ensure.rs BothDebug/NotBothDebug dispatch + render() | 4.6 | 2.3 | 0.591 |
| walker |  | 7354 | 310 | crate-doc tail at src/lib.rs:179 |  |  | 0.591 |
| walker |  | 7437 | 83 | pub item at src/error.rs:934 |  |  | 0.591 |
| walker |  | 7452 | 15 | impl method body at src/ptr.rs:119 body 120 |  |  | 0.591 |
| walker |  | 7467 | 15 | impl method body at src/ptr.rs:175 body 176 |  |  | 0.591 |
| walker |  | 7483 | 16 | impl method at src/error.rs:470 |  |  | 0.591 |
| walker |  | 7495 | 12 | impl method body at src/error.rs:470 body 471 |  |  | 0.591 |
| walker |  | 7570 | 75 | pub-item names surface in src/kind.rs |  |  | 0.602 |
| ns | 7708 |  | 429 | build.rs cfg flags emitted | 5.1 |  | 0.588 |
| ns | 7784 |  | 76 | CI job roster | 5.2 |  | 0.584 |
| walker |  | 7797 | 227 | README.md section #4 |  |  | 0.584 |
| walker |  | 7813 | 16 | impl method body at src/ptr.rs:169 body 170 |  |  | 0.584 |
| walker |  | 7827 | 14 | listing of 'tests/crate' |  |  | 0.596 |
| walker |  | 7925 | 98 | impl method sigs in src/kind.rs |  |  | 0.601 |
| walker |  | 7925 | 0 | impl method at src/kind.rs:117 |  |  | 0.601 |
| walker |  | 7942 | 17 | impl method body at src/error.rs:459 body 460 |  |  | 0.601 |
| walker |  | 7959 | 17 | impl method body at src/ptr.rs:44 body 45 |  |  | 0.601 |
| walker |  | 8050 | 91 | pub-item names surface in src/nightly.rs |  |  | 0.608 |
| walker |  | 8050 | 0 | pub item at src/nightly.rs:41 |  |  | 0.608 |
| walker |  | 8050 | 0 | pub item at src/nightly.rs:52 |  |  | 0.608 |
| walker |  | 8050 | 0 | pub item at src/nightly.rs:56 |  |  | 0.608 |
| walker |  | 8062 | 12 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.608 |
| walker |  | 8075 | 13 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.608 |
| walker |  | 8089 | 14 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.608 |
| walker |  | 8107 | 18 | impl method body at src/error.rs:432 body 433 |  |  | 0.608 |
| ns | 8115 |  | 331 | CI test job: rust matrix + commands | 5.3 | 5.2 | 0.597 |
| walker |  | 8125 | 18 | impl method body at src/error.rs:1010 body 1011 |  |  | 0.597 |
| walker |  | 8144 | 19 | impl method body at src/error.rs:675 body 676 |  |  | 0.597 |
| walker |  | 8191 | 47 | pub-item names surface in tests/common/mod.rs |  |  | 0.597 |
| walker |  | 8191 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.597 |
| walker |  | 8191 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.597 |
| walker |  | 8191 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.597 |
| walker |  | 8201 | 10 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.597 |
| walker |  | 8217 | 16 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.597 |
| ns | 8236 |  | 121 | test_fmt.rs expected Debug strings (2- and 3-level context) | 6.1 |  | 0.590 |
| ns | 8338 |  | 102 | test_repr.rs size/niche assertions | 6.2 |  | 0.586 |
| walker |  | 8509 | 292 | README.md section #9 |  |  | 0.604 |
| ns | 8517 |  | 179 | test_downcast.rs test_downcast | 6.3 |  | 0.595 |
| walker |  | 8533 | 24 | impl method at src/kind.rs:91 |  |  | 0.595 |
| walker |  | 8541 | 8 | impl method body at src/kind.rs:91 body 95 |  |  | 0.595 |
| walker |  | 8678 | 137 | README.md section #6 |  |  | 0.595 |
| walker |  | 8702 | 24 | impl method body at src/ptr.rs:38 body 39 |  |  | 0.595 |
| walker |  | 8726 | 24 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.595 |
| ns | 8792 |  | 275 | test_context.rs test_downcast_ref (3-level chain) | 6.4 |  | 0.584 |
| walker |  | 8869 | 143 | README.md section #3 |  |  | 0.584 |
| walker |  | 8888 | 19 | mod/use plumbing in tests/common/mod.rs |  |  | 0.584 |
| walker |  | 8916 | 28 | impl method at src/error.rs:663 |  |  | 0.584 |
| walker |  | 8945 | 29 | impl method at src/context.rs:46 |  |  | 0.584 |
| walker |  | 8974 | 29 | impl method at src/context.rs:91 |  |  | 0.584 |
| ns | 8981 |  | 189 | test_chain.rs test_iter | 6.5 |  | 0.578 |
| walker |  | 9003 | 29 | impl method at src/error.rs:198 |  |  | 0.578 |
| walker |  | 9032 | 29 | impl method at src/error.rs:372 |  |  | 0.578 |
| walker |  | 9062 | 30 | impl method at src/error.rs:30 |  |  | 0.578 |
| walker |  | 9092 | 30 | impl method at src/error.rs:147 |  |  | 0.578 |
| walker |  | 9123 | 31 | impl method at src/error.rs:77 |  |  | 0.578 |
| walker |  | 9141 | 18 | impl method body at src/error.rs:77 body 81 |  |  | 0.578 |
| ns | 9143 |  | 162 | test_source.rs io/anyhow-from-anyhow source tests | 6.6 |  | 0.572 |
| walker |  | 9172 | 31 | impl method at src/error.rs:170 |  |  | 0.572 |
| walker |  | 9203 | 31 | impl method at src/error.rs:482 |  |  | 0.572 |
| walker |  | 9218 | 15 | impl method body at src/error.rs:482 body 486 |  |  | 0.572 |
| ns | 9240 |  | 97 | test_autotrait.rs Send/Sync assertions | 6.7 |  | 0.567 |
| walker |  | 9249 | 31 | impl method at src/error.rs:490 |  |  | 0.567 |
| walker |  | 9280 | 31 | impl method at src/error.rs:554 |  |  | 0.567 |
| walker |  | 9311 | 31 | impl method at src/error.rs:568 |  |  | 0.567 |
| walker |  | 9342 | 31 | impl method at src/kind.rs:69 |  |  | 0.567 |
| walker |  | 9360 | 18 | impl method body at src/kind.rs:69 body 73 |  |  | 0.567 |
| ns | 9380 |  | 140 | tests/common/mod.rs shared bail! helpers | 6.8 |  | 0.569 |
| walker |  | 9390 | 30 | impl method body at src/ptr.rs:94 body 95 |  |  | 0.569 |
| walker |  | 9421 | 31 | impl method body at src/chain.rs:28 body 29 |  |  | 0.569 |
| walker |  | 9454 | 33 | impl method body at src/error.rs:30 body 34 |  |  | 0.569 |
| walker |  | 9487 | 33 | impl method body at src/ptr.rs:48 body 49 |  |  | 0.569 |
| walker |  | 9520 | 33 | impl method body at src/ptr.rs:55 body 56 |  |  | 0.569 |
| walker |  | 9553 | 33 | impl method body at src/ptr.rs:108 body 109 |  |  | 0.569 |
| walker |  | 9586 | 33 | impl method body at src/ptr.rs:155 body 156 |  |  | 0.561 |
| ns | 9586 |  | 206 | test_ffi.rs extern "C" compatibility | 6.9 |  | 0.561 |
| walker |  | 9619 | 33 | impl method body at src/ptr.rs:162 body 163 |  |  | 0.561 |
| walker |  | 9653 | 34 | impl method body at src/ptr.rs:101 body 102 |  |  | 0.561 |
| walker |  | 9687 | 34 | impl method body at src/ptr.rs:148 body 149 |  |  | 0.561 |
| ns | 9695 |  | 109 | ui/wrong-interpolation.rs + expected stderr | 7.1 |  | 0.557 |
| walker |  | 9722 | 35 | impl method body at src/kind.rs:117 body 118 |  |  | 0.557 |
| walker |  | 9757 | 35 | impl method body at src/ptr.rs:32 body 33 |  |  | 0.557 |
| walker |  | 9783 | 26 | pub item at src/ensure.rs:10 |  |  | 0.557 |
| walker |  | 9809 | 26 | pub item at src/ensure.rs:25 |  |  | 0.557 |
| walker |  | 9960 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.558 |
| ns | 9977 |  | 282 | ui/must-use.rs + expected stderr | 7.2 |  | 0.548 |
| walker |  | 9996 | 36 | impl method body at src/ptr.rs:87 body 88 |  |  | 0.548 |
