scores: Sim=0.420 Reached=19/49 Early=3 Late=10 Partial=8 Missing=22 Used=9993/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 6 ranking-recoverable (w×gap=1.23), 21 wrong-slice/granularity (w×gap=1.71), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 2 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 3.2, 3.4, 1.5, 2.2, 3.1, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 21 | 1.71 | 6/9/21 | nearby candidates have low exact atom overlap | 3.2, 3.4, 1.5, 2.2, 3.1, ... |
| free final budget / demote late waste | 4 | 1.10 | 2/3/4 | high-overlap candidates exceed final remaining budget, exact total=36/38 | 1.8, 3.3, 3.8, 4.2 |
| promote pub-item names surfaces | 2 | 0.13 | 0/1/2 | 1 file, exact total=65/70 | 3.10, 3.12 |

Tiers: 1=6/8 reached, 1 partial, 1 missing, avg=0.82; 2=7/10 reached, 2 partial, 1 missing, avg=0.86; 3=5/17 reached, 2 partial, 10 missing, avg=0.35; 4=0/7 reached, 2 partial, 5 missing, avg=0.19; 5=0/2 reached, 0 partial, 2 missing, avg=0.00; 6=1/5 reached, 1 partial, 3 missing, avg=0.40

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 6 | 6 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 21 | 14 | 7 | 0 | walker granularity / wrong slice |
| fs/listing | 3 | 2 | 1 | 0 | filesystem/listing value |
| timing-only | 18 | 0 | 0 | 18 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.13 | promote predecessor |
| too expensive at final margin | 4 | 1.10 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=25, unscheduled bbox=12, scheduled same-file=2, unscheduled same-file=1, fs-only=8

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | none | 1 |
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 2 |
| scheduled bbox | late | full | 4 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 6 |
| unscheduled bbox | missing | low | 7 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| group | 5119 | — | — | 0.00 | predecessor-gated | 2 children of `pub-item names surface in crates/mdbook-core/src/config.rs` | exact total=65/70; rows: 3.10, 3.12 |
| 1.8 | 741 | — | — | 0.08 | missing | CLI subcommand dispatch — match arms | [scheduled bbox exact=0/12] mod/use plumbing in src/main.rs (t=3745, 9 atoms); better unscheduled exact=11/12: entry item body at src/main.rs:18 body 24 (23 atoms, too expensive at final margin) |
| 3.3 | 2246 | — | — | 0.00 | missing | Book impl method names | [unscheduled bbox exact=7/7] impl method sigs in crates/mdbook-core/src/book.rs (19 atoms, too expensive at final margin) |
| 3.8 | 4133 | — | — | 0.00 | missing | Config method names + Config::set | [unscheduled bbox exact=8/8] impl method sigs in crates/mdbook-core/src/config.rs (15 atoms, too expensive at final margin) |
| 4.2 | 7841 | — | — | 0.00 | missing | LinkPreprocessor — supported helpers list | [unscheduled bbox exact=10/11] pub-item doc lede at crates/mdbook-driver/src/builtin_preprocessors/links.rs:32 (10 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.5 | 341 | — | — | 0.73 | partial | src/cmd module map | [scheduled bbox exact=7/11] mod/use plumbing in src/cmd/mod.rs (t=6737, 7 atoms) |
| 2.2 | 1047 | — | — | 0.69 | partial | mdbook-core lib + module map | [scheduled bbox exact=5/16] mod/use plumbing in crates/mdbook-core/src/lib.rs (t=3457, 5 atoms) |
| 2.7 | 1496 | — | — | 0.75 | partial | mdbook-html crate lib + module map | [scheduled bbox exact=5/8] mod/use plumbing in crates/mdbook-html/src/lib.rs (t=3538, 5 atoms) |
| 3.1 | 1960 | — | — | 0.54 | partial | Book / BookItem struct shapes | [scheduled bbox exact=8/24] pub item at crates/mdbook-core/src/book.rs:119 (t=7897, 8 atoms) |
| 3.2 | 2164 | — | — | 0.06 | missing | Chapter struct fields | [scheduled bbox exact=2/17] pub-item names surface in crates/mdbook-core/src/book.rs (t=7794, 2 atoms); better unscheduled exact=10/17: pub item at crates/mdbook-core/src/book.rs:141 (29 atoms, too expensive at final margin) |
| 3.4 | 2625 | — | — | 0.03 | missing | MDBook struct + public method names | [scheduled bbox exact=2/32] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=5207, 2 atoms); better unscheduled exact=15/32: impl method sigs in crates/mdbook-driver/src/mdbook.rs (34 atoms, too expensive at final margin) |
| 3.7 | 4011 | — | — | 0.00 | missing | Config struct (top-level book.toml shape) | [unscheduled bbox exact=16/22] pub item at crates/mdbook-core/src/config.rs:63 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.9 | 4753 | — | — | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields | [unscheduled bbox exact=16/50] pub item at crates/mdbook-core/src/config.rs:318 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.11 | 5489 | — | — | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines | [unscheduled bbox exact=13/31] pub item at crates/mdbook-core/src/config.rs:611 (13 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.15 | 6973 | — | — | 0.78 | partial | Preprocessor input parsing + MDBOOK_VERSION re-export | [scheduled bbox exact=0/9] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2583, 16 atoms) |
| 3.17 | 7552 | — | — | 0.00 | missing | MDBOOK_* env-var override rules | [unscheduled same-file] pub item at crates/mdbook-core/src/config.rs:449 (73 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 4.1 | 7635 | — | — | 0.78 | partial | Builtin preprocessors module — re-exports | [scheduled bbox exact=6/9] mod/use plumbing in crates/mdbook-driver/src/builtin_preprocessors/mod.rs (t=9042, 6 atoms) |
| 4.3 | 8077 | — | — | 0.00 | missing | MDBook::load body — book.toml + Config wiring | [unscheduled bbox exact=2/22] impl method sigs in crates/mdbook-driver/src/mdbook.rs (2 atoms, too expensive at final margin) |
| 4.4 | 8287 | — | — | 0.07 | missing | load.rs — load_book entrypoint | [scheduled bbox exact=2/15] pub item at crates/mdbook-driver/src/load.rs:10 (t=9563, 2 atoms); better unscheduled exact=9/15: pub item body at crates/mdbook-driver/src/load.rs:10 body 11 (9 atoms, too expensive at final margin) |
| 4.5 | 8522 | — | — | 0.00 | missing | execute_build_process — the build pipeline | [unscheduled bbox exact=2/22] impl method sigs in crates/mdbook-driver/src/mdbook.rs (2 atoms, too expensive at final margin) |
| 4.6 | 8610 | — | — | 0.00 | missing | Default preprocessors + topological order | [scheduled same-file] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=5207, 2 atoms) |
| 4.7 | 8735 | — | — | 0.50 | partial | BookBuilder type — mdbook init API | [scheduled bbox exact=6/12] pub item at crates/mdbook-driver/src/init.rs:13 (t=5787, 6 atoms) |
| 5.1 | 8977 | — | — | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline | [unscheduled bbox exact=11/16] crate-doc lede in crates/mdbook-html/src/html/mod.rs (11 atoms, predecessor not scheduled: listing of 'crates/mdbook-html/src/html') |
| 5.2 | 9440 | — | — | 0.00 | missing | Theme — bundled-asset names | [scheduled same-file] pub item at crates/mdbook-html/src/theme/mod.rs:40 (t=8716, 22 atoms) |
| 6.2 | 9597 | — | — | 0.25 | missing | command_prelude shared args | [scheduled bbox exact=3/8] pub-item names surface in src/cmd/command_prelude.rs (t=6775, 3 atoms); better unscheduled exact=6/8: pub item at src/cmd/command_prelude.rs:7 (44 atoms, too expensive at final margin) |
| 6.3 | 9801 | — | — | 0.00 | missing | tests/testsuite/main.rs — test module map | [unscheduled bbox exact=18/25] mod/use plumbing in tests/testsuite/main.rs (18 atoms, predecessor not scheduled: Fs(DirListing { dir: "/Users/yoav/projects/precis/tests/fixtures/mdbook/tests/testsuite" })) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.10 | 1701 | — | — | 0.35 | missing | mdbook-html source layout | fs-only |
| 6.4 | 9848 | — | — | 0.30 | missing | examples/ + guide/ tree | fs-only |
| 6.5 | 9944 | — | — | 0.59 | partial | guide/src tree — doc-source files | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 246 | +171 | 0.80 | late | README lede | [scheduled bbox exact=2/5] README.md section #0 (t=246, 2 atoms) |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Repo root listing | fs-only |
| 1.3 | 202 | 671 | +469 | 1.00 | late | crates/ directory listing | fs-only |
| 1.4 | 239 | 623 | +384 | 1.00 | late | Workspace package summary — name + description | [scheduled bbox exact=3/3] [package] in Cargo.toml (t=623, 15 atoms) |
| 1.6 | 482 | 623 | +141 | 0.92 | aligned | Workspace members + edition | [scheduled bbox exact=11/12] [package] in Cargo.toml (t=623, 11 atoms) |
| 1.7 | 536 | 6492 | +5956 | 1.00 | late | src/ tree (top-level bin sources) | fs-only |
| 2.1 | 882 | 5354 | +4472 | 1.00 | late | mdbook-driver crate role | [scheduled bbox exact=10/10] crate-doc lede in crates/mdbook-driver/src/lib.rs (t=5354, 10 atoms) |
| 2.3 | 1121 | 2103 | +982 | 1.00 | late | mdbook-summary crate lede | [scheduled bbox exact=5/5] crate-doc lede in crates/mdbook-summary/src/lib.rs (t=2103, 5 atoms) |
| 2.4 | 1229 | 1032 | -197 | 0.86 | aligned | mdbook-markdown crate lede | [scheduled bbox exact=6/7] crate-doc lede in crates/mdbook-markdown/src/lib.rs (t=1032, 6 atoms) |
| 2.6 | 1428 | 1868 | +440 | 1.00 | late | mdbook-renderer crate lede | [scheduled bbox exact=6/6] crate-doc lede in crates/mdbook-renderer/src/lib.rs (t=1868, 6 atoms) |
| 2.8 | 1555 | 8933 | +7378 | 1.00 | late | mdbook-driver source layout | fs-only |
| 2.9 | 1591 | 7915 | +6324 | 1.00 | late | mdbook-core source layout | fs-only |
| 3.5 | 3208 | 7440 | +4232 | 0.87 | late | Preprocessor trait + PreprocessorContext | [scheduled bbox exact=16/45] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2583, 16 atoms) |
| 3.6 | 3742 | 4773 | +1031 | 0.90 | aligned | Renderer trait + RenderContext | [scheduled bbox exact=22/40] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3175, 22 atoms) |
| 3.13 | 6324 | 1227 | -5097 | 0.95 | early | MarkdownOptions + new_cmark_parser | [scheduled bbox exact=17/21] pub item at crates/mdbook-markdown/src/lib.rs:15 (t=1227, 17 atoms) |
| 3.14 | 6851 | 2392 | -4459 | 0.81 | early | Summary / SummaryItem / Link types + parse_summary | [scheduled bbox exact=11/42] pub item at crates/mdbook-summary/src/lib.rs:84 (t=2233, 11 atoms) |
| 3.16 | 7060 | 5154 | -1906 | 1.00 | aligned+over | RenderContext impl methods (incl. from_json) | [scheduled bbox exact=0/7] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3175, 22 atoms) |
| 6.1 | 9515 | 9449 | -66 | 0.86 | aligned | build subcommand — clap surface | [scheduled bbox exact=5/7] pub item body at src/cmd/build.rs:8 body 9 (t=9449, 5 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 306 | 1.00 | 306 | 8716 | pub item at crates/mdbook-html/src/theme/mod.rs:40 |
| 257 | 1.00 | 257 | 6454 | [features] in Cargo.toml |
| 192 | 1.00 | 192 | 7676 | entry item at examples/remove-emphasis/mdbook-remove-emphasis/src/main.rs:49 |
| 183 | 1.00 | 183 | 7325 | entry item at examples/remove-emphasis/mdbook-remove-emphasis/src/main.rs:10 |
| 176 | 0.50 | 349 | 623 | [package] in Cargo.toml |
| 159 | 1.00 | 159 | 4270 | headings outline in CONTRIBUTING.md |
| 145 | 1.00 | 145 | 8281 | macro_export body at crates/mdbook-core/src/utils/mod.rs:17 |
| 137 | 1.00 | 137 | 4910 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 131 | 1.00 | 131 | 5744 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 111 | 1.00 | 111 | 9247 | README headline in guide/src/for_developers/README.md |
| 2104 | — | — | — | +29 more rows |
