scores: Sim=0.444 Reached=15/39 Early=4 Late=10 Partial=1 Missing=23 Used=9751/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 3 | 0 | 2 | 0.60 |
| 2 | 6 | 4 | 1 | 1 | 0.79 |
| 3 | 7 | 6 | 0 | 1 | 0.85 |
| 4 | 6 | 1 | 0 | 5 | 0.24 |
| 5 | 8 | 0 | 0 | 8 | 0.16 |
| 6 | 7 | 1 | 0 | 6 | 0.14 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 96 | — | — | 0.00 | missing | Readme one-line lede |  |
| 1.2 | 161 | 65 | -96 | 1.00 | early | Repo root listing |  |
| 1.3 | 240 | 1807 | +1567 | 1.00 | late | Public entry — src/index.ts in full | imports in src/index.ts (t=1807, 6 atoms) |
| 1.4 | 306 | — | — | 0.00 | missing | Readme runtime-errors paragraph |  |
| 1.5 | 394 | 822 | +428 | 1.00 | late | package.json — name, version, license | package identity in package.json (t=822, 5 atoms) |
| 2.1 | 431 | 1672 | +1241 | 1.00 | late | src/ tree (flat layout) |  |
| 2.2 | 583 | 7584 | +7001 | 1.00 | late | package.json — module shape & engines | package entrypoints in package.json (t=898, 7 atoms) |
| 2.3 | 694 | 2192 | +1498 | 1.00 | late | docs/ subtree listings |  |
| 2.4 | 802 | — | — | 0.00 | missing | examples/ + test/ + test/api/ tree |  |
| 2.5 | 851 | 145 | -706 | 1.00 | early | Readme section headings (locations only) | Readme.md section #0 (t=9327, 76 atoms) |
| 2.6 | 1168 | — | — | 0.76 | partial | docs/summary.md (the GitBook TOC) | mdBook SUMMARY at docs/summary.md (t=2141, 19 atoms) |
| 3.1 | 1448 | — | — | 0.00 | missing | structs/types.ts — all 24 type-factory names |  |
| 3.2 | 1533 | 3207 | +1674 | 1.00 | late | structs/refinements.ts — all 7 refinement names | export names surface in src/structs/refinements.ts (t=3207, 13 atoms) |
| 3.3 | 1571 | 1728 | +157 | 1.00 | aligned+over | structs/coercions.ts — coerce / defaulted / trimmed | export at src/structs/coercions.ts:38 (t=6183, 8 atoms) |
| 3.4 | 1682 | 6609 | +4927 | 1.00 | late | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | export names surface in src/structs/utilities.ts (t=6609, 25 atoms) |
| 3.5 | 1789 | 8166 | +6377 | 1.00 | late | struct.ts — Struct class + assert/create/is/mask/validate signatures | export at src/struct.ts:10 (t=8166, 37 atoms) |
| 3.6 | 1985 | 4067 | +2082 | 1.00 | late | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | export names surface in src/struct.ts (t=3996, 13 atoms) |
| 3.7 | 2170 | 989 | -1181 | 0.95 | early | error.ts — Failure type + StructError class signature | export at src/error.ts:5 (t=441, 10 atoms) |
| 4.1 | 2380 | — | — | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) |  |
| 4.2 | 2578 | 3765 | +1187 | 1.00 | late | docs/reference/{refinements,coercions,utilities,core,typescript}.md headings | headings outline in docs/reference/refinements.md (t=3686, 13 atoms) |
| 4.3 | 2854 | — | — | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) |  |
| 4.4 | 3305 | — | — | 0.15 | missing | errors.md — StructError property table | headings outline in docs/reference/errors.md (t=1543, 2 atoms) |
| 4.5 | 3875 | — | — | 0.18 | missing | core.md — `assert` / `create` / `validate` definitions | headings outline in docs/reference/core.md (t=3108, 10 atoms) |
| 4.6 | 4064 | — | — | 0.13 | missing | coercions.md — `defaulted` worked example | headings outline in docs/reference/coercions.md (t=1503, 2 atoms) |
| 5.1 | 4816 | — | — | 0.00 | missing | utils.ts run() — the central traversal generator |  |
| 5.2 | 5074 | — | — | 0.33 | missing | struct.ts validate() body | export at src/struct.ts:185 (t=4264, 10 atoms) |
| 5.3 | 5610 | — | — | 0.35 | missing | struct.ts Struct class constructor + 4 hook fields | export at src/struct.ts:10 (t=8166, 19 atoms) |
| 5.4 | 6093 | — | — | 0.00 | missing | object() implementation |  |
| 5.5 | 6541 | — | — | 0.00 | missing | union() implementation |  |
| 5.6 | 6716 | — | — | 0.28 | missing | refine() implementation | export at src/structs/refinements.ts:146 (t=3494, 6 atoms) |
| 5.7 | 7243 | — | — | 0.22 | missing | coerce() + defaulted() implementations | export at src/structs/coercions.ts:38 (t=6183, 8 atoms) |
| 5.8 | 7434 | — | — | 0.07 | missing | error.ts StructError constructor | export at src/error.ts:25 (t=989, 2 atoms) |
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
| 6 | 2272 | Readme.md section #<n> |
| 3 | 434 | export at src/structs/utilities.ts:<n> |
| 3 | 344 | Changelog.md section #<n> |
| 3 | 169 | export doc at src/structs/utilities.ts:<n> |
| 2 | 166 | export doc at src/struct.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 884 | 1.00 | 884 | 9327 | Readme.md section #0 |
| 454 | 0.95 | 477 | 5418 | Readme.md section #5 |
| 434 | 1.00 | 434 | 5852 | headings outline in Changelog.md |
| 425 | 0.91 | 468 | 4732 | Readme.md section #1 |
| 324 | 1.00 | 324 | 2745 | package scripts in package.json |
| 295 | 0.78 | 381 | 822 | package identity in package.json |
| 260 | 0.86 | 304 | 3049 | Readme.md section #2 |
| 255 | 0.85 | 301 | 7584 | package dependencies in package.json |
| 185 | 1.00 | 185 | 1394 | json config tsconfig.json |
| 182 | 1.00 | 182 | 7283 | export at src/structs/utilities.ts:44 |
| 176 | 0.92 | 193 | 1182 | Readme.md section #4 |
| 150 | 1.00 | 150 | 7101 | export at src/structs/utilities.ts:30 |
| 142 | 0.65 | 218 | 6609 | export names surface in src/structs/utilities.ts |
| 136 | 1.00 | 136 | 6391 | Changelog.md section #2 |
| 121 | 0.32 | 376 | 8166 | export at src/struct.ts:10 |
| 112 | 1.00 | 112 | 8347 | Changelog.md section #4 |
| 110 | 1.00 | 110 | 3604 | json config jsr.json |
| 109 | 1.00 | 109 | 7790 | export doc at src/error.ts:25 |
| 102 | 1.00 | 102 | 6793 | export at src/structs/utilities.ts:21 |
| 97 | 1.00 | 97 | 7681 | export doc at src/struct.ts:221 |
| 96 | 1.00 | 96 | 8443 | Changelog.md section #5 |
| 73 | 0.75 | 98 | 243 | Readme.md section #3 |
| 73 | 0.32 | 231 | 3996 | export names surface in src/struct.ts |
| 69 | 1.00 | 69 | 8235 | export doc at src/struct.ts:10 |
| 57 | 1.00 | 57 | 9647 | export doc at src/structs/utilities.ts:171 |
| 57 | 1.00 | 57 | 9704 | export doc at src/structs/utilities.ts:221 |
| 55 | 1.00 | 55 | 9590 | export doc at src/structs/utilities.ts:80 |
