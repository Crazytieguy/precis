Score(3000)=0.713 I=0.910 C=0.559 ns_rows≤3K=19/52 grid(1000/1442/2080/3000/4327/6240/9000)=0.808/0.917/0.881/0.713/0.625/0.550/0.594

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 52 | 12 | Fs::DirListing { dir: media } |  |  | 0.000 |
| walker |  | 68 | 16 | Fs::DirListing { dir: source } |  |  | 0.000 |
| walker |  | 76 | 8 | Fs::DirListing { dir: source/core } |  |  | 0.000 |
| ns | 81 |  | 81 | Readme lede: what ky is, what it targets | 1.1 |  | 0.000 |
| walker |  | 97 | 21 | Fs::DirListing { dir: source/errors } |  |  | 0.000 |
| ns | 121 |  | 40 | Complete repository root listing | 1.2 |  | 0.644 |
| walker |  | 130 | 33 | Fs::DirListing { dir: source/types } |  |  | 0.702 |
| walker |  | 168 | 38 | Fs::DirListing { dir: source/utils } |  |  | 0.781 |
| ns | 237 |  | 116 | Complete `source/` tree: every library file | 1.3 |  | 0.736 |
| walker |  | 239 | 71 | Json::Identity { file: package.json } |  |  | 0.746 |
| walker |  | 250 | 11 | Fs::DirListing { dir: .github } |  |  | 0.746 |
| walker |  | 254 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.747 |
| walker |  | 285 | 31 | Json::Runtime { file: package.json } |  |  | 0.748 |
| ns | 313 |  | 76 | package.json identity: name, version, description, license, repository | 1.4 |  | 0.727 |
| ns | 415 |  | 102 | Every H2 section heading in readme.md | 1.5 |  | 0.649 |
| walker |  | 514 | 229 | Markdown::ReadmeHeadline { file: readme.md } |  |  | 0.892 |
| ns | 535 |  | 120 | "Benefits over plain fetch" feature bullets | 1.6 | 1.5 | 0.893 |
| walker |  | 580 | 66 | Fs::DirListing { dir: test } |  |  | 0.900 |
| ns | 669 |  | 134 | source/index.ts: the runtime exports (default `ky`, error classes, type guards) | 1.7 |  | 0.818 |
| walker |  | 683 | 103 | Json::Scripts { file: package.json } |  |  | 0.818 |
| ns | 751 |  | 82 | Readme usage example: the canonical call shape | 1.8 | 1.5 | 0.774 |
| walker |  | 786 | 103 | Json::Entry { file: package.json } |  |  | 0.782 |
| ns | 873 |  | 122 | package.json module contract: type, exports, main, engines | 1.9 | 1.4 | 0.797 |
| ns | 1123 |  | 250 | source/index.ts: the complete public type-export block | 1.10 | 1.7 | 0.691 |
| walker |  | 1196 | 410 | Code::CodeKey { rung: Names, file: source/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.901 |
| walker |  | 1208 | 12 | Code::CodeKey { rung: Names, file: source/core/Ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.901 |
| ns | 1221 |  | 98 | Complete `test/` tree, including helpers | 1.11 |  | 0.863 |
| walker |  | 1258 | 50 | Code::CodeKey { rung: Decl, file: source/core/Ky.ts, decl: 1, sub: 0, line: 33 } |  |  | 0.863 |
| walker |  | 1290 | 32 | Fs::DirListing { dir: test/helpers } |  |  | 0.907 |
| walker |  | 1304 | 14 | Code::CodeKey { rung: Names, file: source/types/ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.907 |
| ns | 1461 |  | 240 | Every H3 heading in readme.md (API entries and tips) | 1.12 | 1.5 | 0.841 |
| walker |  | 1644 | 340 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.961 |
| walker |  | 1660 | 16 | Markdown::Section { file: readme.md, section_index: 38, keeps_default_concavity: false } |  |  | 0.961 |
| walker |  | 1680 | 20 | Markdown::Section { file: readme.md, section_index: 37, keeps_default_concavity: false } |  |  | 0.961 |
| ns | 1731 |  | 270 | Readme `ky(input, options?)` contract and the body shortcuts | 1.13 | 1.12 | 0.951 |
| walker |  | 1774 | 94 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.951 |
| walker |  | 1849 | 75 | Markdown::Section { file: readme.md, section_index: 39, keeps_default_concavity: false } |  |  | 0.951 |
| walker |  | 1875 | 26 | Code::CodeKey { rung: Names, file: source/utils/delay.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.951 |
| walker |  | 1890 | 15 | Code::CodeKey { rung: Decl, file: source/utils/delay.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.951 |
| walker |  | 1918 | 28 | Code::CodeKey { rung: Decl, file: source/utils/delay.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.951 |
| walker |  | 1944 | 26 | Code::CodeKey { rung: Names, file: source/utils/timeout.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.951 |
| walker |  | 1967 | 23 | Code::CodeKey { rung: Decl, file: source/utils/timeout.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.952 |
| ns | 1983 |  | 252 | `KyInstance`: every member signature (types/ky.ts) | 2.1 |  | 0.881 |
| walker |  | 2015 | 48 | Code::CodeKey { rung: Decl, file: source/utils/timeout.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.881 |
| walker |  | 2042 | 27 | Code::CodeKey { rung: Names, file: source/types/retry.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.881 |
| ns | 2094 |  | 111 | `ResponsePromise`: all six body-shortcut signatures | 2.2 |  | 0.856 |
| walker |  | 2124 | 82 | Code::CodeKey { rung: Decl, file: source/types/retry.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.856 |
| ns | 2326 |  | 232 | `KyOptions`: every ky-specific option with its type | 2.3 |  | 0.795 |
| walker |  | 2427 | 303 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.797 |
| walker |  | 2509 | 82 | Json::Whole { file: tsconfig.json } |  |  | 0.797 |
| ns | 2600 |  | 274 | Concrete retry defaults (`defaultRetryOptions`, utils/normalize.ts) | 2.4 |  | 0.762 |
| walker |  | 2728 | 219 | Markdown::Section { file: readme.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.762 |
| walker |  | 2743 | 15 | Code::CodeKey { rung: Names, file: source/errors/NonError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| walker |  | 2784 | 41 | Code::CodeKey { rung: Decl, file: source/errors/NonError.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.762 |
| walker |  | 2841 | 57 | Code::CodeKey { rung: Doc, file: source/errors/NonError.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.762 |
| walker |  | 2856 | 15 | Code::CodeKey { rung: Names, file: source/errors/TimeoutError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| ns | 2865 |  | 265 | Where the non-retry defaults are applied (core/Ky.ts constructor) | 2.5 |  | 0.727 |
| walker |  | 2886 | 30 | Code::CodeKey { rung: Decl, file: source/errors/TimeoutError.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.727 |
| walker |  | 2927 | 41 | Code::CodeKey { rung: Body, file: source/errors/TimeoutError.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.728 |
| ns | 2934 |  | 69 | `Hooks`: the four hook arrays | 2.6 |  | 0.713 |
| walker |  | 2943 | 16 | Code::CodeKey { rung: Names, file: source/errors/ForceRetryError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| walker |  | 3012 | 69 | Code::CodeKey { rung: Decl, file: source/errors/ForceRetryError.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.714 |
| walker |  | 3059 | 47 | Code::CodeKey { rung: Doc, file: source/errors/ForceRetryError.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.714 |
| walker |  | 3075 | 16 | Code::CodeKey { rung: Names, file: source/types/request.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 3096 | 21 | Code::CodeKey { rung: Decl, file: source/types/request.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.714 |
| walker |  | 3112 | 16 | Code::CodeKey { rung: Names, file: source/types/response.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 3134 | 22 | Code::CodeKey { rung: Decl, file: source/types/response.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.714 |
| ns | 3139 |  | 205 | `RetryOptions`: every retry field (types/retry.ts) | 2.7 |  | 0.680 |
| walker |  | 3168 | 34 | Code::CodeKey { rung: Names, file: source/utils/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 3186 | 18 | Code::CodeKey { rung: Decl, file: source/utils/types.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.681 |
| walker |  | 3222 | 36 | Code::CodeKey { rung: Decl, file: source/utils/types.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.681 |
| ns | 3480 |  | 341 | Hook function signatures and their `*State` objects | 2.8 | 2.6 | 0.642 |
| walker |  | 3512 | 290 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.645 |
| ns | 3745 |  | 265 | Public error classes: `HTTPError`, `TimeoutError`, `ForceRetryError` | 2.9 |  | 0.634 |
| walker |  | 3819 | 307 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 1, line: 5 } |  |  | 0.645 |
| ns | 4021 |  | 276 | `Options` and `NormalizedOptions` interfaces | 2.10 |  | 0.627 |
| walker |  | 4084 | 265 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 2, line: 5 } |  |  | 0.643 |
| ns | 4199 |  | 178 | `ky.stop`, `ky.retry()` and the `ForceRetryOptions` fields | 2.11 |  | 0.623 |
| walker |  | 4370 | 286 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 3, line: 5 } |  |  | 0.621 |
| ns | 4370 |  | 171 | `requestMethods`, `responseTypes` and `maxSafeTimeout` | 2.12 |  | 0.621 |
| ns | 4571 |  | 201 | Option registries: `kyOptionKeys`, `requestOptionsRegistry`, `vendorSpecificOptions` | 2.13 |  | 0.603 |
| ns | 4784 |  | 213 | Core public type aliases: `Input`, `Progress`, search-param and method types | 2.14 |  | 0.593 |
| walker |  | 4847 | 477 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 4, line: 5 } |  |  | 0.598 |
| ns | 5042 |  | 258 | `Ky` class: every field and method, name-only (core/Ky.ts) | 3.1 |  | 0.573 |
| ns | 5148 |  | 106 | Runtime capability flags exported by core/constants.ts | 3.2 |  | 0.569 |
| ns | 5339 |  | 191 | Every exported symbol of `source/utils/` (merge, normalize, options) | 3.3 |  | 0.556 |
| walker |  | 5400 | 553 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 5, line: 5 } |  |  | 0.561 |
| walker |  | 5418 | 18 | Code::CodeKey { rung: Names, file: source/types/ResponsePromise.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 5633 |  | 294 | Every exported symbol of `source/utils/` (timeout, delay, body, guards, misc) | 3.4 | 3.3 | 0.555 |
| ns | 5813 |  | 180 | Every readme option anchor (`##### <option>`) and hook anchor | 3.5 |  | 0.545 |
| walker |  | 5830 | 412 | Code::CodeKey { rung: Decl, file: source/types/ResponsePromise.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.560 |
| ns | 6049 |  | 236 | `test/helpers/`: every exported test helper | 3.6 |  | 0.550 |
| walker |  | 6086 | 256 | Markdown::Section { file: readme.md, section_index: 35, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 6320 | 234 | Markdown::Section { file: readme.md, section_index: 36, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 6357 | 37 | Code::CodeKey { rung: Names, file: source/utils/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| ns | 6360 |  | 311 | Remaining readme H4/H6 headings (input, defaultOptions, ky.retry options, CDN, FAQ) | 3.7 |  | 0.548 |
| walker |  | 6388 | 31 | Code::CodeKey { rung: Decl, file: source/utils/options.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.548 |
| ns | 6456 |  | 96 | The internal-only `NonError` wrapper | 3.8 |  | 0.546 |
| walker |  | 6557 | 169 | Code::CodeKey { rung: Names, file: source/types/hooks.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 6603 | 46 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 2, sub: 0, line: 14 } |  |  | 0.557 |
| walker |  | 6660 | 57 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 6, sub: 0, line: 41 } |  |  | 0.566 |
| walker |  | 6748 | 88 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 3, sub: 0, line: 20 } |  |  | 0.589 |
| ns | 6748 |  | 292 | `createInstance`: how `ky`, the method shortcuts, `create` and `extend` are built | 4.1 |  | 0.589 |
| walker |  | 6839 | 91 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 5, sub: 0, line: 32 } |  |  | 0.593 |
| walker |  | 6932 | 93 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.598 |
| ns | 6965 |  | 217 | `#calculateRetryDelay`: limit, non-Error wrapping, forced and method checks | 4.2 | 3.1 | 0.588 |
| walker |  | 7027 | 95 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.593 |
| walker |  | 7046 | 19 | Code::CodeKey { rung: Names, file: source/errors/HTTPError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 7108 | 62 | Code::CodeKey { rung: Decl, file: source/errors/HTTPError.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.601 |
| ns | 7168 |  | 203 | `#calculateRetryDelay`: the `shouldRetry` predicate contract | 4.3 | 4.2 | 0.593 |
| walker |  | 7433 | 325 | Code::CodeKey { rung: Names, file: source/core/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 7451 | 18 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 12, sub: 0, line: 148 } |  |  | 0.606 |
| walker |  | 7479 | 28 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 16, sub: 0, line: 256 } |  |  | 0.608 |
| walker |  | 7589 | 110 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 7, sub: 0, line: 46 } |  |  | 0.619 |
| ns | 7600 |  | 432 | `#calculateRetryDelay`: timeouts, status codes and `Retry-After` parsing | 4.4 | 4.3 | 0.603 |
| walker |  | 7713 | 124 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.622 |
| walker |  | 7842 | 129 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 17, sub: 0, line: 265 } |  |  | 0.624 |
| walker |  | 7872 | 30 | Code::CodeKey { rung: Doc, file: source/core/constants.ts, decl: 12, sub: 0, line: 148 } |  |  | 0.628 |
| ns | 8012 |  | 412 | `#retry`: the recursive retry loop and the `beforeRetry` hook contract | 4.5 | 3.1 | 0.611 |
| walker |  | 8189 | 317 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.612 |
| ns | 8221 |  | 209 | Constructor: input validation and `prefixUrl` joining | 4.6 | 2.5 | 0.605 |
| ns | 8398 |  | 177 | Where a non-2xx response becomes an `HTTPError` | 4.7 | 3.1 | 0.599 |
| walker |  | 8549 | 360 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.611 |
| ns | 8553 |  | 155 | `mergeHeaders`: how `.extend()` removes a header | 5.1 | 3.3 | 0.605 |
| walker |  | 8661 | 112 | Json::Whole { file: tsconfig.dist.json } |  |  | 0.606 |
| ns | 8726 |  | 173 | `mergeHooks` / `newHookValue`: hook array inheritance | 5.2 | 3.3 | 0.600 |
| ns | 9000 |  | 274 | `deepMerge`: signal collection, shallow `context`, `searchParams` accumulation | 5.3 | 3.3 | 0.591 |
| walker |  | 9007 | 346 | Markdown::Section { file: readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.594 |
| walker |  | 9055 | 48 | Code::CodeKey { rung: Names, file: source/utils/normalize.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 9077 | 22 | Code::CodeKey { rung: Decl, file: source/utils/normalize.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.597 |
| ns | 9125 |  | 125 | Install instructions, CDN entry points and the Deno import | 6.1 | 3.7 | 0.600 |
| walker |  | 9174 | 97 | Code::CodeKey { rung: Names, file: source/utils/type-guards.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 9195 | 21 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 2, sub: 0, line: 49 } |  |  | 0.606 |
| walker |  | 9216 | 21 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.606 |
| walker |  | 9239 | 23 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 1, sub: 0, line: 27 } |  |  | 0.606 |
| walker |  | 9262 | 23 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 4, sub: 0, line: 98 } |  |  | 0.606 |
| ns | 9282 |  | 157 | Support matrix, related packages and maintainers | 6.2 | 1.5 | 0.605 |
| ns | 9370 |  | 88 | "Extending types": why ky uses type aliases | 6.3 | 1.12 | 0.604 |
| ns | 9473 |  | 103 | npm scripts: how to build, test and debug | 7.1 | 1.9 | 0.607 |
| ns | 9500 |  | 27 | Remaining root directories: `.github/` and `media/` | 7.2 |  | 0.610 |
| walker |  | 9657 | 395 | Markdown::Section { file: readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.611 |
| ns | 9694 |  | 194 | TypeScript configuration (both tsconfigs, complete) | 7.3 | 7.1 | 0.620 |
| ns | 9962 |  | 268 | AVA configuration and the shape of a typical test | 7.4 | 3.6 | 0.610 |
| walker |  | 9999 | 342 | Code::CodeKey { rung: Names, file: source/types/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
