Score(3000)=0.697 I=0.872 C=0.557 ns_rows≤3K=18/44 (reached=11 partial=1 missing=6)

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
| walker |  | 407 | 76 | README.md section #0 |  |  | 0.479 |
| ns | 424 |  | 139 | lib.rs module declarations | 2.1 |  | 0.392 |
| walker |  | 456 | 49 | listing of 'src' |  |  | 0.569 |
| ns | 458 |  | 34 | Error struct definition | 2.2 |  | 0.550 |
| ns | 481 |  | 23 | Result type alias | 2.3 |  | 0.545 |
| ns | 541 |  | 60 | Chain struct | 2.4 |  | 0.524 |
| walker |  | 544 | 88 | pub-item names surface in src/lib.rs |  |  | 0.539 |
| walker |  | 544 | 0 | pub item at src/lib.rs:468 |  |  | 0.539 |
| walker |  | 555 | 11 | pub item at src/lib.rs:650 |  |  | 0.539 |
| walker |  | 563 | 8 | pub item body at src/lib.rs:650 body 651 |  |  | 0.539 |
| walker |  | 587 | 24 | pub item at src/lib.rs:390 |  |  | 0.576 |
| walker |  | 614 | 27 | pub item at src/lib.rs:415 |  |  | 0.597 |
| ns | 697 |  | 156 | Error vs Box<dyn Error>: three differences | 2.5 | 2.2 | 0.555 |
| walker |  | 772 | 158 | pub item at src/lib.rs:616 |  |  | 0.564 |
| walker |  | 850 | 78 | macro_export names across src |  |  | 0.564 |
| ns | 858 |  | 161 | Error::* method signatures (locations) | 2.6 |  | 0.516 |
| walker |  | 1028 | 178 | mod/use plumbing in src/lib.rs |  |  | 0.568 |
| ns | 1036 |  | 178 | Context trait signature | 2.7 |  | 0.606 |
| walker |  | 1069 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.606 |
| ns | 1130 |  | 94 | Crate-level public items (locations) | 2.8 |  | 0.592 |
| ns | 1302 |  | 172 | Macro export locations | 2.9 |  | 0.554 |
| walker |  | 1343 | 274 | crate-doc lede in src/lib.rs |  |  | 0.586 |
| walker |  | 1408 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.586 |
| ns | 1542 |  | 240 | Cargo features + dev-deps | 2.10 | 1.4 | 0.548 |
| walker |  | 1545 | 137 | impl method sigs in src/context.rs |  |  | 0.548 |
| walker |  | 1681 | 136 | macro_export body at src/macros.rs:58 |  |  | 0.548 |
| ns | 1757 |  | 215 | Display reprs: `{}` and `{:#}` | 2.11 | 2.5 | 0.516 |
| walker |  | 1855 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.565 |
| ns | 1979 |  | 222 | anyhow! macro body | 2.12 | 2.9 | 0.527 |
| walker |  | 2067 | 212 | [dependencies] in Cargo.toml |  |  | 0.590 |
| walker |  | 2090 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.590 |
| walker |  | 2310 | 220 | macro_export body at src/macros.rs:204 |  |  | 0.653 |
| walker |  | 2348 | 38 | impl method sigs in src/chain.rs |  |  | 0.653 |
| walker |  | 2438 | 90 | listing of 'tests' |  |  | 0.741 |
| walker |  | 2469 | 31 | pub-item names surface in src/chain.rs |  |  | 0.741 |
| walker |  | 2484 | 15 | pub item at src/chain.rs:11 |  |  | 0.741 |
| ns | 2558 |  | 579 | Context impls for Result and Option | 2.13 | 2.7 | 0.651 |
| walker |  | 2808 | 324 | impl method sigs in src/error.rs |  |  | 0.697 |
| ns | 3026 |  | 468 | Debug repr `{:?}` (default for `fn main`) | 2.14 | 2.5 | 0.647 |
| walker |  | 3198 | 390 | impl method sigs in src/ptr.rs |  |  | 0.647 |
| ns | 3399 |  | 373 | Debug repr `{:#?}` + manual chain rendering | 2.15 | 2.5 | 0.604 |
| walker |  | 3420 | 222 | pub-item doc lede at src/lib.rs:468 |  |  | 0.604 |
| walker |  | 3454 | 34 | pub-item names surface in src/error.rs |  |  | 0.604 |
| walker |  | 3485 | 31 | pub item at src/error.rs:952 |  |  | 0.604 |
| ns | 3672 |  | 273 | error.rs item locations | 3.1 |  | 0.584 |
| walker |  | 3771 | 286 | pub-item doc lede at src/lib.rs:650 |  |  | 0.584 |
| walker |  | 3908 | 137 | README.md section #8 |  |  | 0.584 |
| ns | 3934 |  | 262 | context.rs item locations | 3.2 |  | 0.564 |
| walker |  | 3958 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.564 |
| walker |  | 3969 | 11 | pub item at src/ptr.rs:181 |  |  | 0.564 |
| walker |  | 4014 | 45 | pub item at src/ptr.rs:6 |  |  | 0.565 |
| walker |  | 4073 | 59 | pub item at src/ptr.rs:64 |  |  | 0.566 |
| walker |  | 4133 | 60 | pub item at src/ptr.rs:125 |  |  | 0.567 |
| walker |  | 4185 | 52 | pub-item names surface in src/wrapper.rs |  |  | 0.567 |
| walker |  | 4194 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.567 |
| walker |  | 4203 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.567 |
| walker |  | 4212 | 9 | pub item at src/wrapper.rs:58 |  |  | 0.567 |
| ns | 4232 |  | 298 | chain.rs item locations | 3.3 |  | 0.548 |
| walker |  | 4313 | 101 | pub item at src/chain.rs:16 |  |  | 0.562 |
| walker |  | 4394 | 81 | pub item at src/error.rs:934 |  |  | 0.562 |
| walker |  | 4612 | 218 | README.md section #7 |  |  | 0.562 |
| walker |  | 4704 | 92 | impl method sigs in src/kind.rs |  |  | 0.562 |
| ns | 4770 |  | 538 | kind.rs tagged-dispatch types | 3.4 |  | 0.525 |
| walker |  | 4777 | 73 | pub-item names surface in src/kind.rs |  |  | 0.530 |
| walker |  | 4859 | 82 | listing of 'tests/ui' |  |  | 0.531 |
| walker |  | 4948 | 89 | pub-item names surface in src/nightly.rs |  |  | 0.531 |
| walker |  | 4948 | 0 | pub item at src/nightly.rs:41 |  |  | 0.531 |
| walker |  | 4948 | 0 | pub item at src/nightly.rs:52 |  |  | 0.531 |
| walker |  | 4948 | 0 | pub item at src/nightly.rs:56 |  |  | 0.531 |
| ns | 4950 |  | 180 | wrapper.rs: MessageError / DisplayError / BoxedError | 3.5 |  | 0.523 |
| walker |  | 4958 | 10 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.523 |
| walker |  | 4969 | 11 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.523 |
| walker |  | 4981 | 12 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.523 |
| walker |  | 4989 | 8 | listing of '.github' |  |  | 0.523 |
| walker |  | 4993 | 4 | listing of '.github/workflows' |  |  | 0.523 |
| ns | 5232 |  | 282 | fmt.rs: ErrorImpl::display body + debug/Indented locations | 3.6 |  | 0.507 |
| ns | 5548 |  | 316 | ptr.rs internal pointer newtypes | 3.7 |  | 0.530 |
| ns | 6006 |  | 458 | backtrace.rs cfg landscape (locations) | 3.8 |  | 0.508 |
| ns | 6235 |  | 229 | nightly.rs locations + signatures | 3.9 |  | 0.505 |
| walker |  | 6557 | 1564 | crate-doc body in src/lib.rs |  |  | 0.505 |
| ns | 6577 |  | 342 | ensure.rs orientation header (locations) | 3.10 |  | 0.492 |
| ns | 6817 |  | 240 | tests/common + drop helpers | 4.1 |  | 0.480 |
| walker |  | 6849 | 292 | README.md section #9 |  |  | 0.480 |
| walker |  | 6984 | 135 | README.md section #6 |  |  | 0.480 |
| ns | 7104 |  | 287 | test_context: Low/Mid/High chain helper + fns | 4.2 |  | 0.467 |
| walker |  | 7127 | 143 | README.md section #3 |  |  | 0.467 |
| walker |  | 7151 | 24 | pub item at src/ensure.rs:10 |  |  | 0.468 |
| walker |  | 7175 | 24 | pub item at src/ensure.rs:25 |  |  | 0.469 |
| ns | 7307 |  | 203 | test_downcast / test_repr / test_convert fns (locations) | 4.3 |  | 0.461 |
| walker |  | 7326 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.461 |
| walker |  | 7450 | 124 | pub-item doc body at src/lib.rs:468 |  |  | 0.455 |
| ns | 7450 |  | 143 | test_macros + test_chain + test_source fns | 4.4 |  | 0.455 |
| walker |  | 7665 | 215 | README.md section #5 |  |  | 0.455 |
| ns | 7686 |  | 236 | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | 4.5 |  | 0.447 |
| walker |  | 7884 | 219 | README.md section #1 |  |  | 0.447 |
| ns | 7952 |  | 266 | test_autotrait + test_boxed + test_ffi + test_backtrace | 4.6 |  | 0.439 |
| ns | 8056 |  | 104 | tests/ui + tests/crate listings | 4.7 |  | 0.444 |
| walker |  | 8109 | 225 | README.md section #4 |  |  | 0.444 |
| walker |  | 8147 | 38 | pub item at src/kind.rs:80 |  |  | 0.447 |
| ns | 8313 |  | 257 | ErrorImpl<E> layout + vtable() reader | 5.1 | 3.1 | 0.444 |
| walker |  | 8413 | 266 | README.md section #2 |  |  | 0.444 |
| walker |  | 8453 | 40 | pub item at src/kind.rs:58 |  |  | 0.448 |
| walker |  | 8493 | 40 | pub item at src/kind.rs:104 |  |  | 0.452 |
| ns | 8514 |  | 201 | Chain DoubleEndedIterator buffering | 5.2 | 3.3 | 0.446 |
| walker |  | 8562 | 69 | pub-item names surface in tests/test_ffi.rs |  |  | 0.446 |
| walker |  | 8562 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.446 |
| walker |  | 8562 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.446 |
| walker |  | 8562 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.446 |
| walker |  | 8571 | 9 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.446 |
| walker |  | 8581 | 10 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.446 |
| walker |  | 8594 | 13 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.446 |
| walker |  | 8608 | 14 | listing of 'tests/crate' |  |  | 0.452 |
| walker |  | 8675 | 67 | [package] in tests/crate/Cargo.toml |  |  | 0.452 |
| walker |  | 8715 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.452 |
| walker |  | 8719 | 4 | listing of 'tests/common' |  |  | 0.454 |
| ns | 8742 |  | 228 | kind.rs autoref-precedence note | 5.3 | 3.4 | 0.449 |
| walker |  | 8764 | 45 | pub-item names surface in tests/common/mod.rs |  |  | 0.450 |
| walker |  | 8764 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.450 |
| walker |  | 8764 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.450 |
| walker |  | 8764 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.450 |
| walker |  | 8772 | 8 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.450 |
| walker |  | 8786 | 14 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.451 |
| walker |  | 8808 | 22 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.452 |
| walker |  | 8829 | 21 | mod/use plumbing in tests/common/mod.rs |  |  | 0.454 |
| walker |  | 8833 | 4 | listing of 'tests/drop' |  |  | 0.456 |
| walker |  | 8854 | 21 | pub-item names surface in tests/drop/mod.rs |  |  | 0.458 |
| walker |  | 8876 | 22 | pub item at tests/drop/mod.rs:26 |  |  | 0.460 |
| walker |  | 8899 | 23 | pub item at tests/drop/mod.rs:9 |  |  | 0.464 |
| walker |  | 8953 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.464 |
| walker |  | 9077 | 124 | impl method sigs in tests/drop/mod.rs |  |  | 0.469 |
| walker |  | 9103 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.469 |
| walker |  | 9118 | 15 | plaintext config .gitignore |  |  | 0.469 |
| ns | 9288 |  | 546 | ErrorImpl::debug body | 5.4 | 3.6 | 0.454 |
| ns | 9528 |  | 240 | __fancy_ensure! macro body | 5.5 | 3.10 | 0.446 |
| ns | 9798 |  | 270 | ensure!: render `{msg} ({lhs} vs {rhs})` | 5.6 | 3.10 | 0.440 |
| ns | 9946 |  | 148 | build.rs cfg pipeline (rustc version + cfgs) | 5.7 |  | 0.437 |
