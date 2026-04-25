scores: Sim=0.285 Reached=12/45 Early=4 Late=7 Partial=7 Missing=26 Used=9961/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 4 | 1 | 2 | 0.67 |
| 2 | 13 | 2 | 3 | 8 | 0.35 |
| 3 | 17 | 3 | 3 | 11 | 0.42 |
| 4 | 8 | 3 | 0 | 5 | 0.37 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 71 | — | — | 0.00 | missing | README lede + tagline |  |
| 1.2 | 151 | — | — | 0.00 | missing | package.json identity fields |  |
| 1.3 | 223 | — | — | 0.67 | partial | src/index.ts — full public re-export surface | imports in src/index.ts (t=838, 3 atoms) |
| 1.4 | 271 | 48 | -223 | 1.00 | early | Repo root listing |  |
| 1.5 | 364 | 1839 | +1475 | 1.00 | late | src/ + src/types/ + src/internals/ listings |  |
| 1.6 | 434 | 1783 | +1349 | 1.00 | late | README features — data structures + typesafety | README.md section #1 (t=1783, 3 atoms) |
| 1.7 | 546 | 1783 | +1237 | 1.00 | late | README features — patterns, wildcards, predicates, bundle | README.md section #1 (t=1783, 5 atoms) |
| 2.1 | 603 | — | — | 0.60 | partial | match() exported signature | export at src/match.ts:32 (t=389, 4 atoms) |
| 2.3 | 772 | — | — | 0.00 | missing | MatchExpression — method-name catalog |  |
| 2.4 | 981 | 7486 | +6505 | 1.00 | late | match() JSDoc | export doc at src/match.ts:32 (t=7486, 16 atoms) |
| 2.5 | 1115 | — | — | 0.00 | missing | MatchExpression class doc + constructor |  |
| 2.6 | 1278 | — | — | 0.36 | missing | match.ts imports + MatchState/unmatched | imports in src/match.ts (t=4668, 5 atoms) |
| 2.7 | 1489 | — | — | 0.00 | missing | MatchExpression.with — argument parsing half |  |
| 2.8 | 1764 | — | — | 0.00 | missing | MatchExpression.with — selection + dispatch half |  |
| 2.9 | 2017 | — | — | 0.00 | missing | MatchExpression.when / otherwise / exhaustive bodies |  |
| 2.10 | 2115 | — | — | 0.00 | missing | MatchExpression.run / returnType / narrow + defaultCatcher |  |
| 2.11 | 2477 | — | — | 0.61 | partial | isMatching — JSDoc + PatternConstraint helper | export doc at src/is-matching.ts:32 (t=4593, 15 atoms) |
| 2.12 | 2851 | — | — | 0.53 | partial | isMatching — JSDoc for two-arg + runtime body | export doc at src/is-matching.ts:48 (t=3868, 13 atoms) |
| 2.13 | 3005 | — | — | 0.40 | missing | NonExhaustiveError class | export doc at src/errors.ts:5 (t=880, 4 atoms) |
| 3.1 | 3161 | — | — | 0.67 | partial | P module-doc + combinator-function name catalog | export names surface in src/patterns.ts (t=8773, 77 atoms) |
| 3.2 | 3350 | 8773 | +5423 | 1.00 | late | P wildcards — every const definition | export names surface in src/patterns.ts (t=8773, 19 atoms) |
| 3.3 | 3422 | 8796 | +5374 | 1.00 | late | P type-level re-exports + matcher symbol | export names surface in src/patterns.ts (t=8773, 10 atoms) |
| 3.4 | 3482 | — | — | 0.00 | missing | P.string predicates — name catalog |  |
| 3.5 | 3560 | — | — | 0.00 | missing | P.number / P.bigint predicates — name catalog |  |
| 3.6 | 3780 | — | — | 0.24 | missing | P.union — JSDoc + signature | export at src/patterns.ts:572 (t=9901, 4 atoms) |
| 3.7 | 3971 | — | — | 0.25 | missing | P.not — JSDoc + signature | export at src/patterns.ts:611 (t=9273, 4 atoms) |
| 3.8 | 4233 | — | — | 0.45 | missing | P.when — JSDoc + both overload signatures | export at src/patterns.ts:637 (t=9153, 6 atoms) |
| 3.9 | 4659 | — | — | 0.69 | partial | P.select — JSDoc + all three overload signatures | export at src/patterns.ts:673 (t=9856, 13 atoms) |
| 3.10 | 4865 | — | — | 0.33 | missing | P.array — JSDoc + overload signatures | export at src/patterns.ts:242 (t=9016, 4 atoms) |
| 3.11 | 5071 | — | — | 0.38 | missing | P.optional — JSDoc + signature | export at src/patterns.ts:187 (t=9961, 6 atoms) |
| 3.12 | 5524 | — | — | 0.19 | missing | P.intersection / P.instanceOf — JSDoc + signatures | export names surface in src/patterns.ts (t=8773, 41 atoms) |
| 3.13 | 5846 | — | — | 0.50 | partial | P.record — JSDoc + signatures | export at src/patterns.ts:437 (t=9419, 8 atoms) |
| 3.14 | 6315 | — | — | 0.33 | missing | P.map / P.set — JSDoc + signatures | export names surface in src/patterns.ts (t=8773, 8 atoms) |
| 3.15 | 6498 | 8973 | +2475 | 0.93 | late | P.shape + matcher protocol re-exports | export names surface in src/patterns.ts (t=8773, 76 atoms) |
| 3.16 | 6988 | — | — | 0.00 | missing | chainable() factory + variadic / arrayChainable |  |
| 3.17 | 7489 | — | — | 0.13 | missing | P.union / P.not / P.when — full bodies | export names surface in src/patterns.ts (t=8773, 8 atoms) |
| 4.1 | 7838 | 4966 | -2872 | 1.00 | early | tests/ + docs/ + examples/ listings |  |
| 4.2 | 7921 | — | — | 0.12 | missing | Match<i, o> — public builder type signature | export names surface in src/types/Match.ts (t=1953, 2 atoms) |
| 4.3 | 8414 | — | — | 0.00 | missing | Pattern<T> — public pattern type alias + typed wildcards |  |
| 4.4 | 8718 | — | — | 0.00 | missing | MatcherType union + Matcher interface |  |
| 4.5 | 8816 | 3715 | -5101 | 1.00 | early | internals/symbols.ts — core matcher / unset / isVariadic brands | export names surface in src/internals/symbols.ts (t=3715, 8 atoms) |
| 4.6 | 9151 | — | — | 0.03 | missing | matchPattern() — Matcher / object / primitive branches | export names surface in src/internals/helpers.ts (t=1226, 2 atoms) |
| 4.7 | 9767 | — | — | 0.00 | missing | matchPattern() — array / tuple / variadic branch |  |
| 4.8 | 9976 | 1459 | -8517 | 0.82 | early | getSelectionKeys() + flatMap helpers | export at src/internals/helpers.ts:120 (t=1459, 10 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 350 | 1.00 | 350 | 4375 | export at src/types/BuildMany.ts:19 |
| 232 | 1.00 | 232 | 7199 | export at tests/types-catalog/utils.ts:26 |
| 217 | 1.00 | 217 | 1097 | README.md section #2 |
| 212 | 1.00 | 212 | 6619 | export names surface in tests/types-catalog/utils.ts |
| 195 | 0.30 | 652 | 8773 | export names surface in src/patterns.ts |
| 146 | 1.00 | 146 | 3559 | export at src/types/FindSelected.ts:174 |
| 145 | 1.00 | 145 | 9706 | export at src/patterns.ts:48 |
| 144 | 1.00 | 144 | 3135 | export names surface in src/types/FindSelected.ts |
| 129 | 1.00 | 129 | 2812 | export at src/types/DistributeUnions.ts:183 |
| 128 | 1.00 | 128 | 766 | README.md section #3 |
| 128 | 1.00 | 128 | 2683 | export at src/types/DistributeUnions.ts:46 |
| 117 | 1.00 | 117 | 638 | README.md section #4 |
| 107 | 1.00 | 107 | 5091 | export at tests/types-catalog/definition.ts:585 |
| 104 | 1.00 | 104 | 215 | README.md section #0 |
| 99 | 1.00 | 99 | 2245 | export at src/types/InvertPattern.ts:180 |
| 99 | 1.00 | 99 | 2344 | export at src/types/InvertPattern.ts:192 |
| 97 | 1.00 | 97 | 8121 | imports in tests/exhaustive-match.test.ts |
| 95 | 1.00 | 95 | 8024 | imports in tests/generics.test.ts |
| 94 | 1.00 | 94 | 7929 | imports in tests/extract-precise-value.test.ts |
| 93 | 1.00 | 93 | 2555 | export at src/types/DistributeUnions.ts:174 |
| 90 | 1.00 | 90 | 2434 | export names surface in src/types/DistributeUnions.ts |
| 90 | 1.00 | 90 | 7835 | imports in tests/distribute-unions.test.ts |
| 88 | 1.00 | 88 | 7745 | imports in tests/select.test.ts |
| 87 | 1.00 | 87 | 7657 | imports in tests/helpers.test.ts |
| 84 | 1.00 | 84 | 7570 | imports in tests/wildcards.test.ts |
| 80 | 1.00 | 80 | 2104 | export names surface in src/types/InvertPattern.ts |
| 78 | 1.00 | 78 | 7277 | imports in tests/invert-pattern.test.ts |
| 70 | 1.00 | 70 | 1356 | export at src/internals/helpers.ts:16 |
| 69 | 1.00 | 69 | 3413 | export at src/types/FindSelected.ts:159 |
| 68 | 1.00 | 68 | 3344 | export at src/types/FindSelected.ts:191 |
| 66 | 0.43 | 156 | 3715 | export names surface in src/internals/symbols.ts |
| 63 | 1.00 | 63 | 111 | README headline in README.md |
| 61 | 1.00 | 61 | 6967 | export at tests/types-catalog/utils.ts:20 |
| 59 | 1.00 | 59 | 3276 | export at src/types/FindSelected.ts:165 |
| 56 | 1.00 | 56 | 6886 | export at tests/types-catalog/utils.ts:14 |
| 55 | 1.00 | 55 | 2867 | export doc at src/types/InvertPattern.ts:106 |
| 55 | 1.00 | 55 | 6236 | imports in tests/type-is-matching.test.ts |
