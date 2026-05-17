Score(3000)=0.659 I=0.878 C=0.495 ns_rows≤3K=14/39 (reached=7 partial=3 missing=4)

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
| walker |  | 655 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.650 |
| walker |  | 666 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.650 |
| walker |  | 688 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.651 |
| ns | 700 |  | 150 | README — instrumentation options (@typechecked vs import hook) | 1.7 | 1.6 | 0.584 |
| ns | 964 |  | 264 | Public re-exports — head of typeguard/__init__.py | 1.8 |  | 0.640 |
| walker |  | 1106 | 418 | README headline in README.rst |  |  | 0.827 |
| ns | 1119 |  | 155 | Public re-exports — tail of typeguard/__init__.py | 1.9 | 1.8 | 0.832 |
| walker |  | 1163 | 57 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.834 |
| walker |  | 1176 | 13 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.834 |
| walker |  | 1176 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.834 |
| walker |  | 1210 | 34 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.834 |
| walker |  | 1210 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.834 |
| walker |  | 1210 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.834 |
| walker |  | 1229 | 19 | python imports in src/typeguard/_exceptions.py |  |  | 0.834 |
| walker |  | 1329 | 100 | listing of 'tests' |  |  | 0.951 |
| ns | 1351 |  | 232 | typeguard/__init__.py — module rewrite, lazy `config`, autoload | 1.10 | 1.9 | 0.864 |
| walker |  | 1409 | 80 | python imports in tests/__init__.py |  |  | 0.864 |
| walker |  | 1460 | 51 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.865 |
| walker |  | 1460 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.865 |
| walker |  | 1460 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1460 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1460 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.865 |
| walker |  | 1479 | 19 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.865 |
| walker |  | 1499 | 20 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.865 |
| walker |  | 1529 | 30 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.867 |
| walker |  | 1571 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.869 |
| walker |  | 1658 | 87 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.871 |
| walker |  | 1658 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.871 |
| walker |  | 1658 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.871 |
| walker |  | 1658 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.871 |
| walker |  | 1658 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.871 |
| walker |  | 1658 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.871 |
| ns | 1663 |  | 312 | _exceptions.py — class signatures + summary docstrings | 2.1 |  | 0.822 |
| walker |  | 1667 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.822 |
| walker |  | 1676 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.822 |
| walker |  | 1685 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.822 |
| walker |  | 1741 | 56 | python decl names surface in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1741 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1741 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1748 | 7 | python decl at src/typeguard/_config.py:62 |  |  | 0.822 |
| walker |  | 1769 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.822 |
| walker |  | 1769 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.822 |
| walker |  | 1789 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.822 |
| walker |  | 1817 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.822 |
| walker |  | 1901 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.823 |
| walker |  | 1915 | 14 | listing of 'tests/mypy' |  |  | 0.823 |
| walker |  | 1984 | 69 | python method at src/typeguard/_memo.py:37 |  |  | 0.824 |
| walker |  | 2021 | 37 | python imports in src/typeguard/_memo.py |  |  | 0.825 |
| walker |  | 2046 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.838 |
| walker |  | 2124 | 78 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.838 |
| walker |  | 2124 | 0 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.838 |
| walker |  | 2124 | 0 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.838 |
| walker |  | 2156 | 32 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.838 |
| walker |  | 2170 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.838 |
| walker |  | 2190 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.839 |
| ns | 2249 |  | 586 | check_type — primary entry-point signature | 2.2 |  | 0.728 |
| walker |  | 2293 | 103 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.729 |
| walker |  | 2398 | 105 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.730 |
| walker |  | 2398 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.730 |
| walker |  | 2398 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.730 |
| walker |  | 2398 | 0 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.730 |
| walker |  | 2430 | 32 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.730 |
| walker |  | 2450 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.730 |
| walker |  | 2471 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.730 |
| walker |  | 2556 | 85 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.730 |
| walker |  | 2577 | 21 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.731 |
| walker |  | 2679 | 102 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.732 |
| ns | 2766 |  | 517 | @typechecked — overloaded signatures | 2.3 |  | 0.685 |
| walker |  | 2792 | 113 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.685 |
| walker |  | 2792 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.685 |
| walker |  | 2792 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.685 |
| walker |  | 2792 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.685 |
| walker |  | 2792 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.685 |
| walker |  | 2810 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.685 |
| walker |  | 2839 | 29 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.685 |
| walker |  | 2886 | 47 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.685 |
| walker |  | 2917 | 31 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.685 |
| walker |  | 2930 | 13 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.685 |
| ns | 2985 |  | 219 | install_import_hook — signature + docstring | 2.4 |  | 0.659 |
| walker |  | 3079 | 149 | python method sigs in src/typeguard/_importhook.py |  |  | 0.660 |
| walker |  | 3079 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.660 |
| walker |  | 3079 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.660 |
| walker |  | 3079 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.660 |
| walker |  | 3079 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.660 |
| walker |  | 3079 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.660 |
| walker |  | 3084 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.660 |
| walker |  | 3094 | 10 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.660 |
| walker |  | 3103 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.660 |
| walker |  | 3130 | 27 | python method at src/typeguard/_importhook.py:99 |  |  | 0.660 |
| walker |  | 3180 | 50 | python method at src/typeguard/_importhook.py:167 |  |  | 0.660 |
| walker |  | 3189 | 9 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.660 |
| walker |  | 3243 | 54 | python method at src/typeguard/_importhook.py:124 |  |  | 0.660 |
| walker |  | 3267 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.660 |
| ns | 3337 |  | 352 | suppress_type_checks — overloaded signatures + docstring | 2.5 |  | 0.621 |
| walker |  | 3366 | 99 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.622 |
| walker |  | 3444 | 78 | python method at src/typeguard/_importhook.py:56 |  |  | 0.622 |
| walker |  | 3508 | 64 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.623 |
| walker |  | 3663 | 155 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.624 |
| walker |  | 3792 | 129 | python decl names surface in src/typeguard/_utils.py |  |  | 0.625 |
| walker |  | 3792 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.625 |
| walker |  | 3792 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.625 |
| walker |  | 3792 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.625 |
| walker |  | 3792 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.625 |
| walker |  | 3792 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.625 |
| walker |  | 3792 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.625 |
| walker |  | 3798 | 6 | python decl at src/typeguard/_utils.py:172 |  |  | 0.625 |
| walker |  | 3808 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.625 |
| ns | 3817 |  | 480 | TypeCheckConfiguration — dataclass + attribute docstrings | 2.6 |  | 0.580 |
| walker |  | 3824 | 16 | python method sigs in src/typeguard/_utils.py |  |  | 0.580 |
| walker |  | 3824 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.580 |
| walker |  | 3831 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.580 |
| walker |  | 3892 | 61 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.580 |
| walker |  | 3961 | 69 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.580 |
| walker |  | 4017 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.580 |
| walker |  | 4057 | 40 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.580 |
| ns | 4180 |  | 363 | ForwardRefPolicy + CollectionCheckStrategy enums | 2.7 |  | 0.580 |
| walker |  | 4204 | 147 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.607 |
| walker |  | 4280 | 76 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.607 |
| walker |  | 4323 | 43 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.608 |
| ns | 4402 |  | 222 | TypeCheckMemo — class skeleton + __init__ | 2.8 |  | 0.618 |
| walker |  | 4469 | 146 | python decl names surface in src/typeguard/_functions.py |  |  | 0.618 |
| walker |  | 4469 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.618 |
| walker |  | 4513 | 44 | python decl at src/typeguard/_functions.py:118 |  |  | 0.618 |
| walker |  | 4558 | 45 | python decl at src/typeguard/_functions.py:149 |  |  | 0.619 |
| ns | 4573 |  | 171 | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | 2.9 |  | 0.604 |
| walker |  | 4577 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.604 |
| walker |  | 4623 | 46 | python decl at src/typeguard/_functions.py:185 |  |  | 0.605 |
| walker |  | 4669 | 46 | python decl at src/typeguard/_functions.py:216 |  |  | 0.606 |
| walker |  | 4716 | 47 | python decl at src/typeguard/_functions.py:245 |  |  | 0.607 |
| walker |  | 4796 | 80 | python decl at src/typeguard/_functions.py:39 |  |  | 0.607 |
| walker |  | 4805 | 9 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.607 |
| walker |  | 4886 | 81 | python decl at src/typeguard/_functions.py:28 |  |  | 0.607 |
| walker |  | 4895 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.607 |
| ns | 4926 |  | 353 | TypeguardFinder + ImportHookManager — class + key methods | 2.10 |  | 0.623 |
| walker |  | 4951 | 56 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.623 |
| walker |  | 5074 | 123 | python decl at src/typeguard/_functions.py:50 |  |  | 0.628 |
| walker |  | 5150 | 76 | python imports in src/typeguard/_suppression.py |  |  | 0.628 |
| ns | 5177 |  | 251 | warn_on_error + load_plugins — signatures | 2.11 |  | 0.617 |
| walker |  | 5232 | 82 | python imports in src/typeguard/_config.py |  |  | 0.617 |
| ns | 5358 |  | 181 | check_type_internal — signature + docstring | 2.12 |  | 0.605 |
| walker |  | 5451 | 219 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.607 |
| walker |  | 5459 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.607 |
| walker |  | 5509 | 50 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.607 |
| walker |  | 5560 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.620 |
| ns | 5642 |  | 284 | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | 2.13 | 2.2 | 0.638 |
| walker |  | 5760 | 200 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.666 |
| ns | 5813 |  | 171 | Unset sentinel + small _utils helpers | 2.14 |  | 0.669 |
| walker |  | 5859 | 99 | python decl body at src/typeguard/_utils.py:142 body 143 |  |  | 0.669 |
| walker |  | 5999 | 140 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.670 |
| walker |  | 5999 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.670 |
| walker |  | 5999 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.670 |
| walker |  | 5999 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.670 |
| walker |  | 6005 | 6 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.670 |
| walker |  | 6020 | 15 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.670 |
| ns | 6041 |  | 228 | _checkers.py — every check_* function name (locations) | 3.1 | 2.9 | 0.652 |
| walker |  | 6044 | 24 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.652 |
| walker |  | 6070 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.652 |
| walker |  | 6096 | 26 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.652 |
| walker |  | 6124 | 28 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.652 |
| walker |  | 6152 | 28 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.653 |
| walker |  | 6264 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.653 |
| walker |  | 6399 | 135 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.655 |
| walker |  | 6557 | 158 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.655 |
| ns | 6639 |  | 598 | _checkers.py — origin_type_checkers dispatch table | 3.2 | 3.1 | 0.625 |
| walker |  | 6698 | 141 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.630 |
| walker |  | 6724 | 26 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.632 |
| walker |  | 6753 | 29 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.636 |
| walker |  | 6784 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.643 |
| walker |  | 6832 | 48 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.643 |
| walker |  | 6880 | 48 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.643 |
| walker |  | 6928 | 48 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.643 |
| walker |  | 6976 | 48 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.643 |
| walker |  | 7024 | 48 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.643 |
| walker |  | 7072 | 48 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.643 |
| ns | 7082 |  | 443 | _checkers.py — builtin_checker_lookup dispatch fallbacks | 3.3 | 3.2 | 0.621 |
| ns | 7148 |  | 66 | _transformer.py — class locations | 3.4 |  | 0.624 |
| ns | 7354 |  | 206 | _transformer.py — TypeguardTransformer visit_* methods (locations) | 3.5 | 3.4 | 0.613 |
| walker |  | 7463 | 391 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:574 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:577 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.624 |
| walker |  | 7463 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.624 |
| walker |  | 7490 | 27 | python method at src/typeguard/_transformer.py:913 |  |  | 0.624 |
| walker |  | 7520 | 30 | python method at src/typeguard/_transformer.py:650 |  |  | 0.624 |
| walker |  | 7535 | 15 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.624 |
| walker |  | 7569 | 34 | python method at src/typeguard/_transformer.py:489 |  |  | 0.624 |
| walker |  | 7581 | 12 | python method body at src/typeguard/_transformer.py:913 body 916 |  |  | 0.624 |
| walker |  | 7602 | 21 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.624 |
| walker |  | 7622 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.624 |
| ns | 7639 |  | 285 | _transformer.py — generator_names + annotated_names + ignore_decorators tables | 3.6 |  | 0.638 |
| walker |  | 7657 | 35 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.638 |
| ns | 7673 |  | 34 | _decorators.py — function locations | 3.7 |  | 0.637 |
| walker |  | 7696 | 39 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.637 |
| walker |  | 7745 | 49 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.637 |
| walker |  | 7797 | 52 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.637 |
| walker |  | 7850 | 53 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.637 |
| walker |  | 7917 | 67 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.637 |
| walker |  | 7960 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.637 |
| walker |  | 7977 | 17 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.637 |
| walker |  | 7994 | 17 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.637 |
| ns | 8036 |  | 363 | docs/api.rst — public API by topic group (head) | 3.8 |  | 0.615 |
| ns | 8213 |  | 177 | docs/userguide.rst — H2 section locations | 3.9 |  | 0.605 |
| walker |  | 8351 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.605 |
| ns | 8392 |  | 179 | docs/features.rst — H2 section locations | 3.10 |  | 0.595 |
| walker |  | 8408 | 57 | python method body at src/typeguard/_transformer.py:608 body 609 |  |  | 0.595 |
| walker |  | 8432 | 24 | python decl body at src/typeguard/_checkers.py:153 body 159 |  |  | 0.595 |
| ns | 8692 |  | 300 | docs/api.rst — Custom checkers / Suppression / Exceptions sections | 4.1 | 3.8 | 0.582 |
| walker |  | 8725 | 293 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.610 |
| walker |  | 8787 | 62 | python method body at src/typeguard/_config.py:52 body 53 |  |  | 0.621 |
| walker |  | 8850 | 63 | python method body at src/typeguard/_transformer.py:600 body 601 |  |  | 0.621 |
| walker |  | 8977 | 127 | python imports in src/typeguard/_pytest_plugin.py |  |  | 0.621 |
| walker |  | 9105 | 128 | python decl body at src/typeguard/_checkers.py:305 body 311 |  |  | 0.621 |
| walker |  | 9236 | 131 | python decl body at src/typeguard/_checkers.py:324 body 330 |  |  | 0.621 |
| ns | 9266 |  | 574 | docs/extending.rst — writing a lookup + checker function (head) | 4.2 | 2.9 | 0.603 |
| walker |  | 9269 | 33 | python method body at src/typeguard/_transformer.py:570 body 571 |  |  | 0.603 |
| ns | 9578 |  | 312 | docs/extending.rst — MySpecialType worked example | 4.3 | 4.2 | 0.592 |
| ns | 9827 |  | 249 | tests/test_checkers.py — TestX class locations | 4.4 |  | 0.582 |
| walker |  | 9854 | 585 | python method sigs in src/typeguard/_transformer.py |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.590 |
| walker |  | 9854 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.590 |
| walker |  | 9863 | 9 | python method at src/typeguard/_transformer.py:352 |  |  | 0.590 |
| walker |  | 9872 | 9 | python method at src/typeguard/_transformer.py:472 |  |  | 0.590 |
| walker |  | 9877 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.590 |
| walker |  | 9882 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.590 |
| walker |  | 9887 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.590 |
| walker |  | 9895 | 8 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.590 |
| walker |  | 9906 | 11 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.590 |
| walker |  | 9915 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.590 |
| walker |  | 9925 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.590 |
| walker |  | 9935 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.590 |
| walker |  | 9947 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.590 |
| walker |  | 9969 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.590 |
| ns | 9976 |  | 149 | tests/test_typechecked.py — Test class + module test locations | 4.5 |  | 0.585 |
| walker |  | 9993 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.585 |
