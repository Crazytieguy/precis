scores: Sim=0.298 Reached=10/41 Early=2 Late=6 Partial=4 Missing=27 Used=9963/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (w×gap=0.77), 12 wrong-slice/granularity (w×gap=3.94), 13 no-discovered (w×gap=1.37)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 0 discovered-unscheduled
Top rows: 2.5, 2.6, 2.10, 1.6, 2.11, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 12 | 3.94 | 10/10/12 | nearby candidates have low exact atom overlap | 2.5, 2.6, 2.10, 1.6, 2.11, ... |
| add walker candidates for no-discovered rows | 13 | 1.37 | 1/6/13 | NS rows have no discovered line candidate | 2.7, 3.3, 3.4, 3.5, 3.6, ... |
| free final budget / demote late waste | 2 | 0.77 | 2/2/2 | high-overlap candidates exceed final remaining budget, exact total=35/35 | 2.8, 2.12 |

Tiers: 1=3/7 reached, 4 partial, 0 missing, avg=0.78; 2=3/12 reached, 0 partial, 9 missing, avg=0.28; 3=2/15 reached, 0 partial, 13 missing, avg=0.13; 4=1/4 reached, 0 partial, 3 missing, avg=0.43; 5=1/3 reached, 0 partial, 2 missing, avg=0.38

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 12 | 8 | 4 | 0 | walker granularity / wrong slice |
| no discovered candidate | 13 | 13 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 4 | 4 | 0 | 0 | filesystem/listing value |
| timing-only | 8 | 0 | 0 | 8 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.77 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=12, unscheduled bbox=7, fs-only=7, no discovered candidate=13

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | none | 1 |
| scheduled bbox | late | none | 1 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | full | 2 |
| scheduled bbox | missing | low | 3 |
| scheduled bbox | partial | low | 4 |
| unscheduled bbox | missing | none | 1 |
| unscheduled bbox | missing | low | 5 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.8 | 1382 | — | — | 0.06 | missing | MultiSet method names (full catalog) | [scheduled bbox exact=1/16] export names surface in packages/d2ts/src/multiset.ts (t=3206, 2 atoms); better unscheduled exact=16/16: export at packages/d2ts/src/multiset.ts:9 (39 atoms, too expensive at final margin) |
| 2.12 | 2436 | — | — | 0.00 | missing | Top-level README — operator catalog with descriptions | [unscheduled bbox exact=19/19] README.md section #2 (19 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 77 | — | — | 0.67 | partial | README one-liner | [scheduled bbox exact=2/3] README headline in README.md (t=182, 2 atoms) |
| 1.5 | 313 | — | — | 0.67 | partial | README — incremental + Electric pitch | [scheduled bbox exact=2/3] README.md section #0 (t=1003, 2 atoms) |
| 1.6 | 419 | — | — | 0.50 | partial | d2mini one-liner | [scheduled bbox exact=2/6] README headline in packages/d2mini/README.md (t=633, 2 atoms) |
| 1.7 | 494 | — | — | 0.60 | partial | d2ql one-liner | [scheduled bbox exact=2/5] README headline in packages/d2ql/README.md (t=482, 2 atoms) |
| 2.5 | 919 | — | — | 0.00 | missing | Core types: KeyValue, MessageType, Message | [unscheduled bbox exact=9/18] export at packages/d2ts/src/types.ts:14 (9 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/types.ts) |
| 2.6 | 1019 | — | — | 0.00 | missing | Core types: DataMessage, FrontierMessage, PipedOperator | [unscheduled bbox exact=0/10] export at packages/d2ts/src/types.ts:64 (50 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/types.ts) |
| 2.9 | 1537 | — | — | 0.29 | missing | D2 class + RootStreamBuilder method names | [scheduled bbox exact=4/17] export names surface in packages/d2ts/src/d2.ts (t=3271, 8 atoms); better unscheduled exact=13/17: export at packages/d2ts/src/d2.ts:15 (32 atoms, too expensive at final margin) |
| 2.10 | 1760 | — | — | 0.00 | missing | Operator interfaces: IOperator, IDifferenceStreamReader/Writer | [unscheduled bbox exact=8/20] export at packages/d2ts/src/types.ts:43 (8 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/types.ts) |
| 2.11 | 2056 | — | — | 0.00 | missing | ID2 + IStreamBuilder shape | [unscheduled bbox exact=11/22] export at packages/d2ts/src/types.ts:52 (11 atoms, predecessor not scheduled: export names surface in packages/d2ts/src/types.ts) |
| 3.1 | 2667 | — | — | 0.00 | missing | Antichain.create polymorphic constructor | [unscheduled bbox exact=15/19] export body at packages/d2ts/src/order.ts:138 (15 atoms, predecessor not scheduled: export at packages/d2ts/src/order.ts:138) |
| 5.1 | 9394 | — | — | 0.14 | missing | d2ql Query interface + compileQuery signature | [scheduled bbox exact=4/28] export at packages/d2ql/src/compiler.ts:16 (t=9276, 4 atoms); better unscheduled exact=12/28: export at packages/d2ql/src/schema.ts:207 (16 atoms, predecessor not scheduled: export names surface in packages/d2ql/src/schema.ts) |
| 5.2 | 9687 | — | — | 0.00 | missing | d2ql function + aggregate + comparator names | [unscheduled bbox exact=13/31] export at packages/d2ql/src/schema.ts:105 (13 atoms, predecessor not scheduled: export names surface in packages/d2ql/src/schema.ts) |

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
| 1.3 | 153 | 257 | +104 | 1.00 | late | Packages directory listing | fs-only |
| 1.4 | 179 | 1086 | +907 | 1.00 | late | pnpm workspace globs | [scheduled bbox exact=3/3] plaintext config pnpm-workspace.yaml (t=1086, 3 atoms) |
| 2.1 | 538 | 1983 | +1445 | 1.00 | late | d2ts src layout | fs-only |
| 2.2 | 608 | 3081 | +2473 | 1.00 | late | d2ts package re-exports (`index.ts`) | [scheduled bbox exact=6/6] imports in packages/d2ts/src/index.ts (t=3081, 6 atoms) |
| 2.4 | 744 | 3134 | +2390 | 1.00 | late | Version + Antichain class names + factory `v(…)` | [scheduled bbox exact=0/4] export body at packages/d2ts/src/order.ts:10 (t=8163, 8 atoms); better unscheduled exact=1/4: export at packages/d2ts/src/order.ts:138 (36 atoms, too expensive at final margin) |
| 3.2 | 2908 | 7981 | +5073 | 1.00 | late | graph.ts class hierarchy (signatures only) | [scheduled bbox exact=7/22] export at packages/d2ts/src/graph.ts:171 (t=7981, 20 atoms) |
| 5.3 | 9955 | 2671 | -7284 | 1.00 | early | Electric adapter entry points | [scheduled bbox exact=0/16] export body at packages/d2ts/src/electric/index.ts:222 (t=6360, 76 atoms) |

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
| 3799 | — | — | — | +48 more rows |
