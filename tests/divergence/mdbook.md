scores: Sim=0.428 Reached=17/49 Early=2 Late=9 Partial=8 Missing=24 Used=9803/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 3 ranking-recoverable (w×gap=0.15), 25 wrong-slice/granularity (w×gap=2.82), 2 no-discovered (w×gap=0.02)
Secondary intervention: free final budget for 1 too-expensive candidate
Loss reasons: 2 predecessor-gated, 1 too-expensive, 0 discovered-unscheduled
Top rows: 1.8, 3.3, 3.2, 3.4, 1.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 25 | 2.82 | 8/12/25 | nearby candidates have low exact atom overlap | 1.8, 3.3, 3.2, 3.4, 1.5, ... |
| promote pub-item names surfaces | 2 | 0.13 | 0/1/2 | 1 file, exact total=65/70 | 3.10, 3.12 |
| free final budget / demote late waste | 1 | 0.02 | 0/0/1 | high-overlap candidates exceed final remaining budget, exact total=10/11 | 4.2 |
| add walker candidates for no-discovered rows | 2 | 0.02 | 0/0/2 | NS rows have no discovered line candidate | 5.1, 6.3 |

Tiers: 1=6/8 reached, 1 partial, 1 missing, avg=0.82; 2=7/10 reached, 3 partial, 0 missing, avg=0.88; 3=3/17 reached, 2 partial, 12 missing, avg=0.28; 4=0/7 reached, 2 partial, 5 missing, avg=0.20; 5=0/2 reached, 0 partial, 2 missing, avg=0.00; 6=1/5 reached, 0 partial, 4 missing, avg=0.34

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 3 | 3 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 25 | 18 | 7 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 1 | 1 | 0 | filesystem/listing value |
| timing-only | 14 | 0 | 0 | 14 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.13 | promote predecessor |
| too expensive at final margin | 1 | 0.02 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=24, unscheduled bbox=6, scheduled same-file=5, unscheduled same-file=2, fs-only=7, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 2 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | none | 3 |
| scheduled bbox | missing | low | 5 |
| scheduled bbox | partial | low | 7 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| group | 5119 | — | — | 0.00 | predecessor-gated | 2 children of `pub-item names surface in crates/mdbook-core/src/config.rs` | exact total=65/70; rows: 3.10, 3.12 |
| 4.2 | 7841 | — | — | 0.00 | missing | LinkPreprocessor — supported helpers list | [unscheduled bbox exact=10/11] pub-item doc lede at crates/mdbook-driver/src/builtin_preprocessors/links.rs:32 (10 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.5 | 341 | — | — | 0.73 | partial | src/cmd module map | [scheduled bbox exact=7/11] mod/use plumbing in src/cmd/mod.rs (t=6462, 7 atoms) |
| 1.8 | 741 | — | — | 0.08 | missing | CLI subcommand dispatch — match arms | [scheduled bbox exact=0/12] mod/use plumbing in src/main.rs (t=6558, 9 atoms) |
| 2.2 | 1047 | — | — | 0.69 | partial | mdbook-core lib + module map | [scheduled bbox exact=5/16] mod/use plumbing in crates/mdbook-core/src/lib.rs (t=3392, 5 atoms) |
| 2.7 | 1496 | — | — | 0.75 | partial | mdbook-html crate lib + module map | [scheduled bbox exact=5/8] mod/use plumbing in crates/mdbook-html/src/lib.rs (t=3621, 5 atoms) |
| 3.1 | 1960 | — | — | 0.54 | partial | Book / BookItem struct shapes | [scheduled bbox exact=8/24] pub item at crates/mdbook-core/src/book.rs:119 (t=6737, 8 atoms) |
| 3.2 | 2164 | — | — | 0.06 | missing | Chapter struct fields | [scheduled bbox exact=2/17] pub-item names surface in crates/mdbook-core/src/book.rs (t=6634, 2 atoms); better unscheduled exact=10/17: pub item at crates/mdbook-core/src/book.rs:141 (29 atoms, too expensive at final margin) |
| 3.3 | 2246 | — | — | 0.00 | missing | Book impl method names | [scheduled same-file] pub-item names surface in crates/mdbook-core/src/book.rs (t=6634, 10 atoms) |
| 3.4 | 2625 | — | — | 0.03 | missing | MDBook struct + public method names | [scheduled bbox exact=2/32] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=4754, 2 atoms); better unscheduled exact=12/32: pub item at crates/mdbook-driver/src/mdbook.rs:29 (12 atoms, discovered unscheduled) |
| 3.7 | 4011 | — | — | 0.00 | missing | Config struct (top-level book.toml shape) | [unscheduled bbox exact=16/22] pub item at crates/mdbook-core/src/config.rs:63 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.8 | 4133 | — | — | 0.00 | missing | Config method names + Config::set | [unscheduled same-file] pub item at crates/mdbook-core/src/config.rs:449 (73 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.9 | 4753 | — | — | 0.00 | missing | BookConfig + BuildConfig + RustConfig + RustEdition fields | [unscheduled bbox exact=16/50] pub item at crates/mdbook-core/src/config.rs:318 (16 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.11 | 5489 | — | — | 0.00 | missing | HTML config sub-tables — Print / Fold / Playground / Code field lines | [unscheduled bbox exact=13/31] pub item at crates/mdbook-core/src/config.rs:611 (13 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 3.14 | 6851 | — | — | 0.76 | partial | Summary / SummaryItem / Link types + parse_summary | [scheduled bbox exact=11/42] pub item at crates/mdbook-summary/src/lib.rs:84 (t=2137, 11 atoms) |
| 3.15 | 6973 | — | — | 0.22 | missing | Preprocessor input parsing + MDBOOK_VERSION re-export | [scheduled bbox exact=0/9] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2487, 16 atoms); better unscheduled exact=4/9: mod/use plumbing in crates/mdbook-preprocessor/src/lib.rs (4 atoms, discovered unscheduled) |
| 3.16 | 7060 | — | — | 0.43 | missing | RenderContext impl methods (incl. from_json) | [scheduled bbox exact=0/7] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3106, 22 atoms); better unscheduled exact=4/7: mod/use plumbing in crates/mdbook-renderer/src/lib.rs (4 atoms, discovered unscheduled) |
| 3.17 | 7552 | — | — | 0.00 | missing | MDBOOK_* env-var override rules | [unscheduled same-file] pub item at crates/mdbook-core/src/config.rs:449 (73 atoms, predecessor not scheduled: pub-item names surface in crates/mdbook-core/src/config.rs) |
| 4.1 | 7635 | — | — | 0.78 | partial | Builtin preprocessors module — re-exports | [scheduled bbox exact=6/9] mod/use plumbing in crates/mdbook-driver/src/builtin_preprocessors/mod.rs (t=7662, 6 atoms) |
| 4.3 | 8077 | — | — | 0.00 | missing | MDBook::load body — book.toml + Config wiring | [scheduled same-file] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=4754, 2 atoms) |
| 4.4 | 8287 | — | — | 0.07 | missing | load.rs — load_book entrypoint | [scheduled bbox exact=2/15] pub item at crates/mdbook-driver/src/load.rs:10 (t=8083, 2 atoms) |
| 4.5 | 8522 | — | — | 0.00 | missing | execute_build_process — the build pipeline | [scheduled same-file] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=4754, 2 atoms) |
| 4.6 | 8610 | — | — | 0.00 | missing | Default preprocessors + topological order | [scheduled same-file] pub-item names surface in crates/mdbook-driver/src/mdbook.rs (t=4754, 2 atoms) |
| 4.7 | 8735 | — | — | 0.58 | partial | BookBuilder type — mdbook init API | [scheduled bbox exact=6/12] pub item at crates/mdbook-driver/src/init.rs:13 (t=5114, 6 atoms) |
| 5.2 | 9440 | — | — | 0.00 | missing | Theme — bundled-asset names | [scheduled same-file] pub item at crates/mdbook-html/src/theme/mod.rs:40 (t=7393, 22 atoms) |
| 6.1 | 9515 | — | — | 0.15 | missing | build subcommand — clap surface | [scheduled bbox exact=2/7] pub item at src/cmd/build.rs:8 (t=5736, 2 atoms) |
| 6.2 | 9597 | — | — | 0.25 | missing | command_prelude shared args | [scheduled bbox exact=3/8] pub-item names surface in src/cmd/command_prelude.rs (t=5877, 3 atoms); better unscheduled exact=6/8: pub item at src/cmd/command_prelude.rs:7 (44 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.1 | 8977 | — | — | 0.00 | missing | html/mod.rs — markdown→HTML pipeline outline | no discovered line candidate |
| 6.3 | 9801 | — | — | 0.00 | missing | tests/testsuite/main.rs — test module map | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.10 | 1701 | — | — | 0.54 | partial | mdbook-html source layout | fs-only |
| 6.4 | 9848 | — | — | 0.30 | missing | examples/ + guide/ tree | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 75 | 227 | +152 | 0.80 | late | README lede | [scheduled bbox exact=2/5] README.md section #0 (t=227, 2 atoms) |
| 1.2 | 154 | 79 | -75 | 1.00 | early | Repo root listing | fs-only |
| 1.3 | 202 | 652 | +450 | 1.00 | late | crates/ directory listing | fs-only |
| 1.4 | 239 | 604 | +365 | 1.00 | late | Workspace package summary — name + description | [scheduled bbox exact=3/3] [package] in Cargo.toml (t=604, 15 atoms) |
| 1.6 | 482 | 604 | +122 | 0.92 | aligned | Workspace members + edition | [scheduled bbox exact=11/12] [package] in Cargo.toml (t=604, 11 atoms) |
| 1.7 | 536 | 5653 | +5117 | 1.00 | late | src/ tree (top-level bin sources) | fs-only |
| 2.1 | 882 | 4901 | +4019 | 1.00 | late | mdbook-driver crate role | [scheduled bbox exact=10/10] crate-doc lede in crates/mdbook-driver/src/lib.rs (t=4901, 10 atoms) |
| 2.3 | 1121 | 2007 | +886 | 1.00 | late | mdbook-summary crate lede | [scheduled bbox exact=5/5] crate-doc lede in crates/mdbook-summary/src/lib.rs (t=2007, 5 atoms) |
| 2.4 | 1229 | 976 | -253 | 0.86 | aligned | mdbook-markdown crate lede | [scheduled bbox exact=6/7] crate-doc lede in crates/mdbook-markdown/src/lib.rs (t=976, 6 atoms) |
| 2.8 | 1555 | 7508 | +5953 | 1.00 | late | mdbook-driver source layout | fs-only |
| 2.9 | 1591 | 6755 | +5164 | 1.00 | late | mdbook-core source layout | fs-only |
| 3.5 | 3208 | 6359 | +3151 | 0.87 | late | Preprocessor trait + PreprocessorContext | [scheduled bbox exact=16/45] pub item at crates/mdbook-preprocessor/src/lib.rs:51 (t=2487, 16 atoms) |
| 3.6 | 3742 | 4601 | +859 | 0.90 | aligned | Renderer trait + RenderContext | [scheduled bbox exact=22/40] pub item at crates/mdbook-renderer/src/lib.rs:40 (t=3106, 22 atoms) |
| 3.13 | 6324 | 1171 | -5153 | 0.95 | early | MarkdownOptions + new_cmark_parser | [scheduled bbox exact=17/21] pub item at crates/mdbook-markdown/src/lib.rs:15 (t=1171, 17 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 306 | 1.00 | 306 | 7393 | pub item at crates/mdbook-html/src/theme/mod.rs:40 |
| 257 | 1.00 | 257 | 5615 | [features] in Cargo.toml |
| 176 | 0.50 | 349 | 604 | [package] in Cargo.toml |
| 159 | 1.00 | 159 | 4098 | headings outline in CONTRIBUTING.md |
| 145 | 1.00 | 145 | 6969 | macro_export body at crates/mdbook-core/src/utils/mod.rs:17 |
| 137 | 1.00 | 137 | 9536 | mod/use plumbing in crates/mdbook-summary/src/lib.rs |
| 131 | 1.00 | 131 | 9054 | mod/use plumbing in crates/mdbook-driver/src/lib.rs |
| 111 | 1.00 | 111 | 7822 | README headline in guide/src/for_developers/README.md |
| 108 | 1.00 | 108 | 9221 | README.md section #1 |
| 102 | 1.00 | 102 | 9750 | mod/use plumbing in crates/mdbook-html/src/theme/mod.rs |
| 1976 | — | — | — | +29 more rows |
