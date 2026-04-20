# anyhow — North Star

Revision pin: `769cba0b`

The fixture is dtolnay's `anyhow` crate (v1.0.101): a `no_std`-capable Rust error library exposing a single-word `Error` smart pointer, a `Context` extension trait for `Result`/`Option`, an iterator over the source-error chain, and three macros (`anyhow!`, `bail!`, `ensure!`). The implementation pivots on a hand-rolled vtable in `src/error.rs` (so `Error` is one word instead of the two-word `Box<dyn StdError>`) and a token-tree parser macro `__parse_ensure!` in `src/ensure.rs` (~770 lines of macro arms, ~19k tokens) that decomposes the `ensure!` condition into `lhs`/`op`/`rhs` for `(2 vs 1)`-style failure rendering. The top-level `lib.rs` carries the canonical user-facing rustdoc; the rest of `src/` is small focused modules; `tests/` is a behavioral test suite plus a `trybuild` UI suite.

## Batches

### 1.1 Repo-root listing
- Content: rendered listing of root entries — `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`, `build.rs`, `rust-toolchain.toml`, `src/`, `tests/`, `.github/`
- Cost: 31 tokens (helper: `echo 'Cargo.toml\nLICENSE-APACHE\nLICENSE-MIT\nREADME.md\nbuild.rs\nrust-toolchain.toml\nsrc/\ntests/\n.github/' | count-tokens.py --stdin`)
- Notes: orients the agent — flags that there's a `build.rs` (non-trivial cfg-probe) and a separate `tests/` tree.

### 1.2 `src/` listing
- Content: rendered listing of `src/` — `backtrace.rs`, `chain.rs`, `context.rs`, `ensure.rs`, `error.rs`, `fmt.rs`, `kind.rs`, `lib.rs`, `macros.rs`, `nightly.rs`, `ptr.rs`, `wrapper.rs`
- Cost: 39 tokens (helper: `echo '<files>' | count-tokens.py --stdin`)
- Notes: each filename names its responsibility; this listing alone gives a near-complete mental map of where to look for any feature.

### 1.3 `tests/` listing
- Content: rendered listing of `tests/` — `common/`, `compiletest.rs`, `crate/`, `drop/`, `test_autotrait.rs`, `test_backtrace.rs`, `test_boxed.rs`, `test_chain.rs`, `test_context.rs`, `test_convert.rs`, `test_downcast.rs`, `test_ensure.rs`, `test_ffi.rs`, `test_fmt.rs`, `test_macros.rs`, `test_repr.rs`, `test_source.rs`, `ui/`
- Cost: 72 tokens (helper: `echo '<files>' | count-tokens.py --stdin`)
- Notes: every behavior has a dedicated `test_*.rs`; `ui/` carries `trybuild` compile-fail tests; `crate/` is a downstream-style consumer used by the no-default-features CI matrix.

### 1.4 Top-level config trivia
- Content: `rust-toolchain.toml` (full, 2 lines — declares `components = ["rust-src"]`), `.gitignore` (full, 2 lines), `.github/FUNDING.yml` (full, 1 line)
- Cost: 22 tokens (helper: `count-tokens.py rust-toolchain.toml .gitignore .github/FUNDING.yml`)
- Notes: cheap to keep early — `rust-src` requirement explains why `cargo doc-rs` and `tests/crate/` show up; the rest is incidental.

### 1.5 Crate one-liner
- Content: `src/lib.rs:9-13` (the "This library provides…" paragraph from the crate-level rustdoc)
- Cost: 37 tokens (helper: `count-tokens.py src/lib.rs:9-13`)
- Notes: a 30-token sentence answering "what does this crate do?".

### 1.6 `Cargo.toml` package identity
- Content: `Cargo.toml:1-13` (the `[package]` block — name, version, authors, edition, MSRV `rust-version = "1.68"`, license, repository)
- Cost: 119 tokens (helper: `count-tokens.py Cargo.toml:1-13`)

### 1.7 `Cargo.toml` features and dependencies
- Content: `Cargo.toml:14-30` (the `[features]` block declaring `default = ["std"]` and the `std` feature, the `[dependencies]` with the optional `backtrace = "0.3.51"` carrying a comment about why it's a no-op on 1.65+, the `[dev-dependencies]` listing `futures`, `rustversion`, `syn`, `thiserror`, `trybuild`)
- Cost: 170 tokens (helper: `count-tokens.py Cargo.toml:14-30`)
- Notes: tells the agent that std is the only feature gate users ordinarily flip and that `thiserror`/`trybuild` are dev-only.

### 1.8 README usage example
- Content: `README.md:22-65` (the "Use `Result<T, anyhow::Error>`…" + "Attach context…" bullets from the README, the canonical 30-line "what does anyhow look like in code" snippet, plus the Caused-by output sample)
- Cost: 316 tokens (helper: `count-tokens.py README.md:22-65`)
- Notes: highest-value semantic teach for newcomers; if a user query says "how do I add context to an error", this batch alone answers it.

### 1.9 Elision marker for the rest of the crate-level docs
- Content: a single `…` standing in for the unshown remainder of `src/lib.rs:1-208` (badges, downcasting / backtrace / no-std bullets) and `README.md:1-179` (badges, downcasting / no-std / comparison sections)
- Cost: ~1 token
- Notes: tells the agent "more usage docs exist at the top of `lib.rs` and in README"; the lib.rs body is below the fold and the README extras are too.

---

### 2.1 `lib.rs` module skeleton
- Content: `src/lib.rs:209-244` (top-level attrs: `html_root_url`, feature gates, `#![no_std]`, `deny`/`allow` lints, `compile_error!` for nightly-testing) and `src/lib.rs:246-286` (`extern crate alloc;`, `extern crate std;`, the `mod backtrace; … mod wrapper;` block, the `use crate::error::ErrorImpl; use crate::ptr::Own;` re-imports, the cfg-cascaded `StdError` alias / fallback trait, and `pub use anyhow as format_err`)
- Cost: 520 tokens (helper: `count-tokens.py src/lib.rs:209-244 src/lib.rs:246-286`)
- Notes: shows the full module graph, the `#![no_std]` posture, the three-way cfg for what `StdError` resolves to (`std::error::Error` vs `core::error::Error` vs an in-crate fallback trait), and the `format_err = anyhow` alias. Together with 1.2 this is the structural skeleton.

### 2.2 Public-item declarations in `lib.rs`
- Content: `src/lib.rs:389-393` (`pub struct Error { inner: Own<ErrorImpl> }`), `src/lib.rs:413-417` (`pub struct Chain<'a> { state: ChainState<'a> }` with its cfg gate), `src/lib.rs:468` (`pub type Result<T, E = Error> = core::result::Result<T, E>;`), `src/lib.rs:616-628` (the `pub trait Context<T, E>: context::private::Sealed` definition with both method signatures), `src/lib.rs:648-652` (the capital-Ok helper signature)
- Cost: 257 tokens (helper: `count-tokens.py src/lib.rs:389-393 src/lib.rs:413-417 src/lib.rs:468 src/lib.rs:616-628 src/lib.rs:648-652`)
- Notes: the entire public type/trait surface declared in `lib.rs` in one compact batch. Pairs with 2.3 for the per-item rustdoc summary.

### 2.3 First-paragraph rustdoc on the public types
- Content: `src/lib.rs:288-296` ("The `Error` type, a wrapper around a dynamic error type" + the three-bullet summary: Send/Sync/'static, backtrace guarantee, narrow-pointer representation), `src/lib.rs:394-395` ("Iterator of a chain of source errors…"), `src/lib.rs:419-426` (`Result` summary + the "may be used with one or two type parameters" note), `src/lib.rs:630-636` ("Equivalent to `Ok::<_, anyhow::Error>(value)`…")
- Cost: 274 tokens (helper: `count-tokens.py src/lib.rs:288-296 src/lib.rs:394-395 src/lib.rs:419-426 src/lib.rs:630-636`)
- Notes: load-bearing — the "narrow pointer / one word in size" claim is the single most distinguishing fact about `anyhow::Error` and motivates everything in `error.rs` and `ptr.rs`.

### 2.4 Module-imports and `impl Error {` opening of `error.rs`
- Content: `src/error.rs:1-19` (the use-imports — `Backtrace`, `Chain`, `nightly::Request`, `Mut`/`Own`/`Ref`, `StdError`, plus the `impl Error {` line)
- Cost: 166 tokens (helper: `count-tokens.py src/error.rs:1-19`)
- Notes: tells the agent which internal modules `error.rs` pulls together — it's the integration site for `chain`/`ptr`/`backtrace`/`nightly`.

### 2.5 `__private` module shape
- Content: `src/lib.rs:654-728` (the entire `pub mod __private` block: `pub use` re-exports of `BothDebug`/`NotBothDebug`/`format`/`Err`/`concat`/`format_args`/`stringify`, the `kind` submodule re-exporting `AdhocKind`/`TraitKind`/`BoxedKind`, plus `format_err`, `must_use`, `not`, and the `Bool` autoref-trick trait)
- Cost: 650 tokens (helper: `count-tokens.py src/lib.rs:654-728`)
- Notes: this is what every macro expansion calls into; without it the `anyhow!`/`bail!`/`ensure!` expansions look mysterious. Predecessor for any deep dive into the macro definitions in 3.x.

---

### 3.1 `impl Error` method signatures with summary docs
- Content: for every public method in `src/error.rs:19-689`, the first `///` summary line + the attribute/`pub fn` signature lines, specifically:
  - `:20` + `:27-32` (`pub fn new<E>(error: E) -> Self where E: StdError + Send + Sync + 'static`)
  - `:38` + `:75-79` (`pub fn msg<M>(message: M) -> Self where M: Display + Debug + Send + Sync + 'static`)
  - `:84` + `:137-140` (`pub fn from_boxed(boxed_error: Box<dyn StdError + Send + Sync + 'static>) -> Self`)
  - `:316` + `:370-374` (`pub fn context<C>(self, context: C) -> Self where C: Display + Send + Sync + 'static`)
  - `:404` + `:431-432` (`pub fn backtrace(&self) -> &impl_backtrace!()`)
  - `:436` + `:457-459` (`pub fn chain(&self) -> Chain`)
  - `:463-464` + `:468-470` (`pub fn root_cause(&self) -> &(dyn StdError + 'static)`)
  - `:474` + `:482-485` (`pub fn is<E>(&self) -> bool`)
  - `:489` + `:490-492` (`pub fn downcast<E>(mut self) -> Result<E, Self>`)
  - `:518` + `:554-556` (`pub fn downcast_ref<E>(&self) -> Option<&E>`)
  - `:567` + `:568-570` (`pub fn downcast_mut<E>(&mut self) -> Option<&mut E>`)
  - `:582` + `:620-622` (`pub fn into_boxed_dyn_error(self) -> Box<dyn StdError + Send + Sync + 'static>`)
  - `:631` + `:661-665` (`pub fn reallocate_into_boxed_dyn_error_without_backtrace(self) -> Box<dyn StdError + Send + Sync + 'static>`)
- Cost: 701 tokens (helper: `count-tokens.py src/error.rs:20 src/error.rs:27-32 src/error.rs:38 src/error.rs:75-79 src/error.rs:84 src/error.rs:137-140 src/error.rs:316 src/error.rs:370-374 src/error.rs:404 src/error.rs:431-432 src/error.rs:436 src/error.rs:457-459 src/error.rs:463-464 src/error.rs:468-470 src/error.rs:474 src/error.rs:482-485 src/error.rs:489 src/error.rs:490-492 src/error.rs:518 src/error.rs:554-556 src/error.rs:567 src/error.rs:568-570 src/error.rs:582 src/error.rs:620-622 src/error.rs:631 src/error.rs:661-665`)
- Predecessor: 2.4
- Notes: the API table for `Error`. Combined with 2.2/2.3 the agent can answer "what can I call on an `anyhow::Error`?" with zero follow-up. Disjoint spans — render with elision markers between groups so it's coherent as a single batch.

### 3.2 `impl Error` trait impls
- Content: `src/error.rs:691-738` (`impl<E: StdError + Send + Sync + 'static> From<E> for Error`, `impl Deref/DerefMut for Error` to `dyn StdError + Send + Sync`, `impl Display/Debug for Error`, `impl Drop for Error`)
- Cost: 334 tokens (helper: `count-tokens.py src/error.rs:691-738`)
- Notes: explains why `?` works (`From<E>`) and why `error.source()` works (`Deref`). The manual `Drop` is the only non-trivial one (it dispatches via the vtable's `object_drop`).

### 3.3 `Context` impls — function signatures
- Content: `src/context.rs:1-8` (imports), `src/context.rs:42-46` (`impl<T, E> Context<T, E> for Result<T, E> where E: ext::StdError + Send + Sync + 'static`), `src/context.rs:90-93` (`impl<T> Context<T, Infallible> for Option<T>`), `src/context.rs:186-193` (the `private::Sealed` module with its two impls)
- Cost: 293 tokens (helper: `count-tokens.py src/context.rs:1-8 src/context.rs:42-46 src/context.rs:90-93 src/context.rs:186-193`)
- Notes: documents that `Context` is implemented for both `Result` and `Option<T>` — the latter is non-obvious and frequently surprises users.

### 3.4 `anyhow!` and `bail!` macros — doc + body
- Content: `src/macros.rs:1-29` (the `bail!` macro doc + first example), `src/macros.rs:56-68` (the `bail!` `macro_rules!` body — three arms: literal / expr / fmt), `src/macros.rs:174-201` (the `anyhow!` macro doc + first example), `src/macros.rs:202-223` (the `anyhow!` `macro_rules!` body — three arms)
- Cost: 700 tokens (helper: `count-tokens.py src/macros.rs:1-29 src/macros.rs:56-68 src/macros.rs:174-201 src/macros.rs:202-223`)
- Notes: makes `anyhow!` and `bail!` legible without reading the underlying `__anyhow!` / `must_use` / `format_err` plumbing. `ensure!` doc separately at 3.5 since its body is in `ensure.rs`.

### 3.5 `ensure!` macro doc + dispatcher arm
- Content: `src/macros.rs:70-124` (the `__ensure![]` wrapper macro carrying the `ensure!` doc + first two examples) and `src/macros.rs:155-172` (the `#[cfg(not(doc))]` arm that delegates to `__parse_ensure!`)
- Cost: 564 tokens (helper: `count-tokens.py src/macros.rs:70-124 src/macros.rs:155-172`)
- Notes: explains *what* `ensure!` does and reveals the `__parse_ensure!` indirection. The full parser body is below the fold.

### 3.6 Test function names across `tests/`
- Content: rendered as a per-file grouped list — for each of the 13 `tests/test_*.rs` files, the filename header followed by every `fn test_…` line truncated to `fn test_NAME(…`. Total of 69 test functions across the suite.
- Cost: 510 tokens (helper: `for f in tests/test_*.rs; do echo "$f"; grep '^fn test_' "$f" | sed 's/[(].*$/(…/'; done | count-tokens.py --stdin`)
- Notes: gives the agent a *menu* of behaviors that are exercised, so a query like "is there a test for downcasting after a context chain?" lands on `test_context.rs::test_downcast_ref` in one hop.

---

### 4.1 Full `chain.rs`
- Content: `src/chain.rs:1-102` (the entire file: `ChainState` enum with `Linked`/`Buffered`, `Chain::new`, `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `Default`)
- Cost: 654 tokens (helper: `count-tokens.py src/chain.rs`)
- Predecessor: 2.2
- Notes: complete, small, and load-bearing for anything involving `error.chain()`, `.rev()`, `.len()`. Reading end-to-end fits comfortably here.

### 4.2 Full `context.rs`
- Content: `src/context.rs:1-193` (the entire file: `ext::StdError` shim trait with two impls, the two `Context` impls in full, the `Debug`/`Display`/`StdError` impls for `ContextError`, the `Quoted` helper, the `private::Sealed` module)
- Cost: 1252 tokens (helper: `count-tokens.py src/context.rs`)
- Predecessor: 3.3
- Notes: explains how `?` produces context-wrapped errors (the `ext_context` indirection saves backtrace frames) and how `Debug` of a context-wrapped error renders.

### 4.3 Full `macros.rs`
- Content: `src/macros.rs:1-245` (entire file — `bail!`, the doc/non-doc `__ensure![]` wrappers, `anyhow!`, and the `__anyhow!` private companion)
- Cost: 1815 tokens (helper: `count-tokens.py src/macros.rs`)
- Predecessor: 2.5
- Notes: completes the macro story for anything except `__parse_ensure!`. The agent can edit/trace any of `anyhow!`/`bail!`/`ensure!` from this batch.

### 4.4 `lib.rs` `Error` doc — display/debug representations
- Content: `src/lib.rs:300-365` (the `# Display representations` section showing how `{}`, `{:#}`, `{:?}`, `{:#?}` render an error chain + backtrace, including the worked `Stack backtrace:` example with `<E as anyhow::context::ext::StdError>::ext_context` frames)
- Cost: 548 tokens (helper: `count-tokens.py src/lib.rs:300-365`)
- Notes: the canonical reference for "what does an anyhow error look like when printed" — load-bearing for any UX/log-output question.

### 4.5 `lib.rs` `Context` trait doc — example + effect on downcasting
- Content: `src/lib.rs:477-615` (the `# Example` `do_it`/`detach`/`fs::read` snippet showing `Context` in use, followed by the `# Effect on downcasting` section with both worked examples — adding context that doesn't break downcasts to the original error, and using context-as-machine-readable-tag)
- Cost: 998 tokens (helper: `count-tokens.py src/lib.rs:476-615`)
- Notes: explains the unusual property that adding `.context(...)` does *not* break downcasts to the underlying error type — one of `anyhow`'s defining design decisions.

### 4.6 `ErrorVTable`, `ErrorImpl`, `ContextError` definitions
- Content: `src/error.rs:740-754` (the `struct ErrorVTable` with seven function-pointer fields — `object_drop`, `object_ref`, `object_downcast`, `object_drop_rest` always-present, plus the cfg-gated `object_boxed`/`object_reallocate_boxed`/`object_backtrace`), `src/error.rs:933-955` (the `#[repr(C)] struct ErrorImpl<E = ()>` + the `vtable()` helper that exploits the "vtable is the first field" invariant + the `#[repr(C)] struct ContextError<C, E>`)
- Cost: 445 tokens (helper: `count-tokens.py src/error.rs:740-754 src/error.rs:933-955`)
- Predecessor: 3.1
- Notes: the central data structures of the implementation. The `repr(C)` + first-field invariants are documented inline and are critical to understand before reading any vtable function.

### 4.7 `Error::construct` core
- Content: `src/error.rs:287-314` (the unsafe `construct` function that boxes `ErrorImpl<E>` and erases it to `Own<ErrorImpl>` via cast, with the inline comment explaining the unsize-coercion analogy)
- Cost: 271 tokens (helper: `count-tokens.py src/error.rs:287-314`)
- Predecessor: 4.6
- Notes: the single function that ties together the vtable, the boxed ErrorImpl, and the narrow-pointer `Own` cast. Reading this answers "how is the one-word `Error` actually constructed".

### 4.8 Full `fmt.rs`
- Content: `src/fmt.rs:1-158` (`ErrorImpl::display`/`debug` rendering — the "Caused by:" loop, multi-cause numbering, backtrace stitching — plus the `Indented` writer with its three small unit tests)
- Cost: 979 tokens (helper: `count-tokens.py src/fmt.rs`)
- Predecessor: 4.4
- Notes: the implementation behind the formats described in 4.4. The unit tests double as worked examples of the indenting algorithm.

### 4.9 Backtrace cfg dispatch
- Content: `src/backtrace.rs:1-69` (the cfg cascade: `pub(crate) use std::backtrace::Backtrace` on stable 1.65+, the `backtrace`-crate fallback, the empty-enum `Backtrace` for the no-backtrace path; the `impl_backtrace!`/`backtrace!`/`backtrace_if_absent!` macros for each cfg combination)
- Cost: 401 tokens (helper: `count-tokens.py src/backtrace.rs:1-69`)
- Notes: load-bearing for understanding how the same `Error::backtrace()` API works across `std_backtrace`, `feature = "backtrace"`, `error_generic_member_access`, and the no-backtrace path.

### 4.10 `build.rs` probe overview
- Content: `build.rs:1-97` (the `main` function: cfg cascade for `error_generic_member_access` / `RUSTC_BOOTSTRAP`, the `cargo:rustc-cfg`/`rustc-check-cfg` emissions, the `rustc < 81` and `rustc < 85` cfg gates)
- Cost: 923 tokens (helper: `count-tokens.py build.rs:1-97`)
- Notes: the canonical reference for which cfg flags the rest of the codebase keys on. Without this batch the `#[cfg(error_generic_member_access)]` etc. gates scattered across `error.rs`/`backtrace.rs`/`nightly.rs` look unmotivated.

### 4.11 `test_fmt.rs` — display/debug expectations
- Content: `tests/test_fmt.rs:1-93` (the entire file — three nested fns `f`/`g`/`h` building a context chain over `io::Error`, plus the six `EXPECTED_*` string constants with the exact `g failed\n\nCaused by:\n    0: f failed\n    1: oh no!` formatting, plus the four `#[test]` cases)
- Cost: 569 tokens (helper: `count-tokens.py tests/test_fmt.rs`)
- Predecessor: 4.4
- Notes: these `EXPECTED_*` constants are the most precise specification of anyhow's debug/display behavior anywhere in the repo.

### 4.12 `test_context.rs` — drop-checked downcast suite
- Content: `tests/test_context.rs:1-172` (the full file: `mod drop;`, the `context_type!` macro, `HighLevel`/`MidLevel`/`LowLevel` types each carrying a `DetectDrop`, `make_chain()`, the `test_downcast_ref`/`_high`/`_mid`/`_low` cases, `test_unsuccessful_downcast`, `test_root_cause`)
- Cost: 1007 tokens (helper: `count-tokens.py tests/test_context.rs`)
- Predecessor: 3.2
- Notes: the canonical worked example of "build a 3-deep context chain, downcast through it, verify drop semantics." Covers the most failure-prone behavior.

### 4.13 `tests/drop/mod.rs` and `tests/common/mod.rs`
- Content: `tests/drop/mod.rs:1-53` (the `Flag`/`DetectDrop` helper used by drop-checked tests) and `tests/common/mod.rs:1-14` (`bail_literal`/`bail_fmt`/`bail_error` helpers)
- Cost: 355 tokens (helper: `count-tokens.py tests/drop/mod.rs tests/common/mod.rs`)
- Predecessor: 4.12
- Notes: the shared fixtures the test files refer to; without them the test bodies are partially opaque.

---

### 5.1 `error.rs` vtable functions
- Content: `src/error.rs:756-928` (the 12 type-erased vtable function bodies: `object_drop`, `object_drop_front`, `object_ref`, `object_boxed`, `object_reallocate_boxed`, `object_downcast`, `no_backtrace`, `context_downcast`, `context_drop_rest`, `context_chain_downcast`, `context_chain_drop_rest`, `context_backtrace`)
- Cost: 1761 tokens (helper: `count-tokens.py src/error.rs:756-928`)
- Predecessor: 4.6
- Notes: the unsafe core that makes the narrow-pointer trick work. Every function carries a "Safety: requires layout of *e to match …" comment.

### 5.2 `error.rs` `construct_from_*` family
- Content: `src/error.rs:145-285` (`construct_from_std`, `construct_from_adhoc`, `construct_from_display`, `construct_from_context`, `construct_from_boxed` — each builds an `ErrorVTable` literal with the appropriate type-monomorphized function pointers)
- Cost: 1369 tokens (helper: `count-tokens.py src/error.rs:145-285`)
- Predecessor: 4.7

### 5.3 `error.rs` `ErrorImpl` impls and conversion impls
- Content: `src/error.rs:957-1086` (the small `erase()` helper, `ErrorImpl::error`/`error_mut`/`backtrace`/`provide`/`chain`, the `StdError`/`Debug`/`Display` impls for `ErrorImpl<E>`, the three `From<Error> for Box<dyn StdError + …>` impls, the two `AsRef<dyn StdError…> for Error` impls, the `UnwindSafe`/`RefUnwindSafe` markers)
- Cost: 1032 tokens (helper: `count-tokens.py src/error.rs:957-1086`)
- Predecessor: 4.6

### 5.4 `ensure.rs` rendering preamble
- Content: `src/ensure.rs:1-101` (the `BothDebug`/`NotBothDebug` autoref-dispatch traits + the 40-byte fixed `Buf` writer + the `render(msg, lhs, rhs)` formatter that produces `"{msg} ({lhs} vs {rhs})"`)
- Cost: 665 tokens (helper: `count-tokens.py src/ensure.rs:1-101`)
- Predecessor: 3.5
- Notes: explains the "{lhs} vs {rhs}" failure-message format that surfaces in user-visible error strings.

### 5.5 `ensure.rs` macro entry/exit terminals
- Content: `src/ensure.rs:103-108` (the `__parse_ensure!` declaration line + the `(atom () $bail:tt …)` accept-arm that invokes `__fancy_ensure!`), `src/ensure.rs:884-908` (the `__fancy_ensure!` body that performs the comparison and calls `__dispatch_ensure`), `src/ensure.rs:910-935` (the `__fallback_ensure!` four arms used when expression parsing hits a low-precedence construct)
- Cost: 491 tokens (helper: `count-tokens.py src/ensure.rs:103-108 src/ensure.rs:884-908 src/ensure.rs:910-935`)
- Predecessor: 5.4
- Notes: shows the entry-point shape and both terminals (`__fancy_ensure!` for the structured `lhs OP rhs` case, `__fallback_ensure!` for everything else). Body of `__parse_ensure!` is below the fold.

## Below-the-fold

- **`src/lib.rs:1-208` — full crate-level rustdoc body** (1850 tokens; helper: `count-tokens.py src/lib.rs:1-208`). Largely overlaps with the README opening (1.8) and the type-specific rustdoc batches (2.3, 4.4, 4.5). The elision marker at 1.9 alerts the agent.
- **`README.md:1-21` and `README.md:67-179` — README badges + downcasting / no-std / comparison sections** (~1300 tokens). The same content as `src/lib.rs:1-208` from a different angle; one `Read README.md` away.
- **`src/ensure.rs:109-882` — the `__parse_ensure!` macro arms** (19185 tokens; helper: `count-tokens.py src/ensure.rs:109-882`). By far the largest single artifact in the fixture and the lowest-density per token: ~770 lines of token-tree-parser arms following a stylized state-machine pattern (`atom`/`cond`/`pat` states × every Rust expression form). The header comment + entry/terminal arms in 5.5 establish the shape; the body is one `Read src/ensure.rs:109-882` away when actually editing the parser. Including it would consume ~85% of the cap.
- **`src/backtrace.rs:70-405` — the `mod capture` body** (2406 tokens). Used only when the `backtrace` feature is enabled on a pre-1.65 compiler; modern users get `std::backtrace::Backtrace` and never touch this code. The cfg dispatch in 4.9 makes this discoverable; the body itself is conventional `backtrace`-crate glue.
- **`src/ptr.rs`** (956 tokens). The `Own`/`Ref`/`Mut` newtype family. Mechanical smart-pointer plumbing: each method is a one-liner casting `NonNull<T>` with a `PhantomData` lifetime tag. The vtable function bodies in 5.1 are perfectly readable without it; one `Read src/ptr.rs` away.
- **`src/kind.rs`** (825 tokens). The `AdhocKind`/`TraitKind`/`BoxedKind` autoref-dispatch traits used by `anyhow!($expr)`. The header comment is self-contained explanation of the technique; users editing `anyhow!` will visit `src/kind.rs` directly.
- **`src/wrapper.rs`** (569 tokens). The three `repr(transparent)` adapters (`MessageError`/`DisplayError`/`BoxedError`) with their `Debug`/`Display`/`StdError` impls. Trivial wrappers; named in 2.5 indirectly via `__private` re-exports.
- **`src/nightly.rs`** (414 tokens). The `core::error::Request` shim used only when the build script enables `error_generic_member_access`. Discoverable via the build-script overview in 4.10 and via the `#[cfg(error_generic_member_access)]` hits in `error.rs`.
- **`build.rs:99-207` — `compile_probe` and `rustc_minor_version`** (954 tokens). Implementation detail of the probe; the cfg outputs in 4.10 are what other code keys on. One `Read build.rs:99-207` away when actually modifying the probe.
- **`.github/workflows/ci.yml`** (1194 tokens). The CI matrix is descriptive of "which rustc versions are tested" but not load-bearing for code work; the `1.68.0` MSRV value is already in 1.6 (`Cargo.toml`).
- **`tests/test_ensure.rs:101-756` — the bulk of `__parse_ensure!`'s test coverage** (~6.6k tokens). Twenty-plus tests each driving one expression form through the parser. Useful only when modifying the parser; the test names from 3.6 cover the orientation need. The intro fixtures and recursion test are valuable but at ~600 tokens collectively they fall just outside the cap on a tight budget.
- **`tests/test_macros.rs`, `tests/test_downcast.rs`, `tests/test_chain.rs`, `tests/test_source.rs`, `tests/test_boxed.rs`, `tests/test_convert.rs`, `tests/test_repr.rs`, `tests/test_autotrait.rs`, `tests/test_ffi.rs`, `tests/test_backtrace.rs`** (~3.5k tokens combined). Test names are in 3.6; bodies are the standard `assert_eq!`-style verifications of behavior already documented in the rustdoc batches. The `Error: Send+Sync+Unpin` and `size_of::<Error>() == size_of::<usize>()` invariants in `test_autotrait.rs`/`test_repr.rs` are stated in 2.3.
- **`tests/compiletest.rs` + `tests/ui/*` bodies** (~1.2k tokens). The `tests/ui/` filenames (`chained-comparison`, `empty-ensure`, `ensure-nonbool`, `must-use`, `no-impl`, `temporary-value`, `wrong-interpolation`) self-describe what each compile-fail asserts; bodies are 5-10 lines of trivial `trybuild` boilerplate.
- **`tests/crate/Cargo.toml` + `tests/crate/test.rs`** (~100 tokens). The no_std smoke crate referenced in `.github/workflows/ci.yml`'s `1.68.0` build job. `test.rs` is just `#![no_std] pub use anyhow::*;`.
- **`LICENSE-APACHE`, `LICENSE-MIT`** (~3k tokens combined, standard text). Identified by name in 1.1.
- **The `Cargo.toml` `[package.metadata.docs.rs]` block** (lines 32-40, 89 tokens). Docs.rs config; not relevant to in-crate work.
