Score(3000)=0.694 I=0.898 C=0.537 ns_rows≤3K=14/39 (reached=9 partial=1 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| ns | 75 |  | 41 | docs/ listing | 1.2 |  | 0.682 |
| walker |  | 89 | 51 | [dependencies] in pyproject.toml |  |  | 0.682 |
| ns | 147 |  | 72 | src/typeguard/ module listing | 1.3 |  | 0.484 |
| walker |  | 155 | 66 | [package] in pyproject.toml |  |  | 0.484 |
| walker |  | 227 | 72 | listing of 'src/typeguard' |  |  | 0.799 |
| ns | 247 |  | 100 | tests/ listing | 1.4 |  | 0.618 |
| ns | 349 |  | 102 | README — one-paragraph lede | 1.5 |  | 0.581 |
| ns | 550 |  | 201 | README — check_type vs. code instrumentation modes | 1.6 | 1.5 | 0.503 |
| walker |  | 646 | 419 | python imports in src/typeguard/__init__.py |  |  | 0.523 |
| walker |  | 684 | 38 | python decl names surface in src/typeguard/__init__.py |  |  | 0.523 |
| walker |  | 684 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.523 |
| ns | 700 |  | 150 | README — instrumentation options (@typechecked vs import hook) | 1.7 | 1.6 | 0.469 |
| walker |  | 725 | 41 | listing of 'docs' |  |  | 0.585 |
| walker |  | 736 | 11 | python decl names surface in src/typeguard/_memo.py |  |  | 0.585 |
| walker |  | 736 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.585 |
| walker |  | 747 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.585 |
| walker |  | 769 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.585 |
| ns | 964 |  | 264 | Public re-exports — head of typeguard/__init__.py | 1.8 |  | 0.648 |
| ns | 1119 |  | 155 | Public re-exports — tail of typeguard/__init__.py | 1.9 | 1.8 | 0.669 |
| walker |  | 1232 | 463 | README headline in README.rst |  |  | 0.884 |
| walker |  | 1253 | 21 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.884 |
| walker |  | 1262 | 9 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.884 |
| walker |  | 1329 | 67 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.887 |
| ns | 1351 |  | 232 | typeguard/__init__.py — module rewrite, lazy `config`, autoload | 1.10 | 1.9 | 0.814 |
| walker |  | 1363 | 34 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.814 |
| walker |  | 1363 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.814 |
| walker |  | 1363 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.814 |
| walker |  | 1382 | 19 | python imports in src/typeguard/_exceptions.py |  |  | 0.814 |
| walker |  | 1482 | 100 | listing of 'tests' |  |  | 0.917 |
| walker |  | 1533 | 51 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.918 |
| walker |  | 1533 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.918 |
| walker |  | 1533 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.918 |
| walker |  | 1533 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.918 |
| walker |  | 1533 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.918 |
| walker |  | 1552 | 19 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.918 |
| walker |  | 1572 | 20 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.919 |
| walker |  | 1602 | 30 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.920 |
| walker |  | 1644 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.923 |
| ns | 1663 |  | 312 | _exceptions.py — class signatures + summary docstrings | 2.1 |  | 0.850 |
| walker |  | 1731 | 87 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.868 |
| walker |  | 1731 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.868 |
| walker |  | 1731 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.868 |
| walker |  | 1731 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.868 |
| walker |  | 1731 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.868 |
| walker |  | 1731 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.868 |
| walker |  | 1740 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.868 |
| walker |  | 1749 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.868 |
| walker |  | 1758 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.868 |
| walker |  | 1821 | 63 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.868 |
| walker |  | 1829 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.868 |
| walker |  | 1837 | 8 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.868 |
| walker |  | 1869 | 32 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.868 |
| walker |  | 1885 | 16 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.868 |
| walker |  | 1907 | 22 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.869 |
| walker |  | 1973 | 66 | python decl names surface in src/typeguard/_config.py |  |  | 0.869 |
| walker |  | 1973 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.869 |
| walker |  | 1973 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.869 |
| walker |  | 1980 | 7 | python decl at src/typeguard/_config.py:62 |  |  | 0.869 |
| walker |  | 2001 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.869 |
| walker |  | 2001 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.869 |
| walker |  | 2021 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.869 |
| walker |  | 2049 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.869 |
| walker |  | 2071 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.869 |
| walker |  | 2155 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.870 |
| walker |  | 2169 | 14 | listing of 'tests/mypy' |  |  | 0.870 |
| walker |  | 2238 | 69 | python method at src/typeguard/_memo.py:37 |  |  | 0.871 |
| ns | 2249 |  | 586 | check_type — primary entry-point signature | 2.2 |  | 0.756 |
| walker |  | 2263 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.767 |
| walker |  | 2348 | 85 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.768 |
| walker |  | 2348 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.768 |
| walker |  | 2348 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.768 |
| walker |  | 2356 | 8 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.768 |
| walker |  | 2388 | 32 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.768 |
| walker |  | 2408 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.768 |
| walker |  | 2431 | 23 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.768 |
| walker |  | 2526 | 95 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.768 |
| walker |  | 2547 | 21 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.769 |
| walker |  | 2649 | 102 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.771 |
| walker |  | 2671 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.771 |
| ns | 2766 |  | 517 | @typechecked — overloaded signatures | 2.3 |  | 0.724 |
| walker |  | 2795 | 124 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.725 |
| walker |  | 2843 | 48 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.725 |
| walker |  | 2891 | 48 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.725 |
| walker |  | 2939 | 48 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.725 |
| ns | 2985 |  | 219 | install_import_hook — signature + docstring | 2.4 |  | 0.694 |
| walker |  | 2987 | 48 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.694 |
| walker |  | 3035 | 48 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.694 |
| walker |  | 3083 | 48 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.694 |
| walker |  | 3131 | 48 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.694 |
| walker |  | 3179 | 48 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.694 |
| walker |  | 3227 | 48 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.694 |
| walker |  | 3275 | 48 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.694 |
| walker |  | 3293 | 18 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.694 |
| ns | 3337 |  | 352 | suppress_type_checks — overloaded signatures + docstring | 2.5 |  | 0.652 |
| walker |  | 3341 | 48 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.652 |
| walker |  | 3402 | 61 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.652 |
| walker |  | 3437 | 35 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.652 |
| walker |  | 3563 | 126 | python decl names surface in src/typeguard/_functions.py |  |  | 0.652 |
| walker |  | 3563 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.652 |
| walker |  | 3607 | 44 | python decl at src/typeguard/_functions.py:118 |  |  | 0.653 |
| walker |  | 3652 | 45 | python decl at src/typeguard/_functions.py:149 |  |  | 0.653 |
| walker |  | 3671 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.653 |
| walker |  | 3717 | 46 | python decl at src/typeguard/_functions.py:185 |  |  | 0.654 |
| walker |  | 3763 | 46 | python decl at src/typeguard/_functions.py:216 |  |  | 0.655 |
| walker |  | 3810 | 47 | python decl at src/typeguard/_functions.py:245 |  |  | 0.656 |
| ns | 3817 |  | 480 | TypeCheckConfiguration — dataclass + attribute docstrings | 2.6 |  | 0.610 |
| walker |  | 3900 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.610 |
| walker |  | 3909 | 9 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.610 |
| walker |  | 4000 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.610 |
| walker |  | 4009 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.610 |
| walker |  | 4075 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.610 |
| ns | 4180 |  | 363 | ForwardRefPolicy + CollectionCheckStrategy enums | 2.7 |  | 0.579 |
| walker |  | 4198 | 123 | python decl at src/typeguard/_functions.py:50 |  |  | 0.585 |
| walker |  | 4220 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.587 |
| walker |  | 4333 | 113 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.587 |
| walker |  | 4333 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.587 |
| walker |  | 4333 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.587 |
| walker |  | 4333 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.587 |
| walker |  | 4333 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.587 |
| walker |  | 4351 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.587 |
| walker |  | 4380 | 29 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.587 |
| ns | 4402 |  | 222 | TypeCheckMemo — class skeleton + __init__ | 2.8 |  | 0.578 |
| walker |  | 4427 | 47 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.581 |
| walker |  | 4458 | 31 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.581 |
| walker |  | 4471 | 13 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.581 |
| ns | 4573 |  | 171 | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | 2.9 |  | 0.567 |
| walker |  | 4620 | 149 | python method sigs in src/typeguard/_importhook.py |  |  | 0.567 |
| walker |  | 4620 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.567 |
| walker |  | 4620 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.567 |
| walker |  | 4620 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.567 |
| walker |  | 4620 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.567 |
| walker |  | 4620 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.567 |
| walker |  | 4625 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.567 |
| walker |  | 4635 | 10 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.567 |
| walker |  | 4644 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.567 |
| walker |  | 4671 | 27 | python method at src/typeguard/_importhook.py:99 |  |  | 0.567 |
| walker |  | 4721 | 50 | python method at src/typeguard/_importhook.py:167 |  |  | 0.567 |
| walker |  | 4730 | 9 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.567 |
| walker |  | 4784 | 54 | python method at src/typeguard/_importhook.py:124 |  |  | 0.568 |
| walker |  | 4808 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.568 |
| walker |  | 4886 | 78 | python method at src/typeguard/_importhook.py:56 |  |  | 0.568 |
| ns | 4926 |  | 353 | TypeguardFinder + ImportHookManager — class + key methods | 2.10 |  | 0.556 |
| walker |  | 4995 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.576 |
| walker |  | 5108 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.591 |
| walker |  | 5155 | 47 | python imports in src/typeguard/_memo.py |  |  | 0.602 |
| ns | 5177 |  | 251 | warn_on_error + load_plugins — signatures | 2.11 |  | 0.594 |
| walker |  | 5284 | 129 | python decl names surface in src/typeguard/_utils.py |  |  | 0.594 |
| walker |  | 5284 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.594 |
| walker |  | 5284 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.594 |
| walker |  | 5284 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.594 |
| walker |  | 5284 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.594 |
| walker |  | 5284 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.594 |
| walker |  | 5284 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.594 |
| walker |  | 5290 | 6 | python decl at src/typeguard/_utils.py:172 |  |  | 0.594 |
| walker |  | 5300 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.595 |
| walker |  | 5316 | 16 | python method sigs in src/typeguard/_utils.py |  |  | 0.595 |
| walker |  | 5316 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.595 |
| walker |  | 5323 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.595 |
| ns | 5358 |  | 181 | check_type_internal — signature + docstring | 2.12 |  | 0.583 |
| walker |  | 5394 | 71 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.583 |
| walker |  | 5473 | 79 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.583 |
| walker |  | 5529 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.583 |
| ns | 5642 |  | 284 | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | 2.13 | 2.2 | 0.604 |
| walker |  | 5678 | 149 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.605 |
| walker |  | 5678 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.605 |
| walker |  | 5678 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.605 |
| walker |  | 5678 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.605 |
| walker |  | 5678 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.605 |
| walker |  | 5684 | 6 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.605 |
| walker |  | 5699 | 15 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.605 |
| walker |  | 5723 | 24 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.605 |
| walker |  | 5749 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.605 |
| walker |  | 5775 | 26 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.605 |
| walker |  | 5803 | 28 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.605 |
| ns | 5813 |  | 171 | Unset sentinel + small _utils helpers | 2.14 |  | 0.611 |
| walker |  | 5831 | 28 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.611 |
| walker |  | 5943 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.611 |
| ns | 6041 |  | 228 | _checkers.py — every check_* function name (locations) | 3.1 | 2.9 | 0.601 |
| walker |  | 6300 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.601 |
| walker |  | 6551 | 251 | python method sigs in src/typeguard/_transformer.py |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.601 |
| walker |  | 6551 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.601 |
| walker |  | 6562 | 11 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.601 |
| walker |  | 6574 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.601 |
| walker |  | 6598 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.601 |
| walker |  | 6622 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.601 |
| ns | 6639 |  | 598 | _checkers.py — origin_type_checkers dispatch table | 3.2 | 3.1 | 0.574 |
| walker |  | 6653 | 31 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.574 |
| walker |  | 6727 | 74 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.591 |
| walker |  | 6790 | 63 | python imports in src/typeguard/_config.py |  |  | 0.591 |
| walker |  | 6830 | 40 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.591 |
| walker |  | 7005 | 175 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.602 |
| ns | 7082 |  | 443 | _checkers.py — builtin_checker_lookup dispatch fallbacks | 3.3 | 3.2 | 0.582 |
| walker |  | 7140 | 135 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.583 |
| ns | 7148 |  | 66 | _transformer.py — class locations | 3.4 |  | 0.587 |
| walker |  | 7183 | 43 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.596 |
| walker |  | 7264 | 81 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.596 |
| ns | 7354 |  | 206 | _transformer.py — TypeguardTransformer visit_* methods (locations) | 3.5 | 3.4 | 0.586 |
| walker |  | 7431 | 167 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.607 |
| walker |  | 7476 | 45 | python method body at src/typeguard/_transformer.py:206 body 207 |  |  | 0.607 |
| walker |  | 7557 | 81 | python imports in src/typeguard/_suppression.py |  |  | 0.607 |
| ns | 7639 |  | 285 | _transformer.py — generator_names + annotated_names + ignore_decorators tables | 3.6 |  | 0.622 |
| ns | 7673 |  | 34 | _decorators.py — function locations | 3.7 |  | 0.622 |
| walker |  | 7812 | 255 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.622 |
| walker |  | 7812 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.622 |
| walker |  | 7821 | 9 | python method at src/typeguard/_transformer.py:352 |  |  | 0.622 |
| walker |  | 7826 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.622 |
| walker |  | 7831 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.622 |
| walker |  | 7836 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.622 |
| walker |  | 7845 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.622 |
| walker |  | 7855 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.622 |
| walker |  | 7865 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.622 |
| walker |  | 7887 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.622 |
| walker |  | 7921 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.622 |
| walker |  | 7955 | 34 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.622 |
| walker |  | 7999 | 44 | python method body at src/typeguard/_transformer.py:328 body 329 |  |  | 0.622 |
| ns | 8036 |  | 363 | docs/api.rst — public API by topic group (head) | 3.8 |  | 0.600 |
| walker |  | 8157 | 158 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.600 |
| walker |  | 8165 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.600 |
| ns | 8213 |  | 177 | docs/userguide.rst — H2 section locations | 3.9 |  | 0.590 |
| walker |  | 8215 | 50 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.590 |
| walker |  | 8266 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.599 |
| ns | 8392 |  | 179 | docs/features.rst — H2 section locations | 3.10 |  | 0.590 |
| walker |  | 8416 | 150 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.606 |
| walker |  | 8416 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.606 |
| walker |  | 8416 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.606 |
| walker |  | 8464 | 48 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.606 |
| walker |  | 8512 | 48 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.606 |
| walker |  | 8526 | 14 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.606 |
| walker |  | 8574 | 48 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.606 |
| walker |  | 8588 | 14 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.606 |
| walker |  | 8636 | 48 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.606 |
| walker |  | 8684 | 48 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.606 |
| ns | 8692 |  | 300 | docs/api.rst — Custom checkers / Suppression / Exceptions sections | 4.1 | 3.8 | 0.592 |
| walker |  | 8732 | 48 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.592 |
| walker |  | 8780 | 48 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.592 |
| walker |  | 8828 | 48 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.592 |
| walker |  | 8876 | 48 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.592 |
| walker |  | 8901 | 25 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.592 |
| walker |  | 8968 | 67 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.592 |
| walker |  | 9002 | 34 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.592 |
| walker |  | 9019 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.592 |
| walker |  | 9035 | 16 | python decl body at src/typeguard/_checkers.py:834 body 840 |  |  | 0.592 |
| walker |  | 9111 | 76 | python decl body at src/typeguard/_checkers.py:651 body 657 |  |  | 0.592 |
| ns | 9266 |  | 574 | docs/extending.rst — writing a lookup + checker function (head) | 4.2 | 2.9 | 0.576 |
| walker |  | 9346 | 235 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.602 |
| walker |  | 9455 | 109 | python decl body at src/typeguard/_utils.py:142 body 143 |  |  | 0.602 |
| ns | 9578 |  | 312 | docs/extending.rst — MySpecialType worked example | 4.3 | 4.2 | 0.591 |
| walker |  | 9662 | 207 | python method sigs #2 in src/typeguard/_transformer.py |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:574 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:577 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.593 |
| walker |  | 9662 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.593 |
| walker |  | 9671 | 9 | python method at src/typeguard/_transformer.py:472 |  |  | 0.593 |
| walker |  | 9679 | 8 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.593 |
| walker |  | 9713 | 34 | python method at src/typeguard/_transformer.py:489 |  |  | 0.593 |
| walker |  | 9733 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.593 |
| walker |  | 9767 | 34 | python method body at src/typeguard/_transformer.py:466 body 467 |  |  | 0.593 |
| walker |  | 9810 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.593 |
| walker |  | 9827 | 17 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.582 |
| ns | 9827 |  | 249 | tests/test_checkers.py — TestX class locations | 4.4 |  | 0.582 |
| walker |  | 9844 | 17 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.582 |
| walker |  | 9868 | 24 | python decl body at src/typeguard/_checkers.py:153 body 159 |  |  | 0.582 |
| ns | 9976 |  | 149 | tests/test_typechecked.py — Test class + module test locations | 4.5 |  | 0.577 |
| walker |  | 9984 | 116 | python imports in src/typeguard/_pytest_plugin.py |  |  | 0.577 |
