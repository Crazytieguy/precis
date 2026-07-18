Score(3000)=0.643 I=0.875 C=0.473 ns_rows≤3K=17/41 (reached=9 partial=0 missing=8)

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
| walker |  | 326 | 34 | listing of 'ext' |  |  | 0.492 |
| ns | 379 |  | 84 | .github/ and workflows/ listing | 1.5 |  | 0.431 |
| walker |  | 380 | 54 | listing of 'docs' |  |  | 0.563 |
| walker |  | 384 | 4 | listing of 'docs/_templates' |  |  | 0.563 |
| walker |  | 393 | 9 | listing of 'docs/_static' |  |  | 0.563 |
| walker |  | 403 | 10 | listing of 'docs/dev' |  |  | 0.563 |
| walker |  | 417 | 14 | listing of 'docs/_themes' |  |  | 0.563 |
| walker |  | 438 | 21 | listing of 'docs/user' |  |  | 0.563 |
| walker |  | 496 | 58 | [package] in pyproject.toml |  |  | 0.563 |
| ns | 554 |  | 175 | README lede + usage example | 1.6 |  | 0.619 |
| walker |  | 637 | 141 | [dependencies] in pyproject.toml |  |  | 0.624 |
| walker |  | 727 | 90 | listing of 'src/requests' |  |  | 0.793 |
| ns | 754 |  | 200 | pyproject.toml identity | 1.7 |  | 0.731 |
| ns | 839 |  | 85 | pyproject.toml Python-version support + dependencies | 1.8 | 1.7 | 0.741 |
| ns | 1027 |  | 188 | Makefile build/test targets | 1.9 | 1.7 | 0.693 |
| ns | 1257 |  | 230 | __version__.py package metadata | 1.10 |  | 0.656 |
| ns | 1331 |  | 74 | api.py function locations | 2.1 |  | 0.636 |
| ns | 1561 |  | 230 | __init__.py public export tuple | 2.2 |  | 0.582 |
| walker |  | 1729 | 1002 | python imports in src/requests/__init__.py |  |  | 0.685 |
| walker |  | 1763 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.685 |
| walker |  | 1779 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.685 |
| walker |  | 1817 | 38 | listing of 'docs/community' |  |  | 0.685 |
| ns | 1828 |  | 267 | exceptions.py class hierarchy locations | 3.1 |  | 0.637 |
| walker |  | 1864 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.637 |
| walker |  | 1884 | 20 | python imports in setup.py |  |  | 0.637 |
| walker |  | 1947 | 63 | README.md section #2 |  |  | 0.637 |
| walker |  | 1990 | 43 | listing of '.github' |  |  | 0.655 |
| walker |  | 2031 | 41 | listing of '.github/workflows' |  |  | 0.701 |
| walker |  | 2045 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.701 |
| ns | 2310 |  | 482 | hooks.py full (event hook dispatch) | 3.2 |  | 0.624 |
| walker |  | 2401 | 356 | plaintext config Makefile |  |  | 0.668 |
| ns | 2419 |  | 109 | structures.py class/method locations | 3.3 |  | 0.649 |
| walker |  | 2440 | 39 | plaintext config docs/requirements.txt |  |  | 0.649 |
| walker |  | 2517 | 77 | listing of 'tests' |  |  | 0.704 |
| walker |  | 2527 | 10 | listing of 'tests/testserver' |  |  | 0.704 |
| walker |  | 2541 | 14 | listing of 'tests/certs' |  |  | 0.704 |
| walker |  | 2794 | 253 | python decl names surface in src/requests/utils.py |  |  | 0.705 |
| ns | 2888 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.649 |
| ns | 2940 |  | 52 | models.py class roster | 4.1 |  | 0.643 |
| walker |  | 3050 | 256 | README.md section #4 |  |  | 0.643 |
| walker |  | 3086 | 36 | python decl names surface in src/requests/help.py |  |  | 0.643 |
| walker |  | 3086 | 0 | python decl at src/requests/help.py:37 |  |  | 0.643 |
| walker |  | 3086 | 0 | python decl at src/requests/help.py:69 |  |  | 0.643 |
| walker |  | 3086 | 0 | python decl at src/requests/help.py:128 |  |  | 0.643 |
| ns | 3089 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.629 |
| walker |  | 3099 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.629 |
| walker |  | 3113 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.629 |
| walker |  | 3131 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.629 |
| walker |  | 3181 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.629 |
| walker |  | 3181 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.629 |
| walker |  | 3328 | 147 | README.md section #1 |  |  | 0.629 |
| ns | 3345 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.607 |
| walker |  | 3354 | 26 | python imports in src/requests/packages.py |  |  | 0.607 |
| ns | 3399 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.602 |
| walker |  | 3411 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.603 |
| walker |  | 3411 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.603 |
| walker |  | 3428 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.604 |
| ns | 3477 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.597 |
| walker |  | 3481 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.601 |
| walker |  | 3499 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.603 |
| walker |  | 3556 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.613 |
| walker |  | 3556 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.613 |
| walker |  | 3556 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.613 |
| walker |  | 3556 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.613 |
| walker |  | 3564 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.613 |
| walker |  | 3595 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.613 |
| walker |  | 3627 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.613 |
| walker |  | 3680 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.613 |
| ns | 3683 |  | 206 | Session method locations | 5.4 | 5.1 | 0.595 |
| walker |  | 3953 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.595 |
| ns | 4408 |  | 725 | tests/conftest.py full | 6.1 |  | 0.543 |
| walker |  | 4427 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.589 |
| walker |  | 4427 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.589 |
| walker |  | 4453 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.589 |
| walker |  | 4479 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.589 |
| walker |  | 4495 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.589 |
| walker |  | 4513 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.589 |
| walker |  | 4533 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.589 |
| walker |  | 4576 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.589 |
| walker |  | 4619 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.589 |
| walker |  | 4664 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.589 |
| walker |  | 4714 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.589 |
| ns | 4725 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.567 |
| walker |  | 4772 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.567 |
| walker |  | 4829 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.567 |
| ns | 4923 |  | 198 | test_structures.py locations | 6.3 |  | 0.556 |
| walker |  | 5005 | 176 | README.md section #3 |  |  | 0.556 |
| walker |  | 5039 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.556 |
| walker |  | 5106 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.556 |
| ns | 5152 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.545 |
| ns | 5361 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.537 |
| walker |  | 5379 | 273 | python decl names surface #1 in src/requests/utils.py |  |  | 0.537 |
| walker |  | 5418 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.537 |
| ns | 5513 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.530 |
| ns | 5608 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.525 |
| ns | 5622 |  | 14 | tests/certs/ directory listing | 6.8 |  | 0.529 |
| walker |  | 6008 | 590 | manifest config in pyproject.toml |  |  | 0.532 |
| walker |  | 6049 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.532 |
| walker |  | 6121 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.532 |
| walker |  | 6194 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.534 |
| walker |  | 6194 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.534 |
| walker |  | 6194 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.534 |
| walker |  | 6205 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.534 |
| walker |  | 6213 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.534 |
| walker |  | 6230 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.534 |
| walker |  | 6262 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.534 |
| walker |  | 6631 | 369 | python method sigs in src/requests/structures.py |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:64 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:67 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:70 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:73 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:80 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:89 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:92 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:101 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:105 |  |  | 0.552 |
| walker |  | 6631 | 0 | python method at src/requests/structures.py:129 |  |  | 0.552 |
| walker |  | 6638 | 7 | python method at src/requests/structures.py:126 |  |  | 0.552 |
| walker |  | 6638 | 0 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.552 |
| walker |  | 6647 | 9 | python method at src/requests/structures.py:123 |  |  | 0.552 |
| walker |  | 6647 | 0 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.552 |
| walker |  | 6665 | 18 | python method at src/requests/structures.py:118 |  |  | 0.552 |
| walker |  | 6696 | 31 | python method at src/requests/structures.py:108 |  |  | 0.552 |
| walker |  | 6728 | 32 | python method at src/requests/structures.py:59 |  |  | 0.552 |
| walker |  | 6744 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 6798 | 54 | python method at src/requests/structures.py:49 |  |  | 0.552 |
| walker |  | 6887 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.552 |
| walker |  | 6887 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6887 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 6898 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 6913 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.552 |
| walker |  | 7067 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.552 |
| ns | 7120 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.494 |
| walker |  | 7352 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.495 |
| walker |  | 7352 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.495 |
| walker |  | 7365 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.495 |
| walker |  | 7396 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.495 |
| ns | 7401 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.510 |
| walker |  | 7430 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.510 |
| walker |  | 7469 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.510 |
| walker |  | 7509 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.510 |
| walker |  | 7557 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.510 |
| walker |  | 7616 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.510 |
| walker |  | 7688 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.510 |
| walker |  | 7733 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.510 |
| walker |  | 7809 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.510 |
| walker |  | 7966 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.510 |
| walker |  | 7966 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.510 |
| walker |  | 7978 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.510 |
| walker |  | 8059 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.510 |
| walker |  | 8101 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.510 |
| walker |  | 8154 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.510 |
| walker |  | 8507 | 353 | python decl names surface in src/requests/exceptions.py |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:110 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:157 |  |  | 0.543 |
| walker |  | 8507 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8515 | 8 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.543 |
| walker |  | 8524 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.543 |
| walker |  | 8533 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.543 |
| walker |  | 8542 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.543 |
| walker |  | 8551 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.543 |
| walker |  | 8560 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.543 |
| walker |  | 8569 | 9 | python decl doc at src/requests/exceptions.py:142 |  |  | 0.543 |
| walker |  | 8578 | 9 | python decl doc at src/requests/exceptions.py:153 |  |  | 0.543 |
| walker |  | 8588 | 10 | python decl doc at src/requests/exceptions.py:134 |  |  | 0.543 |
| walker |  | 8599 | 11 | python decl doc at src/requests/exceptions.py:118 |  |  | 0.543 |
| walker |  | 8610 | 11 | python decl doc at src/requests/exceptions.py:126 |  |  | 0.543 |
| walker |  | 8622 | 12 | python decl doc at src/requests/exceptions.py:122 |  |  | 0.543 |
| walker |  | 8635 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.543 |
| walker |  | 8648 | 13 | python decl doc at src/requests/exceptions.py:138 |  |  | 0.543 |
| walker |  | 8662 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.543 |
| walker |  | 8676 | 14 | python decl doc at src/requests/exceptions.py:114 |  |  | 0.543 |
| walker |  | 8690 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.543 |
| walker |  | 8706 | 16 | python decl doc at src/requests/exceptions.py:130 |  |  | 0.543 |
| walker |  | 8723 | 17 | python decl doc at src/requests/exceptions.py:146 |  |  | 0.543 |
| walker |  | 8741 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.543 |
| walker |  | 8759 | 18 | python decl doc at src/requests/exceptions.py:110 |  |  | 0.543 |
| ns | 8775 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.498 |
| walker |  | 8778 | 19 | python decl doc at src/requests/exceptions.py:157 |  |  | 0.498 |
| walker |  | 8805 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.498 |
| walker |  | 8833 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.498 |
| walker |  | 8903 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.498 |
| walker |  | 8903 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.498 |
| walker |  | 8903 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.498 |
| walker |  | 8903 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.498 |
| walker |  | 8922 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.498 |
| walker |  | 8965 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.498 |
| walker |  | 9030 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.498 |
| walker |  | 9086 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.498 |
| ns | 9110 |  | 335 | auth.py class/method locations | 8.1 |  | 0.492 |
| walker |  | 9232 | 146 | python decl names surface in src/requests/models.py |  |  | 0.497 |
| walker |  | 9232 | 0 | python decl at src/requests/models.py:109 |  |  | 0.497 |
| walker |  | 9232 | 0 | python decl at src/requests/models.py:255 |  |  | 0.497 |
| walker |  | 9232 | 0 | python decl at src/requests/models.py:283 |  |  | 0.497 |
| walker |  | 9232 | 0 | python decl at src/requests/models.py:376 |  |  | 0.497 |
| walker |  | 9232 | 0 | python decl at src/requests/models.py:730 |  |  | 0.497 |
| walker |  | 9243 | 11 | python class body at src/requests/models.py:109 |  |  | 0.497 |
| walker |  | 9260 | 17 | python class body at src/requests/models.py:255 |  |  | 0.497 |
| walker |  | 9278 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.497 |
| walker |  | 9317 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.497 |
| ns | 9322 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.493 |
| ns | 9350 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.492 |
| ns | 9427 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.496 |
| walker |  | 9576 | 259 | python class body at src/requests/models.py:730 |  |  | 0.496 |
| ns | 9801 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.485 |
| ns | 9862 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.483 |
| ns | 9968 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.479 |
| ns | 9997 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.478 |
