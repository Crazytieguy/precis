scores: Sim=0.393 Reached=18/49 Early=2 Late=14 Partial=8 Missing=23 Used=9956/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 1 | 1 | 0.82 |
| 2 | 10 | 7 | 2 | 1 | 0.85 |
| 3 | 17 | 4 | 3 | 10 | 0.36 |
| 4 | 7 | 0 | 2 | 5 | 0.19 |
| 5 | 2 | 0 | 0 | 2 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.34 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 173 | +98 | 0.80 | late | README lede | README headline in README.md (t=105, 2 atoms) |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Repo root listing |  |
| 1.3 | 202 | 724 | +522 | 1.00 | late | crates/ directory listing |  |
| 1.4 | 239 | 676 | +437 | 1.00 | late | Workspace package summary — name + description | [package] in Cargo.toml (t=676, 15 atoms) |
| 1.5 | 341 | — | — | 0.73 | partial | src/cmd module map | mod/use plumbing in src/cmd/mod.rs (t=2442, 7 atoms) |
| 1.6 | 482 | 676 | +194 | 0.92 | late | Workspace members + edition | [package] in Cargo.toml (t=676, 11 atoms) |
| 1.7 | 536 | 1951 | +1415 | 1.00 | late | src/ tree (top-level bin sources) |  |
| 1.8 | 741 | — | — | 0.08 | missing | CLI subcommand dispatch — match arms | mod/use plumbing in src/main.rs (t=2538, 9 atoms) |
| 2.1 | 882 | 3790 | +2908 | 1.00 | late | mdbook-driver crate role | crate-doc lede in crates/mdbook-driver/src/lib.rs (t=3790, 10 atoms) |
| 2.2 | 1047 | — | — | 0.69 | partial | mdbook-core lib + module map | mod/use plumbing in crates/mdbook-core/src/lib.rs (t=3381, 5 atoms) |
| 2.3 | 1121 | 6169 | +5048 | 1.00 | late | mdbook-summary crate lede | crate-doc lede in crates/mdbook-summary/src/lib.rs (t=6169, 5 atoms) |
| 2.4 | 1229 | 4284 | +3055 | 0.86 | late | mdbook-markdown crate lede | crate-doc lede in crates/mdbook-markdown/src/lib.rs (t=4284, 6 atoms) |
| 2.5 | 1330 | 4696 | +3366 | 1.00 | late | mdbook-preprocessor crate lede | crate-doc lede in crates/mdbook-preprocessor/src/lib.rs (t=4696, 6 atoms) |
| 2.6 | 1428 | 5462 | +4034 | 1.00 | late | mdbook-renderer crate lede | crate-doc lede in crates/mdbook-renderer/src/lib.rs (t=5462, 6 atoms) |
| 2.7 | 1496 | — | — | 0.75 | partial | mdbook-html crate lib + module map | mod/use plumbing in crates/mdbook-html/src/lib.rs (t=4133, 5 atoms) |
| 2.8 | 1555 | 9641 | +8086 | 1.00 | late | mdbook-driver source layout |  |
| 2.9 | 1591 | 9275 | +7684 | 1.00 | late | mdbook-core source layout |  |
| 2.10 | 1701 | — | — | 0.19 | missing | mdbook-html source layout |  |
| 3.1 | 1960 | — | — | 0.50 | partial | Book / BookItem struct shapes | pub item at crates/mdbook-core/src/book.rs:119 (t=3590, 8 atoms) |
| 3.2 | 2164 | — | — | 0.06 | missing | Chapter struct fields | pub-item names surface in crates/mdbook-core/src/book.rs (t=3487, 2 atoms) |
| 3.3 | 2246 | — | — | 0.00 | missing | Book impl method names |  |
| 3.4 | 2625 | — | — | 0.38 | missing | MDBook struct + public method names | pub item at crates/mdbook-driver/src/mdbook.rs:29 (t=9094, 12 atoms) |
| 3.5 | 3208 | 5258 | +2050 | 0.87 | late | Preprocessor trait + PreprocessorContext | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=5100, 16 atoms) |
| 3.6 | 3742 | 5978 | +2236 | 0.90 | late | Renderer trait + RenderContext | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=5873, 22 atoms) |
| 3.7 | 4011 | — | — | 0.00 | missing | Config struct (top-level book.toml shape) |  |
| 3.8 | 4133 | — | — | 0.00 | missing | Config method names + Config::set |  |
| 3.9 | 4753 | — | — | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields |  |
| 3.10 | 5119 | — | — | 0.00 | missing | HtmlConfig field-line catalog |  |
| 3.11 | 5489 | — | — | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines |  |
| 3.12 | 6067 | — | — | 0.00 | missing | Search config fields + chapter override settings |  |
| 3.13 | 6324 | 4484 | -1840 | 0.95 | aligned | MarkdownOptions + new_cmark_parser | pub item at crates/mdbook-markdown/src/lib.rs:15 (t=4484, 17 atoms) |
| 3.14 | 6851 | — | — | 0.76 | partial | Summary / SummaryItem / Link types + parse_summary | pub item at crates/mdbook-summary/src/lib.rs:84 (t=6299, 11 atoms) |
| 3.15 | 6973 | — | — | 0.67 | partial | Preprocessor input parsing + MDBOOK_VERSION re-export | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=5100, 16 atoms) |
| 3.16 | 7060 | 7670 | +610 | 1.00 | aligned+over | RenderContext impl methods (incl. from_json) | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=5873, 22 atoms) |
| 3.17 | 7552 | — | — | 0.00 | missing | MDBOOK_* env-var override rules |  |
| 4.1 | 7635 | — | — | 0.78 | partial | Builtin preprocessors module — re-exports | mod/use plumbing in crates/mdbook-driver/src/builtin_preprocessors/mod.rs (t=9790, 6 atoms) |
| 4.2 | 7841 | — | — | 0.00 | missing | LinkPreprocessor — supported helpers list |  |
| 4.3 | 8077 | — | — | 0.00 | missing | MDBook::load body — book.toml + Config wiring |  |
| 4.4 | 8287 | — | — | 0.07 | missing | load.rs — load_book entrypoint | pub-item names surface in crates/mdbook-driver/src/load.rs (t=3902, 2 atoms) |
| 4.5 | 8522 | — | — | 0.00 | missing | execute_build_process — the build pipeline |  |
| 4.6 | 8610 | — | — | 0.00 | missing | Default preprocessors + topological order |  |
| 4.7 | 8735 | — | — | 0.50 | partial | BookBuilder type — mdbook init API | pub item at crates/mdbook-driver/src/init.rs:13 (t=3833, 6 atoms) |
| 5.1 | 8977 | — | — | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline |  |
| 5.2 | 9440 | — | — | 0.00 | missing | Theme — bundled-asset names |  |
| 6.1 | 9515 | — | — | 0.15 | missing | build subcommand — clap surface | pub-item names surface in src/cmd/build.rs (t=2001, 2 atoms) |
| 6.2 | 9597 | — | — | 0.25 | missing | command_prelude shared args | pub-item names surface in src/cmd/command_prelude.rs (t=2142, 3 atoms) |
| 6.3 | 9801 | — | — | 0.00 | missing | tests/testsuite/main.rs — test module map |  |
| 6.4 | 9848 | — | — | 0.30 | missing | examples/ + guide/ tree |  |
| 6.5 | 9944 | 6700 | -3244 | 1.00 | early | guide/src tree — doc-source files |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 219 | CONTRIBUTING.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 549 | 1.00 | 549 | 8303 | impl method sigs in crates/mdbook-summary/src/lib.rs |
| 257 | 1.00 | 257 | 1221 | [features] in Cargo.toml |
| 231 | 1.00 | 231 | 7507 | README headline in guide/src/README.md |
| 176 | 0.50 | 349 | 676 | [package] in Cargo.toml |
| 159 | 1.00 | 159 | 891 | headings outline in CONTRIBUTING.md |
| 152 | 1.00 | 152 | 8797 | guide/src/README.md section #1 |
| 146 | 1.00 | 146 | 8645 | CONTRIBUTING.md section #1 |
| 145 | 1.00 | 145 | 9622 | macro_export bodies across crates/mdbook-core/src/utils |
| 137 | 1.00 | 137 | 6661 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 131 | 1.00 | 131 | 4033 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 111 | 1.00 | 111 | 6920 | README headline in guide/src/for_developers/README.md |
| 108 | 1.00 | 108 | 281 | README.md section #1 |
| 106 | 0.68 | 156 | 8459 | mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs |
| 98 | 1.00 | 98 | 1492 | README headline in crates/mdbook-driver/README.md |
| 96 | 1.00 | 96 | 2538 | mod/use plumbing in src/main.rs |
| 94 | 0.65 | 144 | 7670 | mod/use plumbing in crates/mdbook-renderer/src/lib.rs |
| 88 | 1.00 | 88 | 9477 | mod/use plumbing in crates/mdbook-core/src/utils/mod.rs |
| 84 | 1.00 | 84 | 7754 | pub-item names surface in crates/mdbook-html/src/utils.rs |
| 81 | 1.00 | 81 | 2658 | headings outline in guide/src/continuous-integration.md |
| 79 | 1.00 | 79 | 8953 | headings outline in guide/src/guide/creating.md |
| 78 | 1.00 | 78 | 9956 | impl method sigs in crates/mdbook-driver/src/builtin_renderers/mod.rs |
| 77 | 1.00 | 77 | 9171 | guide/src/for_developers/README.md section #0 |
| 77 | 1.00 | 77 | 8874 | headings outline in guide/src/for_developers/backends.md |
| 73 | 1.00 | 73 | 964 | CONTRIBUTING.md section #0 |
| 71 | 1.00 | 71 | 2344 | pub-item names surface in src/cmd/clean.rs |
| 68 | 1.00 | 68 | 1576 | README headline in crates/mdbook-html/README.md |
| 68 | 1.00 | 68 | 6789 | README headline in guide/src/cli/README.md |
| 59 | 1.00 | 59 | 1363 | README headline in crates/mdbook-core/README.md |
| 59 | 1.00 | 59 | 7190 | headings outline in guide/src/guide/installation.md |
| 58 | 1.00 | 58 | 1668 | README headline in crates/mdbook-markdown/README.md |
| 58 | 1.00 | 58 | 1738 | README headline in crates/mdbook-preprocessor/README.md |
| 58 | 1.00 | 58 | 2200 | pub-item names surface in src/cmd/watch.rs |
| 57 | 1.00 | 57 | 1807 | README headline in crates/mdbook-renderer/README.md |
| 57 | 1.00 | 57 | 1876 | README headline in crates/mdbook-summary/README.md |
| 56 | 1.00 | 56 | 2273 | pub item at src/cmd/watch.rs:62 |
| 55 | 1.00 | 55 | 7002 | headings outline in guide/src/for_developers/preprocessors.md |
| 54 | 1.00 | 54 | 6524 | pub-item doc at crates/mdbook-summary/src/lib.rs:84 |
| 50 | 1.00 | 50 | 2708 | crates/mdbook-core/README.md section #0 |
| 50 | 1.00 | 50 | 2876 | crates/mdbook-html/README.md section #0 |
