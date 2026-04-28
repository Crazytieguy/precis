scores: Sim=0.433 Reached=18/50 Early=8 Late=5 Partial=12 Missing=20 Used=9476/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (w×gap=0.00), 31 wrong-slice/granularity (w×gap=5.18), 0 no-discovered (w×gap=0.00)
Secondary intervention: split wrong-slice batches for 31 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 0 discovered-unscheduled
Top rows: 2.4, 2.7, 2.8, 1.2, 2.9, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 31 | 5.18 | 12/22/31 | nearby candidates have low exact atom overlap | 2.4, 2.7, 2.8, 1.2, 2.9, ... |

Tiers: 1=5/6 reached, 1 partial, 0 missing, avg=0.90; 2=2/11 reached, 3 partial, 6 missing, avg=0.38; 3=1/10 reached, 3 partial, 6 missing, avg=0.43; 4=3/6 reached, 1 partial, 2 missing, avg=0.59; 5=5/13 reached, 4 partial, 4 missing, avg=0.59; 6=2/4 reached, 0 partial, 2 missing, avg=0.56

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| wrong-slice / granularity | 31 | 19 | 12 | 0 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 14 | 0 | 0 | 14 | usually no code change |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=31, scheduled same-file=11, fs-only=4

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | early | none | 1 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 2 |
| scheduled bbox | early | full | 2 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 7 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 11 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 54 | — | — | 0.50 | partial | README H1 + crate one-liner | [scheduled bbox exact=1/2] README headline in README.md (t=79, 1 atoms) |
| 2.2 | 534 | — | — | 0.60 | partial | ContentType enum context | [scheduled bbox exact=6/10] pub item at src/parse/mod.rs:18 (t=3100, 14 atoms) |
| 2.4 | 836 | — | — | 0.16 | missing | ContentType::new_parser dispatch | [scheduled bbox exact=3/13] impl method sigs in src/parse/mod.rs (t=2850, 3 atoms) |
| 2.5 | 931 | — | — | 0.70 | partial | Tree struct fields | [scheduled bbox exact=7/10] pub item at src/tree.rs:14 (t=1455, 7 atoms) |
| 2.6 | 1144 | — | — | 0.76 | partial | ItemValue + FieldType | [scheduled bbox exact=8/25] pub item at src/tree.rs:42 (t=618, 8 atoms) |
| 2.7 | 1215 | — | — | 0.00 | missing | Tree public fn signatures | [scheduled same-file] pub item at src/tree.rs:42 (t=618, 8 atoms) |
| 2.8 | 1432 | — | — | 0.00 | missing | main.rs run() — config + flag dispatch | [scheduled same-file] mod/use plumbing in src/main.rs (t=5581, 22 atoms) |
| 2.9 | 1755 | — | — | 0.00 | missing | main.rs run() — content type + data read | [scheduled same-file] mod/use plumbing in src/main.rs (t=5581, 22 atoms) |
| 2.10 | 1929 | — | — | 0.00 | missing | main.rs run() — --to short-circuit | [scheduled same-file] mod/use plumbing in src/main.rs (t=5581, 22 atoms) |
| 2.11 | 2171 | — | — | 0.00 | missing | main.rs run() — size check, Tree, App, ui::start | [scheduled same-file] mod/use plumbing in src/main.rs (t=5581, 22 atoms) |
| 3.2 | 2495 | — | — | 0.72 | partial | App struct fields | [scheduled bbox exact=23/32] pub item at src/ui/app.rs:53 (t=5272, 23 atoms) |
| 3.3 | 2739 | — | — | 0.28 | missing | ElementInFocus + Refresh + ShowResult | [scheduled bbox exact=0/29] pub item at src/ui/app.rs:53 (t=5272, 23 atoms) |
| 3.4 | 3020 | — | — | 0.04 | missing | ui::start event loop | [scheduled bbox exact=2/27] pub item at src/ui/mod.rs:35 (t=1780, 2 atoms) |
| 3.5 | 3205 | — | — | 0.00 | missing | App method names | [scheduled same-file] pub item at src/ui/app.rs:53 (t=5272, 23 atoms) |
| 3.6 | 3560 | — | — | 0.36 | missing | TreeOverview struct + impl method names | [scheduled bbox exact=12/33] pub item at src/ui/tree_overview.rs:19 (t=4040, 12 atoms) |
| 3.7 | 3838 | — | — | 0.00 | missing | TreeOverview::on_key action dispatch | [scheduled same-file] pub item at src/ui/tree_overview.rs:19 (t=4040, 12 atoms) |
| 3.8 | 4084 | — | — | 0.46 | missing | DataBlock struct + method names | [scheduled bbox exact=12/26] pub item at src/ui/data_block.rs:16 (t=3897, 12 atoms) |
| 3.9 | 4359 | — | — | 0.67 | partial | Filter widget surface | [scheduled bbox exact=8/33] pub-item names surface in src/ui/filter.rs (t=2164, 8 atoms) |
| 3.10 | 4672 | — | — | 0.75 | partial | Footer / Header / Popup surfaces | [scheduled bbox exact=6/36] pub item at src/ui/header.rs:11 (t=2121, 6 atoms) |
| 4.2 | 4844 | — | — | 0.71 | partial | SyntaxToken enum | [scheduled bbox exact=12/17] pub item at src/parse/syntax.rs:11 (t=3619, 12 atoms) |
| 4.3 | 5287 | — | — | 0.00 | missing | Per-parser Parser impl headers | [scheduled same-file] pub-item names surface in src/parse/json.rs (t=3444, 4 atoms) |
| 4.5 | 5796 | — | — | 0.10 | missing | AnyParser auto-detect parse_root | [scheduled bbox exact=3/30] pub item at src/parse/any.rs:9 (t=2939, 3 atoms) |
| 5.2 | 6234 | — | — | 0.69 | partial | Config struct field names | [scheduled bbox exact=24/35] pub item at src/config/mod.rs:17 (t=1727, 24 atoms) |
| 5.3 | 6578 | — | — | 0.00 | missing | Action enum variants | [scheduled same-file] pub item at src/config/keys.rs:195 (t=9153, 64 atoms) |
| 5.4 | 6976 | — | — | 0.00 | missing | Default key bindings | [scheduled same-file] pub item at src/config/keys.rs:195 (t=9153, 64 atoms) |
| 5.6 | 7644 | — | — | 0.76 | partial | Per-branch struct fields (Tree / Layout / Filter / Data) | [scheduled bbox exact=10/45] pub-item names surface in src/config/mod.rs (t=855, 16 atoms) |
| 5.7 | 7894 | — | — | 0.72 | partial | Editor + Header + Footer struct fields | [scheduled bbox exact=6/25] pub-item names surface in src/config/mod.rs (t=855, 10 atoms) |
| 5.8 | 8331 | — | — | 0.08 | missing | Config::load + Config::parse | [scheduled bbox exact=6/39] impl method sigs in src/config/mod.rs (t=5038, 6 atoms) |
| 5.9 | 8613 | — | — | 0.04 | missing | Config::get_path resolution | [scheduled bbox exact=2/24] impl method sigs in src/config/mod.rs (t=5038, 2 atoms) |
| 5.10 | 8986 | — | — | 0.70 | partial | Color struct + Colors namespaces | [scheduled bbox exact=0/37] pub item at src/config/colors.rs:162 (t=6808, 24 atoms) |
| 6.4 | 9967 | — | — | 0.00 | missing | clipboard OS routing | [scheduled same-file] pub item at src/clipboard.rs:28 (t=437, 2 atoms) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 6.2 | 9652 | — | — | 0.25 | missing | examples/ + docs/ listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 92 | 374 | +282 | 1.00 | late | src/ module layout | fs-only |
| 1.4 | 156 | 5581 | +5425 | 1.00 | late | main.rs module declarations | [scheduled bbox exact=9/9] mod/use plumbing in src/main.rs (t=5581, 9 atoms) |
| 1.5 | 185 | 320 | +135 | 1.00 | late | README section headings | [scheduled bbox exact=1/4] README.md section #4 (t=7220, 12 atoms) |
| 1.6 | 329 | 255 | -74 | 0.92 | aligned | Cargo package metadata | [scheduled bbox exact=11/12] [package] in Cargo.toml (t=255, 11 atoms) |
| 2.1 | 391 | 3100 | +2709 | 1.00 | late | ContentType variants — locations only | [scheduled bbox exact=8/8] pub item at src/parse/mod.rs:18 (t=3100, 13 atoms) |
| 2.3 | 653 | 3403 | +2750 | 1.00 | late | Parser trait method signatures | [scheduled bbox exact=7/7] pub item at src/parse/mod.rs:36 (t=3403, 23 atoms) |
| 4.1 | 4715 | 2801 | -1914 | 1.00 | early | parse/ module layout | fs-only |
| 4.4 | 5491 | 3403 | -2088 | 0.89 | early | Parser::parse_root default body | [scheduled bbox exact=17/19] pub item at src/parse/mod.rs:36 (t=3403, 17 atoms) |
| 4.6 | 5872 | 3526 | -2346 | 0.86 | early | syntax helper signatures | [scheduled bbox exact=4/7] pub-item names surface in src/parse/syntax.rs (t=3504, 5 atoms) |
| 5.1 | 5888 | 754 | -5134 | 1.00 | early | config/ module layout | fs-only |
| 5.5 | 7174 | 2410 | -4764 | 0.80 | early | Key enum + KeyAction | [scheduled bbox exact=0/25] pub item at src/config/keys.rs:195 (t=9153, 64 atoms) |
| 5.11 | 9095 | 5840 | -3255 | 1.00 | early | DataColors fields | [scheduled bbox exact=11/11] pub item at src/config/colors.rs:78 (t=5840, 21 atoms) |
| 5.13 | 9311 | 4193 | -5118 | 0.88 | early | Types config struct | [scheduled bbox exact=7/8] pub item at src/config/types.rs:28 (t=4193, 14 atoms) |
| 6.3 | 9808 | 6482 | -3326 | 1.00 | early | Changelog version headings | [scheduled bbox exact=13/13] headings outline in docs/changelog.md (t=6482, 53 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 489 | pub item at src/config/colors.rs:<n> |
| 5 | 483 | README.md section #<n> |
| 2 | 182 | docs/changelog.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 870 | 1.00 | 870 | 9153 | pub item at src/config/keys.rs:195 |
| 626 | 0.69 | 905 | 8125 | pub item at src/cmd.rs:13 |
| 348 | 0.82 | 425 | 5038 | impl method sigs in src/config/mod.rs |
| 321 | 1.00 | 321 | 6161 | [dependencies] in Cargo.toml |
| 188 | 1.00 | 188 | 6996 | mod/use plumbing in src/ui/mod.rs |
| 187 | 1.00 | 187 | 7220 | README.md section #4 |
| 178 | 0.60 | 295 | 6808 | pub item at src/config/colors.rs:162 |
| 165 | 0.51 | 321 | 6482 | headings outline in docs/changelog.md |
| 159 | 0.61 | 259 | 5840 | pub item at src/config/colors.rs:78 |
| 149 | 0.70 | 213 | 5581 | mod/use plumbing in src/main.rs |
| 1208 | — | — | — | +14 more rows |
