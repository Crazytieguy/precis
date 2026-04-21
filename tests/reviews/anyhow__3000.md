---
snapshot_hash: b58582ecf5efd3347b9dc541edcf31ef93c76823f226395ee71ec2e145f6050e
---

## Summary

At a 3k budget the North Star's ranked front-loading expects the snapshot to sit inside tier 1 and the top of tier 2 — roughly 1.1 through 2.3/2.4. The snapshot instead uses its budget to sprinkle shallow "header only" fragments from deep-ranked batches (kind.rs tier-4 traits, nightly.rs tier-6 function signatures, wrapper.rs tier-5 structs, ensure.rs tier-6 trait headers, fmt.rs tier-3 impl headers, ptr.rs which isn't ranked at all) while *omitting* or silently *truncating* most of tier 1 and skipping all of tier 2 and tier 3. The dominant pattern is therefore: (a) severe ranking inversions that pull material from tiers 4-6 above essentially all of tiers 2-3, and (b) systemic partial-inclusion of multi-range batches — headers/declarations kept, bodies elided without the reader being able to tell what's been dropped.

## Divergences

### Ranking
- [markdown] [major] 1.4 (README `:9-17` headline+install) is absent; snapshot instead carries README `:1-7` (title + four badge lines, not part of any ranked batch) plus lower-ranked Rust content from tiers 4-6 (e.g., `kind.rs` traits from 4.5, `nightly.rs` fn signatures from 6.2, `wrapper.rs` structs from 5.4). Skips the highest-value "what is this crate" batch while keeping tier-4+ content.
- [rust] [major] 1.9's `pub use anyhow as format_err` (lib.rs `:285-286`) is absent; `pub fn Ok` half of the same batch is present, and content from 4.5/5.1/6.2 is present. Major inversion.
- [rust] [major] 1.14 (`ensure!` user-facing `#[cfg(doc)]` arms, macros.rs `:127-153`) is absent; snapshot instead surfaces the `__parse_ensure!`/`__fancy_ensure!`/`__fallback_ensure!` macro-header lines (6.5 / below-the-fold material) and the `BothDebug`/`NotBothDebug` trait headers from 6.4. Tier-6/below-the-fold content displaces tier-1 content.
- [rust] [major] 2.1 (`extern crate` + 11 internal `mod` decls, lib.rs `:246-263`) is absent; lower-ranked tier-4-6 content is present throughout.
- [markdown] [major] 2.2 / 2.5 / 2.7 (README `Details` bullets and no-std/comparison/license footer) are all absent; tier-4-6 Rust content is present.
- [rust] [major] 2.3 (`Error` short doc preamble, lib.rs `:288-298`) absent; tier-4-6 content present.
- [rust] [major] 2.4 (`Error` constructor signatures from error.rs) absent; tier-5 `ErrorImpl`/`ContextError` decl from 5.1 IS present, so the agent sees the private vtable struct before the public constructors.
- [rust] [major] 2.6 (`Error` consumer-method signatures) absent; tier-5 `ErrorImpl` struct present instead.
- [rust] [major] 2.8 / 2.9 (full `test_repr.rs`, `test_autotrait.rs`) absent; lower-ranked partial fragments of `test_context.rs`, `test_ensure.rs`, `test_source.rs`, `test_chain.rs` are shown in their place.
- [rust] [major] 2.10 (full `test_ffi.rs:1-19`) — most of the file IS shown, but it's placed alongside partial content from test files that rank strictly below it while tier-1 / early-tier-2 Rust+Markdown content above it is missing.
- [markdown] [major] 3.1 (`Error` Display-representations rustdoc, lib.rs `:299-388`) absent; much lower-ranked tier-5/6 content present.
- [rust] [major] 3.2 (`test_fmt.rs` EXPECTED_* constants) absent; snapshot names `test_fmt.rs` but shows no content — at the same time it shows `kind.rs` trait bodies (4.5) and `nightly.rs` signatures (6.2).
- [rust] [major] 3.3 (`fmt.rs:1-67` `Display`/`Debug` impl block) present only as impl+fn headers with bodies elided; displaces nothing, but tier-4-6 content (kind.rs, nightly.rs) is present while 3.3 is not fully materialized — still a ranking inversion against missing tier-2 batches.
- [rust] [major] 3.4 / 3.5 (`Context` trait rustdoc + `Context` impls in context.rs) absent; `context.rs` is named in the snapshot with zero content shown, while `kind.rs` (4.5) is rendered with full trait bodies.
- [rust] [major] 3.6 / 3.7 / 3.8 (full `chain.rs`, `test_chain.rs`, shared test helpers) — 3.8 is partially present (common/mod.rs headers, drop/mod.rs headers) but bodies are elided; 3.6 shows only the `Chain`/`ChainState` struct decls. Meanwhile kind.rs (4.5) and nightly.rs (6.2) and wrapper.rs (5.4) appear at higher fidelity, which is a tier-3-vs-tier-4/5/6 inversion.
- [rust] [major] 4.1 / 4.2 / 4.3 / 4.6 absent while 4.5 (kind.rs traits) is rendered essentially in full — inversion inside tier 4 is [minor], but the displacing of absent tier 2-3 content is [major].
- [rust] [minor] 4.4 (kind.rs autoref-dispatch design comment, `:1-46`) absent; 4.5 (kind.rs traits + `new` impls, `:55-121`) is present. Stays inside the tier-4 major group, and is also a predecessor violation (see below).
- [rust] [major] 5.2 / 5.3 / 5.5 / 5.6 absent; 5.1 is partially present and 5.4 (wrapper.rs structs) is partially present — but the bigger issue is that all of tier-2-3 is missing while these tier-5 fragments appear.
- [rust] [major] 6.1 (build.rs fn main / cfg decision tree) absent; 6.2 (nightly.rs, which has `Predecessor: 6.1`) is partially present. The build.rs content in the snapshot is only the `use std::...` header lines (`:3-10`, not ranked), not the decision tree.
- [rust] [major] 6.3 (backtrace.rs cfg dispatch + macros) absent; only `pub(crate) enum Backtrace {}` (one line) is shown.
- [rust] [major] 6.6 / 6.7 absent; 6.4 and 6.5 are each only partially rendered (trait headers / macro headers without bodies).
- [rust] [major] 7.1 / 7.2 / 7.3 / 7.4 / 7.5 / 7.6 / 7.7 / 7.8 absent; much lower-priority tier-4-6 fragments are present instead.
- [other-language] [major] ptr.rs struct decls (`Own`, `Ref`, `Mut`, `CastTo`) are rendered despite ptr.rs not appearing in any ranked batch of the North Star — ranking dead-weight that displaces budget away from missing tier-1 / tier-2 batches.

### Batch correctness
- [rust] [major] 1.9 violated: `pub fn Ok<T>` (lib.rs `:650-652`) is present but `pub use anyhow as format_err` (lib.rs `:285-286`) — the other half of the batch — is silently omitted. Reader has no signal that the re-export alias exists.
- [other-language] [major] 1.11 violated: only the `[features]` block (Cargo.toml `:14-16`) is kept; the `[dependencies]` block (including `backtrace = { version = "0.3.51", optional = true }` with its no-std comment) and the entire `[dev-dependencies]` block (lines 17-30) are silently dropped — North Star flags this as catastrophic-omission risk.
- [rust] [major] 1.12 violated: only `macro_rules! bail {` header (line 58) is shown with an ellipsis marker; the three body arms (lines 59-68) are elided. Batch is supposed to be the complete 56-68 definition.
- [rust] [major] 1.13 violated: only `macro_rules! anyhow {` header (line 204) is shown with an ellipsis marker; the three body arms (lines 205-223) are elided.
- [rust] [major] 2.10 violated: file `tests/test_ffi.rs` is shown with the three `extern "C" fn` signatures (`:7`, `:12`, `:17`) and the `use` (`:4`) — but each fn body is elided with `…`, and the North Star wants the full 1-19 (there is no rustdoc to split — the file is 19 lines total). Partial inclusion.
- [rust] [major] 3.3 violated: `impl ErrorImpl { display, debug }` (fmt.rs `:6-67`) is rendered as fn signatures with bodies elided (e.g., `display` at `:7` with `…`, `debug` at `:20` with `…`, `write_str` at `:79` with `…`). The entire batch is supposed to be `:1-67` as a contiguous block — the `: ` separator / `Caused by:` header / `Indented` numbering logic is precisely the load-bearing content in the elided bodies.
- [rust] [major] 3.8 violated: both `common/mod.rs` and `drop/mod.rs` are rendered as signatures only, bodies elided (`bail_literal`, `bail_fmt`, `bail_error` and `Flag::new`, `Flag::get`, `DetectDrop::new`, `fmt`, `drop` all appear with `…`). The whole point of 3.8 is making these primitives legible — signatures alone can't substitute for 14 + 53 lines of code.
- [rust] [minor] 4.5 mostly intact (trait defs + fn bodies for `anyhow_kind` inlined). OK.
- [rust] [major] 5.1 violated: `ErrorImpl<E>` (error.rs `:934-940`) and `ContextError<C, E>` (`:952-955`) ARE shown, but the `ErrorVTable` struct (`:740-755`) — the first half of the batch and the "core trick" of anyhow — is entirely absent. Reader gets the client of the vtable without the vtable.
- [rust] [major] 5.4 violated: wrapper.rs shows only the three struct declaration lines (`MessageError`, `DisplayError`, `BoxedError` at lines 11, 34, 58) — the rest of the 84-line file (the `#[repr(transparent)]` attributes, `StdError` impls, `Display` impls) is silently dropped. Transparent layout is the entire safety argument for this file.
- [rust] [major] 6.2 violated: nightly.rs is rendered as three function signatures (`request_ref_backtrace`, `provide_ref_backtrace`, `provide` at lines 41, 52, 56) with bodies elided; the `#[cfg(anyhow_build_probe)] const _: () = { ... }` probe block (the whole reason 6.2 is coupled to 6.1) is absent.
- [rust] [major] 6.4 violated: only the `pub trait BothDebug` and `pub trait NotBothDebug` declarations (lines 10-12, 25-27) are shown; the `Buf` no-alloc 40-byte stack buffer and the `render` function — the load-bearing content per the North Star notes — are entirely missing.
- [rust] [major] 6.5 violated: only the `macro_rules! __fancy_ensure {` (line 886) and `macro_rules! __fallback_ensure {` (line 912) headers are shown, both with `…` for bodies. Batch is supposed to be `:884-935`.

### Honesty
- [rust] Spot-checked snapshot line `src/ensure.rs 10→pub trait BothDebug {` against fixture — verified, line 10 of ensure.rs is indeed `pub trait BothDebug {`. OK.
- [rust] Spot-checked snapshot `src/lib.rs 468→pub type Result<T, E = Error> = core::result::Result<T, E>;` against fixture — verified.
- [rust] Spot-checked snapshot `src/error.rs 934→pub(crate) struct ErrorImpl<E = ()> {` against fixture — verified.
- [rust] Spot-checked `src/kind.rs 55→pub struct Adhoc;` — verified.
- (none otherwise — no suspicious paraphrasing detected in spot-checks; the snapshot's elision markers visibly declare what's been dropped.)
