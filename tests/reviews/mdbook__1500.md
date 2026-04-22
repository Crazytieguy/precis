---
snapshot_hash: f15376367131fefb1d7fbcb6829cba3695be9bd39a2733141acd99dbf6ee3c34
---

## Summary

At 1500 tokens the snapshot covers only a small slice of the North Star's batch 1 orientation layer, and the slice it includes is heavily out of order: chunks of `Cargo.toml` (NS 3.11/3.12) and several below-the-fold per-crate `Cargo.toml`/README fragments are shown while higher-ranked batches 1.4, 1.6, 1.8, and 1.9-1.14 are absent. Most ranked items that are present are also partial, violating batch correctness for 1.1, 1.5, 1.7, 3.11, and 3.12.

## Divergences

### Ranking
- [rust] [major] NS 3.11 shown (`Cargo.toml:72-88`, `[package]` block) while NS 1.6 (subcommand about-line map across `src/cmd/*.rs` + `src/main.rs`) is missing.
- [rust] [major] NS 3.11 shown while NS 1.8 (`src/main.rs:18-55` clap dispatch) is missing — snapshot only shows the `src/main.rs:1` doc line (a below-the-fold fragment).
- [rust] [major] NS 3.12 fragments shown (`Cargo.toml:129-133, 135-153`, features + `[[bin]]`/`[[example]]`/`[[test]]`) while NS 1.6, 1.8, 1.9 (`MDBook` struct), 1.10 (`Preprocessor` trait), 1.11 (`Renderer` trait), 1.12 (link helpers), 1.13 (`IndexPreprocessor`), 1.14 (guide `SUMMARY.md`) are all missing.
- [rust] [major] Snapshot extends past the NS 3.12 range into `Cargo.toml:135-153` (explicitly deferred below-the-fold as "CLI internals" `Cargo.toml:135-158`) while 1.4, 1.6, 1.8-1.14, and all of 2.x are missing.
- [generic] [major] Per-crate plumbing (`crates/mdbook-compare/Cargo.toml:1-7`, `crates/mdbook-compare/README.md`, `crates/mdbook-core/Cargo.toml:1-8`, `crates/mdbook-core/README.md`, `crates/mdbook-driver/Cargo.toml:31-32`, `crates/mdbook-driver/README.md`) is shown while NS 1.4 (the one-line crate-description map that would cover all nine crates in 112 tokens) is missing.
- [markdown] [major] NS 6.1 tail (`README.md:14-20`, License section) present while most of 1.4-1.14 and all of 2.x are missing. (6.1 itself is batch `:1-13`; snapshot overruns into the below-the-fold tail.)
- [rust] [minor] NS 3.12 material shown while NS 3.11 itself is not complete (missing `:89-104` dependency list), creating intra-tier-3 disorder.

### Batch correctness
- [generic] [minor] NS 1.1 violated: top-level listing includes `.cargo/` (with `config.toml`), `.github/` (with full `ISSUE_TEMPLATE/`, `renovate.json5`, `workflows/` subtree), and `.gitignore`, which the North Star batch explicitly excludes. The batch specifies the filtered set `CHANGELOG.md … triagebot.toml`.
- [rust] [minor] NS 1.5 violated: `src/` listing shown (`main.rs`, `cmd/`) but the paired `src/cmd/` listing (`build.rs`, `clean.rs`, `command_prelude.rs`, `init.rs`, `mod.rs`, `serve.rs`, `test.rs`, `watch.rs`, `watch/native.rs`, `watch/poller.rs`) is silently omitted; batch specifies two listings together-or-not.
- [rust] [minor] NS 1.7 violated: batch is `Cargo.toml:1-26`; snapshot shows only `:1-5` and `:21-25`, silently dropping `:6-20` (the `[workspace.lints.*]` block) and line `:26`.
- [rust] [minor] NS 3.11 violated: batch is `Cargo.toml:72-104`; snapshot shows only `:72-88`, silently dropping `:89-104` (`[dependencies]` — clap, internal `mdbook-*` crates, opener, toml, tracing).
- [rust] [minor] NS 3.12 violated: batch is `Cargo.toml:105-134`; snapshot shows only `:129-133`, silently dropping `:105-128` (optional watch/serve deps, `[dev-dependencies]`). Snapshot also extends past `:134` into deferred `:135-153`.

### Honesty
- (none)
