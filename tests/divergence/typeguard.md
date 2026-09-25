Score(3000)=0.640 I=0.892 C=0.459 ns_rows≤3K=17/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.705/0.902/0.729/0.640/0.604/0.501/0.549

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repository root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 79 | 41 | listing of 'docs' |  |  | 1.000 |
| ns | 140 |  | 106 | README identity paragraph | 1.2 |  | 0.771 |
| walker |  | 153 | 74 | listing of 'src/typeguard' |  |  | 0.816 |
| ns | 218 |  | 78 | Package module roster: src/typeguard/ | 1.3 |  | 0.867 |
| ns | 313 |  | 95 | README: the two principal checking modes | 1.4 |  | 0.746 |
| ns | 419 |  | 106 | README: what instrumentation actually covers | 1.5 | 1.4 | 0.680 |
| ns | 552 |  | 133 | README: the two instrumentation entry points | 1.6 |  | 0.591 |
| walker |  | 574 | 421 | python imports in src/typeguard/__init__.py |  |  | 0.621 |
| walker |  | 612 | 38 | python decl names surface in src/typeguard/__init__.py |  |  | 0.621 |
| walker |  | 612 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.621 |
| walker |  | 667 | 55 | [dependencies] in pyproject.toml |  |  | 0.621 |
| walker |  | 694 | 27 | listing of '.github' |  |  | 0.622 |
| walker |  | 702 | 8 | listing of '.github/workflows' |  |  | 0.622 |
| ns | 748 |  | 196 | Public exports of typeguard/__init__.py, first half | 1.7 |  | 0.674 |
| walker |  | 772 | 70 | [package] in pyproject.toml |  |  | 0.675 |
| walker |  | 807 | 35 | package metadata in pyproject.toml |  |  | 0.675 |
| ns | 954 |  | 206 | Public exports of typeguard/__init__.py, second half | 1.8 | 1.7 | 0.705 |
| ns | 1109 |  | 155 | Docs and test tree listings | 1.9 |  | 0.605 |
| walker |  | 1274 | 467 | README headline in README.rst |  |  | 0.854 |
| ns | 1339 |  | 230 | __init__.py package-level machinery | 1.10 | 1.8 | 0.769 |
| walker |  | 1374 | 100 | listing of 'tests' |  |  | 0.872 |
| walker |  | 1388 | 14 | listing of 'tests/mypy' |  |  | 0.902 |
| ns | 1473 |  | 134 | docs/index.rst in full | 1.11 |  | 0.833 |
| walker |  | 1578 | 190 | manifest config in pyproject.toml |  |  | 0.834 |
| ns | 1636 |  | 163 | check_type(): real signature and the TypeCheckFailCallback alias | 2.1 |  | 0.792 |
| ns | 1752 |  | 116 | @typechecked: implementation signature | 2.2 |  | 0.769 |
| walker |  | 1784 | 206 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.770 |
| walker |  | 1853 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.784 |
| walker |  | 1874 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.784 |
| walker |  | 1953 | 79 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.785 |
| walker |  | 1953 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.785 |
| walker |  | 1990 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.785 |
| walker |  | 2040 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.785 |
| ns | 2059 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.729 |
| walker |  | 2090 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.729 |
| walker |  | 2140 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.729 |
| walker |  | 2190 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.729 |
| walker |  | 2240 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.729 |
| walker |  | 2253 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.729 |
| walker |  | 2253 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.729 |
| walker |  | 2276 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.729 |
| walker |  | 2287 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.729 |
| walker |  | 2358 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.730 |
| walker |  | 2407 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.730 |
| walker |  | 2407 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.730 |
| walker |  | 2407 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.730 |
| walker |  | 2407 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.730 |
| walker |  | 2407 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.730 |
| walker |  | 2428 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.731 |
| walker |  | 2450 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.731 |
| ns | 2451 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.674 |
| walker |  | 2482 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.674 |
| walker |  | 2524 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.675 |
| walker |  | 2605 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.676 |
| walker |  | 2605 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.676 |
| walker |  | 2605 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.676 |
| walker |  | 2605 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.676 |
| walker |  | 2605 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.676 |
| walker |  | 2605 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.676 |
| ns | 2613 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.651 |
| walker |  | 2614 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.651 |
| walker |  | 2623 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.652 |
| walker |  | 2645 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.652 |
| walker |  | 2654 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.652 |
| walker |  | 2721 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.652 |
| ns | 2737 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.633 |
| walker |  | 2745 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.634 |
| walker |  | 2745 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.634 |
| walker |  | 2773 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.634 |
| walker |  | 2773 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.634 |
| walker |  | 2807 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.635 |
| walker |  | 2875 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.636 |
| walker |  | 2875 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.636 |
| walker |  | 2875 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.636 |
| walker |  | 2884 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.637 |
| walker |  | 2906 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.638 |
| walker |  | 2934 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.640 |
| walker |  | 2956 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.640 |
| ns | 3035 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.599 |
| walker |  | 3040 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.625 |
| walker |  | 3059 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.626 |
| walker |  | 3059 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.626 |
| walker |  | 3172 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.660 |
| walker |  | 3217 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.660 |
| ns | 3264 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.634 |
| walker |  | 3389 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.635 |
| walker |  | 3389 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.635 |
| walker |  | 3389 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.635 |
| walker |  | 3389 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.635 |
| walker |  | 3389 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.635 |
| walker |  | 3397 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.635 |
| walker |  | 3419 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.635 |
| ns | 3432 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.618 |
| walker |  | 3443 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.618 |
| walker |  | 3469 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.618 |
| walker |  | 3495 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.618 |
| walker |  | 3512 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.618 |
| walker |  | 3538 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.618 |
| ns | 3559 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.609 |
| ns | 3765 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.616 |
| ns | 3958 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.608 |
| ns | 4104 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.612 |
| ns | 4214 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.604 |
| ns | 4449 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.584 |
| walker |  | 4492 | 954 | python method sigs in src/typeguard/_transformer.py |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.586 |
| walker |  | 4492 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.586 |
| walker |  | 4503 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.586 |
| walker |  | 4514 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.586 |
| walker |  | 4543 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.586 |
| walker |  | 4575 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.586 |
| walker |  | 4611 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.586 |
| ns | 4612 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.586 |
| walker |  | 4624 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.586 |
| walker |  | 4652 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.586 |
| walker |  | 4652 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.586 |
| walker |  | 4680 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.586 |
| walker |  | 4680 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.586 |
| walker |  | 4697 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.586 |
| walker |  | 4720 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.586 |
| walker |  | 4763 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.586 |
| walker |  | 4896 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.586 |
| ns | 4963 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.561 |
| walker |  | 5008 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.561 |
| ns | 5222 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.544 |
| walker |  | 5363 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.544 |
| walker |  | 5519 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.544 |
| walker |  | 5562 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.544 |
| ns | 5575 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.525 |
| walker |  | 5608 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.525 |
| walker |  | 5664 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.525 |
| ns | 5758 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.518 |
| walker |  | 5794 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.519 |
| walker |  | 5794 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.519 |
| walker |  | 5840 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.519 |
| walker |  | 5887 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.519 |
| walker |  | 5935 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.519 |
| walker |  | 5983 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.519 |
| ns | 6030 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.507 |
| walker |  | 6032 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.507 |
| walker |  | 6051 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.508 |
| walker |  | 6150 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.508 |
| walker |  | 6150 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.508 |
| ns | 6174 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.501 |
| walker |  | 6248 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.502 |
| walker |  | 6248 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.502 |
| walker |  | 6314 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.514 |
| walker |  | 6439 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.536 |
| walker |  | 6461 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.537 |
| ns | 6473 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.524 |
| walker |  | 6578 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.524 |
| walker |  | 6578 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.524 |
| walker |  | 6578 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.524 |
| walker |  | 6578 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.524 |
| walker |  | 6578 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.524 |
| walker |  | 6596 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.524 |
| walker |  | 6645 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.526 |
| walker |  | 6676 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.528 |
| walker |  | 6709 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.528 |
| walker |  | 6860 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.529 |
| walker |  | 6860 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.529 |
| walker |  | 6860 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.529 |
| walker |  | 6860 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.529 |
| walker |  | 6860 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.529 |
| walker |  | 6860 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.529 |
| walker |  | 6872 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.529 |
| walker |  | 6901 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.529 |
| walker |  | 6906 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.529 |
| walker |  | 6958 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.529 |
| ns | 6981 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.509 |
| walker |  | 7014 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.509 |
| walker |  | 7090 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.509 |
| ns | 7158 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.502 |
| walker |  | 7199 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.519 |
| walker |  | 7259 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.519 |
| walker |  | 7320 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.520 |
| walker |  | 7368 | 48 | tool.mypy config in pyproject.toml |  |  | 0.520 |
| walker |  | 7404 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.521 |
| walker |  | 7404 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.521 |
| walker |  | 7404 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.521 |
| ns | 7525 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.507 |
| walker |  | 7535 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.508 |
| walker |  | 7535 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.508 |
| walker |  | 7535 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.508 |
| walker |  | 7535 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.508 |
| walker |  | 7535 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.508 |
| walker |  | 7535 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.508 |
| walker |  | 7535 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.508 |
| walker |  | 7543 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.508 |
| walker |  | 7553 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.508 |
| walker |  | 7567 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.508 |
| walker |  | 7567 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.508 |
| walker |  | 7574 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.508 |
| ns | 7601 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.513 |
| walker |  | 7647 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.513 |
| walker |  | 7728 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.513 |
| ns | 7732 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.510 |
| ns | 7878 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.519 |
| walker |  | 8076 | 348 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.550 |
| walker |  | 8076 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.550 |
| walker |  | 8076 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.550 |
| walker |  | 8100 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.550 |
| walker |  | 8127 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.550 |
| walker |  | 8165 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.540 |
| ns | 8165 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.540 |
| walker |  | 8196 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.540 |
| walker |  | 8246 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.540 |
| walker |  | 8296 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.540 |
| ns | 8324 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.534 |
| walker |  | 8346 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.534 |
| walker |  | 8396 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.534 |
| walker |  | 8446 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.534 |
| walker |  | 8496 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.534 |
| walker |  | 8546 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.534 |
| ns | 8556 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.530 |
| walker |  | 8596 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.530 |
| walker |  | 8646 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.530 |
| walker |  | 8696 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.530 |
| ns | 8731 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.538 |
| walker |  | 8746 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.538 |
| walker |  | 8796 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.538 |
| walker |  | 8846 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.538 |
| walker |  | 8896 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.538 |
| walker |  | 8946 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.538 |
| ns | 8963 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.549 |
| walker |  | 8996 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.549 |
| walker |  | 9059 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.549 |
| walker |  | 9128 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.549 |
| walker |  | 9217 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.550 |
| walker |  | 9217 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.550 |
| walker |  | 9217 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.550 |
| walker |  | 9248 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.551 |
| walker |  | 9248 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.551 |
| walker |  | 9282 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.551 |
| walker |  | 9302 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.551 |
| walker |  | 9406 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.561 |
| walker |  | 9428 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.561 |
| ns | 9434 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.572 |
| walker |  | 9544 | 116 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.573 |
| walker |  | 9544 | 0 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.573 |
| walker |  | 9616 | 72 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.583 |
| ns | 9624 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.589 |
| walker |  | 9745 | 129 | python decl doc at src/typeguard/_checkers.py:1099 |  |  | 0.589 |
| walker |  | 9759 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.589 |
| ns | 9825 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.583 |
| walker |  | 9835 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.587 |
| ns | 9860 |  | 35 | GitHub workflows and repository meta files | 7.3 |  | 0.590 |
| walker |  | 9898 | 63 | python imports in src/typeguard/_config.py |  |  | 0.590 |
| ns | 9916 |  | 56 | CI interpreter matrix | 7.4 |  | 0.590 |
