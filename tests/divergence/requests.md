Score(3000)=0.614 I=0.861 C=0.438 ns_rows≤3K=17/41 (reached=8 partial=0 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 74 | 74 | listing of '.' |  |  | 1.000 |
| ns | 74 |  | 74 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 77 | 3 | listing of 'src' |  |  | 1.000 |
| ns | 164 |  | 90 | src/requests/ package listing | 1.2 |  | 0.659 |
| ns | 241 |  | 77 | tests/ top-level listing | 1.3 |  | 0.544 |
| walker |  | 252 | 175 | README headline in README.md |  |  | 0.558 |
| walker |  | 292 | 40 | headings outline in README.md |  |  | 0.558 |
| ns | 295 |  | 54 | docs/ directory listing | 1.4 |  | 0.492 |
| ns | 379 |  | 84 | .github/ and workflows/ listing | 1.5 |  | 0.431 |
| walker |  | 439 | 147 | README.md section #0 |  |  | 0.431 |
| walker |  | 497 | 58 | [package] in pyproject.toml |  |  | 0.431 |
| ns | 554 |  | 175 | README lede + usage example | 1.6 |  | 0.521 |
| walker |  | 638 | 141 | [dependencies] in pyproject.toml |  |  | 0.524 |
| walker |  | 728 | 90 | listing of 'src/requests' |  |  | 0.701 |
| ns | 754 |  | 200 | pyproject.toml identity | 1.7 |  | 0.647 |
| ns | 839 |  | 85 | pyproject.toml Python-version support + dependencies | 1.8 | 1.7 | 0.662 |
| ns | 1027 |  | 188 | Makefile build/test targets | 1.9 | 1.7 | 0.619 |
| ns | 1257 |  | 230 | __version__.py package metadata | 1.10 |  | 0.586 |
| ns | 1331 |  | 74 | api.py function locations | 2.1 |  | 0.569 |
| ns | 1561 |  | 230 | __init__.py public export tuple | 2.2 |  | 0.520 |
| walker |  | 1730 | 1002 | python imports in src/requests/__init__.py |  |  | 0.628 |
| walker |  | 1764 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.628 |
| walker |  | 1780 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.628 |
| walker |  | 1814 | 34 | listing of 'ext' |  |  | 0.628 |
| ns | 1828 |  | 267 | exceptions.py class hierarchy locations | 3.1 |  | 0.584 |
| walker |  | 1861 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.584 |
| walker |  | 1915 | 54 | listing of 'docs' |  |  | 0.637 |
| walker |  | 1936 | 21 | listing of 'docs/user' |  |  | 0.637 |
| walker |  | 1956 | 20 | python imports in setup.py |  |  | 0.637 |
| walker |  | 2019 | 63 | README.md section #1 |  |  | 0.637 |
| walker |  | 2057 | 38 | listing of 'docs/community' |  |  | 0.637 |
| walker |  | 2071 | 14 | listing of 'docs/_themes' |  |  | 0.637 |
| walker |  | 2075 | 4 | listing of 'docs/_templates' |  |  | 0.637 |
| walker |  | 2114 | 39 | plaintext config docs/requirements.txt |  |  | 0.637 |
| walker |  | 2123 | 9 | listing of 'docs/_static' |  |  | 0.637 |
| walker |  | 2159 | 36 | python decl names surface in src/requests/help.py |  |  | 0.637 |
| walker |  | 2159 | 0 | python decl at src/requests/help.py:37 |  |  | 0.637 |
| walker |  | 2159 | 0 | python decl at src/requests/help.py:69 |  |  | 0.637 |
| walker |  | 2159 | 0 | python decl at src/requests/help.py:128 |  |  | 0.637 |
| walker |  | 2172 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.637 |
| walker |  | 2186 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.637 |
| walker |  | 2204 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.637 |
| walker |  | 2247 | 43 | listing of '.github' |  |  | 0.655 |
| walker |  | 2288 | 41 | listing of '.github/workflows' |  |  | 0.701 |
| ns | 2310 |  | 482 | hooks.py full (event hook dispatch) | 3.2 |  | 0.624 |
| walker |  | 2365 | 77 | listing of 'tests' |  |  | 0.680 |
| walker |  | 2375 | 10 | listing of 'tests/testserver' |  |  | 0.680 |
| walker |  | 2389 | 14 | listing of 'tests/certs' |  |  | 0.681 |
| walker |  | 2399 | 10 | listing of 'docs/dev' |  |  | 0.681 |
| ns | 2419 |  | 109 | structures.py class/method locations | 3.3 |  | 0.662 |
| walker |  | 2449 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.662 |
| walker |  | 2449 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.662 |
| walker |  | 2475 | 26 | python imports in src/requests/packages.py |  |  | 0.662 |
| walker |  | 2532 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.663 |
| walker |  | 2532 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.663 |
| walker |  | 2549 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.664 |
| walker |  | 2602 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.669 |
| walker |  | 2620 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.671 |
| walker |  | 2677 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.672 |
| walker |  | 2677 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.672 |
| walker |  | 2677 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.672 |
| walker |  | 2677 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.672 |
| walker |  | 2685 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.672 |
| walker |  | 2716 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.672 |
| walker |  | 2748 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.672 |
| walker |  | 2801 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.672 |
| ns | 2888 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.619 |
| ns | 2940 |  | 52 | models.py class roster | 4.1 |  | 0.614 |
| walker |  | 3074 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.614 |
| ns | 3089 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.600 |
| walker |  | 3131 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.600 |
| walker |  | 3156 | 25 | python decl names surface #1 in src/requests/cookies.py |  |  | 0.600 |
| walker |  | 3223 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.600 |
| walker |  | 3239 | 16 | python decl names surface #2 in src/requests/exceptions.py |  |  | 0.600 |
| walker |  | 3239 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.600 |
| walker |  | 3253 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.600 |
| walker |  | 3267 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.600 |
| ns | 3345 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.579 |
| ns | 3399 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.584 |
| walker |  | 3443 | 176 | README.md section #2 |  |  | 0.584 |
| ns | 3477 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.578 |
| walker |  | 3644 | 201 | python method sigs in src/requests/sessions.py |  |  | 0.593 |
| walker |  | 3644 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.593 |
| walker |  | 3644 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.593 |
| walker |  | 3644 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.593 |
| walker |  | 3644 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.593 |
| walker |  | 3644 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.593 |
| walker |  | 3644 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.593 |
| walker |  | 3644 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.593 |
| walker |  | 3650 | 6 | python method body at src/requests/sessions.py:505 body 506 |  |  | 0.593 |
| walker |  | 3656 | 6 | python method body at src/requests/sessions.py:508 body 509 |  |  | 0.593 |
| walker |  | 3682 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.593 |
| ns | 3683 |  | 206 | Session method locations | 5.4 | 5.1 | 0.578 |
| walker |  | 3708 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.578 |
| walker |  | 3726 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.578 |
| walker |  | 3746 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.578 |
| walker |  | 3791 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.578 |
| walker |  | 3841 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.578 |
| walker |  | 3880 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.578 |
| walker |  | 3953 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.580 |
| walker |  | 3953 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.580 |
| walker |  | 3953 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.580 |
| walker |  | 3964 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.580 |
| walker |  | 3972 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.580 |
| walker |  | 3989 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.580 |
| walker |  | 4021 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.580 |
| walker |  | 4350 | 329 | python method sigs in src/requests/structures.py |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:64 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:67 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:70 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:73 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:76 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:80 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:89 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:92 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:101 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:105 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:123 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:126 |  |  | 0.605 |
| walker |  | 4350 | 0 | python method at src/requests/structures.py:129 |  |  | 0.605 |
| walker |  | 4368 | 18 | python method at src/requests/structures.py:118 |  |  | 0.605 |
| walker |  | 4377 | 9 | python method body at src/requests/structures.py:73 body 74 |  |  | 0.605 |
| walker |  | 4386 | 9 | python method body at src/requests/structures.py:92 body 93 |  |  | 0.605 |
| walker |  | 4396 | 10 | python method body at src/requests/structures.py:67 body 68 |  |  | 0.605 |
| ns | 4408 |  | 725 | tests/conftest.py full | 6.1 |  | 0.552 |
| walker |  | 4427 | 31 | python method at src/requests/structures.py:108 |  |  | 0.552 |
| walker |  | 4459 | 32 | python method at src/requests/structures.py:59 |  |  | 0.552 |
| walker |  | 4475 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 4487 | 12 | python method body at src/requests/structures.py:64 body 65 |  |  | 0.552 |
| walker |  | 4499 | 12 | python method body at src/requests/structures.py:89 body 90 |  |  | 0.552 |
| walker |  | 4512 | 13 | python method body at src/requests/structures.py:105 body 106 |  |  | 0.552 |
| walker |  | 4525 | 13 | python method body at src/requests/structures.py:118 body 121 |  |  | 0.552 |
| walker |  | 4538 | 13 | python method body at src/requests/structures.py:129 body 130 |  |  | 0.552 |
| walker |  | 4553 | 15 | python method body at src/requests/structures.py:59 body 62 |  |  | 0.552 |
| walker |  | 4570 | 17 | python method body at src/requests/structures.py:101 body 102 |  |  | 0.552 |
| walker |  | 4624 | 54 | python method at src/requests/structures.py:49 |  |  | 0.552 |
| walker |  | 4644 | 20 | python method body at src/requests/structures.py:70 body 71 |  |  | 0.552 |
| walker |  | 4668 | 24 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.552 |
| ns | 4725 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.531 |
| walker |  | 4757 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.531 |
| walker |  | 4757 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.531 |
| walker |  | 4757 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.531 |
| walker |  | 4768 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.531 |
| walker |  | 4783 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.531 |
| ns | 4923 |  | 198 | test_structures.py locations | 6.3 |  | 0.521 |
| walker |  | 4937 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.521 |
| ns | 5152 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.510 |
| walker |  | 5222 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.511 |
| walker |  | 5222 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.511 |
| walker |  | 5229 | 7 | python method body at src/requests/adapters.py:125 body 126 |  |  | 0.511 |
| walker |  | 5236 | 7 | python method body at src/requests/adapters.py:599 body 611 |  |  | 0.511 |
| walker |  | 5249 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.511 |
| walker |  | 5257 | 8 | python method body at src/requests/adapters.py:153 body 155 |  |  | 0.511 |
| walker |  | 5288 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.511 |
| walker |  | 5322 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.511 |
| walker |  | 5361 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.504 |
| ns | 5361 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.504 |
| walker |  | 5401 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.504 |
| walker |  | 5449 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.504 |
| walker |  | 5508 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.504 |
| ns | 5513 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.497 |
| walker |  | 5529 | 21 | python method body at src/requests/adapters.py:403 body 453 |  |  | 0.497 |
| walker |  | 5551 | 22 | python method body at src/requests/adapters.py:223 body 224 |  |  | 0.497 |
| ns | 5608 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.493 |
| ns | 5622 |  | 14 | tests/certs/ directory listing | 6.8 |  | 0.497 |
| walker |  | 5623 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.497 |
| walker |  | 5699 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.497 |
| walker |  | 5856 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.497 |
| walker |  | 5856 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.497 |
| walker |  | 5868 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.497 |
| walker |  | 5895 | 27 | python method body at src/requests/structures.py:76 body 78 |  |  | 0.497 |
| walker |  | 5922 | 27 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.497 |
| walker |  | 6512 | 590 | manifest config in pyproject.toml |  |  | 0.501 |
| walker |  | 6565 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.501 |
| walker |  | 6621 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.501 |
| walker |  | 6767 | 146 | python decl names surface in src/requests/models.py |  |  | 0.509 |
| walker |  | 6767 | 0 | python decl at src/requests/models.py:109 |  |  | 0.509 |
| walker |  | 6767 | 0 | python decl at src/requests/models.py:255 |  |  | 0.509 |
| walker |  | 6767 | 0 | python decl at src/requests/models.py:283 |  |  | 0.509 |
| walker |  | 6767 | 0 | python decl at src/requests/models.py:376 |  |  | 0.509 |
| walker |  | 6767 | 0 | python decl at src/requests/models.py:730 |  |  | 0.509 |
| walker |  | 6778 | 11 | python class body at src/requests/models.py:109 |  |  | 0.509 |
| walker |  | 6795 | 17 | python class body at src/requests/models.py:255 |  |  | 0.509 |
| walker |  | 6813 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.509 |
| walker |  | 6852 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.509 |
| walker |  | 7111 | 259 | python class body at src/requests/models.py:730 |  |  | 0.509 |
| ns | 7120 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.455 |
| walker |  | 7234 | 123 | python class body at src/requests/models.py:283 |  |  | 0.455 |
| walker |  | 7333 | 99 | python class body at src/requests/models.py:376 |  |  | 0.455 |
| ns | 7401 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.472 |
| walker |  | 7482 | 149 | python method sigs in src/requests/models.py |  |  | 0.472 |
| walker |  | 7482 | 0 | python method at src/requests/models.py:271 |  |  | 0.472 |
| walker |  | 7482 | 0 | python method at src/requests/models.py:355 |  |  | 0.472 |
| walker |  | 7482 | 0 | python method at src/requests/models.py:358 |  |  | 0.472 |
| walker |  | 7495 | 13 | python method at src/requests/models.py:112 |  |  | 0.472 |
| walker |  | 7503 | 8 | python method at src/requests/models.py:133 |  |  | 0.472 |
| walker |  | 7511 | 8 | python method at src/requests/models.py:137 |  |  | 0.472 |
| walker |  | 7519 | 8 | python method at src/requests/models.py:147 |  |  | 0.472 |
| walker |  | 7532 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.472 |
| walker |  | 7544 | 12 | python method body at src/requests/models.py:355 body 356 |  |  | 0.472 |
| walker |  | 7580 | 36 | python method at src/requests/models.py:258 |  |  | 0.472 |
| walker |  | 7592 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.472 |
| walker |  | 7616 | 24 | python method doc at src/requests/models.py:358 |  |  | 0.472 |
| walker |  | 7651 | 35 | python method doc at src/requests/models.py:271 |  |  | 0.472 |
| walker |  | 7725 | 74 | python decl at src/requests/models.py:96 |  |  | 0.472 |
| walker |  | 7759 | 34 | python method at src/requests/models.py:141 |  |  | 0.472 |
| walker |  | 7893 | 134 | python decl names surface in src/requests/api.py |  |  | 0.483 |
| walker |  | 7893 | 0 | python decl at src/requests/api.py:90 |  |  | 0.483 |
| walker |  | 7893 | 0 | python decl at src/requests/api.py:102 |  |  | 0.483 |
| walker |  | 7893 | 0 | python decl at src/requests/api.py:171 |  |  | 0.483 |
| walker |  | 7928 | 35 | python decl at src/requests/api.py:24 |  |  | 0.483 |
| walker |  | 7943 | 15 | python decl body at src/requests/api.py:90 body 99 |  |  | 0.483 |
| walker |  | 7958 | 15 | python decl body at src/requests/api.py:171 body 180 |  |  | 0.483 |
| walker |  | 7977 | 19 | python decl doc at src/requests/api.py:24 |  |  | 0.483 |
| walker |  | 8017 | 40 | python decl at src/requests/api.py:74 |  |  | 0.483 |
| walker |  | 8057 | 40 | python decl at src/requests/api.py:137 |  |  | 0.483 |
| walker |  | 8097 | 40 | python decl at src/requests/api.py:154 |  |  | 0.483 |
| walker |  | 8115 | 18 | python decl body at src/requests/api.py:74 body 87 |  |  | 0.483 |
| walker |  | 8133 | 18 | python decl body at src/requests/api.py:137 body 151 |  |  | 0.483 |
| walker |  | 8151 | 18 | python decl body at src/requests/api.py:154 body 168 |  |  | 0.483 |
| walker |  | 8215 | 64 | python decl at src/requests/api.py:117 |  |  | 0.483 |
| walker |  | 8236 | 21 | python decl body at src/requests/api.py:117 body 134 |  |  | 0.483 |
| walker |  | 8326 | 90 | python decl doc at src/requests/api.py:90 |  |  | 0.483 |
| walker |  | 8416 | 90 | python decl doc at src/requests/api.py:171 |  |  | 0.483 |
| walker |  | 8431 | 15 | python method body at src/requests/models.py:133 body 135 |  |  | 0.483 |
| walker |  | 8446 | 15 | python method body at src/requests/models.py:137 body 139 |  |  | 0.483 |
| walker |  | 8461 | 15 | python method body at src/requests/models.py:141 body 145 |  |  | 0.483 |
| walker |  | 8558 | 97 | python method at src/requests/adapters.py:128 |  |  | 0.483 |
| walker |  | 8568 | 10 | python method body at src/requests/adapters.py:128 body 151 |  |  | 0.483 |
| walker |  | 8665 | 97 | python method at src/requests/adapters.py:634 |  |  | 0.483 |
| ns | 8775 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.444 |
| walker |  | 8821 | 156 | python decl names surface in src/requests/exceptions.py |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.452 |
| walker |  | 8821 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.452 |
| walker |  | 8830 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.452 |
| walker |  | 8839 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.452 |
| walker |  | 8848 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.452 |
| walker |  | 8857 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.452 |
| walker |  | 8866 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.452 |
| walker |  | 8876 | 10 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.452 |
| walker |  | 8889 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.452 |
| walker |  | 8903 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.452 |
| walker |  | 8921 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.452 |
| walker |  | 8948 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.452 |
| walker |  | 8976 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.452 |
| walker |  | 9019 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.452 |
| walker |  | 9089 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.452 |
| walker |  | 9089 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.452 |
| walker |  | 9089 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.452 |
| walker |  | 9089 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.452 |
| walker |  | 9108 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.452 |
| ns | 9110 |  | 335 | auth.py class/method locations | 8.1 |  | 0.446 |
| walker |  | 9123 | 15 | python method body at src/requests/exceptions.py:55 body 63 |  |  | 0.446 |
| walker |  | 9188 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.446 |
| walker |  | 9314 | 126 | python decl names surface in src/requests/auth.py |  |  | 0.447 |
| walker |  | 9314 | 0 | python decl at src/requests/auth.py:34 |  |  | 0.447 |
| walker |  | 9314 | 0 | python decl at src/requests/auth.py:78 |  |  | 0.447 |
| walker |  | 9314 | 0 | python decl at src/requests/auth.py:85 |  |  | 0.447 |
| walker |  | 9314 | 0 | python decl at src/requests/auth.py:116 |  |  | 0.447 |
| walker |  | 9314 | 0 | python decl at src/requests/auth.py:124 |  |  | 0.447 |
| ns | 9322 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.443 |
| walker |  | 9329 | 15 | python decl doc at src/requests/auth.py:78 |  |  | 0.443 |
| walker |  | 9346 | 17 | python decl doc at src/requests/auth.py:85 |  |  | 0.443 |
| ns | 9350 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.442 |
| walker |  | 9368 | 22 | python class body at src/requests/auth.py:85 |  |  | 0.442 |
| walker |  | 9385 | 17 | python decl doc at src/requests/auth.py:116 |  |  | 0.442 |
| walker |  | 9402 | 17 | python decl doc at src/requests/auth.py:124 |  |  | 0.442 |
| walker |  | 9414 | 12 | python decl doc at src/requests/auth.py:34 |  |  | 0.442 |
| ns | 9427 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.441 |
| walker |  | 9505 | 91 | python class body at src/requests/auth.py:124 |  |  | 0.441 |
| ns | 9801 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.431 |
| walker |  | 9857 | 352 | python method sigs in src/requests/auth.py |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:81 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:91 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:93 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:96 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:100 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:108 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:111 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:119 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:136 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:138 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:141 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:157 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:268 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:273 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:345 |  |  | 0.450 |
| walker |  | 9857 | 0 | python method at src/requests/auth.py:353 |  |  | 0.450 |
| ns | 9862 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.448 |
| walker |  | 9871 | 14 | python method at src/requests/auth.py:321 |  |  | 0.448 |
| walker |  | 9886 | 15 | python method at src/requests/auth.py:147 |  |  | 0.448 |
| walker |  | 9895 | 9 | python method body at src/requests/auth.py:108 body 109 |  |  | 0.448 |
| walker |  | 9904 | 9 | python method body at src/requests/auth.py:353 body 354 |  |  | 0.448 |
| walker |  | 9919 | 15 | python method doc at src/requests/auth.py:268 |  |  | 0.448 |
| walker |  | 9933 | 14 | python method body at src/requests/auth.py:81 body 82 |  |  | 0.448 |
| walker |  | 9955 | 22 | python method doc at src/requests/auth.py:157 |  |  | 0.448 |
| ns | 9968 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.444 |
| walker |  | 9973 | 18 | python method body at src/requests/auth.py:96 body 97 |  |  | 0.444 |
| walker |  | 9994 | 21 | python method body at src/requests/auth.py:91 body 92 |  |  | 0.444 |
| ns | 9997 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.443 |
