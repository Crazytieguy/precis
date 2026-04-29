scores: Score(3000)=0.609 ns_rows≤3K=12/27 (reached=4 partial=3 missing=5)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 109 | 0.815 | 0.540 | 0.663 | 988 |
| 1442 | 130 | 0.814 | 0.557 | 0.673 | 1416 |
| 2080 | 168 | 0.812 | 0.455 | 0.608 | 2075 |
| 3000 | 220 | 0.794 | 0.467 | 0.609 | 2919 |
| 4327 | 370 | 0.823 | 0.538 | 0.665 | 4075 |
| 6240 | 506 | 0.785 | 0.393 | 0.556 | 4436 |
| 9000 | 709 | 0.814 | 0.384 | 0.559 | 8968 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (gap@3k=0.00), 15 wrong-slice/granularity (gap@3k=1.41), 0 no-discovered (gap@3k=0.00)
Top rows: 5.2, 5.1, 3.2, 6.2, 3.1, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 15 | 1.80 | 1.41 | 1.14 | nearby candidates have low exact atom overlap | 5.2, 5.1, 3.2, 6.2, 3.1, ... |
| finish partially-delivered NS batches | 4 | 0.86 | 0.47 | 0.26 | avg batch completion=0.63 | 3.2, 3.1, 2.5, 4.1 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| wrong-slice / granularity | 15 | 13 | 2 | walker granularity / wrong slice |
| fs/listing | 2 | 2 | 0 | filesystem/listing value |
| mixed/unknown | 6 | 5 | 1 | inspect row |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=17, scheduled same-file=4, fs-only=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 9 |
| scheduled bbox | missing | high | 2 |
| scheduled bbox | missing | full | 3 |
| scheduled bbox | partial | low | 2 |
| scheduled bbox | partial | high | 1 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.5 | 961 | 0.74 | 0.80 | partial | Macro entry point — derive_error in impl/src/lib.rs | [scheduled bbox exact=14/23] mod/use plumbing in impl/src/lib.rs (t=1367, 14 atoms) |
| 3.1 | 2515 | 0.71 | 0.71 | partial | ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind type defs | [scheduled bbox exact=12/52] pub-item names surface in impl/src/ast.rs (t=1492, 12 atoms) |
| 3.2 | 3231 | 0.67 | 0.63 | missing | attr.rs — Attrs / Display / Source / From / Transparent / Fmt / Trait type defs | [scheduled bbox exact=14/67] pub-item names surface in impl/src/attr.rs (t=2025, 14 atoms) |
| 3.3 | 3478 | 0.04 | 0.03 | missing | ast/attr/prop — public fn name locator | [scheduled bbox exact=13/28] impl method sigs in impl/src/prop.rs (t=7915, 29 atoms) |
| 4.1 | 3848 | 0.28 | 0.36 | missing | expand.rs — derive entry + try_expand | [scheduled bbox exact=10/29] mod/use plumbing in impl/src/expand.rs (t=3663, 10 atoms) |
| 4.2 | 4095 | 0.04 | 0.01 | missing | expand.rs / fmt.rs / generics.rs — public fn locator | [scheduled bbox exact=5/26] impl method sigs in impl/src/generics.rs (t=7281, 10 atoms) |
| 4.3 | 4579 | 0.00 | 0.00 | missing | expand.rs — impl_struct source/transparent branches | [scheduled same-file] mod/use plumbing in impl/src/expand.rs (t=3663, 10 atoms) |
| 4.4 | 4854 | 0.00 | 0.00 | missing | expand.rs — from_initializer (#[from] body) | [scheduled same-file] mod/use plumbing in impl/src/expand.rs (t=3663, 10 atoms) |
| 5.1 | 5637 | 0.00 | 0.00 | missing | valid.rs — Struct + Enum + Variant validate (the rejection rules) | [scheduled bbox exact=9/76] impl method sigs in impl/src/valid.rs (t=7147, 9 atoms) |
| 5.2 | 6828 | 0.00 | 0.00 | missing | valid.rs — check_non_field_attrs / check_field_attrs (cross-field rules) | [scheduled same-file] impl method sigs in impl/src/valid.rs (t=7147, 15 atoms) |
| 6.1 | 7693 | 0.00 | 0.00 | missing | fmt.rs — expand_shorthand entry + state setup | [scheduled bbox exact=2/26] impl method sigs in impl/src/fmt.rs (t=3516, 2 atoms) |
| 6.2 | 9072 | 0.00 | 0.00 | missing | fmt.rs — placeholder loop ({var}/{0}/{:?} mechanic) | [scheduled same-file] mod/use plumbing in impl/src/fmt.rs (t=4010, 13 atoms) |
| 7.1 | 9392 | 0.19 | 0.11 | missing | src/lib.rs — module decls + cfg gates + private include! | [scheduled bbox exact=6/31] mod/use plumbing in src/lib.rs (t=276, 6 atoms) |
| 7.2 | 9711 | 0.14 | 0.15 | missing | src/provide.rs + var.rs — runtime helpers | [scheduled bbox exact=4/29] pub-item names surface in src/provide.rs (t=2688, 4 atoms) |
| 8.2 | 9966 | 0.00 | 0.00 | missing | test_source.rs — three source-shape examples | [scheduled bbox exact=6/19] pub-item names surface in tests/test_source.rs (t=9688, 6 atoms) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 5.3 | 7356 | 0.00 | 0.00 | missing | tests/ui/ — trybuild compile-fail test listing | fs-only |
| 8.1 | 9792 | 0.00 | 0.00 | missing | tests/ listing | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 48 | 0.80 | 0.99 | partial | Crate-doc lede | [scheduled bbox exact=4/5] README headline in README.md (t=81, 4 atoms) |
| 2.1 | 245 | 0.00 | 0.00 | missing | README example — enum head + first variant | [scheduled bbox exact=7/8] README.md section #1 (t=3318, 7 atoms) |
| 2.2 | 339 | 0.00 | 0.00 | missing | README example — remaining variants | [scheduled bbox exact=10/10] README.md section #1 (t=3318, 10 atoms) |
| 2.6 | 1319 | 0.00 | 0.00 | missing | Crate-doc bullets — Display + format-shorthand summary | [scheduled bbox exact=21/21] crate-doc body in src/lib.rs (t=6917, 21 atoms) |
| 2.7 | 1606 | 0.00 | 0.00 | missing | Crate-doc bullets — From + source headlines | [scheduled bbox exact=17/17] crate-doc body in src/lib.rs (t=6917, 26 atoms) |
| 2.8 | 1983 | 0.00 | 0.00 | missing | Crate-doc bullets — Backtrace + transparent headlines | [scheduled bbox exact=20/21] crate-doc body in src/lib.rs (t=6917, 50 atoms) |

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
