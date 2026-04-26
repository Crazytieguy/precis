scores: Sim=0.364 Reached=12/44 Early=0 Late=9 Partial=4 Missing=28 Used=8871/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 4 | 0 | 1 | 0.80 |
| 2 | 15 | 8 | 3 | 4 | 0.67 |
| 3 | 10 | 0 | 1 | 9 | 0.20 |
| 4 | 7 | 0 | 0 | 7 | 0.00 |
| 5 | 7 | 0 | 0 | 7 | 0.06 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 79 | 1205 | +1126 | 1.00 | late | Crate-doc lede | crate-doc lede in src/lib.rs (t=1205, 3 atoms) |
| 1.3 | 128 | 578 | +450 | 1.00 | late | src/ listing |  |
| 1.4 | 195 | 331 | +136 | 1.00 | late | Crate identity (name, edition, MSRV) | [package] in Cargo.toml (t=331, 12 atoms) |
| 1.5 | 285 | — | — | 0.00 | missing | tests/ listing |  |
| 2.1 | 424 | — | — | 0.58 | partial | lib.rs module declarations | mod/use plumbing in src/lib.rs (t=2609, 11 atoms) |
| 2.2 | 458 | — | — | 0.75 | partial | Error struct definition | pub item at src/lib.rs:390 (t=681, 3 atoms) |
| 2.3 | 481 | 666 | +185 | 1.00 | late | Result type alias | pub-item names surface in src/lib.rs (t=666, 1 atoms) |
| 2.4 | 541 | — | — | 0.60 | partial | Chain struct | pub item at src/lib.rs:415 (t=700, 3 atoms) |
| 2.5 | 697 | 8871 | +8174 | 1.00 | late | Error vs Box<dyn Error>: three differences | pub-item doc at src/lib.rs:390 (t=8871, 10 atoms) |
| 2.6 | 858 | — | — | 0.00 | missing | Error::* method signatures (locations) |  |
| 2.7 | 1036 | 853 | -183 | 0.92 | aligned | Context trait signature | pub item at src/lib.rs:616 (t=853, 12 atoms) |
| 2.8 | 1130 | — | — | 0.33 | missing | Crate-level public items (locations) | pub-item doc at src/lib.rs:390 (t=8871, 101 atoms) |
| 2.9 | 1302 | — | — | 0.13 | missing | Macro export locations | macro_export bodies across src (t=1561, 12 atoms) |
| 2.10 | 1542 | 2244 | +702 | 0.88 | late | Cargo features + dev-deps | [dependencies] in Cargo.toml (t=2244, 12 atoms) |
| 2.11 | 1757 | 8871 | +7114 | 1.00 | late | Display reprs: `{}` and `{:#}` | pub-item doc at src/lib.rs:390 (t=8871, 17 atoms) |
| 2.12 | 1979 | 1561 | -418 | 0.91 | aligned | anyhow! macro body | macro_export bodies across src (t=1561, 20 atoms) |
| 2.13 | 2558 | — | — | 0.00 | missing | Context impls for Result and Option |  |
| 2.14 | 3026 | 8871 | +5845 | 1.00 | late | Debug repr `{:?}` (default for `fn main`) | pub-item doc at src/lib.rs:390 (t=8871, 34 atoms) |
| 2.15 | 3399 | 8871 | +5472 | 1.00 | late | Debug repr `{:#?}` + manual chain rendering | pub-item doc at src/lib.rs:390 (t=8871, 35 atoms) |
| 3.1 | 3672 | — | — | 0.11 | missing | error.rs item locations | pub item at src/error.rs:934 (t=4469, 7 atoms) |
| 3.2 | 3934 | — | — | 0.00 | missing | context.rs item locations |  |
| 3.3 | 4232 | — | — | 0.48 | missing | chain.rs item locations | pub item at src/chain.rs:16 (t=4562, 9 atoms) |
| 3.4 | 4770 | — | — | 0.12 | missing | kind.rs tagged-dispatch types | pub-item names surface in src/kind.rs (t=4635, 12 atoms) |
| 3.5 | 4950 | — | — | 0.21 | missing | wrapper.rs: MessageError / DisplayError / BoxedError | pub-item names surface in src/wrapper.rs (t=4396, 6 atoms) |
| 3.6 | 5232 | — | — | 0.00 | missing | fmt.rs: ErrorImpl::display body + debug/Indented locations |  |
| 3.7 | 5548 | — | — | 0.68 | partial | ptr.rs internal pointer newtypes | pub-item names surface in src/ptr.rs (t=4196, 8 atoms) |
| 3.8 | 6006 | — | — | 0.03 | missing | backtrace.rs cfg landscape (locations) | pub-item names surface in src/backtrace.rs (t=2037, 2 atoms) |
| 3.9 | 6235 | — | — | 0.19 | missing | nightly.rs locations + signatures | pub-item names surface in src/nightly.rs (t=5263, 6 atoms) |
| 3.10 | 6577 | — | — | 0.17 | missing | ensure.rs orientation header (locations) | macro_export names across src (t=931, 5 atoms) |
| 4.1 | 6817 | — | — | 0.00 | missing | tests/common + drop helpers |  |
| 4.2 | 7104 | — | — | 0.00 | missing | test_context: Low/Mid/High chain helper + fns |  |
| 4.3 | 7307 | — | — | 0.00 | missing | test_downcast / test_repr / test_convert fns (locations) |  |
| 4.4 | 7450 | — | — | 0.00 | missing | test_macros + test_chain + test_source fns |  |
| 4.5 | 7686 | — | — | 0.00 | missing | test_fmt: f/g/h chain + EXPECTED_* constants (locations) |  |
| 4.6 | 7952 | — | — | 0.00 | missing | test_autotrait + test_boxed + test_ffi + test_backtrace |  |
| 4.7 | 8056 | — | — | 0.00 | missing | tests/ui + tests/crate listings |  |
| 5.1 | 8313 | — | — | 0.39 | missing | ErrorImpl<E> layout + vtable() reader | pub item at src/error.rs:934 (t=4469, 7 atoms) |
| 5.2 | 8514 | — | — | 0.00 | missing | Chain DoubleEndedIterator buffering |  |
| 5.3 | 8742 | — | — | 0.00 | missing | kind.rs autoref-precedence note |  |
| 5.4 | 9288 | — | — | 0.00 | missing | ErrorImpl::debug body |  |
| 5.5 | 9528 | — | — | 0.04 | missing | __fancy_ensure! macro body | macro_export names across src (t=931, 2 atoms) |
| 5.6 | 9798 | — | — | 0.00 | missing | ensure!: render `{msg} ({lhs} vs {rhs})` |  |
| 5.7 | 9946 | — | — | 0.00 | missing | build.rs cfg pipeline (rustc version + cfgs) |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 1774 | README.md section #<n> |
| 4 | 1078 | pub-item doc at src/lib.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 2368 | 1.00 | 2368 | 7631 | crate-doc body in src/lib.rs |
| 1125 | 1.00 | 1125 | 4146 | README.md section #1 |
| 539 | 1.00 | 539 | 5174 | pub-item doc at src/lib.rs:468 |
| 286 | 1.00 | 286 | 3021 | pub-item doc at src/lib.rs:650 |
| 259 | 1.00 | 259 | 2024 | README.md section #4 |
| 210 | 0.77 | 274 | 1205 | crate-doc lede in src/lib.rs |
| 200 | 1.00 | 200 | 1761 | README.md section #2 |
| 192 | 1.00 | 192 | 2436 | pub-item doc at src/lib.rs:415 |
| 124 | 1.00 | 124 | 529 | README.md section #3 |
| 114 | 0.32 | 356 | 1561 | macro_export bodies across src |
| 83 | 0.50 | 167 | 331 | [package] in Cargo.toml |
| 66 | 1.00 | 66 | 397 | README.md section #0 |
| 63 | 1.00 | 63 | 97 | README headline in README.md |
| 61 | 0.05 | 1240 | 8871 | pub-item doc at src/lib.rs:390 |
| 60 | 0.35 | 173 | 2609 | mod/use plumbing in src/lib.rs |
