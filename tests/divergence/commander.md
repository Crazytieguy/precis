Score(3000)=0.476 I=0.751 C=0.302 ns_rows≤3K=27/65 grid(1000/1442/2080/3000/4327/6240/9000)=0.694/0.624/0.516/0.476/0.445/0.539/0.568

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 34 |  | 34 | Readme title + one-line pitch | 1.1 |  | 0.000 |
| ns | 59 |  | 25 | lib/ listing — the entire implementation | 1.2 |  | 0.000 |
| walker |  | 102 | 102 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 109 |  | 50 | package.json name, version, npm description | 1.3 |  | 0.000 |
| walker |  | 119 | 17 | Fs::DirListing { dir: typings } |  |  | 0.000 |
| walker |  | 144 | 25 | Fs::DirListing { dir: lib } |  |  | 0.538 |
| walker |  | 180 | 36 | Fs::DirListing { dir: docs } |  |  | 0.356 |
| ns | 180 |  | 71 | package.json module type, main, types, engines | 1.4 |  | 0.356 |
| walker |  | 257 | 77 | Json::Identity { file: package.json } |  |  | 0.481 |
| ns | 265 |  | 85 | index.js: which lib module each public class comes from | 1.5 |  | 0.421 |
| walker |  | 282 | 25 | Fs::DirListing { dir: docs/zh-CN } |  |  | 0.421 |
| ns | 336 |  | 71 | index.js: the `program` singleton and the three createX factories | 1.6 |  | 0.377 |
| walker |  | 347 | 65 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.554 |
| walker |  | 377 | 30 | Json::Runtime { file: package.json } |  |  | 0.605 |
| walker |  | 412 | 35 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 432 | 20 | Fs::DirListing { dir: .github } |  |  | 0.606 |
| ns | 441 |  | 105 | index.js: the complete class + error export block | 1.7 |  | 0.495 |
| walker |  | 442 | 10 | Fs::DirListing { dir: .github/workflows } |  |  | 0.495 |
| ns | 577 |  | 136 | esm.mjs — the named-export ESM wrapper in full | 1.8 |  | 0.408 |
| walker |  | 679 | 237 | Fs::DirListing { dir: examples } |  |  | 0.578 |
| ns | 679 |  | 102 | Repository root listing (complete) | 1.9 |  | 0.578 |
| walker |  | 690 | 11 | Code::CodeKey { rung: Names, file: esm.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 787 | 97 | Code::CodeKey { rung: Decl, file: esm.mjs, decl: 1, sub: 0, line: 4 } |  |  | 0.659 |
| walker |  | 829 | 42 | Markdown::HeadingsOutline { file: docs/help-in-depth.md } |  |  | 0.659 |
| ns | 928 |  | 249 | docs/terminology.md in full — the domain vocabulary | 1.10 |  | 0.588 |
| walker |  | 987 | 158 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| ns | 1213 |  | 285 | docs/parsing-and-hooks.md in full — the parse life cycle | 1.11 |  | 0.611 |
| walker |  | 1248 | 261 | Json::Scripts { file: package.json } |  |  | 0.618 |
| ns | 1312 |  | 99 | Readme: the library's job, in five lines of prose | 1.12 |  | 0.601 |
| ns | 1446 |  | 134 | Readme Quick Start: the complete split.js program | 1.13 |  | 0.567 |
| ns | 1524 |  | 78 | Readme Quick Start console transcript (unknown-option error + suggestion) | 1.14 | 1.13 | 0.551 |
| walker |  | 1528 | 280 | Json::Entry { file: package.json } |  |  | 0.598 |
| walker |  | 1618 | 90 | Markdown::Section { file: Readme.md, section_index: 29, keeps_default_concavity: false } |  |  | 0.598 |
| ns | 1657 |  | 133 | Command roster 1/10 — construction, subcommands, help/output configuration (lib/command.js 13-288) | 2.1 |  | 0.570 |
| walker |  | 1687 | 69 | Markdown::HeadingsOutline { file: docs/options-in-depth.md } |  |  | 0.571 |
| walker |  | 1725 | 38 | Markdown::Section { file: Readme_zh-CN.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.571 |
| walker |  | 1767 | 42 | Code::CodeKey { rung: Names, file: lib/option.js, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 1767 |  | 110 | Command roster 2/10 — command-arguments, help command, hooks, action (316-556) | 2.2 | 2.1 | 0.548 |
| walker |  | 1800 | 33 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 18, sub: 0, line: 268 } |  |  | 0.548 |
| walker |  | 1823 | 23 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 19, sub: 0, line: 272 } |  |  | 0.548 |
| ns | 1851 |  | 84 | Command roster 3/10 — option creation and registration (585-805) | 2.3 | 2.2 | 0.533 |
| ns | 1998 |  | 147 | Command roster 4/10 — parsing-behaviour toggles and the option-value store (826-983) | 2.4 | 2.3 | 0.512 |
| walker |  | 2029 | 206 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 1, sub: 0, line: 3 } |  |  | 0.516 |
| walker |  | 2045 | 16 | Code::CodeKey { rung: Body, file: lib/option.js, decl: 16, sub: 0, line: 243 } |  |  | 0.516 |
| walker |  | 2066 | 21 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 11, sub: 0, line: 165 } |  |  | 0.516 |
| walker |  | 2105 | 39 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 13, sub: 0, line: 203 } |  |  | 0.516 |
| ns | 2128 |  | 130 | Command roster 5/10 — parse entry points and stand-alone-executable dispatch (1001-1380) | 2.5 | 2.4 | 0.500 |
| walker |  | 2148 | 43 | Code::CodeKey { rung: Names, file: lib/help.js, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 2200 | 52 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 43, sub: 0, line: 740 } |  |  | 0.500 |
| walker |  | 2254 | 54 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 15, sub: 0, line: 230 } |  |  | 0.500 |
| ns | 2284 |  | 156 | Command roster 6/10 — argument processing, hook chaining, lookup and conflict checks (1403-1723) | 2.6 | 2.5 | 0.485 |
| walker |  | 2309 | 55 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 10, sub: 0, line: 156 } |  |  | 0.485 |
| ns | 2358 |  | 74 | Command roster 7/10 — parseOptions, opts, error, env/implied resolution (1748-1996) | 2.7 | 2.6 | 0.476 |
| walker |  | 2367 | 58 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 12, sub: 0, line: 181 } |  |  | 0.476 |
| walker |  | 2417 | 50 | Code::CodeKey { rung: Names, file: lib/argument.js, decl: 0, sub: 0, line: 0 } |  |  | 0.476 |
| ns | 2443 |  | 85 | Command roster 8/10 — the complete set of user-facing error reporters (2034-2162) | 2.8 | 2.7 | 0.467 |
| walker |  | 2520 | 103 | Code::CodeKey { rung: Decl, file: lib/argument.js, decl: 1, sub: 0, line: 3 } |  |  | 0.469 |
| walker |  | 2530 | 10 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 3, sub: 0, line: 48 } |  |  | 0.469 |
| walker |  | 2548 | 18 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 8, sub: 0, line: 119 } |  |  | 0.469 |
| walker |  | 2566 | 18 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 9, sub: 0, line: 129 } |  |  | 0.469 |
| walker |  | 2587 | 21 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 4, sub: 0, line: 56 } |  |  | 0.469 |
| ns | 2595 |  | 152 | Command roster 9/10 — metadata and help-grouping accessors (2195-2437) | 2.9 | 2.8 | 0.452 |
| walker |  | 2626 | 39 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 3, sub: 0, line: 48 } |  |  | 0.452 |
| walker |  | 2665 | 39 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 8, sub: 0, line: 119 } |  |  | 0.452 |
| ns | 2703 |  | 108 | Command roster 10/10 — help output and help-option API (2450-2686) | 2.10 | 2.9 | 0.442 |
| walker |  | 2704 | 39 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 9, sub: 0, line: 129 } |  |  | 0.442 |
| ns | 2755 |  | 52 | lib/command.js module-level helpers and exports | 2.11 | 2.10 | 0.438 |
| walker |  | 2762 | 58 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 7, sub: 0, line: 98 } |  |  | 0.438 |
| walker |  | 2775 | 13 | Code::CodeKey { rung: Names, file: typings/esm.d.mts, decl: 0, sub: 0, line: 0 } |  |  | 0.438 |
| walker |  | 2836 | 61 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 10, sub: 0, line: 143 } |  |  | 0.438 |
| ns | 2851 |  | 96 | lib/option.js roster 1/2 — Option's declaration methods (3-156) | 2.12 |  | 0.462 |
| walker |  | 2897 | 61 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 14, sub: 0, line: 217 } |  |  | 0.462 |
| ns | 2994 |  | 143 | lib/option.js roster 2/2 — remaining Option methods, DualOptions, module functions and exports | 2.13 | 2.12 | 0.476 |
| walker |  | 3024 | 127 | Markdown::HeadingsOutline { file: docs/zh-CN/可变参数的选项.md } |  |  | 0.476 |
| walker |  | 3077 | 53 | Code::CodeKey { rung: Names, file: lib/command.js, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 3111 | 34 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 101, sub: 0, line: 2752 } |  |  | 0.483 |
| ns | 3122 |  | 128 | lib/argument.js — complete roster (150-line file) | 2.14 |  | 0.502 |
| walker |  | 3164 | 53 | Code::CodeKey { rung: Names, file: lib/error.js, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 3180 | 16 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.502 |
| walker |  | 3203 | 23 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.502 |
| walker |  | 3220 | 17 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.502 |
| walker |  | 3238 | 18 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.502 |
| walker |  | 3282 | 44 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 4, sub: 0, line: 30 } |  |  | 0.502 |
| walker |  | 3344 | 62 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 6, sub: 0, line: 86 } |  |  | 0.502 |
| ns | 3353 |  | 231 | lib/error.js — both error classes in full | 2.15 |  | 0.490 |
| ns | 3396 |  | 43 | lib/suggestSimilar.js — complete symbol set | 2.16 |  | 0.487 |
| walker |  | 3406 | 62 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 8, sub: 0, line: 132 } |  |  | 0.487 |
| walker |  | 3469 | 63 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 9, sub: 0, line: 144 } |  |  | 0.487 |
| ns | 3476 |  | 80 | lib/help.js roster 1/4 — visibility and ordering (12-139) | 2.17 |  | 0.479 |
| walker |  | 3534 | 65 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 2, sub: 0, line: 11 } |  |  | 0.479 |
| walker |  | 3562 | 28 | Code::CodeKey { rung: Names, file: lib/suggestSimilar.js, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| ns | 3608 |  | 132 | lib/help.js roster 2/4 — term/description/width methods (162-372) | 2.18 | 2.17 | 0.472 |
| walker |  | 3627 | 65 | Code::CodeKey { rung: Doc, file: lib/suggestSimilar.js, decl: 1, sub: 0, line: 56 } |  |  | 0.472 |
| walker |  | 3696 | 69 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 16, sub: 0, line: 243 } |  |  | 0.472 |
| ns | 3804 |  | 196 | lib/help.js roster 3/4 — assembly plus the complete styleX hook set (403-606) | 2.19 | 2.18 | 0.458 |
| ns | 3874 |  | 70 | lib/help.js roster 4/4 — layout tail, stripColor, exports (618-747) | 2.20 | 2.19 | 0.456 |
| ns | 3891 |  | 17 | typings/ listing | 3.1 |  | 0.461 |
| ns | 4013 |  | 122 | typings/index.d.ts — every exported class declaration | 3.2 |  | 0.454 |
| ns | 4258 |  | 245 | typings/index.d.ts — every exported interface, type alias, function and const | 3.3 | 3.2 | 0.444 |
| ns | 4322 |  | 64 | typings/index.d.ts — Command's public instance properties | 3.4 |  | 0.440 |
| ns | 4413 |  | 91 | Readme.md top-level section map (all H2 headings) | 3.5 |  | 0.436 |
| ns | 4449 |  | 36 | docs/ listing | 3.6 |  | 0.445 |
| walker |  | 4509 | 813 | Fs::DirListing { dir: tests } |  |  | 0.454 |
| walker |  | 4538 | 29 | Fs::DirListing { dir: tests/fixtures-extensions } |  |  | 0.454 |
| ns | 4564 |  | 115 | docs/*.md top-level heading map | 3.7 |  | 0.458 |
| walker |  | 4631 | 93 | Fs::DirListing { dir: tests/fixtures } |  |  | 0.460 |
| walker |  | 4634 | 3 | Fs::DirListing { dir: tests/fixtures/another-dir } |  |  | 0.460 |
| walker |  | 4637 | 3 | Fs::DirListing { dir: tests/fixtures/other-dir } |  |  | 0.460 |
| walker |  | 4709 | 72 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 43, sub: 0, line: 740 } |  |  | 0.460 |
| ns | 4729 |  | 165 | Readme.md subsection map 1/2 — Options and Commands (H3/H4, lines 211-725) | 3.8 |  | 0.452 |
| ns | 4992 |  | 263 | Readme.md subsection map 2/2 — Automated help, Bits and pieces, Support (H3, lines 787-1172) | 3.9 | 3.8 | 0.440 |
| walker |  | 5097 | 388 | Code::CodeKey { rung: Names, file: typings/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 5112 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 4, sub: 0, line: 31 } |  |  | 0.485 |
| walker |  | 5127 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 171, sub: 0, line: 1100 } |  |  | 0.485 |
| walker |  | 5148 | 21 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 72, sub: 0, line: 342 } |  |  | 0.485 |
| walker |  | 5171 | 23 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.485 |
| walker |  | 5195 | 24 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 172, sub: 0, line: 1104 } |  |  | 0.485 |
| walker |  | 5224 | 29 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 73, sub: 0, line: 345 } |  |  | 0.485 |
| ns | 5241 |  | 249 | docs/ subsection map — the complete deprecation list plus options-in-depth subsections | 3.10 |  | 0.475 |
| walker |  | 5260 | 36 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 78, sub: 0, line: 370 } |  |  | 0.475 |
| walker |  | 5298 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 74, sub: 0, line: 349 } |  |  | 0.475 |
| walker |  | 5351 | 53 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 170, sub: 0, line: 1094 } |  |  | 0.476 |
| walker |  | 5413 | 62 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.476 |
| walker |  | 5476 | 63 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 6, sub: 0, line: 40 } |  |  | 0.477 |
| ns | 5478 |  | 237 | examples/ listing (complete, 45 entries) | 3.11 |  | 0.517 |
| walker |  | 5607 | 131 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 75, sub: 0, line: 354 } |  |  | 0.518 |
| ns | 5609 |  | 131 | typings: the OutputConfiguration shape | 3.12 |  | 0.526 |
| ns | 5748 |  | 139 | typings: ErrorOptions, ParseOptions, HelpContext, AddHelpTextContext bodies | 3.13 | 3.3 | 0.534 |
| walker |  | 5802 | 195 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.534 |
| walker |  | 5822 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 9, sub: 0, line: 67 } |  |  | 0.534 |
| walker |  | 5842 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 13, sub: 0, line: 87 } |  |  | 0.534 |
| walker |  | 5862 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 14, sub: 0, line: 92 } |  |  | 0.534 |
| ns | 5869 |  | 121 | typings: OptionValueSource members, CommandOptions, ExecutableCommandOptions, ParseOptionsResult | 3.14 | 3.3 | 0.539 |
| walker |  | 5888 | 26 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 12, sub: 0, line: 82 } |  |  | 0.539 |
| walker |  | 5917 | 29 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 11, sub: 0, line: 77 } |  |  | 0.539 |
| walker |  | 5950 | 33 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 10, sub: 0, line: 72 } |  |  | 0.539 |
| walker |  | 5990 | 40 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 5, sub: 0, line: 36 } |  |  | 0.539 |
| walker |  | 6062 | 72 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 8, sub: 0, line: 62 } |  |  | 0.539 |
| walker |  | 6513 | 451 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 15, sub: 0, line: 95 } |  |  | 0.539 |
| walker |  | 6533 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 26, sub: 0, line: 189 } |  |  | 0.539 |
| walker |  | 6554 | 21 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 24, sub: 0, line: 179 } |  |  | 0.539 |
| walker |  | 6576 | 22 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 28, sub: 0, line: 200 } |  |  | 0.539 |
| walker |  | 6602 | 26 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 25, sub: 0, line: 184 } |  |  | 0.539 |
| walker |  | 6631 | 29 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 22, sub: 0, line: 169 } |  |  | 0.539 |
| walker |  | 6660 | 29 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 23, sub: 0, line: 174 } |  |  | 0.539 |
| ns | 6682 |  | 813 | tests/ listing (complete, 113 entries) | 3.15 |  | 0.595 |
| walker |  | 6693 | 33 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 17, sub: 0, line: 120 } |  |  | 0.595 |
| walker |  | 6735 | 42 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 27, sub: 0, line: 195 } |  |  | 0.595 |
| walker |  | 6785 | 50 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 29, sub: 0, line: 207 } |  |  | 0.595 |
| ns | 6810 |  | 128 | tests/fixtures/, tests/fixtures-extensions/ and their subdirectories (complete) | 3.16 |  | 0.604 |
| ns | 6840 |  | 30 | .github/ and .github/workflows/ listings | 3.17 |  | 0.606 |
| walker |  | 6860 | 75 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 20, sub: 0, line: 297 } |  |  | 0.606 |
| ns | 6900 |  | 60 | Help's five data properties | 4.1 |  | 0.604 |
| walker |  | 6937 | 77 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 5, sub: 0, line: 73 } |  |  | 0.604 |
| walker |  | 7014 | 77 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 3, sub: 0, line: 47 } |  |  | 0.604 |
| walker |  | 7092 | 78 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 17, sub: 0, line: 256 } |  |  | 0.604 |
| ns | 7143 |  | 243 | parseOptions()'s documented contract | 4.2 |  | 0.596 |
| walker |  | 7170 | 78 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 3, sub: 0, line: 28 } |  |  | 0.596 |
| walker |  | 7333 | 163 | Markdown::HeadingsOutline { file: docs/zh-CN/不再推荐使用的功能.md } |  |  | 0.596 |
| walker |  | 7412 | 79 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 21, sub: 0, line: 164 } |  |  | 0.596 |
| ns | 7538 |  | 395 | Option's complete field set (constructor body) | 4.3 |  | 0.585 |
| walker |  | 7596 | 184 | Markdown::Section { file: Readme_zh-CN.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.585 |
| walker |  | 7680 | 84 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 2, sub: 0, line: 11 } |  |  | 0.585 |
| ns | 7892 |  | 354 | Command instance state 1/4 — commands, options, args and option values | 4.4 |  | 0.574 |
| walker |  | 7951 | 271 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 0, line: 12 } |  |  | 0.594 |
| walker |  | 7960 | 9 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 10, sub: 0, line: 182 } |  |  | 0.594 |
| walker |  | 7969 | 9 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 11, sub: 0, line: 193 } |  |  | 0.594 |
| walker |  | 7990 | 21 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 3, sub: 0, line: 29 } |  |  | 0.594 |
| walker |  | 8045 | 55 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 17, sub: 0, line: 300 } |  |  | 0.594 |
| ns | 8051 |  | 159 | Command instance state 2/4 — behaviour flags, descriptions, hooks, saved state | 4.5 | 4.4 | 0.590 |
| walker |  | 8105 | 60 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 10, sub: 0, line: 182 } |  |  | 0.590 |
| walker |  | 8165 | 60 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 11, sub: 0, line: 193 } |  |  | 0.590 |
| walker |  | 8225 | 60 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 19, sub: 0, line: 325 } |  |  | 0.590 |
| ns | 8270 |  | 219 | Command instance state 3/4 — the default _outputConfiguration | 4.6 | 4.5 | 0.583 |
| walker |  | 8285 | 60 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 20, sub: 0, line: 372 } |  |  | 0.583 |
| walker |  | 8346 | 61 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 8, sub: 0, line: 139 } |  |  | 0.583 |
| walker |  | 8407 | 61 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 9, sub: 0, line: 162 } |  |  | 0.583 |
| walker |  | 8470 | 63 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 7, sub: 0, line: 112 } |  |  | 0.583 |
| ns | 8482 |  | 212 | Command instance state 4/4 — help option/command and group headings | 4.7 | 4.6 | 0.578 |
| walker |  | 8534 | 64 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 16, sub: 0, line: 276 } |  |  | 0.578 |
| walker |  | 8600 | 66 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 5, sub: 0, line: 62 } |  |  | 0.578 |
| walker |  | 8668 | 68 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 12, sub: 0, line: 205 } |  |  | 0.578 |
| walker |  | 8736 | 68 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 13, sub: 0, line: 224 } |  |  | 0.578 |
| ns | 8764 |  | 282 | Argument's constructor — the `<req>` / `[opt]` / `name...` grammar | 4.8 |  | 0.568 |
| walker |  | 8804 | 68 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 15, sub: 0, line: 258 } |  |  | 0.568 |
| walker |  | 8873 | 69 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 14, sub: 0, line: 241 } |  |  | 0.568 |
| walker |  | 8944 | 71 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 6, sub: 0, line: 79 } |  |  | 0.568 |
| walker |  | 9016 | 72 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 4, sub: 0, line: 40 } |  |  | 0.568 |
| walker |  | 9092 | 76 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 18, sub: 0, line: 313 } |  |  | 0.568 |
| ns | 9094 |  | 330 | splitOptionFlags — the flag-string grammar and its error messages | 4.9 | 2.13 | 0.560 |
| walker |  | 9181 | 89 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 21, sub: 0, line: 403 } |  |  | 0.560 |
| ns | 9210 |  | 116 | useColor() — the colour environment-variable contract | 4.10 |  | 0.556 |
| walker |  | 9470 | 289 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 1, line: 12 } |  |  | 0.577 |
| ns | 9471 |  | 261 | package.json scripts — how to test, lint, format and type-check | 5.1 | 1.4 | 0.581 |
| walker |  | 9534 | 64 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 40, sub: 0, line: 633 } |  |  | 0.581 |
| walker |  | 9600 | 66 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 24, sub: 0, line: 535 } |  |  | 0.581 |
| walker |  | 9667 | 67 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 25, sub: 0, line: 545 } |  |  | 0.581 |
| ns | 9673 |  | 202 | CONTRIBUTING.md — PR rules and the surfaces a change must update | 5.2 |  | 0.576 |
| walker |  | 9735 | 68 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 23, sub: 0, line: 443 } |  |  | 0.576 |
| walker |  | 9806 | 71 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 39, sub: 0, line: 618 } |  |  | 0.576 |
| ns | 9848 |  | 175 | package.json exports map | 5.3 | 1.4 | 0.581 |
| walker |  | 9900 | 94 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 42, sub: 0, line: 695 } |  |  | 0.581 |
| ns | 9940 |  | 92 | jest.config.js | 5.4 |  | 0.578 |
| walker |  | 9999 | 99 | Code::CodeKey { rung: Doc, file: lib/help.js, decl: 22, sub: 0, line: 417 } |  |  | 0.578 |
| walker |  | 9999 | 0 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 0, line: 13 } |  |  | 0.578 |
