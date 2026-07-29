Score(3000)=0.636 I=0.891 C=0.454 ns_rows≤3K=17/50 (reached=9 partial=0 missing=8)

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
| walker |  | 2361 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.729 |
| walker |  | 2374 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.729 |
| walker |  | 2374 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.729 |
| walker |  | 2397 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.729 |
| walker |  | 2408 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.729 |
| ns | 2452 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.672 |
| walker |  | 2479 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.673 |
| walker |  | 2528 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.673 |
| walker |  | 2528 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.673 |
| walker |  | 2528 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.673 |
| walker |  | 2528 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.673 |
| walker |  | 2528 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.673 |
| walker |  | 2549 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.673 |
| walker |  | 2571 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.673 |
| walker |  | 2603 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.674 |
| ns | 2614 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.649 |
| walker |  | 2645 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.650 |
| walker |  | 2726 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.651 |
| walker |  | 2726 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.651 |
| walker |  | 2726 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.651 |
| walker |  | 2726 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.651 |
| walker |  | 2726 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.651 |
| walker |  | 2726 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.651 |
| walker |  | 2735 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.651 |
| ns | 2738 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.632 |
| walker |  | 2744 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.633 |
| walker |  | 2766 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.633 |
| walker |  | 2775 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.633 |
| walker |  | 2842 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.633 |
| walker |  | 2866 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.633 |
| walker |  | 2866 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.633 |
| walker |  | 2894 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.634 |
| walker |  | 2894 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.634 |
| walker |  | 2928 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.634 |
| walker |  | 2996 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.636 |
| walker |  | 2996 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.636 |
| walker |  | 2996 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.636 |
| walker |  | 3005 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.637 |
| walker |  | 3027 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.638 |
| ns | 3036 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.596 |
| walker |  | 3055 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.598 |
| walker |  | 3077 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.599 |
| walker |  | 3161 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.625 |
| walker |  | 3180 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.626 |
| walker |  | 3180 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.626 |
| ns | 3265 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.601 |
| walker |  | 3293 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.634 |
| walker |  | 3338 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.634 |
| ns | 3433 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.617 |
| walker |  | 3510 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.617 |
| walker |  | 3510 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.617 |
| walker |  | 3510 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.617 |
| walker |  | 3510 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.617 |
| walker |  | 3510 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.617 |
| walker |  | 3518 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.618 |
| walker |  | 3540 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.618 |
| ns | 3560 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.608 |
| walker |  | 3564 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.608 |
| walker |  | 3590 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.608 |
| walker |  | 3616 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.608 |
| walker |  | 3633 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.608 |
| walker |  | 3659 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.608 |
| ns | 3766 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.616 |
| ns | 3959 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.608 |
| ns | 4105 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.612 |
| ns | 4215 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.604 |
| ns | 4450 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.584 |
| walker |  | 4613 | 954 | python method sigs in src/typeguard/_transformer.py |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.586 |
| walker |  | 4613 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.586 |
| ns | 4613 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.586 |
| walker |  | 4624 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.586 |
| walker |  | 4635 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.586 |
| walker |  | 4664 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.586 |
| walker |  | 4696 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.586 |
| walker |  | 4732 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.586 |
| walker |  | 4745 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.586 |
| walker |  | 4773 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.586 |
| walker |  | 4773 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.586 |
| walker |  | 4801 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.586 |
| walker |  | 4801 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.586 |
| walker |  | 4818 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.586 |
| walker |  | 4841 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.586 |
| walker |  | 4884 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.586 |
| ns | 4964 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.561 |
| walker |  | 5017 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.561 |
| walker |  | 5129 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.561 |
| ns | 5223 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.544 |
| walker |  | 5484 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.544 |
| ns | 5576 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.524 |
| walker |  | 5640 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.524 |
| walker |  | 5683 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.525 |
| walker |  | 5729 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.525 |
| ns | 5759 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.517 |
| walker |  | 5785 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.517 |
| walker |  | 5915 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.519 |
| walker |  | 5915 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.519 |
| walker |  | 5961 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.519 |
| walker |  | 6008 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.519 |
| ns | 6031 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.506 |
| walker |  | 6056 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.506 |
| walker |  | 6104 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.506 |
| walker |  | 6153 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.506 |
| walker |  | 6172 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.508 |
| ns | 6175 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.501 |
| walker |  | 6271 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.501 |
| walker |  | 6271 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.501 |
| walker |  | 6369 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.501 |
| walker |  | 6369 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.501 |
| walker |  | 6435 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.514 |
| ns | 6474 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.500 |
| walker |  | 6560 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.521 |
| walker |  | 6582 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.521 |
| walker |  | 6699 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.522 |
| walker |  | 6699 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.522 |
| walker |  | 6699 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.522 |
| walker |  | 6699 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.522 |
| walker |  | 6699 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.522 |
| walker |  | 6717 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.522 |
| walker |  | 6766 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.524 |
| walker |  | 6797 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.526 |
| walker |  | 6830 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.526 |
| walker |  | 6981 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.526 |
| walker |  | 6981 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.526 |
| walker |  | 6981 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.526 |
| walker |  | 6981 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.526 |
| walker |  | 6981 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.526 |
| walker |  | 6981 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.526 |
| ns | 6982 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.507 |
| walker |  | 6993 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.507 |
| walker |  | 7022 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.507 |
| walker |  | 7027 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.507 |
| walker |  | 7079 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.507 |
| walker |  | 7135 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.507 |
| ns | 7159 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.498 |
| walker |  | 7211 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.498 |
| walker |  | 7320 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.516 |
| walker |  | 7380 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.516 |
| walker |  | 7441 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.517 |
| walker |  | 7489 | 48 | tool.mypy config in pyproject.toml |  |  | 0.517 |
| walker |  | 7525 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.517 |
| walker |  | 7525 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.517 |
| walker |  | 7525 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.517 |
| ns | 7526 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.504 |
| ns | 7602 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.510 |
| walker |  | 7656 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.510 |
| walker |  | 7656 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.510 |
| walker |  | 7656 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.510 |
| walker |  | 7656 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.510 |
| walker |  | 7656 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.510 |
| walker |  | 7656 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.510 |
| walker |  | 7656 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.510 |
| walker |  | 7664 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.510 |
| walker |  | 7674 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.510 |
| walker |  | 7688 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.510 |
| walker |  | 7688 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.510 |
| walker |  | 7695 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.510 |
| ns | 7733 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.507 |
| walker |  | 7768 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.507 |
| walker |  | 7849 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.507 |
| ns | 7879 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.516 |
| walker |  | 7938 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.517 |
| walker |  | 7938 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.517 |
| walker |  | 7938 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.517 |
| walker |  | 7969 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.518 |
| walker |  | 7969 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.518 |
| walker |  | 8003 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.518 |
| walker |  | 8023 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.518 |
| walker |  | 8127 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.530 |
| walker |  | 8149 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.530 |
| ns | 8166 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.520 |
| walker |  | 8265 | 116 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.521 |
| walker |  | 8265 | 0 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.521 |
| ns | 8325 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.515 |
| walker |  | 8337 | 72 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.515 |
| walker |  | 8413 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.520 |
| walker |  | 8476 | 63 | python imports in src/typeguard/_config.py |  |  | 0.520 |
| ns | 8557 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.516 |
| walker |  | 8651 | 175 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.534 |
| ns | 8732 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.542 |
| ns | 8964 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.553 |
| walker |  | 9078 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.582 |
| walker |  | 9078 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.582 |
| walker |  | 9078 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.582 |
| walker |  | 9078 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.582 |
| walker |  | 9102 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.582 |
| walker |  | 9129 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.582 |
| walker |  | 9166 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.582 |
| walker |  | 9204 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.583 |
| walker |  | 9235 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.583 |
| walker |  | 9285 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.583 |
| walker |  | 9335 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.583 |
| walker |  | 9385 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.583 |
| walker |  | 9435 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.602 |
| ns | 9435 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.602 |
| walker |  | 9485 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.602 |
| walker |  | 9535 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.602 |
| walker |  | 9585 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.602 |
| ns | 9625 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.607 |
| walker |  | 9635 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.607 |
| walker |  | 9685 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.607 |
| walker |  | 9735 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.607 |
| walker |  | 9785 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.607 |
| ns | 9826 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.601 |
| walker |  | 9835 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.601 |
| ns | 9862 |  | 36 | GitHub workflows and repository meta files | 7.3 |  | 0.604 |
| walker |  | 9885 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.604 |
| ns | 9918 |  | 56 | CI interpreter matrix | 7.4 |  | 0.605 |
| walker |  | 9935 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.605 |
| walker |  | 9985 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.605 |
