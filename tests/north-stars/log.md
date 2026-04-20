# log — North Star

Revision pin: `43f2c283`

`log` is a tiny logging *facade* crate. It defines the logging macros (`error!`, `warn!`, `info!`, `debug!`, `trace!`, `log!`, `log_enabled!`), the `Log` trait that downstream loggers implement, the `Level`/`LevelFilter` enums, the `Record`/`Metadata` types, the global-logger state machine (`set_logger`/`set_max_level`/`logger()`/`STATIC_MAX_LEVEL`), and an opt-in `kv` module for structured key-value attributes (with sval/serde feature-gated capture). The dominant axes of plausible queries are: "how do I log X?", "what features control level filtering?", "how do I write a logger backend?", "how does the kv stuff work?" — so the snapshot's job is to make those four directions cheap.

The crate is small enough (~5k lines under `src/`) that nearly every named API can be fit in the 20k cap. The thresholding strategy below tries to put the orientation, public surface, and core trait/struct declarations early; bodies, blanket impls, and macro-expansion internals after; with a long tail of feature-gated helpers / tests / RFC content explicitly deferred to below-the-fold.

## Batches

### 1.1 Top-level repo listing
- Content: rendered listing of every entry directly under fixture root:
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
- Cost: 46 tokens (helper: `printf '.github/\nbenches/\nrfcs/\nsrc/\ntest_max_level_features/\ntests/\n.gitignore\nCHANGELOG.md\nCargo.toml\nLICENSE-APACHE\nLICENSE-MIT\nREADME.md\ntriagebot.toml\n' | ./scripts/count-tokens.py --stdin`)
- Notes: highest-priority orientation. Tells the agent the crate is a small library (single `src/`), has an RFC dir, a CI workflow, and a separate `test_max_level_features/` workspace member that exercises compile-time level features.

### 1.2 `src/` and `src/kv/` listings
- Content: paths under `src/` (`__private_api.rs`, `lib.rs`, `macros.rs`, `serde.rs`, `kv/`) plus `src/kv/` contents (`error.rs`, `key.rs`, `mod.rs`, `source.rs`, `value.rs`).
- Cost: 54 tokens (helper: `printf 'src/__private_api.rs\nsrc/lib.rs\nsrc/macros.rs\nsrc/serde.rs\nsrc/kv/\nsrc/kv/error.rs\nsrc/kv/key.rs\nsrc/kv/mod.rs\nsrc/kv/source.rs\nsrc/kv/value.rs\n' | ./scripts/count-tokens.py --stdin`)
- Predecessor: 1.1
- Notes: completes the file-tree orientation by drilling into `src/`. The crate's *entire* module map is now in context for ~100 tokens combined with 1.1.

### 1.3 Crate identity (name, version, description)
- Content: `tests/fixtures/log/Cargo.toml:4` (the `name = "log"`/`version = "0.4.29"` line) and `tests/fixtures/log/Cargo.toml:10-12` (the `description = "A lightweight logging facade for Rust"`).
- Cost: 28 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/Cargo.toml:4 tests/fixtures/log/Cargo.toml:10-12`)
- Notes: needed before anything else makes sense; cheap and high-leverage. Other Cargo.toml fields (MSRV, repository, exclude) are below-the-fold.

### 1.4 Public log-macro names
- Content: rendered list `error! warn! info! debug! trace! log! log_enabled!` (the `#[macro_export] macro_rules!` names from `src/macros.rs:75, 165, 204, 252, 292, 336, 391`).
- Cost: 15 tokens (helper: `printf 'error!\nwarn!\ninfo!\ndebug!\ntrace!\nlog!\nlog_enabled!\n' | ./scripts/count-tokens.py --stdin`)
- Notes: tells the agent the macro surface exists without bodies. With this alone, an agent can guess `log::info!("…")` works.

### 1.5 Crate-doc lede — facade concept
- Content: `tests/fixtures/log/src/lib.rs:11-19`.
- Cost: 95 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:11-19`)
- Notes: smallest semantic anchor — explains the *facade* concept and the noop fallback. Zero-follow-up answer to "what is this crate?".

### 1.6 Crate-doc — log-request data model
- Content: `tests/fixtures/log/src/lib.rs:20-26`.
- Cost: 81 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:20-26`)
- Predecessor: 1.5
- Notes: defines the target/level/body triple — useful on its own once 1.5 is in context.

### 1.7 `kv` module re-exports
- Content: `tests/fixtures/log/src/kv/mod.rs:246-266` — what `log::kv` exposes (`Source`, `VisitSource`, `Key`, `ToKey`, `Value`, `ToValue`, `VisitValue`, `Error`, plus the deprecated `kv_unstable::Visitor` alias).
- Cost: 121 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/mod.rs:246-266`)
- Predecessor: 1.2
- Notes: catastrophic-omission risk if absent — without this an agent never realises the crate has structured logging. The `kv` mod itself only exists when the feature is on (gated at `src/lib.rs:409-410`).

### 1.8 `LevelFilter` enum (variants + doc)
- Content: `tests/fixtures/log/src/lib.rs:626-649`.
- Cost: 206 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:626-649`)
- Notes: the *filter* type, distinct from `Level` (admits `Off`); fundamental for `set_max_level`.

### 1.9 `Level` enum (variants + doc)
- Content: `tests/fixtures/log/src/lib.rs:467-499`.
- Cost: 277 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:467-499`)
- Notes: defines the verbosity vocabulary used everywhere. The discriminant comment ("These … line up with the discriminants for LevelFilter below") is load-bearing.

### 1.10 `Log` trait declaration
- Content: `tests/fixtures/log/src/lib.rs:1248-1280` — trait with `enabled`/`log`/`flush` methods and the per-method "for implementors" notes.
- Cost: 278 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1248-1280`)
- Notes: the abstraction every backend must implement. The `enabled is *not* necessarily called before log` note is not derivable from the signatures.

### 1.11 Cargo features matrix
- Content: `tests/fixtures/log/Cargo.toml:22-55` — every `[features]` entry: `std`, `kv`, `kv_sval`, `kv_std`, `kv_serde`, `serde = ["serde_core"]` alias, the six `max_level_*` and six `release_max_level_*` toggles, and the deprecated `kv_unstable*` aliases.
- Cost: 275 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/Cargo.toml:22-55`)
- Notes: feature flags govern almost every `#[cfg]` in the crate. Without this, the agent will be lost the moment it sees a `#[cfg(feature = "kv_std")]` gate. Deps are split into 1.13.

### 1.12 `STATIC_MAX_LEVEL` constant
- Content: `tests/fixtures/log/src/lib.rs:1602-1624` — the const item with its doc and the `match cfg!(debug_assertions) { … }` arms.
- Cost: 305 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1602-1624`)
- Predecessor: 1.11
- Notes: the only place the twelve `*max_level_*` features actually take effect at compile time. Without it, the agent can't reason about compile-time level filtering at all.

### 1.13 Cargo dependencies
- Content: `tests/fixtures/log/Cargo.toml:56-77` — runtime deps (`serde_core`, `sval`, `sval_ref`, `value-bag`), dev-deps (`serde`, `serde_json`, `serde_test`, `sval_derive`), and the `proc-macro2` minimal-versions pin with its explanatory comment.
- Cost: 321 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/Cargo.toml:56-77`)
- Predecessor: 1.11
- Notes: the actual external crates wired in. Splitting deps from features keeps every batch ≤ 2× the largest above it and makes the dep list scannable on its own.

Cumulative through 1.x: 2102 tokens. Largest batch so far: 321 (1.13).

### 2.1 Crate-internal `mod`/`use` statements
- Content: `tests/fixtures/log/src/lib.rs:396-410` — `extern crate core as std;`, the `use std::*` block, `mod macros; mod serde; #[cfg(feature = "kv")] pub mod kv;`.
- Cost: 74 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:396-410`)
- Predecessor: 1.2
- Notes: tells the agent the module map of the crate root (and that `kv` is feature-gated; that `serde` is private; that under `no_std` the crate aliases `core as std`).

### 2.2 Five level-macro signatures (line-prefix slices with elision teasing arms)
- Content: rendered as `macros.rs:73-75` for `log!` (16 tokens), then five `error!`/`warn!`/`info!`/`debug!`/`trace!` headers from `:163-165`, `:202-204`, `:249-252`, `:289-292`, `:333-336`, then `:389-391` for `log_enabled!`. Each shown as a few lines (the `#[macro_export]` attribute, the `macro_rules! NAME {` opening) followed by an inline elision marker `…` standing in for the arms.
- Cost: 133 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:73-75 tests/fixtures/log/src/macros.rs:163-165 tests/fixtures/log/src/macros.rs:202-204 tests/fixtures/log/src/macros.rs:249-252 tests/fixtures/log/src/macros.rs:289-292 tests/fixtures/log/src/macros.rs:333-336 tests/fixtures/log/src/macros.rs:389-391`)
- Predecessor: 1.4
- Notes: ranks the macros' shapes early without their bodies. The elision markers tell the agent each macro has additional arms — fetched on demand (or via the later batches that show full bodies).

### 2.3 Compile-time filters + Crate Feature Flags doc
- Content: `tests/fixtures/log/src/lib.rs:257-303`.
- Cost: 432 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:257-303`)
- Predecessor: 1.11
- Notes: the user-facing prose for what each feature toggle does and the `max_level_debug` + `release_max_level_warn` example. Together with 1.11 + 1.12, the compile-time filtering story is fully specified.

### 2.4 README — usage in libraries
- Content: `tests/fixtures/log/README.md:1-56` — tagline, MSRV note, the `[dependencies] log = "0.4"` snippet, and the `shave_the_yak` library-author example.
- Cost: 411 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/README.md:1-56`)
- Notes: first batch where the agent sees actual call-site syntax for the basic macros. Cut at 56 because the long enumerated list of third-party logger backends (which lib.rs has too) dilutes value-per-token.

### 2.5 `set_max_level` family
- Content: `tests/fixtures/log/src/lib.rs:1344-1404` — `set_max_level`, `set_max_level_racy`, and `max_level` with full rustdoc (including the safety note on the racy variant and the `mem::transmute` justification on `max_level`).
- Cost: 622 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1344-1404`)
- Notes: every executable must call `set_max_level`; the rustdoc explains the perf rationale and the unsafe-transmute safety argument.

### 2.6 `set_logger` + `set_boxed_logger` with embedded `MyLogger` example
- Content: `tests/fixtures/log/src/lib.rs:1419-1480` — `set_boxed_logger` signature, then `set_logger` doc + signature with the runnable `MyLogger` / `set_logger` / `set_max_level` example. (`set_boxed_logger`'s longer leading paragraph at `:1406-1418` is in below-the-fold since `set_logger`'s doc covers the same single-installation invariant.)
- Cost: 436 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1419-1480`)
- Notes: zero-follow-up answer to "how do I install a logger?" — the example is the most-cited snippet in user code.

### 2.7 `kv` module synopsis (capturing modifiers)
- Content: `tests/fixtures/log/src/kv/mod.rs:1-60` — opening doc explaining structured logging, the `info!(a = 1; "msg")` syntax, and the `:?`/`:%`/`:err`/`:sval`/`:serde` capturing-modifier table.
- Cost: 541 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/mod.rs:1-60`)
- Predecessor: 1.7
- Notes: gates correct understanding of the kv macro syntax. Without this, every `info!(yak:serde; …)` in the README is opaque.

### 2.8 lib.rs "Implementing a Logger" walkthrough
- Content: `tests/fixtures/log/src/lib.rs:172-232` — the `# Implementing a Logger` section with the `SimpleLogger` example explaining `set_logger`/`set_max_level`/`set_boxed_logger` interaction and the `LevelFilter::Off` default-filter trap.
- Cost: 530 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:172-232`)
- Notes: addresses the most common "I implemented Log but nothing prints" failure. Long but the canonical "how do I write a logger" tutorial; pure docstring with one running example.

### 2.9 `error!` macro full body (representative of the level-macro family)
- Content: `tests/fixtures/log/src/macros.rs:149-186` — doc + all four `macro_rules!` arms.
- Cost: 465 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:149-186`)
- Predecessor: 2.2
- Notes: with this one macro shown in full plus the agent knowing "warn/info/debug/trace are structurally identical at different levels", they have near-complete information to use or modify any of the five. The arms also disclose how `logger:` and `target:` arguments compose — neither obvious from the README.

### 2.10 `Record` struct definition
- Content: `tests/fixtures/log/src/lib.rs:842-867` — the `pub struct Record<'a>` block including all six fields (`metadata`, `args`, `module_path`, `file`, `line`, `key_values`) plus the `KeyValues<'a>` wrapper definition that surfaces feature = "kv" gating.
- Cost: 208 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:842-867`)
- Notes: tells the agent what data flows from the macros into a `Log::log` call. The `#[cfg(feature = "kv")] key_values` line is also where the agent first sees that kv is integrated into `Record`.

### 2.11 `Metadata` struct + accessors
- Content: `tests/fixtures/log/src/lib.rs:1157-1182` — `pub struct Metadata<'a> { level, target }` plus the `impl Metadata { builder, level, target }` accessors.
- Cost: 141 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1157-1182`)
- Predecessor: 1.10
- Notes: `Metadata` is what `Log::enabled` consumes; almost every example logger pattern-matches on it. Small enough not to defer.

Cumulative through 2.x: 6095 tokens. Largest batch so far: 622 (2.5).

### 3.1 `log!` macro full body (the dispatcher)
- Content: `tests/fixtures/log/src/macros.rs:11-115` — doc with `target:`/`logger:`/no-arg invocation forms and all four `macro_rules! log` arms.
- Cost: 784 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:11-115`)
- Notes: shows precisely how the `logger:` and `target:` arguments compose. With 2.9, the macro family is fully specified. Within 2× of 622 (2.5).

### 3.2 Crate-doc — Structured logging walkthrough
- Content: `tests/fixtures/log/src/lib.rs:91-128` — the kv-flavoured yak-shaving example with `info!(target: …, yak:serde; …)`.
- Cost: 308 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:91-128`)
- Predecessor: 2.7
- Notes: the in-crate rustdoc version of the kv usage; shows the kv capture syntaxes inside a runnable doctest gated on `kv_serde`. The deferred README structured-logging snippet covers the same ground in markdown.

### 3.3 Crate-doc — In libraries / In executables / Warning
- Content: `tests/fixtures/log/src/lib.rs:46-90` — yak-shaving library-author example + executable-side notes + `set_logger` once-only warning.
- Cost: 333 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:46-90`)
- Predecessor: 1.5
- Notes: parallel to README §1-56 (2.4) but lives in the rustdoc and includes the explicit "logging system may only be initialized once" warning.

### 3.4 `kv` doc — Working-with-key-values examples (consumer side)
- Content: `tests/fixtures/log/src/kv/mod.rs:60-130` — `record.key_values().get(...)` and `VisitSource::visit_pair` consumer-side examples.
- Cost: 603 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/mod.rs:60-130`)
- Predecessor: 2.7
- Notes: complements 2.7 (producer-side syntax) with the matching consumer-side recipes. Shows the `Source::get` and `VisitSource::visit_pair` patterns concretely.

### 3.5 `Record` accessor methods
- Content: `tests/fixtures/log/src/lib.rs:869-942` — `args()`, `metadata()`, `level()`, `target()`, `module_path()`/`_static()`, `file()`/`_static()`, `line()`, plus the `#[cfg(feature = "kv")] key_values()` accessor.
- Cost: 465 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:869-942`)
- Predecessor: 2.10
- Notes: the surface a `Log` implementor reads. The `*_static` accessors return a longer-lived `&'static str` when the underlying field is a static rather than borrowed string (see the `MaybeStaticStr` enum in below-the-fold).

### 3.6 KV `Source` trait + module doc + default helpers
- Content: `tests/fixtures/log/src/kv/source.rs:1-128` — module doc with the `Printer` visitor example, `Source` trait with `visit`/`get`/`count`, `get_default` and `count_default`.
- Cost: 928 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/source.rs:1-128`)
- Predecessor: 1.7
- Notes: the central trait for collections of key-values. Largest batch so far at 928 — within 2× of 784 (3.1).

### 3.7 KV `Key` (full file minus cfg-gated impls and tests)
- Content: `tests/fixtures/log/src/kv/key.rs:1-91` — `ToKey` trait, `Key<'k>` struct, `from_str`/`as_str`/`to_borrowed_str`, `Display`/`AsRef`/`Borrow`/`From<&str>` impls.
- Cost: 640 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/key.rs:1-91`)
- Predecessor: 1.7
- Notes: small enough to show whole; the public Key API is heavily relied on by both producers and consumers of structured records.

### 3.8 KV `Value` rustdoc + struct definition
- Content: `tests/fixtures/log/src/kv/value.rs:33-122`.
- Cost: 731 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/value.rs:33-122`)
- Predecessor: 1.7
- Notes: explains the type-erasure concept; the data-model bullet list (Null/Strings/Booleans/Integers/Floats/Errors/serde/sval) is hard to derive from code alone. Constructors and `to_*` extractors are split into 3.9 / 4.6.

### 3.9 KV `Value` constructors + `visit`
- Content: `tests/fixtures/log/src/kv/value.rs:123-220` — `from_any`/`from_debug`/`from_display`/`from_serde`/`from_sval`/`from_dyn_debug`/`from_dyn_display`/`from_dyn_error`/`null`/`from_inner`/`visit`.
- Cost: 664 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/value.rs:123-220`)
- Predecessor: 3.8
- Notes: producer-side API for constructing `Value`s manually outside the macro path.

### 3.10 `Level` impl methods
- Content: `tests/fixtures/log/src/lib.rs:534-624` — `from_usize`, `max`, `to_level_filter`, `as_str`, `iter`, `increment_severity`, `decrement_severity` (all with doc-test examples).
- Cost: 695 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:534-624`)
- Predecessor: 1.9
- Notes: the methods are largely paralleled by `LevelFilter` (3.11); `Level` ranks first because it lacks the `Off` discriminant and its methods are referenced more often in user code.

### 3.11 `LevelFilter` impl methods
- Content: `tests/fixtures/log/src/lib.rs:684-778` — `from_usize`, `max`, `to_level`, `as_str`, `iter`, `increment_severity`, `decrement_severity`.
- Cost: 789 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:684-778`)
- Predecessor: 1.8
- Notes: parallels 3.10 but for `LevelFilter` (which has the additional `Off` variant). Within 2× of 928 (3.6).

Cumulative through 3.x: 13035 tokens. Largest batch: 928 (3.6).

### 4.1 `RecordBuilder` definition + setters
- Content: `tests/fixtures/log/src/lib.rs:1003-1110` — struct + every setter (`args`, `metadata`, `level`, `target`, `module_path`/`_static`, `file`/`_static`, `line`, `key_values`) plus `build()`.
- Cost: 891 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1003-1110`)
- Predecessor: 2.10
- Notes: how custom code can construct a `Record` outside the macro path. Setters are tiny so a signature-only split would barely save tokens — kept intact. Within 2× of 928 (3.6).

### 4.2 KV `VisitValue` trait
- Content: `tests/fixtures/log/src/kv/value.rs:450-536`.
- Cost: 751 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/value.rs:450-536`)
- Predecessor: 3.9
- Notes: lightweight alternative to `serde`/`sval` for value backends. Often the right starting point for someone writing a custom kv consumer.

### 4.3 KV `Error` type (file minus the std-only `Error::boxed` block)
- Content: `tests/fixtures/log/src/kv/error.rs:1-66`.
- Cost: 455 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/error.rs:1-66`)
- Predecessor: 1.7
- Notes: the error type returned by every visitor method. Includes the `Inner` enum, `msg`/`from_value`/`into_value`, `Display`, `From<fmt::Error>`. The `#[cfg(feature = "std")]` `Error::boxed` block at 67-94 is below-the-fold.

### 4.4 KV `Source` foundational impls (`(K,V)`, `[S]`, `[S;N]`, `Option<S>`, `&T`)
- Content: `tests/fixtures/log/src/kv/source.rs:130-232`.
- Cost: 623 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/source.rs:130-232`)
- Predecessor: 3.6
- Notes: explains why `&[("a", 1), ("b", 2)]` and `("a", 1)` both work as a `Source` — i.e., what users actually pass to the macros.

### 4.5 `VisitSource` trait + `fmt::Debug*` adapters
- Content: `tests/fixtures/log/src/kv/source.rs:234-277` — `VisitSource` trait + impls for `&mut T`, `fmt::DebugMap`/`DebugList`/`DebugSet`/`DebugTuple`.
- Cost: 457 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/kv/source.rs:234-277`)
- Predecessor: 3.6
- Notes: explains how visitors plug into `fmt` machinery — a common need when implementing a logger that pretty-prints records. Also where the `Record::Debug` impl ends up (via the `KeyValues` wrapper at 2.10).

### 4.6 `log_enabled!` macro full body
- Content: `tests/fixtures/log/src/macros.rs:359-411` — doc with examples and the four `macro_rules!` arms.
- Cost: 527 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:359-411`)
- Notes: critical for the "avoid expensive computation if disabled" usage pattern. Distinct API often missed without a separate batch.

### 4.7 `__log!` internal macro
- Content: `tests/fixtures/log/src/macros.rs:117-147` (`__log!` two arms with kvs and without).
- Cost: 375 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/macros.rs:117-147`)
- Predecessor: 3.1
- Notes: where the actual `STATIC_MAX_LEVEL` / `max_level()` short-circuit lives. Critical for macro-expansion debugging — the public macros all expand into `__log!`. (`__log_enabled!` and `__log_logger!` at `:413-437` are below-the-fold; their pattern is implied by `__log!`.)

### 4.8 `__private_api` log/enabled/loc + `GlobalLogger`
- Content: `tests/fixtures/log/src/__private_api.rs:1-110` — `GlobalLogger` struct, `log_impl`/`log` functions, `enabled`, `loc()`, the `KVs` sealed trait.
- Cost: 681 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/__private_api.rs:1-110`)
- Predecessor: 4.7
- Notes: shows what `__log!` actually invokes; explains the `track_caller` location capture and how `Record` is built before hand-off. Closes the macro-expansion path.

### 4.9 `__private_api::kv_support` capture functions
- Content: `tests/fixtures/log/src/__private_api.rs:112-152` — `capture_to_value`/`capture_debug`/`capture_display`/`capture_error`/`capture_sval`/`capture_serde`.
- Cost: 369 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/__private_api.rs:112-152`)
- Predecessor: 4.8
- Notes: closes the loop between the macro `:?` / `:%` / `:err` / `:sval` / `:serde` modifiers (shown in 2.7) and `kv::Value` constructors.

### 4.10 `set_logger_inner` state machine
- Content: `tests/fixtures/log/src/lib.rs:1482-1542` — the `compare_exchange` between UNINITIALIZED/INITIALIZING/INITIALIZED, plus the `_racy` variant.
- Cost: 431 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1482-1542`)
- Predecessor: 2.6
- Notes: the actual atomics protocol that gates installation. Important for "why didn't my logger get installed" / "why is my logger called twice" questions.

### 4.11 `logger()` accessor + `SetLoggerError` + `ParseLevelError`
- Content: `tests/fixtures/log/src/lib.rs:1544-1596`.
- Cost: 427 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1544-1596`)
- Notes: small but load-bearing — `logger()` is how a backend can call into the currently-installed logger without going through the macros, and the two error structs are returned by `set_logger`/`from_str`.

### 4.12 Static state + state constants
- Content: `tests/fixtures/log/src/lib.rs:445-466` — the `LOGGER` `static mut`, `STATE`, `UNINITIALIZED`/`INITIALIZING`/`INITIALIZED` constants, `MAX_LOG_LEVEL_FILTER`, `LOG_LEVEL_NAMES = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"]`, and the `SET_LOGGER_ERROR`/`LEVEL_PARSE_ERROR` strings.
- Cost: 229 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:445-466`)
- Predecessor: 4.10
- Notes: the global state cited indirectly by 4.10/4.11. Knowing the names and ordering of `LOG_LEVEL_NAMES` is also enough to predict the wire shape used by `Display`/`FromStr`/`serde::Serialize`.

Cumulative through 4.x: 19251 tokens. Largest batch: 928 (3.6).

### 5.1 `Log` blanket impls + `NopLogger`
- Content: `tests/fixtures/log/src/lib.rs:1283-1342` — `NopLogger`, `impl Log for &T`, `impl Log for Box<T>` (gated on `std`), `impl Log for Arc<T>` (gated on `std`).
- Cost: 315 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1283-1342`)
- Predecessor: 1.10
- Notes: the `NopLogger` defines the "default" no-op behavior before `set_logger`; the blanket impls explain why `&MyLogger`, `Box<MyLogger>`, and `Arc<MyLogger>` all satisfy `set_logger`'s `&'static dyn Log` bound.

### 5.2 `MetadataBuilder`
- Content: `tests/fixtures/log/src/lib.rs:1199-1245`.
- Cost: 286 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:1199-1245`)
- Predecessor: 2.11
- Notes: pairs with `RecordBuilder` (4.1).

### 5.3 `tests/integration.rs` (canonical end-to-end backend test)
- Content: full file, 101 lines. Defines a minimal `Logger` capturing `record.level()` and `record.line()`, then iterates `LevelFilter::Off..=Trace` checking which records pass the filter, and verifies `track_caller` line numbers.
- Cost: 785 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/tests/integration.rs`)
- Notes: the simplest end-to-end usage example in the repo — useful as a starting template for backend-implementation queries. Within 2× of 928 (3.6).

### 5.4 `Level` ↔ `LevelFilter` cross-impls (FromStr, Display, ordering)
- Content: `tests/fixtures/log/src/lib.rs:501-532` — `Level`'s `PartialEq<LevelFilter>`, `PartialOrd<LevelFilter>`, `FromStr`, `Display`. (The mirrored `LevelFilter` impls at `:651-682` are below-the-fold; identical structure, cheap one-Read away.)
- Cost: 228 tokens (helper: `./scripts/count-tokens.py tests/fixtures/log/src/lib.rs:501-532`)
- Predecessor: 1.9
- Notes: cross-type comparison is what enables `level <= max_level_filter` checks in the macros' static guard.

Cumulative through 5.x: 20865 tokens. Largest batch: 928 (3.6).

## Below-the-fold

The cap of ≲20k means the items below are not ranked. Each is one or two `Read`/`Grep` away from the cloned-repo state, and the snapshot's structural pointers (folder listings, kv re-exports, item names, macro names) make each location obvious.

- **`rfcs/0296-structured-logging.md` (~14k tokens)** — historical RFC for the kv module. `Cargo.toml`'s `exclude = ["rfcs/**/*"]` keeps it out of the published crate. Current implementation supersedes it; agents asking about *design history* of structured logging would benefit, but tokens are better spent on current code. Listed in 1.1 so the agent can fetch it deliberately.
- **`CHANGELOG.md` (~5800 tokens, 423 lines)** — release notes. The `[Unreleased]` / `[0.4.29]` / `[0.4.28]` entries (the most recent ~440 tokens of `:1-30`) are the only useful slice for typical refactor/bugfix tasks; older entries are one Read away. Promote when the task is "what shipped recently?".
- **`LICENSE-APACHE`, `LICENSE-MIT`** — boilerplate license texts. Their *existence* is shown in 1.1.
- **`triagebot.toml`, `.gitignore`** — one-line config files; not load-bearing for any plausible development task.
- **`.github/workflows/main.yml` (~1219 tokens)** — full CI pipeline (test/check/doc/features/minimalv/msrv/embedded jobs). The `cargo hack test --feature-powerset` invocation and the stable/beta/nightly + macOS/Win + thumbv6m + riscv32imc matrix are the most informative parts; the rest is boilerplate.
- **`Cargo.toml:1-21` package metadata (148 tokens, minus the 28 promoted in 1.3)** — `repository`, `categories`, `keywords`, `exclude = ["rfcs/**/*"]`, `rust-version = "1.68.0"`, `edition = "2021"`, `[package.metadata.docs.rs] features = [...]`. Most fields are visible elsewhere (README mentions MSRV; `rfcs/**/*` exclusion is implicit from the rfcs/ dir's existence).
- **`src/lib.rs:1-10`, `:347-356`, `:305-346`** — the copyright header, the `#![doc(html_logo_url = ...)]` / `#![warn(missing_docs)]` crate attributes, and the long block of intra-doc link definitions (env_logger/log4rs/syslog/...). Pure docs.rs link plumbing and the same logger-ecosystem catalogue that's in README.
- **`src/lib.rs:130-170` — duplicate of the README ecosystem catalogue** (~330 tokens) inside the crate doc.
- **`src/lib.rs:233-256` — "Use with std" doc** (~245 tokens) explaining `set_boxed_logger` rationale; partly redundant with 2.6's embedded example.
- **`src/lib.rs:357-394` — `compile_error!` guards for conflicting `max_level_*` features** (547 tokens) — by-enumeration pair of `#[cfg(any(...))] compile_error!(...)` blocks; their behavior is implied by the feature list (1.11) and `STATIC_MAX_LEVEL` (1.12).
- **`src/lib.rs:412-443` — non-atomic `AtomicUsize` polyfill** (218 tokens) — `Cell`-based fallback for targets without `target_has_atomic = "ptr"`. Promote when discussing `thumbv6` or other no-atomics embedded targets.
- **`src/lib.rs:651-682` — mirrored `LevelFilter` cross-impls** (224 tokens) — `LevelFilter`'s `PartialEq<Level>`, `PartialOrd<Level>`, `FromStr`, `Display`. Identical pattern to 5.4 but for the `Off`-aware variant.
- **`src/lib.rs:780-794` — `MaybeStaticStr` enum** (~104 tokens) — implementation detail explaining the `Static`/`Borrowed` distinction visible in `Record::module_path` vs `module_path_static`. Names alone (3.5's `*_static` accessors) imply the role.
- **`src/lib.rs:943-961` — `Record::to_builder`** (~118 tokens) — kv-feature-only conversion to `RecordBuilder`. Mechanical.
- **`src/lib.rs:1626-2009` — in-file `mod tests`** (~3300 tokens) — internal unit tests for `Level`/`LevelFilter` parsing/ordering, builders, `STATIC_MAX_LEVEL`, `Log` foreign impls. Behavior under test is specified by items already ranked.
- **`src/lib.rs:1406-1418` — `set_boxed_logger` doc preamble** — already covered in spirit by the in-batch `set_logger` doc in 2.6.
- **`src/macros.rs:413-437` — `__log_enabled!` and `__log_logger!`** (~174 tokens) — `__log_enabled!` mirrors the `__log!` short-circuit shown in 4.7; `__log_logger!` decides by-value vs by-reference logger handoff.
- **`src/macros.rs:439-521` — `__log_key!` / `__log_value!` macros** (589 tokens) — what the `key:capture = expr` syntax desugars to. The `cfg(not(feature = "kv"))` arms produce `compile_error!("key value support requires the kv feature of log")`. User-side syntax is in 2.7; capture functions in 4.9. Promote when debugging kv macro expansion.
- **`src/macros.rs:523-579` — `__log_value_sval!` / `__log_value_serde!` / `__log_value_error!`** (~357 tokens) — per-feature dispatchers; mirror of 4.9's capture functions. Mostly mechanical.
- **`src/macros.rs:165-186, 204-225, 252-273, 292-313, 336-357` — `error!`/`warn!`/`info!`/`debug!`/`trace!` arm bodies for the four call shapes** — all structurally identical to `error!` (already shown in 2.9). Each is ~345 tokens. The whole bundle is redundant once 2.9 is in context.
- **`src/serde.rs` (entire file, ~3000 tokens)** — `Serialize`/`Deserialize` impls for `Level`/`LevelFilter` plus their `serde_test` round-trip tests. The `LOG_LEVEL_NAMES` table (in 4.12) and the case-insensitive `FromStr` impls (in 5.4) imply the wire shape (`"ERROR"`/`"WARN"`/...). Promote when the user query touches serde explicitly. Note: `Level::Error` serialises with variant *index* `0`, not the discriminant `1` — a behavioural detail only visible by reading the file.
- **`src/__private_api.rs` lines beyond what's promoted** — the file is fully covered by 4.8 + 4.9.
- **`src/kv/mod.rs:130-244` — extended consumer examples** (~970 tokens) — `IsNumeric` `VisitValue`, serde JSON, Debug formatting. 2.7 + 3.4 already convey the consumer pattern; these are extended worked examples. Promote when the agent needs to write a serde-flavoured kv consumer.
- **`src/kv/key.rs:93-164` — sval/serde/std support sub-modules + tests** (~325 tokens) — small cfg-gated impls that mostly delegate; `From`/`AsRef`/`Borrow` already in 3.7 cover the load-bearing surface.
- **`src/kv/error.rs:67-94` — `Error::boxed` constructor + `error::Error` impl + `From<io::Error>`** (~145 tokens) — std-feature shim. Implicit in 4.3's `Inner::Boxed` variant.
- **`src/kv/source.rs:279-406` — std-feature `Source`/`VisitSource` impls for `Box`/`Arc`/`Rc`/`Vec`/`HashMap`/`BTreeMap`** (~910 tokens) — needed only when answering "can I pass a `HashMap` as my key-values?". The collection list itself is short; the impls are mechanical delegations to 4.4's pattern.
- **`src/kv/source.rs:408-515` — source `tests` mod** (~410 tokens) — `count`, `get`, `hash_map`, `btree_map`, `source_is_object_safe`, `visitor_is_object_safe`. Their *names* already convey the invariants tested.
- **`src/kv/source.rs:460-462` — deprecated `Visitor` alias** — single `pub use VisitSource as Visitor;` line; surfaced by 1.7.
- **`src/kv/value.rs:222-374` — `Display`/`Debug`/`serde`/`sval` impls + `From<&str>` + primitive `From` blanket impls + `to_borrowed_*`** (~1158 tokens) — plumbing that connects 3.8/3.9 to actual primitive conversions. The `impl_to_value_primitive!` invocation at `:349-351` enumerates every covered primitive (`usize, u8..u128, isize, i8..i128, f32, f64, char, bool` plus the parallel `NonZero*`); the `impl_value_to_primitive!` at `:359-374` declares the `to_u64`/`to_i64`/`to_u128`/`to_i128`/`to_f64`/`to_char`/`to_bool` extractors. Promote when the user is writing kv value conversions.
- **`src/kv/value.rs:389-449` — `kv_std` `Cow`/`String`/`Box`/`Arc`/`Rc` `ToValue` impls** (~340 tokens) — std-feature glue.
- **`src/kv/value.rs:538-600` — `VisitValue for &mut T` blanket impl** (~280 tokens) — mechanical delegation.
- **`src/kv/value.rs:601-1049` — `value-bag` and dependency-free `inner` modules** (~5000 tokens) — the `Inner<'v>` enum that actually stores `Value`s, the `value_bag` adapter (`#[cfg(feature = "value-bag")]`), every `From<Tn>` widening conversion, the visit dispatcher. Internal implementation; the public `Value` API in 3.8/3.9/4.2 tells an agent everything needed to *use* `Value`.
- **`src/kv/value.rs:1051-1173` — deprecated `kv_unstable` shims** (~700 tokens) — `Value::capture_*`, `is`, `downcast_ref`, plus the `as_debug!`/`as_display!`/`as_error!`/`as_serde!`/`as_sval!` macros. All `#[deprecated]`; deprecation messages point to current syntax.
- **`src/kv/value.rs:1175-1396` — value `tests` mod** (~1900 tokens) — exhaustive enumeration of value conversions. Behavior already specified by 3.8/3.9/4.2.
- **`tests/macros.rs` (entire file, ~4400 tokens)** — `no_args`, `anonymous_args`, `named_args`, `inlined_args`, `kv_no_args`, `kv_expr_args`, `kv_anonymous_args`, `kv_named_args`, `kv_ident`, `kv_string_keys`, `kv_common_value_types`, `kv_debug`, `kv_display`, `kv_error`, `kv_sval`, `kv_serde`, `regression_issue_494`, `logger_short_lived`, `logger_expr`, `implicit_named_args`, `kv_implicit_named_args`. Exhaustive enumeration of every legal macro call shape. The macro definitions in 2.2 + 2.9 + 3.1 + 4.6 already document each call shape via doc comments.
- **`test_max_level_features/main.rs` + `Cargo.toml` (~544 tokens combined)** — workspace member declaring `features = ["max_level_debug", "release_max_level_info"]` and a runnable `main` exercising those features end-to-end. The only test in the repo covering compile-time release-vs-debug behaviour, but the contract is fully described by 1.11 + 1.12 + 2.3 in the snapshot itself; the runnable proof is one `Read` away. Promote when investigating a bug specifically in the `STATIC_MAX_LEVEL` cascade.
- **`README.md:106-134` — README structured-logging snippet** (227 tokens) — second compact `kv` usage example showing `yak:serde`/`razor`/`e:err` capture syntax. Subsumed by the in-crate version in 3.2 plus the kv synopsis in 2.7.
- **`benches/value.rs`** (155 tokens) — four `#[bench]` functions for `Value::from`/`Value::from_debug` on `u8`, `&str`, and a custom struct. Tiny; tells the agent which `kv::Value` paths are perf-sensitive when promoted.
