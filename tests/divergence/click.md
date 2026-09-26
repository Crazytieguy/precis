Score(3000)=0.837 I=0.952 C=0.736 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.804/0.832/0.904/0.837/0.712/0.779/0.718

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
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.863 |
| walker |  | 2540 | 359 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.889 |
| walker |  | 2551 | 11 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.889 |
| walker |  | 2574 | 23 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.889 |
| walker |  | 2597 | 23 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.889 |
| walker |  | 2623 | 26 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.889 |
| walker |  | 2667 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 62, sub: 0, line: 1516 } |  |  | 0.889 |
| walker |  | 2759 | 92 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 83, sub: 0, line: 1988 } |  |  | 0.889 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.855 |
| walker |  | 2808 | 49 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 84, sub: 0, line: 2004 } |  |  | 0.855 |
| walker |  | 2944 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.857 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.837 |
| walker |  | 3141 | 197 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 123, sub: 0, line: 3399 } |  |  | 0.837 |
| walker |  | 3160 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 127, sub: 0, line: 3450 } |  |  | 0.837 |
| walker |  | 3207 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 124, sub: 0, line: 3409 } |  |  | 0.838 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.803 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.788 |
| walker |  | 3565 | 358 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 109, sub: 0, line: 2671 } |  |  | 0.790 |
| walker |  | 3581 | 16 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 112, sub: 0, line: 2935 } |  |  | 0.790 |
| walker |  | 3600 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 114, sub: 0, line: 2974 } |  |  | 0.790 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.773 |
| walker |  | 3619 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 121, sub: 0, line: 3319 } |  |  | 0.773 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.748 |
| walker |  | 3887 | 268 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 110, sub: 0, line: 2745 } |  |  | 0.750 |
| walker |  | 4088 | 201 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 0, line: 185 } |  |  | 0.751 |
| walker |  | 4135 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 14, sub: 0, line: 497 } |  |  | 0.751 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.718 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.712 |
| walker |  | 4384 | 249 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 10, sub: 0, line: 289 } |  |  | 0.745 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.749 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.742 |
| walker |  | 4581 | 197 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 1, line: 185 } |  |  | 0.751 |
| walker |  | 4628 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 21, sub: 0, line: 639 } |  |  | 0.751 |
| walker |  | 4810 | 182 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 2, line: 185 } |  |  | 0.767 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.748 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.743 |
| walker |  | 4915 | 105 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 3, line: 185 } |  |  | 0.753 |
| walker |  | 4944 | 29 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 34, sub: 0, line: 800 } |  |  | 0.753 |
| walker |  | 4955 | 11 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 29, sub: 0, line: 758 } |  |  | 0.753 |
| walker |  | 5174 | 219 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 38, sub: 0, line: 903 } |  |  | 0.753 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.761 |
| walker |  | 5355 | 181 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 39, sub: 0, line: 965 } |  |  | 0.782 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.766 |
| walker |  | 5550 | 195 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 38, sub: 1, line: 903 } |  |  | 0.772 |
| walker |  | 5727 | 177 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 38, sub: 2, line: 903 } |  |  | 0.784 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.766 |
| walker |  | 5783 | 56 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 55, sub: 0, line: 1210 } |  |  | 0.766 |
| walker |  | 5910 | 127 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 38, sub: 3, line: 903 } |  |  | 0.776 |
| walker |  | 5960 | 50 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 60, sub: 0, line: 1479 } |  |  | 0.776 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.781 |
| walker |  | 6053 | 93 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 59, sub: 0, line: 1366 } |  |  | 0.787 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.784 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.777 |
| walker |  | 6267 | 214 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 89, sub: 0, line: 2054 } |  |  | 0.779 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.773 |
| walker |  | 6283 | 16 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 96, sub: 0, line: 2285 } |  |  | 0.773 |
| walker |  | 6302 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 2249 } |  |  | 0.773 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.763 |
| walker |  | 6502 | 200 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 89, sub: 1, line: 2054 } |  |  | 0.771 |
| walker |  | 6522 | 20 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 98, sub: 0, line: 2321 } |  |  | 0.771 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.763 |
| walker |  | 6652 | 130 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 89, sub: 2, line: 2054 } |  |  | 0.772 |
| walker |  | 6677 | 25 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 104, sub: 0, line: 2570 } |  |  | 0.772 |
| walker |  | 6690 | 13 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 23, sub: 0, line: 676 } |  |  | 0.772 |
| walker |  | 6705 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.772 |
| walker |  | 6720 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 46, sub: 0, line: 1076 } |  |  | 0.772 |
| walker |  | 6735 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 61, sub: 0, line: 1511 } |  |  | 0.772 |
| walker |  | 6751 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 24, sub: 0, line: 683 } |  |  | 0.772 |
| walker |  | 6767 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 48, sub: 0, line: 1111 } |  |  | 0.772 |
| walker |  | 6783 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 85, sub: 0, line: 2014 } |  |  | 0.772 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.756 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.759 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.757 |
| walker |  | 7118 | 335 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 66, sub: 0, line: 1531 } |  |  | 0.757 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.739 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.735 |
| walker |  | 7324 | 206 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 66, sub: 1, line: 1531 } |  |  | 0.739 |
| walker |  | 7342 | 18 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 70, sub: 0, line: 1668 } |  |  | 0.739 |
| walker |  | 7360 | 18 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 71, sub: 0, line: 1717 } |  |  | 0.739 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.731 |
| walker |  | 7513 | 153 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 67, sub: 0, line: 1584 } |  |  | 0.745 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.734 |
| walker |  | 7714 | 201 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 66, sub: 2, line: 1531 } |  |  | 0.745 |
| walker |  | 7729 | 15 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 80, sub: 0, line: 1935 } |  |  | 0.745 |
| walker |  | 7899 | 170 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 90, sub: 0, line: 2147 } |  |  | 0.747 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.734 |
| walker |  | 8072 | 173 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 90, sub: 1, line: 2147 } |  |  | 0.739 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.730 |
| walker |  | 8185 | 113 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 90, sub: 2, line: 2147 } |  |  | 0.743 |
| walker |  | 8203 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 52, sub: 0, line: 1166 } |  |  | 0.743 |
| walker |  | 8221 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 53, sub: 0, line: 1189 } |  |  | 0.743 |
| walker |  | 8239 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 54, sub: 0, line: 1201 } |  |  | 0.743 |
| walker |  | 8260 | 21 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 74, sub: 0, line: 1812 } |  |  | 0.743 |
| walker |  | 8292 | 32 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 31, sub: 0, line: 772 } |  |  | 0.743 |
| walker |  | 8324 | 32 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 32, sub: 0, line: 778 } |  |  | 0.743 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.734 |
| walker |  | 8360 | 36 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 77, sub: 0, line: 1825 } |  |  | 0.734 |
| walker |  | 8397 | 37 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 50, sub: 0, line: 1127 } |  |  | 0.734 |
| walker |  | 8436 | 39 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 57, sub: 0, line: 1283 } |  |  | 0.734 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.723 |
| walker |  | 8773 | 337 | Code::CodeKey { rung: Names, file: src/click/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 8794 | 21 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 33, sub: 0, line: 527 } |  |  | 0.724 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.713 |
| walker |  | 8826 | 32 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 27, sub: 0, line: 411 } |  |  | 0.713 |
| walker |  | 8870 | 44 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 25, sub: 0, line: 341 } |  |  | 0.713 |
| walker |  | 8921 | 51 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 34, sub: 0, line: 582 } |  |  | 0.713 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.718 |
| walker |  | 8980 | 59 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 29, sub: 0, line: 502 } |  |  | 0.718 |
| walker |  | 9044 | 64 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 23, sub: 0, line: 226 } |  |  | 0.718 |
| walker |  | 9117 | 73 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 26, sub: 0, line: 362 } |  |  | 0.718 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.709 |
| walker |  | 9237 | 120 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 16, sub: 0, line: 201 } |  |  | 0.709 |
| walker |  | 9284 | 47 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 20, sub: 0, line: 211 } |  |  | 0.709 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.701 |
| walker |  | 9449 | 165 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 6, sub: 0, line: 113 } |  |  | 0.701 |
| walker |  | 9496 | 47 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 14, sub: 0, line: 188 } |  |  | 0.701 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.692 |
| walker |  | 9565 | 69 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 7, sub: 0, line: 120 } |  |  | 0.692 |
| walker |  | 9580 | 15 | Code::CodeKey { rung: Doc, file: src/click/utils.py, decl: 4, sub: 0, line: 50 } |  |  | 0.692 |
| walker |  | 9596 | 16 | Code::CodeKey { rung: Doc, file: src/click/utils.py, decl: 11, sub: 0, line: 173 } |  |  | 0.692 |
| walker |  | 9636 | 40 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 45, sub: 0, line: 1065 } |  |  | 0.692 |
| walker |  | 9653 | 17 | Code::CodeKey { rung: Doc, file: src/click/utils.py, decl: 3, sub: 0, line: 37 } |  |  | 0.692 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.686 |
| walker |  | 9858 | 205 | Code::CodeKey { rung: Names, file: src/click/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.685 |
| walker |  | 9870 | 12 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 37, sub: 0, line: 422 } |  |  | 0.685 |
| walker |  | 9885 | 15 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 14, sub: 0, line: 181 } |  |  | 0.685 |
| walker |  | 9902 | 17 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 2, sub: 0, line: 32 } |  |  | 0.685 |
| walker |  | 9923 | 21 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 25, sub: 0, line: 248 } |  |  | 0.685 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.681 |
| walker |  | 9973 | 50 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 22, sub: 0, line: 222 } |  |  | 0.681 |
| walker |  | 9995 | 22 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 23, sub: 0, line: 225 } |  |  | 0.681 |
| walker |  | 9995 | 0 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 19, sub: 0, line: 210 } |  |  | 0.681 |
