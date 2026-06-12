Score(3000)=0.676 I=0.860 C=0.531 ns_rows≤3K=18/44 (reached=10 partial=2 missing=6)

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
| walker |  | 1497 | 137 | impl method sigs in src/context.rs |  |  | 0.555 |
| ns | 1542 |  | 240 | Cargo features + dev-deps | 2.10 | 1.4 | 0.519 |
| walker |  | 1633 | 136 | macro_export body at src/macros.rs:58 |  |  | 0.519 |
| ns | 1757 |  | 215 | Display reprs: `{}` and `{:#}` | 2.11 | 2.5 | 0.489 |
| walker |  | 1807 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.539 |
| ns | 1979 |  | 222 | anyhow! macro body | 2.12 | 2.9 | 0.502 |
| walker |  | 2014 | 207 | [dependencies] in Cargo.toml |  |  | 0.563 |
| walker |  | 2037 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.563 |
| walker |  | 2257 | 220 | macro_export body at src/macros.rs:204 |  |  | 0.628 |
| walker |  | 2295 | 38 | impl method sigs in src/chain.rs |  |  | 0.628 |
| walker |  | 2385 | 90 | listing of 'tests' |  |  | 0.716 |
| walker |  | 2416 | 31 | pub-item names surface in src/chain.rs |  |  | 0.716 |
| walker |  | 2431 | 15 | pub item at src/chain.rs:11 |  |  | 0.716 |
| ns | 2558 |  | 579 | Context impls for Result and Option | 2.13 | 2.7 | 0.630 |
| walker |  | 2755 | 324 | impl method sigs in src/error.rs |  |  | 0.676 |
| ns | 3026 |  | 468 | Debug repr `{:?}` (default for `fn main`) | 2.14 | 2.5 | 0.627 |
| walker |  | 3145 | 390 | impl method sigs in src/ptr.rs |  |  | 0.627 |
| walker |  | 3367 | 222 | pub-item doc lede at src/lib.rs:468 |  |  | 0.627 |
| ns | 3399 |  | 373 | Debug repr `{:#?}` + manual chain rendering | 2.15 | 2.5 | 0.586 |
| walker |  | 3401 | 34 | pub-item names surface in src/error.rs |  |  | 0.586 |
| walker |  | 3424 | 23 | pub item at src/error.rs:952 |  |  | 0.586 |
| ns | 3672 |  | 273 | error.rs item locations | 3.1 |  | 0.567 |
| walker |  | 3710 | 286 | pub-item doc lede at src/lib.rs:650 |  |  | 0.567 |
| walker |  | 3834 | 124 | README.md section #8 |  |  | 0.567 |
| walker |  | 3884 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.567 |
| walker |  | 3895 | 11 | pub item at src/ptr.rs:181 |  |  | 0.567 |
| walker |  | 3931 | 36 | pub item at src/ptr.rs:6 |  |  | 0.567 |
| ns | 3934 |  | 262 | context.rs item locations | 3.2 |  | 0.548 |
| walker |  | 3981 | 50 | pub item at src/ptr.rs:64 |  |  | 0.548 |
| walker |  | 4032 | 51 | pub item at src/ptr.rs:125 |  |  | 0.549 |
| walker |  | 4084 | 52 | pub-item names surface in src/wrapper.rs |  |  | 0.549 |
| walker |  | 4157 | 73 | pub item at src/error.rs:934 |  |  | 0.549 |
| ns | 4232 |  | 298 | chain.rs item locations | 3.3 |  | 0.531 |
| walker |  | 4357 | 200 | README.md section #7 |  |  | 0.531 |
| walker |  | 4450 | 93 | pub item at src/chain.rs:16 |  |  | 0.545 |
| walker |  | 4542 | 92 | impl method sigs in src/kind.rs |  |  | 0.545 |
| walker |  | 4615 | 73 | pub-item names surface in src/kind.rs |  |  | 0.546 |
| walker |  | 4697 | 82 | listing of 'tests/ui' |  |  | 0.546 |
| ns | 4770 |  | 538 | kind.rs tagged-dispatch types | 3.4 |  | 0.515 |
| walker |  | 4786 | 89 | pub-item names surface in src/nightly.rs |  |  | 0.515 |
| walker |  | 4786 | 0 | pub item at src/nightly.rs:41 |  |  | 0.515 |
| walker |  | 4786 | 0 | pub item at src/nightly.rs:52 |  |  | 0.515 |
| walker |  | 4786 | 0 | pub item at src/nightly.rs:56 |  |  | 0.515 |
| walker |  | 4796 | 10 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.515 |
| walker |  | 4807 | 11 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.515 |
| walker |  | 4819 | 12 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.515 |
| walker |  | 4827 | 8 | listing of '.github' |  |  | 0.515 |
| walker |  | 4831 | 4 | listing of '.github/workflows' |  |  | 0.515 |
| ns | 4950 |  | 180 | wrapper.rs: MessageError / DisplayError / BoxedError | 3.5 |  | 0.507 |
| ns | 5232 |  | 282 | fmt.rs: ErrorImpl::display body + debug/Indented locations | 3.6 |  | 0.492 |
| ns | 5548 |  | 316 | ptr.rs internal pointer newtypes | 3.7 |  | 0.507 |
| ns | 6006 |  | 458 | backtrace.rs cfg landscape (locations) | 3.8 |  | 0.486 |
| ns | 6235 |  | 229 | nightly.rs locations + signatures | 3.9 |  | 0.483 |
| walker |  | 6395 | 1564 | crate-doc body in src/lib.rs |  |  | 0.483 |
| ns | 6577 |  | 342 | ensure.rs orientation header (locations) | 3.10 |  | 0.471 |
| walker |  | 6654 | 259 | README.md section #9 |  |  | 0.471 |
| walker |  | 6774 | 120 | README.md section #6 |  |  | 0.471 |
| ns | 6817 |  | 240 | tests/common + drop helpers | 4.1 |  | 0.460 |
| walker |  | 6912 | 138 | README.md section #3 |  |  | 0.460 |
| walker |  | 6936 | 24 | pub item at src/ensure.rs:10 |  |  | 0.461 |
| walker |  | 6960 | 24 | pub item at src/ensure.rs:25 |  |  | 0.462 |
| ns | 7104 |  | 287 | test_context: Low/Mid/High chain helper + fns | 4.2 |  | 0.449 |
| walker |  | 7111 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.449 |
| ns | 7307 |  | 203 | test_downcast / test_repr / test_convert fns (locations) | 4.3 |  | 0.441 |
| walker |  | 7310 | 199 | README.md section #1 |  |  | 0.441 |
| walker |  | 7434 | 124 | pub-item doc body at src/lib.rs:468 |  |  | 0.441 |
| ns | 7450 |  | 143 | test_macros + test_chain + test_source fns | 4.4 |  | 0.435 |
| walker |  | 7639 | 205 | README.md section #5 |  |  | 0.435 |
| ns | 7686 |  | 236 | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | 4.5 |  | 0.428 |
| walker |  | 7854 | 215 | README.md section #4 |  |  | 0.428 |
| ns | 7952 |  | 266 | test_autotrait + test_boxed + test_ffi + test_backtrace | 4.6 |  | 0.420 |
| ns | 8056 |  | 104 | tests/ui + tests/crate listings | 4.7 |  | 0.426 |
| walker |  | 8095 | 241 | README.md section #2 |  |  | 0.426 |
| walker |  | 8133 | 38 | pub item at src/kind.rs:80 |  |  | 0.429 |
| walker |  | 8173 | 40 | pub item at src/kind.rs:58 |  |  | 0.433 |
| walker |  | 8213 | 40 | pub item at src/kind.rs:104 |  |  | 0.438 |
| walker |  | 8282 | 69 | pub-item names surface in tests/test_ffi.rs |  |  | 0.438 |
| walker |  | 8282 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.438 |
| walker |  | 8282 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.438 |
| walker |  | 8282 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.438 |
| walker |  | 8291 | 9 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.438 |
| walker |  | 8301 | 10 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.438 |
| ns | 8313 |  | 257 | ErrorImpl<E> layout + vtable() reader | 5.1 | 3.1 | 0.435 |
| walker |  | 8314 | 13 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.435 |
| walker |  | 8328 | 14 | listing of 'tests/crate' |  |  | 0.441 |
| walker |  | 8395 | 67 | [package] in tests/crate/Cargo.toml |  |  | 0.441 |
| walker |  | 8435 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.441 |
| walker |  | 8439 | 4 | listing of 'tests/common' |  |  | 0.443 |
| walker |  | 8484 | 45 | pub-item names surface in tests/common/mod.rs |  |  | 0.444 |
| walker |  | 8484 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.444 |
| walker |  | 8484 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.444 |
| walker |  | 8484 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.444 |
| walker |  | 8492 | 8 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.444 |
| walker |  | 8506 | 14 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.445 |
| ns | 8514 |  | 201 | Chain DoubleEndedIterator buffering | 5.2 | 3.3 | 0.438 |
| walker |  | 8528 | 22 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.439 |
| walker |  | 8549 | 21 | mod/use plumbing in tests/common/mod.rs |  |  | 0.441 |
| walker |  | 8553 | 4 | listing of 'tests/drop' |  |  | 0.443 |
| walker |  | 8574 | 21 | pub-item names surface in tests/drop/mod.rs |  |  | 0.446 |
| walker |  | 8588 | 14 | pub item at tests/drop/mod.rs:26 |  |  | 0.447 |
| walker |  | 8603 | 15 | pub item at tests/drop/mod.rs:9 |  |  | 0.449 |
| walker |  | 8657 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.449 |
| ns | 8742 |  | 228 | kind.rs autoref-precedence note | 5.3 | 3.4 | 0.444 |
| walker |  | 8781 | 124 | impl method sigs in tests/drop/mod.rs |  |  | 0.449 |
| walker |  | 8807 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.449 |
| walker |  | 8822 | 15 | plaintext config .gitignore |  |  | 0.449 |
| ns | 9288 |  | 546 | ErrorImpl::debug body | 5.4 | 3.6 | 0.434 |
| ns | 9528 |  | 240 | __fancy_ensure! macro body | 5.5 | 3.10 | 0.427 |
| ns | 9798 |  | 270 | ensure!: render `{msg} ({lhs} vs {rhs})` | 5.6 | 3.10 | 0.422 |
| ns | 9946 |  | 148 | build.rs cfg pipeline (rustc version + cfgs) | 5.7 |  | 0.419 |
| walker |  | 9982 | 1160 | pub-item doc body at src/lib.rs:616 |  |  | 0.419 |
