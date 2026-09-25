Score(3000)=0.812 I=0.943 C=0.699 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.804/0.781/0.829/0.812/0.685/0.625/0.633

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 53 | 3 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 81 |  | 81 | README identity: what Click is | 1.1 |  | 0.000 |
| walker |  | 84 | 31 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.000 |
| walker |  | 162 | 78 | Toml::Identity { file: pyproject.toml } |  |  | 0.000 |
| ns | 182 |  | 101 | Click in three points | 1.2 |  | 0.000 |
| walker |  | 243 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.584 |
| ns | 270 |  | 88 | Complete module roster of the package: src/click/ | 1.3 |  | 0.355 |
| walker |  | 273 | 30 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.355 |
| walker |  | 284 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.355 |
| ns | 320 |  | 50 | Repository root listing | 1.4 |  | 0.527 |
| walker |  | 372 | 88 | Fs::DirListing { dir: src/click } |  |  | 0.810 |
| walker |  | 385 | 13 | Fs::DirListing { dir: .github } |  |  | 0.810 |
| walker |  | 408 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.811 |
| ns | 476 |  | 156 | The canonical hello-world program | 1.5 |  | 0.694 |
| walker |  | 483 | 75 | Code::CodeKey { rung: ModuleDoc, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| walker |  | 521 | 38 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.695 |
| ns | 524 |  | 48 | The terminal session that program produces | 1.6 | 1.5 | 0.664 |
| ns | 707 |  | 183 | Test suite listing: tests/ and tests/typing/ | 1.7 |  | 0.525 |
| walker |  | 713 | 192 | Fs::DirListing { dir: docs } |  |  | 0.544 |
| walker |  | 728 | 15 | Fs::DirListing { dir: docs/_static } |  |  | 0.547 |
| ns | 776 |  | 69 | Examples listing | 1.8 |  | 0.498 |
| walker |  | 827 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.610 |
| walker |  | 863 | 36 | Fs::DirListing { dir: examples } |  |  | 0.658 |
| ns | 983 |  | 207 | Documentation listing: docs/ | 1.9 |  | 0.728 |
| walker |  | 990 | 127 | Fs::DirListing { dir: tests } |  |  | 0.804 |
| walker |  | 1019 | 29 | Markdown::Section { file: docs/setuptools.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.804 |
| ns | 1248 |  | 265 | Public API surface, part 1: object model and decorators | 2.1 |  | 0.753 |
| walker |  | 1267 | 248 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.814 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.781 |
| walker |  | 1500 | 233 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.832 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.801 |
| walker |  | 1739 | 239 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.841 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.800 |
| walker |  | 1959 | 220 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.852 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.829 |
| walker |  | 2034 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 2090 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.878 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.855 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.847 |
| walker |  | 2180 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.847 |
| walker |  | 2233 | 53 | Markdown::Section { file: docs/license.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.847 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.820 |
| walker |  | 2453 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.884 |
| walker |  | 2467 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.885 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.864 |
| walker |  | 2578 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 2588 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.864 |
| walker |  | 2604 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.864 |
| walker |  | 2637 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.864 |
| walker |  | 2694 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.864 |
| walker |  | 2762 | 68 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 18, sub: 0, line: 103 } |  |  | 0.864 |
| walker |  | 2770 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 22, sub: 0, line: 154 } |  |  | 0.864 |
| walker |  | 2778 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 23, sub: 0, line: 158 } |  |  | 0.831 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.831 |
| walker |  | 2847 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.831 |
| walker |  | 2855 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.831 |
| walker |  | 2863 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.831 |
| walker |  | 2871 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.831 |
| walker |  | 2946 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 19, sub: 0, line: 117 } |  |  | 0.831 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.812 |
| walker |  | 3021 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.812 |
| walker |  | 3060 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.812 |
| walker |  | 3105 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.812 |
| walker |  | 3118 | 13 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.812 |
| walker |  | 3195 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.812 |
| walker |  | 3210 | 15 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.812 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.779 |
| walker |  | 3306 | 96 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 35, sub: 0, line: 311 } |  |  | 0.779 |
| walker |  | 3423 | 117 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 26, sub: 0, line: 205 } |  |  | 0.779 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.764 |
| walker |  | 3546 | 123 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 36, sub: 0, line: 525 } |  |  | 0.765 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.748 |
| walker |  | 3730 | 184 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 1, sub: 0, line: 26 } |  |  | 0.748 |
| walker |  | 3775 | 45 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.748 |
| walker |  | 3832 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.748 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.722 |
| walker |  | 3889 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 33, sub: 0, line: 295 } |  |  | 0.722 |
| walker |  | 3950 | 61 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.722 |
| walker |  | 4018 | 68 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 20, sub: 0, line: 131 } |  |  | 0.722 |
| walker |  | 4108 | 90 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.722 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.690 |
| walker |  | 4319 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.685 |
| walker |  | 4330 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.685 |
| walker |  | 4356 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.685 |
| walker |  | 4382 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.685 |
| walker |  | 4408 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.685 |
| walker |  | 4441 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.685 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.675 |
| walker |  | 4483 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.675 |
| walker |  | 4525 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.675 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.669 |
| walker |  | 4568 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.669 |
| walker |  | 4612 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.669 |
| walker |  | 4624 | 12 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.669 |
| walker |  | 4694 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.669 |
| walker |  | 4764 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.669 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.653 |
| walker |  | 4834 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.653 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.648 |
| walker |  | 4905 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.648 |
| walker |  | 4919 | 14 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 33, sub: 0, line: 330 } |  |  | 0.648 |
| walker |  | 5008 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.648 |
| walker |  | 5025 | 17 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.648 |
| walker |  | 5131 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.648 |
| walker |  | 5147 | 16 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.648 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.631 |
| walker |  | 5189 | 42 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.631 |
| walker |  | 5240 | 51 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.632 |
| walker |  | 5315 | 75 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 26, sub: 0, line: 295 } |  |  | 0.632 |
| walker |  | 5404 | 89 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.632 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.617 |
| walker |  | 5710 | 306 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.622 |
| walker |  | 5738 | 28 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.622 |
| walker |  | 5771 | 33 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.622 |
| walker |  | 5808 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.622 |
| walker |  | 5846 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.622 |
| walker |  | 5890 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.622 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.609 |
| walker |  | 5970 | 80 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.610 |
| walker |  | 6031 | 61 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.610 |
| walker |  | 6046 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.610 |
| walker |  | 6062 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 95, sub: 0, line: 2014 } |  |  | 0.610 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.619 |
| walker |  | 6198 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.631 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.625 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.620 |
| walker |  | 6355 | 157 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.623 |
| walker |  | 6364 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 137, sub: 0, line: 3430 } |  |  | 0.623 |
| walker |  | 6402 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 139, sub: 0, line: 3450 } |  |  | 0.623 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.624 |
| walker |  | 6461 | 59 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 136, sub: 0, line: 3409 } |  |  | 0.626 |
| walker |  | 6506 | 45 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 98, sub: 0, line: 2044 } |  |  | 0.626 |
| walker |  | 6554 | 48 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 71, sub: 0, line: 1524 } |  |  | 0.626 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.620 |
| walker |  | 6602 | 48 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 92, sub: 0, line: 1981 } |  |  | 0.620 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.607 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.613 |
| walker |  | 6888 | 286 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 121, sub: 0, line: 2671 } |  |  | 0.633 |
| walker |  | 6924 | 36 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 133, sub: 0, line: 3319 } |  |  | 0.633 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.631 |
| walker |  | 6962 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 126, sub: 0, line: 2974 } |  |  | 0.631 |
| walker |  | 7002 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 124, sub: 0, line: 2935 } |  |  | 0.631 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.616 |
| walker |  | 7282 | 280 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 122, sub: 0, line: 2745 } |  |  | 0.641 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.637 |
| walker |  | 7337 | 55 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 123, sub: 0, line: 2916 } |  |  | 0.637 |
| walker |  | 7410 | 73 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.637 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.631 |
| walker |  | 7486 | 76 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 130, sub: 0, line: 3203 } |  |  | 0.631 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.640 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.629 |
| walker |  | 7930 | 444 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 99, sub: 0, line: 2054 } |  |  | 0.650 |
| walker |  | 7939 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 104, sub: 0, line: 2254 } |  |  | 0.650 |
| walker |  | 7949 | 10 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 109, sub: 0, line: 2318 } |  |  | 0.650 |
| walker |  | 7986 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 110, sub: 0, line: 2321 } |  |  | 0.650 |
| walker |  | 8026 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 108, sub: 0, line: 2285 } |  |  | 0.650 |
| walker |  | 8068 | 42 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 116, sub: 0, line: 2570 } |  |  | 0.650 |
| walker |  | 8116 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 103, sub: 0, line: 2249 } |  |  | 0.650 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.643 |
| walker |  | 8164 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 106, sub: 0, line: 2275 } |  |  | 0.643 |
| walker |  | 8211 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 107, sub: 0, line: 2280 } |  |  | 0.643 |
| walker |  | 8258 | 47 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 104, sub: 0, line: 2254 } |  |  | 0.643 |
| walker |  | 8305 | 47 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 111, sub: 0, line: 2369 } |  |  | 0.643 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.635 |
| walker |  | 8385 | 80 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 119, sub: 0, line: 2637 } |  |  | 0.635 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.626 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.616 |
| walker |  | 8853 | 468 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 100, sub: 0, line: 2147 } |  |  | 0.635 |
| walker |  | 8935 | 82 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.640 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.633 |
| walker |  | 9044 | 109 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 115, sub: 0, line: 2554 } |  |  | 0.633 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.626 |
| walker |  | 9291 | 247 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 0, line: 185 } |  |  | 0.628 |
| walker |  | 9299 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 11, sub: 0, line: 459 } |  |  | 0.628 |
| walker |  | 9307 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 16, sub: 0, line: 549 } |  |  | 0.628 |
| walker |  | 9316 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 15, sub: 0, line: 511 } |  |  | 0.628 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.621 |
| walker |  | 9376 | 60 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 14, sub: 0, line: 497 } |  |  | 0.621 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.613 |
| walker |  | 9636 | 260 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 10, sub: 0, line: 289 } |  |  | 0.632 |
| walker |  | 9743 | 107 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 17, sub: 0, line: 577 } |  |  | 0.632 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.627 |
| walker |  | 9853 | 110 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 12, sub: 0, line: 471 } |  |  | 0.627 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.623 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.618 |
| walker |  | 9969 | 116 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 19, sub: 0, line: 620 } |  |  | 0.618 |
