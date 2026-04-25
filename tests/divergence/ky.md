scores: Sim=0.385 Reached=11/38 Early=2 Late=8 Partial=4 Missing=23 Used=9636/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 5 | 0 | 4 | 0.52 |
| 2 | 10 | 2 | 2 | 6 | 0.41 |
| 3 | 7 | 3 | 2 | 2 | 0.64 |
| 4 | 2 | 0 | 0 | 2 | 0.02 |
| 5 | 3 | 0 | 0 | 3 | 0.32 |
| 6 | 4 | 1 | 0 | 3 | 0.37 |
| 7 | 3 | 0 | 0 | 3 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 79 | — | — | 0.00 | missing | Readme one-line tagline |  |
| 1.3 | 163 | 4972 | +4809 | 0.80 | late | Readme top-level section headings (## only) | readme.md section #2 (t=2889, 28 atoms) |
| 1.4 | 204 | — | — | 0.00 | missing | Readme target environments + 'no deps' note |  |
| 1.5 | 331 | 593 | +262 | 0.92 | late | Readme benefits-over-fetch list | readme.md section #0 (t=593, 10 atoms) |
| 1.6 | 447 | 1727 | +1280 | 1.00 | late | source/ tree (all immediate children + every subdir) |  |
| 1.7 | 545 | — | — | 0.00 | missing | test/ tree (top-level + helpers/) |  |
| 1.8 | 714 | 6516 | +5802 | 0.93 | late | package.json identity (name, version, description, exports, engines) | package identity in package.json (t=1071, 12 atoms) |
| 1.9 | 845 | — | — | 0.00 | missing | Readme `## API` H3 location index |  |
| 2.1 | 1051 | 6977 | +5926 | 0.87 | late | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | export at source/core/constants.ts:46 (t=6977, 10 atoms) |
| 2.2 | 1133 | — | — | 0.75 | partial | Readme Usage block (canonical example) | readme.md section #2 (t=2889, 6 atoms) |
| 2.3 | 1356 | — | — | 0.08 | missing | KyInstance: every member's signature (location batch) | export names surface in source/types/ky.ts (t=1496, 2 atoms) |
| 2.4 | 1768 | — | — | 0.00 | missing | source/index.ts named-export block |  |
| 2.5 | 1953 | — | — | 0.44 | missing | source/index.ts imports + createInstance signature + default export | imports in source/index.ts (t=2480, 6 atoms) |
| 2.6 | 2246 | — | — | 0.00 | missing | source/index.ts createInstance body |  |
| 2.7 | 2473 | — | — | 0.00 | missing | Default ky() body-method behavior + body shortcuts list |  |
| 2.8 | 2901 | 7560 | +4659 | 0.88 | late | core/constants.ts: feature-detection flags | export at source/core/constants.ts:4 (t=7560, 26 atoms) |
| 2.9 | 3186 | — | — | 0.29 | missing | ForceRetryOptions type + RetryMarker + retry() factory signature | export names surface in source/core/constants.ts (t=6830, 5 atoms) |
| 2.10 | 3658 | — | — | 0.77 | partial | kyOptionKeys + vendor/request option registries | export at source/core/constants.ts:265 (t=7258, 16 atoms) |
| 3.1 | 3855 | — | — | 0.07 | missing | KyOptions: every option's name + type signature (location batch) | export names surface in source/types/options.ts (t=7891, 2 atoms) |
| 3.2 | 4024 | — | — | 0.29 | missing | RetryOptions: every field + ShouldRetryState (location batch) | export at source/types/retry.ts:3 (t=1689, 10 atoms) |
| 3.3 | 4324 | — | — | 0.78 | partial | Hooks types: every state + hook type alias (location batch) | export names surface in source/types/hooks.ts (t=3715, 18 atoms) |
| 3.4 | 4587 | 7957 | +3370 | 0.83 | late | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | export names surface in source/types/options.ts (t=7891, 12 atoms) |
| 3.5 | 5127 | 8677 | +3550 | 0.89 | late | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | export at source/types/options.ts:312 (t=8677, 34 atoms) |
| 3.6 | 5252 | — | — | 0.67 | partial | ResponsePromise type signature (no examples) | export at source/types/ResponsePromise.ts:6 (t=5780, 32 atoms) |
| 3.7 | 5448 | 2293 | -3155 | 0.93 | early | KyRequest + KyResponse + common Primitive/LiteralUnion types | export names surface in source/types/common.ts (t=2272, 6 atoms) |
| 4.1 | 5804 | — | — | 0.04 | missing | Ky class member declarations + every method signature (location batch) | export names surface in source/core/Ky.ts (t=1301, 2 atoms) |
| 4.2 | 6709 | — | — | 0.00 | missing | #calculateRetryDelay — full retry-decision logic |  |
| 5.1 | 6904 | — | — | 0.34 | missing | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | export doc at source/utils/type-guards.ts:49 (t=9464, 16 atoms) |
| 5.2 | 7223 | — | — | 0.29 | missing | HTTPError + TimeoutError class bodies | export at source/errors/HTTPError.ts:5 (t=1451, 6 atoms) |
| 5.3 | 7546 | — | — | 0.32 | missing | ForceRetryError class body + NonError signature | export at source/errors/ForceRetryError.ts:8 (t=2163, 7 atoms) |
| 6.1 | 7855 | — | — | 0.10 | missing | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | export names surface in source/utils/normalize.ts (t=2008, 2 atoms) |
| 6.2 | 8679 | — | — | 0.13 | missing | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | export at source/utils/merge.ts:38 (t=3281, 8 atoms) |
| 6.3 | 8901 | — | — | 0.25 | missing | timeout.ts body | export at source/utils/timeout.ts:9 (t=2104, 7 atoms) |
| 6.4 | 9072 | 6116 | -2956 | 1.00 | early | options.ts utils + body.ts streaming exports + small util one-liners | export at source/utils/body.ts:90 (t=5189, 24 atoms) |
| 7.1 | 9532 | — | — | 0.00 | missing | test/main.ts: representative test names (truncated) |  |
| 7.2 | 9911 | — | — | 0.00 | missing | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names |  |
| 7.3 | 9987 | — | — | 0.00 | missing | test/retry.ts: 4 most-distinctive test names (truncated) |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 587 | 0.96 | 611 | 4972 | readme.md section #5 |
| 352 | 0.88 | 400 | 6516 | package dependencies in package.json |
| 322 | 0.88 | 366 | 1071 | package identity in package.json |
| 280 | 0.76 | 367 | 8677 | export at source/types/options.ts:312 |
| 267 | 0.75 | 357 | 5780 | export at source/types/ResponsePromise.ts:6 |
| 238 | 0.82 | 291 | 6116 | export at source/utils/options.ts:4 |
| 235 | 0.75 | 314 | 2889 | readme.md section #2 |
| 234 | 1.00 | 234 | 5423 | export at source/utils/normalize.ts:28 |
| 207 | 0.96 | 217 | 5189 | export at source/utils/body.ts:90 |
| 186 | 0.95 | 196 | 4361 | export at source/utils/options.ts:30 |
| 172 | 1.00 | 172 | 9636 | export doc at source/utils/type-guards.ts:71 |
| 165 | 1.00 | 165 | 9464 | export doc at source/utils/type-guards.ts:49 |
| 145 | 1.00 | 145 | 3554 | export at source/utils/merge.ts:16 |
| 117 | 0.92 | 128 | 3409 | export at source/utils/body.ts:119 |
| 112 | 1.00 | 112 | 705 | json config tsconfig.dist.json |
| 103 | 1.00 | 103 | 1279 | package scripts in package.json |
| 100 | 0.30 | 331 | 7891 | export names surface in source/types/options.ts |
| 82 | 1.00 | 82 | 3189 | export at source/utils/merge.ts:6 |
| 82 | 1.00 | 82 | 133 | json config tsconfig.json |
| 77 | 0.88 | 88 | 221 | readme.md section #1 |
| 77 | 0.86 | 90 | 4165 | export at source/types/hooks.ts:48 |
| 75 | 0.86 | 88 | 4075 | export at source/types/hooks.ts:5 |
| 75 | 0.24 | 314 | 6830 | export names surface in source/core/constants.ts |
| 73 | 0.89 | 83 | 3901 | export at source/types/hooks.ts:20 |
| 73 | 0.86 | 86 | 3987 | export at source/types/hooks.ts:32 |
| 73 | 0.62 | 117 | 3107 | export names surface in source/utils/merge.ts |
| 64 | 1.00 | 64 | 9299 | imports in source/utils/merge.ts |
| 62 | 0.39 | 161 | 3715 | export names surface in source/types/hooks.ts |
| 56 | 0.75 | 75 | 422 | readme.md section #9 |
| 55 | 1.00 | 55 | 8913 | imports in source/types/hooks.ts |
| 54 | 1.00 | 54 | 2990 | export doc at source/errors/NonError.ts:6 |
| 53 | 0.70 | 77 | 1689 | export at source/types/retry.ts:3 |
| 53 | 1.00 | 53 | 9235 | imports in source/utils/type-guards.ts |
| 52 | 0.67 | 79 | 347 | readme.md section #8 |
| 52 | 1.00 | 52 | 9124 | imports in source/utils/normalize.ts |
| 50 | 1.00 | 50 | 8963 | imports in source/types/ky.ts |
