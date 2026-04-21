---
snapshot_hash: 2e38d5222966d3907774681db3c485929f29ad622580553ebedc5507e0a551c9
---

## Summary

At the 1500-token budget the snapshot spends almost all of its bytes on a wide recursive directory tree plus per-crate `Cargo.toml` `[package]` headers and the README title/badges, while skipping nearly every curated tier-1 batch. The dominant pattern is breadth-over-depth: cheap tree nodes and below-the-fold plumbing (crate `Cargo.toml` metadata, crate READMEs, first-line doc comments) are shown while batches 1.2, 1.6, 1.7, 1.8, 1.9, 1.10, 1.11, 1.12, 1.13, and 1.14 are entirely absent.

## Divergences

### Ranking
- [markdown] [major] 1.2 (README.md:7 one-liner, 14 toks) missing; README.md is shown with lines 1,3,4,5 (title + badges — part of 6.1) but the one-liner on line 7 is omitted. Tier-6 partial content displacing the highest-value 14-token Tier-1 batch.
- [rust] [major] 1.6 (subcommand about-line map, 92 toks) missing; `src/cmd/{build,clean,init,serve,test,watch}.rs` appear only as bare filenames. Displaced by below-the-fold per-crate `Cargo.toml` headers and crate READMEs.
- [rust] [major] 1.7 (workspace Cargo.toml:1-26 header, 198 toks) missing; root `Cargo.toml` appears only as a filename. Displaced by below-the-fold per-sub-crate `Cargo.toml` `[package]` fragments.
- [rust] [major] 1.8 (src/main.rs:18-55 CLI dispatch, 291 toks) missing; only the `//! The mdbook CLI.` doc line is shown — a slice ranked in below-the-fold "CLI internals".
- [rust] [major] 1.9 (MDBook struct fields, 116 toks) missing; `crates/mdbook-driver/src/` is a collapsed directory node. Displaced by below-the-fold `mdbook-driver/Cargo.toml` header + `[features]` fragment.
- [rust] [major] 1.10 (Preprocessor trait, 133 toks) missing; displaced by below-the-fold `mdbook-preprocessor/Cargo.toml` header.
- [rust] [major] 1.11 (Renderer trait, 108 toks) missing; displaced by below-the-fold `mdbook-renderer/Cargo.toml` header.
- [rust] [major] 1.12 (LinkPreprocessor `{{# ... }}` helper list, 171 toks) missing; displaced by below-the-fold per-crate `Cargo.toml` content.
- [markdown] [major] 1.13 (IndexPreprocessor doc, 102 toks) missing; displaced by below-the-fold per-crate content.
- [markdown] [major] 1.14 (guide/src/SUMMARY.md full, 364 toks) missing; `guide/src/SUMMARY.md` appears only as a filename in the tree. Displaced by expanded below-the-fold plumbing (full `.cargo/`, `.github/ISSUE_TEMPLATE/*`, `.github/workflows/*`, `.gitignore`, `ci/*`, guide/ subtree).
- [generic] [major] Below-the-fold plumbing entries (`.cargo/config.toml`, `.github/ISSUE_TEMPLATE/{bug_report,feature_request,question}.yml`, `.github/renovate.json5`, `.gitignore`) appear while every Tier-2+ batch is absent.
- [rust] [major] Below-the-fold per-file line-1 doc-comment snippets (`src/main.rs:1`, `src/cmd/mod.rs:1`, `crates/mdbook-compare/src/main.rs:1`) appear while Tier-1 batches 1.6-1.14 are absent.

### Batch correctness
- [generic] [minor] 1.1 violated: the top-level listing includes `.cargo/`, `.github/`, `.gitignore` which the North Star's 17-entry enumeration explicitly excludes; the batch's named set is interleaved with dotfiles and rendered as a tree rather than the specified per-line listing.
- [rust] [minor] 1.4 violated: descriptions appear fragmentarily via per-crate `Cargo.toml` `[package]` blocks for 7 of 9 crates (mdbook-core, -driver, -html, -markdown, -preprocessor, -renderer, -summary). `mdbook-compare` and `xtask` have no `description =` field, and the batch's derived one-liners for those two are not emitted — silent partial inclusion (reader cannot tell 2 of 9 are missing).
- [rust] [minor] 1.5 violated: `src/cmd/` shows `watch/` as a collapsed directory; `src/cmd/watch/native.rs` and `src/cmd/watch/poller.rs`, both enumerated in the batch, are omitted.
- [markdown] [minor] 6.1 violated: of `README.md:1-13`, only lines 1, 3, 4, 5 appear. The intro paragraph (line 7) and the User Guide / CONTRIBUTING links (lines 9-13) are silently elided, so a reader cannot tell from the snapshot that they exist.
- [rust] [minor] Sub-crate Cargo.toml fragments emitted non-contiguously for `mdbook-driver` (lines 1-8 then 31-32) and `mdbook-html` (lines 1-8 then 36-37) with no ellipsis or gap marker — a reader cannot tell `[dependencies]` lies between.

### Honesty
- (none)
