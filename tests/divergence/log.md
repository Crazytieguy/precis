scores: Sim=0.536 Reached=14/44 Early=2 Late=9 Partial=2 Missing=28 Used=9967/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 0 | 1 | 0.83 |
| 2 | 7 | 6 | 0 | 1 | 0.85 |
| 3 | 9 | 1 | 0 | 8 | 0.19 |
| 4 | 8 | 1 | 2 | 5 | 0.34 |
| 5 | 14 | 1 | 0 | 13 | 0.08 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 102 | 310 | +208 | 1.00 | late | Cargo name + description | [package] in Cargo.toml (t=310, 9 atoms) |
| 1.3 | 190 | 578 | +388 | 1.00 | late | Crate-doc one-liner | crate-doc lede in src/lib.rs (t=578, 6 atoms) |
| 1.4 | 231 | 2881 | +2650 | 1.00 | late | src/ + src/kv/ listing |  |
| 1.5 | 381 | 578 | +197 | 1.00 | late | Crate-doc target/level/body model | crate-doc lede in src/lib.rs (t=578, 9 atoms) |
| 1.6 | 545 | — | — | 0.00 | missing | Five-macro user-facing summary |  |
| 2.1 | 735 | 961 | +226 | 1.00 | late | Public-item map of lib.rs | pub-item names surface in src/lib.rs (t=961, 31 atoms) |
| 2.2 | 831 | 3842 | +3011 | 1.00 | late | Macro names (src/macros.rs) | macro_export bodies across src (t=9967, 151 atoms) |
| 2.3 | 883 | 2044 | +1161 | 1.00 | late | Log trait method signatures | pub item at src/lib.rs:1249 (t=2044, 17 atoms) |
| 2.4 | 1153 | 1557 | +404 | 1.00 | late | Level enum body | pub item at src/lib.rs:475 (t=1557, 24 atoms) |
| 2.6 | 1531 | — | — | 0.01 | missing | Logger installation entry-point signatures | pub-item doc lede at src/lib.rs:1396 (t=5970, 13 atoms) |
| 2.7 | 2033 | 9967 | +7934 | 0.93 | late | log! macro shapes (4 forms) | macro_export bodies across src (t=9967, 37 atoms) |
| 3.1 | 2481 | — | — | 0.24 | missing | Record struct + accessor signatures | pub item at src/lib.rs:842 (t=1113, 9 atoms) |
| 3.2 | 2587 | — | — | 0.40 | missing | Metadata struct + accessors | pub item at src/lib.rs:1158 (t=1003, 4 atoms) |
| 3.3 | 2682 | — | — | 0.00 | missing | Level public-method index |  |
| 3.4 | 2779 | — | — | 0.00 | missing | LevelFilter public-method index |  |
| 3.5 | 3082 | — | — | 0.00 | missing | Global state + ordering constants |  |
| 3.6 | 3382 | 2337 | -1045 | 1.00 | early | STATIC_MAX_LEVEL compile-time match | pub item at src/lib.rs:1611 (t=2337, 14 atoms) |
| 3.7 | 3856 | — | — | 0.00 | missing | __log internal-macro body (the actual gate) | macro_export names across src (t=3842, 1 atoms) |
| 3.8 | 4415 | — | — | 0.06 | missing | __private_api log dispatcher | pub-item names surface in src/__private_api.rs (t=7104, 6 atoms) |
| 3.9 | 4715 | — | — | 0.00 | missing | set_logger_inner state transitions |  |
| 4.1 | 5085 | — | — | 0.33 | missing | kv module concept | crate-doc lede in src/kv/mod.rs (t=2980, 10 atoms) |
| 4.2 | 5290 | — | — | 0.55 | partial | kv module re-exports | mod/use plumbing in src/kv/mod.rs (t=5323, 11 atoms) |
| 4.3 | 5477 | — | — | 0.00 | missing | kv capture-modifier table |  |
| 4.5 | 5889 | — | — | 0.18 | missing | kv::Value capture constructors | pub item at src/kv/value.rs:119 (t=3231, 3 atoms) |
| 4.6 | 6053 | — | — | 0.58 | partial | kv::Key surface | pub-item names surface in src/kv/key.rs (t=3004, 4 atoms) |
| 4.7 | 6361 | — | — | 0.07 | missing | VisitValue trait method index | pub-item names surface in src/kv/value.rs (t=3186, 2 atoms) |
| 4.8 | 6552 | — | — | 0.00 | missing | kv::Value to_* primitive accessors |  |
| 5.1 | 6964 | 4753 | -2211 | 0.85 | early | Cargo features list | [features] in Cargo.toml (t=4753, 28 atoms) |
| 5.2 | 7149 | — | — | 0.00 | missing | Implementing-a-Logger doc snippet |  |
| 5.3 | 7366 | — | — | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note |  |
| 5.4 | 7693 | — | — | 0.00 | missing | Compile-time max_level_* conflict guards |  |
| 5.5 | 7986 | — | — | 0.00 | missing | FromStr impls for Level/LevelFilter | pub item at src/lib.rs:636 (t=1252, 14 atoms) |
| 5.6 | 8318 | — | — | 0.00 | missing | RecordBuilder method index |  |
| 5.7 | 8407 | — | — | 0.00 | missing | MetadataBuilder method index |  |
| 5.8 | 8659 | — | — | 0.00 | missing | Logger blanket impls (&T, Box, Arc) |  |
| 5.9 | 8977 | — | — | 0.07 | missing | non-atomic AtomicUsize fallback | mod/use plumbing in src/lib.rs (t=5005, 2 atoms) |
| 5.10 | 8998 | — | — | 0.00 | missing | tests/ + benches/ + harness listings |  |
| 5.11 | 9234 | — | — | 0.00 | missing | Macro test-fn names (tests/macros.rs) |  |
| 5.12 | 9640 | — | — | 0.00 | missing | kv::Source impl matrix | pub item at src/kv/source.rs:235 (t=3104, 4 atoms) |
| 5.13 | 9784 | — | — | 0.27 | missing | kv::Error variants | pub item at src/kv/error.rs:5 (t=3116, 3 atoms) |
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
| 2368 | 0.83 | 2853 | 9967 | macro_export bodies across src |
| 397 | 1.00 | 397 | 7013 | [dependencies] in Cargo.toml |
| 359 | 0.84 | 425 | 6512 | pub item at src/kv/source.rs:51 |
| 349 | 0.87 | 401 | 2044 | pub item at src/lib.rs:1249 |
| 319 | 1.00 | 319 | 5757 | README.md section #4 |
| 213 | 1.00 | 213 | 3511 | macro_export bodies across src/kv |
| 213 | 1.00 | 213 | 5970 | pub-item doc lede at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 2598 | README.md section #0 |
| 156 | 0.61 | 256 | 3842 | macro_export names across src |
| 128 | 0.70 | 184 | 310 | [package] in Cargo.toml |
| 126 | 1.00 | 126 | 5152 | pub-item doc lede at src/lib.rs:1611 |
| 125 | 1.00 | 125 | 4878 | pub-item doc lede at src/lib.rs:1478 |
| 122 | 1.00 | 122 | 4057 | pub-item doc lede at src/lib.rs:636 |
| 109 | 1.00 | 109 | 4366 | pub-item doc lede at src/lib.rs:1420 |
| 104 | 1.00 | 104 | 6616 | pub-item doc body at src/lib.rs:1200 |
| 100 | 1.00 | 100 | 2771 | pub-item doc lede at src/lib.rs:475 |
| 97 | 1.00 | 97 | 4257 | pub-item doc lede at src/lib.rs:1375 |
| 96 | 1.00 | 96 | 6087 | CHANGELOG.md section #4 |
| 95 | 0.75 | 127 | 5005 | mod/use plumbing in src/lib.rs |
| 94 | 1.00 | 94 | 4151 | pub-item doc lede at src/lib.rs:1529 |
| 93 | 1.00 | 93 | 3935 | pub-item doc lede at src/lib.rs:1200 |
| 86 | 1.00 | 86 | 2857 | pub-item doc lede at src/lib.rs:1003 |
| 78 | 1.00 | 78 | 5401 | CHANGELOG.md section #3 |
| 75 | 1.00 | 75 | 653 | README.md section #1 |
| 73 | 1.00 | 73 | 2671 | pub-item doc lede at src/lib.rs:1351 |
| 67 | 1.00 | 67 | 3298 | macro_export names across src/kv |
| 63 | 1.00 | 63 | 2400 | pub-item doc lede at src/lib.rs:1566 |
| 55 | 1.00 | 55 | 5207 | pub-item doc body at src/lib.rs:1420 |
| 53 | 1.00 | 53 | 126 | headings outline in README.md |
