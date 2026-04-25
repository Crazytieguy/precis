scores: Sim=0.433 Reached=13/39 Early=2 Late=10 Partial=1 Missing=25 Used=9958/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 3 | 0 | 2 | 0.60 |
| 2 | 6 | 4 | 1 | 1 | 0.79 |
| 3 | 7 | 6 | 0 | 1 | 0.85 |
| 4 | 6 | 0 | 0 | 6 | 0.00 |
| 5 | 8 | 0 | 0 | 8 | 0.17 |
| 6 | 7 | 0 | 0 | 7 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 96 | — | — | 0.00 | missing | Readme one-line lede |  |
| 1.2 | 161 | 65 | -96 | 1.00 | early | Repo root listing |  |
| 1.3 | 240 | 1689 | +1449 | 1.00 | late | Public entry — src/index.ts in full | imports in src/index.ts (t=1689, 6 atoms) |
| 1.4 | 306 | — | — | 0.00 | missing | Readme runtime-errors paragraph |  |
| 1.5 | 394 | 782 | +388 | 1.00 | late | package.json — name, version, license | package identity in package.json (t=782, 5 atoms) |
| 2.1 | 431 | 1507 | +1076 | 1.00 | late | src/ tree (flat layout) |  |
| 2.2 | 583 | 7918 | +7335 | 1.00 | late | package.json — module shape & engines | package entrypoints in package.json (t=858, 7 atoms) |
| 2.3 | 694 | 2027 | +1333 | 1.00 | late | docs/ subtree listings |  |
| 2.4 | 802 | — | — | 0.00 | missing | examples/ + test/ + test/api/ tree |  |
| 2.5 | 851 | 4976 | +4125 | 1.00 | late | Readme section headings (locations only) | Readme.md section #0 (t=9062, 76 atoms) |
| 2.6 | 1168 | — | — | 0.76 | partial | docs/summary.md (the GitBook TOC) | mdBook SUMMARY at docs/summary.md (t=1976, 19 atoms) |
| 3.1 | 1448 | — | — | 0.00 | missing | structs/types.ts — all 24 type-factory names |  |
| 3.2 | 1533 | 2761 | +1228 | 1.00 | late | structs/refinements.ts — all 7 refinement names | export names surface in src/structs/refinements.ts (t=2761, 13 atoms) |
| 3.3 | 1571 | 1563 | -8 | 1.00 | aligned+over | structs/coercions.ts — coerce / defaulted / trimmed | export at src/structs/coercions.ts:38 (t=4319, 8 atoms) |
| 3.4 | 1682 | 6489 | +4807 | 1.00 | late | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | export names surface in src/structs/utilities.ts (t=6489, 25 atoms) |
| 3.5 | 1789 | 6039 | +4250 | 1.00 | late | struct.ts — Struct class + assert/create/is/mask/validate signatures | export at src/struct.ts:10 (t=6039, 37 atoms) |
| 3.6 | 1985 | 3631 | +1646 | 1.00 | late | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | export names surface in src/struct.ts (t=3560, 13 atoms) |
| 3.7 | 2170 | 949 | -1221 | 0.95 | early | error.ts — Failure type + StructError class signature | export at src/error.ts:5 (t=401, 10 atoms) |
| 4.1 | 2380 | — | — | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) |  |
| 4.2 | 2578 | — | — | 0.00 | missing | docs/reference/{refinements,coercions,utilities,core,typescript}.md headings |  |
| 4.3 | 2854 | — | — | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) |  |
| 4.4 | 3305 | — | — | 0.00 | missing | errors.md — StructError property table |  |
| 4.5 | 3875 | — | — | 0.00 | missing | core.md — `assert` / `create` / `validate` definitions |  |
| 4.6 | 4064 | — | — | 0.00 | missing | coercions.md — `defaulted` worked example |  |
| 5.1 | 4816 | — | — | 0.02 | missing | utils.ts run() — the central traversal generator | export names surface in src/utils.ts (t=9607, 2 atoms) |
| 5.2 | 5074 | — | — | 0.33 | missing | struct.ts validate() body | export at src/struct.ts:185 (t=3828, 10 atoms) |
| 5.3 | 5610 | — | — | 0.35 | missing | struct.ts Struct class constructor + 4 hook fields | export at src/struct.ts:10 (t=6039, 19 atoms) |
| 5.4 | 6093 | — | — | 0.00 | missing | object() implementation |  |
| 5.5 | 6541 | — | — | 0.00 | missing | union() implementation |  |
| 5.6 | 6716 | — | — | 0.28 | missing | refine() implementation | export at src/structs/refinements.ts:146 (t=3048, 6 atoms) |
| 5.7 | 7243 | — | — | 0.33 | missing | coerce() + defaulted() implementations | export at src/structs/coercions.ts:38 (t=4319, 8 atoms) |
| 5.8 | 7434 | — | — | 0.07 | missing | error.ts StructError constructor | export at src/error.ts:25 (t=949, 2 atoms) |
| 6.1 | 7559 | — | — | 0.00 | missing | test/validation/ tree — all 41 kind directories |  |
| 6.2 | 7742 | — | — | 0.00 | missing | test/typings/ tree — all type-level test files |  |
| 6.3 | 8099 | — | — | 0.00 | missing | Sample validation fixture — three exemplar modules |  |
| 6.4 | 8999 | — | — | 0.00 | missing | test/index.test.ts — the validation-fixture harness |  |
| 6.5 | 9499 | — | — | 0.00 | missing | types.ts — `object` and `type` reference bodies |  |
| 6.6 | 9863 | — | — | 0.00 | missing | examples/default-values.js — defaulted + create |  |
| 6.7 | 9978 | — | — | 0.00 | missing | Guide H2 headings — all 14 across guides 02-06 |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 6 | 2302 | Readme.md section #<n> |
| 7 | 484 | export doc at src/structs/utilities.ts:<n> |
| 3 | 434 | export at src/structs/utilities.ts:<n> |
| 2 | 166 | export doc at src/struct.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 884 | 1.00 | 884 | 9062 | Readme.md section #0 |
| 525 | 0.96 | 545 | 9607 | export names surface in src/utils.ts |
| 460 | 0.95 | 484 | 5460 | Readme.md section #5 |
| 431 | 0.91 | 475 | 4976 | Readme.md section #1 |
| 324 | 1.00 | 324 | 2351 | package scripts in package.json |
| 295 | 0.78 | 381 | 782 | package identity in package.json |
| 266 | 0.86 | 311 | 2662 | Readme.md section #2 |
| 255 | 0.85 | 301 | 7918 | package dependencies in package.json |
| 185 | 1.00 | 185 | 1389 | json config tsconfig.json |
| 183 | 0.92 | 200 | 1204 | Readme.md section #4 |
| 182 | 1.00 | 182 | 7163 | export at src/structs/utilities.ts:44 |
| 157 | 1.00 | 157 | 8178 | Changelog.md section #2 |
| 150 | 1.00 | 150 | 6981 | export at src/structs/utilities.ts:30 |
| 142 | 0.65 | 218 | 6489 | export names surface in src/structs/utilities.ts |
| 121 | 0.32 | 376 | 6039 | export at src/struct.ts:10 |
| 110 | 1.00 | 110 | 3158 | json config jsr.json |
| 109 | 1.00 | 109 | 3267 | export doc at src/error.ts:25 |
| 103 | 1.00 | 103 | 8021 | export doc at src/structs/utilities.ts:140 |
| 102 | 1.00 | 102 | 6673 | export at src/structs/utilities.ts:21 |
| 97 | 1.00 | 97 | 4262 | export doc at src/struct.ts:221 |
| 91 | 1.00 | 91 | 6271 | export doc at src/structs/refinements.ts:146 |
| 81 | 1.00 | 81 | 7617 | export doc at src/structs/utilities.ts:106 |
| 78 | 0.75 | 105 | 219 | Readme.md section #3 |
| 77 | 1.00 | 77 | 5581 | export doc at src/structs/coercions.ts:79 |
| 73 | 0.32 | 231 | 3560 | export names surface in src/struct.ts |
| 69 | 1.00 | 69 | 6108 | export doc at src/struct.ts:10 |
| 68 | 1.00 | 68 | 7536 | export doc at src/structs/utilities.ts:17 |
| 63 | 1.00 | 63 | 7468 | export doc at src/structs/utilities.ts:197 |
| 57 | 1.00 | 57 | 7301 | export doc at src/structs/utilities.ts:171 |
| 57 | 1.00 | 57 | 7358 | export doc at src/structs/utilities.ts:221 |
| 55 | 1.00 | 55 | 7244 | export doc at src/structs/utilities.ts:80 |
