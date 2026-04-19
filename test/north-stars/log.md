# North Star — `log`

- Fixture: `test/fixtures/log/`
- Revision pin: `43f2c283`

`log` is the de-facto Rust logging *facade*: a tiny crate that defines a single `Log` trait, five level macros (`error!`/`warn!`/`info!`/`debug!`/`trace!`), an installable global logger, and an optional structured key-value extension behind the `kv` feature. It does not itself emit log output — libraries call the macros, the binary picks an implementation crate (env_logger, log4rs, ...) and calls `log::set_logger` once at startup. An agent landing in this repo needs to know three things on arrival: (1) what the user-facing macro/level surface is, (2) what trait a new logger implementation has to satisfy, and (3) what the kv module exposes for structured fields. Most other content is doc, license, CI, regression tests, or implementation detail behind cargo features.

The fixture is a single-crate workspace plus a tiny side crate (`test_max_level_features/`) that exercises the compile-time level-filter features under release vs debug. The 1.6k-line RFC under `rfcs/` is historical context, not current API.

Whole-fixture token count: ~72.5k. North-Star ranked content: ~20.3k tokens (T1+T2+T3+T4+T5, with T4.5 in its line-prefix-truncated variant). Without T4.5 the total drops to ~18.6k; with T4.5 in its full form it rises to ~21.3k — the cap is comfortably negotiated by toggling that one batch's truncation. Tier subtotals: T1 = 5.6k, T2 = 5.4k, T3 = 4.1k, T4 = 4.0–5.0k, T5 = 1.2k. The chief omissions are the CHANGELOG body, the structured-logging RFC, the value-bag-backed inner Value implementation, the serde Visitor machinery, and the verbose macro-permutation test files — see *Below the fold*.

The repository renders as a tree of folders → files → file content. Throughout the document, "structural parent" means the batch one level up in that tree (folder name → file name → content), and a file-content batch is only useful once its file's name is shown under its folder.

---

## Tier 1 — without these, nothing else makes sense

These batches collectively answer "what is this crate, what does the user type, what do I implement to plug into it." They are small, dense, and high-leverage. Tier total: ~5.6k tokens.

### T1.1 — Crate-root file list and folder names (~30 tokens)
Show the top-level directory entries: `src/`, `tests/`, `benches/`, `rfcs/`, `test_max_level_features/`, `.github/`, plus the files `Cargo.toml`, `README.md`, `CHANGELOG.md`, `LICENSE-APACHE`, `LICENSE-MIT`, `triagebot.toml`, `.gitignore`. Plus the `src/` listing (`lib.rs`, `macros.rs`, `__private_api.rs`, `serde.rs`, `kv/`) and the `src/kv/` listing (`mod.rs`, `key.rs`, `value.rs`, `source.rs`, `error.rs`).

Structural parent: the fixture root. Ordering: folders before files within each level; within `src/`, `lib.rs` first (entry point), then `macros.rs` (the user-visible surface), then `kv/`, then private modules.

Token estimate: ~30 (folder/file names only).

### T1.2 — `src/lib.rs` crate-level doc summary (lines 11–127), the "what is this crate" paragraph and Usage/Libraries/Executables/Structured-logging examples (~1020 tokens)
The crate-root `//!` doc is the single most informative artifact in the entire fixture for a cold agent. It explains the facade model ("if no logging implementation is selected, the facade falls back to a 'noop' implementation"), the target/level/body record shape, the five-macro surface, the library-vs-executable split, and the kv structured-logging story — all with runnable examples.

Path: `src/lib.rs:11-127`. Structural parent: `src/lib.rs` file batch (T1.4). Ordering: must precede any in-file item batch.

Token estimate: 1020.

### T1.3 — `Cargo.toml` package metadata + feature matrix (lines 1–55) (~395 tokens)
Two adjacent ranges read together: the `[package]` block (1–18) gives crate identity, MSRV (`rust-version = "1.68.0"`), edition, the `exclude = ["rfcs/**/*"]` (which itself signals that rfcs/ is ignorable in normal builds), and the `[package.metadata.docs.rs]` enabled features. The `[features]` block (22–55) is load-bearing: it enumerates `max_level_*`, `release_max_level_*`, `std`, `kv`, `kv_sval`, `kv_std`, `kv_serde`, the `serde = ["serde_core"]` alias, and the deprecated `kv_unstable*` aliases. Almost every conditional `#[cfg(feature = ...)]` in the source refers back to this matrix; an agent that hasn't seen it will misread half the source.

Path: `Cargo.toml:1-55`. Structural parent: crate root. Ordering: should precede any source-file batch that gates content on a feature.

Token estimate: 395 (120 + 275).

### T1.4 — `src/lib.rs` public-item skeleton: enum bodies of `Level` and `LevelFilter` (lines 467–498 and 626–649) (~482 tokens)
The two enums are the load-bearing vocabulary of the entire crate. They are tiny (six discriminants between them) but essential — every macro invocation, every `Log::enabled` call, and every cross-comparison is in terms of these. Show both bodies together, including the rustdoc on each variant ("Designates very serious errors", "A level lower than all log levels", etc.) so the agent knows that `Level` has 5 variants starting at 1 (no `Off`) while `LevelFilter` has 6 starting at 0 (with `Off`). The discriminant alignment between the two is a subtle invariant that the doc comments call out.

Paths: `src/lib.rs:467-498` (Level body), `src/lib.rs:626-649` (LevelFilter body). Structural parent: `src/lib.rs` file batch (T1.2 region). Ordering: Level before LevelFilter (matches in-source order and the conceptual "value" → "filter that admits values" relationship).

Token estimate: 482 (276 + 206).

### T1.5 — `src/lib.rs` `Log` trait definition with rustdoc, lines 1248–1280 (~278 tokens)
The three-method trait (`enabled`, `log`, `flush`) plus the per-method "For implementors" notes (especially: "Note that `enabled` is *not* necessarily called before this method") is the entire contract a downstream logger crate must satisfy. Showing this without the surrounding `impl Log for &T / Box<T> / Arc<T>` blocks keeps it focused; the impl-for-wrappers are deferred to T3.

Path: `src/lib.rs:1248-1280`. Structural parent: `src/lib.rs`.

Token estimate: 278.

### T1.6 — `src/macros.rs` rustdoc-bearing entries for the user-facing macros (lines 11–66 + 149–186 + 188–225 + 227–273 + 275–313 + 315–357 + 359–411) (~3360 tokens)
The seven sibling macros are the actual public API surface for users. Each carries a non-trivial doc block with example invocations that demonstrate the four argument shapes: bare, `target:`, `logger:`, and `target: + logger:`. Together they are the bulk of what a user types. Group as one batch because rendering only some of them would mislead — the user should see that *all* level macros accept the same shapes, and that `log!` is the generic underlying macro.

Within this batch, ordering follows the source: `log!` (with its own header doc, lines 11–66) first, then severity-descending `error!` → `warn!` → `info!` → `debug!` → `trace!`, then `log_enabled!`. This matches both source order and the documented "from highest to lowest priority" convention.

Sibling ordering constraint with T1.5: `Log` trait first (what to implement) then macros (what to call), because an agent reading the macros benefits from already knowing what `Log::log` and `Log::enabled` look like.

Path ranges: `src/macros.rs:11-66`, `src/macros.rs:149-186`, `src/macros.rs:188-225`, `src/macros.rs:227-273`, `src/macros.rs:275-313`, `src/macros.rs:315-357`, `src/macros.rs:359-411`. Structural parent: `src/macros.rs` file batch (file is `mod macros;` in `src/lib.rs:406`, declared `#[macro_use]`).

Token estimate: 3360 (363 + 465 + 446 + 517 + 494 + 564 + 527, summing the per-range counts measured above; the gap-skipping vs. the contiguous 11–411 sum shows ~800 tokens of `__log!`/`__log_enabled!`/`__log_logger!` internal-macro plumbing that we deliberately drop here and reintroduce as a smaller T3 batch).

---

## Tier 2 — the next things you reach for on a typical task

The Record/Metadata data model, the global-state initialization API, the kv module overview, and the compile-time-level-filter machinery. With T1+T2 the agent can answer the vast majority of "how do I do X with log" questions without a follow-up read. Tier total: ~5.4k tokens.

### T2.1 — `src/lib.rs` `Record` + `RecordBuilder` definitions and methods (lines 780–961 and 1003–1117) (~1.81k tokens)
`Record<'a>` is the payload `Log::log` receives. The struct fields (`metadata`, `args`, `module_path`, `file`, `line`, optional `key_values`), the accessor methods (`args()`, `metadata()`, `level()`, `target()`, `module_path()`, `module_path_static()`, `file()`, `file_static()`, `line()`, `key_values()`, `to_builder()`), and the parallel `RecordBuilder` with one setter per field. The `MaybeStaticStr` two-variant helper sits just above `Record` (lines 780–794) and is worth including in the same batch because both `module_path_static`/`module_path` and `file_static`/`file` only make sense once you know about it.

Path: `src/lib.rs:780-961` (MaybeStaticStr + Record + KeyValues + accessors), `src/lib.rs:1003-1117` (RecordBuilder). Structural parent: `src/lib.rs`.

Token estimate: 1810 (1227 + 583).

### T2.2 — `src/lib.rs` `Metadata` + `MetadataBuilder` (lines 1119–1240) (~825 tokens)
`Metadata<'a>` is the (level, target) pair passed to `Log::enabled`; `MetadataBuilder` mirrors it. Smaller than Record but referenced by the `Log::enabled` method shown in T1.5, so close in priority. Worth a dedicated sibling-of-Record batch with explicit ordering: Record first (because it's referenced by `Log::log`), Metadata second.

Path: `src/lib.rs:1119-1240`. Structural parent: `src/lib.rs`.

Token estimate: 823 (345 + 478).

### T2.3 — `src/lib.rs` global-logger init API: `set_max_level`, `max_level`, `set_boxed_logger`, `set_logger`, `logger` (lines 1344–1404 + 1406–1481 + 1578–1601) (~1.45k tokens)
The five free functions that any logger implementation calls during init, plus the `logger()` accessor. Includes the inline example showing `set_logger(&MY_LOGGER).unwrap(); set_max_level(LevelFilter::Info);`. The example alone resolves a frequent point of confusion (logger and max-level are independent stores; both must be set).

Skip the `_racy` variants and the `set_logger_inner` private helper — they're niche and demoted to T4.

Paths: `src/lib.rs:1344-1404` (set_max_level + max_level with their docs), `src/lib.rs:1406-1481` (set_boxed_logger + set_logger with the worked example), `src/lib.rs:1581-1596` (the `logger()` accessor — small but completes the picture).

Token estimate: 1300 (622 + 527 + 151).

### T2.4 — `src/lib.rs` `STATIC_MAX_LEVEL` const with its cfg-feature ladder (lines 1602–1624) (~305 tokens)
The compile-time max level constant — the one that lets `error!`/`info!` macros optimize away entirely under `release_max_level_*`. The match arms are a near-1-to-1 mirror of the cargo features in T1.3, so showing this batch second resolves the "what do those features actually do" question. Also referenced from `src/macros.rs` `__log!` body (`if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level()`).

Path: `src/lib.rs:1602-1624`. Structural parent: `src/lib.rs`. Sibling ordering: must come *after* T1.3 cargo features so the `cfg!(feature = ...)` branches read meaningfully.

Token estimate: 305.

### T2.5 — `src/kv/mod.rs` module-level rustdoc: structured-logging guide (lines 1–130) (~1.13k tokens)
The first half of the kv module doc covers the full agent-relevant surface: the `key = value` macro syntax, the capture modifiers (`:?`, `:debug`, `:%`, `:display`, `:err`, `:sval`, `:serde`) and which feature each requires, accessing kv via `Record::key_values()`, getting one value by key, and visiting all values via `VisitSource`. This is the only place the modifier table is documented. The second half (130–244, more advanced serde/sval examples) is demoted to T4 to keep the batch tight.

Path: `src/kv/mod.rs:1-130`. Structural parent: `src/kv/mod.rs`.

Token estimate: 1134.

### T2.6 — `src/kv/mod.rs` re-exports block: lines 246–266 (~120 tokens)
The `mod`/`pub use` block is the single source of truth for the kv module's public shape: `Error`, `Key`, `ToKey`, `Source`, `VisitSource`, `Value`, `ToValue`, `VisitValue`, plus the `kv_unstable` re-export tweaks. Tiny but indispensable for "where do I import X from" questions.

Path: `src/kv/mod.rs:246-266`. Structural parent: `src/kv/mod.rs`. Sibling ordering: comes after the module rustdoc (T2.5) — first you read about the API, then you see the export list.

Token estimate: 121.

---

## Tier 3 — concrete API for tasks involving kv or wrappers

Once an agent is in the kv module or building a logger wrapper, these are the exact items they need. Same priority bucket because they tend to be needed together. Tier total: ~4.1k tokens.

### T3.1 — `src/kv/key.rs` Key + ToKey core (lines 1–91) (~640 tokens)
`ToKey` trait with its four blanket impls, then `Key<'k>` struct + `from_str`, `as_str`, `to_borrowed_str`, plus the `Display`/`AsRef<str>`/`Borrow<str>`/`From<&str>` impls. The `// NOTE: This may become Cow<'k, str>` comment is worth preserving — it tells an agent why some methods return `&str` instead of `&'k str`.

Demote the `cfg(feature = "std")` `String`/`Cow` `ToKey` impls and the `kv_sval`/`kv_serde` impls (lines 92–149) to T4.

Path: `src/kv/key.rs:1-91`. Structural parent: `src/kv/key.rs`.

Token estimate: 640.

### T3.2 — `src/kv/source.rs` Source + VisitSource trait definitions (lines 1–88 and 234–238) (~715 tokens)
`Source` (with the worked Printer example), its `visit`/`get`/`count` methods, and the `VisitSource` trait. Together these are the API for reading kv data from a `Record`. Crucial for any agent writing a custom logger that wants to consume structured fields.

Skip the blanket impls for tuples, slices, arrays, Option, Box, Arc, Rc, Vec, HashMap, BTreeMap, DebugMap, etc. (lines 89–432) — those follow the obvious `Source for T` pattern and an agent can infer them. Demote to T4 as a "blanket-impl coverage list" batch.

Paths: `src/kv/source.rs:1-88`, `src/kv/source.rs:234-238`. Structural parent: `src/kv/source.rs`.

Token estimate: 713 (651 + 62).

### T3.3 — `src/kv/value.rs` Value documentation header, lines 33–122 (~730 tokens)
The big rustdoc on `Value<'v>` is the data-model document: it spells out the three capture paths (`Value::from_*`, `ToValue`, `From`), the supported underlying types (Null/Strings/Booleans/Integers/Floats/Errors/serde/sval), and the serialization story (`visit` for primitives, `serde::Serialize`/`sval::Value` for complex). This is the conceptual bridge from "I have some data" to "what do I do with it."

Path: `src/kv/value.rs:33-122` (covers ToValue blanket impls + Value struct decl + the doc block). Structural parent: `src/kv/value.rs`.

Token estimate: 731.

### T3.4 — `src/kv/value.rs` Value constructors: `from_any`/`from_debug`/`from_display`/`from_serde`/`from_sval`/`from_dyn_debug`/`from_dyn_display`/`from_dyn_error`/`null`/`visit` (lines 123–220) (~664 tokens)
The actual construction surface for `Value`. Each method is one to three lines plus a one-line rustdoc. Critical for "how do I capture X" tasks. The `cfg(feature = ...)` gates on `from_serde`/`from_sval`/`from_dyn_error` are visible here, which avoids mistakes.

Path: `src/kv/value.rs:123-220`. Structural parent: `src/kv/value.rs`. Sibling ordering: after T3.3 (header explains why; this shows how).

Token estimate: 664.

### T3.5 — `src/kv/value.rs` primitive-conversion macros and the `to_*` accessors (lines 349–374) (~302 tokens)
The `impl_to_value_primitive![usize, u8, ..., bool]`, `impl_to_value_nonzero_primitive![NonZeroUsize, ...]`, and `impl_value_to_primitive![to_u64, to_i64, to_u128, to_i128, to_f64, to_char, to_bool]` macro invocations. These are the entire "what numeric/scalar types convert in and out of Value" inventory in two dozen lines. Far better than reading the macro definitions themselves.

Path: `src/kv/value.rs:349-374`. Structural parent: `src/kv/value.rs`.

Token estimate: 302.

### T3.6 — `src/kv/value.rs` `VisitValue` trait (lines 450–536) (~750 tokens)
The visitor surface for primitive-typed extraction from a `Value`: `visit_any` (required) plus default-implemented `visit_null`/`visit_u64`/`visit_i64`/`visit_u128`/`visit_i128`/`visit_f64`/`visit_bool`/`visit_str`/`visit_borrowed_str`/`visit_char`/`visit_error`/`visit_borrowed_error`. Mirrored later for `serde_value`/`sval` integration.

Path: `src/kv/value.rs:450-536`. Structural parent: `src/kv/value.rs`. Sibling ordering: after `Value` itself (T3.3/T3.4) — visitor only makes sense once you know what's being visited.

Token estimate: 751.

### T3.7 — `src/lib.rs` `Log` blanket impls for `&T`, `Box<T>`, `Arc<T>` (lines 1294–1342) plus `NopLogger` (1283–1292) (~360 tokens)
Three boilerplate blanket impls plus the no-op default logger. Important to show because they answer "can I store my logger in an `Arc`?" yes, and "what does the global logger do before init?" → `NopLogger`. Compact when shown together.

Path: `src/lib.rs:1283-1342`. Structural parent: `src/lib.rs`. Sibling ordering: after T1.5 (`Log` trait); these are extensions of it.

Token estimate: 315.

---

## Tier 4 — useful when you're already in the area

Detail-tier batches that an agent only reaches after narrowing in. With T1+T2+T3+T4 the agent has ~19k tokens and a near-complete picture. Tier total: ~4.0–5.0k tokens depending on whether T4.5 is shown full or line-prefix-truncated.

### T4.1 — `src/__private_api.rs` GlobalLogger + log entry-point + `loc()` (lines 1–110) (~680 tokens)
This is the seam between the macros and the user-facing API. The macros call `__private_api::log(logger, args, level, &(target, module_path, loc), kvs)`. Showing this resolves the "where does the macro actually go" question. The `KVs` sealed trait + `GlobalLogger` are the type machinery; `log_impl` is the body that builds a `Record` and calls `Log::log`.

Path: `src/__private_api.rs:1-110`. Structural parent: `src/__private_api.rs`. Sibling ordering: after macros (T1.6) — answers "what does the macro expand to."

Token estimate: 681.

### T4.2 — `src/__private_api.rs` kv-feature capture functions (lines 112–152) (~370 tokens)
The `capture_to_value`/`capture_debug`/`capture_display`/`capture_error`/`capture_sval`/`capture_serde` functions used by the `__log_value!` macro arms. Each is two or three lines. Together they show how each `key:?`/`key:%`/`key:err`/`key:sval`/`key:serde` modifier translates into a typed call.

Path: `src/__private_api.rs:112-152`. Structural parent: `src/__private_api.rs`. Sibling ordering: after T4.1.

Token estimate: 369.

### T4.3 — `src/macros.rs` internal expansion-helper macros: `__log!`, `__log_enabled!`, `__log_logger!`, `__log_key!`, `__log_value!`, `__log_value_sval!`, `__log_value_serde!`, `__log_value_error!` (lines 117–147 + 413–579) (~800 tokens)
The five-or-so internal `#[doc(hidden)]` macros that the user-visible macros expand into. They're what make the kv-modifier syntax work and what produce the `compile_error!` messages when a feature isn't enabled. Useful when debugging a macro-expansion error or extending the syntax. Lower priority than the public macros because users never type these directly.

Paths: `src/macros.rs:117-147` (the `__log!` body), `src/macros.rs:413-579` (everything else hidden). Structural parent: `src/macros.rs`. Sibling ordering: after public macros (T1.6).

Token estimate: 1494 (375 + 1119) — heavier than first estimated; could be cut to T5 if budget pressure mounts, since `__log_value_*` arms are mostly cfg-gated `compile_error!` arms.

### T4.4 — `src/lib.rs` error types `SetLoggerError` and `ParseLevelError` (lines 1547–1576) (~190 tokens)
Two zero-field tuple structs with `Display` impls, conditionally implementing `error::Error`. Tiny but essential for any code that handles the `Result` from `set_logger` or `from_str`.

Path: `src/lib.rs:1547-1576`. Structural parent: `src/lib.rs`.

Token estimate: 224.

### T4.5 — `src/lib.rs` Level/LevelFilter helper methods (`from_str`, `Display`, `from_usize`, `to_level_filter`/`to_level`, `as_str`, `iter`, `increment_severity`, `decrement_severity`, plus cross-type Eq/Ord) (lines 501–533 + 651–683 + 534–624 method bodies + 684–778 method bodies) (~1.7k tokens, line-prefix-truncatable)
The methods around the two enums. Most have a short doc comment with an example. Worth showing as a single batch because Level and LevelFilter mirror each other and an agent comparing them benefits from seeing both side by side. If budget is tight, a line-prefix-truncated version (each `pub fn` truncated at `{`) costs ~700 tokens and is sufficient for surface-area tasks.

Paths: `src/lib.rs:501-533` (cross-type Eq/Ord/FromStr/Display for Level), `src/lib.rs:534-624` (Level methods), `src/lib.rs:651-683` (cross-type for LevelFilter), `src/lib.rs:684-778` (LevelFilter methods). Structural parent: `src/lib.rs`. Sibling ordering: after T1.4 (enum bodies).

Token estimate: ~1700 full / ~700 if line-prefix-truncated at `{`.

### T4.6 — `src/lib.rs` `_racy` thread-unsafe variants: `set_max_level_racy`, `set_logger_racy` (lines 1355–1380 + 1510–1542) (~520 tokens)
Niche but called out in the embedded/no-atomics paths and in the CI matrix (`thumbv6m-none-eabi`). Worth keeping discoverable for any embedded-target work.

Path: `src/lib.rs:1355-1380` and `src/lib.rs:1510-1542`. Structural parent: `src/lib.rs`. Sibling ordering: paired with T2.3 init API.

Token estimate: 520 (253 + 267).

---

## Tier 5 — nice to have, often skippable

Test files, side crate, CI, README. Useful for context but not for typical implementation tasks. Tier total: ~1.2k tokens shown selectively.

### T5.1 — `tests/integration.rs:1-23` (test logger struct + Log impl) and `tests/macros.rs:1-22` (the `all_log_macros!` test helper + `Logger` test struct) (~290 tokens)
The minimal "here's how to write a test logger" pattern, useful as a copy-paste reference. The two test-logger struct definitions show the two common shapes (with state via `Arc<Mutex<...>>` vs. stateless).

Paths: `tests/integration.rs:1-23`, `tests/macros.rs:1-22`. Structural parent: `tests/`.

Token estimate: 291 (152 + 139).

### T5.2 — `test_max_level_features/Cargo.toml` + `main.rs:1-42` (~340 tokens)
Side crate that exercises `max_level_debug + release_max_level_info`. Worth showing because (a) it documents the intended use of those features (test the compile-time level filter) and (b) the file paths show up in the CI yml. The full main.rs body is testing logic; the first 42 lines (logger struct + init + the `test()` function calls) are the structural core.

Paths: `test_max_level_features/Cargo.toml`, `test_max_level_features/main.rs:1-42`. Structural parent: `test_max_level_features/`.

Token estimate: 342 (65 + 277).

### T5.3 — `.github/workflows/main.yml:1-45` job names plus the test/check matrix (~376 tokens)
The `test` job line (`cargo hack test --feature-powerset --exclude-features max_level_*,release_max_level_*`) is itself documentation — it tells an agent that the only sanctioned way to test all features is `cargo hack` and that the max_level features must be excluded from the powerset. The other jobs (check/doc/features/minimalv/msrv/embedded) are listed by name only at this level.

Path: `.github/workflows/main.yml:1-45`. Structural parent: `.github/workflows/`.

Token estimate: 376.

### T5.4 — `README.md:106-134` structured-logging example (~227 tokens)
The README is largely redundant with the lib.rs crate doc, *except* for this final block which is a slightly different framing of the kv example. Worth showing if budget remains; otherwise drop entirely in favor of T1.2.

Path: `README.md:106-134`. Structural parent: crate root.

Token estimate: 227.

---

## Below the fold — considered and omitted

Each of these was a real candidate. The justification for each cut is brief and concrete.

- **`CHANGELOG.md` (6196 tokens, 423 lines).** Almost pure release-history prose. Even the most recent entry (T1's "show me what changed lately" use case) is satisfied by the version field in `Cargo.toml` and the `## [Unreleased]`/`## [0.4.29]` header pair, which a focused 15-line slice could hit (~172 tokens), but in practice an agent debugging *current* code rarely needs PR-by-PR history. **Cut entirely.** If we wanted a token, slice `CHANGELOG.md:1-15` (172 tokens) for "what's the current/last release."

- **`rfcs/0296-structured-logging.md` (13634 tokens, 1651 lines).** Historical design RFC from 2019. Cargo.toml literally `exclude`s `rfcs/**/*` from packaging. The current kv API is documented in `src/kv/mod.rs:1-130` (T2.5), which captures the actual shipped surface. Reading the RFC is only useful for archaeology of why-did-they-pick-this. **Cut entirely** — keep the file/folder name visible (T1.1) so the agent can find it if needed.

- **`LICENSE-APACHE` (10847 bytes), `LICENSE-MIT` (1071 bytes).** Standard boilerplate. **Cut entirely** — show file names only (T1.1).

- **`triagebot.toml` (3 tokens, content `[assign]`).** Nothing to show beyond the file name. **Cut.**

- **`.gitignore` (7 tokens).** Three standard entries (`target/`, `Cargo.lock`, `.idea/`). **Cut** — nothing surprising.

- **`src/lib.rs:357-394` (the two big `compile_error!` `cfg(any(...))` ladders for "multiple max_level features set").** Important *behaviour*, but the body is mechanical and an agent who's seen the feature list (T1.3) and `STATIC_MAX_LEVEL` (T2.4) can infer that conflicts are detected. **Cut** — could resurface as a one-line "conflict detection lives in lib.rs:357-394" cross-reference.

- **`src/lib.rs:399-444` (`AtomicUsize` polyfill for non-atomic targets).** Implementation detail behind `#[cfg(not(target_has_atomic = "ptr"))]`. The fact that it exists is communicated by the `_racy` variants (T4.6); the polyfill itself is rarely touched. **Cut.**

- **`src/lib.rs:1626-2010` (the `#[cfg(test)]` mod tests` for the crate root).** ~1100 tokens of unit tests covering Level/LevelFilter/Record/Metadata builders. Useful as a worked example of the API but redundant with the doc-comment examples in T1/T2/T4. **Cut**, but worth a single sentence in any rendered output: "unit tests live in `src/lib.rs:1626-2010`."

- **`src/lib.rs:1483-1508` (`set_logger_inner` private helper).** Atomic compare-exchange dance. Important *invariant* (initialization is one-shot), but the public surface (T2.3) and the doc on `set_logger` already tell the agent that. **Cut.**

- **`src/serde.rs` (2853 tokens, 397 lines).** The full Serialize/Deserialize Visitor implementation for Level and LevelFilter. The fact that these impls exist is communicated by `Cargo.toml`'s `serde` feature (T1.3) and the doc in T1.2. The Visitor mechanics are textbook serde and an agent can re-read them on demand from the file. The `mod tests` at lines 205–397 is large and adds little. **Cut entirely**, but keep the file name (T1.1).

- **`src/kv/value.rs:601-1049` (the `value-bag`-backed and dependency-free `inner` modules).** ~3500 tokens of two parallel `Inner<'v>` enum implementations, one delegating to the `value-bag` crate and one a hand-rolled enum. Implementation detail behind the `value-bag` feature. The public `Value` API (T3.3/T3.4/T3.5/T3.6) is what consumers touch. **Cut entirely.**

- **`src/kv/value.rs:1051-1173` (deprecated `kv_unstable` shim methods and `as_debug!`/`as_display!`/`as_error!`/`as_serde!`/`as_sval!` macros).** All `#[deprecated]` with messages pointing to the `key:? = value` macro syntax already covered in T2.5. **Cut.**

- **`src/kv/value.rs:1175-1396` (the `tests` mod).** Useful as worked examples but redundant with the trait definitions and conversion macros. **Cut**, mention by line range only.

- **`src/kv/source.rs:89-432` (blanket impls for tuples, slices, arrays, Option, Box, Arc, Rc, Vec, HashMap, BTreeMap, plus VisitSource impls for `fmt::Debug{Map,List,Set,Tuple}`).** ~2000 tokens of mechanical glue. Each is a textbook "delegate to the inner type" pattern. **Cut**, but worth a one-line summary in T3.2: "Source is implemented for `(K, V)`, `[S]`, `[S; N]`, `Option<S>`, `Box<S>`, `Arc<S>`, `Rc<S>`, `Vec<S>`, `HashMap`, `BTreeMap`."

- **`src/kv/source.rs:464-515` and `source.rs:408-457` (`#[cfg(test)] mod tests`).** Behavioral coverage; redundant with the trait docs. **Cut.**

- **`src/kv/key.rs:92-149` (std-feature `String`/`Cow` impls + `kv_sval`/`kv_serde` impls).** Mechanical; the existence of these impls is implied by T1.3 features + T3.1 trait. **Cut.**

- **`src/kv/error.rs:46-94` (`std_support` mod with `Error::boxed`, `error::Error` impl, `From<io::Error>`).** The `Error::msg` constructor and `Display` impl in lines 1–66 are enough for an agent to use the type. The `boxed` constructor is mentioned in the doc anyway. **Cut**, but mention the file by name in T2.6's re-export list.

- **`benches/value.rs` (155 tokens, 27 lines, requires nightly `#![feature(test)]`).** Three or four micro-benches for `Value::from`/`from_debug`. Tiny but only useful if performance work is the task. **Cut entirely**, keep the file name visible.

- **`tests/macros.rs:23-430` (~4300 tokens of macro-permutation tests).** Exhaustive enumeration of every macro × every argument shape × kv vs. non-kv. As copy-paste fodder it's useful but bulky; the patterns are obvious from the macro doc examples (T1.6). **Cut the body**, keep the file name visible. The first ~22 lines are kept as T5.1.

- **`tests/integration.rs:24-101` (~630 tokens).** Test body for the level-filter behavior. Behavioral spec, but redundant with the `Level <= LevelFilter` semantics already conveyed by the enum docs (T1.4) and `Log::enabled` doc (T1.5). **Cut**, keep the file name.

- **`test_max_level_features/main.rs:43-76` (the `test()` and `last()` helpers).** Tests the cfg gating; useful behavior but the cargo feature combination in `test_max_level_features/Cargo.toml` (T5.2) is the load-bearing part. **Cut.**

- **`.github/workflows/main.yml:46-118` (check/doc/features/minimalv/msrv/embedded jobs).** Each is two or three `cargo` invocations. Useful as a lookup for "how do I reproduce CI" but mostly mechanical. **Cut**, keep the job names visible by including the first ~45 lines (T5.3).

- **`Cargo.toml:56-77` (the `[dependencies]`/`[dev-dependencies]` blocks plus the `proc-macro2` MSRV-pinning workaround).** 321 tokens. The dependency *names* (`serde_core`, `sval`, `sval_ref`, `value-bag`) matter and could be a 30-token line-prefix batch, but the version specs and the long comment about proc-macro2 are noise for typical tasks. **Demote** — would be the next thing to add if budget were ~22k instead of ~20k.
