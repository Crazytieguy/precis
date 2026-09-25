Score(3000)=0.666 I=0.901 C=0.493 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.776/0.759/0.732/0.666/0.602/0.495/0.537

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
| ns | 954 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.670 |
| walker |  | 959 | 227 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.776 |
| walker |  | 1028 | 69 | Code::CodeKey { rung: Body, file: src/typeguard/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.779 |
| ns | 1109 |  | 155 | Docs and test tree listings | 1.9 |  | 0.667 |
| walker |  | 1128 | 100 | Fs::DirListing { dir: tests } |  |  | 0.790 |
| walker |  | 1142 | 14 | Fs::DirListing { dir: tests/mypy } |  |  | 0.825 |
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
| walker |  | 1796 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.787 |
| walker |  | 1816 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.787 |
| walker |  | 1846 | 30 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.787 |
| walker |  | 1915 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| walker |  | 1943 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.788 |
| walker |  | 1989 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.788 |
| ns | 2059 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.732 |
| walker |  | 2081 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.735 |
| walker |  | 2123 | 42 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 1, sub: 0, line: 5 } |  |  | 0.736 |
| walker |  | 2240 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.736 |
| walker |  | 2272 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.736 |
| walker |  | 2321 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.737 |
| walker |  | 2381 | 60 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.737 |
| walker |  | 2386 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.737 |
| walker |  | 2447 | 61 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.738 |
| ns | 2451 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.680 |
| walker |  | 2499 | 52 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.680 |
| walker |  | 2555 | 56 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.680 |
| walker |  | 2588 | 33 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.680 |
| walker |  | 2600 | 12 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 16, sub: 0, line: 175 } |  |  | 0.680 |
| ns | 2613 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.658 |
| walker |  | 2676 | 76 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.658 |
| walker |  | 2705 | 29 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.658 |
| ns | 2737 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.664 |
| walker |  | 2822 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 2828 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 22 } |  |  | 0.665 |
| walker |  | 2834 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 6, sub: 0, line: 26 } |  |  | 0.665 |
| walker |  | 2868 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 7, sub: 0, line: 30 } |  |  | 0.666 |
| walker |  | 3000 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_decorators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 3008 | 8 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 6, sub: 0, line: 146 } |  |  | 0.666 |
| ns | 3035 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.625 |
| walker |  | 3042 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 3, sub: 0, line: 36 } |  |  | 0.625 |
| walker |  | 3146 | 104 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 7, sub: 0, line: 150 } |  |  | 0.648 |
| walker |  | 3252 | 106 | Code::CodeKey { rung: Decl, file: src/typeguard/_decorators.py, decl: 5, sub: 0, line: 136 } |  |  | 0.648 |
| ns | 3264 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.625 |
| walker |  | 3272 | 20 | Code::CodeKey { rung: Body, file: src/typeguard/_decorators.py, decl: 2, sub: 0, line: 32 } |  |  | 0.625 |
| walker |  | 3404 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| ns | 3432 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.612 |
| walker |  | 3440 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 7, sub: 0, line: 172 } |  |  | 0.612 |
| walker |  | 3447 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 8, sub: 0, line: 176 } |  |  | 0.612 |
| walker |  | 3503 | 56 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 5, sub: 0, line: 154 } |  |  | 0.612 |
| ns | 3559 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.605 |
| walker |  | 3576 | 73 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 3, sub: 0, line: 127 } |  |  | 0.605 |
| ns | 3765 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.609 |
| ns | 3958 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.598 |
| walker |  | 4003 | 427 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 4027 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 1, sub: 0, line: 81 } |  |  | 0.601 |
| walker |  | 4054 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 2, sub: 0, line: 84 } |  |  | 0.601 |
| walker |  | 4085 | 31 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 4, sub: 0, line: 89 } |  |  | 0.601 |
| ns | 4104 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.610 |
| walker |  | 4122 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 32, sub: 0, line: 924 } |  |  | 0.610 |
| walker |  | 4160 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 34, sub: 0, line: 1061 } |  |  | 0.610 |
| walker |  | 4210 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 7, sub: 0, line: 153 } |  |  | 0.610 |
| ns | 4214 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.602 |
| walker |  | 4260 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 8, sub: 0, line: 212 } |  |  | 0.602 |
| walker |  | 4310 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 9, sub: 0, line: 247 } |  |  | 0.602 |
| walker |  | 4360 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 10, sub: 0, line: 305 } |  |  | 0.602 |
| walker |  | 4410 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 11, sub: 0, line: 324 } |  |  | 0.602 |
| ns | 4449 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.582 |
| walker |  | 4460 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 12, sub: 0, line: 343 } |  |  | 0.582 |
| walker |  | 4510 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 13, sub: 0, line: 365 } |  |  | 0.582 |
| walker |  | 4560 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 14, sub: 0, line: 423 } |  |  | 0.582 |
| walker |  | 4610 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 15, sub: 0, line: 447 } |  |  | 0.582 |
| ns | 4612 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.583 |
| walker |  | 4660 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 16, sub: 0, line: 474 } |  |  | 0.583 |
| walker |  | 4710 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 17, sub: 0, line: 536 } |  |  | 0.583 |
| walker |  | 4760 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 18, sub: 0, line: 545 } |  |  | 0.583 |
| walker |  | 4810 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 21, sub: 0, line: 590 } |  |  | 0.583 |
| walker |  | 4860 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 22, sub: 0, line: 623 } |  |  | 0.583 |
| walker |  | 4910 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 23, sub: 0, line: 632 } |  |  | 0.583 |
| walker |  | 4960 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 24, sub: 0, line: 641 } |  |  | 0.583 |
| ns | 4963 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.557 |
| walker |  | 5010 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 25, sub: 0, line: 651 } |  |  | 0.557 |
| walker |  | 5060 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 26, sub: 0, line: 663 } |  |  | 0.557 |
| walker |  | 5110 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 28, sub: 0, line: 834 } |  |  | 0.557 |
| walker |  | 5160 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 29, sub: 0, line: 885 } |  |  | 0.557 |
| walker |  | 5210 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 30, sub: 0, line: 895 } |  |  | 0.557 |
| ns | 5222 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.540 |
| walker |  | 5260 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 31, sub: 0, line: 915 } |  |  | 0.540 |
| walker |  | 5329 | 69 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 19, sub: 0, line: 555 } |  |  | 0.540 |
| walker |  | 5501 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 5523 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.541 |
| walker |  | 5547 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.541 |
| walker |  | 5573 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.541 |
| ns | 5575 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.522 |
| walker |  | 5599 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.522 |
| walker |  | 5732 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.522 |
| ns | 5758 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.514 |
| walker |  | 5872 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.514 |
| walker |  | 6015 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.514 |
| walker |  | 6030 | 15 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.502 |
| ns | 6030 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.502 |
| ns | 6174 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.495 |
| walker |  | 6188 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.495 |
| walker |  | 6463 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.495 |
| ns | 6473 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.521 |
| walker |  | 6879 | 416 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.522 |
| walker |  | 6886 | 7 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 47, sub: 0, line: 577 } |  |  | 0.522 |
| walker |  | 6915 | 29 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 913 } |  |  | 0.522 |
| walker |  | 6924 | 9 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 46, sub: 0, line: 574 } |  |  | 0.522 |
| walker |  | 6956 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.522 |
| ns | 6981 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.502 |
| walker |  | 6992 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.502 |
| walker |  | 7009 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 56, sub: 0, line: 918 } |  |  | 0.502 |
| walker |  | 7032 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 60, sub: 0, line: 1138 } |  |  | 0.502 |
| walker |  | 7075 | 43 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.502 |
| walker |  | 7118 | 43 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 61, sub: 0, line: 1181 } |  |  | 0.503 |
| ns | 7158 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.495 |
| walker |  | 7164 | 46 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 58, sub: 0, line: 994 } |  |  | 0.496 |
| walker |  | 7385 | 221 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.496 |
| ns | 7525 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.484 |
| walker |  | 7592 | 207 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.484 |
| ns | 7601 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.481 |
| walker |  | 7705 | 113 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.481 |
| walker |  | 7718 | 13 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 15, sub: 0, line: 225 } |  |  | 0.481 |
| ns | 7732 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.482 |
| walker |  | 7774 | 56 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 57, sub: 0, line: 945 } |  |  | 0.482 |
| walker |  | 7834 | 60 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 62, sub: 0, line: 1224 } |  |  | 0.482 |
| ns | 7878 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.492 |
| walker |  | 7895 | 61 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 59, sub: 0, line: 1036 } |  |  | 0.492 |
| walker |  | 7967 | 72 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.493 |
| walker |  | 8048 | 81 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 2, sub: 0, line: 104 } |  |  | 0.493 |
| walker |  | 8089 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.502 |
| walker |  | 8103 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.502 |
| ns | 8165 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.493 |
| walker |  | 8253 | 150 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 8299 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 118 } |  |  | 0.502 |
| ns | 8324 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.496 |
| walker |  | 8346 | 47 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 149 } |  |  | 0.496 |
| walker |  | 8394 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 185 } |  |  | 0.496 |
| walker |  | 8442 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 216 } |  |  | 0.496 |
| walker |  | 8491 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 10, sub: 0, line: 245 } |  |  | 0.496 |
| ns | 8556 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.492 |
| walker |  | 8580 | 89 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 39 } |  |  | 0.492 |
| walker |  | 8668 | 88 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 28 } |  |  | 0.492 |
| ns | 8731 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.502 |
| walker |  | 8793 | 125 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 50 } |  |  | 0.519 |
| walker |  | 8861 | 68 | Code::CodeKey { rung: Doc, file: src/typeguard/_functions.py, decl: 11, sub: 0, line: 291 } |  |  | 0.525 |
| ns | 8963 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.537 |
| walker |  | 9355 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.537 |
| walker |  | 9364 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 5, sub: 0, line: 22 } |  |  | 0.539 |
| ns | 9434 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.560 |
| ns | 9624 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.563 |
| ns | 9825 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.555 |
| ns | 9860 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.559 |
| walker |  | 9880 | 516 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 0, line: 1005 } |  |  | 0.593 |
| ns | 9916 |  | 56 | CI interpreter matrix | 7.4 |  | 0.592 |
| walker |  | 9982 | 102 | Plaintext::DeclSurface { file: docs/api.rst } |  |  | 0.592 |
