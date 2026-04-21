---
snapshot_hash: 2cd1b67e3692a9c6df7a960485efc79579f0ee08a5b88f6f2cdc32a1e22fc537
---

## Summary

The snapshot is a first-pass walker-style rendering of the whole tree (directory listings plus scattered declaration skeletons with elided bodies), not a North-Star-aligned precis at a 6k budget. It includes most Tier 1 declarations (1.5-1.11 are largely present) but systematically skips almost every Tier 2 and Tier 3 batch (README prose, Error docs and constructor/consumer signatures, Display-representations rustdoc, test_repr/autotrait/ffi full bodies, Context rustdoc, Chain full, test_chain, shared test helpers) while still surfacing chunks of Tier 4-6 material (wrapper.rs declarations, kind.rs trait declarations, ensure.rs runtime helpers, ErrorImpl/ContextError structs, build.rs imports, nightly.rs signatures, fmt.rs signature heads). The dominant pattern is a depth-first, uniform-skeleton walker output that ignores batch groupings: many batches are violated by partial inclusion (headers + elided bodies), and several predecessor edges are violated because higher-ranked user-facing macro/README content is omitted while lower-ranked internals are shown.

## Divergences

### Ranking
- [rust] [predecessor] 1.14 (`ensure!` user-facing arms, macros.rs:127-153) is missing, yet 6.4 (ensure.rs runtime helpers, `BothDebug`/`NotBothDebug`/`Buf`) is partially surfaced. 6.4 declares `Predecessor: 1.14`.
- [rust] [predecessor] 4.4 (kind.rs autoref-dispatch design comment, lines 1-46) is missing — snapshot's kind.rs section starts at line 47 — yet 4.5 (kind.rs traits and `Adhoc`/`Trait`/`Boxed` declarations) is present. 4.5 declares `Predecessor: 4.4`.
- [rust] [predecessor] 6.1 (build.rs cfg decision tree, lines 1-97) is missing — only the `use` block 3-10 is shown — yet 6.2 (nightly.rs signatures) is partially present. 6.2 declares `Predecessor: 6.1`.
- [rust] [predecessor] 2.6 (`Error` consumer-method signatures) is missing, yet 3.3 (fmt.rs 1-67 `display`/`debug` impls) is partially present. 3.3 declares `Predecessor: 2.6`.
- [markdown] [major] 1.4 (README:9-17 "what is this" + install snippet) is omitted while README:1-7 (title + badges) is shown; lower-ranked 5.4 (wrapper.rs file), 6.2 (nightly.rs fragments), and 6.4 (ensure.rs runtime) are present.
- [rust] [major] 1.12 (`bail!` macro body, macros.rs:56-68) is missing (only the `macro_rules! bail {` header line is shown with `…`), while lower-ranked 4.5 (kind.rs traits), 5.4 (wrapper.rs), 6.4 (ensure.rs runtime) are present.
- [rust] [major] 1.13 (`anyhow!` macro body, macros.rs:202-223) is missing (only header + `…`), while the same tier 4-6 internals are present.
- [markdown] [major] 2.2 (README:21-67 `?` propagation + `.context(...)` tutorial) is missing while lower-ranked internals (wrapper.rs full, kind.rs traits, ensure.rs runtime) are present.
- [rust] [major] 2.3 (`Error` short doc preamble, lib.rs:288-298) is missing while tier 4-6 internals are present.
- [rust] [major] 2.4 (`Error::new`/`msg`/`from_boxed` constructor signatures in error.rs) is missing — the error.rs section jumps from the `use` block straight to line 934 — while 5.1 partial (ErrorImpl/ContextError structs) and 5.4 (wrapper.rs) are present.
- [markdown] [major] 2.5 (README:68-122 downcasting/backtrace env/thiserror/macros bullets) is missing while tier 4-6 internals are present.
- [rust] [major] 2.6 (`Error` consumer-method signatures — downcast/chain/backtrace/etc.) is missing while 5.4 (wrapper.rs), 4.5 (kind.rs traits), 6.4 (ensure.rs runtime) are present.
- [markdown] [major] 2.7 (README:124-180 no-std + comparison + license) is missing while tier 4-6 internals are present.
- [rust] [major] 2.8 (test_repr.rs full file, the one-word repr invariant) is missing — only `use` lines 3-7 are shown — while 5.4 wrapper.rs body and 4.5 kind.rs trait declarations are present.
- [rust] [major] 2.9 (test_autotrait.rs full file) is missing (only `use` lines 3-4 shown) while lower-ranked internals are present.
- [rust] [major] 2.10 (test_ffi.rs full file) is missing (only `use` + fn signatures with `…` bodies shown) while lower-ranked internals are present.
- [rust] [major] 3.1 (`Error` Display-representations rustdoc, lib.rs:299-388) is missing while 5.4 (wrapper.rs full) and 6.4 (ensure.rs runtime) are present.
- [rust] [major] 3.2 (test_fmt.rs:1-67 expected-output constants) is missing (only `use` lines 1-2 shown) while lower-ranked internals are present.
- [rust] [major] 3.3 (fmt.rs:1-67 Display/Debug impls) is only partially present — display/debug signature headers at lines 7 and 20 are shown but bodies are elided — while 4.5 and 5.4 declarations are present.
- [rust] [major] 3.4 (`Context` trait rustdoc + `ImportantThing` example) is missing while lower-ranked internals are present.
- [rust] [major] 3.5 (context.rs:1-113 `mod ext` + Result/Option impls) is missing — only the `mod ext { … }` header and `mod private { … }` header are shown — while lower-ranked 5.4 wrapper.rs and 6.4 ensure.rs runtime are present.
- [rust] [major] 3.6 (chain.rs full file — Chain iterator impls) is only partially present: struct/enum declarations and impl headers are shown but method bodies are uniformly elided with `…`, while lower-ranked content (5.4 wrapper.rs declarations) has the same partial treatment but the iterator semantics that make 3.6 useful live in the elided bodies.
- [rust] [major] 3.7 (test_chain.rs full) is missing (only `use` line 1 shown) while lower-ranked internals are present.
- [rust] [major] 3.8 (common/mod.rs + drop/mod.rs shared test helpers) is partially present — fn signatures shown, bodies elided — while lower-ranked 5.4 wrapper.rs content is similarly partial.

### Batch correctness
- [markdown] [minor] 1.4: only README:1-7 (title + badges) is included; the North Star's content window is 9-17 (one-sentence description + `anyhow = "1.0"` install snippet), which is entirely missing.
- [rust] [minor] 1.12: macros.rs:58 header `macro_rules! bail {` is shown with the body elided via `…`. The batch's content is the three-arm body spanning 56-68; header-only is a silent partial inclusion.
- [rust] [minor] 1.13: same shape — only the `macro_rules! anyhow {` header at line 204 is shown; the three-arm body 202-223 is elided.
- [rust] [minor] 2.1: the 11 `mod ...;` declarations (252-263) are shown, but the `extern crate alloc;` and `#[cfg(feature = "std")] extern crate std;` at 246-250 are not.
- [rust] [minor] 5.1: only the `ErrorImpl<E>` struct at 934-939 and `ContextError` at 952-955 are present; the `ErrorVTable` struct at 740-755 and the intervening `vtable()` reader are missing. The North Star pairs these two ranges as a single batch.
- [rust] [minor] 3.3: the `impl ErrorImpl { display, debug }` headers at fmt.rs:7 and :20 are shown but bodies are elided; the `Indented` `Write` impl header at :75 is shown but body elided; the full 1-67 range is not coherently represented.
- [rust] [minor] 3.6: chain.rs struct/enum/impl headers are shown but every method body (`next`, `next_back`, `size_hint`, `len`, `default`, `new`) is elided. The state machine is unreadable without the bodies.
- [rust] [minor] 4.5: Adhoc/Trait/Boxed structs and the three `*Kind` traits (55-121) have their declarations shown, but the `new()` method bodies (lines 72-76, 94-98, 117-121 approximately) are elided with `…`, making the autoref-dispatch wiring opaque.
- [rust] [minor] 5.4: wrapper.rs structs and `impl` headers are shown; every `fmt` body and the `source`/`provide` bodies are elided.
- [rust] [minor] 6.2: nightly.rs `use`s and the three fn signatures (`request_ref_backtrace`, `provide_ref_backtrace`, `provide`) are shown with bodies elided; the `#[cfg(anyhow_build_probe)] const _` block is absent.
- [rust] [minor] 6.4: ensure.rs trait/struct declarations and fn headers are shown (`BothDebug`, `NotBothDebug`, `Buf::new`, `Buf::as_str`, `Write for Buf`), but bodies and the `render` function are elided.

### Honesty
- (none)
