Score(3000)=0.651 I=0.876 C=0.484 ns_rows≤3K=17/41 (reached=9 partial=0 missing=8)

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
| walker |  | 350 | 58 | [package] in pyproject.toml |  |  | 0.492 |
| ns | 379 |  | 84 | .github/ and workflows/ listing | 1.5 |  | 0.431 |
| walker |  | 497 | 147 | README.md section #0 |  |  | 0.431 |
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
| walker |  | 1919 | 4 | listing of 'docs/_templates' |  |  | 0.637 |
| walker |  | 1928 | 9 | listing of 'docs/_static' |  |  | 0.637 |
| walker |  | 1938 | 10 | listing of 'docs/dev' |  |  | 0.637 |
| walker |  | 1952 | 14 | listing of 'docs/_themes' |  |  | 0.637 |
| walker |  | 1973 | 21 | listing of 'docs/user' |  |  | 0.637 |
| walker |  | 1993 | 20 | python imports in setup.py |  |  | 0.637 |
| walker |  | 2056 | 63 | README.md section #1 |  |  | 0.637 |
| walker |  | 2094 | 38 | listing of 'docs/community' |  |  | 0.637 |
| ns | 2310 |  | 482 | hooks.py full (event hook dispatch) | 3.2 |  | 0.566 |
| ns | 2419 |  | 109 | structures.py class/method locations | 3.3 |  | 0.551 |
| walker |  | 2450 | 356 | plaintext config Makefile |  |  | 0.595 |
| walker |  | 2489 | 39 | plaintext config docs/requirements.txt |  |  | 0.595 |
| walker |  | 2532 | 43 | listing of '.github' |  |  | 0.611 |
| walker |  | 2573 | 41 | listing of '.github/workflows' |  |  | 0.649 |
| walker |  | 2587 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.649 |
| walker |  | 2664 | 77 | listing of 'tests' |  |  | 0.704 |
| walker |  | 2674 | 10 | listing of 'tests/testserver' |  |  | 0.704 |
| walker |  | 2688 | 14 | listing of 'tests/certs' |  |  | 0.704 |
| walker |  | 2724 | 36 | python decl names surface in src/requests/help.py |  |  | 0.704 |
| walker |  | 2724 | 0 | python decl at src/requests/help.py:37 |  |  | 0.704 |
| walker |  | 2724 | 0 | python decl at src/requests/help.py:69 |  |  | 0.704 |
| walker |  | 2724 | 0 | python decl at src/requests/help.py:128 |  |  | 0.704 |
| walker |  | 2737 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.704 |
| walker |  | 2751 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.704 |
| walker |  | 2769 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.704 |
| walker |  | 2819 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.704 |
| walker |  | 2819 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.704 |
| walker |  | 2845 | 26 | python imports in src/requests/packages.py |  |  | 0.704 |
| ns | 2888 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.649 |
| walker |  | 2902 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.650 |
| walker |  | 2902 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.650 |
| walker |  | 2919 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.651 |
| ns | 2940 |  | 52 | models.py class roster | 4.1 |  | 0.645 |
| walker |  | 2972 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.649 |
| walker |  | 2990 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.651 |
| walker |  | 3047 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.652 |
| walker |  | 3047 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.652 |
| walker |  | 3047 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.652 |
| walker |  | 3047 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.652 |
| walker |  | 3055 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.652 |
| walker |  | 3086 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.652 |
| ns | 3089 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.638 |
| walker |  | 3118 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.638 |
| walker |  | 3171 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.638 |
| ns | 3345 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.615 |
| ns | 3399 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.619 |
| walker |  | 3444 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.619 |
| ns | 3477 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.613 |
| ns | 3683 |  | 206 | Session method locations | 5.4 | 5.1 | 0.595 |
| walker |  | 3918 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.645 |
| walker |  | 3918 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.645 |
| walker |  | 3944 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.645 |
| walker |  | 3970 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.645 |
| walker |  | 3986 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.645 |
| walker |  | 4004 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.645 |
| walker |  | 4024 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.645 |
| walker |  | 4067 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.645 |
| walker |  | 4110 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.645 |
| walker |  | 4155 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.645 |
| walker |  | 4205 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.645 |
| walker |  | 4263 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.645 |
| walker |  | 4320 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.645 |
| walker |  | 4354 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.645 |
| ns | 4408 |  | 725 | tests/conftest.py full | 6.1 |  | 0.589 |
| walker |  | 4610 | 256 | README.md section #3 |  |  | 0.589 |
| walker |  | 4677 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.589 |
| walker |  | 4716 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.589 |
| ns | 4725 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.567 |
| walker |  | 4892 | 176 | README.md section #2 |  |  | 0.567 |
| ns | 4923 |  | 198 | test_structures.py locations | 6.3 |  | 0.556 |
| walker |  | 4933 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.556 |
| walker |  | 5005 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.556 |
| walker |  | 5078 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.558 |
| walker |  | 5078 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.558 |
| walker |  | 5078 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.558 |
| walker |  | 5089 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.558 |
| walker |  | 5097 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.558 |
| walker |  | 5114 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.558 |
| walker |  | 5146 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.558 |
| ns | 5152 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.546 |
| ns | 5361 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.538 |
| ns | 5513 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.531 |
| walker |  | 5515 | 369 | python method sigs in src/requests/structures.py |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:64 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:67 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:70 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:73 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:76 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:80 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:89 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:92 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:101 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:105 |  |  | 0.550 |
| walker |  | 5515 | 0 | python method at src/requests/structures.py:129 |  |  | 0.550 |
| walker |  | 5522 | 7 | python method at src/requests/structures.py:126 |  |  | 0.550 |
| walker |  | 5522 | 0 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.550 |
| walker |  | 5531 | 9 | python method at src/requests/structures.py:123 |  |  | 0.550 |
| walker |  | 5531 | 0 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.550 |
| walker |  | 5549 | 18 | python method at src/requests/structures.py:118 |  |  | 0.550 |
| walker |  | 5580 | 31 | python method at src/requests/structures.py:108 |  |  | 0.550 |
| ns | 5608 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.545 |
| walker |  | 5612 | 32 | python method at src/requests/structures.py:59 |  |  | 0.545 |
| ns | 5622 |  | 14 | tests/certs/ directory listing | 6.8 |  | 0.548 |
| walker |  | 5628 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.548 |
| walker |  | 5682 | 54 | python method at src/requests/structures.py:49 |  |  | 0.548 |
| walker |  | 5771 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.548 |
| walker |  | 5771 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.548 |
| walker |  | 5771 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.548 |
| walker |  | 5782 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.548 |
| walker |  | 5797 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.548 |
| walker |  | 5951 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.548 |
| walker |  | 6236 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.550 |
| walker |  | 6236 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.550 |
| walker |  | 6249 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.550 |
| walker |  | 6280 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.550 |
| walker |  | 6314 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.550 |
| walker |  | 6353 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.550 |
| walker |  | 6393 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.550 |
| walker |  | 6441 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.550 |
| walker |  | 6500 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.550 |
| walker |  | 6572 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.550 |
| walker |  | 6617 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.550 |
| walker |  | 6693 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.550 |
| walker |  | 6850 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.550 |
| walker |  | 6850 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.550 |
| walker |  | 6862 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.550 |
| ns | 7120 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.491 |
| ns | 7401 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.506 |
| walker |  | 7452 | 590 | manifest config in pyproject.toml |  |  | 0.509 |
| walker |  | 7533 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.509 |
| walker |  | 7575 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.509 |
| walker |  | 7628 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.509 |
| walker |  | 7981 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 7981 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 7989 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 7998 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8007 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8016 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8025 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8034 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8043 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8052 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8062 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8073 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8084 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8096 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8109 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8122 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8136 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8150 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8164 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8180 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8197 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8215 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8233 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8252 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8279 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8307 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8377 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8377 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8377 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.543 |
| walker |  | 8377 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.543 |
| walker |  | 8396 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.543 |
| walker |  | 8439 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8504 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8560 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.543 |
| walker |  | 8706 | 146 | python decl names surface in src/requests/models.py |  |  | 0.549 |
| walker |  | 8706 | 0 | python decl at src/requests/models.py:109 |  |  | 0.549 |
| walker |  | 8706 | 0 | python decl at src/requests/models.py:255 |  |  | 0.549 |
| walker |  | 8706 | 0 | python decl at src/requests/models.py:283 |  |  | 0.549 |
| walker |  | 8706 | 0 | python decl at src/requests/models.py:376 |  |  | 0.549 |
| walker |  | 8706 | 0 | python decl at src/requests/models.py:730 |  |  | 0.549 |
| walker |  | 8717 | 11 | python class body at src/requests/models.py:109 |  |  | 0.549 |
| walker |  | 8734 | 17 | python class body at src/requests/models.py:255 |  |  | 0.549 |
| walker |  | 8752 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.549 |
| ns | 8775 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.503 |
| walker |  | 8791 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.503 |
| walker |  | 9050 | 259 | python class body at src/requests/models.py:730 |  |  | 0.503 |
| ns | 9110 |  | 335 | auth.py class/method locations | 8.1 |  | 0.497 |
| ns | 9322 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.493 |
| ns | 9350 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.492 |
| ns | 9427 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.490 |
| ns | 9801 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.479 |
| ns | 9862 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.477 |
| walker |  | 9873 | 823 | python method sigs in src/requests/models.py |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:271 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:355 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:358 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:451 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:454 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:465 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:563 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:652 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:720 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:763 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:810 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:813 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:824 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:832 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:835 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:845 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:855 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:1087 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:1140 |  |  | 0.513 |
| walker |  | 9873 | 0 | python method at src/requests/models.py:1169 |  |  | 0.513 |
| walker |  | 9879 | 6 | python method at src/requests/models.py:112 |  |  | 0.513 |
| walker |  | 9887 | 8 | python method at src/requests/models.py:859 |  |  | 0.513 |
| walker |  | 9895 | 8 | python method at src/requests/models.py:874 |  |  | 0.513 |
| walker |  | 9903 | 8 | python method at src/requests/models.py:881 |  |  | 0.513 |
| walker |  | 9911 | 8 | python method at src/requests/models.py:889 |  |  | 0.513 |
| walker |  | 9919 | 8 | python method at src/requests/models.py:894 |  |  | 0.513 |
| walker |  | 9928 | 9 | python method at src/requests/models.py:1030 |  |  | 0.513 |
| walker |  | 9937 | 9 | python method at src/requests/models.py:1049 |  |  | 0.513 |
| walker |  | 9946 | 9 | python method at src/requests/models.py:1122 |  |  | 0.513 |
| walker |  | 9960 | 14 | python method at src/requests/models.py:405 |  |  | 0.513 |
| walker |  | 9968 | 8 | python method at src/requests/models.py:471 |  |  | 0.509 |
| ns | 9968 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.509 |
| walker |  | 9980 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.509 |
| walker |  | 9993 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.509 |
| ns | 9997 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.508 |
