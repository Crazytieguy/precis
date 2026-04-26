scores: Sim=0.407 Reached=14/38 Early=3 Late=8 Partial=7 Missing=17 Used=9850/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 6 | 0 | 3 | 0.65 |
| 2 | 10 | 2 | 2 | 6 | 0.41 |
| 3 | 7 | 3 | 2 | 2 | 0.64 |
| 4 | 2 | 1 | 0 | 1 | 0.49 |
| 5 | 3 | 0 | 3 | 0 | 0.72 |
| 6 | 4 | 2 | 0 | 2 | 0.53 |
| 7 | 3 | 0 | 0 | 3 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 79 | — | — | 0.00 | missing | Readme one-line tagline |  |
| 1.3 | 163 | 2850 | +2687 | 1.00 | late | Readme top-level section headings (## only) | headings outline in readme.md (t=2850, 52 atoms) |
| 1.4 | 204 | — | — | 0.00 | missing | Readme target environments + 'no deps' note |  |
| 1.5 | 331 | 244 | -87 | 0.92 | aligned | Readme benefits-over-fetch list | README headline in readme.md (t=244, 11 atoms) |
| 1.6 | 447 | 1567 | +1120 | 1.00 | late | source/ tree (all immediate children + every subdir) |  |
| 1.7 | 545 | — | — | 0.00 | missing | test/ tree (top-level + helpers/) |  |
| 1.8 | 714 | 6840 | +6126 | 0.93 | late | package.json identity (name, version, description, exports, engines) | package identity in package.json (t=985, 12 atoms) |
| 1.9 | 845 | 2850 | +2005 | 1.00 | late | Readme `## API` H3 location index | headings outline in readme.md (t=2850, 20 atoms) |
| 2.1 | 1051 | 7524 | +6473 | 0.87 | late | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | export at source/core/constants.ts:46 (t=7524, 10 atoms) |
| 2.2 | 1133 | — | — | 0.75 | partial | Readme Usage block (canonical example) | readme.md section #2 (t=4036, 6 atoms) |
| 2.3 | 1356 | — | — | 0.08 | missing | KyInstance: every member's signature (location batch) | export names surface in source/types/ky.ts (t=1336, 2 atoms) |
| 2.4 | 1768 | — | — | 0.00 | missing | source/index.ts named-export block |  |
| 2.5 | 1953 | — | — | 0.44 | missing | source/index.ts imports + createInstance signature + default export | imports in source/index.ts (t=2291, 6 atoms) |
| 2.6 | 2246 | — | — | 0.00 | missing | source/index.ts createInstance body |  |
| 2.7 | 2473 | — | — | 0.00 | missing | Default ky() body-method behavior + body shortcuts list |  |
| 2.8 | 2901 | 9574 | +6673 | 0.88 | late | core/constants.ts: feature-detection flags | export at source/core/constants.ts:4 (t=9574, 26 atoms) |
| 2.9 | 3186 | — | — | 0.29 | missing | ForceRetryOptions type + RetryMarker + retry() factory signature | export names surface in source/core/constants.ts (t=7347, 5 atoms) |
| 2.10 | 3658 | — | — | 0.77 | partial | kyOptionKeys + vendor/request option registries | export at source/core/constants.ts:265 (t=7775, 16 atoms) |
| 3.1 | 3855 | — | — | 0.07 | missing | KyOptions: every option's name + type signature (location batch) | export names surface in source/types/options.ts (t=8211, 2 atoms) |
| 3.2 | 4024 | — | — | 0.29 | missing | RetryOptions: every field + ShouldRetryState (location batch) | export at source/types/retry.ts:3 (t=1529, 10 atoms) |
| 3.3 | 4324 | — | — | 0.78 | partial | Hooks types: every state + hook type alias (location batch) | export names surface in source/types/hooks.ts (t=4362, 18 atoms) |
| 3.4 | 4587 | 8277 | +3690 | 0.83 | late | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | export names surface in source/types/options.ts (t=8211, 12 atoms) |
| 3.5 | 5127 | 8997 | +3870 | 0.89 | late | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | export at source/types/options.ts:312 (t=8997, 34 atoms) |
| 3.6 | 5252 | — | — | 0.67 | partial | ResponsePromise type signature (no examples) | export at source/types/ResponsePromise.ts:6 (t=6440, 32 atoms) |
| 3.7 | 5448 | 2074 | -3374 | 0.93 | early | KyRequest + KyResponse + common Primitive/LiteralUnion types | export names surface in source/types/common.ts (t=2053, 6 atoms) |
| 4.1 | 5804 | 5856 | +52 | 0.96 | aligned | Ky class member declarations + every method signature (location batch) | export at source/core/Ky.ts:33 (t=5856, 34 atoms) |
| 4.2 | 6709 | — | — | 0.01 | missing | #calculateRetryDelay — full retry-decision logic | export at source/core/Ky.ts:33 (t=5856, 2 atoms) |
| 5.1 | 6904 | — | — | 0.67 | partial | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | export names surface in source/utils/type-guards.ts (t=2386, 8 atoms) |
| 5.2 | 7223 | — | — | 0.71 | partial | HTTPError + TimeoutError class bodies | export body at source/errors/HTTPError.ts:5 (t=4172, 9 atoms) |
| 5.3 | 7546 | — | — | 0.79 | partial | ForceRetryError class body + NonError signature | export body at source/errors/ForceRetryError.ts:8 (t=5257, 12 atoms) |
| 6.1 | 7855 | — | — | 0.10 | missing | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | export names surface in source/utils/normalize.ts (t=1955, 2 atoms) |
| 6.2 | 8679 | — | — | 0.13 | missing | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | export at source/utils/merge.ts:38 (t=3305, 8 atoms) |
| 6.3 | 8901 | 3497 | -5404 | 0.88 | early | timeout.ts body | export body at source/utils/timeout.ts:9 (t=3497, 15 atoms) |
| 6.4 | 9072 | 2161 | -6911 | 1.00 | early | options.ts utils + body.ts streaming exports + small util one-liners | export body at source/utils/body.ts:90 (t=9786, 22 atoms) |
| 7.1 | 9532 | — | — | 0.00 | missing | test/main.ts: representative test names (truncated) |  |
| 7.2 | 9911 | — | — | 0.00 | missing | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names |  |
| 7.3 | 9987 | — | — | 0.00 | missing | test/retry.ts: 4 most-distinctive test names (truncated) |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 453 | readme.md section #<n> |
| 4 | 347 | export at source/types/hooks.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 371 | 0.93 | 400 | 6840 | package dependencies in package.json |
| 312 | 0.85 | 366 | 985 | package identity in package.json |
| 271 | 0.74 | 367 | 8997 | export at source/types/options.ts:312 |
| 268 | 0.75 | 357 | 6440 | export at source/types/ResponsePromise.ts:6 |
| 235 | 0.77 | 307 | 4036 | readme.md section #2 |
| 212 | 1.00 | 212 | 9786 | export body at source/utils/body.ts:90 |
| 191 | 1.00 | 191 | 9272 | export body at source/utils/options.ts:30 |
| 155 | 1.00 | 155 | 3729 | export body at source/utils/delay.ts:9 |
| 148 | 0.92 | 161 | 4973 | export body at source/errors/NonError.ts:6 |
| 140 | 1.00 | 140 | 6037 | export body at source/utils/merge.ts:16 |
| 131 | 0.39 | 335 | 2850 | headings outline in readme.md |
| 123 | 1.00 | 123 | 5434 | export body at source/utils/body.ts:119 |
| 112 | 1.00 | 112 | 619 | json config tsconfig.dist.json |
| 103 | 1.00 | 103 | 1193 | package scripts in package.json |
| 94 | 1.00 | 94 | 6934 | plaintext config .editorconfig |
| 90 | 1.00 | 90 | 4812 | export at source/types/hooks.ts:48 |
| 88 | 1.00 | 88 | 4722 | export at source/types/hooks.ts:5 |
| 86 | 1.00 | 86 | 4634 | export at source/types/hooks.ts:32 |
| 83 | 1.00 | 83 | 4548 | export at source/types/hooks.ts:20 |
| 82 | 1.00 | 82 | 326 | json config tsconfig.json |
| 81 | 1.00 | 81 | 2959 | readme.md section #1 |
| 77 | 1.00 | 77 | 3574 | export body at source/utils/merge.ts:6 |
| 71 | 1.00 | 71 | 3096 | readme.md section #21 |
| 66 | 1.00 | 66 | 3025 | readme.md section #22 |
| 64 | 1.00 | 64 | 9850 | imports in source/utils/merge.ts |
| 62 | 0.53 | 117 | 3213 | export names surface in source/utils/merge.ts |
| 59 | 0.77 | 77 | 1529 | export at source/types/retry.ts:3 |
| 55 | 1.00 | 55 | 9081 | imports in source/types/hooks.ts |
| 54 | 1.00 | 54 | 5311 | export doc at source/errors/NonError.ts:6 |
| 53 | 1.00 | 53 | 7880 | imports in source/utils/type-guards.ts |
| 52 | 1.00 | 52 | 7827 | imports in source/utils/normalize.ts |
| 50 | 1.00 | 50 | 7033 | imports in source/types/ky.ts |
