Score(3000)=0.619 I=0.875 C=0.438 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.883/0.901/0.718/0.619/0.552/0.487/0.550

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 34 |  | 34 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 140 | 106 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 1.000 |
| ns | 140 |  | 106 | README identity paragraph | 1.2 |  | 1.000 |
| walker |  | 181 | 41 | Fs::DirListing { dir: docs } |  |  | 1.000 |
| ns | 216 |  | 76 | Package module roster: src/typeguard/ | 1.3 |  | 0.661 |
| walker |  | 255 | 74 | Toml::Identity { file: pyproject.toml } |  |  | 0.661 |
| ns | 311 |  | 95 | README: the two principal checking modes | 1.4 |  | 0.568 |
| walker |  | 331 | 76 | Fs::DirListing { dir: src/typeguard } |  |  | 0.863 |
| walker |  | 382 | 51 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.864 |
| walker |  | 409 | 27 | Fs::DirListing { dir: .github } |  |  | 0.864 |
| walker |  | 417 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.788 |
| ns | 417 |  | 106 | README: what instrumentation actually covers | 1.5 | 1.4 | 0.788 |
| ns | 550 |  | 133 | README: the two instrumentation entry points | 1.6 |  | 0.686 |
| ns | 746 |  | 196 | Public exports of typeguard/__init__.py, first half | 1.7 |  | 0.607 |
| walker |  | 796 | 379 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.890 |
| ns | 952 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.805 |
| walker |  | 989 | 193 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.883 |
| ns | 1107 |  | 155 | Docs and test tree listings | 1.9 |  | 0.753 |
| walker |  | 1250 | 261 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.854 |
| ns | 1337 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.769 |
| walker |  | 1350 | 100 | Fs::DirListing { dir: tests } |  |  | 0.872 |
| walker |  | 1364 | 14 | Fs::DirListing { dir: tests/mypy } |  |  | 0.901 |
| ns | 1471 |  | 134 | docs/index.rst in full | 1.11 |  | 0.833 |
| ns | 1634 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.791 |
| ns | 1750 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.769 |
| walker |  | 1791 | 427 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 1815 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 1, sub: 0, line: 81 } |  |  | 0.772 |
| walker |  | 1842 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 2, sub: 0, line: 84 } |  |  | 0.772 |
| walker |  | 1873 | 31 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 4, sub: 0, line: 89 } |  |  | 0.772 |
| walker |  | 1910 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 32, sub: 0, line: 924 } |  |  | 0.772 |
| walker |  | 1948 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 34, sub: 0, line: 1061 } |  |  | 0.772 |
| walker |  | 1998 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 7, sub: 0, line: 153 } |  |  | 0.772 |
| walker |  | 2048 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 8, sub: 0, line: 212 } |  |  | 0.772 |
| ns | 2057 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.718 |
| walker |  | 2098 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 9, sub: 0, line: 247 } |  |  | 0.718 |
| walker |  | 2148 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 10, sub: 0, line: 305 } |  |  | 0.718 |
| walker |  | 2198 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 11, sub: 0, line: 324 } |  |  | 0.718 |
| walker |  | 2248 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 12, sub: 0, line: 343 } |  |  | 0.718 |
| walker |  | 2298 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 13, sub: 0, line: 365 } |  |  | 0.718 |
| walker |  | 2348 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 14, sub: 0, line: 423 } |  |  | 0.718 |
| walker |  | 2398 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 15, sub: 0, line: 447 } |  |  | 0.718 |
| walker |  | 2448 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 16, sub: 0, line: 474 } |  |  | 0.718 |
| ns | 2449 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.661 |
| walker |  | 2498 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 17, sub: 0, line: 536 } |  |  | 0.661 |
| walker |  | 2548 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 18, sub: 0, line: 545 } |  |  | 0.661 |
| walker |  | 2598 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 21, sub: 0, line: 590 } |  |  | 0.661 |
| ns | 2611 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.638 |
| walker |  | 2648 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 22, sub: 0, line: 623 } |  |  | 0.638 |
| walker |  | 2698 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 23, sub: 0, line: 632 } |  |  | 0.638 |
| ns | 2735 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.619 |
| walker |  | 2748 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 24, sub: 0, line: 641 } |  |  | 0.619 |
| walker |  | 2798 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 25, sub: 0, line: 651 } |  |  | 0.619 |
| walker |  | 2848 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 26, sub: 0, line: 663 } |  |  | 0.619 |
| walker |  | 2898 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 28, sub: 0, line: 834 } |  |  | 0.619 |
| walker |  | 2948 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 29, sub: 0, line: 885 } |  |  | 0.619 |
| walker |  | 2998 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 30, sub: 0, line: 895 } |  |  | 0.619 |
| ns | 3033 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.578 |
| walker |  | 3048 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 31, sub: 0, line: 915 } |  |  | 0.578 |
| walker |  | 3117 | 69 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 19, sub: 0, line: 555 } |  |  | 0.578 |
| walker |  | 3130 | 13 | Code::CodeKey { rung: Names, file: src/typeguard/_memo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3168 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 1, sub: 0, line: 8 } |  |  | 0.578 |
| walker |  | 3239 | 71 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.579 |
| ns | 3262 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.556 |
| walker |  | 3292 | 53 | Code::CodeKey { rung: Names, file: src/typeguard/_exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 3309 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.556 |
| walker |  | 3326 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.556 |
| walker |  | 3379 | 53 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.556 |
| walker |  | 3388 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 3, sub: 0, line: 15 } |  |  | 0.556 |
| walker |  | 3397 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 5, sub: 0, line: 22 } |  |  | 0.556 |
| ns | 3430 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.541 |
| walker |  | 3547 | 150 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| ns | 3557 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.535 |
| walker |  | 3593 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 118 } |  |  | 0.535 |
| walker |  | 3640 | 47 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 149 } |  |  | 0.535 |
| walker |  | 3688 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 185 } |  |  | 0.535 |
| walker |  | 3736 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 216 } |  |  | 0.535 |
| ns | 3763 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.525 |
| walker |  | 3785 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 10, sub: 0, line: 245 } |  |  | 0.525 |
| walker |  | 3874 | 89 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 39 } |  |  | 0.525 |
| ns | 3956 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.513 |
| walker |  | 3962 | 88 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 28 } |  |  | 0.513 |
| walker |  | 4087 | 125 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 50 } |  |  | 0.546 |
| ns | 4102 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.534 |
| walker |  | 4156 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 4184 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.537 |
| ns | 4212 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.530 |
| walker |  | 4230 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.532 |
| walker |  | 4322 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.552 |
| walker |  | 4439 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 4447 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.535 |
| walker |  | 4471 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.535 |
| walker |  | 4504 | 33 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.535 |
| walker |  | 4553 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.537 |
| ns | 4610 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.540 |
| walker |  | 4613 | 60 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.540 |
| walker |  | 4665 | 52 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.540 |
| walker |  | 4726 | 61 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.541 |
| walker |  | 4782 | 56 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.541 |
| walker |  | 4858 | 76 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.541 |
| walker |  | 4870 | 12 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 16, sub: 0, line: 175 } |  |  | 0.541 |
| walker |  | 4889 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.544 |
| walker |  | 4909 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.548 |
| ns | 4961 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.524 |
| walker |  | 5041 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 5077 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 7, sub: 0, line: 172 } |  |  | 0.524 |
| walker |  | 5084 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 8, sub: 0, line: 176 } |  |  | 0.524 |
| walker |  | 5201 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 5207 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 22 } |  |  | 0.535 |
| walker |  | 5213 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 6, sub: 0, line: 26 } |  |  | 0.537 |
| ns | 5220 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.520 |
| walker |  | 5247 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 7, sub: 0, line: 30 } |  |  | 0.528 |
| ns | 5573 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.510 |
| walker |  | 5741 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.510 |
| ns | 5756 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.502 |
| ns | 6028 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.490 |
| ns | 6172 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.484 |
| walker |  | 6257 | 516 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 0, line: 1005 } |  |  | 0.487 |
| walker |  | 6389 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_decorators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 6397 | 8 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 6, sub: 0, line: 146 } |  |  | 0.487 |
| walker |  | 6431 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 3, sub: 0, line: 36 } |  |  | 0.487 |
| ns | 6471 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.513 |
| walker |  | 6535 | 104 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 7, sub: 0, line: 150 } |  |  | 0.527 |
| walker |  | 6641 | 106 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 5, sub: 0, line: 136 } |  |  | 0.527 |
| walker |  | 6710 | 69 | Code::CodeKey { rung: Body, file: src/typeguard/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.533 |
| walker |  | 6882 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 6904 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.533 |
| walker |  | 6928 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.533 |
| walker |  | 6954 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.533 |
| ns | 6979 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.560 |
| walker |  | 6980 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.560 |
| walker |  | 7113 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.560 |
| ns | 7156 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.552 |
| walker |  | 7253 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.552 |
| walker |  | 7396 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.552 |
| ns | 7523 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.539 |
| walker |  | 7554 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.539 |
| ns | 7599 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.543 |
| ns | 7730 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.543 |
| walker |  | 7829 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.543 |
| walker |  | 7844 | 15 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.543 |
| ns | 7876 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.551 |
| ns | 8163 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.541 |
| walker |  | 8260 | 416 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.542 |
| walker |  | 8267 | 7 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 47, sub: 0, line: 577 } |  |  | 0.542 |
| walker |  | 8276 | 9 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 46, sub: 0, line: 574 } |  |  | 0.542 |
| walker |  | 8305 | 29 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 913 } |  |  | 0.542 |
| ns | 8322 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.535 |
| walker |  | 8337 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.535 |
| walker |  | 8373 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.535 |
| walker |  | 8416 | 43 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.535 |
| walker |  | 8433 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 56, sub: 0, line: 918 } |  |  | 0.535 |
| walker |  | 8456 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 60, sub: 0, line: 1138 } |  |  | 0.535 |
| ns | 8554 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.531 |
| walker |  | 8637 | 181 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.531 |
| ns | 8729 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.539 |
| walker |  | 8799 | 162 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.539 |
| ns | 8961 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.550 |
| walker |  | 8997 | 198 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.550 |
| walker |  | 9010 | 13 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 15, sub: 0, line: 225 } |  |  | 0.550 |
| walker |  | 9039 | 29 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.551 |
| walker |  | 9059 | 20 | Code::CodeKey { rung: Body, file: src/typeguard/_decorators.py, decl: 2, sub: 0, line: 32 } |  |  | 0.551 |
| walker |  | 9089 | 30 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.554 |
| walker |  | 9132 | 43 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 61, sub: 0, line: 1181 } |  |  | 0.554 |
| walker |  | 9178 | 46 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 58, sub: 0, line: 994 } |  |  | 0.554 |
| walker |  | 9214 | 36 | Code::CodeKey { rung: Names, file: src/typeguard/_pytest_plugin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 9219 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.555 |
| walker |  | 9275 | 56 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 57, sub: 0, line: 945 } |  |  | 0.555 |
| walker |  | 9316 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.563 |
| walker |  | 9358 | 42 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 1, sub: 0, line: 5 } |  |  | 0.569 |
| walker |  | 9372 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.569 |
| walker |  | 9432 | 60 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 62, sub: 0, line: 1224 } |  |  | 0.573 |
| ns | 9432 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.573 |
| walker |  | 9493 | 61 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 59, sub: 0, line: 1036 } |  |  | 0.580 |
| walker |  | 9565 | 72 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.590 |
| walker |  | 9621 | 56 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 5, sub: 0, line: 154 } |  |  | 0.590 |
| ns | 9622 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.588 |
| walker |  | 9630 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 8, sub: 0, line: 35 } |  |  | 0.589 |
| walker |  | 9692 | 62 | Code::CodeKey { rung: Body, file: src/typeguard/_config.py, decl: 4, sub: 0, line: 52 } |  |  | 0.594 |
| walker |  | 9699 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.594 |
| walker |  | 9767 | 68 | Code::CodeKey { rung: Doc, file: src/typeguard/_functions.py, decl: 11, sub: 0, line: 291 } |  |  | 0.600 |
| ns | 9823 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.592 |
| walker |  | 9840 | 73 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 3, sub: 0, line: 127 } |  |  | 0.592 |
| ns | 9858 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.595 |
| ns | 9914 |  | 56 | CI interpreter matrix | 7.4 |  | 0.594 |
| walker |  | 9916 | 76 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 11, sub: 0, line: 138 } |  |  | 0.598 |
| walker |  | 9997 | 81 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 2, sub: 0, line: 104 } |  |  | 0.598 |
