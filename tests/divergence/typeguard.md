Score(3000)=0.640 I=0.892 C=0.459 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.705/0.902/0.729/0.640/0.604/0.501/0.549

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
| walker |  | 772 | 70 | [package] in pyproject.toml |  |  | 0.675 |
| walker |  | 807 | 35 | package metadata in pyproject.toml |  |  | 0.675 |
| ns | 956 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.705 |
| ns | 1110 |  | 154 | Docs and test tree listings | 1.9 |  | 0.605 |
| walker |  | 1274 | 467 | README headline in README.rst |  |  | 0.854 |
| ns | 1340 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.769 |
| walker |  | 1374 | 100 | listing of 'tests' |  |  | 0.872 |
| walker |  | 1387 | 13 | listing of 'tests/mypy' |  |  | 0.902 |
| ns | 1474 |  | 134 | docs/index.rst in full | 1.11 |  | 0.833 |
| walker |  | 1577 | 190 | manifest config in pyproject.toml |  |  | 0.834 |
| ns | 1637 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.792 |
| ns | 1753 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.769 |
| walker |  | 1783 | 206 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.770 |
| walker |  | 1852 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.784 |
| walker |  | 1873 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.784 |
| walker |  | 1952 | 79 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.785 |
| walker |  | 1952 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.785 |
| walker |  | 1989 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.785 |
| walker |  | 2039 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.785 |
| ns | 2060 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.729 |
| walker |  | 2089 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.729 |
| walker |  | 2139 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.729 |
| walker |  | 2189 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.729 |
| walker |  | 2239 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.729 |
| walker |  | 2252 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.729 |
| walker |  | 2252 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.729 |
| walker |  | 2275 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.729 |
| walker |  | 2286 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.729 |
| walker |  | 2357 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.730 |
| walker |  | 2406 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.730 |
| walker |  | 2406 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.730 |
| walker |  | 2406 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.730 |
| walker |  | 2406 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.730 |
| walker |  | 2406 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.730 |
| walker |  | 2427 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.731 |
| walker |  | 2449 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.731 |
| ns | 2452 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.674 |
| walker |  | 2481 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.674 |
| walker |  | 2523 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.675 |
| walker |  | 2604 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.676 |
| walker |  | 2604 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.676 |
| walker |  | 2604 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.676 |
| walker |  | 2604 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.676 |
| walker |  | 2604 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.676 |
| walker |  | 2604 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.676 |
| walker |  | 2613 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.676 |
| ns | 2614 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.651 |
| walker |  | 2622 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.652 |
| walker |  | 2644 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.652 |
| walker |  | 2653 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.652 |
| walker |  | 2720 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.652 |
| ns | 2738 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.633 |
| walker |  | 2744 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.634 |
| walker |  | 2744 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.634 |
| walker |  | 2772 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.634 |
| walker |  | 2772 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.634 |
| walker |  | 2806 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.635 |
| walker |  | 2874 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.636 |
| walker |  | 2874 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.636 |
| walker |  | 2874 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.636 |
| walker |  | 2883 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.637 |
| walker |  | 2905 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.638 |
| walker |  | 2933 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.640 |
| walker |  | 2955 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.640 |
| ns | 3036 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.599 |
| walker |  | 3039 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.625 |
| walker |  | 3058 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.626 |
| walker |  | 3058 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.626 |
| walker |  | 3171 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.660 |
| walker |  | 3216 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.660 |
| ns | 3265 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.634 |
| walker |  | 3388 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.635 |
| walker |  | 3388 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.635 |
| walker |  | 3388 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.635 |
| walker |  | 3388 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.635 |
| walker |  | 3388 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.635 |
| walker |  | 3396 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.635 |
| walker |  | 3418 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.635 |
| ns | 3433 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.618 |
| walker |  | 3442 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.618 |
| walker |  | 3468 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.618 |
| walker |  | 3494 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.618 |
| walker |  | 3511 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.618 |
| walker |  | 3537 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.618 |
| ns | 3560 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.609 |
| ns | 3766 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.616 |
| ns | 3959 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.608 |
| ns | 4105 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.612 |
| ns | 4215 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.604 |
| ns | 4450 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.584 |
| walker |  | 4491 | 954 | python method sigs in src/typeguard/_transformer.py |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.586 |
| walker |  | 4491 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.586 |
| walker |  | 4502 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.586 |
| walker |  | 4513 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.586 |
| walker |  | 4542 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.586 |
| walker |  | 4574 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.586 |
| walker |  | 4610 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.586 |
| ns | 4613 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.586 |
| walker |  | 4623 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.586 |
| walker |  | 4651 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.586 |
| walker |  | 4651 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.586 |
| walker |  | 4679 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.586 |
| walker |  | 4679 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.586 |
| walker |  | 4696 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.586 |
| walker |  | 4719 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.586 |
| walker |  | 4762 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.586 |
| walker |  | 4895 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.586 |
| ns | 4964 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.561 |
| walker |  | 5007 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.561 |
| ns | 5223 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.544 |
| walker |  | 5362 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.544 |
| walker |  | 5518 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.544 |
| walker |  | 5561 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.544 |
| ns | 5576 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.525 |
| walker |  | 5607 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.525 |
| walker |  | 5663 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.525 |
| ns | 5759 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.518 |
| walker |  | 5793 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.519 |
| walker |  | 5793 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.519 |
| walker |  | 5839 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.519 |
| walker |  | 5886 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.519 |
| walker |  | 5934 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.519 |
| walker |  | 5982 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.519 |
| walker |  | 6031 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.507 |
| ns | 6031 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.507 |
| walker |  | 6050 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.508 |
| walker |  | 6149 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.508 |
| walker |  | 6149 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.508 |
| ns | 6175 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.501 |
| walker |  | 6247 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.502 |
| walker |  | 6247 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.502 |
| walker |  | 6313 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.514 |
| walker |  | 6438 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.536 |
| walker |  | 6460 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.537 |
| ns | 6474 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.524 |
| walker |  | 6577 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.524 |
| walker |  | 6577 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.524 |
| walker |  | 6577 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.524 |
| walker |  | 6577 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.524 |
| walker |  | 6577 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.524 |
| walker |  | 6595 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.524 |
| walker |  | 6644 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.526 |
| walker |  | 6675 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.528 |
| walker |  | 6708 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.528 |
| walker |  | 6859 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.529 |
| walker |  | 6859 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.529 |
| walker |  | 6859 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.529 |
| walker |  | 6859 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.529 |
| walker |  | 6859 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.529 |
| walker |  | 6859 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.529 |
| walker |  | 6871 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.529 |
| walker |  | 6900 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.529 |
| walker |  | 6905 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.529 |
| walker |  | 6957 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.529 |
| ns | 6982 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.509 |
| walker |  | 7013 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.509 |
| walker |  | 7089 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.509 |
| ns | 7159 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.502 |
| walker |  | 7198 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.519 |
| walker |  | 7258 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.519 |
| walker |  | 7319 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.520 |
| walker |  | 7367 | 48 | tool.mypy config in pyproject.toml |  |  | 0.520 |
| walker |  | 7403 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.521 |
| walker |  | 7403 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.521 |
| walker |  | 7403 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.521 |
| ns | 7526 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.507 |
| walker |  | 7534 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.508 |
| walker |  | 7534 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.508 |
| walker |  | 7534 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.508 |
| walker |  | 7534 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.508 |
| walker |  | 7534 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.508 |
| walker |  | 7534 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.508 |
| walker |  | 7534 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.508 |
| walker |  | 7542 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.508 |
| walker |  | 7552 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.508 |
| walker |  | 7566 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.508 |
| walker |  | 7566 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.508 |
| walker |  | 7573 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.508 |
| ns | 7602 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.513 |
| walker |  | 7646 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.513 |
| walker |  | 7727 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.513 |
| ns | 7733 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.510 |
| ns | 7879 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.519 |
| walker |  | 8075 | 348 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.550 |
| walker |  | 8075 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.550 |
| walker |  | 8075 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.550 |
| walker |  | 8099 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.550 |
| walker |  | 8126 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.550 |
| walker |  | 8164 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.551 |
| ns | 8166 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.540 |
| walker |  | 8195 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.540 |
| walker |  | 8245 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.540 |
| walker |  | 8295 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.540 |
| ns | 8325 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.534 |
| walker |  | 8345 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.534 |
| walker |  | 8395 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.534 |
| walker |  | 8445 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.534 |
| walker |  | 8495 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.534 |
| walker |  | 8545 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.534 |
| ns | 8557 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.530 |
| walker |  | 8595 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.530 |
| walker |  | 8645 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.530 |
| walker |  | 8695 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.530 |
| ns | 8732 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.538 |
| walker |  | 8745 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.538 |
| walker |  | 8795 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.538 |
| walker |  | 8845 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.538 |
| walker |  | 8895 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.538 |
| walker |  | 8945 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.538 |
| ns | 8964 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.549 |
| walker |  | 8995 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.549 |
| walker |  | 9058 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.549 |
| walker |  | 9127 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.549 |
| walker |  | 9140 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.549 |
| walker |  | 9229 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.550 |
| walker |  | 9229 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.550 |
| walker |  | 9229 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.550 |
| walker |  | 9260 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.551 |
| walker |  | 9260 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.551 |
| walker |  | 9294 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.551 |
| walker |  | 9314 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.551 |
| walker |  | 9418 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.561 |
| ns | 9435 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.572 |
| walker |  | 9440 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.572 |
| walker |  | 9556 | 116 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.573 |
| walker |  | 9556 | 0 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.573 |
| ns | 9625 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.579 |
| walker |  | 9628 | 72 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.589 |
| walker |  | 9757 | 129 | python decl doc at src/typeguard/_checkers.py:1099 |  |  | 0.589 |
| ns | 9826 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.583 |
| walker |  | 9833 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.587 |
| ns | 9862 |  | 36 | GitHub workflows and repository meta files | 7.3 |  | 0.590 |
| walker |  | 9896 | 63 | python imports in src/typeguard/_config.py |  |  | 0.590 |
| ns | 9918 |  | 56 | CI interpreter matrix | 7.4 |  | 0.590 |
