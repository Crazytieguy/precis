Score(3000)=0.669 I=0.908 C=0.492 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.672/0.760/0.733/0.669/0.619/0.520/0.543

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
| walker |  | 803 | 71 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.742 |
| ns | 954 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.672 |
| walker |  | 1030 | 227 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.778 |
| walker |  | 1099 | 69 | Code::CodeKey { rung: Body, file: src/typeguard/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.781 |
| ns | 1109 |  | 155 | Docs and test tree listings | 1.9 |  | 0.668 |
| walker |  | 1199 | 100 | Fs::DirListing { dir: tests } |  |  | 0.792 |
| walker |  | 1213 | 14 | Fs::DirListing { dir: tests/mypy } |  |  | 0.827 |
| walker |  | 1249 | 36 | Code::CodeKey { rung: Names, file: src/typeguard/_pytest_plugin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.827 |
| ns | 1339 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.760 |
| ns | 1473 |  | 134 | docs/index.rst in full | 1.11 |  | 0.702 |
| walker |  | 1628 | 379 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.851 |
| ns | 1636 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.809 |
| walker |  | 1681 | 53 | Code::CodeKey { rung: Names, file: src/typeguard/_exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| walker |  | 1698 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.809 |
| walker |  | 1715 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.809 |
| ns | 1752 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.786 |
| walker |  | 1768 | 53 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.786 |
| walker |  | 1777 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 3, sub: 0, line: 15 } |  |  | 0.786 |
| walker |  | 1786 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 5, sub: 0, line: 22 } |  |  | 0.786 |
| walker |  | 1805 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.787 |
| walker |  | 1825 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.787 |
| walker |  | 1855 | 30 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.788 |
| walker |  | 1924 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| walker |  | 1952 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.788 |
| walker |  | 1998 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.788 |
| ns | 2059 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.733 |
| walker |  | 2090 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.735 |
| walker |  | 2132 | 42 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 1, sub: 0, line: 5 } |  |  | 0.736 |
| walker |  | 2173 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.738 |
| walker |  | 2182 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 8, sub: 0, line: 35 } |  |  | 0.738 |
| walker |  | 2299 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 2331 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.738 |
| walker |  | 2380 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.738 |
| walker |  | 2440 | 60 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.739 |
| walker |  | 2445 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.739 |
| ns | 2451 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.681 |
| walker |  | 2497 | 52 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.681 |
| walker |  | 2558 | 61 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.681 |
| ns | 2613 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.659 |
| walker |  | 2614 | 56 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.659 |
| walker |  | 2626 | 12 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 16, sub: 0, line: 175 } |  |  | 0.659 |
| walker |  | 2702 | 76 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.659 |
| walker |  | 2735 | 33 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.659 |
| ns | 2737 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.665 |
| walker |  | 2764 | 29 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.666 |
| walker |  | 2881 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 2887 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 22 } |  |  | 0.667 |
| walker |  | 2893 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 6, sub: 0, line: 26 } |  |  | 0.667 |
| walker |  | 2927 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 7, sub: 0, line: 30 } |  |  | 0.667 |
| walker |  | 2989 | 62 | Code::CodeKey { rung: Body, file: src/typeguard/_config.py, decl: 4, sub: 0, line: 52 } |  |  | 0.669 |
| walker |  | 2996 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.669 |
| ns | 3035 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.640 |
| walker |  | 3128 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_decorators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 3136 | 8 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 6, sub: 0, line: 146 } |  |  | 0.640 |
| walker |  | 3170 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 3, sub: 0, line: 36 } |  |  | 0.640 |
| ns | 3264 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.617 |
| walker |  | 3274 | 104 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 7, sub: 0, line: 150 } |  |  | 0.639 |
| walker |  | 3380 | 106 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 5, sub: 0, line: 136 } |  |  | 0.639 |
| walker |  | 3400 | 20 | Code::CodeKey { rung: Body, file: src/typeguard/_decorators.py, decl: 2, sub: 0, line: 32 } |  |  | 0.639 |
| ns | 3432 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.626 |
| walker |  | 3532 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| ns | 3559 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.618 |
| walker |  | 3568 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 7, sub: 0, line: 172 } |  |  | 0.618 |
| walker |  | 3575 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 8, sub: 0, line: 176 } |  |  | 0.618 |
| walker |  | 3631 | 56 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 5, sub: 0, line: 154 } |  |  | 0.618 |
| walker |  | 3704 | 73 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 3, sub: 0, line: 127 } |  |  | 0.618 |
| ns | 3765 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.625 |
| ns | 3958 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.616 |
| ns | 4104 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.625 |
| walker |  | 4131 | 427 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 4155 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 1, sub: 0, line: 81 } |  |  | 0.628 |
| walker |  | 4182 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 2, sub: 0, line: 84 } |  |  | 0.628 |
| walker |  | 4213 | 31 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 4, sub: 0, line: 89 } |  |  | 0.628 |
| ns | 4214 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.619 |
| walker |  | 4250 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 32, sub: 0, line: 924 } |  |  | 0.619 |
| walker |  | 4288 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 34, sub: 0, line: 1061 } |  |  | 0.619 |
| walker |  | 4338 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 7, sub: 0, line: 153 } |  |  | 0.619 |
| walker |  | 4388 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 8, sub: 0, line: 212 } |  |  | 0.619 |
| walker |  | 4438 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 9, sub: 0, line: 247 } |  |  | 0.619 |
| ns | 4449 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.599 |
| walker |  | 4488 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 10, sub: 0, line: 305 } |  |  | 0.599 |
| walker |  | 4538 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 11, sub: 0, line: 324 } |  |  | 0.599 |
| walker |  | 4588 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 12, sub: 0, line: 343 } |  |  | 0.599 |
| ns | 4612 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.611 |
| walker |  | 4638 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 13, sub: 0, line: 365 } |  |  | 0.611 |
| walker |  | 4688 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 14, sub: 0, line: 423 } |  |  | 0.611 |
| walker |  | 4738 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 15, sub: 0, line: 447 } |  |  | 0.611 |
| walker |  | 4788 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 16, sub: 0, line: 474 } |  |  | 0.611 |
| walker |  | 4838 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 17, sub: 0, line: 536 } |  |  | 0.611 |
| walker |  | 4888 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 18, sub: 0, line: 545 } |  |  | 0.611 |
| walker |  | 4938 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 21, sub: 0, line: 590 } |  |  | 0.611 |
| ns | 4963 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.584 |
| walker |  | 4988 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 22, sub: 0, line: 623 } |  |  | 0.584 |
| walker |  | 5038 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 23, sub: 0, line: 632 } |  |  | 0.584 |
| walker |  | 5088 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 24, sub: 0, line: 641 } |  |  | 0.584 |
| walker |  | 5138 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 25, sub: 0, line: 651 } |  |  | 0.584 |
| walker |  | 5188 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 26, sub: 0, line: 663 } |  |  | 0.584 |
| ns | 5222 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.566 |
| walker |  | 5238 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 28, sub: 0, line: 834 } |  |  | 0.566 |
| walker |  | 5288 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 29, sub: 0, line: 885 } |  |  | 0.566 |
| walker |  | 5338 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 30, sub: 0, line: 895 } |  |  | 0.566 |
| walker |  | 5388 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 31, sub: 0, line: 915 } |  |  | 0.566 |
| walker |  | 5457 | 69 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 19, sub: 0, line: 555 } |  |  | 0.566 |
| walker |  | 5533 | 76 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 11, sub: 0, line: 138 } |  |  | 0.566 |
| ns | 5575 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.547 |
| walker |  | 5705 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 5727 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.547 |
| walker |  | 5751 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.547 |
| ns | 5758 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.539 |
| walker |  | 5777 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.539 |
| walker |  | 5803 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.539 |
| walker |  | 5936 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.539 |
| ns | 6030 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.526 |
| walker |  | 6076 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.526 |
| walker |  | 6081 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_transformer.py, decl: 24, sub: 0, line: 306 } |  |  | 0.526 |
| walker |  | 6086 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_transformer.py, decl: 25, sub: 0, line: 309 } |  |  | 0.526 |
| ns | 6174 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.520 |
| walker |  | 6229 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.520 |
| walker |  | 6244 | 15 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.520 |
| walker |  | 6402 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.520 |
| ns | 6473 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.543 |
| walker |  | 6677 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.543 |
| walker |  | 6682 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_transformer.py, decl: 29, sub: 0, line: 325 } |  |  | 0.543 |
| ns | 6981 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.523 |
| walker |  | 7098 | 416 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.524 |
| walker |  | 7105 | 7 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 47, sub: 0, line: 577 } |  |  | 0.524 |
| walker |  | 7134 | 29 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 913 } |  |  | 0.524 |
| walker |  | 7143 | 9 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 46, sub: 0, line: 574 } |  |  | 0.524 |
| ns | 7158 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.516 |
| walker |  | 7175 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.516 |
| walker |  | 7211 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.516 |
| walker |  | 7228 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 56, sub: 0, line: 918 } |  |  | 0.516 |
| walker |  | 7251 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 60, sub: 0, line: 1138 } |  |  | 0.516 |
| walker |  | 7294 | 43 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.516 |
| walker |  | 7337 | 43 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 61, sub: 0, line: 1181 } |  |  | 0.517 |
| walker |  | 7383 | 46 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 58, sub: 0, line: 994 } |  |  | 0.517 |
| walker |  | 7439 | 56 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 57, sub: 0, line: 945 } |  |  | 0.517 |
| walker |  | 7499 | 60 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 62, sub: 0, line: 1224 } |  |  | 0.517 |
| ns | 7525 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.505 |
| walker |  | 7560 | 61 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 59, sub: 0, line: 1036 } |  |  | 0.505 |
| ns | 7601 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.502 |
| walker |  | 7632 | 72 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.503 |
| ns | 7732 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.503 |
| walker |  | 7853 | 221 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.503 |
| ns | 7878 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.512 |
| walker |  | 8060 | 207 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.512 |
| ns | 8165 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.503 |
| walker |  | 8173 | 113 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.503 |
| walker |  | 8186 | 13 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 15, sub: 0, line: 225 } |  |  | 0.503 |
| walker |  | 8267 | 81 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 2, sub: 0, line: 104 } |  |  | 0.503 |
| walker |  | 8281 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.503 |
| ns | 8324 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.501 |
| walker |  | 8431 | 150 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 8477 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 118 } |  |  | 0.510 |
| walker |  | 8524 | 47 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 149 } |  |  | 0.510 |
| ns | 8556 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.507 |
| walker |  | 8572 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 185 } |  |  | 0.507 |
| walker |  | 8620 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 216 } |  |  | 0.507 |
| walker |  | 8669 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 10, sub: 0, line: 245 } |  |  | 0.507 |
| ns | 8731 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.516 |
| walker |  | 8758 | 89 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 39 } |  |  | 0.516 |
| walker |  | 8846 | 88 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 28 } |  |  | 0.516 |
| ns | 8963 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.527 |
| walker |  | 8971 | 125 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 50 } |  |  | 0.543 |
| walker |  | 9039 | 68 | Code::CodeKey { rung: Doc, file: src/typeguard/_functions.py, decl: 11, sub: 0, line: 291 } |  |  | 0.550 |
| ns | 9434 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.571 |
| walker |  | 9533 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.571 |
| ns | 9624 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.573 |
| ns | 9825 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.565 |
| ns | 9860 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.569 |
| ns | 9916 |  | 56 | CI interpreter matrix | 7.4 |  | 0.568 |
