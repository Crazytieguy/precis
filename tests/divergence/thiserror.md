scores: Sim=0.478 Reached=12/27 Early=2 Late=9 Partial=4 Missing=11 Used=9993/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (w×gap=0.00), 11 wrong-slice/granularity (w×gap=1.00), 4 no-discovered (w×gap=0.12)
Secondary intervention: split wrong-slice batches for 11 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 0 discovered-unscheduled
Top rows: 2.5, 3.3, 4.1, 4.2, 4.3, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 11 | 1.00 | 2/8/11 | nearby candidates have low exact atom overlap | 2.5, 3.3, 4.1, 4.2, 4.3, ... |
| add walker candidates for no-discovered rows | 4 | 0.12 | 0/1/4 | NS rows have no discovered line candidate | 5.1, 5.2, 6.1, 6.2 |

Tiers: 1=3/3 reached, 0 partial, 0 missing, avg=0.93; 2=7/8 reached, 1 partial, 0 missing, avg=0.91; 3=0/3 reached, 2 partial, 1 missing, avg=0.47; 4=0/4 reached, 0 partial, 4 missing, avg=0.02; 5=1/3 reached, 0 partial, 2 missing, avg=0.33; 6=0/2 reached, 0 partial, 2 missing, avg=0.00; 7=0/2 reached, 0 partial, 2 missing, avg=0.20; 8=1/2 reached, 1 partial, 0 missing, avg=0.79

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| wrong-slice / granularity | 11 | 7 | 4 | 0 | walker granularity / wrong slice |
| no discovered candidate | 4 | 4 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=16, scheduled same-file=2, fs-only=4, no discovered candidate=4

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | high | 3 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | low | 4 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.5 | 961 | — | — | 0.65 | partial | Macro entry point — derive_error in impl/src/lib.rs | [scheduled bbox exact=14/23] mod/use plumbing in impl/src/lib.rs (t=2362, 14 atoms) |
| 3.1 | 2515 | — | — | 0.71 | partial | ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind type defs | [scheduled bbox exact=12/52] pub-item names surface in impl/src/ast.rs (t=1457, 12 atoms) |
| 3.2 | 3231 | — | — | 0.67 | partial | attr.rs — Attrs / Display / Source / From / Transparent / Fmt / Trait type defs | [scheduled bbox exact=14/67] pub-item names surface in impl/src/attr.rs (t=1868, 14 atoms) |
| 3.3 | 3478 | — | — | 0.04 | missing | ast/attr/prop — public fn name locator | [scheduled bbox exact=1/28] pub item at impl/src/attr.rs:69 (t=1868, 2 atoms) |
| 4.1 | 3848 | — | — | 0.04 | missing | expand.rs — derive entry + try_expand | [scheduled bbox exact=2/29] pub item at impl/src/expand.rs:12 (t=1381, 2 atoms) |
| 4.2 | 4095 | — | — | 0.04 | missing | expand.rs / fmt.rs / generics.rs — public fn locator | [scheduled bbox exact=0/26] pub item at impl/src/generics.rs:48 (t=1339, 4 atoms) |
| 4.3 | 4579 | — | — | 0.00 | missing | expand.rs — impl_struct source/transparent branches | [scheduled same-file] pub-item names surface in impl/src/expand.rs (t=1381, 4 atoms) |
| 4.4 | 4854 | — | — | 0.00 | missing | expand.rs — from_initializer (#[from] body) | [scheduled same-file] pub-item names surface in impl/src/expand.rs (t=1381, 4 atoms) |
| 7.1 | 9392 | — | — | 0.19 | missing | src/lib.rs — module decls + cfg gates + private include! | [scheduled bbox exact=6/31] mod/use plumbing in src/lib.rs (t=887, 6 atoms) |
| 7.2 | 9711 | — | — | 0.21 | missing | src/provide.rs + var.rs — runtime helpers | [scheduled bbox exact=4/29] pub-item names surface in src/provide.rs (t=2410, 4 atoms) |
| 8.2 | 9966 | — | — | 0.58 | partial | test_source.rs — three source-shape examples | [scheduled bbox exact=6/19] pub-item names surface in tests/test_source.rs (t=6893, 6 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.1 | 5637 | — | — | 0.00 | missing | valid.rs — Struct + Enum + Variant validate (the rejection rules) | no discovered line candidate |
| 5.2 | 6828 | — | — | 0.00 | missing | valid.rs — check_non_field_attrs / check_field_attrs (cross-field rules) | no discovered line candidate |
| 6.1 | 7693 | — | — | 0.00 | missing | fmt.rs — expand_shorthand entry + state setup | no discovered line candidate |
| 6.2 | 9072 | — | — | 0.00 | missing | fmt.rs — placeholder loop ({var}/{0}/{:?} mechanic) | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 48 | 81 | +33 | 0.80 | late | Crate-doc lede | [scheduled bbox exact=4/5] README headline in README.md (t=81, 4 atoms) |
| 1.2 | 86 | 38 | -48 | 1.00 | early | Top-level repo listing | fs-only |
| 1.3 | 158 | 1105 | +947 | 1.00 | late | src/ + impl/src/ listings | fs-only |
| 2.1 | 245 | 2648 | +2403 | 0.88 | late | README example — enum head + first variant | [scheduled bbox exact=7/8] README.md section #1 (t=2648, 7 atoms) |
| 2.2 | 339 | 2648 | +2309 | 1.00 | late | README example — remaining variants | [scheduled bbox exact=10/10] README.md section #1 (t=2648, 10 atoms) |
| 2.4 | 735 | 1043 | +308 | 0.82 | late | Cargo.toml — std/no_std feature + workspace | [scheduled bbox exact=14/22] [features] in Cargo.toml (t=841, 14 atoms) |
| 2.6 | 1319 | 5603 | +4284 | 1.00 | late | Crate-doc bullets — Display + format-shorthand summary | [scheduled bbox exact=21/21] crate-doc body in src/lib.rs (t=5603, 21 atoms) |
| 2.7 | 1606 | 5603 | +3997 | 1.00 | late | Crate-doc bullets — From + source headlines | [scheduled bbox exact=17/17] crate-doc body in src/lib.rs (t=5603, 26 atoms) |
| 2.8 | 1983 | 5603 | +3620 | 0.95 | late | Crate-doc bullets — Backtrace + transparent headlines | [scheduled bbox exact=20/21] crate-doc body in src/lib.rs (t=5603, 50 atoms) |
| 5.3 | 7356 | 9903 | +2547 | 1.00 | late | tests/ui/ — trybuild compile-fail test listing | fs-only |
| 8.1 | 9792 | 6738 | -3054 | 1.00 | early | tests/ listing | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 11 | 2154 | README.md section #<n> |
| 2 | 419 | pub item at tests/test_expr.rs:<n> |
| 2 | 129 | pub item at tests/test_generics.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1476 | 0.59 | 2481 | 5603 | crate-doc body in src/lib.rs |
| 476 | 1.00 | 476 | 9006 | README.md section #4 |
| 344 | 1.00 | 344 | 7825 | README.md section #10 |
| 300 | 1.00 | 300 | 9375 | plaintext config LICENSE-MIT |
| 295 | 1.00 | 295 | 8530 | pub item at tests/test_expr.rs:11 |
| 272 | 1.00 | 272 | 3070 | README.md section #12 |
| 267 | 1.00 | 267 | 680 | crate-doc lede in src/lib.rs |
| 222 | 1.00 | 222 | 6666 | README.md section #6 |
| 193 | 1.00 | 193 | 6444 | README.md section #8 |
| 170 | 1.00 | 170 | 6185 | README.md section #5 |
| 1383 | — | — | — | +15 more rows |
