Score(3000)=0.667 I=0.814 C=0.547 ns_rows≤3K=27/65 grid(1000/1442/2080/3000/4327/6240/9000)=0.664/0.620/0.624/0.667/0.719/0.740/0.597

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 34 |  | 34 | Readme title + one-line pitch | 1.1 |  | 0.000 |
| ns | 59 |  | 25 | lib/ listing — the entire implementation | 1.2 |  | 0.000 |
| walker |  | 102 | 102 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 109 |  | 50 | package.json name, version, npm description | 1.3 |  | 0.000 |
| walker |  | 167 | 65 | Markdown::ReadmeHeadline { file: Readme.md } |  |  | 0.336 |
| ns | 180 |  | 71 | package.json module type, main, types, engines | 1.4 |  | 0.221 |
| walker |  | 184 | 17 | Fs::DirListing { dir: typings } |  |  | 0.222 |
| walker |  | 209 | 25 | Fs::DirListing { dir: lib } |  |  | 0.569 |
| walker |  | 245 | 36 | Fs::DirListing { dir: docs } |  |  | 0.571 |
| ns | 265 |  | 85 | index.js: which lib module each public class comes from | 1.5 |  | 0.499 |
| walker |  | 322 | 77 | Json::Identity { file: package.json } |  |  | 0.619 |
| ns | 336 |  | 71 | index.js: the `program` singleton and the three createX factories | 1.6 |  | 0.554 |
| walker |  | 347 | 25 | Fs::DirListing { dir: docs/zh-CN } |  |  | 0.554 |
| walker |  | 377 | 30 | Json::Runtime { file: package.json } |  |  | 0.605 |
| walker |  | 397 | 20 | Fs::DirListing { dir: .github } |  |  | 0.606 |
| walker |  | 407 | 10 | Fs::DirListing { dir: .github/workflows } |  |  | 0.606 |
| ns | 441 |  | 105 | index.js: the complete class + error export block | 1.7 |  | 0.495 |
| walker |  | 500 | 93 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.497 |
| walker |  | 524 | 24 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.497 |
| walker |  | 535 | 11 | Code::CodeKey { rung: Names, file: esm.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| ns | 577 |  | 136 | esm.mjs — the named-export ESM wrapper in full | 1.8 |  | 0.410 |
| walker |  | 632 | 97 | Code::CodeKey { rung: Decl, file: esm.mjs, decl: 1, sub: 0, line: 4 } |  |  | 0.552 |
| walker |  | 646 | 14 | Code::CodeKey { rung: Doc, file: esm.mjs, decl: 1, sub: 0, line: 4 } |  |  | 0.590 |
| ns | 679 |  | 102 | Repository root listing (complete) | 1.9 |  | 0.676 |
| walker |  | 883 | 237 | Fs::DirListing { dir: examples } |  |  | 0.684 |
| ns | 928 |  | 249 | docs/terminology.md in full — the domain vocabulary | 1.10 |  | 0.611 |
| walker |  | 1041 | 158 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 1118 | 77 | Markdown::Section { file: Readme.md, section_index: 29, keeps_default_concavity: false } |  |  | 0.710 |
| ns | 1213 |  | 285 | docs/parsing-and-hooks.md in full — the parse life cycle | 1.11 |  | 0.630 |
| ns | 1312 |  | 99 | Readme: the library's job, in five lines of prose | 1.12 |  | 0.613 |
| walker |  | 1379 | 261 | Json::Scripts { file: package.json } |  |  | 0.620 |
| ns | 1446 |  | 134 | Readme Quick Start: the complete split.js program | 1.13 |  | 0.584 |
| ns | 1524 |  | 78 | Readme Quick Start console transcript (unknown-option error + suggestion) | 1.14 | 1.13 | 0.569 |
| ns | 1657 |  | 133 | Command roster 1/10 — construction, subcommands, help/output configuration (lib/command.js 13-288) | 2.1 |  | 0.542 |
| walker |  | 1659 | 280 | Json::Entry { file: package.json } |  |  | 0.586 |
| walker |  | 1712 | 53 | Code::CodeKey { rung: Names, file: lib/command.js, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| ns | 1767 |  | 110 | Command roster 2/10 — command-arguments, help command, hooks, action (316-556) | 2.2 | 2.1 | 0.564 |
| ns | 1851 |  | 84 | Command roster 3/10 — option creation and registration (585-805) | 2.3 | 2.2 | 0.548 |
| walker |  | 1921 | 209 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 0, line: 13 } |  |  | 0.604 |
| ns | 1998 |  | 147 | Command roster 4/10 — parsing-behaviour toggles and the option-value store (826-983) | 2.4 | 2.3 | 0.581 |
| ns | 2128 |  | 130 | Command roster 5/10 — parse entry points and stand-alone-executable dispatch (1001-1380) | 2.5 | 2.4 | 0.563 |
| walker |  | 2130 | 209 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 1, line: 13 } |  |  | 0.620 |
| ns | 2284 |  | 156 | Command roster 6/10 — argument processing, hook chaining, lookup and conflict checks (1403-1723) | 2.6 | 2.5 | 0.600 |
| walker |  | 2317 | 187 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 2, line: 13 } |  |  | 0.635 |
| ns | 2358 |  | 74 | Command roster 7/10 — parseOptions, opts, error, env/implied resolution (1748-1996) | 2.7 | 2.6 | 0.624 |
| ns | 2443 |  | 85 | Command roster 8/10 — the complete set of user-facing error reporters (2034-2162) | 2.8 | 2.7 | 0.612 |
| walker |  | 2521 | 204 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 3, line: 13 } |  |  | 0.655 |
| ns | 2595 |  | 152 | Command roster 9/10 — metadata and help-grouping accessors (2195-2437) | 2.9 | 2.8 | 0.633 |
| ns | 2703 |  | 108 | Command roster 10/10 — help output and help-option API (2450-2686) | 2.10 | 2.9 | 0.619 |
| walker |  | 2735 | 214 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 4, line: 13 } |  |  | 0.659 |
| ns | 2755 |  | 52 | lib/command.js module-level helpers and exports | 2.11 | 2.10 | 0.659 |
| ns | 2851 |  | 96 | lib/option.js roster 1/2 — Option's declaration methods (3-156) | 2.12 |  | 0.643 |
| walker |  | 2985 | 250 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 5, line: 13 } |  |  | 0.685 |
| ns | 2994 |  | 143 | lib/option.js roster 2/2 — remaining Option methods, DualOptions, module functions and exports | 2.13 | 2.12 | 0.666 |
| ns | 3122 |  | 128 | lib/argument.js — complete roster (150-line file) | 2.14 |  | 0.651 |
| walker |  | 3228 | 243 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 6, line: 13 } |  |  | 0.700 |
| walker |  | 3249 | 21 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 37, sub: 0, line: 885 } |  |  | 0.700 |
| walker |  | 3273 | 24 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 52, sub: 0, line: 1352 } |  |  | 0.700 |
| walker |  | 3316 | 43 | Code::CodeKey { rung: Names, file: lib/help.js, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| ns | 3353 |  | 231 | lib/error.js — both error classes in full | 2.15 |  | 0.676 |
| ns | 3396 |  | 43 | lib/suggestSimilar.js — complete symbol set | 2.16 |  | 0.671 |
| ns | 3476 |  | 80 | lib/help.js roster 1/4 — visibility and ordering (12-139) | 2.17 |  | 0.661 |
| walker |  | 3534 | 218 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 0, line: 12 } |  |  | 0.680 |
| ns | 3608 |  | 132 | lib/help.js roster 2/4 — term/description/width methods (162-372) | 2.18 | 2.17 | 0.677 |
| ns | 3804 |  | 196 | lib/help.js roster 3/4 — assembly plus the complete styleX hook set (403-606) | 2.19 | 2.18 | 0.657 |
| ns | 3874 |  | 70 | lib/help.js roster 4/4 — layout tail, stripColor, exports (618-747) | 2.20 | 2.19 | 0.653 |
| walker |  | 3878 | 344 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 1, line: 12 } |  |  | 0.706 |
| ns | 3891 |  | 17 | typings/ listing | 3.1 |  | 0.708 |
| walker |  | 3920 | 42 | Code::CodeKey { rung: Names, file: lib/option.js, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 3953 | 33 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 18, sub: 0, line: 268 } |  |  | 0.712 |
| ns | 4013 |  | 122 | typings/index.d.ts — every exported class declaration | 3.2 |  | 0.702 |
| walker |  | 4159 | 206 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 1, sub: 0, line: 3 } |  |  | 0.734 |
| walker |  | 4192 | 33 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 4, sub: 0, line: 121 } |  |  | 0.734 |
| walker |  | 4226 | 34 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 101, sub: 0, line: 2752 } |  |  | 0.734 |
| ns | 4258 |  | 245 | typings/index.d.ts — every exported interface, type alias, function and const | 3.3 | 3.2 | 0.717 |
| walker |  | 4276 | 50 | Code::CodeKey { rung: Names, file: lib/argument.js, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| ns | 4322 |  | 64 | typings/index.d.ts — Command's public instance properties | 3.4 |  | 0.715 |
| walker |  | 4379 | 103 | Code::CodeKey { rung: Decl, file: lib/argument.js, decl: 1, sub: 0, line: 3 } |  |  | 0.730 |
| walker |  | 4389 | 10 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 3, sub: 0, line: 48 } |  |  | 0.730 |
| ns | 4413 |  | 91 | Readme.md top-level section map (all H2 headings) | 3.5 |  | 0.733 |
| walker |  | 4427 | 38 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 88, sub: 0, line: 2392 } |  |  | 0.733 |
| ns | 4449 |  | 36 | docs/ listing | 3.6 |  | 0.736 |
| walker |  | 4465 | 38 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 89, sub: 0, line: 2401 } |  |  | 0.736 |
| walker |  | 4506 | 41 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 77, sub: 0, line: 2162 } |  |  | 0.736 |
| walker |  | 4534 | 28 | Code::CodeKey { rung: Names, file: lib/suggestSimilar.js, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| ns | 4564 |  | 115 | docs/*.md top-level heading map | 3.7 |  | 0.728 |
| walker |  | 4587 | 53 | Code::CodeKey { rung: Names, file: lib/error.js, decl: 0, sub: 0, line: 0 } |  |  | 0.729 |
| walker |  | 4603 | 16 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.730 |
| walker |  | 4626 | 23 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.731 |
| walker |  | 4643 | 17 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.731 |
| walker |  | 4685 | 42 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 2, sub: 0, line: 20 } |  |  | 0.731 |
| walker |  | 4703 | 18 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.731 |
| ns | 4729 |  | 165 | Readme.md subsection map 1/2 — Options and Commands (H3/H4, lines 211-725) | 3.8 |  | 0.718 |
| walker |  | 4747 | 44 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 51, sub: 0, line: 1202 } |  |  | 0.718 |
| ns | 4992 |  | 263 | Readme.md subsection map 2/2 — Automated help, Bits and pieces, Support (H3, lines 787-1172) | 3.9 | 3.8 | 0.698 |
| walker |  | 5135 | 388 | Code::CodeKey { rung: Names, file: typings/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| walker |  | 5150 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 4, sub: 0, line: 31 } |  |  | 0.733 |
| walker |  | 5165 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 171, sub: 0, line: 1100 } |  |  | 0.733 |
| walker |  | 5186 | 21 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 72, sub: 0, line: 342 } |  |  | 0.733 |
| walker |  | 5209 | 23 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.733 |
| walker |  | 5233 | 24 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 172, sub: 0, line: 1104 } |  |  | 0.733 |
| ns | 5241 |  | 249 | docs/ subsection map — the complete deprecation list plus options-in-depth subsections | 3.10 |  | 0.717 |
| walker |  | 5262 | 29 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 73, sub: 0, line: 345 } |  |  | 0.717 |
| walker |  | 5298 | 36 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 78, sub: 0, line: 370 } |  |  | 0.717 |
| walker |  | 5336 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 74, sub: 0, line: 349 } |  |  | 0.718 |
| walker |  | 5389 | 53 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 170, sub: 0, line: 1094 } |  |  | 0.718 |
| walker |  | 5451 | 62 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.718 |
| ns | 5478 |  | 237 | examples/ listing (complete, 45 entries) | 3.11 |  | 0.731 |
| walker |  | 5514 | 63 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 6, sub: 0, line: 40 } |  |  | 0.732 |
| ns | 5609 |  | 131 | typings: the OutputConfiguration shape | 3.12 |  | 0.723 |
| walker |  | 5645 | 131 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 75, sub: 0, line: 354 } |  |  | 0.736 |
| ns | 5748 |  | 139 | typings: ErrorOptions, ParseOptions, HelpContext, AddHelpTextContext bodies | 3.13 | 3.3 | 0.738 |
| walker |  | 5840 | 195 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.738 |
| ns | 5869 |  | 121 | typings: OptionValueSource members, CommandOptions, ExecutableCommandOptions, ParseOptionsResult | 3.14 | 3.3 | 0.740 |
| walker |  | 6291 | 451 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 15, sub: 0, line: 95 } |  |  | 0.740 |
| walker |  | 6483 | 192 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 0, line: 376 } |  |  | 0.746 |
| walker |  | 6517 | 34 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 84, sub: 0, line: 420 } |  |  | 0.746 |
| walker |  | 6557 | 40 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 85, sub: 0, line: 442 } |  |  | 0.746 |
| ns | 6682 |  | 813 | tests/ listing (complete, 113 entries) | 3.15 |  | 0.673 |
| walker |  | 6760 | 203 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 1, line: 376 } |  |  | 0.673 |
| ns | 6810 |  | 128 | tests/fixtures/, tests/fixtures-extensions/ and their subdirectories (complete) | 3.16 |  | 0.659 |
| walker |  | 6814 | 54 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 89, sub: 0, line: 487 } |  |  | 0.659 |
| ns | 6840 |  | 30 | .github/ and .github/workflows/ listings | 3.17 |  | 0.661 |
| walker |  | 6872 | 58 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 98, sub: 0, line: 542 } |  |  | 0.661 |
| ns | 6900 |  | 60 | Help's five data properties | 4.1 |  | 0.658 |
| walker |  | 7084 | 212 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 2, line: 376 } |  |  | 0.658 |
| walker |  | 7122 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 110, sub: 0, line: 648 } |  |  | 0.658 |
| ns | 7143 |  | 243 | parseOptions()'s documented contract | 4.2 |  | 0.650 |
| walker |  | 7160 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 113, sub: 0, line: 673 } |  |  | 0.650 |
| walker |  | 7210 | 50 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 112, sub: 0, line: 660 } |  |  | 0.650 |
| walker |  | 7260 | 50 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 115, sub: 0, line: 685 } |  |  | 0.650 |
| walker |  | 7316 | 56 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 111, sub: 0, line: 653 } |  |  | 0.650 |
| walker |  | 7372 | 56 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 114, sub: 0, line: 678 } |  |  | 0.650 |
| walker |  | 7379 | 7 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 103, sub: 0, line: 572 } |  |  | 0.650 |
| walker |  | 7386 | 7 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 105, sub: 0, line: 592 } |  |  | 0.650 |
| ns | 7538 |  | 395 | Option's complete field set (constructor body) | 4.3 |  | 0.637 |
| walker |  | 7567 | 181 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 3, line: 376 } |  |  | 0.637 |
| walker |  | 7586 | 19 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 119, sub: 0, line: 715 } |  |  | 0.637 |
| walker |  | 7623 | 37 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 123, sub: 0, line: 733 } |  |  | 0.637 |
| walker |  | 7825 | 202 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 4, line: 376 } |  |  | 0.637 |
| walker |  | 7855 | 30 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 132, sub: 0, line: 840 } |  |  | 0.637 |
| ns | 7892 |  | 354 | Command instance state 1/4 — commands, options, args and option values | 4.4 |  | 0.625 |
| ns | 8051 |  | 159 | Command instance state 2/4 — behaviour flags, descriptions, hooks, saved state | 4.5 | 4.4 | 0.620 |
| walker |  | 8063 | 208 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 0, line: 210 } |  |  | 0.620 |
| walker |  | 8101 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 32, sub: 0, line: 226 } |  |  | 0.620 |
| ns | 8270 |  | 219 | Command instance state 3/4 — the default _outputConfiguration | 4.6 | 4.5 | 0.614 |
| walker |  | 8278 | 177 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 1, line: 210 } |  |  | 0.614 |
| walker |  | 8466 | 188 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 2, line: 210 } |  |  | 0.614 |
| ns | 8482 |  | 212 | Command instance state 4/4 — help option/command and group headings | 4.7 | 4.6 | 0.608 |
| walker |  | 8619 | 153 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 3, line: 210 } |  |  | 0.608 |
| walker |  | 8664 | 45 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 67, sub: 0, line: 316 } |  |  | 0.608 |
| walker |  | 8713 | 49 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 69, sub: 0, line: 331 } |  |  | 0.608 |
| ns | 8764 |  | 282 | Argument's constructor — the `<req>` / `[opt]` / `name...` grammar | 4.8 |  | 0.597 |
| walker |  | 8932 | 219 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 5, line: 376 } |  |  | 0.597 |
| ns | 9094 |  | 330 | splitOptionFlags — the flag-string grammar and its error messages | 4.9 | 2.13 | 0.589 |
| ns | 9210 |  | 116 | useColor() — the colour environment-variable contract | 4.10 |  | 0.585 |
| walker |  | 9212 | 280 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 6, line: 376 } |  |  | 0.585 |
| walker |  | 9253 | 41 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 168, sub: 0, line: 1083 } |  |  | 0.585 |
| walker |  | 9263 | 10 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 51, sub: 0, line: 275 } |  |  | 0.585 |
| walker |  | 9274 | 11 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 57, sub: 0, line: 284 } |  |  | 0.585 |
| walker |  | 9285 | 11 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 161, sub: 0, line: 1047 } |  |  | 0.585 |
| walker |  | 9296 | 11 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 166, sub: 0, line: 1074 } |  |  | 0.585 |
| walker |  | 9308 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 40, sub: 0, line: 248 } |  |  | 0.585 |
| walker |  | 9320 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 45, sub: 0, line: 260 } |  |  | 0.585 |
| walker |  | 9332 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 46, sub: 0, line: 262 } |  |  | 0.585 |
| walker |  | 9344 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 48, sub: 0, line: 266 } |  |  | 0.585 |
| walker |  | 9356 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 70, sub: 0, line: 338 } |  |  | 0.585 |
| walker |  | 9369 | 13 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 47, sub: 0, line: 264 } |  |  | 0.585 |
| walker |  | 9383 | 14 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 52, sub: 0, line: 277 } |  |  | 0.585 |
| walker |  | 9398 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 44, sub: 0, line: 257 } |  |  | 0.585 |
| walker |  | 9413 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 61, sub: 0, line: 291 } |  |  | 0.585 |
| walker |  | 9428 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 63, sub: 0, line: 295 } |  |  | 0.585 |
| walker |  | 9443 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 64, sub: 0, line: 298 } |  |  | 0.585 |
| walker |  | 9458 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 96, sub: 0, line: 535 } |  |  | 0.585 |
| ns | 9471 |  | 261 | package.json scripts — how to test, lint, format and type-check | 5.1 | 1.4 | 0.590 |
| walker |  | 9473 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 97, sub: 0, line: 537 } |  |  | 0.590 |
| walker |  | 9489 | 16 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 62, sub: 0, line: 293 } |  |  | 0.590 |
| walker |  | 9505 | 16 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 78, sub: 0, line: 370 } |  |  | 0.590 |
| walker |  | 9518 | 13 | Code::CodeKey { rung: Names, file: typings/esm.d.mts, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 9535 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 35, sub: 0, line: 237 } |  |  | 0.590 |
| walker |  | 9552 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 36, sub: 0, line: 239 } |  |  | 0.590 |
| walker |  | 9569 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 37, sub: 0, line: 241 } |  |  | 0.590 |
| walker |  | 9586 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 38, sub: 0, line: 243 } |  |  | 0.590 |
| walker |  | 9604 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 33, sub: 0, line: 233 } |  |  | 0.590 |
| walker |  | 9622 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 34, sub: 0, line: 235 } |  |  | 0.590 |
| walker |  | 9640 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 112, sub: 0, line: 660 } |  |  | 0.590 |
| walker |  | 9658 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 115, sub: 0, line: 685 } |  |  | 0.590 |
| ns | 9673 |  | 202 | CONTRIBUTING.md — PR rules and the surfaces a change must update | 5.2 |  | 0.584 |
| walker |  | 9677 | 19 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 43, sub: 0, line: 255 } |  |  | 0.584 |
| walker |  | 9696 | 19 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 50, sub: 0, line: 272 } |  |  | 0.584 |
| walker |  | 9716 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 9, sub: 0, line: 67 } |  |  | 0.584 |
| walker |  | 9736 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 13, sub: 0, line: 87 } |  |  | 0.584 |
| walker |  | 9756 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 14, sub: 0, line: 92 } |  |  | 0.584 |
| walker |  | 9776 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 26, sub: 0, line: 189 } |  |  | 0.584 |
| walker |  | 9796 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 121, sub: 0, line: 723 } |  |  | 0.584 |
| walker |  | 9816 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 122, sub: 0, line: 728 } |  |  | 0.584 |
| walker |  | 9836 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 140, sub: 0, line: 897 } |  |  | 0.584 |
| ns | 9848 |  | 175 | package.json exports map | 5.3 | 1.4 | 0.590 |
| walker |  | 9856 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 142, sub: 0, line: 909 } |  |  | 0.590 |
| walker |  | 9904 | 48 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 54, sub: 0, line: 1403 } |  |  | 0.590 |
| walker |  | 9925 | 21 | Code::CodeKey { rung: Doc, file: lib/argument.js, decl: 4, sub: 0, line: 56 } |  |  | 0.590 |
| ns | 9940 |  | 92 | jest.config.js | 5.4 |  | 0.587 |
| walker |  | 9946 | 21 | Code::CodeKey { rung: Doc, file: lib/option.js, decl: 11, sub: 0, line: 165 } |  |  | 0.587 |
| walker |  | 9967 | 21 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 24, sub: 0, line: 179 } |  |  | 0.587 |
| walker |  | 9988 | 21 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 39, sub: 0, line: 246 } |  |  | 0.587 |
