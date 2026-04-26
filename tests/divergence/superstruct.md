scores: Sim=0.473 Reached=19/39 Early=5 Late=10 Partial=1 Missing=19 Used=9965/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 3 | 0 | 2 | 0.60 |
| 2 | 6 | 4 | 1 | 1 | 0.79 |
| 3 | 7 | 6 | 0 | 1 | 0.85 |
| 4 | 6 | 1 | 0 | 5 | 0.24 |
| 5 | 8 | 4 | 0 | 4 | 0.50 |
| 6 | 7 | 1 | 0 | 6 | 0.14 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 96 | — | — | 0.00 | missing | Readme one-line lede |  |
| 1.2 | 161 | 65 | -96 | 1.00 | early | Repo root listing |  |
| 1.3 | 240 | 1763 | +1523 | 1.00 | late | Public entry — src/index.ts in full | imports in src/index.ts (t=1763, 6 atoms) |
| 1.4 | 306 | — | — | 0.00 | missing | Readme runtime-errors paragraph |  |
| 1.5 | 394 | 863 | +469 | 1.00 | late | package.json — name, version, license | package identity in package.json (t=863, 5 atoms) |
| 2.2 | 583 | 9918 | +9335 | 1.00 | late | package.json — module shape & engines | package entrypoints in package.json (t=939, 7 atoms) |
| 2.3 | 694 | 2192 | +1498 | 1.00 | late | docs/ subtree listings |  |
| 2.4 | 802 | — | — | 0.00 | missing | examples/ + test/ + test/api/ tree |  |
| 2.5 | 851 | 260 | -591 | 1.00 | early | Readme section headings (locations only) | Readme.md section #8 (t=4103, 12 atoms) |
| 2.6 | 1168 | — | — | 0.76 | partial | docs/summary.md (the GitBook TOC) | mdBook SUMMARY at docs/summary.md (t=2141, 19 atoms) |
| 3.1 | 1448 | — | — | 0.00 | missing | structs/types.ts — all 24 type-factory names |  |
| 3.2 | 1533 | 2844 | +1311 | 1.00 | late | structs/refinements.ts — all 7 refinement names | export names surface in src/structs/refinements.ts (t=2844, 13 atoms) |
| 3.3 | 1571 | 1122 | -449 | 1.00 | aligned+over | structs/coercions.ts — coerce / defaulted / trimmed | export at src/structs/coercions.ts:38 (t=1243, 8 atoms) |
| 3.4 | 1682 | 8322 | +6640 | 1.00 | late | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | export names surface in src/structs/utilities.ts (t=8322, 25 atoms) |
| 3.5 | 1789 | 5998 | +4209 | 1.00 | late | struct.ts — Struct class + assert/create/is/mask/validate signatures | export at src/struct.ts:10 (t=5998, 37 atoms) |
| 3.6 | 1985 | 4602 | +2617 | 1.00 | late | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | export names surface in src/struct.ts (t=4531, 13 atoms) |
| 3.7 | 2170 | 482 | -1688 | 0.95 | early | error.ts — Failure type + StructError class signature | export at src/error.ts:5 (t=391, 10 atoms) |
| 4.1 | 2380 | — | — | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) |  |
| 4.2 | 2578 | 3660 | +1082 | 1.00 | late | docs/reference/{refinements,coercions,utilities,core,typescript}.md headings | headings outline in docs/reference/utilities.md (t=3578, 13 atoms) |
| 4.3 | 2854 | — | — | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) |  |
| 4.4 | 3305 | — | — | 0.15 | missing | errors.md — StructError property table | headings outline in docs/reference/errors.md (t=1353, 2 atoms) |
| 4.5 | 3875 | — | — | 0.18 | missing | core.md — `assert` / `create` / `validate` definitions | headings outline in docs/reference/core.md (t=1822, 10 atoms) |
| 4.6 | 4064 | — | — | 0.13 | missing | coercions.md — `defaulted` worked example | headings outline in docs/reference/coercions.md (t=1398, 2 atoms) |
| 5.1 | 4816 | — | — | 0.00 | missing | utils.ts run() — the central traversal generator |  |
| 5.2 | 5074 | 5492 | +418 | 0.89 | aligned | struct.ts validate() body | export body at src/struct.ts:185 (t=5492, 15 atoms) |
| 5.3 | 5610 | 8104 | +2494 | 0.90 | late | struct.ts Struct class constructor + 4 hook fields | export body at src/struct.ts:10 (t=8104, 28 atoms) |
| 5.4 | 6093 | — | — | 0.00 | missing | object() implementation |  |
| 5.5 | 6541 | — | — | 0.00 | missing | union() implementation |  |
| 5.6 | 6716 | 6898 | +182 | 0.89 | aligned | refine() implementation | export body at src/structs/refinements.ts:146 (t=6898, 11 atoms) |
| 5.7 | 7243 | — | — | 0.36 | missing | coerce() + defaulted() implementations | export at src/structs/coercions.ts:38 (t=1243, 8 atoms) |
| 5.8 | 7434 | 3499 | -3935 | 0.93 | early | error.ts StructError constructor | export body at src/error.ts:25 (t=3499, 12 atoms) |
| 6.1 | 7559 | — | — | 0.00 | missing | test/validation/ tree — all 41 kind directories |  |
| 6.2 | 7742 | — | — | 0.00 | missing | test/typings/ tree — all type-level test files |  |
| 6.3 | 8099 | — | — | 0.00 | missing | Sample validation fixture — three exemplar modules |  |
| 6.4 | 8999 | — | — | 0.00 | missing | test/index.test.ts — the validation-fixture harness |  |
| 6.5 | 9499 | — | — | 0.00 | missing | types.ts — `object` and `type` reference bodies |  |
| 6.6 | 9863 | — | — | 0.00 | missing | examples/default-values.js — defaulted + create |  |
| 6.7 | 9978 | 2421 | -7557 | 1.00 | early | Guide H2 headings — all 14 across guides 02-06 | headings outline in docs/guides/02-validating-data.md (t=2421, 7 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 759 | Readme.md section #<n> |
| 3 | 434 | export at src/structs/utilities.ts:<n> |
| 3 | 176 | export at src/structs/refinements.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 468 | 1.00 | 468 | 7500 | Readme.md section #1 |
| 434 | 1.00 | 434 | 6432 | headings outline in Changelog.md |
| 324 | 1.00 | 324 | 2745 | package scripts in package.json |
| 269 | 0.89 | 301 | 9918 | package dependencies in package.json |
| 249 | 0.65 | 381 | 863 | package identity in package.json |
| 193 | 1.00 | 193 | 4103 | Readme.md section #8 |
| 185 | 1.00 | 185 | 1656 | json config tsconfig.json |
| 182 | 1.00 | 182 | 9379 | export at src/structs/utilities.ts:44 |
| 150 | 1.00 | 150 | 8971 | export at src/structs/utilities.ts:30 |
| 136 | 1.00 | 136 | 7765 | Changelog.md section #2 |
| 119 | 1.00 | 119 | 9617 | export body at src/structs/utilities.ts:171 |
| 119 | 1.00 | 119 | 9498 | export body at src/structs/utilities.ts:80 |
| 113 | 1.00 | 113 | 9084 | export body at src/structs/utilities.ts:197 |
| 113 | 1.00 | 113 | 9197 | export body at src/structs/utilities.ts:221 |
| 110 | 1.00 | 110 | 3804 | json config jsr.json |
| 107 | 1.00 | 107 | 6603 | export body at src/structs/refinements.ts:33 |
| 107 | 1.00 | 107 | 6710 | export body at src/structs/refinements.ts:55 |
| 102 | 1.00 | 102 | 8730 | export at src/structs/utilities.ts:21 |
| 98 | 1.00 | 98 | 1066 | Readme.md section #7 |
| 82 | 0.22 | 376 | 5998 | export at src/struct.ts:10 |
| 82 | 1.00 | 82 | 3910 | export body at src/structs/refinements.ts:8 |
| 72 | 1.00 | 72 | 3336 | export body at src/structs/refinements.ts:77 |
| 71 | 1.00 | 71 | 3264 | export body at src/structs/refinements.ts:93 |
| 69 | 1.00 | 69 | 4997 | export body at src/struct.ts:157 |
| 69 | 1.00 | 69 | 6779 | export doc at src/struct.ts:10 |
| 67 | 0.31 | 218 | 8322 | export names surface in src/structs/utilities.ts |
| 65 | 1.00 | 65 | 4928 | export body at src/struct.ts:139 |
| 65 | 1.00 | 65 | 8795 | export body at src/structs/utilities.ts:60 |
| 62 | 1.00 | 62 | 3193 | export at src/structs/refinements.ts:109 |
| 57 | 1.00 | 57 | 3074 | export at src/structs/refinements.ts:33 |
| 57 | 1.00 | 57 | 3131 | export at src/structs/refinements.ts:55 |
| 53 | 0.16 | 339 | 8104 | export body at src/struct.ts:10 |
