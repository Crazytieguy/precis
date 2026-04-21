---
snapshot_hash: 33fcef0fc41463a9d411d91782ab20df74acecb961a4b1b9974aaaca7b4ca105
---

## Summary

At budget 3000 the snapshot has the cheap tier-1 backbone mostly right (1.1, 1.2, 1.3, 1.5, 1.6, 1.7, 1.8, 1.10, 1.12, 1.13 present) but leaks a large volume of lower-ranked structural content from tiers 2-6 (ptr.rs struct decls, kind.rs dispatch types, wrapper.rs newtype heads, ErrorImpl+ContextError, ChainState, ensure.rs trait heads, nightly.rs fn signatures, test_ffi signatures, common/drop helper heads) while skipping 1.4 (README headline) and 1.14 (ensure! doc arms) and omitting the README `Details` tutorial, `Error` docs, and all of the tier-2/3 prose batches. The dominant pattern is a structural top-of-file bias: the walker emits one-line declarations from every module rather than selecting ranked batches whole, so most batches that do appear are silently partial and several highly ranked prose/README batches (1.4, 2.2, 2.3, 2.5, 2.7, 3.1) are displaced by tier-4-6 code fragments.

## Divergences

### Ranking
- [markdown] [major] 1.4 (README.md:9-17, headline + install snippet) missing; snapshot shows README.md:1-7 (title + badge lines — explicitly below-the-fold cosmetic per the North Star) and also lower-ranked major-3-6 content (kind.rs 4.5, nightly.rs 6.2, wrapper.rs 5.4, ensure.rs 6.4 heads). Tier-1 batch skipped while major-4-6 content is present.
- [rust] [major] 1.14 (macros.rs:127-153 `ensure!` `#[cfg(doc)]` arms) missing; lower-ranked 6.4 (BothDebug/NotBothDebug trait heads) and 6.5 (`__fancy_ensure!`/`__fallback_ensure!` macro headers) and below-the-fold `__parse_ensure!` header are present.
- [rust] [major] 1.9's `pub use anyhow as format_err` (lib.rs:285-286) missing; major-3-6 content (chain.rs ChainState, kind.rs 4.5, ErrorImpl/ContextError 5.1, wrapper.rs 5.4, nightly.rs 6.2) present.
- [rust] [major] 1.11's `backtrace` optional dep and the five dev-dependencies (Cargo.toml:17-30) missing; same major-3-6 content present.
- [markdown] [major] 2.2 (README.md:21-67 `Details` bullets — `?` propagation and `.context(...)`) missing; major-4-6 content present.
- [markdown] [major] 2.5 (README.md:68-122 `Details` bullets 3-6) missing.
- [markdown] [major] 2.7 (README.md:124-180 no-std + comparisons + license footer) missing.
- [rust] [major] 2.1 (lib.rs:246-263 `extern crate` + 11 `mod` decls) missing; major-3-6 content present.
- [rust] [major] 2.3 (lib.rs:288-298 `Error` short doc preamble) missing.
- [rust] [major] 2.4 (error.rs constructor signatures `new`/`msg`/`from_boxed`) missing; tier-5 `ErrorImpl`/`ContextError` from 5.1 is present, so the agent sees the private vtable struct before the public constructors.
- [rust] [major] 2.6 (error.rs consumer-method signatures: `context`/`backtrace`/`chain`/`downcast*`/`into_boxed_dyn_error`/...) missing; tier-5 `ErrorImpl` struct present instead.
- [rust] [major] 2.8 (full `test_repr.rs`) missing; tier-5/6 content present.
- [rust] [major] 2.9 (full `test_autotrait.rs`) missing; tier-5/6 content present.
- [markdown] [major] 3.1 (lib.rs:299-388 `Error` Display-representations rustdoc) missing; major-5/6 content present.
- [rust] [major] 3.2 (test_fmt.rs:1-67 `EXPECTED_*` constants) missing; kind.rs 4.5 and nightly.rs 6.2 fragments present.
- [rust] [major] 3.4 (lib.rs:469-523 `Context` rustdoc + `ImportantThing` example) missing; kind.rs 4.5 present.
- [rust] [major] 3.5 (context.rs:1-113 `Context` impls for `Result`/`Option`) missing; `context.rs` is named with zero content shown, while `kind.rs` (4.5) is rendered with trait bodies.
- [rust] [major] 3.7 (full `test_chain.rs`) missing; major-4-6 fragments present.
- [rust] [major] 4.1 (error.rs:316-402 `Error::context` body + rationale) missing; 4.5 content present.
- [rust] [major] 4.2 (error.rs:582-672 `into_boxed_dyn_error` + `reallocate_...`) missing.
- [rust] [major] 4.3 (error.rs:691-738 `From<E>`/`Deref`/`Display`/`Debug`/`Drop` impls) missing.
- [rust] [minor] 4.4 (kind.rs:1-46 autoref-dispatch design comment) missing while 4.5 (kind.rs:55-121) is largely present. Inside tier-4 major group. Also a predecessor violation (see below).
- [rust] [major] 4.6 (fmt.rs:69-158 `Indented` helper + tests) missing.
- [rust] [major] 5.2 (error.rs:144-225 `construct_from_std`/`_adhoc`/`_display`) missing.
- [rust] [major] 5.3 (error.rs:226-314 `construct_from_context`/`_boxed` + unsafe `construct<E>`) missing.
- [rust] [major] 5.5 (error.rs:482-580 `is`/`downcast`/`downcast_ref`/`downcast_mut` bodies) missing.
- [rust] [major] 5.6 (error.rs:957-1013 `ErrorImpl` private accessors) missing.
- [rust] [major] 6.1 (build.rs:1-97 cfg decision tree) missing; only the file name is shown.
- [rust] [major] 6.3 (backtrace.rs:1-68 cfg dispatch + `impl_backtrace!`/`backtrace!`/`backtrace_if_absent!`) missing — only line 8 `pub(crate) enum Backtrace {}` is shown.
- [rust] [major] 6.6 (lib.rs:654-728 `__private` module) missing.
- [rust] [major] 6.7 (error.rs:1047-1086 `From<Error> for Box<dyn StdError>` + `AsRef` + `UnwindSafe`) missing.
- [rust] [major] 7.1 (macros.rs:1-55 and :174-201 `bail!`/`anyhow!` rustdoc) missing.
- [rust] [major] 7.2 (macros.rs:70-124 + :155-172 `__ensure!` doc wrapper + dispatch arm) missing.
- [rust] [major] 7.3 (ci.yml matrix/minimal-versions/clippy+miri) missing.
- [rust] [major] 7.4 (tests/crate/ no-std smoke crate) — Cargo.toml is partially present; test.rs body missing.
- [rust] [major] 7.5 (tests/ui/ listing) missing as a dedicated rendered listing (ui children are shown individually by tree expansion).
- [rust] [major] 7.6 (full `test_source.rs`) missing.
- [rust] [major] 7.7 (test_ensure.rs:1-39 preamble + fn index) missing.
- [rust] [major] 7.8 (cross-test `#[test]`/`fn test_` index for the five bodies) missing.
- [other-language] [minor] `src/ptr.rs` struct declarations (Own, Ref, Mut, CastTo) are rendered despite ptr.rs not appearing in any ranked batch of the North Star — unranked content displacing tier-1/2 batch budget.

### Batch correctness
- [rust] [major] 1.9 partial: `pub fn Ok<T>` (lib.rs:650) is present but `pub use anyhow as format_err` (lib.rs:285-286) — the other half — is silently omitted.
- [other-language] [major] 1.11 partial: only `[features]` (Cargo.toml:14-16) is kept; the `[dependencies]` block (backtrace optional dep with its no-std comment) and the entire `[dev-dependencies]` block (lines 17-30) are silently dropped. North Star flags this as catastrophic-omission risk.
- [rust] [major] 1.12 partial: only `macro_rules! bail {` header (line 58) is shown with an ellipsis marker; the three body arms (lines 59-67) are elided. Batch is the full 56-68 definition.
- [rust] [major] 1.13 partial: only `macro_rules! anyhow {` header (line 204) is shown with an ellipsis; actually the three arms at lines 205-223 appear to be rendered — verified present.
- [rust] [major] 1.5 partial: Cargo.toml lines 1-12 are shown; line 13 (blank) omitted is fine, but `description` / `documentation` / `keywords` fields are all present — OK.
- [rust] [major] 2.10 partial: `tests/test_ffi.rs` signatures at lines 7, 12, 17 are shown; bodies at 8-10, 13-15, 18-19 are elided with `…`. Batch is the full 1-19 file (assertions inside the extern fns are the point).
- [rust] [major] 3.8 partial: both `common/mod.rs` (bail_literal/bail_fmt/bail_error with bodies elided) and `drop/mod.rs` (Flag::new, Flag::get, DetectDrop::new, Display::fmt, Drop::drop signatures with bodies elided) are rendered as signatures only. The whole point of 3.8 is the Flag/DetectDrop drop-detection primitives whose bodies are what unlock downstream test files.
- [rust] [major] 4.5 partial: kind.rs trait declarations with their `anyhow_kind` fn bodies are present at lines 55-109, but the `impl Adhoc { fn new }` / `impl Trait { fn new }` / `impl Boxed { fn new }` blocks (extending through 121) that the batch requires are silently omitted.
- [rust] [major] 5.1 partial: `ErrorImpl<E>` (error.rs:934-940) and `ContextError<C, E>` (:952-955) are shown, but the `ErrorVTable` struct (error.rs:740-755) — the first half of the batch and the "core trick" of anyhow — is entirely absent. Reader gets the client of the vtable without the vtable itself.
- [rust] [major] 5.4 partial: wrapper.rs shows only the three struct declaration lines (`MessageError`, `DisplayError`, `BoxedError` at lines 11, 34, 58); the `#[repr(transparent)]` attributes, `StdError` / `Display` / `Debug` impls — the transparent-layout safety argument — are silently dropped. Batch is the full 1-84 file.
- [rust] [major] 6.2 partial: nightly.rs is rendered as three fn signatures (`request_ref_backtrace`:41, `provide_ref_backtrace`:52, `provide`:56) with bodies elided; the `#[cfg(anyhow_build_probe)] const _: () = { ... }` probe block is absent. Batch is the full 1-58 file.
- [rust] [major] 6.3 partial: only `pub(crate) enum Backtrace {}` at line 8 is shown; the rest of the three-way cfg dispatch and the `impl_backtrace!` / `backtrace!` / `backtrace_if_absent!` macros (batch is 1-68) are silently omitted.
- [rust] [major] 6.4 partial: only `BothDebug` (lines 10-12) and `NotBothDebug` (lines 25-27) trait declarations are shown; the `Buf` no-alloc 40-byte stack buffer and `render` function — the load-bearing content per the North Star notes — are missing. Batch is the full 1-101 range.
- [rust] [major] 6.5 partial: only `macro_rules! __fancy_ensure {` header at 886 and `macro_rules! __fallback_ensure {` header at 912 are shown, both with bodies elided. Batch is the full 884-935 terminal arms.
- [rust] [major] 3.6 partial: chain.rs shows only the `pub(crate) struct Chain<'a>` (line 11-13) and `pub(crate) enum ChainState` (lines 16-24); the `Iterator`/`DoubleEndedIterator`/`ExactSizeIterator`/`Default` impls are silently omitted. Batch is the full 1-102 file.
- [rust] [major] 7.4 partial: `tests/crate/Cargo.toml` is rendered but `tests/crate/test.rs` shows only the file name with no contents (batch includes both files).

### Honesty
- (none)
