Score(3000)=0.488 I=0.767 C=0.310 ns_rows≤3K=27/65 grid(1000/1442/2080/3000/4327/6240/9000)=0.694/0.625/0.518/0.488/0.504/0.632/0.655

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
| ns | 928 |  | 249 | docs/terminology.md in full — the domain vocabulary | 1.10 |  | 0.588 |
| walker |  | 945 | 158 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| walker |  | 1206 | 261 | Json::Scripts { file: package.json } |  |  | 0.695 |
| ns | 1213 |  | 285 | docs/parsing-and-hooks.md in full — the parse life cycle | 1.11 |  | 0.618 |
| ns | 1312 |  | 99 | Readme: the library's job, in five lines of prose | 1.12 |  | 0.601 |
| ns | 1446 |  | 134 | Readme Quick Start: the complete split.js program | 1.13 |  | 0.567 |
| walker |  | 1486 | 280 | Json::Entry { file: package.json } |  |  | 0.614 |
| ns | 1524 |  | 78 | Readme Quick Start console transcript (unknown-option error + suggestion) | 1.14 | 1.13 | 0.597 |
| walker |  | 1576 | 90 | Markdown::Section { file: Readme.md, section_index: 29, keeps_default_concavity: false } |  |  | 0.597 |
| walker |  | 1618 | 42 | Code::CodeKey { rung: Names, file: lib/option.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 1651 | 33 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 18, sub: 0, line: 268 } |  |  | 0.598 |
| ns | 1657 |  | 133 | Command roster 1/10 — construction, subcommands, help/output configuration (lib/command.js 13-288) | 2.1 |  | 0.570 |
| ns | 1767 |  | 110 | Command roster 2/10 — command-arguments, help command, hooks, action (316-556) | 2.2 | 2.1 | 0.547 |
| ns | 1851 |  | 84 | Command roster 3/10 — option creation and registration (585-805) | 2.3 | 2.2 | 0.532 |
| walker |  | 1857 | 206 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 1, sub: 0, line: 3 } |  |  | 0.536 |
| walker |  | 1900 | 43 | Code::CodeKey { rung: Names, file: lib/help.js, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 1950 | 50 | Code::CodeKey { rung: Names, file: lib/argument.js, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| ns | 1998 |  | 147 | Command roster 4/10 — parsing-behaviour toggles and the option-value store (826-983) | 2.4 | 2.3 | 0.516 |
| walker |  | 2053 | 103 | Code::CodeKey { rung: Decl, file: lib/argument.js, decl: 1, sub: 0, line: 3 } |  |  | 0.518 |
| walker |  | 2063 | 10 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 3, sub: 0, line: 48 } |  |  | 0.518 |
| walker |  | 2079 | 16 | Code::CodeKey { rung: Body, file: lib/option.js, decl: 16, sub: 0, line: 243 } |  |  | 0.518 |
| walker |  | 2092 | 13 | Code::CodeKey { rung: Names, file: typings/esm.d.mts, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| ns | 2128 |  | 130 | Command roster 5/10 — parse entry points and stand-alone-executable dispatch (1001-1380) | 2.5 | 2.4 | 0.502 |
| walker |  | 2145 | 53 | Code::CodeKey { rung: Names, file: lib/command.js, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 2179 | 34 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 101, sub: 0, line: 2752 } |  |  | 0.503 |
| walker |  | 2232 | 53 | Code::CodeKey { rung: Names, file: lib/error.js, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 2248 | 16 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.503 |
| walker |  | 2271 | 23 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.503 |
| ns | 2284 |  | 156 | Command roster 6/10 — argument processing, hook chaining, lookup and conflict checks (1403-1723) | 2.6 | 2.5 | 0.487 |
| walker |  | 2288 | 17 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.487 |
| walker |  | 2316 | 28 | Code::CodeKey { rung: Names, file: lib/suggestSimilar.js, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 2334 | 18 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.488 |
| walker |  | 2352 | 18 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 8, sub: 0, line: 119 } |  |  | 0.488 |
| ns | 2358 |  | 74 | Command roster 7/10 — parseOptions, opts, error, env/implied resolution (1748-1996) | 2.7 | 2.6 | 0.479 |
| walker |  | 2370 | 18 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 9, sub: 0, line: 129 } |  |  | 0.479 |
| walker |  | 2389 | 19 | Code::CodeKey { rung: Body, file: lib/option.js, decl: 7, sub: 0, line: 120 } |  |  | 0.479 |
| ns | 2443 |  | 85 | Command roster 8/10 — the complete set of user-facing error reporters (2034-2162) | 2.8 | 2.7 | 0.469 |
| ns | 2595 |  | 152 | Command roster 9/10 — metadata and help-grouping accessors (2195-2437) | 2.9 | 2.8 | 0.453 |
| ns | 2703 |  | 108 | Command roster 10/10 — help output and help-option API (2450-2686) | 2.10 | 2.9 | 0.443 |
| ns | 2755 |  | 52 | lib/command.js module-level helpers and exports | 2.11 | 2.10 | 0.446 |
| ns | 2851 |  | 96 | lib/option.js roster 1/2 — Option's declaration methods (3-156) | 2.12 |  | 0.469 |
| ns | 2994 |  | 143 | lib/option.js roster 2/2 — remaining Option methods, DualOptions, module functions and exports | 2.13 | 2.12 | 0.483 |
| ns | 3122 |  | 128 | lib/argument.js — complete roster (150-line file) | 2.14 |  | 0.502 |
| walker |  | 3202 | 813 | Fs::DirListing { dir: tests } |  |  | 0.511 |
| walker |  | 3231 | 29 | Fs::DirListing { dir: tests/fixtures-extensions } |  |  | 0.511 |
| walker |  | 3324 | 93 | Fs::DirListing { dir: tests/fixtures } |  |  | 0.513 |
| walker |  | 3327 | 3 | Fs::DirListing { dir: tests/fixtures/another-dir } |  |  | 0.513 |
| walker |  | 3330 | 3 | Fs::DirListing { dir: tests/fixtures/other-dir } |  |  | 0.513 |
| ns | 3353 |  | 231 | lib/error.js — both error classes in full | 2.15 |  | 0.501 |
| ns | 3396 |  | 43 | lib/suggestSimilar.js — complete symbol set | 2.16 |  | 0.500 |
| ns | 3476 |  | 80 | lib/help.js roster 1/4 — visibility and ordering (12-139) | 2.17 |  | 0.492 |
| ns | 3608 |  | 132 | lib/help.js roster 2/4 — term/description/width methods (162-372) | 2.18 | 2.17 | 0.482 |
| walker |  | 3718 | 388 | Code::CodeKey { rung: Names, file: typings/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 3733 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 4, sub: 0, line: 31 } |  |  | 0.485 |
| walker |  | 3748 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 171, sub: 0, line: 1100 } |  |  | 0.485 |
| walker |  | 3769 | 21 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 72, sub: 0, line: 342 } |  |  | 0.485 |
| walker |  | 3792 | 23 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.485 |
| ns | 3804 |  | 196 | lib/help.js roster 3/4 — assembly plus the complete styleX hook set (403-606) | 2.19 | 2.18 | 0.470 |
| walker |  | 3816 | 24 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 172, sub: 0, line: 1104 } |  |  | 0.470 |
| walker |  | 3845 | 29 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 73, sub: 0, line: 345 } |  |  | 0.470 |
| ns | 3874 |  | 70 | lib/help.js roster 4/4 — layout tail, stripColor, exports (618-747) | 2.20 | 2.19 | 0.469 |
| walker |  | 3881 | 36 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 78, sub: 0, line: 370 } |  |  | 0.470 |
| ns | 3891 |  | 17 | typings/ listing | 3.1 |  | 0.474 |
| walker |  | 3919 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 74, sub: 0, line: 349 } |  |  | 0.474 |
| walker |  | 3972 | 53 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 170, sub: 0, line: 1094 } |  |  | 0.475 |
| ns | 4013 |  | 122 | typings/index.d.ts — every exported class declaration | 3.2 |  | 0.487 |
| walker |  | 4034 | 62 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.487 |
| walker |  | 4097 | 63 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 6, sub: 0, line: 40 } |  |  | 0.487 |
| walker |  | 4228 | 131 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 75, sub: 0, line: 354 } |  |  | 0.488 |
| ns | 4258 |  | 245 | typings/index.d.ts — every exported interface, type alias, function and const | 3.3 | 3.2 | 0.508 |
| ns | 4322 |  | 64 | typings/index.d.ts — Command's public instance properties | 3.4 |  | 0.504 |
| ns | 4413 |  | 91 | Readme.md top-level section map (all H2 headings) | 3.5 |  | 0.498 |
| walker |  | 4423 | 195 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.498 |
| walker |  | 4443 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 9, sub: 0, line: 67 } |  |  | 0.498 |
| ns | 4449 |  | 36 | docs/ listing | 3.6 |  | 0.506 |
| walker |  | 4463 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 13, sub: 0, line: 87 } |  |  | 0.506 |
| walker |  | 4483 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 14, sub: 0, line: 92 } |  |  | 0.506 |
| ns | 4564 |  | 115 | docs/*.md top-level heading map | 3.7 |  | 0.499 |
| ns | 4729 |  | 165 | Readme.md subsection map 1/2 — Options and Commands (H3/H4, lines 211-725) | 3.8 |  | 0.490 |
| walker |  | 4934 | 451 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 15, sub: 0, line: 95 } |  |  | 0.490 |
| walker |  | 4954 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 26, sub: 0, line: 189 } |  |  | 0.490 |
| walker |  | 4975 | 21 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 4, sub: 0, line: 56 } |  |  | 0.490 |
| ns | 4992 |  | 263 | Readme.md subsection map 2/2 — Automated help, Bits and pieces, Support (H3, lines 787-1172) | 3.9 | 3.8 | 0.477 |
| walker |  | 4996 | 21 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 11, sub: 0, line: 165 } |  |  | 0.477 |
| walker |  | 5017 | 21 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 24, sub: 0, line: 179 } |  |  | 0.477 |
| walker |  | 5039 | 22 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 28, sub: 0, line: 200 } |  |  | 0.477 |
| walker |  | 5062 | 23 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 19, sub: 0, line: 272 } |  |  | 0.477 |
| ns | 5241 |  | 249 | docs/ subsection map — the complete deprecation list plus options-in-depth subsections | 3.10 |  | 0.467 |
| walker |  | 5280 | 218 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 0, line: 12 } |  |  | 0.494 |
| walker |  | 5289 | 9 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 10, sub: 0, line: 182 } |  |  | 0.494 |
| walker |  | 5298 | 9 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 11, sub: 0, line: 193 } |  |  | 0.494 |
| walker |  | 5319 | 21 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 3, sub: 0, line: 29 } |  |  | 0.494 |
| ns | 5478 |  | 237 | examples/ listing (complete, 45 entries) | 3.11 |  | 0.532 |
| ns | 5609 |  | 131 | typings: the OutputConfiguration shape | 3.12 |  | 0.540 |
| walker |  | 5663 | 344 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 1, line: 12 } |  |  | 0.582 |
| walker |  | 5671 | 8 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 25, sub: 0, line: 545 } |  |  | 0.582 |
| walker |  | 5679 | 8 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 31, sub: 0, line: 575 } |  |  | 0.582 |
| walker |  | 5705 | 26 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 12, sub: 0, line: 82 } |  |  | 0.582 |
| walker |  | 5731 | 26 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 25, sub: 0, line: 184 } |  |  | 0.582 |
| ns | 5748 |  | 139 | typings: ErrorOptions, ParseOptions, HelpContext, AddHelpTextContext bodies | 3.13 | 3.3 | 0.588 |
| ns | 5869 |  | 121 | typings: OptionValueSource members, CommandOptions, ExecutableCommandOptions, ParseOptionsResult | 3.14 | 3.3 | 0.592 |
| walker |  | 5940 | 209 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 0, line: 13 } |  |  | 0.612 |
| walker |  | 5950 | 10 | Code::CodeKey { rung: Body, file: lib/command.js, decl: 6, sub: 0, line: 192 } |  |  | 0.612 |
| walker |  | 5962 | 12 | Code::CodeKey { rung: Body, file: lib/command.js, decl: 13, sub: 0, line: 316 } |  |  | 0.612 |
| walker |  | 5977 | 15 | Code::CodeKey { rung: Body, file: lib/command.js, decl: 7, sub: 0, line: 203 } |  |  | 0.612 |
| walker |  | 6010 | 33 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 4, sub: 0, line: 121 } |  |  | 0.612 |
| walker |  | 6052 | 42 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 2, sub: 0, line: 20 } |  |  | 0.612 |
| walker |  | 6261 | 209 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 1, line: 13 } |  |  | 0.634 |
| walker |  | 6311 | 50 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 19, sub: 0, line: 463 } |  |  | 0.634 |
| walker |  | 6370 | 59 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 28, sub: 0, line: 670 } |  |  | 0.634 |
| walker |  | 6557 | 187 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 2, line: 13 } |  |  | 0.649 |
| walker |  | 6578 | 21 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 37, sub: 0, line: 885 } |  |  | 0.649 |
| walker |  | 6632 | 54 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 39, sub: 0, line: 925 } |  |  | 0.649 |
| ns | 6682 |  | 813 | tests/ listing (complete, 113 entries) | 3.15 |  | 0.685 |
| walker |  | 6695 | 63 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 29, sub: 0, line: 741 } |  |  | 0.685 |
| ns | 6810 |  | 128 | tests/fixtures/, tests/fixtures-extensions/ and their subdirectories (complete) | 3.16 |  | 0.691 |
| ns | 6840 |  | 30 | .github/ and .github/workflows/ listings | 3.17 |  | 0.692 |
| walker |  | 6899 | 204 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 3, line: 13 } |  |  | 0.708 |
| ns | 6900 |  | 60 | Help's five data properties | 4.1 |  | 0.705 |
| walker |  | 6923 | 24 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 52, sub: 0, line: 1352 } |  |  | 0.705 |
| walker |  | 6967 | 44 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 51, sub: 0, line: 1202 } |  |  | 0.705 |
| walker |  | 7032 | 65 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 7, sub: 0, line: 203 } |  |  | 0.705 |
| walker |  | 7098 | 66 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 16, sub: 0, line: 375 } |  |  | 0.705 |
| ns | 7143 |  | 243 | parseOptions()'s documented contract | 4.2 |  | 0.695 |
| walker |  | 7303 | 205 | Markdown::Section { file: Readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.696 |
| walker |  | 7517 | 214 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 4, line: 13 } |  |  | 0.712 |
| ns | 7538 |  | 395 | Option's complete field set (constructor body) | 4.3 |  | 0.698 |
| walker |  | 7565 | 48 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 54, sub: 0, line: 1403 } |  |  | 0.698 |
| walker |  | 7616 | 51 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 63, sub: 0, line: 1694 } |  |  | 0.698 |
| walker |  | 7670 | 54 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 55, sub: 0, line: 1428 } |  |  | 0.698 |
| walker |  | 7726 | 56 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 60, sub: 0, line: 1649 } |  |  | 0.698 |
| walker |  | 7788 | 62 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 53, sub: 0, line: 1380 } |  |  | 0.698 |
| walker |  | 7852 | 64 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 59, sub: 0, line: 1550 } |  |  | 0.698 |
| ns | 7892 |  | 354 | Command instance state 1/4 — commands, options, args and option values | 4.4 |  | 0.685 |
| ns | 8051 |  | 159 | Command instance state 2/4 — behaviour flags, descriptions, hooks, saved state | 4.5 | 4.4 | 0.679 |
| walker |  | 8058 | 206 | Markdown::Section { file: Readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.679 |
| walker |  | 8125 | 67 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 64, sub: 0, line: 1723 } |  |  | 0.679 |
| walker |  | 8154 | 29 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 11, sub: 0, line: 77 } |  |  | 0.679 |
| walker |  | 8183 | 29 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 22, sub: 0, line: 169 } |  |  | 0.679 |
| walker |  | 8212 | 29 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 23, sub: 0, line: 174 } |  |  | 0.679 |
| ns | 8270 |  | 219 | Command instance state 3/4 — the default _outputConfiguration | 4.6 | 4.5 | 0.672 |
| walker |  | 8407 | 195 | Markdown::Section { file: Readme.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.673 |
| walker |  | 8477 | 70 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 10, sub: 0, line: 261 } |  |  | 0.673 |
| ns | 8482 |  | 212 | Command instance state 4/4 — help option/command and group headings | 4.7 | 4.6 | 0.667 |
| walker |  | 8547 | 70 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 26, sub: 0, line: 619 } |  |  | 0.667 |
| walker |  | 8617 | 70 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 62, sub: 0, line: 1675 } |  |  | 0.667 |
| walker |  | 8688 | 71 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 44, sub: 0, line: 1001 } |  |  | 0.667 |
| walker |  | 8760 | 72 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 11, sub: 0, line: 273 } |  |  | 0.667 |
| ns | 8764 |  | 282 | Argument's constructor — the `<req>` / `[opt]` / `name...` grammar | 4.8 |  | 0.655 |
| walker |  | 8832 | 72 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 40, sub: 0, line: 940 } |  |  | 0.655 |
| walker |  | 8840 | 8 | Code::CodeKey { rung: Body, file: lib/help.js, decl: 35, sub: 0, line: 597 } |  |  | 0.655 |
| walker |  | 8913 | 73 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 42, sub: 0, line: 971 } |  |  | 0.655 |
| walker |  | 8987 | 74 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 61, sub: 0, line: 1664 } |  |  | 0.655 |
| walker |  | 9062 | 75 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 20, sub: 0, line: 487 } |  |  | 0.655 |
| ns | 9094 |  | 330 | splitOptionFlags — the flag-string grammar and its error messages | 4.9 | 2.13 | 0.646 |
| walker |  | 9137 | 75 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 27, sub: 0, line: 644 } |  |  | 0.646 |
| ns | 9210 |  | 116 | useColor() — the colour environment-variable contract | 4.10 |  | 0.641 |
| walker |  | 9212 | 75 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 57, sub: 0, line: 1499 } |  |  | 0.641 |
| walker |  | 9462 | 250 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 5, line: 13 } |  |  | 0.657 |
| ns | 9471 |  | 261 | package.json scripts — how to test, lint, format and type-check | 5.1 | 1.4 | 0.660 |
| walker |  | 9503 | 41 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 77, sub: 0, line: 2162 } |  |  | 0.660 |
| walker |  | 9555 | 52 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 66, sub: 0, line: 1902 } |  |  | 0.660 |
| walker |  | 9607 | 52 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 70, sub: 0, line: 1996 } |  |  | 0.660 |
| walker |  | 9662 | 55 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 67, sub: 0, line: 1924 } |  |  | 0.660 |
| ns | 9673 |  | 202 | CONTRIBUTING.md — PR rules and the surfaces a change must update | 5.2 |  | 0.654 |
| walker |  | 9719 | 57 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 75, sub: 0, line: 2114 } |  |  | 0.654 |
| walker |  | 9778 | 59 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 71, sub: 0, line: 2034 } |  |  | 0.654 |
| walker |  | 9838 | 60 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 72, sub: 0, line: 2046 } |  |  | 0.654 |
| ns | 9848 |  | 175 | package.json exports map | 5.3 | 1.4 | 0.658 |
| walker |  | 9898 | 60 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 76, sub: 0, line: 2146 } |  |  | 0.658 |
| ns | 9940 |  | 92 | jest.config.js | 5.4 |  | 0.654 |
| walker |  | 9964 | 66 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 69, sub: 0, line: 1966 } |  |  | 0.654 |
| walker |  | 9996 | 32 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 83, sub: 0, line: 2304 } |  |  | 0.654 |
