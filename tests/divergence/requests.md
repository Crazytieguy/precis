Score(3000)=0.684 I=0.796 C=0.587 ns_rows≤3K=17/49 (reached=9 partial=0 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 47 |  | 47 | Identity lede: what the library is, and its version | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | listing of '.' |  |  | 0.000 |
| walker |  | 82 | 3 | listing of 'src' |  |  | 0.000 |
| walker |  | 115 | 33 | listing of 'ext' |  |  | 0.000 |
| ns | 126 |  | 79 | Repository root listing (complete) | 1.2 |  | 0.671 |
| walker |  | 174 | 59 | listing of 'docs' |  |  | 0.680 |
| walker |  | 177 | 3 | listing of 'docs/_templates' |  |  | 0.680 |
| walker |  | 185 | 8 | listing of 'docs/_static' |  |  | 0.680 |
| walker |  | 194 | 9 | listing of 'docs/dev' |  |  | 0.683 |
| walker |  | 207 | 13 | listing of 'docs/_themes' |  |  | 0.683 |
| ns | 219 |  | 93 | `src/requests/` module roster (complete) | 1.3 |  | 0.463 |
| walker |  | 227 | 20 | listing of 'docs/user' |  |  | 0.467 |
| walker |  | 264 | 37 | listing of 'docs/community' |  |  | 0.478 |
| ns | 324 |  | 105 | `tests/` roster (complete, incl. `testserver/` and `certs/`) | 1.4 |  | 0.378 |
| walker |  | 353 | 89 | listing of 'src/requests' |  |  | 0.607 |
| ns | 471 |  | 147 | README: the canonical `requests.get(...)` doctest + every `##` heading | 1.5 | 1.1 | 0.540 |
| ns | 589 |  | 118 | Full package metadata block (`__version__.py`) | 1.6 | 1.1 | 0.507 |
| ns | 688 |  | 99 | CI: `.github/` and workflow file roster (complete) | 1.7 |  | 0.449 |
| ns | 768 |  | 80 | Makefile: install / test / CI targets | 1.8 |  | 0.431 |
| ns | 896 |  | 128 | `docs/` tree listing (complete) | 1.9 |  | 0.487 |
| ns | 1198 |  | 302 | pyproject: build backend, metadata, runtime dependencies | 1.10 |  | 0.441 |
| walker |  | 1355 | 1002 | python imports in src/requests/__init__.py |  |  | 0.454 |
| walker |  | 1389 | 34 | python decl names surface in src/requests/__init__.py |  |  | 0.454 |
| walker |  | 1405 | 16 | python decl at src/requests/__init__.py:99 |  |  | 0.454 |
| ns | 1469 |  | 271 | `requests/__init__.py`: name → module re-export map | 2.1 |  | 0.494 |
| ns | 1543 |  | 74 | `api.py`: names of all eight module-level functions | 2.2 |  | 0.482 |
| walker |  | 1580 | 175 | README headline in README.md |  |  | 0.600 |
| walker |  | 1620 | 40 | headings outline in README.md |  |  | 0.624 |
| walker |  | 1667 | 47 | python decl at src/requests/__init__.py:60 |  |  | 0.624 |
| walker |  | 1711 | 44 | listing of '.github' |  |  | 0.639 |
| walker |  | 1751 | 40 | listing of '.github/workflows' |  |  | 0.675 |
| walker |  | 1764 | 13 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.694 |
| walker |  | 1822 | 58 | [package] in pyproject.toml |  |  | 0.697 |
| ns | 1896 |  | 353 | `exceptions.py`: the complete class hierarchy | 2.3 |  | 0.651 |
| walker |  | 1969 | 147 | README.md section #0 |  |  | 0.651 |
| walker |  | 2046 | 77 | manifest config in pyproject.toml |  |  | 0.656 |
| walker |  | 2066 | 20 | python imports in setup.py |  |  | 0.656 |
| ns | 2072 |  | 176 | `Session` class declaration + docstring | 2.4 |  | 0.628 |
| walker |  | 2148 | 82 | tool.setuptools config in pyproject.toml |  |  | 0.628 |
| walker |  | 2287 | 139 | [dependencies] in pyproject.toml |  |  | 0.641 |
| ns | 2343 |  | 271 | `Session` attribute set: typed fields + `__attrs__` | 2.5 | 2.4 | 0.604 |
| walker |  | 2350 | 63 | README.md section #1 |  |  | 0.604 |
| walker |  | 2475 | 125 | package metadata in pyproject.toml |  |  | 0.643 |
| walker |  | 2553 | 78 | listing of 'tests' |  |  | 0.680 |
| walker |  | 2562 | 9 | listing of 'tests/testserver' |  |  | 0.690 |
| walker |  | 2578 | 16 | listing of 'tests/certs' |  |  | 0.713 |
| ns | 2631 |  | 288 | `Session` method roster (complete, with line coordinates) | 2.6 | 2.4 | 0.686 |
| ns | 2882 |  | 251 | `Session.request`: the complete keyword signature | 2.7 | 2.6 | 0.663 |
| walker |  | 2934 | 356 | plaintext config Makefile |  |  | 0.684 |
| walker |  | 2973 | 39 | plaintext config docs/requirements.txt |  |  | 0.684 |
| ns | 3065 |  | 183 | `Session.__init__`: every default value | 2.8 | 2.6 | 0.667 |
| walker |  | 3229 | 256 | README.md section #3 |  |  | 0.667 |
| walker |  | 3255 | 26 | python imports in src/requests/packages.py |  |  | 0.667 |
| ns | 3418 |  | 353 | `models.py` module map: constants, all five classes, `Request` fields | 3.1 |  | 0.638 |
| walker |  | 3431 | 176 | README.md section #2 |  |  | 0.638 |
| walker |  | 3504 | 73 | declaration surface of requirements-dev.txt |  |  | 0.638 |
| walker |  | 3510 | 6 | listing of 'tests/certs/valid' |  |  | 0.638 |
| walker |  | 3567 | 57 | python decl names surface in src/requests/sessions.py |  |  | 0.638 |
| walker |  | 3567 | 0 | python decl at src/requests/sessions.py:127 |  |  | 0.638 |
| walker |  | 3567 | 0 | python decl at src/requests/sessions.py:395 |  |  | 0.638 |
| walker |  | 3567 | 0 | python decl at src/requests/sessions.py:908 |  |  | 0.638 |
| walker |  | 3575 | 8 | python decl body at src/requests/sessions.py:908 body 920 |  |  | 0.638 |
| walker |  | 3606 | 31 | python decl at src/requests/sessions.py:76 |  |  | 0.638 |
| walker |  | 3659 | 53 | python decl at src/requests/sessions.py:108 |  |  | 0.638 |
| walker |  | 3691 | 32 | python class body at src/requests/sessions.py:127 |  |  | 0.638 |
| ns | 3739 |  | 321 | `PreparedRequest`: attributes + every `prepare_*` step | 3.2 | 3.1 | 0.617 |
| walker |  | 3964 | 273 | python class body at src/requests/sessions.py:395 |  |  | 0.668 |
| walker |  | 4021 | 57 | python decl doc at src/requests/sessions.py:108 |  |  | 0.668 |
| ns | 4042 |  | 303 | `Response` attribute set: typed fields + `__attrs__` | 3.3 | 3.1 | 0.638 |
| ns | 4324 |  | 282 | `Response`: `__init__`, dunders and every `@property` | 3.4 | 3.3 | 0.621 |
| ns | 4476 |  | 152 | `Response`: the remaining members (`iter_content` … `close`) | 3.5 | 3.4 | 0.612 |
| walker |  | 4495 | 474 | python method sigs in src/requests/sessions.py |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:132 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:134 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:154 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:505 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:508 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:511 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:673 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:684 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:742 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:752 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:870 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:883 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:888 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:899 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method at src/requests/sessions.py:903 |  |  | 0.644 |
| walker |  | 4495 | 0 | python method body at src/requests/sessions.py:132 body 132 |  |  | 0.644 |
| walker |  | 4521 | 26 | python method at src/requests/sessions.py:309 |  |  | 0.644 |
| walker |  | 4547 | 26 | python method at src/requests/sessions.py:370 |  |  | 0.644 |
| walker |  | 4563 | 16 | python method doc at src/requests/sessions.py:883 |  |  | 0.644 |
| walker |  | 4581 | 18 | python method doc at src/requests/sessions.py:154 |  |  | 0.644 |
| walker |  | 4601 | 20 | python method doc at src/requests/sessions.py:134 |  |  | 0.645 |
| walker |  | 4644 | 43 | python method at src/requests/sessions.py:714 |  |  | 0.645 |
| walker |  | 4687 | 43 | python method at src/requests/sessions.py:728 |  |  | 0.645 |
| ns | 4691 |  | 215 | `adapters.py`: pool defaults + the `BaseAdapter` contract | 3.6 |  | 0.630 |
| walker |  | 4732 | 45 | python method at src/requests/sessions.py:334 |  |  | 0.630 |
| walker |  | 4782 | 50 | python method at src/requests/sessions.py:442 |  |  | 0.630 |
| walker |  | 4840 | 58 | python method at src/requests/sessions.py:655 |  |  | 0.630 |
| walker |  | 4874 | 34 | python method doc at src/requests/sessions.py:752 |  |  | 0.630 |
| walker |  | 4941 | 67 | python decl doc at src/requests/sessions.py:76 |  |  | 0.630 |
| walker |  | 4980 | 39 | python method doc at src/requests/sessions.py:370 |  |  | 0.630 |
| walker |  | 5021 | 41 | python method doc at src/requests/sessions.py:888 |  |  | 0.630 |
| ns | 5055 |  | 364 | `HTTPAdapter`: constructor knobs + complete method roster | 3.7 | 3.6 | 0.607 |
| walker |  | 5093 | 72 | python method at src/requests/sessions.py:695 |  |  | 0.607 |
| walker |  | 5129 | 36 | python decl names surface in src/requests/help.py |  |  | 0.607 |
| walker |  | 5129 | 0 | python decl at src/requests/help.py:37 |  |  | 0.607 |
| walker |  | 5129 | 0 | python decl at src/requests/help.py:69 |  |  | 0.607 |
| walker |  | 5129 | 0 | python decl at src/requests/help.py:128 |  |  | 0.607 |
| walker |  | 5142 | 13 | python decl doc at src/requests/help.py:69 |  |  | 0.607 |
| walker |  | 5156 | 14 | python decl doc at src/requests/help.py:128 |  |  | 0.607 |
| walker |  | 5174 | 18 | python decl body at src/requests/help.py:128 body 130 |  |  | 0.607 |
| walker |  | 5219 | 45 | python method doc at src/requests/sessions.py:870 |  |  | 0.607 |
| walker |  | 5300 | 81 | python method at src/requests/sessions.py:831 |  |  | 0.607 |
| ns | 5324 |  | 269 | `SessionRedirectMixin` + session-module merge helpers | 3.8 |  | 0.611 |
| walker |  | 5342 | 42 | python method doc at src/requests/sessions.py:831 |  |  | 0.611 |
| walker |  | 5431 | 89 | python decl names surface in src/requests/adapters.py |  |  | 0.613 |
| walker |  | 5431 | 0 | python decl at src/requests/adapters.py:122 |  |  | 0.613 |
| walker |  | 5431 | 0 | python decl at src/requests/adapters.py:158 |  |  | 0.613 |
| walker |  | 5442 | 11 | python decl doc at src/requests/adapters.py:122 |  |  | 0.613 |
| walker |  | 5457 | 15 | python decl doc at src/requests/adapters.py:158 |  |  | 0.613 |
| ns | 5560 |  | 236 | `cookies.py` module map: classes + free functions | 3.9 |  | 0.604 |
| walker |  | 5611 | 154 | python class body at src/requests/adapters.py:158 |  |  | 0.604 |
| ns | 5868 |  | 308 | `auth.py`: every auth handler and its methods | 3.10 |  | 0.590 |
| walker |  | 5896 | 285 | python method sigs in src/requests/adapters.py |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:125 |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:153 |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:223 |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:269 |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:365 |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:555 |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:599 |  |  | 0.603 |
| walker |  | 5896 | 0 | python method at src/requests/adapters.py:613 |  |  | 0.603 |
| walker |  | 5909 | 13 | python method doc at src/requests/adapters.py:153 |  |  | 0.605 |
| walker |  | 5940 | 31 | python method at src/requests/adapters.py:565 |  |  | 0.605 |
| walker |  | 5974 | 34 | python method at src/requests/adapters.py:512 |  |  | 0.605 |
| walker |  | 6013 | 39 | python method at src/requests/adapters.py:307 |  |  | 0.605 |
| walker |  | 6053 | 40 | python method at src/requests/adapters.py:226 |  |  | 0.605 |
| walker |  | 6101 | 48 | python method at src/requests/adapters.py:403 |  |  | 0.605 |
| walker |  | 6160 | 59 | python method at src/requests/adapters.py:239 |  |  | 0.605 |
| walker |  | 6232 | 72 | python method at src/requests/adapters.py:455 |  |  | 0.605 |
| ns | 6240 |  | 372 | `_types.py`: public type-alias roster | 4.1 |  | 0.588 |
| walker |  | 6308 | 76 | python method at src/requests/adapters.py:201 |  |  | 0.599 |
| walker |  | 6361 | 53 | python method doc at src/requests/adapters.py:555 |  |  | 0.599 |
| walker |  | 6471 | 110 | declaration surface of tox.ini |  |  | 0.599 |
| ns | 6551 |  | 311 | `_types.py`: the `Unpack` kwargs TypedDicts | 4.2 | 4.1 | 0.584 |
| walker |  | 6628 | 157 | python decl names surface in src/requests/compat.py |  |  | 0.584 |
| walker |  | 6628 | 0 | python decl at src/requests/compat.py:37 |  |  | 0.584 |
| walker |  | 6640 | 12 | python decl doc at src/requests/compat.py:37 |  |  | 0.584 |
| walker |  | 6696 | 56 | python method doc at src/requests/sessions.py:309 |  |  | 0.584 |
| ns | 6750 |  | 199 | Environment-driven settings: the `merge_environment_settings` core | 4.3 | 2.6 | 0.575 |
| walker |  | 6793 | 97 | python method at src/requests/adapters.py:128 |  |  | 0.591 |
| ns | 6857 |  | 107 | `utils.py` module-level constants | 5.1 |  | 0.587 |
| walker |  | 6890 | 97 | python method at src/requests/adapters.py:634 |  |  | 0.604 |
| walker |  | 6915 | 25 | python imports in tests/__init__.py |  |  | 0.604 |
| walker |  | 6965 | 50 | python decl names surface in src/requests/status_codes.py |  |  | 0.604 |
| walker |  | 6965 | 0 | python decl at src/requests/status_codes.py:109 |  |  | 0.604 |
| ns | 7005 |  | 148 | `utils.py` function roster 1/3: proxies shim, lengths, netrc, encoders | 5.2 |  | 0.597 |
| walker |  | 7111 | 146 | python decl names surface in src/requests/models.py |  |  | 0.601 |
| walker |  | 7111 | 0 | python decl at src/requests/models.py:109 |  |  | 0.601 |
| walker |  | 7111 | 0 | python decl at src/requests/models.py:255 |  |  | 0.601 |
| walker |  | 7111 | 0 | python decl at src/requests/models.py:283 |  |  | 0.601 |
| walker |  | 7111 | 0 | python decl at src/requests/models.py:376 |  |  | 0.601 |
| walker |  | 7111 | 0 | python decl at src/requests/models.py:730 |  |  | 0.601 |
| walker |  | 7129 | 18 | python decl doc at src/requests/models.py:283 |  |  | 0.601 |
| walker |  | 7168 | 39 | python decl doc at src/requests/models.py:730 |  |  | 0.602 |
| ns | 7172 |  | 167 | `utils.py` function roster 2/3: cookies, encoding detection, URI quoting, CIDR | 5.3 | 5.2 | 0.594 |
| walker |  | 7179 | 11 | python class body at src/requests/models.py:109 |  |  | 0.594 |
| ns | 7352 |  | 180 | `utils.py` function roster 3/3: proxy resolution, default headers, header validation | 5.4 | 5.3 | 0.587 |
| walker |  | 7438 | 259 | python class body at src/requests/models.py:730 |  |  | 0.619 |
| walker |  | 7561 | 123 | python class body at src/requests/models.py:283 |  |  | 0.630 |
| ns | 7613 |  | 261 | `structures.py`: `CaseInsensitiveDict` and `LookupDict` in full shape | 5.5 |  | 0.620 |
| walker |  | 7660 | 99 | python class body at src/requests/models.py:376 |  |  | 0.622 |
| ns | 7723 |  | 110 | `status_codes.py`: how `requests.codes` is built | 5.6 |  | 0.618 |
| walker |  | 7734 | 74 | python decl at src/requests/models.py:96 |  |  | 0.628 |
| walker |  | 7751 | 17 | python class body at src/requests/models.py:255 |  |  | 0.628 |
| ns | 7965 |  | 242 | The five small modules: hooks, `_internal_utils`, compat, certs, packages | 5.7 |  | 0.620 |
| ns | 8142 |  | 177 | `help.py`: the bug-report payload | 5.8 |  | 0.613 |
| walker |  | 8452 | 701 | python method sigs in src/requests/models.py |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:271 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:355 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:358 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:451 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:454 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:465 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:563 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:652 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:720 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:763 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:810 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:813 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:824 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:832 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:835 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:845 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:855 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:1087 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:1140 |  |  | 0.652 |
| walker |  | 8452 | 0 | python method at src/requests/models.py:1169 |  |  | 0.652 |
| walker |  | 8458 | 6 | python method at src/requests/models.py:112 |  |  | 0.652 |
| walker |  | 8466 | 8 | python method at src/requests/models.py:859 |  |  | 0.653 |
| walker |  | 8474 | 8 | python method at src/requests/models.py:874 |  |  | 0.654 |
| ns | 8475 |  | 333 | `resolve_redirects`: the rules the redirect loop enforces | 6.1 | 3.8 | 0.641 |
| walker |  | 8482 | 8 | python method at src/requests/models.py:881 |  |  | 0.642 |
| walker |  | 8490 | 8 | python method at src/requests/models.py:889 |  |  | 0.643 |
| walker |  | 8498 | 8 | python method at src/requests/models.py:894 |  |  | 0.645 |
| walker |  | 8507 | 9 | python method at src/requests/models.py:1030 |  |  | 0.646 |
| walker |  | 8516 | 9 | python method at src/requests/models.py:1049 |  |  | 0.647 |
| walker |  | 8525 | 9 | python method at src/requests/models.py:1122 |  |  | 0.648 |
| walker |  | 8539 | 14 | python method at src/requests/models.py:405 |  |  | 0.648 |
| walker |  | 8547 | 8 | python method at src/requests/models.py:471 |  |  | 0.650 |
| walker |  | 8559 | 12 | python method doc at src/requests/models.py:720 |  |  | 0.650 |
| walker |  | 8572 | 13 | python method doc at src/requests/models.py:112 |  |  | 0.650 |
| walker |  | 8585 | 13 | python method doc at src/requests/models.py:465 |  |  | 0.650 |
| walker |  | 8598 | 13 | python method doc at src/requests/models.py:563 |  |  | 0.650 |
| walker |  | 8627 | 29 | python method at src/requests/models.py:816 |  |  | 0.650 |
| walker |  | 8642 | 15 | python method doc at src/requests/models.py:1030 |  |  | 0.650 |
| walker |  | 8675 | 33 | python method at src/requests/models.py:697 |  |  | 0.650 |
| walker |  | 8691 | 16 | python method doc at src/requests/models.py:855 |  |  | 0.650 |
| walker |  | 8708 | 17 | python method doc at src/requests/models.py:652 |  |  | 0.650 |
| walker |  | 8744 | 36 | python method at src/requests/models.py:258 |  |  | 0.650 |
| walker |  | 8756 | 12 | python method doc at src/requests/models.py:258 |  |  | 0.650 |
| ns | 8764 |  | 289 | `HTTPAdapter.send`: the urllib3 call and the `MaxRetryError` fan-out | 6.2 | 3.7 | 0.638 |
| walker |  | 8792 | 36 | python method at src/requests/models.py:912 |  |  | 0.638 |
| walker |  | 8810 | 18 | python method doc at src/requests/models.py:881 |  |  | 0.638 |
| walker |  | 8828 | 18 | python method doc at src/requests/models.py:1140 |  |  | 0.638 |
| walker |  | 8847 | 19 | python method doc at src/requests/models.py:1122 |  |  | 0.638 |
| walker |  | 8886 | 39 | python method at src/requests/models.py:574 |  |  | 0.638 |
| walker |  | 8900 | 14 | python method doc at src/requests/models.py:574 |  |  | 0.638 |
| walker |  | 8940 | 40 | python method at src/requests/models.py:481 |  |  | 0.638 |
| walker |  | 8953 | 13 | python method doc at src/requests/models.py:481 |  |  | 0.638 |
| ns | 8966 |  | 202 | `HTTPAdapter.send`: the remaining `except` arms and the return | 6.3 | 6.2 | 0.629 |
| walker |  | 8994 | 41 | python method at src/requests/models.py:668 |  |  | 0.629 |
| walker |  | 9008 | 14 | python method doc at src/requests/models.py:668 |  |  | 0.629 |
| walker |  | 9030 | 22 | python method doc at src/requests/models.py:894 |  |  | 0.629 |
| walker |  | 9054 | 24 | python method doc at src/requests/models.py:358 |  |  | 0.633 |
| ns | 9054 |  | 88 | Makefile: the remaining targets (coverage, publish, docs) | 7.1 | 1.8 | 0.633 |
| walker |  | 9078 | 24 | python method doc at src/requests/models.py:889 |  |  | 0.633 |
| walker |  | 9134 | 56 | python method at src/requests/models.py:908 |  |  | 0.633 |
| walker |  | 9134 | 0 | python method body at src/requests/models.py:908 body 911 |  |  | 0.633 |
| walker |  | 9191 | 57 | python method at src/requests/models.py:904 |  |  | 0.633 |
| walker |  | 9191 | 0 | python method body at src/requests/models.py:904 body 907 |  |  | 0.633 |
| walker |  | 9252 | 61 | python method at src/requests/models.py:990 |  |  | 0.633 |
| walker |  | 9287 | 35 | python method doc at src/requests/models.py:271 |  |  | 0.633 |
| walker |  | 9333 | 46 | python method doc at src/requests/models.py:874 |  |  | 0.633 |
| ns | 9337 |  | 283 | pyproject: every remaining table, and the settings that change how you work here | 7.2 | 1.10 | 0.624 |
| walker |  | 9415 | 82 | python method at src/requests/models.py:975 |  |  | 0.624 |
| walker |  | 9415 | 0 | python method body at src/requests/models.py:975 body 981 |  |  | 0.624 |
| ns | 9492 |  | 155 | `tests/conftest.py`: the fixture set every test builds on | 7.3 |  | 0.619 |
| walker |  | 9499 | 84 | python method at src/requests/models.py:982 |  |  | 0.619 |
| walker |  | 9499 | 0 | python method body at src/requests/models.py:982 body 989 |  |  | 0.619 |
| walker |  | 9535 | 36 | python method at src/requests/models.py:133 |  |  | 0.619 |
| walker |  | 9535 | 0 | python method body at src/requests/models.py:133 body 135 |  |  | 0.619 |
| walker |  | 9569 | 34 | python method at src/requests/models.py:137 |  |  | 0.619 |
| walker |  | 9569 | 0 | python method body at src/requests/models.py:137 body 139 |  |  | 0.619 |
| ns | 9587 |  | 95 | `tests/test_requests.py`: top-level class roster | 7.4 |  | 0.616 |
| walker |  | 9610 | 41 | python method at src/requests/models.py:147 |  |  | 0.616 |
| walker |  | 9610 | 0 | python method body at src/requests/models.py:147 body 149 |  |  | 0.616 |
| walker |  | 9651 | 41 | python method at src/requests/models.py:151 |  |  | 0.616 |
| walker |  | 9694 | 43 | python method at src/requests/models.py:183 |  |  | 0.616 |
| ns | 9769 |  | 182 | `tests/testserver/server.py`: the local socket server API (complete) | 7.5 |  | 0.609 |
| ns | 9926 |  | 157 | `docs/api.rst`: every section of the developer interface | 7.6 |  | 0.603 |
| ns | 9974 |  | 48 | `HISTORY.md`: the changelog's head | 7.7 |  | 0.602 |
