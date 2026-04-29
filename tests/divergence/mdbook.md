scores: Score(3000)=0.507 ns_rows≤3K=22/49 (reached=9 partial=2 missing=11)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 97 | 0.860 | 0.701 | 0.776 | 992 |
| 1442 | 137 | 0.815 | 0.540 | 0.663 | 1372 |
| 2080 | 218 | 0.769 | 0.367 | 0.531 | 2010 |
| 3000 | 274 | 0.773 | 0.332 | 0.507 | 2971 |
| 4327 | 389 | 0.777 | 0.447 | 0.590 | 4183 |
| 6240 | 540 | 0.750 | 0.333 | 0.500 | 6161 |
| 9000 | 765 | 0.751 | 0.385 | 0.538 | 8917 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 6 ranking-recoverable (gap@3k=0.36), 27 wrong-slice/granularity (gap@3k=1.72), 0 no-discovered (gap@3k=0.00)
Secondary intervention: promote predecessors for 2 gated candidates
Top rows: 1.1, 2.2, 3.4, 3.9, 3.1, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 27 | 1.86 | 1.72 | 1.45 | nearby candidates have low exact atom overlap | 1.1, 2.2, 3.4, 3.9, 3.1, ... |
| tune ranking for high-overlap unscheduled candidates | 4 | 0.21 | 0.21 | 0.21 | high-overlap candidates not in the schedule by T_max, exact total=36/38 | 1.8, 3.3, 3.8, 4.2 |
| promote pub-item names surfaces | 2 | 0.14 | 0.14 | 0.14 | 1 file, exact total=65/70 | 3.12, 3.10 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 6 | 6 | 0 | value/ranking |
| wrong-slice / granularity | 27 | 23 | 4 | walker granularity / wrong slice |
| fs/listing | 5 | 5 | 0 | filesystem/listing value |
| mixed/unknown | 1 | 1 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 2 | 0.14 | promote predecessor |
| too expensive at final margin | 4 | 0.21 | tune ranking |

Candidate hint kinds: scheduled bbox=19, unscheduled bbox=12, scheduled same-file=2, unscheduled same-file=1, fs-only=5 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 3 |
| scheduled bbox | missing | low | 11 |
| scheduled bbox | missing | full | 1 |
| scheduled bbox | partial | low | 4 |
| unscheduled bbox | missing | low | 7 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| group | 5119 | 0.00 | 0.00 | predecessor-gated | 2 children of `pub-item names surface in crates/mdbook-core/src/config.rs` | exact total=65/70; rows: 3.12, 3.10 |
| 1.8 | 741 | 0.08 | 0.03 | missing | CLI subcommand dispatch — match arms | [scheduled bbox exact=0/12] mod/use plumbing in src/main.rs (t=4810, 9 atoms); better unscheduled exact=11/12: entry item body at src/main.rs:18 body 24 (23 atoms, too expensive at final margin) |
| 3.3 | 2246 | 0.00 | 0.00 | missing | Book impl method names | [unscheduled bbox exact=7/7] impl method sigs in crates/mdbook-core/src/book.rs (19 atoms, too expensive at final margin) |
| 3.8 | 4133 | 0.00 | 0.00 | missing | Config method names + Config::set | [unscheduled bbox exact=8/8] impl method sigs in crates/mdbook-core/src/config.rs (15 atoms, too expensive at final margin) |
| 4.2 | 7841 | 0.00 | 0.00 | missing | LinkPreprocessor — supported helpers list | [unscheduled bbox exact=10/11] pub-item doc lede at crates/mdbook-driver/src/builtin_preprocessors/links.rs:32 (10 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 0.80 | 1.00 | partial | README lede | [scheduled bbox exact=2/5] README.md section #0 (t=300, 2 atoms) |
| 1.5 | 341 | 0.73 | 0.75 | partial | src/cmd module map | [scheduled bbox exact=7/11] mod/use plumbing in src/cmd/mod.rs (t=557, 7 atoms) |
| 2.2 | 1047 | 0.00 | 0.00 | missing | mdbook-core lib + module map | [scheduled bbox exact=5/16] mod/use plumbing in crates/mdbook-core/src/lib.rs (t=3823, 5 atoms) |
| 2.7 | 1496 | 0.00 | 0.00 | missing | mdbook-html crate lib + module map | [scheduled bbox exact=5/8] mod/use plumbing in crates/mdbook-html/src/lib.rs (t=4183, 5 atoms) |
| 3.1 | 1960 | 0.00 | 0.00 | missing | Book / BookItem struct shapes | [scheduled bbox exact=8/24] pub item at crates/mdbook-core/src/book.rs:119 (t=9610, 8 atoms) |
| 3.2 | 2164 | 0.00 | 0.00 | missing | Chapter struct fields | [scheduled bbox exact=2/17] pub-item names surface in crates/mdbook-core/src/book.rs (t=9507, 2 atoms); better unscheduled exact=10/17: pub item at crates/mdbook-core/src/book.rs:141 (29 atoms, too expensive at final margin) |
| 3.4 | 2625 | 0.00 | 0.00 | missing | MDBook struct + public method names | [scheduled bbox exact=2/32] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=7196, 2 atoms); better unscheduled exact=15/32: impl method sigs in crates/mdbook-driver/src/mdbook.rs (34 atoms, too expensive at final margin) |
| 3.5 | 3208 | 0.71 | 0.72 | partial | Preprocessor trait + PreprocessorContext | [scheduled bbox exact=16/45] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2904, 16 atoms) |
| 3.6 | 3742 | 0.20 | 0.17 | missing | Renderer trait + RenderContext | [scheduled bbox exact=22/40] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3496, 22 atoms) |
| 3.7 | 4011 | 0.00 | 0.00 | missing | Config struct (top-level book.toml shape) | [unscheduled bbox exact=16/22] pub item at crates/mdbook-core/src/config.rs:63 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.9 | 4753 | 0.00 | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields | [unscheduled bbox exact=16/50] pub item at crates/mdbook-core/src/config.rs:318 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.11 | 5489 | 0.00 | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines | [unscheduled bbox exact=13/31] pub item at crates/mdbook-core/src/config.rs:611 (13 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.14 | 6851 | 0.81 | 0.84 | partial | Summary / SummaryItem / Link types + parse_summary | [scheduled bbox exact=11/42] pub item at crates/mdbook-summary/src/lib.rs:84 (t=2554, 11 atoms) |
| 3.15 | 6973 | 0.33 | 0.63 | missing | Preprocessor input parsing + MDBOOK_VERSION re-export | [scheduled bbox exact=0/9] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2904, 16 atoms) |
| 3.16 | 7060 | 0.00 | 0.00 | missing | RenderContext impl methods (incl. from_json) | [scheduled bbox exact=0/7] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3496, 22 atoms) |
| 3.17 | 7552 | 0.00 | 0.00 | missing | MDBOOK_* env-var override rules | [unscheduled same-file] pub item at crates/mdbook-core/src/config.rs:449 (73 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 4.1 | 7635 | 0.00 | 0.00 | missing | Builtin preprocessors module — re-exports | [scheduled bbox exact=6/9] mod/use plumbing in crates/mdbook-driver/src/builtin_preprocessors/mod.rs (t=7535, 6 atoms) |
| 4.3 | 8077 | 0.00 | 0.00 | missing | MDBook::load body — book.toml + Config wiring | [unscheduled bbox exact=2/22] impl method sigs in crates/mdbook-driver/src/mdbook.rs (2 atoms, too expensive at final margin) |
| 4.4 | 8287 | 0.00 | 0.00 | missing | load.rs — load_book entrypoint | [unscheduled bbox exact=9/15] pub item body at crates/mdbook-driver/src/load.rs:10 body 11 (9 atoms, predecessor not scheduled: pub item at crates/mdbook-driver/src/load.rs:10) |
| 4.5 | 8522 | 0.00 | 0.00 | missing | execute_build_process — the build pipeline | [unscheduled bbox exact=2/22] impl method sigs in crates/mdbook-driver/src/mdbook.rs (2 atoms, too expensive at final margin) |
| 4.6 | 8610 | 0.00 | 0.00 | missing | Default preprocessors + topological order | [scheduled same-file] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=7196, 2 atoms) |
| 4.7 | 8735 | 0.00 | 0.00 | missing | BookBuilder type — mdbook init API | [scheduled bbox exact=6/12] pub item at crates/mdbook-driver/src/init.rs:13 (t=7803, 6 atoms) |
| 5.1 | 8977 | 0.00 | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline | [scheduled bbox exact=11/16] crate-doc lede in crates/mdbook-html/src/html/mod.rs (t=4361, 11 atoms) |
| 5.2 | 9440 | 0.00 | 0.00 | missing | Theme — bundled-asset names | [scheduled same-file] pub item at crates/mdbook-html/src/theme/mod.rs:40 (t=5354, 22 atoms) |
| 6.1 | 9515 | 0.15 | 0.21 | missing | build subcommand — clap surface | [scheduled bbox exact=2/7] pub item at src/cmd/build.rs:8 (t=395, 2 atoms); better unscheduled exact=5/7: pub item body at src/cmd/build.rs:8 body 9 (5 atoms, too expensive at final margin) |
| 6.2 | 9597 | 0.25 | 0.31 | missing | command_prelude shared args | [scheduled bbox exact=3/8] pub-item names surface in src/cmd/command_prelude.rs (t=595, 3 atoms); better unscheduled exact=6/8: pub item at src/cmd/command_prelude.rs:7 (44 atoms, too expensive at final margin) |
| 6.3 | 9801 | 0.00 | 0.00 | missing | tests/testsuite/main.rs — test module map | [unscheduled bbox exact=18/25] mod/use plumbing in tests/testsuite/main.rs (18 atoms, predecessor not scheduled: Fs(DirListing { dir: "/Users/yoav/projects/precis/tests/fixtures/mdbook/tests/testsuite" })) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.8 | 1555 | 0.00 | 0.00 | missing | mdbook-driver source layout | fs-only |
| 2.9 | 1591 | 0.00 | 0.00 | missing | mdbook-core source layout | fs-only |
| 2.10 | 1701 | 0.00 | 0.00 | missing | mdbook-html source layout | fs-only |
| 6.4 | 9848 | 0.30 | 0.30 | missing | examples/ + guide/ tree | fs-only |
| 6.5 | 9944 | 0.00 | 0.00 | missing | guide/src tree — doc-source files | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.1 | 882 | 0.00 | 0.00 | missing | mdbook-driver crate role | [scheduled bbox exact=10/10] crate-doc lede in crates/mdbook-driver/src/lib.rs (t=7343, 10 atoms) |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 176 | 0.50 | 349 | 944 | [package] in Cargo.toml |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 306 | 1.00 | 306 | 5354 | pub item at crates/mdbook-html/src/theme/mod.rs:40 |
| 257 | 1.00 | 257 | 8560 | [features] in Cargo.toml |
| 192 | 1.00 | 192 | 9268 | entry item at examples/remove-emphasis/mdbook-remove-emphasis/src/main.rs:49 |
| 183 | 1.00 | 183 | 8917 | entry item at examples/remove-emphasis/mdbook-remove-emphasis/src/main.rs:10 |
| 159 | 1.00 | 159 | 5874 | headings outline in CONTRIBUTING.md |
| 145 | 1.00 | 145 | 5668 | macro_export body at crates/mdbook-core/src/utils/mod.rs:17 |
| 137 | 1.00 | 137 | 6667 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 131 | 1.00 | 131 | 7760 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 111 | 1.00 | 111 | 9881 | README headline in guide/src/for_developers/README.md |
| 106 | 0.68 | 156 | 8303 | mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs |
| 2154 | — | — | — | +30 more rows |
