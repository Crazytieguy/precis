Score(3000)=0.630 I=0.844 C=0.470 ns_rows≤3K=17/41 (reached=9 partial=0 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 79 | 79 | listing of '.' |  |  | 1.000 |
| ns | 79 |  | 79 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 82 | 3 | listing of 'src' |  |  | 1.000 |
| walker |  | 115 | 33 | listing of 'ext' |  |  | 1.000 |
| ns | 169 |  | 90 | src/requests/ package listing | 1.2 |  | 0.659 |
| walker |  | 174 | 59 | listing of 'docs' |  |  | 0.677 |
| walker |  | 177 | 3 | listing of 'docs/_templates' |  |  | 0.677 |
| walker |  | 185 | 8 | listing of 'docs/_static' |  |  | 0.677 |
| walker |  | 194 | 9 | listing of 'docs/dev' |  |  | 0.677 |
| walker |  | 207 | 13 | listing of 'docs/_themes' |  |  | 0.677 |
| walker |  | 227 | 20 | listing of 'docs/user' |  |  | 0.677 |
| ns | 247 |  | 78 | tests/ top-level listing | 1.3 |  | 0.560 |
| walker |  | 264 | 37 | listing of 'docs/community' |  |  | 0.560 |
| ns | 306 |  | 59 | docs/ directory listing | 1.4 |  | 0.628 |
| walker |  | 353 | 89 | listing of 'src/requests' |  |  | 0.856 |
| ns | 391 |  | 85 | .github/ and workflows/ listing | 1.5 |  | 0.750 |
| ns | 566 |  | 175 | README lede + usage example | 1.6 |  | 0.673 |
| ns | 766 |  | 200 | pyproject.toml identity | 1.7 |  | 0.618 |
| ns | 851 |  | 85 | pyproject.toml Python-version support + dependencies | 1.8 | 1.7 | 0.597 |
| ns | 1039 |  | 188 | Makefile build/test targets | 1.9 | 1.7 | 0.559 |
| ns | 1269 |  | 230 | __version__.py package metadata | 1.10 |  | 0.529 |
| ns | 1343 |  | 74 | api.py function locations | 2.1 |  | 0.513 |
| walker |  | 1355 | 1002 | python imports in src/requests/__init__.py |  |  | 0.522 |
| walker |  | 1389 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.522 |
| walker |  | 1405 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.522 |
| ns | 1573 |  | 230 | __init__.py public export tuple | 2.2 |  | 0.584 |
| walker |  | 1580 | 175 | README headline in README.md |  |  | 0.655 |
| walker |  | 1620 | 40 | headings outline in README.md |  |  | 0.655 |
| walker |  | 1667 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.655 |
| walker |  | 1725 | 58 | [package] in pyproject.toml |  |  | 0.659 |
| ns | 1840 |  | 267 | exceptions.py class hierarchy locations | 3.1 |  | 0.613 |
| walker |  | 1872 | 147 | README.md section #0 |  |  | 0.613 |
| walker |  | 1949 | 77 | manifest config in pyproject.toml |  |  | 0.620 |
| walker |  | 1969 | 20 | python imports in setup.py |  |  | 0.620 |
| walker |  | 2051 | 82 | tool.setuptools config in pyproject.toml |  |  | 0.620 |
| walker |  | 2095 | 44 | listing of '.github' |  |  | 0.639 |
| walker |  | 2135 | 40 | listing of '.github/workflows' |  |  | 0.685 |
| walker |  | 2148 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.685 |
| walker |  | 2287 | 139 | [dependencies] in pyproject.toml |  |  | 0.708 |
| ns | 2322 |  | 482 | hooks.py full (event hook dispatch) | 3.2 |  | 0.630 |
| walker |  | 2350 | 63 | README.md section #1 |  |  | 0.630 |
| ns | 2431 |  | 109 | structures.py class/method locations | 3.3 |  | 0.613 |
| walker |  | 2459 | 109 | tool.pytest+pyright config in pyproject.toml |  |  | 0.613 |
| walker |  | 2584 | 125 | package metadata in pyproject.toml |  |  | 0.648 |
| ns | 2900 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.597 |
| walker |  | 2940 | 356 | plaintext config Makefile |  |  | 0.636 |
| ns | 2952 |  | 52 | models.py class roster | 4.1 |  | 0.630 |
| walker |  | 3080 | 140 | dev/build/target dependencies in pyproject.toml |  |  | 0.630 |
| ns | 3101 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.616 |
| walker |  | 3158 | 78 | listing of 'tests' |  |  | 0.664 |
| walker |  | 3167 | 9 | listing of 'tests/testserver' |  |  | 0.664 |
| walker |  | 3183 | 16 | listing of 'tests/certs' |  |  | 0.664 |
| walker |  | 3222 | 39 | plaintext config docs/requirements.txt |  |  | 0.664 |
| ns | 3357 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.640 |
| ns | 3411 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.635 |
| walker |  | 3478 | 256 | README.md section #3 |  |  | 0.635 |
| ns | 3489 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.628 |
| ns | 3695 |  | 206 | Session method locations | 5.4 | 5.1 | 0.610 |
| walker |  | 3792 | 314 | tool.ruff config in pyproject.toml |  |  | 0.610 |
| walker |  | 3818 | 26 | python imports in src/requests/packages.py |  |  | 0.610 |
| walker |  | 3994 | 176 | README.md section #2 |  |  | 0.610 |
| walker |  | 4067 | 73 | declaration surface of requirements-dev.txt |  |  | 0.610 |
| walker |  | 4124 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.620 |
| walker |  | 4124 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.620 |
| walker |  | 4124 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.620 |
| walker |  | 4124 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.620 |
| walker |  | 4132 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.620 |
| walker |  | 4163 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.620 |
| walker |  | 4216 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.620 |
| walker |  | 4248 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.620 |
| ns | 4420 |  | 725 | tests/conftest.py full | 6.1 |  | 0.566 |
| walker |  | 4521 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.566 |
| walker |  | 4578 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.566 |
| ns | 4737 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.545 |
| ns | 4935 |  | 198 | test_structures.py locations | 6.3 |  | 0.534 |
| walker |  | 5052 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.577 |
| walker |  | 5052 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.577 |
| walker |  | 5078 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.577 |
| walker |  | 5104 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.577 |
| walker |  | 5120 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.577 |
| walker |  | 5138 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.577 |
| walker |  | 5158 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.577 |
| ns | 5164 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.565 |
| walker |  | 5201 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.565 |
| walker |  | 5244 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.565 |
| walker |  | 5289 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.565 |
| walker |  | 5339 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.565 |
| ns | 5373 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.557 |
| walker |  | 5397 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.557 |
| walker |  | 5431 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.557 |
| walker |  | 5498 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.557 |
| ns | 5525 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.549 |
| walker |  | 5537 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.549 |
| walker |  | 5573 | 36 | python decl names surface in src/requests/help.py |  |  | 0.549 |
| walker |  | 5573 | 0 | python decl at src/requests/help.py:37 |  |  | 0.549 |
| walker |  | 5573 | 0 | python decl at src/requests/help.py:69 |  |  | 0.549 |
| walker |  | 5573 | 0 | python decl at src/requests/help.py:128 |  |  | 0.549 |
| walker |  | 5586 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.549 |
| walker |  | 5600 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.549 |
| walker |  | 5618 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.549 |
| ns | 5620 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.545 |
| ns | 5636 |  | 16 | tests/certs/ directory listing | 6.8 |  | 0.548 |
| walker |  | 5659 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.548 |
| walker |  | 5731 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.548 |
| walker |  | 5820 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.548 |
| walker |  | 5820 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.548 |
| walker |  | 5820 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.548 |
| walker |  | 5831 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.548 |
| walker |  | 5846 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.548 |
| walker |  | 6000 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.548 |
| walker |  | 6285 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.549 |
| walker |  | 6285 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.549 |
| walker |  | 6298 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.549 |
| walker |  | 6329 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.549 |
| walker |  | 6363 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.549 |
| walker |  | 6402 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.549 |
| walker |  | 6442 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.549 |
| walker |  | 6490 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.549 |
| walker |  | 6549 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.549 |
| walker |  | 6621 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.549 |
| walker |  | 6666 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.549 |
| walker |  | 6742 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.549 |
| walker |  | 6899 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.549 |
| walker |  | 6899 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.549 |
| walker |  | 6911 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.549 |
| walker |  | 6992 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.549 |
| walker |  | 7034 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.549 |
| walker |  | 7087 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.549 |
| ns | 7134 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.491 |
| walker |  | 7197 | 110 | declaration surface of tox.ini |  |  | 0.491 |
| walker |  | 7203 | 6 | listing of 'tests/certs/valid' |  |  | 0.491 |
| walker |  | 7259 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.491 |
| walker |  | 7309 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.492 |
| walker |  | 7309 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.492 |
| ns | 7415 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.507 |
| walker |  | 7662 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.540 |
| walker |  | 7662 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.540 |
| walker |  | 7670 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.540 |
| walker |  | 7679 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.540 |
| walker |  | 7688 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.540 |
| walker |  | 7697 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.540 |
| walker |  | 7706 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.540 |
| walker |  | 7715 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.540 |
| walker |  | 7724 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.540 |
| walker |  | 7733 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.540 |
| walker |  | 7743 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.540 |
| walker |  | 7754 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.540 |
| walker |  | 7765 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.540 |
| walker |  | 7777 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.540 |
| walker |  | 7790 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.540 |
| walker |  | 7803 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.540 |
| walker |  | 7817 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.540 |
| walker |  | 7831 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.540 |
| walker |  | 7845 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.540 |
| walker |  | 7861 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.540 |
| walker |  | 7878 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.540 |
| walker |  | 7896 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.540 |
| walker |  | 7914 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.540 |
| walker |  | 7933 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.540 |
| walker |  | 7963 | 30 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.540 |
| walker |  | 8006 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.540 |
| walker |  | 8031 | 25 | python class body at src/requests/exceptions.py:20 |  |  | 0.540 |
| walker |  | 8096 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.540 |
| walker |  | 8166 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.540 |
| walker |  | 8166 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.540 |
| walker |  | 8166 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.540 |
| walker |  | 8166 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.540 |
| walker |  | 8185 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.540 |
| walker |  | 8331 | 146 | python decl names surface in src/requests/models.py |  |  | 0.547 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:109 |  |  | 0.547 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:255 |  |  | 0.547 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:283 |  |  | 0.547 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:376 |  |  | 0.547 |
| walker |  | 8331 | 0 | python decl at src/requests/models.py:730 |  |  | 0.547 |
| walker |  | 8349 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.547 |
| walker |  | 8388 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.547 |
| walker |  | 8399 | 11 | python class body at src/requests/models.py:109 |  |  | 0.547 |
| walker |  | 8658 | 259 | python class body at src/requests/models.py:730 |  |  | 0.547 |
| walker |  | 8781 | 123 | python class body at src/requests/models.py:283 |  |  | 0.547 |
| ns | 8789 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.501 |
| walker |  | 8880 | 99 | python class body at src/requests/models.py:376 |  |  | 0.501 |
| walker |  | 8954 | 74 | python decl at src/requests/models.py:96 |  |  | 0.501 |
| walker |  | 8971 | 17 | python class body at src/requests/models.py:255 |  |  | 0.501 |
| ns | 9124 |  | 335 | auth.py class/method locations | 8.1 |  | 0.495 |
| ns | 9336 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.491 |
| ns | 9364 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.490 |
| ns | 9441 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.488 |
| walker |  | 9672 | 701 | python method sigs in src/requests/models.py |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:271 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:355 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:358 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:451 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:454 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:465 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:563 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:652 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:720 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:763 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:810 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:813 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:824 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:832 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:835 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:845 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:855 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:1087 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:1140 |  |  | 0.525 |
| walker |  | 9672 | 0 | python method at src/requests/models.py:1169 |  |  | 0.525 |
| walker |  | 9678 | 6 | python method at src/requests/models.py:112 |  |  | 0.525 |
| walker |  | 9686 | 8 | python method at src/requests/models.py:859 |  |  | 0.525 |
| walker |  | 9694 | 8 | python method at src/requests/models.py:874 |  |  | 0.525 |
| walker |  | 9702 | 8 | python method at src/requests/models.py:881 |  |  | 0.525 |
| walker |  | 9710 | 8 | python method at src/requests/models.py:889 |  |  | 0.525 |
| walker |  | 9718 | 8 | python method at src/requests/models.py:894 |  |  | 0.525 |
| walker |  | 9727 | 9 | python method at src/requests/models.py:1030 |  |  | 0.525 |
| walker |  | 9736 | 9 | python method at src/requests/models.py:1049 |  |  | 0.525 |
| walker |  | 9745 | 9 | python method at src/requests/models.py:1122 |  |  | 0.525 |
| walker |  | 9759 | 14 | python method at src/requests/models.py:405 |  |  | 0.525 |
| walker |  | 9767 | 8 | python method at src/requests/models.py:471 |  |  | 0.525 |
| walker |  | 9779 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.525 |
| walker |  | 9792 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.525 |
| walker |  | 9805 | 13 | python method doc at src/requests/models.py:465 |  |  | 0.525 |
| ns | 9815 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.513 |
| walker |  | 9818 | 13 | python method doc at src/requests/models.py:563 |  |  | 0.513 |
| walker |  | 9847 | 29 | python method at src/requests/models.py:816 |  |  | 0.513 |
| walker |  | 9862 | 15 | python method doc at src/requests/models.py:1030 |  |  | 0.513 |
| ns | 9876 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.511 |
| walker |  | 9895 | 33 | python method at src/requests/models.py:697 |  |  | 0.511 |
| walker |  | 9911 | 16 | python method doc at src/requests/models.py:855 |  |  | 0.511 |
| walker |  | 9928 | 17 | python method doc at src/requests/models.py:652 |  |  | 0.511 |
| walker |  | 9964 | 36 | python method at src/requests/models.py:258 |  |  | 0.511 |
| walker |  | 9976 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.511 |
| ns | 9982 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.512 |
| ns | 10011 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.512 |
