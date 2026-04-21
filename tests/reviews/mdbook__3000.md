---
snapshot_hash: 2a523d2d44b95f5714698b17177a35e47d08735f28d479ed36518c4818f3750b
---

## Summary

At a 3000-token budget the snapshot is badly misaligned with the North Star. The budget is well below the ~24k of ranked content, so only the top of tier 1 should fit, yet the snapshot spends most of its tokens on plumbing that the North Star explicitly marks below-the-fold: every sub-crate `Cargo.toml` `[package]` header, every sub-crate `README.md` badge block, and interior slices of `src/cmd/clean.rs` / `src/cmd/watch.rs` (both named in the "CLI internals" below-the-fold list). As a result, almost every tier-1 and tier-2 batch from 1.2 onwards is either missing or only silently partially included. The dominant pattern is a tree-walker that shows file names for the right files but picks the wrong content slices and orders by filesystem position rather than North Star rank.

## Divergences

### Ranking
- [markdown] [major] 1.2 (README one-liner at `README.md:7`) is missing, but `README.md:1,3,4,5` (the title + three shields.io badges) are present. The badge block is scoped under 6.1 (`README.md:1-13`, a tier-6 batch); showing badges while eliding the actual one-liner inverts 1.2 vs 6.1.
- [markdown] [major] 1.2 missing while all eight `crates/*/README.md` badge blocks are present (explicitly in the below-the-fold "Plumbing" bullet: "`crates/*/README.md` (8 files, ~600 toks combined) — one-line readmes pointing at the user guide").
- [rust] [major] 1.6 (subcommand about-line map from `src/cmd/{build,init,clean,serve,test,watch}.rs`) is missing, but the snapshot includes `src/cmd/clean.rs:38 human_readable_bytes`, `src/cmd/clean.rs:46-50 pub struct Clean { ... }`, and the `WatcherKind` enum body at `src/cmd/watch.rs:21-27` — all three are named in the below-the-fold "CLI internals" bullets (`src/cmd/clean.rs` ~896 toks, `src/cmd/watch.rs` ~513 toks).
- [rust] [major] 1.8 (mdbook CLI subcommand dispatch, `src/main.rs:18-55`) is missing — only `src/main.rs:1` (the `//!` doc) is shown. The snapshot substitutes `clean.rs`/`watch.rs` interior slices from the below-the-fold CLI internals section.
- [rust] [major] 1.9 (`MDBook` struct fields, `crates/mdbook-driver/src/mdbook.rs:28-44`) is missing and the entire `crates/mdbook-driver/src/` directory is not expanded, while the snapshot keeps every sub-crate `Cargo.toml` `[package]` header (below-the-fold: "Sub-crate `Cargo.toml` files ... ~600 toks combined").
- [rust] [major] 1.10 (`Preprocessor` trait) is missing — `crates/mdbook-preprocessor/src/lib.rs` is listed but has no content. Displaced by `mdbook-preprocessor/README.md` badges (below-the-fold plumbing).
- [rust] [major] 1.11 (`Renderer` trait) is missing — `crates/mdbook-renderer/src/` is listed but has no content. Same displacement pattern.
- [rust] [major] 1.12 (`LinkPreprocessor` doc) is missing while `crates/mdbook-compare/README.md:1,3` content is present (below-the-fold plumbing).
- [rust] [major] 1.13 (`IndexPreprocessor` doc) is missing while `crates/xtask/README.md:1,3,4` content is present (below-the-fold plumbing).
- [markdown] [major] 1.14 (Guide `SUMMARY.md` full file) is missing — `guide/src/SUMMARY.md` is listed but has no content. Displaced by `guide/guide-helper/Cargo.toml:1-7` `[package]` header (below-the-fold).
- [rust] [major] All of 2.1-2.22 (core data model + config shape) are missing. The snapshot contains zero content from `crates/mdbook-core/src/`, `crates/mdbook-summary/src/`, `crates/mdbook-driver/src/lib.rs`, or `crates/mdbook-html/src/`, while the tokens that would fund them go to plumbing.
- [rust] [major] All of 3.1-3.17 (build pipeline + CLI surfaces) are missing except the partial 3.11 noted under Batch correctness. `make_subcommand` bodies for 3.1/3.2 are shown only as `pub fn make_subcommand() -> Command {` signature stubs with no body; the flag enumerations that make those batches load-bearing are absent.
- [rust] [major] All of 4.x and 5.x are missing (no `cmd::*::execute` bodies, no `nop-preprocessor` content, no `CONTRIBUTING.md` excerpt, no `workspace.dependencies`, no `guide/book.toml`, no `guide/src/README.md`). Meanwhile the snapshot keeps displaying sub-crate boilerplate.
- [rust] [minor] 3.3 (`command_prelude` whole file) is missing the `CommandExt` trait body — only `impl CommandExt for Command { fn _arg(self, arg: Arg) -> Self {` is shown, which is the impl head after the trait definition. With the body elided this is effectively missing; but `src/cmd/watch.rs:62-67 rebuild_on_change` signature (below-the-fold) is present.

### Batch correctness
- [rust] [major] 1.7 (`Cargo.toml:1-26`) is silently split: the snapshot shows `Cargo.toml:1-5` (`[workspace] members`) and `Cargo.toml:21-25` (`[workspace.package]`) but omits lines 8-19 (`[workspace.lints.clippy]` and `[workspace.lints.rust]`). A reader cannot tell the lints block existed. No ellipsis marker between the two slices.
- [rust] [major] 3.11 (`Cargo.toml:72-104`) is silently truncated: snapshot stops at line 88 (the end of the `[package]` metadata) and omits lines 89-104 (the `[dependencies]` block listing `clap`, `clap_complete`, `mdbook-*`, `opener`, `toml`, `tracing`). The batch's load-bearing content — the dependency list — is elided with no marker.
- [rust] [minor] 3.12 (`Cargo.toml:105-134`, optional deps + features) is absent entirely and 3.11 is partial, but `mdbook-driver/Cargo.toml:31-32` and `mdbook-html/Cargo.toml:36-37` `[features] search = [...]` fragments are included, giving a scattered per-crate feature view while the binary's own feature table is omitted.

### Honesty
- [markdown] `README.md` slice shows lines 1, 3, 4, 5 with no trailing ellipsis marker. A reader cannot see that line 7 (the one-liner `mdBook is a utility to create modern online books from Markdown files.`) exists — this is a non-obvious elision because the snapshot stops right before the most load-bearing line in the file. Quoted lines 1/3/4/5 match the fixture verbatim.
- [rust] `Cargo.toml` root slice shows `1-5`, jumps to `21-25`, then jumps to `72-88` with no ellipsis markers between the three ranges. All quoted lines match the fixture verbatim, but the two silent gaps (lines 6-20 and 26-71) hide `[workspace.lints]` and the full `[workspace.dependencies]` (44 crate pins) respectively.
- [rust] `src/cmd/clean.rs` slice alternates visible line numbers (11, 19, 38, 46-50) with no markers for the bodies between them; a reader cannot tell whether the function bodies were read and summarized or simply skipped.
