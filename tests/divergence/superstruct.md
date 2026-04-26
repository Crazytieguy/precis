scores: Sim=0.466 Reached=19/39 Early=5 Late=11 Partial=1 Missing=19 Used=9965/10000

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
| 1.3 | 240 | 1614 | +1374 | 1.00 | late | Public entry — src/index.ts in full | imports in src/index.ts (t=1614, 6 atoms) |
| 1.4 | 306 | — | — | 0.00 | missing | Readme runtime-errors paragraph |  |
| 1.5 | 394 | 708 | +314 | 1.00 | late | package.json — name, version, license | package identity in package.json (t=708, 5 atoms) |
| 2.1 | 431 | 1479 | +1048 | 1.00 | late | src/ tree (flat layout) |  |
| 2.2 | 583 | 8662 | +8079 | 1.00 | late | package.json — module shape & engines | package entrypoints in package.json (t=784, 7 atoms) |
| 2.3 | 694 | 2016 | +1322 | 1.00 | late | docs/ subtree listings |  |
| 2.4 | 802 | — | — | 0.00 | missing | examples/ + test/ + test/api/ tree |  |
| 2.5 | 851 | 145 | -706 | 1.00 | early | Readme section headings (locations only) | Readme.md section #8 (t=3705, 12 atoms) |
| 2.6 | 1168 | — | — | 0.76 | partial | docs/summary.md (the GitBook TOC) | mdBook SUMMARY at docs/summary.md (t=1948, 19 atoms) |
| 3.1 | 1448 | — | — | 0.00 | missing | structs/types.ts — all 24 type-factory names |  |
| 3.2 | 1533 | 2727 | +1194 | 1.00 | late | structs/refinements.ts — all 7 refinement names | export names surface in src/structs/refinements.ts (t=2727, 13 atoms) |
| 3.3 | 1571 | 1535 | -36 | 1.00 | aligned+over | structs/coercions.ts — coerce / defaulted / trimmed | export body at src/structs/coercions.ts:16 (t=3788, 8 atoms) |
| 3.4 | 1682 | 7092 | +5410 | 1.00 | late | structs/utilities.ts — assign/define/deprecated/dynamic/lazy/omit/partial/pick/struct | export names surface in src/structs/utilities.ts (t=7092, 25 atoms) |
| 3.5 | 1789 | 9294 | +7505 | 1.00 | late | struct.ts — Struct class + assert/create/is/mask/validate signatures | export at src/struct.ts:10 (t=9294, 37 atoms) |
| 3.6 | 1985 | 4251 | +2266 | 1.00 | late | struct.ts — public types (Context, Infer, Describe, Result, Coercer, Validator, Refiner) | export names surface in src/struct.ts (t=4180, 13 atoms) |
| 3.7 | 2170 | 973 | -1197 | 0.95 | early | error.ts — Failure type + StructError class signature | export at src/error.ts:5 (t=327, 10 atoms) |
| 4.1 | 2380 | — | — | 0.00 | missing | docs/reference/types.md — H3 catalog (all 27 entries) |  |
| 4.2 | 2578 | 3949 | +1371 | 1.00 | late | docs/reference/{refinements,coercions,utilities,core,typescript}.md headings | headings outline in docs/reference/refinements.md (t=3870, 13 atoms) |
| 4.3 | 2854 | — | — | 0.00 | missing | Canonical usage — examples/basic-validation.js (full file) |  |
| 4.4 | 3305 | — | — | 0.15 | missing | errors.md — StructError property table | headings outline in docs/reference/errors.md (t=1350, 2 atoms) |
| 4.5 | 3875 | — | — | 0.18 | missing | core.md — `assert` / `create` / `validate` definitions | headings outline in docs/reference/core.md (t=2628, 10 atoms) |
| 4.6 | 4064 | — | — | 0.13 | missing | coercions.md — `defaulted` worked example | headings outline in docs/reference/coercions.md (t=1310, 2 atoms) |
| 5.1 | 4816 | — | — | 0.00 | missing | utils.ts run() — the central traversal generator |  |
| 5.2 | 5074 | 4804 | -270 | 0.89 | aligned | struct.ts validate() body | export body at src/struct.ts:185 (t=4804, 15 atoms) |
| 5.3 | 5610 | 9702 | +4092 | 0.90 | late | struct.ts Struct class constructor + 4 hook fields | export body at src/struct.ts:10 (t=9702, 28 atoms) |
| 5.4 | 6093 | — | — | 0.00 | missing | object() implementation |  |
| 5.5 | 6541 | — | — | 0.00 | missing | union() implementation |  |
| 5.6 | 6716 | 6054 | -662 | 0.89 | aligned | refine() implementation | export body at src/structs/refinements.ts:146 (t=6054, 11 atoms) |
| 5.7 | 7243 | — | — | 0.36 | missing | coerce() + defaulted() implementations | export body at src/structs/coercions.ts:16 (t=3788, 8 atoms) |
| 5.8 | 7434 | 3320 | -4114 | 0.93 | early | error.ts StructError constructor | export body at src/error.ts:25 (t=3320, 12 atoms) |
| 6.1 | 7559 | — | — | 0.00 | missing | test/validation/ tree — all 41 kind directories |  |
| 6.2 | 7742 | — | — | 0.00 | missing | test/typings/ tree — all type-level test files |  |
| 6.3 | 8099 | — | — | 0.00 | missing | Sample validation fixture — three exemplar modules |  |
| 6.4 | 8999 | — | — | 0.00 | missing | test/index.test.ts — the validation-fixture harness |  |
| 6.5 | 9499 | — | — | 0.00 | missing | types.ts — `object` and `type` reference bodies |  |
| 6.6 | 9863 | — | — | 0.00 | missing | examples/default-values.js — defaulted + create |  |
| 6.7 | 9978 | 2245 | -7733 | 1.00 | early | Guide H2 headings — all 14 across guides 02-06 | headings outline in docs/guides/02-validating-data.md (t=2245, 7 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 779 | Readme.md section #<n> |
| 3 | 434 | export at src/structs/utilities.ts:<n> |
| 3 | 344 | Changelog.md section #<n> |
| 2 | 166 | export doc at src/struct.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 434 | 1.00 | 434 | 5447 | headings outline in Changelog.md |
| 425 | 0.91 | 468 | 6579 | Readme.md section #1 |
| 324 | 1.00 | 324 | 2569 | package scripts in package.json |
| 295 | 0.78 | 381 | 708 | package identity in package.json |
| 255 | 0.85 | 301 | 8662 | package dependencies in package.json |
| 185 | 1.00 | 185 | 1185 | json config tsconfig.json |
| 182 | 1.00 | 182 | 8123 | export at src/structs/utilities.ts:44 |
| 176 | 0.92 | 193 | 3705 | Readme.md section #8 |
| 150 | 1.00 | 150 | 7715 | export at src/structs/utilities.ts:30 |
| 142 | 0.65 | 218 | 7092 | export names surface in src/structs/utilities.ts |
| 136 | 1.00 | 136 | 6874 | Changelog.md section #2 |
| 121 | 0.32 | 376 | 9294 | export at src/struct.ts:10 |
| 119 | 1.00 | 119 | 8361 | export body at src/structs/utilities.ts:171 |
| 119 | 1.00 | 119 | 8242 | export body at src/structs/utilities.ts:80 |
| 113 | 1.00 | 113 | 7828 | export body at src/structs/utilities.ts:197 |
| 113 | 1.00 | 113 | 7941 | export body at src/structs/utilities.ts:221 |
| 112 | 1.00 | 112 | 9869 | Changelog.md section #4 |
| 110 | 1.00 | 110 | 3430 | json config jsr.json |
| 109 | 1.00 | 109 | 8918 | export doc at src/error.ts:25 |
| 107 | 1.00 | 107 | 5636 | export body at src/structs/refinements.ts:33 |
| 107 | 1.00 | 107 | 5743 | export body at src/structs/refinements.ts:55 |
| 102 | 1.00 | 102 | 7342 | export at src/structs/utilities.ts:21 |
| 97 | 1.00 | 97 | 8759 | export doc at src/struct.ts:221 |
| 96 | 1.00 | 96 | 9965 | Changelog.md section #5 |
| 82 | 1.00 | 82 | 3512 | export body at src/structs/refinements.ts:8 |
| 73 | 0.75 | 98 | 882 | Readme.md section #7 |
| 73 | 0.32 | 231 | 4180 | export names surface in src/struct.ts |
| 72 | 1.00 | 72 | 3157 | export body at src/structs/refinements.ts:77 |
| 71 | 1.00 | 71 | 3085 | export body at src/structs/refinements.ts:93 |
| 69 | 1.00 | 69 | 4563 | export body at src/struct.ts:157 |
| 69 | 1.00 | 69 | 9363 | export doc at src/struct.ts:10 |
| 65 | 1.00 | 65 | 4494 | export body at src/struct.ts:139 |
| 65 | 1.00 | 65 | 7445 | export body at src/structs/utilities.ts:60 |
| 55 | 1.00 | 55 | 9757 | Readme.md section #2 |
| 51 | 0.15 | 339 | 9702 | export body at src/struct.ts:10 |
| 50 | 1.00 | 50 | 8809 | Readme.md section #3 |
