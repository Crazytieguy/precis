---
snapshot_hash: ba2b51b64f2b7af7494b1d9d848e99d4a5e33bf1b6731e2bf7419f32d5b100f4
---

## Summary

The snapshot skims wide and shallow at this budget. It names almost every file in the tree and surfaces most top-level public type/trait/macro declarations, but omits nearly every piece of prose or body content that the North Star ranks in tiers 2 and 3. The dominant pattern is "declaration shown, body/rustdoc withheld" across many files (error.rs, wrapper.rs, chain.rs, nightly.rs, backtrace.rs, ensure.rs, kind.rs-partial) combined with "file-named-only" for most tier-2/3 test files (test_repr, test_autotrait, test_chain, test_fmt, test_source) and for build.rs, fmt.rs, context.rs. The two README pillars of the 60-second tour (`:21-67` for `?`/`.context(...)`/`Caused by` and `:68-122` for downcasting/env vars/thiserror/`anyhow!`) are absent — only the title/badges (below-the-fold), the `:9-17` install snippet (1.4), and the `:126-179` no-std/comparison/license (2.7) are rendered. Meanwhile the snapshot spends tokens on lib.rs rustdoc the NS places below-the-fold (`:1-13` badges, `:394-412` Chain rustdoc, `:419-467` Result rustdoc + cluster_info example, `:630-647` Ok rustdoc) and on `ptr.rs` struct declarations (not in any ranked batch). The net effect is many cross-major ranking inversions and three clean predecessor violations.

## Divergences

### Ranking
- [rust] [major] 1.14 (`src/macros.rs:127-153` — `ensure!` user-facing `#[cfg(doc)]` arms) missing; displaced by below-the-fold `lib.rs:1-13` badges, `lib.rs:394-412` / `:419-467` / `:630-647` rustdoc, the explicitly-below-the-fold `__parse_ensure!` header at `ensure.rs:105`, and `ptr.rs` declarations.
- [rust] [major] 2.1 missing `extern crate` lines — `lib.rs:246-251` (`extern crate alloc;` + `#[cfg(feature="std")] extern crate std;`) absent; `lib.rs:252-263` (11 `mod ...;` lines) present. Displaced by below-the-fold `lib.rs` rustdoc ranges and `ptr.rs`.
- [markdown] [major] 2.2 (`README.md:21-67` — `?` propagation, `.context(...)`, `Caused by:`) missing; displaced by below-the-fold `lib.rs:1-13` badges, `lib.rs:394-412` / `:419-467` / `:630-647` rustdoc, and `ptr.rs`.
- [rust] [major] 2.3 (`src/lib.rs:288-298` — `Error` doc preamble: Send+Sync+'static, backtrace, one word wide) missing; lib.rs jumps from `:286` directly to `:390`. Displaced by below-the-fold `lib.rs:394-412` / `:419-467` / `:630-647` and `ptr.rs`.
- [rust] [major] 2.4 (`src/error.rs:19-36, 75-82, 137-143` — `Error::new` / `msg` / `from_boxed` signatures) missing; error.rs contains only `:934-939` and `:952-954`. Displaced by below-the-fold `lib.rs` rustdoc and `ptr.rs`.
- [markdown] [major] 2.5 (`README.md:68-122` — downcast_ref, `RUST_BACKTRACE`, thiserror, `anyhow!`/`bail!`) missing; same displacers.
- [rust] [major] 2.6 (`src/error.rs` consumer-method signatures — context/backtrace/chain/root_cause/is/downcast*/into_boxed_dyn_error) missing; same displacers.
- [rust] [major] 2.8 (`tests/test_repr.rs` full file — one-word-repr invariant) missing; filename only. Displaced by below-the-fold `lib.rs` rustdoc, `ptr.rs`, and 7.4 (`tests/crate/Cargo.toml`) / 7.5 (`tests/ui/` listing).
- [rust] [major] 2.9 (`tests/test_autotrait.rs` full file) missing; filename only. Same displacers.
- [markdown] [major] 3.1 (`src/lib.rs:299-388` — Error Display-representations rustdoc) missing; displaced by below-the-fold `lib.rs:394-412` / `:419-467` / `:630-647`, `ptr.rs`, and 7.4 / 7.5.
- [rust] [major] 3.2 (`tests/test_fmt.rs:1-67` EXPECTED_* constants — executable formatter spec) missing; filename only.
- [rust] [major] 3.3 (`src/fmt.rs:1-67` Display/Debug impls for ErrorImpl) missing; fmt.rs appears as an empty filename.
- [rust] [major] 3.4 (`src/lib.rs:469-523` — `Context` rustdoc + ImportantThing + downcast rule) missing; displaced by below-the-fold lib.rs rustdoc ranges and ptr.rs.
- [rust] [major] 3.5 (`src/context.rs:1-68, 70-113` — Context impls for Result and Option) missing; context.rs appears as an empty filename.
- [rust] [major] 3.6 (`src/chain.rs` full file — ChainState machine + Iterator impls) missing; chain.rs renders only `struct Chain` and `enum ChainState` headers at lines 11-23.
- [rust] [major] 3.7 (`tests/test_chain.rs` full file) missing; filename only.
- [rust] [minor] 4.1 (`src/error.rs:316-402` — `Error::context` method body + rationale doc) missing; displaced by 4.5-partial and 6.2/6.3/6.4/6.5/6.6-partial (cross-major 4→6 inversion).
- [rust] [minor] 4.2 (`src/error.rs:582-672` — `into_boxed_dyn_error` + `reallocate_..._without_backtrace`) missing; same displacers.
- [rust] [minor] 4.3 (`src/error.rs:691-738` — From/Deref/DerefMut/Display/Debug/Drop for Error) missing; same displacers.
- [rust] [minor] 4.4 (`src/kind.rs:1-46` — autoref-dispatch design comment) missing; kind.rs content starts at `:55`. Displaced by 4.5 (which declares 4.4 as predecessor) and by 6.2/6.3/6.4/6.5/6.6-partial.
- [rust] [minor] 4.6 (`src/fmt.rs:69-158` — `Indented` formatter helper + tests) missing; displaced by 6.2/6.3/6.4/6.5/6.6-partial.
- [rust] [minor] 5.2 (`src/error.rs:144-225` — `construct_from_std/_adhoc/_display`), 5.3 (`error.rs:226-314` — `construct_from_context/_boxed` + unsafe `construct` core), 5.5 (`error.rs:482-580` — is/downcast/downcast_ref/downcast_mut bodies), 5.6 (`error.rs:957-1013` — ErrorImpl private accessors) all missing; displaced by 6.2/6.3/6.4/6.5/6.6-partial (cross-major 5→6 inversion).
- [rust] [major] 6.1 (`build.rs:1-97` — cfg decision tree) missing; build.rs is a bare filename. Displaced by partial 6.2/6.3/6.4/6.5/6.6 content and by `ptr.rs`, 7.4, 7.5.
- [rust] [major] 6.7 (`src/error.rs:1047-1086` — From<Error> for Box + AsRef + UnwindSafe) missing; displaced by 7.4, 7.5.
- [rust] [major] 7.1 (`src/macros.rs:1-55, 174-201` — bail!/anyhow! rustdoc) missing.
- [rust] [major] 7.2 (`src/macros.rs:70-124, 155-172` — `__ensure!` doc wrapper + `#[cfg(not(doc))]` dispatch arm) missing.
- [other-language] [major] 7.3 (`.github/workflows/ci.yml` matrix + minimal-versions + clippy/miri) missing; ci.yml appears only as a path in the directory listing.
- [rust] [major] 7.6 (`tests/test_source.rs` full file) missing; filename only.
- [rust] [major] 7.7 (`tests/test_ensure.rs:1-39` preamble + per-test fn index) missing; filename only.
- [rust] [major] 7.8 (cross-test `#[test] fn` index across test_boxed / test_context / test_convert / test_downcast / test_macros) missing.
- [rust] [predecessor] 4.5-partial (Adhoc/Trait/Boxed + AdhocKind/TraitKind/BoxedKind declarations) is included without 4.4 (`src/kind.rs:1-46` autoref-dispatch design comment). NS declares `4.5 Predecessor: 4.4`. Reader sees the dispatch types with no explanation of the autoref method-resolution mechanism.
- [rust] [predecessor] 6.2-partial (nightly.rs fn signatures) is included without 6.1 (`build.rs:1-97`). NS declares `6.2 Predecessor: 6.1`. Reader sees `request_ref_backtrace` / `provide_ref_backtrace` / `provide` without the `anyhow_build_probe` / `error_generic_member_access` cfg story that gates compilation.
- [rust] [predecessor] 6.4-partial (`BothDebug` / `NotBothDebug` trait declarations) is included without 1.14 (`src/macros.rs:127-153` `ensure!` `#[cfg(doc)]` arms). NS declares `6.4 Predecessor: 1.14`. Reader sees the dispatch traits with no user-facing `ensure!` signature to motivate them.
- [other-language] [major] `ptr.rs` struct declarations (`Own`, `Ref`, `Mut`, `CastTo`) rendered as a near-full below-the-fold file not present in any ranked batch, displacing 1.14 / 2.1-2.6 / 2.8 / 2.9 / 3.x / 4.x / 5.x / 6.1 / 6.7 / 7.1-7.3 / 7.6-7.8.

### Batch correctness
- [toml] [minor] 1.5 partial: `Cargo.toml:1-12` rendered; line 13 (the blank line closing the `[package]` block per NS `:1-13`) is elided. Batch asks for `:1-13`.
- [rust] [minor] 1.6 partial: `lib.rs:390-392` (struct header + field + `}`) rendered; the load-bearing `#[repr(transparent)]` attribute at `:389` is elided — the whole point of splitting the declaration off from its rustdoc per NS notes.
- [rust] [minor] 1.7 partial: `lib.rs:468` (the `pub type Result<T, E = Error>` alias line) rendered; batch asks for `:467-468` (the prior doc-stub line and the alias). The snapshot also bleeds extra content — lines 419-467 (the long `Result` rustdoc + cluster_info example) are NS-below-the-fold.
- [rust] [minor] 1.9 partial: `lib.rs:650` (the `pub fn Ok<T>` signature) rendered but attributes `#[allow(non_snake_case)] #[inline]` at 648-649 and body at 651-652 elided; `lib.rs:286` (`pub use anyhow as format_err;`) rendered but `#[doc(no_inline)]` at `:285` elided.
- [rust] [minor] 1.10 partial: `lib.rs:415-417` (struct header + field) rendered but `#[cfg(any(feature = "std", not(anyhow_no_core_error)))]` and `#[derive(Clone)]` at 413-414 elided — the cfg NS explicitly calls out as load-bearing.
- [rust] [minor] 1.12 partial: `macros.rs:58-68` (the three-arm `macro_rules! bail`) rendered; attributes `#[macro_export]` at `:56` and `#[cfg_attr(not(anyhow_no_clippy_format_args), clippy::format_args)]` at `:57` elided.
- [rust] [minor] 1.13 partial: `macros.rs:204-223` (the three-arm `macro_rules! anyhow`) rendered; attributes at 202-203 elided.
- [rust] [minor] 2.1 partial: `lib.rs:252-263` (11 `mod ...;` lines) rendered; `extern crate alloc;` / `#[cfg(feature="std")] extern crate std;` pair at `:246-251` is absent. Extra `lib.rs:265-276` `use` lines are rendered outside the batch.
- [markdown] [minor] 2.7 partial: `README.md:126-179` rendered; batch asks for `:124-180` (leading `<br>`/blank before `## No-std support` and trailing blank line elided).
- [rust] [minor] 2.10 partial: `test_ffi.rs` shows only the three `pub extern "C" fn` signature lines (7, 12, 17) with bodies elided; batch specifies `:1-19` in full.
- [rust] [minor] 3.8 partial: `common/mod.rs` and `drop/mod.rs` rendered with function/struct/impl headers only; bodies (the `bail_*` helpers and the `Flag`/`DetectDrop` drop-detection primitives) are elided, leaving them unreadable from signatures alone.
- [rust] [predecessor] 4.5 partial: the Adhoc/Trait/Boxed unit structs and the AdhocKind/TraitKind/BoxedKind traits with `anyhow_kind` default methods rendered, but the `impl Adhoc { fn new }` / `impl Trait { fn new }` / `impl Boxed { fn new }` blocks (which actually construct Error values and complete the batch `:55-121`) are omitted. Severity elevated by 4.5's declared `Predecessor: 4.4` being absent.
- [rust] [major] 5.1 partial: only `pub(crate) struct ErrorImpl<E = ()>` at `:934-939` and `pub(crate) struct ContextError<C, E>` at `:952-954` rendered; the `ErrorVTable` struct at `:740-755` (the "core trick" per NS) and the `vtable()` pointer reader are missing. Silent elision of half the batch.
- [rust] [minor] 5.4 partial: `wrapper.rs` renders only the three `pub struct MessageError<M>(pub M);` / `DisplayError<M>` / `BoxedError` header lines (11, 34, 58); batch specifies the full 84-line file including the `#[repr(transparent)]` attributes (the safety condition for vtable downcasts per NS) and the StdError/Display/Debug impls.
- [rust] [minor] 6.2 partial: `nightly.rs` renders only the three `pub fn` signature lines (41, 52, 56) with bodies elided; batch specifies `:1-58` in full including the `#[cfg(anyhow_build_probe)] const _: () = { ... }` probe block (dual-purpose-with-build.rs per NS).
- [rust] [minor] 6.3 partial: `backtrace.rs` renders only `pub(crate) enum Backtrace {}` at line 8 (one of three cfg arms); batch specifies `:1-68` including the three-way `Backtrace` re-export decision tree plus the `impl_backtrace!` / `backtrace!` / `backtrace_if_absent!` macros.
- [rust] [minor] 6.4 partial: `ensure.rs` renders only the `BothDebug` and `NotBothDebug` trait header lines (10-12, 25-27); batch specifies `:1-101` including the `Buf` 40-byte stack buffer and the `render` function (which produce the `(2 vs 1)` formatting suffix on `ensure!` errors per NS).
- [rust] [minor] 6.5 partial: `macro_rules! __fancy_ensure` and `macro_rules! __fallback_ensure` appear only as header lines (886, 912) with ellipsis; batch specifies the arm bodies at `:884-935` which define what `ensure!` actually expands to.
- [rust] [minor] 6.6 partial: `pub mod __private` appears only as the `{` header line (656) with bare ellipsis; batch specifies `:654-728` including `format_err`, `must_use`, the `Bool` trait + impls, and the `kind` re-exports.
- [other-language] [minor] 7.4 partial: `tests/crate/Cargo.toml` lines 1-17 rendered; `test.rs` content is absent (filename only). Batch is a two-file pairing.

### Honesty
- (none)
