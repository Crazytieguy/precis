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
| walker |  | 2084 | 9 | listing of 'docs/_static' |  |  | 0.637 |
| walker |  | 2120 | 36 | python decl names surface in src/requests/help.py |  |  | 0.637 |
| walker |  | 2120 | 0 | python decl at src/requests/help.py:37 |  |  | 0.637 |
| walker |  | 2120 | 0 | python decl at src/requests/help.py:69 |  |  | 0.637 |
| walker |  | 2120 | 0 | python decl at src/requests/help.py:128 |  |  | 0.637 |
| walker |  | 2133 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.637 |
| walker |  | 2147 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.637 |
| walker |  | 2165 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.637 |
| walker |  | 2208 | 43 | listing of '.github' |  |  | 0.655 |
| walker |  | 2249 | 41 | listing of '.github/workflows' |  |  | 0.701 |
| ns | 2310 |  | 482 | hooks.py full (event hook dispatch) | 3.2 |  | 0.624 |
| walker |  | 2326 | 77 | listing of 'tests' |  |  | 0.680 |
| walker |  | 2336 | 10 | listing of 'tests/testserver' |  |  | 0.680 |
| walker |  | 2350 | 14 | listing of 'tests/certs' |  |  | 0.681 |
| walker |  | 2360 | 10 | listing of 'docs/dev' |  |  | 0.681 |
| walker |  | 2410 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.681 |
| walker |  | 2410 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.681 |
| ns | 2419 |  | 109 | structures.py class/method locations | 3.3 |  | 0.662 |
| walker |  | 2436 | 26 | python imports in src/requests/packages.py |  |  | 0.662 |
| walker |  | 2493 | 57 | python decl names surface in src/requests/hooks.py |  |  | 0.663 |
| walker |  | 2493 | 0 | python decl at src/requests/hooks.py:25 |  |  | 0.663 |
| walker |  | 2510 | 17 | python decl body at src/requests/hooks.py:25 body 26 |  |  | 0.664 |
| walker |  | 2563 | 53 | python decl at src/requests/hooks.py:32 |  |  | 0.669 |
| walker |  | 2581 | 18 | python decl doc at src/requests/hooks.py:32 |  |  | 0.671 |
| walker |  | 2638 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.672 |
| walker |  | 2638 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.672 |
| walker |  | 2638 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.672 |
| walker |  | 2638 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.672 |
| walker |  | 2646 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.672 |
| walker |  | 2677 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.672 |
| walker |  | 2709 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.672 |
| walker |  | 2762 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.672 |
| ns | 2888 |  | 469 | status_codes.py docstring + init logic | 3.4 |  | 0.619 |
| ns | 2940 |  | 52 | models.py class roster | 4.1 |  | 0.614 |
| walker |  | 3035 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.614 |
| ns | 3089 |  | 149 | PreparedRequest method locations | 4.2 | 4.1 | 0.600 |
| walker |  | 3092 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.600 |
| walker |  | 3117 | 25 | python decl names surface #1 in src/requests/cookies.py |  |  | 0.600 |
| walker |  | 3184 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.600 |
| walker |  | 3200 | 16 | python decl names surface #2 in src/requests/exceptions.py |  |  | 0.600 |
| walker |  | 3200 | 0 | python decl at src/requests/exceptions.py:161 |  |  | 0.600 |
| walker |  | 3214 | 14 | python decl doc at src/requests/exceptions.py:161 |  |  | 0.600 |
| walker |  | 3228 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.600 |
| ns | 3345 |  | 256 | Response method/property locations | 4.3 | 4.1 | 0.579 |
| ns | 3399 |  | 54 | sessions.py module-level roster | 5.1 |  | 0.584 |
| walker |  | 3404 | 176 | README.md section #2 |  |  | 0.584 |
| ns | 3477 |  | 78 | SessionRedirectMixin method locations | 5.2 | 5.1 | 0.578 |
| walker |  | 3605 | 201 | python method sigs in src/requests/sessions.py |  |  | 0.593 |
| walker |  | 3605 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.593 |
| walker |  | 3605 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.593 |
| walker |  | 3605 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.593 |
| walker |  | 3605 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.593 |
| walker |  | 3605 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.593 |
| walker |  | 3605 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.593 |
| walker |  | 3605 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.593 |
| walker |  | 3611 | 6 | python method body at src/requests/sessions.py:505 body 506 |  |  | 0.593 |
| walker |  | 3617 | 6 | python method body at src/requests/sessions.py:508 body 509 |  |  | 0.593 |
| walker |  | 3643 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.593 |
| walker |  | 3669 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.593 |
| ns | 3683 |  | 206 | Session method locations | 5.4 | 5.1 | 0.578 |
| walker |  | 3687 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.578 |
| walker |  | 3707 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.578 |
| walker |  | 3752 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.578 |
| walker |  | 3802 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.578 |
| walker |  | 3841 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.578 |
| walker |  | 3914 | 73 | python decl names surface in src/requests/structures.py |  |  | 0.580 |
| walker |  | 3914 | 0 | python decl at src/requests/structures.py:20 |  |  | 0.580 |
| walker |  | 3914 | 0 | python decl at src/requests/structures.py:96 |  |  | 0.580 |
| walker |  | 3925 | 11 | python class body at src/requests/structures.py:96 |  |  | 0.580 |
| walker |  | 3933 | 8 | python decl doc at src/requests/structures.py:96 |  |  | 0.580 |
| walker |  | 3950 | 17 | python decl doc at src/requests/structures.py:20 |  |  | 0.580 |
| walker |  | 3982 | 32 | python class body at src/requests/structures.py:20 |  |  | 0.580 |
| walker |  | 4311 | 329 | python method sigs in src/requests/structures.py |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:64 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:67 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:70 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:73 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:76 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:80 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:89 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:92 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:101 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:105 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:123 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:126 |  |  | 0.605 |
| walker |  | 4311 | 0 | python method at src/requests/structures.py:129 |  |  | 0.605 |
| walker |  | 4329 | 18 | python method at src/requests/structures.py:118 |  |  | 0.605 |
| walker |  | 4338 | 9 | python method body at src/requests/structures.py:73 body 74 |  |  | 0.605 |
| walker |  | 4347 | 9 | python method body at src/requests/structures.py:92 body 93 |  |  | 0.605 |
| walker |  | 4357 | 10 | python method body at src/requests/structures.py:67 body 68 |  |  | 0.605 |
| walker |  | 4388 | 31 | python method at src/requests/structures.py:108 |  |  | 0.605 |
| ns | 4408 |  | 725 | tests/conftest.py full | 6.1 |  | 0.552 |
| walker |  | 4420 | 32 | python method at src/requests/structures.py:59 |  |  | 0.552 |
| walker |  | 4436 | 16 | python method doc at src/requests/structures.py:76 |  |  | 0.552 |
| walker |  | 4448 | 12 | python method body at src/requests/structures.py:64 body 65 |  |  | 0.552 |
| walker |  | 4460 | 12 | python method body at src/requests/structures.py:89 body 90 |  |  | 0.552 |
| walker |  | 4473 | 13 | python method body at src/requests/structures.py:105 body 106 |  |  | 0.552 |
| walker |  | 4486 | 13 | python method body at src/requests/structures.py:118 body 121 |  |  | 0.552 |
| walker |  | 4499 | 13 | python method body at src/requests/structures.py:129 body 130 |  |  | 0.552 |
| walker |  | 4514 | 15 | python method body at src/requests/structures.py:59 body 62 |  |  | 0.552 |
| walker |  | 4531 | 17 | python method body at src/requests/structures.py:101 body 102 |  |  | 0.552 |
| walker |  | 4585 | 54 | python method at src/requests/structures.py:49 |  |  | 0.552 |
| walker |  | 4605 | 20 | python method body at src/requests/structures.py:70 body 71 |  |  | 0.552 |
| walker |  | 4629 | 24 | python method body at src/requests/structures.py:123 body 124 |  |  | 0.552 |
| walker |  | 4718 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.552 |
| walker |  | 4718 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.552 |
| walker |  | 4718 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.552 |
| ns | 4725 |  | 317 | tests/utils.py + tests/__init__.py full | 6.2 |  | 0.531 |
| walker |  | 4729 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.531 |
| walker |  | 4744 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.531 |
| walker |  | 4898 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.531 |
| ns | 4923 |  | 198 | test_structures.py locations | 6.3 |  | 0.521 |
| ns | 5152 |  | 229 | test_testserver.py + tests/testserver/server.py locations | 6.4 |  | 0.510 |
| walker |  | 5183 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.511 |
| walker |  | 5183 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.511 |
| walker |  | 5190 | 7 | python method body at src/requests/adapters.py:125 body 126 |  |  | 0.511 |
| walker |  | 5197 | 7 | python method body at src/requests/adapters.py:599 body 611 |  |  | 0.511 |
| walker |  | 5210 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.511 |
| walker |  | 5218 | 8 | python method body at src/requests/adapters.py:153 body 155 |  |  | 0.511 |
| walker |  | 5249 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.511 |
| walker |  | 5283 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.511 |
| walker |  | 5322 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.511 |
| ns | 5361 |  | 209 | test_lowlevel.py locations | 6.5 |  | 0.504 |
| walker |  | 5362 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.504 |
| walker |  | 5410 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.504 |
| walker |  | 5469 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.504 |
| walker |  | 5490 | 21 | python method body at src/requests/adapters.py:403 body 453 |  |  | 0.504 |
| walker |  | 5512 | 22 | python method body at src/requests/adapters.py:223 body 224 |  |  | 0.504 |
| ns | 5513 |  | 152 | test_utils.py class roster (sampled) | 6.6 |  | 0.497 |
| walker |  | 5584 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.497 |
| ns | 5608 |  | 95 | test_requests.py class roster (sampled) | 6.7 |  | 0.493 |
| ns | 5622 |  | 14 | tests/certs/ directory listing | 6.8 |  | 0.497 |
| walker |  | 5660 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.497 |
| walker |  | 5817 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.497 |
| walker |  | 5817 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.497 |
| walker |  | 5829 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.497 |
| walker |  | 5856 | 27 | python method body at src/requests/structures.py:76 body 78 |  |  | 0.497 |
| walker |  | 5883 | 27 | python method body at src/requests/structures.py:126 body 127 |  |  | 0.497 |
| walker |  | 6473 | 590 | manifest config in pyproject.toml |  |  | 0.501 |
| walker |  | 6526 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.501 |
| walker |  | 6582 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.501 |
| walker |  | 6728 | 146 | python decl names surface in src/requests/models.py |  |  | 0.509 |
| walker |  | 6728 | 0 | python decl at src/requests/models.py:109 |  |  | 0.509 |
| walker |  | 6728 | 0 | python decl at src/requests/models.py:255 |  |  | 0.509 |
| walker |  | 6728 | 0 | python decl at src/requests/models.py:283 |  |  | 0.509 |
| walker |  | 6728 | 0 | python decl at src/requests/models.py:376 |  |  | 0.509 |
| walker |  | 6728 | 0 | python decl at src/requests/models.py:730 |  |  | 0.509 |
| walker |  | 6739 | 11 | python class body at src/requests/models.py:109 |  |  | 0.509 |
| walker |  | 6756 | 17 | python class body at src/requests/models.py:255 |  |  | 0.509 |
| walker |  | 6774 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.509 |
| walker |  | 6813 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.509 |
| walker |  | 7072 | 259 | python class body at src/requests/models.py:730 |  |  | 0.509 |
| ns | 7120 |  | 1498 | resolve_redirects() full body | 6.9 | 5.2 | 0.455 |
| walker |  | 7195 | 123 | python class body at src/requests/models.py:283 |  |  | 0.455 |
| walker |  | 7294 | 99 | python class body at src/requests/models.py:376 |  |  | 0.455 |
| ns | 7401 |  | 281 | adapters.py class/method locations | 7.1 |  | 0.472 |
| walker |  | 7443 | 149 | python method sigs in src/requests/models.py |  |  | 0.472 |
| walker |  | 7443 | 0 | python method at src/requests/models.py:271 |  |  | 0.472 |
| walker |  | 7443 | 0 | python method at src/requests/models.py:355 |  |  | 0.472 |
| walker |  | 7443 | 0 | python method at src/requests/models.py:358 |  |  | 0.472 |
| walker |  | 7456 | 13 | python method at src/requests/models.py:112 |  |  | 0.472 |
| walker |  | 7464 | 8 | python method at src/requests/models.py:133 |  |  | 0.472 |
| walker |  | 7472 | 8 | python method at src/requests/models.py:137 |  |  | 0.472 |
| walker |  | 7480 | 8 | python method at src/requests/models.py:147 |  |  | 0.472 |
| walker |  | 7493 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.472 |
| walker |  | 7505 | 12 | python method body at src/requests/models.py:355 body 356 |  |  | 0.472 |
| walker |  | 7541 | 36 | python method at src/requests/models.py:258 |  |  | 0.472 |
| walker |  | 7553 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.472 |
| walker |  | 7577 | 24 | python method doc at src/requests/models.py:358 |  |  | 0.472 |
| walker |  | 7612 | 35 | python method doc at src/requests/models.py:271 |  |  | 0.472 |
| walker |  | 7686 | 74 | python decl at src/requests/models.py:96 |  |  | 0.472 |
| walker |  | 7720 | 34 | python method at src/requests/models.py:141 |  |  | 0.472 |
| walker |  | 7854 | 134 | python decl names surface in src/requests/api.py |  |  | 0.483 |
| walker |  | 7854 | 0 | python decl at src/requests/api.py:90 |  |  | 0.483 |
| walker |  | 7854 | 0 | python decl at src/requests/api.py:102 |  |  | 0.483 |
| walker |  | 7854 | 0 | python decl at src/requests/api.py:171 |  |  | 0.483 |
| walker |  | 7889 | 35 | python decl at src/requests/api.py:24 |  |  | 0.483 |
| walker |  | 7904 | 15 | python decl body at src/requests/api.py:90 body 99 |  |  | 0.483 |
| walker |  | 7919 | 15 | python decl body at src/requests/api.py:171 body 180 |  |  | 0.483 |
| walker |  | 7938 | 19 | python decl doc at src/requests/api.py:24 |  |  | 0.483 |
| walker |  | 7978 | 40 | python decl at src/requests/api.py:74 |  |  | 0.483 |
| walker |  | 8018 | 40 | python decl at src/requests/api.py:137 |  |  | 0.483 |
| walker |  | 8058 | 40 | python decl at src/requests/api.py:154 |  |  | 0.483 |
| walker |  | 8076 | 18 | python decl body at src/requests/api.py:74 body 87 |  |  | 0.483 |
| walker |  | 8094 | 18 | python decl body at src/requests/api.py:137 body 151 |  |  | 0.483 |
| walker |  | 8112 | 18 | python decl body at src/requests/api.py:154 body 168 |  |  | 0.483 |
| walker |  | 8176 | 64 | python decl at src/requests/api.py:117 |  |  | 0.483 |
| walker |  | 8197 | 21 | python decl body at src/requests/api.py:117 body 134 |  |  | 0.483 |
| walker |  | 8287 | 90 | python decl doc at src/requests/api.py:90 |  |  | 0.483 |
| walker |  | 8377 | 90 | python decl doc at src/requests/api.py:171 |  |  | 0.483 |
| walker |  | 8392 | 15 | python method body at src/requests/models.py:133 body 135 |  |  | 0.483 |
| walker |  | 8407 | 15 | python method body at src/requests/models.py:137 body 139 |  |  | 0.483 |
| walker |  | 8422 | 15 | python method body at src/requests/models.py:141 body 145 |  |  | 0.483 |
| walker |  | 8519 | 97 | python method at src/requests/adapters.py:128 |  |  | 0.483 |
| walker |  | 8529 | 10 | python method body at src/requests/adapters.py:128 body 151 |  |  | 0.483 |
| walker |  | 8626 | 97 | python method at src/requests/adapters.py:634 |  |  | 0.483 |
| ns | 8775 |  | 1374 | HTTPAdapter.send() full body | 7.2 | 7.1 | 0.444 |
| walker |  | 8782 | 156 | python decl names surface in src/requests/exceptions.py |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:20 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:38 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:42 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:66 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:70 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:74 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:78 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:82 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:91 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:98 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:102 |  |  | 0.452 |
| walker |  | 8782 | 0 | python decl at src/requests/exceptions.py:106 |  |  | 0.452 |
| walker |  | 8791 | 9 | python decl doc at src/requests/exceptions.py:38 |  |  | 0.452 |
| walker |  | 8800 | 9 | python decl doc at src/requests/exceptions.py:66 |  |  | 0.452 |
| walker |  | 8809 | 9 | python decl doc at src/requests/exceptions.py:70 |  |  | 0.452 |
| walker |  | 8818 | 9 | python decl doc at src/requests/exceptions.py:74 |  |  | 0.452 |
| walker |  | 8827 | 9 | python decl doc at src/requests/exceptions.py:78 |  |  | 0.452 |
| walker |  | 8837 | 10 | python decl doc at src/requests/exceptions.py:106 |  |  | 0.452 |
| walker |  | 8850 | 13 | python decl doc at src/requests/exceptions.py:42 |  |  | 0.452 |
| walker |  | 8864 | 14 | python decl doc at src/requests/exceptions.py:102 |  |  | 0.452 |
| walker |  | 8882 | 18 | python decl doc at src/requests/exceptions.py:98 |  |  | 0.452 |
| walker |  | 8909 | 27 | python class body at src/requests/exceptions.py:20 |  |  | 0.452 |
| walker |  | 8937 | 28 | python decl doc at src/requests/exceptions.py:20 |  |  | 0.452 |
| walker |  | 8980 | 43 | python decl doc at src/requests/exceptions.py:91 |  |  | 0.452 |
| walker |  | 9050 | 70 | python method sigs in src/requests/exceptions.py |  |  | 0.452 |
| walker |  | 9050 | 0 | python method at src/requests/exceptions.py:28 |  |  | 0.452 |
| walker |  | 9050 | 0 | python method at src/requests/exceptions.py:45 |  |  | 0.452 |
| walker |  | 9050 | 0 | python method at src/requests/exceptions.py:55 |  |  | 0.452 |
| walker |  | 9069 | 19 | python method doc at src/requests/exceptions.py:28 |  |  | 0.452 |
| walker |  | 9084 | 15 | python method body at src/requests/exceptions.py:55 body 63 |  |  | 0.452 |
| ns | 9110 |  | 335 | auth.py class/method locations | 8.1 |  | 0.446 |
| walker |  | 9149 | 65 | python decl doc at src/requests/exceptions.py:82 |  |  | 0.446 |
| walker |  | 9275 | 126 | python decl names surface in src/requests/auth.py |  |  | 0.447 |
| walker |  | 9275 | 0 | python decl at src/requests/auth.py:34 |  |  | 0.447 |
| walker |  | 9275 | 0 | python decl at src/requests/auth.py:78 |  |  | 0.447 |
| walker |  | 9275 | 0 | python decl at src/requests/auth.py:85 |  |  | 0.447 |
| walker |  | 9275 | 0 | python decl at src/requests/auth.py:116 |  |  | 0.447 |
| walker |  | 9275 | 0 | python decl at src/requests/auth.py:124 |  |  | 0.447 |
| walker |  | 9290 | 15 | python decl doc at src/requests/auth.py:78 |  |  | 0.447 |
| walker |  | 9307 | 17 | python decl doc at src/requests/auth.py:85 |  |  | 0.447 |
| ns | 9322 |  | 212 | cookies.py top-level class/function locations | 9.1 |  | 0.443 |
| walker |  | 9329 | 22 | python class body at src/requests/auth.py:85 |  |  | 0.443 |
| walker |  | 9346 | 17 | python decl doc at src/requests/auth.py:116 |  |  | 0.443 |
| ns | 9350 |  | 28 | RequestsCookieJar method locations (sampled) | 9.2 | 9.1 | 0.442 |
| walker |  | 9363 | 17 | python decl doc at src/requests/auth.py:124 |  |  | 0.442 |
| walker |  | 9375 | 12 | python decl doc at src/requests/auth.py:34 |  |  | 0.442 |
| ns | 9427 |  | 77 | utils.py function locations (sampled) | 10.1 |  | 0.441 |
| walker |  | 9466 | 91 | python class body at src/requests/auth.py:124 |  |  | 0.441 |
| ns | 9801 |  | 374 | certs.py full + packages.py (near-complete) | 11.1 |  | 0.431 |
| walker |  | 9818 | 352 | python method sigs in src/requests/auth.py |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:81 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:91 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:93 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:96 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:100 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:108 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:111 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:119 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:136 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:138 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:141 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:157 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:268 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:273 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:345 |  |  | 0.450 |
| walker |  | 9818 | 0 | python method at src/requests/auth.py:353 |  |  | 0.450 |
| walker |  | 9832 | 14 | python method at src/requests/auth.py:321 |  |  | 0.450 |
| walker |  | 9847 | 15 | python method at src/requests/auth.py:147 |  |  | 0.450 |
| walker |  | 9856 | 9 | python method body at src/requests/auth.py:108 body 109 |  |  | 0.450 |
| ns | 9862 |  | 61 | docs/user/quickstart.rst heading roster (sampled) | 12.1 |  | 0.448 |
| walker |  | 9865 | 9 | python method body at src/requests/auth.py:353 body 354 |  |  | 0.448 |
| walker |  | 9880 | 15 | python method doc at src/requests/auth.py:268 |  |  | 0.448 |
| walker |  | 9894 | 14 | python method body at src/requests/auth.py:81 body 82 |  |  | 0.448 |
| walker |  | 9916 | 22 | python method doc at src/requests/auth.py:157 |  |  | 0.448 |
| walker |  | 9934 | 18 | python method body at src/requests/auth.py:96 body 97 |  |  | 0.448 |
| walker |  | 9955 | 21 | python method body at src/requests/auth.py:91 body 92 |  |  | 0.448 |
| ns | 9968 |  | 106 | tox.ini (near-complete) | 13.1 |  | 0.444 |
| walker |  | 9976 | 21 | python method body at src/requests/auth.py:93 body 94 |  |  | 0.444 |
| walker |  | 9997 | 21 | python method body at src/requests/auth.py:136 body 137 |  |  | 0.443 |
| ns | 9997 |  | 29 | HISTORY.md recent release heading roster | 14.1 |  | 0.443 |
