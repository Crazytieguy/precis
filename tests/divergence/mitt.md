scores: Sim=0.507 Reached=13/43 Early=4 Late=6 Partial=8 Missing=22 Used=3997/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 1 | 0 | 0.92 |
| 2 | 7 | 4 | 3 | 0 | 0.85 |
| 3 | 8 | 0 | 1 | 7 | 0.09 |
| 4 | 10 | 2 | 0 | 8 | 0.22 |
| 5 | 12 | 2 | 3 | 7 | 0.31 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 57 | — | — | 0.67 | partial | README title + tagline | README headline in README.md (t=370, 2 atoms) |
| 1.3 | 104 | 202 | +98 | 1.00 | late | package.json name + version + description | package identity in package.json (t=202, 3 atoms) |
| 1.4 | 120 | 3928 | +3808 | 1.00 | late | src/ and test/ listings |  |
| 1.5 | 212 | 582 | +370 | 1.00 | late | package.json entrypoint + source fields | package entrypoints in package.json (t=582, 6 atoms) |
| 1.6 | 362 | 370 | +8 | 0.86 | aligned | README feature bullets | README headline in README.md (t=370, 6 atoms) |
| 2.1 | 470 | 756 | +286 | 1.00 | late | src/index.ts public exports — name-only locations | export at src/index.ts:23 (t=1009, 14 atoms) |
| 2.2 | 667 | 1009 | +342 | 0.82 | late | Emitter<Events> interface — full | export at src/index.ts:23 (t=1009, 14 atoms) |
| 2.3 | 889 | — | — | 0.73 | partial | README quickstart code example | README.md section #3 (t=2790, 16 atoms) |
| 2.5 | 989 | — | — | 0.75 | partial | Handler / WildcardHandler type aliases | export names surface in src/index.ts (t=756, 5 atoms) |
| 2.6 | 1096 | — | — | 0.73 | partial | EventHandlerMap type | export names surface in src/index.ts (t=756, 5 atoms) |
| 2.7 | 1302 | 3384 | +2082 | 0.90 | late | README API one-line method descriptions | README.md section #5 (t=3384, 20 atoms) |
| 3.1 | 1401 | — | — | 0.00 | missing | mitt() body — Map default + return-shape skeleton | export at src/index.ts:46 (t=1032, 1 atoms) |
| 3.2 | 1606 | — | — | 0.00 | missing | emit() body — the only non-trivial method |  |
| 3.3 | 1717 | — | — | 0.00 | missing | on() body |  |
| 3.4 | 1844 | — | — | 0.00 | missing | off() body |  |
| 3.5 | 1981 | — | — | 0.00 | missing | emit() JSDoc |  |
| 3.6 | 2294 | — | — | 0.71 | partial | README TypeScript usage section | README.md section #3 (t=2790, 21 atoms) |
| 3.7 | 2626 | — | — | 0.00 | missing | test/index_test.ts test labels — all describe + it titles |  |
| 3.8 | 2829 | — | — | 0.00 | missing | on() / off() JSDoc |  |
| 4.1 | 2949 | — | — | 0.00 | missing | compressed-size CI workflow |  |
| 4.2 | 3089 | — | — | 0.00 | missing | package.json mocha + prettier blocks |  |
| 4.3 | 3238 | 1229 | -2009 | 1.00 | early | tsconfig.json — full | json config tsconfig.json (t=1229, 15 atoms) |
| 4.4 | 3455 | — | — | 0.00 | missing | CI workflow (main.yml) |  |
| 4.5 | 3690 | 1821 | -1869 | 1.00 | early | package.json scripts | package scripts in package.json (t=1821, 12 atoms) |
| 4.6 | 3890 | — | — | 0.05 | missing | test-types-compilation.ts preamble — Events type + handler decls | imports in test/test-types-compilation.ts (t=3997, 1 atoms) |
| 4.7 | 4287 | — | — | 0.00 | missing | test-types-compilation.ts on()/off() blocks |  |
| 4.8 | 4485 | — | — | 0.00 | missing | test-types-compilation.ts emit() block |  |
| 4.9 | 4724 | — | — | 0.17 | missing | test/index_test.ts imports + outer-block tests | imports in test/index_test.ts (t=3988, 4 atoms) |
| 4.10 | 4955 | — | — | 0.00 | missing | test/index_test.ts mitt# Events type + beforeEach |  |
| 5.1 | 5244 | — | — | 0.72 | partial | README install section | README.md section #2 (t=2072, 17 atoms) |
| 5.2 | 5401 | — | — | 0.00 | missing | Test body: wildcard '*' invocation |  |
| 5.3 | 5926 | — | — | 0.00 | missing | Test bodies: on() registration semantics |  |
| 5.4 | 6411 | — | — | 0.00 | missing | Test bodies: off() removal semantics |  |
| 5.5 | 6664 | — | — | 0.00 | missing | Test bodies: emit() typed dispatch + case sensitivity |  |
| 5.6 | 7020 | — | — | 0.60 | partial | README API parameter tables | README.md section #5 (t=3384, 16 atoms) |
| 5.7 | 7520 | — | — | 0.00 | missing | .eslintrc — full |  |
| 5.8 | 8020 | 2343 | -5677 | 0.84 | early | README Examples / Contribute / License sections | README.md section #5 (t=3384, 34 atoms) |
| 5.9 | 8386 | 3811 | -4575 | 1.00 | early | package.json devDependencies | package dependencies in package.json (t=3811, 23 atoms) |
| 5.10 | 8625 | — | — | 0.00 | missing | .editorconfig + .gitignore |  |
| 5.11 | 8837 | — | — | 0.58 | partial | .github/PULL_REQUEST_TEMPLATE.md | .github/PULL_REQUEST_TEMPLATE.md section #0 (t=3916, 8 atoms) |
| 5.12 | 8863 | — | — | 0.00 | missing | LICENSE — MIT preamble |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 272 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 192 | 0.32 | 594 | 3384 | README.md section #5 |
| 138 | 0.80 | 173 | 202 | package identity in package.json |
| 137 | 0.65 | 212 | 582 | package entrypoints in package.json |
| 80 | 1.00 | 80 | 1455 | README.md section #1 |
