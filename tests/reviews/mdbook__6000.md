---
snapshot_hash: c81692cd57044b7c4aab9e5db07e1a8a9129f705e4311450c2d7658692d1450c
---

## Summary

The snapshot is a broad, skeletal sweep of the whole repo (full directory tree plus top-of-file doc comments and item headers with mostly-elided bodies) rather than a North-Star-ordered prefix. High-value tier-1 batches (1.6 subcommand about-lines, 1.8 CLI dispatch, 1.9 `MDBook` fields, 1.10 `Preprocessor` trait body, 1.12 `LinkPreprocessor` doc, 1.13 `IndexPreprocessor` doc, 1.14 Guide SUMMARY) are absent or reduced to bare signatures, while budget is consumed by below-the-fold plumbing (xtask/main.rs, xtask/changelog.rs, guide/guide-helper/src/lib.rs, cmd/clean.rs Clean struct, cmd/watch.rs WatcherKind) and all nine crates' `[package]` headers + README badges. The dominant pattern is cross-cutting partial inclusion: nearly every batch that appears is a signature-only teaser, which is itself a batch-correctness problem. Several included fragments violate declared predecessor edges.

## Divergences

### Ranking

- [rust] [major] Below-the-fold `src/cmd/clean.rs` body (line 38 `human_readable_bytes` signature, lines 46-50 `Clean` struct) is present while tier-1 1.6, 1.8, 1.9, 1.10, 1.12, 1.13, 1.14 are missing.
- [rust] [major] Below-the-fold `src/cmd/watch.rs` body (`WatcherKind` enum + `rebuild_on_change` signature) is present while the same tier-1 batches above are missing.
- [rust] [major] Below-the-fold `crates/xtask/src/main.rs` + `changelog.rs` appear in the tree with file heads while tier-1 1.6/1.8/1.9/1.10/1.12/1.13/1.14 are missing.
- [rust] [major] Below-the-fold `guide/guide-helper/` Cargo.toml is fully expanded while tier-1 1.6/1.8/1.9/1.10/1.12/1.13/1.14 are missing.
- [rust] [major] 3.5 `MarkdownOptions` struct (with doc comments, lines 15-31) is shown while tier-2 2.3 Book, 2.4 BookItem, 2.5 Chapter, 2.6 Config, 2.7 BookConfig, 2.8 BuildConfig, 2.14 MDBook::load, 2.19 builtin_preprocessors/mod.rs are missing.
- [markdown] [major] 5.11 `ci/` + `.github/workflows/` listings are present while tier-1 1.6/1.8/1.9/1.10/1.12/1.13/1.14 are missing.
- [rust] [minor] 1.11 `Renderer` trait body is shown in full while 1.10 `Preprocessor` trait body (ranked immediately before it) is reduced to a `pub trait Preprocessor {` header with body elided.
- [rust] [minor] 2.4 `BookItem` enum (lines 119-126) is shown while 2.3 `Book` struct's module-doc preamble (lines 12-25) and 2.5 `Chapter` struct body (134-170) are both reduced to bare headers.
- [rust] [minor] 2.13 `RenderContext` struct header is shown while 2.12 `PreprocessorContext` body and `parse_input` are likewise reduced to headers — but even 2.12's `PreprocessorContext::new` signature appears, while 2.3-2.11 batches of higher rank within tier 2 are missing.
- [rust] [predecessor] `PreprocessorContext::new` signature appears in the snapshot (partial 2.12) while its declared predecessor 1.10 (`Preprocessor` trait body) is missing beyond the `pub trait Preprocessor {` header.
- [rust] [predecessor] `load_book` / `load_book_from_disk` signatures in `crates/mdbook-driver/src/load.rs` (partial 4.8) appear without their declared predecessor 2.14 (`MDBook::load` + `load_with_config` + `iter`); 2.14 is absent beyond the `MDBook` struct header.
- [rust] [predecessor] `BookBuilder` struct in `crates/mdbook-driver/src/init.rs` (partial 4.10) appears without its declared predecessor 2.15 (`MDBook::init` BookBuilder doc with directory layout); 2.15 is absent.

### Batch correctness

- [rust] [minor] 1.4 — Per-crate descriptions are visible via inline `[package]` `description = "..."` lines for 7/9 crates (mdbook-core, mdbook-driver, mdbook-html, mdbook-markdown, mdbook-preprocessor, mdbook-renderer, mdbook-summary), but `mdbook-compare` and `xtask` have no description in their Cargo.toml and no derived one-liner is supplied. The batch explicitly calls out those two as needing derived descriptions.
- [toml] [minor] 1.7 — Workspace `Cargo.toml` header shown as lines 1-5 (members) and 21-25 (workspace.package) with the `[workspace.lints.*]` block (lines ~6-20, covering `missing_docs`, `unreachable_pub`, `rust_2018_idioms`) silently omitted between them. Batch scope is `Cargo.toml:1-26` as a single contiguous block.
- [rust] [minor] 2.2 — `crates/mdbook-core/src/lib.rs` shown as lines 1, 3-7, 9-11, 14 with `…` elisions; the `pub mod errors { ... }` body with its anyhow re-export is not expanded. Batch scope is "whole file".
- [rust] [minor] 2.3 — `Book` struct at 26-29 is shown but the module-doc preamble (lines 12-25, explicitly named in the batch as "module doc") is absent.
- [rust] [minor] 2.4 — `BookItem` enum at 119-126 is shown; the batch range is 113-127 and the leading doc comments at 113-118 are missing.
- [rust] [minor] 2.12 — Only `PreprocessorContext::new` signature (line 70) and crate-doc lines 1-6 are shown; the remainder of the crate doc (7-22), `PreprocessorContext` struct fields, and `parse_input` are absent from what should be a single coherent batch (`lib.rs:1-22` + `48-84`).
- [rust] [minor] 2.13 — Only `Renderer` trait (which is 1.11 content) plus a bare `pub struct RenderContext {` header appear; the crate doc (1-22), struct fields, `new`/`source_dir`/`from_json` bodies specified by the batch are all absent.
- [rust] [minor] 2.20 — `mdbook-html/src/lib.rs` is partially included (missing blank line 2, but otherwise present), but the paired `html/mod.rs:1-32` pipeline-doc half of the batch is entirely absent — `html/` appears only as a directory name.
- [generic] [minor] 1.5 — `src/cmd/` listing shows `watch/` as a bare directory; the batch explicitly lists `watch/native.rs` and `watch/poller.rs` as required listing contents.
- [rust] [minor] 3.1 — `make_subcommand()` and `execute()` appear as signature-only teasers for build, clean, init, test, watch; the clap `.about(...)` / flag definitions (the point of the batch) are not shown.
- [rust] [minor] 3.2 — `make_subcommand` for serve at line 23 shows only the signature; the serve subcommand body including `LIVE_RELOAD_ENDPOINT = "__livereload"` constant and `--hostname`/`--port`/`arg_open`/`arg_watcher` flag definitions (lines 1-50) are absent.
- [rust] [minor] 3.3 — `command_prelude.rs` appears only with bare `CommandExt` trait header (line 7) and `set_dest_dir` signature (line 62); the `arg_dest_dir`/`arg_root_dir`/`arg_open`/`arg_watcher` helpers and trait body (batch scope "whole file") are not excerpted.
- [rust] [minor] 3.5 — `mdbook-markdown/src/lib.rs` shows crate doc, `pub use pulldown_cmark`, and `MarkdownOptions` struct in full, but the `impl Default for MarkdownOptions` body and `new_cmark_parser` body are elided. Batch scope is the whole file.
- [toml] [minor] 3.11 — `Cargo.toml:72-88` `[package]` block is shown, but the `[dependencies]` section (89-104, the load-bearing portion naming clap/internal crates/opener/toml/tracing) is silently omitted.
- [toml] [minor] 3.12 — Only lines 129-133 `[features]` block is included; the optional-deps listing (105-128, covering notify/axum/tokio etc.) is silently omitted.
- [generic] [minor] 5.12 — Top-level `front-end/` subdirs (`css/`, `fonts/`, `images/`, `js/`, `playground_editor/`, `searcher/`, `templates/`) are shown, but the batch requires listing the files inside `templates/` and `css/` too; those are absent.

### Honesty

(none)
