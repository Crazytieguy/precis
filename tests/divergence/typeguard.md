Score(3000)=0.662 I=0.883 C=0.496 ns_rows≤3K=14/39 (reached=7 partial=3 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| ns | 75 |  | 41 | docs/ listing | 1.2 |  | 0.682 |
| walker |  | 110 | 72 | listing of 'src/typeguard' |  |  | 0.742 |
| ns | 147 |  | 72 | src/typeguard/ module listing | 1.3 |  | 0.799 |
| ns | 247 |  | 100 | tests/ listing | 1.4 |  | 0.618 |
| ns | 349 |  | 102 | README — one-paragraph lede | 1.5 |  | 0.581 |
| walker |  | 524 | 414 | python imports in src/typeguard/__init__.py |  |  | 0.603 |
| ns | 550 |  | 201 | README — check_type vs. code instrumentation modes | 1.6 | 1.5 | 0.522 |
| walker |  | 552 | 28 | python decl names surface in src/typeguard/__init__.py |  |  | 0.522 |
| walker |  | 552 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.522 |
| walker |  | 593 | 41 | listing of 'docs' |  |  | 0.650 |
| walker |  | 644 | 51 | [dependencies] in pyproject.toml |  |  | 0.650 |
| walker |  | 655 | 11 | python decl names surface in src/typeguard/_memo.py |  |  | 0.650 |
| walker |  | 672 | 17 | python decl at src/typeguard/_memo.py:8 |  |  | 0.650 |
| walker |  | 683 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.650 |
| ns | 700 |  | 150 | README — instrumentation options (@typechecked vs import hook) | 1.7 | 1.6 | 0.584 |
| walker |  | 705 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.584 |
| ns | 964 |  | 264 | Public re-exports — head of typeguard/__init__.py | 1.8 |  | 0.640 |
| ns | 1119 |  | 155 | Public re-exports — tail of typeguard/__init__.py | 1.9 | 1.8 | 0.662 |
| walker |  | 1123 | 418 | README headline in README.rst |  |  | 0.832 |
| walker |  | 1180 | 57 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.834 |
| walker |  | 1193 | 13 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.834 |
| walker |  | 1193 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.834 |
| walker |  | 1227 | 34 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.834 |
| walker |  | 1227 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.834 |
| walker |  | 1227 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.834 |
| walker |  | 1246 | 19 | python imports in src/typeguard/_exceptions.py |  |  | 0.834 |
| walker |  | 1346 | 100 | listing of 'tests' |  |  | 0.951 |
| ns | 1351 |  | 232 | typeguard/__init__.py — module rewrite, lazy `config`, autoload | 1.10 | 1.9 | 0.864 |
| walker |  | 1397 | 51 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.865 |
| walker |  | 1397 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.865 |
| walker |  | 1397 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1397 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1397 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.865 |
| walker |  | 1416 | 19 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1436 | 20 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1466 | 30 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.867 |
| walker |  | 1508 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.869 |
| walker |  | 1595 | 87 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.871 |
| walker |  | 1595 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.871 |
| walker |  | 1595 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.871 |
| walker |  | 1595 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.871 |
| walker |  | 1595 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.871 |
| walker |  | 1595 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.871 |
| walker |  | 1604 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.871 |
| walker |  | 1613 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.871 |
| walker |  | 1622 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.871 |
| ns | 1663 |  | 312 | _exceptions.py — class signatures + summary docstrings | 2.1 |  | 0.822 |
| walker |  | 1678 | 56 | python decl names surface in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1678 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1694 | 16 | python decl at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1717 | 23 | python decl at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1738 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1738 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.822 |
| walker |  | 1758 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1786 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.823 |
| walker |  | 1870 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.823 |
| walker |  | 1884 | 14 | listing of 'tests/mypy' |  |  | 0.823 |
| walker |  | 1953 | 69 | python method at src/typeguard/_memo.py:37 |  |  | 0.824 |
| walker |  | 1990 | 37 | python imports in src/typeguard/_memo.py |  |  | 0.825 |
| walker |  | 2015 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.838 |
| walker |  | 2093 | 78 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.838 |
| walker |  | 2093 | 0 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.838 |
| walker |  | 2093 | 0 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.838 |
| walker |  | 2107 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.839 |
| walker |  | 2151 | 44 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.839 |
| walker |  | 2171 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.839 |
| ns | 2249 |  | 586 | check_type — primary entry-point signature | 2.2 |  | 0.728 |
| walker |  | 2274 | 103 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.730 |
| walker |  | 2379 | 105 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.730 |
| walker |  | 2379 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.730 |
| walker |  | 2379 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.730 |
| walker |  | 2379 | 0 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.730 |
| walker |  | 2411 | 32 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.730 |
| walker |  | 2431 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.730 |
| walker |  | 2452 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.730 |
| walker |  | 2537 | 85 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.731 |
| walker |  | 2558 | 21 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.731 |
| walker |  | 2676 | 118 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.733 |
| ns | 2766 |  | 517 | @typechecked — overloaded signatures | 2.3 |  | 0.689 |
| walker |  | 2815 | 139 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.690 |
| walker |  | 2928 | 113 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.691 |
| walker |  | 2928 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.691 |
| walker |  | 2928 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.691 |
| walker |  | 2928 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.691 |
| walker |  | 2928 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.691 |
| walker |  | 2946 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.691 |
| walker |  | 2975 | 29 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.691 |
| ns | 2985 |  | 219 | install_import_hook — signature + docstring | 2.4 |  | 0.662 |
| walker |  | 3039 | 64 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.668 |
| walker |  | 3070 | 31 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.668 |
| walker |  | 3083 | 13 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.668 |
| walker |  | 3232 | 149 | python method sigs in src/typeguard/_importhook.py |  |  | 0.668 |
| walker |  | 3232 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.668 |
| walker |  | 3232 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.668 |
| walker |  | 3232 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.668 |
| walker |  | 3232 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.668 |
| walker |  | 3232 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.668 |
| walker |  | 3237 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.668 |
| walker |  | 3247 | 10 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.668 |
| walker |  | 3256 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.668 |
| walker |  | 3283 | 27 | python method at src/typeguard/_importhook.py:99 |  |  | 0.668 |
| walker |  | 3333 | 50 | python method at src/typeguard/_importhook.py:167 |  |  | 0.668 |
| ns | 3337 |  | 352 | suppress_type_checks — overloaded signatures + docstring | 2.5 |  | 0.630 |
| walker |  | 3342 | 9 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.630 |
| walker |  | 3396 | 54 | python method at src/typeguard/_importhook.py:124 |  |  | 0.630 |
| walker |  | 3420 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.630 |
| walker |  | 3519 | 99 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.631 |
| walker |  | 3597 | 78 | python method at src/typeguard/_importhook.py:56 |  |  | 0.631 |
| walker |  | 3661 | 64 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.633 |
| walker |  | 3791 | 130 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.664 |
| ns | 3817 |  | 480 | TypeCheckConfiguration — dataclass + attribute docstrings | 2.6 |  | 0.616 |
| walker |  | 3920 | 129 | python decl names surface in src/typeguard/_utils.py |  |  | 0.617 |
| walker |  | 3920 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.617 |
| walker |  | 3920 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.617 |
| walker |  | 3920 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.617 |
| walker |  | 3920 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.617 |
| walker |  | 3920 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.617 |
| walker |  | 3920 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.617 |
| walker |  | 3926 | 6 | python decl at src/typeguard/_utils.py:172 |  |  | 0.617 |
| walker |  | 3936 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.617 |
| walker |  | 3952 | 16 | python method sigs in src/typeguard/_utils.py |  |  | 0.617 |
| walker |  | 3952 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.617 |
| walker |  | 3959 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.617 |
| walker |  | 4020 | 61 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.617 |
| walker |  | 4089 | 69 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.617 |
| walker |  | 4145 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.617 |
| ns | 4180 |  | 363 | ForwardRefPolicy + CollectionCheckStrategy enums | 2.7 |  | 0.613 |
| walker |  | 4185 | 40 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.613 |
| walker |  | 4261 | 76 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.613 |
| walker |  | 4304 | 43 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.614 |
| ns | 4402 |  | 222 | TypeCheckMemo — class skeleton + __init__ | 2.8 |  | 0.623 |
| walker |  | 4450 | 146 | python decl names surface in src/typeguard/_functions.py |  |  | 0.623 |
| walker |  | 4450 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.623 |
| walker |  | 4494 | 44 | python decl at src/typeguard/_functions.py:118 |  |  | 0.624 |
| walker |  | 4539 | 45 | python decl at src/typeguard/_functions.py:149 |  |  | 0.624 |
| walker |  | 4558 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.624 |
| ns | 4573 |  | 171 | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | 2.9 |  | 0.610 |
| walker |  | 4604 | 46 | python decl at src/typeguard/_functions.py:185 |  |  | 0.610 |
| walker |  | 4650 | 46 | python decl at src/typeguard/_functions.py:216 |  |  | 0.611 |
| walker |  | 4697 | 47 | python decl at src/typeguard/_functions.py:245 |  |  | 0.612 |
| walker |  | 4777 | 80 | python decl at src/typeguard/_functions.py:39 |  |  | 0.612 |
| walker |  | 4786 | 9 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.612 |
| walker |  | 4867 | 81 | python decl at src/typeguard/_functions.py:28 |  |  | 0.612 |
| walker |  | 4876 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.612 |
| ns | 4926 |  | 353 | TypeguardFinder + ImportHookManager — class + key methods | 2.10 |  | 0.628 |
| walker |  | 4932 | 56 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.628 |
| walker |  | 5071 | 139 | python decl at src/typeguard/_functions.py:50 |  |  | 0.634 |
| walker |  | 5147 | 76 | python imports in src/typeguard/_suppression.py |  |  | 0.634 |
| ns | 5177 |  | 251 | warn_on_error + load_plugins — signatures | 2.11 |  | 0.623 |
| walker |  | 5349 | 202 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.625 |
| ns | 5358 |  | 181 | check_type_internal — signature + docstring | 2.12 |  | 0.612 |
| walker |  | 5431 | 82 | python imports in src/typeguard/_config.py |  |  | 0.612 |
| walker |  | 5619 | 188 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.642 |
| walker |  | 5627 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.642 |
| ns | 5642 |  | 284 | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | 2.13 | 2.2 | 0.658 |
| walker |  | 5677 | 50 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.658 |
| walker |  | 5728 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.670 |
| ns | 5813 |  | 171 | Unset sentinel + small _utils helpers | 2.14 |  | 0.673 |
| walker |  | 5827 | 99 | python decl body at src/typeguard/_utils.py:142 body 143 |  |  | 0.673 |
| walker |  | 5967 | 140 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.674 |
| walker |  | 5967 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.674 |
| walker |  | 5967 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.674 |
| walker |  | 5967 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.674 |
| walker |  | 5973 | 6 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.674 |
| walker |  | 5988 | 15 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.674 |
| walker |  | 6012 | 24 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.674 |
| walker |  | 6038 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.674 |
| ns | 6041 |  | 228 | _checkers.py — every check_* function name (locations) | 3.1 | 2.9 | 0.656 |
| walker |  | 6064 | 26 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.656 |
| walker |  | 6092 | 28 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.656 |
| walker |  | 6120 | 28 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.657 |
| walker |  | 6232 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.657 |
| walker |  | 6367 | 135 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.658 |
| walker |  | 6525 | 158 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.658 |
| ns | 6639 |  | 598 | _checkers.py — origin_type_checkers dispatch table | 3.2 | 3.1 | 0.629 |
| walker |  | 6666 | 141 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.634 |
| walker |  | 6692 | 26 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.636 |
| walker |  | 6721 | 29 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.640 |
| walker |  | 6752 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.646 |
| walker |  | 6800 | 48 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.646 |
| walker |  | 6848 | 48 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.646 |
| walker |  | 6896 | 48 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.646 |
| walker |  | 6944 | 48 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.646 |
| walker |  | 6992 | 48 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.646 |
| walker |  | 7040 | 48 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.646 |
| ns | 7082 |  | 443 | _checkers.py — builtin_checker_lookup dispatch fallbacks | 3.3 | 3.2 | 0.625 |
| ns | 7148 |  | 66 | _transformer.py — class locations | 3.4 |  | 0.628 |
| ns | 7354 |  | 206 | _transformer.py — TypeguardTransformer visit_* methods (locations) | 3.5 | 3.4 | 0.616 |
| walker |  | 7431 | 391 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:574 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:577 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.627 |
| walker |  | 7431 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.627 |
| walker |  | 7458 | 27 | python method at src/typeguard/_transformer.py:913 |  |  | 0.627 |
| walker |  | 7488 | 30 | python method at src/typeguard/_transformer.py:650 |  |  | 0.627 |
| walker |  | 7503 | 15 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.627 |
| walker |  | 7537 | 34 | python method at src/typeguard/_transformer.py:489 |  |  | 0.627 |
| walker |  | 7549 | 12 | python method body at src/typeguard/_transformer.py:913 body 916 |  |  | 0.627 |
| walker |  | 7570 | 21 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.627 |
| walker |  | 7590 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.627 |
| walker |  | 7625 | 35 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.627 |
| ns | 7639 |  | 285 | _transformer.py — generator_names + annotated_names + ignore_decorators tables | 3.6 |  | 0.641 |
| walker |  | 7664 | 39 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.641 |
| ns | 7673 |  | 34 | _decorators.py — function locations | 3.7 |  | 0.641 |
| walker |  | 7713 | 49 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.641 |
| walker |  | 7765 | 52 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.641 |
| walker |  | 7818 | 53 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.641 |
| walker |  | 7885 | 67 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.641 |
| walker |  | 7928 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.641 |
| walker |  | 7945 | 17 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.641 |
| walker |  | 7962 | 17 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.641 |
| ns | 8036 |  | 363 | docs/api.rst — public API by topic group (head) | 3.8 |  | 0.618 |
| ns | 8213 |  | 177 | docs/userguide.rst — H2 section locations | 3.9 |  | 0.608 |
| walker |  | 8319 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.608 |
| walker |  | 8376 | 57 | python method body at src/typeguard/_transformer.py:608 body 609 |  |  | 0.608 |
| ns | 8392 |  | 179 | docs/features.rst — H2 section locations | 3.10 |  | 0.598 |
| walker |  | 8653 | 277 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.626 |
| walker |  | 8677 | 24 | python decl body at src/typeguard/_checkers.py:153 body 159 |  |  | 0.626 |
| ns | 8692 |  | 300 | docs/api.rst — Custom checkers / Suppression / Exceptions sections | 4.1 | 3.8 | 0.612 |
| walker |  | 8739 | 62 | python method body at src/typeguard/_config.py:52 body 53 |  |  | 0.623 |
| walker |  | 8802 | 63 | python method body at src/typeguard/_transformer.py:600 body 601 |  |  | 0.623 |
| walker |  | 8929 | 127 | python imports in src/typeguard/_pytest_plugin.py |  |  | 0.623 |
| walker |  | 9057 | 128 | python decl body at src/typeguard/_checkers.py:305 body 311 |  |  | 0.623 |
| walker |  | 9188 | 131 | python decl body at src/typeguard/_checkers.py:324 body 330 |  |  | 0.623 |
| walker |  | 9221 | 33 | python method body at src/typeguard/_transformer.py:570 body 571 |  |  | 0.623 |
| ns | 9266 |  | 574 | docs/extending.rst — writing a lookup + checker function (head) | 4.2 | 2.9 | 0.605 |
| ns | 9578 |  | 312 | docs/extending.rst — MySpecialType worked example | 4.3 | 4.2 | 0.594 |
| walker |  | 9806 | 585 | python method sigs in src/typeguard/_transformer.py |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.603 |
| walker |  | 9806 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.603 |
| walker |  | 9815 | 9 | python method at src/typeguard/_transformer.py:352 |  |  | 0.603 |
| walker |  | 9824 | 9 | python method at src/typeguard/_transformer.py:472 |  |  | 0.603 |
| ns | 9827 |  | 249 | tests/test_checkers.py — TestX class locations | 4.4 |  | 0.592 |
| walker |  | 9829 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.592 |
| walker |  | 9834 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.592 |
| walker |  | 9839 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.592 |
| walker |  | 9847 | 8 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.592 |
| walker |  | 9858 | 11 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.592 |
| walker |  | 9867 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.592 |
| walker |  | 9877 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.592 |
| walker |  | 9887 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.592 |
| walker |  | 9899 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.592 |
| walker |  | 9921 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.592 |
| walker |  | 9945 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.592 |
| walker |  | 9969 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.592 |
| ns | 9976 |  | 149 | tests/test_typechecked.py — Test class + module test locations | 4.5 |  | 0.587 |
| walker |  | 9998 | 29 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.587 |
