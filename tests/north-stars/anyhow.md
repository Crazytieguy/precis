# anyhow — North Star

Revision pin: `769cba0b`

The fixture is a single small-but-dense Rust crate (`anyhow` 1.0.101): one
trait-object error type `anyhow::Error` (one machine word wide, backed by a
hand-rolled vtable in `Own<ErrorImpl>`), an `anyhow::Result<T, E = Error>`
alias, a `Context` extension trait, and three macros (`anyhow!`, `bail!`,
`ensure!`). Implementation is split across 12 `src/` files plus a sizable
`build.rs` that probes the toolchain and emits the cfg flags (`std_backtrace`,
`error_generic_member_access`, `anyhow_no_core_error`, `anyhow_no_clippy_format_args`,
`anyhow_nightly_testing`) referenced throughout `src/`. Two of those source
files dominate by size: `src/error.rs` (1086 lines, the unsafe vtable core)
and `src/ensure.rs` (935 lines, of which lines 103-883 are a recursive
`__parse_ensure!` token-tree parser ~19k tokens — far too big to ship). The
16-file `tests/` tree (plus a small `tests/crate/` no-std smoke crate and a
`tests/ui/` set of trybuild compile-fail cases) is the executable spec, and
several test files are the cleanest available reference for invariants
(one-word repr, auto-traits, formatter output strings, `Chain` iteration).
The ranking front-loads orientation and the user-facing surface (listings,
public types, macro shapes, README tutorial, formatting doctrine), then the
`Context` story, then the internals (vtable, kind dispatch, ensure runtime),
then the cfg/build matrix and CI, with the giant `__parse_ensure!` parser and
the bundled `mod capture` backtrace fallback below the fold.

## Batches

### 1.1 Repo-root entry listing
- Content: rendered listing of the 10 root entries — `.github/`, `.gitignore`, `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`, `build.rs`, `rust-toolchain.toml`, `src/`, `tests/`.
- Cost: 34 tokens (helper: `printf '.github/\n.gitignore\nCargo.toml\nLICENSE-APACHE\nLICENSE-MIT\nREADME.md\nbuild.rs\nrust-toolchain.toml\nsrc/\ntests/\n' | scripts/count-tokens.py --stdin`)
- Notes: cheapest possible orientation. Tells the agent there's no workspace, no benches, no examples, no scripts.

### 1.2 `src/` file listing
- Content: rendered listing of the 12 modules in `tests/fixtures/anyhow/src/` — `backtrace.rs`, `chain.rs`, `context.rs`, `ensure.rs`, `error.rs`, `fmt.rs`, `kind.rs`, `lib.rs`, `macros.rs`, `nightly.rs`, `ptr.rs`, `wrapper.rs`.
- Cost: 39 tokens (helper: `printf 'backtrace.rs\nchain.rs\ncontext.rs\nensure.rs\nerror.rs\nfmt.rs\nkind.rs\nlib.rs\nmacros.rs\nnightly.rs\nptr.rs\nwrapper.rs\n' | scripts/count-tokens.py --stdin`)
- Notes: critical map of the implementation. The names alone (`kind.rs`, `nightly.rs`, `ptr.rs`, `wrapper.rs`, `ensure.rs`) are diagnostic of structure; every public item lives in one of these files.

### 1.3 `tests/` directory listing
- Content: rendered listing of the 18 entries under `tests/fixtures/anyhow/tests/` — `common/`, `compiletest.rs`, `crate/`, `drop/`, `test_autotrait.rs`, `test_backtrace.rs`, `test_boxed.rs`, `test_chain.rs`, `test_context.rs`, `test_convert.rs`, `test_downcast.rs`, `test_ensure.rs`, `test_ffi.rs`, `test_fmt.rs`, `test_macros.rs`, `test_repr.rs`, `test_source.rs`, `ui/`.
- Cost: 72 tokens (helper: `printf 'common/\ncompiletest.rs\ncrate/\ndrop/\ntest_autotrait.rs\ntest_backtrace.rs\ntest_boxed.rs\ntest_chain.rs\ntest_context.rs\ntest_convert.rs\ntest_downcast.rs\ntest_ensure.rs\ntest_ffi.rs\ntest_fmt.rs\ntest_macros.rs\ntest_repr.rs\ntest_source.rs\nui/\n' | scripts/count-tokens.py --stdin`)
- Notes: the `test_*.rs` names (`test_chain`, `test_context`, `test_downcast`, `test_ensure`, `test_fmt`, `test_macros`, `test_repr`, `test_source`, ...) are themselves a near-perfect API map.

### 1.4 README headline + install snippet
- Content: `tests/fixtures/anyhow/README.md:9-17`.
- Cost: 68 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/README.md:9-17`)
- Notes: tightest "what is this crate" answer — one-sentence description ("trait object based error type for easy idiomatic error handling") plus the `anyhow = "1.0"` snippet. Cheap semantic anchor.

### 1.5 Cargo package identity (name/version/edition/MSRV)
- Content: `tests/fixtures/anyhow/Cargo.toml:1-13`.
- Cost: 119 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/Cargo.toml:1-13`)
- Notes: crate name "anyhow", version `1.0.101`, `categories = ["rust-patterns", "no-std"]`, `rust-version = "1.68"` MSRV, edition 2021, repository URL.

### 1.6 `pub struct Error` (declaration only)
- Content: `tests/fixtures/anyhow/src/lib.rs:389-393` — the `#[repr(transparent)] pub struct Error { inner: Own<ErrorImpl> }`. Render with a `…` elision marker standing in for the long Display-representations rustdoc above (lines 288-388).
- Cost: 18 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:389-393`)
- Notes: the central type, named cheaply. Splitting the declaration off from its rustdoc is the load-bearing move — the rustdoc is 700 tokens and ranks much later (3.1).

### 1.7 `pub type Result<T, E = Error>` (alias only)
- Content: `tests/fixtures/anyhow/src/lib.rs:467-468` — the type alias line (and its prior doc-stub line).
- Cost: 21 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:467-468`)
- Notes: minimum-viable answer to "what is `anyhow::Result`?". The "one or two type parameters" subtlety doc (lines 419-441) is below-the-fold.

### 1.8 `pub trait Context<T, E>` declaration with both methods
- Content: `tests/fixtures/anyhow/src/lib.rs:616-628` — the trait header plus the `fn context` and `fn with_context` signatures with their bounds.
- Cost: 125 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:616-628`)
- Notes: one of the two extension points users actually touch; tiny and vital. The 1k-token rustdoc above (lines 475-615) ranks later (3.4).

### 1.9 `pub fn Ok<T>` and `pub use anyhow as format_err`
- Content: `tests/fixtures/anyhow/src/lib.rs:285-286` (`pub use anyhow as format_err;`) plus `tests/fixtures/anyhow/src/lib.rs:648-652` (`pub fn Ok<T>(value: T) -> Result<T>`).
- Cost: 43 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:285-286 tests/fixtures/anyhow/src/lib.rs:648-652`)
- Notes: the two remaining top-level public items. `format_err` is the deprecated-style alias for `anyhow!`; `Ok` exists because `anyhow::Result::Ok(value)` cannot infer `E`.

### 1.10 `pub struct Chain<'a>` declaration (cfg-gated)
- Content: `tests/fixtures/anyhow/src/lib.rs:413-417` — the `#[cfg(any(feature = "std", not(anyhow_no_core_error)))] #[derive(Clone)] pub struct Chain<'a> { state: ... }`.
- Cost: 40 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:413-417`)
- Notes: the iterator type returned by `Error::chain()`. Tiny declaration; full file `chain.rs` ranks later (3.6). The cfg on the struct is the first place the agent sees the `(feature = "std", not(anyhow_no_core_error))` predicate that recurs throughout.

### 1.11 Cargo features and dependencies
- Content: `tests/fixtures/anyhow/Cargo.toml:14-30`.
- Cost: 170 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/Cargo.toml:14-30`)
- Notes: `default = ["std"]`, optional `backtrace = "0.3.51"` (with the comment that it's unused on Rust ≥ 1.65), and the dev-deps (`thiserror`, `trybuild`, `rustversion`, `syn`, `futures`). Catastrophic-omission risk if missing — agent might invent a non-existent feature gate or miss that `default-features = false` is the no-std path.

### 1.12 `bail!` macro definition
- Content: `tests/fixtures/anyhow/src/macros.rs:56-68` — the three-arm `macro_rules! bail`.
- Cost: 125 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:56-68`)
- Notes: one-line shorthand for `return Err(anyhow!(...))`. Cheap and high-value.

### 1.13 `anyhow!` macro definition
- Content: `tests/fixtures/anyhow/src/macros.rs:202-223` — `#[macro_export] macro_rules! anyhow` with all three arms (literal, expr, fmt).
- Cost: 173 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:202-223`)
- Notes: the `expr` arm shows the autoref-dispatch pattern `(&error).anyhow_kind().new(error)`, which previews `kind.rs` (4.4-4.5).

### 1.14 `ensure!` user-facing arms (`#[cfg(doc)]` form)
- Content: `tests/fixtures/anyhow/src/macros.rs:127-153` — the `#[cfg(doc)]` form of `ensure!` with its four readable arms (`cond` / `cond, $msg:literal` / `cond, $err:expr` / `cond, $fmt, $($arg)*`). Lives inside the `__ensure!` wrapping macro.
- Cost: 213 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:127-153`)
- Notes: this is the human-readable signature visible in rustdoc; the actual `#[cfg(not(doc))]` form trampolines into the giant `__parse_ensure!` token-tree parser (below the fold). Vastly more readable than the parser.

### 2.1 `extern crate` + internal `mod` declarations
- Content: `tests/fixtures/anyhow/src/lib.rs:246-263` — `extern crate alloc;`, `#[cfg(feature = "std")] extern crate std;`, then the 11 internal `mod ...;` declarations (`backtrace`, `chain`, `context`, `ensure`, `error`, `fmt`, `kind`, `macros`, `nightly`, `ptr`, `wrapper`).
- Cost: 60 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:246-263`)
- Notes: the routing table from concept ("the macro parser") to file (`mod ensure`). Same order as the `src/` listing in 1.2.

### 2.2 README "Details" — `?` propagation, `.context(...)`, `Caused by`
- Content: `tests/fixtures/anyhow/README.md:21-67` — the first two bullets of the canonical tutorial (`Result<T, anyhow::Error>` / `anyhow::Result<T>` / `?`, then `.context(...)` / `.with_context(|| ...)` and the resulting `Caused by:` console output).
- Cost: 342 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/README.md:21-67`)
- Notes: doctrine and intent for the most-asked queries. The most efficient single-batch route to "how do I use anyhow in user code?".
- Predecessor: 1.4

### 2.3 `Error` short doc preamble (the three differences from `Box<dyn StdError>`)
- Content: `tests/fixtures/anyhow/src/lib.rs:288-298` — the doc lead-in saying `Error` is `Send + Sync + 'static`, guarantees a backtrace, and is one word wide. (The long `# Display representations` body that follows ranks at 3.1.) Render with a `…` elision marker after.
- Cost: 113 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:288-298`)
- Notes: the load-bearing characterization of `Error`. Ranks here so a small budget gets it together with the declaration (1.6).
- Predecessor: 1.6

### 2.4 `Error` constructor signatures (`new`, `msg`, `from_boxed`)
- Content: signature lines from `tests/fixtures/anyhow/src/error.rs`, each with full where-clause and elision marker between the three: `19-36` (`Error::new<E>(error: E) -> Self where E: StdError + Send + Sync + 'static`), `75-82` (`Error::msg<M>(message: M) -> Self where M: Display + Debug + Send + Sync + 'static`), `137-143` (`Error::from_boxed(boxed_error: Box<dyn StdError + Send + Sync + 'static>) -> Self`).
- Cost: 301 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:19-36 tests/fixtures/anyhow/src/error.rs:75-82 tests/fixtures/anyhow/src/error.rs:137-143`)
- Notes: the three most-used construction entry points, with brief docs each. The bound `Send + Sync + 'static` recurs across the API.

### 2.5 README "Details" — downcasting, backtrace env, thiserror, `anyhow!`/`bail!`
- Content: `tests/fixtures/anyhow/README.md:68-122` — bullets 3-6: `downcast_ref::<T>()`, the `RUST_BACKTRACE`/`RUST_LIB_BACKTRACE` env vars, deriving error types via `thiserror`, and the `anyhow!`/`bail!` macros in user code.
- Cost: 474 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/README.md:68-122`)
- Notes: completes the canonical 60-second tour started in 2.2. Reads as one continuous tutorial with 2.2.
- Predecessor: 2.2

### 2.6 `Error` consumer-method signatures (downcast, chain, backtrace, ...)
- Content: signature ranges in `tests/fixtures/anyhow/src/error.rs`, no rustdoc, with `…` elision markers between non-contiguous ranges: `372-375` (`Error::context<C>(self, context: C) -> Self where C: Display + Send + Sync + 'static`), `432-433` (`Error::backtrace(&self) -> &impl_backtrace!()`, cfg-gated), `459-460` (`Error::chain(&self) -> Chain`), `470-472` (`Error::root_cause(&self) -> &(dyn StdError + 'static)`), `482-485` (`Error::is<E>(&self) -> bool`), `490-493` (`Error::downcast<E>(mut self) -> Result<E, Self>`), `554-557` (`Error::downcast_ref<E>(&self) -> Option<&E>`), `568-571` (`Error::downcast_mut<E>(&mut self) -> Option<&mut E>`), `619-622` (`Error::into_boxed_dyn_error(self) -> Box<dyn StdError + Send + Sync + 'static>`), `660-665` (`Error::reallocate_into_boxed_dyn_error_without_backtrace(self) -> Box<...>`).
- Cost: 374 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:372-375 tests/fixtures/anyhow/src/error.rs:432-433 tests/fixtures/anyhow/src/error.rs:459-460 tests/fixtures/anyhow/src/error.rs:470-472 tests/fixtures/anyhow/src/error.rs:482-485 tests/fixtures/anyhow/src/error.rs:490-493 tests/fixtures/anyhow/src/error.rs:554-557 tests/fixtures/anyhow/src/error.rs:568-571 tests/fixtures/anyhow/src/error.rs:619-622 tests/fixtures/anyhow/src/error.rs:660-665`)
- Notes: the entire instance-method API of `Error` as bare signatures. With this plus 2.4 (constructors), the agent can answer "does anyhow have downcast?" / "how do I get the source chain?" without follow-ups. The rich rustdoc on `Error::context` (316-369), `Error::backtrace` (404-431) with the env-var doctrine, `Error::chain` (436-458) with the `underlying_io_error_kind` example, and `Error::into_boxed_dyn_error` / `reallocate_..._backtrace` (582-615 / 631-660) lives later (4.1, 4.2) or below-the-fold.
- Predecessor: 1.6

### 2.7 README — no_std + comparison + license footer
- Content: `tests/fixtures/anyhow/README.md:124-180` — `## No-std support` (with the `default-features = false` snippet and the Rust < 1.81 `.map_err(Error::msg)` caveat), `## Comparison to failure`, `## Comparison to thiserror`, `#### License`.
- Cost: 457 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/README.md:124-180`)
- Notes: positions the crate vs `thiserror`/`failure` (a common first question) and surfaces the no-std story with its only material caveat.

### 2.8 `tests/test_repr.rs` (full file — one-word size invariant)
- Content: `tests/fixtures/anyhow/tests/test_repr.rs:1-30`.
- Cost: 169 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_repr.rs`)
- Notes: asserts `mem::size_of::<Error>() == size_of::<usize>()` and that `Result<(), Error>` is also one word. The single most important architectural invariant of the crate, asserted in code — the entire rationale for the elaborate vtable in `error.rs`.

### 2.9 `tests/test_autotrait.rs` (full file — auto-trait guarantees)
- Content: `tests/fixtures/anyhow/tests/test_autotrait.rs:1-34`.
- Cost: 174 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_autotrait.rs`)
- Notes: enumerates the auto-trait guarantees on `Error`: `Send`, `Sync`, `UnwindSafe`, `RefUnwindSafe`, `Unpin`. Important and not obvious from a function signature.

### 2.10 `tests/test_ffi.rs` (full file — FFI-safety of `Error` / `Option<Error>`)
- Content: `tests/fixtures/anyhow/tests/test_ffi.rs:1-19`.
- Cost: 126 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_ffi.rs`)
- Notes: shows `anyhow::Error` and `Option<anyhow::Error>` are FFI-safe — pairs naturally with 2.8 (one-word repr).

### 3.1 `Error` Display-representations rustdoc (the canonical formatting reference)
- Content: `tests/fixtures/anyhow/src/lib.rs:299-388` — the long doc comment showing `{}`, `{:#}`, `{:?}`, `{:#?}` rendered output examples, including the literal "Caused by:" / "Stack backtrace:" layout.
- Cost: 701 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:299-388`)
- Notes: the *only* place that actually shows what error messages look like end-to-end. Drives almost every formatting/UX question. Most agent queries about anyhow output ("how do I print causes?") are answered here without touching `fmt.rs`.
- Predecessor: 1.6

### 3.2 `tests/test_fmt.rs` expected-output constants (executable formatter spec)
- Content: `tests/fixtures/anyhow/tests/test_fmt.rs:1-67` — functions `f`/`g`/`h` plus the `EXPECTED_*` constants for `Display`, `{:#}`, `{:?}`, `{:#?}` across a 3-level cause chain.
- Cost: 346 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_fmt.rs:1-67`)
- Notes: the `EXPECTED_*` const strings are the *authoritative* spec for what error formatting produces. Reading these constants is faster than reading `fmt.rs` for "what string does `{:?}` produce?" queries. Pairs with 3.1 and 4.6.

### 3.3 `Display`/`Debug` impls for `ErrorImpl` (`fmt.rs` head)
- Content: `tests/fixtures/anyhow/src/fmt.rs:1-67` — the `impl ErrorImpl { display + debug }` printing logic, including the `: ` alt-display separator, the `Caused by:` header, the `Indented` numbering, and the conditional `Stack backtrace:` capitalization.
- Cost: 502 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/fmt.rs:1-67`)
- Notes: the exact code emitting every formatting shape documented in 3.1 and asserted in 3.2. Without this batch the agent can only describe the format from docs; with it, it can answer questions like "where does the `: ` separator come from" with no follow-ups.
- Predecessor: 2.6

### 3.4 `Context` trait rustdoc — `ImportantThing` example + downcast rule statement
- Content: `tests/fixtures/anyhow/src/lib.rs:469-523` — the `Context` trait header doc (`Provides the context method...`), the `ImportantThing` worked example for plain `.context(...)`, the resulting `Caused by:` console output, and the lead-in to "Effect on downcasting" stating the rule (`anyhow::Error` may be downcast to `C` **or** `E`).
- Cost: 329 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:469-523`)
- Notes: the canonical use-case example plus the surprising downcast invariant statement. The two extended examples (`SuspiciousError`/`HelperFailed`) that justify the rule (lines 524-615) are below-the-fold.
- Predecessor: 1.8

### 3.5 `Context` impls for `Result<T, E>` and `Option<T>`
- Content: `tests/fixtures/anyhow/src/context.rs:1-68` (the `mod ext` private trait shim plus `impl<T,E> Context<T,E> for Result<T,E>`) and `tests/fixtures/anyhow/src/context.rs:70-113` (the `Option<T>` doctest then `impl<T> Context<T, Infallible> for Option<T>` body).
- Cost: 745 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/src/context.rs:1-68` 453 + `:70-113` 292)
- Notes: where `.context(...)` actually comes from. Together they say `.context(...)` works on any `Result<T, E>` whose error implements `std::error::Error`, *and* on any `Option<T>` (where `None` becomes a synthesized error). The "Not using map_err to save 2 useless frames" comment is a load-bearing rationale.
- Predecessor: 1.8

### 3.6 `chain.rs` full file (`Chain` iterator)
- Content: `tests/fixtures/anyhow/src/chain.rs:1-102` — `ChainState::{Linked, Buffered}`, the `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `Default` impls, and the cfg-conditional `pub(crate) use crate::Chain` re-export.
- Cost: 654 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/chain.rs`)
- Notes: small, self-contained, and the reference for `Error::chain()` / `Error::root_cause()`. Splitting hurts — the impls only make sense together with the state machine.

### 3.7 `tests/test_chain.rs` (full file)
- Content: `tests/fixtures/anyhow/tests/test_chain.rs:1-69`.
- Cost: 581 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_chain.rs`)
- Notes: the `Chain` iterator's behavior is documented in tests at least as clearly as in code. Covers `next`, `next_back`, `len`, `size_hint`, `default`, `clone` and the iteration-order contract (outermost context first).

### 3.8 `tests/common/mod.rs` and `tests/drop/mod.rs` (shared test helpers)
- Content: `tests/fixtures/anyhow/tests/common/mod.rs:1-14` (the `bail_literal`/`bail_fmt`/`bail_error` fixtures) plus `tests/fixtures/anyhow/tests/drop/mod.rs:1-53` (the `Flag` / `DetectDrop` drop-detection primitives).
- Cost: 355 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/tests/common/mod.rs` 81 + `:drop/mod.rs` 274)
- Notes: re-used by ~half of the test files; without these, `test_repr.rs`, `test_context.rs`, `test_convert.rs`, `test_downcast.rs` test bodies are unintelligible. Showing them once unlocks all four.

### 4.1 `Error::context` method body + rationale doc (the inherent method)
- Content: `tests/fixtures/anyhow/src/error.rs:316-402` — the docstring explaining "primary reason to use `error.context(...)` instead of `result.context(...)`" with the `ParseError` example, and the body that builds a fresh vtable for `ContextError<C, Error>`.
- Cost: 744 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:316-402`)
- Notes: distinct from `Context::context`. Both should be discoverable. The "context computed from the error's payload" pattern is shown only here.
- Predecessor: 2.6

### 4.2 `Error::into_boxed_dyn_error` + `reallocate_..._without_backtrace` doc + bodies
- Content: `tests/fixtures/anyhow/src/error.rs:582-672` — both rustdoc blocks (with the `error_generic_member_access` doctest examples illustrating the "lose-backtrace vs lose-downcast" tradeoff) and both method bodies.
- Cost: 1000 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:582-672`)
- Notes: the round-trip story for `anyhow::Error` ↔ `Box<dyn StdError>`. The two methods make opposite tradeoffs; their docs and bodies are paired.
- Predecessor: 2.6

### 4.3 `From<E>`, `Deref`/`DerefMut`, `Display`, `Debug`, `Drop` impls for `Error`
- Content: `tests/fixtures/anyhow/src/error.rs:691-738`.
- Cost: 334 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:691-738`)
- Notes: the `From<E> for Error` impl is what makes `?` work; the `Drop` impl invokes the vtable's drop hook; the `Display`/`Debug` impls forward to `ErrorImpl`. Short and load-bearing.
- Predecessor: 5.1

### 4.4 `kind.rs` autoref-dispatch design comment
- Content: `tests/fixtures/anyhow/src/kind.rs:1-46` — the long header comment explaining tagged dispatch via autoref ("we rely on autoref behavior of method resolution to perform tagged dispatch").
- Cost: 373 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/kind.rs:1-46`)
- Notes: explains *why* `anyhow!(expr)` resolves differently for `&str` vs `io::Error` vs `Box<dyn StdError + Send + Sync>`. One of the most pedagogically valuable comments in the crate; the only place this design is explained.
- Predecessor: 1.13

### 4.5 `kind.rs` traits and `Adhoc`/`Trait`/`Boxed` `new()` impls
- Content: `tests/fixtures/anyhow/src/kind.rs:55-121` — the three traits `AdhocKind`, `TraitKind`, `BoxedKind` and their `Adhoc::new`, `Trait::new`, `Boxed::new` impls.
- Cost: 393 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/kind.rs:55-121`)
- Notes: the three dispatch types `anyhow!` resolves to. The cfg gating on `BoxedKind` is the same `(feature = "std", not(anyhow_no_core_error))` predicate seen elsewhere.
- Predecessor: 4.4

### 4.6 `Indented` formatter helper (and its `#[cfg(test)]` cases)
- Content: `tests/fixtures/anyhow/src/fmt.rs:69-158` — the `Indented` `Write` impl and three `#[test]` cases (`one_digit` / `two_digits` / `no_digits`).
- Cost: 477 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/fmt.rs:69-158`)
- Notes: the numeric/non-numeric indentation logic that produces `"    0: ..."` / `"    foo"` shapes. The three small tests are the cleanest spec for the indent format.
- Predecessor: 3.3

### 5.1 `ErrorVTable` struct + `ErrorImpl<E>` repr + `ContextError`
- Content: `tests/fixtures/anyhow/src/error.rs:740-755` (the `struct ErrorVTable { object_drop, object_ref, object_boxed, object_reallocate_boxed, object_downcast, object_drop_rest, [object_backtrace] }`) plus `tests/fixtures/anyhow/src/error.rs:930-955` (the `#[repr(C)] pub(crate) struct ErrorImpl<E = ()>`, the `vtable()` reader, and `pub(crate) struct ContextError<C, E>`). Render with a `…` elision marker between non-contiguous ranges.
- Cost: 485 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:740-755` 220 + `:930-955` 265)
- Notes: the *core trick* of anyhow — type-erasure via a manually-curated vtable so `Error` is one word wide. The `#[repr(C)]` placement of `vtable` first justifies the pointer trick used by `vtable()`. Without this, the unsafe code in `error.rs` reads as opaque magic.

### 5.2 `Error::construct_from_std/_adhoc/_display` (first three constructor wirings)
- Content: `tests/fixtures/anyhow/src/error.rs:144-225`.
- Cost: 780 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:144-225`)
- Notes: how standard / ad-hoc / display-only errors get a vtable. Each method mounts a vtable that knows the original `E`.
- Predecessor: 5.1

### 5.3 `Error::construct_from_context/_boxed` + the unsafe `construct<E>` core
- Content: `tests/fixtures/anyhow/src/error.rs:226-314` — `construct_from_context`, `construct_from_boxed`, and the `unsafe fn construct<E>` that erases the type into the thin pointer.
- Cost: 861 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:226-314`)
- Notes: continues the pattern from 5.2 with the context-wrapping constructor and the type-erasing core. This is the unsafe heart of the thin-pointer design.
- Predecessor: 5.2

### 5.4 `wrapper.rs` full file (`MessageError`, `DisplayError`, `BoxedError`)
- Content: `tests/fixtures/anyhow/src/wrapper.rs:1-84`.
- Cost: 569 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/wrapper.rs`)
- Notes: the three `#[repr(transparent)]` newtypes that adapt message-only / display-only / boxed-dyn-error inputs into `StdError`-implementing values used by `Error::construct_from_*`. The transparent layout is the safety condition for vtable downcasts. Compact and load-bearing; splitting hurts.
- Predecessor: 5.1

### 5.5 `Error::is`/`downcast`/`downcast_ref`/`downcast_mut` bodies
- Content: `tests/fixtures/anyhow/src/error.rs:482-580` — the four method bodies with their vtable lookups and the `ManuallyDrop` dance for owning extraction.
- Cost: 809 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:482-580`)
- Notes: the actual implementation behind the most-used non-construction methods. Predecessor edges to both 5.1 (vtable) and 2.6 (signatures).
- Predecessor: 5.1

### 5.6 `ErrorImpl` private accessors (`error`, `error_mut`, `backtrace`, `provide`, `chain`, `vtable`)
- Content: `tests/fixtures/anyhow/src/error.rs:957-1013` — `ErrorImpl::erase`, `error`/`error_mut`, `backtrace`, `provide`, `chain`, `unsafe fn vtable(p)`.
- Cost: 526 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:957-1013`)
- Notes: the unsafe accessors that the public methods on `Error` (and the `Display`/`Debug` impls) actually call into. The `Error::backtrace` panic-on-failure expectation lives here.
- Predecessor: 5.1

### 6.1 `build.rs` `fn main` (cfg decision tree)
- Content: `tests/fixtures/anyhow/build.rs:1-97` — the entire decision tree for `error_generic_member_access`, `std_backtrace`, `anyhow_no_core_error`, `anyhow_no_clippy_format_args`, `anyhow_build_probe`, `anyhow_nightly_testing` cfgs, plus the `compile_error!` for `backtrace` without `std`, plus the `cargo:rustc-check-cfg` declarations for rustc ≥ 80.
- Cost: 923 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/build.rs:1-97`)
- Notes: explains every `#[cfg(...)]` predicate that appears throughout `src/`. Without this, an agent reading `error.rs` will be confused by the conditional compilation. *The* source of every `#[cfg(error_generic_member_access)]` in `src/`.

### 6.2 `nightly.rs` full file (generic-member-access probe + runtime API)
- Content: `tests/fixtures/anyhow/src/nightly.rs:1-58`.
- Cost: 414 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/nightly.rs`)
- Notes: dual-purpose — the `#[cfg(anyhow_build_probe)] const _: () = { ... }` block is what `build.rs` exercises; the `request_ref_backtrace`/`provide_ref_backtrace`/`provide` functions are the runtime wrappers around `error::request_ref` and `Request::provide_ref`. Predecessor edge to 6.1 (build.rs gates compilation of this file's body).
- Predecessor: 6.1

### 6.3 `backtrace.rs` cfg dispatch + `impl_backtrace!`/`backtrace!`/`backtrace_if_absent!` macros
- Content: `tests/fixtures/anyhow/src/backtrace.rs:1-68`.
- Cost: 401 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/backtrace.rs:1-68`)
- Notes: the three-way `Backtrace` re-export decision tree (`std::backtrace` vs the `backtrace` crate vs an empty enum) plus the three macros that propagate cfg-gated behavior into the rest of `src/`. The 339-line `mod capture { ... }` body (lines 70-410) is below-the-fold.

### 6.4 `ensure.rs` runtime helpers (`Buf`, `BothDebug`/`NotBothDebug`, `render`)
- Content: `tests/fixtures/anyhow/src/ensure.rs:1-101` — the `BothDebug` / `NotBothDebug` autoref-dispatch traits, the `Buf` no-alloc 40-byte stack buffer, and the `render` function.
- Cost: 665 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/ensure.rs:1-101`)
- Notes: explains where ``Condition failed: `v + v == 1` (2 vs 1)`` comes from — the `(2 vs 1)` formatting suffix on `ensure!` errors when both sides implement `Debug`, and the no-alloc buffer used to do it.
- Predecessor: 1.14

### 6.5 `ensure.rs` terminal macros (`__fancy_ensure!`, `__fallback_ensure!`)
- Content: `tests/fixtures/anyhow/src/ensure.rs:884-935`.
- Cost: 409 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/ensure.rs:884-935`)
- Notes: the two terminal macros the parser dispatches to. Together they fully define what `ensure!` actually expands to — fancy form uses `BothDebug`/`NotBothDebug` autoref dispatch (predecessor 6.4), fallback handles literal/expr/fmt forms.
- Predecessor: 6.4

### 6.6 `lib.rs` `__private` module
- Content: `tests/fixtures/anyhow/src/lib.rs:654-728`.
- Cost: 410 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/lib.rs:654-728`)
- Notes: items the macros need at expansion time: `format_err`, `must_use`, the `Bool` trait & impls (the last is what `ensure-nonbool.stderr` complains about), the `kind` re-exports. Required to understand any macro-related compilation error.

### 6.7 `From<Error> for Box<dyn StdError + ...>` + `AsRef` + `UnwindSafe`
- Content: `tests/fixtures/anyhow/src/error.rs:1047-1086`.
- Cost: 304 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/src/error.rs:1047-1086`)
- Notes: the three `From<Error> for Box<dyn StdError + ...>` impls, the two `AsRef`s, and the `UnwindSafe`/`RefUnwindSafe` markers. Round-trip story for anyhow ↔ stdlib boxed errors. Thin trampolines to `into_boxed_dyn_error`.
- Predecessor: 4.2

### 7.1 `bail!` and `anyhow!` rustdoc (with examples)
- Content: `tests/fixtures/anyhow/src/macros.rs:1-55` (`bail!` rustdoc + two examples) plus `tests/fixtures/anyhow/src/macros.rs:174-201` (`anyhow!` rustdoc + lookup example). Render with `…` elision marker between.
- Cost: 550 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:1-55` 337 + `:174-201` 213)
- Notes: each summary captures equivalence ("`bail!` ≡ `return Err(anyhow!(…))`", `anyhow!` source-preservation rules) and the `Result<_, anyhow::Error>` requirement. Adds the prose layer on top of the macro definitions in 1.12-1.13.
- Predecessor: 1.13

### 7.2 `__ensure!` doc wrapper + `#[cfg(not(doc))]` dispatch arm
- Content: `tests/fixtures/anyhow/src/macros.rs:70-124` (the `__ensure!` macro with the rustdoc + two `# Example` blocks) plus `tests/fixtures/anyhow/src/macros.rs:155-172` (the `#[cfg(not(doc))]` arm that delegates to `__parse_ensure!`).
- Cost: 572 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/src/macros.rs:70-124` 422 + `:155-172` 150)
- Notes: shows the dual-form trick — the `cfg(doc)` arm in 1.14 is what users see in rustdoc; the `cfg(not(doc))` arm here is what the build actually compiles, trampolining into the giant `__parse_ensure!` parser (below-the-fold).
- Predecessor: 1.14

### 7.3 CI matrix and probes (test job + minimal-versions + clippy/miri)
- Content: `tests/fixtures/anyhow/.github/workflows/ci.yml:19-50` (the `test` job + matrix `[nightly, beta, stable, 1.82.0, 1.80.0, 1.76.0]` + `cargo test` / `cargo check --no-default-features` / `cargo check --features backtrace` invocations + the `--cfg=anyhow_nightly_testing` flag), `tests/fixtures/anyhow/.github/workflows/ci.yml:71-82` (minimal versions), `tests/fixtures/anyhow/.github/workflows/ci.yml:112-140` (clippy + miri jobs, including the miri pin `nightly-2026-02-11`). Render with `…` elision markers between non-contiguous ranges.
- Cost: 674 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/.github/workflows/ci.yml:19-50` 330 + `:71-82` 97 + `:112-140` 247)
- Notes: documents which rustc versions are supported in CI (matrix vs MSRV from `Cargo.toml`), the explicit `--features backtrace` exercise, and the strict `clippy::pedantic` lint level. The `pre_ci`, `windows`, `doc`, `outdated` jobs are below-the-fold.

### 7.4 `tests/crate/` no-std smoke crate
- Content: `tests/fixtures/anyhow/tests/crate/Cargo.toml:1-17` plus `tests/fixtures/anyhow/tests/crate/test.rs:1-3`.
- Cost: 103 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/tests/crate/Cargo.toml` 94 + `:test.rs` 9)
- Notes: a tiny `#![no_std] pub use anyhow::*;` crate that CI exercises against the 1.68 MSRV via `cargo check --manifest-path tests/crate/Cargo.toml`. Two-file batch.

### 7.5 `tests/ui/` directory listing
- Content: rendered listing of the 14 entries under `tests/ui/` — 7 paired `.rs` + `.stderr` files (`chained-comparison`, `empty-ensure`, `ensure-nonbool`, `must-use`, `no-impl`, `temporary-value`, `wrong-interpolation`).
- Cost: 70 tokens (helper: `ls tests/fixtures/anyhow/tests/ui/ | scripts/count-tokens.py --stdin`)
- Notes: each pair is a `trybuild` compile-fail case asserting a specific diagnostic. Names alone tell the agent which footguns are guarded.

### 7.6 `tests/test_source.rs` (full file — `anyhow!` source-preservation rules)
- Content: `tests/fixtures/anyhow/tests/test_source.rs:1-62`.
- Cost: 368 tokens (helper: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_source.rs`)
- Notes: demonstrates exactly which `anyhow!` invocations preserve `source()` and which don't (literal/variable/fmt produce no source; `impl StdError` does). Avoids subtle bugs.

### 7.7 `tests/test_ensure.rs` preamble + per-test fn index
- Content: `tests/fixtures/anyhow/tests/test_ensure.rs:1-39` (allow attrs + `use`s + helper struct/trait definitions) plus rendered output of `grep -nE '^(#\[test\]|fn test_)' tests/fixtures/anyhow/tests/test_ensure.rs`.
- Cost: 544 tokens (helper sum: `scripts/count-tokens.py tests/fixtures/anyhow/tests/test_ensure.rs:1-39` 308 + grep-pipe 236)
- Notes: this file is a *spec* for what the `ensure!` parser must accept (closures, control flow, paths, raw addrs, `as` casts, patterns). Preamble shows the operator/trait setup; the fn-signature index lets the agent jump to any specific scenario. Surfacing this is what makes it safe to keep the gigantic `__parse_ensure!` parser below-the-fold.

### 7.8 Cross-test `#[test] fn` index (test files not ranked above)
- Content: rendered output of `grep -nE '^(#\[test\]|fn test_)' tests/fixtures/anyhow/tests/test_boxed.rs tests/fixtures/anyhow/tests/test_context.rs tests/fixtures/anyhow/tests/test_convert.rs tests/fixtures/anyhow/tests/test_downcast.rs tests/fixtures/anyhow/tests/test_macros.rs` — `file:line:` followed by the `#[test]` attribute or `fn test_*` line, for the five files whose bodies are below-the-fold.
- Cost: 975 tokens (helper: `grep -nE '^(#\[test\]|fn test_)' tests/fixtures/anyhow/tests/test_boxed.rs tests/fixtures/anyhow/tests/test_context.rs tests/fixtures/anyhow/tests/test_convert.rs tests/fixtures/anyhow/tests/test_downcast.rs tests/fixtures/anyhow/tests/test_macros.rs | scripts/count-tokens.py --stdin`)
- Notes: lets the agent jump precisely to a specific test scenario in those bodies without grepping. `test_ensure.rs` is indexed separately in 7.7; `test_chain`, `test_repr`, `test_autotrait`, `test_ffi`, `test_fmt`, `test_source` are already shown in full above; `test_backtrace` is trivial enough to be below-the-fold.

## Below-the-fold

The fixture's gross size is ~58k tokens; the cap is ~20k. Cumulative ranked
content above sums to ~22.8k tokens, modestly over the soft cap. The fixture
is information-dense, and the threshold test (every cut from K=5 through
K=59 yields a coherent slice) is what set the size; the omissions below are
deliberate.

- **`tests/fixtures/anyhow/src/ensure.rs:103-883`** (≈19,300 tokens): the gigantic `__parse_ensure!` recursive macro_rules parser (~780 lines covering operators, control flow, paths, types, patterns, `as` casts). Single-handedly larger than the entire ranking budget; would also wildly violate the 2× growth constraint at any reasonable position. Pure machinery to parse arbitrary Rust expressions for `ensure!`. Real-world value to readers is near-zero — nobody hand-edits this. Shape is fully captured by 1.14 (cfg-doc arm), 6.4 (runtime helpers), 6.5 (terminal macros), 7.2 (dispatch entry), and 7.7 (the test spec). If needed, the agent can `Read tests/fixtures/anyhow/src/ensure.rs:103-883`.
- **`tests/fixtures/anyhow/src/backtrace.rs:70-410`** (≈2,430 tokens): the `mod capture` body containing the vendored `backtrace` crate integration (`Backtrace`/`BacktraceStatus`/`Inner`/`Capture`/`BacktraceFrame`/`BacktraceSymbol`/`BytesOrWide`/`LazilyResolvedCapture`/`output_filename`). Only relevant when building with `--features backtrace` on a pre-1.65 toolchain that the CI matrix barely touches; one `Read` away once 6.3 has named the file.
- **`tests/fixtures/anyhow/src/error.rs:756-928`** (~1,761 tokens): the eleven `object_*` and `context_*` free vtable functions (`object_drop`, `object_ref`, `object_boxed`, `object_reallocate_boxed`, `object_downcast`, `object_drop_front`, `no_backtrace`, `context_downcast`, `context_drop_rest`, `context_chain_downcast`, `context_chain_drop_rest`, `context_backtrace`). These are the actual implementations behind the function pointers in `ErrorVTable` (5.1) and the constructors (5.2-5.3). Mechanical type-erased forwarding code; demand-fetch when a downcast bug needs investigation. The `context_*` variants specialize the same patterns for `ContextError<C, E>` and `ContextError<C, Error>` — same shape, more cases.
- **`tests/fixtures/anyhow/src/lib.rs:14-208`** (~1,632 tokens): body of the crate-level rustdoc, including the `# Details` and `# No-std support` sections. Mirrors the README (already ranked at 1.4 / 2.2 / 2.5 / 2.7) almost line-for-line, just with hidden-doctest scaffolding (`# ...`); README carries the same content more compactly. The lead-in (`:1-13`, badges + html_root_url) is also redundant.
- **`tests/fixtures/anyhow/src/lib.rs:524-615`** (~705 tokens): the "Effect on downcasting" worked examples (`SuspiciousError` / `HelperFailed`) on the `Context` trait. The downcast invariant rule itself is stated cheaply in 3.4; these example functions justify it but are not needed to use the trait correctly. One `Read` away from 3.4.
- **`tests/fixtures/anyhow/src/error.rs:1015-1045`** (~202 tokens): `StdError`/`Debug`/`Display` impls on `ErrorImpl<E>`. Forwarders only; the interesting versions are on `ErrorImpl` (no `<E>`) and live in 5.6.
- **`tests/fixtures/anyhow/src/error.rs:1-18`** (163 tokens): the `use` block at file head. Imports list is recoverable from the items it brings in.
- **`tests/fixtures/anyhow/src/lib.rs:1-13`** (218 tokens): badges + `#![doc(html_root_url = ...)]`. Cosmetic; identity is in 1.5.
- **`tests/fixtures/anyhow/src/lib.rs:209-247`** (~321 tokens): the `#![cfg_attr(error_generic_member_access, ...)]`, `#![no_std]`, `#![deny(...)]`, `#![allow(clippy::*)]` attribute bag and the `compile_error!` for nightly-probe failure. Lint preferences and the no-std switch only; the cfg the lint references is wired in 6.1.
- **`tests/fixtures/anyhow/src/lib.rs:269-283`** (~109 tokens): the `StdError` re-import dance and the `trait StdError { ... }` shim for `(not feature="std", anyhow_no_core_error)`. Implementation detail; covered conceptually by 1.11 (features) and 6.1 (cfgs).
- **`tests/fixtures/anyhow/src/lib.rs:394-412`** (~111 tokens): `Chain` rustdoc + `underlying_io_error_kind` example. Same example as `Error::chain` doc in 2.6; one `Read` away.
- **`tests/fixtures/anyhow/src/lib.rs:442-467`** (~170 tokens): long `cluster_info` worked example for `Result<T>`. Same shape as the README example in 2.2.
- **`tests/fixtures/anyhow/src/error.rs:84-136`** (~453 tokens): long `from_boxed` rustdoc with the `Report ↔ anyhow::Error` interop skeleton. Pattern is unique but `from_boxed` is already named in 2.4 and one `Read` away.
- **`tests/fixtures/anyhow/src/context.rs:115-193`** (~510 tokens): `ContextError` `Debug`/`Display`/`StdError` impls, the `Quoted<C>` Debug helper, the `pub(crate) mod private { trait Sealed }` module. Supporting types whose existence is implied by 3.5; rarely the answer to a user question.
- **`tests/fixtures/anyhow/build.rs:99-207`** (~954 tokens): `compile_probe`, `rustc_minor_version`, `cargo_env_var` helpers. Implementation details of the build script's nightly probe; the *decision* logic is in 6.1, the helpers are one `Read` away.
- **`tests/fixtures/anyhow/.github/workflows/ci.yml`** `pre_ci`, `build` (the 1.68 MSRV check that exercises `tests/crate/`), `windows`, `doc`, `outdated` jobs (~520 tokens combined). Routine; the `pre_ci.yml@master` workflow lives in another repo, the windows job just runs `cargo check --features backtrace`, `outdated` is a scheduled freshness check, and the `build` job is conceptually summarized by 7.4.
- **Test-file bodies** — `test_macros.rs` (618 tokens), `test_downcast.rs` (791 tokens), `test_convert.rs` (314 tokens), `test_boxed.rs` (290 tokens), `test_context.rs` (full 1007-token file, of which the `context_type!` macro and `LowLevel`/`MidLevel`/`HighLevel` types and `make_chain` helper plus the `test_downcast_ref` test demonstrate stacking `.context(MidLevel)` then `.context(HighLevel)` and reading back via `downcast_ref::<X>()` across both `Context for Result<T, E: StdError>` and `Context for Result<T, anyhow::Error>` paths), `test_fmt.rs:69-93` (the `#[test]` bodies for `test_display`/`test_altdisplay`/`test_debug`/`test_altdebug`, ~223 tokens), `test_backtrace.rs` (77 tokens, nightly-gated trivial). Function names indexed in 7.8 with file:line; the shared `Flag`/`DetectDrop` primitives needed to read most of these are in 3.8. `test_boxed.rs`'s three `anyhow_kind` dispatch paths are already named in 4.4-4.5. Bodies are one `Read` away. Notable individual omission: `test_downcast.rs` includes a `LargeAlignedError(#[repr(align(64))])` case that's the only piece not also exercised by `test_context.rs`.
- **`tests/fixtures/anyhow/tests/test_ensure.rs:40-756`** (~6,977 tokens of edge-case parser regression tests, e.g. `test_low_precedence_control_flow`, `test_unary`, `test_path`, `test_pat`). All "this exact source string must produce this exact error message"; preamble + fn index in 7.7 give the agent enough to read individual cases on demand.
- **`tests/fixtures/anyhow/tests/ui/*.rs` and `tests/ui/*.stderr`**: 7 compile-fail cases listed in 7.5 (small inputs 22-173 tokens each) plus their generated stderr (the largest is `no-impl.stderr` at 413 tokens, which surfaces the `AdhocKind`/`TraitKind`/`BoxedKind` dispatch from 4.4-4.5 in a diagnostic). Names alone in 7.5 indicate which footguns are guarded.
- **`tests/fixtures/anyhow/Cargo.toml:32-40`** (89 tokens): `[package.metadata.docs.rs]` rustdoc args. Not load-bearing for development.
- **`tests/fixtures/anyhow/LICENSE-APACHE`** (1998 tokens) and **`LICENSE-MIT`** (216 tokens): boilerplate; the SPDX in `Cargo.toml` (1.5) already records `MIT OR Apache-2.0`, and the README footer (2.7) restates it.
- **`tests/fixtures/anyhow/.github/FUNDING.yml`** (6 tokens), **`.gitignore`** (6 tokens), **`rust-toolchain.toml`** (10 tokens): trivial single-line files; presence is conveyed by 1.1. `rust-toolchain.toml` only adds `components = ["rust-src"]`, mentioned in 7.3.
