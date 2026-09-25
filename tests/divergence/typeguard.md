Score(3000)=0.665 I=0.897 C=0.493 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.691/0.759/0.734/0.665/0.583/0.484/0.520

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 34 |  | 34 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | Fs::DirListing { dir: src } |  |  | 1.000 |
| walker |  | 79 | 41 | Fs::DirListing { dir: docs } |  |  | 1.000 |
| ns | 140 |  | 106 | README identity paragraph | 1.2 |  | 0.771 |
| walker |  | 153 | 74 | Toml::Identity { file: pyproject.toml } |  |  | 0.772 |
| ns | 218 |  | 78 | Package module roster: src/typeguard/ | 1.3 |  | 0.510 |
| walker |  | 227 | 74 | Fs::DirListing { dir: src/typeguard } |  |  | 0.868 |
| walker |  | 278 | 51 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.868 |
| ns | 313 |  | 95 | README: the two principal checking modes | 1.4 |  | 0.746 |
| walker |  | 384 | 106 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 0.864 |
| walker |  | 419 | 35 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.788 |
| ns | 419 |  | 106 | README: what instrumentation actually covers | 1.5 | 1.4 | 0.788 |
| walker |  | 446 | 27 | Fs::DirListing { dir: .github } |  |  | 0.788 |
| walker |  | 454 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.789 |
| ns | 552 |  | 133 | README: the two instrumentation entry points | 1.6 |  | 0.686 |
| walker |  | 681 | 227 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.703 |
| walker |  | 694 | 13 | Code::CodeKey { rung: Names, file: src/typeguard/_memo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.703 |
| walker |  | 732 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 1, sub: 0, line: 8 } |  |  | 0.703 |
| ns | 748 |  | 196 | Public exports of typeguard/__init__.py, first half | 1.7 |  | 0.740 |
| walker |  | 832 | 100 | Fs::DirListing { dir: tests } |  |  | 0.759 |
| walker |  | 846 | 14 | Fs::DirListing { dir: tests/mypy } |  |  | 0.764 |
| ns | 954 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.691 |
| walker |  | 1073 | 227 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.800 |
| ns | 1109 |  | 155 | Docs and test tree listings | 1.9 |  | 0.823 |
| walker |  | 1142 | 69 | Code::CodeKey { rung: Body, file: src/typeguard/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.825 |
| walker |  | 1178 | 36 | Code::CodeKey { rung: Names, file: src/typeguard/_pytest_plugin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.825 |
| ns | 1339 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.759 |
| ns | 1473 |  | 134 | docs/index.rst in full | 1.11 |  | 0.701 |
| walker |  | 1557 | 379 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.850 |
| walker |  | 1628 | 71 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.851 |
| ns | 1636 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.809 |
| walker |  | 1681 | 53 | Code::CodeKey { rung: Names, file: src/typeguard/_exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| walker |  | 1698 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.809 |
| walker |  | 1715 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.809 |
| ns | 1752 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.786 |
| walker |  | 1768 | 53 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.786 |
| walker |  | 1777 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 3, sub: 0, line: 15 } |  |  | 0.786 |
| walker |  | 1786 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 5, sub: 0, line: 22 } |  |  | 0.786 |
| walker |  | 1795 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 8, sub: 0, line: 35 } |  |  | 0.787 |
| walker |  | 1864 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| walker |  | 1892 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.787 |
| walker |  | 1938 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.787 |
| walker |  | 2030 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.790 |
| walker |  | 2049 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.790 |
| ns | 2059 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.734 |
| walker |  | 2166 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.735 |
| walker |  | 2198 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.735 |
| walker |  | 2247 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.735 |
| walker |  | 2307 | 60 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.735 |
| walker |  | 2312 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.735 |
| walker |  | 2373 | 61 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.736 |
| walker |  | 2425 | 52 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.736 |
| ns | 2451 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.678 |
| walker |  | 2481 | 56 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.678 |
| walker |  | 2514 | 33 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.678 |
| walker |  | 2590 | 76 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.678 |
| ns | 2613 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.656 |
| walker |  | 2707 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 2713 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 22 } |  |  | 0.657 |
| walker |  | 2719 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 6, sub: 0, line: 26 } |  |  | 0.657 |
| ns | 2737 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.663 |
| walker |  | 2753 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 7, sub: 0, line: 30 } |  |  | 0.664 |
| walker |  | 2885 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_decorators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 2893 | 8 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 6, sub: 0, line: 146 } |  |  | 0.665 |
| walker |  | 2927 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 3, sub: 0, line: 36 } |  |  | 0.665 |
| walker |  | 3031 | 104 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 7, sub: 0, line: 150 } |  |  | 0.690 |
| ns | 3035 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.647 |
| walker |  | 3137 | 106 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 5, sub: 0, line: 136 } |  |  | 0.647 |
| walker |  | 3157 | 20 | Code::CodeKey { rung: Body, file: src/typeguard/_decorators.py, decl: 2, sub: 0, line: 32 } |  |  | 0.647 |
| ns | 3264 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.623 |
| walker |  | 3289 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 3325 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 7, sub: 0, line: 172 } |  |  | 0.624 |
| walker |  | 3332 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 8, sub: 0, line: 176 } |  |  | 0.624 |
| walker |  | 3388 | 56 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 5, sub: 0, line: 154 } |  |  | 0.624 |
| ns | 3432 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.608 |
| walker |  | 3461 | 73 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 3, sub: 0, line: 127 } |  |  | 0.608 |
| ns | 3559 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.600 |
| ns | 3765 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.591 |
| walker |  | 3888 | 427 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 3912 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 1, sub: 0, line: 81 } |  |  | 0.593 |
| walker |  | 3939 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 2, sub: 0, line: 84 } |  |  | 0.593 |
| ns | 3958 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.581 |
| walker |  | 3970 | 31 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 4, sub: 0, line: 89 } |  |  | 0.581 |
| walker |  | 4007 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 32, sub: 0, line: 924 } |  |  | 0.581 |
| walker |  | 4045 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 34, sub: 0, line: 1061 } |  |  | 0.581 |
| walker |  | 4095 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 7, sub: 0, line: 153 } |  |  | 0.581 |
| ns | 4104 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.591 |
| walker |  | 4145 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 8, sub: 0, line: 212 } |  |  | 0.591 |
| walker |  | 4195 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 9, sub: 0, line: 247 } |  |  | 0.591 |
| ns | 4214 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.583 |
| walker |  | 4245 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 10, sub: 0, line: 305 } |  |  | 0.583 |
| walker |  | 4295 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 11, sub: 0, line: 324 } |  |  | 0.583 |
| walker |  | 4345 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 12, sub: 0, line: 343 } |  |  | 0.583 |
| walker |  | 4395 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 13, sub: 0, line: 365 } |  |  | 0.583 |
| walker |  | 4445 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 14, sub: 0, line: 423 } |  |  | 0.583 |
| ns | 4449 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.564 |
| walker |  | 4495 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 15, sub: 0, line: 447 } |  |  | 0.564 |
| walker |  | 4545 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 16, sub: 0, line: 474 } |  |  | 0.564 |
| walker |  | 4595 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 17, sub: 0, line: 536 } |  |  | 0.564 |
| ns | 4612 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.565 |
| walker |  | 4645 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 18, sub: 0, line: 545 } |  |  | 0.565 |
| walker |  | 4695 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 21, sub: 0, line: 590 } |  |  | 0.565 |
| walker |  | 4745 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 22, sub: 0, line: 623 } |  |  | 0.565 |
| walker |  | 4795 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 23, sub: 0, line: 632 } |  |  | 0.565 |
| walker |  | 4845 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 24, sub: 0, line: 641 } |  |  | 0.565 |
| walker |  | 4895 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 25, sub: 0, line: 651 } |  |  | 0.565 |
| walker |  | 4945 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 26, sub: 0, line: 663 } |  |  | 0.565 |
| ns | 4963 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.540 |
| walker |  | 4995 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 28, sub: 0, line: 834 } |  |  | 0.540 |
| walker |  | 5045 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 29, sub: 0, line: 885 } |  |  | 0.540 |
| walker |  | 5095 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 30, sub: 0, line: 895 } |  |  | 0.540 |
| walker |  | 5145 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 31, sub: 0, line: 915 } |  |  | 0.540 |
| walker |  | 5214 | 69 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 19, sub: 0, line: 555 } |  |  | 0.540 |
| ns | 5222 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.523 |
| walker |  | 5234 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.527 |
| walker |  | 5406 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 5428 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.528 |
| walker |  | 5452 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.528 |
| walker |  | 5478 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.528 |
| walker |  | 5504 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.528 |
| ns | 5575 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.509 |
| walker |  | 5637 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.509 |
| ns | 5758 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.502 |
| walker |  | 5777 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.502 |
| walker |  | 5920 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.502 |
| ns | 6030 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.490 |
| walker |  | 6078 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.490 |
| ns | 6174 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.484 |
| walker |  | 6353 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.484 |
| walker |  | 6358 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_transformer.py, decl: 24, sub: 0, line: 306 } |  |  | 0.484 |
| ns | 6473 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.510 |
| walker |  | 6774 | 416 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.511 |
| walker |  | 6781 | 7 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 47, sub: 0, line: 577 } |  |  | 0.511 |
| walker |  | 6810 | 29 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 913 } |  |  | 0.511 |
| walker |  | 6819 | 9 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 46, sub: 0, line: 574 } |  |  | 0.511 |
| walker |  | 6851 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.511 |
| walker |  | 6887 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.511 |
| walker |  | 6930 | 43 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.511 |
| ns | 6981 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.492 |
| walker |  | 7151 | 221 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.492 |
| ns | 7158 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.485 |
| walker |  | 7358 | 207 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.485 |
| walker |  | 7399 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.495 |
| walker |  | 7413 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.495 |
| ns | 7525 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.483 |
| walker |  | 7563 | 150 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| ns | 7601 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.490 |
| walker |  | 7609 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 118 } |  |  | 0.490 |
| walker |  | 7656 | 47 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 149 } |  |  | 0.490 |
| walker |  | 7704 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 185 } |  |  | 0.490 |
| ns | 7732 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.491 |
| walker |  | 7752 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 216 } |  |  | 0.491 |
| walker |  | 7801 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 10, sub: 0, line: 245 } |  |  | 0.491 |
| ns | 7878 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.500 |
| walker |  | 7890 | 89 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 39 } |  |  | 0.500 |
| walker |  | 7978 | 88 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 28 } |  |  | 0.500 |
| walker |  | 8103 | 125 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 50 } |  |  | 0.518 |
| ns | 8165 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.509 |
| ns | 8324 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.502 |
| ns | 8556 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.499 |
| walker |  | 8597 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.499 |
| walker |  | 8604 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.499 |
| ns | 8731 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.508 |
| ns | 8963 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.520 |
| walker |  | 9120 | 516 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 0, line: 1005 } |  |  | 0.559 |
| walker |  | 9139 | 19 | Code::CodeKey { rung: Body, file: src/typeguard/_functions.py, decl: 11, sub: 0, line: 291 } |  |  | 0.560 |
| walker |  | 9241 | 102 | Plaintext::DeclSurface { file: docs/api.rst } |  |  | 0.560 |
| walker |  | 9354 | 113 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.560 |
| ns | 9434 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.546 |
| ns | 9624 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.548 |
| ns | 9825 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.541 |
| ns | 9860 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.545 |
| ns | 9916 |  | 56 | CI interpreter matrix | 7.4 |  | 0.544 |
