Score(3000)=0.541 I=0.813 C=0.360 ns_rows≤3K=17/38 (reached=7 partial=0 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | listing of '.' |  |  | 1.000 |
| ns | 40 |  | 40 | Repo root listing | 1.1 |  | 1.000 |
| ns | 79 |  | 39 | Readme one-line tagline | 1.2 |  | 0.953 |
| walker |  | 105 | 65 | package identity in package.json |  |  | 0.955 |
| walker |  | 117 | 12 | listing of 'media' |  |  | 0.955 |
| walker |  | 133 | 16 | listing of 'source' |  |  | 0.958 |
| walker |  | 143 | 10 | export names surface in source/index.ts |  |  | 0.959 |
| walker |  | 151 | 8 | listing of 'source/core' |  |  | 0.961 |
| ns | 163 |  | 84 | Readme top-level section headings (## only) | 1.3 |  | 0.681 |
| ns | 204 |  | 41 | Readme target environments + 'no deps' note | 1.4 |  | 0.633 |
| walker |  | 323 | 172 | README headline in readme.md |  |  | 0.703 |
| ns | 331 |  | 127 | Readme benefits-over-fetch list | 1.5 | 1.3 | 0.745 |
| walker |  | 344 | 21 | listing of 'source/errors' |  |  | 0.750 |
| walker |  | 357 | 13 | export names surface in source/errors/TimeoutError.ts |  |  | 0.750 |
| walker |  | 377 | 20 | export at source/errors/TimeoutError.ts:3 |  |  | 0.750 |
| walker |  | 391 | 14 | export names surface in source/errors/ForceRetryError.ts |  |  | 0.750 |
| walker |  | 408 | 17 | export names surface in source/errors/HTTPError.ts |  |  | 0.750 |
| ns | 447 |  | 116 | source/ tree (all immediate children + every subdir) | 1.6 |  | 0.583 |
| walker |  | 460 | 52 | export at source/errors/HTTPError.ts:5 |  |  | 0.584 |
| walker |  | 470 | 10 | export names surface in source/core/Ky.ts |  |  | 0.584 |
| walker |  | 529 | 59 | export at source/errors/ForceRetryError.ts:8 |  |  | 0.584 |
| ns | 545 |  | 98 | test/ tree (top-level + helpers/) | 1.7 |  | 0.496 |
| walker |  | 568 | 39 | export body at source/errors/TimeoutError.ts:3 body 7 |  |  | 0.496 |
| walker |  | 581 | 13 | export names surface in source/errors/NonError.ts |  |  | 0.496 |
| walker |  | 614 | 33 | listing of 'source/types' |  |  | 0.576 |
| walker |  | 626 | 12 | export names surface in source/types/ky.ts |  |  | 0.576 |
| walker |  | 642 | 16 | export names surface in source/types/ResponsePromise.ts |  |  | 0.576 |
| walker |  | 658 | 16 | export names surface in source/types/request.ts |  |  | 0.576 |
| walker |  | 674 | 16 | export names surface in source/types/response.ts |  |  | 0.576 |
| walker |  | 699 | 25 | export names surface in source/types/retry.ts |  |  | 0.577 |
| ns | 714 |  | 169 | package.json identity (name, version, description, exports, engines) | 1.8 |  | 0.535 |
| walker |  | 720 | 21 | export at source/types/request.ts:1 |  |  | 0.535 |
| walker |  | 742 | 22 | export at source/types/response.ts:1 |  |  | 0.535 |
| walker |  | 780 | 38 | listing of 'source/utils' |  |  | 0.651 |
| walker |  | 811 | 31 | export at source/errors/NonError.ts:6 |  |  | 0.651 |
| walker |  | 826 | 15 | imports in source/types/ResponsePromise.ts |  |  | 0.651 |
| walker |  | 841 | 15 | imports in source/types/retry.ts |  |  | 0.651 |
| ns | 845 |  | 131 | Readme `## API` H3 location index | 1.9 |  | 0.606 |
| walker |  | 941 | 100 | imports in source/index.ts |  |  | 0.608 |
| ns | 1051 |  | 206 | source/core/constants.ts: requestMethods + responseTypes + maxSafeTimeout + stop | 2.1 |  | 0.566 |
| ns | 1133 |  | 82 | Readme Usage block (canonical example) | 2.2 |  | 0.546 |
| walker |  | 1272 | 331 | export names surface in source/types/options.ts |  |  | 0.548 |
| walker |  | 1272 | 0 | export at source/types/options.ts:307 |  |  | 0.548 |
| walker |  | 1288 | 16 | imports in source/errors/TimeoutError.ts |  |  | 0.548 |
| ns | 1356 |  | 223 | KyInstance: every member's signature (location batch) | 2.3 |  | 0.522 |
| walker |  | 1370 | 82 | json config tsconfig.json |  |  | 0.522 |
| walker |  | 1394 | 24 | export names surface in source/utils/delay.ts |  |  | 0.522 |
| walker |  | 1409 | 15 | export at source/utils/delay.ts:5 |  |  | 0.522 |
| walker |  | 1437 | 28 | export at source/utils/delay.ts:9 |  |  | 0.522 |
| walker |  | 1461 | 24 | export names surface in source/utils/timeout.ts |  |  | 0.522 |
| walker |  | 1482 | 21 | export at source/utils/timeout.ts:3 |  |  | 0.522 |
| walker |  | 1530 | 48 | export at source/utils/timeout.ts:9 |  |  | 0.522 |
| ns | 1768 |  | 412 | source/index.ts named-export block | 2.4 |  | 0.456 |
| walker |  | 1865 | 335 | headings outline in readme.md |  |  | 0.582 |
| walker |  | 1877 | 12 | readme.md section #48 |  |  | 0.582 |
| walker |  | 1893 | 16 | readme.md section #47 |  |  | 0.582 |
| walker |  | 1908 | 15 | readme.md section #22 |  |  | 0.582 |
| walker |  | 1925 | 17 | readme.md section #18 |  |  | 0.582 |
| walker |  | 1943 | 18 | readme.md section #7 |  |  | 0.582 |
| ns | 1953 |  | 185 | source/index.ts imports + createInstance signature + default export | 2.5 |  | 0.571 |
| walker |  | 1962 | 19 | readme.md section #11 |  |  | 0.571 |
| walker |  | 1981 | 19 | readme.md section #13 |  |  | 0.571 |
| walker |  | 2062 | 81 | readme.md section #1 |  |  | 0.571 |
| walker |  | 2090 | 28 | export names surface in source/utils/is.ts |  |  | 0.571 |
| walker |  | 2112 | 22 | readme.md section #12 |  |  | 0.571 |
| ns | 2246 |  | 293 | source/index.ts createInstance body | 2.6 | 2.5 | 0.541 |
| walker |  | 2413 | 301 | package identity metadata in package.json |  |  | 0.541 |
| ns | 2473 |  | 227 | Default ky() body-method behavior + body shortcuts list | 2.7 |  | 0.537 |
| walker |  | 2518 | 105 | package entrypoints in package.json |  |  | 0.569 |
| walker |  | 2547 | 29 | package runtime metadata in package.json |  |  | 0.584 |
| walker |  | 2650 | 103 | package scripts in package.json |  |  | 0.584 |
| walker |  | 2745 | 95 | export names surface in source/utils/type-guards.ts |  |  | 0.584 |
| walker |  | 2745 | 0 | export at source/utils/type-guards.ts:27 |  |  | 0.584 |
| walker |  | 2745 | 0 | export at source/utils/type-guards.ts:49 |  |  | 0.584 |
| walker |  | 2745 | 0 | export at source/utils/type-guards.ts:71 |  |  | 0.584 |
| walker |  | 2745 | 0 | export at source/utils/type-guards.ts:98 |  |  | 0.584 |
| walker |  | 2764 | 19 | export body at source/utils/type-guards.ts:49 body 50 |  |  | 0.584 |
| walker |  | 2783 | 19 | export body at source/utils/type-guards.ts:71 body 72 |  |  | 0.585 |
| walker |  | 2804 | 21 | export body at source/utils/type-guards.ts:27 body 28 |  |  | 0.585 |
| walker |  | 2825 | 21 | export body at source/utils/type-guards.ts:98 body 99 |  |  | 0.585 |
| walker |  | 2847 | 22 | readme.md section #45 |  |  | 0.585 |
| ns | 2901 |  | 428 | core/constants.ts: feature-detection flags | 2.8 |  | 0.540 |
| walker |  | 2913 | 66 | export at source/types/options.ts:16 |  |  | 0.541 |
| walker |  | 3025 | 112 | json config tsconfig.dist.json |  |  | 0.541 |
| walker |  | 3161 | 136 | export body at source/errors/HTTPError.ts:5 body 11 |  |  | 0.542 |
| ns | 3186 |  | 285 | ForceRetryOptions type + RetryMarker + retry() factory signature | 2.9 |  | 0.511 |
| walker |  | 3195 | 34 | export names surface in source/utils/types.ts |  |  | 0.511 |
| walker |  | 3213 | 18 | export at source/utils/types.ts:1 |  |  | 0.511 |
| walker |  | 3242 | 29 | plaintext config .gitignore |  |  | 0.511 |
| walker |  | 3277 | 35 | export names surface in source/utils/options.ts |  |  | 0.511 |
| walker |  | 3277 | 0 | export at source/utils/options.ts:30 |  |  | 0.511 |
| walker |  | 3308 | 31 | export at source/utils/options.ts:4 |  |  | 0.511 |
| walker |  | 3385 | 77 | export at source/types/retry.ts:3 |  |  | 0.511 |
| walker |  | 3451 | 66 | readme.md section #50 |  |  | 0.511 |
| walker |  | 3522 | 71 | readme.md section #49 |  |  | 0.511 |
| walker |  | 3569 | 47 | export doc at source/errors/ForceRetryError.ts:8 |  |  | 0.511 |
| walker |  | 3601 | 32 | imports in source/errors/ForceRetryError.ts |  |  | 0.511 |
| ns | 3658 |  | 472 | kyOptionKeys + vendor/request option registries | 2.10 |  | 0.471 |
| walker |  | 3774 | 173 | export body at source/errors/ForceRetryError.ts:8 body 15 |  |  | 0.472 |
| walker |  | 3807 | 33 | readme.md section #8 |  |  | 0.472 |
| walker |  | 3853 | 46 | export names surface in source/utils/normalize.ts |  |  | 0.472 |
| walker |  | 3853 | 0 | export at source/utils/normalize.ts:28 |  |  | 0.472 |
| ns | 3855 |  | 197 | KyOptions: every option's name + type signature (location batch) | 3.1 |  | 0.462 |
| walker |  | 3873 | 20 | export at source/utils/normalize.ts:5 |  |  | 0.462 |
| walker |  | 4005 | 132 | export at source/types/options.ts:373 |  |  | 0.462 |
| ns | 4024 |  | 169 | RetryOptions: every field + ShouldRetryState (location batch) | 3.2 |  | 0.454 |
| walker |  | 4166 | 161 | export names surface in source/types/hooks.ts |  |  | 0.454 |
| walker |  | 4212 | 46 | export at source/types/hooks.ts:14 |  |  | 0.455 |
| walker |  | 4269 | 57 | export at source/types/hooks.ts:41 |  |  | 0.456 |
| ns | 4324 |  | 300 | Hooks types: every state + hook type alias (location batch) | 3.3 |  | 0.474 |
| walker |  | 4352 | 83 | export at source/types/hooks.ts:20 |  |  | 0.474 |
| walker |  | 4438 | 86 | export at source/types/hooks.ts:32 |  |  | 0.474 |
| walker |  | 4526 | 88 | export at source/types/hooks.ts:5 |  |  | 0.474 |
| ns | 4587 |  | 263 | Type aliases: Input, SearchParams*, Progress, KyHeadersInit, RequestHttpMethod, HttpMethod | 3.4 |  | 0.489 |
| walker |  | 4616 | 90 | export at source/types/hooks.ts:48 |  |  | 0.489 |
| walker |  | 4653 | 37 | readme.md section #19 |  |  | 0.489 |
| walker |  | 4719 | 66 | listing of 'test' |  |  | 0.517 |
| walker |  | 4751 | 32 | listing of 'test/helpers' |  |  | 0.543 |
| walker |  | 5122 | 371 | package dependencies in package.json |  |  | 0.543 |
| ns | 5127 |  | 540 | Options interface (extends KyOptions + RequestInit) + InternalOptions + NormalizedOptions | 3.5 | 3.1 | 0.529 |
| walker |  | 5158 | 36 | export at source/utils/types.ts:5 |  |  | 0.529 |
| walker |  | 5204 | 46 | imports in source/types/options.ts |  |  | 0.529 |
| ns | 5252 |  | 125 | ResponsePromise type signature (no examples) | 3.6 |  | 0.521 |
| walker |  | 5298 | 94 | plaintext config .editorconfig |  |  | 0.521 |
| walker |  | 5347 | 49 | imports in source/errors/HTTPError.ts |  |  | 0.521 |
| walker |  | 5397 | 50 | imports in source/types/ky.ts |  |  | 0.521 |
| walker |  | 5428 | 31 | export doc at source/types/options.ts:373 |  |  | 0.526 |
| ns | 5448 |  | 196 | KyRequest + KyResponse + common Primitive/LiteralUnion types | 3.7 |  | 0.520 |
| walker |  | 5571 | 143 | export at source/types/options.ts:358 |  |  | 0.540 |
| walker |  | 5624 | 53 | imports in source/utils/type-guards.ts |  |  | 0.540 |
| walker |  | 5670 | 46 | readme.md section #28 |  |  | 0.540 |
| walker |  | 5686 | 16 | imports in source/utils/delay.ts |  |  | 0.540 |
| walker |  | 5741 | 55 | imports in source/types/hooks.ts |  |  | 0.540 |
| walker |  | 5789 | 48 | readme.md section #3 |  |  | 0.540 |
| ns | 5804 |  | 356 | Ky class member declarations + every method signature (location batch) | 4.1 |  | 0.526 |
| walker |  | 5806 | 17 | imports in source/utils/timeout.ts |  |  | 0.526 |
| walker |  | 5855 | 49 | readme.md section #14 |  |  | 0.526 |
| walker |  | 5906 | 51 | readme.md section #27 |  |  | 0.526 |
| walker |  | 5984 | 78 | export names surface in source/types/common.ts |  |  | 0.537 |
| walker |  | 6005 | 21 | export at source/types/common.ts:6 |  |  | 0.542 |
| walker |  | 6092 | 87 | export names surface in source/utils/body.ts |  |  | 0.542 |
| walker |  | 6092 | 0 | export at source/utils/body.ts:5 |  |  | 0.542 |
| walker |  | 6092 | 0 | export at source/utils/body.ts:90 |  |  | 0.542 |
| walker |  | 6092 | 0 | export at source/utils/body.ts:119 |  |  | 0.542 |
| walker |  | 6239 | 147 | export body at source/utils/timeout.ts:9 body 15 |  |  | 0.543 |
| walker |  | 6394 | 155 | export body at source/utils/delay.ts:9 body 13 |  |  | 0.543 |
| walker |  | 6457 | 63 | readme.md section #37 |  |  | 0.543 |
| walker |  | 6504 | 47 | export doc at source/types/options.ts:307 |  |  | 0.543 |
| walker |  | 6621 | 117 | export names surface in source/utils/merge.ts |  |  | 0.543 |
| walker |  | 6621 | 0 | export at source/utils/merge.ts:6 |  |  | 0.543 |
| walker |  | 6621 | 0 | export at source/utils/merge.ts:16 |  |  | 0.543 |
| walker |  | 6621 | 0 | export at source/utils/merge.ts:86 |  |  | 0.543 |
| ns | 6709 |  | 905 | #calculateRetryDelay — full retry-decision logic | 4.2 | 4.1 | 0.502 |
| walker |  | 6713 | 92 | export at source/utils/merge.ts:38 |  |  | 0.502 |
| walker |  | 6790 | 77 | export body at source/utils/merge.ts:6 body 7 |  |  | 0.502 |
| ns | 6904 |  | 195 | type-guards.ts: isKyError + isHTTPError + isTimeoutError + isForceRetryError signatures | 5.1 |  | 0.507 |
| walker |  | 6951 | 161 | export body at source/errors/NonError.ts:6 body 11 |  |  | 0.507 |
| walker |  | 7028 | 77 | readme.md section #31 |  |  | 0.507 |
| walker |  | 7069 | 41 | readme.md section #32 |  |  | 0.507 |
| ns | 7223 |  | 319 | HTTPError + TimeoutError class bodies | 5.2 |  | 0.519 |
| walker |  | 7436 | 367 | export at source/types/options.ts:312 |  |  | 0.534 |
| walker |  | 7465 | 29 | export doc at source/types/options.ts:312 |  |  | 0.534 |
| walker |  | 7544 | 79 | readme.md section #23 |  |  | 0.534 |
| ns | 7546 |  | 323 | ForceRetryError class body + NonError signature | 5.3 |  | 0.546 |
| walker |  | 7576 | 32 | imports in source/utils/body.ts |  |  | 0.546 |
| walker |  | 7630 | 54 | export doc at source/errors/NonError.ts:6 |  |  | 0.546 |
| walker |  | 7753 | 123 | export body at source/utils/body.ts:119 body 120 |  |  | 0.546 |
| ns | 7855 |  | 309 | normalize.ts: defaultRetryOptions values + normalizeRequestMethod | 6.1 |  | 0.536 |
| walker |  | 8060 | 307 | readme.md section #2 |  |  | 0.545 |
| walker |  | 8417 | 357 | export at source/types/ResponsePromise.ts:6 |  |  | 0.554 |
| walker |  | 8582 | 165 | export doc at source/utils/type-guards.ts:49 |  |  | 0.554 |
| walker |  | 8648 | 66 | export names surface #1 in source/core/constants.ts |  |  | 0.554 |
| walker |  | 8648 | 0 | export at source/core/constants.ts:235 |  |  | 0.554 |
| walker |  | 8674 | 26 | export at source/core/constants.ts:256 |  |  | 0.555 |
| ns | 8679 |  | 824 | merge.ts deepMerge: special-cased keys (signal/context/searchParams/hooks/headers) | 6.2 |  | 0.527 |
| walker |  | 8796 | 122 | export at source/core/constants.ts:237 |  |  | 0.534 |
| ns | 8901 |  | 222 | timeout.ts body | 6.3 |  | 0.545 |
| walker |  | 8925 | 129 | export at source/core/constants.ts:265 |  |  | 0.558 |
| walker |  | 8965 | 40 | imports in source/core/constants.ts |  |  | 0.558 |
| ns | 9072 |  | 171 | options.ts utils + body.ts streaming exports + small util one-liners | 6.4 |  | 0.562 |
| walker |  | 9306 | 341 | export at source/core/Ky.ts:33 |  |  | 0.583 |
| walker |  | 9478 | 172 | export doc at source/utils/type-guards.ts:71 |  |  | 0.583 |
| ns | 9532 |  | 460 | test/main.ts: representative test names (truncated) | 7.1 |  | 0.569 |
| walker |  | 9573 | 95 | readme.md section #9 |  |  | 0.569 |
| walker |  | 9614 | 41 | imports in source/utils/options.ts |  |  | 0.569 |
| walker |  | 9711 | 97 | readme.md section #26 |  |  | 0.569 |
| walker |  | 9851 | 140 | export body at source/utils/merge.ts:16 body 17 |  |  | 0.569 |
| ns | 9911 |  | 379 | test/{http-error,methods,prefix-url,bytes,memory-leak,fetch,context}.ts test names | 7.2 |  | 0.558 |
| walker |  | 9950 | 99 | readme.md section #5 |  |  | 0.559 |
| walker |  | 9961 | 11 | listing of '.github' |  |  | 0.559 |
| walker |  | 9965 | 4 | listing of '.github/workflows' |  |  | 0.559 |
| ns | 9987 |  | 76 | test/retry.ts: 4 most-distinctive test names (truncated) | 7.3 |  | 0.557 |
