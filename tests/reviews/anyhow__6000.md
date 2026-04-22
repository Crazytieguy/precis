---
snapshot_hash: 818f2447697a01d16b9bd9607f623fdb871538ab577a1b17c0e79f0fe76f5a45
---

## Summary

At 6k tokens the snapshot keeps the top-level listings and most of the batch-1 lib.rs declarations (1.5-1.13, 2.1, 2.3, 3.1) but then collapses into mechanical signature/declaration extraction across many below-the-fold files (ptr.rs, partial kind.rs, partial chain.rs, partial ensure.rs, partial nightly.rs, partial __private) while skipping load-bearing mid-ranked batches entirely (README headline 1.4, README details 2.2/2.5/2.7, ensure! doc arms 1.14, Error constructors 2.4, Error consumer signatures 2.6, test_repr 2.8, test_autotrait 2.9, fmt.rs 3.3, Context rustdoc/impls 3.4/3.5, build.rs 6.1). The dominant pattern is a walker that indiscriminately surfaces type/fn declarations regardless of rank, producing many partial-batch inclusions, several cross-major inversions, and predecessor violations.

## Divergences

### Ranking
- [markdown] [major] 1.4 (README headline + install snippet, README.md:9-17) skipped; snapshot renders README.md:1-7 (title + badges, below-the-fold per NS) and includes ptr.rs struct declarations (below-the-fold) plus 7.5 (ui/ listing) far lower in the ranking.
- [rust] [major] 1.14 (`ensure!` user-facing `#[cfg(doc)]` arms, macros.rs:127-153) skipped; snapshot includes partial 6.4/6.5 content (far lower-ranked) plus ptr.rs and the `__parse_ensure!` header (explicitly below-the-fold).
- [markdown] [major] 2.2 (README "Details" bullets 1-2, README.md:21-67) skipped; displaced by ptr.rs struct declarations and 7.5 (ui/ listing).
- [rust] [major] 2.4 (Error constructor signatures `new`/`msg`/`from_boxed`, error.rs:19-36/75-82/137-143) skipped; snapshot includes ptr.rs (below-the-fold) and partial 4.5 / 5.1 / 6.x content.
- [markdown] [major] 2.5 (README bullets 3-6, README.md:68-122) skipped; same displacers as 2.2.
- [rust] [major] 2.6 (Error consumer-method signatures — context/backtrace/chain/root_cause/is/downcast*/into_boxed_dyn_error) skipped; displaced by ptr.rs and partial 4.5/5.1/6.x.
- [markdown] [major] 2.7 (README no-std + comparisons + license, README.md:124-180) skipped; same displacers.
- [rust] [major] 2.8 (tests/test_repr.rs, full file — one-word-repr invariant) skipped; filename only in the listing, displaced by ptr.rs and 7.5.
- [rust] [major] 2.9 (tests/test_autotrait.rs, full file) skipped; filename only.
- [rust] [major] 3.1 — present (lib.rs:299-388 rendered); no divergence for this batch.
- [rust] [major] 3.2 (test_fmt.rs:1-67 EXPECTED_* constants) skipped; filename only, displaced by partial 4.5/5.1/6.x and 7.5.
- [rust] [major] 3.3 (fmt.rs:1-67 Display/Debug impls head) skipped; filename only.
- [rust] [major] 3.4 (Context trait rustdoc, lib.rs:469-523) skipped while partial lower-ranked kind.rs/ensure.rs content is present.
- [rust] [major] 3.5 (context.rs Context impls for Result/Option) skipped; context.rs is a bare filename in the listing.
- [rust] [major] 3.7 (tests/test_chain.rs full) skipped; filename only.
- [rust] [minor] 4.1 (Error::context method body + rationale doc, error.rs:316-402) skipped while partial 4.5 / 5.1 / 6.x content is present (intra-major-4 inversion with 6.x cross-major as well).
- [rust] [minor] 4.2 (Error::into_boxed_dyn_error + reallocate_..._without_backtrace) skipped while partial 4.5 / 6.x content present.
- [rust] [minor] 4.3 (From<E>/Deref/DerefMut/Display/Debug/Drop impls for Error) skipped while partial 4.5 / 6.x present.
- [rust] [minor] 4.4 (kind.rs:1-46 autoref-dispatch design comment) skipped; snapshot's kind.rs content starts at line 55.
- [rust] [minor] 4.6 (fmt.rs:69-158 Indented formatter + tests) skipped while partial 6.2/6.3/6.4/6.5 present.
- [rust] [minor] 5.2/5.3/5.5/5.6 skipped while partial 6.2/6.3/6.4/6.5 present (inversion within majors 5 vs 6).
- [rust] [major] 6.1 (build.rs fn main cfg decision tree) skipped; build.rs is a bare filename while partial 6.2/6.3/6.4/6.5/6.6 content is present and ptr.rs (below-the-fold) content is present.
- [rust] [major] 6.7 (error.rs:1047-1086 From<Error> for Box + AsRef + UnwindSafe) skipped while 7.5 (lower-ranked) is present.
- [rust] [major] 7.1 (bail!/anyhow! rustdoc with examples, macros.rs:1-55 + 174-201) skipped.
- [rust] [major] 7.2 (__ensure! doc wrapper + cfg(not(doc)) dispatch arm) skipped.
- [other-language] [major] 7.3 (CI matrix from .github/workflows/ci.yml) skipped; only the directory path appears in the listing.
- [rust] [major] 7.6 (tests/test_source.rs full file) skipped; filename only.
- [rust] [major] 7.7 (test_ensure.rs preamble + per-test fn index) skipped; filename only.
- [rust] [major] 7.8 (cross-test #[test] fn index across 5 files) skipped.
- [rust] [predecessor] 4.5 partial content included without 4.4 (kind.rs:1-46 autoref-dispatch design comment); NS declares `4.5 Predecessor: 4.4`. Reader sees Adhoc/Trait/Boxed dispatch types with no explanation of the autoref method-resolution mechanism.
- [rust] [predecessor] 6.2 partial content included without 6.1 (build.rs); NS declares `6.2 Predecessor: 6.1`. Reader sees request_ref_backtrace/provide_ref_backtrace/provide signatures without the anyhow_build_probe/error_generic_member_access cfg story that gates them.
- [rust] [predecessor] 6.4 partial content included without 1.14 (`ensure!` cfg(doc) arms); NS declares `6.4 Predecessor: 1.14`. Reader sees BothDebug/NotBothDebug traits with no user-facing `ensure!` signature to motivate them.
- [rust] [predecessor] 6.5 partial content included without 6.4 full (and transitively 1.14); NS declares `6.5 Predecessor: 6.4`.
- [other-language] [major] ptr.rs struct declarations (`Own`, `Ref`, `Mut`, `CastTo`) are below-the-fold per the NS (ptr.rs is only referenced by `use crate::ptr::Own` in 2.1) yet are rendered, displacing 1.4/1.14/2.2/2.4/2.5/2.6/2.7/2.8/2.9.

### Batch correctness
- [markdown] [minor] 1.4 partially rendered at the wrong range: README.md:1-7 shown in place of 9-17 (title+badges rendered; the one-sentence description and the `anyhow = "1.0"` install snippet are omitted).
- [rust] [minor] 2.1 (lib.rs:246-263) partial: mod declarations (252-263) shown; `extern crate alloc;` / `#[cfg(feature="std")] extern crate std;` (246-251) omitted. Extra lib.rs:265-276 `use` lines also rendered, outside the batch.
- [rust] [minor] 2.10 (test_ffi.rs:1-19) partial: three `pub extern "C" fn` signature lines with bodies elided; the batch asks for lines 1-19 in full.
- [rust] [minor] 3.6 (chain.rs:1-102) partial: only `Chain` struct and `ChainState` enum declarations rendered; the Iterator/DoubleEndedIterator/ExactSizeIterator/Default impls and the `pub(crate) use` re-export are elided. NS explicitly notes "Splitting hurts — the impls only make sense together with the state machine."
- [rust] [minor] 3.8 (common/mod.rs + drop/mod.rs) partial: fn/struct/impl headers rendered with bodies elided; bail_* helpers and Flag/DetectDrop drop-detection bodies are unreadable from signatures alone.
- [rust] [minor] 4.5 (kind.rs:55-121) partial: the Adhoc/Trait/Boxed unit structs and the AdhocKind/TraitKind/BoxedKind traits with their `anyhow_kind` methods are shown, but the `impl Adhoc { fn new }` / `impl Trait { fn new }` / `impl Boxed { fn new }` blocks that actually construct Error values are omitted.
- [rust] [minor] 5.1 (error.rs:740-755 + 930-955) partial: only ErrorImpl<E> and ContextError<C,E> struct decls from the 930-955 range rendered; ErrorVTable struct (740-755), which is the "core trick" per NS, and the `vtable()` pointer-trick reader at 941-951 are missing.
- [rust] [minor] 5.4 (wrapper.rs:1-84) partial: only the three `pub struct` declarations (MessageError/DisplayError/BoxedError) rendered; the #[repr(transparent)] attrs, StdError impls, and Display/Debug forwarders that make the newtype pattern work are omitted.
- [rust] [minor] 6.2 (nightly.rs:1-58) partial: three `pub fn` signature lines (request_ref_backtrace/provide_ref_backtrace/provide) with bodies elided; the `#[cfg(anyhow_build_probe)] const _: ()` probe block (the build-script coupling per NS) is absent.
- [rust] [minor] 6.3 (backtrace.rs:1-68) partial: only a single `pub(crate) enum Backtrace {}` line at line 8 (one of three cfg arms); the impl_backtrace!/backtrace!/backtrace_if_absent! macros and the three-way dispatch tree are absent.
- [rust] [minor] 6.4 (ensure.rs:1-101) partial: BothDebug and NotBothDebug trait declarations shown; the Buf 40-byte stack buffer and the `render` function (which do the `(2 vs 1)` formatting per NS) are missing.
- [rust] [minor] 6.5 (ensure.rs:884-935) partial: only `macro_rules! __fancy_ensure` and `macro_rules! __fallback_ensure` header lines with immediate `…` elisions; the macro arms that define what `ensure!` expands to are absent.
- [rust] [minor] 6.6 (lib.rs:654-728 `__private` module) partial: only `pub mod __private {` header rendered with bare-ellipsis; `format_err`, `must_use`, `Bool` trait + impls, and `kind` re-exports are silently elided.
- [other-language] [minor] 7.4 (tests/crate/Cargo.toml + test.rs) partial: Cargo.toml lines 1-17 rendered; test.rs content absent (only the filename).

### Honesty
- (none)
