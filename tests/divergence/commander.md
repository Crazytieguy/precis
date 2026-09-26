Score(3000)=0.582 I=0.794 C=0.427 ns_rows≤3K=27/65 grid(1000/1442/2080/3000/4327/6240/9000)=0.596/0.620/0.600/0.582/0.671/0.668/0.536

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
| walker |  | 464 | 57 | Json::Scripts { file: package.json } |  |  | 0.508 |
| walker |  | 557 | 93 | Markdown::HeadingsOutline { file: Readme.md } |  |  | 0.510 |
| ns | 577 |  | 136 | esm.mjs — the named-export ESM wrapper in full | 1.8 |  | 0.420 |
| walker |  | 603 | 46 | Markdown::CommandBlock { file: Readme.md, row: 1094 } |  |  | 0.420 |
| walker |  | 627 | 24 | Markdown::Section { file: Readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.420 |
| walker |  | 652 | 25 | Markdown::Section { file: Readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.420 |
| walker |  | 663 | 11 | Code::CodeKey { rung: Names, file: esm.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.421 |
| ns | 679 |  | 102 | Repository root listing (complete) | 1.9 |  | 0.580 |
| walker |  | 760 | 97 | Code::CodeKey { rung: Decl, file: esm.mjs, decl: 1, sub: 0, line: 4 } |  |  | 0.660 |
| ns | 928 |  | 249 | docs/terminology.md in full — the domain vocabulary | 1.10 |  | 0.589 |
| walker |  | 997 | 237 | Fs::DirListing { dir: examples } |  |  | 0.596 |
| walker |  | 1011 | 14 | Code::CodeKey { rung: Doc, file: esm.mjs, decl: 1, sub: 0, line: 4 } |  |  | 0.617 |
| walker |  | 1169 | 158 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| ns | 1213 |  | 285 | docs/parsing-and-hooks.md in full — the parse life cycle | 1.11 |  | 0.636 |
| ns | 1312 |  | 99 | Readme: the library's job, in five lines of prose | 1.12 |  | 0.619 |
| walker |  | 1373 | 204 | Json::ScriptsTail { file: package.json } |  |  | 0.620 |
| ns | 1446 |  | 134 | Readme Quick Start: the complete split.js program | 1.13 |  | 0.584 |
| walker |  | 1450 | 77 | Markdown::Section { file: Readme.md, section_index: 29, keeps_default_concavity: false } |  |  | 0.584 |
| ns | 1524 |  | 78 | Readme Quick Start console transcript (unknown-option error + suggestion) | 1.14 | 1.13 | 0.569 |
| ns | 1657 |  | 133 | Command roster 1/10 — construction, subcommands, help/output configuration (lib/command.js 13-288) | 2.1 |  | 0.542 |
| walker |  | 1730 | 280 | Json::Entry { file: package.json } |  |  | 0.586 |
| ns | 1767 |  | 110 | Command roster 2/10 — command-arguments, help command, hooks, action (316-556) | 2.2 | 2.1 | 0.563 |
| walker |  | 1783 | 53 | Code::CodeKey { rung: Names, file: lib/command.js, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| ns | 1851 |  | 84 | Command roster 3/10 — option creation and registration (585-805) | 2.3 | 2.2 | 0.548 |
| walker |  | 1981 | 198 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 0, line: 13 } |  |  | 0.597 |
| ns | 1998 |  | 147 | Command roster 4/10 — parsing-behaviour toggles and the option-value store (826-983) | 2.4 | 2.3 | 0.574 |
| ns | 2128 |  | 130 | Command roster 5/10 — parse entry points and stand-alone-executable dispatch (1001-1380) | 2.5 | 2.4 | 0.556 |
| walker |  | 2188 | 207 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 1, line: 13 } |  |  | 0.593 |
| ns | 2284 |  | 156 | Command roster 6/10 — argument processing, hook chaining, lookup and conflict checks (1403-1723) | 2.6 | 2.5 | 0.574 |
| ns | 2358 |  | 74 | Command roster 7/10 — parseOptions, opts, error, env/implied resolution (1748-1996) | 2.7 | 2.6 | 0.564 |
| walker |  | 2411 | 223 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 2, line: 13 } |  |  | 0.603 |
| ns | 2443 |  | 85 | Command roster 8/10 — the complete set of user-facing error reporters (2034-2162) | 2.8 | 2.7 | 0.591 |
| ns | 2595 |  | 152 | Command roster 9/10 — metadata and help-grouping accessors (2195-2437) | 2.9 | 2.8 | 0.570 |
| ns | 2703 |  | 108 | Command roster 10/10 — help output and help-option API (2450-2686) | 2.10 | 2.9 | 0.558 |
| walker |  | 2733 | 322 | Code::CodeKey { rung: Decl, file: lib/command.js, decl: 1, sub: 3, line: 13 } |  |  | 0.610 |
| ns | 2755 |  | 52 | lib/command.js module-level helpers and exports | 2.11 | 2.10 | 0.610 |
| walker |  | 2776 | 43 | Code::CodeKey { rung: Names, file: lib/help.js, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| ns | 2851 |  | 96 | lib/option.js roster 1/2 — Option's declaration methods (3-156) | 2.12 |  | 0.596 |
| walker |  | 2987 | 211 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 0, line: 12 } |  |  | 0.599 |
| ns | 2994 |  | 143 | lib/option.js roster 2/2 — remaining Option methods, DualOptions, module functions and exports | 2.13 | 2.12 | 0.582 |
| ns | 3122 |  | 128 | lib/argument.js — complete roster (150-line file) | 2.14 |  | 0.569 |
| walker |  | 3338 | 351 | Code::CodeKey { rung: Decl, file: lib/help.js, decl: 1, sub: 1, line: 12 } |  |  | 0.573 |
| ns | 3353 |  | 231 | lib/error.js — both error classes in full | 2.15 |  | 0.554 |
| walker |  | 3380 | 42 | Code::CodeKey { rung: Names, file: lib/option.js, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| ns | 3396 |  | 43 | lib/suggestSimilar.js — complete symbol set | 2.16 |  | 0.552 |
| walker |  | 3413 | 33 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 17, sub: 0, line: 268 } |  |  | 0.556 |
| ns | 3476 |  | 80 | lib/help.js roster 1/4 — visibility and ordering (12-139) | 2.17 |  | 0.566 |
| walker |  | 3604 | 191 | Code::CodeKey { rung: Decl, file: lib/option.js, decl: 1, sub: 0, line: 3 } |  |  | 0.601 |
| ns | 3608 |  | 132 | lib/help.js roster 2/4 — term/description/width methods (162-372) | 2.18 | 2.17 | 0.613 |
| walker |  | 3654 | 50 | Code::CodeKey { rung: Names, file: lib/argument.js, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 3742 | 88 | Code::CodeKey { rung: Decl, file: lib/argument.js, decl: 1, sub: 0, line: 3 } |  |  | 0.633 |
| walker |  | 3776 | 34 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 67, sub: 0, line: 2752 } |  |  | 0.633 |
| walker |  | 3786 | 10 | Code::CodeKey { rung: Body, file: lib/argument.js, decl: 3, sub: 0, line: 48 } |  |  | 0.633 |
| ns | 3804 |  | 196 | lib/help.js roster 3/4 — assembly plus the complete styleX hook set (403-606) | 2.19 | 2.18 | 0.647 |
| walker |  | 3814 | 28 | Code::CodeKey { rung: Names, file: lib/suggestSimilar.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3867 | 53 | Code::CodeKey { rung: Names, file: lib/error.js, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| ns | 3874 |  | 70 | lib/help.js roster 4/4 — layout tail, stripColor, exports (618-747) | 2.20 | 2.19 | 0.655 |
| walker |  | 3883 | 16 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.656 |
| ns | 3891 |  | 17 | typings/ listing | 3.1 |  | 0.658 |
| walker |  | 3906 | 23 | Code::CodeKey { rung: Decl, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.659 |
| ns | 4013 |  | 122 | typings/index.d.ts — every exported class declaration | 3.2 |  | 0.650 |
| ns | 4258 |  | 245 | typings/index.d.ts — every exported interface, type alias, function and const | 3.3 | 3.2 | 0.635 |
| walker |  | 4294 | 388 | Code::CodeKey { rung: Names, file: typings/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 4309 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 4, sub: 0, line: 31 } |  |  | 0.676 |
| ns | 4322 |  | 64 | typings/index.d.ts — Command's public instance properties | 3.4 |  | 0.671 |
| walker |  | 4324 | 15 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 171, sub: 0, line: 1100 } |  |  | 0.671 |
| walker |  | 4345 | 21 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 72, sub: 0, line: 342 } |  |  | 0.671 |
| walker |  | 4368 | 23 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.671 |
| walker |  | 4392 | 24 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 172, sub: 0, line: 1104 } |  |  | 0.671 |
| ns | 4413 |  | 91 | Readme.md top-level section map (all H2 headings) | 3.5 |  | 0.675 |
| walker |  | 4421 | 29 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 73, sub: 0, line: 345 } |  |  | 0.675 |
| ns | 4449 |  | 36 | docs/ listing | 3.6 |  | 0.678 |
| walker |  | 4457 | 36 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 78, sub: 0, line: 370 } |  |  | 0.679 |
| walker |  | 4495 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 74, sub: 0, line: 349 } |  |  | 0.679 |
| walker |  | 4548 | 53 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 170, sub: 0, line: 1094 } |  |  | 0.680 |
| ns | 4564 |  | 115 | docs/*.md top-level heading map | 3.7 |  | 0.671 |
| walker |  | 4610 | 62 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.671 |
| walker |  | 4673 | 63 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 6, sub: 0, line: 40 } |  |  | 0.672 |
| ns | 4729 |  | 165 | Readme.md subsection map 1/2 — Options and Commands (H3/H4, lines 211-725) | 3.8 |  | 0.660 |
| walker |  | 4804 | 131 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 75, sub: 0, line: 354 } |  |  | 0.661 |
| ns | 4992 |  | 263 | Readme.md subsection map 2/2 — Automated help, Bits and pieces, Support (H3, lines 787-1172) | 3.9 | 3.8 | 0.643 |
| walker |  | 4999 | 195 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.643 |
| ns | 5241 |  | 249 | docs/ subsection map — the complete deprecation list plus options-in-depth subsections | 3.10 |  | 0.629 |
| walker |  | 5450 | 451 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 15, sub: 0, line: 95 } |  |  | 0.629 |
| ns | 5478 |  | 237 | examples/ listing (complete, 45 entries) | 3.11 |  | 0.650 |
| ns | 5609 |  | 131 | typings: the OutputConfiguration shape | 3.12 |  | 0.655 |
| walker |  | 5636 | 186 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 0, line: 376 } |  |  | 0.662 |
| walker |  | 5670 | 34 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 84, sub: 0, line: 420 } |  |  | 0.662 |
| walker |  | 5710 | 40 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 85, sub: 0, line: 442 } |  |  | 0.662 |
| ns | 5748 |  | 139 | typings: ErrorOptions, ParseOptions, HelpContext, AddHelpTextContext bodies | 3.13 | 3.3 | 0.666 |
| ns | 5869 |  | 121 | typings: OptionValueSource members, CommandOptions, ExecutableCommandOptions, ParseOptionsResult | 3.14 | 3.3 | 0.668 |
| walker |  | 5913 | 203 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 1, line: 376 } |  |  | 0.668 |
| walker |  | 5967 | 54 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 89, sub: 0, line: 487 } |  |  | 0.668 |
| walker |  | 6025 | 58 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 98, sub: 0, line: 542 } |  |  | 0.668 |
| walker |  | 6228 | 203 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 0, line: 210 } |  |  | 0.668 |
| walker |  | 6266 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 32, sub: 0, line: 226 } |  |  | 0.668 |
| walker |  | 6443 | 177 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 1, line: 210 } |  |  | 0.668 |
| walker |  | 6631 | 188 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 2, line: 210 } |  |  | 0.668 |
| ns | 6682 |  | 813 | tests/ listing (complete, 113 entries) | 3.15 |  | 0.602 |
| walker |  | 6789 | 158 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 30, sub: 3, line: 210 } |  |  | 0.602 |
| ns | 6810 |  | 128 | tests/fixtures/, tests/fixtures-extensions/ and their subdirectories (complete) | 3.16 |  | 0.590 |
| walker |  | 6834 | 45 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 67, sub: 0, line: 316 } |  |  | 0.590 |
| ns | 6840 |  | 30 | .github/ and .github/workflows/ listings | 3.17 |  | 0.593 |
| walker |  | 6883 | 49 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 69, sub: 0, line: 331 } |  |  | 0.593 |
| ns | 6900 |  | 60 | Help's five data properties | 4.1 |  | 0.590 |
| walker |  | 7095 | 212 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 2, line: 376 } |  |  | 0.590 |
| walker |  | 7133 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 110, sub: 0, line: 648 } |  |  | 0.590 |
| ns | 7143 |  | 243 | parseOptions()'s documented contract | 4.2 |  | 0.582 |
| walker |  | 7171 | 38 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 113, sub: 0, line: 673 } |  |  | 0.582 |
| walker |  | 7221 | 50 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 112, sub: 0, line: 660 } |  |  | 0.582 |
| walker |  | 7271 | 50 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 115, sub: 0, line: 685 } |  |  | 0.582 |
| walker |  | 7327 | 56 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 111, sub: 0, line: 653 } |  |  | 0.582 |
| walker |  | 7383 | 56 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 114, sub: 0, line: 678 } |  |  | 0.582 |
| ns | 7538 |  | 395 | Option's complete field set (constructor body) | 4.3 |  | 0.571 |
| walker |  | 7564 | 181 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 3, line: 376 } |  |  | 0.571 |
| walker |  | 7583 | 19 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 119, sub: 0, line: 715 } |  |  | 0.571 |
| walker |  | 7620 | 37 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 123, sub: 0, line: 733 } |  |  | 0.571 |
| walker |  | 7822 | 202 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 4, line: 376 } |  |  | 0.571 |
| walker |  | 7852 | 30 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 132, sub: 0, line: 840 } |  |  | 0.571 |
| walker |  | 7859 | 7 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 103, sub: 0, line: 572 } |  |  | 0.571 |
| walker |  | 7866 | 7 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 105, sub: 0, line: 592 } |  |  | 0.571 |
| ns | 7892 |  | 354 | Command instance state 1/4 — commands, options, args and option values | 4.4 |  | 0.560 |
| ns | 8051 |  | 159 | Command instance state 2/4 — behaviour flags, descriptions, hooks, saved state | 4.5 | 4.4 | 0.556 |
| walker |  | 8085 | 219 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 5, line: 376 } |  |  | 0.556 |
| ns | 8270 |  | 219 | Command instance state 3/4 — the default _outputConfiguration | 4.6 | 4.5 | 0.550 |
| walker |  | 8371 | 286 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 80, sub: 6, line: 376 } |  |  | 0.550 |
| walker |  | 8412 | 41 | Code::CodeKey { rung: Decl, file: typings/index.d.ts, decl: 168, sub: 0, line: 1083 } |  |  | 0.550 |
| walker |  | 8422 | 10 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 51, sub: 0, line: 275 } |  |  | 0.550 |
| walker |  | 8433 | 11 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 57, sub: 0, line: 284 } |  |  | 0.550 |
| walker |  | 8444 | 11 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 161, sub: 0, line: 1047 } |  |  | 0.550 |
| walker |  | 8455 | 11 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 166, sub: 0, line: 1074 } |  |  | 0.550 |
| walker |  | 8467 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 40, sub: 0, line: 248 } |  |  | 0.550 |
| walker |  | 8479 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 45, sub: 0, line: 260 } |  |  | 0.550 |
| ns | 8482 |  | 212 | Command instance state 4/4 — help option/command and group headings | 4.7 | 4.6 | 0.545 |
| walker |  | 8491 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 46, sub: 0, line: 262 } |  |  | 0.545 |
| walker |  | 8503 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 48, sub: 0, line: 266 } |  |  | 0.545 |
| walker |  | 8515 | 12 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 70, sub: 0, line: 338 } |  |  | 0.545 |
| walker |  | 8528 | 13 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 47, sub: 0, line: 264 } |  |  | 0.545 |
| walker |  | 8542 | 14 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 52, sub: 0, line: 277 } |  |  | 0.545 |
| walker |  | 8557 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 44, sub: 0, line: 257 } |  |  | 0.545 |
| walker |  | 8572 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 61, sub: 0, line: 291 } |  |  | 0.545 |
| walker |  | 8587 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 63, sub: 0, line: 295 } |  |  | 0.545 |
| walker |  | 8602 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 64, sub: 0, line: 298 } |  |  | 0.545 |
| walker |  | 8617 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 96, sub: 0, line: 535 } |  |  | 0.545 |
| walker |  | 8632 | 15 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 97, sub: 0, line: 537 } |  |  | 0.545 |
| walker |  | 8648 | 16 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 62, sub: 0, line: 293 } |  |  | 0.545 |
| walker |  | 8664 | 16 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 78, sub: 0, line: 370 } |  |  | 0.545 |
| walker |  | 8681 | 17 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 1, sub: 0, line: 4 } |  |  | 0.545 |
| walker |  | 8698 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 35, sub: 0, line: 237 } |  |  | 0.545 |
| walker |  | 8715 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 36, sub: 0, line: 239 } |  |  | 0.545 |
| walker |  | 8732 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 37, sub: 0, line: 241 } |  |  | 0.545 |
| walker |  | 8749 | 17 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 38, sub: 0, line: 243 } |  |  | 0.545 |
| ns | 8764 |  | 282 | Argument's constructor — the `<req>` / `[opt]` / `name...` grammar | 4.8 |  | 0.536 |
| walker |  | 8790 | 41 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 48, sub: 0, line: 2162 } |  |  | 0.536 |
| walker |  | 8832 | 42 | Code::CodeKey { rung: Doc, file: lib/command.js, decl: 2, sub: 0, line: 20 } |  |  | 0.536 |
| walker |  | 8850 | 18 | Code::CodeKey { rung: Doc, file: lib/error.js, decl: 3, sub: 0, line: 25 } |  |  | 0.536 |
| walker |  | 8868 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 33, sub: 0, line: 233 } |  |  | 0.536 |
| walker |  | 8886 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 34, sub: 0, line: 235 } |  |  | 0.536 |
| walker |  | 8904 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 112, sub: 0, line: 660 } |  |  | 0.536 |
| walker |  | 8922 | 18 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 115, sub: 0, line: 685 } |  |  | 0.536 |
| walker |  | 8941 | 19 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 43, sub: 0, line: 255 } |  |  | 0.536 |
| walker |  | 8960 | 19 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 50, sub: 0, line: 272 } |  |  | 0.536 |
| ns | 9094 |  | 330 | splitOptionFlags — the flag-string grammar and its error messages | 4.9 | 2.13 | 0.528 |
| ns | 9210 |  | 116 | useColor() — the colour environment-variable contract | 4.10 |  | 0.525 |
| ns | 9471 |  | 261 | package.json scripts — how to test, lint, format and type-check | 5.1 | 1.4 | 0.531 |
| ns | 9673 |  | 202 | CONTRIBUTING.md — PR rules and the surfaces a change must update | 5.2 |  | 0.526 |
| walker |  | 9773 | 813 | Fs::DirListing { dir: tests } |  |  | 0.615 |
| walker |  | 9802 | 29 | Fs::DirListing { dir: tests/fixtures-extensions } |  |  | 0.615 |
| ns | 9848 |  | 175 | package.json exports map | 5.3 | 1.4 | 0.620 |
| walker |  | 9895 | 93 | Fs::DirListing { dir: tests/fixtures } |  |  | 0.634 |
| walker |  | 9898 | 3 | Fs::DirListing { dir: tests/fixtures/another-dir } |  |  | 0.636 |
| walker |  | 9901 | 3 | Fs::DirListing { dir: tests/fixtures/other-dir } |  |  | 0.637 |
| ns | 9940 |  | 92 | jest.config.js | 5.4 |  | 0.634 |
| walker |  | 9956 | 55 | Code::CodeKey { rung: Names, file: typings/esm.d.mts, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 9976 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 9, sub: 0, line: 67 } |  |  | 0.634 |
| walker |  | 9996 | 20 | Code::CodeKey { rung: Doc, file: typings/index.d.ts, decl: 13, sub: 0, line: 87 } |  |  | 0.634 |
