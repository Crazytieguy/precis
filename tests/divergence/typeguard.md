Score(3000)=0.750 I=0.899 C=0.625 ns_rows≤3K=16/45 (reached=11 partial=2 missing=3)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 34 | 34 | listing of '.' |  |  | 1.000 |
| ns | 34 |  | 34 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 38 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 79 | 41 | listing of 'docs' |  |  | 1.000 |
| ns | 130 |  | 96 | README lede opening | 1.2 |  | 0.811 |
| walker |  | 151 | 72 | listing of 'src/typeguard' |  |  | 0.845 |
| ns | 241 |  | 111 | Package identity: name, Python support, deps | 1.3 |  | 0.633 |
| ns | 367 |  | 126 | Project URLs + pytest entry point | 1.4 |  | 0.525 |
| ns | 480 |  | 113 | Source + docs file rosters | 2.1 |  | 0.672 |
| walker |  | 572 | 421 | python imports in src/typeguard/__init__.py |  |  | 0.696 |
| ns | 580 |  | 100 | Tests directory roster | 2.2 |  | 0.591 |
| walker |  | 610 | 38 | python decl names surface in src/typeguard/__init__.py |  |  | 0.592 |
| walker |  | 610 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.592 |
| ns | 643 |  | 63 | CI, community-health, and mypy-fixture file rosters | 2.3 |  | 0.529 |
| walker |  | 665 | 55 | [dependencies] in pyproject.toml |  |  | 0.551 |
| walker |  | 713 | 48 | tool.mypy config in pyproject.toml |  |  | 0.552 |
| walker |  | 740 | 27 | listing of '.github' |  |  | 0.576 |
| walker |  | 748 | 8 | listing of '.github/workflows' |  |  | 0.594 |
| ns | 841 |  | 198 | typeguard/__init__.py: import re-exports (checkers/config/decorators) | 3.1 |  | 0.631 |
| ns | 1135 |  | 294 | typeguard/__init__.py: import re-exports (exceptions/functions/importhook/memo/suppression/utils) + rebinding loop | 3.2 | 3.1 | 0.624 |
| walker |  | 1223 | 475 | YAML config at .github/workflows/test.yml |  |  | 0.634 |
| walker |  | 1293 | 70 | [package] in pyproject.toml |  |  | 0.687 |
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
| walker |  | 2720 | 100 | listing of 'tests' |  |  | 0.800 |
| walker |  | 2734 | 14 | listing of 'tests/mypy' |  |  | 0.815 |
| ns | 2744 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.733 |
| walker |  | 2803 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.749 |
| walker |  | 2839 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.749 |
| walker |  | 2839 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.749 |
| walker |  | 2839 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.749 |
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
| walker |  | 3125 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.707 |
| walker |  | 3192 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.707 |
| walker |  | 3200 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.707 |
| walker |  | 3210 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.707 |
| walker |  | 3244 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.707 |
| walker |  | 3258 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.707 |
| walker |  | 3278 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.707 |
| ns | 3300 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.684 |
| walker |  | 3346 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.684 |
| walker |  | 3346 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.684 |
| walker |  | 3346 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.684 |
| walker |  | 3355 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.684 |
| walker |  | 3376 | 21 | python method sigs in src/typeguard/_config.py |  |  | 0.684 |
| walker |  | 3376 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.684 |
| walker |  | 3396 | 20 | python class body at src/typeguard/_config.py:30 |  |  | 0.684 |
| walker |  | 3424 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.685 |
| walker |  | 3446 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.685 |
| walker |  | 3530 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.685 |
| ns | 3536 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.658 |
| walker |  | 3601 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.658 |
| walker |  | 3773 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.658 |
| walker |  | 3773 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.658 |
| walker |  | 3773 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.658 |
| walker |  | 3773 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.658 |
| walker |  | 3773 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.658 |
| walker |  | 3781 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.658 |
| walker |  | 3803 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.658 |
| ns | 3816 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.634 |
| walker |  | 3820 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.634 |
| walker |  | 3844 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.634 |
| walker |  | 3870 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.634 |
| walker |  | 3896 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.634 |
| walker |  | 3922 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.634 |
| walker |  | 4036 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.634 |
| ns | 4210 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.606 |
| walker |  | 4393 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.607 |
| walker |  | 4402 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.607 |
| walker |  | 4532 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.608 |
| walker |  | 4532 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.608 |
| walker |  | 4578 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.608 |
| walker |  | 4625 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.608 |
| ns | 4646 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.604 |
| walker |  | 4673 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.604 |
| walker |  | 4721 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.604 |
| walker |  | 4770 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.604 |
| walker |  | 4789 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.604 |
| ns | 4869 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.594 |
| walker |  | 4880 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.594 |
| walker |  | 4889 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.594 |
| walker |  | 4979 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.594 |
| walker |  | 4986 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.594 |
| walker |  | 5052 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.594 |
| walker |  | 5177 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.595 |
| ns | 5197 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.584 |
| walker |  | 5199 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.587 |
| walker |  | 5288 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.587 |
| walker |  | 5288 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.587 |
| walker |  | 5288 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.587 |
| walker |  | 5298 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.587 |
| walker |  | 5332 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.587 |
| walker |  | 5352 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.587 |
| walker |  | 5373 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.588 |
| ns | 5468 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.571 |
| walker |  | 5470 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.571 |
| walker |  | 5489 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.571 |
| walker |  | 5593 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.572 |
| walker |  | 5615 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.572 |
| walker |  | 5732 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.572 |
| walker |  | 5732 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.572 |
| walker |  | 5732 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.572 |
| walker |  | 5732 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.572 |
| walker |  | 5732 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.572 |
| walker |  | 5750 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.572 |
| walker |  | 5799 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.572 |
| walker |  | 5830 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.572 |
| walker |  | 5863 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.572 |
| walker |  | 6014 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.573 |
| walker |  | 6014 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.573 |
| walker |  | 6014 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.573 |
| walker |  | 6014 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.573 |
| walker |  | 6014 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.573 |
| walker |  | 6014 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.573 |
| walker |  | 6026 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.573 |
| ns | 6031 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.541 |
| walker |  | 6055 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.541 |
| walker |  | 6060 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.541 |
| walker |  | 6112 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.541 |
| ns | 6126 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.546 |
| walker |  | 6168 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.546 |
| walker |  | 6244 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.546 |
| ns | 6308 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.547 |
| walker |  | 6353 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.547 |
| ns | 6443 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.542 |
| walker |  | 6466 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.542 |
| walker |  | 6511 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.551 |
| ns | 6522 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.555 |
| walker |  | 6642 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.556 |
| walker |  | 6642 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.556 |
| walker |  | 6642 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.556 |
| walker |  | 6642 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.556 |
| walker |  | 6642 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.556 |
| walker |  | 6642 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.556 |
| walker |  | 6642 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.556 |
| walker |  | 6650 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.556 |
| walker |  | 6660 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.556 |
| walker |  | 6674 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.556 |
| walker |  | 6674 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.556 |
| walker |  | 6681 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.556 |
| walker |  | 6754 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.556 |
| ns | 6819 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.544 |
| walker |  | 6835 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.544 |
| walker |  | 6849 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.553 |
| ns | 6990 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.561 |
| ns | 7253 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.550 |
| walker |  | 7276 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.586 |
| walker |  | 7276 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.586 |
| walker |  | 7276 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.586 |
| walker |  | 7276 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.586 |
| ns | 7289 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.588 |
| walker |  | 7300 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.588 |
| walker |  | 7327 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.588 |
| walker |  | 7364 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.588 |
| walker |  | 7402 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.588 |
| walker |  | 7433 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.588 |
| walker |  | 7483 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.588 |
| ns | 7493 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.587 |
| walker |  | 7533 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.587 |
| walker |  | 7583 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.587 |
| walker |  | 7633 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.587 |
| walker |  | 7683 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.587 |
| walker |  | 7733 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.587 |
| ns | 7758 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.581 |
| walker |  | 7783 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.582 |
| walker |  | 7833 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.582 |
| walker |  | 7883 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.582 |
| walker |  | 7933 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.582 |
| walker |  | 7983 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.582 |
| walker |  | 8033 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.582 |
| walker |  | 8083 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.582 |
| walker |  | 8133 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.582 |
| walker |  | 8183 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.582 |
| ns | 8212 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.567 |
| walker |  | 8233 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.567 |
| walker |  | 8283 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.567 |
| walker |  | 8333 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.567 |
| walker |  | 8383 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.567 |
| walker |  | 8433 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.567 |
| walker |  | 8483 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.567 |
| walker |  | 8546 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.567 |
| ns | 8580 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.579 |
| walker |  | 8615 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.579 |
| ns | 8879 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.568 |
| ns | 9031 |  | 152 | features.rst: what is checked | 16.1 |  | 0.562 |
| ns | 9308 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.555 |
| ns | 9458 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.551 |
| ns | 9563 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.549 |
| walker |  | 9610 | 995 | python method sigs in src/typeguard/_transformer.py |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.562 |
| walker |  | 9610 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.562 |
| walker |  | 9621 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.562 |
| walker |  | 9632 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.562 |
| walker |  | 9639 | 7 | python method at src/typeguard/_transformer.py:577 |  |  | 0.562 |
| walker |  | 9639 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.562 |
| walker |  | 9648 | 9 | python method at src/typeguard/_transformer.py:574 |  |  | 0.562 |
| walker |  | 9648 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.562 |
| walker |  | 9661 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.562 |
| walker |  | 9690 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.562 |
| walker |  | 9722 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.562 |
| walker |  | 9739 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.562 |
| walker |  | 9775 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.562 |
| walker |  | 9798 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.562 |
| ns | 9815 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.557 |
| walker |  | 9841 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.557 |
| walker |  | 9887 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.557 |
| walker |  | 9943 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.557 |
| ns | 9971 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.551 |
