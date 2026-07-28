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
| walker |  | 1944 | 147 | dev/build/target dependencies in pyproject.toml |  |  | 0.657 |
| ns | 2060 |  | 307 | check_type() docstring: semantics and the global-config exemption | 2.3 | 2.1 | 0.610 |
| walker |  | 2132 | 188 | manifest config in pyproject.toml |  |  | 0.611 |
| walker |  | 2321 | 189 | tool.tox config in pyproject.toml |  |  | 0.611 |
| ns | 2452 |  | 392 | @typechecked docstring: what it instruments and the -O no-op | 2.4 | 2.2 | 0.563 |
| walker |  | 2525 | 204 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.563 |
| ns | 2614 |  | 162 | ForwardRefPolicy enum with member docs | 2.5 |  | 0.543 |
| walker |  | 2729 | 204 | tool.ruff config in pyproject.toml |  |  | 0.543 |
| ns | 2738 |  | 124 | TypeCheckConfiguration fields and the global_config singleton | 2.6 |  | 0.527 |
| walker |  | 2829 | 100 | listing of 'tests' |  |  | 0.598 |
| walker |  | 2842 | 13 | listing of 'tests/mypy' |  |  | 0.618 |
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
| walker |  | 3337 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.568 |
| walker |  | 3346 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.568 |
| walker |  | 3413 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.569 |
| ns | 3433 |  | 168 | TypeguardFinder and ImportHookManager class docstrings | 2.9 |  | 0.553 |
| walker |  | 3437 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.553 |
| walker |  | 3437 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.553 |
| walker |  | 3465 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.554 |
| walker |  | 3465 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.554 |
| walker |  | 3499 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.554 |
| ns | 3560 |  | 127 | Complete roster of pytest plugin command-line flags | 2.10 |  | 0.546 |
| walker |  | 3567 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.547 |
| walker |  | 3567 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.547 |
| walker |  | 3567 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.547 |
| walker |  | 3576 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.548 |
| walker |  | 3598 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.549 |
| walker |  | 3626 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.551 |
| walker |  | 3648 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.552 |
| walker |  | 3732 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.576 |
| walker |  | 3751 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.577 |
| walker |  | 3751 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.577 |
| ns | 3766 |  | 206 | Warning classes: TypeHintWarning, TypeCheckWarning, InstrumentationWarning | 2.11 |  | 0.587 |
| walker |  | 3864 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.616 |
| walker |  | 3909 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.616 |
| ns | 3959 |  | 193 | TypeCheckError and its path-accumulation machinery | 2.12 |  | 0.608 |
| walker |  | 4081 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.609 |
| walker |  | 4081 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.609 |
| walker |  | 4081 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.609 |
| walker |  | 4081 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.609 |
| walker |  | 4081 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.609 |
| walker |  | 4089 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.609 |
| ns | 4105 |  | 146 | suppress_type_checks(): module state, overloads and signature | 2.13 |  | 0.613 |
| walker |  | 4111 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.613 |
| walker |  | 4128 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.613 |
| walker |  | 4152 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.613 |
| walker |  | 4178 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.613 |
| walker |  | 4204 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.613 |
| ns | 4215 |  | 110 | warn_on_error(): the stock fail callback | 2.14 |  | 0.605 |
| walker |  | 4230 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.605 |
| walker |  | 4344 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.605 |
| ns | 4450 |  | 235 | suppress_type_checks() docstring | 2.15 | 2.13 | 0.585 |
| ns | 4613 |  | 163 | TypeCheckMemo: __slots__ and constructor | 2.16 |  | 0.585 |
| walker |  | 4701 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.585 |
| walker |  | 4831 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.587 |
| walker |  | 4831 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.587 |
| walker |  | 4877 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.587 |
| walker |  | 4924 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.587 |
| ns | 4964 |  | 351 | TypeCheckConfiguration attribute documentation | 2.17 | 2.6 | 0.562 |
| walker |  | 4972 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.562 |
| walker |  | 5020 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.562 |
| walker |  | 5069 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.562 |
| walker |  | 5088 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.563 |
| walker |  | 5187 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.563 |
| walker |  | 5187 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.563 |
| ns | 5223 |  | 259 | TypeCheckMemo attribute documentation | 2.18 | 2.16 | 0.546 |
| walker |  | 5285 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.546 |
| walker |  | 5285 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.546 |
| walker |  | 5351 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.560 |
| walker |  | 5476 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.584 |
| walker |  | 5498 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.584 |
| ns | 5576 |  | 353 | pytest plugin flag help texts and choices | 2.19 |  | 0.564 |
| walker |  | 5615 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.564 |
| walker |  | 5615 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.564 |
| walker |  | 5615 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.564 |
| walker |  | 5615 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.564 |
| walker |  | 5615 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.564 |
| walker |  | 5633 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.564 |
| walker |  | 5682 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.566 |
| walker |  | 5713 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.568 |
| walker |  | 5746 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.568 |
| ns | 5759 |  | 183 | Complete section-heading roster of docs/features.rst | 3.1 |  | 0.560 |
| walker |  | 5897 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.561 |
| walker |  | 5897 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.561 |
| walker |  | 5897 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.561 |
| walker |  | 5897 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.561 |
| walker |  | 5897 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.561 |
| walker |  | 5897 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.561 |
| walker |  | 5909 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.561 |
| walker |  | 5938 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.561 |
| walker |  | 5943 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.561 |
| walker |  | 5995 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.561 |
| ns | 6031 |  | 272 | features.rst: the checked and explicitly-unchecked lists | 3.2 | 3.1 | 0.548 |
| walker |  | 6051 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.548 |
| walker |  | 6127 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.548 |
| ns | 6175 |  | 144 | Complete section-heading roster of docs/userguide.rst | 3.3 |  | 0.541 |
| walker |  | 6236 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.560 |
| walker |  | 6249 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.560 |
| walker |  | 6285 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.560 |
| walker |  | 6285 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.560 |
| walker |  | 6285 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.560 |
| walker |  | 6416 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.561 |
| walker |  | 6416 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.561 |
| walker |  | 6416 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.561 |
| walker |  | 6416 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.561 |
| walker |  | 6416 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.561 |
| walker |  | 6416 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.561 |
| walker |  | 6416 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.561 |
| walker |  | 6424 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.561 |
| walker |  | 6434 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.561 |
| walker |  | 6448 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.561 |
| walker |  | 6448 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.561 |
| walker |  | 6455 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.561 |
| ns | 6474 |  | 299 | Complete roster of check_* functions in _checkers.py | 4.1 |  | 0.545 |
| walker |  | 6528 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.545 |
| walker |  | 6609 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.545 |
| ns | 6982 |  | 508 | origin_type_checkers: the annotation-to-checker dispatch table | 4.2 |  | 0.524 |
| ns | 7159 |  | 177 | check_type_internal(): signature and contract | 4.3 | 4.1 | 0.515 |
| ns | 7526 |  | 367 | builtin_checker_lookup(): the structural fallback chain | 4.4 | 4.1 | 0.502 |
| walker |  | 7559 | 950 | python method sigs in src/typeguard/_transformer.py |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.504 |
| walker |  | 7559 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.504 |
| walker |  | 7570 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.504 |
| walker |  | 7581 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.504 |
| walker |  | 7594 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.504 |
| ns | 7602 |  | 76 | Roster of the runtime helpers instrumented code calls | 5.1 |  | 0.509 |
| walker |  | 7623 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.509 |
| walker |  | 7655 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.509 |
| walker |  | 7672 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.509 |
| walker |  | 7708 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.509 |
| walker |  | 7731 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.509 |
| ns | 7733 |  | 131 | Roster of _decorators.py module-level functions | 5.2 |  | 0.506 |
| walker |  | 7759 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.506 |
| walker |  | 7759 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.506 |
| walker |  | 7787 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.506 |
| walker |  | 7787 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.506 |
| walker |  | 7830 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.506 |
| walker |  | 7876 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.507 |
| ns | 7879 |  | 146 | Roster of _importhook.py symbols | 5.3 |  | 0.515 |
| walker |  | 7932 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.516 |
| walker |  | 7992 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.516 |
| walker |  | 8053 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.517 |
| walker |  | 8096 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.517 |
| ns | 8166 |  | 287 | instrument(): the refusal reasons and the recompile pipeline | 5.4 | 5.2 | 0.507 |
| walker |  | 8185 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.508 |
| walker |  | 8185 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.508 |
| walker |  | 8185 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.508 |
| walker |  | 8216 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.510 |
| walker |  | 8216 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.510 |
| walker |  | 8250 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.510 |
| walker |  | 8270 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.510 |
| ns | 8325 |  | 159 | TypeguardFinder.should_instrument(): the module-selection rule | 5.5 | 5.3 | 0.503 |
| walker |  | 8374 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.514 |
| walker |  | 8396 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.514 |
| walker |  | 8512 | 116 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.515 |
| walker |  | 8512 | 0 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.515 |
| ns | 8557 |  | 232 | Roster of _utils.py helpers, including the version-gated evaluate_forwardref | 5.6 |  | 0.512 |
| walker |  | 8584 | 72 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.512 |
| walker |  | 8717 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.512 |
| ns | 8732 |  | 175 | _transformer.py top-level structure: constants and the four visitor classes | 6.1 |  | 0.522 |
| walker |  | 8793 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.526 |
| walker |  | 8856 | 63 | python imports in src/typeguard/_config.py |  |  | 0.526 |
| ns | 8964 |  | 232 | Complete method roster of TypeguardTransformer | 6.2 |  | 0.538 |
| walker |  | 9031 | 175 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.554 |
| ns | 9435 |  | 471 | Docstrings of the TypeguardTransformer visit_ handlers | 6.3 | 6.2 | 0.575 |
| walker |  | 9458 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.602 |
| walker |  | 9458 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.602 |
| walker |  | 9458 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.602 |
| walker |  | 9458 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.602 |
| walker |  | 9482 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.602 |
| walker |  | 9509 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.602 |
| walker |  | 9546 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.602 |
| walker |  | 9584 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.603 |
| walker |  | 9615 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.603 |
| ns | 9625 |  | 190 | pyproject.toml: package identity, runtime requirements, pytest entry point | 7.1 |  | 0.608 |
| walker |  | 9665 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.608 |
| walker |  | 9715 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.608 |
| walker |  | 9765 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.608 |
| walker |  | 9815 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.608 |
| ns | 9826 |  | 201 | pyproject.toml: dependency groups and pytest configuration | 7.2 |  | 0.615 |
| ns | 9862 |  | 36 | GitHub workflows and repository meta files | 7.3 |  | 0.618 |
| walker |  | 9865 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.618 |
| walker |  | 9915 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.618 |
| ns | 9918 |  | 56 | CI interpreter matrix | 7.4 |  | 0.619 |
| walker |  | 9965 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.619 |
