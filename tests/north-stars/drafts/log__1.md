# log — North Star

Revision pin: `43f2c283`

`log` is a tiny logging *facade* crate. It defines the macros (`error!`, `warn!`, `info!`, `debug!`, `trace!`, `log!`, `log_enabled!`), the `Log` trait that downstream loggers implement, the `Level`/`LevelFilter` enums, the `Record`/`Metadata` types, the global-logger state machine (`set_logger`/`set_max_level`/`logger()`/`STATIC_MAX_LEVEL`), and an opt-in `kv` module for structured key-value attributes (with sval/serde feature-gated capture). The dominant axes of plausible queries are: "how do I log X?" / "what features control level filtering?" / "how do I write a logger backend?" / "how does the kv stuff work?" — so the snapshot's job is to make those four directions cheap.

## Batches

### 1.1 Crate identity (one-line description and version)
- Content: `tests/fixtures/log/Cargo.toml:4` (version) plus `tests/fixtures/log/Cargo.toml:10-12` (`description = "A lightweight logging facade for Rust"`).
- Cost: 28 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/Cargo.toml:4 tests/fixtures/log/Cargo.toml:10-12`)
- Notes: needed before anything else makes sense; cheap and high-leverage.

### 1.2 Top-level filesystem entries
- Content: rendered listing of repo root: `.github/`, `benches/`, `rfcs/`, `src/`, `test_max_level_features/`, `tests/`, `.gitignore`, `CHANGELOG.md`, `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`, `triagebot.toml`.
- Cost: 46 tokens (helper: `printf '.github/\nbenches/\nrfcs/\nsrc/\ntest_max_level_features/\ntests/\n.gitignore\nCHANGELOG.md\nCargo.toml\nLICENSE-APACHE\nLICENSE-MIT\nREADME.md\ntriagebot.toml\n' | ./scripts/count-tokens.py --stdin`)
- Notes: cheapest "what's in this repo?" answer.

### 1.3 Names of the public logging macros
- Content: rendered list `error! warn! info! debug! trace! log! log_enabled!` (the macro_rules names from `src/macros.rs:75, 165, 204, 252, 292, 336, 391`).
- Cost: 15 tokens (helper: `printf 'error! warn! info! debug! trace! log! log_enabled!\n' | ./scripts/count-tokens.py --stdin`)
- Notes: tells the agent the macro surface exists without bodies. With this alone, an agent can guess `log::info!("…")` works.

### 1.4 `src/` and `src/kv/` listings
- Content: paths under `src/` (`__private_api.rs`, `lib.rs`, `macros.rs`, `serde.rs`, `kv/`) plus `src/kv/` (`error.rs`, `key.rs`, `mod.rs`, `source.rs`, `value.rs`).
- Cost: 54 tokens (helper: `printf 'src/__private_api.rs\nsrc/lib.rs\nsrc/macros.rs\nsrc/serde.rs\nsrc/kv/\nsrc/kv/error.rs\nsrc/kv/key.rs\nsrc/kv/mod.rs\nsrc/kv/source.rs\nsrc/kv/value.rs\n' | ./scripts/count-tokens.py --stdin`)
- Predecessor: 1.2
- Notes: completes the file-tree orientation by drilling into `src/`.

### 1.5 Crate abstract — facade concept paragraph
- Content: `tests/fixtures/log/src/lib.rs:11-19`.
- Cost: 95 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:11-19`)
- Notes: smallest possible semantic anchor — explains the *facade* concept and the noop fallback.

### 1.6 Crate abstract — log-request data model paragraph
- Content: `tests/fixtures/log/src/lib.rs:20-26`.
- Cost: 81 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:20-26`)
- Predecessor: 1.5
- Notes: defines target/level/body — useful on its own once 1.5 is in context.

### 1.7 `kv` module re-exports
- Content: `tests/fixtures/log/src/kv/mod.rs:246-266` — what `log::kv` exposes (`Source`, `VisitSource`, `Key`, `ToKey`, `Value`, `ToValue`, `VisitValue`, `Error`, plus the deprecated `kv_unstable::Visitor` alias).
- Cost: 121 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/mod.rs:246-266`)
- Notes: the public surface of the `kv` module in one glance — tells the agent the kv types exist before any deeper kv content lands.

### 1.8 `LevelFilter` enum (variants + doc)
- Content: `tests/fixtures/log/src/lib.rs:626-649`.
- Cost: 206 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:626-649`)
- Notes: the *filter* type, distinct from `Level` (admits `Off`); fundamental for `set_max_level`.

### 1.9 `Cargo.toml` features
- Content: `tests/fixtures/log/Cargo.toml:22-54` — the `[features]` table (`std`, `kv`, `kv_sval`, `kv_std`, `kv_serde`, `serde`, the 12 `*max_level_*` toggles, deprecated `kv_unstable*` aliases).
- Cost: 275 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/Cargo.toml:22-54`)
- Notes: the feature-flag landscape is load-bearing — many code paths and even macro emissions are gated on these. Deps are split into 1.13.

### 1.10 `Level` enum (variants + doc)
- Content: `tests/fixtures/log/src/lib.rs:467-499`.
- Cost: 277 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:467-499`)
- Notes: defines the verbosity vocabulary used everywhere.

### 1.11 `Log` trait declaration
- Content: `tests/fixtures/log/src/lib.rs:1248-1280` — trait with `enabled`/`log`/`flush` methods and "for implementors" notes.
- Cost: 278 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1248-1280`)
- Notes: the abstraction every backend must implement. The "for implementors" doc bits are not derivable from the signatures.

### 1.12 `STATIC_MAX_LEVEL` constant
- Content: `tests/fixtures/log/src/lib.rs:1602-1624` — the `cfg!`-cascade resolving the compile-time level cap from `max_level_*` / `release_max_level_*` features.
- Cost: 305 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1602-1624`)
- Predecessor: 1.9
- Notes: the only place the `*max_level_*` features actually take effect.

### 1.13 `Cargo.toml` dependencies
- Content: `tests/fixtures/log/Cargo.toml:56-77` — `[dependencies]` (`serde_core`, `sval`, `sval_ref`, `value-bag`) plus `[dev-dependencies]` (`serde`, `serde_json`, `serde_test`, `sval`, `sval_derive`, `value-bag`) and the `proc-macro2` minimal-versions hack.
- Cost: 321 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/Cargo.toml:56-77`)
- Predecessor: 1.9
- Notes: the actual external crates wired in. Splitting deps from features keeps every batch ≤ 2× the largest above it.

Cumulative through 1.x: 2102 tokens. Largest batch so far: 321 (1.13).

### 2.1 `lib.rs` "Usage" doc section (basic example)
- Content: `tests/fixtures/log/src/lib.rs:27-90` — module-level doc covering basic macro use, `target:` argument, in-libraries vs in-executables guidance.
- Cost: 537 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:27-90`)
- Notes: the `info!(target: "yak_events", ...)` example is the canonical "how do I use this" answer.

### 2.2 Five level macros — top doc line + simplest arm of each
- Content: `tests/fixtures/log/src/macros.rs:149-150` (`error!` doc) + `:184-186` (its trailing default arm); same shape for `warn!` (`:188-189` + `:222-224`), `info!` (`:227-228` + `:270-272`), `debug!` (`:275-276` + `:310-312`), `trace!` (`:315-316` + `:354-356`). Render each macro section with an elision marker `…` between the doc and the simplest arm to indicate other arms exist.
- Cost: 240 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:149-150 tests/fixtures/log/src/macros.rs:184-186 tests/fixtures/log/src/macros.rs:188-189 tests/fixtures/log/src/macros.rs:222-224 tests/fixtures/log/src/macros.rs:227-228 tests/fixtures/log/src/macros.rs:270-272 tests/fixtures/log/src/macros.rs:275-276 tests/fixtures/log/src/macros.rs:310-312 tests/fixtures/log/src/macros.rs:315-316 tests/fixtures/log/src/macros.rs:354-356`)
- Predecessor: 1.3
- Notes: shows that `error!($($arg)+) => ($crate::log!($crate::Level::Error, $($arg)+))` — i.e., the level macros are thin shims over `log!`. Elision teases the other three arms (logger:/target:/logger:+target:) without forcing tokens for them.

### 2.3 Global state functions: `set_max_level`, `set_max_level_racy`, `max_level`
- Content: `tests/fixtures/log/src/lib.rs:1344-1404` — signatures with rustdoc.
- Cost: 622 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1344-1404`)
- Notes: every executable must call `set_max_level`; the rustdoc explains the perf rationale.

### 2.4 Global installer functions: `set_boxed_logger`, `set_logger`
- Content: `tests/fixtures/log/src/lib.rs:1406-1480`.
- Cost: 527 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1406-1480`)
- Notes: the only way to wire a `Log` impl into the global slot. The example block is canonical.

### 2.5 `kv` module synopsis (doc opening)
- Content: `tests/fixtures/log/src/kv/mod.rs:1-60` — opening doc explaining structured logging, the `info!(a = 1; "msg")` syntax, and the `:?`/`:%`/`:err`/`:sval`/`:serde` capturing-modifier table.
- Cost: 541 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/mod.rs:1-60`)
- Notes: gates correct understanding of the kv macro syntax.

### 2.6 Crate-internal use/mod statements
- Content: `tests/fixtures/log/src/lib.rs:396-410` — `mod macros; mod serde; #[cfg(feature = "kv")] pub mod kv;` plus the `use std::*` block.
- Cost: 74 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:396-410`)
- Notes: tells the agent the module map of the crate root.

Cumulative through 2.x: 4643 tokens (2102 + 2541). Largest batch so far: 622 (2.3).

### 3.1 `Record` doc + struct definition
- Content: `tests/fixtures/log/src/lib.rs:796-867` — `Record` rustdoc (with the `SimpleLogger` example) and the `Record<'a>` struct fields including the `KeyValues` wrapper.
- Cost: 541 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:796-867`)
- Notes: `Record` is the central data structure passed to every `Log::log` call.

### 3.2 `Record` accessor methods
- Content: `tests/fixtures/log/src/lib.rs:869-961` — `args()`, `metadata()`, `level()`, `target()`, `module_path()`/`_static()`, `file()`/`_static()`, `line()`, `key_values()`, `to_builder()`.
- Cost: 583 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:869-961`)
- Predecessor: 3.1
- Notes: the surface a `Log` implementor reads.

### 3.3 `Metadata` struct + accessors
- Content: `tests/fixtures/log/src/lib.rs:1119-1181` — `Metadata<'a>` struct and `level()`/`target()` methods (with rustdoc and the `MyLogger` example).
- Cost: 345 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1119-1181`)
- Notes: passed into `Log::enabled`; small enough not to defer.

### 3.4 `error!` macro full body (all four arms)
- Content: `tests/fixtures/log/src/macros.rs:149-186`.
- Cost: 465 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:149-186`)
- Predecessor: 2.2
- Notes: with this one macro shown in full plus a noted "warn/info/debug/trace are structurally identical at different levels", the agent has near-complete information to use or modify any of the five.

### 3.5 `log!` macro full body (entry-point macro the others delegate to)
- Content: `tests/fixtures/log/src/macros.rs:11-115` — doc + all four arms.
- Cost: 784 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:11-115`)
- Notes: shows precisely how the `logger:` and `target:` arguments compose. With 3.4, the macro family is fully specified.

### 3.6 `log_enabled!` macro
- Content: `tests/fixtures/log/src/macros.rs:359-411`.
- Cost: 527 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:359-411`)
- Notes: critical for "avoid expensive computation if disabled" usage pattern.

### 3.7 `__log` and `__log_logger` internal macros
- Content: `tests/fixtures/log/src/macros.rs:117-147` (`__log`) and `tests/fixtures/log/src/macros.rs:425-437` (`__log_logger`).
- Cost: 445 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:117-147 tests/fixtures/log/src/macros.rs:425-437`)
- Notes: how the public macros actually call into `__private_api::log` with the static-level guard. Worth showing for anyone debugging macro expansion or codegen.

### 3.8 `__private_api` log/enabled/loc + `GlobalLogger`
- Content: `tests/fixtures/log/src/__private_api.rs:1-110` — `GlobalLogger` struct, `log_impl`/`log` functions, `enabled`, `loc()`, the `KVs` sealed trait.
- Cost: 681 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/__private_api.rs:1-110`)
- Predecessor: 3.7
- Notes: shows what `__log!` actually invokes; explains the `track_caller` location capture and how `Record` is built before hand-off.

### 3.9 `kv::Key` (full file minus cfg-gated impls and tests)
- Content: `tests/fixtures/log/src/kv/key.rs:1-91` — `ToKey` trait, `Key<'k>` struct, `from_str`/`as_str`/`to_borrowed_str`, `Display`/`AsRef`/`Borrow`/`From<&str>` impls.
- Cost: 640 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/key.rs:1-91`)
- Notes: small enough to show whole; the public Key API is heavily relied on by both producers and consumers of structured records.

### 3.10 `kv::Source` trait + module doc + default helpers
- Content: `tests/fixtures/log/src/kv/source.rs:1-128` — module doc with the `Printer` visitor example, `Source` trait with `visit`/`get`/`count`, `get_default` and `count_default`.
- Cost: 928 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/source.rs:1-128`)
- Notes: the central trait for collections of key-values. Largest batch so far at 928 — within 2× of 622 (2.3).

### 3.11 `VisitSource` trait + `fmt::Debug*` adapters
- Content: `tests/fixtures/log/src/kv/source.rs:234-277` — `VisitSource` trait + impls for `&mut T`, `fmt::DebugMap`/`DebugList`/`DebugSet`/`DebugTuple`.
- Cost: 457 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/source.rs:234-277`)
- Predecessor: 3.10
- Notes: explains how visitors plug into `fmt` machinery — a common need when implementing a logger that pretty-prints records.

Cumulative through 3.x: 11039 tokens. Largest batch: 928 (3.10).

### 4.1 `kv::Value` rustdoc + struct definition
- Content: `tests/fixtures/log/src/kv/value.rs:33-122`.
- Cost: 731 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/value.rs:33-122`)
- Notes: explains the type-erasure concept; the data-model bullet list (Null/Strings/Booleans/Integers/Floats/Errors/serde/sval) is hard to derive from code alone.

### 4.2 `kv::Value` `from_*` constructors and `visit`
- Content: `tests/fixtures/log/src/kv/value.rs:123-220` — `from_any`/`from_debug`/`from_display`/`from_serde`/`from_sval`/`from_dyn_debug`/`from_dyn_display`/`from_dyn_error`/`null`/`from_inner`/`visit`.
- Cost: 664 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/value.rs:123-220`)
- Predecessor: 4.1
- Notes: producer-side API for constructing values manually.

### 4.3 `kv::Value` primitive `From`/`ToValue` impls + `to_*` extractors
- Content: `tests/fixtures/log/src/kv/value.rs:258-374` — `impl ToValue for str/()/Option`, the `impl_to_value_primitive!`/`impl_to_value_nonzero_primitive!` macros, and the `impl_value_to_primitive!`-generated `to_u64`/`to_i64`/`to_u128`/`to_i128`/`to_f64`/`to_char`/`to_bool` extractors.
- Cost: 851 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/value.rs:258-374`)
- Predecessor: 4.1
- Notes: the consumer-side conversion API plus the implicit conversions that make `info!(a = 1; ...)` compile.

### 4.4 `kv::VisitValue` trait
- Content: `tests/fixtures/log/src/kv/value.rs:450-536`.
- Cost: 751 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/value.rs:450-536`)
- Notes: lightweight alternative to `serde`/`sval` for value backends. Often the right starting point for someone writing a custom kv consumer.

### 4.5 `kv::Error`
- Content: `tests/fixtures/log/src/kv/error.rs:1-66`.
- Cost: 455 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/error.rs:1-66`)
- Notes: the error type returned by every visitor method. Includes the `Inner` enum, `msg`/`from_value`/`into_value`, `Display`, `From<fmt::Error>`.

### 4.6 `Log` blanket impls (`&T`, `Box<T>`, `Arc<T>`) + `NopLogger`
- Content: `tests/fixtures/log/src/lib.rs:1283-1342`.
- Cost: 315 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1283-1342`)
- Predecessor: 1.11
- Notes: the `NopLogger` defines the "default" no-op behavior before `set_logger`; the blanket impls explain why `&MyLogger` and `Box<dyn Log>` are interchangeable.

### 4.7 `RecordBuilder`
- Content: `tests/fixtures/log/src/lib.rs:1003-1111` — `RecordBuilder<'a>` with all setters and `build()`.
- Cost: 892 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1003-1111`)
- Predecessor: 3.1
- Notes: how custom code can construct a `Record` outside the macro path. Within 2× of 928 (3.10).

### 4.8 `MetadataBuilder`
- Content: `tests/fixtures/log/src/lib.rs:1199-1246`.
- Cost: 287 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1199-1246`)
- Predecessor: 3.3
- Notes: pairs with `RecordBuilder`.

### 4.9 Global-state machinery (statics + `set_logger_inner` + `set_logger_racy`)
- Content: `tests/fixtures/log/src/lib.rs:445-465` (LOGGER/STATE statics + UNINITIALIZED/INITIALIZING/INITIALIZED constants + level-name table + error strings) and `tests/fixtures/log/src/lib.rs:1483-1542` (`set_logger_inner`, `set_logger_racy`).
- Cost: 651 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:445-465 tests/fixtures/log/src/lib.rs:1483-1542`)
- Predecessor: 2.4
- Notes: the actual atomics protocol that gates installation. Important for "why didn't my logger get installed" / "why is my logger called twice".

### 4.10 `logger()` accessor + `SetLoggerError` / `ParseLevelError`
- Content: `tests/fixtures/log/src/lib.rs:1544-1596`.
- Cost: 427 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1544-1596`)
- Notes: small but load-bearing — `logger()` is how a backend can call into the currently-installed logger without going through the macros.

### 4.11 `Level` impl methods
- Content: `tests/fixtures/log/src/lib.rs:534-624` — `from_usize`, `max`, `to_level_filter`, `as_str`, `iter`, `increment_severity`, `decrement_severity`.
- Cost: 695 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:534-624`)
- Predecessor: 1.10
- Notes: the methods are largely paralleled by `LevelFilter`; we rank `Level`'s methods here and put `LevelFilter`'s impl block lower since the docs/structure repeat.

### 4.12 `Level`/`LevelFilter` cross-impls (FromStr, Display, ordering)
- Content: `tests/fixtures/log/src/lib.rs:501-532` (`Level`'s `PartialEq<LevelFilter>`, `PartialOrd<LevelFilter>`, `FromStr`, `Display`) and `tests/fixtures/log/src/lib.rs:651-682` (`LevelFilter`'s mirror impls).
- Cost: 519 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:501-532 tests/fixtures/log/src/lib.rs:651-682`)
- Predecessor: 1.8, 1.10
- Notes: cross-type comparison is what enables `level <= max_level_filter` checks in the macros' static guard.

### 4.13 `__private_api` kv_support module (`capture_*` functions)
- Content: `tests/fixtures/log/src/__private_api.rs:112-152` — `capture_to_value`/`capture_debug`/`capture_display`/`capture_error`/`capture_sval`/`capture_serde`.
- Cost: 369 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/__private_api.rs:112-152`)
- Predecessor: 3.8
- Notes: closes the loop between the macro `:?` / `:%` / `:err` / `:sval` / `:serde` modifiers and `kv::Value` constructors.

### 4.14 `kv` module — Working-with-key-values examples
- Content: `tests/fixtures/log/src/kv/mod.rs:60-130` — `record.key_values().get(...)` and `VisitSource` consumer-side examples.
- Cost: 603 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/mod.rs:60-130`)
- Predecessor: 2.5
- Notes: complements 2.5 (producer-side syntax) with the matching consumer-side recipes.

Cumulative through 4.x: 19249 tokens. Largest batch: 928 (3.10).

### 5.1 README structured-logging example
- Content: `tests/fixtures/log/README.md:106-135`.
- Cost: 227 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/README.md:106-135`)
- Notes: a second compact `kv` usage example.

### 5.2 `LevelFilter` impl block (methods)
- Content: `tests/fixtures/log/src/lib.rs:684-778` — `from_usize`/`max`/`to_level`/`as_str`/`iter`/`increment_severity`/`decrement_severity`.
- Cost: 789 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:684-778`)
- Predecessor: 1.8
- Notes: parallels 4.11 but for `LevelFilter` (which has the additional `Off` variant).

### 5.3 `lib.rs` "Implementing a Logger" doc walk-through
- Content: `tests/fixtures/log/src/lib.rs:172-256` — the `MyLogger` walkthrough explaining `set_logger`/`set_max_level`/`set_boxed_logger` interaction.
- Cost: 775 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:172-256`)
- Notes: long but the canonical "how do I write a logger" tutorial; pure docstring with one running example.

### 5.4 `lib.rs` "Compile time filters" + "Crate Feature Flags" docs
- Content: `tests/fixtures/log/src/lib.rs:257-303`.
- Cost: 432 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:257-303`)
- Notes: complements 1.9 with prose explaining what each feature toggle does.

### 5.5 `kv::Source` foundational impls (`(K,V)`, `[S]`, `[S;N]`, `Option<S>`, `&T`)
- Content: `tests/fixtures/log/src/kv/source.rs:130-232`.
- Cost: 623 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/source.rs:130-232`)
- Predecessor: 3.10
- Notes: explains why `&[("a", 1), ("b", 2)]` and `("a", 1)` both work as a `Source`.

Cumulative through 5.x: 22095 tokens.

### 6.1 `tests/integration.rs` (filter behavior end-to-end)
- Content: `tests/fixtures/log/tests/integration.rs` — full file, 101 lines. Demonstrates a minimal in-memory `Log` impl exercising every `LevelFilter` setting and the `track_caller` line-number capture.
- Cost: 785 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/tests/integration.rs`)
- Notes: the integration test is the simplest end-to-end usage example in the repo. Within 2× of 928 (3.10).

### 6.2 `benches/value.rs`
- Content: full file `tests/fixtures/log/benches/value.rs` — four `#[bench]` functions for `Value::from`/`Value::from_debug` on `u8`, `&str`, and a custom struct.
- Cost: 155 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/benches/value.rs`)
- Predecessor: 4.2
- Notes: tiny; cheap to include and tells the agent which `kv::Value` paths are perf-sensitive.

### 6.3 `test_max_level_features/main.rs` + `Cargo.toml`
- Content: `tests/fixtures/log/test_max_level_features/main.rs` and `tests/fixtures/log/test_max_level_features/Cargo.toml`.
- Cost: 544 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/test_max_level_features/main.rs tests/fixtures/log/test_max_level_features/Cargo.toml`)
- Predecessor: 1.12
- Notes: integration test with `max_level_debug` + `release_max_level_info` features. Confirms the `STATIC_MAX_LEVEL` cascade actually filters at compile time and is the only test in the repo covering release-vs-debug behavior.

### 6.4 CHANGELOG most-recent entries
- Content: `tests/fixtures/log/CHANGELOG.md:1-30` — the `[Unreleased]`, `[0.4.29]`, `[0.4.28]` entries.
- Cost: 437 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/CHANGELOG.md:1-30`)
- Notes: orients on what's new (e.g., `serde_core` substitution, `increment_severity`/`decrement_severity` methods).

Cumulative through 6.x: 24016 tokens. (Modestly over the ≲20k guideline; budgets that don't reach 6.x land naturally at ~22k after 5.x or ~19k after 4.x.)

## Below-the-fold

The strict ≲20k cap means the items below are not ranked. Each is one or two `Read`/`Grep` away from the cloned-repo state, and the snapshot's structural pointers (folder listings, `kv` re-exports, item names) make each location obvious — so omitting them avoids only a small per-query cost.

- **`rfcs/0296-structured-logging.md` (1651 lines, ~12k tokens)** — historical RFC for the kv module. `Cargo.toml`'s `exclude = ["rfcs/**/*"]` keeps it out of the published crate. Current implementation supersedes it; agents asking about *design history* of structured logging would benefit, but tokens are better spent on current code.
- **`CHANGELOG.md:31-423` (older releases, ~5800 tokens)** — release notes from 0.4.27 back to early 0.4.x. The most recent three entries are promoted in 6.4; deeper history is rarely load-bearing.
- **`LICENSE-APACHE`, `LICENSE-MIT`** — boilerplate license texts. Their *existence* is shown in 1.2.
- **`triagebot.toml`, `.gitignore`** — one-line config files; not load-bearing for any plausible development task.
- **`.github/workflows/main.yml` (1219 tokens)** — full CI pipeline (test/check/doc/features/minimalv/msrv/embedded jobs). The matrix and `cargo hack test --feature-powerset` invocation are the most informative parts; the rest is boilerplate for stable/beta/nightly/macos/win matrices.
- **`src/serde.rs` (entire file, 2853 tokens)** — `Serialize`/`Deserialize` impls for `Level`/`LevelFilter` plus their `serde_test` round-trip tests. The `LOG_LEVEL_NAMES` table referenced in `lib.rs:460` and the case-insensitive `FromStr` impls (already shown in 4.12) imply the wire shape (`"ERROR"`/`"WARN"`/...). Promote when the user query touches serde explicitly.
- **`src/kv/value.rs:601-718` (`value-bag` adapter, ~900 tokens)** — wires the external `value_bag` crate's visitor model into our `VisitValue`. Mostly mechanical mapping; relevant only to someone modifying the value-bag bridge.
- **`src/kv/value.rs:720-1049` (no-`value-bag` `Inner` enum + From impls + to_* converters, ~2650 tokens)** — long mechanical block. The `Inner` variants (`None`/`Bool`/`Str`/`Char`/`I64`/`U64`/`F64`/`I128`/`U128`/`Debug`/`Display`) and the public `to_*` methods (already shown in 4.3) tell the agent enough to find this on demand.
- **`src/kv/value.rs:376-448` (`to_borrowed_*` and `Cow<str>` impls, ~457 tokens)** — borrowed-string accessors used by zero-copy consumers. One `Read` away once `Value` is in context.
- **`src/kv/value.rs:1051-1173` (`#[deprecated]` `kv_unstable` shims, ~976 tokens)** — `capture_debug`/`capture_display`/`capture_error`/`capture_serde`/`capture_sval`/`is`/`downcast_ref` and the `as_debug!`/`as_display!`/`as_error!`/`as_sval!`/`as_serde!` macros. Encountered only in pre-0.4.21 code; deprecation messages already point to current syntax.
- **`src/kv/value.rs:1175-1396` (test fixtures, ~1900 tokens)** — long unit-test enumeration of value conversions. Behavior already specified by 4.2/4.3/4.4.
- **`src/kv/key.rs:93-149` (sval/serde/std support sub-modules, ~323 tokens)** — small cfg-gated impls that mostly delegate; `From`/`AsRef`/`Borrow` covered in 3.9 cover the load-bearing surface.
- **`src/kv/source.rs:279-407` (std-feature `Source`/`VisitSource` impls for `Box`/`Arc`/`Rc`/`Vec`/`HashMap`/`BTreeMap`, ~908 tokens)** — needed only when answering "can I pass a `HashMap` as my key-values?". The collection list itself is short; the impls are mechanical delegations.
- **`src/kv/source.rs:408-515` (test bodies, ~370 tokens)** — `count`, `get`, `hash_map`, `btree_map`, `source_is_object_safe`, `visitor_is_object_safe`. Their *names* already convey the invariants tested.
- **`src/lib.rs:357-394` (`compile_error!` guards for conflicting `max_level_*` features, 547 tokens)** — by-enumeration pair of compile_error blocks; their behavior is implied by the feature list (1.9) and `STATIC_MAX_LEVEL` (1.12).
- **`src/lib.rs:412-443` (non-atomic `AtomicUsize` polyfill, 218 tokens)** — `Cell`-based fallback for targets without `target_has_atomic = "ptr"`. Promote when discussing `thumbv6` or other no-atomics embedded targets.
- **`src/lib.rs:780-794` + `:852-867` (`MaybeStaticStr` and `KeyValues` helpers, ~242 tokens)** — implementation detail explaining the `Static`/`Borrowed` distinction visible in `Record::module_path` vs `module_path_static`. Names alone (already in `Record` accessors 3.2) imply the role.
- **`src/lib.rs:233-256` ("Use with std" doc, ~245 tokens)** — explains `set_boxed_logger` rationale; partly redundant with 2.4.
- **`src/lib.rs:305-346` (rustdoc link table, ~700 tokens)** — alias table mapping `set_logger` etc. to docs.rs URLs and the listing of available logger crates (env_logger, log4rs, …). README.md covers the same ground; the link table itself is reference data, not semantic.
- **`README.md:58-104` (logger-implementations bullet list, ~700 tokens)** — same content as `lib.rs:305-346` in markdown form. The first 56 lines (intro + basic-usage code) overlap with 1.5/1.6 + 2.1.
- **`src/kv/mod.rs:130-244` (further consumer examples — `IsNumeric` `VisitValue`, serde JSON, Debug formatting, ~1100 tokens)** — 2.5 + 4.14 already convey the consumer pattern; these are extended worked examples.
- **`tests/macros.rs:1-152` (basic shape-of-call tests, 1267 tokens)** — `no_args`, `anonymous_args`, `named_args`, `inlined_args`, `enabled`, `expr` test bodies. Exhaustive enumeration of every legal macro call shape but redundant with 2.2 + 3.4 + 3.5 once those are shown.
- **`tests/macros.rs:153-430` (kv-feature tests + regressions, ~3170 tokens)** — `kv_no_args`, `kv_expr_args`, `kv_anonymous_args`, `kv_named_args`, `kv_ident`, `kv_string_keys`, `kv_common_value_types`, `kv_debug`, `kv_display`, `kv_error`, `kv_sval`, `kv_serde`, `regression_issue_494`, `logger_short_lived`, `logger_expr`, `implicit_named_args`, `kv_implicit_named_args`. All shape regressions; the `:?`/`:%`/`:err` syntax shown in 2.5 + `tests/integration.rs` (6.1) cover the majority of "how do I call this" queries.
- **`src/lib.rs:1626-2009` (`mod tests`, ~3300 tokens)** — internal unit tests for `Level`/`LevelFilter` parsing/ordering, builders, `STATIC_MAX_LEVEL`, `Log` foreign impls. Behavior under test is specified by items already ranked.
