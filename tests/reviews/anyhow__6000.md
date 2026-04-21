---
snapshot_hash: 95f8879c3f9bca0e391c383026f60be9f37394cf3c721ab86ad7024362d1bc66
---

## Summary

The snapshot is very far from the North Star at this budget. Tier-1 listings
(1.1/1.2/1.3) and most tier-1 public-surface declarations (1.5-1.13) are
present as short header lines, but the output then diverges hard: the full
207-line crate-level rustdoc `lib.rs:1-207` (explicitly below-the-fold per
North Star) and the public struct declarations from `ptr.rs` (not ranked at
all) are rendered, while nearly every batch in tiers 2-7 is absent — README
Details (2.2/2.5/2.7), `Error` constructor/method signatures (2.4/2.6), the
invariant tests (2.8/2.9/2.10), the formatting doctrine (3.1-3.3), `Context`
internals (3.4/3.5), full `chain.rs` (3.6), `test_chain.rs` (3.7), most of
tiers 4-5 (constructor wirings, downcast bodies, `Error::context` method,
`__private`), the cfg/build story (6.1/6.3), CI (7.3), and the cross-test
indices (7.7/7.8). The dominant pattern is **shallow, breadth-first walker
output**: short declaration lines plucked from every file plus two large
docstring dumps, rather than a ranked depth-first selection. Multiple included
batches are rendered only partially (declaration shown, impls/bodies elided),
producing batch-correctness violations across most of tiers 3-6.

## Divergences

### Ranking

- [markdown] [minor] 1.4 (README headline + install snippet, `README.md:9-17`) missing; snapshot shows only `README.md:1-7` (title + badges). The "trait object based error type" one-liner and the `anyhow = "1.0"` snippet are absent. 1.5-1.13 (all ranked below 1.4) are present.
- [rust] [minor] 1.14 (`ensure!` `#[cfg(doc)]` arms, `macros.rs:127-153`) missing; 1.12 (`bail!`) and 1.13 (`anyhow!`) are both present.
- [markdown] [major] 2.2 (README Details bullets 1-2, `README.md:21-67`) missing while below-the-fold `lib.rs:1-207` (the crate rustdoc mirroring the README with doctest scaffolding) is rendered. NS explicitly lists `lib.rs:14-208` as redundant-with-README below-the-fold content.
- [rust] [major] 2.3 (`Error` short doc preamble, `lib.rs:288-298`) missing while below-the-fold `lib.rs:1-207` and unranked `ptr.rs` struct decls are present.
- [rust] [major] 2.4 (`Error::new`/`msg`/`from_boxed` signatures in `error.rs`) missing; the snapshot instead surfaces the much lower-ranked `ErrorImpl`/`ContextError` structs from `error.rs:934-955`.
- [markdown] [major] 2.5 (README Details bullets 3-6) missing while below-the-fold content is rendered.
- [rust] [major] 2.6 (`Error` consumer-method signatures: `context`/`backtrace`/`chain`/`root_cause`/`is`/`downcast*`/`into_boxed_dyn_error`/`reallocate_...`) missing; displaced by below-the-fold rustdoc and unranked ptr.rs structs.
- [markdown] [major] 2.7 (README no_std + comparisons + license footer) missing while below-the-fold content is rendered.
- [rust] [major] 2.8 (`tests/test_repr.rs` full file — the one-word-size invariant) missing; file listed only as bare filename. Displaced by below-the-fold content.
- [rust] [major] 2.9 (`tests/test_autotrait.rs` full file) missing; bare filename only.
- [rust] [major] 3.1 (`Error` Display-representations rustdoc, `lib.rs:299-388`) missing — the canonical formatting reference — while below-the-fold `lib.rs:1-207` is included instead.
- [rust] [major] 3.2 (`test_fmt.rs:1-67` `EXPECTED_*` constants) missing; `test_fmt.rs` is a bare filename.
- [rust] [major] 3.3 (`fmt.rs:1-67` `Display`/`Debug` impls) missing; `fmt.rs` is a bare filename.
- [rust] [major] 3.4 (`Context` trait rustdoc, `lib.rs:469-523`) missing.
- [rust] [major] 3.5 (`context.rs:1-113` `Context` impls for `Result`/`Option`) missing; `context.rs` is a bare filename.
- [rust] [major] 3.7 (`tests/test_chain.rs` full file) missing; bare filename only.
- [rust] [major] 4.1 (`Error::context` method body + rationale doc, `error.rs:316-402`) missing.
- [rust] [major] 4.2 (`Error::into_boxed_dyn_error` + `reallocate_..._without_backtrace`) missing.
- [rust] [major] 4.3 (`From<E>`/`Deref`/`DerefMut`/`Display`/`Debug`/`Drop` impls on `Error`) missing.
- [rust] [major] 4.4 (`kind.rs:1-46` autoref-dispatch design comment) missing — snapshot's `kind.rs` content starts at line 55.
- [rust] [major] 4.6 (`fmt.rs:69-158` `Indented` formatter + tests) missing.
- [rust] [major] 5.2 (`Error::construct_from_std/_adhoc/_display`) missing.
- [rust] [major] 5.3 (`Error::construct_from_context/_boxed` + unsafe `construct<E>`) missing.
- [rust] [major] 5.5 (`Error::is`/`downcast`/`downcast_ref`/`downcast_mut` bodies) missing.
- [rust] [major] 5.6 (`ErrorImpl` private accessors, `error.rs:957-1013`) missing.
- [rust] [major] 6.1 (`build.rs` `fn main` cfg decision tree) missing; `build.rs` appears as bare filename.
- [rust] [major] 6.6 (`lib.rs:654-728` `__private` module) missing; only the `pub mod __private {` header with immediate elision is shown.
- [rust] [major] 6.7 (`error.rs:1047-1086` `From<Error> for Box<dyn StdError>` + AsRef + UnwindSafe) missing.
- [rust] [major] 7.1 (`bail!`/`anyhow!` rustdoc with examples) missing.
- [rust] [major] 7.2 (`__ensure!` doc wrapper + `#[cfg(not(doc))]` dispatch arm) missing.
- [other-language] [major] 7.3 (CI matrix from `.github/workflows/ci.yml`) missing; only the directory path `.github/workflows/` is shown.
- [rust] [major] 7.6 (`tests/test_source.rs` full file) missing; bare filename only.
- [rust] [major] 7.7 (`test_ensure.rs` preamble + per-test fn index) missing; bare filename only.
- [rust] [major] 7.8 (cross-test `#[test] fn` index across 5 files) missing.

### Batch correctness

- [rust] [major] 2.1 (`lib.rs:246-263`) partially rendered — lines 252-263 (the 11 `mod ...;` declarations) are shown, but lines 246-251 (`extern crate alloc;` and `#[cfg(feature = "std")] extern crate std;`) are silently omitted. The `extern crate` statements are the load-bearing alloc/std toggle per the NS note.
- [rust] [major] 2.10 (`test_ffi.rs:1-19`) partially rendered — three `pub extern "C" fn ...` signature lines with bodies elided; the test-assertion bodies that actually exercise FFI-safety are missing.
- [rust] [major] 3.6 (`chain.rs:1-102`) partially rendered — `Chain` struct decl (lines 11-13) and `ChainState` enum (lines 16-24) shown, but the `Iterator`/`DoubleEndedIterator`/`ExactSizeIterator`/`Default` impls and the `pub(crate) use` re-export are missing. NS explicitly notes "Splitting hurts — the impls only make sense together with the state machine."
- [rust] [major] 3.8 (`common/mod.rs` + `drop/mod.rs`) partially rendered — only fn/struct signatures with bodies elided; the `bail_*` helpers and `Flag`/`DetectDrop` drop-detection bodies are unreadable from signatures alone.
- [rust] [major] 4.5 (`kind.rs:55-121`) partially rendered — the `Adhoc`/`Trait`/`Boxed` unit structs and the `AdhocKind`/`TraitKind`/`BoxedKind` traits with their `anyhow_kind` methods are shown, but the NS batch also includes the `Adhoc::new`/`Trait::new`/`Boxed::new` free-function impls (the three concrete constructors that produce an `Error`). These are silently omitted.
- [rust] [major] 5.1 (`error.rs:740-755` + `:930-955`) partially rendered — only the `ErrorImpl<E>` struct and `ContextError<C, E>` struct decls from the 930-955 range are shown; the `ErrorVTable` struct (740-755), which is the *core trick* per the NS, is missing entirely, as is the `vtable()` pointer-trick reader at 941-951.
- [rust] [major] 5.4 (`wrapper.rs:1-84`) partially rendered — only the three `pub struct` declarations (`MessageError`, `DisplayError`, `BoxedError`) are shown; the `#[repr(transparent)]` attrs, `StdError` impls, and `Display`/`Debug` forwarders that make the newtype pattern work are omitted.
- [rust] [major] 6.2 (`nightly.rs:1-58`) partially rendered — three `pub fn` signature lines (`request_ref_backtrace`, `provide_ref_backtrace`, `provide`) with bodies elided; the `#[cfg(anyhow_build_probe)] const _: () = { ... }` probe block (the build-script coupling per NS) is absent.
- [rust] [major] 6.3 (`backtrace.rs:1-68`) partially rendered — only a single `pub(crate) enum Backtrace {}` line at line 8, which is itself just one of the three cfg arms; the `impl_backtrace!`/`backtrace!`/`backtrace_if_absent!` macros and the full cfg dispatch tree are absent.
- [rust] [major] 6.4 (`ensure.rs:1-101`) partially rendered — the `BothDebug` and `NotBothDebug` trait declarations are shown, but the `Buf` 40-byte stack buffer and the `render` function (which do the no-alloc `(2 vs 1)` formatting per NS) are missing.
- [rust] [major] 6.5 (`ensure.rs:884-935`) partially rendered — only `macro_rules! __fancy_ensure` and `macro_rules! __fallback_ensure` header lines with immediate `…` elisions; the actual macro arms that define what `ensure!` expands to are absent.
- [other-language] [major] 7.4 (`tests/crate/`) partially rendered — `Cargo.toml` lines 1-17 shown but `test.rs` content absent (only the filename).
- [rust] [predecessor] 4.5 partial content present without 4.4 (`kind.rs:1-46` autoref-dispatch design comment). NS declares `4.5 Predecessor: 4.4`; the reader sees `Adhoc`/`Trait`/`Boxed` dispatch types with no explanation of why autoref method resolution is the mechanism.
- [rust] [predecessor] 6.2 (nightly.rs fn sigs) present without 6.1 (`build.rs` cfg decision tree). NS declares `6.2 Predecessor: 6.1`; the reader sees `request_ref_backtrace`/`provide_ref_backtrace`/`provide` without any story for the `anyhow_build_probe`/`error_generic_member_access` cfgs that gate them.

### Honesty

- (none)
