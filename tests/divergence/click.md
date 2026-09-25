Score(3000)=0.753 I=0.890 C=0.637 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.605/0.768/0.829/0.753/0.684/0.610/0.549

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
| walker |  | 447 | 75 | Code::CodeKey { rung: ModuleDoc, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.810 |
| walker |  | 460 | 13 | Fs::DirListing { dir: .github } |  |  | 0.810 |
| ns | 476 |  | 156 | The canonical hello-world program | 1.5 |  | 0.694 |
| walker |  | 483 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.694 |
| walker |  | 521 | 38 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.695 |
| ns | 524 |  | 48 | The terminal session that program produces | 1.6 | 1.5 | 0.664 |
| ns | 707 |  | 183 | Test suite listing: tests/ and tests/typing/ | 1.7 |  | 0.525 |
| walker |  | 769 | 248 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| ns | 776 |  | 69 | Examples listing | 1.8 |  | 0.484 |
| walker |  | 961 | 192 | Fs::DirListing { dir: docs } |  |  | 0.502 |
| walker |  | 976 | 15 | Fs::DirListing { dir: docs/_static } |  |  | 0.504 |
| ns | 983 |  | 207 | Documentation listing: docs/ | 1.9 |  | 0.605 |
| walker |  | 1075 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.703 |
| walker |  | 1111 | 36 | Fs::DirListing { dir: examples } |  |  | 0.736 |
| ns | 1248 |  | 265 | Public API surface, part 1: object model and decorators | 2.1 |  | 0.745 |
| walker |  | 1344 | 233 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.759 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.768 |
| walker |  | 1583 | 239 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.773 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.780 |
| walker |  | 1803 | 220 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.787 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.795 |
| walker |  | 1930 | 127 | Fs::DirListing { dir: tests } |  |  | 0.852 |
| walker |  | 1959 | 29 | Markdown::Section { file: docs/setuptools.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.852 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.829 |
| walker |  | 2034 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 2090 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.878 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.855 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.847 |
| walker |  | 2180 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.847 |
| walker |  | 2233 | 53 | Markdown::Section { file: docs/license.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.847 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.820 |
| walker |  | 2457 | 224 | Code::CodeKey { rung: Names, file: src/click/_winconsole.py, decl: 0, sub: 0, line: 0 } |  |  | 0.820 |
| walker |  | 2478 | 21 | Code::CodeKey { rung: Decl, file: src/click/_winconsole.py, decl: 9, sub: 0, line: 49 } |  |  | 0.820 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.801 |
| walker |  | 2718 | 240 | Code::CodeKey { rung: Names, file: src/click/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| walker |  | 2728 | 10 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 37, sub: 0, line: 422 } |  |  | 0.801 |
| walker |  | 2743 | 15 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 14, sub: 0, line: 181 } |  |  | 0.801 |
| walker |  | 2760 | 17 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 2, sub: 0, line: 32 } |  |  | 0.801 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.771 |
| walker |  | 2781 | 21 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 25, sub: 0, line: 248 } |  |  | 0.771 |
| walker |  | 2814 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 12, sub: 0, line: 173 } |  |  | 0.771 |
| walker |  | 2830 | 16 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 13, sub: 0, line: 176 } |  |  | 0.771 |
| walker |  | 2869 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 19, sub: 0, line: 210 } |  |  | 0.771 |
| walker |  | 2903 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 20, sub: 0, line: 213 } |  |  | 0.771 |
| walker |  | 2942 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 22, sub: 0, line: 222 } |  |  | 0.771 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.753 |
| walker |  | 2975 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 23, sub: 0, line: 225 } |  |  | 0.753 |
| walker |  | 3032 | 57 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 15, sub: 0, line: 185 } |  |  | 0.753 |
| walker |  | 3067 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 18, sub: 0, line: 193 } |  |  | 0.753 |
| walker |  | 3100 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 45, sub: 0, line: 495 } |  |  | 0.753 |
| walker |  | 3135 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 46, sub: 0, line: 498 } |  |  | 0.753 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.722 |
| walker |  | 3275 | 140 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 38, sub: 0, line: 426 } |  |  | 0.722 |
| walker |  | 3308 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 43, sub: 0, line: 468 } |  |  | 0.722 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.708 |
| walker |  | 3508 | 200 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 26, sub: 0, line: 253 } |  |  | 0.708 |
| walker |  | 3539 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 36, sub: 0, line: 398 } |  |  | 0.708 |
| walker |  | 3573 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 27, sub: 0, line: 278 } |  |  | 0.708 |
| walker |  | 3608 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 33, sub: 0, line: 357 } |  |  | 0.708 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.693 |
| walker |  | 3640 | 32 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 29, sub: 0, line: 291 } |  |  | 0.693 |
| walker |  | 3691 | 51 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 32, sub: 0, line: 347 } |  |  | 0.693 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.669 |
| walker |  | 3911 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.722 |
| walker |  | 4139 | 228 | Code::CodeKey { rung: Names, file: src/click/_compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 4153 | 14 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.722 |
| walker |  | 4168 | 15 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 7, sub: 0, line: 48 } |  |  | 0.722 |
| walker |  | 4209 | 41 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 8, sub: 0, line: 56 } |  |  | 0.722 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.689 |
| walker |  | 4247 | 38 | Code::CodeKey { rung: Body, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.689 |
| walker |  | 4316 | 69 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 5, sub: 0, line: 19 } |  |  | 0.689 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.684 |
| walker |  | 4402 | 86 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 9, sub: 0, line: 57 } |  |  | 0.684 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.674 |
| walker |  | 4497 | 95 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 12, sub: 0, line: 82 } |  |  | 0.674 |
| walker |  | 4547 | 50 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 13, sub: 0, line: 92 } |  |  | 0.674 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.668 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.651 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.646 |
| walker |  | 4895 | 348 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 3, sub: 0, line: 37 } |  |  | 0.647 |
| walker |  | 4926 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 11, sub: 0, line: 155 } |  |  | 0.647 |
| walker |  | 4961 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 8, sub: 0, line: 110 } |  |  | 0.647 |
| walker |  | 5014 | 53 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 10, sub: 0, line: 146 } |  |  | 0.647 |
| walker |  | 5070 | 56 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 5, sub: 0, line: 90 } |  |  | 0.647 |
| walker |  | 5086 | 16 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 6, sub: 0, line: 100 } |  |  | 0.647 |
| walker |  | 5102 | 16 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 10, sub: 0, line: 146 } |  |  | 0.647 |
| walker |  | 5148 | 46 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 7, sub: 0, line: 103 } |  |  | 0.647 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.630 |
| walker |  | 5208 | 60 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 33, sub: 0, line: 357 } |  |  | 0.630 |
| walker |  | 5268 | 60 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 34, sub: 0, line: 381 } |  |  | 0.630 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.615 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.602 |
| walker |  | 5776 | 508 | Code::CodeKey { rung: Body, file: src/click/__init__.py, decl: 1, sub: 0, line: 77 } |  |  | 0.619 |
| walker |  | 5790 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.619 |
| walker |  | 5901 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 5917 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.619 |
| walker |  | 5950 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.619 |
| walker |  | 5960 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.619 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.607 |
| walker |  | 6017 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.607 |
| walker |  | 6086 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.607 |
| walker |  | 6094 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.607 |
| walker |  | 6102 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.607 |
| walker |  | 6110 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.607 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.615 |
| walker |  | 6185 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.615 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.610 |
| walker |  | 6224 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.610 |
| walker |  | 6269 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.610 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.605 |
| walker |  | 6282 | 13 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.605 |
| walker |  | 6359 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.605 |
| walker |  | 6374 | 15 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.605 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.597 |
| walker |  | 6470 | 96 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 35, sub: 0, line: 311 } |  |  | 0.597 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.591 |
| walker |  | 6587 | 117 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 26, sub: 0, line: 205 } |  |  | 0.591 |
| walker |  | 6710 | 123 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 36, sub: 0, line: 525 } |  |  | 0.601 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.589 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.596 |
| walker |  | 6894 | 184 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 1, sub: 0, line: 26 } |  |  | 0.596 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.594 |
| walker |  | 6962 | 68 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 18, sub: 0, line: 103 } |  |  | 0.594 |
| walker |  | 6970 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 22, sub: 0, line: 154 } |  |  | 0.594 |
| walker |  | 6978 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 23, sub: 0, line: 158 } |  |  | 0.594 |
| walker |  | 7053 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 19, sub: 0, line: 117 } |  |  | 0.594 |
| walker |  | 7098 | 45 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.594 |
| walker |  | 7155 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.594 |
| walker |  | 7212 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 33, sub: 0, line: 295 } |  |  | 0.594 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.586 |
| walker |  | 7273 | 61 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.586 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.583 |
| walker |  | 7357 | 84 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 4, sub: 0, line: 69 } |  |  | 0.583 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.583 |
| walker |  | 7447 | 90 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.583 |
| walker |  | 7658 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 7669 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.583 |
| walker |  | 7695 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.583 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.581 |
| walker |  | 7721 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.581 |
| walker |  | 7747 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.581 |
| walker |  | 7780 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.581 |
| walker |  | 7822 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.581 |
| walker |  | 7864 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.581 |
| walker |  | 7907 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.581 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.571 |
| walker |  | 7951 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.571 |
| walker |  | 7963 | 12 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.571 |
| walker |  | 8033 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.572 |
| walker |  | 8103 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.572 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.565 |
| walker |  | 8173 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.565 |
| walker |  | 8244 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.565 |
| walker |  | 8258 | 14 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 33, sub: 0, line: 330 } |  |  | 0.567 |
| walker |  | 8347 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.561 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.561 |
| walker |  | 8364 | 17 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.561 |
| walker |  | 8470 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.563 |
| walker |  | 8486 | 16 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.566 |
| walker |  | 8528 | 42 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.566 |
| walker |  | 8579 | 51 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.569 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.560 |
| walker |  | 8654 | 75 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 26, sub: 0, line: 295 } |  |  | 0.560 |
| walker |  | 8743 | 89 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.563 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.555 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.549 |
| walker |  | 9049 | 306 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 9086 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.562 |
| walker |  | 9124 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.562 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.555 |
| walker |  | 9204 | 80 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.556 |
| walker |  | 9265 | 61 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.556 |
| walker |  | 9280 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.556 |
| walker |  | 9296 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 95, sub: 0, line: 2014 } |  |  | 0.556 |
| walker |  | 9324 | 28 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.556 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.549 |
| walker |  | 9357 | 33 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.549 |
| walker |  | 9493 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.558 |
| walker |  | 9537 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.558 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.551 |
| walker |  | 9694 | 157 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.553 |
| walker |  | 9703 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 137, sub: 0, line: 3430 } |  |  | 0.553 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.549 |
| walker |  | 9762 | 59 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 136, sub: 0, line: 3409 } |  |  | 0.550 |
| walker |  | 9800 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 139, sub: 0, line: 3450 } |  |  | 0.550 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.546 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.543 |
