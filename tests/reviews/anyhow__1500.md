---
snapshot_hash: 0f44de9e28e47829d93dca69c50dcd7da6721e2d77bb714635095c13333fd5bc
---

## Summary

At ~1500 tokens the snapshot behaves like a "skeleton of everything" rather than a budget-respecting prefix of the North Star. Tier-1 listings 1.1 and 1.2 are rendered (1.1 with `.github/` expanded into children the batch doesn't cover), but 1.3 (`tests/` children), 1.4 (README headline), and 1.14 (`ensure!` cfg-doc arms) are absent, and every tier-2 batch (README tutorial, constructor and consumer signatures, invariant tests) is missing. In their place the snapshot surfaces shallow stubs that map to batches 3.6, 5.1, 5.4, 6.2, 6.3, 6.4, 6.5 plus below-the-fold `ptr.rs` declarations — a wide-but-shallow first-pass walker pattern that produces many partial-batch inclusions where the North Star wanted a deep, contiguous tier-1/tier-2 slice.

## Divergences

### Ranking
- [rust] [minor] 1.3 (`tests/` 18-entry listing) absent; snapshot renders `tests/` with no children while lower-ranked stubs of 3.6, 5.1, 5.4, 6.2, 6.3, 6.4, 6.5 are present and displace it.
- [markdown] [major] 1.4 (`README.md:9-17` headline + install snippet) absent; displaced by lower-ranked stubs from majors 3-6.
- [rust] [major] 1.14 (`macros.rs:127-153` `#[cfg(doc)]` ensure! arms) absent; displaced by lower-ranked stubs of 6.4 and 6.5.
- [rust] [major] 2.1 (`lib.rs:246-263` extern crate + mod declarations) absent; displaced by stubs from 5.x and 6.x.
- [markdown] [major] 2.2 (`README.md:21-67` canonical tutorial) absent; displaced by stubs of 3.6 (`chain.rs:11-24`), 5.1 (`error.rs:934-955`), 5.4 (`wrapper.rs` struct-decl lines), 6.2 (`nightly.rs` fn sig lines), 6.3 (`backtrace.rs:8`), 6.4 (`ensure.rs:10-27`), 6.5 (`ensure.rs:886,912`).
- [rust] [major] 2.3 (`lib.rs:288-298` Error short-doc preamble) absent; displaced as above.
- [rust] [major] 2.4 (`error.rs` constructor signatures `new`/`msg`/`from_boxed`) absent; displaced by 5.x/6.x stubs.
- [markdown] [major] 2.5 (`README.md:68-122` downcasting/backtrace-env/thiserror/macros) absent; displaced.
- [rust] [major] 2.6 (`Error` consumer-method signatures — `context`/`backtrace`/`chain`/`root_cause`/`is`/`downcast`/`downcast_ref`/`downcast_mut`/`into_boxed_dyn_error`/`reallocate_...`) absent; displaced by 5.1 `ErrorImpl` fragment.
- [markdown] [major] 2.7 (`README.md:124-180` no-std + comparisons + license) absent; displaced.
- [rust] [major] 2.8 (`tests/test_repr.rs` full file, one-word-repr invariant) absent; displaced.
- [rust] [major] 2.9 (`tests/test_autotrait.rs` full file) absent; displaced.
- [rust] [major] 2.10 (`tests/test_ffi.rs` full file) absent; displaced.
- [rust] [major] Below-the-fold `src/ptr.rs` declarations (`Own`, `Ref`, `Mut`, `CastTo`) rendered even though `ptr.rs` body is not ranked at all — displaces every missing tier-1/2 batch above.

### Batch correctness
- [generic] [minor] 1.1 violated: the rendered root listing expands `.github/` into `FUNDING.yml` and `workflows/ci.yml`, adding two entries the North Star batch excludes (batch is the 10 top-level entries flat).
- [rust] [minor] 1.5 violated: `Cargo.toml` rendered as lines 1-12 of the 1-13 batch range.
- [rust] [minor] 1.6 violated: snapshot shows 390-392 only, omitting line 389 (`#[repr(transparent)]`) — the attribute is the load-bearing half of the "one machine word" claim.
- [rust] [minor] 1.7 violated: snapshot shows 468 only, omitting 467 (the preceding doc-stub line).
- [rust] [minor] 1.9 violated: snapshot shows only line 650 header of `pub fn Ok<T>`; the other half of the batch (`pub use anyhow as format_err;` at `lib.rs:285-286`) is missing entirely.
- [rust] [minor] 1.10 violated: snapshot shows 415-417 only, omitting 413-414 — the `#[cfg(any(feature = "std", not(anyhow_no_core_error)))]` and `#[derive(Clone)]` attributes the North Star calls out as load-bearing.
- [rust] [minor] 1.11 violated: snapshot shows only lines 14-16 (`[features]`, `default`, `std`), omitting 18-30 (`[dependencies]` with the optional `backtrace` dep + comment, and `[dev-dependencies]`) — the content the North Star flags as "catastrophic-omission risk."
- [rust] [minor] 1.12 violated: snapshot shows 58-68, omitting 56-57 (`#[macro_export]` + `clippy::format_args` cfg_attr).
- [rust] [minor] 1.13 violated: snapshot shows 204-223, omitting 202-203 (`#[macro_export]` + cfg_attr). Snapshot also surfaces a `__anyhow` header stub at line 230 which is not in any tier-1 ranked batch.
- [rust] [major] 3.6 violated: batch is `chain.rs:1-102` full file; snapshot shows only 11-24 (struct declarations), omitting the `Iterator`/`DoubleEndedIterator`/`ExactSizeIterator`/`Default` impls the North Star explicitly says must stay together with the state machine.
- [rust] [major] 5.1 violated: batch is `error.rs:740-755` joined with `:930-955` via an elision marker; snapshot shows only 934-940 (ErrorImpl fields) and 952-955 (ContextError) — the entire `ErrorVTable` struct is missing, and the `vtable()` accessor between 940 and 952 is silently elided without a marker.
- [rust] [major] 5.4 violated: batch is `wrapper.rs:1-84` full file; snapshot shows only the three `pub struct` declaration lines, omitting the `#[repr(transparent)]` annotations and all impls (the repr is specifically called out as the safety condition for vtable downcasts).
- [rust] [major] 6.2 violated: batch is `nightly.rs:1-58` full file; snapshot shows only a few function signature lines, omitting the `#[cfg(anyhow_build_probe)] const _` probe block and every body.
- [rust] [major] 6.3 violated: batch is `backtrace.rs:1-68`; snapshot shows only line 8 (`pub(crate) enum Backtrace {}`), omitting the three-way cfg dispatch and all three macros (`impl_backtrace!`/`backtrace!`/`backtrace_if_absent!`).
- [rust] [major] 6.4 violated: batch is `ensure.rs:1-101`; snapshot shows only lines 10-27 (BothDebug/NotBothDebug trait declarations), omitting the `Buf` 40-byte stack buffer and the `render` function.
- [rust] [major] 6.5 violated: batch is `ensure.rs:884-935`; snapshot surfaces only the two macro headers (886, 912) with bodies elided. Snapshot also includes a stub of line 105 (`macro_rules! __parse_ensure {`) which corresponds to content the North Star explicitly places below-the-fold.
- [rust] [predecessor] 6.2 (`nightly.rs` fragments) present without predecessor 6.1 (`build.rs:1-97`) — `build.rs` appears only as a root-listing filename, not its body. The declared `Predecessor: 6.1` edge is violated.

### Honesty
- [rust] In the 5.1 rendering at `error.rs`, lines 934-940 and 952-955 appear contiguously with no elision marker between them, even though lines 941-951 (including the `vtable()` accessor) are omitted. Spot-checked against `tests/fixtures/anyhow/src/error.rs` — the intervening content is real and was silently dropped; reader cannot tell.
