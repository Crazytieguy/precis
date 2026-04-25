scores: Sim=0.276 Reached=16/45 Early=3 Late=11 Partial=7 Missing=22 Used=9906/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 5 | 1 | 1 | 0.81 |
| 2 | 13 | 2 | 3 | 8 | 0.35 |
| 3 | 17 | 7 | 3 | 7 | 0.58 |
| 4 | 8 | 2 | 0 | 6 | 0.25 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline |  |
| 1.2 | 151 | 1550 | +1399 | 1.00 | late | package.json identity fields | package identity in package.json (t=384, 4 atoms) |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | imports in src/index.ts (t=1589, 3 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing |  |
| 1.5 | 364 | 3053 | +2689 | 1.00 | late | src/ + src/types/ + src/internals/ listings |  |
| 1.6 | 434 | 2997 | +2563 | 1.00 | late | README features — data structures + typesafety | README.md section #1 (t=2997, 3 atoms) |
| 1.7 | 546 | 2997 | +2451 | 1.00 | late | README features — patterns, wildcards, predicates, bundle | README.md section #1 (t=2997, 5 atoms) |
| 2.1 | 603 | — | — | 0.60 | partial | match() exported signature | export at src/match.ts:32 (t=626, 4 atoms) |
| 2.3 | 772 | — | — | 0.00 | missing | MatchExpression — method-name catalog |  |
| 2.4 | 981 | 5910 | +4929 | 1.00 | late | match() JSDoc | export doc at src/match.ts:32 (t=5910, 16 atoms) |
| 2.5 | 1115 | — | — | 0.00 | missing | MatchExpression class doc + constructor |  |
| 2.6 | 1278 | — | — | 0.36 | missing | match.ts imports + MatchState/unmatched | imports in src/match.ts (t=5701, 5 atoms) |
| 2.7 | 1489 | — | — | 0.00 | missing | MatchExpression.with — argument parsing half |  |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half |  |
| 2.9 | 2017 | — | — | 0.00 | missing | MatchExpression.when / otherwise / exhaustive bodies |  |
| 2.10 | 2115 | — | — | 0.00 | missing | MatchExpression.run / returnType / narrow + defaultCatcher |  |
| 2.11 | 2477 | — | — | 0.61 | partial | isMatching — JSDoc + PatternConstraint helper | export doc at src/is-matching.ts:32 (t=5626, 15 atoms) |
| 2.12 | 2851 | — | — | 0.53 | partial | isMatching — JSDoc for two-arg + runtime body | export doc at src/is-matching.ts:48 (t=4980, 13 atoms) |
| 2.13 | 3005 | — | — | 0.40 | missing | NonExhaustiveError class | export doc at src/errors.ts:5 (t=1631, 4 atoms) |
| 3.1 | 3161 | — | — | 0.67 | partial | P module-doc + combinator-function name catalog | export names surface in src/patterns.ts (t=6562, 77 atoms) |
| 3.2 | 3350 | 6562 | +3212 | 1.00 | late | P wildcards — every const definition | export names surface in src/patterns.ts (t=6562, 19 atoms) |
| 3.3 | 3422 | 6585 | +3163 | 1.00 | late | P type-level re-exports + matcher symbol | export doc at src/patterns.ts:81 (t=9105, 14 atoms) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog |  |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog |  |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | export at src/patterns.ts:572 (t=7690, 4 atoms) |
| 3.7 | 3971 | 9760 | +5789 | 0.94 | late | P.not — JSDoc + signature | export doc at src/patterns.ts:611 (t=9760, 11 atoms) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | export at src/patterns.ts:637 (t=6942, 6 atoms) |
| 3.9 | 4659 | 9616 | +4957 | 1.00 | late | P.select — JSDoc + all three overload signatures | export at src/patterns.ts:673 (t=7645, 13 atoms) |
| 3.10 | 4865 | 9359 | +4494 | 1.00 | late | P.array — JSDoc + overload signatures | export doc at src/patterns.ts:241 (t=9359, 10 atoms) |
| 3.11 | 5071 | 9906 | +4835 | 1.00 | late | P.optional — JSDoc + signature | export doc at src/patterns.ts:187 (t=9906, 10 atoms) |
| 3.12 | 5524 | — | — | 0.42 | missing | P.intersection / P.instanceOf — JSDoc + signatures | export names surface in src/patterns.ts (t=6562, 41 atoms) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | export at src/patterns.ts:437 (t=7208, 8 atoms) |
| 3.14 | 6315 | — | — | 0.64 | partial | P.map / P.set — JSDoc + signatures | export doc at src/patterns.ts:294 (t=8510, 10 atoms) |
| 3.15 | 6498 | 6762 | +264 | 0.93 | aligned | P.shape + matcher protocol re-exports | export names surface in src/patterns.ts (t=6562, 76 atoms) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable |  |
| 3.17 | 7489 | — | — | 0.13 | missing | P.union / P.not / P.when — full bodies | export doc at src/patterns.ts:611 (t=9760, 11 atoms) |
| 4.1 | 7838 | — | — | 0.05 | missing | tests/ + docs/ + examples/ listings |  |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | export names surface in src/types/Match.ts (t=3167, 2 atoms) |
| 4.3 | 8414 | — | — | 0.00 | missing | Pattern<T> — public pattern type alias + typed wildcards |  |
| 4.4 | 8718 | — | — | 0.00 | missing | MatcherType union + Matcher interface |  |
| 4.5 | 8816 | 4827 | -3989 | 1.00 | early | internals/symbols.ts — core matcher / unset / isVariadic brands | export names surface in src/internals/symbols.ts (t=4827, 8 atoms) |
| 4.6 | 9151 | — | — | 0.03 | missing | matchPattern() — Matcher / object / primitive branches | export names surface in src/internals/helpers.ts (t=2218, 2 atoms) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch |  |
| 4.8 | 9976 | 2580 | -7396 | 0.82 | early | getSelectionKeys() + flatMap helpers | export at src/internals/helpers.ts:120 (t=2580, 10 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 9 | 1078 | export doc at src/patterns.ts:<n> |
| 4 | 566 | README.md section #<n> |
| 3 | 350 | export at src/types/DistributeUnions.ts:<n> |
| 4 | 342 | export at src/types/FindSelected.ts:<n> |
| 3 | 290 | export at src/patterns.ts:<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 350 | 1.00 | 350 | 5392 | export at src/types/BuildMany.ts:19 |
| 338 | 0.94 | 360 | 1550 | package entrypoints in package.json |
| 271 | 1.00 | 271 | 2137 | package scripts in package.json |
| 227 | 0.83 | 273 | 384 | package identity in package.json |
| 217 | 1.00 | 217 | 1848 | README.md section #2 |
| 195 | 0.30 | 652 | 6562 | export names surface in src/patterns.ts |
| 155 | 1.00 | 155 | 9105 | export doc at src/patterns.ts:81 |
| 154 | 1.00 | 154 | 1190 | json config tsconfig.json |
| 146 | 1.00 | 146 | 4671 | export at src/types/FindSelected.ts:174 |
| 145 | 1.00 | 145 | 7495 | export at src/patterns.ts:48 |
| 144 | 1.00 | 144 | 4247 | export names surface in src/types/FindSelected.ts |
| 134 | 1.00 | 134 | 8379 | export doc at src/patterns.ts:100 |
| 129 | 1.00 | 129 | 4026 | export at src/types/DistributeUnions.ts:183 |
| 129 | 1.00 | 129 | 2477 | package dependencies in package.json |
| 128 | 1.00 | 128 | 1003 | README.md section #3 |
| 128 | 1.00 | 128 | 3897 | export at src/types/DistributeUnions.ts:46 |
| 123 | 1.00 | 123 | 8126 | export doc at src/patterns.ts:116 |
| 117 | 1.00 | 117 | 875 | README.md section #4 |
| 114 | 1.00 | 114 | 9473 | export doc at src/patterns.ts:1201 |
| 112 | 1.00 | 112 | 9217 | export doc at src/patterns.ts:793 |
| 111 | 1.00 | 111 | 8950 | export doc at src/patterns.ts:1211 |
| 111 | 1.00 | 111 | 8839 | export doc at src/patterns.ts:928 |
| 109 | 1.00 | 109 | 8728 | export doc at src/patterns.ts:1221 |
| 109 | 1.00 | 109 | 8619 | export doc at src/patterns.ts:782 |
| 104 | 1.00 | 104 | 488 | README.md section #0 |
| 99 | 1.00 | 99 | 3459 | export at src/types/InvertPattern.ts:180 |
| 99 | 1.00 | 99 | 3558 | export at src/types/InvertPattern.ts:192 |
| 95 | 1.00 | 95 | 7967 | export at src/patterns.ts:362 |
| 93 | 1.00 | 93 | 3769 | export at src/types/DistributeUnions.ts:174 |
| 93 | 1.00 | 93 | 2673 | json config jsr.json |
| 90 | 1.00 | 90 | 3648 | export names surface in src/types/DistributeUnions.ts |
| 80 | 1.00 | 80 | 3318 | export names surface in src/types/InvertPattern.ts |
| 70 | 1.00 | 70 | 2348 | export at src/internals/helpers.ts:16 |
| 69 | 1.00 | 69 | 4525 | export at src/types/FindSelected.ts:159 |
| 68 | 1.00 | 68 | 4456 | export at src/types/FindSelected.ts:191 |
| 66 | 0.43 | 156 | 4827 | export names surface in src/internals/symbols.ts |
| 63 | 1.00 | 63 | 111 | README headline in README.md |
| 59 | 1.00 | 59 | 4388 | export at src/types/FindSelected.ts:165 |
| 55 | 1.00 | 55 | 4081 | export doc at src/types/InvertPattern.ts:106 |
| 50 | 1.00 | 50 | 7800 | export at src/patterns.ts:299 |
