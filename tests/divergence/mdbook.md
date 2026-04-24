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

## Walker waste (cost ≥ 50, no NS intersection)

| first_t | cost | batch |
|--------:|-----:|:------|
| 9899 | 135 | crates/mdbook-core/README.md section #0 |
| 9502 | 131 | crates/mdbook-markdown/README.md section #0 |
| 9633 | 131 | crates/mdbook-preprocessor/README.md section #0 |
| 9349 | 130 | crates/mdbook-renderer/README.md section #0 |
| 9764 | 131 | crates/mdbook-summary/README.md section #0 |
| 1477 | 115 | README headline in crates/mdbook-core/README.md |
| 1706 | 115 | README headline in crates/mdbook-driver/README.md |
| 1939 | 115 | README headline in crates/mdbook-html/README.md |
| 2154 | 120 | README headline in crates/mdbook-markdown/README.md |
| 2372 | 120 | README headline in crates/mdbook-preprocessor/README.md |
| 2589 | 120 | README headline in crates/mdbook-renderer/README.md |
| 2797 | 115 | README headline in crates/mdbook-summary/README.md |
| 2921 | 50 | README headline in crates/xtask/README.md |
| 8093 | 231 | README headline in guide/src/README.md |
| 7597 | 68 | README headline in guide/src/cli/README.md |
| 7728 | 111 | README headline in guide/src/for_developers/README.md |
| 1126 | 257 | [features] in Cargo.toml |
| 1230 | 64 | [package] in crates/mdbook-compare/Cargo.toml |
| 1362 | 89 | [package] in crates/mdbook-core/Cargo.toml |
| 1572 | 83 | [package] in crates/mdbook-driver/Cargo.toml |
| 1802 | 80 | [package] in crates/mdbook-html/Cargo.toml |
| 2034 | 83 | [package] in crates/mdbook-markdown/Cargo.toml |
| 2252 | 86 | [package] in crates/mdbook-preprocessor/Cargo.toml |
| 2469 | 85 | [package] in crates/mdbook-renderer/Cargo.toml |
| 2682 | 81 | [package] in crates/mdbook-summary/Cargo.toml |
| 2871 | 62 | [package] in crates/xtask/Cargo.toml |
| 7296 | 50 | [package] in examples/remove-emphasis/mdbook-remove-emphasis/Cargo.toml |
| 3015 | 62 | [package] in guide/guide-helper/Cargo.toml |
| 4263 | 147 | crate-doc lede in crates/mdbook-driver/src/lib.rs |
| 4757 | 103 | crate-doc lede in crates/mdbook-markdown/src/lib.rs |
| 5169 | 101 | crate-doc lede in crates/mdbook-preprocessor/src/lib.rs |
| 5935 | 98 | crate-doc lede in crates/mdbook-renderer/src/lib.rs |
| 6642 | 74 | crate-doc lede in crates/mdbook-summary/src/lib.rs |
| 6148 | 121 | impl method sigs in crates/mdbook-renderer/src/lib.rs |
| 9063 | 549 | impl method sigs in crates/mdbook-summary/src/lib.rs |
| 7413 | 57 | impl method sigs in guide/guide-helper/src/lib.rs |
| 9219 | 156 | mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs |
| 8237 | 144 | mod/use plumbing in crates/mdbook-renderer/src/lib.rs |
| 7134 | 137 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 7238 | 68 | mod/use plumbing in crates/xtask/src/main.rs |
| 7490 | 67 | mod/use plumbing in guide/guide-helper/src/lib.rs |
| 3677 | 96 | mod/use plumbing in src/main.rs |
| 4957 | 176 | pub item at crates/mdbook-markdown/src/lib.rs:15 |
| 3412 | 56 | pub item at src/cmd/watch.rs:62 |
| 5731 | 115 | pub-item doc at crates/mdbook-preprocessor/src/lib.rs:30 |
| 6027 | 92 | pub-item doc at crates/mdbook-renderer/src/lib.rs:28 |
| 6997 | 54 | pub-item doc at crates/mdbook-summary/src/lib.rs:84 |
| 8321 | 84 | pub-item names surface in crates/mdbook-html/src/utils.rs |
| 3483 | 71 | pub-item names surface in src/cmd/clean.rs |
