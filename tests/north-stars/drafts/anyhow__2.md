# anyhow — North Star

Revision pin: `769cba0b`

This fixture is a single-crate library (`anyhow` 1.0.101): a flexible concrete `Error` type built on `std::error::Error`, with `Context`, `Result`, and the `anyhow!` / `bail!` / `ensure!` macros. It supports `no_std` and uses a vtable-based type-erasure trick that keeps `Error` one word wide. The most load-bearing user-facing material is the public API in `src/lib.rs` and the README (which double as the usage manual). Internals worth surfacing: the vtable in `src/error.rs`, tagged-dispatch in `src/kind.rs`, the `Context` impls in `src/context.rs`, formatting in `src/fmt.rs`, the chain iterator in `src/chain.rs`, the unusual `ensure!` macro_rules parser in `src/ensure.rs`, and the build-script cfg-probing in `build.rs` that wires it all together.

## Batches

### 1.1 Crate identity (name + tagline + version + edition + MSRV)
- Content: `Cargo.toml:1-13` (`[package]` block: name "anyhow", description, version, edition, rust-version 1.68, repo URL).
- Cost: 119 tokens (helper: `count-tokens.py Cargo.toml:1-13`)
- Notes: Orients the agent in two seconds — what crate, what version, what the project is for.

### 1.2 Top-level directory listing
- Content: rendered listing of repo root entries (`src/`, `tests/`, `build.rs`, `Cargo.toml`, `README.md`, `LICENSE-APACHE`, `LICENSE-MIT`, `rust-toolchain.toml`, `.github/`, `.gitignore`).
- Cost: 34 tokens (helper: `printf '...' | count-tokens.py --stdin`)

### 1.3 `src/` file listing
- Content: rendered listing of the 12 modules in `src/` (`backtrace.rs`, `chain.rs`, `context.rs`, `ensure.rs`, `error.rs`, `fmt.rs`, `kind.rs`, `lib.rs`, `macros.rs`, `nightly.rs`, `ptr.rs`, `wrapper.rs`).
- Cost: 39 tokens (helper: `printf '...' | count-tokens.py --stdin`)

### 1.4 README headline pitch (with one usage example)
- Content: `README.md:9-37` — one-paragraph description plus the `Result<T, anyhow::Error>` + `?` example.
- Cost: 197 tokens (helper: `count-tokens.py README.md:9-37`)
- Notes: Fastest route to "what does anyhow look like in user code?"

### 1.5 Cargo features
- Content: `Cargo.toml:14-30` — `[features]` (`default = ["std"]`, `std`, optional `backtrace` dep with comment about it being unused on Rust ≥1.65).
- Cost: 170 tokens (helper: `count-tokens.py Cargo.toml:14-30`)
- Notes: Catastrophic-omission risk if missing — agent might invent a non-existent `backtrace` feature gate or miss that `default-features = false` is the no-std path.

### 1.6 Public API spine — items declared in `src/lib.rs`
- Content: `src/lib.rs:285-286` (`pub use anyhow as format_err`) + `src/lib.rs:288-298` (the short `Error works a lot like Box<dyn std::error::Error>` doc preamble explaining the three differences) + `src/lib.rs:389-392` (`#[repr(transparent)] pub struct Error { inner: Own<ErrorImpl> }`) + `src/lib.rs:413-417` (cfg-gated `pub struct Chain<'a>`) + `src/lib.rs:468-468` (`pub type Result<T, E = Error> = ...`) + `src/lib.rs:616-628` (`pub trait Context<T, E>` with the two trait-method signatures) + `src/lib.rs:648-652` (`pub fn Ok<T>`).
- Cost: 358 tokens (helper sum: 12 + 113 + 18 + 40 + 19 + 125 + 31)
- Notes: Catastrophic-omission baseline for the public type/trait/fn surface. The doc preamble at `:288-298` explains that `Error` is `Send+Sync+'static`, guarantees a backtrace, and is one word wide. Pair with 1.7 for macros.

### 1.7 Macro signatures (`anyhow!`, `bail!`, `ensure!`)
- Content: `src/macros.rs:56-68` (`macro_rules! bail` with its three arms), `src/macros.rs:127-153` (the `cfg(doc)` `__ensure!` definition giving the canonical `ensure!` arms), `src/macros.rs:202-223` (`macro_rules! anyhow` with its three arms).
- Cost: 511 tokens (helper sum: `macros.rs:56-68` 125 + `:127-153` 213 + `:202-223` 173)
- Notes: Without these the agent doesn't know macro arities/forms. The `cfg(doc)` ensure arms are the documentation-shape declaration; the real expansion is the gigantic parser (below the fold).

### 1.8 README — Context + downcast usage examples
- Content: `README.md:41-76` (`Context`/`with_context` example, output rendering, downcast-by-shared-ref example).
- Cost: 265 tokens (helper: `count-tokens.py README.md:41-76`)

### 2.1 `Error` constructor signatures (`new`, `msg`, `from_boxed`)
- Content: `src/error.rs:19-36` (`Error::new`), `src/error.rs:75-82` (`Error::msg`), `src/error.rs:137-143` (`Error::from_boxed`) — bare signatures with cfg/attrs and brief doc each.
- Cost: 225 tokens (helper sum: 84 + 57 + 84)
- Notes: The three most-used construction entry points. Pair with the `From<E>` impl (2.2).

### 2.2 `From<E> for Error` and `Deref/DerefMut/Display/Debug/Drop` impls
- Content: `src/error.rs:691-738`.
- Cost: 334 tokens (helper: `count-tokens.py src/error.rs:691-738`)
- Notes: The `From<E> for Error` impl is what makes `?` work. Crucial for understanding ergonomics.

### 2.3 `Error::context` (the inherent method, with rationale doc)
- Content: `src/error.rs:316-376` — the doc comment explaining when to use `error.context(...)` instead of the trait method, with a `parse` example, plus the signature.
- Cost: 509 tokens (helper: `count-tokens.py src/error.rs:316-376`)
- Notes: Distinct from `Context::context`. Both should be discoverable.

### 2.4 Downcast surface — `is`, `downcast`, `downcast_ref`, `downcast_mut`
- Content: `src/error.rs:473-487` (doc + `is`), `src/error.rs:489-493` (signature of `downcast`), `src/error.rs:554-557` (signature of `downcast_ref`), `src/error.rs:567-571` (signature of `downcast_mut`).
- Cost: ~270 tokens (helper sum: 144 + 49 + 34 + 47)

### 2.5 Chain navigation — `chain`, `root_cause`, `backtrace`
- Content: `src/error.rs:404-435` (doc + signature of `backtrace`), `src/error.rs:436-461` (doc + signature of `chain`), `src/error.rs:462-472` (doc + signature of `root_cause`).
- Cost: 700 tokens (helper sum: 383 + 206 + 111)

### 2.6 Boxed-conversion exits — `into_boxed_dyn_error`, `reallocate_into_boxed_dyn_error_without_backtrace`
- Content: `src/error.rs:619-622` (signature) + `src/error.rs:660-665` (signature).
- Cost: 125 tokens (helper sum: 66 + 59)
- Notes: Long names are self-documenting; full doc comments (`:582-615` and `:631-660`) live below the fold.

### 2.7 Crate-level configuration & module declarations
- Content: `src/lib.rs:209-264` — `#![doc(html_root_url)]`, `#![cfg_attr(error_generic_member_access, …)]`, `#![no_std]`, `#![deny(...)]`, `#![allow(...)]` clippy block, `compile_error!` for nightly-probe failure, `extern crate alloc`, `extern crate std` (cfg-gated), and the `mod` declarations (`backtrace`, `chain`, `context`, `ensure`, `error`, `fmt`, `kind`, `macros`, `nightly`, `ptr`, `wrapper`).
- Cost: 377 tokens (helper: `count-tokens.py src/lib.rs:209-264`)

### 2.8 `tests/` directory listing
- Content: rendered listing of test files (`test_autotrait.rs`, `test_backtrace.rs`, `test_boxed.rs`, `test_chain.rs`, `test_context.rs`, `test_convert.rs`, `test_downcast.rs`, `test_ensure.rs`, `test_ffi.rs`, `test_fmt.rs`, `test_macros.rs`, `test_repr.rs`, `test_source.rs`, `compiletest.rs`, `common/mod.rs`, `drop/mod.rs`, `crate/Cargo.toml`, `crate/test.rs`, `ui/`).
- Cost: 84 tokens (helper: `printf '...' | count-tokens.py --stdin`)

### 3.1 README — `bail!`/`anyhow!` + thiserror interop
- Content: `README.md:91-122` — thiserror interop block + `anyhow!` / `bail!` macro examples.
- Cost: 223 tokens (helper: `count-tokens.py README.md:91-122`)

### 3.2 README — no_std support section
- Content: `README.md:124-141`.
- Cost: 157 tokens (helper: `count-tokens.py README.md:124-141`)
- Notes: Tells the agent no_std works and what the gotcha is (Rust < 1.81 needs `.map_err(Error::msg)`).

### 3.3 README — comparison to thiserror & failure
- Content: `README.md:142-165`.
- Cost: 196 tokens (helper: `count-tokens.py README.md:142-165`)

### 3.4 `Error` Display-representations doc (the canonical formatting reference)
- Content: `src/lib.rs:299-388` — the long doc comment on `Error` showing `{}`, `{:#}`, `{:?}`, `{:#?}` rendered output examples.
- Cost: 701 tokens (helper: `count-tokens.py src/lib.rs:299-388`)
- Notes: The *only* place that actually shows what error messages look like. High value for any debugging/UX question.
- Predecessor: 1.6

### 3.5 `Context` trait — long doc on effect-on-downcasting
- Content: `src/lib.rs:475-615` — the doc between the `Context` trait header and its method declarations, including the two example functions showing context-vs-downcast use cases.
- Cost: 1002 tokens (helper: `count-tokens.py src/lib.rs:475-615`)
- Notes: Crucial for non-trivial usage; explains the non-obvious downcast invariant.
- Predecessor: 1.6

### 3.6 `Context` impls — `Context for Result<T, E>` and `Context for Option<T>`
- Content: `src/context.rs:1-113` (the `ext::StdError` shim, `Context for Result<T, E>`, `Context for Option<T>`).
- Cost: 745 tokens (helper sum: `:1-68` 453 + `:70-113` 292)
- Notes: Where `.context(...)` actually comes from (and how `Option<T>::context` works via `Infallible`).
- Predecessor: 1.6

### 3.7 `tests/common/mod.rs` and `tests/drop/mod.rs` (test helpers)
- Content: `tests/common/mod.rs:1-14` plus `tests/drop/mod.rs:1-53`.
- Cost: 355 tokens (helper sum: 81 + 274)
- Notes: Re-used by many tests; surfaces the `bail_*` helpers and the `DetectDrop`/`Flag` testing primitives.

### 4.1 Test function name index across `tests/test_*.rs` (excluding `test_ensure.rs`)
- Content: rendered output of `grep -nHE '^(#\[test\]|#\[rustversion::|fn test_|fn ui|pub extern "C" fn anyhow)' tests/test_autotrait.rs tests/test_backtrace.rs tests/test_boxed.rs tests/test_chain.rs tests/test_context.rs tests/test_convert.rs tests/test_downcast.rs tests/test_ffi.rs tests/test_fmt.rs tests/test_macros.rs tests/test_repr.rs tests/test_source.rs tests/compiletest.rs` (file:line: <line>).
- Cost: 1415 tokens (helper: pipe the grep output into `count-tokens.py --stdin`)
- Notes: Lets the agent jump precisely to any test scenario without grepping. `test_ensure.rs` test names are ranked separately in 6.9.

### 4.2 README — License block (closing)
- Content: `README.md:166-179`.
- Cost: 104 tokens (helper: `count-tokens.py README.md:166-179`)
- Notes: Cheap, settles license questions immediately.

### 4.3 `src/error.rs` — `ErrorVTable`, `ErrorImpl`, `ContextError` repr declarations
- Content: `src/error.rs:740-755` (`ErrorVTable` struct fields with cfg-gated entries) plus `src/error.rs:930-955` (`ErrorImpl<E = ()>` and `ContextError<C, E>` repr-C definitions with the layout-related comments).
- Cost: ~485 tokens (helper sum: 220 + 265)
- Notes: This is the *core trick* of anyhow — type-erasure via a manually-curated vtable so `Error` is one word wide. Without this, the unsafe code in error.rs reads as opaque magic.

### 4.4 `src/kind.rs` — tagged-dispatch design comment
- Content: `src/kind.rs:1-46` — the long header comment explaining autoref-based specialization (why `(&error).anyhow_kind().new(error)` picks the right impl).
- Cost: 373 tokens (helper: `count-tokens.py src/kind.rs:1-46`)
- Notes: Explains *why* `anyhow!(expr)` does the right thing whether `expr` is `&str`, `String`, `impl Error`, or `Box<dyn Error + Send + Sync>`. Predecessor: 1.7 (anyhow! arms call into `kind`).
- Predecessor: 1.7

### 4.5 `src/kind.rs` — `Adhoc`, `Trait`, `Boxed` types and their `new()` methods
- Content: `src/kind.rs:55-121`.
- Cost: 393 tokens (helper: `count-tokens.py src/kind.rs:55-121`)
- Notes: The three dispatch types `anyhow!` resolves to. Predecessor: 4.4.
- Predecessor: 4.4

### 4.6 `src/fmt.rs` — `ErrorImpl::display` and `ErrorImpl::debug`
- Content: `src/fmt.rs:1-67` (the `display`/`debug` impls including `Caused by:` rendering and backtrace appending logic).
- Cost: 502 tokens (helper: `count-tokens.py src/fmt.rs:1-67`)
- Notes: Predecessor: 4.3 (defines `ErrorImpl`).
- Predecessor: 4.3

### 4.7 `src/chain.rs` — Chain iterator (full file)
- Content: `src/chain.rs:1-102` (`ChainState` enum, `Chain::new`, `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `Default` impls).
- Cost: 654 tokens (helper: `count-tokens.py src/chain.rs`)
- Notes: Small enough to ship in full at this priority; explains how `error.chain()` walks the cause chain.

### 4.8 `src/wrapper.rs` — `MessageError`, `DisplayError`, `BoxedError`
- Content: full file `src/wrapper.rs:1-84`.
- Cost: 569 tokens (helper: `count-tokens.py src/wrapper.rs`)
- Notes: These three repr-transparent wrappers explain how `Error::msg`, `construct_from_display`, and `from_boxed` keep their underlying types downcastable.
- Predecessor: 4.3

### 4.9 `build.rs` — what the build script does (the `main` function)
- Content: `build.rs:1-97` — the cfg-emission `main`: probes for `error_generic_member_access`, emits `std_backtrace`, `error_generic_member_access`, `anyhow_no_core_error`, `anyhow_no_clippy_format_args`, `anyhow_nightly_testing`, `anyhow_build_probe` cfgs.
- Cost: ~920 tokens (helper sum: 102 + 821)
- Notes: Critical for understanding why all the `cfg(...)` gates throughout the crate exist and what they mean.

### 4.10 `src/backtrace.rs` — backtrace abstraction header (cfg dance + macros)
- Content: `src/backtrace.rs:1-68` — three `pub(crate) use` aliases, `impl_backtrace!`, `backtrace!`, and `backtrace_if_absent!` macro definitions across the std/feature/no-backtrace configurations.
- Cost: 401 tokens (helper: `count-tokens.py src/backtrace.rs:1-68`)
- Notes: The three-way fork: `std::backtrace`, `backtrace` crate (vendored capture below the fold), or unit-type fallback.

### 5.1 `tests/test_fmt.rs` — full file (Display/Debug expected outputs)
- Content: `tests/test_fmt.rs:1-93`.
- Cost: 569 tokens (helper: `count-tokens.py tests/test_fmt.rs`)
- Notes: The `EXPECTED_*` const strings are the *authoritative* spec for what error formatting produces. Catastrophic-omission risk: without this, the agent might guess wrong about `Caused by:` formatting. Pairs with 3.4 and 4.6.

### 5.2 `tests/test_repr.rs` and `tests/test_autotrait.rs` (full)
- Content: `tests/test_repr.rs:1-30` plus `tests/test_autotrait.rs:1-34`.
- Cost: 343 tokens (helper sum: 169 + 174)
- Notes: Asserts `size_of::<Error>() == size_of::<usize>()`, null-pointer optimization holds, and `Error: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe`. Surfacing avoids wrong claims.

### 5.3 `tests/test_source.rs` (full)
- Content: `tests/test_source.rs:1-62`.
- Cost: 368 tokens (helper: `count-tokens.py tests/test_source.rs`)
- Notes: Demonstrates exactly which `anyhow!` invocations preserve `source()` and which don't (literal/variable/fmt vs `impl StdError`). Avoids subtle bugs.

### 5.4 `tests/test_backtrace.rs` (full)
- Content: `tests/test_backtrace.rs:1-15`.
- Cost: 77 tokens (helper: `count-tokens.py tests/test_backtrace.rs`)
- Notes: Tiny; documents the `#[rustversion::nightly]` gating pattern used elsewhere too.

### 6.1 `src/error.rs` — first-three constructor wirings (`construct_from_std`, `construct_from_adhoc`, `construct_from_display`)
- Content: `src/error.rs:144-225`.
- Cost: 780 tokens (helper: `count-tokens.py src/error.rs:144-225`)
- Notes: How standard / ad-hoc / display-only errors get a vtable. Predecessor: 4.3.
- Predecessor: 4.3

### 6.2 `src/error.rs` — `construct_from_context`, `construct_from_boxed`, and `unsafe fn construct`
- Content: `src/error.rs:226-314`.
- Cost: 861 tokens (helper: `count-tokens.py src/error.rs:226-314`)
- Notes: The context-wrapping constructor and the `unsafe fn construct` that erases the type into the thin pointer. Predecessor: 6.1.
- Predecessor: 6.1

### 6.3 `src/error.rs` — `downcast`/`downcast_ref`/`downcast_mut` bodies
- Content: `src/error.rs:489-580`.
- Cost: 766 tokens (helper: `count-tokens.py src/error.rs:489-580`)
- Notes: Unsafe vtable-driven downcast bodies. Predecessor: 4.3, 2.4.
- Predecessor: 4.3

### 6.4 `src/error.rs` — `ErrorImpl` impls + the `vtable()` accessor
- Content: `src/error.rs:957-1013` (`ErrorImpl::erase`, `error/error_mut`, `backtrace`, `provide`, `chain`, `unsafe fn vtable(p)`).
- Cost: 526 tokens (helper: `count-tokens.py src/error.rs:957-1013`)
- Notes: Predecessor: 4.3, 6.1.
- Predecessor: 6.1

### 6.5 `src/nightly.rs` — generic-member-access probe
- Content: `src/nightly.rs:1-58`.
- Cost: 414 tokens (helper: `count-tokens.py src/nightly.rs`)
- Notes: The compiled-as-build-probe file; explains what the `build.rs` probe is testing for.
- Predecessor: 4.9

### 6.6 `src/macros.rs` — `bail!` doc, `anyhow!` doc, `__anyhow!` private macro
- Content: `src/macros.rs:1-55` (`bail!` doc with the two examples) + `src/macros.rs:174-201` (`anyhow!` doc + lookup example) + `src/macros.rs:225-245` (`__anyhow!` private macro).
- Cost: 729 tokens (helper sum: 337 + 213 + 179)
- Predecessor: 1.7

### 6.7 `src/macros.rs` — the `__ensure!` doc-only wrapper + the real `ensure!` dispatch
- Content: `src/macros.rs:70-124` (the `__ensure!` wrapper carrying the doc comment, with cfg(doc) variant) + `src/macros.rs:155-172` (the cfg(not(doc)) variant that delegates to `__parse_ensure!`).
- Cost: 572 tokens (helper sum: 422 + 150)
- Predecessor: 1.7

### 6.8 `src/ensure.rs` — `__fancy_ensure!` and `__fallback_ensure!` macros
- Content: `src/ensure.rs:884-935`.
- Cost: 409 tokens (helper: `count-tokens.py src/ensure.rs:884-935`)
- Notes: The two terminal macros invoked by the `__parse_ensure!` parser. Helpful even without the parser body, because it shows what the parser ultimately produces.

### 6.9 `tests/test_ensure.rs` — preamble + test fn signature index
- Content: `tests/test_ensure.rs:1-39` (allow attrs + `use`s + helper struct/trait definitions) plus rendered listing of all `#[test] fn test_*` lines from this file (`grep -nE '^(#\[test\]|fn test_)' tests/test_ensure.rs`).
- Cost: 556 tokens (helper sum: preamble 308 + fn-signature index 248)
- Notes: This file is a *spec* for what the `ensure!` parser must accept (closures, control flow, paths, raw addrs, etc.). Preamble shows the operator/trait setup; the fn-signature index lets the agent jump to any specific scenario.

### 6.10 `tests/ui/` — directory listing
- Content: rendered `ls` of `tests/ui/` (14 entries: 7 `.rs` + 7 `.stderr`).
- Cost: 70 tokens (helper: `ls tests/ui/ | count-tokens.py --stdin`)
- Notes: Surfaces that trybuild compile-fail cases exist; per-test inputs are mostly tiny (22-173 tokens each).

### 6.11 `tests/ui/ensure-nonbool.rs` (one full ui-test as exemplar)
- Content: `tests/ui/ensure-nonbool.rs:1-39`.
- Cost: 173 tokens (helper: `count-tokens.py tests/ui/ensure-nonbool.rs`)
- Notes: Concrete shape of a trybuild input — the largest, most informative one. The `Deref<Target=bool>` and `Not<Output=bool>` setup shows what `ensure!(expr)` does and doesn't accept.
- Predecessor: 6.10

### 6.12 `.github/workflows/ci.yml` — CI matrix and probes
- Content: `.github/workflows/ci.yml:1-50` (env, `pre_ci`, `test` job with rustc matrix `[nightly, beta, stable, 1.82.0, 1.80.0, 1.76.0]` and the `cfg=anyhow_nightly_testing` flag).
- Cost: 403 tokens (helper: `count-tokens.py .github/workflows/ci.yml:1-50`)
- Notes: Documents which rustc versions are supported in CI vs MSRV from `Cargo.toml`.

### 6.13 `tests/crate/` mini-crate (no_std smoke test)
- Content: `tests/crate/Cargo.toml:1-17` plus `tests/crate/test.rs:1-3`.
- Cost: 103 tokens (helper sum: 94 + 9)
- Notes: This tiny `#![no_std] pub use anyhow::*;` crate is what `cargo check --manifest-path tests/crate/Cargo.toml` in CI exercises against the 1.68 MSRV.

### 6.14 `tests/compiletest.rs`
- Content: `tests/compiletest.rs:1-7`.
- Cost: 60 tokens (helper: `count-tokens.py tests/compiletest.rs`)
- Notes: Wires up trybuild on `tests/ui/*.rs`.
- Predecessor: 6.10

## Below-the-fold

The fixture is small (~22k tokens of source content total) but a few large chunks are deeply internal or duplicate ranked content; cumulative ranked content is ~21k tokens, slightly over the soft 20k cap.

- **`src/lib.rs:14-189`** (~1460 tokens) — crate-level rustdoc that mirrors README sections almost line-for-line, just with hidden-doctest scaffolding (`# ...`). README (already ranked) carries the same content more compactly.
- **`src/lib.rs:1-13`** (218 tokens) — badges + html_root_url. Cosmetic; identity is in 1.1.
- **`src/lib.rs:654-728`** (410 tokens) — `__private` non-public module body (`Bool` trait, `format_err`, `must_use`, `not`, kind re-exports). Touched only by macro-generated code; one `Read` away when editing macros.
- **`src/error.rs:84-143`** (537 tokens) — long `from_boxed` doc comment with the `Report` ↔ `anyhow::Error` interop skeleton. Pattern is unique but `from_boxed` is already named in 2.1; one `Read` away.
- **`src/error.rs:582-615`** + **`:631-660`** (~740 tokens combined) — the `into_boxed_dyn_error` and `reallocate_into_boxed_dyn_error_without_backtrace` doc comments with their `error_generic_member_access` doctest examples. Pattern is unusual but the methods are surfaced in 2.6.
- **`src/error.rs:756-928`** (1761 tokens) — vtable trampolines (`object_drop`, `object_ref`, `object_boxed`, `object_reallocate_boxed`, `object_downcast`, `context_downcast`, `context_drop_rest`, `context_chain_downcast`, `context_chain_drop_rest`, `context_backtrace`, `no_backtrace`). Internals only; structure is captured by 4.3 (vtable shape) + 6.1/6.2 (constructor wirings).
- **`src/error.rs:1015-1045`** (199 tokens) — `StdError`/`Debug`/`Display` impls on `ErrorImpl<E>`. Forwarders only; the interesting versions are on `ErrorImpl` (no `<E>`) in 6.4.
- **`src/error.rs:1047-1086`** (304 tokens) — `From<Error>` to `Box<dyn StdError + Send + Sync>` / `Box<dyn StdError + Send>` / `Box<dyn StdError>` plus `AsRef<dyn StdError + Send + Sync>` / `AsRef<dyn StdError>` plus `UnwindSafe`/`RefUnwindSafe`. All thin trampolines to `into_boxed_dyn_error`.
- **`src/ptr.rs`** (956 tokens) — `Own<T>`, `Ref<'a,T>`, `Mut<'a,T>` thin-pointer wrappers. Unsafe pointer plumbing; relevant only when modifying `error.rs`. Type names appear in 4.3 and 1.6 ("`inner: Own<ErrorImpl>`").
- **`src/ensure.rs:1-101`** (665 tokens) — `BothDebug`/`NotBothDebug` traits, `Buf`, `render` function. The runtime side of `ensure!`'s `(2 vs 1)` rendering. Useful only when debugging `ensure!` failure messages; one `Read` from 6.8.
- **`src/ensure.rs:103-883`** (≈19,300 tokens) — the gigantic `__parse_ensure!` token-tree parser (~780 lines of `macro_rules!` arms covering operators, control flow, paths, types, patterns). Single-handedly larger than the entire ranking budget; would also wildly violate the 2× growth constraint at any reasonable position. Shape is fully captured by 6.7 (dispatch entry), 6.8 (terminal macros), and 6.9 (the test spec).
- **`src/backtrace.rs:70-410`** (≈2430 tokens) — `mod capture` containing the vendored `backtrace` crate integration (`Backtrace`/`BacktraceFrame`/`BacktraceSymbol`/`BytesOrWide`/`LazilyResolvedCapture`/`output_filename`). Only relevant when modifying anyhow's vendored backtrace path; gated behind `feature = "backtrace"` which is unused on Rust ≥1.65 per `Cargo.toml`.
- **`build.rs:99-207`** (~954 tokens) — `compile_probe`, `rustc_minor_version`, `cargo_env_var`. Implementation details of the build script's nightly probe; what it does is captured in 4.9, what it probes for is captured in 6.5.
- **`.github/workflows/ci.yml:51-151`** (~790 tokens) — `build` (1.68 MSRV check), `minimal`, `windows`, `doc`, `clippy`, `miri`, `outdated` jobs. Useful only when changing CI configuration.
- **`tests/test_chain.rs`** (581 tokens), **`tests/test_macros.rs`** (618 tokens), **`tests/test_context.rs`** (1007 tokens full — `make_chain`, `context_type!` macro, downcast tests), **`tests/test_downcast.rs`** (791 tokens), **`tests/test_boxed.rs`** (290 tokens), **`tests/test_convert.rs`** (314 tokens), **`tests/test_ffi.rs`** (126 tokens) — full bodies. Function names already indexed in 4.1 with file:line; bodies are one `Read` away when investigating a specific scenario.
- **`tests/ui/*.rs` other than `ensure-nonbool.rs`** + **all `tests/ui/*.stderr`** — small (22-59 tokens each) compile-fail fixtures and their generated stderr; structure captured by 6.10.
- **`Cargo.toml:32-40`** (89 tokens) — `[package.metadata.docs.rs]` rustdoc args; not load-bearing for development.
- **`LICENSE-APACHE`, `LICENSE-MIT`** — license text. License identity is in 1.1 + 4.2.
- **`.github/FUNDING.yml`, `rust-toolchain.toml`, `.gitignore`** — trivial single-line files; presence is conveyed by 1.2.
