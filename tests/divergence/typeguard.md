Score(3000)=0.631 I=0.881 C=0.452 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.828/0.901/0.732/0.631/0.494/0.441/0.521

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
| walker |  | 554 | 100 | Fs::DirListing { dir: tests } |  |  | 0.704 |
| walker |  | 568 | 14 | Fs::DirListing { dir: tests/mypy } |  |  | 0.709 |
| ns | 748 |  | 196 | Public exports of typeguard/__init__.py, first half | 1.7 |  | 0.627 |
| walker |  | 947 | 379 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.905 |
| ns | 954 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.828 |
| ns | 1109 |  | 155 | Docs and test tree listings | 1.9 |  | 0.849 |
| walker |  | 1174 | 227 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.929 |
| ns | 1339 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.833 |
| walker |  | 1401 | 227 | Code::CodeKey { rung: Names, file: src/typeguard/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.901 |
| walker |  | 1470 | 69 | Code::CodeKey { rung: Body, file: src/typeguard/__init__.py, decl: 2, sub: 0, line: 37 } |  |  | 0.919 |
| ns | 1473 |  | 134 | docs/index.rst in full | 1.11 |  | 0.849 |
| ns | 1636 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.807 |
| ns | 1752 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.784 |
| walker |  | 1897 | 427 | Code::CodeKey { rung: Names, file: src/typeguard/_checkers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| walker |  | 1921 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 1, sub: 0, line: 81 } |  |  | 0.788 |
| walker |  | 1948 | 27 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 2, sub: 0, line: 84 } |  |  | 0.788 |
| walker |  | 1979 | 31 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 4, sub: 0, line: 89 } |  |  | 0.788 |
| walker |  | 2016 | 37 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 32, sub: 0, line: 924 } |  |  | 0.788 |
| walker |  | 2054 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 34, sub: 0, line: 1061 } |  |  | 0.788 |
| ns | 2059 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.732 |
| walker |  | 2104 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 7, sub: 0, line: 153 } |  |  | 0.732 |
| walker |  | 2154 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 8, sub: 0, line: 212 } |  |  | 0.732 |
| walker |  | 2204 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 9, sub: 0, line: 247 } |  |  | 0.732 |
| walker |  | 2254 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 10, sub: 0, line: 305 } |  |  | 0.732 |
| walker |  | 2304 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 11, sub: 0, line: 324 } |  |  | 0.732 |
| walker |  | 2354 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 12, sub: 0, line: 343 } |  |  | 0.732 |
| walker |  | 2404 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 13, sub: 0, line: 365 } |  |  | 0.732 |
| ns | 2451 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.675 |
| walker |  | 2454 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 14, sub: 0, line: 423 } |  |  | 0.675 |
| walker |  | 2504 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 15, sub: 0, line: 447 } |  |  | 0.675 |
| walker |  | 2554 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 16, sub: 0, line: 474 } |  |  | 0.675 |
| walker |  | 2604 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 17, sub: 0, line: 536 } |  |  | 0.675 |
| ns | 2613 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.650 |
| walker |  | 2654 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 18, sub: 0, line: 545 } |  |  | 0.650 |
| walker |  | 2704 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 21, sub: 0, line: 590 } |  |  | 0.650 |
| ns | 2737 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.631 |
| walker |  | 2754 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 22, sub: 0, line: 623 } |  |  | 0.631 |
| walker |  | 2804 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 23, sub: 0, line: 632 } |  |  | 0.631 |
| walker |  | 2854 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 24, sub: 0, line: 641 } |  |  | 0.631 |
| walker |  | 2904 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 25, sub: 0, line: 651 } |  |  | 0.631 |
| walker |  | 2954 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 26, sub: 0, line: 663 } |  |  | 0.631 |
| walker |  | 3004 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 28, sub: 0, line: 834 } |  |  | 0.631 |
| ns | 3035 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.590 |
| walker |  | 3054 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 29, sub: 0, line: 885 } |  |  | 0.590 |
| walker |  | 3104 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 30, sub: 0, line: 895 } |  |  | 0.590 |
| walker |  | 3154 | 50 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 31, sub: 0, line: 915 } |  |  | 0.590 |
| walker |  | 3223 | 69 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 19, sub: 0, line: 555 } |  |  | 0.590 |
| walker |  | 3236 | 13 | Code::CodeKey { rung: Names, file: src/typeguard/_memo.py, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| ns | 3264 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.566 |
| walker |  | 3274 | 38 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 1, sub: 0, line: 8 } |  |  | 0.566 |
| walker |  | 3345 | 71 | Code::CodeKey { rung: Decl, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.567 |
| walker |  | 3386 | 41 | Code::CodeKey { rung: Body, file: src/typeguard/_memo.py, decl: 2, sub: 0, line: 37 } |  |  | 0.568 |
| ns | 3432 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.552 |
| walker |  | 3558 | 172 | Code::CodeKey { rung: Names, file: src/typeguard/_transformer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 3559 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.545 |
| walker |  | 3580 | 22 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 4, sub: 0, line: 84 } |  |  | 0.545 |
| walker |  | 3604 | 24 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 5, sub: 0, line: 88 } |  |  | 0.545 |
| walker |  | 3630 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 6, sub: 0, line: 92 } |  |  | 0.545 |
| walker |  | 3656 | 26 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 7, sub: 0, line: 96 } |  |  | 0.545 |
| ns | 3765 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.527 |
| walker |  | 3789 | 133 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 3, sub: 0, line: 70 } |  |  | 0.527 |
| walker |  | 3929 | 140 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 18, sub: 0, line: 285 } |  |  | 0.527 |
| walker |  | 3934 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_transformer.py, decl: 24, sub: 0, line: 306 } |  |  | 0.527 |
| walker |  | 3939 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_transformer.py, decl: 25, sub: 0, line: 309 } |  |  | 0.527 |
| ns | 3958 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.512 |
| walker |  | 4082 | 143 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.512 |
| walker |  | 4097 | 15 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 26, sub: 0, line: 313 } |  |  | 0.512 |
| ns | 4104 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.501 |
| ns | 4214 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.494 |
| walker |  | 4255 | 158 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 8, sub: 0, line: 100 } |  |  | 0.494 |
| ns | 4449 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.478 |
| walker |  | 4530 | 275 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 32, sub: 0, line: 338 } |  |  | 0.478 |
| walker |  | 4535 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_transformer.py, decl: 29, sub: 0, line: 325 } |  |  | 0.478 |
| ns | 4612 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.498 |
| walker |  | 4951 | 416 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 41, sub: 0, line: 488 } |  |  | 0.499 |
| walker |  | 4958 | 7 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 47, sub: 0, line: 577 } |  |  | 0.499 |
| ns | 4963 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.478 |
| walker |  | 4967 | 9 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 46, sub: 0, line: 574 } |  |  | 0.478 |
| walker |  | 4996 | 29 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 55, sub: 0, line: 913 } |  |  | 0.478 |
| walker |  | 5028 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.478 |
| walker |  | 5064 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 42, sub: 0, line: 489 } |  |  | 0.478 |
| walker |  | 5107 | 43 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 44, sub: 0, line: 516 } |  |  | 0.478 |
| walker |  | 5124 | 17 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 56, sub: 0, line: 918 } |  |  | 0.478 |
| walker |  | 5147 | 23 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 60, sub: 0, line: 1138 } |  |  | 0.478 |
| walker |  | 5190 | 43 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 61, sub: 0, line: 1181 } |  |  | 0.478 |
| ns | 5222 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.463 |
| walker |  | 5236 | 46 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 58, sub: 0, line: 994 } |  |  | 0.463 |
| walker |  | 5292 | 56 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 57, sub: 0, line: 945 } |  |  | 0.463 |
| walker |  | 5352 | 60 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 62, sub: 0, line: 1224 } |  |  | 0.464 |
| walker |  | 5413 | 61 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 59, sub: 0, line: 1036 } |  |  | 0.464 |
| walker |  | 5485 | 72 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 54, sub: 0, line: 650 } |  |  | 0.465 |
| ns | 5575 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.448 |
| walker |  | 5706 | 221 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 0, line: 117 } |  |  | 0.449 |
| ns | 5758 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.442 |
| walker |  | 5913 | 207 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 1, line: 117 } |  |  | 0.442 |
| walker |  | 6026 | 113 | Code::CodeKey { rung: Decl, file: src/typeguard/_transformer.py, decl: 9, sub: 2, line: 117 } |  |  | 0.442 |
| ns | 6030 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.431 |
| walker |  | 6039 | 13 | Code::CodeKey { rung: Doc, file: src/typeguard/_transformer.py, decl: 15, sub: 0, line: 225 } |  |  | 0.431 |
| walker |  | 6092 | 53 | Code::CodeKey { rung: Names, file: src/typeguard/_exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.433 |
| walker |  | 6109 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.434 |
| walker |  | 6126 | 17 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.435 |
| ns | 6174 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.429 |
| walker |  | 6179 | 53 | Code::CodeKey { rung: Decl, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.431 |
| walker |  | 6188 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 3, sub: 0, line: 15 } |  |  | 0.433 |
| walker |  | 6197 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 5, sub: 0, line: 22 } |  |  | 0.434 |
| walker |  | 6216 | 19 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 4, sub: 0, line: 19 } |  |  | 0.437 |
| walker |  | 6236 | 20 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 2, sub: 0, line: 12 } |  |  | 0.441 |
| walker |  | 6266 | 30 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 6, sub: 0, line: 26 } |  |  | 0.445 |
| walker |  | 6308 | 42 | Code::CodeKey { rung: Doc, file: src/typeguard/_exceptions.py, decl: 1, sub: 0, line: 5 } |  |  | 0.455 |
| walker |  | 6317 | 9 | Code::CodeKey { rung: Body, file: src/typeguard/_exceptions.py, decl: 8, sub: 0, line: 35 } |  |  | 0.457 |
| walker |  | 6467 | 150 | Code::CodeKey { rung: Names, file: src/typeguard/_functions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
| ns | 6473 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.488 |
| walker |  | 6513 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 6, sub: 0, line: 118 } |  |  | 0.488 |
| walker |  | 6560 | 47 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 7, sub: 0, line: 149 } |  |  | 0.488 |
| walker |  | 6608 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 8, sub: 0, line: 185 } |  |  | 0.488 |
| walker |  | 6656 | 48 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 9, sub: 0, line: 216 } |  |  | 0.488 |
| walker |  | 6705 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 10, sub: 0, line: 245 } |  |  | 0.488 |
| walker |  | 6794 | 89 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 4, sub: 0, line: 39 } |  |  | 0.488 |
| walker |  | 6882 | 88 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 3, sub: 0, line: 28 } |  |  | 0.488 |
| ns | 6981 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.469 |
| walker |  | 7007 | 125 | Code::CodeKey { rung: Decl, file: src/typeguard/_functions.py, decl: 5, sub: 0, line: 50 } |  |  | 0.490 |
| walker |  | 7075 | 68 | Code::CodeKey { rung: Doc, file: src/typeguard/_functions.py, decl: 11, sub: 0, line: 291 } |  |  | 0.498 |
| walker |  | 7089 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.498 |
| walker |  | 7158 | 69 | Code::CodeKey { rung: Names, file: src/typeguard/_config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| ns | 7158 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.492 |
| walker |  | 7186 | 28 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 2, sub: 0, line: 14 } |  |  | 0.493 |
| walker |  | 7232 | 46 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 3, sub: 0, line: 30 } |  |  | 0.495 |
| walker |  | 7324 | 92 | Code::CodeKey { rung: Decl, file: src/typeguard/_config.py, decl: 5, sub: 0, line: 62 } |  |  | 0.507 |
| walker |  | 7386 | 62 | Code::CodeKey { rung: Body, file: src/typeguard/_config.py, decl: 4, sub: 0, line: 52 } |  |  | 0.514 |
| ns | 7525 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.501 |
| ns | 7601 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.507 |
| ns | 7732 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.504 |
| ns | 7878 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.499 |
| walker |  | 7880 | 494 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 6, sub: 0, line: 99 } |  |  | 0.499 |
| walker |  | 7997 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_importhook.py, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 8029 | 32 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 5, sub: 0, line: 55 } |  |  | 0.502 |
| walker |  | 8062 | 33 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 3, sub: 0, line: 45 } |  |  | 0.502 |
| walker |  | 8111 | 49 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 17, sub: 0, line: 183 } |  |  | 0.504 |
| ns | 8165 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.494 |
| walker |  | 8171 | 60 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.500 |
| walker |  | 8176 | 5 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 14, sub: 0, line: 164 } |  |  | 0.500 |
| walker |  | 8228 | 52 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.500 |
| walker |  | 8289 | 61 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 8, sub: 0, line: 109 } |  |  | 0.506 |
| ns | 8324 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.500 |
| walker |  | 8345 | 56 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 10, sub: 0, line: 124 } |  |  | 0.500 |
| walker |  | 8357 | 12 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 16, sub: 0, line: 175 } |  |  | 0.500 |
| walker |  | 8433 | 76 | Code::CodeKey { rung: Decl, file: src/typeguard/_importhook.py, decl: 6, sub: 0, line: 56 } |  |  | 0.500 |
| walker |  | 8462 | 29 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 12, sub: 0, line: 156 } |  |  | 0.501 |
| walker |  | 8469 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_importhook.py, decl: 15, sub: 0, line: 167 } |  |  | 0.501 |
| walker |  | 8545 | 76 | Code::CodeKey { rung: Doc, file: src/typeguard/_importhook.py, decl: 11, sub: 0, line: 138 } |  |  | 0.506 |
| ns | 8556 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.499 |
| ns | 8731 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.509 |
| ns | 8963 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.521 |
| walker |  | 9061 | 516 | Code::CodeKey { rung: Decl, file: src/typeguard/_checkers.py, decl: 33, sub: 0, line: 1005 } |  |  | 0.560 |
| walker |  | 9193 | 132 | Code::CodeKey { rung: Names, file: src/typeguard/_utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 9229 | 36 | Code::CodeKey { rung: Decl, file: src/typeguard/_utils.py, decl: 7, sub: 0, line: 172 } |  |  | 0.563 |
| walker |  | 9236 | 7 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 8, sub: 0, line: 176 } |  |  | 0.563 |
| walker |  | 9292 | 56 | Code::CodeKey { rung: Body, file: src/typeguard/_utils.py, decl: 5, sub: 0, line: 154 } |  |  | 0.563 |
| walker |  | 9365 | 73 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 3, sub: 0, line: 127 } |  |  | 0.563 |
| ns | 9434 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.582 |
| walker |  | 9446 | 81 | Code::CodeKey { rung: Doc, file: src/typeguard/_utils.py, decl: 2, sub: 0, line: 104 } |  |  | 0.582 |
| walker |  | 9548 | 102 | Plaintext::DeclSurface { file: docs/api.rst } |  |  | 0.582 |
| ns | 9624 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.584 |
| walker |  | 9665 | 117 | Code::CodeKey { rung: Names, file: src/typeguard/_suppression.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 9671 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 5, sub: 0, line: 22 } |  |  | 0.590 |
| walker |  | 9677 | 6 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 6, sub: 0, line: 26 } |  |  | 0.590 |
| walker |  | 9711 | 34 | Code::CodeKey { rung: Decl, file: src/typeguard/_suppression.py, decl: 7, sub: 0, line: 30 } |  |  | 0.595 |
| ns | 9825 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.587 |
| ns | 9860 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.590 |
| ns | 9916 |  | 56 | CI interpreter matrix | 7.4 |  | 0.589 |
