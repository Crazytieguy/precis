# North Star — `anyhow`

**Fixture**: `test/fixtures/anyhow` &middot; **Revision**: `769cba0b`

`anyhow` (David Tolnay, v1.0.101) is a small Rust crate that provides `anyhow::Error`, a single-pointer-wide trait-object error type for application-level error handling. The crate is `no_std`-compatible (with `alloc`), exports the `anyhow!` / `bail!` / `ensure!` macros and a `Context` extension trait, and supports preserved-cause-chain downcasting and a fallback-captured backtrace. Internally it is built around a hand-rolled vtable that erases the original error type while keeping a thin pointer; this is the load-bearing trick that makes `size_of::<Error>() == size_of::<usize>()` and `Result<(), Error>` niche-optimized to one word. The 6-line crate (`tests/crate/test.rs`, `pub use anyhow::*`) plus a Cargo manifest in `tests/crate/` exists so CI can probe MSRV (Rust 1.68) under the `no_std` configuration. A `build.rs` probe decides at compile time which of three backtrace strategies to wire up: nightly `error_generic_member_access`, stable `std::backtrace::Backtrace`, or the optional `backtrace` crate (or none, on `no_std`).

This North Star ranks what an agent must see, in priority order, to act on an arbitrary task in this repo without follow-up reads. Total ranked content ≈ 19.8k tokens (Tiers 1–6).

---

## Tier 1 — Crate identity & user-facing macro API

What the crate *is*, what it offers, and the three macros every user touches first. Everything below presupposes this.

### 1.1 README narrative — `README.md` lines 1–179 — **1610 tok**
The full README. Short prose, idiomatic examples for `Result`/`?`/`.context(...)`/`.with_context(|| …)`, downcasting via `root_cause.downcast_ref::<DataStoreError>()`, the `RUST_BACKTRACE` / `RUST_LIB_BACKTRACE` env-var matrix, the comparison-to-`thiserror` paragraph (use anyhow when you don't care what error type comes back; use thiserror when you're a library designing your own), and the `no_std` instructions. This is the canonical "what is this" answer.

### 1.2 Cargo manifest — `Cargo.toml` lines 1–40 — **378 tok**
Edition 2021, MSRV 1.68, `default = ["std"]`, optional `backtrace = "0.3.51"` dep, dev-deps on `futures` / `rustversion` / `syn` / `thiserror` / `trybuild`, and the `[package.metadata.docs.rs]` rustdoc-args (most notably `--generate-link-to-definition` and `--generate-macro-expansion`).

### 1.3 Module map — `src/lib.rs` lines 251–263 — **45 tok**
The 11-module structure: `backtrace` (`#[macro_use]` for the `backtrace!` / `backtrace_if_absent!` macros), `chain`, `context`, `ensure`, `error`, `fmt`, `kind`, `macros`, `nightly` (cfg-gated), `ptr`, `wrapper`. Use this to navigate.

### 1.4 Public-item declaration lines — `src/lib.rs` lines 285–286, 389–392, 413–417, 468, 616–628, 648–652 — **290 tok**
The bare definitions every downstream import resolves to:
- `pub use anyhow as format_err;` (alias re-export)
- `#[repr(transparent)] pub struct Error { inner: Own<ErrorImpl> }`
- `pub struct Chain<'a> { state: crate::chain::ChainState<'a> }` (cfg-gated on `feature = "std"` or non-`anyhow_no_core_error`)
- `pub type Result<T, E = Error> = core::result::Result<T, E>;` (the `E = Error` default is the load-bearing detail)
- `pub trait Context<T, E>: context::private::Sealed` with `fn context<C>` and `fn with_context<C, F>` signatures and their `C: Display + Send + Sync + 'static` bounds
- `pub fn Ok<T>(value: T) -> Result<T>` (the inference helper)

### 1.5 `bail!` macro with rustdoc — `src/macros.rs` lines 1–69 — **462 tok**
Rustdoc + `#[macro_export]` definition. Three arms: `($msg:literal)`, `($err:expr)`, `($fmt:expr, $($arg:tt)*)`. Documents the `Result<_, anyhow::Error>` return-type requirement.

### 1.6 `anyhow!` and `__anyhow!` macros — `src/macros.rs` lines 174–245 — **565 tok**
The user-visible `anyhow!` macro (with full rustdoc) plus the internal `__anyhow!` used by `bail!`/`ensure!` to skip the `must_use` wrap. Reading these together makes the dispatch (`(&error).anyhow_kind().new(error)`) legible.

### 1.7 `ensure!` user-facing form — `src/macros.rs` lines 70–153 — **639 tok**
The `cfg(doc)` branch of `__ensure![...]` that documents `ensure!` for rustdoc. Shows the four conceptual arms (`cond` / `cond, msg` / `cond, err` / `cond, fmt, args…`) and that `ensure!` returns `Error` rather than panicking like `assert!`. The actual non-doc implementation (lines 155–172) is a single `($($tt:tt)*)` arm that delegates to the `__parse_ensure!` TT-muncher in `src/ensure.rs`; an agent only needs to know this delegation exists, not the muncher itself, unless modifying parsing.

**Tier 1 subtotal: ~3989 tokens**

---

## Tier 2 — Crate rustdoc with semantic specs

The rustdoc on `lib.rs` is not duplicative of the README; it specifies *behavior contracts* — display representations, downcasting interactions with context, the size guarantee — that any task touching observable behavior depends on.

### 2.1 Crate-level rustdoc — `src/lib.rs` lines 1–208 — **1850 tok**
The `//!` block. Same examples as README but as compilable doctests (note the `# pub trait Deserialize {}` etc. hidden lines). Includes the `no_std` / `map_err(Error::msg)` workaround note for pre-1.81 toolchains. The doctest hidden lines (`# struct ClusterMap;` etc.) are part of the test suite; modifying examples requires updating these.

### 2.2 Crate attributes — `src/lib.rs` lines 209–249 — **332 tok**
`#![doc(html_root_url = "https://docs.rs/anyhow/1.0.101")]`, `#![cfg_attr(error_generic_member_access, feature(...))]`, `#![no_std]`, the `extern crate alloc;` and conditional `extern crate std;`, the `compile_error!` guard for `anyhow_nightly_testing`, the in-crate `trait StdError` shim used when neither `std` nor `core::error::Error` is available, the `#![deny(dead_code, unsafe_op_in_unsafe_fn, unused_imports, unused_mut)]` (so unused vtable arms must be `#[cfg]`-gated, not silently dead), and the long `clippy::*` allow list. The `unsafe_op_in_unsafe_fn` deny is what forces every `unsafe { … }` inside an `unsafe fn` throughout the crate.

### 2.3 `Error` rustdoc — `src/lib.rs` lines 288–388 — **830 tok**
Specifies the four-way display matrix: `{}` shows only the outermost; `{:#}` chains causes with `: ` separators; `{:?}` produces the `Caused by:` block plus `Stack backtrace:` if captured; `{:#?}` is conventional struct-Debug. Includes the worked `try_main` example for hand-rolled rendering. This rustdoc is a contract — `tests/test_fmt.rs` in Tier 6 asserts each format byte-for-byte.

### 2.4 `Result` alias rustdoc — `src/lib.rs` lines 419–467 — **353 tok**
Documents the one-or-two-type-parameter trick (`E = Error` default) and shows the canonical `fn main() -> Result<()>` pattern.

### 2.5 `Context` trait rustdoc with downcasting effect — `src/lib.rs` lines 469–615 — **1157 tok**
Defines `Context` as **sealed**, gives the `ImportantThing::detach` worked example, points at `Error#display-representations`, and — most importantly — the **"Effect on downcasting"** section: after attaching context `C` onto error `E`, the resulting `anyhow::Error` is downcastable to **either** `C` **or** `E`. This bidirectionality is the contract that `error.rs`'s `context_chain_downcast` and `context_downcast` functions implement, and that `tests/test_context.rs` enforces.

**Tier 2 subtotal: ~4522 tokens**

**Ordering constraint within T2**: 2.1 (crate doc) before 2.2 (attrs) — the doc block precedes the attribute block in the file, and the doc is what most readers want. 2.3–2.5 are siblings under the same parent (`src/lib.rs`).

---

## Tier 3 — Public method signatures + sibling trait impls on `Error`

Once an agent knows what `Error` *is*, they need to know what it *does*. This tier is the menu of operations.

### 3.1 `impl Error` — public-method signatures with one-line summaries — `src/error.rs` line 19; lines 20+27–32, 38+75–79, 84+137–140, 316+370–374, 404+431–432, 436+457–459, 463–464+468–470, 474–484, 489–492, 518+554–556, 567+568–570, 582+620–622, 631+661–664 — **~765 tok**
The 13 user-callable methods of `Error`, each rendered as `[first sentence of rustdoc] + cfg attrs + signature` (truncated before the body):
- `pub fn new<E>(error: E) -> Self where E: StdError + Send + Sync + 'static`
- `pub fn msg<M>(message: M) -> Self where M: Display + Debug + Send + Sync + 'static`
- `pub fn from_boxed(boxed_error: Box<dyn StdError + Send + Sync + 'static>) -> Self`
- `pub fn context<C>(self, context: C) -> Self where C: Display + Send + Sync + 'static`
- `pub fn backtrace(&self) -> &impl_backtrace!()` (cfg `std_backtrace` or `feature = "backtrace"`)
- `pub fn chain(&self) -> Chain`
- `pub fn root_cause(&self) -> &(dyn StdError + 'static)`
- `pub fn is<E>(&self) -> bool`
- `pub fn downcast<E>(mut self) -> Result<E, Self>`
- `pub fn downcast_ref<E>(&self) -> Option<&E>`
- `pub fn downcast_mut<E>(&mut self) -> Option<&mut E>`
- `pub fn into_boxed_dyn_error(self) -> Box<dyn StdError + Send + Sync + 'static>`
- `pub fn reallocate_into_boxed_dyn_error_without_backtrace(self) -> Box<...>`

Plus the cfg attributes (`#[cfg(any(feature = "std", not(anyhow_no_core_error)))]` on most of them, `#[cold]`, `#[must_use]`) — these decide which methods exist in pure `core::error::Error` mode.

### 3.2 Standard trait impls on `Error` — `src/error.rs` lines 691–738 — **334 tok**
`impl<E: StdError + Send + Sync + 'static> From<E> for Error` (this is what powers `?`), `impl Deref for Error` (target = `dyn StdError + Send + Sync + 'static`), `impl DerefMut`, `impl Display`, `impl Debug`, `impl Drop` (the vtable's `object_drop` fan-out happens here).

### 3.3 Boxed-error and AsRef conversions — `src/error.rs` lines 1047–1086 — **304 tok**
Three `impl From<Error> for Box<dyn StdError + …>` variants (with/without `Send`/`Sync`), two `impl AsRef<dyn StdError + …> for Error`, and the `unsafe impl UnwindSafe for Error` / `unsafe impl RefUnwindSafe for Error` markers.

### 3.4 `Chain` iterator — `src/chain.rs` lines 1–103 — **654 tok**
Full file. `enum ChainState<'a> { Linked { next }, Buffered { rest } }` (with the `Buffered` arm cfg-gated), `Chain::new`, `impl Iterator`, `impl DoubleEndedIterator` (the trick: when reversed, eagerly drain into a `Vec` and switch state to `Buffered`), `impl ExactSizeIterator` (forward `len()` walks the chain), `impl Default`. This is small, complete, self-contained, and exactly the spec the chain tests exercise.

### 3.5 `Context` impls — `src/context.rs` lines 9–113 — **~576 tok**
The `mod ext { pub trait StdError { fn ext_context… } }` indirection (so `Result<T, Error>` and `Result<T, E: StdError>` can both implement `Context` without overlap), the `impl<T, E> Context<T, E> for Result<T, E> where E: ext::StdError + Send + Sync + 'static` (with the comment "Not using map_err to save 2 useless frames off the captured backtrace"), and `impl<T> Context<T, Infallible> for Option<T>`. The `Infallible` here is what makes `option.context("…")` work — the `E` parameter is uninhabited so there's nothing to convert from.

### 3.6 `private::Sealed` — `src/context.rs` lines 186–193 — **50 tok**
The sealing pattern used to make `Context` un-implementable downstream. Tiny but contractually important.

**Tier 3 subtotal: ~2683 tokens**

**Ordering constraint within T3**: 3.1 before 3.2 before 3.3 (same parent file `src/error.rs`, same order they appear). 3.4–3.6 are siblings under different parents; rank `Chain` before `Context` impls because `Chain` is exposed in `Error::chain()`'s return type from 3.1.

---

## Tier 4 — Internal architecture: the vtable trick

This tier is what an agent needs whenever the task touches "how does anyhow actually work?" — adding a new construction path, modifying drop semantics, debugging downcasting, changing the layout. It is unusually load-bearing for an internal-implementation tier because the unsafe vtable trick **is** the crate.

### 4.1 `ErrorVTable` definition — `src/error.rs` lines 740–754 — **220 tok**
The seven function pointers: `object_drop`, `object_ref`, `object_boxed`, `object_reallocate_boxed`, `object_downcast`, `object_drop_rest`, `object_backtrace` (the last three cfg-gated). The cfg gating on `object_boxed`/`object_reallocate_boxed` mirrors the `Error::into_boxed_dyn_error` cfg.

### 4.2 `ErrorImpl<E>` and `ContextError<C, E>` layouts — `src/error.rs` lines 933–955 — **225 tok**
Both `#[repr(C)]` with detailed comments explaining *why* — `ErrorImpl` so the vtable pointer stays at offset 0 (read by the bare `vtable(p)` function without going through a reference), and `ContextError` so `ContextError<C, E>`, `ContextError<ManuallyDrop<C>, E>`, and `ContextError<C, ManuallyDrop<E>>` share a layout (used during `downcast` to take ownership of one field while dropping the other).

### 4.3 `ErrorImpl` methods + `StdError`/`Debug`/`Display` impls — `src/error.rs` lines 957–1045 — **728 tok**
`ErrorImpl::erase` (`Ref::new(self).cast::<ErrorImpl>()`, the analog of an unsize coercion to a thin pointer), `ErrorImpl::error` / `error_mut` (vtable-driven dyn StdError reconstruction), `ErrorImpl::backtrace` (with the `expect("backtrace capture failed")` and the long comment about why the unwrap is only reachable via maliciously-constructed code), `ErrorImpl::provide` (nightly only), `ErrorImpl::chain`, and the three forwarding trait impls. The `vtable(this.ptr)` calls here are the entire reason the type-erasure works.

### 4.4 Smart-pointer scaffolding — `src/ptr.rs` lines 1–187 — **956 tok**
Full file. `Own<T>` (unique owning thin pointer, `Send + Sync` with manual unsafe impls, `Copy` because the wrapping `Error` enforces ownership through `Drop`), `Ref<'a, T>`, `Mut<'a, T>` (lifetime-tagged shared/mutable variants), `cast::<U>()` via the `CastTo` trait (forces turbofish so accidental implicit casts can't compile). These three types are the discipline that lets the vtable manipulate `ErrorImpl<E>` for arbitrary `E` through a single `NonNull<ErrorImpl>`.

### 4.5 Type-erased wrappers — `src/wrapper.rs` lines 1–84 — **569 tok**
Full file. `MessageError<M>` and `DisplayError<M>` (both `#[repr(transparent)]`, used so `construct_from_adhoc` and `construct_from_display` can hand back a `dyn StdError` for things that don't already impl it), and `BoxedError` (so a pre-boxed `Box<dyn StdError + Send + Sync>` can be downcast back to its boxed form without unwrapping). The `repr(transparent)` is what permits the vtable's `object_downcast` to expose the inner `M` directly.

### 4.6 Tagged-dispatch comment + `*Kind` traits — `src/kind.rs` lines 1–121 — **825 tok**
Full file. The 46-line top comment is the **only** place where the autoref-to-disambiguate-overlapping-impls trick is documented; without it the dispatch in `anyhow!($expr)` looks like magic. Then the three traits — `AdhocKind` (impl'd for `&T where T: Display + Debug`), `TraitKind` (impl'd for any `T: Into<Error>`, no autoref), `BoxedKind` (impl'd for `Box<dyn StdError + Send + Sync>`) — and their `Adhoc`/`Trait`/`Boxed` types with `new` methods. The dispatch is `(&error).anyhow_kind().new(error)` from `__anyhow!` in T1.6.

### 4.7 Display/Debug formatter for `ErrorImpl` — `src/fmt.rs` lines 1–67 — **502 tok**
The actual byte-for-byte formatting referenced by the contract in T2.3. `display`: `{}` writes only the head; `{:#}` walks `chain.skip(1)` writing `: ` separators. `debug`: writes head, then if `source().is_some()` writes `\n\nCaused by:` and enumerates causes (with `multiple = cause.source().is_some()` deciding numbered vs unnumbered indent), then if `BacktraceStatus::Captured` writes the backtrace with the `stack backtrace:` → `Stack backtrace:` capitalization fixup. The `Indented` writer at lines 69–101 is omitted from this batch (see T6.7).

**Tier 4 subtotal: ~4025 tokens**

**Ordering constraint**: 4.1 → 4.2 → 4.3 must appear in that order (they are increasingly downstream within `src/error.rs` and 4.3 references 4.2's layouts). 4.4–4.7 are siblings under different file parents; rank 4.4 (`ptr.rs`) first because the vtable functions in T4.3 use `Ref<ErrorImpl>` / `Own<ErrorImpl>`.

---

## Tier 5 — Compile-time configuration matrix

The cfg flags fan out across every file. An agent that doesn't understand the matrix will mis-edit cfg gates. This tier is small and high-leverage.

### 5.1 Backtrace cfg switch + macros — `src/backtrace.rs` lines 1–68 — **401 tok**
The four cfg branches that decide what `Backtrace` resolves to: `std::backtrace::Backtrace` (cfg `std_backtrace`), the `capture::Backtrace` defined in this same file (cfg `feature = "backtrace"` and not `std_backtrace`), an uninhabited `enum Backtrace {}` (neither), and the `impl_backtrace!` / `backtrace!` / `backtrace_if_absent!` macro_rules switches that propagate the cfg into method signatures and call sites. The three `backtrace_if_absent!` cfg branches alone (lines 38–68) explain why `Error::new` doesn't always capture: on nightly it asks the inner error first via `request_ref::<Backtrace>`.

### 5.2 Nightly probe + provider shims — `src/nightly.rs` lines 1–58 — **414 tok**
Full file. The `#[cfg(anyhow_build_probe)] const _: () = { … MyError(Backtrace) … };` is the actual surface area `build.rs` compiles to decide whether `error_generic_member_access` is available. Then the `request_ref_backtrace` / `provide_ref_backtrace` / `provide` shims used by `error.rs` and `context.rs`. Tiny file, high leverage.

### 5.3 `build.rs` rustc probe driver — `build.rs` lines 1–97 — **923 tok**
The `main` function only. The decision tree over `compile_probe(false)` / `compile_probe(true)` / `RUSTC_BOOTSTRAP` that decides whether to set `cargo:rustc-cfg=error_generic_member_access` and `cargo:rustc-cfg=std_backtrace`. The `rustc < 81` branch sets `anyhow_no_core_error` (which gates the in-crate `trait StdError` shim from T2.2). The `rustc < 85` branch sets `anyhow_no_clippy_format_args` (which gates the `#[cfg_attr(not(anyhow_no_clippy_format_args), clippy::format_args)]` on the macros in T1.5/1.6). The full `compile_probe` and `rustc_minor_version` helpers (lines 99–207) are below-the-fold — their existence is signalled by the tagged calls in `main`.

**Tier 5 subtotal: ~1738 tokens**

**Ordering constraint**: 5.1 → 5.2 → 5.3 — backtrace.rs is the consumer of the cfg flags; nightly.rs is the surface area; build.rs is the producer. Reading them in that order each one motivates the next.

---

## Tier 6 — Behavior-spec tests

The tests are short and double as executable specifications for the contracts in Tiers 2–4. They are the cheapest way to confirm "what should this output be?" without running the crate.

### 6.1 Format-output golden values — `tests/test_fmt.rs` lines 1–93 — **569 tok**
Full file. `fn f` / `g` / `h` build a 3-level chain (an `io::Error` with `PermissionDenied`, wrapped twice with `.context("f failed")` then `.context("g failed")`). Then `EXPECTED_ALTDISPLAY_*`, `EXPECTED_DEBUG_*`, and `EXPECTED_ALTDEBUG_*` are byte-for-byte expected strings for `{:#}`, `{:?}`, `{:#?}`. The `EXPECTED_DEBUG_H` block in particular shows the numbered indent (`    0: f failed\n    1: oh no!`) that matches the `multiple` branch of `fmt.rs::debug`.

### 6.2 Context chain semantics + drop ordering — `tests/test_context.rs` lines 1–172 — **1007 tok**
Full file. `make_chain()` builds a `LowLevel`-via-`thiserror` → `MidLevel` → `HighLevel` chain via `Err::<(), _>(…).context(…)`. Tests prove: `is::<HighLevel>` / `is::<MidLevel>` / `is::<LowLevel>` are all true on the same `Error` (the bidirectional-downcast contract from T2.5); `downcast::<MidLevel>` consumes the mid level and drops only the other two; `root_cause` returns the deepest `Display`. Uses the `drop/mod.rs` helper from 6.6.

### 6.3 Chain iterator forward/reverse contract — `tests/test_chain.rs` lines 1–69 — **581 tok**
Full file. Builds `anyhow!({ 0 }).context(1).context(2).context(3)` and asserts the iteration order is 3,2,1,0 (newest first); reverse order is 0,1,2,3; `len()` and `size_hint()` track per-step; `Chain::default()` is empty; `clone()` works.

### 6.4 Layout + autotrait assertions — `tests/test_repr.rs` lines 1–30 — **169 tok**
Full file. `assert_eq!(mem::size_of::<Error>(), mem::size_of::<usize>())` and the same for `Result<(), Error>` (proves the niche-optimization). Plus `Error: Unpin + Send + Sync + 'static` and a `DetectDrop` round-trip.

### 6.5 Autotrait assertions — `tests/test_autotrait.rs` lines 1–34 — **174 tok**
Full file. Five `assert_*<Error>()` calls covering `Send`, `Sync`, `UnwindSafe`, `RefUnwindSafe`, `Unpin`. Pair with T3.3's marker impls.

### 6.6 Test fixtures — `tests/common/mod.rs` and `tests/drop/mod.rs` — **355 tok** (81 + 274)
- `tests/common/mod.rs`: `bail_literal()` / `bail_fmt()` / `bail_error()` — minimal call sites for each `bail!` arm.
- `tests/drop/mod.rs`: `Flag` (atomic bool wrapper) and `DetectDrop` (a `StdError` whose `Drop` impl flips the flag and asserts it wasn't already set). Used by test_context, test_convert, test_repr, test_downcast.

### 6.7 `Indented` formatter unit tests — `src/fmt.rs` lines 69–158 — **~700 tok** (omit if budget tight; see below)
The `Indented` writer struct + its three `#[cfg(test)]` cases (`one_digit`, `two_digits`, `no_digits`) exercise the per-cause indentation logic. Often demoted because the goldens in T6.1 already cover the integration; included here only when `fmt.rs` itself is the editing target.

**Tier 6 subtotal (excluding 6.7): ~2855 tokens**

**Ordering constraint**: T6.6 (helpers) is the structural parent of T6.2/6.3/6.4 — they `mod drop;` it in. Show 6.6 before its consumers. T6.1 has no helper dependency.

---

## Below-the-fold — considered and explicitly demoted

Each of these was a candidate; the one-line reason captures why it didn't make the cut.

- **`src/ensure.rs` lines 103–883 — the `__parse_ensure!` TT-muncher (~19,266 tok)**. Too large for the budget by itself, and the value/token ratio is poor: it's one giant macro_rules that simulates a Rust expression parser to find the binary comparison operator `ensure!(lhs == rhs)` should split on. Touching it requires this whole file plus the `__fancy_ensure!` consumer; agents not editing it can rely on the `ensure!`-as-`if !$cond { return Err(...) }` mental model from T1.7.
- **`src/ensure.rs` lines 1–101 — `Buf`, `BothDebug`/`NotBothDebug`, `render` (665 tok)**. The "lhs vs rhs" rendering helper used when the ensure parser succeeds. Excluded because the `(2 vs 1)` output format is already evident from `tests/test_macros.rs::test_ensure` (which is itself below-the-fold), and the `Buf` no-alloc 40-byte stack writer is an internal optimization.
- **`src/ensure.rs` lines 884–934 — `__fancy_ensure!` and `__fallback_ensure!` (408 tok)**. The terminal arms invoked by the muncher; same reasoning as above.
- **`src/error.rs` lines 145–285 — `construct_from_std/adhoc/display/context/boxed` bodies (~1369 tok)**. The five private constructors that each populate an `ErrorVTable`. Pattern is uniform (each builds a vtable literal then calls `unsafe { Error::construct }`); the structure is implied by T4.1's vtable definition.
- **`src/error.rs` lines 287–314 — `unsafe fn construct` (271 tok)**. The Box-allocate-then-cast-to-thin-pointer step. Demoted because its single caller pattern is referenced from T4.1 and the comment in T4.2 already explains "equivalent to the safe unsize coercion … except that the result is a thin pointer".
- **`src/error.rs` lines 756–928 — the 13 unsafe vtable functions (`object_drop`, `object_ref`, `object_boxed`, `object_reallocate_boxed`, `object_downcast`, `no_backtrace`, `context_downcast`, `context_drop_rest`, `context_chain_downcast`, `context_chain_drop_rest`, `context_backtrace`, plus `vtable`) (1761 tok)**. The actual vtable entries pointed to by `ErrorVTable`. Demoted to below-the-fold because each is short and pattern-driven, and T4.1 + T4.2 + the construct sites in T3.1 + T2.5 transitively imply their behavior. Promote whenever the task is "modify type-erasure semantics".
- **`src/backtrace.rs` lines 70–410 — the `mod capture` fallback (2406 tok) and `_assert_send_sync` (24 tok)**. The full ~340-line reimplementation of `std::backtrace::Backtrace` on top of the `backtrace = "0.3.51"` crate, used only when neither nightly nor `std_backtrace` is available. Demoted because the `backtrace` Cargo feature is rarely needed on modern toolchains and the file is long; agents needing it can land here from T5.1 which names the cfg.
- **`build.rs` lines 99–207 — `compile_probe`, `rustc_minor_version`, `cargo_env_var` (~954 tok)**. The mechanical helpers: invoke `RUSTC` against `src/nightly.rs` with the right flags, parse `rustc --version`, fetch env vars or exit. T5.3's `main` call sites name them, which is enough orientation; an agent only needs the bodies if porting the probe to a new target.
- **`tests/test_downcast.rs` (791 tok)**, **`tests/test_macros.rs` (618 tok)**, **`tests/test_source.rs` (368 tok)**, **`tests/test_convert.rs` (314 tok)**, **`tests/test_boxed.rs` (290 tok)**. Each is a focused behavior test; collectively they're worth 2381 tokens for marginal additional spec coverage beyond Tier 6. The `test_ensure!(v + v == 1)` "(2 vs 1)" assertion in `test_macros.rs` is the most spec-relevant single line.
- **`tests/test_ensure.rs` (7285 tok)**. ~750 lines of corner cases for the `ensure!` parser. Pair with `src/ensure.rs` if (and only if) editing the muncher.
- **`tests/test_ffi.rs` (126 tok)**. Three `#[no_mangle] pub extern "C"` functions; useful when verifying that `Error` and `Option<Error>` are FFI-safe (the `extern "C"` plus `#![deny(improper_ctypes)]` is the actual proof). Demoted because most tasks don't touch FFI.
- **`tests/test_backtrace.rs` (77 tok)**. Two-arm test that's a no-op on stable; carries little information beyond "backtrace capture compiles on nightly".
- **`tests/compiletest.rs` (60 tok) + `tests/ui/*.rs` and `*.stderr` (~1300 tok total)**. The `trybuild` compile-fail catalog: `chained-comparison.rs`, `empty-ensure.rs`, `ensure-nonbool.rs`, `must-use.rs`, `no-impl.rs`, `temporary-value.rs`, `wrong-interpolation.rs`. Each is a one-shot diagnostic regression. The catalog (file names alone — 7 entries — would cost ~50 tokens if rendered as a list) is worth surfacing as a structural-only batch when token budget allows; the file bodies themselves are rarely needed, so they're below-the-fold.
- **`tests/crate/Cargo.toml` (~80 tok) + `tests/crate/test.rs` (9 tok) + `tests/crate/.gitignore`**. The 3-line `pub use anyhow::*;` dummy crate that CI uses to verify the MSRV (1.68) build under `--no-default-features` and `--features backtrace`. Existence is mentioned in the header overview; the contents are trivial.
- **`.github/workflows/ci.yml` (1194 tok)**. The CI matrix: `nightly`, `beta`, `stable`, `1.82.0`, `1.80.0`, `1.76.0` for `cargo test`; `1.68.0` for `cargo check --manifest-path tests/crate/Cargo.toml`; plus separate `minimal` (with `-Z minimal-versions`), `windows`, `doc` (cargo-docs-rs), `clippy` (`-Dclippy::all -Dclippy::pedantic`), `miri` (with `-Zmiri-strict-provenance`), `outdated`. Useful when the task is "what does CI enforce?" but unnecessary for code-editing tasks.
- **`.github/FUNDING.yml` (single line), `rust-toolchain.toml` (just `components = ["rust-src"]`), `.gitignore` (`/target/`, `/Cargo.lock`)**. Truly inert.
- **`LICENSE-APACHE` / `LICENSE-MIT`**. Standard dual license; no agent-relevant content.
