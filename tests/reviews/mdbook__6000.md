---
snapshot_hash: 408b183e0e54a5011c19f760df0dc2699cbeda0aa67e7d2edbca330630daa68f
---

## Summary

The snapshot is produced by a breadth-first tree walker that lists every file/directory and emits only a thin slice of each file (top-of-file doc comments, `pub struct`/`pub fn`/`pub trait` header lines with bodies elided, and `Cargo.toml` metadata stanzas). It spends budget on below-the-fold plumbing — every crate's badge-only `README.md`, every sub-crate `Cargo.toml` `[package]` header, `.github/ISSUE_TEMPLATE/*.yml` leaves, the full `ci/` tree, `mdbook-compare/src/main.rs:1-4`, `xtask/src/main.rs` head, `guide-helper/Cargo.toml` — while dropping or reducing many high-ranked batches (1.6, 1.8, 1.9, 1.12, 1.13, 1.14, 2.5–2.8, 2.14–2.18, 3.13, almost all of sections 4 and 5). The dominant divergence pattern is cross-cutting partial inclusion: nearly every present batch is a signature-only teaser, which is itself a batch-correctness violation.

## Divergences

### Ranking

- [rust] [major] NS 1.6 (subcommand → about-line map, tier 1) missing; `src/cmd/*.rs` shows only `make_subcommand() -> Command` signature lines with `.about(...)` strings elided. Displaced by below-the-fold `src/cmd/clean.rs:38-50` (`human_readable_bytes` + `Clean` struct) and `src/cmd/watch.rs:21-67` (`WatcherKind`, `rebuild_on_change` signature).
- [rust] [major] NS 1.8 (CLI dispatch, `src/main.rs:18-55`, tier 1) missing; only `main.rs:1-14` (imports + `mod cmd`) shown. Displaced by every crate-README badge triplet and full `.github/ISSUE_TEMPLATE/*.yml` listings.
- [rust] [major] NS 1.9 (`MDBook` struct fields, tier 1) missing — only bare `pub struct MDBook {` header at line 29 emitted with all fields elided. Displaced by `mdbook-markdown/src/lib.rs` `MarkdownOptions` full struct with field docs (3.5, tier 3) and every sub-crate `Cargo.toml` header.
- [rust] [major] NS 1.12 (`LinkPreprocessor` helpers doc, tier 1) missing; no content from `builtin_preprocessors/links.rs` appears at all. Displaced by tier-5/below-the-fold items (xtask `Cargo.toml`, `mdbook-compare/Cargo.toml` + `README.md`, `guide-helper/Cargo.toml`).
- [rust] [major] NS 1.13 (`IndexPreprocessor` doc, tier 1) missing; no content from `builtin_preprocessors/index.rs` appears.
- [markdown] [major] NS 1.14 (Guide `SUMMARY.md` full, tier 1) missing — file listed by name only. Displaced by below-the-fold listings of `.github/`, `ci/`, all `front-end/` subdirectories, and the empty `tests/gui/`, `tests/testsuite/` directory nodes.
- [rust] [major] NS 2.5 (`Chapter` struct with field docs, tier 2) missing — only bare `pub struct Chapter {` at line 141 shown, with every field and the load-bearing `path`/`source_path` README-vs-`index.md` doc-comments elided. Displaced by tier-5 crate-README badge boilerplate (7 identical blocks).
- [rust] [major] NS 2.6 (`Config` struct, tier 2) missing entirely — `config.rs` appears only as a filename under `mdbook-core/src/`.
- [rust] [major] NS 2.7 (`BookConfig` + `TextDirection`, tier 2) missing.
- [rust] [major] NS 2.8 (`BuildConfig` / `RustConfig` / `RustEdition`, tier 2) missing.
- [rust] [major] NS 2.14 (`MDBook::load` + `load_with_config` + `iter`, tier 2) missing — `mdbook.rs` shows only the `MDBook` struct header line.
- [rust] [major] NS 2.15 (`MDBook::init` / `BookBuilder` doc with dir layout, tier 2) missing.
- [rust] [major] NS 2.16 (`with_renderer` / `with_preprocessor`, tier 2) missing.
- [rust] [major] NS 2.17 (`MDBook::test` / `test_chapter`, tier 2) missing.
- [rust] [major] NS 2.18 (`build_dir_for` / `source_dir` / `theme_dir`, tier 2) missing.
- [rust] [minor] NS 2.19 (builtin_preprocessors/mod.rs names, tier 2) missing — directory listed without its mod.rs file.
- [rust] [major] NS 2.20 (`mdbook-html` crate root + html pipeline doc, tier 2) partially missing — `lib.rs` shown but `html/mod.rs:1-32` pipeline doc (the load-bearing half) absent; `html/` appears as a bare directory.
- [generic] [minor] NS 2.21 (recursive listings under `mdbook-html/src/`, tier 2) missing — `html/`, `html_handlebars/`, `theme/` listed by directory name only, without file contents.
- [rust] [major] NS 2.22 (builtin_renderers/mod.rs + `CmdRenderer` intro, tier 2) missing — directory listed without any files.
- [rust] [major] NS 3.4 (`MDBook::build` + `preprocess_book` + `execute_build_process`, tier 3) missing.
- [rust] [major] NS 3.6, 3.7 (`LinkPreprocessor::run` head + `LinkType`/`RangeOrAnchor` enums, tier 3) missing.
- [rust] [major] NS 3.8 (`IndexPreprocessor` full, tier 3) missing.
- [rust] [major] NS 3.9 (`CmdRenderer` full, tier 3) missing.
- [rust] [major] NS 3.10 (`CmdPreprocessor` decl + `write_input`, tier 3) missing.
- [rust] [major] NS 3.13 (`HtmlConfig` every field, tier 3) missing — largest single config-surface omission.
- [rust] [major] NS 3.14, 3.15, 3.16, 3.17 (theme_dir/get_404_output_file, Theme + include_bytes!, `Config::from_disk` + env, `mdbook-core::config` module doc, tier 3) missing.
- [rust] [major] NS 4.1–4.14 (entire tier-4 band of `execute` bodies, `determine_renderers`/`determine_preprocessors`, `load_book` flow, `Book` impl methods, etc.) all missing; `src/cmd/*.rs` emits only `execute` signature lines with bodies elided.
- [markdown] [major] NS 5.3 (CONTRIBUTING.md xtask commands, tier 5) missing — `CONTRIBUTING.md` listed as bare filename.
- [generic] [minor] NS 5.4, 5.5, 5.7 (tests folder + testsuite listing + `main.rs`, tier 5) missing; `tests/testsuite/` is an empty directory node in the snapshot.
- [markdown] [major] NS 5.6 (`tests/testsuite/README.md`, tier 5) missing.
- [rust] [major] NS 5.8 (workspace.dependencies, tier 5) missing — `Cargo.toml:27-71` silently elided.
- [toml] [major] NS 5.9 (`guide/book.toml`, tier 5) missing — file not shown at all, only `guide/` directory walk.
- [markdown] [major] NS 5.10 (guide/src/README.md feature bullets, tier 5) missing.
- [generic] [minor] NS 5.12 (`front-end/` directory listings, tier 5) partially missing — top-level subdirs listed, but `templates/`, `css/` file contents absent.
- [markdown] [minor] NS 6.2 (CHANGELOG.md head, tier 6) missing; `CHANGELOG.md` shown as bare filename.
- [rust] [predecessor] `load_book` / `load_book_from_disk` signatures from `crates/mdbook-driver/src/load.rs` (partial NS 4.8) appear without their declared predecessor NS 2.14 (`MDBook::load`), which is absent beyond the struct header.
- [rust] [predecessor] `BookBuilder` struct in `crates/mdbook-driver/src/init.rs` (partial NS 4.10) appears without its declared predecessor NS 2.15 (`MDBook::init` BookBuilder doc with directory layout), which is absent.

### Batch correctness

- [rust] [minor] NS 1.4 — Per-crate descriptions are visible via inline `[package]` `description = "..."` lines for 7/9 crates, but `mdbook-compare` and `xtask` have no description and no derived one-liner is supplied. The batch explicitly calls out those two as needing derived descriptions.
- [toml] [minor] NS 1.7 — `Cargo.toml` shown as lines 1-5 (members) and 21-25 (workspace.package) with `[workspace.lints.*]` (lines ~6-20, declaring `missing_docs`/`unreachable_pub`/`rust_2018_idioms` warn levels) silently omitted. Batch scope is `Cargo.toml:1-26` as one contiguous block.
- [rust] [minor] NS 2.2 — `mdbook-core/src/lib.rs` shown as lines 1, 3-7, 9-11, 14 with `…` elisions; the `pub mod errors { ... }` body with its anyhow re-export is not expanded. Batch scope is "whole file".
- [rust] [minor] NS 2.3 — `Book` struct fields at 26-29 shown, but the module-doc preamble (lines 12-25, explicitly named in the batch) is absent.
- [rust] [minor] NS 2.4 — `BookItem` enum body at 119-126 shown; the batch range is 113-127 and the leading `/// Enum representing...` doc comment at 113-118 along with the `#[allow(...)]` attribute block is missing.
- [rust] [minor] NS 2.9 — Only header lines for `Summary` (line 67), `Link` (line 84), `SummaryItem` (line 122) are shown with all bodies elided to `…`. Required fields (`title`, `prefix_chapters`, `numbered_chapters`, `suffix_chapters`; `name`, `location`, `number`, `nested_items`; enum variants) are entirely absent.
- [rust] [minor] NS 2.10 — Only `parse_summary` signature at line 59 appears; crate doc `:1-58` with the prefix/part/numbered/suffix chapter explanation is absent.
- [rust] [minor] NS 2.12 — Only `PreprocessorContext::new` signature (line 70), crate-doc lines 1-6, and the `PreprocessorContext` struct (lines 51-66) are present. `parse_input` appears only as a signature at line 82 with body elided; the remainder of the crate doc (7-22) is also absent.
- [rust] [minor] NS 2.13 — `RenderContext` appears only as bare `pub struct RenderContext {` header at line 40 with all fields and the `new`/`source_dir`/`from_json` methods collapsed to `…`. Required fields (`version`, `root`, `book`, `config`, `destination`, `chapter_titles`) are absent.
- [rust] [minor] NS 3.1 — `make_subcommand()` and `execute()` appear as signature-only teasers for build, clean, init, test, watch; the clap `.about(...)` / flag definitions (the point of the batch) are elided.
- [rust] [minor] NS 3.2 — `make_subcommand` for serve at line 23 shows only the signature; the `LIVE_RELOAD_ENDPOINT = "__livereload"` constant and `--hostname`/`--port`/`arg_open`/`arg_watcher` flag definitions are absent.
- [rust] [minor] NS 3.3 — `command_prelude.rs` shows only bare `CommandExt` trait header (line 7) and `set_dest_dir` signature (line 62); `arg_dest_dir`/`arg_root_dir`/`arg_open`/`arg_watcher` helpers and trait body absent. Batch scope is "whole file".
- [rust] [minor] NS 3.5 — `mdbook-markdown/src/lib.rs` shows crate doc, `pub use pulldown_cmark`, and `MarkdownOptions` struct in full, but `impl Default for MarkdownOptions` body and `new_cmark_parser` body (which contain the TABLES/FOOTNOTES/STRIKETHROUGH/TASKLISTS/HEADING_ATTRIBUTES option flags — the load-bearing portion) are elided.
- [toml] [minor] NS 3.11 — `Cargo.toml:72-88` `[package]` block shown, but `[dependencies]` (89-104, naming clap/internal `mdbook-*`/opener/toml/tracing) silently omitted.
- [toml] [minor] NS 3.12 — Only lines 129-133 `[features]` block included; optional-deps listing (105-128, covering notify/axum/tokio/tower-http etc.) and `[dev-dependencies]` silently omitted.
- [generic] [minor] NS 1.5 — `src/cmd/` listing shows `watch/` as a bare directory; the batch explicitly requires `watch/native.rs` and `watch/poller.rs` as listed contents.

### Honesty

- [toml] Un-marked elision between `Cargo.toml` line ranges: snapshot jumps from line 5 directly to line 21, from 88 to 129, and from 133 to 135 with no ellipsis marker or `…` between them. Spot-check against `tests/fixtures/mdbook/Cargo.toml` confirms lines 6-20 (`[workspace.lints.*]`), 89-128 (`[dependencies]` + optional deps), and 134 (blank before `[[bin]]`) exist and contain meaningful content. A reader relying on line numbers can infer the gap, but no explicit truncation marker is present.
- [rust] Un-marked elision in `mdbook-core/src/book.rs`: snapshot shows lines 26-29 (`Book`) immediately followed by 119-126 (`BookItem`) with no `…` between them; the ~90 intervening lines (impl Book block etc.) are silently dropped. Same pattern recurs throughout the rendered Rust files.
