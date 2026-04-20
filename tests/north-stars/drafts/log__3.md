# log — North Star

Revision pin: `43f2c283`

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
- Cost: 46 tokens (helper: `echo … | count-tokens.py --stdin`)
- Notes: highest-priority orientation. Tells the agent the crate is a small library (single `src/`), has an RFC dir, a CI workflow, and a separate `test_max_level_features/` workspace member used to exercise compile-time level features.

### 1.2 `src/` listing
- Content:
  ```
  src/lib.rs
  src/macros.rs
  src/__private_api.rs
  src/serde.rs
  src/kv/
  ```
- Cost: 24 tokens (helper: `echo … | count-tokens.py --stdin`)
- Notes: shows the four root files plus the `kv/` submodule — the entire crate's source layout.

### 1.3 Cargo.toml package + features header
- Content: `Cargo.toml:1-21` (package metadata, `rust-version = "1.68.0"`, `edition = "2021"`, `exclude = ["rfcs/**/*"]`, the `docs.rs` features list)
- Cost: 148 tokens (helper: `count-tokens.py Cargo.toml:1-21`)
- Notes: tells the agent crate name, version (`0.4.29`), MSRV, what features docs.rs enables — anchors any feature/version-related question.

### 1.4 Crate-level rustdoc lede
- Content: `src/lib.rs:11-44` — facade overview, "log request consists of target/level/body", noop fallback note, the five macro names listed.
- Cost: 379 tokens (helper: `count-tokens.py src/lib.rs:11-44`)
- Notes: smallest possible answer to "what is this crate?" — every other batch builds on this framing.

### 2.1 The five log-level macros — names only
- Content: line-prefix-truncated rendering of the `macro_rules! NAME {` headers from `src/macros.rs:165`, `205`, `252`, `292`, `336`, with elision markers in between to indicate the bodies and doc comments exist. Effectively five lines like `macro_rules! error {…}` shown together.
- Cost: ~70 tokens for the five headers (helper: `count-tokens.py src/macros.rs:163-165 src/macros.rs:202-204 src/macros.rs:249-251 src/macros.rs:289-291 src/macros.rs:333-335` → 71)
- Notes: the absolute minimum the agent needs to know "these five names exist and are macros." Each macro's full doc/body is ranked separately later. Predecessor-free; can stand alone with 1.4.

### 2.2 `log!` macro — signature and arms
- Content: `src/macros.rs:73-115` (the `#[macro_export] macro_rules! log` definition with all four call-shape arms; comments above each arm document the calling convention).
- Cost: 355 tokens (helper: `count-tokens.py src/macros.rs:73-115`)
- Notes: `log!` is the parameterised primitive the level macros expand to. Shows the four call shapes (`logger:`, `target:`, both, neither) — directly answers "how do I log at a runtime-chosen level?" and reveals the optional `logger:` and `target:` arguments shared by every macro.

### 2.3 Level + LevelFilter enum declarations
- Content: `src/lib.rs:467-499` (`pub enum Level { Error=1, Warn, Info, Debug, Trace }` with discriminants and per-variant docs) and `src/lib.rs:626-649` (`pub enum LevelFilter { Off, Error, Warn, Info, Debug, Trace }`).
- Cost: 277 + 206 = 483 tokens (helper: `count-tokens.py src/lib.rs:467-499 src/lib.rs:626-649`)
- Notes: the two central enums. Discriminants matter — `Level::Error = 1` so `LevelFilter` and `Level` line up at the same numeric values. Heavily referenced from every other API.

### 2.4 `Log` trait declaration with method docs
- Content: `src/lib.rs:1248-1280` (the trait + the three methods `enabled`, `log`, `flush` with their `# For implementors` notes).
- Cost: 278 tokens (helper: `count-tokens.py src/lib.rs:1248-1280`)
- Notes: implementing `Log` is the central extension point. Contains the critical implementor note that `enabled` is **not** automatically called before `log`.

### 2.5 Global state + initialization functions — signatures
- Content: line-prefix-truncated rendering of `set_max_level` (`src/lib.rs:1349-1351`), `max_level` (`src/lib.rs:1395-1396`), `set_boxed_logger` (`src/lib.rs:1419-1420`), `set_logger` (`src/lib.rs:1477-1478`), `logger` (`src/lib.rs:1581`), shown as their `pub fn …` signature lines together with their `#[cfg(...)]` gating attributes, truncated at the body `{`.
- Cost: 111 tokens (helper: `count-tokens.py src/lib.rs:1349-1351 src/lib.rs:1395-1396 src/lib.rs:1419-1420 src/lib.rs:1477-1478 src/lib.rs:1581`)
- Notes: the public init/access surface, condensed. Their bodies and doc comments come later. Names alone unblock most "how do I install a logger?" questions; the `cfg` attributes disclose which functions need `feature = "std"` / `target_has_atomic = "ptr"`.

### 2.6 `STATIC_MAX_LEVEL` constant — full definition
- Content: `src/lib.rs:1602-1624` (the const item with its doc comment and `match cfg!(debug_assertions) { … }` arms).
- Cost: 305 tokens (helper: `count-tokens.py src/lib.rs:1602-1624`)
- Notes: this `match` is the canonical cross-reference of the twelve `max_level_*` / `release_max_level_*` Cargo features. Without it, the agent can't reason about compile-time level filtering at all.

### 2.7 KV submodule entry — what it is and how to enable
- Content: `src/kv/mod.rs:1-60` (module-level rustdoc up to the end of the capturing-modifiers list — explains the `kv` Cargo feature, `info!(a = 1; "...")` syntax, the `:?`/`:%`/`:err`/`:sval`/`:serde` modifiers).
- Cost: 541 tokens (helper: `count-tokens.py src/kv/mod.rs:1-60`)
- Notes: catastrophic-omission risk — without this, an agent never realises the crate supports structured logging at all. The `kv` mod itself only exists when the feature is on (gated at `src/lib.rs:409-410`).

### 2.8 README crate description and library-usage example
- Content: `README.md:1-56` (badges + facade description + `In libraries` section with the yak-shaving example).
- Cost: 411 tokens (helper: `count-tokens.py README.md:1-56`)
- Notes: hits the "what's the user-facing usage pattern?" question with a minimal runnable example.

### 3.1 Logger initialization rustdoc walkthrough
- Content: `src/lib.rs:172-232` — "Implementing a Logger" section, including a complete `SimpleLogger` example with the `set_logger` + `set_max_level` boilerplate and the warning that the default `LevelFilter::Off` means no messages emit.
- Cost: 530 tokens (helper: `count-tokens.py src/lib.rs:172-232`)
- Notes: addresses the most common "I implemented Log but nothing prints" failure. Contains the `static LOGGER: SimpleLogger = SimpleLogger; pub fn init() -> Result<(), SetLoggerError> { … }` pattern verbatim.

### 3.2 Compile-time level filter feature docs
- Content: `src/lib.rs:257-290` — "Compile time filters" section listing the six `max_level_*` and six `release_max_level_*` Cargo features and the rationale ("Libraries should avoid using the max level features").
- Cost: 302 tokens (helper: `count-tokens.py src/lib.rs:257-290`)
- Predecessor: 2.6 (the `STATIC_MAX_LEVEL` const definition gives the matching arms; this batch gives the prose).

### 3.3 `error!` macro — doc + example
- Content: `src/macros.rs:149-165`
- Cost: 125 tokens (helper: `count-tokens.py src/macros.rs:149-165`)
- Predecessor: 2.1
- Notes: shows every call shape (`error!("...")`, `error!(target: …)`, `error!(logger: …)`).

### 3.4 `warn!` macro — doc + example
- Content: `src/macros.rs:188-204`
- Cost: 106 tokens (helper: `count-tokens.py src/macros.rs:188-204`)
- Predecessor: 2.1

### 3.5 `info!` macro — doc + example
- Content: `src/macros.rs:227-251`
- Cost: 172 tokens (helper: `count-tokens.py src/macros.rs:227-251`)
- Predecessor: 2.1

### 3.6 `debug!` macro — doc + example
- Content: `src/macros.rs:275-291`
- Cost: 149 tokens (helper: `count-tokens.py src/macros.rs:275-291`)
- Predecessor: 2.1

### 3.7 `trace!` macro — doc + example
- Content: `src/macros.rs:315-335`
- Cost: 219 tokens (helper: `count-tokens.py src/macros.rs:315-335`)
- Predecessor: 2.1

### 3.8 `log_enabled!` macro — full definition
- Content: `src/macros.rs:359-411` (doc-comment, examples, `macro_rules! log_enabled` with its four arms).
- Cost: 527 tokens (helper: `count-tokens.py src/macros.rs:359-411`)
- Notes: a distinct API the agent needs when expensive-arg-elision questions come up.

### 3.9 `Record` struct fields
- Content: `src/lib.rs:842-867` (the `pub struct Record<'a>` field block including `#[cfg(feature = "kv")] key_values: KeyValues<'a>` and the `KeyValues` wrapper definition).
- Cost: 208 tokens (helper: `count-tokens.py src/lib.rs:842-867`)
- Predecessor: 1.4

### 3.10 `Metadata` struct + accessors
- Content: `src/lib.rs:1157-1182` (`pub struct Metadata<'a> { level, target }` + `impl Metadata<'a>` with `builder()`, `level()`, `target()`).
- Cost: 141 tokens (helper: `count-tokens.py src/lib.rs:1157-1182`)
- Predecessor: 2.4

### 3.11 `Record` accessor methods
- Content: `src/lib.rs:869-942` (`impl<'a> Record<'a>` block: `builder`, `args`, `metadata`, `level`, `target`, `module_path`, `module_path_static`, `file`, `file_static`, `line`, plus the `#[cfg(feature = "kv")] key_values` accessor).
- Cost: 465 tokens (helper: `count-tokens.py src/lib.rs:869-942`)
- Predecessor: 3.9

### 3.12 Crate features section of Cargo.toml
- Content: `Cargo.toml:22-55` (`[features]`: the twelve `*max_level_*` features, `std`, `kv`, `kv_sval`, `kv_std`, `kv_serde`, the deprecated `kv_unstable_*` aliases, and the `serde = ["serde_core"]` shim).
- Cost: 275 tokens (helper: `count-tokens.py Cargo.toml:22-55`)
- Predecessor: 1.3

### 3.13 Cargo.toml dependencies
- Content: `Cargo.toml:56-77` (runtime deps `serde_core`, `sval`, `sval_ref`, `value-bag`; dev deps; the `proc-macro2` minimal-version pin and its explanatory comment).
- Cost: 321 tokens (helper: `count-tokens.py Cargo.toml:56-77`)
- Predecessor: 1.3

### 3.14 Crate-level structured-logging walkthrough
- Content: `src/lib.rs:91-128` ("Structured logging" section with the kv-flavoured yak-shaving example).
- Cost: 308 tokens (helper: `count-tokens.py src/lib.rs:91-128`)
- Predecessor: 1.4

### 3.15 KV submodule item re-exports
- Content: `src/kv/mod.rs:246-266` (the `mod` declarations and `pub use` lines for `Error`, `Key`, `ToKey`, `Source`, `VisitSource`, `ToValue`, `Value`, `VisitValue`, plus the `kv_unstable` `Visitor` alias).
- Cost: 121 tokens (helper: `count-tokens.py src/kv/mod.rs:246-266`)
- Predecessor: 2.7
- Notes: enumerates the entire kv public surface in 21 lines — the cheapest possible "what's in the kv module".

### 3.16 KV `Key` type + `ToKey` trait — sigs and core methods
- Content: `src/kv/key.rs:1-67` (doc + `ToKey` trait + `pub struct Key<'k>` + impls of `from_str`, `as_str`, `to_borrowed_str`).
- Cost: 461 tokens (helper: `count-tokens.py src/kv/key.rs:1-40 src/kv/key.rs:42-67`)
- Predecessor: 2.7

### 3.17 KV `Source` trait declaration
- Content: `src/kv/source.rs:1-88` (module doc + `pub trait Source` with `visit`, `get`, `count` and example `Printer` visitor).
- Cost: 651 tokens (helper: `count-tokens.py src/kv/source.rs:1-88`)
- Predecessor: 2.7

### 3.18 KV `Value` overview rustdoc + `ToValue` trait
- Content: `src/kv/value.rs:1-80` (intro doc up through "capturing values" section, plus `ToValue` trait declaration).
- Cost: 424 tokens (helper: `count-tokens.py src/kv/value.rs:1-80`)
- Predecessor: 2.7

### 4.1 Crate-level rustdoc — middle section ("In libraries", "In executables", "Warning")
- Content: `src/lib.rs:46-90` (yak-shaving library-author example + executable-side notes + `set_logger` once-only warning).
- Cost: 333 tokens (helper: `count-tokens.py src/lib.rs:46-90`)
- Predecessor: 1.4

### 4.2 Crate-level rustdoc — `set_boxed_logger` and `Use with std`
- Content: `src/lib.rs:233-256` ("Use with `std`" section explaining `set_boxed_logger`).
- Cost: 243 tokens (helper: `count-tokens.py src/lib.rs:233-255`)
- Predecessor: 4.1

### 4.3 `set_max_level` family — full bodies + docs
- Content: `src/lib.rs:1344-1404` (`set_max_level`, `set_max_level_racy`, `max_level`, with their full doc comments including the safety note on `set_max_level_racy` and the unsafe `mem::transmute` justification on `max_level`).
- Cost: 622 tokens (helper: `count-tokens.py src/lib.rs:1344-1404`)
- Predecessor: 2.5

### 4.4 `set_boxed_logger` and `set_logger` — full bodies + docs
- Content: `src/lib.rs:1405-1481` (`set_boxed_logger`, `set_logger` with full rustdoc and its long `MyLogger` example).
- Cost: 528 tokens (helper: `count-tokens.py src/lib.rs:1405-1481`)
- Predecessor: 2.5

### 4.5 `set_logger_inner` and `set_logger_racy` bodies
- Content: `src/lib.rs:1482-1542` (the actual `compare_exchange` / `STATE` machinery, plus the `_racy` variant).
- Cost: 431 tokens (helper: `count-tokens.py src/lib.rs:1482-1542`)
- Predecessor: 4.4

### 4.6 `SetLoggerError`, `ParseLevelError`, `logger()` accessor
- Content: `src/lib.rs:1544-1596` (both error structs with their `Display`/`Error` impls plus the `logger()` global accessor with its memory-ordering comment).
- Cost: 428 tokens (helper: `count-tokens.py src/lib.rs:1544-1576 src/lib.rs:1577-1596`)
- Predecessor: 2.5

### 4.7 Static state + state constants
- Content: `src/lib.rs:445-466` (the `LOGGER` `static mut`, `STATE`, `UNINITIALIZED`/`INITIALIZING`/`INITIALIZED` constants, `MAX_LOG_LEVEL_FILTER`, `LOG_LEVEL_NAMES`, and the `SET_LOGGER_ERROR`/`LEVEL_PARSE_ERROR` strings).
- Cost: 229 tokens (helper: `count-tokens.py src/lib.rs:445-466`)
- Predecessor: 4.4

### 4.8 `Level` impl methods (full bodies)
- Content: `src/lib.rs:534-624` (`from_usize`, `max`, `to_level_filter`, `as_str`, `iter`, `increment_severity`, `decrement_severity`).
- Cost: 695 tokens (helper: `count-tokens.py src/lib.rs:534-624`)
- Predecessor: 2.3

### 4.9 `LevelFilter` impl methods (full bodies)
- Content: `src/lib.rs:684-778` (`from_usize`, `max`, `to_level`, `as_str`, `iter`, `increment_severity`, `decrement_severity`).
- Cost: 789 tokens (helper: `count-tokens.py src/lib.rs:684-778`)
- Predecessor: 2.3

### 4.10 `RecordBuilder` definition + setters
- Content: `src/lib.rs:1003-1117` (struct + every setter: `args`, `metadata`, `level`, `target`, `module_path`, `module_path_static`, `file`, `file_static`, `line`, `key_values`, `build`).
- Cost: 916 tokens (helper: `count-tokens.py src/lib.rs:1003-1117`)
- Predecessor: 3.9
- Notes: under the `2×` cap of the largest preceding batch (4.9 = 789 → 1578).

### 4.11 `MetadataBuilder` definition + setters
- Content: `src/lib.rs:1199-1240`
- Cost: 263 tokens (helper: `count-tokens.py src/lib.rs:1199-1240`)
- Predecessor: 3.10

### 4.12 `__log!` and `__log_enabled!` internal macros
- Content: `src/macros.rs:117-147` (`__log!` two-arm definition with kvs and without) and `src/macros.rs:413-437` (`__log_enabled!` and `__log_logger!`).
- Cost: 375 + 175 = 550 tokens (helper: `count-tokens.py src/macros.rs:117-147 src/macros.rs:413-437`)
- Predecessor: 2.2
- Notes: where the actual `STATIC_MAX_LEVEL` / `max_level()` short-circuit lives. Critical for macro-expansion debugging.

### 4.13 `__log_key!` / `__log_value!` internal macros (kv shape)
- Content: `src/macros.rs:439-521` (the `cfg(feature = "kv")` and not-kv variants of `__log_key`, `__log_value`, plus `__log_value_*` per-capture-trait dispatchers).
- Cost: 589 tokens (helper: `count-tokens.py src/macros.rs:439-521`)
- Predecessor: 2.7
- Notes: defines what the `key:capture = expr` syntax desugars to. The `compile_error!` arms are how the crate reports "feature not enabled" messages — agents debugging "kv requires the kv feature" land here.

### 4.14 `__private_api` module overview + `log` / `enabled` entry points
- Content: `src/__private_api.rs:1-110` (the `KVs` sealed trait with its two impls, `GlobalLogger` + its `Log` impl, the `log_impl` function that builds a `Record` and forwards it to the logger, and the `enabled` and `loc` helpers).
- Cost: 681 tokens (helper: `count-tokens.py src/__private_api.rs:1-110`)
- Predecessor: 4.12
- Notes: the path from a macro call to a `Logger::log` call.

### 4.15 KV `Value` data model + serialization rustdoc
- Content: `src/kv/value.rs:80-117` (the data-model bullet list — Null, Strings, Booleans, Integers, Floats, Errors, serde, sval — and the serialization paragraph).
- Cost: 448 tokens (helper: `count-tokens.py src/kv/value.rs:80-117`)
- Predecessor: 3.18

### 4.16 KV `Value` constructor methods (full bodies)
- Content: `src/kv/value.rs:118-220` (`pub struct Value<'v>` + `from_any`, `from_debug`, `from_display`, `from_serde`, `from_sval`, `from_dyn_debug`, `from_dyn_display`, `from_dyn_error`, `null`, `from_inner`, `visit`).
- Cost: 685 tokens (helper: `count-tokens.py src/kv/value.rs:118-220`)
- Predecessor: 4.15

### 4.17 KV `Value::to_*` conversion methods + primitive conversions
- Content: `src/kv/value.rs:336-388` (the `impl_to_value_primitive!` / `impl_to_value_nonzero_primitive!` / `impl_value_to_primitive!` macro invocations declaring `to_u64`, `to_i64`, `to_u128`, `to_i128`, `to_f64`, `to_char`, `to_bool`, `to_borrowed_error`, `to_borrowed_str`).
- Cost: 498 tokens (helper: `count-tokens.py src/kv/value.rs:336-388`)
- Predecessor: 4.16

### 4.18 KV `Error` type definition + impls
- Content: `src/kv/error.rs:1-65` (the `pub struct Error`, the `Inner` enum, `msg`, `Display`, `From<fmt::Error>`).
- Cost: 454 tokens (helper: `count-tokens.py src/kv/error.rs:1-65`)
- Predecessor: 2.7

### 4.19 `Log` trait blanket impls (`&T`, `Box`, `Arc`) + `NopLogger`
- Content: `src/lib.rs:1281-1342` (`NopLogger`, `Log for &T`, `Log for Box<T>`, `Log for Arc<T>`).
- Cost: 324 tokens (helper: `count-tokens.py src/lib.rs:1281-1342`)
- Predecessor: 2.4

### 4.20 `tests/integration.rs` end-to-end mock-logger pattern
- Content: full file, `tests/integration.rs:1-101` (the `Logger`/`State` mock plus `test_filter` and `test_line_numbers`).
- Cost: 785 tokens (helper: `count-tokens.py tests/integration.rs`)
- Notes: the canonical mock-logger pattern. Anyone writing tests for downstream code copies this verbatim.

### 4.21 `test_max_level_features/main.rs` (compile-time level test harness) + its Cargo.toml
- Content: full files, `test_max_level_features/main.rs:1-76` and `test_max_level_features/Cargo.toml:1-13`.
- Cost: 479 + 65 = 544 tokens (helper: `count-tokens.py test_max_level_features/main.rs test_max_level_features/Cargo.toml`)
- Predecessor: 3.2
- Notes: only place where `max_level_debug` + `release_max_level_info` interaction is exercised end-to-end; combined with the `Cargo.toml` features key, fully describes the compile-time filtering contract.

### 4.22 `serde::Serialize` impls for `Level` and `LevelFilter`
- Content: `src/serde.rs:1-29` (`Serialize for Level`) and `src/serde.rs:110-124` (`Serialize for LevelFilter`).
- Cost: 227 + 167 = 394 tokens (helper: `count-tokens.py src/serde.rs:1-29 src/serde.rs:110-124`)
- Predecessor: 2.3
- Notes: behaviour worth pre-loading: `Level::Error` serialises with variant index `0` (not `1`); `LevelFilter::Off` serialises with index `0`. The full `Deserialize` machinery (case-insensitive `FromStr`, byte/integer support) is one `Read` away.

### 4.23 CHANGELOG header — current version + most recent releases
- Content: `CHANGELOG.md:1-31` (the `[Unreleased]`, `[0.4.29]`, `[0.4.28]` entries).
- Cost: 437 tokens (helper: `count-tokens.py CHANGELOG.md:1-31`)
- Predecessor: 1.3
- Notes: tells the agent what shipped in the pinned version (`perf: reduce llvm-lines of FromStr`, `Replace serde with serde_core`) so it doesn't propose changes that already landed.

## Below-the-fold

- `src/kv/mod.rs:60-244` (~1570 tokens) — the rest of the `kv` module rustdoc walking through `Source::get`, `VisitSource`, `Value::to_i64`, `VisitValue` for primitive extraction, serde/sval serialisation, and Debug/Display fallback. The `kv` API surface is already in 2.7 + 3.15–3.18 + 4.15–4.18; this is example-heavy prose, deferrable.
- `tests/macros.rs:1-429` (~4400 tokens) — exercises every call shape of every level macro (`no_args`, `anonymous_args`, `named_args`, kv shapes, level-iter). The macro definitions in 2.1, 2.2, 3.3–3.7 already document each call shape via doc comments; a `Read tests/macros.rs` is one hop when an agent needs to verify what compiles.
- `.github/workflows/main.yml` (~1219 tokens) — CI matrix covering stable/beta/nightly + macOS/Win32/Win64/MinGW + thumbv6m + riscv32imc, MSRV pin (1.68.0), `cargo hack --feature-powerset`, `-Z avoid-dev-deps`, `-Z minimal-versions`, `cargo doc --features std,kv,kv_std,kv_sval,kv_serde`. Useful for "what targets are supported?" questions but rarely critical at session start; one `Read` away.
- `src/serde.rs:30-130` plus `src/serde.rs:131-397` (~2500 tokens combined) — the `Deserialize` impls for `Level`/`LevelFilter` (case-insensitive, accepts both string and integer variants), and the `serde_test` round-trip tests. Behaviour summary already in 4.22; full implementation one `Read` away.
- `src/lib.rs:305-346` (~700 tokens) — "Version compatibility" section + the long block of intra-doc link definitions. Pure docs.rs link plumbing.
- `src/lib.rs:357-394` (~547 tokens) — the two `#[cfg(any(...))] compile_error!("multiple max_level_* features set");` guards. Their effect is fully visible from Cargo.toml's feature list (3.12) plus the `STATIC_MAX_LEVEL` const (2.6); the message is shown directly at compile time when triggered.
- `src/lib.rs:399-444` (~273 tokens) — top-level `use` statements, module declarations, the `#[cfg(not(target_has_atomic = "ptr"))]` `Cell`-based `AtomicUsize` fallback. Niche unless the task is specifically about `thumbv6m` / no-atomics targets.
- `src/lib.rs:501-532` and `src/lib.rs:651-682` (~450 tokens combined) — `PartialEq<LevelFilter> for Level`, `PartialOrd`, `FromStr for Level`, `Display for Level`, and the matching `LevelFilter` impls. Behaviour is implied by the enum declarations + `Level::as_str` + `Level::iter` already in 2.3 + 4.8 + 4.9.
- `src/kv/value.rs:450-536` (~750 tokens) — `pub trait VisitValue<'v>` with all default `visit_*` methods (u64/i64/u128/i128/f64/bool/str/borrowed_str/char/error/borrowed_error). Internals trait — most users go through `Value::to_*` (4.17) instead.
- `src/kv/source.rs:130-232` (~620 tokens) — `Source` impls for `&T`, `(K, V)`, `[S]`, `[S; N]`, `Option<S>`. Mechanical glue; the `Source` trait itself in 3.17 is the load-bearing piece.
- `src/kv/source.rs:234-278` (~460 tokens) — `pub trait VisitSource` and its blanket impls for `fmt::DebugMap`/`DebugList`/`DebugSet`/`DebugTuple`. Shape implied by the `Printer` example already shown in 3.17.
- `src/kv/source.rs:280-406` (~900 tokens) — `cfg(feature = "std") mod std_support` impls of `Source` for `Box`/`Arc`/`Rc`/`Vec`/`HashMap`/`BTreeMap`. Mechanical std-glue.
- `src/kv/source.rs:408-515` (~410 tokens) — `Source`/`VisitSource` unit tests.
- `src/__private_api.rs:112-152` (~387 tokens) — `kv_support` capture functions (`capture_to_value`, `capture_debug`, `capture_display`, `capture_error`, `capture_sval`, `capture_serde`). Trivially one-line wrappers over `Value::from_*` already in 4.16.
- `src/macros.rs:523-579` (~360 tokens) — `__log_value_sval!`, `__log_value_serde!`, `__log_value_error!` and their `compile_error!` siblings. Mirror of the kv-feature dispatchers in 4.13.
- `src/kv/key.rs:90-164` (~410 tokens) — `cfg(feature = "std")` `String`/`Cow<str>` `ToKey` impls; `kv_sval` `Value`/`ValueRef` impls for `Key`; `kv_serde` `Serialize` impl for `Key`; two trivial unit tests. Mechanical glue.
- `src/kv/error.rs:67-94` (~145 tokens) — `cfg(feature = "std")` `Error::boxed` constructor, `From<io::Error>` impl. Behaviour implicit in the `Inner::Boxed` variant already shown in 4.18.
- `src/kv/value.rs:222-335` and `:389-449` (~700 tokens combined) — the `Display`/`Debug`/`serde::Serialize`/`sval::Value` impls for `Value`, the `From<&str>` and primitive `From` impls, and the `cfg(feature = "kv_std")` `Cow` impls. Plumbing for the constructors already in 4.16.
- `src/kv/value.rs:601-1049` (~5000 tokens) — the `value-bag` and dependency-free `inner` modules implementing the actual storage `Inner` enum with all `From` conversions, the visit dispatch, and the test `Token` enum. Internal implementation; the public `Value` API in 4.15–4.17 tells an agent everything needed to *use* `Value`.
- `src/kv/value.rs:1051-1173` (~700 tokens) — deprecated `kv_unstable` constructors (`capture_debug`, `capture_display`, `capture_error`, `capture_serde`, `capture_sval`, `is`, `downcast_ref`) and the deprecated `as_debug!` / `as_display!` / `as_error!` / `as_serde!` / `as_sval!` macros. Pure deprecation surface; the docs explicitly point to the non-deprecated replacements.
- `src/kv/value.rs:1175-1396` (~1900 tokens) — the in-file `tests` module for `Value`. Useful when modifying the type but irrelevant to the API.
- `src/lib.rs:1626-2009` (~2000 tokens) — the in-file unit-test module covering every `Level`/`LevelFilter` impl, builder construction, and the `#[test] fn test_foreign_impl` type-level check. Adds no public-API insight beyond what's already ranked.
- `README.md:58-105` (~700 tokens) — bulleted catalogue of compatible logger crates (`env_logger`, `log4rs`, `syslog`, `console_log`, etc.). Useful for "which logger should I pick?" but rarely critical mid-session; the same list is repeated in `src/lib.rs:130-170`.
- `src/lib.rs:130-170` (~330 tokens) — duplicate of the README ecosystem catalogue.
- `README.md:107-134` (~225 tokens) — README structured-logging example. Subsumed by the in-crate version in 3.14.
- `benches/value.rs` (155 tokens) — four micro-benchmarks (`u8_to_value`, `u8_to_value_debug`, `str_to_value_debug`, `custom_to_value_debug`). One-call-away if the agent's task is benchmark-related.
- `rfcs/0296-structured-logging.md` (~13.6k tokens) — the original 2019 design RFC. `Cargo.toml` excludes it from the published crate (`exclude = ["rfcs/**/*"]`); the live `kv` rustdoc already supplies the design rationale at a fraction of the cost. Below-the-fold for any task that doesn't specifically ask "why was kv designed this way?". One `Read` away.
- `CHANGELOG.md:32-423` (~5760 tokens) — older version history (0.4.27 and earlier). 4.23 carries the latest two entries, which is what most refactor/bugfix tasks need.
- `LICENSE-APACHE`, `LICENSE-MIT`, `triagebot.toml`, `.gitignore` — boilerplate. The top-level listing in 1.1 already discloses they exist.
