scores: Sim=0.500 Reached=13/44 Early=1 Late=9 Partial=2 Missing=29 Used=7285/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 0 | 1 | 0.83 |
| 2 | 7 | 5 | 0 | 2 | 0.72 |
| 3 | 9 | 1 | 0 | 8 | 0.21 |
| 4 | 8 | 1 | 2 | 5 | 0.34 |
| 5 | 14 | 1 | 0 | 13 | 0.10 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 102 | 355 | +253 | 1.00 | late | Cargo name + description | [package] in Cargo.toml (t=355, 9 atoms) |
| 1.3 | 190 | 674 | +484 | 1.00 | late | Crate-doc one-liner | crate-doc lede in src/lib.rs (t=674, 6 atoms) |
| 1.4 | 231 | 715 | +484 | 1.00 | late | src/ + src/kv/ listing |  |
| 1.5 | 381 | 674 | +293 | 1.00 | late | Crate-doc target/level/body model | crate-doc lede in src/lib.rs (t=674, 9 atoms) |
| 1.6 | 545 | — | — | 0.00 | missing | Five-macro user-facing summary |  |
| 2.1 | 735 | 1432 | +697 | 1.00 | late | Public-item map of lib.rs | pub-item names surface in src/lib.rs (t=1432, 31 atoms) |
| 2.2 | 831 | 3893 | +3062 | 1.00 | late | Macro names (src/macros.rs) | macro_export names across src (t=3893, 19 atoms) |
| 2.3 | 883 | 2515 | +1632 | 1.00 | late | Log trait method signatures | pub item at src/lib.rs:1249 (t=2515, 17 atoms) |
| 2.4 | 1153 | 2028 | +875 | 1.00 | late | Level enum body | pub item at src/lib.rs:475 (t=2028, 24 atoms) |
| 2.5 | 1294 | 1723 | +429 | 1.00 | late | LevelFilter enum body | pub item at src/lib.rs:636 (t=1723, 13 atoms) |
| 2.6 | 1531 | — | — | 0.01 | missing | Logger installation entry-point signatures | pub-item doc lede at src/lib.rs:1396 (t=5991, 13 atoms) |
| 2.7 | 2033 | — | — | 0.00 | missing | log! macro shapes (4 forms) | macro_export names across src (t=3893, 1 atoms) |
| 3.1 | 2481 | — | — | 0.24 | missing | Record struct + accessor signatures | pub item at src/lib.rs:842 (t=1584, 9 atoms) |
| 3.2 | 2587 | — | — | 0.40 | missing | Metadata struct + accessors | pub item at src/lib.rs:1158 (t=1490, 4 atoms) |
| 3.3 | 2682 | — | — | 0.00 | missing | Level public-method index |  |
| 3.4 | 2779 | — | — | 0.00 | missing | LevelFilter public-method index |  |
| 3.5 | 3082 | — | — | 0.00 | missing | Global state + ordering constants |  |
| 3.7 | 3856 | — | — | 0.00 | missing | __log internal-macro body (the actual gate) | macro_export names across src (t=3893, 1 atoms) |
| 3.8 | 4415 | — | — | 0.22 | missing | __private_api log dispatcher | pub item at src/__private_api.rs:84 (t=7285, 11 atoms) |
| 3.9 | 4715 | — | — | 0.00 | missing | set_logger_inner state transitions |  |
| 4.1 | 5085 | — | — | 0.33 | missing | kv module concept | crate-doc lede in src/kv/mod.rs (t=826, 10 atoms) |
| 4.2 | 5290 | — | — | 0.55 | partial | kv module re-exports | mod/use plumbing in src/kv/mod.rs (t=5344, 11 atoms) |
| 4.3 | 5477 | — | — | 0.00 | missing | kv capture-modifier table |  |
| 4.5 | 5889 | — | — | 0.18 | missing | kv::Value capture constructors | pub item at src/kv/value.rs:119 (t=1036, 3 atoms) |
| 4.6 | 6053 | — | — | 0.58 | partial | kv::Key surface | pub-item names surface in src/kv/key.rs (t=850, 4 atoms) |
| 4.7 | 6361 | — | — | 0.07 | missing | VisitValue trait method index | pub-item names surface in src/kv/value.rs (t=1020, 2 atoms) |
| 4.8 | 6552 | — | — | 0.00 | missing | kv::Value to_* primitive accessors |  |
| 5.1 | 6964 | 4795 | -2169 | 0.85 | early | Cargo features list | [features] in Cargo.toml (t=4795, 28 atoms) |
| 5.2 | 7149 | — | — | 0.00 | missing | Implementing-a-Logger doc snippet |  |
| 5.3 | 7366 | — | — | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note |  |
| 5.4 | 7693 | — | — | 0.00 | missing | Compile-time max_level_* conflict guards |  |
| 5.5 | 7986 | — | — | 0.00 | missing | FromStr impls for Level/LevelFilter | pub item at src/lib.rs:636 (t=1723, 14 atoms) |
| 5.6 | 8318 | — | — | 0.00 | missing | RecordBuilder method index |  |
| 5.7 | 8407 | — | — | 0.00 | missing | MetadataBuilder method index |  |
| 5.8 | 8659 | — | — | 0.00 | missing | Logger blanket impls (&T, Box, Arc) |  |
| 5.9 | 8977 | — | — | 0.07 | missing | non-atomic AtomicUsize fallback | mod/use plumbing in src/lib.rs (t=5047, 2 atoms) |
| 5.10 | 8998 | — | — | 0.20 | missing | tests/ + benches/ + harness listings |  |
| 5.11 | 9234 | — | — | 0.00 | missing | Macro test-fn names (tests/macros.rs) |  |
| 5.12 | 9640 | — | — | 0.00 | missing | kv::Source impl matrix | pub item at src/kv/source.rs:235 (t=982, 4 atoms) |
| 5.13 | 9784 | — | — | 0.27 | missing | kv::Error variants | pub item at src/kv/error.rs:5 (t=737, 3 atoms) |
| 5.14 | 9861 | — | — | 0.00 | missing | logger() global accessor |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 12 | 1301 | pub-item doc lede at src/lib.rs:<n> |
| 3 | 592 | README.md section #<n> |
| 2 | 174 | CHANGELOG.md section #<n> |
| 2 | 159 | pub-item doc body at src/lib.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 397 | 1.00 | 397 | 7048 | [dependencies] in Cargo.toml |
| 359 | 0.84 | 425 | 6512 | pub item at src/kv/source.rs:51 |
| 349 | 0.87 | 401 | 2515 | pub item at src/lib.rs:1249 |
| 319 | 1.00 | 319 | 5778 | README.md section #4 |
| 213 | 1.00 | 213 | 3303 | macro_export bodies across src/kv |
| 213 | 1.00 | 213 | 5991 | pub-item doc lede at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 3090 | README.md section #0 |
| 156 | 0.61 | 256 | 3893 | macro_export names across src |
| 128 | 0.70 | 184 | 355 | [package] in Cargo.toml |
| 126 | 1.00 | 126 | 5173 | pub-item doc lede at src/lib.rs:1611 |
| 125 | 1.00 | 125 | 4920 | pub-item doc lede at src/lib.rs:1478 |
| 122 | 1.00 | 122 | 4108 | pub-item doc lede at src/lib.rs:636 |
| 109 | 1.00 | 109 | 4408 | pub-item doc lede at src/lib.rs:1420 |
| 104 | 1.00 | 104 | 6616 | pub-item doc body at src/lib.rs:1200 |
| 100 | 1.00 | 100 | 3530 | pub-item doc lede at src/lib.rs:475 |
| 97 | 1.00 | 97 | 4299 | pub-item doc lede at src/lib.rs:1375 |
| 96 | 1.00 | 96 | 6087 | CHANGELOG.md section #4 |
| 95 | 0.75 | 127 | 5047 | mod/use plumbing in src/lib.rs |
| 94 | 1.00 | 94 | 4202 | pub-item doc lede at src/lib.rs:1529 |
| 93 | 1.00 | 93 | 3986 | pub-item doc lede at src/lib.rs:1200 |
| 86 | 1.00 | 86 | 3637 | pub-item doc lede at src/lib.rs:1003 |
| 78 | 1.00 | 78 | 5422 | CHANGELOG.md section #3 |
| 75 | 1.00 | 75 | 430 | README.md section #1 |
| 73 | 1.00 | 73 | 3390 | pub-item doc lede at src/lib.rs:1351 |
| 67 | 1.00 | 67 | 1132 | macro_export names across src/kv |
| 63 | 1.00 | 63 | 2871 | pub-item doc lede at src/lib.rs:1566 |
| 55 | 1.00 | 55 | 5228 | pub-item doc body at src/lib.rs:1420 |
| 53 | 1.00 | 53 | 171 | headings outline in README.md |
