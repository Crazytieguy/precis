scores: Sim=0.369 Reached=6/41 Early=3 Late=0 Partial=4 Missing=31 Used=9988/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 2 ranking-recoverable (w×gap=0.01), 6 wrong-slice/granularity (w×gap=1.14), 25 no-discovered (w×gap=6.38)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 0 discovered-unscheduled
Top rows: 2.1, 2.2, 2.5, 2.6, 2.7, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 25 | 6.38 | 12/19/25 | NS rows have no discovered line candidate | 2.1, 2.2, 2.5, 2.6, 2.7, ... |
| split wrong-slice walker batches | 6 | 1.14 | 4/5/6 | nearby candidates have low exact atom overlap | 1.5, 1.1, 2.4, 2.3, 5.1, ... |
| free final budget / demote late waste | 2 | 0.01 | 0/0/2 | high-overlap candidates exceed final remaining budget, exact total=19/19 | 7.4, 7.6 |

Tiers: 1=3/5 reached, 1 partial, 1 missing, avg=0.84; 2=1/8 reached, 2 partial, 5 missing, avg=0.31; 3=0/8 reached, 0 partial, 8 missing, avg=0.00; 4=0/4 reached, 0 partial, 4 missing, avg=0.00; 5=0/7 reached, 1 partial, 6 missing, avg=0.11; 6=0/2 reached, 0 partial, 2 missing, avg=0.00; 7=2/7 reached, 0 partial, 5 missing, avg=0.30

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 6 | 2 | 4 | 0 | walker granularity / wrong slice |
| no discovered candidate | 25 | 25 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 3 | 0 | 0 | 3 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.01 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=7, unscheduled bbox=3, fs-only=3, no discovered candidate=25

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | full | 2 |
| scheduled bbox | missing | low | 1 |
| scheduled bbox | partial | low | 4 |
| unscheduled bbox | missing | low | 1 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 7.4 | 9679 | — | — | 0.00 | missing | Readme Options sub-headings | [unscheduled bbox exact=8/8] Readme.md section #4 (186 atoms, too expensive at final margin) |
| 7.6 | 9921 | — | — | 0.00 | missing | Readme Bits-and-pieces sub-headings | [unscheduled bbox exact=11/11] Readme.md section #8 (130 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 51 | — | — | 0.75 | partial | Package identity (name, version, description) | [scheduled bbox exact=3/4] package identity in package.json (t=548, 3 atoms) |
| 1.5 | 300 | — | — | 0.44 | missing | Readme major H2 sections | [scheduled bbox exact=1/9] Readme.md section #3 (t=6373, 23 atoms) |
| 2.3 | 710 | — | — | 0.73 | partial | Terminology doc — definitions | [scheduled bbox exact=8/11] docs/terminology.md section #0 (t=6597, 8 atoms) |
| 2.4 | 791 | — | — | 0.71 | partial | Terminology doc — example | [scheduled bbox exact=5/7] docs/terminology.md section #0 (t=6597, 5 atoms) |
| 5.1 | 4569 | — | — | 0.78 | partial | parsing-and-hooks doc | [scheduled bbox exact=18/23] docs/parsing-and-hooks.md section #0 (t=6857, 18 atoms) |
| 7.5 | 9802 | — | — | 0.00 | missing | Readme Commands + Automated help sub-headings | [unscheduled bbox exact=4/13] Readme.md section #5 (140 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.1 | 383 | — | — | 0.00 | missing | index.js — class exports | no discovered line candidate |
| 2.2 | 542 | — | — | 0.00 | missing | index.js — imports, program, factories | no discovered line candidate |
| 2.5 | 1040 | — | — | 0.00 | missing | Quick Start — split subcommand | no discovered line candidate |
| 2.6 | 1240 | — | — | 0.00 | missing | Quick Start — join subcommand and parse | no discovered line candidate |
| 2.7 | 1376 | — | — | 0.00 | missing | esm.mjs wrapper | no discovered line candidate |
| 3.1 | 2067 | — | — | 0.00 | missing | CommanderError + InvalidArgumentError class shapes | no discovered line candidate |
| 3.2 | 2170 | — | — | 0.00 | missing | Argument class — method names | no discovered line candidate |
| 3.3 | 2360 | — | — | 0.00 | missing | Option class — method names | no discovered line candidate |
| 3.4 | 2477 | — | — | 0.00 | missing | Command class — registration & options method names | no discovered line candidate |
| 3.5 | 2641 | — | — | 0.00 | missing | Command class — config & option-value method names | no discovered line candidate |
| 3.6 | 2751 | — | — | 0.00 | missing | Command class — parsing & action method names | no discovered line candidate |
| 3.7 | 2925 | — | — | 0.00 | missing | Command class — help/usage/info method names | no discovered line candidate |
| 3.8 | 3205 | — | — | 0.00 | missing | Help class — public method names | no discovered line candidate |
| 4.1 | 3497 | — | — | 0.00 | missing | Argument constructor body | no discovered line candidate |
| 4.2 | 3559 | — | — | 0.00 | missing | humanReadableArgName | no discovered line candidate |
| 4.3 | 3964 | — | — | 0.00 | missing | Option constructor body | no discovered line candidate |
| 4.4 | 4284 | — | — | 0.00 | missing | splitOptionFlags — flag-parser core | no discovered line candidate |
| 5.2 | 5020 | — | — | 0.00 | missing | _parseCommand body — option pass + subcommand routing | no discovered line candidate |
| 5.2b | 5411 | — | — | 0.00 | missing | _parseCommand body — action handler + hooks | no discovered line candidate |
| 5.3 | 6016 | — | — | 0.00 | missing | parseOptions body — entry + main loop frame | no discovered line candidate |
| 5.3b | 6799 | — | — | 0.00 | missing | parseOptions body — flag dispatch branches | no discovered line candidate |
| 5.4 | 7221 | — | — | 0.00 | missing | addOption body — defaults + listener registration | no discovered line candidate |
| 5.4b | 7597 | — | — | 0.00 | missing | addOption body — handleOptionValue closure | no discovered line candidate |
| 6.1 | 7964 | — | — | 0.00 | missing | Help formatHelp — usage + description + arguments | no discovered line candidate |
| 6.1b | 8479 | — | — | 0.00 | missing | Help formatHelp — options, global options, commands | no discovered line candidate |

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
| 7.7 | 9995 | 6115 | -3880 | 1.00 | early | options-in-depth — table of contents | [scheduled bbox exact=3/3] docs/options-in-depth.md section #0 (t=6115, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 7 | 1637 | export at typings/index.d.ts:<n> |
| 4 | 1363 | Readme.md section #<n> |
| 5 | 728 | docs/deprecated.md section #<n> |
| 4 | 338 | CHANGELOG.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 825 | 1.00 | 825 | 4103 | Readme.md section #0 |
| 759 | 1.00 | 759 | 7860 | export at typings/index.d.ts:210 |
| 527 | 1.00 | 527 | 9706 | json config tsconfig.json |
| 410 | 1.00 | 410 | 5349 | export at typings/index.d.ts:95 |
| 340 | 1.00 | 340 | 8778 | docs/deprecated.md section #0 |
| 303 | 0.97 | 311 | 8402 | docs/release-policy.md section #0 |
| 259 | 1.00 | 259 | 2238 | package scripts in package.json |
| 256 | 1.00 | 256 | 2621 | package dependencies in package.json |
| 246 | 0.95 | 258 | 6373 | Readme.md section #3 |
| 239 | 1.00 | 239 | 4939 | json config tsconfig.ts.json |
| 3624 | — | — | — | +31 more rows |
