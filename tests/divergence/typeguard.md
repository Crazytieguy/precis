Score(3000)=0.613 I=0.835 C=0.451 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.743/0.672/0.536/0.613/0.518/0.431/0.524

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
| walker |  | 784 | 90 | README.rst section #1 |  |  | 0.821 |
| ns | 954 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.743 |
| walker |  | 1011 | 227 | python names src/typeguard/__init__.py #1 |  |  | 0.845 |
| walker |  | 1047 | 36 | python names src/typeguard/_pytest_plugin.py |  |  | 0.846 |
| walker |  | 1085 | 38 | python decl src/typeguard/_memo.py:8 |  |  | 0.846 |
| ns | 1109 |  | 155 | Docs and test tree listings | 1.9 |  | 0.722 |
| walker |  | 1154 | 69 | python body src/typeguard/__init__.py:37 |  |  | 0.724 |
| walker |  | 1207 | 53 | python names src/typeguard/_exceptions.py |  |  | 0.725 |
| walker |  | 1224 | 17 | python decl src/typeguard/_exceptions.py:12 |  |  | 0.725 |
| walker |  | 1241 | 17 | python decl src/typeguard/_exceptions.py:19 |  |  | 0.725 |
| walker |  | 1294 | 53 | python decl src/typeguard/_exceptions.py:26 |  |  | 0.725 |
| ns | 1339 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.671 |
| walker |  | 1363 | 69 | python names src/typeguard/_config.py |  |  | 0.671 |
| walker |  | 1391 | 28 | python decl src/typeguard/_config.py:14 |  |  | 0.671 |
| walker |  | 1437 | 46 | python decl src/typeguard/_config.py:30 |  |  | 0.672 |
| ns | 1473 |  | 134 | docs/index.rst in full | 1.11 |  | 0.621 |
| walker |  | 1529 | 92 | python decl src/typeguard/_config.py:62 |  |  | 0.623 |
| walker |  | 1538 | 9 | python body src/typeguard/_exceptions.py:15 |  |  | 0.623 |
| ns | 1636 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.592 |
| walker |  | 1655 | 117 | python names src/typeguard/_importhook.py |  |  | 0.592 |
| walker |  | 1687 | 32 | python decl src/typeguard/_importhook.py:55 |  |  | 0.593 |
| walker |  | 1736 | 49 | python decl src/typeguard/_importhook.py:183 |  |  | 0.593 |
| ns | 1752 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.576 |
| walker |  | 1796 | 60 | python decl src/typeguard/_importhook.py:156 |  |  | 0.576 |
| walker |  | 1801 | 5 | python body src/typeguard/_importhook.py:164 |  |  | 0.576 |
| walker |  | 1862 | 61 | python decl src/typeguard/_importhook.py:109 |  |  | 0.577 |
| walker |  | 1914 | 52 | python decl src/typeguard/_importhook.py:167 |  |  | 0.577 |
| walker |  | 1970 | 56 | python decl src/typeguard/_importhook.py:124 |  |  | 0.577 |
| ns | 2059 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.536 |
| walker |  | 2087 | 117 | python names src/typeguard/_suppression.py |  |  | 0.537 |
| walker |  | 2093 | 6 | python decl src/typeguard/_suppression.py:22 |  |  | 0.537 |
| walker |  | 2099 | 6 | python decl src/typeguard/_suppression.py:26 |  |  | 0.537 |
| walker |  | 2133 | 34 | python decl src/typeguard/_suppression.py:30 |  |  | 0.538 |
| walker |  | 2166 | 33 | python decl src/typeguard/_importhook.py:45 |  |  | 0.538 |
| walker |  | 2266 | 100 | listing of 'tests' |  |  | 0.623 |
| walker |  | 2280 | 14 | listing of 'tests/mypy' |  |  | 0.647 |
| walker |  | 2412 | 132 | python names src/typeguard/_decorators.py |  |  | 0.648 |
| walker |  | 2420 | 8 | python decl src/typeguard/_decorators.py:146 |  |  | 0.648 |
| ns | 2451 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.597 |
| walker |  | 2454 | 34 | python decl src/typeguard/_decorators.py:36 |  |  | 0.597 |
| walker |  | 2558 | 104 | python decl src/typeguard/_decorators.py:150 |  |  | 0.625 |
| ns | 2613 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.605 |
| walker |  | 2664 | 106 | python decl src/typeguard/_decorators.py:136 |  |  | 0.605 |
| ns | 2737 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.613 |
| walker |  | 2796 | 132 | python names src/typeguard/_utils.py |  |  | 0.613 |
| walker |  | 2832 | 36 | python decl src/typeguard/_utils.py:172 |  |  | 0.613 |
| walker |  | 2839 | 7 | python body src/typeguard/_utils.py:176 |  |  | 0.613 |
| ns | 3035 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.576 |
| ns | 3264 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.555 |
| walker |  | 3266 | 427 | python names src/typeguard/_checkers.py |  |  | 0.558 |
| walker |  | 3290 | 24 | python decl src/typeguard/_checkers.py:81 |  |  | 0.558 |
| walker |  | 3317 | 27 | python decl src/typeguard/_checkers.py:84 |  |  | 0.558 |
| walker |  | 3348 | 31 | python decl src/typeguard/_checkers.py:89 |  |  | 0.558 |
| walker |  | 3385 | 37 | python decl src/typeguard/_checkers.py:924 |  |  | 0.558 |
| walker |  | 3423 | 38 | python decl src/typeguard/_checkers.py:1061 |  |  | 0.558 |
| ns | 3432 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.544 |
| walker |  | 3473 | 50 | python decl src/typeguard/_checkers.py:153 |  |  | 0.544 |
| walker |  | 3523 | 50 | python decl src/typeguard/_checkers.py:212 |  |  | 0.544 |
| ns | 3559 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.537 |
| walker |  | 3573 | 50 | python decl src/typeguard/_checkers.py:247 |  |  | 0.537 |
| walker |  | 3623 | 50 | python decl src/typeguard/_checkers.py:305 |  |  | 0.537 |
| walker |  | 3673 | 50 | python decl src/typeguard/_checkers.py:324 |  |  | 0.537 |
| walker |  | 3723 | 50 | python decl src/typeguard/_checkers.py:343 |  |  | 0.537 |
| ns | 3765 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.525 |
| walker |  | 3773 | 50 | python decl src/typeguard/_checkers.py:365 |  |  | 0.525 |
| walker |  | 3823 | 50 | python decl src/typeguard/_checkers.py:423 |  |  | 0.525 |
| walker |  | 3873 | 50 | python decl src/typeguard/_checkers.py:447 |  |  | 0.525 |
| walker |  | 3923 | 50 | python decl src/typeguard/_checkers.py:474 |  |  | 0.525 |
| ns | 3958 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.513 |
| walker |  | 3973 | 50 | python decl src/typeguard/_checkers.py:536 |  |  | 0.513 |
| walker |  | 4023 | 50 | python decl src/typeguard/_checkers.py:545 |  |  | 0.513 |
| walker |  | 4073 | 50 | python decl src/typeguard/_checkers.py:590 |  |  | 0.513 |
| ns | 4104 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.525 |
| walker |  | 4123 | 50 | python decl src/typeguard/_checkers.py:623 |  |  | 0.525 |
| walker |  | 4173 | 50 | python decl src/typeguard/_checkers.py:632 |  |  | 0.525 |
| ns | 4214 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.518 |
| walker |  | 4223 | 50 | python decl src/typeguard/_checkers.py:641 |  |  | 0.518 |
| walker |  | 4273 | 50 | python decl src/typeguard/_checkers.py:651 |  |  | 0.518 |
| walker |  | 4323 | 50 | python decl src/typeguard/_checkers.py:663 |  |  | 0.518 |
| walker |  | 4373 | 50 | python decl src/typeguard/_checkers.py:834 |  |  | 0.518 |
| walker |  | 4423 | 50 | python decl src/typeguard/_checkers.py:885 |  |  | 0.518 |
| ns | 4449 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.501 |
| walker |  | 4473 | 50 | python decl src/typeguard/_checkers.py:895 |  |  | 0.501 |
| walker |  | 4523 | 50 | python decl src/typeguard/_checkers.py:915 |  |  | 0.501 |
| walker |  | 4592 | 69 | python decl src/typeguard/_checkers.py:555 |  |  | 0.501 |
| ns | 4612 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.491 |
| walker |  | 4663 | 71 | python decl src/typeguard/_memo.py:37 |  |  | 0.505 |
| walker |  | 4835 | 172 | python names src/typeguard/_transformer.py |  |  | 0.506 |
| walker |  | 4857 | 22 | python decl src/typeguard/_transformer.py:84 |  |  | 0.506 |
| walker |  | 4881 | 24 | python decl src/typeguard/_transformer.py:88 |  |  | 0.506 |
| walker |  | 4907 | 26 | python decl src/typeguard/_transformer.py:92 |  |  | 0.506 |
| walker |  | 4933 | 26 | python decl src/typeguard/_transformer.py:96 |  |  | 0.506 |
| ns | 4963 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.484 |
| walker |  | 5066 | 133 | python decl src/typeguard/_transformer.py:70 |  |  | 0.484 |
| walker |  | 5206 | 140 | python decl src/typeguard/_transformer.py:285 |  |  | 0.484 |
| ns | 5222 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.469 |
| walker |  | 5349 | 143 | python decl src/typeguard/_transformer.py:313 |  |  | 0.469 |
| walker |  | 5507 | 158 | python decl src/typeguard/_transformer.py:100 |  |  | 0.469 |
| ns | 5575 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.452 |
| ns | 5758 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.446 |
| walker |  | 5782 | 275 | python decl src/typeguard/_transformer.py:338 |  |  | 0.446 |
| walker |  | 5932 | 150 | python names src/typeguard/_functions.py |  |  | 0.447 |
| walker |  | 5978 | 46 | python decl src/typeguard/_functions.py:118 |  |  | 0.447 |
| walker |  | 6025 | 47 | python decl src/typeguard/_functions.py:149 |  |  | 0.447 |
| ns | 6030 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.436 |
| walker |  | 6073 | 48 | python decl src/typeguard/_functions.py:185 |  |  | 0.436 |
| walker |  | 6121 | 48 | python decl src/typeguard/_functions.py:216 |  |  | 0.436 |
| walker |  | 6170 | 49 | python decl src/typeguard/_functions.py:245 |  |  | 0.436 |
| ns | 6174 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.431 |
| walker |  | 6259 | 89 | python decl src/typeguard/_functions.py:39 |  |  | 0.431 |
| walker |  | 6347 | 88 | python decl src/typeguard/_functions.py:28 |  |  | 0.431 |
| walker |  | 6472 | 125 | python decl src/typeguard/_functions.py:50 |  |  | 0.454 |
| ns | 6473 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.482 |
| walker |  | 6548 | 76 | python decl src/typeguard/_importhook.py:56 |  |  | 0.482 |
| walker |  | 6557 | 9 | python body src/typeguard/_exceptions.py:22 |  |  | 0.483 |
| walker |  | 6973 | 416 | python decl src/typeguard/_transformer.py:488 |  |  | 0.484 |
| walker |  | 6980 | 7 | python decl src/typeguard/_transformer.py:577 |  |  | 0.484 |
| ns | 6981 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.466 |
| walker |  | 7009 | 29 | python decl src/typeguard/_transformer.py:913 |  |  | 0.466 |
| walker |  | 7018 | 9 | python decl src/typeguard/_transformer.py:574 |  |  | 0.466 |
| walker |  | 7050 | 32 | python decl src/typeguard/_transformer.py:650 |  |  | 0.466 |
| walker |  | 7086 | 36 | python decl src/typeguard/_transformer.py:489 |  |  | 0.466 |
| walker |  | 7129 | 43 | python decl src/typeguard/_transformer.py:516 |  |  | 0.466 |
| walker |  | 7149 | 20 | python body src/typeguard/_decorators.py:32 |  |  | 0.466 |
| walker |  | 7158 | 9 | python body src/typeguard/_exceptions.py:35 |  |  | 0.460 |
| ns | 7158 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.460 |
| walker |  | 7379 | 221 | python decl src/typeguard/_transformer.py:117 |  |  | 0.460 |
| walker |  | 7398 | 19 | python doc src/typeguard/_exceptions.py:19 |  |  | 0.463 |
| walker |  | 7454 | 56 | python body src/typeguard/_utils.py:154 |  |  | 0.463 |
| ns | 7525 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.452 |
| ns | 7601 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.457 |
| walker |  | 7661 | 207 | python decl src/typeguard/_transformer.py:117 #1 |  |  | 0.457 |
| ns | 7732 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.458 |
| walker |  | 7734 | 73 | python doc src/typeguard/_utils.py:127 |  |  | 0.458 |
| walker |  | 7754 | 20 | python doc src/typeguard/_exceptions.py:12 |  |  | 0.461 |
| walker |  | 7795 | 41 | python body src/typeguard/_memo.py:37 |  |  | 0.470 |
| ns | 7878 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.480 |
| ns | 8165 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.471 |
| walker |  | 8289 | 494 | python decl src/typeguard/_checkers.py:99 |  |  | 0.471 |
| walker |  | 8296 | 7 | python body src/typeguard/_importhook.py:167 |  |  | 0.471 |
| ns | 8324 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.465 |
| ns | 8556 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.462 |
| ns | 8731 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.472 |
| walker |  | 8812 | 516 | python decl src/typeguard/_checkers.py:1005 |  |  | 0.512 |
| walker |  | 8831 | 19 | python body src/typeguard/_functions.py:291 |  |  | 0.513 |
| walker |  | 8933 | 102 | declaration surface of docs/api.rst |  |  | 0.513 |
| ns | 8963 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.524 |
| walker |  | 9046 | 113 | python decl src/typeguard/_transformer.py:117 #2 |  |  | 0.524 |
| ns | 9434 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.511 |
| ns | 9624 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.514 |
| ns | 9825 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.507 |
| ns | 9860 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.511 |
| walker |  | 9898 | 852 | manifest config in pyproject.toml |  |  | 0.518 |
| ns | 9916 |  | 56 | CI interpreter matrix | 7.4 |  | 0.517 |
