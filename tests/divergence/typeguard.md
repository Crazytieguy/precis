Score(3000)=0.759 I=0.910 C=0.632 ns_rows≤3K=16/45 (reached=11 partial=2 missing=3)

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
| walker |  | 695 | 28 | listing of '.github' |  |  | 0.575 |
| walker |  | 702 | 7 | listing of '.github/workflows' |  |  | 0.593 |
| ns | 844 |  | 198 | typeguard/__init__.py: import re-exports (checkers/config/decorators) | 3.1 |  | 0.630 |
| ns | 1138 |  | 294 | typeguard/__init__.py: import re-exports (exceptions/functions/importhook/memo/suppression/utils) + rebinding loop | 3.2 | 3.1 | 0.623 |
| walker |  | 1177 | 475 | YAML config at .github/workflows/test.yml |  |  | 0.633 |
| walker |  | 1247 | 70 | [package] in pyproject.toml |  |  | 0.686 |
| walker |  | 1282 | 35 | package metadata in pyproject.toml |  |  | 0.701 |
| ns | 1302 |  | 164 | typeguard/__init__.py: config attr + plugin autoload | 3.3 | 3.2 | 0.649 |
| walker |  | 1330 | 48 | tool.mypy config in pyproject.toml |  |  | 0.650 |
| ns | 1358 |  | 56 | pytest config (pyproject.toml) | 4.1 |  | 0.635 |
| ns | 1566 |  | 208 | ruff lint config (pyproject.toml) | 4.2 |  | 0.586 |
| ns | 1664 |  | 98 | mypy + tox config (pyproject.toml) | 4.3 |  | 0.577 |
| walker |  | 1797 | 467 | README headline in README.rst |  |  | 0.615 |
| ns | 1899 |  | 235 | CI test workflow (.github/workflows/test.yml) | 4.4 |  | 0.648 |
| walker |  | 1944 | 147 | dev/build/target dependencies in pyproject.toml |  |  | 0.648 |
| walker |  | 2132 | 188 | manifest config in pyproject.toml |  |  | 0.696 |
| ns | 2204 |  | 305 | _checkers.py: every check_* function's signature | 5.1 |  | 0.642 |
| walker |  | 2321 | 189 | tool.tox config in pyproject.toml |  |  | 0.663 |
| walker |  | 2525 | 204 | tool.setuptools+setuptools_scm+pytest+coverage config in pyproject.toml |  |  | 0.680 |
| walker |  | 2729 | 204 | tool.ruff config in pyproject.toml |  |  | 0.747 |
| ns | 2747 |  | 543 | _checkers.py: origin_type_checkers dispatch table | 5.2 | 5.1 | 0.672 |
| walker |  | 2829 | 100 | listing of 'tests' |  |  | 0.729 |
| walker |  | 2842 | 13 | listing of 'tests/mypy' |  |  | 0.743 |
| walker |  | 2911 | 69 | python decl body at src/typeguard/__init__.py:37 body 38 |  |  | 0.758 |
| walker |  | 2932 | 21 | python imports in src/typeguard/_exceptions.py |  |  | 0.759 |
| walker |  | 2945 | 13 | python decl names surface in src/typeguard/_memo.py |  |  | 0.759 |
| walker |  | 2945 | 0 | python decl at src/typeguard/_memo.py:8 |  |  | 0.759 |
| walker |  | 2968 | 23 | python decl doc at src/typeguard/_memo.py:8 |  |  | 0.759 |
| walker |  | 2979 | 11 | python method sigs in src/typeguard/_memo.py |  |  | 0.759 |
| walker |  | 3050 | 71 | python method at src/typeguard/_memo.py:37 |  |  | 0.759 |
| ns | 3093 |  | 346 | _checkers.py: check_type_internal's dispatch loop | 5.3 | 5.1 | 0.714 |
| walker |  | 3099 | 49 | python decl names surface in src/typeguard/_exceptions.py |  |  | 0.714 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:5 |  |  | 0.714 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:12 |  |  | 0.714 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:19 |  |  | 0.714 |
| walker |  | 3099 | 0 | python decl at src/typeguard/_exceptions.py:26 |  |  | 0.714 |
| walker |  | 3120 | 21 | python decl doc at src/typeguard/_exceptions.py:19 |  |  | 0.714 |
| walker |  | 3142 | 22 | python decl doc at src/typeguard/_exceptions.py:12 |  |  | 0.715 |
| walker |  | 3174 | 32 | python decl doc at src/typeguard/_exceptions.py:26 |  |  | 0.715 |
| walker |  | 3216 | 42 | python decl doc at src/typeguard/_exceptions.py:5 |  |  | 0.715 |
| walker |  | 3297 | 81 | python method sigs in src/typeguard/_exceptions.py |  |  | 0.716 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:15 |  |  | 0.716 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:22 |  |  | 0.716 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:31 |  |  | 0.716 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:35 |  |  | 0.716 |
| walker |  | 3297 | 0 | python method at src/typeguard/_exceptions.py:38 |  |  | 0.716 |
| ns | 3303 |  | 210 | _checkers.py: check_typed_dict's extra_items handling | 5.4 | 5.1 | 0.692 |
| walker |  | 3306 | 9 | python method body at src/typeguard/_exceptions.py:15 body 16 |  |  | 0.693 |
| walker |  | 3315 | 9 | python method body at src/typeguard/_exceptions.py:22 body 23 |  |  | 0.693 |
| walker |  | 3382 | 67 | python decl names surface in src/typeguard/_suppression.py |  |  | 0.693 |
| walker |  | 3406 | 24 | python decl at src/typeguard/_suppression.py:26 |  |  | 0.693 |
| walker |  | 3406 | 0 | python decl body at src/typeguard/_suppression.py:26 body 27 |  |  | 0.693 |
| walker |  | 3434 | 28 | python decl at src/typeguard/_suppression.py:22 |  |  | 0.693 |
| walker |  | 3434 | 0 | python decl body at src/typeguard/_suppression.py:22 body 23 |  |  | 0.693 |
| walker |  | 3468 | 34 | python decl at src/typeguard/_suppression.py:30 |  |  | 0.693 |
| walker |  | 3536 | 68 | python decl names surface in src/typeguard/_config.py |  |  | 0.693 |
| walker |  | 3536 | 0 | python decl at src/typeguard/_config.py:14 |  |  | 0.693 |
| walker |  | 3536 | 0 | python decl at src/typeguard/_config.py:30 |  |  | 0.693 |
| ns | 3539 |  | 236 | _checkers.py: check_union (exemplar checker body) | 5.5 | 5.1 | 0.665 |
| walker |  | 3545 | 9 | python decl at src/typeguard/_config.py:62 |  |  | 0.665 |
| walker |  | 3567 | 22 | python decl doc at src/typeguard/_config.py:62 |  |  | 0.665 |
| walker |  | 3595 | 28 | python class body at src/typeguard/_config.py:14 |  |  | 0.665 |
| walker |  | 3617 | 22 | python class body at src/typeguard/_config.py:30 |  |  | 0.666 |
| walker |  | 3701 | 84 | python class body at src/typeguard/_config.py:62 |  |  | 0.666 |
| walker |  | 3720 | 19 | python method sigs in src/typeguard/_config.py |  |  | 0.666 |
| walker |  | 3720 | 0 | python method at src/typeguard/_config.py:52 |  |  | 0.666 |
| walker |  | 3742 | 22 | python class body at src/typeguard/_memo.py:8 |  |  | 0.667 |
| ns | 3819 |  | 280 | _config.py: policy enums (members + iterate_samples) | 6.1 |  | 0.642 |
| walker |  | 3914 | 172 | python decl names surface in src/typeguard/_transformer.py |  |  | 0.642 |
| walker |  | 3914 | 0 | python decl at src/typeguard/_transformer.py:285 |  |  | 0.642 |
| walker |  | 3914 | 0 | python decl at src/typeguard/_transformer.py:313 |  |  | 0.642 |
| walker |  | 3914 | 0 | python decl at src/typeguard/_transformer.py:338 |  |  | 0.642 |
| walker |  | 3914 | 0 | python decl at src/typeguard/_transformer.py:488 |  |  | 0.642 |
| walker |  | 3922 | 8 | python decl at src/typeguard/_transformer.py:117 |  |  | 0.642 |
| walker |  | 3944 | 22 | python decl at src/typeguard/_transformer.py:84 |  |  | 0.642 |
| walker |  | 3961 | 17 | python decl doc at src/typeguard/_transformer.py:313 |  |  | 0.642 |
| walker |  | 3985 | 24 | python decl at src/typeguard/_transformer.py:88 |  |  | 0.642 |
| walker |  | 4011 | 26 | python decl at src/typeguard/_transformer.py:92 |  |  | 0.642 |
| walker |  | 4037 | 26 | python decl at src/typeguard/_transformer.py:96 |  |  | 0.642 |
| walker |  | 4063 | 26 | python class body at src/typeguard/_transformer.py:313 |  |  | 0.642 |
| walker |  | 4177 | 114 | python class body at src/typeguard/_transformer.py:338 |  |  | 0.642 |
| ns | 4213 |  | 394 | _config.py: TypeCheckConfiguration (incl. docstring defaults) | 6.2 | 6.1 | 0.613 |
| walker |  | 4534 | 357 | python class body at src/typeguard/_transformer.py:117 |  |  | 0.615 |
| walker |  | 4543 | 9 | python method body at src/typeguard/_exceptions.py:35 body 36 |  |  | 0.615 |
| ns | 4649 |  | 436 | _exceptions.py (full) | 7.1 |  | 0.610 |
| walker |  | 4673 | 130 | python decl names surface in src/typeguard/_functions.py |  |  | 0.611 |
| walker |  | 4673 | 0 | python decl at src/typeguard/_functions.py:291 |  |  | 0.611 |
| walker |  | 4719 | 46 | python decl at src/typeguard/_functions.py:118 |  |  | 0.611 |
| walker |  | 4766 | 47 | python decl at src/typeguard/_functions.py:149 |  |  | 0.611 |
| walker |  | 4814 | 48 | python decl at src/typeguard/_functions.py:185 |  |  | 0.611 |
| walker |  | 4862 | 48 | python decl at src/typeguard/_functions.py:216 |  |  | 0.611 |
| ns | 4872 |  | 223 | _memo.py: TypeCheckMemo (slots + init) | 7.2 |  | 0.601 |
| walker |  | 4911 | 49 | python decl at src/typeguard/_functions.py:245 |  |  | 0.601 |
| walker |  | 4930 | 19 | python decl body at src/typeguard/_functions.py:291 body 299 |  |  | 0.601 |
| walker |  | 5029 | 99 | python decl at src/typeguard/_functions.py:39 |  |  | 0.601 |
| walker |  | 5029 | 0 | python decl body at src/typeguard/_functions.py:39 body 47 |  |  | 0.601 |
| walker |  | 5127 | 98 | python decl at src/typeguard/_functions.py:28 |  |  | 0.601 |
| walker |  | 5127 | 0 | python decl body at src/typeguard/_functions.py:28 body 36 |  |  | 0.601 |
| walker |  | 5193 | 66 | python decl doc at src/typeguard/_functions.py:291 |  |  | 0.601 |
| ns | 5200 |  | 328 | _functions.py: check_type() signature + how-it-differs | 8.1 |  | 0.583 |
| walker |  | 5318 | 125 | python decl at src/typeguard/_functions.py:50 |  |  | 0.591 |
| walker |  | 5340 | 22 | python decl doc at src/typeguard/_functions.py:50 |  |  | 0.594 |
| walker |  | 5457 | 117 | python decl names surface in src/typeguard/_importhook.py |  |  | 0.594 |
| walker |  | 5457 | 0 | python decl at src/typeguard/_importhook.py:51 |  |  | 0.594 |
| walker |  | 5457 | 0 | python decl at src/typeguard/_importhook.py:55 |  |  | 0.594 |
| walker |  | 5457 | 0 | python decl at src/typeguard/_importhook.py:109 |  |  | 0.594 |
| walker |  | 5457 | 0 | python decl at src/typeguard/_importhook.py:156 |  |  | 0.594 |
| ns | 5471 |  | 271 | _functions.py: check_type() body | 8.2 | 8.1 | 0.578 |
| walker |  | 5475 | 18 | python decl body at src/typeguard/_importhook.py:51 body 52 |  |  | 0.578 |
| walker |  | 5524 | 49 | python decl at src/typeguard/_importhook.py:183 |  |  | 0.578 |
| walker |  | 5555 | 31 | python decl doc at src/typeguard/_importhook.py:156 |  |  | 0.578 |
| walker |  | 5588 | 33 | python decl at src/typeguard/_importhook.py:45 |  |  | 0.578 |
| walker |  | 5739 | 151 | python method sigs in src/typeguard/_importhook.py |  |  | 0.578 |
| walker |  | 5739 | 0 | python method at src/typeguard/_importhook.py:120 |  |  | 0.578 |
| walker |  | 5739 | 0 | python method at src/typeguard/_importhook.py:138 |  |  | 0.578 |
| walker |  | 5739 | 0 | python method at src/typeguard/_importhook.py:161 |  |  | 0.578 |
| walker |  | 5739 | 0 | python method at src/typeguard/_importhook.py:164 |  |  | 0.578 |
| walker |  | 5739 | 0 | python method at src/typeguard/_importhook.py:175 |  |  | 0.578 |
| walker |  | 5751 | 12 | python method doc at src/typeguard/_importhook.py:175 |  |  | 0.578 |
| walker |  | 5780 | 29 | python method at src/typeguard/_importhook.py:99 |  |  | 0.578 |
| walker |  | 5785 | 5 | python method body at src/typeguard/_importhook.py:164 body 165 |  |  | 0.578 |
| walker |  | 5837 | 52 | python method at src/typeguard/_importhook.py:167 |  |  | 0.578 |
| walker |  | 5893 | 56 | python method at src/typeguard/_importhook.py:124 |  |  | 0.578 |
| walker |  | 5969 | 76 | python method at src/typeguard/_importhook.py:56 |  |  | 0.578 |
| ns | 6034 |  | 563 | _suppression.py: suppress_type_checks (signature + body) | 9.1 |  | 0.546 |
| walker |  | 6078 | 109 | python decl doc at src/typeguard/_importhook.py:109 |  |  | 0.546 |
| ns | 6129 |  | 95 | _decorators.py: top-level function/class name roster | 10.1 |  | 0.543 |
| walker |  | 6191 | 113 | python decl doc at src/typeguard/_config.py:14 |  |  | 0.543 |
| walker |  | 6236 | 45 | python imports in src/typeguard/_memo.py |  |  | 0.552 |
| ns | 6311 |  | 182 | _decorators.py: typechecked() signature + summary | 10.2 | 10.1 | 0.544 |
| walker |  | 6367 | 131 | python decl names surface in src/typeguard/_utils.py |  |  | 0.545 |
| walker |  | 6367 | 0 | python decl at src/typeguard/_utils.py:66 |  |  | 0.545 |
| walker |  | 6367 | 0 | python decl at src/typeguard/_utils.py:104 |  |  | 0.545 |
| walker |  | 6367 | 0 | python decl at src/typeguard/_utils.py:127 |  |  | 0.545 |
| walker |  | 6367 | 0 | python decl at src/typeguard/_utils.py:142 |  |  | 0.545 |
| walker |  | 6367 | 0 | python decl at src/typeguard/_utils.py:154 |  |  | 0.545 |
| walker |  | 6367 | 0 | python decl at src/typeguard/_utils.py:162 |  |  | 0.545 |
| walker |  | 6375 | 8 | python decl at src/typeguard/_utils.py:172 |  |  | 0.545 |
| walker |  | 6385 | 10 | python class body at src/typeguard/_utils.py:172 |  |  | 0.545 |
| walker |  | 6399 | 14 | python method sigs in src/typeguard/_utils.py |  |  | 0.545 |
| walker |  | 6399 | 0 | python method at src/typeguard/_utils.py:176 |  |  | 0.545 |
| walker |  | 6406 | 7 | python method body at src/typeguard/_utils.py:176 body 177 |  |  | 0.545 |
| ns | 6446 |  | 135 | _decorators.py: instrument() — locate the target in the re-parsed AST | 10.3 | 10.1 | 0.540 |
| walker |  | 6479 | 73 | python decl doc at src/typeguard/_utils.py:127 |  |  | 0.540 |
| ns | 6525 |  | 79 | _functions.py: remaining check_*_type roster | 11.1 |  | 0.544 |
| walker |  | 6560 | 81 | python decl doc at src/typeguard/_utils.py:104 |  |  | 0.544 |
| walker |  | 6596 | 36 | python decl names surface in src/typeguard/_pytest_plugin.py |  |  | 0.545 |
| walker |  | 6596 | 0 | python decl at src/typeguard/_pytest_plugin.py:16 |  |  | 0.545 |
| walker |  | 6596 | 0 | python decl at src/typeguard/_pytest_plugin.py:75 |  |  | 0.545 |
| walker |  | 6609 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.555 |
| walker |  | 6698 | 89 | python decl names surface in src/typeguard/_decorators.py |  |  | 0.558 |
| walker |  | 6698 | 0 | python decl at src/typeguard/_decorators.py:32 |  |  | 0.558 |
| walker |  | 6698 | 0 | python decl at src/typeguard/_decorators.py:56 |  |  | 0.558 |
| walker |  | 6729 | 31 | python decl at src/typeguard/_decorators.py:146 |  |  | 0.561 |
| walker |  | 6729 | 0 | python decl body at src/typeguard/_decorators.py:146 body 147 |  |  | 0.561 |
| walker |  | 6763 | 34 | python decl at src/typeguard/_decorators.py:36 |  |  | 0.561 |
| walker |  | 6783 | 20 | python decl body at src/typeguard/_decorators.py:32 body 33 |  |  | 0.561 |
| ns | 6822 |  | 297 | _functions.py: check_return_type() — the NotImplemented exemption | 11.2 | 11.1 | 0.549 |
| walker |  | 6887 | 104 | python decl at src/typeguard/_decorators.py:150 |  |  | 0.555 |
| walker |  | 6909 | 22 | python decl doc at src/typeguard/_decorators.py:150 |  |  | 0.558 |
| ns | 6993 |  | 171 | _importhook.py: class/function roster | 12.1 |  | 0.565 |
| walker |  | 7025 | 116 | python decl at src/typeguard/_decorators.py:136 |  |  | 0.567 |
| walker |  | 7025 | 0 | python decl body at src/typeguard/_decorators.py:136 body 143 |  |  | 0.567 |
| ns | 7256 |  | 263 | _importhook.py: install_import_hook() body | 12.2 | 12.1 | 0.556 |
| ns | 7292 |  | 36 | _pytest_plugin.py: function roster | 13.1 |  | 0.557 |
| ns | 7496 |  | 204 | _utils.py: function/class roster | 14.1 |  | 0.556 |
| ns | 7761 |  | 265 | _transformer.py: class/method roster | 15.1 |  | 0.551 |
| walker |  | 7975 | 950 | python method sigs in src/typeguard/_transformer.py |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:141 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:171 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:183 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:206 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:212 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:225 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:238 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:275 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:286 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:289 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:293 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:297 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:302 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:306 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:309 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:319 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:322 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:325 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:328 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:334 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:347 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:374 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:401 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:407 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:466 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:476 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:498 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:570 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:580 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:596 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:600 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:608 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:615 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:624 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:918 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:945 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:994 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1036 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1138 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1181 |  |  | 0.567 |
| walker |  | 7975 | 0 | python method at src/typeguard/_transformer.py:1224 |  |  | 0.567 |
| walker |  | 7986 | 11 | python method at src/typeguard/_transformer.py:352 |  |  | 0.567 |
| walker |  | 7997 | 11 | python method at src/typeguard/_transformer.py:472 |  |  | 0.567 |
| walker |  | 8010 | 13 | python method doc at src/typeguard/_transformer.py:225 |  |  | 0.567 |
| walker |  | 8039 | 29 | python method at src/typeguard/_transformer.py:913 |  |  | 0.567 |
| walker |  | 8071 | 32 | python method at src/typeguard/_transformer.py:650 |  |  | 0.567 |
| walker |  | 8088 | 17 | python method doc at src/typeguard/_transformer.py:918 |  |  | 0.567 |
| walker |  | 8124 | 36 | python method at src/typeguard/_transformer.py:489 |  |  | 0.567 |
| walker |  | 8147 | 23 | python method doc at src/typeguard/_transformer.py:1138 |  |  | 0.567 |
| walker |  | 8175 | 28 | python method at src/typeguard/_transformer.py:577 |  |  | 0.567 |
| walker |  | 8175 | 0 | python method body at src/typeguard/_transformer.py:577 body 578 |  |  | 0.567 |
| walker |  | 8203 | 28 | python method at src/typeguard/_transformer.py:574 |  |  | 0.567 |
| walker |  | 8203 | 0 | python method body at src/typeguard/_transformer.py:574 body 575 |  |  | 0.567 |
| ns | 8215 |  | 454 | _transformer.py: recognized-annotation name tables | 15.2 |  | 0.553 |
| walker |  | 8246 | 43 | python method doc at src/typeguard/_transformer.py:1181 |  |  | 0.553 |
| walker |  | 8292 | 46 | python method doc at src/typeguard/_transformer.py:994 |  |  | 0.553 |
| walker |  | 8348 | 56 | python method doc at src/typeguard/_transformer.py:945 |  |  | 0.553 |
| walker |  | 8408 | 60 | python method doc at src/typeguard/_transformer.py:1224 |  |  | 0.553 |
| walker |  | 8469 | 61 | python method doc at src/typeguard/_transformer.py:1036 |  |  | 0.553 |
| walker |  | 8512 | 43 | python method at src/typeguard/_transformer.py:516 |  |  | 0.553 |
| ns | 8583 |  | 368 | _transformer.py: TransformMemo fields | 15.3 | 15.1 | 0.565 |
| ns | 8882 |  | 299 | _transformer.py: visit_FunctionDef (target selection + overload handling) | 15.4 | 15.1 | 0.555 |
| walker |  | 8939 | 427 | python decl names surface in src/typeguard/_checkers.py |  |  | 0.587 |
| walker |  | 8939 | 0 | python decl at src/typeguard/_checkers.py:586 |  |  | 0.587 |
| walker |  | 8939 | 0 | python decl at src/typeguard/_checkers.py:679 |  |  | 0.587 |
| walker |  | 8939 | 0 | python decl at src/typeguard/_checkers.py:1099 |  |  | 0.587 |
| walker |  | 8963 | 24 | python decl at src/typeguard/_checkers.py:81 |  |  | 0.587 |
| walker |  | 8990 | 27 | python decl at src/typeguard/_checkers.py:84 |  |  | 0.587 |
| walker |  | 9027 | 37 | python decl at src/typeguard/_checkers.py:924 |  |  | 0.587 |
| ns | 9034 |  | 152 | features.rst: what is checked | 16.1 |  | 0.581 |
| walker |  | 9065 | 38 | python decl at src/typeguard/_checkers.py:1061 |  |  | 0.581 |
| walker |  | 9096 | 31 | python decl at src/typeguard/_checkers.py:89 |  |  | 0.581 |
| walker |  | 9146 | 50 | python decl at src/typeguard/_checkers.py:153 |  |  | 0.581 |
| walker |  | 9196 | 50 | python decl at src/typeguard/_checkers.py:212 |  |  | 0.581 |
| walker |  | 9246 | 50 | python decl at src/typeguard/_checkers.py:247 |  |  | 0.581 |
| walker |  | 9296 | 50 | python decl at src/typeguard/_checkers.py:305 |  |  | 0.581 |
| ns | 9311 |  | 277 | userguide.rst: forward reference handling notes | 16.2 |  | 0.573 |
| walker |  | 9346 | 50 | python decl at src/typeguard/_checkers.py:324 |  |  | 0.573 |
| walker |  | 9396 | 50 | python decl at src/typeguard/_checkers.py:343 |  |  | 0.573 |
| walker |  | 9446 | 50 | python decl at src/typeguard/_checkers.py:423 |  |  | 0.574 |
| ns | 9461 |  | 150 | userguide.rst: debugging instrumented code | 16.3 |  | 0.570 |
| walker |  | 9496 | 50 | python decl at src/typeguard/_checkers.py:447 |  |  | 0.570 |
| walker |  | 9546 | 50 | python decl at src/typeguard/_checkers.py:474 |  |  | 0.570 |
| ns | 9566 |  | 105 | versionhistory.rst: latest UNRELEASED entry | 17.1 |  | 0.568 |
| walker |  | 9596 | 50 | python decl at src/typeguard/_checkers.py:536 |  |  | 0.568 |
| walker |  | 9646 | 50 | python decl at src/typeguard/_checkers.py:545 |  |  | 0.568 |
| walker |  | 9696 | 50 | python decl at src/typeguard/_checkers.py:590 |  |  | 0.568 |
| walker |  | 9746 | 50 | python decl at src/typeguard/_checkers.py:623 |  |  | 0.568 |
| walker |  | 9796 | 50 | python decl at src/typeguard/_checkers.py:632 |  |  | 0.568 |
| ns | 9818 |  | 252 | tests/dummymodule.py: function/class roster | 18.1 |  | 0.563 |
| walker |  | 9846 | 50 | python decl at src/typeguard/_checkers.py:641 |  |  | 0.563 |
| walker |  | 9896 | 50 | python decl at src/typeguard/_checkers.py:651 |  |  | 0.563 |
| walker |  | 9946 | 50 | python decl at src/typeguard/_checkers.py:663 |  |  | 0.563 |
| ns | 9974 |  | 156 | tests/dummymodule_py312.py: type-alias + generic-syntax patterns | 19.1 |  | 0.557 |
| walker |  | 9996 | 50 | python decl at src/typeguard/_checkers.py:834 |  |  | 0.557 |
