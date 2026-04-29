scores: Score(3000)=0.398 ns_rows≤3K=20/41 (reached=5 partial=1 missing=14)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 88 | 0.657 | 0.467 | 0.554 | 994 |
| 1442 | 142 | 0.600 | 0.289 | 0.417 | 1362 |
| 2080 | 208 | 0.591 | 0.327 | 0.440 | 2005 |
| 3000 | 307 | 0.587 | 0.270 | 0.398 | 2971 |
| 4327 | 423 | 0.563 | 0.211 | 0.345 | 3812 |
| 6240 | 560 | 0.557 | 0.201 | 0.335 | 5874 |
| 9000 | 820 | 0.597 | 0.222 | 0.364 | 8487 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 17 ranking-recoverable (gap@3k=0.23), 13 wrong-slice/granularity (gap@3k=1.91), 2 no-discovered (gap@3k=0.36)
Secondary intervention: free T_max budget for 9 too-expensive candidates
Top rows: 1.1, 3.1, 1.5, 2.3, 2.2, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms with rank ≤ |A_B|: (1 − damped_credit(a)) / rank(a)`. `gap@3k` is the primary sort key — direct proxy for `Score(3000)` headroom. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector. Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 13 | 1.60 | 1.91 | 1.47 | nearby candidates have low exact atom overlap | 1.1, 3.1, 1.5, 2.3, 2.2, ... |
| add walker candidates for no-discovered rows | 2 | 0.00 | 0.36 | 0.36 | NS rows have no discovered line candidate | 2.5, 2.6 |
| finish partially-delivered NS batches | 5 | 0.17 | 0.27 | 0.04 | avg batch completion=0.47 | 2.2, 3.3, 3.2, 4.2, 7.3 |
| free T_max budget / demote late waste | 9 | 0.00 | 0.23 | 0.47 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=158/170 | 3.7, 3.5, 3.4, 3.6, 3.8, ... |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 17 | 17 | 0 | value/ranking |
| wrong-slice / granularity | 13 | 11 | 2 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | filesystem/listing value |
| mixed/unknown | 2 | 2 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 8 | 0.00 | promote predecessor |
| too expensive at final margin | 9 | 0.23 | free T_max budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=18, unscheduled bbox=13, scheduled same-file=1, fs-only=2, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 13 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | missing | full | 1 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | low | 1 |
| unscheduled bbox | missing | high | 8 |
| unscheduled bbox | missing | full | 4 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| group | 5020 | 0.00 | 0.00 | predecessor-gated | 6 children of `export at lib/command.js:13` | exact total=209/238; rows: 5.2, 5.2b, 5.3, 5.3b, 5.4, 5.4b |
| group | 7964 | 0.00 | 0.00 | predecessor-gated | 2 children of `export at lib/help.js:12` | exact total=75/84; rows: 6.1, 6.1b |
| 3.4 | 2477 | 0.00 | 0.00 | missing | Command class — registration & options method names | [scheduled bbox exact=1/15] export names surface in lib/command.js (t=3276, 2 atoms); better unscheduled exact=15/15: export at lib/command.js:13 (60 atoms, too expensive at final margin) |
| 3.5 | 2641 | 0.00 | 0.00 | missing | Command class — config & option-value method names | [unscheduled bbox exact=17/17] export at lib/command.js:13 (119 atoms, too expensive at final margin) |
| 3.6 | 2751 | 0.00 | 0.00 | missing | Command class — parsing & action method names | [scheduled bbox exact=0/11] export doc at lib/command.js:2752 (t=7983, 4 atoms); better unscheduled exact=9/11: export at lib/command.js:13 (162 atoms, too expensive at final margin) |
| 3.7 | 2925 | 0.00 | 0.00 | missing | Command class — help/usage/info method names | [unscheduled bbox exact=20/20] export at lib/command.js:13 (165 atoms, too expensive at final margin) |
| 3.8 | 3205 | 0.12 | 0.15 | missing | Help class — public method names | [scheduled bbox exact=5/33] export names surface in lib/help.js (t=2371, 6 atoms); better unscheduled exact=30/33: export at lib/help.js:12 (83 atoms, too expensive at final margin) |
| 4.1 | 3497 | 0.00 | 0.00 | missing | Argument constructor body | [scheduled bbox exact=2/28] export at lib/argument.js:3 (t=4799, 2 atoms); better unscheduled exact=24/28: export body at lib/argument.js:3 body 14 (24 atoms, too expensive at final margin) |
| 4.3 | 3964 | 0.00 | 0.00 | missing | Option constructor body | [scheduled bbox exact=2/27] export at lib/option.js:3 (t=7252, 2 atoms); better unscheduled exact=24/27: export body at lib/option.js:3 body 12 (24 atoms, too expensive at final margin) |
| 7.4 | 9679 | 0.00 | 0.00 | missing | Readme Options sub-headings | [unscheduled bbox exact=8/8] Readme.md section #4 (186 atoms, too expensive at final margin) |
| 7.6 | 9921 | 0.00 | 0.00 | missing | Readme Bits-and-pieces sub-headings | [unscheduled bbox exact=11/11] Readme.md section #8 (130 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 51 | 0.75 | 0.99 | partial | Package identity (name, version, description) | [scheduled bbox exact=3/4] package identity in package.json (t=548, 3 atoms) |
| 1.5 | 300 | 0.11 | 0.10 | missing | Readme major H2 sections | [scheduled bbox exact=1/9] Readme.md section #3 (t=7725, 23 atoms) |
| 2.2 | 542 | 0.45 | 0.48 | missing | index.js — imports, program, factories | [scheduled bbox exact=5/11] imports in index.js (t=6936, 5 atoms) |
| 2.3 | 710 | 0.00 | 0.00 | missing | Terminology doc — definitions | [scheduled bbox exact=8/11] docs/terminology.md section #0 (t=7949, 8 atoms) |
| 2.4 | 791 | 0.00 | 0.00 | missing | Terminology doc — example | [scheduled bbox exact=5/7] docs/terminology.md section #0 (t=7949, 5 atoms) |
| 3.1 | 2067 | 0.00 | 0.00 | missing | CommanderError + InvalidArgumentError class shapes | [scheduled bbox exact=7/39] export body at lib/error.js:4 body 12 (t=5571, 7 atoms) |
| 3.2 | 2170 | 0.33 | 0.58 | missing | Argument class — method names | [scheduled bbox exact=9/12] export at lib/argument.js:3 (t=4799, 17 atoms) |
| 3.3 | 2360 | 0.25 | 0.36 | missing | Option class — method names | [scheduled bbox exact=17/24] export at lib/option.js:3 (t=7252, 33 atoms) |
| 4.2 | 3559 | 0.20 | 0.20 | missing | humanReadableArgName | [scheduled bbox exact=2/5] export body at lib/argument.js:143 body 144 (t=4847, 2 atoms) |
| 4.4 | 4284 | 0.00 | 0.00 | missing | splitOptionFlags — flag-parser core | [scheduled same-file] export at lib/option.js:3 (t=7252, 33 atoms) |
| 5.1 | 4569 | 0.00 | 0.00 | missing | parsing-and-hooks doc | [scheduled bbox exact=18/23] docs/parsing-and-hooks.md section #0 (t=8243, 18 atoms) |
| 7.3 | 9598 | 0.75 | 0.74 | partial | docs/ — H1/H2 headings of in-depth guides | [scheduled bbox exact=1/4] docs/release-policy.md section #0 (t=9788, 1 atoms) |
| 7.5 | 9802 | 0.00 | 0.00 | missing | Readme Commands + Automated help sub-headings | [unscheduled bbox exact=9/13] Readme.md section #6 (126 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.5 | 1040 | 0.00 | 0.00 | missing | Quick Start — split subcommand | no discovered line candidate |
| 2.6 | 1240 | 0.00 | 0.00 | missing | Quick Start — join subcommand and parse | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 7.1 | 8752 | 0.13 | 0.13 | missing | examples/ + docs/ listings | fs-only |
| 7.2 | 9565 | 0.00 | 0.00 | missing | tests/ listing | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.7 | 1376 | 0.00 | 0.00 | missing | esm.mjs wrapper | [scheduled bbox exact=13/16] export at esm.mjs:4 (t=7053, 13 atoms) |
| 7.7 | 9995 | 0.00 | 0.00 | missing | options-in-depth — table of contents | [scheduled bbox exact=3/3] docs/options-in-depth.md section #0 (t=7467, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 7 | 1637 | export at typings/index.d.ts:<n> |
| 4 | 1363 | Readme.md section #<n> |
| 4 | 338 | CHANGELOG.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 825 | 1.00 | 825 | 4637 | Readme.md section #0 |
| 759 | 1.00 | 759 | 9246 | export at typings/index.d.ts:210 |
| 410 | 1.00 | 410 | 6284 | export at typings/index.d.ts:95 |
| 303 | 0.97 | 311 | 9788 | docs/release-policy.md section #0 |
| 259 | 1.00 | 259 | 2264 | package scripts in package.json |
| 256 | 1.00 | 256 | 2923 | package dependencies in package.json |
| 246 | 0.95 | 258 | 7725 | Readme.md section #3 |
| 239 | 1.00 | 239 | 5857 | json config tsconfig.ts.json |
| 223 | 0.97 | 231 | 9477 | Readme.md section #9 |
| 200 | 1.00 | 200 | 5047 | export names surface #1 in typings/index.d.ts |
| 2879 | — | — | — | +26 more rows |
