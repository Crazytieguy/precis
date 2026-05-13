Score(3000)=0.520 I=0.794 C=0.340 ns_rows≤3K=17/38 (reached=5 partial=1 missing=11)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | listing of '.' |  |  | 1.000 |
| ns | 40 |  | 40 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 51 | 11 | listing of '.github' |  |  | 1.000 |
| walker |  | 55 | 4 | listing of '.github/workflows' |  |  | 1.000 |
| walker |  | 67 | 12 | listing of 'media' |  |  | 1.000 |
| ns | 79 |  | 39 | Readme one-line tagline | 1.2 |  | 0.953 |
| walker |  | 83 | 16 | listing of 'source' |  |  | 0.956 |
| walker |  | 93 | 10 | export names surface in source/index.ts |  |  | 0.956 |
| walker |  | 101 | 8 | listing of 'source/core' |  |  | 0.959 |
| walker |  | 111 | 10 | export names surface in source/core/Ky.ts |  |  | 0.959 |
| ns | 163 |  | 84 | Readme top-level section headings (## only) | 1.3 |  | 0.680 |
| ns | 204 |  | 41 | Readme target environments + 'no deps' note | 1.4 |  | 0.631 |
| walker |  | 244 | 133 | README headline in readme.md |  |  | 0.670 |
| walker |  | 265 | 21 | listing of 'source/errors' |  |  | 0.674 |
| walker |  | 278 | 13 | export names surface in source/errors/NonError.ts |  |  | 0.674 |
| walker |  | 291 | 13 | export names surface in source/errors/TimeoutError.ts |  |  | 0.674 |
| walker |  | 311 | 20 | export at source/errors/TimeoutError.ts:3 |  |  | 0.675 |
| walker |  | 325 | 14 | export names surface in source/errors/ForceRetryError.ts |  |  | 0.675 |
| ns | 331 |  | 127 | Readme benefits-over-fetch list | 1.5 | 1.3 | 0.726 |
| walker |  | 342 | 17 | export names surface in source/errors/HTTPError.ts |  |  | 0.726 |
| walker |  | 373 | 31 | export at source/errors/NonError.ts:6 |  |  | 0.726 |
| walker |  | 425 | 52 | export at source/errors/HTTPError.ts:5 |  |  | 0.726 |
| ns | 447 |  | 116 | source/ tree (all immediate children + every subdir) | 1.6 |  | 0.567 |
| ns | 545 |  | 98 | test/ tree (top-level + helpers/) | 1.7 |  | 0.481 |
| ns | 714 |  | 169 | package.json identity (name, version, description, exports, engines) | 1.8 |  | 0.438 |
| walker |  | 791 | 366 | package identity in package.json |  |  | 0.450 |
| ns | 845 |  | 131 | Readme `## API` H3 location index | 1.9 |  | 0.419 |
| walker |  | 850 | 59 | export at source/errors/ForceRetryError.ts:8 |  |  | 0.420 |
| walker |  | 889 | 39 | export body at source/errors/TimeoutError.ts:3 body 7 |  |  | 0.420 |
| walker |  | 994 | 105 | package entrypoints in package.json |  |  | 0.482 |
| ns | 1051 |  | 206 | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | 2.1 |  | 0.448 |
| walker |  | 1097 | 103 | package scripts in package.json |  |  | 0.448 |
| walker |  | 1130 | 33 | listing of 'source/types' |  |  | 0.507 |
| ns | 1133 |  | 82 | Readme Usage block (canonical example) | 2.2 |  | 0.489 |
| walker |  | 1142 | 12 | export names surface in source/types/ky.ts |  |  | 0.489 |
| walker |  | 1158 | 16 | export names surface in source/types/ResponsePromise.ts |  |  | 0.489 |
| walker |  | 1174 | 16 | export names surface in source/types/request.ts |  |  | 0.489 |
| walker |  | 1190 | 16 | export names surface in source/types/response.ts |  |  | 0.489 |
| walker |  | 1215 | 25 | export names surface in source/types/retry.ts |  |  | 0.489 |
| walker |  | 1236 | 21 | export at source/types/request.ts:1 |  |  | 0.490 |
| walker |  | 1258 | 22 | export at source/types/response.ts:1 |  |  | 0.490 |
| walker |  | 1296 | 38 | listing of 'source/utils' |  |  | 0.582 |
| walker |  | 1320 | 24 | export names surface in source/utils/delay.ts |  |  | 0.582 |
| walker |  | 1335 | 15 | export at source/utils/delay.ts:5 |  |  | 0.582 |
| ns | 1356 |  | 223 | KyInstance: every member's signature (location batch) | 2.3 |  | 0.555 |
| walker |  | 1363 | 28 | export at source/utils/delay.ts:9 |  |  | 0.555 |
| walker |  | 1387 | 24 | export names surface in source/utils/timeout.ts |  |  | 0.555 |
| walker |  | 1408 | 21 | export at source/utils/timeout.ts:3 |  |  | 0.555 |
| walker |  | 1456 | 48 | export at source/utils/timeout.ts:9 |  |  | 0.555 |
| walker |  | 1484 | 28 | export names surface in source/utils/is.ts |  |  | 0.555 |
| walker |  | 1518 | 34 | export names surface in source/utils/types.ts |  |  | 0.555 |
| walker |  | 1536 | 18 | export at source/utils/types.ts:1 |  |  | 0.555 |
| walker |  | 1571 | 35 | export names surface in source/utils/options.ts |  |  | 0.555 |
| walker |  | 1571 | 0 | export at source/utils/options.ts:30 |  |  | 0.555 |
| walker |  | 1602 | 31 | export at source/utils/options.ts:4 |  |  | 0.555 |
| walker |  | 1648 | 46 | export names surface in source/utils/normalize.ts |  |  | 0.555 |
| walker |  | 1648 | 0 | export at source/utils/normalize.ts:28 |  |  | 0.555 |
| walker |  | 1668 | 20 | export at source/utils/normalize.ts:5 |  |  | 0.555 |
| walker |  | 1704 | 36 | export at source/utils/types.ts:5 |  |  | 0.555 |
| walker |  | 1719 | 15 | imports in source/types/ResponsePromise.ts |  |  | 0.555 |
| walker |  | 1734 | 15 | imports in source/types/retry.ts |  |  | 0.555 |
| ns | 1768 |  | 412 | source/index.ts named-export block | 2.4 |  | 0.485 |
| walker |  | 1834 | 100 | imports in source/index.ts |  |  | 0.487 |
| ns | 1953 |  | 185 | source/index.ts imports + createInstance signature + default export | 2.5 |  | 0.481 |
| walker |  | 2165 | 331 | export names surface in source/types/options.ts |  |  | 0.482 |
| walker |  | 2165 | 0 | export at source/types/options.ts:307 |  |  | 0.482 |
| walker |  | 2181 | 16 | imports in source/errors/TimeoutError.ts |  |  | 0.482 |
| walker |  | 2197 | 16 | imports in source/utils/delay.ts |  |  | 0.482 |
| ns | 2246 |  | 293 | source/index.ts createInstance body | 2.6 | 2.5 | 0.457 |
| walker |  | 2279 | 82 | json config tsconfig.json |  |  | 0.457 |
| walker |  | 2296 | 17 | imports in source/utils/timeout.ts |  |  | 0.457 |
| walker |  | 2374 | 78 | export names surface in source/types/common.ts |  |  | 0.457 |
| walker |  | 2395 | 21 | export at source/types/common.ts:6 |  |  | 0.458 |
| ns | 2473 |  | 227 | Default ky() body-method behavior + body shortcuts list | 2.7 |  | 0.454 |
| walker |  | 2730 | 335 | headings outline in readme.md |  |  | 0.563 |
| walker |  | 2742 | 12 | readme.md section #48 |  |  | 0.563 |
| walker |  | 2758 | 16 | readme.md section #47 |  |  | 0.563 |
| walker |  | 2773 | 15 | readme.md section #22 |  |  | 0.563 |
| walker |  | 2790 | 17 | readme.md section #18 |  |  | 0.563 |
| walker |  | 2808 | 18 | readme.md section #7 |  |  | 0.563 |
| walker |  | 2827 | 19 | readme.md section #11 |  |  | 0.563 |
| walker |  | 2846 | 19 | readme.md section #13 |  |  | 0.563 |
| ns | 2901 |  | 428 | core/constants.ts: feature-detection flags | 2.8 |  | 0.520 |
| walker |  | 2927 | 81 | readme.md section #1 |  |  | 0.520 |
| walker |  | 2972 | 45 | .github/security.md section #0 |  |  | 0.520 |
| walker |  | 3059 | 87 | export names surface in source/utils/body.ts |  |  | 0.520 |
| walker |  | 3059 | 0 | export at source/utils/body.ts:5 |  |  | 0.520 |
| walker |  | 3059 | 0 | export at source/utils/body.ts:90 |  |  | 0.520 |
| walker |  | 3059 | 0 | export at source/utils/body.ts:119 |  |  | 0.520 |
| walker |  | 3081 | 22 | readme.md section #12 |  |  | 0.520 |
| ns | 3186 |  | 285 | ForceRetryOptions type + RetryMarker + retry() factory signature | 2.9 |  | 0.491 |
| walker |  | 3228 | 147 | export body at source/utils/timeout.ts:9 body 15 |  |  | 0.492 |
| walker |  | 3323 | 95 | export names surface in source/utils/type-guards.ts |  |  | 0.492 |
| walker |  | 3323 | 0 | export at source/utils/type-guards.ts:27 |  |  | 0.492 |
| walker |  | 3323 | 0 | export at source/utils/type-guards.ts:49 |  |  | 0.492 |
| walker |  | 3323 | 0 | export at source/utils/type-guards.ts:71 |  |  | 0.492 |
| walker |  | 3323 | 0 | export at source/utils/type-guards.ts:98 |  |  | 0.492 |
| walker |  | 3342 | 19 | export body at source/utils/type-guards.ts:49 body 50 |  |  | 0.492 |
| walker |  | 3361 | 19 | export body at source/utils/type-guards.ts:71 body 72 |  |  | 0.492 |
| walker |  | 3382 | 21 | export body at source/utils/type-guards.ts:27 body 28 |  |  | 0.492 |
| walker |  | 3403 | 21 | export body at source/utils/type-guards.ts:98 body 99 |  |  | 0.493 |
| walker |  | 3425 | 22 | readme.md section #45 |  |  | 0.493 |
| walker |  | 3580 | 155 | export body at source/utils/delay.ts:9 body 13 |  |  | 0.493 |
| walker |  | 3646 | 66 | export at source/types/options.ts:16 |  |  | 0.493 |
| ns | 3658 |  | 472 | kyOptionKeys + vendor/request option registries | 2.10 |  | 0.455 |
| walker |  | 3758 | 112 | json config tsconfig.dist.json |  |  | 0.455 |
| ns | 3855 |  | 197 | KyOptions: every option's name + type signature (location batch) | 3.1 |  | 0.444 |
| walker |  | 3894 | 136 | export body at source/errors/HTTPError.ts:5 body 11 |  |  | 0.445 |
| walker |  | 3923 | 29 | plaintext config .gitignore |  |  | 0.445 |
| walker |  | 4000 | 77 | export at source/types/retry.ts:3 |  |  | 0.445 |
| ns | 4024 |  | 169 | RetryOptions: every field + ShouldRetryState (location batch) | 3.2 |  | 0.437 |
| walker |  | 4117 | 117 | export names surface in source/utils/merge.ts |  |  | 0.437 |
| walker |  | 4117 | 0 | export at source/utils/merge.ts:6 |  |  | 0.437 |
| walker |  | 4117 | 0 | export at source/utils/merge.ts:16 |  |  | 0.437 |
| walker |  | 4117 | 0 | export at source/utils/merge.ts:86 |  |  | 0.437 |
| walker |  | 4209 | 92 | export at source/utils/merge.ts:38 |  |  | 0.438 |
| walker |  | 4286 | 77 | export body at source/utils/merge.ts:6 body 7 |  |  | 0.438 |
| ns | 4324 |  | 300 | Hooks types: every state + hook type alias (location batch) | 3.3 |  | 0.422 |
| walker |  | 4352 | 66 | readme.md section #50 |  |  | 0.422 |
| walker |  | 4513 | 161 | export body at source/errors/NonError.ts:6 body 11 |  |  | 0.422 |
| walker |  | 4584 | 71 | readme.md section #49 |  |  | 0.422 |
| ns | 4587 |  | 263 | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | 3.4 |  | 0.442 |
| walker |  | 4631 | 47 | export doc at source/errors/ForceRetryError.ts:8 |  |  | 0.442 |
| walker |  | 4663 | 32 | imports in source/errors/ForceRetryError.ts |  |  | 0.442 |
| walker |  | 4695 | 32 | imports in source/utils/body.ts |  |  | 0.442 |
| walker |  | 4868 | 173 | export body at source/errors/ForceRetryError.ts:8 body 15 |  |  | 0.443 |
| walker |  | 4901 | 33 | readme.md section #8 |  |  | 0.443 |
| walker |  | 4955 | 54 | export doc at source/errors/NonError.ts:6 |  |  | 0.443 |
| walker |  | 5078 | 123 | export body at source/utils/body.ts:119 body 120 |  |  | 0.443 |
| ns | 5127 |  | 540 | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | 3.5 | 3.1 | 0.423 |
| walker |  | 5210 | 132 | export at source/types/options.ts:373 |  |  | 0.437 |
| ns | 5252 |  | 125 | ResponsePromise type signature (no examples) | 3.6 |  | 0.430 |
| walker |  | 5276 | 66 | export names surface #1 in source/core/constants.ts |  |  | 0.431 |
| walker |  | 5276 | 0 | export at source/core/constants.ts:235 |  |  | 0.431 |
| walker |  | 5302 | 26 | export at source/core/constants.ts:256 |  |  | 0.432 |
| walker |  | 5424 | 122 | export at source/core/constants.ts:237 |  |  | 0.445 |
| ns | 5448 |  | 196 | KyRequest + KyResponse + common Primitive/LiteralUnion types | 3.7 |  | 0.458 |
| walker |  | 5553 | 129 | export at source/core/constants.ts:265 |  |  | 0.480 |
| walker |  | 5593 | 40 | imports in source/core/constants.ts |  |  | 0.480 |
| ns | 5804 |  | 356 | Ky class member declarations + every method signature (location batch) | 4.1 |  | 0.467 |
| walker |  | 5934 | 341 | export at source/core/Ky.ts:33 |  |  | 0.501 |
| walker |  | 5975 | 41 | imports in source/utils/options.ts |  |  | 0.501 |
| walker |  | 6136 | 161 | export names surface in source/types/hooks.ts |  |  | 0.508 |
| walker |  | 6182 | 46 | export at source/types/hooks.ts:14 |  |  | 0.515 |
| walker |  | 6239 | 57 | export at source/types/hooks.ts:41 |  |  | 0.526 |
| walker |  | 6322 | 83 | export at source/types/hooks.ts:20 |  |  | 0.526 |
| walker |  | 6408 | 86 | export at source/types/hooks.ts:32 |  |  | 0.526 |
| walker |  | 6496 | 88 | export at source/types/hooks.ts:5 |  |  | 0.526 |
| walker |  | 6586 | 90 | export at source/types/hooks.ts:48 |  |  | 0.526 |
| walker |  | 6623 | 37 | readme.md section #19 |  |  | 0.526 |
| ns | 6709 |  | 905 | #calculateRetryDelay — full retry-decision logic | 4.2 | 4.1 | 0.485 |
| walker |  | 6763 | 140 | export body at source/utils/merge.ts:16 body 17 |  |  | 0.485 |
| walker |  | 6829 | 66 | listing of 'test' |  |  | 0.507 |
| walker |  | 6861 | 32 | listing of 'test/helpers' |  |  | 0.527 |
| ns | 6904 |  | 195 | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | 5.1 |  | 0.531 |
| walker |  | 6907 | 46 | imports in source/types/options.ts |  |  | 0.531 |
| ns | 7223 |  | 319 | HTTPError + TimeoutError class bodies | 5.2 |  | 0.541 |
| walker |  | 7307 | 400 | package dependencies in package.json |  |  | 0.547 |
| walker |  | 7401 | 94 | plaintext config .editorconfig |  |  | 0.547 |
| walker |  | 7450 | 49 | imports in source/errors/HTTPError.ts |  |  | 0.547 |
| walker |  | 7500 | 50 | imports in source/types/ky.ts |  |  | 0.547 |
| ns | 7546 |  | 323 | ForceRetryError class body + NonError signature | 5.3 |  | 0.558 |
| walker |  | 7552 | 52 | imports in source/utils/normalize.ts |  |  | 0.558 |
| walker |  | 7583 | 31 | export doc at source/types/options.ts:373 |  |  | 0.561 |
| walker |  | 7726 | 143 | export at source/types/options.ts:358 |  |  | 0.575 |
| walker |  | 7779 | 53 | imports in source/utils/type-guards.ts |  |  | 0.575 |
| walker |  | 7825 | 46 | readme.md section #28 |  |  | 0.575 |
| ns | 7855 |  | 309 | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | 6.1 |  | 0.565 |
| walker |  | 7880 | 55 | imports in source/types/hooks.ts |  |  | 0.565 |
| walker |  | 8071 | 191 | export body at source/utils/options.ts:30 body 31 |  |  | 0.565 |
| walker |  | 8119 | 48 | readme.md section #3 |  |  | 0.565 |
| walker |  | 8168 | 49 | readme.md section #14 |  |  | 0.565 |
| walker |  | 8219 | 51 | readme.md section #27 |  |  | 0.565 |
| walker |  | 8431 | 212 | export body at source/utils/body.ts:90 body 91 |  |  | 0.565 |
| walker |  | 8495 | 64 | imports in source/utils/merge.ts |  |  | 0.565 |
| ns | 8679 |  | 824 | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | 6.2 |  | 0.536 |
| walker |  | 8724 | 229 | export body at source/utils/normalize.ts:28 body 29 |  |  | 0.536 |
| ns | 8901 |  | 222 | timeout.ts body | 6.3 |  | 0.546 |
| walker |  | 8979 | 255 | export body at source/utils/options.ts:4 body 8 |  |  | 0.546 |
| ns | 9072 |  | 171 | options.ts utils + body.ts streaming exports + small util one-liners | 6.4 |  | 0.551 |
| walker |  | 9227 | 248 | export names surface in source/core/constants.ts |  |  | 0.556 |
| walker |  | 9238 | 11 | export at source/core/constants.ts:148 |  |  | 0.556 |
| walker |  | 9268 | 30 | export doc at source/core/constants.ts:148 |  |  | 0.558 |
| walker |  | 9376 | 108 | export at source/core/constants.ts:46 |  |  | 0.571 |
| ns | 9532 |  | 460 | test/main.ts: representative test names (truncated) | 7.1 |  | 0.558 |
| walker |  | 9678 | 302 | export at source/core/constants.ts:4 |  |  | 0.588 |
| walker |  | 9741 | 63 | readme.md section #37 |  |  | 0.588 |
| walker |  | 9788 | 47 | export doc at source/types/options.ts:307 |  |  | 0.588 |
| ns | 9911 |  | 379 | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names | 7.2 |  | 0.578 |
| ns | 9987 |  | 76 | test/retry.ts: 4 most-distinctive test names (truncated) | 7.3 |  | 0.576 |
