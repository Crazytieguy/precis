# log — North Star

Revision pin: `43f2c283`

`log` is a small Rust crate (~5k lines under `src/`, plus a 1.6k-line RFC and a 423-line CHANGELOG). It's a logging *facade*: applications call macros (`error!`/`warn!`/`info!`/`debug!`/`trace!`/`log!`/`log_enabled!`); a single global `Log` trait object — installed at startup with `set_logger` / `set_boxed_logger` — receives `Record`s and decides what to do with them. An optional `kv` feature adds structured key-value pairs to records; a `serde` feature adds (de)serialization for `Level`/`LevelFilter`. Compile-time `max_level_*` Cargo features can statically remove logs below a chosen severity.

Because the crate is so doc-heavy (the public API is mostly thin getters/setters around tiny structs), most batches below pair a doc-comment with the signature it documents. Bodies are split out and ranked lower.

## Batches

### 1.1 Whole-repo file tree
- Content: rendered listing of every file in the fixture (folders plus files at every depth: `.github/workflows/main.yml`, `.gitignore`, `CHANGELOG.md`, `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`, `benches/value.rs`, `rfcs/0296-structured-logging.md`, `src/__private_api.rs`, `src/lib.rs`, `src/macros.rs`, `src/serde.rs`, `src/kv/error.rs`, `src/kv/key.rs`, `src/kv/mod.rs`, `src/kv/source.rs`, `src/kv/value.rs`, `test_max_level_features/Cargo.toml`, `test_max_level_features/main.rs`, `tests/integration.rs`, `tests/macros.rs`, `triagebot.toml`).
- Cost: 127 tokens (helper: `printf '<lines>' | count-tokens.py --stdin`)
- Notes: this is the highest-leverage batch in the document — the entire crate is small enough that every file's existence fits in ~130 tokens, eliminating the catastrophic-omission failure mode for "is there a file for X?" questions. Anchors all later batches' file references.

### 1.2 Crate-doc opening (what `log` is)
- Content: `src/lib.rs:11-30` — the "lightweight logging facade" paragraph plus the "log request consists of a target, level, and body" definition.
- Cost: 224 tokens
- Notes: zero-follow-up answer to "what does this crate do?" Establishes the facade pattern, the role of the `target`, and that there's a noop fallback.

### 1.3 README usage example (library + executable patterns)
- Content: `README.md:1-56` — tagline, MSRV (`1.68.0`), the `[dependencies] log = "0.4"` snippet, the `shave_the_yak` example showing `trace!`/`info!`/`warn!` in a library, and the lead-in to the executable-side discussion.
- Cost: 411 tokens
- Notes: this is the first batch where the agent sees actual call-site syntax. Cut here (rather than at line 134) because the long enumerated list of third-party logger implementations dilutes value-per-token.

### 1.4 Cargo features matrix
- Content: `Cargo.toml:22-55` — every `[features]` entry plus the `[dependencies]`, including the six `max_level_*`, six `release_max_level_*`, `std`, `kv`, `kv_sval`, `kv_std`, `kv_serde`, the `serde = ["serde_core"]` alias, and the deprecated `kv_unstable*` aliases.
- Cost: 275 tokens
- Notes: feature flags govern almost every `#[cfg]` in the crate. Without this, the agent will be lost the moment it sees a `#[cfg(feature = "kv_std")]` gate.

### 1.5 Top-level `pub` item inventory in `src/lib.rs`
- Content: `src/lib.rs:405-411` (the three `mod`/`pub mod` declarations) plus a partial slice with elision markers showing the public-item line for each top-level `pub`: lines 410, 475, 636, 842, 1003, 1158, 1200, 1249, 1351, 1375, 1396, 1420, 1478, 1529, 1549, 1566, 1581, 1600, 1611. Render as a sequence of single-line slices interleaved with `…` markers, e.g. `pub mod kv;` / `…` / `pub enum Level {` / `…` / `pub enum LevelFilter {` / `…` / `pub struct Record<'a> {` / etc.
- Cost: 215 tokens (helper: `count-tokens.py src/lib.rs:405-411 src/lib.rs:410 src/lib.rs:475 src/lib.rs:636 src/lib.rs:842 src/lib.rs:1003 src/lib.rs:1158 src/lib.rs:1200 src/lib.rs:1249 src/lib.rs:1351 src/lib.rs:1375 src/lib.rs:1396 src/lib.rs:1420 src/lib.rs:1478 src/lib.rs:1529 src/lib.rs:1549 src/lib.rs:1566 src/lib.rs:1581 src/lib.rs:1600 src/lib.rs:1611`)
- Notes: this is the public-API skeleton for the entire crate. Anchored to real lines so the agent can `Read` to expand any one of them. The `…` markers tell the agent there's hidden code between the listed items.

### 1.6 Five level-macro names + invocation forms
- Content: `src/macros.rs:11-15` (`/// The standard logging macro.` doc opening for `log!`) plus the five single-line `macro_rules! error|warn|info|debug|trace` declarations from lines 165, 204, 252, 292, 336 with `…` between them.
- Cost: 54 tokens (helper: `count-tokens.py src/macros.rs:11-15 src/macros.rs:165 src/macros.rs:204 src/macros.rs:252 src/macros.rs:292 src/macros.rs:336`)
- Notes: the agent sees that `log!`, `error!`, `warn!`, `info!`, `debug!`, `trace!` exist as `#[macro_export]` items. Together with 1.5 this completes the public-name surface area.

## Batches — major group 2

### 2.1 `Log` trait declaration (the core extension point)
- Content: `src/lib.rs:1248-1280` — the trait doc plus all three required methods (`enabled`, `log`, `flush`) with their per-method doc comments.
- Cost: 278 tokens
- Notes: anyone implementing a logger backend must read this. Single most important code block in the crate after the level enums.

### 2.2 `Level` enum declaration
- Content: `src/lib.rs:467-499` — the `pub enum Level` block including doc comment and the five variants (`Error = 1` through `Trace`) with their per-variant docs.
- Cost: 277 tokens
- Notes: the discriminant comment ("These … line up with the discriminants for LevelFilter below") is load-bearing — explains the parallel encodings used everywhere else.

### 2.3 `LevelFilter` enum declaration + `Off` story
- Content: `src/lib.rs:626-649` — `pub enum LevelFilter` with all six variants (`Off`, `Error`, `Warn`, `Info`, `Debug`, `Trace`) and their docs.
- Cost: 206 tokens
- Predecessor: 2.2  (the discriminant relationship is documented in `Level`).

### 2.4 `Record` struct + field list
- Content: `src/lib.rs:842-867` — the `pub struct Record<'a>` declaration with all six fields (`metadata`, `args`, `module_path`, `file`, `line`, `key_values`) plus the `KeyValues<'a>` wrapper.
- Cost: 208 tokens
- Notes: tells the agent what data flows from the macros into a `Log::log` call. The `#[cfg(feature = "kv")] key_values` line is also where the agent first sees that kv is integrated into `Record`.

### 2.5 `Metadata` struct + accessors
- Content: `src/lib.rs:1157-1180` — `pub struct Metadata<'a> { level: Level, target: &'a str }` plus the `impl Metadata { builder, level, target }` accessors.
- Cost: 140 tokens (helper: `count-tokens.py src/lib.rs:1157-1180`)
- Notes: implementers of `Log::enabled` consume this; almost every example logger pattern-matches on it.

### 2.6 `set_logger` / `set_boxed_logger` signatures + the embedded `MyLogger` example
- Content: `src/lib.rs:1419-1480` — the `set_boxed_logger` signature (`#[cfg(all(feature = "std", target_has_atomic = "ptr"))] pub fn set_boxed_logger(logger: Box<dyn Log>) -> Result<(), SetLoggerError>`), then the `set_logger` doc + signature including the runnable `MyLogger`/`set_logger`/`set_max_level` example. The longer leading paragraph for `set_boxed_logger` (`:1406-1418`) is below-the-fold since `set_logger`'s doc covers the same single-installation invariant.
- Cost: 436 tokens (helper: `count-tokens.py src/lib.rs:1419-1480`)
- Notes: zero-follow-up answer to "how do I install a logger?" The example is the most-cited snippet in user code.

### 2.7 `set_max_level` / `max_level` signatures + docs
- Content: `src/lib.rs:1344-1404` — `set_max_level` doc + body, the racy variant doc + body, and `max_level` with its safety transmute comment.
- Cost: 622 tokens (≤ 2× the 436-token batch above ✓)
- Notes: pairs with 2.6 — together they're the runtime configuration story for any logger.

### 2.8 `STATIC_MAX_LEVEL` and the compile-time filter feature matrix doc
- Content: `src/lib.rs:255-303` (the `# Compile time filters` and `# Crate Feature Flags` sections of the crate doc) plus `src/lib.rs:1602-1624` (the `STATIC_MAX_LEVEL` const definition with its match-on-`debug_assertions`).
- Cost: 305 + 436 = 741 tokens — split into two batches:
  - **2.8a** Crate-doc on compile-time filters: `src/lib.rs:255-303` — 436 tokens.
  - **2.8b** `STATIC_MAX_LEVEL` const definition: `src/lib.rs:1602-1624` — 305 tokens.
- Notes: 2.8a is the user-facing "how do I disable trace logs at compile time?" answer; 2.8b is the actual implementation that determines whether a `log!` invocation compiles to a no-op. Both are gated by the same Cargo features documented in 1.4.

### 2.9 `kv` module index & purpose
- Content: `src/kv/mod.rs:1-30` (module-doc opening: "Structured logging" plus the "Add the kv feature" Cargo snippet plus the first usage example) and `src/kv/mod.rs:246-266` (the `pub use self::error::Error;`-style re-exports, the `mod` declarations, and the `kv_unstable` re-exports).
- Cost: 247 + 121 = 368 tokens — split:
  - **2.9a** `src/kv/mod.rs:1-30` — kv-module purpose paragraph (247 tokens).
  - **2.9b** `src/kv/mod.rs:246-266` — kv re-export inventory (121 tokens).

### 2.10 `error!` macro doc + invocation forms
- Content: `src/macros.rs:149-186` — the doc-comment with three example invocations (`error!("…")`, `error!(target: …)`, `error!(logger: …)`) and the four `macro_rules!` arms.
- Cost: 465 tokens
- Notes: pattern shared by `warn!`/`info!`/`debug!`/`trace!`. Reading just this one is enough to understand the family. The arms also show the `logger:` and `target:` argument forms — neither obvious from the README.

## Batches — major group 3

### 3.1 `Level` ↔ `LevelFilter` cross-comparison and `FromStr`/`Display`
- Content: `src/lib.rs:501-532` — the `PartialEq<LevelFilter>` and `PartialOrd<LevelFilter>` impls on `Level`, and the `FromStr` + `Display` impls.
- Cost: 228 tokens (helper: `count-tokens.py src/lib.rs:501-532`)
- Notes: lets the agent answer "can I compare a `Level` to a `LevelFilter`?" without follow-up.

### 3.2 `Level::max`, `as_str`, `to_level_filter`, `iter`, `increment_severity`, `decrement_severity`
- Content: `src/lib.rs:534-624` — the entire `impl Level { … }` block including the from_usize, max, to_level_filter, as_str, iter, and increment/decrement methods with their doc-test examples.
- Cost: 619 tokens (≤ 2× the 465-token batch in 2.10 ✓)
- Predecessor: 2.2

### 3.3 `LevelFilter::max`, `to_level`, `as_str`, `iter`, `increment_severity`, `decrement_severity`
- Content: `src/lib.rs:684-778` — the parallel `impl LevelFilter { … }` block.
- Cost: 789 tokens (≤ 2× the 619-token batch above ✓)
- Predecessor: 2.3
- Notes: parallel structure to 3.2 — they have identical method names but different `Off`-handling.

### 3.4 `Record` accessors (`args`, `level`, `target`, `module_path`, `file`, `line`, `key_values`)
- Content: `src/lib.rs:869-942` — the `impl<'a> Record<'a>` block with every accessor method including `module_path_static`/`file_static` and `to_builder`.
- Cost: 465 tokens
- Predecessor: 2.4

### 3.5 `RecordBuilder` declaration
- Content: `src/lib.rs:1003-1033` — the struct doc, struct definition, `impl RecordBuilder { new() }` showing the default values for every field including the `KeyValues(&None::<(kv::Key, kv::Value)>)` initialization.
- Cost: 233 tokens
- Predecessor: 2.4

### 3.6 `RecordBuilder` setters (full body)
- Content: `src/lib.rs:1034-1110` — every setter (`args`, `metadata`, `level`, `target`, `module_path`, `module_path_static`, `file`, `file_static`, `line`, `key_values`) plus the `build()` method.
- Cost: 659 tokens (helper: `count-tokens.py src/lib.rs:1034-1110`)
- Predecessor: 3.5
- Notes: setters interleave one-line docs with one-line `pub fn`s; the bodies (`self.record.X = X; self`) are mechanical but only ~3 tokens per body, so a signature-only split would barely save. The full block is small enough to keep intact.

### 3.7 `MetadataBuilder` (declaration + setters + `Default`)
- Content: `src/lib.rs:1199-1245` — `pub struct MetadataBuilder`, the `new()`/`level()`/`target()`/`build()` methods, and the `Default` impl.
- Cost: 286 tokens
- Predecessor: 2.5

### 3.8 `Log` trait blanket impls (`&T`, `Box<T>`, `Arc<T>`)
- Content: `src/lib.rs:1281-1342` — the `NopLogger` definition, `impl Log for &T`, `impl Log for Box<T>` (gated on `std`), `impl Log for Arc<T>` (gated on `std`).
- Cost: 324 tokens
- Predecessor: 2.1
- Notes: explains why `&MyLogger`, `Box<MyLogger>`, and `Arc<MyLogger>` all satisfy `set_logger`'s `&'static dyn Log` bound.

### 3.9 `info!` macro doc (representative of the warn!/debug!/trace! family)
- Content: `src/macros.rs:227-251` — `info!` doc with the three example invocations (`info!("Connected to port {} ...")`, `info!(target: "connection_events", ...)`, `info!(logger: my_logger, ...)`). The other three macros (`warn!`, `debug!`, `trace!`) follow the identical four-arm pattern shown in `error!` (2.10).
- Cost: 172 tokens (helper: `count-tokens.py src/macros.rs:227-251`)
- Predecessor: 2.10
- Notes: the four severity macros are near-identical; one canonical doc is enough. The arm-pattern is in 2.10.

### 3.10 `log!` macro doc + signature (dispatcher every other macro expands into)
- Content: `src/macros.rs:11-72` — the doc with examples for `target:`/`logger:`/no-arg invocation forms. Mark the four `macro_rules! log` arms (`:73-115`) as elided — they compile to `$crate::__log!(logger: ..., target: ..., $lvl, $($arg)+)`, which is shown in 2.10 for `error!`.
- Cost: 429 tokens
- Notes: the four other level macros (3.9) all expand to `$crate::log!(...)`. Doc + invocation form is enough to answer "what does `error!(logger: x, ...)` mean?" without seeing the arms.

### 3.11 `log_enabled!` macro
- Content: `src/macros.rs:359-390` — doc with examples for the `target:`/`logger:` argument variants. The four `macro_rules!` arms (`:391-411`) are below-the-fold; they parallel the `log!` arms.
- Cost: 269 tokens (helper: `count-tokens.py src/macros.rs:359-390`)
- Notes: the "guard expensive log args" pattern. Often shows up in user code.

### 3.12 `kv` data-flow doc (Source/VisitSource walkthrough)
- Content: `src/kv/mod.rs:60-115` — the "Working with key-values on log records" subsection, with the runnable examples for `Source::get` and `VisitSource::visit_pair`.
- Cost: 480 tokens (helper: `count-tokens.py src/kv/mod.rs:60-115`)
- Predecessor: 2.9a
- Notes: shows the consumer-side API of structured logging concretely.

### 3.13 `Source` trait declaration + default `get`/`count`
- Content: `src/kv/source.rs:51-128` — the `pub trait Source` doc + three method signatures (`visit`, `get`, `count`) and the `get_default`/`count_default` helpers that show what the defaults do.
- Cost: 568 tokens
- Predecessor: 2.9a

### 3.14 `VisitSource` trait + standard `fmt::Debug*` impls
- Content: `src/kv/source.rs:234-278` — the `pub trait VisitSource` with `visit_pair`, plus the `&mut T` blanket impl and the four `fmt::DebugMap`/`DebugList`/`DebugSet`/`DebugTuple` impls (which is how `Record`'s `Debug` impl prints kv pairs).
- Cost: 457 tokens
- Predecessor: 3.13

### 3.15 `Key` type + `ToKey` trait
- Content: `src/kv/key.rs:1-91` — `ToKey` trait, the four base `ToKey` impls, the `Key<'k>` struct with `from_str`/`as_str`/`to_borrowed_str`, the `Display`/`AsRef`/`Borrow`/`From` impls.
- Cost: 640 tokens
- Predecessor: 2.9a

### 3.16 `Value` API surface (constructors + visit + data model)
- Content: `src/kv/value.rs:33-220` — the `pub struct Value<'v>` with its data-model documentation (Strings, Booleans, Integers, Floats, Errors, serde, sval), all the `from_*` constructors (`from_any`, `from_debug`, `from_display`, `from_serde`, `from_sval`, `from_dyn_debug`, `from_dyn_display`, `from_dyn_error`, `null`), and the `visit` method.
- Cost: ~1500 tokens — split into two batches to stay coherent and respect the 2× rule:
  - **3.16a** `src/kv/value.rs:33-117` — `Value` doc + data model — 714 tokens, capped at 2× the 789-token batch above ✓.
  - **3.16b** `src/kv/value.rs:118-220` — `Value` constructors and `visit` — 685 tokens.
- Predecessor: 2.9a

### 3.17 `ToValue` trait + primitive coverage list
- Content: `src/kv/value.rs:1-32` (`ToValue` trait + impls) plus `src/kv/value.rs:336-374` (the `impl_to_value_primitive!` invocation listing every covered primitive: `usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64, char, bool` and the `NonZero*` list). Render as one combined batch with `…` between the two slices.
- Cost: 162 + 389 = 551 tokens (helper: `count-tokens.py src/kv/value.rs:1-32 src/kv/value.rs:336-374`)
- Predecessor: 3.16a
- Notes: enumerating the supported primitives once in the snapshot saves the agent from grep-hunting in `value.rs` later.

## Batches — major group 4

### 4.1 `__private_api` module declaration + `GlobalLogger`
- Content: `src/__private_api.rs:1-53` — the warning-comment header, the `Value<'a> = &'a str` non-kv fallback, the `KVs` sealed trait + impls, and the `pub struct GlobalLogger` + its `Log` impl.
- Cost: 336 tokens (helper: `count-tokens.py src/__private_api.rs:1-53`)
- Notes: this is what every macro-expansion ultimately invokes; needed for any "what does the macro actually do at runtime?" question.

### 4.2 `__private_api::log` / `log_impl` / `enabled` / `loc`
- Content: `src/__private_api.rs:55-110` — the `log_impl` generic-collapsing function, the `log<K, L>` trampoline, `enabled`, and `#[track_caller] fn loc`.
- Cost: 346 tokens
- Predecessor: 4.1
- Notes: shows how kv pairs are forwarded into the `Record::builder()` flow and how `Location::caller()` produces file/line.

### 4.3 `__private_api::kv_support` (capture functions)
- Content: `src/__private_api.rs:112-152` — the `capture_to_value`/`capture_debug`/`capture_display`/`capture_error`/`capture_sval`/`capture_serde` functions that the `:?`/`:%`/`:err`/`:sval`/`:serde` syntaxes desugar into.
- Cost: 369 tokens
- Predecessor: 4.1

### 4.4 `set_logger_inner` state machine
- Content: `src/lib.rs:1482-1542` — the `set_logger_inner` `compare_exchange` between UNINITIALIZED/INITIALIZING/INITIALIZED, plus `set_logger_racy`.
- Cost: 431 tokens
- Predecessor: 2.6
- Notes: explains the single-installation invariant. Cited in the documentation but only readable here.

### 4.5 `logger()` accessor
- Content: `src/lib.rs:1578-1596` — the `pub fn logger() -> &'static dyn Log` body and its acquire/release-ordering doc-comment.
- Cost: 174 tokens
- Predecessor: 4.4

### 4.6 `SetLoggerError` and `ParseLevelError` types
- Content: `src/lib.rs:1543-1576` — both error structs, their `Display` impls, and the conditional `error::Error` impls.
- Cost: 254 tokens

### 4.7 `serde` module — `Level` Serialize impl
- Content: `src/serde.rs:1-30` — the `#![cfg(feature = "serde_core")]` gate, imports, and the `impl Serialize for Level` block emitting `serialize_unit_variant("Level", N, "ERROR"|...)`.
- Cost: 227 tokens
- Notes: anchors the agent to the file; `LevelFilter`'s parallel impl is one Read away.

### 4.8 `kv::Error` type
- Content: `src/kv/error.rs:1-46` — the `pub struct Error { inner: Inner }`, the `Inner` enum (`Boxed`, `Msg`, `Value`, `Fmt`), and `Error::msg`/`from_value`/`into_value`.
- Cost: 319 tokens

### 4.9 `kv::Source` blanket impls (single pair, slice, array, Option)
- Content: `src/kv/source.rs:130-232` — the blanket `impl<T> Source for &T`, `impl<K,V> Source for (K, V)`, `impl<S> Source for [S]`, `impl<S, const N: usize> Source for [S; N]`, `impl<S> Source for Option<S>`.
- Cost: 623 tokens
- Predecessor: 3.13
- Notes: explains why `&[("k", v)]` works as a kv source — what users actually pass to the macros.

### 4.10 `VisitValue` trait + per-type visit methods
- Content: `src/kv/value.rs:450-536` — the `pub trait VisitValue` with `visit_any`/`visit_null`/`visit_u64`/`visit_i64`/`visit_u128`/`visit_i128`/`visit_f64`/`visit_bool`/`visit_str`/`visit_borrowed_str`/`visit_char`/`visit_error`/`visit_borrowed_error`.
- Cost: 751 tokens
- Predecessor: 3.16b

### 4.11 `kv::value::inner::Inner` no-deps backing enum (variants only)
- Content: `src/kv/value.rs:737-859` — the `pub enum Inner<'v>` variants (`None`, `Bool(bool)`, `Str(&'v str)`, `Char(char)`, `I64(i64)`, `U64(u64)`, `F64(f64)`, `I128(i128)`, `U128(u128)`, `Debug(&'v dyn fmt::Debug)`, `Display(&'v dyn fmt::Display)`) plus the `From<Tn>` widening conversions for every primitive. The matching `Debug`/`Display`/`to_*`/`visit` blocks at `:861-1049` are below-the-fold.
- Cost: 771 tokens (helper: `count-tokens.py src/kv/value.rs:737-859`)
- Predecessor: 4.10
- Notes: only place the no-deps `Value` data model is materialized; the variant list answers "what types can be in a `Value`?" without a follow-up.

### 4.12 `tests/integration.rs` — canonical end-to-end backend test
- Content: full file (`tests/integration.rs:1-101`) — defines a `Logger` capturing `record.level()` and `record.line()`, then iterates `LevelFilter::Off..=Trace` checking which records pass the filter, and verifies `track_caller` line numbers.
- Cost: 785 tokens
- Notes: most direct example of "implement Log, install it, exercise it" — useful as a starting template for backend-implementation queries.

### 4.13 README — structured-logging snippet
- Content: `README.md:106-134` — the `kv` feature usage example showing `yak:serde`/`razor`/`e:err` capture syntax in actual `info!`/`warn!` calls.
- Cost: 227 tokens
- Predecessor: 2.9a
- Notes: end-to-end usage demonstration of the kv capture syntaxes (`:?`/`:%`/`:err`/`:sval`/`:serde`); the macro-side dispatch is below-the-fold via the deferred `__log_value!` entry.

### 4.14 `tests/macros.rs` — macro arg-form coverage map
- Content: `tests/macros.rs:1-23` (the `all_log_macros!` test helper + minimal `Logger`) plus the first test `:24-53` (`no_args` running every macro shape). Mark the rest of the file (the `anonymous_args`, `named_args`, `inlined_args`, etc., similar tests) as elided.
- Cost: 389 tokens
- Notes: shows every supported macro invocation form once and signals "more arg forms exist" via the `…`.

### 4.15 `test_max_level_features/Cargo.toml`
- Content: full file (`test_max_level_features/Cargo.toml:1-13`) — declares a workspace member `optimized` that depends on the parent `log` crate with `features = ["max_level_debug", "release_max_level_info"]`.
- Cost: 65 tokens
- Notes: only Cargo manifest in the fixture demonstrating the `max_level_*`/`release_max_level_*` features actually being set; the matching `main.rs` is below-the-fold.

### 4.16 `benches/value.rs` — the only benchmark
- Content: full file (`benches/value.rs:1-27`).
- Cost: 155 tokens
- Notes: tells the agent what's benchmarked (kv `Value` construction) and that nothing else is — saves a `find benches/` follow-up.

### 4.17 Recent CHANGELOG entries
- Content: `CHANGELOG.md:1-15` — Unreleased + 0.4.29 sections (notes that `serde_core` replaced `serde` in 0.4.29).
- Cost: 172 tokens

### 4.18 Cargo `[dependencies]` and `[dev-dependencies]`
- Content: `Cargo.toml:56-77` — actual deps (`serde_core` optional, `sval`, `sval_ref`, `value-bag` with `inline-i128`) plus dev-deps (`serde`, `serde_json`, `serde_test`, `sval_derive`) and the `proc-macro2 = "1.0.63"` MSRV pin with its explanatory comment.
- Cost: 321 tokens
- Notes: 1.4 covered feature flags; this covers the actual dependency graph.

### 4.19 `compile_error!` feature-conflict guards
- Content: `src/lib.rs:357-374` (the full first `#[cfg(any(...))] compile_error!("multiple max_level_* features set");` block, listing every conflicting pair) plus `src/lib.rs:393-394` (the parallel block's closing two lines `))] compile_error!("multiple release_max_level_* features set");`). Mark the parallel `release_max_level_*` pair-list (`:376-392`) as elided since it mirrors the first block exactly.
- Cost: 267 tokens (helper: `count-tokens.py src/lib.rs:357-374 src/lib.rs:393-394`)
- Notes: only place that documents the "you can't set two `max_level_*` features at once" rule. Cited from 1.4 and 2.8a.

### 4.20 `triagebot.toml` and `.gitignore`
- Content: full files for both (9 + 26 bytes).
- Cost: ~10 tokens
- Notes: included for completeness — confirms there are no other configuration knobs at the root.

## Below-the-fold

- **`src/lib.rs:172-231` — crate doc "Implementing a Logger" walkthrough** (~530 tokens): the `# Implementing a Logger` section with the `SimpleLogger` example. Composes `Log` (2.1), `set_logger` (2.6), and `set_max_level` (2.7); each of those is shown with its own embedded usage example, so this combined walkthrough is redundant within the cap.
- **`src/macros.rs:73-115` — `log!` macro arms** (~355 tokens): the four `macro_rules! log` arms; they expand to the `__log!` invocations whose pattern is shown in the `error!` arms (2.10).
- **`src/lib.rs:1034-1110` (RecordBuilder bodies)**: already in-batch (3.6) since splitting at 3-token bodies wouldn't save meaningfully.
- **`src/kv/value.rs:355-388` — `impl_value_to_primitive!` accessors** (~340 tokens): the `Value::to_u64`/`to_i64`/`to_u128`/`to_i128`/`to_f64`/`to_char`/`to_bool` accessor block. Covered in spirit by 3.16b's `visit` method (which is the structured alternative). One Read away.
- **`src/kv/value.rs:861-1049` — `Inner` accessor bodies** (~1640 tokens): the `Debug`/`Display` impls, every `to_*` accessor, the `Token` enum, and the `visit` dispatcher for the no-deps `Inner`. Variants are in 4.11; the rest is mechanical pattern-match. The `to_f64` "only `i32::MIN..=u32::MAX`" subtlety is documented in the doc-comment shown in 3.16a.
- **`src/kv/value.rs:601-718` — `value-bag` integration mode** (~900 tokens): the `#[cfg(feature = "value-bag")]` `Inner = ValueBag` re-export and `InnerVisitValue` adapter. Parallel to the no-deps `Inner` in 4.11; only relevant when the agent is debugging across the value-bag boundary.
- **`src/kv/value.rs:1051-1173` — deprecated `kv_unstable` shims** (~980 tokens): `Value::capture_*`, `is`, `downcast_ref`, plus the `as_debug!`/`as_display!`/`as_error!`/`as_serde!`/`as_sval!` macros. All `#[deprecated]`. Agents grepping for old API names will land here via 1.1's listing.
- **`src/kv/source.rs:279-406` — `Source` std-only impls** (~910 tokens): the `Box`/`Arc`/`Rc`/`Vec` smart-pointer impls plus `HashMap`/`BTreeMap` impls. Pattern is identical to 4.9; one Read away.
- **`README.md:60-105` — full list of compatible logger implementations** (~480 tokens): bulleted list of `env_logger`/`log4rs`/`fern`/etc. backend crates. Long, low signal density; once the agent knows the facade pattern from 1.3 and the trait from 2.1, picking a backend is a one-search task.
- **`src/lib.rs:233-310` — crate doc on std/version compatibility** (~530 tokens): the `# Use with std` section (which 2.6's `set_boxed_logger` doc covers in essence) and the `# Version compatibility` 0.3↔0.4 compatibility note (now mostly historical).
- **`.github/workflows/main.yml`** (~410 tokens): CI matrix and `cargo hack test --feature-powerset` invocation. Useful only for CI-related questions.
- **`test_max_level_features/main.rs`** (~480 tokens): the runnable test that exercises 4.15's feature set. The Cargo.toml in 4.15 alone documents the configuration pattern; the test body is mechanical.
- **`Cargo.toml:1-21` — package metadata** (~150 tokens): `name`, `version = "0.4.29"`, `repository`, `categories`, `exclude = ["rfcs/**/*"]`, `rust-version = "1.68.0"`, `edition = "2021"`, `[package.metadata.docs.rs] features = [...]`. Most fields are visible elsewhere (README mentions MSRV; `rfcs/**/*` exclusion is implicit from the rfcs/ dir's existence). Low marginal value over 1.1.
- **`src/lib.rs:1626-2010` — lib.rs in-file `tests` mod** (~3000 tokens): exhaustive `test_levelfilter_from_str`, `test_level_as_str`, `test_metadata_builder`, etc. The pattern is "construct via builder, assert getter returns expected"; once 3.4/3.5/3.7 are shown, tests are one Read away.
- **`src/serde.rs:31-203` — `Level`/`LevelFilter` Deserialize impls** (~840 tokens): the case-insensitive `Deserialize for Level` (`:31-108`) and the parallel `Deserialize for LevelFilter` (`:110-203`). Structurally mirror the Serialize impls in 4.7 — once Serialize is shown the Deserialize boilerplate is one Read away.
- **`src/serde.rs:205-397` — serde tests** (~1700 tokens): every `Level`/`LevelFilter` variant exercised against `serde_test::assert_tokens`/`assert_de_tokens_error`. Token shape is captured by 4.7.
- **`src/kv/value.rs:1175-1396` — value tests** (~1800 tokens): the `tests` mod with `unsigned()`/`signed()`/`float()`/`bool()`/`str()`/`char()` iterators and the `test_to_value_*`/`test_visit_*` battery. Public surface is covered above.
- **`src/kv/source.rs:408-515` — source `tests` mod** (~700 tokens): `Source::count`/`get` checks. Covered in spirit by 3.13.
- **`src/kv/key.rs:112-164` — key sval/serde shims and tests** (~250 tokens): one-line `stream` and `serialize` impls. Mechanical.
- **`rfcs/0296-structured-logging.md` (1651 lines, ~14000 tokens)**: 2019 design RFC for the kv module. Excluded from publication via `[package] exclude = ["rfcs/**/*"]`; far exceeds the 20k cap on its own. The implementation it describes is captured in 2.9/3.13–3.16; the historical motivation is interesting but not load-bearing for typical agent queries. Listed in 1.1 so the agent can fetch it deliberately.
- **`LICENSE-APACHE` (~10800 bytes), `LICENSE-MIT` (~1100 bytes)**: standard texts. The `license = "MIT OR Apache-2.0"` field is in the deferred `Cargo.toml:1-21` package-metadata entry below.
- **`CHANGELOG.md:31-423`**: pre-0.4.29 release history. The recent slice in 4.17 covers what's new; older entries are one Read away.
- **Global state declarations, `src/lib.rs:441-466`** (~260 tokens): the `LOGGER`/`STATE` static-mut globals, `MAX_LOG_LEVEL_FILTER`, the `LOG_LEVEL_NAMES = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"]` array, and the `SET_LOGGER_ERROR`/`LEVEL_PARSE_ERROR` strings. Cited indirectly by 4.4 (`set_logger_inner` reads/writes `STATE` and `LOGGER`) and 4.5 (`logger()` reads `STATE`); knowing they exist is enough.
- **`__log_value!` macro (`src/macros.rs:439-512`)** (~530 tokens): the `__log_value!` macro arms for each capture suffix (`:?`, `:debug`, `:%`, `:display`, `:err`, `:sval`, `:serde`) routing into `__private_api::capture_*`. The user-side syntax is shown in 2.9a's kv-module doc and 4.13's README snippet; the dispatch table itself is mechanical.
- **`AtomicUsize` Cell-fallback shim, `src/lib.rs:412-444`** (~250 tokens): the `cfg(not(target_has_atomic = "ptr"))` `Cell`-backed `AtomicUsize`. Only relevant for embedded targets without atomics.
- **`MaybeStaticStr` enum, `src/lib.rs:780-794`** (~80 tokens): tiny wrapper enabling `Record::file_static`/`module_path_static`. Covered indirectly by 3.4's `*_static` accessors.
- **`#![doc(html_logo_url = ...)]` and `#![warn(missing_docs)]` crate attributes, `src/lib.rs:347-356`** (~110 tokens): docs.rs branding, `html_root_url`, lint setup. No semantic load.
