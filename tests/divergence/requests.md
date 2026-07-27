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
| walker |  | 3514 | 36 | python decl names surface in src/requests/help.py |  |  | 0.628 |
| walker |  | 3514 | 0 | python decl at src/requests/help.py:37 |  |  | 0.628 |
| walker |  | 3514 | 0 | python decl at src/requests/help.py:69 |  |  | 0.628 |
| walker |  | 3514 | 0 | python decl at src/requests/help.py:128 |  |  | 0.628 |
| walker |  | 3527 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.628 |
| walker |  | 3541 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.628 |
| walker |  | 3559 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.628 |
| ns | 3695 |  | 206 | Session method locations | 5.4 | 5.1 | 0.610 |
| walker |  | 3873 | 314 | tool.ruff config in pyproject.toml |  |  | 0.610 |
| walker |  | 3923 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.611 |
| walker |  | 3923 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.611 |
| walker |  | 3949 | 26 | python imports in src/requests/packages.py |  |  | 0.611 |
| walker |  | 4006 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.612 |
| walker |  | 4006 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.612 |
| walker |  | 4023 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.613 |
| walker |  | 4076 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.616 |
| walker |  | 4094 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.618 |
| walker |  | 4151 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.628 |
| walker |  | 4151 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.628 |
| walker |  | 4151 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.628 |
| walker |  | 4151 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.628 |
| walker |  | 4159 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.628 |
| walker |  | 4190 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.628 |
| walker |  | 4222 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.628 |
| walker |  | 4275 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.628 |
| ns | 4420 |  | 725 | tests/conftest.py full | 6.1 |  | 0.573 |
| walker |  | 4548 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.573 |
| walker |  | 4605 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.573 |
| ns | 4737 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.552 |
| walker |  | 4781 | 176 | README.md section #2 |  |  | 0.552 |
| ns | 4935 |  | 198 | test_structures.py locations | 6.3 |  | 0.541 |
| ns | 5164 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.529 |
| walker |  | 5255 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.571 |
| walker |  | 5255 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.571 |
| walker |  | 5281 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.571 |
| walker |  | 5307 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.571 |
| walker |  | 5323 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.571 |
| walker |  | 5341 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.571 |
| walker |  | 5361 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.571 |
| ns | 5373 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.563 |
| walker |  | 5404 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.563 |
| walker |  | 5447 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.563 |
| walker |  | 5492 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.563 |
| ns | 5525 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.555 |
| walker |  | 5542 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.555 |
| walker |  | 5600 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.555 |
| ns | 5620 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.551 |
| walker |  | 5634 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.551 |
| ns | 5636 |  | 16 | tests/certs/ directory listing | 6.8 |  | 0.554 |
| walker |  | 5701 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.554 |
| walker |  | 5740 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.554 |
| walker |  | 5781 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.554 |
| walker |  | 5853 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.554 |
| walker |  | 5926 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.555 |
| walker |  | 5926 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.555 |
| walker |  | 5926 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.555 |
| walker |  | 5937 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.555 |
| walker |  | 5945 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.555 |
| walker |  | 5962 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.555 |
| walker |  | 5994 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.555 |
| walker |  | 6363 | 369 | python method sigs in src/requests/structures.py |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:64 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:67 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:70 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:73 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:76 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:80 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:89 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:92 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:101 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:105 |  |  | 0.573 |
| walker |  | 6363 | 0 | python method at src/requests/structures.py:129 |  |  | 0.573 |
| walker |  | 6370 | 7 | python method at src/requests/structures.py:126 |  |  | 0.573 |
| walker |  | 6370 | 0 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.573 |
| walker |  | 6379 | 9 | python method at src/requests/structures.py:123 |  |  | 0.573 |
| walker |  | 6379 | 0 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.573 |
| walker |  | 6397 | 18 | python method at src/requests/structures.py:118 |  |  | 0.573 |
| walker |  | 6428 | 31 | python method at src/requests/structures.py:108 |  |  | 0.573 |
| walker |  | 6460 | 32 | python method at src/requests/structures.py:59 |  |  | 0.573 |
| walker |  | 6476 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.573 |
| walker |  | 6530 | 54 | python method at src/requests/structures.py:49 |  |  | 0.573 |
| walker |  | 6619 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.573 |
| walker |  | 6619 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.573 |
| walker |  | 6619 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.573 |
| walker |  | 6630 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.573 |
| walker |  | 6645 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.573 |
| walker |  | 6799 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.573 |
| walker |  | 7084 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.575 |
| walker |  | 7084 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.575 |
| walker |  | 7097 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.575 |
| walker |  | 7128 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.575 |
| ns | 7134 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.514 |
| walker |  | 7162 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.514 |
| walker |  | 7201 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.514 |
| walker |  | 7241 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.514 |
| walker |  | 7289 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.514 |
| walker |  | 7348 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.514 |
| ns | 7415 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.528 |
| walker |  | 7420 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.528 |
| walker |  | 7465 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.528 |
| walker |  | 7541 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.528 |
| walker |  | 7698 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.528 |
| walker |  | 7698 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.528 |
| walker |  | 7710 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.528 |
| walker |  | 7791 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.528 |
| walker |  | 7833 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.528 |
| walker |  | 7886 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.528 |
| walker |  | 7892 | 6 | listing of 'tests/certs/valid' |  |  | 0.528 |
| walker |  | 7948 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.528 |
| walker |  | 8301 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.561 |
| walker |  | 8301 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.561 |
| walker |  | 8309 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.561 |
| walker |  | 8318 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.561 |
| walker |  | 8327 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.561 |
| walker |  | 8336 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.561 |
| walker |  | 8345 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.561 |
| walker |  | 8354 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.561 |
| walker |  | 8363 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.561 |
| walker |  | 8372 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.561 |
| walker |  | 8382 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.561 |
| walker |  | 8393 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.561 |
| walker |  | 8404 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.561 |
| walker |  | 8416 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.561 |
| walker |  | 8429 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.561 |
| walker |  | 8442 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.561 |
| walker |  | 8456 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.561 |
| walker |  | 8470 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.561 |
| walker |  | 8484 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.561 |
| walker |  | 8500 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.561 |
| walker |  | 8517 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.561 |
| walker |  | 8535 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.561 |
| walker |  | 8553 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.561 |
| walker |  | 8572 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.561 |
| walker |  | 8599 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.561 |
| walker |  | 8627 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.561 |
| walker |  | 8697 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.561 |
| walker |  | 8697 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.561 |
| walker |  | 8697 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.561 |
| walker |  | 8697 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.561 |
| walker |  | 8716 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.561 |
| walker |  | 8759 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.561 |
| ns | 8789 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.514 |
| walker |  | 8824 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.514 |
| walker |  | 8970 | 146 | python decl names surface in src/requests/models.py |  |  | 0.520 |
| walker |  | 8970 | 0 | python decl at src/requests/models.py:109 |  |  | 0.520 |
| walker |  | 8970 | 0 | python decl at src/requests/models.py:255 |  |  | 0.520 |
| walker |  | 8970 | 0 | python decl at src/requests/models.py:283 |  |  | 0.520 |
| walker |  | 8970 | 0 | python decl at src/requests/models.py:376 |  |  | 0.520 |
| walker |  | 8970 | 0 | python decl at src/requests/models.py:730 |  |  | 0.520 |
| walker |  | 8981 | 11 | python class body at src/requests/models.py:109 |  |  | 0.520 |
| walker |  | 8998 | 17 | python class body at src/requests/models.py:255 |  |  | 0.520 |
| walker |  | 9016 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.520 |
| walker |  | 9055 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.520 |
| ns | 9124 |  | 335 | auth.py class/method locations | 8.1 |  | 0.513 |
| walker |  | 9314 | 259 | python class body at src/requests/models.py:730 |  |  | 0.513 |
| ns | 9336 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.509 |
| ns | 9364 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.508 |
| walker |  | 9437 | 123 | python class body at src/requests/models.py:283 |  |  | 0.508 |
| ns | 9441 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.506 |
| walker |  | 9536 | 99 | python class body at src/requests/models.py:376 |  |  | 0.506 |
| walker |  | 9610 | 74 | python decl at src/requests/models.py:96 |  |  | 0.506 |
| ns | 9815 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.495 |
| ns | 9876 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.492 |
| ns | 9982 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.489 |
| ns | 10011 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.488 |
