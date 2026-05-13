scores: Score(3000)=0.520 ns_rows≤3K=17/38 (reached=5 partial=1 missing=11)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 116 | 0.699 | 0.332 | 0.706 | 0.482 | 994 |
| 1442 | 151 | 0.697 | 0.312 | 0.643 | 0.466 | 1190 |
| 2080 | 206 | 0.713 | 0.302 | 0.602 | 0.464 | 2065 |
| 3000 | 263 | 0.794 | 0.340 | 0.825 | 0.520 | 2972 |
| 4327 | 386 | 0.762 | 0.234 | 0.685 | 0.422 | 4286 |
| 6240 | 490 | 0.763 | 0.362 | 0.655 | 0.526 | 6239 |
| 9000 | 743 | 0.766 | 0.390 | 0.674 | 0.546 | 8979 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 28 | 1.97 | 1.91 | 1.60 | nearby candidates have low exact atom overlap | 2.4, 2.10, 4.2, 2.8, 2.1, ... |
| tune ranking for high-overlap unscheduled candidates | 2 | 0.12 | 0.12 | 0.12 | high-overlap candidates not in the schedule by T_max, exact total=26/26 | 2.3, 3.1 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| package identity in package.json | 1 | 312 | 312 | 312 | off_3k=312 | package identity in package.json |
| headings outline in readme.md | 1 | 0 | 131 | 131 | off_3k=131 | headings outline in readme.md |
| package scripts in package.json | 1 | 103 | 103 | 103 | off_3k=103 | package scripts in package.json |
| json config tsconfig.json | 1 | 0 | 82 | 82 | off_3k=82 | json config tsconfig.json |
| readme.md section #<n> | 1 | 0 | 81 | 269 | off_3k=81 | readme.md section #1 |

Top missed paths (NS rows ≤ 3K): source/index.ts (3 rows, 75 atoms), source/core/constants.ts (2 rows, 49 atoms), test (1 row, 21 atoms), readme.md (4 rows, 15 atoms), package.json (1 row, 15 atoms), source/types/ky.ts (1 row, 12 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.3 | 1356 | 0.08 | missing | KyInstance: every member's signature (location batch) | [scheduled bbox exact=1/12] export names surface in source/types/ky.ts (t=1142, 2 atoms); better unscheduled exact=12/12: export at source/types/ky.ts:5 (140 atoms, too expensive at final margin) |
| 3.1 | 3855 | 0.07 | missing | KyOptions: every option's name + type signature (location batch) | [scheduled bbox exact=1/14] export names surface in source/types/options.ts (t=1521, 2 atoms); better unscheduled exact=14/14: export at source/types/options.ts:33 (207 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.2 | 79 | 0.00 | missing | Readme one-line tagline | [scheduled same-file] headings outline in readme.md (t=2730, 53 atoms) |
| 1.4 | 204 | 0.00 | missing | Readme target environments + 'no deps' note | [scheduled same-file] headings outline in readme.md (t=2730, 53 atoms) |
| 1.8 | 714 | 0.73 | partial | package.json identity (name, version, description, exports, engines) | [scheduled bbox exact=4/15] package identity in package.json (t=791, 12 atoms) |
| 2.1 | 1051 | 0.00 | missing | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | [scheduled bbox exact=10/15] export at source/core/constants.ts:46 (t=9376, 10 atoms) |
| 2.2 | 1133 | 0.00 | missing | Readme Usage block (canonical example) | [unscheduled bbox exact=6/8] readme.md section #2 (6 atoms, too expensive at final margin) |
| 2.4 | 1768 | 0.00 | missing | source/index.ts named-export block | [scheduled same-file] imports in source/index.ts (t=2165, 6 atoms) |
| 2.5 | 1953 | 0.44 | missing | source/index.ts imports + createInstance signature + default export | [scheduled bbox exact=6/16] imports in source/index.ts (t=2165, 6 atoms) |
| 2.6 | 2246 | 0.00 | missing | source/index.ts createInstance body | [scheduled same-file] imports in source/index.ts (t=2165, 6 atoms) |
| 2.7 | 2473 | 0.00 | missing | Default ky() body-method behavior + body shortcuts list | [unscheduled bbox exact=1/3] readme.md section #5 (1 atoms, discovered unscheduled) |
| 2.8 | 2901 | 0.00 | missing | core/constants.ts: feature-detection flags | [scheduled bbox exact=26/34] export at source/core/constants.ts:4 (t=9678, 26 atoms) |
| 2.9 | 3186 | 0.00 | missing | ForceRetryOptions type + RetryMarker + retry() factory signature | [scheduled bbox exact=4/28] export names surface in source/core/constants.ts (t=9227, 4 atoms); better unscheduled exact=13/28: export at source/core/constants.ts:68 (63 atoms, too expensive at final margin) |
| 2.10 | 3658 | 0.00 | missing | kyOptionKeys + vendor/request option registries | [scheduled bbox exact=16/44] export at source/core/constants.ts:265 (t=5553, 16 atoms) |
| 3.2 | 4024 | 0.14 | missing | RetryOptions: every field + ShouldRetryState (location batch) | [scheduled bbox exact=3/14] export at source/types/retry.ts:3 (t=4000, 10 atoms); better unscheduled exact=11/14: export at source/types/retry.ts:15 (117 atoms, too expensive at final margin) |
| 3.3 | 4324 | 0.00 | missing | Hooks types: every state + hook type alias (location batch) | [scheduled bbox exact=11/23] export names surface in source/types/hooks.ts (t=6136, 18 atoms) |
| 3.4 | 4587 | 0.50 | missing | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | [scheduled bbox exact=9/18] export names surface in source/types/options.ts (t=1521, 12 atoms) |
| 3.5 | 5127 | 0.09 | missing | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | [scheduled bbox exact=11/37] export at source/types/options.ts:358 (t=7726, 11 atoms) |
| 3.6 | 5252 | 0.09 | missing | ResponsePromise type signature (no examples) | [scheduled bbox exact=2/12] export names surface in source/types/ResponsePromise.ts (t=1158, 2 atoms); better unscheduled exact=8/12: export at source/types/ResponsePromise.ts:6 (32 atoms, too expensive at final margin) |
| 4.2 | 6709 | 0.00 | missing | #calculateRetryDelay — full retry-decision logic | [scheduled bbox exact=2/73] export at source/core/Ky.ts:33 (t=5934, 2 atoms); better unscheduled exact=57/73: export body at source/core/Ky.ts:33 body 35 (57 atoms, too expensive at final margin) |
| 5.1 | 6904 | 0.00 | missing | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | [scheduled bbox exact=8/12] export names surface in source/utils/type-guards.ts (t=3323, 8 atoms) |
| 5.2 | 7223 | 0.39 | missing | HTTPError + TimeoutError class bodies | [scheduled bbox exact=9/28] export body at source/errors/HTTPError.ts:5 body 11 (t=3894, 9 atoms) |
| 5.3 | 7546 | 0.32 | missing | ForceRetryError class body + NonError signature | [scheduled bbox exact=12/28] export body at source/errors/ForceRetryError.ts:8 body 15 (t=4868, 12 atoms) |
| 6.1 | 7855 | 0.10 | missing | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | [scheduled bbox exact=2/21] export at source/utils/normalize.ts:5 (t=1999, 2 atoms) |
| 6.2 | 8679 | 0.00 | missing | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | [scheduled bbox exact=8/67] export at source/utils/merge.ts:38 (t=4209, 8 atoms); better unscheduled exact=48/67: export body at source/utils/merge.ts:86 body 87 (59 atoms, too expensive at final margin) |
| 6.3 | 8901 | 0.25 | missing | timeout.ts body | [scheduled bbox exact=15/24] export body at source/utils/timeout.ts:9 body 15 (t=3228, 15 atoms) |
| 6.4 | 9072 | 0.67 | missing | options.ts utils + body.ts streaming exports + small util one-liners | [scheduled bbox exact=0/9] export body at source/utils/body.ts:90 body 91 (t=8431, 22 atoms) |
| 7.1 | 9532 | 0.00 | missing | test/main.ts: representative test names (truncated) | [unscheduled same-file] imports in test/main.ts (7 atoms, discovered unscheduled) |
| 7.2 | 9911 | 0.00 | missing | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names | [unscheduled same-file] imports in test/memory-leak.ts (5 atoms, discovered unscheduled) |
| 7.3 | 9987 | 0.00 | missing | test/retry.ts: 4 most-distinctive test names (truncated) | [unscheduled same-file] imports in test/retry.ts (5 atoms, discovered unscheduled) |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.7 | 545 | 0.00 | missing | test/ tree (top-level + helpers/) | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.1 | 5804 | 0.04 | missing | Ky class member declarations + every method signature (location batch) | [scheduled bbox exact=22/23] export at source/core/Ky.ts:33 (t=5934, 34 atoms) |

Top wasted paths (off-NS at 3K): package.json (415t, 2 batches), source/types/options.ts (331t, 1 batch), readme.md (212t, 2 batches), source/utils/body.ts (87t, 1 batch), tsconfig.json (82t, 1 batch), source/types/common.ts (78t, 1 batch), source/errors/ForceRetryError.ts (59t, 1 batch), source/errors/HTTPError.ts (52t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 331 | 1.00 | 32 | 331 | 1190 | export names surface in source/types/options.ts |
| 312 | 0.85 | 312 | 366 | 425 | package identity in package.json |
| 131 | 0.39 | 131 | 335 | 2395 | headings outline in readme.md |
| 103 | 1.00 | 103 | 103 | 994 | package scripts in package.json |
| 87 | 1.00 | 6 | 87 | 2972 | export names surface in source/utils/body.ts |
| 82 | 1.00 | 82 | 82 | 2197 | json config tsconfig.json |
| 81 | 1.00 | 81 | 81 | 2846 | readme.md section #1 |
| 78 | 1.00 | 0 | 78 | 2296 | export names surface in source/types/common.ts |
| 59 | 1.00 | 0 | 59 | 791 | export at source/errors/ForceRetryError.ts:8 |
| 52 | 1.00 | 0 | 52 | 373 | export at source/errors/HTTPError.ts:5 |
