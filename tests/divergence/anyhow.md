Score(3000)=0.676 I=0.875 C=0.522 ns_rows≤3K=18/46 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | listing of '.' |  |  | 1.000 |
| ns | 37 |  | 37 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 64 | 27 | [features] in Cargo.toml |  |  | 1.000 |
| ns | 85 |  | 48 | src/ module listing | 1.2 |  | 0.606 |
| walker |  | 131 | 67 | README headline in README.md |  |  | 0.608 |
| ns | 178 |  | 93 | tests/ top-level listing | 1.3 |  | 0.419 |
| walker |  | 179 | 48 | listing of 'src' |  |  | 0.696 |
| walker |  | 187 | 8 | listing of '.github' |  |  | 0.696 |
| walker |  | 190 | 3 | listing of '.github/workflows' |  |  | 0.696 |
| ns | 278 |  | 100 | test-support subdirectory listings | 1.4 |  | 0.549 |
| walker |  | 280 | 90 | pub-item names surface in src/lib.rs |  |  | 0.550 |
| walker |  | 280 | 0 | pub item at src/lib.rs:468 |  |  | 0.550 |
| walker |  | 293 | 13 | pub item at src/lib.rs:650 |  |  | 0.550 |
| ns | 305 |  | 27 | Cargo.toml feature flags | 1.5 |  | 0.569 |
| walker |  | 319 | 26 | pub item at src/lib.rs:390 |  |  | 0.569 |
| walker |  | 348 | 29 | pub item at src/lib.rs:415 |  |  | 0.570 |
| walker |  | 358 | 10 | pub item body at src/lib.rs:650 body 651 |  |  | 0.570 |
| walker |  | 402 | 44 | headings outline in README.md |  |  | 0.570 |
| ns | 470 |  | 165 | Cargo.toml package identity | 1.6 |  | 0.512 |
| ns | 511 |  | 41 | README pitch sentence | 1.7 |  | 0.523 |
| walker |  | 562 | 160 | pub item at src/lib.rs:616 |  |  | 0.524 |
| ns | 621 |  | 110 | Cargo.toml dependencies | 1.8 |  | 0.501 |
| walker |  | 655 | 93 | listing of 'tests' |  |  | 0.695 |
| ns | 674 |  | 53 | Misc root config: toolchain, gitignore, funding, pin | 1.9 |  | 0.666 |
| walker |  | 820 | 165 | [package] in Cargo.toml |  |  | 0.764 |
| ns | 899 |  | 225 | Cargo.toml dev-dependencies + docs.rs metadata | 1.10 |  | 0.692 |
| walker |  | 902 | 82 | macro_export names across src |  |  | 0.692 |
| ns | 934 |  | 35 | README install snippet | 1.11 |  | 0.676 |
| walker |  | 976 | 74 | README.md section #0 |  |  | 0.703 |
| ns | 1072 |  | 138 | lib.rs module declarations + extern crate | 1.12 |  | 0.641 |
| walker |  | 1086 | 110 | [dependencies] in Cargo.toml |  |  | 0.678 |
| ns | 1292 |  | 220 | README no-std support section | 1.13 |  | 0.634 |
| walker |  | 1360 | 274 | crate-doc lede in src/lib.rs |  |  | 0.634 |
| walker |  | 1550 | 190 | mod/use plumbing in src/lib.rs |  |  | 0.665 |
| walker |  | 1647 | 97 | dev/build/target dependencies in Cargo.toml |  |  | 0.678 |
| ns | 1693 |  | 401 | lib.rs crate-level attributes | 1.14 |  | 0.607 |
| walker |  | 1770 | 123 | manifest config in Cargo.toml |  |  | 0.665 |
| ns | 1857 |  | 164 | README license section | 1.15 |  | 0.635 |
| walker |  | 1911 | 141 | impl method sigs in src/context.rs |  |  | 0.635 |
| walker |  | 1914 | 3 | listing of 'tests/common' |  |  | 0.635 |
| walker |  | 1955 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.636 |
| ns | 1969 |  | 112 | lib.rs top-level public API roster | 2.1 |  | 0.646 |
| walker |  | 2020 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.647 |
| walker |  | 2023 | 3 | listing of 'tests/drop' |  |  | 0.648 |
| walker |  | 2161 | 138 | macro_export body at src/macros.rs:58 |  |  | 0.649 |
| walker |  | 2335 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.649 |
| ns | 2367 |  | 398 | error.rs Error impl method roster | 2.2 |  | 0.600 |
| walker |  | 2736 | 401 | crate attributes in src/lib.rs |  |  | 0.699 |
| walker |  | 2759 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.699 |
| walker |  | 2840 | 81 | listing of 'tests/ui' |  |  | 0.754 |
| ns | 2972 |  | 605 | Rest-of-crate public/crate-visible item roster | 2.3 |  | 0.676 |
| walker |  | 3062 | 222 | macro_export body at src/macros.rs:204 |  |  | 0.678 |
| walker |  | 3390 | 328 | impl method sigs in src/error.rs |  |  | 0.695 |
| ns | 3704 |  | 732 | Context trait doc (prose, code examples elided) | 3.1 | 2.1 | 0.626 |
| walker |  | 3788 | 398 | impl method sigs in src/ptr.rs |  |  | 0.640 |
| walker |  | 3821 | 33 | pub-item names surface in src/chain.rs |  |  | 0.640 |
| walker |  | 3838 | 17 | pub item at src/chain.rs:11 |  |  | 0.640 |
| ns | 4013 |  | 309 | macros.rs bail! doc + expansion | 3.2 |  | 0.625 |
| walker |  | 4064 | 226 | pub-item doc lede at src/lib.rs:468 |  |  | 0.627 |
| walker |  | 4098 | 34 | pub-item names surface in src/error.rs |  |  | 0.627 |
| walker |  | 4131 | 33 | pub item at src/error.rs:952 |  |  | 0.627 |
| walker |  | 4415 | 284 | pub-item doc lede at src/lib.rs:650 |  |  | 0.627 |
| walker |  | 4455 | 40 | impl method sigs in src/chain.rs |  |  | 0.629 |
| ns | 4462 |  | 449 | macros.rs anyhow! doc + expansion | 3.3 |  | 0.613 |
| ns | 4692 |  | 230 | macros.rs ensure! doc summary | 3.4 |  | 0.600 |
| walker |  | 4983 | 528 | crate-doc body in src/lib.rs |  |  | 0.600 |
| walker |  | 5120 | 137 | README.md section #8 |  |  | 0.600 |
| walker |  | 5170 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.606 |
| walker |  | 5183 | 13 | pub item at src/ptr.rs:181 |  |  | 0.606 |
| walker |  | 5230 | 47 | pub item at src/ptr.rs:6 |  |  | 0.606 |
| ns | 5276 |  | 584 | kind.rs tagged-dispatch explanation (top comment) | 3.5 | 2.3 | 0.570 |
| walker |  | 5291 | 61 | pub item at src/ptr.rs:64 |  |  | 0.571 |
| walker |  | 5314 | 23 | pub-item names surface in tests/drop/mod.rs |  |  | 0.571 |
| walker |  | 5338 | 24 | pub item at tests/drop/mod.rs:26 |  |  | 0.571 |
| walker |  | 5363 | 25 | pub item at tests/drop/mod.rs:9 |  |  | 0.571 |
| walker |  | 5425 | 62 | pub item at src/ptr.rs:125 |  |  | 0.572 |
| walker |  | 5479 | 54 | pub-item names surface in src/wrapper.rs |  |  | 0.580 |
| walker |  | 5488 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.580 |
| walker |  | 5497 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.580 |
| walker |  | 5506 | 9 | pub item at src/wrapper.rs:58 |  |  | 0.580 |
| ns | 5625 |  | 349 | Error struct: Display/Debug output forms | 3.6 |  | 0.561 |
| ns | 5712 |  | 87 | Chain doc + struct def | 3.7 | 2.1 | 0.563 |
| walker |  | 5724 | 218 | README.md section #7 |  |  | 0.591 |
| walker |  | 5823 | 99 | pub item at src/chain.rs:16 |  |  | 0.591 |
| ns | 5952 |  | 240 | Result alias doc | 3.8 | 2.1 | 0.600 |
| walker |  | 6192 | 369 | crate-doc tail at src/lib.rs:95 |  |  | 0.600 |
| ns | 6325 |  | 373 | error.rs unsafe construct() — the type-erasure trick | 4.1 | 2.2 | 0.582 |
| ns | 6541 |  | 216 | error.rs object_downcast — concrete unsafe cast | 4.2 |  | 0.571 |
| walker |  | 6573 | 381 | crate-doc tail at src/lib.rs:142 |  |  | 0.571 |
| ns | 6693 |  | 152 | ptr.rs Own/Ref/Mut struct shapes | 4.3 | 2.3 | 0.585 |
| ns | 6817 |  | 124 | chain.rs Iterator::next impl | 4.4 | 2.3 | 0.579 |
| walker |  | 6883 | 310 | crate-doc tail at src/lib.rs:179 |  |  | 0.579 |
| walker |  | 6966 | 83 | pub item at src/error.rs:934 |  |  | 0.579 |
| walker |  | 7041 | 75 | pub-item names surface in src/kind.rs |  |  | 0.578 |
| ns | 7041 |  | 224 | fmt.rs ErrorImpl::debug impl | 4.5 | 2.3 | 0.578 |
| walker |  | 7054 | 13 | listing of 'tests/crate' |  |  | 0.590 |
| ns | 7280 |  | 239 | ensure.rs BothDebug/NotBothDebug dispatch + render() | 4.6 | 2.3 | 0.581 |
| walker |  | 7281 | 227 | README.md section #4 |  |  | 0.581 |
| walker |  | 7379 | 98 | impl method sigs in src/kind.rs |  |  | 0.586 |
| walker |  | 7470 | 91 | pub-item names surface in src/nightly.rs |  |  | 0.593 |
| walker |  | 7470 | 0 | pub item at src/nightly.rs:41 |  |  | 0.593 |
| walker |  | 7470 | 0 | pub item at src/nightly.rs:52 |  |  | 0.593 |
| walker |  | 7470 | 0 | pub item at src/nightly.rs:56 |  |  | 0.593 |
| walker |  | 7482 | 12 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.593 |
| walker |  | 7495 | 13 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.593 |
| walker |  | 7509 | 14 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.593 |
| walker |  | 7556 | 47 | pub-item names surface in tests/common/mod.rs |  |  | 0.593 |
| walker |  | 7556 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.593 |
| walker |  | 7556 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.593 |
| walker |  | 7556 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.593 |
| walker |  | 7566 | 10 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.593 |
| walker |  | 7582 | 16 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.593 |
| ns | 7709 |  | 429 | build.rs cfg flags emitted | 5.1 |  | 0.579 |
| ns | 7785 |  | 76 | CI job roster | 5.2 |  | 0.575 |
| walker |  | 7874 | 292 | README.md section #9 |  |  | 0.594 |
| walker |  | 8011 | 137 | README.md section #6 |  |  | 0.594 |
| walker |  | 8035 | 24 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.594 |
| ns | 8116 |  | 331 | CI test job: rust matrix + commands | 5.3 | 5.2 | 0.584 |
| walker |  | 8178 | 143 | README.md section #3 |  |  | 0.584 |
| walker |  | 8197 | 19 | mod/use plumbing in tests/common/mod.rs |  |  | 0.584 |
| walker |  | 8223 | 26 | pub item at src/ensure.rs:10 |  |  | 0.584 |
| ns | 8237 |  | 121 | test_fmt.rs expected Debug strings (2- and 3-level context) | 6.1 |  | 0.577 |
| walker |  | 8249 | 26 | pub item at src/ensure.rs:25 |  |  | 0.577 |
| ns | 8339 |  | 102 | test_repr.rs size/niche assertions | 6.2 |  | 0.573 |
| walker |  | 8400 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.574 |
| ns | 8518 |  | 179 | test_downcast.rs test_downcast | 6.3 |  | 0.565 |
| walker |  | 8613 | 213 | README.md section #5 |  |  | 0.565 |
| ns | 8793 |  | 275 | test_context.rs test_downcast_ref (3-level chain) | 6.4 |  | 0.555 |
| walker |  | 8832 | 219 | README.md section #1 |  |  | 0.555 |
| walker |  | 8958 | 126 | pub-item doc body at src/lib.rs:468 |  |  | 0.555 |
| ns | 8982 |  | 189 | test_chain.rs test_iter | 6.5 |  | 0.548 |
| ns | 9144 |  | 162 | test_source.rs io/anyhow-from-anyhow source tests | 6.6 |  | 0.543 |
| walker |  | 9222 | 264 | README.md section #2 |  |  | 0.543 |
| ns | 9241 |  | 97 | test_autotrait.rs Send/Sync assertions | 6.7 |  | 0.539 |
| walker |  | 9262 | 40 | pub item at src/kind.rs:80 |  |  | 0.539 |
| walker |  | 9304 | 42 | pub item at src/kind.rs:58 |  |  | 0.539 |
| walker |  | 9346 | 42 | pub item at src/kind.rs:104 |  |  | 0.539 |
| ns | 9381 |  | 140 | tests/common/mod.rs shared bail! helpers | 6.8 |  | 0.541 |
| walker |  | 9415 | 69 | [package] in tests/crate/Cargo.toml |  |  | 0.541 |
| walker |  | 9469 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.541 |
| walker |  | 9540 | 71 | pub-item names surface in tests/test_ffi.rs |  |  | 0.541 |
| walker |  | 9540 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.541 |
| walker |  | 9540 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.541 |
| walker |  | 9540 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.541 |
| walker |  | 9551 | 11 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.541 |
| walker |  | 9563 | 12 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.541 |
| walker |  | 9578 | 15 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.541 |
| ns | 9587 |  | 206 | test_ffi.rs extern "C" compatibility | 6.9 |  | 0.537 |
| walker |  | 9618 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.537 |
| ns | 9696 |  | 109 | ui/wrong-interpolation.rs + expected stderr | 7.1 |  | 0.533 |
| walker |  | 9738 | 120 | impl method sigs in tests/drop/mod.rs |  |  | 0.533 |
| walker |  | 9753 | 15 | plaintext config .gitignore |  |  | 0.534 |
| walker |  | 9779 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.534 |
| ns | 9978 |  | 282 | ui/must-use.rs + expected stderr | 7.2 |  | 0.525 |
