scores: Sim=0.283 Reached=21/53 Early=3 Late=16 Partial=1 Missing=31 Over=6 Cap=10000

## Arrival ledger (non-aligned NS batches)

| id | exp_t | seen_t | credit | status | descriptor |
|----|------:|-------:|-------:|:-------|:-----------|
| 1.2 | 87 | 6839 | 1.00 | late | src/ and src/kv/ listings |
| 1.3 | 154 | 257 | 1.00 | late | Crate identity (name / version / description) |
| 1.4 | 218 | 637 | 1.00 | late | Public log-macro declaration lines |
| 1.5 | 351 | 3734 | 1.00 | late | Crate-doc lede — facade concept + noop-fallback |
| 1.6 | 438 | 3734 | 1.00 | late | Crate-doc — log-request data model (target/level/body) |
| 1.7 | 566 | — | 0.00 | missing | kv module re-exports |
| 1.8 | 686 | 4967 | 0.93 | late+over | Level enum — variants only (docs elided) |
| 1.9 | 787 | 4477 | 0.88 | late+over | LevelFilter enum — variants only (docs elided) |
| 1.10 | 923 | 5866 | 0.94 | late+over | Log trait — signature (docs elided, body skeleton) |
| 1.11 | 1055 | 8716 | 0.93 | late | Cargo features — max_level_* + release_max_level_* |
| 1.12 | 1216 | 8716 | 0.82 | late | Cargo features — std + kv family |
| 1.13 | 1534 | 6444 | 1.00 | late+over | STATIC_MAX_LEVEL constant — match body |
| 1.14 | 1941 | — | 0.00 | missing | Cargo dependencies |
| 2.1 | 2339 | 5866 | 0.93 | late | Log trait — full declaration with implementor docs |
| 2.2 | 2570 | — | 0.43 | partial | Crate-internal mod / use statements |
| 2.3 | 2894 | — | 0.00 | missing | Global-logger state machine — constants + statics |
| 2.4 | 3035 | 4186 | 1.00 | late+over | Global-logger fn signatures (location batch) |
| 2.5 | 3152 | 4338 | 0.90 | late | Record struct fields |
| 2.6 | 3215 | 4228 | 0.80 | late | Metadata struct (level + target) |
| 2.7 | 3329 | — | 0.00 | missing | Record public method names |
| 2.8 | 3450 | — | 0.00 | missing | Level / LevelFilter public method names |
| 2.9 | 3615 | — | 0.00 | missing | RecordBuilder + MetadataBuilder method names |
| 2.10 | 3990 | — | 0.36 | partial | Error types — SetLoggerError + ParseLevelError |
| 2.11 | 4100 | — | 0.67 | partial+over | Five level-macro declaration lines with arms elided |
| 2.12 | 4545 | — | 0.03 | partial | kv::Source trait declaration |
| 2.13 | 4767 | — | 0.46 | partial | kv::VisitSource + VisitValue trait signatures |
| 2.14 | 5156 | — | 0.32 | partial | kv::Key struct + ToKey trait |
| 2.15 | 5466 | — | 0.00 | missing | kv::Value data model (rustdoc only) |
| 2.16 | 5563 | 7743 | 0.89 | late | kv::Value + ToValue declaration lines |
| 2.17 | 5702 | — | 0.00 | missing | kv::Value constructor + extractor method names |
| 2.18 | 5924 | — | 0.17 | partial | kv::Error struct + msg ctor |
| 2.19 | 6105 | — | 0.00 | missing | kv capturing-modifier cheatsheet |
| 3.1 | 6399 | — | 0.00 | missing | Crate-doc — Implementing a Logger walkthrough (SimpleLogger) |
| 3.2 | 6755 | — | 0.03 | partial | set_logger body + set_logger_inner state machine |
| 3.3 | 7026 | — | 0.21 | partial | logger() body — Acquire load + NopLogger fallback |
| 3.4 | 7398 | — | 0.08 | partial | set_max_level + max_level bodies + NopLogger impl |
| 3.5 | 7682 | — | 0.00 | missing | __private_api — log_impl body |
| 3.6 | 8212 | 3490 | 0.88 | early | log! macro body (user-facing arms) |
| 3.7 | 8712 | — | 0.03 | partial | __log! expansion — STATIC_MAX_LEVEL + max_level() double-gate |
| 4.1 | 8747 | 3886 | 1.00 | early | tests/ + .github/workflows/ + supporting listings |
| 4.2 | 8795 | — | 0.00 | missing | Log trait blanket-impl declaration lines |
| 4.3 | 8921 | — | 0.00 | missing | kv::Source blanket-impl declaration lines |
| 4.4 | 8987 | — | 0.00 | missing | kv::Value numeric From-impl invocation |
| 4.5 | 9085 | — | 0.00 | missing | kv::Value NonZero From-impl invocation |
| 4.6 | 9319 | — | 0.00 | missing | kv::Value::to_X extractor macro invocation |
| 4.7 | 9654 | 3490 | 0.85 | early | log_enabled! macro body |
| 4.8 | 9734 | — | 0.11 | partial | __log_logger — global-vs-user logger dispatch |
| 4.9 | 9932 | — | 0.00 | missing | tests/integration.rs — filter-matrix test helper |
| 4.10 | 9965 | — | 0.00 | missing | test_max_level_features — declared features |
| 4.11 | 10069 | — | 0.00 | missing | CHANGELOG — latest release (0.4.29) headline |
| 4.13 | 10201 | — | 0.00 | missing | License preamble + triagebot stub |

## Walker waste (cost ≥ 50, no NS intersection)

| first_t | cost | key |
|--------:|-----:|:----|
| 343 | 86 | Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/log/README.md", section_index: 1 }) |
| 3802 | 59 | Rust(MethodSigs { file: "/Users/yoav/projects/precis/tests/fixtures/log/test_max_level_features/main.rs" }) |
| 3878 | 76 | Toml(Identity { file: "/Users/yoav/projects/precis/tests/fixtures/log/test_max_level_features/Cargo.toml" }) |
| 4699 | 73 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1351 }) |
| 5067 | 100 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 475 }) |
| 5189 | 122 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 636 }) |
| 5974 | 91 | Rust(PubItemNames { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/__private_api.rs" }) |
| 6067 | 93 | Rust(PubItem { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/__private_api.rs", start_line: 84 }) |
| 6120 | 53 | Rust(ModUse { file: "/Users/yoav/projects/precis/tests/fixtures/log/test_max_level_features/main.rs" }) |
| 6318 | 198 | Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/log/README.md", section_index: 0 }) |
| 6608 | 164 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1420 }) |
| 6805 | 197 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1200 }) |
| 6906 | 67 | Rust(MacroNames { src_dir: "/Users/yoav/projects/precis/tests/fixtures/log/src/kv" }) |
| 7119 | 213 | Rust(MacroBodies { src_dir: "/Users/yoav/projects/precis/tests/fixtures/log/src/kv" }) |
| 7218 | 89 | Rust(CrateDocLede { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/kv/mod.rs" }) |
| 7682 | 213 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1396 }) |
| 8002 | 259 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1529 }) |
| 8329 | 327 | Markdown(Section { file: "/Users/yoav/projects/precis/tests/fixtures/log/README.md", section_index: 3 }) |
| 8983 | 267 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1375 }) |
| 9534 | 410 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1158 }) |
| 9956 | 422 | Rust(PubItemDoc { file: "/Users/yoav/projects/precis/tests/fixtures/log/src/lib.rs", start_line: 1003 }) |
