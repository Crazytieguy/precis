scores: Sim=0.263 Reached=7/41 Early=2 Late=5 Partial=5 Missing=29 Used=9879/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 2 | 4 | 1 | 0.63 |
| 2 | 12 | 3 | 0 | 9 | 0.28 |
| 3 | 15 | 1 | 1 | 13 | 0.12 |
| 4 | 4 | 0 | 0 | 4 | 0.18 |
| 5 | 3 | 1 | 0 | 2 | 0.33 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 77 | — | — | 0.67 | partial | README one-liner | README headline in README.md (t=182, 2 atoms) |
| 1.2 | 131 | 54 | -77 | 1.00 | early | Top-level workspace listing |  |
| 1.3 | 153 | 257 | +104 | 1.00 | late | Packages directory listing |  |
| 1.4 | 179 | — | — | 0.00 | missing | pnpm workspace globs |  |
| 1.5 | 313 | — | — | 0.67 | partial | README — incremental + Electric pitch | README.md section #0 (t=1197, 2 atoms) |
| 1.6 | 419 | — | — | 0.50 | partial | d2mini one-liner | README headline in packages/d2mini/README.md (t=827, 2 atoms) |
| 1.7 | 494 | — | — | 0.60 | partial | d2ql one-liner | README headline in packages/d2ql/README.md (t=676, 2 atoms) |
| 2.1 | 538 | 3565 | +3027 | 1.00 | late | d2ts src layout |  |
| 2.2 | 608 | 4663 | +4055 | 1.00 | late | d2ts package re-exports (`index.ts`) | imports in packages/d2ts/src/index.ts (t=4663, 6 atoms) |
| 2.3 | 706 | — | — | 0.00 | missing | Operator catalog (operators/ filenames) |  |
| 2.4 | 744 | 4716 | +3972 | 1.00 | late | Version + Antichain class names + factory `v(…)` | export names surface in packages/d2ts/src/order.ts (t=4716, 7 atoms) |
| 2.5 | 919 | — | — | 0.00 | missing | Core types: KeyValue, MessageType, Message |  |
| 2.6 | 1019 | — | — | 0.00 | missing | Core types: DataMessage, FrontierMessage, PipedOperator |  |
| 2.7 | 1259 | — | — | 0.00 | missing | operators/index.ts (operator re-exports) |  |
| 2.8 | 1382 | — | — | 0.06 | missing | MultiSet method names (full catalog) | export names surface in packages/d2ts/src/multiset.ts (t=4788, 2 atoms) |
| 2.9 | 1537 | — | — | 0.29 | missing | D2 class + RootStreamBuilder method names | export names surface in packages/d2ts/src/d2.ts (t=4853, 8 atoms) |
| 2.10 | 1760 | — | — | 0.00 | missing | Operator interfaces: IOperator, IDifferenceStreamReader/Writer |  |
| 2.11 | 2056 | — | — | 0.00 | missing | ID2 + IStreamBuilder shape |  |
| 2.12 | 2436 | — | — | 0.00 | missing | Top-level README — operator catalog with descriptions |  |
| 3.1 | 2667 | — | — | 0.00 | missing | Antichain.create polymorphic constructor |  |
| 3.2 | 2908 | — | — | 0.73 | partial | graph.ts class hierarchy (signatures only) | export at packages/d2ts/src/graph.ts:111 (t=8796, 17 atoms) |
| 3.3 | 3322 | — | — | 0.00 | missing | LinearUnaryOperator base class (operators/base.ts) |  |
| 3.4 | 3747 | — | — | 0.00 | missing | Keying operators (keyBy/unkey/rekey) |  |
| 3.5 | 3988 | — | — | 0.00 | missing | map() operator: canonical factory pattern |  |
| 3.6 | 4482 | — | — | 0.00 | missing | ConsolidateOperator: the "versions complete" pattern |  |
| 3.7 | 4670 | 7021 | +2351 | 1.00 | late | Index<K,V> trace: signatures + interface | export at packages/d2ts/src/version-index.ts:8 (t=7021, 10 atoms) |
| 3.8 | 5362 | — | — | 0.00 | missing | ReduceOperator: class + ingest loop |  |
| 3.9 | 6080 | — | — | 0.00 | missing | ReduceOperator: finished-versions + delta computation |  |
| 3.10 | 6843 | — | — | 0.00 | missing | JoinOperator: class shape + delta-join kernel |  |
| 3.11 | 7254 | — | — | 0.00 | missing | Join factory + JoinType + named variants |  |
| 3.12 | 7601 | — | — | 0.00 | missing | iterate(): scope/feedback wiring |  |
| 3.13 | 7651 | — | — | 0.00 | missing | iterate.ts class names |  |
| 3.14 | 8104 | — | — | 0.00 | missing | groupBy + aggregate function names |  |
| 3.15 | 8641 | — | — | 0.00 | missing | topK + orderBy + indexed variants (signatures) |  |
| 4.1 | 8722 | — | — | 0.33 | missing | sqlite/ + electric/ subdir listings |  |
| 4.2 | 8859 | — | — | 0.28 | missing | d2mini src + operators listings |  |
| 4.3 | 8937 | — | — | 0.00 | missing | d2ql src + query-builder listings |  |
| 4.4 | 9068 | — | — | 0.10 | missing | Examples + benchmark listings |  |
| 5.1 | 9394 | — | — | 0.00 | missing | d2ql Query interface + compileQuery signature |  |
| 5.2 | 9687 | — | — | 0.00 | missing | d2ql function + aggregate + comparator names |  |
| 5.3 | 9955 | 4253 | -5702 | 1.00 | early | Electric adapter entry points | export body at packages/d2ts/src/electric/index.ts:222 (t=8437, 76 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 394 | export at packages/d2ts/src/electric/index.ts:<n> |
| 5 | 380 | export at packages/d2mini/src/graph.ts:<n> |
| 3 | 270 | export at packages/d2ts/src/graph.ts:<n> |
| 2 | 198 | export at packages/d2ts/src/sqlite/database.ts:<n> |
| 2 | 167 | export at packages/d2ts/src/utils.ts:<n> |
| 2 | 136 | export at packages/d2mini/src/utils.ts:<n> |
| 2 | 131 | export at packages/d2ts-benchmark/src/base.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 948 | 1.00 | 948 | 8437 | export body at packages/d2ts/src/electric/index.ts:222 |
| 416 | 1.00 | 416 | 5526 | json config tsconfig.json |
| 247 | 1.00 | 247 | 2889 | package identity in packages/d2ql/package.json |
| 244 | 1.00 | 244 | 2559 | package identity in packages/d2mini/package.json |
| 238 | 1.00 | 238 | 2140 | package identity in packages/d2ts/package.json |
| 216 | 1.00 | 216 | 6558 | package dependencies in package.json |
| 175 | 1.00 | 175 | 2315 | package entrypoints in packages/d2ts/package.json |
| 172 | 1.00 | 172 | 3061 | package entrypoints in packages/d2ql/package.json |
| 167 | 1.00 | 167 | 1033 | headings outline in README.md |
| 165 | 0.88 | 188 | 4486 | export body at packages/d2ts/src/electric/index.ts:328 |
| 160 | 1.00 | 160 | 4004 | export at packages/d2ts/src/electric/index.ts:222 |
| 136 | 1.00 | 136 | 603 | package scripts in package.json |
| 131 | 0.82 | 160 | 8796 | export at packages/d2ts/src/graph.ts:111 |
| 123 | 1.00 | 123 | 9079 | export names surface in packages/d2mini/src/graph.ts |
| 121 | 1.00 | 121 | 3844 | export at packages/d2ts/src/electric/index.ts:103 |
| 118 | 1.00 | 118 | 3409 | imports in packages/d2ts-benchmark/src/index.ts |
| 113 | 0.45 | 249 | 4253 | export at packages/d2ts/src/electric/index.ts:40 |
| 106 | 1.00 | 106 | 7303 | export names surface in packages/d2mini/src/types.ts |
| 102 | 1.00 | 102 | 8539 | export at packages/d2ts/src/sqlite/database.ts:49 |
| 99 | 1.00 | 99 | 9644 | export at packages/d2mini/src/graph.ts:56 |
| 97 | 1.00 | 97 | 8636 | export body at packages/d2ts/src/utils.ts:7 |
| 97 | 1.00 | 97 | 6687 | export names surface in packages/d2mini/src/utils.ts |
| 96 | 1.00 | 96 | 5896 | export at packages/d2ts/src/sqlite/database.ts:32 |
| 94 | 1.00 | 94 | 9506 | export at packages/d2mini/src/graph.ts:100 |
| 94 | 1.00 | 94 | 9798 | export body at packages/d2mini/src/graph.ts:32 |
| 92 | 1.00 | 92 | 5633 | export names surface in packages/d2ts/src/utils.ts |
| 89 | 1.00 | 89 | 3498 | headings outline in packages/d2ts-benchmark/CHANGELOG.md |
| 88 | 1.00 | 88 | 5800 | export at packages/d2ts/src/utils.ts:7 |
| 88 | 1.00 | 88 | 7109 | export body at packages/d2mini/src/utils.ts:6 |
| 88 | 1.00 | 88 | 7197 | export body at packages/d2ts/src/utils.ts:31 |
| 87 | 1.00 | 87 | 344 | README headline in .changeset/README.md |
| 87 | 1.00 | 87 | 1598 | package scripts in packages/d2ts-benchmark/package.json |
| 83 | 1.00 | 83 | 2642 | package entrypoints in packages/d2mini/package.json |
| 82 | 1.00 | 82 | 9326 | export at packages/d2mini/src/graph.ts:80 |
| 81 | 1.00 | 81 | 8877 | export body at packages/d2ts/src/graph.ts:111 |
| 79 | 1.00 | 79 | 7489 | export at packages/d2mini/src/types.ts:21 |
| 79 | 1.00 | 79 | 6867 | export at packages/d2mini/src/utils.ts:6 |
| 79 | 1.00 | 79 | 5712 | export at packages/d2ts/src/utils.ts:31 |
| 76 | 1.00 | 76 | 1674 | json config packages/d2mini/tsconfig.json |
| 76 | 1.00 | 76 | 1750 | json config packages/d2ql/tsconfig.json |
| 76 | 1.00 | 76 | 1902 | json config packages/d2ts-benchmark/tsconfig.json |
| 76 | 1.00 | 76 | 1826 | json config packages/d2ts/tsconfig.json |
| 74 | 1.00 | 74 | 1433 | export at packages/d2ts-benchmark/src/base.ts:4 |
| 71 | 0.61 | 116 | 6342 | export at packages/d2ts/src/graph.ts:54 |
| 71 | 1.00 | 71 | 4557 | export doc at packages/d2ts/src/electric/index.ts:222 |
| 69 | 1.00 | 69 | 1359 | README headline in packages/d2ts/README.md |
| 68 | 0.61 | 111 | 6174 | export at packages/d2ts/src/graph.ts:144 |
| 58 | 1.00 | 58 | 419 | package identity in packages/d2ts-benchmark/package.json |
| 57 | 1.00 | 57 | 6788 | export at packages/d2mini/src/utils.ts:119 |
| 57 | 1.00 | 57 | 1254 | export at packages/d2ts-benchmark/src/base.ts:19 |
| 55 | 1.00 | 55 | 9412 | export body at packages/d2mini/src/graph.ts:11 |
| 54 | 1.00 | 54 | 3291 | export names surface in packages/d2mini/src/multiset.ts |
| 53 | 1.00 | 53 | 9244 | export at packages/d2mini/src/graph.ts:11 |
| 53 | 1.00 | 53 | 235 | headings outline in RELEASING.md |
| 52 | 1.00 | 52 | 9191 | export at packages/d2mini/src/graph.ts:32 |
| 52 | 1.00 | 52 | 6226 | export body at packages/d2ts/src/graph.ts:144 |
| 52 | 1.00 | 52 | 3207 | export names surface in packages/d2mini/src/d2.ts |
