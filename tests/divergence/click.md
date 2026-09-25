Score(3000)=0.711 I=0.877 C=0.576 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.728/0.781/0.789/0.711/0.650/0.565/0.540

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
| walker |  | 1345 | 29 | Markdown::Section { file: docs/setuptools.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.814 |
| walker |  | 1420 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.814 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.781 |
| walker |  | 1476 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.840 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.808 |
| walker |  | 1709 | 233 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.857 |
| walker |  | 1799 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.857 |
| walker |  | 1852 | 53 | Markdown::Section { file: docs/license.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.857 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.811 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.789 |
| walker |  | 2091 | 239 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.828 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.806 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.799 |
| walker |  | 2315 | 224 | Code::CodeKey { rung: Names, file: src/click/_winconsole.py, decl: 0, sub: 0, line: 0 } |  |  | 0.799 |
| walker |  | 2336 | 21 | Code::CodeKey { rung: Decl, file: src/click/_winconsole.py, decl: 9, sub: 0, line: 49 } |  |  | 0.799 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.774 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.756 |
| walker |  | 2576 | 240 | Code::CodeKey { rung: Names, file: src/click/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 2586 | 10 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 37, sub: 0, line: 422 } |  |  | 0.756 |
| walker |  | 2601 | 15 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 14, sub: 0, line: 181 } |  |  | 0.756 |
| walker |  | 2618 | 17 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 2, sub: 0, line: 32 } |  |  | 0.756 |
| walker |  | 2639 | 21 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 25, sub: 0, line: 248 } |  |  | 0.756 |
| walker |  | 2672 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 12, sub: 0, line: 173 } |  |  | 0.756 |
| walker |  | 2688 | 16 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 13, sub: 0, line: 176 } |  |  | 0.756 |
| walker |  | 2727 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 19, sub: 0, line: 210 } |  |  | 0.756 |
| walker |  | 2766 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 22, sub: 0, line: 222 } |  |  | 0.756 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.727 |
| walker |  | 2823 | 57 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 15, sub: 0, line: 185 } |  |  | 0.727 |
| walker |  | 2856 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 23, sub: 0, line: 225 } |  |  | 0.727 |
| walker |  | 2890 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 20, sub: 0, line: 213 } |  |  | 0.727 |
| walker |  | 2925 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 18, sub: 0, line: 193 } |  |  | 0.727 |
| walker |  | 2958 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 45, sub: 0, line: 495 } |  |  | 0.727 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.711 |
| walker |  | 3098 | 140 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 38, sub: 0, line: 426 } |  |  | 0.711 |
| walker |  | 3131 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 43, sub: 0, line: 468 } |  |  | 0.711 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.681 |
| walker |  | 3331 | 200 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 26, sub: 0, line: 253 } |  |  | 0.681 |
| walker |  | 3362 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 36, sub: 0, line: 398 } |  |  | 0.681 |
| walker |  | 3396 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 27, sub: 0, line: 278 } |  |  | 0.681 |
| walker |  | 3431 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 33, sub: 0, line: 357 } |  |  | 0.681 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.669 |
| walker |  | 3463 | 32 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 29, sub: 0, line: 291 } |  |  | 0.669 |
| walker |  | 3498 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 46, sub: 0, line: 498 } |  |  | 0.669 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.654 |
| walker |  | 3718 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.709 |
| walker |  | 3823 | 105 | Code::CodeKey { rung: Names, file: src/click/formatting.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3860 | 37 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.709 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.685 |
| walker |  | 3925 | 65 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 4, sub: 0, line: 31 } |  |  | 0.685 |
| walker |  | 3952 | 27 | Code::CodeKey { rung: Body, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.685 |
| walker |  | 4162 | 210 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 5, sub: 0, line: 104 } |  |  | 0.686 |
| walker |  | 4171 | 9 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 15, sub: 0, line: 254 } |  |  | 0.686 |
| walker |  | 4180 | 9 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 16, sub: 0, line: 269 } |  |  | 0.686 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.655 |
| walker |  | 4236 | 56 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 6, sub: 0, line: 116 } |  |  | 0.655 |
| walker |  | 4294 | 58 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 14, sub: 0, line: 210 } |  |  | 0.655 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.650 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.641 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.635 |
| walker |  | 4642 | 348 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 3, sub: 0, line: 37 } |  |  | 0.635 |
| walker |  | 4673 | 31 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 11, sub: 0, line: 155 } |  |  | 0.635 |
| walker |  | 4708 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 8, sub: 0, line: 110 } |  |  | 0.635 |
| walker |  | 4761 | 53 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 10, sub: 0, line: 146 } |  |  | 0.635 |
| walker |  | 4817 | 56 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 5, sub: 0, line: 90 } |  |  | 0.635 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.619 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.615 |
| walker |  | 5045 | 228 | Code::CodeKey { rung: Names, file: src/click/_compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5059 | 14 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.615 |
| walker |  | 5074 | 15 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 7, sub: 0, line: 48 } |  |  | 0.615 |
| walker |  | 5115 | 41 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 8, sub: 0, line: 56 } |  |  | 0.615 |
| walker |  | 5153 | 38 | Code::CodeKey { rung: Body, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.615 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.599 |
| walker |  | 5222 | 69 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 5, sub: 0, line: 19 } |  |  | 0.599 |
| walker |  | 5317 | 95 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 12, sub: 0, line: 82 } |  |  | 0.599 |
| walker |  | 5367 | 50 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 13, sub: 0, line: 92 } |  |  | 0.599 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.585 |
| walker |  | 5478 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 5494 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.585 |
| walker |  | 5527 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.585 |
| walker |  | 5537 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.585 |
| walker |  | 5594 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.585 |
| walker |  | 5663 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.585 |
| walker |  | 5671 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.585 |
| walker |  | 5679 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.585 |
| walker |  | 5687 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.585 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.572 |
| walker |  | 5762 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.572 |
| walker |  | 5801 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.572 |
| walker |  | 5846 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.572 |
| walker |  | 5923 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.572 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.560 |
| walker |  | 6019 | 96 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 35, sub: 0, line: 311 } |  |  | 0.560 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.570 |
| walker |  | 6203 | 184 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 1, sub: 0, line: 26 } |  |  | 0.570 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.565 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.560 |
| walker |  | 6320 | 117 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 26, sub: 0, line: 205 } |  |  | 0.560 |
| walker |  | 6443 | 123 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 36, sub: 0, line: 525 } |  |  | 0.561 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.564 |
| walker |  | 6511 | 68 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 18, sub: 0, line: 103 } |  |  | 0.564 |
| walker |  | 6519 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 22, sub: 0, line: 154 } |  |  | 0.564 |
| walker |  | 6527 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 23, sub: 0, line: 158 } |  |  | 0.564 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.558 |
| walker |  | 6584 | 57 | Code::CodeKey { rung: Decl, file: src/click/_textwrap.py, decl: 2, sub: 0, line: 9 } |  |  | 0.558 |
| walker |  | 6699 | 115 | Code::CodeKey { rung: Names, file: src/click/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 6742 | 43 | Code::CodeKey { rung: Decl, file: src/click/_utils.py, decl: 1, sub: 0, line: 7 } |  |  | 0.558 |
| walker |  | 6758 | 16 | Code::CodeKey { rung: Body, file: src/click/_utils.py, decl: 2, sub: 0, line: 18 } |  |  | 0.558 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.546 |
| walker |  | 6823 | 65 | Code::CodeKey { rung: Doc, file: src/click/_utils.py, decl: 1, sub: 0, line: 7 } |  |  | 0.546 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.548 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.547 |
| walker |  | 7043 | 220 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.573 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.566 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.563 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.563 |
| walker |  | 7551 | 508 | Code::CodeKey { rung: Body, file: src/click/__init__.py, decl: 1, sub: 0, line: 77 } |  |  | 0.578 |
| walker |  | 7690 | 139 | Code::CodeKey { rung: Names, file: src/click/parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.569 |
| walker |  | 7710 | 20 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 12, sub: 0, line: 216 } |  |  | 0.569 |
| walker |  | 7749 | 39 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 9, sub: 0, line: 185 } |  |  | 0.569 |
| walker |  | 7798 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 2, sub: 0, line: 51 } |  |  | 0.569 |
| walker |  | 7847 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 5, sub: 0, line: 127 } |  |  | 0.569 |
| walker |  | 7855 | 8 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 7, sub: 0, line: 165 } |  |  | 0.569 |
| walker |  | 7905 | 50 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 11, sub: 0, line: 191 } |  |  | 0.569 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.559 |
| walker |  | 8048 | 143 | Code::CodeKey { rung: Names, file: src/click/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 8055 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 2, sub: 0, line: 12 } |  |  | 0.559 |
| walker |  | 8062 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 3, sub: 0, line: 16 } |  |  | 0.559 |
| walker |  | 8072 | 10 | Code::CodeKey { rung: Body, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.559 |
| walker |  | 8085 | 13 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.559 |
| walker |  | 8101 | 16 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 5, sub: 0, line: 44 } |  |  | 0.559 |
| walker |  | 8118 | 17 | Code::CodeKey { rung: Body, file: src/click/globals.py, decl: 5, sub: 0, line: 44 } |  |  | 0.559 |
| walker |  | 8132 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.565 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.559 |
| walker |  | 8207 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 19, sub: 0, line: 117 } |  |  | 0.559 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.552 |
| walker |  | 8418 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 8429 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.559 |
| walker |  | 8455 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.559 |
| walker |  | 8481 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.559 |
| walker |  | 8507 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.559 |
| walker |  | 8549 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.559 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.551 |
| walker |  | 8591 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.551 |
| walker |  | 8634 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.551 |
| walker |  | 8678 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.551 |
| walker |  | 8711 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.551 |
| walker |  | 8781 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.551 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.543 |
| walker |  | 8887 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.546 |
| walker |  | 8957 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.546 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.540 |
| walker |  | 9027 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.540 |
| walker |  | 9098 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.540 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.533 |
| walker |  | 9187 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.533 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.543 |
| walker |  | 9493 | 306 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 9530 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.556 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.551 |
| walker |  | 9568 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.551 |
| walker |  | 9648 | 80 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.552 |
| walker |  | 9663 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.552 |
| walker |  | 9691 | 28 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.552 |
| walker |  | 9724 | 33 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.552 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.551 |
| walker |  | 9785 | 61 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.551 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.547 |
| walker |  | 9921 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.556 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.553 |
| walker |  | 9965 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.553 |
