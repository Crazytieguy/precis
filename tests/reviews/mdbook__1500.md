---
snapshot_hash: 2e38d5222966d3907774681db3c485929f29ad622580553ebedc5507e0a551c9
---

## Summary

At 1500 tokens the snapshot is dominated by a wide directory tree plus eight sub-crate `Cargo.toml` package-header fragments and a badge-only README excerpt. The dominant pattern is tree-and-plumbing displacing content: below-the-fold material (`.cargo/`, `.github/ISSUE_TEMPLATE/`, `.github/workflows/`, `ci/`, per-crate `Cargo.toml` metadata, crate READMEs, per-file line-1 doc comments) is shown while Tier-1 batches 1.2, 1.6, 1.7, 1.8, 1.9-1.13, 1.14 and all of Tier-2+ are absent. The snapshot gives enough structure to locate files but almost none of the load-bearing content.

## Divergences

### Ranking

- [markdown] [major] 1.2 (README.md:7 one-liner, "mdBook is a utility to create modern online books from Markdown files.") is missing, while README lines 1/3/4/5 (badge-only head of 6.1) are shown. A Tier-6 surface is displacing the highest-value 14-token Tier-1 batch.
- [rust] [major] 1.6 (subcommand → about-line map from `src/cmd/*.rs`) is missing; `src/cmd/mod.rs:1` doc comment and the `src/cmd/` file listing appear in its place, along with the below-the-fold sub-crate `Cargo.toml` headers.
- [rust] [major] 1.7 (workspace `Cargo.toml:1-26` header: `[workspace] members`, `[workspace.lints]`, `[workspace.package]`) is missing; per-crate `[package]` fragments (below-the-fold plumbing) appear instead.
- [rust] [major] 1.8 (`src/main.rs:18-55` CLI dispatch) is missing; only `src/main.rs:1` doc-comment line is shown — a slice ranked in below-the-fold "CLI internals".
- [rust] [major] 1.9 (`MDBook` struct fields, `crates/mdbook-driver/src/mdbook.rs:28-44`) is missing; below-the-fold `mdbook-driver/Cargo.toml` package header + `[features]` fragment is shown instead.
- [rust] [major] 1.10 (`Preprocessor` trait) is missing; displaced by below-the-fold `mdbook-preprocessor/Cargo.toml` header.
- [rust] [major] 1.11 (`Renderer` trait) is missing; displaced by below-the-fold `mdbook-renderer/Cargo.toml` header.
- [rust] [major] 1.12 (`LinkPreprocessor` doc listing `{{# include}}`/`{{# rustdoc_include}}`/`{{# playground}}`/`{{# title}}`) is missing; displaced by below-the-fold per-crate `Cargo.toml` content.
- [rust] [major] 1.13 (`IndexPreprocessor` doc) is missing; displaced by below-the-fold per-crate `Cargo.toml` content.
- [markdown] [major] 1.14 (Guide `SUMMARY.md` full) is missing — `guide/src/SUMMARY.md` appears only as a filename entry. Displaced by expanded plumbing listings (`.cargo/config.toml`, `.github/ISSUE_TEMPLATE/*`, `.github/renovate.json5`, `.gitignore`, full `.github/workflows/`, full `ci/`).
- [markdown] [minor] 6.1 is partially shown (lines 1,3,4,5) but 1.2 (line 7, within the same 1-13 span) is not; lower-ranked badge context displaces the higher-ranked sentence.
- [generic] [major] Pure plumbing entries (`.cargo/config.toml`, `.github/ISSUE_TEMPLATE/{bug_report,feature_request,question}.yml`, `.github/renovate.json5`, `.gitignore`) appear while every Tier-2+ batch is absent. These are named in the Plumbing section of below-the-fold.
- [rust] [major] Below-the-fold per-file line-1 doc-comment snippets (`src/main.rs:1`, `src/cmd/mod.rs:1`, `crates/mdbook-compare/src/main.rs:1`) appear while Tier-1 content is absent.

### Batch correctness

- [markdown] [minor] 1.1 violated: the rendered top-level listing includes `.gitignore`, `.cargo/`, and `.github/` which the North Star's 1.1 enumeration explicitly excludes (it starts at `CHANGELOG.md` and ends at `triagebot.toml`). The batch is supposed to be an orthogonal named set of 17 entries; the snapshot merges it into a tree with dotfiles interleaved.
- [markdown] [minor] 6.1 violated: of `README.md:1-13` only lines 1, 3, 4, 5 appear. Lines 2, 6 (blanks) and lines 7-13 (the intro paragraph and the links to User Guide / Contribution Guide) are silently elided. A reader cannot tell from the snapshot that the paragraph exists.
- [rust] [minor] 1.5 violated: `src/cmd/` listing shows `watch/` and `watch.rs` as siblings, but `src/cmd/watch/native.rs` and `src/cmd/watch/poller.rs` — both enumerated in the batch — are omitted (the `watch/` subdir is unexpanded).
- [rust] [minor] 1.4 violated: the snapshot conveys crate descriptions only via fragmentary `Cargo.toml` `[package]` blocks (with name, version, description, plus workspace boilerplate). It does not emit a compact crate→description map, and for `mdbook-compare`/`xtask` (no `description` field) the batch calls for a derived one-liner — the snapshot emits neither a derived description nor any description line for those two, so 2 of 9 workspace members are missing description content.
- [rust] [minor] Sub-crate `Cargo.toml` plumbing emitted partially: for `mdbook-driver` and `mdbook-html`, non-contiguous `[features]` snippets (lines 31-32 and 36-37 respectively) are appended to the `[package]` block with no indication of the gap, so a reader cannot tell what `[dependencies]` lie between.

### Honesty

- (none)
