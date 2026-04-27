scores: Sim=0.421 Reached=13/38 Early=3 Late=7 Partial=6 Missing=19 Used=9233/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 9 | 6 | 0 | 3 | 0.65 |
| 2 | 10 | 2 | 1 | 7 | 0.33 |
| 3 | 7 | 2 | 2 | 3 | 0.53 |
| 4 | 2 | 1 | 0 | 1 | 0.49 |
| 5 | 3 | 0 | 3 | 0 | 0.72 |
| 6 | 4 | 2 | 0 | 2 | 0.53 |
| 7 | 3 | 0 | 0 | 3 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 79 | — | — | 0.00 | missing | Readme one-line tagline |  |
| 1.3 | 163 | 2661 | +2498 | 1.00 | late | Readme top-level section headings (## only) | headings outline in readme.md (t=2661, 52 atoms) |
| 1.4 | 204 | — | — | 0.00 | missing | Readme target environments + 'no deps' note |  |
| 1.5 | 331 | 244 | -87 | 0.92 | aligned | Readme benefits-over-fetch list | README headline in readme.md (t=244, 11 atoms) |
| 1.6 | 447 | 1296 | +849 | 1.00 | late | source/ tree (all immediate children + every subdir) |  |
| 1.7 | 545 | — | — | 0.00 | missing | test/ tree (top-level + helpers/) |  |
| 1.8 | 714 | 6135 | +5421 | 0.93 | late | package.json identity (name, version, description, exports, engines) | package identity in package.json (t=791, 12 atoms) |
| 1.9 | 845 | 2661 | +1816 | 1.00 | late | Readme `## API` H3 location index | headings outline in readme.md (t=2661, 20 atoms) |
| 2.1 | 1051 | 6819 | +5768 | 0.87 | late | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | export at source/core/constants.ts:46 (t=6819, 10 atoms) |
| 2.2 | 1133 | — | — | 0.00 | missing | Readme Usage block (canonical example) |  |
| 2.3 | 1356 | — | — | 0.08 | missing | KyInstance: every member's signature (location batch) | export names surface in source/types/ky.ts (t=1142, 2 atoms) |
| 2.4 | 1768 | — | — | 0.00 | missing | source/index.ts named-export block |  |
| 2.5 | 1953 | — | — | 0.44 | missing | source/index.ts imports + createInstance signature + default export | imports in source/index.ts (t=2020, 6 atoms) |
| 2.6 | 2246 | — | — | 0.00 | missing | source/index.ts createInstance body |  |
| 2.7 | 2473 | — | — | 0.00 | missing | Default ky() body-method behavior + body shortcuts list |  |
| 2.8 | 2901 | 8426 | +5525 | 0.88 | late | core/constants.ts: feature-detection flags | export at source/core/constants.ts:4 (t=8426, 26 atoms) |
| 2.9 | 3186 | — | — | 0.29 | missing | ForceRetryOptions type + RetryMarker + retry() factory signature | export names surface in source/core/constants.ts (t=6642, 5 atoms) |
| 2.10 | 3658 | — | — | 0.77 | partial | kyOptionKeys + vendor/request option registries | export at source/core/constants.ts:265 (t=7070, 16 atoms) |
| 3.1 | 3855 | — | — | 0.07 | missing | KyOptions: every option's name + type signature (location batch) | export names surface in source/types/options.ts (t=7506, 2 atoms) |
| 3.2 | 4024 | — | — | 0.29 | missing | RetryOptions: every field + ShouldRetryState (location batch) | export at source/types/retry.ts:3 (t=4021, 10 atoms) |
| 3.3 | 4324 | — | — | 0.78 | partial | Hooks types: every state + hook type alias (location batch) | export names surface in source/types/hooks.ts (t=3841, 18 atoms) |
| 3.4 | 4587 | 7572 | +2985 | 0.83 | late | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | export names surface in source/types/options.ts (t=7506, 12 atoms) |
| 3.5 | 5127 | — | — | 0.71 | partial | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | export at source/types/options.ts:373 (t=7704, 11 atoms) |
| 3.6 | 5252 | — | — | 0.09 | missing | ResponsePromise type signature (no examples) | export names surface in source/types/ResponsePromise.ts (t=1158, 2 atoms) |
| 3.7 | 5448 | 1767 | -3681 | 0.93 | early | KyRequest + KyResponse + common Primitive/LiteralUnion types | export names surface in source/types/common.ts (t=1746, 6 atoms) |
| 4.1 | 5804 | 5508 | -296 | 0.96 | aligned | Ky class member declarations + every method signature (location batch) | export at source/core/Ky.ts:33 (t=5508, 34 atoms) |
| 4.2 | 6709 | — | — | 0.01 | missing | #calculateRetryDelay — full retry-decision logic | export at source/core/Ky.ts:33 (t=5508, 2 atoms) |
| 5.1 | 6904 | — | — | 0.67 | partial | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | export names surface in source/utils/type-guards.ts (t=2115, 8 atoms) |
| 5.2 | 7223 | — | — | 0.71 | partial | HTTPError + TimeoutError class bodies | export body at source/errors/HTTPError.ts:5 (t=3651, 9 atoms) |
| 5.3 | 7546 | — | — | 0.79 | partial | ForceRetryError class body + NonError signature | export body at source/errors/ForceRetryError.ts:8 (t=4860, 12 atoms) |
| 6.1 | 7855 | — | — | 0.10 | missing | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | export names surface in source/utils/normalize.ts (t=1648, 2 atoms) |
| 6.2 | 8679 | — | — | 0.13 | missing | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | export at source/utils/merge.ts:38 (t=2898, 8 atoms) |
| 6.3 | 8901 | 3171 | -5730 | 0.88 | early | timeout.ts body | export body at source/utils/timeout.ts:9 (t=3171, 15 atoms) |
| 6.4 | 9072 | 1890 | -7182 | 1.00 | early | options.ts utils + body.ts streaming exports + small util one-liners | export body at source/utils/body.ts:90 (t=8638, 22 atoms) |
| 7.1 | 9532 | — | — | 0.00 | missing | test/main.ts: representative test names (truncated) |  |
| 7.2 | 9911 | — | — | 0.00 | missing | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names |  |
| 7.3 | 9987 | — | — | 0.00 | missing | test/retry.ts: 4 most-distinctive test names (truncated) |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 347 | export at source/types/hooks.ts:<n> |
| 3 | 218 | readme.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 371 | 0.93 | 400 | 6135 | package dependencies in package.json |
| 312 | 0.85 | 366 | 791 | package identity in package.json |
| 255 | 1.00 | 255 | 9186 | export body at source/utils/options.ts:4 |
| 229 | 1.00 | 229 | 8931 | export body at source/utils/normalize.ts:28 |
| 212 | 1.00 | 212 | 8638 | export body at source/utils/body.ts:90 |
| 191 | 1.00 | 191 | 8124 | export body at source/utils/options.ts:30 |
| 155 | 1.00 | 155 | 3403 | export body at source/utils/delay.ts:9 |
| 148 | 0.92 | 161 | 4248 | export body at source/errors/NonError.ts:6 |
| 140 | 1.00 | 140 | 5689 | export body at source/utils/merge.ts:16 |
| 131 | 0.39 | 335 | 2661 | headings outline in readme.md |
| 123 | 1.00 | 123 | 5127 | export body at source/utils/body.ts:119 |
| 112 | 1.00 | 112 | 3515 | json config tsconfig.dist.json |
| 103 | 1.00 | 103 | 1097 | package scripts in package.json |
| 94 | 1.00 | 94 | 6229 | plaintext config .editorconfig |
| 90 | 1.00 | 90 | 4950 | export at source/types/hooks.ts:48 |
| 88 | 1.00 | 88 | 4687 | export at source/types/hooks.ts:5 |
| 86 | 1.00 | 86 | 4535 | export at source/types/hooks.ts:32 |
| 83 | 1.00 | 83 | 4331 | export at source/types/hooks.ts:20 |
| 82 | 1.00 | 82 | 2309 | json config tsconfig.json |
| 81 | 1.00 | 81 | 2979 | readme.md section #1 |
| 77 | 1.00 | 77 | 3248 | export body at source/utils/merge.ts:6 |
| 71 | 1.00 | 71 | 4402 | readme.md section #21 |
| 66 | 1.00 | 66 | 4087 | readme.md section #22 |
| 64 | 1.00 | 64 | 8702 | imports in source/utils/merge.ts |
| 62 | 0.53 | 117 | 2806 | export names surface in source/utils/merge.ts |
| 59 | 0.77 | 77 | 4021 | export at source/types/retry.ts:3 |
| 55 | 1.00 | 55 | 7933 | imports in source/types/hooks.ts |
| 54 | 1.00 | 54 | 5004 | export doc at source/errors/NonError.ts:6 |
| 53 | 1.00 | 53 | 7175 | imports in source/utils/type-guards.ts |
| 52 | 1.00 | 52 | 7122 | imports in source/utils/normalize.ts |
| 50 | 1.00 | 50 | 6328 | imports in source/types/ky.ts |
