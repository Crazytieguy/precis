Score(3000)=0.750 I=0.900 C=0.625 ns_rows≤3K=16/45 (reached=11 partial=2 missing=3)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 1.000 |
| ns | 38 |  | 38 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 42 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 82 | 40 | listing of 'docs' |  |  | 1.000 |
| ns | 134 |  | 96 | README lede opening | 1.2 |  | 0.811 |
| walker |  | 153 | 71 | listing of 'src/typeguard' |  |  | 0.845 |
| ns | 245 |  | 111 | Package identity: name, Python support, deps | 1.3 |  | 0.633 |
| ns | 371 |  | 126 | Project URLs + pytest entry point | 1.4 |  | 0.525 |
| ns | 483 |  | 112 | Source + docs file rosters | 2.1 |  | 0.672 |
| walker |  | 574 | 421 | python imports in src/typeguard/__init__.py |  |  | 0.696 |
| ns | 583 |  | 100 | Tests directory roster | 2.2 |  | 0.591 |
| walker |  | 612 | 38 | python decl names surface in src/typeguard/__init__.py |  |  | 0.592 |
| walker |  | 612 | 0 | python decl at src/typeguard/__init__.py:37 |  |  | 0.592 |
| ns | 646 |  | 63 | CI, community-health, and mypy-fixture file rosters | 2.3 |  | 0.529 |
| walker |  | 667 | 55 | [dependencies] in pyproject.toml |  |  | 0.551 |
| walker |  | 715 | 48 | tool.mypy config in pyproject.toml |  |  | 0.552 |
| walker |  | 743 | 28 | listing of '.github' |  |  | 0.576 |
| walker |  | 750 | 7 | listing of '.github/workflows' |  |  | 0.594 |
| ns | 844 |  | 198 | typeguard/__init__.py: import re-exports (checkers/config/decorators) | 3.1 |  | 0.631 |
| ns | 1138 |  | 294 | typeguard/__init__.py: import re-exports (exceptions/functions/importhook/memo/suppression/utils) + rebinding loop | 3.2 | 3.1 | 0.624 |
| walker |  | 1225 | 475 | YAML config at .github/workflows/test.yml |  |  | 0.634 |
| walker |  | 1295 | 70 | [package] in pyproject.toml |  |  | 0.687 |
| ns | 1302 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.635 |
| ns | 1358 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.621 |
| ns | 1566 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.573 |
| ns | 1664 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.565 |
| walker |  | 1762 | 467 | README headline in README.rst |  |  | 0.602 |
| ns | 1899 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.637 |
| walker |  | 1951 | 189 | tool.tox config in pyproject.toml |  |  | 0.659 |
| walker |  | 2143 | 192 | manifest config in pyproject.toml |  |  | 0.706 |
| ns | 2204 |  | 305 | _checkers.py: every check_* function's signature | 5.1 |  | 0.652 |
| walker |  | 2349 | 206 | tool.ruff config in pyproject.toml |  |  | 0.720 |
| walker |  | 2553 | 204 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.736 |
| walker |  | 2653 | 100 | listing of 'tests' |  |  | 0.799 |
| walker |  | 2666 | 13 | listing of 'tests/mypy' |  |  | 0.815 |
| walker |  | 2735 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.832 |
| ns | 2747 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.749 |
| walker |  | 2756 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.749 |
| walker |  | 2769 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.749 |
| walker |  | 2769 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.749 |
| walker |  | 2792 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.749 |
| walker |  | 2803 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.749 |
| walker |  | 2874 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.749 |
| walker |  | 2923 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.749 |
| walker |  | 2923 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.749 |
| walker |  | 2923 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.749 |
| walker |  | 2923 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.749 |
| walker |  | 2923 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.749 |
| walker |  | 2944 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.750 |
| walker |  | 2966 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.750 |
| walker |  | 2998 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.750 |
| walker |  | 3040 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.751 |
| ns | 3093 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.706 |
| walker |  | 3121 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.707 |
| walker |  | 3121 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.707 |
| walker |  | 3121 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.707 |
| walker |  | 3121 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.707 |
| walker |  | 3121 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.707 |
| walker |  | 3121 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.707 |
| walker |  | 3130 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.707 |
| walker |  | 3139 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.708 |
| walker |  | 3206 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.708 |
| walker |  | 3214 | 8 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.708 |
| walker |  | 3224 | 10 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.708 |
| walker |  | 3258 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.708 |
| walker |  | 3272 | 14 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.708 |
| walker |  | 3292 | 20 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.708 |
| ns | 3303 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.684 |
| walker |  | 3360 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.684 |
| walker |  | 3360 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.684 |
| walker |  | 3360 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.684 |
| walker |  | 3369 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.684 |
| walker |  | 3391 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.684 |
| walker |  | 3419 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.684 |
| walker |  | 3441 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.685 |
| walker |  | 3525 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.685 |
| ns | 3539 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.658 |
| walker |  | 3544 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.658 |
| walker |  | 3544 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.658 |
| walker |  | 3566 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.658 |
| walker |  | 3738 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.658 |
| walker |  | 3738 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.658 |
| walker |  | 3738 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.658 |
| walker |  | 3738 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.658 |
| walker |  | 3738 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.658 |
| walker |  | 3746 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.658 |
| walker |  | 3768 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.658 |
| walker |  | 3785 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.658 |
| walker |  | 3809 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.658 |
| ns | 3819 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.634 |
| walker |  | 3835 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.634 |
| walker |  | 3861 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.634 |
| walker |  | 3887 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.634 |
| walker |  | 4001 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.634 |
| ns | 4213 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.605 |
| walker |  | 4358 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.607 |
| walker |  | 4367 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.607 |
| walker |  | 4497 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.608 |
| walker |  | 4497 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.608 |
| walker |  | 4543 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.608 |
| walker |  | 4590 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.608 |
| walker |  | 4638 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.608 |
| ns | 4649 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.604 |
| walker |  | 4686 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.604 |
| walker |  | 4735 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.604 |
| walker |  | 4754 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.604 |
| walker |  | 4845 | 91 | python decl at src/typeguard/_functions.py:28 |  |  | 0.604 |
| walker |  | 4854 | 9 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.604 |
| ns | 4872 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.594 |
| walker |  | 4944 | 90 | python decl at src/typeguard/_functions.py:39 |  |  | 0.594 |
| walker |  | 4951 | 7 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.594 |
| walker |  | 5017 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.594 |
| walker |  | 5142 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.595 |
| walker |  | 5164 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.595 |
| ns | 5200 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.587 |
| walker |  | 5281 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.587 |
| walker |  | 5281 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.587 |
| walker |  | 5281 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.587 |
| walker |  | 5281 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.587 |
| walker |  | 5281 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.587 |
| walker |  | 5299 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.587 |
| walker |  | 5348 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.587 |
| walker |  | 5379 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.587 |
| walker |  | 5412 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.587 |
| ns | 5471 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.571 |
| walker |  | 5563 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.572 |
| walker |  | 5563 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.572 |
| walker |  | 5563 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.572 |
| walker |  | 5563 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.572 |
| walker |  | 5563 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.572 |
| walker |  | 5563 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.572 |
| walker |  | 5575 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.572 |
| walker |  | 5604 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.572 |
| walker |  | 5609 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.572 |
| walker |  | 5661 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.572 |
| walker |  | 5717 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.572 |
| walker |  | 5793 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.572 |
| walker |  | 5902 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.572 |
| walker |  | 6015 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.572 |
| ns | 6034 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.540 |
| walker |  | 6060 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.549 |
| ns | 6129 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.546 |
| walker |  | 6191 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.546 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.546 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.546 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.546 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.546 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.546 |
| walker |  | 6191 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.546 |
| walker |  | 6199 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.546 |
| walker |  | 6209 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.546 |
| walker |  | 6223 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.546 |
| walker |  | 6223 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.546 |
| walker |  | 6230 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.546 |
| walker |  | 6303 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.546 |
| ns | 6311 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.538 |
| walker |  | 6384 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.538 |
| walker |  | 6420 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.539 |
| walker |  | 6420 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.539 |
| walker |  | 6420 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.539 |
| walker |  | 6433 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.549 |
| ns | 6446 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.544 |
| walker |  | 6522 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.548 |
| walker |  | 6522 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.548 |
| walker |  | 6522 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.548 |
| ns | 6525 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.552 |
| walker |  | 6532 | 10 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.552 |
| walker |  | 6566 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.552 |
| walker |  | 6586 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.552 |
| walker |  | 6607 | 21 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.555 |
| walker |  | 6704 | 97 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.557 |
| walker |  | 6723 | 19 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.557 |
| ns | 6822 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.545 |
| walker |  | 6827 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.550 |
| walker |  | 6849 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.553 |
| ns | 6993 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.561 |
| ns | 7256 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.550 |
| walker |  | 7276 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.586 |
| walker |  | 7276 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.586 |
| walker |  | 7276 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.586 |
| walker |  | 7276 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.586 |
| ns | 7292 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.588 |
| walker |  | 7300 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.588 |
| walker |  | 7327 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.588 |
| walker |  | 7364 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.588 |
| walker |  | 7402 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.588 |
| walker |  | 7433 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.588 |
| walker |  | 7483 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.588 |
| ns | 7496 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.587 |
| walker |  | 7533 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.587 |
| walker |  | 7583 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.587 |
| walker |  | 7633 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.587 |
| walker |  | 7683 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.587 |
| walker |  | 7733 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.587 |
| ns | 7761 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.581 |
| walker |  | 7783 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.582 |
| walker |  | 7833 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.582 |
| walker |  | 7883 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.582 |
| walker |  | 7933 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.582 |
| walker |  | 7983 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.582 |
| walker |  | 8033 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.582 |
| walker |  | 8083 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.582 |
| walker |  | 8133 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.582 |
| walker |  | 8183 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.582 |
| ns | 8215 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.567 |
| walker |  | 8233 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.567 |
| walker |  | 8283 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.567 |
| walker |  | 8333 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.567 |
| walker |  | 8383 | 50 | python decl at src/typeguard/_checkers.py:885 |  |  | 0.567 |
| walker |  | 8433 | 50 | python decl at src/typeguard/_checkers.py:895 |  |  | 0.567 |
| walker |  | 8483 | 50 | python decl at src/typeguard/_checkers.py:915 |  |  | 0.567 |
| walker |  | 8546 | 63 | python decl at src/typeguard/_checkers.py:365 |  |  | 0.567 |
| ns | 8583 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.579 |
| walker |  | 8615 | 69 | python decl at src/typeguard/_checkers.py:555 |  |  | 0.579 |
| ns | 8882 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.568 |
| ns | 9034 |  | 152 | features.rst: what is checked | 16.1 |  | 0.562 |
| ns | 9311 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.555 |
| ns | 9461 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.551 |
| ns | 9566 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.549 |
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
| ns | 9818 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.557 |
| walker |  | 9841 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.557 |
| walker |  | 9887 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.557 |
| walker |  | 9943 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.557 |
| ns | 9974 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.551 |
