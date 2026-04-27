scores: Sim=0.330 Reached=11/45 Early=4 Late=4 Partial=7 Missing=27 Used=9918/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 3 | 1 | 3 | 0.52 |
| 2 | 13 | 3 | 1 | 9 | 0.28 |
| 3 | 17 | 4 | 4 | 9 | 0.50 |
| 4 | 8 | 1 | 1 | 6 | 0.26 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline |  |
| 1.2 | 151 | 3304 | +3153 | 1.00 | late | package.json identity fields | package identity in package.json (t=566, 4 atoms) |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | imports in src/index.ts (t=862, 3 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing |  |
| 1.5 | 364 | 1305 | +941 | 1.00 | late | src/ + src/types/ + src/internals/ listings |  |
| 1.6 | 434 | — | — | 0.00 | missing | README features — data structures + typesafety |  |
| 1.7 | 546 | — | — | 0.00 | missing | README features — patterns, wildcards, predicates, bundle |  |
| 2.1 | 603 | 285 | -318 | 0.80 | early | match() exported signature | export at src/match.ts:32 (t=271, 4 atoms) |
| 2.3 | 772 | — | — | 0.00 | missing | MatchExpression — method-name catalog |  |
| 2.4 | 981 | — | — | 0.00 | missing | match() JSDoc |  |
| 2.5 | 1115 | — | — | 0.00 | missing | MatchExpression class doc + constructor |  |
| 2.6 | 1278 | — | — | 0.36 | missing | match.ts imports + MatchState/unmatched | imports in src/match.ts (t=4286, 5 atoms) |
| 2.7 | 1489 | — | — | 0.00 | missing | MatchExpression.with — argument parsing half |  |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half |  |
| 2.9 | 2017 | — | — | 0.00 | missing | MatchExpression.when / otherwise / exhaustive bodies |  |
| 2.10 | 2115 | — | — | 0.00 | missing | MatchExpression.run / returnType / narrow + defaultCatcher |  |
| 2.11 | 2477 | — | — | 0.13 | missing | isMatching — JSDoc + PatternConstraint helper | imports in src/is-matching.ts (t=3766, 4 atoms) |
| 2.12 | 2851 | — | — | 0.50 | partial | isMatching — JSDoc for two-arg + runtime body | export body at src/is-matching.ts:53 (t=2222, 12 atoms) |
| 2.13 | 3005 | 1249 | -1756 | 0.87 | early | NonExhaustiveError class | export body at src/errors.ts:5 (t=935, 7 atoms) |
| 3.1 | 3161 | — | — | 0.67 | partial | P module-doc + combinator-function name catalog | export names surface in src/patterns.ts (t=5751, 77 atoms) |
| 3.2 | 3350 | 5751 | +2401 | 1.00 | late | P wildcards — every const definition | export names surface in src/patterns.ts (t=5751, 19 atoms) |
| 3.3 | 3422 | 5774 | +2352 | 1.00 | late | P type-level re-exports + matcher symbol | export doc at src/patterns.ts:100 (t=9369, 13 atoms) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog |  |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog |  |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | export at src/patterns.ts:572 (t=6350, 4 atoms) |
| 3.7 | 3971 | — | — | 0.25 | missing | P.not — JSDoc + signature | export at src/patterns.ts:611 (t=6085, 4 atoms) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | export at src/patterns.ts:637 (t=6399, 6 atoms) |
| 3.9 | 4659 | — | — | 0.69 | partial | P.select — JSDoc + all three overload signatures | export at src/patterns.ts:673 (t=7287, 13 atoms) |
| 3.10 | 4865 | — | — | 0.33 | missing | P.array — JSDoc + overload signatures | export at src/patterns.ts:242 (t=6128, 4 atoms) |
| 3.11 | 5071 | — | — | 0.38 | missing | P.optional — JSDoc + signature | export at src/patterns.ts:187 (t=6561, 6 atoms) |
| 3.12 | 5524 | — | — | 0.44 | missing | P.intersection / P.instanceOf — JSDoc + signatures | export names surface in src/patterns.ts (t=5751, 41 atoms) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | export at src/patterns.ts:437 (t=6733, 8 atoms) |
| 3.14 | 6315 | — | — | 0.64 | partial | P.map / P.set — JSDoc + signatures | export body at src/patterns.ts:299 (t=8342, 24 atoms) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable |  |
| 3.17 | 7489 | 8057 | +568 | 0.93 | aligned | P.union / P.not / P.when — full bodies | export body at src/patterns.ts:572 (t=8057, 21 atoms) |
| 4.1 | 7838 | — | — | 0.05 | missing | tests/ + docs/ + examples/ listings |  |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | export names surface in src/types/Match.ts (t=1429, 2 atoms) |
| 4.3 | 8414 | — | — | 0.00 | missing | Pattern<T> — public pattern type alias + typed wildcards |  |
| 4.4 | 8718 | — | — | 0.00 | missing | MatcherType union + Matcher interface |  |
| 4.5 | 8816 | 2823 | -5993 | 1.00 | early | internals/symbols.ts — core matcher / unset / isVariadic brands | export names surface in src/internals/symbols.ts (t=2823, 8 atoms) |
| 4.6 | 9151 | — | — | 0.16 | missing | matchPattern() — Matcher / object / primitive branches | export at src/internals/helpers.ts:32 (t=1124, 6 atoms) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch |  |
| 4.8 | 9976 | — | — | 0.76 | partial | getSelectionKeys() + flatMap helpers | export body at src/internals/helpers.ts:120 (t=2944, 8 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 350 | export at src/types/DistributeUnions.ts:<n> |
| 3 | 349 | README.md section #<n> |
| 4 | 342 | export at src/types/FindSelected.ts:<n> |
| 3 | 290 | export at src/patterns.ts:<n> |
| 2 | 257 | export doc at src/patterns.ts:<n> |
| 2 | 198 | export at src/types/InvertPattern.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 418 | 1.00 | 418 | 9787 | export body at src/patterns.ts:362 |
| 339 | 1.00 | 339 | 8993 | export body at src/patterns.ts:696 |
| 336 | 0.93 | 360 | 3304 | package entrypoints in package.json |
| 312 | 1.00 | 312 | 8654 | export body at src/patterns.ts:246 |
| 285 | 1.00 | 285 | 8342 | export body at src/patterns.ts:299 |
| 271 | 1.00 | 271 | 3575 | package scripts in package.json |
| 235 | 1.00 | 235 | 7811 | export body at src/patterns.ts:187 |
| 217 | 0.79 | 273 | 566 | package identity in package.json |
| 193 | 1.00 | 193 | 7576 | export body at src/patterns.ts:536 |
| 154 | 1.00 | 154 | 4211 | json config tsconfig.json |
| 146 | 1.00 | 146 | 5006 | export at src/types/FindSelected.ts:174 |
| 145 | 1.00 | 145 | 7137 | export at src/patterns.ts:48 |
| 144 | 1.00 | 144 | 2389 | export names surface in src/types/FindSelected.ts |
| 144 | 1.00 | 144 | 2016 | headings outline in docs/v4-to-v5-migration-guide.md |
| 142 | 1.00 | 142 | 1872 | headings outline in docs/v3-to-v4-migration-guide.md |
| 134 | 1.00 | 134 | 9369 | export doc at src/patterns.ts:100 |
| 129 | 1.00 | 129 | 4810 | export at src/types/DistributeUnions.ts:183 |
| 129 | 1.00 | 129 | 3704 | package dependencies in package.json |
| 128 | 1.00 | 128 | 4681 | README.md section #3 |
| 128 | 1.00 | 128 | 4553 | export at src/types/DistributeUnions.ts:46 |
| 123 | 1.00 | 123 | 9116 | export doc at src/patterns.ts:116 |
| 117 | 1.00 | 117 | 4425 | README.md section #4 |
| 104 | 1.00 | 104 | 823 | README.md section #0 |
| 99 | 1.00 | 99 | 3958 | export at src/types/InvertPattern.ts:180 |
| 99 | 1.00 | 99 | 4057 | export at src/types/InvertPattern.ts:192 |
| 95 | 1.00 | 95 | 6828 | export at src/patterns.ts:362 |
| 93 | 1.00 | 93 | 3859 | export at src/types/DistributeUnions.ts:174 |
| 93 | 1.00 | 93 | 5099 | json config jsr.json |
| 90 | 1.00 | 90 | 1702 | export names surface in src/types/DistributeUnions.ts |
| 83 | 0.13 | 652 | 5751 | export names surface in src/patterns.ts |
| 80 | 1.00 | 80 | 1570 | export names surface in src/types/InvertPattern.ts |
| 69 | 1.00 | 69 | 2667 | export at src/types/FindSelected.ts:159 |
| 68 | 1.00 | 68 | 2598 | export at src/types/FindSelected.ts:191 |
| 64 | 0.41 | 156 | 2823 | export names surface in src/internals/symbols.ts |
| 63 | 1.00 | 63 | 126 | README headline in README.md |
| 59 | 1.00 | 59 | 2530 | export at src/types/FindSelected.ts:165 |
| 50 | 1.00 | 50 | 6449 | export at src/patterns.ts:299 |
| 50 | 1.00 | 50 | 4860 | imports in src/internals/helpers.ts |
