scores: Sim=0.298 Reached=10/41 Early=2 Late=6 Partial=4 Missing=27 Used=9963/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 3 | 4 | 0 | 0.78 |
| 2 | 12 | 3 | 0 | 9 | 0.28 |
| 3 | 15 | 2 | 0 | 13 | 0.13 |
| 4 | 4 | 1 | 0 | 3 | 0.43 |
| 5 | 3 | 1 | 0 | 2 | 0.38 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 77 | — | — | 0.67 | partial | README one-liner | README headline in README.md (t=182, 2 atoms) |
| 1.2 | 131 | 54 | -77 | 1.00 | early | Top-level workspace listing |  |
| 1.3 | 153 | 257 | +104 | 1.00 | late | Packages directory listing |  |
| 1.4 | 179 | 1086 | +907 | 1.00 | late | pnpm workspace globs | plaintext config pnpm-workspace.yaml (t=1086, 3 atoms) |
| 1.5 | 313 | — | — | 0.67 | partial | README — incremental + Electric pitch | README.md section #0 (t=1003, 2 atoms) |
| 1.6 | 419 | — | — | 0.50 | partial | d2mini one-liner | README headline in packages/d2mini/README.md (t=633, 2 atoms) |
| 1.7 | 494 | — | — | 0.60 | partial | d2ql one-liner | README headline in packages/d2ql/README.md (t=482, 2 atoms) |
| 2.1 | 538 | 1983 | +1445 | 1.00 | late | d2ts src layout |  |
| 2.2 | 608 | 3081 | +2473 | 1.00 | late | d2ts package re-exports (`index.ts`) | imports in packages/d2ts/src/index.ts (t=3081, 6 atoms) |
| 2.3 | 706 | — | — | 0.00 | missing | Operator catalog (operators/ filenames) |  |
| 2.4 | 744 | 3134 | +2390 | 1.00 | late | Version + Antichain class names + factory `v(…)` | export body at packages/d2ts/src/order.ts:10 (t=8163, 8 atoms) |
| 2.5 | 919 | — | — | 0.00 | missing | Core types: KeyValue, MessageType, Message |  |
| 2.6 | 1019 | — | — | 0.00 | missing | Core types: DataMessage, FrontierMessage, PipedOperator |  |
| 2.7 | 1259 | — | — | 0.00 | missing | operators/index.ts (operator re-exports) |  |
| 2.8 | 1382 | — | — | 0.06 | missing | MultiSet method names (full catalog) | export names surface in packages/d2ts/src/multiset.ts (t=3206, 2 atoms) |
| 2.9 | 1537 | — | — | 0.29 | missing | D2 class + RootStreamBuilder method names | export names surface in packages/d2ts/src/d2.ts (t=3271, 8 atoms) |
| 2.10 | 1760 | — | — | 0.00 | missing | Operator interfaces: IOperator, IDifferenceStreamReader/Writer |  |
| 2.11 | 2056 | — | — | 0.00 | missing | ID2 + IStreamBuilder shape |  |
| 2.12 | 2436 | — | — | 0.00 | missing | Top-level README — operator catalog with descriptions |  |
| 3.1 | 2667 | — | — | 0.00 | missing | Antichain.create polymorphic constructor |  |
| 3.2 | 2908 | 7981 | +5073 | 1.00 | late | graph.ts class hierarchy (signatures only) | export at packages/d2ts/src/graph.ts:171 (t=7981, 20 atoms) |
| 3.3 | 3322 | — | — | 0.00 | missing | LinearUnaryOperator base class (operators/base.ts) |  |
| 3.4 | 3747 | — | — | 0.00 | missing | Keying operators (keyBy/unkey/rekey) |  |
| 3.5 | 3988 | — | — | 0.00 | missing | map() operator: canonical factory pattern |  |
| 3.6 | 4482 | — | — | 0.00 | missing | ConsolidateOperator: the "versions complete" pattern |  |
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
| 4.4 | 9068 | — | — | 0.10 | missing | Examples + benchmark listings |  |
| 5.1 | 9394 | — | — | 0.14 | missing | d2ql Query interface + compileQuery signature | export at packages/d2ql/src/compiler.ts:16 (t=9276, 4 atoms) |
| 5.2 | 9687 | — | — | 0.00 | missing | d2ql function + aggregate + comparator names |  |
| 5.3 | 9955 | 2671 | -7284 | 1.00 | early | Electric adapter entry points | export body at packages/d2ts/src/electric/index.ts:222 (t=6360, 76 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 394 | export at packages/d2ts/src/electric/index.ts:<n> |
| 5 | 380 | export at packages/d2mini/src/graph.ts:<n> |
| 4 | 357 | export at packages/d2ts/src/graph.ts:<n> |
| 2 | 198 | export at packages/d2ts/src/sqlite/database.ts:<n> |
| 2 | 167 | export at packages/d2ts/src/utils.ts:<n> |
| 2 | 136 | export at packages/d2mini/src/utils.ts:<n> |
| 2 | 131 | export at packages/d2ts-benchmark/src/base.ts:<n> |
| 2 | 104 | export at packages/d2ql/src/extractors.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 948 | 1.00 | 948 | 6360 | export body at packages/d2ts/src/electric/index.ts:222 |
| 238 | 1.00 | 238 | 8920 | imports in packages/d2ts/src/electric/index.ts |
| 216 | 1.00 | 216 | 4560 | package dependencies in package.json |
| 169 | 1.00 | 169 | 8518 | headings outline in packages/d2ts/README.md |
| 167 | 1.00 | 167 | 839 | headings outline in README.md |
| 165 | 0.88 | 188 | 2904 | export body at packages/d2ts/src/electric/index.ts:328 |
| 164 | 1.00 | 164 | 8682 | packages/d2ts/README.md section #0 |
| 160 | 1.00 | 160 | 2422 | export at packages/d2ts/src/electric/index.ts:222 |
| 136 | 1.00 | 136 | 1327 | package scripts in package.json |
| 131 | 0.82 | 160 | 6719 | export at packages/d2ts/src/graph.ts:111 |
| 123 | 1.00 | 123 | 7002 | export names surface in packages/d2mini/src/graph.ts |
| 121 | 1.00 | 121 | 2262 | export at packages/d2ts/src/electric/index.ts:103 |
| 118 | 1.00 | 118 | 1827 | imports in packages/d2ts-benchmark/src/index.ts |
| 113 | 0.45 | 249 | 2671 | export at packages/d2ts/src/electric/index.ts:40 |
| 106 | 1.00 | 106 | 5305 | export names surface in packages/d2mini/src/types.ts |
| 102 | 1.00 | 102 | 6462 | export at packages/d2ts/src/sqlite/database.ts:49 |
| 99 | 1.00 | 99 | 7567 | export at packages/d2mini/src/graph.ts:56 |
| 97 | 1.00 | 97 | 6559 | export body at packages/d2ts/src/utils.ts:7 |
| 97 | 1.00 | 97 | 4689 | export names surface in packages/d2mini/src/utils.ts |
| 96 | 1.00 | 96 | 3898 | export at packages/d2ts/src/sqlite/database.ts:32 |
| 94 | 1.00 | 94 | 7429 | export at packages/d2mini/src/graph.ts:100 |
| 94 | 1.00 | 94 | 7721 | export body at packages/d2mini/src/graph.ts:32 |
| 93 | 1.00 | 93 | 8256 | export body at packages/d2mini/src/utils.ts:37 |
| 93 | 1.00 | 93 | 8349 | export body at packages/d2ts/src/utils.ts:62 |
| 92 | 1.00 | 92 | 3635 | export names surface in packages/d2ts/src/utils.ts |
| 91 | 1.00 | 91 | 8072 | export body at packages/d2ts/src/graph.ts:171 |
| 91 | 1.00 | 91 | 8163 | export body at packages/d2ts/src/order.ts:10 |
| 89 | 1.00 | 89 | 1916 | headings outline in packages/d2ts-benchmark/CHANGELOG.md |
| 88 | 1.00 | 88 | 3802 | export at packages/d2ts/src/utils.ts:7 |
| 88 | 1.00 | 88 | 5111 | export body at packages/d2mini/src/utils.ts:6 |
| 88 | 1.00 | 88 | 5199 | export body at packages/d2ts/src/utils.ts:31 |
| 87 | 1.00 | 87 | 344 | README headline in .changeset/README.md |
| 87 | 0.49 | 179 | 7981 | export at packages/d2ts/src/graph.ts:171 |
| 82 | 1.00 | 82 | 7249 | export at packages/d2mini/src/graph.ts:80 |
| 81 | 1.00 | 81 | 6800 | export body at packages/d2ts/src/graph.ts:111 |
| 79 | 1.00 | 79 | 4869 | export at packages/d2mini/src/utils.ts:6 |
| 79 | 1.00 | 79 | 3714 | export at packages/d2ts/src/utils.ts:31 |
| 74 | 1.00 | 74 | 1401 | export at packages/d2ts-benchmark/src/base.ts:4 |
| 71 | 0.61 | 116 | 4344 | export at packages/d2ts/src/graph.ts:54 |
| 71 | 1.00 | 71 | 2975 | export doc at packages/d2ts/src/electric/index.ts:222 |
| 69 | 1.00 | 69 | 1191 | README headline in packages/d2ts/README.md |
| 68 | 0.61 | 111 | 4176 | export at packages/d2ts/src/graph.ts:144 |
| 67 | 1.00 | 67 | 9963 | export at packages/d2ql/src/order-by.ts:16 |
| 60 | 1.00 | 60 | 9896 | export at packages/d2ql/src/select.ts:7 |
| 57 | 1.00 | 57 | 4790 | export at packages/d2mini/src/utils.ts:119 |
| 57 | 1.00 | 57 | 1060 | export at packages/d2ts-benchmark/src/base.ts:19 |
| 56 | 1.00 | 56 | 9836 | export at packages/d2ql/src/key-by.ts:4 |
| 55 | 1.00 | 55 | 7335 | export body at packages/d2mini/src/graph.ts:11 |
| 55 | 1.00 | 55 | 9044 | module-doc lede in packages/d2ql/src/index.ts |
| 54 | 1.00 | 54 | 1709 | export names surface in packages/d2mini/src/multiset.ts |
| 53 | 1.00 | 53 | 7167 | export at packages/d2mini/src/graph.ts:11 |
| 53 | 1.00 | 53 | 235 | headings outline in RELEASING.md |
| 52 | 1.00 | 52 | 7114 | export at packages/d2mini/src/graph.ts:32 |
| 52 | 1.00 | 52 | 9695 | export at packages/d2ql/src/extractors.ts:12 |
| 52 | 1.00 | 52 | 9747 | export at packages/d2ql/src/extractors.ts:80 |
| 52 | 1.00 | 52 | 4228 | export body at packages/d2ts/src/graph.ts:144 |
| 52 | 1.00 | 52 | 1625 | export names surface in packages/d2mini/src/d2.ts |
| 51 | 1.00 | 51 | 9457 | export at packages/d2ql/src/evaluators.ts:14 |
