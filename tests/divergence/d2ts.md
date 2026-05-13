scores: Score(3000)=0.469 ns_rows≤3K=21/41 (reached=7 partial=2 missing=12)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 99 | 0.668 | 0.286 | 0.881 | 0.437 | 989 |
| 1442 | 145 | 0.629 | 0.195 | 0.881 | 0.351 | 1428 |
| 2080 | 204 | 0.637 | 0.193 | 0.896 | 0.350 | 1931 |
| 3000 | 264 | 0.708 | 0.311 | 0.793 | 0.469 | 2969 |
| 4327 | 354 | 0.740 | 0.350 | 0.872 | 0.509 | 4326 |
| 6240 | 517 | 0.700 | 0.240 | 0.835 | 0.410 | 6211 |
| 9000 | 787 | 0.667 | 0.237 | 0.848 | 0.398 | 8953 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 22 | 2.40 | 2.35 | 1.98 | nearby candidates have low exact atom overlap | 1.1, 2.5, 1.7, 3.8, 3.3, ... |
| tune ranking for high-overlap unscheduled candidates | 3 | 0.37 | 0.37 | 0.37 | high-overlap candidates not in the schedule by T_max, exact total=52/55 | 2.7, 2.8, 2.12 |
| promote export batches | 1 | 0.12 | 0.12 | 0.12 | 1 file, exact total=48/57 | 3.9 |
| promote imports in packages/d2ts/src/operators/index.ts | 1 | 0.00 | 0.00 | 0.00 | 0 files, exact total=3/3 | 3.13 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| export at packages/d2ts/src/electric/index.ts:<n> | 4 | 0 | 394 | 394 | off_3k=604 | export at packages/d2ts/src/electric/index.ts:40, export at packages/d2ts/src/electric/index.ts:222, export at packages/d2ts/src/electric/index.ts:103, ... |
| headings outline in README.md | 1 | 167 | 167 | 167 | off_3k=167 | headings outline in README.md |
| export body at packages/d2ts/src/electric/index.ts:328 body 333 | 1 | 0 | 165 | 165 | off_3k=188 | export body at packages/d2ts/src/electric/index.ts:328 body 333 |
| package scripts in package.json | 1 | 0 | 136 | 136 | off_3k=136 | package scripts in package.json |
| README headline in .changeset/README.md | 1 | 87 | 87 | 87 | off_3k=87 | README headline in .changeset/README.md |

Top missed paths (NS rows ≤ 3K): packages/d2ts/src/types.ts (3 rows, 60 atoms), README.md (3 rows, 25 atoms), packages/d2ts/src/order.ts (2 rows, 23 atoms), packages/d2ts/src/graph.ts (1 row, 22 atoms), packages/d2ts/src/operators/index.ts (1 row, 20 atoms), packages/d2ts/src/d2.ts (1 row, 17 atoms), packages/d2ts/src/multiset.ts (1 row, 16 atoms), packages/d2mini/README.md (1 row, 6 atoms), +1 more

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.7 | 1259 | 0.00 | missing | operators/index.ts (operator re-exports) | [unscheduled bbox exact=20/20] imports in packages/d2ts/src/operators/index.ts (20 atoms, too expensive at final margin) |
| 2.8 | 1382 | 0.00 | missing | MultiSet method names (full catalog) | [scheduled bbox exact=1/16] export names surface in packages/d2ts/src/multiset.ts (t=4079, 2 atoms); better unscheduled exact=16/16: export at packages/d2ts/src/multiset.ts:9 (39 atoms, too expensive at final margin) |
| 2.12 | 2436 | 0.00 | missing | Top-level README — operator catalog with descriptions | [scheduled bbox exact=1/19] README.md section #4 (t=3971, 1 atoms); better unscheduled exact=16/19: README.md section #3 (16 atoms, too expensive at final margin) |
| 3.9 | 6080 | 0.00 | missing | ReduceOperator: finished-versions + delta computation | [unscheduled bbox exact=48/57] export body at packages/d2ts/src/operators/reduce.ts:15 body 28 (48 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/reduce.ts:15) |
| 3.13 | 7651 | 0.00 | missing | iterate.ts class names | [unscheduled bbox exact=3/3] export names surface in packages/d2ts/src/operators/iterate.ts (5 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.1 | 77 | 0.67 | partial | README one-liner | [scheduled bbox exact=2/3] README headline in README.md (t=182, 2 atoms) |
| 1.5 | 313 | 0.67 | partial | README — incremental + Electric pitch | [scheduled bbox exact=2/3] README.md section #0 (t=963, 2 atoms) |
| 1.6 | 419 | 0.50 | missing | d2mini one-liner | [scheduled bbox exact=2/6] README headline in packages/d2mini/README.md (t=593, 2 atoms) |
| 1.7 | 494 | 0.40 | missing | d2ql one-liner | [scheduled bbox exact=2/5] README headline in packages/d2ql/README.md (t=475, 2 atoms) |
| 2.5 | 919 | 0.35 | missing | Core types: KeyValue, MessageType, Message | [scheduled bbox exact=9/18] export at packages/d2ts/src/types.ts:14 (t=3335, 9 atoms) |
| 2.9 | 1537 | 0.00 | missing | D2 class + RootStreamBuilder method names | [scheduled bbox exact=4/17] export names surface in packages/d2ts/src/d2.ts (t=6588, 8 atoms); better unscheduled exact=13/17: export at packages/d2ts/src/d2.ts:15 (32 atoms, too expensive at final margin) |
| 2.10 | 1760 | 0.36 | missing | Operator interfaces: IOperator, IDifferenceStreamReader/Writer | [scheduled bbox exact=8/20] export at packages/d2ts/src/types.ts:43 (t=3186, 8 atoms) |
| 2.11 | 2056 | 0.09 | missing | ID2 + IStreamBuilder shape | [scheduled bbox exact=11/22] export at packages/d2ts/src/types.ts:52 (t=4198, 11 atoms) |
| 3.1 | 2667 | 0.00 | missing | Antichain.create polymorphic constructor | [unscheduled bbox exact=15/19] export body at packages/d2ts/src/order.ts:138 body 142 (15 atoms, predecessor not scheduled: export at packages/d2ts/src/order.ts:138) |
| 3.2 | 2908 | 0.00 | missing | graph.ts class hierarchy (signatures only) | [unscheduled bbox exact=7/22] export at packages/d2ts/src/graph.ts:171 (20 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/graph.ts) |
| 3.3 | 3322 | 0.00 | missing | LinearUnaryOperator base class (operators/base.ts) | [unscheduled bbox exact=19/34] export body at packages/d2ts/src/operators/base.ts:9 body 13 (19 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/base.ts:9) |
| 3.4 | 3747 | 0.00 | missing | Keying operators (keyBy/unkey/rekey) | [unscheduled bbox exact=8/34] export names surface in packages/d2ts/src/operators/keying.ts (8 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |
| 3.5 | 3988 | 0.00 | missing | map() operator: canonical factory pattern | [unscheduled bbox exact=16/22] export body at packages/d2ts/src/operators/map.ts:34 body 35 (16 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/map.ts:34) |
| 3.6 | 4482 | 0.00 | missing | ConsolidateOperator: the "versions complete" pattern | [unscheduled bbox exact=32/41] export body at packages/d2ts/src/operators/consolidate.ts:16 body 20 (32 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/consolidate.ts:16) |
| 3.8 | 5362 | 0.00 | missing | ReduceOperator: class + ingest loop | [unscheduled bbox exact=33/54] export body at packages/d2ts/src/operators/reduce.ts:15 body 28 (33 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/reduce.ts:15) |
| 3.10 | 6843 | 0.00 | missing | JoinOperator: class shape + delta-join kernel | [unscheduled bbox exact=36/60] export body at packages/d2ts/src/operators/join.ts:29 body 42 (68 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/join.ts:29) |
| 3.11 | 7254 | 0.00 | missing | Join factory + JoinType + named variants | [unscheduled bbox exact=14/34] export body at packages/d2ts/src/operators/join.ts:130 body 139 (14 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/join.ts:130) |
| 3.12 | 7601 | 0.00 | missing | iterate(): scope/feedback wiring | [unscheduled bbox exact=23/33] export body at packages/d2ts/src/operators/iterate.ts:225 body 228 (23 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/iterate.ts:225) |
| 3.14 | 8104 | 0.00 | missing | groupBy + aggregate function names | [unscheduled bbox exact=17/33] export names surface in packages/d2ts/src/operators/groupBy.ts (18 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |
| 3.15 | 8641 | 0.00 | missing | topK + orderBy + indexed variants (signatures) | [unscheduled bbox exact=8/42] export at packages/d2ts/src/operators/orderBy.ts:76 (10 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/operators/orderBy.ts) |
| 5.1 | 9394 | 0.00 | missing | d2ql Query interface + compileQuery signature | [scheduled bbox exact=7/28] export names surface #2 in packages/d2ql/src/schema.ts (t=7699, 8 atoms); better unscheduled exact=12/28: export at packages/d2ql/src/schema.ts:207 (16 atoms, too expensive at final margin) |
| 5.2 | 9687 | 0.00 | missing | d2ql function + aggregate + comparator names | [scheduled bbox exact=4/31] export names surface in packages/d2ql/src/schema.ts (t=4670, 10 atoms); better unscheduled exact=13/31: export at packages/d2ql/src/schema.ts:105 (13 atoms, too expensive at final margin) |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.1 | 8722 | 0.33 | missing | sqlite/ + electric/ subdir listings | fs-only |
| 4.3 | 8937 | 0.00 | missing | d2ql src + query-builder listings | fs-only |
| 4.4 | 9068 | 0.10 | missing | Examples + benchmark listings | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.4 | 744 | 0.00 | missing | Version + Antichain class names + factory `v(…)` | [scheduled bbox exact=4/4] export names surface in packages/d2ts/src/order.ts (t=3926, 7 atoms) |
| 3.7 | 4670 | 0.00 | missing | Index<K,V> trace: signatures + interface | [scheduled bbox exact=10/11] export at packages/d2ts/src/version-index.ts:8 (t=7191, 10 atoms) |

Top wasted paths (off-NS at 3K): packages/d2ts/src/electric/index.ts (943t, 7 batches), README.md (167t, 1 batch), package.json (136t, 1 batch), packages/d2mini/src/operators (104t, 1 batch), .changeset/README.md (87t, 1 batch), packages/d2ts/README.md (69t, 1 batch), RELEASING.md (53t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 4 | 604 | export at packages/d2ts/src/electric/index.ts:<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 249 | 1.00 | 113 | 249 | 1931 | export at packages/d2ts/src/electric/index.ts:40 |
| 188 | 1.00 | 165 | 188 | 2260 | export body at packages/d2ts/src/electric/index.ts:328 body 333 |
| 167 | 1.00 | 167 | 167 | 632 | headings outline in README.md |
| 160 | 1.00 | 160 | 160 | 1771 | export at packages/d2ts/src/electric/index.ts:222 |
| 136 | 1.00 | 136 | 136 | 1292 | package scripts in package.json |
| 121 | 1.00 | 121 | 121 | 1650 | export at packages/d2ts/src/electric/index.ts:103 |
| 104 | 1.00 | 0 | 104 | 1188 | listing of 'packages/d2mini/src/operators' |
| 87 | 1.00 | 87 | 87 | 266 | README headline in .changeset/README.md |
| 80 | 1.00 | 45 | 80 | 1496 | export names surface in packages/d2ts/src/electric/index.ts |
| 74 | 1.00 | 0 | 74 | 1576 | export at packages/d2ts/src/electric/index.ts:328 |
| 193 | — | — | — | — | +3 more rows |
