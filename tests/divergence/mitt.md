scores: Sim=0.475 Reached=6/43 Early=1 Late=1 Partial=10 Missing=27 Used=2832/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 3 | 1 | 2 | 0.59 |
| 2 | 7 | 3 | 4 | 0 | 0.81 |
| 3 | 8 | 0 | 1 | 7 | 0.09 |
| 4 | 10 | 0 | 0 | 10 | 0.02 |
| 5 | 12 | 0 | 4 | 8 | 0.21 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.2 | 57 | — | — | 0.67 | partial | README title + tagline |
| 1.3 | 104 | — | — | 0.00 | missing | package.json name + version + description |
| 1.4 | 120 | 636 | +516 | 1.00 | late | src/ and test/ listings |
| 1.5 | 212 | — | — | 0.00 | missing | package.json entrypoint + source fields |
| 1.6 | 362 | 197 | -165 | 0.86 | early | README feature bullets |
| 2.1 | 470 | 371 | -99 | 1.00 | aligned+over | src/index.ts public exports — name-only locations |
| 2.2 | 667 | 624 | -43 | 0.82 | aligned | Emitter<Events> interface — full |
| 2.3 | 889 | — | — | 0.73 | partial | README quickstart code example |
| 2.5 | 989 | — | — | 0.75 | partial | Handler / WildcardHandler type aliases |
| 2.6 | 1096 | — | — | 0.73 | partial | EventHandlerMap type |
| 2.7 | 1302 | — | — | 0.67 | partial | README API one-line method descriptions |
| 3.1 | 1401 | — | — | 0.00 | missing | mitt() body — Map default + return-shape skeleton |
| 3.2 | 1606 | — | — | 0.00 | missing | emit() body — the only non-trivial method |
| 3.3 | 1717 | — | — | 0.00 | missing | on() body |
| 3.4 | 1844 | — | — | 0.00 | missing | off() body |
| 3.5 | 1981 | — | — | 0.00 | missing | emit() JSDoc |
| 3.6 | 2294 | — | — | 0.68 | partial | README TypeScript usage section |
| 3.7 | 2626 | — | — | 0.00 | missing | test/index_test.ts test labels — all describe + it titles |
| 3.8 | 2829 | — | — | 0.00 | missing | on() / off() JSDoc |
| 4.1 | 2949 | — | — | 0.00 | missing | compressed-size CI workflow |
| 4.2 | 3089 | — | — | 0.00 | missing | package.json mocha + prettier blocks |
| 4.3 | 3238 | — | — | 0.00 | missing | tsconfig.json — full |
| 4.4 | 3455 | — | — | 0.00 | missing | CI workflow (main.yml) |
| 4.5 | 3690 | — | — | 0.00 | missing | package.json scripts |
| 4.6 | 3890 | — | — | 0.05 | missing | test-types-compilation.ts preamble — Events type + handler decls |
| 4.7 | 4287 | — | — | 0.00 | missing | test-types-compilation.ts on()/off() blocks |
| 4.8 | 4485 | — | — | 0.00 | missing | test-types-compilation.ts emit() block |
| 4.9 | 4724 | — | — | 0.17 | missing | test/index_test.ts imports + outer-block tests |
| 4.10 | 4955 | — | — | 0.00 | missing | test/index_test.ts mitt# Events type + beforeEach |
| 5.1 | 5244 | — | — | 0.68 | partial | README install section |
| 5.2 | 5401 | — | — | 0.00 | missing | Test body: wildcard '*' invocation |
| 5.3 | 5926 | — | — | 0.00 | missing | Test bodies: on() registration semantics |
| 5.4 | 6411 | — | — | 0.00 | missing | Test bodies: off() removal semantics |
| 5.5 | 6664 | — | — | 0.00 | missing | Test bodies: emit() typed dispatch + case sensitivity |
| 5.6 | 7020 | — | — | 0.60 | partial | README API parameter tables |
| 5.7 | 7520 | — | — | 0.00 | missing | .eslintrc — full |
| 5.8 | 8020 | — | — | 0.70 | partial | README Examples / Contribute / License sections |
| 5.9 | 8386 | — | — | 0.00 | missing | package.json devDependencies |
| 5.10 | 8625 | — | — | 0.00 | missing | .editorconfig + .gitignore |
| 5.11 | 8837 | — | — | 0.58 | partial | .github/PULL_REQUEST_TEMPLATE.md |
| 5.12 | 8863 | — | — | 0.00 | missing | LICENSE — MIT preamble |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 292 | 0.16 | 1881 | 2832 | README.md section #0 |
