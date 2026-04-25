scores: Sim=0.442 Reached=6/39 Early=0 Late=5 Partial=11 Missing=22 Used=2832/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 5 | 3 | 1 | 1 | 0.73 |
| 2 | 6 | 3 | 3 | 0 | 0.84 |
| 3 | 4 | 0 | 0 | 4 | 0.00 |
| 4 | 8 | 0 | 2 | 6 | 0.20 |
| 5 | 16 | 0 | 5 | 11 | 0.20 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor |
|----|------:|----------:|--------:|-------:|:-------|:-----------|
| 1.2 | 57 | — | — | 0.67 | partial | README title + tagline |
| 1.3 | 73 | 636 | +563 | 1.00 | late | src/ and test/ listings |
| 1.4 | 120 | — | — | 0.00 | missing | package.json identity (name, version, description) |
| 1.5 | 166 | 2832 | +2666 | 1.00 | late | README H2 section locations |
| 2.1 | 243 | 371 | +128 | 1.00 | late | src/index.ts exported names (locations only) |
| 2.2 | 269 | — | — | 0.67 | partial | Emitter<Events> interface — header + `all` field |
| 2.3 | 449 | 624 | +175 | 0.86 | late | Emitter.on / Emitter.off / Emitter.emit overload signatures |
| 2.4 | 529 | 707 | +178 | 1.00 | late | mitt() factory signature + jsdoc |
| 2.5 | 613 | — | — | 0.78 | partial | EventType, Handler, WildcardHandler aliases |
| 2.6 | 731 | — | — | 0.73 | partial | EventHandlerList, WildCardEventHandlerList, EventHandlerMap aliases |
| 3.1 | 825 | — | — | 0.00 | missing | mitt() factory body — opening + return shell |
| 3.2 | 936 | — | — | 0.00 | missing | on() method body |
| 3.3 | 1063 | — | — | 0.00 | missing | off() method body |
| 3.4 | 1268 | — | — | 0.00 | missing | emit() method body |
| 4.1 | 1495 | — | — | 0.71 | partial | README Usage example |
| 4.2 | 1810 | — | — | 0.00 | missing | test/index_test.ts — describe/it skeleton |
| 4.3 | 2073 | — | — | 0.17 | missing | test/index_test.ts — top-of-file imports + Events fixture |
| 4.4 | 2386 | — | — | 0.68 | partial | README Typescript section — Events generic example |
| 4.5 | 2591 | — | — | 0.05 | missing | test/test-types-compilation.ts — overview |
| 4.6 | 2789 | — | — | 0.00 | missing | test-types-compilation.ts — emit overload assertions |
| 4.7 | 2947 | — | — | 0.00 | missing | test/index_test.ts — wildcard + emit assertions body |
| 4.8 | 3175 | — | — | 0.00 | missing | test/index_test.ts — off() removal-semantics body |
| 5.1 | 3410 | — | — | 0.00 | missing | package.json scripts |
| 5.2 | 3622 | — | — | 0.00 | missing | package.json entry-points + files + exports |
| 5.3 | 3762 | — | — | 0.00 | missing | package.json mocha + prettier config |
| 5.4 | 3911 | — | — | 0.00 | missing | tsconfig.json |
| 5.5 | 4277 | — | — | 0.00 | missing | package.json devDependencies |
| 5.6 | 4549 | — | — | 0.00 | missing | .eslintrc rules |
| 5.7 | 4772 | — | — | 0.00 | missing | .eslintrc — extends + parser + env |
| 5.8 | 4989 | — | — | 0.00 | missing | .github/workflows/main.yml — CI pipeline |
| 5.9 | 5109 | — | — | 0.00 | missing | .github/workflows/compressed-size.yml — bundle-size guard |
| 5.10 | 5348 | — | — | 0.00 | missing | .gitignore + .editorconfig |
| 5.11 | 5635 | — | — | 0.65 | partial | README Install + import examples |
| 5.12 | 6024 | — | — | 0.62 | partial | README API reference — mitt / all / on (auto-generated prose) |
| 5.13 | 6374 | — | — | 0.59 | partial | README API reference — off / emit (auto-generated prose) |
| 5.14 | 6731 | — | — | 0.70 | partial | README Contribute + License |
| 5.15 | 6943 | — | — | 0.58 | partial | .github/PULL_REQUEST_TEMPLATE.md |
| 5.16 | 7252 | — | — | 0.00 | missing | LICENSE — full MIT text |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 200 | 0.11 | 1881 | 2832 | README.md section #0 |
| 126 | 0.75 | 168 | 197 | README headline in README.md |
