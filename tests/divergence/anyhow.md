Score(3000)=0.625 I=0.834 C=0.468 ns_rows≤3K=18/44 (reached=9 partial=2 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Top-level fixture listing | 1.1 |  | 1.000 |
| walker |  | 57 | 23 | [features] in Cargo.toml |  |  | 1.000 |
| ns | 79 |  | 45 | Crate-doc lede | 1.2 |  | 0.842 |
| walker |  | 120 | 63 | README headline in README.md |  |  | 0.842 |
| ns | 128 |  | 49 | src/ listing | 1.3 |  | 0.554 |
| walker |  | 164 | 44 | headings outline in README.md |  |  | 0.554 |
| ns | 195 |  | 67 | Crate identity (name, edition, MSRV) | 1.4 |  | 0.484 |
| ns | 285 |  | 90 | tests/ listing | 1.5 |  | 0.365 |
| walker |  | 331 | 167 | [package] in Cargo.toml |  |  | 0.479 |
| walker |  | 397 | 66 | README.md section #0 |  |  | 0.479 |
| ns | 424 |  | 139 | lib.rs module declarations | 2.1 |  | 0.392 |
| walker |  | 446 | 49 | listing of 'src' |  |  | 0.569 |
| ns | 458 |  | 34 | Error struct definition | 2.2 |  | 0.550 |
| ns | 481 |  | 23 | Result type alias | 2.3 |  | 0.545 |
| walker |  | 534 | 88 | pub-item names surface in src/lib.rs |  |  | 0.559 |
| walker |  | 534 | 0 | pub item at src/lib.rs:468 |  |  | 0.559 |
| walker |  | 534 | 0 | pub item at src/lib.rs:650 |  |  | 0.559 |
| ns | 541 |  | 60 | Chain struct | 2.4 |  | 0.539 |
| walker |  | 549 | 15 | pub item at src/lib.rs:390 |  |  | 0.557 |
| walker |  | 568 | 19 | pub item at src/lib.rs:415 |  |  | 0.569 |
| walker |  | 576 | 8 | pub item body at src/lib.rs:650 body 651 |  |  | 0.569 |
| ns | 697 |  | 156 | Error vs Box<dyn Error>: three differences | 2.5 | 2.2 | 0.529 |
| walker |  | 729 | 153 | pub item at src/lib.rs:616 |  |  | 0.537 |
| walker |  | 807 | 78 | macro_export names across src |  |  | 0.537 |
| ns | 858 |  | 161 | Error::* method signatures (locations) | 2.6 |  | 0.492 |
| walker |  | 980 | 173 | mod/use plumbing in src/lib.rs |  |  | 0.540 |
| walker |  | 1021 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.540 |
| ns | 1036 |  | 178 | Context trait signature | 2.7 |  | 0.576 |
| ns | 1130 |  | 94 | Crate-level public items (locations) | 2.8 |  | 0.558 |
| walker |  | 1295 | 274 | crate-doc lede in src/lib.rs |  |  | 0.592 |
| ns | 1302 |  | 172 | Macro export locations | 2.9 |  | 0.554 |
| walker |  | 1360 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.554 |
| walker |  | 1496 | 136 | macro_export body at src/macros.rs:58 |  |  | 0.554 |
| ns | 1542 |  | 240 | Cargo features + dev-deps | 2.10 | 1.4 | 0.519 |
| walker |  | 1670 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.572 |
| ns | 1757 |  | 215 | Display reprs: `{}` and `{:#}` | 2.11 | 2.5 | 0.538 |
| walker |  | 1877 | 207 | [dependencies] in Cargo.toml |  |  | 0.603 |
| walker |  | 1900 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.603 |
| ns | 1979 |  | 222 | anyhow! macro body | 2.12 | 2.9 | 0.562 |
| walker |  | 2120 | 220 | macro_export body at src/macros.rs:204 |  |  | 0.627 |
| walker |  | 2151 | 31 | pub-item names surface in src/chain.rs |  |  | 0.627 |
| walker |  | 2166 | 15 | pub item at src/chain.rs:11 |  |  | 0.627 |
| walker |  | 2388 | 222 | pub-item doc lede at src/lib.rs:468 |  |  | 0.627 |
| walker |  | 2422 | 34 | pub-item names surface in src/error.rs |  |  | 0.627 |
| walker |  | 2445 | 23 | pub item at src/error.rs:952 |  |  | 0.627 |
| ns | 2558 |  | 579 | Context impls for Result and Option | 2.13 | 2.7 | 0.548 |
| walker |  | 2731 | 286 | pub-item doc lede at src/lib.rs:650 |  |  | 0.548 |
| walker |  | 2855 | 124 | README.md section #8 |  |  | 0.548 |
| walker |  | 2945 | 90 | listing of 'tests' |  |  | 0.625 |
| walker |  | 2995 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.625 |
| walker |  | 3006 | 11 | pub item at src/ptr.rs:181 |  |  | 0.625 |
| ns | 3026 |  | 468 | Debug repr `{:?}` (default for `fn main`) | 2.14 | 2.5 | 0.580 |
| walker |  | 3042 | 36 | pub item at src/ptr.rs:6 |  |  | 0.580 |
| walker |  | 3092 | 50 | pub item at src/ptr.rs:64 |  |  | 0.580 |
| walker |  | 3143 | 51 | pub item at src/ptr.rs:125 |  |  | 0.581 |
| walker |  | 3195 | 52 | pub-item names surface in src/wrapper.rs |  |  | 0.581 |
| walker |  | 3268 | 73 | pub item at src/error.rs:934 |  |  | 0.581 |
| ns | 3399 |  | 373 | Debug repr `{:#?}` + manual chain rendering | 2.15 | 2.5 | 0.543 |
| walker |  | 3468 | 200 | README.md section #7 |  |  | 0.543 |
| walker |  | 3561 | 93 | pub item at src/chain.rs:16 |  |  | 0.544 |
| walker |  | 3634 | 73 | pub-item names surface in src/kind.rs |  |  | 0.544 |
| ns | 3672 |  | 273 | error.rs item locations | 3.1 |  | 0.526 |
| walker |  | 3723 | 89 | pub-item names surface in src/nightly.rs |  |  | 0.526 |
| walker |  | 3723 | 0 | pub item at src/nightly.rs:41 |  |  | 0.526 |
| walker |  | 3723 | 0 | pub item at src/nightly.rs:52 |  |  | 0.526 |
| walker |  | 3723 | 0 | pub item at src/nightly.rs:56 |  |  | 0.526 |
| walker |  | 3733 | 10 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.527 |
| walker |  | 3744 | 11 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.527 |
| walker |  | 3756 | 12 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.527 |
| walker |  | 3838 | 82 | listing of 'tests/ui' |  |  | 0.527 |
| walker |  | 3846 | 8 | listing of '.github' |  |  | 0.527 |
| walker |  | 3850 | 4 | listing of '.github/workflows' |  |  | 0.527 |
| ns | 3934 |  | 262 | context.rs item locations | 3.2 |  | 0.509 |
| ns | 4232 |  | 298 | chain.rs item locations | 3.3 |  | 0.501 |
| ns | 4770 |  | 538 | kind.rs tagged-dispatch types | 3.4 |  | 0.467 |
| ns | 4950 |  | 180 | wrapper.rs: MessageError / DisplayError / BoxedError | 3.5 |  | 0.460 |
| ns | 5232 |  | 282 | fmt.rs: ErrorImpl::display body + debug/Indented locations | 3.6 |  | 0.447 |
| walker |  | 5414 | 1564 | crate-doc body in src/lib.rs |  |  | 0.447 |
| ns | 5548 |  | 316 | ptr.rs internal pointer newtypes | 3.7 |  | 0.453 |
| walker |  | 5673 | 259 | README.md section #9 |  |  | 0.453 |
| walker |  | 5697 | 24 | pub item at src/ensure.rs:10 |  |  | 0.453 |
| walker |  | 5721 | 24 | pub item at src/ensure.rs:25 |  |  | 0.453 |
| walker |  | 5872 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.453 |
| walker |  | 5996 | 124 | pub-item doc body at src/lib.rs:468 |  |  | 0.453 |
| ns | 6006 |  | 458 | backtrace.rs cfg landscape (locations) | 3.8 |  | 0.435 |
| walker |  | 6116 | 120 | README.md section #6 |  |  | 0.435 |
| walker |  | 6154 | 38 | pub item at src/kind.rs:80 |  |  | 0.437 |
| ns | 6235 |  | 229 | nightly.rs locations + signatures | 3.9 |  | 0.436 |
| walker |  | 6292 | 138 | README.md section #3 |  |  | 0.436 |
| walker |  | 6332 | 40 | pub item at src/kind.rs:58 |  |  | 0.439 |
| walker |  | 6372 | 40 | pub item at src/kind.rs:104 |  |  | 0.443 |
| walker |  | 6571 | 199 | README.md section #1 |  |  | 0.443 |
| ns | 6577 |  | 342 | ensure.rs orientation header (locations) | 3.10 |  | 0.434 |
| walker |  | 6640 | 69 | pub-item names surface in tests/test_ffi.rs |  |  | 0.434 |
| walker |  | 6640 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.434 |
| walker |  | 6640 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.434 |
| walker |  | 6640 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.434 |
| walker |  | 6649 | 9 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.434 |
| walker |  | 6659 | 10 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.434 |
| walker |  | 6672 | 13 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.434 |
| ns | 6817 |  | 240 | tests/common + drop helpers | 4.1 |  | 0.423 |
| walker |  | 6877 | 205 | README.md section #5 |  |  | 0.423 |
| walker |  | 7092 | 215 | README.md section #4 |  |  | 0.423 |
| ns | 7104 |  | 287 | test_context: Low/Mid/High chain helper + fns | 4.2 |  | 0.412 |
| ns | 7307 |  | 203 | test_downcast / test_repr / test_convert fns (locations) | 4.3 |  | 0.405 |
| walker |  | 7333 | 241 | README.md section #2 |  |  | 0.405 |
| walker |  | 7347 | 14 | listing of 'tests/crate' |  |  | 0.405 |
| walker |  | 7414 | 67 | [package] in tests/crate/Cargo.toml |  |  | 0.405 |
| ns | 7450 |  | 143 | test_macros + test_chain + test_source fns | 4.4 |  | 0.399 |
| walker |  | 7454 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.399 |
| walker |  | 7458 | 4 | listing of 'tests/common' |  |  | 0.399 |
| walker |  | 7503 | 45 | pub-item names surface in tests/common/mod.rs |  |  | 0.400 |
| walker |  | 7503 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.400 |
| walker |  | 7503 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.400 |
| walker |  | 7503 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.400 |
| walker |  | 7511 | 8 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.401 |
| walker |  | 7525 | 14 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.402 |
| walker |  | 7547 | 22 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.403 |
| walker |  | 7568 | 21 | mod/use plumbing in tests/common/mod.rs |  |  | 0.405 |
| walker |  | 7572 | 4 | listing of 'tests/drop' |  |  | 0.405 |
| walker |  | 7593 | 21 | pub-item names surface in tests/drop/mod.rs |  |  | 0.408 |
| walker |  | 7607 | 14 | pub item at tests/drop/mod.rs:26 |  |  | 0.410 |
| walker |  | 7622 | 15 | pub item at tests/drop/mod.rs:9 |  |  | 0.412 |
| walker |  | 7676 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.412 |
| ns | 7686 |  | 236 | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | 4.5 |  | 0.406 |
| walker |  | 7800 | 124 | impl method sigs in tests/drop/mod.rs |  |  | 0.412 |
| walker |  | 7826 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.412 |
| walker |  | 7841 | 15 | plaintext config .gitignore |  |  | 0.412 |
| ns | 7952 |  | 266 | test_autotrait + test_boxed + test_ffi + test_backtrace | 4.6 |  | 0.404 |
| ns | 8056 |  | 104 | tests/ui + tests/crate listings | 4.7 |  | 0.421 |
| ns | 8313 |  | 257 | ErrorImpl<E> layout + vtable() reader | 5.1 | 3.1 | 0.418 |
| ns | 8514 |  | 201 | Chain DoubleEndedIterator buffering | 5.2 | 3.3 | 0.412 |
| ns | 8742 |  | 228 | kind.rs autoref-precedence note | 5.3 | 3.4 | 0.407 |
| walker |  | 9001 | 1160 | pub-item doc body at src/lib.rs:616 |  |  | 0.407 |
| ns | 9288 |  | 546 | ErrorImpl::debug body | 5.4 | 3.6 | 0.394 |
| ns | 9528 |  | 240 | __fancy_ensure! macro body | 5.5 | 3.10 | 0.387 |
| ns | 9798 |  | 270 | ensure!: render `{msg} ({lhs} vs {rhs})` | 5.6 | 3.10 | 0.382 |
| ns | 9946 |  | 148 | build.rs cfg pipeline (rustc version + cfgs) | 5.7 |  | 0.379 |
