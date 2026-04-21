---
snapshot_hash: 2e38d5222966d3907774681db3c485929f29ad622580553ebedc5507e0a551c9
---

## Summary

The snapshot at this budget is far from the North Star. It spends budget on broad directory skeletons (including plumbing like `.cargo/`, `.github/ISSUE_TEMPLATE/`, `.github/renovate.json5`) and on per-crate `Cargo.toml` headers + tiny per-file first-line doc comments, while skipping almost every batch above 1.5. High-value batches 1.2 (README one-liner), 1.6 (subcommand about-lines), 1.7 (workspace Cargo.toml), 1.8 (CLI dispatch), 1.9-1.13 (core traits and built-in preprocessor docs), and 1.14 (Guide SUMMARY.md) are absent, displaced by Below-the-fold Plumbing and Tooling content (sub-crate Cargo.toml headers, `crates/*/README.md`, module-doc line-1 snippets). The dominant pattern is a breadth-first directory-walker whose per-file slice never reaches the ranked doc ranges.

## Divergences

### Ranking
- [markdown] [major] 1.2 (README.md:7 one-liner) missing; snapshot includes README.md lines 1,3-5 (badges only, line 7 skipped) plus below-the-fold content such as `crates/mdbook-compare/README.md`, `crates/xtask/README.md`, and module-doc line-1 snippets from `src/main.rs`, `src/cmd/mod.rs`, `crates/mdbook-compare/src/main.rs` — all plumbing/tooling READMEs and tiny doc-comment slices ranked in Below-the-fold.
- [rust] [major] 1.6 (subcommand `.about()` map from `src/cmd/*.rs`) missing; displaced by sub-crate `Cargo.toml` headers for `mdbook-core`, `mdbook-driver`, `mdbook-html`, `mdbook-markdown`, `mdbook-preprocessor`, `mdbook-renderer`, `mdbook-summary`, `xtask`, `mdbook-compare` (Below-the-fold Plumbing/Tooling: "Sub-crate Cargo.toml files ~600 toks combined").
- [rust] [major] 1.7 (workspace `Cargo.toml:1-26` header) missing; displaced by the same sub-crate `Cargo.toml` headers listed above.
- [rust] [major] 1.8 (`src/main.rs:18-55` CLI subcommand dispatch) missing; snapshot only carries `src/main.rs:1` and `src/cmd/mod.rs:1` module docs — line-1 slices ranked in Below-the-fold "CLI internals".
- [rust] [major] 1.9 (`MDBook` struct fields) missing; displaced by below-the-fold per-crate `Cargo.toml` + `README.md` content.
- [rust] [major] 1.10 (`Preprocessor` trait) missing; displaced by below-the-fold per-crate content.
- [rust] [major] 1.11 (`Renderer` trait) missing; displaced by below-the-fold per-crate content.
- [rust] [major] 1.12 (`LinkPreprocessor` doc listing `{{# ... }}` helpers) missing; displaced by below-the-fold per-crate content.
- [rust] [major] 1.13 (`IndexPreprocessor` doc) missing; displaced by below-the-fold per-crate content.
- [markdown] [major] 1.14 (Guide `SUMMARY.md` full) missing; `guide/src/SUMMARY.md` appears only as a file-name entry. Displaced by expanded directory listings for plumbing such as `.cargo/`, `.github/ISSUE_TEMPLATE/`, `.github/workflows/`, `.github/renovate.json5`, `.gitignore` (pure plumbing, Below-the-fold).

### Batch correctness
- [rust] [minor] 1.4 (crate-name → one-line description map) is rendered indirectly: descriptions surface as part of full `Cargo.toml:1-8` header slices per crate, not as the compact map specified by the batch. Additionally `mdbook-compare/Cargo.toml` and `xtask/Cargo.toml` have no `description` field and the batch specifies a derived one-liner for those two — the snapshot emits neither a derived description nor any description line for those two crates, so the batch's semantic content is incomplete for 2 of 9 workspace members.
- [generic] [minor] 1.5 (`src/` and `src/cmd/` listings) is partially included: `src/cmd/` lists `watch.rs` and `watch/` as a sibling but does not expand `watch/` to show `watch/native.rs` and `watch/poller.rs` as the batch specifies. The batch's listing is truncated at the directory boundary.

### Honesty
- (none)
