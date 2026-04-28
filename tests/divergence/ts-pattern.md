scores: Sim=0.317 Reached=8/45 Early=4 Late=3 Partial=8 Missing=29 Used=9720/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=2.19), 32 wrong-slice/granularity (w×gap=6.34), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 3 too-expensive candidates
Loss reasons: 0 predecessor-gated, 3 too-expensive, 1 discovered-unscheduled
Top rows: 1.1, 2.3, 2.5, 2.6, 2.7, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 32 | 6.34 | 11/23/32 | nearby candidates have low exact atom overlap | 1.1, 2.3, 2.5, 2.6, 2.7, ... |
| free final budget / demote late waste | 3 | 1.57 | 2/2/3 | high-overlap candidates exceed final remaining budget, exact total=57/63 | 1.6, 1.7, 4.7 |
| tune ranking for discovered unscheduled candidates | 1 | 0.61 | 1/1/1 | high-overlap candidates fit but did not win, exact total=16/16 | 2.4 |

Tiers: 1=3/7 reached, 1 partial, 3 missing, avg=0.52; 2=3/13 reached, 1 partial, 9 missing, avg=0.25; 3=1/17 reached, 4 partial, 12 missing, avg=0.32; 4=1/8 reached, 2 partial, 5 missing, avg=0.39

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 32 | 24 | 8 | 0 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 7 | 0 | 0 | 7 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 3 | 1.57 | free final budget |
| discovered unscheduled | 1 | 0.61 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=24, unscheduled bbox=7, scheduled same-file=10, fs-only=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 2 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 10 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 7 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.6 | 434 | — | — | 0.00 | missing | README features — data structures + typesafety | [unscheduled bbox exact=3/3] README.md section #1 (3 atoms, too expensive at final margin) |
| 1.7 | 546 | — | — | 0.00 | missing | README features — patterns, wildcards, predicates, bundle | [unscheduled bbox exact=5/5] README.md section #1 (5 atoms, too expensive at final margin) |
| 2.4 | 981 | — | — | 0.00 | missing | match() JSDoc | [unscheduled bbox exact=16/16] export doc at src/match.ts:32 (16 atoms, discovered unscheduled) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch | [unscheduled bbox exact=49/55] export body at src/internals/helpers.ts:32 (49 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline | [scheduled same-file] README.md section #0 (t=823, 2 atoms) |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | [scheduled bbox exact=3/6] imports in src/index.ts (t=862, 3 atoms) |
| 2.3 | 772 | — | — | 0.00 | missing | MatchExpression — method-name catalog | [scheduled same-file] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.5 | 1115 | — | — | 0.00 | missing | MatchExpression class doc + constructor | [scheduled same-file] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.6 | 1278 | — | — | 0.00 | missing | match.ts imports + MatchState/unmatched | [unscheduled bbox exact=5/14] imports in src/match.ts (5 atoms, discovered unscheduled) |
| 2.7 | 1489 | — | — | 0.00 | missing | MatchExpression.with — argument parsing half | [scheduled same-file] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half | [scheduled same-file] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.9 | 2017 | — | — | 0.00 | missing | MatchExpression.when / otherwise / exhaustive bodies | [scheduled same-file] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.10 | 2115 | — | — | 0.00 | missing | MatchExpression.run / returnType / narrow + defaultCatcher | [scheduled same-file] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.11 | 2477 | — | — | 0.13 | missing | isMatching — JSDoc + PatternConstraint helper | [scheduled bbox exact=4/31] imports in src/is-matching.ts (t=7847, 4 atoms); better unscheduled exact=15/31: export doc at src/is-matching.ts:32 (15 atoms, discovered unscheduled) |
| 2.12 | 2851 | — | — | 0.50 | partial | isMatching — JSDoc for two-arg + runtime body | [scheduled bbox exact=12/30] export body at src/is-matching.ts:53 (t=5299, 12 atoms); better unscheduled exact=13/30: export doc at src/is-matching.ts:48 (13 atoms, discovered unscheduled) |
| 3.1 | 3161 | — | — | 0.50 | partial | P module-doc + combinator-function name catalog | [scheduled bbox exact=6/18] export names surface #1 in src/patterns.ts (t=8088, 23 atoms) |
| 3.2 | 3350 | — | — | 0.00 | missing | P wildcards — every const definition | [unscheduled bbox exact=7/10] export names surface #2 in src/patterns.ts (14 atoms, discovered unscheduled) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog | [scheduled same-file] export names surface #1 in src/patterns.ts (t=8088, 23 atoms) |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog | [scheduled same-file] export names surface #1 in src/patterns.ts (t=8088, 23 atoms) |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | [scheduled bbox exact=4/17] export at src/patterns.ts:572 (t=8330, 4 atoms); better unscheduled exact=13/17: export doc at src/patterns.ts:572 (13 atoms, discovered unscheduled) |
| 3.7 | 3971 | — | — | 0.25 | missing | P.not — JSDoc + signature | [scheduled bbox exact=4/16] export at src/patterns.ts:611 (t=8195, 4 atoms); better unscheduled exact=11/16: export doc at src/patterns.ts:611 (11 atoms, discovered unscheduled) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | [scheduled bbox exact=6/20] export at src/patterns.ts:637 (t=8379, 6 atoms); better unscheduled exact=11/20: export doc at src/patterns.ts:637 (11 atoms, discovered unscheduled) |
| 3.9 | 4659 | — | — | 0.00 | missing | P.select — JSDoc + all three overload signatures | [unscheduled bbox exact=13/35] export at src/patterns.ts:673 (13 atoms, predecessor not scheduled: export names surface #2 in src/patterns.ts) |
| 3.10 | 4865 | — | — | 0.33 | missing | P.array — JSDoc + overload signatures | [scheduled bbox exact=4/15] export at src/patterns.ts:242 (t=4890, 4 atoms); better unscheduled exact=10/15: export doc at src/patterns.ts:241 (10 atoms, discovered unscheduled) |
| 3.11 | 5071 | — | — | 0.38 | missing | P.optional — JSDoc + signature | [scheduled bbox exact=6/16] export at src/patterns.ts:187 (t=4993, 6 atoms); better unscheduled exact=10/16: export doc at src/patterns.ts:187 (10 atoms, discovered unscheduled) |
| 3.12 | 5524 | — | — | 0.11 | missing | P.intersection / P.instanceOf — JSDoc + signatures | [scheduled bbox exact=0/36] export body at src/patterns.ts:572 (t=9231, 21 atoms); better unscheduled exact=19/36: export doc at src/patterns.ts:536 (19 atoms, discovered unscheduled) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | [scheduled bbox exact=8/24] export at src/patterns.ts:437 (t=8601, 8 atoms); better unscheduled exact=12/24: export doc at src/patterns.ts:433 (12 atoms, discovered unscheduled) |
| 3.14 | 6315 | — | — | 0.33 | missing | P.map / P.set — JSDoc + signatures | [scheduled bbox exact=5/33] export at src/patterns.ts:357 (t=8507, 5 atoms); better unscheduled exact=12/33: export doc at src/patterns.ts:356 (12 atoms, discovered unscheduled) |
| 3.15 | 6498 | — | — | 0.67 | partial | P.shape + matcher protocol re-exports | [scheduled bbox exact=0/15] export names surface #1 in src/patterns.ts (t=8088, 23 atoms); better unscheduled exact=4/15: export names surface #3 in src/patterns.ts (12 atoms, discovered unscheduled) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable | [scheduled same-file] export names surface #1 in src/patterns.ts (t=8088, 23 atoms) |
| 3.17 | 7489 | — | — | 0.65 | partial | P.union / P.not / P.when — full bodies | [scheduled bbox exact=21/46] export body at src/patterns.ts:572 (t=9231, 21 atoms) |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | [scheduled bbox exact=2/10] export names surface in src/types/Match.ts (t=2381, 2 atoms); better unscheduled exact=6/10: export at src/types/Match.ts:22 (6 atoms, too expensive at final margin) |
| 4.3 | 8414 | — | — | 0.62 | partial | Pattern<T> — public pattern type alias + typed wildcards | [scheduled bbox exact=8/34] export names surface #2 in src/types/Pattern.ts (t=2780, 8 atoms); better unscheduled exact=11/34: export doc at src/types/Pattern.ts:157 (11 atoms, discovered unscheduled) |
| 4.4 | 8718 | — | — | 0.41 | missing | MatcherType union + Matcher interface | [scheduled bbox exact=12/32] export at src/types/Pattern.ts:7 (t=9422, 12 atoms); better unscheduled exact=20/32: export at src/types/Pattern.ts:50 (20 atoms, discovered unscheduled) |
| 4.6 | 9151 | — | — | 0.16 | missing | matchPattern() — Matcher / object / primitive branches | [scheduled bbox exact=6/32] export at src/internals/helpers.ts:32 (t=1124, 6 atoms); better unscheduled exact=20/32: export body at src/internals/helpers.ts:32 (69 atoms, too expensive at final margin) |
| 4.8 | 9976 | — | — | 0.76 | partial | getSelectionKeys() + flatMap helpers | [scheduled bbox exact=8/17] export body at src/internals/helpers.ts:120 (t=6440, 8 atoms) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 7838 | — | — | 0.05 | missing | tests/ + docs/ + examples/ listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 151 | 6800 | +6649 | 1.00 | late | package.json identity fields | [scheduled bbox exact=4/6] package identity in package.json (t=566, 4 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing | fs-only |
| 1.5 | 364 | 1305 | +941 | 1.00 | late | src/ + src/types/ + src/internals/ listings | fs-only |
| 2.1 | 603 | 249 | -354 | 0.80 | early | match() exported signature | [scheduled bbox exact=4/5] export at src/match.ts:32 (t=235, 4 atoms) |
| 2.13 | 3005 | 1249 | -1756 | 0.87 | early | NonExhaustiveError class | [scheduled bbox exact=7/15] export body at src/errors.ts:5 (t=935, 7 atoms) |
| 3.3 | 3422 | 4731 | +1309 | 1.00 | late | P type-level re-exports + matcher symbol | [scheduled bbox exact=4/6] export names surface in src/patterns.ts (t=4708, 10 atoms) |
| 4.5 | 8816 | 6024 | -2792 | 1.00 | early | internals/symbols.ts — core matcher / unset / isVariadic brands | [scheduled bbox exact=8/8] export names surface in src/internals/symbols.ts (t=6024, 8 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 10 | 752 | export at src/types/helpers.ts:<n> |
| 3 | 290 | export at src/patterns.ts:<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |
| 3 | 196 | export at src/types/FindSelected.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 336 | 0.93 | 360 | 6800 | package entrypoints in package.json |
| 271 | 1.00 | 271 | 7071 | package scripts in package.json |
| 270 | 1.00 | 270 | 3074 | export names surface #2 in src/types/helpers.ts |
| 267 | 1.00 | 267 | 3799 | export names surface #3 in src/types/helpers.ts |
| 259 | 1.00 | 259 | 1807 | export names surface in src/types/helpers.ts |
| 235 | 1.00 | 235 | 7518 | export body at src/patterns.ts:187 |
| 217 | 0.79 | 273 | 566 | package identity in package.json |
| 212 | 0.91 | 233 | 1548 | export names surface in src/types/Pattern.ts |
| 205 | 1.00 | 205 | 2090 | export names surface #1 in src/types/helpers.ts |
| 193 | 1.00 | 193 | 8985 | export body at src/patterns.ts:536 |
| 2926 | — | — | — | +32 more rows |
