scores: Sim=0.393 Reached=17/49 Early=2 Late=11 Partial=8 Missing=24 Used=9803/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 1 | 1 | 0.82 |
| 2 | 10 | 7 | 3 | 0 | 0.88 |
| 3 | 17 | 3 | 2 | 12 | 0.28 |
| 4 | 7 | 0 | 2 | 5 | 0.20 |
| 5 | 2 | 0 | 0 | 2 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.34 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 227 | +152 | 0.80 | late | README lede | README headline in README.md (t=105, 2 atoms) |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Repo root listing |  |
| 1.3 | 202 | 896 | +694 | 1.00 | late | crates/ directory listing |  |
| 1.4 | 239 | 848 | +609 | 1.00 | late | Workspace package summary — name + description | [package] in Cargo.toml (t=848, 15 atoms) |
| 1.5 | 341 | — | — | 0.73 | partial | src/cmd module map | mod/use plumbing in src/cmd/mod.rs (t=6343, 7 atoms) |
| 1.6 | 482 | 848 | +366 | 0.92 | late | Workspace members + edition | [package] in Cargo.toml (t=848, 11 atoms) |
| 1.7 | 536 | 5534 | +4998 | 1.00 | late | src/ tree (top-level bin sources) |  |
| 1.8 | 741 | — | — | 0.08 | missing | CLI subcommand dispatch — match arms | mod/use plumbing in src/main.rs (t=6439, 9 atoms) |
| 2.1 | 882 | 7570 | +6688 | 1.00 | late | mdbook-driver crate role | crate-doc lede in crates/mdbook-driver/src/lib.rs (t=7570, 10 atoms) |
| 2.2 | 1047 | — | — | 0.69 | partial | mdbook-core lib + module map | mod/use plumbing in crates/mdbook-core/src/lib.rs (t=4847, 5 atoms) |
| 2.3 | 1121 | 2251 | +1130 | 1.00 | late | mdbook-summary crate lede | crate-doc lede in crates/mdbook-summary/src/lib.rs (t=2251, 5 atoms) |
| 2.4 | 1229 | 1220 | -9 | 0.86 | aligned | mdbook-markdown crate lede | crate-doc lede in crates/mdbook-markdown/src/lib.rs (t=1220, 6 atoms) |
| 2.6 | 1428 | 2036 | +608 | 1.00 | late | mdbook-renderer crate lede | crate-doc lede in crates/mdbook-renderer/src/lib.rs (t=2036, 6 atoms) |
| 2.7 | 1496 | — | — | 0.75 | partial | mdbook-html crate lib + module map | mod/use plumbing in crates/mdbook-html/src/lib.rs (t=4971, 5 atoms) |
| 2.8 | 1555 | 7861 | +6306 | 1.00 | late | mdbook-driver source layout |  |
| 2.9 | 1591 | 6636 | +5045 | 1.00 | late | mdbook-core source layout |  |
| 2.10 | 1701 | — | — | 0.54 | partial | mdbook-html source layout |  |
| 3.1 | 1960 | — | — | 0.54 | partial | Book / BookItem struct shapes | pub item at crates/mdbook-core/src/book.rs:119 (t=6618, 8 atoms) |
| 3.2 | 2164 | — | — | 0.06 | missing | Chapter struct fields | pub-item names surface in crates/mdbook-core/src/book.rs (t=6515, 2 atoms) |
| 3.3 | 2246 | — | — | 0.00 | missing | Book impl method names |  |
| 3.4 | 2625 | — | — | 0.03 | missing | MDBook struct + public method names | pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=7423, 2 atoms) |
| 3.5 | 3208 | 6240 | +3032 | 0.87 | late | Preprocessor trait + PreprocessorContext | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2731, 16 atoms) |
| 3.6 | 3742 | 4649 | +907 | 0.90 | aligned | Renderer trait + RenderContext | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3350, 22 atoms) |
| 3.7 | 4011 | — | — | 0.00 | missing | Config struct (top-level book.toml shape) |  |
| 3.8 | 4133 | — | — | 0.00 | missing | Config method names + Config::set |  |
| 3.9 | 4753 | — | — | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields |  |
| 3.10 | 5119 | — | — | 0.00 | missing | HtmlConfig field-line catalog |  |
| 3.11 | 5489 | — | — | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines |  |
| 3.12 | 6067 | — | — | 0.00 | missing | Search config fields + chapter override settings |  |
| 3.13 | 6324 | 1415 | -4909 | 0.95 | early | MarkdownOptions + new_cmark_parser | pub item at crates/mdbook-markdown/src/lib.rs:15 (t=1415, 17 atoms) |
| 3.14 | 6851 | — | — | 0.76 | partial | Summary / SummaryItem / Link types + parse_summary | pub item at crates/mdbook-summary/src/lib.rs:84 (t=2381, 11 atoms) |
| 3.15 | 6973 | — | — | 0.22 | missing | Preprocessor input parsing + MDBOOK_VERSION re-export | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2731, 16 atoms) |
| 3.16 | 7060 | — | — | 0.43 | missing | RenderContext impl methods (incl. from_json) | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3350, 22 atoms) |
| 3.17 | 7552 | — | — | 0.00 | missing | MDBOOK_* env-var override rules |  |
| 4.1 | 7635 | — | — | 0.78 | partial | Builtin preprocessors module — re-exports | mod/use plumbing in crates/mdbook-driver/src/builtin_preprocessors/mod.rs (t=8015, 6 atoms) |
| 4.2 | 7841 | — | — | 0.00 | missing | LinkPreprocessor — supported helpers list |  |
| 4.3 | 8077 | — | — | 0.00 | missing | MDBook::load body — book.toml + Config wiring |  |
| 4.4 | 8287 | — | — | 0.07 | missing | load.rs — load_book entrypoint | pub-item names surface in crates/mdbook-driver/src/load.rs (t=8436, 2 atoms) |
| 4.5 | 8522 | — | — | 0.00 | missing | execute_build_process — the build pipeline |  |
| 4.6 | 8610 | — | — | 0.00 | missing | Default preprocessors + topological order |  |
| 4.7 | 8735 | — | — | 0.58 | partial | BookBuilder type — mdbook init API | pub item at crates/mdbook-driver/src/init.rs:13 (t=7783, 6 atoms) |
| 5.1 | 8977 | — | — | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline |  |
| 5.2 | 9440 | — | — | 0.00 | missing | Theme — bundled-asset names |  |
| 6.1 | 9515 | — | — | 0.15 | missing | build subcommand — clap surface | pub-item names surface in src/cmd/build.rs (t=5617, 2 atoms) |
| 6.2 | 9597 | — | — | 0.25 | missing | command_prelude shared args | pub-item names surface in src/cmd/command_prelude.rs (t=5758, 3 atoms) |
| 6.3 | 9801 | — | — | 0.00 | missing | tests/testsuite/main.rs — test module map |  |
| 6.4 | 9848 | — | — | 0.30 | missing | examples/ + guide/ tree |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 306 | 1.00 | 306 | 7274 | pub item at crates/mdbook-html/src/theme/mod.rs:40 |
| 257 | 1.00 | 257 | 5496 | [features] in Cargo.toml |
| 176 | 0.50 | 349 | 848 | [package] in Cargo.toml |
| 159 | 1.00 | 159 | 4111 | headings outline in CONTRIBUTING.md |
| 145 | 1.00 | 145 | 6850 | macro_export body at crates/mdbook-core/src/utils/mod.rs:17 |
| 137 | 1.00 | 137 | 9536 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 131 | 1.00 | 131 | 9162 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 111 | 1.00 | 111 | 8175 | README headline in guide/src/for_developers/README.md |
| 108 | 1.00 | 108 | 452 | README.md section #1 |
| 102 | 1.00 | 102 | 9750 | mod/use plumbing in crates/mdbook-html/src/theme/mod.rs |
| 98 | 1.00 | 98 | 4474 | README headline in crates/mdbook-driver/README.md |
| 96 | 1.00 | 96 | 6439 | mod/use plumbing in src/main.rs |
| 96 | 1.00 | 96 | 7370 | pub-item doc lede at crates/mdbook-html/src/theme/mod.rs:40 |
| 89 | 1.00 | 89 | 8613 | mod/use plumbing in crates/mdbook-driver/src/builtin_renderers/mod.rs |
| 89 | 1.00 | 89 | 344 | package scripts in package.json |
| 88 | 1.00 | 88 | 8524 | mod/use plumbing in crates/mdbook-core/src/utils/mod.rs |
| 81 | 1.00 | 81 | 8283 | headings outline in guide/src/continuous-integration.md |
| 80 | 1.00 | 80 | 6968 | impl method sigs in crates/mdbook-html/src/theme/mod.rs |
| 78 | 1.00 | 78 | 7740 | impl method sigs in crates/mdbook-driver/src/builtin_renderers/mod.rs |
| 73 | 1.00 | 73 | 4184 | CONTRIBUTING.md section #0 |
| 71 | 1.00 | 71 | 6086 | pub-item names surface in src/cmd/clean.rs |
| 68 | 1.00 | 68 | 3606 | README headline in crates/mdbook-html/README.md |
| 68 | 1.00 | 68 | 9349 | README headline in guide/src/cli/README.md |
| 63 | 1.00 | 63 | 5239 | mod/use plumbing in guide/src/for_developers/mdbook-wordcount/src/main.rs |
| 59 | 1.00 | 59 | 3538 | README headline in crates/mdbook-core/README.md |
| 59 | 1.00 | 59 | 9221 | headings outline in guide/src/guide/installation.md |
| 59 | 1.00 | 59 | 7842 | pub-item doc lede at crates/mdbook-driver/src/builtin_renderers/mod.rs:20 |
| 58 | 1.00 | 58 | 3421 | README headline in crates/mdbook-markdown/README.md |
| 58 | 1.00 | 58 | 3479 | README headline in crates/mdbook-preprocessor/README.md |
| 58 | 1.00 | 58 | 5816 | pub-item names surface in src/cmd/watch.rs |
| 57 | 1.00 | 57 | 3003 | README headline in crates/mdbook-renderer/README.md |
| 57 | 1.00 | 57 | 3060 | README headline in crates/mdbook-summary/README.md |
| 56 | 1.00 | 56 | 8869 | README headline in guide/src/format/theme/README.md |
| 56 | 1.00 | 56 | 5889 | pub item at src/cmd/watch.rs:62 |
| 55 | 1.00 | 55 | 9031 | headings outline in guide/src/for_developers/preprocessors.md |
| 54 | 1.00 | 54 | 3913 | pub-item doc lede at crates/mdbook-summary/src/lib.rs:84 |
| 52 | 1.00 | 52 | 5176 | [package] in guide/src/for_developers/mdbook-wordcount/Cargo.toml |
| 50 | 1.00 | 50 | 4699 | crates/mdbook-core/README.md section #0 |
| 50 | 1.00 | 50 | 4749 | crates/mdbook-html/README.md section #0 |
