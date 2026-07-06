Score(3000)=0.652 I=0.878 C=0.484 ns_rows≤3K=17/41 (reached=9 partial=0 missing=8)

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
| walker |  | 368 | 76 | [dependencies] in pyproject.toml |  |  | 0.494 |
| ns | 379 |  | 84 | .github/ and workflows/ listing | 1.5 |  | 0.433 |
| walker |  | 422 | 54 | [package] in pyproject.toml |  |  | 0.434 |
| ns | 554 |  | 175 | README lede + usage example | 1.6 |  | 0.524 |
| walker |  | 569 | 147 | README.md section #0 |  |  | 0.524 |
| walker |  | 659 | 90 | listing of 'src/requests' |  |  | 0.701 |
| ns | 754 |  | 200 | pyproject.toml identity | 1.7 |  | 0.647 |
| ns | 839 |  | 85 | pyproject.toml Python-version support + dependencies | 1.8 | 1.7 | 0.662 |
| ns | 1027 |  | 188 | Makefile build/test targets | 1.9 | 1.7 | 0.619 |
| ns | 1257 |  | 230 | __version__.py package metadata | 1.10 |  | 0.586 |
| ns | 1331 |  | 74 | api.py function locations | 2.1 |  | 0.569 |
| ns | 1561 |  | 230 | __init__.py public export tuple | 2.2 |  | 0.520 |
| walker |  | 1661 | 1002 | python imports in src/requests/__init__.py |  |  | 0.628 |
| walker |  | 1695 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.628 |
| walker |  | 1711 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.628 |
| walker |  | 1745 | 34 | listing of 'ext' |  |  | 0.628 |
| walker |  | 1792 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.628 |
| ns | 1828 |  | 267 | exceptions.py class hierarchy locations | 3.1 |  | 0.584 |
| walker |  | 1846 | 54 | listing of 'docs' |  |  | 0.637 |
| walker |  | 1867 | 21 | listing of 'docs/user' |  |  | 0.637 |
| walker |  | 1887 | 20 | python imports in setup.py |  |  | 0.637 |
| walker |  | 1950 | 63 | README.md section #1 |  |  | 0.637 |
| walker |  | 1988 | 38 | listing of 'docs/community' |  |  | 0.637 |
| walker |  | 2002 | 14 | listing of 'docs/_themes' |  |  | 0.637 |
| walker |  | 2006 | 4 | listing of 'docs/_templates' |  |  | 0.637 |
| ns | 2310 |  | 482 | hooks.py full (event hook dispatch) | 3.2 |  | 0.566 |
| walker |  | 2362 | 356 | plaintext config Makefile |  |  | 0.612 |
| walker |  | 2371 | 9 | listing of 'docs/_static' |  |  | 0.612 |
| walker |  | 2407 | 36 | python decl names surface in src/requests/help.py |  |  | 0.612 |
| walker |  | 2407 | 0 | python decl at src/requests/help.py:37 |  |  | 0.612 |
| walker |  | 2407 | 0 | python decl at src/requests/help.py:69 |  |  | 0.612 |
| walker |  | 2407 | 0 | python decl at src/requests/help.py:128 |  |  | 0.612 |
| ns | 2419 |  | 109 | structures.py class/method locations | 3.3 |  | 0.595 |
| walker |  | 2420 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.595 |
| walker |  | 2434 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.595 |
| walker |  | 2452 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.595 |
| walker |  | 2495 | 43 | listing of '.github' |  |  | 0.611 |
| walker |  | 2536 | 41 | listing of '.github/workflows' |  |  | 0.649 |
| walker |  | 2613 | 77 | listing of 'tests' |  |  | 0.704 |
| walker |  | 2623 | 10 | listing of 'tests/testserver' |  |  | 0.704 |
| walker |  | 2637 | 14 | listing of 'tests/certs' |  |  | 0.704 |
| walker |  | 2647 | 10 | listing of 'docs/dev' |  |  | 0.704 |
| walker |  | 2697 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.704 |
| walker |  | 2697 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.704 |
| walker |  | 2723 | 26 | python imports in src/requests/packages.py |  |  | 0.704 |
| walker |  | 2780 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.706 |
| walker |  | 2780 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.706 |
| walker |  | 2797 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.706 |
| walker |  | 2850 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.711 |
| walker |  | 2868 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.713 |
| ns | 2888 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.657 |
| walker |  | 2925 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.658 |
| walker |  | 2925 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.658 |
| walker |  | 2925 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.658 |
| walker |  | 2925 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.658 |
| walker |  | 2933 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.658 |
| ns | 2940 |  | 52 | models.py class roster | 4.1 |  | 0.652 |
| walker |  | 2964 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.652 |
| walker |  | 2996 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.652 |
| walker |  | 3049 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.652 |
| ns | 3089 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.638 |
| walker |  | 3322 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.638 |
| ns | 3345 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.615 |
| walker |  | 3379 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.615 |
| ns | 3399 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.619 |
| walker |  | 3404 | 25 | python decl names surface #1 in src/requests/cookies.py |  |  | 0.619 |
| walker |  | 3471 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.619 |
| ns | 3477 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.613 |
| walker |  | 3487 | 16 | python decl names surface #2 in src/requests/exceptions.py |  |  | 0.613 |
| walker |  | 3487 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.613 |
| walker |  | 3501 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.613 |
| walker |  | 3515 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.613 |
| ns | 3683 |  | 206 | Session method locations | 5.4 | 5.1 | 0.595 |
| walker |  | 3691 | 176 | README.md section #2 |  |  | 0.595 |
| walker |  | 3892 | 201 | python method sigs in src/requests/sessions.py |  |  | 0.612 |
| walker |  | 3892 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.612 |
| walker |  | 3892 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.612 |
| walker |  | 3892 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.612 |
| walker |  | 3892 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.612 |
| walker |  | 3892 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.612 |
| walker |  | 3892 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.612 |
| walker |  | 3892 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.612 |
| walker |  | 3898 | 6 | python method body at src/requests/sessions.py:505 body 506 |  |  | 0.612 |
| walker |  | 3904 | 6 | python method body at src/requests/sessions.py:508 body 509 |  |  | 0.612 |
| walker |  | 3930 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.612 |
| walker |  | 3956 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.612 |
| walker |  | 3974 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.612 |
| walker |  | 3994 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.612 |
| walker |  | 4039 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.612 |
| walker |  | 4089 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.612 |
| walker |  | 4128 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.612 |
| walker |  | 4201 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.614 |
| walker |  | 4201 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.614 |
| walker |  | 4201 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.614 |
| walker |  | 4212 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.614 |
| walker |  | 4220 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.614 |
| walker |  | 4237 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.614 |
| walker |  | 4269 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.614 |
| ns | 4408 |  | 725 | tests/conftest.py full | 6.1 |  | 0.560 |
| walker |  | 4598 | 329 | python method sigs in src/requests/structures.py |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:64 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:67 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:70 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:73 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:76 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:80 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:89 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:92 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:101 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:105 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:123 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:126 |  |  | 0.582 |
| walker |  | 4598 | 0 | python method at src/requests/structures.py:129 |  |  | 0.582 |
| walker |  | 4616 | 18 | python method at src/requests/structures.py:118 |  |  | 0.582 |
| walker |  | 4625 | 9 | python method body at src/requests/structures.py:73 body 74 |  |  | 0.582 |
| walker |  | 4634 | 9 | python method body at src/requests/structures.py:92 body 93 |  |  | 0.582 |
| walker |  | 4644 | 10 | python method body at src/requests/structures.py:67 body 68 |  |  | 0.582 |
| walker |  | 4675 | 31 | python method at src/requests/structures.py:108 |  |  | 0.582 |
| walker |  | 4707 | 32 | python method at src/requests/structures.py:59 |  |  | 0.582 |
| walker |  | 4723 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.582 |
| ns | 4725 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.560 |
| walker |  | 4735 | 12 | python method body at src/requests/structures.py:64 body 65 |  |  | 0.560 |
| walker |  | 4747 | 12 | python method body at src/requests/structures.py:89 body 90 |  |  | 0.560 |
| walker |  | 4760 | 13 | python method body at src/requests/structures.py:105 body 106 |  |  | 0.560 |
| walker |  | 4773 | 13 | python method body at src/requests/structures.py:118 body 121 |  |  | 0.560 |
| walker |  | 4786 | 13 | python method body at src/requests/structures.py:129 body 130 |  |  | 0.560 |
| walker |  | 4801 | 15 | python method body at src/requests/structures.py:59 body 62 |  |  | 0.560 |
| walker |  | 4818 | 17 | python method body at src/requests/structures.py:101 body 102 |  |  | 0.560 |
| walker |  | 4872 | 54 | python method at src/requests/structures.py:49 |  |  | 0.560 |
| walker |  | 4892 | 20 | python method body at src/requests/structures.py:70 body 71 |  |  | 0.560 |
| walker |  | 4916 | 24 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.560 |
| ns | 4923 |  | 198 | test_structures.py locations | 6.3 |  | 0.549 |
| walker |  | 5005 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.549 |
| walker |  | 5005 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.549 |
| walker |  | 5005 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.549 |
| walker |  | 5016 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.549 |
| walker |  | 5031 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.549 |
| ns | 5152 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.538 |
| walker |  | 5185 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.538 |
| ns | 5361 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.530 |
| walker |  | 5470 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.532 |
| walker |  | 5470 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.532 |
| walker |  | 5477 | 7 | python method body at src/requests/adapters.py:125 body 126 |  |  | 0.532 |
| walker |  | 5484 | 7 | python method body at src/requests/adapters.py:599 body 611 |  |  | 0.532 |
| walker |  | 5497 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.532 |
| walker |  | 5505 | 8 | python method body at src/requests/adapters.py:153 body 155 |  |  | 0.532 |
| ns | 5513 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.524 |
| walker |  | 5536 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.524 |
| walker |  | 5570 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.524 |
| ns | 5608 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.520 |
| walker |  | 5609 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.520 |
| ns | 5622 |  | 14 | tests/certs/ directory listing | 6.8 |  | 0.524 |
| walker |  | 5649 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.524 |
| walker |  | 5697 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.524 |
| walker |  | 5756 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.524 |
| walker |  | 5777 | 21 | python method body at src/requests/adapters.py:403 body 453 |  |  | 0.524 |
| walker |  | 5799 | 22 | python method body at src/requests/adapters.py:223 body 224 |  |  | 0.524 |
| walker |  | 5871 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.524 |
| walker |  | 5947 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.524 |
| walker |  | 6104 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.524 |
| walker |  | 6104 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.524 |
| walker |  | 6116 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.524 |
| walker |  | 6143 | 27 | python method body at src/requests/structures.py:76 body 78 |  |  | 0.524 |
| walker |  | 6170 | 27 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.524 |
| walker |  | 6223 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.524 |
| walker |  | 6279 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.524 |
| walker |  | 6425 | 146 | python decl names surface in src/requests/models.py |  |  | 0.531 |
| walker |  | 6425 | 0 | python decl at src/requests/models.py:109 |  |  | 0.531 |
| walker |  | 6425 | 0 | python decl at src/requests/models.py:255 |  |  | 0.531 |
| walker |  | 6425 | 0 | python decl at src/requests/models.py:283 |  |  | 0.531 |
| walker |  | 6425 | 0 | python decl at src/requests/models.py:376 |  |  | 0.531 |
| walker |  | 6425 | 0 | python decl at src/requests/models.py:730 |  |  | 0.531 |
| walker |  | 6436 | 11 | python class body at src/requests/models.py:109 |  |  | 0.531 |
| walker |  | 6453 | 17 | python class body at src/requests/models.py:255 |  |  | 0.531 |
| walker |  | 6471 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.531 |
| walker |  | 6510 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.531 |
| walker |  | 6769 | 259 | python class body at src/requests/models.py:730 |  |  | 0.531 |
| walker |  | 6892 | 123 | python class body at src/requests/models.py:283 |  |  | 0.531 |
| walker |  | 6991 | 99 | python class body at src/requests/models.py:376 |  |  | 0.531 |
| ns | 7120 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.475 |
| walker |  | 7140 | 149 | python method sigs in src/requests/models.py |  |  | 0.475 |
| walker |  | 7140 | 0 | python method at src/requests/models.py:271 |  |  | 0.475 |
| walker |  | 7140 | 0 | python method at src/requests/models.py:355 |  |  | 0.475 |
| walker |  | 7140 | 0 | python method at src/requests/models.py:358 |  |  | 0.475 |
| walker |  | 7153 | 13 | python method at src/requests/models.py:112 |  |  | 0.475 |
| walker |  | 7161 | 8 | python method at src/requests/models.py:133 |  |  | 0.475 |
| walker |  | 7169 | 8 | python method at src/requests/models.py:137 |  |  | 0.475 |
| walker |  | 7177 | 8 | python method at src/requests/models.py:147 |  |  | 0.475 |
| walker |  | 7190 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.475 |
| walker |  | 7202 | 12 | python method body at src/requests/models.py:355 body 356 |  |  | 0.475 |
| walker |  | 7238 | 36 | python method at src/requests/models.py:258 |  |  | 0.475 |
| walker |  | 7250 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.475 |
| walker |  | 7274 | 24 | python method doc at src/requests/models.py:358 |  |  | 0.475 |
| walker |  | 7309 | 35 | python method doc at src/requests/models.py:271 |  |  | 0.475 |
| walker |  | 7383 | 74 | python decl at src/requests/models.py:96 |  |  | 0.475 |
| ns | 7401 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.491 |
| walker |  | 7417 | 34 | python method at src/requests/models.py:141 |  |  | 0.491 |
| walker |  | 7551 | 134 | python decl names surface in src/requests/api.py |  |  | 0.502 |
| walker |  | 7551 | 0 | python decl at src/requests/api.py:90 |  |  | 0.502 |
| walker |  | 7551 | 0 | python decl at src/requests/api.py:102 |  |  | 0.502 |
| walker |  | 7551 | 0 | python decl at src/requests/api.py:171 |  |  | 0.502 |
| walker |  | 7586 | 35 | python decl at src/requests/api.py:24 |  |  | 0.502 |
| walker |  | 7601 | 15 | python decl body at src/requests/api.py:90 body 99 |  |  | 0.502 |
| walker |  | 7616 | 15 | python decl body at src/requests/api.py:171 body 180 |  |  | 0.502 |
| walker |  | 7635 | 19 | python decl doc at src/requests/api.py:24 |  |  | 0.502 |
| walker |  | 7675 | 40 | python decl at src/requests/api.py:74 |  |  | 0.502 |
| walker |  | 7715 | 40 | python decl at src/requests/api.py:137 |  |  | 0.502 |
| walker |  | 7755 | 40 | python decl at src/requests/api.py:154 |  |  | 0.502 |
| walker |  | 7773 | 18 | python decl body at src/requests/api.py:74 body 87 |  |  | 0.502 |
| walker |  | 7791 | 18 | python decl body at src/requests/api.py:137 body 151 |  |  | 0.502 |
| walker |  | 7809 | 18 | python decl body at src/requests/api.py:154 body 168 |  |  | 0.502 |
| walker |  | 7873 | 64 | python decl at src/requests/api.py:117 |  |  | 0.502 |
| walker |  | 7894 | 21 | python decl body at src/requests/api.py:117 body 134 |  |  | 0.502 |
| walker |  | 7984 | 90 | python decl doc at src/requests/api.py:90 |  |  | 0.502 |
| walker |  | 8074 | 90 | python decl doc at src/requests/api.py:171 |  |  | 0.502 |
| walker |  | 8089 | 15 | python method body at src/requests/models.py:133 body 135 |  |  | 0.502 |
| walker |  | 8104 | 15 | python method body at src/requests/models.py:137 body 139 |  |  | 0.502 |
| walker |  | 8119 | 15 | python method body at src/requests/models.py:141 body 145 |  |  | 0.502 |
| walker |  | 8216 | 97 | python method at src/requests/adapters.py:128 |  |  | 0.502 |
| walker |  | 8226 | 10 | python method body at src/requests/adapters.py:128 body 151 |  |  | 0.502 |
| walker |  | 8323 | 97 | python method at src/requests/adapters.py:634 |  |  | 0.502 |
| walker |  | 8479 | 156 | python decl names surface in src/requests/exceptions.py |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.511 |
| walker |  | 8479 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.511 |
| walker |  | 8488 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.511 |
| walker |  | 8497 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.511 |
| walker |  | 8506 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.511 |
| walker |  | 8515 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.511 |
| walker |  | 8524 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.511 |
| walker |  | 8534 | 10 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.511 |
| walker |  | 8547 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.511 |
| walker |  | 8561 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.511 |
| walker |  | 8579 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.511 |
| walker |  | 8606 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.511 |
| walker |  | 8634 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.511 |
| walker |  | 8677 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.511 |
| walker |  | 8747 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.511 |
| walker |  | 8747 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.511 |
| walker |  | 8747 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.511 |
| walker |  | 8747 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.511 |
| walker |  | 8766 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.511 |
| ns | 8775 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.469 |
| walker |  | 8781 | 15 | python method body at src/requests/exceptions.py:55 body 63 |  |  | 0.469 |
| walker |  | 8846 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.469 |
| walker |  | 8972 | 126 | python decl names surface in src/requests/auth.py |  |  | 0.469 |
| walker |  | 8972 | 0 | python decl at src/requests/auth.py:34 |  |  | 0.469 |
| walker |  | 8972 | 0 | python decl at src/requests/auth.py:78 |  |  | 0.469 |
| walker |  | 8972 | 0 | python decl at src/requests/auth.py:85 |  |  | 0.469 |
| walker |  | 8972 | 0 | python decl at src/requests/auth.py:116 |  |  | 0.469 |
| walker |  | 8972 | 0 | python decl at src/requests/auth.py:124 |  |  | 0.469 |
| walker |  | 8987 | 15 | python decl doc at src/requests/auth.py:78 |  |  | 0.469 |
| walker |  | 9004 | 17 | python decl doc at src/requests/auth.py:85 |  |  | 0.469 |
| walker |  | 9026 | 22 | python class body at src/requests/auth.py:85 |  |  | 0.469 |
| walker |  | 9043 | 17 | python decl doc at src/requests/auth.py:116 |  |  | 0.469 |
| walker |  | 9060 | 17 | python decl doc at src/requests/auth.py:124 |  |  | 0.469 |
| walker |  | 9072 | 12 | python decl doc at src/requests/auth.py:34 |  |  | 0.469 |
| ns | 9110 |  | 335 | auth.py class/method locations | 8.1 |  | 0.464 |
| walker |  | 9163 | 91 | python class body at src/requests/auth.py:124 |  |  | 0.464 |
| ns | 9322 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.460 |
| ns | 9350 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.459 |
| ns | 9427 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.458 |
| walker |  | 9515 | 352 | python method sigs in src/requests/auth.py |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:81 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:91 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:93 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:96 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:100 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:108 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:111 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:119 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:136 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:138 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:141 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:157 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:268 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:273 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:345 |  |  | 0.476 |
| walker |  | 9515 | 0 | python method at src/requests/auth.py:353 |  |  | 0.476 |
| walker |  | 9529 | 14 | python method at src/requests/auth.py:321 |  |  | 0.476 |
| walker |  | 9544 | 15 | python method at src/requests/auth.py:147 |  |  | 0.476 |
| walker |  | 9553 | 9 | python method body at src/requests/auth.py:108 body 109 |  |  | 0.476 |
| walker |  | 9562 | 9 | python method body at src/requests/auth.py:353 body 354 |  |  | 0.476 |
| walker |  | 9577 | 15 | python method doc at src/requests/auth.py:268 |  |  | 0.476 |
| walker |  | 9591 | 14 | python method body at src/requests/auth.py:81 body 82 |  |  | 0.476 |
| walker |  | 9613 | 22 | python method doc at src/requests/auth.py:157 |  |  | 0.476 |
| walker |  | 9631 | 18 | python method body at src/requests/auth.py:96 body 97 |  |  | 0.476 |
| walker |  | 9652 | 21 | python method body at src/requests/auth.py:91 body 92 |  |  | 0.476 |
| walker |  | 9673 | 21 | python method body at src/requests/auth.py:93 body 94 |  |  | 0.476 |
| walker |  | 9694 | 21 | python method body at src/requests/auth.py:136 body 137 |  |  | 0.476 |
| walker |  | 9715 | 21 | python method body at src/requests/auth.py:138 body 139 |  |  | 0.476 |
| walker |  | 9760 | 45 | python method doc at src/requests/auth.py:273 |  |  | 0.476 |
| walker |  | 9785 | 25 | python method body at src/requests/auth.py:268 body 270 |  |  | 0.476 |
| ns | 9801 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.466 |
| walker |  | 9812 | 27 | python method body at src/requests/auth.py:111 body 112 |  |  | 0.466 |
| walker |  | 9841 | 29 | python method body at src/requests/auth.py:119 body 120 |  |  | 0.466 |
| ns | 9862 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.464 |
| ns | 9968 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.460 |
| ns | 9997 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.459 |
