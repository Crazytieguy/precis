scores: Score(3000)=0.583 ns_rows≤3K=21/39 (reached=10 partial=2 missing=9)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 106 | 0.650 | 0.494 | 0.567 | 991 |
| 1442 | 131 | 0.699 | 0.504 | 0.594 | 1431 |
| 2080 | 201 | 0.704 | 0.489 | 0.587 | 2063 |
| 3000 | 301 | 0.687 | 0.494 | 0.583 | 2979 |
| 4327 | 363 | 0.668 | 0.410 | 0.523 | 4311 |
| 6240 | 556 | 0.690 | 0.355 | 0.495 | 6234 |
| 9000 | 898 | 0.663 | 0.302 | 0.448 | 8995 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 5 ranking-recoverable (gap@3k=0.38), 18 wrong-slice/granularity (gap@3k=2.36), 1 no-discovered (gap@3k=0.05)
Secondary intervention: promote predecessors for 2 gated candidates
Top rows: 1.1, 3.1, 5.1, 4.3, 5.3, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 18 | 2.53 | 2.36 | 2.03 | nearby candidates have low exact atom overlap | 1.1, 3.1, 5.1, 4.3, 5.3, ... |
| tune ranking for high-overlap unscheduled candidates | 3 | 0.21 | 0.21 | 0.21 | high-overlap candidates not in the schedule by T_max, exact total=45/48 | 4.1, 3.4, 4.4 |
| promote export batches | 2 | 0.17 | 0.17 | 0.17 | 1 file, exact total=80/95 | 5.4, 5.5 |
| add walker candidates for no-discovered rows | 1 | 0.05 | 0.05 | 0.05 | NS rows have no discovered line candidate | 6.3 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 5 | 5 | 0 | value/ranking |
| wrong-slice / granularity | 18 | 16 | 2 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 3 | 3 | 0 | filesystem/listing value |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 2 | 0.17 | promote predecessor |
| too expensive at final margin | 3 | 0.21 | tune ranking |

Candidate hint kinds: scheduled bbox=13, unscheduled bbox=8, scheduled same-file=2, fs-only=3, no discovered candidate=1 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 10 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | low | 5 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.4 | 1682 | 0.11 | 0.07 | missing | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | [scheduled bbox exact=0/9] export doc at src/structs/utilities.ts:247 (t=5262, 5 atoms); better unscheduled exact=8/9: export names surface in src/structs/utilities.ts (24 atoms, too expensive at final margin) |
| 4.1 | 2380 | 0.00 | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) | [unscheduled bbox exact=26/26] headings outline in docs/reference/types.md (51 atoms, too expensive at final margin) |
| 4.4 | 3305 | 0.15 | 0.01 | missing | errors.md — StructError property table | [scheduled bbox exact=2/13] headings outline in docs/reference/errors.md (t=1101, 2 atoms); better unscheduled exact=11/13: docs/reference/errors.md section #2 (11 atoms, too expensive at final margin) |
| 5.4 | 6093 | 0.00 | 0.00 | missing | object() implementation | [unscheduled bbox exact=41/48] export body at src/structs/types.ts:298 body 299 (41 atoms, predecessor not scheduled: export at src/structs/types.ts:298) |
| 5.5 | 6541 | 0.00 | 0.00 | missing | union() implementation | [unscheduled bbox exact=39/47] export body at src/structs/types.ts:522 body 525 (39 atoms, predecessor not scheduled: export at src/structs/types.ts:522) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 96 | 0.00 | 0.00 | missing | Readme one-line lede | [scheduled same-file] Readme.md section #0 (t=8973, 76 atoms) |
| 1.4 | 306 | 0.00 | 0.00 | missing | Readme runtime-errors paragraph | [scheduled same-file] Readme.md section #0 (t=8973, 76 atoms) |
| 2.2 | 583 | 0.79 | 0.85 | partial | package.json — module shape & engines | [scheduled bbox exact=7/14] package entrypoints in package.json (t=962, 7 atoms) |
| 2.6 | 1168 | 0.76 | 0.99 | partial | docs/summary.md (the GitBook TOC) | [scheduled bbox exact=19/25] mdBook SUMMARY at docs/summary.md (t=1854, 19 atoms) |
| 3.1 | 1448 | 0.00 | 0.00 | missing | structs/types.ts — all 24 type-factory names | [unscheduled bbox exact=8/25] export names surface in src/structs/types.ts (22 atoms, too expensive at final margin) |
| 3.5 | 1789 | 0.00 | 0.00 | missing | struct.ts — Struct class + assert/create/is/mask/validate signatures | [scheduled bbox exact=6/11] export at src/struct.ts:10 (t=6743, 37 atoms) |
| 3.6 | 1985 | 0.07 | 0.13 | missing | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | [scheduled bbox exact=8/15] export names surface in src/struct.ts (t=5627, 12 atoms) |
| 4.3 | 2854 | 0.00 | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) | [unscheduled bbox exact=9/31] imports in examples/basic-validation.js (9 atoms, too expensive at final margin) |
| 4.5 | 3875 | 0.18 | 0.02 | missing | core.md — `assert` / `create` / `validate` definitions | [scheduled bbox exact=6/34] headings outline in docs/reference/core.md (t=1358, 10 atoms); better unscheduled exact=7/34: docs/reference/core.md section #5 (7 atoms, too expensive at final margin) |
| 4.6 | 4064 | 0.13 | 0.03 | missing | coercions.md — `defaulted` worked example | [scheduled bbox exact=2/15] headings outline in docs/reference/coercions.md (t=1146, 2 atoms); better unscheduled exact=11/15: docs/reference/coercions.md section #1 (11 atoms, too expensive at final margin) |
| 5.1 | 4816 | 0.00 | 0.00 | missing | utils.ts run() — the central traversal generator | [scheduled bbox exact=12/67] export at src/utils.ts:130 (t=9644, 12 atoms); better unscheduled exact=47/67: export body at src/utils.ts:130 body 141 (47 atoms, too expensive at final margin) |
| 5.2 | 5074 | 0.00 | 0.00 | missing | struct.ts validate() body | [scheduled bbox exact=10/27] export at src/struct.ts:185 (t=5937, 10 atoms); better unscheduled exact=15/27: export body at src/struct.ts:185 body 194 (15 atoms, too expensive at final margin) |
| 5.3 | 5610 | 0.00 | 0.00 | missing | struct.ts Struct class constructor + 4 hook fields | [scheduled bbox exact=28/51] export body at src/struct.ts:10 body 30 (t=7151, 28 atoms) |
| 5.6 | 6716 | 0.28 | 0.26 | missing | refine() implementation | [scheduled bbox exact=6/18] export at src/structs/refinements.ts:146 (t=2979, 6 atoms); better unscheduled exact=11/18: export body at src/structs/refinements.ts:146 body 151 (11 atoms, too expensive at final margin) |
| 5.7 | 7243 | 0.36 | 0.39 | missing | coerce() + defaulted() implementations | [scheduled bbox exact=8/55] export body at src/structs/coercions.ts:16 body 21 (t=2669, 8 atoms); better unscheduled exact=20/55: export body at src/structs/coercions.ts:38 body 45 (20 atoms, too expensive at final margin) |
| 6.4 | 8999 | 0.00 | 0.00 | missing | test/index.test.ts — the validation-fixture harness | [unscheduled bbox exact=9/81] imports in test/index.test.ts (9 atoms, predecessor not scheduled: Typescript(Imports { file: "/Users/yoav/projects/precis/tests/fixtures/superstruct/test/index.ts" })) |
| 6.5 | 9499 | 0.00 | 0.00 | missing | types.ts — `object` and `type` reference bodies | [unscheduled bbox exact=18/42] docs/reference/types.md section #23 (18 atoms, predecessor not scheduled: headings outline in docs/reference/types.md) |
| 6.6 | 9863 | 0.00 | 0.00 | missing | examples/default-values.js — defaulted + create | [unscheduled bbox exact=9/38] imports in examples/default-values.js (9 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 6.3 | 8099 | 0.00 | 0.00 | missing | Sample validation fixture — three exemplar modules | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.4 | 802 | 0.30 | 0.30 | missing | examples/ + test/ + test/api/ tree | fs-only |
| 6.1 | 7559 | 0.00 | 0.00 | missing | test/validation/ tree — all 41 kind directories | fs-only |
| 6.2 | 7742 | 0.00 | 0.00 | missing | test/typings/ tree — all type-level test files | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 982 | Readme.md section #<n> |
| 4 | 598 | export at src/utils.ts:<n> |
| 3 | 176 | export at src/structs/refinements.ts:<n> |
| 2 | 166 | export doc at src/struct.ts:<n> |
| 2 | 115 | export doc at src/utils.ts:<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 249 | 0.65 | 381 | 886 | package identity in package.json |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 884 | 1.00 | 884 | 8973 | Readme.md section #0 |
| 434 | 1.00 | 434 | 3973 | headings outline in Changelog.md |
| 422 | 1.00 | 422 | 7979 | export at src/utils.ts:334 |
| 324 | 1.00 | 324 | 4665 | package scripts in package.json |
| 269 | 0.89 | 301 | 5215 | package dependencies in package.json |
| 222 | 0.94 | 236 | 9231 | export names surface in src/utils.ts |
| 109 | 1.00 | 109 | 5419 | export doc at src/error.ts:25 |
| 107 | 1.00 | 107 | 4135 | export body at src/structs/refinements.ts:33 body 40 |
| 107 | 1.00 | 107 | 4242 | export body at src/structs/refinements.ts:55 body 62 |
| 98 | 1.00 | 98 | 4834 | Readme.md section #7 |
| 1332 | — | — | — | +20 more rows |
