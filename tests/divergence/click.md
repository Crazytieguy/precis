Score(3000)=0.753 I=0.890 C=0.637 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.728/0.781/0.829/0.753/0.684/0.594/0.524

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
| walker |  | 2368 | 57 | Code::CodeKey { rung: Decl, file: src/click/_textwrap.py, decl: 2, sub: 0, line: 9 } |  |  | 0.820 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.820 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.801 |
| walker |  | 2592 | 224 | Code::CodeKey { rung: Names, file: src/click/_winconsole.py, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| walker |  | 2613 | 21 | Code::CodeKey { rung: Decl, file: src/click/_winconsole.py, decl: 9, sub: 0, line: 49 } |  |  | 0.801 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.770 |
| walker |  | 2853 | 240 | Code::CodeKey { rung: Names, file: src/click/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| walker |  | 2863 | 10 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 37, sub: 0, line: 422 } |  |  | 0.771 |
| walker |  | 2878 | 15 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 14, sub: 0, line: 181 } |  |  | 0.771 |
| walker |  | 2895 | 17 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 2, sub: 0, line: 32 } |  |  | 0.771 |
| walker |  | 2916 | 21 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 25, sub: 0, line: 248 } |  |  | 0.771 |
| walker |  | 2949 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 12, sub: 0, line: 173 } |  |  | 0.771 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.753 |
| walker |  | 2965 | 16 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 13, sub: 0, line: 176 } |  |  | 0.753 |
| walker |  | 3004 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 19, sub: 0, line: 210 } |  |  | 0.753 |
| walker |  | 3038 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 20, sub: 0, line: 213 } |  |  | 0.753 |
| walker |  | 3077 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 22, sub: 0, line: 222 } |  |  | 0.753 |
| walker |  | 3110 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 23, sub: 0, line: 225 } |  |  | 0.753 |
| walker |  | 3167 | 57 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 15, sub: 0, line: 185 } |  |  | 0.753 |
| walker |  | 3202 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 18, sub: 0, line: 193 } |  |  | 0.753 |
| walker |  | 3235 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 45, sub: 0, line: 495 } |  |  | 0.753 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.722 |
| walker |  | 3270 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 46, sub: 0, line: 498 } |  |  | 0.722 |
| walker |  | 3410 | 140 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 38, sub: 0, line: 426 } |  |  | 0.722 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.708 |
| walker |  | 3443 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 43, sub: 0, line: 468 } |  |  | 0.708 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.693 |
| walker |  | 3643 | 200 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 26, sub: 0, line: 253 } |  |  | 0.693 |
| walker |  | 3674 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 36, sub: 0, line: 398 } |  |  | 0.693 |
| walker |  | 3708 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 27, sub: 0, line: 278 } |  |  | 0.693 |
| walker |  | 3743 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 33, sub: 0, line: 357 } |  |  | 0.693 |
| walker |  | 3775 | 32 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 29, sub: 0, line: 291 } |  |  | 0.693 |
| walker |  | 3826 | 51 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 32, sub: 0, line: 347 } |  |  | 0.693 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.669 |
| walker |  | 4046 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.722 |
| walker |  | 4151 | 105 | Code::CodeKey { rung: Names, file: src/click/formatting.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 4188 | 37 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.722 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.689 |
| walker |  | 4253 | 65 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 4, sub: 0, line: 31 } |  |  | 0.689 |
| walker |  | 4280 | 27 | Code::CodeKey { rung: Body, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.689 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.684 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.674 |
| walker |  | 4490 | 210 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 5, sub: 0, line: 104 } |  |  | 0.675 |
| walker |  | 4499 | 9 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 15, sub: 0, line: 254 } |  |  | 0.675 |
| walker |  | 4508 | 9 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 16, sub: 0, line: 269 } |  |  | 0.675 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.669 |
| walker |  | 4564 | 56 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 6, sub: 0, line: 116 } |  |  | 0.669 |
| walker |  | 4622 | 58 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 14, sub: 0, line: 210 } |  |  | 0.669 |
| walker |  | 4633 | 11 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 8, sub: 0, line: 139 } |  |  | 0.669 |
| walker |  | 4644 | 11 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 9, sub: 0, line: 143 } |  |  | 0.669 |
| walker |  | 4655 | 11 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 17, sub: 0, line: 278 } |  |  | 0.669 |
| walker |  | 4668 | 13 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 11, sub: 0, line: 185 } |  |  | 0.669 |
| walker |  | 4681 | 13 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 12, sub: 0, line: 189 } |  |  | 0.669 |
| walker |  | 4695 | 14 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 16, sub: 0, line: 269 } |  |  | 0.669 |
| walker |  | 4710 | 15 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 7, sub: 0, line: 135 } |  |  | 0.669 |
| walker |  | 4746 | 36 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 13, sub: 0, line: 194 } |  |  | 0.669 |
| walker |  | 4802 | 56 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 15, sub: 0, line: 254 } |  |  | 0.669 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.652 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.647 |
| walker |  | 5150 | 348 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 3, sub: 0, line: 37 } |  |  | 0.648 |
| walker |  | 5181 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 11, sub: 0, line: 155 } |  |  | 0.648 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.631 |
| walker |  | 5216 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 8, sub: 0, line: 110 } |  |  | 0.631 |
| walker |  | 5269 | 53 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 10, sub: 0, line: 146 } |  |  | 0.631 |
| walker |  | 5325 | 56 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 5, sub: 0, line: 90 } |  |  | 0.631 |
| walker |  | 5341 | 16 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 6, sub: 0, line: 100 } |  |  | 0.631 |
| walker |  | 5357 | 16 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 10, sub: 0, line: 146 } |  |  | 0.631 |
| walker |  | 5403 | 46 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 7, sub: 0, line: 103 } |  |  | 0.631 |
| walker |  | 5463 | 60 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 33, sub: 0, line: 357 } |  |  | 0.631 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.616 |
| walker |  | 5523 | 60 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 34, sub: 0, line: 381 } |  |  | 0.616 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.603 |
| walker |  | 5751 | 228 | Code::CodeKey { rung: Names, file: src/click/_compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 5765 | 14 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.603 |
| walker |  | 5780 | 15 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 7, sub: 0, line: 48 } |  |  | 0.603 |
| walker |  | 5821 | 41 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 8, sub: 0, line: 56 } |  |  | 0.603 |
| walker |  | 5859 | 38 | Code::CodeKey { rung: Body, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.603 |
| walker |  | 5928 | 69 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 5, sub: 0, line: 19 } |  |  | 0.603 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.591 |
| walker |  | 6014 | 86 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 9, sub: 0, line: 57 } |  |  | 0.591 |
| walker |  | 6109 | 95 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 12, sub: 0, line: 82 } |  |  | 0.591 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.599 |
| walker |  | 6159 | 50 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 13, sub: 0, line: 92 } |  |  | 0.599 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.594 |
| walker |  | 6270 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.589 |
| walker |  | 6286 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.589 |
| walker |  | 6319 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.589 |
| walker |  | 6329 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.589 |
| walker |  | 6386 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.589 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.581 |
| walker |  | 6455 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.581 |
| walker |  | 6463 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.581 |
| walker |  | 6471 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.581 |
| walker |  | 6479 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.581 |
| walker |  | 6554 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.582 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.576 |
| walker |  | 6593 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.576 |
| walker |  | 6638 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.576 |
| walker |  | 6651 | 13 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.576 |
| walker |  | 6728 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.576 |
| walker |  | 6743 | 15 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.576 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.564 |
| walker |  | 6839 | 96 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 35, sub: 0, line: 311 } |  |  | 0.564 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.565 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.564 |
| walker |  | 6956 | 117 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 26, sub: 0, line: 205 } |  |  | 0.564 |
| walker |  | 7079 | 123 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 36, sub: 0, line: 525 } |  |  | 0.573 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.566 |
| walker |  | 7263 | 184 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 1, sub: 0, line: 26 } |  |  | 0.566 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.563 |
| walker |  | 7331 | 68 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 18, sub: 0, line: 103 } |  |  | 0.563 |
| walker |  | 7339 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 22, sub: 0, line: 154 } |  |  | 0.563 |
| walker |  | 7347 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 23, sub: 0, line: 158 } |  |  | 0.563 |
| walker |  | 7422 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 19, sub: 0, line: 117 } |  |  | 0.563 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.563 |
| walker |  | 7467 | 45 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.563 |
| walker |  | 7524 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.563 |
| walker |  | 7581 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 33, sub: 0, line: 295 } |  |  | 0.563 |
| walker |  | 7642 | 61 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.563 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.554 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.545 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.539 |
| walker |  | 8150 | 508 | Code::CodeKey { rung: Body, file: src/click/__init__.py, decl: 1, sub: 0, line: 77 } |  |  | 0.553 |
| walker |  | 8265 | 115 | Code::CodeKey { rung: Names, file: src/click/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 8308 | 43 | Code::CodeKey { rung: Decl, file: src/click/_utils.py, decl: 1, sub: 0, line: 7 } |  |  | 0.553 |
| walker |  | 8324 | 16 | Code::CodeKey { rung: Body, file: src/click/_utils.py, decl: 2, sub: 0, line: 18 } |  |  | 0.553 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.546 |
| walker |  | 8389 | 65 | Code::CodeKey { rung: Doc, file: src/click/_utils.py, decl: 1, sub: 0, line: 7 } |  |  | 0.546 |
| walker |  | 8528 | 139 | Code::CodeKey { rung: Names, file: src/click/parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 8548 | 20 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 12, sub: 0, line: 216 } |  |  | 0.546 |
| walker |  | 8587 | 39 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 9, sub: 0, line: 185 } |  |  | 0.546 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.538 |
| walker |  | 8636 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 2, sub: 0, line: 51 } |  |  | 0.538 |
| walker |  | 8685 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 5, sub: 0, line: 127 } |  |  | 0.538 |
| walker |  | 8693 | 8 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 7, sub: 0, line: 165 } |  |  | 0.538 |
| walker |  | 8743 | 50 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 11, sub: 0, line: 191 } |  |  | 0.538 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.530 |
| walker |  | 8828 | 85 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 6, sub: 0, line: 128 } |  |  | 0.530 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.524 |
| walker |  | 8971 | 143 | Code::CodeKey { rung: Names, file: src/click/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 8978 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 2, sub: 0, line: 12 } |  |  | 0.524 |
| walker |  | 8985 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 3, sub: 0, line: 16 } |  |  | 0.524 |
| walker |  | 8995 | 10 | Code::CodeKey { rung: Body, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.524 |
| walker |  | 9008 | 13 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.524 |
| walker |  | 9024 | 16 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 5, sub: 0, line: 44 } |  |  | 0.524 |
| walker |  | 9041 | 17 | Code::CodeKey { rung: Body, file: src/click/globals.py, decl: 5, sub: 0, line: 44 } |  |  | 0.524 |
| walker |  | 9096 | 55 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 7, sub: 0, line: 54 } |  |  | 0.524 |
| walker |  | 9110 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.530 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.524 |
| walker |  | 9193 | 83 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 18, sub: 0, line: 283 } |  |  | 0.524 |
| walker |  | 9277 | 84 | Code::CodeKey { rung: Doc, file: src/click/formatting.py, decl: 10, sub: 0, line: 147 } |  |  | 0.524 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.534 |
| walker |  | 9361 | 84 | Code::CodeKey { rung: Doc, file: src/click/types.py, decl: 4, sub: 0, line: 69 } |  |  | 0.534 |
| walker |  | 9451 | 90 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.534 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.530 |
| walker |  | 9662 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 9673 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.536 |
| walker |  | 9699 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.536 |
| walker |  | 9725 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.536 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.535 |
| walker |  | 9751 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.535 |
| walker |  | 9784 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.535 |
| walker |  | 9826 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.535 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.532 |
| walker |  | 9868 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.532 |
| walker |  | 9911 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.532 |
| walker |  | 9955 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.532 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.528 |
| walker |  | 9967 | 12 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.528 |
