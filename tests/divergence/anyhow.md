scores: Sim=0.380 Reached=14/44 Early=0 Late=8 Partial=5 Missing=25 Used=9925/10000

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
| 1.2 | 79 | 1081 | +1002 | 1.00 | late | Crate-doc lede | crate-doc lede in src/lib.rs (t=1081, 3 atoms) |
| 1.3 | 128 | 454 | +326 | 1.00 | late | src/ listing |  |
| 1.4 | 195 | 331 | +136 | 1.00 | late | Crate identity (name, edition, MSRV) | [package] in Cargo.toml (t=331, 12 atoms) |
| 1.5 | 285 | 9314 | +9029 | 1.00 | late | tests/ listing |  |
| 2.1 | 424 | — | — | 0.58 | partial | lib.rs module declarations | mod/use plumbing in src/lib.rs (t=2609, 11 atoms) |
| 2.2 | 458 | — | — | 0.75 | partial | Error struct definition | pub item at src/lib.rs:390 (t=557, 3 atoms) |
| 2.4 | 541 | — | — | 0.60 | partial | Chain struct | pub item at src/lib.rs:415 (t=576, 3 atoms) |
| 2.5 | 697 | 9058 | +8361 | 1.00 | late | Error vs Box<dyn Error>: three differences | pub-item doc at src/lib.rs:390 (t=9058, 10 atoms) |
| 2.6 | 858 | — | — | 0.00 | missing | Error::* method signatures (locations) |  |
| 2.7 | 1036 | 729 | -307 | 0.92 | aligned | Context trait signature | pub item at src/lib.rs:616 (t=729, 12 atoms) |
| 2.8 | 1130 | — | — | 0.33 | missing | Crate-level public items (locations) | pub-item doc at src/lib.rs:616 (t=6992, 109 atoms) |
| 2.9 | 1302 | — | — | 0.13 | missing | Macro export locations | macro_export bodies across src (t=1437, 12 atoms) |
| 2.10 | 1542 | 1985 | +443 | 0.88 | aligned | Cargo features + dev-deps | [dependencies] in Cargo.toml (t=1985, 12 atoms) |
| 2.11 | 1757 | 9058 | +7301 | 1.00 | late | Display reprs: `{}` and `{:#}` | pub-item doc at src/lib.rs:390 (t=9058, 17 atoms) |
| 2.12 | 1979 | 1437 | -542 | 0.91 | aligned | anyhow! macro body | macro_export bodies across src (t=1437, 20 atoms) |
| 2.13 | 2558 | — | — | 0.00 | missing | Context impls for Result and Option |  |
| 2.14 | 3026 | 9058 | +6032 | 1.00 | late | Debug repr `{:?}` (default for `fn main`) | pub-item doc at src/lib.rs:390 (t=9058, 34 atoms) |
| 2.15 | 3399 | 9058 | +5659 | 0.91 | late | Debug repr `{:#?}` + manual chain rendering | pub-item doc at src/lib.rs:390 (t=9058, 32 atoms) |
| 3.1 | 3672 | — | — | 0.11 | missing | error.rs item locations | pub item at src/error.rs:934 (t=3690, 7 atoms) |
| 3.2 | 3934 | — | — | 0.00 | missing | context.rs item locations |  |
| 3.3 | 4232 | — | — | 0.48 | missing | chain.rs item locations | pub item at src/chain.rs:16 (t=3783, 9 atoms) |
| 3.4 | 4770 | — | — | 0.41 | missing | kind.rs tagged-dispatch types | pub-item names surface in src/kind.rs (t=3976, 12 atoms) |
| 3.5 | 4950 | — | — | 0.21 | missing | wrapper.rs: MessageError / DisplayError / BoxedError | pub-item names surface in src/wrapper.rs (t=3271, 6 atoms) |
| 3.6 | 5232 | — | — | 0.00 | missing | fmt.rs: ErrorImpl::display body + debug/Indented locations |  |
| 3.7 | 5548 | — | — | 0.68 | partial | ptr.rs internal pointer newtypes | pub-item names surface in src/ptr.rs (t=3071, 8 atoms) |
| 3.8 | 6006 | — | — | 0.03 | missing | backtrace.rs cfg landscape (locations) | pub-item names surface in src/backtrace.rs (t=1578, 2 atoms) |
| 3.9 | 6235 | — | — | 0.19 | missing | nightly.rs locations + signatures | pub-item names surface in src/nightly.rs (t=4203, 6 atoms) |
| 3.10 | 6577 | — | — | 0.24 | missing | ensure.rs orientation header (locations) | macro_export names across src (t=807, 5 atoms) |
| 4.1 | 6817 | — | — | 0.56 | partial | tests/common + drop helpers | impl method sigs in tests/drop/mod.rs (t=9742, 13 atoms) |
| 4.2 | 7104 | — | — | 0.00 | missing | test_context: Low/Mid/High chain helper + fns |  |
| 4.3 | 7307 | — | — | 0.00 | missing | test_downcast / test_repr / test_convert fns (locations) |  |
| 4.4 | 7450 | — | — | 0.00 | missing | test_macros + test_chain + test_source fns |  |
| 4.5 | 7686 | — | — | 0.00 | missing | test_fmt: f/g/h chain + EXPECTED_* constants (locations) |  |
| 4.6 | 7952 | — | — | 0.12 | missing | test_autotrait + test_boxed + test_ffi + test_backtrace | pub-item names surface in tests/test_ffi.rs (t=9383, 5 atoms) |
| 5.1 | 8313 | — | — | 0.39 | missing | ErrorImpl<E> layout + vtable() reader | pub item at src/error.rs:934 (t=3690, 7 atoms) |
| 5.2 | 8514 | — | — | 0.00 | missing | Chain DoubleEndedIterator buffering |  |
| 5.3 | 8742 | — | — | 0.00 | missing | kind.rs autoref-precedence note |  |
| 5.4 | 9288 | — | — | 0.00 | missing | ErrorImpl::debug body |  |
| 5.5 | 9528 | — | — | 0.04 | missing | __fancy_ensure! macro body | macro_export names across src (t=807, 2 atoms) |
| 5.6 | 9798 | — | — | 0.00 | missing | ensure!: render `{msg} ({lhs} vs {rhs})` |  |
| 5.7 | 9946 | — | — | 0.00 | missing | build.rs cfg pipeline (rustc version + cfgs) |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 2110 | pub-item doc at src/lib.rs:<n> |
| 10 | 1767 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1564 | 1.00 | 1564 | 5767 | crate-doc body in src/lib.rs |
| 1225 | 1.00 | 1225 | 6992 | pub-item doc at src/lib.rs:616 |
| 346 | 1.00 | 346 | 3617 | pub-item doc at src/lib.rs:468 |
| 286 | 1.00 | 286 | 3021 | pub-item doc at src/lib.rs:650 |
| 259 | 1.00 | 259 | 2244 | README.md section #9 |
| 241 | 1.00 | 241 | 7852 | README.md section #2 |
| 215 | 1.00 | 215 | 7611 | README.md section #4 |
| 210 | 0.77 | 274 | 1081 | crate-doc lede in src/lib.rs |
| 205 | 1.00 | 205 | 7396 | README.md section #5 |
| 200 | 1.00 | 200 | 1778 | README.md section #7 |
| 199 | 1.00 | 199 | 7191 | README.md section #1 |
| 192 | 1.00 | 192 | 2436 | pub-item doc at src/lib.rs:415 |
| 138 | 1.00 | 138 | 4114 | README.md section #3 |
| 124 | 1.00 | 124 | 1561 | README.md section #8 |
| 120 | 1.00 | 120 | 3903 | README.md section #6 |
| 114 | 0.32 | 356 | 1437 | macro_export bodies across src |
| 99 | 0.73 | 135 | 9742 | impl method sigs in tests/drop/mod.rs |
| 83 | 0.50 | 167 | 331 | [package] in Cargo.toml |
| 67 | 1.00 | 67 | 9513 | [package] in tests/crate/Cargo.toml |
| 66 | 1.00 | 66 | 397 | README.md section #0 |
| 63 | 1.00 | 63 | 97 | README headline in README.md |
| 61 | 0.05 | 1206 | 9058 | pub-item doc at src/lib.rs:390 |
| 60 | 0.35 | 173 | 2609 | mod/use plumbing in src/lib.rs |
| 54 | 1.00 | 54 | 9817 | mod/use plumbing in tests/drop/mod.rs |
