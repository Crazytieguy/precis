scores: Score(3000)=0.521 ns_rows≤3K=17/38 (reached=5 partial=1 missing=11)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 116 | 0.699 | 0.332 | 0.482 | 994 |
| 1442 | 151 | 0.697 | 0.312 | 0.466 | 1190 |
| 2080 | 206 | 0.715 | 0.302 | 0.465 | 2077 |
| 3000 | 263 | 0.797 | 0.340 | 0.521 | 2992 |
| 4327 | 386 | 0.761 | 0.233 | 0.421 | 4209 |
| 6240 | 490 | 0.756 | 0.322 | 0.493 | 6204 |
| 9000 | 743 | 0.760 | 0.421 | 0.566 | 8820 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (gap@3k=0.12), 24 wrong-slice/granularity (gap@3k=1.81), 3 no-discovered (gap@3k=0.08)
Secondary intervention: investigate 3 no-discovered rows
Top rows: 2.4, 2.10, 4.2, 2.8, 2.1, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 24 | 1.88 | 1.81 | 1.30 | nearby candidates have low exact atom overlap | 2.4, 2.10, 4.2, 2.8, 2.1, ... |
| tune ranking for high-overlap unscheduled candidates | 2 | 0.12 | 0.12 | 0.12 | high-overlap candidates not in the schedule by T_max, exact total=26/26 | 2.3, 3.1 |
| add walker candidates for no-discovered rows | 3 | 0.08 | 0.08 | 0.08 | NS rows have no discovered line candidate | 7.1, 7.2, 7.3 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | value/ranking |
| wrong-slice / granularity | 24 | 22 | 2 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 1 | 1 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 2 | 0.12 | tune ranking |

Candidate hint kinds: scheduled bbox=21, unscheduled bbox=2, scheduled same-file=4, fs-only=1, no discovered candidate=3 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 18 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | low | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.3 | 1356 | 0.08 | 0.04 | missing | KyInstance: every member's signature (location batch) | [scheduled bbox exact=1/12] export names surface in source/types/ky.ts (t=1142, 2 atoms); better unscheduled exact=12/12: export at source/types/ky.ts:5 (140 atoms, too expensive at final margin) |
| 3.1 | 3855 | 0.07 | 0.05 | missing | KyOptions: every option's name + type signature (location batch) | [scheduled bbox exact=1/14] export names surface in source/types/options.ts (t=1521, 2 atoms); better unscheduled exact=14/14: export at source/types/options.ts:33 (207 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.2 | 79 | 0.00 | 0.00 | missing | Readme one-line tagline | [scheduled same-file] headings outline in readme.md (t=2992, 53 atoms) |
| 1.4 | 204 | 0.00 | 0.00 | missing | Readme target environments + 'no deps' note | [scheduled same-file] headings outline in readme.md (t=2992, 53 atoms) |
| 1.8 | 714 | 0.73 | 0.90 | partial | package.json identity (name, version, description, exports, engines) | [scheduled bbox exact=4/15] package identity in package.json (t=791, 12 atoms) |
| 2.1 | 1051 | 0.00 | 0.00 | missing | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | [scheduled bbox exact=10/15] export at source/core/constants.ts:46 (t=8179, 10 atoms) |
| 2.2 | 1133 | 0.00 | 0.00 | missing | Readme Usage block (canonical example) | [unscheduled bbox exact=6/8] readme.md section #2 (6 atoms, discovered unscheduled) |
| 2.4 | 1768 | 0.00 | 0.00 | missing | source/index.ts named-export block | [scheduled same-file] imports in source/index.ts (t=2351, 6 atoms) |
| 2.5 | 1953 | 0.44 | 0.66 | missing | source/index.ts imports + createInstance signature + default export | [scheduled bbox exact=6/16] imports in source/index.ts (t=2351, 6 atoms) |
| 2.6 | 2246 | 0.00 | 0.00 | missing | source/index.ts createInstance body | [scheduled same-file] imports in source/index.ts (t=2351, 6 atoms) |
| 2.7 | 2473 | 0.00 | 0.00 | missing | Default ky() body-method behavior + body shortcuts list | [unscheduled bbox exact=1/3] readme.md section #5 (1 atoms, discovered unscheduled) |
| 2.8 | 2901 | 0.00 | 0.00 | missing | core/constants.ts: feature-detection flags | [scheduled bbox exact=26/34] export at source/core/constants.ts:4 (t=8720, 26 atoms) |
| 2.9 | 3186 | 0.00 | 0.00 | missing | ForceRetryOptions type + RetryMarker + retry() factory signature | [scheduled bbox exact=4/28] export names surface in source/core/constants.ts (t=8030, 4 atoms); better unscheduled exact=13/28: export at source/core/constants.ts:68 (63 atoms, too expensive at final margin) |
| 2.10 | 3658 | 0.00 | 0.00 | missing | kyOptionKeys + vendor/request option registries | [scheduled bbox exact=16/44] export at source/core/constants.ts:265 (t=4816, 16 atoms) |
| 3.2 | 4024 | 0.14 | 0.13 | missing | RetryOptions: every field + ShouldRetryState (location batch) | [scheduled bbox exact=3/14] export at source/types/retry.ts:3 (t=4893, 10 atoms); better unscheduled exact=11/14: export at source/types/retry.ts:15 (117 atoms, too expensive at final margin) |
| 3.3 | 4324 | 0.00 | 0.00 | missing | Hooks types: every state + hook type alias (location batch) | [scheduled bbox exact=11/23] export names surface in source/types/hooks.ts (t=4370, 18 atoms) |
| 3.4 | 4587 | 0.50 | 0.76 | missing | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | [scheduled bbox exact=9/18] export names surface in source/types/options.ts (t=1521, 12 atoms) |
| 3.5 | 5127 | 0.09 | 0.31 | missing | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | [scheduled bbox exact=11/37] export at source/types/options.ts:358 (t=7628, 11 atoms) |
| 3.6 | 5252 | 0.09 | 0.17 | missing | ResponsePromise type signature (no examples) | [scheduled bbox exact=2/12] export names surface in source/types/ResponsePromise.ts (t=1158, 2 atoms); better unscheduled exact=8/12: export at source/types/ResponsePromise.ts:6 (32 atoms, too expensive at final margin) |
| 4.2 | 6709 | 0.00 | 0.00 | missing | #calculateRetryDelay — full retry-decision logic | [scheduled bbox exact=2/73] export at source/core/Ky.ts:33 (t=6545, 2 atoms); better unscheduled exact=57/73: export body at source/core/Ky.ts:33 body 35 (57 atoms, too expensive at final margin) |
| 5.1 | 6904 | 0.67 | 0.99 | partial | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | [scheduled bbox exact=8/12] export names surface in source/utils/type-guards.ts (t=2446, 8 atoms) |
| 5.2 | 7223 | 0.39 | 0.51 | missing | HTTPError + TimeoutError class bodies | [scheduled bbox exact=9/28] export body at source/errors/HTTPError.ts:5 body 11 (t=4180, 9 atoms) |
| 5.3 | 7546 | 0.32 | 0.38 | missing | ForceRetryError class body + NonError signature | [scheduled bbox exact=12/28] export body at source/errors/ForceRetryError.ts:8 body 15 (t=5732, 12 atoms) |
| 6.1 | 7855 | 0.10 | 0.18 | missing | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | [scheduled bbox exact=2/21] export at source/utils/normalize.ts:5 (t=1999, 2 atoms) |
| 6.2 | 8679 | 0.00 | 0.00 | missing | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | [scheduled bbox exact=8/67] export at source/utils/merge.ts:38 (t=3317, 8 atoms); better unscheduled exact=48/67: export body at source/utils/merge.ts:86 body 87 (59 atoms, too expensive at final margin) |
| 6.3 | 8901 | 0.25 | 0.34 | missing | timeout.ts body | [scheduled bbox exact=15/24] export body at source/utils/timeout.ts:9 body 15 (t=3612, 15 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 7.1 | 9532 | 0.00 | 0.00 | missing | test/main.ts: representative test names (truncated) | no discovered line candidate |
| 7.2 | 9911 | 0.00 | 0.00 | missing | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names | no discovered line candidate |
| 7.3 | 9987 | 0.00 | 0.00 | missing | test/retry.ts: 4 most-distinctive test names (truncated) | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.7 | 545 | 0.00 | 0.00 | missing | test/ tree (top-level + helpers/) | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 5804 | 0.04 | 0.02 | missing | Ky class member declarations + every method signature (location batch) | [scheduled bbox exact=22/23] export at source/core/Ky.ts:33 (t=6545, 34 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 347 | export at source/types/hooks.ts:<n> |
| 5 | 332 | readme.md section #<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 312 | 0.85 | 366 | 791 | package identity in package.json |
| 131 | 0.39 | 335 | 2992 | headings outline in readme.md |
| 103 | 1.00 | 103 | 1097 | package scripts in package.json |
| 82 | 1.00 | 82 | 2640 | json config tsconfig.json |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 371 | 0.93 | 400 | 7209 | package dependencies in package.json |
| 255 | 1.00 | 255 | 9580 | export body at source/utils/options.ts:4 body 8 |
| 229 | 1.00 | 229 | 9325 | export body at source/utils/normalize.ts:28 body 29 |
| 212 | 1.00 | 212 | 9032 | export body at source/utils/body.ts:90 body 91 |
| 191 | 1.00 | 191 | 8370 | export body at source/utils/options.ts:30 body 31 |
| 155 | 1.00 | 155 | 3866 | export body at source/utils/delay.ts:9 body 13 |
| 148 | 0.92 | 161 | 5120 | export body at source/errors/NonError.ts:6 body 11 |
| 140 | 1.00 | 140 | 6763 | export body at source/utils/merge.ts:16 body 17 |
| 123 | 1.00 | 123 | 6032 | export body at source/utils/body.ts:119 body 120 |
| 112 | 1.00 | 112 | 4044 | json config tsconfig.dist.json |
| 1299 | — | — | — | +19 more rows |
