Score(3000)=0.608 I=0.811 C=0.455 ns_rows≤3K=16/45 (reached=7 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 93 | 55 | [dependencies] in pyproject.toml |  |  | 1.000 |
| ns | 130 |  | 96 | README lede opening | 1.2 |  | 0.815 |
| walker |  | 163 | 70 | [package] in pyproject.toml |  |  | 0.845 |
| walker |  | 235 | 72 | listing of 'src/typeguard' |  |  | 0.845 |
| ns | 241 |  | 111 | Package identity: name, Python support, deps | 1.3 |  | 0.844 |
| ns | 367 |  | 126 | Project URLs + pytest entry point | 1.4 |  | 0.700 |
| ns | 480 |  | 113 | Source + docs file rosters | 2.1 |  | 0.616 |
| ns | 580 |  | 100 | Tests directory roster | 2.2 |  | 0.524 |
| ns | 643 |  | 63 | CI, community-health, and mypy-fixture file rosters | 2.3 |  | 0.468 |
| walker |  | 656 | 421 | python imports in src/typeguard/__init__.py |  |  | 0.484 |
| walker |  | 694 | 38 | python decl names surface in src/typeguard/__init__.py |  |  | 0.485 |
| walker |  | 694 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.485 |
| walker |  | 735 | 41 | listing of 'docs' |  |  | 0.619 |
| walker |  | 748 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.619 |
| walker |  | 748 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.619 |
| walker |  | 759 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.619 |
| ns | 841 |  | 198 | typeguard/__init__.py: import re-exports (checkers/config/decorators) | 3.1 |  | 0.656 |
| ns | 1135 |  | 294 | typeguard/__init__.py: import re-exports (exceptions/functions/importhook/memo/suppression/utils) + rebinding loop | 3.2 | 3.1 | 0.647 |
| walker |  | 1226 | 467 | README headline in README.rst |  |  | 0.694 |
| walker |  | 1248 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.694 |
| walker |  | 1271 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.694 |
| walker |  | 1284 | 13 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.694 |
| ns | 1299 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.642 |
| walker |  | 1320 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.642 |
| walker |  | 1320 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.642 |
| walker |  | 1320 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.642 |
| ns | 1355 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.628 |
| walker |  | 1389 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.657 |
| walker |  | 1489 | 100 | listing of 'tests' |  |  | 0.748 |
| walker |  | 1510 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.748 |
| walker |  | 1559 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.748 |
| walker |  | 1559 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.748 |
| walker |  | 1559 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.748 |
| walker |  | 1559 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.748 |
| walker |  | 1559 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.748 |
| ns | 1563 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.689 |
| walker |  | 1580 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.689 |
| walker |  | 1602 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.690 |
| walker |  | 1634 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.690 |
| ns | 1661 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.670 |
| walker |  | 1676 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.670 |
| walker |  | 1757 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.672 |
| walker |  | 1757 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.672 |
| walker |  | 1757 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.672 |
| walker |  | 1757 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.672 |
| walker |  | 1757 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.672 |
| walker |  | 1757 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.672 |
| walker |  | 1766 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.672 |
| walker |  | 1775 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.672 |
| walker |  | 1784 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.672 |
| walker |  | 1798 | 14 | listing of 'tests/mypy' |  |  | 0.675 |
| walker |  | 1865 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.675 |
| walker |  | 1873 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.675 |
| walker |  | 1883 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.675 |
| ns | 1896 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.629 |
| walker |  | 1917 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.629 |
| walker |  | 1931 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.629 |
| walker |  | 1951 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.629 |
| walker |  | 2019 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.629 |
| walker |  | 2019 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.629 |
| walker |  | 2019 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.629 |
| walker |  | 2028 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.629 |
| walker |  | 2049 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.629 |
| walker |  | 2049 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.629 |
| walker |  | 2069 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.630 |
| walker |  | 2097 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.630 |
| walker |  | 2119 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.630 |
| ns | 2201 |  | 305 | _checkers.py: every check_* function's signature | 5.1 |  | 0.582 |
| walker |  | 2203 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.582 |
| walker |  | 2274 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.583 |
| walker |  | 2598 | 324 | python decl sigs roster in src/typeguard/_checkers.py |  |  | 0.675 |
| walker |  | 2598 | 0 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.675 |
| walker |  | 2598 | 0 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.675 |
| walker |  | 2598 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.675 |
| walker |  | 2598 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.675 |
| walker |  | 2648 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.675 |
| walker |  | 2698 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.675 |
| ns | 2744 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.608 |
| walker |  | 2748 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.608 |
| walker |  | 2798 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.608 |
| walker |  | 2848 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.608 |
| walker |  | 2898 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.608 |
| walker |  | 2948 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.608 |
| walker |  | 2998 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.608 |
| walker |  | 3048 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.608 |
| ns | 3090 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.572 |
| walker |  | 3098 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.572 |
| walker |  | 3114 | 16 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.572 |
| walker |  | 3164 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.572 |
| walker |  | 3214 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.572 |
| walker |  | 3264 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.572 |
| walker |  | 3276 | 12 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.572 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.553 |
| walker |  | 3326 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.553 |
| walker |  | 3338 | 12 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.553 |
| walker |  | 3388 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.553 |
| walker |  | 3438 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.553 |
| walker |  | 3488 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.553 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.533 |
| walker |  | 3538 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.533 |
| walker |  | 3588 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.533 |
| walker |  | 3638 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.533 |
| walker |  | 3661 | 23 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.533 |
| walker |  | 3724 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.533 |
| walker |  | 3793 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.533 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.515 |
| walker |  | 3825 | 32 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.515 |
| walker |  | 3858 | 33 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.515 |
| walker |  | 3875 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.515 |
| walker |  | 3900 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.516 |
| walker |  | 4030 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.516 |
| walker |  | 4030 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.516 |
| walker |  | 4076 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.516 |
| walker |  | 4123 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.516 |
| walker |  | 4171 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.516 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.495 |
| walker |  | 4219 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.495 |
| walker |  | 4268 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.495 |
| walker |  | 4287 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.495 |
| walker |  | 4378 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.495 |
| walker |  | 4387 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.495 |
| walker |  | 4477 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.495 |
| walker |  | 4484 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.495 |
| walker |  | 4550 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.495 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.505 |
| walker |  | 4675 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.506 |
| walker |  | 4697 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.506 |
| walker |  | 4786 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.506 |
| walker |  | 4786 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.506 |
| walker |  | 4786 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.506 |
| walker |  | 4796 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.506 |
| walker |  | 4830 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.506 |
| walker |  | 4850 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.506 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.500 |
| walker |  | 4871 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.500 |
| walker |  | 4968 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.500 |
| walker |  | 4987 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.500 |
| walker |  | 5091 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.501 |
| walker |  | 5113 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.501 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.496 |
| walker |  | 5230 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.497 |
| walker |  | 5230 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.497 |
| walker |  | 5230 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.497 |
| walker |  | 5230 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.497 |
| walker |  | 5230 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.497 |
| walker |  | 5248 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.497 |
| walker |  | 5297 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.497 |
| walker |  | 5328 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.497 |
| walker |  | 5361 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.497 |
| walker |  | 5372 | 11 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.497 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.483 |
| walker |  | 5523 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.484 |
| walker |  | 5523 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.484 |
| walker |  | 5523 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.484 |
| walker |  | 5523 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.484 |
| walker |  | 5523 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.484 |
| walker |  | 5523 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.484 |
| walker |  | 5528 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.484 |
| walker |  | 5540 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.484 |
| walker |  | 5549 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.484 |
| walker |  | 5578 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.484 |
| walker |  | 5630 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.484 |
| walker |  | 5637 | 7 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.484 |
| walker |  | 5693 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.484 |
| walker |  | 5769 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.484 |
| walker |  | 5793 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.484 |
| walker |  | 5902 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.484 |
| walker |  | 6015 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.484 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.457 |
| walker |  | 6060 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.467 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.472 |
| walker |  | 6191 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.472 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.472 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.472 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.472 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.472 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.472 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.472 |
| walker |  | 6199 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.472 |
| walker |  | 6209 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.472 |
| walker |  | 6223 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.472 |
| walker |  | 6223 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.472 |
| walker |  | 6230 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.472 |
| walker |  | 6303 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.472 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.475 |
| walker |  | 6384 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.475 |
| walker |  | 6440 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.475 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.471 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.476 |
| walker |  | 6599 | 159 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.476 |
| walker |  | 6599 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.476 |
| walker |  | 6599 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.476 |
| walker |  | 6599 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.476 |
| walker |  | 6599 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.476 |
| walker |  | 6607 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.476 |
| walker |  | 6629 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.476 |
| walker |  | 6646 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.476 |
| walker |  | 6670 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.476 |
| walker |  | 6696 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.476 |
| walker |  | 6722 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.476 |
| walker |  | 6748 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.476 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.466 |
| walker |  | 6862 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.466 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.475 |
| walker |  | 7219 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.476 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.467 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.469 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.469 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.465 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.455 |
| walker |  | 8214 | 995 | python method sigs roster in src/typeguard/_transformer.py |  |  | 0.470 |
| walker |  | 8214 | 0 | python method sigs in src/typeguard/_transformer.py |  |  | 0.470 |
| walker |  | 8214 | 0 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.470 |
| walker |  | 8214 | 0 | python method sigs #2 in src/typeguard/_transformer.py |  |  | 0.470 |
| walker |  | 8214 | 0 | python method sigs #3 in src/typeguard/_transformer.py |  |  | 0.470 |
| walker |  | 8214 | 0 | python method sigs #4 in src/typeguard/_transformer.py |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.470 |
| walker |  | 8214 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.470 |
| walker |  | 8225 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.470 |
| walker |  | 8236 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.470 |
| walker |  | 8241 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.470 |
| walker |  | 8246 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.470 |
| walker |  | 8251 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.470 |
| walker |  | 8258 | 7 | python method at src/typeguard/_transformer.py:577 |  |  | 0.470 |
| walker |  | 8258 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.470 |
| walker |  | 8264 | 6 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.470 |
| walker |  | 8273 | 9 | python method at src/typeguard/_transformer.py:574 |  |  | 0.470 |
| walker |  | 8273 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.470 |
| walker |  | 8286 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.470 |
| walker |  | 8295 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.470 |
| walker |  | 8324 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.470 |
| walker |  | 8356 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.470 |
| walker |  | 8366 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.470 |
| walker |  | 8376 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.470 |
| walker |  | 8386 | 10 | python method body at src/typeguard/_transformer.py:913 body 916 |  |  | 0.470 |
| walker |  | 8403 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.470 |
| walker |  | 8439 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.470 |
| walker |  | 8451 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.470 |
| walker |  | 8474 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.470 |
| walker |  | 8494 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.470 |
| walker |  | 8516 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.470 |
| walker |  | 8559 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.470 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.485 |
| walker |  | 8605 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.485 |
| walker |  | 8629 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.485 |
| walker |  | 8653 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.485 |
| walker |  | 8709 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.485 |
| walker |  | 8769 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.485 |
| walker |  | 8830 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.485 |
| walker |  | 8861 | 31 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.485 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.476 |
| walker |  | 8904 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.476 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.471 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.465 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.462 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.460 |
| walker |  | 9758 | 854 | manifest config in pyproject.toml |  |  | 0.521 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.516 |
| walker |  | 9830 | 72 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.519 |
| walker |  | 9864 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.519 |
| walker |  | 9898 | 34 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.519 |
| walker |  | 9932 | 34 | python method body at src/typeguard/_transformer.py:466 body 467 |  |  | 0.519 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.514 |
