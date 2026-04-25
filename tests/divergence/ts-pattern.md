scores: Sim=0.272 Reached=15/45 Early=3 Late=10 Partial=5 Missing=25 Used=9954/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 5 | 1 | 1 | 0.81 |
| 2 | 13 | 1 | 1 | 11 | 0.20 |
| 3 | 17 | 7 | 3 | 7 | 0.58 |
| 4 | 8 | 2 | 0 | 6 | 0.25 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline |  |
| 1.2 | 151 | 1550 | +1399 | 1.00 | late | package.json identity fields | package identity in package.json (t=384, 4 atoms) |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | imports in src/index.ts (t=1589, 3 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing |  |
| 1.5 | 364 | 2908 | +2544 | 1.00 | late | src/ + src/types/ + src/internals/ listings |  |
| 1.6 | 434 | 2852 | +2418 | 1.00 | late | README features — data structures + typesafety | README.md section #1 (t=2852, 3 atoms) |
| 1.7 | 546 | 2852 | +2306 | 1.00 | late | README features — patterns, wildcards, predicates, bundle | README.md section #1 (t=2852, 5 atoms) |
| 2.1 | 603 | — | — | 0.60 | partial | match() exported signature | export at src/match.ts:32 (t=626, 4 atoms) |
| 2.3 | 772 | — | — | 0.00 | missing | MatchExpression — method-name catalog |  |
| 2.4 | 981 | — | — | 0.00 | missing | match() JSDoc |  |
| 2.5 | 1115 | — | — | 0.00 | missing | MatchExpression class doc + constructor |  |
| 2.6 | 1278 | — | — | 0.36 | missing | match.ts imports + MatchState/unmatched | imports in src/match.ts (t=5199, 5 atoms) |
| 2.7 | 1489 | — | — | 0.00 | missing | MatchExpression.with — argument parsing half |  |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half |  |
| 2.9 | 2017 | — | — | 0.00 | missing | MatchExpression.when / otherwise / exhaustive bodies |  |
| 2.10 | 2115 | — | — | 0.00 | missing | MatchExpression.run / returnType / narrow + defaultCatcher |  |
| 2.11 | 2477 | — | — | 0.13 | missing | isMatching — JSDoc + PatternConstraint helper | imports in src/is-matching.ts (t=5098, 4 atoms) |
| 2.12 | 2851 | — | — | 0.10 | missing | isMatching — JSDoc for two-arg + runtime body | export names surface in src/is-matching.ts (t=694, 4 atoms) |
| 2.13 | 3005 | — | — | 0.40 | missing | NonExhaustiveError class | export doc at src/errors.ts:5 (t=4312, 4 atoms) |
| 3.1 | 3161 | — | — | 0.67 | partial | P module-doc + combinator-function name catalog | export names surface in src/patterns.ts (t=6278, 77 atoms) |
| 3.2 | 3350 | 6278 | +2928 | 1.00 | late | P wildcards — every const definition | export names surface in src/patterns.ts (t=6278, 19 atoms) |
| 3.3 | 3422 | 6301 | +2879 | 1.00 | late | P type-level re-exports + matcher symbol | export doc at src/patterns.ts:81 (t=9139, 14 atoms) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog |  |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog |  |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | export at src/patterns.ts:572 (t=7406, 4 atoms) |
| 3.7 | 3971 | 9568 | +5597 | 0.94 | late | P.not — JSDoc + signature | export doc at src/patterns.ts:611 (t=9568, 11 atoms) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | export at src/patterns.ts:637 (t=6658, 6 atoms) |
| 3.9 | 4659 | 9424 | +4765 | 1.00 | late | P.select — JSDoc + all three overload signatures | export at src/patterns.ts:673 (t=7361, 13 atoms) |
| 3.10 | 4865 | 9281 | +4416 | 1.00 | late | P.array — JSDoc + overload signatures | export doc at src/patterns.ts:241 (t=9281, 10 atoms) |
| 3.11 | 5071 | 9714 | +4643 | 1.00 | late | P.optional — JSDoc + signature | export doc at src/patterns.ts:187 (t=9714, 10 atoms) |
| 3.12 | 5524 | — | — | 0.42 | missing | P.intersection / P.instanceOf — JSDoc + signatures | export names surface in src/patterns.ts (t=6278, 41 atoms) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | export at src/patterns.ts:437 (t=6924, 8 atoms) |
| 3.14 | 6315 | — | — | 0.64 | partial | P.map / P.set — JSDoc + signatures | export doc at src/patterns.ts:294 (t=8226, 10 atoms) |
| 3.15 | 6498 | 6478 | -20 | 0.93 | aligned | P.shape + matcher protocol re-exports | export names surface in src/patterns.ts (t=6278, 76 atoms) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable |  |
| 3.17 | 7489 | — | — | 0.13 | missing | P.union / P.not / P.when — full bodies | export doc at src/patterns.ts:611 (t=9568, 11 atoms) |
| 4.1 | 7838 | — | — | 0.05 | missing | tests/ + docs/ + examples/ listings |  |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | export names surface in src/types/Match.ts (t=3022, 2 atoms) |
| 4.3 | 8414 | — | — | 0.00 | missing | Pattern<T> — public pattern type alias + typed wildcards |  |
| 4.4 | 8718 | — | — | 0.00 | missing | MatcherType union + Matcher interface |  |
| 4.5 | 8816 | 5036 | -3780 | 1.00 | early | internals/symbols.ts — core matcher / unset / isVariadic brands | export names surface in src/internals/symbols.ts (t=5036, 8 atoms) |
| 4.6 | 9151 | — | — | 0.03 | missing | matchPattern() — Matcher / object / primitive branches | export names surface in src/internals/helpers.ts (t=2176, 2 atoms) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch |  |
| 4.8 | 9976 | 4270 | -5706 | 0.82 | early | getSelectionKeys() + flatMap helpers | export at src/internals/helpers.ts:120 (t=4270, 10 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 11 | 1318 | export doc at src/patterns.ts:<n> |
| 4 | 566 | README.md section #<n> |
| 3 | 350 | export at src/types/DistributeUnions.ts:<n> |
| 4 | 342 | export at src/types/FindSelected.ts:<n> |
| 3 | 290 | export at src/patterns.ts:<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 350 | 1.00 | 350 | 5626 | export at src/types/BuildMany.ts:19 |
| 338 | 0.94 | 360 | 1550 | package entrypoints in package.json |
| 271 | 1.00 | 271 | 2095 | package scripts in package.json |
| 227 | 0.83 | 273 | 384 | package identity in package.json |
| 217 | 1.00 | 217 | 1806 | README.md section #2 |
| 195 | 0.30 | 652 | 6278 | export names surface in src/patterns.ts |
| 155 | 1.00 | 155 | 9139 | export doc at src/patterns.ts:81 |
| 154 | 1.00 | 154 | 1190 | json config tsconfig.json |
| 146 | 1.00 | 146 | 4880 | export at src/types/FindSelected.ts:174 |
| 145 | 1.00 | 145 | 7211 | export at src/patterns.ts:48 |
| 144 | 1.00 | 144 | 4456 | export names surface in src/types/FindSelected.ts |
| 144 | 1.00 | 144 | 4167 | headings outline in docs/v4-to-v5-migration-guide.md |
| 142 | 1.00 | 142 | 4023 | headings outline in docs/v3-to-v4-migration-guide.md |
| 134 | 1.00 | 134 | 8095 | export doc at src/patterns.ts:100 |
| 129 | 1.00 | 129 | 3881 | export at src/types/DistributeUnions.ts:183 |
| 129 | 1.00 | 129 | 2435 | package dependencies in package.json |
| 128 | 1.00 | 128 | 1003 | README.md section #3 |
| 128 | 1.00 | 128 | 3752 | export at src/types/DistributeUnions.ts:46 |
| 123 | 1.00 | 123 | 7842 | export doc at src/patterns.ts:116 |
| 120 | 1.00 | 120 | 9834 | export doc at src/patterns.ts:1075 |
| 120 | 1.00 | 120 | 9954 | export doc at src/patterns.ts:1231 |
| 117 | 1.00 | 117 | 875 | README.md section #4 |
| 114 | 1.00 | 114 | 8984 | export doc at src/patterns.ts:1201 |
| 112 | 1.00 | 112 | 8870 | export doc at src/patterns.ts:793 |
| 111 | 1.00 | 111 | 8758 | export doc at src/patterns.ts:1211 |
| 111 | 1.00 | 111 | 8647 | export doc at src/patterns.ts:928 |
| 109 | 1.00 | 109 | 8444 | export doc at src/patterns.ts:1221 |
| 109 | 1.00 | 109 | 8335 | export doc at src/patterns.ts:782 |
| 104 | 1.00 | 104 | 488 | README.md section #0 |
| 99 | 1.00 | 99 | 3314 | export at src/types/InvertPattern.ts:180 |
| 99 | 1.00 | 99 | 3413 | export at src/types/InvertPattern.ts:192 |
| 95 | 1.00 | 95 | 7683 | export at src/patterns.ts:362 |
| 93 | 1.00 | 93 | 3624 | export at src/types/DistributeUnions.ts:174 |
| 93 | 1.00 | 93 | 2528 | json config jsr.json |
| 90 | 1.00 | 90 | 3503 | export names surface in src/types/DistributeUnions.ts |
| 80 | 1.00 | 80 | 3173 | export names surface in src/types/InvertPattern.ts |
| 70 | 1.00 | 70 | 2306 | export at src/internals/helpers.ts:16 |
| 69 | 1.00 | 69 | 4734 | export at src/types/FindSelected.ts:159 |
| 68 | 1.00 | 68 | 4665 | export at src/types/FindSelected.ts:191 |
| 66 | 0.43 | 156 | 5036 | export names surface in src/internals/symbols.ts |
| 63 | 1.00 | 63 | 111 | README headline in README.md |
| 59 | 1.00 | 59 | 4597 | export at src/types/FindSelected.ts:165 |
| 55 | 1.00 | 55 | 5254 | export doc at src/types/InvertPattern.ts:106 |
| 50 | 1.00 | 50 | 7516 | export at src/patterns.ts:299 |
| 50 | 1.00 | 50 | 8494 | imports in src/internals/helpers.ts |
