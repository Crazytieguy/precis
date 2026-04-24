scores: Sim=0.305 Reached=10/46 Early=1 Late=8 Partial=2 Missing=34 Used=9899/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 6 | 1 | 2 | 0.70 |
| 2 | 9 | 2 | 1 | 6 | 0.32 |
| 3 | 12 | 2 | 0 | 10 | 0.17 |
| 4 | 16 | 0 | 0 | 16 | 0.08 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.1 | 75 | 869 | +794 | 0.80 | late | README lede |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Top-level directory listing |
| 1.3 | 217 | 627 | +410 | 0.86 | late | Workspace members list |
| 1.4 | 265 | 675 | +410 | 1.00 | late | crates/ directory listing |
| 1.5 | 310 | 3090 | +2780 | 1.00 | late | src/ directory listing |
| 1.6 | 412 | — | — | 0.73 | partial | src/cmd module declaration |
| 1.7 | 609 | — | — | 0.00 | missing | CLI subcommand dispatch (main.rs) |
| 1.8 | 813 | — | — | 0.00 | missing | mdbook-driver crate-root doc: sibling-crate enumeration |
| 1.9 | 913 | 4506 | +3593 | 0.90 | late | mdbook-driver lib module declarations |
| 2.1 | 1100 | — | — | 0.71 | partial | MDBook struct fields |
| 2.2 | 1359 | — | — | 0.00 | missing | MDBook::load (entry point) |
| 2.3 | 1562 | — | — | 0.00 | missing | MDBook::load_with_config (state assembly) |
| 2.4 | 1865 | — | — | 0.00 | missing | MDBook::build + preprocess_book |
| 2.5 | 2093 | — | — | 0.00 | missing | MDBook::execute_build_process |
| 2.6 | 2294 | 5573 | +3279 | 0.88 | late | Preprocessor trait + PreprocessorContext |
| 2.7 | 2446 | 6438 | +3992 | 0.85 | late | Renderer trait + RenderContext |
| 2.8 | 2789 | — | — | 0.41 | missing | Book / BookItem / Chapter |
| 2.9 | 2897 | — | — | 0.00 | missing | Book iteration + mutation method names |
| 3.1 | 3155 | — | — | 0.00 | missing | Config top-level struct |
| 3.2 | 3471 | — | — | 0.00 | missing | BookConfig / BuildConfig / RustConfig field names |
| 3.3 | 3777 | — | — | 0.00 | missing | HtmlConfig field names (catastrophic-omission) |
| 3.4 | 4120 | — | — | 0.00 | missing | HtmlConfig sub-tables field-name list |
| 3.5 | 4190 | — | — | 0.00 | missing | Config accessor method names |
| 3.6 | 4666 | 6923 | +2257 | 0.84 | late | Summary / SummaryItem / Link types |
| 3.7 | 4723 | — | — | 0.33 | missing | parse_summary fn + SUMMARY.md format pointer |
| 3.8 | 4889 | — | — | 0.00 | missing | Builtin preprocessor list + module gate |
| 3.9 | 5217 | — | — | 0.00 | missing | Builtin renderer list + CmdRenderer type |
| 3.10 | 5545 | — | — | 0.00 | missing | determine_renderers / default_preprocessors gating |
| 3.11 | 5901 | — | — | 0.00 | missing | HtmlHandlebars + Renderer impl entry |
| 3.12 | 5958 | 7306 | +1348 | 0.92 | aligned | Plugin author examples: nop-preprocessor + remove-emphasis |
| 4.1 | 6259 | — | — | 0.07 | missing | mdbook build subcommand body |
| 4.2 | 6315 | — | — | 0.33 | missing | command_prelude shared-arg fn names |
| 4.3 | 6603 | — | — | 0.00 | missing | LinkPreprocessor identity |
| 4.4 | 7010 | — | — | 0.00 | missing | IndexPreprocessor (README.md → index.md) |
| 4.5 | 7314 | — | — | 0.00 | missing | CmdPreprocessor (shell-out protocol) |
| 4.6 | 7608 | — | — | 0.00 | missing | Preprocessor topological-sort pointer |
| 4.7 | 7818 | — | — | 0.07 | missing | load_book entry + load.rs overview |
| 4.8 | 7937 | — | — | 0.46 | missing | BookBuilder (init flow) signatures |
| 4.9 | 8325 | — | — | 0.00 | missing | HTML rendering pipeline overview |
| 4.10 | 8648 | — | — | 0.00 | missing | Guide docs SUMMARY (user-facing reference) |
| 4.11 | 8763 | — | — | 0.00 | missing | guide book.toml (config exemplar) |
| 4.12 | 9019 | — | — | 0.00 | missing | Testsuite top-level module list |
| 4.13 | 9190 | — | — | 0.00 | missing | BookTest helper API surface |
| 4.14 | 9489 | — | — | 0.03 | missing | tests/ layout: testsuite + gui + README |
| 4.15 | 9774 | — | — | 0.00 | missing | CHANGELOG 0.5 migration lede |
| 4.16 | 9987 | — | — | 0.34 | missing | watch/serve subcommand identity |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 549 | 1.00 | 549 | 9063 | impl method sigs in crates/mdbook-summary/src/lib.rs |
| 274 | 0.79 | 349 | 627 | [package] in Cargo.toml |
| 257 | 1.00 | 257 | 1126 | [features] in Cargo.toml |
| 231 | 1.00 | 231 | 8093 | README headline in guide/src/README.md |
| 184 | 0.64 | 290 | 6438 | pub item at crates/mdbook-renderer/src/lib.rs:40 |
| 176 | 1.00 | 176 | 4957 | pub item at crates/mdbook-markdown/src/lib.rs:15 |
| 156 | 1.00 | 156 | 9219 | mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs |
| 147 | 1.00 | 147 | 4263 | crate-doc lede in crates/mdbook-driver/src/lib.rs |
| 144 | 1.00 | 144 | 8237 | mod/use plumbing in crates/mdbook-renderer/src/lib.rs |
| 137 | 1.00 | 137 | 7134 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 135 | 1.00 | 135 | 9899 | crates/mdbook-core/README.md section #0 |
| 131 | 1.00 | 131 | 9502 | crates/mdbook-markdown/README.md section #0 |
| 131 | 1.00 | 131 | 9633 | crates/mdbook-preprocessor/README.md section #0 |
| 131 | 1.00 | 131 | 9764 | crates/mdbook-summary/README.md section #0 |
| 130 | 1.00 | 130 | 9349 | crates/mdbook-renderer/README.md section #0 |
| 129 | 0.67 | 194 | 869 | README.md section #0 |
| 121 | 1.00 | 121 | 6148 | impl method sigs in crates/mdbook-renderer/src/lib.rs |
| 120 | 1.00 | 120 | 2154 | README headline in crates/mdbook-markdown/README.md |
| 120 | 1.00 | 120 | 2372 | README headline in crates/mdbook-preprocessor/README.md |
| 120 | 1.00 | 120 | 2589 | README headline in crates/mdbook-renderer/README.md |
| 115 | 1.00 | 115 | 1477 | README headline in crates/mdbook-core/README.md |
| 115 | 1.00 | 115 | 1706 | README headline in crates/mdbook-driver/README.md |
| 115 | 1.00 | 115 | 1939 | README headline in crates/mdbook-html/README.md |
| 115 | 1.00 | 115 | 2797 | README headline in crates/mdbook-summary/README.md |
| 115 | 1.00 | 115 | 5731 | pub-item doc at crates/mdbook-preprocessor/src/lib.rs:30 |
| 111 | 1.00 | 111 | 7728 | README headline in guide/src/for_developers/README.md |
| 103 | 1.00 | 103 | 4757 | crate-doc lede in crates/mdbook-markdown/src/lib.rs |
| 101 | 1.00 | 101 | 5169 | crate-doc lede in crates/mdbook-preprocessor/src/lib.rs |
| 98 | 1.00 | 98 | 5935 | crate-doc lede in crates/mdbook-renderer/src/lib.rs |
| 96 | 1.00 | 96 | 3677 | mod/use plumbing in src/main.rs |
| 95 | 0.50 | 191 | 5573 | pub item at crates/mdbook-preprocessor/src/lib.rs:51 |
| 92 | 1.00 | 92 | 6027 | pub-item doc at crates/mdbook-renderer/src/lib.rs:28 |
| 89 | 1.00 | 89 | 1362 | [package] in crates/mdbook-core/Cargo.toml |
| 89 | 0.50 | 178 | 5382 | pub item at crates/mdbook-preprocessor/src/lib.rs:30 |
| 86 | 1.00 | 86 | 2252 | [package] in crates/mdbook-preprocessor/Cargo.toml |
| 85 | 1.00 | 85 | 2469 | [package] in crates/mdbook-renderer/Cargo.toml |
| 84 | 1.00 | 84 | 8321 | pub-item names surface in crates/mdbook-html/src/utils.rs |
| 83 | 0.75 | 111 | 190 | README headline in README.md |
| 83 | 1.00 | 83 | 1572 | [package] in crates/mdbook-driver/Cargo.toml |
| 83 | 1.00 | 83 | 2034 | [package] in crates/mdbook-markdown/Cargo.toml |
| 81 | 1.00 | 81 | 2682 | [package] in crates/mdbook-summary/Cargo.toml |
| 80 | 1.00 | 80 | 1802 | [package] in crates/mdbook-html/Cargo.toml |
| 74 | 1.00 | 74 | 6642 | crate-doc lede in crates/mdbook-summary/src/lib.rs |
| 71 | 1.00 | 71 | 3483 | pub-item names surface in src/cmd/clean.rs |
| 68 | 1.00 | 68 | 7597 | README headline in guide/src/cli/README.md |
| 68 | 1.00 | 68 | 7238 | mod/use plumbing in crates/xtask/src/main.rs |
| 67 | 1.00 | 67 | 7490 | mod/use plumbing in guide/guide-helper/src/lib.rs |
| 64 | 1.00 | 64 | 1230 | [package] in crates/mdbook-compare/Cargo.toml |
| 62 | 1.00 | 62 | 2871 | [package] in crates/xtask/Cargo.toml |
| 62 | 1.00 | 62 | 3015 | [package] in guide/guide-helper/Cargo.toml |
| 57 | 1.00 | 57 | 7413 | impl method sigs in guide/guide-helper/src/lib.rs |
| 56 | 1.00 | 56 | 3412 | pub item at src/cmd/watch.rs:62 |
| 54 | 1.00 | 54 | 6997 | pub-item doc at crates/mdbook-summary/src/lib.rs:84 |
| 50 | 1.00 | 50 | 2921 | README headline in crates/xtask/README.md |
| 50 | 1.00 | 50 | 7296 | [package] in examples/remove-emphasis/mdbook-remove-emphasis/Cargo.toml |
