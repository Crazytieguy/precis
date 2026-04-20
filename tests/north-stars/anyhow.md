# anyhow — North Star

Revision pin: `769cba0b`

The fixture is the `anyhow` Rust crate (v1.0.101): a flexible concrete `Error` type for application-style error handling, built around `std::error::Error`. Source is a single `[lib]` package (no workspace). The crate is small (~5k LOC across 12 files in `src/`), compiles `no_std` with optional `std`/`backtrace` features, and ships a build-script probe (`build.rs` + `src/nightly.rs`) for nightly `error_generic_member_access` support. Tests are integration-style under `tests/`, plus a `trybuild` UI suite in `tests/ui/`.

The dominant cost outlier is `src/ensure.rs` (20 340 tokens), almost all of it the `__parse_ensure!` TT-muncher macro that hand-parses Rust expression grammar so the user-facing `ensure!` can render `lhs vs rhs` mismatch messages while preserving Rust's true precedence. That huge body is below-the-fold; the runtime helpers and macro-expansion targets are kept in the main ranking because they're what the `ensure!` macro actually expands to.

## Batches

### 1.1 Top-level repository listing
- Content: rendered listing of the top-level entries `.github/`, `src/`, `tests/`, `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`, `build.rs`, `rust-toolchain.toml`
- Cost: 31 tokens (helper: `echo ".github/\nsrc/\ntests/\nCargo.toml\nLICENSE-APACHE\nLICENSE-MIT\nREADME.md\nbuild.rs\nrust-toolchain.toml" | count-tokens.py --stdin`)
- Notes: orienting batch — tells the agent this is a single Rust crate (not a workspace), there's a custom build script, and tests live in two places.

### 1.2 `src/` file listing
- Content: rendered listing of the 12 files under `src/`: `backtrace.rs`, `chain.rs`, `context.rs`, `ensure.rs`, `error.rs`, `fmt.rs`, `kind.rs`, `lib.rs`, `macros.rs`, `nightly.rs`, `ptr.rs`, `wrapper.rs`
- Cost: 39 tokens (helper: `ls test/fixtures/anyhow/src | count-tokens.py --stdin`)
- Notes: module names alone reveal almost the entire architecture — `error`/`context`/`chain`/`backtrace`/`fmt`/`macros` map straight onto the public concepts.

### 1.3 `tests/` listing
- Content: rendered listing of `tests/`: subdirs `common/`, `crate/`, `drop/`, `ui/` plus integration test files `compiletest.rs`, `test_autotrait.rs`, `test_backtrace.rs`, `test_boxed.rs`, `test_chain.rs`, `test_context.rs`, `test_convert.rs`, `test_downcast.rs`, `test_ensure.rs`, `test_ffi.rs`, `test_fmt.rs`, `test_macros.rs`, `test_repr.rs`, `test_source.rs`
- Cost: 72 tokens (helper: `echo "common/\ncrate/\ndrop/\nui/\ncompiletest.rs\ntest_autotrait.rs\ntest_backtrace.rs\ntest_boxed.rs\ntest_chain.rs\ntest_context.rs\ntest_convert.rs\ntest_downcast.rs\ntest_ensure.rs\ntest_ffi.rs\ntest_fmt.rs\ntest_macros.rs\ntest_repr.rs\ntest_source.rs" | count-tokens.py --stdin`)
- Notes: filenames map 1:1 to behaviours under test (chain, context, downcast, fmt, repr, source, …); cheap and high-value for "is there a test for X?" queries.

### 1.4 Crate identity from `Cargo.toml`
- Content: `Cargo.toml:1-16` (the `[package]` table — name, version, authors, categories, description "Flexible concrete Error type built on std::error::Error", documentation URL, edition `2021`, keywords, license `MIT OR Apache-2.0`, repository, `rust-version = "1.68"` — and the `[features]` block)
- Cost: 130 tokens (helper: `count-tokens.py Cargo.toml:1-16`)
- Notes: the description, MSRV, and `default = ["std"]` / optional `std` feature are the highest-leverage facts in the whole crate — they answer "what is this", "how old can my compiler be", and "is this no_std" all at once.

### 1.5 Crate one-line summary
- Content: `src/lib.rs:9-11` (the rustdoc paragraph "This library provides `anyhow::Error`, a trait object based error type for easy idiomatic error handling in Rust applications.")
- Cost: 32 tokens (helper: `count-tokens.py src/lib.rs:9-11`)

### 1.6 `lib.rs` module declarations and `extern crate` setup
- Content: `src/lib.rs:246-263` — `extern crate alloc;`, conditional `extern crate std;`, and the twelve `mod` declarations (with `#[cfg(error_generic_member_access)] mod nightly;`). Establishes which submodules exist and that one is nightly-only.
- Cost: 60 tokens (helper: `count-tokens.py src/lib.rs:246-263`)

### 1.7 README headline + idiomatic-usage bullets
- Content: `README.md:21-123` — the "## Details" section's six bullets that show, with a runnable example each, the canonical use of `Result<T, anyhow::Error>` + `?`, `Context`/`with_context`, `downcast_ref`, backtrace env vars, `thiserror` interop, and `anyhow!`/`bail!` macros. This is the single most concentrated piece of "how to use the crate" content in the repo.
- Cost: 816 tokens (helper: `count-tokens.py README.md:21-123`)
- Notes: covers Priority-3 (zero-tool answers) for the most common "how do I use anyhow" queries.

## Batches

### 2.1 Top-level public re-exports and items in `lib.rs`
- Content: `src/lib.rs:285-300` (`pub use anyhow as format_err;` and the `pub struct Error` declaration with its short doc), `src/lib.rs:413-417` (`pub struct Chain<'a>`), `src/lib.rs:466-468` (`pub type Result<T, E = Error> = …`), `src/lib.rs:613-628` (`pub trait Context<T, E>` with both methods), `src/lib.rs:648-652` (`pub fn Ok<T>(value: T) -> Result<T>`)
- Cost: 363 tokens (helper: `count-tokens.py src/lib.rs:285-300 src/lib.rs:413-417 src/lib.rs:466-468 src/lib.rs:613-628 src/lib.rs:648-652`)
- Notes: declares the crate's entire public type/trait surface in one batch; everything else is methods on these.

### 2.2 Macro signatures with first-line docs
- Content: `src/macros.rs:1-12` (doc + `#[macro_export] macro_rules! bail`), `src/macros.rs:174-186` (doc + `macro_rules! anyhow`), `src/macros.rs:70-82` (the doc preamble of `__ensure!` that documents the public `ensure!` macro). Establishes the three macros' one-line semantics without their bodies.
- Cost: 380 tokens (helper: `count-tokens.py src/macros.rs:1-12 src/macros.rs:174-186 src/macros.rs:70-82`)
- Notes: catastrophic-omission guard — without this, the agent might not realise `bail!`/`ensure!`/`anyhow!` exist as separate items.

### 2.3 `Error` inherent-method signatures (cfg-attrs included, bodies elided)
- Content: signature blocks for every `pub fn` on `impl Error` — `src/error.rs:30-36`, `:75-82`, `:137-143`, `:370-380`, `:430-435`, `:457-461`, `:468-472`, `:482-487`, `:489-493`, `:553-557`, `:567-571`, `:620-625`, `:661-666` (these correspond to `new`, `msg`, `from_boxed`, `context`, `backtrace`, `chain`, `root_cause`, `is`, `downcast`, `downcast_ref`, `downcast_mut`, `into_boxed_dyn_error`, `reallocate_into_boxed_dyn_error_without_backtrace`)
- Cost: 741 tokens (helper: `count-tokens.py src/error.rs:30-36 src/error.rs:75-82 src/error.rs:137-143 src/error.rs:370-380 src/error.rs:430-435 src/error.rs:457-461 src/error.rs:468-472 src/error.rs:482-487 src/error.rs:489-493 src/error.rs:553-557 src/error.rs:567-571 src/error.rs:620-625 src/error.rs:661-666`)
- Notes: each block carries its `#[cfg(...)]` attribute which itself encodes the std/no-core-error feature gating — important for "is X available in no_std?" questions.

### 2.4 README "No-std support" + comparison sections
- Content: `README.md:124-162` — the "## No-std support", "## Comparison to failure", and "## Comparison to thiserror" sections.
- Cost: 351 tokens (helper: `count-tokens.py README.md:124-162`)
- Notes: clarifies positioning vs `thiserror` (key for "should I use anyhow or thiserror?") and the `no_std` mode caveats (`Error::msg` workaround on Rust < 1.81).

## Batches

### 3.1 `Error` doc on Display/Debug formats and example
- Content: `src/lib.rs:288-388` — the rustdoc on `pub struct Error` covering the four format variants (`{}`, `{:#}`, `{:?}`, `{:#?}`), the sample `Caused by:` and stack-backtrace renderings, and the manual cause-chain printing example. This is the canonical reference for what error printing looks like.
- Cost: 812 tokens (helper: `count-tokens.py src/lib.rs:288-388`)

### 3.2 `Context` trait full doc (effect on downcasting, two patterns)
- Content: `src/lib.rs:470-628` — the full rustdoc on `pub trait Context`, including the `ImportantThing` example, the printed-error sample, and the two "effect on downcasting" subsections (insignificant context onto a downcast-able error; downcast-able context onto an insignificant error).
- Cost: 1 157 tokens (helper: `count-tokens.py src/lib.rs:470-628`)
- Notes: the *only* place that documents anyhow's downcast-after-context invariant — high catastrophic-omission risk if dropped.

### 3.3 `Error::context` inherent-method doc + signature
- Content: `src/error.rs:316-380` — the long rustdoc explaining when to use `Error::context` rather than the `Context` trait extension (when the context depends on data inside the error), with the `ParseError { line, column }` example, plus the method signature.
- Cost: 519 tokens (helper: `count-tokens.py src/error.rs:316-380`)

### 3.4 `Result<T, Error>` type-alias doc and example
- Content: `src/lib.rs:419-468` — the rustdoc on `pub type Result` showing `fn main() -> Result<()>` usage and the one/two type-parameter forms.
- Cost: 353 tokens (helper: `count-tokens.py src/lib.rs:419-468`)

### 3.5 Doc + signature for `Error::new`, `Error::msg`, `Error::from_boxed`
- Content: `src/error.rs:19-83` (`new` + `msg` with the `futures` stream example) and `src/error.rs:84-143` (`from_boxed` with the bidirectional `Report ↔ anyhow::Error` interop example).
- Cost: 1 092 tokens (helper: `count-tokens.py src/error.rs:19-83 src/error.rs:84-143`)
- Notes: covers the three entry points for constructing an `anyhow::Error` outside the macros; the `from_boxed` example is the canonical interop recipe.

## Batches

### 4.1 `chain.rs` complete
- Content: `src/chain.rs` (entire file, 102 lines) — `pub struct Chain<'a>`, `ChainState::{Linked, Buffered}`, the `Iterator` / `DoubleEndedIterator` / `ExactSizeIterator` / `Default` impls.
- Cost: 654 tokens (helper: `count-tokens.py src/chain.rs`)
- Notes: entire chain-iteration semantics — referenced from `Error::chain`, `Error::root_cause`, and `fmt::display`.

### 4.2 `fmt.rs` complete (Display/Debug for `ErrorImpl`)
- Content: `src/fmt.rs` — the `ErrorImpl::display`/`debug` methods that drive `{}`/`{:#}`/`{:?}`/`{:#?}` for `Error`, the `Indented` Writer adapter that formats `n: <msg>` cause lines, and its three unit tests.
- Cost: 979 tokens (helper: `count-tokens.py src/fmt.rs`)

### 4.3 `context.rs` `Context` impls for `Result` and `Option`
- Content: `src/context.rs:1-113` — the private `ext::StdError` trait, `impl Context for Result<T, E>`, and `impl Context for Option<T>`. Together these are everything the `.context(...)` / `.with_context(...)` calls dispatch to.
- Cost: 744 tokens (helper: `count-tokens.py src/context.rs:1-113`)

### 4.4 `macros.rs` complete (`bail!`, `ensure!` shim, `anyhow!`, `__anyhow!`)
- Content: `src/macros.rs` whole file (245 lines) — full doc + arms for the three public macros plus the `__anyhow!` doc-hidden helper.
- Cost: 1 815 tokens (helper: `count-tokens.py src/macros.rs`)
- Notes: complete macro definitions — without these the agent has only signatures and can't tell, e.g., that `bail!($expr)` routes through `__anyhow!` rather than `Error::msg`.

### 4.5 `kind.rs` tagged-dispatch comment + traits
- Content: `src/kind.rs` whole file (121 lines) — the long top-of-file comment that explains why anyhow uses autoref-method-resolution as a stand-in for specialization, and the resulting `AdhocKind` / `TraitKind` / `BoxedKind` traits with their `Adhoc` / `Trait` / `Boxed` `new` methods.
- Cost: 825 tokens (helper: `count-tokens.py src/kind.rs`)
- Notes: the top comment is uniquely valuable — there's no other place in the repo that explains the dispatch trick that makes `anyhow!($expr)` polymorphic over `Display`/`Error`/`Box<dyn Error>`.

### 4.6 `wrapper.rs` complete (`MessageError`, `DisplayError`, `BoxedError`)
- Content: `src/wrapper.rs` whole file (84 lines) — the three `repr(transparent)` newtypes that adapt `Display`-only or `Box<dyn StdError>` payloads into the `StdError` shape that `ErrorImpl` expects.
- Cost: 569 tokens (helper: `count-tokens.py src/wrapper.rs`)

### 4.7 `error.rs` core type definitions and vtable
- Content: `src/error.rs:740-754` (`struct ErrorVTable` with its eight function-pointer fields) and `src/error.rs:930-955` (`#[repr(C)] struct ErrorImpl<E = ()>` with `vtable`/`backtrace`/`_object` fields, the `unsafe fn vtable` reader, and `#[repr(C)] struct ContextError<C, E>`).
- Cost: 485 tokens (helper: `count-tokens.py src/error.rs:740-754 src/error.rs:930-955`)
- Notes: the vtable layout is the *whole* reason `Error` is one word wide; it's referenced by every `object_*` function and the answer to "how does anyhow keep `Error` thin?".

### 4.8 `error.rs` `From` / `Deref` / `Display` / `Debug` / `Drop` impls for `Error`
- Content: `src/error.rs:691-738` — the std-error `From` impl (auto-conversion via `?`), the `Deref`/`DerefMut` to `dyn StdError + Send + Sync + 'static`, the `Display`/`Debug` forwarding impls, and the `Drop` impl that calls the vtable's `object_drop`.
- Cost: 334 tokens (helper: `count-tokens.py src/error.rs:691-738`)

### 4.9 `error.rs` boxed-conversion impls
- Content: `src/error.rs:1047-1086` — the three `impl From<Error> for Box<dyn StdError + …>` blocks (variants with/without `Send`/`Sync`) and the two `AsRef<dyn StdError + …>` impls, plus the `UnwindSafe` / `RefUnwindSafe` marker impls.
- Cost: 304 tokens (helper: `count-tokens.py src/error.rs:1047-1086`)

## Batches

### 5.1 `error.rs` construction helpers and unsafe `construct`
- Content: `src/error.rs:147-256` (the four `construct_from_*` functions — `_std`, `_adhoc`, `_display`, `_context`, `_boxed` — each building an `ErrorVTable` literal for the right `E`) and `src/error.rs:287-314` (the `unsafe fn construct` that boxes `ErrorImpl<E>` and casts to `Own<ErrorImpl>`).
- Cost: 1 316 tokens (helper: `count-tokens.py src/error.rs:147-256 src/error.rs:287-314`)
- Notes: the per-call-site vtable construction is the actual "type erasure" mechanism; key for understanding how a generic `E` collapses into a one-word `Error`.

### 5.2 `ensure.rs` runtime helpers and macro expansion targets
- Content: `src/ensure.rs:1-101` (the `BothDebug`/`NotBothDebug` autoref-dispatch traits used by `__fancy_ensure!`, the fixed-size `Buf` for stringifying `lhs`/`rhs` without allocating, and `fn render` that builds the `"{msg} ({lhs} vs {rhs})"` message) and `src/ensure.rs:884-935` (`__fancy_ensure!` and `__fallback_ensure!` — the actual macros that `__parse_ensure!` ultimately calls).
- Cost: 1 074 tokens (helper: `count-tokens.py src/ensure.rs:1-101 src/ensure.rs:884-935`)
- Notes: covers what `ensure!` *expands to* without dragging in the 19k-token TT-muncher.

### 5.3 `backtrace.rs` cfg-gating macros
- Content: `src/backtrace.rs:1-68` — the `cfg`-selected `Backtrace`/`BacktraceStatus` re-exports (std vs feature `backtrace` vs neither uninhabited enum) and the four conditionally-defined macros `impl_backtrace!`, `backtrace!`, `backtrace_if_absent!` that the rest of the crate uses to keep backtrace handling pluggable.
- Cost: 401 tokens (helper: `count-tokens.py src/backtrace.rs:1-68`)
- Notes: explains the `Option<Backtrace>` field on `ErrorImpl` and why its presence depends on three different cfgs.

### 5.4 `nightly.rs` complete
- Content: `src/nightly.rs` whole file (58 lines) — the `error_generic_member_access` probe (`MyError(Backtrace)`), the `request_ref_backtrace` / `provide_ref_backtrace` / `provide` shims, and the `pub use core::error::Request` re-export.
- Cost: 414 tokens (helper: `count-tokens.py src/nightly.rs`)

### 5.5 `lib.rs` `__private` module (macro-helper plumbing)
- Content: `src/lib.rs:653-728` — the `pub mod __private` block that re-exports `BothDebug`/`NotBothDebug`, `format`, `Err`, `concat`/`format_args`/`stringify`, the `kind` sub-module, and exports `format_err`, `must_use`, `not` (with the private `Bool` trait for `&bool` autoref). All the symbols the public macros expand to.
- Cost: 411 tokens (helper: `count-tokens.py src/lib.rs:653-728`)

## Batches

### 6.1 Test-function inventory per integration test file
- Content: rendered listing of each `tests/test_*.rs` file followed by its `fn test_*` names (all 14 files, ~63 test functions).
- Cost: 436 tokens (helper: `for f in tests/test_*.rs; do echo "$f:"; grep -oE 'fn test_[a-zA-Z_]+' "$f" | sort -u; done | count-tokens.py --stdin`)
- Notes: top-of-fold for the testing slice — answers "is there a test for X" without reading any test body.

### 6.2 Test helpers (`tests/common/mod.rs` + `tests/drop/mod.rs`)
- Content: both files in full — `tests/common/mod.rs` (the `bail_literal` / `bail_fmt` / `bail_error` fixtures used by `test_macros.rs` and `test_downcast.rs`) and `tests/drop/mod.rs` (the `Flag` + `DetectDrop` instrumented `StdError` used by every drop-tracking test).
- Cost: 355 tokens (helper: `count-tokens.py tests/common/mod.rs tests/drop/mod.rs`)
- Notes: every other test file refers to these by name; without them the test bodies are partially opaque.

### 6.3 `tests/test_fmt.rs` complete
- Content: `tests/test_fmt.rs` (93 lines) — defines `f`/`g`/`h` (a context chain of three `io::Error`-rooted errors) and the `EXPECTED_ALTDISPLAY_*` / `EXPECTED_DEBUG_*` / `EXPECTED_ALTDEBUG_*` constants that pin the *exact* output strings for `{:#}`, `{:?}`, `{:#?}` on `anyhow::Error`.
- Cost: 569 tokens (helper: `count-tokens.py tests/test_fmt.rs`)
- Notes: the literal expected strings here are the ground truth for what error rendering looks like — high value for "what does an anyhow error print as?" queries.

### 6.4 `tests/test_repr.rs` complete
- Content: `tests/test_repr.rs` (30 lines) — `assert_eq!(mem::size_of::<Error>(), mem::size_of::<usize>())` and the `Result<(), Error>` null-pointer-optimization assertion, plus `Send + Sync + Unpin + 'static` autotrait check and a `Drop` test.
- Cost: 169 tokens (helper: `count-tokens.py tests/test_repr.rs`)
- Notes: pins the load-bearing invariant that `Error` is one word.

### 6.5 `tests/test_chain.rs` complete
- Content: `tests/test_chain.rs` (69 lines) — `error()` builds `anyhow!({0}).context(1).context(2).context(3)`, then iterates forward / reverse, asserts `len()` and `size_hint()` shrink correctly, and verifies `Chain::default()` and `Chain::clone()`.
- Cost: 581 tokens (helper: `count-tokens.py tests/test_chain.rs`)

### 6.6 `tests/test_source.rs` and `tests/test_boxed.rs` together
- Content: `tests/test_source.rs` (`error.source()` is `None` for ad-hoc messages, `Some` for an `anyhow!(StdErrorImpl)` and for `anyhow!(error)` re-wrapping) and `tests/test_boxed.rs` (`anyhow!` accepting a `Box<dyn StdError + Send + Sync>`, a `thiserror::Error` struct with a source, and an existing `anyhow::Error` with context).
- Cost: 658 tokens (helper: `count-tokens.py tests/test_source.rs tests/test_boxed.rs`)

### 6.7 `tests/ui/` listing + first-line preview
- Content: rendered listing pairing each of the 7 `tests/ui/*.rs` files with its first source line — `chained-comparison.rs`, `empty-ensure.rs`, `ensure-nonbool.rs`, `must-use.rs`, `no-impl.rs`, `temporary-value.rs`, `wrong-interpolation.rs`. Communicates which compile-error scenarios are pinned by trybuild.
- Cost: 104 tokens (helper: `for f in tests/ui/*.rs; do echo "$f:"; head -1 "$f"; done | count-tokens.py --stdin`)

### 6.8 `Cargo.toml` `[dev-dependencies]` and `[package.metadata.docs.rs]`
- Content: `Cargo.toml:25-40` — pins `futures` (no-default-features), `rustversion`, `syn` (`features = ["full"]`), `thiserror = "2"`, `trybuild` (with the `diff` feature) as dev-deps, and the docs.rs rustdoc args (`--generate-link-to-definition`, `--generate-macro-expansion`, three `--extern-html-root-url=…`).
- Cost: 162 tokens (helper: `count-tokens.py Cargo.toml:25-40`)

### 6.9 CI matrix and `tests/compiletest.rs`
- Content: `.github/workflows/ci.yml:1-50` (just the `pre_ci` and `test` jobs, including the rust toolchain matrix `[nightly, beta, stable, 1.82.0, 1.80.0, 1.76.0]` and the `--features backtrace` / `--no-default-features` checks) and `tests/compiletest.rs` (the trybuild driver gated to nightly + non-miri).
- Cost: 463 tokens (helper: `count-tokens.py .github/workflows/ci.yml:1-50 tests/compiletest.rs`)

## Below-the-fold

- `src/ensure.rs:104-882` — the `__parse_ensure!` TT-muncher (~19 260 tokens). It hand-implements Rust expression parsing as a recursive `macro_rules!` so the user-facing `ensure!` can extract `lhs OP rhs` while preserving the language's true precedence; the *behaviour* is captured by `__fancy_ensure!`/`__fallback_ensure!` (batch 5.2) and the `test_ensure.rs` cases below. Reading the muncher itself is rarely actionable — almost any change to it requires touching `tests/test_ensure.rs` instead.
- `src/error.rs:756-928` — the dozen `unsafe fn` vtable functions (`object_drop`, `object_drop_front`, `object_ref`, `object_boxed`, `object_reallocate_boxed`, `object_downcast`, `no_backtrace`, `context_downcast`, `context_drop_rest`, `context_chain_downcast`, `context_chain_drop_rest`, `context_backtrace`) the constructed vtables point at (~1 760 tokens). Each is mechanical given batch 4.7 + 5.1; one tool call away when the agent needs to follow a specific vtable slot.
- `src/error.rs:957-1046` — `ErrorImpl::erase`, `ErrorImpl::error`/`error_mut`/`backtrace`/`provide`/`chain` accessors, and the `StdError`/`Debug`/`Display` forwarding impls for `ErrorImpl<E>` (~730 tokens). Internal plumbing once batch 4.7 has explained the layout.
- `src/context.rs:115-194` — `impl Debug for ContextError` (with the `Quoted` newtype's `escape_debug` trick), `Display for ContextError`, two `StdError for ContextError` impls, and `mod private { trait Sealed }` (~510 tokens). The first three are reachable by reading `fmt.rs` (batch 4.2) plus the trait doc (batch 3.2); the sealed trait is mechanical.
- `src/ptr.rs` — `Own<T>` / `Ref<'a, T>` / `Mut<'a, T>` / `CastTo` (~960 tokens). Only relevant when modifying `error.rs`'s pointer manipulations; semantically obvious from how they're used in batch 5.1.
- `src/backtrace.rs:70-411` — the `mod capture` fallback `Backtrace` implementation built on the `backtrace` crate (~2 430 tokens). Only compiled when `feature = "backtrace"` is enabled *and* `std_backtrace` is not, which on Rust 1.65+ never happens. `tests/test_backtrace.rs` only exercises the std path.
- `tests/test_ensure.rs` — 7 285 tokens of macro-corner-case coverage (recursion limit, `if`/`for`/`loop`/`match` expressions, `as` casts, raw-addr operators, etc.). The fn-name inventory in batch 6.1 is enough for "is corner case X covered?"; reading the bodies is only useful when modifying `__parse_ensure!`.
- `tests/test_context.rs` — 1 007 tokens; the `LowLevel`/`MidLevel`/`HighLevel` chain plus six downcast tests pin drop-correctness across the context chain. Function names in batch 6.1 already reveal coverage; full body is one Read away when chasing a downcast bug.
- `tests/test_downcast.rs` — 791 tokens; covers `downcast` / `downcast_ref` / `downcast_mut` to `&str`, `String`, `io::Error`, plus a `repr(align(64))` payload. Same logic as `test_context.rs` — fn-name list suffices for routing.
- `tests/test_macros.rs`, `tests/test_convert.rs`, `tests/test_autotrait.rs`, `tests/test_ffi.rs` — 618 / 314 / 174 / 126 tokens. Each maps to a single concept (macro behavior, `Box<dyn StdError>` round-trip, auto-trait assertion, `extern "C"` safety) already named in batch 6.1.
- `tests/crate/Cargo.toml` + `tests/crate/test.rs` — 103 tokens; tiny driver crate used only by the CI `build` job to exercise no_std on MSRV 1.68. The `[features]` in batch 1.4 already implies it.
- `build.rs:17-97` — the cfg-emitting `main()` (~820 tokens). The output cfgs (`std_backtrace`, `error_generic_member_access`, `anyhow_no_core_error`, `anyhow_no_clippy_format_args`) are visible through their use sites in `lib.rs`/`backtrace.rs`/`nightly.rs`, so the agent can usually infer what each gate means without reading the build script. Useful mainly when changing rust-version support.
- `LICENSE-APACHE`, `LICENSE-MIT` — boilerplate licence text, no semantic content.
- `.github/workflows/ci.yml:51-150` — the `build`, `minimal`, `windows`, `doc`, `clippy`, `miri`, `outdated` jobs; the matrix in batch 6.9 conveys the rustc-version coverage and the rest follow standard `dtolnay/.github` patterns.
- `.github/FUNDING.yml` — single line `github: dtolnay`.
- `rust-toolchain.toml` — two lines requesting `rust-src`; informational.
- `build.rs:1-16` and `build.rs:99-207` — the `compile_error!` for `backtrace`-without-`std`, plus `compile_probe`/`rustc_minor_version`/`cargo_env_var` helpers. Mechanical.
- `tests/ui/*.stderr` — the trybuild expected-error outputs (~2 000 tokens combined). Identifying *which* errors are pinned (batch 6.7 file names) is more useful than the exact spans.
- `Cargo.toml:13` (single comment line about MSRV) — covered implicitly by the `rust-version` field in batch 1.4.
