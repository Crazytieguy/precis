Score(3000)=0.697 I=0.872 C=0.557 ns_rows≤3K=18/44 (reached=11 partial=1 missing=6)

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
| walker |  | 411 | 74 | README.md section #0 |  |  | 0.479 |
| ns | 438 |  | 141 | lib.rs module declarations | 2.1 |  | 0.392 |
| walker |  | 460 | 49 | listing of 'src' |  |  | 0.569 |
| ns | 474 |  | 36 | Error struct definition | 2.2 |  | 0.550 |
| ns | 499 |  | 25 | Result type alias | 2.3 |  | 0.545 |
| walker |  | 550 | 90 | pub-item names surface in src/lib.rs |  |  | 0.559 |
| walker |  | 550 | 0 | pub item at src/lib.rs:468 |  |  | 0.559 |
| ns | 561 |  | 62 | Chain struct | 2.4 |  | 0.539 |
| walker |  | 563 | 13 | pub item at src/lib.rs:650 |  |  | 0.539 |
| walker |  | 589 | 26 | pub item at src/lib.rs:390 |  |  | 0.576 |
| walker |  | 599 | 10 | pub item body at src/lib.rs:650 body 651 |  |  | 0.576 |
| walker |  | 628 | 29 | pub item at src/lib.rs:415 |  |  | 0.597 |
| ns | 719 |  | 158 | Error vs Box<dyn Error>: three differences | 2.5 | 2.2 | 0.555 |
| walker |  | 788 | 160 | pub item at src/lib.rs:616 |  |  | 0.564 |
| walker |  | 870 | 82 | macro_export names across src |  |  | 0.564 |
| ns | 910 |  | 191 | Error::* method signatures (locations) | 2.6 |  | 0.516 |
| walker |  | 911 | 41 | pub-item doc lede at src/lib.rs:415 |  |  | 0.516 |
| ns | 1090 |  | 180 | Context trait signature | 2.7 |  | 0.562 |
| walker |  | 1185 | 274 | crate-doc lede in src/lib.rs |  |  | 0.597 |
| ns | 1194 |  | 104 | Crate-level public items (locations) | 2.8 |  | 0.575 |
| walker |  | 1375 | 190 | mod/use plumbing in src/lib.rs |  |  | 0.626 |
| ns | 1376 |  | 182 | Macro export locations | 2.9 |  | 0.586 |
| walker |  | 1440 | 65 | pub-item doc lede at src/lib.rs:616 |  |  | 0.586 |
| walker |  | 1530 | 90 | listing of 'tests' |  |  | 0.701 |
| ns | 1616 |  | 240 | Cargo features + dev-deps | 2.10 | 1.4 | 0.656 |
| walker |  | 1671 | 141 | impl method sigs in src/context.rs |  |  | 0.656 |
| walker |  | 1809 | 138 | macro_export body at src/macros.rs:58 |  |  | 0.656 |
| ns | 1833 |  | 217 | Display reprs: `{}` and `{:#}` | 2.11 | 2.5 | 0.618 |
| walker |  | 1983 | 174 | pub-item doc lede at src/lib.rs:390 |  |  | 0.664 |
| ns | 2055 |  | 222 | anyhow! macro body | 2.12 | 2.9 | 0.619 |
| walker |  | 2195 | 212 | [dependencies] in Cargo.toml |  |  | 0.679 |
| walker |  | 2218 | 23 | pub-item names surface in src/ensure.rs |  |  | 0.679 |
| walker |  | 2440 | 222 | macro_export body at src/macros.rs:204 |  |  | 0.741 |
| ns | 2640 |  | 585 | Context impls for Result and Option | 2.13 | 2.7 | 0.651 |
| walker |  | 2768 | 328 | impl method sigs in src/error.rs |  |  | 0.697 |
| walker |  | 2810 | 42 | impl method sigs in src/chain.rs |  |  | 0.697 |
| walker |  | 2841 | 31 | pub-item names surface in src/chain.rs |  |  | 0.697 |
| walker |  | 2858 | 17 | pub item at src/chain.rs:11 |  |  | 0.697 |
| ns | 3110 |  | 470 | Debug repr `{:?}` (default for `fn main`) | 2.14 | 2.5 | 0.647 |
| walker |  | 3256 | 398 | impl method sigs in src/ptr.rs |  |  | 0.647 |
| walker |  | 3290 | 34 | pub-item names surface in src/error.rs |  |  | 0.647 |
| walker |  | 3323 | 33 | pub item at src/error.rs:952 |  |  | 0.647 |
| ns | 3483 |  | 373 | Debug repr `{:#?}` + manual chain rendering | 2.15 | 2.5 | 0.604 |
| walker |  | 3549 | 226 | pub-item doc lede at src/lib.rs:468 |  |  | 0.604 |
| ns | 3790 |  | 307 | error.rs item locations | 3.1 |  | 0.584 |
| walker |  | 3833 | 284 | pub-item doc lede at src/lib.rs:650 |  |  | 0.584 |
| walker |  | 3970 | 137 | README.md section #8 |  |  | 0.584 |
| walker |  | 4020 | 50 | pub-item names surface in src/ptr.rs |  |  | 0.585 |
| walker |  | 4033 | 13 | pub item at src/ptr.rs:181 |  |  | 0.585 |
| ns | 4072 |  | 282 | context.rs item locations | 3.2 |  | 0.564 |
| walker |  | 4080 | 47 | pub item at src/ptr.rs:6 |  |  | 0.565 |
| walker |  | 4141 | 61 | pub item at src/ptr.rs:64 |  |  | 0.566 |
| walker |  | 4203 | 62 | pub item at src/ptr.rs:125 |  |  | 0.567 |
| walker |  | 4257 | 54 | pub-item names surface in src/wrapper.rs |  |  | 0.567 |
| walker |  | 4266 | 9 | pub item at src/wrapper.rs:11 |  |  | 0.567 |
| walker |  | 4275 | 9 | pub item at src/wrapper.rs:34 |  |  | 0.567 |
| walker |  | 4284 | 9 | pub item at src/wrapper.rs:58 |  |  | 0.567 |
| ns | 4382 |  | 310 | chain.rs item locations | 3.3 |  | 0.548 |
| walker |  | 4383 | 99 | pub item at src/chain.rs:16 |  |  | 0.562 |
| walker |  | 4601 | 218 | README.md section #7 |  |  | 0.562 |
| walker |  | 4684 | 83 | pub item at src/error.rs:934 |  |  | 0.562 |
| walker |  | 4759 | 75 | pub-item names surface in src/kind.rs |  |  | 0.562 |
| walker |  | 4857 | 98 | impl method sigs in src/kind.rs |  |  | 0.563 |
| ns | 4932 |  | 550 | kind.rs tagged-dispatch types | 3.4 |  | 0.530 |
| walker |  | 4939 | 82 | listing of 'tests/ui' |  |  | 0.531 |
| walker |  | 4947 | 8 | listing of '.github' |  |  | 0.531 |
| walker |  | 4951 | 4 | listing of '.github/workflows' |  |  | 0.531 |
| walker |  | 5042 | 91 | pub-item names surface in src/nightly.rs |  |  | 0.531 |
| walker |  | 5042 | 0 | pub item at src/nightly.rs:41 |  |  | 0.531 |
| walker |  | 5042 | 0 | pub item at src/nightly.rs:52 |  |  | 0.531 |
| walker |  | 5042 | 0 | pub item at src/nightly.rs:56 |  |  | 0.531 |
| walker |  | 5054 | 12 | pub item body at src/nightly.rs:56 body 57 |  |  | 0.531 |
| walker |  | 5067 | 13 | pub item body at src/nightly.rs:41 body 42 |  |  | 0.531 |
| walker |  | 5081 | 14 | pub item body at src/nightly.rs:52 body 53 |  |  | 0.531 |
| ns | 5130 |  | 198 | wrapper.rs: MessageError / DisplayError / BoxedError | 3.5 |  | 0.523 |
| ns | 5418 |  | 288 | fmt.rs: ErrorImpl::display body + debug/Indented locations | 3.6 |  | 0.507 |
| ns | 5750 |  | 332 | ptr.rs internal pointer newtypes | 3.7 |  | 0.530 |
| ns | 6224 |  | 474 | backtrace.rs cfg landscape (locations) | 3.8 |  | 0.508 |
| ns | 6459 |  | 235 | nightly.rs locations + signatures | 3.9 |  | 0.505 |
| walker |  | 6671 | 1590 | crate-doc body in src/lib.rs |  |  | 0.505 |
| ns | 6819 |  | 360 | ensure.rs orientation header (locations) | 3.10 |  | 0.492 |
| walker |  | 6963 | 292 | README.md section #9 |  |  | 0.492 |
| ns | 7071 |  | 252 | tests/common + drop helpers | 4.1 |  | 0.480 |
| walker |  | 7100 | 137 | README.md section #6 |  |  | 0.480 |
| walker |  | 7245 | 145 | README.md section #3 |  |  | 0.480 |
| ns | 7380 |  | 309 | test_context: Low/Mid/High chain helper + fns | 4.2 |  | 0.467 |
| walker |  | 7396 | 151 | pub-item doc body at src/lib.rs:415 |  |  | 0.467 |
| walker |  | 7422 | 26 | pub item at src/ensure.rs:10 |  |  | 0.468 |
| walker |  | 7448 | 26 | pub item at src/ensure.rs:25 |  |  | 0.469 |
| walker |  | 7574 | 126 | pub-item doc body at src/lib.rs:468 |  |  | 0.469 |
| ns | 7615 |  | 235 | test_downcast / test_repr / test_convert fns (locations) | 4.3 |  | 0.461 |
| walker |  | 7789 | 215 | README.md section #5 |  |  | 0.461 |
| ns | 7796 |  | 181 | test_macros + test_chain + test_source fns | 4.4 |  | 0.455 |
| walker |  | 8008 | 219 | README.md section #1 |  |  | 0.455 |
| ns | 8058 |  | 262 | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | 4.5 |  | 0.447 |
| walker |  | 8231 | 223 | README.md section #4 |  |  | 0.447 |
| ns | 8360 |  | 302 | test_autotrait + test_boxed + test_ffi + test_backtrace | 4.6 |  | 0.439 |
| ns | 8464 |  | 104 | tests/ui + tests/crate listings | 4.7 |  | 0.444 |
| walker |  | 8495 | 264 | README.md section #2 |  |  | 0.444 |
| walker |  | 8535 | 40 | pub item at src/kind.rs:80 |  |  | 0.447 |
| walker |  | 8577 | 42 | pub item at src/kind.rs:58 |  |  | 0.451 |
| walker |  | 8619 | 42 | pub item at src/kind.rs:104 |  |  | 0.456 |
| walker |  | 8690 | 71 | pub-item names surface in tests/test_ffi.rs |  |  | 0.456 |
| walker |  | 8690 | 0 | pub item at tests/test_ffi.rs:7 |  |  | 0.456 |
| walker |  | 8690 | 0 | pub item at tests/test_ffi.rs:12 |  |  | 0.456 |
| walker |  | 8690 | 0 | pub item at tests/test_ffi.rs:17 |  |  | 0.456 |
| walker |  | 8701 | 11 | pub item body at tests/test_ffi.rs:7 body 8 |  |  | 0.456 |
| walker |  | 8713 | 12 | pub item body at tests/test_ffi.rs:17 body 18 |  |  | 0.456 |
| ns | 8721 |  | 257 | ErrorImpl<E> layout + vtable() reader | 5.1 | 3.1 | 0.453 |
| walker |  | 8728 | 15 | pub item body at tests/test_ffi.rs:12 body 13 |  |  | 0.453 |
| walker |  | 8742 | 14 | listing of 'tests/crate' |  |  | 0.458 |
| walker |  | 8811 | 69 | [package] in tests/crate/Cargo.toml |  |  | 0.458 |
| walker |  | 8851 | 40 | [features] in tests/crate/Cargo.toml |  |  | 0.458 |
| walker |  | 8855 | 4 | listing of 'tests/common' |  |  | 0.461 |
| walker |  | 8902 | 47 | pub-item names surface in tests/common/mod.rs |  |  | 0.461 |
| walker |  | 8902 | 0 | pub item at tests/common/mod.rs:4 |  |  | 0.461 |
| walker |  | 8902 | 0 | pub item at tests/common/mod.rs:8 |  |  | 0.461 |
| walker |  | 8902 | 0 | pub item at tests/common/mod.rs:12 |  |  | 0.461 |
| walker |  | 8912 | 10 | pub item body at tests/common/mod.rs:4 body 5 |  |  | 0.462 |
| ns | 8920 |  | 199 | Chain DoubleEndedIterator buffering | 5.2 | 3.3 | 0.455 |
| walker |  | 8928 | 16 | pub item body at tests/common/mod.rs:8 body 9 |  |  | 0.456 |
| walker |  | 8952 | 24 | pub item body at tests/common/mod.rs:12 body 13 |  |  | 0.457 |
| walker |  | 8971 | 19 | mod/use plumbing in tests/common/mod.rs |  |  | 0.458 |
| walker |  | 8975 | 4 | listing of 'tests/drop' |  |  | 0.461 |
| walker |  | 8998 | 23 | pub-item names surface in tests/drop/mod.rs |  |  | 0.463 |
| walker |  | 9022 | 24 | pub item at tests/drop/mod.rs:26 |  |  | 0.465 |
| walker |  | 9047 | 25 | pub item at tests/drop/mod.rs:9 |  |  | 0.469 |
| walker |  | 9101 | 54 | mod/use plumbing in tests/drop/mod.rs |  |  | 0.469 |
| ns | 9150 |  | 230 | kind.rs autoref-precedence note | 5.3 | 3.4 | 0.464 |
| walker |  | 9221 | 120 | impl method sigs in tests/drop/mod.rs |  |  | 0.469 |
| walker |  | 9247 | 26 | [dependencies] in tests/crate/Cargo.toml |  |  | 0.469 |
| walker |  | 9262 | 15 | plaintext config .gitignore |  |  | 0.469 |
| ns | 9694 |  | 544 | ErrorImpl::debug body | 5.4 | 3.6 | 0.454 |
| ns | 9932 |  | 238 | __fancy_ensure! macro body | 5.5 | 3.10 | 0.446 |
| ns | 10200 |  | 268 | ensure!: render `{msg} ({lhs} vs {rhs})` | 5.6 | 3.10 | 0.440 |
| ns | 10362 |  | 162 | build.rs cfg pipeline (rustc version + cfgs) | 5.7 |  | 0.437 |
