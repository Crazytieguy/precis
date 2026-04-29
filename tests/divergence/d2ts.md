scores: Score(3000)=0.418 ns_rows≤3K=21/41 (reached=6 partial=2 missing=13)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 99 | 0.668 | 0.286 | 0.437 | 989 |
| 1442 | 145 | 0.629 | 0.195 | 0.351 | 1428 |
| 2080 | 204 | 0.637 | 0.193 | 0.350 | 2067 |
| 3000 | 264 | 0.685 | 0.255 | 0.418 | 2863 |
| 4327 | 354 | 0.735 | 0.343 | 0.502 | 4306 |
| 6240 | 517 | 0.702 | 0.245 | 0.415 | 6180 |
| 9000 | 787 | 0.669 | 0.237 | 0.398 | 8907 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 5 ranking-recoverable (gap@3k=0.49), 23 wrong-slice/granularity (gap@3k=2.50), 0 no-discovered (gap@3k=0.00)
Secondary intervention: free T_max budget for 1 too-expensive candidate
Top rows: 1.1, 2.5, 1.7, 3.8, 3.3, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 23 | 2.50 | 2.50 | 1.95 | nearby candidates have low exact atom overlap | 1.1, 2.5, 1.7, 3.8, 3.3, ... |
| tune ranking for discovered unscheduled candidates | 2 | 0.26 | 0.26 | 0.25 | high-overlap candidates fit but did not win, exact total=36/39 | 2.7, 2.12 |
| promote export batches | 1 | 0.12 | 0.12 | 0.12 | 1 file, exact total=48/57 | 3.9 |
| free T_max budget / demote late waste | 1 | 0.12 | 0.12 | 0.12 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=16/16 | 2.8 |
| finish partially-delivered NS batches | 1 | 0.02 | 0.02 | 0.02 | avg batch completion=0.33 | 4.1 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 5 | 5 | 0 | value/ranking |
| wrong-slice / granularity | 23 | 21 | 2 | walker granularity / wrong slice |
| fs/listing | 3 | 3 | 0 | filesystem/listing value |
| mixed/unknown | 2 | 2 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 2 | 0.12 | promote predecessor |
| too expensive at final margin | 1 | 0.12 | free T_max budget |
| discovered unscheduled | 2 | 0.26 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=16, unscheduled bbox=14, fs-only=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 12 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | missing | full | 1 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | low | 11 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.7 | 1259 | 0.00 | 0.00 | missing | operators/index.ts (operator re-exports) | [unscheduled bbox exact=20/20] imports in packages/d2ts/src/operators/index.ts (20 atoms, discovered unscheduled) |
| 2.8 | 1382 | 0.00 | 0.00 | missing | MultiSet method names (full catalog) | [scheduled bbox exact=1/16] export names surface in packages/d2ts/src/multiset.ts (t=3361, 2 atoms); better unscheduled exact=16/16: export at packages/d2ts/src/multiset.ts:9 (39 atoms, too expensive at final margin) |
| 2.12 | 2436 | 0.00 | 0.00 | missing | Top-level README — operator catalog with descriptions | [scheduled bbox exact=1/19] README.md section #4 (t=4306, 1 atoms); better unscheduled exact=16/19: README.md section #3 (16 atoms, discovered unscheduled) |
| 3.9 | 6080 | 0.00 | 0.00 | missing | ReduceOperator: finished-versions + delta computation | [unscheduled bbox exact=48/57] export body at packages/d2ts/src/operators/reduce.ts:15 body 28 (48 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/reduce.ts:15) |
| 3.13 | 7651 | 0.00 | 0.00 | missing | iterate.ts class names | [unscheduled bbox exact=3/3] export names surface in packages/d2ts/src/operators/iterate.ts (5 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 77 | 0.67 | 1.00 | partial | README one-liner | [scheduled bbox exact=2/3] README headline in README.md (t=182, 2 atoms) |
| 1.5 | 313 | 0.67 | 1.00 | partial | README — incremental + Electric pitch | [scheduled bbox exact=2/3] README.md section #0 (t=963, 2 atoms) |
| 1.6 | 419 | 0.50 | 0.99 | missing | d2mini one-liner | [scheduled bbox exact=2/6] README headline in packages/d2mini/README.md (t=593, 2 atoms) |
| 1.7 | 494 | 0.40 | 0.18 | missing | d2ql one-liner | [scheduled bbox exact=2/5] README headline in packages/d2ql/README.md (t=475, 2 atoms) |
| 2.5 | 919 | 0.00 | 0.00 | missing | Core types: KeyValue, MessageType, Message | [scheduled bbox exact=9/18] export at packages/d2ts/src/types.ts:14 (t=3596, 9 atoms) |
| 2.6 | 1019 | 0.00 | 0.00 | missing | Core types: DataMessage, FrontierMessage, PipedOperator | [scheduled bbox exact=6/10] export names surface in packages/d2ts/src/types.ts (t=3046, 16 atoms) |
| 2.9 | 1537 | 0.00 | 0.00 | missing | D2 class + RootStreamBuilder method names | [scheduled bbox exact=4/17] export names surface in packages/d2ts/src/d2.ts (t=3683, 8 atoms); better unscheduled exact=13/17: export at packages/d2ts/src/d2.ts:15 (32 atoms, discovered unscheduled) |
| 2.10 | 1760 | 0.00 | 0.00 | missing | Operator interfaces: IOperator, IDifferenceStreamReader/Writer | [scheduled bbox exact=8/20] export at packages/d2ts/src/types.ts:43 (t=3447, 8 atoms) |
| 2.11 | 2056 | 0.00 | 0.00 | missing | ID2 + IStreamBuilder shape | [scheduled bbox exact=11/22] export at packages/d2ts/src/types.ts:52 (t=4425, 11 atoms) |
| 3.1 | 2667 | 0.00 | 0.00 | missing | Antichain.create polymorphic constructor | [unscheduled bbox exact=15/19] export body at packages/d2ts/src/order.ts:138 body 142 (15 atoms, predecessor not scheduled: export at packages/d2ts/src/order.ts:138) |
| 3.2 | 2908 | 0.00 | 0.00 | missing | graph.ts class hierarchy (signatures only) | [scheduled bbox exact=4/22] export at packages/d2ts/src/graph.ts:144 (t=7931, 14 atoms); better unscheduled exact=7/22: export at packages/d2ts/src/graph.ts:171 (20 atoms, discovered unscheduled) |
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
| 5.1 | 9394 | 0.00 | 0.00 | missing | d2ql Query interface + compileQuery signature | [scheduled bbox exact=7/28] export names surface #2 in packages/d2ql/src/schema.ts (t=6675, 8 atoms); better unscheduled exact=12/28: export at packages/d2ql/src/schema.ts:207 (16 atoms, discovered unscheduled) |
| 5.2 | 9687 | 0.00 | 0.00 | missing | d2ql function + aggregate + comparator names | [scheduled bbox exact=4/31] export names surface in packages/d2ql/src/schema.ts (t=4862, 10 atoms); better unscheduled exact=13/31: export at packages/d2ql/src/schema.ts:105 (13 atoms, discovered unscheduled) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 8722 | 0.33 | 0.33 | missing | sqlite/ + electric/ subdir listings | fs-only |
| 4.3 | 8937 | 0.00 | 0.00 | missing | d2ql src + query-builder listings | fs-only |
| 4.4 | 9068 | 0.10 | 0.10 | missing | Examples + benchmark listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.4 | 744 | 0.00 | 0.00 | missing | Version + Antichain class names + factory `v(…)` | [scheduled bbox exact=4/4] export names surface in packages/d2ts/src/order.ts (t=3289, 7 atoms) |
| 3.7 | 4670 | 0.00 | 0.00 | missing | Index<K,V> trace: signatures + interface | [scheduled bbox exact=10/11] export at packages/d2ts/src/version-index.ts:8 (t=9061, 10 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 394 | export at packages/d2ts/src/electric/index.ts:<n> |
| 2 | 167 | export at packages/d2ts/src/utils.ts:<n> |
| 2 | 139 | export at packages/d2ts/src/graph.ts:<n> |
| 2 | 136 | export at packages/d2mini/src/utils.ts:<n> |
| 2 | 131 | export at packages/d2ts-benchmark/src/base.ts:<n> |
| 2 | 122 | export at packages/d2ql/src/evaluators.ts:<n> |
| 2 | 104 | export at packages/d2ql/src/extractors.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 216 | 1.00 | 216 | 8315 | package dependencies in package.json |
| 201 | 1.00 | 201 | 5063 | export names surface in packages/d2ql/src/types.ts |
| 183 | 0.95 | 193 | 5336 | export names surface #1 in packages/d2ql/src/schema.ts |
| 167 | 1.00 | 167 | 799 | headings outline in README.md |
| 165 | 0.88 | 188 | 2599 | export body at packages/d2ts/src/electric/index.ts:328 body 333 |
| 164 | 0.87 | 188 | 4862 | export names surface in packages/d2ql/src/schema.ts |
| 160 | 1.00 | 160 | 2067 | export at packages/d2ts/src/electric/index.ts:222 |
| 136 | 1.00 | 136 | 1428 | package scripts in package.json |
| 121 | 1.00 | 121 | 1907 | export at packages/d2ts/src/electric/index.ts:103 |
| 118 | 1.00 | 118 | 3940 | imports in packages/d2ts-benchmark/src/index.ts |
| 2890 | — | — | — | +40 more rows |
