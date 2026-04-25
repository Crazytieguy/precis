scores: Sim=0.342 Reached=16/49 Early=1 Late=12 Partial=9 Missing=24 Used=9908/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 1 | 1 | 0.82 |
| 2 | 10 | 5 | 4 | 1 | 0.75 |
| 3 | 17 | 4 | 3 | 10 | 0.36 |
| 4 | 7 | 0 | 1 | 6 | 0.08 |
| 5 | 2 | 0 | 0 | 2 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.34 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 276 | +201 | 0.80 | late | README lede | README headline in README.md (t=190, 4 atoms) |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Repo root listing |  |
| 1.3 | 202 | 827 | +625 | 1.00 | late | crates/ directory listing |  |
| 1.4 | 239 | 779 | +540 | 1.00 | late | Workspace package summary — name + description | [package] in Cargo.toml (t=779, 15 atoms) |
| 1.5 | 341 | — | — | 0.73 | partial | src/cmd module map | mod/use plumbing in src/cmd/mod.rs (t=3410, 7 atoms) |
| 1.6 | 482 | 779 | +297 | 0.92 | late | Workspace members + edition | [package] in Cargo.toml (t=779, 11 atoms) |
| 1.7 | 536 | 2919 | +2383 | 1.00 | late | src/ tree (top-level bin sources) |  |
| 1.8 | 741 | — | — | 0.08 | missing | CLI subcommand dispatch — match arms | mod/use plumbing in src/main.rs (t=3506, 9 atoms) |
| 2.1 | 882 | 4929 | +4047 | 1.00 | late | mdbook-driver crate role | crate-doc lede in crates/mdbook-driver/src/lib.rs (t=4929, 10 atoms) |
| 2.2 | 1047 | — | — | 0.69 | partial | mdbook-core lib + module map | mod/use plumbing in crates/mdbook-core/src/lib.rs (t=4567, 5 atoms) |
| 2.3 | 1121 | 7308 | +6187 | 1.00 | late | mdbook-summary crate lede | crate-doc lede in crates/mdbook-summary/src/lib.rs (t=7308, 5 atoms) |
| 2.4 | 1229 | 5423 | +4194 | 0.86 | late | mdbook-markdown crate lede | crate-doc lede in crates/mdbook-markdown/src/lib.rs (t=5423, 6 atoms) |
| 2.5 | 1330 | 5835 | +4505 | 1.00 | late | mdbook-preprocessor crate lede | crate-doc lede in crates/mdbook-preprocessor/src/lib.rs (t=5835, 6 atoms) |
| 2.6 | 1428 | 6601 | +5173 | 1.00 | late | mdbook-renderer crate lede | crate-doc lede in crates/mdbook-renderer/src/lib.rs (t=6601, 6 atoms) |
| 2.7 | 1496 | — | — | 0.75 | partial | mdbook-html crate lib + module map | mod/use plumbing in crates/mdbook-html/src/lib.rs (t=5272, 5 atoms) |
| 2.8 | 1555 | — | — | 0.50 | partial | mdbook-driver source layout |  |
| 2.9 | 1591 | — | — | 0.56 | partial | mdbook-core source layout |  |
| 2.10 | 1701 | — | — | 0.19 | missing | mdbook-html source layout |  |
| 3.1 | 1960 | — | — | 0.54 | partial | Book / BookItem struct shapes | pub item at crates/mdbook-core/src/book.rs:119 (t=4729, 8 atoms) |
| 3.2 | 2164 | — | — | 0.06 | missing | Chapter struct fields | pub-item names surface in crates/mdbook-core/src/book.rs (t=4626, 2 atoms) |
| 3.3 | 2246 | — | — | 0.00 | missing | Book impl method names |  |
| 3.4 | 2625 | — | — | 0.38 | missing | MDBook struct + public method names | pub item at crates/mdbook-driver/src/mdbook.rs:29 (t=8871, 12 atoms) |
| 3.5 | 3208 | 6397 | +3189 | 0.87 | late | Preprocessor trait + PreprocessorContext | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=6239, 16 atoms) |
| 3.6 | 3742 | 7104 | +3362 | 0.90 | late | Renderer trait + RenderContext | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=7104, 22 atoms) |
| 3.7 | 4011 | — | — | 0.00 | missing | Config struct (top-level book.toml shape) |  |
| 3.8 | 4133 | — | — | 0.00 | missing | Config method names + Config::set |  |
| 3.9 | 4753 | — | — | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields |  |
| 3.10 | 5119 | — | — | 0.00 | missing | HtmlConfig field-line catalog |  |
| 3.11 | 5489 | — | — | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines |  |
| 3.12 | 6067 | — | — | 0.00 | missing | Search config fields + chapter override settings |  |
| 3.13 | 6324 | 5623 | -701 | 0.95 | aligned | MarkdownOptions + new_cmark_parser | pub item at crates/mdbook-markdown/src/lib.rs:15 (t=5623, 17 atoms) |
| 3.14 | 6851 | — | — | 0.76 | partial | Summary / SummaryItem / Link types + parse_summary | pub item at crates/mdbook-summary/src/lib.rs:84 (t=7438, 11 atoms) |
| 3.15 | 6973 | — | — | 0.67 | partial | Preprocessor input parsing + MDBOOK_VERSION re-export | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=6239, 16 atoms) |
| 3.16 | 7060 | 8594 | +1534 | 1.00 | aligned+over | RenderContext impl methods (incl. from_json) | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=7104, 22 atoms) |
| 3.17 | 7552 | — | — | 0.00 | missing | MDBOOK_* env-var override rules |  |
| 4.1 | 7635 | — | — | 0.00 | missing | Builtin preprocessors module — re-exports |  |
| 4.2 | 7841 | — | — | 0.00 | missing | LinkPreprocessor — supported helpers list |  |
| 4.3 | 8077 | — | — | 0.00 | missing | MDBook::load body — book.toml + Config wiring |  |
| 4.4 | 8287 | — | — | 0.07 | missing | load.rs — load_book entrypoint | pub-item names surface in crates/mdbook-driver/src/load.rs (t=5041, 2 atoms) |
| 4.5 | 8522 | — | — | 0.00 | missing | execute_build_process — the build pipeline |  |
| 4.6 | 8610 | — | — | 0.00 | missing | Default preprocessors + topological order |  |
| 4.7 | 8735 | — | — | 0.50 | partial | BookBuilder type — mdbook init API | pub item at crates/mdbook-driver/src/init.rs:13 (t=4972, 6 atoms) |
| 5.1 | 8977 | — | — | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline |  |
| 5.2 | 9440 | — | — | 0.00 | missing | Theme — bundled-asset names |  |
| 6.1 | 9515 | — | — | 0.15 | missing | build subcommand — clap surface | pub-item names surface in src/cmd/build.rs (t=2969, 2 atoms) |
| 6.2 | 9597 | — | — | 0.25 | missing | command_prelude shared args | pub-item names surface in src/cmd/command_prelude.rs (t=3110, 3 atoms) |
| 6.3 | 9801 | — | — | 0.00 | missing | tests/testsuite/main.rs — test module map |  |
| 6.4 | 9848 | — | — | 0.30 | missing | examples/ + guide/ tree |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 549 | 1.00 | 549 | 9420 | impl method sigs in crates/mdbook-summary/src/lib.rs |
| 257 | 1.00 | 257 | 1173 | [features] in Cargo.toml |
| 231 | 1.00 | 231 | 8403 | README headline in guide/src/README.md |
| 174 | 0.50 | 349 | 779 | [package] in Cargo.toml |
| 154 | 1.00 | 154 | 9908 | CONTRIBUTING.md section #1 |
| 137 | 1.00 | 137 | 9735 | crates/mdbook-driver/README.md section #0 |
| 137 | 1.00 | 137 | 7800 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 131 | 1.00 | 131 | 5172 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 120 | 1.00 | 120 | 2201 | README headline in crates/mdbook-markdown/README.md |
| 120 | 1.00 | 120 | 2419 | README headline in crates/mdbook-preprocessor/README.md |
| 120 | 1.00 | 120 | 2636 | README headline in crates/mdbook-renderer/README.md |
| 115 | 1.00 | 115 | 1524 | README headline in crates/mdbook-core/README.md |
| 115 | 1.00 | 115 | 1753 | README headline in crates/mdbook-driver/README.md |
| 115 | 1.00 | 115 | 1986 | README headline in crates/mdbook-html/README.md |
| 115 | 1.00 | 115 | 2844 | README headline in crates/mdbook-summary/README.md |
| 111 | 1.00 | 111 | 8038 | README headline in guide/src/for_developers/README.md |
| 109 | 1.00 | 109 | 4382 | crates/mdbook-html/README.md section #0 |
| 108 | 1.00 | 108 | 384 | README.md section #1 |
| 108 | 0.69 | 156 | 9576 | mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs |
| 100 | 1.00 | 100 | 3645 | crates/mdbook-core/README.md section #0 |
| 96 | 1.00 | 96 | 3846 | crates/mdbook-markdown/README.md section #0 |
| 96 | 1.00 | 96 | 3977 | crates/mdbook-preprocessor/README.md section #0 |
| 96 | 1.00 | 96 | 4238 | crates/mdbook-summary/README.md section #0 |
| 96 | 0.67 | 144 | 8594 | mod/use plumbing in crates/mdbook-renderer/src/lib.rs |
| 96 | 1.00 | 96 | 3506 | mod/use plumbing in src/main.rs |
| 95 | 1.00 | 95 | 4107 | crates/mdbook-renderer/README.md section #0 |
| 89 | 1.00 | 89 | 1409 | [package] in crates/mdbook-core/Cargo.toml |
| 88 | 0.73 | 121 | 6814 | impl method sigs in crates/mdbook-renderer/src/lib.rs |
| 86 | 1.00 | 86 | 2299 | [package] in crates/mdbook-preprocessor/Cargo.toml |
| 85 | 1.00 | 85 | 2516 | [package] in crates/mdbook-renderer/Cargo.toml |
| 84 | 1.00 | 84 | 8678 | pub-item names surface in crates/mdbook-html/src/utils.rs |
| 83 | 0.75 | 111 | 190 | README headline in README.md |
| 83 | 1.00 | 83 | 1619 | [package] in crates/mdbook-driver/Cargo.toml |
| 83 | 1.00 | 83 | 2081 | [package] in crates/mdbook-markdown/Cargo.toml |
| 81 | 1.00 | 81 | 916 | CONTRIBUTING.md section #0 |
| 81 | 1.00 | 81 | 2729 | [package] in crates/mdbook-summary/Cargo.toml |
| 80 | 1.00 | 80 | 1849 | [package] in crates/mdbook-html/Cargo.toml |
| 71 | 1.00 | 71 | 3312 | pub-item names surface in src/cmd/clean.rs |
| 68 | 1.00 | 68 | 7907 | README headline in guide/src/cli/README.md |
| 64 | 1.00 | 64 | 1277 | [package] in crates/mdbook-compare/Cargo.toml |
| 58 | 1.00 | 58 | 3168 | pub-item names surface in src/cmd/watch.rs |
| 56 | 1.00 | 56 | 3241 | pub item at src/cmd/watch.rs:62 |
| 54 | 1.00 | 54 | 7663 | pub-item doc at crates/mdbook-summary/src/lib.rs:84 |
