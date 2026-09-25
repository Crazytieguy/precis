Score(3000)=0.668 I=0.849 C=0.526 ns_rows≤3K=20/58 grid(1000/1442/2080/3000/4327/6240/9000)=0.605/0.768/0.778/0.668/0.599/0.570/0.542

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
| ns | 476 |  | 156 | The canonical hello-world program | 1.5 |  | 0.694 |
| ns | 524 |  | 48 | The terminal session that program produces | 1.6 | 1.5 | 0.662 |
| walker |  | 695 | 248 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 707 |  | 183 | Test suite listing: tests/ and tests/typing/ | 1.7 |  | 0.531 |
| walker |  | 708 | 13 | Fs::DirListing { dir: .github } |  |  | 0.531 |
| walker |  | 731 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.532 |
| walker |  | 747 | 16 | Code::CodeKey { rung: Names, file: src/click/_textwrap.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| ns | 776 |  | 69 | Examples listing | 1.8 |  | 0.484 |
| walker |  | 785 | 38 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.484 |
| walker |  | 977 | 192 | Fs::DirListing { dir: docs } |  |  | 0.502 |
| ns | 983 |  | 207 | Documentation listing: docs/ | 1.9 |  | 0.580 |
| walker |  | 992 | 15 | Fs::DirListing { dir: docs/_static } |  |  | 0.605 |
| walker |  | 1091 | 99 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.703 |
| walker |  | 1127 | 36 | Fs::DirListing { dir: examples } |  |  | 0.736 |
| ns | 1248 |  | 265 | Public API surface, part 1: object model and decorators | 2.1 |  | 0.745 |
| walker |  | 1360 | 233 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.759 |
| walker |  | 1413 | 53 | Code::CodeKey { rung: Decl, file: src/click/_textwrap.py, decl: 1, sub: 0, line: 8 } |  |  | 0.759 |
| walker |  | 1422 | 9 | Code::CodeKey { rung: Decl, file: src/click/_textwrap.py, decl: 3, sub: 0, line: 27 } |  |  | 0.759 |
| ns | 1440 |  | 192 | Public API surface, part 2: exceptions, formatting, globals | 2.2 |  | 0.768 |
| walker |  | 1549 | 127 | Fs::DirListing { dir: tests } |  |  | 0.832 |
| ns | 1629 |  | 189 | Public API surface, part 3: terminal UI exports | 2.3 |  | 0.801 |
| walker |  | 1788 | 239 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.841 |
| ns | 1899 |  | 270 | Public API surface, part 4: parameter types and utilities | 2.4 |  | 0.800 |
| ns | 1980 |  | 81 | Deprecated names still resolvable via module __getattr__ | 2.5 |  | 0.778 |
| walker |  | 2012 | 224 | Code::CodeKey { rung: Names, file: src/click/_winconsole.py, decl: 0, sub: 0, line: 0 } |  |  | 0.778 |
| walker |  | 2033 | 21 | Code::CodeKey { rung: Decl, file: src/click/_winconsole.py, decl: 9, sub: 0, line: 49 } |  |  | 0.778 |
| ns | 2124 |  | 144 | core.py class roster with exact line numbers | 3.1 |  | 0.758 |
| ns | 2174 |  | 50 | Command: what it is | 3.2 | 3.1 | 0.751 |
| walker |  | 2273 | 240 | Code::CodeKey { rung: Names, file: src/click/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 2283 | 10 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 37, sub: 0, line: 422 } |  |  | 0.751 |
| walker |  | 2298 | 15 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 14, sub: 0, line: 181 } |  |  | 0.751 |
| walker |  | 2315 | 17 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 2, sub: 0, line: 32 } |  |  | 0.751 |
| walker |  | 2336 | 21 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 25, sub: 0, line: 248 } |  |  | 0.751 |
| ns | 2368 |  | 194 | Command constructor: every knob a command accepts | 3.3 |  | 0.728 |
| walker |  | 2369 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 12, sub: 0, line: 173 } |  |  | 0.728 |
| walker |  | 2385 | 16 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 13, sub: 0, line: 176 } |  |  | 0.728 |
| walker |  | 2424 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 19, sub: 0, line: 210 } |  |  | 0.728 |
| walker |  | 2463 | 39 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 22, sub: 0, line: 222 } |  |  | 0.728 |
| ns | 2507 |  | 139 | Context: what it is | 3.4 | 3.1 | 0.711 |
| walker |  | 2520 | 57 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 15, sub: 0, line: 185 } |  |  | 0.711 |
| walker |  | 2553 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 23, sub: 0, line: 225 } |  |  | 0.711 |
| walker |  | 2587 | 34 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 20, sub: 0, line: 213 } |  |  | 0.711 |
| walker |  | 2622 | 35 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 18, sub: 0, line: 193 } |  |  | 0.711 |
| walker |  | 2651 | 29 | Markdown::Section { file: docs/setuptools.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.711 |
| walker |  | 2756 | 105 | Code::CodeKey { rung: Names, file: src/click/formatting.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| ns | 2778 |  | 271 | Context constructor: the invocation-settings surface | 3.5 |  | 0.684 |
| walker |  | 2793 | 37 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.684 |
| walker |  | 2858 | 65 | Code::CodeKey { rung: Decl, file: src/click/formatting.py, decl: 4, sub: 0, line: 31 } |  |  | 0.684 |
| ns | 2959 |  | 181 | Command.main: the process entry point | 3.6 |  | 0.668 |
| walker |  | 3086 | 228 | Code::CodeKey { rung: Names, file: src/click/_compat.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 3100 | 14 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 6, sub: 0, line: 40 } |  |  | 0.668 |
| walker |  | 3115 | 15 | Code::CodeKey { rung: Doc, file: src/click/_compat.py, decl: 7, sub: 0, line: 48 } |  |  | 0.668 |
| walker |  | 3226 | 111 | Code::CodeKey { rung: Names, file: src/click/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 3242 | 16 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 16, sub: 0, line: 89 } |  |  | 0.668 |
| ns | 3246 |  | 287 | Command method roster (core.py 1009-1511) | 3.7 | 3.6 | 0.640 |
| walker |  | 3275 | 33 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 24, sub: 0, line: 163 } |  |  | 0.640 |
| walker |  | 3285 | 10 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.640 |
| walker |  | 3342 | 57 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 12, sub: 0, line: 70 } |  |  | 0.640 |
| walker |  | 3411 | 69 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 25, sub: 0, line: 183 } |  |  | 0.640 |
| walker |  | 3419 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 27, sub: 0, line: 226 } |  |  | 0.640 |
| walker |  | 3427 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 28, sub: 0, line: 238 } |  |  | 0.640 |
| ns | 3434 |  | 188 | Group: nesting and the commands mapping | 3.8 | 3.1 | 0.628 |
| walker |  | 3435 | 8 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 29, sub: 0, line: 245 } |  |  | 0.628 |
| walker |  | 3510 | 75 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 31, sub: 0, line: 261 } |  |  | 0.628 |
| walker |  | 3549 | 39 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 34, sub: 0, line: 302 } |  |  | 0.628 |
| walker |  | 3594 | 45 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 37, sub: 0, line: 638 } |  |  | 0.628 |
| ns | 3611 |  | 177 | Group constructor: chain, result_callback and the rest | 3.9 |  | 0.615 |
| walker |  | 3709 | 115 | Code::CodeKey { rung: Names, file: src/click/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 3752 | 43 | Code::CodeKey { rung: Decl, file: src/click/_utils.py, decl: 1, sub: 0, line: 7 } |  |  | 0.615 |
| ns | 3871 |  | 260 | Group and CommandCollection method rosters | 3.10 |  | 0.594 |
| walker |  | 3972 | 220 | Code::CodeKey { rung: Names, file: src/click/__init__.py, decl: 0, sub: 3, line: 0 } |  |  | 0.632 |
| walker |  | 4005 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 45, sub: 0, line: 495 } |  |  | 0.632 |
| walker |  | 4144 | 139 | Code::CodeKey { rung: Names, file: src/click/parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4164 | 20 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 12, sub: 0, line: 216 } |  |  | 0.632 |
| walker |  | 4203 | 39 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 9, sub: 0, line: 185 } |  |  | 0.632 |
| ns | 4212 |  | 341 | Context method roster (core.py 460-884) | 3.11 |  | 0.604 |
| walker |  | 4278 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.604 |
| ns | 4321 |  | 109 | CommandCollection: composing several groups | 3.12 | 3.1 | 0.599 |
| walker |  | 4421 | 143 | Code::CodeKey { rung: Names, file: src/click/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 4428 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 2, sub: 0, line: 12 } |  |  | 0.599 |
| walker |  | 4435 | 7 | Code::CodeKey { rung: Decl, file: src/click/globals.py, decl: 3, sub: 0, line: 16 } |  |  | 0.599 |
| walker |  | 4445 | 10 | Code::CodeKey { rung: Body, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.599 |
| ns | 4451 |  | 130 | ParameterSource members: where a value came from | 3.13 |  | 0.591 |
| walker |  | 4458 | 13 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 6, sub: 0, line: 49 } |  |  | 0.591 |
| walker |  | 4474 | 16 | Code::CodeKey { rung: Doc, file: src/click/globals.py, decl: 5, sub: 0, line: 44 } |  |  | 0.591 |
| ns | 4551 |  | 100 | Parameter: the shared base of options and arguments | 4.1 | 3.1 | 0.585 |
| walker |  | 4614 | 140 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 38, sub: 0, line: 426 } |  |  | 0.585 |
| walker |  | 4647 | 33 | Code::CodeKey { rung: Decl, file: src/click/types.py, decl: 43, sub: 0, line: 468 } |  |  | 0.585 |
| walker |  | 4688 | 41 | Code::CodeKey { rung: Decl, file: src/click/_compat.py, decl: 8, sub: 0, line: 56 } |  |  | 0.585 |
| walker |  | 4744 | 56 | Fs::DirListing { dir: tests/typing } |  |  | 0.620 |
| ns | 4832 |  | 281 | Parameter constructor: settings shared by options and arguments | 4.2 |  | 0.605 |
| walker |  | 4834 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.605 |
| ns | 4894 |  | 62 | Option: what it adds over a plain parameter | 4.3 | 3.1 | 0.600 |
| walker |  | 4911 | 77 | Code::CodeKey { rung: Decl, file: src/click/testing.py, decl: 32, sub: 0, line: 283 } |  |  | 0.600 |
| walker |  | 5122 | 211 | Code::CodeKey { rung: Names, file: src/click/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 5133 | 11 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 24, sub: 0, line: 278 } |  |  | 0.601 |
| walker |  | 5159 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 11, sub: 0, line: 108 } |  |  | 0.601 |
| walker |  | 5185 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 18, sub: 0, line: 221 } |  |  | 0.601 |
| ns | 5186 |  | 292 | Option constructor: the full option feature set | 4.4 |  | 0.585 |
| walker |  | 5211 | 26 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 21, sub: 0, line: 251 } |  |  | 0.585 |
| walker |  | 5253 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 14, sub: 0, line: 150 } |  |  | 0.585 |
| walker |  | 5295 | 42 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 34, sub: 0, line: 334 } |  |  | 0.585 |
| walker |  | 5338 | 43 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 30, sub: 0, line: 313 } |  |  | 0.585 |
| walker |  | 5382 | 44 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 27, sub: 0, line: 304 } |  |  | 0.585 |
| walker |  | 5415 | 33 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 25, sub: 0, line: 288 } |  |  | 0.585 |
| ns | 5471 |  | 285 | Argument: positional parameters and the required-by-default rule | 4.5 | 3.1 | 0.571 |
| walker |  | 5485 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 8, sub: 0, line: 65 } |  |  | 0.571 |
| walker |  | 5591 | 106 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 3, sub: 0, line: 35 } |  |  | 0.572 |
| walker |  | 5661 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 19, sub: 0, line: 227 } |  |  | 0.572 |
| ns | 5728 |  | 257 | Parameter method roster (core.py 2218-2647) | 4.6 |  | 0.559 |
| walker |  | 5731 | 70 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 22, sub: 0, line: 254 } |  |  | 0.559 |
| walker |  | 5802 | 71 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 12, sub: 0, line: 126 } |  |  | 0.559 |
| ns | 5965 |  | 237 | Option and Argument method rosters | 4.7 |  | 0.548 |
| walker |  | 6108 | 306 | Code::CodeKey { rung: Names, file: src/click/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| ns | 6112 |  | 147 | Package identity and runtime dependencies | 5.1 |  | 0.575 |
| walker |  | 6145 | 37 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.575 |
| walker |  | 6183 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 7, sub: 0, line: 119 } |  |  | 0.575 |
| ns | 6209 |  | 97 | How tests are run: pytest configuration | 5.2 |  | 0.570 |
| walker |  | 6263 | 80 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 93, sub: 0, line: 1988 } |  |  | 0.571 |
| ns | 6270 |  | 61 | The pytest fixture every test uses | 5.3 |  | 0.566 |
| walker |  | 6278 | 15 | Code::CodeKey { rung: Doc, file: src/click/core.py, decl: 6, sub: 0, line: 100 } |  |  | 0.566 |
| walker |  | 6306 | 28 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 3, sub: 0, line: 57 } |  |  | 0.566 |
| walker |  | 6339 | 33 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 4, sub: 0, line: 76 } |  |  | 0.566 |
| walker |  | 6400 | 61 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 94, sub: 0, line: 2004 } |  |  | 0.566 |
| ns | 6452 |  | 182 | CliRunner.invoke: running a CLI in a test | 5.4 |  | 0.559 |
| walker |  | 6536 | 136 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 8, sub: 0, line: 146 } |  |  | 0.571 |
| walker |  | 6580 | 44 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 68, sub: 0, line: 1516 } |  |  | 0.571 |
| ns | 6581 |  | 129 | tox environments: the developer command surface | 5.5 |  | 0.566 |
| walker |  | 6737 | 157 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 135, sub: 0, line: 3399 } |  |  | 0.568 |
| walker |  | 6746 | 9 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 137, sub: 0, line: 3430 } |  |  | 0.568 |
| walker |  | 6805 | 59 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 136, sub: 0, line: 3409 } |  |  | 0.570 |
| ns | 6818 |  | 237 | Lint and type-check configuration | 5.6 |  | 0.558 |
| walker |  | 6843 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 139, sub: 0, line: 3450 } |  |  | 0.558 |
| ns | 6868 |  | 50 | CI and repository automation listing | 5.7 |  | 0.559 |
| ns | 6927 |  | 59 | The exact command CI runs | 5.8 |  | 0.558 |
| walker |  | 7129 | 286 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 121, sub: 0, line: 2671 } |  |  | 0.578 |
| walker |  | 7165 | 36 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 133, sub: 0, line: 3319 } |  |  | 0.578 |
| walker |  | 7205 | 40 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 124, sub: 0, line: 2935 } |  |  | 0.578 |
| ns | 7226 |  | 299 | types.py class roster: the ParamType hierarchy | 6.1 |  | 0.570 |
| walker |  | 7243 | 38 | Code::CodeKey { rung: Decl, file: src/click/core.py, decl: 126, sub: 0, line: 2974 } |  |  | 0.570 |
| walker |  | 7270 | 27 | Code::CodeKey { rung: Body, file: src/click/formatting.py, decl: 3, sub: 0, line: 24 } |  |  | 0.570 |
| ns | 7306 |  | 80 | The exported type singletons | 6.2 |  | 0.567 |
| walker |  | 7319 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 2, sub: 0, line: 51 } |  |  | 0.567 |
| walker |  | 7368 | 49 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 5, sub: 0, line: 127 } |  |  | 0.567 |
| walker |  | 7376 | 8 | Code::CodeKey { rung: Decl, file: src/click/parser.py, decl: 7, sub: 0, line: 165 } |  |  | 0.567 |
| ns | 7442 |  | 136 | ParamType: the interface a custom type implements | 6.3 |  | 0.562 |
| walker |  | 7553 | 177 | Code::CodeKey { rung: Names, file: src/click/_termui_impl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 7563 | 10 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 26, sub: 0, line: 417 } |  |  | 0.562 |
| walker |  | 7610 | 47 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 22, sub: 0, line: 375 } |  |  | 0.562 |
| walker |  | 7646 | 36 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 25, sub: 0, line: 386 } |  |  | 0.562 |
| ns | 7707 |  | 265 | Exception hierarchy with exit codes | 6.4 |  | 0.563 |
| walker |  | 7772 | 126 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 30, sub: 0, line: 608 } |  |  | 0.563 |
| walker |  | 7782 | 10 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 34, sub: 0, line: 668 } |  |  | 0.563 |
| walker |  | 7792 | 10 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 35, sub: 0, line: 673 } |  |  | 0.563 |
| walker |  | 7864 | 72 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 31, sub: 0, line: 609 } |  |  | 0.563 |
| walker |  | 7911 | 47 | Code::CodeKey { rung: Decl, file: src/click/_termui_impl.py, decl: 29, sub: 0, line: 597 } |  |  | 0.563 |
| ns | 7912 |  | 205 | decorators.py roster: every decorator and its overloads | 7.1 |  | 0.553 |
| walker |  | 8000 | 89 | Code::CodeKey { rung: Decl, file: src/click/exceptions.py, decl: 15, sub: 0, line: 162 } |  |  | 0.553 |
| ns | 8143 |  | 231 | @click.command: the naming rule | 7.2 | 7.1 | 0.547 |
| ns | 8347 |  | 204 | style and secho: colours and text attributes | 8.1 |  | 0.541 |
| walker |  | 8507 | 507 | Code::CodeKey { rung: Names, file: src/click/termui.py, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 8533 | 26 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 13, sub: 0, line: 301 } |  |  | 0.542 |
| walker |  | 8584 | 51 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 14, sub: 0, line: 320 } |  |  | 0.542 |
| ns | 8590 |  | 243 | prompt and confirm signatures | 8.2 |  | 0.534 |
| walker |  | 8595 | 11 | Code::CodeKey { rung: Body, file: src/click/termui.py, decl: 21, sub: 0, line: 705 } |  |  | 0.534 |
| walker |  | 8671 | 76 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 12, sub: 0, line: 242 } |  |  | 0.537 |
| walker |  | 8756 | 85 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 24, sub: 0, line: 761 } |  |  | 0.537 |
| ns | 8798 |  | 208 | termui.py roster: the remaining terminal functions | 8.3 |  | 0.548 |
| walker |  | 8842 | 86 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 22, sub: 0, line: 717 } |  |  | 0.548 |
| walker |  | 8928 | 86 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 23, sub: 0, line: 751 } |  |  | 0.548 |
| ns | 8963 |  | 165 | utils.py roster: echo, streams, files and app directories | 8.4 |  | 0.542 |
| walker |  | 9029 | 101 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 25, sub: 0, line: 771 } |  |  | 0.542 |
| walker |  | 9138 | 109 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 26, sub: 0, line: 782 } |  |  | 0.542 |
| ns | 9159 |  | 196 | shell_completion.py roster: per-shell backends | 9.1 |  | 0.536 |
| walker |  | 9287 | 149 | Code::CodeKey { rung: Decl, file: src/click/termui.py, decl: 11, sub: 0, line: 129 } |  |  | 0.552 |
| ns | 9340 |  | 181 | formatting.py: HelpFormatter and text wrapping | 9.2 |  | 0.547 |
| ns | 9555 |  | 215 | parser.py: the private option parser | 9.3 |  | 0.543 |
| walker |  | 9600 | 313 | Code::CodeKey { rung: Names, file: src/click/shell_completion.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 9629 | 29 | Code::CodeKey { rung: Decl, file: src/click/shell_completion.py, decl: 30, sub: 0, line: 449 } |  |  | 0.558 |
| walker |  | 9688 | 59 | Code::CodeKey { rung: Decl, file: src/click/shell_completion.py, decl: 2, sub: 0, line: 57 } |  |  | 0.558 |
| ns | 9748 |  | 193 | globals.py and the UNSET sentinel | 9.4 |  | 0.557 |
| walker |  | 9751 | 63 | Code::CodeKey { rung: Decl, file: src/click/shell_completion.py, decl: 1, sub: 0, line: 19 } |  |  | 0.557 |
| walker |  | 9819 | 68 | Code::CodeKey { rung: Decl, file: src/click/shell_completion.py, decl: 25, sub: 0, line: 403 } |  |  | 0.557 |
| ns | 9861 |  | 113 | _termui_impl.py roster: ProgressBar, pagers, Editor | 9.5 |  | 0.562 |
| walker |  | 9888 | 69 | Code::CodeKey { rung: Decl, file: src/click/shell_completion.py, decl: 22, sub: 0, line: 367 } |  |  | 0.562 |
| ns | 9959 |  | 98 | Changelog head: the unreleased 8.4.0 section | 10.1 |  | 0.558 |
| walker |  | 9982 | 94 | Code::CodeKey { rung: Decl, file: src/click/shell_completion.py, decl: 17, sub: 0, line: 308 } |  |  | 0.558 |
| walker |  | 9988 | 6 | Code::CodeKey { rung: Decl, file: src/click/shell_completion.py, decl: 18, sub: 0, line: 314 } |  |  | 0.558 |
