Score(3000)=0.618 I=0.826 C=0.463 ns_rows≤3K=16/45 (reached=7 partial=2 missing=7)

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
| walker |  | 1298 | 27 | listing of '.github' |  |  | 0.710 |
| ns | 1299 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.657 |
| walker |  | 1306 | 8 | listing of '.github/workflows' |  |  | 0.668 |
| ns | 1355 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.654 |
| ns | 1563 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.603 |
| ns | 1661 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.585 |
| walker |  | 1781 | 475 | YAML config at .github/workflows/test.yml |  |  | 0.593 |
| walker |  | 1817 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.593 |
| walker |  | 1817 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.593 |
| walker |  | 1817 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.593 |
| walker |  | 1886 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.618 |
| ns | 1896 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.650 |
| walker |  | 1986 | 100 | listing of 'tests' |  |  | 0.722 |
| walker |  | 2000 | 14 | listing of 'tests/mypy' |  |  | 0.739 |
| walker |  | 2021 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.739 |
| walker |  | 2070 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.739 |
| walker |  | 2070 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.739 |
| walker |  | 2070 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.739 |
| walker |  | 2070 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.739 |
| walker |  | 2070 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.739 |
| walker |  | 2091 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.739 |
| walker |  | 2113 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.739 |
| walker |  | 2145 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.740 |
| walker |  | 2187 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.740 |
| ns | 2201 |  | 305 | _checkers.py: every check_* function's signature | 5.1 |  | 0.683 |
| walker |  | 2268 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.684 |
| walker |  | 2268 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.684 |
| walker |  | 2268 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.684 |
| walker |  | 2268 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.684 |
| walker |  | 2268 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.684 |
| walker |  | 2268 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.684 |
| walker |  | 2277 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.685 |
| walker |  | 2286 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.685 |
| walker |  | 2295 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.685 |
| walker |  | 2362 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.685 |
| walker |  | 2370 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.685 |
| walker |  | 2380 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.685 |
| walker |  | 2414 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.685 |
| walker |  | 2428 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.685 |
| walker |  | 2448 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.685 |
| walker |  | 2516 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.685 |
| walker |  | 2516 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.685 |
| walker |  | 2516 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.685 |
| walker |  | 2525 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.685 |
| walker |  | 2546 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.685 |
| walker |  | 2546 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.685 |
| walker |  | 2566 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.686 |
| walker |  | 2594 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.686 |
| walker |  | 2616 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.686 |
| walker |  | 2700 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.687 |
| ns | 2744 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.618 |
| walker |  | 2771 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.618 |
| ns | 3090 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.582 |
| walker |  | 3198 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.654 |
| walker |  | 3198 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.654 |
| walker |  | 3198 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.654 |
| walker |  | 3198 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.654 |
| walker |  | 3222 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.654 |
| walker |  | 3249 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.654 |
| walker |  | 3286 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.654 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.632 |
| walker |  | 3324 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.632 |
| walker |  | 3355 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.632 |
| walker |  | 3405 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.632 |
| walker |  | 3455 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.632 |
| walker |  | 3505 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.632 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.607 |
| walker |  | 3555 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.607 |
| walker |  | 3605 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.607 |
| walker |  | 3655 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.607 |
| walker |  | 3705 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.609 |
| walker |  | 3755 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.609 |
| walker |  | 3805 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.609 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.587 |
| walker |  | 3855 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.587 |
| walker |  | 3871 | 16 | python decl body at src/typeguard/_checkers.py:536 body 542 |  |  | 0.587 |
| walker |  | 3921 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.587 |
| walker |  | 3971 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.587 |
| walker |  | 4021 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.587 |
| walker |  | 4033 | 12 | python decl body at src/typeguard/_checkers.py:623 body 629 |  |  | 0.587 |
| walker |  | 4083 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.587 |
| walker |  | 4095 | 12 | python decl body at src/typeguard/_checkers.py:632 body 638 |  |  | 0.587 |
| walker |  | 4145 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.587 |
| walker |  | 4195 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.587 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.561 |
| walker |  | 4245 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.561 |
| walker |  | 4295 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.561 |
| walker |  | 4345 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.561 |
| walker |  | 4395 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.561 |
| walker |  | 4445 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.561 |
| walker |  | 4456 | 11 | python decl body at src/typeguard/_checkers.py:915 body 921 |  |  | 0.561 |
| walker |  | 4479 | 23 | python decl body at src/typeguard/_checkers.py:641 body 647 |  |  | 0.561 |
| walker |  | 4542 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.561 |
| walker |  | 4611 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.561 |
| walker |  | 4643 | 32 | python decl body at src/typeguard/_checkers.py:885 body 891 |  |  | 0.561 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.560 |
| walker |  | 4676 | 33 | python decl body at src/typeguard/_checkers.py:545 body 551 |  |  | 0.560 |
| walker |  | 4693 | 17 | python decl body at src/typeguard/_checkers.py:586 body 587 |  |  | 0.560 |
| walker |  | 4865 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.560 |
| walker |  | 4865 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.560 |
| walker |  | 4865 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.560 |
| walker |  | 4865 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.560 |
| walker |  | 4865 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.560 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.551 |
| walker |  | 4873 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.551 |
| walker |  | 4895 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.551 |
| walker |  | 4912 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.551 |
| walker |  | 4936 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.552 |
| walker |  | 4962 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.552 |
| walker |  | 4988 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.552 |
| walker |  | 5014 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.552 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.535 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.520 |
| walker |  | 6013 | 999 | python method sigs in src/typeguard/_transformer.py |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.521 |
| walker |  | 6013 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.521 |
| walker |  | 6024 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.521 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.493 |
| walker |  | 6035 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.493 |
| walker |  | 6040 | 5 | python method body at src/typeguard/_transformer.py:306 body 307 |  |  | 0.493 |
| walker |  | 6045 | 5 | python method body at src/typeguard/_transformer.py:309 body 310 |  |  | 0.493 |
| walker |  | 6050 | 5 | python method body at src/typeguard/_transformer.py:325 body 326 |  |  | 0.493 |
| walker |  | 6057 | 7 | python method at src/typeguard/_transformer.py:577 |  |  | 0.493 |
| walker |  | 6057 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.493 |
| walker |  | 6063 | 6 | python method body at src/typeguard/_transformer.py:472 body 474 |  |  | 0.493 |
| walker |  | 6072 | 9 | python method at src/typeguard/_transformer.py:574 |  |  | 0.493 |
| walker |  | 6072 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.493 |
| walker |  | 6085 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.493 |
| walker |  | 6094 | 9 | python method body at src/typeguard/_transformer.py:334 body 335 |  |  | 0.493 |
| walker |  | 6123 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.493 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.490 |
| walker |  | 6155 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.490 |
| walker |  | 6165 | 10 | python method body at src/typeguard/_transformer.py:319 body 320 |  |  | 0.490 |
| walker |  | 6175 | 10 | python method body at src/typeguard/_transformer.py:322 body 323 |  |  | 0.490 |
| walker |  | 6185 | 10 | python method body at src/typeguard/_transformer.py:913 body 916 |  |  | 0.490 |
| walker |  | 6202 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.490 |
| walker |  | 6238 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.490 |
| walker |  | 6250 | 12 | python method body at src/typeguard/_transformer.py:286 body 287 |  |  | 0.490 |
| walker |  | 6273 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.490 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.483 |
| walker |  | 6385 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.483 |
| walker |  | 6405 | 20 | python method body at src/typeguard/_transformer.py:596 body 597 |  |  | 0.483 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.479 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.476 |
| walker |  | 6760 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.477 |
| walker |  | 6782 | 22 | python method body at src/typeguard/_transformer.py:302 body 303 |  |  | 0.477 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.466 |
| walker |  | 6825 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.466 |
| walker |  | 6871 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.466 |
| walker |  | 6895 | 24 | python method body at src/typeguard/_transformer.py:289 body 290 |  |  | 0.466 |
| walker |  | 6919 | 24 | python method body at src/typeguard/_transformer.py:293 body 294 |  |  | 0.466 |
| walker |  | 6944 | 25 | python method body at src/typeguard/_exceptions.py:31 body 32 |  |  | 0.471 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.466 |
| walker |  | 7074 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.474 |
| walker |  | 7074 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.474 |
| walker |  | 7120 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.474 |
| walker |  | 7167 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.475 |
| walker |  | 7215 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.475 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.464 |
| walker |  | 7263 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.464 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.466 |
| walker |  | 7312 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.466 |
| walker |  | 7331 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.466 |
| walker |  | 7422 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.466 |
| walker |  | 7431 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.466 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.462 |
| walker |  | 7521 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.462 |
| walker |  | 7528 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.462 |
| walker |  | 7594 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.462 |
| walker |  | 7719 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.469 |
| walker |  | 7741 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.471 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.482 |
| walker |  | 7830 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.485 |
| walker |  | 7830 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.485 |
| walker |  | 7830 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.485 |
| walker |  | 7840 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.485 |
| walker |  | 7874 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.485 |
| walker |  | 7894 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.485 |
| walker |  | 7915 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.488 |
| walker |  | 8012 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.490 |
| walker |  | 8031 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.490 |
| walker |  | 8135 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.495 |
| walker |  | 8157 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.497 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.486 |
| walker |  | 8274 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.490 |
| walker |  | 8274 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.490 |
| walker |  | 8274 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.490 |
| walker |  | 8274 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.490 |
| walker |  | 8274 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.490 |
| walker |  | 8292 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.490 |
| walker |  | 8341 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.491 |
| walker |  | 8372 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.491 |
| walker |  | 8405 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.491 |
| walker |  | 8416 | 11 | python decl body at src/typeguard/_importhook.py:45 body 48 |  |  | 0.491 |
| walker |  | 8567 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.500 |
| walker |  | 8567 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.500 |
| walker |  | 8567 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.500 |
| walker |  | 8567 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.500 |
| walker |  | 8567 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.500 |
| walker |  | 8567 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.500 |
| walker |  | 8572 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.500 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.513 |
| walker |  | 8584 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.513 |
| walker |  | 8593 | 9 | python method body at src/typeguard/_importhook.py:161 body 162 |  |  | 0.513 |
| walker |  | 8622 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.513 |
| walker |  | 8674 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.513 |
| walker |  | 8681 | 7 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.513 |
| walker |  | 8737 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.513 |
| walker |  | 8813 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.513 |
| walker |  | 8837 | 24 | python method body at src/typeguard/_importhook.py:120 body 121 |  |  | 0.513 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.504 |
| walker |  | 8946 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.504 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.499 |
| walker |  | 9059 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.499 |
| walker |  | 9104 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.505 |
| walker |  | 9160 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.505 |
| walker |  | 9291 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.508 |
| walker |  | 9291 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.508 |
| walker |  | 9291 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.508 |
| walker |  | 9291 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.508 |
| walker |  | 9291 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.508 |
| walker |  | 9291 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.508 |
| walker |  | 9291 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.508 |
| walker |  | 9299 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.509 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.502 |
| walker |  | 9309 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.502 |
| walker |  | 9323 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.502 |
| walker |  | 9323 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.502 |
| walker |  | 9330 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.502 |
| walker |  | 9403 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.502 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.499 |
| walker |  | 9484 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.499 |
| walker |  | 9540 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.499 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.497 |
| walker |  | 9600 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.497 |
| walker |  | 9661 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.497 |
| walker |  | 9692 | 31 | python method body at src/typeguard/_transformer.py:297 body 298 |  |  | 0.497 |
| walker |  | 9735 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.497 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.492 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.487 |
