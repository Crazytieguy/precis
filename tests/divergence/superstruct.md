scores: Sim=0.496 Reached=19/39 Early=6 Late=8 Partial=1 Missing=19 Used=9190/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 5 ranking-recoverable (w×gap=1.04), 8 wrong-slice/granularity (w×gap=2.29), 4 no-discovered (w×gap=0.28)
Secondary intervention: promote predecessors for 2 gated candidates
Loss reasons: 2 predecessor-gated, 0 too-expensive, 3 discovered-unscheduled
Top rows: 1.1, 1.4, 2.6, 4.5, 4.6, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 8 | 2.29 | 3/6/8 | nearby candidates have low exact atom overlap | 1.1, 1.4, 2.6, 4.5, 4.6, ... |
| tune ranking for discovered unscheduled candidates | 3 | 0.95 | 2/3/3 | high-overlap candidates fit but did not win, exact total=62/64 | 3.1, 4.1, 4.4 |
| add walker candidates for no-discovered rows | 4 | 0.28 | 1/1/4 | NS rows have no discovered line candidate | 4.3, 6.3, 6.4, 6.6 |
| promote export batches | 2 | 0.09 | 0/0/2 | 1 file, exact total=80/95 | 5.4, 5.5 |

Tiers: 1=3/5 reached, 0 partial, 2 missing, avg=0.60; 2=4/6 reached, 1 partial, 1 missing, avg=0.79; 3=6/7 reached, 0 partial, 1 missing, avg=0.85; 4=1/6 reached, 0 partial, 5 missing, avg=0.24; 5=4/8 reached, 0 partial, 4 missing, avg=0.50; 6=1/7 reached, 0 partial, 6 missing, avg=0.14

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 5 | 5 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 8 | 7 | 1 | 0 | walker granularity / wrong slice |
| no discovered candidate | 4 | 4 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 3 | 3 | 0 | 0 | filesystem/listing value |
| timing-only | 17 | 0 | 0 | 17 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.09 | promote predecessor |
| discovered unscheduled | 3 | 0.95 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=20, unscheduled bbox=6, scheduled same-file=2, fs-only=5, no discovered candidate=4

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 3 |
| scheduled bbox | early | none | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | high | 2 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | full | 4 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | low | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.1 | 1448 | — | — | 0.00 | missing | structs/types.ts — all 24 type-factory names | [unscheduled bbox exact=25/25] export names surface in src/structs/types.ts (67 atoms, discovered unscheduled) |
| 4.1 | 2380 | — | — | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) | [unscheduled bbox exact=26/26] headings outline in docs/reference/types.md (51 atoms, discovered unscheduled) |
| 4.4 | 3305 | — | — | 0.15 | missing | errors.md — StructError property table | [scheduled bbox exact=2/13] headings outline in docs/reference/errors.md (t=1255, 2 atoms); better unscheduled exact=11/13: docs/reference/errors.md section #2 (11 atoms, discovered unscheduled) |
| 5.4 | 6093 | — | — | 0.00 | missing | object() implementation | [unscheduled bbox exact=41/48] export body at src/structs/types.ts:298 (41 atoms, predecessor not scheduled: export at src/structs/types.ts:298) |
| 5.5 | 6541 | — | — | 0.00 | missing | union() implementation | [unscheduled bbox exact=39/47] export body at src/structs/types.ts:522 (39 atoms, predecessor not scheduled: export at src/structs/types.ts:522) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 96 | — | — | 0.00 | missing | Readme one-line lede | [scheduled same-file] headings outline in Readme.md (t=260, 12 atoms) |
| 1.4 | 306 | — | — | 0.00 | missing | Readme runtime-errors paragraph | [scheduled same-file] headings outline in Readme.md (t=260, 12 atoms) |
| 2.6 | 1168 | — | — | 0.76 | partial | docs/summary.md (the GitBook TOC) | [scheduled bbox exact=19/25] mdBook SUMMARY at docs/summary.md (t=1831, 19 atoms) |
| 4.5 | 3875 | — | — | 0.18 | missing | core.md — `assert` / `create` / `validate` definitions | [scheduled bbox exact=6/34] headings outline in docs/reference/core.md (t=1512, 10 atoms); better unscheduled exact=7/34: docs/reference/core.md section #5 (7 atoms, discovered unscheduled) |
| 4.6 | 4064 | — | — | 0.13 | missing | coercions.md — `defaulted` worked example | [scheduled bbox exact=2/15] headings outline in docs/reference/coercions.md (t=1300, 2 atoms); better unscheduled exact=11/15: docs/reference/coercions.md section #1 (11 atoms, discovered unscheduled) |
| 5.1 | 4816 | — | — | 0.00 | missing | utils.ts run() — the central traversal generator | [unscheduled bbox exact=47/67] export body at src/utils.ts:130 (47 atoms, predecessor not scheduled: export at src/utils.ts:130) |
| 5.7 | 7243 | — | — | 0.36 | missing | coerce() + defaulted() implementations | [scheduled bbox exact=8/55] export body at src/structs/coercions.ts:16 (t=3249, 8 atoms); better unscheduled exact=20/55: export body at src/structs/coercions.ts:38 (20 atoms, discovered unscheduled) |
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
| 1.3 | 240 | 1453 | +1213 | 1.00 | late | Public entry — src/index.ts in full | [scheduled bbox exact=6/6] imports in src/index.ts (t=1453, 6 atoms) |
| 1.5 | 394 | 863 | +469 | 1.00 | late | package.json — name, version, license | [scheduled bbox exact=5/5] package identity in package.json (t=863, 5 atoms) |
| 2.2 | 583 | 8697 | +8114 | 1.00 | late | package.json — module shape & engines | [scheduled bbox exact=7/14] package entrypoints in package.json (t=939, 7 atoms) |
| 2.3 | 694 | 1882 | +1188 | 1.00 | late | docs/ subtree listings | fs-only |
| 2.5 | 851 | 260 | -591 | 1.00 | early | Readme section headings (locations only) | [scheduled bbox exact=6/7] headings outline in Readme.md (t=260, 11 atoms) |
| 3.2 | 1533 | 2210 | +677 | 1.00 | late | structs/refinements.ts — all 7 refinement names | [scheduled bbox exact=7/7] export names surface in src/structs/refinements.ts (t=2210, 13 atoms) |
| 3.3 | 1571 | 1024 | -547 | 1.00 | early | structs/coercions.ts — coerce / defaulted / trimmed | [scheduled bbox exact=0/3] export body at src/structs/coercions.ts:16 (t=3249, 8 atoms) |
| 3.4 | 1682 | 7101 | +5419 | 1.00 | late | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | [scheduled bbox exact=9/9] export names surface in src/structs/utilities.ts (t=7101, 25 atoms) |
| 3.5 | 1789 | 5061 | +3272 | 1.00 | late | struct.ts — Struct class + assert/create/is/mask/validate signatures | [scheduled bbox exact=6/11] export at src/struct.ts:10 (t=5061, 37 atoms) |
| 3.6 | 1985 | 3665 | +1680 | 1.00 | late | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | [scheduled bbox exact=9/15] export names surface in src/struct.ts (t=3594, 13 atoms) |
| 3.7 | 2170 | 482 | -1688 | 0.95 | early | error.ts — Failure type + StructError class signature | [scheduled bbox exact=10/19] export at src/error.ts:5 (t=391, 10 atoms) |
| 5.2 | 5074 | 4555 | -519 | 0.89 | aligned | struct.ts validate() body | [scheduled bbox exact=15/27] export body at src/struct.ts:185 (t=4555, 15 atoms) |
| 5.3 | 5610 | 6883 | +1273 | 0.90 | aligned | struct.ts Struct class constructor + 4 hook fields | [scheduled bbox exact=28/51] export body at src/struct.ts:10 (t=6883, 28 atoms) |
| 5.6 | 6716 | 5952 | -764 | 0.89 | aligned | refine() implementation | [scheduled bbox exact=11/18] export body at src/structs/refinements.ts:146 (t=5952, 11 atoms) |
| 5.8 | 7434 | 2865 | -4569 | 0.93 | early | error.ts StructError constructor | [scheduled bbox exact=12/14] export body at src/error.ts:25 (t=2865, 12 atoms) |
| 6.7 | 9978 | 2111 | -7867 | 1.00 | early | Guide H2 headings — all 14 across guides 02-06 | [scheduled bbox exact=4/13] headings outline in docs/guides/02-validating-data.md (t=2111, 7 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 434 | export at src/structs/utilities.ts:<n> |
| 3 | 176 | export at src/structs/refinements.ts:<n> |
| 2 | 166 | export doc at src/struct.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 434 | 1.00 | 434 | 5495 | headings outline in Changelog.md |
| 324 | 1.00 | 324 | 6375 | package scripts in package.json |
| 269 | 0.89 | 301 | 8697 | package dependencies in package.json |
| 249 | 0.65 | 381 | 863 | package identity in package.json |
| 182 | 1.00 | 182 | 8158 | export at src/structs/utilities.ts:44 |
| 150 | 1.00 | 150 | 7750 | export at src/structs/utilities.ts:30 |
| 119 | 1.00 | 119 | 8396 | export body at src/structs/utilities.ts:171 |
| 119 | 1.00 | 119 | 8277 | export body at src/structs/utilities.ts:80 |
| 113 | 1.00 | 113 | 7863 | export body at src/structs/utilities.ts:197 |
| 113 | 1.00 | 113 | 7976 | export body at src/structs/utilities.ts:221 |
| 1683 | — | — | — | +22 more rows |
