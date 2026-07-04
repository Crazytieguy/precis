Score(3000)=0.689 I=0.868 C=0.547 ns_rows≤3K=18/44 (reached=11 partial=1 missing=6)

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
| ns | 541 |  | 60 | Chain struct | 2.4 |  | 0.539 |
| walker |  | 545 | 11 | pub item at src/lib.rs:650 |  |  | 0.539 |
| walker |  | 553 | 8 | pub item body at src/lib.rs:650 body 651 |  |  | 0.539 |
| walker |  | 577 | 24 | pub item at src/lib.rs:390 |  |  | 0.576 |
| walker |  | 604 | 27 | pub item at src/lib.rs:415 |  |  | 0.597 |
| ns | 697 |  | 156 | Error vs Box<dyn Error>: three differences | 2.5 | 2.2 | 0.555 |
| walker |  | 757 | 153 | pub item at src/lib.rs:616 |  |  | 0.563 |
| walker |  | 835 | 78 | macro_export names across src |  |  | 0.563 |
| ns | 858 |  | 161 | Error::* method signatures (locations) | 2.6 |  | 0.516 |
| walker |  | 1008 | 173 | mod/use plumbing in src/lib.rs |  |  | 0.563 |
| ns | 1036 |  | 178 | Context trait signature | 2.7 |  | 0.596 |
| walker |  | 1049 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.596 |
| ns | 1130 |  | 94 | Crate-level public items (locations) | 2.8 |  | 0.582 |
| ns | 1302 |  | 172 | Macro export locations | 2.9 |  | 0.545 |
| walker |  | 1323 | 274 | crate-doc lede in src/lib.rs |  |  | 0.577 |
| walker |  | 1388 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.577 |
| walker |  | 1478 | 90 | listing of 'tests' |  |  | 0.693 |
| ns | 1542 |  | 240 | Cargo features + dev-deps | 2.10 | 1.4 | 0.648 |
| walker |  | 1615 | 137 | impl method sigs in src/context.rs |  |  | 0.648 |
| walker |  | 1751 | 136 | macro_export body at src/macros.rs:58 |  |  | 0.648 |
| ns | 1757 |  | 215 | Display reprs: `{}` and `{:#}` | 2.11 | 2.5 | 0.610 |
| walker |  | 1925 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.657 |
| ns | 1979 |  | 222 | anyhow! macro body | 2.12 | 2.9 | 0.612 |
| walker |  | 2132 | 207 | [dependencies] in Cargo.toml |  |  | 0.669 |
| walker |  | 2155 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.669 |
| walker |  | 2375 | 220 | macro_export body at src/macros.rs:204 |  |  | 0.731 |
| walker |  | 2413 | 38 | impl method sigs in src/chain.rs |  |  | 0.731 |
| walker |  | 2444 | 31 | pub-item names surface in src/chain.rs |  |  | 0.731 |
| walker |  | 2459 | 15 | pub item at src/chain.rs:11 |  |  | 0.731 |
| ns | 2558 |  | 579 | Context impls for Result and Option | 2.13 | 2.7 | 0.643 |
| walker |  | 2783 | 324 | impl method sigs in src/error.rs |  |  | 0.689 |
| ns | 3026 |  | 468 | Debug repr `{:?}` (default for `fn main`) | 2.14 | 2.5 | 0.639 |
| walker |  | 3173 | 390 | impl method sigs in src/ptr.rs |  |  | 0.639 |
| walker |  | 3395 | 222 | pub-item doc lede at src/lib.rs:468 |  |  | 0.639 |
| ns | 3399 |  | 373 | Debug repr `{:#?}` + manual chain rendering | 2.15 | 2.5 | 0.597 |
| walker |  | 3429 | 34 | pub-item names surface in src/error.rs |  |  | 0.597 |
| walker |  | 3460 | 31 | pub item at src/error.rs:952 |  |  | 0.597 |
| ns | 3672 |  | 273 | error.rs item locations | 3.1 |  | 0.577 |
| walker |  | 3746 | 286 | pub-item doc lede at src/lib.rs:650 |  |  | 0.577 |
| walker |  | 3870 | 124 | README.md section #8 |  |  | 0.577 |
| walker |  | 3920 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.578 |
| walker |  | 3931 | 11 | pub item at src/ptr.rs:181 |  |  | 0.578 |
| ns | 3934 |  | 262 | context.rs item locations | 3.2 |  | 0.558 |
| walker |  | 3976 | 45 | pub item at src/ptr.rs:6 |  |  | 0.558 |
| walker |  | 4035 | 59 | pub item at src/ptr.rs:64 |  |  | 0.559 |
| walker |  | 4095 | 60 | pub item at src/ptr.rs:125 |  |  | 0.560 |
| walker |  | 4147 | 52 | pub-item names surface in src/wrapper.rs |  |  | 0.560 |
| walker |  | 4156 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.560 |
| walker |  | 4165 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.560 |
| walker |  | 4174 | 9 | pub item at src/wrapper.rs:58 |  |  | 0.560 |
| ns | 4232 |  | 298 | chain.rs item locations | 3.3 |  | 0.542 |
| walker |  | 4374 | 200 | README.md section #7 |  |  | 0.542 |
| walker |  | 4475 | 101 | pub item at src/chain.rs:16 |  |  | 0.556 |
| walker |  | 4556 | 81 | pub item at src/error.rs:934 |  |  | 0.556 |
| walker |  | 4648 | 92 | impl method sigs in src/kind.rs |  |  | 0.556 |
| walker |  | 4721 | 73 | pub-item names surface in src/kind.rs |  |  | 0.556 |
| ns | 4770 |  | 538 | kind.rs tagged-dispatch types | 3.4 |  | 0.524 |
| walker |  | 4803 | 82 | listing of 'tests/ui' |  |  | 0.525 |
| walker |  | 4892 | 89 | pub-item names surface in src/nightly.rs |  |  | 0.525 |
| walker |  | 4892 | 0 | pub item at src/nightly.rs:41 |  |  | 0.525 |
| walker |  | 4892 | 0 | pub item at src/nightly.rs:52 |  |  | 0.525 |
| walker |  | 4892 | 0 | pub item at src/nightly.rs:56 |  |  | 0.525 |
| walker |  | 4902 | 10 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.525 |
| walker |  | 4913 | 11 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.525 |
| walker |  | 4925 | 12 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.525 |
| walker |  | 4933 | 8 | listing of '.github' |  |  | 0.525 |
| walker |  | 4937 | 4 | listing of '.github/workflows' |  |  | 0.525 |
| ns | 4950 |  | 180 | wrapper.rs: MessageError / DisplayError / BoxedError | 3.5 |  | 0.517 |
| ns | 5232 |  | 282 | fmt.rs: ErrorImpl::display body + debug/Indented locations | 3.6 |  | 0.502 |
| ns | 5548 |  | 316 | ptr.rs internal pointer newtypes | 3.7 |  | 0.525 |
| ns | 6006 |  | 458 | backtrace.rs cfg landscape (locations) | 3.8 |  | 0.503 |
| ns | 6235 |  | 229 | nightly.rs locations + signatures | 3.9 |  | 0.500 |
| walker |  | 6501 | 1564 | crate-doc body in src/lib.rs |  |  | 0.500 |
| ns | 6577 |  | 342 | ensure.rs orientation header (locations) | 3.10 |  | 0.487 |
| walker |  | 6760 | 259 | README.md section #9 |  |  | 0.487 |
| ns | 6817 |  | 240 | tests/common + drop helpers | 4.1 |  | 0.476 |
| walker |  | 6880 | 120 | README.md section #6 |  |  | 0.476 |
| walker |  | 7018 | 138 | README.md section #3 |  |  | 0.476 |
| walker |  | 7042 | 24 | pub item at src/ensure.rs:10 |  |  | 0.477 |
| walker |  | 7066 | 24 | pub item at src/ensure.rs:25 |  |  | 0.478 |
| ns | 7104 |  | 287 | test_context: Low/Mid/High chain helper + fns | 4.2 |  | 0.464 |
| walker |  | 7217 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.464 |
| ns | 7307 |  | 203 | test_downcast / test_repr / test_convert fns (locations) | 4.3 |  | 0.456 |
| walker |  | 7416 | 199 | README.md section #1 |  |  | 0.456 |
| ns | 7450 |  | 143 | test_macros + test_chain + test_source fns | 4.4 |  | 0.450 |
| walker |  | 7540 | 124 | pub-item doc body at src/lib.rs:468 |  |  | 0.450 |
| ns | 7686 |  | 236 | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | 4.5 |  | 0.443 |
| walker |  | 7745 | 205 | README.md section #5 |  |  | 0.443 |
| ns | 7952 |  | 266 | test_autotrait + test_boxed + test_ffi + test_backtrace | 4.6 |  | 0.434 |
| walker |  | 7960 | 215 | README.md section #4 |  |  | 0.434 |
| ns | 8056 |  | 104 | tests/ui + tests/crate listings | 4.7 |  | 0.440 |
| walker |  | 8201 | 241 | README.md section #2 |  |  | 0.440 |
| walker |  | 8239 | 38 | pub item at src/kind.rs:80 |  |  | 0.443 |
| walker |  | 8279 | 40 | pub item at src/kind.rs:58 |  |  | 0.447 |
| ns | 8313 |  | 257 | ErrorImpl<E> layout + vtable() reader | 5.1 | 3.1 | 0.444 |
| walker |  | 8319 | 40 | pub item at src/kind.rs:104 |  |  | 0.448 |
| walker |  | 8388 | 69 | pub-item names surface in tests/test_ffi.rs |  |  | 0.449 |
| walker |  | 8388 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.449 |
| walker |  | 8388 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.449 |
| walker |  | 8388 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.449 |
| walker |  | 8397 | 9 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.449 |
| walker |  | 8407 | 10 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.449 |
| walker |  | 8420 | 13 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.449 |
| walker |  | 8434 | 14 | listing of 'tests/crate' |  |  | 0.455 |
| walker |  | 8501 | 67 | [package] in tests/crate/Cargo.toml |  |  | 0.455 |
| ns | 8514 |  | 201 | Chain DoubleEndedIterator buffering | 5.2 | 3.3 | 0.448 |
| walker |  | 8541 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.448 |
| walker |  | 8545 | 4 | listing of 'tests/common' |  |  | 0.450 |
| walker |  | 8590 | 45 | pub-item names surface in tests/common/mod.rs |  |  | 0.451 |
| walker |  | 8590 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.451 |
| walker |  | 8590 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.451 |
| walker |  | 8590 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.451 |
| walker |  | 8598 | 8 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.451 |
| walker |  | 8612 | 14 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.452 |
| walker |  | 8634 | 22 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.453 |
| walker |  | 8655 | 21 | mod/use plumbing in tests/common/mod.rs |  |  | 0.455 |
| walker |  | 8659 | 4 | listing of 'tests/drop' |  |  | 0.457 |
| walker |  | 8680 | 21 | pub-item names surface in tests/drop/mod.rs |  |  | 0.459 |
| walker |  | 8702 | 22 | pub item at tests/drop/mod.rs:26 |  |  | 0.461 |
| walker |  | 8725 | 23 | pub item at tests/drop/mod.rs:9 |  |  | 0.465 |
| ns | 8742 |  | 228 | kind.rs autoref-precedence note | 5.3 | 3.4 | 0.460 |
| walker |  | 8779 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.460 |
| walker |  | 8903 | 124 | impl method sigs in tests/drop/mod.rs |  |  | 0.466 |
| walker |  | 8929 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.466 |
| walker |  | 8944 | 15 | plaintext config .gitignore |  |  | 0.466 |
| ns | 9288 |  | 546 | ErrorImpl::debug body | 5.4 | 3.6 | 0.450 |
| ns | 9528 |  | 240 | __fancy_ensure! macro body | 5.5 | 3.10 | 0.443 |
| ns | 9798 |  | 270 | ensure!: render `{msg} ({lhs} vs {rhs})` | 5.6 | 3.10 | 0.437 |
| ns | 9946 |  | 148 | build.rs cfg pipeline (rustc version + cfgs) | 5.7 |  | 0.434 |
