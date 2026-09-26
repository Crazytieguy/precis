Score(3000)=0.812 I=0.943 C=0.699 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.804/0.832/0.904/0.812/0.708/0.711/0.693

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 81 |  | 81 | README identity: what Click is | 1.1 |  | 0.000 |
| walker |  | 131 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 182 |  | 101 | Click in three points | 1.2 |  | 0.583 |
| walker |  | 213 | 82 | Toml::Identity { file: pyproject.toml } |  |  | 0.584 |
| walker |  | 243 | 30 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.584 |
| walker |  | 270 | 27 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.355 |
| ns | 270 |  | 88 | Complete module roster of the package: src/click/ | 1.3 |  | 0.355 |
| walker |  | 281 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.355 |
| ns | 320 |  | 50 | Repository root listing | 1.4 |  | 0.527 |
| walker |  | 371 | 90 | Fs::DirListing { dir: src/click } |  |  | 0.810 |
| walker |  | 384 | 13 | Fs::DirListing { dir: .github } |  |  | 0.810 |
| walker |  | 407 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.811 |
| ns | 476 |  | 156 | The canonical hello-world program | 1.5 |  | 0.694 |
| walker |  | 482 | 75 | Code::CodeKey { rung: ModuleDoc, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| ns | 524 |  | 48 | The terminal session that program produces | 1.6 | 1.5 | 0.663 |
| walker |  | 581 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.819 |
| ns | 707 |  | 183 | Test suite listing: tests/ and tests/typing/ | 1.7 |  | 0.648 |
| walker |  | 773 | 192 | Fs::DirListing { dir: docs } |  |  | 0.667 |
| ns | 776 |  | 69 | Examples listing | 1.8 |  | 0.607 |
| walker |  | 788 | 15 | Fs::DirListing { dir: docs/_static } |  |  | 0.609 |
| walker |  | 802 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.610 |
| walker |  | 838 | 36 | Fs::DirListing { dir: examples } |  |  | 0.658 |
| walker |  | 965 | 127 | Fs::DirListing { dir: tests } |  |  | 0.767 |
| ns | 983 |  | 207 | Documentation listing: docs/ | 1.9 |  | 0.804 |
| walker |  | 1168 | 203 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.810 |
| ns | 1248 |  | 265 | Public API surface, part 1: object model and decorators | 2.1 |  | 0.795 |
| walker |  | 1360 | 192 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.825 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.809 |
| walker |  | 1553 | 193 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.834 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.820 |
| walker |  | 1749 | 196 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.843 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.811 |
| walker |  | 1905 | 156 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 4, line: 0 } |  |  | 0.852 |
| walker |  | 1961 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.902 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.878 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.854 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.846 |
| walker |  | 2181 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.913 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.884 |
| walker |  | 2392 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.885 |
| walker |  | 2403 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.885 |
| walker |  | 2429 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.885 |
| walker |  | 2455 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.885 |
| walker |  | 2481 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.885 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.864 |
| walker |  | 2514 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.864 |
| walker |  | 2556 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.864 |
| walker |  | 2598 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.864 |
| walker |  | 2641 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.864 |
| walker |  | 2685 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.864 |
| walker |  | 2755 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.864 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.831 |
| walker |  | 2825 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.831 |
| walker |  | 2895 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.831 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.812 |
| walker |  | 2966 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.812 |
| walker |  | 3055 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.812 |
| walker |  | 3161 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.812 |
| walker |  | 3173 | 12 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.812 |
| walker |  | 3187 | 14 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 33, sub: 0, line: 330 } |  |  | 0.812 |
| walker |  | 3203 | 16 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.813 |
| walker |  | 3220 | 17 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.813 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.779 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.764 |
| walker |  | 3526 | 306 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| walker |  | 3554 | 28 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.787 |
| walker |  | 3587 | 33 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.787 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.770 |
| walker |  | 3624 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.770 |
| walker |  | 3662 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.770 |
| walker |  | 3706 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.770 |
| walker |  | 3786 | 80 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.770 |
| walker |  | 3847 | 61 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.770 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.745 |
| walker |  | 3983 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.746 |
| walker |  | 4140 | 157 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.747 |
| walker |  | 4149 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 137, sub: 0, line: 3430 } |  |  | 0.747 |
| walker |  | 4187 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 139, sub: 0, line: 3450 } |  |  | 0.747 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.713 |
| walker |  | 4246 | 59 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 136, sub: 0, line: 3409 } |  |  | 0.713 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.708 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.712 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.705 |
| walker |  | 4690 | 444 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 99, sub: 0, line: 2054 } |  |  | 0.707 |
| walker |  | 4699 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 104, sub: 0, line: 2254 } |  |  | 0.707 |
| walker |  | 4709 | 10 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 109, sub: 0, line: 2318 } |  |  | 0.707 |
| walker |  | 4746 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 110, sub: 0, line: 2321 } |  |  | 0.707 |
| walker |  | 4786 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 108, sub: 0, line: 2285 } |  |  | 0.707 |
| walker |  | 4828 | 42 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 116, sub: 0, line: 2570 } |  |  | 0.707 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.690 |
| walker |  | 4876 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 103, sub: 0, line: 2249 } |  |  | 0.690 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.685 |
| walker |  | 4924 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 106, sub: 0, line: 2275 } |  |  | 0.685 |
| walker |  | 4971 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 107, sub: 0, line: 2280 } |  |  | 0.685 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.667 |
| walker |  | 5257 | 286 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 121, sub: 0, line: 2671 } |  |  | 0.669 |
| walker |  | 5293 | 36 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 133, sub: 0, line: 3319 } |  |  | 0.669 |
| walker |  | 5331 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 126, sub: 0, line: 2974 } |  |  | 0.669 |
| walker |  | 5371 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 124, sub: 0, line: 2935 } |  |  | 0.669 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.655 |
| walker |  | 5565 | 194 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 0, line: 185 } |  |  | 0.657 |
| walker |  | 5573 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 11, sub: 0, line: 459 } |  |  | 0.657 |
| walker |  | 5581 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 16, sub: 0, line: 549 } |  |  | 0.657 |
| walker |  | 5590 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 15, sub: 0, line: 511 } |  |  | 0.657 |
| walker |  | 5650 | 60 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 14, sub: 0, line: 497 } |  |  | 0.657 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.669 |
| walker |  | 5828 | 178 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 1, line: 185 } |  |  | 0.679 |
| walker |  | 5836 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 22, sub: 0, line: 657 } |  |  | 0.679 |
| walker |  | 5896 | 60 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 21, sub: 0, line: 639 } |  |  | 0.679 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.688 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.688 |
| walker |  | 6194 | 298 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 2, line: 185 } |  |  | 0.717 |
| walker |  | 6204 | 10 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 37, sub: 0, line: 797 } |  |  | 0.717 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.711 |
| walker |  | 6247 | 43 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 38, sub: 0, line: 800 } |  |  | 0.711 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.705 |
| walker |  | 6292 | 45 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 27, sub: 0, line: 718 } |  |  | 0.705 |
| walker |  | 6339 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 36, sub: 0, line: 792 } |  |  | 0.705 |
| walker |  | 6388 | 49 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 28, sub: 0, line: 723 } |  |  | 0.705 |
| walker |  | 6399 | 11 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 31, sub: 0, line: 758 } |  |  | 0.705 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.696 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.689 |
| walker |  | 6659 | 260 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 10, sub: 0, line: 289 } |  |  | 0.713 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.698 |
| walker |  | 6867 | 208 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 42, sub: 0, line: 903 } |  |  | 0.698 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.703 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.701 |
| walker |  | 7059 | 192 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 43, sub: 0, line: 965 } |  |  | 0.718 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.701 |
| walker |  | 7254 | 195 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 42, sub: 1, line: 903 } |  |  | 0.706 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.702 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.695 |
| walker |  | 7543 | 289 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 42, sub: 2, line: 903 } |  |  | 0.716 |
| walker |  | 7605 | 62 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 66, sub: 0, line: 1479 } |  |  | 0.716 |
| walker |  | 7673 | 68 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 59, sub: 0, line: 1210 } |  |  | 0.716 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.717 |
| walker |  | 7775 | 102 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 64, sub: 0, line: 1356 } |  |  | 0.717 |
| walker |  | 7881 | 106 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 65, sub: 0, line: 1366 } |  |  | 0.722 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.710 |
| walker |  | 7989 | 108 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 63, sub: 0, line: 1346 } |  |  | 0.710 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.702 |
| walker |  | 8269 | 280 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 122, sub: 0, line: 2745 } |  |  | 0.723 |
| walker |  | 8282 | 13 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 23, sub: 0, line: 676 } |  |  | 0.723 |
| walker |  | 8297 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.723 |
| walker |  | 8312 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 50, sub: 0, line: 1076 } |  |  | 0.723 |
| walker |  | 8327 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 67, sub: 0, line: 1511 } |  |  | 0.723 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.715 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.704 |
| walker |  | 8662 | 335 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 72, sub: 0, line: 1531 } |  |  | 0.704 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.693 |
| walker |  | 8878 | 216 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 72, sub: 1, line: 1531 } |  |  | 0.701 |
| walker |  | 8889 | 11 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 76, sub: 0, line: 1660 } |  |  | 0.701 |
| walker |  | 8900 | 11 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 79, sub: 0, line: 1709 } |  |  | 0.701 |
| walker |  | 8944 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 78, sub: 0, line: 1668 } |  |  | 0.701 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.693 |
| walker |  | 8988 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 81, sub: 0, line: 1717 } |  |  | 0.693 |
| walker |  | 9037 | 49 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 77, sub: 0, line: 1663 } |  |  | 0.693 |
| walker |  | 9086 | 49 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 80, sub: 0, line: 1712 } |  |  | 0.693 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.685 |
| walker |  | 9251 | 165 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 73, sub: 0, line: 1584 } |  |  | 0.697 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.689 |
| walker |  | 9430 | 179 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 72, sub: 2, line: 1531 } |  |  | 0.702 |
| walker |  | 9467 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 90, sub: 0, line: 1935 } |  |  | 0.702 |
| walker |  | 9483 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 24, sub: 0, line: 683 } |  |  | 0.702 |
| walker |  | 9499 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 52, sub: 0, line: 1111 } |  |  | 0.702 |
| walker |  | 9515 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 95, sub: 0, line: 2014 } |  |  | 0.702 |
| walker |  | 9533 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 56, sub: 0, line: 1166 } |  |  | 0.702 |
| walker |  | 9551 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 57, sub: 0, line: 1189 } |  |  | 0.702 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.693 |
| walker |  | 9569 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 58, sub: 0, line: 1201 } |  |  | 0.693 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.687 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.682 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.678 |
