Score(3000)=0.631 I=0.881 C=0.452 ns_rows≤3K=17/50 (reached=9 partial=0 missing=8)

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
| ns | 1340 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.545 |
| ns | 1474 |  | 134 | docs/index.rst in full | 1.11 |  | 0.504 |
| ns | 1637 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.479 |
| walker |  | 1749 | 467 | README headline in README.rst |  |  | 0.676 |
| ns | 1753 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.656 |
| walker |  | 1849 | 100 | listing of 'tests' |  |  | 0.744 |
| walker |  | 1862 | 13 | listing of 'tests/mypy' |  |  | 0.769 |
| walker |  | 2052 | 190 | manifest config in pyproject.toml |  |  | 0.770 |
| ns | 2060 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.715 |
| walker |  | 2258 | 206 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.715 |
| walker |  | 2327 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.729 |
| walker |  | 2348 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.729 |
| walker |  | 2427 | 79 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.729 |
| walker |  | 2427 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.729 |
| ns | 2452 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.672 |
| walker |  | 2464 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.672 |
| walker |  | 2514 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.672 |
| walker |  | 2564 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.672 |
| walker |  | 2614 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.648 |
| ns | 2614 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.648 |
| walker |  | 2664 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.648 |
| walker |  | 2714 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.648 |
| walker |  | 2727 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.648 |
| walker |  | 2727 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.648 |
| ns | 2738 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.629 |
| walker |  | 2750 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.629 |
| walker |  | 2761 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.629 |
| walker |  | 2832 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.630 |
| walker |  | 2881 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.630 |
| walker |  | 2881 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.630 |
| walker |  | 2881 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.630 |
| walker |  | 2881 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.630 |
| walker |  | 2881 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.630 |
| walker |  | 2902 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.630 |
| walker |  | 2924 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.630 |
| walker |  | 2956 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.630 |
| walker |  | 2998 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.631 |
| ns | 3036 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.590 |
| walker |  | 3079 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.591 |
| walker |  | 3079 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.591 |
| walker |  | 3079 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.591 |
| walker |  | 3079 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.591 |
| walker |  | 3079 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.591 |
| walker |  | 3079 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.591 |
| walker |  | 3088 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.591 |
| walker |  | 3097 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.591 |
| walker |  | 3119 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.591 |
| walker |  | 3128 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.592 |
| walker |  | 3195 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.592 |
| walker |  | 3219 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.592 |
| walker |  | 3219 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.592 |
| walker |  | 3247 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.592 |
| walker |  | 3247 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.592 |
| ns | 3265 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.568 |
| walker |  | 3281 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.569 |
| walker |  | 3349 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.570 |
| walker |  | 3349 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.570 |
| walker |  | 3349 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.570 |
| walker |  | 3358 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.571 |
| walker |  | 3380 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.572 |
| walker |  | 3408 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.574 |
| walker |  | 3430 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.575 |
| ns | 3433 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.559 |
| walker |  | 3514 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.584 |
| walker |  | 3533 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.585 |
| walker |  | 3533 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.585 |
| ns | 3560 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.576 |
| walker |  | 3646 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.608 |
| walker |  | 3691 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.608 |
| ns | 3766 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.615 |
| walker |  | 3863 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.616 |
| walker |  | 3863 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.616 |
| walker |  | 3863 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.616 |
| walker |  | 3863 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.616 |
| walker |  | 3863 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.616 |
| walker |  | 3871 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.616 |
| walker |  | 3893 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.616 |
| walker |  | 3917 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.616 |
| walker |  | 3943 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.616 |
| ns | 3959 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.608 |
| walker |  | 3969 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.608 |
| walker |  | 3986 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.608 |
| walker |  | 4012 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.608 |
| ns | 4105 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.613 |
| ns | 4215 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.604 |
| ns | 4450 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.584 |
| ns | 4613 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.585 |
| ns | 4964 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.559 |
| walker |  | 4966 | 954 | python method sigs in src/typeguard/_transformer.py |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.561 |
| walker |  | 4966 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.561 |
| walker |  | 4977 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.561 |
| walker |  | 4988 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.561 |
| walker |  | 5017 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.561 |
| walker |  | 5049 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.561 |
| walker |  | 5085 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.561 |
| walker |  | 5098 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.561 |
| walker |  | 5126 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.561 |
| walker |  | 5126 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.561 |
| walker |  | 5154 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.561 |
| walker |  | 5154 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.561 |
| walker |  | 5171 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.561 |
| walker |  | 5194 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.561 |
| ns | 5223 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.544 |
| walker |  | 5237 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.544 |
| walker |  | 5370 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.544 |
| walker |  | 5482 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.544 |
| ns | 5576 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.525 |
| ns | 5759 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.517 |
| walker |  | 5837 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.517 |
| walker |  | 5993 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.517 |
| ns | 6031 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.505 |
| walker |  | 6036 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.505 |
| walker |  | 6082 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.505 |
| walker |  | 6138 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.505 |
| ns | 6175 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.499 |
| walker |  | 6268 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.500 |
| walker |  | 6268 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.500 |
| walker |  | 6314 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.500 |
| walker |  | 6361 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.500 |
| walker |  | 6409 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.500 |
| walker |  | 6457 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.500 |
| ns | 6474 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.489 |
| walker |  | 6506 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.489 |
| walker |  | 6525 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.490 |
| walker |  | 6624 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.490 |
| walker |  | 6624 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.490 |
| walker |  | 6722 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.490 |
| walker |  | 6722 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.490 |
| walker |  | 6788 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.502 |
| walker |  | 6913 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.524 |
| walker |  | 6935 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.524 |
| ns | 6982 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.504 |
| walker |  | 7052 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.505 |
| walker |  | 7052 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.505 |
| walker |  | 7052 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.505 |
| walker |  | 7052 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.505 |
| walker |  | 7052 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.505 |
| walker |  | 7070 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.505 |
| walker |  | 7119 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.506 |
| walker |  | 7150 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.508 |
| ns | 7159 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.501 |
| walker |  | 7183 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.501 |
| walker |  | 7334 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.502 |
| walker |  | 7334 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.502 |
| walker |  | 7334 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.502 |
| walker |  | 7334 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.502 |
| walker |  | 7334 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.502 |
| walker |  | 7334 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.502 |
| walker |  | 7346 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.502 |
| walker |  | 7375 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.502 |
| walker |  | 7380 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.502 |
| walker |  | 7432 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.502 |
| walker |  | 7488 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.502 |
| ns | 7526 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.489 |
| walker |  | 7564 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.489 |
| ns | 7602 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.495 |
| walker |  | 7673 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.511 |
| walker |  | 7733 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.509 |
| ns | 7733 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.509 |
| walker |  | 7794 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.509 |
| walker |  | 7842 | 48 | tool.mypy config in pyproject.toml |  |  | 0.509 |
| walker |  | 7878 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.510 |
| walker |  | 7878 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.510 |
| walker |  | 7878 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.510 |
| ns | 7879 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.519 |
| walker |  | 8009 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.519 |
| walker |  | 8009 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.519 |
| walker |  | 8009 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.519 |
| walker |  | 8009 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.519 |
| walker |  | 8009 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.519 |
| walker |  | 8009 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.519 |
| walker |  | 8009 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.519 |
| walker |  | 8017 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.519 |
| walker |  | 8027 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.519 |
| walker |  | 8041 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.519 |
| walker |  | 8041 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.519 |
| walker |  | 8048 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.519 |
| walker |  | 8121 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.519 |
| ns | 8166 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.509 |
| walker |  | 8202 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.509 |
| ns | 8325 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.503 |
| walker |  | 8550 | 348 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.533 |
| walker |  | 8550 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.533 |
| walker |  | 8550 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.533 |
| ns | 8557 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.529 |
| walker |  | 8574 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.529 |
| walker |  | 8601 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.529 |
| walker |  | 8639 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.530 |
| walker |  | 8670 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.530 |
| walker |  | 8720 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.530 |
| ns | 8732 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.538 |
| walker |  | 8770 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.538 |
| walker |  | 8820 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.538 |
| walker |  | 8870 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.538 |
| walker |  | 8920 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.538 |
| ns | 8964 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.549 |
| walker |  | 8970 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.549 |
| walker |  | 9020 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.549 |
| walker |  | 9070 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.549 |
| walker |  | 9120 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.549 |
| walker |  | 9170 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.549 |
| walker |  | 9220 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.549 |
| walker |  | 9270 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.549 |
| walker |  | 9320 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.549 |
| walker |  | 9370 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.549 |
| walker |  | 9420 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.549 |
| ns | 9435 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.560 |
| walker |  | 9470 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.560 |
| walker |  | 9533 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.560 |
| walker |  | 9602 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.560 |
| walker |  | 9615 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.560 |
| ns | 9625 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.567 |
| walker |  | 9704 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.568 |
| walker |  | 9704 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.568 |
| walker |  | 9704 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.568 |
| walker |  | 9735 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.569 |
| walker |  | 9735 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.569 |
| walker |  | 9769 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.569 |
| walker |  | 9789 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.569 |
| ns | 9826 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.564 |
| ns | 9862 |  | 36 | GitHub workflows and repository meta files | 7.3 |  | 0.567 |
| walker |  | 9893 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.576 |
| walker |  | 9915 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.576 |
| ns | 9918 |  | 56 | CI interpreter matrix | 7.4 |  | 0.577 |
