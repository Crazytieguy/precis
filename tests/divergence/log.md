scores: Sim=0.359 Reached=16/49 Early=2 Late=10 Partial=0 Missing=33 Over=3 Cap=10000

## Arrival ledger (non-aligned NS batches)

| id | exp_t | seen_t | credit | status | descriptor |
|----|------:|-------:|-------:|:-------|:-----------|
| 1.1 | 10 | 3734 | 1.00 | late | Crate-doc lede (first sentence) |
| 1.3 | 97 | 6839 | 1.00 | late | src/ and src/kv/ listings |
| 1.4 | 226 | 3734 | 1.00 | late | Crate-doc — facade abstraction + noop fallback |
| 1.5 | 307 | 3734 | 1.00 | late | Crate-doc — log-request shape (target, level, body) |
| 1.6 | 496 | 257 | 0.94 | early | Cargo package identity + MSRV |
| 2.1 | 560 | 637 | 1.00 | aligned+over | Top-level log-macro names (location batch) |
| 2.2 | 828 | 4186 | 1.00 | late+over | Public items in src/lib.rs (location batch) |
| 2.3 | 912 | 5866 | 1.00 | late | `Log` trait signature (three methods) |
| 2.4 | 1136 | 8716 | 0.86 | late | Cargo features — filter + core toggles |
| 2.5 | 1414 | 4967 | 1.00 | late | `Level` enum variants |
| 2.6 | 1564 | 4477 | 1.00 | late | `LevelFilter` enum variants |
| 3.1 | 2066 | 3490 | 0.93 | late | `log!` macro — four call-form patterns |
| 3.2 | 2558 | — | 0.03 | partial | `__log!` expansion — level gate + `__private_api::log` call |
| 3.3 | 3193 | — | 0.20 | partial | `__private_api` log shim (log_impl + GlobalLogger) |
| 3.4 | 3375 | — | 0.00 | missing | Global state machine (LOGGER, STATE, state constants) |
| 3.5 | 3661 | — | 0.00 | missing | `set_logger_inner` compare-exchange install |
| 3.6 | 3932 | — | 0.21 | partial | `logger()` — the Acquire-ordered read |
| 3.7 | 4181 | — | 0.16 | partial | Runtime max-level (atomic + set/get) |
| 3.9 | 4763 | — | 0.36 | partial | `Record` struct fields |
| 3.10 | 5032 | — | 0.16 | partial | `Metadata` struct + accessors |
| 4.1 | 5526 | — | 0.29 | partial | `kv` module — concept + capture-modifier table |
| 4.2 | 5731 | — | 0.00 | missing | `kv` re-exports + submodules |
| 4.3 | 6173 | — | 0.31 | partial+over | Public items across `src/kv/*.rs` (location batch) |
| 4.4 | 6618 | — | 0.03 | partial | `kv::Source` trait signatures |
| 4.5 | 6840 | — | 0.46 | partial | `VisitSource` + `VisitValue` trait heads |
| 4.6 | 7150 | — | 0.00 | missing | `kv::Value` data-model doc comment |
| 4.7 | 7444 | — | 0.12 | partial | Record's `key_values` + `KeyValues` wrapper |
| 5.1 | 7779 | — | 0.00 | missing | `RecordBuilder` method heads (location batch) |
| 5.2 | 7810 | — | 0.00 | missing | Feature-conflict diagnostics (compile_error! locations) |
| 5.3 | 7916 | — | 0.00 | missing | no-atomic-ptr `AtomicUsize` fallback |
| 5.4 | 8061 | — | 0.21 | partial | `set_boxed_logger` + `set_logger` public wrappers |
| 5.5 | 8210 | — | 0.00 | missing | `set_logger_racy` body (no-atomic install) |
| 5.6 | 8593 | — | 0.23 | partial | Error types (`SetLoggerError`, `ParseLevelError`) |
| 5.7 | 8643 | — | 0.00 | missing | `Log` blanket impls (signatures only) |
| 5.8 | 8993 | — | 0.00 | missing | `__log_value!` capture-modifier dispatch |
| 5.9 | 9264 | — | 0.00 | missing | `capture_*` helpers in `__private_api::kv_support` (signatures) |
| 5.10 | 9469 | — | 0.00 | missing | `serde.rs` — Level Serialize impl |
| 5.11 | 9564 | — | 0.00 | missing | `kv::Error::Inner` variants |
| 5.12 | 9827 | — | 0.00 | missing | `kv::Source` default `get` via visitor |
| 5.13 | 9931 | — | 0.00 | missing | `kv::Source` impl-for locations (tuple, slice, Option, HashMap, BTreeMap) |
| 6.1 | 10020 | — | 0.40 | partial | README H2 headings (location batch) |
| 6.2 | 10032 | 3886 | 1.00 | early | tests/ and benches/ listings + top-level fn signatures |
| 6.3 | 10065 | — | 0.00 | missing | `test_max_level_features` sub-binary (features line) |
| 6.4 | 10116 | — | 0.00 | missing | CI matrix jobs (location batch) |
| 6.5 | 10189 | — | 0.00 | missing | CHANGELOG — recent-release H2 locations |
| 6.6 | 10245 | — | 0.00 | missing | RFC 0296 — top-level section headings |

## Walker waste (cost ≥ 50, no NS intersection)

| first_t | cost | key |
|--------:|-----:|:----|
| 3802 | 59 | Rust(MethodSigs { file: "/Users/yoav/projects/precis/tests/fixtures/log/test_max_level_features/main.rs" }) |
| 3878 | 76 | Toml(Identity { file: "/Users/yoav/projects/precis/tests/fixtures/log/test_max_level_features/Cargo.toml" }) |
| 4699 | 73 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1351 }) |
| 5067 | 100 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 475 }) |
| 5189 | 122 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 636 }) |
| 6120 | 53 | Rust(ModUse { file: "/Users/yoav/projects/precis/tests/fixtures/log/test_max_level_features/main.rs" }) |
| 6318 | 198 | Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/log/README.md", section_index: 0 }) |
| 6444 | 126 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1611 }) |
| 6608 | 164 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1420 }) |
| 6805 | 197 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1200 }) |
| 6906 | 67 | Rust(MacroNames { src_dir: "/Users/yoav/projects/precis/tests/fixtures/log/src/kv" }) |
| 7119 | 213 | Rust(MacroBodies { src_dir: "/Users/yoav/projects/precis/tests/fixtures/log/src/kv" }) |
| 7682 | 213 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1396 }) |
| 8002 | 259 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1529 }) |
| 8983 | 267 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1375 }) |
| 9534 | 410 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1158 }) |
| 9956 | 422 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1003 }) |
