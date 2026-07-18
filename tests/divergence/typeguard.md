Score(3000)=0.692 I=0.841 C=0.570 ns_rows≤3K=16/45 (reached=8 partial=2 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 93 | 55 | [dependencies] in pyproject.toml |  |  | 1.000 |
| ns | 130 |  | 96 | README lede opening | 1.2 |  | 0.815 |
| walker |  | 134 | 41 | listing of 'docs' |  |  | 0.823 |
| walker |  | 204 | 70 | [package] in pyproject.toml |  |  | 0.845 |
| ns | 241 |  | 111 | Package identity: name, Python support, deps | 1.3 |  | 0.826 |
| walker |  | 276 | 72 | listing of 'src/typeguard' |  |  | 0.867 |
| ns | 367 |  | 126 | Project URLs + pytest entry point | 1.4 |  | 0.729 |
| ns | 480 |  | 113 | Source + docs file rosters | 2.1 |  | 0.789 |
| ns | 580 |  | 100 | Tests directory roster | 2.2 |  | 0.671 |
| ns | 643 |  | 63 | CI, community-health, and mypy-fixture file rosters | 2.3 |  | 0.599 |
| walker |  | 697 | 421 | python imports in src/typeguard/__init__.py |  |  | 0.618 |
| walker |  | 735 | 38 | python decl names surface in src/typeguard/__init__.py |  |  | 0.619 |
| walker |  | 735 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.619 |
| walker |  | 762 | 27 | listing of '.github' |  |  | 0.642 |
| walker |  | 770 | 8 | listing of '.github/workflows' |  |  | 0.659 |
| ns | 841 |  | 198 | typeguard/__init__.py: import re-exports (checkers/config/decorators) | 3.1 |  | 0.689 |
| ns | 1135 |  | 294 | typeguard/__init__.py: import re-exports (exceptions/functions/importhook/memo/suppression/utils) + rebinding loop | 3.2 | 3.1 | 0.676 |
| walker |  | 1245 | 475 | YAML config at .github/workflows/test.yml |  |  | 0.686 |
| ns | 1299 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.634 |
| ns | 1355 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.620 |
| ns | 1563 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.572 |
| ns | 1661 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.555 |
| walker |  | 1712 | 467 | README headline in README.rst |  |  | 0.593 |
| walker |  | 1725 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.593 |
| walker |  | 1725 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.593 |
| walker |  | 1736 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.593 |
| walker |  | 1758 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.593 |
| walker |  | 1781 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.593 |
| walker |  | 1850 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.618 |
| walker |  | 1886 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.618 |
| walker |  | 1886 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.618 |
| walker |  | 1886 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.618 |
| ns | 1896 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.650 |
| walker |  | 1986 | 100 | listing of 'tests' |  |  | 0.722 |
| walker |  | 2000 | 14 | listing of 'tests/mypy' |  |  | 0.739 |
| walker |  | 2021 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.739 |
| ns | 2201 |  | 305 | _checkers.py: every check_* function's signature | 5.1 |  | 0.682 |
| walker |  | 2276 | 255 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.733 |
| walker |  | 2325 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.734 |
| walker |  | 2325 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.734 |
| walker |  | 2325 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.734 |
| walker |  | 2325 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.734 |
| walker |  | 2325 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.734 |
| walker |  | 2346 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.734 |
| walker |  | 2368 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.734 |
| walker |  | 2400 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.734 |
| walker |  | 2442 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.735 |
| walker |  | 2523 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.736 |
| walker |  | 2523 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.736 |
| walker |  | 2523 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.736 |
| walker |  | 2523 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.736 |
| walker |  | 2523 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.736 |
| walker |  | 2523 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.736 |
| walker |  | 2532 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.736 |
| walker |  | 2704 | 172 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.770 |
| walker |  | 2704 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.770 |
| walker |  | 2704 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.770 |
| walker |  | 2704 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.770 |
| walker |  | 2728 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.770 |
| ns | 2744 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.692 |
| walker |  | 2755 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.692 |
| walker |  | 2792 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.692 |
| walker |  | 2830 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.692 |
| walker |  | 2861 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.692 |
| walker |  | 2911 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.692 |
| walker |  | 2961 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.692 |
| walker |  | 3011 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.692 |
| walker |  | 3061 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.692 |
| ns | 3090 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.651 |
| walker |  | 3111 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.651 |
| walker |  | 3161 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.651 |
| walker |  | 3211 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.652 |
| walker |  | 3261 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.652 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.630 |
| walker |  | 3311 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.630 |
| walker |  | 3361 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.630 |
| walker |  | 3411 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.630 |
| walker |  | 3461 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.630 |
| walker |  | 3511 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.630 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.607 |
| walker |  | 3561 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.607 |
| walker |  | 3611 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.607 |
| walker |  | 3661 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.607 |
| walker |  | 3711 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.607 |
| walker |  | 3761 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.607 |
| walker |  | 3811 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.607 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.577 |
| walker |  | 3861 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.577 |
| walker |  | 3911 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.577 |
| walker |  | 3974 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.577 |
| walker |  | 4043 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.577 |
| walker |  | 4052 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.577 |
| walker |  | 4119 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.577 |
| walker |  | 4127 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.577 |
| walker |  | 4137 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.577 |
| walker |  | 4171 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.577 |
| walker |  | 4185 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.577 |
| walker |  | 4205 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.577 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.544 |
| walker |  | 4273 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.545 |
| walker |  | 4273 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.545 |
| walker |  | 4273 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.545 |
| walker |  | 4282 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.545 |
| walker |  | 4303 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.547 |
| walker |  | 4303 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.547 |
| walker |  | 4323 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.549 |
| walker |  | 4351 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.552 |
| walker |  | 4373 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.553 |
| walker |  | 4457 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.560 |
| walker |  | 4528 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.561 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.557 |
| walker |  | 4700 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.557 |
| walker |  | 4700 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.557 |
| walker |  | 4700 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.557 |
| walker |  | 4700 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.557 |
| walker |  | 4700 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.557 |
| walker |  | 4708 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.557 |
| walker |  | 4730 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.557 |
| walker |  | 4747 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.557 |
| walker |  | 4771 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.557 |
| walker |  | 4797 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.557 |
| walker |  | 4823 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.558 |
| walker |  | 4849 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.558 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.549 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.533 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.518 |
| walker |  | 5848 | 999 | python method sigs in src/typeguard/_transformer.py |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.519 |
| walker |  | 5848 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.519 |
| walker |  | 5859 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.519 |
| walker |  | 5870 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.519 |
| walker |  | 5877 | 7 | python method at src/typeguard/_transformer.py:577 |  |  | 0.519 |
| walker |  | 5877 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.519 |
| walker |  | 5886 | 9 | python method at src/typeguard/_transformer.py:574 |  |  | 0.519 |
| walker |  | 5886 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.519 |
| walker |  | 5899 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.519 |
| walker |  | 5928 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.519 |
| walker |  | 5960 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.519 |
| walker |  | 5977 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.519 |
| walker |  | 6013 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.519 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.490 |
| walker |  | 6036 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.490 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.487 |
| walker |  | 6148 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.487 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.481 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.476 |
| walker |  | 6503 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.478 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.475 |
| walker |  | 6546 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.475 |
| walker |  | 6592 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.475 |
| walker |  | 6601 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.477 |
| walker |  | 6731 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.486 |
| walker |  | 6731 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.486 |
| walker |  | 6777 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.486 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.474 |
| walker |  | 6824 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.475 |
| walker |  | 6872 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.475 |
| walker |  | 6920 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.475 |
| walker |  | 6969 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.475 |
| walker |  | 6988 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.475 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.470 |
| walker |  | 7079 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.470 |
| walker |  | 7088 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.470 |
| walker |  | 7178 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.470 |
| walker |  | 7185 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.470 |
| walker |  | 7251 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.470 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.460 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.462 |
| walker |  | 7376 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.468 |
| walker |  | 7398 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.471 |
| walker |  | 7487 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.474 |
| walker |  | 7487 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.474 |
| walker |  | 7487 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.474 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.470 |
| walker |  | 7497 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.470 |
| walker |  | 7531 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.470 |
| walker |  | 7551 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.470 |
| walker |  | 7572 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.473 |
| walker |  | 7669 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.474 |
| walker |  | 7688 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.474 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.486 |
| walker |  | 7792 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.491 |
| walker |  | 7814 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.494 |
| walker |  | 7931 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.498 |
| walker |  | 7931 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.498 |
| walker |  | 7931 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.498 |
| walker |  | 7931 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.498 |
| walker |  | 7931 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.498 |
| walker |  | 7949 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.498 |
| walker |  | 7998 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.499 |
| walker |  | 8029 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.499 |
| walker |  | 8062 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.499 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.488 |
| walker |  | 8213 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.496 |
| walker |  | 8213 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.496 |
| walker |  | 8213 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.496 |
| walker |  | 8213 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.496 |
| walker |  | 8213 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.496 |
| walker |  | 8213 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.496 |
| walker |  | 8225 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.496 |
| walker |  | 8254 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.496 |
| walker |  | 8259 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.496 |
| walker |  | 8311 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.496 |
| walker |  | 8367 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.496 |
| walker |  | 8443 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.496 |
| walker |  | 8552 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.496 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.510 |
| walker |  | 8665 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.510 |
| walker |  | 8710 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.516 |
| walker |  | 8766 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.516 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.507 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.502 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.495 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.492 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.490 |
| walker |  | 9620 | 854 | manifest config in pyproject.toml |  |  | 0.551 |
| walker |  | 9751 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.554 |
| walker |  | 9751 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.554 |
| walker |  | 9751 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.554 |
| walker |  | 9751 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.554 |
| walker |  | 9751 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.554 |
| walker |  | 9751 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.554 |
| walker |  | 9751 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.554 |
| walker |  | 9759 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.554 |
| walker |  | 9769 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.554 |
| walker |  | 9783 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.554 |
| walker |  | 9783 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.554 |
| walker |  | 9790 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.554 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.549 |
| walker |  | 9863 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.549 |
| walker |  | 9944 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.549 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.544 |
