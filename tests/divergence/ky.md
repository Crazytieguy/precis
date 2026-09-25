Score(3000)=0.711 I=0.905 C=0.560 ns_rows≤3K=19/52 grid(1000/1442/2080/3000/4327/6240/9000)=0.599/0.571/0.737/0.711/0.661/0.604/0.579

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
| walker |  | 250 | 11 | listing of '.github' |  |  | 0.746 |
| walker |  | 254 | 4 | listing of '.github/workflows' |  |  | 0.747 |
| ns | 313 |  | 76 | package.json identity: name, version, description, license, repository | 1.4 |  | 0.726 |
| ns | 415 |  | 102 | Every H2 section heading in readme.md | 1.5 |  | 0.648 |
| ns | 535 |  | 120 | "Benefits over plain fetch" feature bullets | 1.6 | 1.5 | 0.589 |
| walker |  | 589 | 335 | YAML config at .github/workflows/main.yml |  |  | 0.589 |
| walker |  | 603 | 14 | ts names source/types/ky.ts |  |  | 0.589 |
| walker |  | 618 | 15 | ts names source/errors/TimeoutError.ts |  |  | 0.590 |
| walker |  | 634 | 16 | ts names source/errors/ForceRetryError.ts |  |  | 0.590 |
| walker |  | 650 | 16 | ts names source/types/request.ts |  |  | 0.590 |
| walker |  | 666 | 16 | ts names source/types/response.ts |  |  | 0.590 |
| ns | 669 |  | 134 | source/index.ts: the runtime exports (default `ky`, error classes, type guards) | 1.7 |  | 0.535 |
| walker |  | 697 | 31 | package runtime metadata in package.json |  |  | 0.536 |
| walker |  | 715 | 18 | ts names source/types/ResponsePromise.ts |  |  | 0.536 |
| walker |  | 734 | 19 | ts names source/errors/HTTPError.ts |  |  | 0.536 |
| ns | 751 |  | 82 | Readme usage example: the canonical call shape | 1.8 | 1.5 | 0.507 |
| walker |  | 755 | 21 | ts decl source/types/request.ts:1 |  |  | 0.507 |
| walker |  | 777 | 22 | ts decl source/types/response.ts:1 |  |  | 0.507 |
| walker |  | 804 | 27 | ts names source/types/retry.ts |  |  | 0.507 |
| ns | 873 |  | 122 | package.json module contract: type, exports, main, engines | 1.9 | 1.4 | 0.477 |
| walker |  | 987 | 183 | README headline in readme.md |  |  | 0.599 |
| walker |  | 1028 | 41 | README prelude in readme.md |  |  | 0.720 |
| walker |  | 1058 | 30 | ts decl source/errors/TimeoutError.ts:3 |  |  | 0.720 |
| ns | 1123 |  | 250 | source/index.ts: the complete public type-export block | 1.10 | 1.7 | 0.625 |
| ns | 1221 |  | 98 | Complete `test/` tree, including helpers | 1.11 |  | 0.571 |
| ns | 1461 |  | 240 | Every H3 heading in readme.md (API entries and tips) | 1.12 | 1.5 | 0.530 |
| walker |  | 1468 | 410 | ts names source/index.ts |  |  | 0.714 |
| walker |  | 1534 | 66 | listing of 'test' |  |  | 0.758 |
| walker |  | 1639 | 105 | package entrypoints in package.json |  |  | 0.801 |
| ns | 1731 |  | 270 | Readme `ky(input, options?)` contract and the body shortcuts | 1.13 | 1.12 | 0.793 |
| walker |  | 1740 | 101 | package scripts in package.json |  |  | 0.794 |
| walker |  | 1802 | 62 | ts decl source/errors/HTTPError.ts:5 |  |  | 0.794 |
| walker |  | 1871 | 69 | ts decl source/errors/ForceRetryError.ts:8 |  |  | 0.795 |
| walker |  | 1953 | 82 | ts decl source/types/retry.ts:3 |  |  | 0.795 |
| walker |  | 1965 | 12 | ts names source/core/Ky.ts |  |  | 0.795 |
| ns | 1983 |  | 252 | `KyInstance`: every member signature (types/ky.ts) | 2.1 |  | 0.737 |
| walker |  | 2062 | 97 | ts names source/utils/type-guards.ts |  |  | 0.737 |
| walker |  | 2077 | 15 | ts names source/errors/NonError.ts |  |  | 0.737 |
| ns | 2094 |  | 111 | `ResponsePromise`: all six body-shortcut signatures | 2.2 |  | 0.716 |
| ns | 2326 |  | 232 | `KyOptions`: every ky-specific option with its type | 2.3 |  | 0.664 |
| walker |  | 2417 | 340 | headings outline in readme.md |  |  | 0.765 |
| walker |  | 2433 | 16 | readme.md section #50 |  |  | 0.765 |
| walker |  | 2453 | 20 | readme.md section #49 |  |  | 0.765 |
| walker |  | 2468 | 15 | readme.md section #22 |  |  | 0.765 |
| walker |  | 2486 | 18 | readme.md section #7 |  |  | 0.765 |
| walker |  | 2505 | 19 | readme.md section #11 |  |  | 0.765 |
| walker |  | 2587 | 82 | json config tsconfig.json |  |  | 0.765 |
| ns | 2600 |  | 274 | Concrete retry defaults (`defaultRetryOptions`, utils/normalize.ts) | 2.4 |  | 0.731 |
| walker |  | 2619 | 32 | listing of 'test/helpers' |  |  | 0.761 |
| walker |  | 2640 | 21 | readme.md section #13 |  |  | 0.761 |
| walker |  | 2660 | 20 | readme.md section #12 |  |  | 0.761 |
| walker |  | 2681 | 21 | ts body source/utils/type-guards.ts:49 |  |  | 0.761 |
| walker |  | 2702 | 21 | ts body source/utils/type-guards.ts:71 |  |  | 0.761 |
| walker |  | 2724 | 22 | readme.md section #18 |  |  | 0.761 |
| walker |  | 2747 | 23 | ts body source/utils/type-guards.ts:27 |  |  | 0.761 |
| walker |  | 2770 | 23 | ts body source/utils/type-guards.ts:98 |  |  | 0.761 |
| walker |  | 2792 | 22 | readme.md section #45 |  |  | 0.761 |
| ns | 2865 |  | 265 | Where the non-retry defaults are applied (core/Ky.ts constructor) | 2.5 |  | 0.726 |
| walker |  | 2886 | 94 | readme.md section #1 |  |  | 0.726 |
| ns | 2934 |  | 69 | `Hooks`: the four hook arrays | 2.6 |  | 0.711 |
| walker |  | 2936 | 50 | ts decl source/core/Ky.ts:33 |  |  | 0.711 |
| walker |  | 3105 | 169 | ts names source/types/hooks.ts |  |  | 0.713 |
| ns | 3139 |  | 205 | `RetryOptions`: every retry field (types/retry.ts) | 2.7 |  | 0.679 |
| walker |  | 3151 | 46 | ts decl source/types/hooks.ts:14 |  |  | 0.680 |
| walker |  | 3208 | 57 | ts decl source/types/hooks.ts:41 |  |  | 0.681 |
| walker |  | 3296 | 88 | ts decl source/types/hooks.ts:20 |  |  | 0.683 |
| walker |  | 3387 | 91 | ts decl source/types/hooks.ts:32 |  |  | 0.683 |
| walker |  | 3480 | 93 | ts decl source/types/hooks.ts:5 |  |  | 0.702 |
| ns | 3480 |  | 341 | Hook function signatures and their `*State` objects | 2.8 | 2.6 | 0.702 |
| walker |  | 3575 | 95 | ts decl source/types/hooks.ts:48 |  |  | 0.709 |
| walker |  | 3601 | 26 | ts names source/utils/delay.ts |  |  | 0.709 |
| walker |  | 3616 | 15 | ts decl source/utils/delay.ts:5 |  |  | 0.710 |
| walker |  | 3642 | 26 | ts names source/utils/timeout.ts |  |  | 0.710 |
| walker |  | 3665 | 23 | ts decl source/utils/timeout.ts:3 |  |  | 0.710 |
| walker |  | 3693 | 28 | ts decl source/utils/delay.ts:9 |  |  | 0.710 |
| ns | 3745 |  | 265 | Public error classes: `HTTPError`, `TimeoutError`, `ForceRetryError` | 2.9 |  | 0.698 |
| walker |  | 3763 | 70 | readme.md section #52 |  |  | 0.698 |
| walker |  | 3793 | 30 | ts names source/utils/is.ts |  |  | 0.699 |
| ns | 4021 |  | 276 | `Options` and `NormalizedOptions` interfaces | 2.10 |  | 0.680 |
| walker |  | 4096 | 303 | ts body source/index.ts:10 |  |  | 0.682 |
| walker |  | 4171 | 75 | readme.md section #51 |  |  | 0.682 |
| ns | 4199 |  | 178 | `ky.stop`, `ky.retry()` and the `ForceRetryOptions` fields | 2.11 |  | 0.660 |
| walker |  | 4204 | 33 | readme.md section #8 |  |  | 0.660 |
| walker |  | 4238 | 34 | ts names source/utils/types.ts |  |  | 0.661 |
| walker |  | 4256 | 18 | ts decl source/utils/types.ts:1 |  |  | 0.661 |
| walker |  | 4293 | 37 | ts names source/utils/options.ts |  |  | 0.661 |
| walker |  | 4324 | 31 | ts decl source/utils/options.ts:4 |  |  | 0.661 |
| walker |  | 4360 | 36 | ts decl source/utils/types.ts:5 |  |  | 0.661 |
| ns | 4370 |  | 171 | `requestMethods`, `responseTypes` and `maxSafeTimeout` | 2.12 |  | 0.650 |
| walker |  | 4397 | 37 | readme.md section #19 |  |  | 0.650 |
| ns | 4571 |  | 201 | Option registries: `kyOptionKeys`, `requestOptionsRegistry`, `vendorSpecificOptions` | 2.13 |  | 0.631 |
| walker |  | 4696 | 299 | package identity metadata in package.json |  |  | 0.636 |
| walker |  | 4743 | 47 | ts doc source/errors/ForceRetryError.ts:8 |  |  | 0.636 |
| ns | 4784 |  | 213 | Core public type aliases: `Input`, `Progress`, search-param and method types | 2.14 |  | 0.626 |
| ns | 5042 |  | 258 | `Ky` class: every field and method, name-only (core/Ky.ts) | 3.1 |  | 0.600 |
| walker |  | 5104 | 361 | ts names source/types/options.ts |  |  | 0.611 |
| ns | 5148 |  | 106 | Runtime capability flags exported by core/constants.ts | 3.2 |  | 0.607 |
| walker |  | 5177 | 73 | ts decl source/types/options.ts:16 |  |  | 0.617 |
| walker |  | 5314 | 137 | ts decl source/types/options.ts:373 |  |  | 0.634 |
| ns | 5339 |  | 191 | Every exported symbol of `source/utils/` (merge, normalize, options) | 3.3 |  | 0.620 |
| walker |  | 5459 | 145 | ts decl source/types/options.ts:358 |  |  | 0.620 |
| walker |  | 5488 | 29 | ts doc source/types/options.ts:373 |  |  | 0.620 |
| walker |  | 5529 | 41 | ts decl source/errors/NonError.ts:6 |  |  | 0.620 |
| walker |  | 5577 | 48 | ts names source/utils/normalize.ts |  |  | 0.622 |
| walker |  | 5599 | 22 | ts decl source/utils/normalize.ts:5 |  |  | 0.624 |
| ns | 5633 |  | 294 | Every exported symbol of `source/utils/` (timeout, delay, body, guards, misc) | 3.4 | 3.3 | 0.625 |
| walker |  | 5647 | 48 | ts decl source/utils/timeout.ts:9 |  |  | 0.626 |
| walker |  | 5695 | 48 | readme.md section #3 |  |  | 0.627 |
| walker |  | 5743 | 48 | readme.md section #28 |  |  | 0.627 |
| walker |  | 5792 | 49 | readme.md section #14 |  |  | 0.627 |
| ns | 5813 |  | 180 | Every readme option anchor (`##### <option>`) and hook anchor | 3.5 |  | 0.615 |
| walker |  | 5843 | 51 | readme.md section #27 |  |  | 0.615 |
| ns | 6049 |  | 236 | `test/helpers/`: every exported test helper | 3.6 |  | 0.604 |
| walker |  | 6255 | 412 | ts decl source/types/ResponsePromise.ts:6 |  |  | 0.618 |
| ns | 6360 |  | 311 | Remaining readme H4/H6 headings (input, defaultOptions, ky.retry options, CDN, FAQ) | 3.7 |  | 0.606 |
| ns | 6456 |  | 96 | The internal-only `NonError` wrapper | 3.8 |  | 0.603 |
| walker |  | 6677 | 422 | ts decl source/types/options.ts:312 |  |  | 0.611 |
| walker |  | 6704 | 27 | ts doc source/types/options.ts:312 |  |  | 0.611 |
| ns | 6748 |  | 292 | `createInstance`: how `ky`, the method shortcuts, `create` and `extend` are built | 4.1 |  | 0.621 |
| walker |  | 6749 | 45 | readme.md section #32 |  |  | 0.621 |
| walker |  | 6790 | 41 | ts body source/errors/TimeoutError.ts:6 |  |  | 0.627 |
| walker |  | 6836 | 46 | ts names test/main.ts |  |  | 0.627 |
| walker |  | 6899 | 63 | readme.md section #37 |  |  | 0.627 |
| ns | 6965 |  | 217 | `#calculateRetryDelay`: limit, non-Error wrapping, forced and method checks | 4.2 | 3.1 | 0.617 |
| walker |  | 6985 | 86 | ts names source/types/common.ts |  |  | 0.617 |
| walker |  | 7006 | 21 | ts decl source/types/common.ts:6 |  |  | 0.617 |
| walker |  | 7118 | 112 | json config tsconfig.dist.json |  |  | 0.618 |
| ns | 7168 |  | 203 | `#calculateRetryDelay`: the `shouldRetry` predicate contract | 4.3 | 4.2 | 0.609 |
| walker |  | 7207 | 89 | ts names source/utils/body.ts |  |  | 0.616 |
| walker |  | 7284 | 77 | readme.md section #31 |  |  | 0.616 |
| walker |  | 7363 | 79 | readme.md section #23 |  |  | 0.616 |
| walker |  | 7582 | 219 | readme.md section #46 |  |  | 0.618 |
| ns | 7600 |  | 432 | `#calculateRetryDelay`: timeouts, status codes and `Retry-After` parsing | 4.4 | 4.3 | 0.603 |
| walker |  | 7838 | 256 | readme.md section #47 |  |  | 0.604 |
| walker |  | 7957 | 119 | ts names source/utils/merge.ts |  |  | 0.609 |
| ns | 8012 |  | 412 | `#retry`: the recursive retry loop and the `beforeRetry` hook contract | 4.5 | 3.1 | 0.593 |
| walker |  | 8051 | 94 | ts decl source/utils/merge.ts:38 |  |  | 0.594 |
| walker |  | 8103 | 52 | ts doc source/types/options.ts:307 |  |  | 0.594 |
| walker |  | 8198 | 95 | readme.md section #26 |  |  | 0.594 |
| ns | 8221 |  | 209 | Constructor: input validation and `prefixUrl` joining | 4.6 | 2.5 | 0.587 |
| ns | 8398 |  | 177 | Where a non-2xx response becomes an `HTTPError` | 4.7 | 3.1 | 0.581 |
| ns | 8553 |  | 155 | `mergeHeaders`: how `.extend()` removes a header | 5.1 | 3.3 | 0.575 |
| walker |  | 8558 | 360 | readme.md section #2 |  |  | 0.587 |
| walker |  | 8668 | 110 | readme.md section #9 |  |  | 0.588 |
| ns | 8726 |  | 173 | `mergeHooks` / `newHookValue`: hook array inheritance | 5.2 | 3.3 | 0.588 |
| walker |  | 8784 | 116 | readme.md section #5 |  |  | 0.588 |
| walker |  | 8901 | 117 | readme.md section #20 |  |  | 0.588 |
| ns | 9000 |  | 274 | `deepMerge`: signal collection, shallow `context`, `searchParams` accumulation | 5.3 | 3.3 | 0.579 |
| ns | 9125 |  | 125 | Install instructions, CDN entry points and the Deno import | 6.1 | 3.7 | 0.583 |
| walker |  | 9135 | 234 | readme.md section #48 |  |  | 0.587 |
| ns | 9282 |  | 157 | Support matrix, related packages and maintainers | 6.2 | 1.5 | 0.590 |
| walker |  | 9310 | 175 | ts doc source/utils/type-guards.ts:49 |  |  | 0.590 |
| ns | 9370 |  | 88 | "Extending types": why ky uses type aliases | 6.3 | 1.12 | 0.590 |
| ns | 9473 |  | 103 | npm scripts: how to build, test and debug | 7.1 | 1.9 | 0.593 |
| ns | 9500 |  | 27 | Remaining root directories: `.github/` and `media/` | 7.2 |  | 0.596 |
| walker |  | 9635 | 325 | ts names source/core/constants.ts |  |  | 0.606 |
| walker |  | 9653 | 18 | ts decl source/core/constants.ts:148 |  |  | 0.607 |
| walker |  | 9681 | 28 | ts decl source/core/constants.ts:256 |  |  | 0.608 |
| ns | 9694 |  | 194 | TypeScript configuration (both tsconfigs, complete) | 7.3 | 7.1 | 0.617 |
| walker |  | 9791 | 110 | ts decl source/core/constants.ts:46 |  |  | 0.625 |
| walker |  | 9915 | 124 | ts decl source/core/constants.ts:237 |  |  | 0.640 |
| ns | 9962 |  | 268 | AVA configuration and the shape of a typical test | 7.4 | 3.6 | 0.630 |
