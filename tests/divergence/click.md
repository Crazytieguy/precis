Score(3000)=0.811 I=0.941 C=0.699 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.804/0.825/0.866/0.811/0.684/0.629/0.628

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
| walker |  | 1238 | 248 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| ns | 1248 |  | 265 | Public API surface, part 1: object model and decorators | 2.1 |  | 0.814 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.781 |
| walker |  | 1471 | 233 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.832 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.801 |
| walker |  | 1710 | 239 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.841 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.800 |
| walker |  | 1930 | 220 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.852 |
| walker |  | 1959 | 29 | Markdown::Section { file: docs/setuptools.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.852 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.829 |
| walker |  | 2034 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 2090 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.878 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.855 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.847 |
| walker |  | 2180 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.847 |
| walker |  | 2233 | 53 | Markdown::Section { file: docs/license.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.847 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.820 |
| walker |  | 2453 | 220 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.884 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.863 |
| walker |  | 2564 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.863 |
| walker |  | 2574 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.863 |
| walker |  | 2590 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.863 |
| walker |  | 2623 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.863 |
| walker |  | 2680 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.863 |
| walker |  | 2748 | 68 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 18, sub: 0, line: 103 } |  |  | 0.863 |
| walker |  | 2756 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 22, sub: 0, line: 154 } |  |  | 0.863 |
| walker |  | 2764 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 23, sub: 0, line: 158 } |  |  | 0.863 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.830 |
| walker |  | 2833 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.830 |
| walker |  | 2841 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.830 |
| walker |  | 2849 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.830 |
| walker |  | 2857 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.830 |
| walker |  | 2932 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 19, sub: 0, line: 117 } |  |  | 0.830 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.811 |
| walker |  | 3007 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.811 |
| walker |  | 3046 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.811 |
| walker |  | 3091 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.811 |
| walker |  | 3168 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.811 |
| walker |  | 3181 | 13 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.811 |
| walker |  | 3196 | 15 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.811 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.778 |
| walker |  | 3292 | 96 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 35, sub: 0, line: 311 } |  |  | 0.778 |
| walker |  | 3409 | 117 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 26, sub: 0, line: 205 } |  |  | 0.778 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.763 |
| walker |  | 3532 | 123 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 36, sub: 0, line: 525 } |  |  | 0.764 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.748 |
| walker |  | 3716 | 184 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 1, sub: 0, line: 26 } |  |  | 0.748 |
| walker |  | 3761 | 45 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.748 |
| walker |  | 3818 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.748 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.722 |
| walker |  | 3875 | 57 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 33, sub: 0, line: 295 } |  |  | 0.722 |
| walker |  | 3936 | 61 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.722 |
| walker |  | 4004 | 68 | Code::CodeKey { rung: Doc, file: src/click/testing.py, decl: 20, sub: 0, line: 131 } |  |  | 0.722 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.690 |
| walker |  | 4215 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 4226 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.690 |
| walker |  | 4252 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.690 |
| walker |  | 4278 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.690 |
| walker |  | 4304 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.690 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.684 |
| walker |  | 4337 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.684 |
| walker |  | 4379 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.684 |
| walker |  | 4421 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.684 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.675 |
| walker |  | 4464 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.675 |
| walker |  | 4508 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.675 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.669 |
| walker |  | 4578 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.669 |
| walker |  | 4648 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.669 |
| walker |  | 4718 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.669 |
| walker |  | 4789 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.669 |
| walker |  | 4801 | 12 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.669 |
| walker |  | 4815 | 14 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 33, sub: 0, line: 330 } |  |  | 0.669 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.652 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.647 |
| walker |  | 4904 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.647 |
| walker |  | 4921 | 17 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.647 |
| walker |  | 5027 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.647 |
| walker |  | 5043 | 16 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.648 |
| walker |  | 5085 | 42 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.648 |
| walker |  | 5136 | 51 | Code::CodeKey { rung: Doc, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.648 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.631 |
| walker |  | 5442 | 306 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 5470 | 28 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.650 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.635 |
| walker |  | 5503 | 33 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.635 |
| walker |  | 5540 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.635 |
| walker |  | 5578 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.635 |
| walker |  | 5622 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.635 |
| walker |  | 5702 | 80 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.636 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.622 |
| walker |  | 5763 | 61 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.622 |
| walker |  | 5778 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.622 |
| walker |  | 5794 | 16 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 95, sub: 0, line: 2014 } |  |  | 0.622 |
| walker |  | 5930 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.635 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.623 |
| walker |  | 6087 | 157 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.626 |
| walker |  | 6096 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 137, sub: 0, line: 3430 } |  |  | 0.626 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.633 |
| walker |  | 6134 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 139, sub: 0, line: 3450 } |  |  | 0.633 |
| walker |  | 6193 | 59 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 136, sub: 0, line: 3409 } |  |  | 0.635 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.629 |
| walker |  | 6238 | 45 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 98, sub: 0, line: 2044 } |  |  | 0.629 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.624 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.625 |
| walker |  | 6524 | 286 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 121, sub: 0, line: 2671 } |  |  | 0.646 |
| walker |  | 6560 | 36 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 133, sub: 0, line: 3319 } |  |  | 0.646 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.640 |
| walker |  | 6598 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 126, sub: 0, line: 2974 } |  |  | 0.640 |
| walker |  | 6638 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 124, sub: 0, line: 2935 } |  |  | 0.640 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.626 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.627 |
| walker |  | 6918 | 280 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 122, sub: 0, line: 2745 } |  |  | 0.652 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.650 |
| walker |  | 6966 | 48 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 71, sub: 0, line: 1524 } |  |  | 0.650 |
| walker |  | 7014 | 48 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 92, sub: 0, line: 1981 } |  |  | 0.650 |
| walker |  | 7069 | 55 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 123, sub: 0, line: 2916 } |  |  | 0.650 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.635 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.631 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.625 |
| walker |  | 7513 | 444 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 99, sub: 0, line: 2054 } |  |  | 0.647 |
| walker |  | 7522 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 104, sub: 0, line: 2254 } |  |  | 0.647 |
| walker |  | 7532 | 10 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 109, sub: 0, line: 2318 } |  |  | 0.647 |
| walker |  | 7569 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 110, sub: 0, line: 2321 } |  |  | 0.647 |
| walker |  | 7609 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 108, sub: 0, line: 2285 } |  |  | 0.647 |
| walker |  | 7651 | 42 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 116, sub: 0, line: 2570 } |  |  | 0.647 |
| walker |  | 7699 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 103, sub: 0, line: 2249 } |  |  | 0.647 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.653 |
| walker |  | 7747 | 48 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 106, sub: 0, line: 2275 } |  |  | 0.653 |
| walker |  | 7794 | 47 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 107, sub: 0, line: 2280 } |  |  | 0.653 |
| walker |  | 7841 | 47 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 104, sub: 0, line: 2254 } |  |  | 0.653 |
| walker |  | 7888 | 47 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 111, sub: 0, line: 2369 } |  |  | 0.653 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.642 |
| walker |  | 7961 | 73 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.642 |
| walker |  | 8037 | 76 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 130, sub: 0, line: 3203 } |  |  | 0.642 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.635 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.627 |
| walker |  | 8505 | 468 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 100, sub: 0, line: 2147 } |  |  | 0.647 |
| walker |  | 8585 | 80 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 119, sub: 0, line: 2637 } |  |  | 0.647 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.637 |
| walker |  | 8667 | 82 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.642 |
| walker |  | 8776 | 109 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 115, sub: 0, line: 2554 } |  |  | 0.642 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.633 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.626 |
| walker |  | 9023 | 247 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 0, line: 185 } |  |  | 0.628 |
| walker |  | 9031 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 11, sub: 0, line: 459 } |  |  | 0.628 |
| walker |  | 9039 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 16, sub: 0, line: 549 } |  |  | 0.628 |
| walker |  | 9048 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 15, sub: 0, line: 511 } |  |  | 0.628 |
| walker |  | 9108 | 60 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 14, sub: 0, line: 497 } |  |  | 0.628 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.621 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.614 |
| walker |  | 9368 | 260 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 10, sub: 0, line: 289 } |  |  | 0.633 |
| walker |  | 9475 | 107 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 17, sub: 0, line: 577 } |  |  | 0.633 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.625 |
| walker |  | 9585 | 110 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 12, sub: 0, line: 471 } |  |  | 0.625 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.620 |
| walker |  | 9819 | 234 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 9, sub: 1, line: 185 } |  |  | 0.632 |
| walker |  | 9827 | 8 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 22, sub: 0, line: 657 } |  |  | 0.632 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.628 |
| walker |  | 9872 | 45 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 27, sub: 0, line: 718 } |  |  | 0.628 |
| walker |  | 9921 | 49 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 28, sub: 0, line: 723 } |  |  | 0.628 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.624 |
| walker |  | 9981 | 60 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 21, sub: 0, line: 639 } |  |  | 0.624 |
| walker |  | 9992 | 11 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 31, sub: 0, line: 758 } |  |  | 0.624 |
