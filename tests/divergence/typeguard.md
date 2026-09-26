Score(3000)=0.691 I=0.899 C=0.532 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.883/0.901/0.764/0.691/0.541/0.479/0.546

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
| walker |  | 1514 | 150 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.834 |
| walker |  | 1560 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 118 } |  |  | 0.834 |
| walker |  | 1607 | 47 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 149 } |  |  | 0.834 |
| ns | 1634 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.794 |
| walker |  | 1655 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 185 } |  |  | 0.794 |
| walker |  | 1703 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 216 } |  |  | 0.794 |
| ns | 1750 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.771 |
| walker |  | 1752 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 10, sub: 0, line: 245 } |  |  | 0.771 |
| walker |  | 1841 | 89 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 39 } |  |  | 0.771 |
| walker |  | 1929 | 88 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 28 } |  |  | 0.771 |
| walker |  | 2054 | 125 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 50 } |  |  | 0.822 |
| ns | 2057 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.764 |
| walker |  | 2123 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.764 |
| walker |  | 2151 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.764 |
| walker |  | 2197 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.765 |
| walker |  | 2289 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.767 |
| ns | 2449 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.707 |
| ns | 2611 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.684 |
| walker |  | 2716 | 427 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| ns | 2735 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.691 |
| walker |  | 2740 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 1, sub: 0, line: 81 } |  |  | 0.691 |
| walker |  | 2767 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 2, sub: 0, line: 84 } |  |  | 0.691 |
| walker |  | 2798 | 31 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 4, sub: 0, line: 89 } |  |  | 0.691 |
| walker |  | 2835 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 32, sub: 0, line: 924 } |  |  | 0.691 |
| walker |  | 2873 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 34, sub: 0, line: 1061 } |  |  | 0.691 |
| walker |  | 2923 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 7, sub: 0, line: 153 } |  |  | 0.691 |
| walker |  | 2973 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 8, sub: 0, line: 212 } |  |  | 0.691 |
| walker |  | 3023 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 9, sub: 0, line: 247 } |  |  | 0.691 |
| ns | 3033 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.648 |
| walker |  | 3073 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 10, sub: 0, line: 305 } |  |  | 0.648 |
| walker |  | 3123 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 11, sub: 0, line: 324 } |  |  | 0.648 |
| walker |  | 3173 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 12, sub: 0, line: 343 } |  |  | 0.648 |
| walker |  | 3223 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 13, sub: 0, line: 365 } |  |  | 0.648 |
| ns | 3262 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.622 |
| walker |  | 3273 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 14, sub: 0, line: 423 } |  |  | 0.622 |
| walker |  | 3323 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 15, sub: 0, line: 447 } |  |  | 0.622 |
| walker |  | 3373 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 16, sub: 0, line: 474 } |  |  | 0.622 |
| walker |  | 3423 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 17, sub: 0, line: 536 } |  |  | 0.622 |
| ns | 3430 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.605 |
| walker |  | 3473 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 18, sub: 0, line: 545 } |  |  | 0.605 |
| walker |  | 3523 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 21, sub: 0, line: 590 } |  |  | 0.605 |
| ns | 3557 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.596 |
| walker |  | 3573 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 22, sub: 0, line: 623 } |  |  | 0.596 |
| walker |  | 3623 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 23, sub: 0, line: 632 } |  |  | 0.596 |
| walker |  | 3673 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 24, sub: 0, line: 641 } |  |  | 0.596 |
| walker |  | 3723 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 25, sub: 0, line: 651 } |  |  | 0.596 |
| ns | 3763 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.576 |
| walker |  | 3773 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 26, sub: 0, line: 663 } |  |  | 0.576 |
| walker |  | 3823 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 28, sub: 0, line: 834 } |  |  | 0.576 |
| walker |  | 3873 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 29, sub: 0, line: 885 } |  |  | 0.576 |
| walker |  | 3923 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 30, sub: 0, line: 895 } |  |  | 0.576 |
| ns | 3956 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.560 |
| walker |  | 3973 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 31, sub: 0, line: 915 } |  |  | 0.560 |
| walker |  | 4042 | 69 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 19, sub: 0, line: 555 } |  |  | 0.560 |
| ns | 4102 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.548 |
| ns | 4212 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.541 |
| ns | 4447 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.523 |
| walker |  | 4536 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.523 |
| ns | 4610 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.511 |
| ns | 4961 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.489 |
| walker |  | 5052 | 516 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 0, line: 1005 } |  |  | 0.492 |
| walker |  | 5184 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_decorators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 5192 | 8 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 6, sub: 0, line: 146 } |  |  | 0.492 |
| ns | 5220 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.477 |
| walker |  | 5226 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 3, sub: 0, line: 36 } |  |  | 0.477 |
| walker |  | 5330 | 104 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 7, sub: 0, line: 150 } |  |  | 0.494 |
| walker |  | 5436 | 106 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 5, sub: 0, line: 136 } |  |  | 0.494 |
| walker |  | 5568 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| ns | 5573 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.476 |
| walker |  | 5604 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 7, sub: 0, line: 172 } |  |  | 0.476 |
| walker |  | 5611 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 8, sub: 0, line: 176 } |  |  | 0.476 |
| walker |  | 5647 | 36 | Code::CodeKey { rung: Names, file: src/typeguard/_pytest_plugin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 5660 | 13 | Code::CodeKey { rung: Names, file: src/typeguard/_memo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 5698 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 1, sub: 0, line: 8 } |  |  | 0.479 |
| ns | 5756 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.472 |
| walker |  | 5769 | 71 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.484 |
| walker |  | 5822 | 53 | Code::CodeKey { rung: Names, file: src/typeguard/_exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 5839 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.486 |
| walker |  | 5856 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.487 |
| walker |  | 5909 | 53 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.490 |
| walker |  | 5918 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 3, sub: 0, line: 15 } |  |  | 0.491 |
| walker |  | 5927 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 5, sub: 0, line: 22 } |  |  | 0.492 |
| walker |  | 5936 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 8, sub: 0, line: 35 } |  |  | 0.493 |
| ns | 6028 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.482 |
| walker |  | 6053 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 6085 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.482 |
| walker |  | 6118 | 33 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.482 |
| walker |  | 6167 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.484 |
| ns | 6172 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.478 |
| walker |  | 6227 | 60 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.479 |
| walker |  | 6279 | 52 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.479 |
| walker |  | 6340 | 61 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.479 |
| walker |  | 6396 | 56 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.479 |
| ns | 6471 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.506 |
| walker |  | 6472 | 76 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.506 |
| walker |  | 6484 | 12 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 16, sub: 0, line: 175 } |  |  | 0.506 |
| walker |  | 6489 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.506 |
| walker |  | 6508 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.509 |
| walker |  | 6528 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.512 |
| walker |  | 6535 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.512 |
| walker |  | 6707 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 6729 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.513 |
| walker |  | 6753 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.513 |
| walker |  | 6779 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.513 |
| walker |  | 6805 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.513 |
| walker |  | 6938 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.513 |
| ns | 6979 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.542 |
| walker |  | 7078 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.542 |
| ns | 7156 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.534 |
| walker |  | 7221 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.534 |
| walker |  | 7379 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.534 |
| ns | 7523 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.521 |
| ns | 7599 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.526 |
| ns | 7730 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.526 |
| walker |  | 7795 | 416 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.527 |
| walker |  | 7802 | 7 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 47, sub: 0, line: 577 } |  |  | 0.527 |
| walker |  | 7811 | 9 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 46, sub: 0, line: 574 } |  |  | 0.527 |
| walker |  | 7840 | 29 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 913 } |  |  | 0.527 |
| walker |  | 7872 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.527 |
| ns | 7876 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.535 |
| walker |  | 7908 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.535 |
| walker |  | 7951 | 43 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.535 |
| walker |  | 8132 | 181 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.535 |
| ns | 8163 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.525 |
| walker |  | 8294 | 162 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.525 |
| ns | 8322 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.519 |
| walker |  | 8492 | 198 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.519 |
| ns | 8554 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.515 |
| ns | 8729 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.524 |
| walker |  | 8767 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.524 |
| walker |  | 8780 | 13 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 15, sub: 0, line: 225 } |  |  | 0.524 |
| walker |  | 8795 | 15 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.524 |
| walker |  | 8812 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 56, sub: 0, line: 918 } |  |  | 0.524 |
| walker |  | 8835 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 60, sub: 0, line: 1138 } |  |  | 0.524 |
| walker |  | 8952 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 8958 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 22 } |  |  | 0.530 |
| ns | 8961 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.541 |
| walker |  | 8964 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 6, sub: 0, line: 26 } |  |  | 0.542 |
| walker |  | 8998 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 7, sub: 0, line: 30 } |  |  | 0.546 |
| walker |  | 9027 | 29 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.548 |
| walker |  | 9057 | 30 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.551 |
| walker |  | 9100 | 43 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 61, sub: 0, line: 1181 } |  |  | 0.551 |
| walker |  | 9146 | 46 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 58, sub: 0, line: 994 } |  |  | 0.551 |
| walker |  | 9155 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 13, sub: 0, line: 161 } |  |  | 0.551 |
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
