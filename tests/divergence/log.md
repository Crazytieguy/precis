scores: Sim=0.527 Reached=13/44 Early=2 Late=8 Partial=1 Missing=30 Used=7908/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 0 | 1 | 0.83 |
| 2 | 7 | 5 | 0 | 2 | 0.72 |
| 3 | 9 | 1 | 0 | 8 | 0.19 |
| 4 | 8 | 1 | 1 | 6 | 0.31 |
| 5 | 14 | 1 | 0 | 13 | 0.07 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 102 | 310 | +208 | 1.00 | late | Cargo name + description | [package] in Cargo.toml (t=310, 9 atoms) |
| 1.3 | 190 | 653 | +463 | 1.00 | late | Crate-doc one-liner | crate-doc lede in src/lib.rs (t=653, 6 atoms) |
| 1.4 | 231 | 2758 | +2527 | 1.00 | late | src/ + src/kv/ listing |  |
| 1.5 | 381 | 653 | +272 | 1.00 | late | Crate-doc target/level/body model | crate-doc lede in src/lib.rs (t=653, 9 atoms) |
| 1.6 | 545 | — | — | 0.00 | missing | Five-macro user-facing summary |  |
| 2.1 | 735 | 961 | +226 | 1.00 | late | Public-item map of lib.rs | pub-item doc at src/lib.rs:1003 (t=7817, 39 atoms) |
| 2.2 | 831 | 3537 | +2706 | 1.00 | late | Macro names (src/macros.rs) | macro_export names across src (t=3537, 19 atoms) |
| 2.3 | 883 | 1921 | +1038 | 1.00 | late | Log trait method signatures | pub item at src/lib.rs:1249 (t=1921, 17 atoms) |
| 2.4 | 1153 | 1520 | +367 | 1.00 | late | Level enum body | pub item at src/lib.rs:475 (t=1520, 24 atoms) |
| 2.6 | 1531 | — | — | 0.01 | missing | Logger installation entry-point signatures | pub-item doc at src/lib.rs:1375 (t=6535, 19 atoms) |
| 2.7 | 2033 | — | — | 0.00 | missing | log! macro shapes (4 forms) | macro_export names across src (t=3537, 1 atoms) |
| 3.1 | 2481 | — | — | 0.24 | missing | Record struct + accessor signatures | pub item at src/lib.rs:842 (t=1113, 9 atoms) |
| 3.2 | 2587 | — | — | 0.40 | missing | Metadata struct + accessors | pub item at src/lib.rs:1158 (t=1003, 4 atoms) |
| 3.3 | 2682 | — | — | 0.00 | missing | Level public-method index |  |
| 3.4 | 2779 | — | — | 0.00 | missing | LevelFilter public-method index |  |
| 3.5 | 3082 | — | — | 0.00 | missing | Global state + ordering constants |  |
| 3.6 | 3382 | 2214 | -1168 | 1.00 | early | STATIC_MAX_LEVEL compile-time match | pub item at src/lib.rs:1611 (t=2214, 14 atoms) |
| 3.7 | 3856 | — | — | 0.00 | missing | __log internal-macro body (the actual gate) | macro_export names across src (t=3537, 1 atoms) |
| 3.8 | 4415 | — | — | 0.06 | missing | __private_api log dispatcher | pub-item names surface in src/__private_api.rs (t=7908, 6 atoms) |
| 3.9 | 4715 | — | — | 0.00 | missing | set_logger_inner state transitions |  |
| 4.1 | 5085 | — | — | 0.33 | missing | kv module concept | crate-doc lede in src/kv/mod.rs (t=2847, 10 atoms) |
| 4.2 | 5290 | — | — | 0.55 | partial | kv module re-exports | mod/use plumbing in src/kv/mod.rs (t=4774, 11 atoms) |
| 4.3 | 5477 | — | — | 0.00 | missing | kv capture-modifier table |  |
| 4.5 | 5889 | — | — | 0.18 | missing | kv::Value capture constructors | pub item at src/kv/value.rs:119 (t=3001, 3 atoms) |
| 4.6 | 6053 | — | — | 0.34 | missing | kv::Key surface | pub-item names surface in src/kv/key.rs (t=6956, 4 atoms) |
| 4.7 | 6361 | — | — | 0.07 | missing | VisitValue trait method index | pub-item names surface in src/kv/value.rs (t=2956, 2 atoms) |
| 4.8 | 6552 | — | — | 0.00 | missing | kv::Value to_* primitive accessors |  |
| 5.1 | 6964 | 4374 | -2590 | 0.85 | early | Cargo features list | [features] in Cargo.toml (t=4374, 28 atoms) |
| 5.2 | 7149 | — | — | 0.00 | missing | Implementing-a-Logger doc snippet |  |
| 5.3 | 7366 | — | — | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note |  |
| 5.4 | 7693 | — | — | 0.00 | missing | Compile-time max_level_* conflict guards |  |
| 5.5 | 7986 | — | — | 0.00 | missing | FromStr impls for Level/LevelFilter | pub item at src/lib.rs:636 (t=1252, 14 atoms) |
| 5.6 | 8318 | — | — | 0.00 | missing | RecordBuilder method index |  |
| 5.7 | 8407 | — | — | 0.00 | missing | MetadataBuilder method index |  |
| 5.8 | 8659 | — | — | 0.00 | missing | Logger blanket impls (&T, Box, Arc) |  |
| 5.9 | 8977 | — | — | 0.07 | missing | non-atomic AtomicUsize fallback | mod/use plumbing in src/lib.rs (t=4501, 2 atoms) |
| 5.10 | 8998 | — | — | 0.00 | missing | tests/ + benches/ + harness listings |  |
| 5.11 | 9234 | — | — | 0.00 | missing | Macro test-fn names (tests/macros.rs) |  |
| 5.12 | 9640 | — | — | 0.00 | missing | kv::Source impl matrix | pub item at src/kv/source.rs:235 (t=2918, 4 atoms) |
| 5.13 | 9784 | — | — | 0.07 | missing | kv::Error variants | pub-item names surface in src/kv/error.rs (t=4532, 2 atoms) |
| 5.14 | 9861 | — | — | 0.00 | missing | logger() global accessor |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 12 | 2416 | pub-item doc at src/lib.rs:<n> |
| 3 | 592 | README.md section #<n> |
| 2 | 174 | CHANGELOG.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 422 | 1.00 | 422 | 7817 | pub-item doc at src/lib.rs:1003 |
| 410 | 1.00 | 410 | 7395 | pub-item doc at src/lib.rs:1158 |
| 397 | 1.00 | 397 | 6932 | [dependencies] in Cargo.toml |
| 377 | 0.89 | 425 | 6268 | pub item at src/kv/source.rs:51 |
| 334 | 0.83 | 401 | 1921 | pub item at src/lib.rs:1249 |
| 319 | 1.00 | 319 | 3987 | README.md section #4 |
| 267 | 1.00 | 267 | 6535 | pub-item doc at src/lib.rs:1375 |
| 259 | 1.00 | 259 | 5843 | pub-item doc at src/lib.rs:1529 |
| 213 | 1.00 | 213 | 3281 | macro_export bodies across src/kv |
| 213 | 1.00 | 213 | 5426 | pub-item doc at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 2561 | README.md section #0 |
| 197 | 1.00 | 197 | 5213 | pub-item doc at src/lib.rs:1200 |
| 179 | 0.70 | 256 | 3537 | macro_export names across src |
| 164 | 1.00 | 164 | 4938 | pub-item doc at src/lib.rs:1420 |
| 138 | 0.75 | 184 | 310 | [package] in Cargo.toml |
| 126 | 1.00 | 126 | 4658 | pub-item doc at src/lib.rs:1611 |
| 122 | 1.00 | 122 | 3659 | pub-item doc at src/lib.rs:636 |
| 100 | 1.00 | 100 | 2734 | pub-item doc at src/lib.rs:475 |
| 97 | 0.77 | 127 | 4501 | mod/use plumbing in src/lib.rs |
| 96 | 1.00 | 96 | 5584 | CHANGELOG.md section #4 |
| 78 | 1.00 | 78 | 5016 | CHANGELOG.md section #3 |
| 75 | 1.00 | 75 | 385 | README.md section #1 |
| 73 | 1.00 | 73 | 2634 | pub-item doc at src/lib.rs:1351 |
| 70 | 0.24 | 300 | 961 | pub-item names surface in src/lib.rs |
| 67 | 1.00 | 67 | 3068 | macro_export names across src/kv |
| 63 | 1.00 | 63 | 2323 | pub-item doc at src/lib.rs:1566 |
| 53 | 1.00 | 53 | 126 | headings outline in README.md |
