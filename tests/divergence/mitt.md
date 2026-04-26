scores: Sim=0.542 Reached=19/43 Early=5 Late=9 Partial=8 Missing=16 Used=4868/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 1 | 0 | 0.92 |
| 2 | 7 | 4 | 3 | 0 | 0.85 |
| 3 | 8 | 6 | 1 | 1 | 0.81 |
| 4 | 10 | 2 | 0 | 8 | 0.22 |
| 5 | 12 | 2 | 3 | 7 | 0.31 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 57 | — | — | 0.67 | partial | README title + tagline | README headline in README.md (t=393, 2 atoms) |
| 1.3 | 104 | 225 | +121 | 1.00 | late | package.json name + version + description | package identity in package.json (t=225, 3 atoms) |
| 1.4 | 120 | 4615 | +4495 | 1.00 | late | src/ and test/ listings |  |
| 1.5 | 212 | 1089 | +877 | 1.00 | late | package.json entrypoint + source fields | package entrypoints in package.json (t=1089, 6 atoms) |
| 1.6 | 362 | 393 | +31 | 0.86 | aligned | README feature bullets | README headline in README.md (t=393, 6 atoms) |
| 2.1 | 470 | 553 | +83 | 1.00 | aligned+over | src/index.ts public exports — name-only locations | export at src/index.ts:23 (t=829, 14 atoms) |
| 2.2 | 667 | 829 | +162 | 0.82 | aligned | Emitter<Events> interface — full | export at src/index.ts:23 (t=829, 14 atoms) |
| 2.3 | 889 | — | — | 0.73 | partial | README quickstart code example | README.md section #3 (t=3659, 16 atoms) |
| 2.4 | 912 | 588 | -324 | 1.00 | early | mitt() default-export signature | export at src/index.ts:46 (t=588, 3 atoms) |
| 2.5 | 989 | — | — | 0.75 | partial | Handler / WildcardHandler type aliases | export names surface in src/index.ts (t=553, 5 atoms) |
| 2.6 | 1096 | — | — | 0.73 | partial | EventHandlerMap type | export names surface in src/index.ts (t=553, 5 atoms) |
| 2.7 | 1302 | 4457 | +3155 | 0.90 | late | README API one-line method descriptions | headings outline in README.md (t=1266, 10 atoms) |
| 3.1 | 1401 | 3185 | +1784 | 0.82 | late | mitt() body — Map default + return-shape skeleton | export body at src/index.ts:46 (t=3185, 9 atoms) |
| 3.2 | 1606 | 3185 | +1579 | 0.95 | late | emit() body — the only non-trivial method | export body at src/index.ts:46 (t=3185, 18 atoms) |
| 3.3 | 1717 | 3185 | +1468 | 1.00 | late | on() body | export body at src/index.ts:46 (t=3185, 8 atoms) |
| 3.4 | 1844 | 3185 | +1341 | 1.00 | late | off() body | export body at src/index.ts:46 (t=3185, 10 atoms) |
| 3.5 | 1981 | 3185 | +1204 | 1.00 | late | emit() JSDoc | export body at src/index.ts:46 (t=3185, 10 atoms) |
| 3.6 | 2294 | — | — | 0.71 | partial | README TypeScript usage section | README.md section #3 (t=3659, 21 atoms) |
| 3.7 | 2626 | — | — | 0.00 | missing | test/index_test.ts test labels — all describe + it titles |  |
| 4.1 | 2949 | — | — | 0.00 | missing | compressed-size CI workflow |  |
| 4.2 | 3089 | — | — | 0.00 | missing | package.json mocha + prettier blocks |  |
| 4.3 | 3238 | 1558 | -1680 | 1.00 | early | tsconfig.json — full | json config tsconfig.json (t=1558, 15 atoms) |
| 4.4 | 3455 | — | — | 0.00 | missing | CI workflow (main.yml) |  |
| 4.5 | 3690 | 1896 | -1794 | 1.00 | early | package.json scripts | package scripts in package.json (t=1896, 12 atoms) |
| 4.6 | 3890 | — | — | 0.05 | missing | test-types-compilation.ts preamble — Events type + handler decls | imports in test/test-types-compilation.ts (t=4808, 1 atoms) |
| 4.7 | 4287 | — | — | 0.00 | missing | test-types-compilation.ts on()/off() blocks |  |
| 4.8 | 4485 | — | — | 0.00 | missing | test-types-compilation.ts emit() block |  |
| 4.9 | 4724 | — | — | 0.17 | missing | test/index_test.ts imports + outer-block tests | imports in test/index_test.ts (t=4868, 4 atoms) |
| 4.10 | 4955 | — | — | 0.00 | missing | test/index_test.ts mitt# Events type + beforeEach |  |
| 5.1 | 5244 | — | — | 0.72 | partial | README install section | README.md section #2 (t=2314, 17 atoms) |
| 5.2 | 5401 | — | — | 0.00 | missing | Test body: wildcard '*' invocation |  |
| 5.3 | 5926 | — | — | 0.00 | missing | Test bodies: on() registration semantics |  |
| 5.4 | 6411 | — | — | 0.00 | missing | Test bodies: off() removal semantics |  |
| 5.5 | 6664 | — | — | 0.00 | missing | Test bodies: emit() typed dispatch + case sensitivity |  |
| 5.6 | 7020 | — | — | 0.60 | partial | README API parameter tables | README.md section #10 (t=4457, 7 atoms) |
| 5.7 | 7520 | — | — | 0.00 | missing | .eslintrc — full |  |
| 5.8 | 8020 | 4799 | -3221 | 0.84 | early | README Examples / Contribute / License sections | headings outline in README.md (t=1266, 22 atoms) |
| 5.9 | 8386 | 4025 | -4361 | 1.00 | early | package.json devDependencies | package dependencies in package.json (t=4025, 23 atoms) |
| 5.10 | 8625 | — | — | 0.00 | missing | .editorconfig + .gitignore |  |
| 5.11 | 8837 | — | — | 0.58 | partial | .github/PULL_REQUEST_TEMPLATE.md | .github/PULL_REQUEST_TEMPLATE.md section #0 (t=4130, 8 atoms) |
| 5.12 | 8863 | — | — | 0.00 | missing | LICENSE — MIT preamble |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 212 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 132 | 1.00 | 132 | 2028 | README.md section #5 |
| 126 | 0.73 | 173 | 225 | package identity in package.json |
| 120 | 0.57 | 212 | 1089 | package entrypoints in package.json |
| 80 | 1.00 | 80 | 1409 | README.md section #1 |
