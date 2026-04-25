scores: Sim=0.406 Reached=12/38 Early=2 Late=8 Partial=4 Missing=22 Used=9576/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 6 | 0 | 3 | 0.65 |
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
| 1.3 | 163 | 2912 | +2749 | 1.00 | late | Readme top-level section headings (## only) | headings outline in readme.md (t=2912, 52 atoms) |
| 1.4 | 204 | — | — | 0.00 | missing | Readme target environments + 'no deps' note |  |
| 1.5 | 331 | 173 | -158 | 0.92 | early | Readme benefits-over-fetch list | README headline in readme.md (t=173, 11 atoms) |
| 1.6 | 447 | 1386 | +939 | 1.00 | late | source/ tree (all immediate children + every subdir) |  |
| 1.7 | 545 | — | — | 0.00 | missing | test/ tree (top-level + helpers/) |  |
| 1.8 | 714 | 6502 | +5788 | 0.93 | late | package.json identity (name, version, description, exports, engines) | package identity in package.json (t=782, 12 atoms) |
| 1.9 | 845 | 2912 | +2067 | 1.00 | late | Readme `## API` H3 location index | headings outline in readme.md (t=2912, 20 atoms) |
| 2.1 | 1051 | 6963 | +5912 | 0.87 | late | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | export at source/core/constants.ts:46 (t=6963, 10 atoms) |
| 2.2 | 1133 | — | — | 0.75 | partial | Readme Usage block (canonical example) | readme.md section #2 (t=3465, 6 atoms) |
| 2.3 | 1356 | — | — | 0.08 | missing | KyInstance: every member's signature (location batch) | export names surface in source/types/ky.ts (t=1155, 2 atoms) |
| 2.4 | 1768 | — | — | 0.00 | missing | source/index.ts named-export block |  |
| 2.5 | 1953 | — | — | 0.44 | missing | source/index.ts imports + createInstance signature + default export | imports in source/index.ts (t=2101, 6 atoms) |
| 2.6 | 2246 | — | — | 0.00 | missing | source/index.ts createInstance body |  |
| 2.7 | 2473 | — | — | 0.00 | missing | Default ky() body-method behavior + body shortcuts list |  |
| 2.8 | 2901 | 8983 | +6082 | 0.88 | late | core/constants.ts: feature-detection flags | export at source/core/constants.ts:4 (t=8983, 26 atoms) |
| 2.9 | 3186 | — | — | 0.29 | missing | ForceRetryOptions type + RetryMarker + retry() factory signature | export names surface in source/core/constants.ts (t=6816, 5 atoms) |
| 2.10 | 3658 | — | — | 0.77 | partial | kyOptionKeys + vendor/request option registries | export at source/core/constants.ts:265 (t=7214, 16 atoms) |
| 3.1 | 3855 | — | — | 0.07 | missing | KyOptions: every option's name + type signature (location batch) | export names surface in source/types/options.ts (t=7575, 2 atoms) |
| 3.2 | 4024 | — | — | 0.29 | missing | RetryOptions: every field + ShouldRetryState (location batch) | export at source/types/retry.ts:3 (t=1348, 10 atoms) |
| 3.3 | 4324 | — | — | 0.78 | partial | Hooks types: every state + hook type alias (location batch) | export names surface in source/types/hooks.ts (t=3754, 18 atoms) |
| 3.4 | 4587 | 7641 | +3054 | 0.83 | late | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | export names surface in source/types/options.ts (t=7575, 12 atoms) |
| 3.5 | 5127 | 8314 | +3187 | 0.89 | late | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | export at source/types/options.ts:312 (t=8314, 34 atoms) |
| 3.6 | 5252 | — | — | 0.67 | partial | ResponsePromise type signature (no examples) | export at source/types/ResponsePromise.ts:6 (t=6102, 32 atoms) |
| 3.7 | 5448 | 1862 | -3586 | 0.93 | early | KyRequest + KyResponse + common Primitive/LiteralUnion types | export names surface in source/types/common.ts (t=1841, 6 atoms) |
| 4.1 | 5804 | — | — | 0.04 | missing | Ky class member declarations + every method signature (location batch) | export names surface in source/core/Ky.ts (t=1012, 2 atoms) |
| 4.2 | 6709 | — | — | 0.00 | missing | #calculateRetryDelay — full retry-decision logic |  |
| 5.1 | 6904 | — | — | 0.34 | missing | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | export names surface in source/utils/type-guards.ts (t=2196, 8 atoms) |
| 5.2 | 7223 | — | — | 0.29 | missing | HTTPError + TimeoutError class bodies | export at source/errors/HTTPError.ts:5 (t=1914, 6 atoms) |
| 5.3 | 7546 | — | — | 0.32 | missing | ForceRetryError class body + NonError signature | export at source/errors/ForceRetryError.ts:8 (t=2255, 7 atoms) |
| 6.1 | 7855 | — | — | 0.10 | missing | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | export names surface in source/utils/normalize.ts (t=1667, 2 atoms) |
| 6.2 | 8679 | — | — | 0.13 | missing | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | export at source/utils/merge.ts:38 (t=2577, 8 atoms) |
| 6.3 | 8901 | — | — | 0.25 | missing | timeout.ts body | export at source/utils/timeout.ts:9 (t=1763, 7 atoms) |
| 7.1 | 9532 | — | — | 0.00 | missing | test/main.ts: representative test names (truncated) |  |
| 7.2 | 9911 | — | — | 0.00 | missing | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names |  |
| 7.3 | 9987 | — | — | 0.00 | missing | test/retry.ts: 4 most-distinctive test names (truncated) |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 879 | readme.md section #<n> |
| 2 | 424 | export at source/utils/options.ts:<n> |
| 2 | 324 | export at source/utils/body.ts:<n> |
| 4 | 298 | export at source/types/hooks.ts:<n> |
| 2 | 227 | export at source/utils/merge.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 579 | 0.96 | 603 | 4952 | readme.md section #5 |
| 352 | 0.88 | 400 | 6502 | package dependencies in package.json |
| 322 | 0.88 | 366 | 782 | package identity in package.json |
| 280 | 0.76 | 367 | 8314 | export at source/types/options.ts:312 |
| 267 | 0.75 | 357 | 6102 | export at source/types/ResponsePromise.ts:6 |
| 238 | 0.82 | 291 | 8681 | export at source/utils/options.ts:4 |
| 234 | 1.00 | 234 | 5745 | export at source/utils/normalize.ts:28 |
| 230 | 0.75 | 307 | 3465 | readme.md section #2 |
| 207 | 0.96 | 217 | 5511 | export at source/utils/body.ts:90 |
| 195 | 0.58 | 335 | 2912 | headings outline in readme.md |
| 186 | 0.95 | 196 | 5193 | export at source/utils/options.ts:30 |
| 145 | 1.00 | 145 | 4349 | export at source/utils/merge.ts:16 |
| 117 | 0.92 | 128 | 3593 | export at source/utils/body.ts:119 |
| 112 | 1.00 | 112 | 416 | json config tsconfig.dist.json |
| 103 | 1.00 | 103 | 990 | package scripts in package.json |
| 100 | 0.30 | 331 | 7575 | export names surface in source/types/options.ts |
| 82 | 1.00 | 82 | 2485 | export at source/utils/merge.ts:6 |
| 82 | 1.00 | 82 | 255 | json config tsconfig.json |
| 77 | 0.86 | 90 | 4204 | export at source/types/hooks.ts:48 |
| 75 | 0.86 | 88 | 4114 | export at source/types/hooks.ts:5 |
| 75 | 0.24 | 314 | 6816 | export names surface in source/core/constants.ts |
| 73 | 0.89 | 83 | 3940 | export at source/types/hooks.ts:20 |
| 73 | 0.86 | 86 | 4026 | export at source/types/hooks.ts:32 |
| 73 | 0.62 | 117 | 2403 | export names surface in source/utils/merge.ts |
| 70 | 0.88 | 81 | 2993 | readme.md section #1 |
| 64 | 1.00 | 64 | 9576 | imports in source/utils/merge.ts |
| 62 | 0.39 | 161 | 3754 | export names surface in source/types/hooks.ts |
| 55 | 1.00 | 55 | 9190 | imports in source/types/hooks.ts |
| 54 | 1.00 | 54 | 5294 | export doc at source/errors/NonError.ts:6 |
| 53 | 0.70 | 77 | 1348 | export at source/types/retry.ts:3 |
| 53 | 1.00 | 53 | 9512 | imports in source/utils/type-guards.ts |
| 52 | 1.00 | 52 | 9401 | imports in source/utils/normalize.ts |
| 50 | 1.00 | 50 | 9240 | imports in source/types/ky.ts |
