scores: Sim=0.391 Reached=16/49 Early=2 Late=10 Partial=0 Missing=33 Used=9956/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 6 | 0 | 0 | 0.99 |
| 2 | 6 | 6 | 0 | 0 | 0.98 |
| 3 | 10 | 2 | 0 | 8 | 0.30 |
| 4 | 7 | 0 | 0 | 7 | 0.17 |
| 5 | 14 | 1 | 0 | 13 | 0.10 |
| 6 | 6 | 1 | 0 | 5 | 0.23 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.1 | 10 | 3734 | +3724 | 1.00 | late | Crate-doc lede (first sentence) |
| 1.3 | 97 | 6839 | +6742 | 1.00 | late | src/ and src/kv/ listings |
| 1.4 | 226 | 3734 | +3508 | 1.00 | late | Crate-doc — facade abstraction + noop fallback |
| 1.5 | 307 | 3734 | +3427 | 1.00 | late | Crate-doc — log-request shape (target, level, body) |
| 1.6 | 496 | 257 | -239 | 0.94 | early | Cargo package identity + MSRV |
| 2.1 | 560 | 637 | +77 | 1.00 | aligned+over | Top-level log-macro names (location batch) |
| 2.2 | 828 | 4186 | +3358 | 1.00 | late | Public items in src/lib.rs (location batch) |
| 2.3 | 899 | 5866 | +4967 | 1.00 | late | `Log` trait signature (three methods) |
| 2.4 | 1123 | 8716 | +7593 | 0.86 | late | Cargo features — filter + core toggles |
| 2.5 | 1393 | 4967 | +3574 | 1.00 | late | `Level` enum variants |
| 2.6 | 1534 | 4477 | +2943 | 1.00 | late | `LevelFilter` enum variants |
| 3.1 | 2036 | 3490 | +1454 | 0.93 | late | `log!` macro — four call-form patterns |
| 3.2 | 2528 | — | — | 0.03 | missing | `__log!` expansion — level gate + `__private_api::log` call |
| 3.3 | 3163 | — | — | 0.20 | missing | `__private_api` log shim (log_impl + GlobalLogger) |
| 3.4 | 3345 | — | — | 0.00 | missing | Global state machine (LOGGER, STATE, state constants) |
| 3.5 | 3631 | — | — | 0.00 | missing | `set_logger_inner` compare-exchange install |
| 3.6 | 3887 | — | — | 0.21 | missing | `logger()` — the Acquire-ordered read |
| 3.7 | 4106 | — | — | 0.16 | missing | Runtime max-level (atomic + set/get) |
| 3.9 | 4663 | — | — | 0.36 | missing | `Record` struct fields |
| 3.10 | 4920 | — | — | 0.16 | missing | `Metadata` struct + accessors |
| 4.1 | 5414 | — | — | 0.29 | missing | `kv` module — concept + capture-modifier table |
| 4.2 | 5619 | — | — | 0.00 | missing | `kv` re-exports + submodules |
| 4.3 | 6061 | — | — | 0.31 | missing | Public items across `src/kv/*.rs` (location batch) |
| 4.4 | 6498 | — | — | 0.03 | missing | `kv::Source` trait signatures |
| 4.5 | 6695 | — | — | 0.46 | missing | `VisitSource` + `VisitValue` trait heads |
| 4.6 | 7005 | — | — | 0.00 | missing | `kv::Value` data-model doc comment |
| 4.7 | 7269 | — | — | 0.12 | missing | Record's `key_values` + `KeyValues` wrapper |
| 5.1 | 7604 | — | — | 0.00 | missing | `RecordBuilder` method heads (location batch) |
| 5.2 | 7635 | — | — | 0.00 | missing | Feature-conflict diagnostics (compile_error! locations) |
| 5.3 | 7741 | — | — | 0.00 | missing | no-atomic-ptr `AtomicUsize` fallback |
| 5.4 | 7836 | — | — | 0.21 | missing | `set_boxed_logger` + `set_logger` public wrappers |
| 5.5 | 7985 | — | — | 0.00 | missing | `set_logger_racy` body (no-atomic install) |
| 5.6 | 8344 | — | — | 0.23 | missing | Error types (`SetLoggerError`, `ParseLevelError`) |
| 5.7 | 8394 | — | — | 0.00 | missing | `Log` blanket impls (signatures only) |
| 5.8 | 8744 | — | — | 0.00 | missing | `__log_value!` capture-modifier dispatch |
| 5.9 | 9015 | — | — | 0.00 | missing | `capture_*` helpers in `__private_api::kv_support` (signatures) |
| 5.10 | 9220 | — | — | 0.00 | missing | `serde.rs` — Level Serialize impl |
| 5.11 | 9315 | — | — | 0.00 | missing | `kv::Error::Inner` variants |
| 5.12 | 9578 | — | — | 0.00 | missing | `kv::Source` default `get` via visitor |
| 5.13 | 9682 | — | — | 0.00 | missing | `kv::Source` impl-for locations (tuple, slice, Option, HashMap, BTreeMap) |
| 6.1 | 9760 | — | — | 0.40 | missing | README H2 headings (location batch) |
| 6.2 | 9772 | 3886 | -5886 | 1.00 | early | tests/ and benches/ listings + top-level fn signatures |
| 6.3 | 9805 | — | — | 0.00 | missing | `test_max_level_features` sub-binary (features line) |
| 6.4 | 9856 | — | — | 0.00 | missing | CI matrix jobs (location batch) |
| 6.5 | 9929 | — | — | 0.00 | missing | CHANGELOG — recent-release H2 locations |
| 6.6 | 9985 | — | — | 0.00 | missing | RFC 0296 — top-level section headings |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 2021 | 0.71 | 2853 | 3490 | macro_export bodies across src |
| 422 | 1.00 | 422 | 9956 | pub-item doc at src/lib.rs:1003 |
| 410 | 1.00 | 410 | 9534 | pub-item doc at src/lib.rs:1158 |
| 320 | 0.80 | 401 | 5866 | pub item at src/lib.rs:1249 |
| 313 | 0.96 | 327 | 8329 | README.md section #3 |
| 267 | 1.00 | 267 | 8983 | pub-item doc at src/lib.rs:1375 |
| 259 | 1.00 | 259 | 8002 | pub-item doc at src/lib.rs:1529 |
| 213 | 1.00 | 213 | 7119 | macro_export bodies across src/kv |
| 213 | 1.00 | 213 | 7682 | pub-item doc at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 6318 | README.md section #0 |
| 197 | 1.00 | 197 | 6805 | pub-item doc at src/lib.rs:1200 |
| 192 | 0.75 | 256 | 637 | macro_export names across src |
| 164 | 1.00 | 164 | 6608 | pub-item doc at src/lib.rs:1420 |
| 138 | 0.36 | 387 | 8716 | [features] in Cargo.toml |
| 126 | 1.00 | 126 | 6444 | pub-item doc at src/lib.rs:1611 |
| 122 | 1.00 | 122 | 5189 | pub-item doc at src/lib.rs:636 |
| 117 | 0.92 | 127 | 9110 | mod/use plumbing in src/lib.rs |
| 100 | 1.00 | 100 | 5067 | pub-item doc at src/lib.rs:475 |
| 76 | 1.00 | 76 | 3878 | [package] in test_max_level_features/Cargo.toml |
| 73 | 1.00 | 73 | 4699 | pub-item doc at src/lib.rs:1351 |
| 67 | 1.00 | 67 | 6906 | macro_export names across src/kv |
| 61 | 0.25 | 244 | 3734 | crate-doc lede in src/lib.rs |
| 59 | 1.00 | 59 | 3802 | impl method sigs in test_max_level_features/main.rs |
| 57 | 0.67 | 86 | 343 | README.md section #1 |
| 54 | 0.60 | 91 | 5974 | pub-item names surface in src/__private_api.rs |
| 53 | 1.00 | 53 | 6120 | mod/use plumbing in test_max_level_features/main.rs |
