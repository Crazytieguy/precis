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
| ns | 1299 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.642 |
| walker |  | 1307 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.642 |
| walker |  | 1307 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.642 |
| walker |  | 1307 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.642 |
| ns | 1355 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.628 |
| walker |  | 1376 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.657 |
| walker |  | 1476 | 100 | listing of 'tests' |  |  | 0.748 |
| walker |  | 1497 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.748 |
| walker |  | 1546 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.748 |
| walker |  | 1546 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.748 |
| walker |  | 1546 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.748 |
| walker |  | 1546 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.748 |
| walker |  | 1546 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.748 |
| ns | 1563 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.689 |
| walker |  | 1567 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.689 |
| walker |  | 1589 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.690 |
| walker |  | 1621 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.690 |
| ns | 1661 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.670 |
| walker |  | 1663 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.670 |
| walker |  | 1744 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.672 |
| walker |  | 1744 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.672 |
| walker |  | 1744 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.672 |
| walker |  | 1744 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.672 |
| walker |  | 1744 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.672 |
| walker |  | 1744 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.672 |
| walker |  | 1753 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.672 |
| walker |  | 1762 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.672 |
| walker |  | 1771 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.672 |
| walker |  | 1785 | 14 | listing of 'tests/mypy' |  |  | 0.675 |
| walker |  | 1852 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.675 |
| walker |  | 1860 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.675 |
| walker |  | 1870 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.675 |
| ns | 1896 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.629 |
| walker |  | 1904 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.629 |
| walker |  | 1918 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.629 |
| walker |  | 1938 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.629 |
| walker |  | 2006 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.629 |
| walker |  | 2006 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.629 |
| walker |  | 2006 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.629 |
| walker |  | 2015 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.629 |
| walker |  | 2036 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.629 |
| walker |  | 2036 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.629 |
| walker |  | 2056 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.630 |
| walker |  | 2084 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.630 |
| walker |  | 2106 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.630 |
| walker |  | 2190 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.631 |
| ns | 2201 |  | 305 | _checkers.py: every check_* function's signature | 5.1 |  | 0.582 |
| walker |  | 2261 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.583 |
| walker |  | 2688 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.675 |
| walker |  | 2688 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.675 |
| walker |  | 2688 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.675 |
| walker |  | 2688 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.675 |
| walker |  | 2712 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.675 |
| walker |  | 2739 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.675 |
| ns | 2744 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.608 |
| walker |  | 2776 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.608 |
| walker |  | 2814 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.608 |
| walker |  | 2845 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.608 |
| walker |  | 2895 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.608 |
| walker |  | 2945 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.608 |
| walker |  | 2995 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.608 |
| walker |  | 3045 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.608 |
| ns | 3090 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.572 |
| walker |  | 3095 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.572 |
| walker |  | 3145 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.572 |
| walker |  | 3195 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.572 |
| walker |  | 3245 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.572 |
| walker |  | 3295 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.572 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.553 |
| walker |  | 3345 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.553 |
| walker |  | 3361 | 16 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.553 |
| walker |  | 3411 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.553 |
| walker |  | 3461 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.553 |
| walker |  | 3511 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.553 |
| walker |  | 3523 | 12 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.553 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.533 |
| walker |  | 3573 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.533 |
| walker |  | 3585 | 12 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.533 |
| walker |  | 3635 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.533 |
| walker |  | 3685 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.533 |
| walker |  | 3735 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.533 |
| walker |  | 3785 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.533 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.515 |
| walker |  | 3835 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.515 |
| walker |  | 3885 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.515 |
| walker |  | 3935 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.515 |
| walker |  | 3946 | 11 | python decl body at src/typeguard/_checkers.py:915 body 921 |  |  | 0.515 |
| walker |  | 3969 | 23 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.515 |
| walker |  | 4032 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.515 |
| walker |  | 4101 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.515 |
| walker |  | 4133 | 32 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.515 |
| walker |  | 4166 | 33 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.515 |
| walker |  | 4183 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.515 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.494 |
| walker |  | 4355 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.494 |
| walker |  | 4355 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.494 |
| walker |  | 4355 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.494 |
| walker |  | 4355 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.494 |
| walker |  | 4355 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.494 |
| walker |  | 4363 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.494 |
| walker |  | 4385 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.494 |
| walker |  | 4402 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.494 |
| walker |  | 4426 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.494 |
| walker |  | 4452 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.495 |
| walker |  | 4478 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.495 |
| walker |  | 4504 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.495 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.499 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.493 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.478 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.465 |
| walker |  | 5503 | 999 | python method sigs in src/typeguard/_transformer.py |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.466 |
| walker |  | 5503 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.466 |
| walker |  | 5514 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.466 |
| walker |  | 5525 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.466 |
| walker |  | 5530 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.466 |
| walker |  | 5535 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.466 |
| walker |  | 5540 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.466 |
| walker |  | 5547 | 7 | python method at src/typeguard/_transformer.py:577 |  |  | 0.466 |
| walker |  | 5547 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.466 |
| walker |  | 5553 | 6 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.466 |
| walker |  | 5562 | 9 | python method at src/typeguard/_transformer.py:574 |  |  | 0.466 |
| walker |  | 5562 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.466 |
| walker |  | 5575 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.466 |
| walker |  | 5584 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.466 |
| walker |  | 5613 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.466 |
| walker |  | 5645 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.466 |
| walker |  | 5655 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.466 |
| walker |  | 5665 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.466 |
| walker |  | 5675 | 10 | python method body at src/typeguard/_transformer.py:913 body 916 |  |  | 0.466 |
| walker |  | 5692 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.466 |
| walker |  | 5728 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.466 |
| walker |  | 5740 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.466 |
| walker |  | 5763 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.466 |
| walker |  | 5875 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.466 |
| walker |  | 5895 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.466 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.441 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.438 |
| walker |  | 6250 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.440 |
| walker |  | 6272 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.440 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.433 |
| walker |  | 6315 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.433 |
| walker |  | 6361 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.433 |
| walker |  | 6385 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.433 |
| walker |  | 6409 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.433 |
| walker |  | 6434 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.438 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.435 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.432 |
| walker |  | 6564 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.441 |
| walker |  | 6564 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.441 |
| walker |  | 6610 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.441 |
| walker |  | 6657 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.441 |
| walker |  | 6705 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.441 |
| walker |  | 6753 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.441 |
| walker |  | 6802 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.441 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.432 |
| walker |  | 6821 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.432 |
| walker |  | 6912 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.432 |
| walker |  | 6921 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.432 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.427 |
| walker |  | 7011 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.427 |
| walker |  | 7018 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.427 |
| walker |  | 7084 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.427 |
| walker |  | 7209 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.434 |
| walker |  | 7231 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.437 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.427 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.429 |
| walker |  | 7320 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.432 |
| walker |  | 7320 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.432 |
| walker |  | 7320 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.432 |
| walker |  | 7330 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.432 |
| walker |  | 7364 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.432 |
| walker |  | 7384 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.432 |
| walker |  | 7405 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.435 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.432 |
| walker |  | 7502 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.433 |
| walker |  | 7521 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.433 |
| walker |  | 7625 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.439 |
| walker |  | 7647 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.442 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.454 |
| walker |  | 7764 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.458 |
| walker |  | 7764 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.458 |
| walker |  | 7764 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.458 |
| walker |  | 7764 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.458 |
| walker |  | 7764 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.458 |
| walker |  | 7782 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.458 |
| walker |  | 7831 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.460 |
| walker |  | 7862 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.460 |
| walker |  | 7895 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.460 |
| walker |  | 7906 | 11 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.460 |
| walker |  | 8057 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.469 |
| walker |  | 8057 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.469 |
| walker |  | 8057 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.469 |
| walker |  | 8057 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.469 |
| walker |  | 8057 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.469 |
| walker |  | 8057 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.469 |
| walker |  | 8062 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.469 |
| walker |  | 8074 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.469 |
| walker |  | 8083 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.469 |
| walker |  | 8112 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.469 |
| walker |  | 8164 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.469 |
| walker |  | 8171 | 7 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.469 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.459 |
| walker |  | 8227 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.459 |
| walker |  | 8303 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.459 |
| walker |  | 8327 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.459 |
| walker |  | 8436 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.459 |
| walker |  | 8549 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.459 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.474 |
| walker |  | 8594 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.481 |
| walker |  | 8650 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.481 |
| walker |  | 8781 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.484 |
| walker |  | 8781 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.484 |
| walker |  | 8781 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.484 |
| walker |  | 8781 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.484 |
| walker |  | 8781 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.484 |
| walker |  | 8781 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.484 |
| walker |  | 8781 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.484 |
| walker |  | 8789 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.485 |
| walker |  | 8799 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.485 |
| walker |  | 8813 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.485 |
| walker |  | 8813 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.485 |
| walker |  | 8820 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.485 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.476 |
| walker |  | 8893 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.476 |
| walker |  | 8974 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.476 |
| walker |  | 9030 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.476 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.471 |
| walker |  | 9090 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.471 |
| walker |  | 9151 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.471 |
| walker |  | 9182 | 31 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.471 |
| walker |  | 9225 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.471 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.465 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.462 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.460 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.456 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.451 |
