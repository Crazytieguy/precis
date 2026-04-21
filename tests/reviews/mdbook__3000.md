---
snapshot_hash: 24040c251372b6cf3dd5565523a026856c13a084bed27935496d47206dd1063e
---

## Summary

At 3000 tokens the snapshot is dominated by a broad directory-tree walk plus tiny header-only excerpts of every crate's `Cargo.toml` and `README.md`. Tier-1 ranked content is almost entirely absent: only 1.1 (root listing), 1.3 (workspace members), 1.5 (`src/cmd/` listing), and parts of 1.4 (per-crate descriptions, scattered inline via Cargo.toml excerpts) land. High-value surfaces 1.2, 1.6, 1.7 (partial), 1.8, 1.9-1.13, and 1.14 are all missing, while the snapshot spends tokens on below-the-fold plumbing (per-crate READMEs with badge lines, `Clean` struct body + `human_readable_bytes`, `mdbook-driver`/`mdbook-html` `[features]` stanzas, `mdbook-compare/src/main.rs` header, hidden dotfiles `.cargo/`/`.github/`/`.gitignore`). The dominant pattern is breadth-over-depth: the walker enumerates paths cheaply but never spends budget on the high-value code excerpts the North Star ranks first.

## Divergences

### Ranking
- [markdown] [major] NS 1.2 (`README.md:7` one-liner) missing; snapshot instead includes `README.md:1,3-5` (title + three CI/crates.io/license badge lines) which map to NS 6.1 (README:1-13 intro+badges) — a tier-1 → tier-6 inversion.
- [markdown] [major] NS 1.2 also displaced by eight `crates/*/README.md` title+badge stanzas (`mdbook-compare`, `mdbook-core`, `mdbook-driver`, `mdbook-html`, `mdbook-markdown`, `mdbook-preprocessor`, `mdbook-renderer`, `mdbook-summary`, `xtask/README.md`), all routed by NS to §Plumbing below-the-fold (~600 toks combined).
- [rust] [major] NS 1.6 (subcommand → about-line map) missing; snapshot includes `src/cmd/clean.rs:38-50` (`human_readable_bytes` signature + `Clean` struct fields) which NS routes to "CLI internals" below-the-fold (`src/cmd/clean.rs` ~896 toks).
- [rust] [major] NS 1.8 (`src/main.rs:18-55` clap dispatch) missing; only the `//! The mdbook CLI.` file-doc line appears. Displaced by below-the-fold per-crate READMEs and `mdbook-compare/src/main.rs:1-4` header (NS §Tooling crates).
- [rust] [major] NS 1.9 (`MDBook` struct fields) missing; `crates/mdbook-driver/src/` directory is not expanded at all, while budget is spent on `crates/mdbook-driver/Cargo.toml:31-32` `[features]` and per-crate README badges (both below-the-fold).
- [rust] [major] NS 1.10 (`Preprocessor` trait) missing; `crates/mdbook-preprocessor/src/` is not expanded. Same below-the-fold displacement.
- [rust] [major] NS 1.11 (`Renderer` trait) missing; `crates/mdbook-renderer/src/` is not expanded. Same below-the-fold displacement.
- [rust] [major] NS 1.12 (`LinkPreprocessor` `{{# ... }}` helper list) missing; displaced by below-the-fold content.
- [rust] [major] NS 1.13 (`IndexPreprocessor` doc) missing; displaced by below-the-fold content.
- [markdown] [major] NS 1.14 (`guide/src/SUMMARY.md` full file) missing; only the filename appears in the tree. Displaced by `guide/guide-helper/Cargo.toml:1-7` `[package]` header (NS §Guide below-the-fold) and per-crate READMEs.
- [other-language] [major] Snapshot emits hidden-dotfile subtrees (`.cargo/config.toml`, `.github/ISSUE_TEMPLATE/*.yml`, `.github/renovate.json5`, `.github/workflows/*.yml`, `.gitignore`) that NS routes to §Plumbing; these displace tier-1 batches 1.8-1.13.

### Batch correctness
- [rust] [minor] NS 1.7 partially included: `Cargo.toml:1-5` (`[workspace] members`) and `Cargo.toml:21-25` (`[workspace.package]`) present, but `Cargo.toml:8-19` (`[workspace.lints.clippy]` + `[workspace.lints.rust]` — `missing_docs`/`unreachable_pub`/`rust_2018_idioms` policy) silently omitted with no elision marker between the two shown ranges. A reader cannot tell the lints block exists.
- [rust] [minor] NS 1.4 partially included: descriptions for 7 of 9 crates appear (embedded in each `Cargo.toml:4` line), but `mdbook-compare` and `xtask` appear with no description line. NS 1.4 explicitly covers both via derived one-liners; those derived lines are absent, so a reader assumes these two legitimately have nothing to say rather than noticing silent elision.
- [rust] [minor] NS 3.1 partial inclusion below its batch threshold: `make_subcommand`/`execute` function-signature lines appear for `build.rs`, `clean.rs`, `init.rs`, `serve.rs`, `test.rs` with `…` between them, but the clap-body contents (the actual `.arg(...)`/`.about(...)` calls — the load-bearing half of 3.1) are fully elided. The signatures alone give no flag information; 3.1 should not be reached at this budget, and reaching it only as skeletons is a silent partial.
- [rust] [minor] Sub-crate Cargo.toml fragments split inconsistently: `crates/mdbook-driver/Cargo.toml` shows `:1-8` then `:31-32`; `crates/mdbook-html/Cargo.toml` shows `:1-8` then `:36-37`. Not a named NS batch, but the same silent-interior-elision pattern between the two ranges (no marker between `[package]` and `[features]`).

### Honesty
- (none)
