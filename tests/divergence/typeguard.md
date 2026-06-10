Score(3000)=0.631 I=0.872 C=0.456 ns_rows≤3K=14/39 (reached=7 partial=3 missing=4)

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
| walker |  | 1229 | 57 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.834 |
| walker |  | 1242 | 13 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.834 |
| walker |  | 1242 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.834 |
| walker |  | 1276 | 34 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.834 |
| walker |  | 1276 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.834 |
| walker |  | 1276 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.834 |
| walker |  | 1295 | 19 | python imports in src/typeguard/_exceptions.py |  |  | 0.834 |
| ns | 1351 |  | 232 | typeguard/__init__.py — module rewrite, lazy `config`, autoload | 1.10 | 1.9 | 0.759 |
| walker |  | 1395 | 100 | listing of 'tests' |  |  | 0.864 |
| walker |  | 1446 | 51 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.865 |
| walker |  | 1446 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.865 |
| walker |  | 1446 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1446 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1446 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.865 |
| walker |  | 1465 | 19 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1485 | 20 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1515 | 30 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.867 |
| walker |  | 1557 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.869 |
| walker |  | 1644 | 87 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.871 |
| walker |  | 1644 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.871 |
| walker |  | 1644 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.871 |
| walker |  | 1644 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.871 |
| walker |  | 1644 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.871 |
| walker |  | 1644 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.871 |
| walker |  | 1653 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.871 |
| walker |  | 1662 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.871 |
| ns | 1663 |  | 312 | _exceptions.py — class signatures + summary docstrings | 2.1 |  | 0.822 |
| walker |  | 1671 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.822 |
| walker |  | 1727 | 56 | python decl names surface in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1727 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1727 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1734 | 7 | python decl at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1755 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1755 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.822 |
| walker |  | 1775 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1803 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1887 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.823 |
| ns | 2249 |  | 586 | check_type — primary entry-point signature | 2.2 |  | 0.714 |
| walker |  | 2278 | 391 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:574 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:577 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.715 |
| walker |  | 2278 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.715 |
| walker |  | 2305 | 27 | python method at src/typeguard/_transformer.py:913 |  |  | 0.715 |
| walker |  | 2335 | 30 | python method at src/typeguard/_transformer.py:650 |  |  | 0.715 |
| walker |  | 2350 | 15 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.715 |
| walker |  | 2384 | 34 | python method at src/typeguard/_transformer.py:489 |  |  | 0.715 |
| walker |  | 2396 | 12 | python method body at src/typeguard/_transformer.py:913 body 916 |  |  | 0.715 |
| walker |  | 2417 | 21 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.715 |
| walker |  | 2437 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.715 |
| walker |  | 2472 | 35 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.715 |
| walker |  | 2511 | 39 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.715 |
| walker |  | 2589 | 78 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.715 |
| walker |  | 2589 | 0 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.715 |
| walker |  | 2589 | 0 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.715 |
| walker |  | 2621 | 32 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.716 |
| walker |  | 2635 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.716 |
| walker |  | 2655 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.716 |
| walker |  | 2669 | 14 | listing of 'tests/mypy' |  |  | 0.716 |
| walker |  | 2738 | 69 | python method at src/typeguard/_memo.py:37 |  |  | 0.717 |
| ns | 2766 |  | 517 | @typechecked — overloaded signatures | 2.3 |  | 0.646 |
| walker |  | 2775 | 37 | python imports in src/typeguard/_memo.py |  |  | 0.647 |
| walker |  | 2800 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.657 |
| walker |  | 2849 | 49 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.657 |
| walker |  | 2952 | 103 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.659 |
| ns | 2985 |  | 219 | install_import_hook — signature + docstring | 2.4 |  | 0.631 |
| walker |  | 3098 | 146 | python decl names surface in src/typeguard/_functions.py |  |  | 0.631 |
| walker |  | 3098 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.631 |
| walker |  | 3142 | 44 | python decl at src/typeguard/_functions.py:118 |  |  | 0.631 |
| walker |  | 3187 | 45 | python decl at src/typeguard/_functions.py:149 |  |  | 0.632 |
| walker |  | 3206 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.632 |
| walker |  | 3252 | 46 | python decl at src/typeguard/_functions.py:185 |  |  | 0.632 |
| walker |  | 3298 | 46 | python decl at src/typeguard/_functions.py:216 |  |  | 0.633 |
| ns | 3337 |  | 352 | suppress_type_checks — overloaded signatures + docstring | 2.5 |  | 0.596 |
| walker |  | 3345 | 47 | python decl at src/typeguard/_functions.py:245 |  |  | 0.597 |
| walker |  | 3425 | 80 | python decl at src/typeguard/_functions.py:39 |  |  | 0.597 |
| walker |  | 3434 | 9 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.597 |
| walker |  | 3515 | 81 | python decl at src/typeguard/_functions.py:28 |  |  | 0.597 |
| walker |  | 3524 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.597 |
| walker |  | 3580 | 56 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.597 |
| walker |  | 3703 | 123 | python decl at src/typeguard/_functions.py:50 |  |  | 0.604 |
| walker |  | 3755 | 52 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.604 |
| walker |  | 3808 | 53 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.604 |
| ns | 3817 |  | 480 | TypeCheckConfiguration — dataclass + attribute docstrings | 2.6 |  | 0.560 |
| walker |  | 3921 | 113 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.561 |
| walker |  | 3921 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.561 |
| walker |  | 3921 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.561 |
| walker |  | 3921 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.561 |
| walker |  | 3921 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.561 |
| walker |  | 3939 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.561 |
| walker |  | 3968 | 29 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.561 |
| walker |  | 4015 | 47 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.564 |
| walker |  | 4046 | 31 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.564 |
| walker |  | 4059 | 13 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.564 |
| ns | 4180 |  | 363 | ForwardRefPolicy + CollectionCheckStrategy enums | 2.7 |  | 0.550 |
| walker |  | 4208 | 149 | python method sigs in src/typeguard/_importhook.py |  |  | 0.550 |
| walker |  | 4208 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.550 |
| walker |  | 4208 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.550 |
| walker |  | 4208 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.550 |
| walker |  | 4208 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.550 |
| walker |  | 4208 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.550 |
| walker |  | 4213 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.550 |
| walker |  | 4223 | 10 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.550 |
| walker |  | 4232 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.550 |
| walker |  | 4259 | 27 | python method at src/typeguard/_importhook.py:99 |  |  | 0.550 |
| walker |  | 4309 | 50 | python method at src/typeguard/_importhook.py:167 |  |  | 0.550 |
| walker |  | 4318 | 9 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.550 |
| walker |  | 4372 | 54 | python method at src/typeguard/_importhook.py:124 |  |  | 0.550 |
| walker |  | 4396 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.550 |
| ns | 4402 |  | 222 | TypeCheckMemo — class skeleton + __init__ | 2.8 |  | 0.551 |
| walker |  | 4495 | 99 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.552 |
| walker |  | 4573 | 78 | python method at src/typeguard/_importhook.py:56 |  |  | 0.539 |
| ns | 4573 |  | 171 | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | 2.9 |  | 0.539 |
| walker |  | 4678 | 105 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.540 |
| walker |  | 4678 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.540 |
| walker |  | 4678 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.540 |
| walker |  | 4678 | 0 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.540 |
| walker |  | 4710 | 32 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.540 |
| walker |  | 4730 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.540 |
| walker |  | 4751 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.540 |
| walker |  | 4836 | 85 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.545 |
| walker |  | 4857 | 21 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.546 |
| ns | 4926 |  | 353 | TypeguardFinder + ImportHookManager — class + key methods | 2.10 |  | 0.552 |
| walker |  | 4959 | 102 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.563 |
| walker |  | 5099 | 140 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.563 |
| walker |  | 5099 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.563 |
| walker |  | 5099 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.563 |
| walker |  | 5099 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.563 |
| walker |  | 5105 | 6 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.563 |
| walker |  | 5120 | 15 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.563 |
| walker |  | 5144 | 24 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.563 |
| walker |  | 5170 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.563 |
| ns | 5177 |  | 251 | warn_on_error + load_plugins — signatures | 2.11 |  | 0.554 |
| walker |  | 5196 | 26 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.554 |
| walker |  | 5224 | 28 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.555 |
| walker |  | 5252 | 28 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.555 |
| ns | 5358 |  | 181 | check_type_internal — signature + docstring | 2.12 |  | 0.544 |
| walker |  | 5364 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.544 |
| ns | 5642 |  | 284 | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | 2.13 | 2.2 | 0.568 |
| walker |  | 5721 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.568 |
| ns | 5813 |  | 171 | Unset sentinel + small _utils helpers | 2.14 |  | 0.558 |
| ns | 6041 |  | 228 | _checkers.py — every check_* function name (locations) | 3.1 | 2.9 | 0.543 |
| walker |  | 6306 | 585 | python method sigs in src/typeguard/_transformer.py |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.544 |
| walker |  | 6306 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.544 |
| walker |  | 6315 | 9 | python method at src/typeguard/_transformer.py:352 |  |  | 0.544 |
| walker |  | 6324 | 9 | python method at src/typeguard/_transformer.py:472 |  |  | 0.544 |
| walker |  | 6329 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.544 |
| walker |  | 6334 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.544 |
| walker |  | 6339 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.544 |
| walker |  | 6347 | 8 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.544 |
| walker |  | 6358 | 11 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.544 |
| walker |  | 6367 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.544 |
| walker |  | 6377 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.544 |
| walker |  | 6387 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.544 |
| walker |  | 6399 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.544 |
| walker |  | 6421 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.544 |
| walker |  | 6445 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.544 |
| walker |  | 6469 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.544 |
| walker |  | 6498 | 29 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.544 |
| walker |  | 6527 | 29 | python method body at src/typeguard/_transformer.py:466 body 467 |  |  | 0.544 |
| walker |  | 6558 | 31 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.544 |
| ns | 6639 |  | 598 | _checkers.py — origin_type_checkers dispatch table | 3.2 | 3.1 | 0.520 |
| walker |  | 6699 | 141 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.525 |
| walker |  | 6725 | 26 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.528 |
| walker |  | 6754 | 29 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.532 |
| walker |  | 6785 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.539 |
| walker |  | 6833 | 48 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.539 |
| walker |  | 6881 | 48 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.539 |
| walker |  | 6929 | 48 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.539 |
| walker |  | 6977 | 48 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.539 |
| walker |  | 7025 | 48 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.539 |
| walker |  | 7073 | 48 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.539 |
| ns | 7082 |  | 443 | _checkers.py — builtin_checker_lookup dispatch fallbacks | 3.3 | 3.2 | 0.521 |
| ns | 7148 |  | 66 | _transformer.py — class locations | 3.4 |  | 0.525 |
| walker |  | 7202 | 129 | python decl names surface in src/typeguard/_utils.py |  |  | 0.534 |
| walker |  | 7202 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.534 |
| walker |  | 7202 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.534 |
| walker |  | 7202 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.534 |
| walker |  | 7202 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.534 |
| walker |  | 7202 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.534 |
| walker |  | 7202 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.534 |
| walker |  | 7208 | 6 | python decl at src/typeguard/_utils.py:172 |  |  | 0.535 |
| walker |  | 7218 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.536 |
| walker |  | 7234 | 16 | python method sigs in src/typeguard/_utils.py |  |  | 0.539 |
| walker |  | 7234 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.539 |
| walker |  | 7241 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.541 |
| walker |  | 7302 | 61 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.541 |
| ns | 7354 |  | 206 | _transformer.py — TypeguardTransformer visit_* methods (locations) | 3.5 | 3.4 | 0.554 |
| walker |  | 7371 | 69 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.554 |
| walker |  | 7427 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.554 |
| walker |  | 7491 | 64 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.566 |
| walker |  | 7558 | 67 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.566 |
| walker |  | 7592 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.566 |
| ns | 7639 |  | 285 | _transformer.py — generator_names + annotated_names + ignore_decorators tables | 3.6 |  | 0.560 |
| walker |  | 7650 | 58 | python imports in src/typeguard/_config.py |  |  | 0.560 |
| ns | 7673 |  | 34 | _decorators.py — function locations | 3.7 |  | 0.560 |
| walker |  | 7693 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.560 |
| walker |  | 7848 | 155 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.568 |
| walker |  | 7865 | 17 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.568 |
| walker |  | 7882 | 17 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.568 |
| walker |  | 7922 | 40 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.568 |
| walker |  | 7962 | 40 | python method body at src/typeguard/_transformer.py:206 body 207 |  |  | 0.568 |
| ns | 8036 |  | 363 | docs/api.rst — public API by topic group (head) | 3.8 |  | 0.548 |
| walker |  | 8097 | 135 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.572 |
| ns | 8213 |  | 177 | docs/userguide.rst — H2 section locations | 3.9 |  | 0.562 |
| walker |  | 8244 | 147 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.576 |
| walker |  | 8320 | 76 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.576 |
| walker |  | 8363 | 43 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.584 |
| ns | 8392 |  | 179 | docs/features.rst — H2 section locations | 3.10 |  | 0.575 |
| walker |  | 8439 | 76 | python imports in src/typeguard/_suppression.py |  |  | 0.575 |
| walker |  | 8483 | 44 | python method body at src/typeguard/_transformer.py:328 body 329 |  |  | 0.575 |
| walker |  | 8641 | 158 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.575 |
| ns | 8692 |  | 300 | docs/api.rst — Custom checkers / Suppression / Exceptions sections | 4.1 | 3.8 | 0.562 |
| walker |  | 8860 | 219 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.563 |
| walker |  | 8994 | 134 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.571 |
| walker |  | 8994 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.571 |
| walker |  | 9042 | 48 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.571 |
| walker |  | 9090 | 48 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.571 |
| walker |  | 9138 | 48 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.571 |
| walker |  | 9186 | 48 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.571 |
| walker |  | 9204 | 18 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.571 |
| walker |  | 9252 | 48 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.571 |
| ns | 9266 |  | 574 | docs/extending.rst — writing a lookup + checker function (head) | 4.2 | 2.9 | 0.555 |
| walker |  | 9300 | 48 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.555 |
| walker |  | 9348 | 48 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.555 |
| walker |  | 9362 | 14 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.555 |
| walker |  | 9410 | 48 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.555 |
| walker |  | 9424 | 14 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.555 |
| walker |  | 9472 | 48 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.555 |
| walker |  | 9497 | 25 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.555 |
| walker |  | 9558 | 61 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.555 |
| ns | 9578 |  | 312 | docs/extending.rst — MySpecialType worked example | 4.3 | 4.2 | 0.545 |
| walker |  | 9625 | 67 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.545 |
| walker |  | 9660 | 35 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.545 |
| walker |  | 9677 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.545 |
| walker |  | 9685 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.545 |
| walker |  | 9735 | 50 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.545 |
| walker |  | 9786 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.553 |
| ns | 9827 |  | 249 | tests/test_checkers.py — TestX class locations | 4.4 |  | 0.543 |
| ns | 9976 |  | 149 | tests/test_typechecked.py — Test class + module test locations | 4.5 |  | 0.538 |
| walker |  | 9986 | 200 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.557 |
