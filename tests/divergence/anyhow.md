Score(3000)=0.625 I=0.834 C=0.468 ns_rows≤3K=18/44 (reached=9 partial=2 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Top-level fixture listing | 1.1 |  | 1.000 |
| walker |  | 57 | 23 | [features] in Cargo.toml |  |  | 1.000 |
| walker |  | 65 | 8 | listing of '.github' |  |  | 1.000 |
| walker |  | 69 | 4 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 79 |  | 45 | Crate-doc lede | 1.2 |  | 0.842 |
| ns | 128 |  | 49 | src/ listing | 1.3 |  | 0.554 |
| walker |  | 132 | 63 | README headline in README.md |  |  | 0.554 |
| walker |  | 176 | 44 | headings outline in README.md |  |  | 0.554 |
| ns | 195 |  | 67 | Crate identity (name, edition, MSRV) | 1.4 |  | 0.484 |
| ns | 285 |  | 90 | tests/ listing | 1.5 |  | 0.365 |
| walker |  | 343 | 167 | [package] in Cargo.toml |  |  | 0.479 |
| walker |  | 409 | 66 | README.md section #0 |  |  | 0.479 |
| ns | 424 |  | 139 | lib.rs module declarations | 2.1 |  | 0.392 |
| walker |  | 458 | 49 | listing of 'src' |  |  | 0.550 |
| ns | 458 |  | 34 | Error struct definition | 2.2 |  | 0.550 |
| ns | 481 |  | 23 | Result type alias | 2.3 |  | 0.545 |
| ns | 541 |  | 60 | Chain struct | 2.4 |  | 0.524 |
| walker |  | 546 | 88 | pub-item names surface in src/lib.rs |  |  | 0.539 |
| walker |  | 546 | 0 | pub item at src/lib.rs:468 |  |  | 0.539 |
| walker |  | 546 | 0 | pub item at src/lib.rs:650 |  |  | 0.539 |
| walker |  | 561 | 15 | pub item at src/lib.rs:390 |  |  | 0.557 |
| walker |  | 580 | 19 | pub item at src/lib.rs:415 |  |  | 0.569 |
| walker |  | 588 | 8 | pub item body at src/lib.rs:650 body 651 |  |  | 0.569 |
| ns | 697 |  | 156 | Error vs Box<dyn Error>: three differences | 2.5 | 2.2 | 0.529 |
| walker |  | 741 | 153 | pub item at src/lib.rs:616 |  |  | 0.537 |
| walker |  | 819 | 78 | macro_export names across src |  |  | 0.537 |
| ns | 858 |  | 161 | Error::* method signatures (locations) | 2.6 |  | 0.492 |
| walker |  | 860 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.492 |
| ns | 1036 |  | 178 | Context trait signature | 2.7 |  | 0.536 |
| ns | 1130 |  | 94 | Crate-level public items (locations) | 2.8 |  | 0.513 |
| walker |  | 1134 | 274 | crate-doc lede in src/lib.rs |  |  | 0.547 |
| walker |  | 1199 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.547 |
| ns | 1302 |  | 172 | Macro export locations | 2.9 |  | 0.512 |
| walker |  | 1372 | 173 | mod/use plumbing in src/lib.rs |  |  | 0.554 |
| walker |  | 1387 | 15 | plaintext config .gitignore |  |  | 0.554 |
| walker |  | 1400 | 13 | pub-item names surface in src/backtrace.rs |  |  | 0.554 |
| walker |  | 1536 | 136 | macro_export body at src/macros.rs:58 |  |  | 0.554 |
| ns | 1542 |  | 240 | Cargo features + dev-deps | 2.10 | 1.4 | 0.519 |
| walker |  | 1710 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.572 |
| ns | 1757 |  | 215 | Display reprs: `{}` and `{:#}` | 2.11 | 2.5 | 0.538 |
| walker |  | 1917 | 207 | [dependencies] in Cargo.toml |  |  | 0.603 |
| walker |  | 1940 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.603 |
| ns | 1979 |  | 222 | anyhow! macro body | 2.12 | 2.9 | 0.562 |
| walker |  | 2160 | 220 | macro_export body at src/macros.rs:204 |  |  | 0.627 |
| walker |  | 2191 | 31 | pub-item names surface in src/chain.rs |  |  | 0.627 |
| walker |  | 2206 | 15 | pub item at src/chain.rs:11 |  |  | 0.627 |
| walker |  | 2428 | 222 | pub-item doc lede at src/lib.rs:468 |  |  | 0.627 |
| walker |  | 2462 | 34 | pub-item names surface in src/error.rs |  |  | 0.627 |
| walker |  | 2485 | 23 | pub item at src/error.rs:952 |  |  | 0.627 |
| ns | 2558 |  | 579 | Context impls for Result and Option | 2.13 | 2.7 | 0.548 |
| walker |  | 2771 | 286 | pub-item doc lede at src/lib.rs:650 |  |  | 0.548 |
| walker |  | 2895 | 124 | README.md section #8 |  |  | 0.548 |
| walker |  | 2985 | 90 | listing of 'tests' |  |  | 0.625 |
| ns | 3026 |  | 468 | Debug repr `{:?}` (default for `fn main`) | 2.14 | 2.5 | 0.580 |
| walker |  | 3035 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.580 |
| walker |  | 3046 | 11 | pub item at src/ptr.rs:181 |  |  | 0.580 |
| walker |  | 3082 | 36 | pub item at src/ptr.rs:6 |  |  | 0.580 |
| walker |  | 3132 | 50 | pub item at src/ptr.rs:64 |  |  | 0.580 |
| walker |  | 3183 | 51 | pub item at src/ptr.rs:125 |  |  | 0.581 |
| walker |  | 3235 | 52 | pub-item names surface in src/wrapper.rs |  |  | 0.581 |
| walker |  | 3308 | 73 | pub item at src/error.rs:934 |  |  | 0.581 |
| ns | 3399 |  | 373 | Debug repr `{:#?}` + manual chain rendering | 2.15 | 2.5 | 0.543 |
| walker |  | 3508 | 200 | README.md section #7 |  |  | 0.543 |
| walker |  | 3601 | 93 | pub item at src/chain.rs:16 |  |  | 0.544 |
| ns | 3672 |  | 273 | error.rs item locations | 3.1 |  | 0.526 |
| walker |  | 3674 | 73 | pub-item names surface in src/kind.rs |  |  | 0.526 |
| walker |  | 3763 | 89 | pub-item names surface in src/nightly.rs |  |  | 0.526 |
| walker |  | 3763 | 0 | pub item at src/nightly.rs:41 |  |  | 0.526 |
| walker |  | 3763 | 0 | pub item at src/nightly.rs:52 |  |  | 0.526 |
| walker |  | 3763 | 0 | pub item at src/nightly.rs:56 |  |  | 0.526 |
| walker |  | 3773 | 10 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.527 |
| walker |  | 3784 | 11 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.527 |
| walker |  | 3796 | 12 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.527 |
| walker |  | 3878 | 82 | listing of 'tests/ui' |  |  | 0.527 |
| ns | 3934 |  | 262 | context.rs item locations | 3.2 |  | 0.509 |
| ns | 4232 |  | 298 | chain.rs item locations | 3.3 |  | 0.501 |
| ns | 4770 |  | 538 | kind.rs tagged-dispatch types | 3.4 |  | 0.467 |
| ns | 4950 |  | 180 | wrapper.rs: MessageError / DisplayError / BoxedError | 3.5 |  | 0.460 |
| ns | 5232 |  | 282 | fmt.rs: ErrorImpl::display body + debug/Indented locations | 3.6 |  | 0.447 |
| walker |  | 5442 | 1564 | crate-doc body in src/lib.rs |  |  | 0.447 |
| walker |  | 5446 | 4 | listing of 'tests/common' |  |  | 0.447 |
| walker |  | 5450 | 4 | listing of 'tests/drop' |  |  | 0.447 |
| walker |  | 5471 | 21 | pub-item names surface in tests/drop/mod.rs |  |  | 0.447 |
| walker |  | 5485 | 14 | pub item at tests/drop/mod.rs:26 |  |  | 0.447 |
| walker |  | 5500 | 15 | pub item at tests/drop/mod.rs:9 |  |  | 0.447 |
| ns | 5548 |  | 316 | ptr.rs internal pointer newtypes | 3.7 |  | 0.454 |
| walker |  | 5759 | 259 | README.md section #9 |  |  | 0.454 |
| walker |  | 5804 | 45 | pub-item names surface in tests/common/mod.rs |  |  | 0.454 |
| walker |  | 5804 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.454 |
| walker |  | 5804 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.454 |
| walker |  | 5804 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.454 |
| walker |  | 5812 | 8 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.454 |
| walker |  | 5826 | 14 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.454 |
| walker |  | 5848 | 22 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.454 |
| walker |  | 5869 | 21 | mod/use plumbing in tests/common/mod.rs |  |  | 0.454 |
| walker |  | 5893 | 24 | pub item at src/ensure.rs:10 |  |  | 0.454 |
| walker |  | 5917 | 24 | pub item at src/ensure.rs:25 |  |  | 0.454 |
| ns | 6006 |  | 458 | backtrace.rs cfg landscape (locations) | 3.8 |  | 0.436 |
| walker |  | 6068 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.436 |
| walker |  | 6192 | 124 | pub-item doc body at src/lib.rs:468 |  |  | 0.436 |
| ns | 6235 |  | 229 | nightly.rs locations + signatures | 3.9 |  | 0.435 |
| walker |  | 6312 | 120 | README.md section #6 |  |  | 0.435 |
| walker |  | 6350 | 38 | pub item at src/kind.rs:80 |  |  | 0.437 |
| walker |  | 6488 | 138 | README.md section #3 |  |  | 0.437 |
| walker |  | 6528 | 40 | pub item at src/kind.rs:58 |  |  | 0.440 |
| walker |  | 6568 | 40 | pub item at src/kind.rs:104 |  |  | 0.444 |
| ns | 6577 |  | 342 | ensure.rs orientation header (locations) | 3.10 |  | 0.435 |
| walker |  | 6582 | 14 | listing of 'tests/crate' |  |  | 0.435 |
| walker |  | 6649 | 67 | [package] in tests/crate/Cargo.toml |  |  | 0.435 |
| walker |  | 6703 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.435 |
| ns | 6817 |  | 240 | tests/common + drop helpers | 4.1 |  | 0.438 |
| walker |  | 6902 | 199 | README.md section #1 |  |  | 0.438 |
| walker |  | 6971 | 69 | pub-item names surface in tests/test_ffi.rs |  |  | 0.438 |
| walker |  | 6971 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.438 |
| walker |  | 6971 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.438 |
| walker |  | 6971 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.438 |
| walker |  | 6980 | 9 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.438 |
| walker |  | 6990 | 10 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.438 |
| walker |  | 7003 | 13 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.438 |
| ns | 7104 |  | 287 | test_context: Low/Mid/High chain helper + fns | 4.2 |  | 0.425 |
| walker |  | 7208 | 205 | README.md section #5 |  |  | 0.425 |
| walker |  | 7248 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.425 |
| ns | 7307 |  | 203 | test_downcast / test_repr / test_convert fns (locations) | 4.3 |  | 0.418 |
| ns | 7450 |  | 143 | test_macros + test_chain + test_source fns | 4.4 |  | 0.412 |
| walker |  | 7463 | 215 | README.md section #4 |  |  | 0.412 |
| walker |  | 7587 | 124 | impl method sigs in tests/drop/mod.rs |  |  | 0.419 |
| ns | 7686 |  | 236 | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | 4.5 |  | 0.412 |
| walker |  | 7828 | 241 | README.md section #2 |  |  | 0.412 |
| walker |  | 7854 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.412 |
| ns | 7952 |  | 266 | test_autotrait + test_boxed + test_ffi + test_backtrace | 4.6 |  | 0.404 |
| ns | 8056 |  | 104 | tests/ui + tests/crate listings | 4.7 |  | 0.421 |
| ns | 8313 |  | 257 | ErrorImpl<E> layout + vtable() reader | 5.1 | 3.1 | 0.418 |
| ns | 8514 |  | 201 | Chain DoubleEndedIterator buffering | 5.2 | 3.3 | 0.412 |
| ns | 8742 |  | 228 | kind.rs autoref-precedence note | 5.3 | 3.4 | 0.407 |
| walker |  | 9014 | 1160 | pub-item doc body at src/lib.rs:616 |  |  | 0.407 |
| ns | 9288 |  | 546 | ErrorImpl::debug body | 5.4 | 3.6 | 0.394 |
| ns | 9528 |  | 240 | __fancy_ensure! macro body | 5.5 | 3.10 | 0.387 |
| ns | 9798 |  | 270 | ensure!: render `{msg} ({lhs} vs {rhs})` | 5.6 | 3.10 | 0.382 |
| ns | 9946 |  | 148 | build.rs cfg pipeline (rustc version + cfgs) | 5.7 |  | 0.379 |
