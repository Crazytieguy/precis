Score(3000)=0.753 I=0.890 C=0.637 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.728/0.781/0.829/0.753/0.684/0.594/0.537

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
| walker |  | 713 | 192 | Fs::DirListing { dir: docs } |  |  | 0.544 |
| walker |  | 728 | 15 | Fs::DirListing { dir: docs/_static } |  |  | 0.547 |
| ns | 776 |  | 69 | Examples listing | 1.8 |  | 0.498 |
| walker |  | 827 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.610 |
| walker |  | 863 | 36 | Fs::DirListing { dir: examples } |  |  | 0.658 |
| ns | 983 |  | 207 | Documentation listing: docs/ | 1.9 |  | 0.728 |
| walker |  | 1111 | 248 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.736 |
| walker |  | 1127 | 16 | Code::CodeKey { rung: Names, file: src/click/_textwrap.py, decl: 0, sub: 0, line: 0 } |  |  | 0.736 |
| walker |  | 1180 | 53 | Code::CodeKey { rung: Decl, file: src/click/_textwrap.py, decl: 1, sub: 0, line: 8 } |  |  | 0.736 |
| walker |  | 1189 | 9 | Code::CodeKey { rung: Decl, file: src/click/_textwrap.py, decl: 3, sub: 0, line: 27 } |  |  | 0.736 |
| ns | 1248 |  | 265 | Public API surface, part 1: object model and decorators | 2.1 |  | 0.745 |
| walker |  | 1316 | 127 | Fs::DirListing { dir: tests } |  |  | 0.814 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.781 |
| walker |  | 1549 | 233 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.832 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.801 |
| walker |  | 1788 | 239 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.841 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.800 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.778 |
| walker |  | 2008 | 220 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.829 |
| walker |  | 2037 | 29 | Markdown::Section { file: docs/setuptools.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 2112 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.829 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.807 |
| walker |  | 2168 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.855 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.847 |
| walker |  | 2258 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.847 |
| walker |  | 2311 | 53 | Markdown::Section { file: docs/license.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.847 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.820 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.801 |
| walker |  | 2535 | 224 | Code::CodeKey { rung: Names, file: src/click/_winconsole.py, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| walker |  | 2556 | 21 | Code::CodeKey { rung: Decl, file: src/click/_winconsole.py, decl: 9, sub: 0, line: 49 } |  |  | 0.801 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.770 |
| walker |  | 2796 | 240 | Code::CodeKey { rung: Names, file: src/click/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| walker |  | 2806 | 10 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 37, sub: 0, line: 422 } |  |  | 0.771 |
| walker |  | 2821 | 15 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 14, sub: 0, line: 181 } |  |  | 0.771 |
| walker |  | 2838 | 17 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 2, sub: 0, line: 32 } |  |  | 0.771 |
| walker |  | 2859 | 21 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 25, sub: 0, line: 248 } |  |  | 0.771 |
| walker |  | 2892 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 12, sub: 0, line: 173 } |  |  | 0.771 |
| walker |  | 2908 | 16 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 13, sub: 0, line: 176 } |  |  | 0.771 |
| walker |  | 2947 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 19, sub: 0, line: 210 } |  |  | 0.771 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.753 |
| walker |  | 2986 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 22, sub: 0, line: 222 } |  |  | 0.753 |
| walker |  | 3043 | 57 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 15, sub: 0, line: 185 } |  |  | 0.753 |
| walker |  | 3076 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 23, sub: 0, line: 225 } |  |  | 0.753 |
| walker |  | 3110 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 20, sub: 0, line: 213 } |  |  | 0.753 |
| walker |  | 3145 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 18, sub: 0, line: 193 } |  |  | 0.753 |
| walker |  | 3178 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 45, sub: 0, line: 495 } |  |  | 0.753 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.722 |
| walker |  | 3318 | 140 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 38, sub: 0, line: 426 } |  |  | 0.722 |
| walker |  | 3351 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 43, sub: 0, line: 468 } |  |  | 0.722 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.708 |
| walker |  | 3551 | 200 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 26, sub: 0, line: 253 } |  |  | 0.708 |
| walker |  | 3582 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 36, sub: 0, line: 398 } |  |  | 0.708 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.693 |
| walker |  | 3616 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 27, sub: 0, line: 278 } |  |  | 0.693 |
| walker |  | 3651 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 33, sub: 0, line: 357 } |  |  | 0.693 |
| walker |  | 3683 | 32 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 29, sub: 0, line: 291 } |  |  | 0.693 |
| walker |  | 3718 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 46, sub: 0, line: 498 } |  |  | 0.693 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.669 |
| walker |  | 3938 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.722 |
| walker |  | 4043 | 105 | Code::CodeKey { rung: Names, file: src/click/formatting.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 4080 | 37 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.722 |
| walker |  | 4145 | 65 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 4, sub: 0, line: 31 } |  |  | 0.722 |
| walker |  | 4172 | 27 | Code::CodeKey { rung: Body, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.722 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.689 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.684 |
| walker |  | 4382 | 210 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 5, sub: 0, line: 104 } |  |  | 0.685 |
| walker |  | 4391 | 9 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 15, sub: 0, line: 254 } |  |  | 0.685 |
| walker |  | 4400 | 9 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 16, sub: 0, line: 269 } |  |  | 0.685 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.675 |
| walker |  | 4456 | 56 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 6, sub: 0, line: 116 } |  |  | 0.675 |
| walker |  | 4514 | 58 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 14, sub: 0, line: 210 } |  |  | 0.675 |
| walker |  | 4525 | 11 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 8, sub: 0, line: 139 } |  |  | 0.675 |
| walker |  | 4536 | 11 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 9, sub: 0, line: 143 } |  |  | 0.675 |
| walker |  | 4547 | 11 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 17, sub: 0, line: 278 } |  |  | 0.675 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.669 |
| walker |  | 4560 | 13 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 11, sub: 0, line: 185 } |  |  | 0.669 |
| walker |  | 4573 | 13 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 12, sub: 0, line: 189 } |  |  | 0.669 |
| walker |  | 4587 | 14 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 16, sub: 0, line: 269 } |  |  | 0.669 |
| walker |  | 4602 | 15 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 7, sub: 0, line: 135 } |  |  | 0.669 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.652 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.647 |
| walker |  | 4950 | 348 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 3, sub: 0, line: 37 } |  |  | 0.648 |
| walker |  | 4981 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 11, sub: 0, line: 155 } |  |  | 0.648 |
| walker |  | 5016 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 8, sub: 0, line: 110 } |  |  | 0.648 |
| walker |  | 5069 | 53 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 10, sub: 0, line: 146 } |  |  | 0.648 |
| walker |  | 5125 | 56 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 5, sub: 0, line: 90 } |  |  | 0.648 |
| walker |  | 5141 | 16 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 6, sub: 0, line: 100 } |  |  | 0.648 |
| walker |  | 5157 | 16 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 10, sub: 0, line: 146 } |  |  | 0.648 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.631 |
| walker |  | 5385 | 228 | Code::CodeKey { rung: Names, file: src/click/_compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 5399 | 14 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.631 |
| walker |  | 5414 | 15 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 7, sub: 0, line: 48 } |  |  | 0.631 |
| walker |  | 5455 | 41 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 8, sub: 0, line: 56 } |  |  | 0.631 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.616 |
| walker |  | 5493 | 38 | Code::CodeKey { rung: Body, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.616 |
| walker |  | 5562 | 69 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 5, sub: 0, line: 19 } |  |  | 0.616 |
| walker |  | 5657 | 95 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 12, sub: 0, line: 82 } |  |  | 0.616 |
| walker |  | 5707 | 50 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 13, sub: 0, line: 92 } |  |  | 0.616 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.603 |
| walker |  | 5818 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 5834 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.603 |
| walker |  | 5867 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.603 |
| walker |  | 5877 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.603 |
| walker |  | 5934 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.603 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.591 |
| walker |  | 6003 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.591 |
| walker |  | 6011 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.591 |
| walker |  | 6019 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.591 |
| walker |  | 6027 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.591 |
| walker |  | 6102 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.591 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.600 |
| walker |  | 6141 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.600 |
| walker |  | 6186 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.600 |
| walker |  | 6199 | 13 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.600 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.594 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.589 |
| walker |  | 6276 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.589 |
| walker |  | 6291 | 15 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.589 |
| walker |  | 6387 | 96 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 35, sub: 0, line: 311 } |  |  | 0.589 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.582 |
| walker |  | 6571 | 184 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 1, sub: 0, line: 26 } |  |  | 0.582 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.576 |
| walker |  | 6688 | 117 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 26, sub: 0, line: 205 } |  |  | 0.576 |
| walker |  | 6811 | 123 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 36, sub: 0, line: 525 } |  |  | 0.586 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.574 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.575 |
| walker |  | 6879 | 68 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 18, sub: 0, line: 103 } |  |  | 0.575 |
| walker |  | 6887 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 22, sub: 0, line: 154 } |  |  | 0.575 |
| walker |  | 6895 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 23, sub: 0, line: 158 } |  |  | 0.575 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.573 |
| walker |  | 6940 | 45 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.573 |
| walker |  | 7001 | 61 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.573 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.566 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.563 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.563 |
| walker |  | 7509 | 508 | Code::CodeKey { rung: Body, file: src/click/__init__.py, decl: 1, sub: 0, line: 77 } |  |  | 0.578 |
| walker |  | 7566 | 57 | Code::CodeKey { rung: Decl, file: src/click/_textwrap.py, decl: 2, sub: 0, line: 9 } |  |  | 0.578 |
| walker |  | 7681 | 115 | Code::CodeKey { rung: Names, file: src/click/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.569 |
| walker |  | 7724 | 43 | Code::CodeKey { rung: Decl, file: src/click/_utils.py, decl: 1, sub: 0, line: 7 } |  |  | 0.569 |
| walker |  | 7740 | 16 | Code::CodeKey { rung: Body, file: src/click/_utils.py, decl: 2, sub: 0, line: 18 } |  |  | 0.569 |
| walker |  | 7805 | 65 | Code::CodeKey { rung: Doc, file: src/click/_utils.py, decl: 1, sub: 0, line: 7 } |  |  | 0.569 |
| walker |  | 7841 | 36 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 13, sub: 0, line: 194 } |  |  | 0.569 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.559 |
| walker |  | 7980 | 139 | Code::CodeKey { rung: Names, file: src/click/parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 8000 | 20 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 12, sub: 0, line: 216 } |  |  | 0.559 |
| walker |  | 8039 | 39 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 9, sub: 0, line: 185 } |  |  | 0.559 |
| walker |  | 8088 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 2, sub: 0, line: 51 } |  |  | 0.559 |
| walker |  | 8137 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 5, sub: 0, line: 127 } |  |  | 0.559 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.553 |
| walker |  | 8145 | 8 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 7, sub: 0, line: 165 } |  |  | 0.553 |
| walker |  | 8195 | 50 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 11, sub: 0, line: 191 } |  |  | 0.553 |
| walker |  | 8338 | 143 | Code::CodeKey { rung: Names, file: src/click/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 8345 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 2, sub: 0, line: 12 } |  |  | 0.553 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.546 |
| walker |  | 8352 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 3, sub: 0, line: 16 } |  |  | 0.546 |
| walker |  | 8362 | 10 | Code::CodeKey { rung: Body, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.546 |
| walker |  | 8375 | 13 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.546 |
| walker |  | 8391 | 16 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 5, sub: 0, line: 44 } |  |  | 0.546 |
| walker |  | 8408 | 17 | Code::CodeKey { rung: Body, file: src/click/globals.py, decl: 5, sub: 0, line: 44 } |  |  | 0.546 |
| walker |  | 8463 | 55 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 7, sub: 0, line: 54 } |  |  | 0.546 |
| walker |  | 8477 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.552 |
| walker |  | 8560 | 83 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 18, sub: 0, line: 283 } |  |  | 0.552 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.544 |
| walker |  | 8635 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 19, sub: 0, line: 117 } |  |  | 0.544 |
| walker |  | 8681 | 46 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 7, sub: 0, line: 103 } |  |  | 0.544 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.536 |
| walker |  | 8892 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 8903 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.542 |
| walker |  | 8929 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.542 |
| walker |  | 8955 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.542 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.537 |
| walker |  | 8981 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.537 |
| walker |  | 9023 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.537 |
| walker |  | 9065 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.537 |
| walker |  | 9108 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.537 |
| walker |  | 9152 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.537 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.530 |
| walker |  | 9185 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.530 |
| walker |  | 9197 | 12 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.530 |
| walker |  | 9267 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.531 |
| walker |  | 9281 | 14 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 33, sub: 0, line: 330 } |  |  | 0.533 |
| walker |  | 9298 | 17 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.533 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.543 |
| walker |  | 9404 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.545 |
| walker |  | 9420 | 16 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.547 |
| walker |  | 9490 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.547 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.543 |
| walker |  | 9560 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.543 |
| walker |  | 9631 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.543 |
| walker |  | 9720 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.543 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.542 |
| walker |  | 9762 | 42 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.542 |
| walker |  | 9813 | 51 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.545 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.541 |
| walker |  | 9888 | 75 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 26, sub: 0, line: 295 } |  |  | 0.541 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.538 |
| walker |  | 9977 | 89 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.540 |
