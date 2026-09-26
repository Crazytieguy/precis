Score(3000)=0.837 I=0.952 C=0.736 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.804/0.832/0.904/0.837/0.712/0.782/0.720

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
| walker |  | 2548 | 367 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.889 |
| walker |  | 2559 | 11 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.889 |
| walker |  | 2574 | 15 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.889 |
| walker |  | 2597 | 23 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.889 |
| walker |  | 2623 | 26 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.889 |
| walker |  | 2667 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.889 |
| walker |  | 2759 | 92 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.889 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.855 |
| walker |  | 2808 | 49 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.855 |
| walker |  | 2944 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.857 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.837 |
| walker |  | 3141 | 197 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.837 |
| walker |  | 3160 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 139, sub: 0, line: 3450 } |  |  | 0.837 |
| walker |  | 3207 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 136, sub: 0, line: 3409 } |  |  | 0.838 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.803 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.788 |
| walker |  | 3565 | 358 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 121, sub: 0, line: 2671 } |  |  | 0.790 |
| walker |  | 3581 | 16 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 124, sub: 0, line: 2935 } |  |  | 0.790 |
| walker |  | 3600 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 126, sub: 0, line: 2974 } |  |  | 0.790 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.773 |
| walker |  | 3619 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 133, sub: 0, line: 3319 } |  |  | 0.773 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.748 |
| walker |  | 3887 | 268 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 122, sub: 0, line: 2745 } |  |  | 0.750 |
| walker |  | 4088 | 201 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 0, line: 185 } |  |  | 0.751 |
| walker |  | 4135 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 14, sub: 0, line: 497 } |  |  | 0.751 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.718 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.712 |
| walker |  | 4384 | 249 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 10, sub: 0, line: 289 } |  |  | 0.745 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.749 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.742 |
| walker |  | 4581 | 197 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 1, line: 185 } |  |  | 0.751 |
| walker |  | 4628 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 21, sub: 0, line: 639 } |  |  | 0.751 |
| walker |  | 4823 | 195 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 2, line: 185 } |  |  | 0.764 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.745 |
| walker |  | 4842 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 28, sub: 0, line: 723 } |  |  | 0.745 |
| walker |  | 4862 | 20 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 27, sub: 0, line: 718 } |  |  | 0.745 |
| walker |  | 4873 | 11 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 31, sub: 0, line: 758 } |  |  | 0.745 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.739 |
| walker |  | 5134 | 261 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 3, line: 185 } |  |  | 0.764 |
| walker |  | 5161 | 27 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 36, sub: 0, line: 792 } |  |  | 0.764 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.772 |
| walker |  | 5190 | 29 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 38, sub: 0, line: 800 } |  |  | 0.772 |
| walker |  | 5203 | 13 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 23, sub: 0, line: 676 } |  |  | 0.772 |
| walker |  | 5422 | 219 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 42, sub: 0, line: 903 } |  |  | 0.772 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.755 |
| walker |  | 5603 | 181 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 43, sub: 0, line: 965 } |  |  | 0.776 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.758 |
| walker |  | 5798 | 195 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 42, sub: 1, line: 903 } |  |  | 0.764 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.770 |
| walker |  | 5975 | 177 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 42, sub: 2, line: 903 } |  |  | 0.781 |
| walker |  | 6031 | 56 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 59, sub: 0, line: 1210 } |  |  | 0.781 |
| walker |  | 6046 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.781 |
| walker |  | 6061 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 50, sub: 0, line: 1076 } |  |  | 0.781 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.779 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.771 |
| walker |  | 6269 | 208 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 42, sub: 3, line: 903 } |  |  | 0.784 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.778 |
| walker |  | 6319 | 50 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 66, sub: 0, line: 1479 } |  |  | 0.778 |
| walker |  | 6398 | 79 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 64, sub: 0, line: 1356 } |  |  | 0.778 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.768 |
| walker |  | 6482 | 84 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 63, sub: 0, line: 1346 } |  |  | 0.768 |
| walker |  | 6575 | 93 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 65, sub: 0, line: 1366 } |  |  | 0.774 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.766 |
| walker |  | 6590 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 67, sub: 0, line: 1511 } |  |  | 0.766 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.750 |
| walker |  | 6859 | 269 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 99, sub: 0, line: 2054 } |  |  | 0.753 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.756 |
| walker |  | 6874 | 15 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 107, sub: 0, line: 2280 } |  |  | 0.756 |
| walker |  | 6893 | 19 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 103, sub: 0, line: 2249 } |  |  | 0.756 |
| walker |  | 6914 | 21 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 106, sub: 0, line: 2275 } |  |  | 0.756 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.754 |
| walker |  | 7118 | 204 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 99, sub: 1, line: 2054 } |  |  | 0.761 |
| walker |  | 7134 | 16 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 108, sub: 0, line: 2285 } |  |  | 0.761 |
| walker |  | 7154 | 20 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 110, sub: 0, line: 2321 } |  |  | 0.761 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.743 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.739 |
| walker |  | 7314 | 160 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 99, sub: 2, line: 2054 } |  |  | 0.750 |
| walker |  | 7339 | 25 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 116, sub: 0, line: 2570 } |  |  | 0.750 |
| walker |  | 7355 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 24, sub: 0, line: 683 } |  |  | 0.750 |
| walker |  | 7371 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 52, sub: 0, line: 1111 } |  |  | 0.750 |
| walker |  | 7387 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 95, sub: 0, line: 2014 } |  |  | 0.750 |
| walker |  | 7405 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 56, sub: 0, line: 1166 } |  |  | 0.750 |
| walker |  | 7423 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 57, sub: 0, line: 1189 } |  |  | 0.750 |
| walker |  | 7441 | 18 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 58, sub: 0, line: 1201 } |  |  | 0.750 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.742 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.731 |
| walker |  | 7776 | 335 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 72, sub: 0, line: 1531 } |  |  | 0.731 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.718 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.710 |
| walker |  | 8232 | 456 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 100, sub: 0, line: 2147 } |  |  | 0.729 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.720 |
| walker |  | 8437 | 205 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 72, sub: 1, line: 1531 } |  |  | 0.723 |
| walker |  | 8455 | 18 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 77, sub: 0, line: 1663 } |  |  | 0.723 |
| walker |  | 8473 | 18 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 78, sub: 0, line: 1668 } |  |  | 0.723 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.712 |
| walker |  | 8626 | 153 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 73, sub: 0, line: 1584 } |  |  | 0.725 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.714 |
| walker |  | 8830 | 204 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 72, sub: 2, line: 1531 } |  |  | 0.720 |
| walker |  | 8848 | 18 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 80, sub: 0, line: 1712 } |  |  | 0.720 |
| walker |  | 8866 | 18 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 81, sub: 0, line: 1717 } |  |  | 0.720 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.712 |
| walker |  | 9046 | 180 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 72, sub: 3, line: 1531 } |  |  | 0.724 |
| walker |  | 9061 | 15 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 90, sub: 0, line: 1935 } |  |  | 0.724 |
| walker |  | 9082 | 21 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 84, sub: 0, line: 1812 } |  |  | 0.724 |
| walker |  | 9114 | 32 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 33, sub: 0, line: 772 } |  |  | 0.724 |
| walker |  | 9146 | 32 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 34, sub: 0, line: 778 } |  |  | 0.724 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.715 |
| walker |  | 9182 | 36 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 87, sub: 0, line: 1825 } |  |  | 0.715 |
| walker |  | 9219 | 37 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 54, sub: 0, line: 1127 } |  |  | 0.715 |
| walker |  | 9258 | 39 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 61, sub: 0, line: 1283 } |  |  | 0.715 |
| walker |  | 9298 | 40 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 49, sub: 0, line: 1065 } |  |  | 0.715 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.707 |
| walker |  | 9343 | 45 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 53, sub: 0, line: 1118 } |  |  | 0.707 |
| walker |  | 9388 | 45 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 98, sub: 0, line: 2044 } |  |  | 0.707 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.698 |
| walker |  | 9725 | 337 | Code::CodeKey { rung: Names, file: src/click/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| walker |  | 9746 | 21 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 33, sub: 0, line: 527 } |  |  | 0.711 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.705 |
| walker |  | 9778 | 32 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 27, sub: 0, line: 411 } |  |  | 0.705 |
| walker |  | 9822 | 44 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 25, sub: 0, line: 341 } |  |  | 0.705 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.700 |
| walker |  | 9873 | 51 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 34, sub: 0, line: 582 } |  |  | 0.700 |
| walker |  | 9932 | 59 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 29, sub: 0, line: 502 } |  |  | 0.700 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.695 |
| walker |  | 9996 | 64 | Code::CodeKey { rung: Decl, file: src/click/utils.py, decl: 23, sub: 0, line: 226 } |  |  | 0.695 |
