Score(3000)=0.597 I=0.853 C=0.418 ns_rows≤3K=19/52 grid(1000/1442/2080/3000/4327/6240/9000)=0.478/0.572/0.697/0.597/0.648/0.639/0.609

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | listing of '.' |  |  | 0.000 |
| walker |  | 52 | 12 | listing of 'media' |  |  | 0.000 |
| walker |  | 68 | 16 | listing of 'source' |  |  | 0.000 |
| walker |  | 76 | 8 | listing of 'source/core' |  |  | 0.000 |
| ns | 81 |  | 81 | Readme lede: what ky is, what it targets | 1.1 |  | 0.000 |
| walker |  | 97 | 21 | listing of 'source/errors' |  |  | 0.000 |
| ns | 121 |  | 40 | Complete repository root listing | 1.2 |  | 0.644 |
| walker |  | 130 | 33 | listing of 'source/types' |  |  | 0.702 |
| walker |  | 168 | 38 | listing of 'source/utils' |  |  | 0.781 |
| ns | 237 |  | 116 | Complete `source/` tree: every library file | 1.3 |  | 0.736 |
| walker |  | 239 | 71 | package identity in package.json |  |  | 0.746 |
| walker |  | 251 | 12 | ts names source/core/Ky.ts |  |  | 0.746 |
| walker |  | 262 | 11 | listing of '.github' |  |  | 0.746 |
| walker |  | 266 | 4 | listing of '.github/workflows' |  |  | 0.747 |
| ns | 313 |  | 76 | package.json identity: name, version, description, license, repository | 1.4 |  | 0.726 |
| ns | 415 |  | 102 | Every H2 section heading in readme.md | 1.5 |  | 0.648 |
| ns | 535 |  | 120 | "Benefits over plain fetch" feature bullets | 1.6 | 1.5 | 0.589 |
| walker |  | 601 | 335 | YAML config at .github/workflows/main.yml |  |  | 0.589 |
| walker |  | 615 | 14 | ts names source/types/ky.ts |  |  | 0.589 |
| walker |  | 630 | 15 | ts names source/errors/NonError.ts |  |  | 0.590 |
| walker |  | 645 | 15 | ts names source/errors/TimeoutError.ts |  |  | 0.590 |
| walker |  | 661 | 16 | ts names source/errors/ForceRetryError.ts |  |  | 0.590 |
| ns | 669 |  | 134 | source/index.ts: the runtime exports (default `ky`, error classes, type guards) | 1.7 |  | 0.535 |
| walker |  | 677 | 16 | ts names source/types/request.ts |  |  | 0.535 |
| walker |  | 693 | 16 | ts names source/types/response.ts |  |  | 0.535 |
| walker |  | 724 | 31 | package runtime metadata in package.json |  |  | 0.536 |
| walker |  | 742 | 18 | ts names source/types/ResponsePromise.ts |  |  | 0.536 |
| ns | 751 |  | 82 | Readme usage example: the canonical call shape | 1.8 | 1.5 | 0.507 |
| walker |  | 761 | 19 | ts names source/errors/HTTPError.ts |  |  | 0.507 |
| walker |  | 782 | 21 | ts decl source/types/request.ts:1 |  |  | 0.507 |
| walker |  | 832 | 50 | ts decl source/core/Ky.ts:33 |  |  | 0.507 |
| walker |  | 854 | 22 | ts decl source/types/response.ts:1 |  |  | 0.507 |
| ns | 873 |  | 122 | package.json module contract: type, exports, main, engines | 1.9 | 1.4 | 0.477 |
| walker |  | 880 | 26 | ts names source/utils/delay.ts |  |  | 0.478 |
| walker |  | 895 | 15 | ts decl source/utils/delay.ts:5 |  |  | 0.478 |
| walker |  | 921 | 26 | ts names source/utils/timeout.ts |  |  | 0.478 |
| walker |  | 944 | 23 | ts decl source/utils/timeout.ts:3 |  |  | 0.478 |
| walker |  | 971 | 27 | ts names source/types/retry.ts |  |  | 0.478 |
| ns | 1123 |  | 250 | source/index.ts: the complete public type-export block | 1.10 | 1.7 | 0.415 |
| walker |  | 1154 | 183 | README headline in readme.md |  |  | 0.521 |
| walker |  | 1195 | 41 | README prelude in readme.md |  |  | 0.625 |
| ns | 1221 |  | 98 | Complete `test/` tree, including helpers | 1.11 |  | 0.571 |
| walker |  | 1223 | 28 | ts decl source/utils/delay.ts:9 |  |  | 0.571 |
| walker |  | 1253 | 30 | ts names source/utils/is.ts |  |  | 0.572 |
| walker |  | 1283 | 30 | ts decl source/errors/TimeoutError.ts:3 |  |  | 0.572 |
| walker |  | 1317 | 34 | ts names source/utils/types.ts |  |  | 0.572 |
| walker |  | 1335 | 18 | ts decl source/utils/types.ts:1 |  |  | 0.572 |
| walker |  | 1372 | 37 | ts names source/utils/options.ts |  |  | 0.572 |
| walker |  | 1403 | 31 | ts decl source/utils/options.ts:4 |  |  | 0.572 |
| walker |  | 1439 | 36 | ts decl source/utils/types.ts:5 |  |  | 0.572 |
| ns | 1461 |  | 240 | Every H3 heading in readme.md (API entries and tips) | 1.12 | 1.5 | 0.531 |
| ns | 1731 |  | 270 | Readme `ky(input, options?)` contract and the body shortcuts | 1.13 | 1.12 | 0.526 |
| walker |  | 1849 | 410 | ts names source/index.ts |  |  | 0.708 |
| walker |  | 1890 | 41 | ts decl source/errors/NonError.ts:6 |  |  | 0.708 |
| walker |  | 1938 | 48 | ts names source/utils/normalize.ts |  |  | 0.708 |
| walker |  | 1960 | 22 | ts decl source/utils/normalize.ts:5 |  |  | 0.708 |
| ns | 1983 |  | 252 | `KyInstance`: every member signature (types/ky.ts) | 2.1 |  | 0.656 |
| walker |  | 2026 | 66 | listing of 'test' |  |  | 0.697 |
| walker |  | 2074 | 48 | ts decl source/utils/timeout.ts:9 |  |  | 0.697 |
| ns | 2094 |  | 111 | `ResponsePromise`: all six body-shortcut signatures | 2.2 |  | 0.677 |
| walker |  | 2179 | 105 | package entrypoints in package.json |  |  | 0.716 |
| walker |  | 2280 | 101 | package scripts in package.json |  |  | 0.716 |
| ns | 2326 |  | 232 | `KyOptions`: every ky-specific option with its type | 2.3 |  | 0.665 |
| walker |  | 2342 | 62 | ts decl source/errors/HTTPError.ts:5 |  |  | 0.665 |
| walker |  | 2411 | 69 | ts decl source/errors/ForceRetryError.ts:8 |  |  | 0.666 |
| walker |  | 2497 | 86 | ts names source/types/common.ts |  |  | 0.666 |
| walker |  | 2518 | 21 | ts decl source/types/common.ts:6 |  |  | 0.666 |
| walker |  | 2600 | 82 | ts decl source/types/retry.ts:3 |  |  | 0.636 |
| ns | 2600 |  | 274 | Concrete retry defaults (`defaultRetryOptions`, utils/normalize.ts) | 2.4 |  | 0.636 |
| walker |  | 2689 | 89 | ts names source/utils/body.ts |  |  | 0.637 |
| walker |  | 2786 | 97 | ts names source/utils/type-guards.ts |  |  | 0.638 |
| ns | 2865 |  | 265 | Where the non-retry defaults are applied (core/Ky.ts constructor) | 2.5 |  | 0.609 |
| walker |  | 2905 | 119 | ts names source/utils/merge.ts |  |  | 0.609 |
| ns | 2934 |  | 69 | `Hooks`: the four hook arrays | 2.6 |  | 0.597 |
| walker |  | 2999 | 94 | ts decl source/utils/merge.ts:38 |  |  | 0.597 |
| ns | 3139 |  | 205 | `RetryOptions`: every retry field (types/retry.ts) | 2.7 |  | 0.570 |
| walker |  | 3339 | 340 | headings outline in readme.md |  |  | 0.655 |
| walker |  | 3355 | 16 | readme.md section #50 |  |  | 0.655 |
| walker |  | 3375 | 20 | readme.md section #49 |  |  | 0.655 |
| walker |  | 3390 | 15 | readme.md section #22 |  |  | 0.655 |
| walker |  | 3408 | 18 | readme.md section #7 |  |  | 0.655 |
| walker |  | 3427 | 19 | readme.md section #11 |  |  | 0.655 |
| ns | 3480 |  | 341 | Hook function signatures and their `*State` objects | 2.8 | 2.6 | 0.618 |
| walker |  | 3509 | 82 | json config tsconfig.json |  |  | 0.618 |
| walker |  | 3541 | 32 | listing of 'test/helpers' |  |  | 0.643 |
| walker |  | 3562 | 21 | readme.md section #13 |  |  | 0.643 |
| walker |  | 3582 | 20 | readme.md section #12 |  |  | 0.643 |
| walker |  | 3603 | 21 | ts body source/utils/type-guards.ts:49 |  |  | 0.643 |
| walker |  | 3624 | 21 | ts body source/utils/type-guards.ts:71 |  |  | 0.643 |
| walker |  | 3646 | 22 | readme.md section #18 |  |  | 0.643 |
| walker |  | 3669 | 23 | ts body source/utils/type-guards.ts:27 |  |  | 0.643 |
| walker |  | 3692 | 23 | ts body source/utils/type-guards.ts:98 |  |  | 0.643 |
| walker |  | 3714 | 22 | readme.md section #45 |  |  | 0.643 |
| ns | 3745 |  | 265 | Public error classes: `HTTPError`, `TimeoutError`, `ForceRetryError` | 2.9 |  | 0.636 |
| walker |  | 3808 | 94 | readme.md section #1 |  |  | 0.636 |
| walker |  | 3977 | 169 | ts names source/types/hooks.ts |  |  | 0.644 |
| ns | 4021 |  | 276 | `Options` and `NormalizedOptions` interfaces | 2.10 |  | 0.627 |
| walker |  | 4023 | 46 | ts decl source/types/hooks.ts:14 |  |  | 0.634 |
| walker |  | 4080 | 57 | ts decl source/types/hooks.ts:41 |  |  | 0.647 |
| walker |  | 4168 | 88 | ts decl source/types/hooks.ts:20 |  |  | 0.662 |
| ns | 4199 |  | 178 | `ky.stop`, `ky.retry()` and the `ForceRetryOptions` fields | 2.11 |  | 0.641 |
| walker |  | 4259 | 91 | ts decl source/types/hooks.ts:32 |  |  | 0.648 |
| walker |  | 4352 | 93 | ts decl source/types/hooks.ts:5 |  |  | 0.654 |
| ns | 4370 |  | 171 | `requestMethods`, `responseTypes` and `maxSafeTimeout` | 2.12 |  | 0.643 |
| walker |  | 4447 | 95 | ts decl source/types/hooks.ts:48 |  |  | 0.650 |
| walker |  | 4517 | 70 | readme.md section #52 |  |  | 0.650 |
| ns | 4571 |  | 201 | Option registries: `kyOptionKeys`, `requestOptionsRegistry`, `vendorSpecificOptions` | 2.13 |  | 0.631 |
| ns | 4784 |  | 213 | Core public type aliases: `Input`, `Progress`, search-param and method types | 2.14 |  | 0.621 |
| walker |  | 4842 | 325 | ts names source/core/constants.ts |  |  | 0.630 |
| walker |  | 4860 | 18 | ts decl source/core/constants.ts:148 |  |  | 0.632 |
| walker |  | 4888 | 28 | ts decl source/core/constants.ts:256 |  |  | 0.635 |
| walker |  | 4998 | 110 | ts decl source/core/constants.ts:46 |  |  | 0.650 |
| ns | 5042 |  | 258 | `Ky` class: every field and method, name-only (core/Ky.ts) | 3.1 |  | 0.623 |
| walker |  | 5122 | 124 | ts decl source/core/constants.ts:237 |  |  | 0.648 |
| ns | 5148 |  | 106 | Runtime capability flags exported by core/constants.ts | 3.2 |  | 0.650 |
| walker |  | 5251 | 129 | ts decl source/core/constants.ts:265 |  |  | 0.653 |
| ns | 5339 |  | 191 | Every exported symbol of `source/utils/` (merge, normalize, options) | 3.3 |  | 0.649 |
| walker |  | 5554 | 303 | ts body source/index.ts:10 |  |  | 0.651 |
| walker |  | 5629 | 75 | readme.md section #51 |  |  | 0.651 |
| ns | 5633 |  | 294 | Every exported symbol of `source/utils/` (timeout, delay, body, guards, misc) | 3.4 | 3.3 | 0.659 |
| walker |  | 5662 | 33 | readme.md section #8 |  |  | 0.659 |
| walker |  | 5699 | 37 | readme.md section #19 |  |  | 0.659 |
| ns | 5813 |  | 180 | Every readme option anchor (`##### <option>`) and hook anchor | 3.5 |  | 0.647 |
| walker |  | 5998 | 299 | package identity metadata in package.json |  |  | 0.651 |
| walker |  | 6045 | 47 | ts doc source/errors/ForceRetryError.ts:8 |  |  | 0.651 |
| ns | 6049 |  | 236 | `test/helpers/`: every exported test helper | 3.6 |  | 0.639 |
| ns | 6360 |  | 311 | Remaining readme H4/H6 headings (input, defaultOptions, ky.retry options, CDN, FAQ) | 3.7 |  | 0.626 |
| walker |  | 6406 | 361 | ts names source/types/options.ts |  |  | 0.635 |
| ns | 6456 |  | 96 | The internal-only `NonError` wrapper | 3.8 |  | 0.632 |
| walker |  | 6479 | 73 | ts decl source/types/options.ts:16 |  |  | 0.640 |
| walker |  | 6616 | 137 | ts decl source/types/options.ts:373 |  |  | 0.654 |
| ns | 6748 |  | 292 | `createInstance`: how `ky`, the method shortcuts, `create` and `extend` are built | 4.1 |  | 0.662 |
| walker |  | 6761 | 145 | ts decl source/types/options.ts:358 |  |  | 0.662 |
| walker |  | 6790 | 29 | ts doc source/types/options.ts:373 |  |  | 0.662 |
| ns | 6965 |  | 217 | `#calculateRetryDelay`: limit, non-Error wrapping, forced and method checks | 4.2 | 3.1 | 0.652 |
| walker |  | 7107 | 317 | ts decl source/core/constants.ts:4 |  |  | 0.653 |
| walker |  | 7164 | 57 | ts doc source/errors/NonError.ts:6 |  |  | 0.653 |
| ns | 7168 |  | 203 | `#calculateRetryDelay`: the `shouldRetry` predicate contract | 4.3 | 4.2 | 0.644 |
| walker |  | 7212 | 48 | readme.md section #3 |  |  | 0.644 |
| walker |  | 7260 | 48 | readme.md section #28 |  |  | 0.644 |
| walker |  | 7309 | 49 | readme.md section #14 |  |  | 0.644 |
| walker |  | 7360 | 51 | readme.md section #27 |  |  | 0.644 |
| ns | 7600 |  | 432 | `#calculateRetryDelay`: timeouts, status codes and `Retry-After` parsing | 4.4 | 4.3 | 0.628 |
| walker |  | 7772 | 412 | ts decl source/types/ResponsePromise.ts:6 |  |  | 0.640 |
| ns | 8012 |  | 412 | `#retry`: the recursive retry loop and the `beforeRetry` hook contract | 4.5 | 3.1 | 0.623 |
| walker |  | 8194 | 422 | ts decl source/types/options.ts:312 |  |  | 0.629 |
| walker |  | 8221 | 27 | ts doc source/types/options.ts:312 |  |  | 0.622 |
| ns | 8221 |  | 209 | Constructor: input validation and `prefixUrl` joining | 4.6 | 2.5 | 0.622 |
| walker |  | 8266 | 45 | readme.md section #32 |  |  | 0.622 |
| walker |  | 8307 | 41 | ts body source/errors/TimeoutError.ts:6 |  |  | 0.627 |
| walker |  | 8353 | 46 | ts names test/main.ts |  |  | 0.627 |
| walker |  | 8383 | 30 | ts doc source/core/constants.ts:148 |  |  | 0.631 |
| ns | 8398 |  | 177 | Where a non-2xx response becomes an `HTTPError` | 4.7 | 3.1 | 0.624 |
| walker |  | 8467 | 84 | ts body source/utils/merge.ts:6 |  |  | 0.625 |
| walker |  | 8530 | 63 | readme.md section #37 |  |  | 0.625 |
| ns | 8553 |  | 155 | `mergeHeaders`: how `.extend()` removes a header | 5.1 | 3.3 | 0.618 |
| walker |  | 8642 | 112 | json config tsconfig.dist.json |  |  | 0.619 |
| walker |  | 8719 | 77 | readme.md section #31 |  |  | 0.619 |
| ns | 8726 |  | 173 | `mergeHooks` / `newHookValue`: hook array inheritance | 5.2 | 3.3 | 0.618 |
| walker |  | 8798 | 79 | readme.md section #23 |  |  | 0.618 |
| ns | 9000 |  | 274 | `deepMerge`: signal collection, shallow `context`, `searchParams` accumulation | 5.3 | 3.3 | 0.609 |
| walker |  | 9017 | 219 | readme.md section #46 |  |  | 0.611 |
| ns | 9125 |  | 125 | Install instructions, CDN entry points and the Deno import | 6.1 | 3.7 | 0.610 |
| walker |  | 9273 | 256 | readme.md section #47 |  |  | 0.611 |
| ns | 9282 |  | 157 | Support matrix, related packages and maintainers | 6.2 | 1.5 | 0.614 |
| ns | 9370 |  | 88 | "Extending types": why ky uses type aliases | 6.3 | 1.12 | 0.613 |
| ns | 9473 |  | 103 | npm scripts: how to build, test and debug | 7.1 | 1.9 | 0.616 |
| ns | 9500 |  | 27 | Remaining root directories: `.github/` and `media/` | 7.2 |  | 0.619 |
| walker |  | 9524 | 251 | ts decl source/core/constants.ts:68 |  |  | 0.623 |
| walker |  | 9547 | 23 | ts doc source/core/constants.ts:68 |  |  | 0.627 |
| walker |  | 9682 | 135 | ts body source/utils/body.ts:119 |  |  | 0.628 |
| ns | 9694 |  | 194 | TypeScript configuration (both tsconfigs, complete) | 7.3 | 7.1 | 0.636 |
| walker |  | 9734 | 52 | ts doc source/types/options.ts:307 |  |  | 0.636 |
| walker |  | 9829 | 95 | readme.md section #26 |  |  | 0.636 |
| ns | 9962 |  | 268 | AVA configuration and the shape of a typical test | 7.4 | 3.6 | 0.626 |
| walker |  | 9981 | 152 | ts body source/utils/merge.ts:16 |  |  | 0.638 |
