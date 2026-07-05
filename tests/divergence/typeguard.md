Score(3000)=0.659 I=0.883 C=0.491 ns_rows≤3K=13/39 (reached=7 partial=3 missing=3)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| ns | 75 |  | 41 | docs/ listing | 1.2 |  | 0.682 |
| walker |  | 93 | 55 | [dependencies] in pyproject.toml |  |  | 0.682 |
| ns | 147 |  | 72 | src/typeguard/ module listing | 1.3 |  | 0.484 |
| walker |  | 163 | 70 | [package] in pyproject.toml |  |  | 0.484 |
| walker |  | 235 | 72 | listing of 'src/typeguard' |  |  | 0.799 |
| ns | 247 |  | 100 | tests/ listing | 1.4 |  | 0.618 |
| ns | 353 |  | 106 | README — one-paragraph lede | 1.5 |  | 0.581 |
| ns | 554 |  | 201 | README — check_type vs. code instrumentation modes | 1.6 | 1.5 | 0.503 |
| walker |  | 651 | 416 | python imports in src/typeguard/__init__.py |  |  | 0.522 |
| walker |  | 679 | 28 | python decl names surface in src/typeguard/__init__.py |  |  | 0.522 |
| walker |  | 679 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.522 |
| ns | 704 |  | 150 | README — instrumentation options (@typechecked vs import hook) | 1.7 | 1.6 | 0.468 |
| walker |  | 720 | 41 | listing of 'docs' |  |  | 0.584 |
| walker |  | 733 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.584 |
| walker |  | 733 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.584 |
| walker |  | 744 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.584 |
| walker |  | 766 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.584 |
| ns | 970 |  | 266 | Public re-exports — head of typeguard/__init__.py | 1.8 |  | 0.640 |
| ns | 1125 |  | 155 | Public re-exports — tail of typeguard/__init__.py | 1.9 | 1.8 | 0.662 |
| walker |  | 1188 | 422 | README headline in README.rst |  |  | 0.832 |
| walker |  | 1211 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.832 |
| walker |  | 1270 | 59 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.834 |
| walker |  | 1283 | 13 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.834 |
| walker |  | 1319 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.834 |
| walker |  | 1319 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.834 |
| walker |  | 1319 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.834 |
| walker |  | 1340 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.834 |
| ns | 1355 |  | 230 | typeguard/__init__.py — module rewrite, lazy `config`, autoload | 1.10 | 1.9 | 0.759 |
| walker |  | 1389 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.760 |
| walker |  | 1389 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.760 |
| walker |  | 1389 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.760 |
| walker |  | 1389 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.760 |
| walker |  | 1389 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.760 |
| walker |  | 1410 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.760 |
| walker |  | 1432 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.760 |
| walker |  | 1464 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.761 |
| walker |  | 1506 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.764 |
| walker |  | 1587 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.766 |
| walker |  | 1587 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.766 |
| walker |  | 1587 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.766 |
| walker |  | 1587 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.766 |
| walker |  | 1587 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.766 |
| walker |  | 1587 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.766 |
| walker |  | 1596 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.766 |
| walker |  | 1605 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.766 |
| walker |  | 1614 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.766 |
| ns | 1673 |  | 318 | _exceptions.py — class signatures + summary docstrings | 2.1 |  | 0.730 |
| walker |  | 1714 | 100 | listing of 'tests' |  |  | 0.822 |
| walker |  | 1772 | 58 | python decl names surface in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1772 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1772 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1781 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1802 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1802 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.822 |
| walker |  | 1822 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1850 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1872 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1956 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.823 |
| walker |  | 2018 | 62 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.823 |
| walker |  | 2026 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.823 |
| walker |  | 2036 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.823 |
| walker |  | 2050 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.823 |
| walker |  | 2084 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.824 |
| walker |  | 2104 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.824 |
| walker |  | 2118 | 14 | listing of 'tests/mypy' |  |  | 0.824 |
| walker |  | 2153 | 35 | python imports in src/typeguard/_memo.py |  |  | 0.824 |
| walker |  | 2224 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.826 |
| walker |  | 2249 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.839 |
| ns | 2263 |  | 590 | check_type — primary entry-point signature | 2.2 |  | 0.728 |
| walker |  | 2352 | 103 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.730 |
| walker |  | 2478 | 126 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.730 |
| walker |  | 2528 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.730 |
| walker |  | 2578 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.730 |
| walker |  | 2628 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.730 |
| walker |  | 2678 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.730 |
| walker |  | 2728 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.730 |
| walker |  | 2778 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.730 |
| ns | 2784 |  | 521 | @typechecked — overloaded signatures | 2.3 |  | 0.659 |
| walker |  | 2828 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.659 |
| walker |  | 2878 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.659 |
| walker |  | 2928 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.659 |
| walker |  | 2978 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.659 |
| walker |  | 2994 | 16 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.659 |
| ns | 3007 |  | 223 | install_import_hook — signature + docstring | 2.4 |  | 0.631 |
| walker |  | 3044 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.631 |
| walker |  | 3107 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.631 |
| walker |  | 3142 | 35 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.631 |
| walker |  | 3272 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.631 |
| walker |  | 3272 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.631 |
| walker |  | 3291 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.631 |
| walker |  | 3337 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.631 |
| ns | 3363 |  | 356 | suppress_type_checks — overloaded signatures + docstring | 2.5 |  | 0.594 |
| walker |  | 3384 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.594 |
| walker |  | 3432 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.595 |
| walker |  | 3480 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.596 |
| walker |  | 3529 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.597 |
| walker |  | 3585 | 56 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.597 |
| walker |  | 3676 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.597 |
| walker |  | 3685 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.597 |
| walker |  | 3775 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.597 |
| walker |  | 3782 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.597 |
| ns | 3845 |  | 482 | TypeCheckConfiguration — dataclass + attribute docstrings | 2.6 |  | 0.556 |
| walker |  | 3907 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.563 |
| walker |  | 3929 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.565 |
| walker |  | 4018 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.565 |
| walker |  | 4018 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.565 |
| walker |  | 4018 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.565 |
| walker |  | 4028 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.566 |
| walker |  | 4062 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.566 |
| walker |  | 4082 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.566 |
| walker |  | 4103 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.566 |
| walker |  | 4200 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.572 |
| ns | 4210 |  | 365 | ForwardRefPolicy + CollectionCheckStrategy enums | 2.7 |  | 0.557 |
| walker |  | 4219 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.558 |
| walker |  | 4323 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.571 |
| walker |  | 4345 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.575 |
| ns | 4434 |  | 224 | TypeCheckMemo — class skeleton + __init__ | 2.8 |  | 0.576 |
| walker |  | 4462 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.576 |
| walker |  | 4462 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.576 |
| walker |  | 4462 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.576 |
| walker |  | 4462 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.576 |
| walker |  | 4462 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.576 |
| walker |  | 4480 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.576 |
| walker |  | 4511 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.576 |
| walker |  | 4560 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.579 |
| walker |  | 4593 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.579 |
| walker |  | 4604 | 11 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.579 |
| ns | 4609 |  | 175 | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | 2.9 |  | 0.565 |
| walker |  | 4751 | 147 | python method sigs in src/typeguard/_importhook.py |  |  | 0.565 |
| walker |  | 4751 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.565 |
| walker |  | 4751 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.565 |
| walker |  | 4751 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.565 |
| walker |  | 4751 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.565 |
| walker |  | 4751 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.565 |
| walker |  | 4756 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.565 |
| walker |  | 4768 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.565 |
| walker |  | 4777 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.565 |
| walker |  | 4806 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.565 |
| walker |  | 4858 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.565 |
| walker |  | 4865 | 7 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.565 |
| walker |  | 4921 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.566 |
| walker |  | 4945 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.566 |
| ns | 4972 |  | 363 | TypeguardFinder + ImportHookManager — class + key methods | 2.10 |  | 0.554 |
| walker |  | 5044 | 99 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.571 |
| walker |  | 5124 | 80 | python method at src/typeguard/_importhook.py:56 |  |  | 0.571 |
| ns | 5225 |  | 253 | warn_on_error + load_plugins — signatures | 2.11 |  | 0.562 |
| walker |  | 5255 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.563 |
| walker |  | 5255 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.563 |
| walker |  | 5255 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.563 |
| walker |  | 5255 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.563 |
| walker |  | 5255 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.563 |
| walker |  | 5255 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.563 |
| walker |  | 5255 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.563 |
| walker |  | 5263 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.563 |
| walker |  | 5273 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.563 |
| walker |  | 5287 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.563 |
| walker |  | 5287 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.563 |
| walker |  | 5294 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.563 |
| walker |  | 5357 | 63 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.563 |
| ns | 5408 |  | 183 | check_type_internal — signature + docstring | 2.12 |  | 0.552 |
| walker |  | 5428 | 71 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.552 |
| walker |  | 5484 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.552 |
| walker |  | 5550 | 66 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.568 |
| ns | 5702 |  | 294 | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | 2.13 | 2.2 | 0.590 |
| walker |  | 5704 | 154 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.591 |
| walker |  | 5704 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.591 |
| walker |  | 5704 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.591 |
| walker |  | 5704 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.591 |
| walker |  | 5704 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.591 |
| walker |  | 5712 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.591 |
| walker |  | 5729 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.591 |
| walker |  | 5751 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.591 |
| walker |  | 5775 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.591 |
| walker |  | 5801 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.591 |
| walker |  | 5827 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.591 |
| walker |  | 5853 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.591 |
| ns | 5887 |  | 185 | Unset sentinel + small _utils helpers | 2.14 |  | 0.598 |
| walker |  | 5967 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.598 |
| ns | 6165 |  | 278 | _checkers.py — every check_* function name (locations) | 3.1 | 2.9 | 0.588 |
| walker |  | 6324 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.588 |
| walker |  | 6573 | 249 | python method sigs in src/typeguard/_transformer.py |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.588 |
| walker |  | 6573 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.588 |
| walker |  | 6586 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.588 |
| walker |  | 6598 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.588 |
| walker |  | 6622 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.588 |
| walker |  | 6646 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.588 |
| walker |  | 6679 | 33 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.588 |
| walker |  | 6737 | 58 | python imports in src/typeguard/_config.py |  |  | 0.588 |
| ns | 6765 |  | 600 | _checkers.py — origin_type_checkers dispatch table | 3.2 | 3.1 | 0.561 |
| walker |  | 6892 | 155 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.571 |
| walker |  | 6930 | 38 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.571 |
| walker |  | 7063 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.573 |
| walker |  | 7103 | 40 | python method body at src/typeguard/_transformer.py:206 body 207 |  |  | 0.573 |
| ns | 7206 |  | 441 | _checkers.py — builtin_checker_lookup dispatch fallbacks | 3.3 | 3.2 | 0.554 |
| walker |  | 7250 | 147 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.571 |
| ns | 7284 |  | 78 | _transformer.py — class locations | 3.4 |  | 0.575 |
| walker |  | 7326 | 76 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.575 |
| walker |  | 7367 | 41 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.584 |
| walker |  | 7443 | 76 | python imports in src/typeguard/_suppression.py |  |  | 0.584 |
| ns | 7530 |  | 246 | _transformer.py — TypeguardTransformer visit_* methods (locations) | 3.5 | 3.4 | 0.573 |
| walker |  | 7692 | 249 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.573 |
| walker |  | 7692 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.573 |
| walker |  | 7703 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.573 |
| walker |  | 7708 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.573 |
| walker |  | 7713 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.573 |
| walker |  | 7718 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.573 |
| walker |  | 7727 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.573 |
| walker |  | 7737 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.573 |
| walker |  | 7747 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.573 |
| walker |  | 7769 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.573 |
| walker |  | 7800 | 31 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.573 |
| ns | 7817 |  | 287 | _transformer.py — generator_names + annotated_names + ignore_decorators tables | 3.6 |  | 0.590 |
| walker |  | 7834 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.590 |
| ns | 7859 |  | 42 | _decorators.py — function locations | 3.7 |  | 0.590 |
| walker |  | 7878 | 44 | python method body at src/typeguard/_transformer.py:328 body 329 |  |  | 0.590 |
| walker |  | 8034 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.590 |
| walker |  | 8082 | 48 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.590 |
| walker |  | 8090 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.590 |
| walker |  | 8141 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.600 |
| ns | 8224 |  | 365 | docs/api.rst — public API by topic group (head) | 3.8 |  | 0.579 |
| walker |  | 8341 | 200 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.600 |
| ns | 8423 |  | 199 | docs/userguide.rst — H2 section locations | 3.9 |  | 0.590 |
| walker |  | 8440 | 99 | python decl body at src/typeguard/_utils.py:142 body 143 |  |  | 0.590 |
| walker |  | 8588 | 148 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.606 |
| walker |  | 8588 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.606 |
| walker |  | 8588 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.606 |
| ns | 8622 |  | 199 | docs/features.rst — H2 section locations | 3.10 |  | 0.597 |
| walker |  | 8638 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.597 |
| walker |  | 8688 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.597 |
| walker |  | 8700 | 12 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.597 |
| walker |  | 8750 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.597 |
| walker |  | 8762 | 12 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.597 |
| walker |  | 8812 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.597 |
| walker |  | 8862 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.597 |
| walker |  | 8912 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.597 |
| ns | 8920 |  | 298 | docs/api.rst — Custom checkers / Suppression / Exceptions sections | 4.1 | 3.8 | 0.583 |
| walker |  | 8962 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.583 |
| walker |  | 9012 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.583 |
| walker |  | 9062 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.583 |
| walker |  | 9085 | 23 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.583 |
| walker |  | 9154 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.583 |
| walker |  | 9186 | 32 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.583 |
| walker |  | 9203 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.583 |
| walker |  | 9277 | 74 | python decl body at src/typeguard/_checkers.py:651 body 657 |  |  | 0.583 |
| walker |  | 9293 | 16 | python decl body at src/typeguard/_checkers.py:834 body 840 |  |  | 0.583 |
| walker |  | 9399 | 106 | python imports in src/typeguard/_pytest_plugin.py |  |  | 0.583 |
| ns | 9496 |  | 576 | docs/extending.rst — writing a lookup + checker function (head) | 4.2 | 2.9 | 0.567 |
| walker |  | 9604 | 205 | python method sigs #2 in src/typeguard/_transformer.py |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:574 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:577 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.569 |
| walker |  | 9604 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.569 |
| walker |  | 9615 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.569 |
| walker |  | 9621 | 6 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.569 |
| walker |  | 9657 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.569 |
| walker |  | 9679 | 22 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.569 |
| walker |  | 9708 | 29 | python method body at src/typeguard/_transformer.py:466 body 467 |  |  | 0.569 |
| walker |  | 9725 | 17 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.569 |
| walker |  | 9742 | 17 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.569 |
| walker |  | 9787 | 45 | python method at src/typeguard/_transformer.py:516 |  |  | 0.569 |
| ns | 9810 |  | 314 | docs/extending.rst — MySpecialType worked example | 4.3 | 4.2 | 0.558 |
| walker |  | 9811 | 24 | python decl body at src/typeguard/_checkers.py:153 body 159 |  |  | 0.558 |
| walker |  | 9873 | 62 | python method body at src/typeguard/_config.py:52 body 53 |  |  | 0.569 |
| walker |  | 9999 | 126 | python decl body at src/typeguard/_checkers.py:305 body 311 |  |  | 0.569 |
| ns | 10117 |  | 307 | tests/test_checkers.py — TestX class locations | 4.4 |  | 0.559 |
| ns | 10296 |  | 179 | tests/test_typechecked.py — Test class + module test locations | 4.5 |  | 0.554 |
