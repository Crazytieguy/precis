scores: Score(3000)=0.288 ns_rows≤3K=19/45 (reached=4 partial=2 missing=13)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 97 | 0.303 | 0.384 | 0.341 | 986 |
| 1442 | 122 | 0.386 | 0.506 | 0.442 | 1299 |
| 2080 | 190 | 0.357 | 0.337 | 0.347 | 2074 |
| 3000 | 267 | 0.339 | 0.244 | 0.288 | 2937 |
| 4327 | 385 | 0.321 | 0.204 | 0.256 | 4267 |
| 6240 | 511 | 0.373 | 0.183 | 0.261 | 5994 |
| 9000 | 787 | 0.359 | 0.173 | 0.249 | 8795 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (gap@3k=0.24), 35 wrong-slice/granularity (gap@3k=4.92), 0 no-discovered (gap@3k=0.00)
Top rows: 1.1, 1.2, 1.3, 2.8, 2.11, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 35 | 4.95 | 4.92 | 4.42 | nearby candidates have low exact atom overlap | 1.1, 1.2, 1.3, 2.8, 2.11, ... |
| tune ranking for high-overlap unscheduled candidates | 2 | 0.24 | 0.24 | 0.24 | high-overlap candidates not in the schedule by T_max, exact total=65/71 | 2.4, 4.7 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | value/ranking |
| wrong-slice / granularity | 35 | 34 | 1 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 2 | 1 | 1 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 2 | 0.24 | tune ranking |

Candidate hint kinds: scheduled bbox=24, unscheduled bbox=11, scheduled same-file=4, fs-only=1 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 2 |
| scheduled bbox | missing | low | 19 |
| scheduled bbox | missing | full | 1 |
| scheduled bbox | partial | low | 1 |
| scheduled bbox | partial | high | 1 |
| unscheduled bbox | missing | low | 9 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.4 | 981 | 0.00 | 0.00 | missing | match() JSDoc | [unscheduled bbox exact=16/16] export doc at src/match.ts:32 (16 atoms, too expensive at final margin) |
| 4.7 | 9767 | 0.00 | 0.00 | missing | matchPattern() — array / tuple / variadic branch | [unscheduled bbox exact=49/55] export body at src/internals/helpers.ts:32 body 37 (49 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 71 | 0.00 | 0.00 | missing | README lede + tagline | [scheduled same-file] README.md section #10 (t=9577, 15 atoms) |
| 1.2 | 151 | 0.67 | 0.74 | missing | package.json identity fields | [scheduled bbox exact=4/6] package identity in package.json (t=566, 4 atoms) |
| 1.3 | 223 | 0.67 | 0.76 | partial | src/index.ts — full public re-export surface | [scheduled bbox exact=3/6] imports in src/index.ts (t=709, 3 atoms) |
| 1.6 | 434 | 0.33 | 0.26 | missing | README features — data structures + typesafety | [scheduled bbox exact=1/3] README.md section #3 (t=5459, 1 atoms) |
| 1.7 | 546 | 0.00 | 0.00 | missing | README features — patterns, wildcards, predicates, bundle | [scheduled bbox exact=1/5] README.md section #4 (t=4686, 1 atoms) |
| 2.5 | 1115 | 0.18 | 0.25 | missing | MatchExpression class doc + constructor | [scheduled bbox exact=2/11] module item at src/match.ts:47 (t=1207, 2 atoms) |
| 2.6 | 1278 | 0.00 | 0.00 | missing | match.ts imports + MatchState/unmatched | [scheduled bbox exact=5/14] imports in src/match.ts (t=9182, 5 atoms) |
| 2.7 | 1489 | 0.06 | 0.10 | missing | MatchExpression.with — argument parsing half | [scheduled bbox exact=2/16] module item at src/match.ts:47 (t=1207, 2 atoms); better unscheduled exact=7/16: module item body at src/match.ts:47 body 59 (7 atoms, discovered unscheduled) |
| 2.8 | 1764 | 0.00 | 0.00 | missing | MatchExpression.with — selection + dispatch half | [unscheduled bbox exact=6/27] module item body at src/match.ts:47 body 84 (6 atoms, discovered unscheduled) |
| 2.9 | 2017 | 0.24 | 0.37 | missing | MatchExpression.when / otherwise / exhaustive bodies | [scheduled bbox exact=9/25] module item at src/match.ts:47 (t=1207, 9 atoms) |
| 2.10 | 2115 | 0.20 | 0.23 | missing | MatchExpression.run / returnType / narrow + defaultCatcher | [scheduled bbox exact=6/16] module item at src/match.ts:47 (t=1207, 6 atoms) |
| 2.11 | 2477 | 0.00 | 0.00 | missing | isMatching — JSDoc + PatternConstraint helper | [scheduled bbox exact=4/31] imports in src/is-matching.ts (t=7481, 4 atoms); better unscheduled exact=15/31: export doc at src/is-matching.ts:32 (15 atoms, too expensive at final margin) |
| 2.12 | 2851 | 0.10 | 0.15 | missing | isMatching — JSDoc for two-arg + runtime body | [scheduled bbox exact=12/30] export body at src/is-matching.ts:53 body 56 (t=4428, 12 atoms); better unscheduled exact=13/30: export doc at src/is-matching.ts:48 (13 atoms, too expensive at final margin) |
| 3.1 | 3161 | 0.00 | 0.00 | missing | P module-doc + combinator-function name catalog | [scheduled bbox exact=0/18] export body at src/patterns.ts:246 body 249 (t=9107, 27 atoms); better unscheduled exact=6/18: export names surface #1 in src/patterns.ts (23 atoms, too expensive at final margin) |
| 3.2 | 3350 | 0.00 | 0.00 | missing | P wildcards — every const definition | [unscheduled bbox exact=7/10] export names surface #2 in src/patterns.ts (14 atoms, too expensive at final margin) |
| 3.3 | 3422 | 0.00 | 0.00 | missing | P type-level re-exports + matcher symbol | [scheduled bbox exact=4/6] export names surface in src/patterns.ts (t=6665, 10 atoms) |
| 3.4 | 3482 | 0.00 | 0.00 | missing | P.string predicates — name catalog | [scheduled same-file] export body at src/patterns.ts:246 body 249 (t=9107, 27 atoms) |
| 3.5 | 3560 | 0.00 | 0.00 | missing | P.number / P.bigint predicates — name catalog | [scheduled same-file] export body at src/patterns.ts:246 body 249 (t=9107, 27 atoms) |
| 3.6 | 3780 | 0.00 | 0.00 | missing | P.union — JSDoc + signature | [unscheduled bbox exact=13/17] export doc at src/patterns.ts:572 (13 atoms, predecessor not scheduled: export at src/patterns.ts:572) |
| 3.7 | 3971 | 0.00 | 0.00 | missing | P.not — JSDoc + signature | [unscheduled bbox exact=11/16] export doc at src/patterns.ts:611 (11 atoms, predecessor not scheduled: export at src/patterns.ts:611) |
| 3.8 | 4233 | 0.00 | 0.00 | missing | P.when — JSDoc + both overload signatures | [unscheduled bbox exact=11/20] export doc at src/patterns.ts:637 (11 atoms, predecessor not scheduled: export at src/patterns.ts:637) |
| 3.9 | 4659 | 0.00 | 0.00 | missing | P.select — JSDoc + all three overload signatures | [unscheduled bbox exact=13/35] export at src/patterns.ts:673 (13 atoms, predecessor not scheduled: export names surface #2 in src/patterns.ts) |
| 3.10 | 4865 | 0.00 | 0.00 | missing | P.array — JSDoc + overload signatures | [scheduled bbox exact=4/15] export at src/patterns.ts:242 (t=6847, 4 atoms); better unscheduled exact=10/15: export doc at src/patterns.ts:241 (10 atoms, too expensive at final margin) |
| 3.11 | 5071 | 0.00 | 0.00 | missing | P.optional — JSDoc + signature | [scheduled bbox exact=6/16] export at src/patterns.ts:187 (t=6950, 6 atoms); better unscheduled exact=10/16: export doc at src/patterns.ts:187 (10 atoms, too expensive at final margin) |
| 3.12 | 5524 | 0.00 | 0.00 | missing | P.intersection / P.instanceOf — JSDoc + signatures | [unscheduled bbox exact=19/36] export doc at src/patterns.ts:536 (19 atoms, predecessor not scheduled: export at src/patterns.ts:536) |
| 3.13 | 5846 | 0.00 | 0.00 | missing | P.record — JSDoc + signatures | [unscheduled bbox exact=12/24] export doc at src/patterns.ts:433 (12 atoms, predecessor not scheduled: export at src/patterns.ts:433) |
| 3.14 | 6315 | 0.00 | 0.00 | missing | P.map / P.set — JSDoc + signatures | [scheduled bbox exact=4/33] export at src/patterns.ts:295 (t=6890, 4 atoms); better unscheduled exact=12/33: export doc at src/patterns.ts:356 (12 atoms, predecessor not scheduled: export at src/patterns.ts:356) |
| 3.15 | 6498 | 0.00 | 0.00 | missing | P.shape + matcher protocol re-exports | [scheduled bbox exact=0/15] export body at src/patterns.ts:246 body 249 (t=9107, 27 atoms); better unscheduled exact=4/15: export names surface #3 in src/patterns.ts (12 atoms, too expensive at final margin) |
| 3.16 | 6988 | 0.00 | 0.00 | missing | chainable() factory + variadic / arrayChainable | [scheduled same-file] export body at src/patterns.ts:246 body 249 (t=9107, 27 atoms) |
| 3.17 | 7489 | 0.00 | 0.00 | missing | P.union / P.not / P.when — full bodies | [unscheduled bbox exact=21/46] export body at src/patterns.ts:572 body 576 (21 atoms, predecessor not scheduled: export at src/patterns.ts:572) |
| 4.2 | 7921 | 0.12 | 0.12 | missing | Match<i, o> — public builder type signature | [scheduled bbox exact=2/10] export names surface in src/types/Match.ts (t=2123, 2 atoms); better unscheduled exact=6/10: export at src/types/Match.ts:22 (6 atoms, too expensive at final margin) |
| 4.3 | 8414 | 0.44 | 0.52 | missing | Pattern<T> — public pattern type alias + typed wildcards | [scheduled bbox exact=8/34] export names surface #2 in src/types/Pattern.ts (t=2870, 8 atoms); better unscheduled exact=11/34: export doc at src/types/Pattern.ts:157 (11 atoms, too expensive at final margin) |
| 4.4 | 8718 | 0.07 | 0.08 | missing | MatcherType union + Matcher interface | [scheduled bbox exact=12/32] export at src/types/Pattern.ts:7 (t=7765, 12 atoms); better unscheduled exact=20/32: export at src/types/Pattern.ts:50 (20 atoms, too expensive at final margin) |
| 4.6 | 9151 | 0.00 | 0.00 | missing | matchPattern() — Matcher / object / primitive branches | [scheduled bbox exact=6/32] export at src/internals/helpers.ts:32 (t=4060, 6 atoms); better unscheduled exact=20/32: export body at src/internals/helpers.ts:32 body 37 (69 atoms, too expensive at final margin) |
| 4.8 | 9976 | 0.00 | 0.00 | missing | getSelectionKeys() + flatMap helpers | [scheduled bbox exact=8/17] export body at src/internals/helpers.ts:120 body 121 (t=5634, 8 atoms) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 7838 | 0.09 | 0.09 | missing | tests/ + docs/ + examples/ listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.1 | 603 | 0.80 | 0.99 | partial | match() exported signature | [scheduled bbox exact=4/5] export at src/match.ts:32 (t=235, 4 atoms) |
| 4.5 | 8816 | 0.00 | 0.00 | missing | internals/symbols.ts — core matcher / unset / isVariadic brands | [scheduled bbox exact=8/8] export names surface in src/internals/symbols.ts (t=8795, 8 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 10 | 752 | export at src/types/helpers.ts:<n> |
| 3 | 350 | export at src/types/DistributeUnions.ts:<n> |
| 3 | 349 | README.md section #<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |
| 3 | 196 | export at src/types/FindSelected.ts:<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 270 | 1.00 | 270 | 3207 | export names surface #2 in src/types/helpers.ts |
| 259 | 1.00 | 259 | 1791 | export names surface in src/types/helpers.ts |
| 217 | 0.79 | 273 | 566 | package identity in package.json |
| 212 | 0.91 | 233 | 1532 | export names surface in src/types/Pattern.ts |
| 205 | 1.00 | 205 | 2074 | export names surface #1 in src/types/helpers.ts |
| 177 | 0.78 | 228 | 2351 | export names surface #1 in src/types/Pattern.ts |
| 104 | 1.00 | 104 | 670 | README.md section #0 |
| 102 | 0.41 | 249 | 2870 | export names surface #2 in src/types/Pattern.ts |
| 63 | 1.00 | 63 | 126 | README headline in README.md |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 336 | 0.93 | 360 | 5994 | package entrypoints in package.json |
| 312 | 1.00 | 312 | 9107 | export body at src/patterns.ts:246 body 249 |
| 271 | 1.00 | 271 | 6265 | package scripts in package.json |
| 267 | 1.00 | 267 | 5018 | export names surface #3 in src/types/helpers.ts |
| 235 | 1.00 | 235 | 7330 | export body at src/patterns.ts:187 body 193 |
| 154 | 1.00 | 154 | 8639 | json config tsconfig.json |
| 145 | 1.00 | 145 | 7095 | export at src/patterns.ts:48 |
| 144 | 1.00 | 144 | 7909 | export names surface in src/types/FindSelected.ts |
| 144 | 1.00 | 144 | 3605 | headings outline in docs/v4-to-v5-migration-guide.md |
| 142 | 1.00 | 142 | 3461 | headings outline in docs/v3-to-v4-migration-guide.md |
| 2262 | — | — | — | +26 more rows |
