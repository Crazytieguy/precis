scores: Sim=0.419 Reached=13/38 Early=3 Late=6 Partial=6 Missing=19 Used=9233/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (w×gap=0.60), 19 wrong-slice/granularity (w×gap=4.15), 3 no-discovered (w×gap=0.02)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 0 discovered-unscheduled
Top rows: 1.2, 1.4, 2.2, 2.4, 2.6, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 19 | 4.15 | 7/13/19 | nearby candidates have low exact atom overlap | 1.2, 1.4, 2.2, 2.4, 2.6, ... |
| free final budget / demote late waste | 2 | 0.60 | 1/2/2 | high-overlap candidates exceed final remaining budget, exact total=26/26 | 2.3, 3.1 |
| add walker candidates for no-discovered rows | 3 | 0.02 | 0/0/3 | NS rows have no discovered line candidate | 7.1, 7.2, 7.3 |

Tiers: 1=6/9 reached, 0 partial, 3 missing, avg=0.65; 2=2/10 reached, 1 partial, 7 missing, avg=0.33; 3=2/7 reached, 2 partial, 3 missing, avg=0.53; 4=1/2 reached, 0 partial, 1 missing, avg=0.49; 5=0/3 reached, 3 partial, 0 missing, avg=0.72; 6=2/4 reached, 0 partial, 2 missing, avg=0.53; 7=0/3 reached, 0 partial, 3 missing, avg=0.00

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 19 | 13 | 6 | 0 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 12 | 0 | 0 | 12 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.60 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=26, unscheduled bbox=2, scheduled same-file=4, fs-only=2, no discovered candidate=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | early | none | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | low | 9 |
| scheduled bbox | partial | low | 6 |
| unscheduled bbox | missing | low | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.3 | 1356 | — | — | 0.08 | missing | KyInstance: every member's signature (location batch) | [scheduled bbox exact=1/12] export names surface in source/types/ky.ts (t=1142, 2 atoms); better unscheduled exact=12/12: export at source/types/ky.ts:5 (140 atoms, too expensive at final margin) |
| 3.1 | 3855 | — | — | 0.07 | missing | KyOptions: every option's name + type signature (location batch) | [scheduled bbox exact=1/14] export names surface in source/types/options.ts (t=1521, 2 atoms); better unscheduled exact=14/14: export at source/types/options.ts:33 (207 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 79 | — | — | 0.00 | missing | Readme one-line tagline | [scheduled same-file] headings outline in readme.md (t=2992, 53 atoms) |
| 1.4 | 204 | — | — | 0.00 | missing | Readme target environments + 'no deps' note | [scheduled same-file] headings outline in readme.md (t=2992, 53 atoms) |
| 2.2 | 1133 | — | — | 0.00 | missing | Readme Usage block (canonical example) | [unscheduled bbox exact=6/8] readme.md section #2 (6 atoms, discovered unscheduled) |
| 2.4 | 1768 | — | — | 0.00 | missing | source/index.ts named-export block | [scheduled same-file] imports in source/index.ts (t=2351, 6 atoms) |
| 2.5 | 1953 | — | — | 0.44 | missing | source/index.ts imports + createInstance signature + default export | [scheduled bbox exact=6/16] imports in source/index.ts (t=2351, 6 atoms) |
| 2.6 | 2246 | — | — | 0.00 | missing | source/index.ts createInstance body | [scheduled same-file] imports in source/index.ts (t=2351, 6 atoms) |
| 2.7 | 2473 | — | — | 0.00 | missing | Default ky() body-method behavior + body shortcuts list | [unscheduled bbox exact=2/3] readme.md section #3 (2 atoms, discovered unscheduled) |
| 2.9 | 3186 | — | — | 0.29 | missing | ForceRetryOptions type + RetryMarker + retry() factory signature | [scheduled bbox exact=4/28] export names surface in source/core/constants.ts (t=7782, 4 atoms); better unscheduled exact=13/28: export at source/core/constants.ts:68 (63 atoms, too expensive at final margin) |
| 2.10 | 3658 | — | — | 0.77 | partial | kyOptionKeys + vendor/request option registries | [scheduled bbox exact=16/44] export at source/core/constants.ts:265 (t=4684, 16 atoms) |
| 3.2 | 4024 | — | — | 0.29 | missing | RetryOptions: every field + ShouldRetryState (location batch) | [scheduled bbox exact=3/14] export at source/types/retry.ts:3 (t=4761, 10 atoms); better unscheduled exact=11/14: export at source/types/retry.ts:15 (117 atoms, too expensive at final margin) |
| 3.3 | 4324 | — | — | 0.78 | partial | Hooks types: every state + hook type alias (location batch) | [scheduled bbox exact=11/23] export names surface in source/types/hooks.ts (t=4238, 18 atoms) |
| 3.5 | 5127 | — | — | 0.71 | partial | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | [scheduled bbox exact=11/37] export at source/types/options.ts:358 (t=7426, 11 atoms) |
| 3.6 | 5252 | — | — | 0.09 | missing | ResponsePromise type signature (no examples) | [scheduled bbox exact=2/12] export names surface in source/types/ResponsePromise.ts (t=1158, 2 atoms); better unscheduled exact=8/12: export at source/types/ResponsePromise.ts:6 (32 atoms, discovered unscheduled) |
| 4.2 | 6709 | — | — | 0.01 | missing | #calculateRetryDelay — full retry-decision logic | [scheduled bbox exact=2/73] export at source/core/Ky.ts:33 (t=6380, 2 atoms); better unscheduled exact=57/73: export body at source/core/Ky.ts:33 (57 atoms, too expensive at final margin) |
| 5.1 | 6904 | — | — | 0.67 | partial | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | [scheduled bbox exact=8/12] export names surface in source/utils/type-guards.ts (t=2446, 8 atoms) |
| 5.2 | 7223 | — | — | 0.71 | partial | HTTPError + TimeoutError class bodies | [scheduled bbox exact=9/28] export body at source/errors/HTTPError.ts:5 (t=4048, 9 atoms) |
| 5.3 | 7546 | — | — | 0.79 | partial | ForceRetryError class body + NonError signature | [scheduled bbox exact=12/28] export body at source/errors/ForceRetryError.ts:8 (t=5600, 12 atoms) |
| 6.1 | 7855 | — | — | 0.10 | missing | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | [scheduled bbox exact=2/21] export at source/utils/normalize.ts:5 (t=1999, 2 atoms) |
| 6.2 | 8679 | — | — | 0.13 | missing | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | [scheduled bbox exact=8/67] export at source/utils/merge.ts:38 (t=3229, 8 atoms); better unscheduled exact=48/67: export body at source/utils/merge.ts:86 (59 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 7.1 | 9532 | — | — | 0.00 | missing | test/main.ts: representative test names (truncated) | no discovered line candidate |
| 7.2 | 9911 | — | — | 0.00 | missing | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names | no discovered line candidate |
| 7.3 | 9987 | — | — | 0.00 | missing | test/retry.ts: 4 most-distinctive test names (truncated) | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.7 | 545 | — | — | 0.00 | missing | test/ tree (top-level + helpers/) | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 163 | 2992 | +2829 | 1.00 | late | Readme top-level section headings (## only) | [scheduled bbox exact=9/10] headings outline in readme.md (t=2992, 52 atoms) |
| 1.5 | 331 | 244 | -87 | 0.92 | aligned | Readme benefits-over-fetch list | [scheduled bbox exact=11/12] README headline in readme.md (t=244, 11 atoms) |
| 1.6 | 447 | 1627 | +1180 | 1.00 | late | source/ tree (all immediate children + every subdir) | fs-only |
| 1.8 | 714 | 7007 | +6293 | 0.93 | late | package.json identity (name, version, description, exports, engines) | [scheduled bbox exact=4/15] package identity in package.json (t=791, 12 atoms) |
| 1.9 | 845 | 2992 | +2147 | 1.00 | late | Readme `## API` H3 location index | [scheduled bbox exact=13/13] headings outline in readme.md (t=2992, 20 atoms) |
| 2.1 | 1051 | 7933 | +6882 | 0.87 | late | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | [scheduled bbox exact=10/15] export at source/core/constants.ts:46 (t=7933, 10 atoms) |
| 2.8 | 2901 | 8426 | +5525 | 0.88 | late | core/constants.ts: feature-detection flags | [scheduled bbox exact=26/34] export at source/core/constants.ts:4 (t=8426, 26 atoms) |
| 3.4 | 4587 | 3800 | -787 | 0.83 | aligned | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | [scheduled bbox exact=9/18] export names surface in source/types/options.ts (t=1521, 12 atoms) |
| 3.7 | 5448 | 2098 | -3350 | 0.93 | early | KyRequest + KyResponse + common Primitive/LiteralUnion types | [scheduled bbox exact=6/14] export names surface in source/types/common.ts (t=2077, 6 atoms) |
| 4.1 | 5804 | 6380 | +576 | 0.96 | aligned | Ky class member declarations + every method signature (location batch) | [scheduled bbox exact=22/23] export at source/core/Ky.ts:33 (t=6380, 34 atoms) |
| 6.3 | 8901 | 3502 | -5399 | 0.88 | early | timeout.ts body | [scheduled bbox exact=15/24] export body at source/utils/timeout.ts:9 (t=3502, 15 atoms) |
| 6.4 | 9072 | 2221 | -6851 | 1.00 | early | options.ts utils + body.ts streaming exports + small util one-liners | [scheduled bbox exact=0/9] export body at source/utils/body.ts:90 (t=8638, 22 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 347 | export at source/types/hooks.ts:<n> |
| 3 | 218 | readme.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 371 | 0.93 | 400 | 7007 | package dependencies in package.json |
| 312 | 0.85 | 366 | 791 | package identity in package.json |
| 255 | 1.00 | 255 | 9186 | export body at source/utils/options.ts:4 |
| 229 | 1.00 | 229 | 8931 | export body at source/utils/normalize.ts:28 |
| 212 | 1.00 | 212 | 8638 | export body at source/utils/body.ts:90 |
| 191 | 1.00 | 191 | 8124 | export body at source/utils/options.ts:30 |
| 155 | 1.00 | 155 | 3734 | export body at source/utils/delay.ts:9 |
| 148 | 0.92 | 161 | 4988 | export body at source/errors/NonError.ts:6 |
| 140 | 1.00 | 140 | 6561 | export body at source/utils/merge.ts:16 |
| 131 | 0.39 | 335 | 2992 | headings outline in readme.md |
| 1605 | — | — | — | +21 more rows |
