scores: Sim=0.411 Reached=11/41 Early=2 Late=4 Partial=6 Missing=24 Used=9824/10000

## Verdict

Verdict: budget-pressure bound
Likely primary lever: free final budget / demote late low-value spend
Evidence: 17 ranking-recoverable (w×gap=1.78), 9 wrong-slice/granularity (w×gap=1.46), 2 no-discovered (w×gap=1.13)
Secondary intervention: promote predecessors for 8 gated candidates
Loss reasons: 8 predecessor-gated, 9 too-expensive, 0 discovered-unscheduled
Top rows: 3.4, 3.5, 3.7, 3.6, 3.8, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| free final budget / demote late waste | 9 | 1.47 | 4/7/9 | high-overlap candidates exceed final remaining budget, exact total=158/170 | 3.4, 3.5, 3.7, 3.6, 3.8, ... |
| split wrong-slice walker batches | 9 | 1.46 | 5/8/9 | nearby candidates have low exact atom overlap | 1.5, 1.1, 2.4, 2.3, 3.1, ... |
| add walker candidates for no-discovered rows | 2 | 1.13 | 2/2/2 | NS rows have no discovered line candidate | 2.5, 2.6 |
| promote export batches | 8 | 0.31 | 0/2/8 | 2 files, exact total=284/322 | 5.2, 5.2b, 5.3, 5.3b, 5.4, ... |

Tiers: 1=3/5 reached, 1 partial, 1 missing, avg=0.84; 2=4/8 reached, 2 partial, 2 missing, avg=0.65; 3=2/8 reached, 1 partial, 5 missing, avg=0.36; 4=0/4 reached, 1 partial, 3 missing, avg=0.17; 5=0/7 reached, 1 partial, 6 missing, avg=0.11; 6=0/2 reached, 0 partial, 2 missing, avg=0.00; 7=2/7 reached, 0 partial, 5 missing, avg=0.30

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 17 | 17 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 9 | 3 | 6 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 7 | 0 | 0 | 7 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 8 | 0.31 | promote predecessor |
| too expensive at final margin | 9 | 1.47 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=18, unscheduled bbox=13, scheduled same-file=1, fs-only=3, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 5 |
| scheduled bbox | partial | low | 6 |
| unscheduled bbox | missing | low | 1 |
| unscheduled bbox | missing | high | 8 |
| unscheduled bbox | missing | full | 4 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| group | 5020 | — | — | 0.00 | predecessor-gated | 6 children of `export at lib/command.js:13` | exact total=209/238; rows: 5.2, 5.2b, 5.3, 5.3b, 5.4, 5.4b |
| group | 7964 | — | — | 0.00 | predecessor-gated | 2 children of `export at lib/help.js:12` | exact total=75/84; rows: 6.1, 6.1b |
| 3.4 | 2477 | — | — | 0.07 | missing | Command class — registration & options method names | [scheduled bbox exact=1/15] export names surface in lib/command.js (t=3393, 2 atoms); better unscheduled exact=15/15: export at lib/command.js:13 (60 atoms, too expensive at final margin) |
| 3.5 | 2641 | — | — | 0.00 | missing | Command class — config & option-value method names | [unscheduled bbox exact=17/17] export at lib/command.js:13 (119 atoms, too expensive at final margin) |
| 3.6 | 2751 | — | — | 0.18 | missing | Command class — parsing & action method names | [scheduled bbox exact=0/11] export doc at lib/command.js:2752 (t=7983, 4 atoms); better unscheduled exact=9/11: export at lib/command.js:13 (162 atoms, too expensive at final margin) |
| 3.7 | 2925 | — | — | 0.00 | missing | Command class — help/usage/info method names | [unscheduled bbox exact=20/20] export at lib/command.js:13 (165 atoms, too expensive at final margin) |
| 3.8 | 3205 | — | — | 0.12 | missing | Help class — public method names | [scheduled bbox exact=5/33] export names surface in lib/help.js (t=2488, 6 atoms); better unscheduled exact=30/33: export at lib/help.js:12 (83 atoms, too expensive at final margin) |
| 4.1 | 3497 | — | — | 0.04 | missing | Argument constructor body | [scheduled bbox exact=2/28] export at lib/argument.js:3 (t=4916, 2 atoms); better unscheduled exact=24/28: export body at lib/argument.js:3 body 14 (24 atoms, too expensive at final margin) |
| 4.3 | 3964 | — | — | 0.04 | missing | Option constructor body | [scheduled bbox exact=2/27] export at lib/option.js:3 (t=7252, 2 atoms); better unscheduled exact=24/27: export body at lib/option.js:3 body 12 (24 atoms, too expensive at final margin) |
| 7.4 | 9679 | — | — | 0.00 | missing | Readme Options sub-headings | [unscheduled bbox exact=8/8] Readme.md section #4 (186 atoms, too expensive at final margin) |
| 7.6 | 9921 | — | — | 0.00 | missing | Readme Bits-and-pieces sub-headings | [unscheduled bbox exact=11/11] Readme.md section #8 (130 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 51 | — | — | 0.75 | partial | Package identity (name, version, description) | [scheduled bbox exact=3/4] package identity in package.json (t=557, 3 atoms) |
| 1.5 | 300 | — | — | 0.44 | missing | Readme major H2 sections | [scheduled bbox exact=1/9] Readme.md section #3 (t=7725, 23 atoms) |
| 2.3 | 710 | — | — | 0.73 | partial | Terminology doc — definitions | [scheduled bbox exact=8/11] docs/terminology.md section #0 (t=7949, 8 atoms) |
| 2.4 | 791 | — | — | 0.71 | partial | Terminology doc — example | [scheduled bbox exact=5/7] docs/terminology.md section #0 (t=7949, 5 atoms) |
| 3.1 | 2067 | — | — | 0.60 | partial | CommanderError + InvalidArgumentError class shapes | [scheduled bbox exact=7/39] export body at lib/error.js:4 body 12 (t=5688, 7 atoms) |
| 4.2 | 3559 | — | — | 0.60 | partial | humanReadableArgName | [scheduled bbox exact=2/5] export body at lib/argument.js:143 body 144 (t=4964, 2 atoms) |
| 4.4 | 4284 | — | — | 0.00 | missing | splitOptionFlags — flag-parser core | [scheduled same-file] export at lib/option.js:3 (t=7252, 33 atoms) |
| 5.1 | 4569 | — | — | 0.78 | partial | parsing-and-hooks doc | [scheduled bbox exact=18/23] docs/parsing-and-hooks.md section #0 (t=8243, 18 atoms) |
| 7.5 | 9802 | — | — | 0.00 | missing | Readme Commands + Automated help sub-headings | [unscheduled bbox exact=9/13] Readme.md section #6 (126 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.5 | 1040 | — | — | 0.00 | missing | Quick Start — split subcommand | no discovered line candidate |
| 2.6 | 1240 | — | — | 0.00 | missing | Quick Start — join subcommand and parse | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 7.1 | 8752 | — | — | 0.13 | missing | examples/ + docs/ listings | fs-only |
| 7.2 | 9565 | — | — | 0.00 | missing | tests/ listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 153 | 102 | -51 | 1.00 | early | Top-level fs | fs-only |
| 1.4 | 225 | 132 | -93 | 1.00 | early | Readme title + tagline | [scheduled bbox exact=2/2] README headline in Readme.md (t=132, 2 atoms) |
| 2.1 | 383 | 2765 | +2382 | 1.00 | late | index.js — class exports | [scheduled bbox exact=8/8] export names surface in index.js (t=2765, 8 atoms) |
| 2.2 | 542 | 7053 | +6511 | 0.91 | late | index.js — imports, program, factories | [scheduled bbox exact=5/11] imports in index.js (t=7053, 5 atoms) |
| 2.7 | 1376 | 1643 | +267 | 0.88 | aligned | esm.mjs wrapper | [scheduled bbox exact=13/16] export at esm.mjs:4 (t=1643, 13 atoms) |
| 3.2 | 2170 | 4916 | +2746 | 1.00 | late | Argument class — method names | [scheduled bbox exact=9/12] export at lib/argument.js:3 (t=4916, 17 atoms) |
| 3.3 | 2360 | 7252 | +4892 | 0.92 | late | Option class — method names | [scheduled bbox exact=17/24] export at lib/option.js:3 (t=7252, 33 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 7 | 1637 | export at typings/index.d.ts:<n> |
| 4 | 1363 | Readme.md section #<n> |
| 4 | 338 | CHANGELOG.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 825 | 1.00 | 825 | 4754 | Readme.md section #0 |
| 759 | 1.00 | 759 | 9246 | export at typings/index.d.ts:210 |
| 410 | 1.00 | 410 | 6401 | export at typings/index.d.ts:95 |
| 303 | 0.97 | 311 | 9788 | docs/release-policy.md section #0 |
| 259 | 1.00 | 259 | 2381 | package scripts in package.json |
| 256 | 1.00 | 256 | 3040 | package dependencies in package.json |
| 246 | 0.95 | 258 | 7725 | Readme.md section #3 |
| 239 | 1.00 | 239 | 5974 | json config tsconfig.ts.json |
| 223 | 0.97 | 231 | 9477 | Readme.md section #9 |
| 200 | 1.00 | 200 | 5164 | export names surface #1 in typings/index.d.ts |
| 2879 | — | — | — | +26 more rows |
