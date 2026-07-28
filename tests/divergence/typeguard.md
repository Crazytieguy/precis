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
| walker |  | 2945 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.630 |
| walker |  | 2945 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.630 |
| walker |  | 2968 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.630 |
| walker |  | 2979 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.630 |
| ns | 3036 |  | 298 | CollectionCheckStrategy enum and iterate_samples() | 2.7 |  | 0.588 |
| walker |  | 3050 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.589 |
| walker |  | 3099 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.589 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.589 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.589 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.589 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.589 |
| walker |  | 3120 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.589 |
| walker |  | 3142 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.590 |
| walker |  | 3174 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.590 |
| walker |  | 3216 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.590 |
| ns | 3265 |  | 229 | install_import_hook(): signature and docstring | 2.8 |  | 0.567 |
| walker |  | 3297 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.567 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.567 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.567 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.567 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.567 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.567 |
| walker |  | 3306 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.568 |
| walker |  | 3315 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.568 |
| walker |  | 3328 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.568 |
| walker |  | 3395 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.568 |
| walker |  | 3419 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.568 |
| walker |  | 3419 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.568 |
| ns | 3433 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.553 |
| walker |  | 3447 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.553 |
| walker |  | 3447 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.553 |
| walker |  | 3481 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.554 |
| walker |  | 3549 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.555 |
| walker |  | 3549 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.555 |
| walker |  | 3549 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.555 |
| walker |  | 3558 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.556 |
| ns | 3560 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.548 |
| walker |  | 3580 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.549 |
| walker |  | 3608 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.550 |
| walker |  | 3630 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.551 |
| walker |  | 3714 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.575 |
| walker |  | 3733 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.577 |
| walker |  | 3733 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.577 |
| walker |  | 3755 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.577 |
| ns | 3766 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.586 |
| walker |  | 3927 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.587 |
| walker |  | 3927 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.587 |
| walker |  | 3927 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.587 |
| walker |  | 3927 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.587 |
| walker |  | 3927 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.587 |
| walker |  | 3935 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.587 |
| walker |  | 3957 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.587 |
| ns | 3959 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.578 |
| walker |  | 3974 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.578 |
| walker |  | 3998 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.578 |
| walker |  | 4024 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.578 |
| walker |  | 4050 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.578 |
| walker |  | 4076 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.578 |
| ns | 4105 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.584 |
| walker |  | 4190 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.584 |
| ns | 4215 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.576 |
| ns | 4450 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.557 |
| walker |  | 4547 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.557 |
| walker |  | 4556 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.559 |
| ns | 4613 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.561 |
| walker |  | 4686 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.562 |
| walker |  | 4686 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.562 |
| walker |  | 4732 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.562 |
| walker |  | 4779 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.562 |
| walker |  | 4827 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.562 |
| walker |  | 4875 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.562 |
| walker |  | 4924 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.562 |
| walker |  | 4943 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.564 |
| ns | 4964 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.540 |
| walker |  | 5042 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.540 |
| walker |  | 5042 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.540 |
| walker |  | 5140 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.540 |
| walker |  | 5140 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.540 |
| walker |  | 5206 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.555 |
| ns | 5223 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.538 |
| walker |  | 5331 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.562 |
| walker |  | 5353 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.562 |
| walker |  | 5470 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.563 |
| walker |  | 5470 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.563 |
| walker |  | 5470 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.563 |
| walker |  | 5470 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.563 |
| walker |  | 5470 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.563 |
| walker |  | 5488 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.563 |
| walker |  | 5537 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.565 |
| walker |  | 5568 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.567 |
| ns | 5576 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.548 |
| walker |  | 5601 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.548 |
| walker |  | 5752 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.548 |
| walker |  | 5752 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.548 |
| walker |  | 5752 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.548 |
| walker |  | 5752 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.548 |
| walker |  | 5752 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.548 |
| walker |  | 5752 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.548 |
| ns | 5759 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.540 |
| walker |  | 5764 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.540 |
| walker |  | 5793 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.540 |
| walker |  | 5798 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.540 |
| walker |  | 5850 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.540 |
| walker |  | 5906 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.540 |
| walker |  | 5982 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.540 |
| ns | 6031 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.527 |
| walker |  | 6091 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.547 |
| ns | 6175 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.540 |
| walker |  | 6204 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.560 |
| walker |  | 6249 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.560 |
| walker |  | 6380 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.560 |
| walker |  | 6380 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.560 |
| walker |  | 6380 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.560 |
| walker |  | 6380 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.560 |
| walker |  | 6380 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.560 |
| walker |  | 6380 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.560 |
| walker |  | 6380 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.560 |
| walker |  | 6388 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.560 |
| walker |  | 6398 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.560 |
| walker |  | 6412 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.560 |
| walker |  | 6412 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.560 |
| walker |  | 6419 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.560 |
| ns | 6474 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.544 |
| walker |  | 6492 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.544 |
| walker |  | 6573 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.544 |
| walker |  | 6609 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.545 |
| walker |  | 6609 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.545 |
| walker |  | 6609 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.545 |
| walker |  | 6698 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.545 |
| walker |  | 6698 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.545 |
| walker |  | 6698 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.545 |
| walker |  | 6729 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.545 |
| walker |  | 6729 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.545 |
| walker |  | 6763 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.545 |
| walker |  | 6783 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.545 |
| walker |  | 6887 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.558 |
| walker |  | 6909 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.558 |
| ns | 6982 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.537 |
| walker |  | 7025 | 116 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.537 |
| walker |  | 7025 | 0 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.537 |
| ns | 7159 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.528 |
| ns | 7526 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.515 |
| ns | 7602 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.520 |
| ns | 7733 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.520 |
| ns | 7879 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.529 |
| walker |  | 7975 | 950 | python method sigs in src/typeguard/_transformer.py |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.530 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.530 |
| walker |  | 7986 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.530 |
| walker |  | 7997 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.530 |
| walker |  | 8010 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.530 |
| walker |  | 8039 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.530 |
| walker |  | 8071 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.530 |
| walker |  | 8088 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.530 |
| walker |  | 8124 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.530 |
| walker |  | 8147 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.530 |
| ns | 8166 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.520 |
| walker |  | 8175 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.520 |
| walker |  | 8175 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.520 |
| walker |  | 8203 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.520 |
| walker |  | 8203 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.520 |
| walker |  | 8246 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.521 |
| walker |  | 8292 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.521 |
| ns | 8325 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.514 |
| walker |  | 8348 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.515 |
| walker |  | 8408 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.515 |
| walker |  | 8469 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.515 |
| walker |  | 8512 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.515 |
| ns | 8557 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.512 |
| ns | 8732 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.521 |
| walker |  | 8939 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.551 |
| walker |  | 8939 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.551 |
| walker |  | 8939 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.551 |
| walker |  | 8939 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.551 |
| walker |  | 8963 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.551 |
| ns | 8964 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.562 |
| walker |  | 8990 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.562 |
| walker |  | 9027 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.563 |
| walker |  | 9065 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.563 |
| walker |  | 9096 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.563 |
| walker |  | 9146 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.563 |
| walker |  | 9196 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.563 |
| walker |  | 9246 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.563 |
| walker |  | 9296 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.563 |
| walker |  | 9346 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.563 |
| walker |  | 9396 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.563 |
| ns | 9435 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.574 |
| walker |  | 9446 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.574 |
| walker |  | 9496 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.574 |
| walker |  | 9546 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.574 |
| walker |  | 9596 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.574 |
| ns | 9625 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.580 |
| walker |  | 9646 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.580 |
| walker |  | 9696 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.580 |
| walker |  | 9746 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.580 |
| walker |  | 9796 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.580 |
| ns | 9826 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.588 |
| walker |  | 9846 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.588 |
| ns | 9862 |  | 36 | GitHub workflows and repository meta files | 7.3 |  | 0.592 |
| walker |  | 9896 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.592 |
| ns | 9918 |  | 56 | CI interpreter matrix | 7.4 |  | 0.592 |
| walker |  | 9946 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.592 |
| walker |  | 9996 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.592 |
