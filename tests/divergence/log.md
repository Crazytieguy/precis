scores: Sim=0.525 Reached=13/44 Early=1 Late=8 Partial=2 Missing=29 Used=9017/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 0 | 1 | 0.83 |
| 2 | 7 | 5 | 0 | 2 | 0.72 |
| 3 | 9 | 1 | 0 | 8 | 0.18 |
| 4 | 8 | 1 | 2 | 5 | 0.34 |
| 5 | 14 | 1 | 0 | 13 | 0.08 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 102 | 257 | +155 | 1.00 | late | Cargo name + description | [package] in Cargo.toml (t=257, 9 atoms) |
| 1.3 | 190 | 611 | +421 | 1.00 | late | Crate-doc one-liner | crate-doc lede in src/lib.rs (t=611, 6 atoms) |
| 1.4 | 231 | 3325 | +3094 | 1.00 | late | src/ + src/kv/ listing |  |
| 1.5 | 381 | 611 | +230 | 1.00 | late | Crate-doc target/level/body model | crate-doc lede in src/lib.rs (t=611, 9 atoms) |
| 1.6 | 545 | — | — | 0.00 | missing | Five-macro user-facing summary |  |
| 2.1 | 735 | 919 | +184 | 1.00 | aligned+over | Public-item map of lib.rs | pub-item doc at src/lib.rs:1478 (t=8620, 53 atoms) |
| 2.2 | 831 | 4211 | +3380 | 1.00 | late | Macro names (src/macros.rs) | macro_export names across src (t=4211, 19 atoms) |
| 2.3 | 883 | 2599 | +1716 | 1.00 | late | Log trait method signatures | pub item at src/lib.rs:1249 (t=2599, 17 atoms) |
| 2.4 | 1153 | 1700 | +547 | 1.00 | late | Level enum body | pub item at src/lib.rs:475 (t=1700, 24 atoms) |
| 2.6 | 1531 | — | — | 0.01 | missing | Logger installation entry-point signatures | pub-item doc at src/lib.rs:1478 (t=8620, 53 atoms) |
| 2.7 | 2033 | — | — | 0.00 | missing | log! macro shapes (4 forms) | macro_export names across src (t=4211, 1 atoms) |
| 3.1 | 2481 | — | — | 0.24 | missing | Record struct + accessor signatures | pub item at src/lib.rs:842 (t=1071, 9 atoms) |
| 3.2 | 2587 | — | — | 0.40 | missing | Metadata struct + accessors | pub item at src/lib.rs:1158 (t=961, 4 atoms) |
| 3.3 | 2682 | — | — | 0.00 | missing | Level public-method index |  |
| 3.4 | 2779 | — | — | 0.00 | missing | LevelFilter public-method index |  |
| 3.5 | 3082 | — | — | 0.00 | missing | Global state + ordering constants |  |
| 3.6 | 3382 | 2198 | -1184 | 1.00 | early | STATIC_MAX_LEVEL compile-time match | pub item at src/lib.rs:1611 (t=2198, 14 atoms) |
| 3.7 | 3856 | — | — | 0.00 | missing | __log internal-macro body (the actual gate) | macro_export names across src (t=4211, 1 atoms) |
| 3.8 | 4415 | — | — | 0.00 | missing | __private_api log dispatcher |  |
| 3.9 | 4715 | — | — | 0.00 | missing | set_logger_inner state transitions |  |
| 4.1 | 5085 | — | — | 0.33 | missing | kv module concept | crate-doc lede in src/kv/mod.rs (t=3424, 10 atoms) |
| 4.2 | 5290 | — | — | 0.55 | partial | kv module re-exports | mod/use plumbing in src/kv/mod.rs (t=6844, 11 atoms) |
| 4.3 | 5477 | — | — | 0.00 | missing | kv capture-modifier table |  |
| 4.4 | 5612 | 7868 | +2256 | 1.00 | late | kv::Source trait surface | pub item at src/kv/source.rs:51 (t=7868, 36 atoms) |
| 4.5 | 5889 | — | — | 0.18 | missing | kv::Value capture constructors | pub item at src/kv/value.rs:119 (t=3675, 3 atoms) |
| 4.6 | 6053 | — | — | 0.58 | partial | kv::Key surface | pub-item names surface in src/kv/key.rs (t=3448, 4 atoms) |
| 4.7 | 6361 | — | — | 0.07 | missing | VisitValue trait method index | pub-item names surface in src/kv/value.rs (t=3630, 2 atoms) |
| 4.8 | 6552 | — | — | 0.00 | missing | kv::Value to_* primitive accessors |  |
| 5.1 | 6964 | 5467 | -1497 | 0.85 | aligned | Cargo features list | [features] in Cargo.toml (t=5467, 28 atoms) |
| 5.2 | 7149 | — | — | 0.00 | missing | Implementing-a-Logger doc snippet |  |
| 5.3 | 7366 | — | — | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note |  |
| 5.4 | 7693 | — | — | 0.00 | missing | Compile-time max_level_* conflict guards |  |
| 5.5 | 7986 | — | — | 0.00 | missing | FromStr impls for Level/LevelFilter | pub item at src/lib.rs:636 (t=1210, 14 atoms) |
| 5.6 | 8318 | — | — | 0.00 | missing | RecordBuilder method index |  |
| 5.7 | 8407 | — | — | 0.00 | missing | MetadataBuilder method index |  |
| 5.8 | 8659 | — | — | 0.00 | missing | Logger blanket impls (&T, Box, Arc) |  |
| 5.9 | 8977 | — | — | 0.07 | missing | non-atomic AtomicUsize fallback | mod/use plumbing in src/lib.rs (t=5861, 2 atoms) |
| 5.10 | 8998 | — | — | 0.00 | missing | tests/ + benches/ + harness listings |  |
| 5.11 | 9234 | — | — | 0.00 | missing | Macro test-fn names (tests/macros.rs) |  |
| 5.12 | 9640 | — | — | 0.00 | missing | kv::Source impl matrix | pub item at src/kv/source.rs:235 (t=3592, 4 atoms) |
| 5.13 | 9784 | — | — | 0.27 | missing | kv::Error variants | pub item at src/kv/error.rs:5 (t=3489, 3 atoms) |
| 5.14 | 9861 | — | — | 0.00 | missing | logger() global accessor |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 14 | 3572 | pub-item doc at src/lib.rs:<n> |
| 3 | 611 | README.md section #<n> |
| 2 | 174 | CHANGELOG.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 635 | 1.00 | 635 | 8620 | pub-item doc at src/lib.rs:1478 |
| 521 | 1.00 | 521 | 7443 | pub-item doc at src/lib.rs:842 |
| 422 | 1.00 | 422 | 6728 | pub-item doc at src/lib.rs:1003 |
| 410 | 1.00 | 410 | 6306 | pub-item doc at src/lib.rs:1158 |
| 397 | 1.00 | 397 | 9017 | [dependencies] in Cargo.toml |
| 377 | 0.89 | 425 | 7868 | pub item at src/kv/source.rs:51 |
| 334 | 0.83 | 401 | 2599 | pub item at src/lib.rs:1249 |
| 327 | 1.00 | 327 | 5080 | README.md section #3 |
| 267 | 1.00 | 267 | 5734 | pub-item doc at src/lib.rs:1375 |
| 259 | 1.00 | 259 | 4753 | pub-item doc at src/lib.rs:1529 |
| 213 | 1.00 | 213 | 3955 | macro_export bodies across src/kv |
| 213 | 1.00 | 213 | 4424 | pub-item doc at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 2814 | README.md section #0 |
| 197 | 1.00 | 197 | 3301 | pub-item doc at src/lib.rs:1200 |
| 179 | 0.70 | 256 | 4211 | macro_export names across src |
| 164 | 1.00 | 164 | 3104 | pub-item doc at src/lib.rs:1420 |
| 138 | 0.75 | 184 | 257 | [package] in Cargo.toml |
| 126 | 1.00 | 126 | 2940 | pub-item doc at src/lib.rs:1611 |
| 122 | 1.00 | 122 | 1922 | pub-item doc at src/lib.rs:636 |
| 100 | 1.00 | 100 | 1800 | pub-item doc at src/lib.rs:475 |
| 97 | 0.77 | 127 | 5861 | mod/use plumbing in src/lib.rs |
| 96 | 1.00 | 96 | 7985 | CHANGELOG.md section #4 |
| 86 | 1.00 | 86 | 343 | README.md section #1 |
| 78 | 1.00 | 78 | 6922 | CHANGELOG.md section #3 |
| 73 | 1.00 | 73 | 1432 | pub-item doc at src/lib.rs:1351 |
| 70 | 0.24 | 300 | 919 | pub-item names surface in src/lib.rs |
| 67 | 1.00 | 67 | 3742 | macro_export names across src/kv |
| 63 | 1.00 | 63 | 1319 | pub-item doc at src/lib.rs:1566 |
