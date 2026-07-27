Score(3000)=0.648 I=0.877 C=0.479 ns_rows≤3K=17/41 (reached=9 partial=0 missing=8)

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
| walker |  | 2815 | 356 | plaintext config Makefile |  |  | 0.655 |
| walker |  | 2893 | 78 | listing of 'tests' |  |  | 0.710 |
| ns | 2900 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.654 |
| walker |  | 2902 | 9 | listing of 'tests/testserver' |  |  | 0.654 |
| walker |  | 2918 | 16 | listing of 'tests/certs' |  |  | 0.654 |
| ns | 2952 |  | 52 | models.py class roster | 4.1 |  | 0.648 |
| walker |  | 2957 | 39 | plaintext config docs/requirements.txt |  |  | 0.648 |
| ns | 3101 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.634 |
| walker |  | 3213 | 256 | README.md section #3 |  |  | 0.634 |
| ns | 3357 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.611 |
| ns | 3411 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.606 |
| ns | 3489 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.599 |
| walker |  | 3527 | 314 | tool.ruff config in pyproject.toml |  |  | 0.599 |
| walker |  | 3553 | 26 | python imports in src/requests/packages.py |  |  | 0.599 |
| ns | 3695 |  | 206 | Session method locations | 5.4 | 5.1 | 0.582 |
| walker |  | 3729 | 176 | README.md section #2 |  |  | 0.582 |
| walker |  | 3802 | 73 | declaration surface of requirements-dev.txt |  |  | 0.582 |
| walker |  | 3859 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.592 |
| walker |  | 3859 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.592 |
| walker |  | 3859 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.592 |
| walker |  | 3859 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.592 |
| walker |  | 3867 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.592 |
| walker |  | 3898 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.592 |
| walker |  | 3951 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.592 |
| walker |  | 3983 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.592 |
| walker |  | 4256 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.592 |
| walker |  | 4313 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.592 |
| ns | 4420 |  | 725 | tests/conftest.py full | 6.1 |  | 0.540 |
| ns | 4737 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.520 |
| walker |  | 4787 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.565 |
| walker |  | 4787 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.565 |
| walker |  | 4813 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.565 |
| walker |  | 4839 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.565 |
| walker |  | 4855 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.565 |
| walker |  | 4873 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.565 |
| walker |  | 4893 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.565 |
| ns | 4935 |  | 198 | test_structures.py locations | 6.3 |  | 0.554 |
| walker |  | 4936 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.554 |
| walker |  | 4979 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.554 |
| walker |  | 5024 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.554 |
| walker |  | 5074 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.554 |
| walker |  | 5132 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.554 |
| ns | 5164 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.542 |
| walker |  | 5166 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.542 |
| walker |  | 5233 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.542 |
| walker |  | 5272 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.542 |
| walker |  | 5308 | 36 | python decl names surface in src/requests/help.py |  |  | 0.542 |
| walker |  | 5308 | 0 | python decl at src/requests/help.py:37 |  |  | 0.542 |
| walker |  | 5308 | 0 | python decl at src/requests/help.py:69 |  |  | 0.542 |
| walker |  | 5308 | 0 | python decl at src/requests/help.py:128 |  |  | 0.542 |
| walker |  | 5321 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.542 |
| walker |  | 5335 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.542 |
| walker |  | 5353 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.542 |
| ns | 5373 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.534 |
| walker |  | 5394 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.534 |
| walker |  | 5466 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.534 |
| ns | 5525 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.527 |
| walker |  | 5555 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.527 |
| walker |  | 5555 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.527 |
| walker |  | 5555 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.527 |
| walker |  | 5566 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.527 |
| walker |  | 5581 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.527 |
| ns | 5620 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.523 |
| ns | 5636 |  | 16 | tests/certs/ directory listing | 6.8 |  | 0.526 |
| walker |  | 5735 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.526 |
| walker |  | 6020 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.528 |
| walker |  | 6020 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.528 |
| walker |  | 6033 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.528 |
| walker |  | 6064 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.528 |
| walker |  | 6098 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.528 |
| walker |  | 6137 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.528 |
| walker |  | 6177 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.528 |
| walker |  | 6225 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.528 |
| walker |  | 6284 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.528 |
| walker |  | 6356 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.528 |
| walker |  | 6401 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.528 |
| walker |  | 6477 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.528 |
| walker |  | 6634 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.528 |
| walker |  | 6634 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.528 |
| walker |  | 6646 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.528 |
| walker |  | 6727 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.528 |
| walker |  | 6769 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.528 |
| walker |  | 6822 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.528 |
| walker |  | 6932 | 110 | declaration surface of tox.ini |  |  | 0.528 |
| walker |  | 6938 | 6 | listing of 'tests/certs/valid' |  |  | 0.528 |
| walker |  | 6994 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.528 |
| walker |  | 7044 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.528 |
| walker |  | 7044 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.528 |
| ns | 7134 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.472 |
| walker |  | 7397 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.508 |
| walker |  | 7397 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.508 |
| walker |  | 7405 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.508 |
| walker |  | 7414 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.508 |
| ns | 7415 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.522 |
| walker |  | 7423 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.522 |
| walker |  | 7432 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.522 |
| walker |  | 7441 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.522 |
| walker |  | 7450 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.522 |
| walker |  | 7459 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.522 |
| walker |  | 7468 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.522 |
| walker |  | 7478 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.522 |
| walker |  | 7489 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.522 |
| walker |  | 7500 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.522 |
| walker |  | 7512 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.522 |
| walker |  | 7525 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.522 |
| walker |  | 7538 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.522 |
| walker |  | 7552 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.522 |
| walker |  | 7566 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.522 |
| walker |  | 7580 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.522 |
| walker |  | 7596 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.522 |
| walker |  | 7613 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.522 |
| walker |  | 7631 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.522 |
| walker |  | 7649 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.522 |
| walker |  | 7668 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.522 |
| walker |  | 7698 | 30 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.522 |
| walker |  | 7741 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.522 |
| walker |  | 7766 | 25 | python class body at src/requests/exceptions.py:20 |  |  | 0.522 |
| walker |  | 7831 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.522 |
| walker |  | 7901 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.522 |
| walker |  | 7901 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.522 |
| walker |  | 7901 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.522 |
| walker |  | 7901 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.522 |
| walker |  | 7920 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.522 |
| walker |  | 8066 | 146 | python decl names surface in src/requests/models.py |  |  | 0.528 |
| walker |  | 8066 | 0 | python decl at src/requests/models.py:109 |  |  | 0.528 |
| walker |  | 8066 | 0 | python decl at src/requests/models.py:255 |  |  | 0.528 |
| walker |  | 8066 | 0 | python decl at src/requests/models.py:283 |  |  | 0.528 |
| walker |  | 8066 | 0 | python decl at src/requests/models.py:376 |  |  | 0.528 |
| walker |  | 8066 | 0 | python decl at src/requests/models.py:730 |  |  | 0.528 |
| walker |  | 8084 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.528 |
| walker |  | 8123 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.528 |
| walker |  | 8134 | 11 | python class body at src/requests/models.py:109 |  |  | 0.528 |
| walker |  | 8393 | 259 | python class body at src/requests/models.py:730 |  |  | 0.528 |
| walker |  | 8516 | 123 | python class body at src/requests/models.py:283 |  |  | 0.528 |
| walker |  | 8615 | 99 | python class body at src/requests/models.py:376 |  |  | 0.528 |
| walker |  | 8689 | 74 | python decl at src/requests/models.py:96 |  |  | 0.528 |
| walker |  | 8706 | 17 | python class body at src/requests/models.py:255 |  |  | 0.528 |
| ns | 8789 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.485 |
| ns | 9124 |  | 335 | auth.py class/method locations | 8.1 |  | 0.478 |
| ns | 9336 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.474 |
| ns | 9364 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.474 |
| ns | 9441 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.472 |
| walker |  | 9525 | 819 | python method sigs in src/requests/models.py |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:271 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:355 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:358 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:451 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:454 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:465 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:563 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:652 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:720 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:763 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:810 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:813 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:824 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:832 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:835 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:845 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:855 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:1087 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:1140 |  |  | 0.509 |
| walker |  | 9525 | 0 | python method at src/requests/models.py:1169 |  |  | 0.509 |
| walker |  | 9531 | 6 | python method at src/requests/models.py:112 |  |  | 0.509 |
| walker |  | 9539 | 8 | python method at src/requests/models.py:859 |  |  | 0.509 |
| walker |  | 9547 | 8 | python method at src/requests/models.py:874 |  |  | 0.509 |
| walker |  | 9555 | 8 | python method at src/requests/models.py:881 |  |  | 0.509 |
| walker |  | 9563 | 8 | python method at src/requests/models.py:889 |  |  | 0.509 |
| walker |  | 9571 | 8 | python method at src/requests/models.py:894 |  |  | 0.509 |
| walker |  | 9580 | 9 | python method at src/requests/models.py:1030 |  |  | 0.509 |
| walker |  | 9589 | 9 | python method at src/requests/models.py:1049 |  |  | 0.509 |
| walker |  | 9598 | 9 | python method at src/requests/models.py:1122 |  |  | 0.509 |
| walker |  | 9612 | 14 | python method at src/requests/models.py:405 |  |  | 0.509 |
| walker |  | 9620 | 8 | python method at src/requests/models.py:471 |  |  | 0.509 |
| walker |  | 9632 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.509 |
| walker |  | 9645 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.509 |
| walker |  | 9658 | 13 | python method doc at src/requests/models.py:465 |  |  | 0.509 |
| walker |  | 9671 | 13 | python method doc at src/requests/models.py:563 |  |  | 0.509 |
| walker |  | 9700 | 29 | python method at src/requests/models.py:816 |  |  | 0.509 |
| walker |  | 9715 | 15 | python method doc at src/requests/models.py:1030 |  |  | 0.509 |
| walker |  | 9748 | 33 | python method at src/requests/models.py:697 |  |  | 0.509 |
| walker |  | 9764 | 16 | python method doc at src/requests/models.py:855 |  |  | 0.509 |
| walker |  | 9798 | 34 | python method at src/requests/models.py:908 |  |  | 0.509 |
| walker |  | 9815 | 17 | python method doc at src/requests/models.py:652 |  |  | 0.498 |
| ns | 9815 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.498 |
| walker |  | 9851 | 36 | python method at src/requests/models.py:258 |  |  | 0.498 |
| walker |  | 9863 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.498 |
| ns | 9876 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.496 |
| walker |  | 9899 | 36 | python method at src/requests/models.py:904 |  |  | 0.496 |
| walker |  | 9935 | 36 | python method at src/requests/models.py:912 |  |  | 0.496 |
| walker |  | 9950 | 15 | python method at src/requests/models.py:137 |  |  | 0.496 |
| walker |  | 9950 | 0 | python method body at src/requests/models.py:137 body 139 |  |  | 0.496 |
| walker |  | 9968 | 18 | python method doc at src/requests/models.py:881 |  |  | 0.496 |
| ns | 9982 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.497 |
| walker |  | 9986 | 18 | python method doc at src/requests/models.py:1140 |  |  | 0.497 |
| ns | 10011 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.496 |
