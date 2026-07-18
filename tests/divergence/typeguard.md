Score(3000)=0.750 I=0.899 C=0.625 ns_rows≤3K=16/45 (reached=11 partial=2 missing=3)

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
| walker |  | 2909 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.749 |
| walker |  | 2909 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.749 |
| walker |  | 2909 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.749 |
| walker |  | 2909 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.749 |
| walker |  | 2909 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.749 |
| walker |  | 2930 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.749 |
| walker |  | 2952 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.749 |
| walker |  | 2984 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.750 |
| walker |  | 3026 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.750 |
| ns | 3090 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.706 |
| walker |  | 3107 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.707 |
| walker |  | 3107 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.707 |
| walker |  | 3107 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.707 |
| walker |  | 3107 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.707 |
| walker |  | 3107 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.707 |
| walker |  | 3107 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.707 |
| walker |  | 3116 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.707 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.683 |
| walker |  | 3371 | 255 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.723 |
| walker |  | 3380 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.723 |
| walker |  | 3447 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.723 |
| walker |  | 3455 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.723 |
| walker |  | 3465 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.723 |
| walker |  | 3499 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.723 |
| walker |  | 3513 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.723 |
| walker |  | 3533 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.723 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.694 |
| walker |  | 3601 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.694 |
| walker |  | 3601 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.694 |
| walker |  | 3601 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.694 |
| walker |  | 3610 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.694 |
| walker |  | 3631 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.694 |
| walker |  | 3631 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.694 |
| walker |  | 3651 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.694 |
| walker |  | 3679 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.695 |
| walker |  | 3701 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.695 |
| walker |  | 3785 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.695 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.669 |
| walker |  | 3856 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.669 |
| walker |  | 4028 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.670 |
| walker |  | 4028 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.670 |
| walker |  | 4028 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.670 |
| walker |  | 4028 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.670 |
| walker |  | 4028 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.670 |
| walker |  | 4036 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.670 |
| walker |  | 4058 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.670 |
| walker |  | 4075 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.670 |
| walker |  | 4099 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.670 |
| walker |  | 4125 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.670 |
| walker |  | 4151 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.670 |
| walker |  | 4177 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.670 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.639 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.630 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.619 |
| walker |  | 5176 | 999 | python method sigs in src/typeguard/_transformer.py |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.620 |
| walker |  | 5176 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.620 |
| walker |  | 5187 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.620 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.601 |
| walker |  | 5198 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.601 |
| walker |  | 5205 | 7 | python method at src/typeguard/_transformer.py:577 |  |  | 0.601 |
| walker |  | 5205 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.601 |
| walker |  | 5214 | 9 | python method at src/typeguard/_transformer.py:574 |  |  | 0.601 |
| walker |  | 5214 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.601 |
| walker |  | 5227 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.601 |
| walker |  | 5256 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.601 |
| walker |  | 5288 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.601 |
| walker |  | 5305 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.601 |
| walker |  | 5341 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.601 |
| walker |  | 5364 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.601 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.584 |
| walker |  | 5476 | 112 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.584 |
| walker |  | 5831 | 355 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.586 |
| walker |  | 5874 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.586 |
| walker |  | 5920 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.586 |
| walker |  | 5929 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.588 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.556 |
| walker |  | 6059 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.556 |
| walker |  | 6059 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.556 |
| walker |  | 6105 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.556 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.553 |
| walker |  | 6152 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.553 |
| walker |  | 6200 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.553 |
| walker |  | 6248 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.553 |
| walker |  | 6297 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.553 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.545 |
| walker |  | 6316 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.545 |
| walker |  | 6407 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.545 |
| walker |  | 6416 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.545 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.540 |
| walker |  | 6506 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.540 |
| walker |  | 6513 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.540 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.545 |
| walker |  | 6579 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.545 |
| walker |  | 6704 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.552 |
| walker |  | 6726 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.554 |
| walker |  | 6815 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.558 |
| walker |  | 6815 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.558 |
| walker |  | 6815 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.558 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.546 |
| walker |  | 6825 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.546 |
| walker |  | 6859 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.546 |
| walker |  | 6879 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.546 |
| walker |  | 6900 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.548 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.543 |
| walker |  | 6997 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.545 |
| walker |  | 7016 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.545 |
| walker |  | 7120 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.550 |
| walker |  | 7142 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.553 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.541 |
| walker |  | 7259 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.545 |
| walker |  | 7259 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.545 |
| walker |  | 7259 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.545 |
| walker |  | 7259 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.545 |
| walker |  | 7259 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.545 |
| walker |  | 7277 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.545 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.546 |
| walker |  | 7326 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.548 |
| walker |  | 7357 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.548 |
| walker |  | 7390 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.548 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.543 |
| walker |  | 7541 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.552 |
| walker |  | 7541 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.552 |
| walker |  | 7541 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.552 |
| walker |  | 7541 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.552 |
| walker |  | 7541 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.552 |
| walker |  | 7541 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.552 |
| walker |  | 7553 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.552 |
| walker |  | 7582 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.552 |
| walker |  | 7587 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.552 |
| walker |  | 7639 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.552 |
| walker |  | 7695 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.552 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.561 |
| walker |  | 7771 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.561 |
| walker |  | 7880 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.561 |
| walker |  | 7993 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.561 |
| walker |  | 8038 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.568 |
| walker |  | 8094 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.568 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.555 |
| walker |  | 8225 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.558 |
| walker |  | 8225 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.558 |
| walker |  | 8225 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.558 |
| walker |  | 8225 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.558 |
| walker |  | 8225 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.558 |
| walker |  | 8225 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.558 |
| walker |  | 8225 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.558 |
| walker |  | 8233 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.559 |
| walker |  | 8243 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.559 |
| walker |  | 8257 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.559 |
| walker |  | 8257 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.559 |
| walker |  | 8264 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.559 |
| walker |  | 8337 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.559 |
| walker |  | 8418 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.559 |
| walker |  | 8478 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.559 |
| walker |  | 8539 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.559 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.571 |
| walker |  | 8582 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.571 |
| walker |  | 8654 | 72 | python method doc at src/typeguard/_transformer.py:650 |  |  | 0.571 |
| walker |  | 8787 | 133 | python decl at src/typeguard/_transformer.py:70 |  |  | 0.587 |
| walker |  | 8863 | 76 | python method doc at src/typeguard/_importhook.py:138 |  |  | 0.587 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.579 |
| walker |  | 8926 | 63 | python imports in src/typeguard/_config.py |  |  | 0.586 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.579 |
| walker |  | 9101 | 175 | python decl doc at src/typeguard/_config.py:30 |  |  | 0.579 |
| walker |  | 9257 | 156 | python decl at src/typeguard/_transformer.py:100 |  |  | 0.600 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.593 |
| walker |  | 9424 | 167 | python decl doc at src/typeguard/_importhook.py:183 |  |  | 0.593 |
| walker |  | 9438 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.601 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.597 |
| walker |  | 9519 | 81 | python imports in src/typeguard/_suppression.py |  |  | 0.600 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.598 |
| walker |  | 9754 | 235 | python decl doc at src/typeguard/_suppression.py:30 |  |  | 0.598 |
| walker |  | 9795 | 41 | python method body at src/typeguard/_memo.py:37 body 45 |  |  | 0.605 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.599 |
| walker |  | 9911 | 116 | python imports in src/typeguard/_pytest_plugin.py |  |  | 0.599 |
| walker |  | 9918 | 7 | python method body at src/typeguard/_importhook.py:167 body 173 |  |  | 0.599 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.593 |
| walker |  | 9974 | 56 | python decl body at src/typeguard/_utils.py:154 body 155 |  |  | 0.593 |
