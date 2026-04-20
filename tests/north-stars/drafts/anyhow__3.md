# anyhow — North Star

Revision pin: `769cba0b`

The library is an error-handling crate built around a single concrete `anyhow::Error` trait-object error type with `Result<T, E = Error>` alias and a `Context` extension trait, plus three macros (`anyhow!`, `bail!`, `ensure!`). Implementation is tightly packed into 12 src files behind a custom thin-pointer `Own<ErrorImpl>` representation with a hand-rolled vtable. Many `cfg`-gated knobs (std/no_std, nightly `error_generic_member_access`, optional `backtrace` crate fallback) shape what compiles where. The 16-file `tests/` tree is the primary executable specification.

## Batches

### 1.1 fixture-root entries
- Content: top-level listing rendered as `src/`, `tests/`, `build.rs`, `Cargo.toml`, `README.md`, `LICENSE-APACHE`, `LICENSE-MIT`, `rust-toolchain.toml`, `.github/`, `.gitignore`
- Cost: 34 tokens (helper: `printf '...' | scripts/count-tokens.py --stdin`)
- Notes: cheapest possible orientation; tells the agent there's no workspace, no benches, no examples, no scripts.

### 1.2 `src/` file listing
- Content: 12 file names in `tests/fixtures/anyhow/src/` (`lib.rs`, `error.rs`, `macros.rs`, `context.rs`, `chain.rs`, `ensure.rs`, `fmt.rs`, `kind.rs`, `wrapper.rs`, `ptr.rs`, `backtrace.rs`, `nightly.rs`)
- Cost: 39 tokens (helper: `printf 'lib.rs\n...' | scripts/count-tokens.py --stdin`)
- Notes: critical map of the implementation. Every public item lives in one of these files.

### 1.3 `tests/` file listing
- Content: 14 `test_*.rs` plus `compiletest.rs`, `common/`, `drop/`, `crate/`, `ui/`
- Cost: 72 tokens (helper: `printf '...' | scripts/count-tokens.py --stdin`)
- Notes: the test names alone (`test_downcast`, `test_chain`, `test_fmt`, `test_ensure`, `test_boxed`, `test_ffi`, `test_autotrait`, `test_repr`, …) are the best one-glance summary of the library's surface area.

### 1.4 README headline + install snippet
- Content: `README.md:9-17` (the "trait-object based error type for easy idiomatic error handling" sentence and the `[dependencies] anyhow = "1.0"` snippet)
- Cost: ~60 tokens (helper: `sed -n '9,17p' tests/fixtures/anyhow/README.md | scripts/count-tokens.py --stdin`); upper-bounded by 412 from `README.md:1-30` (which includes badges).
- Notes: tightest possible "what is this crate" answer.

### 1.5 crate-root surface (signature group)
- Content: hand-assembled signature group from `src/lib.rs`, ~8 lines: `pub struct Error` (line 390), `pub struct Chain<'a>` (line 415), `pub type Result<T, E = Error> = core::result::Result<T, E>;` (line 468), `pub trait Context<T, E>: context::private::Sealed` (line 616), `pub fn Ok<T>(value: T) -> Result<T>` (line 650), `pub use anyhow as format_err;` (line 286), plus the macro names `anyhow!`, `bail!`, `ensure!`.
- Cost: ~50–80 tokens (helper: `printf 'pub struct Error\npub struct Chain<...>\npub type Result<T, E = Error>\npub trait Context<T, E>\npub fn Ok<T>(value: T) -> Result<T>\npub use anyhow as format_err;\nmacros: anyhow!, bail!, ensure!\n' | scripts/count-tokens.py --stdin`)
- Notes: highest information density on the crate's surface — names and macros in one batch.

### 1.6 `Context` trait body
- Content: `src/lib.rs:616-628` (the trait declaration with `fn context` and `fn with_context` signatures and bounds)
- Cost: 125 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:616-628`)
- Notes: one of the two extension points the user actually touches; tiny and vital.

### 1.7 `bail!` macro definition
- Content: `src/macros.rs:56-68` (the three-arm `macro_rules! bail`)
- Cost: 125 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:56-68`)
- Notes: one-line shorthand for `return Err(anyhow!(...))`. Cheap and high-value.

### 1.8 `anyhow!` macro definition
- Content: `src/macros.rs:202-223` (the `#[macro_export] macro_rules! anyhow` rule with all three arms: literal, expr, fmt)
- Cost: 173 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:202-223`)
- Notes: covers the dispatch into `format_err` / `anyhow_kind().new(error)` / `Error::msg(format!(...))`. Many implementation questions chain into kind.rs from here.

### 1.9 `Result` alias (definition + intro)
- Content: `src/lib.rs:419-437` (rustdoc summary "`Result<T, Error>` … reasonable return type to use throughout your application but also for `fn main`") plus `src/lib.rs:467-468` (the `pub type Result<T, E = Error> = core::result::Result<T, E>;` line)
- Cost: 178 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:419-437 tests/fixtures/anyhow/src/lib.rs:467-468`)
- Notes: minimum-viable answer to "what is `anyhow::Result`?". Predecessor-free even though Context trait references it.

### 1.10 `ensure!` macro readable arms (`#[cfg(doc)]` variant)
- Content: `src/macros.rs:126-153` (the `#[cfg(doc)]` arm shown in rustdoc with three readable arms — `ensure!($cond)`, `ensure!($cond, $msg)`, `ensure!($cond, $err)`, `ensure!($cond, $fmt, ...)`).
- Cost: 217 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:126-153`)
- Notes: this is the human-readable signature; the actual `#[cfg(not(doc))]` impl trampolines into `__parse_ensure!` (deferred).

### 2.1 Cargo manifest (full)
- Content: `Cargo.toml:1-41`
- Cost: 378 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/Cargo.toml`)
- Notes: shows crate version (1.0.101), MSRV 1.68, `no-std` category, `default = ["std"]` and optional `backtrace` feature, dev-deps (`thiserror`, `trybuild`, `rustversion`, `syn`, `futures`) — all load-bearing for understanding what code is gated and what tests need.

### 2.2 crate-level attributes and module declarations
- Content: `src/lib.rs:209-263` (the `#![doc(html_root_url ...)]`, `#![no_std]`, `#![deny(...)]`, `#![allow(clippy::...)]` stack, plus `#[macro_use] mod backtrace;` and the rest of the `mod` declarations)
- Cost: 378 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:209-263`)
- Notes: tells the agent the crate is `no_std`, what lints are silenced (impacts what changes are accepted), and which modules exist in the same file order as the `src/` listing.

### 2.3 `Error` constructor signatures
- Content: line-prefix-truncated render of `src/error.rs` constructors: `pub fn new<E>(error: E) -> Self where E: StdError + Send + Sync + 'static` (line 30), `pub fn msg<M>(message: M) -> Self where M: Display + Debug + Send + Sync + 'static` (line 77), `pub fn from_boxed(boxed_error: Box<dyn StdError + Send + Sync + 'static>) -> Self` (line 140). Use `--regex '^[^{]*'` to drop bodies and keep the bound clauses.
- Cost: ~70 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:30-33 tests/fixtures/anyhow/src/error.rs:77-80 tests/fixtures/anyhow/src/error.rs:140-140 --regex '^[^{]*'`)
- Notes: collapses three constructor entry-points into one cheap signature group. The `Send + Sync + 'static` bound recurs across the API.

### 2.4 `Error` consumer-facing method signatures
- Content: line-prefix-truncated signatures from `error.rs` for `pub fn context<C>(self, context: C) -> Self` (line 372), `pub fn backtrace(&self) -> &impl_backtrace!()` (line 432), `pub fn chain(&self) -> Chain` (line 459), `pub fn root_cause(&self) -> &(dyn StdError + 'static)` (line 470), `pub fn is<E>(&self) -> bool` (line 482), `pub fn downcast<E>(mut self) -> Result<E, Self>` (line 490), `pub fn downcast_ref<E>(&self) -> Option<&E>` (line 554), `pub fn downcast_mut<E>(&mut self) -> Option<&mut E>` (line 568), `pub fn into_boxed_dyn_error(self) -> Box<dyn StdError + Send + Sync + 'static>` (line 622), `pub fn reallocate_into_boxed_dyn_error_without_backtrace(self) -> Box<...>` (line 663).
- Cost: ~150 tokens (helper: combine the targeted single-line signatures with `--regex '^[^{]*'`, e.g. `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:372-372 tests/fixtures/anyhow/src/error.rs:432-432 tests/fixtures/anyhow/src/error.rs:459-459 tests/fixtures/anyhow/src/error.rs:470-470 tests/fixtures/anyhow/src/error.rs:482-482 tests/fixtures/anyhow/src/error.rs:490-490 tests/fixtures/anyhow/src/error.rs:554-554 tests/fixtures/anyhow/src/error.rs:568-568 tests/fixtures/anyhow/src/error.rs:622-622 tests/fixtures/anyhow/src/error.rs:663-665`)
- Notes: this is the most-asked surface — "how do I downcast / chain / get a backtrace from an anyhow::Error?" Hits all of it in one batch.

### 2.5 doc rendering of Error display formats
- Content: `src/lib.rs:300-368` (the rustdoc on `Error` showing `{}`, `{:#}`, `{:?}` and `{:#?}` printed examples, including the "Caused by:" / "Stack backtrace:" layout)
- Cost: 585 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:300-368`)
- Notes: most user questions about anyhow output ("how do I print causes?") are answered here without touching `fmt.rs`.

### 2.6 README "Details" walkthrough
- Content: `README.md:31-93` (the bulleted Details section: `?` propagation, `.context(...)` / `.with_context(...)`, the printed Caused-by example)
- Cost: 573 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/README.md:31-93`)
- Notes: the canonical 60-second tour of using anyhow. Worth carrying full because it ties together `Result`, `Context`, and printed output — the most common first questions.

### 2.7 `tests/test_repr.rs` representation invariants
- Content: `tests/fixtures/anyhow/tests/test_repr.rs:1-30`
- Cost: 169 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_repr.rs`)
- Notes: asserts `mem::size_of::<Error>() == size_of::<usize>()` and `Result<(), Error>` is also one word. The single most important architectural invariant of the crate, asserted in code.

### 2.8 `tests/test_autotrait.rs` (full)
- Content: `tests/fixtures/anyhow/tests/test_autotrait.rs:1-34`
- Cost: 174 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_autotrait.rs`)
- Notes: enumerates the auto-trait guarantees on `Error`: `Send`, `Sync`, `UnwindSafe`, `RefUnwindSafe`, `Unpin`. Important and not visible from a function signature.

### 2.9 `tests/test_ffi.rs` (full)
- Content: `tests/fixtures/anyhow/tests/test_ffi.rs:1-19`
- Cost: 126 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_ffi.rs`)
- Notes: shows `anyhow::Error` and `Option<anyhow::Error>` are FFI-safe — pairs with 2.7 (one-word representation).

### 2.10 README — Comparison + No-std + License sections
- Content: `README.md:124-180` (No-std support, Comparison to failure, Comparison to thiserror, License footer)
- Cost: 457 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/README.md:124-180`)
- Notes: useful for "should I use anyhow vs thiserror?" type queries; positions the crate.

### 2.11 lib.rs no-std support docs
- Content: `src/lib.rs:191-208` (the `# No-std support` rustdoc paragraph including the older-Rust caveat about `.map_err(Error::msg)`)
- Cost: 174 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:191-208`)

### 2.12 `tests/test_backtrace.rs` (full)
- Content: `tests/fixtures/anyhow/tests/test_backtrace.rs:1-15`
- Cost: 77 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_backtrace.rs`)
- Notes: tells the agent backtrace tests are nightly-gated.

### 2.13 `tests/common/mod.rs` shared bail helpers
- Content: `tests/fixtures/anyhow/tests/common/mod.rs:1-15` (the `bail_literal`/`bail_fmt`/`bail_error` test fixtures)
- Cost: 81 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/common/mod.rs`)

### 3.1 `Context` impls for `Result` and `Option`
- Content: `src/context.rs:42-113` (the two `impl<T, E> Context<T, E> for Result<T, E>` bodies and the `impl<T> Context<T, Infallible> for Option<T>` bodies)
- Predecessor: 1.6
- Cost: 485 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/context.rs:42-68 tests/fixtures/anyhow/src/context.rs:70-113`)
- Notes: covers the pattern that `.context(...)` works on `Result` and `Option`, with the inner-trait-dispatch trick. The "Not using map_err to save 2 useless frames" comment is a load-bearing rationale.

### 3.2 chain.rs full contents
- Content: `src/chain.rs:1-102` (the entire `Chain<'a>` iterator)
- Cost: 654 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/chain.rs`)
- Notes: small, self-contained, and the reference for `Error::chain()` / `Error::root_cause()`. Includes both `Iterator` and `DoubleEndedIterator` impls plus the `Buffered`/`Linked` state machine.

### 3.3 fmt.rs: Display + Debug for `ErrorImpl`
- Content: `src/fmt.rs:1-67` (the `impl ErrorImpl { display + debug }` printing logic, including "Caused by:" header, the `Indented` numbering, and the backtrace-status branch)
- Cost: 502 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/fmt.rs:1-67`)
- Notes: complementary to 2.5. Together they let an agent answer almost any "why does this print like this?" question without further reads.

### 3.4 `tests/test_fmt.rs` expected output constants
- Content: `tests/fixtures/anyhow/tests/test_fmt.rs:1-67` (functions `f`/`g`/`h` plus the `EXPECTED_*` constants for `Display`, `{:#}`, `{:?}`, and `{:#?}`)
- Cost: 346 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_fmt.rs:1-67`)
- Notes: the canonical executable spec for what anyhow's formatters emit. Reading these constants is faster than reading `fmt.rs` for "what string does `{:?}` produce?" queries.

### 3.5 `tests/drop/mod.rs` drop-detection fixture
- Content: `tests/fixtures/anyhow/tests/drop/mod.rs:1-53` (the `Flag` / `DetectDrop` fixture used by all the drop-correctness tests)
- Cost: 274 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/drop/mod.rs`)
- Notes: pre-req for `test_repr.rs`, `test_context.rs`, `test_convert.rs`, `test_downcast.rs`. Showing it once unlocks all four.

### 3.6 `tests/test_chain.rs` (full)
- Content: `tests/fixtures/anyhow/tests/test_chain.rs:1-69`
- Cost: 581 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_chain.rs`)
- Notes: the `Chain` iterator's behavior is documented in tests at least as clearly as in code. Covers `next`, `next_back`, `len`, `default`, `clone`.

### 3.7 `tests/test_context.rs` chain-construction example
- Content: `tests/fixtures/anyhow/tests/test_context.rs:38-94` (the `LowLevel`/`MidLevel`/`HighLevel` `make_chain` helper)
- Predecessor: 3.5
- Cost: 315 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_context.rs:38-94`)
- Notes: shows how `Context` stacks across both `Result<T, E: StdError>` and `Result<T, anyhow::Error>` paths.

### 3.8 wrapper.rs full contents
- Content: `src/wrapper.rs:1-84`
- Cost: 569 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/wrapper.rs`)
- Notes: the three `#[repr(transparent)]` wrappers (`MessageError<M>`, `DisplayError<M>`, `BoxedError`) referenced by every `Error::construct_from_*` constructor. The transparent layout is the safety condition for vtable downcasts.

### 3.9 nightly.rs full contents
- Content: `src/nightly.rs:1-58`
- Cost: 414 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/nightly.rs`)
- Notes: tiny module that is both the build-script probe (the `#[cfg(anyhow_build_probe)] const _: () = { ... }` block) and the runtime API (`request_ref_backtrace`, `provide_ref_backtrace`, `provide`).

### 3.10 `tests/test_boxed.rs` (full)
- Content: `tests/fixtures/anyhow/tests/test_boxed.rs:1-45`
- Cost: 290 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_boxed.rs`)
- Notes: shows `anyhow!(Box::<dyn StdError>::from(...))`, `anyhow!(thiserror_struct)`, and `anyhow!(anyhow_error)` — the three `anyhow_kind` dispatch paths in action.

### 3.11 `tests/ui/` directory listing
- Content: file listing of `tests/ui/` (7 `.rs` + 7 `.stderr`)
- Cost: 70 tokens (helper: `ls tests/fixtures/anyhow/tests/ui/ | scripts/count-tokens.py --stdin`)
- Notes: directory of compile-fail tests. The names alone (`chained-comparison`, `empty-ensure`, `ensure-nonbool`, `must-use`, `no-impl`, `temporary-value`, `wrong-interpolation`) describe what's intentionally rejected.

### 3.12 macros.rs `__anyhow!` (hidden helper)
- Content: `src/macros.rs:225-245` (the hidden `__anyhow!` macro — same shape as `anyhow!` but without the `must_use` wrap)
- Cost: 179 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:225-245`)

### 4.1 build.rs feature/cfg detection (decision tree)
- Content: `build.rs:1-97` (the `fn main` body — the entire decision tree for `error_generic_member_access`, `std_backtrace`, `anyhow_no_core_error`, `anyhow_no_clippy_format_args` cfgs, plus the `compile_error!` for `backtrace` without `std`)
- Cost: ~900 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/build.rs:1-97`); upper-bound via full `build.rs` total of 1877 tokens.
- Notes: explains every `#[cfg(...)]` predicate that appears throughout `src/`. Without this, an agent reading `error.rs` will be confused by the conditional compilation.

### 4.2 kind.rs full contents
- Content: `src/kind.rs:1-122`
- Predecessor: 1.8
- Cost: 825 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/kind.rs`)
- Notes: explains the autoref-based tagged-dispatch trick used by `anyhow!($expr)`. The huge top-of-file comment is essentially the rationale doc — the only place this design is explained.

### 4.3 backtrace.rs cfg dispatch + capture-mode header
- Content: `src/backtrace.rs:1-100` — the cfg cascade choosing between `std::backtrace`, the bundled `mod capture`, or a never-type `enum Backtrace {}`, plus the `impl_backtrace!`, `backtrace!`, and `backtrace_if_absent!` macros, plus the start of the `mod capture` types (`Backtrace`, `BacktraceStatus`, `Inner`, `Capture`, `BacktraceFrame`, `BacktraceSymbol`, `BytesOrWide`)
- Cost: ~700 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/backtrace.rs:1-100`)
- Notes: explains why `Backtrace` has three different identities depending on cfg.

### 4.4 `Error::context` method body
- Content: `src/error.rs:316-402` (full method including the "primary reason to use `error.context(...)` instead of `result.context(...)` …" docstring with the parse-error example, and the body that builds a fresh vtable for `ContextError<C, Error>`)
- Predecessor: 2.4
- Cost: 744 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:316-402`)

### 4.5 ensure.rs runtime helpers
- Content: `src/ensure.rs:1-101` (the `BothDebug` / `NotBothDebug` dispatch traits that pick whether to render `(lhs vs rhs)`, plus the `Buf` no-alloc 40-byte stack buffer used to format LHS/RHS for the failure message, plus the `render` function)
- Predecessor: 1.10
- Cost: 665 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/ensure.rs:1-101`)
- Notes: the runtime side of the `ensure!` magic — explains where `Condition failed: \`v + v == 1\` (2 vs 1)` comes from.

### 4.6 `__fancy_ensure!` and `__fallback_ensure!` macros
- Content: `src/ensure.rs:884-935`
- Predecessor: 1.10
- Cost: 409 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/ensure.rs:884-935`)
- Notes: the two output-side helper macros that the parser dispatches into. Self-contained; the giant `__parse_ensure!` table in between is below-the-fold.

### 4.7 lib.rs `__private` module
- Content: `src/lib.rs:654-728`
- Cost: 410 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:654-728`)
- Notes: the `pub mod __private` referenced by every macro expansion. Useful for explaining why the macros work.

### 4.8 CI matrix (rust toolchains tested)
- Content: `.github/workflows/ci.yml:19-44` (the test job: rust matrix `nightly, beta, stable, 1.82.0, 1.80.0, 1.76.0`, plus the `cargo test` / `cargo check --no-default-features` / `cargo check --features backtrace` invocations)
- Cost: ~280 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/.github/workflows/ci.yml:19-44`)
- Notes: the MSRV/test matrix complements `Cargo.toml`'s `rust-version = "1.68"`.

### 4.9 `tests/crate/` no-std smoke test
- Content: `tests/fixtures/anyhow/tests/crate/Cargo.toml:1-18` plus `tests/fixtures/anyhow/tests/crate/test.rs:1-3`
- Cost: 103 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/crate/Cargo.toml tests/fixtures/anyhow/tests/crate/test.rs`)
- Notes: this is how no-std mode is exercised in CI. Two-file batch.

### 4.10 `tests/ui/no-impl.rs` + `.stderr`
- Content: `tests/fixtures/anyhow/tests/ui/no-impl.rs:1-9` plus `tests/fixtures/anyhow/tests/ui/no-impl.stderr:1-33`
- Predecessor: 4.2
- Cost: ~250 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/ui/no-impl.rs tests/fixtures/anyhow/tests/ui/no-impl.stderr`)
- Notes: a single high-signal example of the trait-bound error a user gets when passing a non-`Display` type to `anyhow!`. Picked over the others because it surfaces the `AdhocKind`/`TraitKind`/`BoxedKind` dispatch.

### 5.1 `ErrorImpl` representation + vtable struct
- Content: `src/error.rs:740-754` (the `struct ErrorVTable { object_drop, object_ref, object_boxed, object_reallocate_boxed, object_downcast, object_drop_rest, [object_backtrace] }`) plus `src/error.rs:932-955` (the `#[repr(C)] pub(crate) struct ErrorImpl<E = ()>`, the `vtable()` reader, and `pub(crate) struct ContextError<C, E>`)
- Cost: 458 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:740-754 tests/fixtures/anyhow/src/error.rs:932-955`)
- Notes: the data-layout core of the library. Without this, the unsafe code in 5.x is unreadable.

### 5.2 `Error::downcast` family bodies
- Content: `src/error.rs:482-580` (the `is`, `downcast`, `downcast_ref`, `downcast_mut` method bodies with their vtable lookups and the `ManuallyDrop` dance for owning extraction)
- Predecessor: 5.1
- Cost: ~530 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:482-487 tests/fixtures/anyhow/src/error.rs:489-516 tests/fixtures/anyhow/src/error.rs:554-565 tests/fixtures/anyhow/src/error.rs:567-580` ≈ 525)
- Notes: the actual implementation behind the most-used non-construction methods.

### 5.3 ptr.rs full contents
- Content: `src/ptr.rs:1-187`
- Cost: 956 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/ptr.rs`)
- Notes: the `Own<T>` / `Ref<'a, T>` / `Mut<'a, T>` thin-pointer abstractions referenced everywhere in `error.rs`. Late because all uses are explained by their call sites.

### 5.4 `Error::into_boxed_dyn_error` + `reallocate_into_boxed_dyn_error_without_backtrace` bodies
- Content: `src/error.rs:582-672`
- Predecessor: 5.1
- Cost: 1000 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:582-672`)
- Notes: contains the rustdoc with the `Backtrace` provider-API example, plus both method bodies. The "lose-backtrace vs lose-downcast" tradeoff lives here.

### 5.5 `Error::construct` and the per-vtable `construct_from_*` builders
- Content: `src/error.rs:145-256` (`construct_from_std`, `construct_from_adhoc`, `construct_from_display`, `construct_from_context`) plus `src/error.rs:258-314` (`construct_from_boxed` and the unsafe `construct<E>(...)` core)
- Predecessor: 5.1
- Cost: 1640 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:145-256 tests/fixtures/anyhow/src/error.rs:258-285 tests/fixtures/anyhow/src/error.rs:287-314`)
- Notes: each `construct_from_*` mounts a vtable that knows the original `E`; this is the unsafe heart of the thin-pointer design. Heavy but cohesive — splitting would obscure the pattern.

### 5.6 `From<E>` for Error + `Deref` + `Display`/`Debug`/`Drop` impls
- Content: `src/error.rs:691-738`
- Predecessor: 5.1
- Cost: 334 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:691-738`)
- Notes: where `?` conversion lives (`From<E>`); also the `Drop` hook into the vtable.

### 5.7 `From<Error> for Box<dyn StdError + ...>` + `AsRef`
- Content: `src/error.rs:1047-1086`
- Predecessor: 5.1
- Cost: 304 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:1047-1086`)
- Notes: rounds out the std-interop story for anyhow → Box conversions plus `UnwindSafe` / `RefUnwindSafe`.

## Below-the-fold

- `src/error.rs:756-928` — vtable `object_*` and `context_*` free functions (~1,742 tokens): the actual implementations behind the function pointers in `ErrorVTable` (5.1) and the constructors (5.5). Once the agent has the vtable shape and the constructor pattern, these bodies are mechanical type-erased forwarding code; demand-fetch when a downcast bug needs investigation.
- `src/error.rs:957-1045` — `ErrorImpl` private methods (`erase`, `error`, `error_mut`, `backtrace`, `provide`, `chain`) and the `StdError`/`Display`/`Debug` impls on `ErrorImpl<E>` (~728 tokens): glue between the vtable and the public methods; covered semantically by 5.1 + 3.3.
- `src/context.rs:9-40, 115-150, 166-184, 186-193` — `mod ext` private trait, `ContextError` trait impls, `Quoted` Debug helper, `Sealed` trait (~613 tokens): supporting types whose existence is implied by 3.1 and 4.4; rarely the answer to a user question.
- `src/ensure.rs:103-883` (the `__parse_ensure!` token-tree state machine): ~18,400 tokens. Vast TT-munching macro that recursively classifies expression syntax to decide whether the `ensure!` arm is `__fancy_ensure!` or `__fallback_ensure!`. Real-world value to readers is near-zero — nobody hand-edits this and the docs in 1.10 + 4.5 + 4.6 cover its contract. If needed, the agent can `Read tests/fixtures/anyhow/src/ensure.rs:103-883`.
- `src/backtrace.rs:101-410` — the `mod capture` `Debug`/`Display` impls and `LazilyResolvedCapture` machinery used only when the optional `backtrace` crate is enabled on pre-1.65 Rust (~2,100 tokens): niche fallback path; 4.3 already shows the cfg cascade and the type names.
- `tests/test_ensure.rs:1-756`: 7,285 tokens of edge-case parser regression tests (`test_low_precedence_control_flow`, `test_unary`, `test_path`, `test_pat`, etc.). All of it is "this exact source string must produce this exact error message"; the file name in 1.3 plus 4.5/4.6 give the agent enough to read individual cases on demand.
- `tests/test_downcast.rs`, `tests/test_convert.rs`, `tests/test_macros.rs`, `tests/test_source.rs`: covered semantically by 2.13 + 3.5 + 3.10 + the README walk-throughs and lib.rs doctests. Their names in 1.3 are sufficient pointers.
- `LICENSE-APACHE`, `LICENSE-MIT`, `.gitignore`, `.github/FUNDING.yml`, `rust-toolchain.toml`: standard or two-line files; the agent never needs them in context. `rust-toolchain.toml` only adds `components = ["rust-src"]`, mentioned in 4.8.
- `build.rs:99-208` (the `compile_probe` / `rustc_minor_version` / `cargo_env_var` helpers): ~1,000 tokens of utility code; 4.1 already shows the decision tree that uses these helpers.
- `src/error.rs:1-17` (the `use` block at file head): the imports list is recoverable from the items it brings in.
- The doc-only re-export `pub use anyhow as format_err;` and the no-std `trait StdError` shim at `lib.rs:269-283`: covered briefly in 1.5 / 2.2.
- `tests/ui/*` files other than `no-impl.{rs,stderr}`: 6 more compile-fail cases listed in 3.11; one example (4.10) is enough to convey what UI tests look like.
- `tests/compiletest.rs`: 7-line trybuild driver; subsumed by 3.11.

## Cumulative cost approximation

Group totals (sum of batch costs in the group):
- Group 1 (1.1–1.10): ~1,100 tokens
- Group 2 (2.1–2.13): ~3,400 tokens (cumulative ~4,500)
- Group 3 (3.1–3.12): ~4,700 tokens (cumulative ~9,200)
- Group 4 (4.1–4.10): ~5,300 tokens (cumulative ~14,500)
- Group 5 (5.1–5.7): ~5,200 tokens (cumulative ~19,700)

Each group is roughly a doubling of the prior group's cumulative size, matching a logarithmic budget distribution. Total ranked content sits just under the 20k cap, with the heaviest pieces (5.4 = 1k, 5.5 = 1.6k) at the tail where any budget that admits them can also admit their predecessors (5.1).
