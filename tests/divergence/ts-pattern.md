scores: Sim=0.385 Reached=14/40 Early=3 Late=7 Partial=4 Missing=22 Used=9961/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 4 | 1 | 1 | 0.78 |
| 2 | 7 | 5 | 1 | 1 | 0.86 |
| 3 | 8 | 1 | 1 | 6 | 0.29 |
| 4 | 10 | 0 | 1 | 9 | 0.22 |
| 5 | 9 | 4 | 0 | 5 | 0.53 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.2 | 120 | — | — | 0.67 | partial | Public entry point — src/index.ts (full) |
| 1.3 | 149 | 321 | +172 | 1.00 | late | src/ directory layout |
| 1.4 | 205 | 1839 | +1634 | 1.00 | late | src/types/ directory layout |
| 1.5 | 213 | 1145 | +932 | 1.00 | late | src/internals/ directory layout |
| 1.6 | 293 | — | — | 0.00 | missing | package.json — name/version/description/exports skeleton |
| 2.1 | 350 | — | — | 0.60 | partial | match() — signature only |
| 2.3 | 662 | 7486 | +6824 | 1.00 | late | match() — full JSDoc |
| 2.4 | 816 | — | — | 0.40 | missing | NonExhaustiveError class (full) |
| 2.5 | 943 | 8773 | +7830 | 1.00 | late | P.* namespace — locations of every exported pattern (functions) |
| 2.6 | 1061 | 8773 | +7712 | 1.00 | late | P.* namespace — wildcards (constants) |
| 2.7 | 1390 | 4593 | +3203 | 1.00 | late | isMatching() — JSDoc for both overloads |
| 3.1 | 1729 | — | — | 0.00 | missing | README — opening tagline + lede match() example |
| 3.2 | 2058 | 1783 | -275 | 0.90 | aligned | README — Features bullets |
| 3.3 | 2222 | — | — | 0.06 | missing | Match<i, o> interface — top-level decl + member name locations |
| 3.4 | 2573 | — | — | 0.47 | missing | src/internals/symbols.ts — full file |
| 3.5 | 3002 | — | — | 0.00 | missing | MatcherType + Matcher interface (types/Pattern.ts) |
| 3.6 | 3206 | — | — | 0.00 | missing | Pattern<a> generic type — definition + JSDoc |
| 3.7 | 3432 | — | — | 0.67 | partial | P.* type re-exports header (patterns.ts module doc + Pattern alias) |
| 3.8 | 3818 | — | — | 0.21 | missing | P.infer + P.narrow — JSDoc + type aliases |
| 4.1 | 4302 | — | — | 0.13 | missing | MatchExpression class — JSDoc + member signatures |
| 4.2 | 4777 | — | — | 0.00 | missing | MatchExpression.with body — runtime semantics |
| 4.3 | 5613 | — | — | 0.01 | missing | matchPattern engine — entry + tuple-matching branch |
| 4.4 | 6160 | — | — | 0.46 | missing | matchPattern engine — object branch + helpers (isObject, isMatcher, getSelectionKeys, flatMap) |
| 4.5 | 6589 | — | — | 0.69 | partial | P.select — JSDoc + overload signatures |
| 4.6 | 7232 | — | — | 0.15 | missing | P.union + P.not + P.intersection — JSDoc + signatures |
| 4.7 | 7761 | — | — | 0.49 | missing | P.when + P.optional — JSDoc + signatures |
| 4.8 | 8173 | — | — | 0.29 | missing | P.array + P.instanceOf — JSDoc + signatures |
| 4.9 | 8293 | — | — | 0.00 | missing | P.string predicates — chainable methods (StringChainable shape) |
| 4.10 | 8427 | — | — | 0.00 | missing | P.number / P.bigint chainable predicates — interface methods |
| 5.1 | 8478 | — | — | 0.33 | missing | DeepExclude.ts (full) — exhaustiveness engine |
| 5.2 | 8959 | — | — | 0.11 | missing | DistributeMatchingUnions — JSDoc with worked t1/t2 examples |
| 5.3 | 9237 | — | — | 0.06 | missing | ExtractPreciseValue<a, b> — primary type definition |
| 5.4 | 9372 | 3135 | -6237 | 1.00 | early | InvertPattern + FindSelected + IsMatching + BuildMany — exported type locations |
| 5.5 | 9423 | 1137 | -8286 | 1.00 | early | docs/ + examples/ directory listings |
| 5.6 | 9508 | — | — | 0.12 | missing | v4-to-v5 migration guide — H1/H2 section locations |
| 5.7 | 9622 | 6619 | -3003 | 1.00 | early | tests/types-catalog/utils.ts — exported types and ctors (locations) |
| 5.9 | 9953 | — | — | 0.10 | missing | matcher-protocol.test.ts — Some class with custom [P.matcher]() |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 337 | 0.96 | 350 | 4375 | export at src/types/BuildMany.ts:19 |
| 298 | 1.00 | 298 | 4966 | listing of 'tests' |
| 277 | 0.42 | 652 | 8773 | export names surface in src/patterns.ts |
| 223 | 0.96 | 232 | 7199 | export at tests/types-catalog/utils.ts:26 |
| 217 | 1.00 | 217 | 1097 | README.md section #2 |
| 131 | 0.90 | 146 | 3559 | export at src/types/FindSelected.ts:174 |
| 129 | 1.00 | 129 | 2812 | export at src/types/DistributeUnions.ts:183 |
| 128 | 1.00 | 128 | 766 | README.md section #3 |
| 128 | 1.00 | 128 | 2683 | export at src/types/DistributeUnions.ts:46 |
| 117 | 1.00 | 117 | 638 | README.md section #4 |
| 107 | 1.00 | 107 | 5091 | export at tests/types-catalog/definition.ts:585 |
| 104 | 1.00 | 104 | 215 | README.md section #0 |
| 104 | 0.72 | 144 | 3135 | export names surface in src/types/FindSelected.ts |
| 101 | 0.48 | 212 | 6619 | export names surface in tests/types-catalog/utils.ts |
| 95 | 1.00 | 95 | 8024 | imports in tests/generics.test.ts |
| 94 | 1.00 | 94 | 9419 | export at src/patterns.ts:437 |
| 94 | 1.00 | 94 | 7929 | imports in tests/extract-precise-value.test.ts |
| 93 | 1.00 | 93 | 2555 | export at src/types/DistributeUnions.ts:174 |
| 90 | 0.91 | 99 | 2245 | export at src/types/InvertPattern.ts:180 |
| 90 | 0.91 | 99 | 2344 | export at src/types/InvertPattern.ts:192 |
| 90 | 1.00 | 90 | 7835 | imports in tests/distribute-unions.test.ts |
| 87 | 0.90 | 97 | 8121 | imports in tests/exhaustive-match.test.ts |
| 87 | 1.00 | 87 | 7657 | imports in tests/helpers.test.ts |
| 78 | 1.00 | 78 | 9231 | export at src/patterns.ts:357 |
| 78 | 1.00 | 78 | 7277 | imports in tests/invert-pattern.test.ts |
| 75 | 0.86 | 88 | 7745 | imports in tests/select.test.ts |
| 72 | 0.80 | 90 | 2434 | export names surface in src/types/DistributeUnions.ts |
| 67 | 0.80 | 84 | 7570 | imports in tests/wildcards.test.ts |
| 63 | 1.00 | 63 | 111 | README headline in README.md |
| 62 | 1.00 | 62 | 4025 | imports in src/is-matching.ts |
| 55 | 0.80 | 69 | 3413 | export at src/types/FindSelected.ts:159 |
| 55 | 1.00 | 55 | 2867 | export doc at src/types/InvertPattern.ts:106 |
| 55 | 1.00 | 55 | 6236 | imports in tests/type-is-matching.test.ts |
| 54 | 0.80 | 68 | 3344 | export at src/types/FindSelected.ts:191 |
