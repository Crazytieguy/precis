# log — North Star

Revision pin: `43f2c283`

The fixture is `rust-lang/log` 0.4.29 — a logging *facade* crate. It defines the abstract interface (Level/LevelFilter enums, Log trait, Record/Metadata, the five level macros, global `set_logger`/`max_level`) but ships no concrete logger; downstream "logger implementation" crates (env_logger, slog-stdlog, etc.) plug in. A separately featured `kv` module adds structured key-value attributes to records, deliberately small in core and pluggable through `value-bag` / `serde` / `sval`.

Two facts shape ranking heavily:
- **Compile-time level filtering** lives in Cargo features (`max_level_*`, `release_max_level_*`) feeding `STATIC_MAX_LEVEL`. Agents that don't see this will recommend wrong things.
- **`set_logger` is one-shot global** with `&'static dyn Log`. The whole `STATE`/`AtomicUsize` dance and `set_boxed_logger` only exist because of this.

The fixture also ships a 13.6k-token *historical* RFC under `rfcs/` (excluded from the published crate via `Cargo.toml`'s `exclude`); it is design-history, not implementation-truth, and lands well below-the-fold.

## Batches

### 1.1 top-level filesystem listing
- Content: rendered listing of repo root (folders ending in `/`, then files):
  ```
  .github/
  benches/
  rfcs/
  src/
  test_max_level_features/
  tests/
  .gitignore
  CHANGELOG.md
  Cargo.toml
  LICENSE-APACHE
  LICENSE-MIT
  README.md
  triagebot.toml
  ```
- Cost: 46 tokens (helper: `echo "..." | scripts/count-tokens.py --stdin`)
- Notes: orients the agent immediately. Tiny, indispensable.

### 1.2 `src/` listing
- Content: `kv/` (folder), `__private_api.rs`, `lib.rs`, `macros.rs`, `serde.rs`
- Cost: 17 tokens (helper: `echo "kv/\n__private_api.rs\nlib.rs\nmacros.rs\nserde.rs" | scripts/count-tokens.py --stdin`)
- Notes: tells the agent the implementation is just five files plus `kv/`.

### 1.3 `Cargo.toml` package metadata
- Content: `tests/fixtures/log/Cargo.toml:1-17` (package block + readme/repo/docs/categories/keywords/exclude/rust-version/edition)
- Cost: 120 tokens (helper: `count-tokens.py Cargo.toml:1-17`)
- Notes: pins crate name `log`, version 0.4.29, MSRV 1.68, license `MIT OR Apache-2.0`, repo URL. Critical for "what version is this" and "where do I file issues".

### 1.4 `Cargo.toml` features block
- Content: `tests/fixtures/log/Cargo.toml:19-54` (docs.rs metadata + every `[features]` entry: `max_level_*`, `release_max_level_*`, `std`, `kv`, `kv_sval`, `kv_std`, `kv_serde`, `serde` alias)
- Cost: ~270 tokens (helper: `count-tokens.py Cargo.toml:22-54`)
- Notes: catastrophic-omission risk if missing — agent won't know about static max-level features or kv backends. The feature *names* themselves are the load-bearing content.

### 1.5 crate-level intro paragraph
- Content: `tests/fixtures/log/src/lib.rs:11-46` (the "lightweight logging facade" doc, target/level/body model, list of five macros, "side effects in log statements" warning)
- Cost: 385 tokens (helper: `count-tokens.py src/lib.rs:11-46`)
- Notes: this is the single best paragraph for understanding the crate's purpose.

### 1.6 `src/kv/` listing
- Content: `error.rs`, `key.rs`, `mod.rs`, `source.rs`, `value.rs`
- Cost: 15 tokens (helper: `echo "error.rs\nkey.rs\nmod.rs\nsource.rs\nvalue.rs" | scripts/count-tokens.py --stdin`)
- Notes: confirms `kv` is structured into the four canonical concepts (Source/Key/Value/Error).

### 2.1 the five level macros — names and signatures-by-example
- Content: `tests/fixtures/log/src/macros.rs:149-186` (the `error!` macro: full doc-comment showing `error!("...")`, `error!(target: "...", "...")`, `error!(logger: my_logger, "...")` plus the four arms). One macro is enough; `warn!`/`info!`/`debug!`/`trace!` are line-for-line analogous and that fact can be inferred from the macro name list in 1.5.
- Cost: 465 tokens (helper: `count-tokens.py src/macros.rs:149-186`)
- Notes: covers the most-used surface of the entire crate. The doc-comment at the top is what answers "how do I call this".

### 2.2 `Level` enum (public definition + variants only)
- Content: `tests/fixtures/log/src/lib.rs:467-499` (doc-comment + `pub enum Level { Error=1, Warn, Info, Debug, Trace }` with each variant's doc)
- Cost: 277 tokens (helper: `count-tokens.py src/lib.rs:467-499`)
- Notes: the `Error = 1` discriminant alignment with `LevelFilter` is non-obvious and explained in the source comment — keep that comment.

### 2.3 `LevelFilter` enum (public definition + variants only)
- Content: `tests/fixtures/log/src/lib.rs:626-650` (doc + `pub enum LevelFilter { Off, Error, Warn, Info, Debug, Trace }`)
- Cost: 206 tokens (helper: `count-tokens.py src/lib.rs:626-650`)
- Notes: the `Off` variant is what makes this distinct from `Level`. Catastrophic omission if dropped.

### 2.4 `Log` trait
- Content: `tests/fixtures/log/src/lib.rs:1248-1281` (full trait incl. doc-comments on `enabled`, `log`, `flush` — including the "enabled is *not* necessarily called before log" implementor note)
- Cost: 278 tokens (helper: `count-tokens.py src/lib.rs:1248-1281`)
- Notes: this is the trait every logger implementation needs to satisfy. The implementor notes are non-obvious gotchas that catch people.

### 2.5 global initialization functions — signatures + short docs
- Content: `tests/fixtures/log/src/lib.rs:1344-1404` covering `set_max_level`, `set_max_level_racy`, `max_level`. Body of `set_max_level` and `max_level` are tiny; included.
- Cost: 622 tokens (helper: `count-tokens.py src/lib.rs:1344-1404`)
- Notes: pairs naturally with 2.6.

### 2.6 `set_logger` / `set_boxed_logger` — signatures + docs
- Content: `tests/fixtures/log/src/lib.rs:1406-1481` (doc-comment for `set_boxed_logger`, declaration; doc-comment + example for `set_logger`, declaration). Body of `set_logger_inner` deferred.
- Cost: 527 tokens (helper: `count-tokens.py src/lib.rs:1406-1481`)
- Notes: the example block embedded in the doc-comment is itself a near-complete "how do I write a logger" recipe.

### 2.7 `kv` module intro — what structured logging is in this crate
- Content: `tests/fixtures/log/src/kv/mod.rs:1-77` (everything from `//! Structured logging.` through the capture-modifier table `:?, :debug, :%, :display, :err, :sval, :serde` and feature-gating notes)
- Cost: 694 tokens (helper: `count-tokens.py src/kv/mod.rs:1-77`)
- Notes: catastrophic-omission risk if missing — answers "how do I attach typed fields to a log line" entirely. The capture-modifier table is the API agents most often need to look up.

### 2.8 `STATIC_MAX_LEVEL` and the compile-time max-level cascade
- Content: `tests/fixtures/log/src/lib.rs:1602-1624` (doc + the `match cfg!(debug_assertions)` cascade picking the right `LevelFilter` from features)
- Cost: 305 tokens (helper: `count-tokens.py src/lib.rs:1602-1624`)
- Notes: closes the loop with batch 1.4. Without this, the features in 1.4 look mysterious.

### 3.1 `Record` struct + accessors (signature group)
- Content: `tests/fixtures/log/src/lib.rs:796-961` (doc on `Record` including the SimpleLogger example, the struct fields including `#[cfg(feature = "kv")] key_values`, then all the `&self` accessors `args/metadata/level/target/module_path/module_path_static/file/file_static/line/key_values/to_builder`)
- Cost: 1124 tokens (helper: `count-tokens.py src/lib.rs:796-961`)
- Notes: this is the "what does a logger see" data model. Within 2× of the largest prior batch (3.x max so far is 731 ⇒ cap 1462; ok).

### 3.2 `Metadata` struct + accessors
- Content: `tests/fixtures/log/src/lib.rs:1119-1182` (Metadata doc-comment with the `MyLogger` example, struct, level/target accessors)
- Cost: 345 tokens (helper: `count-tokens.py src/lib.rs:1119-1182`)
- Notes: small; pairs with Record.

### 3.3 `kv::Source` trait (the contract)
- Content: `tests/fixtures/log/src/kv/source.rs:51-89` (the trait doc, `visit`/`get`/`count` declarations and implementor notes; the example block of the doc-comment is the one that demonstrates `impl VisitSource for Printer`)
- Cost: 291 tokens (helper: `count-tokens.py src/kv/source.rs:51-89`)
- Notes: the `(K, V)` and `&[(K, V)]` impls below are the most common way to create a Source; teased here, full impl in 4.4.

### 3.4 `kv::VisitSource` trait
- Content: `tests/fixtures/log/src/kv/source.rs:234-238` (the trait declaration: `visit_pair(key, value) -> Result<(), Error>`)
- Cost: 62 tokens (helper: `count-tokens.py src/kv/source.rs:234-238`)
- Notes: tiny but central — it's the consumer side of structured logging.

### 3.5 `kv::Value` type docstring (data model + capture/serialization paths)
- Content: `tests/fixtures/log/src/kv/value.rs:33-122` (the long doc-comment listing capture methods, the data-model bullet list — Null/Strings/Booleans/Integers/Floats/Errors/serde/sval — and the serialization commentary; ends just before `impl Value`)
- Cost: 731 tokens (helper: `count-tokens.py src/kv/value.rs:33-122`)
- Notes: load-bearing for any "what types can I log as values" question.

### 3.6 `kv::Key` (full file minus impls)
- Content: `tests/fixtures/log/src/kv/key.rs:1-91` (`ToKey` trait, `Key` struct, `from_str`/`as_str`/`to_borrowed_str`, and the trivial `Display`/`AsRef`/`Borrow`/`From<&str>` impls)
- Cost: 640 tokens (helper: `count-tokens.py src/kv/key.rs:1-91`)
- Notes: small file; cheaper to include in full than to split.

### 3.7 `kv::Error` (full surface, minus std-only ext)
- Content: `tests/fixtures/log/src/kv/error.rs:1-66` (Error struct, `msg` constructor, the `Inner` variants enumerating Boxed/Msg/Value/Fmt, Display impl)
- Cost: 455 tokens (helper: `count-tokens.py src/kv/error.rs:1-66`)
- Notes: short and self-contained.

### 3.8 the `log!` master macro — definition (one of five arms shown in full)
- Content: `tests/fixtures/log/src/macros.rs:73-115` (the `#[macro_export] macro_rules! log` with all four arms covering `logger:`/`target:`/both/neither variants)
- Cost: 355 tokens (helper: `count-tokens.py src/macros.rs:73-115`)
- Notes: shows the master shape that the five level macros all delegate to.

### 4.1 `Level` impls (FromStr/Display/methods)
- Content: `tests/fixtures/log/src/lib.rs:501-624` (PartialEq<LevelFilter>, PartialOrd<LevelFilter>, FromStr, Display, then `impl Level` with `from_usize`/`max`/`to_level_filter`/`as_str`/`iter`/`increment_severity`/`decrement_severity` — the latter two with embedded `assert_eq!` examples)
- Cost: 923 tokens (helper: `count-tokens.py src/lib.rs:501-624`)
- Notes: predecessor 2.2.

### 4.2 `LevelFilter` impls (mirroring 4.1)
- Content: `tests/fixtures/log/src/lib.rs:651-778` (PartialEq<Level>, PartialOrd<Level>, FromStr, Display, `impl LevelFilter` with `from_usize`/`max`/`to_level`/`as_str`/`iter`/`increment_severity`/`decrement_severity`)
- Cost: 1013 tokens (helper: `count-tokens.py src/lib.rs:651-778`)
- Notes: predecessor 2.3. Mirror of 4.1 but with the `Off` variant handled.

### 4.3 `RecordBuilder` (struct + builder methods)
- Content: `tests/fixtures/log/src/lib.rs:962-1117` (the doc-comment with the `Record::builder().args(...).level(...).target(...)` example, then `RecordBuilder`, `new`, and each setter `args/metadata/level/target/module_path/module_path_static/file/file_static/line/key_values/build` plus `Default`)
- Cost: 1177 tokens (helper: `count-tokens.py src/lib.rs:962-1117`)
- Notes: predecessor 3.1. Heavy with examples; agents writing test loggers need it. Within 2× of the largest prior batch (4.2 at 1013 ⇒ cap 2026).

### 4.4 `kv::Source` blanket impls for tuples / slices / arrays / Option
- Content: `tests/fixtures/log/src/kv/source.rs:147-232` (`impl<K,V> Source for (K,V)`, `impl Source for [S]`, `impl Source for [S; N]`, `impl Source for Option<S>`)
- Cost: 514 tokens (helper: `count-tokens.py src/kv/source.rs:147-232`)
- Notes: predecessor 3.3. These are how users construct Sources — `&[("a", 1), ("b", 2)]` works because of these.

### 4.5 `kv::Value` constructors and `ToValue` trait
- Content: `tests/fixtures/log/src/kv/value.rs:1-32` (`ToValue` trait + blanket) plus `tests/fixtures/log/src/kv/value.rs:118-220` (`Value` struct + `from_any`/`from_debug`/`from_display`/`from_serde`/`from_sval`/`from_dyn_debug`/`from_dyn_display`/`from_dyn_error`/`null`/`from_inner`/`visit` + Debug/Display/Serialize/sval impls)
- Cost: 847 tokens (helper: `count-tokens.py src/kv/value.rs:1-32 src/kv/value.rs:118-220`; 162 + 685)
- Notes: predecessor 3.5. Demoted body of constructors below the trait surface so a smaller budget gets the docstring (3.5) before constructor signatures.

### 4.6 `kv::VisitValue` trait body
- Content: `tests/fixtures/log/src/kv/value.rs:450-536` (the `VisitValue<'v>` trait with all `visit_any`/`visit_null`/`visit_u64`/`visit_i64`/`visit_u128`/`visit_i128`/`visit_f64`/`visit_bool`/`visit_str`/`visit_borrowed_str`/`visit_char`/`visit_error`/`visit_borrowed_error` defaults)
- Cost: 751 tokens (helper: `count-tokens.py src/kv/value.rs:450-536`)
- Notes: predecessor 3.5. Mirrors VisitSource (3.4) on the value side.

### 4.7 `__private_api` — `GlobalLogger` and `log_impl`
- Content: `tests/fixtures/log/src/__private_api.rs:36-110` (the `GlobalLogger` struct, its `Log` impl proxying to `logger()`, `log_impl`, the public `log` and `enabled` shims, `loc`)
- Cost: ~440 tokens (helper: `count-tokens.py src/__private_api.rs:36-53 src/__private_api.rs:55-101 src/__private_api.rs:103-110`; 79 + 283 + 63 ≈ 425)
- Notes: this is the bridge between the macros (which call `__private_api::log`) and the public `Log` trait. Demystifies "what does `info!()` actually do".

### 4.8 `set_logger_inner`, `set_logger_racy`, `logger()`
- Content: `tests/fixtures/log/src/lib.rs:1482-1601` (the `compare_exchange`-based `set_logger_inner` body, then `set_logger_racy`, the `SetLoggerError`/`ParseLevelError` types, then `logger()` which reads STATE and either returns NopLogger or the installed one)
- Cost: 888 tokens (helper: `count-tokens.py src/lib.rs:1482-1601`)
- Notes: predecessor 2.6. The `STATE`/`UNINITIALIZED`/`INITIALIZING`/`INITIALIZED` machinery here is what makes `set_logger` one-shot.

### 4.9 atomics fallback for no-`target_has_atomic="ptr"` targets
- Content: `tests/fixtures/log/src/lib.rs:412-466` (the `cfg`-gated `Cell`-backed `AtomicUsize` shim + `unsafe impl Sync` + the `LOGGER`/`STATE`/`MAX_LOG_LEVEL_FILTER`/`LOG_LEVEL_NAMES` static declarations)
- Cost: 447 tokens (helper: `count-tokens.py src/lib.rs:412-466`)
- Notes: makes embedded support visible; otherwise the agent won't know `log` works on `thumbv6`.

### 4.10 `NopLogger` + foreign `Log` impls (`&T`, `Box<T>`, `Arc<T>`)
- Content: `tests/fixtures/log/src/lib.rs:1282-1342` (`NopLogger` declaration + Log impl, then the `impl<T: Log> Log for &T`, `Box<T>`, `Arc<T>` blanket impls)
- Cost: 323 tokens (helper: `count-tokens.py src/lib.rs:1282-1342`)
- Notes: predecessor 2.4. Without this an agent might think they need to wrap their own Box.

### 4.11 `kv::ToValue` primitive impls (the `impl_to_value_primitive!` macro invocations and the `impl_value_to_primitive!` invocations)
- Content: `tests/fixtures/log/src/kv/value.rs:288-374` (the three `macro_rules!` definitions + the actual invocations enumerating `usize, u8..u128, isize, i8..i128, f32, f64, char, bool` and `NonZeroUsize..NonZeroI128`, then the `impl_value_to_primitive!` listing `to_u64`/`to_i64`/`to_u128`/`to_i128`/`to_f64`/`to_char`/`to_bool`)
- Cost: 697 tokens (helper: `count-tokens.py src/kv/value.rs:288-374`)
- Notes: predecessor 4.5. The exhaustive list of types that implement `ToValue` is the answer to "can I log a NonZeroI32?" — this avoids a Grep.

### 4.12 `log_enabled!` macro and `__log_enabled` / `__log_logger` internals
- Content: `tests/fixtures/log/src/macros.rs:359-437` (the public `log_enabled!` macro with examples + the `#[doc(hidden)]` `__log_enabled!` and `__log_logger!` machinery used by all the level macros)
- Cost: ~597 tokens (helper: `count-tokens.py src/macros.rs:359-411 src/macros.rs:425-437`; 527 + 70)
- Notes: predecessor 2.1, 3.8. Surfaces the `log_enabled!(Level::Debug)` performance idiom.

### 4.13 `kv` capture-modifier macros (`__log_key`, `__log_value`, `__log_value_*`)
- Content: `tests/fixtures/log/src/macros.rs:439-579` (the cfg-gated kv capture machinery: how `key:? = expr` becomes `capture_debug(...)`, how `:err`/`:sval`/`:serde` route to `kv_std`/`kv_sval`/`kv_serde` features with helpful `compile_error!`s when the feature is off)
- Cost: 946 tokens (helper: `count-tokens.py src/macros.rs:439-579`)
- Notes: predecessor 2.7, 3.8. The `compile_error!` arms are the answer to "why doesn't `:serde` compile" — load-bearing for diagnostics.

### 4.14 README usage examples (yak-shaving + structured-logging snippet)
- Content: `tests/fixtures/log/README.md:25-134` (the "In libraries" Rust snippet + "In executables" with the bullet list of logger-implementation crates, then the "Structured logging" yak-shaving example)
- Cost: 1090 tokens (helper: `count-tokens.py README.md:25-134`)
- Notes: the bullet list of logger-implementation crates is *named*, which is the answer to "what loggers exist". Agents asked "should I use env_logger or simple_logger" benefit hugely.

## Below-the-fold

- `tests/fixtures/log/src/macros.rs:188-357` — the `warn!`/`info!`/`debug!`/`trace!` macros (~2020 tokens). Each is line-for-line analogous to the `error!` macro shown in 2.1, with only the level constant and the doc-comment example differing. The fact that all five exist is in 1.5; the call shape is in 2.1. A user-query that depends on `info!`'s exact arms can land via `Grep info`.
- `tests/fixtures/log/src/lib.rs:1626-2009` — the `mod tests` block (~2400 tokens). Behavior is fully implied by the public API. Agents working *on* the test suite can `Read` the file end with one tool call.
- `tests/fixtures/log/src/lib.rs:1-9, 348-354, 357-403` — copyright header, `html_logo_url`/`html_root_url`/`#![warn]`/`#![cfg_attr]` attributes, and the long `compile_error!("multiple max_level_* features set")` cfg-cascade. The cascade is mechanical (every pairwise combination of features forbidden); the *fact* that simultaneous max_level features are an error is implied by 1.4. ~600 tokens.
- `tests/fixtures/log/src/serde.rs` (entire 397-line file, 2853 tokens) — `Serialize`/`Deserialize` impls for `Level`/`LevelFilter` are mechanically derivable from `LOG_LEVEL_NAMES` and the `FromStr` impl already shown in 4.1/4.2. The agent can `Read` it in one hop if a serde question requires it. The file is also gated on `feature = "serde_core"`.
- `tests/fixtures/log/src/__private_api.rs:112-152` (the `kv_support` mod with `capture_to_value`/`capture_debug`/`capture_display`/`capture_error`/`capture_sval`/`capture_serde`) — implementations are one-line each, fully predictable from 4.14.
- `tests/fixtures/log/src/kv/source.rs:90-128` (default `get`/`count` impls), `233-516` (Box/Arc/Rc/Vec/HashMap/BTreeMap/dyn-VisitSource impls, fmt::DebugMap impls, std-mod tests). All highly predictable from 3.3+4.4. ~2200 tokens.
- `tests/fixtures/log/src/kv/value.rs:376-1396` (the `to_borrowed_error`/`to_borrowed_str`/`to_cow_str` accessors, `from_inner`, the entire `inner` mod which has two implementations — the value-bag one that's a thin Visit adapter, and the dependency-free fallback enum with all its 23 `From<T>` impls and `to_*` conversions and Display/Debug — plus the deprecated `capture_*` and `as_*` macros, and ~220 lines of unit tests). ~9300 tokens. The agent that needs the actual coercion semantics can land precisely via 3.5/4.5.
- `tests/fixtures/log/src/kv/key.rs:92-164` — `std_support`, `sval_support`, `serde_support` mods (each just plumbs `Key` through the relevant trait) and the two unit tests. Mechanical. ~400 tokens.
- `tests/fixtures/log/tests/integration.rs` (785 tokens) and `tests/fixtures/log/tests/macros.rs` (4437 tokens) — exercise the public API; not informative beyond what it reveals. The `kv_no_args`/`kv_expr_args` blocks in `macros.rs` do show some uncommon kv-syntax forms (`cat_math = { let mut x = 0; ... }`); a dedicated query can land via the file name from 1.1.
- `tests/fixtures/log/test_max_level_features/` (Cargo.toml + main.rs, ~580 tokens combined) — a tiny harness that asserts `max_level_debug`/`release_max_level_info` cuts off at the right level. The *existence* of this harness is implied by 1.4 + 2.8.
- `tests/fixtures/log/benches/value.rs` (155 tokens) — four `#[bench]`s on `Value::from`/`Value::from_debug`. Trivial.
- `tests/fixtures/log/CHANGELOG.md` (6196 tokens) — release history back through 0.4.x. Useful for "when did X land" but not for any present-day API question. One Read away.
- `tests/fixtures/log/rfcs/0296-structured-logging.md` (13634 tokens) — the *original* structured-logging RFC from 2019. The crate excludes `rfcs/**/*` from publication (`Cargo.toml:15`), confirming this is design-history. Anything in the RFC that is still true is reflected in `src/kv/`; anything not is wrong-by-default. Including any of it would crowd out current source.
- `tests/fixtures/log/.github/workflows/main.yml` (~520 tokens) — CI matrix. Useful for debugging CI but not for development questions about `log`-the-crate. The MSRV (1.68.0) is already in 1.3.
- `tests/fixtures/log/triagebot.toml` (1 line, `[assign]`), `tests/fixtures/log/.gitignore`, `tests/fixtures/log/LICENSE-APACHE`, `tests/fixtures/log/LICENSE-MIT` — meta-files with no semantic content for development tasks.

**Cut-off rationale.** The fixture totals ~71k tokens; the ranked content above sums to ~18.5k. The omitted ~52k is dominated by (a) the historical RFC (~13k, design-history excluded from the published crate), (b) `kv/value.rs` internals (~9k, mostly two parallel `inner` implementations of the same data model that the agent only needs if it's debugging value-bag interop), (c) the CHANGELOG (~6k), (d) tests (~5k), (e) `serde.rs` Serialize/Deserialize (~3k, mechanical from `FromStr` + `LOG_LEVEL_NAMES`), and (f) the four redundant `warn!`/`info!`/`debug!`/`trace!` macros (~2k). All of these are at most one targeted `Read` away once the agent knows the file exists, and the listings in 1.1/1.2/1.6 ensure they do.
