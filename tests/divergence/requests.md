Score(3000)=0.648 I=0.877 C=0.479 ns_rows≤3K=17/41 (reached=9 partial=0 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 74 | 74 | listing of '.' |  |  | 1.000 |
| ns | 74 |  | 74 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 77 | 3 | listing of 'src' |  |  | 1.000 |
| walker |  | 111 | 34 | listing of 'ext' |  |  | 1.000 |
| ns | 164 |  | 90 | src/requests/ package listing | 1.2 |  | 0.659 |
| ns | 241 |  | 77 | tests/ top-level listing | 1.3 |  | 0.544 |
| walker |  | 286 | 175 | README headline in README.md |  |  | 0.558 |
| ns | 295 |  | 54 | docs/ directory listing | 1.4 |  | 0.492 |
| walker |  | 326 | 40 | headings outline in README.md |  |  | 0.492 |
| ns | 379 |  | 84 | .github/ and workflows/ listing | 1.5 |  | 0.431 |
| walker |  | 380 | 54 | listing of 'docs' |  |  | 0.563 |
| walker |  | 384 | 4 | listing of 'docs/_templates' |  |  | 0.563 |
| walker |  | 393 | 9 | listing of 'docs/_static' |  |  | 0.563 |
| walker |  | 403 | 10 | listing of 'docs/dev' |  |  | 0.563 |
| walker |  | 417 | 14 | listing of 'docs/_themes' |  |  | 0.563 |
| walker |  | 438 | 21 | listing of 'docs/user' |  |  | 0.563 |
| walker |  | 476 | 38 | listing of 'docs/community' |  |  | 0.563 |
| ns | 554 |  | 175 | README lede + usage example | 1.6 |  | 0.619 |
| walker |  | 566 | 90 | listing of 'src/requests' |  |  | 0.787 |
| ns | 754 |  | 200 | pyproject.toml identity | 1.7 |  | 0.722 |
| ns | 839 |  | 85 | pyproject.toml Python-version support + dependencies | 1.8 | 1.7 | 0.698 |
| ns | 1027 |  | 188 | Makefile build/test targets | 1.9 | 1.7 | 0.653 |
| ns | 1257 |  | 230 | __version__.py package metadata | 1.10 |  | 0.618 |
| ns | 1331 |  | 74 | api.py function locations | 2.1 |  | 0.600 |
| ns | 1561 |  | 230 | __init__.py public export tuple | 2.2 |  | 0.548 |
| walker |  | 1568 | 1002 | python imports in src/requests/__init__.py |  |  | 0.655 |
| walker |  | 1602 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.655 |
| walker |  | 1618 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.655 |
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
| ns | 4408 |  | 725 | tests/conftest.py full | 6.1 |  | 0.547 |
| ns | 4725 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.527 |
| walker |  | 4754 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.571 |
| walker |  | 4754 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.571 |
| walker |  | 4780 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.571 |
| walker |  | 4806 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.571 |
| walker |  | 4822 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.571 |
| walker |  | 4840 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.571 |
| walker |  | 4860 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.571 |
| walker |  | 4903 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.571 |
| ns | 4923 |  | 198 | test_structures.py locations | 6.3 |  | 0.560 |
| walker |  | 4946 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.560 |
| walker |  | 4991 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.560 |
| walker |  | 5041 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.560 |
| walker |  | 5099 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.560 |
| ns | 5152 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.548 |
| walker |  | 5156 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.548 |
| walker |  | 5332 | 176 | README.md section #2 |  |  | 0.548 |
| ns | 5361 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.540 |
| walker |  | 5366 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.540 |
| ns | 5513 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.533 |
| ns | 5608 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.529 |
| walker |  | 5619 | 253 | python decl names surface in src/requests/utils.py |  |  | 0.529 |
| ns | 5622 |  | 14 | tests/certs/ directory listing | 6.8 |  | 0.532 |
| walker |  | 5686 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.532 |
| walker |  | 5725 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.532 |
| walker |  | 5766 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.532 |
| walker |  | 5838 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.532 |
| walker |  | 5911 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.534 |
| walker |  | 5911 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.534 |
| walker |  | 5911 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.534 |
| walker |  | 5922 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.534 |
| walker |  | 5930 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.534 |
| walker |  | 5947 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.534 |
| walker |  | 5979 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.534 |
| walker |  | 6348 | 369 | python method sigs in src/requests/structures.py |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:64 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:67 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:70 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:73 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:80 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:89 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:92 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:101 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:105 |  |  | 0.552 |
| walker |  | 6348 | 0 | python method at src/requests/structures.py:129 |  |  | 0.552 |
| walker |  | 6355 | 7 | python method at src/requests/structures.py:126 |  |  | 0.552 |
| walker |  | 6355 | 0 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.552 |
| walker |  | 6364 | 9 | python method at src/requests/structures.py:123 |  |  | 0.552 |
| walker |  | 6364 | 0 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.552 |
| walker |  | 6382 | 18 | python method at src/requests/structures.py:118 |  |  | 0.552 |
| walker |  | 6413 | 31 | python method at src/requests/structures.py:108 |  |  | 0.552 |
| walker |  | 6445 | 32 | python method at src/requests/structures.py:59 |  |  | 0.552 |
| walker |  | 6461 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6515 | 54 | python method at src/requests/structures.py:49 |  |  | 0.552 |
| walker |  | 6604 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.552 |
| walker |  | 6604 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6604 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6615 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6630 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6784 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 7069 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.554 |
| walker |  | 7069 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.554 |
| walker |  | 7082 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.554 |
| walker |  | 7113 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.554 |
| ns | 7120 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.495 |
| walker |  | 7147 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.495 |
| walker |  | 7186 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.495 |
| walker |  | 7226 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.495 |
| walker |  | 7274 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.495 |
| walker |  | 7333 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.495 |
| ns | 7401 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.510 |
| walker |  | 7405 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.510 |
| walker |  | 7450 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.510 |
| walker |  | 7526 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.510 |
| walker |  | 7683 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.510 |
| walker |  | 7683 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.510 |
| walker |  | 7695 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.510 |
| walker |  | 7776 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.510 |
| walker |  | 7818 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.510 |
| walker |  | 7871 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.510 |
| walker |  | 8224 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8224 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8232 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8241 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8250 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8259 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8268 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8277 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8286 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8295 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8305 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8316 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8327 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8339 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8352 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8365 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8379 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8393 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8407 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8423 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8440 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8458 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8476 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8495 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8522 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8550 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8620 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8620 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8620 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.543 |
| walker |  | 8620 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.543 |
| walker |  | 8639 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8682 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8747 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8753 | 6 | listing of 'tests/certs/valid' |  |  | 0.543 |
| ns | 8775 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.498 |
| walker |  | 8809 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.498 |
| walker |  | 8955 | 146 | python decl names surface in src/requests/models.py |  |  | 0.504 |
| walker |  | 8955 | 0 | python decl at src/requests/models.py:109 |  |  | 0.504 |
| walker |  | 8955 | 0 | python decl at src/requests/models.py:255 |  |  | 0.504 |
| walker |  | 8955 | 0 | python decl at src/requests/models.py:283 |  |  | 0.504 |
| walker |  | 8955 | 0 | python decl at src/requests/models.py:376 |  |  | 0.504 |
| walker |  | 8955 | 0 | python decl at src/requests/models.py:730 |  |  | 0.504 |
| walker |  | 8966 | 11 | python class body at src/requests/models.py:109 |  |  | 0.504 |
| walker |  | 8983 | 17 | python class body at src/requests/models.py:255 |  |  | 0.504 |
| walker |  | 9001 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.504 |
| walker |  | 9040 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.504 |
| ns | 9110 |  | 335 | auth.py class/method locations | 8.1 |  | 0.497 |
| walker |  | 9299 | 259 | python class body at src/requests/models.py:730 |  |  | 0.497 |
| ns | 9322 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.493 |
| ns | 9350 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.492 |
| ns | 9427 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.496 |
| ns | 9801 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.485 |
| ns | 9862 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.483 |
| ns | 9968 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.479 |
| ns | 9997 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.478 |
