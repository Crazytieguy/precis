Score(3000)=0.811 I=0.940 C=0.699 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.804/0.831/0.910/0.811/0.684/0.644/0.633

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 53 | 3 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 81 |  | 81 | README identity: what Click is | 1.1 |  | 0.000 |
| walker |  | 135 | 82 | Toml::Identity { file: pyproject.toml } |  |  | 0.000 |
| ns | 182 |  | 101 | Click in three points | 1.2 |  | 0.000 |
| walker |  | 216 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.584 |
| walker |  | 246 | 30 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.584 |
| walker |  | 257 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.584 |
| ns | 270 |  | 88 | Complete module roster of the package: src/click/ | 1.3 |  | 0.355 |
| ns | 320 |  | 50 | Repository root listing | 1.4 |  | 0.527 |
| walker |  | 345 | 88 | Fs::DirListing { dir: src/click } |  |  | 0.810 |
| walker |  | 372 | 27 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.810 |
| walker |  | 385 | 13 | Fs::DirListing { dir: .github } |  |  | 0.810 |
| walker |  | 408 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.811 |
| ns | 476 |  | 156 | The canonical hello-world program | 1.5 |  | 0.694 |
| walker |  | 483 | 75 | Code::CodeKey { rung: ModuleDoc, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| ns | 524 |  | 48 | The terminal session that program produces | 1.6 | 1.5 | 0.663 |
| walker |  | 582 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.819 |
| ns | 707 |  | 183 | Test suite listing: tests/ and tests/typing/ | 1.7 |  | 0.648 |
| walker |  | 774 | 192 | Fs::DirListing { dir: docs } |  |  | 0.667 |
| ns | 776 |  | 69 | Examples listing | 1.8 |  | 0.607 |
| walker |  | 789 | 15 | Fs::DirListing { dir: docs/_static } |  |  | 0.609 |
| walker |  | 825 | 36 | Fs::DirListing { dir: examples } |  |  | 0.658 |
| walker |  | 952 | 127 | Fs::DirListing { dir: tests } |  |  | 0.766 |
| ns | 983 |  | 207 | Documentation listing: docs/ | 1.9 |  | 0.804 |
| walker |  | 1200 | 248 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| ns | 1248 |  | 265 | Public API surface, part 1: object model and decorators | 2.1 |  | 0.814 |
| walker |  | 1433 | 233 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.827 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.831 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.801 |
| walker |  | 1672 | 239 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.840 |
| walker |  | 1892 | 220 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.847 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.851 |
| walker |  | 1948 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.902 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.877 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.854 |
| walker |  | 2168 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.921 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.912 |
| walker |  | 2279 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.912 |
| walker |  | 2289 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.912 |
| walker |  | 2305 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.912 |
| walker |  | 2338 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.912 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.884 |
| walker |  | 2395 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.884 |
| walker |  | 2463 | 68 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 18, sub: 0, line: 103 } |  |  | 0.884 |
| walker |  | 2471 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 22, sub: 0, line: 154 } |  |  | 0.884 |
| walker |  | 2479 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 23, sub: 0, line: 158 } |  |  | 0.884 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.863 |
| walker |  | 2548 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.863 |
| walker |  | 2556 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.863 |
| walker |  | 2564 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.863 |
| walker |  | 2572 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.863 |
| walker |  | 2647 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 19, sub: 0, line: 117 } |  |  | 0.863 |
| walker |  | 2722 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.863 |
| walker |  | 2761 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.863 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.830 |
| walker |  | 2806 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.830 |
| walker |  | 2883 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.830 |
| walker |  | 2896 | 13 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.830 |
| walker |  | 2911 | 15 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.830 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.811 |
| walker |  | 3007 | 96 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 35, sub: 0, line: 311 } |  |  | 0.811 |
| walker |  | 3124 | 117 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 26, sub: 0, line: 205 } |  |  | 0.811 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.777 |
| walker |  | 3247 | 123 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 36, sub: 0, line: 525 } |  |  | 0.778 |
| walker |  | 3431 | 184 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 1, sub: 0, line: 26 } |  |  | 0.778 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.764 |
| walker |  | 3476 | 45 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.764 |
| walker |  | 3533 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.764 |
| walker |  | 3590 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 33, sub: 0, line: 295 } |  |  | 0.764 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.747 |
| walker |  | 3651 | 61 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.747 |
| walker |  | 3719 | 68 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 20, sub: 0, line: 131 } |  |  | 0.747 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.721 |
| walker |  | 3930 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 3941 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.722 |
| walker |  | 3967 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.722 |
| walker |  | 3993 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.722 |
| walker |  | 4019 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.722 |
| walker |  | 4052 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.722 |
| walker |  | 4094 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.722 |
| walker |  | 4136 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.722 |
| walker |  | 4179 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.722 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.690 |
| walker |  | 4223 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.690 |
| walker |  | 4293 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.690 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.684 |
| walker |  | 4363 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.684 |
| walker |  | 4433 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.684 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.674 |
| walker |  | 4504 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.674 |
| walker |  | 4516 | 12 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.674 |
| walker |  | 4530 | 14 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 33, sub: 0, line: 330 } |  |  | 0.675 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.668 |
| walker |  | 4619 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.668 |
| walker |  | 4636 | 17 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.668 |
| walker |  | 4742 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.668 |
| walker |  | 4758 | 16 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.669 |
| walker |  | 4800 | 42 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.669 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.652 |
| walker |  | 4851 | 51 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.652 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.647 |
| walker |  | 5157 | 306 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 5185 | 28 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.667 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.650 |
| walker |  | 5218 | 33 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.650 |
| walker |  | 5255 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.650 |
| walker |  | 5293 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.650 |
| walker |  | 5337 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.650 |
| walker |  | 5417 | 80 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.651 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.636 |
| walker |  | 5478 | 61 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.636 |
| walker |  | 5493 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.636 |
| walker |  | 5509 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 95, sub: 0, line: 2014 } |  |  | 0.636 |
| walker |  | 5645 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.650 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.635 |
| walker |  | 5802 | 157 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.635 |
| walker |  | 5811 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 137, sub: 0, line: 3430 } |  |  | 0.635 |
| walker |  | 5849 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 139, sub: 0, line: 3450 } |  |  | 0.635 |
| walker |  | 5908 | 59 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 136, sub: 0, line: 3409 } |  |  | 0.637 |
| walker |  | 5953 | 45 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 98, sub: 0, line: 2044 } |  |  | 0.637 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.627 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.628 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.622 |
| walker |  | 6239 | 286 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 121, sub: 0, line: 2671 } |  |  | 0.644 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.639 |
| walker |  | 6275 | 36 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 133, sub: 0, line: 3319 } |  |  | 0.639 |
| walker |  | 6313 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 126, sub: 0, line: 2974 } |  |  | 0.639 |
| walker |  | 6353 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 124, sub: 0, line: 2935 } |  |  | 0.639 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.640 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.633 |
| walker |  | 6633 | 280 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 122, sub: 0, line: 2745 } |  |  | 0.660 |
| walker |  | 6681 | 48 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 71, sub: 0, line: 1524 } |  |  | 0.660 |
| walker |  | 6729 | 48 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 92, sub: 0, line: 1981 } |  |  | 0.660 |
| walker |  | 6784 | 55 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 123, sub: 0, line: 2916 } |  |  | 0.660 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.646 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.646 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.644 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.629 |
| walker |  | 7228 | 444 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 99, sub: 0, line: 2054 } |  |  | 0.652 |
| walker |  | 7237 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 104, sub: 0, line: 2254 } |  |  | 0.652 |
| walker |  | 7247 | 10 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 109, sub: 0, line: 2318 } |  |  | 0.652 |
| walker |  | 7284 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 110, sub: 0, line: 2321 } |  |  | 0.652 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.648 |
| walker |  | 7324 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 108, sub: 0, line: 2285 } |  |  | 0.648 |
| walker |  | 7366 | 42 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 116, sub: 0, line: 2570 } |  |  | 0.648 |
| walker |  | 7414 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 103, sub: 0, line: 2249 } |  |  | 0.648 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.642 |
| walker |  | 7462 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 106, sub: 0, line: 2275 } |  |  | 0.642 |
| walker |  | 7509 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 107, sub: 0, line: 2280 } |  |  | 0.642 |
| walker |  | 7556 | 47 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 104, sub: 0, line: 2254 } |  |  | 0.642 |
| walker |  | 7603 | 47 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 111, sub: 0, line: 2369 } |  |  | 0.642 |
| walker |  | 7676 | 73 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.642 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.648 |
| walker |  | 7752 | 76 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 130, sub: 0, line: 3203 } |  |  | 0.648 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.637 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.629 |
| walker |  | 8220 | 468 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 100, sub: 0, line: 2147 } |  |  | 0.649 |
| walker |  | 8300 | 80 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 119, sub: 0, line: 2637 } |  |  | 0.649 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.642 |
| walker |  | 8382 | 82 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.647 |
| walker |  | 8491 | 109 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 115, sub: 0, line: 2554 } |  |  | 0.647 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.637 |
| walker |  | 8738 | 247 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 0, line: 185 } |  |  | 0.640 |
| walker |  | 8746 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 11, sub: 0, line: 459 } |  |  | 0.640 |
| walker |  | 8754 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 16, sub: 0, line: 549 } |  |  | 0.640 |
| walker |  | 8763 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 15, sub: 0, line: 511 } |  |  | 0.640 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.630 |
| walker |  | 8823 | 60 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 14, sub: 0, line: 497 } |  |  | 0.630 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.624 |
| walker |  | 9083 | 260 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 10, sub: 0, line: 289 } |  |  | 0.643 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.636 |
| walker |  | 9190 | 107 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 17, sub: 0, line: 577 } |  |  | 0.636 |
| walker |  | 9300 | 110 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 12, sub: 0, line: 471 } |  |  | 0.636 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.628 |
| walker |  | 9534 | 234 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 1, line: 185 } |  |  | 0.641 |
| walker |  | 9542 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 22, sub: 0, line: 657 } |  |  | 0.641 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.633 |
| walker |  | 9587 | 45 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 27, sub: 0, line: 718 } |  |  | 0.633 |
| walker |  | 9636 | 49 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 28, sub: 0, line: 723 } |  |  | 0.633 |
| walker |  | 9696 | 60 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 21, sub: 0, line: 639 } |  |  | 0.633 |
| walker |  | 9707 | 11 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 31, sub: 0, line: 758 } |  |  | 0.633 |
| walker |  | 9720 | 13 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 23, sub: 0, line: 676 } |  |  | 0.633 |
| walker |  | 9736 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 24, sub: 0, line: 683 } |  |  | 0.633 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.627 |
| walker |  | 9783 | 47 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 25, sub: 0, line: 695 } |  |  | 0.627 |
| walker |  | 9833 | 50 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 20, sub: 0, line: 632 } |  |  | 0.627 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.623 |
| walker |  | 9885 | 52 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 30, sub: 0, line: 750 } |  |  | 0.623 |
| walker |  | 9947 | 62 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 22, sub: 0, line: 657 } |  |  | 0.623 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.619 |
| walker |  | 9982 | 35 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 32, sub: 0, line: 762 } |  |  | 0.619 |
