Score(3000)=0.723 I=0.906 C=0.577 ns_rows≤3K=19/52 grid(1000/1442/2080/3000/4327/6240/9000)=0.803/0.917/0.860/0.723/0.615/0.583/0.552

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 52 | 12 | Fs::DirListing { dir: media } |  |  | 0.000 |
| walker |  | 68 | 16 | Fs::DirListing { dir: source } |  |  | 0.000 |
| walker |  | 76 | 8 | Fs::DirListing { dir: source/core } |  |  | 0.000 |
| ns | 81 |  | 81 | Readme lede: what ky is, what it targets | 1.1 |  | 0.000 |
| walker |  | 97 | 21 | Fs::DirListing { dir: source/errors } |  |  | 0.000 |
| ns | 121 |  | 40 | Complete repository root listing | 1.2 |  | 0.644 |
| ns | 237 |  | 116 | Complete `source/` tree: every library file | 1.3 |  | 0.394 |
| ns | 313 |  | 76 | package.json identity: name, version, description, license, repository | 1.4 |  | 0.369 |
| walker |  | 326 | 229 | Markdown::ReadmeHeadline { file: readme.md } |  |  | 0.587 |
| walker |  | 359 | 33 | Fs::DirListing { dir: source/types } |  |  | 0.730 |
| walker |  | 397 | 38 | Fs::DirListing { dir: source/utils } |  |  | 0.948 |
| ns | 415 |  | 102 | Every H2 section heading in readme.md | 1.5 |  | 0.854 |
| walker |  | 468 | 71 | Json::Identity { file: package.json } |  |  | 0.890 |
| walker |  | 479 | 11 | Fs::DirListing { dir: .github } |  |  | 0.891 |
| walker |  | 483 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.891 |
| walker |  | 514 | 31 | Json::Runtime { file: package.json } |  |  | 0.892 |
| ns | 535 |  | 120 | "Benefits over plain fetch" feature bullets | 1.6 | 1.5 | 0.893 |
| walker |  | 617 | 103 | Json::Scripts { file: package.json } |  |  | 0.894 |
| ns | 669 |  | 134 | source/index.ts: the runtime exports (default `ky`, error classes, type guards) | 1.7 |  | 0.811 |
| walker |  | 720 | 103 | Json::Entry { file: package.json } |  |  | 0.821 |
| ns | 751 |  | 82 | Readme usage example: the canonical call shape | 1.8 | 1.5 | 0.776 |
| walker |  | 786 | 66 | Fs::DirListing { dir: test } |  |  | 0.782 |
| ns | 873 |  | 122 | package.json module contract: type, exports, main, engines | 1.9 | 1.4 | 0.797 |
| ns | 1123 |  | 250 | source/index.ts: the complete public type-export block | 1.10 | 1.7 | 0.691 |
| walker |  | 1196 | 410 | Code::CodeKey { rung: Names, file: source/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.901 |
| walker |  | 1208 | 12 | Code::CodeKey { rung: Names, file: source/core/Ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.901 |
| ns | 1221 |  | 98 | Complete `test/` tree, including helpers | 1.11 |  | 0.863 |
| walker |  | 1258 | 50 | Code::CodeKey { rung: Decl, file: source/core/Ky.ts, decl: 1, sub: 0, line: 33 } |  |  | 0.863 |
| walker |  | 1272 | 14 | Code::CodeKey { rung: Names, file: source/types/ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 1304 | 32 | Fs::DirListing { dir: test/helpers } |  |  | 0.907 |
| ns | 1461 |  | 240 | Every H3 heading in readme.md (API entries and tips) | 1.12 | 1.5 | 0.841 |
| walker |  | 1644 | 340 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.961 |
| walker |  | 1660 | 16 | Markdown::Section { file: readme.md, section_index: 38, keeps_default_concavity: false } |  |  | 0.961 |
| walker |  | 1680 | 20 | Markdown::Section { file: readme.md, section_index: 37, keeps_default_concavity: false } |  |  | 0.961 |
| ns | 1684 |  | 223 | Readme `ky(input, options?)` contract and the body shortcuts | 1.13 | 1.12 | 0.951 |
| walker |  | 1774 | 94 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.951 |
| walker |  | 1849 | 75 | Markdown::Section { file: readme.md, section_index: 39, keeps_default_concavity: false } |  |  | 0.951 |
| ns | 1936 |  | 252 | `KyInstance`: every member signature (types/ky.ts) | 2.1 |  | 0.881 |
| walker |  | 2029 | 180 | Markdown::Section { file: readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.886 |
| ns | 2047 |  | 111 | `ResponsePromise`: all six body-shortcut signatures | 2.2 |  | 0.860 |
| walker |  | 2111 | 82 | Json::Whole { file: tsconfig.json } |  |  | 0.861 |
| ns | 2279 |  | 232 | `KyOptions`: every ky-specific option with its type | 2.3 |  | 0.799 |
| walker |  | 2330 | 219 | Markdown::Section { file: readme.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.799 |
| walker |  | 2346 | 16 | Code::CodeKey { rung: Names, file: source/errors/ForceRetryError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.799 |
| walker |  | 2415 | 69 | Code::CodeKey { rung: Decl, file: source/errors/ForceRetryError.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.799 |
| ns | 2553 |  | 274 | Concrete retry defaults (`defaultRetryOptions`, utils/normalize.ts) | 2.4 |  | 0.764 |
| walker |  | 2700 | 285 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.766 |
| ns | 2818 |  | 265 | Where the non-retry defaults are applied (core/Ky.ts constructor) | 2.5 |  | 0.732 |
| ns | 2887 |  | 69 | `Hooks`: the four hook arrays | 2.6 |  | 0.717 |
| walker |  | 2903 | 203 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 1, line: 5 } |  |  | 0.723 |
| ns | 3092 |  | 205 | `RetryOptions`: every retry field (types/retry.ts) | 2.7 |  | 0.687 |
| walker |  | 3107 | 204 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 2, line: 5 } |  |  | 0.698 |
| ns | 3433 |  | 341 | Hook function signatures and their `*State` objects | 2.8 | 2.6 | 0.658 |
| walker |  | 3558 | 451 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 3, line: 5 } |  |  | 0.679 |
| ns | 3698 |  | 265 | Public error classes: `HTTPError`, `TimeoutError`, `ForceRetryError` | 2.9 |  | 0.656 |
| walker |  | 3716 | 158 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 4, line: 5 } |  |  | 0.658 |
| walker |  | 3938 | 222 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 5, line: 5 } |  |  | 0.658 |
| ns | 3974 |  | 276 | `Options` and `NormalizedOptions` interfaces | 2.10 |  | 0.640 |
| walker |  | 4123 | 185 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 6, line: 5 } |  |  | 0.645 |
| ns | 4152 |  | 178 | `ky.stop`, `ky.retry()` and the `ForceRetryOptions` fields | 2.11 |  | 0.625 |
| ns | 4323 |  | 171 | `requestMethods`, `responseTypes` and `maxSafeTimeout` | 2.12 |  | 0.615 |
| walker |  | 4335 | 212 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 7, line: 5 } |  |  | 0.615 |
| ns | 4524 |  | 201 | Option registries: `kyOptionKeys`, `requestOptionsRegistry`, `vendorSpecificOptions` | 2.13 |  | 0.597 |
| walker |  | 4591 | 256 | Markdown::Section { file: readme.md, section_index: 35, keeps_default_concavity: false } |  |  | 0.597 |
| ns | 4737 |  | 213 | Core public type aliases: `Input`, `Progress`, search-param and method types | 2.14 |  | 0.587 |
| walker |  | 4825 | 234 | Markdown::Section { file: readme.md, section_index: 36, keeps_default_concavity: false } |  |  | 0.587 |
| ns | 4995 |  | 258 | `Ky` class: every field and method, name-only (core/Ky.ts) | 3.1 |  | 0.563 |
| walker |  | 5070 | 245 | Code::CodeKey { rung: Decl, file: source/types/ky.ts, decl: 1, sub: 8, line: 5 } |  |  | 0.568 |
| walker |  | 5089 | 19 | Code::CodeKey { rung: Names, file: source/errors/HTTPError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| ns | 5101 |  | 106 | Runtime capability flags exported by core/constants.ts | 3.2 |  | 0.566 |
| walker |  | 5151 | 62 | Code::CodeKey { rung: Decl, file: source/errors/HTTPError.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.570 |
| walker |  | 5166 | 15 | Code::CodeKey { rung: Names, file: source/errors/NonError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 5207 | 41 | Code::CodeKey { rung: Decl, file: source/errors/NonError.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.570 |
| walker |  | 5222 | 15 | Code::CodeKey { rung: Names, file: source/errors/TimeoutError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 5252 | 30 | Code::CodeKey { rung: Decl, file: source/errors/TimeoutError.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.577 |
| ns | 5292 |  | 191 | Every exported symbol of `source/utils/` (merge, normalize, options) | 3.3 |  | 0.563 |
| walker |  | 5577 | 325 | Code::CodeKey { rung: Names, file: source/core/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 5586 |  | 294 | Every exported symbol of `source/utils/` (timeout, delay, body, guards, misc) | 3.4 | 3.3 | 0.560 |
| walker |  | 5595 | 18 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 12, sub: 0, line: 148 } |  |  | 0.563 |
| walker |  | 5623 | 28 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 16, sub: 0, line: 256 } |  |  | 0.565 |
| walker |  | 5752 | 129 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 17, sub: 0, line: 265 } |  |  | 0.566 |
| ns | 5766 |  | 180 | Every readme option anchor (`##### <option>`) and hook anchor | 3.5 |  | 0.556 |
| walker |  | 5876 | 124 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.581 |
| walker |  | 5986 | 110 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 7, sub: 0, line: 46 } |  |  | 0.594 |
| ns | 6002 |  | 236 | `test/helpers/`: every exported test helper | 3.6 |  | 0.583 |
| walker |  | 6303 | 317 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.584 |
| ns | 6313 |  | 311 | Remaining readme H4/H6 headings (input, defaultOptions, ky.retry options, CDN, FAQ) | 3.7 |  | 0.581 |
| walker |  | 6321 | 18 | Code::CodeKey { rung: Doc, file: source/core/constants.ts, decl: 8, sub: 0, line: 58 } |  |  | 0.581 |
| walker |  | 6340 | 19 | Code::CodeKey { rung: Doc, file: source/core/constants.ts, decl: 9, sub: 0, line: 61 } |  |  | 0.581 |
| ns | 6409 |  | 96 | The internal-only `NonError` wrapper | 3.8 |  | 0.578 |
| walker |  | 6700 | 360 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.593 |
| ns | 6701 |  | 292 | `createInstance`: how `ky`, the method shortcuts, `create` and `extend` are built | 4.1 |  | 0.581 |
| walker |  | 6812 | 112 | Json::Whole { file: tsconfig.dist.json } |  |  | 0.582 |
| ns | 6918 |  | 217 | `#calculateRetryDelay`: limit, non-Error wrapping, forced and method checks | 4.2 | 3.1 | 0.573 |
| ns | 7121 |  | 203 | `#calculateRetryDelay`: the `shouldRetry` predicate contract | 4.3 | 4.2 | 0.565 |
| walker |  | 7193 | 381 | Markdown::Section { file: readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.567 |
| ns | 7553 |  | 432 | `#calculateRetryDelay`: timeouts, status codes and `Retry-After` parsing | 4.4 | 4.3 | 0.553 |
| walker |  | 7595 | 402 | Markdown::Section { file: readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.555 |
| walker |  | 7841 | 246 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 11, sub: 0, line: 68 } |  |  | 0.559 |
| walker |  | 7864 | 23 | Code::CodeKey { rung: Doc, file: source/core/constants.ts, decl: 11, sub: 0, line: 68 } |  |  | 0.563 |
| walker |  | 7894 | 30 | Code::CodeKey { rung: Doc, file: source/core/constants.ts, decl: 12, sub: 0, line: 148 } |  |  | 0.569 |
| ns | 7965 |  | 412 | `#retry`: the recursive retry loop and the `beforeRetry` hook contract | 4.5 | 3.1 | 0.554 |
| ns | 8174 |  | 209 | Constructor: input validation and `prefixUrl` joining | 4.6 | 2.5 | 0.547 |
| walker |  | 8255 | 361 | Code::CodeKey { rung: Names, file: source/types/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 8328 | 73 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 6, sub: 0, line: 16 } |  |  | 0.563 |
| ns | 8351 |  | 177 | Where a non-2xx response becomes an `HTTPError` | 4.7 | 3.1 | 0.557 |
| walker |  | 8465 | 137 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 12, sub: 0, line: 373 } |  |  | 0.570 |
| ns | 8506 |  | 155 | `mergeHeaders`: how `.extend()` removes a header | 5.1 | 3.3 | 0.564 |
| walker |  | 8610 | 145 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 11, sub: 0, line: 358 } |  |  | 0.564 |
| walker |  | 8625 | 15 | Code::CodeKey { rung: Doc, file: source/types/options.ts, decl: 7, sub: 0, line: 27 } |  |  | 0.564 |
| ns | 8679 |  | 173 | `mergeHooks` / `newHookValue`: hook array inheritance | 5.2 | 3.3 | 0.558 |
| ns | 8953 |  | 274 | `deepMerge`: signal collection, shallow `context`, `searchParams` accumulation | 5.3 | 3.3 | 0.550 |
| walker |  | 9047 | 422 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 10, sub: 0, line: 312 } |  |  | 0.556 |
| ns | 9078 |  | 125 | Install instructions, CDN entry points and the Deno import | 6.1 | 3.7 | 0.560 |
| walker |  | 9216 | 169 | Code::CodeKey { rung: Names, file: source/types/hooks.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| ns | 9235 |  | 157 | Support matrix, related packages and maintainers | 6.2 | 1.5 | 0.564 |
| walker |  | 9262 | 46 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 2, sub: 0, line: 14 } |  |  | 0.568 |
| walker |  | 9319 | 57 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 6, sub: 0, line: 41 } |  |  | 0.575 |
| ns | 9323 |  | 88 | "Extending types": why ky uses type aliases | 6.3 | 1.12 | 0.574 |
| walker |  | 9407 | 88 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 3, sub: 0, line: 20 } |  |  | 0.582 |
| ns | 9426 |  | 103 | npm scripts: how to build, test and debug | 7.1 | 1.9 | 0.586 |
| ns | 9453 |  | 27 | Remaining root directories: `.github/` and `media/` | 7.2 |  | 0.589 |
| walker |  | 9498 | 91 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 5, sub: 0, line: 32 } |  |  | 0.592 |
| walker |  | 9591 | 93 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.596 |
| ns | 9647 |  | 194 | TypeScript configuration (both tsconfigs, complete) | 7.3 | 7.1 | 0.605 |
| walker |  | 9686 | 95 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.609 |
| walker |  | 9713 | 27 | Code::CodeKey { rung: Names, file: source/types/retry.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 9795 | 82 | Code::CodeKey { rung: Decl, file: source/types/retry.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.610 |
| walker |  | 9813 | 18 | Code::CodeKey { rung: Names, file: source/types/ResponsePromise.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| ns | 9915 |  | 268 | AVA configuration and the shape of a typical test | 7.4 | 3.6 | 0.601 |
