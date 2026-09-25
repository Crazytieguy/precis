Score(3000)=0.688 I=0.900 C=0.526 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.777/0.623/0.497/0.688/0.575/0.476/0.520

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 79 | 41 | listing of 'docs' |  |  | 1.000 |
| ns | 140 |  | 106 | README identity paragraph | 1.2 |  | 0.771 |
| walker |  | 153 | 74 | [package] in pyproject.toml |  |  | 0.772 |
| ns | 218 |  | 78 | Package module roster: src/typeguard/ | 1.3 |  | 0.510 |
| walker |  | 227 | 74 | listing of 'src/typeguard' |  |  | 0.868 |
| ns | 313 |  | 95 | README: the two principal checking modes | 1.4 |  | 0.746 |
| ns | 419 |  | 106 | README: what instrumentation actually covers | 1.5 | 1.4 | 0.680 |
| walker |  | 454 | 227 | python names src/typeguard/__init__.py |  |  | 0.699 |
| walker |  | 467 | 13 | python names src/typeguard/_memo.py |  |  | 0.699 |
| walker |  | 518 | 51 | [dependencies] in pyproject.toml |  |  | 0.699 |
| ns | 552 |  | 133 | README: the two instrumentation entry points | 1.6 |  | 0.608 |
| walker |  | 624 | 106 | README headline in README.rst |  |  | 0.702 |
| walker |  | 659 | 35 | package metadata in pyproject.toml |  |  | 0.702 |
| walker |  | 686 | 27 | listing of '.github' |  |  | 0.703 |
| walker |  | 694 | 8 | listing of '.github/workflows' |  |  | 0.703 |
| ns | 748 |  | 196 | Public exports of typeguard/__init__.py, first half | 1.7 |  | 0.740 |
| walker |  | 921 | 227 | python names src/typeguard/__init__.py #1 |  |  | 0.755 |
| ns | 954 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.776 |
| walker |  | 957 | 36 | python names src/typeguard/_pytest_plugin.py |  |  | 0.776 |
| walker |  | 995 | 38 | python decl src/typeguard/_memo.py:8 |  |  | 0.777 |
| walker |  | 1064 | 69 | python body src/typeguard/__init__.py:37 |  |  | 0.779 |
| ns | 1109 |  | 155 | Docs and test tree listings | 1.9 |  | 0.667 |
| walker |  | 1117 | 53 | python names src/typeguard/_exceptions.py |  |  | 0.667 |
| walker |  | 1134 | 17 | python decl src/typeguard/_exceptions.py:12 |  |  | 0.667 |
| walker |  | 1151 | 17 | python decl src/typeguard/_exceptions.py:19 |  |  | 0.667 |
| walker |  | 1204 | 53 | python decl src/typeguard/_exceptions.py:26 |  |  | 0.667 |
| walker |  | 1273 | 69 | python names src/typeguard/_config.py |  |  | 0.668 |
| walker |  | 1301 | 28 | python decl src/typeguard/_config.py:14 |  |  | 0.668 |
| ns | 1339 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.620 |
| walker |  | 1347 | 46 | python decl src/typeguard/_config.py:30 |  |  | 0.621 |
| walker |  | 1439 | 92 | python decl src/typeguard/_config.py:62 |  |  | 0.623 |
| walker |  | 1448 | 9 | python body src/typeguard/_exceptions.py:15 |  |  | 0.623 |
| ns | 1473 |  | 134 | docs/index.rst in full | 1.11 |  | 0.576 |
| walker |  | 1565 | 117 | python names src/typeguard/_importhook.py |  |  | 0.576 |
| walker |  | 1597 | 32 | python decl src/typeguard/_importhook.py:55 |  |  | 0.576 |
| ns | 1636 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.548 |
| walker |  | 1646 | 49 | python decl src/typeguard/_importhook.py:183 |  |  | 0.548 |
| walker |  | 1706 | 60 | python decl src/typeguard/_importhook.py:156 |  |  | 0.548 |
| walker |  | 1711 | 5 | python body src/typeguard/_importhook.py:164 |  |  | 0.548 |
| ns | 1752 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.533 |
| walker |  | 1772 | 61 | python decl src/typeguard/_importhook.py:109 |  |  | 0.533 |
| walker |  | 1824 | 52 | python decl src/typeguard/_importhook.py:167 |  |  | 0.533 |
| walker |  | 1880 | 56 | python decl src/typeguard/_importhook.py:124 |  |  | 0.533 |
| walker |  | 1997 | 117 | python names src/typeguard/_suppression.py |  |  | 0.534 |
| walker |  | 2003 | 6 | python decl src/typeguard/_suppression.py:22 |  |  | 0.534 |
| walker |  | 2009 | 6 | python decl src/typeguard/_suppression.py:26 |  |  | 0.534 |
| walker |  | 2043 | 34 | python decl src/typeguard/_suppression.py:30 |  |  | 0.535 |
| ns | 2059 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.497 |
| walker |  | 2076 | 33 | python decl src/typeguard/_importhook.py:45 |  |  | 0.497 |
| walker |  | 2432 | 356 | README.rst section #0 |  |  | 0.631 |
| ns | 2451 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.582 |
| walker |  | 2532 | 100 | listing of 'tests' |  |  | 0.657 |
| walker |  | 2546 | 14 | listing of 'tests/mypy' |  |  | 0.678 |
| ns | 2613 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.656 |
| walker |  | 2678 | 132 | python names src/typeguard/_decorators.py |  |  | 0.657 |
| walker |  | 2686 | 8 | python decl src/typeguard/_decorators.py:146 |  |  | 0.657 |
| walker |  | 2720 | 34 | python decl src/typeguard/_decorators.py:36 |  |  | 0.657 |
| ns | 2737 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.663 |
| walker |  | 2824 | 104 | python decl src/typeguard/_decorators.py:150 |  |  | 0.688 |
| walker |  | 2930 | 106 | python decl src/typeguard/_decorators.py:136 |  |  | 0.688 |
| ns | 3035 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.645 |
| walker |  | 3062 | 132 | python names src/typeguard/_utils.py |  |  | 0.645 |
| walker |  | 3098 | 36 | python decl src/typeguard/_utils.py:172 |  |  | 0.645 |
| walker |  | 3105 | 7 | python body src/typeguard/_utils.py:176 |  |  | 0.645 |
| ns | 3264 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.622 |
| ns | 3432 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.606 |
| walker |  | 3532 | 427 | python names src/typeguard/_checkers.py |  |  | 0.609 |
| walker |  | 3556 | 24 | python decl src/typeguard/_checkers.py:81 |  |  | 0.609 |
| ns | 3559 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.601 |
| walker |  | 3583 | 27 | python decl src/typeguard/_checkers.py:84 |  |  | 0.601 |
| walker |  | 3614 | 31 | python decl src/typeguard/_checkers.py:89 |  |  | 0.601 |
| walker |  | 3651 | 37 | python decl src/typeguard/_checkers.py:924 |  |  | 0.601 |
| walker |  | 3689 | 38 | python decl src/typeguard/_checkers.py:1061 |  |  | 0.601 |
| walker |  | 3739 | 50 | python decl src/typeguard/_checkers.py:153 |  |  | 0.601 |
| ns | 3765 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.586 |
| walker |  | 3789 | 50 | python decl src/typeguard/_checkers.py:212 |  |  | 0.586 |
| walker |  | 3839 | 50 | python decl src/typeguard/_checkers.py:247 |  |  | 0.586 |
| walker |  | 3889 | 50 | python decl src/typeguard/_checkers.py:305 |  |  | 0.586 |
| walker |  | 3939 | 50 | python decl src/typeguard/_checkers.py:324 |  |  | 0.586 |
| ns | 3958 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.572 |
| walker |  | 3989 | 50 | python decl src/typeguard/_checkers.py:343 |  |  | 0.572 |
| walker |  | 4039 | 50 | python decl src/typeguard/_checkers.py:365 |  |  | 0.572 |
| walker |  | 4089 | 50 | python decl src/typeguard/_checkers.py:423 |  |  | 0.572 |
| ns | 4104 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.583 |
| walker |  | 4139 | 50 | python decl src/typeguard/_checkers.py:447 |  |  | 0.583 |
| walker |  | 4189 | 50 | python decl src/typeguard/_checkers.py:474 |  |  | 0.583 |
| ns | 4214 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.575 |
| walker |  | 4239 | 50 | python decl src/typeguard/_checkers.py:536 |  |  | 0.575 |
| walker |  | 4289 | 50 | python decl src/typeguard/_checkers.py:545 |  |  | 0.575 |
| walker |  | 4339 | 50 | python decl src/typeguard/_checkers.py:590 |  |  | 0.575 |
| walker |  | 4389 | 50 | python decl src/typeguard/_checkers.py:623 |  |  | 0.575 |
| walker |  | 4439 | 50 | python decl src/typeguard/_checkers.py:632 |  |  | 0.575 |
| ns | 4449 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.556 |
| walker |  | 4489 | 50 | python decl src/typeguard/_checkers.py:641 |  |  | 0.556 |
| walker |  | 4539 | 50 | python decl src/typeguard/_checkers.py:651 |  |  | 0.556 |
| walker |  | 4589 | 50 | python decl src/typeguard/_checkers.py:663 |  |  | 0.556 |
| ns | 4612 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.545 |
| walker |  | 4639 | 50 | python decl src/typeguard/_checkers.py:834 |  |  | 0.545 |
| walker |  | 4689 | 50 | python decl src/typeguard/_checkers.py:885 |  |  | 0.545 |
| walker |  | 4739 | 50 | python decl src/typeguard/_checkers.py:895 |  |  | 0.545 |
| walker |  | 4789 | 50 | python decl src/typeguard/_checkers.py:915 |  |  | 0.545 |
| walker |  | 4858 | 69 | python decl src/typeguard/_checkers.py:555 |  |  | 0.545 |
| walker |  | 4929 | 71 | python decl src/typeguard/_memo.py:37 |  |  | 0.558 |
| ns | 4963 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.534 |
| walker |  | 5101 | 172 | python names src/typeguard/_transformer.py |  |  | 0.535 |
| walker |  | 5123 | 22 | python decl src/typeguard/_transformer.py:84 |  |  | 0.535 |
| walker |  | 5147 | 24 | python decl src/typeguard/_transformer.py:88 |  |  | 0.535 |
| walker |  | 5173 | 26 | python decl src/typeguard/_transformer.py:92 |  |  | 0.535 |
| walker |  | 5199 | 26 | python decl src/typeguard/_transformer.py:96 |  |  | 0.535 |
| ns | 5222 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.518 |
| walker |  | 5332 | 133 | python decl src/typeguard/_transformer.py:70 |  |  | 0.518 |
| walker |  | 5472 | 140 | python decl src/typeguard/_transformer.py:285 |  |  | 0.518 |
| ns | 5575 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.500 |
| walker |  | 5615 | 143 | python decl src/typeguard/_transformer.py:313 |  |  | 0.500 |
| ns | 5758 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.493 |
| walker |  | 5773 | 158 | python decl src/typeguard/_transformer.py:100 |  |  | 0.493 |
| ns | 6030 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.481 |
| walker |  | 6048 | 275 | python decl src/typeguard/_transformer.py:338 |  |  | 0.481 |
| ns | 6174 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.475 |
| walker |  | 6198 | 150 | python names src/typeguard/_functions.py |  |  | 0.476 |
| walker |  | 6244 | 46 | python decl src/typeguard/_functions.py:118 |  |  | 0.476 |
| walker |  | 6291 | 47 | python decl src/typeguard/_functions.py:149 |  |  | 0.476 |
| walker |  | 6339 | 48 | python decl src/typeguard/_functions.py:185 |  |  | 0.476 |
| walker |  | 6387 | 48 | python decl src/typeguard/_functions.py:216 |  |  | 0.476 |
| walker |  | 6436 | 49 | python decl src/typeguard/_functions.py:245 |  |  | 0.476 |
| ns | 6473 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.504 |
| walker |  | 6525 | 89 | python decl src/typeguard/_functions.py:39 |  |  | 0.504 |
| walker |  | 6613 | 88 | python decl src/typeguard/_functions.py:28 |  |  | 0.504 |
| walker |  | 6738 | 125 | python decl src/typeguard/_functions.py:50 |  |  | 0.525 |
| walker |  | 6814 | 76 | python decl src/typeguard/_importhook.py:56 |  |  | 0.525 |
| walker |  | 6823 | 9 | python body src/typeguard/_exceptions.py:22 |  |  | 0.526 |
| ns | 6981 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.506 |
| ns | 7158 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.499 |
| walker |  | 7239 | 416 | python decl src/typeguard/_transformer.py:488 |  |  | 0.500 |
| walker |  | 7246 | 7 | python decl src/typeguard/_transformer.py:577 |  |  | 0.500 |
| walker |  | 7275 | 29 | python decl src/typeguard/_transformer.py:913 |  |  | 0.500 |
| walker |  | 7284 | 9 | python decl src/typeguard/_transformer.py:574 |  |  | 0.500 |
| walker |  | 7316 | 32 | python decl src/typeguard/_transformer.py:650 |  |  | 0.500 |
| walker |  | 7352 | 36 | python decl src/typeguard/_transformer.py:489 |  |  | 0.500 |
| walker |  | 7395 | 43 | python decl src/typeguard/_transformer.py:516 |  |  | 0.500 |
| walker |  | 7415 | 20 | python body src/typeguard/_decorators.py:32 |  |  | 0.500 |
| walker |  | 7424 | 9 | python body src/typeguard/_exceptions.py:35 |  |  | 0.501 |
| ns | 7525 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.489 |
| ns | 7601 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.495 |
| walker |  | 7645 | 221 | python decl src/typeguard/_transformer.py:117 |  |  | 0.495 |
| walker |  | 7664 | 19 | python doc src/typeguard/_exceptions.py:19 |  |  | 0.497 |
| walker |  | 7720 | 56 | python body src/typeguard/_utils.py:154 |  |  | 0.497 |
| ns | 7732 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.498 |
| ns | 7878 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.507 |
| walker |  | 7927 | 207 | python decl src/typeguard/_transformer.py:117 #1 |  |  | 0.507 |
| walker |  | 8000 | 73 | python doc src/typeguard/_utils.py:127 |  |  | 0.507 |
| walker |  | 8020 | 20 | python doc src/typeguard/_exceptions.py:12 |  |  | 0.509 |
| walker |  | 8061 | 41 | python body src/typeguard/_memo.py:37 |  |  | 0.518 |
| ns | 8165 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.509 |
| ns | 8324 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.502 |
| walker |  | 8555 | 494 | python decl src/typeguard/_checkers.py:99 |  |  | 0.502 |
| ns | 8556 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.499 |
| walker |  | 8562 | 7 | python body src/typeguard/_importhook.py:167 |  |  | 0.499 |
| ns | 8731 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.508 |
| ns | 8963 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.520 |
| walker |  | 9078 | 516 | python decl src/typeguard/_checkers.py:1005 |  |  | 0.559 |
| walker |  | 9097 | 19 | python body src/typeguard/_functions.py:291 |  |  | 0.560 |
| walker |  | 9199 | 102 | declaration surface of docs/api.rst |  |  | 0.560 |
| walker |  | 9312 | 113 | python decl src/typeguard/_transformer.py:117 #2 |  |  | 0.560 |
| ns | 9434 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.546 |
| ns | 9624 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.548 |
| ns | 9825 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.541 |
| ns | 9860 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.545 |
| ns | 9916 |  | 56 | CI interpreter matrix | 7.4 |  | 0.544 |
