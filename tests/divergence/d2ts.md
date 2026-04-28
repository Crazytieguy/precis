scores: Sim=0.337 Reached=12/41 Early=3 Late=9 Partial=7 Missing=22 Used=9457/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (w×gap=0.72), 10 wrong-slice/granularity (w×gap=2.17), 13 no-discovered (w×gap=1.37)
Secondary intervention: split wrong-slice batches for 10 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 2 discovered-unscheduled
Top rows: 1.6, 2.9, 1.1, 1.7, 1.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 10 | 2.17 | 8/8/10 | nearby candidates have low exact atom overlap | 1.6, 2.9, 1.1, 1.7, 1.5, ... |
| add walker candidates for no-discovered rows | 13 | 1.37 | 1/6/13 | NS rows have no discovered line candidate | 2.7, 3.3, 3.4, 3.5, 3.6, ... |
| tune ranking for discovered unscheduled candidates | 2 | 0.72 | 2/2/2 | high-overlap candidates fit but did not win, exact total=32/35 | 2.8, 2.12 |

Tiers: 1=3/7 reached, 4 partial, 0 missing, avg=0.78; 2=6/12 reached, 1 partial, 5 missing, avg=0.57; 3=1/15 reached, 1 partial, 13 missing, avg=0.11; 4=1/4 reached, 0 partial, 3 missing, avg=0.43; 5=1/3 reached, 1 partial, 1 missing, avg=0.55

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 10 | 3 | 7 | 0 | walker granularity / wrong slice |
| no discovered candidate | 13 | 13 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 4 | 4 | 0 | 0 | filesystem/listing value |
| timing-only | 12 | 0 | 0 | 12 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| discovered unscheduled | 2 | 0.72 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=19, unscheduled bbox=1, fs-only=8, no discovered candidate=13

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | low | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | low | 7 |
| unscheduled bbox | missing | low | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.8 | 1382 | — | — | 0.06 | missing | MultiSet method names (full catalog) | [scheduled bbox exact=1/16] export names surface in packages/d2ts/src/multiset.ts (t=3244, 2 atoms); better unscheduled exact=16/16: export at packages/d2ts/src/multiset.ts:9 (39 atoms, discovered unscheduled) |
| 2.12 | 2436 | — | — | 0.16 | missing | Top-level README — operator catalog with descriptions | [scheduled bbox exact=1/19] README.md section #4 (t=4104, 1 atoms); better unscheduled exact=16/19: README.md section #3 (16 atoms, discovered unscheduled) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 77 | — | — | 0.67 | partial | README one-liner | [scheduled bbox exact=2/3] README headline in README.md (t=182, 2 atoms) |
| 1.5 | 313 | — | — | 0.67 | partial | README — incremental + Electric pitch | [scheduled bbox exact=2/3] README.md section #0 (t=1012, 2 atoms) |
| 1.6 | 419 | — | — | 0.50 | partial | d2mini one-liner | [scheduled bbox exact=2/6] README headline in packages/d2mini/README.md (t=642, 2 atoms) |
| 1.7 | 494 | — | — | 0.60 | partial | d2ql one-liner | [scheduled bbox exact=2/5] README headline in packages/d2ql/README.md (t=491, 2 atoms) |
| 2.9 | 1537 | — | — | 0.29 | missing | D2 class + RootStreamBuilder method names | [scheduled bbox exact=4/17] export names surface in packages/d2ts/src/d2.ts (t=3566, 8 atoms); better unscheduled exact=13/17: export at packages/d2ts/src/d2.ts:15 (32 atoms, discovered unscheduled) |
| 2.11 | 2056 | — | — | 0.55 | partial | ID2 + IStreamBuilder shape | [scheduled bbox exact=11/22] export at packages/d2ts/src/types.ts:52 (t=4223, 11 atoms) |
| 3.1 | 2667 | — | — | 0.00 | missing | Antichain.create polymorphic constructor | [unscheduled bbox exact=15/19] export body at packages/d2ts/src/order.ts:138 body 142 (15 atoms, predecessor not scheduled: export at packages/d2ts/src/order.ts:138) |
| 3.2 | 2908 | — | — | 0.64 | partial | graph.ts class hierarchy (signatures only) | [scheduled bbox exact=4/22] export at packages/d2ts/src/graph.ts:144 (t=7729, 14 atoms); better unscheduled exact=7/22: export at packages/d2ts/src/graph.ts:171 (20 atoms, discovered unscheduled) |
| 5.1 | 9394 | — | — | 0.54 | partial | d2ql Query interface + compileQuery signature | [scheduled bbox exact=7/28] export names surface #2 in packages/d2ql/src/schema.ts (t=6473, 8 atoms); better unscheduled exact=12/28: export at packages/d2ql/src/schema.ts:207 (16 atoms, discovered unscheduled) |
| 5.2 | 9687 | — | — | 0.11 | missing | d2ql function + aggregate + comparator names | [scheduled bbox exact=4/31] export names surface in packages/d2ql/src/schema.ts (t=4594, 10 atoms); better unscheduled exact=13/31: export at packages/d2ql/src/schema.ts:105 (13 atoms, discovered unscheduled) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.7 | 1259 | — | — | 0.00 | missing | operators/index.ts (operator re-exports) | no discovered line candidate |
| 3.3 | 3322 | — | — | 0.00 | missing | LinearUnaryOperator base class (operators/base.ts) | no discovered line candidate |
| 3.4 | 3747 | — | — | 0.00 | missing | Keying operators (keyBy/unkey/rekey) | no discovered line candidate |
| 3.5 | 3988 | — | — | 0.00 | missing | map() operator: canonical factory pattern | no discovered line candidate |
| 3.6 | 4482 | — | — | 0.00 | missing | ConsolidateOperator: the "versions complete" pattern | no discovered line candidate |
| 3.8 | 5362 | — | — | 0.00 | missing | ReduceOperator: class + ingest loop | no discovered line candidate |
| 3.9 | 6080 | — | — | 0.00 | missing | ReduceOperator: finished-versions + delta computation | no discovered line candidate |
| 3.10 | 6843 | — | — | 0.00 | missing | JoinOperator: class shape + delta-join kernel | no discovered line candidate |
| 3.11 | 7254 | — | — | 0.00 | missing | Join factory + JoinType + named variants | no discovered line candidate |
| 3.12 | 7601 | — | — | 0.00 | missing | iterate(): scope/feedback wiring | no discovered line candidate |
| 3.13 | 7651 | — | — | 0.00 | missing | iterate.ts class names | no discovered line candidate |
| 3.14 | 8104 | — | — | 0.00 | missing | groupBy + aggregate function names | no discovered line candidate |
| 3.15 | 8641 | — | — | 0.00 | missing | topK + orderBy + indexed variants (signatures) | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.3 | 706 | — | — | 0.00 | missing | Operator catalog (operators/ filenames) | fs-only |
| 4.1 | 8722 | — | — | 0.33 | missing | sqlite/ + electric/ subdir listings | fs-only |
| 4.2 | 8859 | — | — | 0.28 | missing | d2mini src + operators listings | fs-only |
| 4.4 | 9068 | — | — | 0.10 | missing | Examples + benchmark listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 131 | 54 | -77 | 1.00 | early | Top-level workspace listing | fs-only |
| 1.3 | 153 | 266 | +113 | 1.00 | late | Packages directory listing | fs-only |
| 1.4 | 179 | 1095 | +916 | 1.00 | late | pnpm workspace globs | [scheduled bbox exact=3/3] plaintext config pnpm-workspace.yaml (t=1095, 3 atoms) |
| 2.1 | 538 | 1684 | +1146 | 1.00 | late | d2ts src layout | fs-only |
| 2.2 | 608 | 3119 | +2511 | 1.00 | late | d2ts package re-exports (`index.ts`) | [scheduled bbox exact=6/6] imports in packages/d2ts/src/index.ts (t=3119, 6 atoms) |
| 2.4 | 744 | 3172 | +2428 | 1.00 | late | Version + Antichain class names + factory `v(…)` | [scheduled bbox exact=4/4] export names surface in packages/d2ts/src/order.ts (t=3172, 7 atoms) |
| 2.5 | 919 | 3479 | +2560 | 0.94 | late | Core types: KeyValue, MessageType, Message | [scheduled bbox exact=9/18] export at packages/d2ts/src/types.ts:14 (t=3479, 9 atoms) |
| 2.6 | 1019 | 2599 | +1580 | 0.90 | late | Core types: DataMessage, FrontierMessage, PipedOperator | [scheduled bbox exact=6/10] export names surface in packages/d2ts/src/types.ts (t=1867, 16 atoms) |
| 2.10 | 1760 | 3330 | +1570 | 0.90 | late | Operator interfaces: IOperator, IDifferenceStreamReader/Writer | [scheduled bbox exact=8/20] export at packages/d2ts/src/types.ts:43 (t=3330, 8 atoms) |
| 3.7 | 4670 | 8859 | +4189 | 1.00 | late | Index<K,V> trace: signatures + interface | [scheduled bbox exact=10/11] export at packages/d2ts/src/version-index.ts:8 (t=8859, 10 atoms) |
| 4.3 | 8937 | 4316 | -4621 | 1.00 | early | d2ql src + query-builder listings | fs-only |
| 5.3 | 9955 | 2555 | -7400 | 1.00 | early | Electric adapter entry points | [scheduled bbox exact=10/16] export at packages/d2ts/src/electric/index.ts:40 (t=2555, 18 atoms) |

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
| 216 | 1.00 | 216 | 8113 | package dependencies in package.json |
| 201 | 1.00 | 201 | 4795 | export names surface in packages/d2ql/src/types.ts |
| 183 | 0.95 | 193 | 5104 | export names surface #1 in packages/d2ql/src/schema.ts |
| 167 | 1.00 | 167 | 848 | headings outline in README.md |
| 165 | 0.88 | 188 | 2915 | export body at packages/d2ts/src/electric/index.ts:328 body 333 |
| 164 | 0.87 | 188 | 4594 | export names surface in packages/d2ql/src/schema.ts |
| 160 | 1.00 | 160 | 2306 | export at packages/d2ts/src/electric/index.ts:222 |
| 136 | 1.00 | 136 | 1430 | package scripts in package.json |
| 121 | 1.00 | 121 | 2146 | export at packages/d2ts/src/electric/index.ts:103 |
| 118 | 1.00 | 118 | 3918 | imports in packages/d2ts-benchmark/src/index.ts |
| 2890 | — | — | — | +40 more rows |
