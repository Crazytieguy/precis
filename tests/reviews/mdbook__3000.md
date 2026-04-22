---
snapshot_hash: b6fc3ae9bb369c44082e84b3cb6e384a210a9244ede0362bffa9d9f35702f7bd
---

## Summary

At the 3000-token budget, the snapshot is dominated by a full recursive directory tree plus skeletal `Cargo.toml` excerpts (each crate's `[package]`) and per-crate README header blocks. It gets tier-1 structural orientation largely right (1.1 repo root, 1.2 README one-liner via 6.1-embedded, 1.3 workspace members, 1.4 crate descriptions via per-crate `Cargo.toml:1-8`, 5.2 examples listing, 5.4 tests listing, 5.11 ci/workflows). But it skips nearly every code-surface batch above the cut: no `src/cmd/` contents (1.5 partial — `src/cmd/` renders as an empty directory), no subcommand about-lines (1.6), no `main.rs` dispatch (1.8 — only the `//!` file-doc line shown), and **nothing from tier 2** (`MDBook`, `Book`, `Config`, `Preprocessor`/`Renderer` traits, `SUMMARY.md` parser, `mdbook-html` lib root — all `crates/*/src/` directories render empty or with only `lib.rs` as a leaf). Instead, the snapshot spends budget on below-the-fold plumbing: the full `.github/ISSUE_TEMPLATE/` tree, per-crate README badge stanzas repeated across 7 crates, and the `[[bin]]`/`[[example]]`/`[[test]]` manifest tail at `Cargo.toml:135-153` (the "Plumbing" bullet in Below-the-fold). The dominant pattern is breadth-over-depth: scaffolding (directory paths, README headers, manifest tails) displaces ranked code surfaces.

## Divergences

### Ranking
- [rust] [major] 1.5 (`src/` + `src/cmd/` listings) skipped — `src/cmd/` shown as an empty directory — while below-the-fold `Cargo.toml:135-153` manifest tail (`[[bin]]`/`[[example]]`/`[[test]]`, NS §Plumbing/CLI internals) is present.
- [rust] [major] 1.6 (subcommand → about-line map) skipped; displaced by the Cargo.toml manifest tail and per-crate README badge blocks (below-the-fold plumbing).
- [toml] [major] 1.7 tail (`[workspace.lints.*]` at `Cargo.toml:6-20`) skipped; `Cargo.toml:135-153` below-the-fold manifest tail is present (see also Batch correctness entry).
- [rust] [major] 1.8 (`src/main.rs:18-55` CLI dispatch) skipped — only the line-1 doc comment appears — while below-the-fold plumbing content is present.
- [rust] [major] 1.9 (`MDBook` struct fields) skipped; `crates/mdbook-driver/src/` renders empty while below-the-fold `crates/mdbook-driver/Cargo.toml:31-32` `[features]` stanza + per-crate README badges are present.
- [rust] [major] 1.10 (`Preprocessor` trait) skipped; `crates/mdbook-preprocessor/src/` shows only `lib.rs` as a leaf with no content. Same plumbing displacement.
- [rust] [major] 1.11 (`Renderer` trait) skipped; `crates/mdbook-renderer/src/` shows only `lib.rs` with no content. Same plumbing displacement.
- [rust] [major] 1.12 (`LinkPreprocessor` `{{# ... }}` helper list) skipped; displaced by same below-the-fold content.
- [rust] [major] 1.13 (`IndexPreprocessor` doc) skipped; displaced by same.
- [markdown] [major] 1.14 (full `guide/src/SUMMARY.md`) skipped — filename appears in tree but content is absent — while below-the-fold per-crate README badges and Cargo.toml manifest tail are present.
- [rust] [major] Tier 2 batches 2.1-2.22 (`mdbook-driver`/`mdbook-core`/`mdbook-summary` lib roots, `Book`/`BookItem`/`Chapter`/`Config`/`BookConfig`/`BuildConfig`/`Summary`/`Link`/`SummaryItem`/`parse_summary`/grammar/`PreprocessorContext`/`RenderContext`/`MDBook::load`/`MDBook::init`/`with_renderer`/`MDBook::test`/`build_dir_for`/built-in-preprocessors module/`mdbook-html` lib root/pipeline/recursive listing/built-in-renderers) all skipped while below-the-fold plumbing (Cargo.toml:135-153 tail, all 7 `crates/*/README.md` badge stanzas, `.github/ISSUE_TEMPLATE/*` enumeration, `.cargo/config.toml`) is present.
- [rust] [major] Tier 3 batches 3.1-3.17 (`make_subcommand` bodies, `command_prelude`, `MDBook::build`, `mdbook-markdown` crate, `LinkPreprocessor`/`IndexPreprocessor`/`CmdRenderer`/`CmdPreprocessor`, mdbook binary `[dependencies]`/optional-deps, `HtmlConfig`, `Theme` assets, `Config::from_disk`, config module doc) all skipped; same displacement.
- [rust] [major] Tier 4 batches 4.1-4.14 (`cmd::*::execute` bodies, `determine_renderers`/`determine_preprocessors`, `preprocessor_should_run`, `load_book`, `compose_command`, `BookBuilder`, `Book` impls, static-files/search/admonition heads) all skipped; same displacement.
- [rust] [minor] 5.5 (`tests/testsuite/` file listing) skipped — `tests/testsuite/` renders as an empty directory — while tier-5 5.11 (`ci/`+`.github/workflows/`) and 5.2 (`examples/`) listings are present and below-the-fold plumbing is still being emitted.
- [markdown] [minor] 6.2 (`CHANGELOG.md:1-23` current release notes) skipped while 6.1 (README intro) is included.

### Batch correctness
- [toml] [minor] 1.7 partial: `Cargo.toml:1-5` (workspace members) and `Cargo.toml:21-25` (workspace.package subset) are present with no elision marker between them, but the `[workspace.lints.*]` block at lines 6-20 (`missing_docs`/`unreachable_pub`/`rust_2018_idioms` policy) is silently omitted.
- [toml] [minor] 3.11 partial: `Cargo.toml:72-87` (`[package]` header + authors + keywords + description) is present, but the `[dependencies]` block at lines 88-104 (`clap`, `clap_complete`, internal `mdbook-*` crates, `opener`, `toml`, `tracing`) is silently omitted — no ellipsis marker between `rust-version.workspace = true` (line 88) and the next shown range.
- [toml] [minor] 3.12 partial: `Cargo.toml:129-133` (`[features] default=[…]` + `watch`/`serve`/`search` feature definitions) is present, but the optional-deps block at lines 105-128 (`notify`, `notify-debouncer-mini`, `ignore`, `pathdiff`, `walkdir`, `axum`, `futures-util`, `tokio`, `tower-http`, `[dev-dependencies]`) is silently omitted.
- [toml] [minor] Sub-crate Cargo.toml fragments split inconsistently with no named NS batch: `crates/mdbook-driver/Cargo.toml` shows `:1-8` then jumps to `:31-32`; `crates/mdbook-html/Cargo.toml` shows `:1-8` then jumps to `:36-37`. Silent interior elision between `[package]` and `[features]` (dependencies block omitted without marker).
- [markdown] [minor] 6.1 overshoots its declared `README.md:1-13` range: snapshot includes `README.md:1-20` (through `[LICENSE]: …`) with no ellipsis. Lines 14-20 are the License section + link-reference footer — still all in the README header area, but beyond the ranked batch boundary.

### Honesty
- (none)
