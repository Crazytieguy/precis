scores: Sim=0.308 Reached=13/45 Early=3 Late=6 Partial=6 Missing=26 Used=9918/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 5 | 1 | 1 | 0.81 |
| 2 | 13 | 3 | 1 | 9 | 0.28 |
| 3 | 17 | 4 | 3 | 10 | 0.47 |
| 4 | 8 | 1 | 1 | 6 | 0.26 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline |  |
| 1.2 | 151 | 1449 | +1298 | 1.00 | late | package.json identity fields | package identity in package.json (t=566, 4 atoms) |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | imports in src/index.ts (t=862, 3 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing |  |
| 1.5 | 364 | 2536 | +2172 | 1.00 | late | src/ + src/types/ + src/internals/ listings |  |
| 1.6 | 434 | 4587 | +4153 | 1.00 | late | README features — data structures + typesafety | README.md section #1 (t=4587, 3 atoms) |
| 1.7 | 546 | 4587 | +4041 | 1.00 | late | README features — patterns, wildcards, predicates, bundle | README.md section #1 (t=4587, 5 atoms) |
| 2.1 | 603 | 285 | -318 | 0.80 | early | match() exported signature | export at src/match.ts:32 (t=271, 4 atoms) |
| 2.3 | 772 | — | — | 0.00 | missing | MatchExpression — method-name catalog |  |
| 2.4 | 981 | — | — | 0.00 | missing | match() JSDoc |  |
| 2.5 | 1115 | — | — | 0.00 | missing | MatchExpression class doc + constructor |  |
| 2.6 | 1278 | — | — | 0.36 | missing | match.ts imports + MatchState/unmatched | imports in src/match.ts (t=5624, 5 atoms) |
| 2.7 | 1489 | — | — | 0.00 | missing | MatchExpression.with — argument parsing half |  |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half |  |
| 2.9 | 2017 | — | — | 0.00 | missing | MatchExpression.when / otherwise / exhaustive bodies |  |
| 2.10 | 2115 | — | — | 0.00 | missing | MatchExpression.run / returnType / narrow + defaultCatcher |  |
| 2.11 | 2477 | — | — | 0.13 | missing | isMatching — JSDoc + PatternConstraint helper | imports in src/is-matching.ts (t=5494, 4 atoms) |
| 2.12 | 2851 | — | — | 0.50 | partial | isMatching — JSDoc for two-arg + runtime body | export body at src/is-matching.ts:53 (t=4263, 12 atoms) |
| 2.13 | 3005 | 2387 | -618 | 0.87 | aligned | NonExhaustiveError class | export body at src/errors.ts:5 (t=935, 7 atoms) |
| 3.1 | 3161 | — | — | 0.67 | partial | P module-doc + combinator-function name catalog | export names surface in src/patterns.ts (t=6676, 77 atoms) |
| 3.2 | 3350 | 6676 | +3326 | 1.00 | late | P wildcards — every const definition | export names surface in src/patterns.ts (t=6676, 19 atoms) |
| 3.3 | 3422 | 6699 | +3277 | 1.00 | late | P type-level re-exports + matcher symbol | export names surface in src/patterns.ts (t=6676, 10 atoms) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog |  |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog |  |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | export at src/patterns.ts:572 (t=7275, 4 atoms) |
| 3.7 | 3971 | — | — | 0.25 | missing | P.not — JSDoc + signature | export at src/patterns.ts:611 (t=7010, 4 atoms) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | export at src/patterns.ts:637 (t=7324, 6 atoms) |
| 3.9 | 4659 | — | — | 0.69 | partial | P.select — JSDoc + all three overload signatures | export at src/patterns.ts:673 (t=8212, 13 atoms) |
| 3.10 | 4865 | — | — | 0.33 | missing | P.array — JSDoc + overload signatures | export at src/patterns.ts:242 (t=7053, 4 atoms) |
| 3.11 | 5071 | — | — | 0.38 | missing | P.optional — JSDoc + signature | export at src/patterns.ts:187 (t=7486, 6 atoms) |
| 3.12 | 5524 | — | — | 0.22 | missing | P.intersection / P.instanceOf — JSDoc + signatures | export names surface in src/patterns.ts (t=6676, 41 atoms) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | export at src/patterns.ts:437 (t=7658, 8 atoms) |
| 3.14 | 6315 | — | — | 0.33 | missing | P.map / P.set — JSDoc + signatures | export body at src/patterns.ts:299 (t=9267, 24 atoms) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable |  |
| 3.17 | 7489 | 8982 | +1493 | 0.93 | aligned | P.union / P.not / P.when — full bodies | export body at src/patterns.ts:572 (t=8982, 21 atoms) |
| 4.1 | 7838 | — | — | 0.05 | missing | tests/ + docs/ + examples/ listings |  |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | export names surface in src/types/Match.ts (t=2660, 2 atoms) |
| 4.3 | 8414 | — | — | 0.00 | missing | Pattern<T> — public pattern type alias + typed wildcards |  |
| 4.4 | 8718 | — | — | 0.00 | missing | MatcherType union + Matcher interface |  |
| 4.5 | 8816 | 5334 | -3482 | 1.00 | early | internals/symbols.ts — core matcher / unset / isVariadic brands | export names surface in src/internals/symbols.ts (t=5334, 8 atoms) |
| 4.6 | 9151 | — | — | 0.16 | missing | matchPattern() — Matcher / object / primitive branches | export at src/internals/helpers.ts:32 (t=2133, 6 atoms) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch |  |
| 4.8 | 9976 | — | — | 0.76 | partial | getSelectionKeys() + flatMap helpers | export body at src/internals/helpers.ts:120 (t=5432, 8 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 566 | README.md section #<n> |
| 3 | 350 | export at src/types/DistributeUnions.ts:<n> |
| 4 | 342 | export at src/types/FindSelected.ts:<n> |
| 3 | 290 | export at src/patterns.ts:<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 350 | 1.00 | 350 | 5974 | export at src/types/BuildMany.ts:19 |
| 339 | 1.00 | 339 | 9918 | export body at src/patterns.ts:696 |
| 336 | 0.93 | 360 | 1449 | package entrypoints in package.json |
| 312 | 1.00 | 312 | 9579 | export body at src/patterns.ts:246 |
| 285 | 1.00 | 285 | 9267 | export body at src/patterns.ts:299 |
| 271 | 1.00 | 271 | 1965 | package scripts in package.json |
| 235 | 1.00 | 235 | 8736 | export body at src/patterns.ts:187 |
| 217 | 1.00 | 217 | 4083 | README.md section #2 |
| 217 | 0.79 | 273 | 566 | package identity in package.json |
| 193 | 1.00 | 193 | 8501 | export body at src/patterns.ts:536 |
| 154 | 1.00 | 154 | 1089 | json config tsconfig.json |
| 146 | 1.00 | 146 | 5178 | export at src/types/FindSelected.ts:174 |
| 145 | 1.00 | 145 | 8062 | export at src/patterns.ts:48 |
| 144 | 1.00 | 144 | 4754 | export names surface in src/types/FindSelected.ts |
| 144 | 1.00 | 144 | 3840 | headings outline in docs/v4-to-v5-migration-guide.md |
| 142 | 1.00 | 142 | 3544 | headings outline in docs/v3-to-v4-migration-guide.md |
| 129 | 1.00 | 129 | 3696 | export at src/types/DistributeUnions.ts:183 |
| 129 | 1.00 | 129 | 2345 | package dependencies in package.json |
| 128 | 1.00 | 128 | 1577 | README.md section #3 |
| 128 | 1.00 | 128 | 3402 | export at src/types/DistributeUnions.ts:46 |
| 117 | 1.00 | 117 | 1694 | README.md section #4 |
| 104 | 1.00 | 104 | 823 | README.md section #0 |
| 99 | 1.00 | 99 | 2942 | export at src/types/InvertPattern.ts:180 |
| 99 | 1.00 | 99 | 3041 | export at src/types/InvertPattern.ts:192 |
| 95 | 1.00 | 95 | 7753 | export at src/patterns.ts:362 |
| 93 | 1.00 | 93 | 3252 | export at src/types/DistributeUnions.ts:174 |
| 93 | 1.00 | 93 | 2480 | json config jsr.json |
| 90 | 1.00 | 90 | 3131 | export names surface in src/types/DistributeUnions.ts |
| 83 | 0.13 | 652 | 6676 | export names surface in src/patterns.ts |
| 80 | 1.00 | 80 | 2801 | export names surface in src/types/InvertPattern.ts |
| 69 | 1.00 | 69 | 5032 | export at src/types/FindSelected.ts:159 |
| 68 | 1.00 | 68 | 4963 | export at src/types/FindSelected.ts:191 |
| 64 | 0.41 | 156 | 5334 | export names surface in src/internals/symbols.ts |
| 63 | 1.00 | 63 | 126 | README headline in README.md |
| 59 | 1.00 | 59 | 4895 | export at src/types/FindSelected.ts:165 |
| 55 | 1.00 | 55 | 5549 | export doc at src/types/InvertPattern.ts:106 |
| 50 | 1.00 | 50 | 7374 | export at src/patterns.ts:299 |
| 50 | 1.00 | 50 | 6024 | imports in src/internals/helpers.ts |
