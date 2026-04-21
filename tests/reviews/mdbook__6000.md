---
snapshot_hash: a89414b8eabc522b054ea0c58a2678c628a9048bef5e6a357745eed534a23edd
---

## Summary

At the 6k budget on this oversized fixture, the snapshot lands tier-1 directory orientation (1.1, 1.3, 1.5) plus a handful of structural batches (1.2 README one-liner, 1.11 Renderer trait, partial 2.9, 2.13 RenderContext in near-full, part of 3.5) while skipping most other tier-1 anchors (1.6, 1.8, 1.9, 1.10, 1.12, 1.13, 1.14) and nearly every tier-2 data-model batch (2.3-2.8, 2.11, 2.14-2.22). Budget is consumed by below-the-fold bodies (`cmd/clean.rs` `Clean`/`human_readable_bytes`, `cmd/watch.rs` `WatcherKind` + `rebuild_on_change`, `xtask/main.rs` + `changelog.rs`, `guide/guide-helper/src/lib.rs`) and each crate's `Cargo.toml` `[package]` boilerplate + README badges. Dominant pattern: below-the-fold plumbing and signature-only teasers displacing coherent tier-1/tier-2 batches. Several included fragments violate declared predecessor edges.

## Divergences

### Ranking
- [rust] [major] Below-the-fold `src/cmd/clean.rs` body (lines 38, 46-50 — `human_readable_bytes` signature + `Clean` struct fields) is present while 1.6 (subcommand about-line map), 1.8 (main.rs dispatch), 1.9 (MDBook struct), 1.10 (Preprocessor trait), 1.12 (LinkPreprocessor doc), 1.13 (IndexPreprocessor doc), 1.14 (guide SUMMARY.md) are all missing.
- [rust] [major] Below-the-fold `src/cmd/watch.rs` body (lines 12, 21-24, 37, 62-67 — `WatcherKind` enum + `rebuild_on_change` signature) is present while the same tier-1 batches above are missing.
- [rust] [major] Below-the-fold `crates/xtask/src/main.rs` and `crates/xtask/src/changelog.rs` excerpts are present while tier-1 1.6/1.8/1.9/1.10/1.12/1.13/1.14 are missing.
- [rust] [major] Below-the-fold `guide/guide-helper/src/lib.rs` (imports, `handle_preprocessing` signature, `impl Preprocessor for GuideHelper` with `name`/`run` signatures) is present while tier-1 1.6/1.8/1.9/1.10/1.12/1.13/1.14 are missing.
- [rust] [major] 2.13 `RenderContext` (full, lines 38-89 in near-entirety) is shown while tier-2 predecessors/siblings are missing: 2.3 Book, 2.4 BookItem, 2.5 Chapter, 2.6 Config, 2.7 BookConfig, 2.8 BuildConfig, 2.14 MDBook::load.
- [rust] [major] 3.5 `MarkdownOptions` (the struct with doc comments) is shown while tier-2 2.3/2.4/2.5/2.6/2.14/2.19/2.20 are all missing.
- [rust] [minor] 1.11 `Renderer` trait is shown while 1.10 `Preprocessor` trait (ranked immediately before it) is missing.
- [rust] [minor] 2.9 `Summary`/`Link`/`SummaryItem` struct definitions are shown while 2.3 Book, 2.4 BookItem, 2.5 Chapter, 2.6 Config, 2.7 BookConfig, 2.8 BuildConfig are missing (all within tier 2, higher-ranked).
- [rust] [minor] 2.13 RenderContext is shown while 2.12 `PreprocessorContext` body and `parse_input` are missing (only `PreprocessorContext::new` signature visible).
- [rust] [predecessor] 4.8 — `load_book` / `load_book_from_disk` signatures present in `crates/mdbook-driver/src/load.rs` without declared predecessor 2.14 (`MDBook::load` + `load_with_config` + `iter`); 2.14 is absent.
- [rust] [predecessor] 4.10 — `BookBuilder` struct fragment present in `crates/mdbook-driver/src/init.rs` without declared predecessor 2.15 (`MDBook::init` BookBuilder doc with directory layout); 2.15 is absent.
- [rust] [predecessor] 2.12 — `PreprocessorContext::new` signature present without declared predecessor 1.10 (`Preprocessor` trait); 1.10 is absent.

### Batch correctness
- [rust] [minor] 1.4 — Per-crate descriptions are visible via inline `[package]` `description = "..."` lines for 7/9 crates (mdbook-core, mdbook-driver, mdbook-html, mdbook-markdown, mdbook-preprocessor, mdbook-renderer, mdbook-summary), but `mdbook-compare` and `xtask` have no description in their Cargo.toml and no derived one-liner is supplied. The batch explicitly calls out those two as needing derived descriptions.
- [toml] [minor] 1.7 — Workspace `Cargo.toml` header shown as lines 1-6 (members) and 21-25 (workspace.package) with the `[workspace.lints.*]` block (lines 7-20, covering `missing_docs`, `unreachable_pub`, `rust_2018_idioms`) silently omitted between them. Batch 1.7 specifies `Cargo.toml:1-26` as a single contiguous block.
- [rust] [minor] 2.2 — `crates/mdbook-core/src/lib.rs` shown with lines 1, 3-7, 9-11, 14 followed by a `…` elision; the `pub mod errors { ... }` body with its anyhow re-export is not expanded. Batch scope is "whole file"; flagging as partial.
- [rust] [minor] 2.9 — `Summary`, `Link`, `SummaryItem` struct definitions shown (lines 67-76, 84-94, 122-129) but `Link::new` and `Default` impl declared within batch 2.9 (`lib.rs:64-145`) are omitted.
- [rust] [minor] 2.10 — Only the crate doc head (lines 1-5) and the `parse_summary` signature line (59) are shown; lines 6-58 of the crate doc explaining title / prefix / part / numbered / suffix chapter semantics are absent. Batch scope is `lib.rs:1-62`.
- [rust] [minor] 2.12 — Only `PreprocessorContext::new` signature (line 70) is present; the crate-doc body, struct fields (`root`/`config`/`renderer`/`mdbook_version`/`chapter_titles`), and `parse_input` are all missing from what should be a single coherent batch (`lib.rs:1-22` + `48-84`).
- [generic] [minor] 1.5 — `src/cmd/` listing shows `watch/` as a bare directory; the batch explicitly lists `watch/native.rs` and `watch/poller.rs` as contents that must appear in the listing.
- [rust] [minor] 3.1 — `make_subcommand()` and `execute()` appear as signature-only teasers for build, clean, init, test, watch; the clap `.about(...)` / flag definitions (the point of the batch — "what flags does each subcommand take?") are not shown.
- [rust] [minor] 3.2 — `make_subcommand` for serve at line 23 shows only the signature; the serve subcommand body including `LIVE_RELOAD_ENDPOINT = "__livereload"` constant and `--hostname`/`--port`/`arg_open`/`arg_watcher` flag definitions (lines 1-50) are absent.
- [rust] [minor] 3.3 — `command_prelude.rs` appears only as a filename under the `src/cmd/` listing; the `CommandExt` trait + `arg_dest_dir`/`arg_root_dir`/`arg_open`/`arg_watcher` helpers and `set_dest_dir` (batch scope "whole file") are not excerpted.
- [rust] [minor] 3.5 — `mdbook-markdown/src/lib.rs` shows crate doc, `pub use pulldown_cmark`, and `MarkdownOptions` struct in full, but the `impl Default for MarkdownOptions` body and `new_cmark_parser` body are elided. Batch scope is the whole file.
- [rust] [minor] 4.8 — `load_book` signature (line 10) and `load_book_from_disk` signature (line 60) are shown; the `create_missing` call path and body through line 55 specified by batch 4.8 (`load.rs:1-55`) is silently omitted.
- [rust] [minor] 4.10 — `BookBuilder` struct (lines 13-18) is shown; builder methods `new`/`with_config`/`copy_theme`/`create_gitignore` (batch scope `init.rs:11-65`) are omitted.

### Honesty
- (none)
