scores: Sim=0.332 Reached=9/45 Early=5 Late=2 Partial=8 Missing=28 Used=9931/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=2.19), 31 wrong-slice/granularity (w×gap=6.14), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 0 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 1.1, 2.3, 2.5, 2.7, 2.8, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 31 | 6.14 | 11/23/31 | nearby candidates have low exact atom overlap | 1.1, 2.3, 2.5, 2.7, 2.8, ... |
| free final budget / demote late waste | 4 | 2.19 | 3/3/4 | high-overlap candidates exceed final remaining budget, exact total=73/79 | 1.6, 1.7, 2.4, 4.7 |

Tiers: 1=3/7 reached, 1 partial, 3 missing, avg=0.52; 2=3/13 reached, 1 partial, 9 missing, avg=0.28; 3=1/17 reached, 5 partial, 11 missing, avg=0.34; 4=2/8 reached, 1 partial, 5 missing, avg=0.39

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 31 | 23 | 8 | 0 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 7 | 0 | 0 | 7 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 4 | 2.19 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=23, unscheduled bbox=7, scheduled same-file=10, fs-only=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | none | 1 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 9 |
| scheduled bbox | partial | none | 3 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.6 | 434 | — | — | 0.00 | missing | README features — data structures + typesafety | [unscheduled bbox exact=3/3] README.md section #1 (3 atoms, too expensive at final margin) |
| 1.7 | 546 | — | — | 0.00 | missing | README features — patterns, wildcards, predicates, bundle | [unscheduled bbox exact=5/5] README.md section #1 (5 atoms, too expensive at final margin) |
| 2.4 | 981 | — | — | 0.00 | missing | match() JSDoc | [unscheduled bbox exact=16/16] export doc at src/match.ts:32 (16 atoms, too expensive at final margin) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch | [unscheduled bbox exact=49/55] export body at src/internals/helpers.ts:32 (49 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline | [scheduled same-file] README.md section #3 (t=7411, 15 atoms) |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | [scheduled bbox exact=3/6] imports in src/index.ts (t=862, 3 atoms) |
| 2.3 | 772 | — | — | 0.00 | missing | MatchExpression — method-name catalog | [scheduled same-file] imports in src/match.ts (t=7016, 5 atoms) |
| 2.5 | 1115 | — | — | 0.00 | missing | MatchExpression class doc + constructor | [scheduled same-file] imports in src/match.ts (t=7016, 5 atoms) |
| 2.6 | 1278 | — | — | 0.36 | missing | match.ts imports + MatchState/unmatched | [scheduled bbox exact=5/14] imports in src/match.ts (t=7016, 5 atoms) |
| 2.7 | 1489 | — | — | 0.00 | missing | MatchExpression.with — argument parsing half | [scheduled same-file] imports in src/match.ts (t=7016, 5 atoms) |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half | [scheduled same-file] imports in src/match.ts (t=7016, 5 atoms) |
| 2.9 | 2017 | — | — | 0.00 | missing | MatchExpression.when / otherwise / exhaustive bodies | [scheduled same-file] imports in src/match.ts (t=7016, 5 atoms) |
| 2.10 | 2115 | — | — | 0.00 | missing | MatchExpression.run / returnType / narrow + defaultCatcher | [scheduled same-file] imports in src/match.ts (t=7016, 5 atoms) |
| 2.11 | 2477 | — | — | 0.13 | missing | isMatching — JSDoc + PatternConstraint helper | [scheduled bbox exact=4/31] imports in src/is-matching.ts (t=4608, 4 atoms); better unscheduled exact=15/31: export doc at src/is-matching.ts:32 (15 atoms, too expensive at final margin) |
| 2.12 | 2851 | — | — | 0.50 | partial | isMatching — JSDoc for two-arg + runtime body | [scheduled bbox exact=12/30] export body at src/is-matching.ts:53 (t=2829, 12 atoms); better unscheduled exact=13/30: export doc at src/is-matching.ts:48 (13 atoms, too expensive at final margin) |
| 3.1 | 3161 | — | — | 0.50 | partial | P module-doc + combinator-function name catalog | [scheduled bbox exact=0/18] export body at src/patterns.ts:362 (t=9230, 33 atoms); better unscheduled exact=2/18: export names surface #3 in src/patterns.ts (9 atoms, too expensive at final margin) |
| 3.2 | 3350 | — | — | 0.00 | missing | P wildcards — every const definition | [unscheduled bbox exact=7/10] export names surface #2 in src/patterns.ts (14 atoms, too expensive at final margin) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog | [scheduled same-file] export body at src/patterns.ts:362 (t=9230, 33 atoms) |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog | [scheduled same-file] export body at src/patterns.ts:362 (t=9230, 33 atoms) |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | [scheduled bbox exact=4/17] export at src/patterns.ts:572 (t=5091, 4 atoms); better unscheduled exact=13/17: export doc at src/patterns.ts:572 (13 atoms, too expensive at final margin) |
| 3.7 | 3971 | — | — | 0.25 | missing | P.not — JSDoc + signature | [scheduled bbox exact=4/16] export at src/patterns.ts:611 (t=4956, 4 atoms); better unscheduled exact=11/16: export doc at src/patterns.ts:611 (11 atoms, too expensive at final margin) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | [scheduled bbox exact=6/20] export at src/patterns.ts:637 (t=5140, 6 atoms); better unscheduled exact=11/20: export doc at src/patterns.ts:637 (11 atoms, too expensive at final margin) |
| 3.9 | 4659 | — | — | 0.00 | missing | P.select — JSDoc + all three overload signatures | [unscheduled bbox exact=13/35] export at src/patterns.ts:673 (13 atoms, predecessor not scheduled: export names surface #2 in src/patterns.ts) |
| 3.10 | 4865 | — | — | 0.33 | missing | P.array — JSDoc + overload signatures | [scheduled bbox exact=4/15] export at src/patterns.ts:242 (t=2420, 4 atoms); better unscheduled exact=10/15: export doc at src/patterns.ts:241 (10 atoms, too expensive at final margin) |
| 3.11 | 5071 | — | — | 0.38 | missing | P.optional — JSDoc + signature | [scheduled bbox exact=6/16] export at src/patterns.ts:187 (t=2523, 6 atoms); better unscheduled exact=10/16: export doc at src/patterns.ts:187 (10 atoms, too expensive at final margin) |
| 3.12 | 5524 | — | — | 0.11 | missing | P.intersection / P.instanceOf — JSDoc + signatures | [scheduled bbox exact=0/36] export body at src/patterns.ts:572 (t=5992, 21 atoms); better unscheduled exact=19/36: export doc at src/patterns.ts:536 (19 atoms, too expensive at final margin) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | [scheduled bbox exact=8/24] export at src/patterns.ts:437 (t=5362, 8 atoms); better unscheduled exact=12/24: export doc at src/patterns.ts:433 (12 atoms, too expensive at final margin) |
| 3.14 | 6315 | — | — | 0.64 | partial | P.map / P.set — JSDoc + signatures | [scheduled bbox exact=0/33] export body at src/patterns.ts:299 (t=6475, 24 atoms); better unscheduled exact=12/33: export doc at src/patterns.ts:356 (12 atoms, too expensive at final margin) |
| 3.15 | 6498 | — | — | 0.67 | partial | P.shape + matcher protocol re-exports | [scheduled bbox exact=0/15] export body at src/patterns.ts:362 (t=9230, 33 atoms); better unscheduled exact=4/15: export names surface #3 in src/patterns.ts (12 atoms, too expensive at final margin) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable | [scheduled same-file] export body at src/patterns.ts:362 (t=9230, 33 atoms) |
| 3.17 | 7489 | — | — | 0.65 | partial | P.union / P.not / P.when — full bodies | [scheduled bbox exact=21/46] export body at src/patterns.ts:572 (t=5992, 21 atoms) |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | [scheduled bbox exact=2/10] export names surface in src/types/Match.ts (t=1429, 2 atoms); better unscheduled exact=6/10: export at src/types/Match.ts:22 (6 atoms, too expensive at final margin) |
| 4.3 | 8414 | — | — | 0.00 | missing | Pattern<T> — public pattern type alias + typed wildcards | [unscheduled bbox exact=11/34] export doc at src/types/Pattern.ts:157 (11 atoms, predecessor not scheduled: export at src/types/Pattern.ts:157) |
| 4.6 | 9151 | — | — | 0.16 | missing | matchPattern() — Matcher / object / primitive branches | [scheduled bbox exact=6/32] export at src/internals/helpers.ts:32 (t=1124, 6 atoms); better unscheduled exact=20/32: export body at src/internals/helpers.ts:32 (69 atoms, too expensive at final margin) |
| 4.8 | 9976 | — | — | 0.76 | partial | getSelectionKeys() + flatMap helpers | [scheduled bbox exact=8/17] export body at src/internals/helpers.ts:120 (t=3551, 8 atoms) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 7838 | — | — | 0.05 | missing | tests/ + docs/ + examples/ listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 151 | 3911 | +3760 | 1.00 | late | package.json identity fields | [scheduled bbox exact=4/6] package identity in package.json (t=566, 4 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing | fs-only |
| 1.5 | 364 | 1305 | +941 | 1.00 | late | src/ + src/types/ + src/internals/ listings | fs-only |
| 2.1 | 603 | 249 | -354 | 0.80 | early | match() exported signature | [scheduled bbox exact=4/5] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.13 | 3005 | 1249 | -1756 | 0.87 | early | NonExhaustiveError class | [scheduled bbox exact=7/15] export body at src/errors.ts:5 (t=935, 7 atoms) |
| 3.3 | 3422 | 2261 | -1161 | 1.00 | early | P type-level re-exports + matcher symbol | [scheduled bbox exact=0/6] export doc at src/patterns.ts:100 (t=8626, 13 atoms) |
| 4.5 | 8816 | 3430 | -5386 | 1.00 | early | internals/symbols.ts — core matcher / unset / isVariadic brands | [scheduled bbox exact=8/8] export names surface in src/internals/symbols.ts (t=3430, 8 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 350 | export at src/types/DistributeUnions.ts:<n> |
| 3 | 349 | README.md section #<n> |
| 4 | 342 | export at src/types/FindSelected.ts:<n> |
| 3 | 290 | export at src/patterns.ts:<n> |
| 2 | 257 | export doc at src/patterns.ts:<n> |
| 3 | 226 | export at src/types/helpers.ts:<n> |
| 2 | 225 | export at src/types/Pattern.ts:<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 418 | 1.00 | 418 | 9230 | export body at src/patterns.ts:362 |
| 336 | 0.93 | 360 | 3911 | package entrypoints in package.json |
| 312 | 1.00 | 312 | 6941 | export body at src/patterns.ts:246 |
| 285 | 1.00 | 285 | 6475 | export body at src/patterns.ts:299 |
| 271 | 1.00 | 271 | 4182 | package scripts in package.json |
| 259 | 1.00 | 259 | 9489 | export names surface in src/types/helpers.ts |
| 235 | 1.00 | 235 | 4546 | export body at src/patterns.ts:187 |
| 217 | 0.79 | 273 | 566 | package identity in package.json |
| 212 | 0.91 | 233 | 7946 | export names surface in src/types/Pattern.ts |
| 193 | 1.00 | 193 | 5746 | export body at src/patterns.ts:536 |
| 3297 | — | — | — | +32 more rows |
