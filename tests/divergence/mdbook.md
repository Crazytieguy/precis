scores: Sim=0.354 Reached=17/49 Early=2 Late=12 Partial=9 Missing=23 Used=9899/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 1 | 1 | 0.82 |
| 2 | 10 | 5 | 4 | 1 | 0.75 |
| 3 | 17 | 4 | 3 | 10 | 0.36 |
| 4 | 7 | 0 | 1 | 6 | 0.08 |
| 5 | 2 | 0 | 0 | 2 | 0.00 |
| 6 | 5 | 2 | 0 | 3 | 0.48 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 869 | +794 | 0.80 | late | README lede | README headline in README.md (t=190, 4 atoms) |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Repo root listing |  |
| 1.3 | 202 | 675 | +473 | 1.00 | late | crates/ directory listing |  |
| 1.4 | 239 | 627 | +388 | 1.00 | late | Workspace package summary — name + description | [package] in Cargo.toml (t=627, 15 atoms) |
| 1.5 | 341 | — | — | 0.73 | partial | src/cmd module map | mod/use plumbing in src/cmd/mod.rs (t=3581, 7 atoms) |
| 1.6 | 482 | 627 | +145 | 0.92 | late | Workspace members + edition | [package] in Cargo.toml (t=627, 11 atoms) |
| 1.7 | 536 | 3090 | +2554 | 1.00 | late | src/ tree (top-level bin sources) |  |
| 1.8 | 741 | — | — | 0.08 | missing | CLI subcommand dispatch — match arms | mod/use plumbing in src/main.rs (t=3677, 9 atoms) |
| 2.1 | 882 | 4263 | +3381 | 1.00 | late | mdbook-driver crate role | crate-doc lede in crates/mdbook-driver/src/lib.rs (t=4263, 10 atoms) |
| 2.2 | 1047 | — | — | 0.69 | partial | mdbook-core lib + module map | mod/use plumbing in crates/mdbook-core/src/lib.rs (t=3901, 5 atoms) |
| 2.3 | 1121 | 6642 | +5521 | 1.00 | late | mdbook-summary crate lede | crate-doc lede in crates/mdbook-summary/src/lib.rs (t=6642, 5 atoms) |
| 2.4 | 1229 | 4757 | +3528 | 0.86 | late | mdbook-markdown crate lede | crate-doc lede in crates/mdbook-markdown/src/lib.rs (t=4757, 6 atoms) |
| 2.5 | 1330 | 5169 | +3839 | 1.00 | late | mdbook-preprocessor crate lede | crate-doc lede in crates/mdbook-preprocessor/src/lib.rs (t=5169, 6 atoms) |
| 2.6 | 1428 | 5935 | +4507 | 1.00 | late | mdbook-renderer crate lede | crate-doc lede in crates/mdbook-renderer/src/lib.rs (t=5935, 6 atoms) |
| 2.7 | 1496 | — | — | 0.75 | partial | mdbook-html crate lib + module map | mod/use plumbing in crates/mdbook-html/src/lib.rs (t=4606, 5 atoms) |
| 2.8 | 1555 | — | — | 0.50 | partial | mdbook-driver source layout |  |
| 2.9 | 1591 | — | — | 0.56 | partial | mdbook-core source layout |  |
| 2.10 | 1701 | — | — | 0.19 | missing | mdbook-html source layout |  |
| 3.1 | 1960 | — | — | 0.50 | partial | Book / BookItem struct shapes | pub item at crates/mdbook-core/src/book.rs:119 (t=4063, 8 atoms) |
| 3.2 | 2164 | — | — | 0.06 | missing | Chapter struct fields | pub-item names surface in crates/mdbook-core/src/book.rs (t=3960, 2 atoms) |
| 3.3 | 2246 | — | — | 0.00 | missing | Book impl method names |  |
| 3.4 | 2625 | — | — | 0.38 | missing | MDBook struct + public method names | pub item at crates/mdbook-driver/src/mdbook.rs:29 (t=8514, 12 atoms) |
| 3.5 | 3208 | 5731 | +2523 | 0.87 | late | Preprocessor trait + PreprocessorContext | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=5573, 16 atoms) |
| 3.6 | 3742 | 6438 | +2696 | 0.90 | late | Renderer trait + RenderContext | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=6438, 22 atoms) |
| 3.7 | 4011 | — | — | 0.00 | missing | Config struct (top-level book.toml shape) |  |
| 3.8 | 4133 | — | — | 0.00 | missing | Config method names + Config::set |  |
| 3.9 | 4753 | — | — | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields |  |
| 3.10 | 5119 | — | — | 0.00 | missing | HtmlConfig field-line catalog |  |
| 3.11 | 5489 | — | — | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines |  |
| 3.12 | 6067 | — | — | 0.00 | missing | Search config fields + chapter override settings |  |
| 3.13 | 6324 | 4957 | -1367 | 0.95 | aligned | MarkdownOptions + new_cmark_parser | pub item at crates/mdbook-markdown/src/lib.rs:15 (t=4957, 17 atoms) |
| 3.14 | 6851 | — | — | 0.76 | partial | Summary / SummaryItem / Link types + parse_summary | pub item at crates/mdbook-summary/src/lib.rs:84 (t=6772, 11 atoms) |
| 3.15 | 6973 | — | — | 0.67 | partial | Preprocessor input parsing + MDBOOK_VERSION re-export | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=5573, 16 atoms) |
| 3.16 | 7060 | 8237 | +1177 | 1.00 | aligned+over | RenderContext impl methods (incl. from_json) | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=6438, 22 atoms) |
| 3.17 | 7552 | — | — | 0.00 | missing | MDBOOK_* env-var override rules |  |
| 4.1 | 7635 | — | — | 0.00 | missing | Builtin preprocessors module — re-exports |  |
| 4.2 | 7841 | — | — | 0.00 | missing | LinkPreprocessor — supported helpers list |  |
| 4.3 | 8077 | — | — | 0.00 | missing | MDBook::load body — book.toml + Config wiring |  |
| 4.4 | 8287 | — | — | 0.07 | missing | load.rs — load_book entrypoint | pub-item names surface in crates/mdbook-driver/src/load.rs (t=4375, 2 atoms) |
| 4.5 | 8522 | — | — | 0.00 | missing | execute_build_process — the build pipeline |  |
| 4.6 | 8610 | — | — | 0.00 | missing | Default preprocessors + topological order |  |
| 4.7 | 8735 | — | — | 0.50 | partial | BookBuilder type — mdbook init API | pub item at crates/mdbook-driver/src/init.rs:13 (t=4306, 6 atoms) |
| 5.1 | 8977 | — | — | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline |  |
| 5.2 | 9440 | — | — | 0.00 | missing | Theme — bundled-asset names |  |
| 6.1 | 9515 | — | — | 0.15 | missing | build subcommand — clap surface | pub-item names surface in src/cmd/build.rs (t=3140, 2 atoms) |
| 6.2 | 9597 | — | — | 0.25 | missing | command_prelude shared args | pub-item names surface in src/cmd/command_prelude.rs (t=3281, 3 atoms) |
| 6.3 | 9801 | — | — | 0.00 | missing | tests/testsuite/main.rs — test module map |  |
| 6.4 | 9848 | 2945 | -6903 | 1.00 | early | examples/ + guide/ tree |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 549 | 1.00 | 549 | 9063 | impl method sigs in crates/mdbook-summary/src/lib.rs |
| 257 | 1.00 | 257 | 1126 | [features] in Cargo.toml |
| 231 | 1.00 | 231 | 8093 | README headline in guide/src/README.md |
| 174 | 0.50 | 349 | 627 | [package] in Cargo.toml |
| 137 | 1.00 | 137 | 7134 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 135 | 1.00 | 135 | 9899 | crates/mdbook-core/README.md section #0 |
| 131 | 1.00 | 131 | 9502 | crates/mdbook-markdown/README.md section #0 |
| 131 | 1.00 | 131 | 9633 | crates/mdbook-preprocessor/README.md section #0 |
| 131 | 1.00 | 131 | 9764 | crates/mdbook-summary/README.md section #0 |
| 131 | 1.00 | 131 | 4506 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 130 | 1.00 | 130 | 9349 | crates/mdbook-renderer/README.md section #0 |
| 129 | 0.67 | 194 | 869 | README.md section #0 |
| 120 | 1.00 | 120 | 2154 | README headline in crates/mdbook-markdown/README.md |
| 120 | 1.00 | 120 | 2372 | README headline in crates/mdbook-preprocessor/README.md |
| 120 | 1.00 | 120 | 2589 | README headline in crates/mdbook-renderer/README.md |
| 115 | 1.00 | 115 | 1477 | README headline in crates/mdbook-core/README.md |
| 115 | 1.00 | 115 | 1706 | README headline in crates/mdbook-driver/README.md |
| 115 | 1.00 | 115 | 1939 | README headline in crates/mdbook-html/README.md |
| 115 | 1.00 | 115 | 2797 | README headline in crates/mdbook-summary/README.md |
| 111 | 1.00 | 111 | 7728 | README headline in guide/src/for_developers/README.md |
| 108 | 0.69 | 156 | 9219 | mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs |
| 96 | 0.67 | 144 | 8237 | mod/use plumbing in crates/mdbook-renderer/src/lib.rs |
| 96 | 1.00 | 96 | 3677 | mod/use plumbing in src/main.rs |
| 89 | 1.00 | 89 | 1362 | [package] in crates/mdbook-core/Cargo.toml |
| 88 | 0.73 | 121 | 6148 | impl method sigs in crates/mdbook-renderer/src/lib.rs |
| 86 | 1.00 | 86 | 2252 | [package] in crates/mdbook-preprocessor/Cargo.toml |
| 85 | 1.00 | 85 | 2469 | [package] in crates/mdbook-renderer/Cargo.toml |
| 84 | 1.00 | 84 | 8321 | pub-item names surface in crates/mdbook-html/src/utils.rs |
| 83 | 0.75 | 111 | 190 | README headline in README.md |
| 83 | 1.00 | 83 | 1572 | [package] in crates/mdbook-driver/Cargo.toml |
| 83 | 1.00 | 83 | 2034 | [package] in crates/mdbook-markdown/Cargo.toml |
| 81 | 1.00 | 81 | 2682 | [package] in crates/mdbook-summary/Cargo.toml |
| 80 | 1.00 | 80 | 1802 | [package] in crates/mdbook-html/Cargo.toml |
| 71 | 1.00 | 71 | 3483 | pub-item names surface in src/cmd/clean.rs |
| 68 | 1.00 | 68 | 7597 | README headline in guide/src/cli/README.md |
| 68 | 1.00 | 68 | 7238 | mod/use plumbing in crates/xtask/src/main.rs |
| 67 | 1.00 | 67 | 7490 | mod/use plumbing in guide/guide-helper/src/lib.rs |
| 64 | 1.00 | 64 | 1230 | [package] in crates/mdbook-compare/Cargo.toml |
| 62 | 1.00 | 62 | 2871 | [package] in crates/xtask/Cargo.toml |
| 62 | 1.00 | 62 | 3015 | [package] in guide/guide-helper/Cargo.toml |
| 58 | 1.00 | 58 | 3339 | pub-item names surface in src/cmd/watch.rs |
| 57 | 1.00 | 57 | 7413 | impl method sigs in guide/guide-helper/src/lib.rs |
| 56 | 1.00 | 56 | 3412 | pub item at src/cmd/watch.rs:62 |
| 54 | 1.00 | 54 | 6997 | pub-item doc at crates/mdbook-summary/src/lib.rs:84 |
| 50 | 1.00 | 50 | 2921 | README headline in crates/xtask/README.md |
| 50 | 1.00 | 50 | 7296 | [package] in examples/remove-emphasis/mdbook-remove-emphasis/Cargo.toml |
