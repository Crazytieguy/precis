scores: Sim=0.562 Reached=21/43 Early=6 Late=7 Partial=9 Missing=13 Used=5881/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 5 | 1 | 0 | 0.92 |
| 2 | 7 | 4 | 3 | 0 | 0.85 |
| 3 | 8 | 6 | 1 | 1 | 0.81 |
| 4 | 10 | 2 | 0 | 8 | 0.22 |
| 5 | 12 | 4 | 4 | 4 | 0.52 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 57 | — | — | 0.67 | partial | README title + tagline | README headline in README.md (t=393, 2 atoms) |
| 1.3 | 104 | 225 | +121 | 1.00 | late | package.json name + version + description | package identity in package.json (t=225, 3 atoms) |
| 1.4 | 120 | 3919 | +3799 | 1.00 | late | src/ and test/ listings |  |
| 1.5 | 212 | 1409 | +1197 | 1.00 | late | package.json entrypoint + source fields | package entrypoints in package.json (t=1409, 6 atoms) |
| 1.6 | 362 | 393 | +31 | 0.86 | aligned | README feature bullets | README headline in README.md (t=393, 6 atoms) |
| 2.1 | 470 | 553 | +83 | 1.00 | aligned+over | src/index.ts public exports — name-only locations | export at src/index.ts:23 (t=829, 14 atoms) |
| 2.2 | 667 | 829 | +162 | 0.82 | aligned | Emitter<Events> interface — full | export at src/index.ts:23 (t=829, 14 atoms) |
| 2.3 | 889 | — | — | 0.73 | partial | README quickstart code example | README.md section #3 (t=4418, 16 atoms) |
| 2.4 | 912 | 588 | -324 | 1.00 | early | mitt() default-export signature | export at src/index.ts:46 (t=588, 3 atoms) |
| 2.5 | 989 | — | — | 0.75 | partial | Handler / WildcardHandler type aliases | export names surface in src/index.ts (t=553, 5 atoms) |
| 2.6 | 1096 | — | — | 0.73 | partial | EventHandlerMap type | export names surface in src/index.ts (t=553, 5 atoms) |
| 2.7 | 1302 | 5202 | +3900 | 0.90 | late | README API one-line method descriptions | headings outline in README.md (t=1023, 10 atoms) |
| 3.1 | 1401 | 2280 | +879 | 0.82 | late | mitt() body — Map default + return-shape skeleton | export body at src/index.ts:46 (t=2280, 9 atoms) |
| 3.2 | 1606 | 2280 | +674 | 0.95 | late | emit() body — the only non-trivial method | export body at src/index.ts:46 (t=2280, 18 atoms) |
| 3.3 | 1717 | 2280 | +563 | 1.00 | late | on() body | export body at src/index.ts:46 (t=2280, 8 atoms) |
| 3.6 | 2294 | — | — | 0.71 | partial | README TypeScript usage section | README.md section #3 (t=4418, 21 atoms) |
| 3.7 | 2626 | — | — | 0.00 | missing | test/index_test.ts test labels — all describe + it titles |  |
| 4.1 | 2949 | — | — | 0.00 | missing | compressed-size CI workflow |  |
| 4.2 | 3089 | — | — | 0.00 | missing | package.json mocha + prettier blocks |  |
| 4.4 | 3455 | — | — | 0.00 | missing | CI workflow (main.yml) |  |
| 4.5 | 3690 | 2515 | -1175 | 1.00 | early | package.json scripts | package scripts in package.json (t=2515, 12 atoms) |
| 4.6 | 3890 | — | — | 0.05 | missing | test-types-compilation.ts preamble — Events type + handler decls | imports in test/test-types-compilation.ts (t=4927, 1 atoms) |
| 4.7 | 4287 | — | — | 0.00 | missing | test-types-compilation.ts on()/off() blocks |  |
| 4.8 | 4485 | — | — | 0.00 | missing | test-types-compilation.ts emit() block |  |
| 4.9 | 4724 | — | — | 0.17 | missing | test/index_test.ts imports + outer-block tests | imports in test/index_test.ts (t=5592, 4 atoms) |
| 4.10 | 4955 | — | — | 0.00 | missing | test/index_test.ts mitt# Events type + beforeEach |  |
| 5.1 | 5244 | — | — | 0.72 | partial | README install section | README.md section #2 (t=3907, 17 atoms) |
| 5.2 | 5401 | — | — | 0.00 | missing | Test body: wildcard '*' invocation |  |
| 5.3 | 5926 | — | — | 0.00 | missing | Test bodies: on() registration semantics |  |
| 5.4 | 6411 | — | — | 0.00 | missing | Test bodies: off() removal semantics |  |
| 5.5 | 6664 | — | — | 0.00 | missing | Test bodies: emit() typed dispatch + case sensitivity |  |
| 5.6 | 7020 | — | — | 0.60 | partial | README API parameter tables | README.md section #10 (t=5202, 7 atoms) |
| 5.7 | 7520 | 4918 | -2602 | 1.00 | early | .eslintrc — full | plaintext config .eslintrc (t=4918, 52 atoms) |
| 5.8 | 8020 | 5532 | -2488 | 0.84 | early | README Examples / Contribute / License sections | headings outline in README.md (t=1023, 22 atoms) |
| 5.9 | 8386 | 3177 | -5209 | 1.00 | early | package.json devDependencies | package dependencies in package.json (t=3177, 23 atoms) |
| 5.10 | 8625 | 3665 | -4960 | 0.89 | early | .editorconfig + .gitignore | plaintext config .editorconfig (t=3665, 15 atoms) |
| 5.11 | 8837 | — | — | 0.58 | partial | .github/PULL_REQUEST_TEMPLATE.md | .github/PULL_REQUEST_TEMPLATE.md section #0 (t=3282, 8 atoms) |
| 5.12 | 8863 | — | — | 0.67 | partial | LICENSE — MIT preamble | plaintext config LICENSE (t=5881, 2 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 212 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 268 | 0.93 | 289 | 5881 | plaintext config LICENSE |
| 132 | 1.00 | 132 | 3414 | README.md section #5 |
| 126 | 0.73 | 173 | 225 | package identity in package.json |
| 120 | 0.57 | 212 | 1409 | package entrypoints in package.json |
| 80 | 1.00 | 80 | 1166 | README.md section #1 |
