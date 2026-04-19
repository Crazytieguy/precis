# log — North Star

Revision pin: `43f2c283`

The `log` crate is the de-facto Rust logging *facade* — it defines an API (macros, `Log` trait, `Level`/`LevelFilter`, `Record`/`Metadata`) but ships no concrete logger; downstream binaries pick an implementation and install it via `set_logger`. The vast majority of user code touches only the five logging macros (`error!`, `warn!`, `info!`, `debug!`, `trace!`); everything else is plumbing for logger implementors and the optional structured (`kv`) extension.

## Batches

### 1.1 Top-level repo listing
- Content: rendered listing of fixture root entries — `CHANGELOG.md`, `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`, `benches/`, `rfcs/`, `src/`, `test_max_level_features/`, `tests/`, `triagebot.toml`
- Cost: 41 tokens (helper: `printf '...listing...\n' | count-tokens.py --stdin`)
- Notes: orients the agent that this is a small library crate plus a separate `test_max_level_features/` Cargo project and an `rfcs/` directory.

### 1.2 `src/` listing
- Content: rendered listing — `__private_api.rs`, `kv/`, `lib.rs`, `macros.rs`, `serde.rs`
- Cost: 17 tokens (helper: `printf '...\n' | count-tokens.py --stdin`)
- Notes: tells the agent the optional `kv` module exists and that there's a separate `serde.rs` and a `__private_api.rs` (macro hidden internals).

### 1.3 Cargo package metadata header
- Content: `Cargo.toml:1-21` (`[package]` block through `[package.metadata.docs.rs]`)
- Cost: 148 tokens (helper: `count-tokens.py Cargo.toml:1-21`)
- Notes: name `log`, version `0.4.29`, MSRV `1.68.0`, license `MIT OR Apache-2.0`, `exclude = ["rfcs/**/*"]` (so the rfcs dir is dev-only), and the docs.rs feature set used to render API docs.

### 1.4 Crate-level summary and basic library example
- Content: `src/lib.rs:11-77` (the `//! A lightweight logging facade.` paragraph through the `shave_the_yak` example showing `info!`/`warn!`)
- Cost: 621 tokens (helper: `count-tokens.py src/lib.rs:11-77`)
- Notes: smallest single batch that conveys *what* the crate is, the five-level model, and the canonical call shape.

### 1.5 `src/kv/` listing
- Content: rendered listing — `error.rs`, `key.rs`, `mod.rs`, `source.rs`, `value.rs`
- Cost: 15 tokens (helper: `printf '...\n' | count-tokens.py --stdin`)
- Notes: paired with 1.2 to make the structured-logging surface visible without spending tokens on bodies yet.

### 2.1 `Cargo.toml` features section
- Content: `Cargo.toml:22-55` (the entire `[features]` block, including `max_level_*`, `release_max_level_*`, `std`, `kv`/`kv_sval`/`kv_std`/`kv_serde`, `serde` alias, and the deprecated `kv_unstable*` aliases)
- Cost: 275 tokens (helper: `count-tokens.py Cargo.toml:22-55`)
- Notes: catastrophic-omission risk if missing — features are how users statically disable levels and how kv backends are gated. The exact feature names are load-bearing in `[dependencies.log]` blocks downstream.

### 2.2 Five logging macros — brief intro and target/logger syntax
- Content: `src/lib.rs:78-127` (executables section + the `kv` example showing `target:`, `yak:serde`, `e:err` syntaxes)
- Cost: 399 tokens (helper: `count-tokens.py src/lib.rs:78-127`)
- Notes: covers (a) that executables must initialize a logger, (b) the structured-logging shorthand `key:capture = value; "msg"`, (c) the `target:` named arg.

### 2.3 `Log` trait definition (the implementor contract)
- Content: `src/lib.rs:1247-1280` (`pub trait Log: Sync + Send` with `enabled`, `log`, `flush` and their rustdoc)
- Cost: 266 tokens (helper: `count-tokens.py src/lib.rs:1247-1280`)
- Notes: anyone writing or reading a logger implementation needs this exact trait shape.

### 2.4 `Level` and `LevelFilter` enum definitions
- Content: `src/lib.rs:473-499` (`pub enum Level` with `Error=1, Warn, Info, Debug, Trace`) plus `src/lib.rs:634-649` (`pub enum LevelFilter` with `Off, Error, Warn, Info, Debug, Trace`)
- Cost: 322 tokens (helper: `count-tokens.py src/lib.rs:473-499 src/lib.rs:634-649`)
- Notes: variant order and discriminants matter (severity ordering, `Off` only in `LevelFilter`). Cross-comparable with `==`/`<` (impls live elsewhere).

### 2.5 Global init/query functions — rustdoc-stripped signatures
- Content: `src/lib.rs:1349-1353` (`set_max_level`), `1395-1404` (`max_level`), `1419-1422` (`set_boxed_logger`), `1477-1480` (`set_logger`), `1581-1596` (`logger`)
- Cost: 419 tokens (helper: `count-tokens.py src/lib.rs:1349-1353 src/lib.rs:1395-1404 src/lib.rs:1419-1422 src/lib.rs:1477-1480 src/lib.rs:1581-1596`)
- Notes: the four init/query entry-points users call from their binary plus the `logger()` accessor. Includes `cfg` gates that show `set_boxed_logger` requires `std + atomic-ptr` and `set_logger`/`set_max_level` require atomic-ptr.

### 3.1 `error!` / `warn!` / `info!` / `debug!` / `trace!` — rustdoc only
- Content: `src/macros.rs:149-162`, `188-201`, `227-249`, `275-289`, `315-333` (the `///`-block above each of the five macros, showing default form, `target:` form, and `logger:` form examples)
- Cost: 706 tokens (helper: `count-tokens.py src/macros.rs:149-162 src/macros.rs:188-201 src/macros.rs:227-249 src/macros.rs:275-289 src/macros.rs:315-333`)
- Notes: covers the macros that 99% of users invoke. Bodies are repetitive arm matches and not needed for use; they're below-the-fold.

### 3.2 `log!` macro rustdoc (the canonical generic form)
- Content: `src/macros.rs:11-72` (the long doc block before `macro_rules! log`, covering plain form, `target:` form, and the `logger:` form with caveats about global level still applying)
- Cost: 429 tokens (helper: `count-tokens.py src/macros.rs:11-72`)
- Notes: explains how to invoke `log!` and the semantics of all three named args.

### 3.3 `log_enabled!` macro rustdoc
- Content: `src/macros.rs:359-389` (rustdoc above `macro_rules! log_enabled`)
- Cost: 265 tokens (helper: `count-tokens.py src/macros.rs:359-389`)
- Notes: the recommended guard for "expensive computation only when the level is enabled" — a common pattern.

### 3.4 `kv` module overview — what structured logging is and the capture-modifier table
- Content: `src/kv/mod.rs:1-60` (module-level doc through the bullet list of `:?`, `:debug`, `:%`, `:display`, `:err`, `:sval`, `:serde` modifiers)
- Cost: 541 tokens (helper: `count-tokens.py src/kv/mod.rs:1-60`)
- Notes: the modifier table is the single most-asked-about syntax in the crate; without it, an agent has no way to know `:err`, `:%`, `:serde` etc. exist.

### 3.5 `Record` struct definition + accessor list
- Content: `src/lib.rs:841-850` (struct fields) plus `869-961` (impl block: `builder`, `args`, `metadata`, `level`, `target`, `module_path`, `module_path_static`, `file`, `file_static`, `line`, `key_values`, `to_builder`)
- Cost: 660 tokens (helper: `count-tokens.py src/lib.rs:841-850 src/lib.rs:869-961`)
- Notes: a logger implementor needs every accessor on `Record` to format output. The `key_values` accessor and `to_builder` are `cfg(feature = "kv")`-gated.

### 3.6 `Metadata` struct definition + accessors
- Content: `src/lib.rs:1157-1181` (struct fields and `level()`/`target()` impl)
- Cost: 141 tokens (helper: `count-tokens.py src/lib.rs:1157-1181`)
- Notes: small but load-bearing — `Log::enabled` takes `&Metadata`.

### 4.1 Crate doc — "Implementing a Logger" walkthrough
- Content: `src/lib.rs:172-256` (the `# Implementing a Logger` section through `set_logger` / `set_boxed_logger` examples)
- Cost: 775 tokens (helper: `count-tokens.py src/lib.rs:172-256`)
- Notes: end-to-end recipe with code; lets the agent answer "how do I write a logger" without follow-up reads.

### 4.2 Crate doc — compile-time max-level filters and `STATIC_MAX_LEVEL`
- Content: `src/lib.rs:257-303` (the `# Compile time filters` and `# Crate Feature Flags` sections)
- Cost: 432 tokens (helper: `count-tokens.py src/lib.rs:257-303`)
- Notes: explains why the `max_level_*`/`release_max_level_*` features exist and how they interact (debug vs release).

### 4.3 `STATIC_MAX_LEVEL` const definition
- Content: `src/lib.rs:1602-1624` (the `match cfg!(debug_assertions)` table mapping features to `LevelFilter` variants)
- Cost: 305 tokens (helper: `count-tokens.py src/lib.rs:1602-1624`)
- Notes: the canonical place to see which feature wins in which build profile.

### 4.4 `set_logger` / `set_boxed_logger` full rustdoc + body
- Content: `src/lib.rs:1406-1481` (both functions with their full doc, including the once-only contract, atomicity notes, and the working `MyLogger` example)
- Cost: 527 tokens (helper: `count-tokens.py src/lib.rs:1406-1481`)
- Notes: subsumes the signature-only batch 2.5 entries with full rationale; reach this when the budget allows it.

### 4.5 `set_max_level` / `max_level` full rustdoc
- Content: `src/lib.rs:1344-1404`
- Cost: 622 tokens (helper: `count-tokens.py src/lib.rs:1344-1404`)
- Notes: includes the discussion of `Trace` being numerically maximal and the `transmute` in `max_level` (semantic surprise worth showing).

### 4.6 Available logger implementations (third-party catalog)
- Content: `src/lib.rs:128-170` (the bulleted catalog: `env_logger`, `simple_logger`, `log4rs`, `fern`, `syslog`, `console_log`, etc., grouped by category)
- Cost: 329 tokens (helper: `count-tokens.py src/lib.rs:128-170`)
- Notes: useful when the agent is helping the user *pick* a logger — without this, the agent will guess crate names.

### 4.7 `kv` module — adding key-values, accessing via `Source` and `VisitSource`
- Content: `src/kv/mod.rs:60-115` (the "Adding key-values" and "Working with key-values on log records" subsections, with `Source::get` and `VisitSource` examples)
- Cost: 480 tokens (helper: `count-tokens.py src/kv/mod.rs:60-115`)
- Notes: pairs with 3.4; together they cover producer and consumer sides of the kv API.

### 4.8 `Level` method bodies (`max`, `to_level_filter`, `as_str`, `iter`, `increment_severity`, `decrement_severity`)
- Content: `src/lib.rs:534-624` (the `impl Level` block with rustdoc and worked examples)
- Cost: 695 tokens (helper: `count-tokens.py src/lib.rs:534-624`)
- Notes: anchors severity-ordering questions. The parallel `impl LevelFilter` block at `src/lib.rs:684-778` is structurally isomorphic (substitute `Trace` ↔ `Trace`, add `Off`); the agent can derive it from this batch plus the enum definition in 2.4. It is one `Read` away if needed.

### 4.9 `RecordBuilder` impl
- Content: `src/lib.rs:1003-1111` (the builder struct, `new`, `args`, `metadata`, `level`, `target`, `module_path`/`module_path_static`, `file`/`file_static`, `line`, `key_values`, `build`)
- Cost: 892 tokens (helper: `count-tokens.py src/lib.rs:1003-1111`)
- Notes: anyone constructing a `Record` (mock loggers, shim loggers, tests) needs this.

### 4.10 `MetadataBuilder` impl
- Content: `src/lib.rs:1199-1240` (struct + new/level/target/build)
- Cost: 263 tokens (helper: `count-tokens.py src/lib.rs:1199-1240`)
- Notes: small, completes the builder picture.

### 4.11 `__private_api` module (what the macros expand to)
- Content: `src/__private_api.rs` (entire file, 151 lines)
- Cost: 1050 tokens (helper: `count-tokens.py src/__private_api.rs`)
- Notes: contains `GlobalLogger`, the `log_impl` body, the `KVs` sealed trait, and (under `cfg(feature = "kv")`) the `capture_to_value`/`capture_debug`/`capture_display`/`capture_error`/`capture_sval`/`capture_serde` shims. Catastrophic-omission risk for "why does my macro expansion call this private symbol" debugging.

### 5.1 `kv::Value` — type doc, data model, and constructor methods
- Content: `src/kv/value.rs:33-220` (the `Value` rustdoc through the `from_*` family: `from_any`, `from_debug`, `from_display`, `from_serde`, `from_sval`, `from_dyn_debug`, `from_dyn_display`, `from_dyn_error`, `null`, plus `visit`)
- Cost: 1556 tokens (helper: `count-tokens.py src/kv/value.rs:33-220`)
- Notes: the data-model bullet list (Null/Strings/Booleans/Integers/Floats/Errors/serde/sval) is the only place the agent learns *what kinds* of values are representable.

### 5.2 `kv::Source` trait + visitor surface
- Content: `src/kv/source.rs:1-88` (module doc, `Source` trait with `visit`/`get`/`count`) plus `src/kv/source.rs:234-238` (the `VisitSource` trait)
- Cost: 713 tokens (helper: `count-tokens.py src/kv/source.rs:1-88 src/kv/source.rs:234-238`)
- Notes: lets a consumer iterate or look up structured values on a `Record`.

### 5.3 `kv::Key` and `ToKey` surface
- Content: `src/kv/key.rs:1-91` (the `ToKey` trait, the `Key<'k>` struct, `from_str`, `as_str`, `to_borrowed_str`, `Display`/`AsRef<str>`/`Borrow<str>`/`From<&str>` impls)
- Cost: 640 tokens (helper: `count-tokens.py src/kv/key.rs:1-91`)
- Notes: small enough to take whole; keys are referenced everywhere in the kv API.

### 5.4 `kv::VisitValue` trait (primitive-typed visitor)
- Content: `src/kv/value.rs:450-536` (rustdoc + every `visit_*` method)
- Cost: 602 tokens (helper: `count-tokens.py src/kv/value.rs:450-536`)
- Notes: the lightweight alternative to wiring up serde/sval; needed for "what types can I unwrap from a `Value`" questions.

### 5.5 `kv::Error` definition
- Content: `src/kv/error.rs:1-67` (struct, `Inner` enum, `msg`, `Display` impl; the `kv_std`-gated `boxed`/`error::Error`/`From<io::Error>` impls follow)
- Cost: 455 tokens (helper: `count-tokens.py src/kv/error.rs:1-67`)
- Notes: small, completes the kv error story.

### 5.6 `Cargo.toml` dependencies block
- Content: `Cargo.toml:56-77` (`[dependencies]`, `[dev-dependencies]`, and the `proc-macro2` minimal-version pin with its explanatory comment)
- Cost: 321 tokens (helper: `count-tokens.py Cargo.toml:56-77`)
- Notes: shows that `serde_core`, `sval`, `sval_ref`, `value-bag` are the only optional runtime deps and that everything is `default-features = false`.

### 5.7 README install / structured-logging snippets
- Content: `README.md:25-134` (the `## Usage` section through the structured-logging example)
- Cost: 1090 tokens (helper: `count-tokens.py README.md:25-134`)
- Notes: redundant with 1.4 + 2.2 in spirit but adds the "In libraries" / "In executables" framing and the long bulleted catalog of impls (parallels 4.6 from the README's perspective).

### 5.8 `set_logger_inner` / `set_logger_racy` bodies (initialization state machine)
- Content: `src/lib.rs:1482-1542` (the `STATE.compare_exchange` machine, `INITIALIZING` spin-wait, and the `racy` no-atomics variant with its UB note)
- Cost: 431 tokens (helper: `count-tokens.py src/lib.rs:1482-1542`)
- Notes: catastrophic-omission risk for anyone debugging "why did my second `set_logger` return `SetLoggerError`" or working on the unsafe-init paths.

### 5.9 `log!` macro arms (`__log!` expansion shapes)
- Content: `src/macros.rs:73-147` (the four-arm `log!` and the two-arm `__log!` that does the actual `STATIC_MAX_LEVEL`/`max_level()` gate and calls `__private_api::log`)
- Cost: 730 tokens (helper: `count-tokens.py src/macros.rs:73-147`)
- Notes: the exact gate condition `lvl <= STATIC_MAX_LEVEL && lvl <= max_level()` lives here — needed when reasoning about "is my message dropped at compile time or runtime".

### 5.10 Crate-level `kv` example block
- Content: `src/lib.rs:91-127` (the `# Structured logging` example using `info!(target: "yak_events", yak:serde; ...)`, `info!(razor; ...)`, `warn!(e:err; ...)`)
- Cost: 307 tokens (helper: `count-tokens.py src/lib.rs:91-127`)
- Notes: concrete kv usage on real-looking macros, which complements the abstract modifier list in 3.4.

### 5.11 Integration test (canonical end-to-end logger usage)
- Content: `tests/integration.rs` (full file)
- Cost: 785 tokens (helper: `count-tokens.py tests/integration.rs`)
- Notes: the cleanest concrete example of (a) implementing `Log`, (b) calling `set_max_level` to toggle filters, (c) the `logger:` per-call argument, and (d) `Location::caller()` line tracking. Tiny relative to its semantic payoff for "show me how this is actually used".

### 5.12 `test_max_level_features/` helper crate
- Content: `test_max_level_features/Cargo.toml` and `test_max_level_features/main.rs` (whole files)
- Cost: 544 tokens (helper: `count-tokens.py test_max_level_features/Cargo.toml test_max_level_features/main.rs`)
- Notes: shows the canonical end-to-end check that compile-time filter features actually drop trace/debug calls. Useful context for anyone touching the feature-gate logic.

## Below-the-fold

- `src/macros.rs:439-579` (~950 tokens, the `__log_key` / `__log_value` / `__log_value_*` plumbing): maps `:capture` modifiers to `__private_api::capture_*` calls. Functionally implied by 4.11 (`__private_api`'s `capture_*` functions) and the modifier table in 3.4; an agent that wants the exact macro arms is one `Read` away.
- `src/kv/mod.rs:116-244` (~1090 tokens, the second half of the kv module doc): "values have methods for conversions to common types" through the closing Debug/Display example. Rounds out the structured-logging chapter, but the modifier table (3.4), producer/consumer flow (4.7), and `Value`/`VisitValue` surfaces (5.1, 5.4) already cover the API.
- `impl LevelFilter` body at `src/lib.rs:684-778` (~790 tokens): structurally isomorphic to the `impl Level` block in 4.8 (substitute `Off` as the additional discriminant; remove `to_level_filter`, add `to_level`). One `Read` away.
- `src/serde.rs` (~2.85k tokens, full file): hand-written `Serialize`/`Deserialize` for `Level`/`LevelFilter` under the `serde_core` feature. Important if the user is debugging serde round-trips for log levels, but otherwise pure plumbing — the data model is already conveyed by the `Level`/`LevelFilter` definitions in batches 2.4 and 4.8, plus the feature gating in 4.2.
- `src/kv/value.rs:601-1396` (~7.7k tokens, the entire `inner` module): two parallel implementations of `Inner<'v>` — a `value_bag`-backed one and a no-deps fallback — with `From` impls, conversion methods, and serde/sval glue. Effectively private (`pub(in crate::kv)`); users interact only via `Value`/`VisitValue`. Catastrophic-omission risk is low because nothing here is reachable from outside the module.
- `src/kv/source.rs:130-515` (~3k tokens): blanket `Source` impls for `&T`, `(K, V)`, `[S]`, `[S; N]`, `Option<S>`; `cfg(feature = "std")` impls for `Box<S>`/`Arc<S>`/`Rc<S>`/`Vec<S>`/`HashMap<K, V, S>`/`BTreeMap<K, V>`; visitor blanket impls for `fmt::DebugMap`/`DebugList`/`DebugSet`/`DebugTuple`. The trait surface in 5.2 implies these exist; specific impls are one `Read` away.
- `src/kv/key.rs:92-164` (~390 tokens): `kv_std`/`kv_sval`/`kv_serde` impls for `Key` (`String`/`Cow`/`sval::Value`/`serde::Serialize`) plus tiny tests. Implied by the feature names and the trait set in 5.3.
- `src/lib.rs:401-466` (~430 tokens): `LOGGER`/`STATE` statics, the no-atomic `Cell`-backed fallback `AtomicUsize`, the `LOG_LEVEL_NAMES` table, and the `SET_LOGGER_ERROR`/`LEVEL_PARSE_ERROR` strings. Implementation detail unless the agent is porting to a no-atomic target.
- `src/lib.rs:1280-1342` (~480 tokens): `NopLogger` and the `&T` / `Box<T>` / `Arc<T>` blanket impls of `Log`. Useful for "why does my `Box<dyn Log>` work" but trivially derivable.
- `src/lib.rs:780-794` (~120 tokens): `MaybeStaticStr<'a>` enum used internally by `Record` for `module_path`/`file`. Pure internal.
- `src/lib.rs:356-400` (~370 tokens): the giant `cfg(any(...))` walls that emit `compile_error!` when conflicting `max_level_*` features are set. The semantics ("multiple max_level_* features set is an error") are conveyed by 4.2; the wall itself is not load-bearing reading.
- `src/lib.rs:1626-2010` (~3.7k tokens): the in-file `#[cfg(test)] mod tests`. The integration test in 5.11 already shows realistic usage; these are mostly `Level::iter`/`as_str`/`from_str` round-trip checks.
- `tests/macros.rs` (~4.4k tokens): exhaustive macro-form coverage tests (`no_args`, `anonymous_args`, `named_args`, etc.). Good for "what input shapes are guaranteed to compile" but a one-liner skim of the file names tells the agent that and the bodies are mechanical.
- `benches/value.rs` (155 tokens): a four-bench microbenchmark for `Value::from`/`from_debug`. Trivial; the file's existence is shown by listing 1.1.
- `CHANGELOG.md` (~6.2k tokens): version history. Almost never load-bearing for a coding agent on an open-ended task; the version is in `Cargo.toml`.
- `LICENSE-APACHE`, `LICENSE-MIT`, `triagebot.toml`, `.github/workflows/main.yml`, `.gitignore`: standard governance/CI files. The CI yaml does encode the supported feature combinations, but those are also enumerated in `Cargo.toml`.
- `rfcs/0296-structured-logging.md` (~13.6k tokens): the structured-logging RFC. Excluded from the published crate via `Cargo.toml`'s `exclude` list, and the `kv` module + its doc-comments already supersede it. Worth knowing it exists (visible from the listing in 1.1) but not worth ranked tokens.

The fixture totals roughly 71k tokens of raw source. The 20k cap is reached around batch 5.12, with kv internals, the `serde` module, and the RFC document making up the bulk of what stays out.
