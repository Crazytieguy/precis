Score(3000)=0.630 I=0.878 C=0.452 ns_rows≤3K=17/50 (reached=9 partial=0 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 1.000 |
| ns | 38 |  | 38 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 42 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 82 | 40 | listing of 'docs' |  |  | 1.000 |
| ns | 144 |  | 106 | README identity paragraph | 1.2 |  | 0.771 |
| walker |  | 153 | 71 | listing of 'src/typeguard' |  |  | 0.816 |
| ns | 220 |  | 76 | Package module roster: src/typeguard/ | 1.3 |  | 0.867 |
| ns | 315 |  | 95 | README: the two principal checking modes | 1.4 |  | 0.746 |
| ns | 421 |  | 106 | README: what instrumentation actually covers | 1.5 | 1.4 | 0.680 |
| ns | 554 |  | 133 | README: the two instrumentation entry points | 1.6 |  | 0.591 |
| walker |  | 574 | 421 | python imports in src/typeguard/__init__.py |  |  | 0.621 |
| walker |  | 612 | 38 | python decl names surface in src/typeguard/__init__.py |  |  | 0.621 |
| walker |  | 612 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.621 |
| walker |  | 667 | 55 | [dependencies] in pyproject.toml |  |  | 0.621 |
| walker |  | 695 | 28 | listing of '.github' |  |  | 0.622 |
| walker |  | 702 | 7 | listing of '.github/workflows' |  |  | 0.622 |
| ns | 750 |  | 196 | Public exports of typeguard/__init__.py, first half | 1.7 |  | 0.674 |
| ns | 956 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.704 |
| ns | 1110 |  | 154 | Docs and test tree listings | 1.9 |  | 0.604 |
| walker |  | 1177 | 475 | YAML config at .github/workflows/test.yml |  |  | 0.604 |
| walker |  | 1247 | 70 | [package] in pyproject.toml |  |  | 0.605 |
| walker |  | 1282 | 35 | package metadata in pyproject.toml |  |  | 0.605 |
| walker |  | 1330 | 48 | tool.mypy config in pyproject.toml |  |  | 0.605 |
| ns | 1340 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.545 |
| ns | 1474 |  | 134 | docs/index.rst in full | 1.11 |  | 0.504 |
| ns | 1637 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.479 |
| ns | 1753 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.465 |
| walker |  | 1797 | 467 | README headline in README.rst |  |  | 0.656 |
| walker |  | 1897 | 100 | listing of 'tests' |  |  | 0.744 |
| walker |  | 1910 | 13 | listing of 'tests/mypy' |  |  | 0.769 |
| walker |  | 2057 | 147 | dev/build/target dependencies in pyproject.toml |  |  | 0.770 |
| ns | 2060 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.715 |
| walker |  | 2245 | 188 | manifest config in pyproject.toml |  |  | 0.716 |
| walker |  | 2434 | 189 | tool.tox config in pyproject.toml |  |  | 0.716 |
| ns | 2452 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.660 |
| ns | 2614 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.636 |
| walker |  | 2638 | 204 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.637 |
| ns | 2738 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.618 |
| walker |  | 2842 | 204 | tool.ruff config in pyproject.toml |  |  | 0.618 |
| walker |  | 2911 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.630 |
| walker |  | 2932 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.630 |
| walker |  | 2945 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.630 |
| walker |  | 2958 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.630 |
| walker |  | 2958 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.630 |
| walker |  | 2981 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.630 |
| walker |  | 2992 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.630 |
| ns | 3036 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.588 |
| walker |  | 3063 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.589 |
| walker |  | 3112 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.589 |
| walker |  | 3112 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.589 |
| walker |  | 3112 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.589 |
| walker |  | 3112 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.589 |
| walker |  | 3112 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.589 |
| walker |  | 3133 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.589 |
| walker |  | 3155 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.590 |
| walker |  | 3187 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.590 |
| walker |  | 3229 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.590 |
| ns | 3265 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.567 |
| walker |  | 3310 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.567 |
| walker |  | 3310 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.567 |
| walker |  | 3310 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.567 |
| walker |  | 3310 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.567 |
| walker |  | 3310 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.567 |
| walker |  | 3310 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.567 |
| walker |  | 3319 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.568 |
| walker |  | 3328 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.568 |
| walker |  | 3350 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.568 |
| walker |  | 3359 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.568 |
| walker |  | 3426 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.569 |
| ns | 3433 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.553 |
| walker |  | 3450 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.553 |
| walker |  | 3450 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.553 |
| walker |  | 3478 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.554 |
| walker |  | 3478 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.554 |
| walker |  | 3512 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.554 |
| ns | 3560 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.546 |
| walker |  | 3580 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.547 |
| walker |  | 3580 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.547 |
| walker |  | 3580 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.547 |
| walker |  | 3589 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.548 |
| walker |  | 3611 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.549 |
| walker |  | 3639 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.551 |
| walker |  | 3661 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.552 |
| walker |  | 3745 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.576 |
| walker |  | 3764 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.577 |
| walker |  | 3764 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.577 |
| ns | 3766 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.587 |
| walker |  | 3877 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.616 |
| walker |  | 3922 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.616 |
| ns | 3959 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.608 |
| walker |  | 4094 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.609 |
| walker |  | 4094 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.609 |
| walker |  | 4094 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.609 |
| walker |  | 4094 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.609 |
| walker |  | 4094 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.609 |
| walker |  | 4102 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.609 |
| ns | 4105 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.613 |
| walker |  | 4124 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.613 |
| walker |  | 4141 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.613 |
| walker |  | 4165 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.613 |
| walker |  | 4191 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.613 |
| ns | 4215 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.605 |
| walker |  | 4217 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.605 |
| walker |  | 4243 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.605 |
| walker |  | 4357 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.605 |
| ns | 4450 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.585 |
| ns | 4613 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.585 |
| walker |  | 4714 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.585 |
| ns | 4964 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.560 |
| ns | 5223 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.543 |
| ns | 5576 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.524 |
| walker |  | 5664 | 950 | python method sigs in src/typeguard/_transformer.py |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.525 |
| walker |  | 5664 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.525 |
| walker |  | 5675 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.525 |
| walker |  | 5686 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.525 |
| walker |  | 5699 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.525 |
| walker |  | 5728 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.525 |
| ns | 5759 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.517 |
| walker |  | 5760 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.518 |
| walker |  | 5777 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.518 |
| walker |  | 5813 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.518 |
| walker |  | 5836 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.518 |
| walker |  | 5864 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.518 |
| walker |  | 5864 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.518 |
| walker |  | 5892 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.518 |
| walker |  | 5892 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.518 |
| walker |  | 5935 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.518 |
| walker |  | 5981 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.518 |
| ns | 6031 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.506 |
| walker |  | 6037 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.506 |
| walker |  | 6097 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.506 |
| walker |  | 6158 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.507 |
| ns | 6175 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.500 |
| walker |  | 6201 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.500 |
| walker |  | 6273 | 72 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.501 |
| walker |  | 6406 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.501 |
| ns | 6474 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.487 |
| walker |  | 6562 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.487 |
| walker |  | 6692 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.488 |
| walker |  | 6692 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.488 |
| walker |  | 6738 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.488 |
| walker |  | 6785 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.488 |
| walker |  | 6833 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.488 |
| walker |  | 6881 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.488 |
| walker |  | 6930 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.488 |
| walker |  | 6949 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.489 |
| ns | 6982 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.471 |
| walker |  | 7048 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.471 |
| walker |  | 7048 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.471 |
| walker |  | 7146 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.471 |
| walker |  | 7146 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.471 |
| ns | 7159 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.463 |
| walker |  | 7212 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.475 |
| walker |  | 7337 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.495 |
| walker |  | 7359 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.495 |
| walker |  | 7476 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.496 |
| walker |  | 7476 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.496 |
| walker |  | 7476 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.496 |
| walker |  | 7476 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.496 |
| walker |  | 7476 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.496 |
| walker |  | 7494 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.496 |
| ns | 7526 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.483 |
| walker |  | 7543 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.485 |
| walker |  | 7574 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.487 |
| ns | 7602 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.493 |
| walker |  | 7607 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.493 |
| ns | 7733 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.490 |
| walker |  | 7758 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.491 |
| walker |  | 7758 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.491 |
| walker |  | 7758 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.491 |
| walker |  | 7758 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.491 |
| walker |  | 7758 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.491 |
| walker |  | 7758 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.491 |
| walker |  | 7770 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.491 |
| walker |  | 7799 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.491 |
| walker |  | 7804 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.491 |
| walker |  | 7856 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.491 |
| ns | 7879 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.500 |
| walker |  | 7912 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.500 |
| walker |  | 7988 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.500 |
| walker |  | 8097 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.516 |
| walker |  | 8133 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.517 |
| walker |  | 8133 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.517 |
| walker |  | 8133 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.517 |
| ns | 8166 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.507 |
| walker |  | 8264 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.508 |
| walker |  | 8264 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.508 |
| walker |  | 8264 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.508 |
| walker |  | 8264 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.508 |
| walker |  | 8264 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.508 |
| walker |  | 8264 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.508 |
| walker |  | 8264 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.508 |
| walker |  | 8272 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.508 |
| walker |  | 8282 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.508 |
| walker |  | 8296 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.508 |
| walker |  | 8296 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.508 |
| walker |  | 8303 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.508 |
| ns | 8325 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.501 |
| walker |  | 8376 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.501 |
| walker |  | 8457 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.501 |
| walker |  | 8546 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.503 |
| walker |  | 8546 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.503 |
| walker |  | 8546 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.503 |
| ns | 8557 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.499 |
| walker |  | 8577 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.500 |
| walker |  | 8577 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.500 |
| walker |  | 8611 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.500 |
| walker |  | 8631 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.500 |
| ns | 8732 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.510 |
| walker |  | 8735 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.521 |
| walker |  | 8757 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.521 |
| walker |  | 8873 | 116 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.522 |
| walker |  | 8873 | 0 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.522 |
| walker |  | 8949 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.526 |
| ns | 8964 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.538 |
| walker |  | 9012 | 63 | python imports in src/typeguard/_config.py |  |  | 0.538 |
| walker |  | 9187 | 175 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.554 |
| ns | 9435 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.575 |
| walker |  | 9614 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.602 |
| walker |  | 9614 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.602 |
| walker |  | 9614 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.602 |
| walker |  | 9614 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.602 |
| ns | 9625 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.607 |
| walker |  | 9638 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.607 |
| walker |  | 9665 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.607 |
| walker |  | 9702 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.608 |
| walker |  | 9740 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.608 |
| walker |  | 9771 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.608 |
| walker |  | 9821 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.608 |
| ns | 9826 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.615 |
| ns | 9862 |  | 36 | GitHub workflows and repository meta files | 7.3 |  | 0.618 |
| walker |  | 9871 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.618 |
| ns | 9918 |  | 56 | CI interpreter matrix | 7.4 |  | 0.619 |
| walker |  | 9921 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.619 |
| walker |  | 9971 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.619 |
