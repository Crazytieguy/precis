scores: Score(3000)=0.407 ns_rows≤3K=20/41 (reached=4 partial=2 missing=14)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 88 | 0.657 | 0.467 | 0.818 | 0.554 | 994 |
| 1442 | 142 | 0.602 | 0.289 | 0.818 | 0.417 | 1434 |
| 2080 | 208 | 0.581 | 0.198 | 0.689 | 0.339 | 2072 |
| 3000 | 307 | 0.582 | 0.284 | 0.852 | 0.407 | 2845 |
| 4327 | 423 | 0.583 | 0.401 | 0.816 | 0.484 | 4323 |
| 6240 | 560 | 0.598 | 0.358 | 0.693 | 0.463 | 6093 |
| 9000 | 820 | 0.583 | 0.275 | 0.648 | 0.400 | 8991 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 24 | 2.80 | 2.71 | 2.30 | nearby candidates have low exact atom overlap | 1.1, 3.1, 1.5, 2.2, 2.3, ... |
| add walker candidates for no-discovered rows | 2 | 0.36 | 0.36 | 0.36 | NS rows have no discovered line candidate | 2.5, 2.6 |
| tune ranking for high-overlap unscheduled candidates | 5 | 0.24 | 0.24 | 0.24 | high-overlap candidates not in the schedule by T_max, exact total=124/136 | 5.3b, 4.1, 5.4b, 7.6, 7.4 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| package scripts in package.json | 1 | 0 | 259 | 259 | off_3k=259 | package scripts in package.json |
| export at typings/index.d.ts:<n> | 3 | 0 | 239 | 878 | off_3k=239 | export at typings/index.d.ts:354, export at typings/index.d.ts:40, export at typings/index.d.ts:16 |
| headings outline in docs/deprecated.md | 1 | 0 | 190 | 190 | off_3k=197 | headings outline in docs/deprecated.md |
| package identity in package.json | 1 | 162 | 162 | 162 | off_3k=162 | package identity in package.json |
| export names surface in typings/index.d.ts | 1 | 152 | 152 | 152 | off_3k=152 | export names surface in typings/index.d.ts |

Top missed paths (NS rows ≤ 3K): lib/command.js (4 rows, 63 atoms), lib/error.js (1 row, 39 atoms), examples/string-util.js (2 rows, 38 atoms), lib/option.js (1 row, 24 atoms), index.js (2 rows, 19 atoms), docs/terminology.md (2 rows, 18 atoms), esm.mjs (1 row, 16 atoms), lib/argument.js (1 row, 12 atoms), +2 more

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.1 | 3497 | 0.00 | missing | Argument constructor body | [scheduled bbox exact=2/28] export at lib/argument.js:3 (t=7138, 2 atoms); better unscheduled exact=24/28: export body at lib/argument.js:3 body 14 (24 atoms, too expensive at final margin) |
| 5.3b | 6799 | 0.00 | missing | parseOptions body — flag dispatch branches | [unscheduled bbox exact=56/60] export body at lib/command.js:13 body 1772 (56 atoms, too expensive at final margin) |
| 5.4b | 7597 | 0.00 | missing | addOption body — handleOptionValue closure | [unscheduled bbox exact=25/29] export body at lib/command.js:13 body 692 (25 atoms, too expensive at final margin) |
| 7.4 | 9679 | 0.00 | missing | Readme Options sub-headings | [unscheduled bbox exact=8/8] Readme.md section #4 (186 atoms, too expensive at final margin) |
| 7.6 | 9921 | 0.00 | missing | Readme Bits-and-pieces sub-headings | [unscheduled bbox exact=11/11] Readme.md section #8 (130 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.1 | 51 | 0.75 | partial | Package identity (name, version, description) | [scheduled bbox exact=3/4] package identity in package.json (t=548, 3 atoms) |
| 1.5 | 300 | 0.11 | missing | Readme major H2 sections | [scheduled bbox exact=1/9] Readme.md section #3 (t=9917, 23 atoms) |
| 2.2 | 542 | 0.00 | missing | index.js — imports, program, factories | [scheduled bbox exact=5/11] imports in index.js (t=9327, 5 atoms) |
| 2.3 | 710 | 0.00 | missing | Terminology doc — definitions | [unscheduled bbox exact=8/11] docs/terminology.md section #0 (8 atoms, too expensive at final margin) |
| 2.4 | 791 | 0.00 | missing | Terminology doc — example | [unscheduled bbox exact=5/7] docs/terminology.md section #0 (5 atoms, too expensive at final margin) |
| 3.1 | 2067 | 0.00 | missing | CommanderError + InvalidArgumentError class shapes | [scheduled bbox exact=7/39] export body at lib/error.js:4 body 12 (t=7860, 7 atoms) |
| 3.2 | 2170 | 0.00 | missing | Argument class — method names | [scheduled bbox exact=9/12] export at lib/argument.js:3 (t=7138, 17 atoms) |
| 3.3 | 2360 | 0.92 | partial | Option class — method names | [scheduled bbox exact=4/24] export names surface in lib/option.js (t=1402, 6 atoms) |
| 3.4 | 2477 | 0.00 | missing | Command class — registration & options method names | [scheduled bbox exact=0/15] export member at lib/command.js:13 member 741 (t=4950, 2 atoms) |
| 3.5 | 2641 | 0.00 | missing | Command class — config & option-value method names | [scheduled bbox exact=0/17] export member at lib/command.js:13 member 1183 (t=5635, 2 atoms) |
| 3.6 | 2751 | 0.00 | missing | Command class — parsing & action method names | [scheduled bbox exact=2/11] export names surface in lib/command.js (t=3155, 4 atoms) |
| 3.7 | 2925 | 0.00 | missing | Command class — help/usage/info method names | [scheduled bbox exact=0/20] export member at lib/command.js:13 member 1183 (t=5635, 2 atoms) |
| 4.2 | 3559 | 0.00 | missing | humanReadableArgName | [scheduled bbox exact=2/5] export body at lib/argument.js:143 body 144 (t=7186, 2 atoms) |
| 4.3 | 3964 | 0.04 | missing | Option constructor body | [scheduled bbox exact=2/27] export member at lib/option.js:3 member 11 (t=1540, 2 atoms); better unscheduled exact=3/27: export body at lib/option.js:3 body 24 (3 atoms, discovered unscheduled) |
| 4.4 | 4284 | 0.00 | missing | splitOptionFlags — flag-parser core | [scheduled same-file] export names surface in lib/option.js (t=1402, 6 atoms) |
| 5.1 | 4569 | 0.00 | missing | parsing-and-hooks doc | [unscheduled bbox exact=18/23] docs/parsing-and-hooks.md section #0 (18 atoms, too expensive at final margin) |
| 5.2 | 5020 | 0.00 | missing | _parseCommand body — option pass + subcommand routing | [scheduled bbox exact=2/34] export member at lib/command.js:13 member 1550 (t=4395, 2 atoms); better unscheduled exact=9/34: export body at lib/command.js:13 body 1575 (9 atoms, too expensive at final margin) |
| 5.2b | 5411 | 0.00 | missing | _parseCommand body — action handler + hooks | [unscheduled bbox exact=16/30] export body at lib/command.js:13 body 1597 (16 atoms, too expensive at final margin) |
| 5.3 | 6016 | 0.00 | missing | parseOptions body — entry + main loop frame | [scheduled bbox exact=2/50] export member at lib/command.js:13 member 1748 (t=3433, 2 atoms); better unscheduled exact=20/50: export body at lib/command.js:13 body 1772 (111 atoms, too expensive at final margin) |
| 5.4 | 7221 | 0.00 | missing | addOption body — defaults + listener registration | [scheduled bbox exact=2/35] export member at lib/command.js:13 member 670 (t=3282, 2 atoms); better unscheduled exact=13/35: export body at lib/command.js:13 body 677 (13 atoms, too expensive at final margin) |
| 6.1 | 7964 | 0.03 | missing | Help formatHelp — usage + description + arguments | [scheduled bbox exact=2/36] export member at lib/help.js:12 member 443 (t=2152, 2 atoms); better unscheduled exact=9/36: export body at lib/help.js:12 body 459 (9 atoms, discovered unscheduled) |
| 6.1b | 8479 | 0.00 | missing | Help formatHelp — options, global options, commands | [unscheduled bbox exact=13/48] export body at lib/help.js:12 body 496 (13 atoms, too expensive at final margin) |
| 7.3 | 9598 | 0.75 | partial | docs/ — H1/H2 headings of in-depth guides | [scheduled bbox exact=1/4] docs/options-in-depth.md section #0 (t=9659, 1 atoms) |
| 7.5 | 9802 | 0.00 | missing | Readme Commands + Automated help sub-headings | [unscheduled bbox exact=9/13] Readme.md section #6 (126 atoms, too expensive at final margin) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.5 | 1040 | 0.00 | missing | Quick Start — split subcommand | no discovered line candidate |
| 2.6 | 1240 | 0.00 | missing | Quick Start — join subcommand and parse | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 7.1 | 8752 | 0.13 | missing | examples/ + docs/ listings | fs-only |
| 7.2 | 9565 | 0.00 | missing | tests/ listing | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.1 | 383 | 0.00 | missing | index.js — class exports | [scheduled bbox exact=8/8] export names surface in index.js (t=4865, 8 atoms) |
| 2.7 | 1376 | 0.00 | missing | esm.mjs wrapper | [scheduled bbox exact=13/16] export at esm.mjs:4 (t=9444, 13 atoms) |
| 7.7 | 9995 | 0.00 | missing | options-in-depth — table of contents | [scheduled bbox exact=3/3] docs/options-in-depth.md section #0 (t=9659, 3 atoms) |

Top wasted paths (off-NS at 3K): package.json (421t, 2 batches), typings/index.d.ts (391t, 4 batches), docs/deprecated.md (197t, 1 batch), CHANGELOG.md (106t, 1 batch), SECURITY.md (73t, 1 batch), docs/options-in-depth.md (67t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 3 | 239 | export at typings/index.d.ts:<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 259 | 1.00 | 259 | 259 | 2845 | package scripts in package.json |
| 197 | 1.00 | 190 | 197 | 2366 | headings outline in docs/deprecated.md |
| 162 | 0.74 | 162 | 220 | 328 | package identity in package.json |
| 152 | 1.00 | 152 | 152 | 994 | export names surface in typings/index.d.ts |
| 121 | 1.00 | 121 | 121 | 1610 | export at typings/index.d.ts:354 |
| 106 | 1.00 | 106 | 106 | 725 | CHANGELOG.md section #0 |
| 73 | 1.00 | 73 | 73 | 866 | SECURITY.md section #0 |
| 67 | 1.00 | 58 | 67 | 651 | headings outline in docs/options-in-depth.md |
| 63 | 1.00 | 63 | 63 | 1299 | export at typings/index.d.ts:40 |
| 55 | 1.00 | 55 | 55 | 1244 | export at typings/index.d.ts:16 |
