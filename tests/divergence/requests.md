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
| walker |  | 3249 | 36 | python decl names surface in src/requests/help.py |  |  | 0.634 |
| walker |  | 3249 | 0 | python decl at src/requests/help.py:37 |  |  | 0.634 |
| walker |  | 3249 | 0 | python decl at src/requests/help.py:69 |  |  | 0.634 |
| walker |  | 3249 | 0 | python decl at src/requests/help.py:128 |  |  | 0.634 |
| walker |  | 3262 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.634 |
| walker |  | 3276 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.634 |
| walker |  | 3294 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.634 |
| ns | 3357 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.611 |
| ns | 3411 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.606 |
| ns | 3489 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.599 |
| walker |  | 3608 | 314 | tool.ruff config in pyproject.toml |  |  | 0.599 |
| walker |  | 3658 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.600 |
| walker |  | 3658 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.600 |
| walker |  | 3684 | 26 | python imports in src/requests/packages.py |  |  | 0.600 |
| ns | 3695 |  | 206 | Session method locations | 5.4 | 5.1 | 0.583 |
| walker |  | 3741 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.584 |
| walker |  | 3741 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.584 |
| walker |  | 3758 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.584 |
| walker |  | 3811 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.588 |
| walker |  | 3829 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.590 |
| walker |  | 3886 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.600 |
| walker |  | 3886 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.600 |
| walker |  | 3886 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.600 |
| walker |  | 3886 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.600 |
| walker |  | 3894 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.600 |
| walker |  | 3925 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.600 |
| walker |  | 3957 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.600 |
| walker |  | 4010 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.600 |
| walker |  | 4283 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.600 |
| walker |  | 4340 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.600 |
| ns | 4420 |  | 725 | tests/conftest.py full | 6.1 |  | 0.547 |
| walker |  | 4516 | 176 | README.md section #2 |  |  | 0.547 |
| ns | 4737 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.527 |
| ns | 4935 |  | 198 | test_structures.py locations | 6.3 |  | 0.517 |
| walker |  | 4990 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.560 |
| walker |  | 4990 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.560 |
| walker |  | 5016 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.560 |
| walker |  | 5042 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.560 |
| walker |  | 5058 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.560 |
| walker |  | 5076 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.560 |
| walker |  | 5096 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.560 |
| walker |  | 5139 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.560 |
| ns | 5164 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.548 |
| walker |  | 5182 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.548 |
| walker |  | 5227 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.548 |
| walker |  | 5277 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.548 |
| walker |  | 5335 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.548 |
| walker |  | 5369 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.548 |
| ns | 5373 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.540 |
| walker |  | 5436 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.540 |
| walker |  | 5475 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.540 |
| walker |  | 5516 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.540 |
| ns | 5525 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.533 |
| walker |  | 5588 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.533 |
| ns | 5620 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.529 |
| ns | 5636 |  | 16 | tests/certs/ directory listing | 6.8 |  | 0.532 |
| walker |  | 5661 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.533 |
| walker |  | 5661 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.533 |
| walker |  | 5661 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.533 |
| walker |  | 5672 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.533 |
| walker |  | 5680 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.533 |
| walker |  | 5697 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.533 |
| walker |  | 5729 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.533 |
| walker |  | 6098 | 369 | python method sigs in src/requests/structures.py |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:64 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:67 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:70 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:73 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:80 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:89 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:92 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:101 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:105 |  |  | 0.552 |
| walker |  | 6098 | 0 | python method at src/requests/structures.py:129 |  |  | 0.552 |
| walker |  | 6105 | 7 | python method at src/requests/structures.py:126 |  |  | 0.552 |
| walker |  | 6105 | 0 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.552 |
| walker |  | 6114 | 9 | python method at src/requests/structures.py:123 |  |  | 0.552 |
| walker |  | 6114 | 0 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.552 |
| walker |  | 6132 | 18 | python method at src/requests/structures.py:118 |  |  | 0.552 |
| walker |  | 6163 | 31 | python method at src/requests/structures.py:108 |  |  | 0.552 |
| walker |  | 6195 | 32 | python method at src/requests/structures.py:59 |  |  | 0.552 |
| walker |  | 6211 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6265 | 54 | python method at src/requests/structures.py:49 |  |  | 0.552 |
| walker |  | 6354 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.552 |
| walker |  | 6354 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6354 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6365 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6380 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6534 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6819 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.553 |
| walker |  | 6819 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.553 |
| walker |  | 6832 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.553 |
| walker |  | 6863 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.553 |
| walker |  | 6897 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.553 |
| walker |  | 6936 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.553 |
| walker |  | 6976 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.553 |
| walker |  | 7024 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.553 |
| walker |  | 7083 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.553 |
| ns | 7134 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.494 |
| walker |  | 7155 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.494 |
| walker |  | 7200 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.494 |
| walker |  | 7276 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.494 |
| ns | 7415 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.509 |
| walker |  | 7433 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.509 |
| walker |  | 7433 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.509 |
| walker |  | 7445 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.509 |
| walker |  | 7526 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.509 |
| walker |  | 7568 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.509 |
| walker |  | 7621 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.509 |
| walker |  | 7627 | 6 | listing of 'tests/certs/valid' |  |  | 0.509 |
| walker |  | 7683 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.509 |
| walker |  | 8036 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8036 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8044 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8053 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8062 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8071 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8080 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8089 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8098 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8107 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8117 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8128 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8139 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8151 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8164 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8177 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8191 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8205 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8219 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8235 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8252 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8270 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8288 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8307 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8334 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8362 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8432 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8432 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8432 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.543 |
| walker |  | 8432 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.543 |
| walker |  | 8451 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8494 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8559 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8705 | 146 | python decl names surface in src/requests/models.py |  |  | 0.549 |
| walker |  | 8705 | 0 | python decl at src/requests/models.py:109 |  |  | 0.549 |
| walker |  | 8705 | 0 | python decl at src/requests/models.py:255 |  |  | 0.549 |
| walker |  | 8705 | 0 | python decl at src/requests/models.py:283 |  |  | 0.549 |
| walker |  | 8705 | 0 | python decl at src/requests/models.py:376 |  |  | 0.549 |
| walker |  | 8705 | 0 | python decl at src/requests/models.py:730 |  |  | 0.549 |
| walker |  | 8716 | 11 | python class body at src/requests/models.py:109 |  |  | 0.549 |
| walker |  | 8733 | 17 | python class body at src/requests/models.py:255 |  |  | 0.549 |
| walker |  | 8751 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.549 |
| ns | 8789 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.503 |
| walker |  | 8790 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.503 |
| walker |  | 9049 | 259 | python class body at src/requests/models.py:730 |  |  | 0.503 |
| ns | 9124 |  | 335 | auth.py class/method locations | 8.1 |  | 0.497 |
| walker |  | 9172 | 123 | python class body at src/requests/models.py:283 |  |  | 0.497 |
| walker |  | 9271 | 99 | python class body at src/requests/models.py:376 |  |  | 0.497 |
| ns | 9336 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.493 |
| walker |  | 9345 | 74 | python decl at src/requests/models.py:96 |  |  | 0.493 |
| ns | 9364 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.492 |
| ns | 9441 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.490 |
| ns | 9815 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.479 |
| ns | 9876 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.477 |
| ns | 9982 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.473 |
| ns | 10011 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.472 |
