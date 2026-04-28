scores: Sim=0.348 Reached=9/45 Early=3 Late=4 Partial=10 Missing=26 Used=9936/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (w×gap=0.62), 33 wrong-slice/granularity (w×gap=5.94), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 0 discovered-unscheduled
Top rows: 1.1, 2.6, 2.5, 2.7, 2.8, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 33 | 5.94 | 12/24/33 | nearby candidates have low exact atom overlap | 1.1, 2.6, 2.5, 2.7, 2.8, ... |
| free final budget / demote late waste | 2 | 0.62 | 1/1/2 | high-overlap candidates exceed final remaining budget, exact total=65/71 | 2.4, 4.7 |

Tiers: 1=3/7 reached, 3 partial, 1 missing, avg=0.71; 2=4/13 reached, 1 partial, 8 missing, avg=0.38; 3=1/17 reached, 4 partial, 12 missing, avg=0.32; 4=1/8 reached, 2 partial, 5 missing, avg=0.39

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 33 | 23 | 10 | 0 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 7 | 0 | 0 | 7 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.62 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=30, unscheduled bbox=6, scheduled same-file=4, fs-only=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 2 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 14 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 9 |
| unscheduled bbox | missing | low | 4 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.4 | 981 | — | — | 0.00 | missing | match() JSDoc | [unscheduled bbox exact=16/16] export doc at src/match.ts:32 (16 atoms, too expensive at final margin) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch | [unscheduled bbox exact=49/55] export body at src/internals/helpers.ts:32 body 37 (49 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline | [scheduled same-file] README.md section #0 (t=823, 2 atoms) |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | [scheduled bbox exact=3/6] imports in src/index.ts (t=862, 3 atoms) |
| 1.6 | 434 | — | — | 0.67 | partial | README features — data structures + typesafety | [scheduled bbox exact=1/3] README.md section #3 (t=6581, 1 atoms) |
| 1.7 | 546 | — | — | 0.60 | partial | README features — patterns, wildcards, predicates, bundle | [scheduled bbox exact=1/5] README.md section #4 (t=5663, 1 atoms) |
| 2.5 | 1115 | — | — | 0.18 | missing | MatchExpression class doc + constructor | [scheduled bbox exact=2/11] module item at src/match.ts:47 (t=1448, 2 atoms) |
| 2.6 | 1278 | — | — | 0.00 | missing | match.ts imports + MatchState/unmatched | [unscheduled bbox exact=5/14] imports in src/match.ts (5 atoms, too expensive at final margin) |
| 2.7 | 1489 | — | — | 0.06 | missing | MatchExpression.with — argument parsing half | [scheduled bbox exact=2/16] module item at src/match.ts:47 (t=1448, 2 atoms); better unscheduled exact=7/16: module item body at src/match.ts:47 body 59 (7 atoms, too expensive at final margin) |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half | [unscheduled bbox exact=6/27] module item body at src/match.ts:47 body 84 (6 atoms, discovered unscheduled) |
| 2.9 | 2017 | — | — | 0.24 | missing | MatchExpression.when / otherwise / exhaustive bodies | [scheduled bbox exact=9/25] module item at src/match.ts:47 (t=1448, 9 atoms) |
| 2.10 | 2115 | — | — | 0.20 | missing | MatchExpression.run / returnType / narrow + defaultCatcher | [scheduled bbox exact=6/16] module item at src/match.ts:47 (t=1448, 6 atoms) |
| 2.11 | 2477 | — | — | 0.13 | missing | isMatching — JSDoc + PatternConstraint helper | [scheduled bbox exact=4/31] imports in src/is-matching.ts (t=8163, 4 atoms); better unscheduled exact=15/31: export doc at src/is-matching.ts:32 (15 atoms, too expensive at final margin) |
| 2.12 | 2851 | — | — | 0.50 | partial | isMatching — JSDoc for two-arg + runtime body | [scheduled bbox exact=12/30] export body at src/is-matching.ts:53 body 56 (t=5523, 12 atoms); better unscheduled exact=13/30: export doc at src/is-matching.ts:48 (13 atoms, too expensive at final margin) |
| 3.1 | 3161 | — | — | 0.50 | partial | P module-doc + combinator-function name catalog | [scheduled bbox exact=6/18] export names surface #1 in src/patterns.ts (t=8404, 23 atoms) |
| 3.2 | 3350 | — | — | 0.00 | missing | P wildcards — every const definition | [unscheduled bbox exact=7/10] export names surface #2 in src/patterns.ts (14 atoms, too expensive at final margin) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog | [scheduled same-file] export names surface #1 in src/patterns.ts (t=8404, 23 atoms) |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog | [scheduled same-file] export names surface #1 in src/patterns.ts (t=8404, 23 atoms) |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | [scheduled bbox exact=4/17] export at src/patterns.ts:572 (t=8646, 4 atoms); better unscheduled exact=13/17: export doc at src/patterns.ts:572 (13 atoms, too expensive at final margin) |
| 3.7 | 3971 | — | — | 0.25 | missing | P.not — JSDoc + signature | [scheduled bbox exact=4/16] export at src/patterns.ts:611 (t=8511, 4 atoms); better unscheduled exact=11/16: export doc at src/patterns.ts:611 (11 atoms, too expensive at final margin) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | [scheduled bbox exact=6/20] export at src/patterns.ts:637 (t=8695, 6 atoms); better unscheduled exact=11/20: export doc at src/patterns.ts:637 (11 atoms, too expensive at final margin) |
| 3.9 | 4659 | — | — | 0.00 | missing | P.select — JSDoc + all three overload signatures | [unscheduled bbox exact=13/35] export at src/patterns.ts:673 (13 atoms, predecessor not scheduled: export names surface #2 in src/patterns.ts) |
| 3.10 | 4865 | — | — | 0.33 | missing | P.array — JSDoc + overload signatures | [scheduled bbox exact=4/15] export at src/patterns.ts:242 (t=5114, 4 atoms); better unscheduled exact=10/15: export doc at src/patterns.ts:241 (10 atoms, too expensive at final margin) |
| 3.11 | 5071 | — | — | 0.38 | missing | P.optional — JSDoc + signature | [scheduled bbox exact=6/16] export at src/patterns.ts:187 (t=5217, 6 atoms); better unscheduled exact=10/16: export doc at src/patterns.ts:187 (10 atoms, too expensive at final margin) |
| 3.12 | 5524 | — | — | 0.11 | missing | P.intersection / P.instanceOf — JSDoc + signatures | [scheduled bbox exact=0/36] export body at src/patterns.ts:572 body 576 (t=9547, 21 atoms); better unscheduled exact=19/36: export doc at src/patterns.ts:536 (19 atoms, too expensive at final margin) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | [scheduled bbox exact=8/24] export at src/patterns.ts:437 (t=8917, 8 atoms); better unscheduled exact=12/24: export doc at src/patterns.ts:433 (12 atoms, too expensive at final margin) |
| 3.14 | 6315 | — | — | 0.33 | missing | P.map / P.set — JSDoc + signatures | [scheduled bbox exact=5/33] export at src/patterns.ts:357 (t=8823, 5 atoms); better unscheduled exact=12/33: export doc at src/patterns.ts:356 (12 atoms, too expensive at final margin) |
| 3.15 | 6498 | — | — | 0.67 | partial | P.shape + matcher protocol re-exports | [scheduled bbox exact=0/15] export names surface #1 in src/patterns.ts (t=8404, 23 atoms); better unscheduled exact=4/15: export names surface #3 in src/patterns.ts (12 atoms, too expensive at final margin) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable | [scheduled same-file] export names surface #1 in src/patterns.ts (t=8404, 23 atoms) |
| 3.17 | 7489 | — | — | 0.65 | partial | P.union / P.not / P.when — full bodies | [scheduled bbox exact=21/46] export body at src/patterns.ts:572 body 576 (t=9547, 21 atoms) |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | [scheduled bbox exact=2/10] export names surface in src/types/Match.ts (t=2580, 2 atoms); better unscheduled exact=6/10: export at src/types/Match.ts:22 (6 atoms, too expensive at final margin) |
| 4.3 | 8414 | — | — | 0.62 | partial | Pattern<T> — public pattern type alias + typed wildcards | [scheduled bbox exact=8/34] export names surface #2 in src/types/Pattern.ts (t=2979, 8 atoms); better unscheduled exact=11/34: export doc at src/types/Pattern.ts:157 (11 atoms, too expensive at final margin) |
| 4.4 | 8718 | — | — | 0.41 | missing | MatcherType union + Matcher interface | [scheduled bbox exact=12/32] export at src/types/Pattern.ts:7 (t=9738, 12 atoms); better unscheduled exact=20/32: export at src/types/Pattern.ts:50 (20 atoms, too expensive at final margin) |
| 4.6 | 9151 | — | — | 0.16 | missing | matchPattern() — Matcher / object / primitive branches | [scheduled bbox exact=6/32] export at src/internals/helpers.ts:32 (t=1124, 6 atoms); better unscheduled exact=20/32: export body at src/internals/helpers.ts:32 body 37 (69 atoms, too expensive at final margin) |
| 4.8 | 9976 | — | — | 0.76 | partial | getSelectionKeys() + flatMap helpers | [scheduled bbox exact=8/17] export body at src/internals/helpers.ts:120 body 121 (t=6756, 8 atoms) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 7838 | — | — | 0.05 | missing | tests/ + docs/ + examples/ listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 151 | 7116 | +6965 | 1.00 | late | package.json identity fields | [scheduled bbox exact=4/6] package identity in package.json (t=566, 4 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing | fs-only |
| 1.5 | 364 | 1504 | +1140 | 1.00 | late | src/ + src/types/ + src/internals/ listings | fs-only |
| 2.1 | 603 | 249 | -354 | 0.80 | early | match() exported signature | [scheduled bbox exact=4/5] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.3 | 772 | 1448 | +676 | 1.00 | late | MatchExpression — method-name catalog | [scheduled bbox exact=8/8] module item at src/match.ts:47 (t=1448, 18 atoms) |
| 2.13 | 3005 | 1269 | -1736 | 0.87 | early | NonExhaustiveError class | [scheduled bbox exact=7/15] export body at src/errors.ts:5 body 7 (t=935, 7 atoms) |
| 3.3 | 3422 | 4955 | +1533 | 1.00 | late | P type-level re-exports + matcher symbol | [scheduled bbox exact=4/6] export names surface in src/patterns.ts (t=4932, 10 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 9 | 652 | export at src/types/helpers.ts:<n> |
| 3 | 290 | export at src/patterns.ts:<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |
| 3 | 196 | export at src/types/FindSelected.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 336 | 0.93 | 360 | 7116 | package entrypoints in package.json |
| 271 | 1.00 | 271 | 7387 | package scripts in package.json |
| 270 | 1.00 | 270 | 3273 | export names surface #2 in src/types/helpers.ts |
| 267 | 1.00 | 267 | 3998 | export names surface #3 in src/types/helpers.ts |
| 259 | 1.00 | 259 | 2006 | export names surface in src/types/helpers.ts |
| 235 | 1.00 | 235 | 7834 | export body at src/patterns.ts:187 body 193 |
| 217 | 0.79 | 273 | 566 | package identity in package.json |
| 212 | 0.91 | 233 | 1747 | export names surface in src/types/Pattern.ts |
| 205 | 1.00 | 205 | 2289 | export names surface #1 in src/types/helpers.ts |
| 193 | 1.00 | 193 | 9301 | export body at src/patterns.ts:536 body 540 |
| 2826 | — | — | — | +31 more rows |
