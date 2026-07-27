Score(3000)=0.683 I=0.879 C=0.531 ns_rows≤3K=18/46 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 37 | 37 | listing of '.' |  |  | 1.000 |
| ns | 37 |  | 37 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 64 | 27 | [features] in Cargo.toml |  |  | 1.000 |
| ns | 85 |  | 48 | src/ module listing | 1.2 |  | 0.606 |
| walker |  | 112 | 48 | listing of 'src' |  |  | 1.000 |
| ns | 178 |  | 93 | tests/ top-level listing | 1.3 |  | 0.693 |
| walker |  | 179 | 67 | README headline in README.md |  |  | 0.696 |
| walker |  | 187 | 8 | listing of '.github' |  |  | 0.696 |
| walker |  | 190 | 3 | listing of '.github/workflows' |  |  | 0.696 |
| ns | 278 |  | 100 | test-support subdirectory listings | 1.4 |  | 0.549 |
| walker |  | 280 | 90 | pub-item names surface in src/lib.rs |  |  | 0.550 |
| walker |  | 280 | 0 | pub item at src/lib.rs:468 |  |  | 0.550 |
| walker |  | 293 | 13 | pub item at src/lib.rs:650 |  |  | 0.550 |
| ns | 305 |  | 27 | Cargo.toml feature flags | 1.5 |  | 0.569 |
| walker |  | 319 | 26 | pub item at src/lib.rs:390 |  |  | 0.569 |
| walker |  | 329 | 10 | pub item body at src/lib.rs:650 body 651 |  |  | 0.569 |
| walker |  | 378 | 49 | pub item at src/lib.rs:415 |  |  | 0.570 |
| walker |  | 422 | 44 | headings outline in README.md |  |  | 0.570 |
| ns | 470 |  | 165 | Cargo.toml package identity | 1.6 |  | 0.512 |
| ns | 511 |  | 41 | README pitch sentence | 1.7 |  | 0.524 |
| walker |  | 515 | 93 | listing of 'tests' |  |  | 0.727 |
| ns | 621 |  | 110 | Cargo.toml dependencies | 1.8 |  | 0.694 |
| ns | 674 |  | 53 | Misc root config: toolchain, gitignore, funding, pin | 1.9 |  | 0.665 |
| walker |  | 675 | 160 | pub item at src/lib.rs:616 |  |  | 0.666 |
| walker |  | 840 | 165 | [package] in Cargo.toml |  |  | 0.765 |
| ns | 899 |  | 225 | Cargo.toml dev-dependencies + docs.rs metadata | 1.10 |  | 0.692 |
| walker |  | 922 | 82 | macro_export names across src |  |  | 0.692 |
| ns | 934 |  | 35 | README install snippet | 1.11 |  | 0.677 |
| walker |  | 996 | 74 | README.md section #0 |  |  | 0.703 |
| ns | 1072 |  | 138 | lib.rs module declarations + extern crate | 1.12 |  | 0.641 |
| walker |  | 1106 | 110 | [dependencies] in Cargo.toml |  |  | 0.678 |
| ns | 1292 |  | 220 | README no-std support section | 1.13 |  | 0.634 |
| walker |  | 1380 | 274 | crate-doc lede in src/lib.rs |  |  | 0.634 |
| walker |  | 1644 | 264 | mod/use plumbing in src/lib.rs |  |  | 0.678 |
| ns | 1693 |  | 401 | lib.rs crate-level attributes | 1.14 |  | 0.607 |
| walker |  | 1741 | 97 | dev/build/target dependencies in Cargo.toml |  |  | 0.618 |
| ns | 1857 |  | 164 | README license section | 1.15 |  | 0.590 |
| walker |  | 1864 | 123 | manifest config in Cargo.toml |  |  | 0.645 |
| walker |  | 1867 | 3 | listing of 'tests/common' |  |  | 0.645 |
| walker |  | 1908 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.646 |
| ns | 1969 |  | 112 | lib.rs top-level public API roster | 2.1 |  | 0.656 |
| walker |  | 1973 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.657 |
| walker |  | 1976 | 3 | listing of 'tests/drop' |  |  | 0.657 |
| walker |  | 2114 | 138 | macro_export body at src/macros.rs:58 |  |  | 0.658 |
| walker |  | 2288 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.658 |
| ns | 2367 |  | 398 | error.rs Error impl method roster | 2.2 |  | 0.609 |
| walker |  | 2454 | 166 | impl method sigs in src/context.rs |  |  | 0.609 |
| walker |  | 2855 | 401 | crate attributes in src/lib.rs |  |  | 0.707 |
| walker |  | 2878 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.707 |
| walker |  | 2912 | 34 | pub item at src/backtrace.rs:8 |  |  | 0.707 |
| ns | 2972 |  | 605 | Rest-of-crate public/crate-visible item roster | 2.3 |  | 0.634 |
| walker |  | 2993 | 81 | listing of 'tests/ui' |  |  | 0.683 |
| walker |  | 3215 | 222 | macro_export body at src/macros.rs:204 |  |  | 0.685 |
| walker |  | 3252 | 37 | impl method sigs in src/ensure.rs |  |  | 0.685 |
| walker |  | 3252 | 0 | impl method at src/ensure.rs:41 |  |  | 0.685 |
| walker |  | 3252 | 0 | impl method at src/ensure.rs:48 |  |  | 0.685 |
| walker |  | 3285 | 33 | pub-item names surface in src/chain.rs |  |  | 0.685 |
| walker |  | 3320 | 35 | pub item at src/chain.rs:11 |  |  | 0.685 |
| ns | 3704 |  | 732 | Context trait doc (prose, code examples elided) | 3.1 | 2.1 | 0.617 |
| walker |  | 4000 | 680 | impl method sigs in src/error.rs |  |  | 0.680 |
| walker |  | 4000 | 0 | impl method at src/error.rs:958 |  |  | 0.680 |
| walker |  | 4000 | 0 | impl method at src/error.rs:967 |  |  | 0.680 |
| walker |  | 4000 | 0 | impl method at src/error.rs:1010 |  |  | 0.680 |
| ns | 4013 |  | 309 | macros.rs bail! doc + expansion | 3.2 |  | 0.663 |
| walker |  | 4226 | 226 | pub-item doc lede at src/lib.rs:468 |  |  | 0.665 |
| walker |  | 4260 | 34 | pub-item names surface in src/error.rs |  |  | 0.665 |
| walker |  | 4291 | 31 | pub item at src/error.rs:952 |  |  | 0.665 |
| ns | 4462 |  | 449 | macros.rs anyhow! doc + expansion | 3.3 |  | 0.647 |
| walker |  | 4575 | 284 | pub-item doc lede at src/lib.rs:650 |  |  | 0.647 |
| walker |  | 4615 | 40 | impl method sigs in src/chain.rs |  |  | 0.648 |
| walker |  | 4615 | 0 | impl method at src/chain.rs:28 |  |  | 0.648 |
| ns | 4692 |  | 230 | macros.rs ensure! doc summary | 3.4 |  | 0.633 |
| walker |  | 5061 | 446 | impl method sigs in src/ptr.rs |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:32 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:38 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:44 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:48 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:55 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:87 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:94 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:101 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:108 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:115 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:119 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:148 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:155 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:162 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:169 |  |  | 0.646 |
| walker |  | 5061 | 0 | impl method at src/ptr.rs:175 |  |  | 0.646 |
| ns | 5276 |  | 584 | kind.rs tagged-dispatch explanation (top comment) | 3.5 | 2.3 | 0.608 |
| walker |  | 5589 | 528 | crate-doc body in src/lib.rs |  |  | 0.608 |
| ns | 5625 |  | 349 | Error struct: Display/Debug output forms | 3.6 |  | 0.587 |
| ns | 5712 |  | 87 | Chain doc + struct def | 3.7 | 2.1 | 0.593 |
| walker |  | 5726 | 137 | README.md section #8 |  |  | 0.593 |
| walker |  | 5776 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.598 |
| walker |  | 5789 | 13 | pub item at src/ptr.rs:181 |  |  | 0.598 |
| walker |  | 5836 | 47 | pub item at src/ptr.rs:6 |  |  | 0.599 |
| walker |  | 5848 | 12 | impl method at src/error.rs:675 |  |  | 0.599 |
| walker |  | 5909 | 61 | pub item at src/ptr.rs:64 |  |  | 0.599 |
| walker |  | 5932 | 23 | pub-item names surface in tests/drop/mod.rs |  |  | 0.599 |
| ns | 5952 |  | 240 | Result alias doc | 3.8 | 2.1 | 0.608 |
| walker |  | 5956 | 24 | pub item at tests/drop/mod.rs:26 |  |  | 0.608 |
| walker |  | 5981 | 25 | pub item at tests/drop/mod.rs:9 |  |  | 0.608 |
| walker |  | 6043 | 62 | pub item at src/ptr.rs:125 |  |  | 0.609 |
| walker |  | 6097 | 54 | pub-item names surface in src/wrapper.rs |  |  | 0.616 |
| walker |  | 6106 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.616 |
| walker |  | 6115 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.616 |
| walker |  | 6144 | 29 | pub item at src/wrapper.rs:58 |  |  | 0.616 |
| walker |  | 6157 | 13 | impl method at src/error.rs:1002 |  |  | 0.616 |
| walker |  | 6170 | 13 | impl method body at src/ptr.rs:115 body 116 |  |  | 0.616 |
| walker |  | 6242 | 72 | impl method sigs in src/fmt.rs |  |  | 0.621 |
| walker |  | 6242 | 0 | impl method at src/fmt.rs:7 |  |  | 0.621 |
| walker |  | 6242 | 0 | impl method at src/fmt.rs:20 |  |  | 0.621 |
| ns | 6325 |  | 373 | error.rs unsafe construct() — the type-erasure trick | 4.1 | 2.2 | 0.601 |
| walker |  | 6460 | 218 | README.md section #7 |  |  | 0.627 |
| ns | 6541 |  | 216 | error.rs object_downcast — concrete unsafe cast | 4.2 |  | 0.615 |
| walker |  | 6559 | 99 | pub item at src/chain.rs:16 |  |  | 0.615 |
| ns | 6693 |  | 152 | ptr.rs Own/Ref/Mut struct shapes | 4.3 | 2.3 | 0.627 |
| ns | 6817 |  | 124 | chain.rs Iterator::next impl | 4.4 | 2.3 | 0.620 |
| walker |  | 6928 | 369 | crate-doc tail at src/lib.rs:95 |  |  | 0.620 |
| ns | 7041 |  | 224 | fmt.rs ErrorImpl::debug impl | 4.5 | 2.3 | 0.608 |
| ns | 7280 |  | 239 | ensure.rs BothDebug/NotBothDebug dispatch + render() | 4.6 | 2.3 | 0.598 |
| walker |  | 7309 | 381 | crate-doc tail at src/lib.rs:142 |  |  | 0.598 |
| walker |  | 7619 | 310 | crate-doc tail at src/lib.rs:179 |  |  | 0.598 |
| walker |  | 7702 | 83 | pub item at src/error.rs:934 |  |  | 0.598 |
| ns | 7709 |  | 429 | build.rs cfg flags emitted | 5.1 |  | 0.584 |
| walker |  | 7717 | 15 | impl method body at src/ptr.rs:119 body 120 |  |  | 0.584 |
| walker |  | 7732 | 15 | impl method body at src/ptr.rs:175 body 176 |  |  | 0.584 |
| ns | 7785 |  | 76 | CI job roster | 5.2 |  | 0.580 |
| walker |  | 7807 | 75 | pub-item names surface in src/kind.rs |  |  | 0.591 |
| walker |  | 7827 | 20 | pub item at src/kind.rs:100 |  |  | 0.591 |
| walker |  | 7840 | 13 | listing of 'tests/crate' |  |  | 0.603 |
| walker |  | 8067 | 227 | README.md section #4 |  |  | 0.603 |
| walker |  | 8083 | 16 | impl method body at src/ptr.rs:169 body 170 |  |  | 0.603 |
| ns | 8116 |  | 331 | CI test job: rust matrix + commands | 5.3 | 5.2 | 0.592 |
| walker |  | 8181 | 98 | impl method sigs in src/kind.rs |  |  | 0.597 |
| walker |  | 8181 | 0 | impl method at src/kind.rs:117 |  |  | 0.597 |
| walker |  | 8198 | 17 | impl method body at src/ptr.rs:44 body 45 |  |  | 0.597 |
| ns | 8237 |  | 121 | test_fmt.rs expected Debug strings (2- and 3-level context) | 6.1 |  | 0.590 |
| walker |  | 8289 | 91 | pub-item names surface in src/nightly.rs |  |  | 0.596 |
| walker |  | 8289 | 0 | pub item at src/nightly.rs:41 |  |  | 0.596 |
| walker |  | 8289 | 0 | pub item at src/nightly.rs:52 |  |  | 0.596 |
| walker |  | 8289 | 0 | pub item at src/nightly.rs:56 |  |  | 0.596 |
| walker |  | 8301 | 12 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.596 |
| walker |  | 8314 | 13 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.596 |
| walker |  | 8328 | 14 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.596 |
| ns | 8339 |  | 102 | test_repr.rs size/niche assertions | 6.2 |  | 0.592 |
| walker |  | 8346 | 18 | impl method body at src/error.rs:1010 body 1011 |  |  | 0.592 |
| walker |  | 8365 | 19 | impl method at src/error.rs:432 |  |  | 0.592 |
| walker |  | 8383 | 18 | impl method body at src/error.rs:432 body 433 |  |  | 0.592 |
| walker |  | 8402 | 19 | impl method at src/error.rs:985 |  |  | 0.592 |
| walker |  | 8421 | 19 | impl method body at src/error.rs:675 body 676 |  |  | 0.592 |
| walker |  | 8442 | 21 | impl method at src/error.rs:974 |  |  | 0.592 |
| walker |  | 8489 | 47 | pub-item names surface in tests/common/mod.rs |  |  | 0.592 |
| walker |  | 8489 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.592 |
| walker |  | 8489 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.592 |
| walker |  | 8489 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.592 |
| walker |  | 8499 | 10 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.592 |
| walker |  | 8515 | 16 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.592 |
| ns | 8518 |  | 179 | test_downcast.rs test_downcast | 6.3 |  | 0.583 |
| ns | 8793 |  | 275 | test_context.rs test_downcast_ref (3-level chain) | 6.4 |  | 0.573 |
| walker |  | 8807 | 292 | README.md section #9 |  |  | 0.590 |
| walker |  | 8830 | 23 | impl method at src/error.rs:140 |  |  | 0.590 |
| walker |  | 8853 | 23 | impl method at src/error.rs:459 |  |  | 0.590 |
| walker |  | 8870 | 17 | impl method body at src/error.rs:459 body 460 |  |  | 0.590 |
| walker |  | 8893 | 23 | impl method at src/error.rs:622 |  |  | 0.590 |
| walker |  | 8917 | 24 | impl method at src/kind.rs:91 |  |  | 0.590 |
| walker |  | 8925 | 8 | impl method body at src/kind.rs:91 body 95 |  |  | 0.590 |
| ns | 8982 |  | 189 | test_chain.rs test_iter | 6.5 |  | 0.584 |
| walker |  | 9062 | 137 | README.md section #6 |  |  | 0.584 |
| walker |  | 9086 | 24 | impl method body at src/ptr.rs:38 body 39 |  |  | 0.584 |
| walker |  | 9110 | 24 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.584 |
| ns | 9144 |  | 162 | test_source.rs io/anyhow-from-anyhow source tests | 6.6 |  | 0.578 |
| ns | 9241 |  | 97 | test_autotrait.rs Send/Sync assertions | 6.7 |  | 0.573 |
| walker |  | 9253 | 143 | README.md section #3 |  |  | 0.573 |
| walker |  | 9272 | 19 | mod/use plumbing in tests/common/mod.rs |  |  | 0.573 |
| walker |  | 9301 | 29 | impl method at src/context.rs:46 |  |  | 0.573 |
| walker |  | 9330 | 29 | impl method at src/context.rs:91 |  |  | 0.573 |
| walker |  | 9359 | 29 | impl method at src/error.rs:198 |  |  | 0.573 |
| ns | 9381 |  | 140 | tests/common/mod.rs shared bail! helpers | 6.8 |  | 0.575 |
| walker |  | 9388 | 29 | impl method at src/error.rs:372 |  |  | 0.575 |
| walker |  | 9419 | 31 | impl method at src/error.rs:77 |  |  | 0.575 |
| walker |  | 9437 | 18 | impl method body at src/error.rs:77 body 81 |  |  | 0.575 |
| walker |  | 9468 | 31 | impl method at src/error.rs:170 |  |  | 0.575 |
| walker |  | 9499 | 31 | impl method at src/error.rs:482 |  |  | 0.575 |
| walker |  | 9514 | 15 | impl method body at src/error.rs:482 body 486 |  |  | 0.575 |
| walker |  | 9545 | 31 | impl method at src/error.rs:490 |  |  | 0.575 |
| walker |  | 9576 | 31 | impl method at src/error.rs:554 |  |  | 0.575 |
| ns | 9587 |  | 206 | test_ffi.rs extern "C" compatibility | 6.9 |  | 0.567 |
| walker |  | 9607 | 31 | impl method at src/error.rs:568 |  |  | 0.567 |
| walker |  | 9638 | 31 | impl method at src/kind.rs:69 |  |  | 0.567 |
| walker |  | 9656 | 18 | impl method body at src/kind.rs:69 body 73 |  |  | 0.567 |
| walker |  | 9686 | 30 | impl method body at src/ptr.rs:94 body 95 |  |  | 0.567 |
| ns | 9696 |  | 109 | ui/wrong-interpolation.rs + expected stderr | 7.1 |  | 0.563 |
| walker |  | 9717 | 31 | impl method body at src/chain.rs:28 body 29 |  |  | 0.563 |
| walker |  | 9750 | 33 | impl method body at src/ptr.rs:48 body 49 |  |  | 0.563 |
| walker |  | 9783 | 33 | impl method body at src/ptr.rs:55 body 56 |  |  | 0.563 |
| walker |  | 9816 | 33 | impl method body at src/ptr.rs:108 body 109 |  |  | 0.563 |
| walker |  | 9849 | 33 | impl method body at src/ptr.rs:155 body 156 |  |  | 0.563 |
| walker |  | 9882 | 33 | impl method body at src/ptr.rs:162 body 163 |  |  | 0.563 |
| walker |  | 9916 | 34 | impl method body at src/ptr.rs:101 body 102 |  |  | 0.563 |
| walker |  | 9950 | 34 | impl method body at src/ptr.rs:148 body 149 |  |  | 0.563 |
| ns | 9978 |  | 282 | ui/must-use.rs + expected stderr | 7.2 |  | 0.553 |
| walker |  | 9985 | 35 | impl method body at src/kind.rs:117 body 118 |  |  | 0.553 |
