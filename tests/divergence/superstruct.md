scores: Sim=0.488 Reached=17/39 Early=6 Late=9 Partial=1 Missing=21 Used=9986/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=0.55), 11 wrong-slice/granularity (w×gap=2.84), 4 no-discovered (w×gap=0.28)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 2 predecessor-gated, 2 too-expensive, 0 discovered-unscheduled
Top rows: 1.1, 1.4, 3.1, 2.6, 4.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 11 | 2.84 | 4/8/11 | nearby candidates have low exact atom overlap | 1.1, 1.4, 3.1, 2.6, 4.5, ... |
| free final budget / demote late waste | 2 | 0.47 | 1/2/2 | high-overlap candidates exceed final remaining budget, exact total=37/39 | 4.1, 4.4 |
| add walker candidates for no-discovered rows | 4 | 0.28 | 1/1/4 | NS rows have no discovered line candidate | 4.3, 6.3, 6.4, 6.6 |
| promote export batches | 2 | 0.09 | 0/0/2 | 1 file, exact total=80/95 | 5.4, 5.5 |

Tiers: 1=3/5 reached, 0 partial, 2 missing, avg=0.60; 2=4/6 reached, 1 partial, 1 missing, avg=0.79; 3=6/7 reached, 0 partial, 1 missing, avg=0.85; 4=1/6 reached, 0 partial, 5 missing, avg=0.24; 5=2/8 reached, 0 partial, 6 missing, avg=0.37; 6=1/7 reached, 0 partial, 6 missing, avg=0.14

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 11 | 10 | 1 | 0 | walker granularity / wrong slice |
| no discovered candidate | 4 | 4 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 3 | 3 | 0 | 0 | filesystem/listing value |
| timing-only | 15 | 0 | 0 | 15 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.09 | promote predecessor |
| too expensive at final margin | 2 | 0.47 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=21, unscheduled bbox=5, scheduled same-file=2, fs-only=5, no discovered candidate=4

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | none | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | high | 2 |
| scheduled bbox | late | low | 4 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | low | 7 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | low | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 2380 | — | — | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) | [unscheduled bbox exact=26/26] headings outline in docs/reference/types.md (51 atoms, too expensive at final margin) |
| 4.4 | 3305 | — | — | 0.15 | missing | errors.md — StructError property table | [scheduled bbox exact=2/13] headings outline in docs/reference/errors.md (t=1278, 2 atoms); better unscheduled exact=11/13: docs/reference/errors.md section #2 (11 atoms, too expensive at final margin) |
| 5.4 | 6093 | — | — | 0.00 | missing | object() implementation | [unscheduled bbox exact=41/48] export body at src/structs/types.ts:298 body 299 (41 atoms, predecessor not scheduled: export at src/structs/types.ts:298) |
| 5.5 | 6541 | — | — | 0.00 | missing | union() implementation | [unscheduled bbox exact=39/47] export body at src/structs/types.ts:522 body 525 (39 atoms, predecessor not scheduled: export at src/structs/types.ts:522) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 96 | — | — | 0.00 | missing | Readme one-line lede | [scheduled same-file] headings outline in Readme.md (t=260, 12 atoms) |
| 1.4 | 306 | — | — | 0.00 | missing | Readme runtime-errors paragraph | [scheduled same-file] headings outline in Readme.md (t=260, 12 atoms) |
| 2.6 | 1168 | — | — | 0.76 | partial | docs/summary.md (the GitBook TOC) | [scheduled bbox exact=19/25] mdBook SUMMARY at docs/summary.md (t=1879, 19 atoms) |
| 3.1 | 1448 | — | — | 0.00 | missing | structs/types.ts — all 24 type-factory names | [unscheduled bbox exact=8/25] export names surface in src/structs/types.ts (22 atoms, too expensive at final margin) |
| 4.5 | 3875 | — | — | 0.18 | missing | core.md — `assert` / `create` / `validate` definitions | [scheduled bbox exact=6/34] headings outline in docs/reference/core.md (t=1560, 10 atoms); better unscheduled exact=7/34: docs/reference/core.md section #5 (7 atoms, too expensive at final margin) |
| 4.6 | 4064 | — | — | 0.13 | missing | coercions.md — `defaulted` worked example | [scheduled bbox exact=2/15] headings outline in docs/reference/coercions.md (t=1323, 2 atoms); better unscheduled exact=11/15: docs/reference/coercions.md section #1 (11 atoms, too expensive at final margin) |
| 5.1 | 4816 | — | — | 0.16 | missing | utils.ts run() — the central traversal generator | [scheduled bbox exact=12/67] export at src/utils.ts:130 (t=6290, 12 atoms); better unscheduled exact=47/67: export body at src/utils.ts:130 body 141 (47 atoms, too expensive at final margin) |
| 5.2 | 5074 | — | — | 0.33 | missing | struct.ts validate() body | [scheduled bbox exact=10/27] export at src/struct.ts:185 (t=4732, 10 atoms); better unscheduled exact=15/27: export body at src/struct.ts:185 body 194 (15 atoms, too expensive at final margin) |
| 5.6 | 6716 | — | — | 0.28 | missing | refine() implementation | [scheduled bbox exact=6/18] export at src/structs/refinements.ts:146 (t=2335, 6 atoms); better unscheduled exact=11/18: export body at src/structs/refinements.ts:146 body 151 (11 atoms, too expensive at final margin) |
| 5.7 | 7243 | — | — | 0.36 | missing | coerce() + defaulted() implementations | [scheduled bbox exact=8/55] export body at src/structs/coercions.ts:16 body 21 (t=3267, 8 atoms); better unscheduled exact=20/55: export body at src/structs/coercions.ts:38 body 45 (20 atoms, too expensive at final margin) |
| 6.5 | 9499 | — | — | 0.00 | missing | types.ts — `object` and `type` reference bodies | [unscheduled bbox exact=18/42] docs/reference/types.md section #23 (18 atoms, predecessor not scheduled: headings outline in docs/reference/types.md) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.3 | 2854 | — | — | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) | no discovered line candidate |
| 6.3 | 8099 | — | — | 0.00 | missing | Sample validation fixture — three exemplar modules | no discovered line candidate |
| 6.4 | 8999 | — | — | 0.00 | missing | test/index.test.ts — the validation-fixture harness | no discovered line candidate |
| 6.6 | 9863 | — | — | 0.00 | missing | examples/default-values.js — defaulted + create | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.4 | 802 | — | — | 0.00 | missing | examples/ + test/ + test/api/ tree | fs-only |
| 6.1 | 7559 | — | — | 0.00 | missing | test/validation/ tree — all 41 kind directories | fs-only |
| 6.2 | 7742 | — | — | 0.00 | missing | test/typings/ tree — all type-level test files | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 161 | 65 | -96 | 1.00 | early | Repo root listing | fs-only |
| 1.3 | 240 | 1501 | +1261 | 1.00 | late | Public entry — src/index.ts in full | [scheduled bbox exact=6/6] imports in src/index.ts (t=1501, 6 atoms) |
| 1.5 | 394 | 886 | +492 | 1.00 | late | package.json — name, version, license | [scheduled bbox exact=5/5] package identity in package.json (t=886, 5 atoms) |
| 2.2 | 583 | 8541 | +7958 | 1.00 | late | package.json — module shape & engines | [scheduled bbox exact=7/14] package entrypoints in package.json (t=962, 7 atoms) |
| 2.3 | 694 | 1930 | +1236 | 1.00 | late | docs/ subtree listings | fs-only |
| 2.5 | 851 | 260 | -591 | 1.00 | early | Readme section headings (locations only) | [scheduled bbox exact=6/7] headings outline in Readme.md (t=260, 11 atoms) |
| 3.2 | 1533 | 2258 | +725 | 1.00 | late | structs/refinements.ts — all 7 refinement names | [scheduled bbox exact=7/7] export names surface in src/structs/refinements.ts (t=2258, 13 atoms) |
| 3.3 | 1571 | 1047 | -524 | 1.00 | early | structs/coercions.ts — coerce / defaulted / trimmed | [scheduled bbox exact=0/3] export body at src/structs/coercions.ts:16 body 21 (t=3267, 8 atoms) |
| 3.4 | 1682 | 9085 | +7403 | 1.00 | late | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | [scheduled bbox exact=8/9] export names surface in src/structs/utilities.ts (t=9085, 24 atoms) |
| 3.5 | 1789 | 5538 | +3749 | 1.00 | late | struct.ts — Struct class + assert/create/is/mask/validate signatures | [scheduled bbox exact=6/11] export at src/struct.ts:10 (t=5538, 37 atoms) |
| 3.6 | 1985 | 4493 | +2508 | 1.00 | late | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | [scheduled bbox exact=8/15] export names surface in src/struct.ts (t=4422, 12 atoms) |
| 3.7 | 2170 | 505 | -1665 | 0.95 | early | error.ts — Failure type + StructError class signature | [scheduled bbox exact=10/19] export at src/error.ts:5 (t=414, 10 atoms) |
| 5.3 | 5610 | 8240 | +2630 | 0.90 | late | struct.ts Struct class constructor + 4 hook fields | [scheduled bbox exact=28/51] export body at src/struct.ts:10 body 30 (t=8240, 28 atoms) |
| 5.8 | 7434 | 2883 | -4551 | 0.93 | early | error.ts StructError constructor | [scheduled bbox exact=12/14] export body at src/error.ts:25 body 36 (t=2883, 12 atoms) |
| 6.7 | 9978 | 2159 | -7819 | 1.00 | early | Guide H2 headings — all 14 across guides 02-06 | [scheduled bbox exact=4/13] headings outline in docs/guides/02-validating-data.md (t=2159, 7 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 598 | export at src/utils.ts:<n> |
| 3 | 434 | export at src/structs/utilities.ts:<n> |
| 3 | 176 | export at src/structs/refinements.ts:<n> |
| 2 | 166 | export doc at src/struct.ts:<n> |
| 2 | 115 | export doc at src/utils.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 434 | 1.00 | 434 | 3945 | headings outline in Changelog.md |
| 422 | 1.00 | 422 | 7709 | export at src/utils.ts:334 |
| 324 | 1.00 | 324 | 7028 | package scripts in package.json |
| 269 | 0.89 | 301 | 8541 | package dependencies in package.json |
| 249 | 0.65 | 381 | 886 | package identity in package.json |
| 222 | 0.94 | 236 | 5877 | export names surface in src/utils.ts |
| 182 | 1.00 | 182 | 9867 | export at src/structs/utilities.ts:44 |
| 150 | 1.00 | 150 | 9685 | export at src/structs/utilities.ts:30 |
| 119 | 1.00 | 119 | 9986 | export body at src/structs/utilities.ts:80 body 84 |
| 109 | 1.00 | 109 | 8842 | export doc at src/error.ts:25 |
| 1789 | — | — | — | +25 more rows |
