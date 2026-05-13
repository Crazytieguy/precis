scores: Score(3000)=0.507 ns_rows≤3K=22/49 (reached=9 partial=2 missing=11)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 97 | 0.860 | 0.701 | 0.846 | 0.776 | 999 |
| 1442 | 137 | 0.815 | 0.540 | 0.863 | 0.663 | 1379 |
| 2080 | 218 | 0.769 | 0.367 | 0.877 | 0.531 | 2017 |
| 3000 | 274 | 0.773 | 0.332 | 0.897 | 0.507 | 2978 |
| 4327 | 389 | 0.777 | 0.447 | 0.865 | 0.590 | 4190 |
| 6240 | 540 | 0.751 | 0.333 | 0.884 | 0.500 | 6179 |
| 9000 | 765 | 0.752 | 0.385 | 0.859 | 0.538 | 8935 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 27 | 1.86 | 1.72 | 1.45 | nearby candidates have low exact atom overlap | 1.1, 2.2, 3.4, 3.9, 3.1, ... |
| tune ranking for high-overlap unscheduled candidates | 4 | 0.21 | 0.21 | 0.21 | high-overlap candidates not in the schedule by T_max, exact total=36/38 | 1.8, 3.3, 3.8, 4.2 |
| promote pub-item names surfaces | 2 | 0.14 | 0.14 | 0.14 | 1 file, exact total=65/70 | 3.12, 3.10 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| [package] in Cargo.toml | 1 | 176 | 176 | 176 | off_3k=176 | [package] in Cargo.toml |
| impl method sigs in crates/mdbook-renderer/src/lib.rs | 1 | 0 | 49 | 49 | off_3k=121 | impl method sigs in crates/mdbook-renderer/src/lib.rs |

Top missed paths (NS rows ≤ 3K): crates/mdbook-core/src/book.rs (3 rows, 48 atoms), crates/mdbook-driver/src/mdbook.rs (1 row, 32 atoms), crates/mdbook-html/src/html (1 row, 26 atoms), crates/mdbook-core/src/lib.rs (1 row, 16 atoms), crates/mdbook-driver/src (1 row, 14 atoms), src/main.rs (1 row, 12 atoms), src/cmd/mod.rs (1 row, 11 atoms), crates/mdbook-driver/src/lib.rs (1 row, 10 atoms), +3 more

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| group | 5119 | 0.00 | predecessor-gated | 2 children of `pub-item names surface in crates/mdbook-core/src/config.rs` | exact total=65/70; rows: 3.12, 3.10 |
| 1.8 | 741 | 0.08 | missing | CLI subcommand dispatch — match arms | [scheduled bbox exact=0/12] mod/use plumbing in src/main.rs (t=4817, 9 atoms); better unscheduled exact=11/12: entry item body at src/main.rs:18 body 24 (23 atoms, too expensive at final margin) |
| 3.3 | 2246 | 0.00 | missing | Book impl method names | [unscheduled bbox exact=7/7] impl method sigs in crates/mdbook-core/src/book.rs (19 atoms, too expensive at final margin) |
| 3.8 | 4133 | 0.00 | missing | Config method names + Config::set | [unscheduled bbox exact=8/8] impl method sigs in crates/mdbook-core/src/config.rs (15 atoms, too expensive at final margin) |
| 4.2 | 7841 | 0.00 | missing | LinkPreprocessor — supported helpers list | [unscheduled bbox exact=10/11] pub-item doc lede at crates/mdbook-driver/src/builtin_preprocessors/links.rs:32 (10 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.1 | 75 | 0.80 | partial | README lede | [scheduled bbox exact=2/5] README.md section #0 (t=333, 2 atoms) |
| 1.5 | 341 | 0.73 | partial | src/cmd module map | [scheduled bbox exact=7/11] mod/use plumbing in src/cmd/mod.rs (t=564, 7 atoms) |
| 2.2 | 1047 | 0.00 | missing | mdbook-core lib + module map | [scheduled bbox exact=5/16] mod/use plumbing in crates/mdbook-core/src/lib.rs (t=3834, 5 atoms) |
| 2.7 | 1496 | 0.00 | missing | mdbook-html crate lib + module map | [scheduled bbox exact=5/8] mod/use plumbing in crates/mdbook-html/src/lib.rs (t=4190, 5 atoms) |
| 3.1 | 1960 | 0.00 | missing | Book / BookItem struct shapes | [scheduled bbox exact=8/24] pub item at crates/mdbook-core/src/book.rs:119 (t=9628, 8 atoms) |
| 3.2 | 2164 | 0.00 | missing | Chapter struct fields | [scheduled bbox exact=2/17] pub-item names surface in crates/mdbook-core/src/book.rs (t=9525, 2 atoms); better unscheduled exact=10/17: pub item at crates/mdbook-core/src/book.rs:141 (29 atoms, too expensive at final margin) |
| 3.4 | 2625 | 0.00 | missing | MDBook struct + public method names | [scheduled bbox exact=2/32] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=7223, 2 atoms); better unscheduled exact=15/32: impl method sigs in crates/mdbook-driver/src/mdbook.rs (34 atoms, too expensive at final margin) |
| 3.5 | 3208 | 0.71 | partial | Preprocessor trait + PreprocessorContext | [scheduled bbox exact=16/45] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2911, 16 atoms) |
| 3.6 | 3742 | 0.20 | missing | Renderer trait + RenderContext | [scheduled bbox exact=22/40] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3503, 22 atoms) |
| 3.7 | 4011 | 0.00 | missing | Config struct (top-level book.toml shape) | [unscheduled bbox exact=16/22] pub item at crates/mdbook-core/src/config.rs:63 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.9 | 4753 | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields | [unscheduled bbox exact=16/50] pub item at crates/mdbook-core/src/config.rs:318 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.11 | 5489 | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines | [unscheduled bbox exact=13/31] pub item at crates/mdbook-core/src/config.rs:611 (13 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.14 | 6851 | 0.81 | partial | Summary / SummaryItem / Link types + parse_summary | [scheduled bbox exact=11/42] pub item at crates/mdbook-summary/src/lib.rs:84 (t=2561, 11 atoms) |
| 3.15 | 6973 | 0.33 | missing | Preprocessor input parsing + MDBOOK_VERSION re-export | [scheduled bbox exact=0/9] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2911, 16 atoms) |
| 3.16 | 7060 | 0.00 | missing | RenderContext impl methods (incl. from_json) | [scheduled bbox exact=0/7] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3503, 22 atoms) |
| 3.17 | 7552 | 0.00 | missing | MDBOOK_* env-var override rules | [unscheduled same-file] pub item at crates/mdbook-core/src/config.rs:449 (73 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 4.1 | 7635 | 0.00 | missing | Builtin preprocessors module — re-exports | [scheduled bbox exact=6/9] mod/use plumbing in crates/mdbook-driver/src/builtin_preprocessors/mod.rs (t=7558, 6 atoms) |
| 4.3 | 8077 | 0.00 | missing | MDBook::load body — book.toml + Config wiring | [unscheduled bbox exact=2/22] impl method sigs in crates/mdbook-driver/src/mdbook.rs (2 atoms, too expensive at final margin) |
| 4.4 | 8287 | 0.00 | missing | load.rs — load_book entrypoint | [unscheduled bbox exact=9/15] pub item body at crates/mdbook-driver/src/load.rs:10 body 11 (9 atoms, predecessor not scheduled: pub item at crates/mdbook-driver/src/load.rs:10) |
| 4.5 | 8522 | 0.00 | missing | execute_build_process — the build pipeline | [unscheduled bbox exact=2/22] impl method sigs in crates/mdbook-driver/src/mdbook.rs (2 atoms, too expensive at final margin) |
| 4.6 | 8610 | 0.00 | missing | Default preprocessors + topological order | [scheduled same-file] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=7223, 2 atoms) |
| 4.7 | 8735 | 0.00 | missing | BookBuilder type — mdbook init API | [scheduled bbox exact=6/12] pub item at crates/mdbook-driver/src/init.rs:13 (t=7821, 6 atoms) |
| 5.1 | 8977 | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline | [scheduled bbox exact=11/16] crate-doc lede in crates/mdbook-html/src/html/mod.rs (t=4368, 11 atoms) |
| 5.2 | 9440 | 0.00 | missing | Theme — bundled-asset names | [scheduled same-file] pub item at crates/mdbook-html/src/theme/mod.rs:40 (t=5372, 22 atoms) |
| 6.1 | 9515 | 0.15 | missing | build subcommand — clap surface | [scheduled bbox exact=2/7] pub item at src/cmd/build.rs:8 (t=402, 2 atoms); better unscheduled exact=5/7: pub item body at src/cmd/build.rs:8 body 9 (5 atoms, too expensive at final margin) |
| 6.2 | 9597 | 0.25 | missing | command_prelude shared args | [scheduled bbox exact=3/8] pub-item names surface in src/cmd/command_prelude.rs (t=602, 3 atoms); better unscheduled exact=6/8: pub item at src/cmd/command_prelude.rs:7 (44 atoms, too expensive at final margin) |
| 6.3 | 9801 | 0.00 | missing | tests/testsuite/main.rs — test module map | [unscheduled bbox exact=18/25] mod/use plumbing in tests/testsuite/main.rs (18 atoms, predecessor not scheduled: listing of 'tests/testsuite') |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.8 | 1555 | 0.00 | missing | mdbook-driver source layout | fs-only |
| 2.9 | 1591 | 0.00 | missing | mdbook-core source layout | fs-only |
| 2.10 | 1701 | 0.00 | missing | mdbook-html source layout | fs-only |
| 6.4 | 9848 | 0.30 | missing | examples/ + guide/ tree | fs-only |
| 6.5 | 9944 | 0.00 | missing | guide/src tree — doc-source files | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.1 | 882 | 0.00 | missing | mdbook-driver crate role | [scheduled bbox exact=10/10] crate-doc lede in crates/mdbook-driver/src/lib.rs (t=7370, 10 atoms) |

Top wasted paths (off-NS at 3K): crates/mdbook-preprocessor/src/lib.rs (420t, 3 batches), crates/mdbook-summary/src/lib.rs (382t, 4 batches), crates/mdbook-renderer/src/lib.rs (202t, 2 batches), Cargo.toml (176t, 1 batch), crates/mdbook-markdown/src/lib.rs (176t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 2 | 369 | pub item at crates/mdbook-preprocessor/src/lib.rs:<n> |
| 3 | 330 | pub item at crates/mdbook-summary/src/lib.rs:<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 191 | 1.00 | 0 | 191 | 2720 | pub item at crates/mdbook-preprocessor/src/lib.rs:51 |
| 178 | 1.00 | 0 | 178 | 1802 | pub item at crates/mdbook-preprocessor/src/lib.rs:30 |
| 176 | 0.50 | 176 | 349 | 602 | [package] in Cargo.toml |
| 176 | 1.00 | 0 | 176 | 1379 | pub item at crates/mdbook-markdown/src/lib.rs:15 |
| 139 | 1.00 | 0 | 139 | 2561 | pub item at crates/mdbook-summary/src/lib.rs:67 |
| 130 | 1.00 | 0 | 130 | 2431 | pub item at crates/mdbook-summary/src/lib.rs:84 |
| 121 | 1.00 | 49 | 121 | 2978 | impl method sigs in crates/mdbook-renderer/src/lib.rs |
| 81 | 1.00 | 0 | 81 | 2017 | pub item at crates/mdbook-renderer/src/lib.rs:28 |
| 61 | 1.00 | 0 | 61 | 2264 | pub item at crates/mdbook-summary/src/lib.rs:122 |
| 52 | 1.00 | 0 | 52 | 2212 | pub-item names surface in crates/mdbook-summary/src/lib.rs |
| 51 | — | — | — | — | +1 more row |
