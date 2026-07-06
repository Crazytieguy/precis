Score(3000)=0.539 I=0.794 C=0.366 ns_rows≤3K=16/45 (reached=6 partial=1 missing=9)

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
| walker |  | 2299 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.583 |
| walker |  | 2425 | 126 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.599 |
| walker |  | 2475 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.599 |
| walker |  | 2525 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.599 |
| walker |  | 2575 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.599 |
| walker |  | 2625 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.599 |
| walker |  | 2675 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.599 |
| walker |  | 2725 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.599 |
| ns | 2744 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.539 |
| walker |  | 2775 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.539 |
| walker |  | 2825 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.539 |
| walker |  | 2875 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.539 |
| walker |  | 2925 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.539 |
| walker |  | 2941 | 16 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.539 |
| walker |  | 2991 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.539 |
| walker |  | 3054 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.539 |
| walker |  | 3089 | 35 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.539 |
| ns | 3090 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.507 |
| walker |  | 3219 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.508 |
| walker |  | 3219 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.508 |
| walker |  | 3265 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.508 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.491 |
| walker |  | 3312 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.491 |
| walker |  | 3360 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.491 |
| walker |  | 3408 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.491 |
| walker |  | 3457 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.491 |
| walker |  | 3476 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.491 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.474 |
| walker |  | 3567 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.474 |
| walker |  | 3576 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.474 |
| walker |  | 3666 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.474 |
| walker |  | 3673 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.474 |
| walker |  | 3739 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.474 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.460 |
| walker |  | 3864 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.460 |
| walker |  | 3886 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.460 |
| walker |  | 3975 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.461 |
| walker |  | 3975 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.461 |
| walker |  | 3975 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.461 |
| walker |  | 3985 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.461 |
| walker |  | 4019 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.461 |
| walker |  | 4039 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.461 |
| walker |  | 4060 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.461 |
| walker |  | 4157 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.461 |
| walker |  | 4176 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.461 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.444 |
| walker |  | 4280 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.444 |
| walker |  | 4302 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.444 |
| walker |  | 4419 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.445 |
| walker |  | 4419 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.445 |
| walker |  | 4419 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.445 |
| walker |  | 4419 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.445 |
| walker |  | 4419 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.445 |
| walker |  | 4437 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.445 |
| walker |  | 4486 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.445 |
| walker |  | 4517 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.445 |
| walker |  | 4550 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.445 |
| walker |  | 4561 | 11 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.445 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.461 |
| walker |  | 4712 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.462 |
| walker |  | 4712 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.462 |
| walker |  | 4712 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.462 |
| walker |  | 4712 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.462 |
| walker |  | 4712 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.462 |
| walker |  | 4712 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.462 |
| walker |  | 4717 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.462 |
| walker |  | 4729 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.462 |
| walker |  | 4738 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.462 |
| walker |  | 4767 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.462 |
| walker |  | 4819 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.462 |
| walker |  | 4826 | 7 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.462 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.458 |
| walker |  | 4882 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.458 |
| walker |  | 4952 | 70 | python method at src/typeguard/_importhook.py:56 |  |  | 0.458 |
| walker |  | 4976 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.458 |
| walker |  | 5085 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.458 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.455 |
| walker |  | 5198 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.455 |
| walker |  | 5243 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.466 |
| walker |  | 5374 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.466 |
| walker |  | 5374 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.466 |
| walker |  | 5374 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.466 |
| walker |  | 5374 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.466 |
| walker |  | 5374 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.466 |
| walker |  | 5374 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.466 |
| walker |  | 5374 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.466 |
| walker |  | 5382 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.467 |
| walker |  | 5392 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.467 |
| walker |  | 5406 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.467 |
| walker |  | 5406 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.467 |
| walker |  | 5413 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.467 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.454 |
| walker |  | 5486 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.454 |
| walker |  | 5567 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.454 |
| walker |  | 5623 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.454 |
| walker |  | 5782 | 159 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.454 |
| walker |  | 5782 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.454 |
| walker |  | 5782 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.454 |
| walker |  | 5782 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.454 |
| walker |  | 5782 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.454 |
| walker |  | 5790 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.454 |
| walker |  | 5812 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.454 |
| walker |  | 5829 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.454 |
| walker |  | 5853 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.454 |
| walker |  | 5879 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.454 |
| walker |  | 5905 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.454 |
| walker |  | 5931 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.454 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.430 |
| walker |  | 6045 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.430 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.436 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.439 |
| walker |  | 6402 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.441 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.437 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.443 |
| walker |  | 6651 | 249 | python method sigs in src/typeguard/_transformer.py |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.443 |
| walker |  | 6651 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.443 |
| walker |  | 6664 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.443 |
| walker |  | 6676 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.443 |
| walker |  | 6700 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.443 |
| walker |  | 6724 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.443 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.433 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.443 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.435 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.436 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.437 |
| walker |  | 7578 | 854 | manifest config in pyproject.toml |  |  | 0.509 |
| walker |  | 7611 | 33 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.509 |
| walker |  | 7744 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.510 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.508 |
| walker |  | 7820 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.508 |
| walker |  | 7883 | 63 | python imports in src/typeguard/_config.py |  |  | 0.516 |
| walker |  | 7921 | 38 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.516 |
| walker |  | 8096 | 175 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.516 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.521 |
| walker |  | 8345 | 249 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.521 |
| walker |  | 8345 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.521 |
| walker |  | 8356 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.521 |
| walker |  | 8361 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.521 |
| walker |  | 8366 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.521 |
| walker |  | 8371 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.521 |
| walker |  | 8380 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.521 |
| walker |  | 8390 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.521 |
| walker |  | 8400 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.521 |
| walker |  | 8422 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.521 |
| walker |  | 8456 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.521 |
| walker |  | 8492 | 36 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.521 |
| walker |  | 8533 | 41 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.529 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.542 |
| walker |  | 8689 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.564 |
| walker |  | 8770 | 81 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.564 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.554 |
| walker |  | 8937 | 167 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.554 |
| walker |  | 9018 | 81 | python imports in src/typeguard/_suppression.py |  |  | 0.558 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.552 |
| walker |  | 9062 | 44 | python method body at src/typeguard/_transformer.py:328 body 329 |  |  | 0.552 |
| walker |  | 9107 | 45 | python method body at src/typeguard/_transformer.py:206 body 207 |  |  | 0.552 |
| walker |  | 9115 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.552 |
| walker |  | 9163 | 48 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.552 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.545 |
| walker |  | 9311 | 148 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.561 |
| walker |  | 9311 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.561 |
| walker |  | 9311 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.561 |
| walker |  | 9361 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.561 |
| walker |  | 9411 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.561 |
| walker |  | 9423 | 12 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.561 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.557 |
| walker |  | 9473 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.557 |
| walker |  | 9485 | 12 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.557 |
| walker |  | 9535 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.557 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.555 |
| walker |  | 9585 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.555 |
| walker |  | 9635 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.555 |
| walker |  | 9685 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.555 |
| walker |  | 9735 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.555 |
| walker |  | 9785 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.555 |
| walker |  | 9808 | 23 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.555 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.550 |
| walker |  | 9877 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.550 |
| walker |  | 9909 | 32 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.550 |
| walker |  | 9926 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.550 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.544 |
| walker |  | 10000 | 74 | python decl body at src/typeguard/_checkers.py:651 body 657 |  |  | 0.544 |
