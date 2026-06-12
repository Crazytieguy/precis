Score(3000)=0.662 I=0.881 C=0.498 ns_rows≤3K=14/39 (reached=7 partial=3 missing=4)

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
| walker |  | 641 | 414 | python imports in src/typeguard/__init__.py |  |  | 0.522 |
| walker |  | 669 | 28 | python decl names surface in src/typeguard/__init__.py |  |  | 0.522 |
| walker |  | 669 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.522 |
| ns | 700 |  | 150 | README — instrumentation options (@typechecked vs import hook) | 1.7 | 1.6 | 0.468 |
| walker |  | 710 | 41 | listing of 'docs' |  |  | 0.584 |
| walker |  | 721 | 11 | python decl names surface in src/typeguard/_memo.py |  |  | 0.584 |
| walker |  | 721 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.584 |
| walker |  | 732 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.584 |
| walker |  | 754 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.584 |
| ns | 964 |  | 264 | Public re-exports — head of typeguard/__init__.py | 1.8 |  | 0.640 |
| ns | 1119 |  | 155 | Public re-exports — tail of typeguard/__init__.py | 1.9 | 1.8 | 0.662 |
| walker |  | 1172 | 418 | README headline in README.rst |  |  | 0.832 |
| walker |  | 1193 | 21 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.832 |
| walker |  | 1202 | 9 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.832 |
| walker |  | 1259 | 57 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.834 |
| walker |  | 1293 | 34 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.834 |
| walker |  | 1293 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.834 |
| walker |  | 1293 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.834 |
| walker |  | 1312 | 19 | python imports in src/typeguard/_exceptions.py |  |  | 0.834 |
| ns | 1351 |  | 232 | typeguard/__init__.py — module rewrite, lazy `config`, autoload | 1.10 | 1.9 | 0.759 |
| walker |  | 1412 | 100 | listing of 'tests' |  |  | 0.864 |
| walker |  | 1463 | 51 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.865 |
| walker |  | 1463 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.865 |
| walker |  | 1463 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1463 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1463 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.865 |
| walker |  | 1482 | 19 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1502 | 20 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1532 | 30 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.867 |
| walker |  | 1574 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.869 |
| walker |  | 1661 | 87 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.871 |
| walker |  | 1661 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.871 |
| walker |  | 1661 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.871 |
| walker |  | 1661 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.871 |
| walker |  | 1661 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.871 |
| walker |  | 1661 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.871 |
| ns | 1663 |  | 312 | _exceptions.py — class signatures + summary docstrings | 2.1 |  | 0.822 |
| walker |  | 1670 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.822 |
| walker |  | 1679 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.822 |
| walker |  | 1688 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.822 |
| walker |  | 1744 | 56 | python decl names surface in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1744 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1744 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1751 | 7 | python decl at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1772 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1772 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.822 |
| walker |  | 1792 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1820 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1842 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1926 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.823 |
| walker |  | 1984 | 58 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.823 |
| walker |  | 1992 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.823 |
| walker |  | 2000 | 8 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.823 |
| walker |  | 2032 | 32 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.823 |
| walker |  | 2048 | 16 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.824 |
| walker |  | 2070 | 22 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.824 |
| walker |  | 2084 | 14 | listing of 'tests/mypy' |  |  | 0.824 |
| walker |  | 2153 | 69 | python method at src/typeguard/_memo.py:37 |  |  | 0.825 |
| walker |  | 2190 | 37 | python imports in src/typeguard/_memo.py |  |  | 0.826 |
| walker |  | 2215 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.839 |
| ns | 2249 |  | 586 | check_type — primary entry-point signature | 2.2 |  | 0.728 |
| walker |  | 2318 | 103 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.730 |
| walker |  | 2403 | 85 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.730 |
| walker |  | 2403 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.730 |
| walker |  | 2403 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.730 |
| walker |  | 2411 | 8 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.730 |
| walker |  | 2443 | 32 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.730 |
| walker |  | 2463 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.730 |
| walker |  | 2486 | 23 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.730 |
| walker |  | 2581 | 95 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.731 |
| walker |  | 2602 | 21 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.731 |
| walker |  | 2704 | 102 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.733 |
| walker |  | 2726 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.733 |
| ns | 2766 |  | 517 | @typechecked — overloaded signatures | 2.3 |  | 0.691 |
| walker |  | 2850 | 124 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.692 |
| walker |  | 2898 | 48 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.692 |
| walker |  | 2946 | 48 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.692 |
| ns | 2985 |  | 219 | install_import_hook — signature + docstring | 2.4 |  | 0.662 |
| walker |  | 2994 | 48 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.662 |
| walker |  | 3042 | 48 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.662 |
| walker |  | 3090 | 48 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.662 |
| walker |  | 3138 | 48 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.662 |
| walker |  | 3186 | 48 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.662 |
| walker |  | 3234 | 48 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.662 |
| walker |  | 3282 | 48 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.662 |
| walker |  | 3330 | 48 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.662 |
| ns | 3337 |  | 352 | suppress_type_checks — overloaded signatures + docstring | 2.5 |  | 0.623 |
| walker |  | 3348 | 18 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.623 |
| walker |  | 3396 | 48 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.623 |
| walker |  | 3457 | 61 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.623 |
| walker |  | 3492 | 35 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.623 |
| walker |  | 3618 | 126 | python decl names surface in src/typeguard/_functions.py |  |  | 0.623 |
| walker |  | 3618 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.623 |
| walker |  | 3662 | 44 | python decl at src/typeguard/_functions.py:118 |  |  | 0.623 |
| walker |  | 3707 | 45 | python decl at src/typeguard/_functions.py:149 |  |  | 0.624 |
| walker |  | 3726 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.624 |
| walker |  | 3772 | 46 | python decl at src/typeguard/_functions.py:185 |  |  | 0.624 |
| ns | 3817 |  | 480 | TypeCheckConfiguration — dataclass + attribute docstrings | 2.6 |  | 0.581 |
| walker |  | 3818 | 46 | python decl at src/typeguard/_functions.py:216 |  |  | 0.582 |
| walker |  | 3865 | 47 | python decl at src/typeguard/_functions.py:245 |  |  | 0.583 |
| walker |  | 3921 | 56 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.583 |
| walker |  | 4011 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.583 |
| walker |  | 4020 | 9 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.583 |
| walker |  | 4111 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.583 |
| walker |  | 4120 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.583 |
| ns | 4180 |  | 363 | ForwardRefPolicy + CollectionCheckStrategy enums | 2.7 |  | 0.567 |
| walker |  | 4243 | 123 | python decl at src/typeguard/_functions.py:50 |  |  | 0.573 |
| walker |  | 4265 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.575 |
| walker |  | 4378 | 113 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.576 |
| walker |  | 4378 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.576 |
| walker |  | 4378 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.576 |
| walker |  | 4378 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.576 |
| walker |  | 4378 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.576 |
| walker |  | 4396 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.576 |
| ns | 4402 |  | 222 | TypeCheckMemo — class skeleton + __init__ | 2.8 |  | 0.576 |
| walker |  | 4425 | 29 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.576 |
| walker |  | 4472 | 47 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.579 |
| walker |  | 4503 | 31 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.579 |
| walker |  | 4516 | 13 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.579 |
| ns | 4573 |  | 171 | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | 2.9 |  | 0.565 |
| walker |  | 4665 | 149 | python method sigs in src/typeguard/_importhook.py |  |  | 0.565 |
| walker |  | 4665 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.565 |
| walker |  | 4665 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.565 |
| walker |  | 4665 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.565 |
| walker |  | 4665 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.565 |
| walker |  | 4665 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.565 |
| walker |  | 4670 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.565 |
| walker |  | 4680 | 10 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.565 |
| walker |  | 4689 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.565 |
| walker |  | 4716 | 27 | python method at src/typeguard/_importhook.py:99 |  |  | 0.565 |
| walker |  | 4766 | 50 | python method at src/typeguard/_importhook.py:167 |  |  | 0.565 |
| walker |  | 4775 | 9 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.565 |
| walker |  | 4829 | 54 | python method at src/typeguard/_importhook.py:124 |  |  | 0.566 |
| walker |  | 4853 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.566 |
| ns | 4926 |  | 353 | TypeguardFinder + ImportHookManager — class + key methods | 2.10 |  | 0.554 |
| walker |  | 4952 | 99 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.571 |
| walker |  | 5030 | 78 | python method at src/typeguard/_importhook.py:56 |  |  | 0.571 |
| walker |  | 5159 | 129 | python decl names surface in src/typeguard/_utils.py |  |  | 0.572 |
| walker |  | 5159 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.572 |
| walker |  | 5159 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.572 |
| walker |  | 5159 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.572 |
| walker |  | 5159 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.572 |
| walker |  | 5159 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.572 |
| walker |  | 5159 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.572 |
| walker |  | 5165 | 6 | python decl at src/typeguard/_utils.py:172 |  |  | 0.572 |
| walker |  | 5175 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.572 |
| ns | 5177 |  | 251 | warn_on_error + load_plugins — signatures | 2.11 |  | 0.563 |
| walker |  | 5191 | 16 | python method sigs in src/typeguard/_utils.py |  |  | 0.563 |
| walker |  | 5191 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.563 |
| walker |  | 5198 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.563 |
| walker |  | 5259 | 61 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.563 |
| walker |  | 5328 | 69 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.563 |
| ns | 5358 |  | 181 | check_type_internal — signature + docstring | 2.12 |  | 0.552 |
| walker |  | 5384 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.552 |
| walker |  | 5528 | 144 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.553 |
| walker |  | 5528 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.553 |
| walker |  | 5528 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.553 |
| walker |  | 5528 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.553 |
| walker |  | 5528 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.553 |
| walker |  | 5534 | 6 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.553 |
| walker |  | 5549 | 15 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.553 |
| walker |  | 5573 | 24 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.553 |
| walker |  | 5599 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.553 |
| walker |  | 5625 | 26 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.553 |
| ns | 5642 |  | 284 | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | 2.13 | 2.2 | 0.576 |
| walker |  | 5653 | 28 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.576 |
| walker |  | 5681 | 28 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.577 |
| walker |  | 5793 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.577 |
| ns | 5813 |  | 171 | Unset sentinel + small _utils helpers | 2.14 |  | 0.583 |
| ns | 6041 |  | 228 | _checkers.py — every check_* function name (locations) | 3.1 | 2.9 | 0.574 |
| walker |  | 6150 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.574 |
| walker |  | 6401 | 251 | python method sigs in src/typeguard/_transformer.py |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.574 |
| walker |  | 6401 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.574 |
| walker |  | 6412 | 11 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.574 |
| walker |  | 6424 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.574 |
| walker |  | 6448 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.574 |
| walker |  | 6472 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.574 |
| walker |  | 6503 | 31 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.574 |
| walker |  | 6567 | 64 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.588 |
| walker |  | 6625 | 58 | python imports in src/typeguard/_config.py |  |  | 0.588 |
| ns | 6639 |  | 598 | _checkers.py — origin_type_checkers dispatch table | 3.2 | 3.1 | 0.561 |
| walker |  | 6780 | 155 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.571 |
| walker |  | 6820 | 40 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.571 |
| walker |  | 6860 | 40 | python method body at src/typeguard/_transformer.py:206 body 207 |  |  | 0.571 |
| walker |  | 6995 | 135 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.573 |
| ns | 7082 |  | 443 | _checkers.py — builtin_checker_lookup dispatch fallbacks | 3.3 | 3.2 | 0.554 |
| walker |  | 7142 | 147 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.571 |
| ns | 7148 |  | 66 | _transformer.py — class locations | 3.4 |  | 0.575 |
| walker |  | 7218 | 76 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.575 |
| walker |  | 7261 | 43 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.584 |
| walker |  | 7337 | 76 | python imports in src/typeguard/_suppression.py |  |  | 0.584 |
| ns | 7354 |  | 206 | _transformer.py — TypeguardTransformer visit_* methods (locations) | 3.5 | 3.4 | 0.573 |
| walker |  | 7592 | 255 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.573 |
| walker |  | 7592 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.573 |
| walker |  | 7601 | 9 | python method at src/typeguard/_transformer.py:352 |  |  | 0.573 |
| walker |  | 7606 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.573 |
| walker |  | 7611 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.573 |
| walker |  | 7616 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.573 |
| walker |  | 7625 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.573 |
| walker |  | 7635 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.573 |
| ns | 7639 |  | 285 | _transformer.py — generator_names + annotated_names + ignore_decorators tables | 3.6 |  | 0.590 |
| walker |  | 7645 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.590 |
| walker |  | 7667 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.590 |
| ns | 7673 |  | 34 | _decorators.py — function locations | 3.7 |  | 0.590 |
| walker |  | 7696 | 29 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.590 |
| walker |  | 7730 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.590 |
| walker |  | 7774 | 44 | python method body at src/typeguard/_transformer.py:328 body 329 |  |  | 0.590 |
| walker |  | 7932 | 158 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.590 |
| walker |  | 7940 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.590 |
| walker |  | 7990 | 50 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.590 |
| ns | 8036 |  | 363 | docs/api.rst — public API by topic group (head) | 3.8 |  | 0.569 |
| walker |  | 8041 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.579 |
| ns | 8213 |  | 177 | docs/userguide.rst — H2 section locations | 3.9 |  | 0.569 |
| walker |  | 8241 | 200 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.590 |
| walker |  | 8340 | 99 | python decl body at src/typeguard/_utils.py:142 body 143 |  |  | 0.590 |
| ns | 8392 |  | 179 | docs/features.rst — H2 section locations | 3.10 |  | 0.581 |
| walker |  | 8490 | 150 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.597 |
| walker |  | 8490 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.597 |
| walker |  | 8490 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.597 |
| walker |  | 8538 | 48 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.597 |
| walker |  | 8586 | 48 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.597 |
| walker |  | 8600 | 14 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.597 |
| walker |  | 8648 | 48 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.597 |
| walker |  | 8662 | 14 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.597 |
| ns | 8692 |  | 300 | docs/api.rst — Custom checkers / Suppression / Exceptions sections | 4.1 | 3.8 | 0.583 |
| walker |  | 8710 | 48 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.583 |
| walker |  | 8758 | 48 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.583 |
| walker |  | 8806 | 48 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.583 |
| walker |  | 8854 | 48 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.583 |
| walker |  | 8902 | 48 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.583 |
| walker |  | 8950 | 48 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.583 |
| walker |  | 8975 | 25 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.583 |
| walker |  | 9042 | 67 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.583 |
| walker |  | 9076 | 34 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.583 |
| walker |  | 9093 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.583 |
| walker |  | 9109 | 16 | python decl body at src/typeguard/_checkers.py:834 body 840 |  |  | 0.583 |
| walker |  | 9185 | 76 | python decl body at src/typeguard/_checkers.py:651 body 657 |  |  | 0.583 |
| ns | 9266 |  | 574 | docs/extending.rst — writing a lookup + checker function (head) | 4.2 | 2.9 | 0.567 |
| walker |  | 9291 | 106 | python imports in src/typeguard/_pytest_plugin.py |  |  | 0.567 |
| walker |  | 9498 | 207 | python method sigs #2 in src/typeguard/_transformer.py |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:574 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:577 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.569 |
| walker |  | 9498 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.569 |
| walker |  | 9507 | 9 | python method at src/typeguard/_transformer.py:472 |  |  | 0.569 |
| walker |  | 9515 | 8 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.569 |
| walker |  | 9549 | 34 | python method at src/typeguard/_transformer.py:489 |  |  | 0.569 |
| walker |  | 9569 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.569 |
| ns | 9578 |  | 312 | docs/extending.rst — MySpecialType worked example | 4.3 | 4.2 | 0.558 |
| walker |  | 9598 | 29 | python method body at src/typeguard/_transformer.py:466 body 467 |  |  | 0.558 |
| walker |  | 9641 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.558 |
| walker |  | 9658 | 17 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.558 |
| walker |  | 9675 | 17 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.558 |
| walker |  | 9699 | 24 | python decl body at src/typeguard/_checkers.py:153 body 159 |  |  | 0.558 |
| walker |  | 9761 | 62 | python method body at src/typeguard/_config.py:52 body 53 |  |  | 0.569 |
| ns | 9827 |  | 249 | tests/test_checkers.py — TestX class locations | 4.4 |  | 0.559 |
| walker |  | 9889 | 128 | python decl body at src/typeguard/_checkers.py:305 body 311 |  |  | 0.559 |
| ns | 9976 |  | 149 | tests/test_typechecked.py — Test class + module test locations | 4.5 |  | 0.554 |
