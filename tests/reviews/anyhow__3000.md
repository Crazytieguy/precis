---
snapshot_hash: 20941d81d4ec0f9702e5a089f46155d675a6b5e826de195ebb4369274790f26e
---

## Summary

At budget 3000 the snapshot has the cheap tier-1 listings and a handful of tier-1 declarations (1.2, 1.3 via a tree render; 1.6, 1.7, 1.8, 1.10, 1.12, 1.13 partially) but leaks a large volume of lower-ranked structural content from tiers 2-6 (ptr.rs struct decls, kind.rs dispatch types, wrapper.rs newtype heads, ErrorImpl+ContextError, ChainState, ensure.rs trait heads, nightly.rs fn signatures, test_ffi signatures, common/drop helper heads) while skipping 1.4 (README headline) and 1.14 (ensure! doc arms) and omitting the README Details tutorial, `Error` docs, `Context` rustdoc, and all of the tier-2/3 prose batches. The dominant pattern is a structural top-of-file bias: the walker emits one-line declarations from every module rather than selecting ranked batches whole, so most batches that do appear are silently partial and several highly ranked prose/README batches (1.4, 2.2, 2.3, 2.5, 2.7, 3.1) are displaced by tier-4-6 code fragments.

## Divergences

### Ranking
- [markdown] [major] 1.4 (README.md:9-17 headline + install snippet) missing while README.md:1-7 badge lines (below-the-fold cosmetic per North Star) and tier-4-6 code fragments (kind.rs 4.5, nightly.rs 6.2, wrapper.rs 5.4, ensure.rs 6.4 heads, ErrorImpl/ContextError 5.1) are present.
- [rust] [major] 1.14 (macros.rs:127-153 `ensure!` `#[cfg(doc)]` arms) missing; lower-ranked 6.4 (BothDebug/NotBothDebug trait heads), 6.5 (`__fancy_ensure!`/`__fallback_ensure!` macro headers) and below-the-fold `__parse_ensure!` header are present.
- [rust] [major] 1.9's `pub use anyhow as format_err` (lib.rs:285-286) missing; major-3-6 content (chain.rs ChainState, kind.rs 4.5, ErrorImpl/ContextError 5.1, wrapper.rs 5.4, nightly.rs 6.2) present.
- [rust] [major] 1.11's `backtrace` optional dep and the five dev-dependencies (Cargo.toml:17-30) missing; same major-3-6 content present.
- [markdown] [major] 2.2 (README.md:21-67 `Details` bullets — `?` propagation and `.context(...)`) missing; major-4-6 content present.
- [markdown] [major] 2.5 (README.md:68-122 `Details` bullets 3-6) missing.
- [markdown] [major] 2.7 (README.md:124-180 no-std + comparisons + license footer) missing.
- [rust] [major] 2.1 (lib.rs:246-263 `extern crate` + 11 `mod` decls) missing; major-3-6 content present.
- [rust] [major] 2.3 (lib.rs:288-298 `Error` short doc preamble) missing.
- [rust] [major] 2.4 (error.rs constructor signatures `new`/`msg`/`from_boxed`) missing; tier-5 `ErrorImpl`/`ContextError` from 5.1 is present, so the agent sees the private vtable client struct before the public constructors.
- [rust] [major] 2.6 (error.rs consumer-method signatures: `context`/`backtrace`/`chain`/`downcast*`/`into_boxed_dyn_error`/...) missing; tier-5 `ErrorImpl` struct present instead.
- [rust] [major] 2.8 (full `test_repr.rs`) missing; tier-5/6 content present.
- [rust] [major] 2.9 (full `test_autotrait.rs`) missing; tier-5/6 content present.
- [markdown] [major] 3.1 (lib.rs:299-388 `Error` Display-representations rustdoc) missing; major-5/6 content present.
- [rust] [major] 3.2 (test_fmt.rs:1-67 `EXPECTED_*` constants) missing; kind.rs 4.5 and nightly.rs 6.2 fragments present.
- [rust] [major] 3.3 (fmt.rs:1-67 `Display`/`Debug` impls for `ErrorImpl`) missing.
- [rust] [major] 3.4 (lib.rs:469-523 `Context` rustdoc + `ImportantThing` example) missing; kind.rs 4.5 present.
- [rust] [major] 3.5 (context.rs:1-113 `Context` impls for `Result`/`Option`) missing; `context.rs` is named with zero content shown, while `kind.rs` (4.5) is rendered with trait bodies.
- [rust] [major] 3.7 (full `test_chain.rs`) missing; major-4-6 fragments present.
- [rust] [major] 4.1 (error.rs:316-402 `Error::context` body + rationale) missing; 4.5 content present.
- [rust] [major] 4.2 (error.rs:582-672 `into_boxed_dyn_error` + `reallocate_...`) missing; 5.1/5.4/6.2 content present.
- [rust] [major] 4.3 (error.rs:691-738 `From<E>`/`Deref`/`Display`/`Debug`/`Drop` impls) missing; 5.1 content present.
- [rust] [minor] 4.4 (kind.rs:1-46 autoref-dispatch design comment) missing while 4.5 (kind.rs:55-121) is largely present. Inside tier-4 major group.
- [rust] [major] 4.6 (fmt.rs:69-158 `Indented` helper + tests) missing.
- [rust] [major] 5.2 (error.rs:144-225 `construct_from_std`/`_adhoc`/`_display`) missing; 6.2/6.4 content present.
- [rust] [major] 5.3 (error.rs:226-314 `construct_from_context`/`_boxed` + unsafe `construct<E>`) missing.
- [rust] [major] 5.5 (error.rs:482-580 `is`/`downcast`/`downcast_ref`/`downcast_mut` bodies) missing; 6.2/6.4 content present.
- [rust] [major] 5.6 (error.rs:957-1013 `ErrorImpl` private accessors) missing; 6.2/6.4 content present.
- [rust] [major] 6.1 (build.rs:1-97 cfg decision tree) missing; only the file name is shown, while kind.rs 4.5 / wrapper.rs 5.4 / nightly.rs 6.2 / ensure.rs 6.4-6.5 content is present.
- [rust] [minor] 6.3 (backtrace.rs:1-68 cfg dispatch + `impl_backtrace!`/`backtrace!`/`backtrace_if_absent!`) present only as line 8 `pub(crate) enum Backtrace {}`; rest of batch missing while 6.4/6.5 heads are present.
- [rust] [major] 6.6 (lib.rs:654-728 `__private` module) missing.
- [rust] [major] 6.7 (error.rs:1047-1086 `From<Error> for Box<dyn StdError>` + `AsRef` + `UnwindSafe`) missing.
- [rust] [major] 7.1 (macros.rs:1-55 and :174-201 `bail!`/`anyhow!` rustdoc) missing.
- [rust] [major] 7.2 (macros.rs:70-124 + :155-172 `__ensure!` doc wrapper + dispatch arm) missing.
- [rust] [major] 7.3 (ci.yml matrix/minimal-versions/clippy+miri) missing.
- [rust] [major] 7.4 (tests/crate/ no-std smoke crate) — Cargo.toml is partially present; test.rs body missing.
- [rust] [major] 7.5 (tests/ui/ listing as a dedicated rendered entry) missing; ui children are shown individually via tree expansion without contents.
- [rust] [major] 7.6 (full `test_source.rs`) missing.
- [rust] [major] 7.7 (test_ensure.rs:1-39 preamble + fn index) missing.
- [rust] [major] 7.8 (cross-test `#[test]`/`fn test_` index for the five bodies) missing.
- [rust] [minor] `src/ptr.rs` struct declarations (`Own`, `Ref`, `Mut`, `CastTo`) are rendered despite ptr.rs being explicitly unranked in the North Star — below-the-fold content displacing tier-1/2 budget.

### Batch correctness
- [rust] [major] 1.9 partial: `pub fn Ok<T>` (lib.rs:650) is present (truncated to its `fn` line) but `pub use anyhow as format_err` (lib.rs:285-286) — the other half of the batch — is silently omitted.
- [rust] [major] 1.11 partial: only `[features]` (Cargo.toml:14-16) is kept; the `[dependencies]` block (backtrace optional dep with its no-std comment) and the entire `[dev-dependencies]` block (lines 17-30) are silently dropped. North Star flags this as catastrophic-omission risk.
- [rust] [minor] 1.6 partial: the `#[repr(transparent)]` attribute (lib.rs:389) and the trailing `}` (line 393) are silently elided; only lines 390-392 of the 389-393 batch are shown.
- [rust] [minor] 1.7 partial: only `pub type Result<T, E = Error>` (line 468) is shown; the prior doc-stub line 467 is omitted. Minor, but the batch explicitly pairs the two.
- [rust] [minor] 1.10 partial: only the `pub struct Chain<'a> { state: ... }` (lib.rs:415-417) is shown; the `#[cfg(any(feature = "std", not(anyhow_no_core_error)))]` and `#[derive(Clone)]` at lines 413-414 — the cfg gate the batch notes as first appearance of the recurring predicate — are silently dropped.
- [rust] [minor] 1.12 partial: the `bail!` definition is shown as lines 58-68 only; the preceding `#[macro_export]` (line 56) and blank line 57 of the 56-68 batch are dropped.
- [rust] [minor] 1.13 partial: the `anyhow!` definition is shown as lines 204-223; the `#[macro_export]` and docstring shim line at 202-203 of the 202-223 batch are silently dropped.
- [rust] [major] 2.10 partial: `tests/test_ffi.rs` signatures at lines 7, 12, 17 are shown; bodies at 8-10, 13-15, 18-19 are elided with `…`. Batch is the full 1-19 file (bodies are the point).
- [rust] [major] 3.6 partial: chain.rs shows only the `pub(crate) struct Chain<'a>` (lines 11-13) and `pub(crate) enum ChainState` (lines 16-24); the `Iterator`/`DoubleEndedIterator`/`ExactSizeIterator`/`Default` impls and the `pub(crate) use crate::Chain` re-export are silently omitted. Batch is the full 1-102 file, explicitly marked "splitting hurts".
- [rust] [major] 3.8 partial: both `common/mod.rs` (bail_literal/bail_fmt/bail_error with bodies elided) and `drop/mod.rs` (Flag/DetectDrop struct headers with impl bodies elided) are rendered as signatures only. The whole point of 3.8 is the Flag/DetectDrop drop-detection primitives whose bodies are what unlock downstream test files.
- [rust] [major] 4.5 partial: kind.rs is shown as three `pub struct` lines (`Adhoc`:55, `Trait`:77, `Boxed`:100) each followed by `…`, then the three trait declarations with their default `anyhow_kind` fn bodies (lines 58-63, 80-85, 104-109). The `impl Adhoc { fn new }` / `impl Trait { fn new }` / `impl Boxed { fn new }` blocks (extending through 121) that the batch requires are silently omitted, and the cfg gate on `BoxedKind` the batch flags is dropped.
- [rust] [major] 5.1 partial: `ErrorImpl<E>` (error.rs:934-940) and `ContextError<C, E>` (:952-955) are shown, but the `ErrorVTable` struct (error.rs:740-755) — the first half of the batch and the "core trick" of anyhow — is entirely absent. Reader gets the client of the vtable without the vtable itself.
- [rust] [major] 5.4 partial: wrapper.rs shows only the three struct declaration lines (`MessageError`, `DisplayError`, `BoxedError` at lines 11, 34, 58) with `…` between; the `#[repr(transparent)]` attributes, `StdError`/`Display`/`Debug` impls, and `From` impls that make the transparent-layout safety argument — the batch's payload — are silently dropped. Batch is the full 1-84 file.
- [rust] [major] 6.2 partial: nightly.rs is rendered as three fn signatures (`request_ref_backtrace`:41, `provide_ref_backtrace`:52, `provide`:56) with bodies elided; the `#[cfg(anyhow_build_probe)] const _: () = { ... }` probe block is absent. Batch is the full 1-58 file.
- [rust] [major] 6.4 partial: only `BothDebug` (lines 10-12) and `NotBothDebug` (lines 25-27) trait declarations are shown; the `Buf` no-alloc 40-byte stack buffer and `render` function — the load-bearing content per the North Star notes — are missing. Batch is the full 1-101 range.
- [rust] [major] 6.5 partial: only `macro_rules! __fancy_ensure {` header at 886 and `macro_rules! __fallback_ensure {` header at 912 are shown, both with bodies elided. Batch is the full 884-935 terminal arms.
- [rust] [major] 7.4 partial: `tests/crate/Cargo.toml` is rendered (lines 1-6 and 14-17; 7-13 dropped) but `tests/crate/test.rs` shows only the file name with no contents (batch includes both files).
- [rust] [predecessor] 4.5 (kind.rs traits/impls partially shown) without 4.4 (kind.rs:1-46 autoref-dispatch design comment) — the "why" predecessor is missing while the dependent batch is present.
- [rust] [predecessor] 5.4 (wrapper.rs newtype headers) without its predecessor 5.1 being complete — ErrorVTable is absent, so the transparent-layout safety argument cannot be read off what's present.
- [rust] [predecessor] 6.2 (nightly.rs sigs) without its predecessor 6.1 (build.rs cfg decision tree) — the cfgs gating nightly.rs compilation are unexplained.
- [rust] [predecessor] 6.5 (`__fancy_ensure!`/`__fallback_ensure!` headers) without its predecessor 6.4 being complete — the autoref-dispatch traits `BothDebug`/`NotBothDebug` are shown but the `Buf` buffer and `render` function the fancy arm relies on are missing.

### Honesty
- (none)
