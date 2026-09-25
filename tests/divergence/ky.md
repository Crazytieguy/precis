Score(3000)=0.719 I=0.914 C=0.566 ns_rows≤3K=19/52 grid(1000/1442/2080/3000/4327/6240/9000)=0.803/0.790/0.889/0.719/0.668/0.646/0.621

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
| walker |  | 818 | 32 | Fs::DirListing { dir: test/helpers } |  |  | 0.788 |
| ns | 873 |  | 122 | package.json module contract: type, exports, main, engines | 1.9 | 1.4 | 0.803 |
| ns | 1123 |  | 250 | source/index.ts: the complete public type-export block | 1.10 | 1.7 | 0.696 |
| walker |  | 1158 | 340 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.767 |
| walker |  | 1174 | 16 | Markdown::Section { file: readme.md, section_index: 38, keeps_default_concavity: false } |  |  | 0.767 |
| walker |  | 1194 | 20 | Markdown::Section { file: readme.md, section_index: 37, keeps_default_concavity: false } |  |  | 0.767 |
| ns | 1221 |  | 98 | Complete `test/` tree, including helpers | 1.11 |  | 0.790 |
| walker |  | 1288 | 94 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.790 |
| ns | 1461 |  | 240 | Every H3 heading in readme.md (API entries and tips) | 1.12 | 1.5 | 0.804 |
| walker |  | 1698 | 410 | Code::CodeKey { rung: Names, file: source/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.961 |
| ns | 1731 |  | 270 | Readme `ky(input, options?)` contract and the body shortcuts | 1.13 | 1.12 | 0.951 |
| walker |  | 1768 | 70 | Markdown::Section { file: readme.md, section_index: 40, keeps_default_concavity: false } |  |  | 0.951 |
| ns | 1983 |  | 252 | `KyInstance`: every member signature (types/ky.ts) | 2.1 |  | 0.881 |
| walker |  | 2067 | 299 | Json::IdentityMeta { file: package.json } |  |  | 0.889 |
| ns | 2094 |  | 111 | `ResponsePromise`: all six body-shortcut signatures | 2.2 |  | 0.863 |
| walker |  | 2142 | 75 | Markdown::Section { file: readme.md, section_index: 39, keeps_default_concavity: false } |  |  | 0.864 |
| walker |  | 2154 | 12 | Code::CodeKey { rung: Names, file: source/core/Ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 2204 | 50 | Code::CodeKey { rung: Decl, file: source/core/Ky.ts, decl: 1, sub: 0, line: 33 } |  |  | 0.864 |
| walker |  | 2230 | 26 | Code::CodeKey { rung: Names, file: source/utils/delay.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 2245 | 15 | Code::CodeKey { rung: Decl, file: source/utils/delay.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.864 |
| walker |  | 2273 | 28 | Code::CodeKey { rung: Decl, file: source/utils/delay.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.864 |
| walker |  | 2299 | 26 | Code::CodeKey { rung: Names, file: source/utils/timeout.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 2322 | 23 | Code::CodeKey { rung: Decl, file: source/utils/timeout.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.864 |
| ns | 2326 |  | 232 | `KyOptions`: every ky-specific option with its type | 2.3 |  | 0.802 |
| walker |  | 2370 | 48 | Code::CodeKey { rung: Decl, file: source/utils/timeout.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.802 |
| walker |  | 2397 | 27 | Code::CodeKey { rung: Names, file: source/types/retry.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.802 |
| walker |  | 2479 | 82 | Code::CodeKey { rung: Decl, file: source/types/retry.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.802 |
| walker |  | 2493 | 14 | Code::CodeKey { rung: Names, file: source/types/ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.802 |
| ns | 2600 |  | 274 | Concrete retry defaults (`defaultRetryOptions`, utils/normalize.ts) | 2.4 |  | 0.766 |
| walker |  | 2796 | 303 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.769 |
| ns | 2865 |  | 265 | Where the non-retry defaults are applied (core/Ky.ts constructor) | 2.5 |  | 0.734 |
| walker |  | 2878 | 82 | Json::Whole { file: tsconfig.json } |  |  | 0.734 |
| walker |  | 2893 | 15 | Code::CodeKey { rung: Names, file: source/errors/NonError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.734 |
| walker |  | 2934 | 41 | Code::CodeKey { rung: Decl, file: source/errors/NonError.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.719 |
| ns | 2934 |  | 69 | `Hooks`: the four hook arrays | 2.6 |  | 0.719 |
| walker |  | 2991 | 57 | Code::CodeKey { rung: Doc, file: source/errors/NonError.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.719 |
| walker |  | 3006 | 15 | Code::CodeKey { rung: Names, file: source/errors/TimeoutError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.719 |
| walker |  | 3036 | 30 | Code::CodeKey { rung: Decl, file: source/errors/TimeoutError.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.719 |
| walker |  | 3077 | 41 | Code::CodeKey { rung: Body, file: source/errors/TimeoutError.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.719 |
| walker |  | 3093 | 16 | Code::CodeKey { rung: Names, file: source/errors/ForceRetryError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.719 |
| ns | 3139 |  | 205 | `RetryOptions`: every retry field (types/retry.ts) | 2.7 |  | 0.686 |
| walker |  | 3162 | 69 | Code::CodeKey { rung: Decl, file: source/errors/ForceRetryError.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.687 |
| walker |  | 3209 | 47 | Code::CodeKey { rung: Doc, file: source/errors/ForceRetryError.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.687 |
| walker |  | 3225 | 16 | Code::CodeKey { rung: Names, file: source/types/request.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 3246 | 21 | Code::CodeKey { rung: Decl, file: source/types/request.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.687 |
| walker |  | 3262 | 16 | Code::CodeKey { rung: Names, file: source/types/response.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 3284 | 22 | Code::CodeKey { rung: Decl, file: source/types/response.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.687 |
| walker |  | 3318 | 34 | Code::CodeKey { rung: Names, file: source/utils/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 3336 | 18 | Code::CodeKey { rung: Decl, file: source/utils/types.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.687 |
| walker |  | 3372 | 36 | Code::CodeKey { rung: Decl, file: source/utils/types.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.687 |
| walker |  | 3390 | 18 | Code::CodeKey { rung: Names, file: source/types/ResponsePromise.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| ns | 3480 |  | 341 | Hook function signatures and their `*State` objects | 2.8 | 2.6 | 0.648 |
| ns | 3745 |  | 265 | Public error classes: `HTTPError`, `TimeoutError`, `ForceRetryError` | 2.9 |  | 0.637 |
| walker |  | 3802 | 412 | Code::CodeKey { rung: Decl, file: source/types/ResponsePromise.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.658 |
| walker |  | 3839 | 37 | Code::CodeKey { rung: Names, file: source/utils/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 3870 | 31 | Code::CodeKey { rung: Decl, file: source/utils/options.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.658 |
| ns | 4021 |  | 276 | `Options` and `NormalizedOptions` interfaces | 2.10 |  | 0.640 |
| walker |  | 4039 | 169 | Code::CodeKey { rung: Names, file: source/types/hooks.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| walker |  | 4085 | 46 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 2, sub: 0, line: 14 } |  |  | 0.656 |
| walker |  | 4142 | 57 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 6, sub: 0, line: 41 } |  |  | 0.668 |
| ns | 4199 |  | 178 | `ky.stop`, `ky.retry()` and the `ForceRetryOptions` fields | 2.11 |  | 0.647 |
| walker |  | 4230 | 88 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 3, sub: 0, line: 20 } |  |  | 0.662 |
| walker |  | 4321 | 91 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 5, sub: 0, line: 32 } |  |  | 0.668 |
| ns | 4370 |  | 171 | `requestMethods`, `responseTypes` and `maxSafeTimeout` | 2.12 |  | 0.657 |
| walker |  | 4414 | 93 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.663 |
| walker |  | 4509 | 95 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.670 |
| walker |  | 4528 | 19 | Code::CodeKey { rung: Names, file: source/errors/HTTPError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.673 |
| ns | 4571 |  | 201 | Option registries: `kyOptionKeys`, `requestOptionsRegistry`, `vendorSpecificOptions` | 2.13 |  | 0.653 |
| walker |  | 4590 | 62 | Code::CodeKey { rung: Decl, file: source/errors/HTTPError.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.661 |
| ns | 4784 |  | 213 | Core public type aliases: `Input`, `Progress`, search-param and method types | 2.14 |  | 0.650 |
| walker |  | 4915 | 325 | Code::CodeKey { rung: Names, file: source/core/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 4933 | 18 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 12, sub: 0, line: 148 } |  |  | 0.662 |
| walker |  | 4961 | 28 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 16, sub: 0, line: 256 } |  |  | 0.664 |
| ns | 5042 |  | 258 | `Ky` class: every field and method, name-only (core/Ky.ts) | 3.1 |  | 0.636 |
| walker |  | 5071 | 110 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 7, sub: 0, line: 46 } |  |  | 0.650 |
| ns | 5148 |  | 106 | Runtime capability flags exported by core/constants.ts | 3.2 |  | 0.653 |
| walker |  | 5195 | 124 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 15, sub: 0, line: 237 } |  |  | 0.676 |
| walker |  | 5324 | 129 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 17, sub: 0, line: 265 } |  |  | 0.680 |
| ns | 5339 |  | 191 | Every exported symbol of `source/utils/` (merge, normalize, options) | 3.3 |  | 0.665 |
| walker |  | 5354 | 30 | Code::CodeKey { rung: Doc, file: source/core/constants.ts, decl: 12, sub: 0, line: 148 } |  |  | 0.669 |
| ns | 5633 |  | 294 | Every exported symbol of `source/utils/` (timeout, delay, body, guards, misc) | 3.4 | 3.3 | 0.658 |
| walker |  | 5671 | 317 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.659 |
| walker |  | 5783 | 112 | Json::Whole { file: tsconfig.dist.json } |  |  | 0.660 |
| ns | 5813 |  | 180 | Every readme option anchor (`##### <option>`) and hook anchor | 3.5 |  | 0.648 |
| walker |  | 5831 | 48 | Code::CodeKey { rung: Names, file: source/utils/normalize.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 5853 | 22 | Code::CodeKey { rung: Decl, file: source/utils/normalize.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.651 |
| walker |  | 5950 | 97 | Code::CodeKey { rung: Names, file: source/utils/type-guards.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 5971 | 21 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 2, sub: 0, line: 49 } |  |  | 0.658 |
| walker |  | 5992 | 21 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.658 |
| walker |  | 6015 | 23 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 1, sub: 0, line: 27 } |  |  | 0.658 |
| walker |  | 6038 | 23 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 4, sub: 0, line: 98 } |  |  | 0.658 |
| ns | 6049 |  | 236 | `test/helpers/`: every exported test helper | 3.6 |  | 0.646 |
| walker |  | 6257 | 219 | Markdown::Section { file: readme.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.646 |
| ns | 6360 |  | 311 | Remaining readme H4/H6 headings (input, defaultOptions, ky.retry options, CDN, FAQ) | 3.7 |  | 0.635 |
| ns | 6456 |  | 96 | The internal-only `NonError` wrapper | 3.8 |  | 0.633 |
| walker |  | 6618 | 361 | Code::CodeKey { rung: Names, file: source/types/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 6691 | 73 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 6, sub: 0, line: 16 } |  |  | 0.650 |
| ns | 6748 |  | 292 | `createInstance`: how `ky`, the method shortcuts, `create` and `extend` are built | 4.1 |  | 0.658 |
| walker |  | 6828 | 137 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 12, sub: 0, line: 373 } |  |  | 0.671 |
| ns | 6965 |  | 217 | `#calculateRetryDelay`: limit, non-Error wrapping, forced and method checks | 4.2 | 3.1 | 0.661 |
| walker |  | 6973 | 145 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 11, sub: 0, line: 358 } |  |  | 0.661 |
| walker |  | 7002 | 29 | Code::CodeKey { rung: Doc, file: source/types/options.ts, decl: 12, sub: 0, line: 373 } |  |  | 0.661 |
| walker |  | 7054 | 52 | Code::CodeKey { rung: Doc, file: source/types/options.ts, decl: 9, sub: 0, line: 307 } |  |  | 0.661 |
| ns | 7168 |  | 203 | `#calculateRetryDelay`: the `shouldRetry` predicate contract | 4.3 | 4.2 | 0.652 |
| walker |  | 7476 | 422 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 10, sub: 0, line: 312 } |  |  | 0.658 |
| walker |  | 7503 | 27 | Code::CodeKey { rung: Doc, file: source/types/options.ts, decl: 10, sub: 0, line: 312 } |  |  | 0.658 |
| ns | 7600 |  | 432 | `#calculateRetryDelay`: timeouts, status codes and `Retry-After` parsing | 4.4 | 4.3 | 0.642 |
| walker |  | 7754 | 251 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 11, sub: 0, line: 68 } |  |  | 0.647 |
| walker |  | 7777 | 23 | Code::CodeKey { rung: Doc, file: source/core/constants.ts, decl: 11, sub: 0, line: 68 } |  |  | 0.652 |
| walker |  | 7863 | 86 | Code::CodeKey { rung: Names, file: source/types/common.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 7884 | 21 | Code::CodeKey { rung: Decl, file: source/types/common.ts, decl: 3, sub: 0, line: 6 } |  |  | 0.652 |
| ns | 8012 |  | 412 | `#retry`: the recursive retry loop and the `beforeRetry` hook contract | 4.5 | 3.1 | 0.634 |
| ns | 8221 |  | 209 | Constructor: input validation and `prefixUrl` joining | 4.6 | 2.5 | 0.627 |
| ns | 8398 |  | 177 | Where a non-2xx response becomes an `HTTPError` | 4.7 | 3.1 | 0.621 |
| walker |  | 8461 | 577 | Code::CodeKey { rung: Decl, file: source/core/constants.ts, decl: 11, sub: 1, line: 68 } |  |  | 0.626 |
| walker |  | 8550 | 89 | Code::CodeKey { rung: Names, file: source/utils/body.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| ns | 8553 |  | 155 | `mergeHeaders`: how `.extend()` removes a header | 5.1 | 3.3 | 0.624 |
| walker |  | 8685 | 135 | Code::CodeKey { rung: Body, file: source/utils/body.ts, decl: 3, sub: 0, line: 119 } |  |  | 0.625 |
| ns | 8726 |  | 173 | `mergeHooks` / `newHookValue`: hook array inheritance | 5.2 | 3.3 | 0.619 |
| walker |  | 8804 | 119 | Code::CodeKey { rung: Names, file: source/utils/merge.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 8898 | 94 | Code::CodeKey { rung: Decl, file: source/utils/merge.ts, decl: 3, sub: 0, line: 38 } |  |  | 0.630 |
| walker |  | 8982 | 84 | Code::CodeKey { rung: Body, file: source/utils/merge.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.631 |
| ns | 9000 |  | 274 | `deepMerge`: signal collection, shallow `context`, `searchParams` accumulation | 5.3 | 3.3 | 0.621 |
| walker |  | 9012 | 30 | Code::CodeKey { rung: Names, file: source/utils/is.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| ns | 9125 |  | 125 | Install instructions, CDN entry points and the Deno import | 6.1 | 3.7 | 0.624 |
| walker |  | 9268 | 256 | Markdown::Section { file: readme.md, section_index: 35, keeps_default_concavity: false } |  |  | 0.625 |
| ns | 9282 |  | 157 | Support matrix, related packages and maintainers | 6.2 | 1.5 | 0.628 |
| ns | 9370 |  | 88 | "Extending types": why ky uses type aliases | 6.3 | 1.12 | 0.627 |
| ns | 9473 |  | 103 | npm scripts: how to build, test and debug | 7.1 | 1.9 | 0.630 |
| ns | 9500 |  | 27 | Remaining root directories: `.github/` and `media/` | 7.2 |  | 0.632 |
| walker |  | 9502 | 234 | Markdown::Section { file: readme.md, section_index: 36, keeps_default_concavity: false } |  |  | 0.635 |
| walker |  | 9650 | 148 | Code::CodeKey { rung: Body, file: source/errors/HTTPError.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.645 |
| ns | 9694 |  | 194 | TypeScript configuration (both tsconfigs, complete) | 7.3 | 7.1 | 0.652 |
| walker |  | 9802 | 152 | Code::CodeKey { rung: Body, file: source/utils/merge.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.663 |
| walker |  | 9961 | 159 | Code::CodeKey { rung: Body, file: source/utils/timeout.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.663 |
| ns | 9962 |  | 268 | AVA configuration and the shape of a typical test | 7.4 | 3.6 | 0.654 |
