Score(3000)=0.689 I=0.868 C=0.547 ns_rows≤3K=18/44 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Top-level fixture listing | 1.1 |  | 1.000 |
| walker |  | 61 | 27 | [features] in Cargo.toml |  |  | 1.000 |
| ns | 83 |  | 49 | Crate-doc lede | 1.2 |  | 0.842 |
| walker |  | 128 | 67 | README headline in README.md |  |  | 0.842 |
| ns | 132 |  | 49 | src/ listing | 1.3 |  | 0.554 |
| walker |  | 172 | 44 | headings outline in README.md |  |  | 0.554 |
| ns | 207 |  | 75 | Crate identity (name, edition, MSRV) | 1.4 |  | 0.484 |
| ns | 297 |  | 90 | tests/ listing | 1.5 |  | 0.365 |
| walker |  | 337 | 165 | [package] in Cargo.toml |  |  | 0.479 |
| walker |  | 401 | 64 | README.md section #0 |  |  | 0.479 |
| ns | 438 |  | 141 | lib.rs module declarations | 2.1 |  | 0.392 |
| walker |  | 450 | 49 | listing of 'src' |  |  | 0.569 |
| ns | 474 |  | 36 | Error struct definition | 2.2 |  | 0.550 |
| ns | 499 |  | 25 | Result type alias | 2.3 |  | 0.545 |
| walker |  | 540 | 90 | pub-item names surface in src/lib.rs |  |  | 0.559 |
| walker |  | 540 | 0 | pub item at src/lib.rs:468 |  |  | 0.559 |
| walker |  | 553 | 13 | pub item at src/lib.rs:650 |  |  | 0.560 |
| ns | 561 |  | 62 | Chain struct | 2.4 |  | 0.539 |
| walker |  | 579 | 26 | pub item at src/lib.rs:390 |  |  | 0.576 |
| walker |  | 589 | 10 | pub item body at src/lib.rs:650 body 651 |  |  | 0.576 |
| walker |  | 618 | 29 | pub item at src/lib.rs:415 |  |  | 0.597 |
| ns | 719 |  | 158 | Error vs Box<dyn Error>: three differences | 2.5 | 2.2 | 0.555 |
| walker |  | 773 | 155 | pub item at src/lib.rs:616 |  |  | 0.563 |
| walker |  | 855 | 82 | macro_export names across src |  |  | 0.563 |
| walker |  | 896 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.563 |
| ns | 910 |  | 191 | Error::* method signatures (locations) | 2.6 |  | 0.516 |
| walker |  | 1081 | 185 | mod/use plumbing in src/lib.rs |  |  | 0.563 |
| ns | 1090 |  | 180 | Context trait signature | 2.7 |  | 0.596 |
| ns | 1194 |  | 104 | Crate-level public items (locations) | 2.8 |  | 0.582 |
| walker |  | 1355 | 274 | crate-doc lede in src/lib.rs |  |  | 0.616 |
| ns | 1376 |  | 182 | Macro export locations | 2.9 |  | 0.577 |
| walker |  | 1420 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.577 |
| walker |  | 1561 | 141 | impl method sigs in src/context.rs |  |  | 0.577 |
| ns | 1616 |  | 240 | Cargo features + dev-deps | 2.10 | 1.4 | 0.540 |
| walker |  | 1699 | 138 | macro_export body at src/macros.rs:58 |  |  | 0.540 |
| ns | 1833 |  | 217 | Display reprs: `{}` and `{:#}` | 2.11 | 2.5 | 0.508 |
| walker |  | 1873 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.557 |
| ns | 2055 |  | 222 | anyhow! macro body | 2.12 | 2.9 | 0.520 |
| walker |  | 2080 | 207 | [dependencies] in Cargo.toml |  |  | 0.579 |
| walker |  | 2103 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.579 |
| walker |  | 2325 | 222 | macro_export body at src/macros.rs:204 |  |  | 0.643 |
| walker |  | 2415 | 90 | listing of 'tests' |  |  | 0.731 |
| ns | 2640 |  | 585 | Context impls for Result and Option | 2.13 | 2.7 | 0.642 |
| walker |  | 2743 | 328 | impl method sigs in src/error.rs |  |  | 0.688 |
| walker |  | 2785 | 42 | impl method sigs in src/chain.rs |  |  | 0.688 |
| walker |  | 2816 | 31 | pub-item names surface in src/chain.rs |  |  | 0.689 |
| walker |  | 2833 | 17 | pub item at src/chain.rs:11 |  |  | 0.689 |
| ns | 3110 |  | 470 | Debug repr `{:?}` (default for `fn main`) | 2.14 | 2.5 | 0.639 |
| walker |  | 3231 | 398 | impl method sigs in src/ptr.rs |  |  | 0.639 |
| walker |  | 3265 | 34 | pub-item names surface in src/error.rs |  |  | 0.639 |
| walker |  | 3298 | 33 | pub item at src/error.rs:952 |  |  | 0.639 |
| ns | 3483 |  | 373 | Debug repr `{:#?}` + manual chain rendering | 2.15 | 2.5 | 0.597 |
| walker |  | 3524 | 226 | pub-item doc lede at src/lib.rs:468 |  |  | 0.597 |
| ns | 3790 |  | 307 | error.rs item locations | 3.1 |  | 0.577 |
| walker |  | 3808 | 284 | pub-item doc lede at src/lib.rs:650 |  |  | 0.577 |
| walker |  | 3930 | 122 | README.md section #8 |  |  | 0.577 |
| walker |  | 3980 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.578 |
| walker |  | 3993 | 13 | pub item at src/ptr.rs:181 |  |  | 0.578 |
| walker |  | 4040 | 47 | pub item at src/ptr.rs:6 |  |  | 0.578 |
| ns | 4072 |  | 282 | context.rs item locations | 3.2 |  | 0.558 |
| walker |  | 4101 | 61 | pub item at src/ptr.rs:64 |  |  | 0.559 |
| walker |  | 4163 | 62 | pub item at src/ptr.rs:125 |  |  | 0.560 |
| walker |  | 4217 | 54 | pub-item names surface in src/wrapper.rs |  |  | 0.560 |
| walker |  | 4226 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.560 |
| walker |  | 4235 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.560 |
| walker |  | 4244 | 9 | pub item at src/wrapper.rs:58 |  |  | 0.560 |
| ns | 4382 |  | 310 | chain.rs item locations | 3.3 |  | 0.542 |
| walker |  | 4442 | 198 | README.md section #7 |  |  | 0.542 |
| walker |  | 4541 | 99 | pub item at src/chain.rs:16 |  |  | 0.556 |
| walker |  | 4624 | 83 | pub item at src/error.rs:934 |  |  | 0.556 |
| walker |  | 4699 | 75 | pub-item names surface in src/kind.rs |  |  | 0.556 |
| walker |  | 4797 | 98 | impl method sigs in src/kind.rs |  |  | 0.556 |
| walker |  | 4879 | 82 | listing of 'tests/ui' |  |  | 0.557 |
| walker |  | 4887 | 8 | listing of '.github' |  |  | 0.557 |
| walker |  | 4891 | 4 | listing of '.github/workflows' |  |  | 0.557 |
| ns | 4932 |  | 550 | kind.rs tagged-dispatch types | 3.4 |  | 0.525 |
| walker |  | 4982 | 91 | pub-item names surface in src/nightly.rs |  |  | 0.525 |
| walker |  | 4982 | 0 | pub item at src/nightly.rs:41 |  |  | 0.525 |
| walker |  | 4982 | 0 | pub item at src/nightly.rs:52 |  |  | 0.525 |
| walker |  | 4982 | 0 | pub item at src/nightly.rs:56 |  |  | 0.525 |
| walker |  | 4994 | 12 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.525 |
| walker |  | 5007 | 13 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.525 |
| walker |  | 5021 | 14 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.525 |
| ns | 5130 |  | 198 | wrapper.rs: MessageError / DisplayError / BoxedError | 3.5 |  | 0.517 |
| ns | 5418 |  | 288 | fmt.rs: ErrorImpl::display body + debug/Indented locations | 3.6 |  | 0.502 |
| ns | 5750 |  | 332 | ptr.rs internal pointer newtypes | 3.7 |  | 0.525 |
| ns | 6224 |  | 474 | backtrace.rs cfg landscape (locations) | 3.8 |  | 0.503 |
| ns | 6459 |  | 235 | nightly.rs locations + signatures | 3.9 |  | 0.500 |
| walker |  | 6611 | 1590 | crate-doc body in src/lib.rs |  |  | 0.500 |
| ns | 6819 |  | 360 | ensure.rs orientation header (locations) | 3.10 |  | 0.487 |
| walker |  | 6868 | 257 | README.md section #9 |  |  | 0.487 |
| walker |  | 6990 | 122 | README.md section #6 |  |  | 0.487 |
| ns | 7071 |  | 252 | tests/common + drop helpers | 4.1 |  | 0.476 |
| walker |  | 7130 | 140 | README.md section #3 |  |  | 0.476 |
| walker |  | 7281 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.476 |
| walker |  | 7307 | 26 | pub item at src/ensure.rs:10 |  |  | 0.477 |
| walker |  | 7333 | 26 | pub item at src/ensure.rs:25 |  |  | 0.478 |
| ns | 7380 |  | 309 | test_context: Low/Mid/High chain helper + fns | 4.2 |  | 0.464 |
| walker |  | 7532 | 199 | README.md section #1 |  |  | 0.464 |
| ns | 7615 |  | 235 | test_downcast / test_repr / test_convert fns (locations) | 4.3 |  | 0.456 |
| walker |  | 7658 | 126 | pub-item doc body at src/lib.rs:468 |  |  | 0.456 |
| ns | 7796 |  | 181 | test_macros + test_chain + test_source fns | 4.4 |  | 0.450 |
| walker |  | 7863 | 205 | README.md section #5 |  |  | 0.450 |
| ns | 8058 |  | 262 | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | 4.5 |  | 0.443 |
| walker |  | 8076 | 213 | README.md section #4 |  |  | 0.443 |
| walker |  | 8315 | 239 | README.md section #2 |  |  | 0.443 |
| walker |  | 8355 | 40 | pub item at src/kind.rs:80 |  |  | 0.447 |
| ns | 8360 |  | 302 | test_autotrait + test_boxed + test_ffi + test_backtrace | 4.6 |  | 0.438 |
| walker |  | 8397 | 42 | pub item at src/kind.rs:58 |  |  | 0.442 |
| walker |  | 8439 | 42 | pub item at src/kind.rs:104 |  |  | 0.446 |
| ns | 8464 |  | 104 | tests/ui + tests/crate listings | 4.7 |  | 0.452 |
| walker |  | 8510 | 71 | pub-item names surface in tests/test_ffi.rs |  |  | 0.452 |
| walker |  | 8510 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.452 |
| walker |  | 8510 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.452 |
| walker |  | 8510 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.452 |
| walker |  | 8521 | 11 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.452 |
| walker |  | 8533 | 12 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.452 |
| walker |  | 8548 | 15 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.452 |
| walker |  | 8562 | 14 | listing of 'tests/crate' |  |  | 0.458 |
| walker |  | 8631 | 69 | [package] in tests/crate/Cargo.toml |  |  | 0.458 |
| walker |  | 8671 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.458 |
| walker |  | 8675 | 4 | listing of 'tests/common' |  |  | 0.460 |
| ns | 8721 |  | 257 | ErrorImpl<E> layout + vtable() reader | 5.1 | 3.1 | 0.457 |
| walker |  | 8722 | 47 | pub-item names surface in tests/common/mod.rs |  |  | 0.457 |
| walker |  | 8722 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.457 |
| walker |  | 8722 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.457 |
| walker |  | 8722 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.457 |
| walker |  | 8732 | 10 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.458 |
| walker |  | 8748 | 16 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.459 |
| walker |  | 8772 | 24 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.460 |
| walker |  | 8791 | 19 | mod/use plumbing in tests/common/mod.rs |  |  | 0.461 |
| walker |  | 8795 | 4 | listing of 'tests/drop' |  |  | 0.464 |
| walker |  | 8818 | 23 | pub-item names surface in tests/drop/mod.rs |  |  | 0.466 |
| walker |  | 8842 | 24 | pub item at tests/drop/mod.rs:26 |  |  | 0.468 |
| walker |  | 8867 | 25 | pub item at tests/drop/mod.rs:9 |  |  | 0.472 |
| ns | 8920 |  | 199 | Chain DoubleEndedIterator buffering | 5.2 | 3.3 | 0.465 |
| walker |  | 8921 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.465 |
| walker |  | 9041 | 120 | impl method sigs in tests/drop/mod.rs |  |  | 0.471 |
| walker |  | 9067 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.471 |
| walker |  | 9082 | 15 | plaintext config .gitignore |  |  | 0.471 |
| ns | 9150 |  | 230 | kind.rs autoref-precedence note | 5.3 | 3.4 | 0.466 |
| ns | 9694 |  | 544 | ErrorImpl::debug body | 5.4 | 3.6 | 0.450 |
| ns | 9932 |  | 238 | __fancy_ensure! macro body | 5.5 | 3.10 | 0.443 |
| ns | 10200 |  | 268 | ensure!: render `{msg} ({lhs} vs {rhs})` | 5.6 | 3.10 | 0.437 |
| ns | 10362 |  | 162 | build.rs cfg pipeline (rustc version + cfgs) | 5.7 |  | 0.434 |
