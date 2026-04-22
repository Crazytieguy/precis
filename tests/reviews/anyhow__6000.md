---
snapshot_hash: abcbfa5d1dc97e38bb3cd7dd545d23e08878c8cccc69e2bd35e594ceee490c3c
---

## Summary

At 6000 tokens the snapshot should have tier 1 complete, tier 2 substantially covered, and be touching tier 3. Instead it renders a broad shallow skeleton: tier-1 listings (1.1-1.3) and most tier-1 declarations (1.5-1.13) are present, but tier 2 is nearly absent (README Details 2.2/2.5/2.7, Error preamble 2.3, Error constructors 2.4, consumer method signatures 2.6, the invariant tests 2.8-2.10) and tier 3 is reduced to partial skeletons of 3.4/3.6/3.8 with 3.1/3.2/3.3/3.5/3.7 entirely missing. Budget is spent instead on below-the-fold prose (lib.rs:442-467, 524-615), unranked ptr.rs struct headers, and partial tier-4/5/6 file skeletons (kind.rs traits only, wrapper.rs structs only, error.rs ErrorImpl only, nightly/backtrace/ensure headers only). The dominant pattern is shallow, breadth-first walker output: one declaration line plucked from every file, producing batch-correctness violations across most of tiers 3-6 and many major-group displacement violations.

## Divergences

### Ranking
- [markdown] [major] 1.4 (README.md:9-17, headline + install snippet) missing; snapshot renders README.md:1-7 (title + badges, explicitly below-the-fold per NS) instead.
- [rust] [major] 1.14 (`ensure!` cfg(doc) arms, macros.rs:127-153) missing; snapshot renders `__parse_ensure!` header (below-the-fold, ensure.rs:103-883) and 6.5 `__fancy_ensure!`/`__fallback_ensure!` headers instead.
- [markdown] [major] 2.2 (README.md:21-67, `?`/`.context` tutorial) missing; below-the-fold lib.rs:442-467 (cluster_info example) and lib.rs:524-615 (SuspiciousError/HelperFailed) rendered in its place.
- [rust] [major] 2.3 (Error short doc preamble, lib.rs:288-298) missing; tier 4-6 skeletons present instead.
- [rust] [major] 2.4 (Error::new/msg/from_boxed signatures in error.rs) missing; snapshot surfaces only the much lower-ranked ErrorImpl/ContextError structs from error.rs:934-955.
- [markdown] [major] 2.5 (README.md:68-122, downcast/backtrace/thiserror/macros bullets) missing; tier 4-6 skeletons present instead.
- [rust] [major] 2.6 (Error consumer-method signatures: context/backtrace/chain/root_cause/is/downcast*/into_boxed_dyn_error) missing; tier 4-6 skeletons present instead.
- [markdown] [major] 2.7 (README.md:124-180, no-std + comparisons + license) missing; tier 4-6 skeletons present instead.
- [rust] [major] 2.8 (tests/test_repr.rs full, one-word-size invariant) missing; bare filename only, displaced by tier 4-6 skeletons.
- [rust] [major] 2.9 (tests/test_autotrait.rs full, auto-trait guarantees) missing; bare filename only.
- [rust] [major] 3.1 (Error Display-representations rustdoc, lib.rs:299-388) missing; tier 4-6 skeletons present instead.
- [rust] [major] 3.2 (test_fmt.rs:1-67 EXPECTED_* constants) missing; bare filename only.
- [rust] [major] 3.3 (fmt.rs:1-67 Display/Debug impls) missing; bare filename only.
- [rust] [major] 3.5 (context.rs Context impls for Result/Option) missing; context.rs is a bare filename in the listing.
- [rust] [major] 3.7 (tests/test_chain.rs full) missing; bare filename only.
- [rust] [major] 4.1 (Error::context method body + rationale doc, error.rs:316-402) missing.
- [rust] [major] 4.2 (Error::into_boxed_dyn_error + reallocate_..._without_backtrace, error.rs:582-672) missing.
- [rust] [major] 4.3 (From<E>/Deref/DerefMut/Display/Debug/Drop impls for Error, error.rs:691-738) missing.
- [rust] [major] 4.4 (kind.rs:1-46 autoref-dispatch design comment) missing; snapshot's kind.rs content starts at line 55.
- [rust] [major] 4.6 (fmt.rs:69-158 Indented formatter + tests) missing.
- [rust] [major] 5.2 (Error::construct_from_std/_adhoc/_display, error.rs:144-225) missing.
- [rust] [major] 5.3 (Error::construct_from_context/_boxed + unsafe construct<E>) missing.
- [rust] [major] 5.5 (Error::is/downcast/downcast_ref/downcast_mut bodies) missing.
- [rust] [major] 5.6 (ErrorImpl private accessors, error.rs:957-1013) missing.
- [rust] [major] 6.1 (build.rs fn main cfg decision tree) missing; build.rs is a bare filename.
- [rust] [major] 6.6 (lib.rs:654-728 __private module) missing; only `pub mod __private {` header with immediate elision.
- [rust] [major] 6.7 (error.rs:1047-1086 From<Error> for Box<dyn StdError> + AsRef + UnwindSafe) missing.
- [rust] [major] 7.1 (bail!/anyhow! rustdoc with examples, macros.rs:1-55 + 174-201) missing.
- [rust] [major] 7.2 (__ensure! doc wrapper + cfg(not(doc)) dispatch arm) missing.
- [other-language] [major] 7.3 (CI matrix from .github/workflows/ci.yml) missing; only the directory path is shown in the listing.
- [rust] [major] 7.5 (tests/ui/ directory listing as an explicit batch) — the listing is present via the general tree render, so not counted as missing here; noting for completeness.
- [rust] [major] 7.6 (tests/test_source.rs full file) missing; bare filename only.
- [rust] [major] 7.7 (test_ensure.rs preamble + per-test fn index) missing; bare filename only.
- [rust] [major] 7.8 (cross-test #[test] fn index across 5 files) missing.

### Batch correctness
- [markdown] [minor] 1.4 partially rendered at the wrong range: README.md:1-7 shown in place of 9-17 (title+badges rendered, description+install snippet omitted).
- [rust] [minor] 2.1 (lib.rs:246-263) partial: mod declarations (252-263) shown; `extern crate alloc;` / `#[cfg(feature="std")] extern crate std;` (246-251) omitted. Batch is also contaminated by lib.rs:265-276 (use statements, outside the batch).
- [rust] [minor] 2.10 (test_ffi.rs:1-19) partial: three `pub extern "C" fn` signature lines with bodies elided; the assertion bodies that actually exercise FFI-safety are missing.
- [rust] [minor] 3.6 (chain.rs:1-102) partial: only `Chain` struct and `ChainState` enum declarations rendered; the Iterator/DoubleEndedIterator/ExactSizeIterator/Default impls and the `pub(crate) use` re-export are missing. NS explicitly notes "Splitting hurts — the impls only make sense together with the state machine."
- [rust] [minor] 3.8 (common/mod.rs + drop/mod.rs) partial: only fn/struct/impl headers rendered with bodies elided; bail_* helpers and Flag/DetectDrop drop-detection bodies are unreadable from signatures alone.
- [rust] [minor] 4.5 (kind.rs:55-121) partial: the Adhoc/Trait/Boxed unit structs and the AdhocKind/TraitKind/BoxedKind traits with their `anyhow_kind` methods are shown, but the three `impl Adhoc::new` / `impl Trait::new` / `impl Boxed::new` bodies (lines 65-75, 87-97, 111-121) that actually construct Error values are omitted.
- [rust] [minor] 5.1 (error.rs:740-755 + 930-955) partial: only ErrorImpl<E> and ContextError<C,E> struct decls from the 930-955 range rendered; ErrorVTable struct (740-755), which is the "core trick" per NS, and the `vtable()` pointer-trick reader at 941-951 are missing.
- [rust] [minor] 5.4 (wrapper.rs:1-84) partial: only the three `pub struct` declarations (MessageError/DisplayError/BoxedError) rendered; the #[repr(transparent)] attrs, StdError impls, and Display/Debug forwarders that make the newtype pattern work are omitted.
- [rust] [minor] 6.2 (nightly.rs:1-58) partial: three `pub fn` signature lines (request_ref_backtrace/provide_ref_backtrace/provide) with bodies elided; the `#[cfg(anyhow_build_probe)] const _: ()` probe block (the build-script coupling per NS) is absent.
- [rust] [minor] 6.3 (backtrace.rs:1-68) partial: only a single `pub(crate) enum Backtrace {}` line at line 8 (one of three cfg arms); the impl_backtrace!/backtrace!/backtrace_if_absent! macros and the three-way dispatch tree are absent.
- [rust] [minor] 6.4 (ensure.rs:1-101) partial: BothDebug and NotBothDebug trait declarations shown; the Buf 40-byte stack buffer and the `render` function (which do the `(2 vs 1)` formatting per NS) are missing.
- [rust] [minor] 6.5 (ensure.rs:884-935) partial: only `macro_rules! __fancy_ensure` and `macro_rules! __fallback_ensure` header lines with immediate `…` elisions; the macro arms that define what `ensure!` expands to are absent.
- [other-language] [minor] 7.4 (tests/crate/Cargo.toml + test.rs) partial: Cargo.toml lines 1-17 rendered; test.rs content absent (only the filename).
- [rust] [predecessor] 4.5 partial content present without 4.4 (kind.rs:1-46 autoref-dispatch design comment). NS declares "4.5 Predecessor: 4.4"; reader sees Adhoc/Trait/Boxed dispatch types with no explanation of why autoref method resolution is the mechanism.
- [rust] [predecessor] 6.2 partial content present without 6.1 (build.rs cfg decision tree). NS declares "6.2 Predecessor: 6.1"; reader sees request_ref_backtrace/provide_ref_backtrace/provide without any story for the anyhow_build_probe/error_generic_member_access cfgs that gate them.
- [rust] [predecessor] 6.4 partial content present without 1.14 (`ensure!` cfg(doc) arms). NS declares "6.4 Predecessor: 1.14"; reader sees BothDebug/NotBothDebug autoref dispatch traits with no user-facing `ensure!` signature to motivate them.

### Honesty
- (none)
