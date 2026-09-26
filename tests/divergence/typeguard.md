Score(3000)=0.690 I=0.894 C=0.532 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.909/0.902/0.767/0.690/0.560/0.479/0.560

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 34 |  | 34 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 110 | 76 | Fs::DirListing { dir: src/typeguard } |  |  | 1.000 |
| ns | 140 |  | 106 | README identity paragraph | 1.2 |  | 0.816 |
| walker |  | 216 | 106 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 1.000 |
| ns | 216 |  | 76 | Package module roster: src/typeguard/ | 1.3 |  | 1.000 |
| walker |  | 257 | 41 | Fs::DirListing { dir: docs } |  |  | 1.000 |
| ns | 311 |  | 95 | README: the two principal checking modes | 1.4 |  | 0.863 |
| walker |  | 331 | 74 | Toml::Identity { file: pyproject.toml } |  |  | 0.863 |
| walker |  | 382 | 51 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.864 |
| walker |  | 409 | 27 | Fs::DirListing { dir: .github } |  |  | 0.864 |
| walker |  | 417 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.788 |
| ns | 417 |  | 106 | README: what instrumentation actually covers | 1.5 | 1.4 | 0.788 |
| ns | 550 |  | 133 | README: the two instrumentation entry points | 1.6 |  | 0.686 |
| ns | 746 |  | 196 | Public exports of typeguard/__init__.py, first half | 1.7 |  | 0.607 |
| walker |  | 773 | 356 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.890 |
| ns | 952 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.805 |
| ns | 1107 |  | 155 | Docs and test tree listings | 1.9 |  | 0.688 |
| walker |  | 1213 | 440 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.854 |
| walker |  | 1313 | 100 | Fs::DirListing { dir: tests } |  |  | 0.969 |
| walker |  | 1327 | 14 | Fs::DirListing { dir: tests/mypy } |  |  | 1.000 |
| ns | 1337 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.901 |
| ns | 1471 |  | 134 | docs/index.rst in full | 1.11 |  | 0.833 |
| walker |  | 1519 | 192 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.834 |
| walker |  | 1553 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 118 } |  |  | 0.834 |
| walker |  | 1590 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 149 } |  |  | 0.834 |
| walker |  | 1628 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 185 } |  |  | 0.834 |
| ns | 1634 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.795 |
| walker |  | 1666 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 216 } |  |  | 0.795 |
| walker |  | 1705 | 39 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 245 } |  |  | 0.795 |
| ns | 1750 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.772 |
| walker |  | 1820 | 115 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 50 } |  |  | 0.822 |
| walker |  | 1889 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.822 |
| walker |  | 1917 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.822 |
| walker |  | 1963 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.822 |
| walker |  | 2055 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.825 |
| ns | 2057 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.767 |
| walker |  | 2308 | 253 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.767 |
| walker |  | 2332 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 1, sub: 0, line: 81 } |  |  | 0.767 |
| walker |  | 2359 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 2, sub: 0, line: 84 } |  |  | 0.767 |
| walker |  | 2390 | 31 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 4, sub: 0, line: 89 } |  |  | 0.767 |
| walker |  | 2430 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 7, sub: 0, line: 153 } |  |  | 0.767 |
| ns | 2449 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.707 |
| walker |  | 2470 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 8, sub: 0, line: 212 } |  |  | 0.707 |
| walker |  | 2510 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 9, sub: 0, line: 247 } |  |  | 0.707 |
| walker |  | 2550 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 10, sub: 0, line: 305 } |  |  | 0.707 |
| walker |  | 2590 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 11, sub: 0, line: 324 } |  |  | 0.707 |
| ns | 2611 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.684 |
| walker |  | 2630 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 12, sub: 0, line: 343 } |  |  | 0.684 |
| walker |  | 2670 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 13, sub: 0, line: 365 } |  |  | 0.684 |
| walker |  | 2710 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 14, sub: 0, line: 423 } |  |  | 0.684 |
| ns | 2735 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.688 |
| ns | 3033 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.645 |
| walker |  | 3139 | 429 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 1, line: 0 } |  |  | 0.648 |
| walker |  | 3162 | 23 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 34, sub: 0, line: 1061 } |  |  | 0.648 |
| walker |  | 3189 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 32, sub: 0, line: 924 } |  |  | 0.648 |
| walker |  | 3229 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 15, sub: 0, line: 447 } |  |  | 0.648 |
| ns | 3262 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.622 |
| walker |  | 3269 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 16, sub: 0, line: 474 } |  |  | 0.622 |
| walker |  | 3309 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 17, sub: 0, line: 536 } |  |  | 0.622 |
| walker |  | 3349 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 18, sub: 0, line: 545 } |  |  | 0.622 |
| walker |  | 3389 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 21, sub: 0, line: 590 } |  |  | 0.622 |
| walker |  | 3429 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 22, sub: 0, line: 623 } |  |  | 0.622 |
| ns | 3430 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.605 |
| walker |  | 3469 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 23, sub: 0, line: 632 } |  |  | 0.605 |
| walker |  | 3509 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 24, sub: 0, line: 641 } |  |  | 0.605 |
| walker |  | 3549 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 25, sub: 0, line: 651 } |  |  | 0.605 |
| ns | 3557 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.596 |
| walker |  | 3589 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 26, sub: 0, line: 663 } |  |  | 0.596 |
| walker |  | 3629 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 28, sub: 0, line: 834 } |  |  | 0.596 |
| walker |  | 3669 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 29, sub: 0, line: 885 } |  |  | 0.596 |
| walker |  | 3709 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 30, sub: 0, line: 895 } |  |  | 0.596 |
| walker |  | 3749 | 40 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 31, sub: 0, line: 915 } |  |  | 0.596 |
| ns | 3763 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.576 |
| walker |  | 3808 | 59 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 19, sub: 0, line: 555 } |  |  | 0.576 |
| ns | 3956 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.560 |
| walker |  | 3960 | 152 | Code::CodeKey { rung: Names, file: src/typeguard/_decorators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 3981 | 21 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 4, sub: 0, line: 36 } |  |  | 0.560 |
| walker |  | 4075 | 94 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 6, sub: 0, line: 150 } |  |  | 0.580 |
| walker |  | 4092 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_decorators.py, decl: 2, sub: 0, line: 21 } |  |  | 0.580 |
| walker |  | 4100 | 8 | Code::CodeKey { rung: Body, file: src/typeguard/_decorators.py, decl: 2, sub: 0, line: 21 } |  |  | 0.580 |
| ns | 4102 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.568 |
| ns | 4212 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.560 |
| ns | 4447 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.542 |
| walker |  | 4594 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.542 |
| ns | 4610 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.529 |
| walker |  | 4792 | 198 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 0, line: 1005 } |  |  | 0.530 |
| ns | 4961 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.507 |
| walker |  | 5110 | 318 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 1, line: 1005 } |  |  | 0.509 |
| ns | 5220 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.493 |
| walker |  | 5374 | 264 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 5410 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 10, sub: 0, line: 172 } |  |  | 0.494 |
| walker |  | 5417 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 11, sub: 0, line: 176 } |  |  | 0.494 |
| walker |  | 5453 | 36 | Code::CodeKey { rung: Names, file: src/typeguard/_pytest_plugin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 5466 | 13 | Code::CodeKey { rung: Names, file: src/typeguard/_memo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 5512 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 1, sub: 0, line: 8 } |  |  | 0.498 |
| ns | 5573 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.480 |
| walker |  | 5575 | 63 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.492 |
| walker |  | 5628 | 53 | Code::CodeKey { rung: Names, file: src/typeguard/_exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 5645 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.494 |
| walker |  | 5662 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.495 |
| walker |  | 5715 | 53 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.497 |
| walker |  | 5724 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 3, sub: 0, line: 15 } |  |  | 0.499 |
| walker |  | 5733 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 5, sub: 0, line: 22 } |  |  | 0.500 |
| walker |  | 5742 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 8, sub: 0, line: 35 } |  |  | 0.501 |
| ns | 5756 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.494 |
| walker |  | 5881 | 139 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 5904 | 23 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.495 |
| walker |  | 5941 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.497 |
| walker |  | 5991 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.497 |
| ns | 6028 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.485 |
| walker |  | 6049 | 58 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.485 |
| walker |  | 6120 | 71 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.485 |
| walker |  | 6161 | 41 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.485 |
| ns | 6172 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.479 |
| walker |  | 6236 | 75 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.479 |
| walker |  | 6278 | 42 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.479 |
| walker |  | 6289 | 11 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.479 |
| walker |  | 6301 | 12 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 16, sub: 0, line: 175 } |  |  | 0.479 |
| walker |  | 6306 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.479 |
| walker |  | 6325 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.482 |
| walker |  | 6345 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.486 |
| ns | 6471 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.513 |
| walker |  | 6517 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 6539 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.513 |
| walker |  | 6563 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.513 |
| walker |  | 6589 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.513 |
| walker |  | 6615 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.513 |
| walker |  | 6748 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.513 |
| walker |  | 6888 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.513 |
| ns | 6979 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.542 |
| walker |  | 7031 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.542 |
| ns | 7156 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.534 |
| walker |  | 7189 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.534 |
| walker |  | 7370 | 181 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.535 |
| ns | 7523 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.521 |
| walker |  | 7532 | 162 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.521 |
| ns | 7599 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.523 |
| walker |  | 7730 | 198 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.522 |
| ns | 7730 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.522 |
| ns | 7876 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.530 |
| walker |  | 7914 | 184 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.531 |
| walker |  | 7931 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.531 |
| walker |  | 7956 | 25 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.531 |
| ns | 8163 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.521 |
| walker |  | 8216 | 260 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 1, line: 488 } |  |  | 0.522 |
| walker |  | 8227 | 11 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 53, sub: 0, line: 913 } |  |  | 0.522 |
| walker |  | 8241 | 14 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 52, sub: 0, line: 650 } |  |  | 0.522 |
| ns | 8322 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.515 |
| walker |  | 8516 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.515 |
| walker |  | 8529 | 13 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 15, sub: 0, line: 225 } |  |  | 0.515 |
| walker |  | 8544 | 15 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.515 |
| ns | 8554 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.527 |
| walker |  | 8561 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 918 } |  |  | 0.527 |
| walker |  | 8647 | 86 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 8662 | 15 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 30 } |  |  | 0.530 |
| walker |  | 8685 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 58, sub: 0, line: 1138 } |  |  | 0.530 |
| walker |  | 8692 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.530 |
| walker |  | 8721 | 29 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.532 |
| ns | 8729 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.540 |
| walker |  | 8751 | 30 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.543 |
| walker |  | 8794 | 43 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 59, sub: 0, line: 1181 } |  |  | 0.543 |
| walker |  | 8840 | 46 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 56, sub: 0, line: 994 } |  |  | 0.543 |
| walker |  | 8849 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 13, sub: 0, line: 161 } |  |  | 0.543 |
| walker |  | 8863 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.543 |
| walker |  | 8919 | 56 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 945 } |  |  | 0.543 |
| walker |  | 8939 | 20 | Code::CodeKey { rung: Body, file: src/typeguard/_decorators.py, decl: 3, sub: 0, line: 32 } |  |  | 0.543 |
| ns | 8961 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.554 |
| walker |  | 8981 | 42 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 1, sub: 0, line: 5 } |  |  | 0.560 |
| walker |  | 8992 | 11 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.560 |
| walker |  | 9052 | 60 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 60, sub: 0, line: 1224 } |  |  | 0.560 |
| walker |  | 9113 | 61 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 57, sub: 0, line: 1036 } |  |  | 0.561 |
| walker |  | 9138 | 25 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 7, sub: 0, line: 31 } |  |  | 0.564 |
| walker |  | 9210 | 72 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 52, sub: 0, line: 650 } |  |  | 0.564 |
| walker |  | 9251 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.572 |
| walker |  | 9270 | 19 | Code::CodeKey { rung: Body, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 291 } |  |  | 0.573 |
| walker |  | 9336 | 66 | Code::CodeKey { rung: Doc, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 291 } |  |  | 0.581 |
| walker |  | 9354 | 18 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 4, sub: 0, line: 51 } |  |  | 0.581 |
| walker |  | 9423 | 69 | Code::CodeKey { rung: Body, file: src/typeguard/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.586 |
| ns | 9432 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.604 |
| walker |  | 9496 | 73 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 6, sub: 0, line: 127 } |  |  | 0.604 |
| walker |  | 9572 | 76 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 11, sub: 0, line: 138 } |  |  | 0.607 |
| ns | 9622 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.606 |
| walker |  | 9653 | 81 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 5, sub: 0, line: 104 } |  |  | 0.606 |
| walker |  | 9704 | 51 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 9, sub: 0, line: 38 } |  |  | 0.612 |
| walker |  | 9813 | 109 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.624 |
| ns | 9823 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.616 |
| walker |  | 9851 | 38 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 2, sub: 0, line: 40 } |  |  | 0.616 |
| ns | 9858 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.619 |
| ns | 9914 |  | 56 | CI interpreter matrix | 7.4 |  | 0.618 |
| walker |  | 9964 | 113 | Code::CodeKey { rung: Doc, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.630 |
| walker |  | 9987 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_checkers.py, decl: 35, sub: 0, line: 1099 } |  |  | 0.630 |
