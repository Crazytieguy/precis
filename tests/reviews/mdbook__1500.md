---
snapshot_hash: f15376367131fefb1d7fbcb6829cba3695be9bd39a2733141acd99dbf6ee3c34
---

## Summary

At a 1500-token budget the snapshot is far from the North Star ordering. Batches 1.1 and 1.2 land, but essentially every ranked item from 1.5 through 3.10 is skipped while the snapshot instead spends budget on lower-ranked material: the mdbook-binary `[package]` stanza (3.11), the `[features]` stanza (3.12), the CI folder listing (5.11), and several below-the-fold surfaces (the `[[bin]]/[[example]]/[[test]]` tail of `Cargo.toml`, the full README, and plumbing Cargo.toml/README fragments under `crates/mdbook-{compare,core,driver}/`). The dominant pattern is "first-pass walker dumps sibling files around a root while skipping the ranked source-tree content entirely."

## Divergences

### Ranking
- [rust] [major] 1.5 (src/ + src/cmd/ listing) skipped — only `src/cmd/` folder name is shown; contents absent. Snapshot instead includes lower-ranked Cargo.toml slices mapped to 3.11/3.12 (`Cargo.toml:72-88`, `:129-133`) and below-the-fold `Cargo.toml:135-153`.
- [rust] [major] 1.6 (subcommand about-line map) skipped while 3.11/3.12 fragments and below-the-fold `Cargo.toml:135-153` are present.
- [rust] [major] 1.8 (`src/main.rs:18-55` CLI dispatch) skipped; only the line-1 doc comment is shown. Displaced by 3.11/3.12 fragments and the `ci/` listing (5.11).
- [rust] [major] 1.9 (`MDBook` struct fields) skipped while 3.11/3.12 fragments and below-the-fold plumbing (sub-crate Cargo.toml/README fragments under `crates/mdbook-{compare,core,driver}/`) are present.
- [rust] [major] 1.10 (`Preprocessor` trait) skipped while 3.11/3.12 and below-the-fold items are present.
- [rust] [major] 1.11 (`Renderer` trait) skipped while 3.11/3.12 and below-the-fold items are present.
- [rust] [major] 1.12 (`LinkPreprocessor` helper doc) skipped while lower-ranked material is present.
- [markdown] [major] 1.13 (`IndexPreprocessor` doc) skipped while lower-ranked material is present.
- [markdown] [major] 1.14 (guide `SUMMARY.md`) skipped (only filename appears as a collapsed node); displaced by 5.11 and below-the-fold tail README content.
- [rust] [minor] 2.1-2.22 (core data model batches) entirely skipped while 3.11/3.12 fragments are present (within major group 3, so minor relative to tier 3).
- [rust] [major] 2.1-2.22 skipped while below-the-fold material (sub-crate Cargo.toml/README plumbing, `Cargo.toml:135-153`) is present.
- [markdown] [major] 6.1 overreach into below-the-fold: `README.md:1-20` is shown, extending beyond 1.2 (`:7`) and 6.1 (`:1-13`) into the License section (`:14-20`) which is below-the-fold, while most of 1.5-2.x remain absent.
- [rust] [major] Below-the-fold `Cargo.toml:135-153` (`[[bin]]`/`[[example]]`/`[[test]]` tail — explicitly listed under "CLI internals" below-the-fold as `Cargo.toml:135-158`) is present while most ranked 1.x and 2.x batches are absent.
- [rust] [major] Below-the-fold plumbing from "Sub-crate `Cargo.toml` files" and "`crates/*/README.md`" — `crates/mdbook-compare/Cargo.toml:1-7`, `crates/mdbook-compare/README.md:1-3`, `crates/mdbook-core/Cargo.toml:1-8`, `crates/mdbook-core/README.md:1-5`, `crates/mdbook-driver/Cargo.toml:31-32`, `crates/mdbook-driver/README.md` — present while 1.4 (crate-name→description map), 1.9-1.14, and most of 2.x/3.x are skipped.

### Batch correctness
- [rust] [minor] 1.7 partial: snapshot shows `Cargo.toml:1-5` and `:21-25` but omits `:6-20` (the `[workspace.lints.*]` block that the batch explicitly includes). Silent partial of the workspace header.
- [rust] [minor] 3.11 partial: snapshot shows `Cargo.toml:72-88` ([package] metadata) but omits `:89-104` ([dependencies]) which the batch explicitly spans (72-104).
- [rust] [minor] 3.12 partial: snapshot shows `Cargo.toml:129-133` ([features]) but omits `:105-128` (optional watch/serve deps + [dev-dependencies]) which the batch explicitly spans (105-134).
- [rust] [minor] 1.4 partial: of the nine-crate name→description map, only `mdbook-core`'s description surfaces (via its included Cargo.toml slice); `mdbook-compare` (has `publish=false`, no description), `mdbook-driver` (only [features] shown), and `mdbook-html/-markdown/-preprocessor/-renderer/-summary/xtask` (folder-only) are not rendered with their descriptions. Silent partial of the batch.

### Honesty
- (none)
