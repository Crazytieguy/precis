scores: Sim=0.524 Reached=13/27 Early=1 Late=8 Partial=4 Missing=10 Used=9882/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (w×gap=0.00), 13 wrong-slice/granularity (w×gap=0.67), 0 no-discovered (w×gap=0.00)
Secondary intervention: split wrong-slice batches for 13 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 0 discovered-unscheduled
Top rows: 2.5, 4.3, 4.2, 4.4, 4.1, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 13 | 0.67 | 1/7/13 | nearby candidates have low exact atom overlap | 2.5, 4.3, 4.2, 4.4, 4.1, ... |

Tiers: 1=3/3 reached, 0 partial, 0 missing, avg=0.93; 2=7/8 reached, 1 partial, 0 missing, avg=0.92; 3=2/3 reached, 1 partial, 0 missing, avg=0.82; 4=0/4 reached, 1 partial, 3 missing, avg=0.22; 5=0/3 reached, 0 partial, 3 missing, avg=0.03; 6=0/2 reached, 0 partial, 2 missing, avg=0.02; 7=0/2 reached, 0 partial, 2 missing, avg=0.20; 8=1/2 reached, 1 partial, 0 missing, avg=0.79

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| wrong-slice / granularity | 13 | 9 | 4 | 0 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=18, scheduled same-file=4, fs-only=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | high | 3 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | low | 5 |
| scheduled bbox | partial | low | 4 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.5 | 961 | — | — | 0.74 | partial | Macro entry point — derive_error in impl/src/lib.rs | [scheduled bbox exact=14/23] mod/use plumbing in impl/src/lib.rs (t=1367, 14 atoms) |
| 3.3 | 3478 | — | — | 0.79 | partial | ast/attr/prop — public fn name locator | [scheduled bbox exact=13/28] impl method sigs in impl/src/prop.rs (t=7915, 29 atoms) |
| 4.1 | 3848 | — | — | 0.62 | partial | expand.rs — derive entry + try_expand | [scheduled bbox exact=10/29] mod/use plumbing in impl/src/expand.rs (t=3663, 10 atoms) |
| 4.2 | 4095 | — | — | 0.27 | missing | expand.rs / fmt.rs / generics.rs — public fn locator | [scheduled bbox exact=5/26] impl method sigs in impl/src/generics.rs (t=7281, 10 atoms) |
| 4.3 | 4579 | — | — | 0.00 | missing | expand.rs — impl_struct source/transparent branches | [scheduled same-file] mod/use plumbing in impl/src/expand.rs (t=3663, 10 atoms) |
| 4.4 | 4854 | — | — | 0.00 | missing | expand.rs — from_initializer (#[from] body) | [scheduled same-file] mod/use plumbing in impl/src/expand.rs (t=3663, 10 atoms) |
| 5.1 | 5637 | — | — | 0.08 | missing | valid.rs — Struct + Enum + Variant validate (the rejection rules) | [scheduled bbox exact=9/76] impl method sigs in impl/src/valid.rs (t=7147, 9 atoms) |
| 5.2 | 6828 | — | — | 0.00 | missing | valid.rs — check_non_field_attrs / check_field_attrs (cross-field rules) | [scheduled same-file] impl method sigs in impl/src/valid.rs (t=7147, 15 atoms) |
| 6.1 | 7693 | — | — | 0.04 | missing | fmt.rs — expand_shorthand entry + state setup | [scheduled bbox exact=2/26] impl method sigs in impl/src/fmt.rs (t=3516, 2 atoms) |
| 6.2 | 9072 | — | — | 0.00 | missing | fmt.rs — placeholder loop ({var}/{0}/{:?} mechanic) | [scheduled same-file] mod/use plumbing in impl/src/fmt.rs (t=4010, 13 atoms) |
| 7.1 | 9392 | — | — | 0.19 | missing | src/lib.rs — module decls + cfg gates + private include! | [scheduled bbox exact=6/31] mod/use plumbing in src/lib.rs (t=276, 6 atoms) |
| 7.2 | 9711 | — | — | 0.21 | missing | src/provide.rs + var.rs — runtime helpers | [scheduled bbox exact=4/29] pub-item names surface in src/provide.rs (t=2688, 4 atoms) |
| 8.2 | 9966 | — | — | 0.58 | partial | test_source.rs — three source-shape examples | [scheduled bbox exact=6/19] pub-item names surface in tests/test_source.rs (t=9688, 6 atoms) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.3 | 7356 | — | — | 0.00 | missing | tests/ui/ — trybuild compile-fail test listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 48 | 81 | +33 | 0.80 | late | Crate-doc lede | [scheduled bbox exact=4/5] README headline in README.md (t=81, 4 atoms) |
| 1.2 | 86 | 38 | -48 | 1.00 | early | Top-level repo listing | fs-only |
| 1.3 | 158 | 934 | +776 | 1.00 | late | src/ + impl/src/ listings | fs-only |
| 2.1 | 245 | 3318 | +3073 | 0.88 | late | README example — enum head + first variant | [scheduled bbox exact=7/8] README.md section #1 (t=3318, 7 atoms) |
| 2.2 | 339 | 3318 | +2979 | 1.00 | late | README example — remaining variants | [scheduled bbox exact=10/10] README.md section #1 (t=3318, 10 atoms) |
| 2.4 | 735 | 1900 | +1165 | 0.82 | late | Cargo.toml — std/no_std feature + workspace | [scheduled bbox exact=14/22] [features] in Cargo.toml (t=887, 14 atoms) |
| 2.6 | 1319 | 6917 | +5598 | 1.00 | late | Crate-doc bullets — Display + format-shorthand summary | [scheduled bbox exact=21/21] crate-doc body in src/lib.rs (t=6917, 21 atoms) |
| 2.7 | 1606 | 6917 | +5311 | 1.00 | late | Crate-doc bullets — From + source headlines | [scheduled bbox exact=17/17] crate-doc body in src/lib.rs (t=6917, 26 atoms) |
| 2.8 | 1983 | 6917 | +4934 | 0.95 | late | Crate-doc bullets — Backtrace + transparent headlines | [scheduled bbox exact=20/21] crate-doc body in src/lib.rs (t=6917, 50 atoms) |
| 3.1 | 2515 | 3135 | +620 | 0.87 | aligned | ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind type defs | [scheduled bbox exact=12/52] pub-item names surface in impl/src/ast.rs (t=1492, 12 atoms) |
| 3.2 | 3231 | 3816 | +585 | 0.81 | aligned | attr.rs — Attrs / Display / Source / From / Transparent / Fmt / Trait type defs | [scheduled bbox exact=14/67] pub-item names surface in impl/src/attr.rs (t=2025, 14 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 9 | 1334 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1476 | 0.59 | 2481 | 6917 | crate-doc body in src/lib.rs |
| 493 | 1.00 | 493 | 9461 | impl method sigs in impl/src/unraw.rs |
| 272 | 1.00 | 272 | 4347 | README.md section #12 |
| 267 | 1.00 | 267 | 726 | crate-doc lede in src/lib.rs |
| 222 | 1.00 | 222 | 8968 | README.md section #6 |
| 194 | 1.00 | 194 | 4010 | mod/use plumbing in impl/src/fmt.rs |
| 193 | 1.00 | 193 | 8746 | README.md section #8 |
| 170 | 1.00 | 170 | 8487 | README.md section #5 |
| 160 | 1.00 | 160 | 7596 | README.md section #7 |
| 127 | 1.00 | 127 | 7436 | README.md section #9 |
| 1274 | — | — | — | +17 more rows |
