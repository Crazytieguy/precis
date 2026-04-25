scores: Sim=0.441 Reached=15/39 Early=4 Late=10 Partial=1 Missing=23 Used=9432/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 3 | 0 | 2 | 0.60 |
| 2 | 6 | 4 | 1 | 1 | 0.79 |
| 3 | 7 | 6 | 0 | 1 | 0.85 |
| 4 | 6 | 1 | 0 | 5 | 0.24 |
| 5 | 8 | 0 | 0 | 8 | 0.17 |
| 6 | 7 | 1 | 0 | 6 | 0.14 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 96 | — | — | 0.00 | missing | Readme one-line lede |  |
| 1.2 | 161 | 65 | -96 | 1.00 | early | Repo root listing |  |
| 1.3 | 240 | 1882 | +1642 | 1.00 | late | Public entry — src/index.ts in full | imports in src/index.ts (t=1882, 6 atoms) |
| 1.4 | 306 | — | — | 0.00 | missing | Readme runtime-errors paragraph |  |
| 1.5 | 394 | 822 | +428 | 1.00 | late | package.json — name, version, license | package identity in package.json (t=822, 5 atoms) |
| 2.1 | 431 | 1700 | +1269 | 1.00 | late | src/ tree (flat layout) |  |
| 2.2 | 583 | 9121 | +8538 | 1.00 | late | package.json — module shape & engines | package entrypoints in package.json (t=898, 7 atoms) |
| 2.3 | 694 | 2220 | +1526 | 1.00 | late | docs/ subtree listings |  |
| 2.4 | 802 | — | — | 0.00 | missing | examples/ + test/ + test/api/ tree |  |
| 2.5 | 851 | 145 | -706 | 1.00 | early | Readme section headings (locations only) | Readme.md section #5 (t=6081, 21 atoms) |
| 2.6 | 1168 | — | — | 0.76 | partial | docs/summary.md (the GitBook TOC) | mdBook SUMMARY at docs/summary.md (t=2169, 19 atoms) |
| 3.1 | 1448 | — | — | 0.00 | missing | structs/types.ts — all 24 type-factory names |  |
| 3.2 | 1533 | 3235 | +1702 | 1.00 | late | structs/refinements.ts — all 7 refinement names | export names surface in src/structs/refinements.ts (t=3235, 13 atoms) |
| 3.3 | 1571 | 1756 | +185 | 1.00 | aligned+over | structs/coercions.ts — coerce / defaulted / trimmed | export at src/structs/coercions.ts:38 (t=4954, 8 atoms) |
| 3.4 | 1682 | 7692 | +6010 | 1.00 | late | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | export names surface in src/structs/utilities.ts (t=7692, 25 atoms) |
| 3.5 | 1789 | 7106 | +5317 | 1.00 | late | struct.ts — Struct class + assert/create/is/mask/validate signatures | export at src/struct.ts:10 (t=7106, 37 atoms) |
| 3.6 | 1985 | 4266 | +2281 | 1.00 | late | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | export names surface in src/struct.ts (t=4195, 13 atoms) |
| 3.7 | 2170 | 989 | -1181 | 0.95 | early | error.ts — Failure type + StructError class signature | export at src/error.ts:5 (t=441, 10 atoms) |
| 4.1 | 2380 | — | — | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) |  |
| 4.2 | 2578 | 3964 | +1386 | 1.00 | late | docs/reference/{refinements,coercions,utilities,core,typescript}.md headings | headings outline in docs/reference/refinements.md (t=3885, 13 atoms) |
| 4.3 | 2854 | — | — | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) |  |
| 4.4 | 3305 | — | — | 0.15 | missing | errors.md — StructError property table | headings outline in docs/reference/errors.md (t=1571, 2 atoms) |
| 4.5 | 3875 | — | — | 0.18 | missing | core.md — `assert` / `create` / `validate` definitions | headings outline in docs/reference/core.md (t=3136, 10 atoms) |
| 4.6 | 4064 | — | — | 0.13 | missing | coercions.md — `defaulted` worked example | headings outline in docs/reference/coercions.md (t=1531, 2 atoms) |
| 5.1 | 4816 | — | — | 0.00 | missing | utils.ts run() — the central traversal generator |  |
| 5.2 | 5074 | — | — | 0.33 | missing | struct.ts validate() body | export at src/struct.ts:185 (t=4463, 10 atoms) |
| 5.3 | 5610 | — | — | 0.35 | missing | struct.ts Struct class constructor + 4 hook fields | export at src/struct.ts:10 (t=7106, 19 atoms) |
| 5.4 | 6093 | — | — | 0.00 | missing | object() implementation |  |
| 5.5 | 6541 | — | — | 0.00 | missing | union() implementation |  |
| 5.6 | 6716 | — | — | 0.28 | missing | refine() implementation | export at src/structs/refinements.ts:146 (t=3522, 6 atoms) |
| 5.7 | 7243 | — | — | 0.33 | missing | coerce() + defaulted() implementations | export at src/structs/coercions.ts:38 (t=4954, 8 atoms) |
| 5.8 | 7434 | — | — | 0.07 | missing | error.ts StructError constructor | export at src/error.ts:25 (t=989, 2 atoms) |
| 6.1 | 7559 | — | — | 0.00 | missing | test/validation/ tree — all 41 kind directories |  |
| 6.2 | 7742 | — | — | 0.00 | missing | test/typings/ tree — all type-level test files |  |
| 6.3 | 8099 | — | — | 0.00 | missing | Sample validation fixture — three exemplar modules |  |
| 6.4 | 8999 | — | — | 0.00 | missing | test/index.test.ts — the validation-fixture harness |  |
| 6.5 | 9499 | — | — | 0.00 | missing | types.ts — `object` and `type` reference bodies |  |
| 6.6 | 9863 | — | — | 0.00 | missing | examples/default-values.js — defaulted + create |  |
| 6.7 | 9978 | 2449 | -7529 | 1.00 | early | Guide H2 headings — all 14 across guides 02-06 | headings outline in docs/guides/02-validating-data.md (t=2449, 7 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 1388 | Readme.md section #<n> |
| 7 | 484 | export doc at src/structs/utilities.ts:<n> |
| 3 | 434 | export at src/structs/utilities.ts:<n> |
| 3 | 344 | Changelog.md section #<n> |
| 2 | 166 | export doc at src/struct.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 454 | 0.95 | 477 | 6081 | Readme.md section #5 |
| 434 | 1.00 | 434 | 6515 | headings outline in Changelog.md |
| 425 | 0.91 | 468 | 5604 | Readme.md section #1 |
| 324 | 1.00 | 324 | 2773 | package scripts in package.json |
| 295 | 0.78 | 381 | 822 | package identity in package.json |
| 260 | 0.86 | 304 | 3077 | Readme.md section #2 |
| 255 | 0.85 | 301 | 9121 | package dependencies in package.json |
| 185 | 1.00 | 185 | 1394 | json config tsconfig.json |
| 182 | 1.00 | 182 | 8366 | export at src/structs/utilities.ts:44 |
| 176 | 0.92 | 193 | 1182 | Readme.md section #4 |
| 150 | 1.00 | 150 | 8184 | export at src/structs/utilities.ts:30 |
| 142 | 0.65 | 218 | 7692 | export names surface in src/structs/utilities.ts |
| 136 | 1.00 | 136 | 7383 | Changelog.md section #2 |
| 121 | 0.32 | 376 | 7106 | export at src/struct.ts:10 |
| 112 | 1.00 | 112 | 9336 | Changelog.md section #4 |
| 110 | 1.00 | 110 | 3632 | json config jsr.json |
| 109 | 1.00 | 109 | 3741 | export doc at src/error.ts:25 |
| 103 | 1.00 | 103 | 9224 | export doc at src/structs/utilities.ts:140 |
| 102 | 1.00 | 102 | 7876 | export at src/structs/utilities.ts:21 |
| 97 | 1.00 | 97 | 4897 | export doc at src/struct.ts:221 |
| 96 | 1.00 | 96 | 9432 | Changelog.md section #5 |
| 91 | 1.00 | 91 | 7474 | export doc at src/structs/refinements.ts:146 |
| 81 | 1.00 | 81 | 8820 | export doc at src/structs/utilities.ts:106 |
| 77 | 1.00 | 77 | 6648 | export doc at src/structs/coercions.ts:79 |
| 73 | 0.75 | 98 | 243 | Readme.md section #3 |
| 73 | 0.32 | 231 | 4195 | export names surface in src/struct.ts |
| 69 | 1.00 | 69 | 7175 | export doc at src/struct.ts:10 |
| 68 | 1.00 | 68 | 8739 | export doc at src/structs/utilities.ts:17 |
| 63 | 1.00 | 63 | 8671 | export doc at src/structs/utilities.ts:197 |
| 57 | 1.00 | 57 | 8504 | export doc at src/structs/utilities.ts:171 |
| 57 | 1.00 | 57 | 8561 | export doc at src/structs/utilities.ts:221 |
| 55 | 1.00 | 55 | 8447 | export doc at src/structs/utilities.ts:80 |
