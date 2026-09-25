Score(3000)=0.721 I=0.918 C=0.566 ns_rows≤3K=19/52 grid(1000/1442/2080/3000/4327/6240/9000)=0.797/0.730/0.883/0.721/0.685/0.622/0.579

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
| walker |  | 528 | 14 | Code::CodeKey { rung: Names, file: source/types/ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.892 |
| ns | 535 |  | 120 | "Benefits over plain fetch" feature bullets | 1.6 | 1.5 | 0.893 |
| walker |  | 543 | 15 | Code::CodeKey { rung: Names, file: source/errors/TimeoutError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.893 |
| walker |  | 573 | 30 | Code::CodeKey { rung: Decl, file: source/errors/TimeoutError.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.893 |
| walker |  | 639 | 66 | Fs::DirListing { dir: test } |  |  | 0.901 |
| walker |  | 655 | 16 | Code::CodeKey { rung: Names, file: source/errors/ForceRetryError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.901 |
| ns | 669 |  | 134 | source/index.ts: the runtime exports (default `ky`, error classes, type guards) | 1.7 |  | 0.818 |
| walker |  | 671 | 16 | Code::CodeKey { rung: Names, file: source/types/request.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| walker |  | 692 | 21 | Code::CodeKey { rung: Decl, file: source/types/request.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.818 |
| walker |  | 708 | 16 | Code::CodeKey { rung: Names, file: source/types/response.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| walker |  | 730 | 22 | Code::CodeKey { rung: Decl, file: source/types/response.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.818 |
| ns | 751 |  | 82 | Readme usage example: the canonical call shape | 1.8 | 1.5 | 0.773 |
| walker |  | 833 | 103 | Json::Scripts { file: package.json } |  |  | 0.774 |
| ns | 873 |  | 122 | package.json module contract: type, exports, main, engines | 1.9 | 1.4 | 0.727 |
| walker |  | 936 | 103 | Json::Entry { file: package.json } |  |  | 0.797 |
| walker |  | 954 | 18 | Code::CodeKey { rung: Names, file: source/types/ResponsePromise.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 973 | 19 | Code::CodeKey { rung: Names, file: source/errors/HTTPError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 1035 | 62 | Code::CodeKey { rung: Decl, file: source/errors/HTTPError.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.798 |
| walker |  | 1104 | 69 | Code::CodeKey { rung: Decl, file: source/errors/ForceRetryError.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.799 |
| ns | 1123 |  | 250 | source/index.ts: the complete public type-export block | 1.10 | 1.7 | 0.693 |
| walker |  | 1136 | 32 | Fs::DirListing { dir: test/helpers } |  |  | 0.698 |
| walker |  | 1163 | 27 | Code::CodeKey { rung: Names, file: source/types/retry.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| ns | 1221 |  | 98 | Complete `test/` tree, including helpers | 1.11 |  | 0.729 |
| walker |  | 1245 | 82 | Code::CodeKey { rung: Decl, file: source/types/retry.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.730 |
| ns | 1461 |  | 240 | Every H3 heading in readme.md (API entries and tips) | 1.12 | 1.5 | 0.677 |
| walker |  | 1585 | 340 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.806 |
| walker |  | 1601 | 16 | Markdown::Section { file: readme.md, section_index: 38, keeps_default_concavity: false } |  |  | 0.806 |
| walker |  | 1621 | 20 | Markdown::Section { file: readme.md, section_index: 37, keeps_default_concavity: false } |  |  | 0.806 |
| ns | 1731 |  | 270 | Readme `ky(input, options?)` contract and the body shortcuts | 1.13 | 1.12 | 0.797 |
| ns | 1983 |  | 252 | `KyInstance`: every member signature (types/ky.ts) | 2.1 |  | 0.738 |
| walker |  | 2031 | 410 | Code::CodeKey { rung: Names, file: source/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.883 |
| ns | 2094 |  | 111 | `ResponsePromise`: all six body-shortcut signatures | 2.2 |  | 0.858 |
| walker |  | 2125 | 94 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.858 |
| walker |  | 2195 | 70 | Markdown::Section { file: readme.md, section_index: 40, keeps_default_concavity: false } |  |  | 0.858 |
| ns | 2326 |  | 232 | `KyOptions`: every ky-specific option with its type | 2.3 |  | 0.796 |
| walker |  | 2494 | 299 | Json::IdentityMeta { file: package.json } |  |  | 0.804 |
| ns | 2600 |  | 274 | Concrete retry defaults (`defaultRetryOptions`, utils/normalize.ts) | 2.4 |  | 0.768 |
| walker |  | 2797 | 303 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.770 |
| ns | 2865 |  | 265 | Where the non-retry defaults are applied (core/Ky.ts constructor) | 2.5 |  | 0.735 |
| walker |  | 2872 | 75 | Markdown::Section { file: readme.md, section_index: 39, keeps_default_concavity: false } |  |  | 0.735 |
| walker |  | 2913 | 41 | Code::CodeKey { rung: Body, file: source/errors/TimeoutError.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.736 |
| ns | 2934 |  | 69 | `Hooks`: the four hook arrays | 2.6 |  | 0.721 |
| walker |  | 2960 | 47 | Code::CodeKey { rung: Doc, file: source/errors/ForceRetryError.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.721 |
| walker |  | 2972 | 12 | Code::CodeKey { rung: Names, file: source/core/Ky.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 3022 | 50 | Code::CodeKey { rung: Decl, file: source/core/Ky.ts, decl: 1, sub: 0, line: 33 } |  |  | 0.721 |
| walker |  | 3119 | 97 | Code::CodeKey { rung: Names, file: source/utils/type-guards.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| ns | 3139 |  | 205 | `RetryOptions`: every retry field (types/retry.ts) | 2.7 |  | 0.687 |
| walker |  | 3140 | 21 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 2, sub: 0, line: 49 } |  |  | 0.687 |
| walker |  | 3161 | 21 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.687 |
| walker |  | 3184 | 23 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 1, sub: 0, line: 27 } |  |  | 0.687 |
| walker |  | 3207 | 23 | Code::CodeKey { rung: Body, file: source/utils/type-guards.ts, decl: 4, sub: 0, line: 98 } |  |  | 0.687 |
| walker |  | 3222 | 15 | Code::CodeKey { rung: Names, file: source/errors/NonError.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 3263 | 41 | Code::CodeKey { rung: Decl, file: source/errors/NonError.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.688 |
| walker |  | 3345 | 82 | Json::Whole { file: tsconfig.json } |  |  | 0.688 |
| ns | 3480 |  | 341 | Hook function signatures and their `*State` objects | 2.8 | 2.6 | 0.649 |
| ns | 3745 |  | 265 | Public error classes: `HTTPError`, `TimeoutError`, `ForceRetryError` | 2.9 |  | 0.649 |
| walker |  | 3757 | 412 | Code::CodeKey { rung: Decl, file: source/types/ResponsePromise.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.670 |
| walker |  | 3926 | 169 | Code::CodeKey { rung: Names, file: source/types/hooks.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| walker |  | 3972 | 46 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 2, sub: 0, line: 14 } |  |  | 0.686 |
| ns | 4021 |  | 276 | `Options` and `NormalizedOptions` interfaces | 2.10 |  | 0.667 |
| walker |  | 4029 | 57 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 6, sub: 0, line: 41 } |  |  | 0.679 |
| walker |  | 4117 | 88 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 3, sub: 0, line: 20 } |  |  | 0.694 |
| ns | 4199 |  | 178 | `ky.stop`, `ky.retry()` and the `ForceRetryOptions` fields | 2.11 |  | 0.672 |
| walker |  | 4208 | 91 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 5, sub: 0, line: 32 } |  |  | 0.679 |
| walker |  | 4301 | 93 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.685 |
| ns | 4370 |  | 171 | `requestMethods`, `responseTypes` and `maxSafeTimeout` | 2.12 |  | 0.674 |
| walker |  | 4396 | 95 | Code::CodeKey { rung: Decl, file: source/types/hooks.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.680 |
| walker |  | 4422 | 26 | Code::CodeKey { rung: Names, file: source/utils/delay.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 4437 | 15 | Code::CodeKey { rung: Decl, file: source/utils/delay.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.680 |
| walker |  | 4465 | 28 | Code::CodeKey { rung: Decl, file: source/utils/delay.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.680 |
| walker |  | 4491 | 26 | Code::CodeKey { rung: Names, file: source/utils/timeout.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 4514 | 23 | Code::CodeKey { rung: Decl, file: source/utils/timeout.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.681 |
| walker |  | 4562 | 48 | Code::CodeKey { rung: Decl, file: source/utils/timeout.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.681 |
| ns | 4571 |  | 201 | Option registries: `kyOptionKeys`, `requestOptionsRegistry`, `vendorSpecificOptions` | 2.13 |  | 0.661 |
| walker |  | 4674 | 112 | Json::Whole { file: tsconfig.dist.json } |  |  | 0.662 |
| walker |  | 4704 | 30 | Code::CodeKey { rung: Names, file: source/utils/is.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 4784 |  | 213 | Core public type aliases: `Input`, `Progress`, search-param and method types | 2.14 |  | 0.652 |
| walker |  | 4923 | 219 | Markdown::Section { file: readme.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.652 |
| ns | 5042 |  | 258 | `Ky` class: every field and method, name-only (core/Ky.ts) | 3.1 |  | 0.625 |
| ns | 5148 |  | 106 | Runtime capability flags exported by core/constants.ts | 3.2 |  | 0.620 |
| walker |  | 5179 | 256 | Markdown::Section { file: readme.md, section_index: 35, keeps_default_concavity: true } |  |  | 0.620 |
| ns | 5339 |  | 191 | Every exported symbol of `source/utils/` (merge, normalize, options) | 3.3 |  | 0.606 |
| walker |  | 5413 | 234 | Markdown::Section { file: readme.md, section_index: 36, keeps_default_concavity: false } |  |  | 0.606 |
| walker |  | 5447 | 34 | Code::CodeKey { rung: Names, file: source/utils/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 5465 | 18 | Code::CodeKey { rung: Decl, file: source/utils/types.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.607 |
| walker |  | 5501 | 36 | Code::CodeKey { rung: Decl, file: source/utils/types.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.607 |
| walker |  | 5538 | 37 | Code::CodeKey { rung: Names, file: source/utils/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 5569 | 31 | Code::CodeKey { rung: Decl, file: source/utils/options.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.607 |
| ns | 5633 |  | 294 | Every exported symbol of `source/utils/` (timeout, delay, body, guards, misc) | 3.4 | 3.3 | 0.611 |
| walker |  | 5717 | 148 | Code::CodeKey { rung: Body, file: source/errors/HTTPError.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.626 |
| ns | 5813 |  | 180 | Every readme option anchor (`##### <option>`) and hook anchor | 3.5 |  | 0.615 |
| ns | 6049 |  | 236 | `test/helpers/`: every exported test helper | 3.6 |  | 0.603 |
| walker |  | 6078 | 361 | Code::CodeKey { rung: Names, file: source/types/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| walker |  | 6151 | 73 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 6, sub: 0, line: 16 } |  |  | 0.622 |
| walker |  | 6288 | 137 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 12, sub: 0, line: 373 } |  |  | 0.636 |
| ns | 6360 |  | 311 | Remaining readme H4/H6 headings (input, defaultOptions, ky.retry options, CDN, FAQ) | 3.7 |  | 0.631 |
| walker |  | 6433 | 145 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 11, sub: 0, line: 358 } |  |  | 0.631 |
| ns | 6456 |  | 96 | The internal-only `NonError` wrapper | 3.8 |  | 0.628 |
| walker |  | 6462 | 29 | Code::CodeKey { rung: Doc, file: source/types/options.ts, decl: 12, sub: 0, line: 373 } |  |  | 0.628 |
| walker |  | 6514 | 52 | Code::CodeKey { rung: Doc, file: source/types/options.ts, decl: 9, sub: 0, line: 307 } |  |  | 0.628 |
| ns | 6748 |  | 292 | `createInstance`: how `ky`, the method shortcuts, `create` and `extend` are built | 4.1 |  | 0.638 |
| walker |  | 6936 | 422 | Code::CodeKey { rung: Decl, file: source/types/options.ts, decl: 10, sub: 0, line: 312 } |  |  | 0.645 |
| walker |  | 6963 | 27 | Code::CodeKey { rung: Doc, file: source/types/options.ts, decl: 10, sub: 0, line: 312 } |  |  | 0.645 |
| ns | 6965 |  | 217 | `#calculateRetryDelay`: limit, non-Error wrapping, forced and method checks | 4.2 | 3.1 | 0.635 |
| ns | 7168 |  | 203 | `#calculateRetryDelay`: the `shouldRetry` predicate contract | 4.3 | 4.2 | 0.626 |
| walker |  | 7323 | 360 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.639 |
| walker |  | 7498 | 175 | Code::CodeKey { rung: Doc, file: source/utils/type-guards.ts, decl: 2, sub: 0, line: 49 } |  |  | 0.639 |
| ns | 7600 |  | 432 | `#calculateRetryDelay`: timeouts, status codes and `Retry-After` parsing | 4.4 | 4.3 | 0.623 |
| walker |  | 7680 | 182 | Code::CodeKey { rung: Doc, file: source/utils/type-guards.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.623 |
| ns | 8012 |  | 412 | `#retry`: the recursive retry loop and the `beforeRetry` hook contract | 4.5 | 3.1 | 0.607 |
| walker |  | 8026 | 346 | Markdown::Section { file: readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.610 |
| walker |  | 8074 | 48 | Code::CodeKey { rung: Names, file: source/utils/normalize.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 8096 | 22 | Code::CodeKey { rung: Decl, file: source/utils/normalize.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.613 |
| ns | 8221 |  | 209 | Constructor: input validation and `prefixUrl` joining | 4.6 | 2.5 | 0.606 |
| walker |  | 8281 | 185 | Code::CodeKey { rung: Body, file: source/errors/ForceRetryError.ts, decl: 2, sub: 0, line: 14 } |  |  | 0.606 |
| ns | 8398 |  | 177 | Where a non-2xx response becomes an `HTTPError` | 4.7 | 3.1 | 0.599 |
| walker |  | 8499 | 218 | Code::CodeKey { rung: Doc, file: source/utils/type-guards.ts, decl: 1, sub: 0, line: 27 } |  |  | 0.599 |
| ns | 8553 |  | 155 | `mergeHeaders`: how `.extend()` removes a header | 5.1 | 3.3 | 0.593 |
| walker |  | 8719 | 220 | Code::CodeKey { rung: Doc, file: source/utils/type-guards.ts, decl: 4, sub: 0, line: 98 } |  |  | 0.593 |
| ns | 8726 |  | 173 | `mergeHooks` / `newHookValue`: hook array inheritance | 5.2 | 3.3 | 0.588 |
| ns | 9000 |  | 274 | `deepMerge`: signal collection, shallow `context`, `searchParams` accumulation | 5.3 | 3.3 | 0.579 |
| walker |  | 9114 | 395 | Markdown::Section { file: readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.579 |
| ns | 9125 |  | 125 | Install instructions, CDN entry points and the Deno import | 6.1 | 3.7 | 0.583 |
| ns | 9282 |  | 157 | Support matrix, related packages and maintainers | 6.2 | 1.5 | 0.587 |
| ns | 9370 |  | 88 | "Extending types": why ky uses type aliases | 6.3 | 1.12 | 0.586 |
| ns | 9473 |  | 103 | npm scripts: how to build, test and debug | 7.1 | 1.9 | 0.589 |
| walker |  | 9500 | 386 | Markdown::Section { file: readme.md, section_index: 28, keeps_default_concavity: false } |  |  | 0.592 |
| ns | 9500 |  | 27 | Remaining root directories: `.github/` and `media/` | 7.2 |  | 0.592 |
| ns | 9694 |  | 194 | TypeScript configuration (both tsconfigs, complete) | 7.3 | 7.1 | 0.601 |
| walker |  | 9823 | 323 | Markdown::Section { file: readme.md, section_index: 29, keeps_default_concavity: true } |  |  | 0.602 |
| ns | 9962 |  | 268 | AVA configuration and the shape of a typical test | 7.4 | 3.6 | 0.593 |
