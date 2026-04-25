scores: Sim=0.352 Reached=14/44 Early=1 Late=10 Partial=2 Missing=28 Used=9956/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 0 | 1 | 0.83 |
| 2 | 7 | 6 | 0 | 1 | 0.85 |
| 3 | 9 | 1 | 0 | 8 | 0.21 |
| 4 | 8 | 0 | 2 | 6 | 0.22 |
| 5 | 14 | 2 | 0 | 12 | 0.16 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.2 | 102 | 257 | +155 | 1.00 | late | Cargo name + description |
| 1.3 | 190 | 3734 | +3544 | 1.00 | late | Crate-doc one-liner |
| 1.4 | 231 | 6839 | +6608 | 1.00 | late | src/ + src/kv/ listing |
| 1.5 | 381 | 3734 | +3353 | 1.00 | late | Crate-doc target/level/body model |
| 1.6 | 545 | — | — | 0.00 | missing | Five-macro user-facing summary |
| 2.1 | 735 | 4186 | +3451 | 1.00 | late | Public-item map of lib.rs |
| 2.3 | 883 | 5866 | +4983 | 1.00 | late | Log trait method signatures |
| 2.4 | 1153 | 4967 | +3814 | 1.00 | late | Level enum body |
| 2.5 | 1294 | 4477 | +3183 | 1.00 | late | LevelFilter enum body |
| 2.6 | 1531 | — | — | 0.01 | missing | Logger installation entry-point signatures |
| 2.7 | 2033 | 3490 | +1457 | 0.93 | late | log! macro shapes (4 forms) |
| 3.1 | 2481 | — | — | 0.24 | missing | Record struct + accessor signatures |
| 3.2 | 2587 | — | — | 0.40 | missing | Metadata struct + accessors |
| 3.3 | 2682 | — | — | 0.00 | missing | Level public-method index |
| 3.4 | 2779 | — | — | 0.00 | missing | LevelFilter public-method index |
| 3.5 | 3082 | — | — | 0.00 | missing | Global state + ordering constants |
| 3.6 | 3382 | 5465 | +2083 | 1.00 | late | STATIC_MAX_LEVEL compile-time match |
| 3.7 | 3856 | — | — | 0.00 | missing | __log internal-macro body (the actual gate) |
| 3.8 | 4415 | — | — | 0.22 | missing | __private_api log dispatcher |
| 3.9 | 4715 | — | — | 0.00 | missing | set_logger_inner state transitions |
| 4.1 | 5085 | — | — | 0.33 | missing | kv module concept |
| 4.2 | 5290 | — | — | 0.00 | missing | kv module re-exports |
| 4.3 | 5477 | — | — | 0.00 | missing | kv capture-modifier table |
| 4.4 | 5612 | — | — | 0.62 | partial | kv::Source trait surface |
| 4.5 | 5889 | — | — | 0.18 | missing | kv::Value capture constructors |
| 4.6 | 6053 | — | — | 0.58 | partial | kv::Key surface |
| 4.7 | 6361 | — | — | 0.07 | missing | VisitValue trait method index |
| 4.8 | 6552 | — | — | 0.00 | missing | kv::Value to_* primitive accessors |
| 5.1 | 6964 | 8716 | +1752 | 0.85 | aligned | Cargo features list |
| 5.2 | 7149 | — | — | 0.00 | missing | Implementing-a-Logger doc snippet |
| 5.3 | 7366 | — | — | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note |
| 5.4 | 7693 | — | — | 0.00 | missing | Compile-time max_level_* conflict guards |
| 5.5 | 7986 | — | — | 0.00 | missing | FromStr impls for Level/LevelFilter |
| 5.6 | 8318 | — | — | 0.00 | missing | RecordBuilder method index |
| 5.7 | 8407 | — | — | 0.00 | missing | MetadataBuilder method index |
| 5.8 | 8659 | — | — | 0.00 | missing | Logger blanket impls (&T, Box, Arc) |
| 5.9 | 8977 | — | — | 0.07 | missing | non-atomic AtomicUsize fallback |
| 5.10 | 8998 | 3886 | -5112 | 1.00 | early | tests/ + benches/ + harness listings |
| 5.11 | 9234 | — | — | 0.00 | missing | Macro test-fn names (tests/macros.rs) |
| 5.12 | 9640 | — | — | 0.00 | missing | kv::Source impl matrix |
| 5.13 | 9784 | — | — | 0.27 | missing | kv::Error variants |
| 5.14 | 9861 | — | — | 0.00 | missing | logger() global accessor |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 2021 | 0.71 | 2853 | 3490 | macro_export bodies across src |
| 422 | 1.00 | 422 | 9956 | pub-item doc at src/lib.rs:1003 |
| 410 | 1.00 | 410 | 9534 | pub-item doc at src/lib.rs:1158 |
| 334 | 0.83 | 401 | 5866 | pub item at src/lib.rs:1249 |
| 327 | 1.00 | 327 | 8329 | README.md section #3 |
| 267 | 1.00 | 267 | 8983 | pub-item doc at src/lib.rs:1375 |
| 259 | 1.00 | 259 | 8002 | pub-item doc at src/lib.rs:1529 |
| 213 | 1.00 | 213 | 7119 | macro_export bodies across src/kv |
| 213 | 1.00 | 213 | 7682 | pub-item doc at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 6318 | README.md section #0 |
| 197 | 1.00 | 197 | 6805 | pub-item doc at src/lib.rs:1200 |
| 179 | 0.70 | 256 | 637 | macro_export names across src |
| 164 | 1.00 | 164 | 6608 | pub-item doc at src/lib.rs:1420 |
| 138 | 0.75 | 184 | 257 | [package] in Cargo.toml |
| 126 | 1.00 | 126 | 6444 | pub-item doc at src/lib.rs:1611 |
| 122 | 1.00 | 122 | 5189 | pub-item doc at src/lib.rs:636 |
| 100 | 1.00 | 100 | 5067 | pub-item doc at src/lib.rs:475 |
| 97 | 0.77 | 127 | 9110 | mod/use plumbing in src/lib.rs |
| 86 | 1.00 | 86 | 343 | README.md section #1 |
| 76 | 1.00 | 76 | 3878 | [package] in test_max_level_features/Cargo.toml |
| 73 | 1.00 | 73 | 4699 | pub-item doc at src/lib.rs:1351 |
| 70 | 0.24 | 300 | 4186 | pub-item names surface in src/lib.rs |
| 67 | 1.00 | 67 | 6906 | macro_export names across src/kv |
| 63 | 1.00 | 63 | 4586 | pub-item doc at src/lib.rs:1566 |
| 59 | 1.00 | 59 | 3802 | impl method sigs in test_max_level_features/main.rs |
| 53 | 1.00 | 53 | 6120 | mod/use plumbing in test_max_level_features/main.rs |
