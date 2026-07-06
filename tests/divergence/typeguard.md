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
| walker |  | 781 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.619 |
| ns | 841 |  | 198 | typeguard/__init__.py: import re-exports (checkers/config/decorators) | 3.1 |  | 0.656 |
| ns | 1135 |  | 294 | typeguard/__init__.py: import re-exports (exceptions/functions/importhook/memo/suppression/utils) + rebinding loop | 3.2 | 3.1 | 0.647 |
| walker |  | 1248 | 467 | README headline in README.rst |  |  | 0.694 |
| walker |  | 1271 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.694 |
| walker |  | 1284 | 13 | python decl names surface #1 in src/typeguard/_transformer.py |  |  | 0.694 |
| ns | 1299 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.642 |
| walker |  | 1353 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.671 |
| ns | 1355 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.656 |
| walker |  | 1389 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.657 |
| walker |  | 1389 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.657 |
| walker |  | 1389 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.657 |
| walker |  | 1410 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.657 |
| walker |  | 1459 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.657 |
| walker |  | 1459 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.657 |
| walker |  | 1459 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.657 |
| walker |  | 1459 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.657 |
| walker |  | 1459 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.657 |
| walker |  | 1480 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.657 |
| walker |  | 1502 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.657 |
| walker |  | 1534 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.657 |
| ns | 1563 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.606 |
| walker |  | 1576 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.607 |
| walker |  | 1657 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.608 |
| walker |  | 1657 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.608 |
| walker |  | 1657 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.608 |
| walker |  | 1657 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.608 |
| walker |  | 1657 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.608 |
| walker |  | 1657 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.608 |
| ns | 1661 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.590 |
| walker |  | 1666 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.590 |
| walker |  | 1675 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.590 |
| walker |  | 1684 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.591 |
| walker |  | 1784 | 100 | listing of 'tests' |  |  | 0.672 |
| walker |  | 1851 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.672 |
| walker |  | 1859 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.672 |
| walker |  | 1869 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.672 |
| walker |  | 1883 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.672 |
| ns | 1896 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.626 |
| walker |  | 1917 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.626 |
| walker |  | 1937 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.626 |
| walker |  | 1951 | 14 | listing of 'tests/mypy' |  |  | 0.629 |
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
| walker |  | 3202 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.507 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.490 |
| walker |  | 3332 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.491 |
| walker |  | 3332 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.491 |
| walker |  | 3351 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.491 |
| walker |  | 3397 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.491 |
| walker |  | 3444 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.491 |
| walker |  | 3492 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.491 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.474 |
| walker |  | 3540 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.474 |
| walker |  | 3589 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.474 |
| walker |  | 3680 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.474 |
| walker |  | 3689 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.474 |
| walker |  | 3779 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.474 |
| walker |  | 3786 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.474 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.460 |
| walker |  | 3852 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.460 |
| walker |  | 3977 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.460 |
| walker |  | 3999 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.460 |
| walker |  | 4088 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.461 |
| walker |  | 4088 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.461 |
| walker |  | 4088 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.461 |
| walker |  | 4098 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.461 |
| walker |  | 4132 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.461 |
| walker |  | 4152 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.461 |
| walker |  | 4173 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.461 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.444 |
| walker |  | 4270 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.444 |
| walker |  | 4289 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.444 |
| walker |  | 4393 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.444 |
| walker |  | 4415 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.444 |
| walker |  | 4532 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.445 |
| walker |  | 4532 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.445 |
| walker |  | 4532 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.445 |
| walker |  | 4532 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.445 |
| walker |  | 4532 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.445 |
| walker |  | 4550 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.445 |
| walker |  | 4581 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.445 |
| walker |  | 4630 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.445 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.461 |
| walker |  | 4663 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.461 |
| walker |  | 4674 | 11 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.461 |
| walker |  | 4821 | 147 | python method sigs in src/typeguard/_importhook.py |  |  | 0.462 |
| walker |  | 4821 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.462 |
| walker |  | 4821 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.462 |
| walker |  | 4821 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.462 |
| walker |  | 4821 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.462 |
| walker |  | 4821 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.462 |
| walker |  | 4826 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.462 |
| walker |  | 4838 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.462 |
| walker |  | 4847 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.462 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.458 |
| walker |  | 4876 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.458 |
| walker |  | 4928 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.458 |
| walker |  | 4935 | 7 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.458 |
| walker |  | 4991 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.458 |
| walker |  | 5015 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.458 |
| walker |  | 5095 | 80 | python method at src/typeguard/_importhook.py:56 |  |  | 0.458 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.455 |
| walker |  | 5204 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.455 |
| walker |  | 5249 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.466 |
| walker |  | 5380 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.466 |
| walker |  | 5380 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.466 |
| walker |  | 5380 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.466 |
| walker |  | 5380 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.466 |
| walker |  | 5380 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.466 |
| walker |  | 5380 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.466 |
| walker |  | 5380 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.466 |
| walker |  | 5388 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.467 |
| walker |  | 5398 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.467 |
| walker |  | 5412 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.467 |
| walker |  | 5412 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.467 |
| walker |  | 5419 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.467 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.454 |
| walker |  | 5492 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.454 |
| walker |  | 5573 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.454 |
| walker |  | 5629 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.454 |
| walker |  | 5788 | 159 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.454 |
| walker |  | 5788 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.454 |
| walker |  | 5788 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.454 |
| walker |  | 5788 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.454 |
| walker |  | 5788 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.454 |
| walker |  | 5796 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.454 |
| walker |  | 5813 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.454 |
| walker |  | 5835 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.454 |
| walker |  | 5859 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.454 |
| walker |  | 5885 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.454 |
| walker |  | 5911 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.454 |
| walker |  | 5937 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.454 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.430 |
| walker |  | 6051 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.430 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.436 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.439 |
| walker |  | 6408 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.441 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.437 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.443 |
| walker |  | 6657 | 249 | python method sigs in src/typeguard/_transformer.py |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.443 |
| walker |  | 6657 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.443 |
| walker |  | 6670 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.443 |
| walker |  | 6682 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.443 |
| walker |  | 6706 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.443 |
| walker |  | 6730 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.443 |
| walker |  | 6763 | 33 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.443 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.433 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.443 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.435 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.436 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.437 |
| walker |  | 7617 | 854 | manifest config in pyproject.toml |  |  | 0.509 |
| walker |  | 7693 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.509 |
| walker |  | 7756 | 63 | python imports in src/typeguard/_config.py |  |  | 0.517 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.514 |
| walker |  | 7794 | 38 | python method body at src/typeguard/_importhook.py:175 body 177 |  |  | 0.514 |
| walker |  | 7927 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.516 |
| walker |  | 8102 | 175 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.516 |
| walker |  | 8143 | 41 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.524 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.529 |
| walker |  | 8224 | 81 | python decl body at src/typeguard/_utils.py:162 body 163 |  |  | 0.529 |
| walker |  | 8391 | 167 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.529 |
| walker |  | 8436 | 45 | python method body at src/typeguard/_transformer.py:206 body 207 |  |  | 0.529 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.542 |
| walker |  | 8685 | 249 | python method sigs #1 in src/typeguard/_transformer.py |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.542 |
| walker |  | 8685 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.542 |
| walker |  | 8696 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.542 |
| walker |  | 8701 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.542 |
| walker |  | 8706 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.542 |
| walker |  | 8711 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.542 |
| walker |  | 8720 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.542 |
| walker |  | 8730 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.542 |
| walker |  | 8740 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.542 |
| walker |  | 8762 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.542 |
| walker |  | 8796 | 34 | python method body at src/typeguard/_transformer.py:347 body 348 |  |  | 0.542 |
| walker |  | 8832 | 36 | python method body at src/typeguard/_transformer.py:401 body 402 |  |  | 0.542 |
| walker |  | 8876 | 44 | python method body at src/typeguard/_transformer.py:328 body 329 |  |  | 0.542 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.532 |
| walker |  | 8957 | 81 | python imports in src/typeguard/_suppression.py |  |  | 0.536 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.530 |
| walker |  | 9113 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.552 |
| walker |  | 9161 | 48 | python method body at src/typeguard/_importhook.py:99 body 102 |  |  | 0.552 |
| walker |  | 9169 | 8 | python decl body at src/typeguard/_functions.py:118 body 146 |  |  | 0.552 |
| walker |  | 9220 | 51 | python method body at src/typeguard/_exceptions.py:38 body 39 |  |  | 0.559 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.552 |
| walker |  | 9368 | 148 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.568 |
| walker |  | 9368 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.568 |
| walker |  | 9368 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.568 |
| walker |  | 9418 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.568 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.564 |
| walker |  | 9468 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.564 |
| walker |  | 9480 | 12 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.564 |
| walker |  | 9530 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.564 |
| walker |  | 9542 | 12 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.564 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.562 |
| walker |  | 9592 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.562 |
| walker |  | 9642 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.562 |
| walker |  | 9692 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.562 |
| walker |  | 9742 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.562 |
| walker |  | 9792 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.562 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.557 |
| walker |  | 9842 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.557 |
| walker |  | 9865 | 23 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.557 |
| walker |  | 9934 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.557 |
| walker |  | 9966 | 32 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.557 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.551 |
| walker |  | 9983 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.551 |
