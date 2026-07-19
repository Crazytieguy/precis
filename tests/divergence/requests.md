Score(3000)=0.648 I=0.877 C=0.479 ns_rows≤3K=17/41 (reached=9 partial=0 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 74 | 74 | listing of '.' |  |  | 1.000 |
| ns | 74 |  | 74 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 77 | 3 | listing of 'src' |  |  | 1.000 |
| walker |  | 111 | 34 | listing of 'ext' |  |  | 1.000 |
| ns | 164 |  | 90 | src/requests/ package listing | 1.2 |  | 0.659 |
| walker |  | 165 | 54 | listing of 'docs' |  |  | 0.677 |
| walker |  | 169 | 4 | listing of 'docs/_templates' |  |  | 0.677 |
| walker |  | 178 | 9 | listing of 'docs/_static' |  |  | 0.677 |
| walker |  | 188 | 10 | listing of 'docs/dev' |  |  | 0.677 |
| walker |  | 202 | 14 | listing of 'docs/_themes' |  |  | 0.677 |
| walker |  | 223 | 21 | listing of 'docs/user' |  |  | 0.677 |
| ns | 241 |  | 77 | tests/ top-level listing | 1.3 |  | 0.560 |
| walker |  | 261 | 38 | listing of 'docs/community' |  |  | 0.560 |
| ns | 295 |  | 54 | docs/ directory listing | 1.4 |  | 0.628 |
| walker |  | 351 | 90 | listing of 'src/requests' |  |  | 0.856 |
| ns | 379 |  | 84 | .github/ and workflows/ listing | 1.5 |  | 0.750 |
| ns | 554 |  | 175 | README lede + usage example | 1.6 |  | 0.673 |
| ns | 754 |  | 200 | pyproject.toml identity | 1.7 |  | 0.618 |
| ns | 839 |  | 85 | pyproject.toml Python-version support + dependencies | 1.8 | 1.7 | 0.597 |
| ns | 1027 |  | 188 | Makefile build/test targets | 1.9 | 1.7 | 0.559 |
| ns | 1257 |  | 230 | __version__.py package metadata | 1.10 |  | 0.529 |
| ns | 1331 |  | 74 | api.py function locations | 2.1 |  | 0.513 |
| walker |  | 1353 | 1002 | python imports in src/requests/__init__.py |  |  | 0.522 |
| walker |  | 1387 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.522 |
| walker |  | 1403 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.522 |
| ns | 1561 |  | 230 | __init__.py public export tuple | 2.2 |  | 0.584 |
| walker |  | 1578 | 175 | README headline in README.md |  |  | 0.655 |
| walker |  | 1618 | 40 | headings outline in README.md |  |  | 0.655 |
| walker |  | 1665 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.655 |
| walker |  | 1723 | 58 | [package] in pyproject.toml |  |  | 0.659 |
| ns | 1828 |  | 267 | exceptions.py class hierarchy locations | 3.1 |  | 0.613 |
| walker |  | 1870 | 147 | README.md section #0 |  |  | 0.613 |
| walker |  | 1947 | 77 | manifest config in pyproject.toml |  |  | 0.620 |
| walker |  | 1967 | 20 | python imports in setup.py |  |  | 0.620 |
| walker |  | 2049 | 82 | tool.setuptools config in pyproject.toml |  |  | 0.620 |
| walker |  | 2188 | 139 | [dependencies] in pyproject.toml |  |  | 0.644 |
| walker |  | 2251 | 63 | README.md section #1 |  |  | 0.644 |
| walker |  | 2294 | 43 | listing of '.github' |  |  | 0.663 |
| ns | 2310 |  | 482 | hooks.py full (event hook dispatch) | 3.2 |  | 0.589 |
| walker |  | 2335 | 41 | listing of '.github/workflows' |  |  | 0.630 |
| walker |  | 2349 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.630 |
| ns | 2419 |  | 109 | structures.py class/method locations | 3.3 |  | 0.613 |
| walker |  | 2458 | 109 | tool.pytest+pyright config in pyproject.toml |  |  | 0.613 |
| walker |  | 2814 | 356 | plaintext config Makefile |  |  | 0.655 |
| ns | 2888 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.604 |
| walker |  | 2891 | 77 | listing of 'tests' |  |  | 0.654 |
| walker |  | 2901 | 10 | listing of 'tests/testserver' |  |  | 0.654 |
| walker |  | 2915 | 14 | listing of 'tests/certs' |  |  | 0.654 |
| ns | 2940 |  | 52 | models.py class roster | 4.1 |  | 0.648 |
| walker |  | 2954 | 39 | plaintext config docs/requirements.txt |  |  | 0.648 |
| ns | 3089 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.634 |
| walker |  | 3210 | 256 | README.md section #3 |  |  | 0.634 |
| walker |  | 3246 | 36 | python decl names surface in src/requests/help.py |  |  | 0.634 |
| walker |  | 3246 | 0 | python decl at src/requests/help.py:37 |  |  | 0.634 |
| walker |  | 3246 | 0 | python decl at src/requests/help.py:69 |  |  | 0.634 |
| walker |  | 3246 | 0 | python decl at src/requests/help.py:128 |  |  | 0.634 |
| walker |  | 3259 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.634 |
| walker |  | 3273 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.634 |
| walker |  | 3291 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.634 |
| ns | 3345 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.611 |
| ns | 3399 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.606 |
| ns | 3477 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.599 |
| walker |  | 3605 | 314 | tool.ruff config in pyproject.toml |  |  | 0.599 |
| walker |  | 3655 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.600 |
| walker |  | 3655 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.600 |
| walker |  | 3681 | 26 | python imports in src/requests/packages.py |  |  | 0.600 |
| ns | 3683 |  | 206 | Session method locations | 5.4 | 5.1 | 0.583 |
| walker |  | 3738 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.584 |
| walker |  | 3738 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.584 |
| walker |  | 3755 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.584 |
| walker |  | 3808 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.588 |
| walker |  | 3826 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.590 |
| walker |  | 3883 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.600 |
| walker |  | 3883 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.600 |
| walker |  | 3883 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.600 |
| walker |  | 3883 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.600 |
| walker |  | 3891 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.600 |
| walker |  | 3922 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.600 |
| walker |  | 3954 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.600 |
| walker |  | 4007 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.600 |
| walker |  | 4280 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.600 |
| walker |  | 4337 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.600 |
| ns | 4408 |  | 725 | tests/conftest.py full | 6.1 |  | 0.547 |
| walker |  | 4513 | 176 | README.md section #2 |  |  | 0.547 |
| ns | 4725 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.527 |
| ns | 4923 |  | 198 | test_structures.py locations | 6.3 |  | 0.517 |
| walker |  | 4987 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.560 |
| walker |  | 4987 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.560 |
| walker |  | 5013 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.560 |
| walker |  | 5039 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.560 |
| walker |  | 5055 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.560 |
| walker |  | 5073 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.560 |
| walker |  | 5093 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.560 |
| walker |  | 5136 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.560 |
| ns | 5152 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.548 |
| walker |  | 5179 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.548 |
| walker |  | 5224 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.548 |
| walker |  | 5274 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.548 |
| walker |  | 5332 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.548 |
| ns | 5361 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.540 |
| walker |  | 5366 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.540 |
| walker |  | 5433 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.540 |
| walker |  | 5472 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.540 |
| walker |  | 5513 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.533 |
| ns | 5513 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.533 |
| walker |  | 5585 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.533 |
| ns | 5608 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.529 |
| ns | 5622 |  | 14 | tests/certs/ directory listing | 6.8 |  | 0.532 |
| walker |  | 5658 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.533 |
| walker |  | 5658 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.533 |
| walker |  | 5658 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.533 |
| walker |  | 5669 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.533 |
| walker |  | 5677 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.533 |
| walker |  | 5694 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.533 |
| walker |  | 5726 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.533 |
| walker |  | 6095 | 369 | python method sigs in src/requests/structures.py |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:64 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:67 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:70 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:73 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:80 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:89 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:92 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:101 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:105 |  |  | 0.552 |
| walker |  | 6095 | 0 | python method at src/requests/structures.py:129 |  |  | 0.552 |
| walker |  | 6102 | 7 | python method at src/requests/structures.py:126 |  |  | 0.552 |
| walker |  | 6102 | 0 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.552 |
| walker |  | 6111 | 9 | python method at src/requests/structures.py:123 |  |  | 0.552 |
| walker |  | 6111 | 0 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.552 |
| walker |  | 6129 | 18 | python method at src/requests/structures.py:118 |  |  | 0.552 |
| walker |  | 6160 | 31 | python method at src/requests/structures.py:108 |  |  | 0.552 |
| walker |  | 6192 | 32 | python method at src/requests/structures.py:59 |  |  | 0.552 |
| walker |  | 6208 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6262 | 54 | python method at src/requests/structures.py:49 |  |  | 0.552 |
| walker |  | 6351 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.552 |
| walker |  | 6351 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6351 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6362 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6377 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6531 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6816 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.553 |
| walker |  | 6816 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.553 |
| walker |  | 6829 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.553 |
| walker |  | 6860 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.553 |
| walker |  | 6894 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.553 |
| walker |  | 6933 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.553 |
| walker |  | 6973 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.553 |
| walker |  | 7021 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.553 |
| walker |  | 7080 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.553 |
| ns | 7120 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.494 |
| walker |  | 7152 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.494 |
| walker |  | 7197 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.494 |
| walker |  | 7273 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.494 |
| ns | 7401 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.509 |
| walker |  | 7430 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.509 |
| walker |  | 7430 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.509 |
| walker |  | 7442 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.509 |
| walker |  | 7523 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.509 |
| walker |  | 7565 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.509 |
| walker |  | 7618 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.509 |
| walker |  | 7624 | 6 | listing of 'tests/certs/valid' |  |  | 0.509 |
| walker |  | 7680 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.509 |
| walker |  | 8033 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8033 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8041 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8050 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8059 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8068 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8077 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8086 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8095 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8104 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8114 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8125 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8136 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8148 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8161 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8174 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8188 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8202 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8216 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8232 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8249 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8267 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8285 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8304 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8331 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8359 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8429 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8429 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8429 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.543 |
| walker |  | 8429 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.543 |
| walker |  | 8448 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8491 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8556 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8702 | 146 | python decl names surface in src/requests/models.py |  |  | 0.549 |
| walker |  | 8702 | 0 | python decl at src/requests/models.py:109 |  |  | 0.549 |
| walker |  | 8702 | 0 | python decl at src/requests/models.py:255 |  |  | 0.549 |
| walker |  | 8702 | 0 | python decl at src/requests/models.py:283 |  |  | 0.549 |
| walker |  | 8702 | 0 | python decl at src/requests/models.py:376 |  |  | 0.549 |
| walker |  | 8702 | 0 | python decl at src/requests/models.py:730 |  |  | 0.549 |
| walker |  | 8713 | 11 | python class body at src/requests/models.py:109 |  |  | 0.549 |
| walker |  | 8730 | 17 | python class body at src/requests/models.py:255 |  |  | 0.549 |
| walker |  | 8748 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.549 |
| ns | 8775 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.503 |
| walker |  | 8787 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.503 |
| walker |  | 9046 | 259 | python class body at src/requests/models.py:730 |  |  | 0.503 |
| ns | 9110 |  | 335 | auth.py class/method locations | 8.1 |  | 0.497 |
| walker |  | 9169 | 123 | python class body at src/requests/models.py:283 |  |  | 0.497 |
| walker |  | 9268 | 99 | python class body at src/requests/models.py:376 |  |  | 0.497 |
| ns | 9322 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.493 |
| walker |  | 9342 | 74 | python decl at src/requests/models.py:96 |  |  | 0.493 |
| ns | 9350 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.492 |
| ns | 9427 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.490 |
| ns | 9801 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.479 |
| ns | 9862 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.477 |
| ns | 9968 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.473 |
| ns | 9997 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.472 |
