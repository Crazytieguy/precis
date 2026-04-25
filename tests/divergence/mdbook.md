scores: Sim=0.375 Reached=17/49 Early=1 Late=13 Partial=8 Missing=24 Used=9997/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 6 | 1 | 1 | 0.82 |
| 2 | 10 | 6 | 3 | 1 | 0.80 |
| 3 | 17 | 4 | 3 | 10 | 0.36 |
| 4 | 7 | 0 | 1 | 6 | 0.08 |
| 5 | 2 | 0 | 0 | 2 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.34 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 173 | +98 | 0.80 | late | README lede | README headline in README.md (t=105, 2 atoms) |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Repo root listing |  |
| 1.3 | 202 | 724 | +522 | 1.00 | late | crates/ directory listing |  |
| 1.4 | 239 | 676 | +437 | 1.00 | late | Workspace package summary — name + description | [package] in Cargo.toml (t=676, 15 atoms) |
| 1.5 | 341 | — | — | 0.73 | partial | src/cmd module map | mod/use plumbing in src/cmd/mod.rs (t=3093, 7 atoms) |
| 1.6 | 482 | 676 | +194 | 0.92 | late | Workspace members + edition | [package] in Cargo.toml (t=676, 11 atoms) |
| 1.7 | 536 | 2602 | +2066 | 1.00 | late | src/ tree (top-level bin sources) |  |
| 1.8 | 741 | — | — | 0.08 | missing | CLI subcommand dispatch — match arms | mod/use plumbing in src/main.rs (t=3189, 9 atoms) |
| 2.1 | 882 | 4441 | +3559 | 1.00 | late | mdbook-driver crate role | crate-doc lede in crates/mdbook-driver/src/lib.rs (t=4441, 10 atoms) |
| 2.2 | 1047 | — | — | 0.69 | partial | mdbook-core lib + module map | mod/use plumbing in crates/mdbook-core/src/lib.rs (t=4032, 5 atoms) |
| 2.3 | 1121 | 6820 | +5699 | 1.00 | late | mdbook-summary crate lede | crate-doc lede in crates/mdbook-summary/src/lib.rs (t=6820, 5 atoms) |
| 2.4 | 1229 | 4935 | +3706 | 0.86 | late | mdbook-markdown crate lede | crate-doc lede in crates/mdbook-markdown/src/lib.rs (t=4935, 6 atoms) |
| 2.5 | 1330 | 5347 | +4017 | 1.00 | late | mdbook-preprocessor crate lede | crate-doc lede in crates/mdbook-preprocessor/src/lib.rs (t=5347, 6 atoms) |
| 2.6 | 1428 | 6113 | +4685 | 1.00 | late | mdbook-renderer crate lede | crate-doc lede in crates/mdbook-renderer/src/lib.rs (t=6113, 6 atoms) |
| 2.7 | 1496 | — | — | 0.75 | partial | mdbook-html crate lib + module map | mod/use plumbing in crates/mdbook-html/src/lib.rs (t=4784, 5 atoms) |
| 2.8 | 1555 | — | — | 0.50 | partial | mdbook-driver source layout |  |
| 2.9 | 1591 | 9944 | +8353 | 1.00 | late | mdbook-core source layout |  |
| 2.10 | 1701 | — | — | 0.19 | missing | mdbook-html source layout |  |
| 3.1 | 1960 | — | — | 0.50 | partial | Book / BookItem struct shapes | pub item at crates/mdbook-core/src/book.rs:119 (t=4241, 8 atoms) |
| 3.2 | 2164 | — | — | 0.06 | missing | Chapter struct fields | pub-item names surface in crates/mdbook-core/src/book.rs (t=4138, 2 atoms) |
| 3.3 | 2246 | — | — | 0.00 | missing | Book impl method names |  |
| 3.4 | 2625 | — | — | 0.38 | missing | MDBook struct + public method names | pub item at crates/mdbook-driver/src/mdbook.rs:29 (t=9763, 12 atoms) |
| 3.5 | 3208 | 5909 | +2701 | 0.87 | late | Preprocessor trait + PreprocessorContext | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=5751, 16 atoms) |
| 3.6 | 3742 | 6629 | +2887 | 0.90 | late | Renderer trait + RenderContext | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=6524, 22 atoms) |
| 3.7 | 4011 | — | — | 0.00 | missing | Config struct (top-level book.toml shape) |  |
| 3.8 | 4133 | — | — | 0.00 | missing | Config method names + Config::set |  |
| 3.9 | 4753 | — | — | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields |  |
| 3.10 | 5119 | — | — | 0.00 | missing | HtmlConfig field-line catalog |  |
| 3.11 | 5489 | — | — | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines |  |
| 3.12 | 6067 | — | — | 0.00 | missing | Search config fields + chapter override settings |  |
| 3.13 | 6324 | 5135 | -1189 | 0.95 | aligned | MarkdownOptions + new_cmark_parser | pub item at crates/mdbook-markdown/src/lib.rs:15 (t=5135, 17 atoms) |
| 3.14 | 6851 | — | — | 0.76 | partial | Summary / SummaryItem / Link types + parse_summary | pub item at crates/mdbook-summary/src/lib.rs:84 (t=6950, 11 atoms) |
| 3.15 | 6973 | — | — | 0.67 | partial | Preprocessor input parsing + MDBOOK_VERSION re-export | pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=5751, 16 atoms) |
| 3.16 | 7060 | 8361 | +1301 | 1.00 | aligned+over | RenderContext impl methods (incl. from_json) | pub item at crates/mdbook-renderer/src/lib.rs:40 (t=6524, 22 atoms) |
| 3.17 | 7552 | — | — | 0.00 | missing | MDBOOK_* env-var override rules |  |
| 4.1 | 7635 | — | — | 0.00 | missing | Builtin preprocessors module — re-exports |  |
| 4.2 | 7841 | — | — | 0.00 | missing | LinkPreprocessor — supported helpers list |  |
| 4.3 | 8077 | — | — | 0.00 | missing | MDBook::load body — book.toml + Config wiring |  |
| 4.4 | 8287 | — | — | 0.07 | missing | load.rs — load_book entrypoint | pub-item names surface in crates/mdbook-driver/src/load.rs (t=4553, 2 atoms) |
| 4.5 | 8522 | — | — | 0.00 | missing | execute_build_process — the build pipeline |  |
| 4.6 | 8610 | — | — | 0.00 | missing | Default preprocessors + topological order |  |
| 4.7 | 8735 | — | — | 0.50 | partial | BookBuilder type — mdbook init API | pub item at crates/mdbook-driver/src/init.rs:13 (t=4484, 6 atoms) |
| 5.1 | 8977 | — | — | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline |  |
| 5.2 | 9440 | — | — | 0.00 | missing | Theme — bundled-asset names |  |
| 6.1 | 9515 | — | — | 0.15 | missing | build subcommand — clap surface | pub-item names surface in src/cmd/build.rs (t=2652, 2 atoms) |
| 6.2 | 9597 | — | — | 0.25 | missing | command_prelude shared args | pub-item names surface in src/cmd/command_prelude.rs (t=2793, 3 atoms) |
| 6.3 | 9801 | — | — | 0.00 | missing | tests/testsuite/main.rs — test module map |  |
| 6.4 | 9848 | — | — | 0.30 | missing | examples/ + guide/ tree |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 219 | CONTRIBUTING.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 549 | 1.00 | 549 | 9012 | impl method sigs in crates/mdbook-summary/src/lib.rs |
| 257 | 1.00 | 257 | 1221 | [features] in Cargo.toml |
| 231 | 1.00 | 231 | 8158 | README headline in guide/src/README.md |
| 174 | 0.50 | 349 | 676 | [package] in Cargo.toml |
| 159 | 1.00 | 159 | 891 | headings outline in CONTRIBUTING.md |
| 152 | 1.00 | 152 | 9466 | guide/src/README.md section #1 |
| 146 | 1.00 | 146 | 9314 | CONTRIBUTING.md section #1 |
| 137 | 1.00 | 137 | 7312 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 131 | 1.00 | 131 | 4684 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 111 | 1.00 | 111 | 7571 | README headline in guide/src/for_developers/README.md |
| 108 | 1.00 | 108 | 281 | README.md section #1 |
| 108 | 0.69 | 156 | 9168 | mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs |
| 98 | 1.00 | 98 | 1728 | README headline in crates/mdbook-driver/README.md |
| 96 | 0.67 | 144 | 8361 | mod/use plumbing in crates/mdbook-renderer/src/lib.rs |
| 96 | 1.00 | 96 | 3189 | mod/use plumbing in src/main.rs |
| 89 | 1.00 | 89 | 1516 | [package] in crates/mdbook-core/Cargo.toml |
| 88 | 0.73 | 121 | 6234 | impl method sigs in crates/mdbook-renderer/src/lib.rs |
| 86 | 1.00 | 86 | 2223 | [package] in crates/mdbook-preprocessor/Cargo.toml |
| 85 | 1.00 | 85 | 2377 | [package] in crates/mdbook-renderer/Cargo.toml |
| 84 | 1.00 | 84 | 8445 | pub-item names surface in crates/mdbook-html/src/utils.rs |
| 83 | 1.00 | 83 | 1611 | [package] in crates/mdbook-driver/Cargo.toml |
| 83 | 1.00 | 83 | 2067 | [package] in crates/mdbook-markdown/Cargo.toml |
| 81 | 1.00 | 81 | 2470 | [package] in crates/mdbook-summary/Cargo.toml |
| 81 | 1.00 | 81 | 3309 | headings outline in guide/src/continuous-integration.md |
| 80 | 1.00 | 80 | 1824 | [package] in crates/mdbook-html/Cargo.toml |
| 79 | 1.00 | 79 | 9622 | headings outline in guide/src/guide/creating.md |
| 77 | 1.00 | 77 | 9840 | guide/src/for_developers/README.md section #0 |
| 77 | 1.00 | 77 | 9543 | headings outline in guide/src/for_developers/backends.md |
| 73 | 1.00 | 73 | 964 | CONTRIBUTING.md section #0 |
| 71 | 1.00 | 71 | 2995 | pub-item names surface in src/cmd/clean.rs |
| 68 | 1.00 | 68 | 1892 | README headline in crates/mdbook-html/README.md |
| 68 | 1.00 | 68 | 7440 | README headline in guide/src/cli/README.md |
| 64 | 1.00 | 64 | 1325 | [package] in crates/mdbook-compare/Cargo.toml |
| 59 | 1.00 | 59 | 1427 | README headline in crates/mdbook-core/README.md |
| 59 | 1.00 | 59 | 7841 | headings outline in guide/src/guide/installation.md |
| 58 | 1.00 | 58 | 1984 | README headline in crates/mdbook-markdown/README.md |
| 58 | 1.00 | 58 | 2137 | README headline in crates/mdbook-preprocessor/README.md |
| 58 | 1.00 | 58 | 2851 | pub-item names surface in src/cmd/watch.rs |
| 57 | 1.00 | 57 | 2292 | README headline in crates/mdbook-renderer/README.md |
| 57 | 1.00 | 57 | 2527 | README headline in crates/mdbook-summary/README.md |
| 56 | 1.00 | 56 | 2924 | pub item at src/cmd/watch.rs:62 |
| 55 | 1.00 | 55 | 7653 | headings outline in guide/src/for_developers/preprocessors.md |
| 54 | 1.00 | 54 | 7175 | pub-item doc at crates/mdbook-summary/src/lib.rs:84 |
| 50 | 1.00 | 50 | 3359 | crates/mdbook-core/README.md section #0 |
| 50 | 1.00 | 50 | 3527 | crates/mdbook-html/README.md section #0 |
