---
snapshot_hash: f7d56c0e0fceb78a397d3d99118c39ec20b41d21e1d4eb45e50294efa2acbcfc
---

## Summary

At the 6000-token budget the snapshot is far from the North Star. It captures much of the tier-1 directory/listing orientation (1.1, 1.3, 1.5) and lands a handful of structural batches (1.11 Renderer trait, 2.9 Summary/Link/SummaryItem, 2.13 RenderContext, 3.5 MarkdownOptions), but it skips most tier-1 content anchors (1.2, 1.6, 1.8, 1.9, 1.10, 1.12, 1.13, 1.14) and nearly all tier-2 data-model batches (2.1, 2.3–2.8, 2.11, 2.14–2.22). Meanwhile it spends budget on a scattering of below-the-fold content: `cmd::clean` internals (Clean struct + human_readable_bytes), `xtask/src/main.rs`, `xtask/src/changelog.rs`, `guide/guide-helper/src/lib.rs`, and `mdbook-html/utils.rs` signatures. The dominant pattern is "low-rank skeletons displacing high-rank content batches" — many batches appear as teaser signatures or partial excerpts rather than the coherent ranges the North Star specifies.

## Divergences

### Ranking
- [rust] [major] Below-the-fold `src/cmd/clean.rs` body (Clean struct, human_readable_bytes, impl Display) is included while 1.2 (README one-liner), 1.6 (subcommand about-line map), 1.8 (main.rs dispatch), 1.9 (MDBook struct), 1.10 (Preprocessor trait), 1.12 (LinkPreprocessor doc), 1.13 (IndexPreprocessor doc), 1.14 (guide SUMMARY.md) are all missing.
- [rust] [major] Below-the-fold `xtask/src/main.rs` and `xtask/src/changelog.rs` excerpts are included while tier-1 items above are missing (same set).
- [rust] [major] Below-the-fold `guide/guide-helper/src/lib.rs` (full signatures including GuideHelper impl) is included while tier-1 1.2/1.6/1.8/1.9/1.10/1.12/1.13/1.14 are missing.
- [rust] [major] Below-the-fold `mdbook-html/src/utils.rs` signatures (normalize_path, ToUrlPath, unique_id, id_from_content) are included while the same tier-1 batches are missing.
- [rust] [major] `cmd/watch.rs` body (WatcherKind enum + rebuild_on_change signature) is included while tier-1 1.2/1.6/1.8/1.9/1.10/1.12/1.13/1.14 are missing. Note: 1.5 indexes watch.rs structurally; the body here is below-the-fold CLI internals.
- [rust] [major] 3.5 `MarkdownOptions` (full) is shown while tier-2 prerequisites in the build pipeline chain are skipped: 2.3 Book, 2.4 BookItem, 2.5 Chapter, 2.6 Config, 2.14 MDBook::load, 2.19 builtin_preprocessors mod, 2.20 mdbook-html crate root. (Not a predecessor edge in the NS, but a major-group ranking inversion.)
- [rust] [major] 2.13 `RenderContext` (full) is shown while 2.3 Book, 2.4 BookItem, 2.5 Chapter, 2.6 Config are missing. Displaces multiple lower-ranked-but-present 2.x items against higher-ranked 2.3/2.4/2.5/2.6.
- [rust] [minor] 1.11 Renderer trait is shown while 1.10 Preprocessor trait is missing (1.10 ranked immediately before 1.11).
- [rust] [minor] 2.9 Summary/Link/SummaryItem is shown while 2.3 Book, 2.4 BookItem, 2.5 Chapter, 2.6 Config, 2.7 BookConfig, 2.8 BuildConfig are missing (all within tier 2).
- [rust] [minor] 2.13 RenderContext is shown while 2.12 PreprocessorContext is only partially present (only `PreprocessorContext::new` signature visible, not the struct or `parse_input`).

### Batch correctness
- [markdown] [minor] 1.2 — The snapshot shows README.md lines 1, 3–5 (title + three badges) but omits line 7, which is the entire point of batch 1.2. Silent partial inclusion of README head without the one-liner.
- [rust] [minor] 1.4 — Crate-name to description map is partial: Cargo.toml `description` lines are shown for mdbook-core, mdbook-driver, mdbook-html, mdbook-markdown, mdbook-preprocessor, mdbook-renderer, mdbook-summary, but mdbook-compare and xtask lack description fields and no derived one-liner is substituted (their Cargo.toml excerpts omit any description entirely). Silent partial inclusion of the intended map.
- [rust] [minor] 1.7 — Workspace Cargo.toml header shown as lines 1–6 (members) and 21–25 (workspace.package) with the `[workspace.lints.*]` block between them silently omitted. Batch 1.7 specifies `Cargo.toml:1-26` as a single contiguous block.
- [rust] [minor] 2.2 — mdbook-core/src/lib.rs shown with lines 1, 3–7, 9–11, 14 only; the `pub mod errors { ... }` body and anyhow re-export are omitted with no elision marker in the visible content.
- [rust] [minor] 2.10 — Only the crate doc head (lines 1–5) and the `parse_summary` signature line (59) are shown, omitting lines 6–58 (the bulk of the crate doc explaining title / prefix / part / numbered / suffix chapter semantics).
- [rust] [minor] 2.12 — Only `PreprocessorContext::new` signature (line 70) is present; the struct definition with fields (`root`/`config`/`renderer`/`mdbook_version`/`chapter_titles`), the crate-doc body, and `parse_input` are all missing from what should be a single coherent batch.
- [rust] [minor] 3.1 — `make_subcommand()` appears as a signature-only teaser for build, clean, init, test, watch (and execute signatures) with none of the clap arg bodies inside. Batch 3.1 requires the clap-arg content that answers "what flags does each subcommand take?"; signatures alone silently omit that.
- [rust] [minor] 3.2 — `make_subcommand` for serve at line 23 shows only the signature; `LIVE_RELOAD_ENDPOINT` constant and flag definitions in lines 1–50 are absent.
- [rust] [minor] 3.3 — `command_prelude.rs` shown as only `impl CommandExt for Command { fn _arg }` (lines 56–57); the `CommandExt` trait definition, `arg_dest_dir`/`arg_root_dir`/`arg_open`/`arg_watcher` helpers, and `set_dest_dir` are omitted.
- [rust] [minor] 4.10 — BookBuilder struct lines 13–18 shown; builder methods (`new`/`with_config`/`copy_theme`/`create_gitignore` at 19–65) are omitted from the same batch.
- [rust] [minor] 4.8 — `load_book` signature (line 10) and `load_book_from_disk` signature (line 60) are shown; the `create_missing` call path and body through line 55 is silently omitted.
- [rust] [predecessor] 4.10 — BookBuilder struct (partially) present without its declared predecessor 2.15 (`MDBook::init` BookBuilder doc). 2.15 is not in the snapshot.
- [rust] [predecessor] 4.8 — `load_book` signatures present without declared predecessor 2.14 (`MDBook::load` + `load_with_config` + `iter`). 2.14 is not in the snapshot.
- [rust] [predecessor] 2.12 — `PreprocessorContext::new` signature present without declared predecessor 1.10 (`Preprocessor` trait). 1.10 is not in the snapshot.

### Honesty
- (none)
