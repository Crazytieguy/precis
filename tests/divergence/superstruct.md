scores: Score(3000)=0.583 ns_rows≤3K=21/39 (reached=10 partial=2 missing=9)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 106 | 0.650 | 0.494 | 0.872 | 0.567 | 991 |
| 1442 | 131 | 0.699 | 0.504 | 0.938 | 0.594 | 1431 |
| 2080 | 201 | 0.704 | 0.489 | 0.821 | 0.587 | 2063 |
| 3000 | 301 | 0.687 | 0.494 | 0.819 | 0.583 | 2979 |
| 4327 | 363 | 0.668 | 0.410 | 0.686 | 0.523 | 4311 |
| 6240 | 556 | 0.690 | 0.355 | 0.727 | 0.495 | 6234 |
| 9000 | 898 | 0.663 | 0.302 | 0.753 | 0.448 | 8995 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 18 | 2.53 | 2.36 | 2.03 | nearby candidates have low exact atom overlap | 1.1, 3.1, 5.1, 4.3, 5.3, ... |
| tune ranking for high-overlap unscheduled candidates | 3 | 0.21 | 0.21 | 0.21 | high-overlap candidates not in the schedule by T_max, exact total=45/48 | 4.1, 3.4, 4.4 |
| promote export batches | 2 | 0.17 | 0.17 | 0.17 | 1 file, exact total=80/95 | 5.4, 5.5 |
| add walker candidates for no-discovered rows | 1 | 0.05 | 0.05 | 0.05 | NS rows have no discovered line candidate | 6.3 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| package identity in package.json | 1 | 249 | 249 | 249 | off_3k=249 | package identity in package.json |
| headings outline in docs/guides/02-validating-data.md | 1 | 0 | 19 | 19 | off_3k=52 | headings outline in docs/guides/02-validating-data.md |

Top missed paths (NS rows ≤ 3K): examples/basic-validation.js (1 row, 31 atoms), src/struct.ts (2 rows, 26 atoms), docs/reference/types.md (1 row, 26 atoms), docs/summary.md (1 row, 25 atoms), src/structs/types.ts (1 row, 25 atoms), examples (1 row, 23 atoms), package.json (1 row, 14 atoms), src/structs/utilities.ts (1 row, 9 atoms), +1 more

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.4 | 1682 | 0.11 | missing | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | [scheduled bbox exact=0/9] export doc at src/structs/utilities.ts:247 (t=5262, 5 atoms); better unscheduled exact=8/9: export names surface in src/structs/utilities.ts (24 atoms, too expensive at final margin) |
| 4.1 | 2380 | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) | [unscheduled bbox exact=26/26] headings outline in docs/reference/types.md (51 atoms, too expensive at final margin) |
| 4.4 | 3305 | 0.15 | missing | errors.md — StructError property table | [scheduled bbox exact=2/13] headings outline in docs/reference/errors.md (t=1101, 2 atoms); better unscheduled exact=11/13: docs/reference/errors.md section #2 (11 atoms, too expensive at final margin) |
| 5.4 | 6093 | 0.00 | missing | object() implementation | [unscheduled bbox exact=41/48] export body at src/structs/types.ts:298 body 299 (41 atoms, predecessor not scheduled: export at src/structs/types.ts:298) |
| 5.5 | 6541 | 0.00 | missing | union() implementation | [unscheduled bbox exact=39/47] export body at src/structs/types.ts:522 body 525 (39 atoms, predecessor not scheduled: export at src/structs/types.ts:522) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.1 | 96 | 0.00 | missing | Readme one-line lede | [scheduled same-file] Readme.md section #0 (t=8973, 76 atoms) |
| 1.4 | 306 | 0.00 | missing | Readme runtime-errors paragraph | [scheduled same-file] Readme.md section #0 (t=8973, 76 atoms) |
| 2.2 | 583 | 0.79 | partial | package.json — module shape & engines | [scheduled bbox exact=7/14] package entrypoints in package.json (t=962, 7 atoms) |
| 2.6 | 1168 | 0.76 | partial | docs/summary.md (the GitBook TOC) | [scheduled bbox exact=19/25] mdBook SUMMARY at docs/summary.md (t=1854, 19 atoms) |
| 3.1 | 1448 | 0.00 | missing | structs/types.ts — all 24 type-factory names | [unscheduled bbox exact=8/25] export names surface in src/structs/types.ts (22 atoms, too expensive at final margin) |
| 3.5 | 1789 | 0.00 | missing | struct.ts — Struct class + assert/create/is/mask/validate signatures | [scheduled bbox exact=6/11] export at src/struct.ts:10 (t=6743, 37 atoms) |
| 3.6 | 1985 | 0.07 | missing | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | [scheduled bbox exact=8/15] export names surface in src/struct.ts (t=5627, 12 atoms) |
| 4.3 | 2854 | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) | [unscheduled bbox exact=9/31] imports in examples/basic-validation.js (9 atoms, too expensive at final margin) |
| 4.5 | 3875 | 0.18 | missing | core.md — `assert` / `create` / `validate` definitions | [scheduled bbox exact=6/34] headings outline in docs/reference/core.md (t=1358, 10 atoms); better unscheduled exact=7/34: docs/reference/core.md section #5 (7 atoms, too expensive at final margin) |
| 4.6 | 4064 | 0.13 | missing | coercions.md — `defaulted` worked example | [scheduled bbox exact=2/15] headings outline in docs/reference/coercions.md (t=1146, 2 atoms); better unscheduled exact=11/15: docs/reference/coercions.md section #1 (11 atoms, too expensive at final margin) |
| 5.1 | 4816 | 0.00 | missing | utils.ts run() — the central traversal generator | [scheduled bbox exact=12/67] export at src/utils.ts:130 (t=9644, 12 atoms); better unscheduled exact=47/67: export body at src/utils.ts:130 body 141 (47 atoms, too expensive at final margin) |
| 5.2 | 5074 | 0.00 | missing | struct.ts validate() body | [scheduled bbox exact=10/27] export at src/struct.ts:185 (t=5937, 10 atoms); better unscheduled exact=15/27: export body at src/struct.ts:185 body 194 (15 atoms, too expensive at final margin) |
| 5.3 | 5610 | 0.00 | missing | struct.ts Struct class constructor + 4 hook fields | [scheduled bbox exact=28/51] export body at src/struct.ts:10 body 30 (t=7151, 28 atoms) |
| 5.6 | 6716 | 0.28 | missing | refine() implementation | [scheduled bbox exact=6/18] export at src/structs/refinements.ts:146 (t=2979, 6 atoms); better unscheduled exact=11/18: export body at src/structs/refinements.ts:146 body 151 (11 atoms, too expensive at final margin) |
| 5.7 | 7243 | 0.36 | missing | coerce() + defaulted() implementations | [scheduled bbox exact=8/55] export body at src/structs/coercions.ts:16 body 21 (t=2669, 8 atoms); better unscheduled exact=20/55: export body at src/structs/coercions.ts:38 body 45 (20 atoms, too expensive at final margin) |
| 6.4 | 8999 | 0.00 | missing | test/index.test.ts — the validation-fixture harness | [unscheduled bbox exact=9/81] imports in test/index.test.ts (9 atoms, predecessor not scheduled: Typescript(Imports { file: "/Users/yoav/projects/precis/tests/fixtures/superstruct/test/index.ts" })) |
| 6.5 | 9499 | 0.00 | missing | types.ts — `object` and `type` reference bodies | [unscheduled bbox exact=18/42] docs/reference/types.md section #23 (18 atoms, predecessor not scheduled: headings outline in docs/reference/types.md) |
| 6.6 | 9863 | 0.00 | missing | examples/default-values.js — defaulted + create | [unscheduled bbox exact=9/38] imports in examples/default-values.js (9 atoms, too expensive at final margin) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 6.3 | 8099 | 0.00 | missing | Sample validation fixture — three exemplar modules | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.4 | 802 | 0.30 | missing | examples/ + test/ + test/api/ tree | fs-only |
| 6.1 | 7559 | 0.00 | missing | test/validation/ tree — all 41 kind directories | fs-only |
| 6.2 | 7742 | 0.00 | missing | test/typings/ tree — all type-level test files | fs-only |

Top wasted paths (off-NS at 3K): package.json (249t, 1 batch), src/error.ts (163t, 1 batch), src/structs/coercions.ts (140t, 2 batches), docs/guides/02-validating-data.md (52t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 249 | 0.65 | 249 | 381 | 505 | package identity in package.json |
| 163 | 1.00 | 0 | 163 | 2200 | export body at src/error.ts:25 body 36 |
| 83 | 1.00 | 0 | 83 | 2586 | export body at src/structs/coercions.ts:16 body 21 |
| 57 | 1.00 | 0 | 57 | 1478 | export at src/structs/coercions.ts:38 |
| 52 | 1.00 | 19 | 52 | 2107 | headings outline in docs/guides/02-validating-data.md |
