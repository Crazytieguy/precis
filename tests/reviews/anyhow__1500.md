---
snapshot_hash: ef9a323a162da63c9ab5ffe9c1cf4bd52c23d892e8b06b02c2635a799a4a13e9
---

## Summary

At ~1500 tokens the snapshot behaves like a "skeleton of everything" rather than a budget-respecting prefix of the North Star: every `src/` module appears as a stub (a handful of signatures or declaration fragments) and the first two tier-1 listings are rendered in full, but batches 1.3 (`tests/` listing) and 1.4 (README headline + install) are absent while content that maps to far-lower-ranked batches (3.6, 5.1, 6.3, 6.4, 6.5) is present in partial form. Dominant pattern: wide-but-shallow first-pass walker output producing many partial-batch inclusions where the North Star wanted a deeper, contiguous tier-1 slice plus a few later batches.

## Divergences

### Ranking
- [rust] [minor] North Star batch 1.3 (`tests/` directory listing, 72 tokens) is absent — the snapshot renders only `tests/` with no children — while stubs of batches 1.12, 1.13, 3.6, 5.1, 6.3, 6.4, 6.5 are present; any of those displace 1.3.
- [markdown] [minor] North Star batch 1.4 (`README.md:9-17`, the headline + install snippet) is absent; snapshot renders `README.md:1-7` (title + four badge lines) instead, which is not a North Star batch (the closest parallel, `lib.rs:1-13` badges, is explicitly below-the-fold). Lower-ranked content present (1.5 partial, 1.8, 1.12 stub, 3.6 stub, 5.1 stub, etc.) displaces 1.4.
- [rust] [minor] North Star batch 1.14 (`src/macros.rs:127-153`, the `#[cfg(doc)]` `ensure!` arms) is absent; lower-ranked stubs of 3.6, 5.1, 6.3, 6.4, 6.5 are present and displace it.
- [rust] [minor] North Star batch 2.1 (`src/lib.rs:246-263`, `extern crate` + internal `mod` declarations, 60 tokens) is absent; multiple stubs of ≥3.x batches are present.
- [markdown] [major] North Star batch 2.2 (`README.md:21-67`, the canonical tutorial) is absent; snapshot instead allocates budget to partial stubs of batches in majors 3-6 (e.g., `chain.rs:11-24` → 3.6, `error.rs:934-955` → 5.1, `backtrace.rs:8` → 6.3, `ensure.rs:10-27` → 6.4, `ensure.rs:886,912` → 6.5). Crosses the 2.x/3.x and 2.x/5.x boundaries.
- [rust] [major] North Star batch 2.4 (`error.rs` constructor signatures `new`/`msg`/`from_boxed`) is absent; stubs from major-5 and major-6 batches are present.
- [rust] [major] North Star batch 2.6 (`Error` consumer-method signatures — `downcast`, `chain`, `backtrace`, etc.) is absent; stubs mapping to 5.1 (`error.rs:934-955`) and 6.x are present.

### Batch correctness
- [rust] [minor] Batch 1.6 (`src/lib.rs:389-393`) rendered partially: snapshot shows lines 390-392 (the `pub struct Error { ... }` body) and omits line 389 (`#[repr(transparent)]`), which is load-bearing context.
- [rust] [minor] Batch 1.7 (`src/lib.rs:467-468`) rendered partially: snapshot shows only line 468; line 467 (closing doc line `/// ```) omitted.
- [rust] [minor] Batch 1.9 rendered partially: `pub fn Ok<T>` (lines 650-…) is shown (truncated after the signature line), but `pub use anyhow as format_err;` at `src/lib.rs:285-286` — the other half of the batch — is missing.
- [rust] [minor] Batch 1.10 (`src/lib.rs:413-417`) rendered partially: shows 415-417 only; the `#[cfg(any(feature = "std", not(anyhow_no_core_error)))]` at 413 and `#[derive(Clone)]` at 414 are omitted, which removes the cfg predicate the North Star flags as load-bearing.
- [rust] [minor] Batch 1.11 (`Cargo.toml:14-30`) rendered partially: snapshot shows only lines 14-16 (`[features] / default = ["std"] / std = []`); omits the `[dependencies]` block with the optional `backtrace` dep and the `[dev-dependencies]` block — exactly the content the North Star flags as "catastrophic-omission risk".
- [rust] [minor] Batch 1.12 (`src/macros.rs:56-68`, `bail!`) rendered as header-only stub: only line 58 (`macro_rules! bail {`) shown with an ellipsis; the three macro arms are elided.
- [rust] [minor] Batch 1.13 (`src/macros.rs:202-223`, `anyhow!`) rendered as header-only stub: only line 204 (`macro_rules! anyhow {`) shown with an ellipsis; three arms elided. Snapshot also includes a `__anyhow` header stub at line 230 which is not part of any tier-1 ranked batch.
- [rust] [minor] Batch 3.6 (`src/chain.rs:1-102`) rendered partially: snapshot shows lines 11-24 only (the `Chain` and `ChainState` struct declarations); omits all impls (`Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `Default`) which the North Star explicitly calls out as needing to stay together with the state machine.
- [rust] [minor] Batch 5.1 (`src/error.rs:740-755` + `:930-955`) rendered partially: snapshot shows lines 934-939 (`ErrorImpl<E>` fields) and 952-955 (`ContextError<C, E>`); omits the entire `ErrorVTable` struct at 740-755 and the repr/header lines at 930-933 and 940-951. The North Star says splitting this batch defeats the point — the vtable struct is the batch's load-bearing half.
- [rust] [minor] Batch 6.3 (`src/backtrace.rs:1-68`) rendered as a single-line stub: snapshot shows only line 8 (`pub(crate) enum Backtrace {}`); omits the cfg dispatch tree and all three macros (`impl_backtrace!`, `backtrace!`, `backtrace_if_absent!`).
- [rust] [minor] Batch 6.4 (`src/ensure.rs:1-101`) rendered partially: snapshot shows lines 10-27 (the `BothDebug` / `NotBothDebug` trait declarations only); omits `Buf` and `render` helpers.
- [rust] [minor] Batch 6.5 (`src/ensure.rs:884-935`) rendered as header-only stubs: snapshot shows just `macro_rules! __fancy_ensure {` at 886 and `macro_rules! __fallback_ensure {` at 912, with ellipses and no bodies. Line 105 (`macro_rules! __parse_ensure {`) is also surfaced as a stub though it corresponds to content the North Star explicitly places below-the-fold.

### Honesty
- (none)
