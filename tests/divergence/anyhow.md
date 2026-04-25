scores: Sim=0.381 Reached=14/44 Early=2 Late=9 Partial=5 Missing=25 Used=9274/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 5 | 0 | 0 | 1.00 |
| 2 | 15 | 8 | 3 | 4 | 0.67 |
| 3 | 10 | 0 | 1 | 9 | 0.23 |
| 4 | 7 | 1 | 1 | 5 | 0.24 |
| 5 | 7 | 0 | 0 | 7 | 0.06 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 79 | 1867 | +1788 | 1.00 | late | Crate-doc lede | crate-doc lede in src/lib.rs (t=1867, 3 atoms) |
| 1.3 | 128 | 281 | +153 | 1.00 | late | src/ listing |  |
| 1.5 | 285 | 3170 | +2885 | 1.00 | late | tests/ listing |  |
| 2.1 | 424 | — | — | 0.58 | partial | lib.rs module declarations | mod/use plumbing in src/lib.rs (t=4690, 11 atoms) |
| 2.2 | 458 | — | — | 0.75 | partial | Error struct definition | pub item at src/lib.rs:390 (t=902, 3 atoms) |
| 2.3 | 481 | 887 | +406 | 1.00 | late | Result type alias | pub-item names surface in src/lib.rs (t=887, 1 atoms) |
| 2.4 | 541 | — | — | 0.60 | partial | Chain struct | pub item at src/lib.rs:415 (t=921, 3 atoms) |
| 2.5 | 697 | 9274 | +8577 | 1.00 | late | Error vs Box<dyn Error>: three differences | pub-item doc at src/lib.rs:390 (t=9274, 10 atoms) |
| 2.6 | 858 | — | — | 0.00 | missing | Error::* method signatures (locations) |  |
| 2.7 | 1036 | 1074 | +38 | 0.92 | aligned | Context trait signature | pub item at src/lib.rs:616 (t=1074, 12 atoms) |
| 2.8 | 1130 | — | — | 0.33 | missing | Crate-level public items (locations) | pub-item doc at src/lib.rs:616 (t=8034, 146 atoms) |
| 2.9 | 1302 | — | — | 0.13 | missing | Macro export locations | macro_export bodies across src (t=715, 12 atoms) |
| 2.10 | 1542 | 4517 | +2975 | 0.88 | late | Cargo features + dev-deps | [dependencies] in Cargo.toml (t=4517, 12 atoms) |
| 2.11 | 1757 | 9274 | +7517 | 1.00 | late | Display reprs: `{}` and `{:#}` | pub-item doc at src/lib.rs:390 (t=9274, 17 atoms) |
| 2.12 | 1979 | 715 | -1264 | 0.91 | early | anyhow! macro body | macro_export bodies across src (t=715, 20 atoms) |
| 2.13 | 2558 | — | — | 0.00 | missing | Context impls for Result and Option |  |
| 2.14 | 3026 | 9274 | +6248 | 1.00 | late | Debug repr `{:?}` (default for `fn main`) | pub-item doc at src/lib.rs:390 (t=9274, 34 atoms) |
| 2.15 | 3399 | 9274 | +5875 | 1.00 | late | Debug repr `{:#?}` + manual chain rendering | pub-item doc at src/lib.rs:390 (t=9274, 35 atoms) |
| 3.1 | 3672 | — | — | 0.11 | missing | error.rs item locations | pub item at src/error.rs:934 (t=1250, 7 atoms) |
| 3.2 | 3934 | — | — | 0.00 | missing | context.rs item locations |  |
| 3.3 | 4232 | — | — | 0.48 | missing | chain.rs item locations | pub item at src/chain.rs:16 (t=1343, 9 atoms) |
| 3.4 | 4770 | — | — | 0.41 | missing | kind.rs tagged-dispatch types | pub-item names surface in src/kind.rs (t=1940, 12 atoms) |
| 3.5 | 4950 | — | — | 0.21 | missing | wrapper.rs: MessageError / DisplayError / BoxedError | pub-item names surface in src/wrapper.rs (t=1593, 6 atoms) |
| 3.6 | 5232 | — | — | 0.00 | missing | fmt.rs: ErrorImpl::display body + debug/Indented locations |  |
| 3.7 | 5548 | — | — | 0.68 | partial | ptr.rs internal pointer newtypes | pub-item names surface in src/ptr.rs (t=1393, 8 atoms) |
| 3.8 | 6006 | — | — | 0.03 | missing | backtrace.rs cfg landscape (locations) | pub-item names surface in src/backtrace.rs (t=728, 2 atoms) |
| 3.9 | 6235 | — | — | 0.19 | missing | nightly.rs locations + signatures | pub-item names surface in src/nightly.rs (t=2674, 6 atoms) |
| 3.10 | 6577 | — | — | 0.24 | missing | ensure.rs orientation header (locations) | macro_export names across src (t=359, 5 atoms) |
| 4.1 | 6817 | — | — | 0.56 | partial | tests/common + drop helpers | impl method sigs in tests/drop/mod.rs (t=3598, 13 atoms) |
| 4.2 | 7104 | — | — | 0.00 | missing | test_context: Low/Mid/High chain helper + fns |  |
| 4.3 | 7307 | — | — | 0.00 | missing | test_downcast / test_repr / test_convert fns (locations) |  |
| 4.4 | 7450 | — | — | 0.00 | missing | test_macros + test_chain + test_source fns |  |
| 4.5 | 7686 | — | — | 0.00 | missing | test_fmt: f/g/h chain + EXPECTED_* constants (locations) |  |
| 4.6 | 7952 | — | — | 0.12 | missing | test_autotrait + test_boxed + test_ffi + test_backtrace | pub-item names surface in tests/test_ffi.rs (t=3239, 5 atoms) |
| 4.7 | 8056 | 4310 | -3746 | 1.00 | early | tests/ui + tests/crate listings |  |
| 5.1 | 8313 | — | — | 0.39 | missing | ErrorImpl<E> layout + vtable() reader | pub item at src/error.rs:934 (t=1250, 7 atoms) |
| 5.2 | 8514 | — | — | 0.00 | missing | Chain DoubleEndedIterator buffering |  |
| 5.3 | 8742 | — | — | 0.00 | missing | kind.rs autoref-precedence note |  |
| 5.4 | 9288 | — | — | 0.00 | missing | ErrorImpl::debug body |  |
| 5.5 | 9528 | — | — | 0.04 | missing | __fancy_ensure! macro body | macro_export names across src (t=359, 2 atoms) |
| 5.6 | 9798 | — | — | 0.00 | missing | ensure!: render `{msg} ({lhs} vs {rhs})` |  |
| 5.7 | 9946 | — | — | 0.00 | missing | build.rs cfg pipeline (rustc version + cfgs) |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1647 | 1.00 | 1647 | 8034 | pub-item doc at src/lib.rs:616 |
| 1132 | 1.00 | 1132 | 6387 | README.md section #1 |
| 539 | 1.00 | 539 | 5255 | pub-item doc at src/lib.rs:468 |
| 291 | 1.00 | 291 | 2349 | README headline in README.md |
| 286 | 1.00 | 286 | 4228 | pub-item doc at src/lib.rs:650 |
| 269 | 1.00 | 269 | 3942 | README.md section #4 |
| 210 | 1.00 | 210 | 2884 | README.md section #2 |
| 210 | 0.77 | 274 | 1867 | crate-doc lede in src/lib.rs |
| 192 | 1.00 | 192 | 3076 | pub-item doc at src/lib.rs:415 |
| 133 | 1.00 | 133 | 2585 | README.md section #3 |
| 114 | 0.32 | 356 | 715 | macro_export bodies across src |
| 103 | 1.00 | 103 | 2452 | README.md section #0 |
| 99 | 0.73 | 135 | 3598 | impl method sigs in tests/drop/mod.rs |
| 83 | 0.50 | 167 | 224 | [package] in Cargo.toml |
| 67 | 1.00 | 67 | 3369 | [package] in tests/crate/Cargo.toml |
| 61 | 0.05 | 1240 | 9274 | pub-item doc at src/lib.rs:390 |
| 60 | 0.35 | 173 | 4690 | mod/use plumbing in src/lib.rs |
| 54 | 1.00 | 54 | 3673 | mod/use plumbing in tests/drop/mod.rs |
