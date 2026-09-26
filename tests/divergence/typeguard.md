Score(3000)=0.619 I=0.875 C=0.438 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.883/0.901/0.718/0.619/0.549/0.488/0.548

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
| walker |  | 3406 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 8, sub: 0, line: 35 } |  |  | 0.556 |
| ns | 3430 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.542 |
| walker |  | 3556 | 150 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| ns | 3557 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.535 |
| walker |  | 3602 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 118 } |  |  | 0.535 |
| walker |  | 3649 | 47 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 149 } |  |  | 0.535 |
| walker |  | 3697 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 185 } |  |  | 0.535 |
| walker |  | 3745 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 216 } |  |  | 0.535 |
| ns | 3763 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.525 |
| walker |  | 3794 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 10, sub: 0, line: 245 } |  |  | 0.525 |
| walker |  | 3883 | 89 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 39 } |  |  | 0.525 |
| ns | 3956 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.515 |
| walker |  | 3971 | 88 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 28 } |  |  | 0.515 |
| walker |  | 4096 | 125 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 50 } |  |  | 0.547 |
| ns | 4102 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.536 |
| walker |  | 4165 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 4193 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.539 |
| ns | 4212 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.532 |
| walker |  | 4239 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.534 |
| walker |  | 4331 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.554 |
| ns | 4447 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.535 |
| walker |  | 4448 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 4480 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.536 |
| walker |  | 4513 | 33 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.536 |
| walker |  | 4562 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.539 |
| ns | 4610 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.541 |
| walker |  | 4622 | 60 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.542 |
| walker |  | 4674 | 52 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.542 |
| walker |  | 4735 | 61 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.542 |
| walker |  | 4791 | 56 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.542 |
| walker |  | 4867 | 76 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.542 |
| walker |  | 4879 | 12 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 16, sub: 0, line: 175 } |  |  | 0.542 |
| walker |  | 4884 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.542 |
| walker |  | 4903 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.545 |
| walker |  | 4923 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.549 |
| ns | 4961 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.525 |
| walker |  | 5055 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 5091 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 7, sub: 0, line: 172 } |  |  | 0.526 |
| walker |  | 5098 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 8, sub: 0, line: 176 } |  |  | 0.526 |
| walker |  | 5215 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| ns | 5220 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.518 |
| walker |  | 5221 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 22 } |  |  | 0.520 |
| walker |  | 5227 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 6, sub: 0, line: 26 } |  |  | 0.521 |
| walker |  | 5261 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 7, sub: 0, line: 30 } |  |  | 0.529 |
| ns | 5573 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.511 |
| walker |  | 5755 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.511 |
| ns | 5756 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.503 |
| ns | 6028 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.491 |
| ns | 6172 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.485 |
| walker |  | 6271 | 516 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 0, line: 1005 } |  |  | 0.488 |
| walker |  | 6403 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_decorators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 6411 | 8 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 6, sub: 0, line: 146 } |  |  | 0.488 |
| walker |  | 6445 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 3, sub: 0, line: 36 } |  |  | 0.488 |
| ns | 6471 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.514 |
| walker |  | 6549 | 104 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 7, sub: 0, line: 150 } |  |  | 0.528 |
| walker |  | 6655 | 106 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 5, sub: 0, line: 136 } |  |  | 0.528 |
| walker |  | 6662 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.528 |
| walker |  | 6834 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 6856 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.528 |
| walker |  | 6880 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.528 |
| walker |  | 6906 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.528 |
| walker |  | 6932 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.528 |
| ns | 6979 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.556 |
| walker |  | 7065 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.556 |
| ns | 7156 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.547 |
| walker |  | 7205 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.547 |
| walker |  | 7348 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.547 |
| walker |  | 7506 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.547 |
| ns | 7523 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.534 |
| ns | 7599 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.539 |
| ns | 7730 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.539 |
| ns | 7876 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.546 |
| walker |  | 7922 | 416 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.548 |
| walker |  | 7929 | 7 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 47, sub: 0, line: 577 } |  |  | 0.548 |
| walker |  | 7938 | 9 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 46, sub: 0, line: 574 } |  |  | 0.548 |
| walker |  | 7967 | 29 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 913 } |  |  | 0.548 |
| walker |  | 7999 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.548 |
| walker |  | 8035 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.548 |
| walker |  | 8078 | 43 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.548 |
| ns | 8163 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.538 |
| walker |  | 8259 | 181 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.538 |
| ns | 8322 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.531 |
| walker |  | 8421 | 162 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.531 |
| ns | 8554 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.527 |
| walker |  | 8619 | 198 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.527 |
| ns | 8729 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.535 |
| walker |  | 8894 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.535 |
| walker |  | 8907 | 13 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 15, sub: 0, line: 225 } |  |  | 0.535 |
| walker |  | 8922 | 15 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.535 |
| walker |  | 8939 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 56, sub: 0, line: 918 } |  |  | 0.535 |
| ns | 8961 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.546 |
| walker |  | 8962 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 60, sub: 0, line: 1138 } |  |  | 0.546 |
| walker |  | 8991 | 29 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.547 |
| walker |  | 9021 | 30 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.550 |
| walker |  | 9064 | 43 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 61, sub: 0, line: 1181 } |  |  | 0.550 |
| walker |  | 9110 | 46 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 58, sub: 0, line: 994 } |  |  | 0.551 |
| walker |  | 9119 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 13, sub: 0, line: 161 } |  |  | 0.551 |
| walker |  | 9155 | 36 | Code::CodeKey { rung: Names, file: src/typeguard/_pytest_plugin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 9211 | 56 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 57, sub: 0, line: 945 } |  |  | 0.551 |
| walker |  | 9222 | 11 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.551 |
| walker |  | 9264 | 42 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 1, sub: 0, line: 5 } |  |  | 0.558 |
| walker |  | 9278 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.558 |
| walker |  | 9338 | 60 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 62, sub: 0, line: 1224 } |  |  | 0.558 |
| walker |  | 9399 | 61 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 59, sub: 0, line: 1036 } |  |  | 0.559 |
| walker |  | 9424 | 25 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 7, sub: 0, line: 31 } |  |  | 0.561 |
| ns | 9432 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.572 |
| walker |  | 9444 | 20 | Code::CodeKey { rung: Body, file: src/typeguard/_decorators.py, decl: 2, sub: 0, line: 32 } |  |  | 0.572 |
| walker |  | 9516 | 72 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.582 |
| walker |  | 9557 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.589 |
| ns | 9622 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.587 |
| walker |  | 9625 | 68 | Code::CodeKey { rung: Doc, file: src/typeguard/_functions.py, decl: 11, sub: 0, line: 291 } |  |  | 0.593 |
| walker |  | 9643 | 18 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 4, sub: 0, line: 51 } |  |  | 0.593 |
| walker |  | 9716 | 73 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 3, sub: 0, line: 127 } |  |  | 0.593 |
| walker |  | 9785 | 69 | Code::CodeKey { rung: Body, file: src/typeguard/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.597 |
| ns | 9823 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.590 |
| ns | 9858 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.593 |
| walker |  | 9861 | 76 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 11, sub: 0, line: 138 } |  |  | 0.596 |
| ns | 9914 |  | 56 | CI interpreter matrix | 7.4 |  | 0.596 |
| walker |  | 9942 | 81 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 2, sub: 0, line: 104 } |  |  | 0.596 |
| walker |  | 9959 | 17 | Code::CodeKey { rung: Body, file: src/typeguard/_functions.py, decl: 11, sub: 0, line: 291 } |  |  | 0.598 |
| walker |  | 10000 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 9, sub: 0, line: 38 } |  |  | 0.603 |
