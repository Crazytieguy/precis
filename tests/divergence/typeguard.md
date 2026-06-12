Score(3000)=0.656 I=0.879 C=0.490 ns_rows≤3K=14/39 (reached=7 partial=3 missing=4)

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
| walker |  | 1181 | 9 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.832 |
| walker |  | 1238 | 57 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.834 |
| walker |  | 1272 | 34 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.834 |
| walker |  | 1272 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.834 |
| walker |  | 1272 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.834 |
| walker |  | 1291 | 19 | python imports in src/typeguard/_exceptions.py |  |  | 0.834 |
| ns | 1351 |  | 232 | typeguard/__init__.py — module rewrite, lazy `config`, autoload | 1.10 | 1.9 | 0.759 |
| walker |  | 1391 | 100 | listing of 'tests' |  |  | 0.864 |
| walker |  | 1442 | 51 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.865 |
| walker |  | 1442 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.865 |
| walker |  | 1442 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1442 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1442 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.865 |
| walker |  | 1461 | 19 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1481 | 20 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1511 | 30 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.867 |
| walker |  | 1553 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.869 |
| walker |  | 1640 | 87 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.871 |
| walker |  | 1640 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.871 |
| walker |  | 1640 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.871 |
| walker |  | 1640 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.871 |
| walker |  | 1640 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.871 |
| walker |  | 1640 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.871 |
| walker |  | 1649 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.871 |
| walker |  | 1658 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.871 |
| ns | 1663 |  | 312 | _exceptions.py — class signatures + summary docstrings | 2.1 |  | 0.822 |
| walker |  | 1667 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.822 |
| walker |  | 1723 | 56 | python decl names surface in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1723 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1723 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1730 | 7 | python decl at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1751 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1751 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.822 |
| walker |  | 1771 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1799 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1883 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.823 |
| walker |  | 1941 | 58 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.823 |
| walker |  | 1949 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.823 |
| walker |  | 1957 | 8 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.823 |
| walker |  | 1989 | 32 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.823 |
| walker |  | 2005 | 16 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.823 |
| walker |  | 2027 | 22 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.823 |
| walker |  | 2041 | 14 | listing of 'tests/mypy' |  |  | 0.823 |
| walker |  | 2110 | 69 | python method at src/typeguard/_memo.py:37 |  |  | 0.824 |
| walker |  | 2147 | 37 | python imports in src/typeguard/_memo.py |  |  | 0.825 |
| walker |  | 2172 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.839 |
| ns | 2249 |  | 586 | check_type — primary entry-point signature | 2.2 |  | 0.728 |
| walker |  | 2275 | 103 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.729 |
| walker |  | 2360 | 85 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.730 |
| walker |  | 2360 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.730 |
| walker |  | 2360 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.730 |
| walker |  | 2368 | 8 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.730 |
| walker |  | 2400 | 32 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.730 |
| walker |  | 2420 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.730 |
| walker |  | 2443 | 23 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.730 |
| walker |  | 2538 | 95 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.730 |
| walker |  | 2559 | 21 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.731 |
| walker |  | 2661 | 102 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.732 |
| ns | 2766 |  | 517 | @typechecked — overloaded signatures | 2.3 |  | 0.685 |
| walker |  | 2785 | 124 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.685 |
| walker |  | 2833 | 48 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.685 |
| walker |  | 2881 | 48 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.685 |
| walker |  | 2929 | 48 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.685 |
| walker |  | 2977 | 48 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.685 |
| ns | 2985 |  | 219 | install_import_hook — signature + docstring | 2.4 |  | 0.656 |
| walker |  | 3025 | 48 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.656 |
| walker |  | 3073 | 48 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.656 |
| walker |  | 3121 | 48 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.656 |
| walker |  | 3169 | 48 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.656 |
| walker |  | 3217 | 48 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.656 |
| walker |  | 3265 | 48 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.656 |
| walker |  | 3283 | 18 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.656 |
| walker |  | 3331 | 48 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.656 |
| ns | 3337 |  | 352 | suppress_type_checks — overloaded signatures + docstring | 2.5 |  | 0.617 |
| walker |  | 3392 | 61 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.617 |
| walker |  | 3427 | 35 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.617 |
| walker |  | 3553 | 126 | python decl names surface in src/typeguard/_functions.py |  |  | 0.617 |
| walker |  | 3553 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.617 |
| walker |  | 3597 | 44 | python decl at src/typeguard/_functions.py:118 |  |  | 0.617 |
| walker |  | 3642 | 45 | python decl at src/typeguard/_functions.py:149 |  |  | 0.618 |
| walker |  | 3661 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.618 |
| walker |  | 3707 | 46 | python decl at src/typeguard/_functions.py:185 |  |  | 0.619 |
| walker |  | 3753 | 46 | python decl at src/typeguard/_functions.py:216 |  |  | 0.619 |
| walker |  | 3800 | 47 | python decl at src/typeguard/_functions.py:245 |  |  | 0.620 |
| ns | 3817 |  | 480 | TypeCheckConfiguration — dataclass + attribute docstrings | 2.6 |  | 0.575 |
| walker |  | 3856 | 56 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.575 |
| walker |  | 3946 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.575 |
| walker |  | 3955 | 9 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.575 |
| walker |  | 4046 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.575 |
| walker |  | 4055 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.575 |
| walker |  | 4178 | 123 | python decl at src/typeguard/_functions.py:50 |  |  | 0.582 |
| ns | 4180 |  | 363 | ForwardRefPolicy + CollectionCheckStrategy enums | 2.7 |  | 0.566 |
| walker |  | 4291 | 113 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.567 |
| walker |  | 4291 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.567 |
| walker |  | 4291 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.567 |
| walker |  | 4291 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.567 |
| walker |  | 4291 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.567 |
| walker |  | 4309 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.567 |
| walker |  | 4338 | 29 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.567 |
| walker |  | 4385 | 47 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.569 |
| ns | 4402 |  | 222 | TypeCheckMemo — class skeleton + __init__ | 2.8 |  | 0.569 |
| walker |  | 4416 | 31 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.569 |
| walker |  | 4429 | 13 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.569 |
| ns | 4573 |  | 171 | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | 2.9 |  | 0.555 |
| walker |  | 4578 | 149 | python method sigs in src/typeguard/_importhook.py |  |  | 0.556 |
| walker |  | 4578 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.556 |
| walker |  | 4578 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.556 |
| walker |  | 4578 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.556 |
| walker |  | 4578 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.556 |
| walker |  | 4578 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.556 |
| walker |  | 4583 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.556 |
| walker |  | 4593 | 10 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.556 |
| walker |  | 4602 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.556 |
| walker |  | 4629 | 27 | python method at src/typeguard/_importhook.py:99 |  |  | 0.556 |
| walker |  | 4679 | 50 | python method at src/typeguard/_importhook.py:167 |  |  | 0.556 |
| walker |  | 4688 | 9 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.556 |
| walker |  | 4742 | 54 | python method at src/typeguard/_importhook.py:124 |  |  | 0.556 |
| walker |  | 4766 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.556 |
| walker |  | 4865 | 99 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.557 |
| ns | 4926 |  | 353 | TypeguardFinder + ImportHookManager — class + key methods | 2.10 |  | 0.562 |
| walker |  | 4943 | 78 | python method at src/typeguard/_importhook.py:56 |  |  | 0.562 |
| walker |  | 5072 | 129 | python decl names surface in src/typeguard/_utils.py |  |  | 0.563 |
| walker |  | 5072 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.563 |
| walker |  | 5072 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.563 |
| walker |  | 5072 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.563 |
| walker |  | 5072 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.563 |
| walker |  | 5072 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.563 |
| walker |  | 5072 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.563 |
| walker |  | 5078 | 6 | python decl at src/typeguard/_utils.py:172 |  |  | 0.563 |
| walker |  | 5088 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.563 |
| walker |  | 5104 | 16 | python method sigs in src/typeguard/_utils.py |  |  | 0.563 |
| walker |  | 5104 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.563 |
| walker |  | 5111 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.563 |
| walker |  | 5172 | 61 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.563 |
| ns | 5177 |  | 251 | warn_on_error + load_plugins — signatures | 2.11 |  | 0.555 |
| walker |  | 5241 | 69 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.555 |
| walker |  | 5297 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.555 |
| ns | 5358 |  | 181 | check_type_internal — signature + docstring | 2.12 |  | 0.544 |
| walker |  | 5441 | 144 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.544 |
| walker |  | 5441 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.544 |
| walker |  | 5441 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.544 |
| walker |  | 5441 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.544 |
| walker |  | 5441 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.544 |
| walker |  | 5447 | 6 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.544 |
| walker |  | 5462 | 15 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.544 |
| walker |  | 5486 | 24 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.544 |
| walker |  | 5512 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.544 |
| walker |  | 5538 | 26 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.544 |
| walker |  | 5566 | 28 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.544 |
| walker |  | 5594 | 28 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.545 |
| ns | 5642 |  | 284 | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | 2.13 | 2.2 | 0.569 |
| walker |  | 5706 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.569 |
| ns | 5813 |  | 171 | Unset sentinel + small _utils helpers | 2.14 |  | 0.576 |
| ns | 6041 |  | 228 | _checkers.py — every check_* function name (locations) | 3.1 | 2.9 | 0.567 |
| walker |  | 6063 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.567 |
| ns | 6639 |  | 598 | _checkers.py — origin_type_checkers dispatch table | 3.2 | 3.1 | 0.541 |
| walker |  | 7039 | 976 | python method sigs in src/typeguard/_transformer.py |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:574 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:577 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.543 |
| walker |  | 7039 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.543 |
| walker |  | 7048 | 9 | python method at src/typeguard/_transformer.py:352 |  |  | 0.543 |
| walker |  | 7057 | 9 | python method at src/typeguard/_transformer.py:472 |  |  | 0.543 |
| walker |  | 7062 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.543 |
| walker |  | 7067 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.543 |
| walker |  | 7072 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.543 |
| walker |  | 7080 | 8 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.543 |
| ns | 7082 |  | 443 | _checkers.py — builtin_checker_lookup dispatch fallbacks | 3.3 | 3.2 | 0.524 |
| walker |  | 7091 | 11 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.524 |
| walker |  | 7100 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.524 |
| walker |  | 7127 | 27 | python method at src/typeguard/_transformer.py:913 |  |  | 0.524 |
| walker |  | 7137 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.524 |
| walker |  | 7147 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.524 |
| ns | 7148 |  | 66 | _transformer.py — class locations | 3.4 |  | 0.529 |
| walker |  | 7177 | 30 | python method at src/typeguard/_transformer.py:650 |  |  | 0.529 |
| walker |  | 7192 | 15 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.529 |
| walker |  | 7226 | 34 | python method at src/typeguard/_transformer.py:489 |  |  | 0.529 |
| walker |  | 7238 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.529 |
| walker |  | 7250 | 12 | python method body at src/typeguard/_transformer.py:913 body 916 |  |  | 0.529 |
| walker |  | 7271 | 21 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.529 |
| walker |  | 7291 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.529 |
| walker |  | 7326 | 35 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.529 |
| walker |  | 7348 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.529 |
| ns | 7354 |  | 206 | _transformer.py — TypeguardTransformer visit_* methods (locations) | 3.5 | 3.4 | 0.543 |
| walker |  | 7387 | 39 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.543 |
| walker |  | 7411 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.543 |
| walker |  | 7435 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.543 |
| walker |  | 7484 | 49 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.543 |
| walker |  | 7536 | 52 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.543 |
| walker |  | 7589 | 53 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.543 |
| walker |  | 7618 | 29 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.543 |
| ns | 7639 |  | 285 | _transformer.py — generator_names + annotated_names + ignore_decorators tables | 3.6 |  | 0.537 |
| walker |  | 7647 | 29 | python method body at src/typeguard/_transformer.py:466 body 467 |  |  | 0.537 |
| ns | 7673 |  | 34 | _decorators.py — function locations | 3.7 |  | 0.538 |
| walker |  | 7678 | 31 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.538 |
| walker |  | 7742 | 64 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.549 |
| walker |  | 7809 | 67 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.549 |
| walker |  | 7843 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.549 |
| walker |  | 7901 | 58 | python imports in src/typeguard/_config.py |  |  | 0.549 |
| walker |  | 7944 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.549 |
| ns | 8036 |  | 363 | docs/api.rst — public API by topic group (head) | 3.8 |  | 0.530 |
| walker |  | 8099 | 155 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.538 |
| walker |  | 8116 | 17 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.538 |
| walker |  | 8133 | 17 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.538 |
| walker |  | 8173 | 40 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.538 |
| walker |  | 8213 | 40 | python method body at src/typeguard/_transformer.py:206 body 207 |  |  | 0.529 |
| ns | 8213 |  | 177 | docs/userguide.rst — H2 section locations | 3.9 |  | 0.529 |
| walker |  | 8348 | 135 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.552 |
| ns | 8392 |  | 179 | docs/features.rst — H2 section locations | 3.10 |  | 0.544 |
| walker |  | 8495 | 147 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.558 |
| walker |  | 8571 | 76 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.558 |
| walker |  | 8614 | 43 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.565 |
| walker |  | 8690 | 76 | python imports in src/typeguard/_suppression.py |  |  | 0.565 |
| ns | 8692 |  | 300 | docs/api.rst — Custom checkers / Suppression / Exceptions sections | 4.1 | 3.8 | 0.552 |
| walker |  | 8734 | 44 | python method body at src/typeguard/_transformer.py:328 body 329 |  |  | 0.552 |
| walker |  | 8892 | 158 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.552 |
| walker |  | 9111 | 219 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.554 |
| walker |  | 9119 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.554 |
| walker |  | 9169 | 50 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.554 |
| walker |  | 9220 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.562 |
| ns | 9266 |  | 574 | docs/extending.rst — writing a lookup + checker function (head) | 4.2 | 2.9 | 0.546 |
| walker |  | 9420 | 200 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.566 |
| walker |  | 9519 | 99 | python decl body at src/typeguard/_utils.py:142 body 143 |  |  | 0.566 |
| ns | 9578 |  | 312 | docs/extending.rst — MySpecialType worked example | 4.3 | 4.2 | 0.555 |
| walker |  | 9669 | 150 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.570 |
| walker |  | 9669 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.570 |
| walker |  | 9669 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.570 |
| walker |  | 9717 | 48 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.570 |
| walker |  | 9765 | 48 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.570 |
| walker |  | 9779 | 14 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.570 |
| walker |  | 9827 | 48 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.560 |
| ns | 9827 |  | 249 | tests/test_checkers.py — TestX class locations | 4.4 |  | 0.560 |
| walker |  | 9841 | 14 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.560 |
| walker |  | 9889 | 48 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.560 |
| walker |  | 9937 | 48 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.560 |
| ns | 9976 |  | 149 | tests/test_typechecked.py — Test class + module test locations | 4.5 |  | 0.555 |
| walker |  | 9985 | 48 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.555 |
