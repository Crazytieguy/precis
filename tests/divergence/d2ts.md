scores: Sim=0.340 Reached=14/41 Early=4 Late=10 Partial=7 Missing=20 Used=9659/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 5 ranking-recoverable (w×gap=1.32), 20 wrong-slice/granularity (w×gap=2.94), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 1 too-expensive candidate
Loss reasons: 2 predecessor-gated, 1 too-expensive, 2 discovered-unscheduled
Top rows: 1.6, 2.9, 1.1, 1.7, 1.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 20 | 2.94 | 8/13/20 | nearby candidates have low exact atom overlap | 1.6, 2.9, 1.1, 1.7, 1.5, ... |
| tune ranking for discovered unscheduled candidates | 2 | 0.78 | 2/2/2 | high-overlap candidates fit but did not win, exact total=36/39 | 2.7, 2.12 |
| free final budget / demote late waste | 1 | 0.47 | 1/1/1 | high-overlap candidates exceed final remaining budget, exact total=16/16 | 2.8 |
| promote export batches | 1 | 0.05 | 0/0/1 | 1 file, exact total=48/57 | 3.9 |
| promote imports in packages/d2ts/src/operators/index.ts | 1 | 0.02 | 0/0/1 | 0 files, exact total=3/3 | 3.13 |

Tiers: 1=3/7 reached, 4 partial, 0 missing, avg=0.78; 2=7/12 reached, 1 partial, 4 missing, avg=0.65; 3=1/15 reached, 1 partial, 13 missing, avg=0.11; 4=2/4 reached, 0 partial, 2 missing, avg=0.61; 5=1/3 reached, 1 partial, 1 missing, avg=0.55

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 5 | 5 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 20 | 13 | 7 | 0 | walker granularity / wrong slice |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 14 | 0 | 0 | 14 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.07 | promote predecessor |
| too expensive at final margin | 1 | 0.47 | free final budget |
| discovered unscheduled | 2 | 0.78 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=19, unscheduled bbox=14, fs-only=8

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | low | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | low | 7 |
| unscheduled bbox | missing | low | 11 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.7 | 1259 | — | — | 0.00 | missing | operators/index.ts (operator re-exports) | [unscheduled bbox exact=20/20] imports in packages/d2ts/src/operators/index.ts (20 atoms, discovered unscheduled) |
| 2.8 | 1382 | — | — | 0.06 | missing | MultiSet method names (full catalog) | [scheduled bbox exact=1/16] export names surface in packages/d2ts/src/multiset.ts (t=3361, 2 atoms); better unscheduled exact=16/16: export at packages/d2ts/src/multiset.ts:9 (39 atoms, too expensive at final margin) |
| 2.12 | 2436 | — | — | 0.16 | missing | Top-level README — operator catalog with descriptions | [scheduled bbox exact=1/19] README.md section #4 (t=4306, 1 atoms); better unscheduled exact=16/19: README.md section #3 (16 atoms, discovered unscheduled) |
| 3.9 | 6080 | — | — | 0.00 | missing | ReduceOperator: finished-versions + delta computation | [unscheduled bbox exact=48/57] export body at packages/d2ts/src/operators/reduce.ts:15 body 28 (48 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/reduce.ts:15) |
| 3.13 | 7651 | — | — | 0.00 | missing | iterate.ts class names | [unscheduled bbox exact=3/3] export names surface in packages/d2ts/src/operators/iterate.ts (5 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 77 | — | — | 0.67 | partial | README one-liner | [scheduled bbox exact=2/3] README headline in README.md (t=182, 2 atoms) |
| 1.5 | 313 | — | — | 0.67 | partial | README — incremental + Electric pitch | [scheduled bbox exact=2/3] README.md section #0 (t=963, 2 atoms) |
| 1.6 | 419 | — | — | 0.50 | partial | d2mini one-liner | [scheduled bbox exact=2/6] README headline in packages/d2mini/README.md (t=593, 2 atoms) |
| 1.7 | 494 | — | — | 0.60 | partial | d2ql one-liner | [scheduled bbox exact=2/5] README headline in packages/d2ql/README.md (t=475, 2 atoms) |
| 2.9 | 1537 | — | — | 0.29 | missing | D2 class + RootStreamBuilder method names | [scheduled bbox exact=4/17] export names surface in packages/d2ts/src/d2.ts (t=3683, 8 atoms); better unscheduled exact=13/17: export at packages/d2ts/src/d2.ts:15 (32 atoms, discovered unscheduled) |
| 2.11 | 2056 | — | — | 0.55 | partial | ID2 + IStreamBuilder shape | [scheduled bbox exact=11/22] export at packages/d2ts/src/types.ts:52 (t=4425, 11 atoms) |
| 3.1 | 2667 | — | — | 0.00 | missing | Antichain.create polymorphic constructor | [unscheduled bbox exact=15/19] export body at packages/d2ts/src/order.ts:138 body 142 (15 atoms, predecessor not scheduled: export at packages/d2ts/src/order.ts:138) |
| 3.2 | 2908 | — | — | 0.64 | partial | graph.ts class hierarchy (signatures only) | [scheduled bbox exact=4/22] export at packages/d2ts/src/graph.ts:144 (t=7931, 14 atoms); better unscheduled exact=7/22: export at packages/d2ts/src/graph.ts:171 (20 atoms, discovered unscheduled) |
| 3.3 | 3322 | — | — | 0.00 | missing | LinearUnaryOperator base class (operators/base.ts) | [unscheduled bbox exact=19/34] export body at packages/d2ts/src/operators/base.ts:9 body 13 (19 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/base.ts:9) |
| 3.4 | 3747 | — | — | 0.00 | missing | Keying operators (keyBy/unkey/rekey) | [unscheduled bbox exact=8/34] export names surface in packages/d2ts/src/operators/keying.ts (8 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |
| 3.5 | 3988 | — | — | 0.00 | missing | map() operator: canonical factory pattern | [unscheduled bbox exact=16/22] export body at packages/d2ts/src/operators/map.ts:34 body 35 (16 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/map.ts:34) |
| 3.6 | 4482 | — | — | 0.00 | missing | ConsolidateOperator: the "versions complete" pattern | [unscheduled bbox exact=32/41] export body at packages/d2ts/src/operators/consolidate.ts:16 body 20 (32 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/consolidate.ts:16) |
| 3.8 | 5362 | — | — | 0.00 | missing | ReduceOperator: class + ingest loop | [unscheduled bbox exact=33/54] export body at packages/d2ts/src/operators/reduce.ts:15 body 28 (33 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/reduce.ts:15) |
| 3.10 | 6843 | — | — | 0.00 | missing | JoinOperator: class shape + delta-join kernel | [unscheduled bbox exact=36/60] export body at packages/d2ts/src/operators/join.ts:29 body 42 (68 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/join.ts:29) |
| 3.11 | 7254 | — | — | 0.00 | missing | Join factory + JoinType + named variants | [unscheduled bbox exact=14/34] export body at packages/d2ts/src/operators/join.ts:130 body 139 (14 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/join.ts:130) |
| 3.12 | 7601 | — | — | 0.00 | missing | iterate(): scope/feedback wiring | [unscheduled bbox exact=23/33] export body at packages/d2ts/src/operators/iterate.ts:225 body 228 (23 atoms, predecessor not scheduled: export at packages/d2ts/src/operators/iterate.ts:225) |
| 3.14 | 8104 | — | — | 0.00 | missing | groupBy + aggregate function names | [unscheduled bbox exact=17/33] export names surface in packages/d2ts/src/operators/groupBy.ts (18 atoms, predecessor not scheduled: imports in packages/d2ts/src/operators/index.ts) |
| 3.15 | 8641 | — | — | 0.00 | missing | topK + orderBy + indexed variants (signatures) | [unscheduled bbox exact=8/42] export at packages/d2ts/src/operators/orderBy.ts:76 (10 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/operators/orderBy.ts) |
| 5.1 | 9394 | — | — | 0.54 | partial | d2ql Query interface + compileQuery signature | [scheduled bbox exact=7/28] export names surface #2 in packages/d2ql/src/schema.ts (t=6675, 8 atoms); better unscheduled exact=12/28: export at packages/d2ql/src/schema.ts:207 (16 atoms, discovered unscheduled) |
| 5.2 | 9687 | — | — | 0.11 | missing | d2ql function + aggregate + comparator names | [scheduled bbox exact=4/31] export names surface in packages/d2ql/src/schema.ts (t=4862, 10 atoms); better unscheduled exact=13/31: export at packages/d2ql/src/schema.ts:105 (13 atoms, discovered unscheduled) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 8722 | — | — | 0.33 | missing | sqlite/ + electric/ subdir listings | fs-only |
| 4.4 | 9068 | — | — | 0.10 | missing | Examples + benchmark listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 131 | 54 | -77 | 1.00 | early | Top-level workspace listing | fs-only |
| 1.3 | 153 | 266 | +113 | 1.00 | late | Packages directory listing | fs-only |
| 1.4 | 179 | 989 | +810 | 1.00 | late | pnpm workspace globs | [scheduled bbox exact=3/3] plaintext config pnpm-workspace.yaml (t=989, 3 atoms) |
| 2.1 | 538 | 1608 | +1070 | 1.00 | late | d2ts src layout | fs-only |
| 2.2 | 608 | 2863 | +2255 | 1.00 | late | d2ts package re-exports (`index.ts`) | [scheduled bbox exact=6/6] imports in packages/d2ts/src/index.ts (t=2863, 6 atoms) |
| 2.3 | 706 | 2697 | +1991 | 1.00 | late | Operator catalog (operators/ filenames) | fs-only |
| 2.4 | 744 | 3289 | +2545 | 1.00 | late | Version + Antichain class names + factory `v(…)` | [scheduled bbox exact=4/4] export names surface in packages/d2ts/src/order.ts (t=3289, 7 atoms) |
| 2.5 | 919 | 3596 | +2677 | 0.94 | late | Core types: KeyValue, MessageType, Message | [scheduled bbox exact=9/18] export at packages/d2ts/src/types.ts:14 (t=3596, 9 atoms) |
| 2.6 | 1019 | 3090 | +2071 | 0.90 | late | Core types: DataMessage, FrontierMessage, PipedOperator | [scheduled bbox exact=6/10] export names surface in packages/d2ts/src/types.ts (t=3046, 16 atoms) |
| 2.10 | 1760 | 3447 | +1687 | 0.90 | late | Operator interfaces: IOperator, IDifferenceStreamReader/Writer | [scheduled bbox exact=8/20] export at packages/d2ts/src/types.ts:43 (t=3447, 8 atoms) |
| 3.7 | 4670 | 9061 | +4391 | 1.00 | late | Index<K,V> trace: signatures + interface | [scheduled bbox exact=10/11] export at packages/d2ts/src/version-index.ts:8 (t=9061, 10 atoms) |
| 4.2 | 8859 | 1292 | -7567 | 1.00 | early | d2mini src + operators listings | fs-only |
| 4.3 | 8937 | 4518 | -4419 | 1.00 | early | d2ql src + query-builder listings | fs-only |
| 5.3 | 9955 | 2316 | -7639 | 1.00 | early | Electric adapter entry points | [scheduled bbox exact=10/16] export at packages/d2ts/src/electric/index.ts:40 (t=2316, 18 atoms) |

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
