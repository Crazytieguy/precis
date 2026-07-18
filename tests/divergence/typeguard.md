Score(3000)=0.749 I=0.897 C=0.625 ns_rows≤3K=16/45 (reached=11 partial=2 missing=3)

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
| walker |  | 783 | 48 | tool.mypy config in pyproject.toml |  |  | 0.620 |
| walker |  | 810 | 27 | listing of '.github' |  |  | 0.643 |
| walker |  | 818 | 8 | listing of '.github/workflows' |  |  | 0.660 |
| ns | 841 |  | 198 | typeguard/__init__.py: import re-exports (checkers/config/decorators) | 3.1 |  | 0.690 |
| ns | 1135 |  | 294 | typeguard/__init__.py: import re-exports (exceptions/functions/importhook/memo/suppression/utils) + rebinding loop | 3.2 | 3.1 | 0.677 |
| walker |  | 1293 | 475 | YAML config at .github/workflows/test.yml |  |  | 0.687 |
| ns | 1299 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.635 |
| ns | 1355 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.621 |
| ns | 1563 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.573 |
| ns | 1661 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.565 |
| walker |  | 1760 | 467 | README headline in README.rst |  |  | 0.602 |
| walker |  | 1773 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.602 |
| walker |  | 1773 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.602 |
| walker |  | 1784 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.602 |
| walker |  | 1806 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.602 |
| walker |  | 1829 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.602 |
| ns | 1896 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.637 |
| walker |  | 2018 | 189 | tool.tox config in pyproject.toml |  |  | 0.659 |
| ns | 2201 |  | 305 | _checkers.py: every check_* function's signature | 5.1 |  | 0.608 |
| walker |  | 2210 | 192 | manifest config in pyproject.toml |  |  | 0.652 |
| walker |  | 2416 | 206 | tool.ruff config in pyproject.toml |  |  | 0.720 |
| walker |  | 2620 | 204 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.736 |
| walker |  | 2689 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.755 |
| walker |  | 2725 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.755 |
| walker |  | 2725 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.755 |
| walker |  | 2725 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.755 |
| ns | 2744 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.679 |
| walker |  | 2825 | 100 | listing of 'tests' |  |  | 0.735 |
| walker |  | 2839 | 14 | listing of 'tests/mypy' |  |  | 0.749 |
| walker |  | 2860 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.749 |
| ns | 3090 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.705 |
| walker |  | 3115 | 255 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.745 |
| walker |  | 3164 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.745 |
| walker |  | 3164 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.745 |
| walker |  | 3164 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.745 |
| walker |  | 3164 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.745 |
| walker |  | 3164 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.745 |
| walker |  | 3185 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.746 |
| walker |  | 3207 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.746 |
| walker |  | 3239 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.746 |
| walker |  | 3281 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.747 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.722 |
| walker |  | 3362 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.723 |
| walker |  | 3362 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.723 |
| walker |  | 3362 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.723 |
| walker |  | 3362 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.723 |
| walker |  | 3362 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.723 |
| walker |  | 3362 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.723 |
| walker |  | 3371 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.723 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.694 |
| walker |  | 3543 | 172 | python decl names surface #1 in src/typeguard/_checkers.py |  |  | 0.718 |
| walker |  | 3543 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.718 |
| walker |  | 3543 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.718 |
| walker |  | 3543 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.718 |
| walker |  | 3567 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.718 |
| walker |  | 3594 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.718 |
| walker |  | 3631 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.718 |
| walker |  | 3669 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.718 |
| walker |  | 3700 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.718 |
| walker |  | 3750 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.718 |
| walker |  | 3800 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.718 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.683 |
| walker |  | 3850 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.683 |
| walker |  | 3900 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.683 |
| walker |  | 3950 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.683 |
| walker |  | 4000 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.683 |
| walker |  | 4050 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.685 |
| walker |  | 4100 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.685 |
| walker |  | 4150 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.685 |
| walker |  | 4200 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.685 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.646 |
| walker |  | 4250 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.646 |
| walker |  | 4300 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.646 |
| walker |  | 4350 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.646 |
| walker |  | 4400 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.646 |
| walker |  | 4450 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.646 |
| walker |  | 4500 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.646 |
| walker |  | 4550 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.646 |
| walker |  | 4600 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.646 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.633 |
| walker |  | 4650 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.633 |
| walker |  | 4700 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.633 |
| walker |  | 4750 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.633 |
| walker |  | 4813 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.633 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.616 |
| walker |  | 4882 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.616 |
| walker |  | 4891 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.618 |
| walker |  | 4958 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.618 |
| walker |  | 4966 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.618 |
| walker |  | 4976 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.618 |
| walker |  | 5010 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.618 |
| walker |  | 5024 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.618 |
| walker |  | 5044 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.618 |
| walker |  | 5112 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.619 |
| walker |  | 5112 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.619 |
| walker |  | 5112 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.619 |
| walker |  | 5121 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.619 |
| walker |  | 5142 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.620 |
| walker |  | 5142 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.620 |
| walker |  | 5162 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.622 |
| walker |  | 5190 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.625 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.606 |
| walker |  | 5212 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.607 |
| walker |  | 5296 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.612 |
| walker |  | 5367 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.620 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.603 |
| walker |  | 5539 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.603 |
| walker |  | 5539 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.603 |
| walker |  | 5539 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.603 |
| walker |  | 5539 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.603 |
| walker |  | 5539 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.603 |
| walker |  | 5547 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.603 |
| walker |  | 5569 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.603 |
| walker |  | 5586 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.603 |
| walker |  | 5610 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.603 |
| walker |  | 5636 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.603 |
| walker |  | 5662 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.603 |
| walker |  | 5688 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.603 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.570 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.566 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.558 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.553 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.550 |
| walker |  | 6687 | 999 | python method sigs in src/typeguard/_transformer.py |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.551 |
| walker |  | 6687 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.551 |
| walker |  | 6698 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.551 |
| walker |  | 6709 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.551 |
| walker |  | 6716 | 7 | python method at src/typeguard/_transformer.py:577 |  |  | 0.551 |
| walker |  | 6716 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.551 |
| walker |  | 6725 | 9 | python method at src/typeguard/_transformer.py:574 |  |  | 0.551 |
| walker |  | 6725 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.551 |
| walker |  | 6738 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.551 |
| walker |  | 6767 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.551 |
| walker |  | 6799 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.551 |
| walker |  | 6816 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.551 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.538 |
| walker |  | 6852 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.538 |
| walker |  | 6875 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.538 |
| walker |  | 6987 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.538 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.533 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.521 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.523 |
| walker |  | 7342 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.524 |
| walker |  | 7385 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.524 |
| walker |  | 7431 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.524 |
| walker |  | 7440 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.526 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.521 |
| walker |  | 7570 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.529 |
| walker |  | 7570 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.529 |
| walker |  | 7616 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.529 |
| walker |  | 7663 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.530 |
| walker |  | 7711 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.530 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.540 |
| walker |  | 7759 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.540 |
| walker |  | 7808 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.540 |
| walker |  | 7827 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.540 |
| walker |  | 7918 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.540 |
| walker |  | 7927 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.540 |
| walker |  | 8017 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.540 |
| walker |  | 8024 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.540 |
| walker |  | 8090 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.540 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.527 |
| walker |  | 8215 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.533 |
| walker |  | 8237 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.535 |
| walker |  | 8326 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.538 |
| walker |  | 8326 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.538 |
| walker |  | 8326 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.538 |
| walker |  | 8336 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.538 |
| walker |  | 8370 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.538 |
| walker |  | 8390 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.538 |
| walker |  | 8411 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.540 |
| walker |  | 8508 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.542 |
| walker |  | 8527 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.542 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.555 |
| walker |  | 8631 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.559 |
| walker |  | 8653 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.562 |
| walker |  | 8770 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.565 |
| walker |  | 8770 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.565 |
| walker |  | 8770 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.565 |
| walker |  | 8770 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.565 |
| walker |  | 8770 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.565 |
| walker |  | 8788 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.565 |
| walker |  | 8837 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.566 |
| walker |  | 8868 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.566 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.556 |
| walker |  | 8901 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.556 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.550 |
| walker |  | 9052 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.558 |
| walker |  | 9052 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.558 |
| walker |  | 9052 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.558 |
| walker |  | 9052 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.558 |
| walker |  | 9052 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.558 |
| walker |  | 9052 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.558 |
| walker |  | 9064 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.558 |
| walker |  | 9093 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.558 |
| walker |  | 9098 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.558 |
| walker |  | 9150 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.558 |
| walker |  | 9206 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.558 |
| walker |  | 9282 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.558 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.551 |
| walker |  | 9391 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.551 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.547 |
| walker |  | 9504 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.547 |
| walker |  | 9549 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.553 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.551 |
| walker |  | 9605 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.551 |
| walker |  | 9736 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.554 |
| walker |  | 9736 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.554 |
| walker |  | 9736 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.554 |
| walker |  | 9736 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.554 |
| walker |  | 9736 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.554 |
| walker |  | 9736 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.554 |
| walker |  | 9736 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.554 |
| walker |  | 9744 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.554 |
| walker |  | 9754 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.554 |
| walker |  | 9768 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.554 |
| walker |  | 9768 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.554 |
| walker |  | 9775 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.554 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.549 |
| walker |  | 9848 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.549 |
| walker |  | 9929 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.549 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.544 |
| walker |  | 9989 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.544 |
