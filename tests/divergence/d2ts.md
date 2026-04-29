scores: Score(3000)=0.469 ns_rows≤3K=21/41 (reached=7 partial=2 missing=12)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 99 | 0.668 | 0.286 | 0.437 | 989 |
| 1442 | 145 | 0.629 | 0.195 | 0.351 | 1428 |
| 2080 | 204 | 0.637 | 0.193 | 0.350 | 1931 |
| 3000 | 264 | 0.708 | 0.311 | 0.469 | 2969 |
| 4327 | 354 | 0.740 | 0.350 | 0.509 | 4326 |
| 6240 | 517 | 0.700 | 0.240 | 0.410 | 6211 |
| 9000 | 787 | 0.667 | 0.237 | 0.398 | 8953 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 5 ranking-recoverable (gap@3k=0.49), 22 wrong-slice/granularity (gap@3k=2.35), 0 no-discovered (gap@3k=0.00)
Secondary intervention: promote predecessors for 2 gated candidates
Top rows: 1.1, 2.5, 1.7, 3.8, 3.3, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 22 | 2.40 | 2.35 | 1.98 | nearby candidates have low exact atom overlap | 1.1, 2.5, 1.7, 3.8, 3.3, ... |
| tune ranking for high-overlap unscheduled candidates | 3 | 0.37 | 0.37 | 0.37 | high-overlap candidates not in the schedule by T_max, exact total=52/55 | 2.7, 2.8, 2.12 |
| promote export batches | 1 | 0.12 | 0.12 | 0.12 | 1 file, exact total=48/57 | 3.9 |
| promote imports in packages/d2ts/src/operators/index.ts | 1 | 0.00 | 0.00 | 0.00 | 0 files, exact total=3/3 | 3.13 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 5 | 5 | 0 | value/ranking |
| wrong-slice / granularity | 22 | 20 | 2 | walker granularity / wrong slice |
| fs/listing | 3 | 3 | 0 | filesystem/listing value |
| mixed/unknown | 2 | 2 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 2 | 0.12 | promote predecessor |
| too expensive at final margin | 3 | 0.37 | tune ranking |

Candidate hint kinds: scheduled bbox=14, unscheduled bbox=15, fs-only=3 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 10 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | missing | full | 1 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | low | 12 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.7 | 1259 | 0.00 | 0.00 | missing | operators/index.ts (operator re-exports) | [unscheduled bbox exact=20/20] imports in packages/d2ts/src/operators/index.ts (20 atoms, too expensive at final margin) |
| 2.8 | 1382 | 0.00 | 0.00 | missing | MultiSet method names (full catalog) | [scheduled bbox exact=1/16] export names surface in packages/d2ts/src/multiset.ts (t=4079, 2 atoms); better unscheduled exact=16/16: export at packages/d2ts/src/multiset.ts:9 (39 atoms, too expensive at final margin) |
| 2.12 | 2436 | 0.00 | 0.00 | missing | Top-level README — operator catalog with descriptions | [scheduled bbox exact=1/19] README.md section #4 (t=3971, 1 atoms); better unscheduled exact=16/19: README.md section #3 (16 atoms, too expensive at final margin) |
| 3.9 | 6080 | 0.00 | 0.00 | missing | ReduceOperator: finished-versions + delta computation | [unscheduled bbox exact=48/57] export body at packages/d2ts/src/operators/reduce.ts:15 body 28 (48 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/reduce.ts:15) |
| 3.13 | 7651 | 0.00 | 0.00 | missing | iterate.ts class names | [unscheduled bbox exact=3/3] export names surface in packages/d2ts/src/operators/iterate.ts (5 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 77 | 0.67 | 1.00 | partial | README one-liner | [scheduled bbox exact=2/3] README headline in README.md (t=182, 2 atoms) |
| 1.5 | 313 | 0.67 | 1.00 | partial | README — incremental + Electric pitch | [scheduled bbox exact=2/3] README.md section #0 (t=963, 2 atoms) |
| 1.6 | 419 | 0.50 | 0.99 | missing | d2mini one-liner | [scheduled bbox exact=2/6] README headline in packages/d2mini/README.md (t=593, 2 atoms) |
| 1.7 | 494 | 0.40 | 0.18 | missing | d2ql one-liner | [scheduled bbox exact=2/5] README headline in packages/d2ql/README.md (t=475, 2 atoms) |
| 2.5 | 919 | 0.35 | 0.48 | missing | Core types: KeyValue, MessageType, Message | [scheduled bbox exact=9/18] export at packages/d2ts/src/types.ts:14 (t=3335, 9 atoms) |
| 2.9 | 1537 | 0.00 | 0.00 | missing | D2 class + RootStreamBuilder method names | [scheduled bbox exact=4/17] export names surface in packages/d2ts/src/d2.ts (t=6588, 8 atoms); better unscheduled exact=13/17: export at packages/d2ts/src/d2.ts:15 (32 atoms, too expensive at final margin) |
| 2.10 | 1760 | 0.36 | 0.39 | missing | Operator interfaces: IOperator, IDifferenceStreamReader/Writer | [scheduled bbox exact=8/20] export at packages/d2ts/src/types.ts:43 (t=3186, 8 atoms) |
| 2.11 | 2056 | 0.09 | 0.08 | missing | ID2 + IStreamBuilder shape | [scheduled bbox exact=11/22] export at packages/d2ts/src/types.ts:52 (t=4198, 11 atoms) |
| 3.1 | 2667 | 0.00 | 0.00 | missing | Antichain.create polymorphic constructor | [unscheduled bbox exact=15/19] export body at packages/d2ts/src/order.ts:138 body 142 (15 atoms, predecessor not scheduled: export at packages/d2ts/src/order.ts:138) |
| 3.2 | 2908 | 0.00 | 0.00 | missing | graph.ts class hierarchy (signatures only) | [unscheduled bbox exact=7/22] export at packages/d2ts/src/graph.ts:171 (20 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/graph.ts) |
| 3.3 | 3322 | 0.00 | 0.00 | missing | LinearUnaryOperator base class (operators/base.ts) | [unscheduled bbox exact=19/34] export body at packages/d2ts/src/operators/base.ts:9 body 13 (19 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/base.ts:9) |
| 3.4 | 3747 | 0.00 | 0.00 | missing | Keying operators (keyBy/unkey/rekey) | [unscheduled bbox exact=8/34] export names surface in packages/d2ts/src/operators/keying.ts (8 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |
| 3.5 | 3988 | 0.00 | 0.00 | missing | map() operator: canonical factory pattern | [unscheduled bbox exact=16/22] export body at packages/d2ts/src/operators/map.ts:34 body 35 (16 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/map.ts:34) |
| 3.6 | 4482 | 0.00 | 0.00 | missing | ConsolidateOperator: the "versions complete" pattern | [unscheduled bbox exact=32/41] export body at packages/d2ts/src/operators/consolidate.ts:16 body 20 (32 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/consolidate.ts:16) |
| 3.8 | 5362 | 0.00 | 0.00 | missing | ReduceOperator: class + ingest loop | [unscheduled bbox exact=33/54] export body at packages/d2ts/src/operators/reduce.ts:15 body 28 (33 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/reduce.ts:15) |
| 3.10 | 6843 | 0.00 | 0.00 | missing | JoinOperator: class shape + delta-join kernel | [unscheduled bbox exact=36/60] export body at packages/d2ts/src/operators/join.ts:29 body 42 (68 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/join.ts:29) |
| 3.11 | 7254 | 0.00 | 0.00 | missing | Join factory + JoinType + named variants | [unscheduled bbox exact=14/34] export body at packages/d2ts/src/operators/join.ts:130 body 139 (14 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/join.ts:130) |
| 3.12 | 7601 | 0.00 | 0.00 | missing | iterate(): scope/feedback wiring | [unscheduled bbox exact=23/33] export body at packages/d2ts/src/operators/iterate.ts:225 body 228 (23 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/iterate.ts:225) |
| 3.14 | 8104 | 0.00 | 0.00 | missing | groupBy + aggregate function names | [unscheduled bbox exact=17/33] export names surface in packages/d2ts/src/operators/groupBy.ts (18 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |
| 3.15 | 8641 | 0.00 | 0.00 | missing | topK + orderBy + indexed variants (signatures) | [unscheduled bbox exact=8/42] export at packages/d2ts/src/operators/orderBy.ts:76 (10 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/operators/orderBy.ts) |
| 5.1 | 9394 | 0.00 | 0.00 | missing | d2ql Query interface + compileQuery signature | [scheduled bbox exact=7/28] export names surface #2 in packages/d2ql/src/schema.ts (t=7699, 8 atoms); better unscheduled exact=12/28: export at packages/d2ql/src/schema.ts:207 (16 atoms, too expensive at final margin) |
| 5.2 | 9687 | 0.00 | 0.00 | missing | d2ql function + aggregate + comparator names | [scheduled bbox exact=4/31] export names surface in packages/d2ql/src/schema.ts (t=4670, 10 atoms); better unscheduled exact=13/31: export at packages/d2ql/src/schema.ts:105 (13 atoms, too expensive at final margin) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 8722 | 0.33 | 0.33 | missing | sqlite/ + electric/ subdir listings | fs-only |
| 4.3 | 8937 | 0.00 | 0.00 | missing | d2ql src + query-builder listings | fs-only |
| 4.4 | 9068 | 0.10 | 0.10 | missing | Examples + benchmark listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.4 | 744 | 0.00 | 0.00 | missing | Version + Antichain class names + factory `v(…)` | [scheduled bbox exact=4/4] export names surface in packages/d2ts/src/order.ts (t=3926, 7 atoms) |
| 3.7 | 4670 | 0.00 | 0.00 | missing | Index<K,V> trace: signatures + interface | [scheduled bbox exact=10/11] export at packages/d2ts/src/version-index.ts:8 (t=7191, 10 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 394 | export at packages/d2ts/src/electric/index.ts:<n> |
| 2 | 198 | export at packages/d2ts/src/sqlite/database.ts:<n> |
| 2 | 131 | export at packages/d2ts-benchmark/src/base.ts:<n> |
| 2 | 122 | export at packages/d2ql/src/evaluators.ts:<n> |
| 2 | 119 | export at packages/d2ql/src/schema.ts:<n> |
| 2 | 114 | export at packages/d2mini/src/operators/count.ts:<n> |
| 2 | 104 | export at packages/d2ql/src/extractors.ts:<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 167 | 1.00 | 167 | 799 | headings outline in README.md |
| 165 | 0.88 | 188 | 2448 | export body at packages/d2ts/src/electric/index.ts:328 body 333 |
| 160 | 1.00 | 160 | 1931 | export at packages/d2ts/src/electric/index.ts:222 |
| 136 | 1.00 | 136 | 1428 | package scripts in package.json |
| 121 | 1.00 | 121 | 1771 | export at packages/d2ts/src/electric/index.ts:103 |
| 113 | 0.45 | 249 | 2180 | export at packages/d2ts/src/electric/index.ts:40 |
| 87 | 1.00 | 87 | 353 | README headline in .changeset/README.md |
| 71 | 1.00 | 71 | 2632 | export doc at packages/d2ts/src/electric/index.ts:222 |
| 69 | 1.00 | 69 | 1094 | README headline in packages/d2ts/README.md |
| 53 | 1.00 | 53 | 244 | headings outline in RELEASING.md |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 948 | 1.00 | 948 | 8732 | export body at packages/d2ts/src/electric/index.ts:222 body 234 |
| 218 | 1.00 | 218 | 9375 | imports in packages/d2mini/src/operators/index.ts |
| 216 | 1.00 | 216 | 6961 | package dependencies in package.json |
| 201 | 1.00 | 201 | 4871 | export names surface in packages/d2ql/src/types.ts |
| 183 | 0.95 | 193 | 5322 | export names surface #1 in packages/d2ql/src/schema.ts |
| 164 | 0.87 | 188 | 4670 | export names surface in packages/d2ql/src/schema.ts |
| 118 | 1.00 | 118 | 3499 | imports in packages/d2ts-benchmark/src/index.ts |
| 102 | 1.00 | 102 | 8834 | export at packages/d2ts/src/sqlite/database.ts:49 |
| 96 | 1.00 | 96 | 7334 | export at packages/d2ts/src/sqlite/database.ts:32 |
| 89 | 1.00 | 89 | 3768 | headings outline in packages/d2ts-benchmark/CHANGELOG.md |
| 1392 | — | — | — | +23 more rows |
